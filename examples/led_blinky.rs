//! Example LED blinky
//!
//! Toggles board user LED
//!

#![no_std]
#![no_main]

use b_l475e_iot01a::{entry, log_init};

#[entry]
fn main() -> ! {
    log_init!();
    let b_l475e_iot01a::Board { mut user_led, .. } = b_l475e_iot01a::Board::take();

    loop {
        // TODO: Replace by proper delay
        for _ in 0..500_000 {}
        user_led.toggle();
    }
}
