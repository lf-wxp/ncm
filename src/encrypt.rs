use hex;
use std::collections::HashMap;
use std::os::unix::prelude::OsStrExt;
use rand::distributions::{Alphanumeric, DistString};
use rand::{thread_rng};
use serde::Serialize;
use num_bigint::BigUint;
use openssl::{
  hash::{hash, MessageDigest},
  symm::{encrypt, Cipher},
};
use serde_urlencoded;
use base64;


pub struct Encrypt;

static MODULUS:&str = "00e0b509f6259df8642dbc35662901477df22677ec152b5ff68ace615bb7b725152b3ab17a876aea8a5aa76d2e417629ec4ee341f56135fccf695280104e0312ecbda92557c93870114af6c9d05c4f7f0c3685b7a46bee255932575cce10b424d813cfe4875d3e82047b97ddef52741d546b8e289dc6935b3ece0462db0a22b8e7";
static NONCE: &str = "0CoJUm6Qyw8W8jud";
static PUBKEY: &str = "010001";

impl Encrypt {
  fn create_key(len: usize) -> String {
    // hex::encode::<Vec<u8>>(
    //   thread_rng()
    //     .sample_iter(&Alphanumeric)
    //     .take(len)
    //     .collect(),
    // )[..16]
    //   .to_string();
    Alphanumeric.sample_string(&mut thread_rng(), len)
  }

  pub fn encrypt_login(params: HashMap<String, String>) -> [(&'static str, std::string::String); 2] {
    let data = serde_json::to_string(&params).unwrap();
    let secret = Encrypt::create_key(16);
    // let secret = "e0e80547fa3ecd5a".to_owned();
    let params = Encrypt::aes(Encrypt::aes(data, NONCE), &secret);
    let enc_sec_key = Encrypt::rsa(secret);
    [("params", params), ("encSecKey", enc_sec_key)]
    // serde_urlencoded::to_string(&meal).unwrap_or("".to_owned())
  }

  pub fn encrypt_login_string(params: HashMap<String, String>) -> String {
    let data = serde_json::to_string(&params).unwrap();
    let secret = Encrypt::create_key(16);
    let secret = "e0e80547fa3ecd5a".to_owned();
    let params = Encrypt::aes(Encrypt::aes(data, NONCE), &secret);
    let enc_sec_key = Encrypt::rsa(secret);
    let meal = [("params", params), ("encSecKey", enc_sec_key)];
    serde_urlencoded::to_string(&meal).unwrap_or("".to_owned())
  }

  fn aes(text: String, key: &str) -> String {
    let pad = 16 - text.len() % 16;
    let p = pad as u8 as char;
    let mut text = text;
    for _ in 0..pad {
        text.push(p);
    }
    let text = text.as_bytes();
    let cipher = Cipher::aes_128_cbc();
    let ciphertext = encrypt(cipher, key.as_bytes(), Some(b"0102030405060708"), text).unwrap();
    base64::encode(&ciphertext)
  }


  fn rsa(text: String) -> String {
    let text = text.chars().rev().collect::<String>();
    let text = BigUint::parse_bytes(hex::encode(text).as_bytes(), 16).unwrap();
    let pubkey = BigUint::parse_bytes(PUBKEY.as_bytes(), 16).unwrap();
    let modulus = BigUint::parse_bytes(MODULUS.as_bytes(), 16).unwrap();
    let pow = text.modpow(&pubkey, &modulus);
    pow.to_str_radix(16)
  }

  pub fn encrypt_hex(data: &str) -> String {
    let password = hash(MessageDigest::md5(), data.as_bytes()).unwrap();
    hex::encode(password)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn create_key() {
    Encrypt::create_key(16);
  }
}



