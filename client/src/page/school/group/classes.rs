use seed::{*, prelude::*};
use crate::model::class::{Class, ClassContext};
use crate::page::school::group::class::home;
use crate::model::class::{NewClass};
use crate::page::school::detail::SchoolContext;

#[derive(Default, Clone)]
pub struct Model{
    pages: Pages,
    form: NewClass,
    url: Url
}

#[derive(Clone)]
pub enum Pages{
    Classes,
    Class(Box<home::Model>),
    Loading
}
impl Default for Pages{
    fn default()->Self{
        Self::Classes
    }
}
impl Pages{
    fn init(mut url: Url, orders:&mut impl Orders<Msg>, school_ctx: &mut SchoolContext) -> Self {
        match url.next_path_part(){
            Some("") | None => {
                Self::Classes
            }
            _ => {
                Self::Class(Box::new(home::init(url.clone(), &mut orders.proxy(Msg::Class), school_ctx)))
            }
        }
    }
}

#[derive()]
pub enum Msg{
    AddClass,
    ChangeKademe(String),
    ChangeSube(String),
    FetchClass(fetch::Result<Class>),
    FetchClasses(fetch::Result<Vec<Class>>),
    Class(home::Msg),
    DeleteClass(i32),
    FetchDel(fetch::Result<i32>),
    FetchUpdateClass(fetch::Result<Class>),
    Loading
}

