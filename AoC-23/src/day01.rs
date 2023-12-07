use std::collections::LinkedList;
pub fn second_star() {
    let input = read_input(1);
    // let input = String::from(
    //     "two1nine
    //     eightwothree
    //     abcone2threexyz
    //     xtwone3four
    //     4nineeightseven2
    //     zoneight234
    //     7pqrstsixteen",
    // );
    let mut sum = 0;
    for line in input.lines() {
        let nums = get_nums(line);
        sum += 10 * nums.front().unwrap() + nums.back().unwrap();
    }
    println!("{}", sum);
}

fn remove_first_character(s: &mut String) {
    if s.len() == 1 {
        s.clear();
        return;
    }
    s.remove(0);
}

fn get_nums(line: &str) -> LinkedList<u32> {
    let line_copy = String::from(line);
    // println!("{}", line_copy);
    let mut unprocessed = line_copy.clone();
    let mut res: LinkedList<u32> = LinkedList::new();
    while !unprocessed.is_empty() {
        let char = unprocessed.chars().next().unwrap();
        if char.is_digit(10) {
            res.push_back(char.to_digit(10).unwrap());
            remove_first_character(&mut unprocessed);
        } else if unprocessed.starts_with("one") {
            res.push_back(1);
            remove_first_character(&mut unprocessed);
        } else if unprocessed.starts_with("two") {
            res.push_back(2);
            remove_first_character(&mut unprocessed);
        } else if unprocessed.starts_with("three") {
            res.push_back(3);
            remove_first_character(&mut unprocessed);
        } else if unprocessed.starts_with("four") {
            res.push_back(4);
            remove_first_character(&mut unprocessed);
        } else if unprocessed.starts_with("five") {
            res.push_back(5);
            remove_first_character(&mut unprocessed);
        } else if unprocessed.starts_with("six") {
            res.push_back(6);
            remove_first_character(&mut unprocessed);
        } else if unprocessed.starts_with("seven") {
            res.push_back(7);
            remove_first_character(&mut unprocessed);
        } else if unprocessed.starts_with("eight") {
            res.push_back(8);
            remove_first_character(&mut unprocessed);
        } else if unprocessed.starts_with("nine") {
            res.push_back(9);
            remove_first_character(&mut unprocessed);
        } else {
            remove_first_character(&mut unprocessed);
        }
    }
    // println!("{:?}", res);
    res
}

pub fn first_star() {
    let input = read_input(1);
    let mut sum = 0;
    for line in input.lines() {
        let nums = line
            .chars()
            .map(|c| if c.is_digit(10) { c.to_digit(10) } else { None })
            .flatten()
            .collect::<Vec<_>>();
        sum += 10 * nums.first().unwrap() + nums.last().unwrap();
    }
    println!("{}", sum);
}

fn read_input(day: usize) -> String {
    std::fs::read_to_string(format!(
        "{}\\data\\day{:0>2}.txt",
        std::env::current_dir().unwrap().display(),
        day
    ))
    .unwrap()
}
