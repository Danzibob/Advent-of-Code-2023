#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    let mut buff = [' '; 3];
    for (i, x) in input.chars().enumerate() {
        if i > 3
            && (x != buff[0])
            && (x != buff[1])
            && (x != buff[2])
            && (buff[0] != buff[1])
            && (buff[1] != buff[2])
            && (buff[0] != buff[2])
        {
            return i + 1;
        } else {
            buff[(i % 3)] = x;
        }
    }
    panic!();
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    let mut buff = [' '; 14];
    let mut ptr = 0;
    for (i, x) in input.chars().enumerate() {
        if let Some(shift) = buff[..ptr].iter().position(|v| v == &x) {
            for j in 0..(ptr - shift - 1) {
                buff[j] = buff[j + shift + 1]
            }
            ptr = ptr - shift - 1;
        }
        buff[ptr] = x;
        ptr += 1;
        if ptr == 14 {
            return i + 1;
        }
    }
    panic!();
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
        assert_eq!(part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
