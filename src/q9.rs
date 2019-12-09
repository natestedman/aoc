use crate::intcode::Computer;
use std::io::{stdin, Read};
use std::iter::once;
use structopt::StructOpt;

#[derive(StructOpt)]
pub enum Options {
    A,
    B,
}

pub fn run(options: &Options) -> Result<(), failure::Error> {
    let mut program = String::new();
    stdin().read_to_string(&mut program)?;

    let input = match options {
        Options::A => 1,
        Options::B => 2,
    };

    let mut computer = Computer::new(&Computer::parse(&program)?);
    computer.run(&mut once(Ok(input)))?;

    Ok(())
}
