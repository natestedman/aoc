use itertools::iproduct;
use std::borrow::Cow;
use std::fmt::{Display, Formatter};

pub struct Image<'a> {
    pixels: Cow<'a, [u8]>,
    offset: u8,
    width: usize,
    height: usize,
    layers: usize,
}

impl<'a> Image<'a> {
    pub fn new_with_string(
        string: &'a str,
        width: usize,
        height: usize,
    ) -> Result<Image, failure::Error> {
        let pixels = string.as_bytes();
        let dimensions = width * height;
        if pixels.len() % dimensions == 0 {
            let layers = pixels.len() / dimensions;
            Ok(Image {
                pixels: Cow::Borrowed(pixels),
                offset: '0' as u8,
                width,
                height,
                layers,
            })
        } else {
            Err(failure::err_msg("bad dimensions"))
        }
    }

    pub fn layer(&'a self, index: usize) -> Layer<'a> {
        Layer { image: self, index }
    }

    pub fn layers(&'a self) -> impl DoubleEndedIterator<Item = Layer<'a>> {
        (0..self.layers).map(move |index| self.layer(index))
    }

    pub fn flatten(&'a self) -> Image {
        let mut pixels: Vec<_> = self.layer(self.layers - 1).pixels().collect();

        for layer in self.layers().rev().skip(1) {
            layer.overlay(&mut pixels);
        }

        Image {
            pixels: Cow::Owned(pixels),
            width: self.width,
            height: self.height,
            offset: 0,
            layers: 1,
        }
    }

    fn pixel(&self, layer: usize, x: usize, y: usize) -> u8 {
        self.pixels[layer * (self.width * self.height) + y * self.width + x] - self.offset
    }
}

pub struct Layer<'a> {
    image: &'a Image<'a>,
    index: usize,
}

impl<'a> Layer<'a> {
    pub fn pixel(&self, x: usize, y: usize) -> u8 {
        self.image.pixel(self.index, x, y)
    }

    pub fn pixels(&'a self) -> impl Iterator<Item = u8> + 'a {
        iproduct!((0..self.image.height), (0..self.image.width)).map(move |(y, x)| self.pixel(x, y))
    }

    fn overlay(&'a self, on: &mut Vec<u8>) {
        for (dest, pixel) in on
            .iter_mut()
            .zip(self.pixels())
            .filter(|(_, pixel)| *pixel != 2)
        {
            *dest = pixel;
        }
    }
}

impl<'a> Display for Layer<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.image.height {
            for x in 0..self.image.width {
                if self.pixel(x, y) == 0 {
                    write!(f, " ")?;
                } else {
                    write!(f, "{}", self.pixel(x, y))?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
