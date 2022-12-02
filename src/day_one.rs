pub mod day_one {
    pub mod first_puzzle {
        use super::sorted_total_calories;

        pub fn solution() -> i32 {
            sorted_total_calories().pop().unwrap()
        }
    }

    pub mod second_puzzle {
        use super::sorted_total_calories;

        pub fn solution() -> i32 {
            let mut calories = sorted_total_calories();
            let last_three_range = calories.len()-3..;

            calories.drain(last_three_range).sum()
        }
    }

    fn sorted_total_calories() -> Vec<i32> {
        let input_str = std::fs::read_to_string("./puzzle_inputs/day_one.txt").expect("Cannot read input file.");
        let mut calorie_sum: i32 = 0;
        let mut summed_calories: Vec<i32> = vec![];

        for calorie in input_str.lines() {
            if !calorie.is_empty() {
                let cal: i32 = calorie.parse::<i32>().unwrap();
                calorie_sum += cal;
            } else { // If line is empty, store this Elf's calories sum and move on to the next.
                summed_calories.push(calorie_sum);
                calorie_sum = 0;
            }
        }

        summed_calories.sort();
        summed_calories
    }
}
