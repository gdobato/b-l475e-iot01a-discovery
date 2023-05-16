//! led

use crate::hal::gpio::{Output, Pin, PinState, PushPull, H8};

type DigitalOutputPin<HL, const P: char, const N: u8> = Pin<Output<PushPull>, HL, P, N>;
pub type UserLed = DigitalOutputPin<H8, 'B', 14>;

#[derive(Debug)]
pub enum State {
    On,
    Off,
}

pub trait Led {
    fn set_on(&mut self);
    fn set_off(&mut self);
    fn toggle(&mut self);
    fn get_state(&self) -> State;
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

    fn get_state(&self) -> State {
        match self.get_state() {
            PinState::Low => State::Off,
            PinState::High => State::On,
        }
    }
}
