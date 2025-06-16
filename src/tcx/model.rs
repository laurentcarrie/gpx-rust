use polars::prelude::*;

/// the schema of a tcx dataframe
const TCX_DF_SCHEMA: [(&str, DataType); 7] = [
    ("x", DataType::Float64),
    ("y", DataType::Float64),
    ("distance", DataType::Float64),
    ("speed", DataType::Float64),
    ("hr", DataType::Float64),
    ("allure", DataType::Float64),
    ("circuit", DataType::UInt32),
];

/// it is not possible to statically type a polars dataframe with a polar schema
/// this documents the output of [crate::tcx::read::get_df]
/// test ensure that this schema is correct with respect to that function
/// look at source file to see the schema
pub fn tcx_df_schema() -> Schema {
    Schema::from_iter(
        TCX_DF_SCHEMA
            .iter()
            .map(|(a, b)| Field::new(a.to_string().into(), b.clone()))
            .collect::<Vec<_>>(),
    )
}
