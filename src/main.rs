#![no_std]
#![no_main]

mod logger;

use core::panic::PanicInfo;
use nrf52840_hal as _;

use cortex_m::asm;
use cortex_m_rt::entry;
use log::LevelFilter;

#[entry]
fn main() -> ! {
    logger::init_with_level(LevelFilter::Trace);
    log::info!("Hello world!");

    loop {
        panic!("oops");
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    log::error!("{}", info);

    // trigger a hard fault to abort
    asm::udf()
}
