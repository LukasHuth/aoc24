use crate::{
    build_run, build_test,
    utilities::{IteratorToVec as _, IteratorTrimmedLines, StringCharVec},
};

// const SIZE: usize = 140;
const SIZE: usize = 140;
const SEARCH_STR: [(char, usize); 4] = [('X', 0), ('S', 3), ('A', 2), ('M', 1)];
const REVERSE_SEARCH_STR: [(char, usize); 4] = [('X', 3), ('S', 0), ('A', 1), ('M', 2)];
//const REVERSE_SEARCH_STR: &str = "SAMX";

fn part1() -> usize {
    let data = load_data();
    find_vertical(&data)
        + find_horizontal(&data)
        + find_diagonal_right(&data)
        + find_diagonal_left(&data)
}

fn part2() -> usize {
    let data = load_data();
    find_cross_mas(&data)
}

fn load_data() -> [[char; SIZE]; SIZE] {
    let data = include_str!("input.txt");
    data.trimmed_lines()
        .map(|line| line.char_vec().try_into().unwrap())
        .to_vec()
        .try_into()
        .unwrap()
}
fn find_vertical(data: &[[char; SIZE]; SIZE]) -> usize {
    let mut result = 0;
    let strings = [SEARCH_STR, REVERSE_SEARCH_STR];
    for arr_index in 0..SIZE {
        'offset_loop: for offset in 0..SIZE - (SEARCH_STR.len() - 1) {
            'str_loop: for str in strings {
                for (c, i) in str {
                    if data[arr_index][offset + i] != c {
                        continue 'str_loop;
                    };
                }
                result += 1;
                continue 'offset_loop;
            }
        }
    }
    result
}
fn find_horizontal(data: &[[char; SIZE]; SIZE]) -> usize {
    let mut result = 0;
    let strings = [SEARCH_STR, REVERSE_SEARCH_STR];
    for arr_index in 0..SIZE - (SEARCH_STR.len() - 1) {
        'offset_loop: for offset in 0..SIZE {
            'str_loop: for str in strings {
                for (c, i) in str {
                    if data[arr_index + i][offset] != c {
                        continue 'str_loop;
                    };
                }
                result += 1;
                continue 'offset_loop;
            }
        }
    }
    result
}
fn find_diagonal_right(data: &[[char; SIZE]; SIZE]) -> usize {
    let mut result = 0;
    let strings = [SEARCH_STR, REVERSE_SEARCH_STR];
    for arr_index in 0..SIZE - (SEARCH_STR.len() - 1) {
        'offset_loop: for offset in 0..SIZE - (SEARCH_STR.len() - 1) {
            'str_loop: for str in strings {
                for (c, i) in str {
                    if data[arr_index + i][offset + i] != c {
                        continue 'str_loop;
                    };
                }
                result += 1;
                continue 'offset_loop;
            }
        }
    }
    result
}

fn find_diagonal_left(data: &[[char; SIZE]; SIZE]) -> usize {
    let mut result = 0;
    for arr_index in 0..SIZE - (SEARCH_STR.len() - 1) {
        'offset_loop: for offset in SEARCH_STR.len() - 1..SIZE {
            'str_loop: for str in [SEARCH_STR, REVERSE_SEARCH_STR] {
                for (c, i) in str {
                    if data[arr_index + i][offset - i] != c {
                        continue 'str_loop;
                    };
                }
                result += 1;
                continue 'offset_loop;
            }
        }
    }
    result
}

fn find_cross_mas(data: &[[char; SIZE]; SIZE]) -> usize {
    (0..SIZE - 2)
        .flat_map(|i| (0..SIZE - 2).map(move |j| (i + 1, j + 1)))
        .filter(|&(x, y)| {
            data[x][y] == 'A'
                && data[x - 1][y - 1] != data[x + 1][y + 1]
                && data[x + 1][y - 1] != data[x - 1][y + 1]
        })
        .filter(|&(x, y)| {
            let mut letters = [0; 4];
            letters[letter_to_index(data[x - 1][y - 1])] += 1;
            letters[letter_to_index(data[x - 1][y + 1])] += 1;
            letters[letter_to_index(data[x + 1][y - 1])] += 1;
            letters[letter_to_index(data[x + 1][y + 1])] += 1;
            letters[0] == 2 && letters[1] == 2
        })
        .count()
}
fn letter_to_index(c: char) -> usize {
    match c {
        'M' => 0,
        'S' => 1,
        'X' => 2,
        'A' => 3,
        _ => panic!(),
    }
}
build_run!(4, part1, part2);
build_test!(part1: 2462, part2: 1877);
