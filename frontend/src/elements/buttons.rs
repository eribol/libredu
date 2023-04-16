use zoon::{
    button::{LabelFlagSet, OnPressFlagNotSet},
    named_color::*,
    *,
};

pub fn default_with_signal<'a>(
    label: impl Signal<Item = impl IntoElement<'a>> + Unpin + 'static,
) -> Button<LabelFlagSet, OnPressFlagNotSet, RawHtmlEl<web_sys::HtmlDivElement>> {
    let (a, _b) = Mutable::new_and_signal_cloned(false);
    Button::new()
        .s(Borders::all_signal(a.signal().map_bool(
            || Border::new().width(1).color(BLUE_3).solid(),
            || Border::new().width(1).color(BLUE_1).solid(),
        )))
        .s(Height::exact(50))
        .s(RoundedCorners::all(2))
        .label(Label::new().label_signal(label).s(Align::center()))
        .on_focused_change(move |hovered| a.set(hovered))
}

pub fn _default(
    label: &str,
) -> Button<LabelFlagSet, OnPressFlagNotSet, RawHtmlEl<web_sys::HtmlDivElement>> {
    let (a, _b) = Mutable::new_and_signal_cloned(false);
    Button::new()
        .s(Borders::all_signal(a.signal().map_bool(
            || Border::new().width(1).color(BLUE_3).solid(),
            || Border::new().width(1).color(BLUE_1).solid(),
        )))
        .s(Height::exact(50))
        .s(RoundedCorners::all(2))
        .label(
            Label::new()
                .label(label)
                .s(Align::center())
                .s(Font::new().weight(FontWeight::Light)),
        )
        .on_focused_change(move |hovered| a.set(hovered))
}
