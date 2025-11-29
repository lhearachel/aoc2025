pub mod aoc {
    pub fn load_file(root: &str, day: u32) -> String {
        let path = format!("{root}/{day:02}.txt");
        let data = std::fs::read_to_string(path);
        data.expect("could not open input file")
    }
}
