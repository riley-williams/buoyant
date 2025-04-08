use u8g2_fonts::{
    fonts::{
        u8g2_font_helvB12_tr, u8g2_font_helvB14_tr, u8g2_font_helvB18_tr, u8g2_font_helvR08_tr,
        u8g2_font_helvR12_tr, u8g2_font_helvR18_tr,
    },
    FontRenderer,
};

pub static TITLE: FontRenderer = FontRenderer::new::<u8g2_font_helvR18_tr>();
pub static TITLE_BOLD: FontRenderer = FontRenderer::new::<u8g2_font_helvB18_tr>();
pub static SUBTITLE: FontRenderer = FontRenderer::new::<u8g2_font_helvB14_tr>();
pub static BODY: FontRenderer = FontRenderer::new::<u8g2_font_helvR12_tr>();
pub static BODY_BOLD: FontRenderer = FontRenderer::new::<u8g2_font_helvB12_tr>();
pub static FOOTNOTE: FontRenderer = FontRenderer::new::<u8g2_font_helvR08_tr>();
