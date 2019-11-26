use libc;
use libc::c_char;
use libc::c_int;
use std::env;
use std::io;
use std::io::Write;

#[no_mangle]
pub unsafe extern "C" fn yes_main(mut _argc: c_int, mut _argv: *const *const c_char) -> c_int {
  let line = if env::args().count() > 1 {
    env::args().collect::<Vec<String>>().join(" ")
  } else {
    "y".to_string()
  };
  let mut stdout = io::stdout();
  loop {
    print!("{}", line);
    if let Err(_) = stdout.write_all(b"\n") {
      break;
    }
  }
  0
}
