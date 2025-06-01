//! Create a [`HarpConnection`] using [`HarpConnection::build`] — this ensures the data file is created, and is valid json that you can modify.
//! Then, use [`HarpConnection::entry_mut`] to get a mutable reference to the vector of the register that you want to interact with.
//! Do all the changes you want to do, and call [`HarpConnection::save`]. This will write the model back into the json data file.

use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::{collections::HashMap, io::Read};

mod error;
pub use error::Error;

const PROGRAM_NAME: &str = "harp";
const DATA_FILE: &str = "harp.jsonc";

pub type Entries = HashMap<String, HashMap<String, Vec<String>>>;
pub type Result<T> = std::result::Result<T, Error>;

pub struct HarpConnection {
    data_path: PathBuf,
    entries: Entries,
}

impl HarpConnection {
    pub fn build() -> Result<Self> {
        let data_dir = dirs::data_local_dir().ok_or(Error::MissingLocalDataDirectory)?;
        let harp_data_dir = data_dir.join(PROGRAM_NAME);
        fs::create_dir_all(&harp_data_dir).expect("creating parent directories of the data file");
        let data_path = harp_data_dir.join(DATA_FILE);
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&data_path)
            .expect("open harp data file with read, write, create permissions");
        let mut json = String::new();
        file.read_to_string(&mut json)
            .expect("read data file");
        let entries: Entries = serde_json::from_str(&{
            if json.is_empty() {
                String::from("{}")
            } else {
                json
            }
        })
        .map_err(|_| Error::DeserializeData)?;
        Ok(Self { data_path, entries })
    }

    /// Remember to call this at the end of all of your harp operations
    pub fn save(self) -> Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.data_path)
            .expect("open harp data file with write, truncate permissions");
        file.write_all(
            serde_json::to_string_pretty(&self.entries)
                .map_err(|_| Error::SerializeData)?
                .as_bytes(),
        )
        .expect("write to the data file");
        Ok(())
    }

    /// Get a mutable reference to the `Vec` of your desired register, creating the section and the register in the process, if necessary.
    pub fn entry_mut(&mut self, section: String, register: String) -> &mut Vec<String> {
        let section_entry = self
            .entries
            .entry(section)
            .or_default();
        section_entry
            .entry(register)
            .or_default()
    }
}
