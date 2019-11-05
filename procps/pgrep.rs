use c2rust_bitfields;
use c2rust_bitfields::BitfieldStruct;
use libc;
extern "C" {
  pub type __dirstream;
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  fn getpid() -> __pid_t;
  #[no_mangle]
  fn getsid(__pid: __pid_t) -> __pid_t;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  static mut option_mask32: uint32_t;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn get_signum(name: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn print_signames();
  #[no_mangle]
  fn procps_scan(sp: *mut procps_status_t, flags: libc::c_int) -> *mut procps_status_t;
  #[no_mangle]
  static mut applet_name: *const libc::c_char;
  #[no_mangle]
  fn regexec(
    __preg: *const regex_t,
    __string: *const libc::c_char,
    __nmatch: size_t,
    __pmatch: *mut regmatch_t,
    __eflags: libc::c_int,
  ) -> libc::c_int;
  #[no_mangle]
  fn xregcomp(preg: *mut regex_t, regex: *const libc::c_char, cflags: libc::c_int);
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __pid_t = libc::c_int;
use crate::librb::uint8_t;
pub type uint16_t = __uint16_t;
use crate::librb::uint32_t;
use crate::librb::size_t;
pub type DIR = __dirstream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct smaprec {
  pub mapped_rw: libc::c_ulong,
  pub mapped_ro: libc::c_ulong,
  pub shared_clean: libc::c_ulong,
  pub shared_dirty: libc::c_ulong,
  pub private_clean: libc::c_ulong,
  pub private_dirty: libc::c_ulong,
  pub stack: libc::c_ulong,
  pub smap_pss: libc::c_ulong,
  pub smap_swap: libc::c_ulong,
  pub smap_size: libc::c_ulong,
  pub smap_start: libc::c_ulonglong,
  pub smap_mode: [libc::c_char; 5],
  pub smap_name: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct procps_status_t {
  pub dir: *mut DIR,
  pub task_dir: *mut DIR,
  pub shift_pages_to_bytes: uint8_t,
  pub shift_pages_to_kb: uint8_t,
  pub argv_len: uint16_t,
  pub argv0: *mut libc::c_char,
  pub exe: *mut libc::c_char,
  pub main_thread_pid: libc::c_uint,
  pub vsz: libc::c_ulong,
  pub rss: libc::c_ulong,
  pub stime: libc::c_ulong,
  pub utime: libc::c_ulong,
  pub start_time: libc::c_ulong,
  pub pid: libc::c_uint,
  pub ppid: libc::c_uint,
  pub pgid: libc::c_uint,
  pub sid: libc::c_uint,
  pub uid: libc::c_uint,
  pub gid: libc::c_uint,
  pub ruid: libc::c_uint,
  pub rgid: libc::c_uint,
  pub niceness: libc::c_int,
  pub tty_major: libc::c_uint,
  pub tty_minor: libc::c_uint,
  pub smaps: smaprec,
  pub state: [libc::c_char; 4],
  pub comm: [libc::c_char; 16],
  pub last_seen_on_cpu: libc::c_int,
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
pub type reg_syntax_t = libc::c_ulong;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct re_pattern_buffer {
  pub buffer: *mut libc::c_uchar,
  pub allocated: libc::c_ulong,
  pub used: libc::c_ulong,
  pub syntax: reg_syntax_t,
  pub fastmap: *mut libc::c_char,
  pub translate: *mut libc::c_uchar,
  pub re_nsub: size_t,
  #[bitfield(name = "can_be_null", ty = "libc::c_uint", bits = "0..=0")]
  #[bitfield(name = "regs_allocated", ty = "libc::c_uint", bits = "1..=2")]
  #[bitfield(name = "fastmap_accurate", ty = "libc::c_uint", bits = "3..=3")]
  #[bitfield(name = "no_sub", ty = "libc::c_uint", bits = "4..=4")]
  #[bitfield(name = "not_bol", ty = "libc::c_uint", bits = "5..=5")]
  #[bitfield(name = "not_eol", ty = "libc::c_uint", bits = "6..=6")]
  #[bitfield(name = "newline_anchor", ty = "libc::c_uint", bits = "7..=7")]
  pub can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [u8; 1],
  #[bitfield(padding)]
  pub c2rust_padding: [u8; 7],
}
pub type regex_t = re_pattern_buffer;
pub type regoff_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regmatch_t {
  pub rm_so: regoff_t,
  pub rm_eo: regoff_t,
}
/* vi: set sw=4 ts=4: */
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
//applet:IF_PGREP(APPLET_ODDNAME(pgrep, pgrep, BB_DIR_USR_BIN, BB_SUID_DROP, pgrep))
//                APPLET_ODDNAME:name   main   location        suid_type     help
//applet:IF_PKILL(APPLET_ODDNAME(pkill, pgrep, BB_DIR_USR_BIN, BB_SUID_DROP, pkill))
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
#[derive(Copy, Clone)]
#[repr(C)]
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
    kill(pid as __pid_t, signo);
  };
}
#[no_mangle]
pub unsafe extern "C" fn pgrep_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut current_block: u64;
  let mut pid: libc::c_uint = 0;
  let mut signo: libc::c_int = 0;
  let mut opt: libc::c_uint = 0;
  let mut scan_mask: libc::c_int = 0;
  let mut matched_pid: libc::c_int = 0;
  let mut sid2match: libc::c_int = 0;
  let mut ppid2match: libc::c_int = 0;
  let mut cmd_last: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut proc_0: *mut procps_status_t = 0 as *mut procps_status_t;
  /* These are initialized to 0 */
  let mut Z: C2RustUnnamed_1 = C2RustUnnamed_1 {
    re_buffer: regex_t {
      buffer: 0 as *mut libc::c_uchar,
      allocated: 0,
      used: 0,
      syntax: 0,
      fastmap: 0 as *mut libc::c_char,
      translate: 0 as *mut libc::c_uchar,
      re_nsub: 0,
      can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [0; 1],
      c2rust_padding: [0; 7],
    },
    re_match: [regmatch_t { rm_so: 0, rm_eo: 0 }; 1],
  };
  memset(
    &mut Z as *mut C2RustUnnamed_1 as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong,
  );
  /* Parse -SIGNAL for pkill. Must be first option, if present */
  signo = 15i32;
  if 1i32 != 0
    && (1i32 == 0 || *applet_name.offset(1) as libc::c_int == 'k' as i32)
    && !(*argv.offset(1)).is_null()
    && *(*argv.offset(1)).offset(0) as libc::c_int == '-' as i32
  {
    let mut temp: libc::c_int = get_signum((*argv.offset(1)).offset(1));
    if temp != -1i32 {
      signo = temp;
      argv = argv.offset(1)
    }
  }
  /* Parse remaining options */
  ppid2match = -1i32;
  sid2match = -1i32;
  opt = getopt32(
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
    print_signames();
    return 0i32;
  }
  pid = getpid() as libc::c_uint;
  if sid2match == 0i32 {
    sid2match = getsid(pid as __pid_t)
  }
  scan_mask = PSSCAN_COMM as libc::c_int | PSSCAN_ARGV0 as libc::c_int;
  if opt & (1i32 << OPTBIT_F as libc::c_int) as libc::c_uint != 0 {
    scan_mask |= PSSCAN_ARGVN as libc::c_int
  }
  /* One pattern is required, if no -s and no -P */
  if sid2match & ppid2match < 0i32 && ((*argv.offset(0)).is_null() || !(*argv.offset(1)).is_null())
  {
    bb_show_usage();
  }
  if !(*argv.offset(0)).is_null() {
    xregcomp(
      &mut Z.re_buffer,
      *argv.offset(0),
      if opt & (1i32 << OPTBIT_X as libc::c_int) as libc::c_uint != 0 {
        1i32
      } else {
        (1i32) | 1i32 << 1i32 << 1i32 << 1i32
      },
    );
  }
  matched_pid = 0i32;
  cmd_last = 0 as *mut libc::c_char;
  proc_0 = 0 as *mut procps_status_t;
  loop {
    proc_0 = procps_scan(proc_0, scan_mask);
    if proc_0.is_null() {
      break;
    }
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cmdlen: libc::c_int = 0;
    let mut match_0: libc::c_int = 0;
    if (*proc_0).pid == pid {
      continue;
    }
    if opt & (1i32 << OPTBIT_V as libc::c_int) as libc::c_uint == 0 {
      /* Quickly reject -sN -PN mismatches... unless -v */
      if ppid2match >= 0i32 && ppid2match as libc::c_uint != (*proc_0).ppid {
        continue; /* not -a: find first NUL */
      }
      if sid2match >= 0i32 && sid2match as libc::c_uint != (*proc_0).sid {
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
        if !(i >= 0i32) {
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
      if ppid2match >= 0i32 && ppid2match as libc::c_uint != (*proc_0).ppid {
        current_block = 7077386041754914429; /* if no PATTERN, then it's a match, else... */
      } else if sid2match >= 0i32 && sid2match as libc::c_uint != (*proc_0).sid {
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
              0i32,
            ) == 0i32) as libc::c_int;
            if !(match_0 == 0 && cmd != (*proc_0).comm.as_mut_ptr()) {
              break;
            }
            /* if argv[] did not match, try comm */
            cmdlen = -1i32;
            cmd = (*proc_0).comm.as_mut_ptr()
          }
          if match_0 != 0 && opt & (1i32 << OPTBIT_X as libc::c_int) as libc::c_uint != 0 {
            /* -x requires full string match */
            match_0 = ((*Z.re_match.as_mut_ptr().offset(0)).rm_so == 0i32
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
      cmd_last = xstrdup(cmd)
    } else {
      if cmdlen >= 0i32 {
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
  return (matched_pid == 0i32) as libc::c_int;
  /* return 1 if no processes listed/signaled */
}
