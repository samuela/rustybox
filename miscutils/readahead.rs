use libc;
use libc::close;
extern "C" {
  #[no_mangle]
  fn readahead(__fd: libc::c_int, __offset: off64_t, __count: size_t) -> ssize_t;

}
use crate::librb::size_t;
use libc::off64_t;
use libc::off_t;
use libc::ssize_t;
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
//applet:IF_READAHEAD(APPLET(readahead, BB_DIR_USR_SBIN, SUID_DROP))
//kbuild:lib-$(CONFIG_READAHEAD) += readahead.o
//usage:#define readahead_trivial_usage
//usage:       "[FILE]..."
//usage:#define readahead_full_usage "\n\n"
//usage:       "Preload FILEs to RAM"
pub unsafe fn readahead_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut retval: libc::c_int = 0;
  if (*argv.offset(1)).is_null() {
    crate::libbb::appletlib::bb_show_usage();
  }
  loop {
    argv = argv.offset(1);
    if (*argv).is_null() {
      break;
    }
    let mut fd: libc::c_int = crate::libbb::xfuncs_printf::open_or_warn(*argv, 0);
    if fd >= 0 {
      let mut len: off_t = 0;
      let mut r: libc::c_int = 0;
      /* fdlength was reported to be unreliable - use seek */
      len = crate::libbb::xfuncs_printf::xlseek(fd, 0 as off_t, 2i32);
      crate::libbb::xfuncs_printf::xlseek(fd, 0 as off_t, 0);
      r = readahead(fd, 0 as off64_t, len as size_t) as libc::c_int;
      close(fd);
      if r >= 0 {
        continue;
      }
    }
    retval = 1i32
  }
  return retval;
}
