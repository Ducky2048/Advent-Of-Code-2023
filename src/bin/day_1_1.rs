use std::{fs, io};

fn main() -> Result<(), io::Error> {
    let input = fs::read_to_string("day_1_input.txt")?;

    let sum: u32 = input
        .lines()
        .map(|line| {
            let left = line
                .chars()
                .find(|char| char.is_ascii_digit())
                .unwrap()
                .to_digit(10)
                .unwrap();
            let right = line
                .chars()
                .rev()
                .find(|char| char.is_ascii_digit())
                .unwrap()
                .to_digit(10)
                .unwrap();
            (left * 10) + right
        })
        .sum();

    println!("Sum: {sum}");

    Ok(())
}
