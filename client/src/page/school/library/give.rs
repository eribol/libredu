use seed::{*, prelude::*};
use crate::page::school::detail;
use crate::model::library;
use serde::*;
use crate::page::school::detail::{SchoolContext};
use chrono::NaiveDate;

#[derive(Debug)]
pub enum Msg{
    ChangeStudent(String),
    ChangeBook(String),
    Submit,
    FetchBooks(fetch::Result<Vec<Book>>),
    FetchTakens(fetch::Result<Vec<Take>>),
    FetchTaken(fetch::Result<Book>),
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Book {
    pub id: i32,
    pub library: i32,
    pub name: String,
    pub writer: String,
    pub piece: i32,
    pub barkod: i32
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Take {
    pub id: i32,
    pub library: i32,
    pub student: i32,
    pub book: i32,
    pub date: NaiveDate
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct NewTake {
    //pub school: i32,
    pub student: i32,
    pub book: i32,
    pub library: i32
}

#[derive(Debug, Default, Clone)]
pub struct Model{
    pub books: Vec<Book>,
    pub takens: Vec<Take>,
    pub form: NewTake
}

pub fn init(orders: &mut impl Orders<Msg>, ctx_school: &mut detail::SchoolContext, ctx_library: &mut library::LibraryContext)-> Model{
    let model = Model::default();
    /*match ctx_library{
        Some(l) => {
            orders.perform_cmd({
                let adres = format!("/api/schools/{}/library/{}/books", &ctx_school.school.id, &l.id);
                let request = Request::new(adres)
                    .method(Method::Get);
                async { Msg::FetchBooks(async {
                    request
                        .fetch()
                        .await?
                        .check_status()?
                        .json()
                        .await
                }.await)}
            });
        }
        None => {}
    }*/

    model
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>, ctx_school: &mut detail::SchoolContext, ctx_library: &mut library::LibraryContext) {
    match msg {
        Msg::ChangeStudent(s) => {
            //model.form.student = s.parse::<i32>().unwrap();
        }
        Msg::ChangeBook(s) => {
            //model.form.book = s;
        }
        Msg::FetchBooks(books) => {
            match books{
                Ok(b) => {
                    model.books = b.clone();
                }
                Err(_) => {}
            }
        }
        Msg::Submit => {
            /*match ctx_library{
                Some(l) => {
                    orders.perform_cmd({
                        let adres = format!("/api/schools/{}/library/{}/books", &ctx_school.school.id, l.id);
                        let request = Request::new(adres)
                            .method(Method::Post)
                            .json(&model.form);
                        async {
                            Msg::FetchTaken(async {
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
                None => {}
            }*/
        }
        Msg::FetchTakens(t) => {}
        Msg::FetchTaken(t) => {}
    }
}

pub fn view(model: &Model, ctx_school: &detail::SchoolContext, library: &Option<library::home::Library>)->Node<Msg>{
    div![
        C!{"columns"},
        div![
            C!["column"],
            match library{
                Some(_) =>{
                    div![
                    model.books.iter().map(|b|
                        div![
                            C!["columns"],
                            div![
                                C!["column"],
                                &b.name
                            ],
                            div![
                                C!["column"],
                                &b.writer
                            ],
                            div![
                                C!["column"],
                                &b.piece
                            ],
                            hr![]
                        ]
                    )]
                }
                None => {
                    div!["Kütüphane oluşturunuz!"]
                }
            }
        ],
        div![C!["column is-2"]],div![C!["column is-2"]],div![C!["column is-2"]],div![C!["column is-2"]],
        match library{
            Some(l) => {
            div![
                C!["column is-full"],
                div![C!{"field"},
                    label![C!{"label"}, "Öğrenci Numarası:"],
                    p![C!{"control has-icons-left"},
                        input![
                            C!{"input"},
                            input_ev(Ev::Change, Msg::ChangeStudent)
                        ]
                    ]
                ],
                div![C!{"field"},
                    label![C!{"label"}, "Kitap Barkod Numarası:"],
                    p![
                        C!{"control has-icons-left"},
                        input![
                            C!{"input"},
                            input_ev(Ev::Change, Msg::ChangeBook)
                        ]
                    ]
                ],
        div![C!{"field"},
            button![
                C!{"button is-primary"},
                "Oluştur",
                ev(Ev::Click, |event| {
                        event.prevent_default();
                        Msg::Submit
                })
            ]
        ]
        ]
            },
            None => {
                div![
                    "Kütüphane ekleyin",
                ]
            }
        }

    ]
}
