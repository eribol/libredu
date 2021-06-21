use serde::*;
use seed::{*, prelude::*};
use crate::page::school::group::teacher::home;
use crate::page::school::detail;
use crate::page::school::detail::SchoolContext;
use crate::model::teacher::{TeacherContext, Teacher, TeacherGroupContext};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct NewTeacher{
    first_name: String,
    last_name: String,
    role: i16
}

#[derive(Debug, Default, Clone)]
pub struct Model{
    pages: Pages,
    form: NewTeacher,
    url: Url
}

#[derive(Debug, Clone)]
pub enum Pages{
    Teachers,
    Teacher(Box<home::Model>)
}
impl Default for Pages{
    fn default()->Self{
        Self::Teachers
    }
}
pub fn init(mut url: Url, orders: &mut impl Orders<Msg>, school_ctx: &mut SchoolContext)-> Model {
    let mut model = Model{url: url.clone(), ..Default::default()};
    if let Some(_teachers) = &mut school_ctx.teachers{
        match url.next_path_part() {
            Some("") | None => {
                model.pages = Pages::Teachers
            },
            _ => {
                model.pages = Pages::Teacher(Box::new(home::init(url, &mut orders.proxy(Msg::Teacher), school_ctx)));
            }
        }
    }
    else {
        orders.perform_cmd({
            let adres = format!("/api/schools/{}/teachers", school_ctx.school.id);
            let request = Request::new(adres)
                .method(Method::Get);
            async { Msg::FetchTeachers(async {
                request
                    .fetch()
                    .await?
                    .check_status()?
                    .json()
                    .await
            }.await)}
        });
    }
    model.form.role = 5;
    model
}

