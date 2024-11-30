use dht_sensor::dht11;
use dht_sensor::DhtReading;
use esp_idf_svc::hal::delay;
use esp_idf_svc::hal::gpio;
use esp_idf_svc::hal::peripherals::Peripherals;

fn main() -> Result<(), anyhow::Error> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // code

    log::info!("ESP32 is starting");

    let ps = Peripherals::take()?;
    let mut delay = delay::Delay::default();

    let mut led = gpio::PinDriver::output(ps.pins.gpio2)?;
    let mut th_pin = gpio::PinDriver::input_output_od(ps.pins.gpio4)?;
    th_pin.set_pull(gpio::Pull::Up)?;

    delay.delay_ms(1000);

    loop {
        led.set_high()?;

        let th_readings = dht11::Reading::read(&mut delay, &mut th_pin)
            .map_err(|e| anyhow::anyhow!("Failed to read temperature: {:?}", e))?;

        log::info!("Temperature and humidity are: {:?}", th_readings);

        led.set_low()?;

        delay.delay_ms(3000);
    }
}
