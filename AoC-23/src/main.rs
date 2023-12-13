pub mod day01;
pub mod day02;
pub mod puzzle;
pub mod utils;

use day01::Day01;
use day02::Day02;
use puzzle::Puzzle;

use crate::utils::read_input;

fn main() {
    println!("{}", Day01::first_star(read_input(1)));
    println!("{}", Day01::second_star(read_input(1)));

    println!("{}", Day02::first_star(read_input(2)));
    println!("{}", Day02::second_star(read_input(2)));
}
