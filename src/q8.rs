use crate::image::Image;
use std::io::{stdin, Read};
use structopt::StructOpt;

#[derive(StructOpt)]
pub enum Options {
    A { width: usize, height: usize },
    B { width: usize, height: usize },
}

impl Options {
    fn dimensions(&self) -> (usize, usize) {
        match self {
            Options::A { width, height } => (*width, *height),
            Options::B { width, height } => (*width, *height),
        }
    }
}

pub fn run(options: &Options) -> Result<(), failure::Error> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let (width, height) = options.dimensions();
    let image = Image::new_with_string(&input, width, height)?;

    match options {
        Options::A { .. } => {
            let min = image
                .layers()
                .min_by_key(|layer| layer.pixels().filter(|c| *c == 0).count())
                .unwrap();

            let (ones, zeroes) = min
                .pixels()
                .map(|pixel| ((pixel == 1) as i64, (pixel == 2) as i64))
                .fold((0, 0), |total, next| (total.0 + next.0, total.1 + next.1));

            println!("{}", ones * zeroes);
        }
        Options::B { .. } => {
            println!("{}", image.flatten().layer(0));
        }
    }

    Ok(())
}
