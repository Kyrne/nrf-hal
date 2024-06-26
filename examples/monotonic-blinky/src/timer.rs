//! A minimal blinky example using `MonotonicTimer`.
#![no_main]
#![no_std]

use hal::pac;
use nrf52840_hal as hal;
use panic_halt as _;
#[rtic::app(device = pac, dispatchers = [UARTE1])]
mod app {
    use super::*;
    use cortex_m::asm;
    use embedded_hal::digital::{OutputPin, StatefulOutputPin};
    use hal::{
        gpio::{p0::Parts, Level, Output, Pin, PushPull},
        monotonic::MonotonicTimer,
    };
    use pac::TIMER0;
    use rtt_target::{rprintln, rtt_init_print};

    #[monotonic(binds = TIMER0, default = true)]
    type MyMono = MonotonicTimer<TIMER0, 16_000_000>;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: Pin<Output<PushPull>>,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();
        rprintln!("init");

        let p0 = Parts::new(cx.device.P0);
        let led = p0.p0_13.into_push_pull_output(Level::High).degrade();

        // New does not exists for invalid frequencies
        let mono = MyMono::new(cx.device.TIMER0);
        blink::spawn().ok();
        (Shared {}, Local { led }, init::Monotonics(mono))
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            rprintln!("idle");
            // Put core to sleep until next interrupt
            asm::wfe();
        }
    }

    #[task(local = [led])]
    fn blink(ctx: blink::Context) {
        rprintln!("Blink!");
        let led = ctx.local.led;
        // Note this unwrap is safe since is_set_low is allways Ok
        if led.is_set_low().unwrap() {
            led.set_high().ok();
        } else {
            led.set_low().ok();
        }
        // spawn after current time + 1 second
        blink::spawn_after(fugit::ExtU32::millis(1000)).ok();
    }
}
