use anyhow::Result;
use database::Database;
use std::path::Path;

pub mod data_types;
pub mod database;
pub mod tokenizer;

use druid::piet::Text;
use druid::widget::{Align, Controller, CrossAxisAlignment, Flex, Label, MainAxisAlignment};
use druid::{
    AppLauncher, Color, Data, Env, Event, EventCtx, FontDescriptor, FontFamily, Key, Lens,
    LocalizedString, Widget, WidgetExt, WindowDesc,
};

const HORIZONTAL_WIDGET_SPACING: f64 = 64.0;
const BACKGROUND_TEXT_COLOR: Key<Color> = Key::new("background-text-color");
const WINDOW_TITLE: LocalizedString<ApplicationState> = LocalizedString::new("Reader");

#[derive(Clone, Data, Lens)]
struct ApplicationState {
    left: String,
    center: String,
    right: String,
    font: Option<FontFamily>,
}

pub fn main() -> Result<()> {
    launch_app()?;

    Ok(())
}

fn launch_app() -> Result<()> {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget)
        .title(WINDOW_TITLE)
        .window_size((1000.0, 800.0));

    // create the initial app state
    let initial_state = ApplicationState {
        left: "わたしは".into(),
        center: "にほんごが".into(),
        right: "すこししかはなせません。".into(),
        font: None,
    };

    // start the application
    AppLauncher::with_window(main_window)
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
    let left_label = Label::new(|data: &ApplicationState, _env: &Env| format!("{}", data.left))
        .with_font(secondary_font.clone())
        .with_text_color(BACKGROUND_TEXT_COLOR);
    let center_label = Label::new(|data: &ApplicationState, _env: &Env| format!("{}", data.center))
        .with_font(primary_font.clone());
    let right_label = Label::new(|data: &ApplicationState, _env: &Env| format!("{}", data.right))
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
