use std::fs;

const LETTER_A_LOWERCASE: u32 = 'a' as u32;
const LETTER_A_UPPERCASE: u32 = 'A' as u32;

fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("should open input file");

    let rucksacks: Vec<_> = input.split_whitespace().collect();

    let mut sum = 0;
    for chunk in rucksacks.chunks(3) {
        for letter in chunk[0].chars() {
            if !chunk[1].contains(letter) || !chunk[2].contains(letter) {
                continue;
            }

            match letter.is_lowercase() {
                true => sum += get_lowercase_score(letter),
                false => sum += get_uppercase_score(letter),
            }
            break;
        }
    }

    println!("{:?}", sum);
}

fn get_lowercase_score(ch: char) -> u32 {
    return (ch as u32) - LETTER_A_LOWERCASE + 1;
}

fn get_uppercase_score(ch: char) -> u32 {
    return (ch as u32) - LETTER_A_UPPERCASE + 27;
}
