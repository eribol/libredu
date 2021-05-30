// @TODO: Missing module in the repo - I've crated a dummy one.
use seed::{*, prelude::*};
use serde::*;
use crate::{Context};
use crate::model::user::UserDetail;

// ------ ------
//     Init
// ------ ------
pub struct Model{
    user: User,
    error: Option<String>
}

pub fn init() -> Model {
    Model::default()
}

// ------ ------
//     Model
// ------ ------
#[derive(Debug, Serialize, Deserialize)]
pub struct User{
    first_name: String,
    last_name: String,
    tel: String,
    email: String,
    gender: String,
    password1: String,
    password2: String
}

impl Default for Model{
    fn default()-> Self{
        Model{
            user: User {
                first_name: "".to_string(),
                last_name: "".to_string(),
                tel: "".to_string(),
                email: "".to_string(),
                gender: "E".to_string(),
                password1: "".to_string(),
                password2: "".to_string()
            },
            error: None
        }
    }
}
// ------ ------
//    Update
// ------ ------

#[derive(Debug)]
pub enum Msg{
    EmailChanged(String),
    TelChanged(String),
    Password1Changed(String),
    Password2Changed(String),
    FirstNameChanged(String),
    LastNameChanged(String),
    GenderChanged(String),
    Submit,
    Fetched(fetch::Result<UserDetail>),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>, ctx: &mut Context) {
    match msg {
        Msg::EmailChanged(email) => {
            model.user.email = email;
            if model.user.email.contains('@'){
                //model.form_error.email = "".to_string()
            }
        },
        Msg::Password1Changed(password) => {
            model.user.password1 = password;
            if model.user.password2 == model.user.password1{
                //model.form_error.password = "".to_string()
            }
        },
        Msg::Password2Changed(password) => {
            model.user.password2 = password;
            if model.user.password2 == model.user.password1{
                //model.form_error.password = "".to_string()
            }
        },
        Msg::FirstNameChanged(name) => {
            model.user.first_name = name;
            if !model.user.first_name.is_empty(){
                //model.form_error.first_name = "".to_string()
            }
        },
        Msg::TelChanged(tel) => {
            if let Some(t) = tel.chars().last() {
                if t.is_digit(10) && tel.len() <= 10 {
                    model.user.tel = tel;
                    model.error = None;
                }
            }

        },
        Msg::LastNameChanged(name) => {
            model.user.last_name = name;
            if !model.user.last_name.is_empty(){
                model.error = None
            }
            else {
                model.error = Some("Soyadınızı giriniz".to_string())
            }
            //
        },
        Msg::GenderChanged(gender) => {
            model.user.gender = gender;
        },
        Msg::Submit => {
            model.error = None;
            if !model.user.email.contains('@'){
                model.error = Some("Geçerli bir eposta adresi giriniz".to_string())
            }
            if model.user.password1 != model.user.password2{
                model.error = Some("Şifreler uyumlu değil".to_string())
            }
            if model.user.first_name.is_empty(){
                model.error = Some("Ad alanı boş geçilemez".to_string())
            }
            if model.user.last_name.is_empty(){
                model.error = Some("Soyad alanı boş geçilemez".to_string())
            }
            if model.user.tel.len() != 10{
                if model.user.tel.parse::<f64>().is_err(){
                    model.error = Some("Telefon numarası rakamlardan oluşmalı".to_string())
                }
                else{
                    model.error = Some("Telefon numarası 10 haneli olmalı".to_string())
                }
            }
            else if model.error == None{
                orders.perform_cmd({
                    // `request` has to be outside of the async function because we can't pass reference
                    // to the form (`&model.form`) into the async function (~= `Future`).
                    // (As a workaround we can `clone` the form, but then there will be unnecessary cloning.)
                    let request = Request::new("/api/signin")
                        .method(Method::Post)
                        .json(&model.user);
                    // The first `async` is just the function / `Future` / command
                    // that will be executed by `orders.perform_cmd`.
                    // ---
                    // The second `async` function + its `await` allow us to write async id
                    // that returns `Result` (consumed by `Msg::Fetched`) and contains `await`s
                    // and early returns (`?`).
                    async { Msg::Fetched(async {
                        request?
                            .fetch()
                            .await?
                            .check_status()?
                            .json()
                            .await
                    }.await)}
                });
            }

        },
        Msg::Fetched(user) => {
            match user{
                Ok(u)=> {
                    ctx.user = Some(u);
                    orders.notify(
                        subs::UrlRequested::new(crate::Urls::new(&ctx.base_url).home())
                    );
                }
                Err(_)=>{
                    model.error = Some("Sunucu hatası".to_string())
                }
            }
        }
    }
}

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model)-> Node<Msg>{
    div![C!{"columns"},
        div![C!{"column is-2"}],
        div![C!{"column is-4"},
            form![attrs!{At::Action=>"/signin", At::Method=>"Post"},
                div![C!{"field"},
                    label![C!{"label"}, "Üye Ol"],
                    p![C!{"control has-icons-left"},
                        input![C!{"input"},
                            attrs!{
                                At::Type=>"text",
                                At::Placeholder=>"Adınız",
                                // TODO: `username` vs `email`?
                                At::Name=>"first_name",
                                At::Id=>"first_name"
                                At::Value => &model.user.first_name,
                            },
                            input_ev(Ev::Input, Msg::FirstNameChanged),
                        ],
                        span![C!{"icon is-small is-left"}, i![C!{"fa fa-envelop"}]]
                    ],
                    p![
                        C!{"help is-danger"} //&model.error.as_ref()
                    ]
                ],
                div![C!{"field"},
                    p![C!{"control has-icons-left"},
                        input![C!{"input"},
                            attrs!{
                                At::Type=>"text",
                                At::Placeholder=>"Soyadınız",
                                // TODO: `username` vs `email`?
                                At::Name=>"last_name",
                                At::Id=>"last_name"
                                At::Value => &model.user.last_name,
                            },
                            input_ev(Ev::Input, Msg::LastNameChanged),
                        ],
                        span![C!{"icon is-small is-left"}, i![C!{"fa fa-envelop"}]]
                    ],
                    p![
                        C!{"help is-danger"}, //&model.form_error.last_name
                    ]
                ],
                div![C!{"field"},
                    div![
                        C!{" field is-expanded"},
                        div![
                            C!{"field has-addons"},
                            p![
                                C!{"control"},
                                a![
                                    C!{"button is-static"}, "+90"
                                ]
                            ],
                            p![C!{"control has-icons-left"},
                                input![
                                    C!{"input"},
                                    attrs!{
                                        At::Type=>"tel",
                                        At::Placeholder=>"Telefon numaranız",
                                        // TODO: `username` vs `email`?
                                        At::Name=>"tel",
                                        At::Id=>"tel"
                                        At::Value => &model.user.tel,
                                    },
                                    input_ev(Ev::Input, Msg::TelChanged),
                                ]
                            ]
                        ]
                    ],
                    p![
                        C!{"help is-danger"}, //&model.form_error.tel
                    ]
                ],
                div![C!{"field"},
                    p![C!{"control has-icons-left"},
                        input![C!{"input"},
                            attrs!{
                                At::Type=>"text",
                                At::Placeholder=>"E-posta adresiniz",
                                // TODO: `username` vs `email`?
                                At::Name=>"email",
                                At::Id=>"email"
                                At::Value => &model.user.email,
                            },
                            input_ev(Ev::Input, Msg::EmailChanged),
                        ],
                        span![C!{"icon is-small is-left"}, i![C!{"fa fa-envelop"}]]
                    ],
                    p![
                        C!{"help is-danger"},// &model.form_error.email
                    ]
                ],
                div![C!{"field"},
                    p![C!{"control"},
                        span![
                            C!{"select"},
                            select![
                                attrs!{At::Name=>"gender", At::Id=>"gender"},
                                option![
                                    attrs!{At::Value=>"E"},
                                    "Erkek"
                                ],
                                option![
                                    attrs!{At::Value=>"K"},
                                    "Kadın"
                                ],
                                input_ev(Ev::Change, Msg::GenderChanged),
                            ]
                        ]
                    ]
                ],
                div![C!{"field"},
                    p![C!{"control has-icons-left"},
                        input![C!{"input"},
                            attrs!{
                                At::Type=>"password",
                                At::Placeholder=>"Şifreniz",
                                // TODO: `username` vs `password`?
                                At::Name=>"password1",
                                At::Id=>"password1"
                                At::Value => &model.user.password1,
                            },
                            input_ev(Ev::Input, Msg::Password1Changed),
                        ],
                        span![C!{"icon is-small is-left"}, i![C!{"fa fa-envelop"}]],
                    ]
                ],
                div![C!{"field"},
                    p![C!{"control has-icons-left"},
                        input![C!{"input"},
                            attrs!{
                                At::Type=>"password",
                                At::Placeholder=>"Şifreniz(tekrar)",
                                // TODO: `username` vs `password`?
                                At::Name=>"password2",
                                At::Id=>"password2"
                                At::Value => &model.user.password2,
                            },
                            input_ev(Ev::Input, Msg::Password2Changed),
                        ],
                        span![C!{"icon is-small is-left"}, i![C!{"fa fa-envelop"}]],
                    ],
                    p![C!{"help is-danger"}, //&model.form_error.password
                    ]
                ],
                div![C!{"field"},
                    p![C!{"control has-icons-left"},
                        input![C!{"button is-primary"},
                            attrs!{
                                At::Type=>"button",
                                At::Value=>"Giriş Yap",
                                At::Id=>"signin_button"
                            },
                            ev(Ev::Click, |event| {
                                event.prevent_default();
                                Msg::Submit
                            })
                        ],
                        span![C!{"icon is-small is-left"}, i![C!{"fa fa-envelop"}]]
                    ]
                ],
                p![
                    C!{"help is-danger"},
                    match &model.error{
                        Some(e) => {
                            e
                        }
                        None => {
                            ""
                        }
                    }
                    //&model.error.as_ref().unwrap()
                ]

            ]
        ]
    ]
}

