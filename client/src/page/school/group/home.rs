use serde::*;
use seed::{*, prelude::*};
use crate::{Context};
use crate::page::school::detail::SchoolContext;
use crate::page::school::group::{schedules, teachers, timetable};
use crate::page::school::group::classes;
//use crate::page::school::group::teachers;
use crate::model::class::{Class, ClassContext};
use crate::model::group;

#[derive(Clone)]
pub enum Pages{
    Home,
    Classes(Box<classes::Model>),
    Teachers(Box<teachers::Model>),
    Schedules(schedules::Model),
    //CommonExam(common_exam::Model),
    Timetables(Box<timetable::Model>),
    NotFound,
    Loading
}
impl Default for Pages{
    fn default() -> Self{
        Pages::Home
    }
}
impl Pages{
    fn init(mut url: Url, orders:&mut impl Orders<Msg>, school_ctx: &mut SchoolContext) -> Self {
        match url.next_path_part() {
            Some("") | None => Self::Home,
            Some("schedules") => Self::Schedules(schedules::init(url.clone(), school_ctx, &mut orders.proxy(Msg::Schedules))),
            //Some("common_exam") => model.page = Pages::CommonExam(common_exam::init(model.url.clone(),ctx_school, &mut orders.proxy(Msg::CommonExam), ctx_group)),
            Some("classes") => Self::Classes(Box::new(classes::init(url.clone(),&mut orders.proxy(Msg::Classes), school_ctx))),
            Some("teachers") => Self::Teachers(Box::new(teachers::init(url.clone(),&mut orders.proxy(Msg::Teachers), school_ctx))),
            Some("timetables") => Self::Timetables(Box::new(timetable::init(url.clone(), &mut orders.proxy(Msg::Timetables), school_ctx))),
            _ => Self::NotFound
        }
    }
}

#[derive(Default, Clone)]
pub struct Model{
    page: Pages,
    form: Form,
    menu: Vec<Menu>,
    url: Url,
}

