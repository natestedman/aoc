use crate::intcode::{Computer, Step};
use itertools::iproduct;
use std::collections::HashSet;
use std::io::{stdin, Read};
use std::iter::{once, repeat, repeat_with};
use structopt::StructOpt;

#[derive(StructOpt)]
pub enum Options {
    A,
    B,
}

fn unique(settings: &Vec<i64>) -> bool {
    settings.iter().collect::<HashSet<&i64>>().len() == settings.len()
}

fn compute(opcodes: &Vec<i64>, settings: &Vec<i64>) -> Result<i64, failure::Error> {
    let mut storage: Vec<_> = repeat(0).take(settings.len()).collect();
    let mut terminated: Vec<_> = repeat(false).take(settings.len()).collect();
    let mut initial: Vec<_> = settings.iter().copied().map(once).collect();

    let mut computers: Vec<_> = repeat_with(|| Computer::new(&opcodes))
        .take(settings.len())
        .collect();

    loop {
        for i in 0..settings.len() {
            let mut input = initial.get_mut(i).unwrap().chain(once(storage[i])).map(Ok);

            loop {
                match computers[i].step(&mut input)? {
                    Step::Nothing => (),
                    Step::Output(o) => {
                        storage[(i + 1) % 5] = o;
                        break;
                    }
                    Step::Terminated => {
                        terminated[i] = true;
                        break;
                    }
                }
            }
        }

        if terminated.iter().all(|b| *b) {
            break;
        }
    }

    Ok(storage[0])
}

pub fn run(options: &Options) -> Result<(), failure::Error> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let opcodes = Computer::parse(&input)?;

    let values = match options {
        Options::A => iproduct!(0..5, 0..5, 0..5, 0..5, 0..5),
        Options::B => iproduct!(5..10, 5..10, 5..10, 5..10, 5..10),
    };

    let mut max = None;

    for next in values
        .map(|settings| vec![settings.0, settings.1, settings.2, settings.3, settings.4])
        .filter(unique)
        .map(|settings| compute(&opcodes, &settings))
    {
        let next = next?;
        max = Some(
            max.map(|current: i64| std::cmp::max(current, next))
                .unwrap_or(next),
        );
    }

    println!("{:?}", max.ok_or_else(|| failure::err_msg("no output!"))?);

    Ok(())
}
