use std::io::Write;

// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
pub fn link(name: &str, bundled: bool) {
  use std::env::var;
  let target = var("TARGET").unwrap();
  let target: Vec<_> = target.split('-').collect();
  if target.get(2) == Some(&"windows") {
    println!("cargo:rustc-link-lib=dylib={}", name);
    if bundled && target.get(3) == Some(&"gnu") {
      let dir = var("CARGO_MANIFEST_DIR").unwrap();
      println!("cargo:rustc-link-search=native={}/{}", dir, target[0]);
    }
  }
}

fn main() {
  println!("cargo:rustc-link-search=native={}",
           "D:/dev/msys64/mingw64/x86_64-w64-mingw32/lib/");
  link("powrprof", true);
}
