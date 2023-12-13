use std::io::BufRead;

use crate::puzzle::Puzzle;

pub struct Day02;
struct Taken {
    r: u8,
    g: u8,
    b: u8,
}
impl Day02 {
    fn parse_game(line: String) -> Vec<Taken> {
        println!("{}", line);
        let start_index = line.find(':').unwrap() + 2;
        let stripped = &line[start_index..];
        let takes: Vec<&str> = stripped.split(';').collect();
        println!("{:?}", takes);
        let takens: Vec<Taken> = takes
            .iter()
            .map(|take| Self::parse_take(take.to_string()))
            .collect();
        return takens;
    }
    fn parse_take(take: String) -> Taken {
        let trimmed = take.trim();
        println!("{}", trimmed);
        Taken { r: 0, g: 0, b: 0 }
    }
}
impl Puzzle for Day02 {
    fn first_star(data: String) -> String {
        for line in data.lines() {
            Self::parse_game(line.to_string());
        }
        String::from("Not implemented")
    }

    fn second_star(data: String) -> String {
        String::from("Not implemented")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_data = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = Day02::first_star(test_data.to_string());
        assert_eq!(result, "8".to_string());
    }
}
