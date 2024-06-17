use std::ops::Add;
use std::str::FromStr;
use std::{fs, io};

use tap::Pipe;

static RED: &str = "red";
static GREEN: &str = "green";
static BLUE: &str = "blue";

const MAX_RED: u8 = 12;
const MAX_GREEN: u8 = 13;
const MAX_BLUE: u8 = 14;

fn main() -> Result<(), io::Error> {
    let input = fs::read_to_string("day_2_input.txt")?;

    let sum_of_ids = input
        .lines()
        .map(parse_line)
        .filter(Game::is_within_limits)
        .fold(0usize, |acc, it| acc + it.id as usize);

    println!("Sum: {sum_of_ids}");

    Ok(())
}

struct Game {
    id: u8,
    handfuls: Vec<Handful>,
}

impl Game {
    fn is_within_limits(&self) -> bool {
        self.handfuls.iter().all(Handful::is_within_limits)
    }
}

impl Handful {
    fn is_within_limits(&self) -> bool {
        self.red <= MAX_RED && self.green <= MAX_GREEN && self.blue <= MAX_BLUE
    }
}

#[derive(Default, Debug)]
struct Handful {
    red: u8,
    green: u8,
    blue: u8,
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
    let mut line_iter = line.split(':');

    let raw_game = line_iter.next().unwrap();

    let game_id: u8 = raw_game
        .split_ascii_whitespace()
        .nth(1)
        .unwrap()
        .pipe(u8::from_str)
        .unwrap();

    let handfuls = line_iter
        .next()
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

    Game {
        id: game_id,
        handfuls,
    }
}
