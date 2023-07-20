use std::fs;

fn main() {
    let contents = fs::read_to_string("data.txt").expect("Cannot open the file.");

    let mut blocks: Vec<u32> = contents
        .split("\n\n")
        .map(|block| {
            block
                .split("\n")
                .filter(|item| !item.is_empty())
                .map(|item| item.trim().parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect();

    blocks.sort_by(|a, b| b.cmp(a));

    let result: u32 = blocks.iter().take(3).sum();

    println!("Content {:?}", result);
    // println!("Top 3: {result.0} {result.1} {result.2)")
}
