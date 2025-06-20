// #[macro_use]
use reikna::derivative::*;
use reikna::func::*;

use polars::prelude::*;

pub fn add_derivate_column(
    df: &mut DataFrame,
    column_name: &str,
    new_column_name: &str,
) -> Result<DataFrame, Box<dyn std::error::Error>> {
    let dfx = df
        .clone()
        .lazy()
        .sort(vec!["time"], SortMultipleOptions::default())
        // .limit(10)
        .select(vec![col("time"), col(column_name)])
        .collect()?;

    let objects = &dfx.clone().take_columns();
    let mut itert = objects[0].f64()?.iter();
    let mut iterf = objects[1].f64()?.iter();

    let mut vd: Vec<f64> = vec![];
    let mut y0 = iterf.next().flatten().ok_or("series is too short")?;
    let mut y1 = iterf.next().flatten().ok_or("series is too short")?;
    let mut y2 = iterf.next().flatten().ok_or("series is too short")?;
    let dt = 1.0; // 1s

    // let f = func![|x| x * x];
    let f: Function = Rc::new(|x| x * x);
    let first_deriv = derivative(&f);

    //
    let values = itert.map(|t| first_deriv(t.unwrap())).collect::<Vec<f64>>();
    // let values = itert.map(|t| t.unwrap()).collect::<Vec<f64>>();

    // loop {
    //     // let v = (d1 - d0) / ((t1 - t0) as f64);
    //     // speed0.push(kv * v);
    //     // allure0.push(ka / v);

    //     let dy = (-3.0 * y0 + 4.0 * y1 - y2) / 2.0 / dt;
    //     vd.push(dy);

    //     match iterf.next().flatten() {
    //         None => break,
    //         Some(new_y) => {
    //             y0 = y1;
    //             y1 = y2;
    //             y2 = new_y
    //         }
    //     }
    // }

    let col = Column::new(new_column_name.into(), vd);
    let df = df.with_column(col)?;

    Ok(df.clone())
}
