#![no_std]

pub mod led;
pub mod panic;

use core::sync::atomic::{AtomicBool, Ordering};
pub use cortex_m_rt::entry;
use hal::{
    flash::FlashExt,
    gpio::{GpioExt, PinState},
    pac,
    prelude::*,
};
use led::UserLed;

#[allow(unused_imports)]
pub use rtt_target::{rprintln as log, rtt_init_print as log_init};
use stm32l4xx_hal as hal;

pub type CorePeripherals = cortex_m::Peripherals;

pub struct Board {
    pub cp: CorePeripherals,
    pub user_led: UserLed,
}

impl Board {
    pub fn take() -> Self {
        static TAKEN: AtomicBool = AtomicBool::new(false);
        debug_assert!(!TAKEN.swap(true, Ordering::SeqCst));
        Self::setup()
    }

    fn setup() -> Self {
        #[cfg(debug_assertions)]
        log!("Board init");

        let cp = cortex_m::Peripherals::take().unwrap();
        let dp = pac::Peripherals::take().unwrap();

        // Clock tree
        let mut acr = dp.FLASH.constrain().acr;
        let mut rcc = dp.RCC.constrain();
        let mut pwr = dp.PWR.constrain(&mut rcc.apb1r1);
        let _ = rcc.cfgr.sysclk(80.MHz()).freeze(&mut acr, &mut pwr);

        // User LED
        let mut gpiob = dp.GPIOB.split(&mut rcc.ahb2);
        let user_led = gpiob.pb14.into_push_pull_output_in_state(
            &mut gpiob.moder,
            &mut gpiob.otyper,
            PinState::Low,
        );

        Board { cp, user_led }
    }
}
