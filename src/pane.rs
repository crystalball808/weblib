use iced::{
    widget::{container, text, Column},
    Element,
    Length::Fill,
};
use std::fs;

use crate::{
    tabs::{Tab, TabHistoryEntry},
    Message,
};

pub struct Pane;

impl Pane {
    pub fn view<'a>(vault_path: &'a str, active_tab: Option<&Tab>) -> Element<'a, Message> {
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
                    .map(|entry| text(entry.unwrap().path().to_string_lossy().to_string()).into())
                    .collect();

                let column = Column::from_vec(items);

                container(column).center_x(Fill).center_y(Fill).into()
            }
        }
    }
}
