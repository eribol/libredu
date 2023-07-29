use super::classes::timetables;
use shared::{models::timetables::{AddTimetable, TimetableSchedules}, UpMsg};
use zoon::{named_color::*, *, text_input::InputTypeText};
use crate::{i18n::t, elements::{text_inputs, buttons}, connection::send_msg};
use shared::msgs::timetables::*;

pub fn home() -> impl Element {
    Column::new()
        //.s(Padding::new().top(10))
        //.s(Gap::new().y(20))
        .item(form())
        .item(timetables_view())
        .item_signal(timetable_schedules().signal_cloned().map_some(|s|{
            Column::new().item(
                schedules_view(s)    
            ).item(
                Button::new().label("Update Schedules")
                .on_click(update_schedules)
            )
        }))
}
fn schedules_view(s: TimetableSchedules)->impl Element{
    Row::new()
    .item(
        Column::new()
        //.s(AlignContent::center())
        .s(Width::exact(50))
        .item(
            Label::new()
            .s(Borders::all(Border::new().width(1).color(BLUE_2)))
            .s(Height::exact(25))
            .label("Hour")
        )
        .items((0..7).map(|i|{
            Label::new().label(i)
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
            .label("Starts")
        )
        .items(
            s.starts.iter().enumerate().map(|ss|{
                TextInput::new().id("a")
                .s(Height::exact(25))
                .s(Borders::all(Border::new().width(1).color(BLUE_2)))
                .text(ss.1.to_string())
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
            .label("Ends Time")
        )
        .items(
            s.ends.iter().enumerate().map(|ss|{
                TextInput::new().id(ss.0)
                .s(Height::exact(25))
                .s(Borders::all(Border::new().width(1).color(BLUE_2)))
                .text(ss.1.to_string())
                .on_change(move |s| change_end_schedules(ss.0, s))
            })
        )  
    )
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
    Row::new()
    .items_signal_vec(
        timetables()
        .signal_vec_cloned()    
        .map(|timetable| {
            let a = Mutable::new(false);
            Column::new()
            .element_below_signal(
                crate::modals::del_signal(timetable.id).map_true(move ||
                crate::modals::del_modal_all(&timetable.id.to_string(), UpMsg::Timetables(TimetablesUpMsgs::DelTimetable(timetable.id))))
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
                get_schedules(timetable.id);
                selected_timetable().set(Some(timetable.id))
            })
            .item(Label::new().label(timetable.name).s(Align::center()))
            .item({
                let a = Mutable::new_and_signal(false);
                Button::new()
                .s(Font::new()
                .weight_signal(a.0.signal().map_bool(|| FontWeight::Regular, || FontWeight::ExtraLight))
                .color_signal(a.0.signal().map_bool(|| RED_8, || RED_4)))
                .s(Align::new().bottom())
                .on_hovered_change(move |h| a.0.set_neq(h))
                .label_signal(t!("delete")).s(Align::center()).on_click(move|| crate::modals::del_modal().set(Some(timetable.id)))
            })
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

fn change_string(s: String){
    date_string().set(s);
}

fn change_start_schedules(index: usize){
    timetable_schedules().update_mut(|ts|{
        if let Some(s) = ts.as_mut(){
            let new_date = date_string().get_cloned();
            let t = NaiveTime::parse_from_str(&new_date, "%H:%M:%S");
            if let Ok(t) = t{
                s.starts[index] = t     
            }
        }
    });
    change_string("".to_string())
}
fn change_end_schedules(index: usize, new: String){
    timetable_schedules().update_mut(|ts|{
        if let Some(s) = ts.as_mut(){
            let t = NaiveTime::parse_from_str(&new, "%H:%M:%S");
            if let Ok(t) = t{
                s.ends[index] = t     
            }
        }
    });
}

fn get_schedules(id: i32){
    use crate::connection::*;
    use shared::*;
    let msg = UpMsg::Timetables(TimetablesUpMsgs::GetSchedules(id));
    send_msg(msg)
}
fn update_schedules(){
    let t_msg = TimetablesUpMsgs::UpdateSchedules(selected_timetable().get().unwrap(), timetable_schedules().get_cloned().unwrap());
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
