use std::error::Error;

use iced::window::screenshot::CropError;
use iced_aw::BootstrapIcon;

use crate::constants::language_icons;

#[derive(Debug)]
pub enum IconError {
    NoIconForExtension,
}

pub fn get_icon_for_file_extension(extension: String) -> Result<&'static [u8], IconError> {

    let extension_str = extension.as_str();

    if extension_str == "rs" {
        return Ok(language_icons::RUST);
    }

    Err(IconError::NoIconForExtension)
}