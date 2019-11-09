use crate::librb::__syscall_slong_t;
use crate::librb::bb_uidgid_t;
use libc::sigval;
use libc::siginfo_t;
use crate::librb::size_t;
use crate::librb::smallint;
use c2rust_bitfields;
use c2rust_bitfields::BitfieldStruct;
use libc;
use libc::free;
use libc::gid_t;
use libc::mode_t;
use libc::off64_t;
use libc::off_t;
use libc::pid_t;
use libc::sigset_t;
use libc::ssize_t;
use libc::stat;
use libc::time_t;
use libc::timespec;
use libc::timeval;
use libc::uid_t;

extern "C" {
  pub type __dirstream;

  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;

  #[no_mangle]
  fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> *mut libc::c_char;

  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;

  #[no_mangle]
  static mut applet_name: *const libc::c_char;

  #[no_mangle]
  fn procps_scan(sp: *mut procps_status_t, flags: libc::c_int) -> *mut procps_status_t;

  #[no_mangle]
  fn bb_makedev(major: libc::c_uint, minor: libc::c_uint) -> libc::c_ulonglong;

  #[no_mangle]
  fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;

  #[no_mangle]
  fn gnu_dev_major(__dev: libc::dev_t) -> libc::c_uint;

  #[no_mangle]
  fn gnu_dev_minor(__dev: libc::dev_t) -> libc::c_uint;

  #[no_mangle]
  fn kill(__pid: pid_t, __sig: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;

  #[no_mangle]
  fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn sigprocmask(__how: libc::c_int, __set: *const sigset_t, __oset: *mut sigset_t) -> libc::c_int;

  #[no_mangle]
  fn sigtimedwait(
    __set: *const sigset_t,
    __info: *mut siginfo_t,
    __timeout: *const timespec,
  ) -> libc::c_int;

  #[no_mangle]
  fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;

  #[no_mangle]
  fn snprintf(
    _: *mut libc::c_char,
    _: libc::c_ulong,
    _: *const libc::c_char,
    _: ...
  ) -> libc::c_int;

  #[no_mangle]
  fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;

  #[no_mangle]
  fn atoll(__nptr: *const libc::c_char) -> libc::c_longlong;



  #[no_mangle]
  fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;

  #[no_mangle]
  fn putenv(__string: *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn system(__command: *const libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;

  #[no_mangle]
  fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;

  #[no_mangle]
  fn strchrnul(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;

  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;

  #[no_mangle]
  fn chmod(__file: *const libc::c_char, __mode: mode_t) -> libc::c_int;

  #[no_mangle]
  fn umask(__mask: mode_t) -> mode_t;

  #[no_mangle]
  fn mknod(__path: *const libc::c_char, __mode: mode_t, __dev: libc::dev_t) -> libc::c_int;

  #[no_mangle]
  fn gettimeofday(__tv: *mut timeval, __tz: __timezone_ptr_t) -> libc::c_int;

  #[no_mangle]
  fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;

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
  fn xrealloc_vector_helper(
    vector: *mut libc::c_void,
    sizeof_and_shift: libc::c_uint,
    idx: libc::c_int,
  ) -> *mut libc::c_void;

  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;

  #[no_mangle]
  fn xstrndup(s: *const libc::c_char, n: libc::c_int) -> *mut libc::c_char;

  #[no_mangle]
  fn xmemdup(s: *const libc::c_void, n: libc::c_int) -> *mut libc::c_void;

  #[no_mangle]
  fn recursive_action(
    fileName: *const libc::c_char,
    flags: libc::c_uint,
    fileAction_0: Option<
      unsafe extern "C" fn(
        _: *const libc::c_char,
        _: *mut stat,
        _: *mut libc::c_void,
        _: libc::c_int,
      ) -> libc::c_int,
    >,
    dirAction_0: Option<
      unsafe extern "C" fn(
        _: *const libc::c_char,
        _: *mut stat,
        _: *mut libc::c_void,
        _: libc::c_int,
      ) -> libc::c_int,
    >,
    userData: *mut libc::c_void,
    depth: libc::c_uint,
  ) -> libc::c_int;

  #[no_mangle]
  fn bb_copyfd_eof(fd1: libc::c_int, fd2: libc::c_int) -> off_t;

  #[no_mangle]
  fn bb_basename(name: *const libc::c_char) -> *const libc::c_char;

  #[no_mangle]
  fn endofname(name: *const libc::c_char) -> *const libc::c_char;

  #[no_mangle]
  fn is_prefixed_with(string: *const libc::c_char, key: *const libc::c_char) -> *mut libc::c_char;

  #[no_mangle]
  fn close_on_exec_on(fd: libc::c_int);

  #[no_mangle]
  fn xmove_fd(_: libc::c_int, _: libc::c_int);

  #[no_mangle]
  fn xchdir(path: *const libc::c_char);

  #[no_mangle]
  fn bb_unsetenv(key: *const libc::c_char);

  #[no_mangle]
  fn bb_unsetenv_and_free(key: *mut libc::c_char);

  #[no_mangle]
  fn xstat(pathname: *const libc::c_char, buf: *mut stat);

  #[no_mangle]
  fn xlseek(fd: libc::c_int, offset: off_t, whence: libc::c_int) -> off_t;

  #[no_mangle]
  fn strftime_HHMMSS(
    buf: *mut libc::c_char,
    len: libc::c_uint,
    tp: *mut time_t,
  ) -> *mut libc::c_char;

  #[no_mangle]
  fn create_and_bind_to_netlink(
    proto: libc::c_int,
    grp: libc::c_int,
    rcvbuf: libc::c_uint,
  ) -> libc::c_int;

  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;

  #[no_mangle]
  fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;

  #[no_mangle]
  fn open_read_close(
    filename: *const libc::c_char,
    buf: *mut libc::c_void,
    maxsz: size_t,
  ) -> ssize_t;

  #[no_mangle]
  fn full_write(fd: libc::c_int, buf: *const libc::c_void, count: size_t) -> ssize_t;

  #[no_mangle]
  fn xwrite_str(fd: libc::c_int, str: *const libc::c_char);

  #[no_mangle]
  fn fopen_for_read(path: *const libc::c_char) -> *mut FILE;

  #[no_mangle]
  fn utoa(n: libc::c_uint) -> *mut libc::c_char;

  #[no_mangle]
  fn get_uidgid(_: *mut bb_uidgid_t, _: *const libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn bb_daemonize_or_rexec(flags: libc::c_int);

  #[no_mangle]
  fn bb_sanitize_stdio();

  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;

  #[no_mangle]
  fn bb_show_usage() -> !;

  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);

  #[no_mangle]
  fn bb_simple_error_msg(s: *const libc::c_char);

  #[no_mangle]
  fn bb_perror_msg(s: *const libc::c_char, _: ...);

  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;

  #[no_mangle]
  fn bb_parse_mode(s: *const libc::c_char, cur_mode: libc::c_uint) -> libc::c_int;

  #[no_mangle]
  fn config_open2(
    filename: *const libc::c_char,
    fopen_func: Option<unsafe extern "C" fn(_: *const libc::c_char) -> *mut FILE>,
  ) -> *mut parser_t;

  #[no_mangle]
  fn config_read(
    parser: *mut parser_t,
    tokens: *mut *mut libc::c_char,
    flags: libc::c_uint,
    delims: *const libc::c_char,
  ) -> libc::c_int;

  #[no_mangle]
  fn config_close(parser: *mut parser_t);

  #[no_mangle]
  fn concat_path_file(
    path: *const libc::c_char,
    filename: *const libc::c_char,
  ) -> *mut libc::c_char;

  #[no_mangle]
  fn bb_make_directory(
    path: *mut libc::c_char,
    mode: libc::c_long,
    flags: libc::c_int,
  ) -> libc::c_int;

  #[no_mangle]
  fn index_in_strings(strings: *const libc::c_char, key: *const libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn chown(__file: *const libc::c_char, __owner: uid_t, __group: gid_t) -> libc::c_int;

  #[no_mangle]
  fn sleep(__seconds: libc::c_uint) -> libc::c_uint;

  #[no_mangle]
  fn pread(
    __fd: libc::c_int,
    __buf: *mut libc::c_void,
    __nbytes: size_t,
    __offset: off64_t,
  ) -> ssize_t;

  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn chdir(__path: *const libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn unlink(__name: *const libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn readlink(__path: *const libc::c_char, __buf: *mut libc::c_char, __len: size_t) -> ssize_t;

  #[no_mangle]
  fn symlink(__from: *const libc::c_char, __to: *const libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn getpid() -> pid_t;

  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];

  #[no_mangle]
  fn regexec(
    __preg: *const regex_t,
    __string: *const libc::c_char,
    __nmatch: size_t,
    __pmatch: *mut regmatch_t,
    __eflags: libc::c_int,
  ) -> libc::c_int;

  #[no_mangle]
  fn regfree(__preg: *mut regex_t);

  #[no_mangle]
  fn xregcomp(preg: *mut regex_t, regex: *const libc::c_char, cflags: libc::c_int);
}

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

pub type DIR = __dirstream;
pub type smalluint = libc::c_uchar;

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
  pub _pkey: u32,
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
  pub si_pid: pid_t,
  pub si_uid: uid_t,
  pub si_status: libc::c_int,
  pub si_utime: libc::clock_t,
  pub si_stime: libc::clock_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
  pub si_pid: pid_t,
  pub si_uid: uid_t,
  pub si_sigval: sigval,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
  pub si_tid: libc::c_int,
  pub si_overrun: libc::c_int,
  pub si_sigval: sigval,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
  pub si_pid: pid_t,
  pub si_uid: uid_t,
}

use libc::FILE;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct timezone {
  pub tz_minuteswest: libc::c_int,
  pub tz_dsttime: libc::c_int,
}

pub type __timezone_ptr_t = *mut timezone;

pub type C2RustUnnamed_9 = libc::c_int;
// pub const FILEUTILS_IGNORE_CHMOD_ERR: C2RustUnnamed_9 = -2147483648;
// pub const FILEUTILS_REFLINK_ALWAYS: C2RustUnnamed_9 = 262144;
// pub const FILEUTILS_REFLINK: C2RustUnnamed_9 = 131072;
// pub const FILEUTILS_RMDEST: C2RustUnnamed_9 = 32768;
// pub const FILEUTILS_NO_TARGET_DIR: C2RustUnnamed_9 = 16384;
// pub const FILEUTILS_UPDATE: C2RustUnnamed_9 = 8192;
// pub const FILEUTILS_VERBOSE: C2RustUnnamed_9 = 4096;
// pub const FILEUTILS_DEREFERENCE_L0: C2RustUnnamed_9 = 256;
// pub const FILEUTILS_DEREF_SOFTLINK: C2RustUnnamed_9 = 128;
// pub const FILEUTILS_MAKE_SOFTLINK: C2RustUnnamed_9 = 64;
// pub const FILEUTILS_MAKE_HARDLINK: C2RustUnnamed_9 = 32;
// pub const FILEUTILS_INTERACTIVE: C2RustUnnamed_9 = 16;
// pub const FILEUTILS_FORCE: C2RustUnnamed_9 = 8;
pub const FILEUTILS_RECUR: C2RustUnnamed_9 = 4;
// pub const FILEUTILS_DEREFERENCE: C2RustUnnamed_9 = 2;
// pub const FILEUTILS_PRESERVE_STATUS: C2RustUnnamed_9 = 1;

pub type C2RustUnnamed_10 = libc::c_uint;
// pub const ACTION_DANGLING_OK: C2RustUnnamed_10 = 64;
// pub const ACTION_QUIET: C2RustUnnamed_10 = 32;
// pub const ACTION_DEPTHFIRST: C2RustUnnamed_10 = 8;
// pub const ACTION_FOLLOWLINKS_L0: C2RustUnnamed_10 = 4;
pub const ACTION_FOLLOWLINKS: C2RustUnnamed_10 = 2;
pub const ACTION_RECURSE: C2RustUnnamed_10 = 1;

pub type C2RustUnnamed_11 = libc::c_uint;
pub const PARSE_NORMAL: C2RustUnnamed_11 = 4653056;
// pub const PARSE_WS_COMMENTS: C2RustUnnamed_11 = 16777216;
// pub const PARSE_ALT_COMMENTS: C2RustUnnamed_11 = 8388608;
pub const PARSE_EOL_COMMENTS: C2RustUnnamed_11 = 4194304;
// pub const PARSE_KEEP_COPY: C2RustUnnamed_11 = 2097152;
// pub const PARSE_MIN_DIE: C2RustUnnamed_11 = 1048576;
// pub const PARSE_GREEDY: C2RustUnnamed_11 = 262144;
// pub const PARSE_TRIM: C2RustUnnamed_11 = 131072;
// pub const PARSE_COLLAPSE: C2RustUnnamed_11 = 65536;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct parser_t {
  pub fp: *mut FILE,
  pub data: *mut libc::c_char,
  pub line: *mut libc::c_char,
  pub nline: *mut libc::c_char,
  pub line_alloc: size_t,
  pub nline_alloc: size_t,
  pub lineno: libc::c_int,
}

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
  pub shift_pages_to_bytes: u8,
  pub shift_pages_to_kb: u8,
  pub argv_len: u16,
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

pub type C2RustUnnamed_12 = libc::c_uint;
// pub const PSSCAN_TASKS: C2RustUnnamed_12 = 4194304;
// pub const PSSCAN_RUIDGID: C2RustUnnamed_12 = 2097152;
// pub const PSSCAN_NICE: C2RustUnnamed_12 = 1048576;
// pub const PSSCAN_CPU: C2RustUnnamed_12 = 524288;
// pub const PSSCAN_START_TIME: C2RustUnnamed_12 = 262144;
// pub const PSSCAN_CONTEXT: C2RustUnnamed_12 = 0;
// pub const PSSCAN_ARGVN: C2RustUnnamed_12 = 65536;
// pub const PSSCAN_SMAPS: C2RustUnnamed_12 = 32768;
// pub const PSSCAN_TTY: C2RustUnnamed_12 = 16384;
// pub const PSSCAN_UTIME: C2RustUnnamed_12 = 8192;
// pub const PSSCAN_STIME: C2RustUnnamed_12 = 4096;
// pub const PSSCAN_RSS: C2RustUnnamed_12 = 2048;
// pub const PSSCAN_VSZ: C2RustUnnamed_12 = 1024;
// pub const PSSCAN_STATE: C2RustUnnamed_12 = 512;
// pub const PSSCAN_EXE: C2RustUnnamed_12 = 256;
pub const PSSCAN_ARGV0: C2RustUnnamed_12 = 128;
// pub const PSSCAN_COMM: C2RustUnnamed_12 = 32;
// pub const PSSCAN_UIDGID: C2RustUnnamed_12 = 16;
// pub const PSSCAN_SID: C2RustUnnamed_12 = 8;
// pub const PSSCAN_PGID: C2RustUnnamed_12 = 4;
// pub const PSSCAN_PPID: C2RustUnnamed_12 = 2;
// pub const PSSCAN_PID: C2RustUnnamed_12 = 1;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub root_major: libc::c_int,
  pub root_minor: libc::c_int,
  pub verbose: smallint,
  pub subsystem: *mut libc::c_char,
  pub subsys_env: *mut libc::c_char,
  pub filename: *const libc::c_char,
  pub parser: *mut parser_t,
  pub rule_vec: *mut *mut rule,
  pub rule_idx: libc::c_uint,
  pub cur_rule: rule,
  pub timestr: [libc::c_char; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rule {
  pub keep_matching: bool,
  pub regex_compiled: bool,
  pub mode: mode_t,
  pub maj: libc::c_int,
  pub min0: libc::c_int,
  pub min1: libc::c_int,
  pub ugid: bb_uidgid_t,
  pub envvar: *mut libc::c_char,
  pub ren_mov: *mut libc::c_char,
  pub r_cmd: *mut libc::c_char,
  pub match_0: regex_t,
  pub envmatch: *mut envmatch,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct envmatch {
  pub next: *mut envmatch,
  pub envname: *mut libc::c_char,
  pub match_0: regex_t,
}
pub type regex_t = re_pattern_buffer;
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
pub type reg_syntax_t = libc::c_ulong;
pub type regoff_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regmatch_t {
  pub rm_so: regoff_t,
  pub rm_eo: regoff_t,
}
pub type C2RustUnnamed_13 = libc::c_uint;
pub const OP_remove: C2RustUnnamed_13 = 1;
pub const OP_add: C2RustUnnamed_13 = 0;
pub const MDEV_OPT_SCAN: C2RustUnnamed_14 = 1;
pub const MDEV_OPT_FOREGROUND: C2RustUnnamed_14 = 4;
pub const MDEV_OPT_DAEMON: C2RustUnnamed_14 = 2;
pub type C2RustUnnamed_14 = libc::c_uint;
static mut keywords: [libc::c_char; 12] = [97, 100, 100, 0, 114, 101, 109, 111, 118, 101, 0, 0];
unsafe extern "C" fn make_default_cur_rule() {
  memset(
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).cur_rule as *mut rule
      as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<rule>() as libc::c_ulong,
  ); /* "not a @major,minor rule" */
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .cur_rule
    .maj = -1i32;
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .cur_rule
    .mode = 0o660i32 as mode_t;
}
unsafe extern "C" fn clean_up_cur_rule() {
  let mut e: *mut envmatch = 0 as *mut envmatch;
  free(
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .cur_rule
      .envvar as *mut libc::c_void,
  );
  free(
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .cur_rule
      .ren_mov as *mut libc::c_void,
  );
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .cur_rule
    .regex_compiled
  {
    regfree(
      &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .cur_rule
        .match_0,
    );
  }
  free(
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .cur_rule
      .r_cmd as *mut libc::c_void,
  );
  e = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .cur_rule
    .envmatch;
  while !e.is_null() {
    free((*e).envname as *mut libc::c_void);
    regfree(&mut (*e).match_0);
    e = (*e).next
  }
  make_default_cur_rule();
}
unsafe extern "C" fn parse_envmatch_pfx(mut val: *mut libc::c_char) -> *mut libc::c_char {
  let mut nextp: *mut *mut envmatch = &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .cur_rule
    .envmatch;
  loop {
    let mut e: *mut envmatch = 0 as *mut envmatch;
    let mut semicolon: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut eq: *mut libc::c_char = strchr(val, '=' as i32);
    if eq.is_null() {
      /* || eq == val? */
      return val;
    }
    if endofname(val) != eq {
      return val;
    }
    semicolon = strchr(eq, ';' as i32);
    if semicolon.is_null() {
      return val;
    }
    /* ENVVAR=regex;... */
    e = xzalloc(::std::mem::size_of::<envmatch>() as libc::c_ulong) as *mut envmatch;
    *nextp = e;
    nextp = &mut (*e).next;
    (*e).envname = xstrndup(
      val,
      eq.wrapping_offset_from(val) as libc::c_long as libc::c_int,
    );
    *semicolon = '\u{0}' as i32 as libc::c_char;
    xregcomp(&mut (*e).match_0, eq.offset(1), 1i32);
    *semicolon = ';' as i32 as libc::c_char;
    val = semicolon.offset(1)
  }
}
unsafe extern "C" fn parse_next_rule() {
  let mut current_block: u64;
  loop
  /* Note: on entry, G.cur_rule is set to default */
  {
    let mut tokens: [*mut libc::c_char; 4] = [0 as *mut libc::c_char; 4]; /* while (config_read) */
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    /* No PARSE_EOL_COMMENTS, because command may contain '#' chars */
    if config_read(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).parser,
      tokens.as_mut_ptr(),
      (PARSE_NORMAL as libc::c_int & !(PARSE_EOL_COMMENTS as libc::c_int)
        | (3i32 & 0xffi32) << 8i32
        | 4i32 & 0xffi32) as libc::c_uint,
      b"# \t\x00" as *const u8 as *const libc::c_char,
    ) == 0
    {
      break;
    }
    /* Fields: [-]regex uid:gid mode [alias] [cmd] */
    /* 1st field */
    val = tokens[0]; /* swallow leading dash */
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .cur_rule
      .keep_matching = '-' as i32 == *val.offset(0) as libc::c_int;
    val = val.offset(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .cur_rule
        .keep_matching as libc::c_int as isize,
    );
    val = parse_envmatch_pfx(val);
    if *val.offset(0) as libc::c_int == '@' as i32 {
      /* @major,minor[-minor2] */
      /* (useful when name is ambiguous:
       * "/sys/class/usb/lp0" and
       * "/sys/class/printer/lp0")
       */
      let mut sc: libc::c_int = sscanf(
        val,
        b"@%u,%u-%u\x00" as *const u8 as *const libc::c_char,
        &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
          .cur_rule
          .maj as *mut libc::c_int,
        &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
          .cur_rule
          .min0 as *mut libc::c_int,
        &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
          .cur_rule
          .min1 as *mut libc::c_int,
      );
      if sc < 2i32
        || (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
          .cur_rule
          .maj
          < 0i32
      {
        bb_error_msg(
          b"bad @maj,min on line %d\x00" as *const u8 as *const libc::c_char,
          (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).parser).lineno,
        );
        current_block = 8958142635377293549;
      } else {
        if sc == 2i32 {
          (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
            .cur_rule
            .min1 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
            .cur_rule
            .min0
        }
        current_block = 11307063007268554308;
      }
    } else {
      let mut eq: *mut libc::c_char = strchr(val, '=' as i32);
      if *val.offset(0) as libc::c_int == '$' as i32 {
        /* $ENVVAR=regex ... */
        val = val.offset(1);
        if eq.is_null() {
          bb_error_msg(
            b"bad $envvar=regex on line %d\x00" as *const u8 as *const libc::c_char,
            (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).parser).lineno,
          );
          current_block = 8958142635377293549;
        } else {
          let ref mut fresh0 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
            .cur_rule
            .envvar;
          *fresh0 = xstrndup(
            val,
            eq.wrapping_offset_from(val) as libc::c_long as libc::c_int,
          );
          val = eq.offset(1);
          current_block = 224731115979188411;
        }
      } else {
        current_block = 224731115979188411;
      }
      match current_block {
        8958142635377293549 => {}
        _ => {
          xregcomp(
            &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
              .cur_rule
              .match_0,
            val,
            1i32,
          );
          (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
            .cur_rule
            .regex_compiled = 1i32 != 0;
          current_block = 11307063007268554308;
        }
      }
    }
    match current_block {
      11307063007268554308 =>
      /* 2nd field: uid:gid - device ownership */
      {
        if get_uidgid(
          &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
            .cur_rule
            .ugid,
          tokens[1],
        ) == 0i32
        {
          bb_error_msg(
            b"unknown user/group \'%s\' on line %d\x00" as *const u8 as *const libc::c_char,
            tokens[1],
            (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).parser).lineno,
          );
        } else {
          /* 3rd field: mode - device permissions */
          (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
            .cur_rule
            .mode = bb_parse_mode(
            tokens[2],
            (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
              .cur_rule
              .mode,
          ) as mode_t;
          /* 4th field (opt): ">|=alias" or "!" to not create the node */
          val = tokens[3];
          if 1i32 != 0
            && !val.is_null()
            && !strchr(
              b">=!\x00" as *const u8 as *const libc::c_char,
              *val.offset(0) as libc::c_int,
            )
            .is_null()
          {
            let mut s: *mut libc::c_char = skip_non_whitespace(val);
            let ref mut fresh1 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
              .cur_rule
              .ren_mov;
            *fresh1 = xstrndup(
              val,
              s.wrapping_offset_from(val) as libc::c_long as libc::c_int,
            );
            val = skip_whitespace(s)
          }
          if 1i32 != 0 && !val.is_null() && *val.offset(0) as libc::c_int != 0 {
            let mut s_0: *const libc::c_char = b"$@*\x00" as *const u8 as *const libc::c_char;
            let mut s2: *const libc::c_char = strchr(s_0, *val.offset(0) as libc::c_int);
            if s2.is_null() {
              bb_error_msg(
                b"bad line %u\x00" as *const u8 as *const libc::c_char,
                (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).parser).lineno,
              );
              current_block = 8958142635377293549;
            } else {
              let ref mut fresh2 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                .cur_rule
                .r_cmd;
              *fresh2 = xstrdup(val);
              current_block = 4090602189656566074;
            }
          } else {
            current_block = 4090602189656566074;
          }
          match current_block {
            8958142635377293549 => {}
            _ => return,
          }
        }
      }
      _ => {}
    }
    clean_up_cur_rule();
  }
  config_close((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).parser);
  let ref mut fresh3 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).parser;
  *fresh3 = 0 as *mut parser_t;
}
/* If mdev -s, we remember rules in G.rule_vec[].
 * Otherwise, there is no point in doing it, and we just
 * save only one parsed rule in G.cur_rule.
 */
unsafe extern "C" fn next_rule() -> *const rule {
  let mut rule: *mut rule = 0 as *mut rule;
  /* Open conf file if we didn't do it yet */
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .parser
    .is_null()
    && !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .filename
      .is_null()
  {
    let ref mut fresh4 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).parser;
    *fresh4 = config_open2(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).filename,
      Some(fopen_for_read as unsafe extern "C" fn(_: *const libc::c_char) -> *mut FILE),
    );
    let ref mut fresh5 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).filename;
    *fresh5 = 0 as *const libc::c_char
  }
  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .rule_vec
    .is_null()
  {
    /* mdev -s */
    /* Do we have rule parsed already? */
    if !(*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .rule_vec
      .offset((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).rule_idx as isize))
    .is_null()
    {
      let ref mut fresh6 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).rule_idx;
      let fresh7 = *fresh6;
      *fresh6 = (*fresh6).wrapping_add(1);
      return *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .rule_vec
        .offset(fresh7 as isize);
    }
    make_default_cur_rule();
  } else {
    /* not mdev -s */
    clean_up_cur_rule();
  }
  /* Parse one more rule if file isn't fully read */
  rule = &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).cur_rule;
  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .parser
    .is_null()
  {
    parse_next_rule();
    if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .rule_vec
      .is_null()
    {
      /* mdev -s */
      rule = xmemdup(
        &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).cur_rule as *mut rule
          as *const libc::c_void,
        ::std::mem::size_of::<rule>() as libc::c_ulong as libc::c_int,
      ) as *mut rule;
      let ref mut fresh8 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).rule_vec;
      *fresh8 = xrealloc_vector_helper(
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).rule_vec as *mut libc::c_void,
        ((::std::mem::size_of::<*mut rule>() as libc::c_ulong) << 8i32)
          .wrapping_add(4i32 as libc::c_ulong) as libc::c_uint,
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).rule_idx as libc::c_int,
      ) as *mut *mut rule;
      let ref mut fresh9 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).rule_idx;
      let fresh10 = *fresh9;
      *fresh9 = (*fresh9).wrapping_add(1);
      let ref mut fresh11 = *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .rule_vec
        .offset(fresh10 as isize);
      *fresh11 = rule
    }
  }
  return rule;
}
unsafe extern "C" fn env_matches(mut e: *mut envmatch) -> libc::c_int {
  while !e.is_null() {
    let mut r: libc::c_int = 0;
    let mut val: *mut libc::c_char = getenv((*e).envname);
    if val.is_null() {
      return 0i32;
    }
    r = regexec(
      &mut (*e).match_0,
      val,
      0i32 as size_t,
      0 as *mut regmatch_t,
      0i32,
    );
    if r != 0i32 {
      /* no match */
      return 0i32;
    }
    e = (*e).next
  }
  return 1i32;
}
unsafe extern "C" fn mkdir_recursive(mut name: *mut libc::c_char) {
  /* if name has many levels ("dir1/dir2"),
   * bb_make_directory() will create dir1 according to umask,
   * not according to its "mode" parameter.
   * Since we run with umask=0, need to temporarily switch it.
   */
  umask(0o22i32 as mode_t); /* "dir1" (if any) will be 0755 too */
  bb_make_directory(
    name,
    0o755i32 as libc::c_long,
    FILEUTILS_RECUR as libc::c_int,
  );
  umask(0i32 as mode_t);
}
/* Builds an alias path.
 * This function potentionally reallocates the alias parameter.
 * Only used for ENABLE_FEATURE_MDEV_RENAME
 */
