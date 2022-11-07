pub mod solutions {
  use std::collections::HashMap;

  #[derive(Eq, Hash, PartialEq, Debug)]
  struct Point {x: u32, y: u32}

  struct Dimensions {h: u32, w: u32}

  fn remove_last_char(string: &str) -> &str {
    let mut chars = string.chars();
    chars.next_back();
    chars.as_str()
  }

   // Day 3 Part 1 -- https://adventofcode.com/2018/day/3
   // Find the number of overlapping squares given a list of sizes and positions
  pub fn find_multiclaim_squares (input: &Vec<&str>) -> usize {
    // Claim bool: false = claimed, true = multiclaimed
    let claimed_squares: HashMap<Point, bool> = generate_map_from_input(input);

    return claimed_squares
      .into_iter()
      .filter(|pair| pair.1 == true )
      .collect::<Vec<_>>().len();
  }

  // Day 3 Part 2 -- https://adventofcode.com/2018/day/3#part2
  // Find the single entry that overlaps with no others
  pub fn find_island_square(input: &Vec<&str>) -> String {
    let claimed_squares: HashMap<Point, bool> = generate_map_from_input(input);

    'claim: for claim in input {
      let claim_split = claim.split(' ').collect::<Vec<&str>>();

      let positions_split = claim_split[2].split(',').collect::<Vec<&str>>();
      let position = Point {
        x: positions_split[0].parse().unwrap(),
        y: remove_last_char(&positions_split[1]).parse().unwrap(),
      };

      let dimensions_split = claim_split[3]
        .split('x')
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
      let dimensions = Dimensions {
        w: dimensions_split[0],
        h: dimensions_split[1],
      };

      for x in position.x .. position.x + dimensions.w {
        for y in position.y .. position.y + dimensions.h {
          let pos = Point {x, y};
          match claimed_squares.get(&pos) {
            Some (claim_bool) => {  
              if *claim_bool {
                continue 'claim;
              }
            },
            None => {
              panic!("Position not accounted for: [{x},{y}]");
            },
          }
        }
      }

      println!("Solution found!");
      return String::from(claim_split[0]);
    }

    return String::from("mama mia, no solution");
  }

  
  fn generate_map_from_input (input: &Vec<&str>) -> HashMap<Point, bool>{
    let mut claimed_squares: HashMap<Point, bool> = HashMap::new();

    // Parse squares claimed per plot from input
    for claim in input {
      let claim_split = claim.split(' ').collect::<Vec<&str>>();  //0: Plot #, 1: '@', 2: position, 3: height x width
  
      let positions_split = claim_split[2].split(',').collect::<Vec<&str>>();
      let position = Point {
        x: positions_split[0].parse().unwrap(),
        y: remove_last_char(&positions_split[1]).parse().unwrap(),
      };
  
      let dimensions_split = claim_split[3].split('x').map(|n| n.parse::<u32>().unwrap()).collect::<Vec<u32>>();
      let dimensions = Dimensions {
        w: dimensions_split[0],
        h: dimensions_split[1],
      };
  
      // Add squares to claim
      for x in position.x .. position.x + dimensions.w {
        for y in position.y .. position.y + dimensions.h {
          let pos = Point {x, y};
          match claimed_squares.get(&pos) {
            Some (claim_bool) => {  
              if !claim_bool {
                claimed_squares.insert(pos, true);
              }
            },
            None => {
              claimed_squares.insert(pos, false);
            },
          }
        }
      }
    }

    return claimed_squares;
  }
    
    // claimed_squares
    // .into_iter()
    // .filter(|pair| pair.1 == true )
      // .collect::<Vec<_>>().len();
  // }
}