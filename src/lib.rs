pub mod ncm_api_uri;

#[cfg(test)]
mod tests {
    use crate::ncm_api_uri;

  #[test]
  fn it_works() {
    let result = 2 + 2;
    assert_eq!(ncm_api_uri::LOGIN, "https://music.163.com/weapi/login");
    assert_eq!(result, 4);
  }
}
