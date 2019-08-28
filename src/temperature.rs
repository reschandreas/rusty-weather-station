extern crate linux_embedded_hal as hal;
extern crate bme280;

use hal::{Delay, I2cdev};
use bme280::BME280;

pub fn measure_temperature() -> (f32, f32, f32) {
    let i2c_bus = I2cdev::new("/dev/i2c-1").unwrap();

    let mut bme280 = BME280::new_primary(i2c_bus, Delay);

    bme280.init().unwrap();

    let measurements = bme280.measure().unwrap();

    (measurements.humidity, measurements.temperature, measurements.pressure)
}
