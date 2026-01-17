use crate::{
    error::DBError,
    pager::{OpenMode, Pager},
};

mod error;
mod pager;

static PAGE_SIZE: u64 = 4096;

fn main() -> Result<(), DBError> {
    println!("Iniciando bdbee...");
    let _pager =
        Pager::open("data.db", PAGE_SIZE, OpenMode::Create).expect("Falha ao iniciar banco");
    println!("Banco iniciado com sucesso");
    Ok(())
}
