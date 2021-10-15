use super::VERTICAL_WIDGET_SPACING;
use crate::{ApplicationState, Filter, Sort, StatisticsState, Status, TokenInfo, TokenState};
use druid::widget::{Button, Flex, Label, List, Scroll};
use druid::{Env, FontDescriptor, FontFamily, Insets, LensExt, Widget, WidgetExt};
use std::cmp::Reverse;
use std::sync::Arc;

pub fn build_token_view() -> impl Widget<ApplicationState> {
    let noto_cjk = FontFamily::new_unchecked("Noto Sans CJK JP");

    let primary_font = FontDescriptor::new(noto_cjk.clone()).with_size(64.0);
    let data_font = FontDescriptor::new(noto_cjk).with_size(24.0);

    let header_label = Label::new("Tokens").with_font(primary_font);

    let unknown_label: Label<ApplicationState> = Label::new("Unknown").with_font(data_font.clone());

    let unknown_button =
        Button::from_label(unknown_label).on_click(|_ctx, data: &mut ApplicationState, _env| {
            if data.token_state.sort == Sort::Unknown {
                data.token_state.reverse = reverse(data.token_state.reverse);
            }

            data.token_state.tokens = sort_info(
                data.token_state.tokens.to_vec(),
                &Sort::Unknown,
                data.token_state.reverse,
            );

            data.token_state.sort = Sort::Unknown;
        });

    let percent_label = Label::new("Percent").with_font(data_font.clone());

    let percent_button =
        Button::from_label(percent_label).on_click(|_ctx, data: &mut ApplicationState, _env| {
            if data.token_state.sort == Sort::Percent {
                data.token_state.reverse = reverse(data.token_state.reverse);
            }

            data.token_state.tokens = sort_info(
                data.token_state.tokens.to_vec(),
                &Sort::Percent,
                data.token_state.reverse,
            );

            data.token_state.sort = Sort::Percent;
        });

    let total_label = Label::new("Total").with_font(data_font.clone());

    let total_button =
        Button::from_label(total_label).on_click(|_ctx, data: &mut ApplicationState, _env| {
            if data.token_state.sort == Sort::Total {
                data.token_state.reverse = reverse(data.token_state.reverse);
            }

            data.token_state.tokens = sort_info(
                data.token_state.tokens.to_vec(),
                &Sort::Total,
                data.token_state.reverse,
            );

            data.token_state.sort = Sort::Total;
        });

    let filter_label = Label::new(|filter: &Filter, _: &Env| match filter {
        Filter::All => "All".to_string(),
        Filter::Learned => "Learned".to_string(),
        Filter::Unlearned => "Unlearned".to_string(),
    })
    .with_font(data_font.clone());

    let filter_button = Button::from_label(filter_label)
        .lens(ApplicationState::token_state.then(TokenState::filter));

    let header = Flex::row()
        .with_child(header_label)
        .with_flex_spacer(1.0)
        .with_child(unknown_button)
        .with_child(percent_button)
        .with_child(total_button)
        .with_child(filter_button)
        .expand_width();

    let list = Scroll::new(List::new(move || {
        Label::new(|info: &TokenInfo, _: &Env| format_info(info))
            .with_font(data_font.clone())
            .align_left()
    }))
    .vertical()
    .align_left()
    .lens(ApplicationState::token_state.then(TokenState::tokens));

    let view = Flex::column()
        .with_child(header)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_flex_child(list, 1.0);

    WidgetExt::center(view)
}

fn format_info(info: &TokenInfo) -> String {
    format!(
        "{}, {} (marked as known {}% of the time)",
        info.total_unknown(),
        info.lemma(),
        info.percent_known()
    )
}

fn sort_info(info: Vec<TokenInfo>, sort: &Sort, reverse: bool) -> Arc<Vec<TokenInfo>> {
    let mut info = info;

    match *sort {
        Sort::Total => {
            if reverse {
                info.sort_by_key(|token| Reverse(token.total_seen()));
            } else {
                info.sort_by_key(|token| token.total_seen());
            }
        }
        Sort::Unknown => {
            if reverse {
                info.sort_by_key(|token| Reverse(token.total_unknown()));
            } else {
                info.sort_by_key(|token| token.total_unknown());
            }

        }
        Sort::Percent => {
            if reverse {
                info.sort_by_key(|token| Reverse(token.percent_known()));
            } else {
                info.sort_by_key(|token| token.percent_known());
            }
        }
    }

    Arc::new(info.to_vec())
}

fn reverse(b: bool) -> bool {
    match b {
        true => false,
        false => true,
    }
}
