use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgHasArrayType, PgTypeInfo};
use strum::EnumIter;

#[derive(
    Debug, sqlx::Type, PartialEq, Eq, Clone, Copy, EnumIter, strum::Display, Serialize, Hash, Deserialize
)]
#[sqlx(type_name = "language", rename_all = "lowercase")]
pub enum Language {
    #[strum(serialize = "cht")]
    #[serde(rename = "cht")]
    Cht,
    #[strum(serialize = "chs")]
    #[serde(rename = "chs")]
    Chs,
    #[strum(serialize = "de")]
    #[serde(rename = "de")]
    De,
    #[strum(serialize = "en")]
    #[serde(rename = "en")]
    En,
    #[strum(serialize = "es")]
    #[serde(rename = "es")]
    Es,
    #[strum(serialize = "fr")]
    #[serde(rename = "fr")]
    Fr,
    #[strum(serialize = "id")]
    #[serde(rename = "id")]
    Id,
    #[strum(serialize = "jp")]
    #[serde(rename = "jp")]
    Jp,
    #[strum(serialize = "kr")]
    #[serde(rename = "kr")]
    Kr,
    #[strum(serialize = "pt")]
    #[serde(rename = "pt")]
    Pt,
    #[strum(serialize = "ru")]
    #[serde(rename = "ru")]
    Ru,
    #[strum(serialize = "th")]
    #[serde(rename = "th")]
    Th,
    #[strum(serialize = "vi")]
    #[serde(rename = "vi")]
    Vi,
}

impl PgHasArrayType for Language {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("language[]")
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct DictionaryItem {
    pub vocabulary_id: i64,
    pub language: Language,
    pub vocabulary_translation: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NestedDictionaryItem {
    pub vocabulary_id: i64,
    pub target: String,
    pub target_lang: Language,
    pub lan_dict: HashMap<Language, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueryResult {
    pub total_page: u64,
    pub results: Vec<NestedDictionaryItem>,
}
