use std::fs;

fn main() {
    let file_path = "./src/input.txt";
    let contents = fs::read_to_string(file_path).expect("should have been able to read the file");

    let mut calories: Vec<_> = contents
        .split("\n\n")
        .map(|x| {
            x.lines()
                .map(|x| x.parse::<i32>().unwrap_or_default())
                .sum::<i32>()
        })
        .collect::<Vec<i32>>();

    calories.sort();

    let top_three_sum: i32 = calories.iter().rev().take(3).sum();

    println!("{:?}", &top_three_sum);
}
