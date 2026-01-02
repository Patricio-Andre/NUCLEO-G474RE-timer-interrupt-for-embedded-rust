# blink_minimum — Nucleo G474RE

This repository contains a minimal Rust example that blinks the LED on the
Nucleo G474RE board. The project is intended as a starting point for
embedded Rust development on the STM32G4xx family. The repository includes a
canonical project layout, `Cargo.toml`, `.cargo/config.toml`, a `Makefile`, and
`memory.x` linker script — common components for embedded Rust projects.

Main contents:
- `src/main.rs` — embedded application (main loop toggling PA5).
- `src/utils/` — utilities for the example (previously included logging helpers).
- `memory.x` — linker script (Flash/RAM layout).
- `Makefile` — convenient targets (`build`, `release`, `embed`, `flash`, `clippy`, ...).
- `.cargo/config.toml` — build/target configuration.

## Quick overview

The crate is configured for the `thumbv7em-none-eabihf` target and uses the
`stm32g474` feature of `stm32g4xx-hal`. This example focuses on blinking the
LED and does not depend on any logging backend by default.

## Prerequisites

- Rust toolchain (rustup)
- Add the target:

```bash
rustup target add thumbv7em-none-eabihf
```

- Cross toolchain / linker (example for Debian/Ubuntu):

```bash
sudo apt update
sudo apt install gcc-arm-none-eabi binutils-arm-none-eabi
```

- Flashing / debug tools (choose one or more):

```bash
cargo install probe-rs         # runner for `cargo run`
cargo install cargo-flash      # cargo-flash
cargo install cargo-embed      # cargo-embed (optional, supports many boards)
```

## Notes about `.cargo/config.toml`

This project includes `.cargo/config.toml` with `target = "thumbv7em-none-eabihf"`.
It also contains a `rustflags` entry:

```toml
[target.thumbv7em-none-eabihf]
rustflags = ["-C", "link-arg=-Tlink.x"]

[build]
target = "thumbv7em-none-eabihf"
```

Important: the provided linker script is named `memory.x`.

## Local build

Debug build:

```bash
cargo build
```

Release build (optimized):

```bash
cargo build --release
```


Using the `Makefile` is convenient but you should understand the underlying
commands.

## Flash / Run on Nucleo G474RE

If automatic detection fails, specify the chip explicitly:

```bash
cargo run
# or (with cargo-embed)
cargo embed
```

## Board Manuals and References

- **NUCLEO-G474RE product page**: board documentation and user manuals
  https://www.st.com/en/evaluation-tools/nucleo-g474re.html
- **STM32G474RE product page**: device datasheet and technical documents
  https://www.st.com/en/microcontrollers-microprocessors/stm32g474re.html
- **STM32G4 series documentation hub**: datasheets and reference manuals
  https://www.st.com/en/microcontrollers-microprocessors/stm32g4-series.html

Use the product pages above to download the latest datasheet and reference
manuals for the MCU and the Nucleo board. These manuals contain pinouts,
electrical characteristics, peripheral descriptions, and programming
guidelines that are helpful when adapting this example to other boards.

**SVD file for debugging purposes**: https://github.com/modm-io/cmsis-svd-stm32.git

## Logging

Uses defmt for logging, enabled via RTT by default. Read the documentation to use it properly: https://defmt.ferrous-systems.com/

## VsCode Debugging Setup

This project includes a `.vscode/launch.json` file configured for debugging
with the Cortex-Debug extension. Make sure to install the extension and adjust
the `executable` path if necessary.

