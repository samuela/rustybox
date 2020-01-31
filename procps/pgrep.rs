use crate::libbb::appletlib::applet_name;
use crate::librb::procps_status_t;
use crate::librb::re_pattern_buffer;
use crate::librb::size_t;
use libc;
use libc::free;
use libc::getpid;
use libc::kill;
use libc::pid_t;
use libc::printf;
extern "C" {

  #[no_mangle]
  fn getsid(__pid: pid_t) -> pid_t;
  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;

  #[no_mangle]
  static mut option_mask32: u32;

  #[no_mangle]
  fn regexec(
    __preg: *const regex_t,
    __string: *const libc::c_char,
    __nmatch: size_t,
    __pmatch: *mut regmatch_t,
    __eflags: libc::c_int,
  ) -> libc::c_int;

}

pub type C2RustUnnamed = libc::c_uint;
pub const PSSCAN_TASKS: C2RustUnnamed = 4194304;
pub const PSSCAN_RUIDGID: C2RustUnnamed = 2097152;
pub const PSSCAN_NICE: C2RustUnnamed = 1048576;
pub const PSSCAN_CPU: C2RustUnnamed = 524288;
pub const PSSCAN_START_TIME: C2RustUnnamed = 262144;
pub const PSSCAN_CONTEXT: C2RustUnnamed = 0;
pub const PSSCAN_ARGVN: C2RustUnnamed = 65536;
pub const PSSCAN_SMAPS: C2RustUnnamed = 32768;
pub const PSSCAN_TTY: C2RustUnnamed = 16384;
pub const PSSCAN_UTIME: C2RustUnnamed = 8192;
pub const PSSCAN_STIME: C2RustUnnamed = 4096;
pub const PSSCAN_RSS: C2RustUnnamed = 2048;
pub const PSSCAN_VSZ: C2RustUnnamed = 1024;
pub const PSSCAN_STATE: C2RustUnnamed = 512;
pub const PSSCAN_EXE: C2RustUnnamed = 256;
pub const PSSCAN_ARGV0: C2RustUnnamed = 128;
pub const PSSCAN_COMM: C2RustUnnamed = 32;
pub const PSSCAN_UIDGID: C2RustUnnamed = 16;
pub const PSSCAN_SID: C2RustUnnamed = 8;
pub const PSSCAN_PGID: C2RustUnnamed = 4;
pub const PSSCAN_PPID: C2RustUnnamed = 2;
pub const PSSCAN_PID: C2RustUnnamed = 1;

pub type regex_t = re_pattern_buffer;
pub type regoff_t = libc::c_int;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct regmatch_t {
  pub rm_so: regoff_t,
  pub rm_eo: regoff_t,
}

/*
 * Mini pgrep/pkill implementation for busybox
 *
 * Copyright (C) 2007 Loic Grenie <loic.grenie@gmail.com>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config PGREP
//config:	bool "pgrep (6.5 kb)"
//config:	default y
//config:	help
//config:	Look for processes by name.
//config:
//config:config PKILL
//config:	bool "pkill (7.5 kb)"
//config:	default y
//config:	help
//config:	Send signals to processes by name.
//applet:IF_PGREP(APPLET_ODDNAME(pgrep, pgrep, BB_DIR_USR_BIN, SUID_DROP, pgrep))
//                APPLET_ODDNAME:name   main   location        suid_type     help
//applet:IF_PKILL(APPLET_ODDNAME(pkill, pgrep, BB_DIR_USR_BIN, SUID_DROP, pkill))
/* can't be noexec: can find _itself_ under wrong name, since after fork only,
 * /proc/PID/cmdline and comm are wrong! Can fix comm (prctl(PR_SET_NAME)),
 * but cmdline?
 */
