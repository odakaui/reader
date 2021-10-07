use crate::{compressor, reader, ApplicationState, Article, Operation, State};
use anyhow::{anyhow, Result};
use druid::widget::{
    Align, ClipBox, Controller, CrossAxisAlignment, Flex, Label, LineBreaking, MainAxisAlignment,
};
use druid::{
    AppDelegate, AppLauncher, Color, Command, Data, DelegateCtx, Env, FontDescriptor, FontFamily,
    Handled, Key, Lens, LocalizedString, MenuDesc, MenuItem, Point, RawMods, Selector, Target,
    TextAlignment, UpdateCtx, Widget, WidgetExt, WindowDesc,
};
use right_aligned_label::RightAlignedLabel;

mod right_aligned_label;

const HORIZONTAL_WIDGET_SPACING: f64 = 64.0;
const BACKGROUND_TEXT_COLOR: Key<Color> = Key::new("background-text-color");
const WINDOW_TITLE: LocalizedString<ApplicationState> = LocalizedString::new("Reader");
const MARK_UNKNOWN: Selector<()> = Selector::new("MARK_UNKNOWN");
const MARK_KNOWN: Selector<()> = Selector::new("MARK_KNOWN");
const UNDO: Selector<()> = Selector::new("undo");
const REDO: Selector<()> = Selector::new("redo");

struct Delegate;

impl AppDelegate<ApplicationState> for Delegate {
    fn command(
        &mut self,
        _: &mut DelegateCtx<'_>,
        _: Target,
        cmd: &Command,
        data: &mut ApplicationState,
        _: &Env,
    ) -> Handled {
        if cmd.is(MARK_UNKNOWN) {
            println!("Mark Unknown");

            self.add_tokens(data, Operation::MarkUnknown)
                .expect("Mark Unknown failed.");

            Handled::Yes
        } else if cmd.is(MARK_KNOWN) {
            println!("Mark Known");

            self.add_tokens(data, Operation::MarkKnown)
                .expect("Mark Known failed.");

            Handled::Yes
        } else if cmd.is(UNDO) {
            println!("Undo");

            self.undo(data).expect("[error] Undo failed.");

            Handled::Yes
        } else if cmd.is(REDO) {
            println!("Redo");

            Handled::Yes
        }

        else {
            Handled::No
        }
    }
}

impl Delegate {
    fn add_tokens(&self, data: &mut ApplicationState, action: Operation) -> Result<()> {
        let article = &data.article;
        let history = &data.history;
        let mut current_state = data
            .current_state
            .as_ref()
            .expect("Failed to unwrap current_state").clone();

        let current_position = &current_state.position;

        if current_position.is_none() {
            println!("EOF reached. Implementation TODO.");
            return Ok(());
        }

        let next_position = reader::next_position(article, &current_state);

        let file_id = article.id;
        let next_operation_num = current_state.operation_num + 1;

        // add the current word's tokens to the database
        let database = &data.database.borrow_mut();
        let current_word = compressor::compress(article, &current_state);

        match action {
            Operation::MarkKnown => database.add_tokens_known(history, current_word.tokens)?,
            Operation::MarkUnknown => database.add_tokens_unknown(history, current_word.tokens)?,
        }

        // move current_state to undo_stack
        current_state.action = Some(action);

        data.undo_stack.push(current_state.clone());
        data.current_state = Some(State {
            file_id,
            position: next_position,
            operation_num: next_operation_num,
            action: None,
        });

        dbg!("{:?}", &data.undo_stack);

        Ok(())
    }

    fn undo(&self, data: &mut ApplicationState) -> Result<()> {
        let database = data.database.borrow_mut();

        // @TODO add error handling
        if data.current_state.is_none() || data.undo_stack.is_empty() {
            println!("[warning] The undo stack is empty.");

            return Ok(())
        }

        let current_state = data.current_state.as_ref().expect("Failed to unwrap current_state");
        let previous_state = data.undo_stack.pop().expect("Failed to unwrap undo_stack");
        let history = &data.history;
        let article = &data.article;

        let word = compressor::compress(article, &previous_state);

        match previous_state.action.as_ref().expect("[error] Failed to unwrap action.") {
            Operation::MarkKnown =>  {
                database.remove_tokens_known(history, word.tokens)?;
            },
            Operation::MarkUnknown => {
                database.remove_tokens_unknown(history, word.tokens)?;
            },
        }

        data.redo_stack.push(current_state.clone());
        data.current_state = Some(previous_state);

        Ok(())
    }
}

pub fn launch_app(initial_state: ApplicationState) -> Result<()> {
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
    let left_label = RightAlignedLabel::new(
        Label::new(|data: &ApplicationState, _env: &Env| reader::start(data))
            .with_font(secondary_font.clone())
            .with_text_color(BACKGROUND_TEXT_COLOR),
    );

    let center_label = Label::new(|data: &ApplicationState, _env: &Env| reader::middle(data))
        .with_font(primary_font);

    let right_label = Label::new(|data: &ApplicationState, _env: &Env| reader::end(data))
        .with_font(secondary_font)
        .with_text_color(BACKGROUND_TEXT_COLOR);

    let layout = Flex::row()
        .must_fill_main_axis(true)
        // .main_axis_alignment(MainAxisAlignment::Center)
        // .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_flex_child(WidgetExt::expand_width(left_label), 1.0)
        .with_spacer(HORIZONTAL_WIDGET_SPACING)
        .with_flex_child(WidgetExt::expand_width(Align::centered(center_label)), 1.0)
        .with_spacer(HORIZONTAL_WIDGET_SPACING)
        .with_flex_child(WidgetExt::expand_width(Align::left(right_label)), 1.0);

    // center the two widgets in the available space
    Align::centered(layout)
}
