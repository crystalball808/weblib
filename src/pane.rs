use iced::{
    widget::{button, checkbox, column, container, markdown, text, text_editor, Column},
    Element,
    Length::{self, Fill},
    Theme,
};
use std::{fs, path::PathBuf};

use crate::{
    tabs::{Tab, TabHistoryEntry, TabNavigation},
    Buffers, Message,
};

pub struct Pane;

impl Pane {
    pub fn view<'a>(
        vault_path: &'a PathBuf,
        active_tab: Option<&'a Tab>,
        buffers: &'a Buffers,
    ) -> Element<'a, Message> {
        if let None = active_tab {
            return container(text("No active tab")).center(Length::Fill).into();
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
            TabHistoryEntry::File { preview, path, .. } => {
                let buffer = match buffers.get(path) {
                    Some(buffer) => buffer,
                    None => {
                        return container(text("No associated buffer"))
                            .center(Length::Fill)
                            .into();
                    }
                };

                let preview_checkbox: Element<Message> = checkbox("Preview", *preview)
                    .on_toggle(|preview| Message::TogglePreview(active_tab.id, preview))
                    .into();

                let text_view: Element<Message> = if *preview {
                    markdown::view(
                        &buffer.md_items,
                        markdown::Settings::default(),
                        markdown::Style::from_palette(Theme::TokyoNight.palette()),
                    )
                    .map(Message::LinkClicked)
                    .into()
                } else {
                    text_editor(&buffer.content)
                        .on_action(|action| Message::EditFile(active_tab.id, action))
                        .into()
                };

                column![preview_checkbox, text_view].into()
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
