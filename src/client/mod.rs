use crate::encrypt::Encrypt;
use crate::uri;
use chrono::{DateTime, Local};
use openssl::hash::MessageDigest;
use reqwest::header::{
  HeaderMap, ACCEPT, ACCEPT_ENCODING, CONTENT_TYPE, COOKIE, HOST, REFERER, USER_AGENT,
};
use reqwest::{self, blocking::Response};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::Read;

mod model;

struct NcmClient {
  cookie_path: String,
  http_client: reqwest::blocking::Client,
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
    }
  }
  fn request(
    &self,
    url: &str,
    method: Method,
    data: [(&str, std::string::String); 2],
  ) -> Result<(), Box<dyn std::error::Error>> {
    println!("the request params is {:?}", &data);
    let headers = self.get_post_headers();
    println!("the headers is {:#?}", &headers);
    match method {
      Method::GET => {
        let headers = self.get_get_headers();
        let mut response = self.http_client.get(url).headers(headers).send()?;
        Ok(())
      }
      Method::POST => {
        println!("method is post");
        let headers = self.get_post_headers();
        let response = self
          .http_client
          .post(url)
          .headers(headers)
          .form(&data)
          .send()?;
        let text = response.text().unwrap();
        let data = NcmClient::json_parse::<model::Login>(&text);
        println!("the text is {:?}", text);
        println!("the request response is {:#?}", data);
        Ok(())
      }
    }
  }

  fn json_parse<'a, T: Deserialize<'a>>(data: &'a str ) -> T {
    serde_json::from_str::<T>(data).map_err(|e| {
      format!(
        "convert result failed, reason: {:?}; content: [{:?}]",
        e, data
      )
    }).unwrap()
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
    let hex_token = Encrypt::encrypt_hex(&times.to_string());
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

  fn login(&self) -> () {
    let email = "";
    let password = "";
    let password = Encrypt::encrypt_hex(password);
    let client_token = "1_jVUMqWEPke0/1/Vu56xCmJpo5vP1grjn_SOVVDzOc78w8OKLVZ2JH7IfkjSXqgfmh";
    let csrf_token = String::new();
    let mut params = HashMap::new();
    params.insert("csrf_token".to_owned(), csrf_token);
    params.insert("clientToken".to_owned(), client_token.to_string());
    params.insert("username".to_owned(), email.to_string());
    params.insert("password".to_owned(), password);
    params.insert("rememberLogin".to_owned(), "true".to_owned());
    let params = Encrypt::encrypt_login(params);
    // let params = Encrypt::encrypt_login_string(params);
    self.request(uri::LOGIN, Method::POST, params);
  }
}

mod tests {
  use super::*;

  #[test]
  fn login_test() {
    let ncm_client = NcmClient::new();
    ncm_client.login();
  }
}
