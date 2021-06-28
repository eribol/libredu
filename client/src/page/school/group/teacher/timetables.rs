use crate::model::{user, activity};
use seed::{*, prelude::*};
use crate::model::teacher;
use crate::model::timetable;
use crate::page::school::detail;
use crate::model::teacher::{TeacherTimetable, TeacherTimetable2};
use crate::model::group::GroupContext;

#[derive()]
pub enum Msg{
    FetchTimetable(fetch::Result<Vec<TeacherTimetable2>>),
    FetchActivities(fetch::Result<Vec<activity::FullActivity>>),
    FetchLimitation(fetch::Result<Vec<teacher::TeacherAvailable>>),
    ChangeHour((usize,usize)),
    ChangeAllHour(usize),
    ChangeAllDay(usize),
    Submit,
    Loading
}


#[derive(Default, Clone)]
pub struct Model{
    days: Vec<timetable::Day>,
    url: Url
}

pub fn init(url: Url, orders: &mut impl Orders<Msg>, _school_ctx: &mut detail::SchoolContext)-> Model{
    let model = Model{url: url.clone(), days: timetable::Day::new(), ..Default::default()};
    orders.send_msg(Msg::Loading);
    model
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>, school_ctx: &mut detail::SchoolContext) {
    let sc2 = school_ctx.clone();
    let hours = sc2.get_group(&model.url).group.hour;
    match msg {
        Msg::FetchTimetable(t) => {
            if let Ok(tt) = t {
                log!("tt yüklendi");
                let teacher_ctx = &mut school_ctx.get_mut_teacher(&model.url);
                let teacher_group = &mut teacher_ctx.group.iter_mut().find(|g| g.group == model.url.path()[3].parse::<i32>().unwrap());
                if let Some(tg) = teacher_group {
                    if let Some(timetables) = &mut tg.timetables {
                        *timetables = tt;
                    } else {
                        tg.timetables = Some(tt);
                    }
                }
            }
        }
        Msg::FetchActivities(acts) => {
            let teacher_ctx = &mut school_ctx.get_mut_teacher(&model.url);
            let teacher_group = &mut teacher_ctx.group.iter_mut().find(|g| g.group == model.url.path()[3].parse::<i32>().unwrap());
            if let Ok(a) = acts{
                log!("acts yüklendi");
                if let Some(tg) = teacher_group {
                    if let Some(acts) = &mut tg.activities {
                        *acts = a;
                    }
                    else {
                        tg.activities = Some(a);
                    }
                }
            }
        }
        Msg::FetchLimitation(json) => {
            let teacher_ctx = &mut school_ctx.get_mut_teacher(&model.url);
            let teacher_group = &mut teacher_ctx.group.iter_mut().find(|g| g.group == model.url.path()[3].parse::<i32>().unwrap());
            match json {
                Ok(mut l) => {
                    log!("lim yüklendi");
                    l.sort_by(|a, b| a.day.id.cmp(&b.day.id));
                    if let Some(tg) = teacher_group{
                        if let Some(lm) = &mut tg.limitations{
                            *lm = l;
                            let mut changed = false;
                            for d in model.days.iter() {
                                if !lm.iter().any(|ta| ta.day.id == d.id) {
                                    let hours = vec![true; hours as usize];
                                    lm.push(teacher::TeacherAvailable { day: d.clone(), hours, group_id: None });
                                    changed = true;
                                }
                                else if lm[(d.id - 1) as usize].hours.len() != hours as usize {
                                    let hours = vec![true; hours as usize];
                                    lm[(d.id - 1) as usize].hours = hours;
                                    changed = true;
                                }
                            }
                            if changed{
                                orders.perform_cmd({
                                    let url = format!("/api/schools/{}/groups/{}/teachers/{}/limitations", model.url.path()[1], model.url.path()[3], model.url.path()[5]);
                                    let request = Request::new(url)
                                        .method(Method::Post)
                                        .json(&lm);
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
                        else {
                            tg.limitations = Some(l);
                        }
                    }
                }
                Err(_)=>{
                    if let Some(tg) = teacher_group {
                        if let Some(lm) = &mut tg.limitations {
                            if lm.is_empty() {
                                for d in model.days.iter() {
                                    if !lm.iter().any(|ta| ta.day.id == d.id) {
                                        let hours = vec![true; hours as usize];
                                        lm.push(teacher::TeacherAvailable { day: d.clone(), hours, group_id: None })
                                    }
                                    else if lm[(d.id - 1) as usize].hours.len() != hours as usize {
                                        let hours = vec![true; hours as usize];
                                        lm[(d.id - 1) as usize].hours = hours;
                                    }
                                }
                                orders.perform_cmd({
                                    let url = format!("/api/schools/{}/teachers/{}/limitations/{}", model.url.path()[1], model.url.path()[5], model.url.path()[3]);
                                    let request = Request::new(url)
                                        .method(Method::Post)
                                        .json(&lm);
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
            }
        }
        Msg::ChangeHour(ids) => {
            let teacher_ctx = &mut school_ctx.get_mut_teacher(&model.url);
            let teacher_group = &mut teacher_ctx.group.iter_mut().find(|g| g.group == model.url.path()[3].parse::<i32>().unwrap());
            if let Some(tg) = teacher_group {
                if let Some(lm) = &mut tg.limitations {
                    if lm[ids.0].hours[ids.1] {
                        lm[ids.0].hours[ids.1] = false;
                    } else {
                        lm[ids.0].hours[ids.1] = true;
                    }
                }
            }
        }
        Msg::ChangeAllHour(index) => {
            let teacher_ctx = &mut school_ctx.get_mut_teacher(&model.url);
            let teacher_group = &mut teacher_ctx.group.iter_mut().find(|g| g.group == model.url.path()[3].parse::<i32>().unwrap());
            if let Some(tg) = teacher_group {
                if let Some(lm) = &mut tg.limitations {
                    let mut all = true;
                    for l in &*lm{
                        if !l.hours[index]{
                            all = false;
                            break;
                        }
                    }
                    if all{
                        for d in 0..7{
                            lm[d].hours[index] = false;
                        }
                    }
                    else {
                        for d in 0..7{
                            lm[d].hours[index] = true;
                        }
                    }
                }
            }

        }
        Msg::ChangeAllDay(index) => {
            let teacher_ctx = &mut school_ctx.get_mut_teacher(&model.url);
            let teacher_group = &mut teacher_ctx.group.iter_mut().find(|g| g.group == model.url.path()[3].parse::<i32>().unwrap());
            if let Some(tg) = teacher_group {
                if let Some(lm) = &mut tg.limitations {
                    if lm[index].hours.iter().any(|h| !*h){
                        for h in 0..hours as usize{
                            lm[index].hours[h as usize] = true;
                        }
                    }
                    else {
                        for h in 0..hours as usize{
                            lm[index].hours[h as usize] = false;
                        }
                    }
                }
            }

        }
        Msg::Submit =>{
            let teacher_ctx = &mut school_ctx.get_mut_teacher(&model.url);
            let teacher_group = &mut teacher_ctx.group.iter_mut().find(|g| g.group == model.url.path()[3].parse::<i32>().unwrap());
            if let Some(tg) = teacher_group {
                if let Some(lm) = &mut tg.limitations {
                    orders.perform_cmd({
                        let url = format!("/api/schools/{}/groups/{}/teachers/{}/limitations", model.url.path()[1], model.url.path()[3], &model.url.path()[5]);
                        let request = Request::new(url)
                            .method(Method::Post)
                            .json(&lm);
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
        Msg::Loading => {
            let teacher_ctx = &mut school_ctx.get_mut_teacher(&model.url);
            let teacher_group = &mut teacher_ctx.group.iter_mut().find(|g| g.group == model.url.path()[3].parse::<i32>().unwrap());
            if let Some(tg) = teacher_group {
                if tg.activities.is_none() {
                    orders.perform_cmd({
                        let url = format!("/api/schools/{}/groups/{}/teachers/{}/activities", model.url.path()[1], model.url.path()[3], model.url.path()[5]);
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
                if tg.timetables.is_none() {
                    orders.perform_cmd({
                        let url = format!("/api/schools/{}/groups/{}/teachers/{}/timetables", model.url.path()[1], model.url.path()[3], model.url.path()[5]);
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
                if tg.limitations.is_none() {
                    orders.perform_cmd({
                        let url = format!("/api/schools/{}/groups/{}/teachers/{}/limitations", model.url.path()[1], model.url.path()[3], model.url.path()[5]);
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
            }
        }
    }
}

pub fn view(model: &Model, school_ctx: &detail::SchoolContext)->Node<Msg>{
    div![
        Script![
            attrs!{
                At::Src=>"/static/js/print_teachers.js",
                At::Type=>"module"
            }
        ],
        timetable(model, school_ctx)
    ]

}


fn timetable(model: &Model, school_ctx: &detail::SchoolContext)->Node<Msg>{
    let group_ctx = school_ctx.get_group(&model.url);
    let hours = group_ctx.group.hour;
    let teacher_ctx = school_ctx.get_teacher(&model.url);
    let teacher_group = teacher_ctx.group.iter().find(|g| g.group == model.url.path()[3].parse::<i32>().unwrap()).unwrap();
    div![
        C!{"column is-12"},

        table![
            C!{"table is-fullwidth"},
            C!{"table is-bordered"},
            tr![
                td![
                    "Günler/Saatler"
                ],
                (0..group_ctx.group.hour as i32).map(|h|
                    td![
                        &h+1,
                        {
                            let hour_index: usize = h as usize;
                            ev(Ev::Click, move |_event|
                                Msg::ChangeAllHour(hour_index)
                            )
                        }
                    ]
                )
            ],
            teacher_group.limitations.as_ref().map_or(
                tbody![],
                |lm|
                tbody![
                    lm.iter().enumerate().map(|d|
                        tr![
                            td![
                                &d.1.day.name.to_uppercase(),
                                {
                                    let day_index = d.0;
                                    ev(Ev::Click, move |_event|
                                        Msg::ChangeAllDay(day_index)
                                    )
                                }
                            ],
                            d.1.hours.iter().enumerate().map(|h|
                                get_timetable_row((d.0+1) as i32, h, &teacher_group.timetables)
                            )
                        ]
                    )
                ]
            )
        ],
        input![
            attrs!{
                At::Type=>"button", At::Class=>"button is-primary", At::Value=>"Kaydet"
            },
            ev(Ev::Click, |event| {
                event.prevent_default();
                Msg::Submit
            })
        ]
    ]
}

fn get_timetable_row(day: i32, hour: (usize, &bool), timetables: &Option<Vec<TeacherTimetable2>>)->Node<Msg>{
    match timetables{
        Some(timetable) => {
            let get_timetable = timetable.iter().find(|t| t.day_id == day && t.hour == hour.0 as i16);
            if let Some(t) = get_timetable{
                let mut subject = String::new();
                let split_subject = &t.subject.split(' ').collect::<Vec<&str>>();
                if split_subject.len() == 1{
                    subject = subject + &split_subject[0].chars().collect::<Vec<_>>()[..3].iter().cloned().collect::<String>();
                    subject.push('.');
                }
                else {
                    for s in split_subject{
                        subject.push(s.chars().next().unwrap());
                        subject.push('.');
                    }
                }
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
                    &subject.to_uppercase(),
                    {
                        let day_index = day as usize;
                        let hour_index = hour.0;
                        ev(Ev::Click, move |_event|
                            Msg::ChangeHour((day_index-1, hour_index))
                        )
                    }
                ]
            }
            else {
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
                    },
                    {
                        let day_index = day as usize;
                        let hour_index = hour.0;
                        ev(Ev::Click, move |_event|
                            Msg::ChangeHour((day_index-1, hour_index))
                        )
                    }
                ]
            }
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
                },
                {
                    let day_index = day as usize;
                    let hour_index = hour.0;
                    ev(Ev::Click, move |_event|
                        Msg::ChangeHour((day_index-1, hour_index))
                    )
                }
            ]
        }
    }
}