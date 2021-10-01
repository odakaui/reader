use crate::{Article, Position};
use anyhow::Result;
use druid::widget::{
    Align, ClipBox, Controller, CrossAxisAlignment, Flex, Label, LineBreaking, MainAxisAlignment,
};
use druid::{
    AppDelegate, AppLauncher, Color, Command, Data, DelegateCtx, Env, FontDescriptor, FontFamily,
    Handled, Key, Lens, LocalizedString, MenuDesc, MenuItem, RawMods, Rect, Selector, Target,
    TextAlignment, UpdateCtx, Widget, WidgetExt, WindowDesc,
};

const HORIZONTAL_WIDGET_SPACING: f64 = 64.0;
const BACKGROUND_TEXT_COLOR: Key<Color> = Key::new("background-text-color");
const WINDOW_TITLE: LocalizedString<ApplicationState> = LocalizedString::new("Reader");
const SET_UNKNOWN: Selector<()> = Selector::new("set_unknown");
const SET_KNOWN: Selector<()> = Selector::new("set_known");

#[derive(Clone, Data, Lens)]
pub struct ApplicationState {
    pub line_start: String,
    pub line_middle: String,
    pub line_end: String,
    pub font: Option<FontFamily>,
    pub position: Position,

    #[data(ignore)]
    pub article: Article,
}

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
        if cmd.is(SET_UNKNOWN) {
            println!("Set Unknown");

            Self::update_application_state(data);

            Handled::Yes
        } else if cmd.is(SET_KNOWN) {
            println!("Set Known");

            Self::update_application_state(data);

            Handled::Yes
        } else {
            Handled::No
        }
    }
    
}

impl Delegate {
    fn update_application_state(data: &mut ApplicationState) {
            let next_position = data.article.next_position(&data.position);

            match next_position {
                Some(p) => {
                    data.line_start = data.article.calculate_start(&p);
                    data.line_middle = data.article.calculate_middle(&p);
                    data.line_end = data.article.calculate_end(&p);

                    data.position = p
                }
                None => println!("EOF"),
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

        let rect = Rect::new(
            label_size.width - viewport_size.width,
            0.0,
            label_size.width,
            label_size.height,
        );

        child.pan_to_visible(rect);

        child.update(ctx, old_data, data, env);
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
