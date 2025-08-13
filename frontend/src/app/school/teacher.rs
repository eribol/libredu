use shared::{
    models::teacher::{Teacher, TeacherLimitation},
    models::timetables::Activity,
};
use zoon::*;

use super::{classes::selected_timetable_hour, lectures::lectures};
use crate::i18n::t;
use crate::{app::school::teachers::teachers, DAYS};

pub fn home(hamburger_id: &str) -> impl zoon::Element {
    run_once!(|| {
        global_styles().style_group(StyleGroup::new(".below > *").style("pointer-events", "auto"));
    });
    let id = format!("{}{}", "id", hamburger_id);
    zoon::Column::new()
        .id(&id)
        .s(crate::Background::new().color(RED_0))
        .s(zoon::Width::exact(150))
        .s(zoon::Align::new().right())
        .s(zoon::Padding::all(5))
        //.on_key_down_event(|event| event.if_key(Key::Escape, close_modal))
        .item(modal_header())
        .item(schedule_table())
        .items_signal_vec(activities().signal_vec_cloned().map(|act| {
            Column::new()
                .item(Button::new().label_signal(lecture_name(act.clone())))
                .item(Button::new().label_signal(teachers_names(act.clone().teachers)))
        }))
        .update_raw_el(|raw_el| {
            raw_el
                .style("display", "flex")
                //.style("flex-direction", )
                .style("position", "absolute")
                .style("top", "5%")
                .style("left", "5%")
                .style("height", "1000px")
                .style_signal(
                    "background-color",
                    visibile().signal_cloned().map(|v| v.background_color),
                )
                .style_signal(
                    "visibility",
                    visibile().signal_cloned().map(|v| v.visibility),
                )
                .style_signal("opacity", visibile().signal_cloned().map(|v| v.opacity))
                .style("width", "95%")
                //.style("pointer-events", "none")
                .style_signal("z-index", visibile().signal_cloned().map(|v| v.z_index))
        })
}

fn modal_header() -> impl Element {
    Row::new()
        .item(
            Label::new()
                .label_signal(teacher_modal().signal_cloned().map_option(
                    |class| (class.first_name + &class.last_name).into_cow_str(),
                    || "".into_cow_str(),
                ))
                .s(Align::center()),
        )
        .item(
            Row::new()
                .s(Gap::new().x(10))
                .on_click(close_modal)
                .s(Align::new().right())
                .item(
                    Button::new()
                        //.s(Align::new().right())
                        .label("Kapat"), //.on_press(close_modal),
                )
                .item(RawHtmlEl::new("i").attr("class", "fa-solid fa-circle-xmark")),
        )
}

fn schedule_table() -> impl Element {
    Row::new()
        .s(Align::new().center_x())
        .s(Padding::new().top(10))
        .item(
            Column::new()
                .item(
                    Button::new()
                        .s(Height::exact(50))
                        .s(Width::exact(100))
                        .label("Günler/Saatler")
                        .s(Borders::all(Border::new().width(1).solid().color(BLUE_3))),
                )
                .items_signal_vec(
                    selected_timetable_hour()
                        .signal_vec_cloned()
                        .enumerate()
                        .map(|hour| {
                            Button::new()
                                .label(hour.0.get().unwrap() + 1)
                                .s(Height::exact(50))
                                .s(Width::exact(100))
                                .s(Borders::new()
                                    .bottom(Border::new().width(1).solid().color(BLUE_3))
                                    .left(Border::new().width(1).solid().color(BLUE_3))
                                    .right(Border::new().width(1).solid().color(BLUE_3)))
                        }),
                ),
        )
        .items(DAYS.map(|_day| {
            Column::new()
                .item(
                    Button::new()
                        .label_signal(t!(_day))
                        .s(Borders::all(Border::new().width(1).color(BLUE_3).solid()))
                        .s(Width::exact(100))
                        .s(Height::exact(50)),
                )
                .items_signal_vec(
                    selected_timetable_hour()
                        .signal_vec_cloned()
                        .enumerate()
                        .map(|_hour| {
                            Button::new()
                                .s(Width::exact(100))
                                .s(Height::exact(50))
                                .label("")
                                .s(Borders::new()
                                    .bottom(Border::new().width(1).solid().color(BLUE_3))
                                    //.left(Border::new().width(2).solid().color(BLUE_3))
                                    .right(Border::new().width(1).solid().color(BLUE_3)))
                        }),
                )
        }))
        .item(Button::new().label("Add a hour"))
}

pub fn open_modal(teacher: Teacher) {
    let mut modal = visibile().get_cloned();
    modal.opacity = "1".to_string();
    modal.visibility = "visible".to_string();
    modal.z_index = "101".to_string();
    modal.transition = "opacity 0.5s, visibility 0s 0.5s".to_string();
    modal.background_color = "#0ff".to_string();
    visibile().set(modal);
    teacher_modal().set(Some(teacher))
}

fn close_modal() {
    let modal = Modal::default();
    visibile().set(modal);
    teacher_modal().set(None)
}
#[static_ref]
pub fn teacher_modal() -> &'static Mutable<Option<Teacher>> {
    Mutable::new(None)
}

#[static_ref]
fn visibile() -> &'static Mutable<Modal> {
    Mutable::new(Modal {
        z_index: "100".to_string(),
        background_color: "#0ff".to_string(),
        visibility: "hidden".to_string(),
        transition: "opacity 0.5s".to_string(),
        opacity: "0".to_string(),
    })
}

#[static_ref]
pub fn limitations() -> &'static MutableVec<TeacherLimitation> {
    get_limitations();
    MutableVec::new_with_values(vec![])
}

#[static_ref]
pub fn activities() -> &'static MutableVec<Activity> {
    get_activities();
    MutableVec::new_with_values(vec![])
}

fn teachers_names(ids: Vec<i32>) -> impl Signal<Item = String> {
    let mut names = "".to_string();
    let tchrs = teachers().lock_mut().to_vec();
    tchrs
        .iter()
        .filter(|t1| ids.iter().any(|t2| t2 == &t1.id))
        .for_each(|t| names.push_str(&format!("{} {}", t.first_name, t.last_name)));
    Mutable::new_and_signal_cloned(names).1
}
fn lecture_name(act: Activity) -> impl Signal<Item = String> {
    let mut name = "".to_string();
    let tchrs = lectures().lock_mut().to_vec();
    tchrs
        .iter()
        .filter(|t1| t1.id == act.lecture)
        .for_each(|t| name.push_str(&format!("{}", t.name)));
    Mutable::new_and_signal_cloned(name).1
}
fn get_limitations() {
    Task::start(async {
        let id = teacher_modal().get_cloned().unwrap().id;
        let msg = shared::UpMsg::Classes(shared::msgs::classes::ClassUpMsgs::GetLimitations(id));
        match crate::connection::connection().send_up_msg(msg).await {
            Err(_error) => {}
            Ok(_msg) => (), //println!("{:?}", msg),
        }
    });
}
fn get_activities() {
    use shared::msgs::teachers::*;
    use shared::*;
    let id = teacher_modal().get_cloned().unwrap().id;
    let t_msg = TeacherUpMsgs::GetActivities(id);
    let msg = UpMsg::Teachers(t_msg);
    crate::connection::send_msg(msg);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Modal {
    z_index: String,
    background_color: String,
    visibility: String,
    opacity: String,
    transition: String,
}

impl Default for Modal {
    fn default() -> Self {
        Self {
            z_index: "100".to_string(),
            background_color: "#000".to_string(),
            visibility: "hidden".to_string(),
            transition: "".to_string(),
            opacity: "0".to_string(),
        }
    }
}
