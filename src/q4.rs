use structopt::StructOpt;

#[derive(StructOpt)]
pub enum Options {
    A,
    B,
}

fn pairs(digits: &Vec<u32>) -> impl Iterator<Item = (&u32, &u32)> {
    digits[0..digits.len() - 1].iter().zip(digits[1..].iter())
}

fn ascending(digits: &Vec<u32>) -> bool {
    pairs(digits).all(|(a, b)| a <= b)
}

fn has_two_adjacent(digits: &Vec<u32>) -> bool {
    pairs(digits).any(|(a, b)| a == b)
}

fn has_exactly_two_adjacent(digits: &Vec<u32>) -> bool {
    let mut value = 0;
    let mut length = 0;

    for digit in digits.iter() {
        if value == *digit {
            length += 1;
        } else if length == 2 {
            return true;
        } else {
            length = 1;
            value = *digit;
        }
    }

    length == 2
}

pub fn run(options: &Options) -> Result<(), failure::Error> {
    let matches = (231832..767346)
        .filter(|password| {
            let digits = password
                .to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>();

            ascending(&digits)
                && match options {
                    Options::A => has_two_adjacent(&digits),
                    Options::B => has_exactly_two_adjacent(&digits),
                }
        })
        .count();

    println!("{}", matches);

    Ok(())
}
