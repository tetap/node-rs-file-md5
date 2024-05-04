#![deny(clippy::all)]
use md5::Digest;
use md5::Md5;
use std::fs::File;
use std::io::{BufReader, Read};

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn read_file_to_md5(path: String) -> String {
  let file = File::open(path).unwrap();
  let mut render = BufReader::new(file);
  // 计算耗时
  let mut buffer = [0u8; 204800];
  let mut context = Md5::new();
  loop {
    let n = render.read(&mut buffer).unwrap();
    if n == 0 {
      break;
    }
    context.update(&buffer[..n]);
  }
  let digest = context.finalize();
  format!("{:x}", digest)
}
