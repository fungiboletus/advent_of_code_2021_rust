use std::panic;

// Totally unecessary and actually slower for small datasets but fun
use rayon::prelude::*;

pub type DiagnosticRawData<'a> =
    rayon::iter::Map<rayon::str::SplitWhitespace<'a>, fn(&str) -> Vec<i64>>;

pub fn parse_diagnostic(input: &str) -> DiagnosticRawData {
    return input.par_split_whitespace().map(|binary_number| {
        binary_number
            .as_bytes()
            .iter()
            .map(|&x| i64::from(x - 48))
            .collect::<Vec<i64>>()
    });
}

pub struct DiagnosticData<'a> {
    pub data: DiagnosticRawData<'a>,
    pub number_of_lines: usize,
    pub number_of_digits: usize,
    pub half: i64,
}

pub fn compute_sum_on_ones(diagnostic: DiagnosticRawData, number_of_digits: usize) -> Vec<i64> {
    return diagnostic.reduce(
        || vec![0; number_of_digits],
        |acc, x| {
            acc.iter()
                .zip(x.iter())
                .map(|(a, b)| a + b)
                .collect::<Vec<i64>>()
        },
    );
}

pub fn compute_sum_on_ones_2(diagnostic: &Vec<Vec<i64>>, index: usize) -> i64 {
    return diagnostic.par_iter().map(|x| x[index]).sum();
}

pub fn compute_diagnostic_data(data: &str) -> DiagnosticData {
    let diagnostic = parse_diagnostic(data);
    let data = diagnostic.clone();
    let number_of_lines = diagnostic.clone().count();
    let number_of_digits = diagnostic.clone().max_by_key(|x| x.len()).unwrap().len();
    let half = (number_of_lines as f64 / 2.0).ceil() as i64;

    return DiagnosticData {
        data,
        number_of_lines,
        number_of_digits,
        half,
    };
}

#[derive(Debug, PartialEq)]
pub enum CommonMode {
    Common,
    Uncommon,
}

pub fn filter_diagnostic_data_per_common_bit(
    data: &Vec<Vec<i64>>,
    number_of_digits: usize,
    mode: CommonMode,
) -> i64 {
    let mut current_index = 0;
    let mut current_data = data.clone();

    loop {
        if current_index > number_of_digits {
            panic!("We ran out of digits to check");
        }

        let sum_of_ones = compute_sum_on_ones_2(&current_data, current_index);
        let half = (current_data.len() as f64 / 2.0).ceil() as i64;

        let new_data = current_data
            .iter()
            .filter(|digits| {
                let should_be_a_one = (sum_of_ones >= half) == (mode == CommonMode::Common);
                let digit = digits[current_index];
                if digit > 1 {
                    panic!("We should never have a digit greater than 1");
                }
                return (digit == 0 && !should_be_a_one) || (digit == 1 && should_be_a_one);
            })
            .cloned()
            .collect::<Vec<Vec<i64>>>();

        current_data = new_data;
        current_index += 1;

        let len = current_data.len();
        if len == 1 {
            break;
        }
        if len == 0 {
            panic!("no correct digit found");
        }
    }

    return current_data
        .get(0)
        .expect("we should expect at least one element")
        .iter()
        .enumerate()
        .fold(0, |acc, (index, digit)| {
            acc + (digit * (1 << (number_of_digits - 1 - index)))
        });
}

pub fn day_3_part_1(data: &str) -> i64 {
    let diagnostic = compute_diagnostic_data(data);
    let sum_of_ones = compute_sum_on_ones(diagnostic.data, diagnostic.number_of_digits);

    let mut gamma: i64 = 0;
    let mut epsilon: i64 = 0;
    for (index, sum) in sum_of_ones.iter().enumerate() {
        if *sum == diagnostic.half {
            panic!("Undocumented behaviour : we panic");
        }
        if *sum >= diagnostic.half {
            gamma += 1 << (diagnostic.number_of_digits - 1 - index);
        } else {
            epsilon += 1 << (diagnostic.number_of_digits - 1 - index);
        }
    }
    return gamma * epsilon;
}

pub fn day_3_part_2(data: &str) -> i64 {
    let diagnostic = compute_diagnostic_data(data);

    let data = diagnostic.data.collect::<Vec<Vec<i64>>>();

    let oxygen_generator_rating = filter_diagnostic_data_per_common_bit(
        &data,
        diagnostic.number_of_digits,
        CommonMode::Common,
    );
    let co2_scrubber_rating = filter_diagnostic_data_per_common_bit(
        &data,
        diagnostic.number_of_digits,
        CommonMode::Uncommon,
    );
    return oxygen_generator_rating * co2_scrubber_rating;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn test_day_3_part_1() {
        assert_eq!(day_3_part_1(EXAMPLE), 198);
    }

    #[test]
    fn test_day_3_part_2() {
        assert_eq!(day_3_part_2(EXAMPLE), 230);
    }
}
