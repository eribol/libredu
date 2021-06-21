use seed::{*, prelude::*};
use serde::*;
use crate::page::school::detail::SchoolContext;
use crate::model::student::Student;


#[derive(Debug, Default, Clone)]
pub struct Model{
    form: NewStudent,
    form2: NewStudents,
    unused_numbers: Vec<i32>
}
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct NewStudent{
    first_name: String,
    last_name: String,
    number: i32,
}
#[derive(Debug, Default, Clone)]
pub struct NewStudents{
    file: Option<web_sys::File>,
    group: i32
}
#[derive(Debug)]
pub enum Msg{
    SubmitStudents,
    SubmitStudent,
    FileChanged(Option<web_sys::File>),
    GroupChanged(String),
    FirstNameChanged(String),
    LastNameChanged(String),
    NumberChanged(String),
    FetchStudents(fetch::Result<Vec<Student>>),
    FetchStudent(fetch::Result<Student>),
    DelStudent(i32),
    FetchDelStudent(fetch::Result<i32>),
    FetchNumbers(fetch::Result<Vec<i32>>)
}

pub fn init(_url: Url, orders: &mut impl Orders<Msg>, school_ctx: &SchoolContext) ->Model{
    let model = Model::default();
    if school_ctx.students.is_none(){
        orders.perform_cmd({
            let request = Request::new(format!("/api/schools/{}/students", school_ctx.school.id))
                .method(Method::Get);
            async { Msg::FetchStudents(async {
                request
                    .fetch()
                    .await?
                    .check_status()?
                    .json()
                    .await
            }.await)}
        });
    }
    model
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>, school_ctx: &mut SchoolContext) {
    
    match msg{
        Msg::FetchStudents(ss) => {
            if let Ok(s) = ss {
                school_ctx.students = Some(s);
                orders.perform_cmd({
                    let request = Request::new(format!("/api/schools/{}/unused_numbers", school_ctx.school.id))
                        .method(Method::Get);
                    async { Msg::FetchNumbers(async {
                        request
                            .fetch()
                            .await?
                            .check_status()?
                            .json()
                            .await
                    }.await)}
                });
            }
            else {
                school_ctx.students = Some(vec![]);
            }
        }
        Msg::FetchStudent(student) => {
            if let Ok(s) = student {
                model.unused_numbers.retain(|n| n != &s.school_number);
                if let Some(students) = school_ctx.students.iter_mut().next(){
                    students.push(s);
                }
            }
        }
        Msg::DelStudent(id) => {
            orders.perform_cmd({
                let request = Request::new(format!("/api/schools/{}/students/{}", school_ctx.school.id, id))
                    .method(Method::Delete);
                async { Msg::FetchDelStudent(async {
                    request
                        .fetch()
                        .await?
                        .check_status()?
                        .json()
                        .await
                }.await)}
            });
        }
        Msg::FetchDelStudent(number) => {
            //if let Ok(n) = number {
            //    model.students.retain(|s| s.id != n);
            //}
        }
        Msg::FetchNumbers(numbers) => {
            model.unused_numbers = numbers.unwrap_or_default()
        }
        Msg::SubmitStudent=>{
            orders.perform_cmd({
                let request = Request::new(format!("/api/schools/{}/students", school_ctx.school.id))
                    .method(Method::Post)
                    .json(&model.form);
                async { Msg::FetchStudent(async {
                    request?
                        .fetch()
                        .await?
                        .check_status()?
                        .json()
                        .await
                }.await)}
            });
        },
        Msg::SubmitStudents=>{
            let form_data = web_sys::FormData::new().unwrap();
            form_data.append_with_str("group", "15").unwrap();
            if let Some(file) = &model.form2.file {
                form_data.append_with_blob("file", file).unwrap();
            }
            orders.perform_cmd({
                let request = Request::new(format!("/api/schools/{}/students_with_file", school_ctx.school.id))
                    .method(Method::Post)
                    .body(form_data.into());
                async { Msg::FetchStudents(async {
                    request
                        .fetch()
                        .await?
                        //.check_status()?
                        .json()
                        .await
                }.await)}
            });
        },
        Msg::FileChanged(file) => {
            model.form2.file = file;
        }
        Msg::GroupChanged(s) => {
            model.form2.group= s.parse::<i32>().unwrap();
        }
        Msg::FirstNameChanged(name) => {
            model.form.first_name = name
        }
        Msg::LastNameChanged(name) => {
            model.form.last_name = name
        }
        Msg::NumberChanged(number) => {
            model.form.number = number.parse::<i32>().unwrap()
        }
    }
}

