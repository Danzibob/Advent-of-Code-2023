#[aoc(day1, part1)]
pub fn part1(input: &str) -> usize {
    let mut this_elf = 0;
    let mut max_elf_val = 0;
    input.lines().for_each(|n| 
        if n == "" {
            if this_elf > max_elf_val {
                max_elf_val = this_elf;
            }
            this_elf = 0;
        } else {
            this_elf += n.parse::<usize>().unwrap();
        }
    );
    max_elf_val
}

pub fn insert_last(v: &mut Vec::<usize>) {
    let val = v.pop().unwrap();
    let pos = v.binary_search(&val).unwrap_or_else(|e| e);
    if pos > 0 || v.len() < 3 {
    v.insert(pos, val);
    }
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> usize {
    let mut elves: Vec<usize> = Vec::new();
    elves.push(0);
    input.lines().for_each(|item| 
        if item == "" {
            insert_last(&mut elves);
            elves.push(0);
        } else {
            *elves.last_mut().unwrap() += item.parse::<usize>().unwrap();
        }
    );
    insert_last(&mut elves);
    elves[elves.len()-3..].iter().sum()
}