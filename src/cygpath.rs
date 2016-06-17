use std::path::Path;
use std::path::{Component, Prefix};

pub fn from_win<P: AsRef<Path>>(path: P) -> String {
  let mut cygpath = vec![];

  for p in path.as_ref().components() {
    // println!("{:?}", p);
    match p {
      Component::Prefix(c) => {
        match c.kind() {
          Prefix::Disk(d) => {
            cygpath.push("".to_owned());
            cygpath.push("cygdrive".to_owned());
            cygpath.push((d as char).to_lowercase().collect::<String>());
          }
          _ => {}
        };
      }
      Component::Normal(c) => cygpath.push(c.to_str().unwrap().to_owned()),
      Component::RootDir => {}
      _ => cygpath.push(p.as_os_str().to_str().unwrap().to_string()),
    }
  }

  cygpath.join("/")
}

#[cfg(test)]
mod tests {
  use super::from_win;

  #[test]
  fn test_from_win() {
    assert_eq!(from_win("C:\\Programming\\Some stuff\\hErE"),
               "/cygdrive/c/Programming/Some stuff/hErE");
    assert_eq!(from_win("d:\\a_B_cccc\\.\\..\\..\\hey"),
               "/cygdrive/d/a_B_cccc/../../hey");
    assert_eq!(from_win("k:\\//\\/"), "/cygdrive/k");
  }
}
