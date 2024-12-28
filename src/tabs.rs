use std::path::PathBuf;

use uuid::Uuid;

#[derive(Debug)]
pub enum TabHistoryEntry {
    Library,
    File { path: PathBuf, preview: bool },
    Folder { path: PathBuf },
}

#[derive(Debug, Clone)]
pub enum TabNavigation {
    File(PathBuf),
    Folder(PathBuf),
}

pub type TabId = Uuid;

pub struct Tab {
    pub id: TabId,
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
    pub fn navigate(&mut self, navigation: &TabNavigation) {
        let new_entry: TabHistoryEntry = match navigation {
            TabNavigation::File(path) => TabHistoryEntry::File {
                path: path.to_path_buf(),
                preview: false,
            },
            TabNavigation::Folder(path) => TabHistoryEntry::Folder {
                path: path.to_path_buf(),
            },
        };

        self.history.push(new_entry);
        self.active_entry_index += 1;
    }
    pub fn active_entry(&self) -> &TabHistoryEntry {
        return &self.history[self.active_entry_index];
    }
    pub fn active_entry_mut(&mut self) -> &mut TabHistoryEntry {
        return &mut self.history[self.active_entry_index];
    }
}