pub fn init(url: Url, orders: &mut impl Orders<Msg>, school_ctx: &mut SchoolContext)-> Model {
    let mut model = Model{url: url, pages: Pages::Loading, ..Default::default()};
    let group_id = &model.url.path()[3];
    model.form.group_id = group_id.parse::<i32>().unwrap();
    //let group_ctx = school_ctx.get_mut_group(&url);
    orders.send_msg(Msg::Loading);
    model
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>, school_ctx: &mut SchoolContext) {
    let group_ctx = school_ctx.get_mut_group(&model.url);
    let classes_ctx = &mut group_ctx.classes;
    //let mut classes_ctx = classes.classes;
    match msg {
        Msg::DeleteClass(id) => {
            orders.perform_cmd({
                let url = format!("/api/schools/{}/groups/{}/classes/{}", &group_ctx.group.school, group_ctx.group.id, id);
                let request = Request::new(url)
                    .method(Method::Delete);
                async {
                    Msg::FetchDel(async {
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
        Msg::FetchDel(id) =>{
            if let Ok(i) = id {
                if let Some(classes) = classes_ctx{
                    classes.retain(|c| c.class.id != i);
                }
            }
        }
        Msg::Class(msg)=>{
            if let Pages::Class(m) = &mut model.pages {
                home::update(msg, m, &mut orders.proxy(Msg::Class), school_ctx);
            }
        }
        Msg::AddClass=> {
            orders.perform_cmd({
                let url = format!("/api/schools/{}/groups/{}/add_class", group_ctx.group.school, group_ctx.group.id);
                let request = Request::new(url)
                    .method(Method::Post)
                    .json(&model.form);
                async {
                    Msg::FetchClass(async {
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
        Msg::ChangeKademe(kademe)=>{
            model.form.kademe = kademe;
        }
        Msg::ChangeSube(sube)=>{
            model.form.sube = sube
        }
        Msg::FetchClass(class)=>{
            if let Ok(c) = class {
                if let Some(classes) = classes_ctx{
                    let class_ctx = ClassContext{
                        class: c,
                        students: None,
                        activities: None,
                        limitations: None,
                        timetables: None
                    };
                    classes.insert(0, class_ctx);
                }
                else {
                    let class_ctx = ClassContext{
                        class: c,
                        students: None,
                        activities: None,
                        limitations: None,
                        timetables: None
                    };
                    *classes_ctx = Some(vec![class_ctx]);
                }
            }
        }
        Msg::FetchClasses(clsss)=> {
            model.form.group_id = model.url.path()[3].parse().unwrap();
            if let Ok(clss) = clsss {
                if let Some(clss_ctx) = classes_ctx {
                    clss_ctx.clear();
                    for c in clss {
                        let class_ctx = ClassContext {
                            class: c,
                            students: None,
                            activities: None,
                            limitations: None,
                            timetables: None
                        };
                        clss_ctx.push(class_ctx);
                    }
                } else {
                    *classes_ctx = Some(vec![]);
                    if let Some(classes) = classes_ctx {
                        for c in clss {
                            let class_ctx = ClassContext {
                                class: c,
                                students: None,
                                activities: None,
                                limitations: None,
                                timetables: None
                            };
                            classes.push(class_ctx);
                        }
                    }
                    //*classes_ctx = Some(context.clone());
                }
                model.pages = Pages::init(model.url.clone(), orders, school_ctx);
            }
            else {
                *classes_ctx = Some(vec![]);
                model.pages = Pages::Classes
            }

        }
        Msg::FetchUpdateClass(class)=>{
             if class.is_ok() {
                 //model.classes = model.classes.clone().into_iter().filter(|cg| cg.group_id == model.selected_group.id).collect();
             }
        }
        Msg::Loading => {
            if classes_ctx.is_none(){
                orders.perform_cmd({
                    let adres = format!("/api/schools/{}/groups/{}/classes", &school_ctx.school.id, model.url.path()[3]);
                    let request = Request::new(adres)
                        .method(Method::Get);
                    async { Msg::FetchClasses(async {
                        request
                            .fetch()
                            .await?
                            .check_status()?
                            .json()
                            .await
                    }.await)}
                });
            }
            else {
                model.pages = Pages::init(model.url.clone(), orders, school_ctx);
            }
        }
    }
}

pub fn view(model: &Model, school_ctx: &SchoolContext)-> Node<Msg>{
    let groups = school_ctx.get_groups();
    let group_ctx = school_ctx.get_group(&model.url);
    use crate::model::class::CLASS_MENU;
    div![
        C!{"columns"},
        //div![
            //C!{"column is-8"},
        match &model.pages{
            Pages::Classes=>{
                div![
                    C!{"column is-11 is-offset-1"},
                    table![
                        C!{"table table-hover"},
                        thead![
                            tr![
                                th![
                                    attrs!{At::Scope=>"col"},
                                    "Kademe"
                                ],
                                th![
                                    attrs!{At::Scope=>"col"},
                                    "Şube"
                                ],
                                th![
                                    attrs!{At::Scope=>"col"},
                                    "Grup"
                                ]
                            ]
                        ],
                        school_ctx.get_group(&model.url).classes.as_ref().map_or(
                            tbody![
                            ],
                            |classes|
                            tbody![
                                tr![
                                    C!{"table-light"},
                                    td![
                                        input![
                                            attrs!{At::Type=>"text", At::Placeholder=>"Kademe", At::Value=>&model.form.kademe},
                                            input_ev(Ev::Input, Msg::ChangeKademe)
                                        ]
                                    ],
                                    td![
                                        input![
                                            attrs!{At::Type=>"text", At::Placeholder=>"Şube", At::Value=>&model.form.sube},
                                            input_ev(Ev::Input, Msg::ChangeSube)
                                        ]
                                    ],
                                    td![
                                        input![C!{"button is-primary"},
                                            attrs!{
                                                At::Type=>"button",
                                                At::Value=> "Sınıfı Ekle",
                                                At::Id=>"login_button"
                                            },
                                            ev(Ev::Click, |event| {
                                                event.prevent_default();
                                                Msg::AddClass
                                            })
                                        ]
                                    ]
                                ],
                                classes.iter().enumerate().map(|c|
                                    tr![
                                        C!{"table-light"},
                                        td![
                                            a![
                                                &c.1.class.kademe.to_string(),
                                                attrs!{
                                                    At::Href=> format!("/schools/{}/groups/{}/classes/{}",
                                                    &school_ctx.school.id, &school_ctx.get_group(&model.url).group.id, c.1.class.id)
                                                }
                                            ]
                                        ],
                                        td![
                                            a![
                                                &c.1.class.sube,
                                                attrs!{
                                                    At::Href => format!("/schools/{}/groups/{}/classes/{}",
                                                    &school_ctx.school.id, &school_ctx.get_group(&model.url).group.id, c.1.class.id)
                                                }
                                            ]
                                        ],
                                        td![
                                            button![
                                                C!{"button"},
                                                //attrs!{At::Value=>&c.1.id},
                                                "Sil",
                                                {
                                                    let id = c.1.class.id;
                                                    ev(Ev::Click, move |_event| {
                                                        Msg::DeleteClass(id)
                                                    })
                                                }
                                            ]
                                        ]
                                    ]
                                )
                            ]
                        )
                    ]
                ]
            },
            Pages::Class(m) => {
                let class_ctx = group_ctx.get_class(&model.url);
                div![
                    C!{"column"},
                            div![
                        C!{"columns"},
                        div![C!{"column is-12"},
                            nav![
                                C!{"breadcrumb is-centered"},
                                attrs!{At::AriaLabel=>"breadcrumbs"},
                                ul![
                                    li![
                                        a![
                                            attrs!{
                                                At::Href=> format!("/schools/{}/groups/{}/classes", &school_ctx.school.id, &group_ctx.group.id)
                                            },
                                            "<--Sınıflar",
                                        ]
                                    ],
                                    match group_ctx.get_prev_class(&m.url){
                                        Some(class) => {
                                            li![
                                                a![
                                                    attrs!{
                                                        At::Href=> format!("/schools/{}/groups/{}/classes/{}/{}", &school_ctx.school.id, &group_ctx.group.id, class.class.id, &m.tab)
                                                    },
                                                    "Önceki Sınıf"
                                                ]
                                            ]
                                        },
                                        None => {
                                            div![]
                                        }
                                    },
                                    li![
                                        div![" ", &class_ctx.class.kademe.to_string(), "/", &class_ctx.class.sube, " "]
                                    ],
                                    match group_ctx.get_next_class(&m.url){
                                        Some(class) => {
                                            li![
                                                a![
                                                    attrs!{
                                                        At::Href=> format!("/schools/{}/groups/{}/classes/{}/{}", &school_ctx.school.id, &group_ctx.group.id, &class.class.id, &m.tab)
                                                    },
                                                    "Sonraki Sınıf",
                                                ]
                                            ]
                                        },
                                        None => {
                                            div![]
                                        }
                                    },
                                ]
                            ]
                        ]
                    ],
                    div![
                        C!{"columns"},
                        div![
                            C!{"column tabs is-centered"},
                            ul![
                                CLASS_MENU.iter().map(|menu|
                                    li![
                                        if m.tab == menu.link{
                                        C!{"is-active"}} else {C!{""}},
                                        a![
                                            menu.name,
                                            attrs!{
                                                At::Href => format!("/schools/{}/groups/{}/classes/{}/{}", &school_ctx.school.id, &group_ctx.group.id, &class_ctx.class.id, menu.link)
                                            }
                                        ]
                                    ]
                                )
                            ]
                        ]
                    ],
                    div![
                        C!{"columns"},
                        class_detail(m,school_ctx)
                    ]
                ]
            }
            Pages::Loading => {
                div!["yükleniyor..."]
            }
        }
        //]
    ]
}

fn class_detail(c_model: &home::Model, school_ctx: &SchoolContext) ->Node<Msg>{
    let group_ctx = school_ctx.get_group(&c_model.url);
    let class_ctx = school_ctx.get_group(&c_model.url).get_class(&c_model.url);
    div![
        C!{"column is-12"},
        div![
            home::view(c_model, school_ctx).map_msg(Msg::Class),
        ]
    ]
}