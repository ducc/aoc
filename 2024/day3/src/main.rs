use regex::Regex;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("part one: {}", part_one());
    println!("part two: {}", part_two());
}

fn part_one() -> u32 {
    Regex::new(r"mul\(((\d+),(\d+))\)")
        .unwrap()
        .captures_iter(INPUT)
        .map(|cap| {
            let (_, [_, a, b]) = cap.extract();
            a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap()
        })
        .sum::<u32>()
}

enum Instruction {
    Dont,
    Do,
    Mul(u32, u32),
}

fn part_two() -> u32 {
    let re = Regex::new(r"((mul)|(do(n't)?))\(((\d+),(\d+))?\)").unwrap();

    let instructions = re
        .captures_iter(INPUT)
        .map(|cap| match cap.get(1).unwrap().as_str() {
            "don't" => Instruction::Dont,
            "do" => Instruction::Do,
            "mul" => {
                let a = cap.get(6).unwrap().as_str().parse().unwrap();
                let b = cap.get(7).unwrap().as_str().parse().unwrap();
                Instruction::Mul(a, b)
            }
            _ => panic!(),
        });

    let mut skip_mul = false;
    let mut sum = 0;

    for instruction in instructions {
        match instruction {
            Instruction::Dont => {
                skip_mul = true;
            }
            Instruction::Do => {
                skip_mul = false;
            }
            Instruction::Mul(a, b) => {
                if !skip_mul {
                    sum += a * b;
                }
            }
        }
    }

    sum
}
