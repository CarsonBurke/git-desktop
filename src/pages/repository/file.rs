use iced::{
    widget::{column, container, text_editor},
    Element,
};

#[derive(Debug, Clone)]
pub enum FileMessage {}

#[derive(Debug)]
struct TextEditorContent(text_editor::Content);

impl TextEditorContent {
    pub fn new() -> Self {
        TextEditorContent(text_editor::Content::new())
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
        let content = column![text_editor(&self.file_content.0)];
        let container = container(content);
        container.into()
    }
}
