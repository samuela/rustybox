use c2rust_bitfields;
use c2rust_bitfields::BitfieldStruct;
use libc;

extern "C" {
  pub type __dirstream;
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
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
  #[no_mangle]
  fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn chdir(__path: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn getcwd(__buf: *mut libc::c_char, __size: size_t) -> *mut libc::c_char;
  #[no_mangle]
  fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
  #[no_mangle]
  static mut environ: *mut *mut libc::c_char;
  #[no_mangle]
  fn execve(
    __path: *const libc::c_char,
    __argv: *const *mut libc::c_char,
    __envp: *const *mut libc::c_char,
  ) -> libc::c_int;
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
  fn fork() -> __pid_t;
  #[no_mangle]
  fn isatty(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn tcgetpgrp(__fd: libc::c_int) -> __pid_t;
  #[no_mangle]
  fn tcsetpgrp(__fd: libc::c_int, __pgrp_id: __pid_t) -> libc::c_int;
  #[no_mangle]
  fn opendir(__name: *const libc::c_char) -> *mut DIR;
  #[no_mangle]
  fn closedir(__dirp: *mut DIR) -> libc::c_int;
  #[no_mangle]
  fn readdir(__dirp: *mut DIR) -> *mut dirent;
  #[no_mangle]
  fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
  #[no_mangle]
  fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
  #[no_mangle]
  fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
  #[no_mangle]
  fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
  #[no_mangle]
  fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
  #[no_mangle]
  fn killpg(__pgrp: __pid_t, __sig: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn raise(__sig: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn sigfillset(__set: *mut sigset_t) -> libc::c_int;
  #[no_mangle]
  fn sigprocmask(__how: libc::c_int, __set: *const sigset_t, __oset: *mut sigset_t) -> libc::c_int;
  #[no_mangle]
  fn sigsuspend(__set: *const sigset_t) -> libc::c_int;
  #[no_mangle]
  fn sigaction(__sig: libc::c_int, __act: *const sigaction, __oact: *mut sigaction) -> libc::c_int;
  #[no_mangle]
  static mut stdout: *mut _IO_FILE;
  #[no_mangle]
  static mut stderr: *mut _IO_FILE;
  #[no_mangle]
  fn fflush(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn vfprintf(_: *mut FILE, _: *const libc::c_char, _: ::std::ffi::VaList) -> libc::c_int;
  #[no_mangle]
  fn vprintf(_: *const libc::c_char, _: ::std::ffi::VaList) -> libc::c_int;
  #[no_mangle]
  fn vsnprintf(
    _: *mut libc::c_char,
    _: libc::c_ulong,
    _: *const libc::c_char,
    _: ::std::ffi::VaList,
  ) -> libc::c_int;
  #[no_mangle]
  fn dprintf(__fd: libc::c_int, __fmt: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn putc_unlocked(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn puts(__s: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn clearerr(__stream: *mut FILE);
  #[no_mangle]
  fn ferror_unlocked(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn strtoul(
    __nptr: *const libc::c_char,
    __endptr: *mut *mut libc::c_char,
    __base: libc::c_int,
  ) -> libc::c_ulong;
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  fn abort() -> !;
  #[no_mangle]
  fn bsearch(
    __key: *const libc::c_void,
    __base: *const libc::c_void,
    __nmemb: size_t,
    __size: size_t,
    __compar: __compar_fn_t,
  ) -> *mut libc::c_void;
  #[no_mangle]
  fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t, __compar: __compar_fn_t);
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
  #[no_mangle]
  fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strchrnul(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
  #[no_mangle]
  fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn mempcpy(
    __dest: *mut libc::c_void,
    __src: *const libc::c_void,
    __n: size_t,
  ) -> *mut libc::c_void;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn strerror(_: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strsignal(__sig: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn stpcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
  #[no_mangle]
  fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
  #[no_mangle]
  fn lstat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
  #[no_mangle]
  fn umask(__mask: __mode_t) -> __mode_t;
  #[no_mangle]
  fn gettimeofday(__tv: *mut timeval, __tz: __timezone_ptr_t) -> libc::c_int;
  #[no_mangle]
  fn waitpid(__pid: __pid_t, __stat_loc: *mut libc::c_int, __options: libc::c_int) -> __pid_t;
  /* Search for an entry with a matching username.  */
  #[no_mangle]
  fn bb_internal_getpwnam(__name: *const libc::c_char) -> *mut passwd;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xrealloc(old: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xmemdup(s: *const libc::c_void, n: libc::c_int) -> *mut libc::c_void;
  #[no_mangle]
  fn bb_process_escape_sequence(ptr: *mut *const libc::c_char) -> libc::c_char;
  #[no_mangle]
  fn endofname(name: *const libc::c_char) -> *const libc::c_char;
  #[no_mangle]
  fn close_on_exec_on(fd: libc::c_int);
  #[no_mangle]
  fn sigaction_set(sig: libc::c_int, act: *const sigaction) -> libc::c_int;
  #[no_mangle]
  fn sigprocmask_allsigs(how: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn sigprocmask2(how: libc::c_int, set: *mut sigset_t) -> libc::c_int;
  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn nonblock_immune_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn bb_clk_tck() -> libc::c_uint;
  #[no_mangle]
  fn safe_write(fd: libc::c_int, buf: *const libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn full_write(fd: libc::c_int, buf: *const libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn fflush_all() -> libc::c_int;
  #[no_mangle]
  fn utoa(n: libc::c_uint) -> *mut libc::c_char;
  #[no_mangle]
  fn itoa(n: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_strtou(
    arg: *const libc::c_char,
    endp: *mut *mut libc::c_char,
    base: libc::c_int,
  ) -> libc::c_uint;
  #[no_mangle]
  fn xfunc_die() -> !;
  #[no_mangle]
  fn get_script_content(n: libc::c_uint) -> *mut libc::c_char;
  #[no_mangle]
  static bb_msg_memory_exhausted: [libc::c_char; 0];
  #[no_mangle]
  static bb_msg_requires_arg: [libc::c_char; 0];
  #[no_mangle]
  fn concat_path_file(
    path: *const libc::c_char,
    filename: *const libc::c_char,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn size_from_HISTFILESIZE(hp: *const libc::c_char) -> libc::c_uint;
  #[no_mangle]
  static bb_PATH_root_path: [libc::c_char; 0];
  #[no_mangle]
  fn echo_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  static bb_busybox_exec_path: [libc::c_char; 0];
  #[no_mangle]
  fn show_history(st: *const line_input_t);
  #[no_mangle]
  fn kill_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn printf_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn new_line_input_t(flags: libc::c_int) -> *mut line_input_t;
  #[no_mangle]
  fn test_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn get_signame(number_0: libc::c_int) -> *const libc::c_char;
  #[no_mangle]
  fn get_signum(name: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn bb_parse_mode(s: *const libc::c_char, cur_mode: libc::c_uint) -> libc::c_int;
  #[no_mangle]
  fn read_line_input(
    st: *mut line_input_t,
    prompt: *const libc::c_char,
    command: *mut libc::c_char,
    maxsize: libc::c_int,
  ) -> libc::c_int;
  #[no_mangle]
  fn xmalloc(size: size_t) -> *mut libc::c_void;
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
  #[no_mangle]
  static defoptindvar: [libc::c_char; 0];
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
  fn next_random(rnd: *mut random_t) -> uint32_t;
  #[no_mangle]
  static ash_ptr_to_globals_misc: *mut globals_misc;
  #[no_mangle]
  static ash_ptr_to_globals_memstack: *mut globals_memstack;
  #[no_mangle]
  static ash_ptr_to_globals_var: *mut globals_var;
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
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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
pub type smallint = libc::c_schar;
pub type smalluint = libc::c_uchar;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type pid_t = __pid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
  pub d_ino: __ino64_t,
  pub d_off: __off64_t,
  pub d_reclen: libc::c_ushort,
  pub d_type: libc::c_uchar,
  pub d_name: [libc::c_char; 256],
}
pub type DIR = __dirstream;
pub type mode_t = __mode_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
  pub tv_sec: __time_t,
  pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
  pub st_dev: __dev_t,
  pub st_ino: __ino_t,
  pub st_nlink: __nlink_t,
  pub st_mode: __mode_t,
  pub st_uid: __uid_t,
  pub st_gid: __gid_t,
  pub __pad0: libc::c_int,
  pub st_rdev: __dev_t,
  pub st_size: __off_t,
  pub st_blksize: __blksize_t,
  pub st_blocks: __blkcnt_t,
  pub st_atim: timespec,
  pub st_mtim: timespec,
  pub st_ctim: timespec,
  pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
  pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
  pub tv_sec: __time_t,
  pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
  pub sival_int: libc::c_int,
  pub sival_ptr: *mut libc::c_void,
}
pub type __sigval_t = sigval;
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
  pub __jmpbuf: __jmp_buf,
  pub __mask_was_saved: libc::c_int,
  pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
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
pub struct _IO_FILE {
  pub _flags: libc::c_int,
  pub _IO_read_ptr: *mut libc::c_char,
  pub _IO_read_end: *mut libc::c_char,
  pub _IO_read_base: *mut libc::c_char,
  pub _IO_write_base: *mut libc::c_char,
  pub _IO_write_ptr: *mut libc::c_char,
  pub _IO_write_end: *mut libc::c_char,
  pub _IO_buf_base: *mut libc::c_char,
  pub _IO_buf_end: *mut libc::c_char,
  pub _IO_save_base: *mut libc::c_char,
  pub _IO_backup_base: *mut libc::c_char,
  pub _IO_save_end: *mut libc::c_char,
  pub _markers: *mut _IO_marker,
  pub _chain: *mut _IO_FILE,
  pub _fileno: libc::c_int,
  pub _flags2: libc::c_int,
  pub _old_offset: __off_t,
  pub _cur_column: libc::c_ushort,
  pub _vtable_offset: libc::c_schar,
  pub _shortbuf: [libc::c_char; 1],
  pub _lock: *mut libc::c_void,
  pub _offset: __off64_t,
  pub __pad1: *mut libc::c_void,
  pub __pad2: *mut libc::c_void,
  pub __pad3: *mut libc::c_void,
  pub __pad4: *mut libc::c_void,
  pub __pad5: size_t,
  pub _mode: libc::c_int,
  pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
  pub _next: *mut _IO_marker,
  pub _sbuf: *mut _IO_FILE,
  pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
pub type va_list = __builtin_va_list;
pub type __compar_fn_t =
  Option<unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> libc::c_int>;
pub type ptrdiff_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timezone {
  pub tz_minuteswest: libc::c_int,
  pub tz_dsttime: libc::c_int,
}
pub type __timezone_ptr_t = *mut timezone;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
  pub pw_name: *mut libc::c_char,
  pub pw_passwd: *mut libc::c_char,
  pub pw_uid: __uid_t,
  pub pw_gid: __gid_t,
  pub pw_gecos: *mut libc::c_char,
  pub pw_dir: *mut libc::c_char,
  pub pw_shell: *mut libc::c_char,
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
/*
 * We enclose jmp_buf in a structure so that we can declare pointers to
 * jump locations.  The global variable handler contains the location to
 * jump to when an exception occurs, and the global variable exception_type
 * contains a code identifying the exception.  To implement nested
 * exception handlers, the user should save the value of handler on entry
 * to an inner scope, set handler to point to a jmploc structure for the
 * inner scope, and restore handler on exit from the scope.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jmploc {
  pub loc: jmp_buf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals_misc {
  pub exitstatus: uint8_t,
  pub back_exitstatus: uint8_t,
  pub job_warning: smallint,
  pub rootpid: libc::c_int,
  pub shlvl: libc::c_int,
  pub errlinno: libc::c_int,
  pub minusc: *mut libc::c_char,
  pub curdir: *mut libc::c_char,
  pub physdir: *mut libc::c_char,
  pub arg0: *mut libc::c_char,
  pub exception_handler: *mut jmploc,
  pub suppress_int: libc::c_int,
  pub pending_int: smallint,
  pub got_sigchld: smallint,
  pub pending_sig: smallint,
  pub exception_type: smallint,
  pub nullstr: [libc::c_char; 1],
  pub optlist: [libc::c_char; 16],
  pub sigmode: [libc::c_char; 64],
  pub gotsig: [uint8_t; 64],
  pub may_have_traps: uint8_t,
  pub trap: [*mut libc::c_char; 65],
  pub trap_ptr: *mut *mut libc::c_char,
  pub random_gen: random_t,
  pub backgndpid: pid_t,
  /* pid of last background process */
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct random_t {
  pub galois_LFSR: int32_t,
  pub LCG: uint32_t,
  pub xs64_x: uint32_t,
  pub xs64_y: uint32_t,
}
/*
 * Structure specifying which parts of the string should be searched
 * for IFS characters.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifsregion {
  pub next: *mut ifsregion,
  pub begoff: libc::c_int,
  pub endoff: libc::c_int,
  pub nulonly: libc::c_int,
  /* search for nul bytes only */
}
/* ============ Debugging output */
/* DEBUG */
/* ============ Parser data */
/*
 * ash_vmsg() needs parsefile->fd, hence parsefile definition is moved up.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct strlist {
  pub next: *mut strlist,
  pub text: *mut libc::c_char,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct job {
  pub ps0: procstat,
  pub ps: *mut procstat,
  pub stopstatus: libc::c_int,
  pub nprocs: libc::c_uint,
  #[bitfield(name = "state", ty = "libc::c_uint", bits = "0..=7")]
  #[bitfield(name = "sigint", ty = "libc::c_uint", bits = "8..=8")]
  #[bitfield(name = "jobctl", ty = "libc::c_uint", bits = "9..=9")]
  #[bitfield(name = "waited", ty = "libc::c_uint", bits = "10..=10")]
  #[bitfield(name = "used", ty = "libc::c_uint", bits = "11..=11")]
  #[bitfield(name = "changed", ty = "libc::c_uint", bits = "12..=12")]
  pub state_sigint_jobctl_waited_used_changed: [u8; 2],
  #[bitfield(padding)]
  pub c2rust_padding: [u8; 6],
  pub prev_job: *mut job,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct procstat {
  pub ps_pid: pid_t,
  pub ps_status: libc::c_int,
  pub ps_cmd: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union node {
  pub type_0: smallint,
  pub ncmd: ncmd,
  pub npipe: npipe,
  pub nredir: nredir,
  pub nbinary: nbinary,
  pub nif: nif,
  pub nfor: nfor,
  pub ncase: ncase,
  pub nclist: nclist,
  pub ndefun: ndefun,
  pub narg: narg,
  pub nfile: nfile,
  pub ndup: ndup,
  pub nhere: nhere,
  pub nnot: nnot,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nnot {
  pub type_0: smallint,
  pub com: *mut node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nhere {
  pub type_0: smallint,
  pub next: *mut node,
  pub fd: libc::c_int,
  pub doc: *mut node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ndup {
  pub type_0: smallint,
  pub next: *mut node,
  pub fd: libc::c_int,
  pub dupfd: libc::c_int,
  pub vname: *mut node,
  pub _unused_expfname: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nfile {
  pub type_0: smallint,
  pub next: *mut node,
  pub fd: libc::c_int,
  pub _unused_dupfd: libc::c_int,
  pub fname: *mut node,
  pub expfname: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct narg {
  pub type_0: smallint,
  pub next: *mut node,
  pub text: *mut libc::c_char,
  pub backquote: *mut nodelist,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nodelist {
  pub next: *mut nodelist,
  pub n: *mut node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ndefun {
  pub type_0: smallint,
  pub linno: libc::c_int,
  pub text: *mut libc::c_char,
  pub body: *mut node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nclist {
  pub type_0: smallint,
  pub next: *mut node,
  pub pattern: *mut node,
  pub body: *mut node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ncase {
  pub type_0: smallint,
  pub linno: libc::c_int,
  pub expr: *mut node,
  pub cases: *mut node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nfor {
  pub type_0: smallint,
  pub linno: libc::c_int,
  pub args: *mut node,
  pub body: *mut node,
  pub var: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nif {
  pub type_0: smallint,
  pub test: *mut node,
  pub ifpart: *mut node,
  pub elsepart: *mut node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nbinary {
  pub type_0: smallint,
  pub ch1: *mut node,
  pub ch2: *mut node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nredir {
  pub type_0: smallint,
  pub linno: libc::c_int,
  pub n: *mut node,
  pub redirect: *mut node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct npipe {
  pub type_0: smallint,
  pub pipe_backgnd: smallint,
  pub cmdlist: *mut nodelist,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ncmd {
  pub type_0: smallint,
  pub linno: libc::c_int,
  pub assign: *mut node,
  pub args: *mut node,
  pub redirect: *mut node,
}
/* ============ Hashing commands */
/*
 * When commands are first encountered, they are entered in a hash table.
 * This ensures that a full path search will not have to be done for them
 * on each invocation.
 *
 * We should investigate converting to a linear search, even though that
 * would make the command name "hash" a misnomer.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tblentry {
  pub next: *mut tblentry,
  pub param: param,
  pub cmdtype: smallint,
  pub rehash: libc::c_char,
  pub cmdname: [libc::c_char; 1],
  /* name of command */
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union param {
  pub index: libc::c_int,
  pub cmd: *const builtincmd,
  pub func: *mut funcnode,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct funcnode {
  pub count: libc::c_int,
  pub n: node,
}
/* ============ find_command */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct builtincmd {
  pub name: *const libc::c_char,
  pub builtin:
    Option<unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int>,
  /* unsigned flags; */
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redirtab {
  pub next: *mut redirtab,
  pub pair_count: libc::c_int,
  pub two_fd: [squirrel; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct squirrel {
  pub orig_fd: libc::c_int,
  pub moved_to: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct localvar {
  pub next: *mut localvar,
  pub vp: *mut var,
  pub flags: libc::c_int,
  pub text: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct var {
  pub next: *mut var,
  pub flags: libc::c_int,
  pub var_text: *const libc::c_char,
  pub var_func: Option<unsafe extern "C" fn(_: *const libc::c_char) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct alias {
  pub next: *mut alias,
  pub name: *mut libc::c_char,
  pub val: *mut libc::c_char,
  pub flag: libc::c_int,
}
/* ASH_GETOPTS */
/* ============ Shell parser */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct heredoc {
  pub next: *mut heredoc,
  pub here: *mut node,
  pub eofmark: *mut libc::c_char,
  pub striptabs: smallint,
  /* if set, strip leading tabs */
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct localvar_list {
  pub next: *mut localvar_list,
  pub lv: *mut localvar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arglist {
  pub list: *mut strlist,
  pub lastp: *mut *mut strlist,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct parsefile {
  pub prev: *mut parsefile,
  pub linno: libc::c_int,
  pub pf_fd: libc::c_int,
  pub left_in_line: libc::c_int,
  pub left_in_buffer: libc::c_int,
  pub next_to_pgetc: *mut libc::c_char,
  pub buf: *mut libc::c_char,
  pub strpush: *mut strpush,
  pub basestrpush: strpush,
  pub lastc: [libc::c_int; 2],
  pub unget: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct strpush {
  pub prev: *mut strpush,
  pub prev_string: *mut libc::c_char,
  pub prev_left_in_line: libc::c_int,
  pub ap: *mut alias,
  pub string: *mut libc::c_char,
  pub lastc: [libc::c_int; 2],
  pub unget: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals_memstack {
  pub g_stackp: *mut stack_block,
  pub g_stacknxt: *mut libc::c_char,
  pub sstrend: *mut libc::c_char,
  pub g_stacknleft: size_t,
  pub stackbase: stack_block,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_block {
  pub prev: *mut stack_block,
  pub space: [libc::c_char; 504],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stackmark {
  pub stackp: *mut stack_block,
  pub stacknxt: *mut libc::c_char,
  pub stacknleft: size_t,
}
pub const TEOF: C2RustUnnamed_17 = 0;
pub type token_id_t = smallint;
/* "regular" builtins always take precedence over commands,
 * regardless of PATH=....%builtin... position */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmdentry {
  pub cmdtype: smallint,
  pub u: param,
}
pub const SHELL_SIZE: C2RustUnnamed_16 = 7;
pub const MINSIZE: C2RustUnnamed_16 = 504;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals_var {
  pub shellparam: shparam,
  pub redirlist: *mut redirtab,
  pub preverrout_fd: libc::c_int,
  pub vartab: [*mut var; 39],
  pub varinit: [var; 13],
  pub lineno: libc::c_int,
  pub linenovar: [libc::c_char; 20],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct shparam {
  pub nparam: libc::c_int,
  pub optind: libc::c_int,
  pub optoff: libc::c_int,
  pub malloced: libc::c_uchar,
  pub p: *mut *mut libc::c_char,
}
/*
 * Execute a command inside back quotes.  If it's a builtin command, we
 * want to save its output in a block obtained from malloc.  Otherwise
 * we fork off a subprocess and get the output of the command via a pipe.
 * Should be called with interrupts off.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct backcmd {
  pub fd: libc::c_int,
  pub nleft: libc::c_int,
  pub buf: *mut libc::c_char,
  pub jp: *mut job,
  /* job structure for command */
}
pub const UNICODE_ON: C2RustUnnamed_13 = 2;
pub const NOPTS: C2RustUnnamed_15 = 16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct exp_t {
  pub dir: *mut libc::c_char,
  pub dir_max: libc::c_uint,
}
pub const TSEMI: C2RustUnnamed_17 = 4;
pub const TBACKGND: C2RustUnnamed_17 = 5;
pub const TNL: C2RustUnnamed_17 = 1;

#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct synstack_t {
  pub syntax: smalluint,
  #[bitfield(name = "innerdq", ty = "uint8_t", bits = "0..=0")]
  #[bitfield(name = "varpushed", ty = "uint8_t", bits = "1..=1")]
  #[bitfield(name = "dblquote", ty = "uint8_t", bits = "2..=2")]
  pub innerdq_varpushed_dblquote: [u8; 1],
  #[bitfield(padding)]
  pub c2rust_padding: [u8; 2],
  pub varnest: libc::c_int,
  pub dqvarnest: libc::c_int,
  pub parenlevel: libc::c_int,
  pub prev: *mut synstack_t,
  pub next: *mut synstack_t,
}

pub const TRP: C2RustUnnamed_17 = 10;

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
pub const TWORD: C2RustUnnamed_17 = 3;
pub const TREDIR: C2RustUnnamed_17 = 2;
pub const TENDCASE: C2RustUnnamed_17 = 11;
pub const TOR: C2RustUnnamed_17 = 7;
pub const TAND: C2RustUnnamed_17 = 6;
pub const TPIPE: C2RustUnnamed_17 = 8;
pub const TLP: C2RustUnnamed_17 = 9;
pub const INPUT_PUSH_FILE: C2RustUnnamed_19 = 1;
pub const INPUT_NOFILE_OK: C2RustUnnamed_19 = 2;
pub const WITH_PATH_LOOKUP: C2RustUnnamed_12 = 16;
pub const FOR_SHELL: C2RustUnnamed_12 = 7;
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
pub const BUILTIN_READ_RAW: C2RustUnnamed_14 = 2;
pub const BUILTIN_READ_SILENT: C2RustUnnamed_14 = 1;
pub type C2RustUnnamed_10 = libc::c_uint;
pub const VERIFY_VERBOSE: C2RustUnnamed_10 = 2;
pub const VERIFY_BRIEF: C2RustUnnamed_10 = 1;
pub const TFOR: C2RustUnnamed_17 = 21;
pub const TWHILE: C2RustUnnamed_17 = 27;
pub const TUNTIL: C2RustUnnamed_17 = 26;
pub const TCASE: C2RustUnnamed_17 = 14;
pub const TIF: C2RustUnnamed_17 = 23;
pub const TBEGIN: C2RustUnnamed_17 = 28;
pub const TFUNCTION: C2RustUnnamed_17 = 22;
pub const TEND: C2RustUnnamed_17 = 29;
pub const TESAC: C2RustUnnamed_17 = 19;
pub const TIN: C2RustUnnamed_17 = 24;
pub const TDONE: C2RustUnnamed_17 = 16;
pub const TDO: C2RustUnnamed_17 = 15;
pub const TFI: C2RustUnnamed_17 = 20;
pub const TELSE: C2RustUnnamed_17 = 18;
pub const TTHEN: C2RustUnnamed_17 = 25;
pub const TELIF: C2RustUnnamed_17 = 17;
pub const TNOT: C2RustUnnamed_17 = 13;
pub const tokendlist: C2RustUnnamed_18 = 572496897;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
  pub flags: libc::c_int,
  pub var_text: *const libc::c_char,
  pub var_func: Option<unsafe extern "C" fn(_: *const libc::c_char) -> ()>,
}
pub type C2RustUnnamed_12 = libc::c_uint;
pub const VI_MODE: C2RustUnnamed_12 = 0;
pub const USERNAME_COMPLETION: C2RustUnnamed_12 = 4;
pub const TAB_COMPLETION: C2RustUnnamed_12 = 2;
pub const DO_HISTORY: C2RustUnnamed_12 = 1;
/*  0 */
/*  1 */
/*  2 */
/*  3 */
/*  4 */
/*  5 */
/*  6 */
/*  7 */
/*  8 */
/*  9 */
/* 10 */
/* 11 */
/* 12 */
/* 13 */
/* 14 */
/* 15 */
/* 16 */
/* 17 */
/* 18 */
/* 19 */
/* 20 */
/* 21 */
/* 22 */
/* 23 */
/* 24 */
/* 25 */
/* 26 */
/* 27 */
/* 28 */
/* 29 */
/* vi: set sw=4 ts=4: */
/*
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
pub type C2RustUnnamed_13 = libc::c_uint;
pub const UNICODE_OFF: C2RustUnnamed_13 = 1;
pub const UNICODE_UNKNOWN: C2RustUnnamed_13 = 0;
pub type C2RustUnnamed_14 = libc::c_uint;
//bash 4.4.23 also has these opts (with these defaults):
//braceexpand           on
//emacs                 on
//errtrace              off
//functrace             off
//hashall               on
//histexpand            off
//history               on
//interactive-comments  on
//keyword               off
//onecmd                off
//physical              off
//posix                 off
//privileged            off
pub type C2RustUnnamed_15 = libc::c_uint;
pub type C2RustUnnamed_16 = libc::c_uint;
pub type C2RustUnnamed_17 = libc::c_uint;
pub const TENDBQUOTE: C2RustUnnamed_17 = 12;
/* Nth bit indicates if token marks the end of a list */
pub type C2RustUnnamed_18 = libc::c_uint;
/* thus far 29 bits used */
/*
 * This implements the input routines used by the parser.
 */
pub type C2RustUnnamed_19 = libc::c_uint;
#[inline(always)]
unsafe extern "C" fn not_const_pp(mut p: *const libc::c_void) -> *mut libc::c_void {
  return p as *mut libc::c_void;
}
#[inline(always)]
unsafe extern "C" fn bb_ascii_isxdigit(mut a: libc::c_uchar) -> libc::c_int {
  let mut b: libc::c_uchar = (a as libc::c_int - '0' as i32) as libc::c_uchar;
  if b as libc::c_int <= 9i32 {
    return (b as libc::c_int <= 9i32) as libc::c_int;
  }
  b = ((a as libc::c_int | 0x20i32) - 'a' as i32) as libc::c_uchar;
  return (b as libc::c_int <= 'f' as i32 - 'a' as i32) as libc::c_int;
}
#[inline(always)]
unsafe extern "C" fn bb_ascii_isalnum(mut a: libc::c_uchar) -> libc::c_int {
  let mut b: libc::c_uchar = (a as libc::c_int - '0' as i32) as libc::c_uchar;
  if b as libc::c_int <= 9i32 {
    return (b as libc::c_int <= 9i32) as libc::c_int;
  }
  b = ((a as libc::c_int | 0x20i32) - 'a' as i32) as libc::c_uchar;
  return (b as libc::c_int <= 'z' as i32 - 'a' as i32) as libc::c_int;
}
/* should be prime */
/* ============ Shell options */
static mut optletters_optnames: [*const libc::c_char; 16] = [
  b"eerrexit\x00" as *const u8 as *const libc::c_char,
  b"fnoglob\x00" as *const u8 as *const libc::c_char,
  b"Iignoreeof\x00" as *const u8 as *const libc::c_char,
  b"i\x00" as *const u8 as *const libc::c_char,
  b"mmonitor\x00" as *const u8 as *const libc::c_char,
  b"nnoexec\x00" as *const u8 as *const libc::c_char,
  b"s\x00" as *const u8 as *const libc::c_char,
  b"c\x00" as *const u8 as *const libc::c_char,
  b"xxtrace\x00" as *const u8 as *const libc::c_char,
  b"vverbose\x00" as *const u8 as *const libc::c_char,
  b"Cnoclobber\x00" as *const u8 as *const libc::c_char,
  b"aallexport\x00" as *const u8 as *const libc::c_char,
  b"bnotify\x00" as *const u8 as *const libc::c_char,
  b"unounset\x00" as *const u8 as *const libc::c_char,
  b"\x00vi\x00" as *const u8 as *const libc::c_char,
  b"\x00pipefail\x00" as *const u8 as *const libc::c_char,
];
/* ============ DEBUG */
/* ============ Utility functions */
unsafe extern "C" fn isdigit_str9(mut str: *const libc::c_char) -> libc::c_int {
  let mut maxlen: libc::c_int = 9i32 + 1i32; /* max 9 digits: 999999999 */
  loop {
    maxlen -= 1;
    if !(maxlen != 0 && (*str as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32)
    {
      break;
    }
    str = str.offset(1)
  }
  return (*str as libc::c_int == '\u{0}' as i32) as libc::c_int;
}
unsafe extern "C" fn var_end(mut var: *const libc::c_char) -> *const libc::c_char {
  while *var != 0 {
    let fresh0 = var;
    var = var.offset(1);
    if *fresh0 as libc::c_int == '=' as i32 {
      break;
    }
  }
  return var;
}
/*
 * These macros allow the user to suspend the handling of interrupt signals
 * over a period of time.  This is similar to SIGHOLD or to sigblock, but
 * much more efficient and portable.  (But hacking the kernel is so much
 * more fun than worrying about efficiency and portability. :-))
 */
/*
 * Called to raise an exception.  Since C doesn't include exceptions, we
 * just do a longjmp to the exception handler.  The type of exception is
 * stored in the global variable "exception_type".
 */
unsafe extern "C" fn raise_exception(mut e: libc::c_int) -> ! {
  ::std::ptr::write_volatile(
    &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
    ::std::ptr::read_volatile::<libc::c_int>(
      &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
    ) + 1,
  );
  asm!("" : : : "memory" : "volatile");
  (*ash_ptr_to_globals_misc).exception_type = e as smallint;
  longjmp(
    (*(*ash_ptr_to_globals_misc).exception_handler)
      .loc
      .as_mut_ptr(),
    1i32,
  );
}
/*
 * Called when a SIGINT is received.  (If the user specifies
 * that SIGINT is to be trapped or ignored using the trap builtin, then
 * this routine is not called.)  Suppressint is nonzero when interrupts
 * are held using the INT_OFF macro.  (The test for iflag is just
 * defensive programming.)
 */
unsafe extern "C" fn raise_interrupt() -> ! {
  ::std::ptr::write_volatile(
    &mut (*ash_ptr_to_globals_misc).pending_int as *mut smallint,
    0i32 as smallint,
  );
  /* Signal is not automatically unmasked after it is raised,
   * do it ourself - unmask all signals */
  sigprocmask_allsigs(1i32);
  /* pending_sig = 0; - now done in signal_handler() */
  if !((*ash_ptr_to_globals_misc).shlvl == 0
    && (*ash_ptr_to_globals_misc).optlist[3] as libc::c_int != 0)
  {
    /* Kill ourself with SIGINT */
    signal(2i32, None);
    raise(2i32);
  }
  /* bash: ^C even on empty command line sets $? */
  (*ash_ptr_to_globals_misc).exitstatus = (2i32 + 128i32) as uint8_t;
  raise_exception(0i32);
  /* NOTREACHED */
}
#[inline]
unsafe extern "C" fn int_on() {
  asm!("" : : : "memory" : "volatile");
  ::std::ptr::write_volatile(
    &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
    ::std::ptr::read_volatile::<libc::c_int>(
      &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
    ) - 1,
  );
  if ::std::ptr::read_volatile::<libc::c_int>(
    &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
  ) == 0i32
    && (*ash_ptr_to_globals_misc).pending_int as libc::c_int != 0
  {
    raise_interrupt();
  };
}
#[inline]
unsafe extern "C" fn force_int_on() {
  asm!("" : : : "memory" : "volatile");
  ::std::ptr::write_volatile(
    &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
    0i32,
  );
  if (*ash_ptr_to_globals_misc).pending_int != 0 {
    raise_interrupt();
  };
}
/* ============ Stdout/stderr output */
unsafe extern "C" fn outstr(mut p: *const libc::c_char, mut file: *mut FILE) {
  ::std::ptr::write_volatile(
    &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
    ::std::ptr::read_volatile::<libc::c_int>(
      &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
    ) + 1,
  );
  asm!("" : : : "memory" : "volatile");
  fputs_unlocked(p, file);
  int_on();
}
unsafe extern "C" fn flush_stdout_stderr() {
  ::std::ptr::write_volatile(
    &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
    ::std::ptr::read_volatile::<libc::c_int>(
      &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
    ) + 1,
  );
  asm!("" : : : "memory" : "volatile");
  fflush_all();
  int_on();
}
/* Was called outcslow(c,FILE*), but c was always '\n' */
unsafe extern "C" fn newline_and_flush(mut dest: *mut FILE) {
  ::std::ptr::write_volatile(
    &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
    ::std::ptr::read_volatile::<libc::c_int>(
      &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
    ) + 1,
  );
  asm!("" : : : "memory" : "volatile");
  putc_unlocked('\n' as i32, dest);
  fflush(dest);
  int_on();
}
unsafe extern "C" fn out1fmt(mut fmt: *const libc::c_char, mut args: ...) -> libc::c_int {
  let mut ap: ::std::ffi::VaListImpl;
  let mut r: libc::c_int = 0;
  ::std::ptr::write_volatile(
    &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
    ::std::ptr::read_volatile::<libc::c_int>(
      &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
    ) + 1,
  );
  asm!("" : : : "memory" : "volatile");
  ap = args.clone();
  r = vprintf(fmt, ap.as_va_list());
  int_on();
  return r;
}
unsafe extern "C" fn fmtstr(
  mut outbuf: *mut libc::c_char,
  mut length: size_t,
  mut fmt: *const libc::c_char,
  mut args: ...
) -> libc::c_int {
  let mut ap: ::std::ffi::VaListImpl;
  let mut ret: libc::c_int = 0;
  ::std::ptr::write_volatile(
    &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
    ::std::ptr::read_volatile::<libc::c_int>(
      &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
    ) + 1,
  );
  asm!("" : : : "memory" : "volatile");
  ap = args.clone();
  ret = vsnprintf(outbuf, length, fmt, ap.as_va_list());
  int_on();
  return ret;
}
unsafe extern "C" fn out1str(mut p: *const libc::c_char) {
  outstr(p, stdout);
}
unsafe extern "C" fn out2str(mut p: *const libc::c_char) {
  outstr(p, stderr);
  flush_stdout_stderr();
}
/* ${var//pattern/replacement} */
static mut dolatstr: [libc::c_char; 7] = [
  '\u{88}' as i32 as libc::c_uchar as libc::c_char,
  '\u{82}' as i32 as libc::c_uchar as libc::c_char,
  0x1i32 as libc::c_char,
  '@' as i32 as libc::c_char,
  '=' as i32 as libc::c_char,
  '\u{88}' as i32 as libc::c_uchar as libc::c_char,
  '\u{0}' as i32 as libc::c_char,
];
/*
 * Free a parse tree.
 */
unsafe extern "C" fn freefunc(mut f: *mut funcnode) {
  if !f.is_null() && {
    (*f).count -= 1;
    ((*f).count) < 0i32
  } {
    free(f as *mut libc::c_void);
  };
}
static mut basepf: parsefile = parsefile {
  prev: 0 as *const parsefile as *mut parsefile,
  linno: 0,
  pf_fd: 0,
  left_in_line: 0,
  left_in_buffer: 0,
  next_to_pgetc: 0 as *const libc::c_char as *mut libc::c_char,
  buf: 0 as *const libc::c_char as *mut libc::c_char,
  strpush: 0 as *const strpush as *mut strpush,
  basestrpush: strpush {
    prev: 0 as *const strpush as *mut strpush,
    prev_string: 0 as *const libc::c_char as *mut libc::c_char,
    prev_left_in_line: 0,
    ap: 0 as *const alias as *mut alias,
    string: 0 as *const libc::c_char as *mut libc::c_char,
    lastc: [0; 2],
    unget: 0,
  },
  lastc: [0; 2],
  unget: 0,
};
static mut g_parsefile: *mut parsefile = unsafe { &basepf as *const parsefile as *mut parsefile };
static mut commandname: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
unsafe extern "C" fn ash_vmsg(mut msg: *const libc::c_char, mut ap: ::std::ffi::VaList) {
  fprintf(
    stderr,
    b"%s: \x00" as *const u8 as *const libc::c_char,
    (*ash_ptr_to_globals_misc).arg0,
  );
  if !commandname.is_null() {
    if strcmp((*ash_ptr_to_globals_misc).arg0, commandname) != 0 {
      fprintf(
        stderr,
        b"%s: \x00" as *const u8 as *const libc::c_char,
        commandname,
      );
    }
    if (*ash_ptr_to_globals_misc).optlist[3] == 0 || (*g_parsefile).pf_fd > 0i32 {
      fprintf(
        stderr,
        b"line %d: \x00" as *const u8 as *const libc::c_char,
        (*ash_ptr_to_globals_misc).errlinno,
      );
    }
  }
  vfprintf(stderr, msg, ap.as_va_list());
  newline_and_flush(stderr);
}
unsafe extern "C" fn ash_vmsg_and_raise(
  mut cond: libc::c_int,
  mut msg: *const libc::c_char,
  mut ap: ::std::ffi::VaList,
) -> ! {
  ash_vmsg(msg, ap.as_va_list());
  flush_stdout_stderr();
  raise_exception(cond);
}
unsafe extern "C" fn ash_msg_and_raise_error(mut msg: *const libc::c_char, mut args: ...) -> ! {
  let mut ap: ::std::ffi::VaListImpl;
  (*ash_ptr_to_globals_misc).exitstatus = 2i32 as uint8_t;
  ap = args.clone();
  ash_vmsg_and_raise(1i32, msg, ap.as_va_list());
}
unsafe extern "C" fn raise_error_syntax(mut msg: *const libc::c_char) -> ! {
  (*ash_ptr_to_globals_misc).errlinno = (*g_parsefile).linno;
  ash_msg_and_raise_error(
    b"syntax error: %s\x00" as *const u8 as *const libc::c_char,
    msg,
  );
}
unsafe extern "C" fn ash_msg_and_raise(
  mut cond: libc::c_int,
  mut msg: *const libc::c_char,
  mut args: ...
) -> ! {
  let mut ap: ::std::ffi::VaListImpl;
  ap = args.clone();
  ash_vmsg_and_raise(cond, msg, ap.as_va_list());
}
unsafe extern "C" fn ash_msg(mut fmt: *const libc::c_char, mut args: ...) {
  let mut ap: ::std::ffi::VaListImpl;
  ap = args.clone();
  ash_vmsg(fmt, ap.as_va_list());
}
unsafe extern "C" fn errmsg(
  mut e: libc::c_int,
  mut em: *const libc::c_char,
) -> *const libc::c_char {
  if e == 2i32 || e == 20i32 {
    return em;
  }
  return strerror(e);
}
unsafe extern "C" fn stalloc(mut nbytes: size_t) -> *mut libc::c_void {
  let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut aligned: size_t = 0;
  aligned = nbytes.wrapping_add(SHELL_SIZE as libc::c_int as libc::c_ulong)
    & !(SHELL_SIZE as libc::c_int) as libc::c_ulong;
  if aligned > (*ash_ptr_to_globals_memstack).g_stacknleft {
    let mut len: size_t = 0;
    let mut blocksize: size_t = 0;
    let mut sp: *mut stack_block = 0 as *mut stack_block;
    blocksize = aligned;
    if blocksize < MINSIZE as libc::c_int as libc::c_ulong {
      blocksize = MINSIZE as libc::c_int as size_t
    }
    len = (::std::mem::size_of::<stack_block>() as libc::c_ulong)
      .wrapping_sub(MINSIZE as libc::c_int as libc::c_ulong)
      .wrapping_add(blocksize);
    if len < blocksize {
      ash_msg_and_raise_error(bb_msg_memory_exhausted.as_ptr());
    }
    ::std::ptr::write_volatile(
      &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
      ::std::ptr::read_volatile::<libc::c_int>(
        &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
      ) + 1,
    );
    asm!("" : : : "memory" : "volatile");
    sp = xmalloc(len) as *mut stack_block;
    (*sp).prev = (*ash_ptr_to_globals_memstack).g_stackp;
    (*ash_ptr_to_globals_memstack).g_stacknxt = (*sp).space.as_mut_ptr();
    (*ash_ptr_to_globals_memstack).g_stacknleft = blocksize;
    (*ash_ptr_to_globals_memstack).sstrend = (*ash_ptr_to_globals_memstack)
      .g_stacknxt
      .offset(blocksize as isize);
    (*ash_ptr_to_globals_memstack).g_stackp = sp;
    int_on();
  }
  p = (*ash_ptr_to_globals_memstack).g_stacknxt;
  (*ash_ptr_to_globals_memstack).g_stacknxt = (*ash_ptr_to_globals_memstack)
    .g_stacknxt
    .offset(aligned as isize);
  (*ash_ptr_to_globals_memstack).g_stacknleft = ((*ash_ptr_to_globals_memstack).g_stacknleft
    as libc::c_ulong)
    .wrapping_sub(aligned) as size_t as size_t;
  return p as *mut libc::c_void;
}
unsafe extern "C" fn stzalloc(mut nbytes: size_t) -> *mut libc::c_void {
  return memset(stalloc(nbytes), 0i32, nbytes);
}
unsafe extern "C" fn stunalloc(mut p: *mut libc::c_void) {
  (*ash_ptr_to_globals_memstack).g_stacknleft =
    ((*ash_ptr_to_globals_memstack).g_stacknleft as libc::c_ulong).wrapping_add(
      (*ash_ptr_to_globals_memstack)
        .g_stacknxt
        .wrapping_offset_from(p as *mut libc::c_char) as libc::c_long as libc::c_ulong,
    ) as size_t as size_t;
  (*ash_ptr_to_globals_memstack).g_stacknxt = p as *mut libc::c_char;
}
unsafe extern "C" fn sstrdup(mut p: *const libc::c_char) -> *mut libc::c_char {
  let mut len: size_t = strlen(p).wrapping_add(1i32 as libc::c_ulong);
  return memcpy(stalloc(len), p as *const libc::c_void, len) as *mut libc::c_char;
}
#[inline(always)]
unsafe extern "C" fn grabstackblock(mut len: size_t) {
  stalloc(len);
}
unsafe extern "C" fn pushstackmark(mut mark: *mut stackmark, mut len: size_t) {
  (*mark).stackp = (*ash_ptr_to_globals_memstack).g_stackp;
  (*mark).stacknxt = (*ash_ptr_to_globals_memstack).g_stacknxt;
  (*mark).stacknleft = (*ash_ptr_to_globals_memstack).g_stacknleft;
  grabstackblock(len);
}
unsafe extern "C" fn setstackmark(mut mark: *mut stackmark) {
  pushstackmark(
    mark,
    ((*ash_ptr_to_globals_memstack).g_stacknxt
      == (*(*ash_ptr_to_globals_memstack).g_stackp)
        .space
        .as_mut_ptr()
      && (*ash_ptr_to_globals_memstack).g_stackp
        != &mut (*ash_ptr_to_globals_memstack).stackbase as *mut stack_block) as libc::c_int
      as size_t,
  );
}
unsafe extern "C" fn popstackmark(mut mark: *mut stackmark) {
  let mut sp: *mut stack_block = 0 as *mut stack_block;
  if (*mark).stackp.is_null() {
    return;
  }
  ::std::ptr::write_volatile(
    &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
    ::std::ptr::read_volatile::<libc::c_int>(
      &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
    ) + 1,
  );
  asm!("" : : : "memory" : "volatile");
  while (*ash_ptr_to_globals_memstack).g_stackp != (*mark).stackp {
    sp = (*ash_ptr_to_globals_memstack).g_stackp;
    (*ash_ptr_to_globals_memstack).g_stackp = (*sp).prev;
    free(sp as *mut libc::c_void);
  }
  (*ash_ptr_to_globals_memstack).g_stacknxt = (*mark).stacknxt;
  (*ash_ptr_to_globals_memstack).g_stacknleft = (*mark).stacknleft;
  (*ash_ptr_to_globals_memstack).sstrend = (*mark).stacknxt.offset((*mark).stacknleft as isize);
  int_on();
}
unsafe extern "C" fn growstackblock() {
  let mut newlen: size_t = 0;
  newlen = (*ash_ptr_to_globals_memstack)
    .g_stacknleft
    .wrapping_mul(2i32 as libc::c_ulong);
  if newlen < (*ash_ptr_to_globals_memstack).g_stacknleft {
    ash_msg_and_raise_error(bb_msg_memory_exhausted.as_ptr());
  }
  if newlen < 128i32 as libc::c_ulong {
    newlen = (newlen as libc::c_ulong).wrapping_add(128i32 as libc::c_ulong) as size_t as size_t
  }
  if (*ash_ptr_to_globals_memstack).g_stacknxt
    == (*(*ash_ptr_to_globals_memstack).g_stackp)
      .space
      .as_mut_ptr()
    && (*ash_ptr_to_globals_memstack).g_stackp
      != &mut (*ash_ptr_to_globals_memstack).stackbase as *mut stack_block
  {
    let mut sp: *mut stack_block = 0 as *mut stack_block;
    let mut prevstackp: *mut stack_block = 0 as *mut stack_block;
    let mut grosslen: size_t = 0;
    ::std::ptr::write_volatile(
      &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
      ::std::ptr::read_volatile::<libc::c_int>(
        &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
      ) + 1,
    );
    asm!("" : : : "memory" : "volatile");
    sp = (*ash_ptr_to_globals_memstack).g_stackp;
    prevstackp = (*sp).prev;
    grosslen = newlen
      .wrapping_add(::std::mem::size_of::<stack_block>() as libc::c_ulong)
      .wrapping_sub(MINSIZE as libc::c_int as libc::c_ulong);
    sp = xrealloc(sp as *mut libc::c_void, grosslen) as *mut stack_block;
    (*sp).prev = prevstackp;
    (*ash_ptr_to_globals_memstack).g_stackp = sp;
    (*ash_ptr_to_globals_memstack).g_stacknxt = (*sp).space.as_mut_ptr();
    (*ash_ptr_to_globals_memstack).g_stacknleft = newlen;
    (*ash_ptr_to_globals_memstack).sstrend = (*sp).space.as_mut_ptr().offset(newlen as isize);
    int_on();
  } else {
    let mut oldspace: *mut libc::c_char = (*ash_ptr_to_globals_memstack).g_stacknxt;
    let mut oldlen: size_t = (*ash_ptr_to_globals_memstack).g_stacknleft;
    let mut p: *mut libc::c_char = stalloc(newlen) as *mut libc::c_char;
    (*ash_ptr_to_globals_memstack).g_stacknxt = memcpy(
      p as *mut libc::c_void,
      oldspace as *const libc::c_void,
      oldlen,
    ) as *mut libc::c_char;
    (*ash_ptr_to_globals_memstack).g_stacknleft = ((*ash_ptr_to_globals_memstack).g_stacknleft
      as libc::c_ulong)
      .wrapping_add(newlen) as size_t as size_t
  };
}
unsafe extern "C" fn growstackstr() -> *mut libc::c_void {
  let mut len: size_t = (*ash_ptr_to_globals_memstack).g_stacknleft;
  growstackblock();
  return ((*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut libc::c_char)
    .offset(len as isize) as *mut libc::c_void;
}
unsafe extern "C" fn makestrspace(
  mut newlen: size_t,
  mut p: *mut libc::c_char,
) -> *mut libc::c_char {
  let mut len: size_t =
    p.wrapping_offset_from((*ash_ptr_to_globals_memstack).g_stacknxt) as libc::c_long as size_t;
  let mut size: size_t = 0;
  loop {
    let mut nleft: size_t = 0;
    size = (*ash_ptr_to_globals_memstack).g_stacknleft;
    nleft = size.wrapping_sub(len);
    if nleft >= newlen {
      break;
    }
    growstackblock();
  }
  return ((*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut libc::c_char)
    .offset(len as isize);
}
unsafe extern "C" fn stack_nputstr(
  mut s: *const libc::c_char,
  mut n: size_t,
  mut p: *mut libc::c_char,
) -> *mut libc::c_char {
  p = makestrspace(n, p);
  p = mempcpy(p as *mut libc::c_void, s as *const libc::c_void, n) as *mut libc::c_char;
  return p;
}
unsafe extern "C" fn stack_putstr(
  mut s: *const libc::c_char,
  mut p: *mut libc::c_char,
) -> *mut libc::c_char {
  return stack_nputstr(s, strlen(s), p);
}
unsafe extern "C" fn _STPUTC(mut c: libc::c_int, mut p: *mut libc::c_char) -> *mut libc::c_char {
  if p == (*ash_ptr_to_globals_memstack).sstrend {
    p = growstackstr() as *mut libc::c_char
  }
  let fresh1 = p;
  p = p.offset(1);
  *fresh1 = c as libc::c_char;
  return p;
}
unsafe extern "C" fn prefix(
  mut string: *const libc::c_char,
  mut pfx: *const libc::c_char,
) -> *mut libc::c_char {
  while *pfx != 0 {
    let fresh2 = pfx;
    pfx = pfx.offset(1);
    let fresh3 = string;
    string = string.offset(1);
    if *fresh2 as libc::c_int != *fresh3 as libc::c_int {
      return 0 as *mut libc::c_char;
    }
  }
  return string as *mut libc::c_char;
}
unsafe extern "C" fn is_number(mut p: *const libc::c_char) -> libc::c_int {
  loop {
    if !((*p as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32) {
      return 0i32;
    }
    p = p.offset(1);
    if !(*p as libc::c_int != '\u{0}' as i32) {
      break;
    }
  }
  return 1i32;
}
unsafe extern "C" fn number(mut s: *const libc::c_char) -> libc::c_int {
  if is_number(s) == 0 {
    ash_msg_and_raise_error(
      b"Illegal number: %s\x00" as *const u8 as *const libc::c_char,
      s,
    );
  }
  return atoi(s);
}
unsafe extern "C" fn single_quote(mut s: *const libc::c_char) -> *mut libc::c_char {
  let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
  p = (*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut libc::c_char;
  loop {
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    len = strchrnul(s, '\'' as i32).wrapping_offset_from(s) as libc::c_long as size_t;
    p = makestrspace(len.wrapping_add(3i32 as libc::c_ulong), p);
    q = p;
    let fresh4 = q;
    q = q.offset(1);
    *fresh4 = '\'' as i32 as libc::c_char;
    q = mempcpy(q as *mut libc::c_void, s as *const libc::c_void, len) as *mut libc::c_char;
    let fresh5 = q;
    q = q.offset(1);
    *fresh5 = '\'' as i32 as libc::c_char;
    s = s.offset(len as isize);
    p = p.offset(q.wrapping_offset_from(p) as libc::c_long as isize);
    if *s as libc::c_int != '\'' as i32 {
      break;
    }
    len = 0i32 as size_t;
    loop {
      len = len.wrapping_add(1);
      s = s.offset(1);
      if !(*s as libc::c_int == '\'' as i32) {
        break;
      }
    }
    p = makestrspace(len.wrapping_add(3i32 as libc::c_ulong), p);
    q = p;
    let fresh6 = q;
    q = q.offset(1);
    *fresh6 = '\"' as i32 as libc::c_char;
    q = mempcpy(
      q as *mut libc::c_void,
      s.offset(-(len as isize)) as *const libc::c_void,
      len,
    ) as *mut libc::c_char;
    let fresh7 = q;
    q = q.offset(1);
    *fresh7 = '\"' as i32 as libc::c_char;
    p = p.offset(q.wrapping_offset_from(p) as libc::c_long as isize);
    if !(*s != 0) {
      break;
    }
  }
  let fresh8 = p;
  p = p.offset(1);
  *fresh8 = '\u{0}' as i32 as libc::c_char;
  return (*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut libc::c_char;
}
unsafe extern "C" fn maybe_single_quote(mut s: *const libc::c_char) -> *const libc::c_char {
  let mut current_block: u64;
  let mut p: *const libc::c_char = s;
  loop {
    if !(*p != 0) {
      current_block = 1917311967535052937;
      break;
    }
    if (*p as libc::c_int) < '+' as i32 {
      current_block = 9628694359813302936;
      break;
    }
    if *p as libc::c_int >= ';' as i32 && *p as libc::c_int <= '?' as i32 {
      current_block = 9628694359813302936;
      break;
    }
    if *p as libc::c_int == '`' as i32 {
      current_block = 9628694359813302936;
      break;
    }
    if *p as libc::c_int == '[' as i32 {
      current_block = 9628694359813302936;
      break;
    }
    if *p as libc::c_int == '\\' as i32 {
      current_block = 9628694359813302936;
      break;
    }
    if *p as libc::c_int > 'z' as i32 {
      current_block = 9628694359813302936;
      break;
    }
    p = p.offset(1)
  }
  match current_block {
    9628694359813302936 => return single_quote(s),
    _ => return s,
  };
}
static mut argptr: *mut *mut libc::c_char = 0 as *const *mut libc::c_char as *mut *mut libc::c_char;
static mut optionarg: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut optptr: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
unsafe extern "C" fn nextopt(mut optstring: *const libc::c_char) -> libc::c_int {
  let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut q: *const libc::c_char = 0 as *const libc::c_char;
  let mut c: libc::c_char = 0;
  p = optptr;
  if p.is_null() || *p as libc::c_int == '\u{0}' as i32 {
    p = *argptr;
    if p.is_null() {
      return '\u{0}' as i32;
    }
    if *p as libc::c_int != '-' as i32 {
      return '\u{0}' as i32;
    }
    p = p.offset(1);
    if *p as libc::c_int == '\u{0}' as i32 {
      return '\u{0}' as i32;
    }
    argptr = argptr.offset(1);
    if *p.offset(0) as libc::c_int == '-' as i32 && *p.offset(1) == 0 {
      return '\u{0}' as i32;
    }
  }
  let fresh9 = p;
  p = p.offset(1);
  c = *fresh9;
  q = optstring;
  while *q as libc::c_int != c as libc::c_int {
    if *q as libc::c_int == '\u{0}' as i32 {
      ash_msg_and_raise_error(
        b"illegal option -%c\x00" as *const u8 as *const libc::c_char,
        c as libc::c_int,
      );
    }
    q = q.offset(1);
    if *q as libc::c_int == ':' as i32 {
      q = q.offset(1)
    }
  }
  q = q.offset(1);
  if *q as libc::c_int == ':' as i32 {
    if *p as libc::c_int == '\u{0}' as i32 {
      let fresh10 = argptr;
      argptr = argptr.offset(1);
      p = *fresh10;
      if p.is_null() {
        ash_msg_and_raise_error(
          b"no arg for -%c option\x00" as *const u8 as *const libc::c_char,
          c as libc::c_int,
        );
      }
    }
    optionarg = p;
    p = 0 as *mut libc::c_char
  }
  optptr = p;
  return c as libc::c_int;
}
unsafe extern "C" fn freeparam(mut param: *mut shparam) {
  if (*param).malloced != 0 {
    let mut ap: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut ap1: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    ap1 = (*param).p;
    ap = ap1;
    while !(*ap).is_null() {
      let fresh11 = ap;
      ap = ap.offset(1);
      free(*fresh11 as *mut libc::c_void);
    }
    free(ap1 as *mut libc::c_void);
  };
}
static mut varinit_data: [C2RustUnnamed_11; 13] = unsafe {
  [
    {
      let mut init = C2RustUnnamed_11 {
        flags: 0x4i32 | 0x8i32,
        var_text: defifsvar.as_ptr(),
        var_func: None,
      };
      init
    },
    {
      let mut init = C2RustUnnamed_11 {
        flags: 0x4i32 | 0x8i32 | 0x20i32,
        var_text: b"MAIL\x00" as *const u8 as *const libc::c_char,
        var_func: Some(changemail as unsafe extern "C" fn(_: *const libc::c_char) -> ()),
      };
      init
    },
    {
      let mut init = C2RustUnnamed_11 {
        flags: 0x4i32 | 0x8i32 | 0x20i32,
        var_text: b"MAILPATH\x00" as *const u8 as *const libc::c_char,
        var_func: Some(changemail as unsafe extern "C" fn(_: *const libc::c_char) -> ()),
      };
      init
    },
    {
      let mut init = C2RustUnnamed_11 {
        flags: 0x4i32 | 0x8i32,
        var_text: bb_PATH_root_path.as_ptr(),
        var_func: Some(changepath as unsafe extern "C" fn(_: *const libc::c_char) -> ()),
      };
      init
    },
    {
      let mut init = C2RustUnnamed_11 {
        flags: 0x4i32 | 0x8i32,
        var_text: b"PS1=$ \x00" as *const u8 as *const libc::c_char,
        var_func: None,
      };
      init
    },
    {
      let mut init = C2RustUnnamed_11 {
        flags: 0x4i32 | 0x8i32,
        var_text: b"PS2=> \x00" as *const u8 as *const libc::c_char,
        var_func: None,
      };
      init
    },
    {
      let mut init = C2RustUnnamed_11 {
        flags: 0x4i32 | 0x8i32,
        var_text: b"PS4=+ \x00" as *const u8 as *const libc::c_char,
        var_func: None,
      };
      init
    },
    {
      let mut init = C2RustUnnamed_11 {
        flags: 0x4i32 | 0x8i32,
        var_text: defoptindvar.as_ptr(),
        var_func: Some(getoptsreset as unsafe extern "C" fn(_: *const libc::c_char) -> ()),
      };
      init
    },
    {
      let mut init = C2RustUnnamed_11 {
        flags: 0x4i32 | 0x8i32,
        var_text: 0 as *const libc::c_char,
        var_func: None,
      };
      init
    },
    {
      let mut init = C2RustUnnamed_11 {
        flags: 0x4i32 | 0x8i32 | 0x20i32 | 0x200i32,
        var_text: b"RANDOM\x00" as *const u8 as *const libc::c_char,
        var_func: Some(change_random as unsafe extern "C" fn(_: *const libc::c_char) -> ()),
      };
      init
    },
    {
      let mut init = C2RustUnnamed_11 {
        flags: 0x4i32 | 0x8i32 | 0x20i32 | 0x200i32,
        var_text: b"EPOCHSECONDS\x00" as *const u8 as *const libc::c_char,
        var_func: Some(change_seconds as unsafe extern "C" fn(_: *const libc::c_char) -> ()),
      };
      init
    },
    {
      let mut init = C2RustUnnamed_11 {
        flags: 0x4i32 | 0x8i32 | 0x20i32 | 0x200i32,
        var_text: b"EPOCHREALTIME\x00" as *const u8 as *const libc::c_char,
        var_func: Some(change_realtime as unsafe extern "C" fn(_: *const libc::c_char) -> ()),
      };
      init
    },
    {
      let mut init = C2RustUnnamed_11 {
        flags: 0x4i32 | 0x8i32 | 0x20i32,
        var_text: b"HISTFILE\x00" as *const u8 as *const libc::c_char,
        var_func: None,
      };
      init
    },
  ]
};
unsafe extern "C" fn getoptsreset(mut value: *const libc::c_char) {
  (*ash_ptr_to_globals_var).shellparam.optind = 1i32;
  if is_number(value) != 0 {
    let ref mut fresh12 = number(value);
    (*ash_ptr_to_globals_var).shellparam.optind = if *fresh12 != 0 { *fresh12 } else { 1i32 }
  }
  (*ash_ptr_to_globals_var).shellparam.optoff = -1i32;
}
unsafe extern "C" fn varcmp(mut p: *const libc::c_char, mut q: *const libc::c_char) -> libc::c_int {
  let mut current_block: u64;
  let mut c: libc::c_int = 0;
  let mut d: libc::c_int = 0;
  loop {
    c = *p as libc::c_int;
    d = *q as libc::c_int;
    if !(c == d) {
      current_block = 820271813250567934;
      break;
    }
    if c == '\u{0}' as i32 || c == '=' as i32 {
      current_block = 5491243148067017140;
      break;
    }
    p = p.offset(1);
    q = q.offset(1)
  }
  match current_block {
    820271813250567934 => {
      if c == '=' as i32 {
        c = '\u{0}' as i32
      }
      if d == '=' as i32 {
        d = '\u{0}' as i32
      }
    }
    _ => {}
  }
  return c - d;
}
unsafe extern "C" fn hashvar(mut p: *const libc::c_char) -> *mut *mut var {
  let mut hashval: libc::c_uint = 0;
  hashval = ((*p as libc::c_uchar as libc::c_int) << 4i32) as libc::c_uint;
  while *p as libc::c_int != 0 && *p as libc::c_int != '=' as i32 {
    let fresh13 = p;
    p = p.offset(1);
    hashval = hashval.wrapping_add(*fresh13 as libc::c_uchar as libc::c_uint)
  }
  return &mut *(*ash_ptr_to_globals_var)
    .vartab
    .as_mut_ptr()
    .offset(hashval.wrapping_rem(39i32 as libc::c_uint) as isize) as *mut *mut var;
}
unsafe extern "C" fn vpcmp(mut a: *const libc::c_void, mut b: *const libc::c_void) -> libc::c_int {
  return varcmp(
    *(a as *mut *const libc::c_char),
    *(b as *mut *const libc::c_char),
  );
}
unsafe extern "C" fn initvar() {
  let mut vp: *mut var = 0 as *mut var;
  let mut end: *mut var = 0 as *mut var;
  let mut vpp: *mut *mut var = 0 as *mut *mut var;
  (*ash_ptr_to_globals_var).varinit[(1i32 * 2i32 + 2i32) as usize].var_text =
    b"PS1=\\w \\$ \x00" as *const u8 as *const libc::c_char;
  vp = (*ash_ptr_to_globals_var).varinit.as_mut_ptr();
  end = vp.offset(
    (::std::mem::size_of::<[var; 13]>() as libc::c_ulong)
      .wrapping_div(::std::mem::size_of::<var>() as libc::c_ulong) as libc::c_uint as isize,
  );
  loop {
    vpp = hashvar((*vp).var_text);
    (*vp).next = *vpp;
    *vpp = vp;
    vp = vp.offset(1);
    if !(vp < end) {
      break;
    }
  }
}
unsafe extern "C" fn findvar(
  mut vpp: *mut *mut var,
  mut name: *const libc::c_char,
) -> *mut *mut var {
  while !(*vpp).is_null() {
    if varcmp((**vpp).var_text, name) == 0i32 {
      break;
    }
    vpp = &mut (**vpp).next
  }
  return vpp;
}
unsafe extern "C" fn lookupvar(mut name: *const libc::c_char) -> *const libc::c_char {
  let mut v: *mut var = 0 as *mut var;
  v = *findvar(hashvar(name), name);
  if !v.is_null() {
    if (*v).flags & 0x200i32 != 0 {
      (*v).var_func.expect("non-null function pointer")(0 as *const libc::c_char);
    }
    if (*v).flags & 0x20i32 == 0 {
      if v
        == &mut *(*ash_ptr_to_globals_var)
          .varinit
          .as_mut_ptr()
          .offset((1i32 * 2i32 + 1i32 + 5i32) as isize) as *mut var
        && (*v).var_text == (*ash_ptr_to_globals_var).linenovar.as_mut_ptr()
      {
        fmtstr(
          (*ash_ptr_to_globals_var).linenovar.as_mut_ptr().offset(7),
          (::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong)
            .wrapping_sub(7i32 as libc::c_ulong),
          b"%d\x00" as *const u8 as *const libc::c_char,
          (*ash_ptr_to_globals_var).lineno,
        );
      }
      return var_end((*v).var_text);
    }
  }
  return 0 as *const libc::c_char;
}
unsafe extern "C" fn reinit_unicode_for_ash() {
  if 0i32 != 0 || 0i32 != 0 {
    let mut s: *const libc::c_char = lookupvar(b"LC_ALL\x00" as *const u8 as *const libc::c_char);
    if s.is_null() {
      s = lookupvar(b"LC_CTYPE\x00" as *const u8 as *const libc::c_char)
    }
    if s.is_null() {
      s = lookupvar(b"LANG\x00" as *const u8 as *const libc::c_char)
    }
  };
}
#[inline(always)]
unsafe extern "C" fn bltinlookup(mut name: *const libc::c_char) -> *const libc::c_char {
  return lookupvar(name);
}
unsafe extern "C" fn setvareq(mut s: *mut libc::c_char, mut flags: libc::c_int) -> *mut var {
  let mut current_block: u64;
  let mut vp: *mut var = 0 as *mut var;
  let mut vpp: *mut *mut var = 0 as *mut *mut var;
  vpp = hashvar(s);
  flags = (flags as libc::c_uint
    | 0x1i32 as libc::c_uint
      & ((1i32 - (*ash_ptr_to_globals_misc).optlist[11] as libc::c_int) as libc::c_uint)
        .wrapping_sub(1i32 as libc::c_uint)) as libc::c_int;
  vpp = findvar(vpp, s);
  vp = *vpp;
  if !vp.is_null() {
    if (*vp).flags & (0x2i32 | 0x200i32) == 0x2i32 {
      let mut n: *const libc::c_char = 0 as *const libc::c_char;
      if flags & 0x100i32 != 0 {
        free(s as *mut libc::c_void);
      }
      n = (*vp).var_text;
      (*ash_ptr_to_globals_misc).exitstatus = 1i32 as uint8_t;
      ash_msg_and_raise_error(
        b"%.*s: is read only\x00" as *const u8 as *const libc::c_char,
        strchrnul(n, '=' as i32).wrapping_offset_from(n) as libc::c_long,
        n,
      );
    }
    if flags & 0x80i32 != 0 {
      current_block = 14939057063304287752;
    } else {
      if (*vp).var_func.is_some() && flags & 0x40i32 == 0 {
        (*vp).var_func.expect("non-null function pointer")(var_end(s));
      }
      if (*vp).flags & (0x8i32 | 0x10i32) == 0 {
        free((*vp).var_text as *mut libc::c_char as *mut libc::c_void);
      }
      if flags & (0x1i32 | 0x2i32 | 0x4i32 | 0x20i32) | (*vp).flags & 0x4i32 == 0x20i32 {
        *vpp = (*vp).next;
        free(vp as *mut libc::c_void);
        current_block = 12664043378242067457;
      } else {
        flags |= (*vp).flags & !(0x8i32 | 0x10i32 | 0x100i32 | 0x20i32);
        if flags & 0x20i32 != 0 {
          flags &= !0x200i32
        }
        current_block = 11932355480408055363;
      }
    }
  } else if flags & 0x80i32 != 0 {
    current_block = 14939057063304287752;
  } else if flags & (0x1i32 | 0x2i32 | 0x4i32 | 0x20i32) == 0x20i32 {
    current_block = 12664043378242067457;
  } else {
    vp = xzalloc(::std::mem::size_of::<var>() as libc::c_ulong) as *mut var;
    (*vp).next = *vpp;
    *vpp = vp;
    current_block = 11932355480408055363;
  }
  match current_block {
    11932355480408055363 => {
      if flags & (0x8i32 | 0x10i32 | 0x100i32) == 0 {
        s = xstrdup(s)
      }
      (*vp).var_text = s;
      (*vp).flags = flags
    }
    12664043378242067457 => {
      if flags & (0x8i32 | 0x10i32 | 0x100i32) == 0x100i32 {
        free(s as *mut libc::c_void);
      }
    }
    _ => {}
  }
  return vp;
}
unsafe extern "C" fn setvar(
  mut name: *const libc::c_char,
  mut val: *const libc::c_char,
  mut flags: libc::c_int,
) -> *mut var {
  let mut q: *const libc::c_char = 0 as *const libc::c_char;
  let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut nameeq: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut namelen: size_t = 0;
  let mut vallen: size_t = 0;
  let mut vp: *mut var = 0 as *mut var;
  q = endofname(name);
  p = strchrnul(q, '=' as i32);
  namelen = p.wrapping_offset_from(name) as libc::c_long as size_t;
  if namelen == 0 || p != q as *mut libc::c_char {
    ash_msg_and_raise_error(
      b"%.*s: bad variable name\x00" as *const u8 as *const libc::c_char,
      namelen,
      name,
    );
  }
  vallen = 0i32 as size_t;
  if val.is_null() {
    flags |= 0x20i32
  } else {
    vallen = strlen(val)
  }
  ::std::ptr::write_volatile(
    &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
    ::std::ptr::read_volatile::<libc::c_int>(
      &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
    ) + 1,
  );
  asm!("" : : : "memory" : "volatile");
  nameeq = xzalloc(
    namelen
      .wrapping_add(vallen)
      .wrapping_add(2i32 as libc::c_ulong),
  ) as *mut libc::c_char;
  p = mempcpy(
    nameeq as *mut libc::c_void,
    name as *const libc::c_void,
    namelen,
  ) as *mut libc::c_char;
  if !val.is_null() {
    let fresh14 = p;
    p = p.offset(1);
    *fresh14 = '=' as i32 as libc::c_char;
    memcpy(p as *mut libc::c_void, val as *const libc::c_void, vallen);
  }
  vp = setvareq(nameeq, flags | 0x100i32);
  int_on();
  return vp;
}
unsafe extern "C" fn setvar0(mut name: *const libc::c_char, mut val: *const libc::c_char) {
  setvar(name, val, 0i32);
}
unsafe extern "C" fn unsetvar(mut s: *const libc::c_char) {
  setvar(s, 0 as *const libc::c_char, 0i32);
}
unsafe extern "C" fn listsetvar(mut list_set_var: *mut strlist, mut flags: libc::c_int) {
  let mut lp: *mut strlist = list_set_var;
  if lp.is_null() {
    return;
  }
  ::std::ptr::write_volatile(
    &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
    ::std::ptr::read_volatile::<libc::c_int>(
      &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
    ) + 1,
  );
  asm!("" : : : "memory" : "volatile");
  loop {
    setvareq((*lp).text, flags);
    lp = (*lp).next;
    if lp.is_null() {
      break;
    }
  }
  int_on();
}
unsafe extern "C" fn listvars(
  mut on: libc::c_int,
  mut off: libc::c_int,
  mut end: *mut *mut *mut libc::c_char,
) -> *mut *mut libc::c_char {
  let mut vpp: *mut *mut var = 0 as *mut *mut var;
  let mut vp: *mut var = 0 as *mut var;
  let mut ep: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  let mut mask: libc::c_int = 0;
  ep = (*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut *mut libc::c_char;
  vpp = (*ash_ptr_to_globals_var).vartab.as_mut_ptr();
  mask = on | off;
  loop {
    vp = *vpp;
    while !vp.is_null() {
      if (*vp).flags & mask == on {
        if ep
          == (*ash_ptr_to_globals_memstack).sstrend as *mut libc::c_void as *mut *mut libc::c_char
        {
          ep = growstackstr() as *mut *mut libc::c_char
        }
        let fresh15 = ep;
        ep = ep.offset(1);
        *fresh15 = (*vp).var_text as *mut libc::c_char
      }
      vp = (*vp).next
    }
    vpp = vpp.offset(1);
    if !(vpp < (*ash_ptr_to_globals_var).vartab.as_mut_ptr().offset(39)) {
      break;
    }
  }
  if ep == (*ash_ptr_to_globals_memstack).sstrend as *mut libc::c_void as *mut *mut libc::c_char {
    ep = growstackstr() as *mut *mut libc::c_char
  }
  if !end.is_null() {
    *end = ep
  }
  let fresh16 = ep;
  ep = ep.offset(1);
  *fresh16 = 0 as *mut libc::c_char;
  return stalloc((ep as *mut libc::c_char).wrapping_offset_from(
    (*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut libc::c_char,
  ) as libc::c_long as size_t) as *mut *mut libc::c_char;
}
static mut pathopt: *const libc::c_char = 0 as *const libc::c_char;
unsafe extern "C" fn path_advance(
  mut path: *mut *const libc::c_char,
  mut name: *const libc::c_char,
) -> *mut libc::c_char {
  let mut p: *const libc::c_char = 0 as *const libc::c_char;
  let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut start: *const libc::c_char = 0 as *const libc::c_char;
  let mut len: size_t = 0;
  if (*path).is_null() {
    return 0 as *mut libc::c_char;
  }
  start = *path;
  p = start;
  while *p as libc::c_int != 0 && *p as libc::c_int != ':' as i32 && *p as libc::c_int != '%' as i32
  {
    p = p.offset(1)
  }
  len = (p.wrapping_offset_from(start) as libc::c_long as libc::c_ulong)
    .wrapping_add(strlen(name))
    .wrapping_add(2i32 as libc::c_ulong);
  while (*ash_ptr_to_globals_memstack).g_stacknleft < len {
    growstackblock();
  }
  q = (*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut libc::c_char;
  if p != start {
    q = mempcpy(
      q as *mut libc::c_void,
      start as *const libc::c_void,
      p.wrapping_offset_from(start) as libc::c_long as size_t,
    ) as *mut libc::c_char;
    let fresh17 = q;
    q = q.offset(1);
    *fresh17 = '/' as i32 as libc::c_char
  }
  strcpy(q, name);
  pathopt = 0 as *const libc::c_char;
  if *p as libc::c_int == '%' as i32 {
    p = p.offset(1);
    pathopt = p;
    while *p as libc::c_int != 0 && *p as libc::c_int != ':' as i32 {
      p = p.offset(1)
    }
  }
  if *p as libc::c_int == ':' as i32 {
    *path = p.offset(1)
  } else {
    *path = 0 as *const libc::c_char
  }
  return stalloc(len) as *mut libc::c_char;
}
static mut doprompt: smallint = 0;
static mut needprompt: smallint = 0;
static mut line_input_state: *mut line_input_t = 0 as *const line_input_t as *mut line_input_t;
static mut cmdedit_prompt: *const libc::c_char = 0 as *const libc::c_char;
unsafe extern "C" fn putprompt(mut s: *const libc::c_char) {
  free(cmdedit_prompt as *mut libc::c_char as *mut libc::c_void);
  cmdedit_prompt = xstrdup(s);
}
unsafe extern "C" fn setprompt_if(mut do_set: smallint, mut whichprompt: libc::c_int) {
  let mut prompt: *const libc::c_char = 0 as *const libc::c_char;
  let mut smark: stackmark = stackmark {
    stackp: 0 as *mut stack_block,
    stacknxt: 0 as *mut libc::c_char,
    stacknleft: 0,
  };
  if do_set == 0 {
    return;
  }
  needprompt = 0i32 as smallint;
  match whichprompt {
    1 => {
      prompt = (*ash_ptr_to_globals_var).varinit[(1i32 * 2i32 + 2i32) as usize]
        .var_text
        .offset(4)
    }
    2 => {
      prompt = (*ash_ptr_to_globals_var).varinit[(1i32 * 2i32 + 3i32) as usize]
        .var_text
        .offset(4)
    }
    _ => prompt = (*ash_ptr_to_globals_misc).nullstr.as_mut_ptr(),
  }
  pushstackmark(&mut smark, (*ash_ptr_to_globals_memstack).g_stacknleft);
  putprompt(expandstr(prompt, 4i32));
  popstackmark(&mut smark);
}
unsafe extern "C" fn cdopt() -> libc::c_int {
  let mut flags: libc::c_int = 0i32;
  let mut i: libc::c_int = 0;
  let mut j: libc::c_int = 0;
  j = 'L' as i32;
  loop {
    i = nextopt(b"LP\x00" as *const u8 as *const libc::c_char);
    if !(i != '\u{0}' as i32) {
      break;
    }
    if i != j {
      flags ^= 1i32;
      j = i
    }
  }
  return flags;
}
unsafe extern "C" fn updatepwd(mut dir: *const libc::c_char) -> *const libc::c_char {
  let mut new: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut cdcomppath: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut lim: *const libc::c_char = 0 as *const libc::c_char;
  cdcomppath = sstrdup(dir);
  new = (*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut libc::c_char;
  if *dir as libc::c_int != '/' as i32 {
    if (*ash_ptr_to_globals_misc).curdir == (*ash_ptr_to_globals_misc).nullstr.as_mut_ptr() {
      return 0 as *const libc::c_char;
    }
    new = stack_putstr((*ash_ptr_to_globals_misc).curdir, new)
  }
  new = makestrspace(strlen(dir).wrapping_add(2i32 as libc::c_ulong), new);
  lim =
    ((*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut libc::c_char).offset(1);
  if *dir as libc::c_int != '/' as i32 {
    if *new.offset(-1i32 as isize) as libc::c_int != '/' as i32 {
      let fresh18 = new;
      new = new.offset(1);
      *fresh18 = '/' as i32 as libc::c_char
    }
    if new > lim as *mut libc::c_char && *lim as libc::c_int == '/' as i32 {
      lim = lim.offset(1)
    }
  } else {
    let fresh19 = new;
    new = new.offset(1);
    *fresh19 = '/' as i32 as libc::c_char;
    cdcomppath = cdcomppath.offset(1);
    if *dir.offset(1) as libc::c_int == '/' as i32 && *dir.offset(2) as libc::c_int != '/' as i32 {
      let fresh20 = new;
      new = new.offset(1);
      *fresh20 = '/' as i32 as libc::c_char;
      cdcomppath = cdcomppath.offset(1);
      lim = lim.offset(1)
    }
  }
  p = strtok(cdcomppath, b"/\x00" as *const u8 as *const libc::c_char);
  while !p.is_null() {
    let mut current_block_27: u64;
    match *p as libc::c_int {
      46 => {
        if *p.offset(1) as libc::c_int == '.' as i32
          && *p.offset(2) as libc::c_int == '\u{0}' as i32
        {
          while new > lim as *mut libc::c_char {
            new = new.offset(-1);
            if *new.offset(-1i32 as isize) as libc::c_int == '/' as i32 {
              break;
            }
          }
          current_block_27 = 7333393191927787629;
        } else if *p.offset(1) as libc::c_int == '\u{0}' as i32 {
          current_block_27 = 7333393191927787629;
        } else {
          current_block_27 = 4144751698044322783;
        }
      }
      _ => {
        current_block_27 = 4144751698044322783;
      }
    }
    match current_block_27 {
      4144751698044322783 => {
        new = stack_putstr(p, new);
        let fresh21 = new;
        new = new.offset(1);
        *fresh21 = '/' as i32 as libc::c_char
      }
      _ => {}
    }
    p = strtok(
      0 as *mut libc::c_char,
      b"/\x00" as *const u8 as *const libc::c_char,
    )
  }
  if new > lim as *mut libc::c_char {
    new = new.offset(-1)
  }
  *new = 0i32 as libc::c_char;
  return (*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *const libc::c_char;
}
unsafe extern "C" fn getpwd() -> *mut libc::c_char {
  let mut dir: *mut libc::c_char = getcwd(0 as *mut libc::c_char, 0i32 as size_t);
  return if !dir.is_null() {
    dir
  } else {
    (*ash_ptr_to_globals_misc).nullstr.as_mut_ptr()
  };
}
unsafe extern "C" fn setpwd(mut val: *const libc::c_char, mut setold: libc::c_int) {
  let mut oldcur: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut dir: *mut libc::c_char = 0 as *mut libc::c_char;
  dir = (*ash_ptr_to_globals_misc).curdir;
  oldcur = dir;
  if setold != 0 {
    setvar(
      b"OLDPWD\x00" as *const u8 as *const libc::c_char,
      oldcur,
      0x1i32,
    );
  }
  ::std::ptr::write_volatile(
    &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
    ::std::ptr::read_volatile::<libc::c_int>(
      &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
    ) + 1,
  );
  asm!("" : : : "memory" : "volatile");
  if (*ash_ptr_to_globals_misc).physdir != (*ash_ptr_to_globals_misc).nullstr.as_mut_ptr() {
    if (*ash_ptr_to_globals_misc).physdir != oldcur {
      free((*ash_ptr_to_globals_misc).physdir as *mut libc::c_void);
    }
    (*ash_ptr_to_globals_misc).physdir = (*ash_ptr_to_globals_misc).nullstr.as_mut_ptr()
  }
  if oldcur == val as *mut libc::c_char || val.is_null() {
    let mut s: *mut libc::c_char = getpwd();
    (*ash_ptr_to_globals_misc).physdir = s;
    if val.is_null() {
      dir = s
    }
  } else {
    dir = xstrdup(val)
  }
  if oldcur != dir && oldcur != (*ash_ptr_to_globals_misc).nullstr.as_mut_ptr() {
    free(oldcur as *mut libc::c_void);
  }
  (*ash_ptr_to_globals_misc).curdir = dir;
  int_on();
  setvar(b"PWD\x00" as *const u8 as *const libc::c_char, dir, 0x1i32);
}
unsafe extern "C" fn docd(mut dest: *const libc::c_char, mut flags: libc::c_int) -> libc::c_int {
  let mut dir: *const libc::c_char = 0 as *const libc::c_char;
  let mut err: libc::c_int = 0;
  ::std::ptr::write_volatile(
    &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
    ::std::ptr::read_volatile::<libc::c_int>(
      &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
    ) + 1,
  );
  asm!("" : : : "memory" : "volatile");
  if flags & 1i32 == 0 {
    dir = updatepwd(dest);
    if !dir.is_null() {
      dest = dir
    }
  }
  err = chdir(dest);
  if !(err != 0) {
    setpwd(dir, 1i32);
    hashcd();
  }
  int_on();
  return err;
}
unsafe extern "C" fn cdcmd(mut _argc: libc::c_int, mut _argv: *mut *mut libc::c_char) -> libc::c_int {
  let mut current_block: u64;
  let mut dest: *const libc::c_char = 0 as *const libc::c_char;
  let mut path: *const libc::c_char = 0 as *const libc::c_char;
  let mut p: *const libc::c_char = 0 as *const libc::c_char;
  let mut c: libc::c_char = 0;
  let mut statb: stat = stat {
    st_dev: 0,
    st_ino: 0,
    st_nlink: 0,
    st_mode: 0,
    st_uid: 0,
    st_gid: 0,
    __pad0: 0,
    st_rdev: 0,
    st_size: 0,
    st_blksize: 0,
    st_blocks: 0,
    st_atim: timespec {
      tv_sec: 0,
      tv_nsec: 0,
    },
    st_mtim: timespec {
      tv_sec: 0,
      tv_nsec: 0,
    },
    st_ctim: timespec {
      tv_sec: 0,
      tv_nsec: 0,
    },
    __glibc_reserved: [0; 3],
  };
  let mut flags: libc::c_int = 0;
  flags = cdopt();
  dest = *argptr;
  if dest.is_null() {
    dest = bltinlookup(b"HOME\x00" as *const u8 as *const libc::c_char)
  } else if *dest.offset(0) as libc::c_int == '-' as i32 && *dest.offset(1) == 0 {
    dest = bltinlookup(b"OLDPWD\x00" as *const u8 as *const libc::c_char);
    flags |= 2i32
  }
  if dest.is_null() {
    dest = (*ash_ptr_to_globals_misc).nullstr.as_mut_ptr()
  }
  if *dest as libc::c_int == '/' as i32 {
    current_block = 15152346946622375049;
  } else {
    if *dest as libc::c_int == '.' as i32 {
      c = *dest.offset(1);
      loop {
        match c as libc::c_int {
          0 | 47 => {
            current_block = 15152346946622375049;
            break;
          }
          46 => {}
          _ => {
            current_block = 224731115979188411;
            break;
          }
        }
        c = *dest.offset(2);
        if !(c as libc::c_int != '.' as i32) {
          current_block = 224731115979188411;
          break;
        }
      }
    } else {
      current_block = 224731115979188411;
    }
    match current_block {
      15152346946622375049 => {}
      _ => {
        if *dest == 0 {
          dest = b".\x00" as *const u8 as *const libc::c_char
        }
        path = bltinlookup(b"CDPATH\x00" as *const u8 as *const libc::c_char);
        loop {
          if path.is_null() {
            current_block = 15152346946622375049;
            break;
          }
          c = *path;
          p = path_advance(&mut path, dest);
          if !(stat(p, &mut statb) >= 0i32
            && statb.st_mode & 0o170000i32 as libc::c_uint == 0o40000i32 as libc::c_uint)
          {
            continue;
          }
          if c as libc::c_int != 0 && c as libc::c_int != ':' as i32 {
            flags |= 2i32
          }
          current_block = 13197260612278151808;
          break;
        }
      }
    }
  }
  match current_block {
    15152346946622375049 => p = dest,
    _ => {}
  }
  if docd(p, flags) == 0 {
    if flags & 2i32 != 0 {
      out1fmt(
        b"%s\n\x00" as *const u8 as *const libc::c_char,
        (*ash_ptr_to_globals_misc).curdir,
      );
    }
    return 0i32;
  } else {
    ash_msg_and_raise_error(
      b"can\'t cd to %s: %m\x00" as *const u8 as *const libc::c_char,
      dest,
    );
  };
}
unsafe extern "C" fn pwdcmd(
  mut _argc: libc::c_int,
  mut _argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut flags: libc::c_int = 0;
  let mut dir: *const libc::c_char = (*ash_ptr_to_globals_misc).curdir;
  flags = cdopt();
  if flags != 0 {
    if (*ash_ptr_to_globals_misc).physdir == (*ash_ptr_to_globals_misc).nullstr.as_mut_ptr() {
      setpwd(dir, 0i32);
    }
    dir = (*ash_ptr_to_globals_misc).physdir
  }
  out1fmt(b"%s\n\x00" as *const u8 as *const libc::c_char, dir);
  return 0i32;
}
static mut S_I_T: [uint16_t; 12] = [
  (13i32 | 14i32 << 4i32 | 14i32 << 8i32 | 14i32 << 12i32) as uint16_t,
  (13i32 | 0i32 << 4i32 | 0i32 << 8i32 | 0i32 << 12i32) as uint16_t,
  (1i32 | 1i32 << 4i32 | 1i32 << 8i32 | 1i32 << 12i32) as uint16_t,
  (0i32 | 12i32 << 4i32 | 12i32 << 8i32 | 0i32 << 12i32) as uint16_t,
  (4i32 | 5i32 << 4i32 | 0i32 << 8i32 | 0i32 << 12i32) as uint16_t,
  (7i32 | 7i32 << 4i32 | 0i32 << 8i32 | 7i32 << 12i32) as uint16_t,
  (3i32 | 0i32 << 4i32 | 5i32 << 8i32 | 0i32 << 12i32) as uint16_t,
  (13i32 | 0i32 << 4i32 | 0i32 << 8i32 | 9i32 << 12i32) as uint16_t,
  (13i32 | 0i32 << 4i32 | 0i32 << 8i32 | 10i32 << 12i32) as uint16_t,
  (2i32 | 2i32 << 4i32 | 12i32 << 8i32 | 2i32 << 12i32) as uint16_t,
  (6i32 | 6i32 << 4i32 | 0i32 << 8i32 | 6i32 << 12i32) as uint16_t,
  (8i32 | 8i32 << 4i32 | 0i32 << 8i32 | 8i32 << 12i32) as uint16_t,
];
unsafe extern "C" fn SIT(mut c: libc::c_int, mut syntax: libc::c_int) -> libc::c_int {
  static mut spec_symbls: [libc::c_char; 26] = [
    9, 10, 32, 33, 34, 36, 38, 39, 40, 41, 42, 45, 58, 59, 60, 61, 62, 63, 91, 92, 93, 96, 124,
    125, 126, 0,
  ];
  static mut syntax_index_table: [uint8_t; 25] = [
    1i32 as uint8_t,
    2i32 as uint8_t,
    1i32 as uint8_t,
    3i32 as uint8_t,
    4i32 as uint8_t,
    5i32 as uint8_t,
    1i32 as uint8_t,
    6i32 as uint8_t,
    7i32 as uint8_t,
    8i32 as uint8_t,
    3i32 as uint8_t,
    3i32 as uint8_t,
    3i32 as uint8_t,
    1i32 as uint8_t,
    1i32 as uint8_t,
    3i32 as uint8_t,
    1i32 as uint8_t,
    3i32 as uint8_t,
    3i32 as uint8_t,
    9i32 as uint8_t,
    3i32 as uint8_t,
    10i32 as uint8_t,
    1i32 as uint8_t,
    11i32 as uint8_t,
    3i32 as uint8_t,
  ];
  let mut s: *const libc::c_char = 0 as *const libc::c_char;
  let mut indx: libc::c_int = 0;
  if c == 256i32 {
    return 11i32;
  }
  if c == 257i32 {
    indx = 0i32
  } else {
    if c as libc::c_uchar as libc::c_int >= '\u{81}' as i32 as libc::c_uchar as libc::c_int
      && c as libc::c_uchar as libc::c_int <= '\u{88}' as i32 as libc::c_uchar as libc::c_int
    {
      return 12i32;
    }
    s = strchrnul(spec_symbls.as_ptr(), c);
    if *s as libc::c_int == '\u{0}' as i32 {
      return 0i32;
    }
    indx = syntax_index_table[s.wrapping_offset_from(spec_symbls.as_ptr()) as libc::c_long as usize]
      as libc::c_int
  }
  return S_I_T[indx as usize] as libc::c_int >> syntax * 4i32 & 0xfi32;
}
static mut atab: *mut *mut alias = 0 as *const *mut alias as *mut *mut alias;
unsafe extern "C" fn __lookupalias(mut name: *const libc::c_char) -> *mut *mut alias {
  let mut hashval: libc::c_uint = 0;
  let mut app: *mut *mut alias = 0 as *mut *mut alias;
  let mut p: *const libc::c_char = 0 as *const libc::c_char;
  let mut ch: libc::c_uint = 0;
  p = name;
  ch = *p as libc::c_uchar as libc::c_uint;
  hashval = ch << 4i32;
  while ch != 0 {
    hashval = hashval.wrapping_add(ch);
    p = p.offset(1);
    ch = *p as libc::c_uchar as libc::c_uint
  }
  app = &mut *atab.offset(hashval.wrapping_rem(39i32 as libc::c_uint) as isize) as *mut *mut alias;
  while !(*app).is_null() {
    if strcmp(name, (**app).name) == 0i32 {
      break;
    }
    app = &mut (**app).next
  }
  return app;
}
unsafe extern "C" fn lookupalias(
  mut name: *const libc::c_char,
  mut check: libc::c_int,
) -> *mut alias {
  let mut ap: *mut alias = *__lookupalias(name);
  if check != 0 && !ap.is_null() && (*ap).flag & 1i32 != 0 {
    return 0 as *mut alias;
  }
  return ap;
}
unsafe extern "C" fn freealias(mut ap: *mut alias) -> *mut alias {
  let mut next: *mut alias = 0 as *mut alias;
  if (*ap).flag & 1i32 != 0 {
    (*ap).flag |= 2i32;
    return ap;
  }
  next = (*ap).next;
  free((*ap).name as *mut libc::c_void);
  free((*ap).val as *mut libc::c_void);
  free(ap as *mut libc::c_void);
  return next;
}
unsafe extern "C" fn setalias(mut name: *const libc::c_char, mut val: *const libc::c_char) {
  let mut ap: *mut alias = 0 as *mut alias;
  let mut app: *mut *mut alias = 0 as *mut *mut alias;
  app = __lookupalias(name);
  ap = *app;
  ::std::ptr::write_volatile(
    &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
    ::std::ptr::read_volatile::<libc::c_int>(
      &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
    ) + 1,
  );
  asm!("" : : : "memory" : "volatile");
  if !ap.is_null() {
    if (*ap).flag & 1i32 == 0 {
      free((*ap).val as *mut libc::c_void);
    }
    (*ap).val = xstrdup(val);
    (*ap).flag &= !2i32
  } else {
    ap = xzalloc(::std::mem::size_of::<alias>() as libc::c_ulong) as *mut alias;
    (*ap).name = xstrdup(name);
    (*ap).val = xstrdup(val);
    *app = ap
  }
  int_on();
}
unsafe extern "C" fn unalias(mut name: *const libc::c_char) -> libc::c_int {
  let mut app: *mut *mut alias = 0 as *mut *mut alias;
  app = __lookupalias(name);
  if !(*app).is_null() {
    ::std::ptr::write_volatile(
      &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
      ::std::ptr::read_volatile::<libc::c_int>(
        &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
      ) + 1,
    );
    asm!("" : : : "memory" : "volatile");
    *app = freealias(*app);
    int_on();
    return 0i32;
  }
  return 1i32;
}
unsafe extern "C" fn rmaliases() {
  let mut ap: *mut alias = 0 as *mut alias;
  let mut app: *mut *mut alias = 0 as *mut *mut alias;
  let mut i: libc::c_int = 0;
  ::std::ptr::write_volatile(
    &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
    ::std::ptr::read_volatile::<libc::c_int>(
      &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
    ) + 1,
  );
  asm!("" : : : "memory" : "volatile");
  i = 0i32;
  while i < 39i32 {
    app = &mut *atab.offset(i as isize) as *mut *mut alias;
    ap = *app;
    while !ap.is_null() {
      *app = freealias(*app);
      if ap == *app {
        app = &mut (*ap).next
      }
      ap = *app
    }
    i += 1
  }
  int_on();
}
unsafe extern "C" fn printalias(mut ap: *const alias) {
  out1fmt(
    b"%s=%s\n\x00" as *const u8 as *const libc::c_char,
    (*ap).name,
    single_quote((*ap).val),
  );
}
unsafe extern "C" fn aliascmd(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut n: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut v: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut ret: libc::c_int = 0i32;
  let mut ap: *mut alias = 0 as *mut alias;
  if (*argv.offset(1)).is_null() {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 39i32 {
      ap = *atab.offset(i as isize);
      while !ap.is_null() {
        printalias(ap);
        ap = (*ap).next
      }
      i += 1
    }
    return 0i32;
  }
  loop {
    argv = argv.offset(1);
    n = *argv;
    if n.is_null() {
      break;
    }
    v = strchr(n.offset(1), '=' as i32);
    if v.is_null() {
      ap = *__lookupalias(n);
      if ap.is_null() {
        fprintf(
          stderr,
          b"%s: %s not found\n\x00" as *const u8 as *const libc::c_char,
          b"alias\x00" as *const u8 as *const libc::c_char,
          n,
        );
        ret = 1i32
      } else {
        printalias(ap);
      }
    } else {
      let fresh22 = v;
      v = v.offset(1);
      *fresh22 = '\u{0}' as i32 as libc::c_char;
      setalias(n, v);
    }
  }
  return ret;
}
unsafe extern "C" fn unaliascmd(
  mut _argc: libc::c_int,
  mut _argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut i: libc::c_int = 0;
  if nextopt(b"a\x00" as *const u8 as *const libc::c_char) != '\u{0}' as i32 {
    rmaliases();
    return 0i32;
  }
  i = 0i32;
  while !(*argptr).is_null() {
    if unalias(*argptr) != 0 {
      fprintf(
        stderr,
        b"%s: %s not found\n\x00" as *const u8 as *const libc::c_char,
        b"unalias\x00" as *const u8 as *const libc::c_char,
        *argptr,
      );
      i = 1i32
    }
    argptr = argptr.offset(1)
  }
  return i;
}
static mut doing_jobctl: smallint = 0;
unsafe extern "C" fn ignoresig(mut signo: libc::c_int) {
  if (*ash_ptr_to_globals_misc).sigmode[(signo - 1i32) as usize] as libc::c_int != 3i32
    && (*ash_ptr_to_globals_misc).sigmode[(signo - 1i32) as usize] as libc::c_int != 4i32
  {
    signal(
      signo,
      ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(1i32 as libc::intptr_t),
    );
  }
  (*ash_ptr_to_globals_misc).sigmode[(signo - 1i32) as usize] = 4i32 as libc::c_char;
}
unsafe extern "C" fn signal_handler(mut signo: libc::c_int) {
  if signo == 17i32 {
    ::std::ptr::write_volatile(
      &mut (*ash_ptr_to_globals_misc).got_sigchld as *mut smallint,
      1i32 as smallint,
    );
    if (*ash_ptr_to_globals_misc).trap[17].is_null() {
      return;
    }
  }
  (*ash_ptr_to_globals_misc).gotsig[(signo - 1i32) as usize] = 1i32 as uint8_t;
  ::std::ptr::write_volatile(
    &mut (*ash_ptr_to_globals_misc).pending_sig as *mut smallint,
    signo as smallint,
  );
  if signo == 2i32 && (*ash_ptr_to_globals_misc).trap[2].is_null() {
    if (*ash_ptr_to_globals_misc).suppress_int == 0 {
      ::std::ptr::write_volatile(
        &mut (*ash_ptr_to_globals_misc).pending_sig as *mut smallint,
        0i32 as smallint,
      );
      raise_interrupt();
    }
    ::std::ptr::write_volatile(
      &mut (*ash_ptr_to_globals_misc).pending_int as *mut smallint,
      1i32 as smallint,
    )
  };
}
unsafe extern "C" fn setsignal(mut signo: libc::c_int) {
  let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut cur_act: libc::c_char = 0;
  let mut new_act: libc::c_char = 0;
  let mut act: sigaction = sigaction {
    __sigaction_handler: C2RustUnnamed_9 { sa_handler: None },
    sa_mask: __sigset_t { __val: [0; 16] },
    sa_flags: 0,
    sa_restorer: None,
  };
  t = (*ash_ptr_to_globals_misc).trap[signo as usize];
  new_act = 1i32 as libc::c_char;
  if !t.is_null() {
    new_act = 2i32 as libc::c_char;
    if *t.offset(0) as libc::c_int == '\u{0}' as i32 {
      new_act = 3i32 as libc::c_char
    }
  }
  if (*ash_ptr_to_globals_misc).shlvl == 0 && new_act as libc::c_int == 1i32 {
    match signo {
      2 => {
        if (*ash_ptr_to_globals_misc).optlist[3] as libc::c_int != 0
          || !(*ash_ptr_to_globals_misc).minusc.is_null()
          || (*ash_ptr_to_globals_misc).optlist[6] as libc::c_int == 0i32
        {
          new_act = 2i32 as libc::c_char
        }
      }
      3 => new_act = 3i32 as libc::c_char,
      15 => {
        if (*ash_ptr_to_globals_misc).optlist[3] != 0 {
          new_act = 3i32 as libc::c_char
        }
      }
      20 | 22 => {
        if (*ash_ptr_to_globals_misc).optlist[4] != 0 {
          new_act = 3i32 as libc::c_char
        }
      }
      _ => {}
    }
  }
  if signo == 17i32 {
    new_act = 2i32 as libc::c_char
  }
  t = &mut *(*ash_ptr_to_globals_misc)
    .sigmode
    .as_mut_ptr()
    .offset((signo - 1i32) as isize) as *mut libc::c_char;
  cur_act = *t;
  if cur_act as libc::c_int == 0i32 {
    if sigaction(signo, 0 as *const sigaction, &mut act) != 0 {
      return;
    }
    if act.__sigaction_handler.sa_handler
      == ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(1i32 as libc::intptr_t)
    {
      cur_act = 4i32 as libc::c_char;
      if (*ash_ptr_to_globals_misc).optlist[4] as libc::c_int != 0
        && (signo == 20i32 || signo == 21i32 || signo == 22i32)
      {
        cur_act = 3i32 as libc::c_char
      }
    }
    if act.__sigaction_handler.sa_handler.is_none() && new_act as libc::c_int == 1i32 {
      *t = 1i32 as libc::c_char;
      return;
    }
  }
  if cur_act as libc::c_int == 4i32 || cur_act as libc::c_int == new_act as libc::c_int {
    return;
  }
  *t = new_act;
  act.__sigaction_handler.sa_handler = None;
  match new_act as libc::c_int {
    2 => {
      act.__sigaction_handler.sa_handler =
        Some(signal_handler as unsafe extern "C" fn(_: libc::c_int) -> ())
    }
    3 => {
      act.__sigaction_handler.sa_handler =
        ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(1i32 as libc::intptr_t)
    }
    _ => {}
  }
  act.sa_flags = 0i32;
  sigfillset(&mut act.sa_mask);
  sigaction_set(signo, &mut act);
}
static mut initialpgrp: libc::c_int = 0;
static mut ttyfd: libc::c_int = -1i32;
static mut jobtab: *mut job = 0 as *const job as *mut job;
static mut njobs: libc::c_uint = 0;
static mut curjob: *mut job = 0 as *const job as *mut job;
static mut jobless: libc::c_int = 0;
unsafe extern "C" fn set_curjob(mut jp: *mut job, mut mode: libc::c_uint) {
  let mut jp1: *mut job = 0 as *mut job;
  let mut jpp: *mut *mut job = 0 as *mut *mut job;
  let mut curp: *mut *mut job = 0 as *mut *mut job;
  curp = &mut curjob;
  jpp = curp;
  loop {
    jp1 = *jpp;
    if jp1 == jp {
      break;
    }
    jpp = &mut (*jp1).prev_job
  }
  *jpp = (*jp1).prev_job;
  jpp = curp;
  let mut current_block_10: u64;
  match mode {
    1 => {
      loop {
        jp1 = *jpp;
        if jp1.is_null() || (*jp1).state() as libc::c_int != 1i32 {
          break;
        }
        jpp = &mut (*jp1).prev_job
      }
      current_block_10 = 18014961938290295065;
    }
    0 => {
      current_block_10 = 18014961938290295065;
    }
    2 | _ => {
      current_block_10 = 8457315219000651999;
    }
  }
  match current_block_10 {
    18014961938290295065 => {
      (*jp).prev_job = *jpp;
      *jpp = jp
    }
    _ => {}
  };
}
unsafe extern "C" fn jobno(mut jp: *const job) -> libc::c_int {
  return (jp.wrapping_offset_from(jobtab) as libc::c_long + 1i32 as libc::c_long) as libc::c_int;
}
unsafe extern "C" fn getjob(mut name: *const libc::c_char, mut getctl: libc::c_int) -> *mut job {
  let mut current_block: u64;
  let mut jp: *mut job = 0 as *mut job;
  let mut found: *mut job = 0 as *mut job;
  let mut err_msg: *const libc::c_char = b"%s: no such job\x00" as *const u8 as *const libc::c_char;
  let mut num: libc::c_uint = 0;
  let mut c: libc::c_int = 0;
  let mut p: *const libc::c_char = 0 as *const libc::c_char;
  let mut match_0: Option<
    unsafe extern "C" fn(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char,
  > = None;
  jp = curjob;
  p = name;
  if p.is_null() {
    current_block = 17838868982845454862;
  } else if *p as libc::c_int != '%' as i32 {
    current_block = 10520803763519672787;
  } else {
    p = p.offset(1);
    c = *p as libc::c_int;
    if c == 0 {
      current_block = 17838868982845454862;
    } else {
      if *p.offset(1) == 0 {
        if c == '+' as i32 || c == '%' as i32 {
          current_block = 17838868982845454862;
        } else if c == '-' as i32 {
          if !jp.is_null() {
            jp = (*jp).prev_job
          }
          err_msg = b"No previous job\x00" as *const u8 as *const libc::c_char;
          current_block = 2735549783930551457;
        } else {
          current_block = 11584701595673473500;
        }
      } else {
        current_block = 11584701595673473500;
      }
      match current_block {
        17838868982845454862 => {}
        2735549783930551457 => {}
        _ => {
          if is_number(p) != 0 {
            num = atoi(p) as libc::c_uint;
            if num > 0i32 as libc::c_uint && num <= njobs {
              jp = jobtab.offset(num as isize).offset(-1);
              if (*jp).used() != 0 {
                current_block = 4019621590527368594;
              } else {
                current_block = 10520803763519672787;
              }
            } else {
              current_block = 11057878835866523405;
            }
          } else {
            current_block = 11057878835866523405;
          }
          match current_block {
            4019621590527368594 => {}
            10520803763519672787 => {}
            _ => {
              match_0 = Some(
                prefix
                  as unsafe extern "C" fn(
                    _: *const libc::c_char,
                    _: *const libc::c_char,
                  ) -> *mut libc::c_char,
              );
              if *p as libc::c_int == '?' as i32 {
                match_0 = Some(
                  strstr
                    as unsafe extern "C" fn(
                      _: *const libc::c_char,
                      _: *const libc::c_char,
                    ) -> *mut libc::c_char,
                );
                p = p.offset(1)
              }
              found = 0 as *mut job;
              loop {
                if jp.is_null() {
                  current_block = 14072441030219150333;
                  break;
                }
                if !match_0.expect("non-null function pointer")((*(*jp).ps.offset(0)).ps_cmd, p)
                  .is_null()
                {
                  if !found.is_null() {
                    current_block = 10520803763519672787;
                    break;
                  }
                  found = jp;
                  err_msg = b"%s: ambiguous\x00" as *const u8 as *const libc::c_char
                }
                jp = (*jp).prev_job
              }
              match current_block {
                10520803763519672787 => {}
                _ => {
                  if found.is_null() {
                    current_block = 10520803763519672787;
                  } else {
                    jp = found;
                    current_block = 4019621590527368594;
                  }
                }
              }
            }
          }
        }
      }
    }
  }
  match current_block {
    17838868982845454862 => {
      err_msg = b"No current job\x00" as *const u8 as *const libc::c_char;
      current_block = 2735549783930551457;
    }
    _ => {}
  }
  match current_block {
    2735549783930551457 => {
      if jp.is_null() {
        current_block = 10520803763519672787;
      } else {
        current_block = 4019621590527368594;
      }
    }
    _ => {}
  }
  match current_block {
    4019621590527368594 => {
      err_msg = b"job %s not created under job control\x00" as *const u8 as *const libc::c_char;
      if !(getctl != 0 && (*jp).jobctl() as libc::c_int == 0i32) {
        return jp;
      }
    }
    _ => {}
  }
  ash_msg_and_raise_error(err_msg, name);
}
unsafe extern "C" fn freejob(mut jp: *mut job) {
  let mut ps: *mut procstat = 0 as *mut procstat;
  let mut i: libc::c_int = 0;
  ::std::ptr::write_volatile(
    &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
    ::std::ptr::read_volatile::<libc::c_int>(
      &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
    ) + 1,
  );
  asm!("" : : : "memory" : "volatile");
  i = (*jp).nprocs as libc::c_int;
  ps = (*jp).ps;
  loop {
    i -= 1;
    if !(i >= 0i32) {
      break;
    }
    if (*ps).ps_cmd != (*ash_ptr_to_globals_misc).nullstr.as_mut_ptr() {
      free((*ps).ps_cmd as *mut libc::c_void);
    }
    ps = ps.offset(1)
  }
  if (*jp).ps != &mut (*jp).ps0 as *mut procstat {
    free((*jp).ps as *mut libc::c_void);
  }
  (*jp).set_used(0i32 as libc::c_uint);
  set_curjob(jp, 2i32 as libc::c_uint);
  int_on();
}
unsafe extern "C" fn xtcsetpgrp(mut fd: libc::c_int, mut pgrp: pid_t) {
  if tcsetpgrp(fd, pgrp) != 0 {
    ash_msg_and_raise_error(
      b"can\'t set tty process group: %m\x00" as *const u8 as *const libc::c_char,
    );
  };
}
unsafe extern "C" fn setjobctl(mut on: libc::c_int) {
  let mut current_block: u64;
  let mut fd: libc::c_int = 0;
  let mut pgrp: libc::c_int = 0;
  if on == doing_jobctl as libc::c_int
    || ((*ash_ptr_to_globals_misc).shlvl == 0) as libc::c_int == 0i32
  {
    return;
  }
  if on != 0 {
    let mut ofd: libc::c_int = 0;
    fd = open(b"/dev/tty\x00" as *const u8 as *const libc::c_char, 0o2i32);
    ofd = fd;
    if fd < 0i32 {
      fd = 2i32;
      loop {
        if !(isatty(fd) == 0) {
          current_block = 1394248824506584008;
          break;
        }
        fd -= 1;
        if fd < 0i32 {
          current_block = 14414541239968212827;
          break;
        }
      }
    } else {
      current_block = 1394248824506584008;
    }
    match current_block {
      1394248824506584008 => {
        fd = fcntl(fd, 1030i32, 10i32);
        if ofd >= 0i32 {
          close(ofd);
        }
        if fd < 0i32 {
          current_block = 14414541239968212827;
        } else {
          if 1030i32 == 0i32 {
            close_on_exec_on(fd);
          }
          loop {
            pgrp = tcgetpgrp(fd);
            if pgrp < 0i32 {
              current_block = 14414541239968212827;
              break;
            }
            if pgrp == getpgrp() {
              initialpgrp = pgrp;
              setsignal(20i32);
              setsignal(22i32);
              setsignal(21i32);
              pgrp = (*ash_ptr_to_globals_misc).rootpid;
              setpgid(0i32, pgrp);
              xtcsetpgrp(fd, pgrp);
              current_block = 13131896068329595644;
              break;
            } else {
              killpg(0i32, 21i32);
            }
          }
        }
      }
      _ => {}
    }
    match current_block {
      13131896068329595644 => {}
      _ => {
        ash_msg(
          b"can\'t access tty; job control turned off\x00" as *const u8 as *const libc::c_char,
        );
        on = 0i32;
        (*ash_ptr_to_globals_misc).optlist[4] = on as libc::c_char;
        current_block = 16228935912667152374;
      }
    }
  } else {
    fd = ttyfd;
    pgrp = initialpgrp;
    tcsetpgrp(fd, pgrp);
    setpgid(0i32, pgrp);
    setsignal(20i32);
    setsignal(22i32);
    setsignal(21i32);
    current_block = 16228935912667152374;
  }
  match current_block {
    16228935912667152374 => {
      if fd >= 0i32 {
        close(fd);
      }
      fd = -1i32
    }
    _ => {}
  }
  ttyfd = fd;
  doing_jobctl = on as smallint;
}
unsafe extern "C" fn killcmd(
  mut argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  if !(*argv.offset(1)).is_null()
    && strcmp(
      *argv.offset(1),
      b"-l\x00" as *const u8 as *const libc::c_char,
    ) != 0i32
  {
    let mut i: libc::c_int = 1i32;
    loop {
      if *(*argv.offset(i as isize)).offset(0) as libc::c_int == '%' as i32 {
        let mut jp: *mut job = 0 as *mut job;
        let mut dst: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut j: libc::c_int = 0;
        let mut n: libc::c_int = 0;
        jp = getjob(*argv.offset(i as isize), 0i32);
        n = (*jp).nprocs as libc::c_int;
        if (*jp).jobctl() != 0 {
          n = 1i32
        }
        let mut fresh23 = ::std::vec::from_elem(
          0,
          (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(4i32 as libc::c_ulong) as usize,
        );
        dst = fresh23.as_mut_ptr() as *mut libc::c_char;
        let ref mut fresh24 = *argv.offset(i as isize);
        *fresh24 = dst;
        j = 0i32;
        while j < n {
          let mut ps: *mut procstat = &mut *(*jp).ps.offset(j as isize) as *mut procstat;
          if !((*ps).ps_status != -1i32 && !((*ps).ps_status & 0xffi32 == 0x7fi32)) {
            dst = dst.offset(sprintf(
              dst,
              if (*jp).jobctl() as libc::c_int != 0 {
                b" -%u\x00" as *const u8 as *const libc::c_char
              } else {
                b" %u\x00" as *const u8 as *const libc::c_char
              },
              (*ps).ps_pid,
            ) as isize)
          }
          j += 1
        }
        *dst = '\u{0}' as i32 as libc::c_char
      }
      i += 1;
      if (*argv.offset(i as isize)).is_null() {
        break;
      }
    }
  }
  return kill_main(argc, argv);
}
unsafe extern "C" fn showpipe(mut jp: *mut job) {
  let mut ps: *mut procstat = 0 as *mut procstat;
  let mut psend: *mut procstat = 0 as *mut procstat;
  psend = (*jp).ps.offset((*jp).nprocs as isize);
  ps = (*jp).ps.offset(1);
  while ps < psend {
    printf(
      b" | %s\x00" as *const u8 as *const libc::c_char,
      (*ps).ps_cmd,
    );
    ps = ps.offset(1)
  }
  newline_and_flush(stdout);
  flush_stdout_stderr();
}
unsafe extern "C" fn restartjob(mut jp: *mut job, mut mode: libc::c_int) -> libc::c_int {
  let mut ps: *mut procstat = 0 as *mut procstat;
  let mut i: libc::c_int = 0;
  let mut status: libc::c_int = 0;
  let mut pgid: pid_t = 0;
  ::std::ptr::write_volatile(
    &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
    ::std::ptr::read_volatile::<libc::c_int>(
      &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
    ) + 1,
  );
  asm!("" : : : "memory" : "volatile");
  if !((*jp).state() as libc::c_int == 2i32) {
    (*jp).set_state(0i32 as libc::c_uint);
    pgid = (*(*jp).ps.offset(0)).ps_pid;
    if mode == 0i32 {
      xtcsetpgrp(ttyfd, pgid);
    }
    killpg(pgid, 18i32);
    ps = (*jp).ps;
    i = (*jp).nprocs as libc::c_int;
    loop {
      if (*ps).ps_status & 0xffi32 == 0x7fi32 {
        (*ps).ps_status = -1i32
      }
      ps = ps.offset(1);
      i -= 1;
      if !(i != 0) {
        break;
      }
    }
  }
  status = if mode == 0i32 { waitforjob(jp) } else { 0i32 };
  int_on();
  return status;
}
unsafe extern "C" fn fg_bgcmd(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut jp: *mut job = 0 as *mut job;
  let mut mode: libc::c_int = 0;
  let mut retval: libc::c_int = 0;
  mode = if **argv as libc::c_int == 'f' as i32 {
    0i32
  } else {
    1i32
  };
  nextopt((*ash_ptr_to_globals_misc).nullstr.as_mut_ptr());
  argv = argptr;
  loop {
    jp = getjob(*argv, 1i32);
    if mode == 1i32 {
      set_curjob(jp, 1i32 as libc::c_uint);
      printf(b"[%d] \x00" as *const u8 as *const libc::c_char, jobno(jp));
    }
    out1str((*(*jp).ps.offset(0)).ps_cmd);
    showpipe(jp);
    retval = restartjob(jp, mode);
    if !(!(*argv).is_null() && {
      argv = argv.offset(1);
      !(*argv).is_null()
    }) {
      break;
    }
  }
  return retval;
}
unsafe extern "C" fn sprint_status48(
  mut s: *mut libc::c_char,
  mut status: libc::c_int,
  mut sigonly: libc::c_int,
) -> libc::c_int {
  let mut current_block: u64;
  let mut col: libc::c_int = 0;
  let mut st: libc::c_int = 0;
  col = 0i32;
  if !(status & 0x7fi32 == 0i32) {
    if status & 0xffi32 == 0x7fi32 {
      st = (status & 0xff00i32) >> 8i32
    } else {
      st = status & 0x7fi32
    }
    if sigonly != 0 {
      if st == 2i32 || st == 13i32 {
        current_block = 3764826008022351498;
      } else if status & 0xffi32 == 0x7fi32 {
        current_block = 3764826008022351498;
      } else {
        current_block = 13183875560443969876;
      }
    } else {
      current_block = 13183875560443969876;
    }
    match current_block {
      3764826008022351498 => {}
      _ => {
        st &= 0x7fi32;
        col = fmtstr(s, 32i32 as size_t, strsignal(st));
        if status & 0x80i32 != 0 {
          strcpy(
            s.offset(col as isize),
            b" (core dumped)\x00" as *const u8 as *const libc::c_char,
          );
          col = (col as libc::c_ulong).wrapping_add(
            (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
              .wrapping_sub(1i32 as libc::c_ulong),
          ) as libc::c_int as libc::c_int
        }
      }
    }
  } else if sigonly == 0 {
    st = (status & 0xff00i32) >> 8i32;
    col = fmtstr(
      s,
      16i32 as size_t,
      if st != 0 {
        b"Done(%d)\x00" as *const u8 as *const libc::c_char
      } else {
        b"Done\x00" as *const u8 as *const libc::c_char
      },
      st,
    )
  }
  return col;
}
unsafe extern "C" fn wait_block_or_sig(mut status: *mut libc::c_int) -> libc::c_int {
  let mut pid: libc::c_int = 0;
  loop {
    let mut mask: sigset_t = __sigset_t { __val: [0; 16] };
    ::std::ptr::write_volatile(
      &mut (*ash_ptr_to_globals_misc).got_sigchld as *mut smallint,
      0i32 as smallint,
    );
    pid = waitpid(
      -1i32,
      status,
      if doing_jobctl as libc::c_int != 0 {
        (1i32) | 2i32
      } else {
        1i32
      },
    );
    if pid != 0i32 {
      break;
    }
    sigfillset(&mut mask);
    sigprocmask2(2i32, &mut mask);
    while (*ash_ptr_to_globals_misc).got_sigchld == 0 && (*ash_ptr_to_globals_misc).pending_sig == 0
    {
      sigsuspend(&mut mask);
    }
    sigprocmask(2i32, &mut mask, 0 as *mut sigset_t);
    if !((*ash_ptr_to_globals_misc).got_sigchld != 0) {
      break;
    }
  }
  return pid;
}
unsafe extern "C" fn dowait(mut block: libc::c_int, mut job: *mut job) -> libc::c_int {
  let mut current_block: u64;
  let mut pid: libc::c_int = 0;
  let mut status: libc::c_int = 0;
  let mut jp: *mut job = 0 as *mut job;
  let mut thisjob: *mut job = 0 as *mut job;
  let mut want_jobexitstatus: bool = block & 0x10i32 != 0;
  block = block & !0x10i32;
  ::std::ptr::write_volatile(
    &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
    ::std::ptr::read_volatile::<libc::c_int>(
      &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
    ) + 1,
  );
  asm!("" : : : "memory" : "volatile");
  if block == 2i32 {
    pid = wait_block_or_sig(&mut status)
  } else {
    let mut wait_flags: libc::c_int = 0i32;
    if block == 0i32 {
      wait_flags = 1i32
    }
    if doing_jobctl != 0 {
      wait_flags |= 2i32
    }
    pid = waitpid(-1i32, &mut status, wait_flags)
  }
  thisjob = 0 as *mut job;
  if !(pid <= 0i32) {
    jp = curjob;
    loop {
      if jp.is_null() {
        current_block = 5891011138178424807;
        break;
      }
      let mut jobstate: libc::c_int = 0;
      let mut ps: *mut procstat = 0 as *mut procstat;
      let mut psend: *mut procstat = 0 as *mut procstat;
      if !((*jp).state() as libc::c_int == 2i32) {
        jobstate = 2i32;
        ps = (*jp).ps;
        psend = ps.offset((*jp).nprocs as isize);
        loop {
          if (*ps).ps_pid == pid {
            (*ps).ps_status = status;
            thisjob = jp
          }
          if (*ps).ps_status == -1i32 {
            jobstate = 0i32
          }
          if !(jobstate == 0i32) {
            if (*ps).ps_status & 0xffi32 == 0x7fi32 {
              (*jp).stopstatus = (*ps).ps_status;
              jobstate = 1i32
            }
          }
          ps = ps.offset(1);
          if !(ps < psend) {
            break;
          }
        }
        if !thisjob.is_null() {
          if jobstate != 0i32 {
            (*thisjob).set_changed(1i32 as libc::c_uint);
            if (*thisjob).state() as libc::c_int != jobstate {
              (*thisjob).set_state(jobstate as libc::c_uint);
              if jobstate == 1i32 {
                set_curjob(thisjob, 0i32 as libc::c_uint);
              }
            }
          }
          current_block = 8189439656108241486;
          break;
        }
      }
      jp = (*jp).prev_job
    }
    match current_block {
      8189439656108241486 => {}
      _ => {
        if !(status & 0xffi32 == 0x7fi32) {
          jobless -= 1
        }
      }
    }
  }
  int_on();
  if want_jobexitstatus {
    pid = -1i32;
    if !thisjob.is_null() && (*thisjob).state() as libc::c_int == 2i32 {
      pid = (*(*thisjob)
        .ps
        .offset((*thisjob).nprocs.wrapping_sub(1i32 as libc::c_uint) as isize))
      .ps_status
    }
  }
  if !thisjob.is_null() && thisjob == job {
    let mut s: [libc::c_char; 49] = [0; 49];
    let mut len: libc::c_int = 0;
    len = sprint_status48(s.as_mut_ptr(), status, 1i32);
    if len != 0 {
      s[len as usize] = '\n' as i32 as libc::c_char;
      s[(len + 1i32) as usize] = '\u{0}' as i32 as libc::c_char;
      out2str(s.as_mut_ptr());
    }
  }
  return pid;
}
unsafe extern "C" fn showjob(mut jp: *mut job, mut mode: libc::c_int) {
  let mut ps: *mut procstat = 0 as *mut procstat;
  let mut psend: *mut procstat = 0 as *mut procstat;
  let mut col: libc::c_int = 0;
  let mut indent_col: libc::c_int = 0;
  let mut s: [libc::c_char; 80] = [0; 80];
  let mut out: *mut FILE = if mode & 0x8i32 != 0 { stderr } else { stdout };
  ps = (*jp).ps;
  if mode & 0x1i32 != 0 {
    fprintf(
      out,
      b"%d\n\x00" as *const u8 as *const libc::c_char,
      (*ps).ps_pid,
    );
    return;
  }
  col = fmtstr(
    s.as_mut_ptr(),
    16i32 as size_t,
    b"[%d]   \x00" as *const u8 as *const libc::c_char,
    jobno(jp),
  );
  indent_col = col;
  if jp == curjob {
    s[(col - 3i32) as usize] = '+' as i32 as libc::c_char
  } else if !curjob.is_null() && jp == (*curjob).prev_job {
    s[(col - 3i32) as usize] = '-' as i32 as libc::c_char
  }
  if mode & 0x2i32 != 0 {
    col += fmtstr(
      s.as_mut_ptr().offset(col as isize),
      16i32 as size_t,
      b"%d \x00" as *const u8 as *const libc::c_char,
      (*ps).ps_pid,
    )
  }
  psend = ps.offset((*jp).nprocs as isize);
  if (*jp).state() as libc::c_int == 0i32 {
    strcpy(
      s.as_mut_ptr().offset(col as isize),
      b"Running\x00" as *const u8 as *const libc::c_char,
    );
    col = (col as libc::c_ulong).wrapping_add(
      (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong),
    ) as libc::c_int as libc::c_int
  } else {
    let mut status: libc::c_int = (*psend.offset(-1i32 as isize)).ps_status;
    if (*jp).state() as libc::c_int == 1i32 {
      status = (*jp).stopstatus
    }
    col += sprint_status48(s.as_mut_ptr().offset(col as isize), status, 0i32)
  }
  loop {
    fprintf(
      out,
      b"%s%*c%s%s\x00" as *const u8 as *const libc::c_char,
      s.as_mut_ptr(),
      if 33i32 - col >= 0i32 {
        (33i32) - col
      } else {
        0i32
      },
      ' ' as i32,
      if ps == (*jp).ps {
        b"\x00" as *const u8 as *const libc::c_char
      } else {
        b"| \x00" as *const u8 as *const libc::c_char
      },
      (*ps).ps_cmd,
    );
    ps = ps.offset(1);
    if !(ps != psend) {
      break;
    }
    s[0] = '\u{0}' as i32 as libc::c_char;
    col = 33i32;
    if mode & 0x2i32 != 0 {
      col = fmtstr(
        s.as_mut_ptr(),
        48i32 as size_t,
        b"\n%*c%d \x00" as *const u8 as *const libc::c_char,
        indent_col,
        ' ' as i32,
        (*ps).ps_pid,
      ) - 1i32
    }
  }
  newline_and_flush(out);
  (*jp).set_changed(0i32 as libc::c_uint);
  if (*jp).state() as libc::c_int == 2i32 {
    freejob(jp);
  };
}
unsafe extern "C" fn showjobs(mut mode: libc::c_int) {
  let mut jp: *mut job = 0 as *mut job;
  while dowait(0i32, 0 as *mut job) > 0i32 {}
  jp = curjob;
  while !jp.is_null() {
    if mode & 0x4i32 == 0 || (*jp).changed() as libc::c_int != 0 {
      showjob(jp, mode);
    }
    jp = (*jp).prev_job
  }
}
unsafe extern "C" fn jobscmd(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut mode: libc::c_int = 0;
  let mut m: libc::c_int = 0;
  mode = 0i32;
  loop {
    m = nextopt(b"lp\x00" as *const u8 as *const libc::c_char);
    if !(m != '\u{0}' as i32) {
      break;
    }
    if m == 'l' as i32 {
      mode |= 0x2i32
    } else {
      mode |= 0x1i32
    }
  }
  argv = argptr;
  if !(*argv).is_null() {
    loop {
      showjob(getjob(*argv, 0i32), mode);
      argv = argv.offset(1);
      if (*argv).is_null() {
        break;
      }
    }
  } else {
    showjobs(mode);
  }
  return 0i32;
}
unsafe extern "C" fn getstatus(mut job: *mut job) -> libc::c_int {
  let mut status: libc::c_int = 0;
  let mut retval: libc::c_int = 0;
  let mut ps: *mut procstat = 0 as *mut procstat;
  ps = (*job).ps.offset((*job).nprocs as isize).offset(-1);
  status = (*ps).ps_status;
  if (*ash_ptr_to_globals_misc).optlist[15] != 0 {
    while status == 0i32 && {
      ps = ps.offset(-1);
      (ps) >= (*job).ps
    } {
      status = (*ps).ps_status
    }
  }
  retval = (status & 0xff00i32) >> 8i32;
  if !(status & 0x7fi32 == 0i32) {
    retval = (status & 0xff00i32) >> 8i32;
    if !(status & 0xffi32 == 0x7fi32) {
      retval = status & 0x7fi32;
      if retval == 2i32 {
        (*job).set_sigint(1i32 as libc::c_uint)
      }
    }
    retval += 128i32
  }
  return retval;
}
unsafe extern "C" fn waitcmd(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut current_block: u64;
  let mut job: *mut job = 0 as *mut job;
  let mut retval: libc::c_int = 0;
  let mut jp: *mut job = 0 as *mut job;
  let mut status: libc::c_int = 0;
  let mut one: libc::c_char = nextopt(b"n\x00" as *const u8 as *const libc::c_char) as libc::c_char;
  retval = 0i32;
  argv = argptr;
  if (*argv.offset(0)).is_null() {
    's_34: loop {
      jp = curjob;
      if one as libc::c_int != 0 && jp.is_null() {
        retval = 127i32
      }
      loop {
        if jp.is_null() {
          current_block = 12161739351286591700;
          break 's_34;
        }
        if (*jp).state() as libc::c_int == 0i32 {
          break;
        }
        (*jp).set_waited(1i32 as libc::c_uint);
        jp = (*jp).prev_job
      }
      status = dowait(2i32 | 0x10i32, 0 as *mut job);
      if (*ash_ptr_to_globals_misc).pending_sig != 0 {
        current_block = 18434991528557801575;
        break;
      }
      if !(one != 0) {
        continue;
      }
      if !(status != -1i32 && !(status & 0xffi32 == 0x7fi32)) {
        continue;
      }
      retval = (status & 0xff00i32) >> 8i32;
      if ((status & 0x7fi32) + 1i32) as libc::c_schar as libc::c_int >> 1i32 > 0i32 {
        retval = (status & 0x7fi32) + 128i32
      }
      current_block = 12161739351286591700;
      break;
    }
  } else {
    retval = 127i32;
    's_133: loop {
      if **argv as libc::c_int != '%' as i32 {
        let mut pid: pid_t = number(*argv);
        job = curjob;
        loop {
          if job.is_null() {
            current_block = 6669252993407410313;
            break;
          }
          if (*(*job)
            .ps
            .offset((*job).nprocs.wrapping_sub(1i32 as libc::c_uint) as isize))
          .ps_pid
            == pid
          {
            current_block = 6450636197030046351;
            break;
          }
          job = (*job).prev_job
        }
      } else {
        job = getjob(*argv, 0i32);
        current_block = 6450636197030046351;
      }
      loop {
        match current_block {
          6669252993407410313 => {
            argv = argv.offset(1);
            if !(*argv).is_null() {
              break;
            } else {
              current_block = 12161739351286591700;
              break 's_133;
            }
          }
          _ => {
            if (*job).state() as libc::c_int == 0i32 {
              dowait(2i32, 0 as *mut job);
              if (*ash_ptr_to_globals_misc).pending_sig != 0 {
                current_block = 18434991528557801575;
                break 's_133;
              } else {
                current_block = 6450636197030046351;
              }
            } else {
              (*job).set_waited(1i32 as libc::c_uint);
              retval = getstatus(job);
              current_block = 6669252993407410313;
            }
          }
        }
      }
    }
  }
  match current_block {
    12161739351286591700 => return retval,
    _ => {
      retval = 128i32 + (*ash_ptr_to_globals_misc).pending_sig as libc::c_int;
      return retval;
    }
  };
}
unsafe extern "C" fn growjobtab() -> *mut job {
  let mut len: size_t = 0;
  let mut offset: ptrdiff_t = 0;
  let mut jp: *mut job = 0 as *mut job;
  let mut jq: *mut job = 0 as *mut job;
  len = (njobs as libc::c_ulong).wrapping_mul(::std::mem::size_of::<job>() as libc::c_ulong);
  jq = jobtab;
  jp = xrealloc(
    jq as *mut libc::c_void,
    len.wrapping_add(
      (4i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<job>() as libc::c_ulong),
    ),
  ) as *mut job;
  offset = (jp as *mut libc::c_char).wrapping_offset_from(jq as *mut libc::c_char) as libc::c_long;
  if offset != 0 {
    let mut l: size_t = len;
    jq = (jq as *mut libc::c_char).offset(l as isize) as *mut job;
    while l != 0 {
      l = (l as libc::c_ulong).wrapping_sub(::std::mem::size_of::<job>() as libc::c_ulong) as size_t
        as size_t;
      jq = jq.offset(-1);
      if (*((jp as *mut libc::c_char).offset(l as isize) as *mut job)).ps
        == &mut (*jq).ps0 as *mut procstat
      {
        let ref mut fresh25 = (*((jp as *mut libc::c_char).offset(l as isize) as *mut job)).ps;
        *fresh25 = ((*((jp as *mut libc::c_char).offset(l as isize) as *mut job)).ps
          as *mut libc::c_char)
          .offset(offset as isize) as *mut libc::c_void as *mut procstat
      }
      if !(*((jp as *mut libc::c_char).offset(l as isize) as *mut job))
        .prev_job
        .is_null()
      {
        let ref mut fresh26 =
          (*((jp as *mut libc::c_char).offset(l as isize) as *mut job)).prev_job;
        *fresh26 = ((*((jp as *mut libc::c_char).offset(l as isize) as *mut job)).prev_job
          as *mut libc::c_char)
          .offset(offset as isize) as *mut libc::c_void as *mut job
      }
    }
    if !curjob.is_null() {
      curjob =
        (curjob as *mut libc::c_char).offset(offset as isize) as *mut libc::c_void as *mut job
    }
  }
  njobs = njobs.wrapping_add(4i32 as libc::c_uint);
  jobtab = jp;
  jp = (jp as *mut libc::c_char).offset(len as isize) as *mut job;
  jq = jp.offset(3);
  loop {
    (*jq).set_used(0i32 as libc::c_uint);
    jq = jq.offset(-1);
    if !(jq >= jp) {
      break;
    }
  }
  return jp;
}
unsafe extern "C" fn makejob(mut nprocs: libc::c_int) -> *mut job {
  let mut i: libc::c_int = 0;
  let mut jp: *mut job = 0 as *mut job;
  i = njobs as libc::c_int;
  jp = jobtab;
  loop {
    i -= 1;
    if i < 0i32 {
      jp = growjobtab();
      break;
    } else {
      if (*jp).used() as libc::c_int == 0i32 {
        break;
      }
      if !((*jp).state() as libc::c_int != 2i32 || (*jp).waited() == 0) {
        if !(doing_jobctl != 0) {
          freejob(jp);
          break;
        }
      }
      jp = jp.offset(1)
    }
  }
  memset(
    jp as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<job>() as libc::c_ulong,
  );
  if doing_jobctl != 0 {
    (*jp).set_jobctl(1i32 as libc::c_uint)
  }
  (*jp).prev_job = curjob;
  curjob = jp;
  (*jp).set_used(1i32 as libc::c_uint);
  (*jp).ps = &mut (*jp).ps0;
  if nprocs > 1i32 {
    (*jp).ps = xmalloc(
      (nprocs as libc::c_ulong).wrapping_mul(::std::mem::size_of::<procstat>() as libc::c_ulong),
    ) as *mut procstat
  }
  return jp;
}
static mut cmdnextc: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
unsafe extern "C" fn cmdputs(mut s: *const libc::c_char) {
  let mut current_block: u64;
  static mut vstype: [[libc::c_char; 3]; 16] = [
    [0, 0, 0],
    [125, 0, 0],
    [45, 0, 0],
    [43, 0, 0],
    [63, 0, 0],
    [61, 0, 0],
    [37, 0, 0],
    [37, 37, 0],
    [35, 0, 0],
    [35, 35, 0],
    [58, 0, 0],
    [47, 0, 0],
    [47, 47, 0],
    [0; 3],
    [0; 3],
    [0; 3],
  ];
  let mut p: *const libc::c_char = 0 as *const libc::c_char;
  let mut str: *const libc::c_char = 0 as *const libc::c_char;
  let mut cc: [libc::c_char; 2] = [0; 2];
  let mut nextc: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut c: libc::c_uchar = 0;
  let mut subtype: libc::c_uchar = 0i32 as libc::c_uchar;
  let mut quoted: libc::c_int = 0i32;
  cc[1] = '\u{0}' as i32 as libc::c_char;
  nextc = makestrspace(
    strlen(s)
      .wrapping_add(1i32 as libc::c_ulong)
      .wrapping_mul(8i32 as libc::c_ulong),
    cmdnextc,
  );
  p = s;
  loop {
    let fresh27 = p;
    p = p.offset(1);
    c = *fresh27 as libc::c_uchar;
    if !(c as libc::c_int != '\u{0}' as i32) {
      break;
    }
    str = 0 as *const libc::c_char;
    match c as libc::c_int {
      129 => {
        let fresh28 = p;
        p = p.offset(1);
        c = *fresh28 as libc::c_uchar;
        current_block = 8845338526596852646;
      }
      130 => {
        let fresh29 = p;
        p = p.offset(1);
        subtype = *fresh29 as libc::c_uchar;
        if subtype as libc::c_int & 0xfi32 == 0xai32 {
          str = b"${#\x00" as *const u8 as *const libc::c_char
        } else {
          str = b"${\x00" as *const u8 as *const libc::c_char
        }
        current_block = 2861575971432896979;
      }
      131 => {
        str = (b"\"}\x00" as *const u8 as *const libc::c_char)
          .offset((quoted & 1i32 == 0) as libc::c_int as isize);
        quoted >>= 1i32;
        subtype = 0i32 as libc::c_uchar;
        current_block = 2861575971432896979;
      }
      132 => {
        str = b"$(...)\x00" as *const u8 as *const libc::c_char;
        current_block = 2861575971432896979;
      }
      134 => {
        str = b"$((\x00" as *const u8 as *const libc::c_char;
        current_block = 2861575971432896979;
      }
      135 => {
        str = b"))\x00" as *const u8 as *const libc::c_char;
        current_block = 2861575971432896979;
      }
      136 => {
        quoted ^= 1i32;
        c = '\"' as i32 as libc::c_uchar;
        current_block = 8845338526596852646;
      }
      61 => {
        if subtype as libc::c_int == 0i32 {
          current_block = 8845338526596852646;
        } else {
          if subtype as libc::c_int & 0xfi32 != 0x1i32 {
            quoted <<= 1i32
          }
          str = vstype[(subtype as libc::c_int & 0xfi32) as usize].as_ptr();
          if subtype as libc::c_int & 0x10i32 != 0 {
            c = ':' as i32 as libc::c_uchar;
            current_block = 8845338526596852646;
          } else {
            current_block = 7610550352040274009;
          }
        }
      }
      39 | 92 | 34 | 36 => {
        cc[0] = c as libc::c_char;
        str = cc.as_mut_ptr();
        c = '\\' as i32 as libc::c_uchar;
        current_block = 8845338526596852646;
      }
      _ => {
        current_block = 8845338526596852646;
      }
    }
    match current_block {
      8845338526596852646 => {
        let fresh30 = nextc;
        nextc = nextc.offset(1);
        *fresh30 = c as libc::c_char;
        current_block = 7610550352040274009;
      }
      _ => {}
    }
    match current_block {
      7610550352040274009 => {
        if str.is_null() {
          continue;
        }
      }
      _ => {}
    }
    loop {
      let fresh31 = str;
      str = str.offset(1);
      c = *fresh31 as libc::c_uchar;
      if !(c as libc::c_int != '\u{0}' as i32) {
        break;
      }
      let fresh32 = nextc;
      nextc = nextc.offset(1);
      *fresh32 = c as libc::c_char
    }
  }
  if quoted & 1i32 != 0 {
    let fresh33 = nextc;
    nextc = nextc.offset(1);
    *fresh33 = '\"' as i32 as libc::c_char
  }
  *nextc = 0i32 as libc::c_char;
  cmdnextc = nextc;
}
unsafe extern "C" fn cmdlist(mut np: *mut node, mut sep: libc::c_int) {
  while !np.is_null() {
    if sep == 0 {
      cmdputs(b" \x00" as *const u8 as *const libc::c_char);
    }
    cmdtxt(np);
    if sep != 0 && !(*np).narg.next.is_null() {
      cmdputs(b" \x00" as *const u8 as *const libc::c_char);
    }
    np = (*np).narg.next
  }
}
unsafe extern "C" fn cmdtxt(mut n: *mut node) {
  let mut current_block: u64;
  let mut np: *mut node = 0 as *mut node;
  let mut lp: *mut nodelist = 0 as *mut nodelist;
  let mut p: *const libc::c_char = 0 as *const libc::c_char;
  if n.is_null() {
    return;
  }
  match (*n).type_0 as libc::c_int {
    7 => {
      p = b"; \x00" as *const u8 as *const libc::c_char;
      current_block = 15392876659706591536;
    }
    5 => {
      p = b" && \x00" as *const u8 as *const libc::c_char;
      current_block = 15392876659706591536;
    }
    6 => {
      p = b" || \x00" as *const u8 as *const libc::c_char;
      current_block = 15392876659706591536;
    }
    2 | 3 => {
      n = (*n).nredir.n;
      current_block = 1382307715582192433;
    }
    26 => {
      cmdputs(b"!\x00" as *const u8 as *const libc::c_char);
      n = (*n).nnot.com;
      current_block = 1382307715582192433;
    }
    8 => {
      cmdputs(b"if \x00" as *const u8 as *const libc::c_char);
      cmdtxt((*n).nif.test);
      cmdputs(b"; then \x00" as *const u8 as *const libc::c_char);
      if !(*n).nif.elsepart.is_null() {
        cmdtxt((*n).nif.ifpart);
        cmdputs(b"; else \x00" as *const u8 as *const libc::c_char);
        n = (*n).nif.elsepart
      } else {
        n = (*n).nif.ifpart
      }
      p = b"; fi\x00" as *const u8 as *const libc::c_char;
      current_block = 6079003566660255939;
    }
    4 => {
      cmdputs(b"(\x00" as *const u8 as *const libc::c_char);
      n = (*n).nredir.n;
      p = b")\x00" as *const u8 as *const libc::c_char;
      current_block = 6079003566660255939;
    }
    9 => {
      p = b"while \x00" as *const u8 as *const libc::c_char;
      current_block = 14376770496206129373;
    }
    10 => {
      p = b"until \x00" as *const u8 as *const libc::c_char;
      current_block = 14376770496206129373;
    }
    11 => {
      cmdputs(b"for \x00" as *const u8 as *const libc::c_char);
      cmdputs((*n).nfor.var);
      cmdputs(b" in \x00" as *const u8 as *const libc::c_char);
      cmdlist((*n).nfor.args, 1i32);
      n = (*n).nfor.body;
      p = b"; done\x00" as *const u8 as *const libc::c_char;
      current_block = 15181611551387615278;
    }
    14 => {
      cmdputs((*n).ndefun.text);
      p = b"() { ... }\x00" as *const u8 as *const libc::c_char;
      current_block = 1867401210745038125;
    }
    0 => {
      cmdlist((*n).ncmd.args, 1i32);
      cmdlist((*n).ncmd.redirect, 0i32);
      current_block = 17769492591016358583;
    }
    15 => {
      p = (*n).narg.text;
      current_block = 1867401210745038125;
    }
    24 | 25 => {
      p = b"<<...\x00" as *const u8 as *const libc::c_char;
      current_block = 1867401210745038125;
    }
    12 => {
      cmdputs(b"case \x00" as *const u8 as *const libc::c_char);
      cmdputs((*(*n).ncase.expr).narg.text);
      cmdputs(b" in \x00" as *const u8 as *const libc::c_char);
      np = (*n).ncase.cases;
      while !np.is_null() {
        cmdtxt((*np).nclist.pattern);
        cmdputs(b") \x00" as *const u8 as *const libc::c_char);
        cmdtxt((*np).nclist.body);
        cmdputs(b";; \x00" as *const u8 as *const libc::c_char);
        np = (*np).nclist.next
      }
      p = b"esac\x00" as *const u8 as *const libc::c_char;
      current_block = 1867401210745038125;
    }
    16 => {
      p = b">\x00" as *const u8 as *const libc::c_char;
      current_block = 14897169575956643397;
    }
    18 => {
      p = b">|\x00" as *const u8 as *const libc::c_char;
      current_block = 14897169575956643397;
    }
    21 => {
      p = b">>\x00" as *const u8 as *const libc::c_char;
      current_block = 14897169575956643397;
    }
    17 | 22 => {
      p = b">&\x00" as *const u8 as *const libc::c_char;
      current_block = 14897169575956643397;
    }
    19 => {
      p = b"<\x00" as *const u8 as *const libc::c_char;
      current_block = 14897169575956643397;
    }
    23 => {
      p = b"<&\x00" as *const u8 as *const libc::c_char;
      current_block = 14897169575956643397;
    }
    20 => {
      p = b"<>\x00" as *const u8 as *const libc::c_char;
      current_block = 14897169575956643397;
    }
    1 | _ => {
      lp = (*n).npipe.cmdlist;
      loop {
        cmdtxt((*lp).n);
        lp = (*lp).next;
        if lp.is_null() {
          break;
        }
        cmdputs(b" | \x00" as *const u8 as *const libc::c_char);
      }
      current_block = 17769492591016358583;
    }
  }
  match current_block {
    14897169575956643397 => {
      cmdputs(utoa((*n).nfile.fd as libc::c_uint));
      cmdputs(p);
      if (*n).type_0 as libc::c_int == 22i32 || (*n).type_0 as libc::c_int == 23i32 {
        if (*n).ndup.dupfd >= 0i32 {
          cmdputs(utoa((*n).ndup.dupfd as libc::c_uint));
        } else {
          cmdputs(b"-\x00" as *const u8 as *const libc::c_char);
        }
        current_block = 17769492591016358583;
      } else {
        n = (*n).nfile.fname;
        current_block = 1382307715582192433;
      }
    }
    14376770496206129373 => {
      cmdputs(p);
      cmdtxt((*n).nbinary.ch1);
      n = (*n).nbinary.ch2;
      p = b"; done\x00" as *const u8 as *const libc::c_char;
      current_block = 15181611551387615278;
    }
    15392876659706591536 => {
      cmdtxt((*n).nbinary.ch1);
      cmdputs(p);
      n = (*n).nbinary.ch2;
      current_block = 1382307715582192433;
    }
    _ => {}
  }
  match current_block {
    15181611551387615278 => {
      cmdputs(b"; do \x00" as *const u8 as *const libc::c_char);
      current_block = 6079003566660255939;
    }
    1382307715582192433 => {
      cmdtxt(n);
      current_block = 17769492591016358583;
    }
    _ => {}
  }
  match current_block {
    6079003566660255939 => {
      cmdtxt(n);
      current_block = 1867401210745038125;
    }
    _ => {}
  }
  match current_block {
    1867401210745038125 => {
      cmdputs(p);
    }
    _ => {}
  };
}
unsafe extern "C" fn commandtext(mut n: *mut node) -> *mut libc::c_char {
  let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
  cmdnextc = (*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut libc::c_char;
  cmdtxt(n);
  name = (*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut libc::c_char;
  return xstrdup(name);
}
unsafe extern "C" fn clear_traps() {
  let mut tp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  ::std::ptr::write_volatile(
    &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
    ::std::ptr::read_volatile::<libc::c_int>(
      &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
    ) + 1,
  );
  asm!("" : : : "memory" : "volatile");
  tp = (*ash_ptr_to_globals_misc).trap.as_mut_ptr();
  while tp
    < &mut *(*ash_ptr_to_globals_misc)
      .trap
      .as_mut_ptr()
      .offset((64i32 + 1i32) as isize) as *mut *mut libc::c_char
  {
    if !(*tp).is_null() && **tp as libc::c_int != 0 {
      if (*ash_ptr_to_globals_misc).trap_ptr == (*ash_ptr_to_globals_misc).trap.as_mut_ptr() {
        free(*tp as *mut libc::c_void);
      }
      *tp = 0 as *mut libc::c_char;
      if tp.wrapping_offset_from((*ash_ptr_to_globals_misc).trap.as_mut_ptr()) as libc::c_long
        != 0i32 as libc::c_long
      {
        setsignal(
          tp.wrapping_offset_from((*ash_ptr_to_globals_misc).trap.as_mut_ptr()) as libc::c_long
            as libc::c_int,
        );
      }
    }
    tp = tp.offset(1)
  }
  (*ash_ptr_to_globals_misc).may_have_traps = 0i32 as uint8_t;
  int_on();
}
#[inline(never)]
unsafe extern "C" fn forkchild(mut jp: *mut job, mut n: *mut node, mut mode: libc::c_int) {
  let mut oldlvl: libc::c_int = 0;
  oldlvl = (*ash_ptr_to_globals_misc).shlvl;
  (*ash_ptr_to_globals_misc).shlvl += 1;
  closescript();
  if mode == 2i32
    && !n.is_null()
    && (*n).type_0 as libc::c_int == 0i32
    && !(*n).ncmd.args.is_null()
    && strcmp(
      (*(*n).ncmd.args).narg.text,
      b"trap\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    && (*(*n).ncmd.args).narg.next.is_null()
  {
    (*ash_ptr_to_globals_misc).trap_ptr = xmemdup(
      (*ash_ptr_to_globals_misc).trap.as_mut_ptr() as *const libc::c_void,
      ::std::mem::size_of::<[*mut libc::c_char; 65]>() as libc::c_ulong as libc::c_int,
    ) as *mut *mut libc::c_char
  }
  clear_traps();
  doing_jobctl = 0i32 as smallint;
  if mode != 2i32 && (*jp).jobctl() as libc::c_int != 0 && oldlvl == 0i32 {
    let mut pgrp: pid_t = 0;
    if (*jp).nprocs == 0i32 as libc::c_uint {
      pgrp = getpid()
    } else {
      pgrp = (*(*jp).ps.offset(0)).ps_pid
    }
    setpgid(0i32, pgrp);
    if mode == 0i32 {
      xtcsetpgrp(ttyfd, pgrp);
    }
    setsignal(20i32);
    setsignal(22i32);
  } else if mode == 1i32 {
    ignoresig(2i32);
    ignoresig(3i32);
    if (*jp).nprocs == 0i32 as libc::c_uint {
      close(0i32);
      if open(b"/dev/null\x00" as *const u8 as *const libc::c_char, 0i32) != 0i32 {
        ash_msg_and_raise_error(
          b"can\'t open \'%s\': %m\x00" as *const u8 as *const libc::c_char,
          b"/dev/null\x00" as *const u8 as *const libc::c_char,
        );
      }
    }
  }
  if oldlvl == 0i32 {
    if (*ash_ptr_to_globals_misc).optlist[3] != 0 {
      setsignal(2i32);
      setsignal(15i32);
    }
    setsignal(3i32);
  }
  if !n.is_null()
    && (*n).type_0 as libc::c_int == 0i32
    && !(*n).ncmd.args.is_null()
    && strcmp(
      (*(*n).ncmd.args).narg.text,
      b"jobs\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
  {
    freejob(curjob);
    return;
  }
  jp = curjob;
  while !jp.is_null() {
    freejob(jp);
    jp = (*jp).prev_job
  }
  jobless = 0i32;
}
unsafe extern "C" fn forkparent(
  mut jp: *mut job,
  mut n: *mut node,
  mut mode: libc::c_int,
  mut pid: pid_t,
) {
  if jp.is_null() {
    while jobless != 0 && dowait(0i32, 0 as *mut job) > 0i32 {}
    jobless += 1;
    return;
  }
  if mode != 2i32 && (*jp).jobctl() as libc::c_int != 0 {
    let mut pgrp: libc::c_int = 0;
    if (*jp).nprocs == 0i32 as libc::c_uint {
      pgrp = pid
    } else {
      pgrp = (*(*jp).ps.offset(0)).ps_pid
    }
    setpgid(pid, pgrp);
  }
  if mode == 1i32 {
    (*ash_ptr_to_globals_misc).backgndpid = pid;
    set_curjob(jp, 1i32 as libc::c_uint);
  }
  if !jp.is_null() {
    let fresh34 = (*jp).nprocs;
    (*jp).nprocs = (*jp).nprocs.wrapping_add(1);
    let mut ps: *mut procstat = &mut *(*jp).ps.offset(fresh34 as isize) as *mut procstat;
    (*ps).ps_pid = pid;
    (*ps).ps_status = -1i32;
    (*ps).ps_cmd = (*ash_ptr_to_globals_misc).nullstr.as_mut_ptr();
    if doing_jobctl as libc::c_int != 0 && !n.is_null() {
      (*ps).ps_cmd = commandtext(n)
    }
  };
}
unsafe extern "C" fn forkshell(
  mut jp: *mut job,
  mut n: *mut node,
  mut mode: libc::c_int,
) -> libc::c_int {
  let mut pid: libc::c_int = 0;
  pid = fork();
  if pid < 0i32 {
    if !jp.is_null() {
      freejob(jp);
    }
    ash_msg_and_raise_error(b"can\'t fork: %m\x00" as *const u8 as *const libc::c_char);
  }
  if pid == 0i32 {
    (*ash_ptr_to_globals_misc).random_gen.galois_LFSR = 0i32;
    forkchild(jp, n, mode);
  } else {
    forkparent(jp, n, mode, pid);
  }
  return pid;
}
unsafe extern "C" fn waitforjob(mut jp: *mut job) -> libc::c_int {
  let mut st: libc::c_int = 0;
  ::std::ptr::write_volatile(
    &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
    ::std::ptr::read_volatile::<libc::c_int>(
      &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
    ) + 1,
  );
  asm!("" : : : "memory" : "volatile");
  while (*jp).state() as libc::c_int == 0i32 {
    dowait(1i32, jp);
  }
  int_on();
  st = getstatus(jp);
  if (*jp).jobctl() != 0 {
    xtcsetpgrp(ttyfd, (*ash_ptr_to_globals_misc).rootpid);
    if (*jp).sigint() != 0 {
      raise(2i32);
    }
  }
  if (*jp).state() as libc::c_int == 2i32 {
    freejob(jp);
  }
  return st;
}
unsafe extern "C" fn stoppedjobs() -> libc::c_int {
  let mut jp: *mut job = 0 as *mut job;
  let mut retval: libc::c_int = 0;
  retval = 0i32;
  if !((*ash_ptr_to_globals_misc).job_warning != 0) {
    jp = curjob;
    if !jp.is_null() && (*jp).state() as libc::c_int == 1i32 {
      out2str(b"You have stopped jobs.\n\x00" as *const u8 as *const libc::c_char);
      (*ash_ptr_to_globals_misc).job_warning = 2i32 as smallint;
      retval += 1
    }
  }
  return retval;
}
unsafe extern "C" fn openhere(mut redir: *mut node) -> libc::c_int {
  let mut current_block: u64;
  let mut pip: [libc::c_int; 2] = [0; 2];
  let mut len: size_t = 0i32 as size_t;
  if pipe(pip.as_mut_ptr()) < 0i32 {
    ash_msg_and_raise_error(b"can\'t create pipe: %m\x00" as *const u8 as *const libc::c_char);
  }
  if (*redir).type_0 as libc::c_int == 24i32 {
    len = strlen((*(*redir).nhere.doc).narg.text);
    if len <= 4096i32 as libc::c_ulong {
      full_write(
        pip[1],
        (*(*redir).nhere.doc).narg.text as *const libc::c_void,
        len,
      );
      current_block = 1772726115963952798;
    } else {
      current_block = 7815301370352969686;
    }
  } else {
    current_block = 7815301370352969686;
  }
  match current_block {
    7815301370352969686 => {
      if forkshell(
        0 as *mut libc::c_void as *mut job,
        0 as *mut libc::c_void as *mut node,
        2i32,
      ) == 0i32
      {
        close(pip[0]);
        ignoresig(2i32);
        ignoresig(3i32);
        ignoresig(1i32);
        ignoresig(20i32);
        signal(13i32, None);
        if (*redir).type_0 as libc::c_int == 24i32 {
          full_write(
            pip[1],
            (*(*redir).nhere.doc).narg.text as *const libc::c_void,
            len,
          );
        } else {
          expandhere((*redir).nhere.doc, pip[1]);
        }
        _exit(0i32);
      }
    }
    _ => {}
  }
  close(pip[1]);
  return pip[0];
}
unsafe extern "C" fn openredirect(mut redir: *mut node) -> libc::c_int {
  let mut current_block: u64;
  let mut sb: stat = stat {
    st_dev: 0,
    st_ino: 0,
    st_nlink: 0,
    st_mode: 0,
    st_uid: 0,
    st_gid: 0,
    __pad0: 0,
    st_rdev: 0,
    st_size: 0,
    st_blksize: 0,
    st_blocks: 0,
    st_atim: timespec {
      tv_sec: 0,
      tv_nsec: 0,
    },
    st_mtim: timespec {
      tv_sec: 0,
      tv_nsec: 0,
    },
    st_ctim: timespec {
      tv_sec: 0,
      tv_nsec: 0,
    },
    __glibc_reserved: [0; 3],
  };
  let mut fname: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut f: libc::c_int = 0;
  match (*redir).nfile.type_0 as libc::c_int {
    24 | 25 => return openhere(redir),
    _ => {}
  }
  fname = (*redir).nfile.expfname;
  match (*redir).nfile.type_0 as libc::c_int {
    20 => {
      f = open(fname, 0o2i32 | 0o100i32, 0o666i32);
      if f < 0i32 {
        current_block = 402047871037399430;
      } else {
        current_block = 11913429853522160501;
      }
    }
    16 | 17 => {
      if (*ash_ptr_to_globals_misc).optlist[10] != 0 {
        if stat(fname, &mut sb) < 0i32 {
          f = open(fname, 0o1i32 | 0o100i32 | 0o200i32, 0o666i32);
          if f < 0i32 {
            current_block = 402047871037399430;
          } else {
            current_block = 11913429853522160501;
          }
        } else if !(sb.st_mode & 0o170000i32 as libc::c_uint == 0o100000i32 as libc::c_uint) {
          f = open(fname, 0o1i32, 0o666i32);
          if f < 0i32 {
            current_block = 402047871037399430;
          } else if fstat(f, &mut sb) == 0
            && sb.st_mode & 0o170000i32 as libc::c_uint == 0o100000i32 as libc::c_uint
          {
            close(f);
            *bb_errno = 17i32;
            current_block = 402047871037399430;
          } else {
            current_block = 11913429853522160501;
          }
        } else {
          *bb_errno = 17i32;
          current_block = 402047871037399430;
        }
      } else {
        current_block = 9509918723278613251;
      }
    }
    18 => {
      current_block = 9509918723278613251;
    }
    21 => {
      f = open(fname, 0o1i32 | 0o100i32 | 0o2000i32, 0o666i32);
      if f < 0i32 {
        current_block = 402047871037399430;
      } else {
        current_block = 11913429853522160501;
      }
    }
    19 | _ => {
      f = open(fname, 0i32);
      if f < 0i32 {
        ash_msg_and_raise_error(
          b"can\'t open %s: %s\x00" as *const u8 as *const libc::c_char,
          fname,
          errmsg(
            *bb_errno,
            b"no such file\x00" as *const u8 as *const libc::c_char,
          ),
        );
      } else {
        current_block = 11913429853522160501;
      }
    }
  }
  match current_block {
    9509918723278613251 => {
      f = open(fname, 0o1i32 | 0o100i32 | 0o1000i32, 0o666i32);
      if f < 0i32 {
        current_block = 402047871037399430;
      } else {
        current_block = 11913429853522160501;
      }
    }
    _ => {}
  }
  match current_block {
    11913429853522160501 => return f,
    _ => {
      ash_msg_and_raise_error(
        b"can\'t create %s: %s\x00" as *const u8 as *const libc::c_char,
        fname,
        errmsg(
          *bb_errno,
          b"nonexistent directory\x00" as *const u8 as *const libc::c_char,
        ),
      );
    }
  };
}
unsafe extern "C" fn savefd(mut from: libc::c_int) -> libc::c_int {
  let mut newfd: libc::c_int = 0;
  let mut err: libc::c_int = 0;
  newfd = fcntl(from, 1030i32, 10i32);
  err = if newfd < 0i32 { *bb_errno } else { 0i32 };
  if err != 9i32 {
    if err != 0 {
      ash_msg_and_raise_error(b"%d: %m\x00" as *const u8 as *const libc::c_char, from);
    }
    close(from);
    if 1030i32 == 0i32 {
      close_on_exec_on(newfd);
    }
  }
  return newfd;
}
unsafe extern "C" fn dup2_or_raise(mut from: libc::c_int, mut to: libc::c_int) -> libc::c_int {
  let mut newfd: libc::c_int = 0;
  newfd = if from != to { dup2(from, to) } else { to };
  if newfd < 0i32 {
    ash_msg_and_raise_error(b"%d: %m\x00" as *const u8 as *const libc::c_char, from);
  }
  return newfd;
}
unsafe extern "C" fn dup_CLOEXEC(mut fd: libc::c_int, mut avoid_fd: libc::c_int) -> libc::c_int {
  let mut newfd: libc::c_int = 0;
  loop {
    newfd = fcntl(fd, 1030i32, avoid_fd + 1i32);
    if newfd >= 0i32 {
      if 1030i32 == 0i32 {
        close_on_exec_on(newfd);
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
      if *bb_errno == 9i32 {
        return fd;
      }
      ash_msg_and_raise_error(b"%d: %m\x00" as *const u8 as *const libc::c_char, newfd);
    } else {
      if 1030i32 == 0i32 {
        close_on_exec_on(newfd);
      }
      close(fd);
      return newfd;
    }
  }
}
unsafe extern "C" fn add_squirrel_closed(mut sq: *mut redirtab, mut fd: libc::c_int) {
  let mut i: libc::c_int = 0;
  if sq.is_null() {
    return;
  }
  i = 0i32;
  while (*(*sq).two_fd.as_mut_ptr().offset(i as isize)).orig_fd != -2i32 {
    /* If we collide with an already moved fd... */
    if fd == (*(*sq).two_fd.as_mut_ptr().offset(i as isize)).orig_fd {
      /* Examples:
       * "echo 3>FILE 3>&- 3>FILE"
       * "echo 3>&- 3>FILE"
       * No need for last redirect to insert
       * another "need to close 3" indicator.
       */
      return;
    }
    i += 1
  }
  (*(*sq).two_fd.as_mut_ptr().offset(i as isize)).orig_fd = fd;
  (*(*sq).two_fd.as_mut_ptr().offset(i as isize)).moved_to = -1i32;
}
unsafe extern "C" fn save_fd_on_redirect(
  mut fd: libc::c_int,
  mut avoid_fd: libc::c_int,
  mut sq: *mut redirtab,
) -> libc::c_int {
  let mut i: libc::c_int = 0;
  let mut new_fd: libc::c_int = 0;
  if avoid_fd < 9i32 {
    /* the important case here is that it can be -1 */
    avoid_fd = 9i32
  }
  if fd == ttyfd {
    /* Testcase: "ls -l /proc/$$/fd 10>&-" should work */
    ttyfd = xdup_CLOEXEC_and_close(ttyfd, avoid_fd);
    return 1i32;
    /* "we closed fd" */
  }
  /* Are we called from redirect(0)? E.g. redirect
   * in a forked child. No need to save fds,
   * we aren't going to use them anymore, ok to trash.
   */
  if sq.is_null() {
    return 0i32;
  }
  /* If this one of script's fds? */
  if fd != 0i32 {
    let mut pf: *mut parsefile = g_parsefile;
    while !pf.is_null() {
      /* We skip fd == 0 case because of the following:
       * $ ash  # running ash interactively
       * $ . ./script.sh
       * and in script.sh: "exec 9>&0".
       * Even though top-level pf_fd _is_ 0,
       * it's still ok to use it: "read" builtin uses it,
       * why should we cripple "exec" builtin?
       */
      if fd == (*pf).pf_fd {
        (*pf).pf_fd = xdup_CLOEXEC_and_close(fd, avoid_fd);
        return 1i32;
        /* "we closed fd" */
      }
      pf = (*pf).prev
    }
  }
  /* Check whether it collides with any open fds (e.g. stdio), save fds as needed */
  /* First: do we collide with some already moved fds? */
  i = 0i32;
  while (*(*sq).two_fd.as_mut_ptr().offset(i as isize)).orig_fd != -2i32 {
    /* If we collide with an already moved fd... */
    if fd == (*(*sq).two_fd.as_mut_ptr().offset(i as isize)).moved_to {
      new_fd = dup_CLOEXEC(fd, avoid_fd);
      (*(*sq).two_fd.as_mut_ptr().offset(i as isize)).moved_to = new_fd;
      if new_fd < 0i32 {
        /* "we did not close fd" */
        /* what? */
        xfunc_die();
      }
      return 0i32;
    }
    if fd == (*(*sq).two_fd.as_mut_ptr().offset(i as isize)).orig_fd {
      return 0i32;
      /* "we did not close fd" */
    }
    i += 1
  }
  /* If this fd is open, we move and remember it; if it's closed, new_fd = CLOSED (-1) */
  new_fd = dup_CLOEXEC(fd, avoid_fd);
  if new_fd < 0i32 {
    if *bb_errno != 9i32 {
      xfunc_die();
    }
    /* new_fd = CLOSED; - already is -1 */
  }
  (*(*sq).two_fd.as_mut_ptr().offset(i as isize)).moved_to = new_fd;
  (*(*sq).two_fd.as_mut_ptr().offset(i as isize)).orig_fd = fd;
  /* if we move stderr, let "set -x" code know */
  if fd == (*ash_ptr_to_globals_var).preverrout_fd {
    (*ash_ptr_to_globals_var).preverrout_fd = new_fd
  }
  return 0i32;
  /* "we did not close fd" */
}
unsafe extern "C" fn internally_opened_fd(
  mut fd: libc::c_int,
  mut sq: *mut redirtab,
) -> libc::c_int {
  let mut i: libc::c_int = 0;
  if fd == ttyfd {
    return 1i32;
  }
  /* If this one of script's fds? */
  if fd != 0i32 {
    let mut pf: *mut parsefile = g_parsefile;
    while !pf.is_null() {
      if fd == (*pf).pf_fd {
        return 1i32;
      }
      pf = (*pf).prev
    }
  }
  if !sq.is_null() {
    i = 0i32;
    while i < (*sq).pair_count && (*(*sq).two_fd.as_mut_ptr().offset(i as isize)).orig_fd != -2i32 {
      if fd == (*(*sq).two_fd.as_mut_ptr().offset(i as isize)).moved_to {
        return 1i32;
      }
      i += 1
    }
  }
  return 0i32;
}
/* save previous values of file descriptors */
unsafe extern "C" fn redirect(mut redir: *mut node, mut flags: libc::c_int) {
  let mut sv: *mut redirtab = 0 as *mut redirtab;
  if redir.is_null() {
    return;
  }
  sv = 0 as *mut redirtab;
  ::std::ptr::write_volatile(
    &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
    ::std::ptr::read_volatile::<libc::c_int>(
      &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
    ) + 1,
  );
  asm!("" : : : "memory" : "volatile");
  if flags & 0o1i32 != 0 {
    sv = (*ash_ptr_to_globals_var).redirlist
  }
  let mut current_block_30: u64;
  loop {
    let mut fd: libc::c_int = 0;
    let mut newfd: libc::c_int = 0;
    let mut close_fd: libc::c_int = 0;
    let mut closed: libc::c_int = 0;
    fd = (*redir).nfile.fd;
    if (*redir).nfile.type_0 as libc::c_int == 22i32
      || (*redir).nfile.type_0 as libc::c_int == 23i32
    {
      //bb_error_msg("doing %d > %d", fd, newfd);
      newfd = (*redir).ndup.dupfd; /* always >= 0 */
      close_fd = -1i32;
      current_block_30 = 224731115979188411;
    } else {
      newfd = openredirect(redir);
      if fd == newfd {
        /* open() gave us precisely the fd we wanted.
         * This means that this fd was not busy
         * (not opened to anywhere).
         * Remember to close it on restore:
         */
        add_squirrel_closed(sv, fd);
        current_block_30 = 5720623009719927633;
      } else {
        close_fd = newfd;
        current_block_30 = 224731115979188411;
      }
    }
    match current_block_30 {
      224731115979188411 => {
        if !(fd == newfd) {
          loop
          /* if "N>FILE": move newfd to fd */
          /* if "N>&M": dup newfd to fd */
          /* if "N>&-": close fd (newfd is -1) */
          {
            closed = save_fd_on_redirect(fd, newfd, sv);
            if newfd == -1i32 {
              /* "N>&-" means "close me" */
              if closed == 0 {
                /* ^^^ optimization: saving may already
                 * have closed it. If not... */
                close(fd);
              }
              break;
            } else {
              /* if newfd is a script fd or saved fd, simulate EBADF */
              if internally_opened_fd(newfd, sv) != 0 {
                *bb_errno = 9i32;
                ash_msg_and_raise_error(b"%d: %m\x00" as *const u8 as *const libc::c_char, newfd);
              }
              dup2_or_raise(newfd, fd);
              if close_fd >= 0i32 {
                /* "N>FILE" or ">&FILE" or heredoc? */
                close(close_fd);
              }
              if !((*redir).nfile.type_0 as libc::c_int == 17i32 && fd == 1i32) {
                break;
              }
              /* ">&FILE". we already redirected to 1, now copy 1 to 2 */
              fd = 2i32;
              newfd = 1i32;
              close_fd = -1i32
            }
          }
        }
      }
      _ => {}
    }
    redir = (*redir).nfile.next;
    if redir.is_null() {
      break;
    }
  }
  int_on();
  //dash:#define REDIR_SAVEFD2 03        /* set preverrout */
  // dash has a bug: since REDIR_SAVEFD2=3 and REDIR_PUSH=1, this test
  // triggers for pure REDIR_PUSH too. Thus, this is done almost always,
  // not only for calls with flags containing REDIR_SAVEFD2.
  // We do this unconditionally (see save_fd_on_redirect()).
  //if ((flags & REDIR_SAVEFD2) && copied_fd2 >= 0)
  //	preverrout_fd = copied_fd2;
}
unsafe extern "C" fn redirectsafe(mut redir: *mut node, mut flags: libc::c_int) -> libc::c_int {
  let mut err: libc::c_int = 0;
  let mut saveint: libc::c_int = 0;
  let mut savehandler: *mut jmploc = (*ash_ptr_to_globals_misc).exception_handler;
  let mut jmploc: jmploc = jmploc {
    loc: [__jmp_buf_tag {
      __jmpbuf: [0; 8],
      __mask_was_saved: 0,
      __saved_mask: __sigset_t { __val: [0; 16] },
    }; 1],
  };
  ::std::ptr::write_volatile(
    &mut saveint as *mut libc::c_int,
    (*ash_ptr_to_globals_misc).suppress_int,
  );
  /* "echo 9>/dev/null; echo >&9; echo result: $?" - result should be 1, not 2! */
  err = _setjmp(jmploc.loc.as_mut_ptr()); /* was = setjmp(jmploc.loc) * 2; */
  if err == 0 {
    (*ash_ptr_to_globals_misc).exception_handler = &mut jmploc;
    redirect(redir, flags);
  }
  (*ash_ptr_to_globals_misc).exception_handler = savehandler;
  if err != 0 && (*ash_ptr_to_globals_misc).exception_type as libc::c_int != 1i32 {
    longjmp(
      (*(*ash_ptr_to_globals_misc).exception_handler)
        .loc
        .as_mut_ptr(),
      1i32,
    );
  }
  asm!("" : : : "memory" : "volatile");
  ::std::ptr::write_volatile(
    &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
    saveint,
  );
  if (*ash_ptr_to_globals_misc).suppress_int == 0i32
    && (*ash_ptr_to_globals_misc).pending_int as libc::c_int != 0
  {
    raise_interrupt();
  }
  return err;
}
unsafe extern "C" fn pushredir(mut redir: *mut node) -> *mut redirtab {
  let mut sv: *mut redirtab = 0 as *mut redirtab;
  let mut i: libc::c_int = 0;
  if redir.is_null() {
    return (*ash_ptr_to_globals_var).redirlist;
  }
  i = 0i32;
  loop {
    i += 1;
    if (*redir).nfile.type_0 as libc::c_int == 17i32 {
      i += 1
    }
    redir = (*redir).nfile.next;
    if redir.is_null() {
      break;
    }
  }
  sv = xzalloc(
    (::std::mem::size_of::<redirtab>() as libc::c_ulong).wrapping_add(
      (i as libc::c_ulong).wrapping_mul(::std::mem::size_of::<squirrel>() as libc::c_ulong),
    ),
  ) as *mut redirtab;
  (*sv).pair_count = i;
  loop {
    i -= 1;
    if !(i >= 0i32) {
      break;
    }
    let ref mut fresh35 = (*(*sv).two_fd.as_mut_ptr().offset(i as isize)).moved_to;
    *fresh35 = -2i32;
    (*(*sv).two_fd.as_mut_ptr().offset(i as isize)).orig_fd = *fresh35
  }
  (*sv).next = (*ash_ptr_to_globals_var).redirlist;
  (*ash_ptr_to_globals_var).redirlist = sv;
  return (*sv).next;
}
/*
 * Undo the effects of the last redirection.
 */
unsafe extern "C" fn popredir(mut drop_0: libc::c_int) {
  let mut rp: *mut redirtab = 0 as *mut redirtab;
  let mut i: libc::c_int = 0;
  if (*ash_ptr_to_globals_var).redirlist.is_null() {
    return;
  }
  ::std::ptr::write_volatile(
    &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
    ::std::ptr::read_volatile::<libc::c_int>(
      &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
    ) + 1,
  );
  asm!("" : : : "memory" : "volatile");
  rp = (*ash_ptr_to_globals_var).redirlist;
  i = 0i32;
  while i < (*rp).pair_count {
    let mut fd: libc::c_int = (*(*rp).two_fd.as_mut_ptr().offset(i as isize)).orig_fd;
    let mut copy: libc::c_int = (*(*rp).two_fd.as_mut_ptr().offset(i as isize)).moved_to;
    if copy == -1i32 {
      if drop_0 == 0 {
        close(fd);
      }
    } else if copy != -2i32 {
      if drop_0 == 0 {
        /*close(fd);*/
        dup2_or_raise(copy, fd);
      }
      close(copy);
    }
    i += 1
  }
  (*ash_ptr_to_globals_var).redirlist = (*rp).next;
  free(rp as *mut libc::c_void);
  int_on();
}
unsafe extern "C" fn unwindredir(mut stop: *mut redirtab) {
  while (*ash_ptr_to_globals_var).redirlist != stop {
    popredir(0i32);
  }
}
/* ============ Routines to expand arguments to commands
 *
 * We have to deal with backquotes, shell variables, and file metacharacters.
 */
unsafe extern "C" fn ash_arith(mut s: *const libc::c_char) -> arith_t {
  let mut math_state: arith_state_t = arith_state_t {
    errmsg: 0 as *const libc::c_char,
    lookupvar: None,
    setvar: None,
    list_of_recursed_names: 0 as *mut libc::c_void,
  };
  let mut result: arith_t = 0;
  math_state.lookupvar =
    Some(lookupvar as unsafe extern "C" fn(_: *const libc::c_char) -> *const libc::c_char);
  math_state.setvar =
    Some(setvar0 as unsafe extern "C" fn(_: *const libc::c_char, _: *const libc::c_char) -> ());
  //math_state.endofname = endofname;
  ::std::ptr::write_volatile(
    &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
    ::std::ptr::read_volatile::<libc::c_int>(
      &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
    ) + 1,
  );
  asm!("" : : : "memory" : "volatile");
  result = arith(&mut math_state, s);
  if !math_state.errmsg.is_null() {
    ash_msg_and_raise_error(math_state.errmsg);
  }
  int_on();
  return result;
}
unsafe extern "C" fn substr_atoi(mut s: *const libc::c_char) -> libc::c_int {
  let mut t: arith_t = ash_arith(s);
  if ::std::mem::size_of::<arith_t>() as libc::c_ulong
    > ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
  {
    /* clamp very large or very large negative nums for ${v:N:M}:
     * else "${v:0:0x100000001}" would work as "${v:0:1}"
     */
    if t > 2147483647i32 as libc::c_longlong {
      t = 2147483647i32 as arith_t
    }
    if t < (-2147483647i32 - 1i32) as libc::c_longlong {
      t = (-2147483647i32 - 1i32) as arith_t
    }
  }
  return t as libc::c_int;
}
/* output of current string */
static mut expdest: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
/* list of back quote expressions */
static mut argbackq: *mut nodelist = 0 as *const nodelist as *mut nodelist;
/* first struct in list of ifs regions */
static mut ifsfirst: ifsregion = ifsregion {
  next: 0 as *const ifsregion as *mut ifsregion,
  begoff: 0,
  endoff: 0,
  nulonly: 0,
};
/* last struct in list */
static mut ifslastp: *mut ifsregion = 0 as *const ifsregion as *mut ifsregion;
/* holds expanded arg list */
static mut exparg: arglist = arglist {
  list: 0 as *const strlist as *mut strlist,
  lastp: 0 as *const *mut strlist as *mut *mut strlist,
};
/*
 * Our own itoa().
 * cvtnum() is used even if math support is off (to prepare $? values and such).
 */
unsafe extern "C" fn cvtnum(mut num: arith_t) -> libc::c_int {
  let mut len: libc::c_int = 0;
  /* 32-bit and wider ints require buffer size of bytes*3 (or less) */
  len = (::std::mem::size_of::<arith_t>() as libc::c_ulong).wrapping_mul(3i32 as libc::c_ulong)
    as libc::c_int;
  /* If narrower: worst case, 1-byte ints: need 5 bytes: "-127<NUL>" */
  if (::std::mem::size_of::<arith_t>() as libc::c_ulong) < 4i32 as libc::c_ulong {
    len += 2i32
  }
  expdest = makestrspace(len as size_t, expdest);
  len = fmtstr(
    expdest,
    len as size_t,
    b"%lld\x00" as *const u8 as *const libc::c_char,
    num,
  );
  expdest = expdest.offset(len as isize);
  return len;
}
/*
 * Break the argument string into pieces based upon IFS and add the
 * strings to the argument list.  The regions of the string to be
 * searched for IFS characters have been stored by recordregion.
 */
unsafe extern "C" fn ifsbreakup(mut string: *mut libc::c_char, mut arglist: *mut arglist) {
  let mut current_block: u64; /* while */
  let mut ifsp: *mut ifsregion = 0 as *mut ifsregion;
  let mut sp: *mut strlist = 0 as *mut strlist;
  let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut ifs: *const libc::c_char = 0 as *const libc::c_char;
  let mut realifs: *const libc::c_char = 0 as *const libc::c_char;
  let mut ifsspc: libc::c_int = 0;
  let mut nulonly: libc::c_int = 0;
  start = string;
  if !ifslastp.is_null() {
    ifsspc = 0i32;
    nulonly = 0i32;
    realifs = if (*ash_ptr_to_globals_var).varinit[0].flags & 0x20i32 == 0i32 {
      (*ash_ptr_to_globals_var).varinit[0].var_text.offset(4)
    } else {
      defifsvar.as_ptr().offset(4)
    };
    ifsp = &mut ifsfirst;
    loop {
      let mut afternul: libc::c_int = 0;
      p = string.offset((*ifsp).begoff as isize);
      afternul = nulonly;
      nulonly = (*ifsp).nulonly;
      ifs = if nulonly != 0 {
        (*ash_ptr_to_globals_misc).nullstr.as_mut_ptr()
      } else {
        realifs
      };
      ifsspc = 0i32;
      while p < string.offset((*ifsp).endoff as isize) {
        q = p;
        if *p as libc::c_uchar as libc::c_int == '\u{81}' as i32 as libc::c_uchar as libc::c_int {
          p = p.offset(1)
        }
        if strchr(ifs, *p as libc::c_int).is_null() {
          p = p.offset(1)
        } else {
          if !(afternul != 0 || nulonly != 0) {
            ifsspc = (strchr(defifsvar.as_ptr().offset(4), *p as libc::c_int)
              != 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int
          }
          /* Ignore IFS whitespace at start */
          if q == start && ifsspc != 0 {
            p = p.offset(1);
            start = p
          } else {
            *q = '\u{0}' as i32 as libc::c_char;
            sp = stzalloc(::std::mem::size_of::<strlist>() as libc::c_ulong) as *mut strlist;
            (*sp).text = start;
            *(*arglist).lastp = sp;
            (*arglist).lastp = &mut (*sp).next;
            p = p.offset(1);
            if nulonly == 0 {
              while !(p >= string.offset((*ifsp).endoff as isize)) {
                q = p;
                if *p as libc::c_uchar as libc::c_int
                  == '\u{81}' as i32 as libc::c_uchar as libc::c_int
                {
                  p = p.offset(1)
                }
                if strchr(ifs, *p as libc::c_int).is_null() {
                  p = q;
                  break;
                } else if strchr(defifsvar.as_ptr().offset(4), *p as libc::c_int).is_null() {
                  if ifsspc != 0 {
                    p = p.offset(1);
                    ifsspc = 0i32
                  } else {
                    p = q;
                    break;
                  }
                } else {
                  p = p.offset(1)
                }
              }
            }
            start = p
          }
        }
      }
      ifsp = (*ifsp).next;
      if ifsp.is_null() {
        break;
      }
    }
    if nulonly != 0 {
      current_block = 4326832952511410360;
    } else {
      current_block = 7343950298149844727;
    }
  } else {
    current_block = 7343950298149844727;
  }
  match current_block {
    7343950298149844727 => {
      if *start == 0 {
        return;
      }
    }
    _ => {}
  }
  sp = stzalloc(::std::mem::size_of::<strlist>() as libc::c_ulong) as *mut strlist;
  (*sp).text = start;
  *(*arglist).lastp = sp;
  (*arglist).lastp = &mut (*sp).next;
}
unsafe extern "C" fn ifsfree() {
  let mut p: *mut ifsregion = ifsfirst.next;
  if !p.is_null() {
    ::std::ptr::write_volatile(
      &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
      ::std::ptr::read_volatile::<libc::c_int>(
        &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
      ) + 1,
    );
    asm!("" : : : "memory" : "volatile");
    loop {
      let mut ifsp: *mut ifsregion = 0 as *mut ifsregion;
      ifsp = (*p).next;
      free(p as *mut libc::c_void);
      p = ifsp;
      if p.is_null() {
        break;
      }
    }
    ifsfirst.next = 0 as *mut ifsregion;
    int_on();
  }
  ifslastp = 0 as *mut ifsregion;
}
unsafe extern "C" fn esclen(mut start: *const libc::c_char, mut p: *const libc::c_char) -> size_t {
  let mut esc: size_t = 0i32 as size_t;
  while p > start && {
    p = p.offset(-1);
    (*p as libc::c_uchar as libc::c_int) == '\u{81}' as i32 as libc::c_uchar as libc::c_int
  } {
    esc = esc.wrapping_add(1)
  }
  return esc;
}
/*
 * Remove any CTLESC characters from a string.
 */
unsafe extern "C" fn rmescapes(
  mut str: *mut libc::c_char,
  mut flag: libc::c_int,
  mut slash_position: *mut libc::c_int,
) -> *mut libc::c_char {
  static mut qchars: [libc::c_char; 4] = [
    '/' as i32 as libc::c_char,
    '\u{81}' as i32 as libc::c_uchar as libc::c_char,
    '\u{88}' as i32 as libc::c_uchar as libc::c_char,
    '\u{0}' as i32 as libc::c_char,
  ];
  let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut r: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut protect_against_glob: libc::c_uint = 0;
  let mut globbing: libc::c_uint = 0;
  p = strpbrk(
    str,
    qchars
      .as_ptr()
      .offset(slash_position.is_null() as libc::c_int as isize),
  );
  if p.is_null() {
    return str;
  }
  q = p;
  r = str;
  if flag & 0x1i32 != 0 {
    let mut len: size_t = p.wrapping_offset_from(str) as libc::c_long as size_t;
    let mut fulllen: size_t = len
      .wrapping_add(strlen(p))
      .wrapping_add(1i32 as libc::c_ulong);
    if flag & 0x8i32 != 0 {
      let mut strloc: libc::c_int = str.wrapping_offset_from(
        (*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut libc::c_char,
      ) as libc::c_long as libc::c_int;
      r = makestrspace(fulllen, expdest);
      /* p and str may be invalidated by makestrspace */
      str = ((*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut libc::c_char)
        .offset(strloc as isize);
      p = str.offset(len as isize)
    } else if flag & 0x10i32 != 0 {
      r = xmalloc(fulllen) as *mut libc::c_char
    } else {
      r = stalloc(fulllen) as *mut libc::c_char
    }
    q = r;
    if len > 0i32 as libc::c_ulong {
      q = mempcpy(q as *mut libc::c_void, str as *const libc::c_void, len) as *mut libc::c_char
    }
  }
  globbing = (flag & 0x2i32) as libc::c_uint;
  protect_against_glob = globbing;
  while *p != 0 {
    if *p as libc::c_uchar as libc::c_int == '\u{88}' as i32 as libc::c_uchar as libc::c_int {
      // Note: protect_against_glob only affect whether
      // CTLESC,<ch> gets converted to <ch> or to \<ch>
      p = p.offset(1);
      protect_against_glob = globbing
    } else {
      if *p as libc::c_int == '\\' as i32 {
        /* naked back slash */
        protect_against_glob = 0i32 as libc::c_uint
      } else {
        if *p as libc::c_uchar as libc::c_int == '\u{81}' as i32 as libc::c_uchar as libc::c_int {
          p = p.offset(1);
          if protect_against_glob != 0 {
            /*
             * We used to trust glob() and fnmatch() to eat
             * superfluous escapes (\z where z has no
             * special meaning anyway). But this causes
             * bugs such as string of one greek letter rho
             * (unicode-encoded as two bytes "cf,81")
             * getting encoded as "cf,CTLESC,81"
             * and here, converted to "cf,\,81" -
             * which does not go well with some flavors
             * of fnmatch() in unicode locales
             * (for example, glibc <= 2.22).
             *
             * Lets add "\" only on the chars which need it.
             * Testcases for less obvious chars are shown.
             */
            if *p as libc::c_int == '*' as i32
              || *p as libc::c_int == '?' as i32
              || *p as libc::c_int == '[' as i32
              || *p as libc::c_int == '\\' as i32
              || *p as libc::c_int == ']' as i32
              || *p as libc::c_int == '-' as i32
              || *p as libc::c_int == '!' as i32
              || *p as libc::c_int == '^' as i32
            {
              /* case '^' in [\^]  ) echo ok;; *) echo WRONG;; esac */
              let fresh36 = q;
              q = q.offset(1);
              *fresh36 = '\\' as i32 as libc::c_char
            }
          }
        } else if !slash_position.is_null() && p == str.offset(*slash_position as isize) {
          /* stop handling globbing */
          globbing = 0i32 as libc::c_uint;
          *slash_position = q.wrapping_offset_from(r) as libc::c_long as libc::c_int;
          slash_position = 0 as *mut libc::c_int
        }
        protect_against_glob = globbing
      }
      let fresh37 = p;
      p = p.offset(1);
      let fresh38 = q;
      q = q.offset(1);
      *fresh38 = *fresh37
    }
  }
  *q = '\u{0}' as i32 as libc::c_char;
  if flag & 0x8i32 != 0 {
    expdest = r;
    expdest =
      expdest.offset((q.wrapping_offset_from(r) as libc::c_long + 1i32 as libc::c_long) as isize)
  }
  return r;
}
/*
 * Prepare a pattern for a expmeta (internal glob(3)) call.
 *
 * Returns an stalloced string.
 */
unsafe extern "C" fn preglob(
  mut pattern: *const libc::c_char,
  mut flag: libc::c_int,
) -> *mut libc::c_char {
  return rmescapes(
    pattern as *mut libc::c_char,
    flag | 0x2i32,
    0 as *mut libc::c_int,
  );
}
/*
 * Put a string on the stack.
 */
unsafe extern "C" fn memtodest(
  mut p: *const libc::c_char,
  mut len: size_t,
  mut syntax: libc::c_int,
  mut quotes: libc::c_int,
) {
  let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
  if len == 0 {
    return;
  }
  q = makestrspace(
    if quotes & (0x1i32 | 0x10i32) != 0 {
      len.wrapping_mul(2i32 as libc::c_ulong)
    } else {
      len
    },
    expdest,
  );
  let mut current_block_9: u64;
  loop {
    let fresh39 = p;
    p = p.offset(1);
    let mut c: libc::c_uchar = *fresh39 as libc::c_uchar;
    if c != 0 {
      if quotes & (0x1i32 | 0x10i32) != 0 {
        let mut n: libc::c_int = SIT(c as libc::c_int, syntax);
        if n == 12i32 || syntax != 0i32 && n == 2i32 {
          let fresh40 = q;
          q = q.offset(1);
          *fresh40 = '\u{81}' as i32 as libc::c_uchar as libc::c_char
        }
      }
      current_block_9 = 2979737022853876585;
    } else if quotes & 0x2i32 == 0 {
      current_block_9 = 16658872821858055392;
    } else {
      current_block_9 = 2979737022853876585;
    }
    match current_block_9 {
      2979737022853876585 => {
        let fresh41 = q;
        q = q.offset(1);
        *fresh41 = c as libc::c_char
      }
      _ => {}
    }
    len = len.wrapping_sub(1);
    if !(len != 0) {
      break;
    }
  }
  expdest = q;
}
unsafe extern "C" fn strtodest(
  mut p: *const libc::c_char,
  mut syntax: libc::c_int,
  mut quotes: libc::c_int,
) -> size_t {
  let mut len: size_t = strlen(p);
  memtodest(p, len, syntax, quotes);
  return len;
}
/*
 * Record the fact that we have to scan this region of the
 * string for IFS characters.
 */
unsafe extern "C" fn recordregion(
  mut start: libc::c_int,
  mut end: libc::c_int,
  mut nulonly: libc::c_int,
) {
  let mut ifsp: *mut ifsregion = 0 as *mut ifsregion;
  if ifslastp.is_null() {
    ifsp = &mut ifsfirst
  } else {
    ::std::ptr::write_volatile(
      &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
      ::std::ptr::read_volatile::<libc::c_int>(
        &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
      ) + 1,
    );
    asm!("" : : : "memory" : "volatile");
    ifsp = xzalloc(::std::mem::size_of::<ifsregion>() as libc::c_ulong) as *mut ifsregion;
    /*ifsp->next = NULL; - ckzalloc did it */
    (*ifslastp).next = ifsp;
    int_on();
  }
  ifslastp = ifsp;
  (*ifslastp).begoff = start;
  (*ifslastp).endoff = end;
  (*ifslastp).nulonly = nulonly;
}
unsafe extern "C" fn removerecordregions(mut endoff: libc::c_int) {
  if ifslastp.is_null() {
    return;
  }
  if ifsfirst.endoff > endoff {
    while !ifsfirst.next.is_null() {
      let mut ifsp: *mut ifsregion = 0 as *mut ifsregion;
      ::std::ptr::write_volatile(
        &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
        ::std::ptr::read_volatile::<libc::c_int>(
          &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
        ) + 1,
      );
      asm!("" : : : "memory" : "volatile");
      ifsp = (*ifsfirst.next).next;
      free(ifsfirst.next as *mut libc::c_void);
      ifsfirst.next = ifsp;
      int_on();
    }
    if ifsfirst.begoff > endoff {
      ifslastp = 0 as *mut ifsregion
    } else {
      ifslastp = &mut ifsfirst;
      ifsfirst.endoff = endoff
    }
    return;
  }
  ifslastp = &mut ifsfirst;
  while !(*ifslastp).next.is_null() && (*(*ifslastp).next).begoff < endoff {
    ifslastp = (*ifslastp).next
  }
  while !(*ifslastp).next.is_null() {
    let mut ifsp_0: *mut ifsregion = 0 as *mut ifsregion;
    ::std::ptr::write_volatile(
      &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
      ::std::ptr::read_volatile::<libc::c_int>(
        &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
      ) + 1,
    );
    asm!("" : : : "memory" : "volatile");
    ifsp_0 = (*(*ifslastp).next).next;
    free((*ifslastp).next as *mut libc::c_void);
    (*ifslastp).next = ifsp_0;
    int_on();
  }
  if (*ifslastp).endoff > endoff {
    (*ifslastp).endoff = endoff
  };
}
unsafe extern "C" fn exptilde(
  mut startp: *mut libc::c_char,
  mut p: *mut libc::c_char,
  mut flags: libc::c_int,
) -> *mut libc::c_char {
  let mut current_block: u64;
  let mut c: libc::c_uchar = 0;
  let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut pw: *mut passwd = 0 as *mut passwd;
  let mut home: *const libc::c_char = 0 as *const libc::c_char;
  let mut quotes: libc::c_int = flags & (0x1i32 | 0x10i32);
  name = p.offset(1);
  loop {
    p = p.offset(1);
    c = *p as libc::c_uchar;
    if !(c as libc::c_int != '\u{0}' as i32) {
      break;
    }
    match c as libc::c_int {
      129 => return startp,
      136 => return startp,
      58 => {
        if flags & 0x4i32 != 0 {
          break;
        }
      }
      47 | 131 => {
        break;
      }
      _ => {}
    }
  }
  *p = '\u{0}' as i32 as libc::c_char;
  if *name as libc::c_int == '\u{0}' as i32 {
    home = lookupvar(b"HOME\x00" as *const u8 as *const libc::c_char);
    current_block = 15904375183555213903;
  } else {
    pw = bb_internal_getpwnam(name);
    if pw.is_null() {
      current_block = 11235359091809133519;
    } else {
      home = (*pw).pw_dir;
      current_block = 15904375183555213903;
    }
  }
  match current_block {
    15904375183555213903 => {
      if !(home.is_null() || *home == 0) {
        *p = c as libc::c_char;
        strtodest(home, 2i32, quotes);
        return p;
      }
    }
    _ => {}
  }
  *p = c as libc::c_char;
  return startp;
}
/* An evaltree() which is known to never return.
 * Used to use an alias:
 * static int evaltreenr(union node *, int) __attribute__((alias("evaltree"),__noreturn__));
 * but clang was reported to "transfer" noreturn-ness to evaltree() as well.
 */
#[inline(always)]
unsafe extern "C" fn evaltreenr(mut n: *mut node, mut flags: libc::c_int) -> ! {
  evaltree(n, flags);
  abort();
  /* NOTREACHED */
}
unsafe extern "C" fn evalbackcmd(mut n: *mut node, mut result: *mut backcmd) {
  let mut pip: [libc::c_int; 2] = [0; 2];
  let mut jp: *mut job = 0 as *mut job;
  (*result).fd = -1i32;
  (*result).buf = 0 as *mut libc::c_char;
  (*result).nleft = 0i32;
  (*result).jp = 0 as *mut job;
  if !n.is_null() {
    if pipe(pip.as_mut_ptr()) < 0i32 {
      ash_msg_and_raise_error(b"can\'t create pipe: %m\x00" as *const u8 as *const libc::c_char);
    }
    jp = makejob(1i32);
    if forkshell(jp, n, 2i32) == 0i32 {
      /* child */
      force_int_on();
      close(pip[0]);
      if pip[1] != 1i32 {
        /* NOTREACHED */
        /*close(1);*/
        dup2_or_raise(pip[1], 1i32);
        close(pip[1]);
      }
      (*ash_ptr_to_globals_misc).optlist[0] = 0i32 as libc::c_char;
      ifsfree();
      evaltreenr(n, 0o1i32);
    }
    /* TODO: eflag clearing makes the following not abort:
     *  ash -c 'set -e; z=$(false;echo foo); echo $z'
     * which is what bash does (unless it is in POSIX mode).
     * dash deleted "eflag = 0" line in the commit
     *  Date: Mon, 28 Jun 2010 17:11:58 +1000
     *  [EVAL] Don't clear eflag in evalbackcmd
     * For now, preserve bash-like behavior, it seems to be somewhat more useful:
     */
    /* parent */
    close(pip[1]);
    (*result).fd = pip[0];
    (*result).jp = jp
  };
}
/*
 * Expand stuff in backwards quotes.
 */
unsafe extern "C" fn expbackq(mut cmd: *mut node, mut flag: libc::c_int) {
  let mut current_block: u64;
  let mut in_0: backcmd = backcmd {
    fd: 0,
    nleft: 0,
    buf: 0 as *mut libc::c_char,
    jp: 0 as *mut job,
  };
  let mut i: libc::c_int = 0;
  let mut buf: [libc::c_char; 128] = [0; 128];
  let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut dest: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut startloc: libc::c_int = 0;
  let mut syntax: libc::c_int = if flag & 0x100i32 != 0 { 1i32 } else { 0i32 };
  let mut smark: stackmark = stackmark {
    stackp: 0 as *mut stack_block,
    stacknxt: 0 as *mut libc::c_char,
    stacknleft: 0,
  };
  ::std::ptr::write_volatile(
    &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
    ::std::ptr::read_volatile::<libc::c_int>(
      &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
    ) + 1,
  );
  asm!("" : : : "memory" : "volatile");
  startloc = expdest.wrapping_offset_from(
    (*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut libc::c_char,
  ) as libc::c_long as libc::c_int;
  pushstackmark(&mut smark, startloc as size_t);
  evalbackcmd(cmd, &mut in_0);
  popstackmark(&mut smark);
  p = in_0.buf;
  i = in_0.nleft;
  if i == 0i32 {
    current_block = 6396512884640876198;
  } else {
    current_block = 13056961889198038528;
  }
  loop {
    match current_block {
      13056961889198038528 => {
        memtodest(p, i as size_t, syntax, flag & (0x1i32 | 0x10i32));
        current_block = 6396512884640876198;
      }
      _ => {
        if in_0.fd < 0i32 {
          break;
        }
        i = nonblock_immune_read(
          in_0.fd,
          buf.as_mut_ptr() as *mut libc::c_void,
          ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
        ) as libc::c_int;
        if i <= 0i32 {
          break;
        }
        p = buf.as_mut_ptr();
        current_block = 13056961889198038528;
      }
    }
  }
  free(in_0.buf as *mut libc::c_void);
  if in_0.fd >= 0i32 {
    close(in_0.fd);
    (*ash_ptr_to_globals_misc).back_exitstatus = waitforjob(in_0.jp) as uint8_t
  }
  int_on();
  /* Eat all trailing newlines */
  dest = expdest;
  while dest > (*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut libc::c_char
    && *dest.offset(-1i32 as isize) as libc::c_int == '\n' as i32
  {
    dest = dest.offset(-1)
  }
  expdest = dest;
  if flag & 0x100i32 == 0 {
    recordregion(
      startloc,
      dest.wrapping_offset_from(
        (*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut libc::c_char,
      ) as libc::c_long as libc::c_int,
      0i32,
    );
  };
}
/*
 * Expand arithmetic expression.  Backup to start of expression,
 * evaluate, place result in (backed up) result, adjust string position.
 */
unsafe extern "C" fn expari(mut flag: libc::c_int) {
  let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut begoff: libc::c_int = 0;
  let mut len: libc::c_int = 0;
  /* ifsfree(); */
  /*
   * This routine is slightly over-complicated for
   * efficiency.  Next we scan backwards looking for the
   * start of arithmetic.
   */
  start = (*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut libc::c_char;
  p = expdest.offset(-1);
  *p = '\u{0}' as i32 as libc::c_char;
  p = p.offset(-1);
  loop {
    let mut esc: libc::c_int = 0;
    while *p as libc::c_uchar as libc::c_int != '\u{86}' as i32 as libc::c_uchar as libc::c_int {
      p = p.offset(-1)
    }
    esc = esclen(start, p) as libc::c_int;
    if esc % 2i32 == 0 {
      break;
    }
    p = p.offset(-((esc + 1i32) as isize))
  }
  begoff = p.wrapping_offset_from(start) as libc::c_long as libc::c_int;
  removerecordregions(begoff);
  expdest = p;
  if flag & (0x1i32 | 0x10i32) != 0 {
    rmescapes(p.offset(1), 0i32, 0 as *mut libc::c_int);
  }
  len = cvtnum(ash_arith(p.offset(1)));
  if flag & 0x100i32 == 0 {
    recordregion(begoff, begoff + len, 0i32);
  };
}
/*
 * Perform variable and command substitution.  If EXP_FULL is set, output CTLESC
 * characters to allow for further processing.  Otherwise treat
 * $@ like $* since no splitting will be performed.
 */
unsafe extern "C" fn argstr(mut p: *mut libc::c_char, mut flags: libc::c_int) {
  let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut current_block: u64;
  static mut spclchars: [libc::c_char; 9] = [
    '=' as i32 as libc::c_char,
    ':' as i32 as libc::c_char,
    '\u{88}' as i32 as libc::c_uchar as libc::c_char,
    '\u{83}' as i32 as libc::c_uchar as libc::c_char,
    '\u{81}' as i32 as libc::c_uchar as libc::c_char,
    '\u{82}' as i32 as libc::c_uchar as libc::c_char,
    '\u{84}' as i32 as libc::c_uchar as libc::c_char,
    '\u{87}' as i32 as libc::c_uchar as libc::c_char,
    '\u{0}' as i32 as libc::c_char,
  ];
  let mut reject: *const libc::c_char = spclchars.as_ptr();
  let mut breakall: libc::c_int = (flags & (0x40i32 | 0x100i32) == 0x40i32) as libc::c_int;
  let mut inquotes: libc::c_int = 0;
  let mut length: size_t = 0;
  let mut startloc: libc::c_int = 0;
  if flags & 0x4i32 == 0 {
    reject = reject.offset(2)
  } else if flags & 0x20i32 != 0 {
    reject = reject.offset(1)
  }
  inquotes = 0i32;
  length = 0i32 as size_t;
  if flags & 0x2i32 != 0 {
    q = 0 as *mut libc::c_char;
    flags &= !0x2i32;
    current_block = 7498628566730793279;
  } else {
    current_block = 13038395059501912860;
  }
  'c_15354: loop {
    match current_block {
      7498628566730793279 => {
        q = p;
        if *q as libc::c_int == '~' as i32 {
          p = exptilde(p, q, flags)
        }
        current_block = 13038395059501912860;
      }
      _ => {
        startloc = expdest.wrapping_offset_from(
          (*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut libc::c_char,
        ) as libc::c_long as libc::c_int;
        loop {
          let mut c: libc::c_uchar = 0;
          length = (length as libc::c_ulong)
            .wrapping_add(strcspn(p.offset(length as isize), reject)) as size_t
            as size_t;
          c = *p.offset(length as isize) as libc::c_uchar;
          if c != 0 {
            if c as libc::c_int & 0x80i32 == 0
              || c as libc::c_int == '\u{87}' as i32 as libc::c_uchar as libc::c_int
            {
              /* c == '=' || c == ':' || c == CTLENDARI */
              length = length.wrapping_add(1)
            }
          }
          if length > 0i32 as libc::c_ulong {
            let mut newloc: libc::c_int = 0;
            expdest = stack_nputstr(p, length, expdest);
            newloc = expdest.wrapping_offset_from(
              (*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut libc::c_char,
            ) as libc::c_long as libc::c_int;
            if breakall != 0 && inquotes == 0 && newloc > startloc {
              recordregion(startloc, newloc, 0i32);
            }
            startloc = newloc
          }
          p = p.offset(length.wrapping_add(1i32 as libc::c_ulong) as isize);
          length = 0i32 as size_t;
          match c as libc::c_int {
            0 => {
              break 'c_15354;
            }
            61 => {
              if flags & 0x20i32 != 0 {
                p = p.offset(-1);
                continue;
              } else {
                flags |= 0x20i32;
                reject = reject.offset(1)
              }
            }
            58 => {}
            _ => {
              match c as libc::c_int {
                131 => {
                  break 'c_15354;
                }
                136 => {
                  /* "$@" syntax adherence hack */
                  if inquotes == 0
                    && memcmp(
                      p as *const libc::c_void,
                      dolatstr.as_ptr().offset(1) as *const libc::c_void,
                      (6i32 - 1i32) as libc::c_ulong,
                    ) == 0
                  {
                    p = evalvar(p.offset(1), flags | 0x100i32).offset(1);
                    current_block = 13038395059501912860;
                    break;
                  } else {
                    inquotes ^= 0x100i32
                  }
                }
                129 => {
                  startloc += 1;
                  length = length.wrapping_add(1)
                }
                130 => {
                  p = evalvar(p, flags | inquotes);
                  current_block = 13038395059501912860;
                  break;
                }
                132 => {
                  expbackq((*argbackq).n, flags | inquotes);
                  argbackq = (*argbackq).next;
                  current_block = 13038395059501912860;
                  break;
                }
                135 => {
                  p = p.offset(-1);
                  expari(flags | inquotes);
                  current_block = 13038395059501912860;
                  break;
                }
                _ => {
                  continue;
                }
              }
              if flags & (0x1i32 | 0x10i32) != 0 {
                p = p.offset(-1);
                length = length.wrapping_add(1);
                startloc += 1
              }
              continue;
            }
          }
          /*
           * sort of a hack - expand tildes in variable
           * assignments (after the first '=' and after ':'s).
           */
          p = p.offset(-1);
          if *p as libc::c_int == '~' as i32 {
            current_block = 7498628566730793279;
            break;
          }
        }
      }
    }
  }
}
unsafe extern "C" fn scanleft(
  mut startp: *mut libc::c_char,
  mut rmesc: *mut libc::c_char,
  mut _rmescend: *mut libc::c_char,
  mut pattern: *mut libc::c_char,
  mut quotes: libc::c_int,
  mut zero: libc::c_int,
) -> *mut libc::c_char {
  let mut loc: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut loc2: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut c: libc::c_char = 0;
  loc = startp;
  loc2 = rmesc;
  loop {
    let mut match_0: libc::c_int = 0;
    let mut s: *const libc::c_char = loc2;
    c = *loc2;
    if zero != 0 {
      *loc2 = '\u{0}' as i32 as libc::c_char;
      s = rmesc
    }
    match_0 = (fnmatch(pattern, s, 0i32) == 0) as libc::c_int;
    *loc2 = c;
    if match_0 != 0 {
      return loc;
    }
    if quotes != 0
      && *loc as libc::c_uchar as libc::c_int == '\u{81}' as i32 as libc::c_uchar as libc::c_int
    {
      loc = loc.offset(1)
    }
    loc = loc.offset(1);
    loc2 = loc2.offset(1);
    if !(c != 0) {
      break;
    }
  }
  return 0 as *mut libc::c_char;
}
unsafe extern "C" fn scanright(
  mut startp: *mut libc::c_char,
  mut rmesc: *mut libc::c_char,
  mut rmescend: *mut libc::c_char,
  mut pattern: *mut libc::c_char,
  mut quotes: libc::c_int,
  mut match_at_start: libc::c_int,
) -> *mut libc::c_char {
  let mut esc: libc::c_int = 0i32;
  let mut loc: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut loc2: *mut libc::c_char = 0 as *mut libc::c_char;
  /* If we called by "${v/pattern/repl}" or "${v//pattern/repl}":
   * startp="escaped_value_of_v" rmesc="raw_value_of_v"
   * rmescend=""(ptr to NUL in rmesc) pattern="pattern" quotes=match_at_start=1
   * Logic:
   * loc starts at NUL at the end of startp, loc2 starts at the end of rmesc,
   * and on each iteration they go back two/one char until they reach the beginning.
   * We try to find a match in "raw_value_of_v", "raw_value_of_", "raw_value_of" etc.
   */
  /* TODO: document in what other circumstances we are called. */
  loc = pattern.offset(-1);
  loc2 = rmescend;
  while loc >= startp {
    let mut match_0: libc::c_int = 0;
    let mut c: libc::c_char = *loc2;
    let mut s: *const libc::c_char = loc2;
    if match_at_start != 0 {
      *loc2 = '\u{0}' as i32 as libc::c_char;
      s = rmesc
    }
    match_0 = (fnmatch(pattern, s, 0i32) == 0) as libc::c_int;
    //bb_error_msg("pmatch(pattern:'%s',s:'%s'):%d", pattern, s, match);
    *loc2 = c;
    if match_0 != 0 {
      return loc;
    }
    loc = loc.offset(-1);
    if quotes != 0 {
      esc -= 1;
      if esc < 0i32 {
        esc = esclen(startp, loc) as libc::c_int
      }
      if esc % 2i32 != 0 {
        esc -= 1;
        loc = loc.offset(-1)
      }
    }
    loc2 = loc2.offset(-1)
  }
  return 0 as *mut libc::c_char;
}
unsafe extern "C" fn varunset(
  mut end: *const libc::c_char,
  mut var: *const libc::c_char,
  mut umsg: *const libc::c_char,
  mut varflags: libc::c_int,
) -> ! {
  let mut msg: *const libc::c_char = 0 as *const libc::c_char;
  let mut tail: *const libc::c_char = 0 as *const libc::c_char;
  tail = (*ash_ptr_to_globals_misc).nullstr.as_mut_ptr();
  msg = b"parameter not set\x00" as *const u8 as *const libc::c_char;
  if !umsg.is_null() {
    if *end as libc::c_uchar as libc::c_int == '\u{83}' as i32 as libc::c_uchar as libc::c_int {
      if varflags & 0x10i32 != 0 {
        tail = b" or null\x00" as *const u8 as *const libc::c_char
      }
    } else {
      msg = umsg
    }
  }
  ash_msg_and_raise_error(
    b"%.*s: %s%s\x00" as *const u8 as *const libc::c_char,
    (end.wrapping_offset_from(var) as libc::c_long - 1i32 as libc::c_long) as libc::c_int,
    var,
    msg,
    tail,
  );
}
unsafe extern "C" fn subevalvar(
  mut p: *mut libc::c_char,
  mut varname: *mut libc::c_char,
  mut strloc: libc::c_int,
  mut subtype: libc::c_int,
  mut startloc: libc::c_int,
  mut varflags: libc::c_int,
  mut flag: libc::c_int,
) -> *const libc::c_char {
  let mut len_0: libc::c_int = 0;
  let mut idx: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut saveargbackq: *mut nodelist = argbackq;
  let mut quotes: libc::c_int = flag & (0x1i32 | 0x10i32);
  let mut startp: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut loc: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut rmesc: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut rmescend: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut amount: libc::c_int = 0;
  let mut resetloc: libc::c_int = 0;
  let mut argstr_flags: libc::c_int = 0;
  let mut workloc: libc::c_int = 0;
  let mut slash_pos: libc::c_int = 0;
  let mut repl: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut zero: libc::c_int = 0;
  let mut scan: Option<
    unsafe extern "C" fn(
      _: *mut libc::c_char,
      _: *mut libc::c_char,
      _: *mut libc::c_char,
      _: *mut libc::c_char,
      _: libc::c_int,
      _: libc::c_int,
    ) -> *mut libc::c_char,
  > = None;
  //bb_error_msg("subevalvar(p:'%s',varname:'%s',strloc:%d,subtype:%d,startloc:%d,varflags:%x,quotes:%d)",
  //		p, varname, strloc, subtype, startloc, varflags, quotes);
  /* For "${v/pattern/repl}", we must find the delimiter _before_
   * argstr() call expands possible variable references in pattern:
   * think about "v=a; a=a/; echo ${v/$a/r}" case.
   */
  repl = 0 as *mut libc::c_char;
  if subtype == 0xdi32 || subtype == 0xei32 {
    /* Find '/' and replace with NUL */
    repl = p;
    /* The pattern can't be empty.
     * IOW: if the first char after "${v//" is a slash,
     * it does not terminate the pattern - it's the first char of the pattern:
     *  v=/dev/ram; echo ${v////-}  prints -dev-ram (pattern is "/")
     *  v=/dev/ram; echo ${v///r/-} prints /dev-am  (pattern is "/r")
     */
    if *repl as libc::c_int == '/' as i32 {
      repl = repl.offset(1)
    }
    loop {
      if *repl as libc::c_int == '\u{0}' as i32 {
        repl = 0 as *mut libc::c_char;
        break;
      } else if *repl as libc::c_int == '/' as i32 {
        *repl = '\u{0}' as i32 as libc::c_char;
        break;
      } else {
        /* Handle escaped slashes, e.g. "${v/\//_}" (they are CTLESC'ed by this point) */
        if *repl as libc::c_uchar as libc::c_int == '\u{81}' as i32 as libc::c_uchar as libc::c_int
          && *repl.offset(1) as libc::c_int != 0
        {
          repl = repl.offset(1)
        }
        repl = repl.offset(1)
      }
    }
  }
  argstr_flags = 0x2i32;
  if subtype != 0x5i32 && subtype != 0x4i32 && subtype != 0xci32 {
    /* EXP_CASE keeps CTLESC's */
    argstr_flags = 0x2i32 | 0x10i32
  }
  argstr(p, argstr_flags);
  //bb_error_msg("str0:'%s'", (char *)stackblock() + strloc);
  slash_pos = -1i32;
  if !repl.is_null() {
    slash_pos = expdest.wrapping_offset_from(
      ((*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut libc::c_char)
        .offset(strloc as isize),
    ) as libc::c_long as libc::c_int;
    expdest = _STPUTC('/' as i32, expdest);
    //bb_error_msg("repl+1:'%s'", repl + 1);
    argstr(repl.offset(1), 0x2i32); /* EXP_TILDE: echo "${v/x/~}" expands ~ ! */
    *repl = '/' as i32 as libc::c_char
  }
  expdest = _STPUTC('\u{0}' as i32, expdest);
  argbackq = saveargbackq;
  startp = ((*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut libc::c_char)
    .offset(startloc as isize);
  //bb_error_msg("str1:'%s'", (char *)stackblock() + strloc);
  match subtype {
    5 => {
      setvar0(varname, startp);
      amount = startp.wrapping_offset_from(expdest) as libc::c_long as libc::c_int;
      expdest = expdest.offset(amount as isize);
      return startp;
      /* BASH_SUBSTR */
    }
    4 => {
      varunset(p, varname, startp, varflags);
    }
    12 => {
      /* NOTREACHED */
      let mut pos: libc::c_int = 0;
      let mut len: libc::c_int = 0;
      let mut orig_len: libc::c_int = 0;
      let mut colon: *mut libc::c_char = 0 as *mut libc::c_char;
      str = ((*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void).offset(strloc as isize)
        as *mut libc::c_char;
      loc = str;
      /* Read POS in ${var:POS:LEN} */
      colon = strchr(loc, ':' as i32);
      if !colon.is_null() {
        *colon = '\u{0}' as i32 as libc::c_char
      }
      pos = substr_atoi(loc);
      if !colon.is_null() {
        *colon = ':' as i32 as libc::c_char
      }
      /* Read LEN in ${var:POS:LEN} */
      len =
        (str.wrapping_offset_from(startp) as libc::c_long - 1i32 as libc::c_long) as libc::c_int;
      /* *loc != '\0', guaranteed by parser */
      if quotes != 0 {
        let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
        /* Adjust the length by the number of escapes */
        ptr = startp;
        while ptr < str.offset(-1) {
          if *ptr as libc::c_uchar as libc::c_int == '\u{81}' as i32 as libc::c_uchar as libc::c_int
          {
            len -= 1;
            ptr = ptr.offset(1)
          }
          ptr = ptr.offset(1)
        }
      }
      orig_len = len;
      let fresh42 = loc;
      loc = loc.offset(1);
      if *fresh42 as libc::c_int == ':' as i32 {
        /* ${var::LEN} */
        len = substr_atoi(loc)
      } else {
        /* Skip POS in ${var:POS:LEN} */
        len = orig_len;
        while *loc as libc::c_int != 0 && *loc as libc::c_int != ':' as i32 {
          loc = loc.offset(1)
        }
        let fresh43 = loc;
        loc = loc.offset(1);
        if *fresh43 as libc::c_int == ':' as i32 {
          len = substr_atoi(loc)
        }
      }
      if pos < 0i32 {
        /* ${VAR:$((-n)):l} starts n chars from the end */
        pos = orig_len + pos
      }
      if pos as libc::c_uint >= orig_len as libc::c_uint {
        /* apart from obvious ${VAR:999999:l},
         * covers ${VAR:$((-9999999)):l} - result is ""
         * (bash compat)
         */
        pos = 0i32;
        len = 0i32
      }
      if len < 0i32 {
        /* ${VAR:N:-M} sets LEN to strlen()-M */
        len = orig_len - pos + len
      }
      if len as libc::c_uint > (orig_len - pos) as libc::c_uint {
        len = orig_len - pos
      }
      str = startp;
      while pos != 0 {
        if quotes != 0
          && *str as libc::c_uchar as libc::c_int == '\u{81}' as i32 as libc::c_uchar as libc::c_int
        {
          str = str.offset(1)
        }
        str = str.offset(1);
        pos -= 1
      }
      loc = startp;
      while len != 0 {
        if quotes != 0
          && *str as libc::c_uchar as libc::c_int == '\u{81}' as i32 as libc::c_uchar as libc::c_int
        {
          let fresh44 = str;
          str = str.offset(1);
          let fresh45 = loc;
          loc = loc.offset(1);
          *fresh45 = *fresh44
        }
        let fresh46 = str;
        str = str.offset(1);
        let fresh47 = loc;
        loc = loc.offset(1);
        *fresh47 = *fresh46;
        len -= 1
      }
      *loc = '\u{0}' as i32 as libc::c_char;
      amount = loc.wrapping_offset_from(expdest) as libc::c_long as libc::c_int;
      expdest = expdest.offset(amount as isize);
      return loc;
    }
    _ => {}
  }
  resetloc = expdest.wrapping_offset_from(
    (*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut libc::c_char,
  ) as libc::c_long as libc::c_int;
  repl = 0 as *mut libc::c_char;
  'c_22182: loop
  /* We'll comeback here if we grow the stack while handling
   * a VSREPLACE or VSREPLACEALL, since our pointers into the
   * stack will need rebasing, and we'll need to remove our work
   * areas each time
   */
  {
    amount = expdest.wrapping_offset_from(
      ((*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut libc::c_char)
        .offset(resetloc as isize),
    ) as libc::c_long as libc::c_int;
    expdest = expdest.offset(-amount as isize);
    startp = ((*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut libc::c_char)
      .offset(startloc as isize);
    rmesc = startp;
    rmescend = ((*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void
      as *mut libc::c_char)
      .offset(strloc as isize);
    //bb_error_msg("str7:'%s'", rmescend);
    if quotes != 0 {
      //TODO: how to handle slash_pos here if string changes (shortens?)
      rmesc = rmescapes(startp, 0x1i32 | 0x8i32, 0 as *mut libc::c_int);
      if rmesc != startp {
        rmescend = expdest;
        startp = ((*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void
          as *mut libc::c_char)
          .offset(startloc as isize)
      }
    }
    rmescend = rmescend.offset(-1);
    str = ((*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut libc::c_char)
      .offset(strloc as isize);
    /*
     * Example: v='a\bc'; echo ${v/\\b/_\\_\z_}
     * The result is a_\_z_c (not a\_\_z_c)!
     *
     * The search pattern and replace string treat backslashes differently!
     * "&slash_pos" causes rmescapes() to work differently on the pattern
     * and string.  It's only used on the first call.
     */
    //bb_error_msg("str8:'%s' slash_pos:%d", str, slash_pos);
    rmescapes(
      str,
      0x2i32,
      if !repl.is_null() {
        0 as *mut libc::c_int
      } else if slash_pos < 0i32 {
        0 as *mut libc::c_int
      } else {
        &mut slash_pos
      },
    );
    workloc = expdest.wrapping_offset_from(
      (*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut libc::c_char,
    ) as libc::c_long as libc::c_int;
    if subtype == 0xdi32 || subtype == 0xei32 {
      len_0 = 0;
      idx = 0 as *mut libc::c_char;
      end = 0 as *mut libc::c_char;
      if repl.is_null() {
        //bb_error_msg("str9:'%s' slash_pos:%d", str, slash_pos);
        repl = (*ash_ptr_to_globals_misc).nullstr.as_mut_ptr();
        if slash_pos >= 0i32 {
          repl = str.offset(slash_pos as isize);
          let fresh48 = repl;
          repl = repl.offset(1);
          *fresh48 = '\u{0}' as i32 as libc::c_char
        }
      }
      //bb_error_msg("str:'%s' repl:'%s'", str, repl);
      /* If there's no pattern to match, return the expansion unmolested */
      if *str.offset(0) as libc::c_int == '\u{0}' as i32 {
        return 0 as *const libc::c_char;
      }
      len_0 = 0i32;
      idx = startp;
      end = str.offset(-1);
      's_614: loop {
        if !(idx < end) {
          break 'c_22182;
        }
        loop {
          loc = scanright(idx, rmesc, rmescend, str, quotes, 1i32);
          //bb_error_msg("scanright('%s'):'%s'", str, loc);
          if !loc.is_null() {
            break;
          }
          /* No match, advance */
          let mut restart_detect: *mut libc::c_char =
            (*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut libc::c_char;
          loop
          /* Pattern is "*foo". If "*foo" does not match "long_string",
           * it would never match "ong_string" etc, no point in trying.
           */
          {
            expdest = _STPUTC(*idx as libc::c_int, expdest);
            if quotes != 0
              && *idx as libc::c_uchar as libc::c_int
                == '\u{81}' as i32 as libc::c_uchar as libc::c_int
            {
              idx = idx.offset(1);
              len_0 += 1;
              expdest = _STPUTC(*idx as libc::c_int, expdest)
            }
            if (*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void
              != restart_detect as *mut libc::c_void
            {
              break 's_614;
            }
            idx = idx.offset(1);
            len_0 += 1;
            rmesc = rmesc.offset(1);
            /* continue; - prone to quadratic behavior, smarter code: */
            if idx >= end {
              break 'c_22182;
            }
            if !(*str.offset(0) as libc::c_int == '*' as i32) {
              break;
            }
          }
        }
        if subtype == 0xei32 {
          while idx < loc {
            if quotes != 0
              && *idx as libc::c_uchar as libc::c_int
                == '\u{81}' as i32 as libc::c_uchar as libc::c_int
            {
              idx = idx.offset(1)
            }
            idx = idx.offset(1);
            rmesc = rmesc.offset(1)
          }
        } else {
          idx = loc
        }
        //bb_error_msg("repl:'%s'", repl);
        loc = repl;
        while *loc != 0 {
          let mut restart_detect_0: *mut libc::c_char =
            (*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut libc::c_char;
          if quotes != 0 && *loc as libc::c_int == '\\' as i32 {
            expdest = _STPUTC('\u{81}' as i32 as libc::c_uchar as libc::c_int, expdest);
            len_0 += 1
          }
          expdest = _STPUTC(*loc as libc::c_int, expdest);
          if (*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void
            != restart_detect_0 as *mut libc::c_void
          {
            break 's_614;
          }
          len_0 += 1;
          loc = loc.offset(1)
        }
        if !(subtype == 0xdi32) {
          continue;
        }
        loop
        //bb_error_msg("tail:'%s', quotes:%x", idx, quotes);
        {
          if !(*idx != 0) {
            break 'c_22182;
          }
          let mut restart_detect_1: *mut libc::c_char =
            (*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut libc::c_char;
          expdest = _STPUTC(*idx as libc::c_int, expdest);
          if (*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void
            != restart_detect_1 as *mut libc::c_void
          {
            break 's_614;
          }
          len_0 += 1;
          idx = idx.offset(1)
        }
      }
    } else {
      /* BASH_PATTERN_SUBST */
      subtype -= 0x6i32;
      /* zero = (subtype == VSTRIMLEFT || subtype == VSTRIMLEFTMAX) */
      zero = subtype >> 1i32;
      /* VSTRIMLEFT/VSTRIMRIGHTMAX -> scanleft */
      scan = if subtype & 1i32 ^ zero != 0 {
        Some(
          scanleft
            as unsafe extern "C" fn(
              _: *mut libc::c_char,
              _: *mut libc::c_char,
              _: *mut libc::c_char,
              _: *mut libc::c_char,
              _: libc::c_int,
              _: libc::c_int,
            ) -> *mut libc::c_char,
        )
      } else {
        Some(
          scanright
            as unsafe extern "C" fn(
              _: *mut libc::c_char,
              _: *mut libc::c_char,
              _: *mut libc::c_char,
              _: *mut libc::c_char,
              _: libc::c_int,
              _: libc::c_int,
            ) -> *mut libc::c_char,
        )
      };
      loc = scan.expect("non-null function pointer")(startp, rmesc, rmescend, str, quotes, zero);
      if !loc.is_null() {
        if zero != 0 {
          memmove(
            startp as *mut libc::c_void,
            loc as *const libc::c_void,
            str.wrapping_offset_from(loc) as libc::c_long as libc::c_ulong,
          );
          loc = startp
            .offset(str.wrapping_offset_from(loc) as libc::c_long as isize)
            .offset(-1)
        }
        *loc = '\u{0}' as i32 as libc::c_char;
        amount = loc.wrapping_offset_from(expdest) as libc::c_long as libc::c_int;
        expdest = expdest.offset(amount as isize)
      }
      return loc;
    }
  }
  /* We've put the replaced text into a buffer at workloc, now
   * move it to the right place and adjust the stack.
   */
  expdest = _STPUTC('\u{0}' as i32, expdest);
  startp = ((*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut libc::c_char)
    .offset(startloc as isize);
  memmove(
    startp as *mut libc::c_void,
    ((*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut libc::c_char)
      .offset(workloc as isize) as *const libc::c_void,
    (len_0 + 1i32) as libc::c_ulong,
  );
  //bb_error_msg("startp:'%s'", startp);
  amount =
    expdest.wrapping_offset_from(startp.offset(len_0 as isize)) as libc::c_long as libc::c_int;
  expdest = expdest.offset(-amount as isize);
  return startp;
}
/*
 * Add the value of a specialized variable to the stack string.
 * name parameter (examples):
 * ash -c 'echo $1'      name:'1='
 * ash -c 'echo $qwe'    name:'qwe='
 * ash -c 'echo $$'      name:'$='
 * ash -c 'echo ${$}'    name:'$='
 * ash -c 'echo ${$##q}' name:'$=q'
 * ash -c 'echo ${#$}'   name:'$='
 * note: examples with bad shell syntax:
 * ash -c 'echo ${#$1}'  name:'$=1'
 * ash -c 'echo ${#1#}'  name:'1=#'
 */
#[inline(never)]
unsafe extern "C" fn varvalue(
  mut name: *mut libc::c_char,
  mut varflags: libc::c_int,
  mut flags: libc::c_int,
  mut quoted: libc::c_int,
) -> ssize_t {
  let mut ap: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char; /* number(name) fails on ${N#str} etc */
  let mut sepc: libc::c_char = 0;
  let mut c: libc::c_char = 0;
  let mut current_block: u64;
  let mut p: *const libc::c_char = 0 as *const libc::c_char;
  let mut num: libc::c_int = 0;
  let mut i: libc::c_int = 0;
  let mut len: ssize_t = 0i32 as ssize_t;
  let mut sep: libc::c_int = 0;
  let mut subtype: libc::c_int = varflags & 0xfi32;
  let mut discard: libc::c_int = (subtype == 0x3i32 || subtype == 0xai32) as libc::c_int;
  let mut quotes: libc::c_int = (if discard != 0 {
    0i32
  } else {
    (flags) & (0x1i32 | 0x10i32)
  }) | 0x2i32;
  let mut syntax: libc::c_int = 0;
  sep = (flags & 0x1i32) << 8i32;
  syntax = if quoted != 0 { 1i32 } else { 0i32 };
  match *name as libc::c_int {
    36 => {
      num = (*ash_ptr_to_globals_misc).rootpid;
      current_block = 14614856358309981616;
    }
    63 => {
      num = (*ash_ptr_to_globals_misc).exitstatus as libc::c_int;
      current_block = 14614856358309981616;
    }
    35 => {
      num = (*ash_ptr_to_globals_var).shellparam.nparam;
      current_block = 14614856358309981616;
    }
    33 => {
      num = (*ash_ptr_to_globals_misc).backgndpid;
      if num == 0i32 {
        return -1i32 as ssize_t;
      }
      current_block = 14614856358309981616;
    }
    45 => {
      expdest = makestrspace(NOPTS as libc::c_int as size_t, expdest);
      i = NOPTS as libc::c_int - 1i32;
      while i >= 0i32 {
        if (*ash_ptr_to_globals_misc).optlist[i as usize] as libc::c_int != 0
          && *optletters_optnames[i as usize].offset(0) as libc::c_int != 0
        {
          let fresh49 = expdest;
          expdest = expdest.offset(1);
          *fresh49 = *optletters_optnames[i as usize].offset(0);
          len += 1
        }
        i -= 1
      }
      current_block = 17075014677070940716;
    }
    64 => {
      if quoted != 0 && sep != 0 {
        current_block = 12642393550206671977;
      } else {
        current_block = 14763689060501151050;
      }
    }
    42 => {
      current_block = 14763689060501151050;
    }
    48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
      num = atoi(name);
      if num < 0i32 || num > (*ash_ptr_to_globals_var).shellparam.nparam {
        return -1i32 as ssize_t;
      }
      p = if num != 0 {
        *(*ash_ptr_to_globals_var)
          .shellparam
          .p
          .offset((num - 1i32) as isize)
      } else {
        (*ash_ptr_to_globals_misc).arg0
      };
      current_block = 3835246268939772286;
    }
    _ => {
      /* NB: name has form "VAR=..." */
      p = lookupvar(name);
      current_block = 3835246268939772286;
    }
  }
  match current_block {
    14763689060501151050 =>
    /* fall through */
    {
      ap = 0 as *mut *mut libc::c_char; /* case '*' */
      sepc = 0;
      c = 0;
      /* We will set c to 0 or ~0 depending on whether
       * we're doing field splitting.  We won't do field
       * splitting if either we're quoted or sep is zero.
       *
       * Instead of testing (quoted || !sep) the following
       * trick optimises away any branches by using the
       * fact that EXP_QUOTED (which is the only bit that
       * can be set in quoted) is the same as EXP_FULL <<
       * CHAR_BIT (which is the only bit that can be set
       * in sep).
       */
      c = (((quoted | !sep) & 0x100i32 == 0) as libc::c_int - 1i32) as libc::c_char;
      sep &= !quoted;
      sep |= if (*ash_ptr_to_globals_var).varinit[0].flags & 0x20i32 == 0i32 {
        (c as libc::c_int
          & *(*ash_ptr_to_globals_var).varinit[0]
            .var_text
            .offset(4)
            .offset(0) as libc::c_int) as libc::c_uchar as libc::c_int
      } else {
        ' ' as i32
      };
      current_block = 12642393550206671977;
    }
    3835246268939772286 => {
      if p.is_null() {
        return -1i32 as ssize_t;
      }
      len = strtodest(p, syntax, quotes) as ssize_t;
      if subtype == 0xai32 && len > 0i32 as libc::c_long {
        reinit_unicode_for_ash();
        if UNICODE_ON as libc::c_int == UNICODE_ON as libc::c_int {
          expdest = expdest.offset(-len as isize);
          discard = 0i32;
          len = unicode_strlen(p) as ssize_t
        }
      }
      current_block = 17075014677070940716;
    }
    14614856358309981616 => {
      len = cvtnum(num as arith_t) as ssize_t;
      current_block = 17075014677070940716;
    }
    _ => {}
  }
  match current_block {
    12642393550206671977 => {
      sepc = sep as libc::c_char;
      ap = (*ash_ptr_to_globals_var).shellparam.p;
      if ap.is_null() {
        return -1i32 as ssize_t;
      }
      loop {
        let fresh50 = ap;
        ap = ap.offset(1);
        p = *fresh50;
        if p.is_null() {
          break;
        }
        len =
          (len as libc::c_ulong).wrapping_add(strtodest(p, syntax, quotes)) as ssize_t as ssize_t;
        if !(*ap).is_null() && sep != 0 {
          len += 1;
          memtodest(&mut sepc, 1i32 as size_t, syntax, quotes);
        }
      }
    }
    _ => {}
  }
  if discard != 0 {
    expdest = expdest.offset(-len as isize)
  }
  return len;
}
/* argstr needs it */
/*
 * Expand a variable, and return a pointer to the next character in the
 * input string.
 */
unsafe extern "C" fn evalvar(mut p: *mut libc::c_char, mut flag: libc::c_int) -> *mut libc::c_char {
  let mut current_block: u64; //TODO: use var_end(p)?
  let mut varflags: libc::c_char = 0;
  let mut subtype: libc::c_char = 0;
  let mut quoted: libc::c_int = 0;
  let mut var: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut patloc: libc::c_int = 0;
  let mut startloc: libc::c_int = 0;
  let mut varlen: ssize_t = 0;
  let fresh51 = p;
  p = p.offset(1);
  varflags = *fresh51 as libc::c_uchar as libc::c_char;
  subtype = (varflags as libc::c_int & 0xfi32) as libc::c_char;
  if subtype == 0 {
    raise_error_syntax(b"bad substitution\x00" as *const u8 as *const libc::c_char);
  }
  quoted = flag & 0x100i32;
  var = p;
  startloc = expdest.wrapping_offset_from(
    (*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut libc::c_char,
  ) as libc::c_long as libc::c_int;
  p = strchr(p, '=' as i32).offset(1);
  loop {
    varlen = varvalue(var, varflags as libc::c_int, flag, quoted);
    if varflags as libc::c_int & 0x10i32 != 0 {
      varlen -= 1
    }
    if subtype as libc::c_int == 0x3i32 {
      varlen = -1i32 as libc::c_long - varlen;
      current_block = 4602131768141143591;
      break;
    } else {
      if subtype as libc::c_int == 0x2i32 {
        current_block = 4602131768141143591;
        break;
      }
      if subtype as libc::c_int == 0x5i32 || subtype as libc::c_int == 0x4i32 {
        if varlen >= 0i32 as libc::c_long {
          current_block = 12784458099392390209;
          break;
        }
        subevalvar(
          p,
          var,
          0i32,
          subtype as libc::c_int,
          startloc,
          varflags as libc::c_int,
          flag & !(0x1i32 | 0x10i32),
        );
        varflags = (varflags as libc::c_int & !0x10i32) as libc::c_char;
        /*
         * Remove any recorded regions beyond
         * start of variable
         */
        removerecordregions(startloc);
      } else {
        if varlen < 0i32 as libc::c_long
          && (*ash_ptr_to_globals_misc).optlist[13] as libc::c_int != 0
        {
          varunset(p, var, 0 as *const libc::c_char, 0i32);
        }
        if subtype as libc::c_int == 0xai32 {
          current_block = 1608152415753874203;
          break;
        } else {
          current_block = 17788412896529399552;
          break;
        }
      }
    }
  }
  match current_block {
    4602131768141143591 => {
      if varlen < 0i32 as libc::c_long {
        argstr(p, flag | 0x2i32 | 0x40i32);
        current_block = 8905788842858268286;
      } else {
        current_block = 12784458099392390209;
      }
    }
    17788412896529399552 => {
      if subtype as libc::c_int == 0x1i32 {
        current_block = 12784458099392390209;
      } else if varlen >= 0i32 as libc::c_long {
        /*
         * Terminate the string and start recording the pattern
         * right after it
         */
        expdest = _STPUTC('\u{0}' as i32, expdest);
        patloc = expdest.wrapping_offset_from(
          (*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut libc::c_char,
        ) as libc::c_long as libc::c_int;
        if subevalvar(
          p,
          0 as *mut libc::c_char,
          patloc,
          subtype as libc::c_int,
          startloc,
          varflags as libc::c_int,
          flag,
        )
        .is_null()
        {
          let mut amount: libc::c_int = expdest.wrapping_offset_from(
            ((*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut libc::c_char)
              .offset(patloc as isize)
              .offset(-1),
          ) as libc::c_long as libc::c_int;
          expdest = expdest.offset(-amount as isize)
        }
        /* Remove any recorded regions beyond start of variable */
        removerecordregions(startloc);
        current_block = 12784458099392390209;
      } else {
        current_block = 8905788842858268286;
      }
    }
    1608152415753874203 => {
      cvtnum(if varlen > 0i32 as libc::c_long {
        varlen
      } else {
        0i32 as libc::c_long
      } as arith_t);
      current_block = 12784458099392390209;
    }
    _ => {}
  }
  match current_block {
    12784458099392390209 => {
      if quoted != 0 {
        quoted = (*var as libc::c_int == '@' as i32
          && (*ash_ptr_to_globals_var).shellparam.nparam != 0) as libc::c_int;
        if quoted == 0 {
          current_block = 8905788842858268286;
        } else {
          current_block = 2873832966593178012;
        }
      } else {
        current_block = 2873832966593178012;
      }
      match current_block {
        8905788842858268286 => {}
        _ => {
          recordregion(
            startloc,
            expdest.wrapping_offset_from(
              (*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut libc::c_char,
            ) as libc::c_long as libc::c_int,
            quoted,
          );
        }
      }
    }
    _ => {}
  }
  if subtype as libc::c_int != 0x1i32 {
    /* skip to end of alternative */
    let mut nesting: libc::c_int = 1i32;
    loop {
      let fresh52 = p;
      p = p.offset(1);
      let mut c: libc::c_uchar = *fresh52 as libc::c_uchar;
      if c as libc::c_int == '\u{81}' as i32 as libc::c_uchar as libc::c_int {
        p = p.offset(1)
      } else if c as libc::c_int == '\u{84}' as i32 as libc::c_uchar as libc::c_int {
        if varlen >= 0i32 as libc::c_long {
          argbackq = (*argbackq).next
        }
      } else if c as libc::c_int == '\u{82}' as i32 as libc::c_uchar as libc::c_int {
        let fresh53 = p;
        p = p.offset(1);
        if *fresh53 as libc::c_int & 0xfi32 != 0x1i32 {
          nesting += 1
        }
      } else {
        if !(c as libc::c_int == '\u{83}' as i32 as libc::c_uchar as libc::c_int) {
          continue;
        }
        nesting -= 1;
        if nesting == 0i32 {
          break;
        }
      }
    }
  }
  return p;
}
/*
 * Add a file name to the list.
 */
unsafe extern "C" fn addfname(mut name: *const libc::c_char) {
  let mut sp: *mut strlist = 0 as *mut strlist;
  sp = stzalloc(::std::mem::size_of::<strlist>() as libc::c_ulong) as *mut strlist;
  (*sp).text = sstrdup(name);
  *exparg.lastp = sp;
  exparg.lastp = &mut (*sp).next;
}
/* Avoid glob() (and thus, stat() et al) for words like "echo" */
unsafe extern "C" fn hasmeta(mut p: *const libc::c_char) -> libc::c_int {
  static mut chars: [libc::c_char; 7] = [
    '*' as i32 as libc::c_char,
    '?' as i32 as libc::c_char,
    '[' as i32 as libc::c_char,
    '\\' as i32 as libc::c_char,
    '\u{88}' as i32 as libc::c_uchar as libc::c_char,
    '\u{81}' as i32 as libc::c_uchar as libc::c_char,
    0i32 as libc::c_char,
  ];
  loop {
    p = strpbrk(p, chars.as_ptr());
    if p.is_null() {
      break;
    }
    let mut current_block_10: u64;
    match *p as libc::c_uchar as libc::c_int {
      136 => {
        loop {
          p = p.offset(1);
          if *p as libc::c_uchar as libc::c_int == '\u{88}' as i32 as libc::c_uchar as libc::c_int {
            break;
          }
          if *p as libc::c_uchar as libc::c_int == '\u{81}' as i32 as libc::c_uchar as libc::c_int {
            p = p.offset(1)
          }
          if *p as libc::c_int == '\u{0}' as i32 {
            /* huh? */
            return 0i32;
          }
        }
        current_block_10 = 4808432441040389987;
      }
      92 | 129 => {
        p = p.offset(1);
        if *p as libc::c_int == '\u{0}' as i32 {
          return 0i32;
        }
        current_block_10 = 4808432441040389987;
      }
      91 => {
        if strchr(p.offset(1), ']' as i32).is_null() {
          current_block_10 = 4808432441040389987;
        } else {
          current_block_10 = 451145588581359213;
        }
      }
      _ => {
        current_block_10 = 451145588581359213;
      }
    }
    match current_block_10 {
      4808432441040389987 =>
        /* It's not a properly closed [] pattern,
       * but other metas may follow. Continue checking.
       * my[file* _is_ globbed by bash
       * and matches filenames like "my[file1".
       */
        {}
      _ =>
      /* fallthrough */
      /* case '*': */
      /* case '?': */
      {
        return 1i32
      }
    }
    p = p.offset(1)
  }
  return 0i32;
}
unsafe extern "C" fn expmeta(
  mut exp: *mut exp_t,
  mut name: *mut libc::c_char,
  mut name_len: libc::c_uint,
  mut expdir_len: libc::c_uint,
) {
  let mut enddir: *mut libc::c_char = (*exp).dir.offset(expdir_len as isize);
  let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut cp: *const libc::c_char = 0 as *const libc::c_char;
  let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut endname: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut metaflag: libc::c_int = 0;
  let mut statb: stat = stat {
    st_dev: 0,
    st_ino: 0,
    st_nlink: 0,
    st_mode: 0,
    st_uid: 0,
    st_gid: 0,
    __pad0: 0,
    st_rdev: 0,
    st_size: 0,
    st_blksize: 0,
    st_blocks: 0,
    st_atim: timespec {
      tv_sec: 0,
      tv_nsec: 0,
    },
    st_mtim: timespec {
      tv_sec: 0,
      tv_nsec: 0,
    },
    st_ctim: timespec {
      tv_sec: 0,
      tv_nsec: 0,
    },
    __glibc_reserved: [0; 3],
  };
  let mut dirp: *mut DIR = 0 as *mut DIR;
  let mut dp: *mut dirent = 0 as *mut dirent;
  let mut atend: libc::c_int = 0;
  let mut matchdot: libc::c_int = 0;
  let mut esc: libc::c_int = 0;
  metaflag = 0i32;
  start = name;
  p = name;
  loop {
    esc = 0i32;
    if !(*p != 0) {
      break;
    }
    if *p as libc::c_int == '*' as i32 || *p as libc::c_int == '?' as i32 {
      metaflag = 1i32
    } else if *p as libc::c_int == '[' as i32 {
      let mut q: *mut libc::c_char = p.offset(1);
      if *q as libc::c_int == '!' as i32 {
        q = q.offset(1)
      }
      loop {
        if *q as libc::c_int == '\\' as i32 {
          q = q.offset(1)
        }
        if *q as libc::c_int == '/' as i32 || *q as libc::c_int == '\u{0}' as i32 {
          break;
        }
        q = q.offset(1);
        if !(*q as libc::c_int == ']' as i32) {
          continue;
        }
        metaflag = 1i32;
        break;
      }
    } else {
      if *p as libc::c_int == '\\' as i32 && *p.offset(1) as libc::c_int != 0 {
        esc += 1
      }
      if *p.offset(esc as isize) as libc::c_int == '/' as i32 {
        if metaflag != 0 {
          break;
        }
        start = p.offset(esc as isize).offset(1)
      }
    }
    p = p.offset((esc + 1i32) as isize)
  }
  if metaflag == 0i32 {
    /* we've reached the end of the file name */
    if expdir_len == 0 {
      return;
    } /* terminate first half of list */
    p = name; /* sort first half of list */
    loop {
      if *p as libc::c_int == '\\' as i32 && *p.offset(1) as libc::c_int != 0 {
        p = p.offset(1)
      } /* sort second half */
      let fresh54 = enddir;
      enddir = enddir.offset(1);
      *fresh54 = *p;
      let fresh55 = p;
      p = p.offset(1);
      if !(*fresh55 != 0) {
        break;
      }
    }
    if lstat((*exp).dir, &mut statb) == 0i32 {
      addfname((*exp).dir);
    }
    return;
  }
  endname = p;
  if name < start {
    p = name;
    loop {
      if *p as libc::c_int == '\\' as i32 && *p.offset(1) as libc::c_int != 0 {
        p = p.offset(1)
      }
      let fresh56 = p;
      p = p.offset(1);
      let fresh57 = enddir;
      enddir = enddir.offset(1);
      *fresh57 = *fresh56;
      if !(p < start) {
        break;
      }
    }
  }
  *enddir = '\u{0}' as i32 as libc::c_char;
  cp = (*exp).dir;
  expdir_len = enddir.wrapping_offset_from(cp) as libc::c_long as libc::c_uint;
  if expdir_len == 0 {
    cp = b".\x00" as *const u8 as *const libc::c_char
  }
  dirp = opendir(cp);
  if dirp.is_null() {
    return;
  }
  if *endname as libc::c_int == 0i32 {
    atend = 1i32
  } else {
    atend = 0i32;
    *endname = '\u{0}' as i32 as libc::c_char;
    endname = endname.offset((esc + 1i32) as isize)
  }
  name_len =
    (name_len as libc::c_long - endname.wrapping_offset_from(name) as libc::c_long) as libc::c_uint;
  matchdot = 0i32;
  p = start;
  if *p as libc::c_int == '\\' as i32 {
    p = p.offset(1)
  }
  if *p as libc::c_int == '.' as i32 {
    matchdot += 1
  }
  while (*ash_ptr_to_globals_misc).pending_int == 0 && {
    dp = readdir(dirp);
    !dp.is_null()
  } {
    if (*dp).d_name[0] as libc::c_int == '.' as i32 && matchdot == 0 {
      continue;
    }
    if fnmatch(start, (*dp).d_name.as_mut_ptr(), 0i32) == 0 {
      if atend != 0 {
        strcpy(enddir, (*dp).d_name.as_mut_ptr());
        addfname((*exp).dir);
      } else {
        let mut offset: libc::c_uint = 0;
        let mut len: libc::c_uint = 0;
        p = stpcpy(enddir, (*dp).d_name.as_mut_ptr());
        *p = '/' as i32 as libc::c_char;
        offset = (p.wrapping_offset_from((*exp).dir) as libc::c_long + 1i32 as libc::c_long)
          as libc::c_uint;
        len = offset
          .wrapping_add(name_len)
          .wrapping_add(255i32 as libc::c_uint);
        if len > (*exp).dir_max {
          len = len.wrapping_add(4096i32 as libc::c_uint);
          (*exp).dir =
            xrealloc((*exp).dir as *mut libc::c_void, len as size_t) as *mut libc::c_char;
          (*exp).dir_max = len
        }
        expmeta(exp, endname, name_len, offset);
        enddir = (*exp).dir.offset(expdir_len as isize)
      }
    }
  }
  closedir(dirp);
  if atend == 0 {
    *endname.offset((-esc - 1i32) as isize) =
      if esc != 0 { '\\' as i32 } else { '/' as i32 } as libc::c_char
  };
}
unsafe extern "C" fn msort(mut list_0: *mut strlist, mut len: libc::c_int) -> *mut strlist {
  let mut p: *mut strlist = 0 as *mut strlist;
  let mut q: *mut strlist = 0 as *mut strlist;
  let mut lpp: *mut *mut strlist = 0 as *mut *mut strlist;
  let mut half: libc::c_int = 0;
  let mut n: libc::c_int = 0;
  if len <= 1i32 {
    return list_0;
  }
  half = len >> 1i32;
  p = list_0;
  n = half;
  loop {
    n -= 1;
    if !(n >= 0i32) {
      break;
    }
    q = p;
    p = (*p).next
  }
  (*q).next = 0 as *mut strlist;
  q = msort(list_0, half);
  p = msort(p, len - half);
  lpp = &mut list_0;
  loop {
    if strcmp((*p).text, (*q).text) < 0i32 {
      *lpp = p;
      lpp = &mut (*p).next;
      p = *lpp;
      if !p.is_null() {
        continue;
      }
      *lpp = q;
      break;
    } else {
      *lpp = q;
      lpp = &mut (*q).next;
      q = *lpp;
      if !q.is_null() {
        continue;
      }
      *lpp = p;
      break;
    }
  }
  return list_0;
}
/*
 * Sort the results of file name expansion.  It calculates the number of
 * strings to sort and then calls msort (short for merge sort) to do the
 * work.
 */
unsafe extern "C" fn expsort(mut str: *mut strlist) -> *mut strlist {
  let mut len: libc::c_int = 0;
  let mut sp: *mut strlist = 0 as *mut strlist;
  len = 0i32;
  sp = str;
  while !sp.is_null() {
    len += 1;
    sp = (*sp).next
  }
  return msort(str, len);
}
unsafe extern "C" fn expandmeta(mut str: *mut strlist)
/*, int flag*/
{
  let mut current_block: u64;
  /* TODO - EXP_REDIR */
  while !str.is_null() {
    let mut exp: exp_t = exp_t {
      dir: 0 as *mut libc::c_char,
      dir_max: 0,
    };
    let mut savelastp: *mut *mut strlist = 0 as *mut *mut strlist;
    let mut sp: *mut strlist = 0 as *mut strlist;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_uint = 0;
    if (*ash_ptr_to_globals_misc).optlist[1] != 0 {
      current_block = 4536899987212780243;
    } else if hasmeta((*str).text) == 0 {
      current_block = 4536899987212780243;
    } else {
      savelastp = exparg.lastp;
      ::std::ptr::write_volatile(
        &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
        ::std::ptr::read_volatile::<libc::c_int>(
          &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
        ) + 1,
      );
      asm!("" : : : "memory" : "volatile");
      p = preglob((*str).text, 0x1i32 | 0x10i32);
      len = strlen(p) as libc::c_uint;
      exp.dir_max = len.wrapping_add(4096i32 as libc::c_uint);
      exp.dir = xmalloc(exp.dir_max as size_t) as *mut libc::c_char;
      expmeta(&mut exp, p, len, 0i32 as libc::c_uint);
      free(exp.dir as *mut libc::c_void);
      if p != (*str).text {
        free(p as *mut libc::c_void);
      }
      int_on();
      if exparg.lastp == savelastp {
        current_block = 4536899987212780243;
      } else {
        *exparg.lastp = 0 as *mut strlist;
        sp = expsort(*savelastp);
        *savelastp = sp;
        while !(*sp).next.is_null() {
          sp = (*sp).next
        }
        exparg.lastp = &mut (*sp).next;
        current_block = 15345278821338558188;
      }
    }
    match current_block {
      4536899987212780243 =>
      /*
       * no matches
       */
      {
        *exparg.lastp = str;
        rmescapes((*str).text, 0i32, 0 as *mut libc::c_int);
        exparg.lastp = &mut (*str).next
      }
      _ => {}
    }
    str = (*str).next
  }
}
/* ENABLE_ASH_INTERNAL_GLOB */
/*
 * Perform variable substitution and command substitution on an argument,
 * placing the resulting list of arguments in arglist.  If EXP_FULL is true,
 * perform splitting and file name expansion.  When arglist is NULL, perform
 * here document expansion.
 */
unsafe extern "C" fn expandarg(
  mut arg: *mut node,
  mut arglist: *mut arglist,
  mut flag: libc::c_int,
) {
  let mut sp: *mut strlist = 0 as *mut strlist;
  let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
  argbackq = (*arg).narg.backquote;
  expdest = (*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut libc::c_char;
  argstr((*arg).narg.text, flag);
  p = _STPUTC('\u{0}' as i32, expdest);
  expdest = p.offset(-1);
  if !arglist.is_null() {
    p = stalloc(p.wrapping_offset_from(
      (*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut libc::c_char,
    ) as libc::c_long as size_t) as *mut libc::c_char;
    exparg.lastp = &mut exparg.list;
    /*
     * TODO - EXP_REDIR
     */
    if flag & 0x1i32 != 0 {
      ifsbreakup(p, &mut exparg);
      *exparg.lastp = 0 as *mut strlist;
      exparg.lastp = &mut exparg.list;
      expandmeta(exparg.list);
    } else {
      sp = stzalloc(::std::mem::size_of::<strlist>() as libc::c_ulong) as *mut strlist;
      (*sp).text = p;
      *exparg.lastp = sp;
      exparg.lastp = &mut (*sp).next
    }
    *exparg.lastp = 0 as *mut strlist;
    if !exparg.list.is_null() {
      *(*arglist).lastp = exparg.list;
      (*arglist).lastp = exparg.lastp
    }
  }
  /* here document expanded */
  ifsfree();
}
/*
 * Expand shell variables and backquotes inside a here document.
 */
unsafe extern "C" fn expandhere(mut arg: *mut node, mut fd: libc::c_int) {
  expandarg(arg, 0 as *mut libc::c_void as *mut arglist, 0x100i32);
  full_write(
    fd,
    (*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void,
    expdest.wrapping_offset_from(
      (*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut libc::c_char,
    ) as libc::c_long as size_t,
  );
}
/*
 * Returns true if the pattern matches the string.
 */
unsafe extern "C" fn patmatch(
  mut pattern: *mut libc::c_char,
  mut string: *const libc::c_char,
) -> libc::c_int {
  let mut p: *mut libc::c_char = preglob(pattern, 0i32);
  let mut r: libc::c_int = (fnmatch(p, string, 0i32) == 0) as libc::c_int;
  //bb_error_msg("!fnmatch(pattern:'%s',str:'%s',0):%d", p, string, r);
  return r;
}
/*
 * See if a pattern matches in a case statement.
 */
unsafe extern "C" fn casematch(mut pattern: *mut node, mut val: *mut libc::c_char) -> libc::c_int {
  let mut smark: stackmark = stackmark {
    stackp: 0 as *mut stack_block,
    stacknxt: 0 as *mut libc::c_char,
    stacknleft: 0,
  };
  let mut result: libc::c_int = 0;
  setstackmark(&mut smark);
  argbackq = (*pattern).narg.backquote;
  expdest = (*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut libc::c_char;
  argstr((*pattern).narg.text, 0x2i32 | 0x10i32);
  if expdest == (*ash_ptr_to_globals_memstack).sstrend {
    expdest = growstackstr() as *mut libc::c_char
  }
  *expdest = '\u{0}' as i32 as libc::c_char;
  ifsfree();
  result = patmatch(
    (*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut libc::c_char,
    val,
  );
  popstackmark(&mut smark);
  return result;
}
static mut cmdtable: *mut *mut tblentry = 0 as *const *mut tblentry as *mut *mut tblentry;
static mut builtinloc: libc::c_int = -1i32;
/* index in path of %builtin, or -1 */
unsafe extern "C" fn tryexec(
  mut cmd: *const libc::c_char,
  mut argv: *mut *mut libc::c_char,
  mut envp: *mut *mut libc::c_char,
) {
  loop {
    execve(
      cmd,
      argv as *const *mut libc::c_char,
      envp as *const *mut libc::c_char,
    );
    if !(cmd != bb_busybox_exec_path.as_ptr() && *bb_errno == 8i32) {
      break;
    }
    /* Run "cmd" as a shell script:
     * http://pubs.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html
     * "If the execve() function fails with ENOEXEC, the shell
     * shall execute a command equivalent to having a shell invoked
     * with the command name as its first operand,
     * with any remaining arguments passed to the new shell"
     *
     * That is, do not use $SHELL, user's shell, or /bin/sh;
     * just call ourselves.
     *
     * Note that bash reads ~80 chars of the file, and if it sees
     * a zero byte before it sees newline, it doesn't try to
     * interpret it, but fails with "cannot execute binary file"
     * message and exit code 126. For one, this prevents attempts
     * to interpret foreign ELF binaries as shell scripts.
     */
    let ref mut fresh58 = *argv.offset(0);
    *fresh58 = cmd as *mut libc::c_char;
    cmd = bb_busybox_exec_path.as_ptr();
    /* NB: this is only possible because all callers of shellexec()
     * ensure that the argv[-1] slot exists!
     */
    argv = argv.offset(-1);
    let ref mut fresh59 = *argv.offset(0);
    *fresh59 = b"ash\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
  }
}
/*
 * Exec a program.  Never returns.  If you change this routine, you may
 * have to change the find_command routine as well.
 * argv[-1] must exist and be writable! See tryexec() for why.
 */
unsafe extern "C" fn shellexec(
  mut prog: *mut libc::c_char,
  mut argv: *mut *mut libc::c_char,
  mut path: *const libc::c_char,
  mut idx: libc::c_int,
) -> ! {
  let mut cmdname: *mut libc::c_char = 0 as *mut libc::c_char; /* used only by FEATURE_SH_STANDALONE */
  let mut e: libc::c_int = 0;
  let mut envp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  let mut exerrno: libc::c_int = 0;
  let mut applet_no: libc::c_int = -1i32;
  envp = listvars(0x1i32, 0x20i32, 0 as *mut *mut *mut libc::c_char);
  let mut current_block_11: u64;
  if !strchr(prog, '/' as i32).is_null() {
    tryexec(prog, argv, envp);
    if applet_no >= 0i32 {
      current_block_11 = 4217367930683358698;
    } else {
      e = *bb_errno;
      current_block_11 = 6009453772311597924;
    }
  } else {
    current_block_11 = 4217367930683358698;
  }
  match current_block_11 {
    4217367930683358698 =>
    /* We tried execing ourself, but it didn't work.
     * Maybe /proc/self/exe doesn't exist?
     * Try $PATH search.
     */
    {
      e = 2i32;
      loop {
        cmdname = path_advance(&mut path, prog);
        if cmdname.is_null() {
          break;
        }
        idx -= 1;
        if idx < 0i32 && pathopt.is_null() {
          tryexec(cmdname, argv, envp);
          if *bb_errno != 2i32 && *bb_errno != 20i32 {
            e = *bb_errno
          }
        }
        stunalloc(cmdname as *mut libc::c_void);
      }
    }
    _ => {}
  }
  /* Map to POSIX errors */
  match e {
    40 | 36 | 2 | 20 => exerrno = 127i32,
    _ => exerrno = 126i32,
  }
  (*ash_ptr_to_globals_misc).exitstatus = exerrno as uint8_t;
  ash_msg_and_raise(
    4i32,
    b"%s: %s\x00" as *const u8 as *const libc::c_char,
    prog,
    errmsg(e, b"not found\x00" as *const u8 as *const libc::c_char),
  );
  /* NOTREACHED */
}
unsafe extern "C" fn printentry(mut cmdp: *mut tblentry) {
  let mut idx: libc::c_int = 0;
  let mut path: *const libc::c_char = 0 as *const libc::c_char;
  let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
  idx = (*cmdp).param.index;
  path = (*ash_ptr_to_globals_var).varinit[(1i32 * 2i32 + 1i32) as usize]
    .var_text
    .offset(5);
  loop {
    name = path_advance(&mut path, (*cmdp).cmdname.as_mut_ptr());
    stunalloc(name as *mut libc::c_void);
    idx -= 1;
    if !(idx >= 0i32) {
      break;
    }
  }
  out1fmt(
    b"%s%s\n\x00" as *const u8 as *const libc::c_char,
    name,
    if (*cmdp).rehash as libc::c_int != 0 {
      b"*\x00" as *const u8 as *const libc::c_char
    } else {
      (*ash_ptr_to_globals_misc).nullstr.as_mut_ptr()
    },
  );
}
/*
 * Clear out command entries.  The argument specifies the first entry in
 * PATH which has changed.
 */
unsafe extern "C" fn clearcmdentry(mut firstchange: libc::c_int) {
  let mut tblp: *mut *mut tblentry = 0 as *mut *mut tblentry;
  let mut pp: *mut *mut tblentry = 0 as *mut *mut tblentry;
  let mut cmdp: *mut tblentry = 0 as *mut tblentry;
  ::std::ptr::write_volatile(
    &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
    ::std::ptr::read_volatile::<libc::c_int>(
      &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
    ) + 1,
  );
  asm!("" : : : "memory" : "volatile");
  tblp = cmdtable;
  while tblp < &mut *cmdtable.offset(31) as *mut *mut tblentry {
    pp = tblp;
    loop {
      cmdp = *pp;
      if cmdp.is_null() {
        break;
      }
      if (*cmdp).cmdtype as libc::c_int == 0i32 && (*cmdp).param.index >= firstchange
        || (*cmdp).cmdtype as libc::c_int == 2i32 && builtinloc >= firstchange
      {
        *pp = (*cmdp).next;
        free(cmdp as *mut libc::c_void);
      } else {
        pp = &mut (*cmdp).next
      }
    }
    tblp = tblp.offset(1)
  }
  int_on();
}
/*
 * Locate a command in the command hash table.  If "add" is nonzero,
 * add the command to the table if it is not already present.  The
 * variable "lastcmdentry" is set to point to the address of the link
 * pointing to the entry, so that delete_cmd_entry can delete the
 * entry.
 *
 * Interrupts must be off if called with add != 0.
 */
static mut lastcmdentry: *mut *mut tblentry = 0 as *const *mut tblentry as *mut *mut tblentry;
unsafe extern "C" fn cmdlookup(
  mut name: *const libc::c_char,
  mut add: libc::c_int,
) -> *mut tblentry {
  let mut hashval: libc::c_uint = 0;
  let mut p: *const libc::c_char = 0 as *const libc::c_char;
  let mut cmdp: *mut tblentry = 0 as *mut tblentry;
  let mut pp: *mut *mut tblentry = 0 as *mut *mut tblentry;
  p = name;
  hashval = ((*p as libc::c_uchar as libc::c_int) << 4i32) as libc::c_uint;
  while *p != 0 {
    let fresh60 = p;
    p = p.offset(1);
    hashval = hashval.wrapping_add(*fresh60 as libc::c_uchar as libc::c_uint)
  }
  hashval &= 0x7fffi32 as libc::c_uint;
  pp = &mut *cmdtable.offset(hashval.wrapping_rem(31i32 as libc::c_uint) as isize)
    as *mut *mut tblentry;
  cmdp = *pp;
  while !cmdp.is_null() {
    if strcmp((*cmdp).cmdname.as_mut_ptr(), name) == 0i32 {
      break;
    }
    pp = &mut (*cmdp).next;
    cmdp = (*cmdp).next
  }
  if add != 0 && cmdp.is_null() {
    *pp = xzalloc((::std::mem::size_of::<tblentry>() as libc::c_ulong).wrapping_add(strlen(name)))
      as *mut tblentry;
    cmdp = *pp;
    /*cmdp->next = NULL; - ckzalloc did it */
    (*cmdp).cmdtype = -1i32 as smallint;
    strcpy((*cmdp).cmdname.as_mut_ptr(), name);
  }
  lastcmdentry = pp;
  return cmdp;
}
/*
 * Delete the command entry returned on the last lookup.
 */
unsafe extern "C" fn delete_cmd_entry() {
  let mut cmdp: *mut tblentry = 0 as *mut tblentry;
  ::std::ptr::write_volatile(
    &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
    ::std::ptr::read_volatile::<libc::c_int>(
      &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
    ) + 1,
  );
  asm!("" : : : "memory" : "volatile");
  cmdp = *lastcmdentry;
  *lastcmdentry = (*cmdp).next;
  if (*cmdp).cmdtype as libc::c_int == 1i32 {
    freefunc((*cmdp).param.func);
  }
  free(cmdp as *mut libc::c_void);
  int_on();
}
/*
 * Add a new command entry, replacing any existing command entry for
 * the same name - except special builtins.
 */
unsafe extern "C" fn addcmdentry(mut name: *mut libc::c_char, mut entry: *mut cmdentry) {
  let mut cmdp: *mut tblentry = 0 as *mut tblentry;
  cmdp = cmdlookup(name, 1i32);
  if (*cmdp).cmdtype as libc::c_int == 1i32 {
    freefunc((*cmdp).param.func);
  }
  (*cmdp).cmdtype = (*entry).cmdtype;
  (*cmdp).param = (*entry).u;
  (*cmdp).rehash = 0i32 as libc::c_char;
}
unsafe extern "C" fn hashcmd(
  mut _argc: libc::c_int,
  mut _argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut pp: *mut *mut tblentry = 0 as *mut *mut tblentry;
  let mut cmdp: *mut tblentry = 0 as *mut tblentry;
  let mut c: libc::c_int = 0;
  let mut entry: cmdentry = cmdentry {
    cmdtype: 0,
    u: param { index: 0 },
  };
  let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
  if nextopt(b"r\x00" as *const u8 as *const libc::c_char) != '\u{0}' as i32 {
    clearcmdentry(0i32);
    return 0i32;
  }
  if (*argptr).is_null() {
    pp = cmdtable;
    while pp < &mut *cmdtable.offset(31) as *mut *mut tblentry {
      cmdp = *pp;
      while !cmdp.is_null() {
        if (*cmdp).cmdtype as libc::c_int == 0i32 {
          printentry(cmdp);
        }
        cmdp = (*cmdp).next
      }
      pp = pp.offset(1)
    }
    return 0i32;
  }
  c = 0i32;
  loop {
    name = *argptr;
    if name.is_null() {
      break;
    }
    cmdp = cmdlookup(name, 0i32);
    if !cmdp.is_null()
      && ((*cmdp).cmdtype as libc::c_int == 0i32
        || (*cmdp).cmdtype as libc::c_int == 2i32 && builtinloc >= 0i32)
    {
      delete_cmd_entry();
    }
    find_command(
      name,
      &mut entry,
      0x1i32,
      (*ash_ptr_to_globals_var).varinit[(1i32 * 2i32 + 1i32) as usize]
        .var_text
        .offset(5),
    );
    if entry.cmdtype as libc::c_int == -1i32 {
      c = 1i32
    }
    argptr = argptr.offset(1)
  }
  return c;
}
/*
 * Called when a cd is done.  Marks all commands so the next time they
 * are executed they will be rehashed.
 */
unsafe extern "C" fn hashcd() {
  let mut pp: *mut *mut tblentry = 0 as *mut *mut tblentry;
  let mut cmdp: *mut tblentry = 0 as *mut tblentry;
  pp = cmdtable;
  while pp < &mut *cmdtable.offset(31) as *mut *mut tblentry {
    cmdp = *pp;
    while !cmdp.is_null() {
      if (*cmdp).cmdtype as libc::c_int == 0i32
        || (*cmdp).cmdtype as libc::c_int == 2i32
          && *(*(*cmdp).param.cmd).name.offset(0) as libc::c_int & 2i32 == 0
          && builtinloc > 0i32
      {
        (*cmdp).rehash = 1i32 as libc::c_char
      }
      cmdp = (*cmdp).next
    }
    pp = pp.offset(1)
  }
}
/*
 * Fix command hash table when PATH changed.
 * Called before PATH is changed.  The argument is the new value of PATH;
 * pathval() still returns the old value at this point.
 * Called with interrupts off.
 */
unsafe extern "C" fn changepath(mut new: *const libc::c_char) {
  let mut old: *const libc::c_char = 0 as *const libc::c_char; /* assume no change */
  let mut firstchange: libc::c_int = 0;
  let mut idx: libc::c_int = 0;
  let mut idx_bltin: libc::c_int = 0;
  old = (*ash_ptr_to_globals_var).varinit[(1i32 * 2i32 + 1i32) as usize]
    .var_text
    .offset(5);
  firstchange = 9999i32;
  idx = 0i32;
  idx_bltin = -1i32;
  loop {
    if *old as libc::c_int != *new as libc::c_int {
      firstchange = idx;
      if *old as libc::c_int == '\u{0}' as i32 && *new as libc::c_int == ':' as i32
        || *old as libc::c_int == ':' as i32 && *new as libc::c_int == '\u{0}' as i32
      {
        firstchange += 1
      }
      old = new
      /* ignore subsequent differences */
    } /* zap builtins */
    if *new as libc::c_int == '\u{0}' as i32 {
      break;
    }
    if *new as libc::c_int == '%' as i32
      && idx_bltin < 0i32
      && !prefix(
        new.offset(1),
        b"builtin\x00" as *const u8 as *const libc::c_char,
      )
      .is_null()
    {
      idx_bltin = idx
    }
    if *new as libc::c_int == ':' as i32 {
      idx += 1
    }
    new = new.offset(1);
    old = old.offset(1)
  }
  if builtinloc < 0i32 && idx_bltin >= 0i32 {
    builtinloc = idx_bltin
  }
  if builtinloc >= 0i32 && idx_bltin < 0i32 {
    firstchange = 0i32
  }
  clearcmdentry(firstchange);
  builtinloc = idx_bltin;
}
static mut tokname_array: [*const libc::c_char; 30] = [
  b"end of file\x00" as *const u8 as *const libc::c_char,
  b"newline\x00" as *const u8 as *const libc::c_char,
  b"redirection\x00" as *const u8 as *const libc::c_char,
  b"word\x00" as *const u8 as *const libc::c_char,
  b";\x00" as *const u8 as *const libc::c_char,
  b"&\x00" as *const u8 as *const libc::c_char,
  b"&&\x00" as *const u8 as *const libc::c_char,
  b"||\x00" as *const u8 as *const libc::c_char,
  b"|\x00" as *const u8 as *const libc::c_char,
  b"(\x00" as *const u8 as *const libc::c_char,
  b")\x00" as *const u8 as *const libc::c_char,
  b";;\x00" as *const u8 as *const libc::c_char,
  b"`\x00" as *const u8 as *const libc::c_char,
  b"!\x00" as *const u8 as *const libc::c_char,
  b"case\x00" as *const u8 as *const libc::c_char,
  b"do\x00" as *const u8 as *const libc::c_char,
  b"done\x00" as *const u8 as *const libc::c_char,
  b"elif\x00" as *const u8 as *const libc::c_char,
  b"else\x00" as *const u8 as *const libc::c_char,
  b"esac\x00" as *const u8 as *const libc::c_char,
  b"fi\x00" as *const u8 as *const libc::c_char,
  b"for\x00" as *const u8 as *const libc::c_char,
  b"function\x00" as *const u8 as *const libc::c_char,
  b"if\x00" as *const u8 as *const libc::c_char,
  b"in\x00" as *const u8 as *const libc::c_char,
  b"then\x00" as *const u8 as *const libc::c_char,
  b"until\x00" as *const u8 as *const libc::c_char,
  b"while\x00" as *const u8 as *const libc::c_char,
  b"{\x00" as *const u8 as *const libc::c_char,
  b"}\x00" as *const u8 as *const libc::c_char,
];
/* Wrapper around strcmp for qsort/bsearch/... */
unsafe extern "C" fn pstrcmp(
  mut a: *const libc::c_void,
  mut b: *const libc::c_void,
) -> libc::c_int {
  return strcmp(a as *mut libc::c_char, *(b as *mut *mut libc::c_char));
}
unsafe extern "C" fn findkwd(mut s: *const libc::c_char) -> *const *const libc::c_char {
  return bsearch(
    s as *const libc::c_void,
    tokname_array.as_ptr().offset(13) as *const libc::c_void,
    ((::std::mem::size_of::<[*const libc::c_char; 30]>() as libc::c_ulong)
      .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
      as libc::c_uint)
      .wrapping_sub(13i32 as libc::c_uint) as size_t,
    ::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
    Some(
      pstrcmp
        as unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> libc::c_int,
    ),
  ) as *const *const libc::c_char;
}
/*
 * Locate and print what a word is...
 */
unsafe extern "C" fn describe_command(
  mut command: *mut libc::c_char,
  mut path: *const libc::c_char,
  mut describe_command_verbose: libc::c_int,
) -> libc::c_int {
  let mut entry: cmdentry = cmdentry {
    cmdtype: 0,
    u: param { index: 0 },
  };
  let mut ap: *const alias = 0 as *const alias;
  path = if !path.is_null() {
    path
  } else {
    (*ash_ptr_to_globals_var).varinit[(1i32 * 2i32 + 1i32) as usize]
      .var_text
      .offset(5)
  };
  if describe_command_verbose != 0 {
    out1str(command);
  }
  /* First look at the keywords */
  if !findkwd(command).is_null() {
    out1str(if describe_command_verbose != 0 {
      b" is a shell keyword\x00" as *const u8 as *const libc::c_char
    } else {
      command
    });
  } else {
    /* Then look at the aliases */
    ap = lookupalias(command, 0i32);
    if !ap.is_null() {
      if describe_command_verbose == 0 {
        out1str(b"alias \x00" as *const u8 as *const libc::c_char);
        printalias(ap);
        return 0i32;
      }
      out1fmt(
        b" is an alias for %s\x00" as *const u8 as *const libc::c_char,
        (*ap).val,
      );
    } else {
      /* Brute force */
      find_command(command, &mut entry, 0x2i32, path);
      match entry.cmdtype as libc::c_int {
        0 => {
          let mut j: libc::c_int = entry.u.index;
          let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
          if j < 0i32 {
            p = command
          } else {
            loop {
              p = path_advance(&mut path, command);
              stunalloc(p as *mut libc::c_void);
              j -= 1;
              if !(j >= 0i32) {
                break;
              }
            }
          }
          if describe_command_verbose != 0 {
            out1fmt(b" is %s\x00" as *const u8 as *const libc::c_char, p);
          } else {
            out1str(p);
          }
        }
        1 => {
          if describe_command_verbose != 0 {
            /*out1str(" is a shell function");*/
            out1str(b" is a function\x00" as *const u8 as *const libc::c_char);
          /* bash says this */
          } else {
            out1str(command);
          }
        }
        2 => {
          if describe_command_verbose != 0 {
            out1fmt(
              b" is a %sshell builtin\x00" as *const u8 as *const libc::c_char,
              if *(*entry.u.cmd).name.offset(0) as libc::c_int & 1i32 != 0 {
                b"special \x00" as *const u8 as *const libc::c_char
              } else {
                (*ash_ptr_to_globals_misc).nullstr.as_mut_ptr()
              },
            );
          } else {
            out1str(command);
          }
        }
        _ => {
          if describe_command_verbose != 0 {
            out1str(b": not found\n\x00" as *const u8 as *const libc::c_char);
          }
          return 127i32;
        }
      }
    }
  }
  out1str(b"\n\x00" as *const u8 as *const libc::c_char);
  return 0i32;
}
unsafe extern "C" fn typecmd(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut i: libc::c_int = 1i32;
  let mut err: libc::c_int = 0i32;
  let mut verbose: libc::c_int = 1i32;
  /* type -p ... ? (we don't bother checking for 'p') */
  if !(*argv.offset(1)).is_null() && *(*argv.offset(1)).offset(0) as libc::c_int == '-' as i32 {
    i += 1;
    verbose = 0i32
  }
  while !(*argv.offset(i as isize)).is_null() {
    let fresh61 = i;
    i = i + 1;
    err |= describe_command(
      *argv.offset(fresh61 as isize),
      0 as *const libc::c_char,
      verbose,
    )
  }
  return err;
}
/* Is it "command [-p] PROG ARGS" bltin, no other opts? Return ptr to "PROG" if yes */
unsafe extern "C" fn parse_command_args(
  mut argv: *mut *mut libc::c_char,
  mut path: *mut *const libc::c_char,
) -> *mut *mut libc::c_char {
  let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut c: libc::c_char = 0;
  loop {
    argv = argv.offset(1);
    cp = *argv;
    if cp.is_null() {
      return 0 as *mut *mut libc::c_char;
    }
    let fresh62 = cp;
    cp = cp.offset(1);
    if *fresh62 as libc::c_int != '-' as i32 {
      break;
    }
    let fresh63 = cp;
    cp = cp.offset(1);
    c = *fresh63;
    if c == 0 {
      break;
    }
    if c as libc::c_int == '-' as i32 && *cp == 0 {
      argv = argv.offset(1);
      if (*argv).is_null() {
        return 0 as *mut *mut libc::c_char;
      }
      break;
    } else {
      loop {
        match c as libc::c_int {
          112 => {
            *path = bb_PATH_root_path
              .as_ptr()
              .offset(::std::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong as isize)
          }
          _ => {
            /* run 'typecmd' for other options */
            return 0 as *mut *mut libc::c_char;
          }
        }
        let fresh64 = cp;
        cp = cp.offset(1);
        c = *fresh64;
        if !(c != 0) {
          break;
        }
      }
    }
  }
  return argv;
}
unsafe extern "C" fn commandcmd(
  mut _argc: libc::c_int,
  mut _argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut c: libc::c_int = 0;
  let mut verify: C2RustUnnamed_10 = 0 as C2RustUnnamed_10;
  let mut path: *const libc::c_char = 0 as *const libc::c_char;
  loop
  /* "command [-p] PROG ARGS" (that is, without -V or -v)
   * never reaches this function.
   */
  {
    c = nextopt(b"pvV\x00" as *const u8 as *const libc::c_char);
    if !(c != '\u{0}' as i32) {
      break;
    }
    if c == 'V' as i32 {
      verify = ::std::mem::transmute::<libc::c_uint, C2RustUnnamed_10>(
        verify as libc::c_uint | VERIFY_VERBOSE as libc::c_int as libc::c_uint,
      )
    } else if !(c == 'v' as i32) {
      path = bb_PATH_root_path
        .as_ptr()
        .offset(::std::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong as isize)
    }
  }
  /* Mimic bash: just "command -v" doesn't complain, it's a nop */
  cmd = *argptr;
  if !cmd.is_null() {
    return describe_command(cmd, path, verify as libc::c_int);
  }
  return 0i32;
}
/*static int funcblocksize;     // size of structures in function */
/*static int funcstringsize;    // size of strings in node */
static mut funcblock: *mut libc::c_void = 0 as *const libc::c_void as *mut libc::c_void;
/* block to allocate function from */
static mut funcstring_end: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
/* end of block to allocate strings from */
// Initialized in run_static_initializers
static mut nodesize: [uint8_t; 27] = [0; 27];
unsafe extern "C" fn sizenodelist(
  mut funcblocksize: libc::c_int,
  mut lp: *mut nodelist,
) -> libc::c_int {
  while !lp.is_null() {
    funcblocksize = (funcblocksize as libc::c_ulong).wrapping_add(
      (::std::mem::size_of::<nodelist>() as libc::c_ulong)
        .wrapping_add(SHELL_SIZE as libc::c_int as libc::c_ulong)
        & !(SHELL_SIZE as libc::c_int) as libc::c_ulong,
    ) as libc::c_int as libc::c_int; /* was funcstringsize += ... */
    funcblocksize = calcsize(funcblocksize, (*lp).n); /* was funcstringsize += ... */
    lp = (*lp).next
  }
  return funcblocksize;
}
unsafe extern "C" fn calcsize(mut funcblocksize: libc::c_int, mut n: *mut node) -> libc::c_int {
  if n.is_null() {
    return funcblocksize;
  }
  funcblocksize += nodesize[(*n).type_0 as usize] as libc::c_int;
  match (*n).type_0 as libc::c_int {
    0 => {
      funcblocksize = calcsize(funcblocksize, (*n).ncmd.redirect);
      funcblocksize = calcsize(funcblocksize, (*n).ncmd.args);
      funcblocksize = calcsize(funcblocksize, (*n).ncmd.assign)
    }
    1 => funcblocksize = sizenodelist(funcblocksize, (*n).npipe.cmdlist),
    2 | 3 | 4 => {
      funcblocksize = calcsize(funcblocksize, (*n).nredir.redirect);
      funcblocksize = calcsize(funcblocksize, (*n).nredir.n)
    }
    5 | 6 | 7 | 9 | 10 => {
      funcblocksize = calcsize(funcblocksize, (*n).nbinary.ch2);
      funcblocksize = calcsize(funcblocksize, (*n).nbinary.ch1)
    }
    8 => {
      funcblocksize = calcsize(funcblocksize, (*n).nif.elsepart);
      funcblocksize = calcsize(funcblocksize, (*n).nif.ifpart);
      funcblocksize = calcsize(funcblocksize, (*n).nif.test)
    }
    11 => {
      funcblocksize = (funcblocksize as libc::c_ulong).wrapping_add(
        strlen((*n).nfor.var)
          .wrapping_add(1i32 as libc::c_ulong)
          .wrapping_add(SHELL_SIZE as libc::c_int as libc::c_ulong)
          & !(SHELL_SIZE as libc::c_int) as libc::c_ulong,
      ) as libc::c_int as libc::c_int;
      funcblocksize = calcsize(funcblocksize, (*n).nfor.body);
      funcblocksize = calcsize(funcblocksize, (*n).nfor.args)
    }
    12 => {
      funcblocksize = calcsize(funcblocksize, (*n).ncase.cases);
      funcblocksize = calcsize(funcblocksize, (*n).ncase.expr)
    }
    13 => {
      funcblocksize = calcsize(funcblocksize, (*n).nclist.body);
      funcblocksize = calcsize(funcblocksize, (*n).nclist.pattern);
      funcblocksize = calcsize(funcblocksize, (*n).nclist.next)
    }
    14 => {
      funcblocksize = calcsize(funcblocksize, (*n).ndefun.body);
      funcblocksize = (funcblocksize as libc::c_ulong).wrapping_add(
        strlen((*n).ndefun.text)
          .wrapping_add(1i32 as libc::c_ulong)
          .wrapping_add(SHELL_SIZE as libc::c_int as libc::c_ulong)
          & !(SHELL_SIZE as libc::c_int) as libc::c_ulong,
      ) as libc::c_int as libc::c_int
    }
    15 => {
      funcblocksize = sizenodelist(funcblocksize, (*n).narg.backquote);
      funcblocksize = (funcblocksize as libc::c_ulong).wrapping_add(
        strlen((*n).narg.text)
          .wrapping_add(1i32 as libc::c_ulong)
          .wrapping_add(SHELL_SIZE as libc::c_int as libc::c_ulong)
          & !(SHELL_SIZE as libc::c_int) as libc::c_ulong,
      ) as libc::c_int as libc::c_int;
      funcblocksize = calcsize(funcblocksize, (*n).narg.next)
    }
    16 | 17 | 18 | 19 | 20 | 21 => {
      funcblocksize = calcsize(funcblocksize, (*n).nfile.fname);
      funcblocksize = calcsize(funcblocksize, (*n).nfile.next)
    }
    22 | 23 => {
      funcblocksize = calcsize(funcblocksize, (*n).ndup.vname);
      funcblocksize = calcsize(funcblocksize, (*n).ndup.next)
    }
    24 | 25 => {
      funcblocksize = calcsize(funcblocksize, (*n).nhere.doc);
      funcblocksize = calcsize(funcblocksize, (*n).nhere.next)
    }
    26 => funcblocksize = calcsize(funcblocksize, (*n).nnot.com),
    _ => {}
  }
  return funcblocksize;
}
unsafe extern "C" fn nodeckstrdup(mut s: *mut libc::c_char) -> *mut libc::c_char {
  funcstring_end = funcstring_end.offset(
    -((strlen(s)
      .wrapping_add(1i32 as libc::c_ulong)
      .wrapping_add(SHELL_SIZE as libc::c_int as libc::c_ulong)
      & !(SHELL_SIZE as libc::c_int) as libc::c_ulong) as isize),
  );
  return strcpy(funcstring_end, s);
}
unsafe extern "C" fn copynodelist(mut lp: *mut nodelist) -> *mut nodelist {
  let mut start: *mut nodelist = 0 as *mut nodelist;
  let mut lpp: *mut *mut nodelist = 0 as *mut *mut nodelist;
  lpp = &mut start;
  while !lp.is_null() {
    *lpp = funcblock as *mut nodelist;
    funcblock = (funcblock as *mut libc::c_char).offset(
      ((::std::mem::size_of::<nodelist>() as libc::c_ulong)
        .wrapping_add(SHELL_SIZE as libc::c_int as libc::c_ulong)
        & !(SHELL_SIZE as libc::c_int) as libc::c_ulong) as isize,
    ) as *mut libc::c_void;
    (**lpp).n = copynode((*lp).n);
    lp = (*lp).next;
    lpp = &mut (**lpp).next
  }
  *lpp = 0 as *mut nodelist;
  return start;
}
unsafe extern "C" fn copynode(mut n: *mut node) -> *mut node {
  let mut new: *mut node = 0 as *mut node;
  if n.is_null() {
    return 0 as *mut node;
  }
  new = funcblock as *mut node;
  funcblock = (funcblock as *mut libc::c_char)
    .offset(nodesize[(*n).type_0 as usize] as libc::c_int as isize)
    as *mut libc::c_void;
  match (*n).type_0 as libc::c_int {
    0 => {
      (*new).ncmd.redirect = copynode((*n).ncmd.redirect);
      (*new).ncmd.args = copynode((*n).ncmd.args);
      (*new).ncmd.assign = copynode((*n).ncmd.assign);
      (*new).ncmd.linno = (*n).ncmd.linno
    }
    1 => {
      (*new).npipe.cmdlist = copynodelist((*n).npipe.cmdlist);
      (*new).npipe.pipe_backgnd = (*n).npipe.pipe_backgnd
    }
    2 | 3 | 4 => {
      (*new).nredir.redirect = copynode((*n).nredir.redirect);
      (*new).nredir.n = copynode((*n).nredir.n);
      (*new).nredir.linno = (*n).nredir.linno
    }
    5 | 6 | 7 | 9 | 10 => {
      (*new).nbinary.ch2 = copynode((*n).nbinary.ch2);
      (*new).nbinary.ch1 = copynode((*n).nbinary.ch1)
    }
    8 => {
      (*new).nif.elsepart = copynode((*n).nif.elsepart);
      (*new).nif.ifpart = copynode((*n).nif.ifpart);
      (*new).nif.test = copynode((*n).nif.test)
    }
    11 => {
      (*new).nfor.var = nodeckstrdup((*n).nfor.var);
      (*new).nfor.body = copynode((*n).nfor.body);
      (*new).nfor.args = copynode((*n).nfor.args);
      (*new).nfor.linno = (*n).nfor.linno
    }
    12 => {
      (*new).ncase.cases = copynode((*n).ncase.cases);
      (*new).ncase.expr = copynode((*n).ncase.expr);
      (*new).ncase.linno = (*n).ncase.linno
    }
    13 => {
      (*new).nclist.body = copynode((*n).nclist.body);
      (*new).nclist.pattern = copynode((*n).nclist.pattern);
      (*new).nclist.next = copynode((*n).nclist.next)
    }
    14 => {
      (*new).ndefun.body = copynode((*n).ndefun.body);
      (*new).ndefun.text = nodeckstrdup((*n).ndefun.text);
      (*new).ndefun.linno = (*n).ndefun.linno
    }
    15 => {
      (*new).narg.backquote = copynodelist((*n).narg.backquote);
      (*new).narg.text = nodeckstrdup((*n).narg.text);
      (*new).narg.next = copynode((*n).narg.next)
    }
    16 | 17 | 18 | 19 | 20 | 21 => {
      (*new).nfile.fname = copynode((*n).nfile.fname);
      (*new).nfile.fd = (*n).nfile.fd;
      (*new).nfile.next = copynode((*n).nfile.next)
    }
    22 | 23 => {
      (*new).ndup.vname = copynode((*n).ndup.vname);
      (*new).ndup.dupfd = (*n).ndup.dupfd;
      (*new).ndup.fd = (*n).ndup.fd;
      (*new).ndup.next = copynode((*n).ndup.next)
    }
    24 | 25 => {
      (*new).nhere.doc = copynode((*n).nhere.doc);
      (*new).nhere.fd = (*n).nhere.fd;
      (*new).nhere.next = copynode((*n).nhere.next)
    }
    26 => (*new).nnot.com = copynode((*n).nnot.com),
    _ => {}
  }
  (*new).type_0 = (*n).type_0;
  return new;
}
/*
 * Make a copy of a parse tree.
 */
unsafe extern "C" fn copyfunc(mut n: *mut node) -> *mut funcnode {
  let mut f: *mut funcnode = 0 as *mut funcnode;
  let mut blocksize: size_t = 0;
  /*funcstringsize = 0;*/
  blocksize = 8u64.wrapping_add(calcsize(0i32, n) as libc::c_ulong);
  f = xzalloc(blocksize) as *mut funcnode;
  funcblock = (f as *mut libc::c_char).offset(8) as *mut libc::c_void;
  funcstring_end = (f as *mut libc::c_char).offset(blocksize as isize);
  copynode(n);
  /* f->count = 0; - ckzalloc did it */
  return f;
}
/*
 * Define a shell function.
 */
unsafe extern "C" fn defun(mut func: *mut node) {
  let mut entry: cmdentry = cmdentry {
    cmdtype: 0,
    u: param { index: 0 },
  };
  ::std::ptr::write_volatile(
    &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
    ::std::ptr::read_volatile::<libc::c_int>(
      &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
    ) + 1,
  );
  asm!("" : : : "memory" : "volatile");
  entry.cmdtype = 1i32 as smallint;
  entry.u.func = copyfunc(func);
  addcmdentry((*func).ndefun.text, &mut entry);
  int_on();
}
static mut evalskip: smallint = 0;
/* set to SKIPxxx if we are skipping commands */
static mut skipcount: libc::c_int = 0;
/* number of levels to skip */
static mut loopnest: libc::c_int = 0;
/* current loop nesting level */
static mut funcline: libc::c_int = 0;
/* Called to execute a trap.
 * Single callsite - at the end of evaltree().
 * If we return non-zero, evaltree raises EXEXIT exception.
 *
 * Perhaps we should avoid entering new trap handlers
 * while we are executing a trap handler. [is it a TODO?]
 */
unsafe extern "C" fn dotrap() {
  let mut g: *mut uint8_t = 0 as *mut uint8_t;
  let mut sig: libc::c_int = 0;
  let mut last_status: uint8_t = 0;
  if (*ash_ptr_to_globals_misc).pending_sig == 0 {
    return;
  }
  last_status = (*ash_ptr_to_globals_misc).exitstatus;
  ::std::ptr::write_volatile(
    &mut (*ash_ptr_to_globals_misc).pending_sig as *mut smallint,
    0i32 as smallint,
  );
  asm!("" : : : "memory" : "volatile");
  sig = 1i32;
  g = (*ash_ptr_to_globals_misc).gotsig.as_mut_ptr();
  while sig < 64i32 + 1i32 {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if !(*g == 0) {
      if evalskip != 0 {
        ::std::ptr::write_volatile(
          &mut (*ash_ptr_to_globals_misc).pending_sig as *mut smallint,
          sig as smallint,
        );
        break;
      } else {
        p = (*ash_ptr_to_globals_misc).trap[sig as usize];
        /* non-trapped SIGINT is handled separately by raise_interrupt,
         * don't upset it by resetting gotsig[SIGINT-1] */
        if !(sig == 2i32 && p.is_null()) {
          *g = 0i32 as uint8_t;
          if !p.is_null() {
            evalstring(p, 0i32);
          }
        }
      }
    }
    sig += 1;
    g = g.offset(1)
  }
  (*ash_ptr_to_globals_misc).exitstatus = last_status;
}
/* exit status is checked; ignore -e flag */
/*
 * Evaluate a parse tree.  The value is left in the global variable
 * exitstatus.
 */
unsafe extern "C" fn evaltree(mut n: *mut node, mut flags: libc::c_int) -> libc::c_int {
  let mut is_or: libc::c_uint = 0;
  let mut current_block: u64;
  let mut checkexit: libc::c_int = 0i32;
  let mut evalfn: Option<unsafe extern "C" fn(_: *mut node, _: libc::c_int) -> libc::c_int> = None;
  let mut smark: stackmark = stackmark {
    stackp: 0 as *mut stack_block,
    stacknxt: 0 as *mut libc::c_char,
    stacknleft: 0,
  };
  let mut status: libc::c_int = 0i32;
  setstackmark(&mut smark);
  if !n.is_null() {
    dotrap();
    match (*n).type_0 as libc::c_int {
      2 => {
        (*ash_ptr_to_globals_var).lineno = (*n).nredir.linno;
        (*ash_ptr_to_globals_misc).errlinno = (*ash_ptr_to_globals_var).lineno;
        if funcline != 0 {
          (*ash_ptr_to_globals_var).lineno -= funcline - 1i32
        }
        expredir((*n).nredir.redirect);
        pushredir((*n).nredir.redirect);
        status = redirectsafe((*n).nredir.redirect, 0o1i32);
        if status == 0 {
          status = evaltree((*n).nredir.n, flags & 0o2i32)
        }
        if !(*n).nredir.redirect.is_null() {
          popredir(0i32);
        }
        current_block = 15810881232493132710;
      }
      0 => {
        evalfn =
          Some(evalcommand as unsafe extern "C" fn(_: *mut node, _: libc::c_int) -> libc::c_int);
        current_block = 12167440363509275073;
      }
      11 => {
        evalfn = Some(evalfor as unsafe extern "C" fn(_: *mut node, _: libc::c_int) -> libc::c_int);
        current_block = 10582330624169799780;
      }
      9 | 10 => {
        evalfn =
          Some(evalloop as unsafe extern "C" fn(_: *mut node, _: libc::c_int) -> libc::c_int);
        current_block = 10582330624169799780;
      }
      4 | 3 => {
        evalfn =
          Some(evalsubshell as unsafe extern "C" fn(_: *mut node, _: libc::c_int) -> libc::c_int);
        current_block = 12167440363509275073;
      }
      1 => {
        evalfn =
          Some(evalpipe as unsafe extern "C" fn(_: *mut node, _: libc::c_int) -> libc::c_int);
        current_block = 12167440363509275073;
      }
      12 => {
        evalfn =
          Some(evalcase as unsafe extern "C" fn(_: *mut node, _: libc::c_int) -> libc::c_int);
        current_block = 10582330624169799780;
      }
      5 | 6 | 7 => {
        is_or = ((*n).type_0 as libc::c_int - 5i32) as libc::c_uint;
        status = evaltree(
          (*n).nbinary.ch1,
          ((flags as libc::c_uint | (is_or >> 1i32).wrapping_sub(1i32 as libc::c_uint))
            & 0o2i32 as libc::c_uint) as libc::c_int,
        );
        if (status == 0) as libc::c_int as libc::c_uint == is_or || evalskip as libc::c_int != 0 {
          current_block = 2978467952348202197;
        } else {
          n = (*n).nbinary.ch2;
          current_block = 15023758335967450927;
        }
      }
      8 => {
        status = evaltree((*n).nif.test, 0o2i32);
        if evalskip != 0 {
          current_block = 2978467952348202197;
        } else if status == 0 {
          n = (*n).nif.ifpart;
          current_block = 15023758335967450927;
        } else if !(*n).nif.elsepart.is_null() {
          n = (*n).nif.elsepart;
          current_block = 15023758335967450927;
        } else {
          status = 0i32;
          current_block = 15810881232493132710;
        }
      }
      14 => {
        defun(n);
        current_block = 15810881232493132710;
      }
      26 | _ => {
        status = (evaltree((*n).nnot.com, 0o2i32) == 0) as libc::c_int;
        current_block = 15810881232493132710;
      }
    }
    match current_block {
      2978467952348202197 => {}
      _ => {
        match current_block {
          15023758335967450927 => {
            evalfn =
              Some(evaltree as unsafe extern "C" fn(_: *mut node, _: libc::c_int) -> libc::c_int);
            current_block = 10582330624169799780;
          }
          12167440363509275073 => {
            if (*ash_ptr_to_globals_misc).optlist[0] as libc::c_int != 0 && flags & 0o2i32 == 0 {
              checkexit = !0i32
            }
            current_block = 10582330624169799780;
          }
          _ => {}
        }
        match current_block {
          10582330624169799780 => status = evalfn.expect("non-null function pointer")(n, flags),
          _ => {}
        }
        /* Not necessary. To test it:
         * "false; f() { qwerty; }; echo $?" should print 0.
         */
        /* status = 0; */
        (*ash_ptr_to_globals_misc).exitstatus = status as uint8_t
      }
    }
  }
  /* Order of checks below is important:
   * signal handlers trigger before exit caused by "set -e".
   */
  dotrap();
  if checkexit & status != 0 {
    raise_exception(4i32);
  }
  if flags & 0o1i32 != 0 {
    raise_exception(4i32);
  }
  popstackmark(&mut smark);
  return (*ash_ptr_to_globals_misc).exitstatus as libc::c_int;
}
unsafe extern "C" fn skiploop() -> libc::c_int {
  let mut skip: libc::c_int = evalskip as libc::c_int;
  match skip {
    1 | 2 => {
      skipcount -= 1;
      if skipcount <= 0i32 {
        evalskip = 0i32 as smallint
      } else {
        skip = 1i32 << 0i32
      }
    }
    0 | _ => {}
  }
  return skip;
}
/* forward declarations - evaluation is fairly recursive business... */
unsafe extern "C" fn evalloop(mut n: *mut node, mut flags: libc::c_int) -> libc::c_int {
  let mut skip: libc::c_int = 0;
  let mut status: libc::c_int = 0;
  loopnest += 1;
  status = 0i32;
  flags &= 0o2i32;
  loop {
    let mut i: libc::c_int = 0;
    i = evaltree((*n).nbinary.ch1, 0o2i32);
    skip = skiploop();
    if skip == 1i32 << 2i32 {
      status = i
    }
    if !(skip != 0) {
      if (*n).type_0 as libc::c_int != 9i32 {
        i = (i == 0) as libc::c_int
      }
      if i != 0i32 {
        break;
      }
      status = evaltree((*n).nbinary.ch2, flags);
      skip = skiploop()
    }
    if !(skip & !(1i32 << 1i32) == 0) {
      break;
    }
  }
  loopnest -= 1;
  return status;
}
unsafe extern "C" fn evalfor(mut n: *mut node, mut flags: libc::c_int) -> libc::c_int {
  let mut arglist: arglist = arglist {
    list: 0 as *const strlist as *mut strlist,
    lastp: 0 as *const *mut strlist as *mut *mut strlist,
  };
  let mut argp: *mut node = 0 as *mut node;
  let mut sp: *mut strlist = 0 as *mut strlist;
  let mut status: libc::c_int = 0i32;
  (*ash_ptr_to_globals_var).lineno = (*n).ncase.linno;
  (*ash_ptr_to_globals_misc).errlinno = (*ash_ptr_to_globals_var).lineno;
  if funcline != 0 {
    (*ash_ptr_to_globals_var).lineno -= funcline - 1i32
  }
  arglist.list = 0 as *mut strlist;
  arglist.lastp = &mut arglist.list;
  argp = (*n).nfor.args;
  while !argp.is_null() {
    expandarg(argp, &mut arglist, 0x1i32 | 0x2i32);
    argp = (*argp).narg.next
  }
  *arglist.lastp = 0 as *mut strlist;
  loopnest += 1;
  flags &= 0o2i32;
  sp = arglist.list;
  while !sp.is_null() {
    setvar0((*n).nfor.var, (*sp).text);
    status = evaltree((*n).nfor.body, flags);
    if skiploop() & !(1i32 << 1i32) != 0 {
      break;
    }
    sp = (*sp).next
  }
  loopnest -= 1;
  return status;
}
unsafe extern "C" fn evalcase(mut n: *mut node, mut flags: libc::c_int) -> libc::c_int {
  let mut cp: *mut node = 0 as *mut node;
  let mut patp: *mut node = 0 as *mut node;
  let mut arglist: arglist = arglist {
    list: 0 as *const strlist as *mut strlist,
    lastp: 0 as *const *mut strlist as *mut *mut strlist,
  };
  let mut status: libc::c_int = 0i32;
  (*ash_ptr_to_globals_var).lineno = (*n).ncase.linno;
  (*ash_ptr_to_globals_misc).errlinno = (*ash_ptr_to_globals_var).lineno;
  if funcline != 0 {
    (*ash_ptr_to_globals_var).lineno -= funcline - 1i32
  }
  arglist.list = 0 as *mut strlist;
  arglist.lastp = &mut arglist.list;
  expandarg((*n).ncase.expr, &mut arglist, 0x2i32);
  cp = (*n).ncase.cases;
  's_40: while !cp.is_null() && evalskip as libc::c_int == 0i32 {
    patp = (*cp).nclist.pattern;
    while !patp.is_null() {
      if casematch(patp, (*arglist.list).text) != 0 {
        /* Ensure body is non-empty as otherwise
         * EV_EXIT may prevent us from setting the
         * exit status.
         */
        if evalskip as libc::c_int == 0i32 && !(*cp).nclist.body.is_null() {
          status = evaltree((*cp).nclist.body, flags)
        }
        break 's_40;
      } else {
        patp = (*patp).narg.next
      }
    }
    cp = (*cp).nclist.next
  }
  return status;
}
/*
 * Kick off a subshell to evaluate a tree.
 */
unsafe extern "C" fn evalsubshell(mut n: *mut node, mut flags: libc::c_int) -> libc::c_int {
  let mut jp: *mut job = 0 as *mut job; /* FORK_BG(1) if yes, else FORK_FG(0) */
  let mut backgnd: libc::c_int = ((*n).type_0 as libc::c_int == 3i32) as libc::c_int;
  let mut status: libc::c_int = 0;
  (*ash_ptr_to_globals_var).lineno = (*n).nredir.linno;
  (*ash_ptr_to_globals_misc).errlinno = (*ash_ptr_to_globals_var).lineno;
  if funcline != 0 {
    (*ash_ptr_to_globals_var).lineno -= funcline - 1i32
  }
  expredir((*n).nredir.redirect);
  if !(backgnd == 0 && flags & 0o1i32 != 0 && (*ash_ptr_to_globals_misc).may_have_traps == 0) {
    ::std::ptr::write_volatile(
      &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
      ::std::ptr::read_volatile::<libc::c_int>(
        &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
      ) + 1,
    );
    asm!("" : : : "memory" : "volatile");

    // TODO: why was this translated this way?
    // (backgnd) == 0i32;

    jp = makejob(1i32);
    if forkshell(jp, n, backgnd) == 0i32 {
      /* child */
      int_on();
      flags |= 0o1i32;
      if backgnd != 0 {
        flags &= !0o2i32
      }
    /* never returns */
    } else {
      /* parent */
      status = 0i32;
      if backgnd == 0i32 {
        status = waitforjob(jp)
      }
      int_on();
      return status;
    }
  }
  redirect((*n).nredir.redirect, 0i32);
  evaltreenr((*n).nredir.n, flags);
}
unsafe extern "C" fn expredir(mut n: *mut node) {
  let mut redir: *mut node = 0 as *mut node;
  redir = n;
  while !redir.is_null() {
    let mut fn_0: arglist = arglist {
      list: 0 as *const strlist as *mut strlist,
      lastp: 0 as *const *mut strlist as *mut *mut strlist,
    };
    fn_0.list = 0 as *mut strlist;
    fn_0.lastp = &mut fn_0.list;
    let mut current_block_12: u64;
    match (*redir).type_0 as libc::c_int {
      20 | 19 | 16 | 17 | 18 | 21 => {
        expandarg((*redir).nfile.fname, &mut fn_0, 0x2i32 | 0x8i32);
        current_block_12 = 4820410498918392787;
      }
      23 | 22 => {
        /* >& */
        if !(*redir).ndup.vname.is_null() {
          expandarg((*redir).ndup.vname, &mut fn_0, 0x1i32 | 0x2i32);
          if fn_0.list.is_null() {
            ash_msg_and_raise_error(b"redir error\x00" as *const u8 as *const libc::c_char);
          }
          //FIXME: we used expandarg with different args!
          if isdigit_str9((*fn_0.list).text) == 0 {
            /* >&file, not >&fd */
            if (*redir).nfile.fd != 1i32 {
              /* 123>&file - BAD */
              ash_msg_and_raise_error(b"redir error\x00" as *const u8 as *const libc::c_char);
            }
            (*redir).type_0 = 17i32 as smallint;
            current_block_12 = 4820410498918392787;
          } else {
            fixredir(redir, (*fn_0.list).text, 1i32);
            current_block_12 = 15089075282327824602;
          }
        } else {
          current_block_12 = 15089075282327824602;
        }
      }
      _ => {
        current_block_12 = 15089075282327824602;
      }
    }
    match current_block_12 {
      4820410498918392787 => (*redir).nfile.expfname = (*fn_0.list).text,
      _ => {}
    }
    redir = (*redir).nfile.next
  }
}
/*
 * Evaluate a pipeline.  All the processes in the pipeline are children
 * of the process creating the pipeline.  (This differs from some versions
 * of the shell, which make the last process in a pipeline the parent
 * of all the rest.)
 */
unsafe extern "C" fn evalpipe(mut n: *mut node, mut flags: libc::c_int) -> libc::c_int {
  let mut jp: *mut job = 0 as *mut job;
  let mut lp: *mut nodelist = 0 as *mut nodelist;
  let mut pipelen: libc::c_int = 0;
  let mut prevfd: libc::c_int = 0;
  let mut pip: [libc::c_int; 2] = [0; 2];
  let mut status: libc::c_int = 0i32;
  pipelen = 0i32;
  lp = (*n).npipe.cmdlist;
  while !lp.is_null() {
    pipelen += 1;
    lp = (*lp).next
  }
  flags |= 0o1i32;
  ::std::ptr::write_volatile(
    &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
    ::std::ptr::read_volatile::<libc::c_int>(
      &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
    ) + 1,
  );
  asm!("" : : : "memory" : "volatile");

  // TODO: why was this translated this way?
  // ((*n).npipe.pipe_backgnd as libc::c_int) == 0i32;

  jp = makejob(pipelen);
  prevfd = -1i32;
  lp = (*n).npipe.cmdlist;
  while !lp.is_null() {
    prehash((*lp).n);
    pip[1] = -1i32;
    if !(*lp).next.is_null() {
      if pipe(pip.as_mut_ptr()) < 0i32 {
        close(prevfd);
        ash_msg_and_raise_error(b"can\'t create pipe: %m\x00" as *const u8 as *const libc::c_char);
      }
    }
    if forkshell(jp, (*lp).n, (*n).npipe.pipe_backgnd as libc::c_int) == 0i32 {
      /* child */
      int_on();
      if pip[1] >= 0i32 {
        close(pip[0]);
      }
      if prevfd > 0i32 {
        dup2(prevfd, 0i32);
        close(prevfd);
      }
      if pip[1] > 1i32 {
        dup2(pip[1], 1i32);
        close(pip[1]);
      }
      evaltreenr((*lp).n, flags);
      /* never returns */
    }
    /* parent */
    if prevfd >= 0i32 {
      close(prevfd);
    }
    prevfd = pip[0];
    /* Don't want to trigger debugging */
    if pip[1] != -1i32 {
      close(pip[1]);
    }
    lp = (*lp).next
  }
  if (*n).npipe.pipe_backgnd as libc::c_int == 0i32 {
    status = waitforjob(jp)
  }
  int_on();
  return status;
}
/*
 * Controls whether the shell is interactive or not.
 */
unsafe extern "C" fn setinteractive(mut on: libc::c_int) {
  static mut is_interactive: smallint = 0;
  on += 1;
  if on == is_interactive as libc::c_int {
    return;
  }
  is_interactive = on as smallint;
  setsignal(2i32);
  setsignal(3i32);
  setsignal(15i32);
  if is_interactive as libc::c_int > 1i32 {
    if line_input_state.is_null() {
      line_input_state =
        new_line_input_t(FOR_SHELL as libc::c_int | WITH_PATH_LOOKUP as libc::c_int)
    }
  };
}
unsafe extern "C" fn optschanged() {
  setinteractive((*ash_ptr_to_globals_misc).optlist[3] as libc::c_int);
  setjobctl((*ash_ptr_to_globals_misc).optlist[4] as libc::c_int);
  (*ash_ptr_to_globals_misc).optlist[14] = 0i32 as libc::c_char;
  /* forcibly keep the option off */
}
static mut localvar_stack: *mut localvar_list = 0 as *const localvar_list as *mut localvar_list;
/*
 * Called after a function returns.
 * Interrupts must be off.
 */
unsafe extern "C" fn poplocalvars(mut keep: libc::c_int) {
  let mut ll: *mut localvar_list = 0 as *mut localvar_list;
  let mut lvp: *mut localvar = 0 as *mut localvar;
  let mut next: *mut localvar = 0 as *mut localvar;
  let mut vp: *mut var = 0 as *mut var;
  ::std::ptr::write_volatile(
    &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
    ::std::ptr::read_volatile::<libc::c_int>(
      &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
    ) + 1,
  );
  asm!("" : : : "memory" : "volatile");
  ll = localvar_stack;
  localvar_stack = (*ll).next;
  next = (*ll).lv;
  free(ll as *mut libc::c_void);
  loop {
    lvp = next;
    if lvp.is_null() {
      break;
    }
    next = (*lvp).next;
    vp = (*lvp).vp;
    if keep != 0 {
      let mut bits: libc::c_int = 0x4i32;
      if (*lvp).flags != 0x20i32 {
        if (*vp).var_text == (*lvp).text {
          bits |= 0x8i32
        } else if (*lvp).flags & (0x8i32 | 0x10i32) == 0 {
          free((*lvp).text as *mut libc::c_char as *mut libc::c_void);
        }
      }
      (*vp).flags &= !bits;
      (*vp).flags |= (*lvp).flags & bits;
      if (*vp).flags & (0x1i32 | 0x2i32 | 0x4i32 | 0x20i32) == 0x20i32 {
        unsetvar((*vp).var_text);
      }
    } else if vp.is_null() {
      /* $- saved */
      memcpy(
        (*ash_ptr_to_globals_misc).optlist.as_mut_ptr() as *mut libc::c_void,
        (*lvp).text as *const libc::c_void,
        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
      );
      free((*lvp).text as *mut libc::c_char as *mut libc::c_void);
      optschanged();
    } else if (*lvp).flags == 0x20i32 {
      (*vp).flags &= !(0x4i32 | 0x2i32);
      unsetvar((*vp).var_text);
    } else {
      if (*vp).var_func.is_some() {
        (*vp).var_func.expect("non-null function pointer")(var_end((*lvp).text));
      }
      if (*vp).flags & (0x8i32 | 0x10i32) == 0i32 {
        free((*vp).var_text as *mut libc::c_char as *mut libc::c_void);
      }
      (*vp).flags = (*lvp).flags;
      (*vp).var_text = (*lvp).text
    }
    free(lvp as *mut libc::c_void);
  }
  int_on();
}
/*
 * Create a new localvar environment.
 */
unsafe extern "C" fn pushlocalvars() -> *mut localvar_list {
  let mut ll: *mut localvar_list = 0 as *mut localvar_list;
  ::std::ptr::write_volatile(
    &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
    ::std::ptr::read_volatile::<libc::c_int>(
      &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
    ) + 1,
  );
  asm!("" : : : "memory" : "volatile");
  ll = xzalloc(::std::mem::size_of::<localvar_list>() as libc::c_ulong) as *mut localvar_list;
  /*ll->lv = NULL; - zalloc did it */
  (*ll).next = localvar_stack;
  localvar_stack = ll;
  int_on();
  return (*ll).next;
}
unsafe extern "C" fn unwindlocalvars(mut stop: *mut localvar_list) {
  while localvar_stack != stop {
    poplocalvars(0i32);
  }
}
unsafe extern "C" fn evalfun(
  mut func: *mut funcnode,
  mut argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
  mut flags: libc::c_int,
) -> libc::c_int {
  let mut saveparam: shparam = shparam {
    nparam: 0,
    optind: 0,
    optoff: 0,
    malloced: 0,
    p: 0 as *mut *mut libc::c_char,
  };
  let mut savehandler: *mut jmploc = 0 as *mut jmploc;
  let mut jmploc: jmploc = jmploc {
    loc: [__jmp_buf_tag {
      __jmpbuf: [0; 8],
      __mask_was_saved: 0,
      __saved_mask: __sigset_t { __val: [0; 16] },
    }; 1],
  };
  let mut e: libc::c_int = 0;
  let mut savefuncline: libc::c_int = 0;
  ::std::ptr::write_volatile(
    &mut saveparam as *mut shparam,
    (*ash_ptr_to_globals_var).shellparam,
  );
  savefuncline = funcline;
  ::std::ptr::write_volatile(
    &mut savehandler as *mut *mut jmploc,
    (*ash_ptr_to_globals_misc).exception_handler,
  );
  e = _setjmp(jmploc.loc.as_mut_ptr());
  if !(e != 0) {
    ::std::ptr::write_volatile(
      &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
      ::std::ptr::read_volatile::<libc::c_int>(
        &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
      ) + 1,
    );
    asm!("" : : : "memory" : "volatile");
    (*ash_ptr_to_globals_misc).exception_handler = &mut jmploc;
    (*ash_ptr_to_globals_var).shellparam.malloced = 0i32 as libc::c_uchar;
    (*func).count += 1;
    funcline = (*func).n.ndefun.linno;
    int_on();
    (*ash_ptr_to_globals_var).shellparam.nparam = argc - 1i32;
    (*ash_ptr_to_globals_var).shellparam.p = argv.offset(1);
    (*ash_ptr_to_globals_var).shellparam.optind = 1i32;
    (*ash_ptr_to_globals_var).shellparam.optoff = -1i32;
    evaltree((*func).n.ndefun.body, flags & 0o2i32);
  }
  ::std::ptr::write_volatile(
    &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
    ::std::ptr::read_volatile::<libc::c_int>(
      &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
    ) + 1,
  );
  asm!("" : : : "memory" : "volatile");
  funcline = savefuncline;
  freefunc(func);
  freeparam(&mut (*ash_ptr_to_globals_var).shellparam as *mut shparam as *mut shparam);
  (*ash_ptr_to_globals_var).shellparam = saveparam;
  (*ash_ptr_to_globals_misc).exception_handler = savehandler;
  int_on();
  evalskip = (evalskip as libc::c_int & !(1i32 << 2i32)) as smallint;
  return e;
}
/*
 * Make a variable a local variable.  When a variable is made local, it's
 * value and flags are saved in a localvar structure.  The saved values
 * will be restored when the shell function returns.  We handle the name
 * "-" as a special case: it makes changes to "set +-options" local
 * (options will be restored on return from the function).
 */
unsafe extern "C" fn mklocal(mut name: *mut libc::c_char) {
  let mut current_block: u64;
  let mut lvp: *mut localvar = 0 as *mut localvar;
  let mut vpp: *mut *mut var = 0 as *mut *mut var;
  let mut vp: *mut var = 0 as *mut var;
  let mut eq: *mut libc::c_char = strchr(name, '=' as i32);
  ::std::ptr::write_volatile(
    &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
    ::std::ptr::read_volatile::<libc::c_int>(
      &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
    ) + 1,
  );
  asm!("" : : : "memory" : "volatile");
  /* Cater for duplicate "local". Examples:
   * x=0; f() { local x=1; echo $x; local x; echo $x; }; f; echo $x
   * x=0; f() { local x=1; echo $x; local x=2; echo $x; }; f; echo $x
   */
  lvp = (*localvar_stack).lv;
  loop {
    if lvp.is_null() {
      current_block = 11050875288958768710;
      break;
    }
    if !(*lvp).vp.is_null() && varcmp((*(*lvp).vp).var_text, name) == 0i32 {
      if !eq.is_null() {
        setvareq(name, 0i32);
      }
      current_block = 4720956521144816148;
      break;
    } else {
      lvp = (*lvp).next
    }
  }
  match current_block {
    11050875288958768710 => {
      lvp = xzalloc(::std::mem::size_of::<localvar>() as libc::c_ulong) as *mut localvar;
      if *name.offset(0) as libc::c_int == '-' as i32 && *name.offset(1) == 0 {
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        p = xmalloc(::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
          as *mut libc::c_char;
        (*lvp).text = memcpy(
          p as *mut libc::c_void,
          (*ash_ptr_to_globals_misc).optlist.as_mut_ptr() as *const libc::c_void,
          ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
        ) as *const libc::c_char;
        vp = 0 as *mut var
      } else {
        vpp = hashvar(name);
        vp = *findvar(vpp, name);
        if vp.is_null() {
          /* variable did not exist yet */
          if !eq.is_null() {
            vp = setvareq(name, 0x4i32)
          } else {
            vp = setvar(name, 0 as *const libc::c_char, 0x4i32)
          }
          (*lvp).flags = 0x20i32
        } else {
          (*lvp).text = (*vp).var_text;
          (*lvp).flags = (*vp).flags;
          /* make sure neither "struct var" nor string gets freed
           * during (un)setting:
           */
          (*vp).flags |= 0x4i32 | 0x8i32;
          if !eq.is_null() {
            setvareq(name, 0i32);
          } else {
            /* "local VAR" unsets VAR: */
            setvar0(name, 0 as *const libc::c_char);
          }
        }
      }
      (*lvp).vp = vp;
      (*lvp).next = (*localvar_stack).lv;
      (*localvar_stack).lv = lvp
    }
    _ => {}
  }
  /* else:
   * it's a duplicate "local VAR" declaration, do nothing
   */
  int_on();
}
/*
 * The "local" command.
 */
unsafe extern "C" fn localcmd(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
  if localvar_stack.is_null() {
    ash_msg_and_raise_error(b"not in a function\x00" as *const u8 as *const libc::c_char);
  }
  argv = argptr;
  loop {
    let fresh65 = argv;
    argv = argv.offset(1);
    name = *fresh65;
    if name.is_null() {
      break;
    }
    mklocal(name);
  }
  return 0i32;
}
unsafe extern "C" fn falsecmd(
  mut _argc: libc::c_int,
  mut _argv: *mut *mut libc::c_char,
) -> libc::c_int {
  return 1i32;
}
unsafe extern "C" fn truecmd(
  mut _argc: libc::c_int,
  mut _argv: *mut *mut libc::c_char,
) -> libc::c_int {
  return 0i32;
}
unsafe extern "C" fn execcmd(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  optionarg = 0 as *mut libc::c_char;
  /* nextopt() sets optionarg to "-a ARGV0" */
  while nextopt(b"a:\x00" as *const u8 as *const libc::c_char) != '\u{0}' as i32 {}
  argv = argptr;
  if !(*argv.offset(0)).is_null() {
    let mut prog: *mut libc::c_char = 0 as *mut libc::c_char;
    /* NOTREACHED */
    (*ash_ptr_to_globals_misc).optlist[3] = 0i32 as libc::c_char; /* exit on error */
    (*ash_ptr_to_globals_misc).optlist[4] = 0i32 as libc::c_char;
    optschanged();
    (*ash_ptr_to_globals_misc).shlvl += 1;
    setsignal(3i32);
    prog = *argv.offset(0);
    if !optionarg.is_null() {
      let ref mut fresh66 = *argv.offset(0);
      *fresh66 = optionarg
    }
    shellexec(
      prog,
      argv,
      (*ash_ptr_to_globals_var).varinit[(1i32 * 2i32 + 1i32) as usize]
        .var_text
        .offset(5),
      0i32,
    );
  }
  return 0i32;
}
/* We should set up signals for "exec CMD"
 * the same way as for "CMD" without "exec".
 * But optschanged->setinteractive->setsignal
 * still thought we are a root shell. Therefore, for example,
 * SIGQUIT is still set to IGN. Fix it:
 */
/*setsignal(SIGTERM); - unnecessary because of iflag=0 */
/*setsignal(SIGTSTP); - unnecessary because of mflag=0 */
/*setsignal(SIGTTOU); - unnecessary because of mflag=0 */
/*
 * The return command.
 */
unsafe extern "C" fn returncmd(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  /*
   * If called outside a function, do what ksh does;
   * skip the rest of the file.
   */
  evalskip = (1i32 << 2i32) as smallint;
  return if !(*argv.offset(1)).is_null() {
    number(*argv.offset(1))
  } else {
    (*ash_ptr_to_globals_misc).exitstatus as libc::c_int
  };
}
/* Stubs for calling non-FAST_FUNC's */
unsafe extern "C" fn echocmd(
  mut argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  return echo_main(argc, argv);
}
unsafe extern "C" fn printfcmd(
  mut argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  return printf_main(argc, argv);
}
unsafe extern "C" fn testcmd(
  mut argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  return test_main(argc, argv);
}
/* Keep these in proper order since it is searched via bsearch() */
static mut builtintab: [builtincmd; 44] =[
    {
      let mut init = builtincmd {
        name: b"3.\x00" as *const u8 as *const libc::c_char,
        builtin: Some(
          dotcmd as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
        ),
      };
      init
    },
    {
      let mut init = builtincmd {
        name: b"3:\x00" as *const u8 as *const libc::c_char,
        builtin: Some(
          truecmd as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
        ),
      };
      init
    },
    {
      let mut init = builtincmd {
        name: b"2[\x00" as *const u8 as *const libc::c_char,
        builtin: Some(
          testcmd as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
        ),
      };
      init
    },
    {
      let mut init = builtincmd {
        name: b"2[[\x00" as *const u8 as *const libc::c_char,
        builtin: Some(
          testcmd as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
        ),
      };
      init
    },
    {
      let mut init = builtincmd {
        name: b"6alias\x00" as *const u8 as *const libc::c_char,
        builtin: Some(
          aliascmd
            as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
        ),
      };
      init
    },
    {
      let mut init = builtincmd {
        name: b"2bg\x00" as *const u8 as *const libc::c_char,
        builtin: Some(
          fg_bgcmd
            as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
        ),
      };
      init
    },
    {
      let mut init = builtincmd {
        name: b"3break\x00" as *const u8 as *const libc::c_char,
        builtin: Some(
          breakcmd
            as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
        ),
      };
      init
    },
    {
      let mut init = builtincmd {
        name: b"2cd\x00" as *const u8 as *const libc::c_char,
        builtin: Some(
          cdcmd as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
        ),
      };
      init
    },
    {
      let mut init = builtincmd {
        name: b"0chdir\x00" as *const u8 as *const libc::c_char,
        builtin: Some(
          cdcmd as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
        ),
      };
      init
    },
    {
      let mut init = builtincmd {
        name: b"2command\x00" as *const u8 as *const libc::c_char,
        builtin: Some(
          commandcmd
            as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
        ),
      };
      init
    },
    {
      let mut init = builtincmd {
        name: b"3continue\x00" as *const u8 as *const libc::c_char,
        builtin: Some(
          breakcmd
            as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
        ),
      };
      init
    },
    {
      let mut init = builtincmd {
        name: b"2echo\x00" as *const u8 as *const libc::c_char,
        builtin: Some(
          echocmd as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
        ),
      };
      init
    },
    {
      let mut init = builtincmd {
        name: b"3eval\x00" as *const u8 as *const libc::c_char,
        builtin: None,
      };
      init
    },
    {
      let mut init = builtincmd {
        name: b"3exec\x00" as *const u8 as *const libc::c_char,
        builtin: Some(
          execcmd as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
        ),
      };
      init
    },
    {
      let mut init = builtincmd {
        name: b"3exit\x00" as *const u8 as *const libc::c_char,
        builtin: Some(
          exitcmd as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
        ),
      };
      init
    },
    {
      let mut init = builtincmd {
        name: b"7export\x00" as *const u8 as *const libc::c_char,
        builtin: Some(
          exportcmd
            as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
        ),
      };
      init
    },
    {
      let mut init = builtincmd {
        name: b"2false\x00" as *const u8 as *const libc::c_char,
        builtin: Some(
          falsecmd
            as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
        ),
      };
      init
    },
    {
      let mut init = builtincmd {
        name: b"2fg\x00" as *const u8 as *const libc::c_char,
        builtin: Some(
          fg_bgcmd
            as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
        ),
      };
      init
    },
    {
      let mut init = builtincmd {
        name: b"2getopts\x00" as *const u8 as *const libc::c_char,
        builtin: Some(
          getoptscmd
            as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
        ),
      };
      init
    },
    {
      let mut init = builtincmd {
        name: b"0hash\x00" as *const u8 as *const libc::c_char,
        builtin: Some(
          hashcmd as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
        ),
      };
      init
    },
    {
      let mut init = builtincmd {
        name: b"0help\x00" as *const u8 as *const libc::c_char,
        builtin: Some(
          helpcmd as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
        ),
      };
      init
    },
    {
      let mut init = builtincmd {
        name: b"0history\x00" as *const u8 as *const libc::c_char,
        builtin: Some(
          historycmd
            as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
        ),
      };
      init
    },
    {
      let mut init = builtincmd {
        name: b"2jobs\x00" as *const u8 as *const libc::c_char,
        builtin: Some(
          jobscmd as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
        ),
      };
      init
    },
    {
      let mut init = builtincmd {
        name: b"2kill\x00" as *const u8 as *const libc::c_char,
        builtin: Some(
          killcmd as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
        ),
      };
      init
    },
    {
      let mut init = builtincmd {
        name: b"0let\x00" as *const u8 as *const libc::c_char,
        builtin: Some(
          letcmd as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
        ),
      };
      init
    },
    {
      let mut init = builtincmd {
        name: b"7local\x00" as *const u8 as *const libc::c_char,
        builtin: Some(
          localcmd
            as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
        ),
      };
      init
    },
    {
      let mut init = builtincmd {
        name: b"2printf\x00" as *const u8 as *const libc::c_char,
        builtin: Some(
          printfcmd
            as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
        ),
      };
      init
    },
    {
      let mut init = builtincmd {
        name: b"0pwd\x00" as *const u8 as *const libc::c_char,
        builtin: Some(
          pwdcmd as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
        ),
      };
      init
    },
    {
      let mut init = builtincmd {
        name: b"2read\x00" as *const u8 as *const libc::c_char,
        builtin: Some(
          readcmd as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
        ),
      };
      init
    },
    {
      let mut init = builtincmd {
        name: b"7readonly\x00" as *const u8 as *const libc::c_char,
        builtin: Some(
          exportcmd
            as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
        ),
      };
      init
    },
    {
      let mut init = builtincmd {
        name: b"3return\x00" as *const u8 as *const libc::c_char,
        builtin: Some(
          returncmd
            as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
        ),
      };
      init
    },
    {
      let mut init = builtincmd {
        name: b"3set\x00" as *const u8 as *const libc::c_char,
        builtin: Some(
          setcmd as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
        ),
      };
      init
    },
    {
      let mut init = builtincmd {
        name: b"3shift\x00" as *const u8 as *const libc::c_char,
        builtin: Some(
          shiftcmd
            as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
        ),
      };
      init
    },
    {
      let mut init = builtincmd {
        name: b"3source\x00" as *const u8 as *const libc::c_char,
        builtin: Some(
          dotcmd as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
        ),
      };
      init
    },
    {
      let mut init = builtincmd {
        name: b"2test\x00" as *const u8 as *const libc::c_char,
        builtin: Some(
          testcmd as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
        ),
      };
      init
    },
    {
      let mut init = builtincmd {
        name: b"3times\x00" as *const u8 as *const libc::c_char,
        builtin: Some(
          timescmd
            as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
        ),
      };
      init
    },
    {
      let mut init = builtincmd {
        name: b"3trap\x00" as *const u8 as *const libc::c_char,
        builtin: Some(
          trapcmd as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
        ),
      };
      init
    },
    {
      let mut init = builtincmd {
        name: b"2true\x00" as *const u8 as *const libc::c_char,
        builtin: Some(
          truecmd as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
        ),
      };
      init
    },
    {
      let mut init = builtincmd {
        name: b"0type\x00" as *const u8 as *const libc::c_char,
        builtin: Some(
          typecmd as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
        ),
      };
      init
    },
    {
      let mut init = builtincmd {
        name: b"0ulimit\x00" as *const u8 as *const libc::c_char,
        builtin: Some(
          ulimitcmd
            as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
        ),
      };
      init
    },
    {
      let mut init = builtincmd {
        name: b"2umask\x00" as *const u8 as *const libc::c_char,
        builtin: Some(
          umaskcmd
            as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
        ),
      };
      init
    },
    {
      let mut init = builtincmd {
        name: b"2unalias\x00" as *const u8 as *const libc::c_char,
        builtin: Some(
          unaliascmd
            as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
        ),
      };
      init
    },
    {
      let mut init = builtincmd {
        name: b"3unset\x00" as *const u8 as *const libc::c_char,
        builtin: Some(
          unsetcmd
            as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
        ),
      };
      init
    },
    {
      let mut init = builtincmd {
        name: b"2wait\x00" as *const u8 as *const libc::c_char,
        builtin: Some(
          waitcmd as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
        ),
      };
      init
    },
  ];
/*
 * Search the table of builtin commands.
 */
unsafe extern "C" fn pstrcmp1(
  mut a: *const libc::c_void,
  mut b: *const libc::c_void,
) -> libc::c_int {
  return strcmp(
    a as *mut libc::c_char,
    (*(b as *mut *mut libc::c_char)).offset(1),
  );
}
unsafe extern "C" fn find_builtin(mut name: *const libc::c_char) -> *mut builtincmd {
  let mut bp: *mut builtincmd = 0 as *mut builtincmd;
  bp = bsearch(
    name as *const libc::c_void,
    builtintab.as_ptr() as *const libc::c_void,
    (::std::mem::size_of::<[builtincmd; 44]>() as libc::c_ulong)
      .wrapping_div(::std::mem::size_of::<builtincmd>() as libc::c_ulong) as libc::c_uint
      as size_t,
    ::std::mem::size_of::<builtincmd>() as libc::c_ulong,
    Some(
      pstrcmp1
        as unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> libc::c_int,
    ),
  ) as *mut builtincmd;
  return bp;
}
unsafe extern "C" fn isassignment(mut p: *const libc::c_char) -> libc::c_int {
  let mut q: *const libc::c_char = endofname(p);
  if p == q {
    return 0i32;
  }
  return (*q as libc::c_int == '=' as i32) as libc::c_int;
}
unsafe extern "C" fn bltincmd(
  mut _argc: libc::c_int,
  mut _argv: *mut *mut libc::c_char,
) -> libc::c_int {
  /* Preserve exitstatus of a previous possible redirection
   * as POSIX mandates */
  return (*ash_ptr_to_globals_misc).back_exitstatus as libc::c_int;
}
unsafe extern "C" fn evalcommand(mut cmd: *mut node, mut flags: libc::c_int) -> libc::c_int {
  let mut current_block: u64;
  static mut null_bltin: builtincmd =
    {
      let mut init = builtincmd {
        name: b"\x00\x00\x00" as *const u8 as *const libc::c_char,
        builtin: Some(
          bltincmd
            as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
        ),
      };
      init
    };
  let mut localvar_stop: *mut localvar_list = 0 as *mut localvar_list;
  let mut file_stop: *mut parsefile = 0 as *mut parsefile;
  let mut redir_stop: *mut redirtab = 0 as *mut redirtab;
  let mut argp: *mut node = 0 as *mut node;
  let mut arglist: arglist = arglist {
    list: 0 as *const strlist as *mut strlist,
    lastp: 0 as *const *mut strlist as *mut *mut strlist,
  };
  let mut varlist: arglist = arglist {
    list: 0 as *const strlist as *mut strlist,
    lastp: 0 as *const *mut strlist as *mut *mut strlist,
  };
  let mut argv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  let mut argc: libc::c_int = 0;
  let mut sp: *const strlist = 0 as *const strlist;
  let mut cmdentry: cmdentry = cmdentry {
    cmdtype: 0,
    u: param { index: 0 },
  };
  let mut jp: *mut job = 0 as *mut job;
  let mut lastarg: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut path: *const libc::c_char = 0 as *const libc::c_char;
  let mut spclbltin: libc::c_int = 0;
  let mut status: libc::c_int = 0;
  let mut nargv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  let mut cmd_is_exec: smallint = 0;
  (*ash_ptr_to_globals_var).lineno = (*cmd).ncmd.linno;
  (*ash_ptr_to_globals_misc).errlinno = (*ash_ptr_to_globals_var).lineno;
  if funcline != 0 {
    (*ash_ptr_to_globals_var).lineno -= funcline - 1i32
  }
  /* First expand the arguments. */
  localvar_stop = pushlocalvars();
  file_stop = g_parsefile;
  (*ash_ptr_to_globals_misc).back_exitstatus = 0i32 as uint8_t;
  cmdentry.cmdtype = 2i32 as smallint;
  cmdentry.u.cmd = &null_bltin;
  varlist.lastp = &mut varlist.list;
  *varlist.lastp = 0 as *mut strlist;
  arglist.lastp = &mut arglist.list;
  *arglist.lastp = 0 as *mut strlist;
  argc = 0i32;
  if !(*cmd).ncmd.args.is_null() {
    let mut bcmd: *mut builtincmd = 0 as *mut builtincmd;
    let mut pseudovarflag: smallint = 0;
    bcmd = find_builtin((*(*cmd).ncmd.args).narg.text);
    pseudovarflag = (!bcmd.is_null() && *(*bcmd).name.offset(0) as libc::c_int & 4i32 != 0)
      as libc::c_int as smallint;
    argp = (*cmd).ncmd.args;
    while !argp.is_null() {
      let mut spp: *mut *mut strlist = 0 as *mut *mut strlist;
      spp = arglist.lastp;
      if pseudovarflag as libc::c_int != 0 && isassignment((*argp).narg.text) != 0 {
        expandarg(argp, &mut arglist, 0x4i32);
      } else {
        expandarg(argp, &mut arglist, 0x1i32 | 0x2i32);
      }
      sp = *spp;
      while !sp.is_null() {
        argc += 1;
        sp = (*sp).next
      }
      argp = (*argp).narg.next
    }
  }
  /* Reserve one extra spot at the front for shellexec. */
  nargv = stalloc(
    (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
      .wrapping_mul((argc + 2i32) as libc::c_ulong),
  ) as *mut *mut libc::c_char;
  nargv = nargv.offset(1);
  argv = nargv;
  sp = arglist.list;
  while !sp.is_null() {
    let fresh67 = nargv;
    nargv = nargv.offset(1);
    *fresh67 = (*sp).text;
    sp = (*sp).next
  }
  *nargv = 0 as *mut libc::c_char;
  lastarg = 0 as *mut libc::c_char;
  if (*ash_ptr_to_globals_misc).optlist[3] as libc::c_int != 0 && funcline == 0i32 && argc > 0i32 {
    lastarg = *nargv.offset(-1i32 as isize)
  }
  expredir((*cmd).ncmd.redirect);
  redir_stop = pushredir((*cmd).ncmd.redirect);
  (*ash_ptr_to_globals_var).preverrout_fd = 2i32;
  if 1i32 != 0 && (*ash_ptr_to_globals_misc).optlist[8] as libc::c_int != 0 {
    /* NB: bash closes fd == $BASH_XTRACEFD when it is changed.
     * we do not emulate this. We only use its value.
     */
    let mut xtracefd: *const libc::c_char =
      lookupvar(b"BASH_XTRACEFD\x00" as *const u8 as *const libc::c_char);
    if !xtracefd.is_null() && is_number(xtracefd) != 0 {
      (*ash_ptr_to_globals_var).preverrout_fd = atoi(xtracefd)
    }
  }
  status = redirectsafe((*cmd).ncmd.redirect, 0o1i32 | 0i32);
  path = (*ash_ptr_to_globals_var).varinit[(1i32 * 2i32 + 1i32) as usize].var_text;
  argp = (*cmd).ncmd.assign;
  while !argp.is_null() {
    let mut spp_0: *mut *mut strlist = 0 as *mut *mut strlist;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    spp_0 = varlist.lastp;
    expandarg(argp, &mut varlist, 0x4i32);
    mklocal((**spp_0).text);
    /*
     * Modify the command lookup path, if a PATH= assignment
     * is present
     */
    p = (**spp_0).text;
    if varcmp(p, path) == 0i32 {
      path = p
    }
    argp = (*argp).narg.next
  }
  /* Print the command if xflag is set. */
  if (*ash_ptr_to_globals_misc).optlist[8] != 0 {
    let mut pfx: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
    dprintf(
      (*ash_ptr_to_globals_var).preverrout_fd,
      b"%s\x00" as *const u8 as *const libc::c_char,
      expandstr(
        (*ash_ptr_to_globals_var).varinit[(1i32 * 2i32 + 4i32) as usize]
          .var_text
          .offset(4),
        1i32,
      ),
    );
    sp = varlist.list;
    while !sp.is_null() {
      let mut varval: *mut libc::c_char = (*sp).text;
      let mut eq: *mut libc::c_char = strchrnul(varval, '=' as i32);
      if *eq != 0 {
        eq = eq.offset(1)
      }
      dprintf(
        (*ash_ptr_to_globals_var).preverrout_fd,
        b"%s%.*s%s\x00" as *const u8 as *const libc::c_char,
        pfx,
        eq.wrapping_offset_from(varval) as libc::c_long as libc::c_int,
        varval,
        maybe_single_quote(eq),
      );
      sp = (*sp).next;
      pfx = b" \x00" as *const u8 as *const libc::c_char
    }
    sp = arglist.list;
    while !sp.is_null() {
      dprintf(
        (*ash_ptr_to_globals_var).preverrout_fd,
        b"%s%s\x00" as *const u8 as *const libc::c_char,
        pfx,
        if !findkwd((*sp).text).is_null() {
          single_quote((*sp).text)
        } else {
          maybe_single_quote((*sp).text)
        },
      );
      sp = (*sp).next;
      pfx = b" \x00" as *const u8 as *const libc::c_char
    }
    safe_write(
      (*ash_ptr_to_globals_var).preverrout_fd,
      b"\n\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
      1i32 as size_t,
    );
  }
  cmd_is_exec = 0i32 as smallint;
  spclbltin = -1i32;
  /* Now locate the command. */
  if argc != 0 {
    let mut cmd_flag: libc::c_int = 0x1i32;
    let mut oldpath: *const libc::c_char = path.offset(5);
    path = path.offset(5);
    loop {
      find_command(*argv.offset(0), &mut cmdentry, cmd_flag, path);
      if cmdentry.cmdtype as libc::c_int == -1i32 {
        flush_stdout_stderr();
        status = 127i32;
        current_block = 9509359022306583830;
        break;
      } else {
        /* implement bltin and command here */
        if cmdentry.cmdtype as libc::c_int != 2i32 {
          current_block = 4804377075063615140;
          break;
        }
        if spclbltin < 0i32 {
          spclbltin = *(*cmdentry.u.cmd).name.offset(0) as libc::c_int & 1i32
        }
        if cmdentry.u.cmd
          == builtintab
            .as_ptr()
            .offset(2)
            .offset((1i32 * 1i32) as isize)
            .offset((1i32 * (1i32 * 1i32)) as isize)
            .offset((1i32 * 1i32) as isize)
            .offset((1i32 * 1i32) as isize)
            .offset(3)
            .offset((1i32 * 1i32) as isize)
            .offset(1)
            .offset((1i32 * 1i32) as isize)
            .offset(0)
            .offset(1)
        {
          cmd_is_exec = 1i32 as smallint
        }
        if !(cmdentry.u.cmd
          == builtintab
            .as_ptr()
            .offset(2)
            .offset((1i32 * 1i32) as isize)
            .offset((1i32 * (1i32 * 1i32)) as isize)
            .offset((1i32 * 1i32) as isize)
            .offset((1i32 * 1i32) as isize)
            .offset(3))
        {
          current_block = 4804377075063615140;
          break;
        }
        path = oldpath;
        nargv = parse_command_args(argv, &mut path);
        if nargv.is_null() {
          current_block = 4804377075063615140;
          break;
        }
        /* It's "command [-p] PROG ARGS" (that is, no -Vv).
         * nargv => "PROG". path is updated if -p.
         */
        argc =
          (argc as libc::c_long - nargv.wrapping_offset_from(argv) as libc::c_long) as libc::c_int;
        argv = nargv;
        cmd_flag |= 0x4i32
      }
    }
  } else {
    current_block = 4804377075063615140;
  }
  match current_block {
    4804377075063615140 => {
      if status != 0 {
        current_block = 9509359022306583830;
      } else {
        /* Execute the command. */
        match cmdentry.cmdtype as libc::c_int {
          2 => {
            if spclbltin > 0i32 || argc == 0i32 {
              poplocalvars(1i32); /* switch */
              if cmd_is_exec as libc::c_int != 0 && argc > 1i32 {
                listsetvar(varlist.list, 0x1i32);
              }
            }
            /* Tight loop with builtins only:
             * "while kill -0 $child; do true; done"
             * will never exit even if $child died, unless we do this
             * to reap the zombie and make kill detect that it's gone: */
            dowait(0i32, 0 as *mut job);
            if evalbltin(cmdentry.u.cmd, argc, argv, flags) != 0 {
              if (*ash_ptr_to_globals_misc).exception_type as libc::c_int == 1i32
                && spclbltin <= 0i32
              {
                force_int_on();
                current_block = 9756042043304152679;
              } else {
                current_block = 1439023131555407698;
              }
            } else {
              current_block = 9756042043304152679;
            }
          }
          1 => {
            /* See above for the rationale */
            dowait(0i32, 0 as *mut job);
            if evalfun(cmdentry.u.func, argc, argv, flags) != 0 {
              current_block = 1439023131555407698;
            } else {
              current_block = 9756042043304152679;
            }
          }
          _ => {
            /* Can we avoid forking? For example, very last command
             * in a script or a subshell does not need forking,
             * we can just exec it.
             */
            if flags & 0o1i32 == 0 || (*ash_ptr_to_globals_misc).may_have_traps as libc::c_int != 0
            {
              /* No, forking off a child is necessary */
              ::std::ptr::write_volatile(
                &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
                ::std::ptr::read_volatile::<libc::c_int>(
                  &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
                ) + 1,
              );
              asm!("" : : : "memory" : "volatile");
              jp = makejob(1i32);
              if forkshell(jp, cmd, 0i32) != 0i32 {
                /* fall through to exec'ing external program */
                /* parent */
                status = waitforjob(jp);
                int_on();
                current_block = 18005428789809177185;
              } else {
                /* child */
                force_int_on();
                current_block = 1425453989644512380;
              }
            } else {
              current_block = 1425453989644512380;
            }
            match current_block {
              18005428789809177185 => {}
              _ => {
                listsetvar(varlist.list, 0x1i32 | 0x10i32);
                shellexec(*argv.offset(0), argv, path, cmdentry.u.index);
              }
            }
            /* NOTREACHED */
          }
        }
        match current_block {
          18005428789809177185 => {}
          _ => {
            match current_block {
              1439023131555407698 => {
                longjmp(
                  (*(*ash_ptr_to_globals_misc).exception_handler)
                    .loc
                    .as_mut_ptr(),
                  1i32,
                );
              }
              _ => status = (*ash_ptr_to_globals_misc).exitstatus as libc::c_int,
            }
            current_block = 18005428789809177185;
          }
        }
      }
    }
    _ => {}
  }
  match current_block {
    9509359022306583830 => {
      (*ash_ptr_to_globals_misc).exitstatus = status as uint8_t;
      /* We have a redirection error. */
      if spclbltin > 0i32 {
        raise_exception(1i32);
      }
    }
    _ => {}
  }
  if !(*cmd).ncmd.redirect.is_null() {
    popredir(cmd_is_exec as libc::c_int);
  }
  unwindredir(redir_stop);
  unwindfiles(file_stop);
  unwindlocalvars(localvar_stop);
  if !lastarg.is_null() {
    /* dsl: I think this is intended to be used to support
     * '_' in 'vi' command mode during line editing...
     * However I implemented that within libedit itself.
     */
    setvar0(b"_\x00" as *const u8 as *const libc::c_char, lastarg); /* initialize nextopt */
  }
  return status;
}
unsafe extern "C" fn evalbltin(
  mut cmd: *const builtincmd,
  mut argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
  mut flags: libc::c_int,
) -> libc::c_int {
  let mut savecmdname: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut savehandler: *mut jmploc = 0 as *mut jmploc;
  let mut jmploc: jmploc = jmploc {
    loc: [__jmp_buf_tag {
      __jmpbuf: [0; 8],
      __mask_was_saved: 0,
      __saved_mask: __sigset_t { __val: [0; 16] },
    }; 1],
  };
  let mut status: libc::c_int = 0;
  let mut i: libc::c_int = 0;
  ::std::ptr::write_volatile(&mut savecmdname as *mut *mut libc::c_char, commandname);
  ::std::ptr::write_volatile(
    &mut savehandler as *mut *mut jmploc,
    (*ash_ptr_to_globals_misc).exception_handler,
  );
  i = _setjmp(jmploc.loc.as_mut_ptr());
  if !(i != 0) {
    (*ash_ptr_to_globals_misc).exception_handler = &mut jmploc;
    commandname = *argv.offset(0);
    argptr = argv.offset(1);
    optptr = 0 as *mut libc::c_char;
    if cmd
      == builtintab
        .as_ptr()
        .offset(2)
        .offset((1i32 * 1i32) as isize)
        .offset((1i32 * (1i32 * 1i32)) as isize)
        .offset((1i32 * 1i32) as isize)
        .offset((1i32 * 1i32) as isize)
        .offset(3)
        .offset((1i32 * 1i32) as isize)
        .offset(1)
        .offset((1i32 * 1i32) as isize)
        .offset(0)
    {
      status = evalcmd(argc, argv, flags)
    } else {
      status = Some((*cmd).builtin.expect("non-null function pointer"))
        .expect("non-null function pointer")(argc, argv)
    }
    flush_stdout_stderr();
    status |= ferror_unlocked(stdout);
    (*ash_ptr_to_globals_misc).exitstatus = status as uint8_t
  }
  clearerr(stdout);
  commandname = savecmdname;
  (*ash_ptr_to_globals_misc).exception_handler = savehandler;
  return i;
}
unsafe extern "C" fn goodname(mut p: *const libc::c_char) -> libc::c_int {
  return (*endofname(p).offset(0) as libc::c_int == '\u{0}' as i32) as libc::c_int;
}
/*
 * Search for a command.  This is called before we fork so that the
 * location of the command will be available in the parent as well as
 * the child.  The check for "goodname" is an overly conservative
 * check that the name will not be subject to expansion.
 */
unsafe extern "C" fn prehash(mut n: *mut node) {
  let mut entry: cmdentry = cmdentry {
    cmdtype: 0,
    u: param { index: 0 },
  };
  if (*n).type_0 as libc::c_int == 0i32
    && !(*n).ncmd.args.is_null()
    && goodname((*(*n).ncmd.args).narg.text) != 0
  {
    find_command(
      (*(*n).ncmd.args).narg.text,
      &mut entry,
      0i32,
      (*ash_ptr_to_globals_var).varinit[(1i32 * 2i32 + 1i32) as usize]
        .var_text
        .offset(5),
    );
  };
}
/* Forward declarations for builtintab[] */
/* ============ Builtin commands
 *
 * Builtin commands whose functions are closely tied to evaluation
 * are implemented here.
 */
/*
 * Handle break and continue commands.  Break, continue, and return are
 * all handled by setting the evalskip flag.  The evaluation routines
 * above all check this flag, and if it is set they start skipping
 * commands rather than executing them.  The variable skipcount is
 * the number of loops to break/continue, or the number of function
 * levels to return.  (The latter is always 1.)  It should probably
 * be an error to break out of more loops than exist, but it isn't
 * in the standard shell so we don't make it one here.
 */
unsafe extern "C" fn breakcmd(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut n: libc::c_int = if !(*argv.offset(1)).is_null() {
    number(*argv.offset(1))
  } else {
    1i32
  };
  if n <= 0i32 {
    ash_msg_and_raise_error(
      b"Illegal number: %s\x00" as *const u8 as *const libc::c_char,
      *argv.offset(1),
    );
  }
  if n > loopnest {
    n = loopnest
  }
  if n > 0i32 {
    evalskip = if **argv as libc::c_int == 'c' as i32 {
      (1i32) << 1i32
    } else {
      (1i32) << 0i32
    } as smallint;
    skipcount = n
  }
  return 0i32;
}
static mut checkkwd: smallint = 0;
/*
 * Push a string back onto the input at this current parsefile level.
 * We handle aliases this way.
 */
unsafe extern "C" fn pushstring(mut s: *mut libc::c_char, mut ap: *mut alias) {
  let mut sp: *mut strpush = 0 as *mut strpush;
  let mut len: libc::c_int = 0;
  len = strlen(s) as libc::c_int;
  ::std::ptr::write_volatile(
    &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
    ::std::ptr::read_volatile::<libc::c_int>(
      &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
    ) + 1,
  );
  asm!("" : : : "memory" : "volatile");
  if !(*g_parsefile).strpush.is_null() {
    sp = xzalloc(::std::mem::size_of::<strpush>() as libc::c_ulong) as *mut strpush;
    (*sp).prev = (*g_parsefile).strpush
  } else {
    sp = &mut (*g_parsefile).basestrpush
  }
  (*g_parsefile).strpush = sp;
  (*sp).prev_string = (*g_parsefile).next_to_pgetc;
  (*sp).prev_left_in_line = (*g_parsefile).left_in_line;
  (*sp).unget = (*g_parsefile).unget;
  memcpy(
    (*sp).lastc.as_mut_ptr() as *mut libc::c_void,
    (*g_parsefile).lastc.as_mut_ptr() as *const libc::c_void,
    ::std::mem::size_of::<[libc::c_int; 2]>() as libc::c_ulong,
  );
  (*sp).ap = ap;
  if !ap.is_null() {
    (*ap).flag |= 1i32;
    (*sp).string = s
  }
  (*g_parsefile).next_to_pgetc = s;
  (*g_parsefile).left_in_line = len;
  (*g_parsefile).unget = 0i32;
  int_on();
}
unsafe extern "C" fn popstring() {
  let mut sp: *mut strpush = (*g_parsefile).strpush;
  ::std::ptr::write_volatile(
    &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
    ::std::ptr::read_volatile::<libc::c_int>(
      &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
    ) + 1,
  );
  asm!("" : : : "memory" : "volatile");
  if !(*sp).ap.is_null() {
    if *(*g_parsefile).next_to_pgetc.offset(-1i32 as isize) as libc::c_int == ' ' as i32
      || *(*g_parsefile).next_to_pgetc.offset(-1i32 as isize) as libc::c_int == '\t' as i32
    {
      checkkwd = (checkkwd as libc::c_int | 0x1i32) as smallint
    }
    if (*sp).string != (*(*sp).ap).val {
      free((*sp).string as *mut libc::c_void);
    }
    (*(*sp).ap).flag &= !1i32;
    if (*(*sp).ap).flag & 2i32 != 0 {
      unalias((*(*sp).ap).name);
    }
  }
  (*g_parsefile).next_to_pgetc = (*sp).prev_string;
  (*g_parsefile).left_in_line = (*sp).prev_left_in_line;
  (*g_parsefile).unget = (*sp).unget;
  memcpy(
    (*g_parsefile).lastc.as_mut_ptr() as *mut libc::c_void,
    (*sp).lastc.as_mut_ptr() as *const libc::c_void,
    ::std::mem::size_of::<[libc::c_int; 2]>() as libc::c_ulong,
  );
  (*g_parsefile).strpush = (*sp).prev;
  if sp != &mut (*g_parsefile).basestrpush as *mut strpush {
    free(sp as *mut libc::c_void);
  }
  int_on();
}
unsafe extern "C" fn preadfd() -> libc::c_int {
  let mut nr: libc::c_int = 0;
  let mut buf: *mut libc::c_char = (*g_parsefile).buf;
  (*g_parsefile).next_to_pgetc = buf;
  loop {
    if (*ash_ptr_to_globals_misc).optlist[3] == 0 || (*g_parsefile).pf_fd != 0i32 {
      nr = nonblock_immune_read(
        (*g_parsefile).pf_fd,
        buf as *mut libc::c_void,
        ((if 1i32 != 0 { 1024i32 } else { 1024i32 }) - 1i32) as size_t,
      ) as libc::c_int;
      break;
    } else {
      let mut timeout: libc::c_int = -1i32;
      let mut tmout_var: *const libc::c_char =
        lookupvar(b"TMOUT\x00" as *const u8 as *const libc::c_char);
      if !tmout_var.is_null() {
        timeout = atoi(tmout_var) * 1000i32;
        if timeout <= 0i32 {
          timeout = -1i32
        }
      }
      (*line_input_state).timeout = timeout;
      (*line_input_state).path_lookup = (*ash_ptr_to_globals_var).varinit
        [(1i32 * 2i32 + 1i32) as usize]
        .var_text
        .offset(5);
      reinit_unicode_for_ash();
      nr = read_line_input(
        line_input_state,
        cmdedit_prompt,
        buf,
        if 1i32 != 0 { 1024i32 } else { 1024i32 },
      );
      if nr == 0i32 {
        /* ^C pressed, "convert" to SIGINT */
        write(
          1i32,
          b"^C\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
          2i32 as size_t,
        );
        if !(*ash_ptr_to_globals_misc).trap[2].is_null() {
          *buf.offset(0) = '\n' as i32 as libc::c_char;
          *buf.offset(1) = '\u{0}' as i32 as libc::c_char;
          raise(2i32);
          return 1i32;
        }
        (*ash_ptr_to_globals_misc).exitstatus = (128i32 + 2i32) as uint8_t;
        bb_putchar('\n' as i32);
      } else {
        if nr < 0i32 {
          if *bb_errno == 0i32 {
            /* Ctrl+D pressed */
            nr = 0i32
          } else if *bb_errno == 11i32 && timeout > 0i32 {
            puts(
              b"\x07timed out waiting for input: auto-logout\x00" as *const u8
                as *const libc::c_char,
            );
            exitshell();
          }
        }
        break;
      }
    }
  }
  /* disabled: nonblock_immune_read() handles this problem */
  return nr;
}
unsafe extern "C" fn preadbuffer() -> libc::c_int {
  let mut current_block: u64;
  let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut more: libc::c_int = 0;
  if !(*g_parsefile).strpush.is_null() {
    if (*g_parsefile).left_in_line == -1i32
      && !(*(*g_parsefile).strpush).ap.is_null()
      && *(*g_parsefile).next_to_pgetc.offset(-1i32 as isize) as libc::c_int != ' ' as i32
      && *(*g_parsefile).next_to_pgetc.offset(-1i32 as isize) as libc::c_int != '\t' as i32
    {
      return 257i32;
    }
    popstring();
    return pgetc();
  }
  /* on both branches above g_parsefile->left_in_line < 0.
   * "pgetc" needs refilling.
   */
  /* -90 is our -BIGNUM. Below we use -99 to mark "EOF on read",
   * pungetc() may increment it a few times.
   * Assuming it won't increment it to less than -90.
   */
  if (*g_parsefile).left_in_line < -90i32 || (*g_parsefile).buf.is_null() {
    /* even in failure keep left_in_line and next_to_pgetc
     * in lock step, for correct multi-layer pungetc.
     * left_in_line was decremented before preadbuffer(),
     * must inc next_to_pgetc: */
    (*g_parsefile).next_to_pgetc = (*g_parsefile).next_to_pgetc.offset(1);
    return 256i32;
  }
  more = (*g_parsefile).left_in_buffer;
  if more <= 0i32 {
    flush_stdout_stderr();
    current_block = 6729265098275823777;
  } else {
    current_block = 11584701595673473500;
  }
  's_104: loop {
    match current_block {
      6729265098275823777 => {
        more = preadfd();
        if more <= 0i32 {
          /* don't try reading again */
          (*g_parsefile).left_in_line = -99i32;
          (*g_parsefile).next_to_pgetc = (*g_parsefile).next_to_pgetc.offset(1);
          return 256i32;
        }
        current_block = 11584701595673473500;
      }
      _ => {
        /* Find out where's the end of line.
         * Set g_parsefile->left_in_line
         * and g_parsefile->left_in_buffer acordingly.
         * NUL chars are deleted.
         */
        q = (*g_parsefile).next_to_pgetc;
        loop {
          let mut c: libc::c_char = 0;
          more -= 1;
          c = *q;
          if c as libc::c_int == '\u{0}' as i32 {
            memmove(
              q as *mut libc::c_void,
              q.offset(1) as *const libc::c_void,
              more as libc::c_ulong,
            );
          } else {
            q = q.offset(1);
            if c as libc::c_int == '\n' as i32 {
              (*g_parsefile).left_in_line = (q.wrapping_offset_from((*g_parsefile).next_to_pgetc)
                as libc::c_long
                - 1i32 as libc::c_long) as libc::c_int;
              break 's_104;
            }
          }
          if !(more <= 0i32) {
            continue;
          }
          (*g_parsefile).left_in_line = (q.wrapping_offset_from((*g_parsefile).next_to_pgetc)
            as libc::c_long
            - 1i32 as libc::c_long) as libc::c_int;
          if (*g_parsefile).left_in_line < 0i32 {
            current_block = 6729265098275823777;
            break;
          } else {
            break 's_104;
          }
        }
      }
    }
  }
  (*g_parsefile).left_in_buffer = more;
  if (*ash_ptr_to_globals_misc).optlist[9] != 0 {
    let mut save: libc::c_char = *q;
    *q = '\u{0}' as i32 as libc::c_char;
    out2str((*g_parsefile).next_to_pgetc);
    *q = save
  }
  let fresh68 = (*g_parsefile).next_to_pgetc;
  (*g_parsefile).next_to_pgetc = (*g_parsefile).next_to_pgetc.offset(1);
  return *fresh68 as libc::c_uchar as libc::c_int;
}
unsafe extern "C" fn nlprompt() {
  (*g_parsefile).linno += 1;
  setprompt_if(doprompt, 2i32);
}
unsafe extern "C" fn nlnoprompt() {
  (*g_parsefile).linno += 1;
  needprompt = doprompt;
}
/*
 * Refill the input buffer and return the next input character:
 *
 * 1) If a string was pushed back on the input, pop it;
 * 2) If an EOF was pushed back (g_parsefile->left_in_line < -BIGNUM)
 *    or we are reading from a string so we can't refill the buffer,
 *    return EOF.
 * 3) If there is more stuff in this buffer, use it else call read to fill it.
 * 4) Process input up to the next newline, deleting nul characters.
 */
//#define pgetc_debug(...) bb_error_msg(__VA_ARGS__)
unsafe extern "C" fn pgetc() -> libc::c_int {
  let mut c: libc::c_int = 0;
  if (*g_parsefile).unget != 0 {
    (*g_parsefile).unget -= 1;
    return (*g_parsefile).lastc[(*g_parsefile).unget as usize];
  }
  (*g_parsefile).left_in_line -= 1;
  if (*g_parsefile).left_in_line >= 0i32 {
    let fresh69 = (*g_parsefile).next_to_pgetc;
    (*g_parsefile).next_to_pgetc = (*g_parsefile).next_to_pgetc.offset(1);
    c = *fresh69 as libc::c_uchar as libc::c_int
  } else {
    c = preadbuffer()
  }
  (*g_parsefile).lastc[1] = (*g_parsefile).lastc[0];
  (*g_parsefile).lastc[0] = c;
  return c;
}
unsafe extern "C" fn pgetc_without_PEOA() -> libc::c_int {
  let mut c: libc::c_int = 0;
  loop {
    c = pgetc();
    if !(c == 257i32) {
      break;
    }
  }
  return c;
}
/*
 * Undo a call to pgetc.  Only two characters may be pushed back.
 * PEOF may be pushed back.
 */
unsafe extern "C" fn pungetc() {
  (*g_parsefile).unget += 1;
}
/* This one eats backslash+newline */
unsafe extern "C" fn pgetc_eatbnl() -> libc::c_int {
  let mut c: libc::c_int = 0;
  loop {
    c = pgetc();
    if !(c == '\\' as i32) {
      break;
    }
    if pgetc() != '\n' as i32 {
      pungetc();
      break;
    } else {
      nlprompt();
    }
  }
  return c;
}
unsafe extern "C" fn synstack_push(
  mut stack: *mut *mut synstack_t,
  mut next: *mut synstack_t,
  mut syntax: libc::c_int,
) {
  memset(
    next as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<synstack_t>() as libc::c_ulong,
  );
  (*next).syntax = syntax as smalluint;
  (*next).next = *stack;
  (**stack).prev = next;
  *stack = next;
}
#[inline(always)]
unsafe extern "C" fn synstack_pop(mut stack: *mut *mut synstack_t) {
  *stack = (**stack).next;
}
/*
 * To handle the "." command, a stack of input files is used.  Pushfile
 * adds a new entry to the stack and popfile restores the previous level.
 */
unsafe extern "C" fn pushfile() {
  let mut pf: *mut parsefile = 0 as *mut parsefile;
  pf = xzalloc(::std::mem::size_of::<parsefile>() as libc::c_ulong) as *mut parsefile;
  (*pf).prev = g_parsefile;
  (*pf).pf_fd = -1i32;
  /*pf->strpush = NULL; - ckzalloc did it */
  /*pf->basestrpush.prev = NULL;*/
  /*pf->unget = 0;*/
  g_parsefile = pf;
}
unsafe extern "C" fn popfile() {
  let mut pf: *mut parsefile = g_parsefile;
  if pf == &mut basepf as *mut parsefile {
    return;
  }
  ::std::ptr::write_volatile(
    &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
    ::std::ptr::read_volatile::<libc::c_int>(
      &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
    ) + 1,
  );
  asm!("" : : : "memory" : "volatile");
  if (*pf).pf_fd >= 0i32 {
    close((*pf).pf_fd);
  }
  free((*pf).buf as *mut libc::c_void);
  while !(*pf).strpush.is_null() {
    popstring();
  }
  g_parsefile = (*pf).prev;
  free(pf as *mut libc::c_void);
  int_on();
}
/*
 * Execute a simple command.
 */
unsafe extern "C" fn unwindfiles(mut stop: *mut parsefile) {
  while g_parsefile != stop {
    popfile();
  }
}
/*
 * Return to top level.
 */
unsafe extern "C" fn popallfiles() {
  unwindfiles(&mut basepf);
}
/*
 * Close the file(s) that the shell is reading commands from.  Called
 * after a fork is done.
 */
unsafe extern "C" fn closescript() {
  popallfiles();
  if (*g_parsefile).pf_fd > 0i32 {
    close((*g_parsefile).pf_fd);
    (*g_parsefile).pf_fd = 0i32
  };
}
/*
 * Like setinputfile, but takes an open file descriptor.  Call this with
 * interrupts off.
 */
unsafe extern "C" fn setinputfd(mut fd: libc::c_int, mut push: libc::c_int) {
  if push != 0 {
    pushfile();
    (*g_parsefile).buf = 0 as *mut libc::c_char
  }
  (*g_parsefile).pf_fd = fd;
  if (*g_parsefile).buf.is_null() {
    (*g_parsefile).buf =
      xmalloc(if 1i32 != 0 { 1024i32 } else { 1024i32 } as size_t) as *mut libc::c_char
  }
  (*g_parsefile).left_in_buffer = 0i32;
  (*g_parsefile).left_in_line = 0i32;
  (*g_parsefile).linno = 1i32;
}
/*
 * Set the input to take input from a file.  If push is set, push the
 * old input onto the stack first.
 */
unsafe extern "C" fn setinputfile(
  mut fname: *const libc::c_char,
  mut flags: libc::c_int,
) -> libc::c_int {
  let mut fd: libc::c_int = 0;
  ::std::ptr::write_volatile(
    &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
    ::std::ptr::read_volatile::<libc::c_int>(
      &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
    ) + 1,
  );
  asm!("" : : : "memory" : "volatile");
  fd = open(fname, 0i32 | 0o2000000i32);
  if fd < 0i32 {
    if !(flags & INPUT_NOFILE_OK as libc::c_int != 0) {
      (*ash_ptr_to_globals_misc).exitstatus = 127i32 as uint8_t;
      ash_msg_and_raise_error(
        b"can\'t open \'%s\': %m\x00" as *const u8 as *const libc::c_char,
        fname,
      );
    }
  } else {
    if fd < 10i32 {
      fd = savefd(fd)
    } else if 0o2000000i32 == 0i32 {
      /* old libc */
      close_on_exec_on(fd);
    }
    setinputfd(fd, flags & INPUT_PUSH_FILE as libc::c_int);
  }
  int_on();
  return fd;
}
/*
 * Like setinputfile, but takes input from a string.
 */
unsafe extern "C" fn setinputstring(mut string: *mut libc::c_char) {
  ::std::ptr::write_volatile(
    &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
    ::std::ptr::read_volatile::<libc::c_int>(
      &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
    ) + 1,
  );
  asm!("" : : : "memory" : "volatile");
  pushfile();
  (*g_parsefile).next_to_pgetc = string;
  (*g_parsefile).left_in_line = strlen(string) as libc::c_int;
  (*g_parsefile).buf = 0 as *mut libc::c_char;
  (*g_parsefile).linno = 1i32;
  int_on();
}
/*
 * Routines to check for mail.
 */
/* Hash of mtimes of mailboxes */
static mut mailtime_hash: libc::c_uint = 0;
/* Set if MAIL or MAILPATH is changed. */
static mut mail_var_path_changed: smallint = 0;
/*
 * Print appropriate message(s) if mail has arrived.
 * If mail_var_path_changed is set,
 * then the value of MAIL has mail_var_path_changed,
 * so we just update the values.
 */
unsafe extern "C" fn chkmail() {
  let mut mpath: *const libc::c_char = 0 as *const libc::c_char; /* delete trailing '/' */
  let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut new_hash: libc::c_uint = 0;
  let mut smark: stackmark = stackmark {
    stackp: 0 as *mut stack_block,
    stacknxt: 0 as *mut libc::c_char,
    stacknleft: 0,
  };
  let mut statb: stat = stat {
    st_dev: 0,
    st_ino: 0,
    st_nlink: 0,
    st_mode: 0,
    st_uid: 0,
    st_gid: 0,
    __pad0: 0,
    st_rdev: 0,
    st_size: 0,
    st_blksize: 0,
    st_blocks: 0,
    st_atim: timespec {
      tv_sec: 0,
      tv_nsec: 0,
    },
    st_mtim: timespec {
      tv_sec: 0,
      tv_nsec: 0,
    },
    st_ctim: timespec {
      tv_sec: 0,
      tv_nsec: 0,
    },
    __glibc_reserved: [0; 3],
  };
  setstackmark(&mut smark);
  mpath = if (*ash_ptr_to_globals_var).varinit[2].flags & 0x20i32 == 0i32 {
    (*ash_ptr_to_globals_var).varinit[2].var_text.offset(9)
  } else {
    (*ash_ptr_to_globals_var).varinit[1].var_text.offset(5)
  };
  new_hash = 0i32 as libc::c_uint;
  loop {
    p = path_advance(&mut mpath, (*ash_ptr_to_globals_misc).nullstr.as_mut_ptr());
    if p.is_null() {
      break;
    }
    if *p as libc::c_int == '\u{0}' as i32 {
      continue;
    }
    q = p;
    while *q != 0 {
      q = q.offset(1)
    }
    *q.offset(-1i32 as isize) = '\u{0}' as i32 as libc::c_char;
    if stat(p, &mut statb) < 0i32 {
      continue;
    }
    /* Very simplistic "hash": just a sum of all mtimes */
    new_hash = new_hash.wrapping_add(statb.st_mtim.tv_sec as libc::c_uint)
  }
  if mail_var_path_changed == 0 && mailtime_hash != new_hash {
    if mailtime_hash != 0i32 as libc::c_uint {
      out2str(b"you have mail\n\x00" as *const u8 as *const libc::c_char);
    }
    mailtime_hash = new_hash
  }
  mail_var_path_changed = 0i32 as smallint;
  popstackmark(&mut smark);
}
unsafe extern "C" fn changemail(mut _val: *const libc::c_char) {
  mail_var_path_changed = 1i32 as smallint;
}
/* ASH_MAIL */
/* ============ ??? */
/*
 * Set the shell parameters.
 */
unsafe extern "C" fn setparam(mut argv: *mut *mut libc::c_char) {
  let mut newparam: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  let mut ap: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  let mut nparam: libc::c_int = 0;
  nparam = 0i32;
  while !(*argv.offset(nparam as isize)).is_null() {
    nparam += 1
  }
  newparam = xmalloc(
    ((nparam + 1i32) as libc::c_ulong)
      .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
  ) as *mut *mut libc::c_char;
  ap = newparam;
  while !(*argv).is_null() {
    let fresh70 = argv;
    argv = argv.offset(1);
    let fresh71 = ap;
    ap = ap.offset(1);
    *fresh71 = xstrdup(*fresh70)
  }
  *ap = 0 as *mut libc::c_char;
  freeparam(&mut (*ash_ptr_to_globals_var).shellparam as *mut shparam as *mut shparam);
  (*ash_ptr_to_globals_var).shellparam.malloced = 1i32 as libc::c_uchar;
  (*ash_ptr_to_globals_var).shellparam.nparam = nparam;
  (*ash_ptr_to_globals_var).shellparam.p = newparam;
  (*ash_ptr_to_globals_var).shellparam.optind = 1i32;
  (*ash_ptr_to_globals_var).shellparam.optoff = -1i32;
}
/*
 * Process shell options.  The global variable argptr contains a pointer
 * to the argument list; we advance it past the options.
 *
 * SUSv3 section 2.8.1 "Consequences of Shell Errors" says:
 * For a non-interactive shell, an error condition encountered
 * by a special built-in ... shall cause the shell to write a diagnostic message
 * to standard error and exit as shown in the following table:
 * Error                                           Special Built-In
 * ...
 * Utility syntax error (option or operand error)  Shall exit
 * ...
 * However, in bug 1142 (http://busybox.net/bugs/view.php?id=1142)
 * we see that bash does not do that (set "finishes" with error code 1 instead,
 * and shell continues), and people rely on this behavior!
 * Testcase:
 * set -o barfoo 2>/dev/null
 * echo $?
 *
 * Oh well. Let's mimic that.
 */
unsafe extern "C" fn plus_minus_o(
  mut name: *mut libc::c_char,
  mut val: libc::c_int,
) -> libc::c_int {
  let mut i: libc::c_int = 0;
  if !name.is_null() {
    i = 0i32;
    while i < NOPTS as libc::c_int {
      if strcmp(name, optletters_optnames[i as usize].offset(1)) == 0i32 {
        (*ash_ptr_to_globals_misc).optlist[i as usize] = val as libc::c_char;
        return 0i32;
      }
      i += 1
    }
    ash_msg(
      b"illegal option %co %s\x00" as *const u8 as *const libc::c_char,
      if val != 0 { '-' as i32 } else { '+' as i32 },
      name,
    );
    return 1i32;
  }
  i = 0i32;
  while i < NOPTS as libc::c_int {
    if !(*optletters_optnames[i as usize].offset(1).offset(0) as libc::c_int == '\u{0}' as i32) {
      if val != 0 {
        out1fmt(
          b"%-16s%s\n\x00" as *const u8 as *const libc::c_char,
          optletters_optnames[i as usize].offset(1),
          if (*ash_ptr_to_globals_misc).optlist[i as usize] as libc::c_int != 0 {
            b"on\x00" as *const u8 as *const libc::c_char
          } else {
            b"off\x00" as *const u8 as *const libc::c_char
          },
        );
      } else {
        out1fmt(
          b"set %co %s\n\x00" as *const u8 as *const libc::c_char,
          if (*ash_ptr_to_globals_misc).optlist[i as usize] as libc::c_int != 0 {
            '-' as i32
          } else {
            '+' as i32
          },
          optletters_optnames[i as usize].offset(1),
        );
      }
    }
    i += 1
  }
  return 0i32;
}
unsafe extern "C" fn setoption(mut flag: libc::c_int, mut val: libc::c_int) {
  let mut i: libc::c_int = 0;
  i = 0i32;
  while i < NOPTS as libc::c_int {
    if *optletters_optnames[i as usize].offset(0) as libc::c_int == flag
      && *optletters_optnames[i as usize].offset(1).offset(0) as libc::c_int != '\u{0}' as i32
    {
      (*ash_ptr_to_globals_misc).optlist[i as usize] = val as libc::c_char;
      return;
    }
    i += 1
  }
  ash_msg_and_raise_error(
    b"illegal option %c%c\x00" as *const u8 as *const libc::c_char,
    if val != 0 { '-' as i32 } else { '+' as i32 },
    flag,
  );
  /* NOTREACHED */
}
/* If login_sh is not NULL, we are called to parse command line opts,
 * not "set -opts"
 */
unsafe extern "C" fn options(mut login_sh: *mut libc::c_int) -> libc::c_int {
  let mut p: *mut libc::c_char = 0 as *mut libc::c_char; /* val = 0 if c == '+' */
  let mut val: libc::c_int = 0;
  let mut c: libc::c_int = 0;
  if !login_sh.is_null() {
    (*ash_ptr_to_globals_misc).minusc = 0 as *mut libc::c_char
  }
  loop {
    p = *argptr;
    if p.is_null() {
      break;
    }
    let fresh72 = p;
    p = p.offset(1);
    c = *fresh72 as libc::c_int;
    if c != '-' as i32 && c != '+' as i32 {
      break;
    }
    argptr = argptr.offset(1);
    val = 0i32;
    if c == '-' as i32 {
      val = 1i32;
      if *p.offset(0) as libc::c_int == '\u{0}' as i32
        || *p.offset(0) as libc::c_int == '-' as i32 && *p.offset(1) == 0
      {
        if login_sh.is_null() {
          /* "-" means turn off -x and -v */
          if *p.offset(0) as libc::c_int == '\u{0}' as i32 {
            (*ash_ptr_to_globals_misc).optlist[9] = 0i32 as libc::c_char;
            (*ash_ptr_to_globals_misc).optlist[8] = (*ash_ptr_to_globals_misc).optlist[9]
          } else if (*argptr).is_null() {
            setparam(argptr);
          }
        }
        break;
        /* "--" means reset params */
        /* "-" or "--" terminates options */
      }
    }
    loop
    /* first char was + or - */
    {
      let fresh73 = p;
      p = p.offset(1);
      c = *fresh73 as libc::c_int;
      if !(c != '\u{0}' as i32) {
        break;
      }
      if !login_sh.is_null() {
        /* bash 3.2 indeed handles -c CMD and +c CMD the same */
        if c == 'c' as i32 {
          (*ash_ptr_to_globals_misc).minusc = p; /* command is after shell args */
          (*ash_ptr_to_globals_misc).optlist[7] = 1i32 as libc::c_char;
          continue;
        } else if c == 's' as i32 {
          /* -s, +s */
          (*ash_ptr_to_globals_misc).optlist[6] = 1i32 as libc::c_char;
          continue;
        } else if c == 'i' as i32 {
          /* -i, +i */
          (*ash_ptr_to_globals_misc).optlist[3] = 1i32 as libc::c_char; /* -l or +l == --login */
          continue;
        } else if c == 'l' as i32 {
          *login_sh = 1i32;
          continue;
        } else if val != 0 && c == '-' as i32 {
          /* bash does not accept +-login, we also won't */
          /* long options */
          if strcmp(p, b"login\x00" as *const u8 as *const libc::c_char) == 0i32 {
            *login_sh = 1i32
          }
          break;
        }
      }
      if c == 'o' as i32 {
        if plus_minus_o(*argptr, val) != 0 {
          /* it already printed err message */
          return 1i32;
          /* error */
        }
        if !(*argptr).is_null() {
          argptr = argptr.offset(1)
        }
      } else {
        setoption(c, val);
      }
    }
  }
  return 0i32;
}
/*
 * The shift builtin command.
 */
unsafe extern "C" fn shiftcmd(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut n: libc::c_int = 0;
  let mut ap1: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  let mut ap2: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  n = 1i32;
  if !(*argv.offset(1)).is_null() {
    n = number(*argv.offset(1))
  }
  if n > (*ash_ptr_to_globals_var).shellparam.nparam {
    return 1i32;
  }
  ::std::ptr::write_volatile(
    &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
    ::std::ptr::read_volatile::<libc::c_int>(
      &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
    ) + 1,
  );
  asm!("" : : : "memory" : "volatile");
  (*ash_ptr_to_globals_var).shellparam.nparam -= n;
  ap1 = (*ash_ptr_to_globals_var).shellparam.p;
  loop {
    n -= 1;
    if !(n >= 0i32) {
      break;
    }
    if (*ash_ptr_to_globals_var).shellparam.malloced != 0 {
      free(*ap1 as *mut libc::c_void);
    }
    ap1 = ap1.offset(1)
  }
  ap2 = (*ash_ptr_to_globals_var).shellparam.p;
  loop {
    let fresh74 = ap1;
    ap1 = ap1.offset(1);
    let fresh75 = ap2;
    ap2 = ap2.offset(1);
    *fresh75 = *fresh74;
    if (*fresh75).is_null() {
      break;
    }
  }
  (*ash_ptr_to_globals_var).shellparam.optind = 1i32;
  (*ash_ptr_to_globals_var).shellparam.optoff = -1i32;
  int_on();
  return 0i32;
}
/*
 * POSIX requires that 'set' (but not export or readonly) output the
 * variables in lexicographic order - by the locale's collating order (sigh).
 * Maybe we could keep them in an ordered balanced binary tree
 * instead of hashed lists.
 * For now just roll 'em through qsort for printing...
 */
unsafe extern "C" fn showvars(
  mut sep_prefix: *const libc::c_char,
  mut on: libc::c_int,
  mut off: libc::c_int,
) -> libc::c_int {
  let mut sep: *const libc::c_char = 0 as *const libc::c_char;
  let mut ep: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  let mut epend: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  ep = listvars(on, off, &mut epend);
  qsort(
    ep as *mut libc::c_void,
    epend.wrapping_offset_from(ep) as libc::c_long as size_t,
    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
    Some(
      vpcmp as unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> libc::c_int,
    ),
  );
  sep = if *sep_prefix as libc::c_int != 0 {
    b" \x00" as *const u8 as *const libc::c_char
  } else {
    sep_prefix
  };
  while ep < epend {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut q: *const libc::c_char = 0 as *const libc::c_char;
    p = endofname(*ep);
    /* Used to have simple "p = strchrnul(*ep, '=')" here instead, but this
     * makes "export -p" to have output not suitable for "eval":
     * import os
     * os.environ["test-test"]="test"
     * if os.fork() == 0:
     *   os.execv("ash", [ 'ash', '-c', 'eval $(export -p); echo OK' ])  # fixes this
     * os.execv("ash", [ 'ash', '-c', 'env | grep test-test' ])
     */
    q = (*ash_ptr_to_globals_misc).nullstr.as_mut_ptr();
    if *p as libc::c_int == '=' as i32 {
      p = p.offset(1);
      q = single_quote(p)
    }
    out1fmt(
      b"%s%s%.*s%s\n\x00" as *const u8 as *const libc::c_char,
      sep_prefix,
      sep,
      p.wrapping_offset_from(*ep) as libc::c_long as libc::c_int,
      *ep,
      q,
    );
    ep = ep.offset(1)
  }
  return 0i32;
}
/*
 * The set command builtin.
 */
unsafe extern "C" fn setcmd(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut retval: libc::c_int = 0;
  if (*argv.offset(1)).is_null() {
    return showvars(
      (*ash_ptr_to_globals_misc).nullstr.as_mut_ptr(),
      0i32,
      0x20i32,
    );
  }
  ::std::ptr::write_volatile(
    &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
    ::std::ptr::read_volatile::<libc::c_int>(
      &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
    ) + 1,
  );
  asm!("" : : : "memory" : "volatile");
  retval = options(0 as *mut libc::c_int);
  if retval == 0i32 {
    /* if no parse error... */
    optschanged();
    if !(*argptr).is_null() {
      setparam(argptr);
    }
  }
  int_on();
  return retval;
}
unsafe extern "C" fn change_random(mut value: *const libc::c_char) {
  let mut t: uint32_t = 0;
  if value.is_null() {
    /* "get", generate */
    t = next_random(&mut (*ash_ptr_to_globals_misc).random_gen);
    /* set without recursion */
    setvar(
      (*ash_ptr_to_globals_var).varinit[(1i32 * 2i32 + 1i32 + 6i32) as usize].var_text,
      utoa(t),
      0x40i32,
    );
    (*ash_ptr_to_globals_var).varinit[(1i32 * 2i32 + 1i32 + 6i32) as usize].flags &= !0x40i32
  } else {
    /* set/reset */
    t = strtoul(value, 0 as *mut *mut libc::c_char, 10i32) as uint32_t;
    (*ash_ptr_to_globals_misc).random_gen.xs64_x = if t != 0 { t } else { 1i32 as libc::c_uint };
    (*ash_ptr_to_globals_misc).random_gen.galois_LFSR =
      (*ash_ptr_to_globals_misc).random_gen.xs64_x as int32_t;
    (*ash_ptr_to_globals_misc).random_gen.xs64_y = t;
    (*ash_ptr_to_globals_misc).random_gen.LCG = (*ash_ptr_to_globals_misc).random_gen.xs64_y
  };
}
unsafe extern "C" fn change_epoch(mut vepoch: *mut var, mut fmt: *const libc::c_char) {
  let mut tv: timeval = timeval {
    tv_sec: 0,
    tv_usec: 0,
  };
  let mut buffer: [libc::c_char; 35] = [0; 35];
  gettimeofday(&mut tv, 0 as *mut timezone);
  sprintf(
    buffer.as_mut_ptr(),
    fmt,
    tv.tv_sec as libc::c_ulong,
    tv.tv_usec as libc::c_uint,
  );
  setvar((*vepoch).var_text, buffer.as_mut_ptr(), 0x40i32);
  (*vepoch).flags &= !0x40i32;
}
unsafe extern "C" fn change_seconds(mut _value: *const libc::c_char) {
  change_epoch(
    &mut *(*ash_ptr_to_globals_var)
      .varinit
      .as_mut_ptr()
      .offset((1i32 * 2i32 + 1i32 + 1i32 + 6i32) as isize),
    b"%lu\x00" as *const u8 as *const libc::c_char,
  );
}
unsafe extern "C" fn change_realtime(mut _value: *const libc::c_char) {
  change_epoch(
    &mut *(*ash_ptr_to_globals_var)
      .varinit
      .as_mut_ptr()
      .offset((1i32 * 2i32 + 1i32 + 1i32 + 7i32) as isize),
    b"%lu.%06u\x00" as *const u8 as *const libc::c_char,
  );
}
unsafe extern "C" fn getopts(
  mut optstr: *mut libc::c_char,
  mut optvar: *mut libc::c_char,
  mut optfirst: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut current_block: u64;
  let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut c: libc::c_char = '?' as i32 as libc::c_char;
  let mut done: libc::c_int = 0i32;
  let mut sbuf: [libc::c_char; 2] = [0; 2];
  let mut optnext: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  let mut ind: libc::c_int = (*ash_ptr_to_globals_var).shellparam.optind;
  let mut off: libc::c_int = (*ash_ptr_to_globals_var).shellparam.optoff;
  sbuf[1] = '\u{0}' as i32 as libc::c_char;
  (*ash_ptr_to_globals_var).shellparam.optind = -1i32;
  optnext = optfirst.offset(ind as isize).offset(-1);
  if ind <= 1i32 || off < 0i32 || (strlen(*optnext.offset(-1i32 as isize)) as libc::c_int) < off {
    p = 0 as *mut libc::c_char
  } else {
    p = (*optnext.offset(-1i32 as isize)).offset(off as isize)
  }
  if p.is_null() || *p as libc::c_int == '\u{0}' as i32 {
    /* Current word is done, advance */
    p = *optnext;
    if p.is_null() || *p as libc::c_int != '-' as i32 || {
      p = p.offset(1);
      (*p as libc::c_int) == '\u{0}' as i32
    } {
      current_block = 15234592599140099628;
    } else {
      optnext = optnext.offset(1);
      if *p.offset(0) as libc::c_int == '-' as i32 && *p.offset(1) == 0 {
        current_block = 15234592599140099628;
      } else {
        current_block = 6009453772311597924;
      }
    }
    match current_block {
      6009453772311597924 => {}
      _ =>
      /* check for "--" */
      {
        unsetvar(b"OPTARG\x00" as *const u8 as *const libc::c_char);
        p = 0 as *mut libc::c_char;
        done = 1i32;
        current_block = 11120208946601160771;
      }
    }
  } else {
    current_block = 6009453772311597924;
  }
  match current_block {
    6009453772311597924 => {
      let fresh76 = p;
      p = p.offset(1);
      c = *fresh76;
      q = optstr;
      loop {
        if !(*q as libc::c_int != c as libc::c_int) {
          current_block = 1538046216550696469;
          break;
        }
        if *q as libc::c_int == '\u{0}' as i32 {
          /* OPTERR is a bashism */
          let mut cp: *const libc::c_char =
            lookupvar(b"OPTERR\x00" as *const u8 as *const libc::c_char);
          if !cp.is_null() && (*cp.offset(0) as libc::c_int == '0' as i32 && *cp.offset(1) == 0)
            || *optstr.offset(0) as libc::c_int == ':' as i32
          {
            sbuf[0] = c;
            /*sbuf[1] = '\0'; - already is */
            setvar0(
              b"OPTARG\x00" as *const u8 as *const libc::c_char,
              sbuf.as_mut_ptr(),
            );
          } else {
            fprintf(
              stderr,
              b"Illegal option -%c\n\x00" as *const u8 as *const libc::c_char,
              c as libc::c_int,
            );
            unsetvar(b"OPTARG\x00" as *const u8 as *const libc::c_char);
          }
          c = '?' as i32 as libc::c_char;
          current_block = 11120208946601160771;
          break;
        } else {
          q = q.offset(1);
          if *q as libc::c_int == ':' as i32 {
            q = q.offset(1)
          }
        }
      }
      match current_block {
        11120208946601160771 => {}
        _ => {
          q = q.offset(1);
          if *q as libc::c_int == ':' as i32 {
            if *p as libc::c_int == '\u{0}' as i32 && {
              p = *optnext;
              p.is_null()
            } {
              /* OPTERR is a bashism */
              let mut cp_0: *const libc::c_char =
                lookupvar(b"OPTERR\x00" as *const u8 as *const libc::c_char);
              if !cp_0.is_null()
                && (*cp_0.offset(0) as libc::c_int == '0' as i32 && *cp_0.offset(1) == 0)
                || *optstr.offset(0) as libc::c_int == ':' as i32
              {
                sbuf[0] = c;
                /*sbuf[1] = '\0'; - already is */
                setvar0(
                  b"OPTARG\x00" as *const u8 as *const libc::c_char,
                  sbuf.as_mut_ptr(),
                );
                c = ':' as i32 as libc::c_char
              } else {
                fprintf(
                  stderr,
                  b"No arg for -%c option\n\x00" as *const u8 as *const libc::c_char,
                  c as libc::c_int,
                );
                unsetvar(b"OPTARG\x00" as *const u8 as *const libc::c_char);
                c = '?' as i32 as libc::c_char
              }
            } else {
              if p == *optnext {
                optnext = optnext.offset(1)
              }
              setvar0(b"OPTARG\x00" as *const u8 as *const libc::c_char, p);
              p = 0 as *mut libc::c_char
            }
          } else {
            setvar0(
              b"OPTARG\x00" as *const u8 as *const libc::c_char,
              (*ash_ptr_to_globals_misc).nullstr.as_mut_ptr(),
            );
          }
        }
      }
    }
    _ => {}
  }
  ind =
    (optnext.wrapping_offset_from(optfirst) as libc::c_long + 1i32 as libc::c_long) as libc::c_int;
  setvar(
    b"OPTIND\x00" as *const u8 as *const libc::c_char,
    itoa(ind),
    0x40i32,
  );
  sbuf[0] = c;
  /*sbuf[1] = '\0'; - already is */
  setvar0(optvar, sbuf.as_mut_ptr());
  (*ash_ptr_to_globals_var).shellparam.optoff = if !p.is_null() {
    p.wrapping_offset_from(*optnext.offset(-1)) as libc::c_long
  } else {
    -1i32 as libc::c_long
  } as libc::c_int;
  (*ash_ptr_to_globals_var).shellparam.optind = ind;
  return done;
}
/*
 * The getopts builtin.  Shellparam.optnext points to the next argument
 * to be processed.  Shellparam.optptr points to the next character to
 * be processed in the current argument.  If shellparam.optnext is NULL,
 * then it's the first time getopts has been called.
 */
unsafe extern "C" fn getoptscmd(
  mut argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut optbase: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  if argc < 3i32 {
    ash_msg_and_raise_error(
      b"usage: getopts optstring var [arg]\x00" as *const u8 as *const libc::c_char,
    );
  }
  if argc == 3i32 {
    optbase = (*ash_ptr_to_globals_var).shellparam.p;
    if (*ash_ptr_to_globals_var).shellparam.optind as libc::c_uint
      > ((*ash_ptr_to_globals_var).shellparam.nparam + 1i32) as libc::c_uint
    {
      (*ash_ptr_to_globals_var).shellparam.optind = 1i32;
      (*ash_ptr_to_globals_var).shellparam.optoff = -1i32
    }
  } else {
    optbase = &mut *argv.offset(3) as *mut *mut libc::c_char;
    if (*ash_ptr_to_globals_var).shellparam.optind as libc::c_uint > (argc - 2i32) as libc::c_uint {
      (*ash_ptr_to_globals_var).shellparam.optind = 1i32;
      (*ash_ptr_to_globals_var).shellparam.optoff = -1i32
    }
  }
  return getopts(*argv.offset(1), *argv.offset(2), optbase);
}
static mut tokpushback: smallint = 0;
/* last token pushed back */
static mut quoteflag: smallint = 0;
/* set if (part of) last token was quoted */
static mut lasttoken: token_id_t = 0;
/* last token read (integer id Txxx) */
static mut heredoclist: *mut heredoc = 0 as *const heredoc as *mut heredoc;
/* list of here documents to read */
static mut wordtext: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
/* text of last word returned by readtoken */
static mut backquotelist: *mut nodelist = 0 as *const nodelist as *mut nodelist;
static mut redirnode: *mut node = 0 as *const node as *mut node;
static mut heredoc: *mut heredoc = 0 as *const heredoc as *mut heredoc;
unsafe extern "C" fn tokname(
  mut buf: *mut libc::c_char,
  mut tok: libc::c_int,
) -> *const libc::c_char {
  if tok < TSEMI as libc::c_int {
    return tokname_array[tok as usize];
  }
  sprintf(
    buf,
    b"\"%s\"\x00" as *const u8 as *const libc::c_char,
    tokname_array[tok as usize],
  );
  return buf;
}
/* raise_error_unexpected_syntax:
 * Called when an unexpected token is read during the parse.  The argument
 * is the token that is expected, or -1 if more than one type of token can
 * occur at this point.
 */
unsafe extern "C" fn raise_error_unexpected_syntax(mut token: libc::c_int) -> ! {
  let mut msg: [libc::c_char; 64] = [0; 64];
  let mut buf: [libc::c_char; 16] = [0; 16];
  let mut l: libc::c_int = 0;
  l = sprintf(
    msg.as_mut_ptr(),
    b"unexpected %s\x00" as *const u8 as *const libc::c_char,
    tokname(buf.as_mut_ptr(), lasttoken as libc::c_int),
  );
  if token >= 0i32 {
    sprintf(
      msg.as_mut_ptr().offset(l as isize),
      b" (expecting %s)\x00" as *const u8 as *const libc::c_char,
      tokname(buf.as_mut_ptr(), token),
    );
  }
  raise_error_syntax(msg.as_mut_ptr());
  /* NOTREACHED */
}
unsafe extern "C" fn list(mut nlflag: libc::c_int) -> *mut node {
  let mut n1: *mut node = 0 as *mut node;
  let mut n2: *mut node = 0 as *mut node;
  let mut n3: *mut node = 0 as *mut node;
  let mut tok: libc::c_int = 0;
  n1 = 0 as *mut node;
  loop {
    match peektoken() {
      1 => {
        if !(nlflag & 1i32 == 0) {
          parseheredoc();
          return n1;
        }
      }
      0 => {
        if n1.is_null() && nlflag & 1i32 != 0 {
          n1 = -1i64 as *mut node
        }
        parseheredoc();
        return n1;
      }
      _ => {}
    }
    checkkwd = (0x4i32 | 0x2i32 | 0x1i32) as smallint;
    if nlflag == 2i32 && 1i32 << peektoken() & tokendlist as libc::c_int != 0 {
      return n1;
    }
    nlflag |= 2i32;
    n2 = andor();
    tok = readtoken();
    if tok == TBACKGND as libc::c_int {
      if (*n2).type_0 as libc::c_int == 1i32 {
        (*n2).npipe.pipe_backgnd = 1i32 as smallint
      } else {
        if (*n2).type_0 as libc::c_int != 2i32 {
          n3 = stzalloc(::std::mem::size_of::<nredir>() as libc::c_ulong) as *mut node;
          (*n3).nredir.n = n2;
          /*n3->nredir.redirect = NULL; - stzalloc did it */
          n2 = n3
        }
        (*n2).type_0 = 3i32 as smallint
      }
    }
    if n1.is_null() {
      n1 = n2
    } else {
      n3 = stzalloc(::std::mem::size_of::<nbinary>() as libc::c_ulong) as *mut node;
      (*n3).type_0 = 7i32 as smallint;
      (*n3).nbinary.ch1 = n1;
      (*n3).nbinary.ch2 = n2;
      n1 = n3
    }
    match tok {
      1 | 0 => tokpushback = 1i32 as smallint,
      5 | 4 => {}
      _ => {
        if nlflag & 1i32 != 0 {
          raise_error_unexpected_syntax(-1i32);
        }
        tokpushback = 1i32 as smallint;
        return n1;
      }
    }
  }
}
/* parsing is heavily cross-recursive, need these forward decls */
unsafe extern "C" fn andor() -> *mut node {
  let mut n1: *mut node = 0 as *mut node;
  let mut n2: *mut node = 0 as *mut node;
  let mut n3: *mut node = 0 as *mut node;
  let mut t: libc::c_int = 0;
  n1 = pipeline();
  loop {
    t = readtoken();
    if t == TAND as libc::c_int {
      t = 5i32
    } else if t == TOR as libc::c_int {
      t = 6i32
    } else {
      tokpushback = 1i32 as smallint;
      return n1;
    }
    checkkwd = (0x4i32 | 0x2i32 | 0x1i32) as smallint;
    n2 = pipeline();
    n3 = stzalloc(::std::mem::size_of::<nbinary>() as libc::c_ulong) as *mut node;
    (*n3).type_0 = t as smallint;
    (*n3).nbinary.ch1 = n1;
    (*n3).nbinary.ch2 = n2;
    n1 = n3
  }
}
unsafe extern "C" fn pipeline() -> *mut node {
  let mut n1: *mut node = 0 as *mut node;
  let mut n2: *mut node = 0 as *mut node;
  let mut pipenode: *mut node = 0 as *mut node;
  let mut lp: *mut nodelist = 0 as *mut nodelist;
  let mut prev: *mut nodelist = 0 as *mut nodelist;
  let mut negate: libc::c_int = 0;
  negate = 0i32;
  if readtoken() == TNOT as libc::c_int {
    negate = (negate == 0) as libc::c_int;
    checkkwd = (0x2i32 | 0x1i32) as smallint
  } else {
    tokpushback = 1i32 as smallint
  }
  n1 = parse_command();
  if readtoken() == TPIPE as libc::c_int {
    pipenode = stzalloc(::std::mem::size_of::<npipe>() as libc::c_ulong) as *mut node;
    (*pipenode).type_0 = 1i32 as smallint;
    /*pipenode->npipe.pipe_backgnd = 0; - stzalloc did it */
    lp = stzalloc(::std::mem::size_of::<nodelist>() as libc::c_ulong) as *mut nodelist;
    (*pipenode).npipe.cmdlist = lp;
    (*lp).n = n1;
    loop {
      prev = lp;
      lp = stzalloc(::std::mem::size_of::<nodelist>() as libc::c_ulong) as *mut nodelist;
      checkkwd = (0x4i32 | 0x2i32 | 0x1i32) as smallint;
      (*lp).n = parse_command();
      (*prev).next = lp;
      if !(readtoken() == TPIPE as libc::c_int) {
        break;
      }
    }
    (*lp).next = 0 as *mut nodelist;
    n1 = pipenode
  }
  tokpushback = 1i32 as smallint;
  if negate != 0 {
    n2 = stzalloc(::std::mem::size_of::<nnot>() as libc::c_ulong) as *mut node;
    (*n2).type_0 = 26i32 as smallint;
    (*n2).nnot.com = n1;
    return n2;
  }
  return n1;
}
unsafe extern "C" fn makename() -> *mut node {
  let mut n: *mut node = 0 as *mut node;
  n = stzalloc(::std::mem::size_of::<narg>() as libc::c_ulong) as *mut node;
  (*n).type_0 = 15i32 as smallint;
  /*n->narg.next = NULL; - stzalloc did it */
  (*n).narg.text = wordtext;
  (*n).narg.backquote = backquotelist;
  return n;
}
/*
 * Compute the names of the files in a redirection list.
 */
unsafe extern "C" fn fixredir(
  mut n: *mut node,
  mut text: *const libc::c_char,
  mut err: libc::c_int,
) {
  let mut fd: libc::c_int = 0;
  if err == 0 {
    (*n).ndup.vname = 0 as *mut node
  }
  fd = bb_strtou(text, 0 as *mut *mut libc::c_char, 10i32) as libc::c_int;
  if *bb_errno == 0 && fd >= 0i32 {
    (*n).ndup.dupfd = fd
  } else if *text.offset(0) as libc::c_int == '-' as i32 && *text.offset(1) == 0 {
    (*n).ndup.dupfd = -1i32
  } else {
    if err != 0 {
      raise_error_syntax(b"bad fd number\x00" as *const u8 as *const libc::c_char);
    }
    (*n).ndup.vname = makename()
  };
}
unsafe extern "C" fn parsefname() {
  let mut n: *mut node = redirnode;
  if (*n).type_0 as libc::c_int == 24i32 {
    checkkwd = 0x8i32 as smallint
  }
  if readtoken() != TWORD as libc::c_int {
    raise_error_unexpected_syntax(-1i32);
  }
  if (*n).type_0 as libc::c_int == 24i32 {
    let mut here: *mut heredoc = heredoc;
    let mut p: *mut heredoc = 0 as *mut heredoc;
    if quoteflag as libc::c_int == 0i32 {
      (*n).type_0 = 25i32 as smallint
    }
    rmescapes(wordtext, 0i32, 0 as *mut libc::c_int);
    (*here).eofmark = wordtext;
    (*here).next = 0 as *mut heredoc;
    if heredoclist.is_null() {
      heredoclist = here
    } else {
      p = heredoclist;
      while !(*p).next.is_null() {
        p = (*p).next
      }
      (*p).next = here
    }
  } else if (*n).type_0 as libc::c_int == 22i32 || (*n).type_0 as libc::c_int == 23i32 {
    fixredir(n, wordtext, 0i32);
  } else {
    (*n).nfile.fname = makename()
  };
}
unsafe extern "C" fn simplecmd() -> *mut node {
  let mut current_block: u64;
  let mut args: *mut node = 0 as *mut node;
  let mut app: *mut *mut node = 0 as *mut *mut node;
  let mut n: *mut node = 0 as *mut node;
  let mut vars: *mut node = 0 as *mut node;
  let mut vpp: *mut *mut node = 0 as *mut *mut node;
  let mut rpp: *mut *mut node = 0 as *mut *mut node;
  let mut redir: *mut node = 0 as *mut node;
  let mut savecheckkwd: libc::c_int = 0;
  let mut savelinno: libc::c_int = 0;
  let mut double_brackets_flag: smallint = 0i32 as smallint;
  let mut function_flag: smallint = 0i32 as smallint;
  args = 0 as *mut node;
  app = &mut args;
  vars = 0 as *mut node;
  vpp = &mut vars;
  redir = 0 as *mut node;
  rpp = &mut redir;
  savecheckkwd = 0x1i32;
  savelinno = (*g_parsefile).linno;
  loop {
    let mut t: libc::c_int = 0;
    checkkwd = savecheckkwd as smallint;
    t = readtoken();
    match t {
      22 => {
        if peektoken() != TWORD as libc::c_int {
          raise_error_unexpected_syntax(TWORD as libc::c_int);
        }
        function_flag = 1i32 as smallint;
        continue;
      }
      6 => {
        /* "&&" */
        current_block = 7535051484845167340; /* read name of redirection file */
      }
      7 => {
        current_block = 7535051484845167340;
      }
      3 => {
        current_block = 18230082464074504637;
      }
      2 => {
        n = redirnode;
        *rpp = n;
        rpp = &mut (*n).nfile.next;
        parsefname();
        continue;
      }
      9 => {
        current_block = 5423024947874578000;
      }
      _ => {
        current_block = 17245639976027627875;
      }
    }
    match current_block {
      7535051484845167340 =>
      /* "||" */
      {
        if double_brackets_flag == 0 {
          tokpushback = 1i32 as smallint;
          break;
        } else {
          wordtext = if t == TAND as libc::c_int {
            b"-a\x00" as *const u8 as *const libc::c_char
          } else {
            b"-o\x00" as *const u8 as *const libc::c_char
          } as *mut libc::c_char
        }
        current_block = 18230082464074504637;
      }
      _ => {}
    }
    match current_block {
      18230082464074504637 => {
        n = stzalloc(::std::mem::size_of::<narg>() as libc::c_ulong) as *mut node;
        (*n).type_0 = 15i32 as smallint;
        /*n->narg.next = NULL; - stzalloc did it */
        (*n).narg.text = wordtext;
        if strcmp(b"[[\x00" as *const u8 as *const libc::c_char, wordtext) == 0i32 {
          double_brackets_flag = 1i32 as smallint
        } else if strcmp(b"]]\x00" as *const u8 as *const libc::c_char, wordtext) == 0i32 {
          double_brackets_flag = 0i32 as smallint
        }
        (*n).narg.backquote = backquotelist;
        if savecheckkwd != 0 && isassignment(wordtext) != 0 {
          *vpp = n;
          vpp = &mut (*n).narg.next
        } else {
          *app = n;
          app = &mut (*n).narg.next;
          savecheckkwd = 0i32
        }
        if !(function_flag != 0) {
          continue;
        }
        checkkwd = (0x4i32 | 0x2i32) as smallint;
        match peektoken() {
          28 | 23 | 14 | 26 | 27 | 21 => {
            current_block = 5423024947874578000;
          }
          9 => {
            current_block = 1548662447412563631;
            match current_block {
              2006681514697956935 => {
                if strcmp(b"[[\x00" as *const u8 as *const libc::c_char, wordtext) == 0i32 {
                  current_block = 5423024947874578000;
                } else {
                  current_block = 7120670781030353840;
                }
              }
              1548662447412563631 => {
                function_flag = 0i32 as smallint;
                continue;
              }
              _ => {}
            }
            match current_block {
              5423024947874578000 => {}
              _ =>
              /* fall through */
              {
                raise_error_unexpected_syntax(-1i32);
              }
            }
          }
          3 => {
            current_block = 2006681514697956935;
            match current_block {
              2006681514697956935 => {
                if strcmp(b"[[\x00" as *const u8 as *const libc::c_char, wordtext) == 0i32 {
                  current_block = 5423024947874578000;
                } else {
                  current_block = 7120670781030353840;
                }
              }
              1548662447412563631 => {
                function_flag = 0i32 as smallint;
                continue;
              }
              _ => {}
            }
            match current_block {
              5423024947874578000 => {}
              _ => {
                raise_error_unexpected_syntax(-1i32);
              }
            }
          }
          _ => {
            current_block = 7120670781030353840;
            match current_block {
              2006681514697956935 => {
                if strcmp(b"[[\x00" as *const u8 as *const libc::c_char, wordtext) == 0i32 {
                  current_block = 5423024947874578000;
                } else {
                  current_block = 7120670781030353840;
                }
              }
              1548662447412563631 => {
                function_flag = 0i32 as smallint;
                continue;
              }
              _ => {}
            }
            match current_block {
              5423024947874578000 => {}
              _ => {
                raise_error_unexpected_syntax(-1i32);
              }
            }
          }
        }
      }
      _ => {}
    }
    match current_block {
      5423024947874578000 => {
        if !args.is_null()
          && app == &mut (*args).narg.next as *mut *mut node
          && vars.is_null()
          && redir.is_null()
        {
          let mut bcmd: *mut builtincmd = 0 as *mut builtincmd;
          let mut name: *const libc::c_char = 0 as *const libc::c_char;
          /* We have a function */
          if function_flag == 0 && readtoken() != TRP as libc::c_int {
            raise_error_unexpected_syntax(TRP as libc::c_int);
          }
          name = (*n).narg.text;
          if goodname(name) == 0 || {
            bcmd = find_builtin(name);
            (!bcmd.is_null()) && *(*bcmd).name.offset(0) as libc::c_int & 1i32 != 0
          } {
            raise_error_syntax(b"bad function name\x00" as *const u8 as *const libc::c_char);
          }
          (*n).type_0 = 14i32 as smallint;
          checkkwd = (0x4i32 | 0x2i32 | 0x1i32) as smallint;
          (*n).ndefun.text = (*n).narg.text;
          (*n).ndefun.linno = (*g_parsefile).linno;
          (*n).ndefun.body = parse_command();
          return n;
        }
        function_flag = 0i32 as smallint
      }
      _ => {}
    }
    /* fall through */
    tokpushback = 1i32 as smallint;
    break;
  }
  *app = 0 as *mut node;
  *vpp = 0 as *mut node;
  *rpp = 0 as *mut node;
  n = stzalloc(::std::mem::size_of::<ncmd>() as libc::c_ulong) as *mut node;
  if 0i32 != 0i32 {
    (*n).type_0 = 0i32 as smallint
  }
  (*n).ncmd.linno = savelinno;
  (*n).ncmd.args = args;
  (*n).ncmd.assign = vars;
  (*n).ncmd.redirect = redir;
  return n;
}
unsafe extern "C" fn parse_command() -> *mut node {
  let mut current_block: u64;
  let mut n1: *mut node = 0 as *mut node;
  let mut n2: *mut node = 0 as *mut node;
  let mut ap: *mut node = 0 as *mut node;
  let mut app: *mut *mut node = 0 as *mut *mut node;
  let mut cp: *mut node = 0 as *mut node;
  let mut cpp: *mut *mut node = 0 as *mut *mut node;
  let mut redir: *mut node = 0 as *mut node;
  let mut rpp: *mut *mut node = 0 as *mut *mut node;
  let mut rpp2: *mut *mut node = 0 as *mut *mut node;
  let mut t: libc::c_int = 0;
  let mut savelinno: libc::c_int = 0;
  redir = 0 as *mut node;
  rpp2 = &mut redir;
  savelinno = (*g_parsefile).linno;
  match readtoken() {
    23 => {
      /* NOTREACHED */
      n1 = stzalloc(::std::mem::size_of::<nif>() as libc::c_ulong) as *mut node;
      (*n1).type_0 = 8i32 as smallint;
      (*n1).nif.test = list(0i32);
      if readtoken() != TTHEN as libc::c_int {
        raise_error_unexpected_syntax(TTHEN as libc::c_int);
      }
      (*n1).nif.ifpart = list(0i32);
      n2 = n1;
      while readtoken() == TELIF as libc::c_int {
        (*n2).nif.elsepart = stzalloc(::std::mem::size_of::<nif>() as libc::c_ulong) as *mut node;
        n2 = (*n2).nif.elsepart;
        (*n2).type_0 = 8i32 as smallint;
        (*n2).nif.test = list(0i32);
        if readtoken() != TTHEN as libc::c_int {
          raise_error_unexpected_syntax(TTHEN as libc::c_int);
        }
        (*n2).nif.ifpart = list(0i32)
      }
      if lasttoken as libc::c_int == TELSE as libc::c_int {
        (*n2).nif.elsepart = list(0i32)
      } else {
        (*n2).nif.elsepart = 0 as *mut node;
        tokpushback = 1i32 as smallint
      }
      t = TFI as libc::c_int;
      current_block = 12079920068676227593;
    }
    27 | 26 => {
      let mut got: libc::c_int = 0;
      n1 = stzalloc(::std::mem::size_of::<nbinary>() as libc::c_ulong) as *mut node;
      (*n1).type_0 = if lasttoken as libc::c_int == TWHILE as libc::c_int {
        9i32
      } else {
        10i32
      } as smallint;
      (*n1).nbinary.ch1 = list(0i32);
      got = readtoken();
      if got != TDO as libc::c_int {
        raise_error_unexpected_syntax(TDO as libc::c_int);
      }
      (*n1).nbinary.ch2 = list(0i32);
      t = TDONE as libc::c_int;
      current_block = 12079920068676227593;
    }
    21 => {
      if readtoken() != TWORD as libc::c_int
        || quoteflag as libc::c_int != 0
        || goodname(wordtext) == 0
      {
        raise_error_syntax(b"bad for loop variable\x00" as *const u8 as *const libc::c_char);
      }
      n1 = stzalloc(::std::mem::size_of::<nfor>() as libc::c_ulong) as *mut node;
      (*n1).type_0 = 11i32 as smallint;
      (*n1).nfor.linno = savelinno;
      (*n1).nfor.var = wordtext;
      checkkwd = (0x4i32 | 0x2i32 | 0x1i32) as smallint;
      if readtoken() == TIN as libc::c_int {
        app = &mut ap;
        while readtoken() == TWORD as libc::c_int {
          n2 = stzalloc(::std::mem::size_of::<narg>() as libc::c_ulong) as *mut node;
          (*n2).type_0 = 15i32 as smallint;
          /*n2->narg.next = NULL; - stzalloc did it */
          (*n2).narg.text = wordtext;
          (*n2).narg.backquote = backquotelist;
          *app = n2;
          app = &mut (*n2).narg.next
        }
        *app = 0 as *mut node;
        (*n1).nfor.args = ap;
        if lasttoken as libc::c_int != TNL as libc::c_int
          && lasttoken as libc::c_int != TSEMI as libc::c_int
        {
          raise_error_unexpected_syntax(-1i32);
        }
      } else {
        n2 = stzalloc(::std::mem::size_of::<narg>() as libc::c_ulong) as *mut node;
        (*n2).type_0 = 15i32 as smallint;
        /*n2->narg.next = NULL; - stzalloc did it */
        (*n2).narg.text = dolatstr.as_ptr() as *mut libc::c_char;
        /*n2->narg.backquote = NULL;*/
        (*n1).nfor.args = n2;
        /*
         * Newline or semicolon here is optional (but note
         * that the original Bourne shell only allowed NL).
         */
        if lasttoken as libc::c_int != TSEMI as libc::c_int {
          tokpushback = 1i32 as smallint
        }
      }
      checkkwd = (0x4i32 | 0x2i32 | 0x1i32) as smallint;
      if readtoken() != TDO as libc::c_int {
        raise_error_unexpected_syntax(TDO as libc::c_int);
      }
      (*n1).nfor.body = list(0i32);
      t = TDONE as libc::c_int;
      current_block = 12079920068676227593;
    }
    14 => {
      n1 = stzalloc(::std::mem::size_of::<ncase>() as libc::c_ulong) as *mut node;
      (*n1).type_0 = 12i32 as smallint;
      (*n1).ncase.linno = savelinno;
      if readtoken() != TWORD as libc::c_int {
        raise_error_unexpected_syntax(TWORD as libc::c_int);
      }
      n2 = stzalloc(::std::mem::size_of::<narg>() as libc::c_ulong) as *mut node;
      (*n1).ncase.expr = n2;
      (*n2).type_0 = 15i32 as smallint;
      /*n2->narg.next = NULL; - stzalloc did it */
      (*n2).narg.text = wordtext;
      (*n2).narg.backquote = backquotelist;
      checkkwd = (0x4i32 | 0x2i32 | 0x1i32) as smallint;
      if readtoken() != TIN as libc::c_int {
        raise_error_unexpected_syntax(TIN as libc::c_int);
      }
      cpp = &mut (*n1).ncase.cases;
      'c_35068: loop {
        checkkwd = (0x4i32 | 0x2i32) as smallint;
        t = readtoken();
        loop {
          if !(t != TESAC as libc::c_int) {
            break 'c_35068;
          }
          if lasttoken as libc::c_int == TLP as libc::c_int {
            readtoken();
          }
          cp = stzalloc(::std::mem::size_of::<nclist>() as libc::c_ulong) as *mut node;
          *cpp = cp;
          (*cp).type_0 = 13i32 as smallint;
          app = &mut (*cp).nclist.pattern;
          loop {
            ap = stzalloc(::std::mem::size_of::<narg>() as libc::c_ulong) as *mut node;
            *app = ap;
            (*ap).type_0 = 15i32 as smallint;
            /*ap->narg.next = NULL; - stzalloc did it */
            (*ap).narg.text = wordtext;
            (*ap).narg.backquote = backquotelist;
            if readtoken() != TPIPE as libc::c_int {
              break;
            }
            app = &mut (*ap).narg.next;
            readtoken();
          }
          //ap->narg.next = NULL;
          if lasttoken as libc::c_int != TRP as libc::c_int {
            raise_error_unexpected_syntax(TRP as libc::c_int);
          }
          (*cp).nclist.body = list(2i32);
          cpp = &mut (*cp).nclist.next;
          checkkwd = (0x4i32 | 0x2i32) as smallint;
          t = readtoken();
          if !(t != TESAC as libc::c_int) {
            continue;
          }
          if t != TENDCASE as libc::c_int {
            raise_error_unexpected_syntax(TENDCASE as libc::c_int);
          }
          break;
        }
      }
      *cpp = 0 as *mut node;
      current_block = 9438404386301180972;
    }
    9 => {
      n1 = stzalloc(::std::mem::size_of::<nredir>() as libc::c_ulong) as *mut node;
      (*n1).type_0 = 4i32 as smallint;
      (*n1).nredir.linno = savelinno;
      (*n1).nredir.n = list(0i32);
      /*n1->nredir.redirect = NULL; - stzalloc did it */
      t = TRP as libc::c_int;
      current_block = 12079920068676227593;
    }
    28 => {
      n1 = list(0i32);
      t = TEND as libc::c_int;
      current_block = 12079920068676227593;
    }
    22 | 3 | 2 => {
      tokpushback = 1i32 as smallint;
      return simplecmd();
    }
    _ => {
      raise_error_unexpected_syntax(-1i32);
    }
  }
  match current_block {
    12079920068676227593 => {
      if readtoken() != t {
        raise_error_unexpected_syntax(t);
      }
    }
    _ => {}
  }
  /* Now check for redirection which may follow command */
  checkkwd = (0x2i32 | 0x1i32) as smallint; /* unrecognized "\z": print both chars unless ' or " */

  rpp = rpp2;
  while readtoken() == TREDIR as libc::c_int {
    n2 = redirnode;
    *rpp = n2;
    rpp = &mut (*n2).nfile.next;
    parsefname();
  }
  tokpushback = 1i32 as smallint;
  *rpp = 0 as *mut node;
  if !redir.is_null() {
    if (*n1).type_0 as libc::c_int != 4i32 {
      n2 = stzalloc(::std::mem::size_of::<nredir>() as libc::c_ulong) as *mut node;
      (*n2).type_0 = 2i32 as smallint;
      (*n2).nredir.linno = savelinno;
      (*n2).nredir.n = n1;
      n1 = n2
    }
    (*n1).nredir.redirect = redir
  }
  return n1;
}
unsafe extern "C" fn decode_dollar_squote() -> libc::c_int {
  static mut C_escapes: [libc::c_char; 18] = [
    110, 114, 98, 116, 102, 97, 118, 120, 92, 48, 49, 50, 51, 52, 53, 54, 55, 0,
  ];
  let mut c: libc::c_int = 0;
  let mut cnt: libc::c_int = 0;
  let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut buf: [libc::c_char; 4] = [0; 4];
  c = pgetc();
  p = strchr(C_escapes.as_ptr(), c);
  let mut current_block_22: u64;
  if !p.is_null() {
    buf[0] = c as libc::c_char;
    p = buf.as_mut_ptr();
    cnt = 3i32;
    if (c - '0' as i32) as libc::c_uchar as libc::c_int <= 7i32 {
      loop
      /* \ooo */
      {
        c = pgetc(); /* simple seq like \\ or \t */
        p = p.offset(1);
        *p = c as libc::c_char;
        if !((c - '0' as i32) as libc::c_uchar as libc::c_int <= 7i32 && {
          cnt -= 1;
          (cnt) != 0
        }) {
          break;
        }
      }
      pungetc();
      current_block_22 = 10043043949733653460;
    } else if c == 'x' as i32 {
      loop
      /* \xHH */
      {
        c = pgetc();
        p = p.offset(1);
        *p = c as libc::c_char;
        if !(bb_ascii_isxdigit(c as libc::c_uchar) != 0 && {
          cnt -= 1;
          (cnt) != 0
        }) {
          break;
        }
      }
      pungetc();
      if cnt == 3i32 {
        /* \x but next char is "bad" */
        c = 'x' as i32;
        current_block_22 = 14813801982881665196;
      } else {
        current_block_22 = 10043043949733653460;
      }
    } else {
      p = p.offset(1);
      current_block_22 = 10043043949733653460;
    }
    match current_block_22 {
      14813801982881665196 => {}
      _ => {
        *p = '\u{0}' as i32 as libc::c_char;
        p = buf.as_mut_ptr();
        c = bb_process_escape_sequence(
          &mut p as *mut *mut libc::c_char as *mut libc::c_void as *mut *const libc::c_char,
        ) as libc::c_int;
        current_block_22 = 8693738493027456495;
      }
    }
  } else if c != '\'' as i32 && c != '\"' as i32 {
    current_block_22 = 14813801982881665196;
  } else {
    current_block_22 = 8693738493027456495;
  }
  match current_block_22 {
    14813801982881665196 => {
      c |= 0x100i32
      /* "please encode \, then me" */
    }
    _ => {}
  }
  return c;
}
#[inline(always)]
unsafe extern "C" fn realeofmark(mut eofmark: *const libc::c_char) -> libc::c_int {
  return (!eofmark.is_null() && eofmark != 1i32 as uintptr_t as *mut libc::c_char) as libc::c_int;
}
/*
 * If eofmark is NULL, read a word or a redirection symbol.  If eofmark
 * is not NULL, read a here document.  In the latter case, eofmark is the
 * word which marks the end of the document and striptabs is true if
 * leading tabs should be stripped from the document.  The argument c
 * is the first character of the input token or document.
 *
 * Because C does not have internal subroutines, I have simulated them
 * using goto's to implement the subroutine linkage.  The following macros
 * will run code that appears at the end of readtoken1.
 */
unsafe extern "C" fn readtoken1(
  mut c: libc::c_int,
  mut syntax: libc::c_int,
  mut eofmark: *mut libc::c_char,
  mut striptabs: libc::c_int,
) -> libc::c_int {
  let mut cc: libc::c_int = 0;
  let mut current_block: u64;
  /* NB: syntax parameter fits into smallint */
  /* c parameter is an unsigned char or PEOF or PEOA */
  let mut out: *mut libc::c_char = 0 as *mut libc::c_char; /* we are expanding a prompt string */
  let mut len: size_t = 0;
  let mut bqlist: *mut nodelist = 0 as *mut nodelist;
  let mut quotef: smallint = 0;
  let mut oldstyle: smallint = 0;
  let mut pssyntax: smallint = 0;
  let mut bash_dollar_squote: smallint = 0i32 as smallint;
  /* syntax stack */
  let mut synbase: synstack_t = {
    let mut init = synstack_t {
      innerdq_varpushed_dblquote: [0; 1],
      c2rust_padding: [0; 2],
      syntax: 0,
      varnest: 0,
      dqvarnest: 0,
      parenlevel: 0,
      prev: 0 as *mut synstack_t,
      next: 0 as *mut synstack_t,
    };
    init.set_innerdq(0);
    init.set_varpushed(0);
    init.set_dblquote(0);
    init
  };
  let mut synstack: *mut synstack_t = &mut synbase;
  pssyntax = (syntax == 4i32) as libc::c_int as smallint;
  if pssyntax != 0 {
    syntax = 1i32
  }
  (*synstack).syntax = syntax as smalluint;
  if syntax == 1i32 {
    (*synstack).set_dblquote(1i32 as uint8_t)
  }
  quotef = 0i32 as smallint;
  bqlist = 0 as *mut nodelist;
  out = (*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut libc::c_char;
  'c_29317: loop
  /* For each line, until end of word */
  /* end of readtoken routine */
  /*
   * Check to see whether we are at the end of the here document.  When this
   * is called, c is set to the first character of the next input line.  If
   * we are at the end of the here document, this routine sets the c to PEOF.
   */
  {
    if realeofmark(eofmark) != 0 {
      let mut len_here: libc::c_int = 0;
      let mut current_block_147: u64;
      let mut markloc: libc::c_int = 0;
      let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
      if c == 257i32 {
        c = pgetc_without_PEOA()
      }
      if striptabs != 0 {
        while c == '\t' as i32 {
          c = pgetc_without_PEOA()
        }
      }
      markloc = out.wrapping_offset_from(
        (*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut libc::c_char,
      ) as libc::c_long as libc::c_int;
      p = eofmark;
      loop {
        out = _STPUTC(c, out);
        if !(*p != 0) {
          current_block_147 = 16580259026179177070;
          break;
        }
        if c != *p as libc::c_int {
          current_block_147 = 15239049467836318834;
          break;
        }
        /* FIXME: fails for backslash-newlined terminator:
         * cat <<EOF
         * ...
         * EO\
         * F
         * (see heredoc_bkslash_newline2.tests)
         */
        c = pgetc_without_PEOA();
        p = p.offset(1)
      }
      match current_block_147 {
        16580259026179177070 => {
          if c == '\n' as i32 || c == 256i32 {
            c = 256i32;
            (*g_parsefile).linno += 1;
            needprompt = doprompt;
            current_block_147 = 857031028540284188;
          } else {
            len_here = 0;
            current_block_147 = 15239049467836318834;
          }
        }
        _ => {}
      }
      match current_block_147 {
        15239049467836318834 => {
          p = ((*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut libc::c_char)
            .offset(markloc as isize)
            .offset(1);
          len_here = out.wrapping_offset_from(p) as libc::c_long as libc::c_int;
          if len_here != 0 {
            len_here -= (c >= 256i32) as libc::c_int;
            c = *p.offset(-1i32 as isize) as libc::c_int;
            if len_here != 0 {
              let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
              let mut fresh99 =
                ::std::vec::from_elem(0, (len_here + 1i32) as libc::c_ulong as usize);
              str = fresh99.as_mut_ptr() as *mut libc::c_char;
              *(mempcpy(
                str as *mut libc::c_void,
                p as *const libc::c_void,
                len_here as size_t,
              ) as *mut libc::c_char) = '\u{0}' as i32 as libc::c_char;
              pushstring(str, 0 as *mut alias);
            }
          }
        }
        _ => {}
      }
      out = out.offset(
        ((*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut libc::c_char)
          .offset(markloc as isize)
          .wrapping_offset_from(out) as libc::c_long as isize,
      )
    }
    loop
    /* set c to PEOF if at end of here document */
    /* until end of line or end of word */
    {
      let mut q: *mut libc::c_char = out; /* permit 4 calls to USTPUTC */
      let mut l: size_t = 4i32 as size_t;
      let mut m: size_t = (*ash_ptr_to_globals_memstack)
        .sstrend
        .wrapping_offset_from(q) as libc::c_long as size_t;
      if l > m {
        out = makestrspace(l, q)
      }
      match SIT(c, (*synstack).syntax as libc::c_int) {
        1 => {
          /* '\n' */
          if (*synstack).syntax as libc::c_int == 0i32 && (*synstack).varnest == 0 {
            current_block = 15768484401365413375;
            break;
          } else {
            current_block = 7056779235015430508;
            break;
          }
        }
        0 => {
          let fresh78 = out;
          out = out.offset(1);
          *fresh78 = c as libc::c_char;
          current_block = 5916212523694105379;
        }
        12 => {
          if c == '\\' as i32 && bash_dollar_squote as libc::c_int != 0 {
            c = decode_dollar_squote();
            if c == '\u{0}' as i32 {
              /* skip $'\000', $'\x00' (like bash) */
              current_block = 5916212523694105379;
            } else {
              if c & 0x100i32 != 0 {
                /* Unknown escape. Encode as '\z' */
                c = c as libc::c_uchar as libc::c_int;
                if eofmark.is_null() || (*synstack).dblquote() as libc::c_int != 0 {
                  let fresh79 = out;
                  out = out.offset(1);
                  *fresh79 = '\u{81}' as i32 as libc::c_uchar as libc::c_char
                }
                let fresh80 = out;
                out = out.offset(1);
                *fresh80 = '\\' as i32 as libc::c_char
              }
              current_block = 10891380440665537214;
            }
          } else {
            current_block = 10891380440665537214;
          }
          match current_block {
            5916212523694105379 => {}
            _ => {
              if eofmark.is_null()
                || (*synstack).dblquote() as libc::c_int != 0
                || (*synstack).varnest != 0
              {
                let fresh81 = out;
                out = out.offset(1);
                *fresh81 = '\u{81}' as i32 as libc::c_uchar as libc::c_char
              }
              let fresh82 = out;
              out = out.offset(1);
              *fresh82 = c as libc::c_char;
              current_block = 5916212523694105379;
            }
          }
        }
        2 => {
          /* backslash */
          c = pgetc_without_PEOA();
          if c == 256i32 {
            let fresh83 = out;
            out = out.offset(1);
            *fresh83 = '\u{81}' as i32 as libc::c_uchar as libc::c_char;
            let fresh84 = out;
            out = out.offset(1);
            *fresh84 = '\\' as i32 as libc::c_char;
            pungetc();
          } else if c == '\n' as i32 {
            nlprompt();
          } else {
            if pssyntax as libc::c_int != 0 && c == '$' as i32 {
              let fresh85 = out;
              out = out.offset(1);
              *fresh85 = '\u{81}' as i32 as libc::c_uchar as libc::c_char;
              let fresh86 = out;
              out = out.offset(1);
              *fresh86 = '\\' as i32 as libc::c_char
            }
            /* Backslash is retained if we are in "str"
             * and next char isn't dquote-special.
             */
            if (*synstack).dblquote() as libc::c_int != 0
              && c != '\\' as i32
              && c != '`' as i32
              && c != '$' as i32
              && (c != '\"' as i32 || !eofmark.is_null() && (*synstack).varnest == 0)
              && (c != '}' as i32 || (*synstack).varnest == 0)
            {
              let fresh87 = out; /* protect '\' from glob */
              out = out.offset(1);
              *fresh87 = '\u{81}' as i32 as libc::c_uchar as libc::c_char;
              let fresh88 = out;
              out = out.offset(1);
              *fresh88 = '\\' as i32 as libc::c_char
            }
            let fresh89 = out;
            out = out.offset(1);
            *fresh89 = '\u{81}' as i32 as libc::c_uchar as libc::c_char;
            let fresh90 = out;
            out = out.offset(1);
            *fresh90 = c as libc::c_char;
            quotef = 1i32 as smallint
          }
          current_block = 5916212523694105379;
        }
        3 => {
          (*synstack).syntax = 2i32 as smalluint;
          current_block = 5634654558795828693;
        }
        4 => {
          (*synstack).syntax = 1i32 as smalluint;
          (*synstack).set_dblquote(1i32 as uint8_t);
          current_block = 17595510130556652878;
        }
        5 => {
          bash_dollar_squote = 0i32 as smallint;
          if !eofmark.is_null() && (*synstack).varnest == 0i32 {
            let fresh92 = out;
            out = out.offset(1);
            *fresh92 = c as libc::c_char;
            current_block = 5916212523694105379;
          } else {
            if (*synstack).dqvarnest == 0i32 {
              (*synstack).syntax = 0i32 as smalluint;
              (*synstack).set_dblquote(0i32 as uint8_t)
            }
            quotef = 1i32 as smallint;
            if c == '\"' as i32 {
              current_block = 17595510130556652878;
            } else {
              current_block = 5634654558795828693;
            }
          }
        }
        7 => {
          /* '$' */
          /*
           * Parse a substitution.  At this point, we have read the dollar sign
           * and nothing else.
           */
          /* is_special(c) evaluates to 1 for c in "!#$*-0123456789?@"; 0 otherwise
           * (assuming ascii char codes, as the original implementation did) */
          let mut subtype: libc::c_uchar = 0;
          let mut typeloc: libc::c_int = 0;
          c = pgetc_eatbnl();
          if checkkwd as libc::c_int & 0x8i32 != 0
            || c > 255i32
            || c != '(' as i32
              && c != '{' as i32
              && !(c == '_' as i32
                || ((c as libc::c_uchar as libc::c_int | 0x20i32) - 'a' as i32) as libc::c_uchar
                  as libc::c_int
                  <= 'z' as i32 - 'a' as i32)
              && !((c as libc::c_uint).wrapping_sub(33i32 as libc::c_uint) < 32i32 as libc::c_uint
                && 0xc1ff920du32 >> (c as libc::c_uint).wrapping_sub(33i32 as libc::c_uint)
                  & 1i32 as libc::c_uint
                  != 0)
          {
            if (*synstack).syntax as libc::c_int != 1i32 && c == '\'' as i32 {
              bash_dollar_squote = 1i32 as smallint
            } else {
              let fresh100 = out;
              out = out.offset(1);
              *fresh100 = '$' as i32 as libc::c_char
            }
            pungetc();
            current_block = 5916212523694105379;
          } else if c == '(' as i32 {
            /* $(command) or $((arith)) */
            if pgetc_eatbnl() == '(' as i32 {
              /*
               * Parse an arithmetic expansion (indicate start of one and set state)
               */
              let mut poopstack = synstack_t {
                innerdq_varpushed_dblquote: [0; 1],
                c2rust_padding: [0; 2],
                syntax: 0,
                varnest: 0,
                dqvarnest: 0,
                parenlevel: 0,
                prev: 0 as *mut synstack_t,
                next: 0 as *mut synstack_t,
              };
              poopstack.set_innerdq(0);
              poopstack.set_varpushed(0);
              poopstack.set_dblquote(0);
              synstack_push(
                &mut synstack,
                if !(*synstack).prev.is_null() {
                  (*synstack).prev
                } else {
                  // This was an alloca(sizeof(*synstack)) in the original C code. Hopefully this is an acceptable translation.
                  &mut poopstack
                } as *mut synstack_t,
                3i32,
              );
              (*synstack).set_dblquote(1i32 as uint8_t);
              let fresh107 = out;
              out = out.offset(1);
              *fresh107 = '\u{86}' as i32 as libc::c_uchar as libc::c_char;
              current_block = 5916212523694105379;
            } else {
              pungetc();
              oldstyle = 0i32 as smallint;
              current_block = 6356445669134572965;
            }
          } else {
            /* $VAR, $<specialchar>, ${...}, or PEOA/PEOF */
            let mut newsyn: smalluint = (*synstack).syntax;
            let fresh101 = out;
            out = out.offset(1);
            *fresh101 = '\u{82}' as i32 as libc::c_uchar as libc::c_char;
            typeloc = out.wrapping_offset_from(
              (*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut libc::c_char,
            ) as libc::c_long as libc::c_int;
            out = out.offset(1);
            subtype = 0x1i32 as libc::c_uchar;
            if c == '{' as i32 {
              c = pgetc_eatbnl();
              subtype = 0i32 as libc::c_uchar
            }
            loop {
              if c == '_' as i32
                || ((c as libc::c_uchar as libc::c_int | 0x20i32) - 'a' as i32) as libc::c_uchar
                  as libc::c_int
                  <= 'z' as i32 - 'a' as i32
              {
                loop
                /* $[{[#]]NAME[}] */
                {
                  out = _STPUTC(c, out);
                  c = pgetc_eatbnl();
                  if !(c == '_' as i32 || bb_ascii_isalnum(c as libc::c_uchar) != 0) {
                    break;
                  }
                }
                current_block = 168643628589418436;
                break;
              } else if (c - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32 {
                loop
                /* $[{[#]]NUM[}] */
                {
                  out = _STPUTC(c, out);
                  c = pgetc_eatbnl();
                  if !((c - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32) {
                    break;
                  }
                }
                current_block = 168643628589418436;
                break;
              } else {
                if !(c != '}' as i32) {
                  current_block = 18328076825346612136;
                  break;
                }
                /* $[{[#]]<specialchar>[}] */
                cc = c;
                c = pgetc_eatbnl();
                if !(subtype == 0 && cc == '#' as i32) {
                  current_block = 6955753519599442332;
                  break;
                }
                subtype = 0xai32 as libc::c_uchar;
                if c == '_' as i32 || bb_ascii_isalnum(c as libc::c_uchar) != 0 {
                  continue;
                }
                cc = c;
                c = pgetc_eatbnl();
                if cc == '}' as i32 || c != '}' as i32 {
                  pungetc();
                  subtype = 0i32 as libc::c_uchar;
                  c = cc;
                  cc = '#' as i32
                }
                current_block = 6955753519599442332;
                break;
              }
            }
            match current_block {
              6955753519599442332 => {
                if !((cc as libc::c_uint).wrapping_sub(33i32 as libc::c_uint)
                  < 32i32 as libc::c_uint
                  && 0xc1ff920du32 >> (cc as libc::c_uint).wrapping_sub(33i32 as libc::c_uint)
                    & 1i32 as libc::c_uint
                    != 0)
                {
                  if subtype as libc::c_int == 0xai32 {
                    subtype = 0i32 as libc::c_uchar
                  }
                  current_block = 18328076825346612136;
                } else {
                  let fresh102 = out;
                  out = out.offset(1);
                  *fresh102 = cc as libc::c_char;
                  current_block = 168643628589418436;
                }
              }
              _ => {}
            }
            match current_block {
              168643628589418436 => {
                if c != '}' as i32 && subtype as libc::c_int == 0xai32 {
                  current_block = 18328076825346612136;
                } else if subtype as libc::c_int == 0i32 {
                  static mut types: [libc::c_char; 6] = [125, 45, 43, 63, 61, 0];
                  /* ${VAR...} but not $VAR or ${#VAR} */
                  /* c == first char after VAR */
                  let mut cc_0: libc::c_int = c;
                  match c {
                    58 => {
                      c = pgetc_eatbnl();
                      /* This check is only needed to not misinterpret
                       * ${VAR:-WORD}, ${VAR:+WORD}, ${VAR:=WORD}, ${VAR:?WORD}
                       * constructs.
                       */
                      if strchr(types.as_ptr(), c).is_null() {
                        subtype = 0xci32 as libc::c_uchar;
                        pungetc();
                        current_block = 15462824429697920828;
                      /* "goto badsub" is bigger (!) */
                      } else {
                        subtype = 0x10i32 as libc::c_uchar;
                        current_block = 9200713032295880585;
                      }
                    }
                    37 | 35 => {
                      subtype = if c == '#' as i32 { 0x8i32 } else { 0x6i32 } as libc::c_uchar;
                      c = pgetc_eatbnl();
                      if c == cc_0 {
                        subtype = subtype.wrapping_add(1)
                      } else {
                        pungetc();
                      }
                      newsyn = 0i32 as smalluint;
                      current_block = 15462824429697920828;
                    }
                    47 => {
                      /* ${v/[/]pattern/repl} */
                      //TODO: encode pattern and repl separately.
                      // Currently cases like: v=1;echo ${v/$((1/1))/ONE}
                      // are broken (should print "ONE")
                      subtype = 0xdi32 as libc::c_uchar; /* VSREPLACEALL */
                      newsyn = 0i32 as smalluint;
                      c = pgetc_eatbnl();
                      if c != '/' as i32 {
                        current_block = 18328076825346612136;
                      } else {
                        subtype = subtype.wrapping_add(1);
                        current_block = 15462824429697920828;
                      }
                    }
                    _ => {
                      current_block = 9200713032295880585;
                    }
                  }
                  match current_block {
                    15462824429697920828 => {}
                    18328076825346612136 => {}
                    _ =>
                    /*FALLTHROUGH*/
                    {
                      let mut p_0: *const libc::c_char = strchr(types.as_ptr(), c);
                      if p_0.is_null() {
                        current_block = 15462824429697920828;
                      } else {
                        subtype = (subtype as libc::c_long
                          | p_0.wrapping_offset_from(types.as_ptr()) as libc::c_long
                            + 0x1i32 as libc::c_long)
                          as libc::c_uchar;
                        current_block = 15462824429697920828;
                      }
                    }
                  }
                } else {
                  current_block = 18328076825346612136;
                }
              }
              _ => {}
            }
            match current_block {
              18328076825346612136 =>
              /* ${#VAR didn't end with } */
              {
                pungetc();
              }
              _ => {}
            }
            if newsyn as libc::c_int == 3i32 {
              newsyn = 1i32 as smalluint
            }
            if (newsyn as libc::c_int != (*synstack).syntax as libc::c_int
              || (*synstack).innerdq() as libc::c_int != 0)
              && subtype as libc::c_int != 0x1i32
            {
              // This was an alloca(sizeof(*synstack)) in the original C code. Hopefully this is an acceptable translation.
              let mut poopstack: synstack_t = {
                let mut init = synstack_t {
                  innerdq_varpushed_dblquote: [0; 1],
                  c2rust_padding: [0; 2],
                  syntax: 0,
                  varnest: 0,
                  dqvarnest: 0,
                  parenlevel: 0,
                  prev: 0 as *mut synstack_t,
                  next: 0 as *mut synstack_t,
                };
                init.set_innerdq(0);
                init.set_varpushed(0);
                init.set_dblquote(0);
                init
              };
              synstack_push(
                &mut synstack,
                if !(*synstack).prev.is_null() {
                  (*synstack).prev
                } else {
                  &mut poopstack
                } as *mut synstack_t,
                newsyn as libc::c_int,
              );
              (*synstack).set_varpushed(1i32 as uint8_t);
              (*synstack).set_dblquote((newsyn as libc::c_int != 0i32) as libc::c_int as uint8_t)
            }
            *((*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void
              as *mut libc::c_uchar)
              .offset(typeloc as isize) = subtype;
            if subtype as libc::c_int != 0x1i32 {
              (*synstack).varnest += 1;
              if (*synstack).dblquote() != 0 {
                (*synstack).dqvarnest += 1
              }
            }
            out = _STPUTC('=' as i32, out);
            current_block = 5916212523694105379;
          }
        }
        8 => {
          /* '}' */
          if (*synstack).innerdq() == 0 && (*synstack).varnest > 0i32 {
            (*synstack).varnest -= 1;
            if (*synstack).varnest == 0 && (*synstack).varpushed() as libc::c_int != 0 {
              synstack_pop(&mut synstack);
            } else if (*synstack).dqvarnest > 0i32 {
              (*synstack).dqvarnest -= 1
            }
            c = '\u{83}' as i32 as libc::c_uchar as libc::c_int
          }
          let fresh93 = out;
          out = out.offset(1);
          *fresh93 = c as libc::c_char;
          current_block = 5916212523694105379;
        }
        9 => {
          /* '(' in arithmetic */
          (*synstack).parenlevel += 1;
          let fresh94 = out;
          out = out.offset(1);
          *fresh94 = c as libc::c_char;
          current_block = 5916212523694105379;
        }
        10 => {
          /* ')' in arithmetic */
          if (*synstack).parenlevel > 0i32 {
            (*synstack).parenlevel -= 1
          } else if pgetc_eatbnl() == ')' as i32 {
            c = '\u{87}' as i32 as libc::c_uchar as libc::c_int;
            synstack_pop(&mut synstack);
          } else {
            /*
             * unbalanced parens
             * (don't 2nd guess - no error)
             */
            pungetc();
          }
          let fresh95 = out;
          out = out.offset(1);
          *fresh95 = c as libc::c_char;
          current_block = 5916212523694105379;
        }
        6 => {
          /* '`' */
          if checkkwd as libc::c_int & 0x8i32 != 0 {
            quotef = 1i32 as smallint;
            let fresh96 = out;
            out = out.offset(1);
            *fresh96 = '`' as i32 as libc::c_char;
            current_block = 5916212523694105379;
          } else {
            oldstyle = 1i32 as smallint;
            current_block = 6356445669134572965;
          }
        }
        11 => {
          break 'c_29317;
        }
        14 => {
          current_block = 5916212523694105379;
        }
        _ => {
          if (*synstack).varnest == 0i32 {
            if c == '&' as i32 {
              //Can't call pgetc_eatbnl() here, this requires three-deep pungetc()
              if pgetc() == '>' as i32 {
                c = 0x100i32 + '>' as i32
              } /* flag &> */
              pungetc();
            }
            break 'c_29317;
          /* exit outer loop */
          } else if c != 257i32 {
            let fresh97 = out;
            out = out.offset(1);
            *fresh97 = c as libc::c_char
          }
          current_block = 5916212523694105379;
        }
      }
      match current_block {
        6356445669134572965 =>
        /*
         * Called to parse command substitutions.  Newstyle is set if the command
         * is enclosed inside $(...); nlpp is a pointer to the head of the linked
         * list of commands (passed by reference), and savelen is the number of
         * characters on the top of the stack which must be preserved.
         */
        {
          let mut nlpp: *mut *mut nodelist = 0 as *mut *mut nodelist;
          let mut n: *mut node = 0 as *mut node;
          let mut str_0: *mut libc::c_char = 0 as *mut libc::c_char;
          let mut savelen: size_t = 0;
          let mut saveprompt: smallint = 0i32 as smallint;
          str_0 = 0 as *mut libc::c_char;
          savelen = out.wrapping_offset_from(
            (*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut libc::c_char,
          ) as libc::c_long as size_t;
          if savelen > 0i32 as libc::c_ulong {
            /*
             * FIXME: this can allocate very large block on stack and SEGV.
             * Example:
             * echo "..<100kbytes>..`true` $(true) `true` ..."
             * allocates 100kb for every command subst. With about
             * a hundred command substitutions stack overflows.
             * With larger prepended string, SEGV happens sooner.
             */
            let mut fresh104 = ::std::vec::from_elem(0, savelen as usize);
            str_0 = fresh104.as_mut_ptr() as *mut libc::c_char;
            memcpy(
              str_0 as *mut libc::c_void,
              (*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void,
              savelen,
            );
          }
          if oldstyle != 0 {
            let mut current_block_285: u64;
            /* We must read until the closing backquote, giving special
             * treatment to some slashes, and then push the string and
             * reread it as input, interpreting it normally.
             */
            let mut pout: *mut libc::c_char = 0 as *mut libc::c_char; /* or pgetc_eatbnl()? why (example)? */
            let mut psavelen: size_t = 0;
            let mut pstr: *mut libc::c_char = 0 as *mut libc::c_char;
            pout =
              (*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut libc::c_char;
            loop {
              let mut pc: libc::c_int = 0;
              setprompt_if(needprompt, 2i32);
              pc = pgetc_eatbnl();
              match pc {
                96 => {
                  break;
                }
                92 => {
                  pc = pgetc();
                  if pc != '\\' as i32
                    && pc != '`' as i32
                    && pc != '$' as i32
                    && ((*synstack).dblquote() == 0 || pc != '\"' as i32)
                  {
                    pout = _STPUTC('\\' as i32, pout)
                  }
                  if pc <= 255i32 {
                    current_block_285 = 7235894238071384538;
                  } else {
                    current_block_285 = 16907855743034645509;
                  }
                }
                256 | 257 => {
                  current_block_285 = 16907855743034645509;
                }
                10 => {
                  nlnoprompt();
                  current_block_285 = 7235894238071384538;
                }
                _ => {
                  current_block_285 = 7235894238071384538;
                }
              }
              match current_block_285 {
                7235894238071384538 =>
                /* not PEOA or PEOF */
                {
                  pout = _STPUTC(pc, pout)
                }
                _ =>
                /* fall through */
                {
                  raise_error_syntax(
                    b"EOF in backquote substitution\x00" as *const u8 as *const libc::c_char,
                  );
                }
              }
            }
            pout = _STPUTC('\u{0}' as i32, pout);
            psavelen = pout.wrapping_offset_from(
              (*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut libc::c_char,
            ) as libc::c_long as size_t;
            if psavelen > 0i32 as libc::c_ulong {
              pstr = stalloc(pout.wrapping_offset_from(
                (*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut libc::c_char,
              ) as libc::c_long as size_t) as *mut libc::c_char;
              setinputstring(pstr);
            }
          }
          nlpp = &mut bqlist;
          while !(*nlpp).is_null() {
            nlpp = &mut (**nlpp).next
          }
          *nlpp = stzalloc(::std::mem::size_of::<nodelist>() as libc::c_ulong) as *mut nodelist;
          /* (*nlpp)->next = NULL; - stzalloc did it */
          if oldstyle != 0 {
            saveprompt = doprompt;
            doprompt = 0i32 as smallint
          }
          n = list(2i32);
          if oldstyle != 0 {
            doprompt = saveprompt
          } else if readtoken() != TRP as libc::c_int {
            raise_error_unexpected_syntax(TRP as libc::c_int);
          }
          (**nlpp).n = n;
          if oldstyle != 0 {
            /*
             * Start reading from old file again, ignoring any pushed back
             * tokens left from the backquote parsing
             */
            popfile();
            tokpushback = 0i32 as smallint
          }
          while (*ash_ptr_to_globals_memstack).g_stacknleft <= savelen {
            growstackblock();
          }
          out = (*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut libc::c_char;
          if !str_0.is_null() {
            memcpy(
              out as *mut libc::c_void,
              str_0 as *const libc::c_void,
              savelen,
            );
            out = out.offset(savelen as isize)
          }
          let fresh105 = out;
          out = out.offset(1);
          *fresh105 = '\u{84}' as i32 as libc::c_uchar as libc::c_char;
          // TODO: why was this translated this way?
          // (oldstyle) != 0;
          current_block = 5916212523694105379;
        }
        17595510130556652878 => {
          if (*synstack).varnest != 0 {
            (*synstack).set_innerdq((*synstack).innerdq() ^ 1i32 as uint8_t)
          }
          current_block = 5634654558795828693;
        }
        _ => {}
      }
      match current_block {
        5634654558795828693 => {
          if eofmark.is_null() {
            let fresh91 = out;
            out = out.offset(1);
            *fresh91 = '\u{88}' as i32 as libc::c_uchar as libc::c_char
          }
        }
        _ => {}
      }
      /* parse substitution */
      c = pgetc()
    }
    match current_block {
      15768484401365413375 => {
        break;
        /* exit outer loop */
      }
      _ => {
        let fresh77 = out; /* exit outer loop */
        out = out.offset(1);
        *fresh77 = c as libc::c_char;
        nlprompt();
        c = pgetc()
      }
    }
  }
  if (*synstack).syntax as libc::c_int == 3i32 {
    raise_error_syntax(b"missing \'))\'\x00" as *const u8 as *const libc::c_char);
  }
  if (*synstack).syntax as libc::c_int != 0i32 && eofmark.is_null() {
    raise_error_syntax(b"unterminated quoted string\x00" as *const u8 as *const libc::c_char);
  }
  if (*synstack).varnest != 0i32 {
    /* { */
    raise_error_syntax(b"missing \'}\'\x00" as *const u8 as *const libc::c_char);
  }
  let fresh98 = out;
  out = out.offset(1);
  *fresh98 = '\u{0}' as i32 as libc::c_char;
  len = out.wrapping_offset_from(
    (*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut libc::c_char,
  ) as libc::c_long as size_t;
  out = (*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut libc::c_char;
  if eofmark.is_null() {
    if (c == '>' as i32 || c == '<' as i32 || c == 0x100i32 + '>' as i32)
      && quotef as libc::c_int == 0i32
    {
      if isdigit_str9(out) != 0 {
        /* out is already checked to be a valid number or "" */
        let mut fd: libc::c_int = if *out as libc::c_int == '\u{0}' as i32 {
          -1i32
        } else {
          atoi(out)
        };
        let mut np: *mut node = 0 as *mut node;
        np = stzalloc(::std::mem::size_of::<nfile>() as libc::c_ulong) as *mut node;
        if c == '>' as i32 {
          (*np).nfile.fd = 1i32;
          c = pgetc_eatbnl();
          if c == '>' as i32 {
            (*np).type_0 = 21i32 as smallint
          } else if c == '|' as i32 {
            (*np).type_0 = 18i32 as smallint
          } else if c == '&' as i32 {
            (*np).type_0 = 22i32 as smallint
          } else {
            /* it also can be NTO2 (>&file), but we can't figure it out yet */
            (*np).type_0 = 16i32 as smallint;
            pungetc();
          }
        } else if c == 0x100i32 + '>' as i32 {
          /* this flags &> redirection */
          (*np).nfile.fd = 1i32; /* this is '>', no need to check */
          pgetc();
          (*np).type_0 = 17i32 as smallint
        } else {
          /* c == '<' */
          /*np->nfile.fd = 0; - stzalloc did it */
          c = pgetc_eatbnl();
          match c {
            60 => {
              if ::std::mem::size_of::<nfile>() as libc::c_ulong
                != ::std::mem::size_of::<nhere>() as libc::c_ulong
              {
                np = stzalloc(::std::mem::size_of::<nhere>() as libc::c_ulong) as *mut node
                /*np->nfile.fd = 0; - stzalloc did it */
              }
              (*np).type_0 = 24i32 as smallint;
              heredoc = stzalloc(::std::mem::size_of::<heredoc>() as libc::c_ulong) as *mut heredoc;
              (*heredoc).here = np;
              c = pgetc_eatbnl();
              if c == '-' as i32 {
                (*heredoc).striptabs = 1i32 as smallint
              } else {
                /*heredoc->striptabs = 0; - stzalloc did it */
                pungetc();
              }
            }
            38 => (*np).type_0 = 23i32 as smallint,
            62 => (*np).type_0 = 20i32 as smallint,
            _ => {
              (*np).type_0 = 19i32 as smallint;
              pungetc();
            }
          }
        }
        if fd >= 0i32 {
          (*np).nfile.fd = fd
        }
        redirnode = np;
        /* passed as params: out, c */
        lasttoken = TREDIR as libc::c_int as token_id_t;
        return lasttoken as libc::c_int;
      }
      /* else: non-number X seen, interpret it
       * as "NNNX>file" = "NNNX >file" */
    }
    pungetc();
  }
  quoteflag = quotef;
  backquotelist = bqlist;
  grabstackblock(len);
  wordtext = out;
  lasttoken = TWORD as libc::c_int as token_id_t;
  return lasttoken as libc::c_int;
}
/* end of readtoken */
/*
 * Read the next input token.
 * If the token is a word, we set backquotelist to the list of cmds in
 *      backquotes.  We set quoteflag to true if any part of the word was
 *      quoted.
 * If the token is TREDIR, then we set redirnode to a structure containing
 *      the redirection.
 *
 * [Change comment:  here documents and internal procedures]
 * [Readtoken shouldn't have any arguments.  Perhaps we should make the
 *  word parsing code into a separate routine.  In this case, readtoken
 *  doesn't need to have any internal procedures, but parseword does.
 *  We could also make parseoperator in essence the main routine, and
 *  have parseword (readtoken1?) handle both words and redirection.]
 */
/* singles must be first! */
static mut xxreadtoken_chars: [libc::c_char; 7] = [
  '\n' as i32 as libc::c_char,
  '(' as i32 as libc::c_char,
  ')' as i32 as libc::c_char,
  '&' as i32 as libc::c_char,
  '|' as i32 as libc::c_char,
  ';' as i32 as libc::c_char,
  0i32 as libc::c_char,
];
static mut xxreadtoken_tokens: [libc::c_char; 10] = [
  TNL as libc::c_int as libc::c_char,
  TLP as libc::c_int as libc::c_char,
  TRP as libc::c_int as libc::c_char,
  TBACKGND as libc::c_int as libc::c_char,
  TPIPE as libc::c_int as libc::c_char,
  TSEMI as libc::c_int as libc::c_char,
  TEOF as libc::c_int as libc::c_char,
  TAND as libc::c_int as libc::c_char,
  TOR as libc::c_int as libc::c_char,
  TENDCASE as libc::c_int as libc::c_char,
];
unsafe extern "C" fn xxreadtoken() -> libc::c_int {
  let mut c: libc::c_int = 0; /* for (;;) */
  if tokpushback != 0 {
    tokpushback = 0i32 as smallint;
    return lasttoken as libc::c_int;
  }
  setprompt_if(needprompt, 2i32);
  loop
  /* until token or start of word found */
  {
    c = pgetc_eatbnl(); /* return readtoken1(...) */
    if c == ' ' as i32 || c == '\t' as i32 || c == 257i32 {
      continue;
    }
    if c == '#' as i32 {
      loop {
        c = pgetc();
        if !(c != '\n' as i32 && c != 256i32) {
          break;
        }
      }
      pungetc();
    } else {
      if c == '\\' as i32 {
        break;
      }
      let mut p: *const libc::c_char = 0 as *const libc::c_char;
      p = xxreadtoken_chars
        .as_ptr()
        .offset(::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as isize)
        .offset(-1);
      if c != 256i32 {
        if c == '\n' as i32 {
          nlnoprompt();
        }
        p = strchr(xxreadtoken_chars.as_ptr(), c);
        if p.is_null() {
          break;
        }
        if p.wrapping_offset_from(xxreadtoken_chars.as_ptr()) as libc::c_long as libc::c_int >= 3i32
        {
          let mut cc: libc::c_int = pgetc_eatbnl();
          if cc == c {
            /* double occurrence? */
            p = p.offset((3i32 + 1i32) as isize)
          } else {
            pungetc();
            if c == '&' as i32 && cc == '>' as i32 {
              /* return readtoken1(...) */
              /* &> */
              break;
            }
          }
        }
      }
      lasttoken = xxreadtoken_tokens
        [p.wrapping_offset_from(xxreadtoken_chars.as_ptr()) as libc::c_long as usize]
        as token_id_t;
      return lasttoken as libc::c_int;
    }
  }
  return readtoken1(c, 0i32, 0 as *mut libc::c_void as *mut libc::c_char, 0i32);
}
/* old xxreadtoken */
/* old xxreadtoken */
unsafe extern "C" fn readtoken() -> libc::c_int {
  let mut t: libc::c_int = 0;
  let mut kwd: libc::c_int = checkkwd as libc::c_int;
  loop {
    t = xxreadtoken();
    /*
     * eat newlines
     */
    if kwd & 0x4i32 != 0 {
      while t == TNL as libc::c_int {
        parseheredoc();
        t = xxreadtoken()
      }
    }
    if t != TWORD as libc::c_int || quoteflag as libc::c_int != 0 {
      break;
    }
    /*
     * check for keywords
     */
    if kwd & 0x2i32 != 0 {
      let mut pp: *const *const libc::c_char = 0 as *const *const libc::c_char;
      pp = findkwd(wordtext);
      if !pp.is_null() {
        t = pp.wrapping_offset_from(tokname_array.as_ptr()) as libc::c_long as libc::c_int;
        lasttoken = t as token_id_t;
        break;
      }
    }
    if !(checkkwd as libc::c_int & 0x1i32 != 0) {
      break;
    }
    let mut ap: *mut alias = 0 as *mut alias;
    ap = lookupalias(wordtext, 1i32);
    if ap.is_null() {
      break;
    }
    if *(*ap).val != 0 {
      pushstring((*ap).val, ap);
    }
  }
  checkkwd = 0i32 as smallint;
  return t;
}
unsafe extern "C" fn peektoken() -> libc::c_int {
  let mut t: libc::c_int = 0;
  t = readtoken();
  tokpushback = 1i32 as smallint;
  return t;
}
/*
 * Read and parse a command.  Returns NODE_EOF on end of file.
 * (NULL is a valid parse tree indicating a blank line.)
 */
unsafe extern "C" fn parsecmd(mut interact: libc::c_int) -> *mut node {
  tokpushback = 0i32 as smallint;
  checkkwd = 0i32 as smallint;
  heredoclist = 0 as *mut heredoc;
  doprompt = interact as smallint;
  setprompt_if(doprompt, doprompt as libc::c_int);
  needprompt = 0i32 as smallint;
  return list(1i32);
}
/*
 * Input any here documents.
 */
unsafe extern "C" fn parseheredoc() {
  let mut here: *mut heredoc = 0 as *mut heredoc;
  let mut n: *mut node = 0 as *mut node;
  here = heredoclist;
  heredoclist = 0 as *mut heredoc;
  while !here.is_null() {
    tokpushback = 0i32 as smallint;
    setprompt_if(needprompt, 2i32);
    readtoken1(
      pgetc(),
      if (*(*here).here).type_0 as libc::c_int == 24i32 {
        2i32
      } else {
        1i32
      },
      (*here).eofmark,
      (*here).striptabs as libc::c_int,
    );
    n = stzalloc(::std::mem::size_of::<narg>() as libc::c_ulong) as *mut node;
    (*n).narg.type_0 = 15i32 as smallint;
    /*n->narg.next = NULL; - stzalloc did it */
    (*n).narg.text = wordtext;
    (*n).narg.backquote = backquotelist;
    (*(*here).here).nhere.doc = n;
    here = (*here).next
  }
}
unsafe extern "C" fn expandstr(
  mut ps: *const libc::c_char,
  mut syntax_type: libc::c_int,
) -> *const libc::c_char {
  let mut n: node = node { type_0: 0 };
  let mut saveprompt: libc::c_int = 0;
  let mut file_stop: *mut parsefile = g_parsefile;
  let mut saveint: libc::c_int = 0;
  let mut savehandler: *mut jmploc = (*ash_ptr_to_globals_misc).exception_handler;
  let mut jmploc: jmploc = jmploc {
    loc: [__jmp_buf_tag {
      __jmpbuf: [0; 8],
      __mask_was_saved: 0,
      __saved_mask: __sigset_t { __val: [0; 16] },
    }; 1],
  };
  /* XXX Fix (char *) cast. */
  setinputstring(ps as *mut libc::c_char);
  saveprompt = doprompt as libc::c_int;
  doprompt = 0i32 as smallint;
  /* readtoken1() might die horribly.
   * Try a prompt with syntactically wrong command:
   * PS1='$(date "+%H:%M:%S) > ' */

  ::std::ptr::write_volatile(
    &mut saveint as *mut libc::c_int,
    (*ash_ptr_to_globals_misc).suppress_int,
  );
  if _setjmp(jmploc.loc.as_mut_ptr()) == 0i32 {
    (*ash_ptr_to_globals_misc).exception_handler = &mut jmploc;
    readtoken1(
      pgetc(),
      syntax_type,
      1i32 as uintptr_t as *mut libc::c_char,
      0i32,
    );
  }
  (*ash_ptr_to_globals_misc).exception_handler = savehandler;
  asm!("" : : : "memory" : "volatile");
  ::std::ptr::write_volatile(
    &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
    saveint,
  );
  if (*ash_ptr_to_globals_misc).suppress_int == 0i32
    && (*ash_ptr_to_globals_misc).pending_int as libc::c_int != 0
  {
    raise_interrupt();
  }
  doprompt = saveprompt as smallint;
  /* Try: PS1='`xxx(`' */
  unwindfiles(file_stop);
  n.narg.type_0 = 15i32 as smallint;
  n.narg.next = 0 as *mut node;
  n.narg.text = wordtext;
  n.narg.backquote = backquotelist;
  /* expandarg() might fail too:
   * PS1='$((123+))'
   */
  ::std::ptr::write_volatile(
    &mut saveint as *mut libc::c_int,
    (*ash_ptr_to_globals_misc).suppress_int,
  );
  if _setjmp(jmploc.loc.as_mut_ptr()) == 0i32 {
    (*ash_ptr_to_globals_misc).exception_handler = &mut jmploc;
    expandarg(&mut n, 0 as *mut arglist, 0x100i32);
  } else if (*ash_ptr_to_globals_misc).exception_type as libc::c_int == 4i32 {
    exitshell();
  }
  (*ash_ptr_to_globals_misc).exception_handler = savehandler;
  asm!("" : : : "memory" : "volatile");
  ::std::ptr::write_volatile(
    &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
    saveint,
  );
  if (*ash_ptr_to_globals_misc).suppress_int == 0i32
    && (*ash_ptr_to_globals_misc).pending_int as libc::c_int != 0
  {
    raise_interrupt();
  }
  return (*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *const libc::c_char;
}
#[inline]
unsafe extern "C" fn parser_eof() -> libc::c_int {
  return (tokpushback as libc::c_int != 0 && lasttoken as libc::c_int == TEOF as libc::c_int)
    as libc::c_int;
}
/* starting line number of current function, or 0 if not in a function */
/* Forward decl way out to parsing code - dotrap needs it */
/*
 * Execute a command or commands contained in a string.
 */
unsafe extern "C" fn evalstring(mut s: *mut libc::c_char, mut flags: libc::c_int) -> libc::c_int {
  let mut savehandler: *mut jmploc = 0 as *mut jmploc;
  let mut jmploc: jmploc = jmploc {
    loc: [__jmp_buf_tag {
      __jmpbuf: [0; 8],
      __mask_was_saved: 0,
      __saved_mask: __sigset_t { __val: [0; 16] },
    }; 1],
  };
  let mut ex: libc::c_int = 0;
  let mut n: *mut node = 0 as *mut node;
  let mut smark: stackmark = stackmark {
    stackp: 0 as *mut stack_block,
    stacknxt: 0 as *mut libc::c_char,
    stacknleft: 0,
  };
  let mut status: libc::c_int = 0;
  s = sstrdup(s);
  setinputstring(s);
  setstackmark(&mut smark);
  status = 0i32;
  /* On exception inside execution loop, we must popfile().
   * Try interactively:
   *	readonly a=a
   *	command eval "a=b"  # throws "is read only" error
   * "command BLTIN" is not supposed to abort (even in non-interactive use).
   * But if we skip popfile(), we hit EOF in eval's string, and exit.
   */
  ::std::ptr::write_volatile(
    &mut savehandler as *mut *mut jmploc,
    (*ash_ptr_to_globals_misc).exception_handler,
  );
  ex = _setjmp(jmploc.loc.as_mut_ptr());
  if !(ex != 0) {
    (*ash_ptr_to_globals_misc).exception_handler = &mut jmploc;
    loop {
      n = parsecmd(0i32);
      if !(n != -1i64 as *mut node) {
        break;
      }
      let mut i: libc::c_int = 0;
      i = evaltree(n, flags & !(if parser_eof() != 0 { 0i32 } else { 0o1i32 }));
      if !n.is_null() {
        status = i
      }
      popstackmark(&mut smark);
      if evalskip != 0 {
        break;
      }
    }
  }
  popstackmark(&mut smark);
  popfile();
  stunalloc(s as *mut libc::c_void);
  (*ash_ptr_to_globals_misc).exception_handler = savehandler;
  if ex != 0 {
    longjmp(
      (*(*ash_ptr_to_globals_misc).exception_handler)
        .loc
        .as_mut_ptr(),
      ex,
    );
  }
  return status;
}
/*
 * The eval command.
 */
unsafe extern "C" fn evalcmd(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
  mut flags: libc::c_int,
) -> libc::c_int {
  let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut concat: *mut libc::c_char = 0 as *mut libc::c_char;
  if !(*argv.offset(1)).is_null() {
    p = *argv.offset(1);
    argv = argv.offset(2);
    if !(*argv.offset(0)).is_null() {
      concat = (*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut libc::c_char;
      loop {
        concat = stack_putstr(p, concat);
        let fresh108 = argv;
        argv = argv.offset(1);
        p = *fresh108;
        if p.is_null() {
          break;
        }
        concat = _STPUTC(' ' as i32, concat)
      }
      concat = _STPUTC('\u{0}' as i32, concat);
      p = stalloc(concat.wrapping_offset_from(
        (*ash_ptr_to_globals_memstack).g_stacknxt as *mut libc::c_void as *mut libc::c_char,
      ) as libc::c_long as size_t) as *mut libc::c_char
    }
    return evalstring(p, flags & 0o2i32);
  }
  return 0i32;
}
/*
 * Read and execute commands.
 * "Top" is nonzero for the top level command loop;
 * it turns on prompting if the shell is interactive.
 */
unsafe extern "C" fn cmdloop(mut top: libc::c_int) -> libc::c_int {
  let mut n: *mut node = 0 as *mut node;
  let mut smark: stackmark = stackmark {
    stackp: 0 as *mut stack_block,
    stacknxt: 0 as *mut libc::c_char,
    stacknleft: 0,
  };
  let mut inter: libc::c_int = 0;
  let mut status: libc::c_int = 0i32;
  let mut numeof: libc::c_int = 0i32;
  loop {
    let mut skip: libc::c_int = 0;
    setstackmark(&mut smark);
    if doing_jobctl != 0 {
      showjobs(0x4i32 | 0x8i32);
    }
    inter = 0i32;
    if (*ash_ptr_to_globals_misc).optlist[3] as libc::c_int != 0 && top != 0 {
      inter += 1;
      chkmail();
    }
    n = parsecmd(inter);
    if n == -1i64 as *mut node {
      if top == 0 || numeof >= 50i32 {
        break;
      }
      if stoppedjobs() == 0 {
        if (*ash_ptr_to_globals_misc).optlist[2] == 0 {
          break;
        }
        out2str(b"\nUse \"exit\" to leave shell.\n\x00" as *const u8 as *const libc::c_char);
      }
      numeof += 1
    } else if (*ash_ptr_to_globals_misc).optlist[5] as libc::c_int == 0i32 {
      let mut i: libc::c_int = 0;
      /* job_warning can only be 2,1,0. Here 2->1, 1/0->0 */
      (*ash_ptr_to_globals_misc).job_warning =
        ((*ash_ptr_to_globals_misc).job_warning as libc::c_int >> 1i32) as smallint;
      numeof = 0i32;
      i = evaltree(n, 0i32);
      if !n.is_null() {
        status = i
      }
    }
    popstackmark(&mut smark);
    skip = evalskip as libc::c_int;
    if !(skip != 0) {
      continue;
    }
    evalskip = (evalskip as libc::c_int & !(1i32 << 2i32)) as smallint;
    break;
  }
  return status;
}
/*
 * Take commands from a file.  To be compatible we should do a path
 * search for the file, which is necessary to find sub-commands.
 */
unsafe extern "C" fn find_dot_file(mut name: *mut libc::c_char) -> *mut libc::c_char {
  let mut fullname: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut path: *const libc::c_char = (*ash_ptr_to_globals_var).varinit
    [(1i32 * 2i32 + 1i32) as usize]
    .var_text
    .offset(5);
  let mut statb: stat = stat {
    st_dev: 0,
    st_ino: 0,
    st_nlink: 0,
    st_mode: 0,
    st_uid: 0,
    st_gid: 0,
    __pad0: 0,
    st_rdev: 0,
    st_size: 0,
    st_blksize: 0,
    st_blocks: 0,
    st_atim: timespec {
      tv_sec: 0,
      tv_nsec: 0,
    },
    st_mtim: timespec {
      tv_sec: 0,
      tv_nsec: 0,
    },
    st_ctim: timespec {
      tv_sec: 0,
      tv_nsec: 0,
    },
    __glibc_reserved: [0; 3],
  };
  /* don't try this for absolute or relative paths */
  if !strchr(name, '/' as i32).is_null() {
    return name;
  }
  loop {
    fullname = path_advance(&mut path, name);
    if fullname.is_null() {
      break;
    }
    if stat(fullname, &mut statb) == 0i32
      && statb.st_mode & 0o170000i32 as libc::c_uint == 0o100000i32 as libc::c_uint
    {
      /*
       * Don't bother freeing here, since it will
       * be freed by the caller.
       */
      return fullname;
    }
    if fullname != name {
      stunalloc(fullname as *mut libc::c_void);
    }
  }
  /* not found in PATH */
  ash_msg_and_raise_error(
    b"%s: not found\x00" as *const u8 as *const libc::c_char,
    name,
  );
  /* NOTREACHED */
}
unsafe extern "C" fn dotcmd(
  mut _argc_: libc::c_int,
  mut _argv_: *mut *mut libc::c_char,
) -> libc::c_int {
  /* "false; . empty_file; echo $?" should print 0, not 1: */
  let mut status: libc::c_int = 0i32;
  let mut fullname: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut argv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  let mut args_need_save: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut saveparam: shparam = shparam {
    nparam: 0,
    optind: 0,
    optoff: 0,
    malloced: 0,
    p: 0 as *mut *mut libc::c_char,
  };
  //???
  //	struct strlist *sp;
  //	for (sp = cmdenviron; sp; sp = sp->next)
  //		setvareq(ckstrdup(sp->text), VSTRFIXED | VTEXTFIXED);
  nextopt((*ash_ptr_to_globals_misc).nullstr.as_mut_ptr()); /* handle possible "--" */
  argv = argptr;
  if (*argv.offset(0)).is_null() {
    /* bash says: "bash: .: filename argument required" */
    return 2i32;
    /* bash compat */
  }
  /* This aborts if file isn't found, which is POSIXly correct.
   * bash returns exitcode 1 instead.
   */
  fullname = find_dot_file(*argv.offset(0));
  argv = argv.offset(1);
  args_need_save = *argv.offset(0);
  if !args_need_save.is_null() {
    /* ". FILE ARGS", and ARGS are not empty */
    let mut argc: libc::c_int = 0;
    ::std::ptr::write_volatile(
      &mut saveparam as *mut shparam,
      (*ash_ptr_to_globals_var).shellparam,
    );
    (*ash_ptr_to_globals_var).shellparam.malloced = 0i32 as libc::c_uchar;
    argc = 1i32;
    while !(*argv.offset(argc as isize)).is_null() {
      argc += 1
    }
    (*ash_ptr_to_globals_var).shellparam.nparam = argc;
    (*ash_ptr_to_globals_var).shellparam.p = argv
  }
  /* This aborts if file can't be opened, which is POSIXly correct.
   * bash returns exitcode 1 instead.
   */
  setinputfile(fullname, INPUT_PUSH_FILE as libc::c_int);
  commandname = fullname;
  status = cmdloop(0i32);
  popfile();
  if !args_need_save.is_null() {
    freeparam(&mut (*ash_ptr_to_globals_var).shellparam as *mut shparam as *mut shparam);
    (*ash_ptr_to_globals_var).shellparam = saveparam
  }
  return status;
}
unsafe extern "C" fn exitcmd(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  if stoppedjobs() != 0 {
    return 0i32;
  }
  if !(*argv.offset(1)).is_null() {
    (*ash_ptr_to_globals_misc).exitstatus = number(*argv.offset(1)) as uint8_t
  }
  raise_exception(4i32);
  /* NOTREACHED */
}
/*
 * Read a file containing shell functions.
 */
unsafe extern "C" fn readcmdfile(mut name: *mut libc::c_char) {
  setinputfile(name, INPUT_PUSH_FILE as libc::c_int);
  cmdloop(0i32);
  popfile();
}
/* %builtin in alt. path */
/* ============ find_command inplementation */
/*
 * Resolve a command name.  If you change this routine, you may have to
 * change the shellexec routine as well.
 */
unsafe extern "C" fn find_command(
  mut name: *mut libc::c_char,
  mut entry: *mut cmdentry,
  mut act: libc::c_int,
  mut path: *const libc::c_char,
) {
  let mut current_block: u64;
  let mut cmdp: *mut tblentry = 0 as *mut tblentry;
  let mut idx: libc::c_int = 0;
  let mut prev: libc::c_int = 0;
  let mut fullname: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut statb: stat = stat {
    st_dev: 0,
    st_ino: 0,
    st_nlink: 0,
    st_mode: 0,
    st_uid: 0,
    st_gid: 0,
    __pad0: 0,
    st_rdev: 0,
    st_size: 0,
    st_blksize: 0,
    st_blocks: 0,
    st_atim: timespec {
      tv_sec: 0,
      tv_nsec: 0,
    },
    st_mtim: timespec {
      tv_sec: 0,
      tv_nsec: 0,
    },
    st_ctim: timespec {
      tv_sec: 0,
      tv_nsec: 0,
    },
    __glibc_reserved: [0; 3],
  };
  let mut e: libc::c_int = 0;
  let mut updatetbl: libc::c_int = 0;
  let mut bcmd: *mut builtincmd = 0 as *mut builtincmd;
  /* If name contains a slash, don't use PATH or hash table */
  if !strchr(name, '/' as i32).is_null() {
    (*entry).u.index = -1i32;
    if act & 0x2i32 != 0 {
      if stat(name, &mut statb) < 0i32 {
        (*entry).cmdtype = -1i32 as smallint;
        return;
      }
    }
    (*entry).cmdtype = 0i32 as smallint;
    return;
  }
  /* #if ENABLE_FEATURE_SH_STANDALONE... moved after builtin check */
  updatetbl = (path
    == (*ash_ptr_to_globals_var).varinit[(1i32 * 2i32 + 1i32) as usize]
      .var_text
      .offset(5)) as libc::c_int;
  if updatetbl == 0 {
    act |= 0x8i32;
    if !strstr(path, b"%builtin\x00" as *const u8 as *const libc::c_char).is_null() {
      act |= 0x20i32
    }
  }
  /* If name is in the table, check answer will be ok */
  cmdp = cmdlookup(name, 0i32);
  if !cmdp.is_null() {
    let mut bit: libc::c_int = 0;
    match (*cmdp).cmdtype as libc::c_int {
      1 => bit = 0x4i32,
      2 => bit = 0x20i32,
      0 | _ => bit = 0x8i32,
    }
    if act & bit != 0 {
      updatetbl = 0i32;
      cmdp = 0 as *mut tblentry;
      current_block = 1118134448028020070;
    } else if (*cmdp).rehash as libc::c_int == 0i32 {
      current_block = 12367743889786799597;
    } else {
      current_block = 1118134448028020070;
    }
  } else {
    current_block = 1118134448028020070;
  }
  match current_block {
    1118134448028020070 => {
      /* If %builtin not in path, check for builtin next */
      bcmd = find_builtin(name);
      if !bcmd.is_null() {
        if *(*bcmd).name.offset(0) as libc::c_int & 2i32 != 0 {
          current_block = 10188044987189456047;
        } else if act & 0x8i32 != 0 {
          if act & 0x20i32 == 0 {
            current_block = 10188044987189456047;
          } else {
            current_block = 4567019141635105728;
          }
        } else if builtinloc <= 0i32 {
          current_block = 10188044987189456047;
        } else {
          current_block = 4567019141635105728;
        }
      } else {
        current_block = 4567019141635105728;
      }
      match current_block {
        4567019141635105728 => {
          /* We have to search path. */
          prev = -1i32; /* where to start */
          if !cmdp.is_null() && (*cmdp).rehash as libc::c_int != 0 {
            /* doing a rehash */
            if (*cmdp).cmdtype as libc::c_int == 2i32 {
              prev = builtinloc
            } else {
              prev = (*cmdp).param.index
            }
          }
          e = 2i32;
          idx = -1i32;
          loop
          /* ignore unimplemented options */
          {
            fullname = path_advance(&mut path, name);
            if fullname.is_null() {
              current_block = 15864857819291709765;
              break;
            }
            stunalloc(fullname as *mut libc::c_void);
            /* NB: code below will still use fullname
             * despite it being "unallocated" */
            idx += 1;
            if !pathopt.is_null() {
              if !prefix(pathopt, b"builtin\x00" as *const u8 as *const libc::c_char).is_null() {
                if !bcmd.is_null() {
                  current_block = 10188044987189456047;
                  break;
                } else {
                  continue;
                }
              } else if act & 0x4i32 != 0
                || prefix(pathopt, b"func\x00" as *const u8 as *const libc::c_char).is_null()
              {
                continue;
              }
            }
            /* if rehash, don't redo absolute path names */
            if *fullname.offset(0) as libc::c_int == '/' as i32 && idx <= prev {
              if !(idx < prev) {
                current_block = 12367743889786799597; /* if we fail, this will be the error */
                break;
              }
            } else if stat(fullname, &mut statb) < 0i32 {
              if *bb_errno != 2i32 && *bb_errno != 20i32 {
                e = *bb_errno
              }
            } else {
              e = 13i32;
              if !(statb.st_mode & 0o170000i32 as libc::c_uint == 0o100000i32 as libc::c_uint) {
                continue;
              }
              if !pathopt.is_null() {
                /* this is a %func directory */
                stalloc(strlen(fullname).wrapping_add(1i32 as libc::c_ulong));
                /* NB: stalloc will return space pointed by fullname
                 * (because we don't have any intervening allocations
                 * between stunalloc above and this stalloc) */
                readcmdfile(fullname);
                cmdp = cmdlookup(name, 0i32);
                if cmdp.is_null() || (*cmdp).cmdtype as libc::c_int != 1i32 {
                  ash_msg_and_raise_error(
                    b"%s not defined in %s\x00" as *const u8 as *const libc::c_char,
                    name,
                    fullname,
                  );
                }
                stunalloc(fullname as *mut libc::c_void);
                current_block = 12367743889786799597;
                break;
              } else {
                if updatetbl == 0 {
                  (*entry).cmdtype = 0i32 as smallint;
                  (*entry).u.index = idx;
                  return;
                }
                ::std::ptr::write_volatile(
                  &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
                  ::std::ptr::read_volatile::<libc::c_int>(
                    &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
                  ) + 1,
                );
                asm!("" : : : "memory" : "volatile");
                cmdp = cmdlookup(name, 1i32);
                (*cmdp).cmdtype = 0i32 as smallint;
                (*cmdp).param.index = idx;
                int_on();
                current_block = 12367743889786799597;
                break;
              }
            }
          }
          match current_block {
            10188044987189456047 => {}
            12367743889786799597 => {}
            _ => {
              /* We failed.  If there was an entry for this command, delete it */
              if !cmdp.is_null() && updatetbl != 0 {
                delete_cmd_entry();
              }
              if act & 0x1i32 != 0 {
                let mut hookp: *mut tblentry = cmdlookup(
                  b"command_not_found_handle\x00" as *const u8 as *const libc::c_char,
                  0i32,
                );
                if !hookp.is_null() && (*hookp).cmdtype as libc::c_int == 1i32 {
                  let mut argv: [*mut libc::c_char; 3] = [0 as *mut libc::c_char; 3];
                  argv[0] = b"command_not_found_handle\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
                  argv[1] = name;
                  argv[2] = 0 as *mut libc::c_char;
                  evalfun((*hookp).param.func, 2i32, argv.as_mut_ptr(), 0i32);
                  (*entry).cmdtype = -1i32 as smallint;
                  return;
                }
                ash_msg(
                  b"%s: %s\x00" as *const u8 as *const libc::c_char,
                  name,
                  errmsg(e, b"not found\x00" as *const u8 as *const libc::c_char),
                );
              }
              (*entry).cmdtype = -1i32 as smallint;
              return;
            }
          }
        }
        _ => {}
      }
      match current_block {
        12367743889786799597 => {}
        _ => {
          if updatetbl == 0 {
            (*entry).cmdtype = 2i32 as smallint;
            (*entry).u.cmd = bcmd;
            return;
          }
          ::std::ptr::write_volatile(
            &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
            ::std::ptr::read_volatile::<libc::c_int>(
              &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
            ) + 1,
          );
          asm!("" : : : "memory" : "volatile");
          cmdp = cmdlookup(name, 1i32);
          (*cmdp).cmdtype = 2i32 as smallint;
          (*cmdp).param.cmd = bcmd;
          int_on();
        }
      }
    }
    _ => {}
  }
  /* if not invalidated by cd, we're done */
  (*cmdp).rehash = 0i32 as libc::c_char;
  (*entry).cmdtype = (*cmdp).cmdtype;
  (*entry).u = (*cmdp).param;
}
/*
 * The trap builtin.
 */
unsafe extern "C" fn trapcmd(
  mut _argc: libc::c_int,
  mut _argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut action: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut ap: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  let mut signo: libc::c_int = 0;
  let mut exitcode: libc::c_int = 0;
  nextopt((*ash_ptr_to_globals_misc).nullstr.as_mut_ptr());
  ap = argptr;
  if (*ap).is_null() {
    signo = 0i32;
    while signo < 64i32 + 1i32 {
      let mut tr: *mut libc::c_char = *(*ash_ptr_to_globals_misc).trap_ptr.offset(signo as isize);
      if !tr.is_null() {
        /* note: bash adds "SIG", but only if invoked
         * as "bash". If called as "sh", or if set -o posix,
         * then it prints short signal names.
         * We are printing short names: */
        out1fmt(
          b"trap -- %s %s\n\x00" as *const u8 as *const libc::c_char,
          single_quote(tr),
          get_signame(signo),
        );
        /* trap_ptr != trap only if we are in special-cased `trap` code.
         * In this case, we will exit very soon, no need to free(). */
        /* if (trap_ptr != trap && tp[0]) */
        /*	free(tr); */
      }
      signo += 1
    }
    /*
    if (trap_ptr != trap) {
      free(trap_ptr);
      trap_ptr = trap;
    }
    */
    return 0i32;
  }
  /* Why the second check?
   * "trap NUM [sig2]..." is the same as "trap - NUM [sig2]..."
   * In this case, NUM is signal no, not an action.
   */
  action = 0 as *mut libc::c_char;
  if !(*ap.offset(1)).is_null() && is_number(*ap.offset(0)) == 0 {
    let fresh109 = ap;
    ap = ap.offset(1);
    action = *fresh109
  }
  exitcode = 0i32;
  while !(*ap).is_null() {
    signo = get_signum(*ap);
    if signo < 0i32 {
      /* Mimic bash message exactly */
      ash_msg(
        b"%s: invalid signal specification\x00" as *const u8 as *const libc::c_char,
        *ap,
      );
      exitcode = 1i32
    } else {
      ::std::ptr::write_volatile(
        &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
        ::std::ptr::read_volatile::<libc::c_int>(
          &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
        ) + 1,
      );
      asm!("" : : : "memory" : "volatile");
      if !action.is_null() {
        if *action.offset(0) as libc::c_int == '-' as i32 && *action.offset(1) == 0 {
          action = 0 as *mut libc::c_char
        } else {
          if *action.offset(0) != 0 {
            /* not NULL and not "" and not "-" */
            (*ash_ptr_to_globals_misc).may_have_traps = 1i32 as uint8_t
          }
          action = xstrdup(action)
        }
      }
      free((*ash_ptr_to_globals_misc).trap[signo as usize] as *mut libc::c_void);
      (*ash_ptr_to_globals_misc).trap[signo as usize] = action;
      if signo != 0i32 {
        setsignal(signo);
      }
      int_on();
    }
    ap = ap.offset(1)
  }
  return exitcode;
}
/* ============ Builtins */
unsafe extern "C" fn helpcmd(
  mut _argc: libc::c_int,
  mut _argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut col: libc::c_uint = 0;
  let mut i: libc::c_uint = 0;
  out1fmt(b"Built-in commands:\n------------------\n\x00" as *const u8 as *const libc::c_char);
  col = 0i32 as libc::c_uint;
  i = 0i32 as libc::c_uint;
  while i
    < (::std::mem::size_of::<[builtincmd; 44]>() as libc::c_ulong)
      .wrapping_div(::std::mem::size_of::<builtincmd>() as libc::c_ulong) as libc::c_uint
  {
    col = col.wrapping_add(out1fmt(
      b"%c%s\x00" as *const u8 as *const libc::c_char,
      if col == 0i32 as libc::c_uint {
        '\t' as i32
      } else {
        ' ' as i32
      },
      builtintab[i as usize].name.offset(1),
    ) as libc::c_uint);
    if col > 60i32 as libc::c_uint {
      out1fmt(b"\n\x00" as *const u8 as *const libc::c_char);
      col = 0i32 as libc::c_uint
    }
    i = i.wrapping_add(1)
  }
  newline_and_flush(stdout);
  return 0i32;
}
unsafe extern "C" fn historycmd(
  mut _argc: libc::c_int,
  mut _argv: *mut *mut libc::c_char,
) -> libc::c_int {
  if !line_input_state.is_null() {
    show_history(line_input_state);
  }
  return 0i32;
}
/*
 * The export and readonly commands.
 */
unsafe extern "C" fn exportcmd(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut vp: *mut var = 0 as *mut var;
  let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut p: *const libc::c_char = 0 as *const libc::c_char;
  let mut aptr: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  let mut opt: libc::c_char = 0;
  let mut flag: libc::c_int = 0;
  let mut flag_off: libc::c_int = 0;
  /* "readonly" in bash accepts, but ignores -n.
   * We do the same: it saves a conditional in nextopt's param.
   */
  flag_off = 0i32;
  loop {
    opt = nextopt(b"np\x00" as *const u8 as *const libc::c_char) as libc::c_char;
    if !(opt as libc::c_int != '\u{0}' as i32) {
      break;
    }
    if opt as libc::c_int == 'n' as i32 {
      flag_off = 0x1i32
    }
  }
  flag = 0x1i32;
  if *(*argv.offset(0)).offset(0) as libc::c_int == 'r' as i32 {
    flag = 0x2i32;
    flag_off = 0i32
    /* readonly ignores -n */
  }
  flag_off = !flag_off;
  /*if (opt_p_not_specified) - bash doesn't check this. Try "export -p NAME" */
  aptr = argptr;
  name = *aptr;
  if !name.is_null() {
    let mut current_block_18: u64;
    loop {
      p = strchr(name, '=' as i32);
      if !p.is_null() {
        p = p.offset(1);
        current_block_18 = 16203760046146113240;
      } else {
        vp = *findvar(hashvar(name), name);
        if !vp.is_null() {
          (*vp).flags = ((*vp).flags | flag) & flag_off;
          current_block_18 = 15652330335145281839;
        } else {
          current_block_18 = 16203760046146113240;
        }
      }
      match current_block_18 {
        16203760046146113240 => {
          setvar(name, p, flag & flag_off);
        }
        _ => {}
      }
      aptr = aptr.offset(1);
      name = *aptr;
      if name.is_null() {
        break;
      }
    }
    return 0i32;
  }
  /* No arguments. Show the list of exported or readonly vars.
   * -n is ignored.
   */
  showvars(*argv.offset(0), flag, 0i32);
  return 0i32;
}
/*
 * Delete a function if it exists.
 */
unsafe extern "C" fn unsetfunc(mut name: *const libc::c_char) {
  let mut cmdp: *mut tblentry = 0 as *mut tblentry;
  cmdp = cmdlookup(name, 0i32);
  if !cmdp.is_null() && (*cmdp).cmdtype as libc::c_int == 1i32 {
    delete_cmd_entry();
  };
}
/*
 * The unset builtin command.  We unset the function before we unset the
 * variable to allow a function to be unset when there is a readonly variable
 * with the same name.
 */
unsafe extern "C" fn unsetcmd(
  mut _argc: libc::c_int,
  mut _argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut ap: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  let mut i: libc::c_int = 0;
  let mut flag: libc::c_int = 0i32;
  loop {
    i = nextopt(b"vf\x00" as *const u8 as *const libc::c_char);
    if !(i != 0i32) {
      break;
    }
    flag = i
  }
  ap = argptr;
  while !(*ap).is_null() {
    if flag != 'f' as i32 {
      unsetvar(*ap);
    } else if flag != 'v' as i32 {
      unsetfunc(*ap);
    }
    ap = ap.offset(1)
  }
  return 0i32;
}
static mut timescmd_str: [libc::c_uchar; 9] = [
  ' ' as i32 as libc::c_uchar,
  0u64 as libc::c_uchar,
  '\n' as i32 as libc::c_uchar,
  8u64 as libc::c_uchar,
  ' ' as i32 as libc::c_uchar,
  16u64 as libc::c_uchar,
  '\n' as i32 as libc::c_uchar,
  24u64 as libc::c_uchar,
  0i32 as libc::c_uchar,
];
unsafe extern "C" fn timescmd(
  mut _argc: libc::c_int,
  mut _argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut clk_tck: libc::c_uint = 0;
  let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
  let mut buf: tms = tms {
    tms_utime: 0,
    tms_stime: 0,
    tms_cutime: 0,
    tms_cstime: 0,
  };
  clk_tck = bb_clk_tck();
  times(&mut buf);
  p = timescmd_str.as_ptr();
  loop {
    let mut sec: libc::c_uint = 0;
    let mut frac: libc::c_uint = 0;
    let mut t: libc::c_ulong = 0;
    t = *((&mut buf as *mut tms as *mut libc::c_char).offset(*p.offset(1) as libc::c_int as isize)
      as *mut clock_t) as libc::c_ulong;
    sec = t.wrapping_div(clk_tck as libc::c_ulong) as libc::c_uint;
    frac = t.wrapping_rem(clk_tck as libc::c_ulong) as libc::c_uint;
    out1fmt(
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
/*
 * The let builtin. Partially stolen from GNU Bash, the Bourne Again SHell.
 * Copyright (C) 1987, 1989, 1991 Free Software Foundation, Inc.
 *
 * Copyright (C) 2003 Vladimir Oleynik <dzo@simtreas.ru>
 */
unsafe extern "C" fn letcmd(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut i: arith_t = 0;
  argv = argv.offset(1);
  if (*argv).is_null() {
    ash_msg_and_raise_error(b"expression expected\x00" as *const u8 as *const libc::c_char);
  }
  loop {
    i = ash_arith(*argv);
    argv = argv.offset(1);
    if (*argv).is_null() {
      break;
    }
  }
  return (i == 0) as libc::c_int;
}
/*
 * The read builtin. Options:
 *      -r              Do not interpret '\' specially
 *      -s              Turn off echo (tty only)
 *      -n NCHARS       Read NCHARS max
 *      -p PROMPT       Display PROMPT on stderr (if input is from tty)
 *      -t SECONDS      Timeout after SECONDS (tty or pipe only)
 *      -u FD           Read from given FD instead of fd 0
 *      -d DELIM        End on DELIM char, not newline
 * This uses unbuffered input, which may be avoidable in some cases.
 * TODO: bash also has:
 *      -a ARRAY        Read into array[0],[1],etc
 *      -e              Use line editing (tty only)
 */
unsafe extern "C" fn readcmd(
  mut _argc: libc::c_int,
  mut _argv: *mut *mut libc::c_char,
) -> libc::c_int {
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
  }; /* can be NULL */
  let mut r: *const libc::c_char = 0 as *const libc::c_char;
  let mut i: libc::c_int = 0;
  memset(
    &mut params as *mut builtin_read_params as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<builtin_read_params>() as libc::c_ulong,
  );
  loop {
    i = nextopt(b"p:u:rt:n:sd:\x00" as *const u8 as *const libc::c_char);
    if !(i != '\u{0}' as i32) {
      break;
    }
    match i {
      112 => params.opt_p = optionarg,
      110 => params.opt_n = optionarg,
      115 => params.read_flags |= BUILTIN_READ_SILENT as libc::c_int,
      116 => params.opt_t = optionarg,
      114 => params.read_flags |= BUILTIN_READ_RAW as libc::c_int,
      117 => params.opt_u = optionarg,
      100 => params.opt_d = optionarg,
      _ => {}
    }
  }
  params.argv = argptr;
  params.setvar =
    Some(setvar0 as unsafe extern "C" fn(_: *const libc::c_char, _: *const libc::c_char) -> ());
  params.ifs = bltinlookup(b"IFS\x00" as *const u8 as *const libc::c_char);
  loop
  /* "read -s" needs to save/restore termios, can't allow ^C
   * to jump out of it.
   */
  {
    ::std::ptr::write_volatile(
      &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
      ::std::ptr::read_volatile::<libc::c_int>(
        &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
      ) + 1,
    );
    asm!("" : : : "memory" : "volatile");
    r = shell_builtin_read(&mut params);
    int_on();
    if !(r as uintptr_t == 1i32 as libc::c_ulong && *bb_errno == 4i32) {
      break;
    }
    /* To get SIGCHLD: sleep 1 & read x; echo $x
     * Correct behavior is to not exit "read"
     */
    if !((*ash_ptr_to_globals_misc).pending_sig as libc::c_int == 0i32) {
      break;
    }
  }
  if r as uintptr_t > 1i32 as libc::c_ulong {
    ash_msg_and_raise_error(r);
  }
  return r as uintptr_t as libc::c_int;
}
unsafe extern "C" fn umaskcmd(
  mut _argc: libc::c_int,
  mut _argv: *mut *mut libc::c_char,
) -> libc::c_int {
  static mut permuser: [libc::c_char; 3] = [111, 103, 117];
  let mut mask: mode_t = 0;
  let mut symbolic_mode: libc::c_int = 0i32;
  while nextopt(b"S\x00" as *const u8 as *const libc::c_char) != '\u{0}' as i32 {
    symbolic_mode = 1i32
  }
  ::std::ptr::write_volatile(
    &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
    ::std::ptr::read_volatile::<libc::c_int>(
      &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
    ) + 1,
  );
  asm!("" : : : "memory" : "volatile");
  mask = umask(0i32 as __mode_t);
  umask(mask);
  int_on();
  if (*argptr).is_null() {
    if symbolic_mode != 0 {
      let mut buf: [libc::c_char; 19] = [0; 19];
      let mut p: *mut libc::c_char = buf.as_mut_ptr();
      let mut i: libc::c_int = 0;
      i = 2i32;
      loop {
        let fresh110 = p;
        p = p.offset(1);
        *fresh110 = ',' as i32 as libc::c_char;
        let fresh111 = p;
        p = p.offset(1);
        *fresh111 = permuser[i as usize];
        let fresh112 = p;
        p = p.offset(1);
        *fresh112 = '=' as i32 as libc::c_char;
        /* mask is 0..0uuugggooo. i=2 selects uuu bits */
        if mask & 0o400i32 as libc::c_uint == 0 {
          let fresh113 = p;
          p = p.offset(1);
          *fresh113 = 'r' as i32 as libc::c_char
        }
        if mask & 0o200i32 as libc::c_uint == 0 {
          let fresh114 = p;
          p = p.offset(1);
          *fresh114 = 'w' as i32 as libc::c_char
        }
        if mask & 0o100i32 as libc::c_uint == 0 {
          let fresh115 = p;
          p = p.offset(1);
          *fresh115 = 'x' as i32 as libc::c_char
        }
        mask <<= 3i32;
        i -= 1;
        if i < 0i32 {
          break;
        }
      }
      *p = '\u{0}' as i32 as libc::c_char;
      puts(buf.as_mut_ptr().offset(1));
    } else {
      out1fmt(b"%04o\n\x00" as *const u8 as *const libc::c_char, mask);
    }
  } else {
    let mut modestr: *mut libc::c_char = *argptr;
    /* numeric umasks are taken as-is */
    /* symbolic umasks are inverted: "umask a=rx" calls umask(222) */
    if !((*modestr.offset(0) as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32) {
      mask ^= 0o777i32 as libc::c_uint
    }
    mask = bb_parse_mode(modestr, mask) as mode_t;
    if mask > 0o777i32 as libc::c_uint {
      ash_msg_and_raise_error(
        b"illegal mode: %s\x00" as *const u8 as *const libc::c_char,
        modestr,
      );
    }
    if !((*modestr.offset(0) as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32) {
      mask ^= 0o777i32 as libc::c_uint
    }
    umask(mask);
  }
  return 0i32;
}
unsafe extern "C" fn ulimitcmd(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  return shell_builtin_ulimit(argv);
}
/* ============ Interrupts / exceptions */
/* ============ main() and helpers */
/*
 * Called to exit the shell.
 */
unsafe extern "C" fn exitshell() -> ! {
  let mut loc: jmploc = jmploc {
    loc: [__jmp_buf_tag {
      __jmpbuf: [0; 8],
      __mask_was_saved: 0,
      __saved_mask: __sigset_t { __val: [0; 16] },
    }; 1],
  };
  let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut status: libc::c_int = 0;
  status = (*ash_ptr_to_globals_misc).exitstatus as libc::c_int;
  if _setjmp(loc.loc.as_mut_ptr()) != 0 {
    if (*ash_ptr_to_globals_misc).exception_type as libc::c_int == 4i32 {
      status = (*ash_ptr_to_globals_misc).exitstatus as libc::c_int
    }
  } else {
    (*ash_ptr_to_globals_misc).exception_handler = &mut loc;
    p = (*ash_ptr_to_globals_misc).trap[0];
    if !p.is_null() {
      (*ash_ptr_to_globals_misc).trap[0] = 0 as *mut libc::c_char;
      evalskip = 0i32 as smallint;
      evalstring(p, 0i32);
      /*free(p); - we'll exit soon */
    }
  }
  /* dash wraps setjobctl(0) in "if (setjmp(loc.loc) == 0) {...}".
   * our setjobctl(0) does not panic if tcsetpgrp fails inside it.
   */
  setjobctl(0i32);
  flush_stdout_stderr();
  _exit(status);
  /* NOTREACHED */
}
/* Don't inline: conserve stack of caller from having our locals too */
#[inline(never)]
unsafe extern "C" fn init() {
  /* we will never free this */
  basepf.buf = xmalloc(if 1i32 != 0 { 1024i32 } else { 1024i32 } as size_t) as *mut libc::c_char; /* ensure we install handler even if it is SIG_IGNed */
  basepf.next_to_pgetc = basepf.buf;
  basepf.linno = 1i32;
  (*ash_ptr_to_globals_misc).sigmode[(17i32 - 1i32) as usize] = 1i32 as libc::c_char;
  setsignal(17i32);
  /* bash re-enables SIGHUP which is SIG_IGNed on entry.
   * Try: "trap '' HUP; bash; echo RET" and type "kill -HUP $$"
   */
  signal(1i32, None);
  let mut envp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  let mut p: *const libc::c_char = 0 as *const libc::c_char;
  initvar();
  envp = environ;
  while !envp.is_null() && !(*envp).is_null() {
    /* Used to have
     *			p = endofname(*envp);
     *			if (p != *envp && *p == '=') {
     * here to weed out badly-named variables, but this breaks
     * scenarios where people do want them passed to children:
     * import os
     * os.environ["test-test"]="test"
     * if os.fork() == 0:
     *   os.execv("ash", [ 'ash', '-c', 'eval $(export -p); echo OK' ])  # fixes this
     * os.execv("ash", [ 'ash', '-c', 'env | grep test-test' ])  # breaks this
     */
    if !strchr(*envp, '=' as i32).is_null() {
      setvareq(*envp, 0x1i32 | 0x8i32);
    }
    envp = envp.offset(1)
  }
  setvareq(defifsvar.as_ptr() as *mut libc::c_char, 0x8i32);
  setvareq(defoptindvar.as_ptr() as *mut libc::c_char, 0x8i32);
  setvar0(
    b"PPID\x00" as *const u8 as *const libc::c_char,
    utoa(getppid() as libc::c_uint),
  );
  p = lookupvar(b"SHLVL\x00" as *const u8 as *const libc::c_char);
  setvar(
    b"SHLVL\x00" as *const u8 as *const libc::c_char,
    utoa(((if !p.is_null() { atoi(p) } else { 0i32 }) + 1i32) as libc::c_uint),
    0x1i32,
  );
  if lookupvar(b"HOSTNAME\x00" as *const u8 as *const libc::c_char).is_null() {
    let mut uts: utsname = utsname {
      sysname: [0; 65],
      nodename: [0; 65],
      release: [0; 65],
      version: [0; 65],
      machine: [0; 65],
      domainname: [0; 65],
    };
    uname(&mut uts);
    setvar0(
      b"HOSTNAME\x00" as *const u8 as *const libc::c_char,
      uts.nodename.as_mut_ptr(),
    );
  }
  p = lookupvar(b"PWD\x00" as *const u8 as *const libc::c_char);
  if !p.is_null() {
    let mut st1: stat = stat {
      st_dev: 0,
      st_ino: 0,
      st_nlink: 0,
      st_mode: 0,
      st_uid: 0,
      st_gid: 0,
      __pad0: 0,
      st_rdev: 0,
      st_size: 0,
      st_blksize: 0,
      st_blocks: 0,
      st_atim: timespec {
        tv_sec: 0,
        tv_nsec: 0,
      },
      st_mtim: timespec {
        tv_sec: 0,
        tv_nsec: 0,
      },
      st_ctim: timespec {
        tv_sec: 0,
        tv_nsec: 0,
      },
      __glibc_reserved: [0; 3],
    };
    let mut st2: stat = stat {
      st_dev: 0,
      st_ino: 0,
      st_nlink: 0,
      st_mode: 0,
      st_uid: 0,
      st_gid: 0,
      __pad0: 0,
      st_rdev: 0,
      st_size: 0,
      st_blksize: 0,
      st_blocks: 0,
      st_atim: timespec {
        tv_sec: 0,
        tv_nsec: 0,
      },
      st_mtim: timespec {
        tv_sec: 0,
        tv_nsec: 0,
      },
      st_ctim: timespec {
        tv_sec: 0,
        tv_nsec: 0,
      },
      __glibc_reserved: [0; 3],
    };
    if *p.offset(0) as libc::c_int != '/' as i32
      || stat(p, &mut st1) != 0
      || stat(b".\x00" as *const u8 as *const libc::c_char, &mut st2) != 0
      || st1.st_dev != st2.st_dev
      || st1.st_ino != st2.st_ino
    {
      p = 0 as *const libc::c_char
    }
  }
  setpwd(p, 0i32);
}
//usage:#define ash_trivial_usage
//usage:	"[-/+OPTIONS] [-/+o OPT]... [-c 'SCRIPT' [ARG0 [ARGS]] / FILE [ARGS] / -s [ARGS]]"
//usage:#define ash_full_usage "\n\n"
//usage:	"Unix shell interpreter"
/*
 * Process the shell command line arguments.
 */
unsafe extern "C" fn procargs(mut argv: *mut *mut libc::c_char) -> libc::c_int {
  let mut current_block: u64;
  let mut i: libc::c_int = 0;
  let mut xminusc: *const libc::c_char = 0 as *const libc::c_char;
  let mut xargv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  let mut login_sh: libc::c_int = 0;
  xargv = argv;
  login_sh = (!(*xargv.offset(0)).is_null()
    && *(*xargv.offset(0)).offset(0) as libc::c_int == '-' as i32) as libc::c_int;
  if !(*ash_ptr_to_globals_misc).minusc.is_null() {
    current_block = 11877083500734903013;
  } else {
    (*ash_ptr_to_globals_misc).arg0 = *xargv.offset(0);
    /* if (xargv[0]) - mmm, this is always true! */
    xargv = xargv.offset(1);
    argptr = xargv;
    i = 0i32;
    while i < NOPTS as libc::c_int {
      (*ash_ptr_to_globals_misc).optlist[i as usize] = 2i32 as libc::c_char;
      i += 1
    }
    if options(&mut login_sh) != 0 {
      /* it already printed err message */
      raise_exception(1i32);
    }
    xargv = argptr;
    xminusc = (*ash_ptr_to_globals_misc).minusc;
    if (*xargv).is_null() {
      if !xminusc.is_null() {
        ash_msg_and_raise_error(
          bb_msg_requires_arg.as_ptr(),
          b"-c\x00" as *const u8 as *const libc::c_char,
        );
      }
      (*ash_ptr_to_globals_misc).optlist[6] = 1i32 as libc::c_char
    }
    if (*ash_ptr_to_globals_misc).optlist[3] as libc::c_int == 2i32
      && (*ash_ptr_to_globals_misc).optlist[6] as libc::c_int == 1i32
      && (*ash_ptr_to_globals_misc).minusc.is_null()
      && isatty(0i32) != 0
      && isatty(1i32) != 0
    {
      /* we are on tty */
      (*ash_ptr_to_globals_misc).optlist[3] = 1i32 as libc::c_char
    }
    if (*ash_ptr_to_globals_misc).optlist[4] as libc::c_int == 2i32 {
      (*ash_ptr_to_globals_misc).optlist[4] = (*ash_ptr_to_globals_misc).optlist[3]
    }
    i = 0i32;
    while i < NOPTS as libc::c_int {
      if (*ash_ptr_to_globals_misc).optlist[i as usize] as libc::c_int == 2i32 {
        (*ash_ptr_to_globals_misc).optlist[i as usize] = 0i32 as libc::c_char
      }
      i += 1
    }
    /* POSIX 1003.2: first arg after "-c CMD" is $0, remainder $1... */
    if !xminusc.is_null() {
      let fresh116 = xargv;
      xargv = xargv.offset(1);
      (*ash_ptr_to_globals_misc).minusc = *fresh116;
      if !(*xargv).is_null() {
        current_block = 11877083500734903013;
      } else {
        current_block = 7245201122033322888;
      }
    } else if (*ash_ptr_to_globals_misc).optlist[6] == 0 {
      setinputfile(*xargv, 0i32);
      current_block = 11877083500734903013;
    } else {
      current_block = 7245201122033322888;
    }
  }
  match current_block {
    11877083500734903013 => {
      let fresh117 = xargv;
      xargv = xargv.offset(1);
      (*ash_ptr_to_globals_misc).arg0 = *fresh117;
      commandname = (*ash_ptr_to_globals_misc).arg0
    }
    _ => {}
  }
  (*ash_ptr_to_globals_var).shellparam.p = xargv;
  (*ash_ptr_to_globals_var).shellparam.optind = 1i32;
  (*ash_ptr_to_globals_var).shellparam.optoff = -1i32;
  /* assert(shellparam.malloced == 0 && shellparam.nparam == 0); */
  while !(*xargv).is_null() {
    (*ash_ptr_to_globals_var).shellparam.nparam += 1;
    xargv = xargv.offset(1)
  }
  optschanged();
  return login_sh;
}
/*
 * Read /etc/profile, ~/.profile, $ENV.
 */
unsafe extern "C" fn read_profile(mut name: *const libc::c_char) {
  name = expandstr(name, 1i32);
  if setinputfile(
    name,
    INPUT_PUSH_FILE as libc::c_int | INPUT_NOFILE_OK as libc::c_int,
  ) < 0i32
  {
    return;
  }
  cmdloop(0i32);
  popfile();
}
/*
 * This routine is called when an error or an interrupt occurs in an
 * interactive shell and control is returned to the main command loop.
 * (In dash, this function is auto-generated by build machinery).
 */
unsafe extern "C" fn reset() {
  /* from eval.c: */
  evalskip = 0i32 as smallint;
  loopnest = 0i32;
  /* from expand.c: */
  ifsfree();
  /* from input.c: */
  (*g_parsefile).left_in_buffer = 0i32; /* clear input buffer */
  (*g_parsefile).left_in_line = 0i32;
  popallfiles();
  /* from redir.c: */
  unwindredir(0 as *mut redirtab);
  /* from var.c: */
  unwindlocalvars(0 as *mut localvar_list);
}
/* After v = xrealloc_vector(v, SHIFT, idx) it's ok to use
 * at least v[idx] and v[idx+1], for all idx values.
 * SHIFT specifies how many new elements are added (1:2, 2:4, ..., 8:256...)
 * when all elements are used up. New elements are zeroed out.
 * xrealloc_vector(v, SHIFT, idx) *MUST* be called with consecutive IDXs -
 * skipping an index is a bad bug - it may miss a realloc!
 */
//TODO: supply a pointer to char[11] buffer (avoid statics)?
/* cp.c, mv.c, install.c depend on these values. CAREFUL when changing them! */
/* -p */
/* !-d */
/* -R */
/* -f */
/* -i */
/* -l */
/* -s */
/* -L */
/* -H */
/* -a = -pdR (mapped in cp.c) */
/* -r = -dR  (mapped in cp.c) */
/* -P = -d   (mapped in cp.c) */
/* -v */
/* -u */
/* -T */
/* --remove-destination */
/* bit 17 skipped for "cp --parents" */
/* cp --reflink=auto */
/* cp --reflink[=always] */
/*
 * Hole. cp may have some bits set here,
 * they should not affect remove_file()/copy_file()
 */
/* NB: without FILEUTILS_RECUR in flags, it will basically "cat"
 * the source, not copy (unless "source" is a directory).
 * This makes "cp /dev/null file" and "install /dev/null file" (!!!)
 * work coreutils-compatibly. */
/*ACTION_REVERSE      = (1 << 4), - unused */
/* more than enough for "/dev/ttyXXX" */
/* bb_copyfd_XX print read/write errors and return -1 if they occur */
/* "short" copy can be detected by return value < size */
/* this helper yells "short read!" if param is not -1 */
/* xxxx_strip version can modify its parameter:
 * "/"        -> "/"
 * "abc"      -> "abc"
 * "abc/def"  -> "def"
 * "abc/def/" -> "def" !!
 */
/* "abc/def/" -> "" and it never modifies 'path' */
/* Simpler version: does not special case "/" string */
/* NB: can violate const-ness (similarly to strchr) */
/* !RETURNS_MALLOC: it's a realloc-like function */
/* bb_signals(BB_FATAL_SIGS, handler) catches all signals which
 * otherwise would kill us, except for those resulting from bugs:
 * SIGSEGV, SIGILL, SIGFPE.
 * Other fatal signals not included (TODO?):
 * SIGBUS   Bus error (bad memory access)
 * SIGPOLL  Pollable event. Synonym of SIGIO
 * SIGPROF  Profiling timer expired
 * SIGSYS   Bad argument to routine
 * SIGTRAP  Trace/breakpoint trap
 *
 * The only known arch with some of these sigs not fitting
 * into 32 bits is parisc (SIGXCPU=33, SIGXFSZ=34, SIGSTKFLT=36).
 * Dance around with long long to guard against that...
 */
// Write to pipe with no readers
// Quit from keyboard
// Abort signal from abort(3)
// Timer signal from alarm(2)
// Virtual alarm clock
// CPU time limit exceeded
// File size limit exceeded
// Yes kids, these are also fatal!
/* Unlike signal() and bb_signals, sets handler with sigaction()
 * and in a way that while signal handler is run, no other signals
 * will be blocked; syscalls will not be restarted: */
/* syscalls like read() will be interrupted with EINTR: */
/* syscalls like read() won't be interrupted (though select/poll will be): */
/* Will do sigaction(signum, act, NULL): */
/* SIG_BLOCK/SIG_UNBLOCK all signals: */
/* Return old set in the same set: */
/* Standard handler which just records signo */
/* not FAST_FUNC! */
/* In this form code with pipes is much more readable */
/* Useful for having small structure members/global variables */
/* | AF_DECnet */
/* | AF_IPX */
/* SO_REUSEADDR allows a server to rebind to an address that is already
 * "in use" by old connections to e.g. previous server instance which is
 * killed or crashed. Without it bind will fail until all such connections
 * time out. Linux does not allow multiple live binds on same ip:port
 * regardless of SO_REUSEADDR (unlike some other flavors of Unix).
 * Turn it on before you call bind(). */
/* On Linux this never fails. */
/* NB: returns port in host byte order */
/* Create stream socket, and allocate suitable lsa.
 * (lsa of correct size and lsa->sa.sa_family (AF_INET/AF_INET6))
 * af == AF_UNSPEC will result in trying to create IPv6 socket,
 * and if kernel doesn't support it, fall back to IPv4.
 * This is useful if you plan to bind to resulting local lsa.
 */
/* Create server socket bound to bindaddr:port. bindaddr can be NULL,
 * numeric IP ("N.N.N.N") or numeric IPv6 address,
 * and can have ":PORT" suffix (for IPv6 use "[X:X:...:X]:PORT").
 * Only if there is no suffix, port argument is used */
/* NB: these set SO_REUSEADDR before bind */
/* Create client TCP socket connected to peer:port. Peer cannot be NULL.
 * Peer can be numeric IP ("N.N.N.N"), numeric IPv6 address or hostname,
 * and can have ":PORT" suffix (for IPv6 use "[X:X:...:X]:PORT").
 * If there is no suffix, port argument is used */
/* Connect to peer identified by lsa */
/* Get local address of bound or accepted socket */
/* Get remote address of connected or accepted socket */
/* Return malloc'ed len_and_sockaddr with socket address of host:port
 * Currently will return IPv4 or IPv6 sockaddrs only
 * (depending on host), but in theory nothing prevents e.g.
 * UNIX socket address being returned, IPX sockaddr etc...
 * On error does bb_error_msg and returns NULL */
/* Version which dies on error */
/* Same, useful if you want to force family (e.g. IPv6) */
/* Assign sin[6]_port member if the socket is an AF_INET[6] one,
 * otherwise no-op. Useful for ftp.
 * NB: does NOT do htons() internally, just direct assignment. */
/* Retrieve sin[6]_port or return -1 for non-INET[6] lsa's */
/* Reverse DNS. Returns NULL on failure. */
/* This one doesn't append :PORTNUM */
/* This one also doesn't fall back to dotted IP (returns NULL) */
/* inet_[ap]ton on steroids */
// "old" (ipv4 only) API
// users: traceroute.c hostname.c - use _list_ of all IPs
// Also mount.c and inetd.c are using gethostbyname(),
// + inet_common.c has additional IPv4-only stuff
/* opaque */
// RFC 5246
// sequence number
//   Each connection state contains a sequence number, which is
//   maintained separately for read and write states.  The sequence
//   number MUST be set to zero whenever a connection state is made the
//   active state.  Sequence numbers are of type uint64 and may not
//   exceed 2^64-1.
/*uint64_t read_seq64_be;*/
/*uint8_t *server_write_MAC_key;*/
//used by AES_GCM
/* 0 if argv[0] is NULL: */
/* Guaranteed to NOT be a macro (smallest code). Saves nearly 2k on uclibc.
 * But potentially slow, don't use in one-billion-times loops */
/* Note: does not use stdio, writes to fd 2 directly */
// gcc-4.1.1 still isn't good enough at optimizing it
// (+200 bytes compared to macro)
//static ALWAYS_INLINE
//int LONE_DASH(const char *s) { return s[0] == '-' && !s[1]; }
//static ALWAYS_INLINE
//int NOT_LONE_DASH(const char *s) { return s[0] != '-' || s[1]; }
/* Returns a string with unprintable chars replaced by '?' or
 * SUBST_WCHAR. This function is unicode-aware. */
/* Prints unprintable char ch as ^C or M-c to file
 * (M-c is used only if ch is ORed with PRINTABLE_META),
 * else it is printed as-is (except for ch = 0x9b) */
/* Return a string that is the printable representation of character ch.
 * Buffer must hold at least four characters. */
// NB: will return short read on error, not -1,
// if some data was read before error occurred
// Reads one line a-la fgets (but doesn't save terminating '\n').
// Reads byte-by-byte. Useful when it is important to not read ahead.
// Bytes are appended to pfx (which must be malloced, or NULL).
/* Reads block up to *maxsz_p (default: INT_MAX - 4095) */
/* Returns NULL if file can't be opened (default max size: INT_MAX - 4095) */
/* Never returns NULL */
/* Else use variable one (a bit more expensive) */
/* Autodetects gzip/bzip2 formats. fd may be in the middle of the file! */
/* Autodetects .gz etc */
/* lzma has no signature, need a little helper. NB: exist only for ENABLE_FEATURE_SEAMLESS_LZMA=y */
// NB: will return short write on error, not -1,
// if some data was written before error occurred
/* Close fd, but check for failures (some types of write errors) */
/* Reads and prints to stdout till eof, then closes FILE. Exits on error: */
/* Reads a line from a text file, up to a newline or NUL byte, inclusive.
 * Returns malloc'ed char*. If end is NULL '\n' isn't considered
 * end of line. If end isn't NULL, length of the chunk is stored in it.
 * Returns NULL if EOF/error.
 */
/* Reads up to (and including) TERMINATING_STRING: */
/* Same, with limited max size, and returns the length (excluding NUL): */
/* Chops off TERMINATING_STRING from the end: */
/* Reads up to (and including) "\n" or NUL byte: */
/* Chops off '\n' from the end, unlike fgets: */
/* Same, but doesn't try to conserve space (may have some slack after the end) */
/* extern char *xmalloc_fgetline_fast(FILE *file) FAST_FUNC RETURNS_MALLOC; */
/* Prints warning to stderr and returns NULL on failure: */
/* "Opens" stdin if filename is special, else just opens file: */
/* not FAST_FUNC! */
/* Wrapper which restarts poll on EINTR or ENOMEM.
 * On other errors complains [perror("poll")] and returns.
 * Warning! May take (much) longer than timeout_ms to return!
 * If this is a problem, use bare poll and open-code EINTR/ENOMEM handling */
/* Convert each alpha char in str to lower-case */
/* Returns a pointer past the formatted number, does NOT null-terminate */
/* Intelligent formatters of bignums */
/* If block_size == 0, display size without fractional part,
 * else display (size * block_size) with one decimal digit.
 * If display_unit == 0, show value no bigger than 1024 with suffix (K,M,G...),
 * else divide by display_unit and do not use suffix. */
/* "1024.0G" */
//TODO: provide pointer to buf (avoid statics)?
/* Put a string of hex bytes ("1b2e66fe"...), return advanced pointer */
/* Reverse */
/* Generate a UUID */
/* Last element is marked by mult == 0 */
/* Specialized: */
/* Using xatoi() instead of naive atoi() is not always convenient -
 * in many places people want *non-negative* values, but store them
 * in signed int. Therefore we need this one:
 * dies if input is not in [0, INT_MAX] range. Also will reject '-0' etc.
 * It should really be named xatoi_nonnegative (since it allows 0),
 * but that would be too long.
 */
/* Useful for reading port numbers */
/* These parse entries in /etc/passwd and /etc/group.  This is desirable
 * for BusyBox since we want to avoid using the glibc NSS stuff, which
 * increases target size and is often not needed on embedded systems.  */
/* wrapper: allows string to contain numeric uid or gid */
/* always sets uid and gid; returns 0 on failure */
/* always sets uid and gid; exits on failure */
/* chown-like handling of "user[:[group]" */
/* versions which cache results (useful for ps, ls etc) */
/* internally usernames are saved in fixed-sized char[] buffers */
/*
 * Returns (-1) terminated malloced result of getgroups().
 * Reallocs group_array (useful for repeated calls).
 * ngroups is an initial size of array. It is rounded up to 32 for realloc.
 * ngroups is updated on return.
 * ngroups can be NULL: bb_getgroups(NULL, NULL) is valid usage.
 * Dies on errors (on Linux, only xrealloc can cause this, not internal getgroups call).
 */
/* BB_EXECxx always execs (it's not doing NOFORK/NOEXEC stuff),
 * but it may exec busybox and call applet instead of searching PATH.
 */
/* xvfork() can't be a _function_, return after vfork in child mangles stack
 * in the parent. It must be a macro. */
/* NOMMU friendy fork+exec: */
/* wait4pid: unlike waitpid, waits ONLY for one process.
 * Returns sig + 0x180 if child is killed by signal.
 * It's safe to pass negative 'pids' from failed [v]fork -
 * wait4pid will return -1 (and will not clobber [v]fork's errno).
 * IOW: rc = wait4pid(spawn(argv));
 *      if (rc < 0) bb_perror_msg("%s", argv[0]);
 *      if (rc > 0) bb_error_msg("exit code: %d", rc & 0xff);
 */
/* ***********************************************************************/
/* spawn_and_wait/run_nofork_applet/run_applet_no_and_exit need to work */
/* carefully together to reinit some global state while not disturbing  */
/* other. Be careful if you change them. Consult docs/nofork_noexec.txt */
/* ***********************************************************************/
/* Same as wait4pid(spawn(argv)), but with NOFORK/NOEXEC if configured: */
/* Does NOT check that applet is NOFORK, just blindly runs it */
/* Helpers for daemonization.
 *
 * bb_daemonize(flags) = daemonize, does not compile on NOMMU
 *
 * bb_daemonize_or_rexec(flags, argv) = daemonizes on MMU (and ignores argv),
 *      rexec's itself on NOMMU with argv passed as command line.
 * Thus bb_daemonize_or_rexec may cause your <applet>_main() to be re-executed
 * from the start. (It will detect it and not reexec again second time).
 * You have to audit carefully that you don't do something twice as a result
 * (opening files/sockets, parsing config files etc...)!
 *
 * Both of the above will redirect fd 0,1,2 to /dev/null and drop ctty
 * (will do setsid()).
 *
 * fork_or_rexec(argv) = bare-bones fork on MMU,
 *      "vfork + re-exec ourself" on NOMMU. No fd redirection, no setsid().
 *      On MMU ignores argv.
 *
 * Helper for network daemons in foreground mode:
 *
 * bb_sanitize_stdio() = make sure that fd 0,1,2 are opened by opening them
 * to /dev/null if they are not.
 */
/* internal use */
//DAEMON_DOUBLE_FORK     = 1 << 4, /* double fork to avoid controlling tty */
/* Clear dangerous stuff, set PATH. Return 1 if was run by different user. */
/* For top, ps. Some argv[i] are replaced by malloced "-opt" strings */
/* { "-", NULL } */
/* BSD-derived getopt() functions require that optind be set to 1 in
 * order to reset getopt() state.  This used to be generally accepted
 * way of resetting getopt().  However, glibc's getopt()
 * has additional getopt() state beyond optind (specifically, glibc
 * extensions such as '+' and '-' at the start of the string), and requires
 * that optind be set to zero to reset its state.  BSD-derived versions
 * of getopt() misbehaved if optind is set to 0 in order to reset getopt(),
 * and glibc's getopt() used to coredump if optind is set 1 in order
 * to reset getopt().
 * Then BSD introduced additional variable "optreset" which should be
 * set to 1 in order to reset getopt().  Sigh.  Standards, anyone?
 *
 * By ~2008, OpenBSD 3.4 was changed to survive glibc-like optind = 0
 * (to interpret it as if optreset was set).
 */
/*def __GLIBC__*/
/* BSD style */
/* Having next pointer as a first member allows easy creation
 * of "llist-compatible" structs, and using llist_FOO functions
 * on them.
 */
/* BTW, surprisingly, changing API to
 *   llist_t *llist_add_to(llist_t *old_head, void *data)
 * etc does not result in smaller code... */
/* start_stop_daemon and udhcpc are special - they want
 * to create pidfiles regardless of FEATURE_PIDFILE */
/* True only if we created pidfile which is *file*, not /dev/null etc */
/* We need to export XXX_main from libbusybox
 * only if we build "individual" binaries
 */
/* Embedded script support */
/* Applets which are useful from another applets */
/*
 * Main routine.  We initialize things, parse the arguments, execute
 * profiles if we're a login shell, and then call cmdloop to execute
 * commands.  The setjmp call sets up the location to jump to when an
 * exception occurs.  When an exception occurs the variable "state"
 * is used to figure out how far we had gotten.
 */
#[no_mangle]
pub unsafe extern "C" fn ash_main(
  mut argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int
/* note: 'argc' is used only if embedded scripts are enabled */ {
  let mut hp: *const libc::c_char = 0 as *const libc::c_char;
  let mut current_block: u64;
  let mut state: smallint = 0;
  let mut jmploc: jmploc = jmploc {
    loc: [__jmp_buf_tag {
      __jmpbuf: [0; 8],
      __mask_was_saved: 0,
      __saved_mask: __sigset_t { __val: [0; 16] },
    }; 1],
  };
  let mut smark: stackmark = stackmark {
    stackp: 0 as *mut stack_block,
    stacknxt: 0 as *mut libc::c_char,
    stacknleft: 0,
  };
  let mut login_sh: libc::c_int = 0;
  /* Initialize global data */
  let ref mut fresh118 =
    *(not_const_pp(&ash_ptr_to_globals_misc as *const *mut globals_misc as *const libc::c_void)
      as *mut *mut globals_misc); /* enable interrupts */
  *fresh118 = xzalloc(::std::mem::size_of::<globals_misc>() as libc::c_ulong) as *mut globals_misc;
  asm!("" : : : "memory" : "volatile");
  (*ash_ptr_to_globals_misc).curdir = (*ash_ptr_to_globals_misc).nullstr.as_mut_ptr();
  (*ash_ptr_to_globals_misc).physdir = (*ash_ptr_to_globals_misc).nullstr.as_mut_ptr();
  (*ash_ptr_to_globals_misc).trap_ptr = (*ash_ptr_to_globals_misc).trap.as_mut_ptr();
  let ref mut fresh119 = *(not_const_pp(
    &ash_ptr_to_globals_memstack as *const *mut globals_memstack as *const libc::c_void,
  ) as *mut *mut globals_memstack);
  *fresh119 =
    xzalloc(::std::mem::size_of::<globals_memstack>() as libc::c_ulong) as *mut globals_memstack;
  asm!("" : : : "memory" : "volatile");
  (*ash_ptr_to_globals_memstack).g_stackp = &mut (*ash_ptr_to_globals_memstack).stackbase;
  (*ash_ptr_to_globals_memstack).g_stacknxt =
    (*ash_ptr_to_globals_memstack).stackbase.space.as_mut_ptr();
  (*ash_ptr_to_globals_memstack).g_stacknleft = MINSIZE as libc::c_int as size_t;
  (*ash_ptr_to_globals_memstack).sstrend = (*ash_ptr_to_globals_memstack)
    .stackbase
    .space
    .as_mut_ptr()
    .offset(MINSIZE as libc::c_int as isize);
  let mut i: libc::c_uint = 0;
  let ref mut fresh120 =
    *(not_const_pp(&ash_ptr_to_globals_var as *const *mut globals_var as *const libc::c_void)
      as *mut *mut globals_var);
  *fresh120 = xzalloc(::std::mem::size_of::<globals_var>() as libc::c_ulong) as *mut globals_var;
  asm!("" : : : "memory" : "volatile");
  i = 0i32 as libc::c_uint;
  while i
    < (::std::mem::size_of::<[C2RustUnnamed_11; 13]>() as libc::c_ulong)
      .wrapping_div(::std::mem::size_of::<C2RustUnnamed_11>() as libc::c_ulong)
      as libc::c_uint
  {
    (*ash_ptr_to_globals_var).varinit[i as usize].flags = varinit_data[i as usize].flags;
    (*ash_ptr_to_globals_var).varinit[i as usize].var_text = varinit_data[i as usize].var_text;
    (*ash_ptr_to_globals_var).varinit[i as usize].var_func = varinit_data[i as usize].var_func;
    i = i.wrapping_add(1)
  }
  strcpy(
    (*ash_ptr_to_globals_var).linenovar.as_mut_ptr(),
    b"LINENO=\x00" as *const u8 as *const libc::c_char,
  );
  (*ash_ptr_to_globals_var).varinit[(1i32 * 2i32 + 1i32 + 5i32) as usize].var_text =
    (*ash_ptr_to_globals_var).linenovar.as_mut_ptr();
  atab = xzalloc(
    (39i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut alias>() as libc::c_ulong),
  ) as *mut *mut alias;
  cmdtable = xzalloc(
    (31i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut tblentry>() as libc::c_ulong),
  ) as *mut *mut tblentry;
  ::std::ptr::write_volatile(&mut state as *mut smallint, 0i32 as smallint);
  if _setjmp(jmploc.loc.as_mut_ptr()) != 0 {
    let mut e: smallint = 0;
    let mut s: smallint = 0;
    reset();
    e = (*ash_ptr_to_globals_misc).exception_type;
    s = state;
    if e as libc::c_int == 4i32
      || s as libc::c_int == 0i32
      || (*ash_ptr_to_globals_misc).optlist[3] as libc::c_int == 0i32
      || (*ash_ptr_to_globals_misc).shlvl != 0
    {
      exitshell();
    }
    if e as libc::c_int == 0i32 {
      newline_and_flush(stderr);
    }
    popstackmark(&mut smark);
    force_int_on();
    if s as libc::c_int == 1i32 {
      current_block = 3542754180382225997;
    } else if s as libc::c_int == 2i32 {
      current_block = 12975382101409004182;
    } else if s as libc::c_int == 3i32 {
      current_block = 4930981158357217823;
    } else {
      current_block = 17980632775274310027;
    }
  } else {
    (*ash_ptr_to_globals_misc).exception_handler = &mut jmploc;
    (*ash_ptr_to_globals_misc).rootpid = getpid();
    init();
    setstackmark(&mut smark);
    if argc < 0i32 {
      /* Non-NULL minusc tells procargs that an embedded script is being run */
      (*ash_ptr_to_globals_misc).minusc = get_script_content((-argc - 1i32) as libc::c_uint)
    }
    login_sh = procargs(argv);
    if login_sh != 0 {
      hp = 0 as *const libc::c_char;
      ::std::ptr::write_volatile(&mut state as *mut smallint, 1i32 as smallint);
      read_profile(b"/etc/profile\x00" as *const u8 as *const libc::c_char);
      current_block = 3542754180382225997;
    } else {
      current_block = 12975382101409004182;
    }
  }
  match current_block {
    3542754180382225997 => {
      ::std::ptr::write_volatile(&mut state as *mut smallint, 2i32 as smallint);
      hp = lookupvar(b"HOME\x00" as *const u8 as *const libc::c_char);
      if !hp.is_null() {
        read_profile(b"$HOME/.profile\x00" as *const u8 as *const libc::c_char);
      }
      current_block = 12975382101409004182;
    }
    _ => {}
  }
  match current_block {
    12975382101409004182 => {
      ::std::ptr::write_volatile(&mut state as *mut smallint, 3i32 as smallint);
      if (*ash_ptr_to_globals_misc).optlist[3] != 0 {
        let mut shinit: *const libc::c_char =
          lookupvar(b"ENV\x00" as *const u8 as *const libc::c_char);
        if !shinit.is_null() && *shinit as libc::c_int != '\u{0}' as i32 {
          read_profile(shinit);
        }
      }
      popstackmark(&mut smark);
      current_block = 4930981158357217823;
    }
    _ => {}
  }
  match current_block {
    4930981158357217823 => {
      ::std::ptr::write_volatile(&mut state as *mut smallint, 4i32 as smallint);
      if !(*ash_ptr_to_globals_misc).minusc.is_null() {
        /* evalstring pushes parsefile stack.
         * Ensure we don't falsely claim that 0 (stdin)
         * is one of stacked source fds.
         * Testcase: ash -c 'exec 1>&0' must not complain. */
        // if (!sflag) g_parsefile->pf_fd = -1;
        // ^^ not necessary since now we special-case fd 0
        // in save_fd_on_redirect()
        // dash: evalstring(minusc, sflag ? 0 : EV_EXIT);
        // The above makes
        //  ash -sc 'echo $-'
        // continue reading input from stdin after running 'echo'.
        // bash does not do this: it prints "hBcs" and exits.
        evalstring((*ash_ptr_to_globals_misc).minusc, 0o1i32);
      }
      if (*ash_ptr_to_globals_misc).optlist[6] as libc::c_int != 0
        || (*ash_ptr_to_globals_misc).minusc.is_null()
      {
        if (*ash_ptr_to_globals_misc).optlist[3] != 0 {
          let mut hp_0: *const libc::c_char =
            lookupvar(b"HISTFILE\x00" as *const u8 as *const libc::c_char);
          if hp_0.is_null() {
            hp_0 = lookupvar(b"HOME\x00" as *const u8 as *const libc::c_char);
            if !hp_0.is_null() {
              ::std::ptr::write_volatile(
                &mut (*ash_ptr_to_globals_misc).suppress_int as *mut libc::c_int,
                ::std::ptr::read_volatile::<libc::c_int>(
                  &(*ash_ptr_to_globals_misc).suppress_int as *const libc::c_int,
                ) + 1,
              );
              asm!("" : : : "memory" : "volatile");
              hp_0 = concat_path_file(
                hp_0,
                b".ash_history\x00" as *const u8 as *const libc::c_char,
              );
              setvar0(b"HISTFILE\x00" as *const u8 as *const libc::c_char, hp_0);
              free(hp_0 as *mut libc::c_char as *mut libc::c_void);
              int_on();
              hp_0 = lookupvar(b"HISTFILE\x00" as *const u8 as *const libc::c_char)
            }
          }
          if !hp_0.is_null() {
            (*line_input_state).hist_file = hp_0
          }
          hp_0 = lookupvar(b"HISTFILESIZE\x00" as *const u8 as *const libc::c_char);
          (*line_input_state).max_history = size_from_HISTFILESIZE(hp_0) as libc::c_int
        }
        current_block = 17980632775274310027;
      } else {
        current_block = 2467484839200770573;
      }
    }
    _ => {}
  }
  match current_block {
    17980632775274310027 => {
      /* XXX ??? - why isn't this before the "if" statement */
      cmdloop(1i32);
    }
    _ => {}
  }
  exitshell();
  /* NOTREACHED */
}
unsafe extern "C" fn run_static_initializers() {
  nodesize = [
    ((::std::mem::size_of::<ncmd>() as libc::c_ulong)
      .wrapping_add(SHELL_SIZE as libc::c_int as libc::c_ulong)
      & !(SHELL_SIZE as libc::c_int) as libc::c_ulong) as uint8_t,
    ((::std::mem::size_of::<npipe>() as libc::c_ulong)
      .wrapping_add(SHELL_SIZE as libc::c_int as libc::c_ulong)
      & !(SHELL_SIZE as libc::c_int) as libc::c_ulong) as uint8_t,
    ((::std::mem::size_of::<nredir>() as libc::c_ulong)
      .wrapping_add(SHELL_SIZE as libc::c_int as libc::c_ulong)
      & !(SHELL_SIZE as libc::c_int) as libc::c_ulong) as uint8_t,
    ((::std::mem::size_of::<nredir>() as libc::c_ulong)
      .wrapping_add(SHELL_SIZE as libc::c_int as libc::c_ulong)
      & !(SHELL_SIZE as libc::c_int) as libc::c_ulong) as uint8_t,
    ((::std::mem::size_of::<nredir>() as libc::c_ulong)
      .wrapping_add(SHELL_SIZE as libc::c_int as libc::c_ulong)
      & !(SHELL_SIZE as libc::c_int) as libc::c_ulong) as uint8_t,
    ((::std::mem::size_of::<nbinary>() as libc::c_ulong)
      .wrapping_add(SHELL_SIZE as libc::c_int as libc::c_ulong)
      & !(SHELL_SIZE as libc::c_int) as libc::c_ulong) as uint8_t,
    ((::std::mem::size_of::<nbinary>() as libc::c_ulong)
      .wrapping_add(SHELL_SIZE as libc::c_int as libc::c_ulong)
      & !(SHELL_SIZE as libc::c_int) as libc::c_ulong) as uint8_t,
    ((::std::mem::size_of::<nbinary>() as libc::c_ulong)
      .wrapping_add(SHELL_SIZE as libc::c_int as libc::c_ulong)
      & !(SHELL_SIZE as libc::c_int) as libc::c_ulong) as uint8_t,
    ((::std::mem::size_of::<nif>() as libc::c_ulong)
      .wrapping_add(SHELL_SIZE as libc::c_int as libc::c_ulong)
      & !(SHELL_SIZE as libc::c_int) as libc::c_ulong) as uint8_t,
    ((::std::mem::size_of::<nbinary>() as libc::c_ulong)
      .wrapping_add(SHELL_SIZE as libc::c_int as libc::c_ulong)
      & !(SHELL_SIZE as libc::c_int) as libc::c_ulong) as uint8_t,
    ((::std::mem::size_of::<nbinary>() as libc::c_ulong)
      .wrapping_add(SHELL_SIZE as libc::c_int as libc::c_ulong)
      & !(SHELL_SIZE as libc::c_int) as libc::c_ulong) as uint8_t,
    ((::std::mem::size_of::<nfor>() as libc::c_ulong)
      .wrapping_add(SHELL_SIZE as libc::c_int as libc::c_ulong)
      & !(SHELL_SIZE as libc::c_int) as libc::c_ulong) as uint8_t,
    ((::std::mem::size_of::<ncase>() as libc::c_ulong)
      .wrapping_add(SHELL_SIZE as libc::c_int as libc::c_ulong)
      & !(SHELL_SIZE as libc::c_int) as libc::c_ulong) as uint8_t,
    ((::std::mem::size_of::<nclist>() as libc::c_ulong)
      .wrapping_add(SHELL_SIZE as libc::c_int as libc::c_ulong)
      & !(SHELL_SIZE as libc::c_int) as libc::c_ulong) as uint8_t,
    ((::std::mem::size_of::<narg>() as libc::c_ulong)
      .wrapping_add(SHELL_SIZE as libc::c_int as libc::c_ulong)
      & !(SHELL_SIZE as libc::c_int) as libc::c_ulong) as uint8_t,
    ((::std::mem::size_of::<narg>() as libc::c_ulong)
      .wrapping_add(SHELL_SIZE as libc::c_int as libc::c_ulong)
      & !(SHELL_SIZE as libc::c_int) as libc::c_ulong) as uint8_t,
    ((::std::mem::size_of::<nfile>() as libc::c_ulong)
      .wrapping_add(SHELL_SIZE as libc::c_int as libc::c_ulong)
      & !(SHELL_SIZE as libc::c_int) as libc::c_ulong) as uint8_t,
    ((::std::mem::size_of::<nfile>() as libc::c_ulong)
      .wrapping_add(SHELL_SIZE as libc::c_int as libc::c_ulong)
      & !(SHELL_SIZE as libc::c_int) as libc::c_ulong) as uint8_t,
    ((::std::mem::size_of::<nfile>() as libc::c_ulong)
      .wrapping_add(SHELL_SIZE as libc::c_int as libc::c_ulong)
      & !(SHELL_SIZE as libc::c_int) as libc::c_ulong) as uint8_t,
    ((::std::mem::size_of::<nfile>() as libc::c_ulong)
      .wrapping_add(SHELL_SIZE as libc::c_int as libc::c_ulong)
      & !(SHELL_SIZE as libc::c_int) as libc::c_ulong) as uint8_t,
    ((::std::mem::size_of::<nfile>() as libc::c_ulong)
      .wrapping_add(SHELL_SIZE as libc::c_int as libc::c_ulong)
      & !(SHELL_SIZE as libc::c_int) as libc::c_ulong) as uint8_t,
    ((::std::mem::size_of::<nfile>() as libc::c_ulong)
      .wrapping_add(SHELL_SIZE as libc::c_int as libc::c_ulong)
      & !(SHELL_SIZE as libc::c_int) as libc::c_ulong) as uint8_t,
    ((::std::mem::size_of::<ndup>() as libc::c_ulong)
      .wrapping_add(SHELL_SIZE as libc::c_int as libc::c_ulong)
      & !(SHELL_SIZE as libc::c_int) as libc::c_ulong) as uint8_t,
    ((::std::mem::size_of::<ndup>() as libc::c_ulong)
      .wrapping_add(SHELL_SIZE as libc::c_int as libc::c_ulong)
      & !(SHELL_SIZE as libc::c_int) as libc::c_ulong) as uint8_t,
    ((::std::mem::size_of::<nhere>() as libc::c_ulong)
      .wrapping_add(SHELL_SIZE as libc::c_int as libc::c_ulong)
      & !(SHELL_SIZE as libc::c_int) as libc::c_ulong) as uint8_t,
    ((::std::mem::size_of::<nhere>() as libc::c_ulong)
      .wrapping_add(SHELL_SIZE as libc::c_int as libc::c_ulong)
      & !(SHELL_SIZE as libc::c_int) as libc::c_ulong) as uint8_t,
    ((::std::mem::size_of::<nnot>() as libc::c_ulong)
      .wrapping_add(SHELL_SIZE as libc::c_int as libc::c_ulong)
      & !(SHELL_SIZE as libc::c_int) as libc::c_ulong) as uint8_t,
  ]
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
/*-
 * Copyright (c) 1989, 1991, 1993, 1994
 *      The Regents of the University of California.  All rights reserved.
 *
 * This code is derived from software contributed to Berkeley by
 * Kenneth Almquist.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. Neither the name of the University nor the names of its contributors
 *    may be used to endorse or promote products derived from this software
 *    without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE REGENTS AND CONTRIBUTORS ''AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE REGENTS OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */
