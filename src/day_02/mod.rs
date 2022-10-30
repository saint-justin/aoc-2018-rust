pub mod solutions {
  use std::collections::HashMap;

  // Day 2 Part 1 -- https://adventofcode.com/2018/day/2
  pub fn find_dups(input_vec: &Vec<&str>) -> i32 {
    println!("\nStarting solve for doubles and triples...");
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

    println!("  Values counted: \n  Doubles: {}\n  Triples: {}", dups.0, dups.1);
    return dups.0 * dups.1;
  }

  // Day 2 Part 2 https://adventofcode.com/2018/day/2#part2
  pub fn find_common_key(input_vec: &Vec<&str>) -> String {
    fn compare_strings(str_1: &Vec<char>, str_2: &Vec<char>) -> (bool, usize) {
      let mut diffs_found = false;
      let mut diff_index: usize = usize::MAX;  // fail-to-solve defaults to usize::MAX

      for i  in 0..str_1.len() {
        if str_1[i] != str_2[i] {
          if !diffs_found { 
            diffs_found = true; 
            diff_index = i;
          } else { 
            return (false, 0);
          } 
        }
      } 
      return (true, diff_index);
    }

    // Iterate over each string pair and checking if they're valid
    println!("\nStarting solve for common key...");

    for i  in 0..input_vec.len() - 1 {
      for j in i + 1..input_vec.len() {
        let (valid, diff_index) = compare_strings(&input_vec[i].chars().collect(), &input_vec[j].chars().collect());
        if valid {
          println!("  Match found! \n  Diff at position {} of keys: \n    {}\n    {}", diff_index, &input_vec[i], &input_vec[j]);
          let input_str = input_vec[i];
          let mut key: String = String::from(&input_str[..diff_index]); // Grab the 1st half of the key
          key.push_str(&input_str[diff_index + 1..]); // Add the 2nd half of the key
          return key;
        }
      }
    }

    return String::from("Error, no key found");
  }
}

