use serde::*;
use seed::{*, prelude::*};
use crate::{Context};
use crate::page::school::detail::{ClassGroups, SchoolContext};
use chrono::NaiveTime;

#[derive(Debug, Clone,Deserialize, Serialize)]
pub struct Form{
    group_id: i32,
    hour: i32,
    start_time: NaiveTime,
    end_time: NaiveTime
}

pub fn init(url: Url, ctx_school: &mut SchoolContext, group: &ClassGroups, orders: &mut impl Orders<Msg>) -> Model{
    let mut model = Model::default();
    model.group = group.clone();
    orders.perform_cmd({
        let url = format!("/api/schools/{}/groups/{}/schedules", ctx_school.school.id, group.id);
        let request = Request::new(url)
            .method(Method::Get);
        async {
            Msg::FetchSchedules(async {
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
#[derive(Debug, Default, Clone)]
pub struct Model{
    pub form: Vec<Form>,
    group: ClassGroups
}

#[derive(Debug)]
pub enum Msg{
    ChangeTime,
    FetchSchedules(fetch::Result<Vec<Form>>),
    UpdateSchedules,
    ChangeStartTime(usize, String),
    ChangeEndTime(usize, String)
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>, _ctx: &mut Context, ctx_school: &mut SchoolContext) {
    match msg {
        Msg::ChangeTime => {
        }
        Msg::FetchSchedules(schedules) => {
            match schedules{
                Ok(s) => {
                    model.form = s.clone();
                }
                Err(_) => {
                }
            }
        }
        Msg::UpdateSchedules => {
            orders.perform_cmd({
                let url = format!("/api/schools/{}/groups/{}/schedules", ctx_school.school.id, model.group.id);
                let request = Request::new(url)
                    .method(Method::Patch)
                    .json(&model.form);
                async {
                    Msg::FetchSchedules(async {
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
        Msg::ChangeStartTime(u, t) => {
            if model.form.len() == 0{
                let new = Form{
                    group_id: model.group.id.clone(),
                    hour: (u+1)as i32,
                    start_time: NaiveTime::parse_from_str(&t, "%H:%M").unwrap(),
                    end_time: NaiveTime::parse_from_str("00:00", "%H:%M").unwrap()
                };
                model.form.push(new)
            }
            else {
                model.form[u].start_time = NaiveTime::parse_from_str(&t, "%H:%M").unwrap();
            }
        }
        Msg::ChangeEndTime(u, t) => {
            if model.form.len() == 0{
                let new = Form{
                    group_id: model.group.id.clone(),
                    hour: (u+1)as i32,
                    start_time: NaiveTime::parse_from_str("00:00", "%H:%M").unwrap(),
                    end_time: NaiveTime::parse_from_str(&t, "%H:%M").unwrap(),
                };
                model.form.push(new)
            }
            else {
                model.form[u].end_time = NaiveTime::parse_from_str(&t, "%H:%M").unwrap();
            }
        }
    }
}

pub fn view(model: &Model) -> Node<Msg>{
    div![
        table![
            C!{"table"},
            thead![
                tr![
                    th!["Saat"],
                    th!["Giriş"],
                    th!["Çıkış"]
                ]
            ],
            tbody![
                model.form.iter().map(|h|
                    tr![
                        td![(h.hour).to_string(), ". Saat"],
                        td![
                            input![
                                attrs!{
                                    At::Type => "text",
                                    At::Value => &h.start_time.format("%H:%M").to_string(),
                                },
                                {
                                    let u = h.hour as usize;
                                    input_ev(Ev::Change, move |time| Msg::ChangeStartTime(u-1, time))
                                }
                            ]
                        ],
                        td![
                            input![
                                attrs!{
                                    At::Type => "text",
                                    At::Value => &h.end_time.format("%H:%M").to_string(),
                                },
                                {
                                    let u = h.hour as usize;
                                    input_ev(Ev::Change, move |time| Msg::ChangeEndTime(u-1, time))
                                }
                            ]
                        ]
                    ]
                )
            ]
        ],
        input![
            attrs!{
                At::Type => "button",
                At::Value => "Saatleri Güncelle"
            },
            C!{"button is-secondary"},
            ev(Ev::Click, |event| {
                event.prevent_default();
                Msg::UpdateSchedules
            })
        ]
    ]
}