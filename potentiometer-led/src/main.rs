use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::prelude::*;
use esp_idf_svc::hal::{adc, delay, gpio, ledc};

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

    // reading
    let adc2_driver = adc::oneshot::AdcDriver::new(ps.adc2)?;
    let adc_channel_config = adc::oneshot::config::AdcChannelConfig {
        calibration: true,
        attenuation: adc::attenuation::DB_11,
        ..Default::default()
    };
    let mut potentiometer_channel =
        adc::oneshot::AdcChannelDriver::new(&adc2_driver, ps.pins.gpio4, &adc_channel_config)?;

    // writing

    let timer_conf = ledc::config::TimerConfig {
        frequency: 5.kHz().into(),
        resolution: ledc::Resolution::Bits12,
    };
    let timer_driver = ledc::LedcTimerDriver::new(ps.ledc.timer0, &timer_conf)?;
    let mut pwm_driver = ledc::LedcDriver::new(ps.ledc.channel0, &timer_driver, ps.pins.gpio5)?;
    let max_duty = pwm_driver.get_max_duty();

    // initialize the built-in LED
    let _builtin_led = flash_on_startup(ps.pins.gpio2, &delay)?;

    loop {
        let potentiometer_value = adc2_driver.read(&mut potentiometer_channel)?;

        // ADC on esp32 has some deviation at the lowest and the highest values...
        // since we read u16, not a problem to set the maximum from it, but the
        // lowest value is usually around ~128, so if we see <= 130 just turn it off
        if potentiometer_value <= 130 {
            pwm_driver.set_duty(0)?;
        } else if potentiometer_value as u32 >= max_duty {
            pwm_driver.set_duty(max_duty)?;
        } else {
            pwm_driver.set_duty(potentiometer_value as u32)?;
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
