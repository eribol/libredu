use zoon::*;
use crate::{i18n::{t, lang, self}, app, router::Route};

pub fn root() -> impl Element {
    Row::new()
    .item(left_nav())
    .item(right_nav())
}

fn left_nav()-> impl Element{
    Row::new()
    .s(Align::new().left())
    .item(
        Link::new().label("Libredu").to(Route::Home)
    ) 
}

fn right_nav()-> impl Element{
    Row::new()
    .s(Gap::new().x(20))
    .item(lang_label())
    .item_signal(
        app::login_user().signal_ref(|user| 
            match user{
                Some(u) => {
                    Row::new()
                        .item(
                            Link::new().label(&u.first_name).to(Route::Logout)
                        )
                },
                None => {
                    Row::new()
                        .s(Gap::new().x(10))
                        .s(Align::new().right())
                        .item(
                            Link::new().label_signal(t!("signin")).to(Route::Signin)
                        )
                        .item(Link::new().label_signal(t!("login")).to(Route::Login))
                }
            })
        )
}

fn lang_label() -> impl Element{
    Button::new()
    .label_signal(
        lang()
        .signal_ref(|l| 
            l.label()
        )
    ).on_press(i18n::change_locale)
}