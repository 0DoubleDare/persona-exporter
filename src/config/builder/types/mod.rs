use compact_str::CompactString;
use std::collections::HashMap;
pub mod enums;
pub use enums::*;

pub mod structs;
pub use structs::*;

pub type HttpHeaders = HashMap<String, String>;
pub type UrlParams = HashMap<CompactString, CompactString>;
