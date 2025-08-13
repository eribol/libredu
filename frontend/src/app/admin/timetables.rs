use shared::{
    models::{
        class::{Class, ClassLimitation},
        teacher::{Teacher, TeacherLimitation},
        timetables::{Activity, Timetable},
    },
    msgs::admin::AdminUpMsgs,
    UpMsg,
};
use zoon::*;

use crate::{connection::send_msg, elements::buttons};

#[derive(Default, Clone)]
enum AdminPage {
    #[default]
    Classes,
    Teachers,
    Activities,
}

#[static_ref]
fn admin_page() -> &'static Mutable<AdminPage> {
    Mutable::new(AdminPage::default())
}
#[static_ref]
pub fn timetable() -> &'static Mutable<Option<Timetable>> {
    Mutable::new(None)
}

#[static_ref]
pub fn fix_class_lim() -> &'static Mutable<bool> {
    Mutable::new(false)
}
#[static_ref]
pub fn fix_teach_lim() -> &'static Mutable<bool> {
    Mutable::new(false)
}
#[static_ref]
pub fn fix_acts() -> &'static Mutable<bool> {
    Mutable::new(false)
}
#[static_ref]
pub fn classes() -> &'static MutableVec<Class> {
    MutableVec::new()
}
#[static_ref]
pub fn teachers() -> &'static MutableVec<Teacher> {
    MutableVec::new()
}
#[static_ref]
pub fn class_limitations() -> &'static MutableVec<ClassLimitation> {
    MutableVec::new_with_values(vec![])
}
#[static_ref]
pub fn teachers_limitations() -> &'static MutableVec<TeacherLimitation> {
    MutableVec::new_with_values(vec![])
}
#[static_ref]
pub fn activities() -> &'static MutableVec<Activity> {
    MutableVec::new()
}
#[static_ref]
pub fn lim_error() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

pub fn clear_data() {
    timetable().set(None);
    lim_error().set("".to_string());
    fix_acts().set(false);
    fix_class_lim().set(false);
    fix_teach_lim().set(false);
    class_limitations().lock_mut().clear();
    teachers_limitations().lock_mut().clear();
    activities().lock_mut().clear();
    use crate::app::admin::school::timetables;
    timetables().lock_mut().clear();
}

pub fn select_timetable(tt: Timetable) {
    timetable().set(Some(tt));
    get_teachers();
}
fn get_teachers() {
    let tt = timetable().get_cloned();
    if let Some(tt) = tt {
        let msg = AdminUpMsgs::GetTeachers(tt.id);
        let u_msg = UpMsg::Admin(msg);
        send_msg(u_msg);
        get_classes()
    }
}

fn get_classes() {
    let tt = timetable().get_cloned();
    if let Some(tt) = tt {
        let msg = AdminUpMsgs::GetClasses(tt.id);
        let u_msg = UpMsg::Admin(msg);
        send_msg(u_msg);
        get_activities();
    }
}

fn get_teachers_limitations() {
    let tt = timetable().get_cloned();
    if let Some(tt) = tt {
        let msg = AdminUpMsgs::GetTeachersLimitations(tt.id);
        let u_msg = UpMsg::Admin(msg);
        send_msg(u_msg);
        get_classes_limitations();
    }
}
fn get_activities() {
    let tt = timetable().get_cloned();
    if let Some(tt) = tt {
        let msg = AdminUpMsgs::GetActivities(tt.id);
        let u_msg = UpMsg::Admin(msg);
        send_msg(u_msg);
        get_teachers_limitations();
    }
}
fn get_classes_limitations() {
    let tt = timetable().get_cloned();
    if let Some(tt) = tt {
        let msg = AdminUpMsgs::GetClassesLimitations(tt.id);
        let u_msg = UpMsg::Admin(msg);
        send_msg(u_msg);
    }
}

pub fn root() -> impl Element {
    Row::new()
        .s(Padding::new().left(20).top(50))
        .s(Gap::new().x(50))
        .s(Align::new().left())
        .item(page())
        .item_signal(admin_page().signal_cloned().map(|page| match page {
            AdminPage::Classes => classes_page().into_raw(),
            AdminPage::Teachers => teachers_page().into_raw(),
            AdminPage::Activities => acts_page().into_raw(),
        }))
}
fn page() -> impl Element {
    Column::new()
        .item(buttons::_default("Sınıf Kısıtlamalar").on_click(|| {
            classes_lim_check();
            admin_page().set(AdminPage::Classes);
        }))
        .item(buttons::_default("Öğretmen Kısıtlamalar").on_click(|| {
            teachers_lim_check();
            admin_page().set(AdminPage::Teachers);
        }))
        .item(buttons::_default("Aktiviteler").on_click(|| {
            acts_check();
            admin_page().set(AdminPage::Activities);
        }))
}

