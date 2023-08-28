use zoon::*;
use crate::{i18n::{t, lang, self}, app::{self, school::school, is_admin, is_user_admin}, router::Route};

pub fn root() -> impl Element {
    Row::new()
    .item(left_nav())
    .item(right_nav())
}

fn left_nav()-> impl Element{
    Row::new()
    .s(Align::new().left())
    .s(Gap::new().x(10))
    .item(
        Link::new()
        .label(
            Label::new().label("Libredu")
            .s(Font::new().weight(FontWeight::Medium))
        ).to(Route::Home)
    )
    .item_signal(
        school().signal_cloned().map_some(|_|
            Link::new().to("https://timetabling.libredu.org").new_tab(NewTab::new()).label("Timetabling")
        )
    ).item_signal(
        is_user_admin().map_true(|| 
            Link::new()
            .label("Admin")
            .to(Route::Admin).on_click(||{
                crate::app::admin::school::school().set(None);
                crate::app::admin::timetables::timetable().set(None);
            })
        )
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
                        .s(Gap::new().x(5))
                        .item(
                            Link::new().label(&u.first_name).to(Route::User)
                        ).item(
                            Column::new()
                            .s(Cursor::new(CursorIcon::Pointer))
                            .item(
                                Link::new().label("").update_raw_el(|raw_el|
                                    raw_el.attr("class", "fa-solid fa-arrow-right-from-bracket")
                                )
                                .to(Route::Logout)
                            )
                            
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
