use iced::{
    widget::{button, container, text, Column},
    Element,
    Length::Fill,
};
use std::{fs, path::PathBuf};

use crate::{
    tabs::{Tab, TabHistoryEntry},
    Message,
};

pub struct Pane;

impl Pane {
    pub fn view<'a>(vault_path: &'a PathBuf, active_tab: Option<&Tab>) -> Element<'a, Message> {
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
                        let tab_history_entry = if metadata.is_dir() {
                            TabHistoryEntry::Folder { path: path.clone() }
                        } else {
                            TabHistoryEntry::File { path: path.clone() }
                        };
                        button(text(path.into_os_string().into_string().unwrap()))
                            .on_press(Message::NavigateTab(active_tab.id, tab_history_entry))
                            .into()
                    })
                    .collect();

                let column = Column::from_vec(items);

                container(column).center_x(Fill).center_y(Fill).into()
            }
            TabHistoryEntry::File { path } => {
                let content = fs::read_to_string(path).unwrap();
                container(text(content)).into()
            }
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
                                TabHistoryEntry::File { path },
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
