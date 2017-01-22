#[macro_use]
extern crate error_chain;

mod errors { error_chain!{} }
use errors::*;
quick_main!(run);

fn run() -> Result<()> {
    println!("Hello World!");

    Ok(())
}
