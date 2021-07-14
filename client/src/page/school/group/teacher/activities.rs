use seed::{*, prelude::*};
use crate::model::{activity, subject};
use crate::page::school::detail;
use serde::*;
use crate::page::school::detail::{SchoolContext};
use web_sys::{HtmlSelectElement, HtmlOptionElement};
use crate::i18n::I18n;

#[derive()]
pub enum Msg{
    Home,
    FetchActivities(fetch::Result<Vec<activity::FullActivity>>),
    FetchAct(fetch::Result<Vec<activity::FullActivity>>),
    FetchSubjects(fetch::Result<Vec<subject::Subject>>),
    SubmitActivity,
    ChangeActClass(String),
    ChangeActHour(String),
    ChangeActSubject(String),
    DeleteActivity(String),
    FetchDeleteAct(fetch::Result<String>),
    Loading
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Teacher{
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub is_active: bool,
    pub email: Option<String>,
    pub tel: Option<String>
}

#[derive(Default, Clone)]
pub struct Model{
    url: Url,
    act_form: activity::NewActivity,
    select: ElRef<HtmlSelectElement>,
    error: String
}

pub fn init(url: Url, orders: &mut impl Orders<Msg>, _school_ctx: &mut SchoolContext)-> Model{
    let model = Model{url: url, ..Default::default()};
    orders.send_msg(Msg::Loading);
    model
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>, school_ctx: &mut detail::SchoolContext) {

    match msg {
        Msg::Home => {
        }
        Msg::FetchAct(act)=>{
            let teacher_ctx = school_ctx.get_mut_teacher(&model.url);
            if let Ok(mut acts) = act {
                if let Some(activities) = &mut teacher_ctx.activities{
                    log!("len act teacher = {}", acts[0].teachers.len());
                    activities.append(&mut acts);
                }
                else{
                    teacher_ctx.activities = Some(acts);
                }
            }
        }
        Msg::ChangeActClass(_value)=>{
            let group_ctx = school_ctx.get_group(&model.url);
            model.act_form.classes = vec![];
            let selected_options = model.select.get().unwrap().selected_options();
            for i in 0..selected_options.length() {
                let item = selected_options.item(i).unwrap().dyn_into::<HtmlOptionElement>().unwrap();
                if item.selected() {
                    model.act_form.classes.push(item.value().parse::<i32>().unwrap());
                }
            }
            log!(&model.act_form.classes.len());
            //let class= group_ctx.classes.iter().find(|c| c.class.id == model.act_form.classes[0]).unwrap();
            //model.selected_subjects = model.subjects.clone().into_iter().filter(|s| s.kademe == class.kademe).collect();
            //let classes = ctx_group.classes.into_iter()
            //    .filter(|c| c.kademe == class.kademe && model.act_form.classes.iter().any(|c| c)).collect::<Vec<i32>>();
            //model.act_form.classes.push(c.parse::<i32>().unwrap());
        }
        Msg::ChangeActHour(h)=>{
            model.act_form.hour = h;
        }
        Msg::ChangeActSubject(s)=>{
            model.act_form.subject = s.parse::<i32>().unwrap();
        }
        Msg::FetchSubjects(subjects)=>{
            if let Ok(s) = subjects {
                if let Some(sbjcts) = &mut school_ctx.subjects{
                    *sbjcts = s.clone();
                }
                else {
                    school_ctx.subjects = Some(s.clone());
                }
                if !s.is_empty() {
                    model.act_form.subject = s[0].id;
                } else {
                    model.error = "Ders ekleyin.".to_string()
                }
            }

        }
        Msg::FetchActivities(acts)=>{
            let teacher_ctx = school_ctx.get_mut_teacher(&model.url);
            model.act_form.teachers = vec![teacher_ctx.teacher.id];
            if let Ok(a) = acts {
                if let Some(activities) = &mut teacher_ctx.activities{
                    *activities = a.clone();
                }
                else{
                    teacher_ctx.activities = Some(a);
                }
            }
        }
        Msg::DeleteActivity(id)=>{
            if let Ok(i) = id.parse::<i32>() {
                orders.perform_cmd({
                    let url = format!("/api/schools/{}/groups/{}/teachers/{}/activities/{}", school_ctx.school.id, model.url.path()[3], model.url.path()[5], i);
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
        }
        Msg::FetchDeleteAct(id)=>{
            if let Ok(i) = id {
                let teacher_ctx = school_ctx.get_mut_teacher(&model.url);
                if let Some(activities) = &mut teacher_ctx.activities{
                    activities.retain(|a| a.id != i.parse::<i32>().unwrap());
                }
            }
        }
        Msg::SubmitActivity=>{
            let group_ctx = school_ctx.get_group(&model.url);
            model.act_form.teachers = vec![model.url.path()[5].parse().unwrap()];
            model.act_form.split = false;
            if group_ctx.classes.is_some() && school_ctx.subjects.is_some() &&
                !model.act_form.classes.is_empty() &&
                model.act_form.subject != 0 &&
                !model.act_form.hour.is_empty() && !model.act_form.teachers.is_empty(){
                orders.perform_cmd({
                    let url = format!("/api/schools/{}/groups/{}/activities", school_ctx.school.id, model.url.path()[3]);
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
        Msg::Loading => {
            if let Some(t) = school_ctx.teachers.as_ref().map_or(None, |teachers| teachers.iter().find(|t| t.teacher.id == model.url.path()[5].parse::<i32>().unwrap())){
                let teacher_ctx = school_ctx.get_teacher(&model.url);
                if teacher_ctx.activities.is_none(){
                    orders.perform_cmd({
                        let url = format!("/api/schools/{}/groups/{}/teachers/{}/activities", school_ctx.school.id, model.url.path()[3], teacher_ctx.teacher.id);
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
            else if let Some(s) = &school_ctx.subjects{
                if !s.is_empty() {
                    model.act_form.subject = s[0].id;
                }
            }
        }
        /*Msg::PatchActivity(_id) => {
            /*
            orders.perform_cmd({
                let url = format!("/api/schools/{}/teachers/{}/activities/{}", ctx_school.school.id, model.teacher.id, id);
                let request = Request::new(url)
                    .method(Method::Patch);
                async {
                    Msg::FetchPatchAct(async {
                        request
                            .fetch()
                            .await?
                            .check_status()?
                            .text()
                            .await
                    }.await)
                }
            });
            */
        }
        Msg::FetchPatchAct(_act)=>{
            /*
            match act{
                Ok(a) =>{
                    let acts = &mut model.activities;
                    let id = model.teacher.id;
                    for aa in acts{
                        if aa.id == a.parse::<i32>().unwrap(){
                            aa.teacher = Some(id);
                            break;
                        }
                    }
                }
                Err(_) => {

                }
            }
            */
        }
        */
    }
}

pub fn view(model: &Model, school_ctx:&SchoolContext, lang: &I18n)->Node<Msg>{
    let teacher_ctx = school_ctx.get_teacher(&model.url);
    let group_ctx = school_ctx.get_group(&model.url);
    use crate::{create_t, with_dollar_sign};
    create_t![lang];
    div![
        C!{"columns"},
        div![
            C!{"column"},
            t!["total-activity-hour"], &teacher_ctx.activities.as_ref().map_or(0.to_string(), |acts| acts.iter().fold(0, |acc, a| acc+a.hour).to_string()),
            hr![],

            table![
                C!{"table table-hover"},
                thead![
                    tr![
                        th![
                            attrs!{At::Scope=>"col"},
                            t!["classes"]
                        ],
                        th![
                            attrs!{At::Scope=>"col"},
                            t!["activity-subject"]
                        ],
                        th![
                            attrs!{At::Scope=>"col"},
                            t!["activity-hour"]
                        ]
                    ]
                ],
                teacher_ctx.activities.as_ref().map_or(tbody![], |acts|
                    tbody![
                        acts.iter().map(|a|
                            tr![
                                if &a.teachers.len() == &0{
                                    style!{
                                        St::BackgroundColor=>"gray"
                                    }
                                }
                                else {
                                    style!{}
                                },
                                td![
                                    a.classes.iter().map(|c|
                                        div![
                                            c.kademe.to_string()+"/"+&c.sube
                                        ]
                                    )
                                ],
                                td![
                                    &a.subject.name
                                ],
                                td![
                                    &a.hour.to_string()
                                ],
                                if &a.teachers.len() >= &1{
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
                                }
                                else {
                                    td![
                                        input![
                                            attrs!{At::Type=>"button", At::Class=>"button", At::Value=> t!["move"]},
                                            {
                                                let id = a.id;
                                                ev(Ev::Click, move |_event| {
                                                })
                                            }
                                        ]
                                    ]
                                }
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
                    t!["choose-activity-classes"]
                ],
                p![
                    C!{"control"},
                    span![
                        C!{"select is-multiple"},
                        group_ctx.classes.as_ref().map_or(
                            select![
                                el_ref(&model.select),
                                attrs!{
                                    At::from("multiple") => true.as_at_value()
                                }
                            ],
                            |classes|
                            select![
                                el_ref(&model.select),
                                attrs!{
                                    At::from("multiple") => true.as_at_value()
                                },
                                classes.iter().map(|c|
                                    option![
                                        attrs!{At::Value=>&c.class.id},
                                        format!("{}/{} Sınıfı", &c.class.kademe.to_string(), &c.class.sube)
                                    ]
                                ),
                                input_ev(Ev::Change, Msg::ChangeActClass)
                            ]
                        )
                    ]
                ]
            ],
            div![
                C!{"field"},
                label![
                    C!{"label"},
                    t!["choose-activity-subject"]
                ],
                p![
                    C!{"control"},
                    span![
                        C!{"select"},
                        school_ctx.subjects.as_ref().map_or(
                            select![],
                            |subjects|
                            select![
                                subjects.iter().map(|s|
                                    option![
                                        attrs!{At::Value=>&s.id},
                                        &s.name, "(", &s.kademe.to_string(), ")", if s.optional{" Seçmeli"} else{""}
                                    ]
                                ),
                                input_ev(Ev::Change, Msg::ChangeActSubject)
                            ]
                        )
                    ]
                ]

            ],
            div![
                C!{"field"},
                label![
                    C!{"label"},
                    t!["choose-activity-hour"]
                ],
                p![
                    C!{"control"},

                        input![
                            attrs!{At::Class => "input", At::Type=>"text", At::Name=>"hour"},
                            input_ev(Ev::Change, Msg::ChangeActHour)
                        ]

                ]
            ],
            div![
                C!{"field"},
                input![
                    attrs!{At::Type=>"button", At::Class=>"button", At::Value=> t!["add"]},
                    ev(Ev::Click, |event| {
                        event.prevent_default();
                            Msg::SubmitActivity
                        })
                ]
            ]
        ]
        /*div![
            C!{"column is-2"},
            ctx_school.teachers.iter().map(|t|
                &t.first_name
            )
        ]*/
    ]
}