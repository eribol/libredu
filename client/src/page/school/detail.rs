use seed::{*, prelude::*};
use crate::{Context};
//use crate::page::school::groups;
use serde::*;
use crate::model::school::{SchoolDetail, UpdateSchoolForm};
use crate::model::user::Teacher;
use crate::model::class::Class;
use crate::model::post::SchoolPost;
use crate::page::school::group::group;
use crate::page::school::{students, subjects, class_rooms};


#[derive(Debug, Default)]
pub struct Model{
    url: Url,
    menu: Vec<SchoolMenu>,
    page: Pages,
    school: SchoolDetail,
    //role: i16,
    groups: Vec<ClassGroups>,
    form: UpdateSchoolForm,
    group_form: GroupForm,
    ctx_school: SchoolContext,
    posts: Vec<SchoolPost>,
    edit: bool
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GroupForm{
    name: String,
    hour: i32
}

#[derive(Debug)]
pub enum Pages{
    Home,
    Detail(SchoolDetail),
    Group(group::Model),
    Students(students::Model),
    Subjects(subjects::Model),
    Classrooms(class_rooms::Model),
    NotFound,
}
impl Default for Pages{
    fn default()-> Self{
        Pages::Home
    }
}

#[derive(Debug)]
pub enum Msg{
    //Timetable(timetable::Msg),
    //Teachers(teachers::Msg),
    Home,
    Group(group::Msg),
    Students(students::Msg),
    Subjects(subjects::Msg),
    Classrooms(class_rooms::Msg),
    FetchDetail(fetch::Result<(i16, SchoolDetail)>),
    FetchClassGroups(fetch::Result<Vec<ClassGroups>>),
    UpdateSubmit,
    UpdateFetch(fetch::Result<SchoolDetail>),
    NameChanged(String),
    FetchPosts(fetch::Result<Vec<SchoolPost>>),
    TelChanged(String),
    LocationChanged(String),
    ChangeGroupName(String),
    ChangeGroupHour(String),
    AddGroup,
    FetchGroup(fetch::Result<ClassGroups>),
    FetchTeachers(fetch::Result<Vec<Teacher>>)
}

pub fn init(url: Url, orders: &mut impl Orders<Msg>, _ctx: &mut Context) ->Model {
    let mut model = Model::default();
    let id = &url.path()[1];
    orders.perform_cmd({
        let adres = format!("/api/schools/{}/detail", &id);
        let request = Request::new(adres)
            .method(Method::Get);
        async { Msg::FetchDetail(async {
            request
                .fetch()
                .await?
                .check_status()?
                .json()
                .await
        }.await)}
    });
    /**/
    model.url = url;
    model
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>, _ctx: &mut Context) {
    let _menu = &mut model.menu;
    let ctx_school = &mut model.ctx_school;
    //ctx_school.school =
    match msg{
        Msg::Home => {
        }
        Msg::FetchTeachers(t)=>{
            match t{
                Ok(teacher)=>{
                    ctx_school.teachers = teacher;
                    //orders.send_msg(Msg::ChangePage);
                }
                Err(_)=>{}
            }
        }
        Msg::FetchPosts(posts) => {
            match posts{
                Ok(p) => {
                    model.posts = p;
                    //model.page = Pages::Home
                }
                Err(_) => {
                    //model.page = Pages::NotFound
                }
            }
        }
        Msg::UpdateSubmit=>{
            if model.edit{
                orders.perform_cmd({
                    let adres = format!("/api/schools/{}/detail", ctx_school.school.id);
                    let request = Request::new(adres)
                        .method(Method::Patch)
                        .json(&model.form);
                    async { Msg::UpdateFetch(async {
                        request?
                            .fetch()
                            .await?
                            .check_status()?
                            .json()
                            .await
                    }.await)}
                });
            }
            else {
                model.edit = true;
            }
        }
        Msg::UpdateFetch(s)=>{
            match s{
                Ok(school) => {
                    LocalStorage::insert("libredu-school", &school).expect("");
                    _ctx.school.retain(|s| s.id != model.school.id);
                    _ctx.school.push(school);
                }
                Err(_) => {
                    model.form = UpdateSchoolForm{
                        name: ctx_school.school.name.clone(),
                        tel: ctx_school.school.tel.clone(),
                        location: ctx_school.school.location.clone(),
                        city: ctx_school.school.city.clone(),
                        town: ctx_school.school.town.clone()
                    };
                }
            }
            model.edit = false;
        }
        Msg::NameChanged(name)=>{
            model.form.name = name;
        }
        Msg::FetchClassGroups(groups) => {
            match groups{
                Ok(g) => {
                    model.groups = g.clone();
                    ctx_school.groups = g;
                    match model.url.next_path_part(){
                        Some("")  | None => {
                            model.page = Pages::Home;
                        },
                        Some("students") => {
                            model.page = Pages::Students(students::init(model.url.clone(),&mut orders.proxy(Msg::Students),ctx_school));
                        },
                        Some("groups") => {
                            match model.url.next_path_part(){
                                _ => {
                                    model.page = Pages::Group(group::init(model.url.clone(), ctx_school, &mut orders.proxy(Msg::Group)));
                                }
                            }

                        },
                        Some("subjects") => {
                            model.page = Pages::Subjects(subjects::init(&mut orders.proxy(Msg::Subjects), ctx_school))
                        }/*
                        Some("teachers") => {
                            model.page = Pages::Teachers(teachers::init(model.url.clone(), &mut orders.proxy(Msg::Teachers), _ctx, ctx_school))
                        }*/
                        Some("detail") => {
                            model.page = Pages::Detail(SchoolDetail::default())
                        },
                        Some("class_rooms") => {
                            model.page = Pages::Classrooms(class_rooms::init(&mut orders.proxy(Msg::Classrooms), ctx_school))
                        }
                        _ => {
                            model.page = Pages::NotFound
                        }
                    };

                }
                Err(_) => {}
            }
        }
        Msg::Group(msg)=>{
            if let Pages::Group(m)= &mut model.page{
                group::update(msg, m, &mut orders.proxy(Msg::Group), _ctx, ctx_school)
            }
        }
        Msg::Students(msg)=>{
            if let Pages::Students(m)= &mut model.page{
                students::update(msg, m, &mut orders.proxy(Msg::Students), ctx_school)
            }
        }
        Msg::Subjects(msg)=>{
            if let Pages::Subjects(m)= &mut model.page{
                subjects::update(msg, m, &mut orders.proxy(Msg::Subjects), ctx_school)
            }
        }
        Msg::Classrooms(msg)=>{
            if let Pages::Classrooms(m)= &mut model.page{
                class_rooms::update(msg, m, &mut orders.proxy(Msg::Classrooms), ctx_school)
            }
        }
        Msg::FetchDetail(Ok(school))=> {
            ctx_school.school = school.1.clone();
            ctx_school.role = school.0;
            model.form = UpdateSchoolForm{
                name: ctx_school.school.name.clone(),
                tel: ctx_school.school.tel.clone(),
                location: ctx_school.school.location.clone(),
                city: ctx_school.school.city.clone(),
                town: ctx_school.school.town.clone()
            };
            orders.perform_cmd({
                let adres = format!("/api/schools/{}/teachers", &ctx_school.school.id);
                let request = Request::new(adres)
                    .method(Method::Get);
                async {
                    Msg::FetchTeachers(async {
                        request
                            .fetch()
                            .await?
                            .check_status()?
                            .json()
                            .await
                    }.await)
                }
            });
            orders.perform_cmd({
                let adres = format!("/api/schools/{}/groups", model.ctx_school.school.id);
                let request = Request::new(adres)
                    .method(Method::Get);
                async {
                    Msg::FetchClassGroups(async {
                        request
                            .fetch()
                            .await?
                            .check_status()?
                            .json()
                            .await
                    }.await)
                }
            });
            orders.perform_cmd({
                let adres = format!("/api/schools/{}", model.ctx_school.school.id);
                let request = Request::new(adres)
                    .method(Method::Get);
                async {
                    Msg::FetchPosts(async {
                        request
                            .fetch()
                            .await?
                            .check_status()?
                            .json()
                            .await
                    }.await)
                }
            });
            model.menu = vec![
                SchoolMenu {
                    link: "".to_string(),
                    name: "Anasayfa".to_string()
                },
                SchoolMenu {
                    link: "detail".to_string(),
                    name: "Okul Bilgiler".to_string()
                },
                SchoolMenu {
                    link: "students".to_string(),
                    name: "Öğrenciler".to_string()
                },
                SchoolMenu {
                    link: "subjects".to_string(),
                    name: "Dersler".to_string()
                },
                SchoolMenu {
                    link: "class_rooms".to_string(),
                    name: "Derslikler".to_string()
                }
            ]
        }
        Msg::FetchDetail(Err(_))=>{
            model.menu = vec![
                ];
            model.page = Pages::NotFound
        }
        Msg::TelChanged(tel) => {
            model.form.tel = Some(tel)
        }
        Msg::LocationChanged(locate) => {
            model.form.location = Some(locate)
        }
        Msg::ChangeGroupName(name) => {
            model.group_form.name = name;
        }
        Msg::ChangeGroupHour(hour) => {
            match hour.parse::<i32>(){
                Ok(h) => {
                    model.group_form.hour = h
                }
                Err(_) => {
                    model.group_form.hour = 0
                }
            }
        }
        Msg::FetchGroup(group) => {
            match group {
                Ok(g) => {
                    ctx_school.groups.push(g);
                }
                Err(_) => {}
            }
        }
        Msg::AddGroup => {
            orders.perform_cmd({
                let adres = format!("/api/schools/{}/groups", ctx_school.school.id);
                let request = Request::new(adres)
                    .method(Method::Post)
                    .json(&model.group_form);
                async { Msg::FetchGroup(async {
                    request?
                        .fetch()
                        .await?
                        .check_status()?
                        .json()
                        .await
                }.await)}
            });
        }
    }
}

pub fn view(model: &Model, ctx: &Context)-> Node<Msg>{
    let ctx_school = &model.ctx_school;
    div![
        C!{"columns"},
        div![
            C!{"column is-2"},
            aside![
                C!{"menu"},
                p![
                    C!{"menu-label"},
                    &model.ctx_school.school.name
                ],
                menus(&model.menu, ctx, ctx_school, model),
                p![
                    C!{"menu-label"},
                    "Gruplar",
                    ul![
                        C!{"menu-list"},
                        model.ctx_school.groups.iter().map(|g|
                            li![
                                a![
                                    attrs!{
                                        At::Href => format!("/schools/{}/groups/{}", &model.ctx_school.school.id, &g.id)
                                    },
                                    &g.name
                                ]
                            ]
                        )
                    ]
                ]
            ],
            label!["Grup Adı:"],
            input![
                C!{"input"},
                attrs!{
                    At::Value => &model.group_form.name
                },
                input_ev(Ev::Change, Msg::ChangeGroupName)
            ],
            label!["Günlük ders saati sayısı"],
            input![
                C!{"input"},
                attrs!{
                    At::Value => &model.group_form.hour
                },
                input_ev(Ev::Change, Msg::ChangeGroupHour)
            ],
            input![
                C!{"button is-secondary"},
                attrs!{
                    At::Type => "button",
                    At::Value => "Grup Ekle"
                },
                ev(Ev::Click, move |_event| {
                    Msg::AddGroup
                })
            ]
        ],
        match &model.page{
            Pages::Detail(_m)=>{
                detail_page(model, ctx)
            }
            Pages::Group(m) => {
                group::view(m, ctx, &ctx_school).map_msg(Msg::Group)
            }
            Pages::NotFound => not_found(),
            Pages::Home => {
                posts(model, ctx)
            },
            Pages::Students(m) => {
                students::view(m, ctx_school).map_msg(Msg::Students)
            },
            Pages::Subjects(m) => {
                subjects::view(m).map_msg(Msg::Subjects)
            },
            Pages::Classrooms(m) => {
                class_rooms::view(m).map_msg(Msg::Classrooms)
            }
        }
    ]
}

pub fn menus(menu: &Vec<SchoolMenu>, ctx: &Context, ctx_school: &SchoolContext, model: &Model) -> Node<Msg>{
    ul![
        C!{"menu-list"},
        menu.iter().map(|m|
            li![
                a![
                    C!{
                        if active_menu(&model.page, m){"is-active"} else {""}
                    },
                    &m.name,
                    attrs![
                        At::Href=> crate::Urls::new(&ctx.base_url).school_pages(ctx_school.school.id, &m.link)
                    ]
                ]
            ]
        )
    ]
}

fn active_menu (page: &Pages, menu: &SchoolMenu) -> bool{
    match page{
        Pages::Detail(_m) => {
            if menu.link == "detail"{
                true
            }
            else {
                false
            }
        }
        Pages::Home => {
            if menu.link == ""{
                true
            }
            else {
                false
            }
        }
        _ => {
            false
        }
    }
}
fn not_found() -> Node<Msg>{
    div!["Kurum bulunamadı"]
}
fn posts(model: &Model, ctx: &Context) -> Node<Msg>{
    div![
    C!{"column is-4"},
    model.posts.iter().map(|p|
        article![
                    C!{"media"},
                    div![
                        C!{"media-content"},
                        div![
                            C!{"content"},
                            p![
                                strong![
                                    match &p.school{
                                        Some(s) => {
                                            div![
                                                a![
                                                    {&s.name},
                                                    attrs!{
                                                        At::Href => format!("/schools/{}", &s.id)
                                                    }
                                                ]
                                            ]
                                        },
                                        None => div!["Admin"]
                                    }
                                ]
                            ],
                            p.body.split("<br>").map(|p2|
                                p![
                                    &p2
                                ]
                            )
                        ],

                        nav![
                            C!{"level"},
                            div![
                                C!{"level-left"},
                                if ctx.user.is_some() && (ctx.user.as_ref().unwrap().is_admin || ctx.user.as_ref().unwrap().id == p.sender) {
                                    a![
                                        C!{"level-item"},
                                        span![
                                            C!{"icon is-small"},
                                            i![
                                                C!{"fas fa-trash"}
                                            ]
                                        ]
                                    ]
                                }
                                else{
                                    a![]
                                }
                            ]
                        ]
                    ]
                ]
    )
    ]
}
fn detail_page(model: &Model, ctx: &Context)-> Node<Msg> {
    if model.edit{
        div![
            C!{"column is-12"},
            div![C!{"field"},
                label![C!{"label"}, "Okul Adı:"],
                p![C!{"control has-icons-left"},
                    input![C!{"input"},
                        attrs!{
                            At::Type=>"text",
                            At::Name=>"name",
                            At::Id=>"name",
                            At::Value=>&model.form.name,
                            At::Disabled => disabled(model,ctx).as_at_value()
                        },
                        input_ev(Ev::Input, Msg::NameChanged),
                    ]
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
                                    At::Value => &model.form.tel.as_ref().unwrap(),
                                },
                                input_ev(Ev::Input, Msg::TelChanged),
                            ]
                        ]
                    ]
                ]
            ],
            div![C!{"field"},
                div![
                    C!{" field is-expanded"},
                    div![
                        C!{"field has-addons"},
                        input![
                            C!{"input"},
                            attrs!{
                                At::Type=>"text",
                                At::Placeholder=>"Adresi",
                                At::Value => &model.form.location.as_ref().unwrap(),
                            },
                            input_ev(Ev::Input, Msg::LocationChanged),
                        ]
                    ]
                ]
            ],
            div![C!{"field"},
                label![C!{"label"}, "İli:"],
                p![C!{"control has-icons-left"},
                    label![C!{"label"},
                        &model.school.city.name,
                    ]
                ]
            ],
            div![C!{"field"},
                label![C!{"label"}, "İlçesi:"],
                p![C!{"control has-icons-left"},
                    label![C!{"label"},
                        &model.school.town.name,
                    ]
                ]
            ],
            div![C!{"field"},
                p![C!{"control has-icons-left"},
                    input![C!{"button is-primary"},
                        attrs!{
                            At::Type=>"button",
                            At::Value=>"Güncelle",
                            At::Id=>"update_button",
                            //At::Disabled => false.as_at_value()
                        },
                    ],
                    ev(Ev::Click, |event| {
                        event.prevent_default();
                        Msg::UpdateSubmit
                        }
                    )
                ]
            ]
        ]
    }
    else {
        div![
            C!{"column is-12"},
            div![C!{"field"},
                label![C!{"label"}, "Okul Adı:"],
                p![C!{"control has-icons-left"},
                    label![
                        C!{"label"}, &model.form.name
                    ]
                ]
            ],
            div![
                C!{"field"},
                label![C!{"label"}, "Telefon:"],
                p![C!{"control"},
                    label![
                        C!{"label"}, &model.form.tel.as_ref().unwrap_or(&"".to_string())
                    ]
                ]
            ],
            div![
                C!{"field"},
                label![C!{"label"}, "Adresi:"],
                p![C!{"control"},
                    label![
                        C!{"label"}, &model.form.location.as_ref().unwrap_or(&"".to_string())
                    ]
                ]
            ],
            div![C!{"field"},
                label![C!{"label"}, "İli:"],
                p![C!{"control has-icons-left"},
                    label![C!{"label"},
                        &model.form.city.name,
                    ]
                ]
            ],
            div![C!{"field"},
                label![C!{"label"}, "İlçesi:"],
                p![C!{"control has-icons-left"},
                    label![C!{"label"},
                        &model.form.town.name,
                    ]
                ]
            ],
            div![C!{"field"},
                p![C!{"control has-icons-left"},
                    input![C!{"button is-primary"},
                        attrs!{
                            At::Type=>"button",
                            At::Value=>"Düzenle",
                            At::Id=>"update_button",
                            //At::Disabled => false.as_at_value()
                        },
                    ],
                    ev(Ev::Click, |event| {
                        event.prevent_default();
                        Msg::UpdateSubmit
                        }
                    )
                ]
            ]
        ]
    }
}

fn disabled(model: &Model, ctx: &Context) -> bool{
    if ctx.user.is_none(){
        return true
    }
    else if ctx.user.as_ref().unwrap().is_admin {
        return false
    }
    else if ctx.school.iter().any(|s| s.id == model.school.id) {
        return false
    }
    else {
        return true
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct City {
    pub pk: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Town {
    pub city: i32,
    pub pk: i32,
    pub name: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SchoolContext{
    pub teachers: Vec<Teacher>,
    pub role: i16,
    //pub classes: Vec<Class>,
    pub groups: Vec<ClassGroups>,
    pub school: SchoolDetail
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SchoolMenu{
    pub link: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ClassGroups{
    pub id: i32,
    pub name: String,
    pub hour: i32
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GroupContext{
    pub group: ClassGroups ,
    pub classes: Vec<Class>
}
