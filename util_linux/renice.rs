use crate::libbb::ptr_to_globals::bb_errno;
use crate::libpwdgrp::pwd_grp::bb_internal_getpwnam;
use libc;
use libc::passwd;
use libc::strchr;
extern "C" {

  #[no_mangle]
  fn getpriority(__which: __priority_which_t, __who: id_t) -> libc::c_int;
  #[no_mangle]
  fn setpriority(__which: __priority_which_t, __who: id_t, __prio: libc::c_int) -> libc::c_int;
/* Search for an entry with a matching username.  */

}

pub type __id_t = libc::c_uint;
pub type id_t = __id_t;
pub type __priority_which = libc::c_uint;
pub const PRIO_USER: __priority_which = 2;
pub const PRIO_PGRP: __priority_which = 1;
pub const PRIO_PROCESS: __priority_which = 0;
pub type __priority_which_t = __priority_which;

/*
 * renice implementation for busybox
 *
 * Copyright (C) 2005  Manuel Novoa III  <mjn3@codepoet.org>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/* Notes:
 *   Setting an absolute priority was obsoleted in SUSv2 and removed
 *   in SUSv3.  However, the common linux version of renice does
 *   absolute and not relative.  So we'll continue supporting absolute,
 *   although the stdout logging has been removed since both SUSv2 and
 *   SUSv3 specify that stdout isn't used.
 *
 *   This version is lenient in that it doesn't require any IDs.  The
 *   options -p, -g, and -u are treated as mode switches for the
 *   following IDs (if any).  Multiple switches are allowed.
 */
//config:config RENICE
//config:	bool "renice (4.2 kb)"
//config:	default y
//config:	help
//config:	Renice alters the scheduling priority of one or more running
//config:	processes.
//applet:IF_RENICE(APPLET_NOEXEC(renice, renice, BB_DIR_USR_BIN, SUID_DROP, renice))
//kbuild:lib-$(CONFIG_RENICE) += renice.o
//usage:#define renice_trivial_usage
//usage:       "[-n] PRIORITY [[-p | -g | -u] ID...]..."
//usage:#define renice_full_usage "\n\n"
//usage:       "Change scheduling priority of a running process\n"
//usage:     "\n	-n	Add PRIORITY to current nice value"
//usage:     "\n		Without -n, nice value is set to PRIORITY"
//usage:     "\n	-p	Process ids (default)"
//usage:     "\n	-g	Process group ids"
//usage:     "\n	-u	Process user names"
#[no_mangle]
pub unsafe extern "C" fn renice_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut current_block: u64; /* Default 'which' value. */
  static mut Xetpriority_msg: [libc::c_char; 13] =
    [37, 99, 101, 116, 112, 114, 105, 111, 114, 105, 116, 121, 0];
  let mut retval: libc::c_int = 0i32;
  let mut which: libc::c_int = PRIO_PROCESS as libc::c_int;
  let mut use_relative: libc::c_int = 0i32;
  let mut adjustment: libc::c_int = 0;
  let mut new_priority: libc::c_int = 0;
  let mut who: libc::c_uint = 0;
  let mut arg: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  /* Yes, they are not #defines in glibc 2.4! #if won't work */
  argv = argv.offset(1);
  arg = *argv;
  /* Check if we are using a relative adjustment. */
  if !arg.is_null()
    && *arg.offset(0) as libc::c_int == '-' as i32
    && *arg.offset(1) as libc::c_int == 'n' as i32
  {
    use_relative = 1i32;
    if *arg.offset(2) == 0 {
      argv = argv.offset(1);
      arg = *argv
    } else {
      arg = arg.offset(2)
    }
  }
  if arg.is_null() {
    /* No args?  Then show usage. */
    crate::libbb::appletlib::bb_show_usage();
  }
  /* Get the priority adjustment (absolute or relative). */
  adjustment =
    crate::libbb::xatonum::xatoi_range(arg, (-2147483647i32 - 1i32) / 2i32, 2147483647i32 / 2i32);
  loop {
    argv = argv.offset(1);
    arg = *argv;
    if arg.is_null() {
      break;
    }
    /* Check for a mode switch. */
    if *arg.offset(0) as libc::c_int == '-' as i32 && *arg.offset(1) as libc::c_int != 0 {
      static mut opts: [libc::c_char; 7] = [
        'p' as i32 as libc::c_char,
        'g' as i32 as libc::c_char,
        'u' as i32 as libc::c_char,
        0i32 as libc::c_char,
        PRIO_PROCESS as libc::c_int as libc::c_char,
        PRIO_PGRP as libc::c_int as libc::c_char,
        PRIO_USER as libc::c_int as libc::c_char,
      ];
      let mut p: *const libc::c_char = strchr(opts.as_ptr(), *arg.offset(1) as libc::c_int);
      if !p.is_null() {
        which = *p.offset(4) as libc::c_int;
        if *arg.offset(2) == 0 {
          continue;
        }
        arg = arg.offset(2)
      }
    }
    /* Process an ID arg. */
    if which == PRIO_USER as libc::c_int {
      let mut p_0: *mut passwd = 0 as *mut passwd;
      /* NB: use of getpwnam makes it risky to be NOFORK, switch to getpwnam_r? */
      p_0 = bb_internal_getpwnam(arg);
      if p_0.is_null() {
        crate::libbb::verror_msg::bb_error_msg(
          b"unknown user %s\x00" as *const u8 as *const libc::c_char,
          arg,
        );
        current_block = 46536466264865791;
      } else {
        who = (*p_0).pw_uid;
        current_block = 11385396242402735691;
      }
    } else {
      who = crate::libbb::bb_strtonum::bb_strtou(arg, 0 as *mut *mut libc::c_char, 10i32);
      if *bb_errno != 0 {
        crate::libbb::verror_msg::bb_error_msg(
          b"invalid number \'%s\'\x00" as *const u8 as *const libc::c_char,
          arg,
        );
        current_block = 46536466264865791;
      } else {
        current_block = 11385396242402735691;
      }
    }
    match current_block {
      11385396242402735691 =>
      /* Get priority to use, and set it. */
      {
        if use_relative != 0 {
          let mut old_priority: libc::c_int = 0; /* Needed for getpriority error detection. */
          *bb_errno = 0i32;
          old_priority = getpriority(which as __priority_which_t, who);
          if *bb_errno != 0 {
            crate::libbb::perror_msg::bb_perror_msg(Xetpriority_msg.as_ptr(), 'g' as i32);
            current_block = 46536466264865791;
          } else {
            new_priority = old_priority + adjustment;
            current_block = 790185930182612747;
          }
        } else {
          new_priority = adjustment;
          current_block = 790185930182612747;
        }
        match current_block {
          46536466264865791 => {}
          _ => {
            if setpriority(which as __priority_which_t, who, new_priority) == 0i32 {
              continue;
            }
            crate::libbb::perror_msg::bb_perror_msg(Xetpriority_msg.as_ptr(), 's' as i32);
          }
        }
      }
      _ => {}
    }
    retval = 1i32
  }
  /* No need to check for errors outputting to stderr since, if it
   * was used, the HAD_ERROR label was reached and retval was set. */
  return retval;
}
