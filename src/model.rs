extern crate serde;
#[macro_use]
extern crate rmp_serde as rmps;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use rmps::{Deserializer, Serializer};

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchRequest {
    pub clauses: Vec<QuelleSearchClause>,
    pub controls: Option<QuelleSearchControls>,
    pub count: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResult {
    pub success: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub cursor: u64,
    pub remainder: u64,
    pub session: String,
    pub records: HashMap<u64, String>,
    pub summary: String,
    pub enrichedRequest: Option<SearchRequest>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FetchRequest {
    pub session: String,
    pub cursor: u64,
    pub count: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FetchResult {
    pub cursor: u64,
    pub remainder: u64,
    pub session: String,
    pub records: HashMap<u64, String>,
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
    pub request: Option<QuellePageRequest>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchControls {
    pub domain: String,
    pub span: i32,
    pub strict: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchClause {
    pub syntax: String,
    pub fragments: Vec<QuelleSearchFragment>,
    pub segment: String,
    pub polarity: c8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchFragment {
    pub position_aspects: Vec<u32>,
    pub any_of: Vec<QuelleTokenVector>,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenFeature {
    pub feature: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenMatch {
    pub condition: String,
    pub any_feature: Vec<QuelleTokenFeature>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenVector {
    pub specification: String,
    pub match_all: Vec<QuelleTokenMatch>,
}