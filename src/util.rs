use memmap::{Mmap, MmapOptions};
use std::fs::File;
use std::io::Read;

pub fn aoc_mmap_day_input(day: u8) -> (Mmap, &'static [u8]) {
    let file = File::open(format!("day{:02}.txt", day)).unwrap();
    let mmap = unsafe { MmapOptions::new().map(&file).unwrap() };
    let mmap_buf = unsafe { std::mem::transmute::<&[u8], &'static [u8]>(mmap.as_ref()) };
    (mmap, mmap_buf)
}

pub fn aoc_read_day_lines(day: u8) -> Vec<String> {
    let mut file = File::open(format!("day{:02}.txt", day)).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.lines().map(|s| s.to_string()).collect()
}

pub fn aoc_read_day_bytes(day: u8) -> Vec<u8> {
    let mut file = File::open(format!("day{:02}.txt", day)).unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    contents
}
