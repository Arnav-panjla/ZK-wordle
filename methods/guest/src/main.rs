use risc0_zkvm::guest::env;
use risc0_zkvm::sha::{Impl, Sha256};


use risc0_zkvm::sha::Digest;
use serde::{Deserialize, Serialize};

pub const WORD_LENGTH: usize = 3;

pub const ATTEMPT_LIMIT: usize = 6;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum LetterFeedback {
    Correct,
    Present,
    #[default]
    Miss,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct WordFeedback(pub [LetterFeedback; WORD_LENGTH]);

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GameState {
    pub correct_word_hash: Digest,
    pub feedback: WordFeedback,
}

impl WordFeedback {
    pub fn game_is_won(&self) -> bool {
        self.0.iter().all(|x| *x == LetterFeedback::Correct)
    }

    #[cfg(not(target_os = "zkvm"))]
    pub fn print(&self, guess_word: &str) {
        print!("Your results: ");
        for i in 0..WORD_LENGTH {
            match self.0[i] {
                LetterFeedback::Correct => print!("\x1b[41m"), // green
                LetterFeedback::Present => print!("\x1b[43m"), // yellow
                LetterFeedback::Miss => print!("\x1b[40m"),    // black
            }
            print!("{:}", guess_word.chars().nth(i).unwrap());
        }
        println!("\x1b[0m");
        println!();
    }
}


fn main() {
    let secret: String = env::read();
    let guess: String = env::read();

    assert_eq!(
        secret.chars().count(),
        WORD_LENGTH,
        "secret must have length 3!"
    );

    assert_eq!(
        guess.chars().count(),
        WORD_LENGTH,
        "guess must have length 3!"
    );

    let mut feedback: WordFeedback = WordFeedback::default();

    //letters that didn't have an exact match
    let mut secret_unmatched = Vec::<char>::new();

    for i in 0..WORD_LENGTH {
        if secret.as_bytes()[i] != guess.as_bytes()[i] {
            secret_unmatched.push(secret.as_bytes()[i] as char);
        }
    }

    // second round for distinguishing partial matches from misses
    for i in 0..WORD_LENGTH {
        feedback.0[i] = if secret.as_bytes()[i] == guess.as_bytes()[i] {
            LetterFeedback::Correct
        } else if secret_unmatched.contains(&(guess.as_bytes()[i] as char)) {
            LetterFeedback::Present
        } else {
            LetterFeedback::Miss
        }
    }

    let correct_word_hash = *Impl::hash_bytes(&secret.as_bytes());

    let game_state = GameState {
        correct_word_hash,
        feedback,
    };
    env::commit(&game_state); // to the journal
}
