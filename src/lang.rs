use crate::error::EbError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
/// All supported languages in Edu-Byte content
// Make sure to have in alphabetic order
#[derive(Debug, Copy, Clone, Deserialize, Serialize, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Lang {
    C,
    Rust,
    Zig,
}

impl ToString for Lang {
    fn to_string(&self) -> String {
        match self {
            Lang::C => "C".to_string(),
            Lang::Rust => "Rust".to_string(),
            Lang::Zig => "Zig".to_string(),
        }
    }
}

impl FromStr for Lang {
    type Err = EbError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "C" => Ok(Self::C),
            "Rust" | "rust" => Ok(Self::Rust),
            "Zig" | "zig" => Ok(Self::Zig),
            _ => Err(EbError::UnSupportedLang(s.to_string())),
        }
    }
}
