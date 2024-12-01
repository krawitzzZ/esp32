# ESP32

This is a repository with some home projects with
[ESP32 DevKitV1](https://www.circuitstate.com/pinouts/doit-esp32-devkit-v1-wifi-development-board-pinout-diagram-and-reference/)

Main purpose of this repository is to get familiar with embedded development
with rust

## Projects

### temperature

Connects the [DHP11](https://components101.com/sensors/dht11-temperature-sensor)
to the ESP32 board and outputs the current temperature and
humidity to the serial

#### Result temperature

![Result](./temperature/media/temperature.jpg)

### push-button-led

Utilizes 2 pins (4th as an input and 5th as an output) to read the button state
and if the button is pressed put voltage to the LED

#### Result push-button-led

![Result](./push-button-led/media/result.gif)
