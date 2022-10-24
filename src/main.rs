use std::{fs};
mod day_01;

fn main() {
    let file_path: String = String::from("./src/day_01/input_01.txt");
    println!("Starting to solve for file at path {file_path}");
    
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let content_arr: Vec<&str> = contents.split(['\n']).collect();

    let out = day_01::solutions::solve_for_first_repeat(content_arr);
    println!("Repeated value: {out}");

}


