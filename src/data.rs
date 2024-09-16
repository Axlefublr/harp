use std::collections::HashMap;
use std::default::Default;

use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize, Default)]
pub struct Entry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path:   Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line:   Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra:  Option<String>,
}

pub type Entries = HashMap<String, HashMap<String, Entry>>;