unsafe extern "C" fn build_alias(
  mut alias: *mut libc::c_char,
  mut device_name: *const libc::c_char,
) -> *mut libc::c_char {
  let mut dest: *mut libc::c_char = 0 as *mut libc::c_char;
  /* ">bar/": rename to bar/device_name */
  /* ">bar[/]baz": rename to bar[/]baz */
  dest = strrchr(alias, '/' as i32);
  if !dest.is_null() {
    /* ">bar/[baz]" ? */
    *dest = '\u{0}' as i32 as libc::c_char; /* mkdir bar */
    mkdir_recursive(alias);
    *dest = '/' as i32 as libc::c_char;
    if *dest.offset(1) as libc::c_int == '\u{0}' as i32 {
      /* ">bar/" => ">bar/device_name" */
      dest = alias;
      alias = concat_path_file(alias, device_name);
      free(dest as *mut libc::c_void);
    }
  }
  return alias;
}
/* mknod in /dev based on a path like "/sys/block/hda/hda1"
 * NB1: path parameter needs to have SCRATCH_SIZE scratch bytes
 * after NUL, but we promise to not mangle it (IOW: to restore NUL if needed).
 * NB2: "mdev -s" may call us many times, do not leak memory/fds!
 *
 * device_name = $DEVNAME (may be NULL)
 * path        = /sys/$DEVPATH
 */
