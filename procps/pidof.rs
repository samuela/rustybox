use crate::libbb::llist::llist_t;
use libc;
use libc::unlink;

use libc::free;
use libc::pid_t;

extern "C" {

  #[no_mangle]
  fn pidlist_reverse(pidList: *mut pid_t) -> *mut pid_t;
  #[no_mangle]
  fn find_pid_by_name(procName: *const libc::c_char) -> *mut pid_t;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn utoa(n: libc::c_uint) -> *mut libc::c_char;
  #[no_mangle]
  fn xatoull(str: *const libc::c_char) -> libc::c_ulonglong;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  fn llist_free(
    elm: *mut llist_t,
    freeit: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
  );
  #[no_mangle]
  fn llist_find_str(first: *mut llist_t, str: *const libc::c_char) -> *mut llist_t;
  #[no_mangle]
  fn getppid() -> pid_t;
}

/*
 * pidof implementation for busybox
 *
 * Copyright (C) 1999-2004 by Erik Andersen <andersen@codepoet.org>
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
//config:config PIDOF
//config:	bool "pidof (6.3 kb)"
//config:	default y
//config:	help
//config:	Pidof finds the process id's (pids) of the named programs. It prints
//config:	those id's on the standard output.
//config:
//config:config FEATURE_PIDOF_SINGLE
//config:	bool "Enable single shot (-s)"
//config:	default y
//config:	depends on PIDOF
//config:	help
//config:	Support '-s' for returning only the first pid found.
//config:
//config:config FEATURE_PIDOF_OMIT
//config:	bool "Enable omitting pids (-o PID)"
//config:	default y
//config:	depends on PIDOF
//config:	help
//config:	Support '-o PID' for omitting the given pid(s) in output.
//config:	The special pid %PPID can be used to name the parent process
//config:	of the pidof, in other words the calling shell or shell script.
//applet:IF_PIDOF(APPLET(pidof, BB_DIR_BIN, BB_SUID_DROP))
/* can't be noexec: can find _itself_ under wrong name, since after fork only,
 * /proc/PID/cmdline and comm are wrong! Can fix comm (prctl(PR_SET_NAME)),
 * but cmdline?
 */
//kbuild:lib-$(CONFIG_PIDOF) += pidof.o
//usage:#if (ENABLE_FEATURE_PIDOF_SINGLE || ENABLE_FEATURE_PIDOF_OMIT)
//usage:#define pidof_trivial_usage
//usage:       "[OPTIONS] [NAME]..."
//usage:#define USAGE_PIDOF "\n"
//usage:#else
//usage:#define pidof_trivial_usage
//usage:       "[NAME]..."
//usage:#define USAGE_PIDOF /* none */
//usage:#endif
//usage:#define pidof_full_usage "\n\n"
//usage:       "List PIDs of all processes with names that match NAMEs"
//usage:	USAGE_PIDOF
//usage:	IF_FEATURE_PIDOF_SINGLE(
//usage:     "\n	-s	Show only one PID"
//usage:	)
//usage:	IF_FEATURE_PIDOF_OMIT(
//usage:     "\n	-o PID	Omit given pid"
//usage:     "\n		Use %PPID to omit pid of pidof's parent"
//usage:	)
//usage:
//usage:#define pidof_example_usage
//usage:       "$ pidof init\n"
//usage:       "1\n"
//usage:	IF_FEATURE_PIDOF_OMIT(
//usage:       "$ pidof /bin/sh\n20351 5973 5950\n")
//usage:	IF_FEATURE_PIDOF_OMIT(
//usage:       "$ pidof /bin/sh -o %PPID\n20351 5950")
pub type C2RustUnnamed = libc::c_uint;
pub const OPT_OMIT: C2RustUnnamed = 2;
pub const OPT_SINGLE: C2RustUnnamed = 1;
pub const OPTBIT_OMIT: C2RustUnnamed = 1;
pub const OPTBIT_SINGLE: C2RustUnnamed = 0;

#[inline(always)]
unsafe extern "C" fn xatoul(mut str: *const libc::c_char) -> libc::c_ulong {
  return xatoull(str) as libc::c_ulong; /* list of pids to omit */
}
#[no_mangle]
pub unsafe extern "C" fn pidof_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut first: libc::c_uint = 1i32 as libc::c_uint;
  let mut opt: libc::c_uint = 0;
  let mut omits: *mut llist_t = 0 as *mut llist_t;
  /* do unconditional option parsing */
  opt = getopt32(
    argv,
    b"so:*\x00" as *const u8 as *const libc::c_char,
    &mut omits as *mut *mut llist_t,
  );
  /* fill omit list.  */
  let mut omits_p: *mut llist_t = omits;
  loop {
    omits_p = llist_find_str(omits_p, b"%PPID\x00" as *const u8 as *const libc::c_char);
    if omits_p.is_null() {
      break;
    }
    /* are we asked to exclude the parent's process ID?  */
    (*omits_p).data = utoa(getppid() as libc::c_uint)
  }
  /* Looks like everything is set to go.  */
  argv = argv.offset(optind as isize);
  while !(*argv).is_null() {
    let mut pidList: *mut pid_t = 0 as *mut pid_t;
    let mut pl: *mut pid_t = 0 as *mut pid_t;
    /* reverse the pidlist like GNU pidof does.  */
    pidList = pidlist_reverse(find_pid_by_name(*argv));
    let mut current_block_11: u64;
    pl = pidList;
    while *pl != 0 {
      if opt & OPT_OMIT as libc::c_int as libc::c_uint != 0 {
        let mut omits_p_0: *mut llist_t = omits;
        loop {
          if omits_p_0.is_null() {
            current_block_11 = 5601891728916014340;
            break;
          }
          if xatoul((*omits_p_0).data) == *pl as libc::c_ulong {
            current_block_11 = 10599921512955367680;
            break;
          }
          omits_p_0 = (*omits_p_0).link
        }
      } else {
        current_block_11 = 5601891728916014340;
      }
      match current_block_11 {
        5601891728916014340 => {
          printf(
            (b" %u\x00" as *const u8 as *const libc::c_char).offset(first as isize),
            *pl as libc::c_uint,
          );
          first = 0i32 as libc::c_uint;
          if 1i32 != 0 && opt & OPT_SINGLE as libc::c_int as libc::c_uint != 0 {
            break;
          }
        }
        _ => {}
      }
      pl = pl.offset(1)
    }
    free(pidList as *mut libc::c_void);
    argv = argv.offset(1)
  }
  if first == 0 {
    bb_putchar('\n' as i32);
  }
  return first as libc::c_int;
  /* 1 (failure) - no processes found */
}
