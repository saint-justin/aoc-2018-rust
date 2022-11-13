use std::{fs};
mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;

fn main() {
    let file_path: String = format_input_string(5, 0);
    println!("Starting to solve for file at path {file_path}");
    
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let content_arr: Vec<&str> = contents
        .split(['\n'])
        .map(|e| e.trim())
        .collect();

    /*
    let mut out = day_01::solutions::sum_all(&content_arr);
    println!("  Sum of frequencies: [ {out} ]");
    out = day_01::solutions::solve_for_first_repeat(&content_arr);
    println!("  Repeated value: [ {out} ]");    

    let checksum = day_02::solutions::find_dups(&content_arr);
    println!("  Checksum Value: [ {checksum} ]");
    let key = day_02::solutions::find_common_key(&content_arr);
    println!("  Common Key: [ {key} ]")    

    let multiclaim_squares = day_03::solutions::find_multiclaim_squares(&content_arr);
    println!("  Squares claimed multiples times: {multiclaim_squares}");
    let island_square_id = day_03::solutions::find_island_square(&content_arr);
    println!("  Island squares ID: {island_square_id}");

    let out: (u32, u32) = day_04::solutions::find_sleepiest_guard(&content_arr);
    */

    // let mut polymer_remainder = day_05::solutions::collapse_polymer(&content_arr);
    // println!("  Polymer count remaining: {}", polymer_remainder.len());
    let polymer_remainder = day_05::solutions::collapse_unblocked_polymer(&content_arr);
    println!("  Polymer length remaining: {}", polymer_remainder);
}

/// # format_input
/// Formates a date by input and whether its a test
/// ## Args
/// * `input_day` - The day which the test is targeting
/// * `test_number` - Which test to run, where zero is the true input
fn format_input_string(input_day: u32, test_number: u32) -> String {
    if test_number == 0 {
        return String::from(format!("./src/inputs/input_0{input_day}.txt"));
    } else {
        return String::from(format!("./src/inputs/input_0{input_day}_test_0{test_number}.txt"));
    }
}
