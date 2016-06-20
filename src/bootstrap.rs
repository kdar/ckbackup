use hyper::client::Client;
use std::fs::File;
use std::io::{Read, Write};
use std;
use std::env;
use std::process::{Command, Stdio};

const NSSWITCH: &'static str = r#"# /etc/nsswitch.conf
#
#    This file is read once by the first process in a Cygwin process tree.
#    To pick up changes, restart all Cygwin processes.  For a description
#    see https://cygwin.com/cygwin-ug-net/ntsec.html#ntsec-mapping-nsswitch
#
# Defaults:
# passwd:   files db
# group:    files db
# db_enum:  cache builtin
# db_home:  /home/%U
db_home: windows
# db_shell: /bin/bash
# db_gecos: <empty>
"#;

macro_rules! try_s {
  ($expr:expr) => (match $expr {
    std::result::Result::Ok(val) => val,
    std::result::Result::Err(err) => {
      return std::result::Result::Err(err.to_string());
    }
  })
}

macro_rules! cmd {
  ($y:expr, $( $x:expr ),* ) => ({
    let mut cmd = Command::new($y);
    $(
     cmd.arg($x);
    )*
    cmd.stdout(Stdio::inherit())
    .stderr(Stdio::inherit())
    .output()
  });
}

fn download_setup() -> Result<String, String> {
  let mut dir = env::temp_dir();
  dir.push("setup-x86_64.exe");

  info!("Downloading https://cygwin.com/setup-x86_64.exe to {:?}",
        &dir);

  let mut f = try_s!(File::create(&dir));

  let client = Client::new();
  let mut res = try_s!(client.get("https://cygwin.com/setup-x86_64.exe").send());

  let mut buf = [0u8; 1024 * 1024];
  loop {
    let n = try_s!(res.read(&mut buf));
    if n == 0 {
      break;
    } else {
      try_s!(f.write_all(&mut buf[..n]));
    }
  }

  Ok(dir.to_str().unwrap().to_owned())
}

// #[cfg_attr(rustfmt, rustfmt_skip)]
pub fn install() -> Result<(), String> {
  let setup = try_s!(download_setup());

  // let setup = "C:\\Users\\outroot\\AppData\\Local\\Temp\\setup-x86_64.exe";

  let cygbuild = super::get_exe_dir() + "\\vendor\\borg";
  let cygmirror = "ftp://ftp.funet.fi/pub/mirrors/cygwin.com/pub/cygwin/";
  let buildpkgs = "python3,python3-setuptools,binutils,gcc-g++,libopenssl,openssl-devel,git,make,\
                   openssh,liblz4-devel,liblz4_1";

  info!("Installing cygwin...");
  try_s!(cmd!(setup,
              "-q",
              "-B",
              "-o",
              "-n",
              "-R",
              &cygbuild,
              "-L",
              "-l",
              env::temp_dir(),
              "-D",
              "-s",
              cygmirror,
              "-P",
              buildpkgs));

  info!("Installing borg...");
  try_s!(cmd!(format!("{}\\bin\\bash", &cygbuild),
              "--login",
              "-c",
              "easy_install-3.4 --upgrade pip"));
  try_s!(cmd!(format!("{}\\bin\\bash", &cygbuild),
              "--login",
              "-c",
              "pip install --upgrade borgbackup"));

  let mut f = try_s!(File::create(format!("{}\\etc\\nsswitch.conf", &cygbuild)));
  try_s!(f.write(NSSWITCH.as_bytes()));

  Ok(())
}
