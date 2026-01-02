//! Minimal example: blink the LED on the Nucleo G474RE board.
//!
//! This binary is built with `no_std` and `cortex-m-rt` and demonstrates a
//! simple loop that toggles a GPIO pin to turn an LED on and off.
//! Provides information via Real Time Transfer (RTT) logs 
//! Inline comments provide guidance for learning and documentation.

// Deny warnings and unsafe code to simplify teaching and testing.
#![deny(warnings)]
#![deny(unsafe_code)]
// `no_main`: use the entry point provided by `cortex-m-rt`.
#![no_main]
// `no_std`: embedded environment without the standard library.
#![no_std]

// Import convenience traits for configuring pins and clocks.
use hal::prelude::*;
// Access device peripheral structures from the HAL.
use hal::stm32;
// Alias the HAL crate for consistent usage in the code.
use stm32g4xx_hal as hal;

// `#[entry]` macro marks the program entry point.
use cortex_m_rt::entry;

use core::panic::PanicInfo;

use defmt;

use defmt_rtt as _;


// Minimal panic handler for `no_std` embedded programs.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    defmt::error!("Error type: {}", _info);
    loop {}
}


// Application entry point.
#[entry]
fn main() -> ! {
    // Acquire access to microcontroller peripherals.
    // `take()` returns `Some(Peripherals)` only once; it will fail if
    // peripherals have already been taken elsewhere.
    let dp = stm32::Peripherals::take().expect("cannot take peripherals");

    // Build the Reset & Clock Control (RCC) configuration.
    let mut rcc = dp.RCC.constrain();
    // Hardware initialization.

    // Split GPIOA for pin configuration.
    let gpioa = dp.GPIOA.split(&mut rcc);

    // Configure PA5 as push-pull output — LED pin on Nucleo boards.
    let mut led = gpioa.pa5.into_push_pull_output();

    // Main loop: toggle the LED with simple busy-wait delays.
    loop {
        // Set LED low.
        // rprintln!("Set LED Low");
        for _ in 0..20_000 {
            led.set_low().expect("set low gone wrong");
        }
        // info test
        defmt::info!("LED was setted LOW");
        // Set LED high.
        // rprintln!("Set LED High");
        for _ in 0..20_000 {
            led.set_high().expect("set high gone wrong");
        }
        // using info just like you can use format!
        defmt::info!("LED was setted HIGH? {}", led.is_set_high().unwrap());
    }
}

