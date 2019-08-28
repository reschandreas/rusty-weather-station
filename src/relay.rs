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
    
    pub fn on(&mut self) -> Result<(), Box<dyn Error>> {
        if !self.output.is_set_high() {
            self.output.set_high();
        }
        
        Ok(())
    }
    
    pub fn off(&mut self) -> Result<(), Box<dyn Error>> {
        if !self.output.is_set_low() {
            self.output.set_low();
        }

        Ok(())
    }

    pub fn toggle(&mut self) -> Result<(), Box<dyn Error>> {
        self.output.toggle();

        Ok(())
    }
}
