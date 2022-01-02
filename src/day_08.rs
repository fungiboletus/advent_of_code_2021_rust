#[derive(Debug)]
struct DisplayObservations {
    patterns: Vec<String>,
    outputs: Vec<String>,
}

#[derive(Debug)]
struct Display {
    digits: [String; 10],
}

fn parse_data(data: &str) -> Vec<DisplayObservations> {
    // read data line by line
    // 10 patterns strings, a |, and 4 output strings
    return data
        .lines()
        .map(|line| {
            let mut parts = line.split(" | ");
            let patterns: Vec<String> = parts
                .next()
                .expect("Could not get patterns")
                .split(" ")
                .map(|s| s.trim().to_string())
                .collect();
            let outputs: Vec<String> = parts
                .next()
                .expect("Could not get outputs")
                .split(" ")
                .map(|s| s.trim().to_string())
                .collect();
            if (patterns.len() != 10) || (outputs.len() != 4) {
                panic!("Invalid data line: {}", line);
            }
            // check if patterns contains only letters from a to g
            for pattern in &patterns {
                for c in pattern.chars() {
                    if !(c == 'a'
                        || c == 'b'
                        || c == 'c'
                        || c == 'd'
                        || c == 'e'
                        || c == 'f'
                        || c == 'g')
                    {
                        panic!("Invalid pattern: {}", pattern);
                    }
                }
            }
            // same for outputs
            for output in outputs.iter() {
                for c in output.chars() {
                    if !(c == 'a'
                        || c == 'b'
                        || c == 'c'
                        || c == 'd'
                        || c == 'e'
                        || c == 'f'
                        || c == 'g')
                    {
                        panic!("Invalid output: {}", c);
                    }
                }
            }
            return DisplayObservations { patterns, outputs };
        })
        .collect();
}

pub fn day_8_part_1(data: &str) -> i64 {
    let observations = parse_data(data);
    return observations
        .iter()
        .map(|observation| {
            let sum: i64 = observation
                .outputs
                .iter()
                .map(|output| match output.len() {
                    2 => 1, // 1
                    3 => 1, // 7,
                    4 => 1, // 4,
                    5 => 0,
                    6 => 0,
                    7 => 1, // 8,
                    _ => panic!("Invalid output length: {}", output.len()),
                })
                .sum();
            return sum;
        })
        .sum();
}

