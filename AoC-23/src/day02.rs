use core::panic;

use crate::puzzle::Puzzle;

pub struct Day02;
struct Taken(u64, u64, u64);
enum Colors {
    R,
    G,
    B,
}
struct Color {
    color: Colors,
    count: u8,
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
                Colors::R => r = r + c.count,
                Colors::G => g = g + c.count,
                Colors::B => b = b + c.count,
            }
        }
        Taken(r.into(), g.into(), b.into())
    }

    fn parse_color(color: String) -> Color {
        let trimmed = color.trim();
        let splitted: Vec<&str> = trimmed.split(' ').collect();
        match splitted[1] {
            "red" => Color {
                color: Colors::R,
                count: splitted[0].parse::<u8>().unwrap(),
            },
            "blue" => Color {
                color: Colors::B,
                count: splitted[0].parse::<u8>().unwrap(),
            },
            "green" => Color {
                color: Colors::G,
                count: splitted[0].parse::<u8>().unwrap(),
            },
            _ => panic!(),
        }
    }

    fn is_possible(taken: &Vec<Taken>) -> bool {
        const R_MAX: u8 = 12;
        const G_MAX: u8 = 13;
        const B_MAX: u8 = 14;

        for t in taken {
            if t.0 > R_MAX.into() || t.1 > G_MAX.into() || t.2 > B_MAX.into() {
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
        let lines: Vec<String> = data.lines().map(|l| l.to_string()).collect::<Vec<String>>();
        let games = lines.iter().map(|l| Self::parse_game(l.to_string()));
        let mut result = 0;
        for g in games {
            let max_taken: Taken = g.iter().fold(Taken(0, 0, 0), |acc, e| {
                Taken(
                    if e.0 > acc.0 { e.0 } else { acc.0 },
                    if e.1 > acc.1 { e.1 } else { acc.1 },
                    if e.2 > acc.2 { e.2 } else { acc.2 },
                )
            });
            //println!("{} {} {}", max_taken.0, max_taken.1, max_taken.2);
            result = result + (max_taken.0 * max_taken.1 * max_taken.2);
        }

        result.to_string()
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

    #[test]
    fn second_task() {
        let test_data = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = Day02::second_star(test_data.to_string());
        assert_eq!(result, "2286".to_string());
    }
}
