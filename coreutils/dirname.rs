use libc;
use libc::chdir;
use libc::chmod;
use libc::chown;
use libc::closelog;
use libc::dup2;
use libc::fstat;
use libc::getenv;
use libc::geteuid;
use libc::getopt;
use libc::getpid;
use libc::isatty;
use libc::kill;
use libc::openlog;
use libc::sigaddset;
use libc::sigemptyset;
use libc::sigprocmask;
use libc::sleep;
use libc::sscanf;
use libc::strcasecmp;
use libc::strcpy;
use libc::symlink;
use libc::syscall;
use libc::syslog;
use libc::time;
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
  fn dirname(__path: *mut libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn fflush_all() -> libc::c_int;
  #[no_mangle]
  fn single_argv(argv: *mut *mut libc::c_char) -> *mut libc::c_char;
}

/*
 * Mini dirname implementation for busybox
 *
 * Copyright (C) 1999-2004 by Erik Andersen <andersen@codepoet.org>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config DIRNAME
//config:	bool "dirname (329 bytes)"
//config:	default y
//config:	help
//config:	dirname is used to strip a non-directory suffix from
//config:	a file name.
//applet:IF_DIRNAME(APPLET_NOFORK(dirname, dirname, BB_DIR_USR_BIN, BB_SUID_DROP, dirname))
//kbuild:lib-$(CONFIG_DIRNAME) += dirname.o
/* BB_AUDIT SUSv3 compliant */
/* http://www.opengroup.org/onlinepubs/007904975/utilities/dirname.html */
//usage:#define dirname_trivial_usage
//usage:       "FILENAME"
//usage:#define dirname_full_usage "\n\n"
//usage:       "Strip non-directory suffix from FILENAME"
//usage:
//usage:#define dirname_example_usage
//usage:       "$ dirname /tmp/foo\n"
//usage:       "/tmp\n"
//usage:       "$ dirname /tmp/foo/\n"
//usage:       "/tmp\n"
/* This is a NOFORK applet. Be very careful! */
#[no_mangle]
pub unsafe extern "C" fn dirname_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  puts(dirname(single_argv(argv)));
  return fflush_all();
}
