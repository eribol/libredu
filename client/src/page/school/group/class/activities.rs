use crate::model::timetable::{Day};
use serde::*;
use seed::{*, prelude::*};
use crate::model::timetable::ClassAvailable;
use crate::model::class::{Class};
use crate::model::{activity, subject};
use crate::page::school::detail::SchoolContext;
use web_sys::{HtmlOptionElement, HtmlSelectElement};
use crate::model::activity::FullActivity;
use crate::i18n::I18n;

#[derive()]
pub enum Msg{
    Home,
    FetchDays(fetch::Result<Vec<Day>>),
    FetchClass(fetch::Result<Class>),
    //FetchClasses(fetch::Result<Vec<Class>>),
    FetchActivities(fetch::Result<Vec<FullActivity>>),
    FetchAct(fetch::Result<Vec<FullActivity>>),
    FetchSubjects(fetch::Result<Vec<subject::Subject>>),
    ChangeHour((usize,usize)),
    SubmitActivity,
    ChangeActTeacher(String),
    ChangeActHour(String),
    ChangeActSubject(String),
    DeleteActivity(String),
    FetchDeleteAct(fetch::Result<String>),
}

#[derive(Default, Clone)]
pub struct Model{
    days: Vec<Day>,
    pub act_form: activity::NewActivity,
    pub limitation: Vec<ClassAvailable>,
    select: ElRef<HtmlSelectElement>,
    url: Url
}

