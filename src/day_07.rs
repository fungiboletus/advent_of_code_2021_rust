use ndarray_stats::Quantile1dExt;

fn parse_input_numbers(input: &str) -> Vec<i64> {
    input
        .split(",")
        .map(|s| s.parse::<i64>().expect("Could not parse number"))
        .collect()
}

pub fn day_7_part_1(data: &str) -> i64 {
    let crabs = parse_input_numbers(data);
    let mut nd_crabs = ndarray::Array::from_vec(crabs.clone());
    let median = nd_crabs
        .quantile_mut(
            noisy_float::types::n64(0.5),
            &ndarray_stats::interpolate::Nearest,
        )
        .expect("Could not get median");

    return crabs.iter().map(|crab| (crab - median).abs()).sum::<i64>();
}

// I looked at the solution because life is short
fn fuel(d: i64) -> i64 {
    return d * (d + 1) / 2;
}

pub fn day_7_part_2(data: &str) -> i64 {
    let crabs = parse_input_numbers(data);
    let nd_crabs = ndarray::Array::from_vec(crabs.clone());
    let mean = nd_crabs.mean().expect("Could not calculate mean");
    let a = crabs
        .iter()
        .map(|crab| fuel((crab - mean).abs()))
        .sum::<i64>();
    // Compute with a mean 1 higher too, and use the minimum between the two
    // results
    let b = crabs
        .iter()
        .map(|crab| fuel((crab - mean - 1).abs()))
        .sum::<i64>();
    return a.min(b);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_day_7_part_1() {
        assert_eq!(day_7_part_1(EXAMPLE), 37);
    }

    #[test]
    fn test_day_7_part_2() {
        assert_eq!(day_7_part_2(EXAMPLE), 168);
    }
}
