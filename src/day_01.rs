// use rayon::prelude::*;

pub fn string_to_vec_of_ints(input: &str) -> Vec<i64> {
    return input.split_whitespace()
        .map(|s| s.parse::<i64>().expect("Could not parse number"))
        .collect();
}

fn number_of_increases(numbers: &Vec<i64>) -> i64 {
    let mut number_of_increases = 0;
    for i in 0..numbers.len() - 1 {
        if numbers[i] < numbers[i + 1] {
            number_of_increases += 1;
        }
    }
    return number_of_increases;
}

pub fn day_1_part_1(data: &str) -> i64 {
  return number_of_increases(&string_to_vec_of_ints(data));
}

pub fn day_1_part_2(data: &str) -> i64 {
  let data = string_to_vec_of_ints(data);
  let mut sliding_window : Vec<i64> = vec![0; data.len() - 2];
  for i in 0..data.len() - 2 {
    sliding_window[i] = data[i] + data[i + 1] + data[i + 2];
  }
  
  return number_of_increases(&sliding_window);
}

#[cfg(test)]
mod tests {
  use super::*;

  const EXAMPLE: &str = "199
200
208
210
200
207
240
269
260
263";

  #[test]
  fn test_day_1_part_1() {
    assert_eq!(day_1_part_1(EXAMPLE), 7);
  }
 
  #[test]
  fn test_day_1_part_2() {
    assert_eq!(day_1_part_2(EXAMPLE), 5);
  }
}
