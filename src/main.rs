use core::panic;
use iced::{
    widget::{button, container, row, text, text_editor},
    Element,
    Length::Fill,
    Task,
};
use rfd::FileDialog;
use sidebar::Sidebar;
use std::{fs, path::PathBuf};
use uuid::Uuid;

mod config;
mod pane;
mod sidebar;
mod tabs;

use pane::Pane;
use tabs::{Tab, TabHistoryEntry, TabId, TabNavigation};

pub fn main() -> iced::Result {
    match config::get_config() {
        Ok(config) => iced::application(config::APP_NAME, App::update, App::view)
            .run_with(|| (App::new(config.vault_path), Task::none())),
        Err(error) => panic!("Get config error:{error}"),
    }
}

#[derive(Debug, Clone)]
enum Message {
    OpenFilePicker,
    CreateLibraryTab,
    SelectTab(TabId),
    NavigateTab(TabId, TabNavigation),
    EditFile(TabId, text_editor::Action),
    TogglePreview(TabId, bool),
}

enum Screen {
    VaultSelect,
    Main {
        vault_path: PathBuf,
        tabs: Vec<Tab>,
        active_tab_id: Option<Uuid>,
    },
}

struct App {
    screen: Screen,
}

impl App {
    fn new(vault_path: Option<PathBuf>) -> Self {
        if let Some(vault_path) = vault_path {
            Self {
                screen: Screen::Main {
                    vault_path,
                    tabs: Vec::new(),
                    active_tab_id: None,
                },
            }
        } else {
            Self {
                screen: Screen::VaultSelect,
            }
        }
    }
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
                if let Screen::Main {
                    tabs,
                    active_tab_id,
                    ..
                } = &mut self.screen
                {
                    let new_tab = Tab::default();
                    *active_tab_id = Some(new_tab.id);

                    tabs.push(new_tab);
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
            Message::EditFile(tab_id, action) => {
                if let Screen::Main { tabs, .. } = &mut self.screen {
                    let tab = tabs.iter_mut().find(|tab| tab.id == tab_id).unwrap();
                    if let TabHistoryEntry::File {
                        content,
                        path,
                        preview: false,
                    } = tab.active_entry_mut()
                    {
                        content.perform(action);
                        fs::write(path, content.text()).unwrap();
                    }
                }
            }
            Message::TogglePreview(tab_id, set_preview) => {
                if let Screen::Main { tabs, .. } = &mut self.screen {
                    let tab = tabs.iter_mut().find(|tab| tab.id == tab_id).unwrap();
                    if let TabHistoryEntry::File { preview, .. } = tab.active_entry_mut() {
                        *preview = set_preview;
                    }
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
