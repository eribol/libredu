use crate::model::timetable::{Day};
use serde::*;
use seed::{*, prelude::*};
use crate::{Context};
use crate::model::timetable::{ClassAvailable};
use crate::model::class::{Class};
use crate::page::school::detail::{SchoolContext, GroupContext};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Subject{
    pub name: String,
    pub id: i32
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Activity{
    pub(crate) id: i32,
    pub(crate) subject: Subject,
    pub(crate) teacher: i32,
    pub(crate) class: Class,
    pub(crate) hour: i16,
    pub(crate) split: bool,
    classes: Vec<i32>
}

#[derive(Debug)]
pub enum Msg{
    Home,
    FetchDays(fetch::Result<Vec<Day>>),
    FetchClass(fetch::Result<Class>),
    FetchLimitation(fetch::Result<Vec<ClassAvailable>>),
    ChangeHour((usize,usize)),
    ChangeAllHour(usize),
    ChangeAllDay(usize),
    Submit,
    SubmitSameGrades,
    SubmitAll,
    Print
}

#[derive(Debug, Default, Clone)]
pub struct Model{
    pub class: Class,
    pub days: Vec<Day>,
    pub limitation: Vec<ClassAvailable>,
}

pub fn init(class: i32, orders: &mut impl Orders<Msg>, ctx_school: &mut SchoolContext, ctx_group: &mut GroupContext)-> Model{
    let model = Model::default();
    orders.perform_cmd({
        let url = format!("/api/schools/{}/groups/{}/classes/{}", ctx_school.school.id, &ctx_group.group.id, &class);
        let request = Request::new(url)
            .method(Method::Get);
        async {
            Msg::FetchClass(async {
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

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>, _ctx: &mut Context, ctx_school: &mut SchoolContext, ctx_group: &mut GroupContext) {
    match msg {
        Msg::Home => {
        }
        Msg::ChangeAllHour(index) => {
            let mut all = true;
            for l in &model.limitation{
                if !l.hours[index]{
                    all = false;
                    break;
                }
            }
            if all{
                for d in 0..7{
                    model.limitation[d].hours[index] = false;
                }
            }
            else {
                for d in 0..7{
                    model.limitation[d].hours[index] = true;
                }
            }
        }
        Msg::ChangeAllDay(index) => {
            if model.limitation[index].hours.iter().any(|h| !*h){
                for h in 0..ctx_group.group.hour as usize{
                    model.limitation[index].hours[h as usize] = true;
                }
            }
            else {
                for h in 0..ctx_group.group.hour as usize{
                    model.limitation[index].hours[h as usize] = false;
                }
            }
        }
        Msg::Print=>{
            /*let timetable = vec![(&model.class.kademe, &model.class.sube, &model.timetable)];
            let mut first_hour = 0;
            let last_hour = model.school.hour-1;
            for l in &model.limitation{
                let mut index = 0;
                for h in &l.hours{
                    if *h{
                        first_hour = index;
                    }
                    index += 1;
                }
            }
            class_print(&serde_json::to_string(&timetable).unwrap(), &serde_json::to_string(&model.days).unwrap(), first_hour, last_hour, &serde_json::to_string(&model.school).unwrap())*/
        }
        Msg::FetchClass(class)=>{
            model.class = class.unwrap();
            orders.perform_cmd({
                let url = "/api/days".to_string();
                let request = Request::new(url)
                    .method(Method::Get);
                async {
                    Msg::FetchDays(async {
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
        Msg::Submit=>{
            orders.perform_cmd({
                let url = format!("/api/schools/{}/groups/{}/classes/{}/limitations", ctx_school.school.id, &ctx_group.group.id, &model.class.id);
                let request = Request::new(url)
                    .method(Method::Post)
                    .json(&model.limitation);
                async {
                    Msg::FetchLimitation(async {
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
        Msg::SubmitSameGrades => {
            for c in &ctx_group.classes{
                if c.kademe == model.class.kademe{
                    orders.perform_cmd({
                        let url = format!("/api/schools/{}/groups/{}/classes/{}/limitations", ctx_school.school.id, &ctx_group.group.id, &c.id);
                        let request = Request::new(url)
                            .method(Method::Post)
                            .json(&model.limitation);
                        async {
                            Msg::FetchLimitation(async {
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
        Msg::SubmitAll => {
            for c in &ctx_group.classes{
                orders.perform_cmd({
                    let url = format!("/api/schools/{}/groups/{}/classes/{}/limitations", ctx_school.school.id, &ctx_group.group.id, &c.id);
                    let request = Request::new(url)
                        .method(Method::Post)
                        .json(&model.limitation);
                    async {
                        Msg::FetchLimitation(async {
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
        Msg::FetchDays(days)=>{
            model.days=days.unwrap();
            orders.perform_cmd({
                let url = format!("/api/schools/{}/groups/{}/classes/{}/limitations", ctx_school.school.id, &ctx_group.group.id, model.class.id);
                let request = Request::new(url)
                    .method(Method::Get);
                async {
                    Msg::FetchLimitation(async {
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
        Msg::ChangeHour(ids)=>{
            if model.limitation[ids.0].hours[ids.1]{
                model.limitation[ids.0].hours[ids.1]=false;
            }
            else{
                model.limitation[ids.0].hours[ids.1]=true;
            }
        }
        Msg::FetchLimitation(json)=>{
            match json {
                Ok(mut l) => {
                    l.sort_by(|a, b| a.day.id.cmp(&b.day.id));
                    model.limitation = l;
                    let mut changed = false;
                    for d in model.days.iter() {

                        if !model.limitation.iter().any(|ta| ta.day.id == d.id) {
                            let hours = vec![true; ctx_group.group.hour as usize];
                            model.limitation.push(ClassAvailable { day: d.clone(), hours });
                            changed = true;
                        }
                        if model.limitation[(d.id - 1) as usize].hours.len() != ctx_group.group.hour as usize {
                            let hours = vec![true; ctx_group.group.hour as usize];
                            model.limitation[(d.id - 1) as usize].hours = hours;
                            changed = true;
                        }
                    }
                    if changed{
                        orders.perform_cmd({
                            let url = format!("/api/schools/{}/groups/{}/classes/{}/limitations", ctx_school.school.id, ctx_group.group.id ,model.class.id);
                            let request = Request::new(url)
                                .method(Method::Post)
                                .json(&model.limitation);
                            async {
                                Msg::FetchLimitation(async {
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
                Err(_)=>{
                    if model.limitation.is_empty(){
                        for d in model.days.iter() {
                            if !model.limitation.iter().any(|ta| ta.day.id == d.id) {
                                let hours = vec![false; ctx_group.group.hour as usize];
                                model.limitation.push(ClassAvailable { day: d.clone(), hours })
                            }
                            if model.limitation[(d.id - 1) as usize].hours.len() != ctx_group.group.hour as usize {
                                let hours = vec![false; ctx_group.group.hour as usize];
                                model.limitation[(d.id - 1) as usize].hours = hours;
                            }

                        }
                    }
                }
            }

        }
    }
}

pub fn limitations(model: &Model, ctx_group: &GroupContext)->Node<Msg>{
    div![
        C!{"column"},
        table![
            C!{"table is-bordered is-scrollable"},
            tr![
                td![
                    "Günler/Saatler"
                ],
                (0..ctx_group.group.hour as i32).map(|h|
                    td![
                        &h+1, "",
                        {
                            let hour_index: usize = h as usize;
                            ev(Ev::Click, move |_event|
                                Msg::ChangeAllHour(hour_index)
                            )
                        }
                    ]
                )
            ],
            model.limitation.iter().enumerate().map(|d|
                tr![
                    td![
                        &d.1.day.name.to_uppercase(),
                        {
                            let day_index = d.0;
                            ev(Ev::Click, move |_event|
                                Msg::ChangeAllDay(day_index)
                            )
                        }
                    ],
                    d.1.hours.iter().enumerate().map(|l|
                        td![
                            if *l.1 {
                                style!{
                                    St::Background=>"blue"
                                }
                            }
                            else{
                                style!{
                                    St::Background=>"gray"
                                }
                            },
                            {
                                let hour_index = l.0;
                                let day_index = d.0;
                                ev(Ev::Click, move |_event|
                                    Msg::ChangeHour((day_index, hour_index))
                                )
                            }
                        ]
                    )
                ]
            )
        ],
        div![
            C!{"columns"},
            div![
                C!{"column is-2"},
                input![
                    attrs!{
                        At::Type=>"button", At::Class=>"button is-primary", At::Value=>"Kaydet"
                    },
                    ev(Ev::Click, |event| {
                        event.prevent_default();
                        Msg::Submit
                    })
                ],
            ],
            div![
                C!{"column is-4"},
                input![
                    attrs!{
                        At::Type=>"button", At::Class=>"button is-primary", At::Value=> "Tüm ".to_owned() + &model.class.kademe.to_string() + ". sınıflara aktar"
                    },
                    ev(Ev::Click, |event| {
                        event.prevent_default();
                        Msg::SubmitSameGrades
                    })
                ],
            ],
            div![
                C!{"column is-3"},
                input![
                    attrs!{
                        At::Type=>"button", At::Class=>"button is-primary", At::Value=> "Tüm sınıflara aktar"
                    },
                    ev(Ev::Click, |event| {
                        event.prevent_default();
                        Msg::SubmitAll
                    })
                ],
            ]
        ]
    ]
}

pub fn tabs(model: &Model, ctx_school: &SchoolContext, ctx_group: &GroupContext)->Node<Msg>{
    ul![
        li![
            a![
                "Bilgiler",
                attrs!{
                    At::Href => format!("/schools/{}/groups/{}/classes/{}", &ctx_school.school.id, &ctx_group.group.id, &model.class.id)
                }
            ]
        ],
        li![
            //C!{"is-active"},
            a![
                attrs!{
                    At::Href => format!("/schools/{}/groups/{}/classes/{}/students", &ctx_school.school.id, &ctx_group.group.id, &model.class.id)
                },
                "Öğrenciler"
            ]
        ],
        li![
            a![
                "Aktiviteler",
                attrs!{
                    At::Href => format!("/schools/{}/groups/{}/classes/{}/activities", &ctx_school.school.id, &ctx_group.group.id, &model.class.id)
                }
            ]
        ],
        li![
            C!{"is-active"},
            a![
                attrs!{
                    At::Href => format!("/schools/{}/groups/{}/classes/{}/limitations", &ctx_school.school.id, &ctx_group.group.id, &model.class.id)
                },
                "Kısıtlamalar",
            ]
        ],
        li![
            a![
                attrs!{
                    At::Href => format!("/schools/{}/groups/{}/classes/{}/timetables", &ctx_school.school.id, &ctx_group.group.id, &model.class.id)
                },
                "Ders Tablosu"
            ]
        ]
    ]
}