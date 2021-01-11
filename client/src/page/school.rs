use seed::{*, prelude::*};
use crate::{Context};

mod add;
pub(crate) mod detail;
mod group;
mod students;
mod student;
mod subjects;
mod class_rooms;


#[derive(Debug)]
pub struct Model{
    page: SchoolPage
}

#[derive(Debug)]
pub enum SchoolPage{
    Home,
    Add(add::Model),
    Detail(detail::Model),
    //NotFound
}

#[derive(Debug)]
pub enum Msg{
    AddSchool(add::Msg),
    DetailSchool(detail::Msg)
}

pub fn init(mut url: Url, orders: &mut impl Orders<Msg>, ctx: &mut Context) ->Model{
    match url.next_path_part() {
        Some("") | None => Model{
            page: SchoolPage::Home
        },
        Some("add") => Model {
            page: SchoolPage::Add(add::init(url, &mut orders.proxy(Msg::AddSchool)))
        },
        _ => {
            Model {
                page: SchoolPage::Detail(detail::init(url, &mut orders.proxy(Msg::DetailSchool), ctx))
            }
        }
    }
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>, ctx: &mut Context) {
    //let ctx_school = &mut model.
    match msg{
        Msg::AddSchool(msg)=>{
            if let SchoolPage::Add(model) = &mut model.page {
                add::update(msg, model, &mut orders.proxy(Msg::AddSchool), ctx)
            }
            //if let SchoolPage::Add(city) = model.page
        },
        Msg::DetailSchool(msg)=>{
            if let SchoolPage::Detail( model) = &mut model.page {
                detail::update(msg, model, &mut orders.proxy(Msg::DetailSchool), ctx)
            }
        }
    }
}

fn home(ctx: &Context)-> Node<Msg>{

    if ctx.user.is_none(){
        div![
            "Giriş yapınız"
        ]
    }
    else{
        div![
            "Okul seçiniz."
        ]
    }
}
pub fn view(model: &Model, ctx: &Context)-> Node<Msg>{
    match &model.page{
        SchoolPage::Home => home(ctx),
        SchoolPage::Add(city) => add::view(city, ctx).map_msg(Msg::AddSchool),
        SchoolPage::Detail(model) => detail::view(model, ctx).map_msg(Msg::DetailSchool)
    }

}