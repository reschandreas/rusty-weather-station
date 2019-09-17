use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use crate::models::{NewMeasurement, Measurements};
use bigdecimal::BigDecimal;
use bigdecimal::FromPrimitive;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_measurement(conn: &PgConnection, humidity: f32, temperature: f32, pressure: f32, is_raining: bool) -> Measurements {
    use crate::schema::measurements;

    let new_measurement = NewMeasurement {
        temperature: &BigDecimal::from_f32(temperature).unwrap(),
        humidity: &BigDecimal::from_f32(humidity).unwrap(),
        pressure: &BigDecimal::from_f32(pressure).unwrap(),
        is_raining: is_raining == true,
    };

    diesel::insert_into(measurements::table)
        .values(new_measurement)
        .get_result(conn)
        .expect("Error saving new measurement")
}
