use std::fs::File;
use toml::{Parser, Value};
use toml;
use std::io::Read;
use std::path::Path;

#[derive(RustcEncodable, RustcDecodable, Debug, Default, Clone)]
pub struct Retention {
  pub keep_within: Option<String>,
  pub keep_hourly: Option<u32>,
  pub keep_daily: Option<u32>,
  pub keep_weekly: Option<u32>,
  pub keep_monthly: Option<u32>,
  pub keep_yearly: Option<u32>,
}

#[derive(RustcEncodable, RustcDecodable, Debug, Default, Clone)]
pub struct Consistency {
  pub check: Option<bool>,
}

#[derive(RustcEncodable, RustcDecodable, Debug, Default)]
pub struct Config {
  pub user: String,
  pub hostname: String,
  pub repo: String,
  pub sources: Vec<String>,

  pub compression: Option<String>,
  pub one_file_system: Option<bool>,
  pub retention: Option<Retention>,
  pub consistency: Option<Consistency>,
  pub volume_shadow_copy: Option<bool>,
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
