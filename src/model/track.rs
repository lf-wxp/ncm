#![allow(non_snake_case)]
#![allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Track {
  pub id: u64,
  pub name: String,
  pub ar: Option<Vec<Artist>>,
  pub al: Option<Album>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Artist {
  pub id: u64,
  pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Album {
  pub id: u64,
  pub name: String,
  pub picUrl: String,
}
