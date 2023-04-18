use std::borrow::Cow;

use crate::i18n;
use zoon::{eprintln, named_color::*, println, *};

#[static_ref]
pub fn signin_form() -> &'static Mutable<SigninForm> {
    Mutable::new(SigninForm::default())
}
#[static_ref]
pub fn server_error() -> &'static Mutable<Option<Cow<'static, str>>> {
    Mutable::new(None)
}
#[static_ref]
fn first_name_error() -> &'static Mutable<Option<Cow<'static, str>>> {
    Mutable::new(None)
}
#[static_ref]
fn last_name_error() -> &'static Mutable<Option<Cow<'static, str>>> {
    Mutable::new(None)
}
#[static_ref]
fn email_error() -> &'static Mutable<Option<Cow<'static, str>>> {
    Mutable::new(None)
}
#[static_ref]
fn short_name_error() -> &'static Mutable<Option<Cow<'static, str>>> {
    Mutable::new(None)
}
#[static_ref]
fn password_error() -> &'static Mutable<Option<Cow<'static, str>>> {
    Mutable::new(None)
}
#[static_ref]
pub fn register() -> &'static Mutable<bool> {
    Mutable::new(false)
}

fn change_first_name(name: String) {
    first_name_error().set(None);
    let mut form = signin_form().get_cloned();
    form.first_name = name;
    signin_form().set(form);
}

fn first_name_validate(){
    if signin_form().get_cloned().has_error("first_name"){
        first_name_error().set(Some(Cow::from("A name len should be at leats 2 chars")));
    }
}

fn change_last_name(name: String) {
    last_name_error().set(None);
    let mut form = signin_form().get_cloned();
    form.last_name = name;
    signin_form().set(form);
}
fn last_name_validate(){
    if signin_form().get_cloned().has_error("last_name"){
        last_name_error().set(Some(Cow::from("A name len should be at leats 2 chars")));
    }
}

fn change_email(name: String) {
    email_error().set(None);
    let mut form = signin_form().get_cloned();
    form.email = name;
    signin_form().set(form);
}

fn email_validate(){
    if signin_form().get_cloned().has_error("email"){
        email_error().set(Some(Cow::from("It is not proper email")));
    }
}

fn change_short_name(name: String) {
    short_name_error().set(None);
    let mut form = signin_form().get_cloned();
    form.short_name = name;
    signin_form().set(form);
}
fn short_name_validate(){
    if signin_form().get_cloned().has_error("short_name"){
        short_name_error().set(Some(Cow::from("A short name len should be between 2-6")));
    }
}
fn change_password(p: String) {
    password_error().set(None);
    let mut form = signin_form().get_cloned();
    form.password = p;
    signin_form().set(form);
}
fn password_validate(){
    if signin_form().get_cloned().has_error("password"){
        password_error().set(Some(Cow::from("Password is not valid")));
    }
}
fn change_password2(p: String) {
    password_error().set(None);
    let mut form = signin_form().get_cloned();
    form.password2 = p;
    signin_form().set(form);
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
                .on_change(change_first_name)
                .update_raw_el(|raw_el|
                    raw_el.event_handler(|_event: events::FocusOut| first_name_validate())
                )
        )
        .item_signal(
            first_name_error().signal_cloned().map_some(|e| 
                Label::new()
                .s(Font::new().weight(FontWeight::Number(10)).color(RED_6))
                .label(e)
            )
        )
        .item(
            TextInput::new()
                .s(Align::center())
                .s(Height::exact(30))
                .s(Borders::all(Border::new().solid().color(BLUE_5)))
                .id("last_name")
                .placeholder(Placeholder::with_signal(i18n::t!("last_name")))
                .input_type(InputType::text())
                .on_change(change_last_name)
                .update_raw_el(|raw_el|
                    raw_el.event_handler(|_event: events::FocusOut| last_name_validate())
                )
        )
        .item_signal(
            last_name_error().signal_cloned().map_some(|e| 
                Label::new()
                .s(Font::new().weight(FontWeight::Number(10)).color(RED_6))
                .label(e)
            )
        )
        .item(
            TextInput::new()
                .s(Align::center())
                .s(Height::exact(30))
                .s(Borders::all(Border::new().solid().color(BLUE_5)))
                .id("email")
                .placeholder(Placeholder::with_signal(i18n::t!("email")))
                .input_type(InputType::text())
                .on_change(change_email)
                .update_raw_el(|raw_el|
                    raw_el.event_handler(|_event: events::FocusOut| email_validate())
                )
        ).item_signal(
            email_error().signal_cloned().map_some(|e| 
                Label::new()
                .s(Font::new().weight(FontWeight::Number(10)).color(RED_6))
                .label(e)
            )
        )
        .item(
            TextInput::new()
                .s(Align::center())
                .s(Height::exact(30))
                .s(Borders::all(Border::new().solid().color(BLUE_5)))
                .id("short_name")
                .placeholder(Placeholder::with_signal(i18n::t!("short_name")))
                .input_type(InputType::text())
                .on_change(change_short_name)
                .update_raw_el(|raw_el|
                    raw_el.event_handler(|_event: events::FocusOut| short_name_validate())
                )
        ).item_signal(
            short_name_error().signal_cloned().map_some(|e| 
                Label::new()
                .s(Font::new().weight(FontWeight::Number(10)).color(RED_6))
                .label(e)
            )
        )
        .item(
            TextInput::new()
                .s(Align::center())
                .s(Height::exact(30))
                .s(Borders::all(Border::new().solid().color(BLUE_5)))
                .id("password")
                .placeholder(Placeholder::with_signal(i18n::t!("password")))
                .input_type(InputType::password())
                .on_change(change_password)
                .update_raw_el(|raw_el|
                    raw_el.event_handler(|_event: events::FocusOut| password_validate())
                )
        )
        .item(
            TextInput::new()
                .s(Align::center())
                .s(Height::exact(30))
                .s(Borders::all(Border::new().solid().color(BLUE_5)))
                .id("password2")
                .placeholder(Placeholder::with_signal(i18n::t!("password_again")))
                .input_type(InputType::password())
                .on_change(change_password2)
                .update_raw_el(|raw_el|
                    raw_el.event_handler(|_event: events::FocusOut| password_validate())
                )
        )
        .item_signal(
            password_error().signal_cloned().map_some(|e| 
                Label::new()
                .s(Font::new().weight(FontWeight::Number(10)).color(RED_6))
                .label(e)
            )
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
    let user = signin_form().get_cloned();
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
            user.has_error("first_name");
        }
    }
   
}
