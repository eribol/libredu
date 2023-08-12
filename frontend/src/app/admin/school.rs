use shared::{msgs::admin::{AdminSchool, AdminUpMsgs}, models::timetables::Timetable, DownMsg, UpMsg};
use zoon::*;

use crate::{connection::send_msg, app::screen_width};

use super::timetables::select_timetable;

#[static_ref]
pub fn school()->&'static Mutable<Option<AdminSchool>>{
    Mutable::new(None)
}

#[static_ref]
pub fn timetables()->&'static MutableVec<Timetable>{
    get_timetables();
    MutableVec::new_with_values(vec![])
}

pub fn get_timetables(){
    let school = school().get_cloned();
    if let Some(school) = school{
        let msg = AdminUpMsgs::GetTimetables(school.school.id);
        send_msg(UpMsg::Admin(msg));    
    }
}

pub fn school_view(school: AdminSchool)-> impl Element{
    Column::new()
    .s(Width::exact_signal(screen_width().signal()))
    .item(title(&school))
    .item_signal(
        super::timetables::timetable()
        .signal_cloned()
        .map_option(|_tt| super::timetables::root().into_raw_element(), || timetables_view().into_raw_element())
    )
}

fn title(school: &AdminSchool)->impl Element{
    Row::new()
    .s(Align::center())
    .item(
        Button::new().label(&school.school.id.to_string()).on_click(||super::timetables::timetable().set(None)))
    .item_signal(
        super::timetables::timetable()
        .signal_cloned()
        .map_some(|tt|
            Button::new().label(&tt.name)
        )
    )
}

fn timetables_view()->impl Element{
    Column::new()
    .items_signal_vec(timetables().signal_vec_cloned().map(|tt|{
        Button::new().label(&tt.name).on_click(move || select_timetable(tt.clone()))
    }))
}