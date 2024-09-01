#![no_std]
#![no_main]

use arduino_hal::prelude::_unwrap_infallible_UnwrapInfallible;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
    let a0 = pins.a0.into_analog_input(&mut adc);

    loop {
        let value = a0.analog_read(&mut adc);
        ufmt::uwriteln!(&mut serial, "{}", value).unwrap_infallible();
    }
}
