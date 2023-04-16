use super::logged_user;
use crate::{app, i18n};
use zoon::*;

pub fn root() -> impl Element {
    Column::new()
        .s(Padding::new().top(10))
        .s(Align::center())
        .item_signal(app::pages().signal_cloned().map(|page| match page {
            app::Pages::Home => home().into_raw_element(),
            app::Pages::Login => super::login::login_page().into_raw_element(),
            super::Pages::Signin => super::signin::signin_page().into_raw_element(), //_ => Label::new().label(format!("{:?}", page)).into_raw_element()
        }))
}

fn home() -> impl Element {
    Column::new()
        //.s(Align::center())
        .item_signal(logged_user().map(|user| {
            match user {
                Some(_) => super::school::school_page().into_raw_element(),
                None => Column::new()
                    .item(Label::new().s(Align::new().center_x()).label("Libredu"))
                    .item(
                        Label::new()
                            .s(Padding::new().top(10))
                            .label_signal(i18n::t!("libredu-information")),
                    )
                    .into_raw_element(),
            }
        }))
}