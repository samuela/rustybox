use libc;

extern "C" {
  #[no_mangle]
  static mut applet_name: *const libc::c_char;

  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  fn is_prefixed_with(string: *const libc::c_char, key: *const libc::c_char) -> *mut libc::c_char;

  #[no_mangle]
  fn BB_EXECVP_or_die(argv: *mut *mut libc::c_char) -> !;

  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;

  #[no_mangle]
  fn bb_show_usage() -> !;

  #[no_mangle]
  fn bb_perror_msg_and_die(s: *const libc::c_char, _: ...) -> !;

  #[no_mangle]
  fn personality(__persona: libc::c_ulong) -> libc::c_int;
}

use crate::librb::uint32_t;

pub type C2RustUnnamed = libc::c_uint;
// pub const PER_MASK: C2RustUnnamed = 255;
// pub const PER_HPUX: C2RustUnnamed = 16;
// pub const PER_OSF4: C2RustUnnamed = 15;
// pub const PER_UW7: C2RustUnnamed = 68157454;
// pub const PER_SOLARIS: C2RustUnnamed = 67108877;
// pub const PER_RISCOS: C2RustUnnamed = 12;
// pub const PER_IRIX64: C2RustUnnamed = 67108875;
// pub const PER_IRIXN32: C2RustUnnamed = 67108874;
// pub const PER_IRIX32: C2RustUnnamed = 67108873;
// pub const PER_LINUX32_3GB: C2RustUnnamed = 134217736;
pub const PER_LINUX32: C2RustUnnamed = 8;
// pub const PER_XENIX: C2RustUnnamed = 83886087;
// pub const PER_SUNOS: C2RustUnnamed = 67108870;
// pub const PER_BSD: C2RustUnnamed = 6;
// pub const PER_ISCR4: C2RustUnnamed = 67108869;
// pub const PER_WYSEV386: C2RustUnnamed = 83886084;
// pub const PER_OSR5: C2RustUnnamed = 100663299;
// pub const PER_SCOSVR3: C2RustUnnamed = 117440515;
// pub const PER_SVR3: C2RustUnnamed = 83886082;
// pub const PER_SVR4: C2RustUnnamed = 68157441;
// pub const PER_LINUX_FDPIC: C2RustUnnamed = 524288;
// pub const PER_LINUX_32BIT: C2RustUnnamed = 8388608;
pub const PER_LINUX: C2RustUnnamed = 0;

#[no_mangle]
pub unsafe extern "C" fn setarch_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut opts: libc::c_uint = 0;
  let mut pers: libc::c_ulong = 0;
  /* Figure out what personality we are supposed to switch to ...
   * we can be invoked as either:
   * argv[0],argv[1] == "setarch","personality"
   * argv[0]         == "personality"
   */
  if 1i32 != 0
    && *applet_name.offset(0) as libc::c_int == 's' as i32
    && !(*argv.offset(1)).is_null()
    && !is_prefixed_with(
      *argv.offset(1),
      b"linux\x00" as *const u8 as *const libc::c_char,
    )
    .is_null()
  {
    argv = argv.offset(1);
    applet_name = *argv.offset(0)
  }
  if 1i32 == 0 && 1i32 == 0 || *applet_name.offset(5) as libc::c_int == '6' as i32 {
    /* linux64 */
    pers = PER_LINUX as libc::c_int as libc::c_ulong
  } else if 1i32 == 0 && 1i32 == 0 || *applet_name.offset(5) as libc::c_int == '3' as i32 {
    /* linux32 */
    pers = PER_LINUX32 as libc::c_int as libc::c_ulong
  } else {
    bb_show_usage(); /* '+': stop at first non-option */
  }
  opts = getopt32(argv, b"+R\x00" as *const u8 as *const libc::c_char);
  if opts != 0 {
    pers |= 0x40000i32 as libc::c_ulong
  }
  /* Try to set personality */
  if personality(pers) < 0i32 {
    bb_perror_msg_and_die(
      b"personality(0x%lx)\x00" as *const u8 as *const libc::c_char,
      pers,
    );
  }
  argv = argv.offset(optind as isize);
  if (*argv.offset(0)).is_null() {
    argv = argv.offset(-1);
    let ref mut fresh0 = *argv.offset(0);
    *fresh0 = b"/bin/sh\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
  }
  /* Try to execute the program */
  BB_EXECVP_or_die(argv);
}
