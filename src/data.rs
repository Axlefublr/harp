use std::collections::HashMap;
use std::default::Default;

use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize, Default)]
pub struct Entry {
    pub path:   Option<String>,
    pub line:   Option<i32>,
    pub column: Option<i32>,
}

pub type Entries = HashMap<String, HashMap<String, Entry>>;
