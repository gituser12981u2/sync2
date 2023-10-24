// src/main.rs

mod data;
mod transport;

use std::{env, io, path::PathBuf, time::Instant};

use crate::data::file::{receive_file, send_file};

const ADDRESS: &str = "127.0.0.1:8080";

#[tokio::main]
async fn main() -> io::Result<()> {
    env_logger::init();
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: <program> [send|receive]");
        return Ok(());
    }

    let start = Instant::now();
    match args[1].as_str() {
        "send" => {
            let file_path = PathBuf::from(&args[2]);

            send_file(ADDRESS, &file_path).await?;
        }
        "receive" => {
            let dest_file_path = PathBuf::from(&args[2]);

            receive_file(ADDRESS, &dest_file_path).await?;
        }
        _ => {
            eprintln!("Unknown command. Please use either 'send' or 'receive'.");
            return Ok(());
        }
    }
    let duration = start.elapsed();
    println!("Time taken for transfer: {:?}", duration);

    Ok(())
}
