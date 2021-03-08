use serde::*;
use seed::{*, prelude::*};
use crate::{Context, createPDF, class_print, model};
use crate::page::school::detail;
use crate::page::school::group::test_generate;
use crate::model::timetable::Day;
use crate::model::teacher::TeacherTimetable;
use crate::model::class::{ClassTimetable, ClassTimetableActivity};
use crate::model::activity::{ActivityTeacher, Subject};
use crate::page::school::detail::{SchoolContext, GroupContext};
use crate::model::group::Schedule;
use crate::page::school::group::generate;


#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthUser{
    pub id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub username: Option<String>,
    pub is_admin: bool,
}

#[derive(Debug, Default, Clone)]
pub struct Model{
    params: Params,
    days: Vec<Day>,
    pub generating: bool,
    pub total_hour: i32,
    //pub test_async: i32,
    //pub stream_handler: Option<CmdHandle>,
    pub clean_tat: Vec<model::teacher::TeacherAvailableForTimetables>,
    pub clean_cat: Vec<ClassAvailable>,
    pub teacher_available: model::teacher::TeacherAvailableForTimetables,
    pub teacher_timetables: Vec<TeacherTimetable>,
    pub class_timetables: Vec<ClassTimetable>,
    pub data: TimetableData,
    pub test: test_generate::Tests,
    subjects: Vec<Subject>,
    schedules: Vec<Vec<Schedule>>,
    pub error: String
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
struct Params{
    hour: i32,
    depth: usize,
    depth2: usize
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct NewClassTimetable {
    pub class_id: Option<i32>,
    pub day_id: Option<i32>,
    pub hour: Option<i16>,
    pub activities: Option<i32>
}



#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct Activity{
    pub(crate) id: i32,
    pub(crate) subject: i32,
    pub(crate) teacher: Option<i32>,
    //pub(crate) class: i32,
    pub(crate) hour: i16,
    pub(crate) split: bool,
    pub(crate) classes: Vec<i32>,
}
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Class{
    pub id: i32,
    pub kademe: String,
    pub sube: String,
    pub school: i32,
    pub teacher: Option<i32>,
}
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct TimetableData{
    pub(crate) tat: Vec<model::teacher::TeacherAvailableForTimetables>,
    pub(crate) cat: Vec<ClassAvailable>,
    pub(crate) acts: Vec<Activity>,
    classes: Vec<Class>,
    teachers: Vec<model::teacher::Teacher>,
    timetables: Vec<NewClassTimetable>
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ClassAvailable{
    pub(crate) class_id: i32,
    pub(crate) day: i32,
    pub(crate) hours: Vec<bool>
}
#[derive(Debug)]
pub enum Msg{
    Submit,
    DeleteSubmit,
    SubmitTest,
    Generate,
    Stop,
    Save,
    PrintTeacher,
    PrintClass,
    HourChanged(String),
    DepthChanged(String),
    DepthChanged2(String),
    FetchDays(fetch::Result<Vec<Day>>),
    FetchSubjects(fetch::Result<Vec<Subject>>),
    FetchData(fetch::Result<TimetableData>),
    FetchSchedules(fetch::Result<Vec<Schedule>>),
}
pub fn init(orders: &mut impl Orders<Msg>, ctx_school: &detail::SchoolContext, ctx_group: &GroupContext)-> Model{
    orders.perform_cmd({
        let adres = format!("/api/days");
        let request = Request::new(adres)
            .method(Method::Get);
        async { Msg::FetchDays(async {
            request
                .fetch()
                .await?
                .check_status()?
                .json()
                .await
        }.await)}
    });
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
    orders.perform_cmd({
        let adres = format!("/api/schools/{}/groups/{}/timetables", ctx_school.school.id, ctx_group.group.id);
        let request = Request::new(adres)
            .method(Method::Get);
        async { Msg::FetchData(async {
            request
                .fetch()
                .await?
                .check_status()?
                .json()
                .await
        }.await)}
    });
    orders.perform_cmd({
        let url = format!("/api/schools/{}/groups/{}/schedules", ctx_school.school.id, ctx_group.group.id);
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
    let mut model = Model::default();
    model.generating = false;
    model.params.hour = 2;
    model.params.depth2 = 4;
    model.params.depth = 8;
    model
}
pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>, _ctx: &mut Context, _ctx_school: &mut SchoolContext, ctx_group: &mut GroupContext) {
    match msg{
        Msg::Submit => {
            model.generating = true;
            model.data.acts.sort_by(|a, b| b.hour.cmp(&a.hour));
            orders.perform_cmd(cmds::timeout(200, || Msg::Generate));
        }
        Msg::FetchSchedules(schedules) => {
            match schedules{
                Ok(s) => {
                    //log!(&s[0]);
                    model.schedules.push(s);
                }
                Err(_) => {}
            }
        }
        Msg::DeleteSubmit => {
            model.data.tat = model.clean_tat.clone();
            model.data.cat = model.clean_cat.clone();
            model.data.timetables = vec![];
        }
        Msg::FetchDays(days) => {
            match days{
                Ok(d) => model.days=d,
                Err(_) =>{
                    log!("hata days");
                }
            }
        }
        Msg::FetchSubjects(subjects) => {
            match subjects{
                Ok(s) => model.subjects = s,
                _ => {
                    log!("hata subjects");
                }
            }
        }
        Msg::SubmitTest=>{
            let test = &mut model.test;
            let acts = model.data.acts.clone();
            let timetables = model.data.timetables.clone();
            //model.clean_cat = model.data.cat.clone();
            //model.clean_tat = model.data.tat.clone();
            //model.total_hour = model.data.acts.iter().fold(0, |a,b| if b.teacher.is_some(){(b.hour) as i32 + (a)}else{0});
            let acts2: Vec<Activity> = acts.iter().cloned()
                .filter(|a| a.teacher.is_some() && !timetables.iter().cloned()
                    .any(|t| a.id == t.activities.unwrap())).collect();
            for tt in &model.data.timetables{
                let class_lim = model.data.cat.iter().cloned().enumerate().find(|cl| cl.1.class_id==tt.class_id.unwrap() && cl.1.day == tt.day_id.unwrap()).unwrap();
                model.data.cat[class_lim.0].hours[tt.hour.unwrap() as usize]=false;
                let _act = model.data.acts.iter().find(|a| a.id == tt.activities.unwrap()).unwrap();
                let _teacher_lim = model.data.tat.iter().cloned().enumerate().find(|tl| tl.1.day == tt.day_id.unwrap() && _act.teacher.is_some() && tl.1.user_id == _act.teacher.unwrap()).unwrap();
                model.data.tat[_teacher_lim.0].hours[tt.hour.unwrap() as usize] = false;
            }
            test_generate::tests(&acts2, model.params.hour, test, _ctx_school, ctx_group, &model.data.tat, &model.data.cat)
        }
        Msg::DepthChanged(d)=>{
            match d.parse::<usize>(){
                Ok(h)=>{
                    model.params.depth = h;
                }
                Err(_)=>{}
            }
        }
        Msg::DepthChanged2(d)=>{
            match d.parse::<usize>(){
                Ok(h)=>{
                    model.params.depth2 = h;
                }
                Err(_)=>{}
            }
        }
        Msg::Generate=> {
            //let acts: Vec<Activity> = model.data.acts.clone().into_iter().filter(|a| ctx_group.classes.iter().any(|c| a.class == c.id)).collect();
            let acts: Vec<Activity> = model.data.acts.clone();
            let mut timetables: Vec<NewClassTimetable> = model.data.timetables.clone().into_iter().filter(|tt| model.data.acts.iter().any(|a| a.id == tt.activities.unwrap())).collect();

            let cat = &mut model.data.cat;
            let tat = &mut model.data.tat;
            let error = &mut model.error;
            //let timetables = &mut model.data.timetables;
            if model.params.hour >= ctx_group.group.hour {
                model.params.hour = ctx_group.group.hour-1  ;
            }
            if model.params.hour < 2 {
                model.params.hour = 2;
            }

            let result = generate::generate(model.params.hour, model.params.depth, model.params.depth2, tat, cat, &acts, &mut timetables, &model.clean_tat, error);
            model.data.timetables = timetables.clone();
            if result{
                let acts2: Vec<Activity> = acts.iter().cloned()
                    .filter(|a| a.teacher.is_some() && !timetables.iter().cloned()
                        .any(|t| a.id == t.activities.unwrap())).collect();
                let mut not_placed = 0;
                for a in acts2{
                    not_placed += a.hour;
                }
                if (timetables.len()+(not_placed as usize)) != model.total_hour as usize{
                    orders.send_msg(Msg::Stop);
                }
                else if model.generating{
                    let mut second = 0;
                    if (model.total_hour as usize - timetables.len()) < 50 && model.data.acts.len() > 100{
                        second = 80;
                    }
                    orders.perform_cmd(cmds::timeout(second, || Msg::Generate));

                }
            }
            else {
                model.generating = false;
                orders.send_msg(Msg::Stop);
            }
        }
        Msg::PrintTeacher => {
            let mut timetables: Vec<(String, String, Vec<TeacherTimetable>)>=Vec::new();
            let acts: Vec<Activity> = model.data.acts.clone();
            let timetables2: Vec<NewClassTimetable> = model.data.timetables.clone().into_iter().filter(|tt| acts.iter().any(|a| a.id == tt.activities.unwrap())).collect();
            for t in &_ctx_school.teachers{
                let mut teacher_print: Vec<TeacherTimetable>=Vec::new();
                let teacher_timetables: Vec<NewClassTimetable> = timetables2.iter().cloned()
                    .filter(|tt| acts.clone().iter().any(|a| a.teacher.is_some() && a.teacher == Some(t.id) && tt.activities.unwrap() == a.id )).collect();
                for tt in teacher_timetables{
                    //let class = _ctx_school.classes.iter().find(|c| c.id == tt.class_id.unwrap()).unwrap();
                    let act = acts.clone().into_iter().find(|a| a.id == tt.activities.unwrap()).unwrap();
                    let other_class: Vec<model::class::Class> = ctx_group.classes.clone().into_iter().filter(|c| c.id == tt.class_id.unwrap() || act.classes.clone().into_iter().any(|ca| ca == c.id)).collect();
                    //other_class.push(class.clone());
                    let subject = model.subjects.iter().find(|s| s.id == act.subject).unwrap();
                    let timetable = TeacherTimetable{
                        id: tt.activities.unwrap(),
                        class_id: other_class,
                        day_id: tt.day_id.unwrap(),
                        hour: tt.hour.unwrap(),
                        subject: subject.name.clone()
                    };
                    teacher_print.push(timetable)
                }
                if teacher_print.len() > 0 {
                    timetables.push((t.first_name.clone(), t.last_name.clone(), teacher_print));
                }

            }

            let timetables = serde_json::to_string(&timetables).unwrap();
            let days = serde_json::to_string(&model.days).unwrap();
            createPDF(&timetables, &days, 0, (ctx_group.group.hour-1) as i16, &serde_json::to_string(&_ctx_school.school).unwrap(), &serde_json::to_string(&model.schedules).unwrap())
        }
        Msg::PrintClass => {
            let mut timetables: Vec<(String, String, Vec<ClassTimetable>)>=Vec::new();
            for c in &ctx_group.classes{
                let mut c_print: Vec<ClassTimetable>=Vec::new();
                let acts: Vec<Activity> = model.data.acts.clone().into_iter().filter(|a| a.classes.iter().any(|c2| c2 == &c.id)).collect();
                let class_timetables: Vec<NewClassTimetable> = model.data.timetables.iter().cloned()
                    .filter(|tt| acts.iter().any(|a| a.id == tt.activities.unwrap())).collect();
                for tt in &class_timetables{
                    let act = acts.iter().find(|a| a.id == tt.activities.unwrap() && a.teacher.is_some()).unwrap();
                    let teacher = _ctx_school.teachers.iter().find(|t| act.teacher.is_some() && act.teacher.unwrap() == t.id);
                    match teacher{
                        Some(t) => {
                            let subject = model.subjects.iter().find(|s| s.id == act.subject).unwrap();
                            let timetable = ClassTimetable{
                                id: 0,
                                class_id: c.id,
                                day_id: tt.day_id.unwrap(),
                                hour: tt.hour.unwrap(),
                                subject: subject.name.clone(),
                                activity: ClassTimetableActivity{ id: 0, teacher: ActivityTeacher{
                                    id: 0,
                                    first_name: t.first_name.clone(),
                                    last_name: t.last_name.clone()
                                } }
                            };
                            c_print.push(timetable)
                        }
                        None => {

                        }
                    }

                }
                if c_print.len() > 0{
                    timetables.push((c.kademe.clone(), c.sube.to_string(), c_print));
                }
            }
            let last_hour = ctx_group.group.hour-1;
            let timetables = serde_json::to_string(&timetables).unwrap();
            let days = serde_json::to_string(&model.days).unwrap();
            class_print(&timetables, &days, last_hour as i16, &serde_json::to_string(&_ctx_school.school).unwrap(), &serde_json::to_string(&model.schedules).unwrap())
        }
        Msg::HourChanged(hour)=>{

            match hour.parse::<i32>(){
                Ok(h)=>{
                    if h >= 2 {
                        model.params.hour = h;
                    }
                }
                Err(_)=>{}
            }

        }
        Msg::FetchData(Ok(d))=>{
            model.total_hour = 0;
            model.data= d;
            model.clean_cat = model.data.cat.clone();
            model.clean_tat = model.data.tat.clone();
            //let acts: Vec<Activity> = model.data.acts.clone().into_iter().filter(|a| ctx_group.classes.iter().any(|c| a.classes.iter().any(|ac| ac == c.id))).collect();
            for act in &model.data.acts{
                match act.teacher{
                    Some(_t) => {

                        model.total_hour += act.hour as i32;
                    }
                    None => {}
                }
            }
            
            //model.total_hour = model.data.acts.iter().fold(0, |a,b| if b.teacher.is_some(){(b.hour) as i32 + (a)}else{0});
            if model.data.timetables.iter().any(|tt| model.data.cat.iter().any(|c| (tt.hour.unwrap() as usize ) >= c.hours.len())  ||
                    model.data.tat.iter().any(|t| (tt.hour.unwrap() as usize ) >= t.hours.len())){
                //log!(model.data.cat.iter().any(|c| c.hours.len() < 25));
                model.data.timetables.clear();
            }
            for tt in &model.data.timetables{
                let _act = model.data.acts.iter().find(|a| a.id == tt.activities.unwrap());
                let class_lim = model.data.cat.iter().cloned().enumerate().find(|cl| _act.unwrap().classes.iter().any(|c| c == &cl.1.class_id) && cl.1.day == tt.day_id.unwrap()).unwrap();
                model.data.cat[class_lim.0].hours[tt.hour.unwrap() as usize]=false;
                let _act = model.data.acts.iter().find(|a| a.id == tt.activities.unwrap());
                match _act{
                    Some(_a) =>{
                        //log!(&_a);
                        let _teacher_lim = model.data.tat.iter().cloned().enumerate().find(|tl| tl.1.day == tt.day_id.unwrap() && _a.teacher.is_some() && tl.1.user_id == _a.teacher.unwrap());
                        match _teacher_lim{
                            Some(t) => {model.data.tat[t.0].hours[tt.hour.unwrap() as usize] = false;}
                            None => {
                                //model.data.acts.retain(|a| a.id != _a.id);
                            }
                        }

                    }
                    None => {}
                }

            }
        }
        Msg::FetchData(Err(_))=>{
            //log!("Hata");
        }

        Msg::Stop=>{
            model.generating = false;
        }
        Msg::Save=>{
            for t in &mut model.data.timetables{
                if t.activities.unwrap() < 0{
                    t.activities = Some(t.activities.unwrap()*-1);
                }
            }
            orders.perform_cmd({
                let adres = format!("/api/schools/{}/groups/{}/timetables", _ctx_school.school.id, &ctx_group.group.id);
                let request = Request::new(adres)
                    .method(Method::Post)
                    .json(&model.data.timetables);
                async { Msg::FetchData(async {
                    request?
                        .fetch()
                        .await?
                        .check_status()?
                        .json()
                        .await
                }.await)}
            });
        }
    }
}

pub fn view(model: &Model, ctx: &Context, ctx_school: &SchoolContext)-> Node<Msg>{
    div![
        C!{"columns"},
            div![
                C!{"column is-6"},
                div![
                    C!{"field"},
                    label![C!{"label"}, "Bir öğretmenin bir sınıfa verebileceği günlük maksimum ders sayısı"],
                    p![C!{"control has-icons-left"},
                        input![C!{"input"},
                            attrs!{
                                At::Type=>"text",
                                At::Placeholder=>"Bir öğretmenin bir sınıfa verebileceği günlük maksimum ders sayısı",
                                At::Value => &model.params.hour,
                                At::Disabled => disabled(ctx, ctx_school).as_at_value()
                            },
                            input_ev(Ev::Input, Msg::HourChanged),
                        ],
                    ]
                ],
                div![C!{"field"},
                    label![C!{"label"}, "Derinlik"],
                    p![C!{"control has-icons-left"},
                        input![C!{"input"},
                            attrs!{
                                At::Type=>"text",
                                At::Placeholder=>"Derinlik",
                                At::Name=>"depth",
                                At::Id=>"depth"
                                At::Value => &model.params.depth,
                                At::Disabled => true.as_at_value()
                            },
                            input_ev(Ev::Input, Msg::DepthChanged),
                        ],
                    ]
                ],
                div![C!{"field"},
                    label![C!{"label"}, "Derinlik"],
                    p![C!{"control has-icons-left"},
                        input![C!{"input"},
                            attrs!{
                                At::Type=>"text",
                                At::Placeholder=>"Derinlik 2",
                                At::Name=>"depth",
                                At::Id=>"depth"
                                At::Value => &model.params.depth2,
                                At::Disabled => true.as_at_value()
                            },
                            input_ev(Ev::Input, Msg::DepthChanged2),
                        ],
                    ]
                ],
                div![C!{"field"},
                    p![C!{"control has-icons-left"},
                        generate(model)
                    ],
                    input![C!{"button is-secondary"},
                        attrs!{
                            At::Type=>"button",
                            At::Value=>"Sil",
                            At::Id=>"login_button",
                            At::Disabled => disabled(ctx, ctx_school).as_at_value()
                        },
                        ev(Ev::Click, |event| {
                            event.prevent_default();
                            Msg::DeleteSubmit
                        })
                    ],
                    input![C!{"button is-secondary"},
                        attrs!{
                            At::Type=>"button",
                            At::Value=>"Test Et",
                            At::Id=>"login_button",
                            At::Disabled => disabled(ctx, ctx_school).as_at_value()
                        },
                        ev(Ev::Click, |event| {
                            event.prevent_default();
                            Msg::SubmitTest
                        })
                    ],
                    input![C!{"button is-secondary"},
                        attrs!{
                            At::Type=>"button",
                            At::Value=>"Kaydet",
                            At::Id=>"login_button",
                            At::Disabled => disabled(ctx, ctx_school).as_at_value()
                        },
                        ev(Ev::Click, |event| {
                            event.prevent_default();
                            Msg::Save
                        })
                    ],
                ],
                div![
                    " Yerleştirilen saat: ",&model.data.timetables.len(),
                    " Toplam ders saati: ", &model.total_hour.to_string(),
                    //&model.test_async.to_string(),
                    p![
                        " Test1:",br![],
                        model.test.activity.iter().map(|a|
                            div![&a.teacher.first_name, " ", &a.teacher.last_name, " adlı öğretmenin ", &a.class.kademe.to_string(), "/", &a.class.sube, " sınıfına ait aktivitesinin saat sayısı büyük.", br![]]
                        )
                    ],
                    p![
                        " Test2:",br![],
                        model.test.teachers.iter().map(|t|
                            div![&t.first_name, " ", &t.last_name, " adlı öğretmene atanan ders sayısı, boş saatlerinden daha fazla. Öğretmenin kısıtlamasını kontrol edin.", br![]]
                        )
                    ],
                    p![
                        " Test3:",br![],
                        model.test.classes.iter().map(|c|
                            div![&c.kademe.to_string(), "/", &c.sube, " sınıfına atanan ders sayısı, boş saatlerinden daha fazla. Sınıfın kısıtlamasını kontrol edin.", br![]]
                        )
                    ],
                    /*p![
                        " Test4:",br![],
                        model.test.test4.iter().map(|t|
                            div![&t.first_name, " ", &t.last_name, " adlı öğretmenin programının yerleştirilmesi zaman alıyor.", br![]]
                        )
                    ],
                    p![
                        " Test5:",br![],
                        model.test.test5.iter().map(|c|
                            div![&c.kademe.to_string(), "/", &c.sube, " sınıfının programının yerleştirilmesi zaman alıyor.", br![]]
                        )
                    ],*/
                ]
            ],
            div![
                C!{"column is-3"},
                input![C!{"button is-secondary"},
                    attrs!{
                        At::Type=>"button",
                        At::Value=>"Yazdır(Öğretmenler)",
                        At::Id=>"login_button",
                        At::Disabled => disabled(ctx, ctx_school).as_at_value()
                    },
                    ev(Ev::Click, |event| {
                        event.prevent_default();
                        Msg::PrintTeacher
                    })
                ],
                input![C!{"button is-secondary"},
                    attrs!{
                        At::Type=>"button",
                        At::Value=>"Yazdır(Sınıflar)",
                        At::Id=>"login_button",
                        At::Disabled => disabled(ctx, ctx_school).as_at_value()
                    },
                    ev(Ev::Click, |event| {
                        event.prevent_default();
                        Msg::PrintClass
                    })
                ]
            ]

    ]
}
fn disabled(ctx: &Context, ctx_school: &SchoolContext) -> bool {
    if ctx.user.is_none(){
        return true
    }
    else if ctx.user.as_ref().unwrap().is_admin {
        return false
    }
    else if ctx.school.iter().any(|s| s.id == ctx_school.school.id) {
        return false
    }
    else {
        return true
    }
}
fn generate(model: &Model)->Node<Msg>{
    if !model.generating{
        input![C!{"button is-primary"},
            attrs!{
                At::Type=>"button",
                At::Value=>"Başlat",
                At::Id=>"login_button"
            },
            ev(Ev::Click, |event| {
                event.prevent_default();
                Msg::Submit
            })
        ]
    }
    else{
        input![C!{"button is-primary"},
            attrs!{
                At::Type=>"button",
                At::Value=>"Durdur",
                At::Id=>"login_button"
            },
            ev(Ev::Click, |event| {
                event.prevent_default();
                Msg::Stop
            })
        ]
    }
}