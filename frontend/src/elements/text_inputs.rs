use zoon::{
    text_input::{
        IdFlagNotSet, InputTypeFlagNotSet, LabelFlagNotSet, OnChangeFlagNotSet,
        PlaceholderFlagNotSet, ReadOnlyFlagNotSet, TextFlagNotSet,
    },
    TextInput, *,
};

pub fn default() -> TextInput<
    IdFlagNotSet,
    OnChangeFlagNotSet,
    PlaceholderFlagNotSet,
    TextFlagNotSet,
    LabelFlagNotSet,
    InputTypeFlagNotSet,
    ReadOnlyFlagNotSet,
    RawHtmlEl<web_sys::HtmlInputElement>,
> {
    //f("a".to_string());
    let (a, b) = Mutable::new_and_signal_cloned(false);
    TextInput::new()
        .s(Align::center())
        .s(Height::exact(30))
        //.s(Borders::all(Border::new().solid().color(BLUE_5)))
        .s(Borders::all_signal(a.signal().map_bool(
            || Border::new().width(1).color(color!("blue")).solid(),
            || Border::new().width(1).color(color!("gray")).solid(),
        )))
        .s(RoundedCorners::all(5))
        .s(Shadows::with_signal(b.map_bool(
            || {
                [
                    Shadow::new().color(color!("blue")).y(1).blur(3),
                    Shadow::new().color(color!("blue")).y(-1).blur(3),
                ]
            },
            || {
                [
                    Shadow::new().color(color!("gray")).y(1).blur(1),
                    Shadow::new().color(color!("gray")).y(-1).blur(1),
                ]
            },
        )))
        .on_focused_change(move |hovered| a.set(hovered))
}
