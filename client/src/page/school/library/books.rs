use seed::{*, prelude::*};
use crate::page::school::{detail};
use crate::model::library;
use serde::*;
use crate::page::school::detail::{SchoolContext};

#[derive(Debug)]
pub enum Msg{
    Home,
    ChangeName(String),
    ChangeWriter(String),
    ChangePieces(String),
    ChangeBarkod(String),
    Submit,
    FetchBooks(fetch::Result<Vec<library::Book>>),
    FetchBook(fetch::Result<library::Book>),
}

#[derive(Debug, Default, Clone)]
pub struct Model{
    //pub books: Vec<Book>,
    pub form: library::NewBook
}

pub fn init(orders: &mut impl Orders<Msg>, ctx_school: &mut detail::SchoolContext, ctx_library: &mut library::LibraryContext)-> Model {
    let model = Model::default();

    orders.perform_cmd({
        let adres = format!("/api/schools/{}/library/{}/books", &ctx_school.school.id, &ctx_library.library.id);
        let request = Request::new(adres)
            .method(Method::Get);
        async {
            Msg::FetchBooks(async {
                request
                    .fetch()
                    .await?
                    .check_status()?
                    .json()
                    .await
            }.await)
        }
    });
    model
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>, ctx_school: &mut detail::SchoolContext, ctx_library: &mut library::LibraryContext) {

    match msg {
        Msg::Home => {
            //log!("teacher:", ctx_school);
        }
        Msg::ChangeName(s) => {
            model.form.name = s;
        }
        Msg::ChangeWriter(s) => {
            model.form.writer = s;
        }
        Msg::ChangePieces(s) => {
            match s.parse::<i32>(){
                Ok(p) => {
                    model.form.piece = p
                }
                Err(_) => {}
            }
        }
        Msg::ChangeBarkod(s) => {
            match s.parse::<i32>(){
                Ok(p) => {
                    model.form.barkod = p
                }
                Err(_) => {}
            }
        }
        Msg::FetchBooks(books) => {
            match books{
                Ok(b) => {
                    ctx_library.books = b.clone();
                }
                Err(_) => {}
            }
        }
        Msg::FetchBook(book) => {
            match book{
                Ok(b) => {
                    ctx_library.books.push(b.clone());
                }
                Err(_) => {}
            }
        }
        Msg::Submit => {
            orders.perform_cmd({
                let adres = format!("/api/schools/{}/library/{}/books", &ctx_school.school.id, ctx_library.library.id);
                let request = Request::new(adres)
                    .method(Method::Post)
                    .json(&model.form);
                async {
                    Msg::FetchBook(async {
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
    }
}

pub fn view(model: &Model, ctx_school: &detail::SchoolContext, ctx_library: &library::LibraryContext)->Node<Msg>{
    div![
        C!{"columns"},
        div![
            C!["column"],

                    div![
                    ctx_library.books.iter().map(|b|
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


        ],
        div![C!["column is-2"]],div![C!["column is-2"]],div![C!["column is-2"]],div![C!["column is-2"]],

            div![
                C!["column is-full"],
                div![C!{"field"},
                    label![C!{"label"}, "Kitap Adı:"],
                    p![C!{"control has-icons-left"},
                        input![
                            C!{"input"},
                            input_ev(Ev::Change, Msg::ChangeName)
                        ]
                    ]
                ],
                div![C!{"field"},
                    label![C!{"label"}, "Yazarı:"],
                    p![
                        C!{"control has-icons-left"},
                        input![
                            C!{"input"},
                            input_ev(Ev::Change, Msg::ChangeWriter)
                        ]
                    ]
                ],
        div![C!{"field"},
            label![C!{"label"}, "Adet:"],
            p![C!{"control has-icons-left"},
                input![
                    C!{"input"},
                    input_ev(Ev::Change, Msg::ChangePieces)
                ]
            ]
        ],
        div![C!{"field"},
            label![C!{"label"}, "Barkod Numarası:"],
            p![C!{"control has-icons-left"},
                input![
                    C!{"input"},
                    input_ev(Ev::Change, Msg::ChangeBarkod)
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
    ]
}
