#![no_std]
#![no_main]

use esp_hal::{
    clock::CpuClock,
    gpio::{Level, Output, OutputConfig},
    main,
    time::{Duration, Instant},
};
use esp_println::println;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    
    let mut led = Output::new(peripherals.GPIO0, Level::High, OutputConfig::default());

    loop {
        led.toggle();
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(500) {};
        println!("LED toggled2");
    }
}