use seed::{*, prelude::*};
use crate::{Context};
use crate::model::{teacher};
use crate::model::timetable;
use crate::model::activity;
use crate::page::school::detail;
use crate::model::teacher::TeacherTimetable;
use serde::*;
use crate::page::school::detail::{SchoolContext, GroupContext};

#[derive(Debug)]
pub enum Msg{
    Home,
    FetchDays(fetch::Result<Vec<timetable::Day>>),
    FetchTeacher(fetch::Result<Teacher>),
    FetchLimitation(fetch::Result<Vec<teacher::TeacherAvailable>>),
    ChangeHour((usize,usize)),
    SubmitLimitation,
    SubmitAllLimitation,
    ChangeAllHour(usize),
    ChangeAllDay(usize),
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Teacher{
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub is_active: bool,
    pub email: Option<String>,
    pub tel: Option<String>
}


#[derive(Debug, Default, Clone)]
pub struct Model{
    pub teacher: Teacher,
    pub act_form: activity::NewActivity,
    pub activities: Vec<activity::TeacherActivity>,
    subjects: Vec<activity::Subject>,
    pub timetable: Vec<TeacherTimetable>,
    days: Vec<timetable::Day>,
    pub limitation: Vec<teacher::TeacherAvailable>,
}

pub fn init(teacher: i32, orders: &mut impl Orders<Msg>, _ctx: &mut Context, ctx_school: &mut SchoolContext, ctx_group: &GroupContext)-> Model{
    let model = Model::default();
    //model.classes = &ctx_school.classes;
    orders.perform_cmd({
        let url = format!("/api/schools/{}/groups/{}/teachers/{}", ctx_school.school.id, ctx_group.group.id, teacher);
        let request = Request::new(url)
            .method(Method::Get);
        async {
            Msg::FetchTeacher(async {
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

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>, _ctx: &mut Context, ctx_school: &mut detail::SchoolContext, ctx_group: &mut GroupContext) {

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
                for h in 0..ctx_group.group.hour{
                    model.limitation[index].hours[h as usize] = true;
                }
            }
            else {
                for h in 0..ctx_group.group.hour{
                    model.limitation[index].hours[h as usize] = false;
                }
            }
        }
        Msg::FetchDays(days)=>{
            model.days=days.unwrap();
            orders.perform_cmd({
                let url = format!("/api/schools/{}/groups/{}/teachers/{}/limitations", ctx_school.school.id, ctx_group.group.id, model.teacher.id);
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
        Msg::FetchLimitation(json)=>{
            match json {
                Ok(mut l) => {
                    l.sort_by(|a, b| a.day.id.cmp(&b.day.id));
                    model.limitation = l;
                    let mut changed = false;
                    for d in model.days.iter() {
                        if !model.limitation.iter().any(|ta| ta.day.id == d.id) {
                            let hours = vec![true; ctx_group.group.hour as usize];
                            model.limitation.push(teacher::TeacherAvailable { day: d.clone(), hours: hours, group_id: None });
                            changed = true;
                        } else {
                            if model.limitation[(d.id - 1) as usize].hours.len() != ctx_group.group.hour as usize {
                                let hours = vec![true; ctx_group.group.hour as usize];
                                model.limitation[(d.id - 1) as usize].hours = hours;
                                changed = true;
                            }
                        }
                    }
                    if changed{
                        orders.perform_cmd({
                            let url = format!("/api/schools/{}/groups/{}/teachers/{}/limitations", ctx_school.school.id, ctx_group.group.id, model.teacher.id);
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
                    if model.limitation.len()==0 {
                        for d in model.days.iter() {
                            if !model.limitation.iter().any(|ta| ta.day.id == d.id) {
                                let hours = vec![true; ctx_group.group.hour as usize];
                                model.limitation.push(teacher::TeacherAvailable { day: d.clone(), hours: hours, group_id: None })
                            } else {
                                if model.limitation[(d.id - 1) as usize].hours.len() != ctx_group.group.hour as usize {
                                    let hours = vec![true; ctx_group.group.hour as usize];
                                    model.limitation[(d.id - 1) as usize].hours = hours;
                                }
                            }
                        }
                        orders.perform_cmd({
                            let url = format!("/api/schools/{}/teachers/{}/limitations/{}", ctx_school.school.id, model.teacher.id, ctx_group.group.id);
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
        }
        Msg::SubmitLimitation=>{
            orders.perform_cmd({
                let url = format!("/api/schools/{}/groups/{}/teachers/{}/limitations", ctx_school.school.id, ctx_group.group.id, model.teacher.id);
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
        Msg::SubmitAllLimitation => {
            for t in &ctx_school.teachers{
                orders.perform_cmd({
                    let url = format!("/api/schools/{}/groups/{}/teachers/{}/limitations", ctx_school.school.id, ctx_group.group.id, model.teacher.id);
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
        Msg::ChangeHour(ids)=>{
            if model.limitation[ids.0].hours[ids.1]{
                model.limitation[ids.0].hours[ids.1]=false;
            }
            else{
                model.limitation[ids.0].hours[ids.1]=true;
            }
        }
        Msg::FetchTeacher(teacher)=>{
            model.teacher = teacher.unwrap();
            orders.perform_cmd({
                let url = format!("/api/days");
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
    }
}

pub fn view(model: &Model, _ctx_school: &detail::SchoolContext, ctx_group: &GroupContext)->Node<Msg>{
    div![
        C!{"column is-12"},
        table![
            C!{"table is-bordered"},
            tr![
                td![
                    "Günler/Saatler"
                ],
                (0..ctx_group.group.hour).map(|h|
                    td![
                        &h+1,
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
                        &d.1.day.name,
                        {
                            let day_index = d.0;
                            ev(Ev::Click, move |_event|
                                Msg::ChangeAllDay(day_index)
                            )
                        }
                    ],
                    d.1.hours.iter().enumerate().map(|h|
                        td![

                            if *h.1 {
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
                                let hour_index = h.0;
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
                        Msg::SubmitLimitation
                    })
                ]
            ],
            div![
                C!{"column is-2"},
                input![
                    attrs!{
                        At::Type=>"button", At::Class=>"button is-primary", At::Value=>"Tüm Öğretmenlere aktar"
                    },
                    ev(Ev::Click, |event| {
                        event.prevent_default();
                        Msg::SubmitAllLimitation
                    })
                ]
            ]
        ]
    ]
}