//kbuild:lib-$(CONFIG_PGREP) += pgrep.o
//kbuild:lib-$(CONFIG_PKILL) += pgrep.o
//usage:#define pgrep_trivial_usage
//usage:       "[-flanovx] [-s SID|-P PPID|PATTERN]"
//usage:#define pgrep_full_usage "\n\n"
//usage:       "Display process(es) selected by regex PATTERN\n"
//usage:     "\n	-l	Show command name too"
//usage:     "\n	-a	Show command line too"
//usage:     "\n	-f	Match against entire command line"
//usage:     "\n	-n	Show the newest process only"
//usage:     "\n	-o	Show the oldest process only"
//usage:     "\n	-v	Negate the match"
//usage:     "\n	-x	Match whole name (not substring)"
//usage:     "\n	-s	Match session ID (0 for current)"
//usage:     "\n	-P	Match parent process ID"
//usage:
//usage:#define pkill_trivial_usage
//usage:       "[-l|-SIGNAL] [-fnovx] [-s SID|-P PPID|PATTERN]"
//usage:#define pkill_full_usage "\n\n"
//usage:       "Send a signal to process(es) selected by regex PATTERN\n"
//usage:     "\n	-l	List all signals"
//usage:     "\n	-f	Match against entire command line"
//usage:     "\n	-n	Signal the newest process only"
//usage:     "\n	-o	Signal the oldest process only"
//usage:     "\n	-v	Negate the match"
//usage:     "\n	-x	Match whole name (not substring)"
//usage:     "\n	-s	Match session ID (0 for current)"
//usage:     "\n	-P	Match parent process ID"
/* Idea taken from kill.c */
pub type C2RustUnnamed_0 = libc::c_uint;
pub const OPTBIT_P: C2RustUnnamed_0 = 8;
pub const OPTBIT_S: C2RustUnnamed_0 = 7;
pub const OPTBIT_N: C2RustUnnamed_0 = 6;
pub const OPTBIT_O: C2RustUnnamed_0 = 5;
pub const OPTBIT_X: C2RustUnnamed_0 = 4;
pub const OPTBIT_F: C2RustUnnamed_0 = 3;
pub const OPTBIT_A: C2RustUnnamed_0 = 2;
/* must be first, we need OPT_INVERT = 0/1 */
pub const OPTBIT_L: C2RustUnnamed_0 = 1;
/* "vlafxons:+P:+" */
pub const OPTBIT_V: C2RustUnnamed_0 = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_1 {
  pub re_buffer: regex_t,
  pub re_match: [regmatch_t; 1],
}
unsafe extern "C" fn act(
  mut pid: libc::c_uint,
  mut cmd: *mut libc::c_char,
  mut signo: libc::c_int,
) {
  if 1i32 != 0 && (1i32 == 0 || *applet_name.offset(1) as libc::c_int == 'g' as i32) {
    if option_mask32
      & (1i32 << OPTBIT_L as libc::c_int | 1i32 << OPTBIT_A as libc::c_int) as libc::c_uint
      != 0
    {
      /* -l or -a */
      printf(b"%u %s\n\x00" as *const u8 as *const libc::c_char, pid, cmd);
    } else {
      printf(b"%u\n\x00" as *const u8 as *const libc::c_char, pid);
    }
  } else {
    kill(pid as pid_t, signo);
  };
}
pub unsafe fn pgrep_main(mut _argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
  let mut current_block: u64;
  let mut pid: libc::c_uint = 0;
  let mut signo: libc::c_int = 0;
  let mut opt: libc::c_uint = 0;
  let mut scan_mask: libc::c_int = 0;
  let mut matched_pid: libc::c_int = 0;
  let mut sid2match: libc::c_int = 0;
  let mut ppid2match: libc::c_int = 0;
  let mut cmd_last: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut proc_0: *mut procps_status_t = std::ptr::null_mut();
  /* These are initialized to 0 */
  let mut Z: C2RustUnnamed_1 = std::mem::zeroed();
  memset(
    &mut Z as *mut C2RustUnnamed_1 as *mut libc::c_void,
    0,
    ::std::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong,
  );
  /* Parse -SIGNAL for pkill. Must be first option, if present */
  signo = 15i32;
  if 1i32 != 0
    && (1i32 == 0 || *applet_name.offset(1) as libc::c_int == 'k' as i32)
    && !(*argv.offset(1)).is_null()
    && *(*argv.offset(1)).offset(0) as libc::c_int == '-' as i32
  {
    let mut temp: libc::c_int =
      crate::libbb::u_signal_names::get_signum((*argv.offset(1)).offset(1));
    if temp != -1i32 {
      signo = temp;
      argv = argv.offset(1)
    }
  }
  /* Parse remaining options */
  ppid2match = -1i32;
  sid2match = -1i32;
  opt = crate::libbb::getopt32::getopt32(
    argv,
    b"vlafxons:+P:+\x00" as *const u8 as *const libc::c_char,
    &mut sid2match as *mut libc::c_int,
    &mut ppid2match as *mut libc::c_int,
  );
  argv = argv.offset(optind as isize);
  if 1i32 != 0
    && (1i32 == 0 || *applet_name.offset(1) as libc::c_int == 'k' as i32)
    && opt & (1i32 << OPTBIT_L as libc::c_int) as libc::c_uint != 0
  {
    /* -l: print the whole signal list */
    crate::libbb::u_signal_names::print_signames();
    return 0;
  }
  pid = getpid() as libc::c_uint;
  if sid2match == 0 {
    sid2match = getsid(pid as pid_t)
  }
  scan_mask = PSSCAN_COMM as libc::c_int | PSSCAN_ARGV0 as libc::c_int;
  if opt & (1i32 << OPTBIT_F as libc::c_int) as libc::c_uint != 0 {
    scan_mask |= PSSCAN_ARGVN as libc::c_int
  }
  /* One pattern is required, if no -s and no -P */
  if sid2match & ppid2match < 0 && ((*argv.offset(0)).is_null() || !(*argv.offset(1)).is_null()) {
    crate::libbb::appletlib::bb_show_usage();
  }
  if !(*argv.offset(0)).is_null() {
    crate::libbb::xregcomp::xregcomp(
      &mut Z.re_buffer,
      *argv.offset(0),
      if opt & (1i32 << OPTBIT_X as libc::c_int) as libc::c_uint != 0 {
        1i32
      } else {
        (1i32) | 1i32 << 1i32 << 1i32 << 1i32
      },
    );
  }
  matched_pid = 0;
  cmd_last = std::ptr::null_mut::<libc::c_char>();
  proc_0 = std::ptr::null_mut();
  loop {
    proc_0 = crate::libbb::procps::procps_scan(proc_0, scan_mask);
    if proc_0.is_null() {
      break;
    }
    let mut cmd: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut cmdlen: libc::c_int = 0;
    let mut match_0: libc::c_int = 0;
    if (*proc_0).pid == pid {
      continue;
    }
    if opt & (1i32 << OPTBIT_V as libc::c_int) as libc::c_uint == 0 {
      /* Quickly reject -sN -PN mismatches... unless -v */
      if ppid2match >= 0 && ppid2match as libc::c_uint != (*proc_0).ppid {
        continue; /* not -a: find first NUL */
      }
      if sid2match >= 0 && sid2match as libc::c_uint != (*proc_0).sid {
        continue;
      }
    }
    cmdlen = -1i32;
    cmd = (*proc_0).argv0;
    if cmd.is_null() {
      cmd = (*proc_0).comm.as_mut_ptr()
    } else {
      let mut i: libc::c_int = (*proc_0).argv_len as libc::c_int;
      if opt & (1i32 << OPTBIT_A as libc::c_int) as libc::c_uint == 0 {
        cmdlen = strlen(cmd) as libc::c_int
      }
      /*
       * "sleep 11" looks like "sleep""\0""11""\0" in argv0.
       * Make sure last "\0" does not get converted to " ":
       */
      if i != 0 && *cmd.offset((i - 1i32) as isize) as libc::c_int == '\u{0}' as i32 {
        i -= 1
      }
      loop {
        i -= 1;
        if !(i >= 0) {
          break;
        }
        if (*cmd.offset(i as isize) as libc::c_uchar as libc::c_int) < ' ' as i32 {
          *cmd.offset(i as isize) = ' ' as i32 as libc::c_char
        }
      }
    }
    if opt & (1i32 << OPTBIT_V as libc::c_int) as libc::c_uint != 0 {
      /* "pgrep -v -P1 firefox" means "not (ppid=1 AND name=firefox)"
       * or equivalently "ppid!=1 OR name!=firefox".
       * Check the first condition and if true, skip matching.
       */
      if ppid2match >= 0 && ppid2match as libc::c_uint != (*proc_0).ppid {
        current_block = 7077386041754914429; /* if no PATTERN, then it's a match, else... */
      } else if sid2match >= 0 && sid2match as libc::c_uint != (*proc_0).sid {
        current_block = 7077386041754914429;
      } else {
        current_block = 2500484646272006982;
      }
    } else {
      current_block = 2500484646272006982;
    }
    match current_block {
      2500484646272006982 => {
        match_0 = (*argv.offset(0)).is_null() as libc::c_int;
        if match_0 == 0 {
          loop {
            match_0 = (regexec(
              &mut Z.re_buffer,
              cmd,
              1i32 as size_t,
              Z.re_match.as_mut_ptr(),
              0,
            ) == 0) as libc::c_int;
            if !(match_0 == 0 && cmd != (*proc_0).comm.as_mut_ptr()) {
              break;
            }
            /* if argv[] did not match, try comm */
            cmdlen = -1i32;
            cmd = (*proc_0).comm.as_mut_ptr()
          }
          if match_0 != 0 && opt & (1i32 << OPTBIT_X as libc::c_int) as libc::c_uint != 0 {
            /* -x requires full string match */
            match_0 = ((*Z.re_match.as_mut_ptr().offset(0)).rm_so == 0
              && *cmd.offset((*Z.re_match.as_mut_ptr().offset(0)).rm_eo as isize) as libc::c_int
                == '\u{0}' as i32) as libc::c_int
          }
        }
        /* NB: OPT_INVERT is always 0 or 1 */
        if !(match_0 as libc::c_uint ^ opt & (1i32 << OPTBIT_V as libc::c_int) as libc::c_uint != 0)
        {
          continue;
        }
      }
      _ => {}
    }
    matched_pid = (*proc_0).pid as libc::c_int;
    if opt & (1i32 << OPTBIT_N as libc::c_int) as libc::c_uint != 0 {
      free(cmd_last as *mut libc::c_void);
      cmd_last = crate::libbb::xfuncs_printf::xstrdup(cmd)
    } else {
      if cmdlen >= 0 {
        *cmd.offset(cmdlen as isize) = '\u{0}' as i32 as libc::c_char
      }
      act((*proc_0).pid, cmd, signo);
      if opt & (1i32 << OPTBIT_O as libc::c_int) as libc::c_uint != 0 {
        break;
      }
    }
  }
  if !cmd_last.is_null() {
    act(matched_pid as libc::c_uint, cmd_last, signo);
  }
  return (matched_pid == 0) as libc::c_int;
  /* return 1 if no processes listed/signaled */
}
