use std::{fs};
mod day_01;
mod day_02;

fn main() {
    let file_path: String = String::from("./src/inputs/input_02.txt");
    println!("Starting to solve for file at path {file_path}");
    
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let content_arr: Vec<&str> = contents.split(['\n']).map(|e| e.trim()).collect();

    // day_01(&content_arr);
    day_02(&content_arr);
}

/*
fn day_01(input_vec: &Vec<&str>) {
    // Part 1/2
    let mut out = day_01::solutions::sum_all(&input_vec);
    println!("  Sum of frequencies: [ {out} ]");
    out = day_01::solutions::solve_for_first_repeat(&input_vec);
    println!("  Repeated value: [ {out} ]");
}
*/

fn day_02(input_vec: &Vec<&str>) {
    // Part 1/2
    let checksum = day_02::solutions::find_dups(&input_vec);
    println!("  Checksum Value: [ {checksum} ]");
    let key = day_02::solutions::find_common_key(&input_vec);
    println!("  Common Key: [ {key} ]")
}