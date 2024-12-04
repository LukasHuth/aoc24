use std::ops::Range;

use crate::{
    build_run, build_test,
    utilities::{IteratorHasNElementsOf, IteratorToVec as _, IteratorTrimmedLines, StringCharVec},
};

const SIZE: usize = 140;
const SEARCH_STR: [(char, usize); 4] = [('X', 0), ('S', 3), ('A', 2), ('M', 1)];
const REVERSE_SEARCH_STR: [(char, usize); 4] = [('X', 3), ('S', 0), ('A', 1), ('M', 2)];

fn part1() -> usize {
    let data = load_data();
    find_vertical(&data) + find_horizontal(&data) + find_diagonal(&data)
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
    find_word(
        data,
        |data: &[[char; 140]; 140], arr_index: usize, offset: usize, i: usize, c: char| -> bool {
            data[arr_index][offset + i] == c
        },
        0..SIZE,
        0..SIZE - 3,
    )
}
fn find_horizontal(data: &[[char; SIZE]; SIZE]) -> usize {
    find_word(
        data,
        |data: &[[char; 140]; 140], arr_index: usize, offset: usize, i: usize, c: char| -> bool {
            data[arr_index + i][offset] == c
        },
        0..SIZE - 3,
        0..SIZE,
    )
}
fn find_diagonal(data: &[[char; SIZE]; SIZE]) -> usize {
    find_diagonal_right(data) + find_diagonal_left(data)
}
fn find_diagonal_right(data: &[[char; SIZE]; SIZE]) -> usize {
    find_word(
        data,
        |data: &[[char; 140]; 140], arr_index: usize, offset: usize, i: usize, c: char| -> bool {
            data[arr_index + i][offset + i] == c
        },
        0..SIZE - 3,
        0..SIZE - 3,
    )
}
fn find_diagonal_left(data: &[[char; SIZE]; SIZE]) -> usize {
    find_word(
        data,
        |data: &[[char; 140]; 140], arr_index: usize, offset: usize, i: usize, c: char| -> bool {
            data[arr_index + i][offset - i] == c
        },
        0..SIZE - 3,
        3..SIZE,
    )
}
type IdentificationFunction = fn(data: &[[char; SIZE]; SIZE], usize, usize, usize, char) -> bool;
#[inline(always)]
fn find_word(
    data: &[[char; SIZE]; SIZE],
    identification: IdentificationFunction,
    range_1: Range<usize>,
    range_2: Range<usize>,
) -> usize {
    let mut result = 0;
    let strings = [SEARCH_STR, REVERSE_SEARCH_STR];
    for arr_index in range_1 {
        'offset_loop: for offset in range_2.clone() {
            for str in strings {
                if str
                    .iter()
                    .all(|&(c, i)| identification(data, arr_index, offset, i, c))
                {
                    result += 1;
                    continue 'offset_loop;
                }
            }
        }
    }
    result
}

fn find_cross_mas(data: &[[char; SIZE]; SIZE]) -> usize {
    (0..SIZE - 2)
        .flat_map(|i| (0..SIZE - 2).map(move |j| (i + 1, j + 1)))
        .filter(|&(x, y)| data[x][y] == 'A')
        .filter(|&(x, y)| {
            let bottom_left = data[x + 1][y - 1];
            let top_right = data[x - 1][y + 1];
            let top_left = data[x - 1][y - 1];
            let bottom_right = data[x + 1][y + 1];
            let corners = [bottom_left, bottom_right, top_left, top_right];
            top_left != bottom_right
                && bottom_left != top_right
                && corners.into_iter().has_n_elements_of(2, 'S')
                && corners.into_iter().has_n_elements_of(2, 'M')
        })
        .count()
}
build_run!(4, part1, part2);
build_test!(part1: 2462, part2: 1877);
