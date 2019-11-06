use libc;
extern "C" {
  #[no_mangle]
  fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
  #[no_mangle]
  fn flock(__fd: libc::c_int, __operation: libc::c_int) -> libc::c_int;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn xatoi_positive(numstr: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn spawn_and_wait(argv: *mut *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn getopt32long(
    argv: *mut *mut libc::c_char,
    optstring: *const libc::c_char,
    longopts: *const libc::c_char,
    _: ...
  ) -> u32;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);
  #[no_mangle]
  fn bb_perror_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_perror_nomsg_and_die() -> !;
  #[no_mangle]
  fn get_shell_name() -> *const libc::c_char;
}


pub const OPT_c: C2RustUnnamed = 16;
pub const OPT_n: C2RustUnnamed = 4;
pub const OPT_s: C2RustUnnamed = 1;
pub const OPT_u: C2RustUnnamed = 8;
pub const OPT_x: C2RustUnnamed = 2;
pub type C2RustUnnamed = libc::c_uint;
/*
 * Copyright (C) 2010 Timo Teras <timo.teras@iki.fi>
 *
 * This is free software, licensed under the GNU General Public License v2.
 */
//config:config FLOCK
//config:	bool "flock (6.3 kb)"
//config:	default y
//config:	help
//config:	Manage locks from shell scripts
//applet:IF_FLOCK(APPLET(flock, BB_DIR_USR_BIN, BB_SUID_DROP))
//kbuild:lib-$(CONFIG_FLOCK) += flock.o
//usage:#define flock_trivial_usage
//usage:       "[-sxun] FD|{FILE [-c] PROG ARGS}"
//usage:#define flock_full_usage "\n\n"
//usage:       "[Un]lock file descriptor, or lock FILE, run PROG\n"
//usage:     "\n	-s	Shared lock"
//usage:     "\n	-x	Exclusive lock (default)"
//usage:     "\n	-u	Unlock FD"
//usage:     "\n	-n	Fail rather than wait"
#[no_mangle]
pub unsafe extern "C" fn flock_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut mode: libc::c_int = 0;
  let mut opt: libc::c_int = 0;
  let mut fd: libc::c_int = 0;
  static mut flock_longopts: [libc::c_char; 42] = [
    115, 104, 97, 114, 101, 100, 0, 0, 115, 101, 120, 99, 108, 117, 115, 105, 118, 101, 0, 0, 120,
    117, 110, 108, 111, 99, 107, 0, 0, 117, 110, 111, 110, 98, 108, 111, 99, 107, 0, 0, 110, 0,
  ];
  opt = getopt32long(
    argv,
    b"^+sxnu\x00-1\x00" as *const u8 as *const libc::c_char,
    flock_longopts.as_ptr(),
  ) as libc::c_int;
  argv = argv.offset(optind as isize);
  if !(*argv.offset(1)).is_null() {
    fd = open(*argv.offset(0), 0i32 | 0o400i32 | 0o100i32, 0o666i32);
    if fd < 0i32 && *bb_errno == 21i32 {
      fd = open(*argv.offset(0), 0i32 | 0o400i32)
    }
    if fd < 0i32 {
      bb_perror_msg_and_die(
        b"can\'t open \'%s\'\x00" as *const u8 as *const libc::c_char,
        *argv.offset(0),
      );
    }
  //TODO? close_on_exec_on(fd);
  } else {
    fd = xatoi_positive(*argv.offset(0))
  }
  argv = argv.offset(1);
  /* If it is "flock FILE -c PROG", then -c isn't caught by getopt32:
   * we use "+" in order to support "flock -opt FILE PROG -with-opts",
   * we need to remove -c by hand.
   */
  if !(*argv.offset(0)).is_null()
    && *(*argv.offset(0)).offset(0) as libc::c_int == '-' as i32
    && (*(*argv.offset(0)).offset(1) as libc::c_int == 'c' as i32
      && *(*argv.offset(0)).offset(2) == 0
      || 1i32 != 0
        && strcmp(
          (*argv.offset(0)).offset(1),
          b"-command\x00" as *const u8 as *const libc::c_char,
        ) == 0i32)
  {
    argv = argv.offset(1);
    if !(*argv.offset(1)).is_null() {
      bb_simple_error_msg_and_die(
        b"-c takes only one argument\x00" as *const u8 as *const libc::c_char,
      );
    }
    opt |= OPT_c as libc::c_int
  }
  if OPT_s as libc::c_int == 1i32
    && OPT_x as libc::c_int == 2i32
    && OPT_n as libc::c_int == 4i32
    && OPT_u as libc::c_int == 8i32
  {
    /* With suitably matched constants, mode setting is much simpler */
    mode = opt & 1i32 + 2i32 + 4i32 + 8i32;
    if mode & !4i32 == 0 {
      mode |= 2i32
    }
  } else {
    if opt & OPT_u as libc::c_int != 0 {
      mode = 8i32
    } else if opt & OPT_s as libc::c_int != 0 {
      mode = 1i32
    } else {
      mode = 2i32
    }
    if opt & OPT_n as libc::c_int != 0 {
      mode |= 4i32
    }
  }
  if flock(fd, mode) != 0i32 {
    if *bb_errno == 11i32 {
      return 1i32;
    }
    bb_perror_nomsg_and_die();
  }
  if !(*argv.offset(0)).is_null() {
    let mut rc: libc::c_int = 0;
    if opt & OPT_c as libc::c_int != 0 {
      /* -c 'PROG ARGS' means "run sh -c 'PROG ARGS'" */
      argv = argv.offset(-2);
      let ref mut fresh0 = *argv.offset(0);
      *fresh0 = get_shell_name() as *mut libc::c_char;
      let ref mut fresh1 = *argv.offset(1);
      *fresh1 = b"-c\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
      /* argv[2] = "PROG ARGS"; */
      /* argv[3] = NULL; */
    }
    rc = spawn_and_wait(argv);
    if rc < 0i32 {
      bb_simple_perror_msg(*argv.offset(0));
    }
    return rc;
  }
  return 0i32;
}
