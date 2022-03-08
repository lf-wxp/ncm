#![allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
  pub id: u64,
  pub user_name: String,
  pub status: i8,
  pub whitelist_authority: i8,
  pub create_time: i64,
  pub token_version: i8,
  pub baoyue_version: i8,
  pub donate_version: i8,
  pub vip_type: i8,
  pub viptype_version: i64,
  pub anonimous_user: bool,
  pub uninitialized: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
  pub account_status: i8,
  pub auth_status: i8,
  pub authority: i8,
  pub avatar_detail: Option<String>,
  pub avatar_img_id: i64,
  pub avatar_img_id_str: String,
  pub avatar_url: String,
  pub background_img_id: i64,
  pub background_img_id_str: String,
  pub background_url: String,
  pub birthday: u64,
  pub city: u32,
  pub default_avatar: bool,
  pub description: String,
  pub detail_description: String,
  pub dj_status: i8,
  pub event_count: i8,
  pub expert_tags: Option<String>,
  pub followed: bool,
  pub followeds: u64,
  pub follows: u64,
  pub gender: i8,
  pub mutual: bool,
  pub nickname: String,
  pub playlist_be_subscribed_count: i8,
  pub playlist_count: u64,
  pub province: i32,
  pub remark_name: Option<String>,
  pub signature: String,
  pub user_id: u64,
  pub user_type: i8,
  pub vip_type: i8,
}
