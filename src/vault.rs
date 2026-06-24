use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    pub name: String,
    pub username: String,
    pub password: String,
    pub url: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Vault {
    pub entries: Vec<Entry>,
}

impl Entry {
    pub fn new(
        name: String,
        username: String,
        password: String,
        url: Option<String>,
        notes: Option<String>,
    ) -> Self {
        Self {
            name,
            username,
            password,
            url,
            notes,
        }
    }
}

impl Vault {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, entry: Entry) {
        self.entries.push(entry);
    }

    pub fn delete_entry(&mut self, entry_name: &str) -> bool {
        let vault_entries = self.entries();

        for (i, entry) in vault_entries.iter().enumerate() {
            if entry.name == entry_name {
                self.entries.remove(i);
                return true;
            }
        }
        false

    }

    pub fn entries(&self) -> &Vec<Entry> {
        &self.entries
    }
}