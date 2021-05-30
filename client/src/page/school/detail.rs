use seed::{*, prelude::*};
use crate::{Context};
//use crate::page::school::groups;
use serde::*;
use crate::model::school::{SchoolDetail, UpdateSchoolForm};
use crate::model::user::Teacher;
use crate::model::class::Class;
use crate::model::post::SchoolPost;
use crate::page::school::group::home;
use crate::page::school::{students, subjects, class_rooms};
use crate::model::student::Student;
use crate::model::class_room::Classroom;
use crate::model::subject::Subject;


#[derive(Debug, Default)]
pub struct Model{
    url: Url,
    page: Pages,
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
    Group(Box<home::Model>),
    Students(students::Model),
    Subjects(subjects::Model),
    Classrooms(class_rooms::Model),
    //Library(library::home::Model),
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
    Group(home::Msg),
    Students(students::Msg),
    Subjects(subjects::Msg),
    Classrooms(class_rooms::Msg),
    //Library(library::home::Msg),
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
    //log!(&_ctx.schools);
    model.ctx_school = _ctx.schools.clone().into_iter().find(|s| s.school.id == id.parse::<i32>().unwrap()).unwrap();
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
    let ctx_school = &mut model.ctx_school;
    //ctx_school.school =
    match msg{
        Msg::Home => {
        }
        Msg::FetchTeachers(teacher)=>{
            if let Ok(t) = teacher {
                ctx_school.teachers = t;
            }
        }
        Msg::FetchPosts(posts) => {
            if let Ok(p) = posts {
                model.posts = p;
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
                    //_ctx.schools.retain(|s| s.school.id != model.school.id);

                    orders.perform_cmd({
                        let adres = format!("/api/schools/{}/detail", &school.id);
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
                    //SessionStorage::insert("schools", &school).expect("");
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
            if let Ok(g) = groups {
                model.groups = g;
                //ctx_school.groups = g;
                match model.url.next_path_part() {
                    Some("") | None => {
                        model.page = Pages::Home;
                    },
                    Some("students") => {
                        model.page = Pages::Students(students::init(model.url.clone(), &mut orders.proxy(Msg::Students), ctx_school));
                    },
                    Some("groups") => {
                        let _url = model.url.next_path_part();
                        model.page = Pages::Group(Box::new(home::init(model.url.clone(), ctx_school, &mut orders.proxy(Msg::Group))));
                    },
                    Some("subjects") => {
                        model.page = Pages::Subjects(subjects::init(&mut orders.proxy(Msg::Subjects), ctx_school))
                    }
                    Some("detail") => {
                        model.page = Pages::Detail(SchoolDetail::default())
                    },
                    Some("class_rooms") => {
                        model.page = Pages::Classrooms(class_rooms::init(&mut orders.proxy(Msg::Classrooms), ctx_school))
                    }
                    //Some("library") => {
                    //  model.page = Pages::Library(library::home::init(&mut model.url, &mut orders.proxy(Msg::Library), ctx_school))
                    //}
                    _ => {
                        model.page = Pages::NotFound
                    }
                };
            }
        }
        Msg::Group(msg)=>{
            if let Pages::Group(m)= &mut model.page{
                home::update(msg, m, &mut orders.proxy(Msg::Group), _ctx, ctx_school)
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
        /*Msg::Library(msg)=>{
            if let Pages::Library(m)= &mut model.page{
                library::home::update(msg, m, &mut orders.proxy(Msg::Library), ctx_school)
            }
        }*/
        Msg::FetchDetail(Ok(school))=> {
            //_ctx.schools.push(school);
            ctx_school.school = school.1.clone();
            ctx_school.role = school.0;
            model.form = UpdateSchoolForm{
                name: ctx_school.school.name.clone(),
                tel: ctx_school.school.tel.clone(),
                location: ctx_school.school.location.clone(),
                city: ctx_school.school.city.clone(),
                town: ctx_school.school.town.clone()
            };
            if _ctx.user.is_some() {
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
            }
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
        }
        Msg::FetchDetail(Err(_))=>{
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
            if group.is_ok() {
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
                menus(ctx, ctx_school, model),
                p![
                    C!{"menu-label"},
                    "Gruplar",
                    ul![
                        C!{"menu-list"},
                        model.groups.iter().map(|g|
                            li![
                                a![
                                    attrs!{
                                        At::Href => crate::Urls::new(&ctx.base_url).group_detail(ctx_school.school.id, g.id)
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
                home::view(m, ctx, &ctx_school).map_msg(Msg::Group)
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
            },
            //Pages::Library(m) => {
            //    library::home::view(m, ctx_school).map_msg(Msg::Library)
            //}
        }
    ]
}

pub fn menus(ctx: &Context, ctx_school: &SchoolContext, model: &Model) -> Node<Msg>{
    use crate::model::school::LIST;
    ul![
        C!{"menu-list"},
        LIST.iter().map(|m|
            li![
                a![
                    C!{
                        if active_menu(&model.page, m){"is-active"} else {""}
                    },
                    &m.name,
                    attrs![
                        At::Href=> crate::Urls::new(&ctx.base_url).school_pages(ctx_school.school.id, m.link)
                    ]
                ]
            ]
        )
    ]
}

fn active_menu (page: &Pages, menu: &crate::model::school::SchoolMenu) -> bool{
    match page{
        Pages::Detail(_m) => {
            menu.link == "detail"
        }
        Pages::Students(_) => {
            menu.link == "students"
        }
        Pages::Classrooms(_) => {
            menu.link == "class_rooms"
        }
        Pages::Subjects(_) => {
            menu.link == "subjects"
        }
        Pages::Home => {
            menu.link.is_empty()
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
                        &model.ctx_school.school.city.name,
                    ]
                ]
            ],
            div![C!{"field"},
                label![C!{"label"}, "İlçesi:"],
                p![C!{"control has-icons-left"},
                    label![C!{"label"},
                        &model.ctx_school.school.town.name,
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

fn disabled(_: &Model, ctx: &Context) -> bool{
    if ctx.user.is_none(){
        return true;
    }
    else if ctx.user.as_ref().unwrap().is_admin {
        return false;
    }
    !ctx.schools.iter().any(|s| s.school.id == 1)
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

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SchoolContext{
    pub teachers: Vec<Teacher>,
    pub role: i16,
    //pub classes: Vec<Class>,
    pub groups: Option<Vec<ClassGroups>>,
    pub school: SchoolDetail,
    pub students: Option<Vec<Student>>,
    pub subjects: Option<Vec<Subject>>,
    pub class_rooms: Option<Vec<Classroom>>,
    pub menu: Vec<SchoolMenu>
}

#[derive(Debug, Serialize, Deserialize, Default,Clone)]
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
