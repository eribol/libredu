use crate::i18n;
use zoon::{eprintln, named_color::*, println, *};


#[static_ref]
pub fn register() -> &'static Mutable<bool> {
    Mutable::new(false)
}
#[static_ref]
fn first_name() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

#[static_ref]
fn error() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

#[static_ref]
fn last_name() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

#[static_ref]
fn email() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

#[static_ref]
fn short_name() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

#[static_ref]
fn password() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

#[static_ref]
fn password2() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

fn change_first_name(name: String) {
    first_name().set(name)
}

fn change_last_name(name: String) {
    last_name().set(name)
}

fn change_email(name: String) {
    email().set(name)
}

fn change_short_name(name: String) {
    short_name().set(name)
}

fn change_password(p: String) {
    password().set_neq(p)
}

fn change_password2(p: String) {
    password2().set_neq(p)
}

pub fn signin_page()->impl Element{
    Column::new()
    .item_signal(
        register()
        .signal()
        .map_bool(|| registered().into_raw_element(), || register_view().into_raw_element())
    )
}
pub fn registered()->impl Element{
    Column::new().item(
        Label::new().label("E-posta adresinize link gönderildi")
    )
}
pub fn register_view() -> impl Element {
    
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
                .s(Height::exact(30))
                .s(Borders::all(Border::new().solid().color(BLUE_5)))
                .id("first_name")
                .placeholder(Placeholder::with_signal(i18n::t!("first_name")))
                .input_type(InputType::text())
                .on_change(change_first_name),
        )
        .item(
            TextInput::new()
                .s(Align::center())
                .s(Height::exact(30))
                .s(Borders::all(Border::new().solid().color(BLUE_5)))
                .id("last_name")
                .placeholder(Placeholder::with_signal(i18n::t!("last_name")))
                .input_type(InputType::text())
                .on_change(change_last_name),
        )
        .item(
            TextInput::new()
                .s(Align::center())
                .s(Height::exact(30))
                .s(Borders::all(Border::new().solid().color(BLUE_5)))
                .id("email")
                .placeholder(Placeholder::with_signal(i18n::t!("email")))
                .input_type(InputType::text())
                .on_change(change_email),
        )
        .item(
            TextInput::new()
                .s(Align::center())
                .s(Height::exact(30))
                .s(Borders::all(Border::new().solid().color(BLUE_5)))
                .id("short_name")
                .placeholder(Placeholder::with_signal(i18n::t!("short_name")))
                .input_type(InputType::text())
                .on_change(change_short_name),
        )
        .item(
            TextInput::new()
                .s(Align::center())
                .s(Height::exact(30))
                .s(Borders::all(Border::new().solid().color(BLUE_5)))
                .id("password")
                .placeholder(Placeholder::with_signal(i18n::t!("password")))
                .input_type(InputType::password())
                .on_change(change_password),
        )
        .item(
            TextInput::new()
                .s(Align::center())
                .s(Height::exact(30))
                .s(Borders::all(Border::new().solid().color(BLUE_5)))
                .id("password2")
                .placeholder(Placeholder::with_signal(i18n::t!("password_again")))
                .input_type(InputType::password())
                .on_change(change_password2),
        )
        .item(
            Label::new()
            .label_signal(
                error().signal_cloned().map(|a| a))
            )
        .item(
            Button::new()
                .s(Height::exact(35))
                .s(RoundedCorners::all(10))
                .s(Borders::all(Border::new().solid().color(BLUE_5)))
                .label(El::new().s(Align::center()).child_signal(i18n::t!("login")))
                .on_click(|| signin()),
        )
}

use crate::connection::*;
use shared::{signin::SigninForm, UpMsg};
fn signin() {
    let user = SigninForm {
        first_name: first_name().get_cloned(),
        last_name: last_name().get_cloned(),
        email: email().get_cloned(),
        short_name: short_name().get_cloned(),
        password: password().get_cloned(),
        password2: password2().get_cloned(),
    };
    match  user.is_valid(){
        Ok(_u) =>{
            Task::start(async {
                let msg = UpMsg::Signin { form: user };
                match connection().send_up_msg(msg).await {
                    Err(error) => {
                        let error = error.to_string();
                        eprintln!("login request failed: {}", error);
                    }
                    Ok(msg) => println!("{:?}", msg),
                }
            });
        }
        Err(e) =>{
            error().set(e.to_string())
        }
    }
   
}
