use std::fs;
fn main() {
    let some_file = "/home/cvlvxi/Desktop/Projects/Chromium/src/ash/capture_mode/capture_mode_unittests.cc";
    let contents = fs::read_to_string(some_file);
    println!("{}", contents.unwrap());
}
