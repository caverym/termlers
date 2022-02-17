use std::fmt::Display;

use lliw::Bg;

use crate::words::{Char, Word};

const EMPTY: u8 = b'_';

/*

   █████       score: 0
   █████
   █████
   █████
   █████

   > _
*/

#[derive(Debug)]
pub struct Buffer {
    pub(crate) word_size: usize,
    pub(crate) words: [Option<Word>; 5],
    pub(crate) score: usize,
    pub(crate) guess: usize,
}

impl Buffer {
    pub fn new(word_length: usize) -> Buffer {
        Buffer {
            word_size: word_length,
            words: [None, None, None, None, None],
            score: 0,
            guess: 0,
        }
    }

    #[inline]
    pub fn set_score(&mut self, score: usize) {
        self.score = score
    }

    pub fn reset(&mut self) {
        self.words = [None, None, None, None, None];
        self.guess = 0;
    }

    #[inline]
    pub fn all_some(&self) -> bool {
        self.words[0].is_some()
            && self.words[1].is_some()
            && self.words[2].is_some()
            && self.words[3].is_some()
            && self.words[4].is_some()
    }

    #[inline]
    pub fn all_none(&self) -> bool {
        !self.all_some()
    }

    pub fn gather_kcu(&self) -> (Vec<Char>, Vec<Char>, Vec<Char>) {
        let mut known = Vec::new();
        let mut correct = Vec::new();
        let mut unused = Vec::new();

        for opt_word in &self.words {
            if let Some(word) = opt_word {
                for c in word.clone().into_iter() {
                    match c.color {
                        Some(Bg::Green) => {
                            if !correct.contains(&c) {
                                correct.insert(correct.len(), c);
                            }
                        }
                        Some(Bg::Yellow) => {
                            if !known.contains(&c) {
                                known.insert(known.len(), c);
                            }
                        }
                        _ => if !unused.contains(&c) {
                            unused.insert(unused.len(), c);
                        }
                    }
                }
            }
        }

        correct.dedup();
        known.dedup();
        unused.dedup();

        (correct, known, unused)
    }

    pub fn next(&self) -> usize {
        if self.all_some() {
            return 5;
        }

        let mut pos: usize = 0;
        for w in &self.words {
            if w.is_some() {
                pos += 1;
            }
        }

        pos
    }

    pub fn add(&mut self, word: Word) -> bool {
        self.guess += 1;
        if self.all_some() {
            return true;
        }

        let next = self.next();
        assert!(next < 5);

        self.words[next] = Some(word);

        self.all_some()
    }
}

impl Display for Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt: String = "\n".to_string();
        let empty: String = String::from_utf8_lossy(&vec![EMPTY; self.word_size]).to_string();

        for i in 0..5 {
            if let Some(word) = &self.words[i] {
                if i == 0 {
                    fmt = format!("{}\t{}\tscore: {}\n", fmt, word, self.score)
                } else {
                    fmt = format!("{}\t{}\n", fmt, word)
                }
            } else {
                if i == 0 {
                    fmt = format!("{}\t{}\tscore: {}\n", fmt, empty, self.score)
                } else {
                    fmt = format!("{}\t{}\n", fmt, empty)
                }
            }
        }

        Display::fmt(&fmt, f)
    }
}

#[test]
fn gen_board() {
    const BOARD: &str =
        "\n\t_____\tscore: 0\n\t_____\n\t_____\n\t_____\n\t_____\n";

    let buffer = Buffer {
        word_size: 5,
        words: [None, None, None, None, None],
        score: 0,
        guess: 0,
    };

    assert_eq!(BOARD, buffer.to_string());
}
