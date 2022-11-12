pub mod solutions {
  use regex::Regex;
  use chrono::{Datelike, Timelike, Utc, NaiveDateTime};

  #[derive(Debug)]
  struct GuardData {
    timestamp: String,
    activity: String,
  }

  // Day 4 Part 1 -- https://adventofcode.com/2018/day/4
  // Find which guard sleeps the most minutes + the minute they sleep the most
  /// ## Return
  /// * `(u32, u32)` - (Guard ID Number, Guard Most-Slept-Minute)
  pub fn find_sleepiest_guard(input_vec: &Vec<&str>) -> (u32, u32) {
    let sorted_guard_data: Vec<GuardData> = parse_input(input_vec);



    return (0, 0);
  }



  fn parse_input(input_vec: &Vec<&str>) -> Vec<GuardData> {
    let mut guard_entries: Vec<GuardData> = Vec::new();
    for entry in input_vec {
      // Remove square brackets
      let cleaned = entry.replace(&['[', ']'][..], "");

      // Push entries to unsorted vec
      let (timestamp, activity) = cleaned.split_at(16);

      // Parse activity
      let mut activity_str = activity.to_string();
      activity_str.remove(0);

      // Parse minute-based timestamp
      let mut minute_based_timestamp = 0;
      let unix_timestamp = NaiveDateTime::parse_from_str(timestamp, "%Y-%m-%d %H:%M");

      println!("{unix_timestamp:?}");

      guard_entries.push(GuardData { 
          timestamp: String::from(timestamp), 
          activity: activity_str,
        }
      )
    }

    println!("\nBefore: ");
    for entry in &guard_entries { println!("{entry:?}"); }

    guard_entries.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

    println!("\nAfter: ");
    for entry in &guard_entries { println!("{entry:?}"); }

    // Return sorted array
    return guard_entries;
  }
}