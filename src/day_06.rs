fn parse_input_numbers(input: &str) -> Vec<i8> {
    input
        .split(",")
        .map(|s| s.parse::<i8>().expect("Could not parse number"))
        .collect()
}

fn _slow_fishes_simulation(fishes: Vec<i8>, generation: u64) -> i64 {
    let mut fishes = fishes;
    for _i in 0..generation {
        println!("Computing generation {}", _i + 1);
        let mut new_fishes: Vec<i8> = vec![];
        for i in 0..fishes.len() {
            let fish = fishes[i];
            if fish == 0 {
                fishes[i] = 6;
                new_fishes.push(8);
            } else {
                fishes[i] -= 1;
            }
        }
        fishes.append(&mut new_fishes);
    }
    return fishes.len() as i64;
}

fn faster_fishes_simulation(fishes: Vec<i8>, generation: u64) -> i64 {
    let mut generation_0: u64 = 0;
    let mut generation_1: u64 = 0;
    let mut generation_2: u64 = 0;
    let mut generation_3: u64 = 0;
    let mut generation_4: u64 = 0;
    let mut generation_5: u64 = 0;
    let mut generation_6: u64 = 0;
    let mut generation_7: u64 = 0;
    let mut generation_8: u64 = 0;

    for fish in fishes {
        match fish {
            0 => generation_0 += 1,
            1 => generation_1 += 1,
            2 => generation_2 += 1,
            3 => generation_3 += 1,
            4 => generation_4 += 1,
            5 => generation_5 += 1,
            6 => generation_6 += 1,
            _ => panic!("Invalid fish {}", fish),
        }
    }

    for _i in 0..generation {
        let old_generation_0 = generation_0;
        generation_0 = generation_1;
        generation_1 = generation_2;
        generation_2 = generation_3;
        generation_3 = generation_4;
        generation_4 = generation_5;
        generation_5 = generation_6;
        generation_6 = generation_7 + old_generation_0;
        generation_7 = generation_8;
        generation_8 = old_generation_0;
    }

    return i64::try_from(
        generation_0
            + generation_1
            + generation_2
            + generation_3
            + generation_4
            + generation_5
            + generation_6
            + generation_7
            + generation_8,
    )
    .expect("Could not convert to i64");
}

pub fn day_6_part_1(data: &str) -> i64 {
    let fishes = parse_input_numbers(data);
    //return slow_fishes_simulation(fishes, 80);
    return faster_fishes_simulation(fishes, 80);
}

pub fn day_6_part_2(data: &str) -> i64 {
    let fishes = parse_input_numbers(data);
    return faster_fishes_simulation(fishes, 256);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "3,4,3,1,2";

    #[test]
    fn test_day_6_part_1() {
        assert_eq!(day_6_part_1(EXAMPLE), 5934);
    }

    #[test]
    fn test_day_6_part_2() {
        assert_eq!(day_6_part_2(EXAMPLE), 26984457539);
    }
}
