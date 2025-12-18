#![feature(seek_stream_len)]

mod error;
mod pager;

static PAGE_SIZE: u64 = 4096;

fn main() {
    println!("Iniciando bdbee...");
}
