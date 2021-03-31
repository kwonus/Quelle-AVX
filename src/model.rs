extern crate serde;
extern crate serde_json;
use serde_json::json;
use serde_json::Result;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use rmp_serde::{Deserializer, Serializer};

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchRequest {
    pub clauses: Vec<SearchClause>,
    pub controls: Option<QuelleCloudControls>,
    pub count: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResult {
    pub session: String,
    pub abstracts: HashMap<u32, String>,   // AVX extension to Quelle
    pub cursor: u64,
    pub count: u64,
    pub remainder: u64,
    pub messages: HashMap<String, String>,
    pub summary: String,
    pub records: HashMap<u8,HashMap<u8,HashMap<u8,HashMap<u8,u64>>>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FetchRequest {
    pub session: String,
    pub cursor: u64,
    pub count: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FetchResult {
    pub session: String,
    pub abstracts: HashMap<u32, String>,
    pub cursor: u64,
    pub count: u64,
    pub remainder: u64,
    pub messages: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PageRequest {
    pub session: String,
    pub format: String,
    pub page: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PageResult {
    pub result: String,
    pub messages: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuelleCloudControls {
    pub domain: String,
    pub span: i32,
    pub exact: bool,
    pub host: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchClause {
    pub fragments: Vec<String>,  // SearchFragment(s)
    pub segment: String,
    pub polarity: i8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchFragment {
    pub position_aspects: Vec<u32>,
    pub any_of: Vec<TokenVector>,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenFeature {
    pub feature: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenMatch {
    pub condition: String,
    pub any_feature: Vec<TokenFeature>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenVector {
    pub specification: String,
    pub match_all: Vec<TokenMatch>,
}