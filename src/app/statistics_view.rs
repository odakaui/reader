use super::VERTICAL_WIDGET_SPACING;
use crate::{ApplicationState, StatisticsState, TokenInfo};
use druid::widget::{Container, Either, Flex, Label, List, Scroll};
use druid::{Env, FontDescriptor, FontFamily, Insets, Widget, WidgetExt};

pub fn build_statistics_view() -> impl Widget<ApplicationState> {
    let either = Either::new(
        |data: &ApplicationState, _: &Env| -> bool { data.statistics_state.is_none() },
        build_none_view(),
        build_data_view(),
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

    let header_label = Label::new(|data: &Option<StatisticsState>, _: &Env| {
        if data.is_none() {
            return "".to_string();
        }

        format!("Statistics for {}", data.as_ref().unwrap().file_name)
    })
    .with_font(primary_font)
    .lens(ApplicationState::statistics_state);

    let start_label = Label::new(|data: &Option<StatisticsState>, _: &Env| {
        if data.is_none() {
            return "".to_string();
        }

        format!("Start date: {}", data.as_ref().unwrap().start_date)
    })
    .with_font(data_font.clone())
    .align_left()
    .lens(ApplicationState::statistics_state);

    let end_label = Label::new(|data: &Option<StatisticsState>, _: &Env| {
        if data.is_none() {
            return "".to_string();
        }

        format!("End date: {}", data.as_ref().unwrap().start_date)
    })
    .with_font(data_font.clone())
    .align_left()
    .lens(ApplicationState::statistics_state);

    let total_label = Label::new(|data: &Option<StatisticsState>, _: &Env| {
        if data.is_none() {
            return "".to_string();
        }

        format!("Total tokens seen: {}", data.as_ref().unwrap().total_seen)
    })
    .with_font(data_font.clone())
    .align_left()
    .lens(ApplicationState::statistics_state);

    let known_label = Label::new(|data: &Option<StatisticsState>, _: &Env| {
        if data.is_none() {
            return "".to_string();
        }

        let data = data.as_ref().unwrap();

        let total_known = data.total_seen - data.total_unknown;
        let percentage = total_known as f64 / data.total_seen as f64 * 100.0;
        format!("Percent known: {}%", percentage.round())
    })
    .with_font(data_font.clone())
    .align_left()
    .lens(ApplicationState::statistics_state);

    let unknown_list = Scroll::new(List::new(move || {
        Label::new(|token_info: &TokenInfo, _: &Env| format_unknown(token_info))
            .with_font(data_font.clone())
            .align_left()
    }))
    .align_left()
    .lens(ApplicationState::unknown_tokens);

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
        .with_flex_spacer(1.0)
}

fn format_unknown(token_info: &TokenInfo) -> String {
    let token = &token_info.token;

    format!("{}, {}", token_info.total_unknown, token.lemma,)
}
