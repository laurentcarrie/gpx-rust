use polars::prelude::*;

/// look at source file to see the schema
/// - longitude
/// ("x", DataType::Float64),
/// - latitude
/// ("y", DataType::Float64),
/// - distance covered since beginning of workout
/// ("distance", DataType::Float64),
/// - speed in km/h
/// ("speed", DataType::Float64),
/// - heart rate in BPM
/// ("hr", DataType::Float64),
/// - allure in mn/km
/// ("allure", DataType::Float64),
/// - in garmin training, a workout is made of steps, that you can repeat. These are the circuits
/// ("circuit", DataType::UInt32),

pub const TCX_DF_SCHEMA: [(&str, DataType); 7] = [
    ("x", DataType::Float64),
    ("y", DataType::Float64),
    ("distance", DataType::Float64),
    ("speed", DataType::Float64),
    ("hr", DataType::Float64),
    ("allure", DataType::Float64),
    ("circuit", DataType::UInt32),
];

///
pub fn tcx_df_schema() -> Schema {
    Schema::from_iter(
        TCX_DF_SCHEMA
            .iter()
            .map(|(a, b)| Field::new(a.to_string().into(), b.clone()))
            .collect::<Vec<_>>(),
    )
}
