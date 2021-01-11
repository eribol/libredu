use seed::{*, prelude::*};
use serde::*;
use crate::page::school::detail::{SchoolContext};
use crate::model::class_room::{Classroom, NewClassroom};

#[derive(Debug, Default, Clone)]
pub struct Model{
    form: NewClassroom,
    class_rooms: Vec<Classroom>,
}

pub fn init(orders: &mut impl Orders<Msg>, ctx_school: &SchoolContext)-> Model {
    let mut model = Model::default();
    orders.perform_cmd({
        let request = Request::new(format!("/api/schools/{}/class_rooms", ctx_school.school.id))
            .method(Method::Get);
        async { Msg::FetchClassrooms(async {
            request
                .fetch()
                .await?
                .check_status()?
                .json()
                .await
        }.await)}
    });
    model.form.school = ctx_school.school.id;
    model
}

#[derive(Debug)]
pub enum Msg{
    SubmitClassroom,
    ChangeName(String),
    ChangeRw(String),
    ChangeCl(String),
    ChangeWidth(String),
    FetchClassroom(fetch::Result<Classroom>),
    FetchClassrooms(fetch::Result<Vec<Classroom>>),
    DelClassroom(i32),
    FetchDelClassroom(fetch::Result<i32>)
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>, ctx_school: &mut SchoolContext) {
    match msg {
        Msg::SubmitClassroom => {
            orders.perform_cmd({
                let request = Request::new(format!("/api/schools/{}/class_rooms", ctx_school.school.id))
                    .method(Method::Post)
                    .json(&model.form);
                async { Msg::FetchClassroom(async {
                    request?
                        .fetch()
                        .await?
                        .check_status()?
                        .json()
                        .await
                }.await)}
            });
        }
        Msg::ChangeRw(rw) => {
            match rw.parse::<i16>(){
                Ok(r) => {
                    model.form.rw = r
                }
                Err(_) => {}
            }
        }
        Msg::ChangeCl(cl) => {
            match cl.parse::<i16>(){
                Ok(c) => {
                    model.form.cl = c
                }
                Err(_) => {}
            }
        }
        Msg::ChangeName(name) => {
            model.form.name = name
        }
        Msg::ChangeWidth(wd) => {
            match wd.parse::<i16>(){
                Ok(w) => {
                    model.form.width = w
                }
                Err(_) => {}
            }
        }
        Msg::FetchClassroom(cls) => {
            match cls{
                Ok(c) => {
                    model.class_rooms.insert(0, c)
                }
                Err(_) => {}
            }
        }
        Msg::FetchClassrooms(cls) => {
            match cls {
                Ok(c) => {
                    model.class_rooms = c
                }
                Err(_) => {}
            }
        }
        Msg::DelClassroom(id) => {
            orders.perform_cmd({
                let request = Request::new(format!("/api/schools/{}/class_rooms/{}", ctx_school.school.id, id))
                    .method(Method::Delete);
                async { Msg::FetchDelClassroom(async {
                    request
                        .fetch()
                        .await?
                        .check_status()?
                        .json()
                        .await
                }.await)}
            });
        }
        Msg::FetchDelClassroom(id) => {
            match id{
                Ok(i) => {
                    model.class_rooms.retain(|c| c.id != i)
                }
                Err(_) => {}
            }
        }
    }
}

pub fn view(model: &Model) -> Node<Msg> {
    div![
        div![
            C!{"field"},
            p![
                label![
                    C!{"label"},
                    "Derslik adı:"
                ],
                input![
                    C!{"input"},
                    attrs!{
                        At::Value => &model.form.name
                    },
                    input_ev(Ev::Change, Msg::ChangeName),
                ]
            ],
            p![
                label![
                    C!{"label"},
                    "Derslik sıra kolon sayısı:"
                ],
                input![
                    C!{"input"},
                    attrs!{
                        At::Value => &model.form.rw.to_string()
                    },
                    input_ev(Ev::Change, Msg::ChangeRw),
                ]
            ],
            p![
                label![
                    C!{"label"},
                    "Derslik sıra satır sayısı:"
                ],
                input![
                    C!{"input"},
                    attrs!{
                        At::Value => &model.form.cl.to_string()
                    },
                    input_ev(Ev::Change, Msg::ChangeCl)
                ]
            ],
            p![
                label![
                    C!{"label"},
                    "Sıra öğrenci sayısı:"
                ],
                input![
                    C!{"input"},
                    attrs!{
                        At::Value => &model.form.width.to_string()
                    },
                    input_ev(Ev::Change, Msg::ChangeWidth)
                ]
            ],
            p![
                C!{"control"},
                input![
                    C!{"button is-primary"},
                    attrs!{
                        At::Type=>"button",
                        At::Value=>"Ekle",
                        At::Id=>"login_button"
                    },
                    ev(Ev::Click, |event| {
                        event.prevent_default();
                        Msg::SubmitClassroom
                    })
                ]
            ]
        ],
        div![
            C!{"field"},
            table![
                C!{"table"},
                thead![
                    tr![
                        th![
                            "Derslik Adı"
                        ],
                        th![
                            "Sıra sütun sayısı"
                        ],
                        th![
                            "Sıra satır sayısı"
                        ],
                        th![
                            "Sıra öğrenci sayısı"
                        ],
                        th![
                            "İşlem"
                        ]
                    ]
                ],
                tbody![
                    C!{"table-light"},
                    model.class_rooms.iter().map(|c|
                        tr![
                            td![
                                &c.name
                            ],
                            td![
                                &c.rw.to_string()
                            ],
                            td![
                                &c.cl.to_string()
                            ],
                            td![
                                &c.width.to_string()
                            ],
                            td![
                                button![
                                    C!{"button"},
                                    "Sil",
                                    {
                                        let id = c.id;
                                        ev(Ev::Click, move |_event| {
                                            Msg::DelClassroom(id)
                                        })
                                    }
                                ]
                            ]
                        ]
                    )
                ]
            ]
        ]
    ]

}