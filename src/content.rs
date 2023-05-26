use crate::constants::statistics::*;
use crate::content_type::PriceType;
use crate::lang::Lang;
use chrono::prelude::Local;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::Range;
use uuid::Uuid;
use quote::quote;
use paste::paste;

#[derive(Debug, Clone, Deserialize, Serialize, PartialOrd, PartialEq)]
pub struct Content {
    metadata: Metadata,
    description: String,
    lang: Lang,
    ty: PriceType,
    price: f64,
    url: String,
}

#[derive(Debug, Clone)]
pub enum FilterType {
    Lang(Lang),
    PriceType(PriceType),
    PriceEqual(f64),
    PriceRange(Range<f64>),
}

impl Content {
    pub fn new(
        metadata: Metadata,
        description: &str,
        lang: Lang,
        ty: PriceType,
        price: f64,
        url: &str,
    ) -> Self {
        Self {
            metadata,
            description: description.to_string(),
            lang,
            ty,
            price,
            url: url.to_string(),
        }
    }
}

pub fn filter_content(content: &Vec<Content>, ty: FilterType) -> Vec<Content> {
    let result: Vec<Content> = match ty {
        FilterType::Lang(lang) => content.iter().filter(|c| c.lang == lang).cloned().collect(),
        FilterType::PriceType(pt) => content.iter().filter(|c| c.ty == pt).cloned().collect(),
        FilterType::PriceEqual(price) => content
            .iter()
            .filter(|c| c.price == price)
            .cloned()
            .collect(),
        FilterType::PriceRange(range) => content
            .iter()
            .filter(|c| range.contains(&c.price))
            .cloned()
            .collect(),
    };
    result
}

pub fn serialize_content(content: &Vec<Content>) -> String {
    to_string_pretty(&content).unwrap()
}

pub fn deserialize_content(s: &str) -> Vec<Content> {
    from_str(s).unwrap()
}

#[derive(Debug, Clone, Deserialize, Serialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct Metadata {
    title: String,
    authors: Option<Vec<String>>,
    date: String,
    id: String,
}

impl Metadata {
    pub fn new(title: &str, authors: Option<Vec<String>>) -> Self {
        let today = Local::now();
        Self {
            title: title.to_string(),
            authors,
            date: today.format("%d.%m.%Y").to_string(),
            id: Uuid::new_v4().to_string()
        }
    }
}

pub struct Statistics {
    langs: HashMap<Lang, usize>,
    free: usize,
    tier1: usize,
    tier2: usize,
    tier3: usize,
    tier4: usize,
    tier5: usize,
}

macro_rules! generate_insert_statements {
    ($langs:ident, $content:ident $(, $lang:ident)*) => {
        $(
            $langs.insert(
                Lang::$lang,
                filter_content($content, FilterType::Lang(Lang::$lang)).len(),
            );
        )*
    };
}



impl Statistics {
    pub fn new(content: &Vec<Content>) -> Self {
        let mut langs = HashMap::new();

        // price tiers //
        let free = filter_content(content, FilterType::PriceEqual(FREE)).len();

        let tier1 = filter_content(content, FilterType::PriceRange(TIER1)).len();

        let tier2 = filter_content(content, FilterType::PriceRange(TIER2)).len();

        let tier3 = filter_content(content, FilterType::PriceRange(TIER3)).len();

        let tier4 = filter_content(content, FilterType::PriceRange(TIER4)).len();

        let tier5 = filter_content(content, FilterType::PriceRange(TIER5)).len();

        // Langs //
        generate_insert_statements!(langs, content, C, Rust, Zig);


        Self {
            langs,
            free,
            tier1,
            tier2,
            tier3,
            tier4,
            tier5,
        }
    }
    fn to_string(&self) -> String {
        let lang = {
            let mut vec = Vec::new();
            for (lang, count) in &self.langs{
                vec.push(format!("{} => {}", lang.to_string(), count))
            }
            vec.join("\n")
        };
        let free = format!("Free => {}", self.free);
        let tier1 = format!("Tier 1 ({:?}) => {}", TIER1, self.tier1);
        let tier2 = format!("Tier 2 ({:?}) => {}", TIER2, self.tier2);
        let tier3 = format!("Tier 3 ({:?}) => {}", TIER3, self.tier3);
        let tier4 = format!("Tier 4 ({:?}) => {}", TIER4, self.tier4);
        let tier5 = format!("Tier 5 ({:?}) => {}", TIER5, self.tier5);
        vec![lang, free, tier1, tier2, tier3, tier4, tier5].join("\n")
    }
}


impl Display for Statistics{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}
