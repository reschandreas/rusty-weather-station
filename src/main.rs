use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::system::DeviceInfo;

extern crate linux_embedded_hal as hal;
extern crate bme280;

use hal::{Delay, I2cdev};
use bme280::BME280;
// Gpio uses BCM
const GPIO_RELAY: u8 = 21;
const GPIO_RAIN_SENSOR: u8 = 18;

pub use self::relay::Relay;
mod relay;
mod temperature;

pub use self::rain::RainSensor;
mod rain;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Controlling a relay on a {}.", DeviceInfo::new()?.model());

    //read_temperature();
    println!("{:?}", temperature::measure_temperature());
    println!("Relay on");

    let mut relay = Relay::new(GPIO_RELAY)?;
    relay.on();
    thread::sleep(Duration::from_millis(5000));
    println!("Relay off");
    relay.off();

    let mut rain_sensor = RainSensor::new(GPIO_RAIN_SENSOR)?;
    if rain_sensor.is_raining() {
        println!("It is raining!");
    } else {
        println!("It is not raining!");
    }
    Ok(())
}
