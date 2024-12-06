use std::path::PathBuf;

use iced::widget::text_editor;
use uuid::Uuid;

#[derive(Debug)]
pub enum TabHistoryEntry {
    Library,
    File {
        path: PathBuf,
        content: text_editor::Content,
    },
    Folder {
        path: PathBuf,
    },
}

#[derive(Debug, Clone)]
pub enum TabNavigation {
    File(PathBuf),
    Folder(PathBuf),
}

pub struct Tab {
    pub id: Uuid,
    history: Vec<TabHistoryEntry>,
    active_entry_index: usize,
}

impl Default for Tab {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            history: vec![TabHistoryEntry::Library],
            active_entry_index: 0,
        }
    }
}

impl Tab {
    pub fn navigate(&mut self, navigation: TabNavigation) {
        let new_entry: TabHistoryEntry = match navigation {
            TabNavigation::File(path) => TabHistoryEntry::File {
                path,
                content: text_editor::Content::new(),
            },
            TabNavigation::Folder(path) => TabHistoryEntry::Folder { path },
        };

        self.history.push(new_entry);
        self.active_entry_index += 1;
    }
    pub fn active_entry(&self) -> &TabHistoryEntry {
        return &self.history[self.active_entry_index];
    }
}
