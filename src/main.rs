use core::{logger::setup_logger, rpc::decode};
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

        let client_request = decode(&buffer, size);
        if client_request.method == "initialize" {
            info!("Initialize request!");
        } else {
            info!("Other request!");
        }
    }
}
