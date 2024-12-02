use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum TabHistoryEntry {
    Library,
    File { path: String },
}

pub struct Tab {
    pub id: Uuid,
    history: Vec<TabHistoryEntry>,
    active_entry_index: usize,
}

impl Tab {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            history: vec![TabHistoryEntry::Library],
            active_entry_index: 0,
        }
    }
    pub fn navigate(&mut self, new_entry: TabHistoryEntry) {
        self.history.push(new_entry);
        self.active_entry_index += 1;
    }
    pub fn active_entry(&self) -> &TabHistoryEntry {
        return &self.history[self.active_entry_index];
    }
}
