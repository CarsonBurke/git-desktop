use iced::widget::text_editor;

use crate::constants::custom_theme;

pub struct Background2 {

}

impl text_editor::StyleSheet for Background2 {
    type Style = iced::Theme;

    fn active(&self, style: &Self::Style) -> text_editor::Appearance {

        text_editor::Appearance {
            background: iced::Background::Color(custom_theme::BACKGROUND_2),
            // shadow_offset: iced::Vector { x: 1.0, y: 1.0 },
            border: iced::Border {
                ..Default::default()
            },
        }
    }
    
    fn focused(&self, style: &Self::Style) -> text_editor::Appearance {
        Self::active(&self, style)
    }
    
    fn placeholder_color(&self, style: &Self::Style) -> iced::Color {
        custom_theme::GREY_TEXT
    }
    
    fn value_color(&self, style: &Self::Style) -> iced::Color {
        custom_theme::GREY_TEXT
    }
    
    fn disabled_color(&self, style: &Self::Style) -> iced::Color {
        custom_theme::GREY_TEXT
    }
    
    fn selection_color(&self, style: &Self::Style) -> iced::Color {
        custom_theme::BACKGROUND_3
    }
    
    fn disabled(&self, style: &Self::Style) -> text_editor::Appearance {
        todo!()
    }
}