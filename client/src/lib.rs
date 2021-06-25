use seed::{*, prelude::*};
use crate::model::school::{SchoolDetail};
use crate::model::user::UserDetail;
use crate::model::post::{SchoolPost, NewPost};
use crate::page::admin;
use crate::page::school::detail::SchoolContext;

mod page;
mod model;


const LOGIN: &str = "login";
const SCHOOL: &str = "schools";
const SIGN_IN: &str = "signin";
const USER: &str = "users";
const RESET: &str = "reset";

// ------ ------
//     Init
// ------ ------

const STORAGE_KEY: &str = "user";

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    let mut ctx = Context {
        base_url: url.to_base_url(),
        user: None,
        schools: vec![],
        loaded: false
    };
    orders.perform_cmd({
        let request = Request::new("/api/login")
            .method(Method::Get);

        async {
            Msg::GetUser(async {
                request
                    .fetch()
                    .await?
                    .check_status()?
                    .json()
                    .await
            }.await)
        }
    });
    //
    orders.subscribe(Msg::UrlChanged);
    let page = Page::Loading;
    Model {
        ctx,
        page,
        posts: vec![],
        form: NewPost::default(),
        navbar: "".to_string(),
        url,
        loaded: false
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
    navbar: String,
    url: Url,
    loaded: bool
}

#[derive(Debug, Clone)]
pub struct Context {
    pub base_url: Url,
    pub user: Option<UserDetail>,
    pub schools: Vec<SchoolContext>,
    pub loaded: bool
}

pub enum Page {
    Loading,
    Home(page::home::Model),
    Login(page::login::Model),
    UserDetail(page::users::home::Model),
    School(Box<page::school::Model>),
    NotFound,
    SignIn(page::signin::Model),
    Reset(page::reset::ResetPasswordForm),
    Admin(page::admin::home::Model),
    Help
}

impl Page {
    fn init(mut url: Url, orders:&mut impl Orders<Msg>, ctx: &mut Context) -> Self {
        match url.next_path_part() {
            Some("") | None => Self::Home(page::home::init(&mut orders.proxy(Msg::Home))),
            Some(LOGIN) => Self::Login(page::login::init()),
            Some(USER) => Self::UserDetail(page::users::home::init(url, &mut orders.proxy(Msg::UserDetail), ctx)),
            Some(SCHOOL) => Self::School(Box::new(page::school::init(url, &mut orders.proxy(Msg::School), &ctx.user, &mut ctx.schools))),
            Some(SIGN_IN) => Self::SignIn(page::signin::init()),
            Some(RESET) => Self::Reset(page::reset::init()),
            Some("help") => Self::Help,
            Some("admin") => Self::Admin(admin::home::init(url.clone(),&mut orders.proxy(Msg::Admin))),
            _ => Self::NotFound,
        }
    }
}

struct_urls!();
impl<'a> Urls<'a> {
    pub fn home(self) -> Url { self.base_url() }
    pub fn signin(self) -> Url { self.base_url().add_path_part(SIGN_IN) }
    pub fn login(self) -> Url { self.base_url().add_path_part(LOGIN) }
    pub fn reset(self) -> Url { self.base_url().add_path_part(RESET) }
    pub fn help(self) -> Url { self.base_url().add_path_part("help") }
    //pub fn school(self) -> Url { self.base_url().add_path_part(SCHOOL) }
    pub fn school_pages(self, school_id: i32, link: &str) -> Url { self.base_url().add_path_part(SCHOOL).add_path_part(school_id.to_string()).add_path_part(link) }
    pub fn teacher_detail(self, school_id: i32, teacher_id: i32) -> Url {
        self.base_url().add_path_part(SCHOOL).add_path_part(school_id.to_string()).add_path_part("teachers").add_path_part(teacher_id.to_string()) }
    pub fn class_detail(self, school_id: i32, class_id: i32) -> Url {
        self.base_url().add_path_part(SCHOOL).add_path_part(school_id.to_string()).add_path_part("classes").add_path_part(class_id.to_string()) }
    pub fn school_detail(self, school_id: i32) -> Url {
        self.base_url().add_path_part(SCHOOL).add_path_part(school_id.to_string())
    }
    pub fn group_detail(self, school_id: i32, group_id: i32) -> Url {
        self.home().add_path_part("schools").add_path_part(school_id.to_string()).add_path_part("groups").add_path_part(group_id.to_string())
    }
    pub fn user_detail(self, user_id: i32) -> Url {
        self.base_url().add_path_part(USER).add_path_part(user_id.to_string())
    }
}

