use core::{
    logger::setup_logger,
    rpc::{get_content, get_content_length, get_message},
};
use log::{error, info};
use std::{
    io::{self, Read},
    process::exit,
};

pub mod core;

fn main() -> io::Result<()> {
    if let Err(_) = setup_logger() {
        eprintln!("Error setting up logger");
        exit(1);
    }

    info!("Language server started!");
    let mut stdin = io::stdin();

    loop {
        let mut buffer: [u8; 5000] = [0; 5000];
        let size = stdin.read(&mut buffer);

        if let Err(_) = size {
            error!("Could not read message");
            continue;
        }
        let size = size.unwrap();

        let message = get_message(&buffer, size);
        let content_length = get_content_length(message);
        info!("Content length: {}", content_length);

        let content = get_content(message);
        info!("Content: {}", content);
    }
}