#[derive(Default, Clone)]
pub struct Menu{
    link: String,
    title: String
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Form{
    name: String,
    hour: i32
}

#[derive()]
pub enum Msg{
    Home,
    ChangeName(String),
    ChangeHour(String),
    FetchUpdateGroup(fetch::Result<group::ClassGroups>),
    UpdateGroup,
    DelGroup,
    FetchDelGroup(fetch::Result<group::ClassGroups>),
    Schedules(schedules::Msg),
    //CommonExam(common_exam::Msg),
    Classes(classes::Msg),
    Teachers(teachers::Msg),
    Timetables(timetable::Msg),
    FetchClasses(fetch::Result<Vec<Class>>),
    Loading
}

pub fn init(url: Url, school_ctx: &mut SchoolContext, orders: &mut impl Orders<Msg>) -> Model {
    let mut model = Model {
        url: url.clone(),
        ..Default::default()
    };
    model.page = Pages::Loading;

    let group_ctx = &mut school_ctx.get_mut_group(&url);
    model.form.hour = group_ctx.group.hour;
    model.form.name = group_ctx.group.name.clone();
    model.menu = vec![
        Menu { link: "".to_string(), title: group_ctx.group.name.to_string() },
        Menu { link: "schedules".to_string(), title: "Zaman Çizelgesi".to_string() },
        Menu { link: "classes".to_string(), title: "Sınıflar".to_string() },
        Menu { link: "teachers".to_string(), title: "Öğretmenler".to_string() },
        Menu { link: "common_exam".to_string(), title: "Ortak Sınav".to_string() },
        Menu { link: "timetables".to_string(), title: "Ders Programı".to_string() },
    ];
    orders.send_msg(Msg::Loading);
    model
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>, school_ctx:&mut SchoolContext) {
    match msg {
        Msg::Home => {
        }
        Msg::Schedules(msg) => {
            if let Pages::Schedules(m) = &mut model.page {
                let group_ctx = school_ctx.get_mut_group(&model.url);
                schedules::update(msg, m, &mut orders.proxy(Msg::Schedules), group_ctx)
            }
        }
        Msg::Classes(msg)=> {
            if let Pages::Classes(m) = &mut model.page {
                classes::update(msg, m, &mut orders.proxy(Msg::Classes), school_ctx)
            }
        }
        Msg::Teachers(msg)=>{
            if let Pages::Teachers(m)= &mut model.page {
                if school_ctx.get_group(&model.url).classes.is_none(){
                    orders.perform_cmd({
                        let adres = format!("/api/schools/{}/groups/{}/classes", &school_ctx.school.id, &model.url.path()[3]);
                        let request = Request::new(adres)
                            .method(Method::Get);
                        async { Msg::FetchClasses(async {
                            request
                                .fetch()
                                .await?
                                .check_status()?
                                .json()
                                .await
                        }.await)}
                    });
                }
                teachers::update(msg, m, &mut orders.proxy(Msg::Teachers), school_ctx)
            }
        }
        Msg::FetchClasses(classes) => {
            if let Ok(clss) = classes {
                let group_ctx = school_ctx.get_mut_group(&model.url);
                if let Some(clss_ctx) = &mut group_ctx.classes{
                    clss_ctx.clear();
                    for c in clss{
                        let class_ctx = ClassContext{
                            class: c,
                            students: None,
                            activities: None,
                            limitations: None,
                            timetables: None
                        };
                        clss_ctx.push(class_ctx);
                    }
                }
                else {
                    let group_ctx = school_ctx.get_mut_group(&model.url);
                    group_ctx.classes = Some(vec![]);
                    if let Some(classes) = &mut group_ctx.classes{
                        for c in clss{
                            let class_ctx = ClassContext{
                                class: c,
                                students: None,
                                activities: None,
                                limitations: None,
                                timetables: None
                            };
                            classes.push(class_ctx);
                        }
                    }
                    //*classes_ctx = Some(context.clone());

                }
            }
            else {
                let group_ctx = school_ctx.get_mut_group(&model.url);
                group_ctx.classes = Some(vec![]);
            }
            //model.page = Pages::init(model.url.clone(), orders, school_ctx);
        }
        Msg::ChangeName(name) => {
            model.form.name = name;
        }
        Msg::ChangeHour(hour) => {
            if let Ok(h) = hour.parse::<i32>() {
                model.form.hour = h
            }
        }
        Msg::FetchUpdateGroup(group) => {
            if group.is_ok() {
            }
        }
        Msg::UpdateGroup => {
            orders.perform_cmd({
                let adres = format!("/api/schools/{}/groups/{}", school_ctx.school.id, model.url.path()[3]);
                let request = Request::new(adres)
                    .method(Method::Patch)
                    .json(&model.form);
                async {
                    Msg::FetchUpdateGroup(async {
                        request?
                            .fetch()
                            .await?
                            .check_status()?
                            .json()
                            .await
                    }.await)
                }
            });
        }
        Msg::DelGroup => {
            orders.perform_cmd({
                let adres = format!("/api/schools/{}/groups/{}", school_ctx.school.id, model.url.path()[3]);
                let request = Request::new(adres)
                    .method(Method::Delete);
                async { Msg::FetchDelGroup(async {
                    request
                        .fetch()
                        .await?
                        .check_status()?
                        .json()
                        .await
                }.await)}
            });
        }
        Msg::FetchDelGroup(group) => {
            if let Ok(gr) = group {
                if let Some(groups) = &mut school_ctx.groups{
                    orders.notify(
                        subs::UrlRequested::new(format!("").parse().unwrap())
                    );
                    groups.retain(|g| g.group.id != gr.id)
                }
            }
        }
        Msg::Loading => {
            let group_ctx = school_ctx.get_mut_group(&model.url);
            if group_ctx.classes.is_none() {
                orders.perform_cmd({
                    let adres = format!("/api/schools/{}/groups/{}/classes", model.url.path()[1], model.url.path()[3]);
                    let request = Request::new(adres)
                        .method(Method::Get);
                    async {
                        Msg::FetchClasses(async {
                            request
                                .fetch()
                                .await?
                                .check_status()?
                                .json()
                                .await
                        }.await)
                    }
                });
            }
            model.page = Pages::init(model.url.clone(), orders, school_ctx);
        }
        Msg::Timetables(msg)=>{
            if let Pages::Timetables(m)= &mut model.page{
                timetable::update(msg, m, &mut orders.proxy(Msg::Timetables), school_ctx)
            }
        }
        /*

        Msg::CommonExam(msg) => {
            if let Pages::CommonExam(m)= &mut model.page{
                common_exam::update(msg, m, &mut orders.proxy(Msg::CommonExam), _ctx, ctx_school, ctx_group)
            }
        }
         */
    }
}
pub fn view(model: &Model, school_ctx: &SchoolContext)->Node<Msg>{
    div![
        C!{"column"},
        div![
            C!{"columns"},
            //div![C!{"column is-4"}],
            div![
                C!{"column is-11 is-offset-fifth"},
                nav![
                    C!{"breadcrumb is-centered"},
                    ul![
                        model.menu.iter().map(|m|
                            li![
                                a![
                                    attrs!{
                                        At::Href => format!("/schools/{}/groups/{}/{}", school_ctx.school.id, school_ctx.get_group(&model.url).group.id, &m.link)
                                    },
                                    &m.title
                                ]
                            ]
                        )
                    ]
                ]
            ]
        ],
        match &model.page{
            Pages::Home => home(model),
            Pages::Schedules(m) => schedules::view(&m).map_msg(Msg::Schedules),
            Pages::Classes(m) => classes::view(&m, &school_ctx).map_msg(Msg::Classes),
            Pages::Teachers(m) => teachers::view(&m, school_ctx).map_msg(Msg::Teachers),
            //Pages::CommonExam(m) => common_exam::view(&m, &ctx_group).map_msg(Msg::CommonExam),
            Pages::Timetables(m) => timetable::view(&m, school_ctx).map_msg(Msg::Timetables),
            Pages::NotFound => div!["Grup bulunamadı"],
            Pages::Loading => {
                div!["yükleniyor..."]
            }
        }
    ]
}

fn home(model: &Model) -> Node<Msg>{
    div![
        C!{"columns"},
        div![
            C!{"column is-half is-offset-1"},
            p![
                C!{"control"},
                label![C!{"label"}, "Grup Adı:"],
                input![
                    C!{"input"},
                    attrs!{
                        At::Name=>"type",
                        At::Id=>"type",
                        At::Value => &model.form.name,
                    },
                    input_ev(Ev::Input, Msg::ChangeName)
                ],
            ],
            p![
                C!{"control"},
                label![C!{"label"}, "Grup Günlük Ders Saati Sayısı:"],
                input![
                    C!{"input"},
                    attrs!{
                        At::Value => &model.form.hour,
                    },
                    input_ev(Ev::Input, Msg::ChangeHour)
                ]
            ],
            p![
                C!{"control"},
                input![C!{"button is-primary"},
                    attrs!{
                        At::Type=>"button",
                        At::Value=>"Güncelle"
                    },
                    ev(Ev::Click, |event| {
                        event.prevent_default();
                        Msg::UpdateGroup
                    })
                ]
            ],
            p![
                C!{"is-dangerous"},
                "Dikkat! Grubun günlük ders saatini değiştirdiğinizde, sınıfların ve öğretmenlerin kısıtlamaları da değişir. Hatalarla karşılaşmamak için, saat sayısını güncelledikten sonra kısıtlamalara göz atın."
            ],
            p![
                C!{"control"},
                input![C!{"button is-danger"},
                    attrs!{
                        At::Type=>"button",
                        At::Value=>"Grubu Sil(Dikkat! Gruba bağlı tüm veriler silinir)"
                    },
                    ev(Ev::Click, |event| {
                        event.prevent_default();
                        Msg::DelGroup
                    })
                ]
            ]
        ]
    ]
}

/*
pub fn menus(menu: &[Menu], ctx: &Context, ctx_school: &SchoolContext, model: &Model) -> Node<Msg>{
    ul![
        C!{"menu-list"},
        menu.iter().map(|m|
            li![
                a![
                    C!{
                        //if active_menu(&model.page, m){"is-active"} else {""}
                    },
                    &m.title,
                    attrs![
                        //At::Href=> format!("/schools/{}/groups/{}/{}", &ctx_school.school.id, &model.ctx_group.group.id, &m.link)
                    ]
                ]
            ]
        )
    ]
}

fn active_menu (page: &Pages, menu: &Menu) -> bool{
    match page{
        Pages::Home => {
            if menu.link == ""{
                true
            }
            else {
                false
            }
        }
        Pages::Schedules(_m) => {
            if menu.link == "schedules"{
                true
            }
            else {
                false
            }
        }
        Pages::Teachers(_m) => {
            if menu.link == "teachers"{
                true
            }
            else {
                false
            }
        }
        Pages::Timetables(_m) => {
            if menu.link == "timetables"{
                true
            }
            else {
                false
            }
        }
        _ => {
            false
        }
    }
}*/

