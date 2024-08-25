#![no_std]
#![no_main]

use arduino_hal::hal::port::{PB3, PB4, PB5};
use arduino_hal::port::{mode::Output, Pin};
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let mut pin_set = PinSet::new();

    pin_set.set_high();
    arduino_hal::delay_ms(1000);

    loop {
        pin_set.set_low();
        arduino_hal::delay_ms(1000);

        pin_set.set_one_high(Led::Green);
        arduino_hal::delay_ms(3000);


        pin_set.set_low();
        pin_set.set_one_high(Led::Yellow);
        arduino_hal::delay_ms(1000);

        pin_set.set_low();
        pin_set.set_one_high(Led::Red);
        arduino_hal::delay_ms(2000);
    }
}

enum Led {
    Green,
    Yellow,
    Red,
}

struct PinSet {
    green: Pin<Output, PB3>,
    yellow: Pin<Output, PB4>,
    red: Pin<Output, PB5>,
}

impl PinSet {
    fn new() -> Self {
        let dp = arduino_hal::Peripherals::take().expect("Failed to retrieve Peripherals");
        let pins = arduino_hal::pins!(dp);
        let green = pins.d11.into_output();
        let yellow = pins.d12.into_output();
        let red = pins.d13.into_output();

        Self {
            green,
            yellow,
            red,
        }
    }

    fn set_high(&mut self) {
        self.green.set_high();
        self.yellow.set_high();
        self.red.set_high();
    }

    fn set_low(&mut self) {
        self.green.set_low();
        self.yellow.set_low();
        self.red.set_low();
    }

    fn set_one_high(&mut self, led: Led) {
        match led {
            Led::Green => self.green.set_high(),
            Led::Yellow => self.yellow.set_high(),
            Led::Red => self.red.set_high(),
        }
    }
}
