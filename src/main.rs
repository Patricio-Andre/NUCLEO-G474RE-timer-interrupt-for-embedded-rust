//! example: blink the LED on the Nucleo G474RE board.
//!
//! This binary is built with `no_std` and `cortex-m-rt` and demonstrates an
//! application with interrupts driven by both GPIO (button) and a hardware timer (TIM2).
//! It provides information via Real Time Transfer (RTT) logs.
//! Inline comments provide guidance for learning and documentation.

// Deny warnings and unsafe code to simplify teaching and testing.
// #![deny(warnings)]
// #![deny(unsafe_code)]
// `no_main`: use the entry point provided by `cortex-m-rt`.
#![no_main]
// `no_std`: embedded environment without the standard library.
#![no_std]

// Import convenience traits for configuring pins and clocks.
use hal::prelude::*;
// Access device peripheral structures from the HAL.
// stm32 deppends on what board do you use.
use hal::stm32;
use hal::gpio::{ExtiPin,
                Floating,
                PushPull,
                Input,
                Output,
                gpioc,
                gpioa};


// Example HAL structure

use hal::syscfg::SysCfgExt;

// Alias the HAL crate for consistent usage in the code.
use stm32g4xx_hal as hal;


// `#[entry]` macro marks the program entry point.
use cortex_m_rt::entry;

use core::panic::PanicInfo;

use defmt;

use defmt_rtt as _;

// Configuring interrupts
use hal::stm32::TIM2;

use core::cell::{Cell, RefCell};

use cortex_m::interrupt::Mutex;

use hal::gpio::SignalEdge as SignalEdge;

use hal::interrupt;

// Configuring Timer

use hal::timer::{Timer,
                 Event,
                 CountDownTimer};

// Alias for button pin
type ButtonPin = gpioc::PC13<Input<Floating>>;

// Alias for led pin
type LedPin = gpioa::PA5<Output<PushPull>>;


// Setting Mutex for interrupts
// Create a Global Variable for the Button GPIO Peripheral that I'm going to pass around.
static G_BUTTON: Mutex<RefCell<Option<ButtonPin>>> = Mutex::new(RefCell::new(None));
// Create a Global Variable for the Timer Peripheral that I'm going to pass around.
static G_TIM: Mutex<RefCell<Option<CountDownTimer<TIM2>>>> = Mutex::new(RefCell::new(None));
// Create a Global Variable for the LED GPIO Peripheral that I'm going to pass around.
static G_LED: Mutex<RefCell<Option<LedPin>>> = Mutex::new(RefCell::new(None));
// Create a Global Variable for the delay value that I'm going to use to manage the delay.
static G_DELAYMS: Mutex<Cell<u32>> = Mutex::new(Cell::new(1000));


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
    let mut dp = stm32::Peripherals::take().expect("cannot take peripherals");
    // Build the Reset & Clock Control (RCC) configuration.
    let mut rcc = dp.RCC.constrain();
    // Hardware initialization.
    // Split GPIOA and GPIOC for pin configuration.
    let gpioa = dp.GPIOA.split(&mut rcc);
    let gpioc = dp.GPIOC.split(&mut rcc);
    // Setting clocks
    // Constrain method already set clock as default --> HSI clock: 16mhz
    let timer = Timer::new(dp.TIM2, &rcc.clocks);

    // Turn it into a CountDownTimer.
    // Note: 1000ms is roughly the maximum duration settable using this simple method
    // due to integer math limitations when converting ms to Hz (frequency = 1/period).
    // To achieve longer time spans, you should use `fugit` types or manual prescalers.

    let mut count_down_timer = timer.start_count_down(1000.ms());
    
    // starts watching timeouts to trigger interrupts
    count_down_timer.listen(Event::TimeOut);


   // Configure Button Pin for Interrupts
    
    // Configure PA5 as push-pull output — LED pin on Nucleo boards.
    let led = gpioa.pa5.into_push_pull_output();
    // Configure PC13 as input. No need to be mutable, we're only reading it.
    let mut button = gpioc.pc13.into_floating_input();
    
    
    // 1) Promote SYSCFG structure to HAL to be able to configure interrupts
    let mut syscfg = dp.SYSCFG.constrain();
    // 2) Make button an interrupt source
    button.make_interrupt_source(&mut syscfg);
    // 3) Make button an interrupt source
    button.trigger_on_edge(&mut dp.EXTI, SignalEdge::Rising);
    // 4) Enable gpio interrupt for button
    button.enable_interrupt(&mut dp.EXTI);

    // Enable the external interrupt in the NVIC by passing the button interrupt number
    unsafe {
        cortex_m::peripheral::NVIC::unmask(interrupt::EXTI15_10);
        cortex_m::peripheral::NVIC::unmask(interrupt::TIM2);
    }

    // Now that button is configured, move button into global context
    // Define critical section for button, led and the timer that you choose
    cortex_m::interrupt::free(|cs| {
        G_BUTTON.borrow(cs).replace(Some(button));
        G_LED.borrow(cs).replace(Some(led));
        G_TIM.borrow(cs).replace(Some(count_down_timer));
        defmt::info!("Delay Atual: {} ms", G_DELAYMS.borrow(cs).get());
    });

    loop {
        // wfi stands for wait for interrupt and what it does is send the processor to sleep while it's sitting idle
        // Comment this line to use info! or other defmt macros
        cortex_m::asm::wfi();
    }
}


#[interrupt]
fn EXTI15_10() {
    // Start a Critical Section
    cortex_m::interrupt::free(|cs| {
        // Obtain Access to Delay Global Data and Adjust Delay
        G_DELAYMS
            .borrow(cs)
            .set(G_DELAYMS.borrow(cs).get()/2);

        if G_DELAYMS.borrow(cs).get() < 125_u32 {
            G_DELAYMS.borrow(cs).set(1000_u32);
        }

        let mut timer = G_TIM.borrow(cs).borrow_mut();
        defmt::info!("Delay Atual: {} ms", G_DELAYMS.borrow(cs).get());
        timer
            .as_mut()
            .unwrap()
            .start(G_DELAYMS.borrow(cs).get().ms());

        // Obtain access to Global Button Peripheral and Clear Interrupt Pending Flag
        let mut button = G_BUTTON.borrow(cs).borrow_mut();
        button.as_mut().unwrap().clear_interrupt_pending_bit();
    });
}

// Timer Interrupt
#[interrupt]
fn TIM2() {
    // When Timer Interrupt Happens Two Things Need to be Done
    // 1) Toggle the LED
    // 2) Clear Timer Pending Interrupt

    // Start a Critical Section
    cortex_m::interrupt::free(|cs| {
        // Obtain Access to Delay Global Data and Adjust Delay
        let mut led = G_LED.borrow(cs).borrow_mut();
        led.as_mut().unwrap().toggle().ok();

        // Obtain access to Global Timer Peripheral and Clear Interrupt Pending Flag
        let mut timer = G_TIM.borrow(cs).borrow_mut();
        timer.as_mut().unwrap().clear_interrupt(Event::TimeOut);
    });
}