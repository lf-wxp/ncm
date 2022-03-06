
use std::{fs, collections::HashMap};
use std::io::Write;
use std::path;
use log::error;
use config::{Config, ConfigError};
use serde::Deserialize;

use super::encrypt::Encrypt;

const SETTING_PATH: &str = ".setting";

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Setting {
  pub password: String,
  pub email: String,
}

#[allow(unused_must_use)]
pub fn json_to_file(name: &str, text: &str) -> () {
  let path = path::Path::new(name);
  match fs::File::create(&path) {
    Ok(mut file) => {
      file.write_all(text.as_bytes());
    },
    Err(err) => {
      error!("json to file is error {:?}", err);
    }
  };
}

pub fn get_setting() -> Result<Setting, ConfigError>  {
  Config::builder().add_source(config::File::with_name(SETTING_PATH)).build()?.try_deserialize::<Setting>()
}

pub fn build_post_data(data: Vec<(&str, &str)>) -> Option<[(&'static str, String); 2]> {
  let mut params = HashMap::new();
  data.iter().for_each(|&(k, v)| { params.insert(k.to_string(), v.to_string());});
  Some(Encrypt::encrypt_login(params))
}

mod tests {
  use super::*;

  #[test]
  fn get_setting_test() {
    get_setting();
  }
}
