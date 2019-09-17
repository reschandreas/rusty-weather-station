extern crate linux_embedded_hal as hal;
extern crate bme280;
extern crate chrono;
extern crate bigdecimal;
#[macro_use]
extern crate diesel;
extern crate dotenv;

pub use self::relay::Relay;
pub use self::rain::RainSensor;

use std::error::Error;
use std::thread;
use std::time::Duration;
use rppal::system::DeviceInfo;

mod relay;
mod temperature;
mod rain;
mod db;
mod models;
mod schema;

// Gpio uses BCM
const GPIO_RELAY: u8 = 21;
const GPIO_RAIN_SENSOR: u8 = 18;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Controlling a relay on a {}.", DeviceInfo::new()?.model());

    let temp = temperature::measure_temperature();
    println!("{:?}", temp);

    let mut relay = Relay::new(GPIO_RELAY)?;

    println!("Relay on");
    relay.on();
    thread::sleep(Duration::from_millis(50));
    println!("Relay off");
    relay.off();
 
    let mut rain_sensor = RainSensor::new(GPIO_RAIN_SENSOR)?;
    if rain_sensor.is_raining() {
        println!("It is raining!");
    } else {
        println!("It is not raining!");
    }



    let connection = db::establish_connection();
    db::create_measurement(&connection, temp.0, temp.1, temp.2, rain_sensor.is_raining());

    Ok(())
}
