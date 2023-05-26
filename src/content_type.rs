use serde::{Deserialize, Serialize};
#[derive(Debug, Copy, Clone, Deserialize, Serialize, Ord, PartialOrd, Eq, PartialEq)]
pub enum PriceType {
    Free(ArticleType),
    Paid(PaidType),
}
/// Different types of articles in Edu-Byte
#[derive(Debug, Copy, Clone, Deserialize, Serialize, Ord, PartialOrd, Eq, PartialEq)]
pub enum ArticleType {
    /// This represents writing about a topic
    /// Such as "Implementing Dynamic Arrays in C"
    Topic,
    /// This represents a how to guide
    /// Such as "How to write a simple web server in Rust"
    HowtoGuide,
    /// This can be news on a language
    /// Such as "Zig becomes 1.0!!!" (someday)
    News,
    /// These are interactive ways to learn a language
    TrainLings,
}
/// Different types of paid content in Edu-Byte
#[derive(Debug, Copy, Clone, Deserialize, Serialize, Ord, PartialOrd, Eq, PartialEq)]
pub enum PaidType {
    /// A Udemy online course
    Udemy,
    /// An ebook sold directly on Edu-Byte
    Ebook,
}
