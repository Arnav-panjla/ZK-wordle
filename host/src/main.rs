// 40: Black background
// 41: Red background
// 42: Green background
// 43: Yellow background
// 44: Blue background
// 45: Magenta background
// 46: Cyan background
// 47: White background


use methods::{
    GUEST_CODE_FOR_ZK_PROOF_ELF, GUEST_CODE_FOR_ZK_PROOF_ID
};
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt};
use std::io;
use risc0_zkvm::sha::Digest;
use serde::{Deserialize, Serialize};

mod wordlist;

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
                LetterFeedback::Correct => print!("\x1b[42m"), // green
                LetterFeedback::Present => print!("\x1b[46m"), // Cyan
                LetterFeedback::Miss => print!("\x1b[40m"),    // black
            }
            print!("{:}", guess_word.chars().nth(i).unwrap());
        }
        println!("\x1b[0m");
        println!();
    }
}

struct Server<'a> {
    secret_word: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(secret_word: &'a str) -> Self {
        Self { secret_word }
    }

    pub fn get_secret_word_hash(&self) -> Digest {
        let receipt = self.check_round("___");
        let game_state: GameState = receipt.journal.decode().unwrap();
        game_state.correct_word_hash
    }

    pub fn check_round(&self, guess_word: &str) -> Receipt {
        let env = ExecutorEnv::builder()
            .write(&self.secret_word)
            .unwrap()
            .write(&guess_word)
            .unwrap()
            .build()
            .unwrap();

        // Obtain the default prover.
        let prover = default_prover();

        // Produce a receipt by proving the specified ELF binary.
        prover.prove(env, GUEST_CODE_FOR_ZK_PROOF_ELF).unwrap().receipt
    }
}

struct Player {
    pub hash: Digest,
}

impl Player {
    pub fn check_receipt(&self, receipt: Receipt) -> WordFeedback {
        receipt
            .verify(GUEST_CODE_FOR_ZK_PROOF_ID)
            .expect("receipt verification failed");

        let game_state: GameState = receipt.journal.decode().unwrap();
        if game_state.correct_word_hash != self.hash {
            panic!("The hash mismatched, so the server cheated!");
        }
        game_state.feedback
    }
}

fn read_stdin_guess() -> String {
    let mut guess = String::new();
    loop {
        io::stdin().read_line(&mut guess).expect("Enter something");
        guess.pop(); // remove trailing newline

        if guess.chars().count() == WORD_LENGTH {
            break;
        } else {
            println!("Your guess must have 3 letters. Try again :)");
            guess.clear();
        }
    }
    guess
}

fn play_rounds(server: Server, player: Player, rounds: usize) -> bool {
    for turn_index in 0..rounds {
        let remaining_guesses = rounds - turn_index;
        let guess_word = read_stdin_guess();
        let receipt = server.check_round(guess_word.as_str());
        let score = player.check_receipt(receipt);

        if remaining_guesses == rounds {
            println!("Good guess! Our server has calculated your results.");
            println!("You'll have {} chances to get the word right.", ATTEMPT_LIMIT);
        } else {
            println!("You have {} guesses remaining.", remaining_guesses);
        }

        score.print(guess_word.as_str());
        if score.game_is_won() {
            return true;
        }
    }
    false
}

fn print_instructions() {
    println!("Wordle Game Instructions\n");

    println!("1. Guess a Word:");
    println!("   - Enter a {}-letter word as your guess. You will have {} attempts to guess the correct word.\n", WORD_LENGTH, ATTEMPT_LIMIT);

    println!("2. Feedback Colors:");
    println!("   After each guess, you'll receive feedback based on the colors of the letters:");

    println!("   - 🟩 Green: The letter is **correct** and at correct position.");

    println!("   - 🟦 Cyan: The letter is **present**  but  in the **wrong position**.");

    println!("   - ⬛ Black: The letter is **not in the secret word** at all.");

    println!("\nGood luck and have fun guessing the word!");
}

fn main() {
    print_instructions();

    let server = Server::new(wordlist::pick_word());
    let player = Player {
        hash: server.get_secret_word_hash(),
    };

    if play_rounds(server, player,ATTEMPT_LIMIT) {
        println!("You won!\n");
    } else {
        println!("Game over!\n");
    }
}

