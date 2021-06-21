use seed::{*, prelude::*};
//use crate::page::school::groups;
use serde::*;
use crate::model::school::{SchoolDetail, UpdateSchoolForm};
use crate::model::user::UserDetail;
use crate::model::post::SchoolPost;
use crate::model::group::{GroupContext, ClassGroups};
use crate::page::school::{group, students, subjects, class_rooms};
use crate::model::student::Student;
use crate::model::class_room::Classroom;
use crate::model::subject::Subject;
use crate::model::teacher::{TeacherContext, Teacher, TeacherGroupContext};


#[derive(Debug, Default)]
pub struct Model{
    url: Url,
    page: Pages,
    form: UpdateSchoolForm,
    group_form: GroupForm,
    posts: Vec<SchoolPost>,
    selected_group: Option<GroupContext>,
    edit: bool
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GroupForm{
    name: String,
    hour: i32
}

#[derive(Debug)]
pub enum Pages{
    Home,
    Detail(SchoolDetail),
    Group(Box<group::home::Model>),
    Students(students::Model),
    Subjects(subjects::Model),
    Classrooms(class_rooms::Model),
    //Library(library::home::Model),
    NotFound,
    Loading
}
impl Pages{
    fn init(mut url: Url, orders:&mut impl Orders<Msg>, school_ctx: &mut SchoolContext) -> Self {
        match url.next_path_part() {
            Some("") | None => Self::Home,
            Some("detail") => {
                Self::Detail(SchoolDetail::default())
            },
            Some("students") => {
                Self::Students(students::init(url.clone(), &mut orders.proxy(Msg::Students), &school_ctx))
            },
            Some("subjects") => {
                Self::Subjects(subjects::init(&mut orders.proxy(Msg::Subjects), &school_ctx))
            },
            Some("class_rooms") => {
                Self::Classrooms(class_rooms::init(&mut orders.proxy(Msg::Classrooms), &school_ctx))
            },
            Some("groups") => {
                let _url = url.next_path_part();
                if school_ctx.groups.is_none(){
                    orders.perform_cmd({
                        let adres = format!("/api/schools/{}/groups", school_ctx.school.id);
                        let request = Request::new(adres)
                            .method(Method::Get);
                        async {
                            Msg::FetchClassGroups(async {
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
                else if let Some(groups) = &mut school_ctx.groups {
                    if !groups.is_empty(){
                        return Self::Group(Box::new(group::home::init(url, school_ctx, &mut orders.proxy(Msg::Group))));
                    }
                }
                Self::Home
            }
            _ => Self::NotFound
        }
    }
}
impl Default for Pages{
    fn default()-> Self{
        Pages::Home
    }
}

#[derive(Debug)]
pub enum Msg{
    //Timetable(timetable::Msg),
    //Teachers(teachers::Msg),
    Home,
    Group(group::home::Msg),
    Students(students::Msg),
    Subjects(subjects::Msg),
    Classrooms(class_rooms::Msg),
    //Library(library::home::Msg),
    FetchDetail(fetch::Result<(i16, SchoolDetail)>),
    FetchClassGroups(fetch::Result<Vec<ClassGroups>>),
    UpdateSubmit,
    UpdateFetch(fetch::Result<SchoolDetail>),
    NameChanged(String),
    FetchPosts(fetch::Result<Vec<SchoolPost>>),
    TelChanged(String),
    LocationChanged(String),
    ChangeGroupName(String),
    ChangeGroupHour(String),
    AddGroup,
    FetchGroup(fetch::Result<ClassGroups>),
    FetchTeachers(fetch::Result<Vec<Teacher>>),
    Loading
}

pub fn init(url: Url, orders: &mut impl Orders<Msg>, school_ctx: &mut SchoolContext) ->Model {
    let model = Model {
        page: Pages::Loading,
        //page: Pages::init(url.clone(), orders, school_ctx),
        url: url,
        form: UpdateSchoolForm{
            name: school_ctx.school.name.clone(),
            tel: school_ctx.school.tel.clone(),
            location: school_ctx.school.location.clone(),
            city: school_ctx.school.city.clone(),
            town: school_ctx.school.town.clone()
        },
        ..Default::default()
    };
    orders.send_msg(Msg::Loading);
    model
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>, school_ctx: &mut SchoolContext) {

    match msg{
        Msg::Home => {
        }
        Msg::FetchTeachers(teachers)=>{
            if let Ok(teachers) = teachers {
                let sc2 = school_ctx.clone();
                let groups = sc2.get_groups();
                if let Some(tchrs) = &mut school_ctx.teachers {
                    tchrs.clear();
                    for t in teachers {
                        let mut teacher = TeacherContext {
                            teacher: t,
                            group: vec![],
                            activities: None
                        };
                        for g in groups{
                            teacher.group.push(
                                TeacherGroupContext{
                                    group: g.group.id,
                                    activities: None,
                                    limitations: None,
                                    timetables: None
                                }
                            );
                        }
                        tchrs.push(teacher)
                    }
                }
                else{
                    let mut tchrs: Vec<TeacherContext> = vec![];
                    for t in teachers {
                        let mut teacher = TeacherContext {
                            teacher: t,
                            group: vec![],
                            activities: None
                        };
                        for g in groups{
                            teacher.group.push(
                                TeacherGroupContext{
                                    group: g.group.id,
                                    activities: None,
                                    limitations: None,
                                    timetables: None
                                }
                            );
                        }
                        tchrs.push(teacher);
                    }
                    school_ctx.teachers = Some(tchrs);
                }
            }
            model.page = Pages::init(model.url.clone(), orders, school_ctx);
        }
        Msg::FetchPosts(posts) => {
            if let Ok(p) = posts {
                model.posts = p;
            }
            if school_ctx.groups.is_none(){
                orders.perform_cmd({
                    let adres = format!("/api/schools/{}/groups", school_ctx.school.id);
                    let request = Request::new(adres)
                        .method(Method::Get);
                    async {
                        Msg::FetchClassGroups(async {
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
        Msg::UpdateSubmit=>{
            if model.edit{
                orders.perform_cmd({
                    let adres = format!("/api/schools/{}/detail", school_ctx.school.id);
                    let request = Request::new(adres)
                        .method(Method::Patch)
                        .json(&model.form);
                    async { Msg::UpdateFetch(async {
                        request?
                            .fetch()
                            .await?
                            .check_status()?
                            .json()
                            .await
                    }.await)}
                });
            }
            else {
                model.edit = true;
            }
        }
        Msg::UpdateFetch(s)=>{
            match s{
                Ok(school) => {
                    //schools.retain(|s| s.school.id != model.school.id);
                    orders.perform_cmd({
                        let adres = format!("/api/schools/{}/detail", &school.id);
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
                    //SessionStorage::insert("schools", &school).expect("");
                }
                Err(_) => {
                    model.form = UpdateSchoolForm{
                        name: school_ctx.school.name.clone(),
                        tel: school_ctx.school.tel.clone(),
                        location: school_ctx.school.location.clone(),
                        city: school_ctx.school.city.clone(),
                        town: school_ctx.school.town.clone()
                    };
                }
            }
            model.edit = false;
        }
        Msg::NameChanged(name)=>{
            model.form.name = name;
        }
        Msg::FetchClassGroups(groups) => {
            if let Ok(g) = groups {
                let mut ctx_grps: Vec<GroupContext> = vec![];
                for i in g{
                    let ctx_group = GroupContext{
                        group: i,
                        classes: None
                    };
                    ctx_grps.push(ctx_group);
                }
                school_ctx.groups = Some(ctx_grps);
            }
            else {
                model.page = Pages::NotFound;
            }
        }
        Msg::Group(msg)=>{
            //model.url.next_path_part();
            if let Pages::Group(m)= &mut model.page {
                group::home::update(msg, m, &mut orders.proxy(Msg::Group), school_ctx)
            }
        }
        Msg::Students(msg)=>{
            if let Pages::Students(m)= &mut model.page{
                students::update(msg, m, &mut orders.proxy(Msg::Students), school_ctx)
            }
        }
        Msg::Subjects(msg)=>{
            if let Pages::Subjects(m)= &mut model.page{
                subjects::update(msg, m, &mut orders.proxy(Msg::Subjects), school_ctx)
            }
        }
        Msg::Classrooms(msg)=>{
            if let Pages::Classrooms(m)= &mut model.page{
                class_rooms::update(msg, m, &mut orders.proxy(Msg::Classrooms), school_ctx)
            }
        }
        /*
        Msg::Library(msg)=>{
            if let Pages::Library(m)= &mut model.page{
                library::home::update(msg, m, &mut orders.proxy(Msg::Library), school_ctx)
            }
        }*/
        Msg::FetchDetail(Ok(school))=> {
            //_ctx.schools.push(school);
            school_ctx.school = school.1.clone();
            school_ctx.role = school.0;
            model.form = UpdateSchoolForm{
                name: school_ctx.school.name.clone(),
                tel: school_ctx.school.tel.clone(),
                location: school_ctx.school.location.clone(),
                city: school_ctx.school.city.clone(),
                town: school_ctx.school.town.clone()
            };
            orders.perform_cmd({
                let adres = format!("/api/schools/{}", school_ctx.school.id);
                let request = Request::new(adres)
                    .method(Method::Get);
                async {
                    Msg::FetchPosts(async {
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
        Msg::FetchDetail(Err(_))=>{
            model.page = Pages::NotFound
        }
        Msg::TelChanged(tel) => {
            model.form.tel = Some(tel)
        }
        Msg::LocationChanged(locate) => {
            model.form.location = Some(locate)
        }
        Msg::ChangeGroupName(name) => {
            model.group_form.name = name;
        }
        Msg::ChangeGroupHour(hour) => {
            match hour.parse::<i32>(){
                Ok(h) => {
                    model.group_form.hour = h
                }
                Err(_) => {
                    model.group_form.hour = 0
                }
            }
        }
        Msg::FetchGroup(group) => {
            if let Ok(g) = group {
                if let Some(groups) = &mut school_ctx.groups{
                    let ctx = GroupContext{
                        group: g,
                        classes: None
                    };
                    groups.push(ctx);
                }
                else{
                    let ctx = GroupContext{
                        group: g,
                        classes: None
                    };
                    school_ctx.groups = Some(vec![ctx]);
                }
            }
            else{
                log!("hata");
            }
            model.page = Pages::init(model.url.clone(), orders, school_ctx)
        }
        Msg::AddGroup => {
            orders.perform_cmd({
                let adres = format!("/api/schools/{}/groups", school_ctx.school.id);
                let request = Request::new(adres)
                    .method(Method::Post)
                    .json(&model.group_form);
                async { Msg::FetchGroup(async {
                    request?
                        .fetch()
                        .await?
                        .check_status()?
                        .json()
                        .await
                }.await)}
            });
        }
        Msg::Loading => {
            if school_ctx.groups.is_none() {
                orders.perform_cmd({
                    let adres = format!("/api/schools/{}/groups", school_ctx.school.id);
                    let request = Request::new(adres)
                        .method(Method::Get);
                    async {
                        Msg::FetchClassGroups(async {
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
            if school_ctx.teachers.is_none() {
                orders.perform_cmd({
                    let adres = format!("/api/schools/{}/teachers", &school_ctx.school.id);
                    let request = Request::new(adres)
                        .method(Method::Get);
                    async {
                        Msg::FetchTeachers(async {
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
            if school_ctx.subjects.is_none() {
                orders.perform_cmd({
                    let adres = format!("/api/schools/{}/teachers", &school_ctx.school.id);
                    let request = Request::new(adres)
                        .method(Method::Get);
                    async {
                        Msg::FetchTeachers(async {
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

            model.page = Pages::init(model.url.clone(), orders, school_ctx);
        }
    }
}

pub fn view(model: &Model, user_ctx: &Option<UserDetail>, school_ctx: &SchoolContext)-> Node<Msg>{
    div![
        C!{"columns"},
        div![
            C!{"column is-2"},
            aside![
                C!{"menu"},
                p![
                    C!{"menu-label"},
                    &school_ctx.school.name
                ],

                menus(school_ctx, model),
                p![
                    C!{"menu-label"},
                    "Gruplar",
                    C!{"menu-list"},
                    match &school_ctx.groups{
                        Some(groups_ctx) => {
                            ul![
                            groups_ctx.iter().map(|group_ctx|

                                li![
                                    a![
                                        attrs!{
                                            At::Href => format!("/schools/{}/groups/{}", school_ctx.school.id, group_ctx.group.id);
                                        },
                                        &group_ctx.group.name
                                    ]
                                ]

                            )]
                        }
                        None => {
                            div!["Yükleniyor"]
                        }
                    }
                ]
            ],
            label!["Grup Adı:"],
            input![
                C!{"input"},
                attrs!{
                    At::Value => &model.group_form.name
                },
                input_ev(Ev::Change, Msg::ChangeGroupName)
            ],
            label!["Günlük ders saati sayısı"],
            input![
                C!{"input"},
                attrs!{
                    At::Value => &model.group_form.hour
                },
                input_ev(Ev::Change, Msg::ChangeGroupHour)
            ],
            input![
                C!{"button is-secondary"},
                attrs!{
                    At::Type => "button",
                    At::Value => "Grup Ekle"
                },
                ev(Ev::Click, move |_event| {
                    Msg::AddGroup
                })
            ]
        ],
        match &model.page{
            Pages::Detail(_m)=>{
                detail_page(model, user_ctx, school_ctx)
            }
            Pages::Group(m) => {
                //div!["groups"]
                match &school_ctx.groups{
                    Some(groups) => {
                        //let group_ctx = school_ctx.get_group(model.url.path()[3].parse().unwrap());
                        group::home::view(m, &school_ctx).map_msg(Msg::Group)
                    }
                    None => div!["Grup ekleyiniz."]
                }

            }
            Pages::NotFound => not_found(),
            Pages::Home => {
                posts(model, user_ctx)
            }
            Pages::Students(m) => {
                students::view(m, school_ctx).map_msg(Msg::Students)
            },
            Pages::Subjects(m) => {
                subjects::view(m).map_msg(Msg::Subjects)
            },
            Pages::Classrooms(m) => {
                class_rooms::view(m).map_msg(Msg::Classrooms)
            }
            Pages::Loading => {
                div!["yükleniyor..."]
            }
            //Pages::Library(m) => {
            //    library::home::view(m, school_ctx).map_msg(Msg::Library)
            //}
        }
    ]
}

pub fn menus(school_ctx: &SchoolContext, model: &Model) -> Node<Msg>{
    use crate::model::school::LIST;
    ul![
        C!{"menu-list"},
        LIST.iter().map(|m|
            li![
                a![
                    C!{
                        if active_menu(&model.page, m){"is-active"} else {""}
                    },
                    &m.name,
                    attrs![
                        At::Href=> format!("/schools/{}/{}", school_ctx.school.id, &m.link);
                    ]
                ]
            ]
        )
    ]
}
fn active_menu (page: &Pages, menu: &crate::model::school::SchoolMenu) -> bool{
    match page{
        Pages::Detail(_m) => {
            menu.link == "detail"
        }
        Pages::Students(_) => {
            menu.link == "students"
        }

        Pages::Classrooms(_) => {
            menu.link == "class_rooms"
        }
        Pages::Subjects(_) => {
            menu.link == "subjects"
        }
        Pages::Home => {
            menu.link.is_empty()
        }
        _ => {
            false
        }
    }
}
fn not_found() -> Node<Msg>{
    div!["Kurum bulunamadı1"]
}
fn posts(model: &Model, user_ctx: &Option<UserDetail>) -> Node<Msg>{
    div![
        C!{"column is-4"},
        model.posts.iter().map(|p|
            article![
                C!{"media"},
                div![
                    C!{"media-content"},
                    div![
                        C!{"content"},
                        p![
                            strong![
                                match &p.school{
                                    Some(s) => {
                                        div![
                                            a![
                                                {&s.name},
                                                attrs!{
                                                    At::Href => format!("/schools/{}", &s.id)
                                                }
                                            ]
                                        ]
                                    },
                                    None => div!["Admin"]
                                }
                            ]
                        ],
                        p.body.split("<br>").map(|p2|
                            p![
                                &p2
                            ]
                        )
                    ],
                    nav![
                        C!{"level"},
                        div![
                            C!{"level-left"},
                            if user_ctx.is_some() && (user_ctx.as_ref().unwrap().is_admin || user_ctx.as_ref().unwrap().id == p.sender) {
                                a![
                                    C!{"level-item"},
                                    span![
                                        C!{"icon is-small"},
                                        i![
                                            C!{"fas fa-trash"}
                                        ]
                                    ]
                                ]
                            }
                            else{
                                a![]
                            }
                        ]
                    ]
                ]
            ]
        )
    ]
}
fn detail_page(model: &Model, user_ctx: &Option<UserDetail>, school_ctx: &SchoolContext)-> Node<Msg> {
    if model.edit{
        div![
            C!{"column is-12"},
            div![C!{"field"},
                label![C!{"label"}, "Okul Adı:"],
                p![C!{"control has-icons-left"},
                    input![C!{"input"},
                        attrs!{
                            At::Type=>"text",
                            At::Name=>"name",
                            At::Id=>"name",
                            At::Value=>&model.form.name,
                            //At::Disabled => disabled(model,ctx).as_at_value()
                        },
                        input_ev(Ev::Input, Msg::NameChanged),
                    ]
                ]
            ],
            div![C!{"field"},
                div![
                    C!{" field is-expanded"},
                    div![
                        C!{"field has-addons"},
                        p![
                            C!{"control"},
                            a![
                                C!{"button is-static"}, "+90"
                            ]
                        ],
                        p![C!{"control has-icons-left"},
                            input![
                                C!{"input"},
                                attrs!{
                                    At::Type=>"tel",
                                    At::Placeholder=>"Telefon numaranız",
                                    At::Value => &model.form.tel.as_ref().unwrap(),
                                },
                                input_ev(Ev::Input, Msg::TelChanged),
                            ]
                        ]
                    ]
                ]
            ],
            div![C!{"field"},
                div![
                    C!{" field is-expanded"},
                    div![
                        C!{"field has-addons"},
                        input![
                            C!{"input"},
                            attrs!{
                                At::Type=>"text",
                                At::Placeholder=>"Adresi",
                                At::Value => &model.form.location.as_ref().unwrap(),
                            },
                            input_ev(Ev::Input, Msg::LocationChanged),
                        ]
                    ]
                ]
            ],
            div![C!{"field"},
                label![C!{"label"}, "İli:"],
                p![C!{"control has-icons-left"},
                    label![C!{"label"},
                        &school_ctx.school.city.name,
                    ]
                ]
            ],
            div![C!{"field"},
                label![C!{"label"}, "İlçesi:"],
                p![C!{"control has-icons-left"},
                    label![C!{"label"},
                        &school_ctx.school.town.name,
                    ]
                ]
            ],
            div![C!{"field"},
                p![C!{"control has-icons-left"},
                    input![C!{"button is-primary"},
                        attrs!{
                            At::Type=>"button",
                            At::Value=>"Güncelle",
                            At::Id=>"update_button",
                            //At::Disabled => false.as_at_value()
                        },
                    ],
                    ev(Ev::Click, |event| {
                        event.prevent_default();
                        Msg::UpdateSubmit
                        }
                    )
                ]
            ]
        ]
    }
    else {
        div![
            C!{"column is-12"},
            div![C!{"field"},
                label![C!{"label"}, "Okul Adı:"],
                p![C!{"control has-icons-left"},
                    label![
                        C!{"label"}, &model.form.name
                    ]
                ]
            ],
            div![
                C!{"field"},
                label![C!{"label"}, "Telefon:"],
                p![C!{"control"},
                    label![
                        C!{"label"}, &model.form.tel.as_ref().unwrap_or(&"".to_string())
                    ]
                ]
            ],
            div![
                C!{"field"},
                label![C!{"label"}, "Adresi:"],
                p![C!{"control"},
                    label![
                        C!{"label"}, &model.form.location.as_ref().unwrap_or(&"".to_string())
                    ]
                ]
            ],
            div![C!{"field"},
                label![C!{"label"}, "İli:"],
                p![C!{"control has-icons-left"},
                    label![C!{"label"},
                        &model.form.city.name,
                    ]
                ]
            ],
            div![C!{"field"},
                label![C!{"label"}, "İlçesi:"],
                p![C!{"control has-icons-left"},
                    label![C!{"label"},
                        &model.form.town.name,
                    ]
                ]
            ],
            div![C!{"field"},
                p![C!{"control has-icons-left"},
                    input![C!{"button is-primary"},
                        attrs!{
                            At::Type=>"button",
                            At::Value=>"Düzenle",
                            At::Id=>"update_button",
                            //At::Disabled => false.as_at_value()
                        },
                    ],
                    ev(Ev::Click, |event| {
                        event.prevent_default();
                        Msg::UpdateSubmit
                        }
                    )
                ]
            ]
        ]
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct City {
    pub pk: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Town {
    pub city: i32,
    pub pk: i32,
    pub name: String,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SchoolContext{
    pub teachers: Option<Vec<TeacherContext>>,
    pub role: i16,
    //pub classes: Vec<Class>,
    pub groups: Option<Vec<GroupContext>>,
    pub school: SchoolDetail,
    pub students: Option<Vec<Student>>,
    pub subjects: Option<Vec<Subject>>,
    pub class_rooms: Option<Vec<Classroom>>,
    pub menu: Vec<SchoolMenu>
}

#[derive(Debug, Serialize, Deserialize, Default,Clone)]
pub struct SchoolMenu{
    pub link: String,
    pub name: String,
}

impl SchoolContext{
    fn get_mut_groups(&mut self) -> &mut Vec<GroupContext>{
        self.groups.get_or_insert(vec![])
    }
    pub fn get_mut_teachers(&mut self) -> &mut Vec<TeacherContext>{
        self.teachers.get_or_insert(vec![])
    }
    pub fn get_mut_teacher(&mut self, url: &Url) -> &mut TeacherContext{
        let teacher_id: i32 = url.path()[5].parse().unwrap();
        let teachers = self.get_mut_teachers();
        teachers.iter_mut().find(|ref mut t| t.teacher.id == teacher_id).unwrap()
    }
    pub fn get_teachers(&self) -> &Vec<TeacherContext>{
        self.teachers.as_ref().unwrap()
    }
    pub fn get_teacher(&self, url: &Url) -> &TeacherContext{
        let teachers = self.get_teachers();
        teachers.iter().find(|t| t.teacher.id == url.path()[5].parse::<i32>().unwrap()).unwrap()
    }
    pub fn get_subjects(&self) -> &Vec<Subject>{
        self.subjects.as_ref().unwrap()
    }
    pub fn get_mut_group(&mut self, url: &Url) -> &mut GroupContext{
        let group_id: i32 = url.path()[3].parse().unwrap();
        let groups = self.get_mut_groups();
        let group = groups.iter_mut().find(|ref mut g| g.group.id == group_id).unwrap();
        group
    }
    pub fn get_group(&self, url: &Url) -> &GroupContext{
        let groups = self.get_groups();
        groups.iter().find(|g| g.group.id == url.path()[3].parse::<i32>().unwrap()).unwrap()
    }
    pub fn get_groups(&self) -> &Vec<GroupContext>{
        self.groups.as_ref().unwrap()
    }
    pub fn get_students(&self) -> &Vec<Student>{
        self.students.as_ref().unwrap()
    }
    pub fn get_next_teacher(&self, url: &Url) -> Option<TeacherContext>{
        let teachers = self.get_teachers();
        let id = teachers.iter().enumerate().find(|t| t.1.teacher.id == url.path()[5].parse::<i32>().unwrap()).unwrap();
        if id.0 + 1 < teachers.len(){
            Some(teachers[id.0+1].clone())
        }
        else {
            None
        }
    }
    pub fn get_prev_teacher(&self, url: &Url) -> Option<TeacherContext>{
        let teachers = self.get_teachers();
        let id = teachers.iter().enumerate().find(|t| t.1.teacher.id == url.path()[5].parse::<i32>().unwrap()).unwrap();
        if id.0 > 0 {
            Some(teachers[id.0-1].clone())
        }
        else {
            None
        }
    }
}
