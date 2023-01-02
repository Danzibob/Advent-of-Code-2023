#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_example() {
        assert_eq!(part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }
    #[test]
    fn part2_example() {
        assert_eq!(part2(TESTINPUT), 70);
    }
}