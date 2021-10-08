use super::{RightAlignedLabel, BACKGROUND_TEXT_COLOR, HORIZONTAL_WIDGET_SPACING};
use crate::{reader, ApplicationState, ReaderState};
use druid::widget::{Align, Flex, Label};
use druid::{Env, FontDescriptor, FontFamily, Widget, WidgetExt};

pub fn build_empty_view() -> impl Widget<ApplicationState> {
    // set up the fonts - requires that Noto Sans CJK JP is installed
    let noto_cjk = FontFamily::new_unchecked("Noto Sans CJK JP");
    let primary_font = FontDescriptor::new(noto_cjk.clone()).with_size(64.0);

    let label = Label::new("To import a new file press Ctrl + O").with_font(primary_font);

    let layout = Flex::row()
        .must_fill_main_axis(true)
        .with_flex_child(WidgetExt::center(label), 1.0);

    layout
}
