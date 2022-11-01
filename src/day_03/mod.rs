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
  pub fn find_multiclaim_squares (input: &Vec<&str>) -> usize {
    // Claim bool: false = claimed, true = multiclaimed
    let mut claimed_squares: HashMap<Point, bool> = HashMap::new();

    // Parse squares claimed per plot from input
    for claim in input {
      let claim_split = claim.split(' ').collect::<Vec<&str>>();  //0: Plot #, 1: '@', 2: position, 3: height x width

      // Parse position
      let mut positions_split = claim_split[2].split(',').collect::<Vec<&str>>();
      positions_split[1] = remove_last_char(&positions_split[1]); // clear the colon from the end of the string
      let position = Point {
        x: positions_split[0].parse().unwrap(),
        y: positions_split[1].parse().unwrap(),
      };

      // Parse dimensions
      let dimensions_split = claim_split[3].split('x').map(|n| n.parse::<u32>().unwrap()).collect::<Vec<u32>>();
      println!("Dimensions {dimensions_split:?}");
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

    return claimed_squares
    .into_iter()
    .filter(|pair| pair.1 == true )
    .collect::<Vec<_>>().len();
  }
}