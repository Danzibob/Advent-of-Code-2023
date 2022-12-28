#[aoc(day4, part1)]
pub fn part1(input: &str) -> usize {
    input.lines().map(|line| {
        let mut pairs = line.split(",").map(|pair| {
            pair.split("-").map(|x| x.parse::<usize>().unwrap())
        });
        let mut A = pairs.next().unwrap();
        let A0 = A.next().unwrap();
        let A1 = A.next().unwrap();
        let mut B = pairs.next().unwrap();
        let B0 = B.next().unwrap();
        let B1 = B.next().unwrap();
        (A0 <= B0 && A1 >= B1) || (B0 <= A0 && B1 >= A1)
    }).filter(|x| *x).count()
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> usize {
    input.lines().filter(|line| {
        let mut pairs = line.split(",").map(|pair| {
            let mut pair = pair.split("-").map(|x| x.parse::<usize>().unwrap());
            (pair.next().unwrap(), pair.next().unwrap())
        });
        let (A0, A1) = pairs.next().unwrap();
        let (B0, B1) = pairs.next().unwrap();
        (A0 <= B0 && A1 >= B0) || 
        (A0 <= B1 && A1 >= B1)
    }).count()
}

#[cfg(test)]
mod tests {
    const TESTINPUT: &str = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
    use super::*;
    #[test]
    fn part1_example() {
        assert_eq!(part1(TESTINPUT), 2);
    }
    #[test]
    fn part2_example() {
        assert_eq!(part2(TESTINPUT), 4);
    }
}