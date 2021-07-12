use serde::*;
use seed::{*, prelude::*};
use crate::page::school::detail::SchoolContext;
use chrono::NaiveTime;
use crate::model::group;
use crate::i18n::I18n;

#[derive(Debug, Clone,Deserialize, Serialize)]
pub struct Form{
    group_id: i32,
    hour: i32,
    start_time: NaiveTime,
    end_time: NaiveTime
}

pub fn init(url: Url, school_ctx: &SchoolContext, orders: &mut impl Orders<Msg>) -> Model{
    let group_ctx = &school_ctx.get_group(&url);
    orders.perform_cmd({
        let url = format!("/api/schools/{}/groups/{}/schedules", group_ctx.group.school, group_ctx.group.id);
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
    Model::default()
}
#[derive(Debug, Default, Clone)]
pub struct Model{
    pub form: Vec<Form>,
    //group: group::ClassGroups
}

#[derive(Debug)]
pub enum Msg{
    ChangeTime,
    FetchSchedules(fetch::Result<Vec<Form>>),
    UpdateSchedules,
    ChangeStartTime(usize, String),
    ChangeEndTime(usize, String)
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>, group_ctx: &mut group::GroupContext) {
    match msg {
        Msg::ChangeTime => {
        }
        Msg::FetchSchedules(schedules) => {
            if let Ok(s) = schedules {
                model.form = s;
            }
        }
        Msg::UpdateSchedules => {
            orders.perform_cmd({
                let url = format!("/api/schools/{}/groups/{}/schedules", group_ctx.group.school, group_ctx.group.id);
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
            if model.form.is_empty(){
                let new = Form{
                    group_id: group_ctx.group.id,
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
            if model.form.is_empty(){
                let new = Form{
                    group_id: group_ctx.group.id,
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

pub fn view(model: &Model, lang: &I18n) -> Node<Msg>{
    use crate::{create_t, with_dollar_sign};
    create_t![lang];
    div![
        table![
            C!{"table"},
            thead![
                tr![
                    th![t!["hour"]],
                    th![t!["schedules-start"]],
                    th![t!["schedules-end"]]
                ]
            ],
            tbody![
                model.form.iter().map(|h|
                    tr![
                        td![(h.hour).to_string(), ". ", t!["hour"]],
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
                At::Value => t!["update"]
            },
            C!{"button is-secondary"},
            ev(Ev::Click, |event| {
                event.prevent_default();
                Msg::UpdateSchedules
            })
        ]
    ]
}