use libc;
extern "C" {
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  static mut applet_name: *const libc::c_char;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  static mut stdin: *mut FILE;
  #[no_mangle]
  static mut stdout: *mut FILE;
  #[no_mangle]
  fn fclose(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn getc_unlocked(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn putc_unlocked(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn fileno_unlocked(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn fchmod(__fd: libc::c_int, __mode: __mode_t) -> libc::c_int;
  #[no_mangle]
  fn xmalloc_follow_symlinks(path: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xfstat(fd: libc::c_int, buf: *mut stat, errmsg: *const libc::c_char);
  #[no_mangle]
  fn xrename(oldpath: *const libc::c_char, newpath: *const libc::c_char);
  #[no_mangle]
  fn xmkstemp(template: *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  #[no_mangle]
  fn xfopen_for_read(path: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn xfdopen_for_write(fd: libc::c_int) -> *mut FILE;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_perror_nomsg_and_die() -> !;
  #[no_mangle]
  fn fchown(__fd: libc::c_int, __owner: __uid_t, __group: __gid_t) -> libc::c_int;
  #[no_mangle]
  fn unlink(__name: *const libc::c_char) -> libc::c_int;
}

use crate::librb::__blkcnt_t;
use crate::librb::__blksize_t;
use crate::librb::__dev_t;
use crate::librb::__gid_t;
use crate::librb::__ino_t;
use crate::librb::__mode_t;
use crate::librb::__nlink_t;
use crate::librb::__off64_t;
use crate::librb::__off_t;
use crate::librb::__syscall_slong_t;
use crate::librb::__time_t;
use crate::librb::__uid_t;
use crate::librb::stat;
use crate::librb::timespec;
use crate::librb::uint32_t;
use crate::librb::FILE;

/*
 * dos2unix for BusyBox
 *
 * dos2unix '\n' converter 0.5.0
 * based on Unix2Dos 0.9.0 by Peter Hanecak (made 19.2.1997)
 * Copyright 1997,.. by Peter Hanecak <hanecak@megaloman.sk>.
 * All rights reserved.
 *
 * dos2unix filters reading input from stdin and writing output to stdout.
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config DOS2UNIX
//config:	bool "dos2unix (5.2 kb)"
//config:	default y
//config:	help
//config:	dos2unix is used to convert a text file from DOS format to
//config:	UNIX format, and vice versa.
//config:
//config:config UNIX2DOS
//config:	bool "unix2dos (5.2 kb)"
//config:	default y
//config:	help
//config:	unix2dos is used to convert a text file from UNIX format to
//config:	DOS format, and vice versa.
//applet:IF_DOS2UNIX(APPLET_NOEXEC(dos2unix, dos2unix, BB_DIR_USR_BIN, BB_SUID_DROP, dos2unix))
//applet:IF_UNIX2DOS(APPLET_NOEXEC(unix2dos, dos2unix, BB_DIR_USR_BIN, BB_SUID_DROP, unix2dos))
//kbuild:lib-$(CONFIG_DOS2UNIX) += dos2unix.o
//kbuild:lib-$(CONFIG_UNIX2DOS) += dos2unix.o
//usage:#define dos2unix_trivial_usage
//usage:       "[-ud] [FILE]"
//usage:#define dos2unix_full_usage "\n\n"
//usage:       "Convert FILE in-place from DOS to Unix format.\n"
//usage:       "When no file is given, use stdin/stdout.\n"
//usage:     "\n	-u	dos2unix"
//usage:     "\n	-d	unix2dos"
//usage:
//usage:#define unix2dos_trivial_usage
//usage:       "[-ud] [FILE]"
//usage:#define unix2dos_full_usage "\n\n"
//usage:       "Convert FILE in-place from Unix to DOS format.\n"
//usage:       "When no file is given, use stdin/stdout.\n"
//usage:     "\n	-u	dos2unix"
//usage:     "\n	-d	unix2dos"
/* This is a NOEXEC applet. Be very careful! */
pub type C2RustUnnamed = libc::c_uint;
pub const CT_DOS2UNIX: C2RustUnnamed = 2;
pub const CT_UNIX2DOS: C2RustUnnamed = 1;
/* if fn is NULL then input is stdin and output is stdout */
unsafe extern "C" fn convert(mut fn_0: *mut libc::c_char, mut conv_type: libc::c_int) {
  let mut in_0: *mut FILE = 0 as *mut FILE; /* for compiler */
  let mut out: *mut FILE = 0 as *mut FILE;
  let mut ch: libc::c_int = 0;
  let mut temp_fn: *mut libc::c_char = 0 as *mut libc::c_char;
  temp_fn = temp_fn;
  let mut resolved_fn: *mut libc::c_char = 0 as *mut libc::c_char;
  resolved_fn = resolved_fn;
  in_0 = stdin;
  out = stdout;
  if !fn_0.is_null() {
    let mut st: stat = stat {
      st_dev: 0,
      st_ino: 0,
      st_nlink: 0,
      st_mode: 0,
      st_uid: 0,
      st_gid: 0,
      __pad0: 0,
      st_rdev: 0,
      st_size: 0,
      st_blksize: 0,
      st_blocks: 0,
      st_atim: timespec {
        tv_sec: 0,
        tv_nsec: 0,
      },
      st_mtim: timespec {
        tv_sec: 0,
        tv_nsec: 0,
      },
      st_ctim: timespec {
        tv_sec: 0,
        tv_nsec: 0,
      },
      __glibc_reserved: [0; 3],
    };
    let mut fd: libc::c_int = 0;
    resolved_fn = xmalloc_follow_symlinks(fn_0);
    if resolved_fn.is_null() {
      bb_simple_perror_msg_and_die(fn_0);
    }
    in_0 = xfopen_for_read(resolved_fn);
    xfstat(fileno_unlocked(in_0), &mut st, resolved_fn);
    temp_fn = xasprintf(
      b"%sXXXXXX\x00" as *const u8 as *const libc::c_char,
      resolved_fn,
    );
    fd = xmkstemp(temp_fn);
    if fchmod(fd, st.st_mode) == -1i32 {
      bb_simple_perror_msg_and_die(temp_fn);
    }
    fchown(fd, st.st_uid, st.st_gid);
    out = xfdopen_for_write(fd)
  }
  loop {
    ch = getc_unlocked(in_0);
    if !(ch != -1i32) {
      break;
    }
    if ch == '\r' as i32 {
      continue;
    }
    if ch == '\n' as i32 {
      if conv_type == CT_UNIX2DOS as libc::c_int {
        putc_unlocked('\r' as i32, out);
      }
    }
    putc_unlocked(ch, out);
  }
  if !fn_0.is_null() {
    if fclose(in_0) < 0i32 || fclose(out) < 0i32 {
      unlink(temp_fn);
      bb_perror_nomsg_and_die();
    }
    xrename(temp_fn, resolved_fn);
    free(temp_fn as *mut libc::c_void);
    free(resolved_fn as *mut libc::c_void);
  };
}
#[no_mangle]
pub unsafe extern "C" fn dos2unix_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut o: libc::c_int = 0;
  let mut conv_type: libc::c_int = 0;
  /* See if we are supposed to be doing dos2unix or unix2dos */
  if 1i32 != 0 && (1i32 == 0 || *applet_name.offset(0) as libc::c_int == 'd' as i32) {
    conv_type = CT_DOS2UNIX as libc::c_int
  } else {
    conv_type = CT_UNIX2DOS as libc::c_int
  }
  /* -u convert to unix, -d convert to dos */
  o = getopt32(
    argv,
    b"^du\x00u--d:d--u\x00" as *const u8 as *const libc::c_char,
  ) as libc::c_int; /* mutually exclusive */
  /* Do the conversion requested by an argument else do the default
   * conversion depending on our name.  */
  if o != 0 {
    conv_type = o
  }
  argv = argv.offset(optind as isize);
  loop {
    /* might be convert(NULL) if there is no filename given */
    convert(*argv, conv_type);
    if !(!(*argv).is_null() && {
      argv = argv.offset(1);
      !(*argv).is_null()
    }) {
      break;
    }
  }
  return 0i32;
}
