use lliw::{Bg, Reset};
use rand::{prelude::ThreadRng, Rng};
use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

const RAW_WORD_LIST: &str = include_str!("../english-words/words_alpha.txt");

pub struct WordList {
    rng: ThreadRng,
    list: Vec<Word>,
}

impl WordList {
    pub fn load(word_length: usize) -> Self {
        let words_str: Vec<&str> = RAW_WORD_LIST
            .split('\n')
            .filter_map(|word| {
                let new = word.trim();
                if new.len() == word_length {
                    Some(new)
                } else {
                    None
                }
            })
            .collect();

        let list: Vec<Word> = words_str.iter().map(|word| word.into()).collect();

        let rng: ThreadRng = rand::thread_rng();

        Self { rng, list }
    }

    pub fn random(&mut self) -> Word {
        let range = 0..self.len();
        let idx = self.rng.gen_range(range);
        self.index(idx).clone()
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn contains(&self, word: &Word) -> bool {
        self.list.contains(word)
    }
}

impl Index<usize> for WordList {
    type Output = Word;

    fn index(&self, index: usize) -> &Self::Output {
        Index::index(&self.list, index)
    }
}

impl IntoIterator for WordList {
    type Item = Word;

    type IntoIter = WordListIter;

    fn into_iter(self) -> Self::IntoIter {
        WordListIter {
            idx: 0,
            next: 1,
            max: self.len(),
            list: self,
        }
    }
}
pub struct WordListIter {
    list: WordList,
    idx: usize,
    next: usize,
    max: usize,
}

impl Iterator for WordListIter {
    type Item = Word;

    fn next(&mut self) -> Option<Self::Item> {
        self.idx = self.next;
        self.next += 1;

        if self.idx == self.max {
            None
        } else {
            Some(self.list[self.idx].clone())
        }
    }
}

#[test]
fn word_length_1() {
    let wordlist = WordList::load(1);
    wordlist.into_iter().for_each(|w| assert_eq!(w.len(), 1))
}

#[test]
fn word_length_2() {
    let wordlist = WordList::load(2);
    wordlist.into_iter().for_each(|w| assert_eq!(w.len(), 2))
}

#[test]
fn word_length_3() {
    let wordlist = WordList::load(3);
    wordlist.into_iter().for_each(|w| assert_eq!(w.len(), 3))
}

#[test]
fn word_length_4() {
    let wordlist = WordList::load(4);
    wordlist.into_iter().for_each(|w| assert_eq!(w.len(), 4))
}

#[test]
fn word_length_5() {
    let wordlist = WordList::load(5);
    wordlist.into_iter().for_each(|w| assert_eq!(w.len(), 5))
}

#[test]
fn word_length_10() {
    let wordlist = WordList::load(10);
    wordlist.into_iter().for_each(|w| assert_eq!(w.len(), 10))
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Word {
    inner: Vec<Char>,
}

impl Word {
    pub fn starts_with(&self, needle: &str) -> bool {
        let needle: Vec<Char> = needle.chars().map(|c| c.into()).collect();
        self.inner.starts_with(&needle)
    }
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn contains(&self, c: &Char) -> bool {
        self.inner.contains(c)
    }

    pub fn set_all(&mut self) {
        self.inner.iter_mut().for_each(|c| c.set_color(Bg::Green))
    }

    pub fn set_idx_color(&mut self, index: usize, color: Bg) {
        self[index].set_color(color);
    }
}

impl Index<usize> for Word {
    type Output = Char;

    fn index(&self, index: usize) -> &Self::Output {
        Index::index(&self.inner, index)
    }
}

impl IndexMut<usize> for Word {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        IndexMut::index_mut(&mut self.inner, index)
    }
}

impl IntoIterator for Word {
    type Item = Char;

    type IntoIter = WordIter;

    fn into_iter(self) -> Self::IntoIter {
        WordIter {
            size: self.len(),
            word: self,
            pos: 0,
        }
    }
}

pub struct WordIter {
    word: Word,
    size: usize,
    pos: usize,
}

impl Iterator for WordIter {
    type Item = Char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos == self.size {
            return None;
        }
        self.pos += 1;
        let idx = self.pos - 1;

        Some(*self.word.index(idx))
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt: String = String::new();

        for c in &self.inner {
            fmt = format!("{}{}", fmt, c)
        }

        Display::fmt(&fmt, f)
    }
}

impl From<String> for Word {
    fn from(s: String) -> Self {
        s.as_str().into()
    }
}

impl From<&str> for Word {
    fn from(s: &str) -> Self {
        Self {
            inner: s.chars().map(|c| c.into()).collect(),
        }
    }
}

impl From<&&str> for Word {
    fn from(s: &&str) -> Self {
        Self {
            inner: s.chars().map(|c| c.into()).collect(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Char {
    pub color: Option<Bg>,
    c: char,
}

impl Char {
    pub fn set_color(&mut self, color: Bg) {
        self.color = Some(color)
    }
}

impl From<char> for Char {
    fn from(c: char) -> Self {
        Self { color: None, c }
    }
}

impl From<u8> for Char {
    fn from(u: u8) -> Self {
        (u as char).into()
    }
}

impl Into<char> for Char {
    fn into(self) -> char {
        self.c
    }
}

impl Into<u8> for Char {
    fn into(self) -> u8 {
        self.c as u8
    }
}

impl PartialEq for Char {
    fn eq(&self, other: &Self) -> bool {
        self.c.eq(&other.c)
    }
}

impl PartialOrd for Char {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.c.partial_cmp(&other.c)
    }
}

impl Eq for Char {}

impl Ord for Char {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.c.cmp(&other.c)
    }
}

impl Display for Char {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.color {
            Some(color) => write!(f, "{}{}{}", color, self.c, Reset),
            None => Display::fmt(&self.c, f),
        }
    }
}
