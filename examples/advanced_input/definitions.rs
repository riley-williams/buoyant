use core::net::Ipv4Addr;

use buoyant::{
    event::input::{Deactivation, InputRef},
    primitives::Interpolate,
};
use embedded_graphics::prelude::PixelColor;

pub trait GoodPixelColor: PixelColor + Interpolate + 'static {}

impl<T: PixelColor + Interpolate + 'static> GoodPixelColor for T {}

pub const MAX_COLS: usize = 5;
pub const MAX_ROWS: usize = 6;

pub const PALETTE_SIZE: usize = 11;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageAction {
    Next,
    Prev,
}

#[derive(Debug, Clone)]
pub struct State {
    pub static_ip: Ipv4Addr,
    pub gateway: Ipv4Addr,
    pub dns: Ipv4Addr,
    pub net_mask: u8,
    pub dhcp: bool,
    pub page_action: Option<PageAction>,

    pub(crate) opened_input: Option<(IpType, Deactivation)>,

    pub(crate) temporary_ip: TemporaryIp,
}

#[derive(Debug, Clone, Copy)]
pub struct Octet(pub [u8; 3]);
#[derive(Debug, Clone, Copy)]
pub struct TemporaryIp(pub [Octet; 4]);

#[derive(Debug, Clone, Copy)]
pub enum IpType {
    StaticIp,
    Gateway,
    Dns,
}

pub enum Color {
    Black,
    Blue,
    DarkBlue,
    DarkGray,
    Green,
    LightGray,
    Orange,
    Red,
    White,
    Yellow,
    Purple,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palette<C> {
    palette: [C; PALETTE_SIZE],
}

#[derive(Debug, Clone, Copy)]
pub struct RenderData<'a, C: GoodPixelColor> {
    pub palette: &'static Palette<C>,
    pub page: Page<'a>,
    pub input: InputRef<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HwCell {
    Analog(&'static str, u8), // percentage 0..=100
    Digital(&'static str, bool),
    DoubleDigital(&'static str, bool, bool),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IeName<'a> {
    Known(&'a str),
    Addr((u8, u8), (u8, u8, u8)),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Page<'a> {
    IeTable {
        header: &'a [HwCell],
        footer: &'a [HwCell],

        table_dimensions: (usize, usize),
        names: &'a [Option<IeName<'a>>],
        ie: &'a [Option<f32>],
        eu: &'a [Option<&'static str>],
    },
    Settings {
        header: &'a [HwCell],
        footer: &'a [HwCell],
        // todo: display current ip, set static ip, dhcp on/off
    },
}

impl<C: Copy> Palette<C> {
    pub const fn from_array(array: [C; PALETTE_SIZE]) -> Self {
        Self { palette: array }
    }

    pub fn black(&self) -> C {
        self.palette[Color::Black as usize]
    }
    pub fn blue(&self) -> C {
        self.palette[Color::Blue as usize]
    }
    pub fn dark_blue(&self) -> C {
        self.palette[Color::DarkBlue as usize]
    }
    pub fn dark_gray(&self) -> C {
        self.palette[Color::DarkGray as usize]
    }
    pub fn green(&self) -> C {
        self.palette[Color::Green as usize]
    }
    pub fn light_gray(&self) -> C {
        self.palette[Color::LightGray as usize]
    }
    pub fn orange(&self) -> C {
        self.palette[Color::Orange as usize]
    }
    pub fn red(&self) -> C {
        self.palette[Color::Red as usize]
    }
    pub fn white(&self) -> C {
        self.palette[Color::White as usize]
    }
    pub fn yellow(&self) -> C {
        self.palette[Color::Yellow as usize]
    }
    pub fn purple(&self) -> C {
        self.palette[Color::Purple as usize]
    }
}

impl Default for HwCell {
    fn default() -> Self {
        Self::Digital("-", false)
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            static_ip: Ipv4Addr::UNSPECIFIED,
            gateway: Ipv4Addr::UNSPECIFIED,
            dns: Ipv4Addr::UNSPECIFIED,
            net_mask: 0,
            dhcp: true,
            page_action: None,

            opened_input: None,
            temporary_ip: Ipv4Addr::UNSPECIFIED.into(),
        }
    }
}

impl From<u8> for Octet {
    fn from(v: u8) -> Self {
        let [a, b, c] = [v / 100, (v % 100) / 10, v % 10];
        Self([a, b, c])
    }
}

impl TryFrom<Octet> for u8 {
    type Error = &'static str;
    fn try_from(Octet([a, b, c]): Octet) -> Result<Self, &'static str> {
        if a > 2 || a == 2 && b > 5 || (a, b) == (2, 5) && c > 5 {
            return Err("Invalid octet value");
        }
        Ok(a * 100 + b * 10 + c)
    }
}

impl From<Ipv4Addr> for TemporaryIp {
    fn from(ip: Ipv4Addr) -> Self {
        Self(ip.octets().map(Octet::from))
    }
}

impl TryFrom<TemporaryIp> for Ipv4Addr {
    type Error = &'static str;
    fn try_from(octets: TemporaryIp) -> Result<Self, Self::Error> {
        let mut arr = [0u8; 4];
        for (dst, src) in arr.iter_mut().zip(octets.0.into_iter()) {
            *dst = src.try_into()?;
        }
        Ok(Self::from(arr))
    }
}
