#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m::asm;
use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    rprintln!("Initialized rtt on nRF52840");

    loop {
        asm::bkpt();
    }
}
