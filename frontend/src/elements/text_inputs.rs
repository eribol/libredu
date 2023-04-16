use zoon::{
    named_color::*,
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
            || Border::new().width(1).color(BLUE_3).solid(),
            || Border::new().width(1).color(GRAY_1).solid(),
        )))
        .s(RoundedCorners::all(5))
        .s(Shadows::with_signal(b.map_bool(
            || {
                [
                    Shadow::new().color(BLUE_3).y(1).blur(3),
                    Shadow::new().color(BLUE_3).y(-1).blur(3),
                ]
            },
            || {
                [
                    Shadow::new().color(GRAY_2).y(1).blur(1),
                    Shadow::new().color(GRAY_2).y(-1).blur(1),
                ]
            },
        )))
        .on_focused_change(move |hovered| a.set(hovered))
}
