pub mod first_puzzle {
    use std::collections::HashSet;

    pub fn solution() -> usize {
        let input = std::fs::read_to_string("./puzzle_inputs/day_three.txt").expect("Cannot read input file.");
        let rucksacks = input.lines();

        rucksacks.fold(0, |sum, rucksack| {
            let (first_compartment, second_compartment) = rucksack.split_at(rucksack.len() / 2);

            let first_set: HashSet<char> = first_compartment.chars().collect();
            let second_set: HashSet<char> = second_compartment.chars().collect();

            let shared_item = first_set.intersection(&second_set).next().unwrap();

            sum + super::priority_of(shared_item)
        })
    }
}

pub mod second_puzzle {
    pub fn solution() -> i32 {
        unimplemented!()
    }
}

fn priority_of(item: &char) -> usize {
    const LOWERCASE_CODE_OFFSET: usize = 96;
    const UPPERCASE_CODE_OFFSET: usize = 38;

    if item.is_lowercase() {
        *item as usize - LOWERCASE_CODE_OFFSET
    } else {
        *item as usize - UPPERCASE_CODE_OFFSET
    }
}