#[derive(Debug)]
pub enum Msg{
    FetchTeachers(fetch::Result<Vec<Teacher>>),
    AddTeacher,
    DelTeacher(i32),
    FetchDel(fetch::Result<i32>),
    ChangeFirstName(String),
    ChangeLastName(String),
    ChangeRole(String),
    FetchTeacher(fetch::Result<Teacher>),
    Teacher(home::Msg)
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>, school_ctx: &mut detail::SchoolContext) {
    match msg {
        Msg::FetchTeachers(teachers) => {
            if let Ok(teachers) = teachers {
                if let Some(tchrs) = &mut school_ctx.teachers {
                    tchrs.clear();
                    for t in teachers {
                        let teacher_ctx = TeacherContext {
                            teacher: t,
                            group: vec![
                                TeacherGroupContext{
                                    group: model.url.path()[3].parse().unwrap(),
                                    activities: None,
                                    limitations: None,
                                    timetables: None
                                }
                            ],
                            activities: None
                        };

                        tchrs.push(teacher_ctx)
                    }
                }
                else{
                    school_ctx.teachers = Some(vec![]);
                    if let Some(tchrs) = &mut school_ctx.teachers {
                        for t in teachers {
                            let teacher_ctx = TeacherContext {
                                teacher: t,
                                group: vec![
                                    TeacherGroupContext{
                                        group: model.url.path()[3].parse().unwrap(),
                                        activities: None,
                                        limitations: None,
                                        timetables: None
                                    }
                                ],
                                activities: None
                            };
                            tchrs.push(teacher_ctx)
                        }
                    }
                }
            }

        }
        Msg::Teacher(msg) => {
            if let Pages::Teacher(m) = &mut model.pages {
                home::update(msg, m, &mut orders.proxy(Msg::Teacher), school_ctx)
            }
        }
        Msg::AddTeacher=> {
            orders.perform_cmd({
                let url = format!("/api/schools/{}/teachers", school_ctx.school.id);
                let request = Request::new(url)
                    .method(Method::Post)
                    .json(&model.form);
                async {
                    Msg::FetchTeacher(async {
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
        Msg::DelTeacher(id)=> {
            let group_ctx = school_ctx.get_group(&model.url);
            orders.perform_cmd({
                let url = format!("/api/schools/{}/groups/{}/teachers/{}", school_ctx.school.id, group_ctx.group.id, id);
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
        Msg::FetchDel(teacher)=>{
            if let Ok(t) = teacher {
                if let Some(teachers) = &mut school_ctx.teachers{
                    teachers.retain(|tt| tt.teacher.id != t);
                }
            }
        }
        Msg::ChangeFirstName(name)=>{
            model.form.first_name = name;

        }
        Msg::ChangeLastName(name)=>{
            model.form.last_name = name
        }
        Msg::ChangeRole(r) =>{
            model.form.role = r.parse::<i16>().unwrap()
        }
        Msg::FetchTeacher(t)=>{
            if let Ok(_t) = t {
                if let Some(teachers) = &mut school_ctx.teachers{
                    let teacher_ctx = TeacherContext{
                        teacher: _t,
                        group: vec![
                            TeacherGroupContext{
                                group: model.url.path()[3].parse().unwrap(),
                                activities: None,
                                limitations: None,
                                timetables: None
                            }
                        ],
                        activities: None
                    };
                    teachers.insert(0, teacher_ctx);
                }

            }
        }
    }
}

pub fn view(model: &Model, school_ctx: &SchoolContext)-> Node<Msg>{
    let group_ctx = school_ctx.get_group(&model.url);
    div![
        C!{"columns"},
        match &model.pages{
            Pages::Teacher(m) => {
                use crate::model::teacher::TEACHER_MENU;
                let teacher_ctx = school_ctx.get_teacher(&model.url);
                div![
                    C!{"column"},
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
                                                At::Href=> format!("/schools/{}/groups/{}/teachers", &school_ctx.school.id, &group_ctx.group.id)
                                            },
                                            "<--Öğretmenler",
                                        ]
                                    ],
                                    match school_ctx.get_prev_teacher(&m.url){
                                        Some(teacher) => {
                                            li![
                                                a![
                                                    attrs!{
                                                        At::Href=> format!("/schools/{}/groups/{}/teachers/{}/{}", &school_ctx.school.id, &group_ctx.group.id, teacher.teacher.id, &m.tab)
                                                    },
                                                    "Önceki Öğretmen"
                                                ]
                                            ]
                                        },
                                        None => {
                                            div![]
                                        }
                                    },
                                    li![
                                        div![" ", &teacher_ctx.teacher.first_name, " ", &teacher_ctx.teacher.last_name, " "]
                                    ],
                                    match school_ctx.get_next_teacher(&m.url){
                                        Some(teacher) => {
                                            li![
                                                a![
                                                    attrs!{
                                                        At::Href=> format!("/schools/{}/groups/{}/teachers/{}/{}", &school_ctx.school.id, &group_ctx.group.id, &teacher.teacher.id, &m.tab)
                                                    },
                                                    "Sonraki Öğretmen",
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
                        C!{"columns"},
                        div![
                            C!{"column tabs is-centered"},
                            ul![
                                TEACHER_MENU.iter().map(|menu|
                                    li![
                                        if m.tab == menu.link{
                                        C!{"is-active"}} else {C!{""}},
                                        a![
                                            menu.name,
                                            attrs!{
                                                At::Href => format!("/schools/{}/groups/{}/teachers/{}/{}", &school_ctx.school.id, &group_ctx.group.id, &teacher_ctx.teacher.id, menu.link)
                                            }
                                        ]
                                    ]
                                )
                            ]
                        ]
                    ],
                    div![
                        C!{"columns"},
                        teacher_detail(m, school_ctx)
                    ]
                ]
            },
            Pages::Teachers => {
                div![
                    C!{"field"},
                    p![
                        label![C!{"label"}, "Adı:"],
                        input![
                            attrs!{
                                At::Type=>"text",
                                At::Placeholder=>"Adı",
                                At::Value=>&model.form.first_name,
                                At::Disabled => disabled(school_ctx).as_at_value()
                            },
                            input_ev(Ev::Input, Msg::ChangeFirstName)
                        ]
                    ],
                    p![
                        label![C!{"label"},"Soyadı:"],
                        input![
                            attrs!{
                                At::Type=>"text",
                                At::Placeholder=>"Soyadı",
                                At::Value=>&model.form.last_name
                                At::Disabled => disabled(school_ctx).as_at_value()
                            },
                            input_ev(Ev::Input, Msg::ChangeLastName)
                        ]
                    ],
                    p![
                        label![
                            C!{"label"},"Rol:"
                        ],
                        select![
                            C!{"select"},
                            attrs!{
                                At::Name=>"type",
                                At::Id=>"type",
                                //At::Value => &model.form2.group,
                            },
                            option![
                                attrs!{
                                    At::Value=> "2"
                                },
                                "Müdür Başyardımcısı"
                            ],
                            option![
                                attrs!{
                                    At::Value=> "3"
                                },
                                "Müdür Yardımcısı"
                            ],
                            option![
                                attrs!{
                                    At::Value=> "4"
                                },
                                "Rehber Öğretmen"
                            ],
                            option![
                                attrs!{
                                    At::Value=> "5",
                                    At::Selected => true.as_at_value()
                                },
                                "Öğretmen"
                            ],
                            input_ev(Ev::Change, Msg::ChangeRole)
                        ]
                    ],
                    p![
                        input![C!{"button is-primary"},
                            attrs!{
                                At::Type=>"button",
                                At::Value=>"Ekle",
                                At::Id=>"login_button",
                                At::Disabled => disabled(school_ctx).as_at_value()
                            },
                            ev(Ev::Click, |event| {
                                event.prevent_default();
                                Msg::AddTeacher
                            })
                        ]
                    ],
                    table![
                        C!{"table table-hover"},
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
                                    "Rolü"
                                ],
                                th![
                                    attrs!{At::Scope=>"col"},
                                    "İşlem"
                                ]
                            ]
                        ],
                        school_ctx.teachers.as_ref().map_or(
                            tbody![],
                            |teachers|
                            tbody![
                                teachers.iter().map(|t|

                                        tr![
                                            C!{"table-light"},
                                            td![
                                                a![
                                                    &t.teacher.first_name,
                                                    attrs!{
                                                        At::Href=> format!("/schools/{}/groups/{}/teachers/{}", &school_ctx.school.id, &group_ctx.group.id, &t.teacher.id)
                                                    }
                                                ]
                                            ],
                                            td![
                                                a![
                                                    &t.teacher.last_name,
                                                    attrs!{
                                                        At::Href=> format!("/schools/{}/groups/{}/teachers/{}", &school_ctx.school.id, &group_ctx.group.id, &t.teacher.id)
                                                    }
                                                ]
                                            ],
                                            td![
                                                select![
                                                    C!{"select"},
                                                    attrs!{
                                                        At::Disabled => true
                                                        //At::Value => &model.form2.group,
                                                    },
                                                    option![
                                                        attrs!{
                                                            At::Value=> "1",
                                                            At::Selected => (t.teacher.role_id == 1).as_at_value(),
                                                            //At::Disabled => (t.role_id == 1).as_at_value()
                                                        },
                                                        "Müdür"
                                                    ],
                                                    option![
                                                        attrs!{
                                                            At::Value=> "2",
                                                            At::Selected => (t.teacher.role_id == 2).as_at_value()
                                                        },
                                                        "Müdür Başyardımcısı"
                                                    ],
                                                    option![
                                                        attrs!{
                                                            At::Value=> "3",
                                                            At::Selected => (t.teacher.role_id == 3).as_at_value()
                                                        },
                                                        "Müdür Yardımcısı"
                                                    ],
                                                    option![
                                                        attrs!{
                                                            At::Value=> "4",
                                                            At::Selected => (t.teacher.role_id == 4).as_at_value()
                                                        },
                                                        "Rehber Öğretmen"
                                                    ],
                                                    option![
                                                        attrs!{
                                                            At::Value=> "5",
                                                            At::Selected => (t.teacher.role_id == 5).as_at_value()
                                                        },
                                                        "Öğretmen"
                                                    ]
                                                ]
                                            ],
                                            td![
                                                button![
                                                    C!{"button"},
                                                    attrs!{At::Value=>&t.teacher.id},
                                                    "Sil",
                                                    {
                                                        let id = t.teacher.id;
                                                        ev(Ev::Click, move |_event| {
                                                            Msg::DelTeacher(id)
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
            }
        }
    ]
}
fn teacher_detail(model: &home::Model, school_ctx: &SchoolContext) ->Node<Msg>{
    let group_ctx = school_ctx.get_group(&model.url);
    //let teacher_ctx = school_ctx.get_group(&model.url).get_class(&model.url);
    div![
        C!{"column is-12"},
        div![
            home::view(model, school_ctx).map_msg(Msg::Teacher),
        ]
    ]
}

fn disabled(ctx_school: &SchoolContext) -> bool {
    false
    /*
    if ctx.user.is_none(){
        return true;
    }
    else if ctx.user.as_ref().unwrap().is_admin {
        return false;
    }
    !ctx.schools.iter().any(|s| s.school.id == ctx_school.school.id)

     */
}