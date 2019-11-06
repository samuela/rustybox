use libc;
extern "C" {
  pub type sockaddr_x25;
  pub type sockaddr_ns;
  pub type sockaddr_iso;
  pub type sockaddr_ipx;
  pub type sockaddr_inarp;
  pub type sockaddr_eon;
  pub type sockaddr_dl;
  pub type sockaddr_ax25;
  pub type sockaddr_at;
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
  #[no_mangle]
  fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
  #[no_mangle]
  fn geteuid() -> __uid_t;
  #[no_mangle]
  fn unlink(__name: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
  #[no_mangle]
  fn sendto(
    __fd: libc::c_int,
    __buf: *const libc::c_void,
    __n: size_t,
    __flags: libc::c_int,
    __addr: __CONST_SOCKADDR_ARG,
    __addr_len: socklen_t,
  ) -> ssize_t;
  #[no_mangle]
  fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
  #[no_mangle]
  static mut stdin: *mut FILE;
  #[no_mangle]
  fn rename(__old: *const libc::c_char, __new: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
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
  fn fgets_unlocked(
    __s: *mut libc::c_char,
    __n: libc::c_int,
    __stream: *mut FILE,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strchrnul(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
  #[no_mangle]
  fn chmod(__file: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
  #[no_mangle]
  fn time(__timer: *mut time_t) -> time_t;
  #[no_mangle]
  fn ctime(__timer: *const time_t) -> *mut libc::c_char;
  /* Some useful definitions */
  /* Macros for min/max.  */
  /* buffer allocation schemes */
  /* glibc uses __errno_location() to get a ptr to errno */
  /* We can just memorize it once - no multithreading in busybox :) */
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn monotonic_sec() -> libc::c_uint;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xrealloc(old: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn device_open(device: *const libc::c_char, mode: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xmove_fd(_: libc::c_int, _: libc::c_int);
  #[no_mangle]
  fn xmalloc_follow_symlinks(path: *const libc::c_char) -> *mut libc::c_char;
  /* syscalls like read() will be interrupted with EINTR: */
  #[no_mangle]
  fn signal_no_SA_RESTART_empty_mask(
    sig: libc::c_int,
    handler: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
  );
  #[no_mangle]
  fn kill_myself_with_sig(sig: libc::c_int) -> !;
  /* Standard handler which just records signo */
  #[no_mangle]
  static mut bb_got_signal: smallint;
  #[no_mangle]
  fn record_signo(signo: libc::c_int);
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xsocket(domain: libc::c_int, type_0: libc::c_int, protocol: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xbind(sockfd: libc::c_int, my_addr: *mut sockaddr, addrlen: socklen_t);
  /* Return malloc'ed len_and_sockaddr with socket address of host:port
   * Currently will return IPv4 or IPv6 sockaddrs only
   * (depending on host), but in theory nothing prevents e.g.
   * UNIX socket address being returned, IPX sockaddr etc...
   * On error does bb_error_msg and returns NULL */
  #[no_mangle]
  fn host2sockaddr(host: *const libc::c_char, port: libc::c_int) -> *mut len_and_sockaddr;
  #[no_mangle]
  fn safe_strncpy(
    dst: *mut libc::c_char,
    src: *const libc::c_char,
    size: size_t,
  ) -> *mut libc::c_char;
  // NB: will return short write on error, not -1,
  // if some data was written before error occurred
  #[no_mangle]
  fn full_write(fd: libc::c_int, buf: *const libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn fopen_for_read(path: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn xfopen_for_read(path: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn safe_gethostname() -> *mut libc::c_char;
  #[no_mangle]
  fn xatoull_range(
    str: *const libc::c_char,
    l: libc::c_ulonglong,
    u: libc::c_ulonglong,
  ) -> libc::c_ulonglong;
  #[no_mangle]
  fn xatou_range(str: *const libc::c_char, l: libc::c_uint, u: libc::c_uint) -> libc::c_uint;
  #[no_mangle]
  fn bb_strtou(
    arg: *const libc::c_char,
    endp: *mut *mut libc::c_char,
    base: libc::c_int,
  ) -> libc::c_uint;
  #[no_mangle]
  fn uid2uname_utoa(uid: uid_t) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_daemonize_or_rexec(flags: libc::c_int);
  /* { "-", NULL } */
  #[no_mangle]
  static mut option_mask32: u32;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  fn llist_add_to(old_head: *mut *mut llist_t, data: *mut libc::c_void);
  #[no_mangle]
  fn llist_pop(elm: *mut *mut llist_t) -> *mut libc::c_void;
  #[no_mangle]
  fn write_pidfile_std_path_and_ext(path: *const libc::c_char);
  #[no_mangle]
  fn remove_pidfile_std_path_and_ext(path: *const libc::c_char);
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_perror_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn get_linux_version_code() -> libc::c_int;
  #[no_mangle]
  fn config_open2(
    filename: *const libc::c_char,
    fopen_func: Option<unsafe extern "C" fn(_: *const libc::c_char) -> *mut FILE>,
  ) -> *mut parser_t;
  /* delims[0] is a comment char (use '\0' to disable), the rest are token delimiters */
  #[no_mangle]
  fn config_read(
    parser: *mut parser_t,
    tokens: *mut *mut libc::c_char,
    flags: libc::c_uint,
    delims: *const libc::c_char,
  ) -> libc::c_int;
  #[no_mangle]
  fn config_close(parser: *mut parser_t);
  /* '*const' ptr makes gcc optimize code much better.
   * Magic prevents ptr_to_globals from going into rodata.
   * If you want to assign a value, use SET_PTR_TO_GLOBALS(x) */
  #[no_mangle]
  static ptr_to_globals: *mut globals;
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
  #[no_mangle]
  fn closelog();
  #[no_mangle]
  fn openlog(__ident: *const libc::c_char, __option: libc::c_int, __facility: libc::c_int);
  #[no_mangle]
  fn syslog(__pri: libc::c_int, __fmt: *const libc::c_char, _: ...);
  #[no_mangle]
  fn shmctl(__shmid: libc::c_int, __cmd: libc::c_int, __buf: *mut shmid_ds) -> libc::c_int;
  #[no_mangle]
  fn shmget(__key: key_t, __size: size_t, __shmflg: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn shmat(
    __shmid: libc::c_int,
    __shmaddr: *const libc::c_void,
    __shmflg: libc::c_int,
  ) -> *mut libc::c_void;
  #[no_mangle]
  fn shmdt(__shmaddr: *const libc::c_void) -> libc::c_int;
  #[no_mangle]
  fn semget(__key: key_t, __nsems: libc::c_int, __semflg: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn semctl(__semid: libc::c_int, __semnum: libc::c_int, __cmd: libc::c_int, _: ...)
    -> libc::c_int;
  #[no_mangle]
  fn semop(__semid: libc::c_int, __sops: *mut sembuf, __nsops: size_t) -> libc::c_int;
}

use crate::librb::gid_t;
use crate::librb::__uid_t;

use crate::librb::__mode_t;

use crate::librb::__pid_t;
use crate::librb::__time_t;
pub type __key_t = libc::c_int;

pub type __syscall_ulong_t = libc::c_ulong;
pub type __socklen_t = libc::c_uint;
use crate::librb::int32_t;



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
use crate::librb::size_t;
use crate::librb::smallint;
use crate::librb::ssize_t;
use libc::uid_t;
pub type socklen_t = __socklen_t;
use libc::stat;

pub type key_t = __key_t;
use libc::time_t;
pub type __socket_type = libc::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
  pub sa_family: sa_family_t,
  pub sa_data: [libc::c_char; 14],
}
pub type C2RustUnnamed = libc::c_uint;
pub const MSG_CMSG_CLOEXEC: C2RustUnnamed = 1073741824;
pub const MSG_FASTOPEN: C2RustUnnamed = 536870912;
pub const MSG_ZEROCOPY: C2RustUnnamed = 67108864;
pub const MSG_BATCH: C2RustUnnamed = 262144;
pub const MSG_WAITFORONE: C2RustUnnamed = 65536;
pub const MSG_MORE: C2RustUnnamed = 32768;
pub const MSG_NOSIGNAL: C2RustUnnamed = 16384;
pub const MSG_ERRQUEUE: C2RustUnnamed = 8192;
pub const MSG_RST: C2RustUnnamed = 4096;
pub const MSG_CONFIRM: C2RustUnnamed = 2048;
pub const MSG_SYN: C2RustUnnamed = 1024;
pub const MSG_FIN: C2RustUnnamed = 512;
pub const MSG_WAITALL: C2RustUnnamed = 256;
pub const MSG_EOR: C2RustUnnamed = 128;
pub const MSG_DONTWAIT: C2RustUnnamed = 64;
pub const MSG_TRUNC: C2RustUnnamed = 32;
pub const MSG_PROXY: C2RustUnnamed = 16;
pub const MSG_CTRUNC: C2RustUnnamed = 8;
pub const MSG_TRYHARD: C2RustUnnamed = 4;
pub const MSG_DONTROUTE: C2RustUnnamed = 4;
pub const MSG_PEEK: C2RustUnnamed = 2;
pub const MSG_OOB: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_un {
  pub sun_family: sa_family_t,
  pub sun_path: [libc::c_char; 108],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
  pub sin6_family: sa_family_t,
  pub sin6_port: in_port_t,
  pub sin6_flowinfo: u32,
  pub sin6_addr: in6_addr,
  pub sin6_scope_id: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
  pub __in6_u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
  pub __u6_addr8: [u8; 16],
  pub __u6_addr16: [u16; 8],
  pub __u6_addr32: [u32; 4],
}
pub type in_port_t = u16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
  pub sin_family: sa_family_t,
  pub sin_port: in_port_t,
  pub sin_addr: in_addr,
  pub sin_zero: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
  pub s_addr: in_addr_t,
}
pub type in_addr_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __CONST_SOCKADDR_ARG {
  pub __sockaddr__: *const sockaddr,
  pub __sockaddr_at__: *const sockaddr_at,
  pub __sockaddr_ax25__: *const sockaddr_ax25,
  pub __sockaddr_dl__: *const sockaddr_dl,
  pub __sockaddr_eon__: *const sockaddr_eon,
  pub __sockaddr_in__: *const sockaddr_in,
  pub __sockaddr_in6__: *const sockaddr_in6,
  pub __sockaddr_inarp__: *const sockaddr_inarp,
  pub __sockaddr_ipx__: *const sockaddr_ipx,
  pub __sockaddr_iso__: *const sockaddr_iso,
  pub __sockaddr_ns__: *const sockaddr_ns,
  pub __sockaddr_un__: *const sockaddr_un,
  pub __sockaddr_x25__: *const sockaddr_x25,
}
use crate::librb::signal::__sighandler_t;

use libc::FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct len_and_sockaddr {
  pub len: socklen_t,
  pub u: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
  pub sa: sockaddr,
  pub sin: sockaddr_in,
  pub sin6: sockaddr_in6,
}
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
pub type C2RustUnnamed_2 = libc::c_uint;
pub const DAEMON_ONLY_SANITIZE: C2RustUnnamed_2 = 8;
pub const DAEMON_CLOSE_EXTRA_FDS: C2RustUnnamed_2 = 4;
pub const DAEMON_DEVNULL_STDIO: C2RustUnnamed_2 = 2;
pub const DAEMON_CHDIR_ROOT: C2RustUnnamed_2 = 1;
/* internal use */
//DAEMON_DOUBLE_FORK     = 1 << 4, /* double fork to avoid controlling tty */
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
use crate::libbb::llist::llist_t;
/*
 * Config file parser
 */
pub type C2RustUnnamed_3 = libc::c_uint;
// comments are recognized even if there is whitespace before
// ("line start><space><tab><space>#comment" is also comment, not only "line start>#comment")
// NORMAL is:
// * remove leading and trailing delimiters and collapse
//   multiple delimiters into one
// * warn and continue if less than mintokens delimiters found
// * grab everything into last token
// * comments are recognized even if they aren't the first char
pub const PARSE_NORMAL: C2RustUnnamed_3 = 4653056;
// delim[0] and delim[1] are two different allowed comment chars
// (so far, delim[0] will only work as comment char for full-line comment)
// (IOW: it works as if PARSE_EOL_COMMENTS is not set. sysctl applet is okay with this)
pub const PARSE_WS_COMMENTS: C2RustUnnamed_3 = 16777216;
// comments are recognized even if they aren't the first char
pub const PARSE_ALT_COMMENTS: C2RustUnnamed_3 = 8388608;
pub const PARSE_EOL_COMMENTS: C2RustUnnamed_3 = 4194304;
// die if < min tokens found
// keep a copy of current line
pub const PARSE_KEEP_COPY: C2RustUnnamed_3 = 2097152;
// last token takes entire remainder of the line
pub const PARSE_MIN_DIE: C2RustUnnamed_3 = 1048576;
// trim leading and trailing delimiters
// TODO: COLLAPSE and TRIM seem to always go in pair
pub const PARSE_GREEDY: C2RustUnnamed_3 = 262144;
// treat consecutive delimiters as one
pub const PARSE_TRIM: C2RustUnnamed_3 = 131072;
pub const PARSE_COLLAPSE: C2RustUnnamed_3 = 65536;
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
//extern const int const_int_1;
/* This struct is deliberately not defined. */
/* See docs/keep_data_small.txt */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub logFile: logFile_t,
  pub logLevel: libc::c_int,
  pub logFileSize: libc::c_uint,
  pub logFileRotate: libc::c_uint,
  pub shmid: libc::c_int,
  pub s_semid: libc::c_int,
  pub shm_size: libc::c_int,
  pub SMwup: [sembuf; 1],
  pub SMwdn: [sembuf; 3],
  pub log_rules: *mut logRule_t,
  pub kmsgfd: libc::c_int,
  pub primask: libc::c_int,
  pub remoteHosts: *mut llist_t,
  pub shbuf: *mut shbuf_ds,
  pub hostname: *mut libc::c_char,
  pub recvbuf: [libc::c_char; 512],
  pub parsebuf: [libc::c_char; 512],
  pub printbuf: [libc::c_char; 640],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct shbuf_ds {
  pub size: int32_t,
  pub tail: int32_t,
  pub data: [libc::c_char; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct logRule_t {
  pub enabled_facility_priomap: [u8; 24],
  pub file: *mut logFile_t,
  pub next: *mut logRule_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct logFile_t {
  pub path: *const libc::c_char,
  pub fd: libc::c_int,
  pub last_log_time: time_t,
  pub size: libc::c_uint,
  pub isRegular: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sembuf {
  pub sem_num: libc::c_ushort,
  pub sem_op: libc::c_short,
  pub sem_flg: libc::c_short,
}
pub type C2RustUnnamed_4 = libc::c_uint;
pub const COMMON_BUFSIZE: C2RustUnnamed_4 = 1024;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _code {
  pub c_name: *mut libc::c_char,
  pub c_val: libc::c_int,
}
pub type CODE = _code;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct shmid_ds {
  pub shm_perm: ipc_perm,
  pub shm_segsz: size_t,
  pub shm_atime: __time_t,
  pub shm_dtime: __time_t,
  pub shm_ctime: __time_t,
  pub shm_cpid: __pid_t,
  pub shm_lpid: __pid_t,
  pub shm_nattch: shmatt_t,
  pub __glibc_reserved4: __syscall_ulong_t,
  pub __glibc_reserved5: __syscall_ulong_t,
}
pub type shmatt_t = __syscall_ulong_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ipc_perm {
  pub __key: __key_t,
  pub uid: __uid_t,
  pub gid: gid_t,
  pub cuid: __uid_t,
  pub cgid: gid_t,
  pub mode: libc::c_ushort,
  pub __pad1: libc::c_ushort,
  pub __seq: libc::c_ushort,
  pub __pad2: libc::c_ushort,
  pub __glibc_reserved1: __syscall_ulong_t,
  pub __glibc_reserved2: __syscall_ulong_t,
}
pub type C2RustUnnamed_5 = libc::c_uint;
pub const DNS_WAIT_SEC: C2RustUnnamed_5 = 120;
pub const MAX_READ: C2RustUnnamed_5 = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct remoteHost_t {
  pub remoteFD: libc::c_int,
  pub last_dns_resolve: libc::c_uint,
  pub remoteAddr: *mut len_and_sockaddr,
  pub remoteHostname: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct init_globals {
  pub logFile: logFile_t,
  pub logLevel: libc::c_int,
  pub logFileSize: libc::c_uint,
  pub logFileRotate: libc::c_uint,
  pub shmid: libc::c_int,
  pub s_semid: libc::c_int,
  pub shm_size: libc::c_int,
  pub SMwup: [sembuf; 1],
  pub SMwdn: [sembuf; 3],
  pub log_rules: *mut logRule_t,
  pub kmsgfd: libc::c_int,
  pub primask: libc::c_int,
}
/* default shm size */
/* Options */
pub type C2RustUnnamed_6 = libc::c_uint;
pub const OPT_kmsg: C2RustUnnamed_6 = 8192;
pub const OPT_cfg: C2RustUnnamed_6 = 4096;
pub const OPT_dup: C2RustUnnamed_6 = 2048;
pub const OPT_circularlog: C2RustUnnamed_6 = 1024;
pub const OPT_locallog: C2RustUnnamed_6 = 512;
pub const OPT_remotelog: C2RustUnnamed_6 = 256;
pub const OPT_rotatecnt: C2RustUnnamed_6 = 128;
pub const OPT_filesize: C2RustUnnamed_6 = 64;
pub const OPT_timestamp: C2RustUnnamed_6 = 32;
pub const OPT_small: C2RustUnnamed_6 = 16;
pub const OPT_loglevel: C2RustUnnamed_6 = 8;
pub const OPT_outfile: C2RustUnnamed_6 = 4;
pub const OPT_nofork: C2RustUnnamed_6 = 2;
// -K
pub const OPT_mark: C2RustUnnamed_6 = 1;
// -f
pub const OPTBIT_kmsg: C2RustUnnamed_6 = 13;
// -D
pub const OPTBIT_cfg: C2RustUnnamed_6 = 12;
// -C
pub const OPTBIT_dup: C2RustUnnamed_6 = 11;
// -L
pub const OPTBIT_circularlog: C2RustUnnamed_6 = 10;
// -R
pub const OPTBIT_locallog: C2RustUnnamed_6 = 9;
// -b
pub const OPTBIT_remotelog: C2RustUnnamed_6 = 8;
// -s
pub const OPTBIT_rotatecnt: C2RustUnnamed_6 = 7;
// -t
pub const OPTBIT_filesize: C2RustUnnamed_6 = 6;
// -S
pub const OPTBIT_timestamp: C2RustUnnamed_6 = 5;
// -l
pub const OPTBIT_small: C2RustUnnamed_6 = 4;
// -O
pub const OPTBIT_loglevel: C2RustUnnamed_6 = 3;
// -n
pub const OPTBIT_outfile: C2RustUnnamed_6 = 2;
// -m
pub const OPTBIT_nofork: C2RustUnnamed_6 = 1;
pub const OPTBIT_mark: C2RustUnnamed_6 = 0;
/* circular buffer variables/structures */
/* our shared key (syslogd.c and logread.c must be in sync) */
pub type C2RustUnnamed_7 = libc::c_uint;
pub const KEY_ID: C2RustUnnamed_7 = 1095648583;
#[inline(always)]
unsafe extern "C" fn xatoul_range(
  mut str: *const libc::c_char,
  mut l: libc::c_ulong,
  mut u: libc::c_ulong,
) -> libc::c_ulong {
  return xatoull_range(str, l as libc::c_ulonglong, u as libc::c_ulonglong) as libc::c_ulong;
}
#[inline(always)]
unsafe extern "C" fn not_const_pp(mut p: *const libc::c_void) -> *mut libc::c_void {
  return p as *mut libc::c_void;
}
#[no_mangle]
pub static mut prioritynames: [CODE; 13] = [
  {
    let mut init = _code {
      c_name: b"alert\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
      c_val: 1i32,
    };
    init
  },
  {
    let mut init = _code {
      c_name: b"crit\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
      c_val: 2i32,
    };
    init
  },
  {
    let mut init = _code {
      c_name: b"debug\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
      c_val: 7i32,
    };
    init
  },
  {
    let mut init = _code {
      c_name: b"emerg\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
      c_val: 0i32,
    };
    init
  },
  {
    let mut init = _code {
      c_name: b"err\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
      c_val: 3i32,
    };
    init
  },
  {
    let mut init = _code {
      c_name: b"error\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
      c_val: 3i32,
    };
    init
  },
  {
    let mut init = _code {
      c_name: b"info\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
      c_val: 6i32,
    };
    init
  },
  {
    let mut init = _code {
      c_name: b"none\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
      c_val: 0x10i32,
    };
    init
  },
  {
    let mut init = _code {
      c_name: b"notice\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
      c_val: 5i32,
    };
    init
  },
  {
    let mut init = _code {
      c_name: b"panic\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
      c_val: 0i32,
    };
    init
  },
  {
    let mut init = _code {
      c_name: b"warn\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
      c_val: 4i32,
    };
    init
  },
  {
    let mut init = _code {
      c_name: b"warning\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
      c_val: 4i32,
    };
    init
  },
  {
    let mut init = _code {
      c_name: 0 as *const libc::c_char as *mut libc::c_char,
      c_val: -1i32,
    };
    init
  },
];
#[no_mangle]
pub static mut facilitynames: [CODE; 23] = [
  {
    let mut init = _code {
      c_name: b"auth\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
      c_val: 4i32 << 3i32,
    };
    init
  },
  {
    let mut init = _code {
      c_name: b"authpriv\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
      c_val: 10i32 << 3i32,
    };
    init
  },
  {
    let mut init = _code {
      c_name: b"cron\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
      c_val: 9i32 << 3i32,
    };
    init
  },
  {
    let mut init = _code {
      c_name: b"daemon\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
      c_val: 3i32 << 3i32,
    };
    init
  },
  {
    let mut init = _code {
      c_name: b"ftp\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
      c_val: 11i32 << 3i32,
    };
    init
  },
  {
    let mut init = _code {
      c_name: b"kern\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
      c_val: 0i32 << 3i32,
    };
    init
  },
  {
    let mut init = _code {
      c_name: b"lpr\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
      c_val: 6i32 << 3i32,
    };
    init
  },
  {
    let mut init = _code {
      c_name: b"mail\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
      c_val: 2i32 << 3i32,
    };
    init
  },
  {
    let mut init = _code {
      c_name: b"mark\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
      c_val: 24i32 << 3i32 | 0i32,
    };
    init
  },
  {
    let mut init = _code {
      c_name: b"news\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
      c_val: 7i32 << 3i32,
    };
    init
  },
  {
    let mut init = _code {
      c_name: b"security\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
      c_val: 4i32 << 3i32,
    };
    init
  },
  {
    let mut init = _code {
      c_name: b"syslog\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
      c_val: 5i32 << 3i32,
    };
    init
  },
  {
    let mut init = _code {
      c_name: b"user\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
      c_val: 1i32 << 3i32,
    };
    init
  },
  {
    let mut init = _code {
      c_name: b"uucp\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
      c_val: 8i32 << 3i32,
    };
    init
  },
  {
    let mut init = _code {
      c_name: b"local0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
      c_val: 16i32 << 3i32,
    };
    init
  },
  {
    let mut init = _code {
      c_name: b"local1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
      c_val: 17i32 << 3i32,
    };
    init
  },
  {
    let mut init = _code {
      c_name: b"local2\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
      c_val: 18i32 << 3i32,
    };
    init
  },
  {
    let mut init = _code {
      c_name: b"local3\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
      c_val: 19i32 << 3i32,
    };
    init
  },
  {
    let mut init = _code {
      c_name: b"local4\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
      c_val: 20i32 << 3i32,
    };
    init
  },
  {
    let mut init = _code {
      c_name: b"local5\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
      c_val: 21i32 << 3i32,
    };
    init
  },
  {
    let mut init = _code {
      c_name: b"local6\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
      c_val: 22i32 << 3i32,
    };
    init
  },
  {
    let mut init = _code {
      c_name: b"local7\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
      c_val: 23i32 << 3i32,
    };
    init
  },
  {
    let mut init = _code {
      c_name: 0 as *const libc::c_char as *mut libc::c_char,
      c_val: -1i32,
    };
    init
  },
];

/*
 * prioritynames[] and facilitynames[]
 *
 * Copyright (C) 2008 by Denys Vlasenko <vda.linux@gmail.com>
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
/* musl decided to be funny and it implements these as giant defines
 * of the form: ((CODE *)(const CODE []){ ... })
 * Which works, but causes _every_ function using them
 * to have a copy on stack (at least with gcc-6.3.0).
 * If we reference them just once, this saves 150 bytes.
 * The pointers themselves are optimized out
 * (no size change on uclibc).
 */
static mut bb_prioritynames: *const CODE = unsafe { prioritynames.as_ptr() as *mut _ };
static mut bb_facilitynames: *const CODE = unsafe { facilitynames.as_ptr() as *mut _ };
unsafe extern "C" fn find_by_name(
  mut name: *mut libc::c_char,
  mut c_set: *const CODE,
) -> *const CODE {
  while !(*c_set).c_name.is_null() {
    if strcmp(name, (*c_set).c_name) == 0i32 {
      return c_set;
    }
    c_set = c_set.offset(1)
  }
  return 0 as *const CODE;
}
unsafe extern "C" fn find_by_val(mut val: libc::c_int, mut c_set: *const CODE) -> *const CODE {
  while !(*c_set).c_name.is_null() {
    if (*c_set).c_val == val {
      return c_set;
    }
    c_set = c_set.offset(1)
  }
  return 0 as *const CODE;
}
unsafe extern "C" fn parse_syslogdcfg(mut file: *const libc::c_char) {
  let mut current_block: u64;
  let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut pp_rule: *mut *mut logRule_t = 0 as *mut *mut logRule_t;
  /* tok[0] set of selectors */
  /* tok[1] file name */
  /* tok[2] has to be NULL */
  let mut tok: [*mut libc::c_char; 3] = [0 as *mut libc::c_char; 3];
  let mut parser: *mut parser_t = 0 as *mut parser_t;
  parser = config_open2(
    if !file.is_null() {
      file
    } else {
      b"/etc/syslog.conf\x00" as *const u8 as *const libc::c_char
    },
    if !file.is_null() {
      Some(xfopen_for_read as unsafe extern "C" fn(_: *const libc::c_char) -> *mut FILE)
    } else {
      Some(fopen_for_read as unsafe extern "C" fn(_: *const libc::c_char) -> *mut FILE)
    },
  );
  if parser.is_null() {
    /* didn't find default /etc/syslog.conf */
    /* proceed as if we built busybox without config support */
    return;
  }
  /* use ptr to ptr to avoid checking whether head was initialized */
  pp_rule = &mut (*ptr_to_globals).log_rules;
  's_30: loop
  /* iterate through lines of config, skipping comments */
  {
    if !(config_read(
      parser,
      tok.as_mut_ptr(),
      (PARSE_NORMAL as libc::c_int
        | PARSE_MIN_DIE as libc::c_int
        | (2i32 & 0xffi32) << 8i32
        | 3i32 & 0xffi32) as libc::c_uint,
      b"# \t\x00" as *const u8 as *const libc::c_char,
    ) != 0)
    {
      current_block = 18383263831861166299;
      break;
    }
    let mut cur_selector: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cur_rule: *mut logRule_t = 0 as *mut logRule_t;
    /* unexpected trailing token? */
    if !tok[2].is_null() {
      current_block = 12846980873243673022;
      break;
    }
    *pp_rule = xzalloc(::std::mem::size_of::<logRule_t>() as libc::c_ulong) as *mut logRule_t;
    cur_rule = *pp_rule;
    cur_selector = tok[0];
    loop
    /* iterate through selectors: "kern.info;kern.!err;..." */
    {
      let mut code: *const CODE = 0 as *const CODE; /* "kern.!err" */
      let mut next_selector: *mut libc::c_char = 0 as *mut libc::c_char; /* "kern.=err" */
      let mut negated_prio: u8 = 0; /* bitmap of enabled facilities */
      let mut single_prio: u8 = 0; /* bitmap of enabled priorities */
      let mut facmap: u32 = 0; /* separate facility from priority */
      let mut primap: u8 = 0;
      let mut i: libc::c_uint = 0;
      next_selector = strchr(cur_selector, ';' as i32);
      if !next_selector.is_null() {
        let fresh0 = next_selector;
        next_selector = next_selector.offset(1);
        *fresh0 = '\u{0}' as i32 as libc::c_char
      }
      t = strchr(cur_selector, '.' as i32);
      if t.is_null() {
        current_block = 12846980873243673022;
        break 's_30;
      }
      let fresh1 = t;
      t = t.offset(1);
      *fresh1 = '\u{0}' as i32 as libc::c_char;
      negated_prio = 0i32 as u8;
      single_prio = 0i32 as u8;
      if *t as libc::c_int == '!' as i32 {
        negated_prio = 1i32 as u8;
        t = t.offset(1)
      }
      if *t as libc::c_int == '=' as i32 {
        single_prio = 1i32 as u8;
        t = t.offset(1)
      }
      /* parse priority */
      if *t as libc::c_int == '*' as i32 {
        primap = 0xffi32 as u8
      } else {
        let mut priority: u8 = 0; /* all 8 log levels enabled */
        code = find_by_name(t, bb_prioritynames);
        if code.is_null() {
          current_block = 12846980873243673022;
          break 's_30;
        }
        primap = 0i32 as u8;
        priority = (*code).c_val as u8;
        if priority as libc::c_int == 0x10i32 {
          /* ensure we take "enabled_facility_priomap[fac] &= 0" branch below */
          negated_prio = 1i32 as u8
        } else {
          priority = (1i32 << priority as libc::c_int) as u8;
          loop {
            primap = (primap as libc::c_int | priority as libc::c_int) as u8;
            if single_prio != 0 {
              break;
            }
            priority = (priority as libc::c_int >> 1i32) as u8;
            if !(priority != 0) {
              break;
            }
          }
          if negated_prio != 0 {
            primap = !(primap as libc::c_int) as u8
          }
        }
      }
      /* parse facility */
      if *cur_selector as libc::c_int == '*' as i32 {
        facmap = ((1i32 << 24i32) - 1i32) as u32
      } else {
        let mut next_facility: *mut libc::c_char = 0 as *mut libc::c_char;
        facmap = 0i32 as u32;
        t = cur_selector;
        loop
        /* iterate through facilities: "kern,daemon.<priospec>" */
        {
          next_facility = strchr(t, ',' as i32);
          if !next_facility.is_null() {
            let fresh2 = next_facility;
            next_facility = next_facility.offset(1);
            *fresh2 = '\u{0}' as i32 as libc::c_char
          }
          code = find_by_name(t, bb_facilitynames);
          if code.is_null() {
            current_block = 12846980873243673022;
            break 's_30;
          }
          /* "mark" is not a real facility, skip it */
          if (*code).c_val != 24i32 << 3i32 | 0i32 {
            facmap |= (1i32 << (((*code).c_val & 0x3f8i32) >> 3i32)) as libc::c_uint
          }
          t = next_facility;
          if t.is_null() {
            break;
          }
        }
      }
      /* merge result with previous selectors */
      i = 0i32 as libc::c_uint;
      while i < 24i32 as libc::c_uint {
        if !(facmap & (1i32 << i) as libc::c_uint == 0) {
          if negated_prio != 0 {
            (*cur_rule).enabled_facility_priomap[i as usize] =
              ((*cur_rule).enabled_facility_priomap[i as usize] as libc::c_int
                & primap as libc::c_int) as u8
          } else {
            (*cur_rule).enabled_facility_priomap[i as usize] =
              ((*cur_rule).enabled_facility_priomap[i as usize] as libc::c_int
                | primap as libc::c_int) as u8
          }
        }
        i = i.wrapping_add(1)
      }
      cur_selector = next_selector;
      if cur_selector.is_null() {
        break;
      }
    }
    /* check whether current file name was mentioned in previous rules or
     * as global logfile (G.logFile).
     */
    if strcmp((*ptr_to_globals).logFile.path, tok[1]) == 0i32 {
      (*cur_rule).file = &mut (*ptr_to_globals).logFile
    } else {
      /* temporarily use cur_rule as iterator, but *pp_rule still points
       * to currently processing rule entry.
       * NOTE: *pp_rule points to the current (and last in the list) rule.
       */
      cur_rule = (*ptr_to_globals).log_rules;
      loop {
        if !(cur_rule != *pp_rule) {
          current_block = 1352918242886884122;
          break;
        }
        if strcmp((*(*cur_rule).file).path, tok[1]) == 0i32 {
          /* found - reuse the same file structure */
          (**pp_rule).file = (*cur_rule).file;
          cur_rule = *pp_rule;
          current_block = 3380682794502917957;
          break;
        } else {
          cur_rule = (*cur_rule).next
        }
      }
      match current_block {
        3380682794502917957 => {}
        _ => {
          (*cur_rule).file =
            xzalloc(::std::mem::size_of::<logFile_t>() as libc::c_ulong) as *mut logFile_t;
          (*(*cur_rule).file).fd = -1i32;
          (*(*cur_rule).file).path = xstrdup(tok[1])
        }
      }
    }
    pp_rule = &mut (*cur_rule).next
  }
  match current_block {
    12846980873243673022 => {
      bb_error_msg_and_die(
        b"error in \'%s\' at line %d\x00" as *const u8 as *const libc::c_char,
        if !file.is_null() {
          file
        } else {
          b"/etc/syslog.conf\x00" as *const u8 as *const libc::c_char
        },
        (*parser).lineno,
      );
    }
    _ => {
      config_close(parser);
      return;
    }
  };
}
static mut init_data: init_globals = {
  let mut init = init_globals {
    logFile: {
      let mut init = logFile_t {
        path: b"/var/log/messages\x00" as *const u8 as *const libc::c_char,
        fd: -1i32,
        last_log_time: 0,
        size: 0,
        isRegular: 0,
      };
      init
    },
    logLevel: 8i32,
    logFileSize: (200i32 * 1024i32) as libc::c_uint,
    logFileRotate: 1i32 as libc::c_uint,
    shmid: -1i32,
    s_semid: -1i32,
    shm_size: 16i32 * 1024i32,
    SMwup: [{
      let mut init = sembuf {
        sem_num: 1i32 as libc::c_ushort,
        sem_op: -1i32 as libc::c_short,
        sem_flg: 0o4000i32 as libc::c_short,
      };
      init
    }],
    SMwdn: [
      {
        let mut init = sembuf {
          sem_num: 0i32 as libc::c_ushort,
          sem_op: 0i32 as libc::c_short,
          sem_flg: 0,
        };
        init
      },
      {
        let mut init = sembuf {
          sem_num: 1i32 as libc::c_ushort,
          sem_op: 0i32 as libc::c_short,
          sem_flg: 0,
        };
        init
      },
      {
        let mut init = sembuf {
          sem_num: 1i32 as libc::c_ushort,
          sem_op: 1i32 as libc::c_short,
          sem_flg: 0,
        };
        init
      },
    ],
    log_rules: 0 as *const logRule_t as *mut logRule_t,
    kmsgfd: 0,
    primask: 0,
  };
  init
};
/* "GENA" */
unsafe extern "C" fn ipcsyslog_cleanup() {
  if (*ptr_to_globals).shmid != -1i32 {
    shmdt((*ptr_to_globals).shbuf as *const libc::c_void);
  }
  if (*ptr_to_globals).shmid != -1i32 {
    shmctl((*ptr_to_globals).shmid, 0i32, 0 as *mut shmid_ds);
  }
  if (*ptr_to_globals).s_semid != -1i32 {
    semctl((*ptr_to_globals).s_semid, 0i32, 0i32, 0i32);
  };
}
unsafe extern "C" fn ipcsyslog_init() {
  (*ptr_to_globals).shmid = shmget(
    KEY_ID as libc::c_int,
    (*ptr_to_globals).shm_size as size_t,
    0o1000i32 | 0o644i32,
  );
  if (*ptr_to_globals).shmid == -1i32 {
    bb_simple_perror_msg_and_die(b"shmget\x00" as *const u8 as *const libc::c_char);
  }
  (*ptr_to_globals).shbuf =
    shmat((*ptr_to_globals).shmid, 0 as *const libc::c_void, 0i32) as *mut shbuf_ds;
  if (*ptr_to_globals).shbuf == -1i64 as *mut libc::c_void as *mut shbuf_ds {
    /* shmat has bizarre error return */
    bb_simple_perror_msg_and_die(b"shmat\x00" as *const u8 as *const libc::c_char);
  }
  memset(
    (*ptr_to_globals).shbuf as *mut libc::c_void,
    0i32,
    (*ptr_to_globals).shm_size as libc::c_ulong,
  );
  (*(*ptr_to_globals).shbuf).size = ((*ptr_to_globals).shm_size as libc::c_ulong)
    .wrapping_sub(8u64)
    .wrapping_sub(1i32 as libc::c_ulong) as int32_t;
  /*G.shbuf->tail = 0;*/
  /* we'll trust the OS to set initial semval to 0 (let's hope) */
  (*ptr_to_globals).s_semid = semget(KEY_ID as libc::c_int, 2i32, 0o1000i32 | 0o2000i32 | 1023i32);
  if (*ptr_to_globals).s_semid == -1i32 {
    if *bb_errno == 17i32 {
      (*ptr_to_globals).s_semid = semget(KEY_ID as libc::c_int, 2i32, 0i32);
      if (*ptr_to_globals).s_semid != -1i32 {
        return;
      }
    }
    bb_simple_perror_msg_and_die(b"semget\x00" as *const u8 as *const libc::c_char);
  };
}
/* Write message to shared mem buffer */
unsafe extern "C" fn log_to_shmem(mut msg: *const libc::c_char) {
  let mut old_tail: libc::c_int = 0;
  let mut new_tail: libc::c_int = 0;
  let mut len: libc::c_int = 0;
  if semop(
    (*ptr_to_globals).s_semid,
    (*ptr_to_globals).SMwdn.as_mut_ptr(),
    3i32 as size_t,
  ) == -1i32
  {
    bb_simple_perror_msg_and_die(b"SMwdn\x00" as *const u8 as *const libc::c_char);
  }
  /* Circular Buffer Algorithm:
   * --------------------------
   * tail == position where to store next syslog message.
   * tail's max value is (shbuf->size - 1)
   * Last byte of buffer is never used and remains NUL.
   */
  len = strlen(msg).wrapping_add(1i32 as libc::c_ulong) as libc::c_int; /* length with NUL included */
  loop {
    old_tail = (*(*ptr_to_globals).shbuf).tail;
    new_tail = old_tail + len;
    if new_tail < (*(*ptr_to_globals).shbuf).size {
      break;
    }
    /* k == available buffer space ahead of old tail */
    let mut k: libc::c_int = (*(*ptr_to_globals).shbuf).size - old_tail;
    /* copy what fits to the end of buffer, and repeat */
    memcpy(
      (*(*ptr_to_globals).shbuf)
        .data
        .as_mut_ptr()
        .offset(old_tail as isize) as *mut libc::c_void,
      msg as *const libc::c_void,
      k as libc::c_ulong,
    );
    msg = msg.offset(k as isize);
    len -= k;
    (*(*ptr_to_globals).shbuf).tail = 0i32
  }
  /* store message, set new tail */
  memcpy(
    (*(*ptr_to_globals).shbuf)
      .data
      .as_mut_ptr()
      .offset(old_tail as isize) as *mut libc::c_void,
    msg as *const libc::c_void,
    len as libc::c_ulong,
  );
  (*(*ptr_to_globals).shbuf).tail = new_tail;
  if semop(
    (*ptr_to_globals).s_semid,
    (*ptr_to_globals).SMwup.as_mut_ptr(),
    1i32 as size_t,
  ) == -1i32
  {
    bb_simple_perror_msg_and_die(b"SMwup\x00" as *const u8 as *const libc::c_char);
  };
}
/* FEATURE_IPC_SYSLOG */
unsafe extern "C" fn kmsg_init() {
  (*ptr_to_globals).kmsgfd = xopen(b"/dev/kmsg\x00" as *const u8 as *const libc::c_char, 0o1i32);
  /*
   * kernel < 3.5 expects single char printk KERN_* priority prefix,
   * from 3.5 onwards the full syslog facility/priority format is supported
   */
  if get_linux_version_code() < (3i32 << 16i32) + (5i32 << 8i32) + 0i32 {
    (*ptr_to_globals).primask = 0x7i32
  } else {
    (*ptr_to_globals).primask = -1i32
  };
}
unsafe extern "C" fn kmsg_cleanup() {}
/* Write message to /dev/kmsg */
unsafe extern "C" fn log_to_kmsg(mut pri: libc::c_int, mut msg: *const libc::c_char) {
  /*
   * kernel < 3.5 expects single char printk KERN_* priority prefix,
   * from 3.5 onwards the full syslog facility/priority format is supported
   */
  pri &= (*ptr_to_globals).primask;
  full_write(
    (*ptr_to_globals).kmsgfd,
    (*ptr_to_globals).printbuf.as_mut_ptr() as *const libc::c_void,
    sprintf(
      (*ptr_to_globals).printbuf.as_mut_ptr(),
      b"<%d>%s\n\x00" as *const u8 as *const libc::c_char,
      pri,
      msg,
    ) as size_t,
  );
}
/* FEATURE_KMSG_SYSLOG */
/* Print a message to the log file. */
unsafe extern "C" fn log_locally(
  mut now: time_t,
  mut msg: *mut libc::c_char,
  mut log_file: *mut logFile_t,
) {
  let mut current_block: u64;
  let mut len: libc::c_int = strlen(msg) as libc::c_int;
  /* fd can't be 0 (we connect fd 0 to /dev/log socket) */
  /* fd is 1 if "-O -" is in use */
  if (*log_file).fd > 1i32 {
    /* Reopen log files every second. This allows admin
     * to delete the files and not worry about restarting us.
     * This costs almost nothing since it happens
     * _at most_ once a second for each file, and happens
     * only when each file is actually written.
     */
    if now == 0 {
      now = time(0 as *mut time_t)
    }
    if (*log_file).last_log_time != now {
      (*log_file).last_log_time = now;
      close((*log_file).fd);
      current_block = 8218088424915217749;
    } else {
      current_block = 15768484401365413375;
    }
  } else if (*log_file).fd == 1i32 {
    current_block = 15768484401365413375;
  } else if *(*log_file).path.offset(0) as libc::c_int == '-' as i32
    && *(*log_file).path.offset(1) == 0
  {
    (*log_file).fd = 1i32;
    current_block = 15768484401365413375;
  /* log_file->isRegular = 0; - already is */
  } else {
    current_block = 8218088424915217749;
  }
  loop {
    match current_block {
      8218088424915217749 => {
        (*log_file).fd = open(
          (*log_file).path,
          0o1i32 | 0o100i32 | 0o400i32 | 0o2000i32 | 0o4000i32,
          0o666i32,
        );
        if (*log_file).fd < 0i32 {
          /* cannot open logfile? - print to /dev/console then */
          let mut fd: libc::c_int = device_open(
            b"/dev/console\x00" as *const u8 as *const libc::c_char,
            0o1i32 | 0o400i32 | 0o4000i32,
          ); /* then stderr, dammit */
          if fd < 0i32 {
            fd = 2i32
          }
          full_write(fd, msg as *const libc::c_void, len as size_t);
          if fd != 2i32 {
            close(fd);
          }
          return;
        }
        let mut statf: stat = std::mem::zeroed();
        (*log_file).isRegular = (fstat((*log_file).fd, &mut statf) == 0i32
          && statf.st_mode & 0o170000i32 as libc::c_uint == 0o100000i32 as libc::c_uint)
          as libc::c_int as u8;
        /* bug (mostly harmless): can wrap around if file > 4gb */
        (*log_file).size = statf.st_size as libc::c_uint;
        current_block = 15768484401365413375;
      }
      _ =>
      /* We are logging to stdout: do nothing */
      {
        if !((*ptr_to_globals).logFileSize != 0
          && (*log_file).isRegular as libc::c_int != 0
          && (*log_file).size > (*ptr_to_globals).logFileSize)
        {
          break;
        }
        if (*ptr_to_globals).logFileRotate != 0 {
          /* always 0..99 */
          let mut i: libc::c_int = strlen((*log_file).path)
            .wrapping_add(3i32 as libc::c_ulong)
            .wrapping_add(1i32 as libc::c_ulong) as libc::c_int;
          let vla = i as usize;
          let mut oldFile: Vec<libc::c_char> = ::std::vec::from_elem(0, vla);
          let vla_0 = i as usize;
          let mut newFile: Vec<libc::c_char> = ::std::vec::from_elem(0, vla_0);
          i = (*ptr_to_globals)
            .logFileRotate
            .wrapping_sub(1i32 as libc::c_uint) as libc::c_int;
          loop
          /* rename: f.8 -> f.9; f.7 -> f.8; ... */
          {
            sprintf(
              newFile.as_mut_ptr(),
              b"%s.%d\x00" as *const u8 as *const libc::c_char,
              (*log_file).path,
              i,
            );
            if i == 0i32 {
              break;
            }
            i -= 1;
            sprintf(
              oldFile.as_mut_ptr(),
              b"%s.%d\x00" as *const u8 as *const libc::c_char,
              (*log_file).path,
              i,
            );
            /* ignore errors - file might be missing */
            rename(oldFile.as_mut_ptr(), newFile.as_mut_ptr());
          }
          /* newFile == "f.0" now */
          rename((*log_file).path, newFile.as_mut_ptr());
        }
        /* We may or may not have just renamed the file away;
         * if we didn't rename because we aren't keeping any backlog,
         * then it's time to clobber the file. If we did rename it...,
         * incredibly, if F and F.0 are hardlinks, POSIX _demands_
         * that rename returns 0 but does not remove F!!!
         * (hardlinked F/F.0 pair was observed after
         * power failure during rename()).
         * So ensure old file is gone in any case:
         */
        unlink((*log_file).path);
        close((*log_file).fd);
        current_block = 8218088424915217749;
      }
    }
  }
  /* TODO: what to do on write errors ("disk full")? */
  len = full_write((*log_file).fd, msg as *const libc::c_void, len as size_t) as libc::c_int;
  if len > 0i32 {
    (*log_file).size = (*log_file).size.wrapping_add(len as libc::c_uint)
  };
}
unsafe extern "C" fn parse_fac_prio_20(mut pri: libc::c_int, mut res20: *mut libc::c_char) {
  let mut c_pri: *const CODE = 0 as *const CODE;
  let mut c_fac: *const CODE = 0 as *const CODE;
  c_fac = find_by_val((pri & 0x3f8i32) >> 3i32 << 3i32, bb_facilitynames);
  if !c_fac.is_null() {
    c_pri = find_by_val(pri & 0x7i32, bb_prioritynames);
    if !c_pri.is_null() {
      snprintf(
        res20,
        20i32 as libc::c_ulong,
        b"%s.%s\x00" as *const u8 as *const libc::c_char,
        (*c_fac).c_name,
        (*c_pri).c_name,
      );
      return;
    }
  }
  snprintf(
    res20,
    20i32 as libc::c_ulong,
    b"<%d>\x00" as *const u8 as *const libc::c_char,
    pri,
  );
}
/* len parameter is used only for "is there a timestamp?" check.
 * NB: some callers cheat and supply len==0 when they know
 * that there is no timestamp, short-circuiting the test. */
unsafe extern "C" fn timestamp_and_log(
  mut pri: libc::c_int,
  mut msg: *mut libc::c_char,
  mut len: libc::c_int,
) {
  let mut timestamp: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut now: time_t = 0;
  /* Jan 18 00:11:22 msg... */
  /* 01234567890123456 */
  if len >= 16i32
    && *msg.offset(3) as libc::c_int == ' ' as i32
    && *msg.offset(6) as libc::c_int == ' ' as i32
    && *msg.offset(9) as libc::c_int == ':' as i32
    && *msg.offset(12) as libc::c_int == ':' as i32
    && *msg.offset(15) as libc::c_int == ' ' as i32
  {
    if option_mask32 & OPT_timestamp as libc::c_int as libc::c_uint == 0 {
      /* use message timestamp */
      timestamp = msg;
      now = 0i32 as time_t
    }
    msg = msg.offset(16)
  }
  if timestamp.is_null() {
    time(&mut now);
    timestamp = ctime(&mut now).offset(4)
    /* skip day of week */
  }
  *timestamp.offset(15) = '\u{0}' as i32 as libc::c_char;
  if option_mask32 & OPT_kmsg as libc::c_int as libc::c_uint != 0 {
    log_to_kmsg(pri, msg);
    return;
  }
  if option_mask32 & OPT_small as libc::c_int as libc::c_uint != 0 {
    sprintf(
      (*ptr_to_globals).printbuf.as_mut_ptr(),
      b"%s %s\n\x00" as *const u8 as *const libc::c_char,
      timestamp,
      msg,
    );
  } else {
    let mut res: [libc::c_char; 20] = [0; 20];
    parse_fac_prio_20(pri, res.as_mut_ptr());
    sprintf(
      (*ptr_to_globals).printbuf.as_mut_ptr(),
      b"%s %.64s %s %s\n\x00" as *const u8 as *const libc::c_char,
      timestamp,
      (*ptr_to_globals).hostname,
      res.as_mut_ptr(),
      msg,
    );
  }
  /* Log message locally (to file or shared mem) */
  let mut match_0: bool = 0i32 != 0;
  let mut rule: *mut logRule_t = 0 as *mut logRule_t;
  let mut facility: u8 = ((pri & 0x3f8i32) >> 3i32) as u8;
  let mut prio_bit: u8 = (1i32 << (pri & 0x7i32)) as u8;
  rule = (*ptr_to_globals).log_rules;
  while !rule.is_null() {
    if (*rule).enabled_facility_priomap[facility as usize] as libc::c_int & prio_bit as libc::c_int
      != 0
    {
      log_locally(now, (*ptr_to_globals).printbuf.as_mut_ptr(), (*rule).file);
      match_0 = 1i32 != 0
    }
    rule = (*rule).next
  }
  if match_0 {
    return;
  }
  if pri & 0x7i32 < (*ptr_to_globals).logLevel {
    if option_mask32 & OPT_circularlog as libc::c_int as libc::c_uint != 0
      && !(*ptr_to_globals).shbuf.is_null()
    {
      log_to_shmem((*ptr_to_globals).printbuf.as_mut_ptr());
      return;
    }
    log_locally(
      now,
      (*ptr_to_globals).printbuf.as_mut_ptr(),
      &mut (*ptr_to_globals).logFile,
    );
  };
}
unsafe extern "C" fn timestamp_and_log_internal(mut msg: *const libc::c_char) {
  /* -L, or no -R */
  if 1i32 != 0 && option_mask32 & OPT_locallog as libc::c_int as libc::c_uint == 0 {
    return;
  }
  timestamp_and_log(5i32 << 3i32 | 6i32, msg as *mut libc::c_char, 0i32);
}
/* tmpbuf[len] is a NUL byte (set by caller), but there can be other,
 * embedded NULs. Split messages on each of these NULs, parse prio,
 * escape control chars and log each locally. */
unsafe extern "C" fn split_escape_and_log(mut tmpbuf: *mut libc::c_char, mut len: libc::c_int) {
  let mut p: *mut libc::c_char = tmpbuf;
  tmpbuf = tmpbuf.offset(len as isize);
  while p < tmpbuf {
    let mut c: libc::c_char = 0;
    let mut q: *mut libc::c_char = (*ptr_to_globals).parsebuf.as_mut_ptr();
    let mut pri: libc::c_int = 1i32 << 3i32 | 5i32;
    if *p as libc::c_int == '<' as i32 {
      /* Parse the magic priority number */
      pri = bb_strtou(p.offset(1), &mut p, 10i32) as libc::c_int;
      if *p as libc::c_int == '>' as i32 {
        p = p.offset(1)
      }
      if pri & !(0x3f8i32 | 0x7i32) != 0 {
        pri = 1i32 << 3i32 | 5i32
      }
    }
    loop {
      let fresh3 = p;
      p = p.offset(1);
      c = *fresh3;
      if !(c != 0) {
        break;
      }
      if c as libc::c_int == '\n' as i32 {
        c = ' ' as i32 as libc::c_char
      }
      if c as libc::c_int & !0x1fi32 == 0 && c as libc::c_int != '\t' as i32 {
        let fresh4 = q;
        q = q.offset(1);
        *fresh4 = '^' as i32 as libc::c_char;
        c = (c as libc::c_int + '@' as i32) as libc::c_char
        /* ^@, ^A, ^B... */
      }
      let fresh5 = q;
      q = q.offset(1);
      *fresh5 = c
    }
    *q = '\u{0}' as i32 as libc::c_char;
    /* Now log it */
    timestamp_and_log(
      pri,
      (*ptr_to_globals).parsebuf.as_mut_ptr(),
      q.wrapping_offset_from((*ptr_to_globals).parsebuf.as_mut_ptr()) as libc::c_long
        as libc::c_int,
    );
  }
}
/* Don't inline: prevent struct sockaddr_un to take up space on stack
 * permanently */
#[inline(never)]
unsafe extern "C" fn create_socket() -> libc::c_int {
  let mut sunx: sockaddr_un = sockaddr_un {
    sun_family: 0,
    sun_path: [0; 108],
  };
  let mut sock_fd: libc::c_int = 0;
  let mut dev_log_name: *mut libc::c_char = 0 as *mut libc::c_char;
  memset(
    &mut sunx as *mut sockaddr_un as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<sockaddr_un>() as libc::c_ulong,
  );
  sunx.sun_family = 1i32 as sa_family_t;
  /* Unlink old /dev/log or object it points to. */
  /* (if it exists, bind will fail) */
  strcpy(
    sunx.sun_path.as_mut_ptr(),
    b"/dev/log\x00" as *const u8 as *const libc::c_char,
  );
  dev_log_name = xmalloc_follow_symlinks(b"/dev/log\x00" as *const u8 as *const libc::c_char);
  if !dev_log_name.is_null() {
    safe_strncpy(
      sunx.sun_path.as_mut_ptr(),
      dev_log_name,
      ::std::mem::size_of::<[libc::c_char; 108]>() as libc::c_ulong,
    );
    free(dev_log_name as *mut libc::c_void);
  }
  unlink(sunx.sun_path.as_mut_ptr());
  sock_fd = xsocket(1i32, SOCK_DGRAM as libc::c_int, 0i32);
  xbind(
    sock_fd,
    &mut sunx as *mut sockaddr_un as *mut sockaddr,
    ::std::mem::size_of::<sockaddr_un>() as libc::c_ulong as socklen_t,
  );
  chmod(
    b"/dev/log\x00" as *const u8 as *const libc::c_char,
    0o666i32 as __mode_t,
  );
  return sock_fd;
}
unsafe extern "C" fn try_to_resolve_remote(mut rh: *mut remoteHost_t) -> libc::c_int {
  if (*rh).remoteAddr.is_null() {
    let mut now: libc::c_uint = monotonic_sec();
    /* Don't resolve name too often - DNS timeouts can be big */
    if now.wrapping_sub((*rh).last_dns_resolve) < DNS_WAIT_SEC as libc::c_int as libc::c_uint {
      return -1i32;
    }
    (*rh).last_dns_resolve = now;
    (*rh).remoteAddr = host2sockaddr((*rh).remoteHostname, 514i32);
    if (*rh).remoteAddr.is_null() {
      return -1i32;
    }
  }
  return xsocket(
    (*(*rh).remoteAddr).u.sa.sa_family as libc::c_int,
    SOCK_DGRAM as libc::c_int,
    0i32,
  );
}
unsafe extern "C" fn do_syslogd() -> ! {
  let mut item: *mut llist_t = 0 as *mut llist_t;
  let mut last_sz: libc::c_int = -1i32;
  let mut last_buf: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut recvbuf: *mut libc::c_char = (*ptr_to_globals).recvbuf.as_mut_ptr();
  /* Set up signal handlers (so that they interrupt read()) */
  signal_no_SA_RESTART_empty_mask(
    15i32,
    Some(record_signo as unsafe extern "C" fn(_: libc::c_int) -> ()),
  );
  signal_no_SA_RESTART_empty_mask(
    2i32,
    Some(record_signo as unsafe extern "C" fn(_: libc::c_int) -> ()),
  );
  //signal_no_SA_RESTART_empty_mask(SIGQUIT, record_signo);
  signal(
    1i32,
    ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(1i32 as libc::intptr_t),
  ); /* while (!bb_got_signal) */
  xmove_fd(create_socket(), 0i32);
  if option_mask32 & OPT_circularlog as libc::c_int as libc::c_uint != 0 {
    ipcsyslog_init();
  }
  if option_mask32 & OPT_kmsg as libc::c_int as libc::c_uint != 0 {
    kmsg_init();
  }
  timestamp_and_log_internal(
    b"syslogd started: BusyBox v1.32.0.git\x00" as *const u8 as *const libc::c_char,
  );
  let mut current_block_32: u64;
  's_52: while bb_got_signal == 0 {
    let mut sz: ssize_t = 0;
    last_buf = recvbuf;
    if recvbuf == (*ptr_to_globals).recvbuf.as_mut_ptr() {
      recvbuf = (*ptr_to_globals)
        .recvbuf
        .as_mut_ptr()
        .offset(MAX_READ as libc::c_int as isize)
    } else {
      recvbuf = (*ptr_to_globals).recvbuf.as_mut_ptr()
    }
    'c_12157: loop {
      sz = read(
        0i32,
        recvbuf as *mut libc::c_void,
        (MAX_READ as libc::c_int - 1i32) as size_t,
      );
      if sz < 0i32 as libc::c_long {
        if bb_got_signal == 0 {
          bb_perror_msg(
            b"read from %s\x00" as *const u8 as *const libc::c_char,
            b"/dev/log\x00" as *const u8 as *const libc::c_char,
          );
        }
        break 's_52;
      } else {
        loop
        /* Drop trailing '\n' and NULs (typically there is one NUL) */
        {
          if sz == 0i32 as libc::c_long {
            continue 'c_12157;
          }
          /* man 3 syslog says: "A trailing newline is added when needed".
           * However, neither glibc nor uclibc do this:
           * syslog(prio, "test")   sends "test\0" to /dev/log,
           * syslog(prio, "test\n") sends "test\n\0".
           * IOW: newline is passed verbatim!
           * I take it to mean that it's syslogd's job
           * to make those look identical in the log files. */
          if *recvbuf.offset((sz - 1i32 as libc::c_long) as isize) as libc::c_int != '\u{0}' as i32
            && *recvbuf.offset((sz - 1i32 as libc::c_long) as isize) as libc::c_int != '\n' as i32
          {
            break;
          }
          sz -= 1
        }
        if option_mask32 & OPT_dup as libc::c_int as libc::c_uint != 0
          && sz == last_sz as libc::c_long
        {
          current_block_32 = 9828876828309294594;
          break;
        } else {
          current_block_32 = 7056779235015430508;
          break;
        }
      }
    }
    match current_block_32 {
      9828876828309294594 => {
        if memcmp(
          last_buf as *const libc::c_void,
          recvbuf as *const libc::c_void,
          sz as libc::c_ulong,
        ) == 0i32
        {
          continue;
        }
      }
      _ => {}
    }
    last_sz = sz as libc::c_int;
    /* Stock syslogd sends it '\n'-terminated
     * over network, mimic that */
    *recvbuf.offset(sz as isize) = '\n' as i32 as libc::c_char;
    let mut current_block_27: u64;
    /* We are not modifying log messages in any way before send */
    /* Remote site cannot trust _us_ anyway and need to do validation again */
    item = (*ptr_to_globals).remoteHosts;
    while !item.is_null() {
      let mut rh: *mut remoteHost_t = (*item).data as *mut remoteHost_t;
      if (*rh).remoteFD == -1i32 {
        (*rh).remoteFD = try_to_resolve_remote(rh);
        if (*rh).remoteFD == -1i32 {
          current_block_27 = 4775909272756257391;
        } else {
          current_block_27 = 9853141518545631134;
        }
      } else {
        current_block_27 = 9853141518545631134;
      }
      match current_block_27 {
        9853141518545631134 => {
          /* Send message to remote logger.
           * On some errors, close and set remoteFD to -1
           * so that DNS resolution is retried.
           */
          if sendto(
            (*rh).remoteFD,
            recvbuf as *const libc::c_void,
            (sz + 1i32 as libc::c_long) as size_t,
            MSG_DONTWAIT as libc::c_int | MSG_NOSIGNAL as libc::c_int,
            __CONST_SOCKADDR_ARG {
              __sockaddr__: &mut (*(*rh).remoteAddr).u.sa,
            },
            (*(*rh).remoteAddr).len,
          ) == -1i32 as libc::c_long
          {
            match *bb_errno {
              104 | 107 | 32 => {
                /* paranoia */
                close((*rh).remoteFD); /* ensure it *is* NUL terminated */
                (*rh).remoteFD = -1i32;
                free((*rh).remoteAddr as *mut libc::c_void);
                (*rh).remoteAddr = 0 as *mut len_and_sockaddr
              }
              _ => {}
            }
          }
        }
        _ => {}
      }
      item = (*item).link
    }
    if 1i32 == 0 || option_mask32 & OPT_locallog as libc::c_int as libc::c_uint != 0 {
      *recvbuf.offset(sz as isize) = '\u{0}' as i32 as libc::c_char;
      split_escape_and_log(recvbuf, sz as libc::c_int);
    }
  }
  timestamp_and_log_internal(b"syslogd exiting\x00" as *const u8 as *const libc::c_char);
  remove_pidfile_std_path_and_ext(b"syslogd\x00" as *const u8 as *const libc::c_char);
  ipcsyslog_cleanup();
  if option_mask32 & OPT_kmsg as libc::c_int as libc::c_uint != 0 {
    kmsg_cleanup();
  }
  kill_myself_with_sig(bb_got_signal as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn syslogd_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut opts: libc::c_int = 0;
  let mut opt_m: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut opt_l: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut opt_s: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut opt_b: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut opt_C: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut opt_f: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut remoteAddrList: *mut llist_t = 0 as *mut llist_t;
  let ref mut fresh6 = *(not_const_pp(&ptr_to_globals as *const *mut globals as *const libc::c_void)
    as *mut *mut globals);
  *fresh6 = memcpy(
    xzalloc(::std::mem::size_of::<globals>() as libc::c_ulong),
    &init_data as *const init_globals as *const libc::c_void,
    ::std::mem::size_of::<init_globals>() as libc::c_ulong,
  ) as *mut globals;
  asm!("" : : : "memory" : "volatile");
  /* No non-option params */
  opts = getopt32(
    argv,
    b"^m:nO:l:Sts:b:R:*LC::Df:K\x00=0\x00" as *const u8 as *const libc::c_char,
    &mut opt_m as *mut *mut libc::c_char,
    &mut (*ptr_to_globals).logFile.path as *mut *const libc::c_char,
    &mut opt_l as *mut *mut libc::c_char,
    &mut opt_s as *mut *mut libc::c_char,
    &mut opt_b as *mut *mut libc::c_char,
    &mut remoteAddrList as *mut *mut llist_t,
    &mut opt_C as *mut *mut libc::c_char,
    &mut opt_f as *mut *mut libc::c_char,
  ) as libc::c_int;
  while !remoteAddrList.is_null() {
    let mut rh: *mut remoteHost_t =
      xzalloc(::std::mem::size_of::<remoteHost_t>() as libc::c_ulong) as *mut remoteHost_t;
    (*rh).remoteHostname = llist_pop(&mut remoteAddrList) as *const libc::c_char;
    (*rh).remoteFD = -1i32;
    (*rh).last_dns_resolve = monotonic_sec()
      .wrapping_sub(DNS_WAIT_SEC as libc::c_int as libc::c_uint)
      .wrapping_sub(1i32 as libc::c_uint);
    llist_add_to(&mut (*ptr_to_globals).remoteHosts, rh as *mut libc::c_void);
  }
  //if (opts & OPT_nofork) // -n
  //if (opts & OPT_outfile) // -O
  if opts & OPT_loglevel as libc::c_int != 0 {
    // -l
    (*ptr_to_globals).logLevel =
      xatou_range(opt_l, 1i32 as libc::c_uint, 8i32 as libc::c_uint) as libc::c_int
  }
  //if (opts & OPT_small) // -S
  if opts & OPT_filesize as libc::c_int != 0 {
    // -s
    (*ptr_to_globals).logFileSize = xatou_range(
      opt_s,
      0i32 as libc::c_uint,
      (2147483647i32 / 1024i32) as libc::c_uint,
    )
    .wrapping_mul(1024i32 as libc::c_uint)
  }
  if opts & OPT_rotatecnt as libc::c_int != 0 {
    // -b
    (*ptr_to_globals).logFileRotate =
      xatou_range(opt_b, 0i32 as libc::c_uint, 99i32 as libc::c_uint)
  }
  if !opt_C.is_null() {
    // -Cn
    (*ptr_to_globals).shm_size = xatoul_range(
      opt_C,
      4i32 as libc::c_ulong,
      (2147483647i32 / 1024i32) as libc::c_ulong,
    )
    .wrapping_mul(1024i32 as libc::c_ulong) as libc::c_int
  }
  /* If they have not specified remote logging, then log locally */
  if 1i32 != 0 && opts & OPT_remotelog as libc::c_int == 0 {
    // -R
    option_mask32 |= OPT_locallog as libc::c_int as libc::c_uint
  }
  parse_syslogdcfg(opt_f);
  /* Store away localhost's name before the fork */
  (*ptr_to_globals).hostname = safe_gethostname();
  *strchrnul((*ptr_to_globals).hostname, '.' as i32) = '\u{0}' as i32 as libc::c_char;
  if opts & OPT_nofork as libc::c_int == 0 {
    bb_daemonize_or_rexec(DAEMON_CHDIR_ROOT as libc::c_int);
  }
  //umask(0); - why??
  write_pidfile_std_path_and_ext(b"syslogd\x00" as *const u8 as *const libc::c_char);
  do_syslogd();
  /* return EXIT_SUCCESS; */
}

/*
 * Mini logger implementation for busybox
 *
 * Copyright (C) 1999-2004 by Erik Andersen <andersen@codepoet.org>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config LOGGER
//config:	bool "logger (6.3 kb)"
//config:	default y
//config:	select FEATURE_SYSLOG
//config:	help
//config:	The logger utility allows you to send arbitrary text
//config:	messages to the system log (i.e. the 'syslogd' utility) so
//config:	they can be logged. This is generally used to help locate
//config:	problems that occur within programs and scripts.
//applet:IF_LOGGER(APPLET(logger, BB_DIR_USR_BIN, BB_SUID_DROP))
//kbuild:lib-$(CONFIG_LOGGER) += syslogd_and_logger.o
//usage:#define logger_trivial_usage
//usage:       "[OPTIONS] [MESSAGE]"
//usage:#define logger_full_usage "\n\n"
//usage:       "Write MESSAGE (or stdin) to syslog\n"
//usage:     "\n	-s	Log to stderr as well as the system log"
//usage:     "\n	-t TAG	Log using the specified tag (defaults to user name)"
//usage:     "\n	-p PRIO	Priority (numeric or facility.level pair)"
//usage:
//usage:#define logger_example_usage
//usage:       "$ logger \"hello\"\n"
/*
 * Done in syslogd_and_logger.c:
#include "libbb.h"
#define SYSLOG_NAMES
#define SYSLOG_NAMES_CONST
#include <syslog.h>
*/
/* Decode a symbolic name to a numeric value
 * this function is based on code
 * Copyright (c) 1983, 1993
 * The Regents of the University of California.  All rights reserved.
 *
 * Original copyright notice is retained at the end of this file.
 */
unsafe extern "C" fn decode(mut name: *mut libc::c_char, mut codetab: *const CODE) -> libc::c_int {
  let mut c: *const CODE = 0 as *const CODE;
  if (*name as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32 {
    return atoi(name);
  }
  c = codetab;
  while !(*c).c_name.is_null() {
    if strcasecmp(name, (*c).c_name) == 0 {
      return (*c).c_val;
    }
    c = c.offset(1)
  }
  return -1i32;
}
/* Decode a symbolic name to a numeric value
 * this function is based on code
 * Copyright (c) 1983, 1993
 * The Regents of the University of California.  All rights reserved.
 *
 * Original copyright notice is retained at the end of this file.
 */
unsafe extern "C" fn pencode(mut s: *mut libc::c_char) -> libc::c_int {
  let mut save: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut lev: libc::c_int = 0;
  let mut fac: libc::c_int = 1i32 << 3i32;
  save = s;
  while *s as libc::c_int != 0 && *s as libc::c_int != '.' as i32 {
    s = s.offset(1)
  }
  if *s != 0 {
    *s = '\u{0}' as i32 as libc::c_char;
    fac = decode(save, bb_facilitynames);
    if fac < 0i32 {
      bb_error_msg_and_die(
        b"unknown %s name: %s\x00" as *const u8 as *const libc::c_char,
        b"facility\x00" as *const u8 as *const libc::c_char,
        save,
      );
    }
    let fresh7 = s;
    s = s.offset(1);
    *fresh7 = '.' as i32 as libc::c_char
  } else {
    s = save
  }
  lev = decode(s, bb_prioritynames);
  if lev < 0i32 {
    bb_error_msg_and_die(
      b"unknown %s name: %s\x00" as *const u8 as *const libc::c_char,
      b"priority\x00" as *const u8 as *const libc::c_char,
      save,
    );
  }
  return lev & 0x7i32 | fac & 0x3f8i32;
}
#[no_mangle]
pub unsafe extern "C" fn logger_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut str_p: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut str_t: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut opt: libc::c_int = 0;
  let mut i: libc::c_int = 0i32;
  /* Fill out the name string early (may be overwritten later) */
  str_t = uid2uname_utoa(geteuid());
  /* Parse any options */
  opt = getopt32(
    argv,
    b"p:st:\x00" as *const u8 as *const libc::c_char,
    &mut str_p as *mut *mut libc::c_char,
    &mut str_t as *mut *mut libc::c_char,
  ) as libc::c_int;
  if opt & 0x2i32 != 0 {
    /* -s */
    i |= 0x20i32
  }
  //if (opt & 0x4) /* -t */
  openlog(str_t, i, 0i32);
  i = 1i32 << 3i32 | 5i32;
  if opt & 0x1i32 != 0 {
    /* -p */
    i = pencode(str_p)
  }
  argv = argv.offset(optind as isize);
  if (*argv.offset(0)).is_null() {
    while !fgets_unlocked(
      bb_common_bufsiz1.as_mut_ptr(),
      COMMON_BUFSIZE as libc::c_int,
      stdin,
    )
    .is_null()
    {
      if *bb_common_bufsiz1.as_mut_ptr().offset(0) as libc::c_int != 0
        && (*bb_common_bufsiz1.as_mut_ptr().offset(0) as libc::c_int != '\n' as i32
          || *bb_common_bufsiz1.as_mut_ptr().offset(1) as libc::c_int != 0)
      {
        /* Neither "" nor "\n" */
        syslog(
          i,
          b"%s\x00" as *const u8 as *const libc::c_char,
          bb_common_bufsiz1.as_mut_ptr(),
        );
      }
    }
  } else {
    let mut message: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0i32;
    let mut pos: libc::c_int = 0i32;
    loop {
      len = (len as libc::c_ulong).wrapping_add(strlen(*argv).wrapping_add(1i32 as libc::c_ulong))
        as libc::c_int as libc::c_int;
      message = xrealloc(message as *mut libc::c_void, (len + 1i32) as size_t) as *mut libc::c_char;
      sprintf(
        message.offset(pos as isize),
        b" %s\x00" as *const u8 as *const libc::c_char,
        *argv,
      );
      pos = len;
      argv = argv.offset(1);
      if (*argv).is_null() {
        break;
      }
    }
    syslog(
      i,
      b"%s\x00" as *const u8 as *const libc::c_char,
      message.offset(1),
    );
    /* skip leading " " */
  }
  closelog();
  return 0i32;
}
