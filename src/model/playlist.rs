#![allow(non_snake_case)]
#![allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Playlist {
  pub id: u64,
  pub name: String,
  pub coverImgUrl: String,
  pub playCount: u64,
  pub trackCount: u32,
  pub createTime: u64,
  pub updateTime: u64,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Creator {
  pub avatarUrl: String,
  pub useId: u64,
  pub nickname: String,
  pub backgroundUrl: String,
}
