use argh::FromArgs;
use lc_gpx_utils::course::xcourse::{get_gpx, write_gpx};
use log4rs::{
    append::{
        // console::{ConsoleAppender, Target},
        file::FileAppender,
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    // filter::threshold::ThresholdFilter,
};
use std::path::PathBuf;

/// Demo
#[derive(Debug, FromArgs)]
struct Cli {
    #[argh(positional)]
    tcx_file: String,
    #[argh(positional)]
    gpx_file: String,
}

// use polars::lazy::dsl::{col, lit};

fn main() {
    let level = log::LevelFilter::Info;
    let file_path = "/tmp/foo.log";

    let cli: Cli = argh::from_env();

    // Build a stderr logger.
    // let stderr = ConsoleAppender::builder().target(Target::Stderr).build();

    // Logging to log file.
    let logfile = FileAppender::builder()
        // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build(file_path)
        .unwrap();

    // Log Trace level output to file where trace is the default level
    // and the programmatically specified level to stderr.
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        // .appender(
        //     Appender::builder()
        //         .filter(Box::new(ThresholdFilter::new(level)))
        //         .build("stderr", Box::new(stderr)),
        // )
        .build(
            Root::builder()
                .appender("logfile")
                // .appender("stderr")
                // .build(LevelFilter::Trace),
                .build(level),
        )
        .unwrap();

    let _handle = log4rs::init_config(config).expect("Failed to initialize log4rs");

    log::info!("Begin working on tcx file .");

    let gpx = get_gpx(cli.tcx_file).expect("Failed to create courses");
    let p = PathBuf::from(&cli.gpx_file);
    write_gpx(&gpx, &p).expect("Failed to write GPX file");
    log::info!("Finished.");
}
