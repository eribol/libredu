use shared::UpMsg;
use shared::models::lectures::{AddLecture, Lecture};
use shared::msgs::lectures::LecturesUpMsg;
use zoon::named_color::*;
use zoon::{println, *};

use crate::connection::send_msg;
use crate::{
    app::screen_width,
    elements::{buttons, text_inputs},
    i18n::t,
};

pub fn home() -> impl Element {
    Column::new()
        //.s(Align::center())
        .s(Padding::new().top(10))
        .s(Gap::new().y(20))
        .item(form())
        .item(lectures_view())
}

fn form() -> impl Element {
    Column::new()
        .s(Align::center())
        .s(Width::exact_signal(screen_width().signal().map(|a| a / 4)))
        .s(Shadows::new([
            Shadow::new().y(2).blur(4).color(hsluv!(0, 0, 0, 20)),
            Shadow::new().y(25).blur(50).color(hsluv!(0, 0, 0, 10)),
        ]))
        .s(Gap::new().y(20))
        .item(grade_view())
        .item(name_view())
        .item(short_name_view())
        .item(update())
}

fn grade_view() -> impl Element {
    Column::new()
    .s(Align::center())
    .item(Label::new().label("Ders Kademesi").s(Align::center()))
    .item(text_inputs::default().id("grade").on_change(change_grade))
    .item_signal(
        grade_error().signal_cloned().map_some(|s|
            Label::new().label(s)
            .s(Align::center())
            .s(Font::new().weight(FontWeight::Light).color(RED_5))   
        )
    )
}
fn name_view() -> impl Element {
    Column::new()
    .item(Label::new().label("Ders Adı").s(Align::center()))
    .item(text_inputs::default().id("name").on_change(change_name))
    .item_signal(
        name_error().signal_cloned().map_some(|s|
            Label::new().label(s)
            .s(Align::center())
            .s(Font::new().weight(FontWeight::Light).color(RED_5))   
        )
    )        
}
fn short_name_view() -> impl Element {
    Column::new()
    .item(Label::new().label("Kısa Adı").s(Align::center()))
    .item(
        text_inputs::default()
        .id("short_name")
        .on_change(change_short_name),
    )
    .item_signal(
        short_name_error().signal_cloned().map_some(|s|
            Label::new().label(s)
            .s(Align::center())
            .s(Font::new().weight(FontWeight::Light).color(RED_5))   
        )
    )
}

fn update() -> impl Element {
    buttons::default_with_signal(t!("add")).on_click(add_lecture)
}

fn lectures_view() -> impl Element {
    Row::new()
    .s(Gap::new().y(5))
    .s(Gap::new().x(5))
    .multiline()
    .items_signal_vec(lectures().signal_vec_cloned().map(|r| {
        let a = Mutable::new(false);
        Column::new()
        .element_below_signal(
            crate::modals::del_signal(r.id).map_true(move ||
            crate::modals::del_modal_all(&r.id.to_string(), r.id, UpMsg::Lectures(LecturesUpMsg::DelLecture(r.id))))
        )
        .s(Borders::all_signal(a.signal().map_bool(
            || Border::new().width(1).color(BLUE_3).solid(),
            || Border::new().width(1).color(BLUE_1).solid(),
        )))
        .s(RoundedCorners::all(2))
        .s(Width::exact(140))
        .s(Height::exact(75))
        .on_hovered_change(move |b| a.set(b))
        .item(
            Button::new()
            .s(Align::new().center_y())
            .label(
                format!("{} ({})", if r.short_name.len() == 0{
                    &r.name
                }else{
                    &r.short_name
                }, &r.kademe))
        )
        .item({
            let a = Mutable::new_and_signal(false);
            Button::new()
            .s(Font::new()
                .weight_signal(a.0.signal().map_bool(|| FontWeight::Regular, || FontWeight::ExtraLight))
                .color_signal(a.0.signal().map_bool(|| RED_8, || RED_4)))
            .s(Align::new().bottom())
            .on_hovered_change(move |h| a.0.set_neq(h))
            .label_signal( t!("delete")).on_press(move || crate::modals::del_modal().set(Some(r.id)))
        })
    }))
}

#[static_ref]
pub fn lectures() -> &'static MutableVec<Lecture> {
    get_lectures();
    MutableVec::new_with_values(vec![])
}

#[static_ref]
fn grade() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

#[static_ref]
fn name() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

#[static_ref]
fn short_name() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}
#[static_ref]
fn grade_error() -> &'static Mutable<Option<String>> {
    Mutable::new(None)
}

#[static_ref]
fn name_error() -> &'static Mutable<Option<String>> {
    Mutable::new(None)
}

#[static_ref]
fn short_name_error() -> &'static Mutable<Option<String>> {
    Mutable::new(None)
}

fn change_name(value: String) {
    name().set(value)
}
fn change_short_name(value: String) {
    short_name().set(value)
}
fn change_grade(value: String) {
    grade().set(value)
}

pub fn get_lectures() {
    use shared::*;
    let msg = UpMsg::Lectures(msgs::lectures::LecturesUpMsg::GetLectures);
    send_msg(msg);
}
fn add_lecture() {
    use crate::connection::*;
    use shared::*;
    let form = AddLecture {
            kademe: grade().get_cloned(),
            name: name().get_cloned(),
            short_name: short_name().get_cloned(),
    };
    if let Err(_e) = form.is_valid(){
        if form.has_error("name"){
            name_error().set(Some("Name is not valid".to_string()))
        }
        if form.has_error("short_name"){
            short_name_error().set(Some("Short name is not valid".to_string()))
        }
        if form.has_error("grade"){
            grade_error().set(Some("Grade is not valid".to_string()))
        }
    }
    else{
        let msg = LecturesUpMsg::AddLecture(form);
        send_msg(UpMsg::Lectures(msg));
    }
    
}

fn del_lecture(id: i32){
    use shared::*;
    let msg = UpMsg::Lectures(msgs::lectures::LecturesUpMsg::DelLecture(id));
    send_msg(msg);
}
