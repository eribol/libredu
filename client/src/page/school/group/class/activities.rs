use crate::model::timetable::{Day};
use serde::*;
use seed::{*, prelude::*};
use crate::{Context};
use crate::model::timetable::ClassAvailable;
use crate::model::class::{Class, ClassActivity};
use crate::model::school::SchoolDetail;
use crate::model::activity;
use crate::page::school::detail::{SchoolContext, ClassGroups, GroupContext};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Activity{
    pub(crate) id: i32,
    pub(crate) subject: activity::Subject,
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
    //FetchClasses(fetch::Result<Vec<Class>>),
    FetchActivities(fetch::Result<Vec<ClassActivity>>),
    FetchAct(fetch::Result<Vec<ClassActivity>>),
    FetchSubjects(fetch::Result<Vec<activity::Subject>>),
    ChangeHour((usize,usize)),
    SubmitActivity,
    ChangeActTeacher(String),
    ChangeActHour(String),
    ChangeActSubject(String),
    DeleteActivity(String),
    FetchDeleteAct(fetch::Result<String>),
}

#[derive(Debug, Default, Clone)]
pub struct Model{
    pub class: Class,
    pub school: SchoolDetail,
    pub activities: Vec<ClassActivity>,
    pub classes: Vec<Class>,
    subjects: Vec<activity::Subject>,
    days: Vec<Day>,
    pub act_form: activity::NewActivity,
    pub limitation: Vec<ClassAvailable>,
    pub group: ClassGroups,
}

