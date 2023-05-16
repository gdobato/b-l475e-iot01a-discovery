#![no_std]

pub mod led;
pub mod panic;

use core::sync::atomic::{AtomicBool, Ordering};
pub use cortex_m_rt::entry;
pub use embedded_hal::blocking::delay::{DelayMs, DelayUs};
pub use hal::delay::Delay as SysTimer;
use stm32l4xx_hal::{
    self as hal,
    flash::FlashExt,
    gpio::{GpioExt, PinState},
    pac,
    prelude::*,
    rcc::Clocks,
};

#[allow(unused_imports)]
pub use rtt_target::{rprintln as log, rtt_init_print as log_init};

pub type CorePeripherals = cortex_m::Peripherals;

pub struct Board {
    pub cp: CorePeripherals,
    pub clocks: Clocks,
    pub user_led: led::UserLed,
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
        let clocks = rcc.cfgr.sysclk(80.MHz()).freeze(&mut acr, &mut pwr);

        // User LED
        let mut gpiob = dp.GPIOB.split(&mut rcc.ahb2);
        let user_led = gpiob.pb14.into_push_pull_output_in_state(
            &mut gpiob.moder,
            &mut gpiob.otyper,
            PinState::Low,
        );

        Board {
            cp,
            clocks,
            user_led,
        }
    }
}
