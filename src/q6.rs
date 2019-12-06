use std::collections::{HashMap, HashSet};
use std::io::{stdin, Read};
use std::iter::{repeat, successors};
use structopt::StructOpt;

// part 1

fn count(map: &HashMap<&str, HashSet<&str>>, key: &str, depth: u64) -> u64 {
    depth
        + map
            .get(key)
            .iter()
            .flat_map(|orbits| orbits.iter())
            .map(|next| count(map, next, depth + 1))
            .sum::<u64>()
}

// part 2

fn transfer(upwards: &HashMap<&str, &str>, start: &str, end: &str) -> u64 {
    let mut down: Vec<_> = parents(start, upwards).collect();
    down.reverse();

    let mut up: Vec<_> = parents(end, upwards).collect();
    up.reverse();

    let pairs = then_none(down.iter()).zip(then_none(up.iter()));
    pairs
        .skip_while(|(a, b)| a == b)
        .map(|(a, b)| some_val(a) + some_val(b))
        .take_while(|&val| val > 0)
        .sum()
}

fn some_val<T>(option: Option<T>) -> u64 {
    option.map(|_| 1).unwrap_or(0)
}

fn then_none<I: Iterator>(iter: I) -> impl Iterator<Item = Option<I::Item>>
where
    I::Item: Clone,
{
    iter.map(Some).chain(repeat(None))
}

fn parents<'a>(
    start: &'a str,
    upwards: &'a HashMap<&'a str, &'a str>,
) -> impl Iterator<Item = &'a str> {
    successors(upwards.get(start), move |parent| upwards.get(*parent)).copied()
}

// main

#[derive(StructOpt)]
pub enum Options {
    A,
    B,
}

pub fn run(options: &Options) -> Result<(), failure::Error> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let orbits = input.trim().split("\n").map(|line| {
        let mut split = line.split(")");
        (split.next().unwrap(), split.next().unwrap())
    });

    match options {
        Options::A => {
            let mut map: HashMap<&str, HashSet<&str>> = HashMap::new();
            for (outer, inner) in orbits {
                map.entry(outer).or_insert_with(HashSet::new).insert(inner);
            }
            println!("{}", count(&map, "COM", 0));
        }
        Options::B => {
            let map = orbits.map(|(a, b)| (b, a)).collect::<HashMap<_, _>>();
            println!("{}", transfer(&map, "SAN", "YOU"));
        }
    }

    Ok(())
}
