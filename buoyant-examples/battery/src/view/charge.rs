use buoyant::{
    layout::{Alignment, HorizontalAlignment},
    view::{
        padding::Edges,
        shape::{Circle, RoundedRectangle},
        FitAxis, HStack, HorizontalTextAlignment, Spacer, Text, VStack, View, ViewExt,
        ViewThatFits, ZStack,
    },
};
use core::fmt::Write;
use embedded_graphics::mono_font::ascii::{FONT_10X20, FONT_6X13, FONT_6X9, FONT_8X13_BOLD};

use crate::{
    color::{self, ColorFormat},
    state::{BatteryStatus, ChargeEstimate, PortState},
};

const SPACING: u32 = 3;

#[must_use]
pub fn view(battery: &BatteryStatus) -> impl View<ColorFormat> {
    ViewThatFits::new(FitAxis::Vertical, {
        VStack::new((
            VStack::new((
                charge_gauge(battery.state_of_charge()),
                Text::new(format_charge_time(battery), &FONT_6X13).padding(Edges::Bottom, SPACING),
            ))
            .with_spacing(SPACING),
            port_power_view(&battery.ports),
        ))
        .with_spacing(SPACING)
    })
    .or({
        HStack::new((
            VStack::new((
                charge_gauge(battery.state_of_charge()),
                Text::new(format_charge_time(battery), &FONT_6X13).padding(Edges::Bottom, SPACING),
            ))
            .with_spacing(SPACING),
            port_power_view(&battery.ports),
        ))
        .with_spacing(SPACING)
    })
}

fn charge_gauge(charge: f32) -> impl View<ColorFormat> {
    let mut formatted_charge = heapless::String::<8>::new();
    _ = write!(formatted_charge, "{charge:.0}"); // ignore write failure
    ZStack::new((
        Circle.foreground_color(color::GREEN),
        Circle
            .foreground_color(color::BACKGROUND)
            .padding(Edges::All, 8),
        VStack::new((
            Text::new(formatted_charge, &FONT_10X20),
            Text::new("%", &FONT_6X9),
        ))
        .with_spacing(2)
        .foreground_color(color::CONTENT),
    ))
}

fn port_power_view(power: &PortState) -> impl View<ColorFormat> {
    VStack::new((
        port_power_row("C1", power.usbc1_power as i32),
        port_power_row("C2", power.usbc2_power as i32),
        port_power_row("A", power.usba_power as i32),
    ))
    .with_spacing(SPACING)
}

#[expect(clippy::cast_precision_loss)]
fn port_power_row(port_name: &str, power: i32) -> impl View<ColorFormat> + use<'_> {
    let mut formatted_power = heapless::String::<8>::new();
    // save a few thousand cycles by not being lazy with fp
    _ = write!(formatted_power, "{:.1}w", power.abs() as f32); // ignore write failure
    HStack::new((
        Text::new(port_name, &FONT_6X13)
            .foreground_color(color::CONTENT)
            .flex_frame()
            .with_min_width(6 * 2)
            .padding(Edges::Vertical, 5)
            .padding(Edges::Leading, 5),
        match power {
            p if p > 0 => Text::new("^", &FONT_6X13),
            p if p < 0 => Text::new("*", &FONT_6X13),
            _ => Text::new("-", &FONT_6X13),
        },
        Spacer::default(),
        Text::new(formatted_power, &FONT_8X13_BOLD)
            .multiline_text_alignment(HorizontalTextAlignment::Trailing)
            .foreground_color(if power == 0 {
                color::GREY
            } else {
                color::BLACK
            })
            .padding(Edges::All, 5)
            .flex_infinite_width(HorizontalAlignment::Trailing)
            .with_infinite_max_height()
            .background(Alignment::default(), || {
                RoundedRectangle::new(5).foreground_color(match power {
                    p if p > 0 => color::BLUE,
                    p if p < 0 => color::GREEN,
                    _ => color::SECONDARY_BACKGROUND,
                })
            }),
    ))
    .with_spacing(SPACING * 2)
    .padding(Edges::All, 3)
    .background(Alignment::default(), || {
        RoundedRectangle::new(8).foreground_color(color::SECONDARY_BACKGROUND)
    })
}

fn format_charge_time(battery: &BatteryStatus) -> heapless::String<16> {
    let mut formatted_charge_time = heapless::String::<16>::new();
    match battery.charge_estimate() {
        ChargeEstimate::Discharging(time) | ChargeEstimate::Charging(time) => {
            _ = write!(formatted_charge_time, "{}", format_time(time));
        }
        ChargeEstimate::Idle => {
            _ = write!(formatted_charge_time, "--");
        }
    }
    formatted_charge_time
}

fn format_time(time: f32) -> heapless::String<8> {
    let mut formatted_time = heapless::String::<8>::new();
    let minutes = time / 60.0;
    if minutes >= 1.0 {
        _ = write!(formatted_time, "{}m ", minutes as i32);
    }
    let seconds = time % 60.0;
    _ = write!(formatted_time, "{}s", seconds as i32);
    formatted_time
}

#[cfg(test)]
mod tests {
    use super::format_time;

    #[test]
    fn format_0() {
        let s = format_time(0.0);
        assert_eq!(s.as_str(), "0s");
    }

    #[test]
    fn format_1s() {
        let s = format_time(1.0);
        assert_eq!(s.as_str(), "1s");
    }

    #[test]
    fn format_60s() {
        let s = format_time(60.0);
        assert_eq!(s.as_str(), "1m 0s");
    }

    #[test]
    fn format_61s() {
        let s = format_time(61.0);
        assert_eq!(s.as_str(), "1m 1s");
    }

    #[test]
    fn format_3600s() {
        let s = format_time(3600.0);
        assert_eq!(s.as_str(), "60m 0s");
    }
}
