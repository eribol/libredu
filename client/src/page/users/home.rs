use seed::{*, prelude::*};
use crate::{page};
use crate::{Context};
use crate::page::users::detail;


#[derive(Debug)]
pub enum Msg{
    Home,
    DetailPage(page::users::detail::Msg),
}

#[derive(Debug, Default)]
pub struct Menu{
    title: String,
    link: String
}

#[derive(Debug)]
enum Pages{
    Home,
    Detail(detail::Model),
}

impl Default for Pages{
    fn default()-> Pages{
        Pages::Home
    }
}
#[derive(Debug, Default)]
pub struct Model{
    page: Pages
}

pub fn init(mut url: Url, orders: &mut impl Orders<Msg>, ctx: &mut Context)->Model{
    let mut model = Model::default();
    match  url.next_path_part(){
        Some("") | None => {
            model.page = Pages::Home
        }
        _ => {
            model.page = Pages::Detail(detail::init(url.clone(),&mut orders.proxy(Msg::DetailPage),ctx));
        }
    }

    model
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>, ctx: &mut Context) {
    match msg{
        Msg::Home=>{
            /**/
        },
        Msg::DetailPage(msg) => {
            if let Pages::Detail(m) = &mut model.page {
                page::users::detail::update(msg, m, &mut orders.proxy(Msg::DetailPage), ctx)
            }
        }
    }
}

pub fn view(model: &Model, ctx: &Context)-> Node<Msg> {
    match &model.page {
        Pages::Home => {
            home(ctx, model)
        }
        Pages::Detail(m) => {
            detail::view(m, ctx).map_msg(Msg::DetailPage)
        }
    }
}

fn home(_ctx: &Context, _model: &Model)->Node<Msg> {
    div!["Kullanıcı seçmediniz."]
}

fn not_found(_model: &Model, _ctx: &Context)->Node<Msg>{
    div![
        "not found"
    ]
}