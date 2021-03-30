use seed::{*, prelude::*};
use crate::model::school::SchoolType;
use serde::*;
use crate::model::subject::Subject;
//use crate::{Context, Urls};
//use crate::page::school::detail::{SchoolContext};

#[derive(Debug, Default, Clone)]
pub struct Model{
    form: Form,
    subjects: Vec<Subject>
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Form{
    school_type: i32,
    name: String,
    kademe: String,
    optional: bool
}

pub fn init(orders: &mut impl Orders<Msg>, school_type: i32)-> Model {
    let model = Model::default();
    orders.perform_cmd({
        let request = Request::new(format!("/api/admin/subjects/{}", school_type))
            .method(Method::Get);
        async { Msg::FetchSubjects(async {
            request
                .fetch()
                .await?
                .check_status()?
                .json()
                .await
        }.await)}
    });
    model
}

#[derive(Debug)]
pub enum Msg{
    SubmitSubject,
    ChangeType(String),
    ChangeGrade(String),
    ChangeName(String),
    ChangeOptional(String),
    FetchSubject(fetch::Result<Subject>),
    FetchSubjects(fetch::Result<Vec<Subject>>),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {

    match msg {
        Msg::SubmitSubject => {
            orders.perform_cmd({
                let request = Request::new("/api/admin/subjects")
                    .method(Method::Post)
                    .json(&model.form);
                async { Msg::FetchSubject(async {
                    request?
                        .fetch()
                        .await?
                        .check_status()?
                        .json()
                        .await
                }.await)}
            });
        }
        Msg::ChangeGrade(grade) => {
            model.form.kademe = grade
        }
        Msg::ChangeType(school_type) => {
            match school_type.parse::<i32>(){
                Ok(t) => {
                    model.form.school_type = t
                }
                Err(_) => {}
            }
            orders.perform_cmd({
                let request = Request::new(format!("/api/admin/subjects/{}", model.form.school_type))
                    .method(Method::Get);
                async { Msg::FetchSubjects(async {
                    request
                        .fetch()
                        .await?
                        .check_status()?
                        .json()
                        .await
                }.await)}
            });
        }
        Msg::ChangeName(name) => {
            model.form.name = name
        }
        Msg::ChangeOptional(_b) => {
            if model.form.optional{
                model.form.optional = false
            }
            else {
                model.form.optional = true
            }
        }
        Msg::FetchSubject(subject) => {
            match subject{
                Ok(s) => {
                    model.subjects.insert(0, s)
                }
                Err(_) => {}
            }
        }
        Msg::FetchSubjects(subjects) => {
            match subjects{
                Ok(s) => {
                    model.subjects = s
                }
                Err(_) => {}
            }
        }
    }
}

pub fn view(school_types: &Vec<SchoolType>, model: &Model) -> Node<Msg> {
    div![
        div![
            C!{"field"},
            p![
                label![
                    C!{"label"},
                    "Okul Türü:"
                ],
                select![
                    C!{"select"},
                    school_types.iter().map(|t|
                        option![
                            attrs!{
                                At::Value=> &t.id
                            },
                            &t.name
                        ]
                    ),
                    input_ev(Ev::Change, Msg::ChangeType),
                ]
            ],
            p![
                label![
                    C!{"label"},
                    "Ders Sınıf Kademesi:"
                ],
                input![
                    C!{"select"},
                    attrs!{
                        At::Value => &model.form.kademe.to_string()
                    },
                    input_ev(Ev::Change, Msg::ChangeGrade),
                ]
            ],
            p![
                label![
                    C!{"label"},
                    "Dersin Adı:"
                ],
                input![
                    C!{"input"},
                    attrs!{
                        At::Value => &model.form.name
                    },
                    input_ev(Ev::Change, Msg::ChangeName),
                ]
            ],
            p![
                label![
                    C!{"label"},
                    "Seçmeli:"
                ],
                input![
                    C!{"checkbox"},
                    attrs!{
                        At::Type => "checkbox",
                        At::Value => ""
                    },
                    input_ev(Ev::Change, Msg::ChangeOptional),
                ]
            ],
            p![
                C!{"control"},
                input![
                    C!{"button is-primary"},
                    attrs!{
                        At::Type=>"button",
                        At::Value=>"Ekle",
                        At::Id=>"login_button"
                    },
                    ev(Ev::Click, |event| {
                        event.prevent_default();
                        Msg::SubmitSubject
                    })
                ]
            ]
        ],
        div![
            C!{"field"},
            model.subjects.iter().map(|s|
                label![
                    C!{"label"},
                    &s.name, "(", &s.kademe.to_string(), ")"
                ]
            )
        ]
    ]

}