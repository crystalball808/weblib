use std::path::PathBuf;

use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum TabHistoryEntry {
    Library,
    File { path: PathBuf },
    Folder { path: PathBuf },
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
    pub fn navigate(&mut self, new_entry: TabHistoryEntry) {
        self.history.push(new_entry);
        self.active_entry_index += 1;
    }
    pub fn active_entry(&self) -> &TabHistoryEntry {
        return &self.history[self.active_entry_index];
    }
}
