use std::io::BufRead;
use std::{fmt, fs::File, io};
use unicode_segmentation::UnicodeSegmentation;

struct Puzzle {
    #[allow(dead_code)]
    guesses: Vec<(String, Vec<LetterStatus>)>,
    assignments: Vec<Assignment>,
    dictionary: Vec<String>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Assignment {
    letter: char,
    position: usize,
    status: LetterStatus,
}
#[derive(Clone, Copy, Debug, PartialEq)]
enum LetterStatus {
    Correct,
    WrongSpot,
    NotInWord,
}

impl Puzzle {
    fn setup() -> Puzzle {
        println!(">parse");

        let mut dictionary: Vec<String> = Vec::new();
        let input = File::open("./support/less-words").unwrap();

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

    fn solution(&self) -> Option<String> {
        let mut correct_letters: Vec<char> = Vec::new();

        let mut correct_assignments: Vec<&Assignment> = self
            .assignments
            .iter()
            .filter(|a| a.status == LetterStatus::Correct)
            .clone()
            .collect();
        correct_assignments.sort_by(|a, b| a.position.cmp(&b.position));

        assert!(
            correct_assignments.len() <= 5, // TODO: use real length depending on game
            "Found {} 'correct' letters assigned. self.assignments={:?}",
            correct_letters.len(),
            self.assignments
        );

        if correct_assignments.len() != 5 {
            return None;
        }

        for a in &correct_assignments {
            correct_letters.push(a.letter);
        }

        Some(correct_letters.iter().collect())
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
            return Some("RUSTY".to_string());
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

        // println!("Suggestions sorted by score:\n{:?}", permitted);

        match permitted.pop() {
            Some(suggestion) => Some(suggestion.word.to_uppercase()),
            None => None,
        }
    }

    #[allow(dead_code)]
    fn assign_letter(&mut self, letter: char, position: usize, status: LetterStatus) {
        let a = Assignment {
            letter,
            position,
            status,
        };

        // Only add restrictions we aren't already aware of
        if !self.assignments.contains(&a) {
            self.assignments.push(a);
        }
    }

    fn assign_guess_results(&mut self, word: String, letter_statuses: Vec<LetterStatus>) {
        assert!(
            word.len() == letter_statuses.len(),
            "Guessed word <{}> length must match letter statuses exactly: <{:?}>",
            word,
            letter_statuses
        );

        for (i, byte) in word.as_bytes().iter().enumerate() {
            let a = Assignment {
                letter: *byte as char,
                position: i,
                status: letter_statuses[i],
            };

            // Only add restrictions we aren't already aware of
            if !self.assignments.contains(&a) {
                self.assignments.push(a);
            }
        }

        self.guesses.push((word.to_string(), letter_statuses));
    }

    fn assign_guess_from_cli(&mut self, word: String, input: &str) {
        let mut letter_statuses: Vec<LetterStatus> = Vec::new();

        for (i, grapheme) in input.graphemes(true).enumerate() {
            let letter_status = match grapheme {
                "X" => Some(LetterStatus::Correct),
                "." => {
                    // Partial hit
                    Some(LetterStatus::WrongSpot)
                }
                "-" => Some(LetterStatus::NotInWord),
                _ => None,
            };

            if let Some(status) = letter_status {
                letter_statuses.push(status);
            } else {
                println!("Unexpected <{}> in input at character {}.", grapheme, i);
                println!(
                    r#"Expected format for puzzle results:
`X` = direct hit (right letter in right position
`.` = partial hit (right letter in wrong position)
`-` = complete miss (letter not in word)"#
                );
                return;
            }
        }

        self.assign_guess_results(word, letter_statuses);
    }
}

/// When guessing a word, we want to "pin" and elimiate letters as fast as possible. Priorty is given to words that use the most unique letters. Further priority is given to words with the most vowels.
fn score_for_potential_guess(word: &String) -> usize {
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
        let mut display = "".to_string();

        for (_word, statuses) in &self.guesses {
            for s in statuses {
                display.push_str(format!("{}", s).as_str());
            }
            display.push_str("\n");
        }

        write!(f, "{}", display)
    }
}

impl fmt::Display for LetterStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            &LetterStatus::Correct => "🟩",
            &LetterStatus::WrongSpot => "🟨",
            &LetterStatus::NotInWord => "⬜",
        };

        write!(f, "{}", s)
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut puzzle = Puzzle::setup();
    let mut buffer = String::new();
    let stdin = io::stdin();

