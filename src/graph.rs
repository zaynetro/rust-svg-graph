use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::i32::{MAX, MIN};

use maud::PreEscaped;

use entry::Entry;

pub type HTML = PreEscaped<String>;

pub trait Graph {
    fn into_string(&self) -> String {
        self.into_html().into_string()
    }

    fn into_file(&self, filename: &str) -> Result<(), io::Error> {
        let mut f = try!(File::create(filename));
        let bytes = self.into_string().into_bytes();
        try!(f.write_all(&bytes));
        Ok(())
    }

    fn into_html(&self) -> HTML;
}

pub struct Coord {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32
}

impl Coord {
    pub fn from_padding(&Padding { top, right, bottom, left }: &Padding,
                        (x, y, w, h): (f32, f32, f32, f32)) -> Coord {

        Coord {
            x: x + left,
            y: y + top,
            width: w - left - right,
            height: h - top - bottom,
        }
    }

    pub fn center(&self) -> (f32, f32) {
        (self.width / 2.0 + self.x, self.height / 2.0 + self.y)
    }
}

pub struct Padding {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32
}

impl Padding {
    fn new(top: f32, right: f32, bottom: f32, left: f32) -> Padding {
        Padding {
            top: top,
            right: right,
            bottom: bottom,
            left: left
        }
    }

    pub fn with_same(v: f32) -> Padding {
        Padding::new(v, v, v, v)
    }
}

pub struct Size {
    pub width: f32,
    pub height: f32
}

pub struct Tools;

impl Tools {
    pub fn tr(x: f32, y: f32) -> String {
        format!("translate({},{})", x, y)
    }

    pub fn min_max_entry_values(entries: &Vec<Entry>) -> (i32, i32) {
        entries
            .iter()
            .fold((MAX, MIN), |(min, max), ref e| {
                let min = if e.value < min { e.value } else { min };
                let max = if e.value > max { e.value } else { max };
                (min, max)
            })
    }
}
