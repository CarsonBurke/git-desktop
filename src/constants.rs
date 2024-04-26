pub static ICON: &[u8] = include_bytes!("icon.png");

pub mod spacings {
    pub const XSMALL: u16 = 4;
    pub const SMALL: u16 = 8;
    pub const MEDIUM: u16 = 16;
    pub const LARGE: u16 = 32;
    pub const X_LARGE: u16 = 64;
}

pub mod font_sizes {
    pub const H1: u16 = 17;
    pub const H2: u16 = 15;
    pub const H3: u16 = 13;
    pub const P: u16 = 13;
}

pub mod custom_theme {
    use iced::Color;

    pub const PRIMARY: Color = Color {
        r: 0.21,
        g: 0.52,
        b: 0.89,
        a: 1.,
    };

    pub const BACKGROUND_1: Color = Color {
        r: 0.12,
        g: 0.12,
        b: 0.12,
        a: 1.,
    };

    pub const BACKGROUND_2: Color = Color {
        r: 0.16,
        g: 0.16,
        b: 0.16,
        a: 1.,
    };

    pub const BACKGROUND_3: Color = Color {
        r: 0.20,
        g: 0.20,
        b: 0.20,
        a: 1.,
    };

    pub const BACKGROUND_4: Color = Color {
        r: 0.24,
        g: 0.24,
        b: 0.24,
        a: 1.,
    };

    pub const BACKGROUND_5: Color = Color {
        r: 0.28,
        g: 0.28,
        b: 0.28,
        a: 1.,
    };

    pub const TEXT: Color = Color {
        r: 1.,
        g: 1.,
        b: 1.,
        a: 1.,
    };

    pub const GREY_TEXT: Color = Color {
        r: 0.7,
        g: 0.7,
        b: 0.7,
        a: 1.,
    };
}