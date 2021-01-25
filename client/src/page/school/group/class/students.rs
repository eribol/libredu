use crate::model::timetable::{Day};
use serde::*;
use seed::{*, prelude::*};
use crate::{Context};
use crate::model::class::{Class, ClassTimetable, ClassActivity};
use crate::page::school::detail::{SchoolContext, GroupContext};
use crate::model::student::SimpleStudent;

#[derive(Debug)]
pub enum Msg{
    Home,
    FetchClass(fetch::Result<Class>),
    FetchStudents(fetch::Result<Vec<SimpleStudent>>),
    FetchAllStudents(fetch::Result<Vec<SimpleStudent>>),
    AddStudent(i32),
    FetchAddStudent(fetch::Result<SimpleStudent>),
    DelStudent(i32),
    FetchDelStudent(fetch::Result<i32>),
}

#[derive(Debug, Default, Clone)]
pub struct Model{
    pub class: Class,
    pub students: Vec<SimpleStudent>,
    pub all_students: Vec<SimpleStudent>,
    pub classes: Vec<Class>,
    //pub form: SimpleStudent
}

pub fn init(class: i32, orders: &mut impl Orders<Msg>, ctx_school: &mut SchoolContext, ctx_group: &mut GroupContext)-> Model{
    let model = Model::default();
    orders.perform_cmd({
        let url = format!("/api/schools/{}/groups/{}/classes/{}", ctx_school.school.id, ctx_group.group.id,class);
        let request = Request::new(url)
            .method(Method::Get);
        async {
            Msg::FetchClass(async {
                request
                    .fetch()
                    .await?
                    .check_status()?
                    .json()
                    .await
            }.await)
        }
    });
    model
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>, _ctx: &mut Context, ctx_school: &mut SchoolContext, ctx_group: &mut GroupContext) {
    match msg {
        Msg::Home => {
        }
        Msg::FetchClass(class)=>{
            model.class = class.unwrap();
            orders.perform_cmd({
                let url = format!("/api/schools/{}/groups/{}/classes/{}/all_students", &ctx_school.school.id, &ctx_group.group.id, model.class.id);
                let request = Request::new(url)
                    .method(Method::Get);
                async {
                    Msg::FetchAllStudents(async {
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
        Msg::FetchAllStudents(st)=>{
            model.all_students = st.unwrap();
            model.all_students.sort_by(|a, b| a.school_number.cmp(&b.school_number));
            log!(&model.all_students);
            orders.perform_cmd({
                let url = format!("/api/schools/{}/groups/{}/classes/{}/students", &ctx_school.school.id, &ctx_group.group.id, model.class.id);
                let request = Request::new(url)
                    .method(Method::Get);
                async {
                    Msg::FetchStudents(async {
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
        Msg::FetchStudents(st) => {
            model.students = st.unwrap();
            model.students.sort_by(|a, b| a.school_number.cmp(&b.school_number));
            for s in &model.students{
                model.all_students.retain(|ss| ss.id != s.id);
            }
        }
        Msg::AddStudent(id) => {
            let student = SimpleStudent{
                id,
                first_name: "".to_string(),
                last_name: "".to_string(),
                school_number: 0
            };
            orders.perform_cmd({
                let url = format!("/api/schools/{}/groups/{}/classes/{}/students", ctx_school.school.id, ctx_group.group.id, model.class.id);
                let request = Request::new(url)
                    .method(Method::Post)
                    .json(&student);
                async {
                    Msg::FetchAddStudent(async {
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
        Msg::FetchAddStudent(student) => {
            match student{
                Ok(s) => {
                    let student = model.all_students.iter().find(|s2| s2.id == s.id).unwrap();
                    model.students.push(student.clone());
                    model.all_students.retain(|s2| s2.id != s.id);
                    model.students.sort_by(|a, b| a.school_number.cmp(&b.school_number));
                    model.all_students.sort_by(|a, b| a.school_number.cmp(&b.school_number));
                }
                Err(_) => {}
            }
        }
        Msg::DelStudent(id) => {
            orders.perform_cmd({
                let url = format!("/api/schools/{}/groups/{}/classes/{}/students/{}", ctx_school.school.id, ctx_group.group.id, model.class.id, id);
                let request = Request::new(url)
                    .method(Method::Delete);
                async {
                    Msg::FetchDelStudent(async {
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
        Msg::FetchDelStudent(student) => {
            match student{
                Ok(s) => {
                    let student = model.students.iter().find(|s2| s2.id == s).unwrap();
                    model.all_students.insert(0, student.clone());
                    model.students.retain(|s2| s2.id != s);
                    model.students.sort_by(|a, b| a.school_number.cmp(&b.school_number));
                    model.all_students.sort_by(|a, b| a.school_number.cmp(&b.school_number));
                }
                Err(e) => {
                    log!(e);
                }
            }
        }
    }
}
pub fn view(model: &Model, ctx_school:&SchoolContext, ctx_group: &GroupContext)->Node<Msg>{
    div![
        div![
            C!{"tabs is-centered"},
            //tabs(model),
        ],
        div![C!{"columns"},
            div![
                C!{"column table-container"},
                table![
                    C!{"table is-bordered"},
                    thead![
                        tr![
                            th![
                                attrs!{At::Scope=>"col"},
                                "S. No"
                            ],
                            th![
                                attrs!{At::Scope=>"col"},
                                "Okul Numarası"
                            ],
                            th![
                                attrs!{At::Scope=>"col"},
                                "Adı"
                            ],
                            th![
                                attrs!{At::Scope=>"col"},
                                "Soyadı"
                            ],
                            th![
                                attrs!{At::Scope=>"col"},
                                "İşlem"
                            ]
                        ]
                    ],
                    model.students.iter().enumerate().map(|s|
                        tr![
                            td![
                                a![&(s.0+1).to_string()]
                            ],
                            td![
                                a![&s.1.school_number]
                            ],
                            td![
                                a![&s.1.first_name]
                            ],
                            td![
                                a![&s.1.last_name]
                            ],
                            td![
                                button![
                                    "Sınıftan Çıkar",
                                    {
                                        let id = s.1.id;
                                        ev(Ev::Click, move |_event| {
                                            Msg::DelStudent(id)
                                        })
                                    }
                                ]
                            ]
                        ]
                    ),
                    model.all_students.iter().enumerate().map(|s|
                        tr![
                            td![

                            ],
                            td![
                                a![&s.1.school_number]
                            ],
                            td![
                                a![&s.1.first_name]
                            ],
                            td![
                                a![&s.1.last_name]
                            ],
                            td![
                                button![
                                    "Sınıfa Ekle",
                                    {
                                        let id = s.1.id;
                                        ev(Ev::Click, move |_event| {
                                            Msg::AddStudent(id)
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

pub fn tabs(model: &Model, ctx_school: &SchoolContext, ctx_group: &GroupContext)->Node<Msg>{
    ul![
        li![
            a![
                "Bilgiler",
                attrs!{
                    At::Href => format!("/schools/{}/groups/{}/classes/{}", &ctx_school.school.id, &ctx_group.group.id, &model.class.id)
                }
            ]
        ],
        li![
            C!{"is-active"},
            a![
                attrs!{
                    At::Href => format!("/schools/{}/groups/{}/classes/{}/students", &ctx_school.school.id, &ctx_group.group.id, &model.class.id)
                },
                "Öğrenciler"
            ]
        ],
        li![
            a![
                "Aktiviteler",
                attrs!{
                    At::Href => format!("/schools/{}/groups/{}/classes/{}/activities", &ctx_school.school.id, &ctx_group.group.id, &model.class.id)
                }
            ]
        ],
        li![
            a![
                attrs!{
                    At::Href => format!("/schools/{}/groups/{}/classes/{}/limitations", &ctx_school.school.id, &ctx_group.group.id, &model.class.id)
                },
                "Kısıtlamalar",
            ]
        ],
        li![
            //C!{"is-active"},
            a![
                attrs!{
                    At::Href => format!("/schools/{}/groups/{}/classes/{}/timetables", &ctx_school.school.id, &ctx_group.group.id, &model.class.id)
                },
                "Ders Tablosu"
            ]
        ]
    ]
}