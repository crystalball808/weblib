use std::path::PathBuf;

use iced::{
    widget::{button, container, row, text},
    Element,
    Length::Fill,
};
use rfd::FileDialog;
use sidebar::Sidebar;
use uuid::Uuid;

mod pane;
mod sidebar;
mod tabs;

use pane::Pane;
use tabs::{Tab, TabHistoryEntry};

const APP_NAME: &str = "Weblib";

pub fn main() -> iced::Result {
    iced::application(APP_NAME, App::update, App::view).run()
}

#[derive(Debug, Clone)]
enum Message {
    OpenFilePicker,
    CreateLibraryTab,
    SelectTab(Uuid),
    NavigateTab(Uuid, TabHistoryEntry),
}

#[derive(Default)]
enum Screen {
    #[default]
    VaultSelect,
    Main {
        vault_path: PathBuf,
        tabs: Vec<Tab>,
        active_tab_id: Option<Uuid>,
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
                let path = FileDialog::new().pick_folder().unwrap();

                self.screen = Screen::Main {
                    vault_path: path,
                    tabs: Vec::new(),
                    active_tab_id: None,
                };
            }
            Message::CreateLibraryTab => {
                if let Screen::Main { tabs, .. } = &mut self.screen {
                    tabs.push(Tab::default());
                }
            }
            Message::SelectTab(tab_id) => {
                if let Screen::Main { active_tab_id, .. } = &mut self.screen {
                    *active_tab_id = Some(tab_id);
                }
            }
            Message::NavigateTab(tab_id, history_entry) => {
                if let Screen::Main { tabs, .. } = &mut self.screen {
                    let tab = tabs.iter_mut().find(|tab| tab.id == tab_id).unwrap();
                    tab.navigate(history_entry);
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
                tabs,
                active_tab_id,
            } => {
                let active_tab = if let Some(active_tab_id) = active_tab_id {
                    tabs.iter().find(|tab| match tab {
                        Tab { id, .. } => id == active_tab_id,
                    })
                } else {
                    None
                };

                row![
                    Sidebar::view(&tabs, *active_tab_id),
                    Pane::view(vault_path, active_tab)
                ]
                .into()
            }
        }
    }
}
