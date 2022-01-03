use std::ops::AddAssign;

use ndarray::Array2;

fn parse_data(data: &str) -> Array2<i64> {
    let numbers = data
        .chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect::<Vec<i64>>();

    if numbers.len() != 100 {
        panic!("Invalid input, not 100 digits");
    }

    return Array2::from_shape_vec((10, 10), numbers).expect("Could not create 2d array");
}

fn step(data: &mut Array2<i64>) -> i64 {
    data.add_assign(1);
    let mut did_boom = false;
    let mut nb_booms: i64 = 0;
    loop {
        for i in 0..10 {
            for j in 0..10 {
                let energy = data[(i, j)];
                // If a boom is detected
                if energy > 9 {
                    // Use a large negative number to mark the cell as a boom
                    // The idea is to not have them explode twice in the step
                    data[(i, j)] = -0xDEADBEEF;
                    did_boom = true;
                    nb_booms += 1;

                    // Increment all neirbor cells, including diagonals and itself because why not
                    // Check if they are within the boundaries of the 10x10 grid and not negative
                    let i_min = if i > 0 { i - 1 } else { 0 };
                    let j_min = if j > 0 { j - 1 } else { 0 };
                    let i_max = if i < 9 { i + 1 } else { 9 };
                    let j_max = if j < 9 { j + 1 } else { 9 };
                    for x in i_min..=i_max {
                        for y in j_min..=j_max {
                            data[(x, y)] += 1;
                        }
                    }
                }
            }
        }
        if !did_boom {
            // Time to cleanup the cells that exploded (back to 0)
            for energy in data.iter_mut() {
                if *energy < 0 {
                    *energy = 0;
                }
            }
            break;
        }
        did_boom = false;
    }
    return nb_booms;
}

pub fn day_11_part_1(data: &str) -> i64 {
    let mut data = parse_data(data);
    let mut nb_booms = 0;
    for _i in 0..100 {
        nb_booms += step(&mut data);
    }
    return nb_booms;
}

pub fn day_11_part_2(data: &str) -> i64 {
    let mut data = parse_data(data);
    for step_id in 1..=10000 {
        if step(&mut data) == 100 {
            return step_id;
        }
    }
    panic!("Never all blinked");
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn test_day_11_part_1() {
        assert_eq!(day_11_part_1(EXAMPLE), 1656);
    }

    #[test]
    fn test_day_11_part_2() {
        assert_eq!(day_11_part_2(EXAMPLE), 195);
    }
}
