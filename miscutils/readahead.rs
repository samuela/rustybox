use libc;
extern "C" {
  #[no_mangle]
  fn readahead(__fd: libc::c_int, __offset: __off64_t, __count: size_t) -> __ssize_t;
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn open_or_warn(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xlseek(fd: libc::c_int, offset: off_t, whence: libc::c_int) -> off_t;
  #[no_mangle]
  fn bb_show_usage() -> !;
}
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
use crate::librb::size_t;
use crate::librb::off_t;

/*
 * readahead implementation for busybox
 *
 * Preloads the given files in RAM, to reduce access time.
 * Does this by calling the readahead(2) system call.
 *
 * Copyright (C) 2006  Michael Opdenacker <michael@free-electrons.com>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config READAHEAD
//config:	bool "readahead (1.5 kb)"
//config:	default y
//config:	depends on LFS
//config:	select PLATFORM_LINUX
//config:	help
//config:	Preload the files listed on the command line into RAM cache so that
//config:	subsequent reads on these files will not block on disk I/O.
//config:
//config:	This applet just calls the readahead(2) system call on each file.
//config:	It is mainly useful in system startup scripts to preload files
//config:	or executables before they are used. When used at the right time
//config:	(in particular when a CPU bound process is running) it can
//config:	significantly speed up system startup.
//config:
//config:	As readahead(2) blocks until each file has been read, it is best to
//config:	run this applet as a background job.
//applet:IF_READAHEAD(APPLET(readahead, BB_DIR_USR_SBIN, BB_SUID_DROP))
//kbuild:lib-$(CONFIG_READAHEAD) += readahead.o
//usage:#define readahead_trivial_usage
//usage:       "[FILE]..."
//usage:#define readahead_full_usage "\n\n"
//usage:       "Preload FILEs to RAM"
#[no_mangle]
pub unsafe extern "C" fn readahead_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut retval: libc::c_int = 0i32;
  if (*argv.offset(1)).is_null() {
    bb_show_usage();
  }
  loop {
    argv = argv.offset(1);
    if (*argv).is_null() {
      break;
    }
    let mut fd: libc::c_int = open_or_warn(*argv, 0i32);
    if fd >= 0i32 {
      let mut len: off_t = 0;
      let mut r: libc::c_int = 0;
      /* fdlength was reported to be unreliable - use seek */
      len = xlseek(fd, 0i32 as off_t, 2i32);
      xlseek(fd, 0i32 as off_t, 0i32);
      r = readahead(fd, 0i32 as __off64_t, len as size_t) as libc::c_int;
      close(fd);
      if r >= 0i32 {
        continue;
      }
    }
    retval = 1i32
  }
  return retval;
}
