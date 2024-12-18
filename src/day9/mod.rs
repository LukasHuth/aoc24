use crate::{
    build_run, build_test,
    utilities::{IncrementAfter, IsEven, MoveElement},
};

fn part1() -> u128 {
    let files = load_files();
    let compressed_files = fill_gaps(files);
    checksum_files(&compressed_files)
}

fn part2() -> u128 {
    let mut files = load_disk_space();
    print_files(&files);
    let mut i = files.len() - 1;
    let mut c = 0;
    while i != 0 {
        let file = files[i];
        if file.is_free_space() {
            i -= 1;
            continue;
        }
        if let Some(possible_pos) = first_possible_space(&files, &file, i) {
            if false && files[i - 1].is_free_space() {
                // println!("before:");
                // print_files(&files);
                (&mut files[i - 1]).increment_length(file.length());
                files.move_element(i, possible_pos);
            } else {
                // println!("before:");
                // print_files(&files);
                c += 1;
                files[i] = DiskSpace::Free {
                    length: file.length(),
                };
                files.insert(possible_pos, file);
                i += 1;
            }
            if let DiskSpace::Free { length } = files.get_mut(possible_pos + 1).unwrap() {
                *length -= file.length();
            }
            //println!("after:");
            //print_files(&files);
        } else {
            i -= 1;
        }
        while files.last().map(DiskSpace::is_free_space).unwrap_or(false) {
            files.pop();
            i -= 1;
        }
    }
    println!("c: {c}");
    checksum(&files)
}

fn first_possible_space(files: &Vec<DiskSpace>, file: &DiskSpace, current: usize) -> Option<usize> {
    if file.is_free_space() {
        return None;
    }
    let mut counter = 0;
    for free_space in files {
        counter += 1;
        if counter >= current {
            break;
        }
        if !free_space.is_free_space() {
            continue;
        }
        if free_space.length() >= file.length() {
            return Some(counter - 1);
        }
    }
    None
}

fn checksum_files(files: &Vec<File>) -> u128 {
    let mut result = 0;
    for file in files {
        for counter in 0..file.length as u128 {
            result += (file.start as u128 + counter) * (file.id as u128);
        }
    }
    result
}

fn checksum(files: &Vec<DiskSpace>) -> u128 {
    let mut result = 0;
    let mut from_start = 0;
    for file in files {
        match file {
            DiskSpace::Free { length } => from_start += *length as u128,
            DiskSpace::File { id, length } => {
                for counter in 0..*length as u128 {
                    let value = (from_start + counter) * (*id as u128);
                    result += value;
                }
                from_start += file.length() as u128;
            }
        }
    }
    result
}

fn fill_gaps(mut files: Vec<File>) -> Vec<File> {
    let mut compressed_files = Vec::new();
    let mut i = 0;
    let mut file_counter = 0;
    while let Some(file) = files.get(file_counter) {
        if i < file.start {
            if let Some(last_file) = files.last() {
                let length = u32::min(file.start - i, last_file.length);
                compressed_files.push(File::new(last_file.id, i, length));
                i += length;
                if length == last_file.length {
                    files.pop();
                } else {
                    files.last_mut().unwrap().length -= length;
                }
            }
            continue;
        }
        compressed_files.push(*file);
        i = file.start + file.length;
        file_counter += 1;
    }
    compressed_files
}

#[derive(Clone, Copy)]
struct File {
    id: usize,
    start: u32,
    length: u32,
}
impl File {
    fn new(id: usize, start: u32, length: u32) -> Self {
        Self { id, start, length }
    }
}

#[derive(Clone, Copy, Debug)]
enum DiskSpace {
    File { id: usize, length: u32 },
    Free { length: u32 },
}
impl DiskSpace {
    fn length(&self) -> u32 {
        match self {
            Self::File { length, .. } | Self::Free { length } => *length,
        }
    }
    fn is_free_space(&self) -> bool {
        match self {
            Self::Free { .. } => true,
            _ => false,
        }
    }
    fn increment_length(&mut self, arg: u32) {
        match self {
            Self::File { length, .. } | Self::Free { length } => *length += arg,
        }
    }
}

const STR: &str = include_str!("input.txt");
fn load_files() -> Vec<File> {
    let mut numbers = STR.trim().chars().map(process_char).map(Option::unwrap);
    let mut i = 0;
    let mut files = Vec::new();
    while let Some(file_length) = numbers.next() {
        files.push(File::new(files.len(), i, file_length));
        i += file_length;
        if let Some(space_length) = numbers.next() {
            i += space_length;
        }
    }
    files
}

fn load_disk_space() -> Vec<DiskSpace> {
    let mut file_counter = 0;
    STR.trim()
        .chars()
        .map(process_char)
        .map(Option::unwrap)
        .enumerate()
        .map(|(i, length)| {
            if i.is_even() {
                DiskSpace::File {
                    id: file_counter.increment_after(),
                    length,
                }
            } else {
                DiskSpace::Free { length }
            }
        })
        .collect()
}
fn print_files(files: &Vec<DiskSpace>) {
    let mut buffer = String::new();
    for space in files {
        match space {
            DiskSpace::File { id, length } => {
                for _ in 0..*length {
                    buffer.push_str(&format!("{}", id));
                }
            }
            DiskSpace::Free { length } => {
                for _ in 0..*length {
                    buffer.push('.');
                }
            }
        }
    }
    println!("{buffer}");
}

#[inline(always)]
fn process_char(c: char) -> Option<u32> {
    c.to_digit(10)
}

build_run!(9, part1, part2);
build_test!(part1: 6398608069280);
