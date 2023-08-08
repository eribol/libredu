use shared::{School, msgs::admin::{AdminSchool, AdminUpMsgs}, DownMsg, UpMsg};
use zoon::{*, named_color::BLUE_2};

use crate::connection::send_msg;
use crate::i18n::t;
pub mod msgs;
static HEIGHT: u32 = 42;

pub fn root() -> impl Element {
    Column::new()
    .s(Padding::new().top(10).right(10).left(10))
    .item(footer_view())
    .item(search_view())
    .item_signal(
        selected_school()
        .signal()
        .map_bool(||
            Label::new().label("Selected School").into_raw_element(),|| 
            schools().into_raw_element()
        )
    )
}

fn schools()->impl Element{
    Column::new()
    .s(Padding::new().top(10))
    .item(school_title_row())
    .items_signal_vec(last_schools().signal_vec_cloned().map(|s|{
        school_row(s)
    }))
}

fn school_title_row()->impl Element{
    Row::new()
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
    .item(
        Label::new()
        .s(Width::exact(300))
        .s(Height::exact(HEIGHT))
        .s(Borders::all(Border::new().width(1).color(BLUE_2)))
        .label(&school.school.name))
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

fn search_view()->impl Element{
    TextInput::new().id("search")
    .s(Borders::all(Border::new().width(1)))
}

fn footer_view()->impl Element{
    Column::with_tag(Tag::Footer)
    .item("This is footer")
    .update_raw_el(|raw|{
        raw.style("bottom", "0").style("position", "fixed").style("width", "100%")
    })
}

#[static_ref]
fn selected_school()->&'static Mutable<bool>{
    Mutable::new(false)
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