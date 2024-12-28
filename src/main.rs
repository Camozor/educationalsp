use log::info;
use std::{io, process::exit, time::SystemTime};

pub mod core;

fn main() -> io::Result<()> {
    if let Err(_) = setup_logger() {
        eprintln!("Error setting up logger");
        exit(1);
    }

    info!("started");
    let stdin = io::stdin();

    loop {
        let mut buffer = String::new();
        let size = stdin.read_line(&mut buffer)?;

        info!("{}", size);
        info!("Content: {}", buffer);
    }
}

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("/tmp/educationalsp.log")?)
        .apply()?;
    Ok(())
}
