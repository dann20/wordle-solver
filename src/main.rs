use wordle::{Guess, Correctness, Guesser, Algo};

const DICTIONARY: &str = include_str!("../datasets/dictionary.txt");
const GAMES: &str = include_str!("../datasets/answers.txt");

fn play<G: Guesser>(answer: &str, mut guesser: G) -> Option<usize> {
    let mut history = Vec::new();
    for i in 1.. {
        let guess = guesser.guess(&history);
        if answer == guess {
            return Some(i);
        }
        let correctness = Correctness::compute(answer, &guess);
        history.push(Guess {
            word: &guess,
            mask: correctness,
        });
    }
    None
}

fn main() {
    for answer in GAMES.lines() {
        todo!();
    }
    println!("Finish.");
}
