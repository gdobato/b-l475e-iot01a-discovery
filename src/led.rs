//! led

pub use crate::hal::gpio::{GpioExt, Output, Pin, PinState, PushPull, H8};

type DigitalOutputPin<HL, const P: char, const N: u8> = Pin<Output<PushPull>, HL, P, N>;
pub type UserLed = DigitalOutputPin<H8, 'B', 14>;

pub trait Led {
    fn set_on(&mut self);
    fn set_off(&mut self);
    fn toggle(&mut self);
}

impl Led for UserLed {
    #[inline(always)]
    fn set_on(&mut self) {
        self.set_high();
    }

    #[inline(always)]
    fn set_off(&mut self) {
        self.set_low();
    }

    #[inline(always)]
    fn toggle(&mut self) {
        self.toggle();
    }
}
