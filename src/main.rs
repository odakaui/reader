use anyhow::{anyhow, Result};
use app::{launch_app, ApplicationState};
use article::{Article, Line, Position};
use database::Database;
use std::fs;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use token::{Token, POS};
use tokenizer::Tokenizer;
use ron;

pub mod app;
pub mod article;
pub mod compressor;
pub mod database;
pub mod file;
pub mod reader;
pub mod token;
pub mod tokenizer;

fn read_file(path: &Path) -> Result<String> {
    let f = fs::File::open(path)?;
    let mut buf = BufReader::new(f);
    let mut contents = String::new();
    buf.read_to_string(&mut contents)?;

    Ok(contents)
}

fn write_file(path: &Path, text: &str) -> Result<()> {
    fs::write(path, text)?;

    Ok(())
}

fn create_dir(path: &Path) -> Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)?;
    }

    Ok(())
}

fn clean(text: &str) -> Vec<String> {
    let lines = text.lines();

    lines
        .map(|x| x.chars().filter(|c| !c.is_whitespace()).collect())
        .filter(|x: &String| !x.is_empty())
        .collect()
}

fn file_stem(path: &Path) -> Result<String> {
    Ok(path
        .file_stem()
        .ok_or(anyhow!("Failed to parse file name."))?
        .to_str()
        .ok_or(anyhow!("Failed to convert file name."))?
        .to_string())
}

fn file_name(path: &Path) -> Result<String> {
    Ok(path
        .file_name()
        .ok_or(anyhow!("Failed to parse file name."))?
        .to_str()
        .ok_or(anyhow!("Failed to convert file name."))?
        .to_string())
}

// Import a file into the share folder. Will overwrite any files with the same name.
fn import(db: &mut Database, import_dir: &Path, file: &Path) -> Result<()> {
    // create share directory
    create_dir(import_dir)?;

    // add the file to the database
    let name = file_name(file)?;
    let f = file::File {
        id: None,
        name: name.clone(),
        eof: false,
    };

    db.insert_file(&f)?;

    // add the file to the file folder
    let contents = read_file(file)?;
    let clean_lines = clean(&contents);
    let import_path = import_dir.join(&name);
    let mut tokenizer = Tokenizer::new()?;


    dbg!(&clean_lines);

    let mut tokenized_lines: Vec<Line> = Vec::new();
    for x in clean_lines.iter() {
        let tokens = tokenizer.tokenize(x)?;
        let line = Line {
            sentence: x.into(),
            tokens,
        };

        tokenized_lines.push(line);
    }

    let article = Article::new(&name, &tokenized_lines);
    
    fs::write(import_path, ron::to_string(&article)?)?;

    Ok(())
}

fn open(import_dir: &Path, name: &str) -> Result<Article> {
    let path = import_dir.join(name);

    if !path.exists() {
        return Err(anyhow!("The file {} does not exist.", name));
    }

    let article = ron::from_str(&String::from_utf8(fs::read(path)?)?)?;

    Ok(article)
}

pub fn main() -> Result<()> {
    let resources = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources");

    let share = resources.join("share");
    let db_path = share.join("reader.db");
    let imported_dir = share.join("imported");
    let test_file = resources.join("japanese.txt");

    let mut db = Database::new(&db_path)?;

    import(&mut db, &imported_dir, &test_file)?; 

    let name = file_name(&test_file)?;
    let article = open(&imported_dir, &name)?;

    println!("{:?}", article);

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
