use std::{collections::HashSet, io};

static CHAR_LA: u32 = 97;
static CHAR_LZ: u32 = 122;
static CHAR_UA: u32 = 65;
static CHAR_UZ: u32 = 90;
static PRIORITY_LA: u32 = 1;
static PRIORITY_UA: u32 = 27;

fn prioritize(c: u32) -> u32 {
    if c >= CHAR_LA && c <= CHAR_LZ {
        return c - CHAR_LA + PRIORITY_LA;
    } else if c >= CHAR_UA && c <= CHAR_UZ {
        return c - CHAR_UA + PRIORITY_UA;
    } else {
        return 0;
    }
}

fn unique_items(items: &str) -> HashSet<u32> {
    items
        .chars()
        .into_iter()
        .map(|c| prioritize(c.into()))
        .collect()
}

fn calculate_duplicated_priorities(lines: &Vec<String>) -> u32 {
    lines
        .into_iter()
        .filter_map(|line| {
            let (a, b) = line.split_at(line.len() / 2);
            Some((unique_items(a), unique_items(b)))
        })
        .map(|(a, b)| *a.intersection(&b).next().unwrap())
        .sum()
}

fn calculate_groups_authorization(lines: &Vec<String>) -> u32 {
    lines
        .chunks(3)
        .into_iter()
        .filter_map(|group| {
            Some((
                unique_items(&group[0]),
                unique_items(&group[1]),
                unique_items(&group[2]),
            ))
        })
        .map(|(a, b, c)| {
            let ab = a.intersection(&b).collect::<HashSet<_>>();
            let bc = b.intersection(&c).collect::<HashSet<_>>();
            *ab.intersection(&bc).next().unwrap().clone()
        })
        .sum()
}

fn main() {
    let input = io::stdin().lines();
    let lines: &Vec<String> = &input.collect::<Result<_, _>>().unwrap();

    let phase_1 = calculate_duplicated_priorities(lines);
    let phase_2 = calculate_groups_authorization(lines);

    println!("Phase 1: {}", phase_1);
    println!("Phase 2: {}", phase_2);
}
