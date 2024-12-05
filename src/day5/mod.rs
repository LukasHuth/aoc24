use std::cmp::Ordering;

use crate::{
    build_run, build_test,
    utilities::{CollectIntoResult, IteratorToVec, IteratorTrimmedLines, MapParse as _},
};

mod page_ordering;
use page_ordering::*;

#[inline(always)]
fn get_middle(data: Vec<u32>) -> u32 {
    data[data.len() / 2]
}

fn is_in_right_order(page_ordering: &PageOrdering, update: &[u32]) -> bool {
    update.is_sorted_by(|&a, &b| {
        if a == b {
            true
        } else if page_ordering.should_be_before(a, b) {
            true
        } else {
            false
        }
    })
}

fn part1() -> u32 {
    let (page_ordering, updates) = load_data();
    updates
        .into_iter()
        .filter(|update| is_in_right_order(&page_ordering, update))
        .map(get_middle)
        .sum()
}

fn part2() -> u32 {
    let (page_ordering, updates) = load_data();
    updates
        .into_iter()
        .filter(|update| !is_in_right_order(&page_ordering, update))
        .map(|mut update| update_list(&page_ordering, &mut update))
        .map(get_middle)
        .sum()
}

fn update_list(page_ordering: &PageOrdering, update: &mut [u32]) -> Vec<u32> {
    update.sort_by(|&a, &b| {
        if a == b {
            Ordering::Equal
        } else if page_ordering.should_be_before(a, b) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
    update.to_vec()
}

fn load_data() -> (PageOrdering, Vec<Vec<u32>>) {
    let str = include_str!("input.txt");
    let splited_text = str.trim().splitn(2, "\n\n").to_vec();
    let [page_ordering_str, update_str] = [splited_text[0], splited_text[1]];
    let page_ordering_elements: Vec<PageOrderingElement> = page_ordering_str
        .trimmed_lines()
        .parse()
        .collect_result()
        .unwrap();
    let page_ordering = PageOrdering::new(page_ordering_elements);
    let updates: Vec<Vec<u32>> = update_str
        .trimmed_lines()
        .map(|line| line.trim().split(',').parse().collect_result())
        .collect_result()
        .unwrap();
    (page_ordering, updates)
}

build_run!(5, part1, part2);
build_test!(part1: 6498, part2: 5017);
