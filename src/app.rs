use iced::{
    executor, font,
    widget::{column, container, row, scrollable},
    Application, Color, Command, Element, Length, Subscription, Theme,
};

use crate::{
    constants::spacings,
    pages::main::{Main, MainMessage},
    sidebar::sidebar::{Sidebar, SidebarMessage},
    styles,
    theme::CustomThemeChoice,
    topbar::topbar::{Topbar, TopbarGitAction, TopbarMessage},
};

#[derive(Debug, Clone, Copy)]
pub enum AppMessage {
    SidebarMessage(SidebarMessage),
    TopbarMessage(TopbarMessage),
    MainMessage(MainMessage),
    FontLoaded(Result<(), font::Error>),
}

pub struct App {
    sidebar: Sidebar,
    topbar: Topbar,
    main: Main,
}

impl Application for App {
    type Message = AppMessage;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<AppMessage>) {
        (
            Self {
                sidebar: Sidebar::new(),
                topbar: Topbar::new(),
                main: Main::new(),
            },
            Command::batch(vec![
                font::load(iced_aw::BOOTSTRAP_FONT_BYTES).map(AppMessage::FontLoaded)
            ]),
        )
    }

    fn title(&self) -> String {
        String::from("Git Desktop")
    }

    fn update(&mut self, message: AppMessage) -> Command<AppMessage> {
        match message {
            AppMessage::SidebarMessage(sidebar_message) => {
                println!("sidebar message {:?}", sidebar_message);
                Command::none()
            }
            AppMessage::TopbarMessage(topbar_message) => {
                println!("topbar message {:?}", topbar_message);

                let _ = self.topbar.update(topbar_message).map(AppMessage::TopbarMessage);

                match topbar_message {
                    TopbarMessage::CloseWindow => {

                        Command::none()
                    }
                    TopbarMessage::MinimizeWindow => {

                        Command::none()
                    }
                    _ => {
                        println!("[app] unhandled topbar message");
                        Command::none()
                    }
                }
            }
            AppMessage::MainMessage(page_message) => {
                print!("page message {:?}", page_message);
                Command::none()
            }
            AppMessage::FontLoaded(status) => {
                println!("font loaded {:?}", status);
                Command::none()
            }
        }
    }

    /*     fn subscription(&self) -> Subscription<AppMessage> {

    } */

    fn view(&self) -> Element<AppMessage> {
        let left = column![self
            .sidebar
            .view()
            .map(move |message| { AppMessage::SidebarMessage(message) })];

        let main_content = container(
                self.main
                    .view()
                    .map(move |message| AppMessage::MainMessage(message)),
            
        )
        .padding(spacings::MEDIUM)
        .width(Length::Fill)
        .style(styles::container::main_content());
        let main = scrollable(main_content).style(iced::theme::Scrollable::Custom(Box::new(
            styles::scrollable::Background1,
        )))        .height(Length::Fill);
        let right = column![
            self.topbar
                .view(TopbarGitAction::Fetch)
                .map(move |message| { AppMessage::TopbarMessage(message) }),
            main
        ];

        let content = row![left, right];

        let container = container(content);
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
