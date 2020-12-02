use clap::Clap;
use std::fs::File;
use std::io::{self, BufRead};
use std::num::ParseIntError;
use std::path::Path;
use std::str::FromStr;

#[macro_use]
extern crate lazy_static;
use regex::Regex;

struct Range {
    letter: char,
    min: i32,
    max: i32,
}

struct Line {
    range: Range,
    password: String,
}

impl Line {
    fn is_valid_pt1(&self) -> bool {
        let count = self
            .password
            .chars()
            .filter(|x| *x == self.range.letter)
            .count() as i32;
        (count >= self.range.min) && (count <= self.range.max)
    }

    fn is_valid_pt2(&self) -> bool {
        let pw_chars: Vec<char> = self.password.chars().collect();
        if pw_chars[(self.range.min - 1) as usize] == self.range.letter {
            pw_chars[(self.range.max - 1) as usize] != self.range.letter
        } else {
            pw_chars[(self.range.max - 1) as usize] == self.range.letter
        }
    }
}

impl FromStr for Line {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+)-(\d+) (\w{1}): (\w+)").unwrap();
        }
        let cap = RE.captures_iter(s).next().unwrap();
        Ok(Line {
            password: cap[4].to_string(),
            range: Range {
                min: cap[1].parse().unwrap(),
                max: cap[2].parse().unwrap(),
                letter: cap[3].parse().unwrap(),
            },
        })
    }
}

#[derive(Clap)]
struct Opts {
    part: i32,
    input: String,
}

fn main() {
    let opts: Opts = Opts::parse();
    let lines = get_lines(opts.input);
    let count = if opts.part == 1 {
        lines.iter().filter(|x| x.is_valid_pt1()).count()
    } else {
        lines.iter().filter(|x| x.is_valid_pt2()).count()
    };
    println!("{0} valid from {1}", count, lines.len());
}

fn get_lines(filename: String) -> Vec<Line> {
    let mut pw_lines = Vec::<Line>::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line_as_string) = line {
                pw_lines.push(Line::from_str(&line_as_string).unwrap())
            }
        }
    }

    pw_lines
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
