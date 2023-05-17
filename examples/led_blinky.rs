//! Example LED blinky
//!
//! Toggles board user LED
//!

#![no_std]
#![no_main]

use b_l475e_iot01a::{entry, led::Led, log, log_init, DelayMs, SysTimer};

#[entry]
fn main() -> ! {
    log_init!();
    let b_l475e_iot01a::Board {
        core_peripherals,
        clocks,
        mut user_led,
        ..
    } = b_l475e_iot01a::Board::take();

    let mut sys_timer = SysTimer::new(core_peripherals.SYST, clocks);

    loop {
        sys_timer.delay_ms(500u16);
        user_led.toggle();
        log!("User_led {:?}", Led::get_state(&user_led));
    }
}
