use libc;
use libc::pid_t;
use libc::printf;
extern "C" {
  #[no_mangle]
  fn sched_getparam(__pid: pid_t, __param: *mut sched_param) -> libc::c_int;

  #[no_mangle]
  fn sched_setscheduler(
    __pid: pid_t,
    __policy: libc::c_int,
    __param: *const sched_param,
  ) -> libc::c_int;

  #[no_mangle]
  fn sched_getscheduler(__pid: pid_t) -> libc::c_int;

  #[no_mangle]
  fn sched_get_priority_max(__algorithm: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn sched_get_priority_min(__algorithm: libc::c_int) -> libc::c_int;

  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  fn fflush_stdout_and_exit(retval: libc::c_int) -> !;

  #[no_mangle]
  fn utoa(n: libc::c_uint) -> *mut libc::c_char;

  #[no_mangle]
  fn xatoull_range(
    str: *const libc::c_char,
    l: libc::c_ulonglong,
    u: libc::c_ulonglong,
  ) -> libc::c_ulonglong;

  #[no_mangle]
  fn xstrtou_range(
    str: *const libc::c_char,
    b: libc::c_int,
    l: libc::c_uint,
    u: libc::c_uint,
  ) -> libc::c_uint;

  #[no_mangle]
  fn BB_EXECVP_or_die(argv: *mut *mut libc::c_char) -> !;

  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;

  #[no_mangle]
  fn bb_show_usage() -> !;

  #[no_mangle]
  fn bb_perror_msg_and_die(s: *const libc::c_char, _: ...) -> !;

  #[no_mangle]
  fn nth_string(strings: *const libc::c_char, n: libc::c_int) -> *const libc::c_char;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sched_param {
  pub sched_priority: libc::c_int,
}

#[inline(always)]
unsafe extern "C" fn xatoul_range(
  mut str: *const libc::c_char,
  mut l: libc::c_ulong,
  mut u: libc::c_ulong,
) -> libc::c_ulong {
  return xatoull_range(str, l as libc::c_ulonglong, u as libc::c_ulonglong) as libc::c_ulong;
}

/*
 * chrt - manipulate real-time attributes of a process
 * Copyright (c) 2006-2007 Bernhard Reutner-Fischer
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config CHRT
//config:	bool "chrt (4.7 kb)"
//config:	default y
//config:	help
//config:	Manipulate real-time attributes of a process.
//config:	This requires sched_{g,s}etparam support in your libc.
//applet:IF_CHRT(APPLET_NOEXEC(chrt, chrt, BB_DIR_USR_BIN, SUID_DROP, chrt))
//kbuild:lib-$(CONFIG_CHRT) += chrt.o
//usage:#define chrt_trivial_usage
//usage:       "-m | -p [PRIO] PID | [-rfobi] PRIO PROG [ARGS]"
//usage:#define chrt_full_usage "\n\n"
//usage:       "Change scheduling priority and class for a process\n"
//usage:     "\n	-m	Show min/max priorities"
//usage:     "\n	-p	Operate on PID"
//usage:     "\n	-r	Set SCHED_RR class"
//usage:     "\n	-f	Set SCHED_FIFO class"
//usage:     "\n	-o	Set SCHED_OTHER class"
//usage:     "\n	-b	Set SCHED_BATCH class"
//usage:     "\n	-i	Set SCHED_IDLE class"
//usage:
//usage:#define chrt_example_usage
//usage:       "$ chrt -r 4 sleep 900; x=$!\n"
//usage:       "$ chrt -f -p 3 $x\n"
//usage:       "You need CAP_SYS_NICE privileges to set scheduling attributes of a process"
unsafe extern "C" fn policy_name(mut pol: libc::c_int) -> *const libc::c_char {
  if pol > 6i32 {
    return utoa(pol as libc::c_uint);
  } /* for compiler */
  return nth_string(
    b"OTHER\x00FIFO\x00RR\x00BATCH\x00ISO\x00IDLE\x00DEADLINE\x00" as *const u8
      as *const libc::c_char,
    pol,
  );
}
unsafe extern "C" fn show_min_max(mut pol: libc::c_int) {
  let mut fmt: *const libc::c_char =
    b"SCHED_%s min/max priority\t: %u/%u\n\x00" as *const u8 as *const libc::c_char;
  let mut max: libc::c_int = 0;
  let mut min: libc::c_int = 0;
  max = sched_get_priority_max(pol);
  min = sched_get_priority_min(pol);
  if max | min < 0i32 {
    fmt = b"SCHED_%s not supported\n\x00" as *const u8 as *const libc::c_char
  }
  printf(fmt, policy_name(pol), min, max);
}
#[no_mangle]
pub unsafe extern "C" fn chrt_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut pol: libc::c_int = 0;
  let mut current_block: u64;
  let mut pid: pid_t = 0i32;
  let mut opt: libc::c_uint = 0;
  let mut sp: sched_param = sched_param { sched_priority: 0 };
  let mut pid_str: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut priority: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  priority = priority;
  let mut current_new: *const libc::c_char = 0 as *const libc::c_char;
  let mut policy: libc::c_int = 2i32;
  opt = getopt32(
    argv,
    b"^+mprfobi\x00r--fobi:f--robi:o--rfbi:b--rfoi:i--rfob\x00" as *const u8 as *const libc::c_char,
  );
  if opt & (1i32 << 0i32) as libc::c_uint != 0 {
    /* print min/max and exit */
    show_min_max(0i32);
    show_min_max(1i32);
    show_min_max(2i32);
    show_min_max(3i32);
    show_min_max(5i32);
    fflush_stdout_and_exit(0i32);
  }
  //if (opt & OPT_r)
  //	policy = SCHED_RR; - default, already set
  if opt & (1i32 << 3i32) as libc::c_uint != 0 {
    policy = 1i32
  }
  if opt & (1i32 << 4i32) as libc::c_uint != 0 {
    policy = 0i32
  }
  if opt & (1i32 << 5i32) as libc::c_uint != 0 {
    policy = 3i32
  }
  if opt & (1i32 << 6i32) as libc::c_uint != 0 {
    policy = 5i32
  }
  argv = argv.offset(optind as isize);
  if (*argv.offset(0)).is_null() {
    bb_show_usage();
  }
  if opt & (1i32 << 1i32) as libc::c_uint != 0 {
    let fresh0 = argv;
    argv = argv.offset(1);
    pid_str = *fresh0;
    if !(*argv).is_null() {
      /* "-p PRIO PID [...]" */
      priority = pid_str;
      pid_str = *argv
    }
    /* else "-p PID", and *argv == NULL */
    pid = xatoul_range(
      pid_str,
      1i32 as libc::c_ulong,
      ((9223372036854775807i64 as libc::c_ulong)
        .wrapping_mul(2u64)
        .wrapping_add(1u64) as pid_t as libc::c_uint
        >> 1i32) as libc::c_ulong,
    ) as pid_t
  } else {
    let fresh1 = argv;
    argv = argv.offset(1);
    priority = *fresh1;
    if (*argv).is_null() {
      bb_show_usage();
    }
  }
  current_new = b"current\x00new\x00" as *const u8 as *const libc::c_char;
  if opt & (1i32 << 1i32) as libc::c_uint != 0 {
    pol = 0;
    current_block = 6590038594373520143;
  } else {
    current_block = 14447253356787937536;
  }
  loop {
    match current_block {
      14447253356787937536 => {
        sp.sched_priority = xstrtou_range(
          priority,
          0i32,
          sched_get_priority_min(policy) as libc::c_uint,
          sched_get_priority_max(policy) as libc::c_uint,
        ) as libc::c_int;
        if sched_setscheduler(pid, policy, &mut sp) < 0i32 {
          bb_perror_msg_and_die(
            b"can\'t %cet pid %u\'s policy\x00" as *const u8 as *const libc::c_char,
            's' as i32,
            pid,
          );
        }
        if (*argv.offset(0)).is_null() {
          current_block = 6590038594373520143;
        } else {
          break;
        }
      }
      _ =>
      /* "-p PRIO PID [...]" */
      {
        pol = sched_getscheduler(pid);
        if pol < 0i32 {
          bb_perror_msg_and_die(
            b"can\'t %cet pid %u\'s policy\x00" as *const u8 as *const libc::c_char,
            'g' as i32,
            pid,
          );
        }
        /* "Since Linux 2.6.32, the SCHED_RESET_ON_FORK flag
         * can be ORed in policy when calling sched_setscheduler().
         * As a result of including this flag, children created by
         * fork(2) do not inherit privileged scheduling policies"
         *
         * This bit is also returned by sched_getscheduler()!
         * (TODO: do we want to show it?)
         */
        pol &= !0x40000000i32;
        printf(
          b"pid %u\'s %s scheduling policy: SCHED_%s\n\x00" as *const u8 as *const libc::c_char,
          pid,
          current_new,
          policy_name(pol),
        );
        if sched_getparam(pid, &mut sp) != 0 {
          bb_perror_msg_and_die(
            b"can\'t get pid %u\'s attributes\x00" as *const u8 as *const libc::c_char,
            pid,
          );
        }
        printf(
          b"pid %u\'s %s scheduling priority: %d\n\x00" as *const u8 as *const libc::c_char,
          pid,
          current_new,
          sp.sched_priority,
        );
        if (*argv).is_null() {
          /* Either it was just "-p PID",
           * or it was "-p PRIO PID" and we came here
           * for the second time (see goto below) */
          return 0i32;
        }
        *argv = std::ptr::null_mut::<libc::c_char>();
        current_new = current_new.offset(8);
        current_block = 14447253356787937536;
      }
    }
  }
  BB_EXECVP_or_die(argv);
}
