use super::{RightAlignedLabel, BACKGROUND_TEXT_COLOR, HORIZONTAL_WIDGET_SPACING};
use crate::{reader, ApplicationState, ReaderState};
use druid::widget::{Align, Flex, Label};
use druid::{Env, FontDescriptor, FontFamily, Widget, WidgetExt};

pub fn build_reader_view() -> impl Widget<ApplicationState> {
    // set up the fonts - requires that Noto Sans CJK JP is installed
    let noto_cjk = FontFamily::new_unchecked("Noto Sans CJK JP");
    let primary_font = FontDescriptor::new(noto_cjk.clone()).with_size(64.0);
    let secondary_font = FontDescriptor::new(noto_cjk).with_size(48.0);

    // create the labels
    let left_label = RightAlignedLabel::new(
        Label::new(|data: &Option<ReaderState>, _env: &Env| reader::start(data))
            .with_font(secondary_font.clone())
            .with_text_color(BACKGROUND_TEXT_COLOR),
    )
    .lens(ApplicationState::reader_state);

    let center_label = Label::new(|data: &Option<ReaderState>, _env: &Env| reader::middle(data))
        .with_font(primary_font)
        .lens(ApplicationState::reader_state);

    let right_label = Label::new(|data: &Option<ReaderState>, _env: &Env| reader::end(data))
        .with_font(secondary_font)
        .with_text_color(BACKGROUND_TEXT_COLOR)
        .lens(ApplicationState::reader_state);

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
