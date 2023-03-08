#![no_main]
#![no_std]

use defmt_rtt as _;
use panic_halt as _;

use rtic::app;

#[app(device = microbit::pac, peripherals = true)]
mod app {

    use microbit::{
        board::Board,
        display::nonblocking::{Display, Frame, GreyscaleImage, MicrobitFrame},
        hal::{
            clocks::Clocks,
            rtc::{Rtc, RtcInterrupt},
        },
        pac,
    };
    use microbit_text::scrolling::Animate;
    use microbit_text::scrolling_text::ScrollingStaticText;

    const MESSAGE: &[u8] = b"WILLOW IS A CUTIE";

    pub enum State {
        Message,
        Heart,
    }

    fn heart_image(brightness: u8) -> GreyscaleImage {
        let b = brightness;
        GreyscaleImage::new(&[
            [0, b, 0, b, 0],
            [b, b, b, b, b],
            [b, b, b, b, b],
            [0, b, b, b, 0],
            [0, 0, b, 0, 0],
        ])
    }

    #[shared]
    struct Shared {
        display: Display<pac::TIMER1>,
    }

    #[local]
    struct Local {
        anim_timer: Rtc<pac::RTC0>,
        scroller: ScrollingStaticText,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let board = Board::new(cx.device, cx.core);

        // Starting the low-frequency clock (needed for RTC to work)
        Clocks::new(board.CLOCK).start_lfclk();

        // Change prescaler to change scroll speed. Higher = slower
        let mut rtc0 = Rtc::new(board.RTC0, 4095).unwrap();
        rtc0.enable_event(RtcInterrupt::Tick);
        rtc0.enable_interrupt(RtcInterrupt::Tick, None);
        rtc0.enable_counter();

        let display = Display::new(board.TIMER1, board.display_pins);

        let mut scroller = ScrollingStaticText::default();
        scroller.set_message(MESSAGE);

        (
            Shared { display },
            Local {
                anim_timer: rtc0,
                scroller,
            },
            init::Monotonics(),
        )
    }

    #[task(binds = TIMER1, priority = 2, shared = [display])]
    fn timer1(mut cx: timer1::Context) {
        cx.shared
            .display
            .lock(|display| display.handle_display_event());
    }

    #[task(binds = RTC0, priority = 1, shared = [display],
           local = [anim_timer, scroller,
                    frame: MicrobitFrame = MicrobitFrame::default(),
                    step: u8 = 0,
                    state: State = State::Message])]
    fn rtc0(cx: rtc0::Context) {
        let mut shared = cx.shared;
        let local = cx.local;
        local.anim_timer.reset_event(RtcInterrupt::Tick);

        match local.state {
            State::Message => {
                if !local.scroller.is_finished() {
                    local.scroller.tick();
                    local.frame.set(local.scroller);
                    shared.display.lock(|display| {
                        display.show_frame(local.frame);
                    });
                } else {
                    *local.state = State::Heart;
                }
            }
            State::Heart => {
                let brightness = match *local.step {
                    0..=8 => *local.step,
                    9..=17 => 9,
                    18..=26 => 26 - *local.step,
                    _ => unreachable!(),
                };

                local.frame.set(&heart_image(brightness));

                shared.display.lock(|display| {
                    display.show_frame(local.frame);
                });

                *local.step += 1;
                if *local.step == 27 {
                    *local.step = 0;
                    *local.state = State::Message;
                    local.scroller.reset();
                }
            }
        }
    }
}
