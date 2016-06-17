extern crate time;
extern crate toml;
extern crate rustc_serialize;
extern crate kernel32;
extern crate kernel32x;
#[macro_use]
extern crate log;
extern crate simplelog;
extern crate clap;

use std::process::exit;
use simplelog::{TermLogger, LogLevelFilter};
use clap::{App, AppSettings, SubCommand};

mod config;
mod cygpath;
mod vss;
mod backup;

fn cmd_auto() {
  let mut cfg = match config::Config::parse("config.toml") {
    Ok(c) => c,
    Err(e) => {
      println!("{}", e);
      exit(1);
    }
  };

  let mut v = vss::Vss::new();
  v.create(&cfg).unwrap(); // destroyed on drop
  let mapped_drives = v.get_mapped_drives();

  // Do variable subsitution in the repo.
  cfg.repo = cfg.repo.replace("{hostname}", &cfg.hostname).replace("{user}", &cfg.user);
  cfg.sources = cfg.sources
    .iter()
    .map(|source| {
      if !mapped_drives.is_empty() {
        if let Some(l) = mapped_drives.get(&source.chars().nth(0).unwrap()) {
          let mut s = String::new();
          s.push(*l);
          s.push_str(&source[1..]);
          return cygpath::from_win(s);
        }
      }

      cygpath::from_win(source)
    })
    .collect();

  backup::init(&cfg);
  backup::create(&cfg);
  backup::retention(&cfg);
  backup::consistency(&cfg);
}

fn main() {
  TermLogger::init(LogLevelFilter::Info).unwrap();

  let app = App::new("ckbackup")
    .setting(AppSettings::SubcommandRequiredElseHelp)
    .version("0.0.1")
    .author("Kevin Darlington <kevin@outroot.com>")
    .about("A VSS enabled wrapper around Borg to be used in automated scripts.")
    .subcommand(SubCommand::with_name("auto").about("Does automatic backup based on config.toml."));
  let matches = app.get_matches();

  match matches.subcommand() {
    ("auto", Some(_)) => cmd_auto(),
    _ => {}
  };
}
