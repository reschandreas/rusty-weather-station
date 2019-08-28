use std::error::Error;
use rppal::gpio::Gpio;
use rppal::gpio::InputPin;

pub struct RainSensor {
    bmc_pin: u8,
    input: InputPin,
}

impl RainSensor {

    pub fn new(_bcm_pin: u8) -> Result<RainSensor, Box<dyn Error>> {
        Ok(RainSensor {bmc_pin: _bcm_pin, input: Gpio::new()?.get(_bcm_pin)?.into_input()})
    }

    pub fn is_raining(&mut self) -> bool {
        self.input.is_low()
    }
}
