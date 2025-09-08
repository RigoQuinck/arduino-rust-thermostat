arduino-rust-thermostat
=======================

Rust project for the _Arduino Uno_.

## Requirements

### Rust

#### Linux / MacOs
Rust install documentation: https://doc.rust-lang.org/book/ch01-01-installation.html

Rust install command
```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

#### Windows

Rust install documentation: https://doc.rust-lang.org/book/ch01-01-installation.html#installing-rustup-on-windows

### C compiler

#### Linux
gcc is preinstalled or available from package manager

#### MacOs
if you didn’t do it yet
```bash
xcode-select --install
```

## Build
1. Install prerequisites as described in the [`avr-hal` README] (`avr-gcc`, `avr-libc`, `avrdude`, [`ravedude`]).

2. Run `cargo build` to build the firmware.

3. Run `cargo run` to flash the firmware to a connected board.  If `ravedude`
   fails to detect your board, check its documentation at
   <https://crates.io/crates/ravedude>.

4. `ravedude` will open a console session after flashing where you can interact
   with the UART console of your board.

[`avr-hal` README]: https://github.com/Rahix/avr-hal#readme
[`ravedude`]: https://crates.io/crates/ravedude

## License
Licensed under either of

 - Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 - MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
