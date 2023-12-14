use std::collections::LinkedList;

use crate::puzzle::Puzzle;

pub struct Day03;
#[derive(Clone, Debug)]
struct Symbol {
    value: char,
    marked: bool,
}
struct Point {
    x: usize,
    y: usize,
}
impl Symbol {
    fn mark(&mut self) {
        self.marked = true;
    }
}
impl Day03 {
    fn is_symbol(c: &char) -> bool {
        !c.is_digit(10) && *c != '.'
    }
    fn should_mark(board: &Vec<Vec<char>>, point: Point) -> bool {
        if point.x == 0 || point.y == 0 {
            return false;
        };
        if !board[point.x][point.y].is_digit(10) {
            return false;
        }
        if Self::is_symbol(&board[point.x - 1][point.y - 1]) {
            return true;
        };
        if Self::is_symbol(&board[point.x - 1][point.y]) {
            return true;
        };
        if Self::is_symbol(&board[point.x - 1][point.y + 1]) {
            return true;
        };
        if Self::is_symbol(&board[point.x][point.y - 1]) {
            return true;
        };
        if Self::is_symbol(&board[point.x][point.y + 1]) {
            return true;
        };
        if Self::is_symbol(&board[point.x + 1][point.y - 1]) {
            return true;
        };
        if Self::is_symbol(&board[point.x + 1][point.y]) {
            return true;
        };
        if Self::is_symbol(&board[point.x + 1][point.y + 1]) {
            return true;
        };
        return false;
    }

    fn convert_to_symbols(board: &Vec<Vec<char>>) -> Vec<Vec<Symbol>> {
        let mut res = vec![
            vec![
                Symbol {
                    value: '.',
                    marked: false,
                };
                board.len()
            ];
            board.len()
        ];
        for (x, line) in board.iter().enumerate() {
            if x == 0 || x == board.len() - 1 {
                continue;
            }
            for (y, char) in line.iter().enumerate() {
                if y == 0 || y == board.len() - 1 {
                    continue;
                }
                res[x][y] = Symbol {
                    value: char.to_owned(),
                    marked: Self::should_mark(board, Point { x, y }),
                };
            }
        }
        res
    }

    fn create_board(data: String) -> Vec<Vec<char>> {
        let lines: Vec<String> = data
            .lines()
            .map(|l| l.to_string().trim_start().to_string())
            .collect::<Vec<String>>();
        let len = lines.len();
        let mut res: Vec<Vec<char>> = vec![vec!['.'; len + 2]; len + 2];
        for (x, line) in lines.iter().enumerate() {
            for (y, char) in line.to_owned().chars().enumerate() {
                res[x + 1][y + 1] = char;
            }
        }
        res
    }
    fn expand_marks(symbols: &mut Vec<Vec<Symbol>>) {
        for _i in 0..10 {
            let symbols_copy = symbols.to_vec();
            let mut has_change = false;
            for (x, line) in symbols_copy.iter().enumerate() {
                for (y, symbol) in line.iter().enumerate() {
                    if symbol.marked {
                        if symbols_copy[x][y - 1].value.is_digit(10) {
                            has_change = true;
                            (*symbols)[x][y - 1].mark();
                        }

                        if symbols_copy[x][y + 1].value.is_digit(10) {
                            has_change = true;
                            (*symbols)[x][y + 1].mark();
                        }
                    }
                }
            }
            if !has_change {
                break;
            }
        }
    }
}
impl Puzzle for Day03 {
    fn first_star(data: String) -> String {
        let board = Self::create_board(data);
        let mut symbols = Self::convert_to_symbols(&board);
        Self::expand_marks(&mut symbols);
        let mut res: LinkedList<String> = LinkedList::new();
        for x in 0..symbols.len() {
            let mut res_s: String = String::new();
            for y in 0..symbols.len() {
                if symbols[x][y].marked {
                    res_s.push(symbols[x][y].value);
                } else {
                    res.push_back(res_s);
                    res_s = String::new();
                }
            }
        }
        let nums = res
            .iter()
            .filter(|r| !r.is_empty())
            .map(|x| x.parse::<i32>().unwrap());
        let sum: i32 = nums.reduce(|acc, e| acc + e).unwrap();
        sum.to_string()
    }

    fn second_star(data: String) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn first_task() {
        let test_data = "467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..";
        let result = Day03::first_star(test_data.to_string());
        assert_eq!(result, "4361".to_string());
    }
}
