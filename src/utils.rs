use std::fs;

pub(crate) fn read_map() -> Vec<Vec<i32>> {
    fs::read_to_string("map")
        .expect("Unable to read map file")
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|line| {
            line.chars()
                .map(|c| c as i32 - '0' as i32)
                .collect::<Vec<i32>>()
        })
        .collect()
}
