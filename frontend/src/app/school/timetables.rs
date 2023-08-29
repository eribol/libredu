use super::classes::timetables;
use shared::{models::timetables::{AddTimetable, TimetableSchedules, Timetable}, UpMsg};
use zoon::{named_color::*, *};
use crate::{i18n::t, elements::{text_inputs, buttons}, connection::send_msg};
use shared::msgs::timetables::*;

pub fn home() -> impl Element {
    Column::new()
        .s(Padding::new().top(10))
        .s(Gap::new().y(20))
        .item(form())
        .item(timetables_view())
        .item_signal(timetable_schedules().signal_cloned().map_some(|s|{
            Column::new()
            .s(Gap::new().y(20))
            .item(
                schedules_view(s)    
            ).item({
                let (a, _) = Mutable::new_and_signal(false);
                Button::new().label(
                    Label::new()
                    .s(Cursor::new(CursorIcon::Pointer))
                    .s(Align::center())
                    .label_signal(t!("update"))
                )
                .s(Borders::all_signal(a.signal().map_bool(
                    || Border::new().width(1).color(BLUE_5).solid(),
                    || Border::new().width(1).color(BLUE_1).solid(),
                )))
                .s(Height::exact(50))
                .s(Width::exact(100))
                .s(RoundedCorners::all(10))
                .s(Align::center().center_y())
                .s(Borders::all(Border::new().solid().color(BLUE_5)))
                .on_press(update_schedules)
                .on_hovered_change(move |h| a.set(h))
            })
        }))
}
fn timetables_view() -> impl Element {
    Row::new()
    .multiline()
    .s(Gap::both(5))
    .items_signal_vec(timetables().signal_vec_cloned().map(|tt| {
        let a = Mutable::new(false);
        Column::new()
        .s(Align::new().center_y())
        .s(Background::new()
            .color_signal(
                is_selected(tt.id).map_true(|| BLUE_1)
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
        .on_click(move ||{
            timetable_schedules().set(None);
            select_timetable(tt.id)
        })
        .update_raw_el(|raw| 
            raw.attr("title", &format!("{}", tt.name))
        )
        .item(
            Button::new()
            .label(tt.name)
        )
        .item_signal(
            crate::modals::del_signal(tt.id).map_bool(move ||
            crate::modals::del_modal_all(&tt.id.to_string(), UpMsg::Timetables(TimetablesUpMsgs::DelTimetable(tt.id))).into_raw_element(), move ||
            delete_view(tt.id).into_raw_element())
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
        .item(Label::new().label_signal(t!("timetable-name")).s(Align::center()))
        .item(text_inputs::default().id("name").on_change(change_name).s(Align::center()))
}

fn hour_view() -> impl Element {
    Column::new()
        .item(Label::new().label_signal(t!("hour-of-day")).s(Align::center()))
        .item(text_inputs::default().id("hour").on_change(change_hour))
}

fn update() -> impl Element {
    buttons::default_with_signal(t!("update")).on_click(add_timetable)
    
}
fn schedules_view(s: TimetableSchedules)->impl Element{
    Row::new()
    .s(Align::center())
    .item(
        Column::new()
        //.s(AlignContent::center())
        .s(Width::exact(150))
        .item(
            Label::new()
            .s(Borders::all(Border::new().width(1).color(BLUE_2)))
            .s(Height::exact(25))
            .label_signal(t!("hours"))
        )
        .items((0..7).map(|i|{
            Label::new().label(i+1)
            .s(Height::exact(25))
            .s(Borders::all(Border::new().width(1).color(BLUE_2)))
        }))
    )
    .item(
        Column::new()
        .s(Width::exact(150))
        .item(
            Label::new()
            .s(Borders::all(Border::new().width(1).color(BLUE_2)))
            .s(Height::exact(25))
            .label_signal(t!("start-times"))
        )
        .items(
            s.starts.iter().enumerate().map(|ss|{
                TextInput::new().id("a")
                .s(Height::exact(25))
                .s(Borders::all(Border::new().width(1).color(BLUE_2)))
                .text(ss.1.format("%H:%M").to_string())
                .update_raw_el(|raw|{
                    raw.attr("type", "time")
                })
                .on_change(|s| change_string(s))
                .on_focused_change(move |s| {
                    if !s{
                        change_start_schedules(ss.0)    
                    }
                })
            })
        )
    )
    .item(
        Column::new()
        .s(Width::exact(150))
        .item(
            Label::new()
            .s(Borders::all(Border::new().width(1).color(BLUE_2)))
            .s(Height::exact(25))
            .label_signal(t!("ends_time"))
        )
        .items(
            s.ends.iter().enumerate().map(|ss|{
                TextInput::new().id("a")
                .s(Height::exact(25))
                .s(Borders::all(Border::new().width(1).color(BLUE_2)))
                .text(ss.1.format("%H:%M").to_string())
                .update_raw_el(|raw|{
                    raw.attr("type", "time")
                })
                .on_change(|s| change_string(s))
                .on_focused_change(move |s| {
                    if !s{
                        change_end_schedules(ss.0)    
                    }
                })
            })
        )  
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
#[static_ref]
fn selected_timetable() -> &'static Mutable<Option<i32>> {
    Mutable::new(None)
}

#[static_ref]
pub fn timetable_schedules() -> &'static Mutable<Option<TimetableSchedules>> {
    Mutable::new(None)
}
#[static_ref]
pub fn date_string() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}
#[static_ref]
fn hide() -> &'static Mutable<bool> {
    Mutable::new(false)
}
fn is_selected(id: i32)->impl Signal<Item = bool>{
    selected_timetable().signal().map_option(move |s| s == id, || false).dedupe()
}
fn change_string(s: String){
    date_string().set(s);
}

fn change_start_schedules(index: usize){
    timetable_schedules().update_mut(|ts|{
        if let Some(s) = ts.as_mut(){
            let new_date = date_string().get_cloned();
            let t = NaiveTime::parse_from_str(&new_date, "%H:%M");
            if let Ok(t) = t{
                s.starts[index] = t     
            }
        }
    });
    change_string("".to_string())
}
fn change_end_schedules(index: usize){
    timetable_schedules().update_mut(|ts|{
        if let Some(s) = ts.as_mut(){
            let new_date = date_string().get_cloned();
            let t = NaiveTime::parse_from_str(&new_date, "%H:%M");
            if let Ok(t) = t{
                s.ends[index] = t     
            }
        }
    });
    change_string("".to_string())
}
fn select_timetable(id: i32){
    
    if let Some(_id) = selected_timetable().get_cloned(){
        if id == _id{
            clear_data();    
        }
        else{
            let tt = timetables().lock_mut().to_vec();
            let tt = tt.into_iter().find(|t| t.id == id).unwrap();
            create_selected(tt);
        }
    }
    else{
        let tt = timetables().lock_mut().to_vec();
        let tt = tt.into_iter().find(|t| t.id == id).unwrap();
        create_selected(tt);
        get_schedules(id);
    }
}
fn create_selected(tt: Timetable){
    hide().set(false);
    name().set(tt.name);
    hour().set(tt.hour);
    selected_timetable().set(Some(tt.id));
}
fn clear_data(){
    hide().set(false);
    name().set("".to_string());
    hour().set(0);
    selected_timetable().set(None);
    timetable_schedules().take();
}
fn get_schedules(id: i32){
    use crate::connection::*;
    use shared::*;
    let msg = UpMsg::Timetables(TimetablesUpMsgs::GetSchedules(id));
    send_msg(msg)
}
fn update_schedules(){
    let sch = timetable_schedules().get_cloned().unwrap();
    let t_msg = TimetablesUpMsgs::UpdateSchedules(sch);
    let msg = UpMsg::Timetables(t_msg);
    send_msg(msg);
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
