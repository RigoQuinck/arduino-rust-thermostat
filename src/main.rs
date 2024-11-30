#![no_std]
#![no_main]
#![feature(core_intrinsics)]

// ####################################
// # THERMOSTAT
// ####################################

use core::intrinsics::logf32;

use arduino_hal::hal::port::PC0;
use arduino_hal::port::Pin;

use arduino_hal::port::mode::Analog;
use arduino_hal::{delay_ms, entry, pins, Adc, Peripherals};
use panic_halt as _;

const HEATING_THRESHOLD: f32 = 18.0;
const COOLING_THRESHOLD: f32 = 30.0;

enum State {
    Idle,
    Heating,
    Cooling,
}

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = pins!(dp);

    let mut heating_led = pins.d3.into_output();
    let mut idle_led = pins.d4.into_output();
    let mut cooling_led = pins.d5.into_output();

    let mut adc = Adc::new(dp.ADC, Default::default());
    let temp_sensor = pins.a0.into_analog_input(&mut adc);

    let mut state = State::Idle;
    heating_led.set_low();
    cooling_led.set_low();
    idle_led.set_high();

    loop {
        let temperature = read_temperature(&temp_sensor, &mut adc);

        match (&mut state, temperature) {
            (State::Idle, t) if t <= HEATING_THRESHOLD => {
                idle_led.set_low();
                heating_led.set_high();
                state = State::Heating;
            }
            (State::Idle, t) if t >= COOLING_THRESHOLD => {
                idle_led.set_low();
                cooling_led.set_high();
                state = State::Cooling;
            }
            (State::Heating, t) if t > HEATING_THRESHOLD => {
                idle_led.set_high();
                heating_led.set_low();
                state = State::Idle;
            }
            (State::Cooling, t) if t < COOLING_THRESHOLD => {
                idle_led.set_high();
                cooling_led.set_low();
                state = State::Idle;
            }
            _ => {}
        }

        delay_ms(100);
    }
}

fn read_temperature(temp_sensor: &Pin<Analog, PC0>, adc: &mut Adc) -> f32 {
    let beta = 3950.0;
    let volatge = temp_sensor.analog_read(adc) as f32;
    let celsius =
        1.0 / (unsafe { logf32(1.0 / (1023.0 / volatge - 1.0)) } / beta + 1.0 / 298.15) - 273.15;
    celsius
}
