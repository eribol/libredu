use crate::connection::send_msg;
use crate::elements::text_inputs;
use crate::i18n::t;
use shared::msgs::teachers::TeacherUpMsgs;
use shared::{
    models::teacher::{AddTeacher, Teacher},
    UpMsg,
};
use zoon::{named_color::*, *, println};

pub fn home() -> impl Element {
    Column::new()
        .s(Padding::new().top(10))
        .s(Gap::new().y(5))
        .item_signal(hide().signal().map_false(form))
        .item(hide_and_seek())
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
        .item(buttons())
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
                .text(first_name().get_cloned())
                .on_change(change_first_name),
        )
        .item_signal(
            first_name_error()
            .signal_cloned()
            .map_some(
                |e| Label::new().label(e).s(Font::new().color(RED_5).weight(FontWeight::Light)).s(Align::center())
            )
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
                .text(last_name().get_cloned())
                .on_change(change_last_name),
        )
        .item_signal(
            last_name_error()
            .signal_cloned()
            .map_some(
                |e| Label::new().label(e).s(Font::new().color(RED_5).weight(FontWeight::Light)).s(Align::center())
            )
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
                .text(short_name().get_cloned())
                .on_focused_change(|_|() )
                .on_change(change_short_name),
        )
        .item_signal(
            short_name_error()
            .signal_cloned()
            .map_some(
                |e| Label::new().label(e).s(Font::new().color(RED_5).weight(FontWeight::Light)).s(Align::center())
            )
        )
}

fn buttons()->impl Element{
    Column::new()
    .s(Gap::new().y(5))
    .item(add())
}
fn add() -> impl Element {
    Button::new()
    .label_signal(
        selected_teacher()
        .signal()
        .map_option(|_| Label::new().label_signal(t!("update")).on_click(update_teacher),||
            Label::new().label_signal(t!("add")).on_click(add_teacher)
        )
    )
}

fn hide_and_seek()->impl Element{
    let (a, _b) = Mutable::new_and_signal(false);
    Button::new()
    .s(Borders::all_signal(a.signal().map_bool(||
        Border::new().width(1).color(BLUE_5).solid(), ||
        Border::new().width(1).color(BLUE_1).solid()
    )))
    .s(Width::exact_signal(
        crate::app::screen_width().signal().map(|a| a / 4),
    ))
    .s(Height::exact(25))
    .s(Align::center())
    .on_hovered_change(move |h| a.set(h))
    .label_signal(hide()
        .signal()
        .map_bool(|| 
            Label::new().label_signal(t!("seek")),|| 
            Label::new().label_signal(t!("hide")))
    ).on_click(|| hide().set(!hide().get()))
}

fn teachers_view() -> impl Element {
    Row::new()
    .multiline()
    .s(Gap::both(5))
    .items_signal_vec(teachers().signal_vec_cloned().map(|teacher| {
        let a = Mutable::new(false);
        Column::new()
        .s(Align::new().center_y())
        .s(Background::new()
            .color_signal(
                is_selected(teacher.id).map_true(|| BLUE_3)
            )
        )
        .s(Borders::all_signal(a.signal().map_bool(
            || Border::new().width(1).color(BLUE_3).solid(),
            || Border::new().width(1).color(BLUE_1).solid(),
        )))
        .s(RoundedCorners::all(2))
        .s(Width::exact(140))
        .s(Height::exact(75))
        .on_hovered_change(move |b| a.set(b))
        .on_click(move ||select_teacher(teacher.id))
        .update_raw_el(|raw| 
            raw.attr("title", &format!("{} {}", teacher.first_name, teacher.last_name))
        )
        .item(
            Button::new()
            .label(teacher.short_name.to_string())
        )
        .item_signal(
            crate::modals::del_signal(teacher.id).map_bool(move ||
            crate::modals::del_modal_all(&teacher.id.to_string(), UpMsg::Teachers(TeacherUpMsgs::DelTeacher(teacher.id))).into_raw_element(), move ||
            delete_view(teacher.id).into_raw_element())
        )
    }))
}
fn delete_view(id: i32)->impl Element{
    let a = Mutable::new_and_signal(false);
    Button::new()
    .s(Font::new()
    .weight_signal(a.0.signal().map_bool(|| FontWeight::Regular, || FontWeight::ExtraLight))
    .color_signal(a.0.signal().map_bool(|| RED_8, || RED_4)))
    .s(Align::new().bottom().center_x())
    .on_hovered_change(move |h| a.0.set_neq(h))
    .label_signal( t!("delete"))
    .update_raw_el(|raw| raw.event_handler(move |event: events::Click|{
        crate::modals::del_modal().set(Some(id));
        event.stop_propagation();
    }))
}
#[static_ref]
fn first_name() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}
#[static_ref]
fn first_name_error() -> &'static Mutable<Option<String>> {
    Mutable::new(None)
}
#[static_ref]
fn last_name() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}
#[static_ref]
fn last_name_error() -> &'static Mutable<Option<String>> {
    Mutable::new(None)
}
#[static_ref]
fn short_name() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}
#[static_ref]
fn short_name_error() -> &'static Mutable<Option<String>> {
    Mutable::new(None)
}

