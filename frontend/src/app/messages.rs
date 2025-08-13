use shared::{
    msgs::messages::{Message, MessagesUpMsgs, NewMessage},
    UpMsg,
};
use zoon::*;

use crate::{connection::send_msg, elements::text_inputs, i18n::t};

use super::{login_user, school::school};
pub fn help_nav() -> impl Element {
    Column::new()
        .id("message")
        .s(Padding::new().right(20))
        .s(Background::new().color(color!("GRAY")))
        .item(
            Button::new()
                .label_signal(t!("help"))
                .on_click(|| help_message().set(!help_message().get())),
        )
        .update_raw_el(|raw| {
            raw //.style("background-color", "#222")
                .style("position", "fixed")
                .style_signal(
                    "width",
                    help_message().signal().map_bool(|| "25%", || "15%"),
                )
                .style("right", "20px")
                .style("bottom", "20px")
        })
        .element_above_signal(help_message().signal().map_true(messaging_view))
}

fn messaging_view() -> impl Element {
    Column::new()
        .s(Height::exact(500))
        .s(Borders::all(Border::new().color(color!("RED"))))
        .s(Padding::new().left(5).right(5))
        .s(Background::new().color(color!("GRAY")))
        .item(texts())
        .item(
            text_inputs::default()
                .id("text")
                .placeholder(Placeholder::with_signal(t!("write-help")))
                .s(Align::new().bottom())
                .on_change(|s| message().set(s))
                .text_signal(message().signal_cloned())
                .on_key_down_event_with_options(EventOptions::new().preventable(), |event| {
                    let RawKeyboardEvent::KeyDown(raw_event) = &event.raw_event;
                    if let Key::Enter = event.key() {
                        raw_event.prevent_default();
                        send();
                    }
                })
                .update_raw_el(|raw| {
                    raw.style("position", "absolute")
                        .style("width", "100%")
                        .style("bottom", "0")
                }),
        )
        .on_click_outside_with_ids(|| help_message().set(!help_message().get()), ["message"])
    //.after_remove(|_| help_message().set(!help_message().get()))
}

fn texts() -> impl Element {
    Column::new()
        .id("texts")
        .s(Height::exact(480))
        .s(Align::new().bottom())
        .s(Scrollbars::both())
        .s(Padding::new().bottom(40))
        .s(Gap::new().y(5))
        //.s(Height::exact(1000))
        .items_signal_vec(msgs().signal_vec_cloned().map(|m| {
            message_visible().get();
            if m.to_school {
                Label::new()
                    .label(m.body)
                    .s(Align::new().left())
                    .s(Padding::new().left(5).right(10))
                    .s(RoundedCorners::all(10))
            } else {
                Label::new()
                    .label(m.body)
                    .s(Align::new().right())
                    .s(Padding::new().left(5).right(10))
                    .s(Font::new()
                        .color(color!("GREEN"))
                        .weight(FontWeight::SemiBold))
                //.s(Background::new().color(GRAY_4))
            }
        }))
}

#[static_ref]
fn message() -> &'static Mutable<String> {
    Mutable::new(String::from(""))
}
#[static_ref]
fn help_message() -> &'static Mutable<bool> {
    Mutable::new(false)
}

#[static_ref]
pub fn msgs() -> &'static MutableVec<Message> {
    get_messages();
    MutableVec::new()
}
#[static_ref]
pub fn last_message() -> &'static Mutable<Option<i32>> {
    Mutable::new(None)
}
#[static_ref]
fn message_visible() -> &'static Mutable<bool> {
    let visible = window()
        .document()
        .unwrap()
        .get_element_by_id("texts")
        .unwrap()
        .client_height();
    use zoon::println;
    println!("{visible}");
    Mutable::new(false)
}
fn send() {
    let b = message().get_cloned();
    let schol = school().get_cloned();
    if let Some(s) = schol {
        let messaging = NewMessage {
            sender_id: login_user().get_cloned().unwrap().id,
            school_id: Some(s.id),
            school_name: s.name,
            body: b.clone(),
            send_time: Utc::now().naive_utc(),
            to_school: false,
            read: false,
        };
        let m_msg = MessagesUpMsgs::SendMessage(messaging);
        if b.len() > 2 {
            send_msg(UpMsg::Messages(m_msg));
            message().take();
        }
    }
}

fn get_messages() {
    let s = school().get_cloned().unwrap().id;
    let m_msg = MessagesUpMsgs::GetMessages(s);
    send_msg(UpMsg::Messages(m_msg));
}

pub fn streaming_messages() {
    let s = school().get_cloned().unwrap().id;
    Task::start(async move {
        loop {
            let last_m = last_message().get().unwrap();
            let m_msg = MessagesUpMsgs::GetNewMessages(s, last_m);
            send_msg(UpMsg::Messages(m_msg));
            Timer::sleep(5000).await;
        }
    })
}