    loop {
        let suggestion = puzzle.suggest_word();

        match suggestion {
            Some(word) => {
                println!("Go type <{:?}> into the puzzle. What was the result?", word);

                stdin.read_line(&mut buffer)?;
                let input = buffer.trim().clone();

                puzzle.assign_guess_from_cli(word, input);
                println!("{}", puzzle);
                buffer.clear();
            }
            None => {
                println!("No suggestion available. 💥");
                break;
            }
        }

        if let Some(solution) = puzzle.solution() {
            println!("Puzzle solved using {}! 🙌 Share your score. 😘", solution);
            break;
        }
    }

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
    fn solution() {
        let mut puzzle = super::Puzzle::setup();

        assert_eq!(None, puzzle.solution());

        puzzle.assign_letter('R', 0, LetterStatus::NotInWord);
        puzzle.assign_letter('U', 1, LetterStatus::NotInWord);
        puzzle.assign_letter('S', 2, LetterStatus::NotInWord);
        puzzle.assign_letter('T', 3, LetterStatus::NotInWord);
        puzzle.assign_letter('Y', 4, LetterStatus::Correct);

        assert_eq!(None, puzzle.solution());

        puzzle.assign_letter('Z', 0, LetterStatus::NotInWord);
        puzzle.assign_letter('A', 1, LetterStatus::WrongSpot);
        puzzle.assign_letter('I', 2, LetterStatus::NotInWord);
        puzzle.assign_letter('D', 3, LetterStatus::NotInWord);
        puzzle.assign_letter('Y', 4, LetterStatus::Correct);

        assert_eq!(None, puzzle.solution());

        puzzle.assign_letter('V', 0, LetterStatus::NotInWord);
        puzzle.assign_letter('E', 1, LetterStatus::WrongSpot);
        puzzle.assign_letter('A', 2, LetterStatus::WrongSpot);
        puzzle.assign_letter('L', 3, LetterStatus::NotInWord);
        puzzle.assign_letter('Y', 4, LetterStatus::Correct);

        assert_eq!(None, puzzle.solution());

        puzzle.assign_letter('E', 0, LetterStatus::WrongSpot);
        puzzle.assign_letter('M', 1, LetterStatus::NotInWord);
        puzzle.assign_letter('B', 2, LetterStatus::Correct);
        puzzle.assign_letter('A', 3, LetterStatus::WrongSpot);
        puzzle.assign_letter('Y', 4, LetterStatus::Correct);

        assert_eq!(None, puzzle.solution());

        puzzle.assign_letter('A', 0, LetterStatus::Correct);
        puzzle.assign_letter('B', 1, LetterStatus::Correct);
        puzzle.assign_letter('B', 2, LetterStatus::Correct);
        puzzle.assign_letter('E', 3, LetterStatus::Correct);
        puzzle.assign_letter('Y', 4, LetterStatus::Correct);

        assert_eq!(Some("ABBEY".to_string()), puzzle.solution());
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

    #[test]
    fn jan_14() {
        let mut puzzle = super::Puzzle::setup();

        puzzle.assign_guess_results(
            "RUSTY".to_string(),
            vec![
                LetterStatus::NotInWord,
                LetterStatus::NotInWord,
                LetterStatus::NotInWord,
                LetterStatus::WrongSpot,
                LetterStatus::Correct,
            ],
        );

        let mut suggestion = puzzle.suggest_word();

        puzzle.assign_guess_results(
            "TONEY".to_string(),
            vec![
                LetterStatus::Correct,
                LetterStatus::NotInWord,
                LetterStatus::Correct,
                LetterStatus::NotInWord,
                LetterStatus::Correct,
            ],
        );

        suggestion = puzzle.suggest_word();

        puzzle.assign_guess_results(
            "TANKY".to_string(),
            vec![
                LetterStatus::Correct,
                LetterStatus::Correct,
                LetterStatus::Correct,
                LetterStatus::NotInWord,
                LetterStatus::Correct,
            ],
        );

        suggestion = puzzle.suggest_word();

        puzzle.assign_guess_results(
            "TANGY".to_string(),
            vec![
                LetterStatus::Correct,
                LetterStatus::Correct,
                LetterStatus::Correct,
                LetterStatus::Correct,
                LetterStatus::Correct,
            ],
        );

        suggestion = puzzle.suggest_word();

        assert_eq!(None, suggestion);
        assert_eq!(puzzle.solution(), Some("TANGY".to_string()));
    }
}
