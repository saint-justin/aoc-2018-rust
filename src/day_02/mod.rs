pub mod solutions {
  use std::collections::HashMap;

  pub fn find_dups(input_vec: &Vec<&str>) {
    let mut map: HashMap<char, i32> = HashMap::new();
    let mut multiples: (i32, i32) = (0, 0);
    
    for id in input_vec {
      for ch in id.chars() {
        if map.contains_key(&ch) { map.insert(ch, *map.get(&ch).unwrap() + 1); } else { map.insert(ch, 1); }
      }

      let mut copy_found: (bool, bool) = (false, false);
      for (_k, v) in &map {
        if !copy_found.0 && v == &i32::from(2) {
          multiples = (multiples.0, multiples.1 + 1);
          copy_found = (true, copy_found.1);
        } else if !copy_found.1 && v == &i32::from(3) {
          multiples = (multiples.0 + 1, multiples.1);
          copy_found = (copy_found.0, true);
        }
        if copy_found == (true, true) { break; }
      }
      map.clear();
    }

    let checksum = multiples.0 * multiples.1;
    println!("Checksum value: {checksum}");
  }
}