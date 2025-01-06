Rust-CQRobot-TDS-Meter
========================
This is a driver for the CQRobot CQRSENTDS01 TDS Meter, writen in Rust.

Rust project for the _Arduino Uno_.

## The Hardware & The Why
This is a driver for a Total Dissolved Solids meter, which measures the total amount of dissolved (conductive) solids in a solution, such as salt in water. It can also serve as an [indicator of water quality](https://www.google.com/url?sa=t&source=web&rct=j&opi=89978449&url=https://www.epa.gov/sites/default/files/2014-12/documents/akwqs-chapter70.pdf&ved=2ahUKEwjoiJ2A99-KAxX8TDABHcF4NAcQFnoECBAQAw&usg=AOvVaw3XeUlXSSavtET9PGw43goY). There is a surprisingly cheap TDS meter sold by a company called CQRobot, which uses GPIO pins to connect to an Arduino or Raspberry Pi. It has worked excellently for me so far. Each one costs [12$ on Amazon](https://a.co/d/b04iDJ7), and the one I used to write this driver has lasted me a whole year without showing any signs of stopping. Enjoy this Rust port of their [Arduino Code](http://www.cqrobot.wiki/index.php/TDS_(Total_Dissolved_Solids)_Meter_Sensor_SKU:_CQRSENTDS01).

I originally wrote this to see if writing the meter code for my main project, [the HydroBrain](https://github.com/G1rdo/HydroBrain), in Rust was a good option, and very quickly learned why C++ is still majorly dominant. I hope this example helps people learn how to write code for embedded sensor-like applications without the standard library, as learning to work without using floats was quite the experience for me.

![image](https://github.com/user-attachments/assets/1f733e9f-2b33-4e72-840f-e6e4f913f777)

[Total Dissolved Solids - TDS, Premier Water Technologies](https://www.premierwatermn.com/water-quality/water-contaminants/total-dissolved-solids/)


## Build Instructions
1. Install prerequisites as described in the [`avr-hal` README] (`avr-gcc`, `avr-libc`, `avrdude`, [`ravedude`]).

2. Run `cargo build` to build the firmware (cargo build -Z build-std=core --target avr-specs/avr-atmega328p.json
or else it won't work).

3. Run `cargo run -- -P=/dev/ttyUSB1` to flash the firmware to a connected board.  Depending on what usb the Arduino is connected to, it will use different /dev/ttyUSB numbers, such as USB0, USB1, or USB2. Run `ls /dev/ttyUSB*` to see all of your device's USB ports.

[`avr-hal` README]: https://github.com/Rahix/avr-hal#readme
[`ravedude`]: https://crates.io/crates/ravedude

## License
Licensed under
 - MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be licensed as above, without any additional terms or conditions.
