use iced::{
    theme, widget::{column, container, horizontal_space, row, text, text_editor}, Element
};
use iced_aw::BootstrapIcon;

use crate::{
    constants::{font_sizes, spacings},
    general_widgets::icons::bootstrap_icon,
    styles,
};

#[derive(Debug, Clone)]
pub enum FileMessage {
    EditText(text_editor::Action),
}

#[derive(Debug)]
struct TextEditorContent(text_editor::Content);

impl TextEditorContent {
    pub fn new() -> Self {
        TextEditorContent(text_editor::Content::with_text(
            "test / test
        test test
        
        
        test test",
        ))
    }
}

impl Clone for TextEditorContent {
    fn clone(&self) -> Self {
        TextEditorContent(text_editor::Content::with_text("text"))
    }
}

#[derive(Debug, Clone)]
pub struct File {
    file_content: TextEditorContent,
}

// https://github.com/iced-rs/iced/blob/master/examples/editor/src/main.rs

impl File {
    pub fn new() -> Self {
        Self {
            file_content: TextEditorContent::new(),
        }
    }

    pub fn view(&self) -> Element<FileMessage> {
        let top = container(
            row![
                bootstrap_icon(BootstrapIcon::Postage),
                text("File name").size(font_sizes::H2),
                horizontal_space(),
                bootstrap_icon(BootstrapIcon::Sliders)
            ]
            .padding(spacings::SMALL)
            .spacing(spacings::SMALL).align_items(iced::Alignment::Center),
        )
        .style(styles::container::test_background_3());

        let content = column![
            top,
            text_editor(&self.file_content.0)
                .on_action(FileMessage::EditText)
                .style(theme::TextEditor::Custom(Box::new(
                    styles::text_input::Background2 {}
                )))
        ];
        let container = container(content);
        container.into()
    }
}
