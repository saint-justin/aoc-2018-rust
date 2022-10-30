pub mod solutions {
  use std::collections::HashMap;

  // Day 2 Part 1 -- https://adventofcode.com/2018/day/2
  pub fn find_dups(input_vec: &Vec<&str>) {
    let mut dups: (i32, i32) = (0, 0); // (doubles, triples)
    let mut map: HashMap<char, i32> = HashMap::new();
    
    for input in input_vec {
      // Make a map of all chars in a given string
      for ch in input.chars() {
        if map.contains_key(&ch) { 
          map.insert(ch, *map.get(&ch).unwrap() + 1); 
        } else { 
          map.insert(ch, 1); 
        }
      }

      // Count all doubles and triples found
      let mut copy_found: (bool, bool) = (false, false);
      for (_k, v) in &map {
        if !copy_found.0 && *v == 2 {
          dups.1 += 1;
          copy_found.0 = true;
        } else if !copy_found.1 && *v == 3 {
          dups.0 += 1;
          copy_found.1 = true;
        }
        if copy_found == (true, true) { break; } // escape early if both have been found
      }
      map.clear();
    }

    let checksum = dups.0 * dups.1;
    println!("Values counted: \n  Doubles: {}\n  Triples: {}\n  Checksum value: {checksum}", dups.0, dups.1);
  }

  // Day 2 Part 2 https://adventofcode.com/2018/day/2#part2
  pub fn find_commons(input_vec: &Vec<&str>) {
    fn compare_strings(str_1: &Vec<char>, str_2: &Vec<char>) -> (bool, usize) {
      let mut diffs_found = false;
      let mut diff_index: usize = usize::MAX;

      let mut i = 0;
      while i < str_1.len() {
        if str_1[i] != str_2[i] {
          if !diffs_found { 
            diffs_found = true; 
            diff_index = i;
          } else { 
            return (false, 0) 
          } 
        }

        i += 1;
      } 

      println!("Strings:\n  {}\n  {}", str_1.iter().collect::<String>(), str_2.iter().collect::<String>());
      return (true, diff_index);
    }

    // Iterate over each string pair and checking if they're valid
    let mut i = 0;
    while i < input_vec.len() - 1 {
      let mut j = i + 1;
      while j < input_vec.len() {
        let id_1 = input_vec[i];
        let id_2 = input_vec[j];

        println!("New set of items:  {id_1}  {id_2}");
        let comparison: (bool, usize) = compare_strings(&id_1.chars().collect(), &id_2.chars().collect());
        if comparison.0 {
          println!("\n  Match found! \n  Diff at position {}", comparison.1);
          return;
        }

        j += 1;
      }

      i += 1;
    }
  }
}

