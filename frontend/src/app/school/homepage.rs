use std::borrow::Cow;

use crate::{
    app::login_user,
    elements::{buttons, text_inputs},
    i18n::t,
};
use shared::models::school::FullSchool;
use zoon::{*, named_color::RED_6};

use super::{school, teachers::{self, teachers}};

pub fn home() -> impl Element {
    Column::new()
        //
        .s(Padding::new().top(10))
        .item(form())
}
fn form() -> impl Element {
    Column::new()
        .s(Gap::new().y(20))
        .s(Shadows::new([
            Shadow::new().y(2).blur(4).color(hsluv!(0, 0, 0, 20)),
            Shadow::new().y(25).blur(50).color(hsluv!(0, 0, 0, 10)),
        ]))
        .s(Align::center())
        .s(Width::exact(750))
        .item(manager_view())
        .item(name_view())
        .item(phone_view())
        .item(update())
        .item_signal(error().signal_cloned().map_some(|e| 
            Label::new().label(e)
            .s(
                Font::new()
                .weight(FontWeight::Number(10))
                .color(RED_6)
            ).s(Align::center())
        ))
}
fn name_view() -> impl Element {
    Column::new()
        //.s(Padding::new().x(150))
        .item(Label::new().label("Okul Adı:").s(Align::center()))
        .s(Align::center())
        .item(
            text_inputs::default()
                .s(Borders::all(Border::new().color(hsluv!(0, 0, 0, 20))))
                .s(Width::exact(300))
                .id("name")
                .text_signal(
                    school()
                        .signal_cloned()
                        .map_option(|s| s.name, || "".to_string()),
                )
                .placeholder(Placeholder::with_signal(
                    school()
                        .signal_cloned()
                        .map_option(|s| s.name, || "".to_string()),
                ))
                .on_change(change_name),
        )
}

fn manager_view() -> impl Element {
    Column::new()
        //.s(Padding::new().x(150))
        .s(Align::center())
        .item(Label::new().label("Okul Müdürü").s(Align::center()))
        .item(
            RawHtmlEl::new("select").children_signal_vec(teachers().signal_vec_cloned().map(
                |teacher| {
                    if teacher.id == manager().get() {
                        RawHtmlEl::new("option")
                            .event_handler(move |_event: events::Click| {
                                change_manager(teacher.id.to_string())
                            })
                            .attr("selected", "true")
                            .child(format!("{} {}", teacher.first_name, teacher.last_name))
                    } else {
                        RawHtmlEl::new("option")
                            .child(format!(r#"{} {}"#, teacher.first_name, teacher.last_name))
                    }
                },
            )),
        )
}

fn phone_view() -> impl Element {
    Column::new()
        //.s(Padding::new().x(150))
        
        .s(Align::center())
        .s(Width::exact(500))
        .item(Label::new().label("Telefon numarası").s(Align::center()))
        .item(
            text_inputs::default()
                .s(Width::exact(300))
                .id("phone")
                .on_change(change_phone)
        )
        .update_raw_el(|raw_el|
            raw_el.attr("title", "input phone number with country code")
        )
}

fn update() -> impl Element {
    buttons::default_with_signal(t!("update"))
        .on_click(update_school)
        //.s(Align::center())
}

#[static_ref]
fn name() -> &'static Mutable<String> {
    match school().get_cloned() {
        Some(a) => Mutable::new(a.name.clone()),
        None => Mutable::new("".to_string()),
    }
}

#[static_ref]
fn error() -> &'static Mutable<Option<Cow<'static, str>>> {
    Mutable::new(None)
}

#[static_ref]
fn phone() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

#[static_ref]
fn manager() -> &'static Mutable<i32> {
    match login_user().get_cloned() {
        Some(a) => Mutable::new(a.id),
        None => Mutable::new(0),
    }
}

fn change_name(value: String) {
    name().set(value)
}

fn change_phone(value: String) {
    error().set(None);
    phone().set(value)
}

fn change_manager(value: String) {
    manager().set(value.parse::<i32>().unwrap_or(0))
}

fn update_school() {
    use crate::connection::*;
    use shared::*;
    Task::start(async {
        let update_school = FullSchool {
            manager: manager().get(),
            phone: phone().get_cloned(),
            name: name().get_cloned(),
        };
        if let Ok(_) = update_school.is_valid(){
            let msg = UpMsg::UpdateSchool(update_school);
            connection().send_up_msg(msg).await.unwrap();
        }
        else{
            error().set(Some(Cow::from("Form is not valid")))
        }
    });
}
trait NewIntoIterator {
    type Item;
    fn into_iter(self) -> &'static dyn Element<Item = Self::Item, IntoIter = Self::Item>;
}
