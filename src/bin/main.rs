#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{delay::Delay, main};
use esp_println::println;

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    esp_hal::init(esp_hal::Config::default());

    let delay = Delay::new();

    loop {
        println!("Hello world!");
        delay.delay_millis(500);
    }
}
