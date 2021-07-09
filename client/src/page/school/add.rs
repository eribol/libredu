use seed::{*, prelude::*};
use crate::model::school::{NewSchool, SchoolDetail, SchoolType};
use crate::{Context};
use crate::page::school::detail::{City, Town, SchoolContext};


#[derive(Default)]
pub struct Model{
    form: NewSchool,
}

#[derive()]
pub enum Msg{
    SubmitSchool,
    FetchSchool(fetch::Result<(i16, SchoolDetail)>),
    NameChanged(String),
}

pub fn init(_url: Url, orders: &mut impl Orders<Msg>) ->Model{
    Model::default()
}


pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>, _ctx: &mut Context) {

    match msg{
        Msg::SubmitSchool=>{
            orders.perform_cmd({
                let request = Request::new("/api/schools/add")
                    .method(Method::Post)
                    .json(&model.form);
                async { Msg::FetchSchool(async {
                    request?
                        .fetch()
                        .await?
                        .check_status()?
                        .json()
                        .await
                }.await)}
            });
        },
        Msg::FetchSchool(Ok(school))=>{
            let s = SchoolContext{
                teachers: None,
                role: school.0,
                groups: None,
                school: school.1,
                students: None,
                subjects: None,
                class_rooms: None,
                menu: vec![]
            };
            _ctx.schools.push(s.clone());
            orders.notify(
                subs::UrlRequested::new(crate::Urls::new(&_ctx.base_url).school_detail(&s.school.id))
            );
        },
        Msg::FetchSchool(Err(e))=>{log!(e)},
        Msg::NameChanged(name)=>{
            model.form.name = name
        }
    }
}

fn add(model: &Model, ctx: &Context)-> Node<Msg>{
    if ctx.schools.is_empty() && ctx.user.is_some(){
        div![
        div![C!{"columns"},
            div![C!{"column is-2"}],
            div![C!{"column is-4"},
                form![attrs!{At::Action=>"/api/school", At::Method=>"Post"},
                    div![C!{"field"},
                        label![C!{"label"}, "Okulunuzun Adı:"],
                        p![C!{"control has-icons-left"},
                            input![C!{"input"},
                                attrs!{
                                    At::Type=>"text",
                                    At::Name=>"name",
                                    At::Id=>"name",
                                    At::Value => &model.form.name,
                                },
                            input_ev(Ev::Input, Msg::NameChanged),
                            ],
                            span![C!{"icon is-small is-left"}, i![C!{"fa fa-envelop"}]]
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
                                    Msg::SubmitSchool
                                })
                            ]
                        ]
                    ]
                ]
            ]
        ]
    ]
    }
    else if ctx.user.is_none(){
        div![
            "Giriş yapınız"
        ]
    }
    else{
        div![
            "Kayıtlı kurumunuz mevcut"
        ]
    }
}
pub fn view(model: &Model, ctx: &Context)-> Node<Msg>{
    add(model, ctx)
}