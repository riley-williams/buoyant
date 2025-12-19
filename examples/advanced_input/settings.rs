use crate::definitions::{GoodPixelColor, IpType, Octet, Palette, RenderData, State, TemporaryIp};
use buoyant::view::{Pagination, PaginationAction, prelude::*};
use core::{fmt::Write, net::Ipv4Addr};
use heapless::String;

use crate::{FONT, G0, G1};

// Rustc cannot infer certain types of closures so we help it a bit.
fn funnel<F: Fn(&mut State)>(f: F) -> F {
    f
}

pub fn settings<'a, 'b, C: GoodPixelColor, F: Fn(&State) + 'a>(
    data: RenderData<'a, C>,
    state: &'b State,
    save: F,
) -> impl View<C, State> + use<'a, C, F> {
    let p = data.palette;

    let i = data.input;
    let save = funnel(move |s| save(s));
    let toggle_dhcp = funnel(|s| s.dhcp = !s.dhcp);
    let enter_ip = |t: IpType| {
        funnel(move |s| {
            s.opened_input = Some((t, i.deactivate(G0)));
            s.temporary_ip = match t {
                IpType::StaticIp => s.static_ip.into(),
                IpType::Gateway => s.gateway.into(),
                IpType::Dns => s.dns.into(),
            }
        })
    };
    let set_ip = |t: IpType| {
        // activate
        funnel(move |s| {
            if let Ok(ip) = s.temporary_ip.try_into() {
                i.reset(G1); // todo: make guard to that atomically
                s.opened_input.take().map(|(_, d)| d.into_guard(i));
                match t {
                    IpType::StaticIp => s.static_ip = ip,
                    IpType::Gateway => s.gateway = ip,
                    IpType::Dns => s.dns = ip,
                }
            }
        })
    };
    let cancel_ip = funnel(move |s| {
        i.reset(G1); // todo: make guard to that atomically
        s.opened_input.take().map(|(_, d)| d.into_guard(i));
    });

    let static_ip = state.static_ip;
    let net_mask = state.net_mask;
    let gateway = state.gateway;
    let dns = state.dns;
    let dhcp = state.dhcp;

    let main = VStack::new((
        HStack::new((
            Text::new("Static IP", &FONT),
            Spacer::default(),
            Button::new_with_groups(enter_ip(IpType::StaticIp), G0, move |a| {
                button_gut(a, p, ip_view(static_ip))
            }),
        )),
        Spacer::default(),
        HStack::new((
            Text::new("Net Mask", &FONT),
            Spacer::default(),
            netmask(net_mask, p),
        )),
        Spacer::default(),
        HStack::new((
            Text::new("Gateway", &FONT),
            Spacer::default(),
            Button::new_with_groups(enter_ip(IpType::Gateway), G0, move |a| {
                button_gut(a, p, ip_view(gateway))
            }),
        )),
        Spacer::default(),
        HStack::new((
            Text::new("DNS", &FONT),
            Spacer::default(),
            Button::new_with_groups(enter_ip(IpType::Dns), G0, move |a| {
                button_gut(a, p, ip_view(dns))
            }),
        )),
        Spacer::default(),
        HStack::new((
            Text::new("DHCP", &FONT),
            Spacer::default(),
            Button::new_with_groups(toggle_dhcp, G0, move |a| {
                let text = if dhcp { "ON" } else { "OFF" };
                button_gut(a, p, Text::new(text, &FONT))
            }),
        )),
        Button::new_with_groups(save, G0, |a| button_gut(a, p, Text::new("Save", &FONT))),
    ));

    let ip = state.temporary_ip;
    let overlay = buoyant::match_view!( state.opened_input, {
        Some((IpType::StaticIp, _)) => ip_setter(ip, set_ip(IpType::StaticIp), cancel_ip, p),
        Some((IpType::Gateway, _)) => ip_setter(ip, set_ip(IpType::Gateway), cancel_ip, p),
        Some((IpType::Dns, _)) => ip_setter(ip, set_ip(IpType::Dns), cancel_ip, p),
        None => EmptyView,
    })
    .background(
        Alignment::Center,
        RoundedRectangle::new(5)
            .stroked(1)
            .foreground_color(p.light_gray()),
    )
    .background_color(p.dark_blue(), RoundedRectangle::new(5));

    // TODO: handle escape alongside submit
    ZStack::new((main, state.opened_input.as_ref().map(|_| overlay)))
        .with_alignment(Alignment::Center)
        .padding(Edges::All, 10)
}

fn ip_view<C: GoodPixelColor>(ip: Ipv4Addr) -> impl View<C, State> {
    // TODO: implement "display" version of `Text`. It would probably require multiple
    // writes of the same buffer, but sometimes it should be worth it.
    let [a, b, c, d] = ip.octets();
    let mut buf = String::<15, u8>::new();
    write!(&mut buf, "{a}.{b}.{c}.{d}").ok();
    Text::new(buf, &FONT)
}

