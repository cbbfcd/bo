extern crate clap;

use std::process::Command;
use clap::Clap;

#[derive(Clap)]
#[clap(version = "0.0.1", author = "波比小金刚", about = "🈯️ just some command line tools commonly used by individuals.")]
struct Opts {

  // set github config for local or global.
  #[clap(long, default_value = "local")]
  gc: String,
}

fn main() {
  let opts: Opts = Opts::parse();

  // for github config
  if "local" == &opts.gc || "l" == &opts.gc {

    // exec
    Command::new("git")
      .arg("config")
      .arg("user.name")
      .arg("\"cbbfcd\"")
      .spawn()
      .expect("failed to execute set user.name process.");

    Command::new("git")
      .arg("config")
      .arg("user.email")
      .arg("\"2890636389@qq.com\"")
      .spawn()
      .expect("failed to execute set user.email process.");

    let user_name = Command::new("git")
      .arg("config")
      .arg("--get")
      .arg("user.name")
      .output()
      .expect("failed to execute get user.name process.");
    
    let user_email = Command::new("git")
      .arg("config")
      .arg("--get")
      .arg("user.email")
      .output()
      .expect("failed to execute get user.email process.");

    println!("now, your local user.name is {}", String::from_utf8(user_name.stdout).unwrap());
    println!("now, your local user.email is {}", String::from_utf8(user_email.stdout).unwrap());
  }
}
