table! {
    measurements (id) {
        id -> Int4,
        time -> Timestamptz,
        temperature -> Numeric,
        humidity -> Numeric,
        pressure -> Numeric,
        is_raining -> Bool,
    }
}
