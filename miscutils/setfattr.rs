use libc;
extern "C" {
  #[no_mangle]
  fn lremovexattr(__path: *const libc::c_char, __name: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn removexattr(__path: *const libc::c_char, __name: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn lsetxattr(
    __path: *const libc::c_char,
    __name: *const libc::c_char,
    __value: *const libc::c_void,
    __size: size_t,
    __flags: libc::c_int,
  ) -> libc::c_int;
  #[no_mangle]
  fn setxattr(
    __path: *const libc::c_char,
    __name: *const libc::c_char,
    __value: *const libc::c_void,
    __size: size_t,
    __flags: libc::c_int,
  ) -> libc::c_int;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;
  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);
}

use crate::librb::size_t;
use libc::uint32_t;
pub const OPT_h: C2RustUnnamed = 1;
pub const OPT_x: C2RustUnnamed = 2;
pub type C2RustUnnamed = libc::c_uint;
/*
 * setfattr - set extended attributes of filesystem objects.
 *
 * Copyright (C) 2017 by Denys Vlasenko <vda.linux@googlemail.com>
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
//config:config SETFATTR
//config:	bool "setfattr (3.7 kb)"
//config:	default y
//config:	help
//config:	Set/delete extended attributes on files
//applet:IF_SETFATTR(APPLET_NOEXEC(setfattr, setfattr, BB_DIR_USR_BIN, BB_SUID_DROP, setfattr))
//kbuild:lib-$(CONFIG_SETFATTR) += setfattr.o
//usage:#define setfattr_trivial_usage
//usage:       "[-h] -n|-x ATTR [-v VALUE] FILE..."
//usage:#define setfattr_full_usage "\n\n"
//usage:       "Set extended attributes"
//usage:     "\n"
//usage:     "\n	-h		Do not follow symlinks"
//usage:     "\n	-x ATTR		Remove attribute ATTR"
//usage:     "\n	-n ATTR		Set attribute ATTR to VALUE"
//usage:     "\n	-v VALUE	(default: empty)"
#[no_mangle]
pub unsafe extern "C" fn setfattr_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut name: *const libc::c_char = 0 as *const libc::c_char;
  let mut value: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
  let mut status: libc::c_int = 0;
  let mut opt: libc::c_int = 0;
  opt = getopt32(
    argv,
    b"^hx:n:v:\x00-1:x:n:n--x:x--nv:v--x\x00" as *const u8 as *const libc::c_char,
    &mut name as *mut *const libc::c_char,
    &mut name as *mut *const libc::c_char,
    &mut value as *mut *const libc::c_char,
  ) as libc::c_int;
  argv = argv.offset(optind as isize);
  status = 0i32;
  loop {
    let mut r: libc::c_int = 0;
    if opt & OPT_x as libc::c_int != 0 {
      r = if opt & OPT_h as libc::c_int != 0 {
        Some(
          lremovexattr
            as unsafe extern "C" fn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int,
        )
      } else {
        Some(
          removexattr
            as unsafe extern "C" fn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int,
        )
      }
      .expect("non-null function pointer")(*argv, name)
    } else {
      r = if opt & OPT_h as libc::c_int != 0 {
        Some(
          lsetxattr
            as unsafe extern "C" fn(
              _: *const libc::c_char,
              _: *const libc::c_char,
              _: *const libc::c_void,
              _: size_t,
              _: libc::c_int,
            ) -> libc::c_int,
        )
      } else {
        Some(
          setxattr
            as unsafe extern "C" fn(
              _: *const libc::c_char,
              _: *const libc::c_char,
              _: *const libc::c_void,
              _: size_t,
              _: libc::c_int,
            ) -> libc::c_int,
        )
      }
      .expect("non-null function pointer")(
        *argv,
        name,
        value as *const libc::c_void,
        strlen(value),
        0i32,
      )
    }
    if r != 0 {
      bb_simple_perror_msg(*argv);
      status = 1i32
    }
    argv = argv.offset(1);
    if (*argv).is_null() {
      break;
    }
  }
  return status;
}
