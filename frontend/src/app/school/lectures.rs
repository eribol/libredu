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
}
fn name_view() -> impl Element {
    Column::new()
        .item(Label::new().label("Ders Adı").s(Align::center()))
        .item(text_inputs::default().id("name").on_change(change_name))
}
fn short_name_view() -> impl Element {
    Column::new()
        .item(Label::new().label("Kısa Adı").s(Align::center()))
        .item(
            text_inputs::default()
                .id("short_name")
                .on_change(change_short_name),
        )
}

fn update() -> impl Element {
    buttons::default_with_signal(t!("add")).on_click(add_lecture)
}

fn lectures_view() -> impl Element {
    Column::new()
        .on_viewport_size_change(|_, _| create_chunks())
        .s(Gap::new().y(5))
        //.s(Width::exact_signal(screen_width().signal().map(|a| a / 2)))
        .items_signal_vec(lectures2().signal_vec_cloned().map(|col| {
            Row::new()
                .s(Gap::new().x(5))
                .items_signal_vec(col.signal_vec_cloned().map(|r| {
                    let a = Mutable::new(false);
                    Column::new()
                        .s(Borders::all_signal(a.signal().map_bool(
                            || Border::new().width(1).color(BLUE_3).solid(),
                            || Border::new().width(1).color(BLUE_1).solid(),
                        )))
                        .s(RoundedCorners::all(2))
                        .s(Width::exact(140))
                        .s(Height::exact(75))
                        .on_hovered_change(move |b| a.set(b))
                        .item(Button::new().label(format!("{} ({})", if r.short_name.len() == 0{
                            &r.name
                        }else{&r.short_name}, &r.kademe)))
                        .item(Button::new().label_signal( t!("delete")).on_press(move || del_lecture(r.id)))
                }))
        }))
}

#[static_ref]
pub fn lectures() -> &'static MutableVec<Lecture> {
    get_lectures();
    MutableVec::new_with_values(vec![])
}

#[static_ref]
pub fn lectures2() -> &'static MutableVec<MutableVec<Lecture>> {
    //get_lectures();
    MutableVec::new_with_values(vec![])
}

pub fn create_chunks() {
    let width = screen_width().get();
    println!("{width:?}");
    let lects = lectures().lock_mut().to_vec();
    let lects = lects
        .chunks((width / 150) as usize)
        .map(|c| MutableVec::new_with_values(c.into()))
        .collect();
    lectures2().lock_mut().replace_cloned(lects);
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
    let msg = LecturesUpMsg::AddLecture(
        AddLecture {
            kademe: grade().get_cloned(),
            name: name().get_cloned(),
            short_name: short_name().get_cloned(),
        });
    send_msg(UpMsg::Lectures(msg));
}

fn del_lecture(id: i32){
    use shared::*;
    let msg = UpMsg::Lectures(msgs::lectures::LecturesUpMsg::DelLecture(id));
    send_msg(msg);
}
