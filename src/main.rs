use std::io::BufRead;
use std::{fmt, fs::File, io};

struct Puzzle {
    guesses: Vec<String>,
    assignments: Vec<Assignment>,
    dictionary: Vec<String>,
}

#[derive(Clone, Copy, Debug)]
struct Assignment {
    letter: char,
    position: usize,
    status: LetterStatus,
}
#[derive(Clone, Copy, Debug)]
enum LetterStatus {
    Correct,
    WrongSpot,
    NotInWord,
}

impl Puzzle {
    fn setup() -> Puzzle {
        println!(">parse");

        let mut dictionary: Vec<String> = Vec::new();
        let input = File::open("./support/words").unwrap();

        for word_result in io::BufReader::new(input).lines() {
            let word = word_result.unwrap();
            let trimmed = word.trim();
            if trimmed.len() != 5 {
                continue;
            }

            dictionary.push(trimmed.to_string().to_uppercase());
        }

        Puzzle {
            guesses: vec![],
            assignments: vec![],
            dictionary,
        }
    }

    fn is_permitted_word(&self, word: &String) -> bool {
        let debug = *word == "CRONK".to_string();
        for a in &self.assignments {
            if debug {
                println!("Looking at rule {:?}", a);
            }
            let pass = match a.status {
                LetterStatus::Correct => word.as_bytes()[a.position] == a.letter as u8,
                LetterStatus::WrongSpot => {
                    if debug {
                        println!("   DEBUGGING LetterStatus::WrongSpot  ");
                        println!(
                            " - word.contains({})? {}",
                            a.letter,
                            word.contains(a.letter)
                        );
                        println!(" - word[{}] = {}", a.position, word.as_bytes()[a.position]);
                        println!(" - a.letter={}", a.letter as u8);
                    }
                    word.contains(a.letter) && word.as_bytes()[a.position] != a.letter as u8
                }
                LetterStatus::NotInWord => {
                    if debug {
                        println!("Word contains {}? {}", a.letter, !word.contains(a.letter));
                    }
                    !word.contains(a.letter)
                }
            };

            if !pass {
                if debug {
                    println!(
                        "Word <{}> was rejected because it doesn't meet the requirements for {:?}",
                        word, a
                    );
                }
                return false;
            }
        }

        true
    }

    fn suggest_word(&self) -> Option<String> {
        if self.assignments.len() == 0 {
            return Some("rusty".to_string());
        }

        for word in &self.dictionary {
            if self.is_permitted_word(&word) {
                return Some(word.clone());
            }
        }

        None
    }

    fn assign_letter(&mut self, letter: char, position: usize, status: LetterStatus) {
        self.assignments.push(Assignment {
            letter,
            position,
            status,
        })
    }
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut display = "TODO".to_string();

        write!(f, "{}", display)
    }
}
fn main() -> Result<(), std::io::Error> {
    let mut puzzle = Puzzle::setup();
    let suggestion = puzzle.suggest_word();

    println!("Suggested word: {:?}", suggestion);

    Ok(())
}

mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn baby_steps() {
        let mut puzzle = super::Puzzle::setup();

        assert_eq!("rusty", puzzle.suggest_word().unwrap());

        puzzle.assign_letter('R', 1, LetterStatus::Correct);
        puzzle.assign_letter('U', 0, LetterStatus::NotInWord);
        puzzle.assign_letter('S', 0, LetterStatus::NotInWord);
        puzzle.assign_letter('T', 0, LetterStatus::NotInWord);
        puzzle.assign_letter('Y', 0, LetterStatus::NotInWord);
        assert_eq!(false, puzzle.is_permitted_word(&"RUSTY".to_string()));
        assert_eq!(true, puzzle.is_permitted_word(&"GREEN".to_string()));

        puzzle.assign_letter('G', 0, LetterStatus::NotInWord);
        puzzle.assign_letter('R', 1, LetterStatus::Correct);
        puzzle.assign_letter('E', 0, LetterStatus::NotInWord);
        puzzle.assign_letter('N', 4, LetterStatus::WrongSpot);
        assert_eq!(false, puzzle.is_permitted_word(&"GREEN".to_string()));

        let mut suggestion = puzzle.suggest_word().unwrap();

        assert_eq!("BRAND".to_string(), suggestion);

        puzzle.assign_letter('B', 0, LetterStatus::NotInWord);
        puzzle.assign_letter('A', 0, LetterStatus::NotInWord);
        puzzle.assign_letter('N', 3, LetterStatus::Correct);
        puzzle.assign_letter('D', 4, LetterStatus::WrongSpot);

        suggestion = puzzle.suggest_word().unwrap();

        assert_eq!("DRINK".to_string(), suggestion);

        println!("Suggestion: {}", suggestion);
    }
}
