use seed::{*, prelude::*};
use crate::page::school::detail;
use serde::*;
use crate::page::school::detail::{SchoolContext};
use crate::model::student::Student;
use crate::page::school::library::{books, give, take};
use crate::model::library;

#[derive(Debug)]
pub enum Msg{
    Home,
    ChangeMin(String),
    ChangeMax(String),
    Submit,
    FetchLibrary(fetch::Result<library::Library>),
    FetchStudents(fetch::Result<Vec<Student>>),
    ChangeManager(String),
    ChangeStudent(String),
    Books(books::Msg),
    Give(give::Msg),
    Take(take::Msg)
    //page: Pages
    //Timetables
}
#[derive(Debug, Clone)]
pub enum Pages{
    Home,
    Books(books::Model),
    Give(give::Model),
    Take(take::Model)
}
impl Default for Pages{
    fn default()->Self{
        Self::Home
    }
}

#[derive(Debug, Default, Clone)]
pub struct Model{
    pub books: i32,
    page: Pages,
    library: Option<library::Library>,
    form: library::NewLibrary,
    ctx_library: library::LibraryContext,
    url: Url
}

pub fn init(url: &mut Url, orders: &mut impl Orders<Msg>, ctx_school: &mut SchoolContext)-> Model{
    let mut model = Model::default();
    model.url = url.clone();
    orders.perform_cmd({
        let adres = format!("/api/schools/{}/library", &ctx_school.school.id);
        let request = Request::new(adres)
            .method(Method::Get);
        async { Msg::FetchLibrary(async {
            request
                .fetch()
                .await?
                .check_status()?
                .json()
                .await
        }.await)}
    });
    orders.perform_cmd({
        let adres = format!("/api/schools/{}/students", &ctx_school.school.id);
        let request = Request::new(adres)
            .method(Method::Get);
        async { Msg::FetchStudents(async {
            request
                .fetch()
                .await?
                .check_status()?
                .json()
                .await
        }.await)}
    });
    model
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>, ctx_school: &mut detail::SchoolContext) {
    let lbrry = &mut model.ctx_library;
    match msg {
        Msg::Home => {
            //log!("teacher:", ctx_school);
        }
        Msg::ChangeMin(s) => {
            model.form.barkod_min = s.parse::<i32>().unwrap();
        }
        Msg::ChangeMax(s) => {
            model.form.barkod_max = s.parse::<i32>().unwrap();
        }
        Msg::ChangeManager(s) => {
            log!(&s);
            model.form.manager = s.parse::<i32>().unwrap();
        }
        Msg::ChangeStudent(s) => {
            log!(&s);
            model.form.student = s.parse::<i32>().unwrap();
        }
        Msg::FetchLibrary(library) => {
            match library{
                Ok(l) => {

                    *lbrry.library = l.clone();
                    model.form.student = l.student;
                    model.form.manager = l.manager;
                    model.form.barkod_max = l.barkod_max;
                    model.form.barkod_min = l.barkod_min;
                    model.form.school = l.school;
                    model.url.add_path_part(l.id);
                    match model.url.next_path_part() {
                        Some("") | None =>{
                            model.page = Pages::Home
                        },
                        Some("give") => {
                            model.page = Pages::Give(give::init(&mut orders.proxy(Msg::Give), ctx_school, lbrry));
                        },
                        Some("take") =>{
                            model.page = Pages::Take(take::init(&mut orders.proxy(Msg::Take), ctx_school, lbrry));
                        },
                        Some("books") =>{
                            model.page = Pages::Books(books::init(&mut orders.proxy(Msg::Books), ctx_school, lbrry))
                        },
                        _ =>{
                            model.page = Pages::Home
                        }
                    };
                }
                Err(_) => {}
            }
        }
        Msg::FetchStudents(students) => {
            match students{
                Ok(s) => {
                    model.students = s
                }
                Err(_) => {}
            }
        }
        Msg::Submit => {
            match &model.library{
                Some(l) => {
                    orders.perform_cmd({

                        let adres = format!("/api/schools/{}/library/{}", &ctx_school.school.id, l.id);
                        let request = Request::new(adres)
                            .method(Method::Patch)
                            .json(&model.form);
                        async { Msg::FetchLibrary(async {
                            request?
                                .fetch()
                                .await?
                                .check_status()?
                                .json()
                                .await
                        }.await)}
                    });
                }
                None => {
                    orders.perform_cmd({
                        let adres = format!("/api/schools/{}/library", &ctx_school.school.id);
                        let request = Request::new(adres)
                            .method(Method::Post)
                            .json(&model.form);
                        async { Msg::FetchLibrary(async {
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
        Msg::Books(msg) => {
            if let Pages::Books(m)= &mut model.page{
                books::update(msg, m, &mut orders.proxy(Msg::Books), ctx_school, lbrry)
            }
        }
        Msg::Take(msg) => {
            if let Pages::Take(m)= &mut model.page{
                take::update(msg, m, &mut orders.proxy(Msg::Take), ctx_school, lbrry)
            }
        }
        Msg::Give(msg) => {
            if let Pages::Give(m)= &mut model.page{
                give::update(msg, m, &mut orders.proxy(Msg::Give), ctx_school, lbrry)
            }
        }
    }
}

pub fn view(model: &Model, ctx_school: &detail::SchoolContext)->Node<Msg>{
    div![
        //C!["columns"],
    div![
        C!{"columns"},
        div![
            C!{"column is-4"},
        ],
        div![
            C!{"column is-full"},
            nav![
                C!{"breadcrumb is-centered"},
                attrs!{At::AriaLabel=>"breadcrumbs"},
                ul![
                    li![
                        a![
                            attrs!{
                                At::Href=> format!("/schools/{}/library", &ctx_school.school.id)
                            },
                            "Ayarlar"
                        ]
                    ],

                    li![
                        a![
                            attrs!{
                                At::Href=> format!("/schools/{}/library/books", &ctx_school.school.id)
                            },
                            "Kitaplar"
                        ]
                    ],
                    li![
                        a![
                            attrs!{
                                At::Href=> format!("/schools/{}/library/give", &ctx_school.school.id)
                            },
                            "Ödünç Ver"
                        ]
                    ],
                    li![
                        a![
                            attrs!{
                                At::Href=> format!("/schools/{}/library/take", &ctx_school.school.id)
                            },
                            "Ödünç Al"
                        ]
                    ]
                ]
            ]
        ]
        //breadcrumb(model, ctx_school, ctx_group),
    ],
    div![
            C!["columns"],
    match &model.page{

        Pages::Home => home(model, ctx_school),
        Pages::Books(m) => books::view(m, ctx_school, &model.ctx_library).map_msg(Msg::Books),
        Pages::Give(m) => give::view(m, ctx_school, &model.ctx_library).map_msg(Msg::Give),
        Pages::Take(m) => take::view(m, ctx_school, &model.ctx_library).map_msg(Msg::Take)

    }]
    ]
}

/*pub fn breadcrumb(model: &Model, ctx_school: &detail::SchoolContext, ctx_group: &GroupContext)->Node<Msg>{
    div![
        div![
            C!{"tabs is-centered"},
            //tab(model, ctx_school, ctx_group),
        ],
        context(model, ctx_school, ctx_group)
    ]

}*/

fn home(model: &Model, ctx_school: &detail::SchoolContext)->Node<Msg>{
    div![
        C!{"column is-full"},
        div![C!{"field"},
            label![C!{"label"}, "Yetkili Personel"],
            p![C!{"control has-icons-left"},
                select![
                    C!{"select"},
                    ctx_school.teachers.iter().map(|t|
                        option![
                            attrs!{
                                At::Value=>&t.id,
                                At::Selected => (t.id == model.form.manager).as_at_value()
                            },
                            &t.first_name, " ", &t.last_name
                        ]
                    ),
                        input_ev(Ev::Change, Msg::ChangeManager)
                ]
            ]
        ],
        div![C!{"field"},
            label![C!{"label"}, "Yetkili Öğrenci"],
            p![C!{"control has-icons-left"},
                select![
                    C!{"select"},
                    /*model.students.iter().map(|t|
                        option![
                            attrs!{At::Value=>&t.id, At::Selected => (t.id == model.form.student).as_at_value()},
                            &t.first_name, " ", &t.last_name
                        ]
                    ),*/
                        input_ev(Ev::Change, Msg::ChangeStudent)
                ]
            ]
        ],
        div![C!{"field"},
            label![C!{"label"}, "Minimum Barkod Numarası"],
            p![C!{"control has-icons-left"},
                input![C!{"input"},
                    attrs!{
                        At::Type=>"text",
                    },
                    input_ev(Ev::Change, Msg::ChangeMin)
                ]
            ]
        ],
        div![C!{"field"},
            label![C!{"label"}, "Maximum Barkod Numarası"],
            p![C!{"control has-icons-left"},
                input![C!{"input"},
                    attrs!{
                        At::Type=>"text",
                    },
                    input_ev(Ev::Change, Msg::ChangeMax)
                ]
            ]
        ],
        div![C!{"field"},
            button![
                C!{"button is-primary"},
                match model.library{
                    Some(_) => {
                        "Bilgileri Güncelle"
                    },
                    None => {
                        "Oluştur"
                    }
                },
                ev(Ev::Click, |event| {
                        event.prevent_default();
                        Msg::Submit
                })
            ]
        ]
    ]
}
