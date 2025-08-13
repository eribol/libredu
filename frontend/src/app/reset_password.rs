use crate::i18n;
use crate::router::Route;
use shared::models::users::ResetForm;
use std::borrow::Cow;
use zoon::{eprintln, *};

pub fn reset_password() -> impl Element {
    Column::new()
        .s(Align::center())
        .s(Gap::new().y(15))
        .item(
            Label::new()
                .s(Align::center())
                .label_signal(i18n::t!("login"))
                .s(Font::new().weight(FontWeight::SemiBold)),
        )
        .item(
            TextInput::new()
                .s(Align::center())
                .s(Borders::all(Border::new().solid().color(color!("BLUE"))))
                .s(Height::exact(30))
                .id("email")
                .input_type(InputType::password())
                .placeholder(Placeholder::with_signal(i18n::t!("password")))
                .on_change(|s| password().set(s)),
        )
        .item(
            TextInput::new()
                .s(Align::center())
                .s(Borders::all_signal(
                    password_error().signal_cloned().map_option(
                        |_| Border::new().solid().color(color!("red")),
                        || Border::new().solid().color(color!("blue")),
                    ),
                ))
                .s(Height::exact(30))
                .id("password")
                .input_type(InputType::password())
                .placeholder(Placeholder::with_signal(i18n::t!("password")))
                .on_change(|s| {
                    password_error().set(None);
                    password2().set(s)
                }),
        )
        .item(
            Button::new()
                .s(Height::exact(35))
                .s(RoundedCorners::all(10))
                .s(Borders::all(Border::new().solid().color(color!("BLUE"))))
                .label(El::new().s(Align::center()).child_signal(i18n::t!("login")))
                .on_click(send_mail),
        )
        .item(
            Row::new()
                .s(Gap::new().x(25))
                .item(Link::new().label("Sign in").to(Route::Signin))
                .item(" veya ")
                .item(Link::new().label("Login").to(Route::Login)),
        )
}

#[static_ref]
pub fn email() -> &'static Mutable<String> {
    Mutable::new(String::from(""))
}
#[static_ref]
pub fn token() -> &'static Mutable<String> {
    Mutable::new(String::from(""))
}

#[static_ref]
fn password() -> &'static Mutable<String> {
    Mutable::new(String::from(""))
}
#[static_ref]
fn password2() -> &'static Mutable<String> {
    Mutable::new(String::from(""))
}
#[static_ref]
pub fn login_error() -> &'static Mutable<Option<Cow<'static, str>>> {
    Mutable::new(None)
}
#[static_ref]
pub fn email_error() -> &'static Mutable<Option<Cow<'static, str>>> {
    Mutable::new(None)
}
#[static_ref]
pub fn password_error() -> &'static Mutable<Option<Cow<'static, str>>> {
    Mutable::new(None)
}
fn send_mail() {
    use crate::connection::*;
    use shared::*;
    Task::start(async {
        let form = ResetForm {
            token: token().get_cloned(),
            email: email().get_cloned(),
            password: password().get_cloned(),
            password2: password2().get_cloned(),
        };
        if let Err(_e) = form.is_valid() {
            if form.has_error("email") {
                email_error().set(Some(Cow::Borrowed("Email is not valid")))
            }
            if form.has_error("password2") {
                password_error().set(Some(Cow::Borrowed("Password is not valid")))
            }
        } else {
            let msg = UpMsg::ResetPassword(form);
            match connection().send_up_msg(msg).await {
                Err(error) => {
                    let error = error.to_string();
                    eprintln!("login request failed: {}", error);
                }
                Ok(_msg) => (),
            }
        }
    });
}
