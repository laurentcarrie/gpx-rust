use control_sys::model;
// use control_sys::model::Discrete;
// use control_sys::model::DiscreteStateSpaceModel;
use control_sys::model::StateSpaceModel;
// use log::LevelFilter;
use nalgebra as na;
// use polars::prelude::Series;
use polars::prelude::*;

pub fn add_filter_columns(
    df: &mut DataFrame,
    column_name: &str,
    new_column_name: &str,
    time_constant: f64,
    sampling_rate: f64,
) -> Result<DataFrame, Box<dyn std::error::Error>> {
    let mat_ac = na::dmatrix![-1.0 / time_constant];
    let mat_bc = na::dmatrix![1.0 / time_constant];
    let mat_cc = na::dmatrix![1.0];
    let mat_dc = na::dmatrix![0.0];
    let cont_model =
        model::ContinuousStateSpaceModel::from_matrices(&mat_ac, &mat_bc, &mat_cc, &mat_dc);
    log::info!("{}:{} {:?}", file!(), line!(), &cont_model);

    let discrete_model = model::DiscreteStateSpaceModel::from_continuous_ss_forward_euler(
        &cont_model,
        sampling_rate,
    );
    log::info!("{}:{} {:?}", file!(), line!(), &discrete_model);
    log::info!("{:?}", &discrete_model.mat_a().data);
    assert_eq!(discrete_model.mat_a().nrows(), 1);
    assert_eq!(discrete_model.mat_a().ncols(), 1);
    assert_eq!(discrete_model.mat_b().nrows(), 1);
    assert_eq!(discrete_model.mat_b().ncols(), 1);
    assert_eq!(discrete_model.mat_c().nrows(), 1);
    assert_eq!(discrete_model.mat_c().ncols(), 1);
    assert_eq!(discrete_model.mat_d().nrows(), 1);
    assert_eq!(discrete_model.mat_d().ncols(), 1);

    // X = A X + B
    // S = C X + D

    let a = discrete_model.mat_a().get(0).unwrap();
    let b = discrete_model.mat_b().get(0).unwrap();
    let c = discrete_model.mat_c().get(0).unwrap();
    let d = discrete_model.mat_d().get(0).unwrap();
    log::info!("{}:{} {} {} {} {} ", file!(), line!(), &a, &b, &c, &d);

    let dfx = df
        .clone()
        .lazy()
        .sort(vec!["time"], SortMultipleOptions::default())
        // .limit(10)
        .select(vec![col(column_name)])
        .collect()?;

    // log::info!("{}:{} {:?}", file!(), line!(), &dfx);

    let objects = &dfx.clone().take_columns();
    let iteru = objects[0].f64()?.iter();

    let mut x0 = 0.0;
    let mut vx = vec![0.0];
    let mut vs = vec![0.0];
    for u in iteru {
        let newx = a * x0 + b * u.expect("u");
        let s = c * x0 + d * u.expect("u");
        // let s = 42.0;
        x0 = newx;
        vx.push(newx);
        vs.push(s);
    }
    vs.pop();

    let col = Column::new(new_column_name.into(), vs);
    let df = df.with_column(col)?;

    Ok(df.clone())
}
