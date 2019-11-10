use libc;
use libc::access;
use libc::atoi;
use libc::fclose;
use libc::fprintf;
use libc::lstat;
use libc::printf;
use libc::puts;
use libc::rename;
use libc::rmdir;
use libc::sprintf;
use libc::strchr;
use libc::strcmp;
use libc::strrchr;
use libc::strstr;
use libc::system;




extern "C" {
  #[no_mangle]
  fn safe_write(fd: libc::c_int, buf: *const libc::c_void, count: size_t) -> ssize_t;
}

use crate::librb::size_t;
use libc::ssize_t;
// NB: will return short write on error, not -1,
// if some data was written before error occurred

/*
 * Utility routines.
 *
 * Copyright (C) 1999-2004 by Erik Andersen <andersen@codepoet.org>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/*
 * Write all of the supplied buffer out to a file.
 * This does multiple writes as necessary.
 * Returns the amount written, or -1 if error was seen
 * on the very first write.
 */
#[no_mangle]
pub unsafe extern "C" fn full_write(
  mut fd: libc::c_int,
  mut buf: *const libc::c_void,
  mut len: size_t,
) -> ssize_t {
  let mut cc: ssize_t = 0;
  let mut total: ssize_t = 0;
  total = 0i32 as ssize_t;
  while len != 0 {
    cc = safe_write(fd, buf, len);
    if cc < 0 {
      if total != 0 {
        /* we already wrote some! */
        /* user can do another write to know the error code */
        return total;
      }
      return cc;
      /* write() returns -1 on failure. */
    }
    total += cc;
    buf = (buf as *const libc::c_char).offset(cc as isize) as *const libc::c_void;
    len = (len as libc::c_ulong).wrapping_sub(cc as libc::c_ulong) as size_t as size_t
  }
  return total;
}
