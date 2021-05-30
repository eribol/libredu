use seed::{*, prelude::*};
use crate::model::user::UserDetail;
use crate::{Urls, page};
use crate::{Context};


#[derive(Debug)]
pub enum Msg{
    UserUpdateSubmit,
    Fetched(fetch::Result<UserDetail>),
    SchoolPage(page::users::schools::Msg),
    TimetablePage(page::users::timetables::Msg),
    ChangePasswordPage(page::users::change_password::Msg)
}

#[derive(Debug, Default)]
pub struct Menu{
    title: String,
    link: String
}

#[derive(Debug)]
enum Pages{
    NotUser,
    Home,
    //Detail,
    Schools(page::users::schools::Model),
    Timetables(page::users::timetables::Model),
    Password(page::users::change_password::Model),
}

impl Default for Pages{
    fn default()-> Pages{
        Pages::Home
    }
}
#[derive(Debug, Default)]
pub struct Model{
    user: Option<UserDetail>,
    menu: Vec<Menu>,
    page: Pages
}

pub fn init(mut url: Url, orders: &mut impl Orders<Msg>, ctx: &mut Context)->Model{
    let mut model = Model{
        menu: vec![
            Menu{ title: "Kişisel Bilgiler".to_string(), link: "".to_string() },
            Menu{title: "Okullar".to_string(), link: "schools".to_string()},
            Menu{title: "Ders Programlarım".to_string(), link: "timetables".to_string()},
            Menu{title: "Şifre değiştir".to_string(), link: "change_password".to_string()}
        ], ..Default::default()
    };
    match  url.next_path_part(){
        Some("") | None => {
            model.page = Pages::Home
        }
        Some("schools") => {
            model.page = Pages::Schools(page::users::schools::init(url, &mut orders.proxy(Msg::SchoolPage), ctx))
        },
        Some("timetables") => {
            model.page = Pages::Timetables(page::users::timetables::init(url, &mut orders.proxy(Msg::TimetablePage), ctx))
        },
        Some("change_password") => {
            model.page = Pages::Password(page::users::change_password::init(&mut orders.proxy(Msg::ChangePasswordPage), &ctx))
        },
        _ => {
            model.page = Pages::NotUser;
        }
    }
    model
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>, ctx: &mut Context) {
    match msg{
        Msg::UserUpdateSubmit=>{
            /**/
        },
        Msg::Fetched(Ok(u))=>{
            model.user = Some(u);
        },
        Msg::Fetched(Err(_)) => {}
        Msg::SchoolPage(msg) => {
            if let Pages::Schools(m) = &mut model.page {
                page::users::schools::update(msg, m, &mut orders.proxy(Msg::SchoolPage), ctx)
            }
        }
        Msg::ChangePasswordPage(msg) => {
            if let Pages::Password(m) = &mut model.page {
                page::users::change_password::update(msg, m, &mut orders.proxy(Msg::ChangePasswordPage), ctx)
            }
        }
        Msg::TimetablePage(msg) => {
            if let Pages::Timetables(m) = &mut model.page {
                page::users::timetables::update(msg, m, &mut orders.proxy(Msg::TimetablePage), ctx)
            }
        }
    }
}

pub fn view(model: &Model, ctx: &Context)-> Node<Msg>{
    div![
        C!{"columns"},
        left_menu(&model, ctx),
        match &ctx.user{
            Some(_u) => {
                match &model.page{
                    Pages::Home=>{
                        home(ctx, model)
                    }
                    Pages::Timetables(m)=>{
                        page::users::timetables::view(&m, ctx).map_msg(Msg::TimetablePage)
                    }
                    Pages::Schools(m)=>{
                        page::users::schools::view(&m, ctx).map_msg(Msg::SchoolPage)
                    }
                    Pages::Password(m)=>{
                        page::users::change_password::view(&m.form).map_msg(Msg::ChangePasswordPage)
                    }
                    _ =>{
                        not_found(model, ctx)
                    }
                }

            },
            None => {
                div!["Sayfaya erişim izniniz yok"]
            }
        }
    ]
}

fn home(ctx: &Context, _model: &Model)->Node<Msg> {
    match &ctx.user{
        Some(u)=>{
            div![C!{"column is-4"},
                div![C!{"field"},
                    label![C!{"label"}, "Adınız"],
                    p![C!{"control has-icons-left"},
                        input![C!{"input"},
                            attrs!{
                                At::Type=>"text",
                                At::Name=>"first_name",
                                At::Id=>"first_name",
                                At::Disabled => true,
                                At::Value => &u.first_name,
                            }
                        ]
                    ]
                ],
                div![C!{"field"},
                    label![C!{"label"}, "Soyadınız"],
                    p![C!{"control has-icons-left"},
                        input![C!{"input"},
                            attrs!{
                                At::Type=>"text",
                                At::Name=>"last_name",
                                At::Id=>"last_name",
                                At::Disabled => true,
                                At::Value => &u.last_name,
                            }
                        ]
                    ]
                ],
                div![C!{"field"},
                    label![C!{"label"}, "E-posta adresiniz"],
                    p![C!{"control has-icons-left"},
                        input![C!{"input"},
                            attrs!{
                                At::Type=>"text",
                                At::Name=>"email",
                                At::Id=>"email",
                                At::Disabled=> true,
                                At::Value => &u.email,
                            }
                        ]
                    ]
                ]
            ]
        }
        None => {
            div!["Kişisel sayfanız değil"]
        }
    }

}

fn left_menu(model: &Model, ctx: &Context)-> Node<Msg>{
    match &ctx.user{
        Some(u) => {
            div![C!{"column is-2"},
                aside![
                    C!{"menu"},
                    ul![
                        C!{"menu-list"},
                        model.menu.iter().map(|m|
                            li![
                                a![
                                    //C!{"is-active"},
                                    &m.title,
                                    attrs![
                                        At::Href=> Urls::new(&ctx.base_url).user_detail(u.id).add_path_part(&m.link),
                                    ],
                                ]
                            ]
                        )
                    ]
                ]
            ]
        }
        None => {
            div!["bu mu"]
        }
    }

}


fn not_found(_model: &Model, _ctx: &Context)->Node<Msg>{
    div![
        "not found"
    ]
}