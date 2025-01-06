#![no_std]
#![no_main]

use panic_halt as _;
use avr_device::interrupt;
use core::cell::RefCell;
use arduino_hal::Adc;


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
// End of avr-hal code

// This macro compiles but when used stops values from being printed for unknown reasons.
/*macro_rules! choose_voltage_pin {
    ($pin:ident) => {
        arduino_hal::pins!(arduino_hal::Peripherals::take().unwrap()).$pin.into_analog_input(&mut Adc::new(arduino_hal::Peripherals::take().unwrap().ADC, Default::default()))
    }
} */

fn analog_to_voltage(analogread: i64, temperaturex100: i64) -> i64{
    let voltageread: i64 = (analogread * 484 + 5000) /  1000 as i64; // Returns 100x the voltage read
    let compensationcoefficient = 100 + 2 * (temperaturex100 - 2500); // Gets a coefficient to compensate for temperature in TDS
    let compensatedvoltage = 100*voltageread / compensationcoefficient; // Uses the coefficient to compensate
    compensatedvoltage
}


#[arduino_hal::entry]
fn main() -> ! {
    // lines let dp to put_console are from avr-hal
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut adc = Adc::new(dp.ADC, Default::default());
    let serial = arduino_hal::default_serial!(dp, pins, 57600);
    put_console(serial);

    let temperatureincelsius: i64 = 25; // Declare the temperature that the meter is recording data at
    
    //let voltagepin = choose_voltage_pin!(a0);
    let voltagepin = pins.a0.into_analog_input(&mut adc);
    loop {
        let voltageread: i64 = voltagepin.analog_read(&mut adc).into(); // Read from the analog voltage pin
        //print!("Analog Read: {} ", voltageread);
        let compensatedvoltage: i64 = analog_to_voltage(voltageread, temperatureincelsius*100); // Using the analog read and the temperature return real voltage

        //print!("Compensated Voltage: {} (100x higher than real voltage)", compensatedvoltage); // Should only ever be between 0 and 230 as per CQrobot specs (0-2.3 V).

        let tdsvalue = ((133 * compensatedvoltage * compensatedvoltage * compensatedvoltage) / 1000000 - 
                        (256 * compensatedvoltage * compensatedvoltage) / 10000 + 
                        (857 * compensatedvoltage) / 100) / 2;
        println!("TDS Value: {}", tdsvalue);

        arduino_hal::delay_ms(100);
    }
}

