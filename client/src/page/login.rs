use seed::{*, prelude::*};
use serde::{Deserialize, Serialize};

use crate::{Context, STORAGE_KEY};
use crate::model::school::{SchoolDetail};
use crate::model::user::UserDetail;
//use seed::app::subs::url_requested::UrlRequest;

// ------ ------
//     Init
// ------ ------

pub fn init() -> Model {
    Model::default()
}

// ------ ------
//     Model
// ------ ------

#[derive(Default, Serialize)]
pub struct Model {
    form: LoginForm
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LoginForm{
    pub username: String,
    pub password: String
}
// TODO: It should be probably in the `shared` crate.


// ------ ------
//    Update
// ------ ------

#[derive(Debug)]
pub enum Msg{
    EmailChanged(String),
    PasswordChanged(String),
    SubmitForm,
    Fetched(fetch::Result<UserDetail>),
    FetchSchool(fetch::Result<Vec<SchoolDetail>>)
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>, ctx: &mut Context) {
    match msg {
        Msg::EmailChanged(email) => {
            model.form.username = email;
        },
        Msg::PasswordChanged(password) => {
            model.form.password = password;
        },
        Msg::SubmitForm => {
            orders.perform_cmd({
                let request = Request::new("/api/login")
                    .method(Method::Post)
                    .json(&model.form);
                async { Msg::Fetched(async {
                    request?
                        .fetch()
                        .await?
                        .check_status()?
                        .json()
                        .await
                }.await)}
            });
            //orders.skip();
        },
        Msg::Fetched(Ok(auth_user)) => {
            //LocalStorage::remove(STORAGE_KEY).expect("");
            LocalStorage::insert(STORAGE_KEY, &auth_user).expect("");
            ctx.user = LocalStorage::get("libredu-user").unwrap();
            orders.perform_cmd({
                let request = Request::new("/api/schools")
                    .method(Method::Get);
                async { Msg::FetchSchool(async {
                    request
                        .fetch()
                        .await?
                        .check_status()?
                        .json()
                        .await
                }.await)}
            });
            //orders.skip();
        },
        Msg::Fetched(Err(fetch_error)) => {
            log!("fetch AuthUser error:", fetch_error);
            orders.skip();
        },
        Msg::FetchSchool(Ok(schools))=>{
            LocalStorage::insert("libredu-school", &schools).expect("");
            ctx.school = schools;
            orders.notify(
                subs::UrlRequested::new(crate::Urls::new(&ctx.base_url).home())
            );
        },
        Msg::FetchSchool(Err(_e))=>{
            ctx.user = Some(LocalStorage::get("libredu-user").unwrap());
            orders.skip();
        }
    }
}

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model, ctx: &Context)-> Node<Msg>{
    div![C!{"columns"},
        div![C!{"column is-2"}],
        div![C!{"column is-4"},
            form![attrs!{At::Action=>"/login", At::Method=>"Post"},
                div![C!{"field"},
                    label![C!{"label"}, "Giriş Yap"],
                    p![C!{"control has-icons-left"},
                        input![C!{"input"},
                            attrs!{
                                At::Type=>"text",
                                At::Placeholder=>"E-posta veya telefon numarası",
                                // TODO: `username` vs `email`?
                                At::Name=>"email",
                                At::Id=>"email"
                                At::Value => &model.form.username,
                            },
                            input_ev(Ev::Input, Msg::EmailChanged),
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
                                At::Name=>"password",
                                At::Id=>"password"
                                At::Value => &model.form.password,
                            },
                            input_ev(Ev::Input, Msg::PasswordChanged),
                        ]
                    ]
                ],
                div![C!{"field"},
                    p![C!{"control has-icons-left"},
                        input![C!{"button is-primary"},
                            attrs!{
                                At::Type=>"button",
                                At::Value=>"Giriş Yap",
                                At::Id=>"login_button"
                            },
                            ev(Ev::Click, |event| {
                                event.prevent_default();
                                Msg::SubmitForm
                            })
                        ]
                    ]
                ],
                div![C!{"field"},
                    "Üye olmak için",
                    a![attrs!{ At::Href => crate::Urls::new(&ctx.base_url).signin() },
                        " tıklayınız"
                    ]
                ],
                div![C!{"field"},
                    "Şifrenizi mi unuttunuz? ",
                    a![attrs!{ At::Href => crate::Urls::new(&ctx.base_url).reset() },
                        " tıklayınız"
                    ]
                ]
            ]
        ]
    ]
}
