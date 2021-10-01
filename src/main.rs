use anyhow::Result;
use data_types::token::Token;
use database::Database;
use druid::widget::{
    Align, ClipBox, Controller, CrossAxisAlignment, Flex, Label, LineBreaking, MainAxisAlignment,
};
use druid::{
    AppDelegate, AppLauncher, Color, Command, Data, DelegateCtx, Env, Event, EventCtx,
    FontDescriptor, FontFamily, Handled, Key, Lens, LocalizedString, MenuDesc, MenuItem, Point,
    RawMods, Rect, Selector, Target, TextAlignment, UpdateCtx, Widget, WidgetExt, WindowDesc,
    WindowId,
};
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use tokenizer::Tokenizer;

pub mod data_types;
pub mod database;
pub mod tokenizer;

const HORIZONTAL_WIDGET_SPACING: f64 = 64.0;
const BACKGROUND_TEXT_COLOR: Key<Color> = Key::new("background-text-color");
const WINDOW_TITLE: LocalizedString<ApplicationState> = LocalizedString::new("Reader");
const SET_UNKNOWN: Selector<()> = Selector::new("set_unknown");
const SET_KNOWN: Selector<()> = Selector::new("set_known");

#[derive(Clone, Data, Lens)]
struct ApplicationState {
    line_start: String,
    line_middle: String,
    line_end: String,
    font: Option<FontFamily>,
    position: Position,

    #[data(ignore)]
    article: Article,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Article {
    file_name: String,
    lines: Vec<Line>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Line {
    sentence: String,
    tokens: Vec<Token>,
}

#[derive(Clone, Debug, Data, Lens)]
struct Position {
    index: usize,
    line: usize,
}

struct Delegate;

impl AppDelegate<ApplicationState> for Delegate {
    fn command(
        &mut self,
        ctx: &mut DelegateCtx<'_>,
        target: Target,
        cmd: &Command,
        data: &mut ApplicationState,
        env: &Env,
    ) -> Handled {
        if cmd.is(SET_UNKNOWN) {
            println!("Set Unknown");

            let next_position = next_position(&data.article, &data.position);

            match next_position {
                Some(p) => {
                    let a = &data.article;

                    data.line_start = calculate_start(a, &p);
                    data.line_middle = calculate_middle(a, &p);
                    data.line_end = calculate_end(a, &p);

                    data.position = p
                }
                None => println!("EOF"),
            }

            Handled::Yes
        } else if cmd.is(SET_KNOWN) {
            println!("Set Known");

            let next_position = next_position(&data.article, &data.position);

            match next_position {
                Some(p) => {
                    let a = &data.article;

                    data.line_start = calculate_start(a, &p);
                    data.line_middle = calculate_middle(a, &p);
                    data.line_end = calculate_end(a, &p);

                    data.position = p
                }
                None => println!("EOF"),
            }

            Handled::Yes
        } else {
            Handled::No
        }
    }
}

struct RightJustifiedController;

impl<W: Widget<ApplicationState>> Controller<ApplicationState, ClipBox<ApplicationState, W>>
    for RightJustifiedController
{
    fn update(
        &mut self,
        child: &mut ClipBox<ApplicationState, W>,
        ctx: &mut UpdateCtx<'_, '_>,
        old_data: &ApplicationState,
        data: &ApplicationState,
        env: &Env,
    ) {
        let label_size = child.content_size();
        let viewport_size = child.viewport_size();

        println!("{:?}", child.viewport());
        println!("{:?}", child.viewport_origin());
        println!("{:?}, {:?}", viewport_size, label_size);

        let rect = Rect::new(
            label_size.width - viewport_size.width,
            0.0,
            label_size.width,
            label_size.height,
        );
        let changed = child.pan_to_visible(rect);

        println!("{}", changed);

        child.update(ctx, old_data, data, env);
    }
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
            tokens: tokens,
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
        line_start: calculate_start(&article, &position),
        line_middle: calculate_middle(&article, &position),
        line_end: calculate_end(&article, &position),
        position: position,
        font: None,
        article: article,
    };

    launch_app(initial_state)?;

    Ok(())
}

// calculate the String to display in the left Label of the reader view
fn calculate_start(article: &Article, position: &Position) -> String {
    if position.index == 0 {
        "".to_string()
    } else {
        let tokens = article.lines[position.line].tokens[..position.index].to_vec();
        tokens
            .iter()
            .map(|x| x.text.to_string())
            .collect::<Vec<String>>()
            .join("")
    }
}

// calculate the String to display in the center Label of the reader view
fn calculate_middle(article: &Article, position: &Position) -> String {
    article.lines[position.line].tokens[position.index]
        .text
        .to_string()
}

// calculate the String to display in the right Label of the reader view
fn calculate_end(article: &Article, position: &Position) -> String {
    let tokens = article.lines[position.line].tokens.clone();

    if position.index >= tokens.len() {
        "".to_string()
    } else {
        let slice = tokens[position.index + 1..].to_vec();
        slice
            .iter()
            .map(|x| x.text.to_string())
            .collect::<Vec<String>>()
            .join("")
    }
}

// calculate the next available index
// returns None if the next position is past the end of the file
fn next_position(article: &Article, current_position: &Position) -> Option<Position> {
    let article_length = article.lines.len();
    let line_length = article.lines[current_position.line].tokens.len();

    let mut is_eof = false;

    let new_index: usize;
    let new_line: usize;

    if current_position.index + 1 >= line_length {
        new_index = 0;

        if current_position.line + 1 >= article_length {
            new_line = 0;
            is_eof = true;
        } else {
            new_line = current_position.line + 1;
        }
    } else {
        new_index = current_position.index + 1;
        new_line = current_position.line;
    }

    if is_eof {
        None
    } else {
        Some(Position {
            index: new_index,
            line: new_line,
        })
    }
}

fn read_file(path: &Path) -> Result<String> {
    let f = File::open(path)?;
    let mut buf = BufReader::new(f);
    let mut contents = String::new();
    buf.read_to_string(&mut contents)?;

    Ok(contents)
}

fn launch_app(initial_state: ApplicationState) -> Result<()> {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget)
        .title(WINDOW_TITLE)
        .menu(
            MenuDesc::empty()
                .append(
                    MenuDesc::new(LocalizedString::new("File")).append(
                        MenuItem::new(
                            LocalizedString::new("open"),
                            Command::new(Selector::new("open"), 1, Target::Auto),
                        )
                        .hotkey(RawMods::Ctrl, "o"),
                    ),
                )
                .append(
                    MenuDesc::new(LocalizedString::new("Reader"))
                        .append(
                            MenuItem::new(
                                LocalizedString::new("Mark Unknown"),
                                Command::new(SET_UNKNOWN, (), Target::Auto),
                            )
                            .hotkey(None, "d"),
                        )
                        .append(
                            MenuItem::new(
                                LocalizedString::new("Mark Known"),
                                Command::new(SET_KNOWN, (), Target::Auto),
                            )
                            .hotkey(None, "f"),
                        ),
                ),
        )
        .window_size((1000.0, 800.0));

