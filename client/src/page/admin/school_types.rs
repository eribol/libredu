use seed::{*, prelude::*};
use crate::{Context, Urls};
use crate::page::admin::subjects;
use crate::page::school::detail::{SchoolContext};

#[derive(Debug, Default, Clone)]
pub struct Model{
    form: Form
}

#[derive(Debug, Default, Clone)]
pub struct Form{
    school_type: i32
}

pub fn init(mut url: Url, orders: &mut impl Orders<Msg>)-> Model {
    Model::default()
}

#[derive(Debug)]
pub enum Msg{
    FetchSchoolTypes
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {

    match msg {
        Msg::FetchSchoolTypes => {}
    }
}

pub fn view() -> Node<Msg> {
    div!["Subjects"]
}