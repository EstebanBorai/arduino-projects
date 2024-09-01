#![no_std]
#![no_main]

use arduino_hal::{clock::Clock, hal::port::PD3, pac::TC1, port::{mode::Output, Pin}, prelude::_unwrap_infallible_UnwrapInfallible};
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
    let sensor = pins.a0.into_analog_input(&mut adc);
    let mut buzzer = pins.d3.into_output();

    loop {
        let value = sensor.analog_read(&mut adc);
        ufmt::uwriteln!(&mut serial, "{}", value).unwrap_infallible();
        buzzer
    }
}

fn tone<P>(pin: Pin, freq: f32, timer: &TC1) {
    const CLOCK: f32 = arduino_hal::DefaultClock::FREQ as f32;
    let ticks: u16 = (CLOCK / freq) as u16;

    // timer.ocr1a.write(|w| unsafe {
    //     w.bits(ticks)
    // })
}
