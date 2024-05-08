use iced::{
    application::{Appearance, StyleSheet},
    theme::{self, Button},
    widget::{
        self,
        button::{self},
        container,
    },
    Background, Color, Theme,
};

use crate::constants::custom_theme;

pub fn primary() -> widget::button::Appearance {
    widget::button::Appearance {
        text_color: iced::Color::from_rgb(132.0, 123.0, 124.0),
        background: Some(Background::from(custom_theme::PRIMARY)),
        ..Default::default()
    }
}

pub fn button_appearance(theme: &Theme) -> widget::button::Appearance {
    widget::button::Appearance {
        text_color: iced::Color::from_rgb(132.0, 123.0, 124.0),
        background: Some(Background::from(Theme::palette(theme).background)),
        ..Default::default()
    }
}

pub fn primary_blended(theme: &Theme) -> widget::button::Appearance {
    widget::button::Appearance {
        text_color: iced::Color::from_rgb(132.0, 123.0, 124.0),
        background: Some(Background::from(Theme::palette(theme).background)),
        ..Default::default()
    }
}

pub fn secondary_blended(theme: &Theme) -> widget::button::Appearance {
    widget::button::Appearance {
        text_color: iced::Color::from_rgb(132.0, 123.0, 124.0),
        background: Some(Background::from(Theme::palette(theme).background)),
        ..Default::default()
    }
}

pub struct Primary {
    pub display_as_pressed: bool,
}

impl button::StyleSheet for Primary {
    type Style = iced::Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {

        if self.display_as_pressed == true {
            return self.hovered(style);
        }

        button::Appearance {
            background: Some(iced::Background::Color(custom_theme::PRIMARY)),
            text_color: iced::Color::WHITE,
            // shadow_offset: iced::Vector { x: 1.0, y: 1.0 },
            border: iced::Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn hovered(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(custom_theme::PRIMARY_DOWNLIGHT)),
            text_color: iced::Color::WHITE,
            shadow_offset: iced::Vector { x: 1.0, y: 1.0 },
            border: iced::Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn pressed(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(custom_theme::PRIMARY_HIGHLIGHT)),
            text_color: iced::Color::WHITE,
            shadow_offset: iced::Vector { x: 1.0, y: 1.0 },
            border: iced::Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

pub struct Background1 {
    pub display_as_pressed: bool,
}

impl button::StyleSheet for Background1 {
    type Style = iced::Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        if self.display_as_pressed == true {
            return self.hovered(style);
        }

        button::Appearance {
            background: Some(iced::Background::Color(custom_theme::BACKGROUND_1)),
            text_color: iced::Color::WHITE,
            // shadow_offset: iced::Vector { x: 1.0, y: 1.0 },
            border: iced::Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn hovered(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(custom_theme::BACKGROUND_2)),
            text_color: iced::Color::WHITE,
            shadow_offset: iced::Vector { x: 1.0, y: 1.0 },
            border: iced::Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn pressed(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(custom_theme::BACKGROUND_3)),
            text_color: iced::Color::WHITE,
            shadow_offset: iced::Vector { x: 1.0, y: 1.0 },
            border: iced::Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

pub struct Background2 {
    pub display_as_pressed: bool,
}

impl button::StyleSheet for Background2 {
    type Style = iced::Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        if self.display_as_pressed == true {
            return self.hovered(style);
        }

        button::Appearance {
            background: Some(iced::Background::Color(custom_theme::BACKGROUND_1)),
            text_color: iced::Color::WHITE,
            // shadow_offset: iced::Vector { x: 1.0, y: 1.0 },
            border: iced::Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn hovered(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(custom_theme::BACKGROUND_2)),
            text_color: iced::Color::WHITE,
            shadow_offset: iced::Vector { x: 1.0, y: 1.0 },
            border: iced::Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn pressed(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(custom_theme::BACKGROUND_3)),
            text_color: iced::Color::WHITE,
            shadow_offset: iced::Vector { x: 1.0, y: 1.0 },
            border: iced::Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}


pub struct Background3 {
    pub display_as_pressed: bool,
}

impl button::StyleSheet for Background3 {
    type Style = iced::Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        if self.display_as_pressed == true {
            return self.hovered(style);
        }

        button::Appearance {
            background: Some(iced::Background::Color(custom_theme::BACKGROUND_3)),
            text_color: iced::Color::WHITE,
            // shadow_offset: iced::Vector { x: 1.0, y: 1.0 },
            border: iced::Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn hovered(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(custom_theme::BACKGROUND_4)),
            text_color: iced::Color::WHITE,
            shadow_offset: iced::Vector { x: 1.0, y: 1.0 },
            border: iced::Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn pressed(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(custom_theme::BACKGROUND_5)),
            text_color: iced::Color::WHITE,
            shadow_offset: iced::Vector { x: 1.0, y: 1.0 },
            border: iced::Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

pub struct Background3Circle {
    pub display_as_pressed: bool,
}

impl button::StyleSheet for Background3Circle {
    type Style = iced::Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        if self.display_as_pressed == true {
            return self.hovered(style);
        }

        button::Appearance {
            background: Some(iced::Background::Color(custom_theme::BACKGROUND_3)),
            text_color: iced::Color::WHITE,
            // shadow_offset: iced::Vector { x: 1.0, y: 1.0 },
            border: iced::Border {
                radius: 1000.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn hovered(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(custom_theme::BACKGROUND_4)),
            text_color: iced::Color::WHITE,
            shadow_offset: iced::Vector { x: 1.0, y: 1.0 },
            border: iced::Border {
                radius: 1000.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn pressed(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(custom_theme::BACKGROUND_5)),
            text_color: iced::Color::WHITE,
            shadow_offset: iced::Vector { x: 1.0, y: 1.0 },
            border: iced::Border {
                radius: 1000.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

pub struct Background4 {
    pub display_as_pressed: bool,
}

impl button::StyleSheet for Background4 {
    type Style = iced::Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        if self.display_as_pressed == true {
            return self.hovered(style);
        }

        button::Appearance {
            background: Some(iced::Background::Color(custom_theme::BACKGROUND_4)),
            text_color: iced::Color::WHITE,
            // shadow_offset: iced::Vector { x: 1.0, y: 1.0 },
            border: iced::Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn hovered(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(custom_theme::BACKGROUND_5)),
            text_color: iced::Color::WHITE,
            shadow_offset: iced::Vector { x: 1.0, y: 1.0 },
            border: iced::Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn pressed(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(custom_theme::BACKGROUND_6)),
            text_color: iced::Color::WHITE,
            shadow_offset: iced::Vector { x: 1.0, y: 1.0 },
            border: iced::Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

pub struct Background5 {
    pub display_as_pressed: bool,
}

impl button::StyleSheet for Background5 {
    type Style = iced::Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        if self.display_as_pressed == true {
            return self.hovered(style);
        }

        button::Appearance {
            background: Some(iced::Background::Color(custom_theme::BACKGROUND_5)),
            text_color: iced::Color::WHITE,
            // shadow_offset: iced::Vector { x: 1.0, y: 1.0 },
            border: iced::Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn hovered(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(custom_theme::BACKGROUND_6)),
            text_color: iced::Color::WHITE,
            shadow_offset: iced::Vector { x: 1.0, y: 1.0 },
            border: iced::Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn pressed(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(custom_theme::BACKGROUND_7)),
            text_color: iced::Color::WHITE,
            shadow_offset: iced::Vector { x: 1.0, y: 1.0 },
            border: iced::Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}