use shared::{msgs::messages::{MessagesUpMsgs, Message}, UpMsg};
use zoon::{named_color::*, *};

use crate::{elements::text_inputs, connection::send_msg, i18n::t};

use super::{login_user, school::school};
pub fn help_nav()->impl Element{
    Column::new()
    .id("message")
    .s(Padding::new().right(20))
    .s(Background::new().color(GRAY_1))
    .item(
        Button::new().label_signal(t!("help"))
        .on_click(||{
            help_message().set(!help_message().get())
        })
    )
    .update_raw_el(|raw|
         raw//.style("background-color", "#222")
        .style("position", "fixed")
        .style_signal("width", help_message().signal().map_bool(|| "25%", || "15%"))
        .style("right", "20px")
        .style("bottom", "20px")
    ).element_above_signal(help_message().signal().map_true(messaging_view))
}

fn messaging_view()->impl Element{
    Column::new()
    .s(Height::exact(500))
    .s(Borders::all(Border::new().color(RED_1)))
    .s(Padding::new().left(5).right(5))
    .s(Background::new().color(GRAY_1))
    .item(texts())
    .item(
        text_inputs::default().id("text")
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
        .update_raw_el(|raw|{
            raw
            .style("position", "absolute")
            .style("width", "100%")
            .style("bottom", "0")    
        })
    )
    .on_click_outside_with_ids(|| help_message().set(!help_message().get()), ["message"])
    //.after_remove(|_| help_message().set(!help_message().get()))
}

fn texts()->impl Element{
    Column::new()
    .s(Align::new().bottom())
    .s(Scrollbars::both())
    .s(Padding::new().bottom(40))
    .s(Gap::new().y(5))
    //.s(Height::exact(1000))
    .items_signal_vec(msgs().signal_vec_cloned().map(|m|{
        if m.receiver_id == login_user().get_cloned().unwrap().id{
            Label::new().label(m.body).s(Align::new().left())
            .s(Font::new().color(GREEN_5).weight(FontWeight::SemiBold))
            .s(RoundedCorners::all(10))
        }
        else{
            Label::new().label(m.body).s(Align::new().right())
            .s(Background::new().color(GRAY_4))
        }
    }))
}

#[static_ref]
fn message()->&'static Mutable<String>{
    Mutable::new(String::from(""))
}
#[static_ref]
fn help_message()->&'static Mutable<bool>{
    Mutable::new(false)
}

#[static_ref]
pub fn msgs()->&'static MutableVec<Message>{
    get_messages();
    MutableVec::new()
}

fn send(){
    let b = message().get_cloned();
    let schol = school().get_cloned();
    if let Some(s) = schol{
        let messaging = Message{
            sender_id: login_user().get_cloned().unwrap().id,
            receiver_id: 1,
            school_id: Some(s.id),
            school_name: s.name,
            body: b.clone(),
            send_time: Utc::now().naive_utc(),
            sent: 1
        };
        let m_msg =  MessagesUpMsgs::SendMessage(messaging);
        if b.len() > 2{
            send_msg(UpMsg::Messages(m_msg));
            message().take();
        }    
    }
}

fn get_messages(){
    let m_msg =  MessagesUpMsgs::GetMessages;
    send_msg(UpMsg::Messages(m_msg));
    streaming_messages();
}

fn streaming_messages(){
    Task::start(async move {
        loop{
            let m_msg =  MessagesUpMsgs::GetMessages;
            send_msg(UpMsg::Messages(m_msg));
            Timer::sleep(10000).await;    
        }
    })
}