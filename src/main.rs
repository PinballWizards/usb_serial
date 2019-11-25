#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_semihosting;
extern crate feather_m0 as hal;
extern crate panic_halt;

use core::fmt::Write;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::gpio::*;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::sercom::*;

use hal::entry;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = hal::Pins::new(peripherals.PORT);

    let gclk0 = clocks.gclk0();

    let mut delay = Delay::new(core.SYST, &mut clocks);

    let mut uart: UART3<Sercom3Pad3<Pa25<PfC>>, Sercom3Pad2<Pa24<PfC>>, (), ()> = UART3::new(
        &clocks.sercom3_core(&gclk0).unwrap(),
        9600.hz(),
        peripherals.SERCOM3,
        &mut peripherals.PM,
        (
            pins.usb_dp.into_pad(&mut pins.port),
            pins.usb_dm.into_pad(&mut pins.port),
        ),
    );

    loop {
        uart.write_str("hello world!").unwrap();
        delay.delay_ms(1000u32);
    }
}
