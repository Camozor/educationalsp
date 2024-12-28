use ::core::str;
use log::info;
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
        let mut buffer = vec![];
        stdin.read(&buffer);
        let read_bytes = stdin.read_to_end(&mut buffer);
        println!("Read bytes = {}", read_bytes.unwrap());

        if let Ok(string) = str::from_utf8(&buffer) {
            info!("{}", string.len());
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
