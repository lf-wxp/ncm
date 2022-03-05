#![allow(non_snake_case)]
#![allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Account {
  pub id: u64,
  pub userName: String,
  pub status: i8,
  pub whitelistAuthority: i8,
  pub createTime: i64,
  pub tokenVersion: i8,
  pub baoyueVersion: i8,
  pub donateVersion: i8,
  pub vipType: i8,
  pub viptypeVersion: i64,
  pub anonimousUser: bool,
  pub uninitialized: bool,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Profile {
  pub accountStatus: i8,
  pub authStatus: i8,
  pub authority: i8,
  pub avatarDetail: Option<String>,
  pub avatarImgId: i64,
  pub avatarImgIdStr: String,
  pub avatarUrl: String,
  pub backgroundImgId: i64,
  pub backgroundImgIdStr: String,
  pub backgroundUrl: String,
  pub birthday: u64,
  pub city: u32,
  pub defaultAvatar: bool,
  pub description: String,
  pub detailDescription: String,
  pub djStatus: i8,
  pub eventCount: i8,
  pub expertTags: Option<String>,
  pub followed: bool,
  pub followeds: u64,
  pub follows: u64,
  pub gender: i8,
  pub mutual: bool,
  pub nickname: String,
  pub playlistBeSubscribedCount: i8,
  pub playlistCount: u64,
  pub province: i32,
  pub remarkName: Option<String>,
  pub signature: String,
  pub userId: u64,
  pub userType: i8,
  pub vipType: i8,
}
