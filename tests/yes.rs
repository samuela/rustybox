mod common;
use common::exe;
use duct::cmd;

#[test]
fn no_args() {
  // Implicitly checks that exit code is zero.
  let stdout = cmd!(exe(), "yes")
    .pipe(cmd!("head", "-n", "5"))
    .read()
    .unwrap();
  assert_eq!(stdout, ["y"].repeat(5).join("\n"));
}

#[test]
fn one_arg() {
  let stdout = cmd!(exe(), "yes", "foo")
    .pipe(cmd!("head", "-n", "5"))
    .read()
    .unwrap();
  assert_eq!(stdout, ["foo"].repeat(5).join("\n"));
}

#[test]
fn multiple_args() {
  let stdout = cmd!(exe(), "yes", "foo", "bar")
    .pipe(cmd!("head", "-n", "5"))
    .read()
    .unwrap();
  assert_eq!(stdout, ["foo bar"].repeat(5).join("\n"));
}
