use serde::*;
use seed::{*, prelude::*};
use crate::{Context};
use crate::model::class::{Class};
use crate::page::school::detail::{SchoolContext, ClassGroups, GroupContext};
use crate::page::school::group::class::{limitations, activities, timetables, students};

//use crate::page::school::class::class::Pages::Limitations;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Subject{
    pub name: String,
    pub id: i32
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Activity{
    pub(crate) id: i32,
    pub(crate) subject: Subject,
    pub(crate) teacher: i32,
    pub(crate) class: Class,
    pub(crate) hour: i16,
    pub(crate) split: bool,
    classes: Vec<i32>
}

#[derive(Debug)]
pub enum Msg{
    Home,
    Limitations(limitations::Msg),
    Activity(activities::Msg),
    Students(students::Msg),
    Timetables(timetables::Msg),
    FetchGroup(fetch::Result<ClassGroups>),
}

#[derive(Debug, Clone)]
pub enum Pages{
    Home,
    Students(students::Model),
    Activity(activities::Model),
    Limitations(limitations::Model),
    Timetables(timetables::Model),
    //NotFound
}
impl Default for Pages{
    fn default()->Self{
        Self::Home
    }
}
#[derive(Debug, Default, Clone)]
pub struct Model{
    pub class: Class,
    pub page: Pages,
    pub tab: String,
    pub next_class: Option<i32>,
    pub prev_class: Option<i32>,
}

pub fn init(mut url: Url, _orders: &mut impl Orders<Msg>, ctx_school: &mut SchoolContext, ctx_group: &mut GroupContext)-> Model{
    let mut model = Model::default();
    let class = &url.path()[5];
    //model.school_hour = school_hour;
    //let class = ctx_group.classes.iter().enumerate().find(|c| c.1.id == class.parse::<i32>().unwrap()).unwrap();
    //let classes: Vec<Class> = ctx_group.classes.clone().into_iter().filter(|c| c.group_id == class.1.group_id).collect();
    let class = ctx_group.classes.iter().enumerate().find(|c| c.1.id == class.parse::<i32>().unwrap()).unwrap();
    model.class = class.1.clone();
    if class.0 > ctx_group.classes.len() || ctx_group.classes.len() == 0{
        model.next_class = None;
        model.prev_class = None
    }
    if class.0 >= ctx_group.classes.len()-1{
        model.next_class = None
    }
    else {
        model.next_class = Some(ctx_group.classes[class.0+1].id);
    }
    if class.0 <= 0 {
        model.prev_class = None
    }
    else {
        model.prev_class = Some(ctx_group.classes[class.0-1].id);
    }
    match url.next_path_part(){
        Some("") | None => {
            model.page = Pages::Home;
            model.tab = "".to_string();
        }
        Some("limitations") => {
            model.page = Pages::Limitations(limitations::init(model.class.id,&mut _orders.proxy(Msg::Limitations),ctx_school, ctx_group));
            model.tab = "limitations".to_string();
        }
        Some("activities") => {
            model.page = Pages::Activity(activities::init(model.class.id,&mut _orders.proxy(Msg::Activity), ctx_school, ctx_group));
            model.tab = "activities".to_string();
        }
        Some("timetables") => {
            model.page = Pages::Timetables(timetables::init(model.class.id,&mut _orders.proxy(Msg::Timetables), ctx_school, ctx_group));
            model.tab = "timetables".to_string();
        }
        Some("students") => {
            model.page = Pages::Students(students::init(model.class.id,&mut _orders.proxy(Msg::Students), ctx_school, ctx_group));
            model.tab = "students".to_string();
        }
        _ => {
            //model.page = Pages::Activity
        }
    }
    model
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>, _ctx: &mut Context, ctx_school: &mut SchoolContext, ctx_group: &mut GroupContext) {
    match msg {
        Msg::Home => {
        }
        Msg::Limitations(msg) => {
            if let Pages::Limitations(m)= &mut model.page{
                limitations::update(msg, m, &mut orders.proxy(Msg::Limitations), _ctx, ctx_school, ctx_group)
            }
        }
        Msg::Activity(msg) => {
            if let Pages::Activity(m)= &mut model.page{
                activities::update(msg, m, &mut orders.proxy(Msg::Activity), _ctx, ctx_school, ctx_group)
            }
        }
        Msg::Students(msg) => {
            if let Pages::Students(m)= &mut model.page{
                students::update(msg, m, &mut orders.proxy(Msg::Students), _ctx, ctx_school, ctx_group)
            }
        }
        Msg::Timetables(msg) => {
            if let Pages::Timetables(m)= &mut model.page{
                timetables::update(msg, m, &mut orders.proxy(Msg::Timetables), _ctx, ctx_school, ctx_group)
            }
        }
        Msg::FetchGroup(group) => {
            match group{
                Ok(g) => {
                    ctx_school.groups.push(g)
                }
                Err(_) => {
                    //model.group = None
                }
            }
        }
    }
}
pub fn view(model: &Model, ctx: &Context, ctx_school: &SchoolContext, ctx_group: &GroupContext)->Node<Msg>{
    context(model, ctx, ctx_school, ctx_group)
}
pub fn context(model: &Model, ctx: &Context, ctx_school: &SchoolContext, ctx_group: &GroupContext)->Node<Msg>{
    div![
        C!{"columns"},
        div![
            C!{"column"},
            ctx_group.classes.iter().map(|c|
                div![
                    &c.sube, hr![]
                ]
            )
        ],
        match &model.page{
            Pages::Home=>{
                div![
                    //C!["column"],
                        div![
                            C!{"column is-12"},
                            div![
                                C!{"tabs is-centered"},
                                tabs(model, ctx, ctx_school, ctx_group),
                            ]
                        ],
                    home(&model.class)
                ]
            }
            Pages::Activity(m)=>{
                div![
                    div![
                        C!{"columns"},
                        div![
                            C!{"column is-12"},
                            div![
                                C!{"tabs is-centered"},
                                activities::tabs(m, ctx_school, ctx_group).map_msg(Msg::Activity),
                            ]
                        ]
                    ],
                    activities::activities(m, ctx_school).map_msg(Msg::Activity)
                ]
            }
            Pages::Students(m)=>{
                div![
                    div![
                        C!{"columns"},
                        div![
                            C!{"column is-12"},
                            div![
                                C!{"tabs is-centered"},
                                students::tabs(m, ctx_school, ctx_group).map_msg(Msg::Students),
                            ]
                        ]
                    ],
                    students::view(m, ctx_school, ctx_group).map_msg(Msg::Students)
                ]
            }
            Pages::Limitations(m) => {
                div![
                    div![
                        C!{"columns"},
                        div![
                            C!{"column is-12"},
                            div![
                                C!{"tabs is-centered"},
                                limitations::tabs(m, ctx_school, ctx_group).map_msg(Msg::Limitations),
                            ]
                        ]
                    ],
                    limitations::limitations(m, ctx_group).map_msg(Msg::Limitations)
                ]
            }
            Pages::Timetables(m) => {
                div![
                    div![
                        C!{"columns"},
                        div![
                            C!{"column is-12"},
                            div![
                                C!{"tabs is-centered"},
                                timetables::tabs(m, ctx_school, ctx_group).map_msg(Msg::Timetables),
                            ]
                        ]
                    ],
                    timetables::view(m, ctx_school, ctx_group).map_msg(Msg::Timetables)
                ]
            }
        }
    ]
}
fn home(class: &Class)->Node<Msg>{
    div![
        C!{"column is-half"},
        div![C!{"field"},
            label![C!{"label"}, "Kademe"],
            p![C!{"control has-icons-left"},
                input![C!{"input"},
                    attrs!{
                        At::Type=>"text",
                        At::Name=>"kademe",
                        At::Id=>"kademe",
                        At::Disabled => true,
                        At::Value => &class.kademe,
                    },
                ],
            ]
        ],
        div![C!{"field"},
            label![C!{"label"}, "Şube"],
            p![C!{"control has-icons-left"},
                input![C!{"input"},
                    attrs!{
                        At::Type=>"text",
                        At::Name=>"sube",
                        At::Id=>"sube",
                        At::Disabled => true,
                        At::Value => &class.sube,
                    },
                ],
            ]
        ]
    ]
}

