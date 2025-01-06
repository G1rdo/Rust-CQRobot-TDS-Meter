Rust-CQRobot-TDS-Meter
========================
This is a driver for the CQRobot TDS (Total Dissolved Solids) Meter, writen in Rust.

Rust project for the _Arduino Uno_.
## The Device
This is a Total Dissolved Solids meter, which measures the total ammount of dissolved (conductive) solids in a solution, such as salt in water. Also, it can be used as an [indicator of water quality](https://www.google.com/url?sa=t&source=web&rct=j&opi=89978449&url=https://www.epa.gov/sites/default/files/2014-12/documents/akwqs-chapter70.pdf&ved=2ahUKEwjoiJ2A99-KAxX8TDABHcF4NAcQFnoECBAQAw&usg=AOvVaw3XeUlXSSavtET9PGw43goY).
![image](https://github.com/user-attachments/assets/1f733e9f-2b33-4e72-840f-e6e4f913f777)

<https://a.co/d/b04iDJ7>

## Build Instructions
1. Install prerequisites as described in the [`avr-hal` README] (`avr-gcc`, `avr-libc`, `avrdude`, [`ravedude`]).

2. Run `cargo build` to build the firmware (cargo build -Z build-std=core --target avr-specs/avr-atmega328p.json
or else it won't work).

3. Run `cargo run` to flash the firmware to a connected board.  If `ravedude`
   fails to detect your board, check its documentation at
   <https://crates.io/crates/ravedude>.

4. `ravedude` will open a console session after flashing where you can interact
   with the UART console of your board.

[`avr-hal` README]: https://github.com/Rahix/avr-hal#readme
[`ravedude`]: https://crates.io/crates/ravedude

## License
Licensed under
 - MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
