extern crate serde;
#[macro_use]
extern crate rmp_serde as rmps;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use rmps::{Deserializer, Serializer};

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchRequest {
    pub clauses: Vec<SearchClause>,
    pub controls: Option<SearchControls>,
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
    pub success: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
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
    pub success: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub result: String,
    pub request: Option<PageRequest>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchControls {
    pub domain: String,
    pub span: i32,
    pub strict: i32,
    pub host: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchClause {
    pub fragments: Vec<SearchFragment>,
    pub segment: String,
    pub polarity: c8,
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