// Takes a list of 10 observations for each digit in an unknown random order
// and returns the corresponding display definition.
fn solve_display_problem(observation: &DisplayObservations) -> Display {
    // This could probably be implemented in a shorter, faster, and better way
    // but I like to do it step by step in the way I thought about it first.

    /*
      aaaa
     b    c
     b    c
      dddd
     e    f
     e    f
      gggg

    find 2 length : set the right segments a and f with both options (digit 1)
    find 3 length : what is not on the right is the top segment a (digit 7)
    find 4 length : what is not on the right is segments b and d with both options (digit 4)
    find 5 length with the letter of segment a and the two left unused letters (digit 2)
          => c segment is inside it, set c and f segments accordingly
          => d segment is inside it, set b and d segments accordingly
    find one of the remaining 5 length (it's either digit 3 or digit 5)
          => the remaining segment letter is inside it for segment g
            =>  set the remaining segments b and g accordingly
    */

    // find the only pattern with string length of 2
    let pattern_2_length = observation
        .patterns
        .iter()
        .find(|pattern| pattern.len() == 2)
        .expect("Could not find pattern with 2 length");

    let c_or_f_1 = pattern_2_length
        .chars()
        .nth(0)
        .expect("Could not get first char");
    let c_or_f_2 = pattern_2_length
        .chars()
        .nth(1)
        .expect("Could not get second char");

    // find the only pattern with string length of 3
    let pattern_3_length = observation
        .patterns
        .iter()
        .find(|pattern| pattern.len() == 3)
        .expect("Could not find pattern with 3 length");

    let segment_a = pattern_3_length
        .chars()
        .find(|possibility| *possibility != c_or_f_1 && *possibility != c_or_f_2)
        .expect("Could not find segment a");

    // find the only pattern with string length of 4
    let pattern_4_length = observation
        .patterns
        .iter()
        .find(|pattern| pattern.len() == 4)
        .expect("Could not find pattern with 4 length");

    let b_or_d_1 = pattern_4_length
        .chars()
        .find(|possibility| *possibility != c_or_f_1 && *possibility != c_or_f_2)
        .expect("Could not get first char");

    let b_or_d_2 = pattern_4_length
        .chars()
        .find(|possibility| {
            *possibility != c_or_f_1 && *possibility != c_or_f_2 && *possibility != b_or_d_1
        })
        .expect("Could not get second char");

    let all_letters = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];
    let two_left_unused_letters = all_letters.iter().filter(|letter| {
        *letter != &c_or_f_1
            && *letter != &c_or_f_2
            && *letter != &b_or_d_1
            && *letter != &b_or_d_2
            && *letter != &segment_a
    });

    let left_unused_letter_1 = *two_left_unused_letters
        .clone()
        .nth(0)
        .expect("Could not get first unused letter");
    let left_unused_letter_2 = *two_left_unused_letters
        .clone()
        .nth(1)
        .expect("Could not get second unused letter");

    // find pattern of 5 length with the letter of segment a and the two left unused letters
    let pattern_5_length_1 = observation
        .patterns
        .iter()
        .find(|pattern| {
            pattern.len() == 5
                && pattern.contains(segment_a)
                && pattern.contains(left_unused_letter_1)
                && pattern.contains(left_unused_letter_2)
        })
        .expect("Could not find pattern with 5 length with segment a and two left unused letters");

    let mut segment_c: char = '\0';
    let mut segment_f: char = '\0';
    let mut segment_b: char = '\0';
    let mut segment_d: char = '\0';
    for letter in pattern_5_length_1.chars() {
        if letter == c_or_f_1 {
            segment_c = c_or_f_1;
            segment_f = c_or_f_2;
        } else if letter == c_or_f_2 {
            segment_c = c_or_f_2;
            segment_f = c_or_f_1;
        } else if letter == b_or_d_1 {
            segment_d = b_or_d_1;
            segment_b = b_or_d_2;
        } else if letter == b_or_d_2 {
            segment_d = b_or_d_2;
            segment_b = b_or_d_1;
        }
    }

    // find one of the remaining 5 length
    let pattern_5_length_2 = observation
        .patterns
        .iter()
        .find(|pattern| pattern.len() == 5 && *pattern != pattern_5_length_1)
        .expect("Could not find reminding pattern with 5 length");

    let mut segment_e: char = '\0';
    let mut segment_g: char = '\0';
    for letter in pattern_5_length_2.chars() {
        if letter == left_unused_letter_1 {
            segment_g = left_unused_letter_1;
            segment_e = left_unused_letter_2;
        } else if letter == left_unused_letter_2 {
            segment_g = left_unused_letter_2;
            segment_e = left_unused_letter_1;
        }
    }

    // check that any segment is not zero
    assert!(segment_a != '\0', "Segment a is 0");
    assert!(segment_b != '\0', "Segment b is 0");
    assert!(segment_c != '\0', "Segment c is 0");
    assert!(segment_d != '\0', "Segment d is 0");
    assert!(segment_e != '\0', "Segment e is 0");
    assert!(segment_f != '\0', "Segment f is 0");
    assert!(segment_g != '\0', "Segment g is 0");

    /*
     0:      1:      2:      3:      4:
      aaaa    ....    aaaa    aaaa    ....
     b    c  .    c  .    c  .    c  b    c
     b    c  .    c  .    c  .    c  b    c
      ....    ....    dddd    dddd    dddd
     e    f  .    f  e    .  .    f  .    f
     e    f  .    f  e    .  .    f  .    f
      gggg    ....    gggg    gggg    ....

     5:      6:      7:      8:      9:
      aaaa    aaaa    aaaa    aaaa    aaaa
     b    .  b    .  .    c  b    c  b    c
     b    .  b    .  .    c  b    c  b    c
      dddd    dddd    ....    dddd    dddd
     .    f  e    f  .    f  e    f  .    f
     .    f  e    f  .    f  e    f  .    f
      gggg    gggg    ....    gggg    gggg
    */
    let mut digits: [String; 10] = Default::default();

    let mut digit_0 = vec![
        segment_a, segment_b, segment_c, segment_e, segment_f, segment_g,
    ];
    digit_0.sort();
    digits[0] = digit_0.iter().collect();

    let mut digit_1 = vec![segment_c, segment_f];
    digit_1.sort();
    digits[1] = digit_1.iter().collect();

    let mut digit_2 = vec![segment_a, segment_c, segment_d, segment_e, segment_g];
    digit_2.sort();
    digits[2] = digit_2.iter().collect();

    let mut digit_3 = vec![segment_a, segment_c, segment_d, segment_f, segment_g];
    digit_3.sort();
    digits[3] = digit_3.iter().collect();

    let mut digit_4 = vec![segment_b, segment_c, segment_d, segment_f];
    digit_4.sort();
    digits[4] = digit_4.iter().collect();

    let mut digit_5 = vec![segment_a, segment_b, segment_d, segment_f, segment_g];
    digit_5.sort();
    digits[5] = digit_5.iter().collect();

    let mut digit_6 = vec![
        segment_a, segment_b, segment_d, segment_e, segment_f, segment_g,
    ];
    digit_6.sort();
    digits[6] = digit_6.iter().collect();

    let mut digit_7 = vec![segment_a, segment_c, segment_f];
    digit_7.sort();
    digits[7] = digit_7.iter().collect();

    let mut digit_8 = vec![
        segment_a, segment_b, segment_c, segment_d, segment_e, segment_f, segment_g,
    ];
    digit_8.sort();
    digits[8] = digit_8.iter().collect();

    let mut digit_9 = vec![
        segment_a, segment_b, segment_c, segment_d, segment_f, segment_g,
    ];
    digit_9.sort();
    digits[9] = digit_9.iter().collect();

    return Display { digits };
}

