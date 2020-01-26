// See https://github.com/rust-lang/cargo/blob/19fdb308cdbb25faf4f1e25a71351d8d603fa447/tests/cargotest/support/mod.rs#L306.
pub fn exe() -> String {
  let mut path = std::env::current_exe().unwrap();

  // Drop the current executable file name.
  path.pop();

  // Get out of deps folder if we're in there.
  if path.ends_with("deps") {
    path.pop();
  }

  path
    .join(format!("rustybox{}", std::env::consts::EXE_SUFFIX))
    .to_str()
    .unwrap()
    .to_string()
}
