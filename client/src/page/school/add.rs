use seed::{*, prelude::*};
use crate::model::school::{NewSchool, SchoolDetail, SchoolType};
use crate::{Context};
use crate::page::school::detail::{City, Town};


#[derive(Debug, Default)]
pub struct Model{
    form: NewSchool,
    cities: Vec<City>,
    towns: Vec<Town>,
    types: Vec<SchoolType>
}

#[derive(Debug)]
pub enum Msg{
    FetchCity(fetch::Result<Vec<City>>),
    SubmitSchool,
    FetchSchool(fetch::Result<SchoolDetail>),
    FetchSchoolType(fetch::Result<Vec<SchoolType>>),
    FetchTown(fetch::Result<Vec<Town>>),
    NameChanged(String),
    CityChanged(String),
    TownChanged(String),
    TypeChanged(String),
}

pub fn init(_url: Url, orders: &mut impl Orders<Msg>) ->Model{
    orders.perform_cmd({
        let request = Request::new("/api/city")
            .method(Method::Get);

        async { Msg::FetchCity(async {
            request
                .fetch()
                .await?
                .check_status()?
                .json()
                .await
        }.await)}
    });
    orders.perform_cmd({
        let request = Request::new("/api/school_types")
            .method(Method::Get);

        async { Msg::FetchSchoolType(async {
            request
                .fetch()
                .await?
                .check_status()?
                .json()
                .await
        }.await)}
    });
    Model::default()
}


pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>, _ctx: &mut Context) {

    match msg{
        Msg::FetchCity(Ok(c))=>{
            orders.perform_cmd({
                let request = Request::new(format!("/api/city/{}", c[0].pk))
                    .method(Method::Get);
                //.json(&model.form);
                async { Msg::FetchTown(async {
                    request
                        .fetch()
                        .await?
                        .check_status()?
                        .json()
                        .await
                }.await)}
            });
            model.cities = c;
            model.form.city = model.cities[0].pk
        }
        Msg::FetchCity(Err(_e))=>{
        }
        Msg::FetchSchoolType(types)=>{
            match types{
                Ok(t)=>{
                    model.types = t;

                }
                Err(_)=>{}
            }

        }
        Msg::SubmitSchool=>{
           log!("{:?}", model.form);
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
            _ctx.school.push(school.clone());
            orders.notify(
                subs::UrlRequested::new(crate::Urls::new(&_ctx.base_url).school_detail(school.id))
            );
        },
        Msg::FetchSchool(Err(e))=>{log!(e)},
        Msg::FetchTown(Ok(t))=>{
            model.towns = t;
            model.form.town = model.towns[0].pk;
        }
        Msg::FetchTown(Err(_))=>{

        },
        Msg::CityChanged(city)=>{
           orders.perform_cmd({
                let request = Request::new(format!("/api/city/{}", city))
                    .method(Method::Get);
                    //.json(&model.form);
                async { Msg::FetchTown(async {
                    request
                        .fetch()
                        .await?
                        .check_status()?
                        .json()
                        .await
                }.await)}
            });
            model.form.city = city.parse::<i32>().unwrap();
            log!("{:?}", model.form);
        }
        Msg::TownChanged(town)=>{
            model.form.town = town.parse::<i32>().unwrap();
        }
        Msg::NameChanged(name)=>{
            model.form.name = name
        }
        Msg::TypeChanged(t)=>{
            model.form.school_type = t.parse::<i32>().unwrap();
        }
    }
}

fn add(model: &Model, ctx: &Context)-> Node<Msg>{
    if ctx.school.len() == 0 && !ctx.user.is_none(){
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
                        label![C!{"label"}, "Okul türü"],
                        p![C!{"control"},
                            select![C!{"select"},
                                attrs!{
                                    At::Name=>"type",
                                    At::Id=>"type",
                                    At::Value => &model.form.school_type,
                                },
                                model.types.iter().map(|t|
                                    option![
                                        attrs!{
                                            At::Value=> &t.id
                                        }, &t.name
                                    ]
                                ),
                                input_ev(Ev::Change, Msg::TypeChanged)
                            ]
                        ]
                    ],
                    div![C!{"field"},
                        label![C!{"label"}, "İli:"],
                        p![C!{"control has-icons-left"},
                            select![C!{"select"},
                                attrs!{
                                    At::Name=>"city",
                                    At::Id=>"city",
                                    At::Value => &model.form.city,
                                },
                                model.cities.iter().map(|c|
                                    option![
                                        attrs!{
                                            At::Value=> &c.pk
                                        }, &c.name
                                    ]
                                ),
                                input_ev(Ev::Change, Msg::CityChanged),
                            ]
                        ]
                    ],
                    div![C!{"field"},
                        label![C!{"label"}, "İlçesi:"],
                        p![C!{"control has-icons-left"},
                            select![C!{"select"},
                                attrs!{
                                    At::Name=>"town",
                                    At::Id=>"town",
                                    At::Value => &model.form.town,
                                },
                                model.towns.iter().map(|t|
                                    option![
                                        attrs!{
                                            At::Value=> &t.pk
                                        }, &t.name
                                    ]
                                ),
                            input_ev(Ev::Change, Msg::TownChanged),
                            ]
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