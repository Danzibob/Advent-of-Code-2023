type Instruction = (usize, usize, usize);

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> (Vec<Instruction>, Vec<Vec<u8>>) {
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut lines = input.lines().rev();
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {break};
        let words: Vec<&str> = line.split(" ").collect();
        instructions.push((words[1].parse().unwrap(),
                           words[3].parse::<usize>().unwrap() - 1,
                           words[5].parse::<usize>().unwrap() - 1));
    }
    // Get column metadata
    let num_cols = lines.next().unwrap().split("   ").last().unwrap().trim().parse::<usize>().unwrap();
    // Setup column data structure
    let mut stacks: Vec<Vec<u8>> = vec![vec![]; num_cols];
    // Gather column items
    for line in lines {
        let bline = line.as_bytes();
        for i in 0..(line.len()/4+1) {
            let letter = bline[i*4+1];
            if letter != b' ' {stacks[i].push(letter)}
        }
    }
    (instructions, stacks)
}

#[aoc(day5, part1)]
pub fn part1(input: &(Vec<Instruction>, Vec<Vec<u8>>)) -> String {
    let mut stacks = input.1.clone();
    for &(amount, from, to) in input.0.iter().rev() {
        for _ in 0..amount {
            let val = stacks[from].pop().unwrap();
            stacks[to].push(val);
        }
    }
    // Gather top letters
    let mut answer = "".to_string();
    for stack in stacks{
        answer.push(*stack.last().unwrap() as char);
    }
    answer
}

#[aoc(day5, part2)]
pub fn part2(input: &(Vec<Instruction>, Vec<Vec<u8>>)) -> String {
    let mut stacks = input.1.clone();
    for &(amount, from, to) in input.0.iter().rev() {
        let mut vals = Vec::with_capacity(amount);
        for _ in 0..amount {
            vals.push(stacks[from].pop().unwrap());
        }
        vals.reverse();
        stacks[to].append(&mut vals);
    }
    // Gather top letters
    let mut answer = "".to_string();
    for stack in stacks{
        answer.push(*stack.last().unwrap() as char);
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTINPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn gen_example() {
        let (instructions, stacks) = input_generator(TESTINPUT);
        assert_eq!(instructions, vec![(1,1,2),(2,2,1),(3,1,3),(1,2,1)]);
        assert_eq!(stacks, vec![vec![b'Z',b'N'],vec![b'M',b'C',b'D'],vec![b'P']]);
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&input_generator(TESTINPUT)), "CMZ");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&input_generator(TESTINPUT)), "MCD");
    }
}