#[derive(Debug)]
pub enum Msg {
    Home(page::home::Msg),
    UrlChanged(subs::UrlChanged),
    GetSchools(fetch::Result<Vec<(i16, SchoolDetail)>>),
    Logout,
    FetchLogout(fetch::Result<String>),
    ChangeNavbar,
    LoginMsg(page::login::Msg),
    SignIn(page::signin::Msg),
    UserDetail(page::users::home::Msg),
    GetUser(fetch::Result<UserDetail>),
    ChangeBody(String),
    SendPost,
    DelPost(i32),
    FetchDelPost(fetch::Result<i32>),
    Reset(page::reset::Msg),
    FetchPosts(fetch::Result<Vec<SchoolPost>>),
    FetchPost(fetch::Result<SchoolPost>),
    School(page::school::Msg),
    Sse,
    Notify(fetch::Result<String>),
    Admin(page::admin::home::Msg),
    Loading
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    let ctx = &mut model.ctx;
    match msg {
        Msg::Loading => {
            model.page = Page::init(model.url.clone(), orders, ctx);
            //
        }
        Msg::Home(msg) => {
            if let Page::Home(model) = &mut model.page {
                page::home::update(msg, model, &mut orders.proxy(Msg::Home), ctx);
            }
        }
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            model.navbar = "".to_string();
            model.page = Page::init(url, orders, ctx);
        }
        Msg::GetSchools(schools)=> {
            if let Ok(schools) = schools {
                if !model.loaded{
                    for s in schools {
                        let ctx_school = SchoolContext {
                            teachers: None,
                            role: s.0,
                            groups: None,
                            school: s.1,
                            students: None,
                            subjects: None,
                            class_rooms: None,
                            menu: vec![]
                        };
                        ctx.schools.push(ctx_school);
                    }
                }
                SessionStorage::insert("schools", &ctx.schools).expect("Okullar eklenemedi");
                orders.send_msg(Msg::Loading);
            }
            model.loaded= true;
        }
        Msg::Logout => {
            //LocalStorage::remove("libredu-ctx_school").expect("remove saved user")
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
            SessionStorage::remove(STORAGE_KEY).expect("remove saved user");
            SessionStorage::remove("schools").expect("remove saved schools");
            ctx.user = None;
            ctx.schools = Vec::new();
            //ctx.user = None;
            //ctx.school = vec![];
            orders.notify(
                subs::UrlRequested::new(format!("").parse().unwrap())
            );
        },
        Msg::FetchLogout(Err(_e)) => {
            Urls::new(&ctx.base_url).home();
        }
        Msg::ChangeNavbar => {
            if model.navbar.is_empty(){
                model.navbar = "is-active".to_string()
            }
            else {
                model.navbar = "".to_string()
            }
        }
        Msg::SignIn(msg) => {
            if ctx.user.is_none() {
                if let Page::SignIn(model) = &mut model.page {
                    page::signin::update(msg, model, &mut orders.proxy(Msg::SignIn), ctx)
                }
            }
            else{
                //model.page = Page::Home(page::home::init());
                Urls::new(&ctx.base_url).home();
            }
        },
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
        Msg::Reset(msg)=>{
            if let Page::Reset(model) = &mut model.page {
                page::reset::update(msg, model, &mut orders.proxy(Msg::Reset), ctx)
            }
        }
        Msg::GetUser(user)=>{
            if let Ok(u) = user{
                SessionStorage::insert("user", &u);
                ctx.user = Some(u);
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
            else{
                orders.send_msg(Msg::Loading);
            }

        }
        Msg::School(msg) => {
            if let Page::School(model) = &mut model.page {
                page::school::update(msg, model, &mut orders.proxy(Msg::School), ctx)
            }
        }
        Msg::FetchPosts(post)=>{
            match post{
                Ok(p) => { model.posts = p}
                Err(e) => {
                    log!(e)
                }
            }
            //log!("post");
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
        Msg::Sse => {
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
                view_navbar_brand(model, &ctx)
            //view_navbar_end(ctx)
        ],
        match &model.page{
            Page::Home(m) => page::home::view(m, &ctx).map_msg(Msg::Home),
            Page::Help => help(),
            Page::Reset(m) => page::reset::view(m).map_msg(Msg::Reset),
            //Page::Admin(m) => page::admin::home::view(m).map_msg(Msg::Admin),
            Page::Login(m) => {
                if ctx.user.is_none(){
                    div![
                        page::login::view(m, &ctx).map_msg(Msg::LoginMsg)
                    ]
                }
                else{
                    div!["Giriş yapılmış"]
                }
            },
            Page::UserDetail(user) => {
                match &ctx.user {
                    Some(_u) => {
                        page::users::home::view(user, &ctx).map_msg(Msg::UserDetail)
                    },
                    None => div!["Giriş yapınız"]
                }
            },
            Page::School(school) => {
                page::school::view(school, &ctx).map_msg(Msg::School)
            },
            Page::SignIn(model) => {
                page::signin::view(model).map_msg(Msg::SignIn)
            },
            Page::Loading => div!["loading"],
            _ => div!["404"]
        }
    ]
}

fn navbar_brand(model: &Model) -> Node<Msg>{
    div![
        C!{"navbar-brand"},
        a![
            C!{"navbar-item"},
            attrs!{
                At::Href=>"/"
            },
            style!{
                St::Color => "white"
            },
            "L"
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
            if !ctx.schools.is_empty(){
                div![
                    C!{"navbar-start"},
                    div![
                        C!{"navbar-item has-dropdown is-hoverable"},
                        div![
                            C!{"navbar-link"}, "Okullarım"
                        ],
                        div![
                            C!{"navbar-dropdown"},

                            ctx.schools.iter().map(|ctx_s|
                                a![
                                    C!{"navbar-item"},
                                    &ctx_s.school.name,
                                    attrs!{
                                        At::Href=> Urls::new(&ctx.base_url).school_detail(ctx_s.school.id)
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
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
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