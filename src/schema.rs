table! {
    measurements (id) {
        id -> Int4,
        time -> Timestamp,
        temperature -> Numeric,
        humidity -> Numeric,
        pressure -> Numeric,
        is_raining -> Bool,
    }
}
