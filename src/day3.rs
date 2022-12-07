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
    let mut count: [usize;52] = [0;52];
    input.lines().map(|line| {
        let h1 = &line[0..line.len()/2];
        let h2 = &line[line.len()/2..];
        for x in h1.bytes(){
            let idx = to_priority(x) as usize;
            count[idx-1] = 1;
        }
        for x in h2.bytes(){
            let idx = to_priority(x) as usize;
            if count[idx-1] == 1{
                count = [0;52];
                return idx
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
}