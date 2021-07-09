use crate::model::timetable::{Day};
use serde::*;
use seed::{*, prelude::*};
use crate::model::class::{ClassTimetable, ClassContext, ClassAvailable};
use crate::page::school::detail::{SchoolContext};
use crate::model::activity;

#[derive(Serialize, Deserialize, Clone)]
pub struct Subject{
    pub name: String,
    pub id: i32
}

#[derive()]
pub enum Msg{
    Home,
    FetchDays(fetch::Result<Vec<Day>>),
    FetchActivities(fetch::Result<Vec<activity::FullActivity>>),
    FetchTimetable(fetch::Result<Vec<ClassTimetable>>),
    FetchLimitation(fetch::Result<Vec<ClassAvailable>>),
    ChangeHour((usize,usize)),
    ChangeAllHour(usize),
    ChangeAllDay(usize),
    Submit
}

#[derive(Default, Clone)]
pub struct Model{
    url: Url,
    days: Vec<Day>,
    hours: usize,
    classes: Vec<ClassContext>
}

pub fn init(url: Url, orders: &mut impl Orders<Msg>, school_ctx: &SchoolContext)-> Model{
    let group_ctx = school_ctx.get_group(&url);
    //let class_ctx = group_ctx.get_class(&url);
    let classes = school_ctx.get_group(&url).get_classes();
    let model = Model{
        url: url.clone(),
        hours: group_ctx.group.hour as usize,
        classes: classes.clone(),
        ..Default::default()
    };

    let school_id = &url.path()[1];
    let group_id = &url.path()[3];
    let class_id = &url.path()[5];
    orders.perform_cmd({
        let url = format!("/api/schools/{}/groups/{}/classes/{}/timetables", school_id, group_id, class_id);
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
    model
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>, school_ctx: &mut SchoolContext) {
    let group_id = &model.url.path()[3];
    let school_id = &model.url.path()[1];
    let class_ctx = school_ctx.get_mut_group(&model.url).get_mut_class(&model.url);
    let hours = &model.hours;
    match msg {
        Msg::Home => {
        }
        Msg::FetchTimetable(tm)=>{
            class_ctx.timetables = Some(tm.unwrap());
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
        }
        Msg::FetchDays(days)=>{
            model.days=days.unwrap();
            if class_ctx.limitations.is_none(){
                orders.perform_cmd({
                    let url = format!("/api/schools/{}/groups/{}/classes/{}/limitations", school_id, group_id, &class_ctx.class.id);
                    let request = Request::new(url)
                        .method(Method::Get);
                    async {
                        //crate::page::school::group::class::limitations::
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
        Msg::FetchActivities(acts)=>{
            class_ctx.activities = Some(acts.unwrap_or_default());
        }
        Msg::FetchLimitation(json)=>{
            match json {
                Ok(mut l) => {
                    l.sort_by(|a, b| a.day.id.cmp(&b.day.id));
                    let mut changed = false;
                    for d in model.days.iter() {
                        if let Some(limitations) = &mut class_ctx.limitations{
                            if !limitations.iter().any(|ta| ta.day.id == d.id) {
                                let hours = vec![true; *hours];
                                limitations.push(ClassAvailable { day: d.clone(), hours });
                                changed = true;
                            }
                            if limitations[(d.id - 1) as usize].hours.len() != *hours {
                                let hours = vec![true; *hours];
                                limitations[(d.id - 1) as usize].hours = hours;
                                changed = true;
                            }
                        }
                        else {
                            class_ctx.limitations = Some(l.clone());
                        }
                    }
                    if changed{
                        orders.perform_cmd({
                            let url = format!("/api/schools/{}/groups/{}/classes/{}/limitations", school_id, group_id , &class_ctx.class.id);
                            let request = Request::new(url)
                                .method(Method::Post)
                                .json(&class_ctx.limitations.as_ref().unwrap());
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
                Err(_)=>{
                    //class_ctx.limitations = Some(
                    //  vec![ClassAvailable{ hours: vec![], day: Default::default() }]
                    //);

                    if let Some(limitations) = &mut class_ctx.limitations{
                        for d in model.days.iter() {
                            if !limitations.iter().any(|ta| ta.day.id == d.id) {
                                let hours = vec![false; *hours];
                                limitations.push(ClassAvailable { day: d.clone(), hours })
                            }
                            if limitations[(d.id - 1) as usize].hours.len() != *hours {
                                let hours = vec![false; *hours];
                                limitations[(d.id - 1) as usize].hours = hours;
                            }
                        }
                    }
                }
            }
        }

        Msg::ChangeHour(ids)=>{
            if let Some(limitations) = &mut class_ctx.limitations{
                if limitations[ids.0].hours[ids.1]{
                    limitations[ids.0].hours[ids.1]=false;
                }
                else{
                    limitations[ids.0].hours[ids.1]=true;
                }
            }
        }
        Msg::ChangeAllHour(index) => {
            if let Some(limitations) = &mut class_ctx.limitations{
                let mut all = true;
                for l in limitations.iter_mut(){
                    if !l.hours[index]{
                        all = false;
                        break;
                    }
                }
                if all{
                    for l in limitations.iter_mut().take(7){
                        l.hours[index] = false;
                    }
                }
                else {
                    for l in limitations.iter_mut().take(7){
                        l.hours[index] = true;
                    }
                }
            }
        }
        Msg::ChangeAllDay(index) => {
            if let Some(limitations) = &mut class_ctx.limitations{
                if limitations[index].hours.iter().any(|h| !*h){
                    for h in 0..model.hours{
                        limitations[index].hours[h] = true;
                    }
                }
                else {
                    for h in 0..model.hours{
                        limitations[index].hours[h] = false;
                    }
                }
            }

        }
        Msg::Submit=>{
            orders.perform_cmd({
                let url = format!("/api/schools/{}/groups/{}/classes/{}/limitations", school_id, &group_id, &class_ctx.class.id);
                let request = Request::new(url)
                    .method(Method::Post)
                    .json(&class_ctx.limitations.as_ref().unwrap());
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
pub fn timetable(model: &Model, school_ctx:&SchoolContext)->Node<Msg>{
    let group_ctx = school_ctx.get_group(&model.url);
    let class_ctx = group_ctx.get_class(&model.url);
    div![
        C!{"column is-10"},
        table![
            C!{"table is-fullwidth is-scrollable"},
            style!{
                //St::TableLayout=>"fixed",
                //St::Width=>"1050px"
            },
            C!{"table is-bordered"},
            thead![
            tr![
                td![
                    "Günler/Saatler"
                ],
                (0..group_ctx.group.hour as i32).map(|h|
                    td![
                        &h+1, ". Saat",
                        {
                            let hour_index: usize = h as usize;
                            ev(Ev::Click, move |_event|
                                Msg::ChangeAllHour(hour_index)
                            )
                        }
                    ]
                )
            ]],
            class_ctx.limitations.as_ref().map_or(
                tbody![tr![]], |limitations|
                tbody![
                limitations.iter().enumerate().map(|d|
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
                        get_timetable_row((d.0+1) as i32, h, &class_ctx.timetables.as_ref().unwrap_or(&vec![]))
                    )
                ]
            )]
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
            //ev(Ev::Click, |event| {
            //    event.prevent_default();
            //    Msg::Print
            //})
        ]
    ]
}

fn get_timetable_row(day: i32, hour: (usize, &bool), timetable: &[ClassTimetable])->Node<Msg>{
    let get_timetable = timetable.iter().find(|t| t.day_id == day && t.hour == hour.0 as i16);

    match get_timetable{
        Some(t)=>{
            let name = &t.activity.teachers[0].first_name.chars().collect::<Vec<_>>();
            let lastname = &t.activity.teachers[0].last_name.chars().collect::<Vec<_>>();
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
                &name[..3].iter().cloned().collect::<String>().to_uppercase(),".", " ", &lastname[..3].iter().cloned().collect::<String>().to_uppercase(), ".",br![],
                &subject,
                {
                    let day_index = day as usize;
                    let hour_index = hour.0;
                    ev(Ev::Click, move |_event|
                        Msg::ChangeHour((day_index-1, hour_index))
                    )
                }
            ]
        }
        None=>{
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