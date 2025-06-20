use argh::FromArgs;
use control_sys::model;
// use control_sys::model::Discrete;
// use control_sys::model::DiscreteStateSpaceModel;
use control_sys::model::StateSpaceModel;
use log::LevelFilter;
use nalgebra as na;
use plotlars::{LinePlot, Plot};
use polars::prelude::Series;
use polars::prelude::*;

#[derive(Debug, FromArgs)]
#[argh(description = "generate html plot from garmin tcx file")]
struct Cli {
    // #[argh(positional, description = "garmin tcx file to read")]
    // tcx_path: String,
    #[argh(positional, description = "time constant")]
    time_constant: f64,
    #[argh(positional, description = "sampling")]
    sampling_rate: f64,
    #[argh(positional, description = "output html file")]
    html_path: String,
}

fn main() {
    let _ = simple_logging::log_to_file("/tmp/foo.log", LevelFilter::Info)
        .expect("Failed to initialize logging");
    log::info!("start test plot");
    let cli: Cli = argh::from_env();

    let mat_ac = na::dmatrix![-cli.time_constant];
    let mat_bc = na::dmatrix![cli.time_constant];
    let mat_cc = na::dmatrix![1.0];
    let mat_dc = na::dmatrix![0.0];
    let cont_model =
        model::ContinuousStateSpaceModel::from_matrices(&mat_ac, &mat_bc, &mat_cc, &mat_dc);
    log::info!("{}:{} {:?}", file!(), line!(), &cont_model);

    let discrete_model = model::DiscreteStateSpaceModel::from_continuous_ss_forward_euler(
        &cont_model,
        cli.sampling_rate,
    );
    log::info!("{}:{} {:?}", file!(), line!(), &discrete_model);
    log::info!("{:?}", &discrete_model.mat_a().data);
    assert_eq!(discrete_model.mat_a().nrows(), 1 as usize);
    assert_eq!(discrete_model.mat_a().ncols(), 1 as usize);
    assert_eq!(discrete_model.mat_b().nrows(), 1 as usize);
    assert_eq!(discrete_model.mat_b().ncols(), 1 as usize);
    assert_eq!(discrete_model.mat_c().nrows(), 1 as usize);
    assert_eq!(discrete_model.mat_c().ncols(), 1 as usize);
    assert_eq!(discrete_model.mat_d().nrows(), 1 as usize);
    assert_eq!(discrete_model.mat_d().ncols(), 1 as usize);

    // X = A X + B
    // S = C X + D

    let a = discrete_model.mat_a().get(0).unwrap();
    let b = discrete_model.mat_b().get(0).unwrap();
    let c = discrete_model.mat_c().get(0).unwrap();
    let d = discrete_model.mat_d().get(0).unwrap();

    let imax = 1000;
    let vt = (0..imax).map(|t| t as f64).collect::<Vec<_>>();
    let vu = (0..imax).map(|_t| 1 as f64).collect::<Vec<_>>();
    let mut vx: Vec<f64> = vec![0.0];
    let mut vs: Vec<f64> = vec![0.0];
    for index in 0..imax - 1 {
        let u = vu.get(index).unwrap();
        let x = vx.get(index).unwrap();

        let s = c * x + d * u;
        vs.push(s);
        let x = a * x + b * u;
        vx.push(x);
    }
    assert_eq!(vu.len(), imax);
    assert_eq!(vx.len(), imax);
    assert_eq!(vs.len(), imax);

    let df = DataFrame::new(vec![
        Series::new("t".into(), vt).into(),
        Series::new("u".into(), vu).into(),
        Series::new("x".into(), vx).into(),
        Series::new("s".into(), vs).into(),
    ])
    .unwrap();

    let _ = LinePlot::builder()
        .data(&df)
        .x("t")
        .y("u")
        .additional_lines(vec!["s"])
        .size(12)
        .plot_title("1st order")
        .x_title("time")
        .y_title("x")
        .legend_title("Allure")
        .build()
        .write_html(cli.html_path);

    log::info!("{}:{} end of test", file!(), line!());
}
