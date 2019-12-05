use crate::intcode;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Options {
    program: String,
}

pub fn run(options: &Options) -> Result<(), failure::Error> {
    intcode::computer(&intcode::parse(&options.program)?, vec![])?;
    Ok(())
}
