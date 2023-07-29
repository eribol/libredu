use shared::UpMsg;
use zoon::{*, named_color::RED_7};

pub fn del_modal_all(modal_id: &str, msg: UpMsg) -> impl zoon::Element
{
    //run_once!(|| {
    //    global_styles().style_group(StyleGroup::new(".below > *").style("pointer-events", "auto"));
    //});
    //use zoon::HasIds;
    zoon::Row::new()
        .id(modal_id)
        //.s(Background::new().color(hsluv!(200,100,100)))
        .s(Borders::all(Border::new().width(1).solid()))
        //.s(zoon::Width::exact(50))
        .s(zoon::Align::new().right())
        .s(zoon::Padding::all(5))
        .s(Gap::new().x(10))
        .on_click_outside_with_ids(move || del_modal().set(None), [modal_id])
        .after_remove(|_| del_modal().set(None))
        .item(
            Button::new()
            .s(Font::new().color(RED_7).weight(FontWeight::Bold))
            .label("Sil").on_click(move || send_msg(msg.clone()))
        )
        .item(
            Button::new().label("İptal").on_click(move || del_modal().set(None))
        )
        .update_raw_el(|raw_el| {
            raw_el
                .class("below")
                .style("display", "flex")
                .style("flex-direction", "row")
                .style("position", "absolute")
                .style("top", "100%")
                .style("left", "0")
                //.style("width", "100%")
                .style("pointer-events", "none")
                .style("z-index", "100")
        })
}

#[static_ref]
pub fn del_modal() -> &'static Mutable<Option<i32>> {
    Mutable::new(None)
}

pub fn del_signal(id: i32)-> impl Signal<Item = bool>{
    del_modal().signal().map_option(move |s| s == id, || false).dedupe()
}

fn send_msg(msg: UpMsg) {
    use crate::connection::*;
    Task::start(async {
        match connection().send_up_msg(msg).await {
            Err(_error) => {}
            Ok(_msg) => (),
        }
    });
}

/*
pub fn update_modal<F, RE: RawEl>(modal_id: &str, element: F) -> impl zoon::Element
where F: FnMut()-> Column<EmptyFlagNotSet, RE>
{
    run_once!(|| {
        global_styles().style_group(StyleGroup::new(".below > *").style("pointer-events", "auto"));
    });
    //use zoon::HasIds;
    element()
        .id(modal_id)
        .s(Background::new().color(hsluv!(200,100,100)))
        .s(Borders::all(Border::new().width(1).solid()))
        //.s(zoon::Width::exact(50))
        .s(zoon::Align::new().right())
        .s(zoon::Padding::all(5))
        .s(Gap::new().x(10))
        .on_click_outside_with_ids(move || del_modal().set(None), [modal_id])
        .after_remove(|_| del_modal().set(None))
        .item(
            Button::new()
            .s(Font::new().color(RED_7).weight(FontWeight::Bold))
            .label("Sil").on_click(move || ())
        )
        .item(
            Button::new().label("İptal").on_click(move || del_modal().set(None))
        )
        .update_raw_el(|raw_el| {
            raw_el
                .class("below")
                .style("display", "flex")
                .style("flex-direction", "row")
                .style("position", "absolute")
                .style("top", "100%")
                .style("left", "0")
                //.style("width", "100%")
                .style("pointer-events", "none")
                .style("z-index", "100")
        })
}
*/