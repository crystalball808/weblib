use iced::{
    widget::{button, container, row, text, Column},
    Element,
    Length::Fill,
    Theme,
};
use rfd::FileDialog;
use sidebar::Sidebar;
use std::fs;

mod sidebar;
mod tabs;

use tabs::Tab;

pub fn main() -> iced::Result {
    iced::application("Weblib", App::update, App::view)
        .theme(theme)
        .run()
}

fn theme(_state: &App) -> Theme {
    Theme::KanagawaLotus
}

#[derive(Debug, Clone)]
enum Message {
    OpenFilePicker,
    CreateLibraryTab,
}

#[derive(Default)]
enum Screen {
    #[default]
    VaultSelect,
    Main {
        vault_path: String,
        tabs: Vec<Tab>,
        sidebar: Sidebar,
    },
}

#[derive(Default)]
struct App {
    screen: Screen,
}
impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::OpenFilePicker => {
                let path = FileDialog::new()
                    .pick_folder()
                    .unwrap()
                    .into_os_string()
                    .into_string()
                    .unwrap();

                self.screen = Screen::Main {
                    vault_path: path,
                    sidebar: Sidebar::new(),
                    tabs: Vec::new(),
                };
            }
            Message::CreateLibraryTab => {
                if let Screen::Main { tabs, .. } = &mut self.screen {
                    tabs.push(Tab::Library {
                        id: uuid::Uuid::new_v4(),
                    });
                }
            }
        }
    }
    fn view(&self) -> Element<Message> {
        match &self.screen {
            Screen::VaultSelect => container(row![
                text("You need to set the path"),
                button(text("Set mock path")).on_press(Message::OpenFilePicker)
            ])
            .center_x(Fill)
            .center_y(Fill)
            .into(),
            Screen::Main {
                vault_path,
                sidebar,
                tabs,
            } => {
                let entries = match fs::read_dir(vault_path) {
                    Ok(entries) => entries,
                    Err(_) => return text("Failed to read directory").into(),
                };

                let items: Vec<Element<Message>> = entries
                    .map(|entry| text(entry.unwrap().path().to_string_lossy().to_string()).into())
                    .collect();

                let column = Column::from_vec(items);

                row![
                    sidebar.view(&tabs),
                    container(column).center_x(Fill).center_y(Fill)
                ]
                .into()
            }
        }
    }
}
