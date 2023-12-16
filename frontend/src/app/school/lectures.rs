use shared::UpMsg;
use shared::models::lectures::{AddLecture, Lecture};
use shared::msgs::lectures::LecturesUpMsg;
use zoon::named_color::*;
use zoon::{println, *};

use crate::connection::send_msg;
use crate::modals::{del_modal, del_signal};
use crate::{
    app::screen_width,
    elements::text_inputs,
    i18n::{t, t_s},
};

pub fn home() -> impl Element {
    Column::new()
        //.s(Align::center())
        .s(Padding::new().top(10))
        .s(Gap::new().y(20))
        .item_signal(
            hide().signal().map_false(form)
        )
        .item(
            hide_and_seek()
        )
        .item(lectures_view())
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
        .item(buttons())
}

fn grade_view() -> impl Element {
    Column::new()
    .s(Align::center())
    .item(Column::new()
        .item(Label::new().label_signal(t!("lecture-grade")).s(Align::center()))
        .item(Label::new()
            .s(Align::center())
            .s(Font::new().weight(FontWeight::ExtraLight))
            .label("Boşluk kullanarak birden çok kademe türünde ekleyebilirsiniz")
        )
    )
    .item(text_inputs::default().id("grade").on_change(change_grade).text(grade().get_cloned()))
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
    .item(Label::new().label_signal(t!("lecture-name")).s(Align::center()))
    .item(
        text_inputs::default()
        .id("name")
        .text(name().get_cloned())
        .on_change(change_name)
    )
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
    .item(Label::new().label_signal(t!("lecture-shortname")).s(Align::center()))
    .item(
        text_inputs::default()
        .id("short_name")
        .text(short_name().get_cloned())
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

fn buttons()->impl Element{
    Column::new()
    .s(Gap::new().y(5))
    .on_click(||{
        if selected_lecture().get_cloned().is_some(){
            update_lecture()
        }
        else{
            add_lecture()
        }
    })
    .item(add())
}
fn add() -> impl Element {
    let (a, _b) = Mutable::new_and_signal_cloned(false);
    Button::new()
    .s(Width::fill())
    .s(Height::exact(50))
    .s(Cursor::new(CursorIcon::Pointer))
    .s(Borders::all_signal(a.signal().map_bool(
        || Border::new().width(1).color(BLUE_5).solid(),
        || Border::new().width(1).color(BLUE_1).solid(),
    )))
    .label_signal(
        selected_lecture()
        .signal()
        .map_option(|_| 
            t_s!("update"),||
            t_s!("add")
        )
    )
}

fn lectures_view() -> impl Element {
    Row::new()
    .s(Padding::new().bottom(50).right(50).left(50))
    .s(Gap::new().x(10).y(10))
    .multiline()
    .items_signal_vec(lectures().signal_vec_cloned().map(|r| {
        let a = Mutable::new(false);
        Column::new()
        .s(Borders::all_signal(a.signal().map_bool(
            || Border::new().width(1).color(BLUE_3).solid(),
            || Border::new().width(1).color(BLUE_1).solid(),
        )))
        .s(RoundedCorners::all(2))
        .s(Width::exact(100))
        .s(Height::exact(50))
        .s(Background::new().color_signal(is_selected(r.id).map_true(|| BLUE_3)))
        .on_hovered_change(move |b| a.set(b))
        .on_click(move ||select_lecture(r.id))
        .item(
            Button::new()
            .s(Align::new().center_y())
            .s(Font::new().weight(FontWeight::Light))
            .label(format!("{} ({})", r.short_name, r.kademe))
        )
        .item_signal(
            del_signal(r.id)
            .map_bool(move || 
                crate::modals::del_modal_all(&r.id.to_string(), UpMsg::Lectures(LecturesUpMsg::DelLecture(r.id))).into_raw(), 
                move || delete_view(r.id).into_raw()
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
    .s(Align::new().bottom().center_x())
    .on_hovered_change(move |h| a.0.set_neq(h))
    .label_signal( t!("delete"))
    .update_raw_el(|raw| raw.event_handler(move |event: events::Click|{
        crate::modals::del_modal().set(Some(id));
        event.stop_propagation();
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
#[static_ref]
fn hide() -> &'static Mutable<bool> {
    Mutable::new(false)
}

#[static_ref]
fn selected_lecture() -> &'static Mutable<Option<i32>> {
    Mutable::new(None)
}
fn select_lecture(id: i32){
    if let Some(_id) = selected_lecture().get_cloned(){
        if id == _id{
            clear_data();    
        }
        else{
            let lectures = lectures().lock_mut().to_vec();
            let lecture = lectures.into_iter().find(|t| t.id == id).unwrap();
            create_selected(lecture);
        }
    }
    else{
        let lecture = lectures().lock_mut().to_vec();
        let lecture = lecture.into_iter().find(|t| t.id == id).unwrap();
        create_selected(lecture);
    }
}
fn create_selected(lecture: Lecture){
    hide().set(false);
    name().set(lecture.name);
    short_name().set(lecture.short_name);
    grade().set(lecture.kademe);
    selected_lecture().set(Some(lecture.id));
}
fn clear_data(){
    hide().set(false);
    name().set("".to_string());
    short_name().set("".to_string());
    grade().set("".to_string());
    selected_lecture().set(None);
}
fn is_selected(id: i32)->impl Signal<Item = bool>{
    selected_lecture().signal().map_option(move |s| s == id, || false).dedupe()
}
fn change_name(value: String) {
    name_error().set(None);
    name().set(value)
}
fn change_short_name(value: String) {
    short_name_error().set(None);
    short_name().set(value)
}
fn change_grade(value: String) {
    grade_error().set(None);
    grade().set(value)
}

pub fn get_lectures() {
    use shared::*;
    let msg = UpMsg::Lectures(msgs::lectures::LecturesUpMsg::GetLectures);
    send_msg(msg);
}
fn lecture_form()->AddLecture{
    let form = AddLecture {
            kademe: grade().get_cloned(),
            name: name().get_cloned(),
            short_name: short_name().get_cloned(),
    };
    form
}
fn validate_form()->bool{
    let form = lecture_form();
    if let Err(_e) = form.is_valid(){
        if form.has_error("name"){
            name_error().set(Some("Lecture name is not valid".to_string()));
        }
        if form.has_error("short_name"){
            short_name_error().set(Some(" Shortname is not valid".to_string()));
        }
        if form.has_error("grade"){
            grade_error().set(Some("Grade is not valid".to_string()));
        }
        return false
    }
    true
}
fn add_lecture() {
    use crate::connection::*;
    use shared::*;
    if validate_form(){
        let kademe = lecture_form().kademe;
        kademe.split(" ").for_each(|k|{
            let mut form = lecture_form();
            form.kademe = k.to_string();
            let msg = LecturesUpMsg::AddLecture(form);
            send_msg(UpMsg::Lectures(msg));
        })  
    }
}
fn update_lecture() {
    use crate::connection::*;
    use shared::*;
    if validate_form(){
        let form = lecture_form();
        let f = Lecture{
            id: selected_lecture().get().unwrap(),
            name: form.name,
            short_name: form.short_name,
            kademe: form.kademe
        };
        println!("update lecture");
        let t_msg = LecturesUpMsg::UpdateLecture(f);
        let msg = UpMsg::Lectures(t_msg);
        send_msg(msg)
    }
}
