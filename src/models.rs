use bigdecimal::BigDecimal;
use super::schema::measurements;
use serde::{Serialize, Deserialize};

#[derive(Queryable)]
pub struct Measurements {
    pub id: i32,
    pub time: chrono::NaiveDateTime,
    pub temperature: BigDecimal,
    pub humidity: BigDecimal,
    pub pressure: BigDecimal,
    pub is_raining: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Measurement {
    pub time: chrono::NaiveDateTime,
    pub temperature: f32,
    pub humidity: f32,
    pub pressure: f32,
    pub is_raining: bool,
}

#[derive(Insertable)]
#[table_name = "measurements"]
pub struct NewMeasurement<'a> {
    pub time: chrono::NaiveDateTime,
    pub temperature: &'a BigDecimal,
    pub humidity: &'a BigDecimal,
    pub pressure: &'a BigDecimal,
    pub is_raining: bool,
}
