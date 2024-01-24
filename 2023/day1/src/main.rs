fn part_one() {
    let answer: u32 = include_str!("input.txt")
        .split("\n")
        .map(|line| line.chars().filter(|char| char.is_numeric()))
        .map(|mut nums| {
            let first = nums.next().unwrap();
            let last = nums.last().unwrap_or(first);
            format!("{first}{last}").parse::<u32>().unwrap()
        })
        .sum();

    println!("{answer}");
}

fn part_two() {
    let lines = include_str!("input.txt").split("\n");

    let words: Vec<(&str, usize)> = vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let mut answer = 0;

    for line in lines {
        let mut word_indexes: Vec<(usize, usize)> = vec![];

        for (word, word_value) in &words {
            // forward
            for (index, _) in line.match_indices(word) {
                word_indexes.push((index, *word_value));
            }

            // backwards
            for (index, _) in line.rmatch_indices(word) {
                word_indexes.push((index, *word_value));
            }
        }

        for (index, char) in line.char_indices() {
            if char.is_numeric() {
                word_indexes.push((index, char.to_digit(10).unwrap() as usize));
            }
        }

        word_indexes.sort_by_key(|(index, _)| *index);

        let first = word_indexes[0].1;
        let last = word_indexes.last().map(|v| v.1).unwrap_or(first);

        let value = format!("{first}{last}").parse::<i32>().unwrap();
        answer += value;
    }

    println!("{answer}");
}

fn main() {
    part_two();
}
