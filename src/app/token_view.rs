use super::{LEARNED, COPY, HORIZONTAL_WIDGET_SPACING, VERTICAL_WIDGET_SPACING};
use crate::{ApplicationState, Filter, Sort, Token, TokenInfo, TokenState};
use druid::widget::{Button, Checkbox, Controller, Flex, Label, List, Scroll};
use druid::{
    Command, Env, EventCtx, FontDescriptor, FontFamily, Insets, LensExt, Target, UpdateCtx, Widget,
    WidgetExt,
};
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

    let filter_label = Label::new(|data: &ApplicationState, _env: &Env| {
        let filter = &data.token_state.filter;

        match filter {
            Filter::All => "All".to_string(),
            Filter::Learned => "Learned".to_string(),
            Filter::Unlearned => "Unlearned".to_string(),
        }
    })
    .with_font(data_font.clone());

    let filter_button = Button::from_label(filter_label).on_click(
        |_ctx: &mut EventCtx, data: &mut ApplicationState, _env: &Env| {
            let database = data.database.borrow_mut();

            let filter = filter_type(&data.token_state.filter);
            data.token_state = database.tokens(&filter).expect("failed to load tokens");

            data.token_state.tokens =
                filter_info(data.token_state.tokens.to_vec(), &data.token_state.filter);
        },
    );

    let header = Flex::row()
        .with_child(header_label)
        .with_flex_spacer(1.0)
        .with_child(unknown_button)
        .with_child(percent_button)
        .with_child(total_button)
        .with_child(filter_button)
        .expand_width();

    fn create_row(font: FontDescriptor) -> Flex<TokenInfo> {
        let lemma_label = Label::new(|info: &TokenInfo, _env: &Env| info.lemma())
            .with_font(font.clone())
            .with_line_break_mode(druid::widget::LineBreaking::Clip)
            .fix_width(400.);

        // let total_label = Label::new(|info: &TokenInfo, _env: &Env| info.total_seen().to_string())
        //     .with_font(font.clone())
        //     .with_line_break_mode(druid::widget::LineBreaking::Clip)
        //     .fix_width(100.);

        let unknown_label =
            Label::new(|info: &TokenInfo, _env: &Env| info.total_unknown().to_string())
                .with_font(font.clone())
                .with_line_break_mode(druid::widget::LineBreaking::Clip)
                .fix_width(100.);

        let percent_label =
            Label::new(|info: &TokenInfo, _env: &Env| info.percent_known().to_string())
                .with_font(font.clone())
                .with_line_break_mode(druid::widget::LineBreaking::Clip)
                .fix_width(100.);

        let learned_label = Label::new(|info: &TokenInfo, _env: &Env| match info.token.learned {
            true => "Learned".to_string(),
            false => "Not Learned".to_string(),
        })
        .with_font(font.clone())
        .with_line_break_mode(druid::widget::LineBreaking::Clip);

        let learned_button = Button::from_label(learned_label).on_click(|ctx: &mut EventCtx, info: &mut TokenInfo, _env| {
            ctx.submit_command(Command::new(LEARNED, info.history_token.token_id, Target::Auto)); 
        })
        .fix_width(200.);

        let copy_label = Label::new("Copy").with_font(font);

        let copy_button = Button::from_label(copy_label).on_click(
            |ctx: &mut EventCtx, info: &mut TokenInfo, _env| {
                ctx.submit_command(Command::new(COPY, info.lemma(), Target::Auto));
            },
        );

        Flex::row()
            .with_child(learned_button)
            .with_spacer(HORIZONTAL_WIDGET_SPACING)
            .with_child(lemma_label)
            .with_flex_spacer(1.0)
            .with_child(unknown_label)
            .with_child(percent_label)
            .with_child(copy_button)
    }

    let list = Scroll::new(List::new(move || create_row(data_font.clone())))
        .vertical()
        .align_left()
        .lens(ApplicationState::token_state.then(TokenState::tokens));

    let view = Flex::column()
        .with_child(header)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_flex_child(list, 1.0);

    WidgetExt::center(view)
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

fn filter_info(info: Vec<TokenInfo>, filter: &Filter) -> Arc<Vec<TokenInfo>> {
    let mut info = info;

    match *filter {
        Filter::All => Arc::new(info.to_vec()),
        Filter::Learned => Arc::new(
            info.into_iter()
                .filter(|info| info.token.learned == true)
                .collect::<Vec<TokenInfo>>()
                .to_vec(),
        ),
        Filter::Unlearned => Arc::new(
            info.into_iter()
                .filter(|info| info.token.learned != true)
                .collect::<Vec<TokenInfo>>()
                .to_vec(),
        ),
    }
}

fn filter_type(filter: &Filter) -> Filter {
    match filter {
        Filter::All => Filter::Unlearned,
        Filter::Unlearned => Filter::Learned,
        Filter::Learned => Filter::All,
    }
}

fn reverse(b: bool) -> bool {
    match b {
        true => false,
        false => true,
    }
}
