use crate::{app::{self, Pages, forget_password, reset_password}, connection::send_msg};
use std::collections::VecDeque;
use shared::UpMsg;
use zoon::{*, println};

// ------ route_history ------

#[static_ref]
fn route_history() -> &'static Mutable<VecDeque<Route>> {
    Mutable::new(VecDeque::new())
}

fn push_to_route_history(route: Route) {
    let mut history = route_history().lock_mut();
    if history.len() == 2 {
        history.pop_back();
    }
    history.push_front(route);
}

pub fn previous_route() -> Option<Route> {
    route_history().lock_ref().get(1).cloned()
}

// ------ router ------

#[static_ref]
pub fn router() -> &'static Router<Route> {
    Router::new(|route: Option<Route>| async move {
        let route = match route {
            Some(route) => {
                push_to_route_history(route.clone());
                route
            }
            None => Route::Home,
        };

        match route {
            Route::Login => {
                if app::is_user_logged() {
                    return router().replace(Route::Home);
                }
                app::set_page_id(Pages::Login);
            }
            Route::ForgetPassword => {
                if app::is_user_logged() {
                    return router().replace(Route::Home);
                }
                app::set_page_id(Pages::ForgetPassword);
            }
            Route::ResetPassword{token, email} => {
                reset_password::email().set(email);
                reset_password::token().set(token);
                app::set_page_id(Pages::ResetPassword);
            }
            Route::Signin => {
                if app::is_user_logged() {
                    return router().replace(Route::Home);
                }
                app::set_page_id(Pages::Signin);
            }
            Route::Register{token, email} => {
                println!("{token:?}, {email:?}");
                send_msg(UpMsg::Register(token, email))
            }
            Route::Logout => {
                Task::start(async {
                    let msg = shared::UpMsg::Logout;
                    if let Err(_error) = crate::connection::connection().send_up_msg(msg).await {
                        return;
                    }
                    app::login_user().set(None);
                    local_storage().remove("user");
                    return router().replace(Route::Home);
                });
                //app::set_page_id(Pages::Login);
            }
            Route::Home => {
                app::set_page_id(Pages::Home);
            }
            Route::User => {
                app::set_page_id(Pages::User)
            }
        }
    })
}

#[route]
#[derive(Clone)]
pub enum Route {
    #[route("login")]
    Login,
    #[route("forget_password")]
    ForgetPassword,
    #[route("reset", token, email)]
    ResetPassword {token: String, email: String},
    #[route("signin")]
    Signin,
    #[route("logout")]
    Logout,
    #[route("register", token, email)]
    Register {token: String, email: String},
    #[route("user")]
    User,
    #[route()]
    Home,
}
