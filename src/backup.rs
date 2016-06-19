use std::process::{Command, Stdio, exit};
use time;

use config;

fn run_borg(cfg: &config::Config, borg_cmd: &Command) {
  let mut cmd = Command::new(super::get_vendor_dir() + "\\borg\\bin\\bash");
  cmd.arg("--login")
    .arg("-c")
    .arg(format!("{:?}", borg_cmd))
    .stdout(Stdio::inherit())
    .stderr(Stdio::inherit());

  if let Some(env) = cfg.general.env.clone() {
    for (k, v) in env {
      cmd.env(k, v);
    }
  }

  match cmd.output() {
    Ok(_) => (),
    Err(e) => {
      error!("{}: {:?}", e, cmd);
      exit(1);
    }
  };
}

pub fn init(cfg: &config::Config) {
  if let Some(init) = cfg.init.clone() {
    info!("Initializing backup: {}", cfg.general.repo);

    let mut cmd = Command::new("/bin/borg");
    cmd.arg("init")
      .arg(&cfg.general.repo);

    if let Some(args) = init.args {
      for arg in args {
        cmd.arg(arg);
      }
    }

    info!("Running: {:?}", cmd);

    run_borg(cfg, &cmd);
  }
}

pub fn create(cfg: &config::Config) {
  let tm = time::now_utc();
  let tm = tm.rfc3339();

  info!("Creating backup: {}", cfg.general.repo);

  let mut cmd = Command::new("/bin/borg");
  cmd.arg("create")
    .arg(format!("{}::{}", &cfg.general.repo, tm));
  for source in &cfg.create.sources {
    cmd.arg(source);
  }

  if let Some(args) = cfg.create.args.clone() {
    for arg in args {
      cmd.arg(arg);
    }
  }

  info!("Running: {:?}", cmd);

  run_borg(cfg, &cmd);
}

pub fn purge(cfg: &config::Config) {
  if let Some(purge) = cfg.purge.clone() {
    info!("Purging backup: {}", cfg.general.repo);

    let mut cmd = Command::new("/bin/borg");
    cmd.arg("prune")
      .arg(&cfg.general.repo);

    if let Some(args) = purge.args {
      for arg in args {
        cmd.arg(arg);
      }
    }

    info!("Running: {:?}", cmd);

    run_borg(cfg, &cmd);
  }
}

pub fn check(cfg: &config::Config) {
  if let Some(check) = cfg.check.clone() {
    info!("Checking backup: {}", cfg.general.repo);

    let mut cmd = Command::new("/bin/borg");
    cmd.arg("check")
      .arg(&cfg.general.repo);

    if let Some(args) = check.args {
      for arg in args {
        cmd.arg(arg);
      }
    }

    info!("Running: {:?}", cmd);

    run_borg(cfg, &cmd);
  }
}
