use crate::intcode;
use itertools::iproduct;
use std::io::{stdin, Read};
use structopt::StructOpt;

#[derive(StructOpt)]
pub enum Options {
    A { noun: i64, verb: i64 },
    B,
}

pub fn run(options: &Options) -> Result<(), failure::Error> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let opcodes = intcode::parse(&input)?;

    let output = match options {
        Options::A { noun, verb } => intcode::computer(&opcodes, vec![(1, *noun), (2, *verb)])?,
        Options::B => iproduct!((0..99), (0..99))
            .filter_map(
                |(a, b)| match intcode::computer(&opcodes, vec![(1, a), (2, b)]) {
                    Ok(output) if output == 19690720 => Some(Ok(a * 100 + b)),
                    Ok(_) => None,
                    Err(err) => Some(Err(err)),
                },
            )
            .next()
            .ok_or_else(|| failure::err_msg("no match!"))??,
    };

    eprintln!("{}", output);

    Ok(())
}
