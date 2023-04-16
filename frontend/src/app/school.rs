use crate::app::login::get_school;
use crate::i18n::t;
use zoon::strum::{EnumIter, IntoEnumIterator, IntoStaticStr};
use zoon::{named_color::*, *};

pub mod add_school;
//pub mod class;
pub mod classes;
pub mod homepage;
pub mod lectures;
//pub mod teacher;
pub mod teachers;
pub mod timetables;
#[static_ref]
pub fn school() -> &'static Mutable<Option<School>> {
    if let Some(Ok(school)) = local_storage().get("school") {
        return Box::new(Mutable::new(Some(school)));
    };
    get_school();
    Mutable::new(None)
}

#[static_ref]
fn selected_page() -> &'static Mutable<SchoolPages> {
    Mutable::new(SchoolPages::default())
}

fn change_page(p: SchoolPages) {
    selected_page().set(p)
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct School {
    pub id: i32,
    pub name: String,
}
pub fn school_page() -> impl Element {
    El::new().child_signal(school().signal_ref(|schl| {
        match schl {
            Some(_s) => Column::new()
                .s(Gap::new().y(50))
                .item(school_tabs())
                .item_signal(selected_page().signal_ref(|page| match page {
                    SchoolPages::Home => homepage::home().into_raw_element(),
                    SchoolPages::Classes => classes::home().into_raw_element(),
                    SchoolPages::Teachers => teachers::home().into_raw_element(),
                    SchoolPages::Lectures => lectures::home().into_raw_element(),
                    SchoolPages::Timetabling => timetables::home().into_raw_element(),
                })),
            None => Column::new().item(Row::new().item(add_school::add_school_page())),
        }
    }))
    //add_school::add_school_page()
}

fn school_tabs() -> impl Element {
    Row::new()
        .s(Gap::new().x(50))
        .s(Align::center())
        .s(Font::new().weight(FontWeight::Medium))
        .items(SchoolPages::iter().map(|page| {
            Button::new()
                .s(
                    Borders::new().bottom_signal(selected_page().signal_ref(move |p| {
                        if p == &page {
                            Border::new().width(2).solid().color(BLUE_5)
                        } else {
                            Border::new().width(0).solid().color(GRAY_0)
                        }
                    })),
                )
                //.s(Width::exact(150))
                .on_click(move || change_page(page))
                .label_signal(t!(format!("{}", page.label())))
        }))
}
#[derive(Clone, Copy, IntoStaticStr, EnumIter, Debug, Default, PartialEq)]
#[strum(crate = "strum")]
enum SchoolPages {
    #[default]
    Home,
    Classes, //classes
    Teachers,
    Lectures,
    Timetabling,
}

impl SchoolPages {
    fn label<'a>(&'a self) -> &'a str {
        match self {
            Self::Classes => "classes",
            Self::Teachers => "teachers",
            Self::Lectures => "lectures",
            Self::Timetabling => "timetables",
            Self::Home => "homepage",
        }
    }
}
