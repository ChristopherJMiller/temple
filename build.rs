use std::env;
use std::time::SystemTime;
use chrono::offset::Utc;
use chrono::DateTime;

fn get_version_tag() -> String {
  let profile = env::var("PROFILE").unwrap();

  if profile == "release" {
    return env::var("CARGO_PKG_VERSION").unwrap();
  }

  let system_time = SystemTime::now();
  let datetime: DateTime<Utc> = system_time.into();

  return datetime.format("%y%m%d%H%M%S").to_string();
}

fn main() {
  println!("cargo:rustc-env=VERSION={}", get_version_tag());
}
