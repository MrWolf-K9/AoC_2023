use crate::puzzle::Puzzle;

pub struct Day02;
impl Day02 {}
impl Puzzle for Day02 {
    fn first_star(data: String) -> String {
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
        let test_data = "";
        let result = Day02::first_star(test_data.to_string());
        assert_eq!(result, "Not implemented".to_string());
    }
}
