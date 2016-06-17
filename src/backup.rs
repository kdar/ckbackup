use std::process::{Command, Stdio, exit};
use time;

use config;

fn run_borg(borg_cmd: &Command) {
  let mut cmd = Command::new("vendor\\borg\\bin\\bash");
  cmd.arg("--login")
    .arg("-c")
    .arg(format!("{:?}", borg_cmd))
    .env("SSH_AUTH_SOCK", "")
    .stdout(Stdio::inherit())
    .stderr(Stdio::inherit());

  match cmd.output() {
    Ok(_) => (),
    Err(e) => {
      println!("{}: {:?}", e, cmd);
      exit(1);
    }
  };
}

pub fn init(cfg: &config::Config) {
  info!("Initializing backup: {}", cfg.repo);
  run_borg(Command::new("/bin/borg")
    .arg("init")
    .arg(&cfg.repo)
    .arg("--verbose"));
}

pub fn create(cfg: &config::Config) {
  let tm = time::now_utc();
  let tm = tm.rfc3339();

  info!("Creating backup: {}", cfg.repo);

  let mut cmd = Command::new("/bin/borg");
  cmd.arg("create")
    .arg(format!("{}::{}", &cfg.repo, tm));
  for source in &cfg.sources {
    cmd.arg(source);
  }
  if let Some(compression) = cfg.compression.clone() {
    cmd.arg("--compression")
      .arg(compression);
  }
  if cfg.one_file_system.unwrap_or(false) {
    cmd.arg("--one-file-system");
  }
  cmd.arg("--stats")
    .arg("--verbose")
    .arg("--progress");

  info!("Running: {:?}", cmd);

  run_borg(&cmd);
}

pub fn retention(cfg: &config::Config) {
  if let Some(retention) = cfg.retention.clone() {
    info!("Processing backup retention...");

    let mut cmd = Command::new("/bin/borg");
    cmd.arg("prune")
      .arg(&cfg.repo)
      .arg("--stats")
      .arg("--verbose");
    if let Some(within) = retention.keep_within {
      cmd.arg(format!("--keep-within={}", within));
    }
    if let Some(hourly) = retention.keep_hourly {
      cmd.arg(format!("--keep-hourly={}", hourly));
    }
    if let Some(daily) = retention.keep_daily {
      cmd.arg(format!("--keep-daily={}", daily));
    }
    if let Some(weekly) = retention.keep_weekly {
      cmd.arg(format!("--keep-weekly={}", weekly));
    }
    if let Some(monthly) = retention.keep_monthly {
      cmd.arg(format!("--keep-monthly={}", monthly));
    }
    if let Some(yearly) = retention.keep_yearly {
      cmd.arg(format!("--keep-yearly={}", yearly));
    }

    info!("Running: {:?}", cmd);

    run_borg(&cmd);
  }
}

pub fn consistency(cfg: &config::Config) {
  if let Some(consistency) = cfg.consistency.clone() {
    if consistency.check.unwrap_or(false) {
      info!("Checking backup consistency...");

      let mut cmd = Command::new("/bin/borg");
      cmd.arg("check")
        .arg(&cfg.repo)
        .arg("--verbose");

      info!("Running: {:?}", cmd);

      run_borg(&cmd);
    }
  }
}
