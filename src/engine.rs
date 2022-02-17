use crate::buffer::Buffer;
use crate::error::{Error, Result};
use crate::words::{Word, WordList};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{
    cursor::{DisableBlinking, EnableBlinking, MoveLeft, MoveTo, RestorePosition, SavePosition},
    execute, ExecutableCommand,
};
use lliw::Bg;
use std::fmt::Display;
use std::io::{stdin, stdout, BufRead, Read, Stdin, Stdout, Write};
use std::ops::RangeBounds;

pub struct Engine<'a> {
    word_length: usize,
    word_list: &'a mut WordList,
    inp: Stdin,
    oup: Stdout,
    buffer: Buffer,
}

impl<'a> Engine<'a> {
    pub fn init(word_list: &'a mut WordList, word_length: usize) -> Result<Self> {
        let inp: Stdin = stdin();
        let mut out: Stdout = stdout();
        out.execute(SavePosition)?;
        let buffer = Buffer::new(word_length);

        Ok(Self {
            word_length,
            word_list,
            inp,
            oup: out,
            buffer,
        })
    }

    pub fn start(&mut self) -> Result<()> {
        let word = self.word_list.random();
        // self.oup.write(format!("{}\n", word).as_bytes())?;
        loop {
            let mut found: bool = false;
            self.write_board()?;
            let guess = self.read_guess()?;
            if guess == "!exit".into() {
                return Ok(());
            } else if guess == "!give".into() {
                self.buffer.guess = 5;
            } else {
                let (tf, output) = self.compare(&word, &guess);
                found = tf;
                self.buffer.add(output);
            }

            if found || self.buffer.guess == 5 {
                self.write_board()?;
                if found {
                    self.buffer.set_score(self.buffer.score + 1)
                } else {
                    self.buffer.set_score(0);
                    self.write_correct(&word)?;
                }
                self.buffer.reset();
                self.enter()?;
                return self.start();
            }
        }
        Ok(())
    }

    pub fn write_correct(&mut self, word: &Word) -> Result<()> {
        let out = format!("correct word: {}", word);
        self.oup.write(out.as_bytes())?;
        self.oup.flush()?;
        Ok(())
    }

    pub fn compare(&self, word: &Word, guess: &Word) -> (bool, Word) {
        if word == guess {
            let mut green = word.clone();
            green.set_all();
            return (true, green);
        }

        let mut new: Word = guess.clone();

        for (index, c) in guess.clone().into_iter().enumerate() {
            if word.contains(&c) {
                let color = if word[index] == c {
                    Bg::Green
                } else {
                    Bg::Yellow
                };
                new.set_idx_color(index, color);
            }
        }

        (false, new)
    }

    pub fn write_board(&mut self) -> Result<()> {
        self.oup.execute(Clear(ClearType::All))?;
        //self.oup.execute(RestorePosition)?;
        let output = self.buffer.to_string();
        //self.oup.execute(RestorePosition)?;
        self.oup.write(output.as_bytes())?;
        self.write_kc()?;
        self.oup.flush()?;
        Ok(())
    }

    pub fn write_kc(&mut self) -> Result<()> {
        use std::thread::spawn;

        let (corret, known, unused) = self.buffer.gather_kcu();

        let correct_handle = spawn(move || format_vector(corret));
        let known_handle = spawn(move || format_vector(known));
        let unused_handle = spawn(move || format_vector(unused));

        let correct_fmt = correct_handle.join()?;
        let known_fmt = known_handle.join()?;
        let unused_fmt = unused_handle.join()?;
        
        let out = format!("correct: {}\nknown:\t{}\nunused:\t{}\n", correct_fmt, known_fmt, unused_fmt);

        self.oup.write(out.as_bytes())?;

        Ok(())
    }

    pub fn enter(&mut self) -> Result<()> {
        let mut buffer: String = String::new();
        self.inp.read_line(&mut buffer)?;
        Ok(())
    }

    pub fn read_guess(&mut self) -> Result<Word> {
        loop {
            let mut buffer: String = String::new();
            self.inp.read_line(&mut buffer)?;
            let tmp = buffer.trim();
            let word: Word = tmp.into();
            if !word.starts_with("!") {
                if tmp.len() != self.word_length
                    || tmp.is_empty()
                    || !self.word_list.contains(&word)
                {
                    self.write_board()?;
                    continue;
                }
            }
            return Ok(word);
        }
    }
}

fn format_vector<T: Display>(v: Vec<T>) -> String {
    let mut v_fmt = String::new();

    if v.is_empty() {
        v_fmt = format!("\t")
    } else {
        for c in v {
            v_fmt = format!("{}{}", v_fmt, c);
        }
    }

    v_fmt
}