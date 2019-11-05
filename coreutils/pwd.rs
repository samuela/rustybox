use libc;
extern "C" {
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn puts(__s: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
  #[no_mangle]
  fn xrealloc_getcwd_or_warn(cwd: *mut libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn fflush_all() -> libc::c_int;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;
}
pub type __uint32_t = libc::c_uint;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type uint32_t = __uint32_t;
use crate::librb::stat;
use crate::librb::timespec;
/* vi: set sw=4 ts=4: */
/*
 * Mini pwd implementation for busybox
 *
 * Copyright (C) 1995, 1996 by Bruce Perens <bruce@pixar.com>.
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config PWD
//config:	bool "pwd (3.7 kb)"
//config:	default y
//config:	help
//config:	pwd is used to print the current directory.
//applet:IF_PWD(APPLET_NOFORK(pwd, pwd, BB_DIR_BIN, BB_SUID_DROP, pwd))
//kbuild:lib-$(CONFIG_PWD) += pwd.o
//usage:#define pwd_trivial_usage
//usage:       ""
//usage:#define pwd_full_usage "\n\n"
//usage:       "Print the full filename of the current working directory"
//usage:
//usage:#define pwd_example_usage
//usage:       "$ pwd\n"
//usage:       "/root\n"
unsafe extern "C" fn logical_getcwd() -> libc::c_int {
  let mut st1: stat = stat {
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
  let mut st2: stat = stat {
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
  let mut wd: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
  wd = getenv(b"PWD\x00" as *const u8 as *const libc::c_char);
  if wd.is_null() || *wd.offset(0) as libc::c_int != '/' as i32 {
    return 0i32;
  }
  p = wd;
  while *p != 0 {
    /* doing strstr(p, "/.") by hand is smaller and faster... */
    let fresh0 = p;
    p = p.offset(1);
    if *fresh0 as libc::c_int != '/' as i32 {
      continue;
    }
    if *p as libc::c_int != '.' as i32 {
      continue;
    }
    /* we found "/.", skip to next char */
    p = p.offset(1); /* we found "/.." */
    if *p as libc::c_int == '.' as i32 {
      p = p.offset(1)
    }
    if *p as libc::c_int == '\u{0}' as i32 || *p as libc::c_int == '/' as i32 {
      return 0i32;
    }
    /* "/./" or "/../" component: bad */
  }
  if stat(wd, &mut st1) != 0i32 {
    return 0i32;
  }
  if stat(b".\x00" as *const u8 as *const libc::c_char, &mut st2) != 0i32 {
    return 0i32;
  }
  if st1.st_ino != st2.st_ino {
    return 0i32;
  }
  if st1.st_dev != st2.st_dev {
    return 0i32;
  }
  puts(wd);
  return 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn pwd_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
  /* TODO: assume -L if $POSIXLY_CORRECT? (coreutils does that)
   * Rationale:
   * POSIX requires a default of -L, but most scripts expect -P
   */
  let mut opt: libc::c_uint = getopt32(argv, b"LP\x00" as *const u8 as *const libc::c_char);
  if opt & 1i32 as libc::c_uint != 0 && logical_getcwd() != 0 {
    return fflush_all();
  }
  buf = xrealloc_getcwd_or_warn(0 as *mut libc::c_char);
  if !buf.is_null() {
    puts(buf);
    free(buf as *mut libc::c_void);
    return fflush_all();
  }
  return 1i32;
}
