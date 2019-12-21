use libc;
use libc::free;
use libc::puts;

/*
 * Mar 16, 2003      Manuel Novoa III   (mjn3@codepoet.org)
 *
 * Now does proper error checking on output and returns a failure exit code
 * if one or more paths cannot be resolved.
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config REALPATH
//config:	bool "realpath (1.6 kb)"
//config:	default y
//config:	help
//config:	Return the canonicalized absolute pathname.
//config:	This isn't provided by GNU shellutils, but where else does it belong.
//applet:IF_REALPATH(APPLET_NOFORK(realpath, realpath, BB_DIR_USR_BIN, SUID_DROP, realpath))
//kbuild:lib-$(CONFIG_REALPATH) += realpath.o
/* BB_AUDIT SUSv3 N/A -- Apparently a busybox extension. */
//usage:#define realpath_trivial_usage
//usage:       "FILE..."
//usage:#define realpath_full_usage "\n\n"
//usage:       "Return the absolute pathnames of given FILE"
#[no_mangle]
pub unsafe extern "C" fn realpath_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut retval: libc::c_int = 0i32;
  argv = argv.offset(1);
  if (*argv).is_null() {
    crate::libbb::appletlib::bb_show_usage();
  }
  loop {
    /* NOFORK: only one alloc is allowed; must free */
    let mut resolved_path: *mut libc::c_char =
      crate::libbb::xreadlink::xmalloc_realpath_coreutils(*argv);
    if !resolved_path.is_null() {
      puts(resolved_path);
      free(resolved_path as *mut libc::c_void);
    } else {
      retval = 1i32;
      crate::libbb::perror_msg::bb_simple_perror_msg(*argv);
    }
    argv = argv.offset(1);
    if (*argv).is_null() {
      break;
    }
  }
  crate::libbb::fflush_stdout_and_exit::fflush_stdout_and_exit(retval);
}
