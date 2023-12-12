pub mod day01;
pub mod day02;
pub mod puzzle;
pub mod utils;

use day01::Day01;
use day02::Day02;
use puzzle::Puzzle;

fn main() {
    println!("{}", Day01::first_star());
    println!("{}", Day01::second_star());

    println!("{}", Day02::first_star());
    println!("{}", Day02::second_star());
}
