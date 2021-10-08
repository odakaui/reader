use crate::{ApplicationState, StatisticsState, TokenInfo};
use druid::widget::{Either, Flex, Label, Scroll};
use druid::{Env, FontDescriptor, FontFamily, Widget, WidgetExt};

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
    let primary_font = FontDescriptor::new(noto_cjk.clone()).with_size(64.0);

    let information_label = Label::new("No Statistics Available");

    WidgetExt::center(information_label)
}

pub fn build_data_view() -> impl Widget<ApplicationState> {
    let noto_cjk = FontFamily::new_unchecked("Noto Sans CJK JP");
    let primary_font = FontDescriptor::new(noto_cjk.clone()).with_size(64.0);
    let secondary_font = FontDescriptor::new(noto_cjk.clone()).with_size(48.0);
    let data_font = FontDescriptor::new(noto_cjk).with_size(24.0);

    let header_label = Label::new(|data: &Option<StatisticsState>, _: &Env| {
        if data.is_none() {
            return "".to_string();
        }

        format!("Statistics for {}", data.as_ref().unwrap().file_name)
    })
    .with_font(primary_font);

    let start_label = Label::new(|data: &Option<StatisticsState>, _: &Env| {
        if data.is_none() {
            return "".to_string();
        }

        format!("Start date: {}", data.as_ref().unwrap().start_date)
    })
    .with_font(data_font.clone());

    let end_label = Label::new(|data: &Option<StatisticsState>, _: &Env| {
        if data.is_none() {
            return "".to_string();
        }

        format!("End date: {}", data.as_ref().unwrap().start_date)
    })
    .with_font(data_font.clone());

    let total_label = Label::new(|data: &Option<StatisticsState>, _: &Env| {
        if data.is_none() {
            return "".to_string();
        }

        format!("Total tokens seen: {}", data.as_ref().unwrap().total_seen)
    })
    .with_font(data_font.clone());

    let known_label = Label::new(|data: &Option<StatisticsState>, _: &Env| {
        if data.is_none() {
            return "".to_string();
        }

        let data = data.as_ref().unwrap();

        let total_known = data.total_seen - data.total_unknown;
        let percentage = total_known as f64 / data.total_seen as f64 * 100.0;
        format!("Percent known: {}", percentage.round())
    })
    .with_font(data_font.clone());

    let unknown_label = Label::new(|data: &Option<StatisticsState>, _: &Env| {
        if data.is_none() {
            return "".to_string();
        }

        format_unknown(data.as_ref().unwrap().unknown_tokens.clone())
    })
    .with_font(data_font);

    Scroll::new(
        Flex::column()
            .with_flex_child(header_label, 1.0)
            .with_flex_child(start_label, 1.0)
            .with_flex_child(end_label, 1.0)
            .with_flex_child(total_label, 1.0)
            .with_flex_child(known_label, 1.0)
            .with_flex_child(unknown_label, 1.0)
            .lens(ApplicationState::statistics_state),
    )
    .vertical()
}

fn format_unknown(token_info_list: Vec<TokenInfo>) -> String {
    let mut text = String::new();

    for (i, token_info) in token_info_list.iter().enumerate() {
        let token = &token_info.token;

        if i != 0 {
            text.push('\n');
        }

        let total_known = token_info.total_seen - token_info.total_unknown;
        let percentage = (total_known as f64 / token_info.total_seen as f64) * 100.0;

        text.push_str(&format!("{}   {:?} {}% {}", token.lemma, token.pos, percentage.round(), token_info.total_seen));
    }

    text
}
