mod image;
mod intcode;
mod q1;
mod q2;
mod q3;
mod q4;
mod q5;
mod q6;
mod q7;
mod q8;

use structopt::StructOpt;

#[derive(StructOpt)]
enum Options {
    Q1,
    Q2(q2::Options),
    Q3(q3::Options),
    Q4(q4::Options),
    Q5(q5::Options),
    Q6(q6::Options),
    Q7(q7::Options),
    Q8(q8::Options),
}

fn main() -> Result<(), failure::Error> {
    match Options::from_args() {
        Options::Q1 => q1::run(),
        Options::Q2(options) => q2::run(&options),
        Options::Q3(options) => q3::run(&options),
        Options::Q4(options) => q4::run(&options),
        Options::Q5(options) => q5::run(&options),
        Options::Q6(options) => q6::run(&options),
        Options::Q7(options) => q7::run(&options),
        Options::Q8(options) => q8::run(&options),
    }
}
