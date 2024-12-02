use crate::{build_run, build_test, utilities::CollectIntoResult};
fn part1() -> usize {
    let data: Vec<Report> = load_data();
    data.into_iter()
        .filter(Report::is_safe_zero_tolerance)
        .count()
}

fn part2() -> usize {
    let data: Vec<Report> = load_data();
    data.into_iter()
        .filter(Report::is_safe_one_tolerance)
        .count()
}
fn load_data() -> Vec<Report> {
    let input_str: &str = include_str!("input.txt");
    input_str
        .trim()
        .lines()
        .map(str::parse::<Report>)
        .collect_result()
        .unwrap()
}

build_run!(2, part1, part2);
build_test!(part1: 306, part2: 366);

mod report;
use report::Report;
