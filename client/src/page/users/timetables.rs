use seed::{*, prelude::*};
//use crate::models::user::UserDetail;
//use crate::Urls;
use crate::{Context};
use crate::model::timetable::{Day};
use crate::model::school::School;
use crate::model::group::ClassGroups;
use crate::model::teacher::TeacherTimetable;
use crate::model::class;

//use crate::models::school::SchoolDetail;

#[derive(Debug)]
pub struct Model{
    timetable: Vec<(School, ClassGroups, Vec<TeacherTimetable>)>,
    days: Vec<Day>
}

impl Default for Model{
    fn default() -> Model{
        Model {
            timetable: vec![],
            days: vec![
                Day{ id: 1, name: "Pazartesi".to_string() },
                Day{ id: 2, name: "Salı".to_string() },
                Day{ id: 3, name: "Çarşamba".to_string() },
                Day{ id: 4, name: "Perşembe".to_string() },
                Day{ id: 5, name: "Cuma".to_string() },
                Day{ id: 6, name: "Cumartesi".to_string() },
                Day{ id: 7, name: "Pazar".to_string() },
            ]
        }
    }
}
#[derive(Debug)]
pub enum Msg{
    FetchTimetable(fetch::Result<Vec<(School, ClassGroups, Vec<TeacherTimetable>)>>)
}
pub fn init(_url: Url, orders: &mut impl Orders<Msg>, ctx: &mut Context)->Model{
    let model = Model::default();
    match &ctx.user{
        Some(u) => {
            orders.perform_cmd({
                let adres = format!("/api/users/{}/timetables", u.id);
                let request = Request::new(adres)
                    .method(Method::Get);
                async { Msg::FetchTimetable(async {
                    request
                        .fetch()
                        .await?
                        .check_status()?
                        .json()
                        .await
                }.await)}
            });
        }
        None => {}
    }
    model
}

pub fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>, _ctx: &mut Context) {
    match msg{
        Msg::FetchTimetable(timetables)=>{
            match timetables{
                Ok(t) => {
                    log(&t.len());
                    model.timetable= t;
                }
                Err(_) => {}
            }
        }
    }
}

pub fn view(model: &Model, _ctx: &Context)-> Node<Msg>{
    div![
        model.timetable.iter().map(|t|
        table![
            C!{"table table-hover is-bordered"},
            thead![
                tr![
                    th![
                        &t.0.name, " ", &t.1.name, " GRUBUNA AİT DERS PROGRAMINIZ",
                        attrs!{
                            At::ColSpan => "8"
                        }
                    ]
                ]
            ],
            tbody![
                tr![
                    C!{"table-light"},
                    td![],
                    model.days.iter().map(|d|
                        td![
                            &d.name
                        ]
                    )
                ],
                (0..t.1.hour).map(|h|
                    tr![
                        td![
                            &h+1,". Saat",br![],
                        ],
                        model.days.iter().map(|d|
                           td![
                            &get_act(&t.2, d, h)
                           ]
                        )
                    ]
                )

            ]
        ]
        )
    ]
}

fn get_act(t: &Vec<TeacherTimetable>, d: &Day, h: i32) -> Node<Msg>{
    let f = t.iter().find(|a| a.hour == h as i16 && a.day_id == d.id);
    match f{
        Some(a) => {
            div![
                &a.subject,br![],get_classes(&a.class_id)
            ]
        }
        None => {
            div![]
        }
    }
}

fn get_classes(cls: &Vec<class::Class>) -> String{
    let mut class = "".to_string();
    for c in cls{
        class = class + &c.kademe.to_string() + "/" + &c.sube + "\n"
    }
    class
}

