pub fn read_input(day: usize) -> String {
    std::fs::read_to_string(format!(
        "{}/data/day{:0>2}.txt",
        std::env::current_dir().unwrap().display(),
        day
    ))
    .unwrap()
}
