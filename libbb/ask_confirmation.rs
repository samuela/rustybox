use libc;

use crate::librb::{FILE, _IO_FILE};

extern "C" {
  #[no_mangle]
  static mut stdin: *mut _IO_FILE;

  #[no_mangle]
  fn getc_unlocked(__stream: *mut FILE) -> libc::c_int;

  #[no_mangle]
  fn fflush_all() -> libc::c_int;
}

/* Read a line from fp.  If the first non-whitespace char is 'y' or 'Y',
 * return 1.  Otherwise return 0.
 */
#[no_mangle]
pub unsafe extern "C" fn bb_ask_y_confirmation_FILE(mut fp: *mut FILE) -> libc::c_int {
  let mut first: libc::c_char = 0i32 as libc::c_char;
  let mut c: libc::c_int = 0;
  fflush_all();
  loop {
    c = getc_unlocked(fp);
    if !(c != -1i32 && c != '\n' as i32) {
      break;
    }
    if first as libc::c_int == 0i32
      && ({
        let mut bb__isblank: libc::c_uchar = c as libc::c_uchar;
        (bb__isblank as libc::c_int == ' ' as i32 || bb__isblank as libc::c_int == '\t' as i32)
          as libc::c_int
      }) == 0
    {
      first = (c | 0x20i32) as libc::c_char
    }
  }
  return (first as libc::c_int == 'y' as i32) as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn bb_ask_y_confirmation() -> libc::c_int {
  return bb_ask_y_confirmation_FILE(stdin);
}
