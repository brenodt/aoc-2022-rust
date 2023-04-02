use std::fs;

const LETTER_A_LOWERCASE: u32 = 'a' as u32;
const LETTER_A_UPPERCASE: u32 = 'A' as u32;

fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("should open input file");

    let rucksacks: Vec<_> = input
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|x| x.split_at(x.len() / 2))
        .collect();

    let mut sum = 0;

    for rucksack in rucksacks {
        let mut score = 0;
        for letter in rucksack.0.chars() {
            if score > 0 {
                break;
            }
            for letter2 in rucksack.1.chars() {
                if letter == letter2 {
                    if letter.is_lowercase() {
                        score = get_lowercase_score(letter);
                    } else {
                        score = get_uppercase_score(letter);
                    }
                    break;
                }
            }
        }

        sum += score;
    }

    println!("{:?}", sum);
}

fn get_lowercase_score(ch: char) -> u32 {
    return (ch as u32) - LETTER_A_LOWERCASE + 1;
}

fn get_uppercase_score(ch: char) -> u32 {
    return (ch as u32) - LETTER_A_UPPERCASE + 27;
}
