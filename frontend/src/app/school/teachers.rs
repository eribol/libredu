use crate::connection::send_msg;
use crate::elements::{buttons, text_inputs};
use crate::i18n::t;
use shared::msgs::teachers::TeacherUpMsgs;
use shared::{
    models::teacher::{AddTeacher, Teacher},
    UpMsg,
};
use zoon::{named_color::*, *};

pub fn home() -> impl Element {
    Column::new()
        .s(Padding::new().top(10))
        .s(Gap::new().y(5))
        .item(form())
        .item(teachers_view())
}

fn form() -> impl Element {
    Column::new()
        .s(Align::center())
        .s(Width::exact_signal(
            crate::app::screen_width().signal().map(|a| a / 4),
        ))
        .s(Shadows::new([
            Shadow::new().y(2).blur(4).color(hsluv!(0, 0, 0, 20)),
            Shadow::new().y(25).blur(50).color(hsluv!(0, 0, 0, 10)),
        ]))
        .s(Gap::new().y(20))
        .item(role_view())
        .item(first_name_view())
        .item(last_name_view())
        .item(short_name_view())
        .item(update())
}
fn role_view() -> impl Element {
    Column::new()
        .s(Align::center())
        .item(
            Label::new()
                .label_signal(t!("teacher-role"))
                .s(Align::center()),
        )
        .item(RawHtmlEl::new("select").children(vec![
                    RawHtmlEl::new("option")
                        .event_handler(move |_event: events::Click| {
                            change_role(0)
                        })
                        .child_signal(t!("principal")),
                    RawHtmlEl::new("option")
                        .event_handler(move |_event: events::Click| {
                            change_role(1)
                        })
                        .child_signal(t!("vice-principal")),
                    RawHtmlEl::new("option")
                        .event_handler(move |_event: events::Click| {
                            change_role(2)
                        })
                        .child_signal(t!("deputy-principal")),
                    RawHtmlEl::new("option")
                        .event_handler(move |_event: events::Click| {
                            change_role(3)
                        })
                        .child_signal(t!("school-counselor")),
                    RawHtmlEl::new("option")
                        .event_handler(move |_event: events::Click| {
                            change_role(4)
                        })
                        .child_signal(t!("teacher"))
                ]))
}

fn first_name_view() -> impl Element {
    Column::new()
        .item(
            Label::new()
                .label_signal(t!("first_name"))
                .s(Align::center()),
        )
        .item(
            text_inputs::default()
                .s(Align::center())
                .id("first_name")
                .text("")
                .on_change(change_first_name),
        )
}
fn last_name_view() -> impl Element {
    Column::new()
        .item(
            Label::new()
                .label_signal(t!("last_name"))
                .s(Align::center()),
        )
        .item(
            text_inputs::default()
                .s(Align::center())
                .id("last_name")
                .text("")
                .on_change(change_last_name),
        )
}
fn short_name_view() -> impl Element {
    Column::new()
        .item(
            Label::new()
                .label_signal(t!("short_name"))
                .s(Align::center()),
        )
        .item(
            text_inputs::default()
                .s(Align::center())
                .id("short_name")
                .text("")
                .on_change(change_short_name),
        )
}
fn update() -> impl Element {
    buttons::default_with_signal(t!("add")).on_click(add_teacher)
}

fn teachers_view() -> impl Element {
    Column::new()
        .s(Gap::both(5))
        .items_signal_vec(teachers2().signal_vec_cloned().map(|col| {
            Row::new().items_signal_vec(col.signal_vec_cloned().map(|row| {
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
                    .item(Button::new().label(format!("{} {}", row.first_name, row.last_name)))
                    .item(Button::new().label_signal(t!("delete")).on_press(move || del_teacher(row.id)))
                //.on_click(move || super::teacher::open_modal(row.clone()))
            }))
        }))
}

#[static_ref]
fn first_name() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}
#[static_ref]
fn last_name() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}
#[static_ref]
fn short_name() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

#[static_ref]
fn role() -> &'static Mutable<i32> {
    Mutable::new(0)
}

fn change_first_name(value: String) {
    first_name().set(value)
}

fn change_last_name(value: String) {
    last_name().set(value)
}

fn change_short_name(value: String) {
    short_name().set(value)
}

pub fn change_role(value: i32) {
    role().set(value)
}

fn teacher_form() -> AddTeacher {
    AddTeacher {
        first_name: first_name().get_cloned(),
        last_name: last_name().get_cloned(),
        short_name: short_name().get_cloned(),
    }
}

fn add_teacher() {
    use crate::connection::*;
    use shared::*;
    let t_msg = TeacherUpMsgs::AddTeacher(teacher_form());
    let msg = UpMsg::Teachers(t_msg);
    send_msg(msg)
}

fn del_teacher(id: i32) {
    use crate::connection::*;
    use shared::*;
    let t_msg = TeacherUpMsgs::DelTeacher(id);
    let msg = UpMsg::Teachers(t_msg);
    send_msg(msg)
}

#[static_ref]
pub fn teachers() -> &'static MutableVec<Teacher> {
    get_teachers();
    MutableVec::new_with_values(vec![])
}

#[static_ref]
pub fn teachers2() -> &'static MutableVec<MutableVec<Teacher>> {
    MutableVec::new_with_values(vec![])
}

pub fn create_chunks() {
    let teachs = teachers().lock_mut().to_vec();
    let teachs = teachs
        .chunks(10)
        .map(|c| MutableVec::new_with_values(c.into()))
        .collect();
    teachers2().lock_mut().replace_cloned(teachs);
}

fn get_teachers() {
    let t_msg = TeacherUpMsgs::GetTeachers;
    send_msg(UpMsg::Teachers(t_msg))
}
