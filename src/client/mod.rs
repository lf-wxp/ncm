#![allow(dead_code)]
use std::fs;
use chrono::{DateTime, Local};
use anyhow::{self, Result};
use reqwest::{self, blocking::Response};
use serde::{Deserialize};
use serde_json;
use log::{info, error};
use reqwest::header::{
  HeaderMap, ACCEPT, ACCEPT_ENCODING, CONTENT_TYPE, COOKIE, HOST, REFERER, USER_AGENT,
};

use crate::encrypt::Encrypt;
use crate::uri;
use crate::model;
use crate::util;

struct User {
  id: u64,
  nickname: String,
}
struct NcmClient {
  cookie_path: String,
  http_client: reqwest::blocking::Client,
  user: Option<User>,
}

enum Method {
  POST,
  GET,
}

impl NcmClient {
  fn new() -> NcmClient {
    NcmClient {
      cookie_path: "/tmp/ncmt_cookie".to_owned(),
      http_client: reqwest::blocking::Client::new(),
      user: None,
    }
  }

  fn request(
    &self,
    url: &str,
    method: Method,
    data: Option<[(&str, String); 2]>,
  ) -> Result<Response, reqwest::Error> {
    info!("request params is {:#?}", &data);
    let response = match method {
      Method::GET => {
        let headers = self.get_get_headers();
        self.http_client.get(url).headers(headers).send()
      }
      Method::POST => {
        let headers = self.get_post_headers();
        self
          .http_client
          .post(url)
          .headers(headers)
          .form(&data.unwrap())
          .send()
      }
    };

    // log the response
    match &response {
      Ok(res) => {
        info!("response is {:?}", res);
      },
      Err(err) => {
        error!("get response error, error is {:?}", err);
      }
    };
    response
  }

  fn json_parse<'a, T: Deserialize<'a>>(data: &'a str) -> Result<T, serde_json::Error> {
    serde_json::from_str::<T>(data)
      .map_err(|e| {
        format!(
          "convert result failed, reason: {:?}; content: [{:?}]",
          e, data
        );
        e
      })
  }

