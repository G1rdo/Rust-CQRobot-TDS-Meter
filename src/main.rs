#![no_std]
#![no_main]

use panic_halt as _;
use avr_device::interrupt;
use core::cell::RefCell;
use arduino_hal::Adc;

static TDSPIN: &str = "A0";


// Console printing macros and put_console functions from avr-hal example:
// https://github.com/Rahix/avr-hal/blob/main/examples/arduino-uno/src/bin/uno-println.rs
type Console = arduino_hal::hal::usart::Usart0<arduino_hal::DefaultClock>;
static CONSOLE: interrupt::Mutex<RefCell<Option<Console>>> =
    interrupt::Mutex::new(RefCell::new(None));


macro_rules! print {
    ($($t:tt)*) => {
        interrupt::free(
            |cs| {
                if let Some(console) = CONSOLE.borrow(cs).borrow_mut().as_mut() {
                    let _ = ufmt::uwrite!(console, $($t)*);
                }
            },
        )
    };
}

macro_rules! println {
    ($($t:tt)*) => {
        interrupt::free(
            |cs| {
                if let Some(console) = CONSOLE.borrow(cs).borrow_mut().as_mut() {
                    let _ = ufmt::uwriteln!(console, $($t)*);
                }
            },
        )
    };
}

fn put_console(console: Console) {
    interrupt::free(|cs| {
        *CONSOLE.borrow(cs).borrow_mut() = Some(console);
    })
}
/*fn analog_read(voltageread: i64 temperaturex100: i64) -> i64{
    let compensatedvoltage: i64 = (voltageread * 484 + 5000) /  1000 as i64;
    compensatedvoltage

    
}*/


#[arduino_hal::entry]
fn main() -> ! {
    // let dp to put_console are from avr-hal
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut adc = Adc::new(dp.ADC, Default::default());
    let serial = arduino_hal::default_serial!(dp, pins, 57600);
    put_console(serial);

    let mut voltagepin = pins.a0.into_analog_input(&mut adc);
    loop {
        let voltageread: i64 = voltagepin.analog_read(&mut adc).into();
        print!("Analog Read: {} ", voltageread);

        let compensatedvoltage: i64 = (voltageread * 484 + 5000) /  1000 as i64; // Should be divided by 100000, but is instead 1000 to leave the value 100x higher
        print!("Compensated Voltage: {} (100x higher than real voltage)", compensatedvoltage);

        let tdsvalue = ((133 * compensatedvoltage * compensatedvoltage * compensatedvoltage) / 1000000 - 
                        (256 * compensatedvoltage * compensatedvoltage) / 10000 + 
                        (857 * compensatedvoltage) / 100) / 2;
        println!("TDS Value: {}", tdsvalue);

        arduino_hal::delay_ms(100);
    }
}

