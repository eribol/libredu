use super::classes::timetables;
use shared::models::timetables::AddTimetable;
use zoon::{named_color::*, *};
use crate::{i18n::t, elements::{text_inputs, buttons}};
use shared::msgs::timetables::*;

pub fn home() -> impl Element {
    Column::new()
        .s(Padding::new().top(10))
        .s(Gap::new().y(20))
        .item(form())
        .item(timetables_view())
}
fn form()->impl Element{
    Column::new()
        .s(Align::center())
        .s(Width::exact(500))
        .s(Shadows::new([
            Shadow::new().y(2).blur(4).color(hsluv!(0, 0, 0, 20)),
            Shadow::new().y(25).blur(50).color(hsluv!(0, 0, 0, 10)),
        ]))
        .s(Gap::both(20))
        .item(name_view())
        .item(hour_view())
        .item(update())
}
fn name_view() -> impl Element {
    Column::new()
        .item(Label::new().label("Yeni Ders Programı Adı").s(Align::center()))
        .item(text_inputs::default().id("name").on_change(change_name).s(Align::center()))
}

fn hour_view() -> impl Element {
    Column::new()
        .item(Label::new().label("Günlük Ders Sayısı").s(Align::center()))
        .item(text_inputs::default().id("hour").on_change(change_hour))
}

fn update() -> impl Element {
    buttons::default_with_signal(t!("update")).on_click(add_timetable)
}

fn timetables_view() -> impl Element {
    Row::new().items_signal_vec(
        timetables()
            .signal_vec_cloned()
            .map(|timetable| {
                let a = Mutable::new(false);
                Column::new().s(Borders::all_signal(a.signal().map_bool(
                        || Border::new().width(1).color(BLUE_3).solid(),
                        || Border::new().width(1).color(BLUE_1).solid(),
                    )))
                    .s(RoundedCorners::all(2))
                    .s(Width::exact(140))
                    .s(Height::exact(75))
                    .on_hovered_change(move |b| a.set(b))
                    .item(Label::new().label(timetable.name).s(Align::center()))
                    .item(Label::new().label_signal(t!("delete")).s(Align::center()).on_click(move|| del_timetable(timetable.id)))
            })
    )
}

#[static_ref]
fn hour() -> &'static Mutable<i32> {
    Mutable::new(0)
}

#[static_ref]
fn name() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

fn change_name(value: String) {
    name().set(value)
}

fn change_hour(value: String) {
    hour().set(value.parse::<i32>().unwrap_or(0))
}
fn add_timetable() {
    use crate::connection::*;
    use shared::*;
    let form = AddTimetable {
            name: name().get_cloned(),
            hour: hour().get(),
        };
    let msg = UpMsg::Timetables(TimetablesUpMsgs::AddTimetable(form));
    send_msg(msg)
}

fn del_timetable(id: i32) {
    use crate::connection::*;
    use shared::*;
    let msg = UpMsg::Timetables(TimetablesUpMsgs::DelTimetable(id));
    send_msg(msg)
}
