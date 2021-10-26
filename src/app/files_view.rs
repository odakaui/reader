use super::{OPEN, VERTICAL_WIDGET_SPACING};
use crate::{ApplicationState, File, FileState, Filter, Token, TokenInfo, TokenState};
use druid::widget::{Button, Checkbox, Controller, Flex, Label, List, Scroll};
use druid::{
    Command, Env, EventCtx, FontDescriptor, FontFamily, Insets, LensExt, Target, UpdateCtx, Widget,
    WidgetExt,
};
use std::cmp::Reverse;
use std::sync::Arc;

pub fn build_files_view() -> impl Widget<ApplicationState> {
    let noto_cjk = FontFamily::new_unchecked("Noto Sans CJK JP");

    let primary_font = FontDescriptor::new(noto_cjk.clone()).with_size(64.0);
    let data_font = FontDescriptor::new(noto_cjk).with_size(24.0);

    let header_label = Label::new("Files").with_font(primary_font);

    let header = Flex::row()
        .with_child(header_label)
        .with_flex_spacer(1.0)
        .expand_width();

    fn create_row(font: FontDescriptor) -> impl Widget<File> {
        let name_label =
            Label::new(|file: &File, _env: &Env| file.name.to_string()).with_font(font.clone());

        Flex::row()
            .with_child(name_label)
            .with_flex_spacer(1.0)
            .expand_width()
            .on_click(|ctx: &mut EventCtx, file: &mut File, _env| {
                ctx.submit_command(Command::new(OPEN, file.id, Target::Auto))
            })
    }

    let list = Scroll::new(List::new(move || create_row(data_font.clone())))
        .vertical()
        .align_left()
        .lens(ApplicationState::file_state.then(FileState::files));

    let view = Flex::column()
        .with_child(header)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_flex_child(list, 1.0)
        .with_spacer(VERTICAL_WIDGET_SPACING);

    WidgetExt::center(view)
}

// fn sort_info(info: Vec<TokenInfo>, sort: &Sort, reverse: bool) -> Arc<Vec<TokenInfo>> {
//     let mut info = info;

//     match *sort {
//         Sort::Total => {
//             if reverse {
//                 info.sort_by_key(|token| Reverse(token.total_seen()));
//             } else {
//                 info.sort_by_key(|token| token.total_seen());
//             }
//         }
//         Sort::Unknown => {
//             if reverse {
//                 info.sort_by_key(|token| Reverse(token.total_unknown()));
//             } else {
//                 info.sort_by_key(|token| token.total_unknown());
//             }
//         }
//         Sort::Percent => {
//             if reverse {
//                 info.sort_by_key(|token| Reverse(token.percent_known()));
//             } else {
//                 info.sort_by_key(|token| token.percent_known());
//             }
//         }
//     }

//     Arc::new(info.to_vec())
// }

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