unsafe extern "C" fn make_device(
  mut device_name: *mut libc::c_char,
  mut path: *mut libc::c_char,
  mut operation: libc::c_int,
) {
  let mut major: libc::c_int = 0;
  let mut minor: libc::c_int = 0;
  let mut type_0: libc::c_int = 0;
  let mut len: libc::c_int = 0;
  let mut path_end: *mut libc::c_char = path.offset(strlen(path) as isize);
  /* Try to read major/minor string.  Note that the kernel puts \n after
   * the data, so we don't need to worry about null terminating the string
   * because sscanf() will stop at the first nondigit, which \n is.
   * We also depend on path having writeable space after it.
   */
  major = -1i32;
  if operation == OP_add as libc::c_int {
    strcpy(path_end, b"/dev\x00" as *const u8 as *const libc::c_char);
    len = open_read_close(
      path,
      path_end.offset(1) as *mut libc::c_void,
      (128i32 - 1i32) as size_t,
    ) as libc::c_int;
    *path_end = '\u{0}' as i32 as libc::c_char;
    if len < 1i32 {
      if 1i32 == 0 {
        return;
      }
    /* no "dev" file, but we can still run scripts
     * based on device name */
    } else if sscanf(
      path_end.offset(1),
      b"%u:%u\x00" as *const u8 as *const libc::c_char,
      &mut major as *mut libc::c_int,
      &mut minor as *mut libc::c_int,
    ) == 2i32
    {
      if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).verbose != 0 {
        bb_error_msg(
          b"dev %u,%u\x00" as *const u8 as *const libc::c_char,
          major,
          minor,
        );
      }
    } else {
      major = -1i32
    }
  }
  /* else: for delete, -1 still deletes the node, but < -1 suppresses that */
  /* Determine device name */
  if device_name.is_null() {
    /*
     * There was no $DEVNAME envvar (for example, mdev -s never has).
     * But it is very useful: it contains the *path*, not only basename,
     * Thankfully, uevent file has it.
     * Example of .../sound/card0/controlC0/uevent file on Linux-3.7.7:
     * MAJOR=116
     * MINOR=7
     * DEVNAME=snd/controlC0
     */
    strcpy(path_end, b"/uevent\x00" as *const u8 as *const libc::c_char);
    len = open_read_close(
      path,
      path_end.offset(1) as *mut libc::c_void,
      (128i32 - 1i32) as size_t,
    ) as libc::c_int;
    if len < 0i32 {
      len = 0i32
    }
    *path_end = '\u{0}' as i32 as libc::c_char;
    *path_end.offset((1i32 + len) as isize) = '\u{0}' as i32 as libc::c_char;
    device_name = strstr(
      path_end.offset(1),
      b"\nDEVNAME=\x00" as *const u8 as *const libc::c_char,
    );
    if !device_name.is_null() {
      device_name = device_name.offset(
        (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
          .wrapping_sub(1i32 as libc::c_ulong) as isize,
      );
      *strchrnul(device_name, '\n' as i32).offset(0) = '\u{0}' as i32 as libc::c_char
    } else {
      /* Fall back to just basename */
      device_name = bb_basename(path) as *mut libc::c_char
    }
  }
  /* Determine device type */
  /*
   * http://kernel.org/doc/pending/hotplug.txt says that only
   * "/sys/block/..." is for block devices. "/sys/bus" etc is not.
   * But since 2.6.25 block devices are also in /sys/class/block.
   * We use strstr("/block/") to forestall future surprises.
   */
  type_0 = 0o20000i32; /* restart from the beginning (think mdev -s) */
  if !strstr(path, b"/block/\x00" as *const u8 as *const libc::c_char).is_null()
    || !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .subsystem
      .is_null()
      && !is_prefixed_with(
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).subsystem,
        b"block\x00" as *const u8 as *const libc::c_char,
      )
      .is_null()
  {
    type_0 = 0o60000i32
  }
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).rule_idx = 0i32 as libc::c_uint;
  loop {
    let mut str_to_match: *const libc::c_char = 0 as *const libc::c_char;
    let mut off: [regmatch_t; 10] = [regmatch_t { rm_so: 0, rm_eo: 0 }; 10];
    let mut command: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut alias: *mut libc::c_char = 0 as *mut libc::c_char;
    /* this rule doesn't match */
    let mut aliaslink: libc::c_char = 0; /* for compiler */
    aliaslink = aliaslink;
    let mut node_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rule: *const rule = 0 as *const rule;
    str_to_match = device_name;
    rule = next_rule();
    if env_matches((*rule).envmatch) == 0 {
      continue;
    }
    if (*rule).maj >= 0i32 {
      /* @maj,min rule */
      if major != (*rule).maj {
        continue;
      }
      if minor < (*rule).min0 || minor > (*rule).min1 {
        continue;
      }
      memset(
        off.as_mut_ptr() as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<[regmatch_t; 10]>() as libc::c_ulong,
      );
    } else {
      if !(*rule).envvar.is_null() {
        /* $envvar=regex rule */
        str_to_match = getenv((*rule).envvar);
        if str_to_match.is_null() {
          continue;
        }
      }
      /* else: str_to_match = device_name */
      if (*rule).regex_compiled {
        let mut regex_match: libc::c_int = regexec(
          &(*rule).match_0,
          str_to_match,
          (::std::mem::size_of::<[regmatch_t; 10]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<regmatch_t>() as libc::c_ulong)
            as libc::c_uint as size_t,
          off.as_mut_ptr(),
          0i32,
        );
        //bb_error_msg("matches:");
        //for (int i = 0; i < ARRAY_SIZE(off); i++) {
        //	if (off[i].rm_so < 0) continue;
        //	bb_error_msg("match %d: '%.*s'\n", i,
        //		(int)(off[i].rm_eo - off[i].rm_so),
        //		device_name + off[i].rm_so);
        //}
        if regex_match != 0i32
          || off[0].rm_so != 0i32
          || off[0].rm_eo != strlen(str_to_match) as libc::c_int
        {
          continue;
        }
      }
    }
    /* else: it's final implicit "match-all" rule */
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).verbose as libc::c_int >= 2i32 {
      bb_error_msg(
        b"rule matched, line %d\x00" as *const u8 as *const libc::c_char,
        if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
          .parser
          .is_null()
        {
          (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).parser).lineno
        } else {
          -1i32
        },
      );
    }
    /* Build alias name */
    alias = 0 as *mut libc::c_char;
    if 1i32 != 0 && !(*rule).ren_mov.is_null() {
      aliaslink = *(*rule).ren_mov.offset(0);
      if aliaslink as libc::c_int == '!' as i32 {
        /* "!": suppress node creation/deletion */
        major = -2i32
      } else if aliaslink as libc::c_int == '>' as i32 || aliaslink as libc::c_int == '=' as i32 {
        let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut n: libc::c_uint = 0;
        /* substitute %1..9 with off[1..9], if any */
        n = 0i32 as libc::c_uint;
        s = (*rule).ren_mov;
        while *s != 0 {
          let fresh12 = s;
          s = s.offset(1);
          if *fresh12 as libc::c_int == '%' as i32 {
            n = n.wrapping_add(1)
          }
        }
        alias = xzalloc(
          strlen((*rule).ren_mov)
            .wrapping_add((n as libc::c_ulong).wrapping_mul(strlen(str_to_match))),
        ) as *mut libc::c_char;
        p = alias;
        s = (*rule).ren_mov.offset(1);
        while *s != 0 {
          *p = *s;
          if '%' as i32 == *s as libc::c_int {
            let mut i: libc::c_uint = (*s.offset(1) as libc::c_int - '0' as i32) as libc::c_uint;
            if i <= 9i32 as libc::c_uint && off[i as usize].rm_so >= 0i32 {
              n = (off[i as usize].rm_eo - off[i as usize].rm_so) as libc::c_uint;
              strncpy(
                p,
                str_to_match.offset(off[i as usize].rm_so as isize),
                n as libc::c_ulong,
              );
              p = p.offset(n.wrapping_sub(1i32 as libc::c_uint) as isize);
              s = s.offset(1)
            }
          }
          p = p.offset(1);
          s = s.offset(1)
        }
      }
    }
    command = 0 as *mut libc::c_char;
    command = (*rule).r_cmd;
    if !command.is_null() {
      /* Are we running this command now?
       * Run @cmd on create, $cmd on delete, *cmd on any
       */
      if *command.offset(0) as libc::c_int == '@' as i32 && operation == OP_add as libc::c_int
        || *command.offset(0) as libc::c_int == '$' as i32 && operation == OP_remove as libc::c_int
        || *command.offset(0) as libc::c_int == '*' as i32
      {
        command = command.offset(1)
      } else {
        command = 0 as *mut libc::c_char
      }
    }
    /* "Execute" the line we found */
    node_name = device_name;
    if 1i32 != 0 && !alias.is_null() {
      alias = build_alias(alias, device_name);
      node_name = alias
    }
    if operation == OP_add as libc::c_int && major >= 0i32 {
      let mut slash: *mut libc::c_char = strrchr(node_name, '/' as i32);
      if !slash.is_null() {
        *slash = '\u{0}' as i32 as libc::c_char;
        mkdir_recursive(node_name);
        *slash = '/' as i32 as libc::c_char
      }
      if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).verbose != 0 {
        bb_error_msg(
          b"mknod %s (%d,%d) %o %u:%u\x00" as *const u8 as *const libc::c_char,
          node_name,
          major,
          minor,
          (*rule).mode | type_0 as libc::c_uint,
          (*rule).ugid.uid,
          (*rule).ugid.gid,
        );
      }
      if mknod(
        node_name,
        (*rule).mode | type_0 as libc::c_uint,
        bb_makedev(major as libc::c_uint, minor as libc::c_uint) as libc::dev_t,
      ) != 0
        && *bb_errno != 17i32
      {
        bb_perror_msg(
          b"can\'t create \'%s\'\x00" as *const u8 as *const libc::c_char,
          node_name,
        );
      }
      chmod(node_name, (*rule).mode);
      chown(node_name, (*rule).ugid.uid, (*rule).ugid.gid);
      if major == (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).root_major
        && minor == (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).root_minor
      {
        symlink(node_name, b"root\x00" as *const u8 as *const libc::c_char);
      }
      if 1i32 != 0 && !alias.is_null() {
        if aliaslink as libc::c_int == '>' as i32 {
          //TODO: on devtmpfs, device_name already exists and symlink() fails.
          //End result is that instead of symlink, we have two nodes.
          //What should be done?
          if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).verbose != 0 {
            bb_error_msg(
              b"symlink: %s\x00" as *const u8 as *const libc::c_char,
              device_name,
            );
          }
          symlink(node_name, device_name);
        }
      }
    }
    if 1i32 != 0 && !command.is_null() {
      /* setenv will leak memory, use putenv/unsetenv/free */
      let mut s_0: *mut libc::c_char = xasprintf(
        b"%s=%s\x00" as *const u8 as *const libc::c_char,
        b"MDEV\x00" as *const u8 as *const libc::c_char,
        node_name,
      );
      putenv(s_0);
      if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).verbose != 0 {
        bb_error_msg(
          b"running: %s\x00" as *const u8 as *const libc::c_char,
          command,
        );
      }
      if system(command) == -1i32 {
        bb_perror_msg(
          b"can\'t run \'%s\'\x00" as *const u8 as *const libc::c_char,
          command,
        );
      }
      bb_unsetenv_and_free(s_0);
    }
    if operation == OP_remove as libc::c_int && major >= -1i32 {
      if 1i32 != 0 && !alias.is_null() {
        if aliaslink as libc::c_int == '>' as i32 {
          if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).verbose != 0 {
            bb_error_msg(
              b"unlink: %s\x00" as *const u8 as *const libc::c_char,
              device_name,
            );
          }
          unlink(device_name);
        }
      }
      if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).verbose != 0 {
        bb_error_msg(
          b"unlink: %s\x00" as *const u8 as *const libc::c_char,
          node_name,
        );
      }
      unlink(node_name);
    }
    free(alias as *mut libc::c_void);
    if 1i32 == 0 || !(*rule).keep_matching
    /* We found matching line.
     * Stop unless it was prefixed with '-'
     */
    {
      break;
    }
  }
  /* for (;;) */
}
unsafe extern "C" fn readlink2(mut buf: *mut libc::c_char, mut bufsize: size_t) -> ssize_t {
  // Grr... gcc 8.1.1:
  // "passing argument 2 to restrict-qualified parameter aliases with argument 1"
  // dance around that...
  let mut obuf: *mut libc::c_char = 0 as *mut libc::c_char;
  obuf = buf;
  return readlink(buf, obuf, bufsize);
}
/* File callback for /sys/ traversal.
 * We act only on "/sys/.../dev" (pseudo)file
 */
