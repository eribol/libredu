use seed::{*, prelude::*};
use crate::model::school::SchoolType;
use serde::*;
use crate::model::subject::{Subject, NewSubject};
//use crate::{Context, Urls};
use crate::page::school::detail::{SchoolContext};

#[derive(Debug, Default, Clone)]
pub struct Model{
    form: NewSubject,
    subjects: Vec<Subject>,
    filtered_subjects: Vec<Subject>
}

pub fn init(orders: &mut impl Orders<Msg>, ctx_school: &SchoolContext)-> Model {
    let mut model = Model::default();
    orders.perform_cmd({
        let request = Request::new(format!("/api/schools/{}/subjects", ctx_school.school.id))
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
    model.form.school = ctx_school.school.id;
    model
}

#[derive(Debug)]
pub enum Msg{
    SubmitSubject,
    ChangeGrade(String),
    ChangeName(String),
    ChangeOptional(String),
    FetchSubject(fetch::Result<Subject>),
    FilterGrade(String),
    FilterOptional(String),
    FetchSubjects(fetch::Result<Vec<Subject>>),
    DelSubject(i32),
    FetchDelSubject(fetch::Result<i32>)
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>, ctx_school: &mut SchoolContext) {
    match msg {
        Msg::SubmitSubject => {
            orders.perform_cmd({
                let request = Request::new(format!("/api/schools/{}/subjects", ctx_school.school.id))
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
                    model.filtered_subjects.insert(0, s)
                }
                Err(_) => {}
            }
        }
        Msg::FetchSubjects(subjects) => {
            match subjects{
                Ok(s) => {
                    model.subjects = s;
                    model.filtered_subjects = model.subjects.clone()
                }
                Err(_) => {}
            }
        }
        Msg::FilterGrade(g) => {
            if g != "" {
                model.filtered_subjects = model.subjects.clone();
                model.filtered_subjects = model.filtered_subjects.iter().cloned().filter(|s| s.kademe == g).collect()
            }
            else {
                model.filtered_subjects = model.subjects.clone()
            }
        }
        Msg::FilterOptional(_) => {
            if model.filtered_subjects.len() > 0 && model.filtered_subjects[0].optional{
                model.filtered_subjects = model.subjects.iter().cloned().filter(|s| !s.optional && s.kademe == model.filtered_subjects[0].kademe).collect()
            }
            else {
                model.filtered_subjects = model.filtered_subjects.iter().cloned().filter(|s| s.optional).collect()
            }
        }
        Msg::DelSubject(id) => {
            orders.perform_cmd({
                let request = Request::new(format!("/api/schools/{}/subjects/{}", ctx_school.school.id, id))
                    .method(Method::Delete);
                async { Msg::FetchDelSubject(async {
                    request
                        .fetch()
                        .await?
                        .check_status()?
                        .json()
                        .await
                }.await)}
            });
        }
        Msg::FetchDelSubject(id) => {
            match id{
                Ok(i) =>{
                    model.filtered_subjects.retain(|s| s.id != i)
                }
                Err(_) => {}
            }
        }
    }
}

pub fn view(model: &Model) -> Node<Msg> {
    div![
        div![
            C!{"field"},
            p![
                label![
                    C!{"label"},
                    "Ders Sınıf Kademesi:"
                ],
                input![
                    C!{"input"},
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
            p![
                label![
                    "Kademeye göre filtrele"
                ]
            ],
            p![
                input![
                    input_ev(Ev::Input, Msg::FilterGrade)
                ]
            ]
        ],
        div![
            C!{"field"},
            p![
                label![
                    "Seçmeli durumuna göre filtrele"
                ]
            ],
            p![
                input![
                    attrs![
                        At::Type => "checkbox"
                    ],
                    input_ev(Ev::Change, Msg::FilterOptional),
                ]
            ]
        ],
        div![
            C!{"field"},
            table![
                C!{"table"},
                thead![
                    tr![
                        th![
                            "Dersin Adı"
                        ],
                        th![
                            "Kademesi"
                        ],
                        th![
                            "Seçmeli"
                        ],
                        th![
                            "İşlem"
                        ]
                    ]
                ],
                tbody![
                    C!{"table-light"},
                    model.filtered_subjects.iter().map(|s|
                        tr![
                            td![
                                &s.name
                            ],
                            td![
                                &s.kademe
                            ],
                            td![
                                label![
                                    C!{"checkbox"},
                                    input![
                                        attrs![
                                            At::Type => "checkbox",
                                            At::Checked => s.optional.as_at_value()
                                        ]
                                    ]
                                    //"Seçmeli"
                                ]
                            ],
                            td![
                                button![
                                    C!{"button"},
                                    "Sil",
                                    {
                                        let id = s.id;
                                        ev(Ev::Click, move |_event| {
                                            Msg::DelSubject(id)
                                        })
                                    }
                                ]
                            ]
                        ]
                    )
                ]
            ]
        ]
    ]

}