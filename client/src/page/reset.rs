use seed::{*, prelude::*};
use crate::Context;
use serde::*;

#[derive(Debug)]
pub enum Msg{
    ResetSubmit,
    EmailChanged(String),
    TelChanged(String),
    KeyChanged(String),
    P1Changed(String),
    P2Changed(String),
    Fetch(fetch::Result<String>),
    SendKey
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ResetPasswordForm{
    pub email: String,
    pub tel: String,
    pub key: String,
    pub password1: String,
    pub password2: String
}

pub fn init()->ResetPasswordForm{
    ResetPasswordForm::default()
}
pub fn update(msg: Msg, model: &mut ResetPasswordForm, orders: &mut impl Orders<Msg>, _ctx: &mut Context){
    match msg{
        Msg::ResetSubmit=>{
            orders.perform_cmd({
                let request = Request::new("/api/reset")
                    .method(Method::Post)
                    .json(&model);
                async { Msg::Fetch(async {
                    request?
                        .fetch()
                        .await?
                        .check_status()?
                        .text()
                        .await
                }.await)}
            });
        }
        Msg::Fetch(s)=>{
            log!(s);
        }
        Msg::SendKey=>{
            orders.perform_cmd({
                let request = Request::new("/api/send_key")
                    .method(Method::Post)
                    .json(&model.email);
                async { Msg::Fetch(async {
                    request?
                        .fetch()
                        .await?
                        .check_status()?
                        .text()
                        .await
                }.await)}
            });
        }
        Msg::EmailChanged(email)=>{
            model.email = email;
        }
        Msg::TelChanged(tel)=>{
            model.tel = tel;
        }
        Msg::KeyChanged(key)=>{
            model.key = key;
        }
        Msg::P1Changed(p1)=>{
            model.password1 = p1;
        }
        Msg::P2Changed(p2)=>{
            model.password2 = p2;
        }
    }
}
pub fn view(model: &ResetPasswordForm)-> Node<Msg> {
    div![C!{"columns"},
        div![C!{"column is-2"}],
        div![C!{"column is-4"},
            div![C!{"field"},
                label![C!{"label"}, "E-posta adresini gir"],
                p![C!{"control has-icons-left"},
                    input![C!{"input"},
                        attrs!{
                            At::Type=>"text",
                            At::Placeholder=>"E-posta",
                            // TODO: `username` vs `email`?
                            At::Name=>"email",
                            At::Id=>"email"
                            At::Value => &model.email,
                        },
                        input_ev(Ev::Input, Msg::EmailChanged),
                    ]
                ]
            ],
            div![C!{"field"},
                label![C!{"label"}, "Telefon Numaranız"],
                p![C!{"control has-icons-left"},
                    input![C!{"input"},
                        attrs!{
                            At::Type=>"text",
                            At::Placeholder=>"Telefon Numarası",
                            At::Value => &model.tel,
                        },
                        input_ev(Ev::Input, Msg::TelChanged)
                    ]
                ]
            ],
            div![C!{"field"},
                label![C!{"label"}, "Anahtarınız(key)"],
                p![C!{"control has-icons-left"},
                    input![C!{"input"},
                        attrs!{
                            At::Type=>"text",
                            At::Placeholder=>"Anahtar",
                            At::Value => &model.key,
                        },
                        input_ev(Ev::Input, Msg::KeyChanged),
                    ],
                    input![C!{"button"},
                        attrs!{
                            At::Type=>"button",
                            At::Value=>"Anahtar yolla"
                        },
                        ev(Ev::Click, |event| {
                            event.prevent_default();
                            Msg::SendKey
                        })
                    ]
                ]
            ],
            div![C!{"field"},
                label![C!{"label"}, "Yeni Şifre"],
                p![C!{"control has-icons-left"},
                    input![C!{"input"},
                        attrs!{
                            At::Type=>"password",
                            At::Placeholder=>"Yeni Şifre",
                            At::Value => &model.password1,
                        },
                        input_ev(Ev::Input, Msg::P1Changed)
                    ]
                ]
            ],
            div![C!{"field"},
                label![C!{"label"}, "Yeni Şifre(tekrar)"],
                p![C!{"control has-icons-left"},
                    input![C!{"input"},
                        attrs!{
                            At::Type=>"password",
                            At::Placeholder=>"Yeni Şifre(tekrar)",
                            At::Value => &model.password2,
                        },
                        input_ev(Ev::Input, Msg::P2Changed)
                    ],
                    span![C!{"icon is-small is-left"}, i![C!{"fa fa-envelop"}]]
                ]
            ],
            div![C!{"field"},
                p![C!{"control has-icons-left"},
                    input![C!{"button"},
                        attrs!{
                            At::Type=>"button",
                            At::Value=>"Giriş Yap",
                            At::Id=>"login_button"
                        },
                        ev(Ev::Click, |event| {
                            event.prevent_default();
                            Msg::ResetSubmit
                        })
                    ]
                ]
            ],
        ]
    ]
}