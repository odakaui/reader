use super::{File, Token, common, tokenizer};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Line {
    pub sentence: String,
    pub tokens: Vec<Token>,
}

impl Line {
    pub fn new(sentence: &str, tokens: &Vec<Token>) -> Self {
        Line { sentence: sentence.into(), tokens: tokens.to_owned().to_vec() } 
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Article {
    pub file: File,
    pub lines: Vec<Line>,
}

impl Article {
    pub fn open(path: &PathBuf) -> Result<Self> {
        Ok(ron::from_str(&fs::read_to_string(path)?)?)
    }

    pub fn create(file: &File, source_file: &PathBuf, target_dir: &PathBuf) -> Result<Self> {
        let target_file = target_dir.join(common::file_name(source_file)?);

        let contents = fs::read_to_string(source_file)?;
        let clean_text = clean(&contents);

        let mut tokenizer = tokenizer::Tokenizer::new()?;

        let mut lines: Vec<Line> = Vec::new();
        for text in clean_text.iter() {
            let tokens = tokenizer.tokenize(text)?;
            let line = Line::new(text, &tokens);

            lines.push(line);
        }

        Ok(Article {
            file: file.to_owned(),
            lines,
        })
    }
}

fn clean(text: &str) -> Vec<String> {
    let lines = text.lines();

    lines
        .map(|x| x.chars().filter(|c| !c.is_whitespace()).collect())
        .filter(|x: &String| !x.is_empty())
        .collect()
}
