mod q1;

use structopt::StructOpt;

#[derive(StructOpt)]
enum Options {
    Q1,
}

fn main() -> Result<(), failure::Error> {
    match Options::from_args() {
        Options::Q1 => q1::run(),
    }
}
