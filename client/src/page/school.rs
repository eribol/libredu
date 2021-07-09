use seed::{*, prelude::*};
use crate::{Context};
use crate::model::user::UserDetail;
use crate::page::school::detail::SchoolContext;
use crate::model::school::SchoolDetail;

mod add;
pub(crate) mod detail;
mod group;
mod students;
mod student;
mod subjects;
mod class_rooms;
mod library;


#[derive()]
pub struct Model{
    page: SchoolPage,
    selected_school: Option<SchoolContext>,
    url: Url
}

#[derive()]
pub enum SchoolPage{
    Home,
    Add(add::Model),
    Detail(Box<detail::Model>),
    Loading,
    NotFound
}

#[derive()]
pub enum Msg{
    AddSchool(add::Msg),
    DetailSchool(detail::Msg),
    FetchDetail(fetch::Result<(i16, SchoolDetail)>),
    Loading
}

pub fn init(mut url: Url, orders: &mut impl Orders<Msg>, user_ctx: &Option<UserDetail>, schools: &mut Vec<SchoolContext>) ->Model{
    match url.next_path_part() {
        Some("") | None => Model{
            page: SchoolPage::Home,
            selected_school: None,
            url: url.clone()
        },
        Some("add") => Model {
            page: SchoolPage::Add(add::init(url.clone(), &mut orders.proxy(Msg::AddSchool))),
            selected_school: None,
            url: url.clone()
        },
        _ => {
            let id = &url.path()[1].parse::<i32>();
            if let Ok(i) = id{
                let school_ctx = schools.iter_mut().find(|s| s.school.id == *i);
                if let Some(school) = school_ctx {
                    Model {
                        page: SchoolPage::Detail(Box::new(detail::init(url.clone(), &mut orders.proxy(Msg::DetailSchool), school))),
                        selected_school: Some(school.clone()),
                        url: url.clone()
                    }
                }
                else {
                    orders.send_msg(Msg::Loading);
                    Model {
                        page: SchoolPage::Loading,
                        selected_school: None,
                        url: url.clone()
                    }
                }
            }
            else {
                Model {
                    page: SchoolPage::NotFound,
                    selected_school: None,
                    url: url.clone()
                }
            }
        }
    }
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>, ctx: &mut Context) {
    //let _ctx_school = &mut ctx.schools.iter_mut().find(|a| a.school.id == 720917).unwrap();
    match msg{
        Msg::AddSchool(msg)=>{
            if let SchoolPage::Add(model) = &mut model.page {
                add::update(msg, model, &mut orders.proxy(Msg::AddSchool), ctx)
            }
            //if let SchoolPage::Add(city) = model.page
        },
        Msg::DetailSchool(msg)=>{
            let id = model.selected_school.as_ref().unwrap().school.id;
            if let SchoolPage::Detail(m) = &mut model.page {
                let schools = &mut ctx.schools;
                let school = schools.iter_mut().find(|s| s.school.id == id);
                if let Some(schl) = school{
                    detail::update(msg, m, &mut orders.proxy(Msg::DetailSchool), schl)
                }
                else{
                    model.page = SchoolPage::NotFound
                }
            }
        }
        Msg::FetchDetail(school)=> {
            //_ctx.schools.push(school);
            if let Ok(schl) = school{
                let school_ctx = SchoolContext{
                    teachers: None,
                    role: schl.0,
                    groups: None,
                    school: schl.1.clone(),
                    students: None,
                    subjects: None,
                    class_rooms: None,
                    menu: vec![]
                };
                ctx.schools.retain(|s| s.school.id != schl.1.id);
                ctx.schools.push(school_ctx);
                let schools = &mut ctx.schools;
                let school = schools.iter_mut().find(|s| s.school.id == schl.1.id).unwrap();
                model.selected_school = Some(school.clone());
                model.page = SchoolPage::Detail(Box::new(detail::init(model.url.clone(), &mut orders.proxy(Msg::DetailSchool), school)));
            }
            else {
                model.page = SchoolPage::NotFound;
            }
        }
        Msg::Loading => {
            orders.perform_cmd({
                let adres = format!("/api/schools/{}/detail", model.url.path()[1]);
                let request = Request::new(adres)
                    .method(Method::Get);

                async { Msg::FetchDetail(async {
                    request
                        .fetch()
                        .await?
                        .check_status()?
                        .json()
                        .await
                }.await)}
            });
        }
    }
}

fn home(user: &Option<UserDetail>)-> Node<Msg>{
    if user.is_none(){
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
        SchoolPage::Home => home(&ctx.user),
        SchoolPage::Add(city) => add::view(city, ctx).map_msg(Msg::AddSchool),
        SchoolPage::Detail(m) => {
            let id = model.selected_school.as_ref().unwrap().school.id;
            let school_ctx = ctx.schools.iter().find(|s| s.school.id == id);
            if let Some(school) = school_ctx{
                detail::view(m, &ctx.user, &school).map_msg(Msg::DetailSchool)
            }
            else {
                div!["Kurum bulunamadı2"]
            }
        },
        SchoolPage::NotFound => div!["Kurum bulunamadı3"],
        SchoolPage::Loading => div!["yükleniyor"]
    }

}