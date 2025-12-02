use part1::*;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;

mod part1;

fn main() {
    part1()
}

fn part1() {
    let cwd = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(file!());
    let input = File::open(cwd.parent().unwrap().join("day1_input.txt")).unwrap();

    let mut answer = 0;
    let mut dial = part1::Dial::new(StartingPosition(50), MaxPosition(99));

    BufReader::new(input).lines().for_each(|line| {
        let rotation: Rotation = line.unwrap().try_into().unwrap();
        dial.rotate(rotation);
        if dial.position() == 0 {
            answer += 1;
        }
    });

    println!("day 1: {answer}");
}
