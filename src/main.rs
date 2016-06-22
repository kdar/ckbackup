extern crate time;
extern crate toml;
extern crate rustc_serialize;
extern crate kernel32;
extern crate kernel32x;
extern crate powrprofx;
extern crate winapi;
#[macro_use]
extern crate log;
extern crate simplelog;
extern crate clap;
extern crate hyper;

use std::process::exit;
use simplelog::{TermLogger, LogLevelFilter};
use clap::{App, AppSettings, SubCommand};
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
#[cfg(not(debug_assertions))]
use std::path::Path;

mod config;
mod cygpath;
mod vss;
mod backup;
mod bootstrap;

fn to_wstring(str: &str) -> Vec<u16> {
  let v: Vec<u16> = OsStr::new(str).encode_wide().chain(Some(0).into_iter()).collect();
  v
}

#[cfg(debug_assertions)]
pub fn get_exe_dir() -> String {
  return ".".to_owned();
}

#[cfg(not(debug_assertions))]
pub fn get_exe_dir() -> String {
  let mut path: Vec<u16> = vec![0; winapi::MAX_PATH];
  unsafe {
    kernel32x::GetModuleFileNameW(0 as winapi::HMODULE,
                                  path.as_mut_ptr(),
                                  winapi::MAX_PATH as u32);
  }
  let path = String::from_utf16(path.as_slice()).unwrap();
  let path = Path::new(&path);
  let parent = path.parent().unwrap();
  parent.to_str().unwrap().to_owned()
}

fn cmd_auto() {
  let mut cfg = match config::Config::parse("config.toml") {
    Ok(c) => c,
    Err(e) => {
      error!("{}", e);
      exit(1);
    }
  };

  let mut v = vss::Vss::new();
  v.create(&cfg).unwrap(); // destroyed on drop
  let mapped_drives = v.get_mapped_drives();

  // Fix up the config.
  cfg.create.sources = cfg.create
    .sources
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
  backup::purge(&cfg);
  backup::check(&cfg);

  if let Some(post) = cfg.post {
    if post.sleep.unwrap_or(false) {
      unsafe {
        powrprofx::SetSuspendState(1, 0, 0);
      }
    }
  }
}

fn cmd_bootstrap() {
  match bootstrap::install() {
    Ok(_) => (),
    Err(err) => {
      error!("Error bootstrapping: {}", err);
      exit(1);
    }
  };
}

fn main() {
  TermLogger::init(LogLevelFilter::Info).unwrap();

  let app = App::new("ckbackup")
    .setting(AppSettings::SubcommandRequiredElseHelp)
    .version("0.0.1")
    .author("Kevin Darlington <kevin@outroot.com>")
    .about("A VSS enabled wrapper around Borg to be used in automated scripts.")
    .subcommand(SubCommand::with_name("auto").about("Does automatic backup based on config.toml."))
    .subcommand(SubCommand::with_name("bootstrap")
      .about("Downloads borg/cygwin and configures it."));
  let matches = app.get_matches();

  match matches.subcommand() {
    ("auto", Some(_)) => cmd_auto(),
    ("bootstrap", Some(_)) => cmd_bootstrap(),
    _ => {}
  };
}