    // start the application
    AppLauncher::with_window(main_window)
        .delegate(Delegate)
        .configure_env(|env, _state| {
            env.set(
                BACKGROUND_TEXT_COLOR,
                Color::from_hex_str("#3C3C3C").unwrap(),
            );
        })
        .launch(initial_state)?;

    Ok(())
}

fn build_root_widget() -> impl Widget<ApplicationState> {
    // set up the fonts - requires that Noto Sans CJK JP is installed
    let noto_cjk = FontFamily::new_unchecked("Noto Sans CJK JP");
    let primary_font = FontDescriptor::new(noto_cjk.clone()).with_size(64.0);
    let secondary_font = FontDescriptor::new(noto_cjk).with_size(48.0);

    // create the labels
    let left_label = Align::right(
        ClipBox::new(
            Label::new(|data: &ApplicationState, _env: &Env| data.line_start.to_string())
                .with_font(secondary_font.clone())
                .with_text_color(BACKGROUND_TEXT_COLOR)
                .with_line_break_mode(LineBreaking::Clip)
                .with_text_alignment(TextAlignment::Center),
        )
        .controller(RightJustifiedController),
    );
    let center_label =
        Label::new(|data: &ApplicationState, _env: &Env| data.line_middle.to_string())
            .with_font(primary_font)
            .with_text_alignment(TextAlignment::Center);
    let right_label = Label::new(|data: &ApplicationState, _env: &Env| data.line_end.to_string())
        .with_font(secondary_font)
        .with_text_color(BACKGROUND_TEXT_COLOR)
        .with_line_break_mode(LineBreaking::Clip)
        .with_text_alignment(TextAlignment::End);

    let layout = Flex::row()
        .must_fill_main_axis(true)
        .main_axis_alignment(MainAxisAlignment::Center)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_flex_child(WidgetExt::expand_width(left_label), 1.0)
        .with_spacer(HORIZONTAL_WIDGET_SPACING)
        .with_flex_child(WidgetExt::expand_width(Align::centered(center_label)), 1.0)
        .with_spacer(HORIZONTAL_WIDGET_SPACING)
        .with_flex_child(WidgetExt::expand_width(Align::left(right_label)), 1.0);

    // center the two widgets in the available space
    Align::centered(layout)
}