fn add(model: &Model, _ctx_school: &SchoolContext)-> Node<Msg>{
    div![
        C!{"columns"},
        div![C!{"column is-6"},
            //label!["Dosyadan öğrencileri ekle"],
            label!["Dosyadan yükle:", attrs! {At::For => "form-file" }],
            div![
                C!{"field"},
                p![
                    C!{"control"},
                    input![
                        ev(Ev::Change, |event| {
                            let file = event
                            .target()
                            .and_then(|target| target.dyn_into::<web_sys::HtmlInputElement>().ok())
                            .and_then(|file_input| file_input.files())
                            .and_then(|file_list| file_list.get(0));
                        Msg::FileChanged(file)
                        }),
                        attrs! {
                            At::Type => "file",
                            At::Id => "form-file",
                            At::Accept => "text/plain",
                        }
                    ]
                ]
            ],
            div![C!{"field"},
                p![C!{"control has-icons-left"},
                    input![C!{"button is-primary"},
                        attrs!{
                            At::Type=>"button",
                            At::Value=>"Yakında",
                            //At::Id=>"login_button",
                            //At::Disabled => true.as_at_value()
                        },
                        ev(Ev::Click, |event| {
                            event.prevent_default();
                            Msg::SubmitStudents
                        })
                    ]
                ]
            ]
        ],
        div![
            C!{"column is-6"},
            label!["Öğrenci bilgilerini girerek öğrenci ekle"],
            div![C!{"field"},
                label![C!{"label"}, "Adı"],
                p![C!{"control"},
                    input![C!{"input"},
                        attrs!{
                            At::Name=>"type",
                            At::Id=>"type",
                            At::Value => &model.form.first_name,
                        },
                        input_ev(Ev::Change, Msg::FirstNameChanged)
                    ]
                ]
            ],
            div![C!{"field"},
                label![C!{"label"}, "Soyadı"],
                p![C!{"control"},
                    input![C!{"input"},
                        attrs!{
                            At::Value => &model.form.last_name,
                        },
                        input_ev(Ev::Change, Msg::LastNameChanged)
                    ]
                ]
            ],
            div![C!{"field"},
                label![C!{"label"}, "Numarası"],
                p![C!{"control"},
                    input![C!{"input"},
                        attrs!{
                            At::Name=>"type",
                            At::Id=>"type",
                            At::Value => &model.form.number.to_string(),
                        },
                        input_ev(Ev::Change, Msg::NumberChanged)
                    ],
                    "Kullanılmayan numaralar:",
                    model.unused_numbers.iter().map(|n|
                        label![n.to_string(), ", "]
                    )
                ]
            ],
            div![C!{"field"},
                p![C!{"control has-icons-left"},
                    input![C!{"button is-primary"},
                        attrs!{
                            At::Type=>"button",
                            At::Value=>"Ekle",
                            At::Id=>"login_button"
                        },
                        ev(Ev::Click, |event| {
                            event.prevent_default();
                            Msg::SubmitStudent
                        })
                    ]
                ]
            ]
        ],

    ]
}
pub fn view(model: &Model, ctx_school: &SchoolContext)-> Node<Msg>{
    div![
        div![
            //C!{"columns"},
            add(model, ctx_school)
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
                                "Adı"
                            ],
                            th![
                                attrs!{At::Scope=>"col"},
                                "Soyadı"
                            ],
                            th![
                                attrs!{At::Scope=>"col"},
                                "Numarası"
                            ],
                            th![
                                attrs!{At::Scope=>"col"},
                            ]
                        ]
                    ],
                    ctx_school.students.as_ref().map_or(tbody![tr![]],
                        |students|
                        tbody![
                            students.iter().map(|s|
                                tr![
                                    td![
                                        a![&s.first_name]
                                    ],
                                    td![
                                        a![&s.last_name]
                                    ],
                                    td![
                                        a![&s.school_number.to_string()]
                                    ],
                                    td![
                                        button![
                                            "Sil",
                                            {
                                                let id = s.id;
                                                ev(Ev::Click, move |_event| {
                                                    Msg::DelStudent(id)
                                                })
                                            },
                                            attrs!{
                                            //At::Disabled => (ctx_school.role > 5)
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