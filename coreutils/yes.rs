use crate::libbb::perror_nomsg_and_die::bb_perror_nomsg_and_die;
use libc;
use libc::c_char;
use libc::c_int;
use std::ffi::CStr;
use std::io;
use std::io::Write;
use std::slice;

#[no_mangle]
pub unsafe extern "C" fn yes_main(mut argc: c_int, mut argv: *const *const c_char) -> c_int {
  let line = if argc > 1 {
    slice::from_raw_parts(argv, argc as usize)
      .iter()
      .map(|c_char_arg| CStr::from_ptr(*c_char_arg).to_str().unwrap())
      .collect::<Vec<&str>>()
      .join(" ")
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
  bb_perror_nomsg_and_die();
  0
}
