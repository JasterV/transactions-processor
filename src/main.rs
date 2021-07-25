mod models;
mod services;
use std::env;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let file_path = env::args().nth(1).expect("CSV path required");
    println!("Hello, World");
    Ok(())
}
