use std::io::{stdin, Read};
use std::ops::{Add, Mul};
use structopt::StructOpt;

#[derive(StructOpt)]
pub enum Options {
    A { noun: usize, verb: usize },
    B,
}

fn computer(opcodes: &Vec<usize>, noun: usize, verb: usize) -> Result<usize, failure::Error> {
    let mut opcodes = opcodes.clone();
    opcodes[1] = noun;
    opcodes[2] = verb;
    computer_step(&mut opcodes, 0)
}

fn computer_step(opcodes: &mut Vec<usize>, i: usize) -> Result<usize, failure::Error> {
    match opcodes[i] {
        1 => operator_step(opcodes, i, usize::add),
        2 => operator_step(opcodes, i, usize::mul),
        99 => Ok(opcodes[0]),
        _ => Err(failure::err_msg(format!("invalid opcode {}", opcodes[i]))),
    }
}

fn operator_step<F>(
    opcodes: &mut Vec<usize>,
    i: usize,
    operator: F,
) -> Result<usize, failure::Error>
where
    F: Fn(usize, usize) -> usize,
{
    let (lhs, rhs, dest) = (opcodes[i + 1], opcodes[i + 2], opcodes[i + 3]);
    opcodes[dest] = operator(opcodes[lhs], opcodes[rhs]);
    computer_step(opcodes, i + 4)
}

pub fn run(options: &Options) -> Result<(), failure::Error> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let opcodes = input
        .trim_end()
        .split(",")
        .map(|opcode| opcode.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()?;

    let output = match options {
        Options::A { noun, verb } => computer(&opcodes, *noun, *verb)?,
        Options::B => (0..100)
            .flat_map(|a| (0..100).map(move |b| (a, b)))
            .filter_map(|(a, b)| match computer(&opcodes, a, b) {
                Ok(output) if output == 19690720 => Some(Ok(a * 100 + b)),
                Ok(_) => None,
                Err(err) => Some(Err(err)),
            })
            .next()
            .ok_or_else(|| failure::err_msg("no match!"))??,
    };

    eprintln!("{}", output);

    Ok(())
}
