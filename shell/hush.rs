use libc;
extern "C" {
  #[no_mangle]
  fn fclose(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn getc_unlocked(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn putchar_unlocked(__c: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn puts(__s: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  fn glob(
    __pattern: *const libc::c_char,
    __flags: libc::c_int,
    __errfunc: Option<unsafe extern "C" fn(_: *const libc::c_char, _: libc::c_int) -> libc::c_int>,
    __pglob: *mut glob_t,
  ) -> libc::c_int;
  #[no_mangle]
  fn globfree(__pglob: *mut glob_t);
  #[no_mangle]
  fn fnmatch(
    __pattern: *const libc::c_char,
    __name: *const libc::c_char,
    __flags: libc::c_int,
  ) -> libc::c_int;
  #[no_mangle]
  fn times(__buffer: *mut tms) -> clock_t;
  #[no_mangle]
  fn uname(__name: *mut utsname) -> libc::c_int;
  #[no_mangle]
  fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
  #[no_mangle]
  fn chdir(__path: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn dup(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  static mut environ: *mut *mut libc::c_char;
  #[no_mangle]
  fn execvp(__file: *const libc::c_char, __argv: *const *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn _exit(_: libc::c_int) -> !;
  #[no_mangle]
  fn getpid() -> __pid_t;
  #[no_mangle]
  fn getppid() -> __pid_t;
  #[no_mangle]
  fn getpgrp() -> __pid_t;
  #[no_mangle]
  fn setpgid(__pid: __pid_t, __pgid: __pid_t) -> libc::c_int;
  #[no_mangle]
  fn setpgrp() -> libc::c_int;
  #[no_mangle]
  fn fork() -> __pid_t;
  #[no_mangle]
  fn vfork() -> libc::c_int;
  #[no_mangle]
  fn isatty(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn tcgetpgrp(__fd: libc::c_int) -> __pid_t;
  #[no_mangle]
  fn tcsetpgrp(__fd: libc::c_int, __pgrp_id: __pid_t) -> libc::c_int;
  #[no_mangle]
  static mut optarg: *mut libc::c_char;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  static mut opterr: libc::c_int;
  #[no_mangle]
  static mut optopt: libc::c_int;
  #[no_mangle]
  fn getopt(
    ___argc: libc::c_int,
    ___argv: *const *mut libc::c_char,
    __shortopts: *const libc::c_char,
  ) -> libc::c_int;
  #[no_mangle]
  fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
  #[no_mangle]
  fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
  #[no_mangle]
  fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn raise(__sig: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn sigfillset(__set: *mut sigset_t) -> libc::c_int;
  #[no_mangle]
  fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn sigdelset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn sigismember(__set: *const sigset_t, __signo: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn sigisemptyset(__set: *const sigset_t) -> libc::c_int;
  #[no_mangle]
  fn sigprocmask(__how: libc::c_int, __set: *const sigset_t, __oset: *mut sigset_t) -> libc::c_int;
  #[no_mangle]
  fn sigsuspend(__set: *const sigset_t) -> libc::c_int;
  #[no_mangle]
  fn sigaction(__sig: libc::c_int, __act: *const sigaction, __oact: *mut sigaction) -> libc::c_int;
  #[no_mangle]
  fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn putenv(__string: *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn unsetenv(__name: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
  #[no_mangle]
  fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
  #[no_mangle]
  fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
  #[no_mangle]
  fn mempcpy(
    __dest: *mut libc::c_void,
    __src: *const libc::c_void,
    __n: size_t,
  ) -> *mut libc::c_void;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn strnlen(__string: *const libc::c_char, __maxlen: size_t) -> size_t;
  #[no_mangle]
  fn stpcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn umask(__mask: __mode_t) -> __mode_t;
  #[no_mangle]
  fn gettimeofday(__tv: *mut timeval, __tz: __timezone_ptr_t) -> libc::c_int;
  #[no_mangle]
  fn wait(__stat_loc: *mut libc::c_int) -> __pid_t;
  #[no_mangle]
  fn waitpid(__pid: __pid_t, __stat_loc: *mut libc::c_int, __options: libc::c_int) -> __pid_t;
  #[no_mangle]
  fn strchrnul(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn skip_whitespace(_: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn skip_non_whitespace(_: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xmalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xrealloc(old: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xstrndup(s: *const libc::c_char, n: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn xmemdup(s: *const libc::c_void, n: libc::c_int) -> *mut libc::c_void;
  #[no_mangle]
  fn endofname(name: *const libc::c_char) -> *const libc::c_char;
  #[no_mangle]
  fn is_prefixed_with(string: *const libc::c_char, key: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn ndelay_on(fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn ndelay_off(fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn close_on_exec_on(fd: libc::c_int);
  #[no_mangle]
  fn xdup2(_: libc::c_int, _: libc::c_int);
  #[no_mangle]
  fn xmove_fd(_: libc::c_int, _: libc::c_int);
  #[no_mangle]
  fn xrealloc_getcwd_or_warn(cwd: *mut libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_signals(sigs: libc::c_int, f: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>);
  #[no_mangle]
  fn kill_myself_with_sig(sig: libc::c_int) -> !;
  #[no_mangle]
  fn sigprocmask_allsigs(how: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn sigprocmask2(how: libc::c_int, set: *mut sigset_t) -> libc::c_int;
  #[no_mangle]
  fn bb_unsetenv(key: *const libc::c_char);
  #[no_mangle]
  fn open_or_warn(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xpipe(filedes: *mut libc::c_int);
  #[no_mangle]
  fn string_array_len(argv: *mut *mut libc::c_char) -> libc::c_uint;
  #[no_mangle]
  fn overlapping_strcpy(dst: *mut libc::c_char, src: *const libc::c_char);
  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  #[no_mangle]
  fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn bb_clk_tck() -> libc::c_uint;
  #[no_mangle]
  fn full_write(fd: libc::c_int, buf: *const libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn fflush_all() -> libc::c_int;
  #[no_mangle]
  fn xfdopen_for_read(fd: libc::c_int) -> *mut FILE;
  #[no_mangle]
  fn utoa(n: libc::c_uint) -> *mut libc::c_char;
  #[no_mangle]
  fn xatoi(str: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn bb_strtou(
    arg: *const libc::c_char,
    endp: *mut *mut libc::c_char,
    base: libc::c_int,
  ) -> libc::c_uint;
  #[no_mangle]
  fn xatoi_positive(numstr: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn find_executable(
    filename: *const libc::c_char,
    PATHp: *mut *mut libc::c_char,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn xfork() -> pid_t;
  #[no_mangle]
  fn safe_waitpid(pid: pid_t, wstat: *mut libc::c_int, options: libc::c_int) -> pid_t;
  #[no_mangle]
  fn run_nofork_applet(applet_no: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn find_applet_by_name(name: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;
  #[no_mangle]
  static mut xfunc_error_retval: uint8_t;
  #[no_mangle]
  static mut die_func: Option<unsafe extern "C" fn() -> ()>;
  #[no_mangle]
  fn xfunc_die() -> !;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_simple_error_msg(s: *const libc::c_char);
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_perror_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_verror_msg(s: *const libc::c_char, p: ::std::ffi::VaList, strerr: *const libc::c_char);
  #[no_mangle]
  fn bb_die_memory_exhausted() -> !;
  #[no_mangle]
  fn get_signame(number: libc::c_int) -> *const libc::c_char;
  #[no_mangle]
  fn bb_parse_mode(s: *const libc::c_char, cur_mode: libc::c_uint) -> libc::c_int;
  #[no_mangle]
  fn test_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  static bb_msg_unknown: [libc::c_char; 0];
  #[no_mangle]
  fn printf_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn echo_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn get_signum(name: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn index_in_strings(strings: *const libc::c_char, key: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn kill_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn show_history(st: *const line_input_t);
  #[no_mangle]
  fn new_line_input_t(flags: libc::c_int) -> *mut line_input_t;
  #[no_mangle]
  fn concat_path_file(
    path: *const libc::c_char,
    filename: *const libc::c_char,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn size_from_HISTFILESIZE(hp: *const libc::c_char) -> libc::c_uint;
  #[no_mangle]
  static bb_banner: [libc::c_char; 0];
  #[no_mangle]
  fn read_line_input(
    st: *mut line_input_t,
    prompt: *const libc::c_char,
    command: *mut libc::c_char,
    maxsize: libc::c_int,
  ) -> libc::c_int;
  /* NB: (bb_hexdigits_upcase[i] | 0x20) -> lowercase hex digit */
  /* Busybox mount uses either /proc/mounts or /etc/mtab to
   * get the list of currently mounted filesystems */
  /* allow default system PATH to be extended via CFLAGS */
  /* BB_PATH_ROOT_PATH */
  /* util-linux manpage says /sbin:/bin:/usr/sbin:/usr/bin,
   * but I want to save a few bytes here:
   */
  //extern const int const_int_1;
  /* This struct is deliberately not defined. */
  /* See docs/keep_data_small.txt */
  /* '*const' ptr makes gcc optimize code much better.
   * Magic prevents ptr_to_globals from going into rodata.
   * If you want to assign a value, use SET_PTR_TO_GLOBALS(x) */
  #[no_mangle]
  static ptr_to_globals: *mut globals;
  #[no_mangle]
  static bb_PATH_root_path: [libc::c_char; 0];
  /* Number of unicode chars. Falls back to strlen() on invalid unicode */
  #[no_mangle]
  fn unicode_strlen(string: *const libc::c_char) -> size_t;
  /* vi: set sw=4 ts=4: */
  /*
   * Adapted from ash applet code
   *
   * This code is derived from software contributed to Berkeley by
   * Kenneth Almquist.
   *
   * Copyright (c) 1989, 1991, 1993, 1994
   *      The Regents of the University of California.  All rights reserved.
   *
   * Copyright (c) 1997-2005 Herbert Xu <herbert@gondor.apana.org.au>
   * was re-ported from NetBSD and debianized.
   *
   * Copyright (c) 2010 Denys Vlasenko
   * Split from ash.c
   *
   * Licensed under GPLv2 or later, see file LICENSE in this source tree.
   */
  #[no_mangle]
  static defifsvar: [libc::c_char; 0];
  //TODO? do not provide bashisms if not asked for:
  //#if !ENABLE_HUSH_BASH_COMPAT && !ENABLE_ASH_BASH_COMPAT
  //#define shell_builtin_read(setvar,argv,ifs,read_flags,n,p,t,u,d)
  //	shell_builtin_read(setvar,argv,ifs,read_flags)
  //#endif
  #[no_mangle]
  fn shell_builtin_read(params: *mut builtin_read_params) -> *const libc::c_char;
  #[no_mangle]
  fn shell_builtin_ulimit(argv: *mut *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn arith(state: *mut arith_state_t, expr: *const libc::c_char) -> arith_t;
  #[no_mangle]
  fn scan_and_match(
    string: *mut libc::c_char,
    pattern: *const libc::c_char,
    flags: libc::c_uint,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn next_random(rnd: *mut random_t) -> uint32_t;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
  pub gp_offset: libc::c_uint,
  pub fp_offset: libc::c_uint,
  pub overflow_arg_area: *mut libc::c_void,
  pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __ino64_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;



use crate::librb::FILE;
pub type va_list = __builtin_va_list;
use crate::librb::ssize_t;
pub type __size_t = libc::c_ulong;
use crate::librb::stat;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct glob_t {
  pub gl_pathc: __size_t,
  pub gl_pathv: *mut *mut libc::c_char,
  pub gl_offs: __size_t,
  pub gl_flags: libc::c_int,
  pub gl_closedir: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
  pub gl_readdir: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> *mut dirent>,
  pub gl_opendir: Option<unsafe extern "C" fn(_: *const libc::c_char) -> *mut libc::c_void>,
  pub gl_lstat: Option<unsafe extern "C" fn(_: *const libc::c_char, _: *mut stat) -> libc::c_int>,
  pub gl_stat: Option<unsafe extern "C" fn(_: *const libc::c_char, _: *mut stat) -> libc::c_int>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
  pub d_ino: __ino64_t,
  pub d_off: __off64_t,
  pub d_reclen: libc::c_ushort,
  pub d_type: libc::c_uchar,
  pub d_name: [libc::c_char; 256],
}
pub type clock_t = __clock_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tms {
  pub tms_utime: clock_t,
  pub tms_stime: clock_t,
  pub tms_cutime: clock_t,
  pub tms_cstime: clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utsname {
  pub sysname: [libc::c_char; 65],
  pub nodename: [libc::c_char; 65],
  pub release: [libc::c_char; 65],
  pub version: [libc::c_char; 65],
  pub machine: [libc::c_char; 65],
  pub domainname: [libc::c_char; 65],
}
pub type int32_t = __int32_t;
use crate::librb::uint8_t;
pub type uint16_t = __uint16_t;
use crate::librb::uint32_t;
pub type uintptr_t = libc::c_ulong;
/* NB: unaligned parameter should be a pointer, aligned one -
 * a lvalue. This makes it more likely to not swap them by mistake
 */
/* #elif ... - add your favorite arch today! */
/* Unaligned, fixed-endian accessors */
/* unxz needs an aligned fixed-endian accessor.
 * (however, the compiler does not realize it's aligned, the cast is still necessary)
 */
/* ---- Size-saving "small" ints (arch-dependent) ----------- */
/* add other arches which benefit from this... */
use crate::librb::smallint;
pub type smalluint = libc::c_uchar;
use crate::librb::pid_t;
use crate::librb::mode_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
  pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
use crate::librb::timeval;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
  pub sival_int: libc::c_int,
  pub sival_ptr: *mut libc::c_void,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
  pub si_signo: libc::c_int,
  pub si_errno: libc::c_int,
  pub si_code: libc::c_int,
  pub __pad0: libc::c_int,
  pub _sifields: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
  pub _pad: [libc::c_int; 28],
  pub _kill: C2RustUnnamed_8,
  pub _timer: C2RustUnnamed_7,
  pub _rt: C2RustUnnamed_6,
  pub _sigchld: C2RustUnnamed_5,
  pub _sigfault: C2RustUnnamed_2,
  pub _sigpoll: C2RustUnnamed_1,
  pub _sigsys: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
  pub _call_addr: *mut libc::c_void,
  pub _syscall: libc::c_int,
  pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
  pub si_band: libc::c_long,
  pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
  pub si_addr: *mut libc::c_void,
  pub si_addr_lsb: libc::c_short,
  pub _bounds: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
  pub _addr_bnd: C2RustUnnamed_4,
  pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
  pub _lower: *mut libc::c_void,
  pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
  pub si_pid: __pid_t,
  pub si_uid: __uid_t,
  pub si_status: libc::c_int,
  pub si_utime: __clock_t,
  pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
  pub si_pid: __pid_t,
  pub si_uid: __uid_t,
  pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
  pub si_tid: libc::c_int,
  pub si_overrun: libc::c_int,
  pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
  pub si_pid: __pid_t,
  pub si_uid: __uid_t,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
pub type sighandler_t = __sighandler_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
  pub __sigaction_handler: C2RustUnnamed_9,
  pub sa_mask: __sigset_t,
  pub sa_flags: libc::c_int,
  pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
  pub sa_handler: __sighandler_t,
  pub sa_sigaction:
    Option<unsafe extern "C" fn(_: libc::c_int, _: *mut siginfo_t, _: *mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timezone {
  pub tz_minuteswest: libc::c_int,
  pub tz_dsttime: libc::c_int,
}
pub type __timezone_ptr_t = *mut timezone;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_pair {
  pub rd: libc::c_int,
  pub wr: libc::c_int,
}
/* math.h - interface to shell math "library" -- this allows shells to share
 *          the implementation of arithmetic $((...)) expansions.
 *
 * This aims to be a POSIX shell math library as documented here:
 *	http://www.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html#tag_18_06_04
 *
 * See math.c for internal documentation.
 */
/* The math library has just one function:
 *
 * arith_t arith(arith_state_t *state, const char *expr);
 *
 * The expr argument is the math string to parse.  All normal expansions must
 * be done already.  i.e. no dollar symbols should be present.
 *
 * The state argument is a pointer to a struct of hooks for your shell (see below),
 * and an error message string (NULL if no error).
 *
 * The function returns the answer to the expression.  So if you called it
 * with the expression:
 * "1 + 2 + 3"
 * you would obviously get back 6.
 */
/* To add support to a shell, you need to implement three functions:
 *
 * lookupvar() - look up and return the value of a variable
 *
 *	If the shell does:
 *		foo=123
 *	Then the code:
 *		const char *val = lookupvar("foo");
 *	will result in val pointing to "123"
 *
 * setvar() - set a variable to some value
 *
 *	If the arithmetic expansion does something like:
 *		$(( i = 1))
 *	then the math code will make a call like so:
 *		setvar("i", "1", 0);
 *	The storage for the first two parameters are not allocated, so your
 *	shell implementation will most likely need to strdup() them to save.
 *
 * endofname() - return the end of a variable name from input
 *
 *	The arithmetic code does not know about variable naming conventions.
 *	So when it is given an experession, it knows something is not numeric,
 *	but it is up to the shell to dictate what is a valid identifiers.
 *	So when it encounters something like:
 *		$(( some_var + 123 ))
 *	It will make a call like so:
 *		end = endofname("some_var + 123");
 *	So the shell needs to scan the input string and return a pointer to the
 *	first non-identifier string.  In this case, it should return the input
 *	pointer with an offset pointing to the first space.  The typical
 *	implementation will return the offset of first char that does not match
 *	the regex (in C locale): ^[a-zA-Z_][a-zA-Z_0-9]*
 */
pub type arith_t = libc::c_longlong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub interactive_fd: libc::c_int,
  pub line_input_state: *mut line_input_t,
  pub root_pid: pid_t,
  pub root_ppid: pid_t,
  pub last_bg_pid: pid_t,
  pub random_gen: random_t,
  pub run_list_level: libc::c_int,
  pub last_jobid: libc::c_uint,
  pub saved_tty_pgrp: pid_t,
  pub job_list: *mut pipe,
  pub errexit_depth: libc::c_int,
  pub o_opt: [libc::c_char; 4],
  pub opt_s: libc::c_char,
  pub opt_c: libc::c_char,
  pub promptmode: smallint,
  pub flag_SIGINT: smallint,
  pub flag_break_continue: smallint,
  pub flag_return_in_progress: smallint,
  pub exiting: smallint,
  pub last_exitcode: smalluint,
  pub expand_exitcode: smalluint,
  pub last_bg_pid_exitcode: smalluint,
  pub global_args_malloced: smalluint,
  pub dead_job_exitcode: libc::c_int,
  pub global_argc: libc::c_int,
  pub global_argv: *mut *mut libc::c_char,
  pub depth_break_continue: libc::c_uint,
  pub depth_of_loop: libc::c_uint,
  pub getopt_count: libc::c_uint,
  pub ifs: *const libc::c_char,
  pub ifs_whitespace: *mut libc::c_char,
  pub cwd: *const libc::c_char,
  pub top_var: *mut variable,
  pub expanded_assignments: *mut *mut libc::c_char,
  pub shadowed_vars_pp: *mut *mut variable,
  pub var_nest_level: libc::c_uint,
  pub func_nest_level: libc::c_uint,
  pub top_func: *mut function,
  pub parse_lineno: libc::c_uint,
  pub execute_lineno: libc::c_uint,
  pub HFILE_list: *mut HFILE,
  pub special_sig_mask: libc::c_uint,
  pub fatal_sig_mask: libc::c_uint,
  pub traps: *mut *mut libc::c_char,
  pub pending_set: sigset_t,
  pub x_mode_depth: libc::c_uint,
  pub x_mode_fd: libc::c_int,
  pub x_mode_buf: o_string,
  pub sa: sigaction,
  pub optstring_buf: [libc::c_char; 6],
  pub epoch_buf: [libc::c_char; 35],
  pub user_input_buf: [libc::c_char; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct o_string {
  pub data: *mut libc::c_char,
  pub length: libc::c_int,
  pub maxlen: libc::c_int,
  pub o_expflags: libc::c_int,
  pub has_quoted_part: smallint,
  pub has_empty_slot: smallint,
  pub ended_in_ifs: smallint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HFILE {
  pub cur: *mut libc::c_char,
  pub end: *mut libc::c_char,
  pub next_hfile: *mut HFILE,
  pub is_stdin: libc::c_int,
  pub fd: libc::c_int,
  pub buf: [libc::c_char; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct function {
  pub next: *mut function,
  pub name: *mut libc::c_char,
  pub parent_cmd: *mut command,
  pub body: *mut pipe,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pipe {
  pub next: *mut pipe,
  pub num_cmds: libc::c_int,
  pub alive_cmds: libc::c_int,
  pub stopped_cmds: libc::c_int,
  pub jobid: libc::c_uint,
  pub pgrp: pid_t,
  pub cmdtext: *mut libc::c_char,
  pub cmds: *mut command,
  pub followup: smallint,
  pub pi_inverted: smallint,
  pub res_word: smallint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct command {
  pub pid: pid_t,
  pub assignment_cnt: libc::c_uint,
  pub lineno: libc::c_uint,
  pub cmd_type: smallint,
  pub cmd_exitcode: smalluint,
  pub group: *mut pipe,
  pub child_func: *mut function,
  pub argv: *mut *mut libc::c_char,
  pub redirects: *mut redir_struct,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redir_struct {
  pub next: *mut redir_struct,
  pub rd_filename: *mut libc::c_char,
  pub rd_fd: libc::c_int,
  pub rd_dup: libc::c_int,
  pub rd_type: smallint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct variable {
  pub next: *mut variable,
  pub varstr: *mut libc::c_char,
  pub max_len: libc::c_int,
  pub var_nest_level: uint16_t,
  pub flg_export: smallint,
  pub flg_read_only: smallint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct random_t {
  pub galois_LFSR: int32_t,
  pub LCG: uint32_t,
  pub xs64_x: uint32_t,
  pub xs64_y: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct line_input_t {
  pub flags: libc::c_int,
  pub timeout: libc::c_int,
  pub path_lookup: *const libc::c_char,
  pub cnt_history: libc::c_int,
  pub cur_history: libc::c_int,
  pub max_history: libc::c_int,
  pub cnt_history_in_file: libc::c_uint,
  pub hist_file: *const libc::c_char,
  pub history: [*mut libc::c_char; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_str {
  pub p: *const libc::c_char,
  pub peek_buf: [libc::c_int; 2],
  pub last_char: libc::c_int,
  pub file: *mut HFILE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct squirrel {
  pub orig_fd: libc::c_int,
  pub moved_to: libc::c_int,
  /* moved_to = n: fd was moved to n; restore back to orig_fd after redir */
  /* moved_to = -1: fd was opened by redirect; close orig_fd after redir */
}
/* If a reserved word is found and processed, parse context is modified
 * and 1 is returned.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct reserved_combo {
  pub literal: [libc::c_char; 6],
  pub res: libc::c_uchar,
  pub assignment_flag: libc::c_uchar,
  pub flag: libc::c_int,
}
/* Table of built-in functions.  They can be forked or not, depending on
 * context: within pipes, they fork.  As simple commands, they do not.
 * When used in non-forking context, they can change global variables
 * in the parent shell process.  If forked, of course they cannot.
 * For example, 'unset foo | whatever' will parse and run, but foo will
 * still be set at the end. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct built_in_command {
  pub b_cmd: *const libc::c_char,
  pub b_function: Option<unsafe extern "C" fn(_: *mut *mut libc::c_char) -> libc::c_int>,
  pub b_descr: *const libc::c_char,
}
pub const RES_UNTIL: C2RustUnnamed_14 = 8;
pub const RES_WHILE: C2RustUnnamed_14 = 7;
pub const RES_DONE: C2RustUnnamed_14 = 10;
pub const RES_DO: C2RustUnnamed_14 = 9;
pub const OPT_O_PIPEFAIL: C2RustUnnamed_18 = 0;
pub const RES_ELIF: C2RustUnnamed_14 = 3;
pub const RES_IF: C2RustUnnamed_14 = 1;
pub const OPT_O_ERREXIT: C2RustUnnamed_18 = 2;
pub const PIPE_BG: pipe_style = 3;
pub const BC_BREAK: C2RustUnnamed_17 = 1;
pub const SPECIAL_JOBSTOP_SIGS: C2RustUnnamed_19 = 7340032;
pub const SPECIAL_INTERACTIVE_SIGS: C2RustUnnamed_19 = 32774;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct save_arg_t {
  pub sv_argv0: *mut libc::c_char,
  pub sv_g_argv: *mut *mut libc::c_char,
  pub sv_g_argc: libc::c_int,
  pub sv_g_malloced: smallint,
}
pub const OPT_O_XTRACE: C2RustUnnamed_18 = 3;
pub const OPT_O_NOEXEC: C2RustUnnamed_18 = 1;
/* "OPTIND=1" */
/* Builtins */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct builtin_read_params {
  pub read_flags: libc::c_int,
  pub setvar: Option<unsafe extern "C" fn(_: *const libc::c_char, _: *const libc::c_char) -> ()>,
  pub argv: *mut *mut libc::c_char,
  pub ifs: *const libc::c_char,
  pub opt_n: *const libc::c_char,
  pub opt_p: *const libc::c_char,
  pub opt_t: *const libc::c_char,
  pub opt_u: *const libc::c_char,
  pub opt_d: *const libc::c_char,
}
pub const EXP_FLAG_ESC_GLOB_CHARS: C2RustUnnamed_15 = 1;
pub const EXP_FLAG_GLOB: C2RustUnnamed_15 = 2;
pub const EXP_FLAG_SINGLEWORD: C2RustUnnamed_15 = 128;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arith_state_t {
  pub errmsg: *const libc::c_char,
  pub lookupvar: arith_var_lookup_t,
  pub setvar: arith_var_set_t,
  pub list_of_recursed_names: *mut libc::c_void,
}
pub type arith_var_set_t =
  Option<unsafe extern "C" fn(_: *const libc::c_char, _: *const libc::c_char) -> ()>;
pub type arith_var_lookup_t =
  Option<unsafe extern "C" fn(_: *const libc::c_char) -> *const libc::c_char>;
pub const SCAN_MATCH_LEFT_HALF: C2RustUnnamed_13 = 4;
pub const SCAN_MOVE_FROM_RIGHT: C2RustUnnamed_13 = 2;
pub const SCAN_MOVE_FROM_LEFT: C2RustUnnamed_13 = 1;
pub const SCAN_MATCH_RIGHT_HALF: C2RustUnnamed_13 = 8;
pub const REDIRFD_TO_FILE: redir_type = -1;
pub const REDIRFD_CLOSE: redir_type = -3;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
  pub mode: libc::c_int,
  pub default_fd: libc::c_schar,
  pub descrip: [libc::c_char; 3],
}
pub const HEREDOC_QUOTED: redir_type = 2;
pub const REDIRECT_HEREDOC2: redir_type = 5;
pub const RES_ESAC: C2RustUnnamed_14 = 16;
pub const RES_CASE_BODY: C2RustUnnamed_14 = 15;
pub const RES_MATCH: C2RustUnnamed_14 = 14;
pub const RES_CASE: C2RustUnnamed_14 = 12;
pub const RES_IN: C2RustUnnamed_14 = 11;
pub const RES_FOR: C2RustUnnamed_14 = 6;
pub const RES_ELSE: C2RustUnnamed_14 = 4;
pub const RES_THEN: C2RustUnnamed_14 = 2;
pub const PIPE_AND: pipe_style = 1;
pub const PIPE_OR: pipe_style = 2;
pub const PIPE_SEQ: pipe_style = 0;
pub const RES_XXXX: C2RustUnnamed_14 = 17;
pub const RES_NONE: C2RustUnnamed_14 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct parse_context {
  pub list_head: *mut pipe,
  pub pipe: *mut pipe,
  pub command: *mut command,
  pub pending_redirect: *mut redir_struct,
  pub word: o_string,
  pub is_assignment: smallint,
  pub ctx_res_w: smallint,
  pub ctx_inverted: smallint,
  pub ctx_dsemicolon: smallint,
  pub old_flag: libc::c_int,
  pub stack: *mut parse_context,
}
pub type pipe_style = libc::c_uint;
pub const RES_CASE_IN: C2RustUnnamed_14 = 13;
pub const RES_FI: C2RustUnnamed_14 = 5;
pub const MAYBE_ASSIGNMENT: C2RustUnnamed_16 = 0;
pub const DEFINITELY_ASSIGNMENT: C2RustUnnamed_16 = 1;
pub const NOT_ASSIGNMENT: C2RustUnnamed_16 = 2;
pub const WORD_IS_KEYWORD: C2RustUnnamed_16 = 3;
pub const RES_SNTX: C2RustUnnamed_14 = 18;
pub const FLAG_END: C2RustUnnamed_20 = 1;
pub const FLAG_START: C2RustUnnamed_20 = 131072;
pub const FLAG_ESAC: C2RustUnnamed_20 = 65536;
pub const FLAG_MATCH: C2RustUnnamed_20 = 16384;
pub const FLAG_DONE: C2RustUnnamed_20 = 1024;
pub const FLAG_DO: C2RustUnnamed_20 = 512;
pub const FLAG_IN: C2RustUnnamed_20 = 2048;
pub const FLAG_FI: C2RustUnnamed_20 = 32;
pub const FLAG_THEN: C2RustUnnamed_20 = 4;
pub const FLAG_ELSE: C2RustUnnamed_20 = 16;
pub const FLAG_ELIF: C2RustUnnamed_20 = 8;
pub const REDIRECT_HEREDOC: redir_type = 4;
pub type redir_type = libc::c_int;
pub const HEREDOC_SKIPTABS: redir_type = 1;
pub const REDIRFD_SYNTAX_ERR: redir_type = -2;
pub const REDIRECT_IO: redir_type = 3;
pub const REDIRECT_APPEND: redir_type = 2;
pub const REDIRECT_OVERWRITE: redir_type = 1;
pub const REDIRECT_INPUT: redir_type = 0;
pub const FOR_SHELL: C2RustUnnamed_12 = 7;
pub const OPT_login: C2RustUnnamed_11 = 1;
pub type C2RustUnnamed_11 = libc::c_uint;
pub type C2RustUnnamed_12 = libc::c_uint;
pub const WITH_PATH_LOOKUP: C2RustUnnamed_12 = 16;
pub const VI_MODE: C2RustUnnamed_12 = 0;
pub const USERNAME_COMPLETION: C2RustUnnamed_12 = 4;
pub const TAB_COMPLETION: C2RustUnnamed_12 = 2;
pub const DO_HISTORY: C2RustUnnamed_12 = 1;
/* match.h - interface to shell ##/%% matching code */
//TODO! Why ash.c still uses internal version?!
pub type C2RustUnnamed_13 = libc::c_uint;
pub type C2RustUnnamed_14 = libc::c_uint;
pub type C2RustUnnamed_15 = libc::c_uint;
pub type C2RustUnnamed_16 = libc::c_uint;
pub type C2RustUnnamed_17 = libc::c_uint;
pub const BC_CONTINUE: C2RustUnnamed_17 = 2;
pub type C2RustUnnamed_18 = libc::c_uint;
pub const NUM_OPT_O: C2RustUnnamed_18 = 4;
/* Basic theory of signal handling in shell
 * ========================================
 * This does not describe what hush does, rather, it is current understanding
 * what it _should_ do. If it doesn't, it's a bug.
 * http://www.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html#trap
 *
 * Signals are handled only after each pipe ("cmd | cmd | cmd" thing)
 * is finished or backgrounded. It is the same in interactive and
 * non-interactive shells, and is the same regardless of whether
 * a user trap handler is installed or a shell special one is in effect.
 * ^C or ^Z from keyboard seems to execute "at once" because it usually
 * backgrounds (i.e. stops) or kills all members of currently running
 * pipe.
 *
 * Wait builtin is interruptible by signals for which user trap is set
 * or by SIGINT in interactive shell.
 *
 * Trap handlers will execute even within trap handlers. (right?)
 *
 * User trap handlers are forgotten when subshell ("(cmd)") is entered,
 * except for handlers set to '' (empty string).
 *
 * If job control is off, backgrounded commands ("cmd &")
 * have SIGINT, SIGQUIT set to SIG_IGN.
 *
 * Commands which are run in command substitution ("`cmd`")
 * have SIGTTIN, SIGTTOU, SIGTSTP set to SIG_IGN.
 *
 * Ordinary commands have signals set to SIG_IGN/DFL as inherited
 * by the shell from its parent.
 *
 * Signals which differ from SIG_DFL action
 * (note: child (i.e., [v]forked) shell is not an interactive shell):
 *
 * SIGQUIT: ignore
 * SIGTERM (interactive): ignore
 * SIGHUP (interactive):
 *    send SIGCONT to stopped jobs, send SIGHUP to all jobs and exit
 * SIGTTIN, SIGTTOU, SIGTSTP (if job control is on): ignore
 *    Note that ^Z is handled not by trapping SIGTSTP, but by seeing
 *    that all pipe members are stopped. Try this in bash:
 *    while :; do :; done - ^Z does not background it
 *    (while :; do :; done) - ^Z backgrounds it
 * SIGINT (interactive): wait for last pipe, ignore the rest
 *    of the command line, show prompt. NB: ^C does not send SIGINT
 *    to interactive shell while shell is waiting for a pipe,
 *    since shell is bg'ed (is not in foreground process group).
 *    Example 1: this waits 5 sec, but does not execute ls:
 *    "echo $$; sleep 5; ls -l" + "kill -INT <pid>"
 *    Example 2: this does not wait and does not execute ls:
 *    "echo $$; sleep 5 & wait; ls -l" + "kill -INT <pid>"
 *    Example 3: this does not wait 5 sec, but executes ls:
 *    "sleep 5; ls -l" + press ^C
 *    Example 4: this does not wait and does not execute ls:
 *    "sleep 5 & wait; ls -l" + press ^C
 *
 * (What happens to signals which are IGN on shell start?)
 * (What happens with signal mask on shell start?)
 *
 * Old implementation
 * ==================
 * We use in-kernel pending signal mask to determine which signals were sent.
 * We block all signals which we don't want to take action immediately,
 * i.e. we block all signals which need to have special handling as described
 * above, and all signals which have traps set.
 * After each pipe execution, we extract any pending signals via sigtimedwait()
 * and act on them.
 *
 * unsigned special_sig_mask: a mask of such "special" signals
 * sigset_t blocked_set:  current blocked signal set
 *
 * "trap - SIGxxx":
 *    clear bit in blocked_set unless it is also in special_sig_mask
 * "trap 'cmd' SIGxxx":
 *    set bit in blocked_set (even if 'cmd' is '')
 * after [v]fork, if we plan to be a shell:
 *    unblock signals with special interactive handling
 *    (child shell is not interactive),
 *    unset all traps except '' (note: regardless of child shell's type - {}, (), etc)
 * after [v]fork, if we plan to exec:
 *    POSIX says fork clears pending signal mask in child - no need to clear it.
 *    Restore blocked signal set to one inherited by shell just prior to exec.
 *
 * Note: as a result, we do not use signal handlers much. The only uses
 * are to count SIGCHLDs
 * and to restore tty pgrp on signal-induced exit.
 *
 * Note 2 (compat):
 * Standard says "When a subshell is entered, traps that are not being ignored
 * are set to the default actions". bash interprets it so that traps which
 * are set to '' (ignore) are NOT reset to defaults. We do the same.
 *
 * Problem: the above approach makes it unwieldy to catch signals while
 * we are in read builtin, or while we read commands from stdin:
 * masked signals are not visible!
 *
 * New implementation
 * ==================
 * We record each signal we are interested in by installing signal handler
 * for them - a bit like emulating kernel pending signal mask in userspace.
 * We are interested in: signals which need to have special handling
 * as described above, and all signals which have traps set.
 * Signals are recorded in pending_set.
 * After each pipe execution, we extract any pending signals
 * and act on them.
 *
 * unsigned special_sig_mask: a mask of shell-special signals.
 * unsigned fatal_sig_mask: a mask of signals on which we restore tty pgrp.
 * char *traps[sig] if trap for sig is set (even if it's '').
 * sigset_t pending_set: set of sigs we received.
 *
 * "trap - SIGxxx":
 *    if sig is in special_sig_mask, set handler back to:
 *        record_pending_signo, or to IGN if it's a tty stop signal
 *    if sig is in fatal_sig_mask, set handler back to sigexit.
 *    else: set handler back to SIG_DFL
 * "trap 'cmd' SIGxxx":
 *    set handler to record_pending_signo.
 * "trap '' SIGxxx":
 *    set handler to SIG_IGN.
 * after [v]fork, if we plan to be a shell:
 *    set signals with special interactive handling to SIG_DFL
 *    (because child shell is not interactive),
 *    unset all traps except '' (note: regardless of child shell's type - {}, (), etc)
 * after [v]fork, if we plan to exec:
 *    POSIX says fork clears pending signal mask in child - no need to clear it.
 *
 * To make wait builtin interruptible, we handle SIGCHLD as special signal,
 * otherwise (if we leave it SIG_DFL) sigsuspend in wait builtin will not wake up on it.
 *
 * Note (compat):
 * Standard says "When a subshell is entered, traps that are not being ignored
 * are set to the default actions". bash interprets it so that traps which
 * are set to '' (ignore) are NOT reset to defaults. We do the same.
 */
pub type C2RustUnnamed_19 = libc::c_uint;
pub type C2RustUnnamed_20 = libc::c_uint;
pub const FLAG_UNTIL: C2RustUnnamed_20 = 256;
pub const FLAG_WHILE: C2RustUnnamed_20 = 128;
pub const FLAG_FOR: C2RustUnnamed_20 = 64;
pub const FLAG_IF: C2RustUnnamed_20 = 2;
#[inline(always)]
unsafe extern "C" fn not_const_pp(mut p: *const libc::c_void) -> *mut libc::c_void {
  return p as *mut libc::c_void;
}
/* We redefine ctype macros. Unicode-correct handling of char types
 * can't be done with such byte-oriented operations anyway,
 * we don't lose anything.
 */
/* We save ~500 bytes on isdigit alone.
 * BTW, x86 likes (unsigned char) cast more than (unsigned). */
/* These work the same for ASCII and Unicode,
 * assuming no one asks "is this a *Unicode* letter?" using isalpha(letter) */
/* In POSIX/C locale isspace is only these chars: "\t\n\v\f\r" and space.
 * "\t\n\v\f\r" happen to have ASCII codes 9,10,11,12,13.
 */
// Unsafe wrt NUL: #define ispunct(a) (strchr("!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~", (a)) != NULL)
// Bigger code: #define isalnum(a) ({ unsigned char bb__isalnum = (a) - '0'; bb__isalnum <= 9 || ((bb__isalnum - ('A' - '0')) & 0xdf) <= 25; })
#[inline(always)]
unsafe extern "C" fn bb_ascii_isalnum(mut a: libc::c_uchar) -> libc::c_int {
  let mut b: libc::c_uchar = (a as libc::c_int - '0' as i32) as libc::c_uchar; /* % */
  if b as libc::c_int <= 9i32 {
    return (b as libc::c_int <= 9i32) as libc::c_int;
  }
  b = ((a as libc::c_int | 0x20i32) - 'a' as i32) as libc::c_uchar;
  return (b as libc::c_int <= 'z' as i32 - 'a' as i32) as libc::c_int;
}
#[inline]
unsafe extern "C" fn pick_scan(mut op1: libc::c_char, mut op2: libc::c_char) -> libc::c_uint {
  let mut scan_flags: libc::c_uint = 0;
  if op1 as libc::c_int == '#' as i32 {
    scan_flags = (SCAN_MATCH_LEFT_HALF as libc::c_int
      + (if op1 as libc::c_int == op2 as libc::c_int {
        SCAN_MOVE_FROM_RIGHT as libc::c_int
      } else {
        SCAN_MOVE_FROM_LEFT as libc::c_int
      })) as libc::c_uint
  } else {
    scan_flags = (SCAN_MATCH_RIGHT_HALF as libc::c_int
      + (if op1 as libc::c_int == op2 as libc::c_int {
        SCAN_MOVE_FROM_LEFT as libc::c_int
      } else {
        SCAN_MOVE_FROM_RIGHT as libc::c_int
      })) as libc::c_uint
  }
  return scan_flags;
}
static mut hush_version_str: [libc::c_char; 24] = [
  72, 85, 83, 72, 95, 86, 69, 82, 83, 73, 79, 78, 61, 49, 46, 51, 50, 46, 48, 46, 103, 105, 116, 0,
];
static mut redir_table: [C2RustUnnamed_10; 5] = [
  {
    let mut init = C2RustUnnamed_10 {
      mode: 0i32,
      default_fd: 0i32 as libc::c_schar,
      descrip: [60, 0, 0],
    };
    init
  },
  {
    let mut init = C2RustUnnamed_10 {
      mode: 0o100i32 | 0o1000i32 | 0o1i32,
      default_fd: 1i32 as libc::c_schar,
      descrip: [62, 0, 0],
    };
    init
  },
  {
    let mut init = C2RustUnnamed_10 {
      mode: 0o100i32 | 0o2000i32 | 0o1i32,
      default_fd: 1i32 as libc::c_schar,
      descrip: [62, 62, 0],
    };
    init
  },
  {
    let mut init = C2RustUnnamed_10 {
      mode: 0o100i32 | 0o2i32,
      default_fd: 1i32 as libc::c_schar,
      descrip: [60, 62, 0],
    };
    init
  },
  {
    let mut init = C2RustUnnamed_10 {
      mode: 0i32,
      default_fd: 0i32 as libc::c_schar,
      descrip: [60, 60, 0],
    };
    init
  },
];
static mut o_opt_strings: [libc::c_char; 32] = [
  112, 105, 112, 101, 102, 97, 105, 108, 0, 110, 111, 101, 120, 101, 99, 0, 101, 114, 114, 101,
  120, 105, 116, 0, 120, 116, 114, 97, 99, 101, 0, 0,
];
static mut bltins1: [built_in_command; 31] = {
  [
    {
      let mut init = built_in_command {
        b_cmd: b".\x00" as *const u8 as *const libc::c_char,
        b_function: Some(
          builtin_source as unsafe extern "C" fn(_: *mut *mut libc::c_char) -> libc::c_int,
        ),
        b_descr: b"Run commands in file\x00" as *const u8 as *const libc::c_char,
      };
      init
    },
    {
      let mut init = built_in_command {
        b_cmd: b":\x00" as *const u8 as *const libc::c_char,
        b_function: Some(
          builtin_true as unsafe extern "C" fn(_: *mut *mut libc::c_char) -> libc::c_int,
        ),
        b_descr: 0 as *const libc::c_char,
      };
      init
    },
    {
      let mut init = built_in_command {
        b_cmd: b"bg\x00" as *const u8 as *const libc::c_char,
        b_function: Some(
          builtin_fg_bg as unsafe extern "C" fn(_: *mut *mut libc::c_char) -> libc::c_int,
        ),
        b_descr: b"Resume job in background\x00" as *const u8 as *const libc::c_char,
      };
      init
    },
    {
      let mut init = built_in_command {
        b_cmd: b"break\x00" as *const u8 as *const libc::c_char,
        b_function: Some(
          builtin_break as unsafe extern "C" fn(_: *mut *mut libc::c_char) -> libc::c_int,
        ),
        b_descr: b"Exit loop\x00" as *const u8 as *const libc::c_char,
      };
      init
    },
    {
      let mut init = built_in_command {
        b_cmd: b"cd\x00" as *const u8 as *const libc::c_char,
        b_function: Some(
          builtin_cd as unsafe extern "C" fn(_: *mut *mut libc::c_char) -> libc::c_int,
        ),
        b_descr: b"Change directory\x00" as *const u8 as *const libc::c_char,
      };
      init
    },
    {
      let mut init = built_in_command {
        b_cmd: b"continue\x00" as *const u8 as *const libc::c_char,
        b_function: Some(
          builtin_continue as unsafe extern "C" fn(_: *mut *mut libc::c_char) -> libc::c_int,
        ),
        b_descr: b"Start new loop iteration\x00" as *const u8 as *const libc::c_char,
      };
      init
    },
    {
      let mut init = built_in_command {
        b_cmd: b"eval\x00" as *const u8 as *const libc::c_char,
        b_function: Some(
          builtin_eval as unsafe extern "C" fn(_: *mut *mut libc::c_char) -> libc::c_int,
        ),
        b_descr: b"Construct and run shell command\x00" as *const u8 as *const libc::c_char,
      };
      init
    },
    {
      let mut init = built_in_command {
        b_cmd: b"exec\x00" as *const u8 as *const libc::c_char,
        b_function: Some(
          builtin_exec as unsafe extern "C" fn(_: *mut *mut libc::c_char) -> libc::c_int,
        ),
        b_descr: b"Execute command, don\'t return to shell\x00" as *const u8 as *const libc::c_char,
      };
      init
    },
    {
      let mut init = built_in_command {
        b_cmd: b"exit\x00" as *const u8 as *const libc::c_char,
        b_function: Some(
          builtin_exit as unsafe extern "C" fn(_: *mut *mut libc::c_char) -> libc::c_int,
        ),
        b_descr: 0 as *const libc::c_char,
      };
      init
    },
    {
      let mut init = built_in_command {
        b_cmd: b"export\x00" as *const u8 as *const libc::c_char,
        b_function: Some(
          builtin_export as unsafe extern "C" fn(_: *mut *mut libc::c_char) -> libc::c_int,
        ),
        b_descr: b"Set environment variables\x00" as *const u8 as *const libc::c_char,
      };
      init
    },
    {
      let mut init = built_in_command {
        b_cmd: b"fg\x00" as *const u8 as *const libc::c_char,
        b_function: Some(
          builtin_fg_bg as unsafe extern "C" fn(_: *mut *mut libc::c_char) -> libc::c_int,
        ),
        b_descr: b"Bring job to foreground\x00" as *const u8 as *const libc::c_char,
      };
      init
    },
    {
      let mut init = built_in_command {
        b_cmd: b"getopts\x00" as *const u8 as *const libc::c_char,
        b_function: Some(
          builtin_getopts as unsafe extern "C" fn(_: *mut *mut libc::c_char) -> libc::c_int,
        ),
        b_descr: 0 as *const libc::c_char,
      };
      init
    },
    {
      let mut init = built_in_command {
        b_cmd: b"help\x00" as *const u8 as *const libc::c_char,
        b_function: Some(
          builtin_help as unsafe extern "C" fn(_: *mut *mut libc::c_char) -> libc::c_int,
        ),
        b_descr: 0 as *const libc::c_char,
      };
      init
    },
    {
      let mut init = built_in_command {
        b_cmd: b"history\x00" as *const u8 as *const libc::c_char,
        b_function: Some(
          builtin_history as unsafe extern "C" fn(_: *mut *mut libc::c_char) -> libc::c_int,
        ),
        b_descr: b"Show history\x00" as *const u8 as *const libc::c_char,
      };
      init
    },
    {
      let mut init = built_in_command {
        b_cmd: b"jobs\x00" as *const u8 as *const libc::c_char,
        b_function: Some(
          builtin_jobs as unsafe extern "C" fn(_: *mut *mut libc::c_char) -> libc::c_int,
        ),
        b_descr: b"List jobs\x00" as *const u8 as *const libc::c_char,
      };
      init
    },
    {
      let mut init = built_in_command {
        b_cmd: b"kill\x00" as *const u8 as *const libc::c_char,
        b_function: Some(
          builtin_kill as unsafe extern "C" fn(_: *mut *mut libc::c_char) -> libc::c_int,
        ),
        b_descr: b"Send signals to processes\x00" as *const u8 as *const libc::c_char,
      };
      init
    },
    {
      let mut init = built_in_command {
        b_cmd: b"local\x00" as *const u8 as *const libc::c_char,
        b_function: Some(
          builtin_local as unsafe extern "C" fn(_: *mut *mut libc::c_char) -> libc::c_int,
        ),
        b_descr: b"Set local variables\x00" as *const u8 as *const libc::c_char,
      };
      init
    },
    {
      let mut init = built_in_command {
        b_cmd: b"read\x00" as *const u8 as *const libc::c_char,
        b_function: Some(
          builtin_read as unsafe extern "C" fn(_: *mut *mut libc::c_char) -> libc::c_int,
        ),
        b_descr: b"Input into variable\x00" as *const u8 as *const libc::c_char,
      };
      init
    },
    {
      let mut init = built_in_command {
        b_cmd: b"readonly\x00" as *const u8 as *const libc::c_char,
        b_function: Some(
          builtin_readonly as unsafe extern "C" fn(_: *mut *mut libc::c_char) -> libc::c_int,
        ),
        b_descr: b"Make variables read-only\x00" as *const u8 as *const libc::c_char,
      };
      init
    },
    {
      let mut init = built_in_command {
        b_cmd: b"return\x00" as *const u8 as *const libc::c_char,
        b_function: Some(
          builtin_return as unsafe extern "C" fn(_: *mut *mut libc::c_char) -> libc::c_int,
        ),
        b_descr: b"Return from function\x00" as *const u8 as *const libc::c_char,
      };
      init
    },
    {
      let mut init = built_in_command {
        b_cmd: b"set\x00" as *const u8 as *const libc::c_char,
        b_function: Some(
          builtin_set as unsafe extern "C" fn(_: *mut *mut libc::c_char) -> libc::c_int,
        ),
        b_descr: b"Set positional parameters\x00" as *const u8 as *const libc::c_char,
      };
      init
    },
    {
      let mut init = built_in_command {
        b_cmd: b"shift\x00" as *const u8 as *const libc::c_char,
        b_function: Some(
          builtin_shift as unsafe extern "C" fn(_: *mut *mut libc::c_char) -> libc::c_int,
        ),
        b_descr: b"Shift positional parameters\x00" as *const u8 as *const libc::c_char,
      };
      init
    },
    {
      let mut init = built_in_command {
        b_cmd: b"source\x00" as *const u8 as *const libc::c_char,
        b_function: Some(
          builtin_source as unsafe extern "C" fn(_: *mut *mut libc::c_char) -> libc::c_int,
        ),
        b_descr: 0 as *const libc::c_char,
      };
      init
    },
    {
      let mut init = built_in_command {
        b_cmd: b"times\x00" as *const u8 as *const libc::c_char,
        b_function: Some(
          builtin_times as unsafe extern "C" fn(_: *mut *mut libc::c_char) -> libc::c_int,
        ),
        b_descr: 0 as *const libc::c_char,
      };
      init
    },
    {
      let mut init = built_in_command {
        b_cmd: b"trap\x00" as *const u8 as *const libc::c_char,
        b_function: Some(
          builtin_trap as unsafe extern "C" fn(_: *mut *mut libc::c_char) -> libc::c_int,
        ),
        b_descr: b"Trap signals\x00" as *const u8 as *const libc::c_char,
      };
      init
    },
    {
      let mut init = built_in_command {
        b_cmd: b"true\x00" as *const u8 as *const libc::c_char,
        b_function: Some(
          builtin_true as unsafe extern "C" fn(_: *mut *mut libc::c_char) -> libc::c_int,
        ),
        b_descr: 0 as *const libc::c_char,
      };
      init
    },
    {
      let mut init = built_in_command {
        b_cmd: b"type\x00" as *const u8 as *const libc::c_char,
        b_function: Some(
          builtin_type as unsafe extern "C" fn(_: *mut *mut libc::c_char) -> libc::c_int,
        ),
        b_descr: b"Show command type\x00" as *const u8 as *const libc::c_char,
      };
      init
    },
    {
      let mut init = built_in_command {
        b_cmd: b"ulimit\x00" as *const u8 as *const libc::c_char,
        b_function: Some(
          shell_builtin_ulimit as unsafe extern "C" fn(_: *mut *mut libc::c_char) -> libc::c_int,
        ),
        b_descr: b"Control resource limits\x00" as *const u8 as *const libc::c_char,
      };
      init
    },
    {
      let mut init = built_in_command {
        b_cmd: b"umask\x00" as *const u8 as *const libc::c_char,
        b_function: Some(
          builtin_umask as unsafe extern "C" fn(_: *mut *mut libc::c_char) -> libc::c_int,
        ),
        b_descr: b"Set file creation mask\x00" as *const u8 as *const libc::c_char,
      };
      init
    },
    {
      let mut init = built_in_command {
        b_cmd: b"unset\x00" as *const u8 as *const libc::c_char,
        b_function: Some(
          builtin_unset as unsafe extern "C" fn(_: *mut *mut libc::c_char) -> libc::c_int,
        ),
        b_descr: b"Unset variables\x00" as *const u8 as *const libc::c_char,
      };
      init
    },
    {
      let mut init = built_in_command {
        b_cmd: b"wait\x00" as *const u8 as *const libc::c_char,
        b_function: Some(
          builtin_wait as unsafe extern "C" fn(_: *mut *mut libc::c_char) -> libc::c_int,
        ),
        b_descr: b"Wait for process to finish\x00" as *const u8 as *const libc::c_char,
      };
      init
    },
  ]
};
/* These builtins won't be used if we are on NOMMU and need to re-exec
 * (it's cheaper to run an external program in this case):
 */
static mut bltins2: [built_in_command; 6] = {
  [
    {
      let mut init = built_in_command {
        b_cmd: b"[\x00" as *const u8 as *const libc::c_char,
        b_function: Some(
          builtin_test as unsafe extern "C" fn(_: *mut *mut libc::c_char) -> libc::c_int,
        ),
        b_descr: 0 as *const libc::c_char,
      };
      init
    },
    {
      let mut init = built_in_command {
        b_cmd: b"[[\x00" as *const u8 as *const libc::c_char,
        b_function: Some(
          builtin_test as unsafe extern "C" fn(_: *mut *mut libc::c_char) -> libc::c_int,
        ),
        b_descr: 0 as *const libc::c_char,
      };
      init
    },
    {
      let mut init = built_in_command {
        b_cmd: b"echo\x00" as *const u8 as *const libc::c_char,
        b_function: Some(
          builtin_echo as unsafe extern "C" fn(_: *mut *mut libc::c_char) -> libc::c_int,
        ),
        b_descr: 0 as *const libc::c_char,
      };
      init
    },
    {
      let mut init = built_in_command {
        b_cmd: b"printf\x00" as *const u8 as *const libc::c_char,
        b_function: Some(
          builtin_printf as unsafe extern "C" fn(_: *mut *mut libc::c_char) -> libc::c_int,
        ),
        b_descr: 0 as *const libc::c_char,
      };
      init
    },
    {
      let mut init = built_in_command {
        b_cmd: b"pwd\x00" as *const u8 as *const libc::c_char,
        b_function: Some(
          builtin_pwd as unsafe extern "C" fn(_: *mut *mut libc::c_char) -> libc::c_int,
        ),
        b_descr: 0 as *const libc::c_char,
      };
      init
    },
    {
      let mut init = built_in_command {
        b_cmd: b"test\x00" as *const u8 as *const libc::c_char,
        b_function: Some(
          builtin_test as unsafe extern "C" fn(_: *mut *mut libc::c_char) -> libc::c_int,
        ),
        b_descr: 0 as *const libc::c_char,
      };
      init
    },
  ]
};
/* Leak hunting. Use hush_leaktool.sh for post-processing.
 */
/* Syntax and runtime errors. They always abort scripts.
 * In interactive use they usually discard unparsed and/or unexecuted commands
 * and return to the prompt.
 * HUSH_DEBUG >= 2 prints line number in this file where it was detected.
 */
unsafe extern "C" fn die_if_script() {
  if (*ptr_to_globals).interactive_fd == 0 {
    if (*ptr_to_globals).last_exitcode != 0 {
      /* sometines it's 2, not 1 (bash compat) */
      xfunc_error_retval = (*ptr_to_globals).last_exitcode
    }
    xfunc_die();
  };
}
unsafe extern "C" fn msg_and_die_if_script(mut fmt: *const libc::c_char, mut args: ...) {
  let mut p: ::std::ffi::VaListImpl;
  p = args.clone();
  bb_verror_msg(fmt, p.as_va_list(), 0 as *const libc::c_char);
  die_if_script();
}
unsafe extern "C" fn syntax_error(mut msg: *const libc::c_char) {
  if !msg.is_null() {
    bb_error_msg(
      b"syntax error: %s\x00" as *const u8 as *const libc::c_char,
      msg,
    );
  } else {
    bb_simple_error_msg(b"syntax error\x00" as *const u8 as *const libc::c_char);
  }
  die_if_script();
}
unsafe extern "C" fn syntax_error_at(mut msg: *const libc::c_char) {
  bb_error_msg(
    b"syntax error at \'%s\'\x00" as *const u8 as *const libc::c_char,
    msg,
  );
  die_if_script();
}
unsafe extern "C" fn syntax_error_unterm_str(mut s: *const libc::c_char) {
  bb_error_msg(
    b"syntax error: unterminated %s\x00" as *const u8 as *const libc::c_char,
    s,
  );
  //? source4.tests fails: in bash, echo ${^} in script does not terminate the script
  //	die_if_script();
}
unsafe extern "C" fn syntax_error_unterm_ch(mut ch: libc::c_char) {
  let mut msg: [libc::c_char; 2] = [ch, '\u{0}' as i32 as libc::c_char];
  syntax_error_unterm_str(msg.as_mut_ptr());
}
unsafe extern "C" fn syntax_error_unexpected_ch(mut ch: libc::c_int) {
  let mut msg: [libc::c_char; 2] = [0; 2];
  msg[0] = ch as libc::c_char;
  msg[1] = '\u{0}' as i32 as libc::c_char;
  bb_error_msg(
    b"syntax error: unexpected %s\x00" as *const u8 as *const libc::c_char,
    if ch == -1i32 {
      b"EOF\x00" as *const u8 as *const libc::c_char
    } else {
      msg.as_mut_ptr()
    },
  );
  die_if_script();
}
/* Utility functions
 */
/* Replace each \x with x in place, return ptr past NUL. */
unsafe extern "C" fn unbackslash(mut src: *mut libc::c_char) -> *mut libc::c_char {
  src = strchrnul(src, '\\' as i32);
  let mut dst: *mut libc::c_char = src;
  loop {
    if *src as libc::c_int == '\\' as i32 {
      src = src.offset(1);
      if *src as libc::c_int != '\u{0}' as i32 {
        /* fallthrough */
        /* \x -> x */
        let fresh0 = src;
        src = src.offset(1);
        let fresh1 = dst;
        dst = dst.offset(1);
        *fresh1 = *fresh0;
        continue;
      } else {
        /* else: "\<nul>". Do not delete this backslash.
         * Testcase: eval 'echo ok\'
         */
        let fresh2 = dst;
        dst = dst.offset(1);
        *fresh2 = '\\' as i32 as libc::c_char
      }
    }
    let fresh3 = src;
    src = src.offset(1);
    let fresh4 = dst;
    dst = dst.offset(1);
    *fresh4 = *fresh3;
    if *fresh4 as libc::c_int == '\u{0}' as i32 {
      break;
    }
  }
  return dst;
}
unsafe extern "C" fn add_strings_to_strings(
  mut strings: *mut *mut libc::c_char,
  mut add: *mut *mut libc::c_char,
  mut need_to_dup: libc::c_int,
) -> *mut *mut libc::c_char {
  let mut i: libc::c_int = 0;
  let mut count1: libc::c_uint = 0;
  let mut count2: libc::c_uint = 0;
  let mut v: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  v = strings;
  count1 = 0i32 as libc::c_uint;
  if !v.is_null() {
    while !(*v).is_null() {
      count1 = count1.wrapping_add(1);
      v = v.offset(1)
    }
  }
  count2 = 0i32 as libc::c_uint;
  v = add;
  while !(*v).is_null() {
    count2 = count2.wrapping_add(1);
    v = v.offset(1)
  }
  v = xrealloc(
    strings as *mut libc::c_void,
    (count1
      .wrapping_add(count2)
      .wrapping_add(1i32 as libc::c_uint) as libc::c_ulong)
      .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
  ) as *mut *mut libc::c_char;
  let ref mut fresh5 = *v.offset(count1.wrapping_add(count2) as isize);
  *fresh5 = 0 as *mut libc::c_char;
  i = count2 as libc::c_int;
  loop {
    i -= 1;
    if !(i >= 0i32) {
      break;
    }
    let ref mut fresh6 = *v.offset(count1.wrapping_add(i as libc::c_uint) as isize);
    *fresh6 = if need_to_dup != 0 {
      xstrdup(*add.offset(i as isize))
    } else {
      *add.offset(i as isize)
    }
  }
  return v;
}
/* Note: takes ownership of "add" ptr (it is not strdup'ed) */
unsafe extern "C" fn add_string_to_strings(
  mut strings: *mut *mut libc::c_char,
  mut add: *mut libc::c_char,
) -> *mut *mut libc::c_char {
  let mut v: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2]; /* newfd < 0 */
  v[0] = add;
  v[1] = 0 as *mut libc::c_char;
  return add_strings_to_strings(strings, v.as_mut_ptr(), 0i32);
}
unsafe extern "C" fn free_strings(mut strings: *mut *mut libc::c_char) {
  let mut v: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  if strings.is_null() {
    return;
  }
  v = strings;
  while !(*v).is_null() {
    free(*v as *mut libc::c_void);
    v = v.offset(1)
  }
  free(strings as *mut libc::c_void);
}
unsafe extern "C" fn dup_CLOEXEC(mut fd: libc::c_int, mut avoid_fd: libc::c_int) -> libc::c_int {
  let mut newfd: libc::c_int = 0;
  loop {
    newfd = fcntl(fd, 1030i32, avoid_fd + 1i32);
    if newfd >= 0i32 {
      if 1030i32 == 0i32 {
        /* if old libc (w/o F_DUPFD_CLOEXEC) */
        fcntl(newfd, 2i32, 1i32);
      }
      break;
    } else {
      if *bb_errno == 16i32 {
        continue;
      }
      if !(*bb_errno == 4i32) {
        break;
      }
    }
  }
  return newfd;
}
unsafe extern "C" fn xdup_CLOEXEC_and_close(
  mut fd: libc::c_int,
  mut avoid_fd: libc::c_int,
) -> libc::c_int {
  let mut newfd: libc::c_int = 0;
  loop {
    newfd = fcntl(fd, 1030i32, avoid_fd + 1i32);
    if newfd < 0i32 {
      if *bb_errno == 16i32 {
        continue;
      }
      if *bb_errno == 4i32 {
        continue;
      }
      /* fd was not open? */
      if *bb_errno == 9i32 {
        return fd;
      }
      xfunc_die();
    } else {
      if 1030i32 == 0i32 {
        /* if old libc (w/o F_DUPFD_CLOEXEC) */
        fcntl(newfd, 2i32, 1i32);
      }
      close(fd);
      return newfd;
    }
  }
}
/* Manipulating HFILEs */
unsafe extern "C" fn hfopen(mut name: *const libc::c_char) -> *mut HFILE {
  let mut fp: *mut HFILE = 0 as *mut HFILE;
  let mut fd: libc::c_int = 0;
  fd = 0i32;
  if !name.is_null() {
    fd = open(name, 0i32 | 0o2000000i32);
    if fd < 0i32 {
      return 0 as *mut HFILE;
    }
    if 0o2000000i32 == 0i32 {
      /* ancient libc */
      close_on_exec_on(fd);
    }
  }
  fp = xmalloc(::std::mem::size_of::<HFILE>() as libc::c_ulong) as *mut HFILE;
  (*fp).is_stdin = (name == 0 as *mut libc::c_void as *const libc::c_char) as libc::c_int;
  (*fp).fd = fd;
  (*fp).end = (*fp).buf.as_mut_ptr();
  (*fp).cur = (*fp).end;
  (*fp).next_hfile = (*ptr_to_globals).HFILE_list;
  (*ptr_to_globals).HFILE_list = fp;
  return fp;
}
unsafe extern "C" fn hfclose(mut fp: *mut HFILE) {
  let mut pp: *mut *mut HFILE = &mut (*ptr_to_globals).HFILE_list;
  while !(*pp).is_null() {
    let mut cur: *mut HFILE = *pp;
    if cur == fp {
      *pp = (*cur).next_hfile;
      break;
    } else {
      pp = &mut (*cur).next_hfile
    }
  }
  if (*fp).fd >= 0i32 {
    close((*fp).fd);
  }
  free(fp as *mut libc::c_void);
}
unsafe extern "C" fn refill_HFILE_and_getc(mut fp: *mut HFILE) -> libc::c_int {
  let mut n: libc::c_int = 0;
  if (*fp).fd < 0i32 {
    /* Already saw EOF */
    return -1i32;
  }
  /* Try to buffer more input */
  (*fp).cur = (*fp).buf.as_mut_ptr();
  n = safe_read(
    (*fp).fd,
    (*fp).buf.as_mut_ptr() as *mut libc::c_void,
    ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
  ) as libc::c_int;
  if n < 0i32 {
    bb_simple_perror_msg(b"read error\x00" as *const u8 as *const libc::c_char);
    n = 0i32
  }
  (*fp).end = (*fp).buf.as_mut_ptr().offset(n as isize);
  if n == 0i32 {
    /* EOF/error */
    close((*fp).fd);
    (*fp).fd = -1i32;
    return -1i32;
  }
  let fresh7 = (*fp).cur;
  (*fp).cur = (*fp).cur.offset(1);
  return *fresh7 as libc::c_uchar as libc::c_int;
}
/* Inlined for common case of non-empty buffer.
 */
#[inline(always)]
unsafe extern "C" fn hfgetc(mut fp: *mut HFILE) -> libc::c_int {
  if (*fp).cur < (*fp).end {
    let fresh8 = (*fp).cur;
    (*fp).cur = (*fp).cur.offset(1);
    return *fresh8 as libc::c_uchar as libc::c_int;
  }
  /* Buffer empty */
  return refill_HFILE_and_getc(fp);
}
unsafe extern "C" fn move_HFILEs_on_redirect(
  mut fd: libc::c_int,
  mut avoid_fd: libc::c_int,
) -> libc::c_int {
  let mut fl: *mut HFILE = (*ptr_to_globals).HFILE_list;
  while !fl.is_null() {
    if fd == (*fl).fd {
      /* We use it only on script files, they are all CLOEXEC */
      (*fl).fd = xdup_CLOEXEC_and_close(fd, avoid_fd);
      return 1i32;
      /* "found and moved" */
    }
    fl = (*fl).next_hfile
  }
  if (*ptr_to_globals).x_mode_fd > 0i32 && fd == (*ptr_to_globals).x_mode_fd {
    (*ptr_to_globals).x_mode_fd = xdup_CLOEXEC_and_close(fd, avoid_fd);
    return 1i32;
    /* "found and moved" */
  }
  return 0i32;
  /* "not in the list" */
}
unsafe extern "C" fn fd_in_HFILEs(mut fd: libc::c_int) -> libc::c_int {
  let mut fl: *mut HFILE = (*ptr_to_globals).HFILE_list; /* retain $0 */
  while !fl.is_null() {
    if (*fl).fd == fd {
      return 1i32;
    }
    fl = (*fl).next_hfile
  }
  return 0i32;
}
unsafe extern "C" fn save_and_replace_G_args(
  mut sv: *mut save_arg_t,
  mut argv: *mut *mut libc::c_char,
) {
  (*sv).sv_argv0 = *argv.offset(0);
  (*sv).sv_g_argv = (*ptr_to_globals).global_argv;
  (*sv).sv_g_argc = (*ptr_to_globals).global_argc;
  (*sv).sv_g_malloced = (*ptr_to_globals).global_args_malloced as smallint;
  let ref mut fresh9 = *argv.offset(0);
  *fresh9 = *(*ptr_to_globals).global_argv.offset(0);
  (*ptr_to_globals).global_argv = argv;
  (*ptr_to_globals).global_args_malloced = 0i32 as smalluint;
  (*ptr_to_globals).global_argc =
    (1i32 as libc::c_uint).wrapping_add(string_array_len(argv.offset(1))) as libc::c_int;
}
unsafe extern "C" fn restore_G_args(mut sv: *mut save_arg_t, mut argv: *mut *mut libc::c_char) {
  if (*ptr_to_globals).global_args_malloced != 0 {
    /* someone ran "set -- arg1 arg2 ...", undo */
    let mut pp: *mut *mut libc::c_char = (*ptr_to_globals).global_argv;
    loop {
      pp = pp.offset(1);
      if (*pp).is_null() {
        break;
      }
      /* note: does not free $0 */
      free(*pp as *mut libc::c_void);
    }
    free((*ptr_to_globals).global_argv as *mut libc::c_void);
  }
  let ref mut fresh10 = *argv.offset(0);
  *fresh10 = (*sv).sv_argv0;
  (*ptr_to_globals).global_argv = (*sv).sv_g_argv;
  (*ptr_to_globals).global_argc = (*sv).sv_g_argc;
  (*ptr_to_globals).global_args_malloced = (*sv).sv_g_malloced as smalluint;
}
unsafe extern "C" fn record_pending_signo(mut sig: libc::c_int) {
  sigaddset(&mut (*ptr_to_globals).pending_set, sig);
}
unsafe extern "C" fn install_sighandler(
  mut sig: libc::c_int,
  mut handler: sighandler_t,
) -> sighandler_t {
  let mut old_sa: sigaction = sigaction {
    __sigaction_handler: C2RustUnnamed_9 { sa_handler: None },
    sa_mask: __sigset_t { __val: [0; 16] },
    sa_flags: 0,
    sa_restorer: None,
  };
  /* We could use signal() to install handlers... almost:
   * except that we need to mask ALL signals while handlers run.
   * I saw signal nesting in strace, race window isn't small.
   * SA_RESTART is also needed, but in Linux, signal()
   * sets SA_RESTART too.
   */
  /* memset(&G.sa, 0, sizeof(G.sa)); - already done */
  /* sigfillset(&G.sa.sa_mask);      - already done */
  /* G.sa.sa_flags = SA_RESTART;     - already done */
  (*ptr_to_globals).sa.__sigaction_handler.sa_handler = handler;
  sigaction(sig, &mut (*ptr_to_globals).sa, &mut old_sa);
  return old_sa.__sigaction_handler.sa_handler;
}
unsafe extern "C" fn restore_ttypgrp_and__exit() -> ! {
  /* xfunc has failed! die die die */
  /* no EXIT traps, this is an escape hatch! */
  (*ptr_to_globals).exiting = 1i32 as smallint;
  hush_exit(xfunc_error_retval as libc::c_int);
}
/* Needed only on some libc:
 * It was observed that on exit(), fgetc'ed buffered data
 * gets "unwound" via lseek(fd, -NUM, SEEK_CUR).
 * With the net effect that even after fork(), not vfork(),
 * exit() in NOEXECed applet in "sh SCRIPT":
 *	noexec_applet_here
 *	echo END_OF_SCRIPT
 * lseeks fd in input FILE object from EOF to "e" in "echo END_OF_SCRIPT".
 * This makes "echo END_OF_SCRIPT" executed twice.
 * Similar problems can be seen with msg_and_die_if_script() -> xfunc_die()
 * and in `cmd` handling.
 * If set as die_func(), this makes xfunc_die() exit via _exit(), not exit():
 */
unsafe extern "C" fn fflush_and__exit() -> ! {
  fflush_all();
  _exit(xfunc_error_retval as libc::c_int);
}
/* After [v]fork, in child: do not restore tty pgrp on xfunc death */
/* After [v]fork, in parent: restore tty pgrp on xfunc death */
/* Restores tty foreground process group, and exits.
 * May be called as signal handler for fatal signal
 * (will resend signal to itself, producing correct exit state)
 * or called directly with -EXITCODE.
 * We also call it if xfunc is exiting.
 */
unsafe extern "C" fn sigexit(mut sig: libc::c_int) -> ! {
  /* Careful: we can end up here after [v]fork. Do not restore
   * tty pgrp then, only top-level shell process does that */
  if (*ptr_to_globals).saved_tty_pgrp != 0 && getpid() == (*ptr_to_globals).root_pid {
    /* Disable all signals: job control, SIGPIPE, etc.
     * Mostly paranoid measure, to prevent infinite SIGTTOU.
     */
    sigprocmask_allsigs(0i32);
    tcsetpgrp(
      (*ptr_to_globals).interactive_fd,
      (*ptr_to_globals).saved_tty_pgrp,
    );
  }
  /* Not a signal, just exit */
  if sig <= 0i32 {
    _exit(-sig);
  }
  kill_myself_with_sig(sig);
  /* does not return */
}
unsafe extern "C" fn pick_sighandler(mut sig: libc::c_uint) -> sighandler_t {
  let mut handler: sighandler_t = None;
  if (sig as libc::c_ulong)
    < (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong).wrapping_mul(8i32 as libc::c_ulong)
  {
    let mut sigmask: libc::c_uint = (1i32 << sig) as libc::c_uint;
    /* is sig fatal? */
    if (*ptr_to_globals).fatal_sig_mask & sigmask != 0 {
      handler = ::std::mem::transmute::<
        Option<unsafe extern "C" fn(_: libc::c_int) -> !>,
        sighandler_t,
      >(Some(sigexit as unsafe extern "C" fn(_: libc::c_int) -> !))
    } else if (*ptr_to_globals).special_sig_mask & sigmask != 0 {
      handler = Some(record_pending_signo as unsafe extern "C" fn(_: libc::c_int) -> ());
      /* sig has special handling? */
      /* TTIN/TTOU/TSTP can't be set to record_pending_signo
       * in order to ignore them: they will be raised
       * in an endless loop when we try to do some
       * terminal ioctls! We do have to _ignore_ these.
       */
      if SPECIAL_JOBSTOP_SIGS as libc::c_int as libc::c_uint & sigmask != 0 {
        handler = ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(1i32 as libc::intptr_t)
      }
    }
  }
  return handler;
}
/* Restores tty foreground process group, and exits. */
unsafe extern "C" fn hush_exit(mut exitcode: libc::c_int) -> ! {
  fflush_all();
  if (*ptr_to_globals).exiting as libc::c_int <= 0i32
    && !(*ptr_to_globals).traps.is_null()
    && !(*(*ptr_to_globals).traps.offset(0)).is_null()
    && *(*(*ptr_to_globals).traps.offset(0)).offset(0) as libc::c_int != 0
  {
    let mut argv: [*mut libc::c_char; 3] = [0 as *mut libc::c_char; 3];
    /* argv[0] is unused */
    argv[1] = xstrdup(*(*ptr_to_globals).traps.offset(0)); /* copy, since EXIT trap handler may modify G_traps[0] */
    argv[2] = 0 as *mut libc::c_char; /* prevent EXIT trap recursion */
    (*ptr_to_globals).exiting = 1i32 as smallint;
    /* Note: G_traps[0] is not cleared!
     * "trap" will still show it, if executed
     * in the handler */
    builtin_eval(argv.as_mut_ptr());
  }
  fflush_all();
  sigexit(-(exitcode & 0xffi32));
}
//TODO: return a mask of ALL handled sigs?
unsafe extern "C" fn check_and_run_traps() -> libc::c_int {
  let mut last_sig: libc::c_int = 0i32; /* else: "" trap, ignoring signal */
  's_8: loop {
    let mut sig: libc::c_int = 0;
    if sigisemptyset(&mut (*ptr_to_globals).pending_set) != 0 {
      break;
    }
    sig = 0i32;
    loop {
      sig += 1;
      if sigismember(&mut (*ptr_to_globals).pending_set, sig) != 0 {
        sigdelset(&mut (*ptr_to_globals).pending_set, sig);
        break;
      } else if !(sig < 64i32 + 1i32) {
        break 's_8;
      }
    }
    if !(*ptr_to_globals).traps.is_null()
      && !(*(*ptr_to_globals).traps.offset(sig as isize)).is_null()
    {
      if *(*(*ptr_to_globals).traps.offset(sig as isize)).offset(0) != 0 {
        /* We have user-defined handler */
        let mut save_rcode: smalluint = 0;
        let mut argv: [*mut libc::c_char; 3] = [0 as *mut libc::c_char; 3];
        /* argv[0] is unused */
        argv[1] = xstrdup(*(*ptr_to_globals).traps.offset(sig as isize));
        /* why strdup? trap can modify itself: trap 'trap "echo oops" INT' INT */
        argv[2] = 0 as *mut libc::c_char;
        save_rcode = (*ptr_to_globals).last_exitcode;
        builtin_eval(argv.as_mut_ptr());
        free(argv[1] as *mut libc::c_void);
        //FIXME: shouldn't it be set to 128 + sig instead?
        (*ptr_to_globals).last_exitcode = save_rcode;
        last_sig = sig
      }
    } else {
      /* not a trap: special action */
      match sig {
        2 => {
          (*ptr_to_globals).flag_SIGINT = 1i32 as smallint;
          last_sig = sig
        }
        1 => {
          //TODO: why are we doing this? ash and dash don't do this,
          //they have no handler for SIGHUP at all,
          //they rely on kernel to send SIGHUP+SIGCONT to orphaned process groups
          let mut job: *mut pipe = 0 as *mut pipe;
          /* bash is observed to signal whole process groups,
           * not individual processes */
          job = (*ptr_to_globals).job_list;
          while !job.is_null() {
            if !((*job).pgrp <= 0i32) {
              if kill(-(*job).pgrp, 1i32) == 0i32 {
                kill(-(*job).pgrp, 18i32);
              }
            }
            job = (*job).next
          }
          sigexit(1i32);
        }
        _ => {}
      }
    }
  }
  return last_sig;
}
unsafe extern "C" fn get_cwd(mut force: libc::c_int) -> *const libc::c_char {
  if force != 0 || (*ptr_to_globals).cwd.is_null() {
    /* xrealloc_getcwd_or_warn(arg) calls free(arg),
     * we must not try to free(bb_msg_unknown) */
    if (*ptr_to_globals).cwd == bb_msg_unknown.as_ptr() {
      (*ptr_to_globals).cwd = 0 as *const libc::c_char
    }
    (*ptr_to_globals).cwd = xrealloc_getcwd_or_warn((*ptr_to_globals).cwd as *mut libc::c_char);
    if (*ptr_to_globals).cwd.is_null() {
      (*ptr_to_globals).cwd = bb_msg_unknown.as_ptr()
    }
  }
  return (*ptr_to_globals).cwd;
}
/*
 * Shell and environment variable support
 */
unsafe extern "C" fn get_ptr_to_local_var(
  mut name: *const libc::c_char,
  mut len: libc::c_uint,
) -> *mut *mut variable {
  let mut pp: *mut *mut variable = 0 as *mut *mut variable;
  let mut cur: *mut variable = 0 as *mut variable;
  pp = &mut (*ptr_to_globals).top_var;
  loop {
    cur = *pp;
    if cur.is_null() {
      break;
    }
    if strncmp((*cur).varstr, name, len as libc::c_ulong) == 0i32
      && *(*cur).varstr.offset(len as isize) as libc::c_int == '=' as i32
    {
      return pp;
    }
    pp = &mut (*cur).next
  }
  return 0 as *mut *mut variable;
}
unsafe extern "C" fn get_local_var_value(mut name: *const libc::c_char) -> *const libc::c_char {
  let mut vpp: *mut *mut variable = 0 as *mut *mut variable;
  let mut len: libc::c_uint = strlen(name) as libc::c_uint;
  if !(*ptr_to_globals).expanded_assignments.is_null() {
    let mut cpp: *mut *mut libc::c_char = (*ptr_to_globals).expanded_assignments;
    while !(*cpp).is_null() {
      let mut cp: *mut libc::c_char = *cpp;
      if strncmp(cp, name, len as libc::c_ulong) == 0i32
        && *cp.offset(len as isize) as libc::c_int == '=' as i32
      {
        return cp.offset(len as isize).offset(1);
      }
      cpp = cpp.offset(1)
    }
  }
  vpp = get_ptr_to_local_var(name, len);
  if !vpp.is_null() {
    return (**vpp).varstr.offset(len as isize).offset(1);
  }
  if strcmp(name, b"PPID\x00" as *const u8 as *const libc::c_char) == 0i32 {
    return utoa((*ptr_to_globals).root_ppid as libc::c_uint);
  }
  // bash compat: UID? EUID?
  if strcmp(name, b"RANDOM\x00" as *const u8 as *const libc::c_char) == 0i32 {
    return utoa(next_random(&mut (*ptr_to_globals).random_gen));
  } /* including '=' */
  if strcmp(name, b"LINENO\x00" as *const u8 as *const libc::c_char) == 0i32 {
    return utoa((*ptr_to_globals).execute_lineno);
  }
  let mut fmt: *const libc::c_char = 0 as *const libc::c_char;
  if strcmp(
    name,
    b"EPOCHSECONDS\x00" as *const u8 as *const libc::c_char,
  ) == 0i32
  {
    fmt = b"%lu\x00" as *const u8 as *const libc::c_char
  } else if strcmp(
    name,
    b"EPOCHREALTIME\x00" as *const u8 as *const libc::c_char,
  ) == 0i32
  {
    fmt = b"%lu.%06u\x00" as *const u8 as *const libc::c_char
  }
  if !fmt.is_null() {
    let mut tv: timeval = timeval {
      tv_sec: 0,
      tv_usec: 0,
    };
    gettimeofday(&mut tv, 0 as *mut timezone);
    sprintf(
      (*ptr_to_globals).epoch_buf.as_mut_ptr(),
      fmt,
      tv.tv_sec as libc::c_ulong,
      tv.tv_usec as libc::c_uint,
    );
    return (*ptr_to_globals).epoch_buf.as_mut_ptr();
  }
  return 0 as *const libc::c_char;
}
unsafe extern "C" fn handle_changed_special_names(
  mut name: *const libc::c_char,
  mut name_len: libc::c_uint,
) {
  if name_len == 6i32 as libc::c_uint {
    if strncmp(
      name,
      b"OPTIND\x00" as *const u8 as *const libc::c_char,
      6i32 as libc::c_ulong,
    ) == 0i32
    {
      (*ptr_to_globals).getopt_count = 0i32 as libc::c_uint;
      return;
    }
  };
}
unsafe extern "C" fn set_local_var(
  mut str: *mut libc::c_char,
  mut flags: libc::c_uint,
) -> libc::c_int {
  let mut current_block: u64;
  let mut cur_pp: *mut *mut variable = 0 as *mut *mut variable;
  let mut cur: *mut variable = 0 as *mut variable;
  let mut free_me: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut eq_sign: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut name_len: libc::c_int = 0;
  let mut retval: libc::c_int = 0;
  let mut local_lvl: libc::c_uint = flags >> 3i32;
  eq_sign = strchr(str, '=' as i32);
  if 1i32 != 0 && eq_sign.is_null() {
    bb_simple_error_msg_and_die(b"BUG in setvar\x00" as *const u8 as *const libc::c_char);
  }
  name_len =
    (eq_sign.wrapping_offset_from(str) as libc::c_long + 1i32 as libc::c_long) as libc::c_int;
  cur_pp = &mut (*ptr_to_globals).top_var;
  loop {
    cur = *cur_pp;
    if cur.is_null() {
      current_block = 10930818133215224067;
      break;
    }
    if strncmp((*cur).varstr, str, name_len as libc::c_ulong) != 0i32 {
      cur_pp = &mut (*cur).next
    } else {
      /* We found an existing var with this name */
      if (*cur).flg_read_only != 0 {
        bb_error_msg(
          b"%s: readonly variable\x00" as *const u8 as *const libc::c_char,
          str,
        );
        free(str as *mut libc::c_void);
        //NOTE: in bash, assignment in "export READONLY_VAR=Z" fails, and sets $?=1,
        //but export per se succeeds (does put the var in env). We don't mimic that.
        return -1i32;
      }
      if flags & (1i32 << 1i32) as libc::c_uint != 0 {
        // && cur->flg_export ?
        *eq_sign = '\u{0}' as i32 as libc::c_char;
        unsetenv(str);
        *eq_sign = '=' as i32 as libc::c_char
      }
      if ((*cur).var_nest_level as libc::c_uint) < local_lvl {
        /* bash 3.2.33(1) and exported vars:
         * # export z=z
         * # f() { local z=a; env | grep ^z; }
         * # f
         * z=a
         * # env | grep ^z
         * z=z
         */
        if (*cur).flg_export != 0 {
          flags |= (1i32 << 0i32) as libc::c_uint
        }
        /* New variable is local ("local VAR=VAL" or
         * "VAR=VAL cmd")
         * and existing one is global, or local
         * on a lower level that new one.
         * Remove it from global variable list:
         */
        *cur_pp = (*cur).next;
        if !(*ptr_to_globals).shadowed_vars_pp.is_null() {
          /* Save in "shadowed" list */
          (*cur).next = *(*ptr_to_globals).shadowed_vars_pp;
          *(*ptr_to_globals).shadowed_vars_pp = cur
        } else {
          /* Came from pseudo_exec_argv(), no need to save: delete it */
          if (*cur).max_len == 0i32 {
            /* then free it later */
            /* allocated "VAR=VAL"? */
            free_me = (*cur).varstr
          }
          free(cur as *mut libc::c_void);
        }
        current_block = 10930818133215224067;
        break;
      } else {
        if !(strcmp((*cur).varstr.offset(name_len as isize), eq_sign.offset(1)) == 0i32) {
          /* Replace the value in the found "struct variable" */
          if (*cur).max_len != 0i32 {
            if (*cur).max_len as libc::c_ulong >= strnlen(str, ((*cur).max_len + 1i32) as size_t) {
              /* This one is from startup env, reuse space */
              strcpy((*cur).varstr, str);
            } else {
              /* Can't reuse */
              (*cur).max_len = 0i32;
              current_block = 11779985319611290286;
              break;
            }
          } else {
            /* max_len == 0 signifies "malloced" var, which we can
             * (and have to) free. But we can't free(cur->varstr) here:
             * if cur->flg_export is 1, it is in the environment.
             * We should either unsetenv+free, or wait until putenv,
             * then putenv(new)+free(old).
             */
            free_me = (*cur).varstr;
            current_block = 11779985319611290286;
            break;
          }
        }
        free(str as *mut libc::c_void);
        current_block = 13091164153742313170;
        break;
      }
    }
  }
  match current_block {
    10930818133215224067 =>
    /* Not found or shadowed - create new variable struct */
    {
      cur = xzalloc(::std::mem::size_of::<variable>() as libc::c_ulong) as *mut variable;
      (*cur).var_nest_level = local_lvl as uint16_t;
      (*cur).next = *cur_pp;
      *cur_pp = cur;
      current_block = 11779985319611290286;
    }
    _ => {}
  }
  match current_block {
    11779985319611290286 => (*cur).varstr = str,
    _ => {}
  }
  if flags & (1i32 << 2i32) as libc::c_uint != 0 {
    (*cur).flg_read_only = 1i32 as smallint
  }
  if flags & (1i32 << 0i32) as libc::c_uint != 0 {
    (*cur).flg_export = 1i32 as smallint
  }
  retval = 0i32;
  if (*cur).flg_export != 0 {
    if flags & (1i32 << 1i32) as libc::c_uint != 0 {
      (*cur).flg_export = 0i32 as smallint
    /* unsetenv was already done */
    } else {
      retval = putenv((*cur).varstr)
      /* fall through to "free(free_me)" -
       * only now we can free old exported malloced string
       */
    }
  }
  free(free_me as *mut libc::c_void);
  handle_changed_special_names((*cur).varstr, (name_len - 1i32) as libc::c_uint);
  return retval;
}
unsafe extern "C" fn set_local_var_from_halves(
  mut name: *const libc::c_char,
  mut val: *const libc::c_char,
) {
  let mut var: *mut libc::c_char =
    xasprintf(b"%s=%s\x00" as *const u8 as *const libc::c_char, name, val);
  set_local_var(var, 0i32 as libc::c_uint);
}
/* Used at startup and after each cd */
unsafe extern "C" fn set_pwd_var(mut flag: libc::c_uint) {
  set_local_var(
    xasprintf(
      b"PWD=%s\x00" as *const u8 as *const libc::c_char,
      get_cwd(1i32),
    ),
    flag,
  );
}
unsafe extern "C" fn unset_local_var_len(
  mut name: *const libc::c_char,
  mut name_len: libc::c_int,
) -> libc::c_int {
  let mut cur: *mut variable = 0 as *mut variable;
  let mut cur_pp: *mut *mut variable = 0 as *mut *mut variable;
  cur_pp = &mut (*ptr_to_globals).top_var;
  loop {
    cur = *cur_pp;
    if cur.is_null() {
      break;
    }
    if strncmp((*cur).varstr, name, name_len as libc::c_ulong) == 0i32
      && *(*cur).varstr.offset(name_len as isize) as libc::c_int == '=' as i32
    {
      if (*cur).flg_read_only != 0 {
        bb_error_msg(
          b"%s: readonly variable\x00" as *const u8 as *const libc::c_char,
          name,
        );
        return 1i32;
      }
      *cur_pp = (*cur).next;
      bb_unsetenv((*cur).varstr);
      if (*cur).max_len == 0 {
        free((*cur).varstr as *mut libc::c_void);
      }
      free(cur as *mut libc::c_void);
      break;
    } else {
      cur_pp = &mut (*cur).next
    }
  }
  /* Handle "unset LINENO" et al even if did not find the variable to unset */
  handle_changed_special_names(name, name_len as libc::c_uint);
  return 0i32;
}
unsafe extern "C" fn unset_local_var(mut name: *const libc::c_char) -> libc::c_int {
  return unset_local_var_len(name, strlen(name) as libc::c_int);
}
/*
 * Helpers for "var1=val1 var2=val2 cmd" feature
 */
unsafe extern "C" fn add_vars(mut var: *mut variable) {
  let mut next: *mut variable = 0 as *mut variable;
  while !var.is_null() {
    next = (*var).next;
    (*var).next = (*ptr_to_globals).top_var;
    (*ptr_to_globals).top_var = var;
    if (*var).flg_export != 0 {
      putenv((*var).varstr);
    }
    var = next
  }
}
/* We put strings[i] into variable table and possibly putenv them.
 * If variable is read only, we can free the strings[i]
 * which attempts to overwrite it.
 * The strings[] vector itself is freed.
 */
unsafe extern "C" fn set_vars_and_save_old(mut strings: *mut *mut libc::c_char) {
  let mut s: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  if strings.is_null() {
    return;
  }
  s = strings;
  while !(*s).is_null() {
    let mut current_block_21: u64;
    let mut var_p: *mut variable = 0 as *mut variable;
    let mut var_pp: *mut *mut variable = 0 as *mut *mut variable;
    let mut eq: *mut libc::c_char = 0 as *mut libc::c_char;
    eq = strchr(*s, '=' as i32);
    if 1i32 != 0 && eq.is_null() {
      bb_simple_error_msg_and_die(b"BUG in varexp4\x00" as *const u8 as *const libc::c_char);
    }
    var_pp = get_ptr_to_local_var(
      *s,
      eq.wrapping_offset_from(*s) as libc::c_long as libc::c_uint,
    );
    if !var_pp.is_null() {
      var_p = *var_pp;
      if (*var_p).flg_read_only != 0 {
        let mut p: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        bb_error_msg(
          b"%s: readonly variable\x00" as *const u8 as *const libc::c_char,
          *s,
        );
        /* below, set_local_var() with nest level will
         * "shadow" (remove) this variable from
         * global linked list.
         */
        /*
         * "VAR=V BLTIN" unsets VARs after BLTIN completes.
         * If VAR is readonly, leaving it in the list
         * after asssignment error (msg above)
         * causes doubled error message later, on unset.
         */
        free(*s as *mut libc::c_void);
        p = s;
        loop {
          *p = *p.offset(1);
          p = p.offset(1);
          if (*p).is_null() {
            break;
          }
        }
        current_block_21 = 14818589718467733107;
      } else {
        current_block_21 = 10043043949733653460;
      }
    } else {
      current_block_21 = 10043043949733653460;
    }
    match current_block_21 {
      10043043949733653460 => {
        set_local_var(
          *s,
          (*ptr_to_globals).var_nest_level << 3i32 | (1i32 << 0i32) as libc::c_uint,
        );
        s = s.offset(1)
      }
      _ => {}
    }
  }
  free(strings as *mut libc::c_void);
}
/*
 * Unicode helper
 */
unsafe extern "C" fn reinit_unicode_for_hush() {
  /* Unicode support should be activated even if LANG is set
   * _during_ shell execution, not only if it was set when
   * shell was started. Therefore, re-check LANG every time:
   */
  if false || false {
    let mut s: *const libc::c_char =
      get_local_var_value(b"LC_ALL\x00" as *const u8 as *const libc::c_char);
    if s.is_null() {
      s = get_local_var_value(b"LC_CTYPE\x00" as *const u8 as *const libc::c_char)
    }
    if s.is_null() {
      s = get_local_var_value(b"LANG\x00" as *const u8 as *const libc::c_char)
    }
  };
}
/*
 * in_str support (strings, and "strings" read from files).
 */
/* To test correct lineedit/interactive behavior, type from command line:
 *	echo $P\
 *	\
 *	AT\
 *	H\
 *	\
 * It exercises a lot of corner cases.
 */
unsafe extern "C" fn setup_prompt_string() -> *const libc::c_char {
  let mut prompt_str: *const libc::c_char = 0 as *const libc::c_char;
  prompt_str = get_local_var_value(if (*ptr_to_globals).promptmode as libc::c_int == 0i32 {
    b"PS1\x00" as *const u8 as *const libc::c_char
  } else {
    b"PS2\x00" as *const u8 as *const libc::c_char
  });
  if prompt_str.is_null() {
    prompt_str = b"\x00" as *const u8 as *const libc::c_char
  }
  return prompt_str;
}
unsafe extern "C" fn get_user_input(mut i: *mut in_str) -> libc::c_int {
  let mut r: libc::c_int = 0;
  let mut prompt_str: *const libc::c_char = 0 as *const libc::c_char;
  prompt_str = setup_prompt_string();
  loop {
    reinit_unicode_for_hush();
    if (*ptr_to_globals).flag_SIGINT != 0 {
      /* There was ^C'ed, make it look prettier: */
      bb_putchar('\n' as i32);
      (*ptr_to_globals).flag_SIGINT = 0i32 as smallint
    }
    /* buglet: SIGINT will not make new prompt to appear _at once_,
     * only after <Enter>. (^C works immediately) */
    r = read_line_input(
      (*ptr_to_globals).line_input_state,
      prompt_str,
      (*ptr_to_globals).user_input_buf.as_mut_ptr(),
      1024i32 - 1i32,
    );
    /* read_line_input intercepts ^C, "convert" it to SIGINT */
    if r == 0i32 {
      raise(2i32);
    }
    check_and_run_traps();
    if r != 0i32 && (*ptr_to_globals).flag_SIGINT == 0 {
      break;
    }
    /* ^C or SIGINT: repeat */
    /* bash prints ^C even on real SIGINT (non-kbd generated) */
    write(
      1i32,
      b"^C\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
      2i32 as size_t,
    );
    (*ptr_to_globals).last_exitcode = (128i32 + 2i32) as smalluint
  }
  if r < 0i32 {
    /* EOF/error detected */
    (*i).p = 0 as *const libc::c_char;
    r = -1i32;
    (*i).peek_buf[0] = r;
    return r;
  }
  (*i).p = (*ptr_to_globals).user_input_buf.as_mut_ptr();
  let fresh11 = (*i).p;
  (*i).p = (*i).p.offset(1);
  return *fresh11 as libc::c_uchar as libc::c_int;
}
/* This is the magic location that prints prompts
 * and gets data back from the user */
unsafe extern "C" fn fgetc_interactive(mut i: *mut in_str) -> libc::c_int {
  let mut ch: libc::c_int = 0;
  /* If it's interactive stdin, get new line. */
  if (*ptr_to_globals).interactive_fd != 0 && (*(*i).file).is_stdin != 0 {
    /* Returns first char (or EOF), the rest is in i->p[] */
    ch = get_user_input(i); /* PS2 */
    (*ptr_to_globals).promptmode = 1i32 as smallint
  } else {
    loop
    /* Not stdin: script file, sourced file, etc */
    {
      ch = hfgetc((*i).file);
      if !(ch == '\u{0}' as i32) {
        break;
      }
    }
  }
  return ch;
}
/* INTERACTIVE */
unsafe extern "C" fn i_getch(mut i: *mut in_str) -> libc::c_int {
  let mut ch: libc::c_int = 0;
  if (*i).file.is_null() {
    /* string-based in_str */
    ch = *(*i).p as libc::c_uchar as libc::c_int;
    if ch != '\u{0}' as i32 {
      (*i).p = (*i).p.offset(1);
      (*i).last_char = ch;
      return ch;
    }
    return -1i32;
  }
  /* FILE-based in_str */
  /* This can be stdin, check line editing char[] buffer */
  if !(*i).p.is_null() && *(*i).p as libc::c_int != '\u{0}' as i32 {
    let fresh12 = (*i).p;
    (*i).p = (*i).p.offset(1);
    ch = *fresh12 as libc::c_uchar as libc::c_int
  } else {
    /* peek_buf[] is an int array, not char. Can contain EOF. */
    ch = (*i).peek_buf[0];
    if ch != 0i32 {
      let mut ch2: libc::c_int = (*i).peek_buf[1];
      (*i).peek_buf[0] = ch2;
      if !(ch2 == 0i32) {
        (*i).peek_buf[1] = 0i32
      }
    } else {
      ch = fgetc_interactive(i)
    }
  }
  /* very likely, avoid redundant write */
  (*i).last_char = ch;
  if ch == '\n' as i32 {
    (*ptr_to_globals).parse_lineno = (*ptr_to_globals).parse_lineno.wrapping_add(1)
  }
  return ch;
}
unsafe extern "C" fn i_peek(mut i: *mut in_str) -> libc::c_int {
  let mut ch: libc::c_int = 0;
  if (*i).file.is_null() {
    /* string-based in_str */
    /* Doesn't report EOF on NUL. None of the callers care. */
    return *(*i).p as libc::c_uchar as libc::c_int;
  }
  /* FILE-based in_str */
  /* This can be stdin, check line editing char[] buffer */
  if !(*i).p.is_null() && *(*i).p as libc::c_int != '\u{0}' as i32 {
    return *(*i).p as libc::c_uchar as libc::c_int;
  }
  /* peek_buf[] is an int array, not char. Can contain EOF. */
  ch = (*i).peek_buf[0];
  if ch != 0i32 {
    return ch;
  }
  /* Need to get a new char */
  ch = fgetc_interactive(i);
  /* Save it by either rolling back line editing buffer, or in i->peek_buf[0] */
  if !(*i).p.is_null() {
    (*i).p = (*i).p.offset(-1);
    return ch;
  }
  (*i).peek_buf[0] = ch;
  /*i->peek_buf[1] = 0; - already is */
  return ch;
}
/* Only ever called if i_peek() was called, and did not return EOF.
 * IOW: we know the previous peek saw an ordinary char, not EOF, not NUL,
 * not end-of-line. Therefore we never need to read a new editing line here.
 */
unsafe extern "C" fn i_peek2(mut i: *mut in_str) -> libc::c_int {
  let mut ch: libc::c_int = 0;
  /* There are two cases when i->p[] buffer exists.
   * (1) it's a string in_str.
   * (2) It's a file, and we have a saved line editing buffer.
   * In both cases, we know that i->p[0] exists and not NUL, and
   * the peek2 result is in i->p[1].
   */
  if !(*i).p.is_null() {
    return *(*i).p.offset(1) as libc::c_uchar as libc::c_int;
  }
  /* Now we know it is a file-based in_str. */
  /* peek_buf[] is an int array, not char. Can contain EOF. */
  /* Is there 2nd char? */
  ch = (*i).peek_buf[1];
  if ch == 0i32 {
    loop
    /* We did not read it yet, get it now */
    {
      ch = hfgetc((*i).file);
      if !(ch == '\u{0}' as i32) {
        break;
      }
    }
    (*i).peek_buf[1] = ch
  }
  return ch;
}
unsafe extern "C" fn i_getch_and_eat_bkslash_nl(mut input: *mut in_str) -> libc::c_int {
  loop {
    let mut ch: libc::c_int = 0;
    let mut ch2: libc::c_int = 0;
    ch = i_getch(input);
    if ch != '\\' as i32 {
      return ch;
    }
    ch2 = i_peek(input);
    if ch2 != '\n' as i32 {
      return ch;
    }
    /* backslash+newline, skip it */
    i_getch(input);
  }
}
/* Note: this function _eats_ \<newline> pairs, safe to use plain
 * i_getch() after it instead of i_getch_and_eat_bkslash_nl().
 */
unsafe extern "C" fn i_peek_and_eat_bkslash_nl(mut input: *mut in_str) -> libc::c_int {
  loop {
    let mut ch: libc::c_int = 0;
    let mut ch2: libc::c_int = 0;
    ch = i_peek(input);
    if ch != '\\' as i32 {
      return ch;
    }
    ch2 = i_peek2(input);
    if ch2 != '\n' as i32 {
      return ch;
    }
    /* backslash+newline, skip it */
    i_getch(input);
    i_getch(input);
  }
}
unsafe extern "C" fn setup_file_in_str(mut i: *mut in_str, mut fp: *mut HFILE) {
  memset(
    i as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<in_str>() as libc::c_ulong,
  );
  (*i).file = fp;
  /* i->p = NULL; */
}
unsafe extern "C" fn setup_string_in_str(mut i: *mut in_str, mut s: *const libc::c_char) {
  memset(
    i as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<in_str>() as libc::c_ulong,
  );
  /*i->file = NULL */
  (*i).p = s;
}
unsafe extern "C" fn o_reset_to_empty_unquoted(mut o: *mut o_string) {
  (*o).length = 0i32;
  (*o).has_quoted_part = 0i32 as smallint;
  if !(*o).data.is_null() {
    *(*o).data.offset(0) = '\u{0}' as i32 as libc::c_char
  };
}
unsafe extern "C" fn o_free_and_set_NULL(mut o: *mut o_string) {
  free((*o).data as *mut libc::c_void);
  memset(
    o as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<o_string>() as libc::c_ulong,
  );
}
#[inline(always)]
unsafe extern "C" fn o_free(mut o: *mut o_string) {
  free((*o).data as *mut libc::c_void);
}
unsafe extern "C" fn o_grow_by(mut o: *mut o_string, mut len: libc::c_int) {
  if (*o).length + len > (*o).maxlen {
    (*o).maxlen = ((*o).maxlen as libc::c_ulong).wrapping_add(
      (2i32 * len) as libc::c_ulong
        | (32i32 as libc::c_ulong)
          .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
          .wrapping_sub(1i32 as libc::c_ulong),
    ) as libc::c_int as libc::c_int;
    (*o).data = xrealloc(
      (*o).data as *mut libc::c_void,
      (1i32 + (*o).maxlen) as size_t,
    ) as *mut libc::c_char
  };
}
unsafe extern "C" fn o_addchr(mut o: *mut o_string, mut ch: libc::c_int) {
  if !((*o).length < (*o).maxlen) {
    o_grow_by(o, 1i32);
  }
  /* likely. avoid o_grow_by() call */
  *(*o).data.offset((*o).length as isize) = ch as libc::c_char;
  (*o).length += 1;
  *(*o).data.offset((*o).length as isize) = '\u{0}' as i32 as libc::c_char;
}
unsafe extern "C" fn o_addblock(
  mut o: *mut o_string,
  mut str: *const libc::c_char,
  mut len: libc::c_int,
) {
  o_grow_by(o, len);
  *(mempcpy(
    &mut *(*o).data.offset((*o).length as isize) as *mut libc::c_char as *mut libc::c_void,
    str as *const libc::c_void,
    len as size_t,
  ) as *mut libc::c_char)
    .offset(0) = '\u{0}' as i32 as libc::c_char;
  (*o).length += len;
}
unsafe extern "C" fn o_addstr(mut o: *mut o_string, mut str: *const libc::c_char) {
  o_addblock(o, str, strlen(str) as libc::c_int);
}
unsafe extern "C" fn o_addstr_with_NUL(mut o: *mut o_string, mut str: *const libc::c_char) {
  o_addblock(
    o,
    str,
    strlen(str).wrapping_add(1i32 as libc::c_ulong) as libc::c_int,
  );
}
unsafe extern "C" fn x_mode_addchr(mut ch: libc::c_int) {
  o_addchr(&mut (*ptr_to_globals).x_mode_buf, ch);
}
unsafe extern "C" fn x_mode_addstr(mut str: *const libc::c_char) {
  o_addstr(&mut (*ptr_to_globals).x_mode_buf, str);
}
unsafe extern "C" fn x_mode_addblock(mut str: *const libc::c_char, mut len: libc::c_int) {
  o_addblock(&mut (*ptr_to_globals).x_mode_buf, str, len);
}
unsafe extern "C" fn x_mode_prefix() {
  let mut n: libc::c_int = (*ptr_to_globals).x_mode_depth as libc::c_int;
  loop {
    x_mode_addchr('+' as i32);
    n -= 1;
    if !(n >= 0i32) {
      break;
    }
  }
}
unsafe extern "C" fn x_mode_flush() {
  let mut len: libc::c_int = (*ptr_to_globals).x_mode_buf.length;
  if len <= 0i32 {
    return;
  }
  if (*ptr_to_globals).x_mode_fd > 0i32 {
    *(*ptr_to_globals).x_mode_buf.data.offset(len as isize) = '\n' as i32 as libc::c_char;
    full_write(
      (*ptr_to_globals).x_mode_fd,
      (*ptr_to_globals).x_mode_buf.data as *const libc::c_void,
      (len + 1i32) as size_t,
    );
  }
  (*ptr_to_globals).x_mode_buf.length = 0i32;
}
/*
 * HUSH_BRACE_EXPANSION code needs corresponding quoting on variable expansion side.
 * Currently, "v='{q,w}'; echo $v" erroneously expands braces in $v.
 * Apparently, on unquoted $v bash still does globbing
 * ("v='*.txt'; echo $v" prints all .txt files),
 * but NOT brace expansion! Thus, there should be TWO independent
 * quoting mechanisms on $v expansion side: one protects
 * $v from brace expansion, and other additionally protects "$v" against globbing.
 * We have only second one.
 */
/* My analysis of quoting semantics tells me that state information
 * is associated with a destination, not a source.
 */
unsafe extern "C" fn o_addqchr(mut o: *mut o_string, mut ch: libc::c_int) {
  let mut sz: libc::c_int = 1i32;
  let mut found: *mut libc::c_char = strchr(b"*?[\\{}\x00" as *const u8 as *const libc::c_char, ch);
  if !found.is_null() {
    sz += 1
  }
  o_grow_by(o, sz);
  if !found.is_null() {
    *(*o).data.offset((*o).length as isize) = '\\' as i32 as libc::c_char;
    (*o).length += 1
  }
  *(*o).data.offset((*o).length as isize) = ch as libc::c_char;
  (*o).length += 1;
  *(*o).data.offset((*o).length as isize) = '\u{0}' as i32 as libc::c_char;
}
unsafe extern "C" fn o_addQchr(mut o: *mut o_string, mut ch: libc::c_int) {
  let mut sz: libc::c_int = 1i32;
  if (*o).o_expflags & EXP_FLAG_ESC_GLOB_CHARS as libc::c_int != 0
    && !strchr(b"*?[\\{}\x00" as *const u8 as *const libc::c_char, ch).is_null()
  {
    sz += 1;
    *(*o).data.offset((*o).length as isize) = '\\' as i32 as libc::c_char;
    (*o).length += 1
  }
  o_grow_by(o, sz);
  *(*o).data.offset((*o).length as isize) = ch as libc::c_char;
  (*o).length += 1;
  *(*o).data.offset((*o).length as isize) = '\u{0}' as i32 as libc::c_char;
}
unsafe extern "C" fn o_addqblock(
  mut o: *mut o_string,
  mut str: *const libc::c_char,
  mut len: libc::c_int,
) {
  while len != 0 {
    let mut ch: libc::c_char = 0;
    let mut sz: libc::c_int = 0;
    let mut ordinary_cnt: libc::c_int =
      strcspn(str, b"*?[\\{}\x00" as *const u8 as *const libc::c_char) as libc::c_int;
    if ordinary_cnt > len {
      /* paranoia */
      ordinary_cnt = len
    } /* NUL is already added by o_addblock */
    o_addblock(o, str, ordinary_cnt); /* we are processing + 1 char below */
    if ordinary_cnt == len {
      return;
    }
    str = str.offset(ordinary_cnt as isize);
    len -= ordinary_cnt + 1i32;
    let fresh13 = str;
    str = str.offset(1);
    ch = *fresh13;
    sz = 1i32;
    if ch != 0 {
      /* it is necessarily one of "*?[\\" MAYBE_BRACES */
      sz += 1;
      *(*o).data.offset((*o).length as isize) = '\\' as i32 as libc::c_char;
      (*o).length += 1
    }
    o_grow_by(o, sz);
    *(*o).data.offset((*o).length as isize) = ch;
    (*o).length += 1
  }
  *(*o).data.offset((*o).length as isize) = '\u{0}' as i32 as libc::c_char;
}
unsafe extern "C" fn o_addQblock(
  mut o: *mut o_string,
  mut str: *const libc::c_char,
  mut len: libc::c_int,
) {
  if (*o).o_expflags & EXP_FLAG_ESC_GLOB_CHARS as libc::c_int == 0 {
    o_addblock(o, str, len);
    return;
  }
  o_addqblock(o, str, len);
}
unsafe extern "C" fn o_addQstr(mut o: *mut o_string, mut str: *const libc::c_char) {
  o_addQblock(o, str, strlen(str) as libc::c_int);
}
/* A special kind of o_string for $VAR and `cmd` expansion.
 * It contains char* list[] at the beginning, which is grown in 16 element
 * increments. Actual string data starts at the next multiple of 16 * (char*).
 * list[i] contains an INDEX (int!) into this string data.
 * It means that if list[] needs to grow, data needs to be moved higher up
 * but list[i]'s need not be modified.
 * NB: remembering how many list[i]'s you have there is crucial.
 * o_finalize_list() operation post-processes this structure - calculates
 * and stores actual char* ptrs in list[]. Oh, it NULL terminates it as well.
 */
/* n = o_save_ptr_helper(str, n) "starts new string" by storing an index value
 * in list[n] so that it points past last stored byte so far.
 * It returns n+1. */
unsafe extern "C" fn o_save_ptr_helper(mut o: *mut o_string, mut n: libc::c_int) -> libc::c_int {
  let mut list: *mut *mut libc::c_char = (*o).data as *mut *mut libc::c_char;
  let mut string_start: libc::c_int = 0;
  let mut string_len: libc::c_int = 0;
  if (*o).has_empty_slot == 0 {
    string_start = ((n + 0xfi32 & !0xfi32) as libc::c_ulong)
      .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
      as libc::c_int;
    string_len = (*o).length - string_start;
    if n & 0xfi32 == 0 {
      /* 0, 0x10, 0x20...? */
      /* list[n] points to string_start, make space for 16 more pointers */
      (*o).maxlen = ((*o).maxlen as libc::c_ulong).wrapping_add(
        (0x10i32 as libc::c_ulong)
          .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
      ) as libc::c_int as libc::c_int;
      (*o).data = xrealloc(
        (*o).data as *mut libc::c_void,
        ((*o).maxlen + 1i32) as size_t,
      ) as *mut libc::c_char;
      list = (*o).data as *mut *mut libc::c_char;
      memmove(
        list.offset(n as isize).offset(0x10i32 as isize) as *mut libc::c_void,
        list.offset(n as isize) as *const libc::c_void,
        string_len as libc::c_ulong,
      );
      let ref mut fresh14 = *list.offset((n + 0x10i32 - 1i32) as isize);
      *fresh14 = 0 as *mut libc::c_char;
      (*o).length = ((*o).length as libc::c_ulong).wrapping_add(
        (0x10i32 as libc::c_ulong)
          .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
      ) as libc::c_int as libc::c_int
    }
  } else {
    /*
     * expand_on_ifs() has a "previous argv[] ends in IFS?"
     * check. (grep for -prev-ifs-check-).
     * Ensure that argv[-1][last] is not garbage
     * but zero bytes, to save index check there.
     */
    /* We have empty slot at list[n], reuse without growth */
    string_start = ((n + 1i32 + 0xfi32 & !0xfi32) as libc::c_ulong)
      .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
      as libc::c_int; /* NB: n+1! */
    string_len = (*o).length - string_start;
    (*o).has_empty_slot = 0i32 as smallint
  }
  (*o).has_quoted_part = 0i32 as smallint;
  let ref mut fresh15 = *list.offset(n as isize);
  *fresh15 = string_len as uintptr_t as *mut libc::c_char;
  return n + 1i32;
}
/* "What was our last o_save_ptr'ed position (byte offset relative o->data)?" */
unsafe extern "C" fn o_get_last_ptr(mut o: *mut o_string, mut n: libc::c_int) -> libc::c_int {
  let mut list: *mut *mut libc::c_char = (*o).data as *mut *mut libc::c_char;
  let mut string_start: libc::c_int = ((n + 0xfi32 & !0xfi32) as libc::c_ulong)
    .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
    as libc::c_int;
  return *list.offset((n - 1i32) as isize) as uintptr_t as libc::c_int + string_start;
}
/*
 * Globbing routines.
 *
 * Most words in commands need to be globbed, even ones which are
 * (single or double) quoted. This stems from the possiblity of
 * constructs like "abc"* and 'abc'* - these should be globbed.
 * Having a different code path for fully-quoted strings ("abc",
 * 'abc') would only help performance-wise, but we still need
 * code for partially-quoted strings.
 *
 * Unfortunately, if we want to match bash and ash behavior in all cases,
 * the logic can't be "shell-syntax argument is first transformed
 * to a string, then globbed, and if globbing does not match anything,
 * it is used verbatim". Here are two examples where it fails:
 *
 * 	echo 'b\*'?
 *
 * The globbing can't be avoided (because of '?' at the end).
 * The glob pattern is: b\\\*? - IOW, both \ and * are literals
 * and are glob-escaped. If this does not match, bash/ash print b\*?
 * - IOW: they "unbackslash" the glob pattern.
 * Now, look at this:
 *
 * 	v='\\\*'; echo b$v?
 *
 * The glob pattern is the same here: b\\\*? - the unquoted $v expansion
 * should be used as glob pattern with no changes. However, if glob
 * does not match, bash/ash print b\\\*? - NOT THE SAME as first example!
 *
 * ash implements this by having an encoded representation of the word
 * to glob, which IS NOT THE SAME as the glob pattern - it has more data.
 * Glob pattern is derived from it. If glob fails, the decision what result
 * should be is made using that encoded representation. Not glob pattern.
 */
/* There in a GNU extension, GLOB_BRACE, but it is not usable:
 * first, it processes even {a} (no commas), second,
 * I didn't manage to make it return strings when they don't match
 * existing files. Need to re-implement it.
 */
/* Helper */
unsafe extern "C" fn glob_needed(mut s: *const libc::c_char) -> libc::c_int {
  while *s != 0 {
    if *s as libc::c_int == '\\' as i32 {
      if *s.offset(1) == 0 {
        return 0i32;
      }
      s = s.offset(2)
    } else {
      if *s as libc::c_int == '*' as i32
        || *s as libc::c_int == '[' as i32
        || *s as libc::c_int == '?' as i32
        || *s as libc::c_int == '{' as i32
      {
        return 1i32;
      }
      s = s.offset(1)
    }
  }
  return 0i32;
}
/* Return pointer to next closing brace or to comma */
unsafe extern "C" fn next_brace_sub(mut cp: *const libc::c_char) -> *const libc::c_char {
  let mut depth: libc::c_uint = 0i32 as libc::c_uint;
  cp = cp.offset(1);
  while *cp as libc::c_int != '\u{0}' as i32 {
    if *cp as libc::c_int == '\\' as i32 {
      cp = cp.offset(1);
      if *cp as libc::c_int == '\u{0}' as i32 {
        break;
      }
      cp = cp.offset(1)
    } else {
      if *cp as libc::c_int == '}' as i32 && {
        let fresh16 = depth;
        depth = depth.wrapping_sub(1);
        (fresh16) == 0i32 as libc::c_uint
      } || *cp as libc::c_int == ',' as i32 && depth == 0i32 as libc::c_uint
      {
        break;
      }
      let fresh17 = cp;
      cp = cp.offset(1);
      if *fresh17 as libc::c_int == '{' as i32 {
        depth = depth.wrapping_add(1)
      }
    }
  }
  return if *cp as libc::c_int != '\u{0}' as i32 {
    cp
  } else {
    0 as *const libc::c_char
  };
}
/* Recursive brace globber. Note: may garble pattern[]. */
unsafe extern "C" fn glob_brace(
  mut pattern: *mut libc::c_char,
  mut o: *mut o_string,
  mut n: libc::c_int,
) -> libc::c_int {
  let mut current_block: u64;
  let mut new_pattern_buf: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut begin: *const libc::c_char = 0 as *const libc::c_char;
  let mut next: *const libc::c_char = 0 as *const libc::c_char;
  let mut rest: *const libc::c_char = 0 as *const libc::c_char;
  let mut p: *const libc::c_char = 0 as *const libc::c_char;
  let mut rest_len: size_t = 0;
  begin = pattern;
  loop {
    if *begin as libc::c_int == '\u{0}' as i32 {
      current_block = 406090712968627572;
      break;
    }
    if *begin as libc::c_int == '{' as i32 {
      /* Find the first sub-pattern and at the same time
       * find the rest after the closing brace */
      next = next_brace_sub(begin);
      if next.is_null() {
        /* An illegal expression */
        current_block = 406090712968627572;
        break;
      } else {
        if !(*next as libc::c_int == '}' as i32) {
          current_block = 11307063007268554308;
          break;
        }
        /* "{abc}" with no commas - illegal
         * brace expr, disregard and skip it */
        begin = next.offset(1)
      }
    } else {
      if *begin as libc::c_int == '\\' as i32 && *begin.offset(1) as libc::c_int != '\u{0}' as i32 {
        begin = begin.offset(1)
      }
      begin = begin.offset(1)
    }
  }
  match current_block {
    11307063007268554308 => {
      /* Now find the end of the whole brace expression */
      rest = next;
      loop {
        if !(*rest as libc::c_int != '}' as i32) {
          current_block = 11459959175219260272;
          break;
        }
        rest = next_brace_sub(rest);
        if rest.is_null() {
          current_block = 406090712968627572;
          break;
        }
      }
      match current_block {
        406090712968627572 => {}
        _ => {
          rest = rest.offset(1);
          rest_len = strlen(rest).wrapping_add(1i32 as libc::c_ulong);
          /* We are sure the brace expression is well-formed */
          /* Allocate working buffer large enough for our work */
          new_pattern_buf = xmalloc(strlen(pattern)) as *mut libc::c_char;
          /* We have a brace expression.  BEGIN points to the opening {,
           * NEXT points past the terminator of the first element, and REST
           * points past the final }.  We will accumulate result names from
           * recursive runs for each brace alternative in the buffer using
           * GLOB_APPEND.  */
          p = begin.offset(1);
          loop {
            /* Construct the new glob expression */
            memcpy(
              mempcpy(
                mempcpy(
                  new_pattern_buf as *mut libc::c_void,
                  pattern as *const libc::c_void,
                  begin.wrapping_offset_from(pattern) as libc::c_long as size_t,
                ),
                p as *const libc::c_void,
                next.wrapping_offset_from(p) as libc::c_long as size_t,
              ),
              rest as *const libc::c_void,
              rest_len,
            );
            /* Note: glob_brace() may garble new_pattern_buf[].
             * That's why we re-copy prefix every time (1st memcpy above).
             */
            n = glob_brace(new_pattern_buf, o, n);
            if *next as libc::c_int == '}' as i32 {
              break;
            }
            p = next.offset(1);
            next = next_brace_sub(next)
          }
          free(new_pattern_buf as *mut libc::c_void);
          return n;
        }
      }
    }
    _ => {}
  }
  /* An illegal expression */
  let mut gr: libc::c_int = 0;
  let mut globdata: glob_t = glob_t {
    gl_pathc: 0,
    gl_pathv: 0 as *mut *mut libc::c_char,
    gl_offs: 0,
    gl_flags: 0,
    gl_closedir: None,
    gl_readdir: None,
    gl_opendir: None,
    gl_lstat: None,
    gl_stat: None,
  };
  memset(
    &mut globdata as *mut glob_t as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<glob_t>() as libc::c_ulong,
  );
  gr = glob(pattern, 0i32, None, &mut globdata);
  if gr != 0i32 {
    if gr == 3i32 {
      globfree(&mut globdata);
      /* NB: garbles parameter */
      unbackslash(pattern);
      o_addstr_with_NUL(o, pattern);
      return o_save_ptr_helper(o, n);
    }
    if gr == 1i32 {
      bb_die_memory_exhausted();
    }
    /* GLOB_ABORTED? Only happens with GLOB_ERR flag,
     * but we didn't specify it. Paranoia again. */
    bb_error_msg_and_die(
      b"glob error %d on \'%s\'\x00" as *const u8 as *const libc::c_char,
      gr,
      pattern,
    );
  }
  if !globdata.gl_pathv.is_null() && !(*globdata.gl_pathv.offset(0)).is_null() {
    let mut argv: *mut *mut libc::c_char = globdata.gl_pathv;
    loop {
      o_addstr_with_NUL(o, *argv);
      n = o_save_ptr_helper(o, n);
      argv = argv.offset(1);
      if (*argv).is_null() {
        break;
      }
    }
  }
  globfree(&mut globdata);
  return n;
}
/* Performs globbing on last list[],
 * saving each result as a new list[].
 */
unsafe extern "C" fn perform_glob(mut o: *mut o_string, mut n: libc::c_int) -> libc::c_int {
  let mut pattern: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
  if (*o).data.is_null() {
    return o_save_ptr_helper(o, n);
  }
  pattern = (*o).data.offset(o_get_last_ptr(o, n) as isize);
  if glob_needed(pattern) == 0 {
    /* unbackslash last string in o in place, fix length */
    (*o).length =
      unbackslash(pattern).wrapping_offset_from((*o).data) as libc::c_long as libc::c_int;
    return o_save_ptr_helper(o, n);
  }
  copy = xstrdup(pattern);
  /* "forget" pattern in o */
  (*o).length = pattern.wrapping_offset_from((*o).data) as libc::c_long as libc::c_int;
  n = glob_brace(copy, o, n);
  free(copy as *mut libc::c_void);
  return n;
}
/* !HUSH_BRACE_EXPANSION */
/* !HUSH_BRACE_EXPANSION */
/* If o->o_expflags & EXP_FLAG_GLOB, glob the string so far remembered.
 * Otherwise, just finish current list[] and start new */
unsafe extern "C" fn o_save_ptr(mut o: *mut o_string, mut n: libc::c_int) -> libc::c_int {
  if (*o).o_expflags & EXP_FLAG_GLOB as libc::c_int != 0 {
    /* If o->has_empty_slot, list[n] was already globbed
     * (if it was requested back then when it was filled)
     * so don't do that again! */
    if (*o).has_empty_slot == 0 {
      return perform_glob(o, n);
    }
    /* o_save_ptr_helper is inside */
  }
  return o_save_ptr_helper(o, n);
}
/* "Please convert list[n] to real char* ptrs, and NULL terminate it." */
unsafe extern "C" fn o_finalize_list(
  mut o: *mut o_string,
  mut n: libc::c_int,
) -> *mut *mut libc::c_char {
  let mut list: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  let mut string_start: libc::c_int = 0;
  list = (*o).data as *mut *mut libc::c_char;
  string_start = ((n + 0xfi32 & !0xfi32) as libc::c_ulong)
    .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
    as libc::c_int;
  n -= 1;
  let ref mut fresh18 = *list.offset(n as isize);
  *fresh18 = 0 as *mut libc::c_char;
  while n != 0 {
    n -= 1;
    let ref mut fresh19 = *list.offset(n as isize);
    *fresh19 = (*o)
      .data
      .offset(*list.offset(n as isize) as uintptr_t as libc::c_int as isize)
      .offset(string_start as isize)
  }
  return list;
}
/* Returns pi->next - next pipe in the list */
unsafe extern "C" fn free_pipe(mut pi: *mut pipe) -> *mut pipe {
  let mut next: *mut pipe = 0 as *mut pipe;
  let mut i: libc::c_int = 0;
  i = 0i32;
  while i < (*pi).num_cmds {
    let mut command: *mut command = 0 as *mut command;
    let mut r: *mut redir_struct = 0 as *mut redir_struct;
    let mut rnext: *mut redir_struct = 0 as *mut redir_struct;
    command = &mut *(*pi).cmds.offset(i as isize) as *mut command;
    if !(*command).argv.is_null() {
      free_strings((*command).argv);
      //command->redirects = NULL;
      //command->argv = NULL;
    }
    if !(*command).group.is_null() {
      free_pipe_list((*command).group);
    /* not "else if": on syntax error, we may have both! */
    //command->group = NULL;
    } else if !(*command).child_func.is_null() {
      (*(*command).child_func).parent_cmd = 0 as *mut command
    }
    r = (*command).redirects;
    while !r.is_null() {
      /* else is crucial here.
       * If group != NULL, child_func is meaningless */
      /* guard against the case >$FOO, where foo is unset or blank */
      if !(*r).rd_filename.is_null() {
        free((*r).rd_filename as *mut libc::c_void);
        //r->rd_filename = NULL;
      } /* children are an array, they get freed all at once */
      rnext = (*r).next;
      free(r as *mut libc::c_void);
      r = rnext
    }
    i += 1
  }
  free((*pi).cmds as *mut libc::c_void);
  //pi->cmds = NULL;
  free((*pi).cmdtext as *mut libc::c_void);
  //pi->cmdtext = NULL;
  next = (*pi).next;
  free(pi as *mut libc::c_void);
  return next;
}
unsafe extern "C" fn free_pipe_list(mut pi: *mut pipe) {
  while !pi.is_null() {
    pi = free_pipe(pi)
  }
}
/* ** Parsing routines ***/
/* debug_print_tree */
unsafe extern "C" fn new_pipe() -> *mut pipe {
  let mut pi: *mut pipe = 0 as *mut pipe;
  pi = xzalloc(::std::mem::size_of::<pipe>() as libc::c_ulong) as *mut pipe;
  /*pi->res_word = RES_NONE; - RES_NONE is 0 anyway */
  return pi;
}
/* Command (member of a pipe) is complete, or we start a new pipe
 * if ctx->command is NULL.
 * No errors possible here.
 */
unsafe extern "C" fn done_command(mut ctx: *mut parse_context) -> libc::c_int {
  let mut current_block: u64;
  /* The command is really already in the pipe structure, so
   * advance the pipe counter and make a new, null command. */
  let mut pi: *mut pipe = (*ctx).pipe;
  let mut command: *mut command = (*ctx).command;
  /* Instead we emit error message at run time */
  if !command.is_null() {
    if (*command).group.is_null() && (*command).argv.is_null() && (*command).redirects.is_null() {
      current_block = 12802019148267978622;
    } else {
      (*pi).num_cmds += 1;
      current_block = 2979737022853876585;
    }
  //debug_print_tree(ctx->list_head, 20);
  } else {
    current_block = 2979737022853876585;
  }
  match current_block {
    2979737022853876585 => {
      /* Only real trickiness here is that the uncommitted
       * command structure is not counted in pi->num_cmds. */
      (*pi).cmds = xrealloc(
        (*pi).cmds as *mut libc::c_void,
        (::std::mem::size_of::<command>() as libc::c_ulong)
          .wrapping_mul(((*pi).num_cmds + 1i32) as libc::c_ulong),
      ) as *mut command;
      command = &mut *(*pi).cmds.offset((*pi).num_cmds as isize) as *mut command;
      (*ctx).command = command
    }
    _ => {}
  }
  memset(
    command as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<command>() as libc::c_ulong,
  );
  (*command).lineno = (*ptr_to_globals).parse_lineno;
  return (*pi).num_cmds;
  /* used only for 0/nonzero check */
}
unsafe extern "C" fn done_pipe(mut ctx: *mut parse_context, mut type_0: pipe_style) {
  let mut not_null: libc::c_int = 0;
  /* Close previous command */
  not_null = done_command(ctx);
  (*(*ctx).pipe).pi_inverted = (*ctx).ctx_inverted;
  (*ctx).ctx_inverted = 0i32 as smallint;
  (*(*ctx).pipe).res_word = (*ctx).ctx_res_w;
  let mut current_block_20: u64;
  if type_0 as libc::c_uint == PIPE_BG as libc::c_int as libc::c_uint
    && (*ctx).list_head != (*ctx).pipe
  {
    /* Necessary since && and || have precedence over &:
     * "cmd1 && cmd2 &" must spawn both cmds, not only cmd2,
     * in a backgrounded subshell.
     */
    let mut pi: *mut pipe = 0 as *mut pipe;
    let mut command: *mut command = 0 as *mut command;
    /* Is this actually this construct, all pipes end with && or ||? */
    pi = (*ctx).list_head; /* close pN _not_ with "&"! */
    loop {
      if !(pi != (*ctx).pipe) {
        current_block_20 = 8236137900636309791;
        break;
      }
      if (*pi).followup as libc::c_int != PIPE_AND as libc::c_int
        && (*pi).followup as libc::c_int != PIPE_OR as libc::c_int
      {
        current_block_20 = 9580372470263790445;
        break;
      }
      pi = (*pi).next
    }
    match current_block_20 {
      9580372470263790445 => {}
      _ => {
        (*pi).followup = PIPE_SEQ as libc::c_int as smallint;
        pi = xzalloc(::std::mem::size_of::<pipe>() as libc::c_ulong) as *mut pipe;
        (*pi).followup = PIPE_BG as libc::c_int as smallint;
        (*pi).num_cmds = 1i32;
        (*pi).cmds = xzalloc(::std::mem::size_of::<command>() as libc::c_ulong) as *mut command;
        command = &mut *(*pi).cmds.offset(0) as *mut command;
        if 0i32 != 0i32 {
          /* "if xzalloc didn't do that already" */
          (*command).cmd_type = 0i32 as smallint
        }
        (*command).group = (*ctx).list_head;
        /* Replace all pipes in ctx with one newly created */
        (*ctx).pipe = pi;
        (*ctx).list_head = (*ctx).pipe;
        current_block_20 = 16203760046146113240;
      }
    }
  } else {
    current_block_20 = 9580372470263790445;
  }
  match current_block_20 {
    9580372470263790445 => (*(*ctx).pipe).followup = type_0 as smallint,
    _ => {}
  }
  /* Without this check, even just <enter> on command line generates
   * tree of three NOPs (!). Which is harmless but annoying.
   * IOW: it is safe to do it unconditionally. */
  if not_null != 0
    || (*ctx).ctx_res_w as libc::c_int == RES_FI as libc::c_int
    || (*ctx).ctx_res_w as libc::c_int == RES_DONE as libc::c_int
    || (*ctx).ctx_res_w as libc::c_int == RES_FOR as libc::c_int
    || (*ctx).ctx_res_w as libc::c_int == RES_IN as libc::c_int
    || (*ctx).ctx_res_w as libc::c_int == RES_ESAC as libc::c_int
  {
    let mut new_p: *mut pipe = 0 as *mut pipe;
    new_p = new_pipe();
    (*(*ctx).pipe).next = new_p;
    (*ctx).pipe = new_p;
    //debug_print_tree(ctx->list_head, 10);
    if (*ctx).ctx_res_w as libc::c_int == RES_FOR as libc::c_int
      || (*ctx).ctx_res_w as libc::c_int == RES_IN as libc::c_int
    {
      (*ctx).ctx_res_w = RES_NONE as libc::c_int as smallint
    }
    if (*ctx).ctx_res_w as libc::c_int == RES_MATCH as libc::c_int {
      (*ctx).ctx_res_w = RES_CASE_BODY as libc::c_int as smallint
    }
    if (*ctx).ctx_res_w as libc::c_int == RES_CASE as libc::c_int {
      (*ctx).ctx_res_w = RES_CASE_IN as libc::c_int as smallint
    }
    (*ctx).command = 0 as *mut command;
    done_command(ctx);
  };
}
unsafe extern "C" fn initialize_context(mut ctx: *mut parse_context) {
  memset(
    ctx as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<parse_context>() as libc::c_ulong,
  );
  if MAYBE_ASSIGNMENT as libc::c_int != 0i32 {
    (*ctx).is_assignment = MAYBE_ASSIGNMENT as libc::c_int as smallint
  }
  (*ctx).list_head = new_pipe();
  (*ctx).pipe = (*ctx).list_head;
  /* RES_THEN, RES_DO etc are "sticky" -
   * they remain set for pipes inside if/while.
   * This is used to control execution.
   * RES_FOR and RES_IN are NOT sticky (needed to support
   * cases where variable or value happens to match a keyword):
   */
  /* trick done_command below */
  /* Create the memory for command, roughly:
   * ctx->pipe->cmds = new struct command;
   * ctx->command = &ctx->pipe->cmds[0];
   */
  /* Create the memory for command, roughly:
   * ctx->pipe->cmds = new struct command;
   * ctx->command = &ctx->pipe->cmds[0];
   */
  done_command(ctx);
}
unsafe extern "C" fn match_reserved_word(mut word: *mut o_string) -> *const reserved_combo {
  /* Mostly a list of accepted follow-up reserved words.
   * FLAG_END means we are done with the sequence, and are ready
   * to turn the compound list into a command.
   * FLAG_START means the word must start a new compound list.
   */
  static mut reserved_list: [reserved_combo; 14] = [
    {
      let mut init = reserved_combo {
        literal: [33, 0, 0, 0, 0, 0],
        res: RES_NONE as libc::c_int as libc::c_uchar,
        assignment_flag: NOT_ASSIGNMENT as libc::c_int as libc::c_uchar,
        flag: 0i32,
      };
      init
    },
    {
      let mut init = reserved_combo {
        literal: [105, 102, 0, 0, 0, 0],
        res: RES_IF as libc::c_int as libc::c_uchar,
        assignment_flag: MAYBE_ASSIGNMENT as libc::c_int as libc::c_uchar,
        flag: FLAG_THEN as libc::c_int | FLAG_START as libc::c_int,
      };
      init
    },
    {
      let mut init = reserved_combo {
        literal: [116, 104, 101, 110, 0, 0],
        res: RES_THEN as libc::c_int as libc::c_uchar,
        assignment_flag: MAYBE_ASSIGNMENT as libc::c_int as libc::c_uchar,
        flag: FLAG_ELIF as libc::c_int | FLAG_ELSE as libc::c_int | FLAG_FI as libc::c_int,
      };
      init
    },
    {
      let mut init = reserved_combo {
        literal: [101, 108, 105, 102, 0, 0],
        res: RES_ELIF as libc::c_int as libc::c_uchar,
        assignment_flag: MAYBE_ASSIGNMENT as libc::c_int as libc::c_uchar,
        flag: FLAG_THEN as libc::c_int,
      };
      init
    },
    {
      let mut init = reserved_combo {
        literal: [101, 108, 115, 101, 0, 0],
        res: RES_ELSE as libc::c_int as libc::c_uchar,
        assignment_flag: MAYBE_ASSIGNMENT as libc::c_int as libc::c_uchar,
        flag: FLAG_FI as libc::c_int,
      };
      init
    },
    {
      let mut init = reserved_combo {
        literal: [102, 105, 0, 0, 0, 0],
        res: RES_FI as libc::c_int as libc::c_uchar,
        assignment_flag: NOT_ASSIGNMENT as libc::c_int as libc::c_uchar,
        flag: FLAG_END as libc::c_int,
      };
      init
    },
    {
      let mut init = reserved_combo {
        literal: [102, 111, 114, 0, 0, 0],
        res: RES_FOR as libc::c_int as libc::c_uchar,
        assignment_flag: NOT_ASSIGNMENT as libc::c_int as libc::c_uchar,
        flag: FLAG_IN as libc::c_int | FLAG_DO as libc::c_int | FLAG_START as libc::c_int,
      };
      init
    },
    {
      let mut init = reserved_combo {
        literal: [119, 104, 105, 108, 101, 0],
        res: RES_WHILE as libc::c_int as libc::c_uchar,
        assignment_flag: MAYBE_ASSIGNMENT as libc::c_int as libc::c_uchar,
        flag: FLAG_DO as libc::c_int | FLAG_START as libc::c_int,
      };
      init
    },
    {
      let mut init = reserved_combo {
        literal: [117, 110, 116, 105, 108, 0],
        res: RES_UNTIL as libc::c_int as libc::c_uchar,
        assignment_flag: MAYBE_ASSIGNMENT as libc::c_int as libc::c_uchar,
        flag: FLAG_DO as libc::c_int | FLAG_START as libc::c_int,
      };
      init
    },
    {
      let mut init = reserved_combo {
        literal: [105, 110, 0, 0, 0, 0],
        res: RES_IN as libc::c_int as libc::c_uchar,
        assignment_flag: NOT_ASSIGNMENT as libc::c_int as libc::c_uchar,
        flag: FLAG_DO as libc::c_int,
      };
      init
    },
    {
      let mut init = reserved_combo {
        literal: [100, 111, 0, 0, 0, 0],
        res: RES_DO as libc::c_int as libc::c_uchar,
        assignment_flag: MAYBE_ASSIGNMENT as libc::c_int as libc::c_uchar,
        flag: FLAG_DONE as libc::c_int,
      };
      init
    },
    {
      let mut init = reserved_combo {
        literal: [100, 111, 110, 101, 0, 0],
        res: RES_DONE as libc::c_int as libc::c_uchar,
        assignment_flag: NOT_ASSIGNMENT as libc::c_int as libc::c_uchar,
        flag: FLAG_END as libc::c_int,
      };
      init
    },
    {
      let mut init = reserved_combo {
        literal: [99, 97, 115, 101, 0, 0],
        res: RES_CASE as libc::c_int as libc::c_uchar,
        assignment_flag: NOT_ASSIGNMENT as libc::c_int as libc::c_uchar,
        flag: FLAG_MATCH as libc::c_int | FLAG_START as libc::c_int,
      };
      init
    },
    {
      let mut init = reserved_combo {
        literal: [101, 115, 97, 99, 0, 0],
        res: RES_ESAC as libc::c_int as libc::c_uchar,
        assignment_flag: NOT_ASSIGNMENT as libc::c_int as libc::c_uchar,
        flag: FLAG_END as libc::c_int,
      };
      init
    },
  ];
  let mut r: *const reserved_combo = 0 as *const reserved_combo;
  r = reserved_list.as_ptr();
  while r
    < reserved_list.as_ptr().offset(
      (::std::mem::size_of::<[reserved_combo; 14]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<reserved_combo>() as libc::c_ulong)
        as libc::c_uint as isize,
    )
  {
    if strcmp((*word).data, (*r).literal.as_ptr()) == 0i32 {
      return r;
    }
    r = r.offset(1)
  }
  return 0 as *const reserved_combo;
}
/* Return NULL: not a keyword, else: keyword
 */
unsafe extern "C" fn reserved_word(mut ctx: *mut parse_context) -> *const reserved_combo {
  static mut reserved_match: reserved_combo = {
    let mut init = reserved_combo {
      literal: [0, 0, 0, 0, 0, 0],
      res: RES_MATCH as libc::c_int as libc::c_uchar,
      assignment_flag: NOT_ASSIGNMENT as libc::c_int as libc::c_uchar,
      flag: FLAG_MATCH as libc::c_int | FLAG_ESAC as libc::c_int,
    }; /* NULL */
    init
  };
  let mut r: *const reserved_combo = 0 as *const reserved_combo;
  if (*ctx).word.has_quoted_part != 0 {
    return 0 as *const reserved_combo;
  }
  r = match_reserved_word(&mut (*ctx).word);
  if r.is_null() {
    return r;
  }
  if (*r).res as libc::c_int == RES_IN as libc::c_int
    && (*ctx).ctx_res_w as libc::c_int == RES_CASE_IN as libc::c_int
  {
    /* "case word IN ..." - IN part starts first MATCH part */
    r = &reserved_match
  } else if (*r).flag == 0i32 {
    /* '!' */
    if (*ctx).ctx_inverted != 0 {
      /* bash doesn't accept '! ! true' */
      syntax_error(b"! ! command\x00" as *const u8 as *const libc::c_char);
      (*ctx).ctx_res_w = RES_SNTX as libc::c_int as smallint
    }
    (*ctx).ctx_inverted = 1i32 as smallint;
    return r;
  }
  if (*r).flag & FLAG_START as libc::c_int != 0 {
    let mut old: *mut parse_context = 0 as *mut parse_context;
    old = xmemdup(
      ctx as *const libc::c_void,
      ::std::mem::size_of::<parse_context>() as libc::c_ulong as libc::c_int,
    ) as *mut parse_context;
    initialize_context(ctx);
    (*ctx).stack = old
  } else if (*ctx).old_flag & 1i32 << (*r).res as libc::c_int == 0 {
    syntax_error_at((*ctx).word.data);
    (*ctx).ctx_res_w = RES_SNTX as libc::c_int as smallint;
    return r;
  } else {
    /* "{...} fi" is ok. "{...} if" is not
     * Example:
     * if { echo foo; } then { echo bar; } fi */
    if !(*(*ctx).command).group.is_null() {
      done_pipe(ctx, PIPE_SEQ); /* physical copy */
    }
  }
  (*ctx).ctx_res_w = (*r).res as smallint;
  (*ctx).old_flag = (*r).flag;
  (*ctx).is_assignment = (*r).assignment_flag as smallint;
  if (*ctx).old_flag & FLAG_END as libc::c_int != 0 {
    let mut old_0: *mut parse_context = 0 as *mut parse_context;
    done_pipe(ctx, PIPE_SEQ);
    old_0 = (*ctx).stack;
    (*(*old_0).command).group = (*ctx).list_head;
    (*(*old_0).command).cmd_type = 0i32 as smallint;
    *ctx = *old_0;
    free(old_0 as *mut libc::c_void);
  }
  return r;
}
/* HAS_KEYWORDS */
/* Word is complete, look at it and update parsing context.
 * Normal return is 0. Syntax errors return 1.
 * Note: on return, word is reset, but not o_free'd!
 */
unsafe extern "C" fn done_word(mut ctx: *mut parse_context) -> libc::c_int {
  let mut command: *mut command = (*ctx).command;
  if (*ctx).word.length == 0i32 && (*ctx).word.has_quoted_part == 0 {
    return 0i32;
  }
  if !(*ctx).pending_redirect.is_null() {
    /* We do not glob in e.g. >*.tmp case. bash seems to glob here
     * only if run as "bash", not "sh" */
    /* http://pubs.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html
     * "2.7 Redirection
     * If the redirection operator is "<<" or "<<-", the word
     * that follows the redirection operator shall be
     * subjected to quote removal; it is unspecified whether
     * any of the other expansions occur. For the other
     * redirection operators, the word that follows the
     * redirection operator shall be subjected to tilde
     * expansion, parameter expansion, command substitution,
     * arithmetic expansion, and quote removal.
     * Pathname expansion shall not be performed
     * on the word by a non-interactive shell; an interactive
     * shell may perform it, but shall do so only when
     * the expansion would result in one word."
     */
    //bash does not do parameter/command substitution or arithmetic expansion
    //for _heredoc_ redirection word: these constructs look for exact eof marker
    // as written:
    // <<EOF$t
    // <<EOF$((1))
    // <<EOF`true`  [this case also makes heredoc "quoted", a-la <<"EOF". Probably bash-4.3.43 bug]
    (*(*ctx).pending_redirect).rd_filename = xstrdup((*ctx).word.data);
    /* Cater for >\file case:
     * >\a creates file a; >\\a, >"\a", >"\\a" create file \a
     * Same with heredocs:
     * for <<\H delim is H; <<\\H, <<"\H", <<"\\H" - \H
     */
    if (*(*ctx).pending_redirect).rd_type as libc::c_int == REDIRECT_HEREDOC as libc::c_int {
      unbackslash((*(*ctx).pending_redirect).rd_filename);
      /* Is it <<"HEREDOC"? */
      if (*ctx).word.has_quoted_part != 0 {
        (*(*ctx).pending_redirect).rd_dup |= HEREDOC_QUOTED as libc::c_int
      }
    }
    (*ctx).pending_redirect = 0 as *mut redir_struct
  } else {
    if (*ctx).ctx_dsemicolon as libc::c_int != 0
      && strcmp(
        (*ctx).word.data,
        b"esac\x00" as *const u8 as *const libc::c_char,
      ) != 0i32
    {
      /* not "... pattern) cmd;; esac" */
      /* already done when ctx_dsemicolon was set to 1: */
      /* ctx->ctx_res_w = RES_MATCH; */
      (*ctx).ctx_dsemicolon = 0i32 as smallint
    } else if (*command).argv.is_null()
      && (*ctx).ctx_res_w as libc::c_int != RES_FOR as libc::c_int
      && (*ctx).ctx_res_w as libc::c_int != RES_IN as libc::c_int
      && (*ctx).ctx_res_w as libc::c_int != RES_CASE as libc::c_int
    {
      let mut reserved: *const reserved_combo = 0 as *const reserved_combo;
      reserved = reserved_word(ctx);
      if !reserved.is_null() {
        /* fall through */
        /* Case:
         * "while ...; do
         *	cmd ..."
         * If we don't close the pipe _now_, immediately after "do", lineno logic
         * sees "cmd" as starting at "do" - i.e., at the previous line.
         */
        if 0i32 != 0
          || (*reserved).res as libc::c_int == RES_THEN as libc::c_int
          || (*reserved).res as libc::c_int == RES_ELIF as libc::c_int
          || (*reserved).res as libc::c_int == RES_ELSE as libc::c_int
          || (*reserved).res as libc::c_int == RES_DO as libc::c_int
        {
          done_pipe(ctx, PIPE_SEQ);
        }
        o_reset_to_empty_unquoted(&mut (*ctx).word);
        return ((*ctx).ctx_res_w as libc::c_int == RES_SNTX as libc::c_int) as libc::c_int;
      }
      if 0i32 != 0
        || strcmp(
          (*ctx).word.data,
          b"[[\x00" as *const u8 as *const libc::c_char,
        ) == 0i32
        || strcmp(
          (*ctx).word.data,
          b"local\x00" as *const u8 as *const libc::c_char,
        ) == 0i32
        || strcmp(
          (*ctx).word.data,
          b"export\x00" as *const u8 as *const libc::c_char,
        ) == 0i32
        || strcmp(
          (*ctx).word.data,
          b"readonly\x00" as *const u8 as *const libc::c_char,
        ) == 0i32
      {
        (*command).cmd_type = 2i32 as smallint
      }
    }
    /* HAS_KEYWORDS */
    if !(*command).group.is_null() {
      /* "{ echo foo; } echo bar" - bad */
      syntax_error_at((*ctx).word.data);
      return 1i32;
    }
    /* If this word wasn't an assignment, next ones definitely
     * can't be assignments. Even if they look like ones. */
    if (*ctx).is_assignment as libc::c_int != DEFINITELY_ASSIGNMENT as libc::c_int
      && (*ctx).is_assignment as libc::c_int != WORD_IS_KEYWORD as libc::c_int
    {
      (*ctx).is_assignment = NOT_ASSIGNMENT as libc::c_int as smallint
    } else {
      if (*ctx).is_assignment as libc::c_int == DEFINITELY_ASSIGNMENT as libc::c_int {
        (*command).assignment_cnt = (*command).assignment_cnt.wrapping_add(1)
      }
      (*ctx).is_assignment = MAYBE_ASSIGNMENT as libc::c_int as smallint
    }
    (*command).argv = add_string_to_strings((*command).argv, xstrdup((*ctx).word.data))
  }
  if (*ctx).ctx_res_w as libc::c_int == RES_FOR as libc::c_int {
    if (*ctx).word.has_quoted_part as libc::c_int != 0
      || *endofname(*(*command).argv.offset(0)).offset(0) as libc::c_int != '\u{0}' as i32
    {
      /* bash says just "not a valid identifier" */
      syntax_error(b"not a valid identifier in for\x00" as *const u8 as *const libc::c_char);
      return 1i32;
    }
    /* Force FOR to have just one word (variable name) */
    /* NB: basically, this makes hush see "for v in ..."
     * syntax as if it is "for v; in ...". FOR and IN become
     * two pipe structs in parse tree. */
    done_pipe(ctx, PIPE_SEQ);
  }
  /* Force CASE to have just one word */
  if (*ctx).ctx_res_w as libc::c_int == RES_CASE as libc::c_int {
    done_pipe(ctx, PIPE_SEQ);
  }
  o_reset_to_empty_unquoted(&mut (*ctx).word);
  return 0i32;
}
/* Peek ahead in the input to find out if we have a "&n" construct,
 * as in "2>&1", that represents duplicating a file descriptor.
 * Return:
 * REDIRFD_CLOSE if >&- "close fd" construct is seen,
 * REDIRFD_SYNTAX_ERR if syntax error,
 * REDIRFD_TO_FILE if no & was seen,
 * or the number found.
 */
unsafe extern "C" fn parse_redir_right_fd(mut input: *mut in_str) -> libc::c_int {
  let mut ch: libc::c_int = 0; /* get the & */
  let mut d: libc::c_int = 0;
  let mut ok: libc::c_int = 0;
  ch = i_peek(input);
  if ch != '&' as i32 {
    return REDIRFD_TO_FILE as libc::c_int;
  }
  ch = i_getch(input);
  ch = i_peek(input);
  if ch == '-' as i32 {
    ch = i_getch(input);
    return REDIRFD_CLOSE as libc::c_int;
  }
  d = 0i32;
  ok = 0i32;
  while ch != -1i32 && (ch - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32 {
    d = d * 10i32 + (ch - '0' as i32);
    ok = 1i32;
    ch = i_getch(input);
    ch = i_peek(input)
  }
  if ok != 0 {
    return d;
  }
  //TODO: this is the place to catch ">&file" bashism (redirect both fd 1 and 2)
  bb_simple_error_msg(b"ambiguous redirect\x00" as *const u8 as *const libc::c_char);
  return REDIRFD_SYNTAX_ERR as libc::c_int;
}
/* Return code is 0 normal, 1 if a syntax error is detected
 */
unsafe extern "C" fn parse_redirect(
  mut ctx: *mut parse_context,
  mut fd: libc::c_int,
  mut style: redir_type,
  mut input: *mut in_str,
) -> libc::c_int {
  let mut command: *mut command = (*ctx).command;
  let mut redir: *mut redir_struct = 0 as *mut redir_struct;
  let mut redirp: *mut *mut redir_struct = 0 as *mut *mut redir_struct;
  let mut dup_num: libc::c_int = 0;
  dup_num = REDIRFD_TO_FILE as libc::c_int;
  if style as libc::c_int != REDIRECT_HEREDOC as libc::c_int {
    /* Check for a '>&1' type redirect */
    dup_num = parse_redir_right_fd(input); /* HEREDOC_SKIPTABS bit is 1 */
    if dup_num == REDIRFD_SYNTAX_ERR as libc::c_int {
      return 1i32;
    }
  } else {
    let mut ch: libc::c_int = i_peek_and_eat_bkslash_nl(input);
    dup_num = (ch == '-' as i32) as libc::c_int;
    if dup_num != 0 {
      /* <<-... */
      ch = i_getch(input);
      ch = i_peek(input)
    }
  }
  if style as libc::c_int == REDIRECT_OVERWRITE as libc::c_int
    && dup_num == REDIRFD_TO_FILE as libc::c_int
  {
    let mut ch_0: libc::c_int = i_peek_and_eat_bkslash_nl(input);
    if ch_0 == '|' as i32 {
      /* >|FILE redirect ("clobbering" >).
       * Since we do not support "set -o noclobber" yet,
       * >| and > are the same for now. Just eat |.
       */
      ch_0 = i_getch(input)
    }
  }
  /* Create a new redir_struct and append it to the linked list */
  redirp = &mut (*command).redirects;
  loop {
    redir = *redirp;
    if redir.is_null() {
      break;
    }
    redirp = &mut (*redir).next
  }
  redir = xzalloc(::std::mem::size_of::<redir_struct>() as libc::c_ulong) as *mut redir_struct;
  *redirp = redir;
  /* redir->next = NULL; */
  /* redir->rd_filename = NULL; */
  (*redir).rd_type = style as smallint;
  (*redir).rd_fd = if fd == -1i32 {
    redir_table[style as usize].default_fd as libc::c_int
  } else {
    fd
  };
  (*redir).rd_dup = dup_num;
  if !(style as libc::c_int != REDIRECT_HEREDOC as libc::c_int
    && dup_num != REDIRFD_TO_FILE as libc::c_int)
  {
    /* Instead we emit error message at run time */
    /* Set ctx->pending_redirect, so we know what to do at the
     * end of the next parsed word. */
    (*ctx).pending_redirect = redir
  }
  return 0i32;
}
/* If a redirect is immediately preceded by a number, that number is
 * supposed to tell which file descriptor to redirect.  This routine
 * looks for such preceding numbers.  In an ideal world this routine
 * needs to handle all the following classes of redirects...
 *     echo 2>foo     # redirects fd  2 to file "foo", nothing passed to echo
 *     echo 49>foo    # redirects fd 49 to file "foo", nothing passed to echo
 *     echo -2>foo    # redirects fd  1 to file "foo",    "-2" passed to echo
 *     echo 49x>foo   # redirects fd  1 to file "foo",   "49x" passed to echo
 *
 * http://www.opengroup.org/onlinepubs/009695399/utilities/xcu_chap02.html
 * "2.7 Redirection
 * ... If n is quoted, the number shall not be recognized as part of
 * the redirection expression. For example:
 * echo \2>a
 * writes the character 2 into file a"
 * We are getting it right by setting ->has_quoted_part on any \<char>
 *
 * A -1 return means no valid number was found,
 * the caller should use the appropriate default for this redirection.
 */
unsafe extern "C" fn redirect_opt_num(mut o: *mut o_string) -> libc::c_int {
  let mut num: libc::c_int = 0; /* not \ */
  if (*o).data.is_null() {
    return -1i32;
  }
  num = bb_strtou((*o).data, 0 as *mut *mut libc::c_char, 10i32) as libc::c_int;
  if *bb_errno != 0 || num < 0i32 {
    return -1i32;
  }
  o_reset_to_empty_unquoted(o);
  return num;
}
unsafe extern "C" fn fetch_till_str(
  mut input: *mut in_str,
  mut word: *const libc::c_char,
  mut heredoc_flags: libc::c_int,
) -> *mut libc::c_char {
  let mut current_block: u64;
  let mut heredoc: o_string = {
    let mut init = o_string {
      data: 0 as *mut libc::c_char,
      length: 0,
      maxlen: 0,
      o_expflags: 0,
      has_quoted_part: 0,
      has_empty_slot: 0,
      ended_in_ifs: 0,
    };
    init
  };
  let mut past_EOL: libc::c_uint = 0;
  let mut prev: libc::c_int = 0i32;
  let mut ch: libc::c_int = 0;
  /* Starting with "" is necessary for this case:
   * cat <<EOF
   *
   * xxx
   * EOF
   */
  heredoc.data = xzalloc(1i32 as size_t) as *mut libc::c_char; /* start as "", not as NULL */
  loop {
    past_EOL = heredoc.length as libc::c_uint;
    loop
    /* Get 1st char of next line, possibly skipping leading tabs */
    {
      ch = i_getch(input);
      // TODO: why was this translated this way?
      // (ch) != -1i32;
      if !(heredoc_flags & HEREDOC_SKIPTABS as libc::c_int != 0 && ch == '\t' as i32) {
        break;
      }
    }
    /* If this immediately ended the line,
     * go back to end-of-line checks.
     */
    if ch == '\n' as i32 {
      current_block = 12096910145903341084;
    } else {
      current_block = 16924917904204750491;
    }
    loop {
      match current_block {
        16924917904204750491 => {
          if ch == -1i32 {
            o_free(&mut heredoc);
            return 0 as *mut libc::c_char;
            /* error */
          }
          o_addchr(&mut heredoc, ch);
          if prev == '\\' as i32 && ch == '\\' as i32 {
            /* Correctly handle foo\\<eol> (not a line cont.) */
            prev = 0i32
          } else {
            prev = ch
          }
        }
        _ => {
          if heredoc_flags & HEREDOC_QUOTED as libc::c_int != 0 || prev != '\\' as i32 {
            /* not '\' */
            /* End-of-line, and not a line continuation */
            if strcmp(heredoc.data.offset(past_EOL as isize), word) == 0i32 {
              *heredoc.data.offset(past_EOL as isize) = '\u{0}' as i32 as libc::c_char;
              return heredoc.data;
            }
            if ch == '\n' as i32 {
              break;
            } else {
              current_block = 16924917904204750491;
              continue;
            }
          } else {
            /* Backslash-line continuation in an unquoted
             * heredoc. This does not need special handling
             * for heredoc body (unquoted heredocs are
             * expanded on "execution" and that would take
             * care of this case too), but not the case
             * of line continuation *in terminator*:
             *  cat <<EOF
             *  Ok1
             *  EO\
             *  F
             */
            heredoc.length -= 1; /* not '\' */
            *heredoc.data.offset(heredoc.length as isize) = '\u{0}' as i32 as libc::c_char;
            prev = 0i32
          }
        }
      }
      ch = i_getch(input);

      // TODO: why was this translated this way?
      // (ch) != -1i32;
      if ch == '\n' as i32 || ch == -1i32 {
        current_block = 12096910145903341084;
      } else {
        current_block = 16924917904204750491;
      }
    }
    /* This is a new line.
     * Remember position and backslash-escaping status.
     */
    o_addchr(&mut heredoc, ch);
    prev = ch
  }
}
/* Look at entire parse tree for not-yet-loaded REDIRECT_HEREDOCs
 * and load them all. There should be exactly heredoc_cnt of them.
 */
unsafe extern "C" fn fetch_heredocs(
  mut pi: *mut pipe,
  mut heredoc_cnt: libc::c_int,
  mut input: *mut in_str,
) -> libc::c_int {
  while !pi.is_null() && heredoc_cnt != 0 {
    let mut i: libc::c_int = 0;
    let mut cmd: *mut command = (*pi).cmds;
    i = 0i32;
    while i < (*pi).num_cmds {
      let mut redir: *mut redir_struct = (*cmd).redirects;
      while !redir.is_null() {
        if (*redir).rd_type as libc::c_int == REDIRECT_HEREDOC as libc::c_int {
          let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
          (*redir).rd_type = REDIRECT_HEREDOC2 as libc::c_int as smallint;
          /* redir->rd_dup is (ab)used to indicate <<- */
          p = fetch_till_str(input, (*redir).rd_filename, (*redir).rd_dup);
          if p.is_null() {
            syntax_error(
              b"unexpected EOF in here document\x00" as *const u8 as *const libc::c_char,
            );
            return -1i32;
          }
          free((*redir).rd_filename as *mut libc::c_void);
          (*redir).rd_filename = p;
          heredoc_cnt -= 1
        }
        redir = (*redir).next
      }
      if !(*cmd).group.is_null() {
        //bb_error_msg("%s:%u heredoc_cnt:%d", __func__, __LINE__, heredoc_cnt);
        heredoc_cnt = fetch_heredocs((*cmd).group, heredoc_cnt, input);
        /* error */
        if heredoc_cnt < 0i32 {
          return heredoc_cnt;
        }
      }
      cmd = cmd.offset(1);
      i += 1
    }
    pi = (*pi).next
  }
  return heredoc_cnt;
}
//bb_error_msg("%s:%u heredoc_cnt:%d", __func__, __LINE__, heredoc_cnt);
/* Returns number of heredocs not yet consumed,
 * or -1 on error.
 */
unsafe extern "C" fn parse_group(
  mut ctx: *mut parse_context,
  mut input: *mut in_str,
  mut ch: libc::c_int,
) -> libc::c_int {
  /* ctx->word contains characters seen prior to ( or {.
   * Typically it's empty, but for function defs,
   * it contains function name (without '()'). */
  let mut pipe_list: *mut pipe = 0 as *mut pipe; /* (... */
  let mut heredoc_cnt: libc::c_int = 0i32;
  let mut endch: libc::c_int = 0;
  let mut command: *mut command = (*ctx).command;
  if ch == '(' as i32 && (*ctx).word.has_quoted_part == 0 {
    if (*ctx).word.length != 0 {
      if done_word(ctx) != 0 {
        return -1i32;
      }
    }
    if !(*command).argv.is_null() {
      if !(*(*command).argv.offset(1)).is_null() {
        /* word word ... (... */
        syntax_error_unexpected_ch('(' as i32);
        return -1i32;
      }
      loop
      /* it is "word(..." or "word (..." */
      {
        ch = i_getch(input);
        if !(ch == ' ' as i32 || ch == '\t' as i32) {
          break;
        }
      }
      if ch != ')' as i32 {
        syntax_error_unexpected_ch(ch);
        return -1i32;
      }
      loop {
        ch = i_getch(input);
        if !(ch == ' ' as i32 || ch == '\t' as i32 || ch == '\n' as i32) {
          break;
        }
      }
      if ch != '{' as i32 && ch != '(' as i32 {
        syntax_error_unexpected_ch(ch);
        return -1i32;
      }
      (*command).cmd_type = 3i32 as smallint
    }
  }
  /* Prevented by caller */
  endch = '}' as i32;
  if ch == '(' as i32 {
    endch = ')' as i32;
    if (*command).cmd_type as libc::c_int != 3i32 {
      (*command).cmd_type = 1i32 as smallint
    }
  } else {
    /* bash does not allow "{echo...", requires whitespace */
    ch = i_peek(input);
    if ch != ' ' as i32 && ch != '\t' as i32 && ch != '\n' as i32 && ch != '(' as i32 {
      /* but "{(..." is allowed (without whitespace) */
      syntax_error_unexpected_ch(ch);
      return -1i32;
    }
    if ch != '(' as i32 {
      ch = i_getch(input)
    }
  }
  pipe_list = parse_stream(&mut heredoc_cnt, input, endch);
  /* empty ()/{} or parse error? */
  if pipe_list.is_null() || pipe_list == 1i32 as libc::c_long as *mut libc::c_void as *mut pipe {
    /* parse_stream already emitted error msg */
    if 1i32 == 0 {
      free(0 as *mut libc::c_void);
    }
    return -1i32;
  }
  /* Convert "f() (cmds)" to "f() {(cmds)}" */
  if (*command).cmd_type as libc::c_int == 3i32 && endch == ')' as i32 {
    let mut cmd2: *mut command = 0 as *mut command;
    cmd2 = xzalloc(::std::mem::size_of::<command>() as libc::c_ulong) as *mut command;
    (*cmd2).cmd_type = 1i32 as smallint;
    (*cmd2).group = pipe_list;
    pipe_list = new_pipe();
    (*pipe_list).cmds = cmd2;
    (*pipe_list).num_cmds = 1i32
  }
  (*command).group = pipe_list;
  return heredoc_cnt;
  /* command remains "open", available for possible redirects */
}
/* Subroutines for copying $(...) and `...` things */
/* '...' */
unsafe extern "C" fn add_till_single_quote(
  mut dest: *mut o_string,
  mut input: *mut in_str,
) -> libc::c_int {
  loop {
    let mut ch: libc::c_int = i_getch(input);
    if ch == -1i32 {
      syntax_error_unterm_ch('\'' as i32 as libc::c_char);
      return 0i32;
    }
    if ch == '\'' as i32 {
      return 1i32;
    }
    o_addchr(dest, ch);
  }
}
unsafe extern "C" fn add_till_single_quote_dquoted(
  mut dest: *mut o_string,
  mut input: *mut in_str,
) -> libc::c_int {
  loop {
    let mut ch: libc::c_int = i_getch(input);
    if ch == -1i32 {
      syntax_error_unterm_ch('\'' as i32 as libc::c_char);
      return 0i32;
    }
    if ch == '\'' as i32 {
      return 1i32;
    }
    o_addqchr(dest, ch);
  }
}
unsafe extern "C" fn add_till_double_quote(
  mut dest: *mut o_string,
  mut input: *mut in_str,
) -> libc::c_int {
  loop {
    let mut ch: libc::c_int = i_getch(input);
    if ch == -1i32 {
      syntax_error_unterm_ch('\"' as i32 as libc::c_char);
      return 0i32;
    }
    if ch == '\"' as i32 {
      return 1i32;
    }
    if ch == '\\' as i32 {
      //if (ch == '$') ...
      /* \x. Copy both chars. */
      o_addchr(dest, ch);
      ch = i_getch(input)
    }
    o_addchr(dest, ch);
    if !(ch == '`' as i32) {
      continue;
    }
    if add_till_backquote(dest, input, 1i32) == 0 {
      return 0i32;
    }
    o_addchr(dest, ch);
  }
}
/* "...\"...`..`...." - do we need to handle "...$(..)..." too? */
/* Process `cmd` - copy contents until "`" is seen. Complicated by
 * \` quoting.
 * "Within the backquoted style of command substitution, backslash
 * shall retain its literal meaning, except when followed by: '$', '`', or '\'.
 * The search for the matching backquote shall be satisfied by the first
 * backquote found without a preceding backslash; during this search,
 * if a non-escaped backquote is encountered within a shell comment,
 * a here-document, an embedded command substitution of the $(command)
 * form, or a quoted string, undefined results occur. A single-quoted
 * or double-quoted string that begins, but does not end, within the
 * "`...`" sequence produces undefined results."
 * Example                               Output
 * echo `echo '\'TEST\`echo ZZ\`BEST`    \TESTZZBEST
 */
unsafe extern "C" fn add_till_backquote(
  mut dest: *mut o_string,
  mut input: *mut in_str,
  mut in_dquote: libc::c_int,
) -> libc::c_int {
  loop  {
        let mut ch: libc::c_int = i_getch(input);
        if ch == '`' as i32 { return 1i32 }
        if ch == '\\' as i32 {
            /* \x. Copy both unless it is \`, \$, \\ and maybe \" */
            ch = i_getch(input); /* PS2 */
            if ch != '`' as i32 && ch != '$' as i32 && ch != '\\' as i32 &&
                   (in_dquote == 0 || ch != '\"' as i32) {
                o_addchr(dest, '\\' as i32);
            }
        }
        if ch == -1i32 {
            syntax_error_unterm_ch('`' as i32 as libc::c_char);
            return 0i32
        }
        o_addchr(dest, ch);
    }
}
unsafe extern "C" fn add_till_closing_bracket(
  mut dest: *mut o_string,
  mut input: *mut in_str,
  mut end_ch: libc::c_uint,
) -> libc::c_int {
  let mut ch: libc::c_int = 0;
  let mut dbl: libc::c_char = (end_ch & 0x80i32 as libc::c_uint) as libc::c_char;
  let mut end_char2: libc::c_char = (end_ch >> 8i32) as libc::c_char;
  end_ch &= (0x80i32 - 1i32) as libc::c_uint;
  (*ptr_to_globals).promptmode = 1i32 as smallint;
  loop
  //bb_error_msg("%s:o_addchr('%c') after '\\'", __func__, ch);
  {
    ch = i_getch(input);
    if ch == -1i32 {
      syntax_error_unterm_ch(end_ch as libc::c_char);
      return 0i32;
    }
    if ch as libc::c_uint == end_ch || ch == end_char2 as libc::c_int {
      if dbl == 0 {
        break;
      }
      /* we look for closing )) of $((EXPR)) */
      if i_peek_and_eat_bkslash_nl(input) as libc::c_uint == end_ch {
        i_getch(input); /* eat second ')' */
        break;
      }
    }
    o_addchr(dest, ch);
    //bb_error_msg("%s:o_addchr('%c')", __func__, ch);
    if ch == '(' as i32 || ch == '{' as i32 {
      ch = if ch == '(' as i32 {
        ')' as i32
      } else {
        '}' as i32
      };
      if add_till_closing_bracket(dest, input, ch as libc::c_uint) == 0 {
        return 0i32;
      }
      o_addchr(dest, ch);
    } else if ch == '\'' as i32 {
      if add_till_single_quote(dest, input) == 0 {
        return 0i32;
      }
      o_addchr(dest, ch);
    } else if ch == '\"' as i32 {
      if add_till_double_quote(dest, input) == 0 {
        return 0i32;
      }
      o_addchr(dest, ch);
    } else if ch == '`' as i32 {
      if add_till_backquote(dest, input, 0i32) == 0 {
        return 0i32;
      }
      o_addchr(dest, ch);
    } else {
      if !(ch == '\\' as i32) {
        continue;
      }
      /* \x. Copy verbatim. Important for  \(, \) */
      ch = i_getch(input); /* first character after the $ */
      if ch == -1i32 {
        syntax_error_unterm_ch(end_ch as libc::c_char); /* error */
        return 0i32;
      } /* error */
      o_addchr(dest, ch); /* eat '{' */
    }
  } /* first char after '{' */
  return ch;
}
unsafe extern "C" fn parse_dollar(
  mut dest: *mut o_string,
  mut input: *mut in_str,
  mut quote_mask: libc::c_uchar,
) -> libc::c_int {
  let mut ch: libc::c_int = i_peek_and_eat_bkslash_nl(input);
  let mut current_block_96: u64;
  if ((ch | 0x20i32) - 'a' as i32) as libc::c_uchar as libc::c_int <= 'z' as i32 - 'a' as i32 {
    current_block_96 = 9913858794357128733;
  } else {
    if (ch - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32 {
      current_block_96 = 10164679619615682309;
    } else {
      match ch {
        36 => {
          current_block_96 = 6048265317847332541;
          match current_block_96 {
            5005389895767293342 => {
              let mut pos_0: libc::c_uint = 0;
              ch = i_getch(input);
              if i_peek_and_eat_bkslash_nl(input) == '(' as i32 {
                ch = i_getch(input);
                o_addchr(dest, 3i32);
                o_addchr(dest, '+' as i32);
                if 1i32 == 0 {
                  pos_0 = (*dest).length as libc::c_uint
                }
                if add_till_closing_bracket(dest, input, (')' as i32 | 0x80i32) as libc::c_uint)
                  == 0
                {
                  return 0i32;
                }
                if !(0 as *mut libc::c_void).is_null() {
                  o_addstr(0 as *mut o_string, (*dest).data.offset(pos_0 as isize));
                  o_addchr(0 as *mut o_string, ')' as i32);
                  o_addchr(0 as *mut o_string, ')' as i32);
                }
                o_addchr(dest, 3i32);
              } else {
                o_addchr(dest, 3i32);
                o_addchr(dest, quote_mask as libc::c_int | '`' as i32);
                if 1i32 == 0 {
                  pos_0 = (*dest).length as libc::c_uint
                }
                if add_till_closing_bracket(dest, input, ')' as i32 as libc::c_uint) == 0 {
                  return 0i32;
                }
                if !(0 as *mut libc::c_void).is_null() {
                  o_addstr(0 as *mut o_string, (*dest).data.offset(pos_0 as isize));
                  o_addchr(0 as *mut o_string, ')' as i32);
                }
                o_addchr(dest, 3i32);
              }
              current_block_96 = 10041771570435381152;
            }
            14648156034262866959 => {
              let mut len_single_ch: libc::c_char = 0;
              o_addchr(dest, 3i32);
              ch = i_getch(input);
              ch = i_getch_and_eat_bkslash_nl(input);
              /* It should be ${?}, or ${#var},
               * or even ${?+subst} - operator acting on a special variable,
               * or the beginning of variable name.
               */
              if ch == -1i32
                || strchr(b"_*@$!?#-\x00" as *const u8 as *const libc::c_char, ch).is_null()
                  && bb_ascii_isalnum(ch as libc::c_uchar) == 0
              {
                current_block_96 = 3145260671063309080;
              } else {
                len_single_ch = ch as libc::c_char;
                ch |= quote_mask as libc::c_int;
                /* It's possible to just call add_till_closing_bracket() at this point.
                 * However, this regresses some of our testsuite cases
                 * which check invalid constructs like ${%}.
                 * Oh well... let's check that the var name part is fine... */
                's_205: loop {
                  let mut pos: libc::c_uint = 0;
                  o_addchr(dest, ch);
                  ch = i_getch(input);
                  if ch == '}' as i32 {
                    current_block_96 = 15587532755333643506;
                    break;
                  }
                  if bb_ascii_isalnum(ch as libc::c_uchar) == 0 && ch != '_' as i32 {
                    let mut end_ch: libc::c_uint = 0;
                    let mut last_ch: libc::c_uchar = 0;
                    /* it can't be ${#C} op */
                    /* handle parameter expansions
                     * http://www.opengroup.org/onlinepubs/009695399/utilities/xcu_chap02.html#tag_02_06_02
                     */
                    if strchr(
                      (b"\\/%#:-=+?\x00" as *const u8 as *const libc::c_char).offset(1),
                      ch,
                    )
                    .is_null()
                    {
                      /* ${var<bad_char>... */
                      if len_single_ch as libc::c_int != '#' as i32 || i_peek(input) != '}' as i32 {
                        current_block_96 = 3145260671063309080;
                        break;
                      }
                      /* else: it's "length of C" ${#C} op,
                       * where C is a single char
                       * special var name, e.g. ${#!}.
                       */
                    }
                    /* Eat everything until closing '}' (or ':') */
                    end_ch = '}' as i32 as libc::c_uint;
                    if 1i32 != 0
                      && ch == ':' as i32
                      && strchr(
                        (b"\\/%#:-=+?\x00" as *const u8 as *const libc::c_char).offset(5),
                        i_peek(input),
                      )
                      .is_null()
                    {
                      /* It's ${var:N[:M]} thing */
                      end_ch = ('}' as i32 * 0x100i32 + ':' as i32) as libc::c_uint
                    }
                    if 1i32 != 0 && ch == '/' as i32 {
                      /* It's ${var/[/]pattern[/repl]} thing */
                      if i_peek(input) == '/' as i32 {
                        /* ${var//pattern[/repl]}? */
                        i_getch(input);
                        ch = '\\' as i32
                      }
                      end_ch = ('}' as i32 * 0x100i32 + '/' as i32) as libc::c_uint
                    }
                    o_addchr(dest, ch);
                    /* The pattern can't be empty.
                     * IOW: if the first char after "${v//" is a slash,
                     * it does not terminate the pattern - it's the first char of the pattern:
                     *  v=/dev/ram; echo ${v////-}  prints -dev-ram (pattern is "/")
                     *  v=/dev/ram; echo ${v///r/-} prints /dev-am  (pattern is "/r")
                     */
                    if i_peek(input) == '/' as i32 {
                      o_addchr(dest, i_getch(input));
                    }
                    loop {
                      if 1i32 == 0 {
                        pos = (*dest).length as libc::c_uint
                      }
                      last_ch = add_till_closing_bracket(dest, input, end_ch) as libc::c_uchar;
                      if last_ch as libc::c_int == 0i32 {
                        /* error? */
                        return 0i32;
                      }
                      if !(0 as *mut libc::c_void).is_null() {
                        o_addstr(0 as *mut o_string, (*dest).data.offset(pos as isize));
                        o_addchr(0 as *mut o_string, last_ch as libc::c_int);
                      }
                      if !((1i32 != 0 || 1i32 != 0) && end_ch & 0xff00i32 as libc::c_uint != 0) {
                        current_block_96 = 15587532755333643506;
                        break 's_205;
                      }
                      /* close the first block: */
                      o_addchr(dest, 3i32);
                      /* else: it's ${var/[/]pattern} */
                      /* while parsing N from ${var:N[:M]}
                       * or pattern from ${var/[/]pattern[/repl]} */
                      if !(end_ch & 0xffi32 as libc::c_uint == last_ch as libc::c_uint) {
                        break;
                      }
                      end_ch = '}' as i32 as libc::c_uint
                    }
                    /* got ':' or '/'- parse the rest */
                    /* got '}' */
                    if 1i32 != 0 && end_ch == ('}' as i32 * 0x100i32 + ':' as i32) as libc::c_uint {
                      /* it's ${var:N} - emulate :999999999 */
                      o_addstr(dest, b"999999999\x00" as *const u8 as *const libc::c_char);
                    }
                    current_block_96 = 15587532755333643506;
                    break;
                  } else {
                    len_single_ch = 0i32 as libc::c_char
                  }
                }
                match current_block_96 {
                  3145260671063309080 => {}
                  _ => {
                    o_addchr(dest, 3i32);
                    current_block_96 = 10041771570435381152;
                  }
                }
              }
              match current_block_96 {
                10041771570435381152 => {}
                _ =>
                /* not one of those */
                {
                  syntax_error_unterm_str(b"${name}\x00" as *const u8 as *const libc::c_char);
                  return 0i32;
                }
              }
            }
            6048265317847332541 =>
            /* pid */
            {
              current_block_96 = 10614047152442524953;
            }
            12620587843583950988 => {
              o_addQchr(dest, '$' as i32);
              current_block_96 = 10041771570435381152;
            }
            _ => {}
          }
          match current_block_96 {
            10041771570435381152 => {}
            _ => {
              match current_block_96 {
                10614047152442524953 =>
                /* last bg pid */
                {
                  current_block_96 = 8053711674921723655;
                }
                _ => {}
              }
              match current_block_96 {
                8053711674921723655 =>
                /* last exit code */
                {
                  current_block_96 = 8980779101063628349;
                }
                _ => {}
              }
              match current_block_96 {
                8980779101063628349 =>
                /* number of args */
                {
                  current_block_96 = 7216946888939800620;
                }
                _ => {}
              }
              match current_block_96 {
                7216946888939800620 =>
                  /* args */
                  {}
                _ => {}
              }
              /* args */
              current_block_96 = 10164679619615682309;
            }
          }
        }
        33 => {
          current_block_96 = 10614047152442524953;
          match current_block_96 {
            5005389895767293342 => {
              let mut pos_0: libc::c_uint = 0;
              ch = i_getch(input);
              if i_peek_and_eat_bkslash_nl(input) == '(' as i32 {
                ch = i_getch(input);
                o_addchr(dest, 3i32);
                o_addchr(dest, '+' as i32);
                if 1i32 == 0 {
                  pos_0 = (*dest).length as libc::c_uint
                }
                if add_till_closing_bracket(dest, input, (')' as i32 | 0x80i32) as libc::c_uint)
                  == 0
                {
                  return 0i32;
                }
                if !(0 as *mut libc::c_void).is_null() {
                  o_addstr(0 as *mut o_string, (*dest).data.offset(pos_0 as isize));
                  o_addchr(0 as *mut o_string, ')' as i32);
                  o_addchr(0 as *mut o_string, ')' as i32);
                }
                o_addchr(dest, 3i32);
              } else {
                o_addchr(dest, 3i32);
                o_addchr(dest, quote_mask as libc::c_int | '`' as i32);
                if 1i32 == 0 {
                  pos_0 = (*dest).length as libc::c_uint
                }
                if add_till_closing_bracket(dest, input, ')' as i32 as libc::c_uint) == 0 {
                  return 0i32;
                }
                if !(0 as *mut libc::c_void).is_null() {
                  o_addstr(0 as *mut o_string, (*dest).data.offset(pos_0 as isize));
                  o_addchr(0 as *mut o_string, ')' as i32);
                }
                o_addchr(dest, 3i32);
              }
              current_block_96 = 10041771570435381152;
            }
            14648156034262866959 => {
              let mut len_single_ch: libc::c_char = 0;
              o_addchr(dest, 3i32);
              ch = i_getch(input);
              ch = i_getch_and_eat_bkslash_nl(input);
              if ch == -1i32
                || strchr(b"_*@$!?#-\x00" as *const u8 as *const libc::c_char, ch).is_null()
                  && bb_ascii_isalnum(ch as libc::c_uchar) == 0
              {
                current_block_96 = 3145260671063309080;
              } else {
                len_single_ch = ch as libc::c_char;
                ch |= quote_mask as libc::c_int;
                's_206: loop {
                  let mut pos: libc::c_uint = 0;
                  o_addchr(dest, ch);
                  ch = i_getch(input);
                  if ch == '}' as i32 {
                    current_block_96 = 15587532755333643506;
                    break;
                  }
                  if bb_ascii_isalnum(ch as libc::c_uchar) == 0 && ch != '_' as i32 {
                    let mut end_ch: libc::c_uint = 0;
                    let mut last_ch: libc::c_uchar = 0;
                    if strchr(
                      (b"\\/%#:-=+?\x00" as *const u8 as *const libc::c_char).offset(1),
                      ch,
                    )
                    .is_null()
                    {
                      if len_single_ch as libc::c_int != '#' as i32 || i_peek(input) != '}' as i32 {
                        current_block_96 = 3145260671063309080;
                        break;
                      }
                    }
                    end_ch = '}' as i32 as libc::c_uint;
                    if 1i32 != 0
                      && ch == ':' as i32
                      && strchr(
                        (b"\\/%#:-=+?\x00" as *const u8 as *const libc::c_char).offset(5),
                        i_peek(input),
                      )
                      .is_null()
                    {
                      end_ch = ('}' as i32 * 0x100i32 + ':' as i32) as libc::c_uint
                    }
                    if 1i32 != 0 && ch == '/' as i32 {
                      if i_peek(input) == '/' as i32 {
                        i_getch(input);
                        ch = '\\' as i32
                      }
                      end_ch = ('}' as i32 * 0x100i32 + '/' as i32) as libc::c_uint
                    }
                    o_addchr(dest, ch);
                    if i_peek(input) == '/' as i32 {
                      o_addchr(dest, i_getch(input));
                    }
                    loop {
                      if 1i32 == 0 {
                        pos = (*dest).length as libc::c_uint
                      }
                      last_ch = add_till_closing_bracket(dest, input, end_ch) as libc::c_uchar;
                      if last_ch as libc::c_int == 0i32 {
                        return 0i32;
                      }
                      if !(0 as *mut libc::c_void).is_null() {
                        o_addstr(0 as *mut o_string, (*dest).data.offset(pos as isize));
                        o_addchr(0 as *mut o_string, last_ch as libc::c_int);
                      }
                      if !((1i32 != 0 || 1i32 != 0) && end_ch & 0xff00i32 as libc::c_uint != 0) {
                        current_block_96 = 15587532755333643506;
                        break 's_206;
                      }
                      o_addchr(dest, 3i32);
                      if !(end_ch & 0xffi32 as libc::c_uint == last_ch as libc::c_uint) {
                        break;
                      }
                      end_ch = '}' as i32 as libc::c_uint
                    }
                    if 1i32 != 0 && end_ch == ('}' as i32 * 0x100i32 + ':' as i32) as libc::c_uint {
                      o_addstr(dest, b"999999999\x00" as *const u8 as *const libc::c_char);
                    }
                    current_block_96 = 15587532755333643506;
                    break;
                  } else {
                    len_single_ch = 0i32 as libc::c_char
                  }
                }

                match current_block_96 {
                  3145260671063309080 => {}
                  _ => {
                    o_addchr(dest, 3i32);
                    current_block_96 = 10041771570435381152;
                  }
                }
              }
              match current_block_96 {
                10041771570435381152 => {}
                _ => {
                  syntax_error_unterm_str(b"${name}\x00" as *const u8 as *const libc::c_char);
                  return 0i32;
                }
              }
            }
            6048265317847332541 => {
              current_block_96 = 10614047152442524953;
            }
            12620587843583950988 => {
              o_addQchr(dest, '$' as i32);
              current_block_96 = 10041771570435381152;
            }
            _ => {}
          }
          match current_block_96 {
            10041771570435381152 => {}
            _ => {
              match current_block_96 {
                10614047152442524953 => {
                  current_block_96 = 8053711674921723655;
                }
                _ => {}
              }
              match current_block_96 {
                8053711674921723655 => {
                  current_block_96 = 8980779101063628349;
                }
                _ => {}
              }
              match current_block_96 {
                8980779101063628349 => {
                  current_block_96 = 7216946888939800620;
                }
                _ => {}
              }
              match current_block_96 {
                7216946888939800620 => {}
                _ => {}
              }
              current_block_96 = 10164679619615682309;
            }
          }
        }
        63 => {
          current_block_96 = 8053711674921723655;
          match current_block_96 {
            5005389895767293342 => {
              let mut pos_0: libc::c_uint = 0;
              ch = i_getch(input);
              if i_peek_and_eat_bkslash_nl(input) == '(' as i32 {
                ch = i_getch(input);
                o_addchr(dest, 3i32);
                o_addchr(dest, '+' as i32);
                if 1i32 == 0 {
                  pos_0 = (*dest).length as libc::c_uint
                }
                if add_till_closing_bracket(dest, input, (')' as i32 | 0x80i32) as libc::c_uint)
                  == 0
                {
                  return 0i32;
                }
                if !(0 as *mut libc::c_void).is_null() {
                  o_addstr(0 as *mut o_string, (*dest).data.offset(pos_0 as isize));
                  o_addchr(0 as *mut o_string, ')' as i32);
                  o_addchr(0 as *mut o_string, ')' as i32);
                }
                o_addchr(dest, 3i32);
              } else {
                o_addchr(dest, 3i32);
                o_addchr(dest, quote_mask as libc::c_int | '`' as i32);
                if 1i32 == 0 {
                  pos_0 = (*dest).length as libc::c_uint
                }
                if add_till_closing_bracket(dest, input, ')' as i32 as libc::c_uint) == 0 {
                  return 0i32;
                }
                if !(0 as *mut libc::c_void).is_null() {
                  o_addstr(0 as *mut o_string, (*dest).data.offset(pos_0 as isize));
                  o_addchr(0 as *mut o_string, ')' as i32);
                }
                o_addchr(dest, 3i32);
              }
              current_block_96 = 10041771570435381152;
            }
            14648156034262866959 => {
              let mut len_single_ch: libc::c_char = 0;
              o_addchr(dest, 3i32);
              ch = i_getch(input);
              ch = i_getch_and_eat_bkslash_nl(input);
              if ch == -1i32
                || strchr(b"_*@$!?#-\x00" as *const u8 as *const libc::c_char, ch).is_null()
                  && bb_ascii_isalnum(ch as libc::c_uchar) == 0
              {
                current_block_96 = 3145260671063309080;
              } else {
                len_single_ch = ch as libc::c_char;
                ch |= quote_mask as libc::c_int;
                's_207: loop {
                  let mut pos: libc::c_uint = 0;
                  o_addchr(dest, ch);
                  ch = i_getch(input);
                  if ch == '}' as i32 {
                    current_block_96 = 15587532755333643506;
                    break;
                  }
                  if bb_ascii_isalnum(ch as libc::c_uchar) == 0 && ch != '_' as i32 {
                    let mut end_ch: libc::c_uint = 0;
                    let mut last_ch: libc::c_uchar = 0;
                    if strchr(
                      (b"\\/%#:-=+?\x00" as *const u8 as *const libc::c_char).offset(1),
                      ch,
                    )
                    .is_null()
                    {
                      if len_single_ch as libc::c_int != '#' as i32 || i_peek(input) != '}' as i32 {
                        current_block_96 = 3145260671063309080;
                        break;
                      }
                    }
                    end_ch = '}' as i32 as libc::c_uint;
                    if 1i32 != 0
                      && ch == ':' as i32
                      && strchr(
                        (b"\\/%#:-=+?\x00" as *const u8 as *const libc::c_char).offset(5),
                        i_peek(input),
                      )
                      .is_null()
                    {
                      end_ch = ('}' as i32 * 0x100i32 + ':' as i32) as libc::c_uint
                    }
                    if 1i32 != 0 && ch == '/' as i32 {
                      if i_peek(input) == '/' as i32 {
                        i_getch(input);
                        ch = '\\' as i32
                      }
                      end_ch = ('}' as i32 * 0x100i32 + '/' as i32) as libc::c_uint
                    }
                    o_addchr(dest, ch);
                    if i_peek(input) == '/' as i32 {
                      o_addchr(dest, i_getch(input));
                    }
                    loop {
                      if 1i32 == 0 {
                        pos = (*dest).length as libc::c_uint
                      }
                      last_ch = add_till_closing_bracket(dest, input, end_ch) as libc::c_uchar;
                      if last_ch as libc::c_int == 0i32 {
                        return 0i32;
                      }
                      if !(0 as *mut libc::c_void).is_null() {
                        o_addstr(0 as *mut o_string, (*dest).data.offset(pos as isize));
                        o_addchr(0 as *mut o_string, last_ch as libc::c_int);
                      }
                      if !((1i32 != 0 || 1i32 != 0) && end_ch & 0xff00i32 as libc::c_uint != 0) {
                        current_block_96 = 15587532755333643506;
                        break 's_207;
                      }
                      o_addchr(dest, 3i32);
                      if !(end_ch & 0xffi32 as libc::c_uint == last_ch as libc::c_uint) {
                        break;
                      }
                      end_ch = '}' as i32 as libc::c_uint
                    }
                    if 1i32 != 0 && end_ch == ('}' as i32 * 0x100i32 + ':' as i32) as libc::c_uint {
                      o_addstr(dest, b"999999999\x00" as *const u8 as *const libc::c_char);
                    }
                    current_block_96 = 15587532755333643506;
                    break;
                  } else {
                    len_single_ch = 0i32 as libc::c_char
                  }
                }

                match current_block_96 {
                  3145260671063309080 => {}
                  _ => {
                    o_addchr(dest, 3i32);
                    current_block_96 = 10041771570435381152;
                  }
                }
              }
              match current_block_96 {
                10041771570435381152 => {}
                _ => {
                  syntax_error_unterm_str(b"${name}\x00" as *const u8 as *const libc::c_char);
                  return 0i32;
                }
              }
            }
            6048265317847332541 => {
              current_block_96 = 10614047152442524953;
            }
            12620587843583950988 => {
              o_addQchr(dest, '$' as i32);
              current_block_96 = 10041771570435381152;
            }
            _ => {}
          }
          match current_block_96 {
            10041771570435381152 => {}
            _ => {
              match current_block_96 {
                10614047152442524953 => {
                  current_block_96 = 8053711674921723655;
                }
                _ => {}
              }
              match current_block_96 {
                8053711674921723655 => {
                  current_block_96 = 8980779101063628349;
                }
                _ => {}
              }
              match current_block_96 {
                8980779101063628349 => {
                  current_block_96 = 7216946888939800620;
                }
                _ => {}
              }
              match current_block_96 {
                7216946888939800620 => {}
                _ => {}
              }
              current_block_96 = 10164679619615682309;
            }
          }
        }
        35 => {
          current_block_96 = 8980779101063628349;
          match current_block_96 {
            5005389895767293342 => {
              let mut pos_0: libc::c_uint = 0;
              ch = i_getch(input);
              if i_peek_and_eat_bkslash_nl(input) == '(' as i32 {
                ch = i_getch(input);
                o_addchr(dest, 3i32);
                o_addchr(dest, '+' as i32);
                if 1i32 == 0 {
                  pos_0 = (*dest).length as libc::c_uint
                }
                if add_till_closing_bracket(dest, input, (')' as i32 | 0x80i32) as libc::c_uint)
                  == 0
                {
                  return 0i32;
                }
                if !(0 as *mut libc::c_void).is_null() {
                  o_addstr(0 as *mut o_string, (*dest).data.offset(pos_0 as isize));
                  o_addchr(0 as *mut o_string, ')' as i32);
                  o_addchr(0 as *mut o_string, ')' as i32);
                }
                o_addchr(dest, 3i32);
              } else {
                o_addchr(dest, 3i32);
                o_addchr(dest, quote_mask as libc::c_int | '`' as i32);
                if 1i32 == 0 {
                  pos_0 = (*dest).length as libc::c_uint
                }
                if add_till_closing_bracket(dest, input, ')' as i32 as libc::c_uint) == 0 {
                  return 0i32;
                }
                if !(0 as *mut libc::c_void).is_null() {
                  o_addstr(0 as *mut o_string, (*dest).data.offset(pos_0 as isize));
                  o_addchr(0 as *mut o_string, ')' as i32);
                }
                o_addchr(dest, 3i32);
              }
              current_block_96 = 10041771570435381152;
            }
            14648156034262866959 => {
              let mut len_single_ch: libc::c_char = 0;
              o_addchr(dest, 3i32);
              ch = i_getch(input);
              ch = i_getch_and_eat_bkslash_nl(input);
              if ch == -1i32
                || strchr(b"_*@$!?#-\x00" as *const u8 as *const libc::c_char, ch).is_null()
                  && bb_ascii_isalnum(ch as libc::c_uchar) == 0
              {
                current_block_96 = 3145260671063309080;
              } else {
                len_single_ch = ch as libc::c_char;
                ch |= quote_mask as libc::c_int;
                's_208: loop {
                  let mut pos: libc::c_uint = 0;
                  o_addchr(dest, ch);
                  ch = i_getch(input);
                  if ch == '}' as i32 {
                    current_block_96 = 15587532755333643506;
                    break;
                  }
                  if bb_ascii_isalnum(ch as libc::c_uchar) == 0 && ch != '_' as i32 {
                    let mut end_ch: libc::c_uint = 0;
                    let mut last_ch: libc::c_uchar = 0;
                    if strchr(
                      (b"\\/%#:-=+?\x00" as *const u8 as *const libc::c_char).offset(1),
                      ch,
                    )
                    .is_null()
                    {
                      if len_single_ch as libc::c_int != '#' as i32 || i_peek(input) != '}' as i32 {
                        current_block_96 = 3145260671063309080;
                        break;
                      }
                    }
                    end_ch = '}' as i32 as libc::c_uint;
                    if 1i32 != 0
                      && ch == ':' as i32
                      && strchr(
                        (b"\\/%#:-=+?\x00" as *const u8 as *const libc::c_char).offset(5),
                        i_peek(input),
                      )
                      .is_null()
                    {
                      end_ch = ('}' as i32 * 0x100i32 + ':' as i32) as libc::c_uint
                    }
                    if 1i32 != 0 && ch == '/' as i32 {
                      if i_peek(input) == '/' as i32 {
                        i_getch(input);
                        ch = '\\' as i32
                      }
                      end_ch = ('}' as i32 * 0x100i32 + '/' as i32) as libc::c_uint
                    }
                    o_addchr(dest, ch);
                    if i_peek(input) == '/' as i32 {
                      o_addchr(dest, i_getch(input));
                    }
                    loop {
                      if 1i32 == 0 {
                        pos = (*dest).length as libc::c_uint
                      }
                      last_ch = add_till_closing_bracket(dest, input, end_ch) as libc::c_uchar;
                      if last_ch as libc::c_int == 0i32 {
                        return 0i32;
                      }
                      if !(0 as *mut libc::c_void).is_null() {
                        o_addstr(0 as *mut o_string, (*dest).data.offset(pos as isize));
                        o_addchr(0 as *mut o_string, last_ch as libc::c_int);
                      }
                      if !((1i32 != 0 || 1i32 != 0) && end_ch & 0xff00i32 as libc::c_uint != 0) {
                        current_block_96 = 15587532755333643506;
                        break 's_208;
                      }
                      o_addchr(dest, 3i32);
                      if !(end_ch & 0xffi32 as libc::c_uint == last_ch as libc::c_uint) {
                        break;
                      }
                      end_ch = '}' as i32 as libc::c_uint
                    }
                    if 1i32 != 0 && end_ch == ('}' as i32 * 0x100i32 + ':' as i32) as libc::c_uint {
                      o_addstr(dest, b"999999999\x00" as *const u8 as *const libc::c_char);
                    }
                    current_block_96 = 15587532755333643506;
                    break;
                  } else {
                    len_single_ch = 0i32 as libc::c_char
                  }
                }
                match current_block_96 {
                  3145260671063309080 => {}
                  _ => {
                    o_addchr(dest, 3i32);
                    current_block_96 = 10041771570435381152;
                  }
                }
              }
              match current_block_96 {
                10041771570435381152 => {}
                _ => {
                  syntax_error_unterm_str(b"${name}\x00" as *const u8 as *const libc::c_char);
                  return 0i32;
                }
              }
            }
            6048265317847332541 => {
              current_block_96 = 10614047152442524953;
            }
            12620587843583950988 => {
              o_addQchr(dest, '$' as i32);
              current_block_96 = 10041771570435381152;
            }
            _ => {}
          }
          match current_block_96 {
            10041771570435381152 => {}
            _ => {
              match current_block_96 {
                10614047152442524953 => {
                  current_block_96 = 8053711674921723655;
                }
                _ => {}
              }
              match current_block_96 {
                8053711674921723655 => {
                  current_block_96 = 8980779101063628349;
                }
                _ => {}
              }
              match current_block_96 {
                8980779101063628349 => {
                  current_block_96 = 7216946888939800620;
                }
                _ => {}
              }
              match current_block_96 {
                7216946888939800620 => {}
                _ => {}
              }
              current_block_96 = 10164679619615682309;
            }
          }
        }
        42 => {
          current_block_96 = 7216946888939800620;
          match current_block_96 {
            5005389895767293342 => {
              let mut pos_0: libc::c_uint = 0;
              ch = i_getch(input);
              if i_peek_and_eat_bkslash_nl(input) == '(' as i32 {
                ch = i_getch(input);
                o_addchr(dest, 3i32);
                o_addchr(dest, '+' as i32);
                if 1i32 == 0 {
                  pos_0 = (*dest).length as libc::c_uint
                }
                if add_till_closing_bracket(dest, input, (')' as i32 | 0x80i32) as libc::c_uint)
                  == 0
                {
                  return 0i32;
                }
                if !(0 as *mut libc::c_void).is_null() {
                  o_addstr(0 as *mut o_string, (*dest).data.offset(pos_0 as isize));
                  o_addchr(0 as *mut o_string, ')' as i32);
                  o_addchr(0 as *mut o_string, ')' as i32);
                }
                o_addchr(dest, 3i32);
              } else {
                o_addchr(dest, 3i32);
                o_addchr(dest, quote_mask as libc::c_int | '`' as i32);
                if 1i32 == 0 {
                  pos_0 = (*dest).length as libc::c_uint
                }
                if add_till_closing_bracket(dest, input, ')' as i32 as libc::c_uint) == 0 {
                  return 0i32;
                }
                if !(0 as *mut libc::c_void).is_null() {
                  o_addstr(0 as *mut o_string, (*dest).data.offset(pos_0 as isize));
                  o_addchr(0 as *mut o_string, ')' as i32);
                }
                o_addchr(dest, 3i32);
              }
              current_block_96 = 10041771570435381152;
            }
            14648156034262866959 => {
              let mut len_single_ch: libc::c_char = 0;
              o_addchr(dest, 3i32);
              ch = i_getch(input);
              ch = i_getch_and_eat_bkslash_nl(input);
              if ch == -1i32
                || strchr(b"_*@$!?#-\x00" as *const u8 as *const libc::c_char, ch).is_null()
                  && bb_ascii_isalnum(ch as libc::c_uchar) == 0
              {
                current_block_96 = 3145260671063309080;
              } else {
                len_single_ch = ch as libc::c_char;
                ch |= quote_mask as libc::c_int;
                's_209: loop {
                  let mut pos: libc::c_uint = 0;
                  o_addchr(dest, ch);
                  ch = i_getch(input);
                  if ch == '}' as i32 {
                    current_block_96 = 15587532755333643506;
                    break;
                  }
                  if bb_ascii_isalnum(ch as libc::c_uchar) == 0 && ch != '_' as i32 {
                    let mut end_ch: libc::c_uint = 0;
                    let mut last_ch: libc::c_uchar = 0;
                    if strchr(
                      (b"\\/%#:-=+?\x00" as *const u8 as *const libc::c_char).offset(1),
                      ch,
                    )
                    .is_null()
                    {
                      if len_single_ch as libc::c_int != '#' as i32 || i_peek(input) != '}' as i32 {
                        current_block_96 = 3145260671063309080;
                        break;
                      }
                    }
                    end_ch = '}' as i32 as libc::c_uint;
                    if 1i32 != 0
                      && ch == ':' as i32
                      && strchr(
                        (b"\\/%#:-=+?\x00" as *const u8 as *const libc::c_char).offset(5),
                        i_peek(input),
                      )
                      .is_null()
                    {
                      end_ch = ('}' as i32 * 0x100i32 + ':' as i32) as libc::c_uint
                    }
                    if 1i32 != 0 && ch == '/' as i32 {
                      if i_peek(input) == '/' as i32 {
                        i_getch(input);
                        ch = '\\' as i32
                      }
                      end_ch = ('}' as i32 * 0x100i32 + '/' as i32) as libc::c_uint
                    }
                    o_addchr(dest, ch);
                    if i_peek(input) == '/' as i32 {
                      o_addchr(dest, i_getch(input));
                    }
                    loop {
                      if 1i32 == 0 {
                        pos = (*dest).length as libc::c_uint
                      }
                      last_ch = add_till_closing_bracket(dest, input, end_ch) as libc::c_uchar;
                      if last_ch as libc::c_int == 0i32 {
                        return 0i32;
                      }
                      if !(0 as *mut libc::c_void).is_null() {
                        o_addstr(0 as *mut o_string, (*dest).data.offset(pos as isize));
                        o_addchr(0 as *mut o_string, last_ch as libc::c_int);
                      }
                      if !((1i32 != 0 || 1i32 != 0) && end_ch & 0xff00i32 as libc::c_uint != 0) {
                        current_block_96 = 15587532755333643506;
                        break 's_209;
                      }
                      o_addchr(dest, 3i32);
                      if !(end_ch & 0xffi32 as libc::c_uint == last_ch as libc::c_uint) {
                        break;
                      }
                      end_ch = '}' as i32 as libc::c_uint
                    }
                    if 1i32 != 0 && end_ch == ('}' as i32 * 0x100i32 + ':' as i32) as libc::c_uint {
                      o_addstr(dest, b"999999999\x00" as *const u8 as *const libc::c_char);
                    }
                    current_block_96 = 15587532755333643506;
                    break;
                  } else {
                    len_single_ch = 0i32 as libc::c_char
                  }
                }

                match current_block_96 {
                  3145260671063309080 => {}
                  _ => {
                    o_addchr(dest, 3i32);
                    current_block_96 = 10041771570435381152;
                  }
                }
              }
              match current_block_96 {
                10041771570435381152 => {}
                _ => {
                  syntax_error_unterm_str(b"${name}\x00" as *const u8 as *const libc::c_char);
                  return 0i32;
                }
              }
            }
            6048265317847332541 => {
              current_block_96 = 10614047152442524953;
            }
            12620587843583950988 => {
              o_addQchr(dest, '$' as i32);
              current_block_96 = 10041771570435381152;
            }
            _ => {}
          }
          match current_block_96 {
            10041771570435381152 => {}
            _ => {
              match current_block_96 {
                10614047152442524953 => {
                  current_block_96 = 8053711674921723655;
                }
                _ => {}
              }
              match current_block_96 {
                8053711674921723655 => {
                  current_block_96 = 8980779101063628349;
                }
                _ => {}
              }
              match current_block_96 {
                8980779101063628349 => {
                  current_block_96 = 7216946888939800620;
                }
                _ => {}
              }
              match current_block_96 {
                7216946888939800620 => {}
                _ => {}
              }
              current_block_96 = 10164679619615682309;
            }
          }
        }
        64 => {
          current_block_96 = 950592024231325047;
          match current_block_96 {
            5005389895767293342 => {
              let mut pos_0: libc::c_uint = 0;
              ch = i_getch(input);
              if i_peek_and_eat_bkslash_nl(input) == '(' as i32 {
                ch = i_getch(input);
                o_addchr(dest, 3i32);
                o_addchr(dest, '+' as i32);
                if 1i32 == 0 {
                  pos_0 = (*dest).length as libc::c_uint
                }
                if add_till_closing_bracket(dest, input, (')' as i32 | 0x80i32) as libc::c_uint)
                  == 0
                {
                  return 0i32;
                }
                if !(0 as *mut libc::c_void).is_null() {
                  o_addstr(0 as *mut o_string, (*dest).data.offset(pos_0 as isize));
                  o_addchr(0 as *mut o_string, ')' as i32);
                  o_addchr(0 as *mut o_string, ')' as i32);
                }
                o_addchr(dest, 3i32);
              } else {
                o_addchr(dest, 3i32);
                o_addchr(dest, quote_mask as libc::c_int | '`' as i32);
                if 1i32 == 0 {
                  pos_0 = (*dest).length as libc::c_uint
                }
                if add_till_closing_bracket(dest, input, ')' as i32 as libc::c_uint) == 0 {
                  return 0i32;
                }
                if !(0 as *mut libc::c_void).is_null() {
                  o_addstr(0 as *mut o_string, (*dest).data.offset(pos_0 as isize));
                  o_addchr(0 as *mut o_string, ')' as i32);
                }
                o_addchr(dest, 3i32);
              }
              current_block_96 = 10041771570435381152;
            }
            14648156034262866959 => {
              let mut len_single_ch: libc::c_char = 0;
              o_addchr(dest, 3i32);
              ch = i_getch(input);
              ch = i_getch_and_eat_bkslash_nl(input);
              if ch == -1i32
                || strchr(b"_*@$!?#-\x00" as *const u8 as *const libc::c_char, ch).is_null()
                  && bb_ascii_isalnum(ch as libc::c_uchar) == 0
              {
                current_block_96 = 3145260671063309080;
              } else {
                len_single_ch = ch as libc::c_char;
                ch |= quote_mask as libc::c_int;
                's_210: loop {
                  let mut pos: libc::c_uint = 0;
                  o_addchr(dest, ch);
                  ch = i_getch(input);
                  if ch == '}' as i32 {
                    current_block_96 = 15587532755333643506;
                    break;
                  }
                  if bb_ascii_isalnum(ch as libc::c_uchar) == 0 && ch != '_' as i32 {
                    let mut end_ch: libc::c_uint = 0;
                    let mut last_ch: libc::c_uchar = 0;
                    if strchr(
                      (b"\\/%#:-=+?\x00" as *const u8 as *const libc::c_char).offset(1),
                      ch,
                    )
                    .is_null()
                    {
                      if len_single_ch as libc::c_int != '#' as i32 || i_peek(input) != '}' as i32 {
                        current_block_96 = 3145260671063309080;
                        break;
                      }
                    }
                    end_ch = '}' as i32 as libc::c_uint;
                    if 1i32 != 0
                      && ch == ':' as i32
                      && strchr(
                        (b"\\/%#:-=+?\x00" as *const u8 as *const libc::c_char).offset(5),
                        i_peek(input),
                      )
                      .is_null()
                    {
                      end_ch = ('}' as i32 * 0x100i32 + ':' as i32) as libc::c_uint
                    }
                    if 1i32 != 0 && ch == '/' as i32 {
                      if i_peek(input) == '/' as i32 {
                        i_getch(input);
                        ch = '\\' as i32
                      }
                      end_ch = ('}' as i32 * 0x100i32 + '/' as i32) as libc::c_uint
                    }
                    o_addchr(dest, ch);
                    if i_peek(input) == '/' as i32 {
                      o_addchr(dest, i_getch(input));
                    }
                    loop {
                      if 1i32 == 0 {
                        pos = (*dest).length as libc::c_uint
                      }
                      last_ch = add_till_closing_bracket(dest, input, end_ch) as libc::c_uchar;
                      if last_ch as libc::c_int == 0i32 {
                        return 0i32;
                      }
                      if !(0 as *mut libc::c_void).is_null() {
                        o_addstr(0 as *mut o_string, (*dest).data.offset(pos as isize));
                        o_addchr(0 as *mut o_string, last_ch as libc::c_int);
                      }
                      if !((1i32 != 0 || 1i32 != 0) && end_ch & 0xff00i32 as libc::c_uint != 0) {
                        current_block_96 = 15587532755333643506;
                        break 's_210;
                      }
                      o_addchr(dest, 3i32);
                      if !(end_ch & 0xffi32 as libc::c_uint == last_ch as libc::c_uint) {
                        break;
                      }
                      end_ch = '}' as i32 as libc::c_uint
                    }
                    if 1i32 != 0 && end_ch == ('}' as i32 * 0x100i32 + ':' as i32) as libc::c_uint {
                      o_addstr(dest, b"999999999\x00" as *const u8 as *const libc::c_char);
                    }
                    current_block_96 = 15587532755333643506;
                    break;
                  } else {
                    len_single_ch = 0i32 as libc::c_char
                  }
                }

                match current_block_96 {
                  3145260671063309080 => {}
                  _ => {
                    o_addchr(dest, 3i32);
                    current_block_96 = 10041771570435381152;
                  }
                }
              }
              match current_block_96 {
                10041771570435381152 => {}
                _ => {
                  syntax_error_unterm_str(b"${name}\x00" as *const u8 as *const libc::c_char);
                  return 0i32;
                }
              }
            }
            6048265317847332541 => {
              current_block_96 = 10614047152442524953;
            }
            12620587843583950988 => {
              o_addQchr(dest, '$' as i32);
              current_block_96 = 10041771570435381152;
            }
            _ => {}
          }
          match current_block_96 {
            10041771570435381152 => {}
            _ => {
              match current_block_96 {
                10614047152442524953 => {
                  current_block_96 = 8053711674921723655;
                }
                _ => {}
              }
              match current_block_96 {
                8053711674921723655 => {
                  current_block_96 = 8980779101063628349;
                }
                _ => {}
              }
              match current_block_96 {
                8980779101063628349 => {
                  current_block_96 = 7216946888939800620;
                }
                _ => {}
              }
              match current_block_96 {
                7216946888939800620 => {}
                _ => {}
              }
              current_block_96 = 10164679619615682309;
            }
          }
        }
        45 => {
          current_block_96 = 10164679619615682309;
        }
        123 => {
          current_block_96 = 14648156034262866959;
          match current_block_96 {
            5005389895767293342 => {
              let mut pos_0: libc::c_uint = 0;
              ch = i_getch(input);
              if i_peek_and_eat_bkslash_nl(input) == '(' as i32 {
                ch = i_getch(input);
                o_addchr(dest, 3i32);
                o_addchr(dest, '+' as i32);
                if 1i32 == 0 {
                  pos_0 = (*dest).length as libc::c_uint
                }
                if add_till_closing_bracket(dest, input, (')' as i32 | 0x80i32) as libc::c_uint)
                  == 0
                {
                  return 0i32;
                }
                if !(0 as *mut libc::c_void).is_null() {
                  o_addstr(0 as *mut o_string, (*dest).data.offset(pos_0 as isize));
                  o_addchr(0 as *mut o_string, ')' as i32);
                  o_addchr(0 as *mut o_string, ')' as i32);
                }
                o_addchr(dest, 3i32);
              } else {
                o_addchr(dest, 3i32);
                o_addchr(dest, quote_mask as libc::c_int | '`' as i32);
                if 1i32 == 0 {
                  pos_0 = (*dest).length as libc::c_uint
                }
                if add_till_closing_bracket(dest, input, ')' as i32 as libc::c_uint) == 0 {
                  return 0i32;
                }
                if !(0 as *mut libc::c_void).is_null() {
                  o_addstr(0 as *mut o_string, (*dest).data.offset(pos_0 as isize));
                  o_addchr(0 as *mut o_string, ')' as i32);
                }
                o_addchr(dest, 3i32);
              }
              current_block_96 = 10041771570435381152;
            }
            14648156034262866959 => {
              let mut len_single_ch: libc::c_char = 0;
              o_addchr(dest, 3i32);
              ch = i_getch(input);
              ch = i_getch_and_eat_bkslash_nl(input);
              if ch == -1i32
                || strchr(b"_*@$!?#-\x00" as *const u8 as *const libc::c_char, ch).is_null()
                  && bb_ascii_isalnum(ch as libc::c_uchar) == 0
              {
                current_block_96 = 3145260671063309080;
              } else {
                len_single_ch = ch as libc::c_char;
                ch |= quote_mask as libc::c_int;
                's_211: loop {
                  let mut pos: libc::c_uint = 0;
                  o_addchr(dest, ch);
                  ch = i_getch(input);
                  if ch == '}' as i32 {
                    current_block_96 = 15587532755333643506;
                    break;
                  }
                  if bb_ascii_isalnum(ch as libc::c_uchar) == 0 && ch != '_' as i32 {
                    let mut end_ch: libc::c_uint = 0;
                    let mut last_ch: libc::c_uchar = 0;
                    if strchr(
                      (b"\\/%#:-=+?\x00" as *const u8 as *const libc::c_char).offset(1),
                      ch,
                    )
                    .is_null()
                    {
                      if len_single_ch as libc::c_int != '#' as i32 || i_peek(input) != '}' as i32 {
                        current_block_96 = 3145260671063309080;
                        break;
                      }
                    }
                    end_ch = '}' as i32 as libc::c_uint;
                    if 1i32 != 0
                      && ch == ':' as i32
                      && strchr(
                        (b"\\/%#:-=+?\x00" as *const u8 as *const libc::c_char).offset(5),
                        i_peek(input),
                      )
                      .is_null()
                    {
                      end_ch = ('}' as i32 * 0x100i32 + ':' as i32) as libc::c_uint
                    }
                    if 1i32 != 0 && ch == '/' as i32 {
                      if i_peek(input) == '/' as i32 {
                        i_getch(input);
                        ch = '\\' as i32
                      }
                      end_ch = ('}' as i32 * 0x100i32 + '/' as i32) as libc::c_uint
                    }
                    o_addchr(dest, ch);
                    if i_peek(input) == '/' as i32 {
                      o_addchr(dest, i_getch(input));
                    }
                    loop {
                      if 1i32 == 0 {
                        pos = (*dest).length as libc::c_uint
                      }
                      last_ch = add_till_closing_bracket(dest, input, end_ch) as libc::c_uchar;
                      if last_ch as libc::c_int == 0i32 {
                        return 0i32;
                      }
                      if !(0 as *mut libc::c_void).is_null() {
                        o_addstr(0 as *mut o_string, (*dest).data.offset(pos as isize));
                        o_addchr(0 as *mut o_string, last_ch as libc::c_int);
                      }
                      if !((1i32 != 0 || 1i32 != 0) && end_ch & 0xff00i32 as libc::c_uint != 0) {
                        current_block_96 = 15587532755333643506;
                        break 's_211;
                      }
                      o_addchr(dest, 3i32);
                      if !(end_ch & 0xffi32 as libc::c_uint == last_ch as libc::c_uint) {
                        break;
                      }
                      end_ch = '}' as i32 as libc::c_uint
                    }
                    if 1i32 != 0 && end_ch == ('}' as i32 * 0x100i32 + ':' as i32) as libc::c_uint {
                      o_addstr(dest, b"999999999\x00" as *const u8 as *const libc::c_char);
                    }
                    current_block_96 = 15587532755333643506;
                    break;
                  } else {
                    len_single_ch = 0i32 as libc::c_char
                  }
                }

                match current_block_96 {
                  3145260671063309080 => {}
                  _ => {
                    o_addchr(dest, 3i32);
                    current_block_96 = 10041771570435381152;
                  }
                }
              }
              match current_block_96 {
                10041771570435381152 => {}
                _ => {
                  syntax_error_unterm_str(b"${name}\x00" as *const u8 as *const libc::c_char);
                  return 0i32;
                }
              }
            }
            6048265317847332541 => {
              current_block_96 = 10614047152442524953;
            }
            12620587843583950988 => {
              o_addQchr(dest, '$' as i32);
              current_block_96 = 10041771570435381152;
            }
            _ => {}
          }
          match current_block_96 {
            10041771570435381152 => {}
            _ => {
              match current_block_96 {
                10614047152442524953 => {
                  current_block_96 = 8053711674921723655;
                }
                _ => {}
              }
              match current_block_96 {
                8053711674921723655 => {
                  current_block_96 = 8980779101063628349;
                }
                _ => {}
              }
              match current_block_96 {
                8980779101063628349 => {
                  current_block_96 = 7216946888939800620;
                }
                _ => {}
              }
              match current_block_96 {
                7216946888939800620 => {}
                _ => {}
              }
              current_block_96 = 10164679619615682309;
            }
          }
        }
        40 => {
          current_block_96 = 5005389895767293342;
          match current_block_96 {
            5005389895767293342 => {
              let mut pos_0: libc::c_uint = 0;
              ch = i_getch(input);
              if i_peek_and_eat_bkslash_nl(input) == '(' as i32 {
                ch = i_getch(input);
                o_addchr(dest, 3i32);
                o_addchr(dest, '+' as i32);
                if 1i32 == 0 {
                  pos_0 = (*dest).length as libc::c_uint
                }
                if add_till_closing_bracket(dest, input, (')' as i32 | 0x80i32) as libc::c_uint)
                  == 0
                {
                  return 0i32;
                }
                if !(0 as *mut libc::c_void).is_null() {
                  o_addstr(0 as *mut o_string, (*dest).data.offset(pos_0 as isize));
                  o_addchr(0 as *mut o_string, ')' as i32);
                  o_addchr(0 as *mut o_string, ')' as i32);
                }
                o_addchr(dest, 3i32);
              } else {
                o_addchr(dest, 3i32);
                o_addchr(dest, quote_mask as libc::c_int | '`' as i32);
                if 1i32 == 0 {
                  pos_0 = (*dest).length as libc::c_uint
                }
                if add_till_closing_bracket(dest, input, ')' as i32 as libc::c_uint) == 0 {
                  return 0i32;
                }
                if !(0 as *mut libc::c_void).is_null() {
                  o_addstr(0 as *mut o_string, (*dest).data.offset(pos_0 as isize));
                  o_addchr(0 as *mut o_string, ')' as i32);
                }
                o_addchr(dest, 3i32);
              }
              current_block_96 = 10041771570435381152;
            }
            14648156034262866959 => {
              let mut len_single_ch: libc::c_char = 0;
              o_addchr(dest, 3i32);
              ch = i_getch(input);
              ch = i_getch_and_eat_bkslash_nl(input);
              if ch == -1i32
                || strchr(b"_*@$!?#-\x00" as *const u8 as *const libc::c_char, ch).is_null()
                  && bb_ascii_isalnum(ch as libc::c_uchar) == 0
              {
                current_block_96 = 3145260671063309080;
              } else {
                len_single_ch = ch as libc::c_char;
                ch |= quote_mask as libc::c_int;
                's_212: loop {
                  let mut pos: libc::c_uint = 0;
                  o_addchr(dest, ch);
                  ch = i_getch(input);
                  if ch == '}' as i32 {
                    current_block_96 = 15587532755333643506;
                    break;
                  }
                  if bb_ascii_isalnum(ch as libc::c_uchar) == 0 && ch != '_' as i32 {
                    let mut end_ch: libc::c_uint = 0;
                    let mut last_ch: libc::c_uchar = 0;
                    if strchr(
                      (b"\\/%#:-=+?\x00" as *const u8 as *const libc::c_char).offset(1),
                      ch,
                    )
                    .is_null()
                    {
                      if len_single_ch as libc::c_int != '#' as i32 || i_peek(input) != '}' as i32 {
                        current_block_96 = 3145260671063309080;
                        break;
                      }
                    }
                    end_ch = '}' as i32 as libc::c_uint;
                    if 1i32 != 0
                      && ch == ':' as i32
                      && strchr(
                        (b"\\/%#:-=+?\x00" as *const u8 as *const libc::c_char).offset(5),
                        i_peek(input),
                      )
                      .is_null()
                    {
                      end_ch = ('}' as i32 * 0x100i32 + ':' as i32) as libc::c_uint
                    }
                    if 1i32 != 0 && ch == '/' as i32 {
                      if i_peek(input) == '/' as i32 {
                        i_getch(input);
                        ch = '\\' as i32
                      }
                      end_ch = ('}' as i32 * 0x100i32 + '/' as i32) as libc::c_uint
                    }
                    o_addchr(dest, ch);
                    if i_peek(input) == '/' as i32 {
                      o_addchr(dest, i_getch(input));
                    }
                    loop {
                      if 1i32 == 0 {
                        pos = (*dest).length as libc::c_uint
                      }
                      last_ch = add_till_closing_bracket(dest, input, end_ch) as libc::c_uchar;
                      if last_ch as libc::c_int == 0i32 {
                        return 0i32;
                      }
                      if !(0 as *mut libc::c_void).is_null() {
                        o_addstr(0 as *mut o_string, (*dest).data.offset(pos as isize));
                        o_addchr(0 as *mut o_string, last_ch as libc::c_int);
                      }
                      if !((1i32 != 0 || 1i32 != 0) && end_ch & 0xff00i32 as libc::c_uint != 0) {
                        current_block_96 = 15587532755333643506;
                        break 's_212;
                      }
                      o_addchr(dest, 3i32);
                      if !(end_ch & 0xffi32 as libc::c_uint == last_ch as libc::c_uint) {
                        break;
                      }
                      end_ch = '}' as i32 as libc::c_uint
                    }
                    if 1i32 != 0 && end_ch == ('}' as i32 * 0x100i32 + ':' as i32) as libc::c_uint {
                      o_addstr(dest, b"999999999\x00" as *const u8 as *const libc::c_char);
                    }
                    current_block_96 = 15587532755333643506;
                    break;
                  } else {
                    len_single_ch = 0i32 as libc::c_char
                  }
                }

                match current_block_96 {
                  3145260671063309080 => {}
                  _ => {
                    o_addchr(dest, 3i32);
                    current_block_96 = 10041771570435381152;
                  }
                }
              }
              match current_block_96 {
                10041771570435381152 => {}
                _ => {
                  syntax_error_unterm_str(b"${name}\x00" as *const u8 as *const libc::c_char);
                  return 0i32;
                }
              }
            }
            6048265317847332541 => {
              current_block_96 = 10614047152442524953;
            }
            12620587843583950988 => {
              o_addQchr(dest, '$' as i32);
              current_block_96 = 10041771570435381152;
            }
            _ => {}
          }
          match current_block_96 {
            10041771570435381152 => {}
            _ => {
              match current_block_96 {
                10614047152442524953 => {
                  current_block_96 = 8053711674921723655;
                }
                _ => {}
              }
              match current_block_96 {
                8053711674921723655 => {
                  current_block_96 = 8980779101063628349;
                }
                _ => {}
              }
              match current_block_96 {
                8980779101063628349 => {
                  current_block_96 = 7216946888939800620;
                }
                _ => {}
              }
              match current_block_96 {
                7216946888939800620 => {}
                _ => {}
              }
              current_block_96 = 10164679619615682309;
            }
          }
        }
        95 => {
          current_block_96 = 9913858794357128733;
        }
        _ => {
          current_block_96 = 12620587843583950988;
          match current_block_96 {
            5005389895767293342 => {
              let mut pos_0: libc::c_uint = 0;
              ch = i_getch(input);
              if i_peek_and_eat_bkslash_nl(input) == '(' as i32 {
                ch = i_getch(input);
                o_addchr(dest, 3i32);
                o_addchr(dest, '+' as i32);
                if 1i32 == 0 {
                  pos_0 = (*dest).length as libc::c_uint
                }
                if add_till_closing_bracket(dest, input, (')' as i32 | 0x80i32) as libc::c_uint)
                  == 0
                {
                  return 0i32;
                }
                if !(0 as *mut libc::c_void).is_null() {
                  o_addstr(0 as *mut o_string, (*dest).data.offset(pos_0 as isize));
                  o_addchr(0 as *mut o_string, ')' as i32);
                  o_addchr(0 as *mut o_string, ')' as i32);
                }
                o_addchr(dest, 3i32);
              } else {
                o_addchr(dest, 3i32);
                o_addchr(dest, quote_mask as libc::c_int | '`' as i32);
                if 1i32 == 0 {
                  pos_0 = (*dest).length as libc::c_uint
                }
                if add_till_closing_bracket(dest, input, ')' as i32 as libc::c_uint) == 0 {
                  return 0i32;
                }
                if !(0 as *mut libc::c_void).is_null() {
                  o_addstr(0 as *mut o_string, (*dest).data.offset(pos_0 as isize));
                  o_addchr(0 as *mut o_string, ')' as i32);
                }
                o_addchr(dest, 3i32);
              }
              current_block_96 = 10041771570435381152;
            }
            14648156034262866959 => {
              let mut len_single_ch: libc::c_char = 0;
              o_addchr(dest, 3i32);
              ch = i_getch(input);
              ch = i_getch_and_eat_bkslash_nl(input);
              if ch == -1i32
                || strchr(b"_*@$!?#-\x00" as *const u8 as *const libc::c_char, ch).is_null()
                  && bb_ascii_isalnum(ch as libc::c_uchar) == 0
              {
                current_block_96 = 3145260671063309080;
              } else {
                len_single_ch = ch as libc::c_char;
                ch |= quote_mask as libc::c_int;
                's_213: loop {
                  let mut pos: libc::c_uint = 0;
                  o_addchr(dest, ch);
                  ch = i_getch(input);
                  if ch == '}' as i32 {
                    current_block_96 = 15587532755333643506;
                    break;
                  }
                  if bb_ascii_isalnum(ch as libc::c_uchar) == 0 && ch != '_' as i32 {
                    let mut end_ch: libc::c_uint = 0;
                    let mut last_ch: libc::c_uchar = 0;
                    if strchr(
                      (b"\\/%#:-=+?\x00" as *const u8 as *const libc::c_char).offset(1),
                      ch,
                    )
                    .is_null()
                    {
                      if len_single_ch as libc::c_int != '#' as i32 || i_peek(input) != '}' as i32 {
                        current_block_96 = 3145260671063309080;
                        break;
                      }
                    }
                    end_ch = '}' as i32 as libc::c_uint;
                    if 1i32 != 0
                      && ch == ':' as i32
                      && strchr(
                        (b"\\/%#:-=+?\x00" as *const u8 as *const libc::c_char).offset(5),
                        i_peek(input),
                      )
                      .is_null()
                    {
                      end_ch = ('}' as i32 * 0x100i32 + ':' as i32) as libc::c_uint
                    }
                    if 1i32 != 0 && ch == '/' as i32 {
                      if i_peek(input) == '/' as i32 {
                        i_getch(input);
                        ch = '\\' as i32
                      }
                      end_ch = ('}' as i32 * 0x100i32 + '/' as i32) as libc::c_uint
                    }
                    o_addchr(dest, ch);
                    if i_peek(input) == '/' as i32 {
                      o_addchr(dest, i_getch(input));
                    }
                    loop {
                      if 1i32 == 0 {
                        pos = (*dest).length as libc::c_uint
                      }
                      last_ch = add_till_closing_bracket(dest, input, end_ch) as libc::c_uchar;
                      if last_ch as libc::c_int == 0i32 {
                        return 0i32;
                      }
                      if !(0 as *mut libc::c_void).is_null() {
                        o_addstr(0 as *mut o_string, (*dest).data.offset(pos as isize));
                        o_addchr(0 as *mut o_string, last_ch as libc::c_int);
                      }
                      if !((1i32 != 0 || 1i32 != 0) && end_ch & 0xff00i32 as libc::c_uint != 0) {
                        current_block_96 = 15587532755333643506;
                        break 's_213;
                      }
                      o_addchr(dest, 3i32);
                      if !(end_ch & 0xffi32 as libc::c_uint == last_ch as libc::c_uint) {
                        break;
                      }
                      end_ch = '}' as i32 as libc::c_uint
                    }
                    if 1i32 != 0 && end_ch == ('}' as i32 * 0x100i32 + ':' as i32) as libc::c_uint {
                      o_addstr(dest, b"999999999\x00" as *const u8 as *const libc::c_char);
                    }
                    current_block_96 = 15587532755333643506;
                    break;
                  } else {
                    len_single_ch = 0i32 as libc::c_char
                  }
                }

                match current_block_96 {
                  3145260671063309080 => {}
                  _ => {
                    o_addchr(dest, 3i32);
                    current_block_96 = 10041771570435381152;
                  }
                }
              }
              match current_block_96 {
                10041771570435381152 => {}
                _ => {
                  syntax_error_unterm_str(b"${name}\x00" as *const u8 as *const libc::c_char);
                  return 0i32;
                }
              }
            }
            6048265317847332541 => {
              current_block_96 = 10614047152442524953;
            }
            12620587843583950988 => {
              o_addQchr(dest, '$' as i32);
              current_block_96 = 10041771570435381152;
            }
            _ => {}
          }
          match current_block_96 {
            10041771570435381152 => {}
            _ => {
              match current_block_96 {
                10614047152442524953 => {
                  current_block_96 = 8053711674921723655;
                }
                _ => {}
              }
              match current_block_96 {
                8053711674921723655 => {
                  current_block_96 = 8980779101063628349;
                }
                _ => {}
              }
              match current_block_96 {
                8980779101063628349 => {
                  current_block_96 = 7216946888939800620;
                }
                _ => {}
              }
              match current_block_96 {
                7216946888939800620 => {}
                _ => {}
              }
              current_block_96 = 10164679619615682309;
            }
          }
        }
      }
    }
    match current_block_96 {
      10041771570435381152 => {}
      9913858794357128733 => {}
      _ =>
      /* $- option flags set by set builtin or shell options (-i etc) */
      {
        ch = i_getch(input);
        o_addchr(dest, 3i32);
        o_addchr(dest, ch | quote_mask as libc::c_int);
        o_addchr(dest, 3i32);
        current_block_96 = 10041771570435381152;
      }
    }
  }
  match current_block_96 {
    9913858794357128733 => {
      ch = i_getch(input);
      /*make_var1:*/
      o_addchr(dest, 3i32);
      loop {
        o_addchr(dest, ch | quote_mask as libc::c_int);
        quote_mask = 0i32 as libc::c_uchar;
        ch = i_peek_and_eat_bkslash_nl(input);
        if bb_ascii_isalnum(ch as libc::c_uchar) == 0 && ch != '_' as i32 {
          break;
        }
        ch = i_getch(input)
      }
      o_addchr(dest, 3i32);
    }
    _ => {}
  }
  return 1i32;
}
unsafe extern "C" fn encode_string(
  mut dest: *mut o_string,
  mut input: *mut in_str,
  mut dquote_end: libc::c_int,
) -> libc::c_int {
  let mut ch: libc::c_int = 0;
  let mut next: libc::c_int = 0;
  loop
  {
    //debug_printf_subst("SUBST RES3 '%s'\n", dest->data + pos);
        ch = i_getch(input);
        // TODO: why was this translated this way?
        // (ch) != -1i32;
        if ch == dquote_end {
            /* may be only '"' or EOF */
            return 1i32
        }
        /* note: can't move it above ch == dquote_end check! */
        if ch == -1i32 {
            syntax_error_unterm_ch('\"' as i32 as libc::c_char);
            return 0i32
            /* error */
        }
        next = '\u{0}' as i32;
        if ch != '\n' as i32 { next = i_peek(input) }
        if ch == '\\' as i32 {
            if next == -1i32 {
                /* Testcase: in interactive shell a file with
			 *  echo "unterminated string\<eof>
			 * is sourced.
			 */
                syntax_error_unterm_ch('\"' as i32 as libc::c_char);
                return 0i32
                /* error */
            }
            /* bash:
		 * "The backslash retains its special meaning [in "..."]
		 * only when followed by one of the following characters:
		 * $, `, ", \, or <newline>.  A double quote may be quoted
		 * within double quotes by preceding it with a backslash."
		 * NB: in (unquoted) heredoc, above does not apply to ",
		 * therefore we check for it by "next == dquote_end" cond.
		 */
            if next == dquote_end ||
                   !strchr(b"$`\\\n\x00" as *const u8 as *const libc::c_char,
                           next).is_null()
               { /* else: ch remains == '\\', and we double it below: */
                ch = i_getch(input);
                if ch == '\n' as i32 {
                    continue ; /* eat next */
                }
                /* skip \<newline> */
            } /* \c if c is a glob char, else just c */
            o_addqchr(dest, ch);
        } else if ch == '$' as i32 {
            if parse_dollar(dest, input, 0x80i32 as libc::c_uchar) == 0 {
                return 0i32
            }
        } else if ch == '`' as i32 {
            //unsigned pos = dest->length;
            o_addchr(dest, 3i32); /* error */
            o_addchr(dest, 0x80i32 | '`' as i32);
            if add_till_backquote(dest, input,
                                  (dquote_end == '\"' as i32) as libc::c_int)
                   == 0 {
                return 0i32
            }
            o_addchr(dest, 3i32);
        } else { o_addQchr(dest, ch); }
    }
}
/*
 * Scan input until EOF or end_trigger char.
 * Return a list of pipes to execute, or NULL on EOF
 * or if end_trigger character is met.
 * On syntax error, exit if shell is not interactive,
 * reset parsing machinery and start parsing anew,
 * or return ERR_PTR.
 */
unsafe extern "C" fn parse_stream(
  mut heredoc_cnt_ptr: *mut libc::c_int,
  mut input: *mut in_str,
  mut end_trigger: libc::c_int,
) -> *mut pipe {
  let mut current_block: u64;
  let mut ctx: parse_context = parse_context {
    list_head: 0 as *mut pipe,
    pipe: 0 as *mut pipe,
    command: 0 as *mut command,
    pending_redirect: 0 as *mut redir_struct,
    word: o_string {
      data: 0 as *mut libc::c_char,
      length: 0,
      maxlen: 0,
      o_expflags: 0,
      has_quoted_part: 0,
      has_empty_slot: 0,
      ended_in_ifs: 0,
    },
    is_assignment: 0,
    ctx_res_w: 0,
    ctx_inverted: 0,
    ctx_dsemicolon: 0,
    old_flag: 0,
    stack: 0 as *mut parse_context,
  };
  let mut heredoc_cnt: libc::c_int = 0;
  /* Single-quote triggers a bypass of the main loop until its mate is
   * found.  When recursing, quote state is passed in via ctx.word.o_expflags.
   */
  initialize_context(&mut ctx);
  /* If very first arg is "" or '', ctx.word.data may end up NULL.
   * Preventing this:
   */
  ctx.word.data = xzalloc(1i32 as size_t) as *mut libc::c_char; /* start as "", not as NULL */
  /* We used to separate words on $IFS here. This was wrong.
   * $IFS is used only for word splitting when $var is expanded,
   * here we should use blank chars as separators, not $IFS
   */
  heredoc_cnt = 0i32; /* while (1) */
  's_38: loop
  /* not ""(... */
  {
    let mut is_blank: *const libc::c_char = 0 as *const libc::c_char;
    let mut is_special: *const libc::c_char = 0 as *const libc::c_char;
    let mut ch: libc::c_int = 0;
    let mut next: libc::c_int = 0;
    let mut redir_fd: libc::c_int = 0;
    let mut redir_style: redir_type = REDIRECT_INPUT;
    ch = i_getch(input);
    if ch == -1i32 {
      let mut pi: *mut pipe = 0 as *mut pipe;
      if heredoc_cnt != 0 {
        syntax_error_unterm_str(b"here document\x00" as *const u8 as *const libc::c_char);
        current_block = 1907364584679199995;
        break;
      } else if end_trigger == ')' as i32 {
        syntax_error_unterm_ch('(' as i32 as libc::c_char);
        current_block = 1907364584679199995;
        break;
      } else if end_trigger == '}' as i32 {
        syntax_error_unterm_ch('{' as i32 as libc::c_char);
        current_block = 1907364584679199995;
        break;
      } else {
        if done_word(&mut ctx) != 0 {
          current_block = 1907364584679199995;
          break;
        }
        o_free_and_set_NULL(&mut ctx.word);
        done_pipe(&mut ctx, PIPE_SEQ);
        pi = ctx.list_head;
        /* get next char */
        /* If we got nothing... */
        /* (this makes bare "&" cmd a no-op.
         * bash says: "syntax error near unexpected token '&'") */
        if (*pi).num_cmds == 0i32 && (*pi).res_word as libc::c_int == RES_NONE as libc::c_int {
          free_pipe_list(pi);
          pi = 0 as *mut pipe
        }
        // heredoc_cnt must be 0 here anyway
        //if (heredoc_cnt_ptr)
        //	*heredoc_cnt_ptr = heredoc_cnt;
        return pi;
      }
    } else if ch == '\\' as i32 {
      ch = i_getch(input);
      /* Handle "'" and "\" first, as they won't play nice with
       * i_peek_and_eat_bkslash_nl() anyway:
       *   echo z\\
       * and
       *   echo '\
       *   '
       * would break.
       */
      /* get next char */
      if ch == '\n' as i32 {
        continue; /* drop \<newline>, get next char */
      }
      o_addchr(&mut ctx.word, '\\' as i32);
      if !(ch == -1i32) {
        /* Example: echo Hello \2>file
         * we need to know that word 2 is quoted
         */
        ctx.word.has_quoted_part = 1i32 as smallint;
        o_addchr(&mut ctx.word, ch);
      }
    } else {
      if ch == '\'' as i32 {
        ctx.word.has_quoted_part = 1i32 as smallint;
        next = i_getch(input);
        if !(next == '\'' as i32 && ctx.pending_redirect.is_null()) {
          ch = next;
          loop {
            if ch == -1i32 {
              syntax_error_unterm_ch('\'' as i32 as libc::c_char);
              current_block = 1907364584679199995;
              break 's_38;
            } else {
              if ch == '\'' as i32 {
                continue 's_38;
              }
              if ch == 3i32 {
                /* get next char */
                /* Convert raw ^C to corresponding special variable reference */
                o_addchr(&mut ctx.word, 3i32);
                o_addchr(&mut ctx.word, 1i32);
              }
              o_addqchr(&mut ctx.word, ch);
              ch = i_getch(input)
            }
          }
        }
      } else {
        next = '\u{0}' as i32;
        if ch != '\n' as i32 {
          next = i_peek_and_eat_bkslash_nl(input)
        }
        is_special = b"{}<>;&|()#$\"`\x03\x00" as *const u8 as *const libc::c_char;
        /* Are { and } special here? */
        if !(*ctx.command).argv.is_null()
          || ctx.word.length != 0
          || ctx.word.has_quoted_part as libc::c_int != 0
          || next != ';' as i32
            && next != ')' as i32
            && next != '(' as i32
            && next != '&' as i32
            && next != '|' as i32
            && strchr(defifsvar.as_ptr().offset(4), next).is_null()
        {
          /* They are not special, skip "{}" */
          is_special = is_special.offset(2)
        }
        is_special = strchr(is_special, ch);
        is_blank = strchr(defifsvar.as_ptr().offset(4), ch);
        if is_special.is_null() && is_blank.is_null() {
          /* ordinary char */
          current_block = 18309173857163198886;
        } else {
          if !is_blank.is_null() {
            /* Case:
             * "while ...; do<whitespace><newline>
             *	cmd ..."
             * would think that "cmd" starts in <whitespace> -
             * i.e., at the previous line.
             * We need to skip all whitespace before newlines.
             */
            while ch != '\n' as i32 {
              next = i_peek(input); /* next char is not ws */
              if next != ' ' as i32 && next != '\t' as i32 && next != '\n' as i32 {
                break;
              }
              ch = i_getch(input)
            }
            /* ch == last eaten whitespace char */
            if done_word(&mut ctx) != 0 {
              current_block = 1907364584679199995;
              break;
            }
            if ch == '\n' as i32 {
              /* Is this a case when newline is simply ignored?
               * Some examples:
               * "cmd | <newline> cmd ..."
               * "case ... in <newline> word) ..."
               */
              if (*ctx.command).group.is_null()
                && (*ctx.command).argv.is_null()
                && (*ctx.command).redirects.is_null()
                && ctx.word.length == 0i32
                && ctx.word.has_quoted_part == 0
                && heredoc_cnt == 0i32
              {
                /* This newline can be ignored. But...
                 * Without check #1, interactive shell
                 * ignores even bare <newline>,
                 * and shows the continuation prompt:
                 * ps1_prompt$ <enter>
                 * ps2> _   <=== wrong, should be ps1
                 * Without check #2, "cmd & <newline>"
                 * is similarly mistreated.
                 * (BTW, this makes "cmd & cmd"
                 * and "cmd && cmd" non-orthogonal.
                 * Really, ask yourself, why
                 * "cmd && <newline>" doesn't start
                 * cmd but waits for more input?
                 * The only reason is that it might be
                 * a "cmd1 && <nl> cmd2 &" construct,
                 * cmd1 may need to run in BG).
                 */
                let mut pi_0: *mut pipe = ctx.list_head;
                if (*pi_0).num_cmds != 0i32
                  && (*pi_0).followup as libc::c_int != PIPE_BG as libc::c_int
                {
                  /* check #2 */
                  continue;
                }
              }
              /* Treat newline as a command separator. */
              done_pipe(&mut ctx, PIPE_SEQ);
              if heredoc_cnt != 0 {
                heredoc_cnt = fetch_heredocs(ctx.list_head, heredoc_cnt, input);
                if heredoc_cnt != 0i32 {
                  current_block = 1907364584679199995;
                  break;
                }
              }
              ctx.is_assignment = MAYBE_ASSIGNMENT as libc::c_int as smallint;
              ch = ';' as i32
              /* note: if (is_blank) continue;
               * will still trigger for us */
            }
          }
          /* "cmd}" or "cmd }..." without semicolon or &:
           * } is an ordinary char in this case, even inside { cmd; }
           * Pathological example: { ""}; } should exec "}" cmd
           */
          if ch == '}' as i32 {
            if ctx.word.length != 0i32 || ctx.word.has_quoted_part as libc::c_int != 0 {
              current_block = 18309173857163198886;
            } else if !((*ctx.command).group.is_null()
              && (*ctx.command).argv.is_null()
              && (*ctx.command).redirects.is_null())
            {
              /* cmd } */
              /* Generally, there should be semicolon: "cmd; }"
               * However, bash allows to omit it if "cmd" is
               * a group. Examples:
               * { { echo 1; } }
               * {(echo 1)}
               * { echo 0 >&2 | { echo 1; } }
               * { while false; do :; done }
               * { case a in b) ;; esac }
               */
              if !(*ctx.command).group.is_null() {
                current_block = 7518107371177619303;
              } else {
                current_block = 18309173857163198886;
              }
            } else if !((*ctx.pipe).num_cmds == 0i32
              && (*ctx.pipe).res_word as libc::c_int == RES_NONE as libc::c_int)
            {
              current_block = 13059973731526369978;
            } else {
              current_block = 7518107371177619303;
            }
          /* else: } does terminate a group */
          } else {
            current_block = 7518107371177619303;
          }
          match current_block {
            18309173857163198886 => {}
            _ => {
              match current_block {
                7518107371177619303 => {
                  if end_trigger != 0
                    && end_trigger == ch
                    && (ch != ';' as i32 || heredoc_cnt == 0i32)
                    && (ch != ')' as i32
                      || ctx.ctx_res_w as libc::c_int != RES_MATCH as libc::c_int
                      || ctx.word.has_quoted_part == 0
                        && strcmp(
                          ctx.word.data,
                          b"esac\x00" as *const u8 as *const libc::c_char,
                        ) == 0i32)
                  {
                    if done_word(&mut ctx) != 0 {
                      current_block = 1907364584679199995;
                      break;
                    }
                    done_pipe(&mut ctx, PIPE_SEQ);
                    ctx.is_assignment = MAYBE_ASSIGNMENT as libc::c_int as smallint;
                    /* Do we sit outside of any if's, loops or case's? */
                    if 1i32 == 0
                      || ctx.ctx_res_w as libc::c_int == RES_NONE as libc::c_int
                        && ctx.old_flag == 0i32
                    {
                      o_free_and_set_NULL(&mut ctx.word);
                      if ch != ';' as i32
                        && ((*ctx.list_head).num_cmds == 0i32
                          && (*ctx.list_head).res_word as libc::c_int == RES_NONE as libc::c_int)
                      {
                        /* Example: bare "{ }", "()" */
                        (*ptr_to_globals).last_exitcode = 2i32 as smalluint; /* bash compat */
                        syntax_error_unexpected_ch(ch);
                        current_block = 17066738228562814892;
                        break;
                      } else {
                        if !heredoc_cnt_ptr.is_null() {
                          *heredoc_cnt_ptr = heredoc_cnt
                        }
                        return ctx.list_head;
                      }
                    }
                  }
                  if !is_blank.is_null() {
                    continue;
                  }
                  /* Catch <, > before deciding whether this word is
                   * an assignment. a=1 2>z b=2: b=2 is still assignment */
                  match ch {
                    62 => {
                      current_block = 11456536786442373077;
                      match current_block {
                        8338824944507035481 => {
                          if ctx.word.length == 0i32 && ctx.word.has_quoted_part == 0 {
                            loop
                            /* skip "#comment" */
                            /* note: we do not add it to &ctx.as_string */
                            /* TODO: in bash:
                             * comment inside $() goes to the next \n, even inside quoted string (!):
                             * cmd "$(cmd2 #comment)" - syntax error
                             * cmd "`cmd2 #comment`" - ok
                             * We accept both (comment ends where command subst ends, in both cases).
                             */
                            {
                              ch = i_peek(input);
                              if ch == '\n' as i32 {
                                break;
                              }
                              ch = i_getch(input);
                              if ch == -1i32 {
                                break;
                              }
                            }
                            continue;
                            /* get next char */
                          }
                        }
                        11456536786442373077 => {
                          redir_fd = redirect_opt_num(&mut ctx.word); /* get next char */
                          if done_word(&mut ctx) != 0 {
                            current_block = 1907364584679199995; /* get next char */
                            break;
                          }
                          redir_style = REDIRECT_OVERWRITE;
                          if next == '>' as i32 {
                            redir_style = REDIRECT_APPEND;
                            ch = i_getch(input)
                          }
                          if parse_redirect(&mut ctx, redir_fd, redir_style, input) != 0 {
                            current_block = 1907364584679199995;
                            break;
                          }
                          continue;
                        }
                        _ => {
                          redir_fd = redirect_opt_num(&mut ctx.word);
                          if done_word(&mut ctx) != 0 {
                            current_block = 1907364584679199995;
                            break;
                          }
                          redir_style = REDIRECT_INPUT;
                          if next == '<' as i32 {
                            redir_style = REDIRECT_HEREDOC;
                            heredoc_cnt += 1;
                            ch = i_getch(input)
                          } else if next == '>' as i32 {
                            redir_style = REDIRECT_IO;
                            ch = i_getch(input)
                          }
                          if parse_redirect(&mut ctx, redir_fd, redir_style, input) != 0 {
                            current_block = 1907364584679199995;
                            break;
                          }
                          continue;
                        }
                      }
                    }
                    60 => {
                      current_block = 14429734548985272971;
                      match current_block {
                        8338824944507035481 => {
                          if ctx.word.length == 0i32 && ctx.word.has_quoted_part == 0 {
                            loop {
                              ch = i_peek(input);
                              if ch == '\n' as i32 {
                                break;
                              }
                              ch = i_getch(input);
                              if ch == -1i32 {
                                break;
                              }
                            }
                            continue;
                          }
                        }
                        11456536786442373077 => {
                          redir_fd = redirect_opt_num(&mut ctx.word);
                          if done_word(&mut ctx) != 0 {
                            current_block = 1907364584679199995;
                            break;
                          }
                          redir_style = REDIRECT_OVERWRITE;
                          if next == '>' as i32 {
                            redir_style = REDIRECT_APPEND;
                            ch = i_getch(input)
                          }
                          if parse_redirect(&mut ctx, redir_fd, redir_style, input) != 0 {
                            current_block = 1907364584679199995;
                            break;
                          }
                          continue;
                        }
                        _ => {
                          redir_fd = redirect_opt_num(&mut ctx.word);
                          if done_word(&mut ctx) != 0 {
                            current_block = 1907364584679199995;
                            break;
                          }
                          redir_style = REDIRECT_INPUT;
                          if next == '<' as i32 {
                            redir_style = REDIRECT_HEREDOC;
                            heredoc_cnt += 1;
                            ch = i_getch(input)
                          } else if next == '>' as i32 {
                            redir_style = REDIRECT_IO;
                            ch = i_getch(input)
                          }
                          if parse_redirect(&mut ctx, redir_fd, redir_style, input) != 0 {
                            current_block = 1907364584679199995;
                            break;
                          }
                          continue;
                        }
                      }
                    }
                    35 => {
                      current_block = 8338824944507035481;
                      match current_block {
                        8338824944507035481 => {
                          if ctx.word.length == 0i32 && ctx.word.has_quoted_part == 0 {
                            loop {
                              ch = i_peek(input);
                              if ch == '\n' as i32 {
                                break;
                              }
                              ch = i_getch(input);
                              if ch == -1i32 {
                                break;
                              }
                            }
                            continue;
                          }
                        }
                        11456536786442373077 => {
                          redir_fd = redirect_opt_num(&mut ctx.word);
                          if done_word(&mut ctx) != 0 {
                            current_block = 1907364584679199995;
                            break;
                          }
                          redir_style = REDIRECT_OVERWRITE;
                          if next == '>' as i32 {
                            redir_style = REDIRECT_APPEND;
                            ch = i_getch(input)
                          }
                          if parse_redirect(&mut ctx, redir_fd, redir_style, input) != 0 {
                            current_block = 1907364584679199995;
                            break;
                          }
                          continue;
                        }
                        _ => {
                          redir_fd = redirect_opt_num(&mut ctx.word);
                          if done_word(&mut ctx) != 0 {
                            current_block = 1907364584679199995;
                            break;
                          }
                          redir_style = REDIRECT_INPUT;
                          if next == '<' as i32 {
                            redir_style = REDIRECT_HEREDOC;
                            heredoc_cnt += 1;
                            ch = i_getch(input)
                          } else if next == '>' as i32 {
                            redir_style = REDIRECT_IO;
                            ch = i_getch(input)
                          }
                          if parse_redirect(&mut ctx, redir_fd, redir_style, input) != 0 {
                            current_block = 1907364584679199995;
                            break;
                          }
                          continue;
                        }
                      }
                    }
                    _ => {}
                  }
                }
                _ => {}
              }
              /* cmd | } */
              /* Can't be an end of {cmd}, skip the check */
              if ctx.is_assignment as libc::c_int == MAYBE_ASSIGNMENT as libc::c_int
                && ctx.pending_redirect.is_null()
              {
                /* ch is a special char and thus this word
                 * cannot be an assignment */
                ctx.is_assignment = NOT_ASSIGNMENT as libc::c_int as smallint
              }
              /* Note: nommu_addchr(&ctx.as_string, ch) is already done */
              match ch {
                                    3 => {
                                        /* Convert raw ^C to corresponding special variable reference */
                                        o_addchr(&mut ctx.word,
                                                 3i32); /* get next char */
                                        o_addchr(&mut ctx.word,
                                                 1i32); /* eat second " */
                                        current_block = 11704709843737693330;
                                    }
                                    35 => {
                                        current_block = 11704709843737693330;
                                    }
                                    36 => {
                                        if parse_dollar(&mut ctx.word, input,
                                                        0i32 as libc::c_uchar)
                                               == 0 {
                                            current_block =
                                                1907364584679199995;
                                            break ;
                                        }
                                        continue ;
                                    }
                                    34 => {
                                        ctx.word.has_quoted_part =
                                            1i32 as smallint;
                                        if next == '\"' as i32 &&
                                               ctx.pending_redirect.is_null()
                                           {
                                            i_getch(input);
                                            /* get next char */
                                        } else {
                                            if ctx.is_assignment as
                                                   libc::c_int ==
                                                   NOT_ASSIGNMENT as
                                                       libc::c_int {
                                                ctx.word.o_expflags |=
                                                    EXP_FLAG_ESC_GLOB_CHARS as
                                                        libc::c_int
                                            } /* get next char */
                                            if encode_string(&mut ctx.word,
                                                             input,
                                                             '\"' as i32) == 0
                                               {
                                                current_block =
                                                    1907364584679199995;
                                                break ;
                                            }
                                            ctx.word.o_expflags &=
                                                !(EXP_FLAG_ESC_GLOB_CHARS as
                                                      libc::c_int);
                                            continue ;
                                        }
                                        current_block = 2277602629737488951;
                                    }
                                    96 => {
                                        o_addchr(&mut ctx.word, 3i32);
                                        o_addchr(&mut ctx.word, '`' as i32);
                                        if add_till_backquote(&mut ctx.word,
                                                              input, 0i32) ==
                                               0 {
                                            current_block =
                                                1907364584679199995;
                                            break ;
                                        }
                                        o_addchr(&mut ctx.word, 3i32);
                                        /* get next char */
                                        continue ;
                                    }
                                    59 => {
                                        current_block = 4562139175239303272;
                                    }
                                    38 => {
                                        if done_word(&mut ctx) != 0 {
                                            current_block =
                                                1907364584679199995;
                                            break ;
                                        }
                                        if next == '&' as i32 {
                                            ch = i_getch(input);
                                            done_pipe(&mut ctx, PIPE_AND);
                                        } else {
                                            done_pipe(&mut ctx, PIPE_BG);
                                        }
                                        current_block = 14669516692233869547;
                                    }
                                    124 => {
                                        if done_word(&mut ctx) != 0 {
                                            current_block =
                                                1907364584679199995;
                                            break ;
                                        }
                                        //debug_printf_subst("SUBST RES3 '%s'\n", ctx.word.data + pos);
                                        if ctx.ctx_res_w as libc::c_int ==
                                               RES_MATCH as libc::c_int {
                                            continue
                                                ; /* we are in case's "word | word)" */
                                        }
                                        if next == '|' as i32 {
                                            /* || */
                                            ch = i_getch(input);
                                            done_pipe(&mut ctx, PIPE_OR);
                                        } else {
                                            /* we could pick up a file descriptor choice here
				 * with redirect_opt_num(), but bash doesn't do it.
				 * "echo foo 2| cat" yields "foo 2". */
                                            done_command(&mut ctx);
                                        }
                                        current_block = 14669516692233869547;
                                    }
                                    40 => {
                                        /* "case... in [(]word)..." - skip '(' */
                                        if ctx.ctx_res_w as libc::c_int ==
                                               RES_MATCH as libc::c_int &&
                                               (*ctx.command).argv.is_null()
                                               && ctx.word.length == 0i32 &&
                                               ctx.word.has_quoted_part as
                                                   libc::c_int == 0i32 {
                                            continue ;
                                        }
                                        current_block = 613454377845503748;
                                    }
                                    123 => {
                                        current_block = 613454377845503748;
                                    }
                                    41 => {
                                        if ctx.ctx_res_w as libc::c_int ==
                                               RES_MATCH as libc::c_int {
                                            current_block =
                                                4562139175239303272;
                                        } else {
                                            current_block =
                                                7912797836817091754;
                                        }
                                    }
                                    125 => {
                                        current_block = 7912797836817091754;
                                    }
                                    _ => {
                                        bb_error_msg_and_die(b"BUG: unexpected %c\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             ch);
                                    }
                                }
              match current_block {
                2277602629737488951 => {}
                _ => {
                  match current_block {
                    4562139175239303272 => {
                      if done_word(&mut ctx) != 0 {
                        current_block = 1907364584679199995;
                        break;
                      }
                      done_pipe(&mut ctx, PIPE_SEQ);
                      loop
                      /* Eat multiple semicolons, detect
                       * whether it means something special */
                      {
                        ch = i_peek_and_eat_bkslash_nl(input);
                        if ch != ';' as i32 {
                          break;
                        }
                        ch = i_getch(input);
                        if !(ctx.ctx_res_w as libc::c_int == RES_CASE_BODY as libc::c_int) {
                          continue;
                        }
                        ctx.ctx_dsemicolon = 1i32 as smallint;
                        ctx.ctx_res_w = RES_MATCH as libc::c_int as smallint;
                        break;
                      }
                    }
                    11704709843737693330 =>
                    /* fall through */
                    /* non-comment #: "echo a#b" etc */
                    {
                      o_addchr(&mut ctx.word, ch); /* get next char */
                      continue;
                    }
                    613454377845503748 =>
                    /* fall through */
                    {
                      let mut n: libc::c_int = parse_group(&mut ctx, input, ch);
                      if n < 0i32 {
                        current_block = 1907364584679199995;
                        break;
                      }
                      heredoc_cnt += n
                    }
                    7912797836817091754 => {
                      /* proper use of this character is caught by end_trigger:
                       * if we see {, we call parse_group(..., end_trigger='}')
                       * and it will match } earlier (not here). */
                      (*ptr_to_globals).last_exitcode = 2i32 as smalluint;
                      syntax_error_unexpected_ch(ch);
                      current_block = 17066738228562814892;
                      break;
                    }
                    _ => {}
                  }
                  /* We just finished a cmd. New one may start
                   * with an assignment */
                  ctx.is_assignment = MAYBE_ASSIGNMENT as libc::c_int as smallint; /* get next char */
                  continue;
                }
              }
            }
          }
        }
        match current_block {
          2277602629737488951 => {}
          _ =>
          /* ""} */
          {
            o_addQchr(&mut ctx.word, ch);
            if (ctx.is_assignment as libc::c_int == MAYBE_ASSIGNMENT as libc::c_int
              || ctx.is_assignment as libc::c_int == WORD_IS_KEYWORD as libc::c_int)
              && ch == '=' as i32
              && *endofname(ctx.word.data).offset(0) as libc::c_int == '=' as i32
            {
              ctx.is_assignment = DEFINITELY_ASSIGNMENT as libc::c_int as smallint
            }
            continue;
          }
        }
      }
      o_addchr(&mut ctx.word, 3i32);
      o_addchr(&mut ctx.word, 3i32);
    }
  }
  match current_block {
    1907364584679199995 => (*ptr_to_globals).last_exitcode = 1i32 as smalluint,
    _ => {}
  }
  let mut pctx: *mut parse_context = 0 as *mut parse_context;
  let mut p2: *mut parse_context = 0 as *mut parse_context;
  /* Clean up allocated tree.
   * Sample for finding leaks on syntax error recovery path.
   * Run it from interactive shell, watch pmap `pidof hush`.
   * while if false; then false; fi; do break; fi
   * Samples to catch leaks at execution:
   * while if (true | { true;}); then echo ok; fi; do break; done
   * while if (true | { true;}); then echo ok; fi; do (if echo ok; break; then :; fi) | cat; break; done
   */
  pctx = &mut ctx;
  loop {
    /* Update pipe/command counts,
     * otherwise freeing may miss some */
    done_pipe(pctx, PIPE_SEQ);
    free_pipe_list((*pctx).list_head);
    p2 = (*pctx).stack;
    if pctx != &mut ctx as *mut parse_context {
      free(pctx as *mut libc::c_void);
    }
    pctx = p2;
    if !(1i32 != 0 && !pctx.is_null()) {
      break;
    }
  }
  o_free(&mut ctx.word);
  return 1i32 as libc::c_long as *mut libc::c_void as *mut pipe;
}
/* expand_strvec_to_strvec() takes a list of strings, expands
 * all variable references within and returns a pointer to
 * a list of expanded strings, possibly with larger number
 * of strings. (Think VAR="a b"; echo $VAR).
 * This new list is allocated as a single malloc block.
 * NULL-terminated list of char* pointers is at the beginning of it,
 * followed by strings themselves.
 * Caller can deallocate entire list by single free(list). */
/* A horde of its helpers come first: */
unsafe extern "C" fn o_addblock_duplicate_backslash(
  mut o: *mut o_string,
  mut str: *const libc::c_char,
  mut len: libc::c_int,
) {
  loop {
    len -= 1;
    if !(len >= 0i32) {
      break;
    }
    let fresh20 = str;
    str = str.offset(1);
    let mut c: libc::c_char = *fresh20;
    if c as libc::c_int == '{' as i32 || c as libc::c_int == '}' as i32 {
      /* { -> \{, } -> \} */
      o_addchr(o, '\\' as i32);
      /* And now we want to add { or } and continue:
       *  o_addchr(o, c);
       *  continue;
       * luckily, just falling through achieves this.
       */
    }
    o_addchr(o, c as libc::c_int);
    if c as libc::c_int == '\\' as i32 {
      /* \z -> \\\z; \<eol> -> \\<eol> */
      o_addchr(o, '\\' as i32);
      if len != 0 {
        len -= 1;
        o_addchr(o, '\\' as i32);
        let fresh21 = str;
        str = str.offset(1);
        o_addchr(o, *fresh21 as libc::c_int);
      }
    }
  }
}
/* Store given string, finalizing the word and starting new one whenever
 * we encounter IFS char(s). This is used for expanding variable values.
 * End-of-string does NOT finalize word: think about 'echo -$VAR-'.
 * Return in output->ended_in_ifs:
 * 1 - ended with IFS char, else 0 (this includes case of empty str).
 */
unsafe extern "C" fn expand_on_ifs(
  mut output: *mut o_string,
  mut n: libc::c_int,
  mut str: *const libc::c_char,
) -> libc::c_int {
  let mut last_is_ifs: libc::c_int = 0i32;
  loop {
    let mut word_len: libc::c_int = 0;
    if *str == 0 {
      /* EOL - do not finalize word */
      break;
    } else {
      word_len = strcspn(str, (*ptr_to_globals).ifs) as libc::c_int;
      if word_len != 0 {
        /* We have WORD_LEN leading non-IFS chars */
        if (*output).o_expflags & EXP_FLAG_GLOB as libc::c_int == 0 {
          o_addblock(output, str, word_len);
        } else {
          /* Protect backslashes against globbing up :)
           * Example: "v='\*'; echo b$v" prints "b\*"
           * (and does not try to glob on "*")
           */
          o_addblock_duplicate_backslash(output, str, word_len);
          /*/ Why can't we do it easier? */
          /*o_addblock(output, str, word_len); - WRONG: "v='\*'; echo Z$v" prints "Z*" instead of "Z\*" */
          /*o_addqblock(output, str, word_len); - WRONG: "v='*'; echo Z$v" prints "Z*" instead of Z* files */
        }
        last_is_ifs = 0i32;
        str = str.offset(word_len as isize);
        if *str == 0 {
          /* EOL - do not finalize word */
          break;
        }
      }
      /* We know str here points to at least one IFS char */
      last_is_ifs = 1i32; /* skip IFS whitespace chars */
      str = str.offset(strspn(str, (*ptr_to_globals).ifs_whitespace) as isize);
      if *str == 0 {
        break;
      }
      if (*ptr_to_globals).ifs_whitespace != (*ptr_to_globals).ifs as *mut libc::c_char
        && !strchr((*ptr_to_globals).ifs, *str as libc::c_int).is_null()
      {
        /* the second check would fail */
        /* This is a non-whitespace $IFS char */
        /* Skip it and IFS whitespace chars, start new word */
        str = str.offset(1);
        str = str.offset(strspn(str, (*ptr_to_globals).ifs_whitespace) as isize)
      } else if !((*output).has_quoted_part as libc::c_int != 0
        || *(*output).data.offset(((*output).length - 1i32) as isize) as libc::c_int != 0)
      /* Start new word... but not always! */
      /* Case "v=' a'; echo ''$v": we do need to finalize empty word: */
      {
        continue;
      }
      o_addchr(output, '\u{0}' as i32);
      n = o_save_ptr(output, n)
    }
  }
  (*output).ended_in_ifs = last_is_ifs as smallint;
  return n;
}
/* Helper to expand $((...)) and heredoc body. These act as if
 * they are in double quotes, with the exception that they are not :).
 * Just the rules are similar: "expand only $var and `cmd`"
 *
 * Returns malloced string.
 * As an optimization, we return NULL if expansion is not needed.
 */
unsafe extern "C" fn encode_then_expand_string(mut str: *const libc::c_char) -> *mut libc::c_char {
  let mut exp_str: *mut libc::c_char = 0 as *mut libc::c_char; /* string has no special chars */
  let mut input: in_str = in_str {
    p: 0 as *const libc::c_char,
    peek_buf: [0; 2],
    last_char: 0,
    file: 0 as *mut HFILE,
  };
  let mut dest: o_string = {
    let mut init = o_string {
      data: 0 as *mut libc::c_char,
      length: 0,
      maxlen: 0,
      o_expflags: 0,
      has_quoted_part: 0,
      has_empty_slot: 0,
      ended_in_ifs: 0,
    };
    init
  };
  let mut cp: *const libc::c_char = 0 as *const libc::c_char;
  cp = str;
  loop {
    if *cp == 0 {
      return 0 as *mut libc::c_char;
    }
    if *cp as libc::c_int == '$' as i32 {
      break;
    }
    if *cp as libc::c_int == '\\' as i32 {
      break;
    }
    if *cp as libc::c_int == '`' as i32 {
      break;
    }
    cp = cp.offset(1)
  }
  /* We need to expand. Example:
   * echo $(($a + `echo 1`)) $((1 + $((2)) ))
   */
  setup_string_in_str(&mut input, str);
  encode_string(&mut dest, &mut input, -1i32);
  //TODO: error check (encode_string returns 0 on error)?
  //bb_error_msg("'%s' -> '%s'", str, dest.data);
  exp_str = expand_string_to_string(dest.data, EXP_FLAG_ESC_GLOB_CHARS as libc::c_int, 1i32);
  //bb_error_msg("'%s' -> '%s'", dest.data, exp_str);
  o_free(&mut dest); /* string has no special chars */
  return exp_str;
}
unsafe extern "C" fn first_special_char_in_vararg(
  mut cp: *const libc::c_char,
) -> *const libc::c_char {
  loop {
    if *cp == 0 {
      return 0 as *const libc::c_char;
    }
    if *cp as libc::c_int == '$' as i32 {
      return cp;
    }
    if *cp as libc::c_int == '\\' as i32 {
      return cp;
    }
    if *cp as libc::c_int == '\'' as i32 {
      return cp;
    }
    if *cp as libc::c_int == '\"' as i32 {
      return cp;
    }
    if *cp as libc::c_int == '`' as i32 {
      return cp;
    }
    /* dquoted "${x:+ARG}" should not glob, therefore
     * '*' et al require some non-literal processing: */
    if *cp as libc::c_int == '*' as i32 {
      return cp;
    }
    if *cp as libc::c_int == '?' as i32 {
      return cp;
    }
    if *cp as libc::c_int == '[' as i32 {
      return cp;
    }
    cp = cp.offset(1)
  }
}
/* Expanding ARG in ${var#ARG}, ${var%ARG}, or ${var/ARG/ARG}.
 * These can contain single- and double-quoted strings,
 * and treated as if the ARG string is initially unquoted. IOW:
 * ${var#ARG} and "${var#ARG}" treat ARG the same (ARG can even be
 * a dquoted string: "${var#"zz"}"), the difference only comes later
 * (word splitting and globbing of the ${var...} result).
 */
unsafe extern "C" fn encode_then_expand_vararg(
  mut str: *const libc::c_char,
  mut handle_squotes: libc::c_int,
  mut do_unbackslash: libc::c_int,
) -> *mut libc::c_char {
  let mut current_block: u64;
  let mut exp_str: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut input: in_str = in_str {
    p: 0 as *const libc::c_char,
    peek_buf: [0; 2],
    last_char: 0,
    file: 0 as *mut HFILE,
  };
  let mut dest: o_string = {
    let mut init = o_string {
      data: 0 as *mut libc::c_char,
      length: 0,
      maxlen: 0,
      o_expflags: 0,
      has_quoted_part: 0,
      has_empty_slot: 0,
      ended_in_ifs: 0,
    };
    init
  };
  if first_special_char_in_vararg(str).is_null() {
    /* string has no special chars */
    return 0 as *mut libc::c_char;
  } /* start as "", not as NULL */
  setup_string_in_str(&mut input, str);
  dest.data = xzalloc(1i32 as size_t) as *mut libc::c_char;
  exp_str = 0 as *mut libc::c_char;
  /* error */
  loop
  //debug_printf_subst("SUBST RES3 '%s'\n", dest->data + pos);
  {
    let mut ch: libc::c_int = 0; /* error */
    ch = i_getch(&mut input);
    if dest.o_expflags == 0 {
      if ch == -1i32 {
        current_block = 16799951812150840583;
        break;
      }
      if handle_squotes != 0 && ch == '\'' as i32 {
        if add_till_single_quote_dquoted(&mut dest, &mut input) == 0 {
          current_block = 6476622998065200121;
          break;
        } else {
          continue;
        }
      }
    }
    if ch == -1i32 {
      syntax_error_unterm_ch('\"' as i32 as libc::c_char);
      current_block = 6476622998065200121;
      break;
    /* error */
    } else if ch == '\"' as i32 {
      dest.o_expflags ^= EXP_FLAG_ESC_GLOB_CHARS as libc::c_int
    } else if ch == '\\' as i32 {
      ch = i_getch(&mut input);
      if ch == -1i32 {
        //example? error message?	syntax_error_unterm_ch('"');
        current_block = 6476622998065200121;
        break;
      } else {
        o_addqchr(&mut dest, ch);
      }
    } else if ch == '$' as i32 {
      if parse_dollar(&mut dest, &mut input, 0x80i32 as libc::c_uchar) == 0 {
        current_block = 6476622998065200121;
        break;
      }
    } else if ch == '`' as i32 {
      //unsigned pos = dest->length;
      o_addchr(&mut dest, 3i32);
      o_addchr(&mut dest, 0x80i32 | '`' as i32);
      if add_till_backquote(&mut dest, &mut input, dest.o_expflags) == 0 {
        current_block = 6476622998065200121;
        break;
      }
      o_addchr(&mut dest, 3i32);
    } else {
      o_addQchr(&mut dest, ch);
    }
  }
  match current_block {
    16799951812150840583 => {
      exp_str = expand_string_to_string(
        dest.data,
        if do_unbackslash != 0 {
          EXP_FLAG_ESC_GLOB_CHARS as libc::c_int
        } else {
          0i32
        },
        do_unbackslash,
      )
    }
    _ => {}
  }
  o_free(&mut dest);
  return exp_str;
}
/* Expanding ARG in ${var+ARG}, ${var-ARG}
 */
unsafe extern "C" fn encode_then_append_var_plusminus(
  mut output: *mut o_string,
  mut n: libc::c_int,
  mut str: *mut libc::c_char,
  mut dquoted: libc::c_int,
) -> libc::c_int {
  let mut current_block: u64;
  let mut input: in_str = in_str {
    p: 0 as *const libc::c_char,
    peek_buf: [0; 2],
    last_char: 0,
    file: 0 as *mut HFILE,
  };
  let mut dest: o_string = {
    let mut init = o_string {
      data: 0 as *mut libc::c_char,
      length: 0,
      maxlen: 0,
      o_expflags: 0,
      has_quoted_part: 0,
      has_empty_slot: 0,
      ended_in_ifs: 0,
    };
    init
  };
  if first_special_char_in_vararg(str).is_null()
    && '\u{0}' as i32 == *str.offset(strcspn(str, (*ptr_to_globals).ifs) as isize) as libc::c_int
  {
    /* string has no special chars
     * && string has no $IFS chars
     */
    if dquoted != 0 {
      /* Prints 1 (quoted expansion is a "" word, not nothing):
       * set -- "${notexist-}"; echo $#
       */
      (*output).has_quoted_part = 1i32 as smallint
    }
    return expand_vars_to_list(output, n, str);
  }
  setup_string_in_str(&mut input, str);
  /* error */
  loop
  //debug_printf_subst("SUBST RES3 '%s'\n", dest->data + pos);
  {
    let mut ch: libc::c_int = 0;
    ch = i_getch(&mut input);
    if dest.o_expflags == 0 {
      if ch == -1i32 {
        current_block = 14001958660280927786;
        break;
      }
      if dquoted == 0 && !strchr((*ptr_to_globals).ifs, ch).is_null() {
        /* PREFIX${x:d${e}f ...} and we met space: expand "d${e}f" and start new word.
         * do not assume we are at the start of the word (PREFIX above).
         */
        if !dest.data.is_null() {
          n = expand_vars_to_list(output, n, dest.data);
          o_free_and_set_NULL(&mut dest);
          o_addchr(output, '\u{0}' as i32);
          n = o_save_ptr(output, n)
        /* create next word */
        } else if (*output).length != o_get_last_ptr(output, n)
          || (*output).has_quoted_part as libc::c_int != 0
        {
          /* For these cases:
           * f() { for i; do echo "|$i|"; done; }; x=x
           * f a${x:+ }b  # 1st condition
           * |a|
           * |b|
           * f ""${x:+ }b  # 2nd condition
           * ||
           * |b|
           */
          o_addchr(output, '\u{0}' as i32);
          n = o_save_ptr(output, n)
          /* create next word */
        } /* error */
        continue;
      } else if dquoted == 0 && ch == '\'' as i32 {
        if add_till_single_quote_dquoted(&mut dest, &mut input) == 0 {
          current_block = 13928041799263361356;
          break;
        }
        o_addchr(&mut dest, 3i32);
        o_addchr(&mut dest, 3i32);
        continue;
      }
    }
    if ch == -1i32 {
      syntax_error_unterm_ch('\"' as i32 as libc::c_char);
      current_block = 13928041799263361356;
      break;
    /* error */
    } else if ch == '\"' as i32 {
      dest.o_expflags ^= EXP_FLAG_ESC_GLOB_CHARS as libc::c_int;
      if dest.o_expflags != 0 {
        o_addchr(&mut dest, 3i32);
        o_addchr(&mut dest, 3i32);
      }
    } else if ch == '\\' as i32 {
      ch = i_getch(&mut input);
      if ch == -1i32 {
        //example? error message?	syntax_error_unterm_ch('"');
        current_block = 13928041799263361356;
        break;
      } else {
        o_addqchr(&mut dest, ch);
      }
    } else if ch == '$' as i32 {
      if parse_dollar(
        &mut dest,
        &mut input,
        if dest.o_expflags != 0 || dquoted != 0 {
          0x80i32
        } else {
          0i32
        } as libc::c_uchar,
      ) == 0
      {
        current_block = 13928041799263361356;
        break;
      }
    } else if ch == '`' as i32 {
      //unsigned pos = dest->length;
      o_addchr(&mut dest, 3i32);
      o_addchr(
        &mut dest,
        if dest.o_expflags != 0 || dquoted != 0 {
          (0x80i32) | '`' as i32
        } else {
          '`' as i32
        },
      );
      if add_till_backquote(&mut dest, &mut input, dest.o_expflags) == 0 {
        current_block = 13928041799263361356;
        break;
      }
      o_addchr(&mut dest, 3i32);
    } else if dquoted != 0 {
      /* Always glob-protect if in dquotes:
       * x=x; echo "${x:+/bin/c*}" - prints: /bin/c*
       * x=x; echo "${x:+"/bin/c*"}" - prints: /bin/c*
       */
      o_addqchr(&mut dest, ch);
    } else {
      /* Glob-protect only if char is quoted:
       * x=x; echo ${x:+/bin/c*} - prints many filenames
       * x=x; echo ${x:+"/bin/c*"} - prints: /bin/c*
       */
      o_addQchr(&mut dest, ch);
    }
  }
  match current_block {
    14001958660280927786 => {
      if !dest.data.is_null() {
        n = expand_vars_to_list(output, n, dest.data)
      }
    }
    _ => {}
  }
  o_free(&mut dest);
  return n;
}
unsafe extern "C" fn expand_and_evaluate_arith(
  mut arg: *const libc::c_char,
  mut errmsg_p: *mut *const libc::c_char,
) -> arith_t {
  let mut math_state: arith_state_t = arith_state_t {
    errmsg: 0 as *const libc::c_char,
    lookupvar: None,
    setvar: None,
    list_of_recursed_names: 0 as *mut libc::c_void,
  };
  let mut res: arith_t = 0;
  let mut exp_str: *mut libc::c_char = 0 as *mut libc::c_char;
  math_state.lookupvar = Some(
    get_local_var_value as unsafe extern "C" fn(_: *const libc::c_char) -> *const libc::c_char,
  );
  math_state.setvar = Some(
    set_local_var_from_halves
      as unsafe extern "C" fn(_: *const libc::c_char, _: *const libc::c_char) -> (),
  );
  //math_state.endofname = endofname;
  exp_str = encode_then_expand_string(arg);
  res = arith(
    &mut math_state,
    if !exp_str.is_null() { exp_str } else { arg },
  );
  free(exp_str as *mut libc::c_void);
  if !errmsg_p.is_null() {
    *errmsg_p = math_state.errmsg
  }
  if !math_state.errmsg.is_null() {
    msg_and_die_if_script(math_state.errmsg);
  }
  return res;
}
/* ${var/[/]pattern[/repl]} helpers */
unsafe extern "C" fn strstr_pattern(
  mut val: *mut libc::c_char,
  mut pattern: *const libc::c_char,
  mut size: *mut libc::c_int,
) -> *mut libc::c_char {
  loop {
    let mut end: *mut libc::c_char = scan_and_match(
      val,
      pattern,
      (SCAN_MOVE_FROM_RIGHT as libc::c_int + SCAN_MATCH_LEFT_HALF as libc::c_int) as libc::c_uint,
    );
    if !end.is_null() {
      *size = end.wrapping_offset_from(val) as libc::c_long as libc::c_int;
      return val;
    }
    if *val as libc::c_int == '\u{0}' as i32 {
      return 0 as *mut libc::c_char;
    }
    /* Optimization: if "*pat" did not match the start of "string",
     * we know that "tring", "ring" etc will not match too:
     */
    if *pattern.offset(0) as libc::c_int == '*' as i32 {
      return 0 as *mut libc::c_char;
    }
    val = val.offset(1)
  }
}
unsafe extern "C" fn replace_pattern(
  mut val: *mut libc::c_char,
  mut pattern: *const libc::c_char,
  mut repl: *const libc::c_char,
  mut exp_op: libc::c_char,
) -> *mut libc::c_char {
  let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut res_len: libc::c_uint = 0i32 as libc::c_uint;
  let mut repl_len: libc::c_uint = strlen(repl) as libc::c_uint;
  /* Null pattern never matches, including if "var" is empty */
  if *pattern.offset(0) == 0 {
    return result;
  } /* NULL, no replaces happened */
  loop {
    let mut size: libc::c_int = 0;
    let mut s: *mut libc::c_char = strstr_pattern(val, pattern, &mut size);
    if s.is_null() {
      break;
    }
    result = xrealloc(
      result as *mut libc::c_void,
      (res_len as libc::c_long
        + s.wrapping_offset_from(val) as libc::c_long
        + repl_len as libc::c_long
        + 1i32 as libc::c_long) as size_t,
    ) as *mut libc::c_char;
    strcpy(
      mempcpy(
        result.offset(res_len as isize) as *mut libc::c_void,
        val as *const libc::c_void,
        s.wrapping_offset_from(val) as libc::c_long as size_t,
      ) as *mut libc::c_char,
      repl,
    );
    res_len = (res_len as libc::c_long
      + (s.wrapping_offset_from(val) as libc::c_long + repl_len as libc::c_long))
      as libc::c_uint;
    val = s.offset(size as isize);
    if exp_op as libc::c_int == '/' as i32 {
      break;
    }
  }
  if *val as libc::c_int != 0 && !result.is_null() {
    result = xrealloc(
      result as *mut libc::c_void,
      (res_len as libc::c_ulong)
        .wrapping_add(strlen(val))
        .wrapping_add(1i32 as libc::c_ulong),
    ) as *mut libc::c_char;
    strcpy(result.offset(res_len as isize), val);
  }
  return result;
}
/* BASH_PATTERN_SUBST */
unsafe extern "C" fn append_str_maybe_ifs_split(
  mut output: *mut o_string,
  mut n: libc::c_int,
  mut first_ch: libc::c_int,
  mut val: *const libc::c_char,
) -> libc::c_int {
  if first_ch & 0x80i32 == 0 {
    if !val.is_null() && *val.offset(0) as libc::c_int != 0 {
      n = expand_on_ifs(output, n, val)
    }
  } else {
    (*output).has_quoted_part = 1i32 as smallint; /* quoted "$VAR" */
    if !val.is_null() && *val.offset(0) as libc::c_int != 0 {
      o_addQstr(output, val);
    }
  }
  return n;
}
/* Handle <SPECIAL_VAR_SYMBOL>varname...<SPECIAL_VAR_SYMBOL> construct.
 */
#[inline(never)]
unsafe extern "C" fn expand_one_var(
  mut output: *mut o_string,
  mut n: libc::c_int,
  mut first_ch: libc::c_int,
  mut arg: *mut libc::c_char,
  mut pp: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut current_block: u64; /* for compiler */
  let mut val: *const libc::c_char = 0 as *const libc::c_char; /* points to expansion operator */
  let mut to_be_freed: *mut libc::c_char = 0 as *mut libc::c_char; /* for compiler */
  let mut p: *mut libc::c_char = 0 as *mut libc::c_char; /* replace trailing SPECIAL_VAR_SYMBOL */
  let mut var: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut exp_op: libc::c_char = 0;
  let mut exp_save: libc::c_char = 0;
  exp_save = exp_save;
  let mut exp_saveptr: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut exp_word: *mut libc::c_char = 0 as *mut libc::c_char;
  exp_word = exp_word;
  let mut arg0: libc::c_char = 0;
  val = 0 as *const libc::c_char;
  to_be_freed = 0 as *mut libc::c_char;
  p = *pp;
  *p = '\u{0}' as i32 as libc::c_char;
  var = arg;
  exp_saveptr = if *arg.offset(1) as libc::c_int != 0 {
    strchr(
      b"\\/%#:-=+?\x00" as *const u8 as *const libc::c_char,
      *arg.offset(1) as libc::c_int,
    )
  } else {
    0 as *mut libc::c_char
  };
  arg0 = *arg.offset(0);
  *arg.offset(0) = (arg0 as libc::c_int & 0x7fi32) as libc::c_char;
  exp_op = 0i32 as libc::c_char;
  if *arg.offset(0) as libc::c_int == '#' as i32
    && *arg.offset(1) as libc::c_int != 0
    && (exp_saveptr.is_null()
      || *arg.offset(2) as libc::c_int == '\u{0}' as i32
        && !strchr(
          (b"_*@$!?#-\x00" as *const u8 as *const libc::c_char).offset(1),
          *arg.offset(1) as libc::c_int,
        )
        .is_null())
  {
    /* NB: skipping ^^^specvar check mishandles ${#::2} */
    /* It must be length operator: ${#var} */
    var = var.offset(1);
    exp_op = 'L' as i32 as libc::c_char
  } else {
    /* Maybe handle parameter expansion */
    if !exp_saveptr.is_null()
      && !strchr(
        (b"_*@$!?#-\x00" as *const u8 as *const libc::c_char).offset(3),
        *arg.offset(0) as libc::c_int,
      )
      .is_null()
    {
      /* 1st char is special variable */
      /* ${?:0}, ${#[:]%0} etc */
      exp_saveptr = var.offset(1)
    } else {
      /* ${?}, ${var}, ${var:0}, ${var[:]%0} etc */
      exp_saveptr = var.offset(1).offset(strcspn(
        var.offset(1),
        b"\\/%#:-=+?\x00" as *const u8 as *const libc::c_char,
      ) as isize)
    }
    exp_save = *exp_saveptr;
    exp_op = exp_save;
    if exp_op != 0 {
      exp_word = exp_saveptr.offset(1);
      if exp_op as libc::c_int == ':' as i32 {
        let fresh22 = exp_word;
        exp_word = exp_word.offset(1);
        exp_op = *fresh22;
        /* else: it's not an expansion op, but bare ${var} */
        //TODO: try ${var:} and ${var:bogus} in non-bash config
        if 1i32 != 0
          && (exp_op == 0
            || strchr(
              (b"\\/%#:-=+?\x00" as *const u8 as *const libc::c_char).offset(5),
              exp_op as libc::c_int,
            )
            .is_null())
        {
          /* oops... it's ${var:N[:M]}, not ${var:?xxx} or some such */
          exp_op = ':' as i32 as libc::c_char;
          exp_word = exp_word.offset(-1)
        }
      }
      *exp_saveptr = '\u{0}' as i32 as libc::c_char
    }
  }
  /* Look up the variable in question */
  if (*var.offset(0) as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32 {
    /* parse_dollar should have vetted var for us */
    let mut nn: libc::c_int = xatoi_positive(var);
    if nn < (*ptr_to_globals).global_argc {
      val = *(*ptr_to_globals).global_argv.offset(nn as isize)
    }
  } else {
    match *var.offset(0) as libc::c_int {
      36 => {
        /* pid */
        val = utoa((*ptr_to_globals).root_pid as libc::c_uint)
      }
      33 => {
        /* bg pid */
        val = if (*ptr_to_globals).last_bg_pid != 0 {
          utoa((*ptr_to_globals).last_bg_pid as libc::c_uint)
        } else {
          b"\x00" as *const u8 as *const libc::c_char
        }
      }
      63 => {
        /* exitcode */
        val = utoa((*ptr_to_globals).last_exitcode as libc::c_uint)
      }
      35 => {
        /* argc */
        val = utoa(if (*ptr_to_globals).global_argc != 0 {
          ((*ptr_to_globals).global_argc) - 1i32
        } else {
          0i32
        } as libc::c_uint)
      }
      45 => {
        /* active options */
        /* Check set_mode() to see what option chars we support */
        let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
        cp = (*ptr_to_globals).optstring_buf.as_mut_ptr();
        val = cp;
        if (*ptr_to_globals).o_opt[OPT_O_ERREXIT as libc::c_int as usize] != 0 {
          let fresh23 = cp;
          cp = cp.offset(1);
          *fresh23 = 'e' as i32 as libc::c_char
        }
        if (*ptr_to_globals).interactive_fd != 0 {
          let fresh24 = cp;
          cp = cp.offset(1);
          *fresh24 = 'i' as i32 as libc::c_char
        }
        if (*ptr_to_globals).o_opt[OPT_O_XTRACE as libc::c_int as usize] != 0 {
          let fresh25 = cp;
          cp = cp.offset(1);
          *fresh25 = 'x' as i32 as libc::c_char
        }
        /* If G.o_opt[OPT_O_NOEXEC] is true,
         * commands read but are not executed,
         * so $- can not execute too, 'n' is never seen in $-.
         */
        if (*ptr_to_globals).opt_c != 0 {
          let fresh26 = cp;
          cp = cp.offset(1);
          *fresh26 = 'c' as i32 as libc::c_char
        }
        if (*ptr_to_globals).opt_s != 0 {
          let fresh27 = cp;
          cp = cp.offset(1);
          *fresh27 = 's' as i32 as libc::c_char
        }
        *cp = '\u{0}' as i32 as libc::c_char
      }
      _ => val = get_local_var_value(var),
    }
  }
  /* Handle any expansions */
  if exp_op as libc::c_int == 'L' as i32 {
    reinit_unicode_for_hush(); /* if (exp_op) */
    val = utoa(if !val.is_null() {
      unicode_strlen(val)
    } else {
      0i32 as libc::c_ulong
    } as libc::c_uint)
  } else if exp_op != 0 {
    if exp_op as libc::c_int == '%' as i32 || exp_op as libc::c_int == '#' as i32 {
      /* one of "-=+?" */
      /* Standard-mandated substring removal ops:
       * ${parameter%word} - remove smallest suffix pattern
       * ${parameter%%word} - remove largest suffix pattern
       * ${parameter#word} - remove smallest prefix pattern
       * ${parameter##word} - remove largest prefix pattern
       *
       * Word is expanded to produce a glob pattern.
       * Then var's value is matched to it and matching part removed.
       */
      //FIXME: ${x#...${...}...}
      //should evaluate inner ${...} even if x is "" and no shrinking of it is possible -
      //inner ${...} may have side effects!
      if !val.is_null() && *val.offset(0) as libc::c_int != 0 {
        let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut exp_exp_word: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut loc: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut scan_flags: libc::c_uint = pick_scan(exp_op, *exp_word);
        if exp_op as libc::c_int == *exp_word as libc::c_int {
          /* ## or %% */
          exp_word = exp_word.offset(1)
        }
        exp_exp_word = encode_then_expand_vararg(exp_word, 1i32, 0i32);
        if !exp_exp_word.is_null() {
          exp_word = exp_exp_word
        }
        /*
         * HACK ALERT. We depend here on the fact that
         * G.global_argv and results of utoa and get_local_var_value
         * are actually in writable memory:
         * scan_and_match momentarily stores NULs there.
         */
        t = val as *mut libc::c_char;
        loc = scan_and_match(t, exp_word, scan_flags);
        free(exp_exp_word as *mut libc::c_void);
        if !loc.is_null() {
          /* match was found */
          if scan_flags & SCAN_MATCH_LEFT_HALF as libc::c_int as libc::c_uint != 0 {
            /* #[#] */
            val = loc
          } else {
            /* take right part */
            /* %[%] */
            to_be_freed = xstrndup(
              val,
              loc.wrapping_offset_from(val) as libc::c_long as libc::c_int,
            );
            val = to_be_freed
          }
          /* left */
        }
      }
    } else if exp_op as libc::c_int == '/' as i32 || exp_op as libc::c_int == '\\' as i32 {
      /* It's ${var/[/]pattern[/repl]} thing.
       * Note that in encoded form it has TWO parts:
       * var/pattern<SPECIAL_VAR_SYMBOL>repl<SPECIAL_VAR_SYMBOL>
       * and if // is used, it is encoded as \:
       * var\pattern<SPECIAL_VAR_SYMBOL>repl<SPECIAL_VAR_SYMBOL>
       */
      if !val.is_null() && *val.offset(0) as libc::c_int != 0 {
        /* pattern uses non-standard expansion.
         * repl should be unbackslashed and globbed
         * by the usual expansion rules:
         *  >az >bz
         *  v='a bz'; echo "${v/a*z/a*z}" #prints "a*z"
         *  v='a bz'; echo "${v/a*z/\z}"  #prints "z"
         *  v='a bz'; echo ${v/a*z/a*z}   #prints "az"
         *  v='a bz'; echo ${v/a*z/\z}    #prints "z"
         * (note that a*z _pattern_ is never globbed!)
         */
        let mut pattern: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut repl: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut t_0: *mut libc::c_char = 0 as *mut libc::c_char;
        pattern = encode_then_expand_vararg(exp_word, 1i32, 0i32);
        if pattern.is_null() {
          pattern = xstrdup(exp_word)
        }
        let fresh28 = p;
        p = p.offset(1);
        *fresh28 = 3i32 as libc::c_char;
        exp_word = p;
        p = strchr(p, 3i32);
        *p = '\u{0}' as i32 as libc::c_char;
        repl = encode_then_expand_vararg(exp_word, 1i32, 1i32);
        /* HACK ALERT. We depend here on the fact that
         * G.global_argv and results of utoa and get_local_var_value
         * are actually in writable memory:
         * replace_pattern momentarily stores NULs there. */
        t_0 = val as *mut libc::c_char;
        to_be_freed = replace_pattern(
          t_0,
          pattern,
          if !repl.is_null() { repl } else { exp_word },
          exp_op,
        );
        if !to_be_freed.is_null() {
          /* at least one replace happened */
          val = to_be_freed
        }
        free(pattern as *mut libc::c_void);
        free(repl as *mut libc::c_void);
      } else {
        /* Empty variable always gives nothing */
        // "v=''; echo ${v/*/w}" prints "", not "w"
        /* Just skip "replace" part */
        let fresh29 = p;
        p = p.offset(1);
        *fresh29 = 3i32 as libc::c_char;
        p = strchr(p, 3i32);
        *p = '\u{0}' as i32 as libc::c_char
      }
    } else if exp_op as libc::c_int == ':' as i32 {
      /* BASH_PATTERN_SUBST */
      /* It's ${var:N[:M]} bashism.
       * Note that in encoded form it has TWO parts:
       * var:N<SPECIAL_VAR_SYMBOL>M<SPECIAL_VAR_SYMBOL>
       */
      let mut beg: arith_t = 0;
      let mut len: arith_t = 0;
      let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
      beg = expand_and_evaluate_arith(exp_word, &mut errmsg);
      if !errmsg.is_null() {
        current_block = 17565683642146649389;
      } else {
        let fresh30 = p;
        p = p.offset(1);
        *fresh30 = 3i32 as libc::c_char;
        exp_word = p;
        p = strchr(p, 3i32);
        *p = '\u{0}' as i32 as libc::c_char;
        len = expand_and_evaluate_arith(exp_word, &mut errmsg);
        if !errmsg.is_null() {
          current_block = 17565683642146649389;
        } else {
          if beg < 0i32 as libc::c_longlong {
            /* negative beg counts from the end */
            beg = strlen(val) as arith_t + beg;
            if beg < 0i32 as libc::c_longlong {
              /* ${v: -999999} is "" */
              len = 0i32 as arith_t;
              beg = len
            }
          }
          if len < 0i32 as libc::c_longlong {
            /* in bash, len=-n means strlen()-n */
            len = strlen(val) as arith_t - beg + len;
            if len < 0i32 as libc::c_longlong {
              /* bash compat */
              msg_and_die_if_script(
                b"%s: substring expression < 0\x00" as *const u8 as *const libc::c_char,
                var,
              );
            }
          }
          if len <= 0i32 as libc::c_longlong
            || val.is_null()
            || beg as libc::c_ulonglong >= strlen(val) as libc::c_ulonglong
          {
            current_block = 17565683642146649389;
          } else {
            /* Paranoia. What if user entered 9999999999999
             * which fits in arith_t but not int? */
            if len >= 2147483647i32 as libc::c_longlong {
              len = 2147483647i32 as arith_t
            }
            to_be_freed = xstrndup(val.offset(beg as isize), len as libc::c_int);
            val = to_be_freed;
            current_block = 18009804086567542307;
          }
        }
      }
      match current_block {
        18009804086567542307 => {}
        _ => val = 0 as *const libc::c_char,
      }
    /* not (HUSH_SUBSTR_EXPANSION && FEATURE_SH_MATH) */
    } else {
      /* Standard-mandated substitution ops:
       * ${var?word} - indicate error if unset
       *      If var is unset, word (or a message indicating it is unset
       *      if word is null) is written to standard error
       *      and the shell exits with a non-zero exit status.
       *      Otherwise, the value of var is substituted.
       * ${var-word} - use default value
       *      If var is unset, word is substituted.
       * ${var=word} - assign and use default value
       *      If var is unset, word is assigned to var.
       *      In all cases, final value of var is substituted.
       * ${var+word} - use alternative value
       *      If var is unset, null is substituted.
       *      Otherwise, word is substituted.
       *
       * Word is subjected to tilde expansion, parameter expansion,
       * command substitution, and arithmetic expansion.
       * If word is not needed, it is not expanded.
       *
       * Colon forms (${var:-word}, ${var:=word} etc) do the same,
       * but also treat null var as if it is unset.
       *
       * Word-splitting and single quote behavior:
       *
       * $ f() { for i; do echo "|$i|"; done; };
       *
       * $ x=; f ${x:?'x y' z}
       * bash: x: x y z              #BUG: does not abort, ${} results in empty expansion
       * $ x=; f "${x:?'x y' z}"
       * bash: x: x y z       # dash prints: dash: x: 'x y' z   #BUG: does not abort, ${} results in ""
       *
       * $ x=; f ${x:='x y' z}
       * |x|
       * |y|
       * |z|
       * $ x=; f "${x:='x y' z}"
       * |'x y' z|
       *
       * $ x=x; f ${x:+'x y' z}
       * |x y|
       * |z|
       * $ x=x; f "${x:+'x y' z}"
       * |'x y' z|
       *
       * $ x=; f ${x:-'x y' z}
       * |x y|
       * |z|
       * $ x=; f "${x:-'x y' z}"
       * |'x y' z|
       */
      let mut use_word: libc::c_int = (val.is_null()
        || exp_save as libc::c_int == ':' as i32 && *val.offset(0) == 0)
        as libc::c_int;
      if exp_op as libc::c_int == '+' as i32 {
        use_word = (use_word == 0) as libc::c_int
      }
      if use_word != 0 {
        if exp_op as libc::c_int == '+' as i32 || exp_op as libc::c_int == '-' as i32 {
          /* ${var+word} - use alternative value */
          /* ${var-word} - use default value */
          n = encode_then_append_var_plusminus(output, n, exp_word, arg0 as libc::c_int & 0x80i32);
          val = 0 as *const libc::c_char
        } else {
          /* ${var?word} - indicate error if unset */
          /* ${var=word} - assign and use default value */
          to_be_freed = encode_then_expand_vararg(
            exp_word,
            (arg0 as libc::c_int & 0x80i32 == 0) as libc::c_int,
            0i32,
          );
          if !to_be_freed.is_null() {
            exp_word = to_be_freed
          }
          if exp_op as libc::c_int == '?' as i32 {
            /* mimic bash message */
            msg_and_die_if_script(
              b"%s: %s\x00" as *const u8 as *const libc::c_char,
              var,
              if *exp_word.offset(0) as libc::c_int != 0 {
                exp_word
              } else {
                b"parameter null or not set\x00" as *const u8 as *const libc::c_char
              },
            );
          //TODO: how interactive bash aborts expansion mid-command?
          //It aborts the entire line, returns to prompt:
          // $ f() { for i; do echo "|$i|"; done; }; x=; f "${x:?'x y' z}"; echo YO
          // bash: x: x y z
          // $
          // ("echo YO" is not executed, neither the f function call)
          } else {
            val = exp_word
          }
          if exp_op as libc::c_int == '=' as i32 {
            /* ${var=[word]} or ${var:=[word]} */
            if (*var.offset(0) as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32
              || *var.offset(0) as libc::c_int == '#' as i32
            {
              /* mimic bash message */
              msg_and_die_if_script(
                b"$%s: cannot assign in this way\x00" as *const u8 as *const libc::c_char,
                var,
              );
              val = 0 as *const libc::c_char
            } else {
              let mut new_var: *mut libc::c_char =
                xasprintf(b"%s=%s\x00" as *const u8 as *const libc::c_char, var, val);
              set_local_var(new_var, 0i32 as libc::c_uint);
            }
          }
        }
      }
    }
    *exp_saveptr = exp_save
  }
  *arg.offset(0) = arg0;
  *pp = p;
  n = append_str_maybe_ifs_split(output, n, first_ch, val);
  free(to_be_freed as *mut libc::c_void);
  return n;
}
/* Expand all variable references in given string, adding words to list[]
 * at n, n+1,... positions. Return updated n (so that list[n] is next one
 * to be filled). This routine is extremely tricky: has to deal with
 * variables/parameters with whitespace, $* and $@, and constructs like
 * 'echo -$*-'. If you play here, you must run testsuite afterwards! */
#[inline(never)]
unsafe extern "C" fn expand_vars_to_list(
  mut output: *mut o_string,
  mut n: libc::c_int,
  mut arg: *mut libc::c_char,
) -> libc::c_int {
  /* output->o_expflags & EXP_FLAG_SINGLEWORD (0x80) if we are in
   * expansion of right-hand side of assignment == 1-element expand.
   */
  let mut cant_be_null: libc::c_char = 0i32 as libc::c_char; /* only bit 0x80 matters */
  let mut p: *mut libc::c_char = 0 as *mut libc::c_char; /* end of "while (SPECIAL_VAR_SYMBOL is found) ..." */
  loop {
    p = strchr(arg, 3i32);
    if p.is_null() {
      break;
    }
    let mut first_ch: libc::c_char = 0;
    let mut arith_buf: [libc::c_char; 26] = [0; 26];
    if (*output).ended_in_ifs != 0 {
      o_addchr(output, '\u{0}' as i32);
      n = o_save_ptr(output, n);
      (*output).ended_in_ifs = 0i32 as smallint
    }
    o_addblock(
      output,
      arg,
      p.wrapping_offset_from(arg) as libc::c_long as libc::c_int,
    );
    p = p.offset(1);
    arg = p;
    p = strchr(p, 3i32);
    /* Fetch special var name (if it is indeed one of them)
     * and quote bit, force the bit on if singleword expansion -
     * important for not getting v=$@ expand to many words. */
    first_ch = (*arg.offset(0) as libc::c_int
      | (*output).o_expflags & EXP_FLAG_SINGLEWORD as libc::c_int) as libc::c_char;
    /* Is this variable quoted and thus expansion can't be null?
     * "$@" is special. Even if quoted, it can still
     * expand to nothing (not even an empty string),
     * thus it is excluded. */
    if first_ch as libc::c_int & 0x7fi32 != '@' as i32 {
      cant_be_null = (cant_be_null as libc::c_int | first_ch as libc::c_int) as libc::c_char
    } /* switch (char after <SPECIAL_VAR_SYMBOL>) */
    match first_ch as libc::c_int & 0x7fi32 {
      42 | 64 => {
        /* Highest bit in first_ch indicates that var is double-quoted */
        let mut i: libc::c_int = 0; /* do it for "$@" _now_, when we know it's not empty */
        if !(*(*ptr_to_globals).global_argv.offset(1)).is_null() {
          i = 1i32;
          cant_be_null = (cant_be_null as libc::c_int | first_ch as libc::c_int) as libc::c_char;
          if first_ch as libc::c_int & 0x80i32 == 0 {
            /* unquoted $* or $@ */
            while !(*(*ptr_to_globals).global_argv.offset(i as isize)).is_null() {
              n = expand_on_ifs(output, n, *(*ptr_to_globals).global_argv.offset(i as isize));
              let fresh31 = i;
              i = i + 1;
              if *(*(*ptr_to_globals).global_argv.offset(fresh31 as isize)).offset(0) as libc::c_int
                != 0
                && !(*(*ptr_to_globals).global_argv.offset(i as isize)).is_null()
              {
                /* this argv[] is not empty and not last:
                 * put terminating NUL, start new word */
                o_addchr(output, '\u{0}' as i32);
                n = o_save_ptr(output, n)
              }
            }
          } else if first_ch as libc::c_int == ('@' as i32 | 0x80i32) as libc::c_char as libc::c_int
            && (*output).o_expflags & EXP_FLAG_SINGLEWORD as libc::c_int == 0
          {
            /* If EXP_FLAG_SINGLEWORD, we handle assignment 'a=....$@.....'
             * and in this case should treat it like '$*' - see 'else...' below */
            /* not v="$@" case */
            loop {
              o_addQstr(output, *(*ptr_to_globals).global_argv.offset(i as isize)); /* quoted $* (or v="$@" case): add as one word */
              i += 1;
              if i >= (*ptr_to_globals).global_argc {
                break;
              }
              o_addchr(output, '\u{0}' as i32);
              n = o_save_ptr(output, n)
            }
          } else {
            loop {
              o_addQstr(output, *(*ptr_to_globals).global_argv.offset(i as isize));
              i += 1;
              if (*(*ptr_to_globals).global_argv.offset(i as isize)).is_null() {
                break;
              }
              if *(*ptr_to_globals).ifs.offset(0) != 0 {
                o_addchr(output, *(*ptr_to_globals).ifs.offset(0) as libc::c_int);
              }
            }
            (*output).has_quoted_part = 1i32 as smallint
          }
        }
      }
      3 => {
        /* <SPECIAL_VAR_SYMBOL><SPECIAL_VAR_SYMBOL> */
        /* "Empty variable", used to make "" etc to not disappear */
        (*output).has_quoted_part = 1i32 as smallint;
        cant_be_null = 0x80i32 as libc::c_char;
        arg = arg.offset(1)
      }
      1 => {
        /* <SPECIAL_VAR_SYMBOL><SPECIAL_VAR_QUOTED_SVS><SPECIAL_VAR_SYMBOL> */
        /* "^C variable", represents literal ^C char (possible in scripts) */
        o_addchr(output, 3i32);
        arg = arg.offset(1)
      }
      96 => {
        /* <SPECIAL_VAR_SYMBOL>`cmd<SPECIAL_VAR_SYMBOL> */
        let mut subst_result: o_string = {
          let mut init = o_string {
            data: 0 as *mut libc::c_char,
            length: 0,
            maxlen: 0,
            o_expflags: 0,
            has_quoted_part: 0,
            has_empty_slot: 0,
            ended_in_ifs: 0,
          }; /* replace trailing <SPECIAL_VAR_SYMBOL> */
          init
        };
        *p = '\u{0}' as i32 as libc::c_char;
        arg = arg.offset(1);
        /* Can't just stuff it into output o_string,
         * expanded result may need to be globbed
         * and $IFS-split */
        (*ptr_to_globals).last_exitcode = process_command_subs(&mut subst_result, arg) as smalluint;
        (*ptr_to_globals).expand_exitcode = (*ptr_to_globals).last_exitcode;
        n = append_str_maybe_ifs_split(output, n, first_ch as libc::c_int, subst_result.data);
        o_free(&mut subst_result);
      }
      43 => {
        /* <SPECIAL_VAR_SYMBOL>+arith<SPECIAL_VAR_SYMBOL> */
        let mut res: arith_t = 0; /* skip '+' */
        arg = arg.offset(1); /* replace trailing <SPECIAL_VAR_SYMBOL> */
        *p = '\u{0}' as i32 as libc::c_char;
        res = expand_and_evaluate_arith(arg, 0 as *mut *const libc::c_char);
        sprintf(
          arith_buf.as_mut_ptr(),
          b"%lld\x00" as *const u8 as *const libc::c_char,
          res,
        );
        o_addstr(output, arith_buf.as_mut_ptr());
      }
      _ => {
        /* <SPECIAL_VAR_SYMBOL>varname[ops]<SPECIAL_VAR_SYMBOL> */
        n = expand_one_var(output, n, first_ch as libc::c_int, arg, &mut p)
      }
    }
    /* Restore NULL'ed SPECIAL_VAR_SYMBOL.
     * Do the check to avoid writing to a const string. */
    if *p as libc::c_int != 3i32 {
      *p = 3i32 as libc::c_char
    }
    p = p.offset(1);
    arg = p
  }
  if *arg != 0 {
    /* handle trailing string */
    if (*output).ended_in_ifs != 0 {
      o_addchr(output, '\u{0}' as i32);
      n = o_save_ptr(output, n)
    }
    /* this part is literal, and it was already pre-quoted
     * if needed (much earlier), do not use o_addQstr here!
     */
    o_addstr(output, arg);
  } else if (*output).length == o_get_last_ptr(output, n)
    && cant_be_null as libc::c_int & 0x80i32 == 0
    && (*output).has_quoted_part == 0
  {
    n -= 1;
    /* allow to reuse list[n] later without re-growth */
    (*output).has_empty_slot = 1i32 as smallint
  }
  return n;
}
unsafe extern "C" fn expand_variables(
  mut argv: *mut *mut libc::c_char,
  mut expflags: libc::c_uint,
) -> *mut *mut libc::c_char {
  let mut n: libc::c_int = 0;
  let mut list: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  let mut output: o_string = {
    let mut init = o_string {
      data: 0 as *mut libc::c_char,
      length: 0,
      maxlen: 0,
      o_expflags: 0,
      has_quoted_part: 0,
      has_empty_slot: 0,
      ended_in_ifs: 0,
    };
    init
  };
  output.o_expflags = expflags as libc::c_int;
  n = 0i32;
  loop {
    /* go to next list[n] */
    output.ended_in_ifs = 0i32 as smallint;
    n = o_save_ptr(&mut output, n);
    if (*argv).is_null() {
      break;
    }
    /* expand argv[i] */
    let fresh32 = argv;
    argv = argv.offset(1);
    n = expand_vars_to_list(&mut output, n, *fresh32);
    /* if (!output->has_empty_slot) -- need this?? */
    o_addchr(&mut output, '\u{0}' as i32);
  }
  /* output.data (malloced in one block) gets returned in "list" */
  list = o_finalize_list(&mut output, n);
  return list;
}
unsafe extern "C" fn expand_strvec_to_strvec(
  mut argv: *mut *mut libc::c_char,
) -> *mut *mut libc::c_char {
  return expand_variables(
    argv,
    (EXP_FLAG_GLOB as libc::c_int | EXP_FLAG_ESC_GLOB_CHARS as libc::c_int) as libc::c_uint,
  );
}
unsafe extern "C" fn expand_strvec_to_strvec_singleword_noglob(
  mut argv: *mut *mut libc::c_char,
) -> *mut *mut libc::c_char {
  return expand_variables(argv, EXP_FLAG_SINGLEWORD as libc::c_int as libc::c_uint);
}
/* ** Execution routines ***/
/* Expansion can recurse, need forward decls: */
/* Used for expansion of right hand of assignments,
 * $((...)), heredocs, variable expansion parts.
 *
 * NB: should NOT do globbing!
 * "export v=/bin/c*; env | grep ^v=" outputs "v=/bin/c*"
 */
unsafe extern "C" fn expand_string_to_string(
  mut str: *const libc::c_char,
  mut EXP_flags: libc::c_int,
  mut do_unbackslash: libc::c_int,
) -> *mut libc::c_char {
  let mut argv: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2];
  let mut list: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  /* This is generally an optimization, but it also
   * handles "", which otherwise trips over !list[0] check below.
   * (is this ever happens that we actually get str="" here?)
   */
  if strchr(str, 3i32).is_null() && strchr(str, '\\' as i32).is_null() {
    //TODO: Can use on strings with \ too, just unbackslash() them?
    return xstrdup(str);
  }
  argv[0] = str as *mut libc::c_char;
  argv[1] = 0 as *mut libc::c_char;
  list = expand_variables(
    argv.as_mut_ptr(),
    (EXP_flags | EXP_FLAG_SINGLEWORD as libc::c_int) as libc::c_uint,
  );
  if (*list.offset(0)).is_null() {
    /* Example where it happens:
     * x=; echo ${x:-"$@"}
     */
    *(list as *mut libc::c_char).offset(0) = '\u{0}' as i32 as libc::c_char
  } else {
    if !(*list.offset(1)).is_null() {
      bb_simple_error_msg_and_die(b"BUG in varexp2\x00" as *const u8 as *const libc::c_char);
    }
    /* actually, just move string 2*sizeof(char*) bytes back */
    overlapping_strcpy(list as *mut libc::c_char, *list.offset(0));
    if do_unbackslash != 0 {
      unbackslash(list as *mut libc::c_char);
    }
  }
  return list as *mut libc::c_char;
}
unsafe extern "C" fn expand_assignments(
  mut argv: *mut *mut libc::c_char,
  mut count: libc::c_int,
) -> *mut *mut libc::c_char {
  let mut i: libc::c_int = 0;
  let mut p: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  p = 0 as *mut *mut libc::c_char;
  (*ptr_to_globals).expanded_assignments = p;
  /* Expand assignments into one string each */
  i = 0i32;
  while i < count {
    p = add_string_to_strings(
      p,
      expand_string_to_string(
        *argv.offset(i as isize),
        EXP_FLAG_ESC_GLOB_CHARS as libc::c_int,
        1i32,
      ),
    );
    (*ptr_to_globals).expanded_assignments = p;
    i += 1
  }
  (*ptr_to_globals).expanded_assignments = 0 as *mut *mut libc::c_char;
  return p;
}
unsafe extern "C" fn switch_off_special_sigs(mut mask: libc::c_uint) {
  let mut sig: libc::c_uint = 0i32 as libc::c_uint;
  loop
  /* trap is '', has to remain SIG_IGN */
  {
    mask >>= 1i32;
    if !(mask != 0i32 as libc::c_uint) {
      break;
    }
    sig = sig.wrapping_add(1);
    if mask & 1i32 as libc::c_uint == 0 {
      continue;
    }
    if !(*ptr_to_globals).traps.is_null() {
      if !(*(*ptr_to_globals).traps.offset(sig as isize)).is_null()
        && *(*(*ptr_to_globals).traps.offset(sig as isize)).offset(0) == 0
      {
        continue;
      }
      free(*(*ptr_to_globals).traps.offset(sig as isize) as *mut libc::c_void);
      let ref mut fresh33 = *(*ptr_to_globals).traps.offset(sig as isize);
      *fresh33 = 0 as *mut libc::c_char
    }
    /* We are here only if no trap or trap was not '' */
    install_sighandler(sig as libc::c_int, None);
  }
}
unsafe extern "C" fn reset_traps_to_defaults() {
  /* This function is always called in a child shell
   * after fork (not vfork, NOMMU doesn't use this function).
   */
  let mut sig: libc::c_uint = 0;
  let mut mask: libc::c_uint = 0;
  /* Child shells are not interactive.
   * SIGTTIN/SIGTTOU/SIGTSTP should not have special handling.
   * Testcase: (while :; do :; done) + ^Z should background.
   * Same goes for SIGTERM, SIGHUP, SIGINT.
   */
  mask = (*ptr_to_globals).special_sig_mask
    & SPECIAL_INTERACTIVE_SIGS as libc::c_int as libc::c_uint
    | (*ptr_to_globals).fatal_sig_mask; /* already no traps and no special sigs */
  if (*ptr_to_globals).traps.is_null() && mask == 0 {
    return;
  }
  /* Switch off special sigs */
  switch_off_special_sigs(mask);
  (*ptr_to_globals).fatal_sig_mask = 0i32 as libc::c_uint;
  (*ptr_to_globals).special_sig_mask &= !(SPECIAL_INTERACTIVE_SIGS as libc::c_int) as libc::c_uint;
  /* SIGQUIT,SIGCHLD and maybe SPECIAL_JOBSTOP_SIGS
   * remain set in G.special_sig_mask */
  if (*ptr_to_globals).traps.is_null() {
    return;
  }
  /* Reset all sigs to default except ones with empty traps */
  sig = 0i32 as libc::c_uint; /* no trap: nothing to do */
  while sig < (64i32 + 1i32) as libc::c_uint {
    if !(*(*ptr_to_globals).traps.offset(sig as isize)).is_null() {
      if !(*(*(*ptr_to_globals).traps.offset(sig as isize)).offset(0) == 0) {
        /* empty trap: has to remain SIG_IGN */
        /* sig has non-empty trap, reset it: */
        free(*(*ptr_to_globals).traps.offset(sig as isize) as *mut libc::c_void);
        let ref mut fresh34 = *(*ptr_to_globals).traps.offset(sig as isize);
        *fresh34 = 0 as *mut libc::c_char;
        /* There is no signal for trap 0 (EXIT) */
        if !(sig == 0i32 as libc::c_uint) {
          install_sighandler(sig as libc::c_int, pick_sighandler(sig));
        }
      }
    }
    sig = sig.wrapping_add(1)
  }
}
/* Executing from string: eval, sh -c '...'
 *          or from file: /etc/profile, . file, sh <script>, sh (intereactive)
 * end_trigger controls how often we stop parsing
 * NUL: parse all, execute, return
 * ';': parse till ';' or newline, execute, repeat till EOF
 */
unsafe extern "C" fn parse_and_run_stream(mut inp: *mut in_str, mut end_trigger: libc::c_int) {
  /* Why we need empty flag?
   * An obscure corner case "false; ``; echo $?":
   * empty command in `` should still set $? to 0.
   * But we can't just set $? to 0 at the start,
   * this breaks "false; echo `echo $?`" case.
   */
  let mut empty: bool = 1i32 != 0; /* PS1 */
  loop {
    let mut pipe_list: *mut pipe = 0 as *mut pipe;
    if end_trigger == ';' as i32 {
      (*ptr_to_globals).promptmode = 0i32 as smallint
    }
    pipe_list = parse_stream(0 as *mut libc::c_int, inp, end_trigger);
    if pipe_list.is_null() || pipe_list == 1i32 as libc::c_long as *mut libc::c_void as *mut pipe {
      /* EOF/error */
      /* If we are in "big" script
       * (not in `cmd` or something similar)...
       */
      if pipe_list == 1i32 as libc::c_long as *mut libc::c_void as *mut pipe
        && end_trigger == ';' as i32
      {
        /* Discard cached input (rest of line) */
        let mut ch: libc::c_int = (*inp).last_char;
        while ch != -1i32 && ch != '\n' as i32 {
          //bb_error_msg("Discarded:'%c'", ch);
          ch = i_getch(inp)
        }
        /* Force prompt */
        (*inp).p = 0 as *const libc::c_char;
        /* This stream isn't empty */
        empty = 0i32 != 0
      } else {
        if pipe_list.is_null() && empty as libc::c_int != 0 {
          (*ptr_to_globals).last_exitcode = 0i32 as smalluint
        }
        break;
      }
    } else {
      run_and_free_list(pipe_list);
      empty = 0i32 != 0;
      if (*ptr_to_globals).flag_return_in_progress as libc::c_int == 1i32 {
        break;
      }
    }
  }
}
unsafe extern "C" fn parse_and_run_string(mut s: *const libc::c_char) {
  let mut input: in_str = in_str {
    p: 0 as *const libc::c_char,
    peek_buf: [0; 2],
    last_char: 0,
    file: 0 as *mut HFILE,
  };
  //IF_HUSH_LINENO_VAR(unsigned sv = G.parse_lineno;)
  setup_string_in_str(&mut input, s);
  parse_and_run_stream(&mut input, '\u{0}' as i32);
  //IF_HUSH_LINENO_VAR(G.parse_lineno = sv;)
}
unsafe extern "C" fn parse_and_run_file(mut fp: *mut HFILE) {
  let mut input: in_str = in_str {
    p: 0 as *const libc::c_char,
    peek_buf: [0; 2],
    last_char: 0,
    file: 0 as *mut HFILE,
  };
  let mut sv: libc::c_uint = (*ptr_to_globals).parse_lineno;
  (*ptr_to_globals).parse_lineno = 1i32 as libc::c_uint;
  setup_file_in_str(&mut input, fp);
  parse_and_run_stream(&mut input, ';' as i32);
  (*ptr_to_globals).parse_lineno = sv;
}
unsafe extern "C" fn generate_stream_from_string(
  mut s: *const libc::c_char,
  mut pid_p: *mut pid_t,
) -> libc::c_int {
  let mut pid: pid_t = 0;
  let mut channel: [libc::c_int; 2] = [0; 2];
  xpipe(channel.as_mut_ptr());
  pid = if 1i32 != 0 {
    xfork()
  } else {
    ({
      let mut bb__xvfork_pid: pid_t = vfork();
      if bb__xvfork_pid < 0i32 {
        bb_simple_perror_msg_and_die(b"vfork\x00" as *const u8 as *const libc::c_char);
      }
      bb__xvfork_pid
    })
  };
  if pid == 0i32 {
    /* child */
    die_func = ::std::mem::transmute::<
      Option<unsafe extern "C" fn() -> !>,
      Option<unsafe extern "C" fn() -> ()>,
    >(Some(fflush_and__exit as unsafe extern "C" fn() -> !));
    /* Process substitution is not considered to be usual
     * 'command execution'.
     * SUSv3 says ctrl-Z should be ignored, ctrl-C should not.
     */
    bb_signals(
      0i32 + (1i32 << 20i32) + (1i32 << 21i32) + (1i32 << 22i32),
      ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(1i32 as libc::intptr_t),
    ); /* NB: close _first_, then move fd! */
    close(channel[0]);
    xmove_fd(channel[1], 1i32);
    /* Awful hack for `trap` or $(trap).
     *
     * http://www.opengroup.org/onlinepubs/009695399/utilities/trap.html
     * contains an example where "trap" is executed in a subshell:
     *
     * save_traps=$(trap)
     * ...
     * eval "$save_traps"
     *
     * Standard does not say that "trap" in subshell shall print
     * parent shell's traps. It only says that its output
     * must have suitable form, but then, in the above example
     * (which is not supposed to be normative), it implies that.
     *
     * bash (and probably other shell) does implement it
     * (traps are reset to defaults, but "trap" still shows them),
     * but as a result, "trap" logic is hopelessly messed up:
     *
     * # trap
     * trap -- 'echo Ho' SIGWINCH  <--- we have a handler
     * # (trap)        <--- trap is in subshell - no output (correct, traps are reset)
     * # true | trap   <--- trap is in subshell - no output (ditto)
     * # echo `true | trap`    <--- in subshell - output (but traps are reset!)
     * trap -- 'echo Ho' SIGWINCH
     * # echo `(trap)`         <--- in subshell in subshell - output
     * trap -- 'echo Ho' SIGWINCH
     * # echo `true | (trap)`  <--- in subshell in subshell in subshell - output!
     * trap -- 'echo Ho' SIGWINCH
     *
     * The rules when to forget and when to not forget traps
     * get really complex and nonsensical.
     *
     * Our solution: ONLY bare $(trap) or `trap` is special.
     */
    s = skip_whitespace(s); /* important */
    if !is_prefixed_with(s, b"trap\x00" as *const u8 as *const libc::c_char).is_null()
      && *skip_whitespace(s.offset(4)).offset(0) as libc::c_int == '\u{0}' as i32
    {
      static mut argv: [*const libc::c_char; 2] =
        [0 as *const libc::c_char, 0 as *const libc::c_char];
      builtin_trap(argv.as_ptr() as *mut *mut libc::c_char);
      fflush_all();
      _exit(0i32);
    }
    /* Prevent it from trying to handle ctrl-z etc */
    (*ptr_to_globals).run_list_level = 1i32; /* or else $RANDOM repeats in child */
    (*ptr_to_globals).random_gen.galois_LFSR = 0i32;
    reset_traps_to_defaults();
    (*ptr_to_globals).x_mode_depth = (*ptr_to_globals).x_mode_depth.wrapping_add(1);
    //bb_error_msg("%s: ++x_mode_depth=%d", __func__, G.x_mode_depth);
    parse_and_run_string(s);
    _exit((*ptr_to_globals).last_exitcode as libc::c_int);
  }
  /* parent */
  *pid_p = pid;
  die_func = ::std::mem::transmute::<
    Option<unsafe extern "C" fn() -> !>,
    Option<unsafe extern "C" fn() -> ()>,
  >(Some(
    restore_ttypgrp_and__exit as unsafe extern "C" fn() -> !,
  ));
  close(channel[1]);
  return channel[0];
}
/* Return code is exit status of the process that is run. */
unsafe extern "C" fn process_command_subs(
  mut dest: *mut o_string,
  mut s: *const libc::c_char,
) -> libc::c_int {
  let mut fp: *mut FILE = 0 as *mut FILE;
  let mut pid: pid_t = 0;
  let mut status: libc::c_int = 0;
  let mut ch: libc::c_int = 0;
  let mut eol_cnt: libc::c_int = 0;
  fp = xfdopen_for_read(generate_stream_from_string(s, &mut pid));
  /* Now send results of command back into original context */
  eol_cnt = 0i32;
  loop {
    ch = getc_unlocked(fp);
    if !(ch != -1i32) {
      break;
    }
    if ch == '\u{0}' as i32 {
      continue;
    }
    if ch == '\n' as i32 {
      eol_cnt += 1
    } else {
      while eol_cnt != 0 {
        o_addchr(dest, '\n' as i32);
        eol_cnt -= 1
      }
      o_addQchr(dest, ch);
    }
  }
  fclose(fp);
  /* We need to extract exitcode. Test case
   * "true; echo `sleep 1; false` $?"
   * should print 1 */
  safe_waitpid(pid, &mut status, 0i32);
  return (status & 0xff00i32) >> 8i32;
}
/* ENABLE_HUSH_TICK */
unsafe extern "C" fn setup_heredoc(mut redir: *mut redir_struct) {
  let mut pair: fd_pair = fd_pair { rd: 0, wr: 0 };
  let mut pid: pid_t = 0;
  let mut len: libc::c_int = 0;
  let mut written: libc::c_int = 0;
  /* the _body_ of heredoc (misleading field name) */
  let mut heredoc: *const libc::c_char = (*redir).rd_filename; /* often saves dup2+close in xmove_fd */
  let mut expanded: *mut libc::c_char = 0 as *mut libc::c_char;
  expanded = 0 as *mut libc::c_char;
  if (*redir).rd_dup & HEREDOC_QUOTED as libc::c_int == 0 {
    expanded = encode_then_expand_string(heredoc);
    if !expanded.is_null() {
      heredoc = expanded
    }
  }
  len = strlen(heredoc) as libc::c_int;
  close((*redir).rd_fd);
  xpipe(&mut pair.rd);
  xmove_fd(pair.rd, (*redir).rd_fd);
  /* Try writing without forking. Newer kernels have
   * dynamically growing pipes. Must use non-blocking write! */
  ndelay_on(pair.wr);
  loop {
    written = write(pair.wr, heredoc as *const libc::c_void, len as size_t) as libc::c_int;
    if written <= 0i32 {
      break;
    }
    len -= written;
    if len == 0i32 {
      close(pair.wr);
      free(expanded as *mut libc::c_void);
      return;
    }
    heredoc = heredoc.offset(written as isize)
  }
  ndelay_off(pair.wr);
  /* Okay, pipe buffer was not big enough */
  /* Note: we must not create a stray child (bastard? :)
   * for the unsuspecting parent process. Child creates a grandchild
   * and exits before parent execs the process which consumes heredoc
   * (that exec happens after we return from this function) */
  pid = {
    let mut bb__xvfork_pid: pid_t = vfork();
    if bb__xvfork_pid < 0i32 {
      bb_simple_perror_msg_and_die(b"vfork\x00" as *const u8 as *const libc::c_char);
    }
    bb__xvfork_pid
  };
  if pid == 0i32 {
    /* child */
    die_func = ::std::mem::transmute::<
      Option<unsafe extern "C" fn() -> !>,
      Option<unsafe extern "C" fn() -> ()>,
    >(Some(fflush_and__exit as unsafe extern "C" fn() -> !));
    pid = if 1i32 != 0 {
      xfork()
    } else {
      ({
        let mut bb__xvfork_pid: pid_t = vfork();
        if bb__xvfork_pid < 0i32 {
          bb_simple_perror_msg_and_die(b"vfork\x00" as *const u8 as *const libc::c_char);
        }
        bb__xvfork_pid
      })
    };
    if pid != 0i32 {
      _exit(0i32);
    }
    /* grandchild */
    close((*redir).rd_fd); /* read side of the pipe */
    full_write(pair.wr, heredoc as *const libc::c_void, len as size_t); /* may loop or block */
    _exit(0i32);
  }
  /* parent */
  die_func = ::std::mem::transmute::<
    Option<unsafe extern "C" fn() -> !>,
    Option<unsafe extern "C" fn() -> ()>,
  >(Some(
    restore_ttypgrp_and__exit as unsafe extern "C" fn() -> !,
  ));
  close(pair.wr);
  free(expanded as *mut libc::c_void);
  wait(0 as *mut libc::c_int);
  /* wait till child has died */
}
unsafe extern "C" fn append_squirrel(
  mut sq: *mut squirrel,
  mut i: libc::c_int,
  mut orig: libc::c_int,
  mut moved: libc::c_int,
) -> *mut squirrel {
  sq = xrealloc(
    sq as *mut libc::c_void,
    ((i + 2i32) as libc::c_ulong).wrapping_mul(::std::mem::size_of::<squirrel>() as libc::c_ulong),
  ) as *mut squirrel; /* end marker */
  (*sq.offset(i as isize)).orig_fd = orig;
  (*sq.offset(i as isize)).moved_to = moved;
  (*sq.offset((i + 1i32) as isize)).orig_fd = -1i32;
  return sq;
}
unsafe extern "C" fn add_squirrel(
  mut sq: *mut squirrel,
  mut fd: libc::c_int,
  mut avoid_fd: libc::c_int,
) -> *mut squirrel {
  let mut moved_to: libc::c_int = 0;
  let mut i: libc::c_int = 0;
  i = 0i32;
  if !sq.is_null() {
    while (*sq.offset(i as isize)).orig_fd >= 0i32 {
      /* If we collide with an already moved fd... */
      if fd == (*sq.offset(i as isize)).moved_to {
        (*sq.offset(i as isize)).moved_to =
          dup_CLOEXEC((*sq.offset(i as isize)).moved_to, avoid_fd);
        if (*sq.offset(i as isize)).moved_to < 0i32 {
          /* what? */
          xfunc_die();
        }
        return sq;
      }
      if fd == (*sq.offset(i as isize)).orig_fd {
        /* Example: echo Hello >/dev/null 1>&2 */
        return sq;
      }
      i += 1
    }
  }
  /* If this fd is open, we move and remember it; if it's closed, moved_to = -1 */
  moved_to = dup_CLOEXEC(fd, avoid_fd);
  if moved_to < 0i32 && *bb_errno != 9i32 {
    xfunc_die();
  }
  return append_squirrel(sq, i, fd, moved_to);
}
unsafe extern "C" fn add_squirrel_closed(
  mut sq: *mut squirrel,
  mut fd: libc::c_int,
) -> *mut squirrel {
  let mut i: libc::c_int = 0;
  i = 0i32;
  if !sq.is_null() {
    while (*sq.offset(i as isize)).orig_fd >= 0i32 {
      /* If we collide with an already moved fd... */
      if fd == (*sq.offset(i as isize)).orig_fd {
        /* Examples:
         * "echo 3>FILE 3>&- 3>FILE"
         * "echo 3>&- 3>FILE"
         * No need for last redirect to insert
         * another "need to close 3" indicator.
         */
        return sq;
      }
      i += 1
    }
  }
  return append_squirrel(sq, i, fd, -1i32);
}
/* fd: redirect wants this fd to be used (e.g. 3>file).
 * Move all conflicting internally used fds,
 * and remember them so that we can restore them later.
 */
unsafe extern "C" fn save_fd_on_redirect(
  mut fd: libc::c_int,
  mut avoid_fd: libc::c_int,
  mut sqp: *mut *mut squirrel,
) -> libc::c_int {
  if avoid_fd < 9i32 {
    /* the important case here is that it can be -1 */
    avoid_fd = 9i32
  }
  if fd == (*ptr_to_globals).interactive_fd {
    /* Testcase: "ls -l /proc/$$/fd 255>&-" should work */
    (*ptr_to_globals).interactive_fd =
      xdup_CLOEXEC_and_close((*ptr_to_globals).interactive_fd, avoid_fd);
    return 1i32;
    /* "we closed fd" */
  }
  /* Are we called from setup_redirects(squirrel==NULL)
   * in redirect in a [v]forked child?
   */
  if sqp.is_null() {
    /* No need to move script fds.
     * For NOMMU case, it's actively wrong: we'd change ->fd
     * fields in memory for the parent, but parent's fds
     * aren't be moved, it would use wrong fd!
     * Reproducer: "cmd 3>FILE" in script.
     * If we would call move_HFILEs_on_redirect(), child would:
     *  fcntl64(3, F_DUPFD_CLOEXEC, 10)   = 10
     *  close(3)                          = 0
     * and change ->fd to 10 if fd#3 is a script fd. WRONG.
     */
    //bb_error_msg("sqp == NULL: [v]forked child");
    return 0i32;
  }
  /* If this one of script's fds? */
  if move_HFILEs_on_redirect(fd, avoid_fd) != 0 {
    return 1i32;
  } /* yes. "we closed fd" (actually moved it) */
  /* Are we called for "exec 3>FILE"? Came through
   * redirect_and_varexp_helper(squirrel=ERR_PTR) -> setup_redirects(ERR_PTR)
   * This case used to fail for this script:
   *  exec 3>FILE
   *  echo Ok
   *  ...100000 more lines...
   *  echo Ok
   * as follows:
   *  read(3, "exec 3>FILE\necho Ok\necho Ok"..., 1024) = 1024
   *  open("FILE", O_WRONLY|O_CREAT|O_TRUNC|O_LARGEFILE, 0666) = 4
   *  dup2(4, 3)                        = 3
   *  ^^^^^^^^ oops, we lost fd#3 opened to our script!
   *  close(4)                          = 0
   *  write(1, "Ok\n", 3)               = 3
   *  ...                               = 3
   *  write(1, "Ok\n", 3)               = 3
   *  read(3, 0x94fbc08, 1024)          = -1 EBADF (Bad file descriptor)
   *  ^^^^^^^^ oops, wrong fd!!!
   * With this case separate from sqp == NULL and *after* move_HFILEs,
   * it now works:
   */
  if sqp == 1i32 as libc::c_long as *mut libc::c_void as *mut *mut squirrel {
    /* Don't preserve redirected fds: exec is _meant_ to change these */
    //bb_error_msg("sqp == ERR_PTR: exec >FILE");
    return 0i32;
  }
  /* Check whether it collides with any open fds (e.g. stdio), save fds as needed */
  *sqp = add_squirrel(*sqp, fd, avoid_fd);
  return 0i32;
  /* "we did not close fd" */
}
unsafe extern "C" fn restore_redirects(mut sq: *mut squirrel) {
  if !sq.is_null() {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while (*sq.offset(i as isize)).orig_fd >= 0i32 {
      if (*sq.offset(i as isize)).moved_to >= 0i32 {
        /* We simply die on error */
        xmove_fd(
          (*sq.offset(i as isize)).moved_to,
          (*sq.offset(i as isize)).orig_fd,
        );
      } else {
        /* cmd1 9>FILE; cmd2_should_see_fd9_closed */
        close((*sq.offset(i as isize)).orig_fd);
      }
      i += 1
    }
    free(sq as *mut libc::c_void);
  };
  /* If moved, G_interactive_fd stays on new fd, not restoring it */
}
unsafe extern "C" fn internally_opened_fd(
  mut fd: libc::c_int,
  mut sq: *mut squirrel,
) -> libc::c_int {
  let mut i: libc::c_int = 0;
  if fd == (*ptr_to_globals).interactive_fd {
    return 1i32;
  }
  /* If this one of script's fds? */
  if fd_in_HFILEs(fd) != 0 {
    return 1i32;
  }
  if !sq.is_null() {
    i = 0i32;
    while (*sq.offset(i as isize)).orig_fd >= 0i32 {
      if fd == (*sq.offset(i as isize)).moved_to {
        return 1i32;
      }
      i += 1
    }
  }
  return 0i32;
}
/* squirrel != NULL means we squirrel away copies of stdin, stdout,
 * and stderr if they are redirected. */
unsafe extern "C" fn setup_redirects(
  mut prog: *mut command,
  mut sqp: *mut *mut squirrel,
) -> libc::c_int {
  let mut redir: *mut redir_struct = 0 as *mut redir_struct;
  let mut current_block_32: u64;
  redir = (*prog).redirects;
  while !redir.is_null() {
    let mut newfd: libc::c_int = 0;
    let mut closed: libc::c_int = 0;
    if (*redir).rd_type as libc::c_int == REDIRECT_HEREDOC2 as libc::c_int {
      /* "rd_fd<<HERE" case */
      save_fd_on_redirect((*redir).rd_fd, 0i32, sqp);
      /* for REDIRECT_HEREDOC2, rd_filename holds _contents_
       * of the heredoc */
      setup_heredoc(redir);
    } else {
      if (*redir).rd_dup == REDIRFD_TO_FILE as libc::c_int {
        /* "rd_fd<*>file" case (<*> is <,>,>>,<>) */
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut mode: libc::c_int = 0;
        if (*redir).rd_filename.is_null() {
          /* Examples:
           * "cmd >" (no filename)
           * "cmd > <file" (2nd redirect starts too early)
           */
          syntax_error(b"invalid redirect\x00" as *const u8 as *const libc::c_char);
          current_block_32 = 16668937799742929182;
        } else {
          mode = redir_table[(*redir).rd_type as usize].mode;
          p = expand_string_to_string(
            (*redir).rd_filename,
            EXP_FLAG_ESC_GLOB_CHARS as libc::c_int,
            1i32,
          );
          newfd = open_or_warn(p, mode);
          free(p as *mut libc::c_void);
          if newfd < 0i32 {
            /* Error message from open_or_warn can be lost
             * if stderr has been redirected, but bash
             * and ash both lose it as well
             * (though zsh doesn't!)
             */
            return 1i32;
          }
          if newfd == (*redir).rd_fd && !sqp.is_null() {
            /* open() gave us precisely the fd we wanted.
             * This means that this fd was not busy
             * (not opened to anywhere).
             * Remember to close it on restore:
             */
            *sqp = add_squirrel_closed(*sqp, newfd)
          }
          current_block_32 = 15125582407903384992;
        }
      } else {
        /* "rd_fd>&rd_dup" or "rd_fd>&-" case */
        newfd = (*redir).rd_dup;
        current_block_32 = 15125582407903384992;
      }
      match current_block_32 {
        16668937799742929182 => {}
        _ => {
          if !(newfd == (*redir).rd_fd) {
            /* if "N>FILE": move newfd to redir->rd_fd */
            /* if "N>&M": dup newfd to redir->rd_fd */
            /* if "N>&-": close redir->rd_fd (newfd is REDIRFD_CLOSE) */
            closed = save_fd_on_redirect((*redir).rd_fd, newfd, sqp);
            if newfd == REDIRFD_CLOSE as libc::c_int {
              /* "N>&-" means "close me" */
              if closed == 0 {
                /* ^^^ optimization: saving may already
                 * have closed it. If not... */
                close((*redir).rd_fd);
              }
            /* Sometimes we do another close on restore, getting EBADF.
             * Consider "echo 3>FILE 3>&-"
             * first redirect remembers "need to close 3",
             * and second redirect closes 3! Restore code then closes 3 again.
             */
            } else {
              /* if newfd is a script fd or saved fd, simulate EBADF */
              if internally_opened_fd(
                newfd,
                if !sqp.is_null()
                  && sqp != 1i32 as libc::c_long as *mut libc::c_void as *mut *mut squirrel
                {
                  *sqp
                } else {
                  0 as *mut squirrel
                },
              ) != 0
              {
                //errno = EBADF;
                //bb_perror_msg_and_die("can't duplicate file descriptor");
                newfd = -1i32
                /* same effect as code above */
              }
              xdup2(newfd, (*redir).rd_fd);
              if (*redir).rd_dup == REDIRFD_TO_FILE as libc::c_int {
                /* else: "rd_fd > rd_dup" */
                /* "rd_fd > FILE" */
                close(newfd); /* must be int! */
              }
            }
          }
        }
      }
    }
    redir = (*redir).next
  }
  return 0i32;
}
unsafe extern "C" fn find_in_path(mut arg: *const libc::c_char) -> *mut libc::c_char {
  let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut PATH: *const libc::c_char =
    get_local_var_value(b"PATH\x00" as *const u8 as *const libc::c_char);
  if PATH.is_null() {
    return 0 as *mut libc::c_char;
  }
  loop {
    let mut end: *const libc::c_char = strchrnul(PATH, ':' as i32);
    let mut sz: libc::c_int = end.wrapping_offset_from(PATH) as libc::c_long as libc::c_int;
    free(ret as *mut libc::c_void);
    if sz != 0i32 {
      ret = xasprintf(
        b"%.*s/%s\x00" as *const u8 as *const libc::c_char,
        sz,
        PATH,
        arg,
      )
    } else {
      /* We have xxx::yyyy in $PATH,
       * it means "use current dir" */
      ret = xstrdup(arg)
    }
    if access(ret, 0i32) == 0i32 {
      break;
    }
    if *end as libc::c_int == '\u{0}' as i32 {
      free(ret as *mut libc::c_void);
      return 0 as *mut libc::c_char;
    }
    PATH = end.offset(1)
  }
  return ret;
}
unsafe extern "C" fn find_builtin_helper(
  mut name: *const libc::c_char,
  mut x: *const built_in_command,
  mut end: *const built_in_command,
) -> *const built_in_command {
  while x != end {
    if strcmp(name, (*x).b_cmd) != 0i32 {
      x = x.offset(1)
    } else {
      return x;
    }
  }
  return 0 as *const built_in_command;
}
unsafe extern "C" fn find_builtin1(mut name: *const libc::c_char) -> *const built_in_command {
  return find_builtin_helper(
    name,
    bltins1.as_ptr(),
    &*bltins1.as_ptr().offset(
      (::std::mem::size_of::<[built_in_command; 31]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<built_in_command>() as libc::c_ulong)
        as libc::c_uint as isize,
    ),
  );
}
unsafe extern "C" fn find_builtin(mut name: *const libc::c_char) -> *const built_in_command {
  let mut x: *const built_in_command = find_builtin1(name);
  if !x.is_null() {
    return x;
  }
  return find_builtin_helper(
    name,
    bltins2.as_ptr(),
    &*bltins2.as_ptr().offset(
      (::std::mem::size_of::<[built_in_command; 6]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<built_in_command>() as libc::c_ulong)
        as libc::c_uint as isize,
    ),
  );
}
unsafe extern "C" fn remove_nested_vars() {
  let mut cur: *mut variable = 0 as *mut variable;
  let mut cur_pp: *mut *mut variable = 0 as *mut *mut variable;
  cur_pp = &mut (*ptr_to_globals).top_var;
  loop {
    cur = *cur_pp;
    if cur.is_null() {
      break;
    }
    if (*cur).var_nest_level as libc::c_uint <= (*ptr_to_globals).var_nest_level {
      cur_pp = &mut (*cur).next
    } else {
      /* Unexport */
      if (*cur).flg_export != 0 {
        bb_unsetenv((*cur).varstr);
      }
      /* Remove from global list */
      *cur_pp = (*cur).next;
      /* Free */
      if (*cur).max_len == 0 {
        free((*cur).varstr as *mut libc::c_void);
      }
      free(cur as *mut libc::c_void);
    }
  }
}
unsafe extern "C" fn enter_var_nest_level() {
  (*ptr_to_globals).var_nest_level = (*ptr_to_globals).var_nest_level.wrapping_add(1);
  /* Try:	f() { echo -n .; f; }; f
   * struct variable::var_nest_level is uint16_t,
   * thus limiting recursion to < 2^16.
   * In any case, with 8 Mbyte stack SEGV happens
   * not too long after 2^16 recursions anyway.
   */
  if (*ptr_to_globals).var_nest_level > 0xff00i32 as libc::c_uint {
    bb_error_msg_and_die(
      b"fatal recursion (depth %u)\x00" as *const u8 as *const libc::c_char,
      (*ptr_to_globals).var_nest_level,
    );
  };
}
unsafe extern "C" fn leave_var_nest_level() {
  (*ptr_to_globals).var_nest_level = (*ptr_to_globals).var_nest_level.wrapping_sub(1);
  if 1i32 != 0 && ((*ptr_to_globals).var_nest_level as libc::c_int) < 0i32 {
    bb_simple_error_msg_and_die(b"BUG: nesting underflow\x00" as *const u8 as *const libc::c_char);
  }
  remove_nested_vars();
}
unsafe extern "C" fn find_function_slot(mut name: *const libc::c_char) -> *mut *mut function {
  let mut funcp: *mut function = 0 as *mut function;
  let mut funcpp: *mut *mut function = &mut (*ptr_to_globals).top_func;
  loop {
    funcp = *funcpp;
    if funcp.is_null() {
      break;
    }
    if strcmp(name, (*funcp).name) == 0i32 {
      break;
    }
    funcpp = &mut (*funcp).next
  }
  return funcpp;
}
#[inline(always)]
unsafe extern "C" fn find_function(mut name: *const libc::c_char) -> *const function {
  let mut funcp: *const function = *find_function_slot(name);
  return funcp;
}
/* Note: takes ownership on name ptr */
unsafe extern "C" fn new_function(mut name: *mut libc::c_char) -> *mut function {
  let mut funcpp: *mut *mut function = find_function_slot(name);
  let mut funcp: *mut function = *funcpp;
  if !funcp.is_null() {
    let mut cmd: *mut command = (*funcp).parent_cmd;
    if cmd.is_null() {
      free((*funcp).name as *mut libc::c_void);
      /* Note: if !funcp->body, do not free body_as_string!
       * This is a special case of "-F name body" function:
       * body_as_string was not malloced! */
      if !(*funcp).body.is_null() {
        free_pipe_list((*funcp).body);
      }
    } else {
      let ref mut fresh35 = *(*cmd).argv.offset(0);
      *fresh35 = (*funcp).name;
      (*cmd).group = (*funcp).body
    }
  } else {
    *funcpp = xzalloc(::std::mem::size_of::<function>() as libc::c_ulong) as *mut function;
    funcp = *funcpp
    /*funcp->next = NULL;*/
  }
  (*funcp).name = name;
  return funcp;
}
unsafe extern "C" fn unset_func(mut name: *const libc::c_char) {
  let mut funcpp: *mut *mut function = find_function_slot(name);
  let mut funcp: *mut function = *funcpp;
  if !funcp.is_null() {
    *funcpp = (*funcp).next;
    /* funcp is unlinked now, deleting it.
     * Note: if !funcp->body, the function was created by
     * "-F name body", do not free ->body_as_string
     * and ->name as they were not malloced. */
    if !(*funcp).body.is_null() {
      free_pipe_list((*funcp).body);
      free((*funcp).name as *mut libc::c_void);
    }
    free(funcp as *mut libc::c_void);
  };
}
unsafe extern "C" fn exec_function(
  mut funcp: *const function,
  mut argv: *mut *mut libc::c_char,
) -> ! {
  let mut n: libc::c_int = 0;
  let ref mut fresh36 = *argv.offset(0);
  *fresh36 = *(*ptr_to_globals).global_argv.offset(0);
  (*ptr_to_globals).global_argv = argv;
  n = (1i32 as libc::c_uint).wrapping_add(string_array_len(argv.offset(1))) as libc::c_int;
  (*ptr_to_globals).global_argc = n;
  // Example when we are here: "cmd | func"
  // func will run with saved-redirect fds open.
  // $ f() { echo /proc/self/fd/*; }
  // $ true | f
  // /proc/self/fd/0 /proc/self/fd/1 /proc/self/fd/2 /proc/self/fd/255 /proc/self/fd/3
  // stdio^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ G_interactive_fd^ DIR fd for glob
  // Same in script:
  // $ . ./SCRIPT
  // /proc/self/fd/0 /proc/self/fd/1 /proc/self/fd/2 /proc/self/fd/255 /proc/self/fd/3 /proc/self/fd/4
  // stdio^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ G_interactive_fd^ opened ./SCRIPT DIR fd for glob
  // They are CLOEXEC so external programs won't see them, but
  // for "more correctness" we might want to close those extra fds here:
  //?	close_saved_fds_and_FILE_fds();
  /* "we are in a function, ok to use return" */
  (*ptr_to_globals).flag_return_in_progress = -1i32 as smallint;
  enter_var_nest_level();
  (*ptr_to_globals).func_nest_level = (*ptr_to_globals).func_nest_level.wrapping_add(1);
  /* On MMU, funcp->body is always non-NULL */
  n = run_list((*funcp).body);
  fflush_all();
  _exit(n);
}
unsafe extern "C" fn run_function(
  mut funcp: *const function,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut rc: libc::c_int = 0;
  let mut sv: save_arg_t = save_arg_t {
    sv_argv0: 0 as *mut libc::c_char,
    sv_g_argv: 0 as *mut *mut libc::c_char,
    sv_g_argc: 0,
    sv_g_malloced: 0,
  };
  let mut sv_flg: smallint = 0;
  save_and_replace_G_args(&mut sv, argv);
  /* "We are in function, ok to use return" */
  sv_flg = (*ptr_to_globals).flag_return_in_progress;
  (*ptr_to_globals).flag_return_in_progress = -1i32 as smallint;
  /* Make "local" variables properly shadow previous ones */
  enter_var_nest_level();
  (*ptr_to_globals).func_nest_level = (*ptr_to_globals).func_nest_level.wrapping_add(1);
  /* On MMU, funcp->body is always non-NULL */
  rc = run_list((*funcp).body);
  (*ptr_to_globals).func_nest_level = (*ptr_to_globals).func_nest_level.wrapping_sub(1);
  leave_var_nest_level();
  (*ptr_to_globals).flag_return_in_progress = sv_flg;
  restore_G_args(&mut sv, argv);
  return rc;
}
/* ENABLE_HUSH_FUNCTIONS */
unsafe extern "C" fn exec_builtin(
  mut x: *const built_in_command,
  mut argv: *mut *mut libc::c_char,
) -> ! {
  let mut rcode: libc::c_int = 0;
  fflush_all();
  //?	close_saved_fds_and_FILE_fds();
  rcode = (*x).b_function.expect("non-null function pointer")(argv);
  fflush_all();
  _exit(rcode);
}
unsafe extern "C" fn execvp_or_die(mut argv: *mut *mut libc::c_char) -> ! {
  let mut e: libc::c_int = 0;
  /* Don't propagate SIG_IGN to the child */
  if SPECIAL_JOBSTOP_SIGS as libc::c_int != 0i32 {
    switch_off_special_sigs(
      (*ptr_to_globals).special_sig_mask & SPECIAL_JOBSTOP_SIGS as libc::c_int as libc::c_uint,
    );
  }
  execvp(*argv.offset(0), argv as *const *mut libc::c_char);
  e = 2i32;
  if *bb_errno == 13i32 {
    e = 126i32
  }
  if *bb_errno == 2i32 {
    e = 127i32
  }
  bb_perror_msg(
    b"can\'t execute \'%s\'\x00" as *const u8 as *const libc::c_char,
    *argv.offset(0),
  );
  _exit(e);
}
unsafe extern "C" fn x_mode_print_optionally_squoted(mut str: *const libc::c_char) {
  let mut len: libc::c_uint = 0;
  let mut cp: *const libc::c_char = 0 as *const libc::c_char;
  cp = str;
  /* the set of chars which-cause-string-to-be-squoted mimics bash */
  /* test a char with: bash -c 'set -x; echo "CH"' */
  if *str.offset(strcspn(str,
                           b"\\\"\'`$(){}[]<>;#&|~*?!^ \x01\x02\x03\x04\x05\x06\x07\x08\t\n\x0b\x0c\r\x0e\x0f\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1a\x1b\x1c\x1d\x1e\x1f\x00"
                               as *const u8 as *const libc::c_char) as isize)
           as libc::c_int == '\u{0}' as i32 {
        /* string has no special chars */
        x_mode_addstr(str);
        return
    }
  cp = str;
  loop {
    /* print '....' up to EOL or first squote */
    len = strchrnul(cp, '\'' as i32).wrapping_offset_from(cp) as libc::c_long as libc::c_int
      as libc::c_uint;
    if len != 0i32 as libc::c_uint {
      x_mode_addchr('\'' as i32);
      x_mode_addblock(cp, len as libc::c_int);
      x_mode_addchr('\'' as i32);
      cp = cp.offset(len as isize)
    }
    if *cp as libc::c_int == '\u{0}' as i32 {
      break;
    }
    /* string contains squote(s), print them as \' */
    x_mode_addchr('\\' as i32);
    x_mode_addchr('\'' as i32);
    cp = cp.offset(1)
  }
}
unsafe extern "C" fn dump_cmd_in_x_mode(mut argv: *mut *mut libc::c_char) {
  if (*ptr_to_globals).o_opt[OPT_O_XTRACE as libc::c_int as usize] as libc::c_int != 0
    && !argv.is_null()
  {
    let mut n: libc::c_uint = 0;
    /* "+[+++...][ cmd...]\n\0" */
    x_mode_prefix();
    n = 0i32 as libc::c_uint;
    while !(*argv.offset(n as isize)).is_null() {
      x_mode_addchr(' ' as i32);
      if *(*argv.offset(n as isize)).offset(0) as libc::c_int == '\u{0}' as i32 {
        x_mode_addchr('\'' as i32);
        x_mode_addchr('\'' as i32);
      } else {
        x_mode_print_optionally_squoted(*argv.offset(n as isize));
      }
      n = n.wrapping_add(1)
    }
    x_mode_flush();
  };
}
unsafe extern "C" fn if_command_vV_print_and_exit(
  mut opt_vV: libc::c_char,
  mut cmd: *mut libc::c_char,
  mut explanation: *const libc::c_char,
) {
  let mut to_free: *mut libc::c_char = 0 as *mut libc::c_char;
  if opt_vV == 0 {
    return;
  }
  to_free = 0 as *mut libc::c_char;
  if explanation.is_null() {
    let mut path: *mut libc::c_char = getenv(b"PATH\x00" as *const u8 as *const libc::c_char);
    /* -v PROG prints "/path/to/PROG" */
    to_free = find_executable(cmd, &mut path); /* path == NULL is ok */
    explanation = to_free; /* PROG was not found */
    if explanation.is_null() {
      _exit(1i32);
    }
    if opt_vV as libc::c_int != 'V' as i32 {
      cmd = to_free
    }
  }
  printf(
    if opt_vV as libc::c_int == 'V' as i32 {
      b"%s is %s\n\x00" as *const u8 as *const libc::c_char
    } else {
      b"%s\n\x00" as *const u8 as *const libc::c_char
    },
    cmd,
    explanation,
  );
  free(to_free as *mut libc::c_void);
  fflush_all();
  _exit(0i32);
}
/* Called after [v]fork() in run_pipe, or from builtin_exec.
 * Never returns.
 * Don't exit() here.  If you don't exec, use _exit instead.
 * The at_exit handlers apparently confuse the calling process,
 * in particular stdin handling. Not sure why? -- because of vfork! (vda)
 */
#[inline(never)]
unsafe extern "C" fn pseudo_exec_argv(
  mut argv: *mut *mut libc::c_char,
  mut assignment_cnt: libc::c_int,
  mut argv_expanded: *mut *mut libc::c_char,
) -> ! {
  let mut x: *const built_in_command = 0 as *const built_in_command;
  let mut sv_shadowed: *mut *mut variable = 0 as *mut *mut variable;
  let mut new_env: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  let mut opt_vV: libc::c_char = 0i32 as libc::c_char;
  let mut funcp: *const function = 0 as *const function;
  new_env = expand_assignments(argv, assignment_cnt);
  dump_cmd_in_x_mode(new_env);
  if (*argv.offset(assignment_cnt as isize)).is_null() {
    /* Case when we are here: ... | var=val | ...
     * (note that we do not exit early, i.e., do not optimize out
     * expand_assignments(): think about ... | var=`sleep 1` | ...
     */
    free_strings(new_env); /* "don't save, free them instead" */
    _exit(0i32);
  }
  sv_shadowed = (*ptr_to_globals).shadowed_vars_pp;
  (*ptr_to_globals).shadowed_vars_pp = 0 as *mut *mut variable;
  set_vars_and_save_old(new_env);
  (*ptr_to_globals).shadowed_vars_pp = sv_shadowed;
  if !argv_expanded.is_null() {
    argv = argv_expanded
  } else {
    argv = expand_strvec_to_strvec(argv.offset(assignment_cnt as isize))
  }
  dump_cmd_in_x_mode(argv);
  if strchr(*argv.offset(0), '/' as i32).is_null() {
    /* Check if the command matches any functions (this goes before bltins) */
    funcp = find_function(*argv.offset(0));
    if !funcp.is_null() {
      exec_function(funcp, argv);
    }
    /* "command BAR": run BAR without looking it up among functions
     * "command -v BAR": print "BAR" or "/path/to/BAR"; or exit 1
     * "command -V BAR": print "BAR is {a function,a shell builtin,/path/to/BAR}"
     */
    's_102: while strcmp(
      *argv.offset(0),
      b"command\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
      && !(*argv.offset(1)).is_null()
    {
      let mut p: *mut libc::c_char = 0 as *mut libc::c_char; /* bash allows "command command command [-OPT] BAR" */
      argv = argv.offset(1); /* next arg is also -opts, process it too */
      p = *argv;
      if *p.offset(0) as libc::c_int != '-' as i32 || *p.offset(1) == 0 {
        continue;
      }
      loop {
        p = p.offset(1);
        match *p as libc::c_int {
          0 => {
            argv = argv.offset(1);
            p = *argv;
            if *p.offset(0) as libc::c_int != '-' as i32 || *p.offset(1) == 0 {
              break 's_102;
            }
          }
          118 | 86 => opt_vV = *p,
          _ => {
            bb_error_msg_and_die(
              b"%s: %s: invalid option\x00" as *const u8 as *const libc::c_char,
              b"command\x00" as *const u8 as *const libc::c_char,
              *argv.offset(0),
            );
          }
        }
      }
    }
    if opt_vV as libc::c_int != 0 && !find_function(*argv.offset(0)).is_null() {
      if_command_vV_print_and_exit(
        opt_vV,
        *argv.offset(0),
        b"a function\x00" as *const u8 as *const libc::c_char,
      );
    }
    /* Check if the command matches any of the builtins.
     * Depending on context, this might be redundant.  But it's
     * easier to waste a few CPU cycles than it is to figure out
     * if this is one of those cases.
     */
    /* Why "BB_MMU ? :" difference in logic? -
     * On NOMMU, it is more expensive to re-execute shell
     * just in order to run echo or test builtin.
     * It's better to skip it here and run corresponding
     * non-builtin later. */
    x = if 1i32 != 0 {
      find_builtin(*argv.offset(0))
    } else {
      find_builtin1(*argv.offset(0))
    };
    if !x.is_null() {
      if_command_vV_print_and_exit(
        opt_vV,
        *argv.offset(0),
        b"a shell builtin\x00" as *const u8 as *const libc::c_char,
      );
      exec_builtin(x, argv);
    }
  }
  if_command_vV_print_and_exit(opt_vV, *argv.offset(0), 0 as *const libc::c_char);
  execvp_or_die(argv);
}
/* Called after [v]fork() in run_pipe
 */
unsafe extern "C" fn pseudo_exec(
  mut command: *mut command,
  mut argv_expanded: *mut *mut libc::c_char,
) -> ! {
  if (*command).cmd_type as libc::c_int == 3i32 {
    /* Ignore funcdefs in pipes:
     * true | f() { cmd }
     */
    _exit(0i32);
  }
  if !(*command).argv.is_null() {
    pseudo_exec_argv(
      (*command).argv,
      (*command).assignment_cnt as libc::c_int,
      argv_expanded,
    );
  }
  if !(*command).group.is_null() {
    /* Cases when we are here:
     * ( list )
     * { list } &
     * ... | ( list ) | ...
     * ... | { list } | ...
     */
    let mut rcode: libc::c_int = 0;
    reset_traps_to_defaults();
    rcode = run_list((*command).group);
    /* OK to leak memory by not calling free_pipe_list,
     * since this process is about to exit */
    _exit(rcode);
  }
  /* Case when we are here: ... | >file */
  _exit(0i32);
}
unsafe extern "C" fn get_cmdtext(mut pi: *mut pipe) -> *const libc::c_char {
  let mut argv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut len: libc::c_int = 0;
  /* This is subtle. ->cmdtext is created only on first backgrounding.
   * (Think "cat, <ctrl-z>, fg, <ctrl-z>, fg, <ctrl-z>...." here...)
   * On subsequent bg argv is trashed, but we won't use it */
  if !(*pi).cmdtext.is_null() {
    return (*pi).cmdtext;
  }
  argv = (*(*pi).cmds.offset(0)).argv;
  if argv.is_null() {
    (*pi).cmdtext = xzalloc(1i32 as size_t) as *mut libc::c_char;
    return (*pi).cmdtext;
  }
  len = 0i32;
  loop {
    len = (len as libc::c_ulong).wrapping_add(strlen(*argv).wrapping_add(1i32 as libc::c_ulong))
      as libc::c_int as libc::c_int;
    argv = argv.offset(1);
    if (*argv).is_null() {
      break;
    }
  }
  p = xmalloc(len as size_t) as *mut libc::c_char;
  (*pi).cmdtext = p;
  argv = (*(*pi).cmds.offset(0)).argv;
  loop {
    p = stpcpy(p, *argv);
    let fresh37 = p;
    p = p.offset(1);
    *fresh37 = ' ' as i32 as libc::c_char;
    argv = argv.offset(1);
    if (*argv).is_null() {
      break;
    }
  }
  *p.offset(-1i32 as isize) = '\u{0}' as i32 as libc::c_char;
  return (*pi).cmdtext;
}
unsafe extern "C" fn remove_job_from_table(mut pi: *mut pipe) {
  let mut prev_pipe: *mut pipe = 0 as *mut pipe;
  if pi == (*ptr_to_globals).job_list {
    (*ptr_to_globals).job_list = (*pi).next
  } else {
    prev_pipe = (*ptr_to_globals).job_list;
    while (*prev_pipe).next != pi {
      prev_pipe = (*prev_pipe).next
    }
    (*prev_pipe).next = (*pi).next
  }
  (*ptr_to_globals).last_jobid = 0i32 as libc::c_uint;
  if !(*ptr_to_globals).job_list.is_null() {
    (*ptr_to_globals).last_jobid = (*(*ptr_to_globals).job_list).jobid
  };
}
unsafe extern "C" fn delete_finished_job(mut pi: *mut pipe) {
  remove_job_from_table(pi);
  free_pipe(pi);
}
unsafe extern "C" fn clean_up_last_dead_job() {
  if !(*ptr_to_globals).job_list.is_null() && (*(*ptr_to_globals).job_list).alive_cmds == 0 {
    delete_finished_job((*ptr_to_globals).job_list);
  };
}
unsafe extern "C" fn insert_job_into_table(mut pi: *mut pipe) {
  let mut job: *mut pipe = 0 as *mut pipe;
  let mut jobp: *mut *mut pipe = 0 as *mut *mut pipe;
  let mut i: libc::c_int = 0;
  clean_up_last_dead_job();
  /* Find the end of the list, and find next job ID to use */
  i = 0i32;
  jobp = &mut (*ptr_to_globals).job_list;
  loop {
    job = *jobp;
    if job.is_null() {
      break;
    }
    if (*job).jobid > i as libc::c_uint {
      i = (*job).jobid as libc::c_int
    }
    jobp = &mut (*job).next
  }
  (*pi).jobid = (i + 1i32) as libc::c_uint;
  /* Create a new job struct at the end */
  *jobp = xmemdup(
    pi as *const libc::c_void,
    ::std::mem::size_of::<pipe>() as libc::c_ulong as libc::c_int,
  ) as *mut pipe;
  job = *jobp;
  (*job).next = 0 as *mut pipe;
  (*job).cmds = xzalloc(
    (::std::mem::size_of::<command>() as libc::c_ulong)
      .wrapping_mul((*pi).num_cmds as libc::c_ulong),
  ) as *mut command;
  /* Cannot copy entire pi->cmds[] vector! This causes double frees */
  i = 0i32;
  while i < (*pi).num_cmds {
    (*(*job).cmds.offset(i as isize)).pid = (*(*pi).cmds.offset(i as isize)).pid;
    i += 1
    /* all other fields are not used and stay zero */
  }
  (*job).cmdtext = xstrdup(get_cmdtext(pi));
  if (*ptr_to_globals).interactive_fd != 0 {
    printf(
      b"[%u] %u %s\n\x00" as *const u8 as *const libc::c_char,
      (*job).jobid,
      (*(*job).cmds.offset(0)).pid as libc::c_uint,
      (*job).cmdtext,
    );
  }
  (*ptr_to_globals).last_jobid = (*job).jobid;
}
/* JOB */
unsafe extern "C" fn job_exited_or_stopped(mut pi: *mut pipe) -> libc::c_int {
  let mut rcode: libc::c_int = 0;
  let mut i: libc::c_int = 0;
  if (*pi).alive_cmds != (*pi).stopped_cmds {
    return -1i32;
  }
  /* All processes in fg pipe have exited or stopped */
  rcode = 0i32;
  i = (*pi).num_cmds;
  loop {
    i -= 1;
    if !(i >= 0i32) {
      break;
    }
    rcode = (*(*pi).cmds.offset(i as isize)).cmd_exitcode as libc::c_int;
    /* usually last process gives overall exitstatus,
     * but with "set -o pipefail", last *failed* process does */
    if (*ptr_to_globals).o_opt[OPT_O_PIPEFAIL as libc::c_int as usize] as libc::c_int == 0i32
      || rcode != 0i32
    {
      break;
    }
  }
  if (*pi).pi_inverted != 0 {
    rcode = (rcode == 0) as libc::c_int
  }
  return rcode;
}
unsafe extern "C" fn process_wait_result(
  mut fg_pipe: *mut pipe,
  mut childpid: pid_t,
  mut status: libc::c_int,
) -> libc::c_int {
  let mut current_block: u64;
  let mut pi: *mut pipe = 0 as *mut pipe;
  let mut i: libc::c_int = 0;
  let mut dead: libc::c_int = 0;
  dead = (status & 0x7fi32 == 0i32
    || ((status & 0x7fi32) + 1i32) as libc::c_schar as libc::c_int >> 1i32 > 0i32)
    as libc::c_int;
  /* Were we asked to wait for a fg pipe? */
  if !fg_pipe.is_null() {
    i = (*fg_pipe).num_cmds;
    loop {
      i -= 1;
      if !(i >= 0i32) {
        break;
      }
      let mut rcode: libc::c_int = 0;
      if (*(*fg_pipe).cmds.offset(i as isize)).pid != childpid {
        continue;
      }
      if dead != 0 {
        let mut ex: libc::c_int = 0;
        (*(*fg_pipe).cmds.offset(i as isize)).pid = 0i32;
        (*fg_pipe).alive_cmds -= 1;
        ex = (status & 0xff00i32) >> 8i32;
        /* It wasn't in fg_pipe, look for process in bg pipes */
        /* bash prints killer signal's name for *last*
         * process in pipe (prints just newline for SIGINT/SIGPIPE).
         * Mimic this. Example: "sleep 5" + (^\ or kill -QUIT)
         */
        if ((status & 0x7fi32) + 1i32) as libc::c_schar as libc::c_int >> 1i32 > 0i32 {
          let mut sig: libc::c_int = status & 0x7fi32;
          if i == (*fg_pipe).num_cmds - 1i32 {
            /* TODO: use strsignal() instead for bash compat? but that's bloat... */
            puts(if sig == 2i32 || sig == 13i32 {
              b"\x00" as *const u8 as *const libc::c_char
            } else {
              get_signame(sig)
            });
          }
          /* TODO: if (WCOREDUMP(status)) + " (core dumped)"; */
          /* TODO: MIPS has 128 sigs (1..128), what if sig==128 here?
           * Maybe we need to use sig | 128? */
          ex = sig + 128i32
        }
        (*(*fg_pipe).cmds.offset(i as isize)).cmd_exitcode = ex as smalluint
      } else {
        (*fg_pipe).stopped_cmds += 1
      }
      rcode = job_exited_or_stopped(fg_pipe);
      if rcode >= 0i32 {
        /* Note: *non-interactive* bash does not continue if all processes in fg pipe
         * are stopped. Testcase: "cat | cat" in a script (not on command line!)
         * and "killall -STOP cat" */
        if (*ptr_to_globals).interactive_fd != 0 {
          if (*fg_pipe).alive_cmds != 0i32 {
            insert_job_into_table(fg_pipe);
          }
          return rcode;
        }
        if (*fg_pipe).alive_cmds == 0i32 {
          return rcode;
        }
      }
      /* There are still running processes in the fg_pipe */
      return -1i32;
    }
  }
  /* We were asked to wait for bg or orphaned children */
  /* No need to remember exitcode in this case */
  pi = (*ptr_to_globals).job_list;
  's_177: loop {
    if pi.is_null() {
      current_block = 790185930182612747;
      break;
    }
    i = 0i32;
    while i < (*pi).num_cmds {
      if (*(*pi).cmds.offset(i as isize)).pid == childpid {
        current_block = 4076451519832173219;
        break 's_177;
      }
      i += 1
    }
    pi = (*pi).next
  }
  match current_block {
    790185930182612747 =>
    /* Happens when shell is used as init process (init=/bin/sh) */
    {
      return -1i32
    }
    _ => {
      if dead != 0 {
        /* this wasn't a process from fg_pipe */
        /* child exited */
        let mut rcode_0: libc::c_int = (status & 0xff00i32) >> 8i32;
        if ((status & 0x7fi32) + 1i32) as libc::c_schar as libc::c_int >> 1i32 > 0i32 {
          rcode_0 = 128i32 + (status & 0x7fi32)
        }
        (*(*pi).cmds.offset(i as isize)).cmd_exitcode = rcode_0 as smalluint;
        if (*ptr_to_globals).last_bg_pid == (*(*pi).cmds.offset(i as isize)).pid {
          (*ptr_to_globals).last_bg_pid_exitcode = rcode_0 as smalluint
        }
        (*(*pi).cmds.offset(i as isize)).pid = 0i32;
        (*pi).alive_cmds -= 1;
        if (*pi).alive_cmds == 0 {
          (*ptr_to_globals).dead_job_exitcode = job_exited_or_stopped(pi);
          if (*ptr_to_globals).interactive_fd != 0 {
            printf(
              b"[%u] %-22s %.40s\n\x00" as *const u8 as *const libc::c_char,
              (*pi).jobid,
              b"Done\x00" as *const u8 as *const libc::c_char,
              (*pi).cmdtext,
            );
            delete_finished_job(pi);
          } else if pi != (*ptr_to_globals).job_list {
            delete_finished_job(pi);
          }
        }
      } else {
        /*
         * bash deletes finished jobs from job table only in interactive mode,
         * after "jobs" cmd, or if pid of a new process matches one of the old ones
         * (see cleanup_dead_jobs(), delete_old_job(), J_NOTIFIED in bash source).
         * Testcase script: "(exit 3) & sleep 1; wait %1; echo $?" prints 3 in bash.
         * We only retain one "dead" job, if it's the single job on the list.
         * This covers most of real-world scenarios where this is useful.
         */
        /* child stopped */
        (*pi).stopped_cmds += 1
      }
      return -1i32;
    }
  };
  /* this wasn't a process from fg_pipe */
}
/* Check to see if any processes have exited -- if they have,
 * figure out why and see if a job has completed.
 *
 * If non-NULL fg_pipe: wait for its completion or stop.
 * Return its exitcode or zero if stopped.
 *
 * Alternatively (fg_pipe == NULL, waitfor_pid != 0):
 * waitpid(WNOHANG), if waitfor_pid exits or stops, return exitcode+1,
 * else return <0 if waitpid errors out (e.g. ECHILD: nothing to wait for)
 * or 0 if no children changed status.
 *
 * Alternatively (fg_pipe == NULL, waitfor_pid == 0),
 * return <0 if waitpid errors out (e.g. ECHILD: nothing to wait for)
 * or 0 if no children changed status.
 */
unsafe extern "C" fn checkjobs(mut fg_pipe: *mut pipe, mut waitfor_pid: pid_t) -> libc::c_int {
  let mut attributes: libc::c_int = 0;
  let mut status: libc::c_int = 0;
  let mut rcode: libc::c_int = 0i32;
  attributes = 2i32;
  if fg_pipe.is_null() {
    attributes |= 1i32
  }
  *bb_errno = 0i32;
  loop
  /* Do we do this right?
   * bash-3.00# sleep 20 | false
   * <ctrl-Z pressed>
   * [3]+  Stopped          sleep 20 | false
   * bash-3.00# echo $?
   * 1   <========== bg pipe is not fully done, but exitcode is already known!
   * [hush 1.14.0: yes we do it right]
   */
  {
    let mut childpid: pid_t = 0; /* while (waitpid succeeds)... */
    childpid = waitpid(-1i32, &mut status, attributes);
    if childpid <= 0i32 {
      if childpid != 0 && *bb_errno != 10i32 {
        bb_simple_perror_msg(b"waitpid\x00" as *const u8 as *const libc::c_char);
      }
      /* ECHILD (no children), or 0 (no change in children status) */
      rcode = childpid;
      break;
    } else {
      rcode = process_wait_result(fg_pipe, childpid, status);
      if rcode >= 0i32 {
        break;
      }
      if childpid == waitfor_pid {
        /* "wait PID" */
        rcode = (status & 0xff00i32) >> 8i32;
        if ((status & 0x7fi32) + 1i32) as libc::c_schar as libc::c_int >> 1i32 > 0i32 {
          rcode = 128i32 + (status & 0x7fi32)
        }
        if status & 0xffi32 == 0x7fi32 {
          /* bash: "cmd & wait $!" and cmd stops: $? = 128 + stopsig */
          rcode = 128i32 + ((status & 0xff00i32) >> 8i32)
        }
        rcode += 1;
        break;
      /* "wait PID" called us, give it exitcode+1 */
      } else {
        if !(-1i32 == waitfor_pid && (*ptr_to_globals).dead_job_exitcode >= 0i32) {
          continue;
        }
        /* some job did finish */
        rcode = (*ptr_to_globals).dead_job_exitcode + 1i32;
        break;
      }
    }
  }
  return rcode;
}
unsafe extern "C" fn checkjobs_and_fg_shell(mut fg_pipe: *mut pipe) -> libc::c_int {
  let mut p: pid_t = 0;
  let mut rcode: libc::c_int = checkjobs(fg_pipe, 0i32);
  if (*ptr_to_globals).saved_tty_pgrp != 0 {
    /* Job finished, move the shell to the foreground */
    p = getpgrp(); /* our process group id */
    tcsetpgrp((*ptr_to_globals).interactive_fd, p);
  }
  return rcode;
}
/* Start all the jobs, but don't wait for anything to finish.
 * See checkjobs().
 *
 * Return code is normally -1, when the caller has to wait for children
 * to finish to determine the exit status of the pipe.  If the pipe
 * is a simple builtin command, however, the action is done by the
 * time run_pipe returns, and the exit code is provided as the
 * return value.
 *
 * Returns -1 only if started some children. IOW: we have to
 * mask out retvals of builtins etc with 0xff!
 *
 * The only case when we do not need to [v]fork is when the pipe
 * is single, non-backgrounded, non-subshell command. Examples:
 * cmd ; ...   { list } ; ...
 * cmd && ...  { list } && ...
 * cmd || ...  { list } || ...
 * If it is, then we can run cmd as a builtin, NOFORK,
 * or (if SH_STANDALONE) an applet, and we can run the { list }
 * with run_list. If it isn't one of these, we fork and exec cmd.
 *
 * Cases when we must fork:
 * non-single:   cmd | cmd
 * backgrounded: cmd &     { list } &
 * subshell:     ( list ) [&]
 */
unsafe extern "C" fn redirect_and_varexp_helper(
  mut command: *mut command,
  mut sqp: *mut *mut squirrel,
  mut argv_expanded: *mut *mut libc::c_char,
) -> libc::c_int {
  /* Assignments occur before redirects. Try:
   * a=`sleep 1` sleep 2 3>/qwe/rty
   */
  let mut new_env: *mut *mut libc::c_char =
    expand_assignments((*command).argv, (*command).assignment_cnt as libc::c_int);
  dump_cmd_in_x_mode(new_env);
  dump_cmd_in_x_mode(argv_expanded);
  /* this takes ownership of new_env[i] elements, and frees new_env: */
  set_vars_and_save_old(new_env);
  return setup_redirects(command, sqp);
}
#[inline(never)]
unsafe extern "C" fn run_pipe(mut pi: *mut pipe) -> libc::c_int {
  let mut i: libc::c_uint = 0;
  let mut current_block: u64;
  static mut null_ptr: *const libc::c_char = 0 as *const libc::c_char;
  let mut cmd_no: libc::c_int = 0;
  let mut next_infd: libc::c_int = 0;
  let mut command: *mut command = 0 as *mut command;
  let mut argv_expanded: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  let mut argv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  let mut squirrel: *mut squirrel = 0 as *mut squirrel;
  let mut rcode: libc::c_int = 0;
  /* Testcase: set -- q w e; (IFS='' echo "$*"; IFS=''; echo "$*"); echo "$*"
   * Result should be 3 lines: q w e, qwe, q w e
   */
  if (*ptr_to_globals).ifs_whitespace != (*ptr_to_globals).ifs as *mut libc::c_char {
    free((*ptr_to_globals).ifs_whitespace as *mut libc::c_void);
  }
  (*ptr_to_globals).ifs = get_local_var_value(b"IFS\x00" as *const u8 as *const libc::c_char);
  if !(*ptr_to_globals).ifs.is_null() {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    (*ptr_to_globals).ifs_whitespace = (*ptr_to_globals).ifs as *mut libc::c_char;
    p = skip_whitespace((*ptr_to_globals).ifs);
    if *p != 0 {
      /* Not all $IFS is whitespace */
      let mut d: *mut libc::c_char = 0 as *mut libc::c_char; /* can overestimate */
      let mut len: libc::c_int =
        p.wrapping_offset_from((*ptr_to_globals).ifs) as libc::c_long as libc::c_int;
      p = skip_non_whitespace(p);
      (*ptr_to_globals).ifs_whitespace = xmalloc(
        (len as libc::c_ulong)
          .wrapping_add(strlen(p))
          .wrapping_add(1i32 as libc::c_ulong),
      ) as *mut libc::c_char;
      d = mempcpy(
        (*ptr_to_globals).ifs_whitespace as *mut libc::c_void,
        (*ptr_to_globals).ifs as *const libc::c_void,
        len as size_t,
      ) as *mut libc::c_char;
      while *p != 0 {
        if ({
          let mut bb__isspace: libc::c_uchar = (*p as libc::c_int - 9i32) as libc::c_uchar;
          (bb__isspace as libc::c_int == ' ' as i32 - 9i32
            || bb__isspace as libc::c_int <= 13i32 - 9i32) as libc::c_int
        }) != 0
        {
          let fresh38 = d;
          d = d.offset(1);
          *fresh38 = *p
        }
        p = p.offset(1)
      }
      *d = '\u{0}' as i32 as libc::c_char
    }
  } else {
    (*ptr_to_globals).ifs = defifsvar.as_ptr().offset(4);
    (*ptr_to_globals).ifs_whitespace = (*ptr_to_globals).ifs as *mut libc::c_char
  }
  (*pi).pgrp = -1i32;
  (*pi).stopped_cmds = 0i32;
  command = &mut *(*pi).cmds.offset(0) as *mut command;
  argv_expanded = 0 as *mut *mut libc::c_char;
  if !((*pi).num_cmds != 1i32
    || (*pi).followup as libc::c_int == PIPE_BG as libc::c_int
    || (*command).cmd_type as libc::c_int == 1i32)
  {
    (*pi).alive_cmds = 1i32;
    if !(*command).group.is_null() {
      if (*command).cmd_type as libc::c_int == 3i32 {
        /* "executing" func () { list } */
        let mut funcp: *mut function = 0 as *mut function;
        funcp = new_function(*(*command).argv.offset(0));
        /* funcp->name is already set to argv[0] */
        (*funcp).body = (*command).group;
        (*command).group = 0 as *mut pipe;
        let ref mut fresh39 = *(*command).argv.offset(0);
        *fresh39 = 0 as *mut libc::c_char;
        (*funcp).parent_cmd = command;
        (*command).child_func = funcp;
        return 0i32;
      }
      /* { list } */
      rcode = 1i32; /* exitcode if redir failed */
      if setup_redirects(command, &mut squirrel) == 0i32 {
        //FIXME: we need to pass squirrel down into run_list()
        //for SH_STANDALONE case, or else this construct:
        // { find /proc/self/fd; true; } >FILE; cmd2
        //has no way of closing saved fd#1 for "find",
        //and in SH_STANDALONE mode, "find" is not execed,
        //therefore CLOEXEC on saved fd does not help.
        rcode = run_list((*command).group) & 0xffi32
      }
      restore_redirects(squirrel);
      if (*pi).pi_inverted != 0 {
        rcode = (rcode == 0) as libc::c_int
      }
      return rcode;
    }
    argv = if !(*command).argv.is_null() {
      (*command).argv
    } else {
      &null_ptr as *const *const libc::c_char as *mut *mut libc::c_char
    };
    let mut x: *const built_in_command = 0 as *const built_in_command;
    let mut funcp_0: *const function = 0 as *const function;
    let mut sv_shadowed: *mut *mut variable = 0 as *mut *mut variable;
    let mut old_vars: *mut variable = 0 as *mut variable;
    (*ptr_to_globals).execute_lineno = (*command).lineno;
    if (*argv.offset((*command).assignment_cnt as isize)).is_null() {
      /* Assignments, but no command.
       * Ensure redirects take effect (that is, create files).
       * Try "a=t >file"
       */
      i = 0;
      (*ptr_to_globals).expand_exitcode = 0i32 as smalluint;
      current_block = 17116254691753230924;
    } else {
      /* Expand the rest into (possibly) many strings each */
      if (*command).cmd_type as libc::c_int == 2i32 {
        argv_expanded =
          expand_strvec_to_strvec_singleword_noglob(argv.offset((*command).assignment_cnt as isize))
      } else {
        argv_expanded = expand_strvec_to_strvec(argv.offset((*command).assignment_cnt as isize))
      }
      /* If someone gives us an empty string: `cmd with empty output` */
      if (*argv_expanded.offset(0)).is_null() {
        free(argv_expanded as *mut libc::c_void);
        /* `false` still has to set exitcode 1 */
        (*ptr_to_globals).expand_exitcode = (*ptr_to_globals).last_exitcode;
        current_block = 17116254691753230924;
      } else {
        old_vars = 0 as *mut variable;
        sv_shadowed = (*ptr_to_globals).shadowed_vars_pp;
        /* Check if argv[0] matches any functions (this goes before bltins) */
        funcp_0 = find_function(*argv_expanded.offset(0));
        x = 0 as *const built_in_command;
        if funcp_0.is_null() {
          x = find_builtin(*argv_expanded.offset(0))
        }
        if !x.is_null() || !funcp_0.is_null() {
          if !x.is_null()
            && (*x).b_function
              == Some(
                builtin_exec as unsafe extern "C" fn(_: *mut *mut libc::c_char) -> libc::c_int,
              )
            && (*argv_expanded.offset(1)).is_null()
          {
            /*
             * Variable assignments are executed, but then "forgotten":
             *  a=`sleep 1;echo A` exec 3>&-; echo $a
             * sleeps, but prints nothing.
             */
            enter_var_nest_level();
            (*ptr_to_globals).shadowed_vars_pp = &mut old_vars;
            rcode = redirect_and_varexp_helper(
              command,
              1i32 as libc::c_long as *mut libc::c_void as *mut *mut squirrel,
              argv_expanded,
            );
            (*ptr_to_globals).shadowed_vars_pp = sv_shadowed;
            current_block = 9346417539490617685;
          } else {
            /* Bump var nesting, or this will leak exported $a:
             * a=b true; env | grep ^a=
             */
            enter_var_nest_level();
            /* Collect all variables "shadowed" by helper
             * (IOW: old vars overridden by "var1=val1 var2=val2 cmd..." syntax)
             * into old_vars list:
             */
            (*ptr_to_globals).shadowed_vars_pp = &mut old_vars;
            rcode = redirect_and_varexp_helper(command, &mut squirrel, argv_expanded);
            if rcode == 0i32 {
              if funcp_0.is_null() {
                /* Do not collect *to old_vars list* vars shadowed
                 * by e.g. "local VAR" builtin (collect them
                 * in the previously nested list instead):
                 * don't want them to be restored immediately
                 * after "local" completes.
                 */
                (*ptr_to_globals).shadowed_vars_pp = sv_shadowed;
                fflush_all();
                rcode =
                  (*x).b_function.expect("non-null function pointer")(argv_expanded) & 0xffi32;
                fflush_all();
              } else {
                rcode = run_function(funcp_0, argv_expanded) & 0xffi32;
                /*
                 * But do collect *to old_vars list* vars shadowed
                 * within function execution. To that end, restore
                 * this pointer _after_ function run:
                 */
                (*ptr_to_globals).shadowed_vars_pp = sv_shadowed
              }
            }
            current_block = 4299703460566765016;
          }
        }
        // else if false && 396i32 > 1i32 {
        // Originally branched on
        //    if (ENABLE_FEATURE_SH_NOFORK && NUM_APPLETS > 1) {

        // let mut n: libc::c_int = find_applet_by_name(*argv_expanded.offset(0));
        // if n < 0i32 || 0i32 == 0 {
        //   current_block = 7411349665265910262;
        // } else {
        //   enter_var_nest_level();
        //   /* Collect all variables "shadowed" by helper into old_vars list */
        //   (*ptr_to_globals).shadowed_vars_pp = &mut old_vars;
        //   rcode = redirect_and_varexp_helper(command, &mut squirrel, argv_expanded);
        //   (*ptr_to_globals).shadowed_vars_pp = sv_shadowed;
        //   if rcode == 0i32 {
        //     /*
        //      * Note: signals (^C) can't interrupt here.
        //      * We remember them and they will be acted upon
        //      * after applet returns.
        //      * This makes applets which can run for a long time
        //      * and/or wait for user input ineligible for NOFORK:
        //      * for example, "yes" or "rm" (rm -i waits for input).
        //      */
        //     rcode = run_nofork_applet(n, argv_expanded)
        //   }
        //   current_block = 4299703460566765016;
        // }
        // }
        else {
          current_block = 7411349665265910262;
        }
        match current_block {
          7411349665265910262 => {}
          _ => {
            match current_block {
              4299703460566765016 => {
                restore_redirects(squirrel);
              }
              _ => {}
            }
            /* rcode=1 can be if redir file can't be opened */
            leave_var_nest_level();
            add_vars(old_vars);
            /*
             * Try "usleep 99999999" + ^C + "echo $?"
             * with FEATURE_SH_NOFORK=y.
             */
            if funcp_0.is_null() {
              /* It was builtin or nofork.
               * if this would be a real fork/execed program,
               * it should have died if a fatal sig was received.
               * But OTOH, there was no separate process,
               * the sig was sent to _shell_, not to non-existing
               * child.
               * Let's just handle ^C only, this one is obvious:
               * we aren't ok with exitcode 0 when ^C was pressed
               * during builtin/nofork.
               */
              if sigismember(&mut (*ptr_to_globals).pending_set, 2i32) != 0 {
                rcode = 128i32 + 2i32
              }
            }
            free(argv_expanded as *mut libc::c_void);
            if (*pi).pi_inverted != 0 {
              rcode = (rcode == 0) as libc::c_int
            }
            return rcode;
          }
        }
      }
    }
    match current_block {
      7411349665265910262 => {}
      _ => {
        rcode = setup_redirects(command, &mut squirrel);
        restore_redirects(squirrel);
        /* Set shell variables */
        i = 0i32 as libc::c_uint;
        while i < (*command).assignment_cnt {
          let mut p_0: *mut libc::c_char = expand_string_to_string(
            *argv.offset(i as isize),
            EXP_FLAG_ESC_GLOB_CHARS as libc::c_int,
            1i32,
          );
          if (*ptr_to_globals).o_opt[OPT_O_XTRACE as libc::c_int as usize] != 0 {
            let mut eq: *mut libc::c_char = 0 as *mut libc::c_char;
            if i == 0i32 as libc::c_uint {
              x_mode_prefix();
            }
            x_mode_addchr(' ' as i32);
            eq = strchrnul(p_0, '=' as i32);
            if *eq != 0 {
              eq = eq.offset(1)
            }
            x_mode_addblock(
              p_0,
              eq.wrapping_offset_from(p_0) as libc::c_long as libc::c_int,
            );
            x_mode_print_optionally_squoted(eq);
            x_mode_flush();
          }
          if set_local_var(p_0, 0i32 as libc::c_uint) != 0 {
            /* assignment to readonly var / putenv error? */
            rcode = 1i32
          }
          i = i.wrapping_add(1)
        }
        /* Redirect error sets $? to 1. Otherwise,
         * if evaluating assignment value set $?, retain it.
         * Else, clear $?:
         *  false; q=`exit 2`; echo $? - should print 2
         *  false; x=1; echo $? - should print 0
         * Because of the 2nd case, we can't just use G.last_exitcode.
         */
        if rcode == 0i32 {
          rcode = (*ptr_to_globals).expand_exitcode as libc::c_int
        }
        if (*pi).pi_inverted != 0 {
          rcode = (rcode == 0) as libc::c_int
        }
        return rcode;
      }
    }
  }
  /* NB: argv_expanded may already be created, and that
   * might include `cmd` runs! Do not rerun it! We *must*
   * use argv_expanded if it's non-NULL */
  /* Going to fork a child per each pipe member */
  (*pi).alive_cmds = 0i32;
  next_infd = 0i32;
  cmd_no = 0i32;
  while cmd_no < (*pi).num_cmds {
    let mut pipefds: fd_pair = fd_pair { rd: 0, wr: 0 };
    command = &mut *(*pi).cmds.offset(cmd_no as isize) as *mut command;
    cmd_no += 1;

    // Originally this was in the C code:
    // if (command->argv) {
    // 	debug_printf_exec(": pipe member '%s' '%s'...\n",
    // 			command->argv[0], command->argv[1]);
    // } else {
    // 	debug_printf_exec(": pipe member with no argv\n");
    // }
    // But it got replaced with this:
    // !(*command).argv.is_null();

    /* pipes are inserted between pairs of commands */
    pipefds.rd = 0i32;
    pipefds.wr = 1i32;
    if cmd_no < (*pi).num_cmds {
      xpipe(&mut pipefds.rd);
    }
    (*ptr_to_globals).execute_lineno = (*command).lineno;
    (*command).pid = if 1i32 != 0 { fork() } else { vfork() };
    if (*command).pid == 0 {
      /* child */
      die_func = ::std::mem::transmute::<
        Option<unsafe extern "C" fn() -> !>,
        Option<unsafe extern "C" fn() -> ()>,
      >(Some(fflush_and__exit as unsafe extern "C" fn() -> !));
      /* pseudo_exec() does not return */
      (*ptr_to_globals).random_gen.galois_LFSR = 0i32; /* or else $RANDOM repeats in child */
      if (*ptr_to_globals).run_list_level == 1i32 && (*ptr_to_globals).interactive_fd != 0 {
        let mut pgrp: pid_t = 0;
        pgrp = (*pi).pgrp;
        if pgrp < 0i32 {
          /* Every child adds itself to new process group
           * with pgid == pid_of_first_child_in_pipe */
          /* true for 1st process only */
          pgrp = getpid()
        }
        if setpgid(0i32, pgrp) == 0i32
          && (*pi).followup as libc::c_int != PIPE_BG as libc::c_int
          && (*ptr_to_globals).saved_tty_pgrp != 0
        {
          /* we have ctty */
          /* We do it in *every* child, not just first,
           * to avoid races */
          tcsetpgrp((*ptr_to_globals).interactive_fd, pgrp);
        }
      }
      if (*pi).alive_cmds == 0i32 && (*pi).followup as libc::c_int == PIPE_BG as libc::c_int {
        /* 1st cmd in backgrounded pipe
         * should have its stdin /dev/null'ed */
        close(0i32);
        if open(b"/dev/null\x00" as *const u8 as *const libc::c_char, 0i32) != 0 {
          xopen(b"/\x00" as *const u8 as *const libc::c_char, 0i32);
        }
      } else {
        xmove_fd(next_infd, 0i32);
      }
      xmove_fd(pipefds.wr, 1i32);
      if pipefds.rd > 1i32 {
        close(pipefds.rd);
      }
      if setup_redirects(command, 0 as *mut *mut squirrel) != 0 {
        /* Like bash, explicit redirects override pipes,
         * and the pipe fd (fd#1) is available for dup'ing:
         * "cmd1 2>&1 | cmd2": fd#1 is duped to fd#2, thus stderr
         * of cmd1 goes into pipe.
         */
        /* Happens when redir file can't be opened:
         * $ hush -c 'echo FOO >&2 | echo BAR 3>/qwe/rty; echo BAZ'
         * FOO
         * hush: can't open '/qwe/rty': No such file or directory
         * BAZ
         * (echo BAR is not executed, it hits _exit(1) below)
         */
        _exit(1i32);
      }
      pseudo_exec(command, argv_expanded);
    }
    /* Stores to nommu_save list of env vars putenv'ed
     * (NOMMU, on MMU we don't need that) */
    /* cast away volatility... */
    /* parent or error */
    die_func = ::std::mem::transmute::<
      Option<unsafe extern "C" fn() -> !>,
      Option<unsafe extern "C" fn() -> ()>,
    >(Some(
      restore_ttypgrp_and__exit as unsafe extern "C" fn() -> !,
    ));
    free(argv_expanded as *mut libc::c_void);
    argv_expanded = 0 as *mut *mut libc::c_char;
    if (*command).pid < 0i32 {
      /* [v]fork failed */
      /* Clearly indicate, was it fork or vfork */
      bb_simple_perror_msg(if 1i32 != 0 {
        (b"vfork\x00" as *const u8 as *const libc::c_char).offset(1)
      } else {
        b"vfork\x00" as *const u8 as *const libc::c_char
      });
    } else {
      (*pi).alive_cmds += 1;
      /* Second and next children need to know pid of first one */
      if (*pi).pgrp < 0i32 {
        (*pi).pgrp = (*command).pid
      }
    }
    if cmd_no > 1i32 {
      close(next_infd);
    }
    if cmd_no < (*pi).num_cmds {
      close(pipefds.wr);
    }
    /* Pass read (output) pipe end to next iteration */
    next_infd = pipefds.rd
  }
  if (*pi).alive_cmds == 0 {
    return 1i32;
  }
  return -1i32;
}
/* NB: called by pseudo_exec, and therefore must not modify any
 * global data until exec/_exit (we can be a child after vfork!) */
unsafe extern "C" fn run_list(mut pi: *mut pipe) -> libc::c_int {
  let mut current_block: u64; /* RES_foo */
  let mut case_word: *mut libc::c_char = 0 as *mut libc::c_char; /* ditto */
  let mut loop_top: *mut pipe = 0 as *mut pipe;
  let mut for_lcur: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  let mut for_list: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  let mut last_followup: smallint = 0;
  let mut rcode: smalluint = 0;
  let mut cond_code: smalluint = 0i32 as smalluint;
  let mut rword: smallint = 0;
  let mut last_rword: smallint = 0;
  /* Check syntax for "for" */
  let mut cpipe: *mut pipe = 0 as *mut pipe;
  cpipe = pi;
  while !cpipe.is_null() {
    if !((*cpipe).res_word as libc::c_int != RES_FOR as libc::c_int
      && (*cpipe).res_word as libc::c_int != RES_IN as libc::c_int)
    {
      /* current word is FOR or IN (BOLD in comments below) */
      if (*cpipe).next.is_null() {
        syntax_error(b"malformed for\x00" as *const u8 as *const libc::c_char);
        return 1i32;
      }
      /* "FOR v; do ..." and "for v IN a b; do..." are ok */
      if !((*(*cpipe).next).res_word as libc::c_int == RES_DO as libc::c_int) {
        /* next word is not "do". It must be "in" then ("FOR v in ...") */
        if (*cpipe).res_word as libc::c_int == RES_IN as libc::c_int
                       ||
                       (*(*cpipe).next).res_word as libc::c_int !=
                           RES_IN as libc::c_int {
                    /* FOR v not_do_and_not_in..."? */
                    syntax_error(b"malformed for\x00" as *const u8 as
                                     *const libc::c_char);
                    return 1i32
                }
      }
    }
    cpipe = (*cpipe).next
  }
  /* Past this point, all code paths should jump to ret: label
   * in order to return, no direct "return" statements please.
   * This helps to ensure that no memory is leaked. */
  (*ptr_to_globals).run_list_level += 1;
  rword = RES_NONE as libc::c_int as smallint;
  last_rword = RES_XXXX as libc::c_int as smallint;
  last_followup = PIPE_SEQ as libc::c_int as smallint;
  rcode = (*ptr_to_globals).last_exitcode;
  /* Go through list of pipes, (maybe) executing them. */
  while !pi.is_null() {
    let mut r: libc::c_int = 0; /* for (pi) */
    let mut sv_errexit_depth: libc::c_int = 0;
    if (*ptr_to_globals).flag_SIGINT != 0 {
      break;
    }
    if (*ptr_to_globals).flag_return_in_progress as libc::c_int == 1i32 {
      break;
    }
    rword = (*pi).res_word;
    sv_errexit_depth = (*ptr_to_globals).errexit_depth;
    if rword as libc::c_int == RES_IF as libc::c_int
      || rword as libc::c_int == RES_ELIF as libc::c_int
      || (*pi).followup as libc::c_int != PIPE_SEQ as libc::c_int
    {
      (*ptr_to_globals).errexit_depth += 1
    }
    if (rword as libc::c_int == RES_WHILE as libc::c_int
      || rword as libc::c_int == RES_UNTIL as libc::c_int
      || rword as libc::c_int == RES_FOR as libc::c_int)
      && loop_top.is_null()
    {
      /* avoid bumping G.depth_of_loop twice */
      /* start of a loop: remember where loop starts */
      loop_top = pi;
      (*ptr_to_globals).depth_of_loop = (*ptr_to_globals).depth_of_loop.wrapping_add(1)
    }
    /* Still in the same "if...", "then..." or "do..." branch? */
    if rword as libc::c_int == last_rword as libc::c_int && 1i32 != 0 {
      if rcode as libc::c_int == 0i32 && last_followup as libc::c_int == PIPE_OR as libc::c_int
        || rcode as libc::c_int != 0i32 && last_followup as libc::c_int == PIPE_AND as libc::c_int
      {
        /* It is "<true> || CMD" or "<false> && CMD"
         * and we should not execute CMD */
        last_followup = (*pi).followup;
        current_block = 10720305954121010852;
      } else {
        current_block = 17747245473264231573;
      }
    } else {
      current_block = 17747245473264231573;
    }
    match current_block {
      17747245473264231573 => {
        last_followup = (*pi).followup;
        last_rword = rword;
        if cond_code != 0 {
          if rword as libc::c_int == RES_THEN as libc::c_int {
            /* if false; then ... fi has exitcode 0! */
            rcode = 0i32 as smalluint;
            (*ptr_to_globals).last_exitcode = rcode;
            /* "if <false> THEN cmd": skip cmd */
            current_block = 18377268871191777778;
          } else {
            current_block = 2706659501864706830;
          }
        } else if rword as libc::c_int == RES_ELSE as libc::c_int
          || rword as libc::c_int == RES_ELIF as libc::c_int
        {
          /* "if <true> then ... ELSE/ELIF cmd":
           * skip cmd and all following ones */
          break;
        } else {
          current_block = 2706659501864706830;
        }
        match current_block {
          18377268871191777778 => {}
          _ => {
            if rword as libc::c_int == RES_FOR as libc::c_int {
              /* && pi->num_cmds - always == 1 */
              if for_lcur.is_null() {
                /* first loop through for */
                static mut encoded_dollar_at: [libc::c_char; 4] = [
                  3i32 as libc::c_char,
                  ('@' as i32 | 0x80i32) as libc::c_char,
                  3i32 as libc::c_char,
                  '\u{0}' as i32 as libc::c_char,
                ]; /* encoded representation of "$@" */
                static mut encoded_dollar_at_argv: [*const libc::c_char; 2] =
                  unsafe { [encoded_dollar_at.as_ptr(), 0 as *const libc::c_char] }; /* argv list with one element: "$@" */
                let mut vals: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char; /* else: "for var; do..." -> assume "$@" list */
                rcode = 0i32 as smalluint;
                (*ptr_to_globals).last_exitcode = rcode;
                vals = encoded_dollar_at_argv.as_ptr() as *mut *mut libc::c_char;
                if (*(*pi).next).res_word as libc::c_int == RES_IN as libc::c_int {
                  /* if no variable values after "in" we skip "for" */
                  if (*(*(*pi).next).cmds.offset(0)).argv.is_null() {
                    break;
                  }
                  vals = (*(*(*pi).next).cmds.offset(0)).argv
                }
                /* create list of variable values */
                for_list = expand_strvec_to_strvec(vals);
                for_lcur = for_list
              }
              if (*for_lcur).is_null() {
                /* "for" loop is over, clean up */
                free(for_list as *mut libc::c_void);
                for_list = 0 as *mut *mut libc::c_char;
                for_lcur = 0 as *mut *mut libc::c_char;
                break;
              } else {
                /* Insert next value from for_lcur */
                /* note: *for_lcur already has quotes removed, $var expanded, etc */
                let fresh40 = for_lcur;
                for_lcur = for_lcur.offset(1);
                set_local_var(
                  xasprintf(
                    b"%s=%s\x00" as *const u8 as *const libc::c_char,
                    *(*(*pi).cmds.offset(0)).argv.offset(0),
                    *fresh40,
                  ),
                  0i32 as libc::c_uint,
                );
              }
              current_block = 18377268871191777778;
            } else if rword as libc::c_int == RES_IN as libc::c_int {
              current_block = 18377268871191777778;
            /* "for v IN list;..." - "in" has no cmds anyway */
            } else if rword as libc::c_int == RES_DONE as libc::c_int {
              current_block = 18377268871191777778;
            /* "done" has no cmds too */
            } else if rword as libc::c_int == RES_CASE as libc::c_int {
              case_word = expand_string_to_string(
                *(*(*pi).cmds).argv.offset(0),
                EXP_FLAG_ESC_GLOB_CHARS as libc::c_int,
                1i32,
              );
              current_block = 18377268871191777778;
            } else if rword as libc::c_int == RES_MATCH as libc::c_int {
              let mut argv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
              if case_word.is_null() {
                break;
              }
              /* all prev words didn't match, does this one match? */
              argv = (*(*pi).cmds).argv;
              while !(*argv).is_null() {
                let mut pattern: *mut libc::c_char = 0 as *mut libc::c_char;
                pattern =
                  expand_string_to_string(*argv, EXP_FLAG_ESC_GLOB_CHARS as libc::c_int, 0i32);
                /* TODO: which FNM_xxx flags to use? */
                cond_code = (fnmatch(pattern, case_word, 0i32) != 0i32) as libc::c_int as smalluint;
                free(pattern as *mut libc::c_void);
                if cond_code as libc::c_int == 0i32 {
                  /* match! we will execute this branch */
                  free(case_word as *mut libc::c_void); /* make future "word)" stop */
                  case_word = 0 as *mut libc::c_char;
                  break;
                } else {
                  argv = argv.offset(1)
                }
              }
              current_block = 18377268871191777778;
            } else {
              if rword as libc::c_int == RES_CASE_BODY as libc::c_int {
                /* inside of a case branch */
                if cond_code as libc::c_int != 0i32 {
                  current_block = 18377268871191777778;
                } else {
                  current_block = 8533724845731836612;
                }
              /* not matched yet, skip this pipe */
              } else {
                current_block = 8533724845731836612;
              }
              match current_block {
                18377268871191777778 => {}
                _ => {
                  if rword as libc::c_int == RES_ESAC as libc::c_int {
                    if !case_word.is_null() {
                      /* "case" did not match anything: still set $? (to 0) */
                      rcode = 0i32 as smalluint;
                      (*ptr_to_globals).last_exitcode = rcode
                    }
                  }
                  /* Just pressing <enter> in shell should check for jobs.
                   * OTOH, in non-interactive shell this is useless
                   * and only leads to extra job checks */
                  if (*pi).num_cmds == 0i32 {
                    if (*ptr_to_globals).interactive_fd != 0 {
                      current_block = 14155031687646128401;
                    } else {
                      current_block = 18377268871191777778;
                    }
                  } else {
                    /* After analyzing all keywords and conditions, we decided
                     * to execute this pipe. NB: have to do checkjobs(NULL)
                     * after run_pipe to collect any background children,
                     * even if list execution is to be stopped. */
                    (*ptr_to_globals).flag_break_continue = 0i32 as smallint; /* NB: rcode is a smalluint, r is int */
                    r = run_pipe(pi);
                    rcode = r as smalluint;
                    if r != -1i32 {
                      /* We ran a builtin, function, or group.
                       * rcode is already known
                       * and we don't need to wait for anything. */
                      (*ptr_to_globals).last_exitcode = rcode;
                      check_and_run_traps();
                      /* Was it "break" or "continue"? */
                      if (*ptr_to_globals).flag_break_continue != 0 {
                        let mut fbc: smallint = (*ptr_to_globals).flag_break_continue;
                        /* We might fall into outer *loop*,
                         * don't want to break it too */
                        if !loop_top.is_null() {
                          (*ptr_to_globals).depth_break_continue =
                            (*ptr_to_globals).depth_break_continue.wrapping_sub(1);
                          if (*ptr_to_globals).depth_break_continue == 0i32 as libc::c_uint {
                            (*ptr_to_globals).flag_break_continue = 0i32 as smallint
                          } /* else: "while... do... { we are here (innermost list is not a loop!) };...done" */
                          /* else: e.g. "continue 2" should *break* once, *then* continue */
                        }
                        if (*ptr_to_globals).depth_break_continue != 0i32 as libc::c_uint
                          || fbc as libc::c_int == BC_BREAK as libc::c_int
                        {
                          checkjobs(0 as *mut pipe, 0i32);
                          break;
                        } else {
                          /* "continue": simulate end of loop */
                          rword = RES_DONE as libc::c_int as smallint
                        }
                        current_block = 18377268871191777778;
                      } else if (*ptr_to_globals).flag_return_in_progress as libc::c_int == 1i32 {
                        checkjobs(0 as *mut pipe, 0i32);
                        break;
                      } else {
                        current_block = 13479157322803929894;
                      }
                    } else {
                      if (*pi).followup as libc::c_int == PIPE_BG as libc::c_int {
                        /* What does bash do with attempts to background builtins? */
                        /* even bash 3.2 doesn't do that well with nested bg:
                         * try "{ { sleep 10; echo DEEP; } & echo HERE; } &".
                         * I'm NOT treating inner &'s as jobs */
                        if (*ptr_to_globals).run_list_level == 1i32 {
                          insert_job_into_table(pi);
                        }
                        /* Last command's pid goes to $! */
                        (*ptr_to_globals).last_bg_pid =
                          (*(*pi).cmds.offset(((*pi).num_cmds - 1i32) as isize)).pid;
                        (*ptr_to_globals).last_bg_pid_exitcode = 0i32 as smalluint;
                        /* Check pi->pi_inverted? "! sleep 1 & echo $?": bash says 1. dash and ash say 0 */
                        rcode = 0i32 as smalluint
                      } else if (*ptr_to_globals).run_list_level == 1i32
                        && (*ptr_to_globals).interactive_fd != 0
                      {
                        /* Waits for completion, then fg's main shell */
                        rcode = checkjobs_and_fg_shell(pi) as smalluint
                      } else {
                        /* This one just waits for completion */
                        rcode = checkjobs(pi, 0i32) as smalluint
                      }
                      (*ptr_to_globals).last_exitcode = rcode;
                      check_and_run_traps();
                      current_block = 13479157322803929894;
                    }
                    match current_block {
                      18377268871191777778 => {}
                      _ => {
                        /* Handle "set -e" */
                        if rcode as libc::c_int != 0i32
                          && (*ptr_to_globals).o_opt[OPT_O_ERREXIT as libc::c_int as usize]
                            as libc::c_int
                            != 0
                        {
                          if (*ptr_to_globals).errexit_depth == 0i32 {
                            hush_exit(rcode as libc::c_int);
                          }
                        }
                        (*ptr_to_globals).errexit_depth = sv_errexit_depth;
                        /* Analyze how result affects subsequent commands */
                        if rword as libc::c_int == RES_IF as libc::c_int
                          || rword as libc::c_int == RES_ELIF as libc::c_int
                        {
                          cond_code = rcode
                        }
                        current_block = 14155031687646128401;
                      }
                    }
                  }
                  match current_block {
                    18377268871191777778 => {}
                    _ => {
                      checkjobs(0 as *mut pipe, 0i32);
                      current_block = 10720305954121010852;
                    }
                  }
                }
              }
            }
          }
        }
      }
      _ => {}
    }
    match current_block {
      10720305954121010852 =>
      /* Beware of "while false; true; do ..."! */
      {
        if !(*pi).next.is_null()
          && ((*(*pi).next).res_word as libc::c_int == RES_DO as libc::c_int
            || (*(*pi).next).res_word as libc::c_int == RES_DONE as libc::c_int)
        {
          /* check for RES_DONE is needed for "while ...; do \n done" case */
          if rword as libc::c_int == RES_WHILE as libc::c_int {
            if rcode != 0 {
              /* "while false; do...done" - exitcode 0 */
              rcode = 0i32 as smalluint;
              (*ptr_to_globals).last_exitcode = rcode;
              break;
            }
          }
          if rword as libc::c_int == RES_UNTIL as libc::c_int {
            if rcode == 0 {
              break;
            }
          }
        }
      }
      _ => {}
    }
    //unbackslash(case_word);
    //debug_printf_exec("CASE word2:'%s'\n", case_word);
    pi = if rword as libc::c_int == RES_DONE as libc::c_int {
      loop_top
    } else {
      (*pi).next
    }
  }
  /* "case ... matched_word) ... WORD)": we executed selected branch, stop */
  (*ptr_to_globals).run_list_level -= 1;
  if !loop_top.is_null() {
    (*ptr_to_globals).depth_of_loop = (*ptr_to_globals).depth_of_loop.wrapping_sub(1)
  }
  free(for_list as *mut libc::c_void);
  free(case_word as *mut libc::c_void);
  return rcode as libc::c_int;
}
/* !BB_MMU */
/* !BB_MMU */
/* Select which version we will use */
unsafe extern "C" fn run_and_free_list(mut pi: *mut pipe) -> libc::c_int {
  let mut rcode: libc::c_int = 0i32;
  if (*ptr_to_globals).o_opt[OPT_O_NOEXEC as libc::c_int as usize] == 0 {
    rcode = run_list(pi)
  }
  /* free_pipe_list has the side effect of clearing memory.
   * In the long run that function can be merged with run_list,
   * but doing that now would hobble the debugging effort. */
  free_pipe_list(pi);
  return rcode;
}
unsafe extern "C" fn install_sighandlers(mut mask: libc::c_uint) {
  let mut old_handler: sighandler_t = None;
  let mut sig: libc::c_uint = 0i32 as libc::c_uint;
  loop {
    mask >>= 1i32;
    if !(mask != 0i32 as libc::c_uint) {
      break;
    }
    sig = sig.wrapping_add(1);
    if mask & 1i32 as libc::c_uint == 0 {
      continue;
    }
    old_handler = install_sighandler(sig as libc::c_int, pick_sighandler(sig));
    /* POSIX allows shell to re-enable SIGCHLD
     * even if it was SIG_IGN on entry.
     * Therefore we skip IGN check for it:
     */
    if sig == 17i32 as libc::c_uint {
      continue;
    }
    /* bash re-enables SIGHUP which is SIG_IGNed on entry.
     * Try: "trap '' HUP; bash; echo RET" and type "kill -HUP $$"
     */
    //if (sig == SIGHUP) continue; - TODO?
    if old_handler
      == ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(1i32 as libc::intptr_t)
    {
      /* oops... restore back to IGN, and record this fact */
      install_sighandler(sig as libc::c_int, old_handler);
      if (*ptr_to_globals).traps.is_null() {
        (*ptr_to_globals).traps = xzalloc(
          (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul((64i32 + 1i32) as libc::c_ulong),
        ) as *mut *mut libc::c_char
      }
      free(*(*ptr_to_globals).traps.offset(sig as isize) as *mut libc::c_void);
      let ref mut fresh41 = *(*ptr_to_globals).traps.offset(sig as isize);
      *fresh41 = xzalloc(1i32 as size_t) as *mut libc::c_char
      /* == xstrdup(""); */
    }
  }
}
/* Called a few times only (or even once if "sh -c") */
unsafe extern "C" fn install_special_sighandlers() {
  let mut mask: libc::c_uint = 0;
  /* Which signals are shell-special? */
  mask = (1i32 << 3i32 | 1i32 << 17i32) as libc::c_uint;
  if (*ptr_to_globals).interactive_fd != 0 {
    mask |= SPECIAL_INTERACTIVE_SIGS as libc::c_int as libc::c_uint;
    if (*ptr_to_globals).saved_tty_pgrp != 0 {
      /* we have ctty, job control sigs work */
      mask |= SPECIAL_JOBSTOP_SIGS as libc::c_int as libc::c_uint
    }
  }
  /* Careful, do not re-install handlers we already installed */
  if (*ptr_to_globals).special_sig_mask != mask {
    let mut diff: libc::c_uint = mask & !(*ptr_to_globals).special_sig_mask;
    (*ptr_to_globals).special_sig_mask = mask;
    install_sighandlers(diff);
  };
}
/* helper */
/* Set handlers to restore tty pgrp and exit */
unsafe extern "C" fn install_fatal_sighandlers() {
  let mut mask: libc::c_uint = 0;
  /* We will restore tty pgrp on these signals */
  mask = (0i32
    + (1i32 << 7i32) * 1i32
    + (1i32 << 11i32) * 1i32
    + (1i32 << 6i32)
    + (1i32 << 13i32)
    + (1i32 << 14i32)) as libc::c_uint;
  /* if we are interactive, SIGHUP, SIGTERM and SIGINT are special sigs.
   * if we aren't interactive... but in this case
   * we never want to restore pgrp on exit, and this fn is not called
   */
  /*+ (1 << SIGHUP )*/
  /*+ (1 << SIGTERM)*/
  /*+ (1 << SIGINT )*/
  (*ptr_to_globals).fatal_sig_mask = mask;
  install_sighandlers(mask);
}
unsafe extern "C" fn set_mode(
  mut state: libc::c_int,
  mut mode: libc::c_char,
  mut o_opt: *const libc::c_char,
) -> libc::c_int {
  let mut idx: libc::c_int = 0;
  let mut current_block_13: u64;
  match mode as libc::c_int {
    110 => {
      (*ptr_to_globals).o_opt[OPT_O_NOEXEC as libc::c_int as usize] = state as libc::c_char;
      current_block_13 = 11584701595673473500;
    }
    120 => {
      (*ptr_to_globals).o_opt[OPT_O_XTRACE as libc::c_int as usize] = state as libc::c_char;
      if (*ptr_to_globals).x_mode_fd <= 0i32 {
        (*ptr_to_globals).x_mode_fd = dup_CLOEXEC(2i32, 10i32)
      }
      current_block_13 = 11584701595673473500;
    }
    101 => {
      (*ptr_to_globals).o_opt[OPT_O_ERREXIT as libc::c_int as usize] = state as libc::c_char;
      current_block_13 = 11584701595673473500;
    }
    111 => {
      if o_opt.is_null() {
        /* "set -o" or "set +o" without parameter.
         * in bash, set -o produces this output:
         *  pipefail        off
         * and set +o:
         *  set +o pipefail
         * We always use the second form.
         */
        let mut p: *const libc::c_char = o_opt_strings.as_ptr();
        idx = 0i32;
        while *p != 0 {
          printf(
            b"set %co %s\n\x00" as *const u8 as *const libc::c_char,
            if (*ptr_to_globals).o_opt[idx as usize] as libc::c_int != 0 {
              '-' as i32
            } else {
              '+' as i32
            },
            p,
          );
          idx += 1;
          p = p.offset(strlen(p).wrapping_add(1i32 as libc::c_ulong) as isize)
        }
        current_block_13 = 11584701595673473500;
      } else {
        idx = index_in_strings(o_opt_strings.as_ptr(), o_opt);
        if idx >= 0i32 {
          (*ptr_to_globals).o_opt[idx as usize] = state as libc::c_char;
          current_block_13 = 11584701595673473500;
        } else {
          current_block_13 = 17170860877845389804;
        }
      }
    }
    _ => {
      current_block_13 = 17170860877845389804;
    }
  }
  match current_block_13 {
    11584701595673473500 => {}
    _ =>
    /* fall through to error */
    {
      return 1i32
    }
  }
  return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn hush_main(
  mut argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut current_block: u64;
  let mut flags: libc::c_uint = 0;
  let mut builtin_argc: libc::c_uint = 0;
  let mut e: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  let mut cur_var: *mut variable = 0 as *mut variable;
  let mut shell_ver: *mut variable = 0 as *mut variable;
  let ref mut fresh42 =
    *(not_const_pp(&ptr_to_globals as *const *mut globals as *const libc::c_void)
      as *mut *mut globals);
  *fresh42 = xzalloc(::std::mem::size_of::<globals>() as libc::c_ulong) as *mut globals;
  asm!("" : : : "memory" : "volatile");
  sigfillset(&mut (*ptr_to_globals).sa.sa_mask);
  (*ptr_to_globals).sa.sa_flags = 0x10000000i32;
  if 0i32 != 0i32 {
    /* if EXIT_SUCCESS == 0, it is already done */
    (*ptr_to_globals).last_exitcode = 0i32 as smalluint
  }
  /* Deal with HUSH_VERSION */
  unsetenv(b"HUSH_VERSION\x00" as *const u8 as *const libc::c_char); /* in case it exists in initial env */
  shell_ver = xzalloc(::std::mem::size_of::<variable>() as libc::c_ulong) as *mut variable;
  (*shell_ver).flg_export = 1i32 as smallint;
  (*shell_ver).flg_read_only = 1i32 as smallint;
  /* Code which handles ${var<op>...} needs writable values for all variables,
   * therefore we xstrdup: */
  (*shell_ver).varstr = xstrdup(hush_version_str.as_ptr());
  /* Create shell local variables from the values
   * currently living in the environment */
  (*ptr_to_globals).top_var = shell_ver;
  cur_var = (*ptr_to_globals).top_var;
  e = environ;
  if !e.is_null() {
    while !(*e).is_null() {
      let mut value: *mut libc::c_char = strchr(*e, '=' as i32);
      if !value.is_null() {
        /* paranoia */
        (*cur_var).next =
          xzalloc(::std::mem::size_of::<variable>() as libc::c_ulong) as *mut variable;
        cur_var = (*cur_var).next;
        (*cur_var).varstr = *e;
        (*cur_var).max_len = strlen(*e) as libc::c_int;
        (*cur_var).flg_export = 1i32 as smallint
      }
      e = e.offset(1)
    }
  }
  /* (Re)insert HUSH_VERSION into env (AFTER we scanned the env!) */
  putenv((*shell_ver).varstr);
  /* Export PWD */
  set_pwd_var((1i32 << 0i32) as libc::c_uint);
  /* Set (but not export) HOSTNAME unless already set */
  if get_local_var_value(b"HOSTNAME\x00" as *const u8 as *const libc::c_char).is_null() {
    let mut uts: utsname = utsname {
      sysname: [0; 65],
      nodename: [0; 65],
      release: [0; 65],
      version: [0; 65],
      machine: [0; 65],
      domainname: [0; 65],
    };
    uname(&mut uts);
    set_local_var_from_halves(
      b"HOSTNAME\x00" as *const u8 as *const libc::c_char,
      uts.nodename.as_mut_ptr(),
    );
  }
  /* IFS is not inherited from the parent environment */
  set_local_var_from_halves(
    b"IFS\x00" as *const u8 as *const libc::c_char,
    defifsvar.as_ptr().offset(4),
  );
  if get_local_var_value(b"PATH\x00" as *const u8 as *const libc::c_char).is_null() {
    set_local_var_from_halves(
      b"PATH\x00" as *const u8 as *const libc::c_char,
      bb_PATH_root_path
        .as_ptr()
        .offset(::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as isize),
    );
  }
  /* PS1/PS2 are set later, if we determine that we are interactive */
  /* bash also exports SHLVL and _,
   * and sets (but doesn't export) the following variables:
   * BASH=/bin/bash
   * BASH_VERSINFO=([0]="3" [1]="2" [2]="0" [3]="1" [4]="release" [5]="i386-pc-linux-gnu")
   * BASH_VERSION='3.2.0(1)-release'
   * HOSTTYPE=i386
   * MACHTYPE=i386-pc-linux-gnu
   * OSTYPE=linux-gnu
   * PPID=<NNNNN> - we also do it elsewhere
   * EUID=<NNNNN>
   * UID=<NNNNN>
   * GROUPS=()
   * LINES=<NNN>
   * COLUMNS=<NNN>
   * BASH_ARGC=()
   * BASH_ARGV=()
   * BASH_LINENO=()
   * BASH_SOURCE=()
   * DIRSTACK=()
   * PIPESTATUS=([0]="0")
   * HISTFILE=/<xxx>/.bash_history
   * HISTFILESIZE=500
   * HISTSIZE=500
   * MAILCHECK=60
   * PATH=/usr/gnu/bin:/usr/local/bin:/bin:/usr/bin:.
   * SHELL=/bin/bash
   * SHELLOPTS=braceexpand:emacs:hashall:histexpand:history:interactive-comments:monitor
   * TERM=dumb
   * OPTERR=1
   * OPTIND=1
   * PS4='+ '
   */
  /* Initialize some more globals to non-zero values */
  die_func = ::std::mem::transmute::<
    Option<unsafe extern "C" fn() -> !>,
    Option<unsafe extern "C" fn() -> ()>,
  >(Some(
    restore_ttypgrp_and__exit as unsafe extern "C" fn() -> !,
  ));
  /* Shell is non-interactive at first. We need to call
   * install_special_sighandlers() if we are going to execute "sh <script>",
   * "sh -c <cmds>" or login shell's /etc/profile and friends.
   * If we later decide that we are interactive, we run install_special_sighandlers()
   * in order to intercept (more) signals.
   */
  /* Parse options */
  /* http://www.opengroup.org/onlinepubs/9699919799/utilities/sh.html */
  flags =
    if !(*argv.offset(0)).is_null() && *(*argv.offset(0)).offset(0) as libc::c_int == '-' as i32 {
      OPT_login as libc::c_int
    } else {
      0i32
    } as libc::c_uint; /* option parsing loop */
  builtin_argc = 0i32 as libc::c_uint;
  loop
  /* Well, we cannot just declare interactiveness,
   * we have to have some stuff (ctty, etc) */
  /* G_interactive_fd++; */
  {
    let mut opt: libc::c_int = getopt(
      argc,
      argv,
      b"+c:exinsl\x00" as *const u8 as *const libc::c_char,
    );
    if opt <= 0i32 {
      current_block = 10007731352114176167;
      break;
    }
    match opt {
      99 => {
        /* Possibilities:
         * sh ... -c 'script'
         * sh ... -c 'script' ARG0 [ARG1...]
         * On NOMMU, if builtin_argc != 0,
         * sh ... -c 'builtin' BARGV... "" ARG0 [ARG1...]
         * "" needs to be replaced with NULL
         * and BARGV vector fed to builtin function.
         * Note: the form without ARG0 never happens:
         * sh ... -c 'builtin' BARGV... ""
         */
        if (*ptr_to_globals).root_pid == 0 {
          (*ptr_to_globals).root_pid = getpid();
          (*ptr_to_globals).root_ppid = getppid()
        }
        (*ptr_to_globals).global_argv = argv.offset(optind as isize);
        (*ptr_to_globals).global_argc = argc - optind;
        if builtin_argc != 0 {
          /* -c 'builtin' [BARGV...] "" ARG0 [ARG1...] */
          let mut x: *const built_in_command = 0 as *const built_in_command;
          install_special_sighandlers();
          x = find_builtin(optarg);
          if !x.is_null() {
            /* paranoia */
            (*ptr_to_globals).global_argc = ((*ptr_to_globals).global_argc as libc::c_uint)
              .wrapping_sub(builtin_argc) as libc::c_int
              as libc::c_int; /* skip [BARGV...] "" */
            (*ptr_to_globals).global_argv =
              (*ptr_to_globals).global_argv.offset(builtin_argc as isize); /* replace "" */
            let ref mut fresh43 = *(*ptr_to_globals).global_argv.offset(-1i32 as isize); /* else -c 'script' ARG0 [ARG1...]: $0 is ARG0 */
            *fresh43 = 0 as *mut libc::c_char;
            fflush_all();
            (*ptr_to_globals).last_exitcode = (*x).b_function.expect("non-null function pointer")(
              argv.offset(optind as isize).offset(-1),
            ) as smalluint
          }
          current_block = 8111884305677119539;
          break;
        } else {
          (*ptr_to_globals).opt_c = 1i32 as libc::c_char;
          if (*(*ptr_to_globals).global_argv.offset(0)).is_null() {
            /* -c 'script' (no params): prevent empty $0 */
            (*ptr_to_globals).global_argv = (*ptr_to_globals).global_argv.offset(-1); /* points to argv[i] of 'script' */
            let ref mut fresh44 = *(*ptr_to_globals).global_argv.offset(0);
            *fresh44 = *argv.offset(0);
            (*ptr_to_globals).global_argc += 1
          }
          install_special_sighandlers();
          parse_and_run_string(optarg);
          current_block = 8111884305677119539;
          break;
        }
      }
      105 => {
        continue;
      }
      115 => {
        (*ptr_to_globals).opt_s = 1i32 as libc::c_char;
        continue;
      }
      108 => {
        flags |= OPT_login as libc::c_int as libc::c_uint;
        continue;
      }
      110 | 120 | 101 => {
        if set_mode(1i32, opt as libc::c_char, 0 as *const libc::c_char) == 0i32 {
          /* no error */
          continue;
        }
      }
      _ => {}
    }
    bb_show_usage();
  }
  match current_block {
    10007731352114176167 => {
      /* Skip options. Try "hush -l": $1 should not be "-l"! */
      (*ptr_to_globals).global_argc = argc - (optind - 1i32);
      (*ptr_to_globals).global_argv = argv.offset((optind - 1i32) as isize);
      let ref mut fresh45 = *(*ptr_to_globals).global_argv.offset(0);
      *fresh45 = *argv.offset(0);
      if (*ptr_to_globals).root_pid == 0 {
        (*ptr_to_globals).root_pid = getpid();
        (*ptr_to_globals).root_ppid = getppid()
      }
      /* If we are login shell... */
      if flags & OPT_login as libc::c_int as libc::c_uint != 0 {
        let mut input: *mut HFILE = 0 as *mut HFILE;
        input = hfopen(b"/etc/profile\x00" as *const u8 as *const libc::c_char);
        if !input.is_null() {
          install_special_sighandlers();
          parse_and_run_file(input);
          hfclose(input);
        }
        /* bash: after sourcing /etc/profile,
         * tries to source (in the given order):
         * ~/.bash_profile, ~/.bash_login, ~/.profile,
         * stopping on first found. --noprofile turns this off.
         * bash also sources ~/.bash_logout on exit.
         * If called as sh, skips .bash_XXX files.
         */
      }
      /* -s is: hush -s ARGV1 ARGV2 (no SCRIPT) */
      if (*ptr_to_globals).opt_s == 0 && !(*(*ptr_to_globals).global_argv.offset(1)).is_null() {
        let mut input_0: *mut HFILE = 0 as *mut HFILE;
        /*
         * "bash <script>" (which is never interactive (unless -i?))
         * sources $BASH_ENV here (without scanning $PATH).
         * If called as sh, does the same but with $ENV.
         * Also NB, per POSIX, $ENV should undergo parameter expansion.
         */
        (*ptr_to_globals).global_argc -= 1; /* for "hush /does/not/exist" case */
        (*ptr_to_globals).global_argv = (*ptr_to_globals).global_argv.offset(1);
        xfunc_error_retval = 127i32 as uint8_t;
        input_0 = hfopen(*(*ptr_to_globals).global_argv.offset(0));
        if input_0.is_null() {
          bb_simple_perror_msg_and_die(*(*ptr_to_globals).global_argv.offset(0));
        }
        xfunc_error_retval = 1i32 as uint8_t;
        install_special_sighandlers();
        parse_and_run_file(input_0);
      } else {
        /* "implicit" -s: bare interactive hush shows 's' in $- */
        (*ptr_to_globals).opt_s = 1i32 as libc::c_char;
        /* Up to here, shell was non-interactive. Now it may become one.
         * NB: don't forget to (re)run install_special_sighandlers() as needed.
         */
        /* A shell is interactive if the '-i' flag was given,
         * or if all of the following conditions are met:
         *    no -c command
         *    no arguments remaining or the -s flag given
         *    standard input is a terminal
         *    standard output is a terminal
         * Refer to Posix.2, the description of the 'sh' utility.
         */
        if isatty(0i32) != 0 && isatty(1i32) != 0 {
          (*ptr_to_globals).saved_tty_pgrp = tcgetpgrp(0i32);
          if (*ptr_to_globals).saved_tty_pgrp < 0i32 {
            (*ptr_to_globals).saved_tty_pgrp = 0i32
          }
          // TODO: track & disallow any attempts of user
          // to (inadvertently) close/redirect G_interactive_fd
          (*ptr_to_globals).interactive_fd = dup_CLOEXEC(0i32, 254i32);
          if (*ptr_to_globals).interactive_fd < 0i32 {
            /* try to dup stdin to high fd#, >= 255 */
            /* try to dup to any fd */
            (*ptr_to_globals).interactive_fd = dup(0i32);
            if (*ptr_to_globals).interactive_fd < 0i32 {
              /* give up */
              (*ptr_to_globals).interactive_fd = 0i32;
              (*ptr_to_globals).saved_tty_pgrp = 0i32
            }
          }
        }
        if (*ptr_to_globals).interactive_fd != 0 {
          close_on_exec_on((*ptr_to_globals).interactive_fd);
          if (*ptr_to_globals).saved_tty_pgrp != 0 {
            loop
            /* If we were run as 'hush &', sleep until we are
             * in the foreground (tty pgrp == our pgrp).
             * If we get started under a job aware app (like bash),
             * make sure we are now in charge so we don't fight over
             * who gets the foreground */
            {
              let mut shell_pgrp: pid_t = getpgrp();
              (*ptr_to_globals).saved_tty_pgrp = tcgetpgrp((*ptr_to_globals).interactive_fd);
              if (*ptr_to_globals).saved_tty_pgrp == shell_pgrp {
                break;
              }
              /* send TTIN to ourself (should stop us) */
              kill(-shell_pgrp, 21i32);
            }
          }
          /* Install more signal handlers */
          install_special_sighandlers();
          if (*ptr_to_globals).saved_tty_pgrp != 0 {
            /* Set other signals to restore saved_tty_pgrp */
            install_fatal_sighandlers();
            /* Put ourselves in our own process group
             * (bash, too, does this only if ctty is available) */
            setpgrp(); /* is the same as setpgid(our_pid, our_pid); */
            /* Grab control of the terminal */
            tcsetpgrp((*ptr_to_globals).interactive_fd, getpid());
          }
          die_func = ::std::mem::transmute::<
            Option<unsafe extern "C" fn() -> !>,
            Option<unsafe extern "C" fn() -> ()>,
          >(Some(
            restore_ttypgrp_and__exit as unsafe extern "C" fn() -> !,
          ));
          (*ptr_to_globals).line_input_state = new_line_input_t(FOR_SHELL as libc::c_int);
          let mut hp: *const libc::c_char =
            get_local_var_value(b"HISTFILE\x00" as *const u8 as *const libc::c_char);
          if hp.is_null() {
            hp = get_local_var_value(b"HOME\x00" as *const u8 as *const libc::c_char);
            if !hp.is_null() {
              hp = concat_path_file(hp, b".hush_history\x00" as *const u8 as *const libc::c_char)
            }
          } else {
            hp = xstrdup(hp)
          }
          if !hp.is_null() {
            (*(*ptr_to_globals).line_input_state).hist_file = hp
            //set_local_var(xasprintf("HISTFILE=%s", ...));
          }
          hp = get_local_var_value(b"HISTFILESIZE\x00" as *const u8 as *const libc::c_char);
          (*(*ptr_to_globals).line_input_state).max_history =
            size_from_HISTFILESIZE(hp) as libc::c_int
        } else {
          install_special_sighandlers();
        }
        /* bash:
         * if interactive but not a login shell, sources ~/.bashrc
         * (--norc turns this off, --rcfile <file> overrides)
         */
        if (*ptr_to_globals).interactive_fd != 0 {
          /* Set (but not export) PS1/2 unless already set */
          if get_local_var_value(b"PS1\x00" as *const u8 as *const libc::c_char).is_null() {
            set_local_var_from_halves(
              b"PS1\x00" as *const u8 as *const libc::c_char,
              b"\\w \\$ \x00" as *const u8 as *const libc::c_char,
            );
          }
          if get_local_var_value(b"PS2\x00" as *const u8 as *const libc::c_char).is_null() {
            set_local_var_from_halves(
              b"PS2\x00" as *const u8 as *const libc::c_char,
              b"> \x00" as *const u8 as *const libc::c_char,
            );
          }
          if 1i32 == 0 {
            /* note: ash and hush share this string */
            printf(
              b"\n\n%s %s\nEnter \'help\' for a list of built-in commands.\n\n\x00" as *const u8
                as *const libc::c_char,
              bb_banner.as_ptr(),
              b"hush - the humble shell\x00" as *const u8 as *const libc::c_char,
            ); /* stdin */
          }
        }
        parse_and_run_file(hfopen(0 as *const libc::c_char));
      }
    }
    _ => {}
  }
  hush_exit((*ptr_to_globals).last_exitcode as libc::c_int);
}
/*
 * Built-ins
 */
unsafe extern "C" fn builtin_true(mut _argv: *mut *mut libc::c_char) -> libc::c_int {
  return 0i32;
}
unsafe extern "C" fn run_applet_main(
  mut argv: *mut *mut libc::c_char,
  mut applet_main_func: Option<
    unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
  >,
) -> libc::c_int {
  let mut argc: libc::c_int = string_array_len(argv) as libc::c_int;
  return applet_main_func.expect("non-null function pointer")(argc, argv);
}
unsafe extern "C" fn builtin_test(mut argv: *mut *mut libc::c_char) -> libc::c_int {
  return run_applet_main(
    argv,
    Some(
      test_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
  );
}
unsafe extern "C" fn builtin_echo(mut argv: *mut *mut libc::c_char) -> libc::c_int {
  return run_applet_main(
    argv,
    Some(
      echo_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
  );
}
unsafe extern "C" fn builtin_printf(mut argv: *mut *mut libc::c_char) -> libc::c_int {
  return run_applet_main(
    argv,
    Some(
      printf_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
  );
}
unsafe extern "C" fn builtin_help(mut _argv: *mut *mut libc::c_char) -> libc::c_int {
  let mut x: *const built_in_command = 0 as *const built_in_command;
  printf(b"Built-in commands:\n------------------\n\x00" as *const u8 as *const libc::c_char);
  x = bltins1.as_ptr();
  while x
    != &*bltins1.as_ptr().offset(
      (::std::mem::size_of::<[built_in_command; 31]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<built_in_command>() as libc::c_ulong)
        as libc::c_uint as isize,
    ) as *const built_in_command
  {
    if !(*x).b_descr.is_null() {
      printf(
        b"%-10s%s\n\x00" as *const u8 as *const libc::c_char,
        (*x).b_cmd,
        (*x).b_descr,
      );
    }
    x = x.offset(1)
  }
  return 0i32;
}
unsafe extern "C" fn builtin_history(mut _argv: *mut *mut libc::c_char) -> libc::c_int {
  if !(*ptr_to_globals).line_input_state.is_null() {
    show_history((*ptr_to_globals).line_input_state);
  }
  return 0i32;
}
unsafe extern "C" fn skip_dash_dash(mut argv: *mut *mut libc::c_char) -> *mut *mut libc::c_char {
  argv = argv.offset(1);
  if !(*argv.offset(0)).is_null()
    && *(*argv.offset(0)).offset(0) as libc::c_int == '-' as i32
    && *(*argv.offset(0)).offset(1) as libc::c_int == '-' as i32
    && *(*argv.offset(0)).offset(2) as libc::c_int == '\u{0}' as i32
  {
    argv = argv.offset(1)
  }
  return argv;
}
/* Not #defining name to G.name - this quickly gets unwieldy
 * (too many defines). Also, I actually prefer to see when a variable
 * is global, thus "G." prefix is a useful hint */
/* memset(&G.sa, 0, sizeof(G.sa)); */
/* Function prototypes for builtins */
unsafe extern "C" fn builtin_cd(mut argv: *mut *mut libc::c_char) -> libc::c_int {
  let mut newdir: *const libc::c_char = 0 as *const libc::c_char;
  argv = skip_dash_dash(argv);
  newdir = *argv.offset(0);
  if newdir.is_null() {
    /* bash does nothing (exitcode 0) if HOME is ""; if it's unset,
     * bash says "bash: cd: HOME not set" and does nothing
     * (exitcode 1)
     */
    let mut home: *const libc::c_char =
      get_local_var_value(b"HOME\x00" as *const u8 as *const libc::c_char);
    newdir = if !home.is_null() {
      home
    } else {
      b"/\x00" as *const u8 as *const libc::c_char
    }
  }
  if chdir(newdir) != 0 {
    /* Mimic bash message exactly */
    bb_perror_msg(b"cd: %s\x00" as *const u8 as *const libc::c_char, newdir);
    return 1i32;
  }
  /* Read current dir (get_cwd(1) is inside) and set PWD.
   * Note: do not enforce exporting. If PWD was unset or unexported,
   * set it again, but do not export. bash does the same.
   */
  set_pwd_var(0i32 as libc::c_uint);
  return 0i32;
}
unsafe extern "C" fn builtin_pwd(mut _argv: *mut *mut libc::c_char) -> libc::c_int {
  puts(get_cwd(0i32));
  return 0i32;
}
unsafe extern "C" fn builtin_eval(mut argv: *mut *mut libc::c_char) -> libc::c_int {
  argv = skip_dash_dash(argv);
  if (*argv.offset(0)).is_null() {
    return 0i32;
  }
  (*ptr_to_globals).x_mode_depth = (*ptr_to_globals).x_mode_depth.wrapping_add(1);
  //bb_error_msg("%s: ++x_mode_depth=%d", __func__, G.x_mode_depth);
  if (*argv.offset(1)).is_null() {
    /* bash:
     * eval "echo Hi; done" ("done" is syntax error):
     * "echo Hi" will not execute too.
     */
    parse_and_run_string(*argv.offset(0));
  } else {
    /* "The eval utility shall construct a command by
     * concatenating arguments together, separating
     * each with a <space> character."
     */
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_uint = 0i32 as libc::c_uint;
    let mut pp: *mut *mut libc::c_char = argv;
    loop {
      len = (len as libc::c_ulong).wrapping_add(strlen(*pp).wrapping_add(1i32 as libc::c_ulong))
        as libc::c_uint as libc::c_uint;
      pp = pp.offset(1);
      if (*pp).is_null() {
        break;
      }
    }
    p = xmalloc(len as size_t) as *mut libc::c_char;
    str = p;
    pp = argv;
    loop {
      p = stpcpy(p, *pp);
      pp = pp.offset(1);
      if (*pp).is_null() {
        break;
      }
      let fresh46 = p;
      p = p.offset(1);
      *fresh46 = ' ' as i32 as libc::c_char
    }
    parse_and_run_string(str);
    free(str as *mut libc::c_void);
  }
  (*ptr_to_globals).x_mode_depth = (*ptr_to_globals).x_mode_depth.wrapping_sub(1);
  //bb_error_msg("%s: --x_mode_depth=%d", __func__, G.x_mode_depth);
  return (*ptr_to_globals).last_exitcode as libc::c_int; /* bash does this */
}
unsafe extern "C" fn builtin_exec(mut argv: *mut *mut libc::c_char) -> libc::c_int {
  argv = skip_dash_dash(argv);
  if (*argv.offset(0)).is_null() {
    return 0i32;
  }
  /* Careful: we can end up here after [v]fork. Do not restore
   * tty pgrp then, only top-level shell process does that */
  if (*ptr_to_globals).saved_tty_pgrp != 0 && getpid() == (*ptr_to_globals).root_pid {
    tcsetpgrp(
      (*ptr_to_globals).interactive_fd,
      (*ptr_to_globals).saved_tty_pgrp,
    );
  }
  /* Saved-redirect fds, script fds and G_interactive_fd are still
   * open here. However, they are all CLOEXEC, and execv below
   * closes them. Try interactive "exec ls -l /proc/self/fd",
   * it should show no extra open fds in the "ls" process.
   * If we'd try to run builtins/NOEXECs, this would need improving.
   */
  //close_saved_fds_and_FILE_fds();
  /* TODO: if exec fails, bash does NOT exit! We do.
   * We'll need to undo trap cleanup (it's inside execvp_or_die)
   * and tcsetpgrp, and this is inherently racy.
   */
  execvp_or_die(argv);
}
unsafe extern "C" fn builtin_exit(mut argv: *mut *mut libc::c_char) -> libc::c_int {
  /* interactive bash:
   * # trap "echo EEE" EXIT
   * # exit
   * exit
   * There are stopped jobs.
   * (if there are _stopped_ jobs, running ones don't count)
   * # exit
   * exit
   * EEE (then bash exits)
   *
   * TODO: we can use G.exiting = -1 as indicator "last cmd was exit"
   */
  /* note: EXIT trap is run by hush_exit */
  argv = skip_dash_dash(argv);
  if (*argv.offset(0)).is_null() {
    hush_exit((*ptr_to_globals).last_exitcode as libc::c_int);
  }
  /* mimic bash: exit 123abc == exit 255 + error msg */
  xfunc_error_retval = 255i32 as uint8_t;
  /* bash: exit -2 == exit 254, no error msg */
  hush_exit(xatoi(*argv.offset(0)) & 0xffi32);
}
/* http://www.opengroup.org/onlinepubs/9699919799/utilities/type.html */
unsafe extern "C" fn builtin_type(mut argv: *mut *mut libc::c_char) -> libc::c_int {
  let mut ret: libc::c_int = 0i32;
  loop {
    argv = argv.offset(1);
    if (*argv).is_null() {
      break;
    }
    let mut type_0: *const libc::c_char = 0 as *const libc::c_char;
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    /*else if (find_alias(*argv))
    type = "an alias";*/
    if !find_function(*argv).is_null() {
      type_0 = b"a function\x00" as *const u8 as *const libc::c_char
    } else if !find_builtin(*argv).is_null() {
      type_0 = b"a shell builtin\x00" as *const u8 as *const libc::c_char
    } else {
      path = find_in_path(*argv);
      if !path.is_null() {
        type_0 = path
      } else {
        bb_error_msg(
          b"type: %s: not found\x00" as *const u8 as *const libc::c_char,
          *argv,
        );
        ret = 1i32;
        continue;
      }
    }
    /* make conditional compile easier below */
    printf(
      b"%s is %s\n\x00" as *const u8 as *const libc::c_char,
      *argv,
      type_0,
    );
    free(path as *mut libc::c_void);
  }
  return ret;
}
/* Interruptibility of read builtin in bash
 * (tested on bash-4.2.8 by sending signals (not by ^C)):
 *
 * Empty trap makes read ignore corresponding signal, for any signal.
 *
 * SIGINT:
 * - terminates non-interactive shell;
 * - interrupts read in interactive shell;
 * if it has non-empty trap:
 * - executes trap and returns to command prompt in interactive shell;
 * - executes trap and returns to read in non-interactive shell;
 * SIGTERM:
 * - is ignored (does not interrupt) read in interactive shell;
 * - terminates non-interactive shell;
 * if it has non-empty trap:
 * - executes trap and returns to read;
 * SIGHUP:
 * - terminates shell (regardless of interactivity);
 * if it has non-empty trap:
 * - executes trap and returns to read;
 * SIGCHLD from children:
 * - does not interrupt read regardless of interactivity:
 *   try: sleep 1 & read x; echo $x
 */
unsafe extern "C" fn builtin_read(mut argv: *mut *mut libc::c_char) -> libc::c_int {
  let mut r: *const libc::c_char = 0 as *const libc::c_char;
  let mut params: builtin_read_params = builtin_read_params {
    read_flags: 0,
    setvar: None,
    argv: 0 as *mut *mut libc::c_char,
    ifs: 0 as *const libc::c_char,
    opt_n: 0 as *const libc::c_char,
    opt_p: 0 as *const libc::c_char,
    opt_t: 0 as *const libc::c_char,
    opt_u: 0 as *const libc::c_char,
    opt_d: 0 as *const libc::c_char,
  };
  memset(
    &mut params as *mut builtin_read_params as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<builtin_read_params>() as libc::c_ulong,
  );
  /* "!": do not abort on errors.
   * Option string must start with "sr" to match BUILTIN_READ_xxx
   */
  params.read_flags = getopt32(
    argv,
    b"!srn:p:t:u:d:\x00" as *const u8 as *const libc::c_char,
    &mut params.opt_n as *mut *const libc::c_char,
    &mut params.opt_p as *mut *const libc::c_char,
    &mut params.opt_t as *mut *const libc::c_char,
    &mut params.opt_u as *mut *const libc::c_char,
    &mut params.opt_d as *mut *const libc::c_char,
  ) as libc::c_int; /* can be NULL */
  if params.read_flags as uint32_t == -1i32 as uint32_t {
    return 1i32;
  }
  argv = argv.offset(optind as isize);
  params.argv = argv;
  params.setvar = Some(
    set_local_var_from_halves
      as unsafe extern "C" fn(_: *const libc::c_char, _: *const libc::c_char) -> (),
  );
  params.ifs = get_local_var_value(b"IFS\x00" as *const u8 as *const libc::c_char);
  loop {
    r = shell_builtin_read(&mut params);
    if !(r as uintptr_t == 1i32 as libc::c_ulong && *bb_errno == 4i32) {
      break;
    }
    let mut sig: libc::c_uint = check_and_run_traps() as libc::c_uint;
    if !(sig != 2i32 as libc::c_uint) {
      break;
    }
  }
  if r as uintptr_t > 1i32 as libc::c_ulong {
    bb_simple_error_msg(r);
    r = 1i32 as uintptr_t as *mut libc::c_char
  }
  return r as uintptr_t as libc::c_int;
}
unsafe extern "C" fn builtin_umask(mut argv: *mut *mut libc::c_char) -> libc::c_int {
  let mut rc: libc::c_int = 0;
  let mut mask: mode_t = 0;
  rc = 1i32;
  mask = umask(0i32 as __mode_t);
  argv = skip_dash_dash(argv);
  if !(*argv.offset(0)).is_null() {
    let mut old_mask: mode_t = mask;
    /* numeric umasks are taken as-is */
    /* symbolic umasks are inverted: "umask a=rx" calls umask(222) */
    if !((*(*argv.offset(0)).offset(0) as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int
      <= 9i32)
    {
      mask ^= 0o777i32 as libc::c_uint
    }
    mask = bb_parse_mode(*argv.offset(0), mask) as mode_t;
    if !((*(*argv.offset(0)).offset(0) as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int
      <= 9i32)
    {
      mask ^= 0o777i32 as libc::c_uint
    }
    if mask > 0o777i32 as libc::c_uint {
      mask = old_mask;
      /* bash messages:
       * bash: umask: 'q': invalid symbolic mode operator
       * bash: umask: 999: octal number out of range
       */
      bb_error_msg(
        b"%s: invalid mode \'%s\'\x00" as *const u8 as *const libc::c_char,
        b"umask\x00" as *const u8 as *const libc::c_char,
        *argv.offset(0),
      );
      rc = 0i32
    }
  } else {
    /* Mimic bash */
    printf(b"%04o\n\x00" as *const u8 as *const libc::c_char, mask);
    /* fall through and restore mask which we set to 0 */
  }
  umask(mask);
  return (rc == 0) as libc::c_int;
  /* rc != 0 - success */
}
unsafe extern "C" fn print_escaped(mut s: *const libc::c_char) {
  let mut p: *const libc::c_char = 0 as *const libc::c_char;
  let mut current_block: u64;
  if *s as libc::c_int == '\'' as i32 {
    current_block = 11958006367478214059;
  } else {
    current_block = 16559507199688588974;
  }
  loop {
    match current_block {
      16559507199688588974 => {
        p = strchrnul(s, '\'' as i32);
        /* print 'xxxx', possibly just '' */
        printf(
          b"\'%.*s\'\x00" as *const u8 as *const libc::c_char,
          p.wrapping_offset_from(s) as libc::c_long as libc::c_int,
          s,
        );
        if *p as libc::c_int == '\u{0}' as i32 {
          break;
        }
        s = p;
        current_block = 11958006367478214059;
      }
      _ => {
        /* s points to '; print "'''...'''" */
        putchar_unlocked('\"' as i32);
        loop {
          putchar_unlocked('\'' as i32);
          s = s.offset(1);
          if !(*s as libc::c_int == '\'' as i32) {
            break;
          }
        }
        putchar_unlocked('\"' as i32);
        if *s != 0 {
          current_block = 16559507199688588974;
        } else {
          break;
        }
      }
    }
  }
}
unsafe extern "C" fn helper_export_local(
  mut argv: *mut *mut libc::c_char,
  mut flags: libc::c_uint,
) -> libc::c_int {
  let mut current_block_24: u64;
  loop {
    let mut name: *mut libc::c_char = *argv;
    let mut name_end: *const libc::c_char = endofname(name);
    if *name_end as libc::c_int == '\u{0}' as i32 {
      let mut var: *mut variable = 0 as *mut variable;
      let mut vpp: *mut *mut variable = 0 as *mut *mut variable;
      vpp = get_ptr_to_local_var(
        name,
        name_end.wrapping_offset_from(name) as libc::c_long as libc::c_uint,
      );
      var = if !vpp.is_null() {
        *vpp
      } else {
        0 as *mut variable
      };
      if flags & (1i32 << 1i32) as libc::c_uint != 0 {
        /* export -n NAME (without =VALUE) */
        if !var.is_null() {
          (*var).flg_export = 0i32 as smallint; /* else: export -n NOT_EXISTING_VAR: no-op */
          unsetenv(name);
        }
        current_block_24 = 9188997750422900590;
      } else {
        if flags & (1i32 << 0i32) as libc::c_uint != 0 {
          /* export NAME (without =VALUE) */
          if !var.is_null() {
            (*var).flg_export = 1i32 as smallint;
            putenv((*var).varstr);
            current_block_24 = 9188997750422900590;
          } else {
            current_block_24 = 13797916685926291137;
          }
        } else {
          current_block_24 = 13797916685926291137;
        }
        match current_block_24 {
          9188997750422900590 => {}
          _ => {
            if flags & (1i32 << 2i32) as libc::c_uint != 0 {
              /* readonly NAME (without =VALUE) */
              if !var.is_null() {
                (*var).flg_read_only = 1i32 as smallint;
                current_block_24 = 9188997750422900590;
              } else {
                current_block_24 = 10043043949733653460;
              }
            } else {
              current_block_24 = 10043043949733653460;
            }
            match current_block_24 {
              9188997750422900590 => {}
              _ =>
              /* Is this "local" bltin? */
              {
                if flags & (1i32 << 0i32 | 1i32 << 1i32 | 1i32 << 2i32) as libc::c_uint == 0 {
                  let mut lvl: libc::c_uint = flags >> 3i32;
                  if !var.is_null() && (*var).var_nest_level as libc::c_uint == lvl {
                    current_block_24 = 9188997750422900590;
                  } else {
                    current_block_24 = 14648156034262866959;
                  }
                } else {
                  current_block_24 = 14648156034262866959;
                }
                match current_block_24 {
                  9188997750422900590 => {}
                  _ => {
                    /* Exporting non-existing variable.
                     * bash does not put it in environment,
                     * but remembers that it is exported,
                     * and does put it in env when it is set later.
                     * We just set it to "" and export.
                     */
                    /* Or, it's "local NAME" (without =VALUE).
                     * bash sets the value to "".
                     */
                    /* Or, it's "readonly NAME" (without =VALUE).
                     * bash remembers NAME and disallows its creation
                     * in the future.
                     */
                    name = xasprintf(b"%s=\x00" as *const u8 as *const libc::c_char, name);
                    current_block_24 = 15597372965620363352;
                  }
                }
              }
            }
          }
        }
      }
    } else {
      if *name_end as libc::c_int != '=' as i32 {
        bb_error_msg(
          b"\'%s\': bad variable name\x00" as *const u8 as *const libc::c_char,
          name,
        );
        /* do not parse following argv[]s: */
        return 1i32;
      }
      /* (Un)exporting/making local NAME=VALUE */
      name = xstrdup(name);
      /* Testcase: export PS1='\w \$ ' */
      unbackslash(name);
      current_block_24 = 15597372965620363352;
    }
    match current_block_24 {
      15597372965620363352 => {
        if set_local_var(name, flags) != 0 {
          return 1i32;
        }
      }
      _ => {}
    }
    /* "local x=abc; ...; local x" - ignore second local decl */
    argv = argv.offset(1);
    if (*argv).is_null() {
      break;
    }
  }
  return 0i32;
}
unsafe extern "C" fn builtin_export(mut argv: *mut *mut libc::c_char) -> libc::c_int {
  let mut opt_unexport: libc::c_uint = 0;
  /* "!": do not abort on errors */
  opt_unexport = getopt32(argv, b"!n\x00" as *const u8 as *const libc::c_char);
  if opt_unexport == -1i32 as uint32_t {
    return 1i32;
  }
  argv = argv.offset(optind as isize);
  if (*argv.offset(0)).is_null() {
    let mut e: *mut *mut libc::c_char = environ;
    if !e.is_null() {
      /* wtf? take next variable */
      while !(*e).is_null() {
        /* ash emits: export VAR='VAL'
         * bash: declare -x VAR="VAL"
         * we follow ash example */
        let fresh47 = e;
        e = e.offset(1);
        let mut s: *const libc::c_char = *fresh47;
        let mut p: *const libc::c_char = strchr(s, '=' as i32);
        if p.is_null() {
          continue;
        }
        /* export var= */
        printf(
          b"export %.*s\x00" as *const u8 as *const libc::c_char,
          p.wrapping_offset_from(s) as libc::c_long as libc::c_int + 1i32,
          s,
        );
        print_escaped(p.offset(1));
        putchar_unlocked('\n' as i32);
      }
      /*fflush_all(); - done after each builtin anyway */
    }
    return 0i32;
  }
  return helper_export_local(
    argv,
    if opt_unexport != 0 {
      (1i32) << 1i32
    } else {
      (1i32) << 0i32
    } as libc::c_uint,
  );
}
unsafe extern "C" fn builtin_local(mut argv: *mut *mut libc::c_char) -> libc::c_int {
  if (*ptr_to_globals).func_nest_level == 0i32 as libc::c_uint {
    bb_error_msg(
      b"%s: not in a function\x00" as *const u8 as *const libc::c_char,
      *argv.offset(0),
    );
    return 1i32;
    /* bash compat */
  }
  argv = argv.offset(1);
  /* Since all builtins run in a nested variable level,
   * need to use level - 1 here. Or else the variable will be removed at once
   * after builtin returns.
   */
  return helper_export_local(
    argv,
    (*ptr_to_globals)
      .var_nest_level
      .wrapping_sub(1i32 as libc::c_uint)
      << 3i32,
  );
}
unsafe extern "C" fn builtin_readonly(mut argv: *mut *mut libc::c_char) -> libc::c_int {
  argv = argv.offset(1);
  if (*argv).is_null() {
    /* bash: readonly [-p]: list all readonly VARs
     * (-p has no effect in bash)
     */
    let mut e: *mut variable = 0 as *mut variable;
    e = (*ptr_to_globals).top_var;
    while !e.is_null() {
      if (*e).flg_read_only != 0 {
        //TODO: quote value: readonly VAR='VAL'
        printf(
          b"readonly %s\n\x00" as *const u8 as *const libc::c_char,
          (*e).varstr,
        );
      }
      e = (*e).next
    }
    return 0i32;
  }
  return helper_export_local(argv, (1i32 << 2i32) as libc::c_uint);
}
/* http://www.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html#unset */
unsafe extern "C" fn builtin_unset(mut argv: *mut *mut libc::c_char) -> libc::c_int {
  let mut ret: libc::c_int = 0;
  let mut opts: libc::c_uint = 0;
  /* "!": do not abort on errors */
  /* "+": stop at 1st non-option */
  opts = getopt32(argv, b"!+vf\x00" as *const u8 as *const libc::c_char);
  if opts == -1i32 as libc::c_uint {
    return 1i32;
  }
  if opts == 3i32 as libc::c_uint {
    bb_simple_error_msg(b"unset: -v and -f are exclusive\x00" as *const u8 as *const libc::c_char);
    return 1i32;
  }
  argv = argv.offset(optind as isize);
  ret = 0i32;
  while !(*argv).is_null() {
    if opts & 2i32 as libc::c_uint == 0 {
      /* not -f */
      if unset_local_var(*argv) != 0 {
        /* unset <nonexistent_var> doesn't fail.
         * Error is when one tries to unset RO var.
         * Message was printed by unset_local_var. */
        ret = 1i32
      }
    } else {
      unset_func(*argv);
    }
    argv = argv.offset(1)
  }
  return ret;
}
/* http://www.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html#set
 * built-in 'set' handler
 * SUSv3 says:
 * set [-abCefhmnuvx] [-o option] [argument...]
 * set [+abCefhmnuvx] [+o option] [argument...]
 * set -- [argument...]
 * set -o
 * set +o
 * Implementations shall support the options in both their hyphen and
 * plus-sign forms. These options can also be specified as options to sh.
 * Examples:
 * Write out all variables and their values: set
 * Set $1, $2, and $3 and set "$#" to 3: set c a b
 * Turn on the -x and -v options: set -xv
 * Unset all positional parameters: set --
 * Set $1 to the value of x, even if it begins with '-' or '+': set -- "$x"
 * Set the positional parameters to the expansion of x, even if x expands
 * with a leading '-' or '+': set -- $x
 *
 * So far, we only support "set -- [argument...]" and some of the short names.
 */
unsafe extern "C" fn builtin_set(mut argv: *mut *mut libc::c_char) -> libc::c_int {
  let mut current_block: u64;
  let mut n: libc::c_int = 0;
  let mut pp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  let mut g_argv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  argv = argv.offset(1);
  let mut arg: *mut libc::c_char = *argv;
  if arg.is_null() {
    let mut e: *mut variable = 0 as *mut variable;
    e = (*ptr_to_globals).top_var;
    while !e.is_null() {
      puts((*e).varstr);
      e = (*e).next
    }
    return 0i32;
  }
  loop {
    if strcmp(arg, b"--\x00" as *const u8 as *const libc::c_char) == 0i32 {
      argv = argv.offset(1);
      current_block = 11206778511404137033;
      break;
    } else {
      if *arg.offset(0) as libc::c_int != '+' as i32 && *arg.offset(0) as libc::c_int != '-' as i32
      {
        current_block = 4495394744059808450;
        break;
      }
      n = 1i32;
      while *arg.offset(n as isize) != 0 {
        if set_mode(
          (*arg.offset(0) as libc::c_int == '-' as i32) as libc::c_int,
          *arg.offset(n as isize),
          *argv.offset(1),
        ) != 0
        {
          bb_error_msg(
            b"%s: %s: invalid option\x00" as *const u8 as *const libc::c_char,
            b"set\x00" as *const u8 as *const libc::c_char,
            arg,
          );
          return 1i32;
        }
        if *arg.offset(n as isize) as libc::c_int == 'o' as i32 && !(*argv.offset(1)).is_null() {
          argv = argv.offset(1)
        }
        n += 1
      }
      argv = argv.offset(1);
      arg = *argv;
      if arg.is_null() {
        current_block = 4495394744059808450;
        break;
      }
    }
  }
  match current_block {
    4495394744059808450 => {
      /* Now argv[0] is 1st argument */
      if arg.is_null() {
        return 0i32;
      }
    }
    _ => {}
  }
  /* NB: G.global_argv[0] ($0) is never freed/changed */
  g_argv = (*ptr_to_globals).global_argv; /* retain $0 */
  if (*ptr_to_globals).global_args_malloced != 0 {
    pp = g_argv;
    loop {
      pp = pp.offset(1);
      if (*pp).is_null() {
        break;
      }
      free(*pp as *mut libc::c_void);
    }
    let ref mut fresh48 = *g_argv.offset(1);
    *fresh48 = 0 as *mut libc::c_char
  } else {
    (*ptr_to_globals).global_args_malloced = 1i32 as smalluint;
    pp = xzalloc(
      (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
        .wrapping_mul(2i32 as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    let ref mut fresh49 = *pp.offset(0);
    *fresh49 = *g_argv.offset(0);
    g_argv = pp
  }
  /* This realloc's G.global_argv */
  pp = add_strings_to_strings(g_argv, argv, 1i32);
  (*ptr_to_globals).global_argv = pp;
  (*ptr_to_globals).global_argc =
    (1i32 as libc::c_uint).wrapping_add(string_array_len(pp.offset(1))) as libc::c_int;
  return 0i32;
}
unsafe extern "C" fn builtin_shift(mut argv: *mut *mut libc::c_char) -> libc::c_int {
  let mut n: libc::c_int = 1i32;
  argv = skip_dash_dash(argv);
  if !(*argv.offset(0)).is_null() {
    n = bb_strtou(*argv.offset(0), 0 as *mut *mut libc::c_char, 10i32) as libc::c_int;
    if *bb_errno != 0 || n < 0i32 {
      /* shared string with ash.c */
      bb_error_msg(
        b"Illegal number: %s\x00" as *const u8 as *const libc::c_char,
        *argv.offset(0),
      );
      /*
       * ash aborts in this case.
       * bash prints error message and set $? to 1.
       * Interestingly, for "shift 99999" bash does not
       * print error message, but does set $? to 1
       * (and does no shifting at all).
       */
    }
  }
  if n >= 0i32 && n < (*ptr_to_globals).global_argc {
    if (*ptr_to_globals).global_args_malloced != 0 {
      let mut m: libc::c_int = 1i32;
      while m <= n {
        let fresh50 = m;
        m = m + 1;
        free(*(*ptr_to_globals).global_argv.offset(fresh50 as isize) as *mut libc::c_void);
      }
    }
    (*ptr_to_globals).global_argc -= n;
    memmove(
      &mut *(*ptr_to_globals).global_argv.offset(1) as *mut *mut libc::c_char as *mut libc::c_void,
      &mut *(*ptr_to_globals).global_argv.offset((n + 1i32) as isize) as *mut *mut libc::c_char
        as *const libc::c_void,
      ((*ptr_to_globals).global_argc as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    );
    return 0i32;
  }
  return 1i32;
}
unsafe extern "C" fn builtin_getopts(mut argv: *mut *mut libc::c_char) -> libc::c_int {
  /* http://pubs.opengroup.org/onlinepubs/9699919799/utilities/getopts.html

  TODO:
  If a required argument is not found, and getopts is not silent,
  a question mark (?) is placed in VAR, OPTARG is unset, and a
  diagnostic message is printed.  If getopts is silent, then a
  colon (:) is placed in VAR and OPTARG is set to the option
  character found.

  Test that VAR is a valid variable name?

  "Whenever the shell is invoked, OPTIND shall be initialized to 1"
  */
  let mut cbuf: [libc::c_char; 2] = [0; 2]; /* for error messages in getopt() */
  let mut cp: *const libc::c_char = 0 as *const libc::c_char;
  let mut optstring: *const libc::c_char = 0 as *const libc::c_char;
  let mut var: *const libc::c_char = 0 as *const libc::c_char;
  let mut c: libc::c_int = 0;
  let mut n: libc::c_int = 0;
  let mut exitcode: libc::c_int = 0;
  let mut my_opterr: libc::c_int = 0;
  let mut count: libc::c_uint = 0;
  argv = argv.offset(1);
  optstring = *argv;
  if optstring.is_null() || {
    argv = argv.offset(1);
    var = *argv;
    var.is_null()
  } {
    bb_simple_error_msg(
      b"usage: getopts OPTSTRING VAR [ARGS]\x00" as *const u8 as *const libc::c_char,
    );
    return 1i32;
  }
  if !(*argv.offset(1)).is_null() {
    let ref mut fresh51 = *argv.offset(0);
    *fresh51 = *(*ptr_to_globals).global_argv.offset(0)
  } else {
    argv = (*ptr_to_globals).global_argv
  }
  cbuf[1] = '\u{0}' as i32 as libc::c_char;
  my_opterr = 0i32;
  if *optstring.offset(0) as libc::c_int != ':' as i32 {
    cp = get_local_var_value(b"OPTERR\x00" as *const u8 as *const libc::c_char);
    /* 0 if "OPTERR=0", 1 otherwise */
    my_opterr = (cp.is_null()
      || (*cp.offset(0) as libc::c_int != '0' as i32 || *cp.offset(1) as libc::c_int != 0))
      as libc::c_int
  }
  /* getopts stops on first non-option. Add "+" to force that */
  /*if (optstring[0] != '+')*/
  let mut fresh52 = ::std::vec::from_elem(
    0,
    strlen(optstring).wrapping_add(2i32 as libc::c_ulong) as usize,
  );
  let mut s: *mut libc::c_char = fresh52.as_mut_ptr() as *mut libc::c_char;
  sprintf(s, b"+%s\x00" as *const u8 as *const libc::c_char, optstring);
  optstring = s;
  /* Naively, now we should just
   *	cp = get_local_var_value("OPTIND");
   *	optind = cp ? atoi(cp) : 0;
   *	optarg = NULL;
   *	opterr = my_opterr;
   *	c = getopt(string_array_len(argv), argv, optstring);
   * and be done? Not so fast...
   * Unlike normal getopt() usage in C programs, here
   * each successive call will (usually) have the same argv[] CONTENTS,
   * but not the ADDRESSES. Worse yet, it's possible that between
   * invocations of "getopts", there will be calls to shell builtins
   * which use getopt() internally. Example:
   *	while getopts "abc" RES -a -bc -abc de; do
   *		unset -ff func
   *	done
   * This would not work correctly: getopt() call inside "unset"
   * modifies internal libc state which is tracking position in
   * multi-option strings ("-abc"). At best, it can skip options
   * or return the same option infinitely. With glibc implementation
   * of getopt(), it would use outright invalid pointers and return
   * garbage even _without_ "unset" mangling internal state.
   *
   * We resort to resetting getopt() state and calling it N times,
   * until we get Nth result (or failure).
   * (N == G.getopt_count is reset to 0 whenever OPTIND is [un]set).
   */
  optind = 0i32;
  count = 0i32 as libc::c_uint;
  n = string_array_len(argv) as libc::c_int;
  loop {
    optarg = 0 as *mut libc::c_char;
    opterr = if count < (*ptr_to_globals).getopt_count {
      0i32
    } else {
      my_opterr
    };
    c = getopt(n, argv, optstring);
    if c < 0i32 {
      break;
    }
    count = count.wrapping_add(1);
    if !(count <= (*ptr_to_globals).getopt_count) {
      break;
    }
  }
  /* Set OPTIND. Prevent resetting of the magic counter! */
  set_local_var_from_halves(
    b"OPTIND\x00" as *const u8 as *const libc::c_char,
    utoa(optind as libc::c_uint),
  ); /* "next time, give me N+1'th result" */
  (*ptr_to_globals).getopt_count = count; /* just in case */
  optind = 0i32;
  /* Set OPTARG */
  /* Always set or unset, never left as-is, even on exit/error:
   * "If no option was found, or if the option that was found
   * does not have an option-argument, OPTARG shall be unset."
   */
  cp = optarg;
  if c == '?' as i32 {
    /* If ":optstring" and unknown option is seen,
     * it is stored to OPTARG.
     */
    if *optstring.offset(1) as libc::c_int == ':' as i32 {
      cbuf[0] = optopt as libc::c_char;
      cp = cbuf.as_mut_ptr()
    }
  }
  if !cp.is_null() {
    set_local_var_from_halves(b"OPTARG\x00" as *const u8 as *const libc::c_char, cp);
  } else {
    unset_local_var(b"OPTARG\x00" as *const u8 as *const libc::c_char);
  }
  /* Convert -1 to "?" */
  exitcode = 0i32;
  if c < 0i32 {
    /* -1: end of options */
    exitcode = 1i32;
    c = '?' as i32
  }
  /* Set VAR */
  cbuf[0] = c as libc::c_char;
  set_local_var_from_halves(var, cbuf.as_mut_ptr());
  return exitcode;
}
unsafe extern "C" fn builtin_source(mut argv: *mut *mut libc::c_char) -> libc::c_int {
  let mut arg_path: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut input: *mut HFILE = 0 as *mut HFILE;
  let mut sv: save_arg_t = save_arg_t {
    sv_argv0: 0 as *mut libc::c_char,
    sv_g_argv: 0 as *mut *mut libc::c_char,
    sv_g_argc: 0,
    sv_g_malloced: 0,
  };
  let mut args_need_save: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut sv_flg: smallint = 0;
  argv = skip_dash_dash(argv);
  filename = *argv.offset(0);
  if filename.is_null() {
    /* bash says: "bash: .: filename argument required" */
    return 2i32;
    /* bash compat */
  }
  arg_path = 0 as *mut libc::c_char;
  if strchr(filename, '/' as i32).is_null() {
    arg_path = find_in_path(filename);
    if !arg_path.is_null() {
      filename = arg_path
    } else if 0i32 == 0 {
      *bb_errno = 2i32;
      bb_simple_perror_msg(filename);
      return 1i32;
    }
  }
  input = hfopen(filename);
  free(arg_path as *mut libc::c_void);
  if input.is_null() {
    bb_perror_msg(b"%s\x00" as *const u8 as *const libc::c_char, filename);
    /* POSIX: non-interactive shell should abort here,
     * not merely fail. So far no one complained :)
     */
    return 1i32;
  }
  sv_flg = (*ptr_to_globals).flag_return_in_progress;
  /* "we are inside sourced file, ok to use return" */
  (*ptr_to_globals).flag_return_in_progress = -1i32 as smallint; /* used as a boolean variable */
  args_need_save = *argv.offset(1);
  if !args_need_save.is_null() {
    save_and_replace_G_args(&mut sv, argv);
  }
  /* "false; . ./empty_line; echo Zero:$?" should print 0 */
  (*ptr_to_globals).last_exitcode = 0i32 as smalluint;
  parse_and_run_file(input);
  hfclose(input);
  if !args_need_save.is_null() {
    /* can't use argv[1] instead: "shift" can mangle it */
    restore_G_args(&mut sv, argv);
  }
  (*ptr_to_globals).flag_return_in_progress = sv_flg;
  return (*ptr_to_globals).last_exitcode as libc::c_int;
}
unsafe extern "C" fn builtin_trap(mut argv: *mut *mut libc::c_char) -> libc::c_int {
  let mut ret: libc::c_int = 0;
  let mut current_block: u64;
  let mut sig: libc::c_int = 0;
  let mut new_cmd: *mut libc::c_char = 0 as *mut libc::c_char;
  if (*ptr_to_globals).traps.is_null() {
    (*ptr_to_globals).traps = xzalloc(
      (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
        .wrapping_mul((64i32 + 1i32) as libc::c_ulong),
    ) as *mut *mut libc::c_char
  }
  argv = argv.offset(1);
  if (*argv).is_null() {
    let mut i: libc::c_int = 0;
    /* No args: print all trapped */
    i = 0i32;
    while i < 64i32 + 1i32 {
      if !(*(*ptr_to_globals).traps.offset(i as isize)).is_null() {
        printf(b"trap -- \x00" as *const u8 as *const libc::c_char);
        print_escaped(*(*ptr_to_globals).traps.offset(i as isize));
        /* note: bash adds "SIG", but only if invoked
         * as "bash". If called as "sh", or if set -o posix,
         * then it prints short signal names.
         * We are printing short names: */
        printf(
          b" %s\n\x00" as *const u8 as *const libc::c_char,
          get_signame(i),
        );
      }
      i += 1
    }
    /*fflush_all(); - done after each builtin anyway */
    return 0i32;
  }
  new_cmd = 0 as *mut libc::c_char;
  /* If first arg is a number: reset all specified signals */
  sig = bb_strtou(*argv, 0 as *mut *mut libc::c_char, 10i32) as libc::c_int;
  if *bb_errno == 0i32 {
    ret = 0;
  } else {
    if (*argv.offset(1)).is_null() {
      /* no second arg */
      bb_simple_error_msg(b"trap: invalid arguments\x00" as *const u8 as *const libc::c_char);
      return 1i32;
    }
    /* First arg is "-": reset all specified to default */
    /* First arg is "--": skip it, the rest is "handler SIGs..." */
    /* Everything else: set arg as signal handler
     * (includes "" case, which ignores signal) */
    if *(*argv.offset(0)).offset(0) as libc::c_int == '-' as i32 {
      if *(*argv.offset(0)).offset(1) as libc::c_int == '\u{0}' as i32 {
        current_block = 8082207545859869536;
      } else {
        if *(*argv.offset(0)).offset(1) as libc::c_int == '-' as i32
          && *(*argv.offset(0)).offset(2) as libc::c_int == '\u{0}' as i32
        {
          /* "--" */
          argv = argv.offset(1)
        }
        current_block = 3938820862080741272;
      }
    /* else: "-something", no special meaning */
    } else {
      current_block = 3938820862080741272;
    }
    match current_block {
      3938820862080741272 => new_cmd = *argv,
      _ => {}
    }
    /* "-" */
    /* new_cmd remains NULL: "reset these sigs" */
    argv = argv.offset(1)
  }
  ret = 0i32;
  while !(*argv).is_null() {
    let mut handler: sighandler_t = None;
    let fresh53 = argv;
    argv = argv.offset(1);
    sig = get_signum(*fresh53);
    if sig < 0i32 {
      ret = 1i32;
      /* Mimic bash message exactly */
      bb_error_msg(
        b"trap: %s: invalid signal specification\x00" as *const u8 as *const libc::c_char,
        *argv.offset(-1i32 as isize),
      );
    } else {
      free(*(*ptr_to_globals).traps.offset(sig as isize) as *mut libc::c_void);
      let ref mut fresh54 = *(*ptr_to_globals).traps.offset(sig as isize);
      *fresh54 = xstrdup(new_cmd);
      /* There is no signal for 0 (EXIT) */
      if sig == 0i32 {
        continue;
      }
      if !new_cmd.is_null() {
        handler = if *new_cmd.offset(0) as libc::c_int != 0 {
          Some(record_pending_signo as unsafe extern "C" fn(_: libc::c_int) -> ())
        } else {
          ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(1i32 as libc::intptr_t)
        }
      } else {
        /* We are removing trap handler */
        handler = pick_sighandler(sig as libc::c_uint)
      }
      install_sighandler(sig, handler);
    }
  }
  return ret;
}
unsafe extern "C" fn parse_jobspec(mut str: *const libc::c_char) -> *mut pipe {
  let mut pi: *mut pipe = 0 as *mut pipe;
  let mut jobnum: libc::c_uint = 0;
  if sscanf(
    str,
    b"%%%u\x00" as *const u8 as *const libc::c_char,
    &mut jobnum as *mut libc::c_uint,
  ) != 1i32
  {
    if *str.offset(0) as libc::c_int != '%' as i32
      || *str.offset(1) as libc::c_int != '%' as i32
        && *str.offset(1) as libc::c_int != '+' as i32
        && *str.offset(1) as libc::c_int != '\u{0}' as i32
    {
      bb_error_msg(
        b"bad argument \'%s\'\x00" as *const u8 as *const libc::c_char,
        str,
      );
      return 0 as *mut pipe;
    }
    /* It is "%%", "%+" or "%" - current job */
    jobnum = (*ptr_to_globals).last_jobid;
    if jobnum == 0i32 as libc::c_uint {
      bb_simple_error_msg(b"no current job\x00" as *const u8 as *const libc::c_char);
      return 0 as *mut pipe;
    }
  }
  pi = (*ptr_to_globals).job_list;
  while !pi.is_null() {
    if (*pi).jobid == jobnum {
      return pi;
    }
    pi = (*pi).next
  }
  bb_error_msg(
    b"%u: no such job\x00" as *const u8 as *const libc::c_char,
    jobnum,
  );
  return 0 as *mut pipe;
}
unsafe extern "C" fn builtin_jobs(mut _argv: *mut *mut libc::c_char) -> libc::c_int {
  let mut job: *mut pipe = 0 as *mut pipe;
  let mut status_string: *const libc::c_char = 0 as *const libc::c_char;
  checkjobs(0 as *mut pipe, 0i32);
  job = (*ptr_to_globals).job_list;
  while !job.is_null() {
    if (*job).alive_cmds == (*job).stopped_cmds {
      status_string = b"Stopped\x00" as *const u8 as *const libc::c_char
    } else {
      status_string = b"Running\x00" as *const u8 as *const libc::c_char
    }
    printf(
      b"[%u] %-22s %.40s\n\x00" as *const u8 as *const libc::c_char,
      (*job).jobid,
      status_string,
      (*job).cmdtext,
    );
    job = (*job).next
  }
  clean_up_last_dead_job();
  return 0i32;
}
/* built-in 'fg' and 'bg' handler */
unsafe extern "C" fn builtin_fg_bg(mut argv: *mut *mut libc::c_char) -> libc::c_int {
  let mut current_block: u64;
  let mut i: libc::c_int = 0;
  let mut pi: *mut pipe = 0 as *mut pipe;
  if (*ptr_to_globals).interactive_fd == 0 {
    return 1i32;
  }
  /* If they gave us no args, assume they want the last backgrounded task */
  if (*argv.offset(1)).is_null() {
    pi = (*ptr_to_globals).job_list;
    loop {
      if pi.is_null() {
        current_block = 10886091980245723256;
        break;
      }
      if (*pi).jobid == (*ptr_to_globals).last_jobid {
        current_block = 7536034456381975812;
        break;
      }
      pi = (*pi).next
    }
    match current_block {
      7536034456381975812 => {}
      _ => {
        bb_error_msg(
          b"%s: no current job\x00" as *const u8 as *const libc::c_char,
          *argv.offset(0),
        );
        return 1i32;
      }
    }
  } else {
    pi = parse_jobspec(*argv.offset(1));
    if pi.is_null() {
      return 1i32;
    }
  }
  /* TODO: bash prints a string representation
   * of job being foregrounded (like "sleep 1 | cat") */
  if *(*argv.offset(0)).offset(0) as libc::c_int == 'f' as i32
    && (*ptr_to_globals).saved_tty_pgrp != 0
  {
    /* Put the job into the foreground.  */
    tcsetpgrp((*ptr_to_globals).interactive_fd, (*pi).pgrp);
  }
  /* Restart the processes in the job */
  i = 0i32; /* FG job shouldn't be in job table */
  while i < (*pi).num_cmds {
    i += 1
  }
  (*pi).stopped_cmds = 0i32;
  i = kill(-(*pi).pgrp, 18i32);
  if i < 0i32 {
    if *bb_errno == 3i32 {
      delete_finished_job(pi);
      return 0i32;
    }
    bb_simple_perror_msg(b"kill (SIGCONT)\x00" as *const u8 as *const libc::c_char);
  }
  if *(*argv.offset(0)).offset(0) as libc::c_int == 'f' as i32 {
    remove_job_from_table(pi);
    return checkjobs_and_fg_shell(pi);
  }
  return 0i32;
}
unsafe extern "C" fn builtin_kill(mut argv: *mut *mut libc::c_char) -> libc::c_int {
  let mut ret: libc::c_int = 0i32;
  if !(*argv.offset(1)).is_null()
    && strcmp(
      *argv.offset(1),
      b"-l\x00" as *const u8 as *const libc::c_char,
    ) != 0i32
  {
    let mut i: libc::c_int = 1i32;
    loop {
      let mut pi: *mut pipe = 0 as *mut pipe;
      let mut dst: *mut libc::c_char = 0 as *mut libc::c_char;
      let mut j: libc::c_int = 0;
      let mut n: libc::c_int = 0;
      if !(*(*argv.offset(i as isize)).offset(0) as libc::c_int != '%' as i32) {
        /*
         * "kill %N" - job kill
         * Converting to pgrp / pid kill
         */
        pi = parse_jobspec(*argv.offset(i as isize));
        if pi.is_null() {
          /* Eat bad jobspec */
          j = i;
          loop {
            j += 1;
            let ref mut fresh55 = *argv.offset((j - 1i32) as isize);
            *fresh55 = *argv.offset(j as isize);
            if (*argv.offset(j as isize)).is_null() {
              break;
            }
          }
          ret = 1i32;
          i -= 1
        } else {
          /*
           * In jobs started under job control, we signal
           * entire process group by kill -PGRP_ID.
           * This happens, f.e., in interactive shell.
           *
           * Otherwise, we signal each child via
           * kill PID1 PID2 PID3.
           * Testcases:
           * sh -c 'sleep 1|sleep 1 & kill %1'
           * sh -c 'true|sleep 2 & sleep 1; kill %1'
           * sh -c 'true|sleep 1 & sleep 2; kill %1'
           */
          n = if (*ptr_to_globals).interactive_fd != 0 {
            1i32
          } else {
            (*pi).num_cmds
          };
          let mut fresh56 = ::std::vec::from_elem(
            0,
            (n as libc::c_ulong)
              .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
              .wrapping_mul(4i32 as libc::c_ulong) as usize,
          );
          dst = fresh56.as_mut_ptr() as *mut libc::c_char;
          let ref mut fresh57 = *argv.offset(i as isize);
          *fresh57 = dst;
          if (*ptr_to_globals).interactive_fd != 0 {
            dst = dst.offset(sprintf(
              dst,
              b" -%u\x00" as *const u8 as *const libc::c_char,
              (*pi).pgrp,
            ) as isize)
          } else {
            j = 0i32;
            while j < n {
              let mut cmd: *mut command = &mut *(*pi).cmds.offset(j as isize) as *mut command;
              /* Skip exited members of the job */
              if !((*cmd).pid == 0i32) {
                /*
                 * kill_main has matching code to expect
                 * leading space. Needed to not confuse
                 * negative pids with "kill -SIGNAL_NO" syntax
                 */
                dst = dst.offset(sprintf(
                  dst,
                  b" %u\x00" as *const u8 as *const libc::c_char,
                  (*cmd).pid,
                ) as isize)
              }
              j += 1
            }
          }
          *dst = '\u{0}' as i32 as libc::c_char
        }
      }
      i += 1;
      if (*argv.offset(i as isize)).is_null() {
        break;
      }
    }
  }
  if !(*argv.offset(1)).is_null() || ret == 0i32 {
    ret = run_applet_main(
      argv,
      Some(
        kill_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
      ),
    )
  }
  /* else: ret = 1, "kill %bad_jobspec" case */
  return ret;
}
/* http://www.opengroup.org/onlinepubs/9699919799/utilities/wait.html */
unsafe extern "C" fn wait_for_child_or_signal(
  mut waitfor_pipe: *mut pipe,
  mut waitfor_pid: pid_t,
) -> libc::c_int {
  let mut ret: libc::c_int = 0i32;
  loop {
    let mut sig: libc::c_int = 0;
    let mut oldset: sigset_t = __sigset_t { __val: [0; 16] };
    if !(sigisemptyset(&mut (*ptr_to_globals).pending_set) == 0) {
      /* SIGCHLD, or no signal, or ignored one, such as SIGQUIT. Repeat */
      /* waitpid is not interruptible by SA_RESTARTed
       * signals which we use. Thus, this ugly dance:
       */
      /* Make sure possible SIGCHLD is stored in kernel's
       * pending signal mask before we call waitpid.
       * Or else we may race with SIGCHLD, lose it,
       * and get stuck in sigsuspend...
       */
      sigfillset(&mut oldset); /* block all signals, remember old set */
      sigprocmask2(2i32, &mut oldset);
      if !(sigisemptyset(&mut (*ptr_to_globals).pending_set) == 0) {
        /*errno = 0; - checkjobs does this */
        /* Can't pass waitfor_pipe into checkjobs(): it won't be interruptible */
        ret = checkjobs(0 as *mut pipe, waitfor_pid); /* waitpid(WNOHANG) inside */
        if !waitfor_pipe.is_null() {
          let mut rcode: libc::c_int = job_exited_or_stopped(waitfor_pipe);
          if rcode >= 0i32 {
            ret = rcode;
            sigprocmask(2i32, &mut oldset, 0 as *mut sigset_t);
            break;
          }
        }
        /* if ECHILD, there are no children (ret is -1 or 0) */
        /* if ret == 0, no children changed state */
        /* if ret != 0, it's exitcode+1 of exited waitfor_pid child */
        if *bb_errno == 10i32 || ret != 0 {
          ret -= 1;
          if ret < 0i32 {
            /* if ECHILD, may need to fix "ret" */
            ret = 0i32
          }
          if waitfor_pid == -1i32 && *bb_errno == 10i32 {
            /* exitcode of "wait -n" with no children is 127, not 0 */
            ret = 127i32
          }
          sigprocmask(2i32, &mut oldset, 0 as *mut sigset_t);
          break;
        } else {
          /* Wait for SIGCHLD or any other signal */
          /* It is vitally important for sigsuspend that SIGCHLD has non-DFL handler! */
          /* Note: sigsuspend invokes signal handler */
          sigsuspend(&mut oldset);
        }
      }
      /* Crap! we raced with some signal! */
      sigprocmask(2i32, &mut oldset, 0 as *mut sigset_t);
    }
    /* So, did we get a signal? */
    sig = check_and_run_traps();
    if !(sig != 0) {
      continue;
    }
    /*&& sig != SIGCHLD - always true */
    /* Do this for any (non-ignored) signal, not only for ^C */
    ret = 128i32 + sig;
    break;
  }
  return ret;
}
unsafe extern "C" fn builtin_wait(mut argv: *mut *mut libc::c_char) -> libc::c_int {
  let mut ret: libc::c_int = 0;
  let mut status: libc::c_int = 0;
  argv = skip_dash_dash(argv);
  if !(*argv.offset(0)).is_null()
    && strcmp(
      *argv.offset(0),
      b"-n\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
  {
    /* wait -n */
    /* (bash accepts "wait -n PID" too and ignores PID) */
    (*ptr_to_globals).dead_job_exitcode = -1i32;
    return wait_for_child_or_signal(0 as *mut pipe, -1i32);
  }
  if (*argv.offset(0)).is_null() {
    /* Don't care about wait results */
    /* Note 1: must wait until there are no more children */
    /* Note 2: must be interruptible */
    /* Examples:
     * $ sleep 3 & sleep 6 & wait
     * [1] 30934 sleep 3
     * [2] 30935 sleep 6
     * [1] Done                   sleep 3
     * [2] Done                   sleep 6
     * $ sleep 3 & sleep 6 & wait
     * [1] 30936 sleep 3
     * [2] 30937 sleep 6
     * [1] Done                   sleep 3
     * ^C <-- after ~4 sec from keyboard
     * $
     */
    return wait_for_child_or_signal(0 as *mut pipe, 0i32);
  } /* bash compat for bad jobspecs */
  loop {
    let mut pid: pid_t = bb_strtou(*argv, 0 as *mut *mut libc::c_char, 10i32) as pid_t;
    if *bb_errno != 0 || pid <= 0i32 {
      if *(*argv.offset(0)).offset(0) as libc::c_int == '%' as i32 {
        let mut wait_pipe: *mut pipe = 0 as *mut pipe;
        ret = 127i32;
        wait_pipe = parse_jobspec(*argv);
        if !wait_pipe.is_null() {
          ret = job_exited_or_stopped(wait_pipe);
          if ret < 0i32 {
            ret = wait_for_child_or_signal(wait_pipe, 0i32)
          } else {
            /* waiting on "last dead job" removes it */
            clean_up_last_dead_job();
          }
        }
      } else {
        /* mimic bash message */
        bb_error_msg(
          b"wait: \'%s\': not a pid or valid job spec\x00" as *const u8 as *const libc::c_char,
          *argv,
        );
        ret = 1i32
      }
    /* bash checks all argv[] */
    } else {
      /* Do we have such child? */
      ret = waitpid(pid, &mut status, 1i32);
      if ret < 0i32 {
        /* No */
        ret = 127i32;
        if *bb_errno == 10i32 {
          if pid == (*ptr_to_globals).last_bg_pid {
            /* bash checks all argv[] */
            /* "wait $!" but last bg task has already exited. Try:
             * (sleep 1; exit 3) & sleep 2; echo $?; wait $!; echo $?
             * In bash it prints exitcode 0, then 3.
             * In dash, it is 127.
             */
            ret = (*ptr_to_globals).last_bg_pid_exitcode as libc::c_int
          } else {
            /* Example: "wait 1". mimic bash message */
            bb_error_msg(
              b"wait: pid %d is not a child of this shell\x00" as *const u8 as *const libc::c_char,
              pid,
            );
          }
        } else {
          /* ??? */
          bb_perror_msg(b"wait %s\x00" as *const u8 as *const libc::c_char, *argv);
        }
      } else if ret == 0i32 {
        /* Yes, and it still runs */
        ret = wait_for_child_or_signal(0 as *mut pipe, pid)
      } else {
        /* Yes, and it just exited */
        process_wait_result(0 as *mut pipe, pid, status);
        ret = (status & 0xff00i32) >> 8i32;
        if ((status & 0x7fi32) + 1i32) as libc::c_schar as libc::c_int >> 1i32 > 0i32 {
          ret = 128i32 + (status & 0x7fi32)
        }
      }
    }
    /* else: parse_jobspec() already emitted error msg */
    argv = argv.offset(1);
    if (*argv).is_null() {
      break;
    }
  }
  return ret;
}
unsafe extern "C" fn parse_numeric_argv1(
  mut argv: *mut *mut libc::c_char,
  mut def: libc::c_uint,
  mut def_min: libc::c_uint,
) -> libc::c_uint {
  if !(*argv.offset(1)).is_null() {
    def = bb_strtou(*argv.offset(1), 0 as *mut *mut libc::c_char, 10i32);
    if *bb_errno != 0 || def < def_min || !(*argv.offset(2)).is_null() {
      bb_error_msg(
        b"%s: bad arguments\x00" as *const u8 as *const libc::c_char,
        *argv.offset(0),
      );
      def = (2147483647i32 as libc::c_uint)
        .wrapping_mul(2u32)
        .wrapping_add(1u32)
    }
  }
  return def;
}
unsafe extern "C" fn builtin_break(mut argv: *mut *mut libc::c_char) -> libc::c_int {
  let mut depth: libc::c_uint = 0;
  if (*ptr_to_globals).depth_of_loop == 0i32 as libc::c_uint {
    bb_error_msg(
      b"%s: only meaningful in a loop\x00" as *const u8 as *const libc::c_char,
      *argv.offset(0),
    );
    /* bash compat */
    (*ptr_to_globals).flag_break_continue = 0i32 as smallint;
    return 0i32;
  }
  /* if we came from builtin_continue(), need to undo "= 1" */
  (*ptr_to_globals).flag_break_continue += 1; /* BC_BREAK = 1, or BC_CONTINUE = 2 */
  depth = parse_numeric_argv1(argv, 1i32 as libc::c_uint, 1i32 as libc::c_uint); /* BC_CONTINUE = 2 = 1+1 */
  (*ptr_to_globals).depth_break_continue = depth;
  if depth
    == (2147483647i32 as libc::c_uint)
      .wrapping_mul(2u32)
      .wrapping_add(1u32)
  {
    (*ptr_to_globals).flag_break_continue = BC_BREAK as libc::c_int as smallint
  }
  if (*ptr_to_globals).depth_of_loop < depth {
    (*ptr_to_globals).depth_break_continue = (*ptr_to_globals).depth_of_loop
  }
  return 0i32;
}
unsafe extern "C" fn builtin_continue(mut argv: *mut *mut libc::c_char) -> libc::c_int {
  (*ptr_to_globals).flag_break_continue = 1i32 as smallint;
  return builtin_break(argv);
}
unsafe extern "C" fn builtin_return(mut argv: *mut *mut libc::c_char) -> libc::c_int {
  let mut rc: libc::c_int = 0;
  if (*ptr_to_globals).flag_return_in_progress as libc::c_int != -1i32 {
    bb_error_msg(
      b"%s: not in a function or sourced script\x00" as *const u8 as *const libc::c_char,
      *argv.offset(0),
    );
    return 1i32;
    /* bash compat */
  }
  (*ptr_to_globals).flag_return_in_progress = 1i32 as smallint;
  /* bash:
   * out of range: wraps around at 256, does not error out
   * non-numeric param:
   * f() { false; return qwe; }; f; echo $?
   * bash: return: qwe: numeric argument required  <== we do this
   * 255  <== we also do this
   */
  rc = parse_numeric_argv1(
    argv,
    (*ptr_to_globals).last_exitcode as libc::c_uint,
    0i32 as libc::c_uint,
  ) as libc::c_int;
  return rc;
}
unsafe extern "C" fn builtin_times(mut _argv: *mut *mut libc::c_char) -> libc::c_int {
  static mut times_tbl: [uint8_t; 9] = [
    ' ' as i32 as uint8_t,
    0u64 as uint8_t,
    '\n' as i32 as uint8_t,
    8u64 as uint8_t,
    ' ' as i32 as uint8_t,
    16u64 as uint8_t,
    '\n' as i32 as uint8_t,
    24u64 as uint8_t,
    0i32 as uint8_t,
  ];
  let mut p: *const uint8_t = 0 as *const uint8_t;
  let mut clk_tck: libc::c_uint = 0;
  let mut buf: tms = tms {
    tms_utime: 0,
    tms_stime: 0,
    tms_cutime: 0,
    tms_cstime: 0,
  };
  clk_tck = bb_clk_tck();
  times(&mut buf);
  p = times_tbl.as_ptr();
  loop {
    let mut sec: libc::c_uint = 0;
    let mut frac: libc::c_uint = 0;
    let mut t: libc::c_ulong = 0;
    t = *((&mut buf as *mut tms as *mut libc::c_char).offset(*p.offset(1) as libc::c_int as isize)
      as *mut clock_t) as libc::c_ulong;
    sec = t.wrapping_div(clk_tck as libc::c_ulong) as libc::c_uint;
    frac = t.wrapping_rem(clk_tck as libc::c_ulong) as libc::c_uint;
    printf(
      b"%um%u.%03us%c\x00" as *const u8 as *const libc::c_char,
      sec.wrapping_div(60i32 as libc::c_uint),
      sec.wrapping_rem(60i32 as libc::c_uint),
      frac
        .wrapping_mul(1000i32 as libc::c_uint)
        .wrapping_div(clk_tck),
      *p.offset(0) as libc::c_int,
    );
    p = p.offset(2);
    if !(*p != 0) {
      break;
    }
  }
  return 0i32;
}
