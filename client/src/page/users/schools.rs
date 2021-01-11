use seed::{*, prelude::*};
//use crate::models::user::UserDetail;
//use crate::Urls;
use crate::{Context};
use crate::model::school::SchoolDetail;

#[derive(Debug, Default)]
pub struct Model{
    schools: Vec<SchoolDetail>,
}

#[derive(Debug)]
pub enum Msg{
    FetchSchool
}
pub fn init(_url: Url, _orders: &mut impl Orders<Msg>, _ctx: &mut Context)->Model{
    let model = Model::default();
    model
}

pub fn update(msg: Msg, _model: &mut Model, _orders: &mut impl Orders<Msg>, _ctx: &mut Context) {
    match msg{
        Msg::FetchSchool=>{
            /**/
        }
    }
}

pub fn view(_model: &Model, ctx: &Context)-> Node<Msg>{
    div![
        ctx.school.iter().map( |school|
            a![
                &school.name,
                attrs!{
                    At::Href => format!("/schools/{}", school.id)
                }
            ]
        )
    ]
}
