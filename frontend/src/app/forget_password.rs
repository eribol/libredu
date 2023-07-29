use crate::i18n;
use crate::router::Route;
use zoon::named_color::BLUE_5;
use zoon::*;


#[static_ref]
pub fn is_sent() -> &'static Mutable<bool> {
    Mutable::new(false)
}
#[static_ref]
fn email() -> &'static Mutable<String> {
    Mutable::new(String::from(""))
}

#[static_ref]
fn password() -> &'static Mutable<String> {
    Mutable::new(String::from(""))
}

fn set_email(e: String) {
    email().set(e)
}
fn is_sent_view()->impl Element{
    Column::new()
    .item(
        "Reset linki e-posta adresinize gönderildi"
    )
}
pub fn root()->impl Element{
    Column::new()
    .item_signal(
        is_sent().signal().map_true(is_sent_view)
    )
    .item_signal(
        is_sent().signal().map_false(forget_password)
    )
}
pub fn forget_password()->impl Element{
    Column::new()
    .s(Align::center())
    .s(Gap::new().y(15))
    .item(
        Label::new()
        .s(Align::center())
        .label_signal(i18n::t!("login"))
        .s(Font::new().weight(FontWeight::SemiBold)),
    )
    .item(
        TextInput::new()
                .s(Align::center())
                .s(Borders::all(Border::new().solid().color(BLUE_5)))
                .s(Height::exact(30))
                .id("email")
                .input_type(InputType::text())
                .placeholder(Placeholder::with_signal(i18n::t!("email")))
                .on_change(set_email),
        )
        .item(
            Button::new()
                .s(Height::exact(35))
                .s(RoundedCorners::all(10))
                .s(Borders::all(Border::new().solid().color(BLUE_5)))
                .label(El::new().s(Align::center()).child_signal(i18n::t!("login")))
                .on_click(send_mail),
        )
        .item(
           Row::new()
           .item(
                Link::new().label("Sign in").to(Route::Signin)
           )
           .item("  ")
           .item(
               Link::new().label("Login").to(Route::Login)
            )
        )
}

fn send_mail() {
    use crate::connection::*;
    use shared::*;
    is_sent().set(true);
    Task::start(async {
        let msg = UpMsg::ForgetPassword { email: email().get_cloned() };
        match connection().send_up_msg(msg).await {
            Err(_error) => {
            }
            Ok(_msg) => (),
        }
    });
}
