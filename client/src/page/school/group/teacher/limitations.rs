use seed::{*, prelude::*};
use crate::model::{teacher};
use crate::model::timetable;
use crate::page::school::detail;
use crate::model::teacher::{TeacherTimetable, TeacherGroupContext};
use serde::*;
use crate::page::school::detail::{SchoolContext};

#[derive()]
pub enum Msg{
    Home,
    FetchDays(fetch::Result<Vec<timetable::Day>>),
    FetchLimitation(fetch::Result<Vec<teacher::TeacherAvailable>>),
    FetchAllLimitation(fetch::Result<Vec<teacher::TeacherAvailable>>),
    ChangeHour((usize,usize)),
    SubmitLimitation,
    SubmitAllLimitation,
    ChangeAllHour(usize),
    ChangeAllDay(usize),
}


#[derive(Default, Clone)]
pub struct Model{
    days: Vec<timetable::Day>,
    pub limitation: Vec<teacher::TeacherAvailable>,
    url: Url
}

pub fn init(url: Url, orders: &mut impl Orders<Msg>)-> Model{
    let model = Model{url: url, ..Default::default()};
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

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>, school_ctx: &mut detail::SchoolContext) {
    let sc2 = school_ctx.clone();
    let hours = school_ctx.get_group(&model.url).group.hour;
    match msg {
        Msg::Home => {
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
                        for h in 0..hours{
                            lm[index].hours[h as usize] = true;
                        }
                    }
                    else {
                        for h in 0..hours{
                            lm[index].hours[h as usize] = false;
                        }
                    }
                }
            }
        }
        Msg::FetchDays(days)=>{
            let teacher_ctx = &mut school_ctx.get_mut_teacher(&model.url);
            let teacher_group = &mut teacher_ctx.group.iter_mut().find(|g| g.group == model.url.path()[3].parse::<i32>().unwrap());
            model.days=days.unwrap();
            if let Some(tg) = teacher_group{
                if tg.limitations.is_none(){
                    orders.perform_cmd({
                        let url = format!("/api/schools/{}/groups/{}/teachers/{}/limitations", school_ctx.school.id, model.url.path()[3], model.url.path()[5]);
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
            else{
                *teacher_group = Some(
                    &mut TeacherGroupContext{
                        group: model.url.path()[3].parse().unwrap(),
                        activities: None,
                        limitations: None,
                        timetables: None
                    }
                );
                orders.perform_cmd({
                    let url = format!("/api/schools/{}/groups/{}/teachers/{}/limitations", school_ctx.school.id, model.url.path()[3], model.url.path()[5]);
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
        Msg::FetchLimitation(json)=>{
            let teacher_ctx = &mut school_ctx.get_mut_teacher(&model.url);
            let teacher_group = &mut teacher_ctx.group.iter_mut().find(|g| g.group == model.url.path()[3].parse::<i32>().unwrap());
            match json {
                Ok(mut l) => {
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
                                    let url = format!("/api/schools/{}/teachers/{}/limitations/{}", school_ctx.school.id, model.url.path()[5], model.url.path()[3]);
                                    let request = Request::new(url)
                                        .method(Method::Post)
                                        .json(&model.limitation);
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
        Msg::FetchAllLimitation(json)=>{
            let teachers_ctx = school_ctx.get_mut_teachers();
            //
            match json {
                Ok(mut l) => {
                    l.sort_by(|a, b| a.day.id.cmp(&b.day.id));
                    for teacher_ctx in teachers_ctx{
                        let teacher_group = &mut teacher_ctx.group.iter_mut().find(|g| g.group == model.url.path()[3].parse::<i32>().unwrap());
                        if let Some(tg) = teacher_group{
                            if let Some(lm) = &mut tg.limitations{
                                *lm = l.clone();

                            }
                            else {
                                tg.limitations = Some(l.clone());
                            }
                        }
                    }
                }
                Err(_)=>{
                    /*
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
                                    let url = format!("/api/schools/{}/teachers/{}/limitations/{}", school_ctx.school.id, model.url.path()[5], model.url.path()[3]);
                                    let request = Request::new(url)
                                        .method(Method::Post)
                                        .json(&model.limitation);
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

                     */
                }
            }
        }
        Msg::SubmitLimitation=>{
            let teacher_ctx = &mut school_ctx.get_mut_teacher(&model.url);
            let teacher_group = &mut teacher_ctx.group.iter_mut().find(|g| g.group == model.url.path()[3].parse::<i32>().unwrap());
            if let Some(tg) = teacher_group {
                if let Some(lm) = &mut tg.limitations {
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

        }
        Msg::SubmitAllLimitation => {
            let teacher_ctx = &mut school_ctx.get_mut_teacher(&model.url);
            let teacher_group = &mut teacher_ctx.group.iter_mut().find(|g| g.group == model.url.path()[3].parse::<i32>().unwrap());
            if let Some(tg) = teacher_group {
                if let Some(lm) = &mut tg.limitations {
                    if let Some(teachers) = &sc2.teachers{
                        for t in teachers{
                            orders.perform_cmd({
                                let url = format!("/api/schools/{}/groups/{}/teachers/{}/limitations", model.url.path()[1], model.url.path()[3], t.teacher.id);
                                let request = Request::new(url)
                                    .method(Method::Post)
                                    .json(&lm);
                                async {
                                    Msg::FetchAllLimitation(async {
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
        Msg::ChangeHour(ids)=>{
            let teacher_ctx = &mut school_ctx.get_mut_teacher(&model.url);
            let teacher_group = &mut teacher_ctx.group.iter_mut().find(|g| g.group == model.url.path()[3].parse::<i32>().unwrap());
            if let Some(tg) = teacher_group {
                if let Some(lm) = &mut tg.limitations {
                    if lm[ids.0].hours[ids.1]{
                        lm[ids.0].hours[ids.1]=false;
                    }
                    else{
                        lm[ids.0].hours[ids.1]=true;
                    }
                }
            }
        }
    }
}

pub fn view(model: &Model, school_ctx: &detail::SchoolContext)->Node<Msg>{
    let group_ctx = school_ctx.get_group(&model.url);
    let teacher_ctx = school_ctx.get_teacher(&model.url);
    let teacher_group = teacher_ctx.group.iter().find(|g| g.group == model.url.path()[3].parse::<i32>().unwrap()).unwrap();
    div![
        C!{"column is-12"},
        table![
            C!{"table is-bordered"},
            tr![
                td![
                    "Günler/Saatler"
                ],
                (0..group_ctx.group.hour).map(|h|
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
                                &d.1.day.name,
                                {
                                    let day_index = d.0;
                                    ev(Ev::Click, move |_event|
                                        Msg::ChangeAllDay(day_index)
                                    )
                                }
                            ],
                            d.1.hours.iter().enumerate().map(|h|
                                td![
                                    if *h.1 {
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
                                        let hour_index = h.0;
                                        let day_index = d.0;
                                        ev(Ev::Click, move |_event|
                                            Msg::ChangeHour((day_index, hour_index))
                                        )
                                    }
                                ]
                            )
                        ]
                    )
                ]
            )
        ],
        div![
            C!{"columns"},
            div![
                C!{"column is-2"},
                input![
                    attrs!{
                        At::Type=>"button", At::Class=>"button is-primary", At::Value=>"Kaydet"
                    },
                    ev(Ev::Click, |event| {
                        event.prevent_default();
                        Msg::SubmitLimitation
                    })
                ]
            ],
            div![
                C!{"column is-2"},
                input![
                    attrs!{
                        At::Type=>"button", At::Class=>"button is-primary", At::Value=>"Tüm Öğretmenlere aktar"
                    },
                    ev(Ev::Click, |event| {
                        event.prevent_default();
                        Msg::SubmitAllLimitation
                    })
                ]
            ]
        ]
    ]
}