fn ip_setter<C: GoodPixelColor>(
    ip: TemporaryIp,
    submit: impl Fn(&mut State),
    cancel: impl Fn(&mut State),
    palette: &'static Palette<C>,
) -> impl View<C, State> {
    fn set<const OCTET: usize, const INDEX: usize>(s: &mut State, v: u8) {
        const { assert!(INDEX < 3) };
        const { assert!(OCTET < 4) };

        s.temporary_ip.0[OCTET].0[INDEX] = v;
    }

    let TemporaryIp(
        [
            // -
            Octet([a0, a1, a2]),
            Octet([b0, b1, b2]),
            Octet([c0, c1, c2]),
            Octet([d0, d1, d2]),
        ],
    ) = ip;

    let or5_9 = |is_max: bool| if is_max { 5 } else { 9 };

    let [a1max, a2max] = [or5_9(a0 == 2), or5_9((a0, a1) == (2, 5))];
    let [b1max, b2max] = [or5_9(b0 == 2), or5_9((b0, b1) == (2, 5))];
    let [c1max, c2max] = [or5_9(c0 == 2), or5_9((c0, c1) == (2, 5))];
    let [d1max, d2max] = [or5_9(d0 == 2), or5_9((d0, d1) == (2, 5))];

    let a = HStack::new((
        ip_digit(a0, 2, set::<0, 0>, palette),
        ip_digit(a1, a1max, set::<0, 1>, palette),
        ip_digit(a2, a2max, set::<0, 2>, palette),
    ));
    let b = HStack::new((
        ip_digit(b0, 2, set::<1, 0>, palette),
        ip_digit(b1, b1max, set::<1, 1>, palette),
        ip_digit(b2, b2max, set::<1, 2>, palette),
    ));
    let c = HStack::new((
        ip_digit(c0, 2, set::<2, 0>, palette),
        ip_digit(c1, c1max, set::<2, 1>, palette),
        ip_digit(c2, c2max, set::<2, 2>, palette),
    ));
    let d = HStack::new((
        ip_digit(d0, 2, set::<3, 0>, palette),
        ip_digit(d1, d1max, set::<3, 1>, palette),
        ip_digit(d2, d2max, set::<3, 2>, palette),
    ));

    let ip = HStack::new((
        a,
        Text::new(".", &FONT),
        b,
        Text::new(".", &FONT),
        c,
        Text::new(".", &FONT),
        d,
    ));

    VStack::new((
        ip,
        Spacer::default().frame().with_height(10),
        Button::new_with_groups(submit, G1, |i| {
            button_gut(i, palette, Text::new("Submit", &FONT))
        }),
    ))
    .on_cancel(cancel)
    .padding(Edges::Vertical, 5)
    .padding(Edges::Horizontal, 10)
}

fn ip_digit<C: GoodPixelColor>(
    num: u8,
    max: u8,
    set_ditit: impl Fn(&mut State, u8) + 'static,
    palette: &'static Palette<C>,
) -> impl View<C, State> {
    #[derive(Copy, Clone)]
    struct Num(u8);

    #[rustfmt::skip]
    impl AsRef<str> for Num {
        fn as_ref(&self) -> &str {
            match self.0 {
                0 => "0", 1 => "1", 2 => "2", 3 => "3", 4 => "4",
                5 => "5", 6 => "6", 7 => "7", 8 => "8", 9 => "9", _ => "?",
            }
        }
    }

    use PaginationAction as A;

    let on_action = move |a, s: &mut State| match (a, num) {
        (A::Previous, _) if num == max => set_ditit(s, 0),
        (A::Previous, _) => set_ditit(s, num + 1),
        (A::Next, 0) => set_ditit(s, max),
        (A::Next, _) => set_ditit(s, num - 1),
        (A::Submit | A::Enter | A::Escape, _) => (),
    };

    Pagination::new_vertical::<_, _, State>(G1, on_action, move |i| {
        button_gut(i, palette, Text::new(Num(num), &FONT))
    })
}

fn netmask<C: GoodPixelColor>(mask: u8, palette: &'static Palette<C>) -> impl View<C, State> {
    use PaginationAction as A;

    let on_action = |a, s: &mut State| match (a, s.net_mask) {
        (A::Previous, 32) => s.net_mask = 8,
        (A::Previous, _) => s.net_mask += 1,
        (A::Next, 8) => s.net_mask = 32,
        (A::Next, _) => s.net_mask -= 1,
        (A::Submit | A::Enter | A::Escape, _) => (),
    };

    Pagination::new_vertical::<_, _, State>(G0, on_action, move |i| {
        let mut buf = String::<3, u8>::new();
        write!(&mut buf, "/{mask}").ok();
        button_gut(i, palette, Text::new(buf, &FONT))
    })
    .click_to_enter(true)
    .click_to_exit(true)
}

fn button_gut<C: GoodPixelColor>(
    i: buoyant::event::input::Interaction,
    palette: &'static Palette<C>,
    inner: impl View<C, State>,
) -> impl View<C, State> {
    let inner = inner.padding(Edges::All, 3);
    buoyant::if_view!((i.is_focused()) {
        inner.background(
            Alignment::Center,
            RoundedRectangle::new(3)
                .stroked(1)
                .foreground_color(palette.light_gray())
        )
    } else {
        inner
    })
}
