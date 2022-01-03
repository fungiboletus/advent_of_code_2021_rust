use std::str::Chars;

// Return the error score, 0 means no error
fn compute_error_score(instructions: Chars) -> (i64, Option<Vec<char>>) {
    let mut stack: Vec<char> = Vec::new();
    for instruction in instructions {
        match instruction {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            '<' => stack.push('>'),
            ')' | ']' | '}' | '>' => {
                let last_instruction = stack.pop();
                if last_instruction.is_none() || last_instruction.unwrap() != instruction {
                    match instruction {
                        ')' => return (3, None),
                        ']' => return (57, None),
                        '}' => return (1197, None),
                        '>' => return (25137, None),
                        _ => panic!("That shouldn't happen but anyway"),
                    }
                }
            }
            _ => panic!("Invalid instruction"),
        }
    }
    return (0, Some(stack));
}

fn compute_autocomplete_score(stack: Vec<char>) -> i64 {
    let mut score: i64 = 0;
    for instruction in stack.iter().rev() {
        match instruction {
            ')' => score = score * 5 + 1,
            ']' => score = score * 5 + 2,
            '}' => score = score * 5 + 3,
            '>' => score = score * 5 + 4,
            _ => panic!("Invalid instruction in the stack"),
        }
    }
    return score;
}

pub fn day_10_part_1(data: &str) -> i64 {
    return data
        .lines()
        .map(|line| compute_error_score(line.chars()).0)
        .sum();
}

pub fn day_10_part_2(data: &str) -> i64 {
    let mut scores: Vec<i64> = data
        .lines()
        .map(|line| {
            let (_, stack) = compute_error_score(line.chars());
            if let Some(stack) = stack {
                return compute_autocomplete_score(stack);
            }
            return 0;
        })
        .filter(|score| *score != 0)
        .collect();
    scores.sort_unstable();
    // get the middle score element
    return scores[scores.len() / 2];
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_day_10_part_1() {
        assert_eq!(day_10_part_1(EXAMPLE), 26397);
    }

    #[test]
    fn test_day_10_part_2() {
        assert_eq!(day_10_part_2(EXAMPLE), 288957);
    }

    #[test]
    fn test_compute_autocomplete_score() {
        let (_, stack) = compute_error_score("<{([{{}}[<[[[<>{}]]]>[]]".chars());
        let stack = stack.unwrap();
        assert_eq!(stack, vec!['>', '}', ')', ']']);
        assert_eq!(compute_autocomplete_score(stack), 294);
    }
}
