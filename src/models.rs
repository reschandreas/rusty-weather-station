use bigdecimal::BigDecimal;
use super::schema::measurements;

#[derive(Queryable)]
pub struct Measurements {
    pub id: i32,
    pub time: chrono::NaiveDateTime,
    pub temperature: BigDecimal,
    pub humidity: BigDecimal,
    pub pressure: BigDecimal,
    pub is_raining: bool,
}

#[derive(Insertable)]
#[table_name = "measurements"]
pub struct NewMeasurement<'a> {
    pub temperature: &'a BigDecimal,
    pub humidity: &'a BigDecimal,
    pub pressure: &'a BigDecimal,
    pub is_raining: bool,
}
