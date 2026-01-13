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

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let mut led = Output::new(peripherals.GPIO9, Level::High, OutputConfig::default());
    let start_time = Instant::now();
    let mut next_output_time = Instant::now() + Duration::from_millis(1000);
    loop {
        if Instant::now() >= next_output_time {
            led.toggle();
            let elapsed = start_time.elapsed();
            println!(
                "Uptime: {:02}:{:02}:{:02}.{:03}",
                elapsed.as_hours() % 24,
                elapsed.as_minutes() % 60,
                elapsed.as_secs() % 60,
                elapsed.as_millis() % 1000
            );
            next_output_time += Duration::from_millis(1000);
        }
    }
}
