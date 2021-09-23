use anyhow::Result;
use data_types::token::Token;
use database::Database;
use druid::widget::{Align, Controller, CrossAxisAlignment, Flex, Label, MainAxisAlignment};
use druid::{
    AppDelegate, AppLauncher, Color, Command, Data, DelegateCtx, Env, Event, EventCtx,
    FontDescriptor, FontFamily, Handled, Key, Lens, LocalizedString, MenuDesc, MenuItem, RawMods,
    Selector, Target, Widget, WidgetExt, WindowDesc, WindowId,
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
    current_line: usize,
    current_index: usize,

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
            Handled::Yes
        } else if cmd.is(SET_KNOWN) {
            println!("Set Known");
            Handled::Yes
        } else {
            Handled::No
        }
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
    let current_index = 0;
    let current_line = 0;
    let initial_state = ApplicationState {
        line_start: "".into(),
        line_middle: article.lines[current_line].tokens[current_index]
            .text
            .clone(),
        line_end: article.lines[current_line].tokens[current_index..]
            .iter()
            .map(|x| x.text.clone())
            .collect::<Vec<String>>()
            .join("")
            .into(),
        current_index: current_index,
        current_line: current_line,
        font: None,
        article: article,
    };

    launch_app(initial_state)?;

    Ok(())
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
    let secondary_font = FontDescriptor::new(noto_cjk.clone()).with_size(48.0);

    // create the labels
    let left_label =
        Label::new(|data: &ApplicationState, _env: &Env| format!("{}", data.line_start))
            .with_font(secondary_font.clone())
            .with_text_color(BACKGROUND_TEXT_COLOR);
    let center_label =
        Label::new(|data: &ApplicationState, _env: &Env| format!("{}", data.line_middle))
            .with_font(primary_font.clone());
    let right_label =
        Label::new(|data: &ApplicationState, _env: &Env| format!("{}", data.line_end))
            .with_font(secondary_font.clone())
            .with_text_color(BACKGROUND_TEXT_COLOR);

    let layout = Flex::row()
        .must_fill_main_axis(true)
        .main_axis_alignment(MainAxisAlignment::Center)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_flex_child(WidgetExt::expand_width(Align::right(left_label)), 1.0)
        .with_spacer(HORIZONTAL_WIDGET_SPACING)
        .with_flex_child(WidgetExt::expand_width(Align::centered(center_label)), 1.0)
        .with_spacer(HORIZONTAL_WIDGET_SPACING)
        .with_flex_child(WidgetExt::expand_width(Align::left(right_label)), 1.0);

    // center the two widgets in the available space
    Align::centered(layout)
}
