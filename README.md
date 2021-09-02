# funksteckdose cc1101 example

Embedded rust example for WeAct STM32F411 MiniF4.
This uses a fork of the `funksteckdose` crate (forked to enable `no_std`) and
a fork of the `cc1101` crate (forked to allow transmitting).

The application toggles an rc switch when the user key is pressed.

`funksteckose` is a partial reimplementation of `rc-switch`,
so for more details see [https://github.com/sui77/rc-switch].

# c1101 module

c1101 module is connected over SPI (pins A7 to A4) and GDO0 (pin A3).
GDO0 is used to transmit data in asynchronous mode/compatibility mode -
this lets the c1101 module be used in the same way as more common, simpler modules.

# Prereqs
 
rust target

```sh
$ rustup target install thumbv7em-none-eabihf
```

cargo-binutils

```sh
$ cargo install cargo-binutils
$ rustup component add llvm-tools-preview
```

dfu-util

```sh
$ brew install dfu-util
```

# Building

```sh
$ cargo objcopy --release -- -O binary out.bin
```

# Flashing

## dfu-util

Flashing with dfu requires the following procedure.
1. Disconnect board from USB C.
2. Short A9 and A10 (You can leave these connected).
3. Connect USB C to board.
4. Press BOOT0, NRST.
5. Release NRST, wait 500ms, Release BOOT0.
6. Flash with dfu-util with the following command.

```sh
$ dfu-util -a0 -s 0x08000000  -D out.bin
```

## ST-Link

Using the ST-link with probe-run to flash and run with `cargo run`.

### Install deps

```sh
$ brew install libftdi
```

```sh
$ cargo install cargo-flash
```

```sh
$ cargo install probe-run
```

### Run with probe-run

probe-run is already set as the default cargo runner - just run `cargo run`.
