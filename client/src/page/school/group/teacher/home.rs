use seed::{*, prelude::*};
use crate::page::school::detail;
use serde::*;
use crate::page::school::detail::{SchoolContext};
//use crate::page::school::group::teacher::{activities, limitations, timetables};
use crate::model::teacher::Teacher;
use crate::page::school::group::teacher::{activities, limitations, timetables};

#[derive()]
pub enum Msg{
    Home,
    Limitations(limitations::Msg),
    Activities(activities::Msg),
    Timetables(timetables::Msg),
    ChangeFirstName(String),
    ChangeLastName(String),
    ChangeEmail(String),
    ChangeTel(String),
    ChangePass1(String),
    ChangePass2(String),
    SubmitUpdate,
    Loading
    //Timetables
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct UpdateTeacherForm{
    pub first_name: String,
    pub last_name: String,
    pub is_active: bool,
    pub email: String,
    pub tel: String,
    password1: String,
    password2: String,
}

#[derive(Clone)]
pub enum Pages{
    Home,
    Activity(Box<activities::Model>),
    Limitation(limitations::Model),
    Timetable(timetables::Model),
    Loading,
    NotFound
}
impl Default for Pages{
    fn default()->Self{
        Self::Loading
    }
}

#[derive(Default, Clone)]
pub struct Model{
    pub page: Pages,
    form: UpdateTeacherForm,
    pub url: Url,
    pub tab: String
}

pub fn init(mut url: Url, orders: &mut impl Orders<Msg>, school_ctx: &mut SchoolContext)-> Model{
    let mut model = Model::default();
    model.page = Pages::Loading;
    if let Some(teachers) = &school_ctx.teachers{
        if let Some(_) = teachers.iter().find(|t| t.teacher.id == url.path()[5].parse::<i32>().unwrap()) {
            orders.send_msg(Msg::Loading);
        }
        else {
            model.page = Pages::NotFound;
        }
    }
    else {
        model.page = Pages::NotFound;
    }
    model.url = url.clone();
    model
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>, school_ctx: &mut detail::SchoolContext) {
    match msg {
        Msg::Home => {
            //log!("teacher:", ctx_school);
        }
        Msg::Loading => {
            let teacher_ctx = school_ctx.get_teacher(&model.url);
            model.form = UpdateTeacherForm{
                first_name: teacher_ctx.teacher.first_name.clone(),
                last_name: teacher_ctx.teacher.last_name.clone(),
                is_active: teacher_ctx.teacher.is_active,
                email: teacher_ctx.teacher.email.clone().unwrap_or_default(),
                tel: "".to_string(),
                password1: "".to_string(),
                password2: "".to_string()
            };
            match model.url.next_path_part() {
                Some("") | None =>{
                    model.page = Pages::Home
                },
                Some("activities") => {
                    model.page = Pages::Activity(Box::new(activities::init(model.url.clone(), &mut orders.proxy(Msg::Activities), school_ctx)));
                    model.tab = "activities".to_string()
                },
                Some("limitations") =>{
                    model.page = Pages::Limitation(limitations::init(model.url.clone(), &mut orders.proxy(Msg::Limitations)));
                    model.tab = "limitations".to_string()
                },
                Some("timetables") =>{
                    model.page = Pages::Timetable(timetables::init(model.url.clone(), &mut orders.proxy(Msg::Timetables), school_ctx));
                    model.tab = "timetables".to_string()
                },
                _ =>{
                    model.page = Pages::NotFound
                }
            };
        }
        Msg::Activities(msg)=> {
            if let Pages::Activity(m) = &mut model.page {
                activities::update(msg, m, &mut orders.proxy(Msg::Activities), school_ctx)
            }
        }
        Msg::Limitations(msg)=>{
            if let Pages::Limitation(m)= &mut model.page{
                limitations::update(msg, m, &mut orders.proxy(Msg::Limitations), school_ctx)
            }
        }
        Msg::Timetables(msg)=>{
            if let Pages::Timetable(m)= &mut model.page{
                timetables::update(msg, m, &mut orders.proxy(Msg::Timetables), school_ctx)
            }
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
        Msg::SubmitUpdate=> {
            /*
            let school_id = &model.url.path()[1];
            let group_id = &model.url.path()[3];
            let teacher_id = &model.url.path()[5];
            orders.perform_cmd({
                let url = format!("/api/schools/{}/groups/{}/teachers/{}", school_id, group_id, teacher_id);
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

             */
        }
    }
}

pub fn view(model: &Model, school_ctx: &SchoolContext)->Node<Msg>{
    let group_ctx = school_ctx.get_group(&model.url);
    div![
        C!{"columns"},
        match &model.page{
            Pages::Home => home(model, school_ctx),
            Pages::Activity(m) => {
                div![
                    C!{"column is-full"},
                    activities::view(m, school_ctx).map_msg(Msg::Activities)
                ]
            }
            Pages::Limitation(m) => {
                limitations::view(m, school_ctx).map_msg(Msg::Limitations)
            }
            Pages::Timetable(m) => {
                timetables::view(m, school_ctx).map_msg(Msg::Timetables)
            }
            Pages::NotFound => {
                div!["Öğretmen veya Sayfa bulunamadı."]
            }
            Pages::Loading => div!["Yükleniyor..."]
        }
    ]
}

fn home(model: &Model, school_ctx: &SchoolContext)->Node<Msg>{
    let teacher_ctx = school_ctx.get_teacher(&model.url);
    div![
        C!{"column is-half"},
        div![
            C!{"field"},
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
        if !teacher_ctx.teacher.is_active{
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