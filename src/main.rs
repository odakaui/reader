use anyhow::Result;
use app::{launch_app, ApplicationState};
use article::{Article, Line, Position};
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use token::{Token, POS};
use tokenizer::Tokenizer;

pub mod app;
pub mod article;
pub mod compressor;
pub mod database;
pub mod reader;
pub mod token;
pub mod tokenizer;

fn read_file(path: &Path) -> Result<String> {
    let f = File::open(path)?;
    let mut buf = BufReader::new(f);
    let mut contents = String::new();
    buf.read_to_string(&mut contents)?;

    Ok(contents)
}

pub fn main() -> Result<()> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/japanese.txt");
    let contents = read_file(&path)?;

    let lines = contents.lines();
    let clean_lines: Vec<String> = lines
        .map(|x| x.chars().filter(|c| !c.is_whitespace()).collect())
        .filter(|x| x != "")
        .collect();

    let mut tokenizer = Tokenizer::new()?;
    let file_name = path.file_stem().unwrap().to_str().unwrap();

    let mut tokenized_lines: Vec<Line> = Vec::new();
    for x in clean_lines.iter() {
        let tokens = tokenizer.tokenize(x)?;
        let line = Line {
            sentence: x.into(),
            tokens,
        };

        tokenized_lines.push(line);
    }

    let article = Article {
        file_name: file_name.into(),
        lines: tokenized_lines,
    };

    // create the initial app state
    let position = Position { index: 0, line: 0 };

    let initial_state = ApplicationState {
        line_start: reader::calculate_start(&article, &position),
        line_middle: reader::calculate_middle(&article, &position),
        line_end: reader::calculate_end(&article, &position),
        position,
        font: None,
        article,
    };

    launch_app(initial_state)?;

    Ok(())
}
