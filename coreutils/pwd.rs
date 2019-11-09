use libc;
use libc::close;
use libc::free;
extern "C" {

  #[no_mangle]
  fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn puts(__s: *const libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn xrealloc_getcwd_or_warn(cwd: *mut libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn fflush_all() -> libc::c_int;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
}



use libc::stat;

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
  let mut st1: stat = std::mem::zeroed();
  let mut st2: stat = std::mem::zeroed();
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
