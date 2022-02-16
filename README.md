[![License BSD-2-Clause](https://img.shields.io/badge/License-BSD--2--Clause-blue.svg)](https://opensource.org/licenses/BSD-2-Clause)
[![License MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![AppVeyor CI](https://ci.appveyor.com/api/projects/status/github/KizzyCode/SerialPipe-rust?svg=true)](https://ci.appveyor.com/project/KizzyCode/SerialPipe-rust)
[![docs.rs](https://docs.rs/serial_pipe/badge.svg)](https://docs.rs/serial_pipe)
[![crates.io](https://img.shields.io/crates/v/serial_pipe.svg)](https://crates.io/crates/serial_pipe)
[![Download numbers](https://img.shields.io/crates/d/serial_pipe.svg)](https://crates.io/crates/serial_pipe)
[![dependency status](https://deps.rs/crate/serial_pipe/0.1.0/status.svg)](https://deps.rs/crate/serial_pipe/0.1.0)


# `SerialPipe`
Welcome to `SerialPipe` 🎉

`SerialPipe` is a simple program that can be used to read from/write to a serial device node.


## Example
```sh
# Dump /dev/tty.usbmodem21201 @115200 to stdout
spipe /dev/tty.usbmodem21201

# Write "Testolope" to /dev/tty.usbmodem21201 @115200 and dump /dev/tty.usbmodem21201 @115200 to stdout
echo "Testolope" | spipe /dev/tty.usbmodem21201

# Dump /dev/tty.usbmodem21201 @9600 to stdout
spipe /dev/tty.usbmodem666 9600
```

## Usage
```sh
spipe path-to-device-node [baudrate]
```
