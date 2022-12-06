pub mod first_puzzle {
    use std::collections::HashSet;

    pub fn solution() -> usize {
        const LOWERCASE_CODE_OFFSET: usize = 96;
        const UPPERCASE_CODE_OFFSET: usize = 38;

        let input = std::fs::read_to_string("./puzzle_inputs/day_three.txt").expect("Cannot read input file.");

        let mut rucksacks = input.lines();
        let mut priority_sum = 0;

        while let Some(rucksack) = rucksacks.next() {
            let (first_compartment, second_compartment) = rucksack.split_at(rucksack.len() / 2);

            let first_set: HashSet<char> = first_compartment.chars().collect();
            let second_set: HashSet<char> = second_compartment.chars().collect();

            let shared_item = first_set.intersection(&second_set).next().unwrap();

            if shared_item.is_lowercase() {
                priority_sum += *shared_item as usize - LOWERCASE_CODE_OFFSET;
            } else {
                priority_sum += *shared_item as usize - UPPERCASE_CODE_OFFSET;
            }
        }

        priority_sum
    }
}

pub mod second_puzzle {
    pub fn solution() -> i32 {
        unimplemented!()
    }
}
