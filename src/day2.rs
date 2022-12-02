enum Play {
    Rock = 1,
    Paper = 2,
    Scissors = 3
}

enum Outcome {
    Lose = 0,
    Draw = 1,
    Win = 2
}

fn play_string(play: char) -> Play {
    match play {
        'A'|'X' => Play::Rock,
        'B'|'Y' => Play::Paper,
        'C'|'Z' => Play::Scissors,
        _ => panic!("Invalid Play!")
    }
}

fn outcome_string(play: char) -> Outcome {
    match play {
        'X' => Outcome::Lose,
        'Y' => Outcome::Draw,
        'Z' => Outcome::Win,
        _ => panic!("Invalid Play!")
    }
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> usize {
    input.lines().map(|line| {
        let mut items = line.chars();
        let opponent = play_string(items.next().unwrap()) as usize;
        items.next();
        let player = play_string(items.next().unwrap()) as usize;
        let outcome = (player + 4 - opponent) % 3;
        outcome*3 + player
    }).sum::<usize>()
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> usize {
    input.lines().map(|line| {
        let mut items = line.chars();
        let opponent = play_string(items.next().unwrap()) as usize;
        items.next();
        let outcome = outcome_string(items.next().unwrap()) as usize;
        let player = (opponent + outcome + 1) % 3 +1;
        outcome*3 + player
    }).sum::<usize>()
}


#[cfg(test)]
mod tests {
    const TESTINPUT: &str = "\
A Y
B X
C Z";
    use super::*;
    #[test]
    fn part1_example() {
        assert_eq!(part1("A Y"), 8);
        assert_eq!(part1("B X"), 1);
        assert_eq!(part1("C Z"), 6);
        assert_eq!(part1(TESTINPUT), 15);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2("A Y"), 4);
        assert_eq!(part2("B X"), 1);
        assert_eq!(part2("C Z"), 7);
        assert_eq!(part2(TESTINPUT), 12)
    }
}