pub fn tabs(model: &Model, _ctx: &Context, ctx_school: &SchoolContext, ctx_group: &GroupContext)->Node<Msg> {
    ul![
        li![
            C!{"is-active"},
            a![
                attrs!{
                    At::Href => format!("/schools/{}/groups/{}/classes/{}", &ctx_school.school.id, &ctx_group.group.id, &model.class.id)
                },
               "Bilgiler"
            ]
        ],
        li![
            //C!{"is-active"},
            a![
                attrs!{
                    At::Href => format!("/schools/{}/groups/{}/classes/{}/students", &ctx_school.school.id, &ctx_group.group.id, &model.class.id)
                },
               "Öğrenciler"
            ]
        ],
        li![
            a![
                attrs!{
                    At::Href => format!("/schools/{}/groups/{}/classes/{}/activities", &ctx_school.school.id, &ctx_group.group.id, &model.class.id)
                },
                "Aktiviteler"
            ]
        ],
        li![
            a![
                attrs!{
                    At::Href => format!("/schools/{}/groups/{}/classes/{}/limitations", &ctx_school.school.id, &ctx_group.group.id, &model.class.id)
                },
                "Kısıtlamalar"
            ]
        ],
        li![
            a![
                attrs!{
                    At::Href => format!("/schools/{}/groups/{}/classes/{}/timetables", &ctx_school.school.id, &ctx_group.group.id, &model.class.id)
                },
                "Ders Tablosu"
            ]
        ]
    ]
}