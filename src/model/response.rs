use serde::{Deserialize, Serialize};

use super::user;
use super::playlist;
use super::song;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Login {
  pub account: user::Account,
  pub code: i32,
  pub login_type: i8,
  pub profile: user::Profile,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlaylistList {
  pub playlist: Vec<playlist::Playlist>,
  pub code: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Playlist {
  pub playlist: playlist::Playlist,
  pub code: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Song {
  pub data: Vec<song::Song>,
  pub code: i32,
}
