use crate::{
    error::DBError,
    pager::{OpenMode, Pager},
};
use std::env;

mod error;
mod pager;

static PAGE_SIZE: u64 = 4096;

fn main() -> Result<(), DBError> {
    let args: Vec<String> = env::args().collect();

    let mode = match args.get(1).map(|s| s.as_str()) {
        Some("create") => OpenMode::Create,
        Some("open") => OpenMode::Open,
        _ => {
            eprintln!("Uso: bdbee [create|open]");
            std::process::exit(1);
        }
    };

    Pager::open("data.db", PAGE_SIZE, mode).expect("Falha ao iniciar banco");
    Ok(())
}