pub fn init(class: i32, orders: &mut impl Orders<Msg>, ctx_school: &mut SchoolContext, ctx_group: &mut GroupContext)-> Model{
    let mut model = Model::default();
    model.group = ctx_group.group.clone();
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
    orders.perform_cmd({
        let url = format!("/api/schools/{}/groups/{}/classes/{}/activities", ctx_school.school.id, &ctx_group.group.id, &class);
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
        Msg::FetchAct(act)=>{
            model.activities.append(&mut act.unwrap());
        }
        Msg::FetchClass(class)=>{
            match class{
                Ok(cls) => {
                    model.class = cls;
                    orders.perform_cmd({
                        let url = format!("/api/schools/{}/subjects", ctx_school.school.id);
                        let request = Request::new(url)
                            .method(Method::Get);
                        async {
                            Msg::FetchSubjects(async {
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
                Err(e) => {
                    log!(e);
                }
            }

        }
        Msg::FetchDays(days)=>{
            model.days=days.unwrap();
        }
        Msg::ChangeActTeacher(t)=>{
            model.act_form.teacher = t.parse::<i32>().unwrap();
        }
        Msg::ChangeActHour(h)=>{
            model.act_form.hour = h;
        }
        Msg::ChangeActSubject(s)=>{
            model.act_form.subject = s.parse::<i32>().unwrap();
        }
        Msg::ChangeHour(ids)=>{
            if model.limitation[ids.0].hours[ids.1]{
                model.limitation[ids.0].hours[ids.1]=false;
            }
            else{
                model.limitation[ids.0].hours[ids.1]=true;
            }
        }
        Msg::FetchSubjects(subjects)=>{
            match subjects{
                Ok(sbjcts) => {
                    //log!(&sbjcts);
                    model.subjects = sbjcts.clone().into_iter().filter(|s| s.kademe == model.class.kademe).collect();
                    log!(&model.subjects);
                    if model.subjects.len() > 0 {
                        model.act_form.subject = model.subjects[0].id;
                    }
                    if ctx_school.teachers.len() > 0 {
                        model.act_form.teacher = ctx_school.teachers[0].id;
                    }
                }
                Err(_) => {
                    //log!(e);
                }
            }
        }
        Msg::FetchActivities(acts)=>{
            model.activities = acts.unwrap_or(vec![]);
        }
        Msg::DeleteActivity(id)=>{
            let i =id.parse::<i32>().unwrap();
            orders.perform_cmd({
                let url = format!("/api/schools/{}/groups/{}/classes/{}/activities/{}", ctx_school.school.id, ctx_group.group.id, &model.class.id, i);
                let request = Request::new(url)
                    .method(Method::Delete);
                async {
                    Msg::FetchDeleteAct(async {
                        request
                            .fetch()
                            .await?
                            .text()
                            .await
                    }.await)
                }
            });
        }
        Msg::FetchDeleteAct(id)=>{
            match id{
                Ok(i)=>{
                    model.activities.retain(|a| a.id != i.parse::<i32>().unwrap())
                }
                Err(_)=>{}
            }

        }
        Msg::SubmitActivity=>{
            model.act_form.classes.push(model.class.id);
            model.act_form.split = false;
            if model.act_form.teacher != 0 && model.act_form.subject != 0 && model.act_form.hour !=""{
                orders.perform_cmd({
                    let url = format!("/api/schools/{}/groups/{}/classes/{}/activities", ctx_school.school.id, &ctx_group.group.id, &model.class.id);
                    let request = Request::new(url)
                        .method(Method::Post)
                        .json(&model.act_form);
                    async {
                        Msg::FetchAct(async {
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
pub fn activities(model: &Model, ctx_school:&SchoolContext)->Node<Msg>{
    div![
        C!{"columns"},
    div![
        C!{"column is-6"},
        "Toplam Ders saati: ", &model.activities.iter().fold(0, |acc, a| acc+a.hour).to_string(),
        hr![],
        table![
            C!{"table table-hover"},
            thead![
                tr![
                    th![
                        attrs!{At::Scope=>"col"},
                        "Öğretmen"
                    ],
                    th![
                        attrs!{At::Scope=>"col"},
                        "Ders"
                    ],
                    th![
                        attrs!{At::Scope=>"col"},
                        "Süre"
                    ]
                ]
            ],
            tbody![
                model.activities.iter().map(|a|
                    tr![
                        td![
                            &a.teacher.first_name, " ", &a.teacher.last_name
                        ],
                        td![
                            &a.subject.name
                        ],
                        td![
                            &a.hour.to_string()
                        ],
                        td![
                            a![
                                "Sil",
                                //attrs!{At::Type=>"button", At::Class=>"button", At::Value=>"Sil"},
                                {
                                    let id = a.id;
                                    ev(Ev::Click, move |_event| {
                                        Msg::DeleteActivity(id.to_string())
                                    })
                                }
                            ]
                        ]
                    ]
                )
            ]
        ]
    ],
    div![
        C!{"column"},
        div![
            C!{"select"},
            select![
                attrs!{At::Name=>"teacher"},
                ctx_school.teachers.iter().map(|t|
                    option![
                        attrs!{At::Value=>&t.id},
                        &t.first_name, " ", &t.last_name
                    ]
                ),
                input_ev(Ev::Change, Msg::ChangeActTeacher)
            ]
        ],
        div![
            C!{"select"},
            select![
                attrs!{At::Name=>"subject"},
                model.subjects.iter().map(|s|
                    option![
                        attrs!{At::Value=>&s.id},
                        &s.name, if s.optional {" Seçmeli"} else {""}
                    ]
                ),
                input_ev(Ev::Change, Msg::ChangeActSubject)
            ]
        ],
        input![
            attrs!{At::Type=>"text", At::Name=>"hour", At::Id=>"hour"},
            input_ev(Ev::Change, Msg::ChangeActHour)
        ],
        input![
            attrs!{At::Type=>"button", At::Class=>"button", At::Value=>"Ekle"},
            ev(Ev::Click, |event| {
                event.prevent_default();
                Msg::SubmitActivity
            })
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
            C!{"is-active"},
            a![
                "Aktiviteler",
                attrs!{
                    At::Href => format!("/schools/{}/groups/{}/classes/{}/activites", &ctx_school.school.id, &ctx_group.group.id, &model.class.id)
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
            a![
                attrs!{
                    At::Href => format!("/schools/{}/groups/{}/classes/{}/timetables", &ctx_school.school.id, &ctx_group.group.id, &model.class.id)
                },
                "Ders Tablosu"
            ]
        ]
    ]
}