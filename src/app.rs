use iced::{
    executor, font,
    widget::{column, container, row, scrollable},
    Application, Color, Command, Element, Length, Subscription, Theme,
};
use iced_aw::split;

use crate::{
    constants::spacings,
    data::repositories::{Repositories, Repository},
    pages::main::{Main, MainMessage},
    sidebar::sidebar::{Sidebar, SidebarMessage},
    styles,
    theme::CustomThemeChoice,
    topbar::topbar::{Topbar, TopbarGitAction, TopbarMessage},
};

#[derive(Debug, Clone)]
pub enum AppMessage {
    SidebarMessage(SidebarMessage),
    TopbarMessage(TopbarMessage),
    MainMessage(MainMessage),
    FontLoaded(Result<(), font::Error>),
    DividerDragTo(u16),
}

pub struct App {
    sidebar: Sidebar,
    topbar: Topbar,
    main: Main,
    divider_position: Option<u16>,
    repositories: Repositories,
    current_repository_name: Option<String>,
}

impl Application for App {
    type Message = AppMessage;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<AppMessage>) {
        let mut new_self = Self {
            sidebar: Sidebar::new(),
            topbar: Topbar::new(),
            main: Main::new(),
            divider_position: Some(200),
            repositories: Repositories::new(),
            current_repository_name: None,
        };

        let repository = Repository::new();

        new_self
            .repositories
            .insert("Git desktop".to_string(), repository);
        new_self.current_repository_name = Some("Git desktop".to_string());

        (
            new_self,
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

                let _ = self
                    .sidebar
                    .update(sidebar_message)
                    .map(AppMessage::SidebarMessage);

                Command::none()
            }
            AppMessage::TopbarMessage(topbar_message) => {
                println!("topbar message {:?}", topbar_message);

                let _ = self
                    .topbar
                    .update(
                        topbar_message,
                        TopbarGitAction::Fetch,
                        &self.current_repository_name,
                        &self.repositories,
                    )
                    .map(AppMessage::TopbarMessage);

                match topbar_message {
                    _ => Command::none(),
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
            AppMessage::DividerDragTo(drag_position) => {
                println!("split moved {:?}", drag_position);
                self.divider_position = Some(drag_position);
                Command::none()
            }
        }
    }

    /*     fn subscription(&self) -> Subscription<AppMessage> {

    } */

    fn view(&self) -> Element<AppMessage> {
        let left = column![self
            .sidebar
            .view(self.divider_position, &self.current_repository_name, &self.repositories)
            .map(AppMessage::SidebarMessage)];

        let main_content = container(
            self.main
                .view(&self.current_repository_name,
                    &self.repositories)
                .map(move |message| AppMessage::MainMessage(message)),
        )
        .padding(spacings::MEDIUM)
        .width(Length::Fill)
        .style(styles::container::main_content());
        let main = scrollable(main_content)
            .style(iced::theme::Scrollable::Custom(Box::new(
                styles::scrollable::Background1,
            )))
            .height(Length::Fill);
        let right = column![
            self.topbar
                .view(
                    TopbarGitAction::Fetch,
                    &self.current_repository_name,
                    &self.repositories
                )
                .map(AppMessage::TopbarMessage),
            main
        ];

        let content = split::Split::new(
            left,
            right,
            self.divider_position,
            split::Axis::Vertical,
            AppMessage::DividerDragTo,
        );

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
