use super::logged_user;
use crate::{app, i18n};
use zoon::*;

pub fn root() -> impl Element {
    Column::new()
        //.s(Padding::new().top(5))
        .s(Align::center())
        .item_signal(app::pages().signal_cloned().map(|page| match page {
            app::Pages::Home => home().into_raw_element(),
            app::Pages::Login => super::login::login_page().into_raw_element(),
            app::Pages::ForgetPassword => super::forget_password::root().into_raw_element(),
            app::Pages::ResetPassword => super::reset_password::reset_password().into_raw_element(),
            app::Pages::Signin => super::signin::signin_page().into_raw_element(),
            app::Pages::User => Row::new().item(
                Column::new().item(Label::new().label("This is menus page"))
            ).item(Column::new().item(Label::new().label("This is submenu page"))).into_raw_element(),
            app::Pages::Admin => super::admin::root().into_raw_element()
        }))
}

fn home() -> impl Element {
    Column::new()
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
