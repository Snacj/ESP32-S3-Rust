use esp_idf_hal::prelude::*;
use esp_idf_hal::i2c::*;
use esp_idf_hal::gpio::PinDriver;
use esp_idf_svc::sys::link_patches;
use esp_idf_svc::log::EspLogger;

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

fn main() {
    link_patches();
    EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let config = I2cConfig::new().baudrate(400.kHz().into());
    let i2c = I2cDriver::new(
        peripherals.i2c0,
        peripherals.pins.gpio17, // SDA
        peripherals.pins.gpio18, // SCL
        &config,
    ).unwrap();

    // Hardware reset pin for display
    let mut rst = PinDriver::output(peripherals.pins.gpio21).unwrap();
    rst.set_low().unwrap();
    std::thread::sleep(std::time::Duration::from_millis(10));
    rst.set_high().unwrap();

    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    display.init().unwrap();
    display.clear(BinaryColor::Off).unwrap();

    let style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
    Text::new("Hello, world!", Point::new(0, 16), style)
        .draw(&mut display)
        .unwrap();

    display.flush().unwrap();

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

