//! Example blinky
//!
//! Toggles board user LED
//!

#![no_std]
#![no_main]

#[allow(unused_imports)]
use b_l475e_iot01a::{
    led::{Led, UserLed},
    log, log_init,
};
use rtic::app;
use systick_monotonic::{fugit::MillisDurationU64, Systick};

#[app(device = b_l475e_iot01a::hal::pac, peripherals = false, dispatchers = [SPI1])]
mod app {

    use super::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        user_led: UserLed,
    }

    #[monotonic(binds = SysTick, default = true)]
    type MonoTimer = Systick<1_000>;

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        log_init!();

        let mono = Systick::new(cx.core.SYST, b_l475e_iot01a::CORE_FREQUENCY.raw());

        // Get board resources
        let b_l475e_iot01a::Board { user_led } = b_l475e_iot01a::Board::take();

        #[cfg(debug_assertions)]
        log!("Spawning tasks");
        let _ = blink_user_led::spawn();

        (Shared {}, Local { user_led }, init::Monotonics(mono))
    }

    #[task(local = [user_led])]
    fn blink_user_led(cx: blink_user_led::Context) {
        cx.local.user_led.toggle();
        #[cfg(debug_assertions)]
        log!("User led : {:?}", Led::get_state(cx.local.user_led));
        blink_user_led::spawn_after(MillisDurationU64::from_ticks(1_000)).unwrap();
    }
}