unsafe extern "C" fn fileAction(
  mut fileName: *const libc::c_char,
  mut _statbuf: *mut stat,
  mut userData: *mut libc::c_void,
  mut _depth: libc::c_int,
) -> libc::c_int {
  let mut len: size_t = strlen(fileName).wrapping_sub(4i32 as libc::c_ulong); /* can't underflow */
  let mut path: *mut libc::c_char = userData as *mut libc::c_char; /* char array[PATH_MAX + SCRATCH_SIZE] */
  let mut subsys: [libc::c_char; 4096] = [0; 4096];
  let mut res: libc::c_int = 0;
  /* Is it a ".../dev" file? (len check is for paranoid reasons) */
  if strcmp(
    fileName.offset(len as isize),
    b"/dev\x00" as *const u8 as *const libc::c_char,
  ) != 0i32
    || len >= (4096i32 - 32i32) as libc::c_ulong
  {
    return 0i32;
  } /* not .../dev */
  strcpy(path, fileName);
  *path.offset(len as isize) = '\u{0}' as i32 as libc::c_char;
  /* Read ".../subsystem" symlink in the same directory where ".../dev" is */
  strcpy(subsys.as_mut_ptr(), path);
  strcpy(
    subsys.as_mut_ptr().offset(len as isize),
    b"/subsystem\x00" as *const u8 as *const libc::c_char,
  );
  res = readlink2(
    subsys.as_mut_ptr(),
    (::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong)
      .wrapping_sub(1i32 as libc::c_ulong),
  ) as libc::c_int;
  if res > 0i32 {
    subsys[res as usize] = '\u{0}' as i32 as libc::c_char;
    free((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).subsystem as *mut libc::c_void);
    if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .subsys_env
      .is_null()
    {
      bb_unsetenv_and_free((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).subsys_env);
      let ref mut fresh13 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).subsys_env;
      *fresh13 = 0 as *mut libc::c_char
    }
    /* Set G.subsystem and $SUBSYSTEM from symlink's last component */
    let ref mut fresh14 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).subsystem;
    *fresh14 = strrchr(subsys.as_mut_ptr(), '/' as i32);
    if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .subsystem
      .is_null()
    {
      let ref mut fresh15 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).subsystem;
      *fresh15 = xstrdup(
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
          .subsystem
          .offset(1),
      );
      let ref mut fresh16 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).subsys_env;
      *fresh16 = xasprintf(
        b"%s=%s\x00" as *const u8 as *const libc::c_char,
        b"SUBSYSTEM\x00" as *const u8 as *const libc::c_char,
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).subsystem,
      );
      putenv((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).subsys_env);
    }
  }
  make_device(0 as *mut libc::c_char, path, OP_add as libc::c_int);
  return 1i32;
}
/* Directory callback for /sys/ traversal */
unsafe extern "C" fn dirAction(
  mut _fileName: *const libc::c_char,
  mut _statbuf: *mut stat,
  mut _userData: *mut libc::c_void,
  mut depth: libc::c_int,
) -> libc::c_int {
  return if depth >= 3i32 { 2i32 } else { 1i32 };
}
/* For the full gory details, see linux/Documentation/firmware_class/README
 *
 * Firmware loading works like this:
 * - kernel sets FIRMWARE env var
 * - userspace checks /lib/firmware/$FIRMWARE
 * - userspace waits for /sys/$DEVPATH/loading to appear
 * - userspace writes "1" to /sys/$DEVPATH/loading
 * - userspace copies /lib/firmware/$FIRMWARE into /sys/$DEVPATH/data
 * - userspace writes "0" (worked) or "-1" (failed) to /sys/$DEVPATH/loading
 * - kernel loads firmware into device
 */
