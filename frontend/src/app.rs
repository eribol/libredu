use crate::header;
use std::collections::BTreeSet;
use zoon::*;

pub mod login;
pub mod forget_password;
pub mod school;
pub mod signin;
pub mod view;
pub mod register;
pub mod reset_password;
pub mod admin;
use shared::User;
pub static LANG_STORAGE_KEY: &str = "tr";

pub fn root() -> impl Element {
    Column::new()
        .s(Padding::new().top(10).right(10).left(10))
        .item(header::root())
        .item(view::root())
        .on_viewport_size_change(|width, _height| change_screen_width(width))
}

#[derive(Debug, Clone)]
pub enum Pages {
    Home,
    Login,
    Signin,
    User,
    ForgetPassword,
    ResetPassword,
    Admin
    //NotFound
}

// -------------------
// ---- States -------
// -------------------
#[static_ref]
pub fn login_user() -> &'static Mutable<Option<User>> {
    Mutable::new(None)
}

#[static_ref]
pub fn pages() -> &'static Mutable<Pages> {
    Mutable::new(Pages::Home)
}

#[static_ref]
pub fn unfinished_mutations() -> &'static Mutable<BTreeSet<CorId>> {
    Mutable::new(BTreeSet::new())
}

#[static_ref]
pub fn screen_width() -> &'static Mutable<u32> {
    use zoon::println;
    let w = web_sys::window().unwrap().window().screen().unwrap().width().unwrap();
    println!("{w:?}");
    Mutable::new(w as u32)
}

pub fn change_screen_width(w: u32) {
    screen_width().set(w);
}
pub fn on_logged_out_msg() {
    login_user().take();
    local_storage().remove("user");
    crate::router::router().go(crate::router::Route::Home);
}
pub async fn auth_token() -> Option<AuthToken> {
    Some(login_user().lock_ref().as_ref()?.auth_token.clone())
}
fn logged_user() -> impl Signal<Item = Option<User>> {
    login_user().signal_cloned()
}

pub fn is_user_logged() -> bool {
    if login_user().get_cloned().is_some() {
        return true;
    }
    false
}
pub fn is_user_admin() -> bool {
    if let Some(u) = login_user().get_cloned() {
        return u.is_admin;
    }
    false
}
pub fn is_admin() -> impl Signal<Item= bool> {
    if let Some(u) = login_user().get_cloned() {
        return Mutable::new(u.is_admin).signal();
    }
    Mutable::new(false).signal()
}
///-----------
// Functions
///-----------

pub fn set_page_id(page: Pages) {
    pages().set(page)
}

pub fn load_logged_user() {
    if let Some(Ok(user)) = local_storage().get("user") {
        login_user().set(Some(user));
        crate::app::login::get_school();
    }
}
