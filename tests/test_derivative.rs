#[cfg(test)]
mod tests {
    use assert_float_eq::*;
    use log::LevelFilter;
    use num;
    use partial_application::partial;
    use reikna::derivative::*;
    use reikna::func::*;
    #[test]
    fn test_deriv_1() {
        let _ = simple_logging::log_to_file("/tmp/foo.log", LevelFilter::Info)
            .expect("Failed to initialize logging");
        log::info!("start test derivative");
        let f: Function = Rc::new(|x| x * x);
        let first_deriv = derivative(&f);
        expect_float_relative_eq!(first_deriv(0.0), 0.0);
        expect_float_relative_eq!(first_deriv(100.0), 200.0);
        log::info!("end test derivative");
    }

    fn my_power(x: f64, n: usize) -> f64 {
        num::pow(x, n)
    }

    #[test]
    fn test_deriv_2() {
        let _ = simple_logging::log_to_file("/tmp/foo.log", LevelFilter::Info)
            .expect("Failed to initialize logging");
        log::info!("start test derivative");
        let bar: Function = Rc::new(partial!(my_power => _, 2));
        let first_deriv = derivative(&bar);
        expect_float_relative_eq!(first_deriv(0.0), 0.0);
        expect_float_relative_eq!(first_deriv(100.0), 200.0);
        log::info!("end test derivative");
    }
}
