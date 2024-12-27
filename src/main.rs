use std::io;

pub mod core;

fn main() -> io::Result<()> {
    init_logger();
    println!("started");
    let stdin = io::stdin();

    loop {
        let mut buffer = String::new();
        let size = stdin.read_line(&mut buffer)?;

        println!("{}", size);
        println!("{}", buffer);
    }
}

fn init_logger() {}
