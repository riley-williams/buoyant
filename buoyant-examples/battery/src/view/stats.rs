use buoyant::{
    layout::HorizontalAlignment,
    view::{FitAxis, HStack, Text, VStack, View, ViewExt, ViewThatFits},
};

use crate::{
    color::{self, ColorFormat},
    font,
    state::BatteryStatus,
};

#[must_use]
pub fn view(_battery: &BatteryStatus) -> impl View<ColorFormat> {
    ViewThatFits::new(FitAxis::Vertical, {
        VStack::new((
            labeled_pair("Temperature", "23 C / 73 F", HorizontalAlignment::Leading),
            labeled_pair("Battery Health", "100 %", HorizontalAlignment::Leading),
            labeled_pair("Total Input", "12317 wh", HorizontalAlignment::Leading),
            labeled_pair("Battery Cycles", "142", HorizontalAlignment::Leading),
            labeled_pair("Total Output", "12247 wh", HorizontalAlignment::Leading),
            labeled_pair("Screen Uses", "3460", HorizontalAlignment::Leading),
        ))
    })
    .or({
        VStack::new((
            HStack::new((
                labeled_pair("Temperature", "23 C / 73 F", HorizontalAlignment::Leading),
                labeled_pair("Battery Health", "100 %", HorizontalAlignment::Trailing),
            )),
            HStack::new((
                labeled_pair("Total Input", "12317 wh", HorizontalAlignment::Leading),
                labeled_pair("Battery Cycles", "142", HorizontalAlignment::Trailing),
            )),
            HStack::new((
                labeled_pair("Total Output", "12247 wh", HorizontalAlignment::Leading),
                labeled_pair("Screen Uses", "3460", HorizontalAlignment::Trailing),
            )),
        ))
    })
}

#[must_use]
pub fn labeled_pair<'a>(
    label: &'a str,
    value: &'a str,
    alignment: HorizontalAlignment,
) -> impl View<ColorFormat> + use<'a> {
    VStack::new((
        Text::new(value, &font::BODY_BOLD).foreground_color(color::CONTENT),
        Text::new(label, &font::FOOTNOTE).foreground_color(color::SECONDARY_CONTENT),
    ))
    .with_alignment(alignment)
    .flex_infinite_width(alignment)
    .with_infinite_max_height()
}
