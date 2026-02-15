mod test;
mod server;
mod config;
use std::{fs::File, io::Read};

use crate::config::SERVER_CONFIG;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{:?}", SERVER_CONFIG.listeners);
    Ok(())
}

