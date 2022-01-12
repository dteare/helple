use std::io::BufRead;
use std::{fmt, fs::File, io};

struct Puzzle {
    #[allow(dead_code)]
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
        let debug = false; // *word == "CRONK".to_string();
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
        #[derive(Debug)]
        struct Suggestion {
            score: usize,
            word: String,
        }

        if self.assignments.len() == 0 {
            return Some("rusty".to_string());
        }

        let mut permitted: Vec<Suggestion> = Vec::new();
        for word in &self.dictionary {
            if self.is_permitted_word(&word) {
                let score = score_for_potential_guess(word);
                permitted.push(Suggestion {
                    score,
                    word: word.clone(),
                });
            }
        }

        permitted.sort_by(|a, b| a.score.cmp(&b.score));

        println!("Suggestions sorted by score:\n{:?}", permitted);

        match permitted.pop() {
            Some(suggestion) => Some(suggestion.word.clone()),
            None => None,
        }
    }

    fn assign_letter(&mut self, letter: char, position: usize, status: LetterStatus) {
        self.assignments.push(Assignment {
            letter,
            position,
            status,
        })
    }
}

/// When guessing a word, we want to "pin" and elimiate letters as fast as possible. Priorty is given to words that use the most unique letters. Further priority is given to words with the most vowels.
fn score_for_potential_guess(word: &String) -> usize {
    use unicode_segmentation::UnicodeSegmentation;
    let mut score = 100;

    for grapheme in word.graphemes(true) {
        let count = word.matches(grapheme).collect::<String>().len();
        if count > 1 {
            score -= count * 2
        }

        let vowels = "AEIOUY"
            .to_string()
            .matches(grapheme)
            .collect::<String>()
            .len();
        score += vowels;
    }

    score
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display = "TODO".to_string();

        write!(f, "{}", display)
    }
}
fn main() -> Result<(), std::io::Error> {
    let puzzle = Puzzle::setup();
    let suggestion = puzzle.suggest_word();

    println!("Suggested word: {:?}", suggestion);

    Ok(())
}

mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn scores() {
        fn assert_score_better_than(better: &str, lesser: &str) {
            let better_score = score_for_potential_guess(&better.to_string());
            let lesser_score = score_for_potential_guess(&lesser.to_string());

            assert!(
                better_score > lesser_score,
                "Score of {} ({}) expected to be higher than {} ({})",
                better,
                better_score,
                lesser,
                lesser_score
            );
        }

        assert_eq!(102, score_for_potential_guess(&"RUSTY".to_string()));
        assert_score_better_than("RUSTY", "GREEN");
        assert_score_better_than("AEIOU", "RUSTY");
    }

    #[test]
    fn jan_11() {
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

    #[test]
    fn jan_12() {
        let mut puzzle = super::Puzzle::setup();

        assert_eq!("rusty", puzzle.suggest_word().unwrap());

        puzzle.assign_letter('R', 0, LetterStatus::WrongSpot);
        puzzle.assign_letter('U', 0, LetterStatus::NotInWord);
        puzzle.assign_letter('S', 0, LetterStatus::NotInWord);
        puzzle.assign_letter('T', 0, LetterStatus::NotInWord);
        puzzle.assign_letter('Y', 0, LetterStatus::NotInWord);
        assert_eq!(false, puzzle.is_permitted_word(&"RUSTY".to_string()));
        assert_eq!(true, puzzle.is_permitted_word(&"GREEN".to_string()));

        let mut suggestion = puzzle.suggest_word().unwrap();

        assert_eq!(suggestion, "VIREO");
        puzzle.assign_letter('V', 0, LetterStatus::WrongSpot);
        puzzle.assign_letter('I', 0, LetterStatus::NotInWord);
        puzzle.assign_letter('R', 0, LetterStatus::WrongSpot);
        puzzle.assign_letter('E', 0, LetterStatus::NotInWord);
        puzzle.assign_letter('O', 0, LetterStatus::WrongSpot);

        suggestion = puzzle.suggest_word().unwrap();

        // FAVOR

        println!("Suggestion: {}", suggestion);
    }
}
