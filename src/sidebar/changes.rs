use iced::{
    alignment,
    border::Radius,
    widget::{self, column, container, horizontal_space, row, text},
    Background, Border, Element, Length,
};
use iced_aw::BootstrapIcon;

use crate::{
    constants::{
        custom_theme::{BACKGROUND_3, BACKGROUND_4},
        font_sizes, spacings,
    },
    general_widgets::{
        icons::bootstrap_icon,
        seperators::{seperator_background_1, seperator_background_2},
        split_table_single::split_table_single,
    },
    styles,
};

use super::sidebar::DisplayState;

#[derive(Debug, Clone, Copy)]
pub enum ChangesMessage {}

pub struct Changes {
    pub display_state: DisplayState,
}

impl Changes {
    pub fn new() -> Self {
        Self {
            display_state: DisplayState::Visible,
        }
    }

    pub fn view(&self) -> Element<ChangesMessage> {
        let content = container(column![
            row![
                container(text("6").size(font_sizes::H3))
                    .padding(spacings::XSMALL)
                    .style(styles::container::background2_rounded()),
                text("Changes").size(font_sizes::H1),
                horizontal_space(),
                bootstrap_icon(BootstrapIcon::ChevronDown).size(font_sizes::H1)
            ]
            .align_items(iced::Alignment::Center)
            .spacing(spacings::SMALL)
            .padding(spacings::SMALL)
            .width(Length::Fill),
            seperator_background_2(),
            row![bootstrap_icon(BootstrapIcon::Code), text("File name")]
                .spacing(spacings::SMALL)
                .padding(spacings::SMALL),
        ])
        .style(changes_parent());
        let container = container(content);
        container.into()
    }
}

pub fn changes_parent() -> widget::container::Appearance {
    widget::container::Appearance {
        background: Some(Background::from(BACKGROUND_3)),
        border: Border {
            radius: 8.0.into(),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn changes_header() -> widget::container::Appearance {
    widget::container::Appearance {
        background: Some(Background::from(BACKGROUND_4)),
        ..Default::default()
    }
}
