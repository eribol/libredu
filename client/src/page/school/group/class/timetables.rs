use crate::model::timetable::{Day};
use serde::*;
use seed::{*, prelude::*};
use crate::{Context};
use crate::model::class::{Class, ClassTimetable, ClassActivity};
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
    FetchActivities(fetch::Result<Vec<ClassActivity>>),
    FetchTimetable(fetch::Result<Vec<ClassTimetable>>),

}

#[derive(Debug, Default, Clone)]
pub struct Model{
    pub class: Class,
    pub timetable: Vec<ClassTimetable>,
    pub activities: Vec<ClassActivity>,
    pub classes: Vec<Class>,
    days: Vec<Day>,
}

pub fn init(class: i32, orders: &mut impl Orders<Msg>, ctx_school: &mut SchoolContext, ctx_group: &mut GroupContext)-> Model{
    let model = Model::default();
    orders.perform_cmd({
        let url = format!("/api/schools/{}/groups/{}/classes/{}", ctx_school.school.id, ctx_group.group.id,class);
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

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>, _ctx: &mut Context, ctx_school: &mut SchoolContext, ctx_group: &mut GroupContext) {
    match msg {
        Msg::Home => {
        }
        Msg::FetchClass(class)=>{
            model.class = class.unwrap();
            orders.perform_cmd({
                let url = format!("/api/schools/{}/groups/{}/classes/{}/timetables", &ctx_school.school.id, &ctx_group.group.id, model.class.id);
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
        Msg::FetchTimetable(tm)=>{
            model.timetable=tm.unwrap();
        }
        Msg::FetchDays(days)=>{
            model.days=days.unwrap();
        }
        Msg::FetchActivities(acts)=>{
            model.activities = acts.unwrap_or(vec![]);
        }

    }
}
pub fn view(model: &Model, ctx_school:&SchoolContext, ctx_group: &GroupContext)->Node<Msg>{
    div![
        div![
            C!{"tabs is-centered"},
            //tabs(model),
        ],
        context(model, ctx_school, ctx_group)
    ]

}
pub fn context(model: &Model, ctx_school:&SchoolContext, ctx_group: &GroupContext)->Node<Msg>{
    timetable(model, ctx_school, ctx_group)
}
fn timetable(model: &Model, _ctx_school:&SchoolContext, ctx_group: &GroupContext)->Node<Msg>{
    div![
        C!{"column"},
        table![
            style!{
                //St::TableLayout=>"fixed",
                //St::Width=>"1050px"
            },
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
            model.days.iter().enumerate().map(|d|
                tr![
                    td![
                        &d.1.name
                    ],
                    (0..ctx_group.group.hour).map(|h|
                        get_timetable_row((d.0+1) as i32, (h) as i16, &model.timetable)
                    )
                ]
            )
        ],
        input![
            attrs!{
                At::Type=>"button", At::Class=>"button is-primary", At::Value=>"Kaydet"
            }
            //ev(Ev::Click, |event| {
            //    event.prevent_default();
            //    Msg::Print
            //})
        ]
    ]
}

fn get_timetable_row(day: i32, hour: i16, timetable: &Vec<ClassTimetable>)->Node<Msg>{
    let get_timetable = timetable.iter().find(|t| t.day_id == day && t.hour == hour);

    match get_timetable{
        Some(t)=>{
            let name = &t.activity.teacher.first_name.chars().collect::<Vec<_>>();
            let lastname = &t.activity.teacher.last_name.chars().collect::<Vec<_>>();
            let mut subject = String::new();
            let split_subject = &t.subject.split(' ').collect::<Vec<&str>>();
            if split_subject.len() == 1{
                subject = subject + &split_subject[0].chars().collect::<Vec<_>>()[..3].iter().cloned().collect::<String>();
                subject.push_str(".");
            }
            else {
                for s in split_subject{
                    subject.push(s.chars().nth(0).unwrap());
                    subject.push_str(".");
                }
            }
            td![
                &name[..3].iter().cloned().collect::<String>().to_uppercase(),".", " ", &lastname[..3].iter().cloned().collect::<String>().to_uppercase(), ".",br![],
                &subject
            ]
        }
        None=>{
            td![

            ]
        }
    }
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
            a![
                "Aktiviteler",
                attrs!{
                    At::Href => format!("/schools/{}/groups/{}/classes/{}/activities", &ctx_school.school.id, &ctx_group.group.id, &model.class.id)
                }
            ]
        ],
        li![
            a![
                attrs!{
                    At::Href => format!("/schools/{}/groups/{}/classes/{}/limitations", &ctx_school.school.id, &ctx_group.group.id, &model.class.id)
                },
                "Kısıtlamalar",
            ]
        ],
        li![
            C!{"is-active"},
            a![
                attrs!{
                    At::Href => format!("/schools/{}/groups/{}/classes/{}/timetables", &ctx_school.school.id, &ctx_group.group.id, &model.class.id)
                },
                "Ders Tablosu"
            ]
        ]
    ]
}