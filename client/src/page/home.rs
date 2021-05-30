use seed::{*, prelude::*};
use serde::Serialize;
use crate::Context;
use crate::model::post::{SchoolPost, NewPost};
//use seed::app::subs::url_requested::UrlRequest;

// ------ ------
//     Init
// ------ ------

pub fn init(orders: &mut impl Orders<Msg>) -> Model {
    orders.perform_cmd({
        let request = Request::new("/api/posts")
            .method(Method::Get);

        async { Msg::FetchPosts(async {
            request
                .fetch()
                .await?
                .check_status()?
                .json()
                .await
        }.await)}
    });
    Model::default()
}

// ------ ------
//     Model
// ------ ------

#[derive(Default, Serialize)]
pub struct Model {
    posts: Vec<SchoolPost>,
    form: NewPost,
}

// TODO: It should be probably in the `shared` crate.


// ------ ------
//    Update
// ------ ------

#[derive(Debug)]
pub enum Msg{
    FetchPosts(fetch::Result<Vec<SchoolPost>>),
    FetchPost(fetch::Result<SchoolPost>),
    ChangeBody(String),
    SendPost,
    DelPost(i32),
    FetchDelPost(fetch::Result<i32>),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>, ctx: &mut Context) {
    match msg {
        Msg::FetchPosts(post)=>{
            match post{
                Ok(p) => { model.posts = p}
                Err(e) => {
                    log!(e)
                }
            }
            //orders.subscribe(Msg::UrlChanged);
        }
        Msg::FetchPost(post)=>{
            match post {
                Ok(p) => {
                    model.posts.insert(0, p);
                }
                Err(e) => {
                    log!(e);
                }
            }
            model.form.body = "".to_string();
        }
        Msg::ChangeBody(b)=>{
            model.form.body = b;
        }
        Msg::SendPost=>{
            model.form.sender = ctx.user.as_ref().unwrap().id;
            orders.perform_cmd({
                let request = Request::new(format!("/api/schools/{}/posts", &ctx.schools[0].school.id))
                    .method(Method::Post)
                    .json(&model.form);
                async {
                    Msg::FetchPost(async {
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
        Msg::DelPost(id) => {
            orders.perform_cmd({
                let request = Request::new(format!("/api/posts/{}", id))
                    .method(Method::Delete);

                async { Msg::FetchDelPost(async {
                    request
                        .fetch()
                        .await?
                        .check_status()?
                        .json()
                        .await
                }.await)}
            });
        }
        Msg::FetchDelPost(id) => {
            if let Ok(i) = id {
                model.posts.retain(|p| p.id != i);
            }
        }
    }
}

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model, ctx: &Context)-> Node<Msg>{
    div![
        C!{"columns"},
        div![
            C!{"column is-3 is-hidden-mobile"},
        ],
        div![
            C!{"column is-6"},
            if (ctx.user.is_some() && ctx.user.as_ref().unwrap().is_admin) || !ctx.schools.is_empty(){
                article![
                    C!{"media"},
                    div![
                        C!{"media-content"},
                        div![
                            C!{"field"},
                            p![
                                C!{"control"},
                                textarea![
                                    C!{"textarea"},
                                    attrs!{
                                        At::Value => &model.form.body
                                    },
                                    input_ev(Ev::Change, Msg::ChangeBody)
                                ]
                            ]
                        ],
                        nav![
                            C!{"level"},
                            div![
                                C!{"level-left"},
                                div![
                                    C!{"level-item"},
                                    a![
                                        C!{"button is-primary"},
                                        "Paylaş",
                                        ev(Ev::Click, |_event|
                                            //event_present_default(),
                                            Msg::SendPost
                                        )
                                    ]
                                ]
                            ]
                        ]
                    ]
                ]
            }
            else{
                article![]
            },
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
                            if ctx.user.is_some() && (ctx.user.as_ref().unwrap().is_admin || ctx.user.as_ref().unwrap().id == p.sender) {
                                div![
                                    C!{"level-left"},
                                    a![
                                        C!{"level-item"},
                                        span![
                                            C!{"icon is-small"},
                                            i![
                                                C!{"fas fa-trash"}
                                            ]
                                        ],
                                        {
                                            let id = p.id;
                                            ev(Ev::Click, move |_event| {
                                                Msg::DelPost(id)
                                            })
                                        }
                                    ]
                                ]
                            }
                            else{
                                div![
                                    C!{"level-left"},
                                    a![]
                                ]
                            }
                        ]
                    ]
                ]
            )
        ]
    ]
}
