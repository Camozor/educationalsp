use ::core::str;
use log::{error, info};
use std::{
    io::{self, Read},
    process::exit,
    time::SystemTime,
};

pub mod core;

fn main() -> io::Result<()> {
    if let Err(_) = setup_logger() {
        eprintln!("Error setting up logger");
        exit(1);
    }

    info!("started");
    let mut stdin = io::stdin();

    loop {
        let mut buffer: [u8; 5000] = [0; 5000];
        let size = stdin.read(&mut buffer);

        match size {
            Ok(size) => info!("Message size: {}", size),
            Err(a) => error!("{}", a),
        }

        if let Ok(string) = str::from_utf8(&buffer) {
            info!("my string: {}", string);
        }
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
