use std::error::Error;
use rppal::gpio::Gpio;
use rppal::gpio::OutputPin;

pub struct Relay {
    bmc_pin: u8,
    output: OutputPin,
}

impl Relay {

    pub fn new(_bcm_pin: u8) -> Result<Relay, Box<dyn Error>> {
        Ok(Relay {bmc_pin: _bcm_pin, output: Gpio::new()?.get(_bcm_pin)?.into_output()})
    }
    
    pub fn on(&mut self) -> () {
        if !self.output.is_set_high() {
            self.output.set_high();
        }
    }
    
    pub fn off(&mut self) -> () {
        if !self.output.is_set_low() {
            self.output.set_low();
        }
    }

    pub fn toggle(&mut self) -> () {
        self.output.toggle();
    }
}
