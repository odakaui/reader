use crate::{ApplicationState, Status, View};
use anyhow::{anyhow, Result};
use delegate::Delegate;
use druid::widget::{
    Align, ClipBox, Controller, CrossAxisAlignment, Flex, Label, LineBreaking, MainAxisAlignment,
    ViewSwitcher,
};
use druid::{
    AppDelegate, AppLauncher, Color, Command, Data, DelegateCtx, Env, FileDialogOptions, FileSpec,
    FontDescriptor, FontFamily, Handled, Key, Lens, LocalizedString, MenuDesc, MenuItem, Point,
    RawMods, Selector, Target, TextAlignment, UpdateCtx, Widget, WidgetExt, WindowDesc,
};
use right_aligned_label::RightAlignedLabel;
use std::boxed::Box;

mod delegate;
mod empty_view;
mod eof_view;
mod reader_view;
mod right_aligned_label;

const HORIZONTAL_WIDGET_SPACING: f64 = 64.0;
const VERTICAL_WIDGET_SPACING: f64 = 36.0;
const BACKGROUND_TEXT_COLOR: Key<Color> = Key::new("background-text-color");
const WINDOW_TITLE: LocalizedString<ApplicationState> = LocalizedString::new("Reader");

const MARK_UNKNOWN: Selector<()> = Selector::new("MARK_UNKNOWN");
const MARK_KNOWN: Selector<()> = Selector::new("MARK_KNOWN");
const UNDO: Selector<()> = Selector::new("UNDO");
const REDO: Selector<()> = Selector::new("REDO");
const READER: Selector<()> = Selector::new("READER");
const STATISTICS: Selector<()> = Selector::new("STATISTICS");

pub fn launch_app(initial_state: ApplicationState) -> Result<()> {
    // create the open file dialogue
    let txt = FileSpec::new("Text file", &["txt"]);
    let open_dialog_options = FileDialogOptions::new()
        .allowed_types(vec![txt])
        .default_type(txt)
        .name_label("Source")
        .title("Import file")
        .button_text("Import");

    // describe the main window
    let main_window = WindowDesc::new(build_root_widget)
        .title(WINDOW_TITLE)
        .menu(
            MenuDesc::empty()
                .append(
                    MenuDesc::new(LocalizedString::new("File")).append(
                        MenuItem::new(
                            LocalizedString::new("Open"),
                            druid::commands::SHOW_OPEN_PANEL.with(open_dialog_options),
                        )
                        .hotkey(RawMods::Ctrl, "o"),
                    ),
                )
                .append(
                    MenuDesc::new(LocalizedString::new("Edit"))
                        .append(
                            MenuItem::new(
                                LocalizedString::new("Undo"),
                                Command::new(UNDO, (), Target::Auto),
                            )
                            .hotkey(RawMods::Ctrl, "z"),
                        )
                        .append(
                            MenuItem::new(
                                LocalizedString::new("Redo"),
                                Command::new(REDO, (), Target::Auto),
                            )
                            .hotkey(RawMods::Ctrl, "y"),
                        ),
                )
                .append(
                    MenuDesc::new(LocalizedString::new("Reader"))
                        .append(
                            MenuItem::new(
                                LocalizedString::new("Mark Unknown"),
                                Command::new(MARK_UNKNOWN, (), Target::Auto),
                            )
                            .hotkey(None, "d"),
                        )
                        .append(
                            MenuItem::new(
                                LocalizedString::new("Mark Known"),
                                Command::new(MARK_KNOWN, (), Target::Auto),
                            )
                            .hotkey(None, "f"),
                        ),
                )
                .append(
                    MenuDesc::new(LocalizedString::new("View"))
                        .append(
                            MenuItem::new(
                                LocalizedString::new("Reader"),
                                Command::new(READER, (), Target::Auto),
                            )
                            .hotkey(None, "r"),
                        )
                        .append(
                            MenuItem::new(
                                LocalizedString::new("Reader"),
                                Command::new(STATISTICS, (), Target::Auto),
                            )
                            .hotkey(None, "s"),
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
    let switch_view = ViewSwitcher::new(
        |data: &ApplicationState, _: &Env| -> View {
            let mut current_view = data.current_view.clone();

            if current_view == View::Reader {
                match data.reader_state.status {
                    Status::Empty => current_view = View::Empty,
                    Status::Eof => current_view = View::Eof,
                    Status::State => {}
                }
            }

            current_view
        },
        |current_view: &View, _: &ApplicationState, _: &Env| match current_view {
            View::Reader => Box::new(reader_view::build_reader_view()),
            View::Empty => Box::new(empty_view::build_empty_view()),
            View::Statistics => Box::new(empty_view::build_empty_view()),
            View::Eof => Box::new(eof_view::build_empty_view()),
        },
    );

    WidgetExt::center(switch_view)
}
