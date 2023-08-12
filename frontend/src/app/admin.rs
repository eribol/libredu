use shared::{msgs::admin::{AdminSchool, AdminUpMsgs}, UpMsg};
use zoon::{*, named_color::BLUE_2};

use crate::connection::send_msg;
use crate::i18n::t;

use super::screen_width;

pub mod msgs;
pub mod school;
pub mod timetables;
static HEIGHT: u32 = 42;

pub fn root() -> impl Element {
    Column::new()
    .s(Width::growable())
    .s(Padding::new().top(10).right(10).left(10))
    .item(footer_view())
    .item(search_bar())
    .item_signal(
        self::school::school()
        .signal_cloned()
        .map_option(|school|
            self::school::school_view(school).into_raw_element(),|| 
            schools().into_raw_element()
        )
    )
}

fn search_bar()->impl Element{
    Row::new()
    .s(Padding::new().bottom(20))
    .s(Align::center())
    .item(
        El::new()
        .s(Width::exact(30))
        .s(Height::exact(30))
        .s(Borders::new().left(Border::new().width(1)).top(Border::new().width(1)).bottom(Border::new().width(1)))
        .s(RoundedCorners::new().bottom_left(50).top_left(50))
    )
    .item(search_text())
    .item(
        El::new()
        .s(Width::exact(30))
        .s(Height::exact(30))
        .s(Borders::new().right(Border::new().width(1)).top(Border::new().width(1)).bottom(Border::new().width(1)))
        .s(RoundedCorners::new().bottom_right(50).top_right(50))
    )
}

fn schools()->impl Element{
    Column::new()
    .s(Padding::new().top(10))
    .s(Width::exact_signal(screen_width().signal()))
    .item(school_title_row())
    .items_signal_vec(last_schools().signal_vec_cloned().map(|s|{
        school_row(s)
    }))
}

fn school_title_row()->impl Element{
    Row::new()
    .s(Align::center())
    .item(
        Label::new().label("School Name")
        .s(Width::exact(300))
        .s(Height::exact(HEIGHT))
        .s(Font::new().weight(FontWeight::Bold))
        .s(Borders::all(Border::new().width(1).color(BLUE_2)))
    )
    .item(
        Label::new()
        .s(Width::exact(250))
        .s(Height::exact(HEIGHT))
        .s(Font::new().weight(FontWeight::Bold))
        .label_signal(t!("principle"))
        .s(Borders::all(Border::new().width(1).color(BLUE_2)))
    )
    .item(
        Label::new()
        .s(Width::exact(200))
        .s(Height::exact(HEIGHT))
        .s(Font::new().weight(FontWeight::Bold))
        .label("Last Login")
        .s(Borders::all(Border::new().width(1).color(BLUE_2)))
    )
}

fn school_row(school: AdminSchool)->impl Element{
    Row::new()
    .s(Align::center())
    .item(
        Label::new()
        .s(Width::exact(300))
        .s(Height::exact(HEIGHT))
        .s(Borders::all(Border::new().width(1).color(BLUE_2)))
        .on_click(move|| select_school(school.school.id))
        .label(&school.school.id.to_string()))
    .item(
        Label::new()
        .s(Width::exact(250))
        .s(Height::exact(HEIGHT))
        .s(Borders::all(Border::new().width(1).color(BLUE_2)))
        .label(format!("{} {}", school.principle.first_name, &school.principle.last_name))
    )
    .item(
        Label::new()
        .s(Width::exact(200))
        .s(Height::exact(HEIGHT))
        .s(Borders::all(Border::new().width(1).color(BLUE_2)))
        .label(
            &school
            .principle
            .last_login
            .format("%Y-%m-%d %H.%M")
            .to_string()
        )
    )
}

fn search_text()->impl Element{
    TextInput::new().id("search")
    .s(Width::exact(500))
    .s(Height::exact(30))
    .s(Borders::new().top(Border::new().width(1)).bottom(Border::new().width(1)))
}

fn footer_view()->impl Element{
    Column::with_tag(Tag::Footer)
    .item("This is footer")
    .update_raw_el(|raw|{
        raw.style("bottom", "0").style("position", "fixed").style("width", "100%")
    })
}


fn select_school(id: i32){
    let sch = last_schools().lock_mut().to_vec();
    let schl = sch.into_iter().find(|s| s.school.id == id);
    self::school::school().set(schl);
}
#[static_ref]
fn last_schools()->&'static MutableVec<AdminSchool>{
    get_schools();
    MutableVec::new_with_values(vec![])
}

pub fn get_schools(){
    let m = UpMsg::Admin(AdminUpMsgs::GetLastSchools);
    send_msg(m)
}