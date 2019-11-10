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
  fn gethostid() -> libc::c_long;

  #[no_mangle]
  fn fflush_all() -> libc::c_int;
  #[no_mangle]
  fn bb_show_usage() -> !;
}



/*
 * Mini hostid implementation for busybox
 *
 * Copyright (C) 2000  Edward Betts <edward@debian.org>.
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config HOSTID
//config:	bool "hostid (286 bytes)"
//config:	default y
//config:	help
//config:	hostid prints the numeric identifier (in hexadecimal) for
//config:	the current host.
//applet:IF_HOSTID(APPLET_NOFORK(hostid, hostid, BB_DIR_USR_BIN, BB_SUID_DROP, hostid))
//kbuild:lib-$(CONFIG_HOSTID) += hostid.o
/* BB_AUDIT SUSv3 N/A -- Matches GNU behavior. */
//usage:#define hostid_trivial_usage
//usage:       ""
//usage:#define hostid_full_usage "\n\n"
//usage:       "Print out a unique 32-bit identifier for the machine"
/* This is a NOFORK applet. Be very careful! */
#[no_mangle]
pub unsafe extern "C" fn hostid_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  if !(*argv.offset(1)).is_null() {
    bb_show_usage();
  }
  /* POSIX says gethostid returns a "32-bit identifier" */
  printf(
    b"%08x\n\x00" as *const u8 as *const libc::c_char,
    gethostid() as u32,
  );
  return fflush_all();
}
