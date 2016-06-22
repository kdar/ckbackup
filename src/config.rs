use std::fs::File;
use toml::{Parser, Value};
use toml;
use std::io::Read;
use std::path::Path;
use std::collections::HashMap;

#[derive(RustcEncodable, RustcDecodable, Debug, Default, Clone)]
pub struct Post {
  pub sleep: Option<bool>,
}

#[derive(RustcEncodable, RustcDecodable, Debug, Default, Clone)]
pub struct Check {
  pub args: Option<Vec<String>>,
}

#[derive(RustcEncodable, RustcDecodable, Debug, Default, Clone)]
pub struct Purge {
  pub args: Option<Vec<String>>,
}

#[derive(RustcEncodable, RustcDecodable, Debug, Default, Clone)]
pub struct Create {
  pub sources: Vec<String>,
  pub volume_shadow_copy: Option<bool>,
  pub args: Option<Vec<String>>,
}

#[derive(RustcEncodable, RustcDecodable, Debug, Default, Clone)]
pub struct Init {
  pub args: Option<Vec<String>>,
}

#[derive(RustcEncodable, RustcDecodable, Debug, Default, Clone)]
pub struct General {
  pub repo: String,
  pub env: Option<HashMap<String, String>>,
}

#[derive(RustcEncodable, RustcDecodable, Debug, Default)]
pub struct Config {
  pub general: General,
  pub init: Option<Init>,
  pub create: Create,
  pub purge: Option<Purge>,
  pub check: Option<Check>,
  pub post: Option<Post>,
}

impl Config {
  // pub fn new() -> Config {
  //   Config::default()
  // }

  pub fn parse<P: AsRef<Path>>(path: P) -> Result<Config, String> {
    let pb = path.as_ref().to_path_buf();
    let mut config_toml = String::new();

    let mut file = match File::open(&pb) {
      Ok(file) => file,
      Err(err) => {
        return Err(format!("{}: {}", err, pb.to_str().unwrap()));
      }
    };

    try!(file.read_to_string(&mut config_toml)
      .map_err(|err| err.to_string()));

    let mut parser = Parser::new(&config_toml);
    let toml = parser.parse();

    if toml.is_none() {
      for err in &parser.errors {
        let (loline, locol) = parser.to_linecol(err.lo);
        let (hiline, hicol) = parser.to_linecol(err.hi);
        println!("{}:{}:{}-{}:{} error: {}",
                 pb.to_str().unwrap(),
                 loline,
                 locol,
                 hiline,
                 hicol,
                 err.desc);
      }

      return Err("Exiting server".to_owned());
    }

    let config = Value::Table(toml.unwrap());
    ::rustc_serialize::Decodable::decode(&mut toml::Decoder::new(config))
      .map_err(|err| err.to_string())
  }
}
