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
  fn gnu_dev_makedev(__major: libc::c_uint, __minor: libc::c_uint) -> libc::dev_t;
}

/*
 * Utility routines.
 *
 * Copyright (C) 2006 Denys Vlasenko
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
/* We do not include libbb.h - #define makedev() is there! */
/* Different Unixes want different headers for makedev */
/* At least glibc has horrendously large inline for this, so wrap it. */
/* uclibc people please check - do we need "&& !__UCLIBC__" above? */
/* Suppress gcc "no previous prototype" warning */
#[no_mangle]
pub unsafe extern "C" fn bb_makedev(
  mut major: libc::c_uint,
  mut minor: libc::c_uint,
) -> libc::c_ulonglong {
  return gnu_dev_makedev(major, minor) as libc::c_ulonglong;
}
