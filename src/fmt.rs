use std::io::Write;
use std::{
    fs,
    io::{self, BufRead},
};
mod structs;
fn main() {
    let input_file = "src/main.rs";
    let f = fs::File::open(input_file).unwrap();
    let reader = io::BufReader::new(f);
    let mut lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    let start = lines.iter().position(|s| s.contains("fmtbegin")).unwrap();
    let end = lines.iter().position(|s| s.contains("fmtend")).unwrap();
    let s = lines[start + 1..end]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let token_tree = structs::InputStruct::parse_string(s).unwrap();
    let fmted = token_tree.to_string();
    let tmp_file_path = "src/main.tmp.rs";
    let mut tmp_file = fs::File::create(tmp_file_path).unwrap();
    lines.drain(start + 1..end);
    lines.insert(start + 1, fmted);
    for l in lines {
        writeln!(tmp_file, "{l}").unwrap();
    }
    tmp_file.flush().unwrap();
}
