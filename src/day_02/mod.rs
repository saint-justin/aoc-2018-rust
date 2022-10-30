pub mod solutions {
  use std::collections::HashMap;


  pub fn find_dups(input_vec: &Vec<&str>) {
    let mut dups: (i32, i32) = (0, 0); // (doubles, triples)
    let mut map: HashMap<char, i32> = HashMap::new();
    
    for input in input_vec {
      for ch in input.chars() {
        if map.contains_key(&ch) { 
          map.insert(ch, *map.get(&ch).unwrap() + 1); 
        } else { 
          map.insert(ch, 1); 
        }
      }

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
    println!("Checksum value: {checksum}");
  }
}