#[static_ref]
fn role() -> &'static Mutable<i32> {
    Mutable::new(0)
}

#[static_ref]
fn hide() -> &'static Mutable<bool> {
    Mutable::new(false)
}

#[static_ref]
fn selected_teacher() -> &'static Mutable<Option<i32>> {
    Mutable::new(None)
}
fn select_teacher(id: i32){
    if let Some(_id) = selected_teacher().get_cloned(){
        if id == _id{
            clear_data();    
        }
        else{
            let teacher = teachers().lock_mut().to_vec();
            let teacher = teacher.into_iter().find(|t| t.id == id).unwrap();
            create_selected(teacher);
        }
    }
    else{
        let teacher = teachers().lock_mut().to_vec();
        let teacher = teacher.into_iter().find(|t| t.id == id).unwrap();
        create_selected(teacher);
    }
}
fn create_selected(teacher: Teacher){
    hide().set(false);
    last_name().set(teacher.last_name);
    first_name().set(teacher.first_name);
    short_name().set(teacher.short_name);
    selected_teacher().set(Some(teacher.id));
}
fn clear_data(){
    hide().set(false);
    last_name().set("".to_string());
    first_name().set("".to_string());
    short_name().set("".to_string());
    selected_teacher().set(None);
}
fn is_selected(id: i32)->impl Signal<Item = bool>{
    selected_teacher().signal().map_option(move |s| s == id, || false).dedupe()
}
fn change_first_name(value: String) {
    first_name_error().set(None);
    first_name().set(value)
}

fn change_last_name(value: String) {
    last_name_error().set(None);
    last_name().set(value)
}

fn change_short_name(value: String) {
    short_name_error().set(None);
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

fn validate_form()->bool{
    let form = teacher_form();
    if let Err(_e) = form.is_valid(){
        if form.has_error("first_name"){
            first_name_error().set(Some("Firstname is not valid".to_string()));
        }
        if form.has_error("last_name"){
            last_name_error().set(Some("Lastname is not valid".to_string()));
        }
        if form.has_error("short_name"){
            short_name_error().set(Some("Short name is not valid".to_string()));
        }
        return false
    }
    true
}

fn add_teacher() {
    use crate::connection::*;
    use shared::*;
    let form = teacher_form();
    if validate_form(){
        println!("update");
        let t_msg = TeacherUpMsgs::AddTeacher(form);
        let msg = UpMsg::Teachers(t_msg);
        send_msg(msg)
    }
}
fn update_teacher(){
    let form = teacher_form();
    if validate_form(){
        let id = selected_teacher().get_cloned().unwrap();
        let teacher = Teacher{
            id,
            first_name: form.first_name,
            last_name: form.last_name,
            short_name: form.short_name
        };
        let t_msg = TeacherUpMsgs::UpdateTeacher(teacher);
        let msg = UpMsg::Teachers(t_msg);
        send_msg(msg)
    }
}
#[static_ref]
pub fn teachers() -> &'static MutableVec<Teacher> {
    get_teachers();
    MutableVec::new_with_values(vec![])
}

pub fn get_teachers() {
    let t_msg = TeacherUpMsgs::GetTeachers;
    send_msg(UpMsg::Teachers(t_msg))
}
