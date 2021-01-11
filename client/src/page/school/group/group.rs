use serde::*;
use seed::{*, prelude::*};
use crate::{Context};
use crate::page::school::detail::{ClassGroups, SchoolContext, GroupContext};
use crate::page::school::group::{schedules, timetable};
use crate::page::school::group::classes;
use crate::page::school::group::teachers;
use crate::model::class::Class;


#[derive(Debug, Clone)]
pub enum Pages{
    Home,
    Classes(classes::Model),
    Teachers(teachers::Model),
    Schedules(schedules::Model),
    Timetables(timetable::Model),
    NotFound
}
impl Default for Pages{
    fn default() -> Self{
        Pages::Home
    }
}


#[derive(Debug, Default, Clone)]
pub struct Model{
    pub ctx_group: GroupContext,
    page: Pages,
    form: Form,
    menu: Vec<Menu>,
    url: Url
}

#[derive(Debug, Default, Clone)]
pub struct Menu{
    link: String,
    title: String
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Form{
    name: String,
    hour: i32
}

#[derive(Debug)]
pub enum Msg{
    Home,
    ChangeName(String),
    ChangeHour(String),
    FetchUpdateGroup(fetch::Result<ClassGroups>),
    UpdateGroup,
    DelGroup,
    FetchDelGroup(fetch::Result<ClassGroups>),
    Schedules(schedules::Msg),
    Classes(classes::Msg),
    Teachers(teachers::Msg),
    Timetables(timetable::Msg),
    FetchClasses(fetch::Result<Vec<Class>>)
}

pub fn init(url: Url, ctx_school: &mut SchoolContext, orders: &mut impl Orders<Msg>) -> Model{
    let mut model = Model::default();
    model.url = url.clone();
    let group_id = &url.path()[3];
    let group = ctx_school.groups.iter().find(|g| g.id == group_id.parse::<i32>().unwrap());
    match group{
        Some(g) =>{
            model.ctx_group.group = g.clone();
            model.form.hour = g.hour.clone();
            model.form.name = g.name.clone();
            model.menu = vec![
                Menu{ link: "".to_string(), title: g.name.clone() },
                Menu{ link: "schedules".to_string(), title: "Zaman Çizelgesi".to_string() },
                Menu{ link: "classes".to_string(), title: "Sınıflar".to_string() },
                Menu{ link: "teachers".to_string(), title: "Öğretmenler".to_string() },
                Menu{ link: "timetables".to_string(), title: "Ders Programı".to_string() },
            ];
            orders.perform_cmd({
                let adres = format!("/api/schools/{}/classes", &ctx_school.school.id);
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
        None => {
            model.page = Pages::NotFound
        }
    }

    model
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>, _ctx: &mut Context, ctx_school: &mut SchoolContext) {
    let ctx_group = &mut model.ctx_group;
    match msg {
        Msg::Home => {
        }
        Msg::FetchClasses(classes) => {
            match classes{
                Ok(cls) =>{
                    ctx_group.classes = cls.clone().into_iter().filter(|c| c.group_id == ctx_group.group.id).collect();
                    match model.url.next_path_part(){
                        Some("") | None => model.page = Pages::Home,
                        Some("schedules") => model.page = Pages::Schedules(schedules::init(model.url.clone(),ctx_school, &ctx_group.group, &mut orders.proxy(Msg::Schedules))),
                        Some("classes") => model.page = Pages::Classes(classes::init(model.url.clone(),&mut orders.proxy(Msg::Classes),ctx_school, ctx_group)),
                        Some("teachers") => model.page = Pages::Teachers(teachers::init(model.url.clone(),&mut orders.proxy(Msg::Teachers),_ctx, ctx_school, ctx_group)),
                        Some("timetables") => model.page = Pages::Timetables(timetable::init(&mut orders.proxy(Msg::Timetables), ctx_school, ctx_group)),
                        _ => model.page = Pages::NotFound
                    }
                }
                Err(e) => {
                    log!(e);
                    model.page = Pages::NotFound
                }
            }
        }
        Msg::Schedules(msg) => {
            if let Pages::Schedules(m)= &mut model.page{
                schedules::update(msg, m, &mut orders.proxy(Msg::Schedules), _ctx, ctx_school)
            }
        }
        Msg::ChangeName(name) => {
            model.form.name = name;
        }
        Msg::ChangeHour(hour) => {
            match hour.parse::<i32>(){
                Ok(h) => {
                    model.form.hour = h
                }
                Err(_) => {}
            }

        }
        Msg::FetchUpdateGroup(group) => {
            match group {
                Ok(g) => {
                    ctx_school.groups.retain(|gg| gg.id != g.id);
                    ctx_school.groups.push(g);
                }
                Err(_) => {}
            }
        }
        Msg::UpdateGroup => {
            orders.perform_cmd({
                let adres = format!("/api/schools/{}/groups/{}", ctx_school.school.id, model.ctx_group.group.id);
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
                let adres = format!("/api/schools/{}/groups/{}", &ctx_school.school.id, &ctx_group.group.id);
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
            match group {
                Ok(g) => {
                    ctx_school.groups.retain(|gg| gg.id != g.id);
                }
                Err(_) => {}
            }
        }
        Msg::Classes(msg)=>{
            if let Pages::Classes(m)= &mut model.page{
                classes::update(msg, m, &mut orders.proxy(Msg::Classes), _ctx, ctx_school, ctx_group)
            }
        }
        Msg::Teachers(msg)=>{
            if let Pages::Teachers(m)= &mut model.page{
                teachers::update(msg, m, &mut orders.proxy(Msg::Teachers), _ctx, ctx_school, ctx_group)
            }
        }
        Msg::Timetables(msg)=>{
            if let Pages::Timetables(m)= &mut model.page{
                timetable::update(msg, m, &mut orders.proxy(Msg::Timetables), _ctx, ctx_school, ctx_group)
            }
        }
    }
}
pub fn view(model: &Model, ctx: &Context, ctx_school: &SchoolContext)->Node<Msg>{
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
                                        At::Href => format!("/schools/{}/groups/{}/{}", &ctx_school.school.id, &model.ctx_group.group.id, &m.link)
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
            Pages::Classes(m) => classes::view(&m, ctx, ctx_school, &model.ctx_group).map_msg(Msg::Classes),
            Pages::Teachers(m) => teachers::view(&m, ctx, ctx_school, &model.ctx_group).map_msg(Msg::Teachers),
            Pages::Timetables(m) => timetable::view(&m, ctx, ctx_school).map_msg(Msg::Timetables),
            Pages::NotFound => div!["Grup bulunamadı"],
            _ => div!["diğer"]
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
                label![C!{"label"}, "Grup Adı:"],
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

pub fn menus(menu: &Vec<Menu>, ctx: &Context, ctx_school: &SchoolContext, model: &Model) -> Node<Msg>{
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
                        At::Href=> format!("/schools/{}/groups/{}/{}", &ctx_school.school.id, &model.ctx_group.group.id, &m.link)
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
}

