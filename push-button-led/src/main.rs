use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::{delay, gpio};

fn main() -> Result<(), anyhow::Error> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // code

    log::info!("starting ESP32...");

    let ps = Peripherals::take()?;
    let delay = delay::Delay::default();

    let mut built_in_led = gpio::PinDriver::output(ps.pins.gpio2)?;

    let button_pin = gpio::PinDriver::input(ps.pins.gpio4)?;

    let mut led_pin = gpio::PinDriver::input_output(ps.pins.gpio5)?;
    led_pin.set_pull(gpio::Pull::Down)?;

    built_in_led.set_high()?;
    log::info!("ESP32 started!");

    delay.delay_ms(100);

    loop {
        let button_pin_level = button_pin.get_level();

        if led_pin.get_level() != button_pin_level {
            led_pin.set_level(button_pin_level)?;
        }

        delay.delay_ms(1);
    }
}
