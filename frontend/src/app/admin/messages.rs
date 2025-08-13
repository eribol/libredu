use shared::{
    msgs::{
        admin::AdminUpMsgs,
        messages::{Message, NewMessage},
    },
    School, UpMsg,
};
use zoon::{Element, Row, *};

use crate::{app::login_user, connection::send_msg};

use super::last_schools;

#[static_ref]
pub fn messages() -> &'static MutableVec<Message> {
    MutableVec::new_with_values(vec![])
}
#[static_ref]
pub fn message() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}
#[static_ref]
fn text_box_selected() -> &'static Mutable<bool> {
    Mutable::new(false)
}
#[static_ref]
fn selected_school() -> &'static Mutable<Option<School>> {
    Mutable::new(None)
}

#[static_ref]
fn receiver() -> &'static Mutable<Option<i32>> {
    Mutable::new(None)
}
fn select_school(school: School) {
    get_messages(school.id);
    selected_school().set(Some(school))
}

fn get_messages(id: i32) {
    let msg = AdminUpMsgs::GetSchoolMessages(id);
    let msg = UpMsg::Admin(msg);
    send_msg(msg);
}

pub fn messages_view() -> impl Element {
    Row::new()
        .s(Gap::new().x(1))
        .s(Borders::all(Border::new().width(1)))
        .item(messages_schools())
        .item_signal(
            selected_school()
                .signal_cloned()
                .map_some(|_s| message_text()),
        )
}

fn messages_schools() -> impl Element {
    Column::new()
        .s(Align::new().left())
        .s(Borders::all(Border::new().width(1)))
        .s(Width::exact(400))
        .s(Height::fill())
        .items_signal_vec(last_schools().signal_vec_cloned().map(|school| {
            let name = school.school.name.clone();
            let sch = school.school.clone();
            Column::new()
                .on_click(move || {
                    select_school(sch.clone());
                })
                .item(Button::new().label(name))
                .item(
                    Button::new()
                        .label(format!(
                            "{} {}",
                            school.principle.first_name, school.principle.last_name
                        ))
                        .s(Font::new().weight(FontWeight::ExtraLight)),
                )
        }))
}

fn message_text() -> impl Element {
    Column::new()
        .s(Align::new().left())
        .s(Width::fill())
        .s(Height::fill())
        .s(Borders::all(Border::new().width(1)))
        .s(Background::new().color(color!("gray")))
        .item(texts())
        .item(text_box())
}

fn text_box() -> impl Element {
    Row::new()
        .s(RoundedCorners::all(10))
        .item(
            TextArea::new()
                .s(Align::new().bottom())
                .id("message")
                .placeholder(Placeholder::new("write something"))
                .on_change(|s| message().set(s))
                .text_signal(message().signal_cloned())
                .on_key_down_event_with_options(EventOptions::new().preventable(), |event| {
                    let RawKeyboardEvent::KeyDown(raw_event) = &event.raw_event;
                    if let Key::Enter = event.key() {
                        raw_event.prevent_default();
                        send();
                    }
                }),
        )
        .s(borders(border(text_box_selected().signal())))
}
fn texts() -> impl Element {
    Column::new()
        .s(Align::new().bottom())
        .s(Scrollbars::both())
        .items_signal_vec(
            messages()
                .signal_vec_cloned()
                .map(|m| Label::new().label(m.body)),
        )
}

fn borders<'a>(f: impl Signal<Item = Border> + Unpin + 'static) -> Borders<'a> {
    Borders::all_signal(f)
}

fn border(f: impl Signal<Item = bool>) -> impl Signal<Item = Border> {
    f.map_bool(
        || Border::new().width(1).color(color!("gray")),
        || Border::new().width(1).color(color!("blue")),
    )
}

fn send() {
    let b = message().get_cloned();
    let schol = selected_school().get_cloned();
    if let Some(s) = schol {
        let messaging = NewMessage {
            sender_id: login_user().get_cloned().unwrap().id,
            school_id: Some(s.id),
            school_name: s.name,
            body: b.clone(),
            send_time: Utc::now().naive_utc(),
            to_school: true,
            read: false,
        };
        let m_msg = AdminUpMsgs::SendMessage(messaging);
        if b.len() > 2 {
            send_msg(UpMsg::Admin(m_msg));
            message().take();
        }
    }
}