  fn get_common_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
      CONTENT_TYPE,
      "application/x-www-form-urlencoded".parse().unwrap(),
    );
    headers.insert(ACCEPT, "*/*".parse().unwrap());
    headers.insert(REFERER, "https://music.163.com".parse().unwrap());
    headers.insert(
      USER_AGENT,
      "User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:65.0) Gecko/20100101 Firefox/65.0"
        .parse()
        .unwrap(),
    );
    headers.insert(HOST, "music.163.com".parse().unwrap());
    headers.insert(ACCEPT_ENCODING, "gzip,deflate".parse().unwrap());
    headers
  }

  fn set_cookies(mut headers: HeaderMap, cookies: String) -> HeaderMap {
    headers.insert(COOKIE, cookies.parse().unwrap());
    headers
  }

  fn get_post_headers(&self) -> HeaderMap {
    let headers = NcmClient::get_common_headers();
    let cookies = self.build_cookies();
    let headers = NcmClient::set_cookies(headers, cookies);
    headers
  }

  fn get_get_headers(&self) -> HeaderMap {
    let headers = NcmClient::get_common_headers();
    let cookies = self.get_cookies();
    let headers = NcmClient::set_cookies(headers, cookies);
    headers
  }

  fn get_cookies(&self) -> String {
    fs::read_to_string(&self.cookie_path).unwrap_or(String::new())
  }

  fn build_cookies(&self) -> String {
    let name = "os";
    let value = "pc";
    let local: DateTime<Local> = Local::now();
    let times = local.timestamp();
    let hex_token = Encrypt::encrypt_hex(times.to_string());
    let data = self.get_cookies();
    let make_cookie = format!("version=0;{}={};JSESSIONID-WYYY=%2FKSy%2B4xG6fYVld42G9E%2BxAj9OyjC0BYXENKxOIRH%5CR72cpy9aBjkohZ24BNkpjnBxlB6lzAG4D%5C%2FMNUZ7VUeRUeVPJKYu%2BKBnZJjEmqgpOx%2BU6VYmypKB%5CXb%2F3W7%2BDjOElCb8KlhDS2cRkxkTb9PBDXro41Oq7aBB6M6OStEK8E%2Flyc8%3A{}; _iuqxldmzr_=32; _ntes_nnid={},{}; _ntes_nuid={}; {}", name, value, times, hex_token, hex_token, times + 50, data);
    make_cookie.parse().unwrap()
  }

  fn save_cookies(&self, res: &Response) {
    let cookies: Vec<String> = res
      .cookies()
      .into_iter()
      .map(|s| format!("{}={}", s.name().to_string(), s.value().to_string()))
      .collect();
    let mut c: String = cookies.into_iter().map(|s| format!("{}; ", s)).collect();
    c.pop();
    if c.len() > 0 {
      fs::write(&self.cookie_path, &c).expect("Unable to write file");
    }
  }

  fn login_email(&mut self, email: String, password: String) -> Result<(), anyhow::Error> {
    let password = Encrypt::encrypt_hex(password);
    let client_token = "1_jVUMqWEPke0/1/Vu56xCmJpo5vP1grjn_SOVVDzOc78w8OKLVZ2JH7IfkjSXqgfmh";
    let params = util::build_post_data(vec![
      ("clientToken", client_token),
      ("password", &password[..]),
      ("username", &email[..]),
      ("rememberLogin", "true"),
      ("csrf_token", "")
    ]);
    let response = self.request(uri::LOGIN, Method::POST, params)?;
    self.save_cookies(&response);
    let text = response.text()?;
    let login = NcmClient::json_parse::<model::response::Login>(&text)?;
    self.save_login_status(login);
    Ok(())
  }

  fn save_login_status(&mut self, login: model::response::Login) -> () {
    self.user = Some(User {
      id: login.account.id,
      nickname: login.profile.nickname,
    });
  }

  fn get_user_playlist(&self) -> Result<model::response::PlaylistList, anyhow::Error> {
    let uid = self.user.as_ref().unwrap().id;
    let params = util::build_post_data(vec![
      ("uid", &uid.to_string()[..]),
      ("limit", "1000"),
      ("offset", "0"),
      ("csrf_token", "")
    ]);
    let response = self.request(uri::USER_PLAYLIST, Method::POST, params)?;
    let text = response.text()?;
    let playlist = NcmClient::json_parse::<model::response::PlaylistList>(&text)?;
    Ok(playlist)
  }

  fn get_favorite_playlist(&self) -> Result<model::playlist::Playlist, anyhow::Error> {
    let playlist = self.get_user_playlist()?;
    match playlist.playlist.get(0) {
      Some(data ) => Ok(data.clone()),
      None => Err(anyhow::anyhow!("get favorite playlist error")),
    }
  }

  fn get_playlist_detail(&self, playlist_id: u64) -> Result<model::playlist::Playlist, anyhow::Error> {
    let params = util::build_post_data(vec![
      ("id", &playlist_id.to_string()[..]),
      ("total", "true"),
      ("limit", "10000"),
      ("offset", "0"),
      ("n", "1000")
    ]);
    let response = self.request(uri::PLAYLIST_DETAIL, Method::POST, params)?;
    let text = response.text()?;
    let playlist = NcmClient::json_parse::<model::response::Playlist>(&text)?;
    Ok(playlist.playlist)
  }
}

mod tests {
  use super::*;
  use crate::util;

  #[test]
  fn login_email_test() {
    let mut ncm_client = NcmClient::new();
    let setting = util::get_setting().unwrap();
    ncm_client.login_email(setting.email, setting.password);
  }

  #[test]
  fn get_favorite_test() {
    let mut ncm_client = NcmClient::new();
    let setting = util::get_setting().unwrap();
    // ncm_client.login_email(setting.email, setting.password);
    let favorite = ncm_client.get_favorite_playlist();
    match favorite {
      Ok(data) => println!("the favorite playlist is {:?}", data),
      Err(_) => {}
    };
  }

  #[test]
  fn get_playlist_detail_test() {
    let ncm_client = NcmClient::new();
    let playlist_detail = ncm_client.get_playlist_detail(95815468);
    match playlist_detail {
      Ok(data) => println!("the playlist detail is {:?}", data),
      Err(_) => {}
    };
  }
}
