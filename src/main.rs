#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate nom;

mod parser;

mod errors { error_chain!{} }
use errors::*;
quick_main!(run);

fn run() -> Result<()> {
    println!("Hello World!");

    Ok(())
}
