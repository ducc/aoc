use std::str::FromStr;

#[derive(Debug)]
struct Game<'a> {
    id: i32,
    reveals: Vec<Vec<(i32, &'a str)>>,
}

fn part_one() {
    const MAX_RED: i32 = 12;
    const MAX_GREEN: i32 = 13;
    const MAX_BLUE: i32 = 14;

    let games = parse_input();

    let mut possible_game_ids = vec![];

    for game in games {
        let mut impossible = false;

        for reveal in game.reveals {
            for (number, color) in reveal {
                match color {
                    "red" => {
                        if number > MAX_RED {
                            impossible = true;
                            break;
                        }
                    }
                    "green" => {
                        if number > MAX_GREEN {
                            impossible = true;
                            break;
                        }
                    }
                    "blue" => {
                        if number > MAX_BLUE {
                            impossible = true;
                            break;
                        }
                    }
                    _ => unreachable!()
                }
            }
        }

        if !impossible {
            possible_game_ids.push(game.id);
        }
    }

    println!("{}", possible_game_ids.into_iter().sum::<i32>())
}

fn part_two() {
    let games = parse_input();

    let mut output = 0;

    for game in games {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for reveal in game.reveals {
            for (number, color) in reveal {
                match color {
                    "red" => {
                        if number > max_red {
                            max_red = number;
                        }
                    }
                    "green" => {
                        if number > max_green {
                            max_green = number;
                        }
                    }
                    "blue" => {
                        if number > max_blue {
                            max_blue = number;
                        }
                    }
                    _ => unreachable!()
                }
            }
        }

        let pow = max_red * max_green * max_blue;
        output += pow;
    }

    println!("{output}");
}

fn parse_input<'a>() -> Vec<Game<'a>> {
    // yes
    include_str!("input.txt")
        .split("\n")
        .map(|line| {
            let mut splits = line.split(":");
            let game = splits
                .next()
                .unwrap()
                .split(" ")
                .nth(1)
                .unwrap()
                .parse::<i32>()
                .unwrap();
            let reveal_stages = splits
                .next()
                .unwrap()
                .split(";")
                .map(|s| s.trim())
                .map(|subset| {
                    subset
                        .split(",")
                        .map(|s| {
                            let splits = s.trim().split(" ").collect::<Vec<&str>>();
                            let num = splits[0].parse::<i32>().unwrap();
                            let color = splits[1];

                            (num, color)
                        })
                        .collect::<Vec<(i32, &str)>>()
                })
                .collect::<Vec<Vec<(i32, &str)>>>();
            Game {
                id: game,
                reveals: reveal_stages,
            }
        })
        .collect::<Vec<Game>>()
}

fn main() {
    part_two()
}
