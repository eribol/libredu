use seed::{*, prelude::*};
use crate::page::school::detail::SchoolContext;
use crate::model::student::{SimpleStudent, Student};

#[derive(Debug)]
pub enum Msg{
    Home,
    FetchStudents(fetch::Result<Vec<SimpleStudent>>),
    FetchAllStudents(fetch::Result<Vec<Student>>),
    AddStudent(usize),
    FetchAddStudent(fetch::Result<SimpleStudent>),
    DelStudent(i32),
    FetchDelStudent(fetch::Result<i32>),
    Loading
}

#[derive(Debug, Default, Clone)]
pub struct Model{
    url: Url
}

pub fn init(url: Url, orders: &mut impl Orders<Msg>, school_ctx: &SchoolContext)-> Model{
    let model = Model{url: url.clone()};
    let group_ctx = school_ctx.get_group(&url);
    let class_ctx = group_ctx.get_class(&url);
    orders.send_msg(Msg::Loading);
    model
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>, school_ctx: &mut SchoolContext) {
    let school =  school_ctx.clone().school.id;
    match msg {
        Msg::Home => {}
        Msg::FetchStudents(st)=>{
            let group_ctx = school_ctx.get_mut_group(&model.url);
            let class_ctx = group_ctx.get_mut_class(&model.url);
            if let Ok(s) = st{
                class_ctx.students = Some(s);
            }
            else {
                log!("Sınıf öğrencileri yüklenmedi");
            }
        }
        Msg::FetchAllStudents(st)=>{
            if let Ok(s) = st{
                school_ctx.students = Some(s);
            }
        }
        Msg::AddStudent(id) => {
            let student = &school_ctx.students.as_ref().unwrap()[id];
            let std = SimpleStudent{
                id: student.id,
                first_name: student.first_name.clone(),
                last_name: student.last_name.clone(),
                school_number: student.school_number
            };
            orders.perform_cmd({
                let url = format!("/api/schools/{}/groups/{}/classes/{}/students", &school, &model.url.path()[3], &model.url.path()[5]);
                let request = Request::new(url)
                    .method(Method::Post)
                    .json(&std);
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
            if let Ok(s) = student {
                let group_ctx = school_ctx.get_mut_group(&model.url);
                let class_ctx = group_ctx.get_mut_class(&model.url);
                if let Some(students) = &mut class_ctx.students{
                    students.push(SimpleStudent{
                        id: s.id,
                        first_name: s.first_name,
                        last_name: s.last_name,
                        school_number: s.school_number
                    })
                }
                else{
                    class_ctx.students = Some(vec![
                        SimpleStudent{
                            id: s.id,
                            first_name: s.first_name,
                            last_name: s.last_name,
                            school_number: s.school_number
                        }
                    ])
                }
            }
        }
        Msg::DelStudent(id) => {
            let group_ctx = school_ctx.get_mut_group(&model.url);
            let class_ctx = group_ctx.get_mut_class(&model.url);
            orders.perform_cmd({
                let url = format!("/api/schools/{}/groups/{}/classes/{}/students/{}", &school, &class_ctx.class.group_id, &class_ctx.class.id, id);
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
            let group_ctx = school_ctx.get_mut_group(&model.url);
            let class_ctx = group_ctx.get_mut_class(&model.url);
            if let Ok(s) = student {
                let students = class_ctx.get_mut_students();
                students.retain(|s2| s2.id != s);
                students.sort_by(|a, b| a.school_number.cmp(&b.school_number));
            }
        }
        Msg::Loading => {
            if school_ctx.students.is_none(){
                orders.perform_cmd({
                    let url = format!("/api/schools/{}/students", model.url.path()[1]);
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
            let group_ctx = school_ctx.get_mut_group(&model.url);
            let class_ctx = group_ctx.get_mut_class(&model.url);
            if class_ctx.students.is_none(){
                orders.perform_cmd({
                    let url = format!("/api/schools/{}/groups/{}/classes/{}/students", school_ctx.school.id, model.url.path()[3], model.url.path()[5]);
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
        }
    }
}
pub fn view(model: &Model, school_ctx: &SchoolContext)->Node<Msg>{
    let class_ctx = school_ctx.get_group(&model.url).get_class(&model.url);
    let mut class_students: Vec<SimpleStudent> = vec![];
    if let Some(ss) = &class_ctx.students{
        class_students = ss.clone();
    }
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
                    class_students.iter().enumerate().map(
                        |s|
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
                    school_ctx.students.as_ref().map_or(
                        tbody![],
                        |students|
                        tbody![
                            students.iter().enumerate().filter(|s| !class_students.iter().any(|s2| s2.id == s.1.id))
                            .map(
                                |s|
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
                                                let id = s.0;
                                                ev(Ev::Click, move |_event| {
                                                    Msg::AddStudent(id)
                                                })
                                            }
                                        ]
                                    ]
                                ]
                            )
                        ]
                    )
                ]
            ]
        ]
    ]

}