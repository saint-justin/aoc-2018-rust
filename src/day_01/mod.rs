pub mod solutions {
  use std::collections::HashMap;

  pub fn sum_all(input_vec: &Vec<&str>) -> i32 {
    return input_vec.iter().map(|str| str.trim().parse::<i32>().unwrap()).sum();
  }

  pub fn solve_for_first_repeat(input_vec: &Vec<&str>) -> i32 {
      let mut current_freq: i32 = 0;
      let mut visited_freqs: HashMap<String, bool> = HashMap::new();
  
      let int_vec: Vec<i32> = input_vec
          .iter()
          .map(|str| str.trim().parse::<i32>().unwrap())
          .collect::<Vec<i32>>();
  
      loop {
          for freq_mod in int_vec.iter() {
              current_freq += freq_mod;
              if visited_freqs.contains_key(&current_freq.to_string()) {
                  return current_freq;
              } else {
                  visited_freqs.insert(current_freq.to_string(), true);
              }
          }
      };
  }
}
