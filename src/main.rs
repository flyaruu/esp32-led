#![no_std]
#![no_main]

extern crate alloc;
use core::mem::MaybeUninit;
use esp_backtrace as _;
use esp_println::println;
use hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, Delay, ledc::{LEDC, LSGlobalClkSource, LowSpeed, timer, channel::{self, config::PinConfig}}, IO};

use esp_wifi::{initialize, EspWifiInitFor};

use hal::{systimer::SystemTimer, Rng};

use crate::led::LedComponent;

mod led;

#[global_allocator]
static ALLOCATOR: esp_alloc::EspHeap = esp_alloc::EspHeap::empty();

fn init_heap() {
    const HEAP_SIZE: usize = 32 * 1024;
    static mut HEAP: MaybeUninit<[u8; HEAP_SIZE]> = MaybeUninit::uninit();

    unsafe {
        ALLOCATOR.init(HEAP.as_mut_ptr() as *mut u8, HEAP_SIZE);
    }
}
#[entry]
fn main() -> ! {
    init_heap();
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    let io = IO::new(peripherals.GPIO,peripherals.IO_MUX);

    let red_pin = io.pins.gpio3.into_push_pull_output();
    let green_pin = io.pins.gpio4.into_push_pull_output();
    let blue_pin = io.pins.gpio5.into_push_pull_output();

    let mut ledc = LEDC::new(peripherals.LEDC, &clocks);
    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);
    
    let mut lstimer0 = ledc.get_timer::<LowSpeed>(timer::Number::Timer0);
    lstimer0
        .configure(timer::config::Config {
            duty: timer::config::Duty::Duty5Bit,
            clock_source: timer::LSClockSource::APBClk,
            frequency: 24u32.kHz(),
        })
        .unwrap();
    
    let mut channel0 = ledc.get_channel(channel::Number::Channel0, red_pin);
    channel0
        .configure(hal::ledc::channel::config::Config {
            timer: &lstimer0,
            duty_pct: 100,
            pin_config: PinConfig::PushPull,
        })
        .unwrap();

    let mut channel1 = ledc.get_channel(channel::Number::Channel1, green_pin);
    channel1
        .configure(hal::ledc::channel::config::Config {
            timer: &lstimer0,
            duty_pct: 100,
            pin_config: PinConfig::PushPull,
        })
        .unwrap();
    let mut channel2 = ledc.get_channel(channel::Number::Channel2, blue_pin);
    channel2
        .configure(hal::ledc::channel::config::Config {
            timer: &lstimer0,
            duty_pct: 0,
            pin_config: PinConfig::PushPull,
        })
        .unwrap();

    let mut component = LedComponent::new(channel0, channel1, channel2);


    // setup logger
    // To change the log_level change the env section in .cargo/config.toml
    // or remove it and set ESP_LOGLEVEL manually before running cargo run
    // this requires a clean rebuild because of https://github.com/rust-lang/cargo/issues/10358
    esp_println::logger::init_logger_from_env();
    log::info!("Logger is setup");
    println!("Hello world!");
    let timer = SystemTimer::new(peripherals.SYSTIMER).alarm0;
    let _init = initialize(
        EspWifiInitFor::Wifi,
        timer,
        Rng::new(peripherals.RNG),
        system.radio_clock_control,
        &clocks,
    )
    .unwrap();
    loop {
        println!("Loop...");
        delay.delay_ms(500u32);
        component.set_color(100, 0, 0);
        delay.delay_ms(500u32);
        component.set_color(100, 100, 100);
        delay.delay_ms(500u32);
        component.set_color(0, 0, 100);
        delay.delay_ms(500u32);
    }
}
