#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};

use super::user;
use super::playlist;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Login {
  pub account: user::Account,
  pub code: i32,
  pub loginType: i8,
  pub profile: user::Profile,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Playlist {
  pub playlist: Vec<playlist::Playlist>,
  pub code: i32,
}
