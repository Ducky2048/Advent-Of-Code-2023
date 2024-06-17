use std::cmp::max;
use std::ops::Add;
use std::str::FromStr;
use std::{fs, io};

use tap::Pipe;

static RED: &str = "red";
static GREEN: &str = "green";
static BLUE: &str = "blue";

fn main() -> Result<(), io::Error> {
    let input = fs::read_to_string("day_2_input.txt")?;

    let sum_of_min_set_powers: u32 = input
        .lines()
        .map(parse_line)
        .map(Game::min_set)
        .map(Handful::power)
        .sum();

    println!("Sum: {sum_of_min_set_powers}");

    Ok(())
}

struct Game {
    handfuls: Vec<Handful>,
}

impl Game {
    fn min_set(self) -> Handful {
        self.handfuls
            .into_iter()
            .reduce(|acc, it| Handful {
                red: max(acc.red, it.red),
                green: max(acc.green, it.green),
                blue: max(acc.blue, it.blue),
            })
            .unwrap()
    }
}

#[derive(Default, Debug, Clone)]
struct Handful {
    red: u8,
    green: u8,
    blue: u8,
}

impl Handful {
    fn power(self) -> u32 {
        (self.red as u32) * (self.green as u32) * (self.blue as u32)
    }
}

impl Add for Handful {
    type Output = Handful;

    fn add(self, rhs: Self) -> Self::Output {
        Handful {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

fn parse_line(line: &str) -> Game {
    let handfuls = line
        .split(':')
        .nth(1)
        .unwrap()
        .split(';')
        .map(|handful| {
            handful
                .split(", ")
                .map(|colour| {
                    if colour.ends_with(RED) {
                        let red = colour
                            .split_ascii_whitespace()
                            .next()
                            .unwrap()
                            .pipe(u8::from_str)
                            .unwrap();
                        Handful {
                            red,
                            ..Default::default()
                        }
                    } else if colour.ends_with(GREEN) {
                        let green = colour
                            .split_ascii_whitespace()
                            .next()
                            .unwrap()
                            .pipe(u8::from_str)
                            .unwrap();
                        Handful {
                            green,
                            ..Default::default()
                        }
                    } else if colour.ends_with(BLUE) {
                        let blue = colour
                            .split_ascii_whitespace()
                            .next()
                            .unwrap()
                            .pipe(u8::from_str)
                            .unwrap();
                        Handful {
                            blue,
                            ..Default::default()
                        }
                    } else {
                        panic!("{colour}")
                    }
                })
                .fold(Handful::default(), |acc, it| acc + it)
        })
        .collect::<Vec<_>>();

    Game { handfuls }
}
