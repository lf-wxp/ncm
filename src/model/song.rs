#![allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Song {
  pub id: u64,
  pub url: String,
  pub size: u64,
  pub md5: String,
  #[serde(alias="type")]
  pub format: String,
  pub level: String,
  pub encode_type: String,
  pub br: u64,
}
