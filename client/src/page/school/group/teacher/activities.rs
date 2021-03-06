use seed::{*, prelude::*};
use crate::{Context};
use crate::model::activity;
use crate::page::school::detail;
use serde::*;
use crate::page::school::detail::{SchoolContext, GroupContext};
use web_sys::{HtmlSelectElement, HtmlOptionElement};

#[derive(Debug)]
pub enum Msg{
    Home,
    FetchTeacher(fetch::Result<Teacher>),
    FetchActivities(fetch::Result<Vec<activity::TeacherActivity>>),
    FetchAct(fetch::Result<Vec<activity::TeacherActivity>>),
    FetchSubjects(fetch::Result<Vec<activity::Subject>>),
    SubmitActivity,
    ChangeActClass(String),
    ChangeActHour(String),
    ChangeActSubject(String),
    DeleteActivity(String),
    FetchDeleteAct(fetch::Result<String>),
    PatchActivity(String),
    FetchPatchAct(fetch::Result<String>),
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
    teacher: Teacher,
    act_form: activity::NewActivity,
    activities: Vec<activity::TeacherActivity>,
    subjects: Vec<activity::Subject>,
    select: ElRef<HtmlSelectElement>,
    error: String
}

pub fn init(teacher: i32, orders: &mut impl Orders<Msg>, _ctx: &mut Context, ctx_school: &mut SchoolContext, ctx_group: &GroupContext)-> Model{
    let model = Model::default();
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
        Msg::FetchTeacher(t) => {
            match t{
                Ok(teacher) => {
                    model.teacher = teacher;
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
                }
                Err(_) => {}
            }

        }
        Msg::FetchAct(act)=>{
            match act{
                Ok(mut a) => {
                    model.activities.append(&mut a);
                }
                Err(_) => {}
            }
        }
        Msg::ChangeActClass(_value)=>{
            /*let window = web_sys::window().expect("a");
            let d = window.document().expect("aa");
            let classes = d.get_elements_by_name("classes");
            log!(classes.item(0));
            log!(classes.get(0).unwrap().selected_index());*/
            model.act_form.classes = vec![];
            let selected_options = model.select.get().unwrap().selected_options();
            for i in 0..selected_options.length() {
                let item = selected_options.item(i).unwrap().dyn_into::<HtmlOptionElement>().unwrap();
                if item.selected() {
                    model.act_form.classes.push(item.value().parse::<i32>().unwrap());
                }
            }
            //model.act_form.classes.push(c.parse::<i32>().unwrap());
        }
        Msg::ChangeActHour(h)=>{
            model.act_form.hour = h;
        }
        Msg::ChangeActSubject(s)=>{
            model.act_form.subject = s.parse::<i32>().unwrap();
        }
        Msg::FetchSubjects(subjects)=>{
            match subjects{
                Ok(s) => {
                    model.subjects = s.clone();
                    if s.len() > 0{
                        model.act_form.subject = model.subjects[0].id;
                    }
                    else {
                        model.error = "Ders ekleyin.".to_string()
                    }
                }
                Err(_) => {
                }
             }

        }
        Msg::FetchActivities(acts)=>{
            match acts{
                Ok(a) => {
                    model.activities = a.clone().into_iter().filter(|a|
                        ctx_group.classes.iter().any(|c| a.classes.iter().any(|a2| a2.id == c.id))
                    ).collect()
                }
                Err(e) => {
                    log!(e);
                }
            }
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
        Msg::DeleteActivity(id)=>{
            let i =id.parse::<i32>().unwrap();
            orders.perform_cmd({
                let url = format!("/api/schools/{}/groups/{}/teachers/{}/activities/{}", ctx_school.school.id, ctx_group.group.id, model.teacher.id, i);
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
                    // This should change. It search all vec and remove but it does not stop when it find the right element.
                    model.activities.retain(|a| a.id != i.parse::<i32>().unwrap())
                }
                Err(_)=>{}
            }

        }
        Msg::SubmitActivity=>{
            model.act_form.teacher = model.teacher.id;
            model.act_form.split = false;
            if ctx_group.classes.len() > 0 && model.subjects.len() > 0{
                if model.act_form.classes.len() != 0 && model.act_form.subject != 0 && model.act_form.hour != ""{
                    orders.perform_cmd({
                        let url = format!("/api/schools/{}/groups/{}/teachers/{}/activities", ctx_school.school.id, ctx_group.group.id, model.teacher.id);
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
        Msg::PatchActivity(id) => {
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
        }
        Msg::FetchPatchAct(act)=>{
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
        }
    }
}

pub fn view(model: &Model, ctx_group: &GroupContext, ctx_school:&SchoolContext)->Node<Msg>{
    div![
        C!{"columns"},
        div![
            C!{"column is-4"},
            "Toplam Ders saati: ", &model.activities.iter().fold(0, |acc, a| acc+a.hour).to_string(),
            hr![],
            table![
                C!{"table table-hover"},
                thead![
                    tr![
                        th![
                            attrs!{At::Scope=>"col"},
                            "Sınıflar"
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
                    &model.error,
                    model.activities.iter().map(|a|
                        tr![
                            match a.teacher{
                                Some(_ab)=>{
                                    style!{
                                    }
                                }
                                None => {
                                    style!{
                                        St::BackgroundColor=>"gray"
                                    }
                                }
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
                            match a.teacher{
                                Some(_ab) =>{
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
                                }
                                None => {
                                    td![
                                        input![
                                            attrs!{At::Type=>"button", At::Class=>"button", At::Value=>"Aktar"},
                                            {
                                                let id = a.id;
                                                ev(Ev::Click, move |_event| {
                                                    Msg::PatchActivity(id.to_string())
                                                })
                                            }
                                        ]
                                    ]
                                }
                            }
                        ]
                    )
                ]
            ]
        ],
        div![
            C!{"column is-2"},
            style!{
                            //St::Width => "px;"
            },
            div![
                C!{"field"},
                select![
                    el_ref(&model.select),
                    attrs!{At::Name=>"classes", At::Multiple => true.as_at_value(), At::Size => "10"},
                    ctx_group.classes.iter().map(|c|
                        option![
                            attrs!{At::Value=>&c.id},
                            &c.kademe.to_string(), "/", &c.sube," Sınıfı"
                        ]
                    ),
                    input_ev(Ev::Change, Msg::ChangeActClass)
                ]
            ],
            div![
                C!{"field"},
                select![
                    C!{"select"},
                    attrs!{At::Name=>"subject"},
                    model.subjects.iter().map(|s|
                        option![
                            attrs!{At::Value=>&s.id},
                            &s.name, "(", &s.kademe.to_string(), ")", if s.optional{" Seçmeli"} else{""}
                        ]
                    ),
                    input_ev(Ev::Change, Msg::ChangeActSubject)
                ]
            ],
            div![
                C!{"field"},
                input![
                    attrs!{At::Type=>"text", At::Name=>"hour"},
                    input_ev(Ev::Change, Msg::ChangeActHour)
                ]
            ],
            div![
                C!{"field"},
                input![
                    attrs!{At::Type=>"button", At::Class=>"button", At::Value=>"Ekle"},
                    ev(Ev::Click, |event| {
                        event.prevent_default();
                            Msg::SubmitActivity
                        })
                ]
            ]
        ],
        /*div![
            C!{"column is-2"},
            ctx_school.teachers.iter().map(|t|
                &t.first_name
            )
        ]*/
    ]
}