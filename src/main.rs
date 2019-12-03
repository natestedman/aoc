mod q1;
mod q2;
mod q3;

use structopt::StructOpt;

#[derive(StructOpt)]
enum Options {
    Q1,
    Q2(q2::Options),
    Q3(q3::Options),
}

fn main() -> Result<(), failure::Error> {
    match Options::from_args() {
        Options::Q1 => q1::run(),
        Options::Q2(options) => q2::run(&options),
        Options::Q3(options) => q3::run(&options),
    }
}