fn convert_output_to_digit(output: &str, display: &Display) -> i64 {
    // sort output &str chars in ascii order
    let mut output_chars: Vec<char> = output.chars().collect();
    output_chars.sort();
    // convert sorted output &str chars to string
    let output_string: String = output_chars.into_iter().collect();

    // find digit in display
    for (index, digit) in display.digits.iter().enumerate() {
        if output_string == *digit {
            return index as i64;
        }
    }
    return 0;
}

pub fn day_8_part_2(data: &str) -> i64 {
    let observations = parse_data(data);
    return observations
        .iter()
        .map(|observation| {
            let display = solve_display_problem(observation);
            let first_digit = convert_output_to_digit(
                observation.outputs.get(0).expect("Could not get output"),
                &display,
            );
            let second_digit = convert_output_to_digit(
                observation.outputs.get(1).expect("Could not get output"),
                &display,
            );
            let third_digit = convert_output_to_digit(
                observation.outputs.get(2).expect("Could not get output"),
                &display,
            );
            let fourth_digit = convert_output_to_digit(
                observation.outputs.get(3).expect("Could not get output"),
                &display,
            );
            return first_digit * 1000 + second_digit * 100 + third_digit * 10 + fourth_digit;
        })
        .sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
    const EXAMPLE_2: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test_day_8_part_1() {
        assert_eq!(day_8_part_1(EXAMPLE_1), 0);
        assert_eq!(day_8_part_1(EXAMPLE_2), 26);
    }

    #[test]
    fn test_day_8_part_2() {
        assert_eq!(day_8_part_2(EXAMPLE_1), 5353);
        assert_eq!(day_8_part_2(EXAMPLE_2), 61229);
    }
}
