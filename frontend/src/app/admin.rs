use shared::{msgs::admin::{AdminSchool, AdminUpMsgs}, UpMsg};
use zoon::{*, named_color::BLUE_2, strum::{EnumIter, IntoStaticStr, IntoEnumIterator}};

use crate::connection::send_msg;
use crate::i18n::t;

use self::timetables::clear_data;

use super::screen_width;

pub mod msgs;
pub mod school;
pub mod timetables;
pub mod messages;
static HEIGHT: u32 = 42;

#[derive(Debug, Clone, EnumIter, IntoStaticStr)]
#[strum(crate = "strum")]
enum RootPage{
    Schools,
    Message
}

#[static_ref]
fn page()->&'static Mutable<RootPage>{
    Mutable::new(RootPage::Schools)
}

fn change_page(p: RootPage){
    page().set(p.clone());
    if let RootPage::Schools = p{
        self::school::school().set(None);
    }
}

pub fn root()->impl Element{
    Column::new()
    .s(Width::exact_signal(screen_width().signal()))
    .s(Height::fill())
    .s(Padding::new().top(10).right(10).left(10))
    .item(tabs())
    .item_signal(
        page().signal_cloned().map(|p|{
            match p{
                RootPage::Schools=>schools_view().into_raw(),
                RootPage::Message=>messages::messages_view().into_raw()
            }
        })
    )
}

pub fn tabs()-> impl Element{
    Row::new()
    .s(Align::center())
    .s(Gap::new().x(10))
    .items(RootPage::iter().map(|p| 
        Label::new()
        .label(format!("{:?}", &p))
        .on_click(move ||change_page(p.clone()))
    ))
}
pub fn schools_view() -> impl Element {
    Column::new()
    .s(Width::growable())
    .item(search_bar())
    .item_signal(
        self::school::school()
        .signal_cloned()
        .map_option(|school|
            self::school::school_view(school).into_raw(),|| 
            schools().into_raw()
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
        .on_click(move|| {
            select_school(school.school.id);
            clear_data();
            
        })
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


fn select_school(id: i32){
    let sch = last_schools().lock_mut().to_vec();
    let schl = sch.into_iter().find(|s| s.school.id == id);
    self::school::school().set(schl);
    self::school::get_timetables();
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