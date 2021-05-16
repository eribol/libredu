use seed::{*, prelude::*};
use crate::model::school::{SchoolDetail};
use crate::model::user::UserDetail;
use crate::model::post::{SchoolPost, NewPost};
use crate::page::admin;

mod page;
mod model;
mod route;


const LOGIN: &str = "login";
const SCHOOL: &str = "schools";
const SIGN_IN: &str = "signin";
const USER: &str = "users";
const RESET: &str = "reset";

// ------ ------
//     Init
// ------ ------

const STORAGE_KEY: &str = "libredu-user";

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    let mut ctx = Context {
        base_url: url.to_base_url(),
        user: None,
            /*let store = LocalStorage::get(STORAGE_KEY);
            match store{
                Ok(user)=> user,
                Err(_)=> {
                    LocalStorage::remove(STORAGE_KEY).expect("remove saved user");
                    LocalStorage::remove("libredu-school").expect("remove saved user");

                    None
                }
            }*/
        school: vec![]
    };
    orders.perform_cmd({
        let request = Request::new("/api/login")
            .method(Method::Get);

        async { Msg::GetUser(async {
            request
                .fetch()
                .await?
                .check_status()?
                .json()
                .await
        }.await)}
    });
    orders.perform_cmd({
        let request = Request::new("/api/posts")
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
    //orders.send_msg(Msg::SSE);
    Model {
        page: Page::init(url, orders, &mut ctx),
        ctx: ctx,
        posts: vec![],
        form: NewPost::default(),
        navbar: "".to_string()
    }
}

// ------ ------
//     Model
// ------ ------

struct Model {
    ctx: Context,
    page: Page,
    posts: Vec<SchoolPost>,
    form: NewPost,
    navbar: String
}

#[derive(Debug)]
pub struct Context {
    pub base_url: Url,
    pub user: Option<UserDetail>,
    pub school: Vec<SchoolDetail>,
}


// ------ Page ------

pub enum Page {
    Home,
    Login(page::login::Model),
    UserDetail(page::users::home::Model),
    School(page::school::Model),
    NotFound,
    SignIn(page::signin::Model),
    Reset(page::reset::ResetPasswordForm),
    Admin(page::admin::home::Model),
    Help
}

impl Page {
    fn init(mut url: Url, orders:&mut impl Orders<Msg>, ctx: &mut Context) -> Self {
        match url.next_path_part() {
            Some("") | None => Self::Home,
            Some(LOGIN) => Self::Login(page::login::init()),
            Some(USER) => Self::UserDetail(page::users::home::init(url, &mut orders.proxy(Msg::UserDetail), ctx)),
            Some(SCHOOL) => Self::School(page::school::init(url, &mut orders.proxy(Msg::School), ctx)),
            Some(SIGN_IN) => Self::SignIn(page::signin::init()),
            Some(RESET) => Self::Reset(page::reset::init()),
            Some("help") => Self::Help,
            Some("admin") => Self::Admin(admin::home::init(url.clone(),&mut orders.proxy(Msg::Admin))),
            _ => Self::NotFound,
        }
    }
}

// ------ ------
//     Urls
// ------ ------

struct_urls!();
impl<'a> Urls<'a> {
    pub fn home(self) -> Url { self.base_url() }
    pub fn signin(self) -> Url { self.base_url().add_path_part(SIGN_IN) }
    pub fn login(self) -> Url { self.base_url().add_path_part(LOGIN) }
    pub fn reset(self) -> Url { self.base_url().add_path_part(RESET) }
    pub fn help(self) -> Url { self.base_url().add_path_part("help") }
    //pub fn school(self) -> Url { self.base_url().add_path_part(SCHOOL) }
    pub fn school_pages(self, school_id: i32, link: &String) -> Url { self.base_url().add_path_part(SCHOOL).add_path_part(school_id.to_string()).add_path_part(link) }
    pub fn teacher_detail(self, school_id: i32, teacher_id: i32) -> Url {
        self.base_url().add_path_part(SCHOOL).add_path_part(school_id.to_string()).add_path_part("teachers").add_path_part(teacher_id.to_string()) }
    pub fn class_detail(self, school_id: i32, class_id: i32) -> Url {
        self.base_url().add_path_part(SCHOOL).add_path_part(school_id.to_string()).add_path_part("classes").add_path_part(class_id.to_string()) }
    pub fn school_detail(self, school_id: i32) -> Url {
        self.base_url().add_path_part(SCHOOL).add_path_part(school_id.to_string())
    }
    pub fn group_detail(self, school_id: i32, group_id: i32) -> Url {
        self.base_url().add_path_part(SCHOOL).add_path_part(school_id.to_string()).add_path_part("groups").add_path_part(group_id.to_string())
    }
    pub fn user_detail(self, user_id: i32) -> Url {
        self.base_url().add_path_part(USER).add_path_part(user_id.to_string())
    }
}

// ------ ------
//    Update
// ------ ------

#[derive(Debug)]
pub enum Msg {
    UrlChanged(subs::UrlChanged),
    LoginMsg(page::login::Msg),
    SignIn(page::signin::Msg),
    UserDetail(page::users::home::Msg),
    GetUser(fetch::Result<UserDetail>),
    GetSchools(fetch::Result<Vec<SchoolDetail>>),
    ChangeBody(String),
    SendPost,
    DelPost(i32),
    FetchDelPost(fetch::Result<i32>),
    Reset(page::reset::Msg),
    Logout,
    FetchLogout(fetch::Result<String>),
    FetchPosts(fetch::Result<Vec<SchoolPost>>),
    FetchPost(fetch::Result<SchoolPost>),
    School(page::school::Msg),
    ChangeNavbar,
    SSE,
    Notify(fetch::Result<String>),
    Admin(page::admin::home::Msg),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    let ctx = &mut model.ctx;
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            model.navbar = "".to_string();
            model.page = Page::init(url, orders, ctx);
        }
        Msg::Reset(msg)=>{
            if let Page::Reset(model) = &mut model.page {
                page::reset::update(msg, model, &mut orders.proxy(Msg::Reset), ctx)
            }
        }
        Msg::ChangeNavbar => {
            if model.navbar == ""{
                model.navbar = "is-active".to_string()
            }
            else {
                model.navbar = "".to_string()
            }
        }
        Msg::GetUser(user)=>{
            match user{
                Ok(u)=>{
                    ctx.user = Some(u.clone());
                    orders.perform_cmd({
                        let request = Request::new("/api/schools")
                            .method(Method::Get);

                        async { Msg::GetSchools(async {
                            request
                                .fetch()
                                .await?
                                .check_status()?
                                .json()
                                .await
                        }.await)}
                    });
                }
                Err(_)=>{
                    orders.subscribe(Msg::UrlChanged);
                }
            }
        }
        Msg::GetSchools(schools)=>{
            match schools{
                Ok(s)=>{
                    ctx.school = s.clone();
                }
                Err(e)=>{
                    log!(e);
                }
            }
            orders.subscribe(Msg::UrlChanged);
            //orders.subscribe(Msg::UrlChanged);
        }
        Msg::FetchPosts(post)=>{
            match post{
                Ok(p) => { model.posts = p.clone()}
                Err(e) => {
                    log!(e)
                }
            }
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
            model.form.body = "".to_string()
        }
        Msg::ChangeBody(b)=>{
            model.form.body = b;
        }
        Msg::LoginMsg(msg) => {
            if let Page::Login(model) = &mut model.page {
                page::login::update(msg, model, &mut orders.proxy(Msg::LoginMsg), ctx)
            }
        },
        Msg::UserDetail(msg) => {
            if let Page::UserDetail(model) = &mut model.page {
                page::users::home::update(msg, model, &mut orders.proxy(Msg::UserDetail), ctx)
            }
        },
        Msg::Logout => {
            LocalStorage::remove(STORAGE_KEY).expect("remove saved user");
            LocalStorage::remove("libredu-school").expect("remove saved user");
            LocalStorage::remove("libredu-ctx_school").expect("remove saved user");
            ctx.user = None;
            ctx.school = Vec::new();
            orders.perform_cmd({
                let request = Request::new("/logout")
                    .method(Method::Get);
                async {
                    Msg::FetchLogout(async {
                        request
                            .fetch()
                            .await?
                            .check_status()?
                            .text()
                            .await
                    }.await)
                }
            });
            //orders.skip();
        },
        Msg::FetchLogout(Ok(_s)) => {
            LocalStorage::remove(STORAGE_KEY).expect("remove saved user");
            LocalStorage::remove("libredu-school").expect("remove saved user");
            ctx.user = None;
            ctx.school = Vec::new();
            orders.notify(
                subs::UrlRequested::new(crate::Urls::new(&ctx.base_url).home())
            );
        },
        Msg::FetchLogout(Err(_e)) => {
            Urls::new(&ctx.base_url).home();
        }
        Msg::SendPost=>{
            model.form.sender = ctx.user.as_ref().unwrap().id;
            orders.perform_cmd({
                let request = Request::new(format!("/api/schools/{}/posts", &ctx.school[0].id))
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
            match id{
                Ok(i) =>{
                    model.posts.retain(|p| p.id != i);
                }
                Err(_) => {}
            }
        }
        Msg::SignIn(msg) => {
            if ctx.user.is_none() {
                if let Page::SignIn(model) = &mut model.page {
                    page::signin::update(msg, model, &mut orders.proxy(Msg::SignIn), ctx)
                }
            }
            else{
                model.page = Page::Home;
                Urls::new(&ctx.base_url).home();
            }
        },
        Msg::School(msg) => {
            if let Page::School(model) = &mut model.page {
                page::school::update(msg, model, &mut orders.proxy(Msg::School), ctx)
            }
        }
        Msg::SSE => {
            //use web_sys::{EventSource};
            //let event_source = EventSource::new("/sse").unwrap();

            //let event = web_sys::EventSource
        }
        Msg::Notify(_s) => {

        }
        Msg::Admin(msg) => {
            if let Page::Admin(model) = &mut model.page {
                page::admin::home::update(msg, model, &mut orders.proxy(Msg::Admin))
            }
        },
    };
}

fn view(model: &Model) -> Node<Msg> {
    let ctx = &model.ctx;
    div![
        nav![
            C!{"navbar is-fixed-top"},

                navbar_brand(model),
                view_navbar_brand(model, ctx)
            //view_navbar_end(ctx)
        ],
        match &model.page{
            Page::Home => home(model),
            Page::Help => help(),
            Page::Reset(m) => page::reset::view(m).map_msg(Msg::Reset),
            Page::Admin(m) => page::admin::home::view(m).map_msg(Msg::Admin),
            Page::Login(m) => {
                if ctx.user.is_none(){
                    div![
                        page::login::view(m, ctx).map_msg(Msg::LoginMsg)
                    ]
                }
                else{
                    div!["Giriş yapılmış"]
                }
            },
            Page::UserDetail(user) => {
                match &ctx.user {
                    Some(_u) => {
                        page::users::home::view(user, ctx).map_msg(Msg::UserDetail)
                    },
                    None => div!["Giriş yapınız"]
                }
            },
            Page::School(school) => {
                page::school::view(school, ctx).map_msg(Msg::School)
            },
            Page::SignIn(model) => {
                page::signin::view(model).map_msg(Msg::SignIn)
            },
            Page::NotFound => div!["404"]
        }
    ]
}

fn navbar_brand(model: &Model) -> Node<Msg>{
    div![
        C!{"navbar-brand"},
        a![
            C!{"navbar-item is-hidden-mobile"},
            attrs!{
                At::Href=>"/"
            },
            style!{
                St::Color => "white"
            },
            "Libredu"
        ],
        a![
            C!{"navbar-burger", &model.navbar},
            attrs!{
                At::from("role") => "button",
                //At::Class => model.navbar
            },
            span![
            ],
            span![
            ],
            span![
            ],
            ev(Ev::Click, |event| {
                event.prevent_default();
                Msg::ChangeNavbar
            })
        ]
    ]
}

fn view_navbar_brand(model: &Model, ctx: &Context) -> Node<Msg>{
        div![
            C!{"navbar-menu", &model.navbar},
            if ctx.school.len()>0{
                div![
                    C!{"navbar-start"},
                    div![
                        C!{"navbar-item has-dropdown is-hoverable"},
                        div![
                            C!{"navbar-link"}, "Okullarım"
                        ],
                        div![
                            C!{"navbar-dropdown"},
                            ctx.school.iter().map(|s|
                                a![
                                    C!{"navbar-item"},
                                    &s.name,
                                    attrs!{
                                        At::Href=> Urls::new(&ctx.base_url).school_detail(s.id)
                                    }
                                ]
                            )
                        ]
                    ],
                    if ctx.user.as_ref().unwrap().is_admin{
                    a![
                        C!{"navbar-item"},
                        "Admin",
                        attrs!{
                            At::Href=> "/admin"
                        }
                    ]}
                    else {
                        a![]
                    }
                ]
            }
            else {
                div![
                    C!["navbar-item"],
                    a![
                        C!{"navbar-item"},
                        attrs!{At::Href=>"/schools/add"},"Okul Ekle",
                    ]
                ]
            },
            view_navbar_end(ctx)
        ]

}

fn view_navbar_end(ctx: &Context) -> Node<Msg> {
    match &ctx.user {
        Some(user) => {
            div![
                C!{"navbar-end"},
                a![C!{"navbar-item"}, attrs!{At::Href => Urls::new(&ctx.base_url).help()}, "Yardım"],
                div![
                    C!{"navbar-item has-dropdown is-hoverable"},
                    a![
                        C!{"navbar-link"},
                        attrs!{At::Href => Urls::new(&ctx.base_url).user_detail(user.id)},
                        &user.first_name
                    ],
                    div![
                        C!{"navbar-dropdown"},
                        a![
                            C!{"navbar-item"}, "Kişisel Bilgiler",
                            attrs!{At::Href => Urls::new(&ctx.base_url).user_detail(user.id)}
                        ],
                        a![
                            C!{"navbar-item"},  "Çıkış",
                            attrs!{At::Href => Urls::new(&ctx.base_url).home()},
                            ev(Ev::Click, |_| Msg::Logout),
                        ],
                    ]
                ]
            ]
        },
        None => {
            div![
                C!{"navbar-end"},
                a![C!{"navbar-item"}, attrs!{At::Href => Urls::new(&ctx.base_url).help()}, "Yardım"],
                a![C!{"navbar-item"}, attrs!{At::Href => Urls::new(&ctx.base_url).login()}, "Giriş Yap"],
                a![C!{"navbar-item"}, attrs!{At::Href => Urls::new(&ctx.base_url).signin()}, "Üye Ol"]
            ]
        }
    }
}

fn home(model: &Model)->Node<Msg>{
    div![
        C!{"columns"},
        div![
            C!{"column is-3 is-hidden-mobile"},
        ],
        div![
            C!{"column is-6"},
            if (model.ctx.user.is_some() && model.ctx.user.as_ref().unwrap().is_admin) || model.ctx.school.len() > 0{
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
                            if model.ctx.user.is_some() && (model.ctx.user.as_ref().unwrap().is_admin || model.ctx.user.as_ref().unwrap().id == p.sender) {
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

fn help() -> Node<Msg>{
div![
    C!{"columns"},
    div![
        C!{"column is-2"},
    ],
    div![
        C!{"column is-8"},
        strong![
            "1- Üye Olma"
        ],
        p![
            "Sistemimizi kullanmak için gerekli olan tek şart üye olunması. Ki bu da okulların yönetimi için şart. Bir okul/kurum için tek bir hesap açılması yeterlidir.\
            Üye olunurken girilen telefon numarası ve eposta ayırt edici olarak kullanılacaktır. Öte yandan telefon numarası sadece ve sadece, ileride güvenlik adımı için gerekli olacağı için \
            istenmektedir. Bu sebeple girilen telefon numarası şifrelenerek veritabanına kaydedilmektedir. Telefon numaranızın herhangi bir şekilde tarafımızdan görülmesi mümkün değildir."
        ],
        p![
            strong![
                "2- Okul Ekleme"
            ],
        ],
        p![
            "Bir üye en fazla bir okul ekleyebilir ve eklenen okulun yöneticisi, okulu ekleyen kullanıcı olur. Okul eklemek için, Okulun adı, ili ve ilçesi bilgilerini girmek yeterlidir."
        ],
        p![
            strong![
                "3- Grup Ekleme"
            ],
        ],
        p![
            "Okul ekledikten sonra bir grup eklemeden diğer ayarları yapmak mümkün değildir. Bu sebeple okul ekledikten sonra muhakkak grup ekleyin. Grup eklemek için Okul Bilgileri sayfasını \
            kullanabilirsiniz. Gruplar, belli zaman dilimlerinde eğitim gören sınıfları ayırt etmek için kullanılıyor. Ancak, sistem, karmaşa oluşmasın diye grubu zorunlu kılıyor. Örneğin;\
            Okulunuzda sabahçı ve öğlenci sınıflar için iki ayrı grup oluşturmak karmaşayı ortadan kaldıracaktır. Veya tekli eğitim gören okullar, haftasonu kursları için ikinci bir sınıf grubu\
            oluşturup, zorlanmadan kurs için yeni bir ders dağılımı yapabilir."
        ],
        p![
            strong![
                "4- Sınıf Ekleme"
            ],
        ],
        p![
            "Gruplar eklendikten sonra sınıf ekleme işlemi yapılabilir. Her sınıf muhakkak bir gruba üye olmak zorundadır. Ve bir sınıf yalnızca bir gruba üye olabilir. \
            Sınıfların bir kademesi bir de şubesi olmak zorundadır."
        ],
        p![
            "Sınıf eklendikten sonra, sınıfa aktivite eklenebilir ve sınıf için kısıtlama oluşturulabilir. Bir sınıfa, sınıfın sayfasından aktivite eklenirken başka bir sınıf ile birleştirilemez.\
            Birleştirilmiş sınıflar için yalnızca öğretmen sayfasından aktivite eklenebilir. Sınıfın kısıtlaması için sınıf grubu seçilmesine gerek yoktur zira sınıfın grubu bellidir ve \
            ona göre kısıtlama yapılacaktır."
        ],
        p![
            "Bir sınıf için kısıtlama oluşturulurken gün isminin yazdığı hücreye tıklandığında tümünü açar veya kapatır. Aynı durum saat hücresine tıklanırken de mevcuttur. Unutmadan, bir sınıfa kısıtlama \
            oluşturulmadan ders programı çalışmayacaktır. Bu sebeple oluşturulan her sınıf ve öğretmen için muhakkak kısıtlama eklemek geekecektir. Her ne kadar sınıf veya öğretmen eklenirken kısıtlama oluşturuluyor olsa da, \
            hataların önüne geçilmesi babında tekrardan kısıt eklenmesinde fayda vardır."
        ],
        p![
            "Bir sınıfa veya öğretmene aktivite eklerken dikkat edilmesi gereken en önemli nokta aktivite saat sayısıdır. Bu kısım idarecilerin ve/veya öğretmenlerin kafasını karıştırabilir.\
            Aktivite dediğimiz şey esasen bir bloktur. Mesela, türkçe dersi 8/A sınıfına 6 saattir ve siz de aktivite eklerken direkt 6 saat deyip eklediğinizde, ders dağıtım işlemi, 6 saatin tamamını\
            arka arkaya yerleştirmeye çalışacaktır. Ki bu da ders dağıtımını namümkün yapar. Aktivite dediğimiz şey işte bu ders bloklarıdır. Arka arkaya gelir tüm saatleri. Bu sebeple aktivite\
            eklenirken, örneğin türkçe dersinin aktivitesini eklerken ya üç kez 2 saatlik aktivite eklersiniz veya aktivite saat sayısına '2 2 2' girersiniz. Her ikisi de aynı işlevi yerine getirir. \
            Saat sayıları arasına boşluk bırakılması yeterlidir. "
        ],
        p![
            "Yine aktivite eklerken bilmenizde fayda olan bir durum vardır ki o da şudur. Mesela türkçe dersi için '2 2 2' ders bloklarını eklemek görünüm açısından iyi olsa bile ders dağıtım için zor olur.\
            Özellikle ortaokullarda, 7 saatlik ders süreleri nazara alındığında çift saatlik blokların fazlalığı, ders dağıtımını zorlaştırır. Mesela türkçe dersi için '2 2 1 1! şeklinde aktivite eklenmesi, \
            ders dağıtımı için daha kolay olur. Hatta bazen bu şekilde yerleştirilmesi zorunlu olur."
        ],
        p![
            "Örneğin, okulunuza 2 gün gelen ve 14 saat ders alan bir beden eğitimi öğretmeni düşünün. Ortaokullarda 7 saatlik süreler mevcut. Siz beden eğitimi dersini ikişer saatler şeklinde eklediğinizde, bu öğretmenin\
            ders programının yerleştirilmesi mümkün olmaz. Mecburen bir sınıfın beden dersini '1 1' şeklinde eklemek zorundasınız. Bu durum,çok fazla sorun çıkarabilir ve kullanıcı bunun farkında olmayabilir. \
            Bu sebeple ileride bunların testini yapmayı da ekleyeceğim."
        ],
        p![
            strong![
                "5- Öğretmen Ekleme"
            ],
        ],
        p![
            "Öğretmen eklemek için, öğretmenin ad ve soyadının girilmesi yeterlidir. Ancak ileride eklenecek özellikler nazara alındığında, öğretmen de bir kullanıcı olarak ekleniyor.\
            Şu an için öğretmenin telefon veya eposta bilgisini girmek pek bir şey ifade etmiyor, sadece kaydedilen bir kullanıcı oluyor."
        ],
        p![
            "Öğretmene aktivite eklerken, sınıflardan farklı olarak, birleştirilmiş sınıf oluşturulabilir. Yalnız burada kullanıcıların kafası karışabilir. Aktivite eklerken bir kaç sınıf eklemeniz, \
            o öğretmen için ayrı ayrı o sınıflara atamış olmazsınız, o sınıfları aynı derste birleştirmiş olursunuz. Örneğin 5/A ve 5/B sınıfını seçip, '2 2 2' şeklinde aktivite eklerseniz, bu iki sınıf \
            aynı saatlerde, beraberce ders işliyor demiş olursunuz."
        ],
        p![
            "Öğretmen için kısıtlama oluşturulurken, her grup için ayrı ayrı kısıtlama oluşturmanız gerekir. Mesela normal öğretim zamanları için öğretmene bir boş gün verdiğinizde, bu kısıtlama\
            yalnızca haftaiçi sınıf grupları için geçerli olur. Aynı öğretmene, misalen haftasonu kurs sınıfları için cumartesi gününü kapattığınızda, bu da sadece haftasonu dersleri için geçerli olur.\
            Bu şekilde bir yöntemle, her defasında yeni yeni okul eklemektense, bir sınıf grubu oluşturup, bu sınıf gruplarının ders dağıtımlarını ayrı ayrı yapabilirsiniz."
        ],
    ]
]
}

// ------ ------
// Before Mount
// ------ ------

fn before_mount(_: Url) -> BeforeMount {
    // Since we have the "loading..." text in the app section of index.html,
    // we use MountType::Takover which will overwrite it with the seed generated html
    BeforeMount::new().mount_type(MountType::Takeover)
}

// ------ ------
//  After Mount
// ------ ------

fn after_mount(
    url: Url,
    orders: &mut impl Orders<Msg<'static>, GMsg>,
) -> AfterMount<Model<'static>> {
    orders.send_msg(Msg::RouteChanged(url.try_into().ok()));

    let model = Model::Redirect(Session::new(storage::load_viewer()));
    AfterMount::new(model).url_handling(UrlHandling::None)
}
pub enum GMsg {
    RoutePushed(Route<'static>),
    SessionChanged(Session),
}
fn sink<'a>(g_msg: GMsg, model: &mut Model<'a>, orders: &mut impl Orders<Msg<'static>, GMsg>) {
    if let GMsg::RoutePushed(ref route) = g_msg {
        orders.send_msg(Msg::RouteChanged(Some(route.clone())));
    }

    match model {
        Model::NotFound(_) | Model::Redirect(_) => {
            if let GMsg::SessionChanged(session) = g_msg {
                *model = Model::Redirect(session);
                route::go_to(Route::Home, orders);
            }
        }
        Model::Settings(model) => {
            page::settings::sink(g_msg, model, &mut orders.proxy(Msg::SettingsMsg));
        }
        Model::Home(model) => {
            page::home::sink(g_msg, model);
        }
        Model::Login(model) => {
            page::login::sink(g_msg, model, &mut orders.proxy(Msg::LoginMsg));
        }
        Model::Register(model) => {
            page::register::sink(g_msg, model, &mut orders.proxy(Msg::RegisterMsg));
        }
        Model::Profile(model, _) => {
            page::profile::sink(g_msg, model, &mut orders.proxy(Msg::ProfileMsg));
        }
        Model::Article(model) => {
            page::article::sink(g_msg, model, &mut orders.proxy(Msg::ArticleMsg));
        }
        Model::ArticleEditor(model, _) => {
            page::article_editor::sink(g_msg, model, &mut orders.proxy(Msg::ArticleEditorMsg));
        }
    }
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::builder(update, view)
        .before_mount(before_mount)
        .after_mount(after_mount)
        .routes(|url| Some(Msg::RouteChanged(url.try_into().ok())))
        .sink(sink)
        .build_and_start();
}

#[wasm_bindgen(module = "/pkg/js/print_teachers.js")]
extern "C" {
    fn createPDF(cls: &str, days: &str, start: i16, stop: i16, school: &str, schedules: &str);
}

#[wasm_bindgen(module = "/pkg/js/print_classes.js")]
extern "C" {
    fn class_print(cls: &str, days: &str, stop: i16, school: &str, schedules: &str);
}

#[wasm_bindgen(module = "/pkg/js/print_student_class.js")]
extern "C" {
    fn print_student_class(cls: &str, days: &str, stop: i16, school: &str, schedules: &str);
}

#[wasm_bindgen(module = "/pkg/js/print_class_rooms.js")]
extern "C" {
    fn print_class_rooms(cls: &str);
}