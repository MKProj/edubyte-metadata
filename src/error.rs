use thiserror::Error;

/// Custom error type for Edu-Byte
#[derive(Error, Debug)]
pub enum EbError {
    #[error("This language `{0}` isn't supported yet!")]
    UnSupportedLang(String),
}
