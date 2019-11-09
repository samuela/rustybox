use libc;



extern "C" {
  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  fn syscall(__sysno: libc::c_long, _: ...) -> libc::c_long;

  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;

  #[no_mangle]
  fn xatoi_positive(numstr: *const libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn BB_EXECVP_or_die(argv: *mut *mut libc::c_char) -> !;

  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;

  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;

  #[no_mangle]
  fn bb_perror_msg_and_die(s: *const libc::c_char, _: ...) -> !;

  #[no_mangle]
  fn nth_string(strings: *const libc::c_char, n: libc::c_int) -> *const libc::c_char;
}



pub type C2RustUnnamed = libc::c_uint;
// pub const IOPRIO_WHO_USER: C2RustUnnamed = 3;
// pub const IOPRIO_WHO_PGRP: C2RustUnnamed = 2;
pub const IOPRIO_WHO_PROCESS: C2RustUnnamed = 1;

pub type C2RustUnnamed_0 = libc::c_uint;
pub const IOPRIO_CLASS_IDLE: C2RustUnnamed_0 = 3;
// pub const IOPRIO_CLASS_BE: C2RustUnnamed_0 = 2;
// pub const IOPRIO_CLASS_RT: C2RustUnnamed_0 = 1;
// pub const IOPRIO_CLASS_NONE: C2RustUnnamed_0 = 0;

pub type C2RustUnnamed_1 = libc::c_uint;
pub const OPT_p: C2RustUnnamed_1 = 4;
pub const OPT_c: C2RustUnnamed_1 = 2;
pub const OPT_n: C2RustUnnamed_1 = 1;

/*
 * ionice implementation for busybox based on linux-utils-ng 2.14
 *
 * Copyright (C) 2008 by  <u173034@informatik.uni-oldenburg.de>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config IONICE
//config:	bool "ionice (3.8 kb)"
//config:	default y
//config:	select PLATFORM_LINUX
//config:	help
//config:	Set/set program io scheduling class and priority
//config:	Requires kernel >= 2.6.13
//applet:IF_IONICE(APPLET_NOEXEC(ionice, ionice, BB_DIR_BIN, BB_SUID_DROP, ionice))
//kbuild:lib-$(CONFIG_IONICE) += ionice.o
//usage:#define ionice_trivial_usage
//usage:	"[-c 1-3] [-n 0-7] [-p PID] [PROG]"
//usage:#define ionice_full_usage "\n\n"
//usage:       "Change I/O priority and class\n"
//usage:     "\n	-c	Class. 1:realtime 2:best-effort 3:idle"
//usage:     "\n	-n	Priority"
unsafe extern "C" fn ioprio_set(
  mut which: libc::c_int,
  mut who: libc::c_int,
  mut ioprio: libc::c_int,
) -> libc::c_int {
  return syscall(251i32 as libc::c_long, which, who, ioprio) as libc::c_int;
}
unsafe extern "C" fn ioprio_get(mut which: libc::c_int, mut who: libc::c_int) -> libc::c_int {
  return syscall(252i32 as libc::c_long, which, who) as libc::c_int;
}
static mut to_prio: [libc::c_char; 31] = [
  110, 111, 110, 101, 0, 114, 101, 97, 108, 116, 105, 109, 101, 0, 98, 101, 115, 116, 45, 101, 102,
  102, 111, 114, 116, 0, 105, 100, 108, 101, 0,
];

#[no_mangle]
pub unsafe extern "C" fn ionice_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  /* Defaults */
  let mut ioclass: libc::c_int = 0i32; /* affect own porcess */
  let mut pri: libc::c_int = 0i32;
  let mut pid: libc::c_int = 0i32;
  let mut opt: libc::c_int = 0;
  /* Numeric params */
  /* '+': stop at first non-option */
  opt = getopt32(
    argv,
    b"+n:+c:+p:+\x00" as *const u8 as *const libc::c_char,
    &mut pri as *mut libc::c_int,
    &mut ioclass as *mut libc::c_int,
    &mut pid as *mut libc::c_int,
  ) as libc::c_int;
  argv = argv.offset(optind as isize);
  if opt & OPT_c as libc::c_int != 0 {
    if ioclass > 3i32 {
      bb_error_msg_and_die(
        b"bad class %d\x00" as *const u8 as *const libc::c_char,
        ioclass,
      );
    }
    // Do we need this (compat?)?
    //		if (ioclass == IOPRIO_CLASS_NONE)
    //			ioclass = IOPRIO_CLASS_BE;
    //		if (ioclass == IOPRIO_CLASS_IDLE) {
    //			//if (opt & OPT_n)
    //			//	bb_error_msg("ignoring priority for idle class");
    //			pri = 7;
    //		}
  }
  if opt & (OPT_n as libc::c_int | OPT_c as libc::c_int) == 0 {
    if opt & OPT_p as libc::c_int == 0 && !(*argv).is_null() {
      pid = xatoi_positive(*argv)
    }
    pri = ioprio_get(IOPRIO_WHO_PROCESS as libc::c_int, pid);
    if pri == -1i32 {
      bb_perror_msg_and_die(
        b"ioprio_%cet\x00" as *const u8 as *const libc::c_char,
        'g' as i32,
      );
    }
    ioclass = pri >> 13i32 & 0x3i32;
    pri &= 0xffi32;
    printf(
      if ioclass == IOPRIO_CLASS_IDLE as libc::c_int {
        b"%s\n\x00" as *const u8 as *const libc::c_char
      } else {
        b"%s: prio %d\n\x00" as *const u8 as *const libc::c_char
      },
      nth_string(to_prio.as_ptr(), ioclass),
      pri,
    );
  } else {
    //printf("pri=%d class=%d val=%x\n",
    //pri, ioclass, pri | (ioclass << IOPRIO_CLASS_SHIFT));
    pri |= ioclass << 13i32;
    if ioprio_set(IOPRIO_WHO_PROCESS as libc::c_int, pid, pri) == -1i32 {
      bb_perror_msg_and_die(
        b"ioprio_%cet\x00" as *const u8 as *const libc::c_char,
        's' as i32,
      );
    }
    if !(*argv.offset(0)).is_null() {
      BB_EXECVP_or_die(argv);
    }
  }
  return 0i32;
}
