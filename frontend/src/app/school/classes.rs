use crate::{
    elements::{buttons, text_inputs},
    i18n::t, modals::del_signal,
};
use shared::{
    models::{
        class::{AddClass, Class},
        timetables::Timetable,
    },
    msgs::{classes::ClassUpMsgs, timetables::TimetablesUpMsgs},
    UpMsg,
};
use zoon::named_color::*;
use zoon::*;

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
        .item_signal(
            add_class_error().signal_cloned().map_some(|e| 
                Label::new()
                .s(Font::new().weight(FontWeight::Light))
                .s(Font::new().color(RED_5))
                .s(Align::new().center_x())
                .label(e)
            )
        )
}
fn groups_view() -> impl Element {
    Column::new()
        .s(Align::center())
        .item(Label::new().label_signal(t!("timetable")))
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
            ))
        )
}

fn grade_view() -> impl Element {
    Column::new()
        .s(Align::center())
        .item(Column::new()
            .item(Label::new().label_signal(t!("grade")).s(Align::center()))
            .item(Label::new()
                .s(Align::center())
                .s(Font::new().weight(FontWeight::ExtraLight))
                .label("Boşluk kullanarak birden çok kademe ekleyebilirsiniz")
            )
        )
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
        .item(
            Column::new()
            .item(Label::new().label_signal(t!("branch")).s(Align::center()))
            .item(Label::new()
                .s(Align::center())
                .s(Font::new().weight(FontWeight::ExtraLight))
                .label("Boşluk kullanarak birden çok sınıf şubesi ekleyebilirsiniz")
            )
        )
        .item(text_inputs::default().on_change(change_branch).id("sube"))
}
fn update() -> impl Element {
    buttons::default_with_signal(t!("add")).on_click(add_class)
}

fn classes_view() -> impl Element {
    Row::new()
    .s(Gap::new().x(2))
    .multiline()
    .items_signal_vec(
        classes().signal_vec_cloned()
        .filter_signal_cloned(|c| is_timetable_selected(c.group_id))
        .map(|row| {
        let a = Mutable::new(false);
        Column::new()
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
            Button::new().label(format!("{} {} {}", row.kademe, row.sube, row.group_id))
        )
        .item_signal(
            del_signal(row.id)
            .map_bool(move||
                crate::modals::del_modal_all(&row.id.to_string(), UpMsg::Classes(ClassUpMsgs::DelClass(row.id))).into_raw_element(),move||
                delete_view(row.id).into_raw_element()
            )
        )
    }))
}

fn delete_view(id: i32)->impl Element{
    let a = Mutable::new_and_signal(false);
    Button::new()
    .s(Font::new()
        .weight_signal(a.0.signal().map_bool(|| FontWeight::Regular, || FontWeight::ExtraLight))
        .color_signal(a.0.signal().map_bool(|| RED_8, || RED_4)))
    .s(Align::new().bottom())
    .label_signal(t!("delete")).on_press(move || crate::modals::del_modal().set(Some(id)))
    .on_hovered_change(move|h| a.0.set_neq(h))
        
}

#[static_ref]
pub fn add_class_error() -> &'static Mutable<Option<String>> {
    Mutable::new(None)
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

fn is_timetable_selected(group_id: i32)->impl Signal<Item=bool>{
    selected_timetable().signal_ref(move|t| t==&group_id).dedupe()
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
    let group_id = form.group_id;
    if let Ok(_) = form.is_valid(){
        form.kademe.split(" ").for_each(|k|{
            form.sube.split(" ").for_each(|s|{
                let f = AddClass{
                    group_id,
                    sube: s.to_string(),
                    kademe: k.to_string()
                };
                let msg = UpMsg::Classes(ClassUpMsgs::AddClass(f));
                send_msg(msg);
            })
        })
        
    }
    
}
