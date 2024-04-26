use iced::{executor, widget::{column, container}, Application, Color, Command, Element, Subscription, Theme};

use crate::{sidebar::sidebar::{Sidebar, SidebarMessage}, theme::CustomThemeChoice};

#[derive(Debug)]
pub enum AppMessage {
    SidebarMessage(SidebarMessage),
}

pub struct App {
    sidebar: Sidebar,
}

impl Application for App {
    type Message = AppMessage;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<AppMessage>) {

        (Self {
            sidebar: Sidebar::new(),
        }, Command::none())
    }

    fn title(&self) -> String {
        String::from("Git Desktop")
    }

    fn update(&mut self, message: AppMessage) -> Command<AppMessage> {

        match message {
            AppMessage::SidebarMessage(sidebar_message) => {
                println!("sidebar message {:?}", sidebar_message);
            }
        }

        Command::none()
    }

/*     fn subscription(&self) -> Subscription<AppMessage> {
        
    } */

    fn view(&self) -> Element<AppMessage> {

        let main = container(column![]);

        let container = container(column![self.sidebar.view().map(move |message| {
            AppMessage::SidebarMessage(message)
        }), main]);
        container.into()
    }

    fn theme(&self) -> Theme {
        let theme_color = CustomThemeChoice::from_system();

        let theme = match theme_color {
            CustomThemeChoice::Dark => iced::Theme::custom(
                String::from("Custom"),
                iced::theme::Palette {
                    success: Color::from_rgb(46. / 255., 194. / 255., 126. / 255.),
                    danger: Color::from_rgb(244. / 255., 27. / 255., 36. / 255.),
                    text: Color::from_rgb(255. / 255., 255. / 255., 255. / 255.),
                    // primary: Color::from_rgb(30. / 255., 30. / 255., 30. / 255.),
                    primary: Color::from_rgb(0.21, 0.52, 0.89),
                    background: Color::from_rgb(42. / 255., 42. / 255., 42. / 255.),
                },
            ),
            CustomThemeChoice::Light => iced::Theme::custom(
                String::from("Custom"),
                iced::theme::Palette {
                    success: Color::from_rgb(46. / 255., 194. / 255., 126. / 255.),
                    danger: Color::from_rgb(244. / 255., 27. / 255., 36. / 255.),
                    text: Color::from_rgb(255. / 255., 255. / 255., 255. / 255.),
                    // primary: Color::from_rgb(30. / 255., 30. / 255., 30. / 255.),
                    primary: Color::from_rgb(0.21, 0.52, 0.89),
                    background: Color::from_rgb(42. / 255., 42. / 255., 42. / 255.),
                },
            ),
        };

        theme
    }
}