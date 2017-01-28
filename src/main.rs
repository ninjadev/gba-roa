#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate nom;

#[macro_use]
extern crate enum_primitive;

extern crate num;

mod cpu;
mod instruction;
mod opcode;
mod parser;

mod errors {
    error_chain!{}
}
use errors::*;
quick_main!(run);

fn run() -> Result<()> {
    println!("Hello World!");

    Ok(())
}
