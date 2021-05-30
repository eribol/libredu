use seed::{*, prelude::*};
use crate::{Context};
use crate::model::class::Class;
use crate::page::school::group::class::home;
use crate::model::class::{NewClass};
use crate::page::school::detail::{SchoolContext, GroupContext};

#[derive(Debug, Default, Clone)]
pub struct Model{
    pages: Pages,
    form: NewClass,
}

#[derive(Debug, Clone)]
pub enum Pages{
    Classes,
    Class(Box<home::Model>)
}
impl Default for Pages{
    fn default()->Self{
        Self::Classes
    }
}

#[derive(Debug)]
pub enum Msg{
    AddClass,
    ChangeKademe(String),
    ChangeSube(String),
    FetchClass(fetch::Result<Class>),
    Class(home::Msg),
    DeleteClass(i32),
    FetchDel(fetch::Result<i32>),
    FetchUpdateClass(fetch::Result<Class>)
}

pub fn init(mut url: Url, orders: &mut impl Orders<Msg>, ctx_school: &mut SchoolContext, ctx_group: &mut GroupContext)-> Model {
    let mut model = Model::default();

    match url.next_path_part(){
        Some("") | None => {
            model.pages = Pages::Classes;
            if ctx_school.groups.is_some() && !ctx_school.groups.as_ref().unwrap().is_empty(){
                model.form.group_id = ctx_group.group.id;
            }
            else {
                model.form.group_id = 0
            }
        }
        Some(_id) => {
            model.pages = Pages::Class(Box::new(home::init(url.clone(), &mut orders.proxy(Msg::Class), ctx_school, ctx_group)))
        }
    }
    model
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>, _ctx: &mut Context, ctx_school: &mut SchoolContext, ctx_group: &mut GroupContext) {
    match msg {
        Msg::DeleteClass(id) => {
            orders.perform_cmd({
                let url = format!("/api/schools/{}/groups/{}/classes/{}", ctx_school.school.id, ctx_group.group.id, id);
                let request = Request::new(url)
                    .method(Method::Delete);
                async {
                    Msg::FetchDel(async {
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
        Msg::FetchDel(id) =>{
            if let Ok(i) = id {
                ctx_group.classes.retain(|c| c.id != i);
            }
        }
        Msg::Class(msg)=>{
            if let Pages::Class(m) = &mut model.pages {
                home::update(msg, m, &mut orders.proxy(Msg::Class), _ctx, ctx_school, ctx_group)
            }
        }
        Msg::AddClass=> {
            orders.perform_cmd({
                let url = format!("/api/schools/{}/groups/{}/add_class", ctx_school.school.id, ctx_group.group.id);
                let request = Request::new(url)
                    .method(Method::Post)
                    .json(&model.form);
                async {
                    Msg::FetchClass(async {
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
        Msg::ChangeKademe(kademe)=>{
            model.form.kademe = kademe;
        }
        Msg::ChangeSube(sube)=>{
            model.form.sube = sube
        }
        Msg::FetchClass(class)=>{
            if let Ok(c) = class {
                ctx_group.classes.insert(0, c);
            }
        }
        Msg::FetchUpdateClass(class)=>{
             if class.is_ok() {
                 //model.classes = model.classes.clone().into_iter().filter(|cg| cg.group_id == model.selected_group.id).collect();
             }
        }
    }
}

pub fn view(model: &Model, ctx: &Context, ctx_school: &SchoolContext, ctx_group: &GroupContext)-> Node<Msg>{
    div![
        C!{"columns"},
        //div![
            //C!{"column is-8"},
            match &model.pages{
                //Pages::Class(model)=>class_detail(model, ctx, ctx_school),
                Pages::Classes=>{
                    div![
                        C!{"column is-11 is-offset-1"},
                        table![
                        C!{"table table-hover"},
                        thead![
                            tr![
                                th![
                                    attrs!{At::Scope=>"col"},
                                    "Kademe"
                                ],
                                th![
                                    attrs!{At::Scope=>"col"},
                                    "Şube"
                                ],
                                th![
                                    attrs!{At::Scope=>"col"},
                                    "Grup"
                                ]
                            ]
                        ],
                        tbody![
                            tr![
                                C!{"table-light"},
                                td![
                                    input![
                                        attrs!{At::Type=>"text", At::Placeholder=>"Kademe", At::Value=>&model.form.kademe},
                                        input_ev(Ev::Input, Msg::ChangeKademe)
                                    ]
                                ],
                                td![
                                    input![
                                        attrs!{At::Type=>"text", At::Placeholder=>"Şube", At::Value=>&model.form.sube},
                                        input_ev(Ev::Input, Msg::ChangeSube)
                                    ]
                                ],
                                td![
                                    input![C!{"button is-primary"},
                                        attrs!{
                                            At::Type=>"button",
                                            //At::Value=> add_button(ctx_school.groups.len()),
                                            At::Id=>"login_button"
                                        },
                                        ev(Ev::Click, |event| {
                                            event.prevent_default();
                                            Msg::AddClass
                                        })
                                    ]
                                ]
                            ],
                            ctx_group.classes.iter().enumerate().map(|c|
                                //log!(&c),
                                tr![
                                    C!{"table-light"},
                                    td![
                                        a![
                                            &c.1.kademe.to_string(),
                                            attrs!{
                                                At::Href=> format!("/schools/{}/groups/{}/classes/{}", &ctx_school.school.id, &ctx_group.group.id, c.1.id)
                                            }
                                        ]
                                    ],
                                    td![
                                        a![
                                            &c.1.sube,
                                            attrs!{
                                                At::Href=> format!("/schools/{}/groups/{}/classes/{}", &ctx_school.school.id, &ctx_group.group.id, c.1.id)
                                            }
                                        ]
                                    ],
                                    td![

                                    ],
                                    td![
                                        button![
                                            C!{"button"},
                                            attrs!{At::Value=>&c.1.id},
                                            "Sil",
                                            {
                                                let id = c.1.id;
                                                ev(Ev::Click, move |_event| {
                                                    Msg::DeleteClass(id)
                                                })
                                            }
                                        ]
                                    ]
                                ]
                            )
                        ]
                    ]
                    ]
                },
                Pages::Class(m) => {
                    class_detail(m, ctx, ctx_school, ctx_group)
                }
            }
        //]
    ]
}

fn class_detail(c_model: &home::Model, ctx: &Context, ctx_school: &SchoolContext, ctx_group: &GroupContext) ->Node<Msg>{
    div![
        C!{"column is-12"},
        div![
            C!{"columns"},
            div![C!{"column is-12"},
                nav![
                    C!{"breadcrumb is-centered"},
                    attrs!{At::AriaLabel=>"breadcrumbs"},
                    ul![
                        li![
                            a![
                                attrs!{
                                    At::Href=> format!("/schools/{}/groups/{}/classes", &ctx_school.school.id, &ctx_group.group.id)
                                },
                                "<--Sınıflar",
                            ]
                        ],
                        match c_model.prev_class{
                            Some(class) => {
                                li![
                                    a![
                                        attrs!{
                                            At::Href=> format!("/schools/{}/groups/{}/classes/{}/{}", &ctx_school.school.id, &ctx_group.group.id, class, &c_model.tab)
                                        },
                                        "Önceki Sınıf",
                                    ]
                                ]
                            },
                            None => {
                                div![]
                            }
                        },
                        li![
                            div![" ", &c_model.class.kademe.to_string(), "/", &c_model.class.sube, " "]
                        ],
                        match c_model.next_class{
                            Some(class) => {
                                li![
                                    a![
                                        attrs!{
                                            At::Href=> format!("/schools/{}/groups/{}/classes/{}/{}", &ctx_school.school.id, &ctx_group.group.id, class, &c_model.tab)
                                        },
                                        "Sonraki Sınıf",
                                    ]
                                ]
                            },
                            None => {
                                div![]
                            }
                        },
                    ]
                ]
            ]
        ],
        div![
            home::view(c_model, ctx, ctx_school, ctx_group).map_msg(Msg::Class),
        ]
    ]
}

/*
fn add_button<'a>(group_size: usize) -> &'a str {
    if group_size == 0 {
        return "Grup Ekle";
    }
    "Ekle"
}
*/