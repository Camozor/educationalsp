use core::{
    logger::setup_logger,
    rpc::{decode, encode},
    serialization::{ResponseMessage, ResponseResult, ServerCapabilities},
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

        let client_request = decode(&buffer, size);
        if client_request.method == "initialize" {
            info!("Initialize request!");
            let response = ResponseMessage {
                id: client_request.id,
                result: Some(ResponseResult {
                    capabilities: ServerCapabilities {},
                }),
            };
            let encoded_response = encode(&response);
            println!("{}", encoded_response);
            info!("initialize response sent: {}", encoded_response);
        } else {
            info!("Other request!");
        }
    }
}
