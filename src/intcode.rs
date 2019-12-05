use std::io::stdin;
use std::ops::{Add, Mul};

pub fn parse(input: &str) -> Result<Vec<i64>, std::num::ParseIntError> {
    input
        .trim_end()
        .split(",")
        .map(|opcode| opcode.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()
}

pub fn computer(opcodes: &Vec<i64>, overrides: Vec<(usize, i64)>) -> Result<i64, failure::Error> {
    let mut opcodes = opcodes.clone();

    for (i, val) in overrides.iter() {
        opcodes[*i] = *val;
    }

    step(&mut opcodes, 0)
}

fn step(opcodes: &mut Vec<i64>, i: usize) -> Result<i64, failure::Error> {
    let opcode = opcodes[i];

    match opcode % 100 {
        1 => operator_step(opcodes, i, i64::add),
        2 => operator_step(opcodes, i, i64::mul),
        3 => input_step(opcodes, i),
        4 => output_step(opcodes, i),
        5 => jump_step(opcodes, i, i64::ne),
        6 => jump_step(opcodes, i, i64::eq),
        7 => store_step(opcodes, i, i64::lt),
        8 => store_step(opcodes, i, i64::eq),
        99 => Ok(opcodes[0]),
        _ => Err(failure::err_msg(format!("invalid opcode {}", opcodes[i]))),
    }
}

fn input_step(opcodes: &mut Vec<i64>, i: usize) -> Result<i64, failure::Error> {
    let dest = opcodes[i + 1];
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    opcodes[dest as usize] = input.trim().parse::<i64>()?;
    step(opcodes, i + 2)
}

fn output_step(opcodes: &mut Vec<i64>, i: usize) -> Result<i64, failure::Error> {
    println!("{}", arg(opcodes, i, 1));
    step(opcodes, i + 2)
}

fn operator_step<F>(opcodes: &mut Vec<i64>, i: usize, operator: F) -> Result<i64, failure::Error>
where
    F: Fn(i64, i64) -> i64,
{
    let dest = opcodes[i + 3] as usize;
    opcodes[dest] = operator(arg(opcodes, i, 1), arg(opcodes, i, 2));
    step(opcodes, i + 4)
}

fn jump_step<F>(opcodes: &mut Vec<i64>, i: usize, operator: F) -> Result<i64, failure::Error>
where
    F: Fn(&i64, &i64) -> bool,
{
    let dest = if operator(&arg(opcodes, i, 1), &0) {
        arg(opcodes, i, 2) as usize
    } else {
        i + 3
    };
    step(opcodes, dest)
}

fn store_step<F>(opcodes: &mut Vec<i64>, i: usize, operator: F) -> Result<i64, failure::Error>
where
    F: Fn(&i64, &i64) -> bool,
{
    let dest = opcodes[i + 3] as usize;
    opcodes[dest] = if operator(&arg(opcodes, i, 1), &arg(opcodes, i, 2)) {
        1
    } else {
        0
    };

    step(opcodes, i + 4)
}

fn arg(opcodes: &Vec<i64>, i: usize, argi: usize) -> i64 {
    let val = opcodes[i + argi];
    if opcodes[i] / (i64::pow(10, argi as u32 + 1)) % 10 == 1 {
        val
    } else {
        opcodes[val as usize]
    }
}
