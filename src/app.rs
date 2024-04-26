use iced::{
    executor, font,
    widget::{column, container, row, scrollable},
    Application, Color, Command, Element, Length, Subscription, Theme,
};

use crate::{
    constants::spacings,
    pages::page::{Page, PageMessage},
    sidebar::sidebar::{Sidebar, SidebarMessage},
    styles,
    theme::CustomThemeChoice,
    topbar::topbar::{Topbar, TopbarMessage},
};

#[derive(Debug)]
pub enum AppMessage {
    SidebarMessage(SidebarMessage),
    TopbarMessage(TopbarMessage),
    PageMessage(PageMessage),
    FontLoaded(Result<(), font::Error>),
}

pub struct App {
    sidebar: Sidebar,
    topbar: Topbar,
    page: Page,
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
                page: Page::new(),
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
            }
            AppMessage::TopbarMessage(topbar_message) => {
                println!("topbar message {:?}", topbar_message);
            }
            AppMessage::PageMessage(page_message) => {
                print!("page message {:?}", page_message)
            }
            AppMessage::FontLoaded(status) => {
                println!("font loaded {:?}", status)
            }
        }

        Command::none()
    }

    /*     fn subscription(&self) -> Subscription<AppMessage> {

    } */

    fn view(&self) -> Element<AppMessage> {
        let left = column![self
            .sidebar
            .view()
            .map(move |message| { AppMessage::SidebarMessage(message) })];

        let main_content = container(
            container(
                self.page
                    .view()
                    .map(move |message| AppMessage::PageMessage(message)),
            )
            .style(styles::container::test_background_2())
            .width(Length::Fill), // .height(Length::Fill),
        )
        .padding(spacings::MEDIUM)
        .width(Length::Fill)
        .style(styles::container::main_content());
        let main = scrollable(main_content).style(iced::theme::Scrollable::Custom(Box::new(
            styles::scrollable::Background1,
        )));
        let right = column![
            self.topbar
                .view()
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
