pub mod solutions {
    use std::{time::{SystemTime, UNIX_EPOCH}, alloc::System};

  // Collapse a polymer of of characters 'til they stop reacting.
  pub fn collapse_polymer(input_vec: &Vec<&str>) -> Vec<String> {
    let mut polymer_vec: Vec<String> = input_vec[0].chars().into_iter().map(|ch| ch.to_string()).collect();
    let start_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let mut i: usize = 0;
    while i + 1 < polymer_vec.len() {
      if polymers_are_collapsable(&polymer_vec[i], &polymer_vec[i+1]) {
        polymer_vec.remove(i+1);
        polymer_vec.remove(i);
        if i >= 1 { i -= 1; } // with this line swapped to i = 1 duration is 34s -- with this line, duration < 1s (932ms)
        continue;
      }

      i += 1;
    }

    return polymer_vec;
  }

  fn polymers_are_collapsable(polymer_1: &String, polymer_2: &String) -> bool {
    // Check polymers of of same character type
    if polymer_1.to_lowercase() != polymer_2.to_lowercase() {
      return false
    }
    return are_opposite_types(&polymer_1, &polymer_2)
  }

  fn are_opposite_types(polymer_1: &String, polymer_2: &String) -> bool {
    if (is_capital(&polymer_1) && !is_capital(&polymer_2)) || (!is_capital(&polymer_1) && is_capital(&polymer_2)) {
      return true
    }
    return false
  }

  fn is_capital(polymer: &String) -> bool {
    polymer == &(polymer.to_uppercase())
  }
}