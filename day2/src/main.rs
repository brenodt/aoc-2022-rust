use std::fs;

const OUTCOME_LOSE: &str = "X";
const OUTCOME_DRAW: &str = "Y";
const OUTCOME_WIN: &str = "Z";

fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("should open input");
    let max_score: usize = input
        .split("\n")
        .filter(|x| x.len() > 1)
        .map(|x| calculate_score(x))
        .sum();
    println!("{}", max_score);
}

#[derive(Debug)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

#[derive(Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

fn calculate_score(input: &str) -> usize {
    let outcome_win = 6;
    let outcome_draw = 3;
    let outcome_lose = 0;

    let shape_played = get_shape_to_send(input);

    let shape_score = get_score_for_shape(shape_played);

    match check_match_outcome(input) {
        Outcome::Win => return outcome_win + shape_score,
        Outcome::Draw => return outcome_draw + shape_score,
        Outcome::Lose => return outcome_lose + shape_score,
    }
}

fn get_score_for_shape(shape_played: Shape) -> usize {
    let shape_rock = 1;
    let shape_paper = 2;
    let shape_scissors = 3;

    match shape_played {
        Shape::Rock => return shape_rock,
        Shape::Paper => return shape_paper,
        Shape::Scissors => return shape_scissors,
    }
}

fn check_match_outcome(input: &str) -> Outcome {
    let inputs: Vec<_> = input.split(" ").collect();

    if inputs[1] == OUTCOME_WIN {
        return Outcome::Win;
    } else if inputs[1] == OUTCOME_LOSE {
        return Outcome::Lose;
    } else {
        return Outcome::Draw;
    }
}

fn get_shape_to_send(input: &str) -> Shape {
    let rock_in = "A";
    let paper_in = "B";

    let inputs: Vec<_> = input.split(" ").collect();

    if inputs[0] == rock_in {
        if inputs[1] == OUTCOME_LOSE {
            return Shape::Scissors;
        } else if inputs[1] == OUTCOME_DRAW {
            return Shape::Rock;
        } else {
            return Shape::Paper;
        }
    } else if inputs[0] == paper_in {
        if inputs[1] == OUTCOME_DRAW {
            return Shape::Paper;
        } else if inputs[1] == OUTCOME_WIN {
            return Shape::Scissors;
        } else {
            return Shape::Rock;
        }
    } else {
        if inputs[1] == OUTCOME_WIN {
            return Shape::Rock;
        } else if inputs[1] == OUTCOME_LOSE {
            return Shape::Paper;
        } else {
            return Shape::Scissors;
        }
    }
}
