use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

use crate::data::Entries;
use crate::data::Entry;

mod data;

const DATA_FILE: &str = "harp.yml";

const NO_FLAGS_ERROR: &str = "specify at least one of `--path`, `--line`, `--column`, `--extra`";

pub struct HarpReady {
    harp_data_file: PathBuf,
    entries:        Entries,
}

impl HarpReady {
    pub fn build() -> Result<Self, Box<dyn Error>> {
        let data_dir = dirs::data_local_dir().ok_or("couldn't find local data directory")?;
        let harp_data_dir = data_dir.join("harp");
        fs::create_dir_all(&harp_data_dir)?;
        let harp_data_file = harp_data_dir.join(DATA_FILE);
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&harp_data_file)?;
        let entries: Entries = serde_yaml::from_reader(file)?;
        Ok(Self {
            harp_data_file,
            entries,
        })
    }

    fn save(&self) -> Result<(), Box<dyn Error>> {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.harp_data_file)?;
        file.write_all(
            serde_yaml::to_string(&self.entries)
                .map_err(|_| "couldn't serialize data model")?
                .as_bytes(),
        )?;
        Ok(())
    }

    pub fn update(
        &mut self,
        section: String,
        register: String,
        path: Option<String>,
        line: Option<i32>,
        column: Option<i32>,
        extra: Option<String>,
    ) -> Result<(), Box<dyn Error>> {
        if path.is_none() && line.is_none() && column.is_none() && extra.is_none() {
            return Err(NO_FLAGS_ERROR.into());
        }
        let section_entry = self.entries.entry(section).or_default();
        let register_entry = section_entry.entry(register).or_default();
        if let Some(path) = path {
            register_entry.path = Some(path);
        }
        if let Some(line) = line {
            register_entry.line = Some(line);
        }
        if let Some(column) = column {
            register_entry.column = Some(column);
        }
        if let Some(extra) = extra {
            register_entry.extra = Some(extra);
        }
        self.save()
    }

    pub fn clear(&mut self, section: String, register: Option<String>) -> Result<(), Box<dyn Error>> {
        if let Some(register) = register {
            if let Some(section_map) = self.entries.get_mut(&section) {
                if section_map.remove(&register).is_some() {
                    self.save()?;
                };
            };
        } else if self.entries.remove(&section).is_some() {
            self.save()?;
        }
        // Because our intention is to clear an entry,
        // if we don't find one, we still succeed (because the entry is already "cleared")
        // This is also why we don't *always* save: we don't always need to.
        // This would usually be too anal of a decision, but because our data is a hashmap of
        // hashmaps, it's not ordered, and modifying the data file basically randomly is
        // wack, since the user might have it under version control.
        // If nothing changed, but there are diffs all over the place, you're more likely to
        // just trust that there was a change, which might not be the case.
        // I could also use IndexMap to make the diffs clean, but that introduces a dependency,
        // and will be slower due to the sorting. Which you would then let the user opt into by
        // making them specify an environment variable. But all of that doesn't seem worth it.
        // If you believe it *is*, open an issue or PR the change directly.
        Ok(())
    }

    /// At least one of the 4 bool arguments needs to be true,
    /// The section needs to exist, and the register needs to exist;
    /// Otherwise you get an Err().
    pub fn get(
        &self,
        section: &str,
        register: &str,
        path: bool,
        line: bool,
        column: bool,
        extra: bool,
    ) -> Result<&Entry, &'static str> {
        if !path && !line && !column && !extra {
            return Err(NO_FLAGS_ERROR);
        }
        self.entries
            .get(section)
            .ok_or("section doesn't exist")?
            .get(register)
            .ok_or("register doesn't exist")
    }
}
