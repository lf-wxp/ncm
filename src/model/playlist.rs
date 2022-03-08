#![allow(dead_code)]
use serde::{Deserialize, Serialize};

use super::track;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Playlist {
  pub id: u64,
  pub name: String,
  pub cover_img_url: String,
  pub play_count: u64,
  pub track_count: u32,
  pub create_time: u64,
  pub update_time: u64,
  pub creator: Creator,
  pub tracks: Option<Vec<track::Track>>,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Creator {
  pub avatar_url: String,
  pub user_id: u64,
  pub nickname: String,
  pub background_url: String,
}
