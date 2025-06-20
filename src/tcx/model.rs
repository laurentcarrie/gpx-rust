use polars::prelude::*;

/// the schema of a tcx dataframe
const TCX_DF_SCHEMA: [(&str, DataType); 8] = [
    ("x", DataType::Float64),
    ("y", DataType::Float64),
    ("distance", DataType::Float64),
    ("speed", DataType::Float64),
    ("hr", DataType::Float64),
    ("allure", DataType::Float64),
    ("circuit", DataType::UInt32),
    ("time", DataType::Int64),
];

/// the schema of a dataframe obtained from a Garmin tcx file
pub fn tcx_df_schema() -> Schema {
    Schema::from_iter(
        TCX_DF_SCHEMA
            .iter()
            .map(|(a, b)| Field::new(a.to_string().into(), b.clone()))
            .collect::<Vec<_>>(),
    )
}
