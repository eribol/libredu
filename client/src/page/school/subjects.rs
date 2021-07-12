use seed::{*, prelude::*};
use crate::model::subject::{Subject, NewSubject};
//use crate::{Context, Urls};
use crate::page::school::detail::{SchoolContext};
use crate::i18n::I18n;

#[derive(Debug, Default, Clone)]
pub struct Model{
    form: NewSubject,
    subjects: Vec<Subject>,
    filtered_subjects: Vec<Subject>,
    opt: bool,
    grade: String
}

pub fn init(orders: &mut impl Orders<Msg>, ctx_school: &SchoolContext)-> Model {
    let mut model = Model::default();
    if let Some(subjects) = &ctx_school.subjects{
        orders.send_msg(Msg::Filtering);
    }
    else {
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
    }
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
    FetchDelSubject(fetch::Result<i32>),
    Filtering
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
            if let Ok(s) = subject {
                model.filtered_subjects.insert(0, s.clone());
                if let Some(subjects) = &mut ctx_school.subjects{
                    subjects.insert(0, s);
                }
                else {
                    ctx_school.subjects = Some(vec![s])
                }
            }
        }
        Msg::FetchSubjects(subjects) => {
            if let Ok(s) = subjects {
                ctx_school.subjects = Some(s.clone());
                model.filtered_subjects = s;
            }
        }
        Msg::FilterGrade(g) => {
            model.grade = g;
            orders.send_msg(Msg::Filtering);
        }
        Msg::FilterOptional(_) => {
            model.opt = !model.opt;
            orders.send_msg(Msg::Filtering);
        }
        Msg::Filtering => {
            if let Some(subjects) = &ctx_school.subjects{
                model.filtered_subjects = subjects.clone().into_iter().filter(|s| s.optional == model.opt && s.kademe.contains(&model.grade)).collect();
                log!(&model);
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
            if let Ok(i) = id {
                model.filtered_subjects.retain(|s| s.id != i)
            }
        }
    }
}

pub fn view(model: &Model, lang: &I18n) -> Node<Msg> {
    use crate::{create_t, with_dollar_sign};
    create_t![lang];
    div![
        div![
            C!{"field"},
            p![
                label![
                    C!{"label"},
                    t!["subject-class-grade"]
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
                    t!["subject-name"]
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
                    t!["subject-optional"]
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
                        At::Value=> t!["add"],
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
                    t!["filter-subject-grade"]
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
                    t!["filter-subject-optional"]
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
                            t!["subject-name"]
                        ],
                        th![
                            t!["subject-class-grade"]
                        ],
                        th![
                            t!["subject-optional"]
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
                                    t!["delete"],
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