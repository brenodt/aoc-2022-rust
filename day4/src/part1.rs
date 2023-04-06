use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("should read file");

    let pairs: Vec<Vec<_>> = input
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|pairs| pairs.split(",").collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut count = 0;

    for pair in pairs {
        let first_elf_bounds: Vec<usize> = pair[0].split("-").map(|x| x.parse().unwrap()).collect();
        let second_elf_bounds: Vec<usize> =
            pair[1].split("-").map(|x| x.parse().unwrap()).collect();

        if is_contained(&first_elf_bounds, &second_elf_bounds)
            || is_contained(&second_elf_bounds, &first_elf_bounds)
        {
            count += 1;
        }
    }
    println!("{:?}", count);
}

fn is_contained(reference: &Vec<usize>, being_compared: &Vec<usize>) -> bool {
    return reference.first() <= being_compared.first()
        && reference.last() >= being_compared.last();
}