unsafe extern "C" fn load_firmware(
  mut firmware: *const libc::c_char,
  mut sysfs_path: *const libc::c_char,
) {
  let mut current_block: u64;
  let mut cnt: libc::c_int = 0;
  let mut firmware_fd: libc::c_int = 0;
  let mut loading_fd: libc::c_int = 0;
  /* check for /lib/firmware/$FIRMWARE */
  firmware_fd = -1i32; /* can fail */
  if chdir(b"/lib/firmware\x00" as *const u8 as *const libc::c_char) == 0i32 {
    firmware_fd = open(firmware, 0i32)
  }
  /* check for /sys/$DEVPATH/loading ... give 30 seconds to appear */
  xchdir(sysfs_path);
  cnt = 0i32;
  loop {
    if !(cnt < 30i32) {
      current_block = 128850887951311672;
      break;
    }
    loading_fd = open(b"loading\x00" as *const u8 as *const libc::c_char, 0o1i32);
    if loading_fd >= 0i32 {
      current_block = 8596762549756051270;
      break;
    }
    sleep(1i32 as libc::c_uint);
    cnt += 1
  }
  match current_block {
    8596762549756051270 => {
      cnt = 0i32;
      if firmware_fd >= 0i32 {
        let mut data_fd: libc::c_int = 0;
        /* tell kernel we're loading by "echo 1 > /sys/$DEVPATH/loading" */
        if full_write(
          loading_fd,
          b"1\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
          1i32 as size_t,
        ) != 1
        {
          current_block = 128850887951311672;
        } else {
          /* load firmware into /sys/$DEVPATH/data */
          data_fd = open(b"data\x00" as *const u8 as *const libc::c_char, 0o1i32);
          if data_fd < 0i32 {
            current_block = 128850887951311672;
          } else {
            cnt = bb_copyfd_eof(firmware_fd, data_fd) as libc::c_int;
            current_block = 12124785117276362961;
          }
        }
      } else {
        current_block = 12124785117276362961;
      }
      match current_block {
        128850887951311672 => {}
        _ => {
          /* Tell kernel result by "echo [0|-1] > /sys/$DEVPATH/loading"
           * Note: we emit -1 also if firmware file wasn't found.
           * There are cases when otherwise kernel would wait for minutes
           * before timing out.
           */
          if cnt > 0i32 {
            full_write(
              loading_fd,
              b"0\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
              1i32 as size_t,
            );
          } else {
            full_write(
              loading_fd,
              b"-1\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
              2i32 as size_t,
            );
          }
        }
      }
    }
    _ => {}
  }
  xchdir(b"/dev\x00" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn curtime() -> *mut libc::c_char {
  let mut tv: timeval = timeval {
    tv_sec: 0,
    tv_usec: 0,
  };
  gettimeofday(&mut tv, 0 as *mut timezone);
  sprintf(
    strftime_HHMMSS(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .timestr
        .as_mut_ptr(),
      ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as libc::c_uint,
      &mut tv.tv_sec,
    ),
    b".%06u\x00" as *const u8 as *const libc::c_char,
    tv.tv_usec as libc::c_uint,
  );
  return (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .timestr
    .as_mut_ptr();
}
unsafe extern "C" fn open_mdev_log(mut seq: *const libc::c_char, mut my_pid: libc::c_uint) {
  let mut logfd: libc::c_int = open(
    b"mdev.log\x00" as *const u8 as *const libc::c_char,
    0o1i32 | 0o2000i32,
  );
  if logfd >= 0i32 {
    xmove_fd(logfd, 2i32);
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).verbose = 2i32 as smallint;
    applet_name = xasprintf(
      b"%s[%s]\x00" as *const u8 as *const libc::c_char,
      applet_name,
      if !seq.is_null() { seq } else { utoa(my_pid) },
    )
  };
}
/* If it exists, does /dev/mdev.seq match $SEQNUM?
 * If it does not match, earlier mdev is running
 * in parallel, and we need to wait.
 * Active mdev pokes us with SIGCHLD to check the new file.
 */
unsafe extern "C" fn wait_for_seqfile(mut expected_seq: libc::c_uint) -> libc::c_int {
  /* We time out after 2 sec */
  static mut ts: timespec = {
    let mut init = timespec {
      tv_sec: 0i32 as time_t,
      tv_nsec: (32i32 * 1000i32 * 1000i32) as __syscall_slong_t,
    };
    init
  };
  let mut timeout: libc::c_int = 2000i32 / 32i32;
  let mut seq_fd: libc::c_int = -1i32;
  let mut do_once: libc::c_int = 1i32;
  let mut set_CHLD: sigset_t = std::mem::zeroed();
  sigemptyset(&mut set_CHLD);
  sigaddset(&mut set_CHLD, 17i32);
  sigprocmask(0i32, &mut set_CHLD, 0 as *mut sigset_t);
  loop {
    let mut seqlen: libc::c_int = 0;
    let mut seqbuf: [libc::c_char; 26] = [0; 26];
    let mut seqbufnum: libc::c_uint = 0;
    if seq_fd < 0i32 {
      seq_fd = open(b"mdev.seq\x00" as *const u8 as *const libc::c_char, 0o2i32);
      if seq_fd < 0i32 {
        break;
      }
      close_on_exec_on(seq_fd);
    }
    seqlen = pread(
      seq_fd,
      seqbuf.as_mut_ptr() as *mut libc::c_void,
      (::std::mem::size_of::<[libc::c_char; 26]>() as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong),
      0i32 as off64_t,
    ) as libc::c_int;
    if seqlen < 0i32 {
      close(seq_fd);
      seq_fd = -1i32;
      break;
    } else {
      seqbuf[seqlen as usize] = '\u{0}' as i32 as libc::c_char;
      if seqbuf[0] as libc::c_int == '\n' as i32 || seqbuf[0] as libc::c_int == '\u{0}' as i32 {
        /* don't decrement timeout! */
        /* seed file: write out seq ASAP */
        xwrite_str(seq_fd, utoa(expected_seq));
        xlseek(seq_fd, 0i32 as off_t, 0i32);
        if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).verbose as libc::c_int >= 2i32 {
          bb_simple_error_msg(b"first seq written\x00" as *const u8 as *const libc::c_char);
        }
        break;
      } else {
        seqbufnum = atoll(seqbuf.as_mut_ptr()) as libc::c_uint;
        if seqbufnum == expected_seq {
          break;
        }
        if seqbufnum > expected_seq {
          /* a later mdev runs already (this was seen by users to happen) */
          /* do not overwrite seqfile on exit */
          close(seq_fd); /* for compiler */
          seq_fd = -1i32;
          break;
        } else {
          if do_once != 0 {
            if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).verbose as libc::c_int >= 2i32 {
              bb_error_msg(
                b"%s mdev.seq=\'%s\', need \'%u\'\x00" as *const u8 as *const libc::c_char,
                curtime(),
                seqbuf.as_mut_ptr(),
                expected_seq,
              );
            }
            do_once = 0i32
          }
          if sigtimedwait(&mut set_CHLD, 0 as *mut siginfo_t, &ts) >= 0i32 {
            continue;
          }
          timeout -= 1;
          if !(timeout == 0i32) {
            continue;
          }
          if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).verbose != 0 {
            bb_error_msg(
              b"%s mdev.seq=\'%s\'\x00" as *const u8 as *const libc::c_char,
              b"timed out\x00" as *const u8 as *const libc::c_char,
              seqbuf.as_mut_ptr(),
            );
          }
          break;
        }
      }
    }
  }
  sigprocmask(1i32, &mut set_CHLD, 0 as *mut sigset_t);
  return seq_fd;
}
unsafe extern "C" fn signal_mdevs(mut my_pid: libc::c_uint) {
  let mut p: *mut procps_status_t = 0 as *mut procps_status_t;
  loop {
    p = procps_scan(p, PSSCAN_ARGV0 as libc::c_int);
    if p.is_null() {
      break;
    }
    if (*p).pid != my_pid
      && !(*p).argv0.is_null()
      && strcmp(
        bb_basename((*p).argv0),
        b"mdev\x00" as *const u8 as *const libc::c_char,
      ) == 0i32
    {
      kill((*p).pid as pid_t, 17i32);
    }
  }
}
unsafe extern "C" fn process_action(mut temp: *mut libc::c_char, mut my_pid: libc::c_uint) {
  let mut fw: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut seq: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut action: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut env_devname: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut env_devpath: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut seqnum: libc::c_uint = 0;
  seqnum = seqnum;
  let mut seq_fd: libc::c_int = 0;
  let mut op: smalluint = 0;
  /* Hotplug:
   * env ACTION=... DEVPATH=... SUBSYSTEM=... [SEQNUM=...] mdev
   * ACTION can be "add", "remove", "change"
   * DEVPATH is like "/block/sda" or "/class/input/mice"
   */
  env_devname = getenv(b"DEVNAME\x00" as *const u8 as *const libc::c_char); /* can be NULL */
  let ref mut fresh17 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).subsystem;
  *fresh17 = getenv(b"SUBSYSTEM\x00" as *const u8 as *const libc::c_char);
  action = getenv(b"ACTION\x00" as *const u8 as *const libc::c_char);
  env_devpath = getenv(b"DEVPATH\x00" as *const u8 as *const libc::c_char);
  if action.is_null() || env_devpath.is_null() {
    /*|| !G.subsystem*/
    bb_show_usage();
  }
  fw = getenv(b"FIRMWARE\x00" as *const u8 as *const libc::c_char);
  seq = getenv(b"SEQNUM\x00" as *const u8 as *const libc::c_char);
  op = index_in_strings(keywords.as_ptr(), action) as smalluint;
  if my_pid != 0 {
    open_mdev_log(seq, my_pid);
  }
  seq_fd = -1i32;
  if my_pid != 0 && !seq.is_null() {
    seqnum = atoll(seq) as libc::c_uint;
    seq_fd = wait_for_seqfile(seqnum)
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).verbose != 0 {
    bb_error_msg(
      b"%s ACTION:%s SEQNUM:%s SUBSYSTEM:%s DEVNAME:%s DEVPATH:%s%s%s\x00" as *const u8
        as *const libc::c_char,
      curtime(),
      action,
      seq,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).subsystem,
      env_devname,
      env_devpath,
      if !fw.is_null() {
        b" FW:\x00" as *const u8 as *const libc::c_char
      } else {
        b"\x00" as *const u8 as *const libc::c_char
      },
      if !fw.is_null() {
        fw
      } else {
        b"\x00" as *const u8 as *const libc::c_char
      },
    );
  }
  snprintf(
    temp,
    4096i32 as libc::c_ulong,
    b"/sys%s\x00" as *const u8 as *const libc::c_char,
    env_devpath,
  );
  if op as libc::c_int == OP_remove as libc::c_int {
    /* Ignoring "remove firmware". It was reported
     * to happen and to cause erroneous deletion
     * of device nodes. */
    if fw.is_null() {
      make_device(env_devname, temp, op as libc::c_int);
    }
  } else {
    make_device(env_devname, temp, op as libc::c_int);
    if op as libc::c_int == OP_add as libc::c_int && !fw.is_null() {
      load_firmware(fw, temp);
    }
  }
  if seq_fd >= 0i32 {
    xwrite_str(seq_fd, utoa(seqnum.wrapping_add(1i32 as libc::c_uint)));
    signal_mdevs(my_pid);
  };
}
unsafe extern "C" fn initial_scan(mut temp: *mut libc::c_char) {
  let mut st: stat = std::mem::zeroed();
  xstat(b"/\x00" as *const u8 as *const libc::c_char, &mut st);
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).root_major =
    gnu_dev_major(st.st_dev) as libc::c_int;
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).root_minor =
    gnu_dev_minor(st.st_dev) as libc::c_int;
  putenv(b"ACTION=add\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
  /* Create all devices from /sys/dev hierarchy */
  recursive_action(
    b"/sys/dev\x00" as *const u8 as *const libc::c_char,
    (ACTION_RECURSE as libc::c_int | ACTION_FOLLOWLINKS as libc::c_int) as libc::c_uint,
    Some(
      fileAction
        as unsafe extern "C" fn(
          _: *const libc::c_char,
          _: *mut stat,
          _: *mut libc::c_void,
          _: libc::c_int,
        ) -> libc::c_int,
    ),
    Some(
      dirAction
        as unsafe extern "C" fn(
          _: *const libc::c_char,
          _: *mut stat,
          _: *mut libc::c_void,
          _: libc::c_int,
        ) -> libc::c_int,
    ),
    temp as *mut libc::c_void,
    0i32 as libc::c_uint,
  );
}
unsafe extern "C" fn daemon_loop(mut temp: *mut libc::c_char, mut fd: libc::c_int) {
  loop {
    let mut netbuf: [libc::c_char; 2048] = [0; 2048];
    let mut env: [*mut libc::c_char; 32] = [0 as *mut libc::c_char; 32];
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: ssize_t = 0;
    let mut idx: libc::c_int = 0;
    len = safe_read(
      fd,
      netbuf.as_mut_ptr() as *mut libc::c_void,
      (::std::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong),
    );
    if len < 0 {
      bb_simple_perror_msg_and_die(b"read\x00" as *const u8 as *const libc::c_char);
    }
    end = netbuf.as_mut_ptr().offset(len as isize);
    *end = '\u{0}' as i32 as libc::c_char;
    idx = 0i32;
    s = netbuf.as_mut_ptr();
    while s < end && idx < 32i32 {
      if *endofname(s).offset(0) as libc::c_int == '=' as i32 {
        let fresh18 = idx;
        idx = idx + 1;
        env[fresh18 as usize] = s;
        putenv(s);
      }
      s = s.offset(strlen(s).wrapping_add(1i32 as libc::c_ulong) as isize)
    }
    process_action(temp, 0i32 as libc::c_uint);
    while idx != 0 {
      idx -= 1;
      bb_unsetenv(env[idx as usize]);
    }
  }
}
#[no_mangle]
pub unsafe extern "C" fn mdev_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut opt: libc::c_int = 0;
  let mut temp: *mut libc::c_char = xmalloc((4096i32 + 128i32) as size_t) as *mut libc::c_char;
  /* We can be called as hotplug helper */
  /* Kernel cannot provide suitable stdio fds for us, do it ourself */
  bb_sanitize_stdio();
  /* Force the configuration file settings exactly */
  umask(0i32 as mode_t);
  xchdir(b"/dev\x00" as *const u8 as *const libc::c_char);
  opt = getopt32(argv, b"sdf\x00" as *const u8 as *const libc::c_char) as libc::c_int;
  let ref mut fresh19 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).filename;
  *fresh19 = b"/etc/mdev.conf\x00" as *const u8 as *const libc::c_char;
  if opt & (MDEV_OPT_SCAN as libc::c_int | MDEV_OPT_DAEMON as libc::c_int) != 0 {
    /* Same as xrealloc_vector(NULL, 4, 0): */
    let ref mut fresh20 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).rule_vec;
    *fresh20 = xzalloc(
      ((1i32 << 4i32) as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<*mut rule>() as libc::c_ulong),
    ) as *mut *mut rule
  }
  if opt & MDEV_OPT_DAEMON as libc::c_int != 0 {
    /*
     * Daemon mode listening on uevent netlink socket.
     */
    let mut fd: libc::c_int = 0;
    /* Subscribe for UEVENT kernel messages */
    /* Without a sufficiently big RCVBUF, a ton of simultaneous events
     * can trigger ENOBUFS on read, which is unrecoverable.
     * Reproducer:
     *	mdev -d
     *	find /sys -name uevent -exec sh -c 'echo add >"{}"' ';'
     */
    fd = create_and_bind_to_netlink(
      15i32,
      1i32 << 0i32,
      (2i32 * 1024i32 * 1024i32) as libc::c_uint,
    );
    /*
     * Make inital scan after the uevent socket is alive and
     * _before_ we fork away.
     */
    initial_scan(temp);
    if opt & MDEV_OPT_FOREGROUND as libc::c_int == 0 {
      bb_daemonize_or_rexec(0i32);
    }
    open_mdev_log(0 as *const libc::c_char, getpid() as libc::c_uint);
    daemon_loop(temp, fd);
  }
  if opt & MDEV_OPT_SCAN as libc::c_int != 0 {
    /*
     * Scan: mdev -s
     */
    initial_scan(temp);
  } else {
    process_action(temp, getpid() as libc::c_uint);
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).verbose != 0 {
      bb_error_msg(
        b"%s exiting\x00" as *const u8 as *const libc::c_char,
        curtime(),
      );
    }
  }
  return 0i32;
}
