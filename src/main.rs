#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate nom;

#[macro_use]
extern crate enum_primitive;

extern crate num;

mod parser;
mod opcode;

mod cpu;

mod errors { error_chain!{} }
use errors::*;
quick_main!(run);

fn run() -> Result<()> {
    println!("Hello World!");

    Ok(())
}
