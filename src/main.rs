#![feature(iter_map_windows)]
#![allow(internal_features)]
#![feature(nonzero_internals)]
use std::time::Duration;

mod macros;
mod utilities;
trait RunDay {
    fn run() -> Duration;
}
build_mods!(1, 2, 3, 4, 5, 6);
fn main() {
    if std::env::args().count() <= 1 {
        return;
    }
    let x = std::env::args()
        .nth(1)
        .map(|str| str.parse::<u8>().ok())
        .flatten()
        .unwrap();
    build_execute!(x, 1, 2, 3, 4, 5, 6);
}
