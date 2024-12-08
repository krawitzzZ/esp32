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

    let mut p26 = gpio::PinDriver::input_output(ps.pins.gpio26)?;
    let p27 = gpio::PinDriver::input(ps.pins.gpio27)?;
    p26.set_level(gpio::Level::Low)?;

    // initialize the built-in LED
    let _builtin_led = flash_on_startup(ps.pins.gpio2, &delay)?;

    loop {
        let motion_sensor_lvl = p27.get_level();

        log::info!("motion level is {motion_sensor_lvl:?}");

        if motion_sensor_lvl != p26.get_level() {
            p26.set_level(motion_sensor_lvl)?;
        }

        delay.delay_ms(1);
    }
}

fn flash_on_startup(
    pin: gpio::Gpio2,
    delay: &delay::Delay,
) -> Result<gpio::PinDriver<gpio::Gpio2, gpio::Output>, anyhow::Error> {
    let mut led = gpio::PinDriver::output(pin)?;
    led.set_low()?;

    for _ in 0..2 {
        led.set_high()?;
        delay.delay_ms(300);
        led.set_low()?;
        delay.delay_ms(300);
    }

    led.set_high()?;

    log::info!("ESP32 started!");

    Ok(led)
}
