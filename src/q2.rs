use crate::intcode::Computer;
use itertools::iproduct;
use std::io::{stdin, Read};
use std::iter::empty;
use structopt::StructOpt;

#[derive(StructOpt)]
pub enum Options {
    A { noun: i64, verb: i64 },
    B,
}

pub fn run(options: &Options) -> Result<(), failure::Error> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let opcodes = Computer::parse(&input)?;

    let output = match options {
        Options::A { noun, verb } => {
            Computer::new_overrides(&opcodes, vec![(1, *noun), (2, *verb)]).run(&mut empty())?
        }
        Options::B => iproduct!((0..99), (0..99))
            .filter_map(|(a, b)| {
                match Computer::new_overrides(&opcodes, vec![(1, a), (2, b)]).run(&mut empty()) {
                    Ok(output) if output == 19690720 => Some(Ok(a * 100 + b)),
                    Ok(_) => None,
                    Err(err) => Some(Err(err)),
                }
            })
            .next()
            .ok_or_else(|| failure::err_msg("no match!"))??,
    };

    println!("{}", output);

    Ok(())
}
