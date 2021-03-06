use crate::model::timetable::{Day};
use serde::*;
use seed::{*, prelude::*};
use crate::model::class::{Class, ClassAvailable, ClassContext};
use crate::model::group::GroupContext;
use crate::page::school::detail::SchoolContext;
use crate::i18n::I18n;

#[derive(Serialize, Deserialize, Clone)]
pub struct Subject{
    pub name: String,
    pub id: i32
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Activity{
    pub(crate) id: i32,
    pub(crate) subject: Subject,
    pub(crate) teacher: i32,
    pub(crate) class: Class,
    pub(crate) hour: i16,
    pub(crate) split: bool,
    classes: Vec<i32>
}

#[derive()]
pub enum Msg{
    Home,
    FetchDays(fetch::Result<Vec<Day>>),
    FetchLimitation(fetch::Result<Vec<ClassAvailable>>),
    FetchGradeLimitation(fetch::Result<Vec<ClassAvailable>>),
    FetchAllLimitation(fetch::Result<Vec<ClassAvailable>>),
    ChangeHour((usize,usize)),
    ChangeAllHour(usize),
    ChangeAllDay(usize),
    Submit,
    SubmitSameGrades,
    SubmitAll,
    Print
}

#[derive(Default, Clone)]
pub struct Model{
    pub class: Class,
    pub days: Vec<Day>,
    hours: usize,
    classes: Vec<ClassContext>,
    url: Url
}

pub fn init(url: Url, orders: &mut impl Orders<Msg>, school_ctx: &mut SchoolContext)-> Model{
    let mut model = Model{url: url.clone(), ..Default::default()};
    let classes = school_ctx.get_group(&url).get_classes();
    let group = school_ctx.get_group(&url);
    model.classes = classes.clone();
    model.hours = group.group.hour as usize;
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
    orders.perform_cmd({
        let url = format!("/api/schools/{}/groups/{}/classes/{}/limitations", &school_ctx.school.id, &group.group.id, &group.get_class(&url).class.id);
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
    model
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>, school_ctx: &mut SchoolContext) {
    let group_ctx = &model.url.path()[3];
    let school_id = &model.url.path()[1];
    let hours = &model.hours;
    let classes = &model.classes;

    match msg {
        Msg::Home => {
        }
        Msg::ChangeAllHour(index) => {
            let class_ctx = school_ctx.get_mut_group(&model.url).get_mut_class(&model.url);
            if let Some(limitations) = &mut class_ctx.limitations{
                let mut all = true;
                for l in limitations.iter_mut(){
                    if !l.hours[index]{
                        all = false;
                        break;
                    }
                }
                if all{
                    for l in limitations.iter_mut(){
                        l.hours[index] = false;
                    }
                }
                else {
                    for l in limitations.iter_mut(){
                        l.hours[index] = true;
                    }
                }
            }
        }
        Msg::ChangeAllDay(index) => {
            let class_ctx = school_ctx.get_mut_group(&model.url).get_mut_class(&model.url);
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
        Msg::Print=>{
            /*let timetable = vec![(&model.class.kademe, &model.class.sube, &model.timetable)];
            let mut first_hour = 0;
            let last_hour = model.school.hour-1;
            for l in &model.limitation{
                let mut index = 0;
                for h in &l.hours{
                    if *h{
                        first_hour = index;
                    }
                    index += 1;
                }
            }
            class_print(&serde_json::to_string(&timetable).unwrap(), &serde_json::to_string(&model.days).unwrap(), first_hour, last_hour, &serde_json::to_string(&model.school).unwrap())*/
        }
        Msg::Submit=>{
            let class_ctx = school_ctx.get_mut_group(&model.url).get_mut_class(&model.url);
            orders.perform_cmd({
                let url = format!("/api/schools/{}/groups/{}/classes/{}/limitations", &school_id, &group_ctx, &class_ctx.class.id);
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
        Msg::SubmitSameGrades => {
            let class_ctx = school_ctx.get_mut_group(&model.url).get_mut_class(&model.url);
            for c in classes{
                if c.class.kademe == class_ctx.class.kademe{
                    orders.perform_cmd({
                        let url = format!("/api/schools/{}/groups/{}/classes/{}/limitations", &school_id, &group_ctx, &c.class.id);
                        let request = Request::new(url)
                            .method(Method::Post)
                            .json(&class_ctx.limitations.as_ref().unwrap());
                        async {
                            Msg::FetchGradeLimitation(async {
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
        Msg::SubmitAll => {
            let class_ctx = school_ctx.get_mut_group(&model.url).get_mut_class(&model.url);

            orders.perform_cmd({
                let url = format!("/api/schools/{}/groups/{}/classes/limitations", school_id, &group_ctx);
                let request = Request::new(url)
                    .method(Method::Post)
                    .json(&class_ctx.limitations.as_ref().unwrap());
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
        Msg::FetchDays(days)=>{
            model.days=days.unwrap();
        }
        Msg::ChangeHour(ids)=>{
            let class_ctx = school_ctx.get_mut_group(&model.url).get_mut_class(&model.url);
            if let Some(limitations) = &mut class_ctx.limitations{
                if limitations[ids.0].hours[ids.1]{
                    limitations[ids.0].hours[ids.1]=false;
                }
                else{
                    limitations[ids.0].hours[ids.1]=true;
                }
            }
        }
        Msg::FetchLimitation(json)=>{
            let class_ctx = school_ctx.get_mut_group(&model.url).get_mut_class(&model.url);
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
                            let url = format!("/api/schools/{}/groups/{}/classes/{}/limitations", school_id, group_ctx , &class_ctx.class.id);
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
        Msg::FetchGradeLimitation(json)=>{
            let class_ctx = &model.classes.iter().find(|c| c.class.id == model.url.path()[5].parse::<i32>().unwrap()).unwrap();
            let classes_ctx = &mut school_ctx.get_mut_group(&model.url).get_mut_classes().iter_mut().filter(|c| c.class.kademe == class_ctx.class.kademe);

            match json {
                Ok(mut l) => {
                    l.sort_by(|a, b| a.day.id.cmp(&b.day.id));
                    while let Some(class_ctx) = classes_ctx.next(){
                        if let Some(limitations) = &mut class_ctx.limitations{
                            *limitations = l.clone();
                        }
                        else {
                            class_ctx.limitations = Some(l.clone());
                        }
                    }
                }
                Err(_)=>{
                    log!("not worked");
                    //class_ctx.limitations = Some(
                    //  vec![ClassAvailable{ hours: vec![], day: Default::default() }]
                    //);
                    /*
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

                     */
                }
            }
        }
        Msg::FetchAllLimitation(json)=>{
            let classes_ctx = school_ctx.get_mut_group(&model.url).get_mut_classes();
            match json {
                Ok(mut l) => {
                    l.sort_by(|a, b| a.day.id.cmp(&b.day.id));
                    for class_ctx in classes_ctx{
                        if let Some(limitations) = &mut class_ctx.limitations{
                            *limitations = l.clone()
                        }
                        else {
                            class_ctx.limitations = Some(l.clone());
                        }
                    }
                }
                Err(_)=>{
                    //class_ctx.limitations = Some(
                    //  vec![ClassAvailable{ hours: vec![], day: Default::default() }]
                    //);
                    /*
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

                     */
                }
            }
        }
    }
}

pub fn limitations(model: &Model, group_ctx: &GroupContext, lang: &I18n)->Node<Msg>{
    let class_ctx = group_ctx.get_class(&model.url);
    use crate::{create_t, with_dollar_sign};
    create_t![lang];
    use fluent::fluent_args;
    div![
        C!{"column"},
        div![
            C!{"columns"},
            table![
                C!{"table is-bordered is-scrollable"},
                thead![
                tr![
                    td![
                        t!["days"], t!["hours"],
                    ],
                    (0..group_ctx.group.hour as i32).map(|h|
                        td![
                            &h+1, ". ", t!["hour"],
                            {
                                let hour_index: usize = h as usize;
                                ev(Ev::Click, move |_event|
                                    Msg::ChangeAllHour(hour_index)
                                )
                            }
                        ]
                    )
                ]],
                class_ctx.limitations.as_ref().map_or(tbody![tr![]], |limitations|
                    tbody![
                    limitations.iter().enumerate().map(|d|
                        tr![
                            td![
                                t![t!("day", fluent_args!["dayId" => &d.1.day.id])],
                                {
                                    let day_index = d.0;
                                    ev(Ev::Click, move |_event|
                                        Msg::ChangeAllDay(day_index)
                                    )
                                }
                            ],
                            d.1.hours.iter().enumerate().map(|l|
                                td![
                                    if *l.1 {
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
                                        let hour_index = l.0;
                                        let day_index = d.0;
                                        ev(Ev::Click, move |_event|
                                            Msg::ChangeHour((day_index, hour_index))
                                    )
                                    }
                                ]
                            )
                        ]
                    )]
                )
            ]
        ],
        div![
            C!{"columns is-multiline"},
            div![
                C!{"column is-2"},
                input![
                    attrs!{
                        At::Type=>"button", At::Class=>"button is-primary", At::Value=> t!["save"]
                    },
                    ev(Ev::Click, |event| {
                        event.prevent_default();
                        Msg::Submit
                    })
                ],
            ],
            div![
                C!{"column is-3"},
                input![
                    attrs!{
                        At::Type=>"button", At::Class=>"button is-primary", At::Value=> t!["save-for-all-same-grade"]
                    },
                    ev(Ev::Click, |event| {
                        event.prevent_default();
                        Msg::SubmitSameGrades
                    })
                ],
            ],
            div![
                C!{"column is-1"},
                input![
                    attrs!{
                        At::Type=>"button", At::Class=>"button is-primary", At::Value=> t!["save-for-all-classes"]
                    },
                    ev(Ev::Click, |event| {
                        event.prevent_default();
                        Msg::SubmitAll
                    })
                ],
            ]
        ]
    ]
}