use std::io;

fn parse_sections(string: &str) -> (u8, u8) {
    string.split_once("-").map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap())).unwrap()
}

fn main() {
    let input = io::stdin().lines();
    let lines: &Vec<String> = &input.collect::<Result<_, _>>().unwrap();

    let mut contained: u16 = 0;
    let mut overlaps: u16 = 0;

    lines.into_iter().for_each(|line| {
        let ( elf_a, elf_b ) = line.split_once(",").map(|(a, b)| (parse_sections(a), parse_sections(b))).unwrap();
        let (a_start, a_end) = elf_a;
        let (b_start, b_end) = elf_b;
        if (a_start <= b_start && a_end >= b_end) || (a_start >= b_start && a_end <= b_end) {
            contained += 1;
        }
        if (a_start >= b_start && a_start <= b_end) || (b_start >= a_start && b_start <= a_end) {
            overlaps += 1;
        }
    });

    println!("contained: {}", contained);
    println!("overlaps: {}", overlaps);
}
