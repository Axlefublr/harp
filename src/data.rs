use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Entry {
    pub path: Option<PathBuf>,
    pub line: Option<u32>,
    pub column: Option<u32>,
}

pub type Entries = HashMap<String, HashMap<String, Entry>>;
