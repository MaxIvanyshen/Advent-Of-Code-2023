use std::fs::read_to_string;

fn main() {
    let file_content = read_to_string("input").unwrap();
    let path: Vec<char> = file_content.lines().collect::<Vec<&str>>()[0].chars().collect();

}
