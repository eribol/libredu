use serde::*;
use seed::{*, prelude::*};
use crate::{Context};
use crate::page::school::group::teacher::home;
use crate::model::user::Teacher;
use crate::page::school::detail;
use crate::page::school::detail::{SchoolContext, GroupContext};
use crate::model::school::SchoolDetail;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct NewTeacher{
    first_name: String,
    last_name: String,
    role: i16
}

#[derive(Debug, Default, Clone)]
pub struct Model{
    pages: Pages,
    teachers: Vec<Teacher>,
    school: SchoolDetail,
    form: NewTeacher,
    //school: SchoolDetail
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
pub fn init(mut url: Url, orders: &mut impl Orders<Msg>, ctx: &mut Context, ctx_school: &mut SchoolContext, ctx_group: &mut GroupContext)-> Model {
    let mut model = Model::default();
    //log!("teachers:", ctx_school.school);
    orders.perform_cmd({
        let adres = format!("/api/schools/{}/teachers", ctx_school.school.id);
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
    match url.next_path_part(){
        Some("") | None => {},
        Some(id) => {
            model.pages = Pages::Teacher(Box::new(home::init(id.parse::<i32>().unwrap(), &mut orders.proxy(Msg::Teacher), ctx, ctx_school, url, ctx_group)));
        }
    }
    //if _url.path().len()>=4{
    //
    //}
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

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>, _ctx: &mut Context, ctx_school: &mut detail::SchoolContext, ctx_group: &mut GroupContext) {
    match msg {
        Msg::FetchTeachers(teachers) => {
            if let Ok(t) = teachers {
                model.teachers = t.clone();
                ctx_school.teachers = t
            }
        }
        Msg::Teacher(msg)=>{
            if let Pages::Teacher(m) = &mut model.pages {
                home::update(msg, m, &mut orders.proxy(Msg::Teacher), _ctx, ctx_school, ctx_group)
            }
        }
        Msg::AddTeacher=> {
            orders.perform_cmd({
                let url = format!("/api/schools/{}/teachers", ctx_school.school.id);
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
            orders.perform_cmd({
                let url = format!("/api/schools/{}/groups/{}/teachers/{}", ctx_school.school.id, ctx_group.group.id, id);
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
                model.teachers.retain(|tt| tt.id != t);
                ctx_school.teachers.retain(|tt| tt.id != t);
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
                ctx_school.teachers.insert(0, _t);
            }
        }
    }
}

pub fn view(model: &Model, ctx: &Context, ctx_school: &SchoolContext, ctx_group: &GroupContext)-> Node<Msg>{
    div![
        C!{"column is-full"},
            match &model.pages{
                Pages::Teacher(m) => {
                    home::view(m, ctx_school, ctx_group).map_msg(Msg::Teacher)
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
                                    At::Disabled => disabled(ctx, ctx_school).as_at_value()
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
                                    At::Disabled => disabled(ctx, ctx_school).as_at_value()
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
                                        At::Value=> "5"
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
                                    At::Disabled => disabled(ctx, ctx_school).as_at_value()
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
                        tbody![
                            ctx_school.teachers.iter().map(|t|
                                tr![
                                    C!{"table-light"},
                                    td![
                                        a![
                                            &t.first_name,
                                            attrs!{
                                                At::Href=> format!("/schools/{}/groups/{}/teachers/{}", &ctx_school.school.id, &ctx_group.group.id, &t.id)
                                            }
                                        ]
                                    ],
                                    td![
                                        a![
                                            &t.last_name,
                                            attrs!{
                                                At::Href=> format!("/schools/{}/groups/{}/teachers/{}", &ctx_school.school.id, &ctx_group.group.id, &t.id)
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
                                                    At::Selected => (t.role_id == 1).as_at_value(),
                                                    //At::Disabled => (t.role_id == 1).as_at_value()
                                                },
                                                "Müdür"
                                            ],
                                            option![
                                                attrs!{
                                                    At::Value=> "2",
                                                    At::Selected => (t.role_id == 2).as_at_value()
                                                },
                                                "Müdür Başyardımcısı"
                                            ],
                                            option![
                                                attrs!{
                                                    At::Value=> "3",
                                                    At::Selected => (t.role_id == 3).as_at_value()
                                                },
                                                "Müdür Yardımcısı"
                                            ],
                                            option![
                                                attrs!{
                                                    At::Value=> "4",
                                                    At::Selected => (t.role_id == 4).as_at_value()
                                                },
                                                "Rehber Öğretmen"
                                            ],
                                            option![
                                                attrs!{
                                                    At::Value=> "5",
                                                    At::Selected => (t.role_id == 5).as_at_value()
                                                },
                                                "Öğretmen"
                                            ]
                                        ]
                                    ],
                                    td![
                                        button![
                                            C!{"button"},
                                            attrs!{At::Value=>&t.id},
                                            "Sil",
                                            {
                                                let id = t.id;
                                                ev(Ev::Click, move |_event| {
                                                    Msg::DelTeacher(id)
                                                })
                                            }
                                        ]
                                    ]
                                ]
                            )
                        ]
                    ]
                    ]
                }
            }
        //]
    ]
}

fn disabled(ctx: &Context, ctx_school: &SchoolContext) -> bool {
    if ctx.user.is_none(){
        return true;
    }
    else if ctx.user.as_ref().unwrap().is_admin {
        return false;
    }
    !ctx.schools.iter().any(|s| s.school.id == ctx_school.school.id)
}