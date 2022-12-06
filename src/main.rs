mod day_one;
mod day_two;
mod day_three;

fn main() {
    println!("Day one first puzzle answer: {}", day_one::first_puzzle::solution());
    println!("Day one second puzzle answer: {}", day_one::second_puzzle::solution());

    println!("Day two first puzzle answer: {}", day_two::first_puzzle::solution());
    println!("Day two second puzzle answer: {}", day_two::second_puzzle::solution());

    println!("Day three first puzzle answer: {}", day_three::first_puzzle::solution());
    println!("Day three second puzzle answer: {}", day_three::second_puzzle::solution());
}
