use seed::{*, prelude::*};
use crate::{Context};
use crate::page::school::detail;
use serde::*;
use crate::page::school::detail::{SchoolContext, GroupContext};
use crate::page::school::group::teacher::{activities, limitations, timetables};

#[derive(Debug)]
pub enum Msg{
    Home,
    FetchTeacher(fetch::Result<Teacher>),
    Limitations(limitations::Msg),
    Activities(activities::Msg),
    Timetables(timetables::Msg),
    ChangeFirstName(String),
    ChangeLastName(String),
    ChangeEmail(String),
    ChangeTel(String),
    ChangePass1(String),
    ChangePass2(String),
    SubmitUpdate
    //Timetables
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Teacher{
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub is_active: bool,
    pub email: Option<String>,
    pub tel: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct UpdateTeacherForm{
    pub first_name: String,
    pub last_name: String,
    pub is_active: bool,
    pub email: String,
    pub tel: String,
    password1: String,
    password2: String,
}

#[derive(Debug, Clone)]
pub enum Pages{
    Home,
    Activity(activities::Model),
    Limitation(limitations::Model),
    Timetable(timetables::Model)
}
impl Default for Pages{
    fn default()->Self{
        Self::Home
    }
}

#[derive(Debug, Default, Clone)]
pub struct Model{
    pub teacher: Teacher,
    pub page: Pages,
    form: UpdateTeacherForm,
    back_teacher: Option<crate::model::user::Teacher>,
    next_teacher: Option<crate::model::user::Teacher>,
    tab: String
}

pub fn init(teacher: i32, orders: &mut impl Orders<Msg>, ctx: &mut Context, ctx_school: &mut SchoolContext, mut url: Url, ctx_group: &mut GroupContext)-> Model{
    let mut model = Model::default();
    match url.next_path_part() {
        Some("") | None =>{
            model.page = Pages::Home
        },
        Some("activities") => {
            model.page = Pages::Activity(activities::init(teacher, &mut orders.proxy(Msg::Activities), ctx, ctx_school, ctx_group));
            model.tab = "activities".to_string()
        },
        Some("limitations") =>{
            model.page = Pages::Limitation(limitations::init(teacher, &mut orders.proxy(Msg::Limitations), ctx, ctx_school, ctx_group));
            model.tab = "limitations".to_string()
        },
        Some("timetables") =>{
            model.page = Pages::Timetable(timetables::init(teacher, &mut orders.proxy(Msg::Timetables), ctx_school, ctx_group));
            model.tab = "timetables".to_string()
        },
        _ =>{
            model.page = Pages::Home
        }
    };
    //model.classes = &ctx_school.classes;
    orders.perform_cmd({
        let url = format!("/api/schools/{}/groups/{}/teachers/{}", ctx_school.school.id, ctx_group.group.id, teacher);
        let request = Request::new(url)
            .method(Method::Get);
        async {
            Msg::FetchTeacher(async {
                request
                    .fetch()
                    .await?
                    .check_status()?
                    .json()
                    .await
            }.await)
        }
    });
    let teacher_id = ctx_school.teachers.iter().enumerate().find(|t| t.1.id == teacher);
    match teacher_id{
        Some(t_id)=>{
            if t_id.0 >0{
                model.back_teacher = Some(ctx_school.teachers[t_id.0-1].clone());
            }
            if t_id.0 < ctx_school.teachers.len()-1{
                model.next_teacher = Some(ctx_school.teachers[t_id.0+1].clone());
            }
        }
        None=>{}
    }
    model
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>, _ctx: &mut Context, ctx_school: &mut detail::SchoolContext, ctx_group: &mut GroupContext) {
    match msg {
        Msg::Home => {
            //log!("teacher:", ctx_school);
        }
        Msg::Activities(msg)=>{

            if let Pages::Activity(m)= &mut model.page{
                activities::update(msg, m, &mut orders.proxy(Msg::Activities), _ctx, ctx_school, ctx_group)
            }
        }
        Msg::Limitations(msg)=>{
            if let Pages::Limitation(m)= &mut model.page{
                limitations::update(msg, m, &mut orders.proxy(Msg::Limitations), _ctx, ctx_school, ctx_group)
            }
        }
        Msg::Timetables(msg)=>{
            if let Pages::Timetable(m)= &mut model.page{
                timetables::update(msg, m, &mut orders.proxy(Msg::Timetables), _ctx, ctx_school, ctx_group)
            }
        }
        Msg::FetchTeacher(teacher)=>{
            model.teacher = teacher.unwrap();
            model.form = UpdateTeacherForm{
                first_name: model.teacher.first_name.clone(),
                last_name: model.teacher.last_name.clone(),
                is_active: model.teacher.is_active,
                email: model.teacher.email.clone().unwrap_or("".to_string()),
                tel: model.teacher.tel.clone().unwrap_or("".to_string()),
                password1: "".to_string(),
                password2: "".to_string()
            };
        }
        Msg::ChangeFirstName(f_name) => {
            model.form.first_name = f_name
        }
        Msg::ChangeLastName(l_name) => {
            model.form.last_name = l_name
        }
        Msg::ChangeEmail(email) => {
            model.form.email = email
        }
        Msg::ChangeTel(tel) => {
            model.form.tel = tel
        }
        Msg::ChangePass1(p1) => {
            model.form.password1 = p1
        }
        Msg::ChangePass2(p2) => {
            model.form.password2 = p2
        }
        Msg::SubmitUpdate=>{
            orders.perform_cmd({
                let url = format!("/api/schools/{}/groups/{}/teachers/{}", ctx_school.school.id, ctx_group.group.id, model.teacher.id);
                let request = Request::new(url)
                    .method(Method::Patch)
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
    }
}

pub fn view(model: &Model, ctx_school: &SchoolContext, ctx_group: &GroupContext)->Node<Msg>{
    div![
        nav![
            C!{"breadcrumb is-centered"},
            attrs!{At::AriaLabel=>"breadcrumbs"},
            ul![
                li![
                    a![
                        attrs!{
                            At::Href=> format!("/schools/{}/groups/{}/teachers", &ctx_school.school.id, &ctx_group.group.id)
                        },
                        "<--Öğretmenler"
                    ],
                ],
                match &model.back_teacher{
                    Some(teacher)=>{
                        a![
                            attrs!{
                                At::Href => format!("/schools/{}/groups/{}/teachers/{}/{}", &ctx_school.school.id, &ctx_group.group.id, &teacher.id, &model.tab)
                            },
                            li![
                                &teacher.first_name, " ", &teacher.last_name
                            ]
                        ]

                    },
                    None =>{
                        a![]
                    }
                },
                strong![
                    li![
                        &model.teacher.first_name, " ", &model.teacher.last_name
                    ]
                ],
                match &model.next_teacher{
                    Some(teacher)=>{
                        a![
                            attrs!{
                                At::Href => format!("/schools/{}/groups/{}/teachers/{}/{}", &ctx_school.school.id, &ctx_group.group.id, &teacher.id, &model.tab)
                            },
                            li![
                                &teacher.first_name, " ", &teacher.last_name
                            ]
                        ]
                    },
                    None =>{
                        a![]
                    }
                },
                //li![
                //    &model.next_teacher.first_name, " ", &model.next_teacher.last_name
                //],
            ]
        ],
        breadcrumb(model, ctx_school, ctx_group),
    ]
}

pub fn breadcrumb(model: &Model, ctx_school: &detail::SchoolContext, ctx_group: &GroupContext)->Node<Msg>{
    div![
        div![
            C!{"tabs is-centered"},
            tab(model, ctx_school, ctx_group),
        ],
        context(model, ctx_school, ctx_group)
    ]

}

pub fn context(model: &Model, ctx_school: &detail::SchoolContext, ctx_group: &GroupContext)->Node<Msg>{
    match &model.page{
        Pages::Home=>{
            home(&model)
        }
        Pages::Activity(m)=>{
            activities::view(m, ctx_group, ctx_school).map_msg(Msg::Activities)
        }
        Pages::Limitation(m) => {
            limitations::view(m, ctx_school, ctx_group).map_msg(Msg::Limitations)
        }
        Pages::Timetable(m) => {
            timetables::view(m, ctx_group).map_msg(Msg::Timetables)
        }
    }
}
fn home(model: &Model)->Node<Msg>{
    div![
        C!{"column is-4"},
        div![C!{"field"},
            label![C!{"label"}, "Adı:"],
            p![C!{"control has-icons-left"},
                input![C!{"input"},
                    attrs!{
                        At::Type=>"text",
                        At::Name=>"first_name",
                        At::Id=>"first_name",
                        At::Disabled => model.form.is_active.as_at_value(),
                        At::Value => &model.form.first_name,
                    },
                    input_ev(Ev::Change, Msg::ChangeFirstName)
                ],
            ]
        ],
        div![C!{"field"},
            label![C!{"label"}, "Soyadı:"],
            p![C!{"control has-icons-left"},
                input![C!{"input"},
                    attrs!{
                        At::Type=>"text",
                        At::Name=>"last_name",
                        At::Id=>"last_name",
                        At::Disabled => model.form.is_active.as_at_value(),
                        At::Value => &model.form.last_name,
                    },
                    input_ev(Ev::Change, Msg::ChangeLastName)
                ]
            ]
        ],
        div![C!{"field"},
            label![C!{"label"}, "E-posta adresi:"],
            p![C!{"control has-icons-left"},
                input![C!{"input"},
                    attrs!{
                        At::Type=>"text",
                        At::Name=>"email",
                        At::Id=>"email",
                        At::Disabled=> model.form.is_active.as_at_value(),
                        At::Value => &model.form.email,
                    },
                    input_ev(Ev::Change, Msg::ChangeEmail)
                ]
            ]
        ],
        if !model.teacher.is_active{
        div![
            div![C!{"field"},
                label![C!{"label"}, "Telefon numarası:"],
                div![
                    C!{"field-body"},
                    div![
                        C!{"field is-expanded"},
                        div![
                            C!{"field has-addons"},
                            p![
                                C!{"control"},
                                a![
                                    C!{"button is-static"},
                                    "+90"
                                ]
                            ],
                            p![
                                C!{"control is-expanded"},
                                input![
                                    C!{"input"},
                                    attrs!{
                                        At::Type=>"text",
                                        At::Value => &model.form.tel,
                                    },
                                    input_ev(Ev::Change, Msg::ChangeTel)
                                ]
                            ]
                        ],
                        p![
                            C!{"help"},
                            "Telefon numarasını başında 0(sıfır) olmadan giriniz."
                        ]
                    ]
                ]
            ],
            div![C!{"field"},
                label![C!{"label"}, "Şifre:"],
                p![C!{"control has-icons-left"},
                    input![C!{"input"},
                        attrs!{
                            At::Type=>"password",
                            At::Value => &model.form.password1,
                        },
                        input_ev(Ev::Change, Msg::ChangePass1)
                    ],
                ]
            ],
            div![C!{"field"},
                label![C!{"label"}, "Şifre(tekrar):"],
                p![C!{"control has-icons-left"},
                    input![C!{"input"},
                        attrs!{
                            At::Type=>"password",
                            At::Value => &model.form.password2,
                        },
                        input_ev(Ev::Change, Msg::ChangePass2)
                    ]
                ]
            ],
            div![C!{"field"},
                input![
                    C!{"button is-primary"},
                    "Bilgileri Güncelle",
                    ev(Ev::Click, |event| {
                        event.prevent_default();
                        Msg::SubmitUpdate
                    })
                ]
            ],
            p![
                C!{"help is-danger"},
                "Uyarı: Bir öğretmene, eğer daha önce telefon, eposta ve şifre ataması yapılmamışsa bu sayfadan bu bilgileri ekleyebilirsiniz."
            ],
            p![
                C!{"help is-danger"},
                "Uyarı2: Bir öğretmene ait telefon, eposta veya şifre ataması yapılmışsa, bu öğretmenin bilgilerini ancak öğretmen giriş yapıp değiştirebilir."
            ]
        ]
        }
        else{
            div![]
        }
    ]
}
pub fn tab(model: &Model, ctx_school: &SchoolContext, ctx_group: &GroupContext)->Node<Msg> {
    ul![
        li![
            C!{IF!(active_tab(&model.page, 0)=>"is-active")},
            a![
                "Bilgiler",
                attrs!{
                    At::Href => format!("/schools/{}/groups/{}/teachers/{}", &ctx_school.school.id, &ctx_group.group.id, model.teacher.id)
                }
            ]
        ],
        li![
            C!{IF!(active_tab(&model.page, 1)=>"is-active")},
            a![
                "Aktiviteler",
                attrs!{
                    At::Href=> format!("/schools/{}/groups/{}/teachers/{}/activities", &ctx_school.school.id, &ctx_group.group.id, model.teacher.id)
                }

            ]
        ],
        li![
            C!{IF!(active_tab(&model.page, 2)=>"is-active")},
            a![
                "Kısıtlamalar",
                attrs!{
                    At::Href => format!("/schools/{}/groups/{}/teachers/{}/limitations", &ctx_school.school.id, &ctx_group.group.id, model.teacher.id)
                }
            ]
        ],
        li![
            C!{IF!(active_tab(&model.page, 3)=>"is-active")},
            a![
                "Ders Tablosu",
                attrs!{
                    At::Href => format!("/schools/{}/groups/{}/teachers/{}/timetables", &ctx_school.school.id, &ctx_group.group.id, model.teacher.id)
                }
            ]
        ]
    ]
}

fn active_tab(page: &Pages, i: u8)->bool{
    match page{
        Pages::Home=>{if i == 0{true}else{false}},
        Pages::Activity(_m)=>{if i == 1{true}else{false}},
        Pages::Limitation(_m) => {if i == 2{true}else{false}},
        Pages::Timetable(_m)=>{if i == 3{true}else{false}},
    }
}