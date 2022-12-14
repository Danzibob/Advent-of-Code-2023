use core::panic;

fn to_priority(c: u8) -> u8{
    match c {
        b'a'..=b'z' => c - 96,
        b'A'..=b'Z' => c - 38,
        _ => panic!("Invalid input for priority")
    }
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> usize {
    let mut count: [usize;123] = [0;123];
    input.lines().map(|line| {
        let h1 = &line[0..line.len()/2];
        let h2 = &line[line.len()/2..];
        for x in h1.bytes(){
            count[x as usize] = 1;
        }
        for x in h2.bytes(){
            if count[x as usize] == 1{
                count = [0;123];
                return to_priority(x) as usize;
            };
        }
        panic!("No common items found!");
    }).sum::<usize>()
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> usize {
    let mut count: [usize;123] = [0;123];
    input.lines().collect::<Vec<_>>().chunks(3).map(|group|{
        for x in group[0].bytes(){
            count[x as usize] = 1;
        }
        for x in group[1].bytes(){
            if count[x as usize] == 1 {
                count[x as usize] = 2;
            }
        }
        for x in group[2].bytes(){
            if count[x as usize] == 2{
                count = [0;123];
                return x as usize;
            };
        }
        panic!("No common items found!");
    }).sum::<usize>()
}

#[cfg(test)]
mod tests {
    const TESTINPUT: &str = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
    use super::*;
    #[test]
    fn priority_conversion() {
        assert_eq!(to_priority(b'a'), 1);
        assert_eq!(to_priority(b'z'), 26);
        assert_eq!(to_priority(b'A'), 27);
        assert_eq!(to_priority(b'Z'), 52);
    }
    #[test]
    fn part1_example() {
        assert_eq!(part1(TESTINPUT), 157);
    }
    #[test]
    fn part2_example() {
        assert_eq!(part2(TESTINPUT), 70);
    }
}