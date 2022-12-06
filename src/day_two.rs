pub mod first_puzzle {
    pub fn solution() -> i32 {
        let input = std::fs::read_to_string("./puzzle_inputs/day_two.txt").expect("Cannot read input file.");
        let mut total_score = 0;
        let mut strategies = input.lines();

        while let Some(strategy) = strategies.next() {
            match strategy.split_once(' ') {
                // Quick and dirty. This is the way.
                Some(("A", "X")) => total_score += 4,
                Some(("A", "Y")) => total_score += 8,
                Some(("A", "Z")) => total_score += 3,
                Some(("B", "X")) => total_score += 1,
                Some(("B", "Y")) => total_score += 5,
                Some(("B", "Z")) => total_score += 9,
                Some(("C", "X")) => total_score += 7,
                Some(("C", "Y")) => total_score += 2,
                Some(("C", "Z")) => total_score += 6,
                _ => unreachable!()
            };
        };

        total_score
    }

}

pub mod second_puzzle {
    pub fn solution() -> i32 {
        let input = std::fs::read_to_string("./puzzle_inputs/day_two.txt").expect("Cannot read input file.");
        let mut total_score = 0;
        let mut strategies = input.lines();

        while let Some(strategy) = strategies.next() {
            match strategy.split_once(' ') {
                Some(("A", "X")) => total_score += 3,
                Some(("A", "Y")) => total_score += 4,
                Some(("A", "Z")) => total_score += 8,
                Some(("B", "X")) => total_score += 1,
                Some(("B", "Y")) => total_score += 5,
                Some(("B", "Z")) => total_score += 9,
                Some(("C", "X")) => total_score += 2,
                Some(("C", "Y")) => total_score += 6,
                Some(("C", "Z")) => total_score += 7,
                _ => unreachable!()
            }
        }

        total_score
    }
}
