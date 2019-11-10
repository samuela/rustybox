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
  fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;

  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn get_console_fd_or_die() -> libc::c_int;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn ioctl_or_perror(
    fd: libc::c_int,
    request: libc::c_uint,
    argp: *mut libc::c_void,
    fmt: *const libc::c_char,
    _: ...
  ) -> libc::c_int;
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
}

use crate::librb::size_t;
use libc::ssize_t;

/*
 * Mini dumpkmap implementation for busybox
 *
 * Copyright (C) Arne Bernin <arne@matrix.loopback.org>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config DUMPKMAP
//config:	bool "dumpkmap (1.6 kb)"
//config:	default y
//config:	select PLATFORM_LINUX
//config:	help
//config:	This program dumps the kernel's keyboard translation table to
//config:	stdout, in binary format. You can then use loadkmap to load it.
//applet:IF_DUMPKMAP(APPLET_NOEXEC(dumpkmap, dumpkmap, BB_DIR_BIN, BB_SUID_DROP, dumpkmap))
/* bb_common_bufsiz1 usage here is safe wrt NOEXEC: not expecting it to be zeroed. */
//kbuild:lib-$(CONFIG_DUMPKMAP) += dumpkmap.o
//usage:#define dumpkmap_trivial_usage
//usage:       "> keymap"
//usage:#define dumpkmap_full_usage "\n\n"
//usage:       "Print a binary keyboard translation table to stdout"
//usage:
//usage:#define dumpkmap_example_usage
//usage:       "$ dumpkmap > keymap\n"
/* From <linux/kd.h> */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kbentry {
  pub kb_table: libc::c_uchar,
  pub kb_index: libc::c_uchar,
  pub kb_value: libc::c_ushort,
}
#[no_mangle]
pub unsafe extern "C" fn dumpkmap_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut ke: kbentry = kbentry {
    kb_table: 0,
    kb_index: 0,
    kb_value: 0,
  };
  let mut i: libc::c_int = 0;
  let mut j: libc::c_int = 0;
  let mut fd: libc::c_int = 0;
  /* When user accidentally runs "dumpkmap FILE"
   * instead of "dumpkmap >FILE", we'd dump binary stuff to tty.
   * Let's prevent it:
   */
  if !(*argv.offset(1)).is_null() {
    bb_show_usage();
  }
  /*	bb_warn_ignoring_args(argv[1]);*/
  fd = get_console_fd_or_die();
  /*                     0 1 2 3 4 5 6 7 8 9 a b c=12 */
  memcpy(
    bb_common_bufsiz1.as_mut_ptr() as *mut libc::c_void,
    b"bkeymap\x01\x01\x01\x00\x01\x01\x01\x00\x01\x01\x01\x00\x01\x00" as *const u8
      as *const libc::c_char as *const libc::c_void,
    ::std::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong,
  );
  write(
    1i32,
    bb_common_bufsiz1.as_mut_ptr() as *const libc::c_void,
    (7i32 + 256i32) as size_t,
  );
  i = 0i32;
  while i < 13i32 {
    if *bb_common_bufsiz1.as_mut_ptr().offset(7).offset(i as isize) != 0 {
      j = 0i32;
      while j < 128i32 {
        ke.kb_index = j as libc::c_uchar;
        ke.kb_table = i as libc::c_uchar;
        if ioctl_or_perror(
          fd,
          0x4b46i32 as libc::c_uint,
          &mut ke as *mut kbentry as *mut libc::c_void,
          b"ioctl(KDGKBENT{%d,%d}) failed\x00" as *const u8 as *const libc::c_char,
          j,
          i,
        ) == 0
        {
          write(
            1i32,
            &mut ke.kb_value as *mut libc::c_ushort as *const libc::c_void,
            2i32 as size_t,
          );
        }
        j += 1
      }
    }
    i += 1
  }
  return 0i32;
}
