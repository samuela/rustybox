// #[cfg(all(unix, not(target_os = "macos")))]
fn main() {
  println!("cargo:rustc-flags=-l resolv");
}

// #[cfg(target_os = "macos")]
// fn main() {}
