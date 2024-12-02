use crate::{build_run, build_test};
fn part1() -> i64 {
    let [mut l1, mut l2] = get_list();
    l1.sort();
    l2.sort();
    l1.into_iter().zip(l2).map(|(v1, v2)| (v1 - v2).abs()).sum()
}
fn part2() -> i64 {
    let [l1, l2] = get_list();
    let mut numbers_right = vec![0; 100_000];
    for number in l2 {
        numbers_right[number as usize] += 1;
    }
    l1.into_iter().map(|v| v * numbers_right[v as usize]).sum()
}
fn get_list() -> [Vec<i64>; 2] {
    let mut l1 = Vec::new();
    let mut l2 = Vec::new();
    let input = include_str!("input.txt");
    for line in input.trim().lines() {
        let mut splited = line.trim().splitn(2, "   ");
        let num1 = splited.next().unwrap().parse::<i64>().unwrap();
        let num2 = splited.next().unwrap().parse::<i64>().unwrap();
        l1.push(num1);
        l2.push(num2);
    }
    [l1, l2]
}
build_run!(1, part1, part2);
build_test!(part1: 2264607, part2: 19457120);
