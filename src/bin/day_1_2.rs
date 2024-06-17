use std::{fs, io};

static ONE: &str = "one";
static TWO: &str = "two";
static THREE: &str = "three";
static FOUR: &str = "four";
static FIVE: &str = "five";
static SIX: &str = "six";
static SEVEN: &str = "seven";
static EIGHT: &str = "eight";
static NINE: &str = "nine";

static NUMS: [&str; 9] = [ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE];

fn main() -> Result<(), io::Error> {
    let input = fs::read_to_string("day_1_input.txt")?;

    let sum: usize = input
        .lines()
        .map(|line| {
            let left_char_idx = line.chars().position(|char| char.is_ascii_digit());
            let right_char_idx = line.as_bytes().iter().rposition(|char| char.is_ascii_digit());

            let left_word_idx_and_value = NUMS
                .iter()
                .enumerate()
                .filter_map(|(i, it)| line.find(it).map(|offset| (offset, i + 1)))
                .min_by(|(left, _), (right, _)| left.cmp(right));

            let right_word_idx_and_value = NUMS
                .iter()
                .enumerate()
                .filter_map(|(i, it)| line.rfind(it).map(|offset| (offset, i + 1)))
                .max_by(|(left, _), (right, _)| left.cmp(right));

            let left = match (left_char_idx, left_word_idx_and_value) {
                (Some(idx), None) => (line.as_bytes()[idx] as char).to_digit(10).unwrap() as usize,
                (None, Some((_, value))) => value,
                (Some(char_idx), Some((word_idx, value))) => {
                    if char_idx < word_idx {
                        (line.as_bytes()[char_idx] as char).to_digit(10).unwrap() as usize
                    } else {
                        value
                    }
                }
                _ => panic!(),
            };

            let right = match (right_char_idx, right_word_idx_and_value) {
                (Some(idx), None) => (line.as_bytes()[idx] as char).to_digit(10).unwrap() as usize,
                (None, Some((_, value))) => value,
                (Some(char_idx), Some((word_idx, value))) => {
                    if char_idx > word_idx {
                        (line.as_bytes()[char_idx] as char).to_digit(10).unwrap() as usize
                    } else {
                        value
                    }
                }
                _ => panic!(),
            };

            (left * 10) + right
        })
        .sum();

    println!("Sum: {sum}");

    Ok(())
}
