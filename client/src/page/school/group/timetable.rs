use serde::*;
use seed::{*, prelude::*};
use crate::{Context, createPDF, class_print, model};
use crate::page::school::detail;
use crate::page::school::group::test_generate;
use crate::model::timetable::Day;
use crate::model::teacher::{TeacherTimetable, TeacherAvailableForTimetables};
use crate::model::class::{ClassTimetable, ClassTimetableActivity};
use crate::page::school::detail::{SchoolContext};
use crate::model::group::Schedule;
use crate::page::school::group::generate;
use crate::model::{teacher, subject};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AuthUser{
    pub id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub username: Option<String>,
    pub is_admin: bool,
}

#[derive(Default, Clone)]
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
    subjects: Vec<subject::Subject>,
    schedules: Vec<Vec<Schedule>>,
    pub error: String,
    pub url: Url
}

#[derive(Serialize, Deserialize, Default, Clone)]
struct Params{
    hour: i32,
    depth: usize,
    depth2: usize
}

#[derive(Serialize, Deserialize, Clone, Default, PartialEq)]
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
#[derive(Clone, Serialize, Deserialize, Default)]
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
#[derive()]
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
    FetchSubjects(fetch::Result<Vec<subject::Subject>>),
    FetchData(fetch::Result<TimetableData>),
    FetchSchedules(fetch::Result<Vec<Schedule>>),
}
pub fn init(url: Url, orders: &mut impl Orders<Msg>, ctx_school: &detail::SchoolContext)-> Model{
    orders.perform_cmd({
        let adres = "/api/days".to_string();
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
        let adres = format!("/api/schools/{}/groups/{}/timetables", ctx_school.school.id, &url.path()[3]);
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
        let url = format!("/api/schools/{}/groups/{}/schedules", ctx_school.school.id, &url.path()[3]);
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
    let model = Model{
        generating: false, url, params: Params{
            hour: 8,
            depth: 15,
            depth2: 2
        }, ..Default::default()
    };
    model
}
pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>, school_ctx: &mut SchoolContext) {
    match msg{
        Msg::Submit => {
            model.generating = true;
            //model.data.acts.shuffle(&mut thread_rng());
            //model.data.acts.sort_by(|a, b| b.hour.cmp(&a.hour));
            orders.perform_cmd(cmds::timeout(100, || Msg::Generate));
        }
        Msg::FetchSchedules(schedules) => {
            if let Ok(s) = schedules {
                model.schedules.push(s);
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
            test_generate::tests(&acts2, model.params.hour, test, school_ctx, &model.data.tat, &model.data.cat, model.url.clone())
        }
        Msg::DepthChanged(d)=>{
            if let Ok(h) = d.parse::<usize>() {
                model.params.depth = h;
            }
        }
        Msg::DepthChanged2(d)=>{
            if let Ok(h) = d.parse::<usize>() {
                model.params.depth2 = h;
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
            let group_ctx = school_ctx.get_group(&model.url);
            if model.params.hour >= group_ctx.group.hour {
                model.params.hour = group_ctx.group.hour  ;
            }
            if model.params.hour < 2 {
                model.params.hour = 2;
            }

            let result = generate::generate(model.params.hour, model.params.depth, model.params.depth2, tat, cat, &acts, &mut timetables, &model.clean_tat, error);
            model.data.timetables = timetables.clone();
            if result{
                let _acts2: Vec<Activity> = acts.iter().cloned()
                    .filter(|a| a.teacher.is_some() && !timetables.iter().cloned()
                        .any(|t| a.id == t.activities.unwrap())).collect();
                /*
                let mut not_placed = 0;
                for a in acts2{
                    not_placed += a.hour;
                }
                */
                if model.generating {
                    let mut second = 0;
                    if (model.total_hour as usize - timetables.len()) < 50 && model.data.acts.len() > 100 {
                        second = 1000;
                    }
                    orders.perform_cmd(cmds::timeout(1, || Msg::Generate));
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
            if let Some(teachers) = &school_ctx.teachers{
                for t in teachers{
                    let mut teacher_print: Vec<TeacherTimetable>=Vec::new();
                    let teacher_timetables: Vec<NewClassTimetable> = timetables2.iter().cloned()
                        .filter(|tt| acts.clone().iter().any(|a| a.teacher.is_some() && a.teacher == Some(t.teacher.id) && tt.activities.unwrap() == a.id )).collect();
                    for tt in teacher_timetables{
                        //let class = school_ctx.classes.iter().find(|c| c.id == tt.class_id.unwrap()).unwrap();
                        let group_ctx = school_ctx.get_group(&model.url);
                        let act = acts.clone().into_iter().find(|a| a.id == tt.activities.unwrap()).unwrap();
                        let other_class: Vec<model::class::ClassContext> = group_ctx.classes.clone().unwrap().into_iter().filter(|c| c.class.id == tt.class_id.unwrap() || act.classes.clone().into_iter().any(|ca| ca == c.class.id)).collect();
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
                    if !teacher_print.is_empty() {
                        timetables.push((t.teacher.first_name.clone(), t.teacher.last_name.clone(), teacher_print));
                    }
                }
            }
            let group_ctx = school_ctx.get_group(&model.url);
            let timetables = serde_json::to_string(&timetables).unwrap();
            let days = serde_json::to_string(&model.days).unwrap();
            createPDF(&timetables, &days, 0, (group_ctx.group.hour-1) as i16, &serde_json::to_string(&school_ctx.school).unwrap(), &serde_json::to_string(&model.schedules).unwrap())
        }
        Msg::PrintClass => {
            let mut timetables: Vec<(String, String, Vec<ClassTimetable>)>=Vec::new();
            let group_ctx = school_ctx.get_group(&model.url);
            if let Some(classes) = &group_ctx.classes{
                for c in classes{
                    let mut c_print: Vec<ClassTimetable>=Vec::new();
                    let acts: Vec<Activity> = model.data.acts.clone().into_iter().filter(|a| a.classes.iter().any(|c2| c2 == &c.class.id)).collect();
                    let class_timetables: Vec<NewClassTimetable> = model.data.timetables.iter().cloned()
                        .filter(|tt| acts.iter().any(|a| a.id == tt.activities.unwrap())).collect();
                    for tt in &class_timetables{
                        let act = acts.iter().find(|a| a.id == tt.activities.unwrap() && a.teacher.is_some()).unwrap();
                        if let Some(teachers) = &school_ctx.teachers{
                            let teacher = teachers.iter().find(|t| act.teacher.is_some() && act.teacher.unwrap() == t.teacher.id);
                            if let Some(t) = teacher {
                                let subject = model.subjects.iter().find(|s| s.id == act.subject).unwrap();
                                let timetable = ClassTimetable {
                                    id: 0,
                                    class_id: c.class.id,
                                    day_id: tt.day_id.unwrap(),
                                    hour: tt.hour.unwrap(),
                                    subject: subject.name.clone(),
                                    activity: ClassTimetableActivity {
                                        id: 0,
                                        teacher: teacher::Teacher {
                                            id: 0,
                                            first_name: t.teacher.first_name.clone(),
                                            last_name: t.teacher.last_name.clone(),
                                            role_id: 0,
                                            role_name: "".to_string(),
                                            is_active: false,
                                            email: None,
                                            tel: None
                                        }
                                    }
                                };
                                c_print.push(timetable)
                            }
                        }

                    }
                    if !c_print.is_empty(){
                        timetables.push((c.class.kademe.clone(), c.class.sube.to_string(), c_print));
                    }
                }
            }

            let last_hour = group_ctx.group.hour-1;
            let timetables = serde_json::to_string(&timetables).unwrap();
            let days = serde_json::to_string(&model.days).unwrap();
            class_print(&timetables, &days, last_hour as i16, &serde_json::to_string(&school_ctx.school).unwrap(), &serde_json::to_string(&model.schedules).unwrap())
        }
        Msg::HourChanged(hour)=>{

            if let Ok(h) = hour.parse::<i32>() {
                if h >= 2 {
                    model.params.hour = h;
                }
            }

        }
        Msg::FetchData(Ok(d))=>{
            model.total_hour = 0;
            model.data= d;
            model.clean_cat = model.data.cat.clone();
            model.clean_tat = model.data.tat.clone();
            if let Some(teachers) = &school_ctx.teachers{
                for t in teachers {
                    let limitations = model.clean_tat.clone().into_iter().filter(|l| l.user_id == t.teacher.id).collect::<Vec<TeacherAvailableForTimetables>>();
                    let acts = model.data.acts.clone().into_iter().filter(|a| a.teacher.unwrap() == t.teacher.id).collect::<Vec<Activity>>();
                    for l in limitations {
                        for h in l.hours.iter().enumerate() {
                            if !*h.1 {
                                let tt = model.data.timetables.clone().into_iter().enumerate()
                                    .filter(|tt2| tt2.1.day_id.unwrap() == l.day && tt2.1.hour.unwrap() == h.0 as i16 &&
                                        acts.iter().any(|a| a.id == tt2.1.activities.unwrap())).collect::<Vec<(usize, NewClassTimetable)>>();
                                if tt.len() == 1 {
                                    model.data.timetables.remove(tt[0].0);
                                }
                                //
                            }
                        }
                    }
                }
            }

            //let acts: Vec<Activity> = model.data.acts.clone().into_iter().filter(|a| ctx_group.classes.iter().any(|c| a.classes.iter().any(|ac| ac == c.id))).collect();
            for act in &model.data.acts{
                if let Some(_t) = act.teacher {
                    model.total_hour += act.hour as i32;
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
                if let Some(a) = _act {
                    let _teacher_lim = model.data.tat.iter().cloned().enumerate().find(|tl| tl.1.day == tt.day_id.unwrap() && a.teacher.is_some() && tl.1.user_id == a.teacher.unwrap());
                    if let Some(t) = _teacher_lim {
                        model.data.tat[t.0].hours[tt.hour.unwrap() as usize] = false;
                    }
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
                    t.activities = Some(-t.activities.unwrap());
                }
            }
            orders.perform_cmd({
                let adres = format!("/api/schools/{}/groups/{}/timetables", school_ctx.school.id, &model.url.path()[3]);
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
            if let Some(teachers) = &mut school_ctx.teachers{
                for t in teachers{
                    for g in &mut t.group{
                        g.timetables = None;
                    }
                }
            }
            let group = school_ctx.get_mut_group(&model.url);
            let classes = group.get_mut_classes();
            for c in classes{
                c.timetables = None
            }

        }
    }
}

pub fn view(model: &Model, ctx_school: &SchoolContext)-> Node<Msg>{
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
                                //At::Disabled => disabled(ctx, ctx_school).as_at_value()
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
                /*div![C!{"field"},
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
                ],*/
                div![C!{"field"},
                    p![C!{"control has-icons-left"},
                        generate(model)
                    ],
                    input![C!{"button is-secondary"},
                        attrs!{
                            At::Type=>"button",
                            At::Value=>"Sil",
                            At::Id=>"login_button",
                            //At::Disabled => disabled(ctx, ctx_school).as_at_value()
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
                            //At::Disabled => disabled(ctx, ctx_school).as_at_value()
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
                            //At::Disabled => disabled(ctx, ctx_school).as_at_value()
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
                        //At::Disabled => disabled(ctx, ctx_school).as_at_value()
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
                        //At::Disabled => disabled(ctx, ctx_school).as_at_value()
                    },
                    ev(Ev::Click, |event| {
                        event.prevent_default();
                        Msg::PrintClass
                    })
                ]
            ]

    ]
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