use serde::*;
use seed::{*, prelude::*};
use crate::model::class::{Class, ClassContext};
use crate::page::school::detail::SchoolContext;
use crate::page::school::group::class::{activities, limitations, timetables, students};
use crate::model::group::ClassGroups;
use crate::i18n::I18n;

//use crate::page::school::class::class::Pages::Limitations;

#[derive(Serialize, Deserialize, Clone)]
pub struct Subject{
    pub name: String,
    pub id: i32
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Activity{
    pub(crate) id: i32,
    pub(crate) subject: Subject,
    pub(crate) teacher: i32,
    pub(crate) class: Class,
    pub(crate) hour: i16,
    pub(crate) split: bool,
    classes: Vec<i32>
}

#[derive()]
pub enum Msg{
    Home,
    Limitations(limitations::Msg),
    Activity(activities::Msg),
    Students(students::Msg),
    Timetables(timetables::Msg),
    FetchGroup(fetch::Result<ClassGroups>),
}

#[derive(Clone)]
pub enum Pages{
    Home,
    Students(students::Model),
    Activity(Box<activities::Model>),
    Limitations(limitations::Model),
    Timetables(timetables::Model),
    NotFound
}
impl Default for Pages{
    fn default()->Self{
        Self::NotFound
    }
}
#[derive(Default, Clone)]
pub struct Model{
    pub page: Pages,
    pub tab: String,
    pub url: Url
}

pub fn init(url: Url, _orders: &mut impl Orders<Msg>, school_ctx: &mut SchoolContext)-> Model {
    let mut model = Model{url: url, ..Default::default()};

    match model.url.next_path_part() {
        Some("") | None => {
            model.page = Pages::Home;
            model.tab = "".to_string();
        }
        Some("activities") => {
            model.page = Pages::Activity(Box::new(activities::init(model.url.clone(), &mut _orders.proxy(Msg::Activity), school_ctx)));
            model.tab = "activities".to_string();
        }

        Some("limitations") => {
            model.page = Pages::Limitations(limitations::init(model.url.clone(), &mut _orders.proxy(Msg::Limitations), school_ctx));
            model.tab = "limitations".to_string();
        }
        Some("timetables") => {
            model.page = Pages::Timetables(timetables::init(model.url.clone(), &mut _orders.proxy(Msg::Timetables), school_ctx));
            model.tab = "timetables".to_string();
        }

        Some("students") => {
            model.page = Pages::Students(students::init(model.url.clone(), &mut _orders.proxy(Msg::Students), school_ctx));
            model.tab = "students".to_string();
        }

        _ => {
            model.page = Pages::Home
        }
    }
    //}

    //let classes: Vec<Class> = ctx_group.classes.clone().into_iter().filter(|c| c.group_id == class.1.group_id).collect();
    //let class = ctx_group.classes.iter().enumerate().find(|c| c.1.id == class.parse::<i32>().unwrap()).unwrap();
    //model.class = class.1.clone();
    model
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>, school_ctx: &mut SchoolContext) {
    let group_ctx = school_ctx.get_mut_group(&model.url);
    match msg {
        Msg::Home => {
        }
        Msg::Activity(msg) => {
            if let Pages::Activity(m)= &mut model.page{
                activities::update(msg, m, &mut orders.proxy(Msg::Activity), school_ctx)
            }
        }
        Msg::Limitations(msg) => {
            if let Pages::Limitations(m)= &mut model.page{
                limitations::update(msg, m, &mut orders.proxy(Msg::Limitations), school_ctx)
            }
        }
        Msg::Timetables(msg) => {
            if let Pages::Timetables(m)= &mut model.page{
                timetables::update(msg, m, &mut orders.proxy(Msg::Timetables), school_ctx)
            }
        }
        Msg::Students(msg) => {
            if let Pages::Students(m)= &mut model.page{
                students::update(msg, m, &mut orders.proxy(Msg::Students), school_ctx)
            }
        }
        _ => {}
        /*


        Msg::FetchGroup(group) => {
            if group.is_ok(){
            }
        }

         */
    }
}

pub fn view(model: &Model, school_ctx: &SchoolContext, lang: &I18n)->Node<Msg>{
    let group_ctx = school_ctx.get_group(&model.url);
    let class_ctx = school_ctx.get_group(&model.url).get_class(&model.url);
    div![
        C!{"columns"},
        match &model.page{
            Pages::Home => home(&class_ctx, lang),
            Pages::Activity(m) => {
                activities::activities(m, school_ctx, lang).map_msg(Msg::Activity)
            }
            Pages::Limitations(m) => {
                limitations::limitations(m, group_ctx, lang).map_msg(Msg::Limitations)
            }
            Pages::NotFound => {
                div![]
            }
            Pages::Timetables(m) => {
                timetables::timetable(m, school_ctx, lang).map_msg(Msg::Timetables)
            }
            Pages::Students(m) => {
                students::view(m, school_ctx).map_msg(Msg::Students)
            }
        }
    ]
}
fn home(class_ctx: &ClassContext, lang: &I18n)->Node<Msg>{
    use crate::{create_t, with_dollar_sign};
    create_t![lang];
    div![
        C!{"column is-4"},
        div![C!{"field"},
            label![C!{"label"}, t!["class-grade"]],
            p![C!{"control has-icons-left"},
                input![C!{"input"},
                    attrs!{
                        At::Type=>"text",
                        At::Name=>"kademe",
                        At::Id=>"kademe",
                        At::Disabled => true,
                        At::Value => &class_ctx.class.kademe,
                    },
                ],
            ]
        ],
        div![C!{"field"},
            label![C!{"label"}, t!["class-branch"]],
            p![C!{"control has-icons-left"},
                input![C!{"input"},
                    attrs!{
                        At::Type=>"text",
                        At::Name=>"sube",
                        At::Id=>"sube",
                        At::Disabled => true,
                        At::Value => &class_ctx.class.sube,
                    },
                ],
            ]
        ]
    ]
}