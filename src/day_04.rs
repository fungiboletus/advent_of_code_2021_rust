use std::panic;

#[derive(Debug)]
struct BingoBoard {
    data: [[i8; 5]; 5],

    lines: [i8; 5],
    columns: [i8; 5],

    lines_index: [usize; i8::MAX as usize],
    columns_index: [usize; i8::MAX as usize],

    draw_sum: i64,

    has_won: bool,
}

impl Default for BingoBoard {
    fn default() -> Self {
        BingoBoard {
            data: [[0; 5]; 5],

            lines: [0; 5],
            columns: [0; 5],

            lines_index: [0xBADCAFE; i8::MAX as usize],
            columns_index: [0xBADCAFE; i8::MAX as usize],

            draw_sum: 0,

            has_won: false,
        }
    }
}

fn day_4_core_algorithm(data: &str, part_2_mode: bool) -> i64 {
    let mut lines = data.lines();

    let first_line = lines.next().expect("Could not read first line");

    // Split the first line digits with the comma and parse them as i64
    let first_line_digits: Vec<i8> = first_line
        .split(",")
        .map(|x| x.parse::<i8>().expect("not a small number"))
        .collect();

    let mut boards: Vec<BingoBoard> = vec![];

    loop {
        let blank_line = lines.next();
        if blank_line.is_none() {
            break;
        }

        let mut board = BingoBoard::default();

        // do 5 times
        for i in 0..5 {
            let line = lines.next().expect("Could not read line");

            // Split the line digits with the comma and parse them as i64
            let line_digits: Vec<i8> = line
                .split_whitespace()
                .map(|x| x.parse::<i8>().expect("not a small number"))
                .collect();

            board.data[i] = line_digits
                .try_into()
                .expect("Line has wrong number of digits");
        }

        // build board index
        for line_index in 0..5 {
            for column_index in 0..5 {
                let value = board.data[line_index][column_index] as usize;
                board.lines_index[value] = line_index;
                board.columns_index[value] = column_index;
            }
        }

        boards.push(board);
    }

    let number_of_boards = boards.len();
    let mut number_of_winning_boards = 0;

    for draw_number in first_line_digits {
        for board in &mut boards {
            if part_2_mode && board.has_won {
                continue;
            }
            let index = draw_number as usize;
            let line_index = board.lines_index[index];
            if line_index == 0xBADCAFE {
                continue;
            }
            board.draw_sum += draw_number as i64;

            board.lines[line_index] += 1;
            let column_index = board.columns_index[index];
            board.columns[column_index] += 1;

            if board.lines[line_index] == 5 || board.columns[column_index] == 5 {
                let total_sum = board
                    .data
                    .iter()
                    .map(|x| {
                        x.iter()
                            .map(|n| i64::try_from(*n).expect("ah"))
                            .sum::<i64>()
                    })
                    .sum::<i64>();
                if part_2_mode {
                    board.has_won = true;
                    number_of_winning_boards += 1;
                    if number_of_winning_boards == number_of_boards {
                        return (total_sum - board.draw_sum) * draw_number as i64;
                    }
                } else {
                    return (total_sum - board.draw_sum) * draw_number as i64;
                }
            }
        }
    }
    panic!("No solution found");
}

pub fn day_4_part_1(data: &str) -> i64 {
    return day_4_core_algorithm(data, false);
}

pub fn day_4_part_2(data: &str) -> i64 {
    return day_4_core_algorithm(data, true);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn test_day_4_part_1() {
        assert_eq!(day_4_part_1(EXAMPLE), 4512);
    }

    #[test]
    fn test_day_4_part_2() {
        assert_eq!(day_4_part_2(EXAMPLE), 1924);
    }
}
