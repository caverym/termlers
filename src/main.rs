#![feature(thread_is_running)]

use std::io::{stdout, Stdout};

use engine::Engine;
use words::WordList;

use crate::buffer::Buffer;

mod buffer;
mod engine;
mod error;
mod words;

fn main() {

    let runner = || -> error::Result<()> {
        let mut arg = pico_args::Arguments::from_env();
        let word_length = arg.value_from_str(["-l", "--length"]).unwrap_or(5);
        let mut word_list = WordList::load(word_length);
        let mut engine = Engine::init(&mut word_list, word_length)?;
        engine.start()
    };

    std::process::exit(
        if let Err(e) = runner() {
            eprintln!("{e}");
            1
        } else {
            0
        }
    );

}
