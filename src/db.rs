use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use crate::models::{NewMeasurement, Measurements, Measurement};
use bigdecimal::BigDecimal;
use bigdecimal::FromPrimitive;
use chrono::NaiveDateTime;
use chrono::Utc;
use std::fs::OpenOptions;
use std::io::prelude::*;
use serde_json::Result;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_measurement(conn: &PgConnection, humidity: f32, temperature: f32, pressure: f32, is_raining: bool) -> Measurements {
    use crate::schema::measurements;
    
    let measurement = Measurement {
        time: Utc::now().naive_local(),
        temperature: temperature,
        humidity: humidity,
        pressure: pressure,
        is_raining: is_raining == true,
    };

    let new_measurement = NewMeasurement {
        temperature: &BigDecimal::from_f32(measurement.temperature).unwrap(),
        time: measurement.time,
        humidity: &BigDecimal::from_f32(measurement.humidity).unwrap(),
        pressure: &BigDecimal::from_f32(measurement.pressure).unwrap(),
        is_raining: measurement.is_raining,
    };

    clear_cache(conn);

    let done: Measurements = diesel::insert_into(measurements::table)
        .values(new_measurement)
        .get_result(conn)
        .expect("Error saving new measurement");

    if done.id < 1 {
        cache_measurement(measurement);
    }

    done
}

fn clear_cache(conn: &PgConnection) {
    use crate::schema::measurements;
    
    let mut file = File::open("/etc/rsensor/tempcache.json").unwrap();
    let reader = BufReader::new(&file);

    for line in reader.lines() {
        let measurement: Measurement = serde_json::from_str(&line.unwrap()).unwrap();
        let new_measurement = NewMeasurement {
            time: measurement.time,
            temperature: &BigDecimal::from_f32(measurement.temperature).unwrap(),
            humidity: &BigDecimal::from_f32(measurement.humidity).unwrap(),
            pressure: &BigDecimal::from_f32(measurement.pressure).unwrap(),
            is_raining: measurement.is_raining,
        };

        diesel::insert_into(measurements::table)
            .values(new_measurement)
            .execute(conn)
            .expect("Error saving cached measurement");
    }
    file.write_all(b"");
}

fn cache_measurement(measurement: Measurement) {
    let serialized = serde_json::to_string(&measurement).unwrap();
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("/etc/rsensor/tempcache.json")
        .unwrap();

    if let Err(e) = writeln!(file, "{}", serialized) {
        eprintln!("Couldn't write to file: {}", e);
    }
}
