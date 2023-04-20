use crate::{
    elements::{buttons, text_inputs},
    i18n::t,
};
use shared::{
    models::{
        class::{AddClass, Class},
        timetables::Timetable,
    },
    msgs::{classes::ClassUpMsgs, timetables::TimetablesUpMsgs},
    UpMsg,
};
use zoon::{named_color::*, println, *, web_sys::EventTarget};

pub fn home() -> impl Element {
    Column::new()
        .s(Padding::new().top(10))
        .s(Gap::new().y(20))
        .item(form())
        .item(classes_view())
}
fn form() -> impl Element {
    Column::new()
        .s(Align::center())
        .s(Width::exact(500))
        .s(Shadows::new([
            Shadow::new().y(2).blur(4).color(hsluv!(0, 0, 0, 20)),
            Shadow::new().y(25).blur(50).color(hsluv!(0, 0, 0, 10)),
        ]))
        .s(Gap::both(20))
        .item(groups_view())
        .item(grade_view())
        .item(branch_view())
        .item(update())
}
fn groups_view() -> impl Element {
    Column::new()
        .s(Align::center())
        .item(Label::new().label("Select Timetable Group"))
        .item(
            RawHtmlEl::new("select").children_signal_vec(timetables().signal_vec_cloned().map(
                |group| {
                    RawHtmlEl::new("option")
                        .attr("value", &group.id.to_string())
                        //.attr("name", &group.name)
                        .event_handler(move |_event: events::Click| {
                            change_timetable(group.id.to_string())
                        })
                        .child(group.name)
                },
            )).event_handler(move |_event: events::Click| {
                let form = _event.target().unwrap().dyn_ref::<web_sys::HtmlSelectElement>().unwrap().clone();
                //println!("{:?}", form.value());
                change_timetable(form.value())
            })
        )
}

fn grade_view() -> impl Element {
    Column::new()
        .s(Align::center())
        .item(Label::new().label("Kademe").s(Align::center()))
        .item(
            text_inputs::default()
                .id("grade")
                .on_change(change_grade)
                .s(Align::center()),
        )
}
fn branch_view() -> impl Element {
    Column::new()
        .s(Align::center())
        .item(Label::new().label("Şube").s(Align::center()))
        .item(text_inputs::default().on_change(change_branch).id("sube"))
}
fn update() -> impl Element {
    buttons::default_with_signal(t!("add")).on_click(add_class)
}

fn classes_view() -> impl Element {
    Column::new()
        .s(Gap::both(2))
        .items_signal_vec(classes2().signal_vec_cloned().map(|col| {
            Row::new()
            .s(Gap::new().x(2))
            .items_signal_vec(col.signal_vec_cloned().map(|row| {
                let a = Mutable::new(false);
                Column::new()
                    //.s(Align::center())
                    .s(Borders::all_signal(a.signal().map_bool(
                        || Border::new().width(1).color(BLUE_3).solid(),
                        || Border::new().width(1).color(BLUE_1).solid(),
                    )))
                    .s(RoundedCorners::all(2))
                    .s(Width::exact(140))
                    .s(Height::exact(75))
                    .s(Align::new().center_y())
                    .on_hovered_change(move |b| a.set(b))
                    .item(
                        Button::new().label(format!("{} {}", row.kademe, row.sube))
                    )
                    .item(
                        Button::new().label_signal(t!("delete")).on_press(move || del_class(row.id))
                    )
                //.on_click(move || super::teacher::open_modal(row.clone()))
            }))
        }))
}

#[static_ref]
pub fn classes() -> &'static MutableVec<Class> {
    get_classes();
    MutableVec::new()
}
#[static_ref]
fn branch() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

#[static_ref]
fn grade() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

#[static_ref]
fn selected_timetable() -> &'static Mutable<i32> {
    Mutable::new(0)
}

#[static_ref]
pub fn selected_timetable_hour() -> &'static MutableVec<i32> {
    MutableVec::new_with_values(vec![])
}

#[static_ref]
pub fn timetables() -> &'static MutableVec<Timetable> {
    get_timetables();
    MutableVec::new_with_values(vec![])
}
#[static_ref]
pub fn classes2() -> &'static MutableVec<MutableVec<Class>> {
    get_classes();
    MutableVec::new_with_values(vec![])
}

pub fn create_chunks() {
    let clss = classes().lock_mut().to_vec().into_iter().filter(|c| c.group_id == selected_timetable().get()).collect::<Vec<Class>>();
    let clss = clss
        .chunks(10)
        .map(|c| MutableVec::new_with_values(c.into()))
        .collect();
    classes2().lock_mut().replace_cloned(clss);
}

fn change_branch(value: String) {
    branch().set(value)
}

fn change_grade(value: String) {
    grade().set(value)
}

pub fn change_timetable(value: String) {
    let id = value.parse::<i32>().unwrap_or(0);
    if let Some(timetable) = timetables().lock_ref().iter().find(|t| t.id == id) {
        //println!("{}", &timetable.hour);
        selected_timetable_hour()
            .lock_mut()
            .replace_cloned(vec![0; timetable.hour as usize]);
    };
    selected_timetable().set(id);
    create_chunks();
}

fn del_class(id: i32) {
    //let id = id.parse::<i32>().unwrap();
    send_msg(UpMsg::Classes(ClassUpMsgs::DelClass(id)))
}

fn send_msg(msg: UpMsg) {
    use crate::connection::*;
    Task::start(async {
        match connection().send_up_msg(msg).await {
            Err(_error) => {}
            Ok(_msg) => (),
        }
    });
}
fn get_timetables() {
    use crate::connection::*;
    use shared::*;
    let tt_msg = TimetablesUpMsgs::GetTimetable;
    send_msg(UpMsg::Timetables(tt_msg));
}

fn get_classes() {
    use crate::connection::*;
    use shared::*;
    let msg = UpMsg::Classes(ClassUpMsgs::GetClasses);
    send_msg(msg)
}

fn class_form() -> AddClass {
    AddClass {
        kademe: grade().get_cloned(),
        sube: branch().get_cloned(),
        group_id: selected_timetable().get(),
    }
}

fn add_class() {
    use crate::connection::*;
    use shared::*;
    let form = class_form();
    if let Ok(_) = form.is_valid(){
        let msg = UpMsg::Classes(
            ClassUpMsgs::AddClass(form)
        );
        send_msg(msg);
    }
    
}