fn classes_page() -> impl Element {
    Column::new()
        .item(
            Label::new()
                .label_signal(fix_class_lim().signal().map_bool(
                    || "Sınıf Kısıtlamaları Düzeltildi",
                    || "Sınıf Kısıtlamaları Düzeltilmedi",
                ))
                .s(Font::new()
                    .weight_signal(
                        fix_class_lim()
                            .signal()
                            .map_bool(|| FontWeight::Light, || FontWeight::Bold),
                    )
                    .color_signal(
                        fix_class_lim()
                            .signal()
                            .map_bool(|| color!("green"), || color!("red")),
                    )),
        )
        .items_signal_vec(
            class_limitations()
                .signal_vec_cloned()
                .map(|lim| Label::new().label(format!("{:?}", lim))),
        )
}
fn teachers_page() -> impl Element {
    Label::new()
        .label_signal(fix_teach_lim().signal().map_bool(
            || "Öğretmen Kısıtlamaları Düzeltildi",
            || "Öğretmen Kısıtlamaları Düzeltilmedi",
        ))
        .s(Font::new()
            .weight_signal(
                fix_teach_lim()
                    .signal()
                    .map_bool(|| FontWeight::Light, || FontWeight::Bold),
            )
            .color_signal(
                fix_teach_lim()
                    .signal()
                    .map_bool(|| color!("green"), || color!("red")),
            ))
}
fn acts_page() -> impl Element {
    Label::new()
        .label_signal(
            fix_acts()
                .signal()
                .map_bool(|| "Aktiviteler Düzeltildi", || "Aktiviteler Düzeltilmedi"),
        )
        .s(Font::new()
            .weight_signal(
                fix_acts()
                    .signal()
                    .map_bool(|| FontWeight::Light, || FontWeight::Bold),
            )
            .color_signal(
                fix_acts()
                    .signal()
                    .map_bool(|| color!("green"), || color!("red")),
            ))
}

fn class_lim_check(class: Class) -> bool {
    let t_len = timetable().get_cloned().unwrap().hour;
    let lims = class_limitations().lock_mut().to_vec();
    let c_lims = lims
        .iter()
        .find(|l| l.class_id == class.id.clone())
        .unwrap();
    let mut shifted = false;
    if c_lims.limitations.len() != 7 || c_lims.limitations.iter().any(|l| l.len() != t_len as usize)
    {
        shifted = true;
    }
    if shifted {
        lim_error().set(format!("{:?} class limitations have fault", class));
        return false;
    }
    true
}

fn classes_lim_check() {
    let classes = classes().lock_mut().to_vec();
    // let mut c = true;
    for class in classes {
        if !class_lim_check(class.clone()) {
            create_default_lim(class.id);
            // c = false;
        }
    }
    fix_class_lim().set(true)
}

fn create_default_lim(class_id: i32) {
    let t_len = timetable().get_cloned().unwrap().hour as usize;
    let mut lims: Vec<ClassLimitation> = vec![];
    for day in 1..=7 {
        let new_lim = ClassLimitation {
            class_id,
            // day,
            // hours: vec![true; t_len],
            limitations: vec![],
        };
        lims.push(new_lim);
    }
    let c_msg = AdminUpMsgs::UpdateClassLimitations(lims.clone());
    let msg = UpMsg::Admin(c_msg);
    send_msg(msg);
}
fn teacher_lim_check(teacher: Teacher) -> bool {
    let t_len = timetable().get_cloned().unwrap().hour;
    let lims = teachers_limitations().lock_mut().to_vec();
    let c_lims = lims
        .iter()
        .filter(|l| l.user_id == teacher.id.clone())
        .collect::<Vec<&TeacherLimitation>>();
    let mut shifted = false;
    for i in 1..=7 {
        if let Some(c_l) = c_lims.iter().find(|cl| cl.day == i) {
            if c_l.hours.len() < t_len as usize {
                shifted = true;
                break;
            }
        } else {
            shifted = true;
            break;
        }
    }
    if shifted {
        lim_error().set(format!("{:?} class limitations have fault", teacher));
        return false;
    }
    true
}
fn teachers_lim_check() {
    let tchrs = teachers().lock_mut().to_vec();
    // let mut c = true;
    for teacher in tchrs {
        if !teacher_lim_check(teacher.clone()) {
            create_default_lim_t(teacher.id);
            // c = false;
        }
    }
    fix_teach_lim().set(true);
}

fn create_default_lim_t(user_id: i32) {
    let tt = timetable().get_cloned().unwrap();
    let tt_len = tt.hour as usize;
    let school_id = super::school::school().get_cloned().unwrap().school.id;
    let mut lims: Vec<TeacherLimitation> = vec![];
    for day in 1..=7 {
        let new_lim = TeacherLimitation {
            school_id,
            group_id: tt.id,
            user_id,
            day,
            hours: vec![true; tt_len],
        };
        lims.push(new_lim);
    }
    let c_msg = AdminUpMsgs::UpdateTeacherLimitations(lims.clone());
    let msg = UpMsg::Admin(c_msg);
    send_msg(msg);
}

fn act_check(act: Activity) -> bool {
    let clss = classes().lock_mut().to_vec();
    let tchrs = teachers().lock_mut().to_vec();
    let t = act
        .teachers
        .iter()
        .all(|t| tchrs.iter().any(|t2| t2.id == *t));
    let c = act
        .classes
        .iter()
        .all(|c| clss.iter().any(|c2| c2.id == *c));
    t && c
}
fn acts_check() {
    let acts = activities().lock_mut().to_vec();
    for a in acts {
        if !act_check(a.clone()) {
            AdminUpMsgs::DelAct(a.id);
        }
    }
    fix_acts().set(true)
}
/*
fn create_teacher_lim(user_id: i32)-> Vec<TeacherLimitation>{
    let t_len = selected_timetable_hour().lock_mut().len();
    let group_id = selected_timetable().get();
    let mut lims: Vec<TeacherLimitation> = vec![];
    for day in 1..=7{
        let new_lim = TeacherLimitation{
            user_id,
            school_id: school().get_cloned().unwrap().id,
            group_id: selected_timetable().get(),
            day,
            hours: vec![true; t_len]
        };
        lims.push(new_lim);
    }
    let c_msg = TeacherUpMsgs::UpdateLimitations((group_id, lims.clone()));
    let t_msg = TimetableUpMsgs::Teacher(c_msg);
    let msg = UpMsg::Timetable(t_msg);
    send_msg(msg);
    lims
}
*/
