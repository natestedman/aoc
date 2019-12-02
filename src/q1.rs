use std::io::{self, BufRead};

fn fuel(weight: i64) -> i64 {
    let required = (weight / 3) - 2;
    if required > 0 {
        required + fuel(required)
    } else {
        0
    }
}

pub fn run() -> Result<(), failure::Error> {
    let sum = io::stdin()
        .lock()
        .lines()
        .fold(Ok(0), |total, next| -> Result<i64, failure::Error> {
            Ok(total? + fuel(next?.parse::<i64>()?))
        })?;

    println!("{}", sum);

    Ok(())
}
