use core::panic;

use crate::puzzle::Puzzle;

pub struct Day02;
struct Taken(u8, u8, u8);
enum Colors {
    R,
    G,
    B,
}
struct Color {
    color: Colors,
    amounght: u8,
}
impl Day02 {
    fn parse_game(line: String) -> Vec<Taken> {
        let start_index = line.find(':').unwrap() + 2;
        let stripped = &line[start_index..];
        let takes: Vec<&str> = stripped.split(';').collect();
        let takens: Vec<Taken> = takes
            .iter()
            .map(|take| Self::parse_take(take.to_string()))
            .collect();
        return takens;
    }
    fn parse_take(take: String) -> Taken {
        let trimmed = take.trim();
        let splitted: Vec<&str> = trimmed.split(',').collect();
        let colors: Vec<Color> = splitted
            .iter()
            .map(|c| Self::parse_color(c.to_string()))
            .collect();
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        for c in colors {
            match c.color {
                Colors::R => r = r + c.amounght,
                Colors::G => g = g + c.amounght,
                Colors::B => b = b + c.amounght,
            }
        }
        Taken(r, g, b)
    }

    fn parse_color(color: String) -> Color {
        let trimmed = color.trim();
        let splitted: Vec<&str> = trimmed.split(' ').collect();
        match splitted[1] {
            "red" => Color {
                color: Colors::R,
                amounght: splitted[0].parse::<u8>().unwrap(),
            },
            "blue" => Color {
                color: Colors::B,
                amounght: splitted[0].parse::<u8>().unwrap(),
            },
            "green" => Color {
                color: Colors::G,
                amounght: splitted[0].parse::<u8>().unwrap(),
            },
            _ => panic!(),
        }
    }

    fn is_possible(taken: &Vec<Taken>) -> bool {
        const R_MAX: u8 = 12;
        const G_MAX: u8 = 13;
        const B_MAX: u8 = 14;

        for t in taken {
            if t.0 > R_MAX || t.1 > G_MAX || t.2 > B_MAX {
                return false;
            }
        }
        true
    }
}
impl Puzzle for Day02 {
    fn first_star(data: String) -> String {
        let lines: Vec<String> = data.lines().map(|l| l.to_string()).collect::<Vec<String>>();
        let games = lines.iter().map(|l| Self::parse_game(l.to_string()));
        let mut result: usize = 0;
        for (pos, e) in games.map(|g| Self::is_possible(&g)).enumerate() {
            if e {
                result = result + pos + 1
            }
        }
        result.to_string()
    }

    fn second_star(data: String) -> String {
        String::from("Not implemented")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn first_task() {
        let test_data = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = Day02::first_star(test_data.to_string());
        assert_eq!(result, "8".to_string());
    }
}
