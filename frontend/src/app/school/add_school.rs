use crate::i18n;
use zoon::{eprintln, *};

#[static_ref]
fn school_name() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

fn change_school_name(name: String) {
    school_name().set(name)
}

pub fn add_school_page() -> impl Element {
    Column::new()
        .s(Align::center())
        .s(Gap::new().y(15))
        .item(
            Label::new()
                .s(Align::center())
                .label("En az 5 karakterlik okul adı girin.")
                .s(Font::new().weight(FontWeight::ExtraLight)),
        )
        .item(
            Label::new()
                .s(Align::center())
                .label_signal(i18n::t!("add-school"))
                .s(Font::new().weight(FontWeight::SemiBold)),
        )
        .item(
            TextInput::new()
                .s(Align::center())
                .s(Height::exact(30))
                .s(Borders::all(Border::new().solid().color(color!("blue"))))
                .id("school_name")
                .placeholder(Placeholder::with_signal(i18n::t!("school-name")))
                .input_type(InputType::text())
                .on_change(change_school_name),
        )
        .item(
            Button::new()
                .s(Height::exact(35))
                .s(RoundedCorners::all(10))
                .s(Borders::all(Border::new().solid().color(color!("blue"))))
                .label(
                    El::new()
                        .s(Align::center())
                        .child_signal(i18n::t!("add-school")),
                )
                .on_click(add),
        )
}

fn add() {
    Task::start(async {
        use crate::connection::connection;
        if school_name().get_cloned().len() > 5 {
            let msg = shared::UpMsg::AddSchool {
                name: school_name().get_cloned(),
            };
            match connection().send_up_msg(msg).await {
                Err(error) => {
                    let error = error.to_string();
                    eprintln!("Add school failed: {}", error);
                    //set_login_error(error);
                }
                Ok(_) => (),
            }
        }
    });
}
