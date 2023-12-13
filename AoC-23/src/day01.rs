use std::collections::LinkedList;

use crate::puzzle::Puzzle;
pub struct Day01;
impl Day01 {
    fn remove_first_character(s: &mut String) {
        if s.len() == 1 {
            s.clear();
            return;
        }
        s.remove(0);
    }

    fn get_nums(line: &str) -> LinkedList<u32> {
        let line_copy = String::from(line);
        let mut unprocessed = line_copy.clone();
        let mut res: LinkedList<u32> = LinkedList::new();

        while !unprocessed.is_empty() {
            let char = unprocessed.chars().next().unwrap();
            if char.is_digit(10) {
                res.push_back(char.to_digit(10).unwrap());
                Self::remove_first_character(&mut unprocessed);
            } else if unprocessed.starts_with("one") {
                res.push_back(1);
                Self::remove_first_character(&mut unprocessed);
            } else if unprocessed.starts_with("two") {
                res.push_back(2);
                Self::remove_first_character(&mut unprocessed);
            } else if unprocessed.starts_with("three") {
                res.push_back(3);
                Self::remove_first_character(&mut unprocessed);
            } else if unprocessed.starts_with("four") {
                res.push_back(4);
                Self::remove_first_character(&mut unprocessed);
            } else if unprocessed.starts_with("five") {
                res.push_back(5);
                Self::remove_first_character(&mut unprocessed);
            } else if unprocessed.starts_with("six") {
                res.push_back(6);
                Self::remove_first_character(&mut unprocessed);
            } else if unprocessed.starts_with("seven") {
                res.push_back(7);
                Self::remove_first_character(&mut unprocessed);
            } else if unprocessed.starts_with("eight") {
                res.push_back(8);
                Self::remove_first_character(&mut unprocessed);
            } else if unprocessed.starts_with("nine") {
                res.push_back(9);
                Self::remove_first_character(&mut unprocessed);
            } else {
                Self::remove_first_character(&mut unprocessed);
            }
        }
        res
    }
}

impl Puzzle for Day01 {
    fn second_star(data: String) -> String {
        let mut sum = 0;
        for line in data.lines() {
            let nums = Self::get_nums(line);
            sum += 10 * nums.front().unwrap() + nums.back().unwrap();
        }
        sum.to_string()
    }
    fn first_star(data: String) -> String {
        let mut sum = 0;
        for line in data.lines() {
            let nums = line
                .chars()
                .map(|c| if c.is_digit(10) { c.to_digit(10) } else { None })
                .flatten()
                .collect::<Vec<_>>();
            sum += 10 * nums.first().unwrap() + nums.last().unwrap();
        }
        sum.to_string()
    }
}
