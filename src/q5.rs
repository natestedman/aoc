use crate::intcode::Computer;
use std::io::{stdin, BufRead};
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Options {
    program: String,
}

pub fn run(options: &Options) -> Result<(), failure::Error> {
    let stdin = stdin();
    let mut lines = stdin.lock().lines().map(|line| Ok(line?.parse::<i64>()?));
    Computer::new(&Computer::parse(&options.program)?).run(&mut lines)?;

    Ok(())
}
