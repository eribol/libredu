use crate::model::{user, activity};
use seed::{*, prelude::*};
use crate::{Context};
use crate::model::teacher;
use crate::model::timetable;
use crate::page::school::detail;
use crate::model::teacher::TeacherTimetable;
use crate::page::school::detail::{GroupContext};

#[derive(Debug)]
pub enum Msg{
    FetchDays(fetch::Result<Vec<timetable::Day>>),
    FetchTeacher(fetch::Result<user::TeacherAct>),
    FetchTimetable(fetch::Result<Vec<teacher::TeacherTimetable>>),
    FetchActivities(fetch::Result<Vec<activity::TeacherActivity>>),
    FetchLimitation(fetch::Result<Vec<teacher::TeacherAvailable>>),
    Print
}


#[derive(Debug, Default, Clone)]
pub struct Model{
    pub teacher: user::TeacherAct,
    pub timetable: Vec<TeacherTimetable>,
    activities: Vec<activity::TeacherActivity>,
    limitation: Vec<teacher::TeacherAvailable>,
    days: Vec<timetable::Day>,
}

pub fn init(teacher: i32, orders: &mut impl Orders<Msg>, ctx_school: &mut detail::SchoolContext, ctx_group: &GroupContext)-> Model{
    let model = Model::default();
    orders.perform_cmd({
        let url = format!("/api/schools/{}/groups/{}/teachers/{}", &ctx_school.school.id, ctx_group.group.id, teacher);
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
    model
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>, _ctx: &mut Context, ctx_school: &mut detail::SchoolContext, ctx_group: &mut detail::GroupContext) {

    match msg {
        Msg::FetchTimetable(t)=>{
            match t{
                Ok(tt) => {
                    model.timetable = tt;
                    orders.perform_cmd({
                        let url = format!("/api/schools/{}/groups/{}/teachers/{}/activities", ctx_school.school.id, ctx_group.group.id, model.teacher.id);
                        let request = Request::new(url)
                            .method(Method::Get);
                        async {
                            Msg::FetchActivities(async {
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

                Err(_) => {}
            }
        }
        Msg::FetchActivities(acts)=>{
            match acts{
                Ok(a) => {
                    model.activities = a.clone().into_iter().filter(|a|
                        ctx_group.classes.iter().any(|c| a.classes.iter().any(|a2| a2.id == c.id))
                    ).collect()
                }
                Err(_) => {
                }
            }
        }
        Msg::FetchLimitation(json)=>{
            match json {
                Ok(mut l) => {
                    log!(l.len());
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
            }
        }
        Msg::FetchTeacher(teacher)=>{
            model.teacher = teacher.unwrap();
            orders.perform_cmd({
                let url = format!("/api/schools/{}/groups/{}/teachers/{}/timetables", ctx_school.school.id, ctx_group.group.id, model.teacher.id);
                let request = Request::new(url)
                    .method(Method::Get);
                async {
                    Msg::FetchTimetable(async {
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
        Msg::FetchDays(days)=>{
            model.days=days.unwrap();
        }
        Msg::Print=>{
            //use crate::print_timetable::print_timetable;
            //let _printable = print_timetable();
            //let table = vec![model.timetable.clone()];
            //let timetables = vec![(&model.teacher.first_name, &model.teacher.last_name, &model.timetable)];
            //createPDF(&serde_json::to_string(&timetables).unwrap(), &serde_json::to_string(&model.days).unwrap(), 0, (model.selected_group.hour-1) as i16, &serde_json::to_string(&_ctx_school.school).unwrap());
        }
    }
}

pub fn view(model: &Model, ctx_group: &GroupContext)->Node<Msg>{
    div![
        Script![
            attrs!{
                At::Src=>"/static/js/print_teachers.js",
                At::Type=>"module"
            }
        ],
        timetable(model, ctx_group)
    ]

}


fn timetable(model: &Model, ctx_group: &GroupContext)->Node<Msg>{
    div![
        C!{"column is-12"},

        table![
            C!{"table is-bordered"},
            tr![
                td![
                    "Günler/Saatler"
                ],
                (0..ctx_group.group.hour as i32).map(|h|
                    td![
                        &h+1
                    ]
                )
            ],
            model.limitation.iter().enumerate().map(|d|
                tr![
                    td![
                        &d.1.day.name.to_uppercase()
                    ],
                    d.1.hours.iter().enumerate().map(|h|
                        get_timetable_row((d.0+1) as i32, h, &model.timetable, &model)
                    )
                ]
            )
        ],
        input![
            attrs!{
                At::Type=>"button", At::Class=>"button is-primary", At::Value=>"Yazdır"
            },
            ev(Ev::Click, |event| {
                event.prevent_default();
                Msg::Print
            })
        ]
    ]
}

fn get_timetable_row(day: i32, hour: (usize, &bool), timetable: &Vec<TeacherTimetable>, _model: &Model)->Node<Msg>{
    let get_timetable = timetable.iter().find(|t| t.day_id == day && t.hour == hour.0 as i16);
    match get_timetable{
        Some(t)=>{
            td![
                if *hour.1 {
                    style!{
                        St::Background=>"blue",
                        St::Color => "white"
                    }
                }
                else{
                    style!{
                        St::Background=>"gray"
                    }
                },
                t.class_id.iter().map(|tc|
                    div![
                        &tc.kademe.to_string(), "/", &tc.sube
                    ]
                ),
                &t.subject[0..3]
            ]
        }
        None=>{
            td![
                if *hour.1 {
                    style!{
                        St::Background=>"blue"
                    }
                }
                else{
                    style!{
                        St::Background=>"gray"
                    }
                }
            ]
        }
    }
}