use super::VERTICAL_WIDGET_SPACING;
use crate::{ApplicationState, StatisticsState, Status, TokenInfo};
use druid::widget::{Container, Either, Flex, Label, List, Scroll};
use druid::{Env, FontDescriptor, FontFamily, Insets, LensExt, Widget, WidgetExt};

pub fn build_statistics_view() -> impl Widget<ApplicationState> {
    let either = Either::new(
        |data: &ApplicationState, _: &Env| -> bool {
            let status = &data.reader_state.status;

            status == &Status::State || status == &Status::Eof
        },
        build_data_view(),
        build_none_view(),
    );

    WidgetExt::center(either)
}

pub fn build_none_view() -> impl Widget<ApplicationState> {
    let noto_cjk = FontFamily::new_unchecked("Noto Sans CJK JP");
    let primary_font = FontDescriptor::new(noto_cjk).with_size(64.0);

    let information_label = Label::new("No Statistics Available").with_font(primary_font);

    WidgetExt::center(information_label)
}

pub fn build_data_view() -> impl Widget<ApplicationState> {
    let noto_cjk = FontFamily::new_unchecked("Noto Sans CJK JP");
    let primary_font = FontDescriptor::new(noto_cjk.clone()).with_size(64.0);
    let data_font = FontDescriptor::new(noto_cjk).with_size(24.0);

    let header_label =
        Label::new(|data: &StatisticsState, _: &Env| format!("Statistics for {}", data.name))
            .with_font(primary_font)
            .lens(ApplicationState::statistics_state);

    let start_label =
        Label::new(|data: &StatisticsState, _: &Env| format!("Started {}", data.start_date))
            .with_font(data_font.clone())
            .align_left()
            .lens(ApplicationState::statistics_state);

    let end_label = Label::new(|data: &StatisticsState, _: &Env| {
        let end_date = match data.end_date {
            Some(date) => date.to_string(),
            None => "".to_string(),
        };

        format!("Ended {}", end_date)
    })
    .with_font(data_font.clone())
    .align_left()
    .lens(ApplicationState::statistics_state);

    let total_label = Label::new(|data: &StatisticsState, _: &Env| {
        format!("You've seen {} tokens.", data.total_seen)
    })
    .with_font(data_font.clone())
    .align_left()
    .lens(ApplicationState::statistics_state);

    let known_label = Label::new(|data: &StatisticsState, _: &Env| {
        format!("And marked {}% as known.", data.percent_known())
    })
    .with_font(data_font.clone())
    .align_left()
    .lens(ApplicationState::statistics_state);

    let unknown_list = List::new(move || {
        Label::new(|info: &TokenInfo, _: &Env| format_info(info))
            .with_font(data_font.clone())
            .align_left()
    })
    .lens(ApplicationState::statistics_state.then(StatisticsState::unknown))
    .align_left();

    Scroll::new(
        Flex::column()
            .must_fill_main_axis(false)
            .with_child(header_label)
            .with_spacer(VERTICAL_WIDGET_SPACING)
            .with_child(start_label.padding(Insets::new(16., 0., 0., 0.)))
            .with_child(end_label.padding(Insets::new(16., 0., 0., 0.)))
            .with_spacer(VERTICAL_WIDGET_SPACING)
            .with_child(total_label.padding(Insets::new(16., 0., 0., 0.)))
            .with_child(known_label.padding(Insets::new(16., 0., 0., 0.)))
            .with_spacer(VERTICAL_WIDGET_SPACING)
            .with_child(unknown_list.padding(Insets::new(16., 0., 0., 0.)))
            .with_spacer(VERTICAL_WIDGET_SPACING),
    )
    .vertical()
}

fn format_info(info: &TokenInfo) -> String {
    format!(
        "{}, {} (marked as known {}% of the time)",
        info.total_unknown(),
        info.lemma(),
        info.percent_known()
    )
}