pub fn init(url: Url, orders: &mut impl Orders<Msg>, school_ctx: &mut SchoolContext)-> Model{
    let mut model = Model{url: url.clone(),..Default::default()};
    let group_ctx = school_ctx.get_group(&url);
    let class_ctx = group_ctx.get_class(&url);
    model.act_form.teachers = vec![school_ctx.teachers.as_ref().unwrap()[0].teacher.id];
    if class_ctx.activities.is_none(){
        orders.perform_cmd({
            let url = format!("/api/schools/{}/groups/{}/classes/{}/activities", school_ctx.school.id, group_ctx.group.id, &class_ctx.class.id);
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
    }
    if school_ctx.subjects.is_none(){
        orders.perform_cmd({
            let url = format!("/api/schools/{}/subjects", school_ctx.school.id);
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
    else{
        let subject = school_ctx.subjects.as_ref().unwrap();
        if !subject.is_empty(){
            model.act_form.subject = subject[0].id;
        }
    }
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
    model
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>, school_ctx: &mut SchoolContext) {
    let group_ctx = &model.url.path()[3];
    let school_id = &model.url.path()[1];
    let class_ctx = school_ctx.get_mut_group(&model.url).get_mut_class(&model.url);
    match msg {
        Msg::Home => {
        }
        Msg::FetchAct(act)=>{
            if let Some(acts) = class_ctx.activities.as_mut() { acts.append(&mut act.unwrap()) }
        }
        Msg::FetchClass(class)=>{
            /*
            if let Ok(cls) = class {
                model.class = cls;

            }

             */
        }
        Msg::FetchDays(days)=>{
            model.days=days.unwrap();
            model.act_form.classes = vec![class_ctx.class.id];
        }
        Msg::ChangeActTeacher(t)=>{
            model.act_form.teachers = vec![];
            let selected_options = model.select.get().unwrap().selected_options();
            for i in 0..selected_options.length() {
                let item = selected_options.item(i).unwrap().dyn_into::<HtmlOptionElement>().unwrap();
                if item.selected() {
                    model.act_form.teachers.push(item.value().parse::<i32>().unwrap());
                }
            }
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
            if let Ok(sbjcts) = subjects {
                school_ctx.subjects = Some(sbjcts.into_iter().filter(|s| s.kademe == class_ctx.class.kademe).collect());
                if let Some(subs) = &mut school_ctx.subjects{
                    if !subs.is_empty(){
                        model.act_form.subject = subs[0].id;
                    }
                }
            }
            else {
                school_ctx.subjects = Some(vec![]);
            }
        }
        Msg::FetchActivities(acts)=>{
            if let Ok(acts) = acts{
                class_ctx.activities = Some(acts);
            }
        }
        Msg::DeleteActivity(id)=>{
            let i =id.parse::<i32>().unwrap();
            orders.perform_cmd({
                let url = format!("/api/schools/{}/groups/{}/classes/{}/activities/{}", &school_id, &group_ctx, &class_ctx.class.id, i);
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
            if let Ok(i) = id {
                if let Some(activities) = &mut class_ctx.activities{
                    activities.retain(|a| a.id != i.parse::<i32>().unwrap())
                }
            }

        }
        Msg::SubmitActivity=>{
            if model.act_form.classes.is_empty(){
                model.act_form.classes.push(class_ctx.class.id);
            }
            model.act_form.split = false;

            if !model.act_form.teachers.is_empty() && model.act_form.subject != 0 && !model.act_form.hour.is_empty(){
                orders.perform_cmd({
                    let url = format!("/api/schools/{}/groups/{}/activities", &school_ctx.school.id, &group_ctx);
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

pub fn activities(model: &Model, school_ctx:&SchoolContext, lang: &I18n)->Node<Msg>{
    let group_ctx = school_ctx.get_group(&model.url);
    let class_ctx = group_ctx.get_class(&model.url);
    use crate::{create_t, with_dollar_sign};
    create_t![lang];
    div![
        C!{"columns"},
        div![
            C!{"column is-7"},
            t!["total-activity-hour"], " ", &class_ctx.activities.as_ref().map_or(0.to_string(), |activities| activities.iter().fold(0, |acc, a| acc+a.hour).to_string()),
            hr![],
            table![
                C!{"table table-hover"},
                thead![
                    tr![
                        th![
                            attrs!{At::Scope=>"col"},
                            t!["activity-teachers"]
                        ],
                        th![
                            attrs!{At::Scope=>"col"},
                            t!["activity-subject"]
                        ],
                        th![
                            attrs!{At::Scope=>"col"},
                            t!["activity-hour"]
                        ],
                        th![
                            attrs!{At::Scope=>"col"},
                            t!["process"]
                        ]
                    ]
                ],
                class_ctx.activities.as_ref().map_or(
                    tbody![], |activities|
                    tbody![
                        activities.iter().map(|a|
                            tr![
                                td![
                                    a.teachers.iter().map(|t|
                                        label![
                                            &t.first_name, " ", &t.last_name, br![]
                                        ]
                                    )
                                ],
                                td![
                                    &a.subject.name
                                ],
                                td![
                                    &a.hour.to_string()
                                ],
                                td![
                                    a![
                                        t!["delete"],
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
                )
            ]
        ],
        div![
            C!{"column"},
            div![
                C!{"field"},
                label![
                    C!{"label"},
                    t!["choose-activity-teachers"]
                ],
                p![
                    C!{"control"},
                    span![
                        C!{"select is-multiple"},
                        school_ctx.teachers.as_ref().map_or(
                            select![
                                el_ref(&model.select),
                                attrs!{
                                    At::from("multiple") => true.as_at_value()
                                }
                            ],
                            |teachers|
                            select![
                                el_ref(&model.select),
                                attrs!{
                                    At::from("multiple") => true.as_at_value()
                                },
                                teachers.iter().map(|t|
                                    option![
                                        attrs!{At::Value => &t.teacher.id},
                                        format!("{} {}", &t.teacher.first_name, &t.teacher.last_name)
                                    ]
                                ),
                                input_ev(Ev::Change, Msg::ChangeActTeacher)
                            ]
                        )
                    ]
                ]
            ],
            div![
                C!{"field"},
                label![
                    C!["label"],
                    t!["choose-activity-subject"]
                ]
            ],
            div![
                C!{"field"},
                C!{"select"},
                school_ctx.subjects.as_ref().map_or(select![], |subjects|
                    select![
                        attrs!{At::Name=>"subject"},
                        subjects.iter().map(|s|
                            option![
                                attrs!{At::Value=>&s.id},
                                &s.name
                            ]
                        ),
                        input_ev(Ev::Change, Msg::ChangeActSubject)
                    ]
                )
            ],
            div![
                C!{"field"},
                label![
                    C!["label"],
                    t!["choose-activity-hour"]
                ]
            ],
            div![
                C!{"field"},
                input![
                    C!["input"],
                    attrs!{At::Type=>"text", At::Name=>"hour", At::Id=>"hour"},
                    input_ev(Ev::Change, Msg::ChangeActHour)
                ]
            ],
            div![
                C!{"field"},
                input![
                //C!{"butt"},
                    attrs!{At::Type=>"button", At::Class=>"button", At::Value=> t!["add"]},
                    ev(Ev::Click, |event| {
                        event.prevent_default();
                        Msg::SubmitActivity
                    })
                ]
            ],
            div![
                C!{"field"},
                label![
                    t!["activity-info"]
                ]
            ]
        ]
    ]
}