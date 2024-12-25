use iced::{
    widget::{button, container, text, text_editor, Column},
    Element,
    Length::Fill,
};
use std::{fs, path::PathBuf};

use crate::{
    tabs::{Tab, TabHistoryEntry, TabNavigation},
    Message,
};

pub struct Pane;

impl Pane {
    pub fn view<'a>(vault_path: &'a PathBuf, active_tab: Option<&'a Tab>) -> Element<'a, Message> {
        if let None = active_tab {
            return container(text("No active tab")).into();
        }
        let active_tab = active_tab.unwrap();

        match active_tab.active_entry() {
            TabHistoryEntry::Library => {
                let entries = match fs::read_dir(vault_path) {
                    Ok(entries) => entries,
                    Err(_) => return text("Failed to read directory").into(),
                };
                let items: Vec<Element<Message>> = entries
                    .map(|entry| {
                        let entry = entry.unwrap();
                        let path = entry.path();
                        let metadata = entry.metadata().unwrap();
                        let tab_history_entry: TabNavigation = if metadata.is_dir() {
                            TabNavigation::Folder(path.clone())
                        } else {
                            TabNavigation::File(path.clone())
                        };
                        button(text(path.into_os_string().into_string().unwrap()))
                            .on_press(Message::NavigateTab(active_tab.id, tab_history_entry))
                            .into()
                    })
                    .collect();

                let column = Column::from_vec(items);

                container(column).center_x(Fill).center_y(Fill).into()
            }
            TabHistoryEntry::File { content, .. } => container(
                text_editor(&content).on_action(|action| Message::EditFile(active_tab.id, action)),
            )
            .into(),
            TabHistoryEntry::Folder { path } => {
                let entries = match fs::read_dir(path) {
                    Ok(entries) => entries,
                    Err(_) => return text("Failed to read directory").into(),
                };
                let items: Vec<Element<Message>> = entries
                    .map(|entry| {
                        let path = entry.unwrap().path();
                        button(text(path.clone().into_os_string().into_string().unwrap()))
                            .on_press(Message::NavigateTab(
                                active_tab.id,
                                TabNavigation::File(path),
                            ))
                            .into()
                    })
                    .collect();

                let column = Column::from_vec(items);

                container(column).center_x(Fill).center_y(Fill).into()
            }
        }
    }
}
