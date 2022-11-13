pub mod solutions {
    use std::collections::HashSet;


  // Collapse a polymer of of characters 'til they stop reacting.
  pub fn collapse_polymer(input_vec: &Vec<String>) -> Vec<String> {
    let mut polymer_vec: Vec<String> = input_vec.clone();

    let mut i: usize = 0;
    while i + 1 < polymer_vec.len() {
      if polymers_are_collapsable(&polymer_vec[i], &polymer_vec[i+1]) {
        polymer_vec.remove(i+1);
        polymer_vec.remove(i);
        if i >= 1 { i -= 1; } // with this line swapped to i = 1 duration is 34s -- with this line, duration < 1s
        continue;
      }
      i += 1;
    }

    return polymer_vec;
  }

  // Find the polymer blocker type and collapse the chain with all instances of it removed.
  pub fn collapse_unblocked_polymer(input_vec: &Vec<&str>) -> usize {
    let full_polymer_vec: Vec<String> = input_vec[0].chars().into_iter().map(|ch| ch.to_string()).collect();
    
    // Generate list of all polymer types
    let polymer_vec_lowercase: Vec<String> = (&full_polymer_vec).into_iter().map(|ch| ch.to_lowercase()).collect();
    let mut all_polymer_types: HashSet<String> = HashSet::new();
    for polymer_strand in polymer_vec_lowercase {
      all_polymer_types.insert(polymer_strand);
    }

    // Cycle through all polymer types and look for the shortest chain
    let mut shortest_chain = usize::MAX;
    for polymer_type in all_polymer_types {
      let mut unblocked_polymer_vec = full_polymer_vec.clone();
      unblocked_polymer_vec.retain(|strand| !polymer_types_match(&polymer_type, &strand));

      let collapsed_polymer = collapse_polymer(&unblocked_polymer_vec);
      let chain_length = collapsed_polymer.len();
      if chain_length < shortest_chain { shortest_chain = chain_length; }
    }

    return shortest_chain;
  }

  // Helper Expressions --------------------------------------------------------
  fn polymer_types_match(strand_1: &String, stand_2: &String) -> bool {
    if is_capital(strand_1) {
      return (strand_1 == stand_2) || (&strand_1.to_lowercase() == stand_2)
    } 
    return (strand_1 == stand_2) || (&strand_1.to_uppercase() == stand_2)
  }

  fn polymers_are_collapsable(polymer_1: &String, polymer_2: &String) -> bool {
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