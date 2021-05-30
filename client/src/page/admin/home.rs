use seed::{*, prelude::*};
//use crate::{Context, Urls};
use serde::*;
use crate::page::admin::subjects;
use crate::model::school::SchoolType;

#[derive(Debug, Default, Clone)]
pub struct Model{
    pages: Pages,
    menu: Vec<Menu>,
    school_types: Vec<SchoolType>,
    form: SchoolTypeForm,
    url: Url
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SchoolTypeForm{
    name: String,
}

#[derive(Debug, Default, Clone)]
pub struct Menu{
    name: String,
    link: String
}

#[derive(Debug, Clone)]
pub enum Pages{
    SchoolTypes,
    Subjects(subjects::Model)
}
impl Default for Pages{
    fn default()->Self{
        Self::SchoolTypes
    }
}
pub fn init(url: Url, orders: &mut impl Orders<Msg>)-> Model {
    orders.perform_cmd({
        let request = Request::new("/api/admin/school_types".to_string())
            .method(Method::Get);

        async { Msg::FetchSchoolTypes(async {
            request
                .fetch()
                .await?
                .check_status()?
                .json()
                .await
        }.await)}
    });
    Model {
        menu: vec![
            Menu { name: "Okul Türleri".to_string(), link: "school_types".to_string() },
            Menu { name: "Dersler".to_string(), link: "subjects".to_string() },
        ],
        url, ..Default::default()
    }
}

#[derive(Debug)]
pub enum Msg{
    SchoolTypes,
    Subjects(subjects::Msg),
    FetchSchoolTypes(fetch::Result<Vec<SchoolType>>),
    FetchType(fetch::Result<SchoolType>),
    NameChanged(String),
    SubmitForm
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {

    match msg {

        Msg::Subjects(msg)=>{
            if let Pages::Subjects(m) = &mut model.pages {
                subjects::update(msg, m, &mut orders.proxy(Msg::Subjects))
            }
        }
        Msg::SchoolTypes => {}
        Msg::FetchSchoolTypes(types) => {
            if let Ok(t) = types {
                model.school_types = t
            }
            match model.url.next_path_part(){
                Some("") | None => {
                    model.pages = Pages::SchoolTypes
                }
                Some("school_types") => {
                    model.pages = Pages::SchoolTypes
                }
                Some("subjects") => {
                    model.pages = Pages::Subjects(subjects::init(&mut orders.proxy(Msg::Subjects), model.school_types[0].id))
                }
                _ => {
                }
            }
        }
        Msg::NameChanged(name) => {
            model.form.name = name
        }
        Msg::SubmitForm => {
            orders.perform_cmd({
                let request = Request::new("/api/admin/school_types")
                    .method(Method::Post)
                    .json(&model.form);
                async { Msg::FetchType(async {
                    request?
                        .fetch()
                        .await?
                        .check_status()?
                        .json()
                        .await
                }.await)}
            });
        }
        Msg::FetchType(school_type) => {
            if let Ok(t) = school_type {
                model.school_types.push(t)
            }
        }
    }
}

pub fn view(model: &Model) -> Node<Msg>{
    div![
        C!{"columns"},
        div![
            C!{"column is-2"},
            ul![
                C!{"menu-list"},
                model.menu.iter().map(|m|
                    li![
                        a![
                            &m.name,
                            attrs![
                                At::Href=> format!("/admin/{}", m.link)
                            ]
                        ]
                    ]
                )
            ],
        ],
        div![
            C!{"column is-4"},
            match &model.pages{
                Pages::Subjects(m) => {
                    subjects::view(&model.school_types, m).map_msg(Msg::Subjects)
                }
                Pages::SchoolTypes => {
                    school_type_view(model)
                }
            }
        ]
    ]
}

fn school_type_view(model: &Model) -> Node<Msg>{
    div![
        div![
            C!{"field"},
            label![
                C!{"label"},
                "Okul Türü Adı:"
            ],
            p![
                C!{"control"},
                input![C!{"input"},
                    attrs!{
                        At::Type=>"text",
                        At::Value => &model.form.name,
                    },
                    input_ev(Ev::Input, Msg::NameChanged),
                ]
            ]
        ],
        div![C!{"field"},
            p![
                C!{"control"},
                input![
                    C!{"button is-primary"},
                    attrs!{
                        At::Type=>"button",
                        At::Value=>"Giriş Yap",
                    },
                    ev(Ev::Click, |event| {
                        event.prevent_default();
                        Msg::SubmitForm
                    })
                ]
            ]
        ],
        model.school_types.iter().map(|t|
            div![
                C!{"field"},
                &t.name
            ]
        )
    ]
}