use crate::librb::size_t;
use crate::librb::smallint;
use c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use libc;
use libc::unlink;
use libc::close;
use libc::free;
use libc::gid_t;
use libc::off64_t;
use libc::off_t;
use libc::pid_t;
use libc::ssize_t;
use libc::stat;
use libc::time_t;
use libc::uid_t;

extern "C" {
  pub type sockaddr_x25;
  pub type sockaddr_un;
  pub type sockaddr_ns;
  pub type sockaddr_iso;
  pub type sockaddr_ipx;
  pub type sockaddr_inarp;
  pub type sockaddr_eon;
  pub type sockaddr_dl;
  pub type sockaddr_ax25;
  pub type sockaddr_at;

  #[no_mangle]
  fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn lseek(__fd: libc::c_int, __offset: off64_t, __whence: libc::c_int) -> off64_t;



  #[no_mangle]
  fn alarm(__seconds: libc::c_uint) -> libc::c_uint;

  #[no_mangle]
  fn chdir(__path: *const libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn execv(__path: *const libc::c_char, __argv: *const *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn _exit(_: libc::c_int) -> !;

  #[no_mangle]
  fn fork() -> pid_t;

  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  static ptr_to_globals: *mut globals;

  #[no_mangle]
  fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;

  #[no_mangle]
  fn socket(__domain: libc::c_int, __type: libc::c_int, __protocol: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn connect(__fd: libc::c_int, __addr: __CONST_SOCKADDR_ARG, __len: socklen_t) -> libc::c_int;

  #[no_mangle]
  fn getpeername(__fd: libc::c_int, __addr: __SOCKADDR_ARG, __len: *mut socklen_t) -> libc::c_int;

  #[no_mangle]
  fn accept(__fd: libc::c_int, __addr: __SOCKADDR_ARG, __addr_len: *mut socklen_t) -> libc::c_int;

  #[no_mangle]
  fn shutdown(__fd: libc::c_int, __how: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;

  #[no_mangle]
  static mut stdout: *mut FILE;

  #[no_mangle]
  static mut stderr: *mut FILE;

  #[no_mangle]
  fn fclose(__stream: *mut FILE) -> libc::c_int;

  #[no_mangle]
  fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;

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
  fn dprintf(__fd: libc::c_int, __fmt: *const libc::c_char, _: ...) -> libc::c_int;

  #[no_mangle]
  fn fgets_unlocked(
    __s: *mut libc::c_char,
    __n: libc::c_int,
    __stream: *mut FILE,
  ) -> *mut libc::c_char;

  #[no_mangle]
  fn puts(__s: *const libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;

  #[no_mangle]
  fn atoi(__nptr: *const libc::c_char) -> libc::c_int;



  #[no_mangle]
  fn putenv(__string: *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn setenv(
    __name: *const libc::c_char,
    __value: *const libc::c_char,
    __replace: libc::c_int,
  ) -> libc::c_int;

  #[no_mangle]
  fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;

  #[no_mangle]
  fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;

  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;

  #[no_mangle]
  fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;

  #[no_mangle]
  fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;

  #[no_mangle]
  fn strchrnul(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;

  #[no_mangle]
  fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;

  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;

  #[no_mangle]
  fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn strncasecmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;

  #[no_mangle]
  fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;

  #[no_mangle]
  fn time(__timer: *mut time_t) -> time_t;

  #[no_mangle]
  fn strftime(
    __s: *mut libc::c_char,
    __maxsize: size_t,
    __format: *const libc::c_char,
    __tp: *const tm,
  ) -> size_t;

  #[no_mangle]
  fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;

  #[no_mangle]
  fn setgroups(__n: size_t, __groups: *const gid_t) -> libc::c_int;
  /* Search for an entry with a matching username.  */

  #[no_mangle]
  fn bb_internal_getpwnam(__name: *const libc::c_char) -> *mut passwd;
  /* All function names below should be remapped by #defines above
   * in order to not collide with libc names. */

  #[no_mangle]
  fn bb_internal_getspnam_r(
    __name: *const libc::c_char,
    __result_buf: *mut spwd,
    __buffer: *mut libc::c_char,
    __buflen: size_t,
    __result: *mut *mut spwd,
  ) -> libc::c_int;

  #[no_mangle]
  static bb_errno: *mut libc::c_int;

  #[no_mangle]
  fn skip_whitespace(_: *const libc::c_char) -> *mut libc::c_char;

  #[no_mangle]
  fn xmalloc(size: size_t) -> *mut libc::c_void;

  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;

  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;

  #[no_mangle]
  fn xstrndup(s: *const libc::c_char, n: libc::c_int) -> *mut libc::c_char;

  #[no_mangle]
  fn is_directory(name: *const libc::c_char, followLinks: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn bb_basename(name: *const libc::c_char) -> *const libc::c_char;

  #[no_mangle]
  fn is_prefixed_with(string: *const libc::c_char, key: *const libc::c_char) -> *mut libc::c_char;

  #[no_mangle]
  fn xdup2(_: libc::c_int, _: libc::c_int);

  #[no_mangle]
  fn xmove_fd(_: libc::c_int, _: libc::c_int);

  #[no_mangle]
  fn xrealloc_getcwd_or_warn(cwd: *mut libc::c_char) -> *mut libc::c_char;

  #[no_mangle]
  fn bb_signals(sigs: libc::c_int, f: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>);

  #[no_mangle]
  fn xsetgid(gid: gid_t);

  #[no_mangle]
  fn xsetuid(uid: uid_t);

  #[no_mangle]
  fn xchdir(path: *const libc::c_char);

  #[no_mangle]
  fn xpipe(filedes: *mut libc::c_int);

  #[no_mangle]
  fn xlisten(s: libc::c_int, backlog: libc::c_int);

  #[no_mangle]
  fn setsockopt_keepalive(fd: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn create_and_bind_stream_or_die(bindaddr: *const libc::c_char, port: libc::c_int)
    -> libc::c_int;

  #[no_mangle]
  fn host2sockaddr(host: *const libc::c_char, port: libc::c_int) -> *mut len_and_sockaddr;

  #[no_mangle]
  fn xmalloc_sockaddr2dotted(sa: *const sockaddr) -> *mut libc::c_char;

  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;

  #[no_mangle]
  fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;

  #[no_mangle]
  fn safe_write(fd: libc::c_int, buf: *const libc::c_void, count: size_t) -> ssize_t;

  #[no_mangle]
  fn full_write(fd: libc::c_int, buf: *const libc::c_void, count: size_t) -> ssize_t;

  #[no_mangle]
  fn fopen_for_read(path: *const libc::c_char) -> *mut FILE;

  #[no_mangle]
  fn safe_poll(ufds: *mut pollfd, nfds: nfds_t, timeout_ms: libc::c_int) -> libc::c_int;
  /* Non-aborting kind of convertors: bb_strto[u][l]l */
  /* On exit: errno = 0 only if there was non-empty, '\0' terminated value
   * errno = EINVAL if value was not '\0' terminated, but otherwise ok
   *    Return value is still valid, caller should just check whether end[0]
   *    is a valid terminating char for particular case. OTOH, if caller
   *    requires '\0' terminated input, [s]he can just check errno == 0.
   * errno = ERANGE if value had alphanumeric terminating char ("1234abcg").
   * errno = ERANGE if value is out of range, missing, etc.
   * errno = ERANGE if value had minus sign for strtouXX (even "-0" is not ok )
   *    return value is all-ones in this case.
   */

  #[no_mangle]
  fn bb_strtoull(
    arg: *const libc::c_char,
    endp: *mut *mut libc::c_char,
    base: libc::c_int,
  ) -> libc::c_ulonglong;

  #[no_mangle]
  fn bb_strtou(
    arg: *const libc::c_char,
    endp: *mut *mut libc::c_char,
    base: libc::c_int,
  ) -> libc::c_uint;

  #[no_mangle]
  fn xget_uidgid(_: *mut bb_uidgid_t, _: *const libc::c_char);

  #[no_mangle]
  fn bb_daemonize_or_rexec(flags: libc::c_int);

  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;

  #[no_mangle]
  static mut xfunc_error_retval: u8;

  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);

  #[no_mangle]
  fn bb_simple_error_msg(s: *const libc::c_char);

  #[no_mangle]
  fn bb_perror_msg(s: *const libc::c_char, _: ...);

  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);

  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;

  #[no_mangle]
  fn concat_path_file(
    path: *const libc::c_char,
    filename: *const libc::c_char,
  ) -> *mut libc::c_char;

  #[no_mangle]
  fn bb_simplify_abs_path_inplace(path: *mut libc::c_char) -> *mut libc::c_char;

  #[no_mangle]
  fn pw_encrypt(
    clear: *const libc::c_char,
    salt: *const libc::c_char,
    cleanup: libc::c_int,
  ) -> *mut libc::c_char;

  #[no_mangle]
  fn crypt_make_salt(p: *mut libc::c_char, cnt: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn percent_decode_in_place(str: *mut libc::c_char, strict: libc::c_int) -> *mut libc::c_char;

  #[no_mangle]
  static mut applet_name: *const libc::c_char;

  #[no_mangle]
  fn vfork() -> libc::c_int;

  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];

  #[no_mangle]
  fn sendfile(
    __out_fd: libc::c_int,
    __in_fd: libc::c_int,
    __offset: *mut off64_t,
    __count: size_t,
  ) -> ssize_t;
}

pub type __socklen_t = libc::c_uint;

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
pub type socklen_t = __socklen_t;

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
pub const SHUT_RDWR: C2RustUnnamed = 2;
pub const SHUT_WR: C2RustUnnamed = 1;
pub const SHUT_RD: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __SOCKADDR_ARG {
  pub __sockaddr__: *mut sockaddr,
  pub __sockaddr_at__: *mut sockaddr_at,
  pub __sockaddr_ax25__: *mut sockaddr_ax25,
  pub __sockaddr_dl__: *mut sockaddr_dl,
  pub __sockaddr_eon__: *mut sockaddr_eon,
  pub __sockaddr_in__: *mut sockaddr_in,
  pub __sockaddr_in6__: *mut sockaddr_in6,
  pub __sockaddr_inarp__: *mut sockaddr_inarp,
  pub __sockaddr_ipx__: *mut sockaddr_ipx,
  pub __sockaddr_iso__: *mut sockaddr_iso,
  pub __sockaddr_ns__: *mut sockaddr_ns,
  pub __sockaddr_un__: *mut sockaddr_un,
  pub __sockaddr_x25__: *mut sockaddr_x25,
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
pub type nfds_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
  pub fd: libc::c_int,
  pub events: libc::c_short,
  pub revents: libc::c_short,
}
use libc::tm;
use libc::passwd;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spwd {
  pub sp_namp: *mut libc::c_char,
  pub sp_pwdp: *mut libc::c_char,
  pub sp_lstchg: libc::c_long,
  pub sp_min: libc::c_long,
  pub sp_max: libc::c_long,
  pub sp_warn: libc::c_long,
  pub sp_inact: libc::c_long,
  pub sp_expire: libc::c_long,
  pub sp_flag: libc::c_ulong,
}
use crate::librb::fd_pair;
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
pub type C2RustUnnamed_2 = libc::c_uint;
pub const LSA_SIZEOF_SA: C2RustUnnamed_2 = 28;
pub const LSA_LEN_SIZE: C2RustUnnamed_2 = 4;
use crate::librb::bb_uidgid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub verbose: libc::c_int,
  pub flg_deny_all: smallint,
  pub content_gzip: smallint,
  pub last_mod: time_t,
  pub rmt_ip_str: *mut libc::c_char,
  pub bind_addr_or_port: *const libc::c_char,
  pub g_query: *mut libc::c_char,
  pub opt_c_configFile: *const libc::c_char,
  pub home_httpd: *const libc::c_char,
  pub index_page: *const libc::c_char,
  pub found_mime_type: *const libc::c_char,
  pub found_moved_temporarily: *const libc::c_char,
  pub ip_a_d: *mut Htaccess_IP,
  pub g_realm: *const libc::c_char,
  pub remoteuser: *mut libc::c_char,
  pub file_size: off_t,
  pub range_start: off_t,
  pub range_end: off_t,
  pub range_len: off_t,
  pub g_auth: *mut Htaccess,
  pub mime_a: *mut Htaccess,
  pub script_i: *mut Htaccess,
  pub iobuf: *mut libc::c_char,
  pub hdr_ptr: *mut libc::c_char,
  pub hdr_cnt: libc::c_int,
  pub http_error_page: [*const libc::c_char; 11],
  pub proxy: *mut Htaccess_Proxy,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Htaccess_Proxy {
  pub next: *mut Htaccess_Proxy,
  pub url_from: *mut libc::c_char,
  pub host_port: *mut libc::c_char,
  pub url_to: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Htaccess {
  pub next: *mut Htaccess,
  pub after_colon: *mut libc::c_char,
  pub before_colon: [libc::c_char; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Htaccess_IP {
  pub next: *mut Htaccess_IP,
  pub ip: libc::c_uint,
  pub mask: libc::c_uint,
  pub allow_deny: libc::c_int,
}
pub type C2RustUnnamed_3 = libc::c_uint;
pub const COMMON_BUFSIZE: C2RustUnnamed_3 = 1024;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct has_next_ptr {
  pub next: *mut has_next_ptr,
}
pub type CGI_type = libc::c_uint;
pub const CGI_INTERPRETER: CGI_type = 3;
pub const CGI_INDEX: CGI_type = 2;
pub const CGI_NORMAL: CGI_type = 1;
pub const CGI_NONE: CGI_type = 0;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const HTTP_CONTINUE: C2RustUnnamed_4 = 100;
pub const HTTP_ENTITY_TOO_LARGE: C2RustUnnamed_4 = 413;
pub const HTTP_INTERNAL_SERVER_ERROR: C2RustUnnamed_4 = 500;
pub const HTTP_NOT_IMPLEMENTED: C2RustUnnamed_4 = 501;
pub const HTTP_REQUEST_TIMEOUT: C2RustUnnamed_4 = 408;
pub const HTTP_FORBIDDEN: C2RustUnnamed_4 = 403;
pub const HTTP_NOT_FOUND: C2RustUnnamed_4 = 404;
pub const HTTP_UNAUTHORIZED: C2RustUnnamed_4 = 401;
pub const HTTP_BAD_REQUEST: C2RustUnnamed_4 = 400;
pub const HTTP_MOVED_TEMPORARILY: C2RustUnnamed_4 = 302;
pub const HTTP_PARTIAL_CONTENT: C2RustUnnamed_4 = 206;
pub const HTTP_OK: C2RustUnnamed_4 = 200;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
  pub name: *const libc::c_char,
  pub info: *const libc::c_char,
}
/* Prototypes */
pub type C2RustUnnamed_6 = libc::c_uint;
pub const SEND_HEADERS_AND_BODY: C2RustUnnamed_6 = 3;
pub const SEND_BODY: C2RustUnnamed_6 = 2;
pub const SEND_HEADERS: C2RustUnnamed_6 = 1;
/*
 * Parse configuration file into in-memory linked list.
 *
 * Any previous IP rules are discarded.
 * If the flag argument is not SUBDIR_PARSE then all /path and mime rules
 * are also discarded.  That is, previous settings are retained if flag is
 * SUBDIR_PARSE.
 * Error pages are only parsed on the main config file.
 *
 * path   Path where to look for httpd.conf (without filename).
 * flag   Type of the parse request.
 */
/* flag param: */
pub type C2RustUnnamed_7 = libc::c_uint;
/* path will be derived from URL */
/* path will be "/etc" */
pub const SUBDIR_PARSE: C2RustUnnamed_7 = 2;
/* path will be "/etc" */
pub const SIGNALED_PARSE: C2RustUnnamed_7 = 1;
pub const FIRST_PARSE: C2RustUnnamed_7 = 0;
pub const TRY_CURDIR_PARSE: C2RustUnnamed_8 = 3;
pub type C2RustUnnamed_8 = libc::c_uint;
pub const FROM_CGI: C2RustUnnamed_9 = 1;
pub const TO_CGI: C2RustUnnamed_9 = 2;
pub type C2RustUnnamed_9 = libc::c_uint;
pub type C2RustUnnamed_10 = libc::c_uint;
pub const OPT_VERBOSE: C2RustUnnamed_10 = 1024;
pub const OPT_FOREGROUND: C2RustUnnamed_10 = 512;
pub const OPT_INETD: C2RustUnnamed_10 = 256;
pub const OPT_PORT: C2RustUnnamed_10 = 128;
pub const OPT_SETUID: C2RustUnnamed_10 = 64;
pub const OPT_MD5: C2RustUnnamed_10 = 32;
pub const OPT_REALM: C2RustUnnamed_10 = 16;
pub const OPT_ENCODE_URL: C2RustUnnamed_10 = 8;
pub const OPT_HOME_HTTPD: C2RustUnnamed_10 = 4;
pub const OPT_DECODE_URL: C2RustUnnamed_10 = 2;
pub const OPT_CONFIG_FILE: C2RustUnnamed_10 = 1;
pub const p_opt_verbose: C2RustUnnamed_10 = 10;
pub const p_opt_foreground: C2RustUnnamed_10 = 9;
pub const p_opt_inetd: C2RustUnnamed_10 = 8;
pub const p_opt_port: C2RustUnnamed_10 = 7;
pub const u_opt_setuid: C2RustUnnamed_10 = 6;
pub const m_opt_md5: C2RustUnnamed_10 = 5;
pub const r_opt_realm: C2RustUnnamed_10 = 4;
pub const e_opt_encode_url: C2RustUnnamed_10 = 3;
pub const h_opt_home_httpd: C2RustUnnamed_10 = 2;
pub const d_opt_decode_url: C2RustUnnamed_10 = 1;
pub const c_opt_config_file: C2RustUnnamed_10 = 0;
#[inline(always)]
unsafe extern "C" fn bb_ascii_isalnum(mut a: libc::c_uchar) -> libc::c_int {
  let mut b: libc::c_uchar = (a as libc::c_int - '0' as i32) as libc::c_uchar;
  if b as libc::c_int <= 9i32 {
    return (b as libc::c_int <= 9i32) as libc::c_int;
  }
  b = ((a as libc::c_int | 0x20i32) - 'a' as i32) as libc::c_uchar;
  return (b as libc::c_int <= 'z' as i32 - 'a' as i32) as libc::c_int;
}
#[inline(always)]
unsafe extern "C" fn not_const_pp(mut p: *const libc::c_void) -> *mut libc::c_void {
  return p as *mut libc::c_void;
}
#[inline(always)]
unsafe extern "C" fn bb_strtoul(
  mut arg: *const libc::c_char,
  mut endp: *mut *mut libc::c_char,
  mut base: libc::c_int,
) -> libc::c_ulong {
  return bb_strtoull(arg, endp, base) as libc::c_ulong;
}
static mut DEFAULT_PATH_HTTPD_CONF: [libc::c_char; 5] = [47, 101, 116, 99, 0];
static mut HTTPD_CONF: [libc::c_char; 11] = [104, 116, 116, 112, 100, 46, 99, 111, 110, 102, 0];
static mut HTTP_200: [libc::c_char; 18] = [
  72, 84, 84, 80, 47, 49, 46, 48, 32, 50, 48, 48, 32, 79, 75, 13, 10, 0,
];
static mut index_html: [libc::c_char; 11] = [105, 110, 100, 101, 120, 46, 104, 116, 109, 108, 0];
static mut http_response_type: [u16; 11] = [
  HTTP_OK as libc::c_int as u16,
  HTTP_PARTIAL_CONTENT as libc::c_int as u16,
  HTTP_MOVED_TEMPORARILY as libc::c_int as u16,
  HTTP_REQUEST_TIMEOUT as libc::c_int as u16,
  HTTP_NOT_IMPLEMENTED as libc::c_int as u16,
  HTTP_UNAUTHORIZED as libc::c_int as u16,
  HTTP_NOT_FOUND as libc::c_int as u16,
  HTTP_BAD_REQUEST as libc::c_int as u16,
  HTTP_FORBIDDEN as libc::c_int as u16,
  HTTP_INTERNAL_SERVER_ERROR as libc::c_int as u16,
  HTTP_ENTITY_TOO_LARGE as libc::c_int as u16,
];
static mut http_response: [C2RustUnnamed_5; 11] = [
  {
    let mut init = C2RustUnnamed_5 {
      name: b"OK\x00" as *const u8 as *const libc::c_char,
      info: 0 as *const libc::c_char,
    };
    init
  },
  {
    let mut init = C2RustUnnamed_5 {
      name: b"Partial Content\x00" as *const u8 as *const libc::c_char,
      info: 0 as *const libc::c_char,
    };
    init
  },
  {
    let mut init = C2RustUnnamed_5 {
      name: b"Found\x00" as *const u8 as *const libc::c_char,
      info: 0 as *const libc::c_char,
    };
    init
  },
  {
    let mut init = C2RustUnnamed_5 {
      name: b"Request Timeout\x00" as *const u8 as *const libc::c_char,
      info: b"No request appeared within 60 seconds\x00" as *const u8 as *const libc::c_char,
    };
    init
  },
  {
    let mut init = C2RustUnnamed_5 {
      name: b"Not Implemented\x00" as *const u8 as *const libc::c_char,
      info: b"The requested method is not recognized\x00" as *const u8 as *const libc::c_char,
    };
    init
  },
  {
    let mut init = C2RustUnnamed_5 {
      name: b"Unauthorized\x00" as *const u8 as *const libc::c_char,
      info: b"\x00" as *const u8 as *const libc::c_char,
    };
    init
  },
  {
    let mut init = C2RustUnnamed_5 {
      name: b"Not Found\x00" as *const u8 as *const libc::c_char,
      info: b"The requested URL was not found\x00" as *const u8 as *const libc::c_char,
    };
    init
  },
  {
    let mut init = C2RustUnnamed_5 {
      name: b"Bad Request\x00" as *const u8 as *const libc::c_char,
      info: b"Unsupported method\x00" as *const u8 as *const libc::c_char,
    };
    init
  },
  {
    let mut init = C2RustUnnamed_5 {
      name: b"Forbidden\x00" as *const u8 as *const libc::c_char,
      info: b"\x00" as *const u8 as *const libc::c_char,
    };
    init
  },
  {
    let mut init = C2RustUnnamed_5 {
      name: b"Internal Server Error\x00" as *const u8 as *const libc::c_char,
      info: b"Internal Server Error\x00" as *const u8 as *const libc::c_char,
    };
    init
  },
  {
    let mut init = C2RustUnnamed_5 {
      name: b"Entity Too Large\x00" as *const u8 as *const libc::c_char,
      info: b"Entity Too Large\x00" as *const u8 as *const libc::c_char,
    };
    init
  },
];
unsafe extern "C" fn free_llist(mut pptr: *mut *mut has_next_ptr) {
  let mut cur: *mut has_next_ptr = *pptr;
  while !cur.is_null() {
    let mut t: *mut has_next_ptr = cur;
    cur = (*cur).next;
    free(t as *mut libc::c_void);
  }
  *pptr = 0 as *mut has_next_ptr;
}
#[inline(always)]
unsafe extern "C" fn free_Htaccess_list(mut pptr: *mut *mut Htaccess) {
  free_llist(pptr as *mut *mut has_next_ptr);
}
#[inline(always)]
unsafe extern "C" fn free_Htaccess_IP_list(mut pptr: *mut *mut Htaccess_IP) {
  free_llist(pptr as *mut *mut has_next_ptr);
}
/* Returns presumed mask width in bits or < 0 on error.
 * Updates strp, stores IP at provided pointer */
unsafe extern "C" fn scan_ip(
  mut strp: *mut *const libc::c_char,
  mut ipp: *mut libc::c_uint,
  mut endc: libc::c_uchar,
) -> libc::c_int {
  let mut p: *const libc::c_char = *strp;
  let mut auto_mask: libc::c_int = 8i32;
  let mut ip: libc::c_uint = 0i32 as libc::c_uint;
  let mut j: libc::c_int = 0;
  if *p as libc::c_int == '/' as i32 {
    return -auto_mask;
  }
  j = 0i32;
  while j < 4i32 {
    let mut octet: libc::c_uint = 0;
    if ((*p as libc::c_int) < '0' as i32 || *p as libc::c_int > '9' as i32)
      && *p as libc::c_int != '/' as i32
      && *p as libc::c_int != 0
    {
      return -auto_mask;
    }
    octet = 0i32 as libc::c_uint;
    while *p as libc::c_int >= '0' as i32 && *p as libc::c_int <= '9' as i32 {
      octet = octet.wrapping_mul(10i32 as libc::c_uint);
      octet = octet.wrapping_add((*p as libc::c_int - '0' as i32) as libc::c_uint);
      if octet > 255i32 as libc::c_uint {
        return -auto_mask;
      }
      p = p.offset(1)
    }
    if *p as libc::c_int == '.' as i32 {
      p = p.offset(1)
    }
    if *p as libc::c_int != '/' as i32 && *p as libc::c_int != 0 {
      auto_mask += 8i32
    }
    ip = ip << 8i32 | octet;
    j += 1
  }
  if *p != 0 {
    if *p as libc::c_int != endc as libc::c_int {
      return -auto_mask;
    }
    p = p.offset(1);
    if *p as libc::c_int == '\u{0}' as i32 {
      return -auto_mask;
    }
  }
  *ipp = ip;
  *strp = p;
  return auto_mask;
}
/* Returns 0 on success. Stores IP and mask at provided pointers */
unsafe extern "C" fn scan_ip_mask(
  mut str: *const libc::c_char,
  mut ipp: *mut libc::c_uint,
  mut maskp: *mut libc::c_uint,
) -> libc::c_int {
  let mut i: libc::c_int = 0;
  let mut mask: libc::c_uint = 0;
  let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
  i = scan_ip(&mut str, ipp, '/' as i32 as libc::c_uchar);
  if i < 0i32 {
    return i;
  }
  if *str != 0 {
    /* there is /xxx after dotted-IP address */
    i = bb_strtou(str, &mut p, 10i32) as libc::c_int;
    if *p as libc::c_int == '.' as i32 {
      /* 'xxx' itself is dotted-IP mask, parse it */
      /* (return 0 (success) only if it has N.N.N.N form) */
      return scan_ip(&mut str, maskp, '\u{0}' as i32 as libc::c_uchar) - 32i32;
    }
    if *p != 0 {
      return -1i32;
    }
  }
  if i > 32i32 {
    return -1i32;
  }
  if ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong == 4i32 as libc::c_ulong && i == 32i32 {
    /* mask >>= 32 below may not work */
    mask = 0i32 as libc::c_uint
  } else {
    mask = 0xffffffffu32;
    mask >>= i
  }
  /* i == 0 -> *maskp = 0x00000000
   * i == 1 -> *maskp = 0x80000000
   * i == 4 -> *maskp = 0xf0000000
   * i == 31 -> *maskp = 0xfffffffe
   * i == 32 -> *maskp = 0xffffffff */
  *maskp = !mask;
  return 0i32;
}
unsafe extern "C" fn parse_conf(mut path: *const libc::c_char, mut flag: libc::c_int) {
  /* internally used extra flag state */
  let mut f: *mut FILE = 0 as *mut FILE;
  let mut filename: *const libc::c_char = 0 as *const libc::c_char;
  let mut buf: [libc::c_char; 160] = [0; 160];
  /* discard old rules */
  free_Htaccess_IP_list(&mut (*ptr_to_globals).ip_a_d);
  (*ptr_to_globals).flg_deny_all = 0i32 as smallint;
  /* retain previous auth and mime config only for subdir parse */
  if flag != SUBDIR_PARSE as libc::c_int {
    free_Htaccess_list(&mut (*ptr_to_globals).mime_a);
    free_Htaccess_list(&mut (*ptr_to_globals).g_auth);
    free_Htaccess_list(&mut (*ptr_to_globals).script_i);
  }
  filename = (*ptr_to_globals).opt_c_configFile;
  if flag == SUBDIR_PARSE as libc::c_int || filename.is_null() {
    let mut fresh0 = ::std::vec::from_elem(
      0,
      strlen(path)
        .wrapping_add(::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong)
        .wrapping_add(2i32 as libc::c_ulong) as usize,
    );
    filename = fresh0.as_mut_ptr() as *const libc::c_char;
    sprintf(
      filename as *mut libc::c_char,
      b"%s/%s\x00" as *const u8 as *const libc::c_char,
      path,
      HTTPD_CONF.as_ptr(),
    );
  }
  loop {
    f = fopen_for_read(filename);
    if !f.is_null() {
      break;
    }
    if flag >= SUBDIR_PARSE as libc::c_int {
      /* SUBDIR or TRY_CURDIR */
      /* config file not found, no changes to config */
      return;
    }
    if flag == FIRST_PARSE as libc::c_int {
      /* -c CONFFILE given, but CONFFILE doesn't exist? */
      if !(*ptr_to_globals).opt_c_configFile.is_null() {
        bb_simple_perror_msg_and_die((*ptr_to_globals).opt_c_configFile);
      }
      /* else: no -c, thus we looked at /etc/httpd.conf,
       * and it's not there. try ./httpd.conf: */
    }
    flag = TRY_CURDIR_PARSE as libc::c_int;
    filename = HTTPD_CONF.as_ptr()
  }
  /* in "/file:user:pass" lines, we prepend path in subdirs */
  if flag != SUBDIR_PARSE as libc::c_int {
    path = b"\x00" as *const u8 as *const libc::c_char
  }
  /* The lines can be:
   *
   * I:default_index_file
   * H:http_home
   * [AD]:IP[/mask]   # allow/deny, * for wildcard
   * Ennn:error.html  # error page for status nnn
   * P:/url:[http://]hostname[:port]/new/path # reverse proxy
   * .ext:mime/type   # mime type
   * *.php:/path/php  # run xxx.php through an interpreter
   * /file:user:pass  # username and password
   */
  while !fgets_unlocked(
    buf.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_char; 160]>() as libc::c_ulong as libc::c_int,
    f,
  )
  .is_null()
  {
    let mut strlen_buf: libc::c_uint = 0; /* while (fgets) */
    let mut ch: libc::c_uchar = 0;
    let mut after_colon: *mut libc::c_char = 0 as *mut libc::c_char;
    /* empty line */
    /* skip assumed "A:*", it is a default anyway */
    /* remove all whitespace, and # comments */
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p0: *mut libc::c_char = 0 as *mut libc::c_char;
    p0 = buf.as_mut_ptr();
    loop
    /* skip non-whitespace beginning. Often the whole line
     * is non-whitespace. We want this case to work fast,
     * without needless copying, therefore we don't merge
     * this operation into next while loop. */
    {
      ch = *p0 as libc::c_uchar;
      if !(ch as libc::c_int != '\u{0}' as i32
        && ch as libc::c_int != '\n' as i32
        && ch as libc::c_int != '#' as i32
        && ch as libc::c_int != ' ' as i32
        && ch as libc::c_int != '\t' as i32)
      {
        break;
      }
      p0 = p0.offset(1)
    }
    p = p0;
    /* if we enter this loop, we have some whitespace.
     * discard it */
    while ch as libc::c_int != '\u{0}' as i32
      && ch as libc::c_int != '\n' as i32
      && ch as libc::c_int != '#' as i32
    {
      if ch as libc::c_int != ' ' as i32 && ch as libc::c_int != '\t' as i32 {
        let fresh1 = p;
        p = p.offset(1);
        *fresh1 = ch as libc::c_char
      }
      p0 = p0.offset(1);
      ch = *p0 as libc::c_uchar
    }
    *p = '\u{0}' as i32 as libc::c_char;
    strlen_buf = p.wrapping_offset_from(buf.as_mut_ptr()) as libc::c_long as libc::c_uint;
    if strlen_buf == 0i32 as libc::c_uint {
      continue;
    }
    after_colon = strchr(buf.as_mut_ptr(), ':' as i32);
    /* strange line? */
    if !(after_colon.is_null() || {
      after_colon = after_colon.offset(1); /* toupper if it's a letter */
      (*after_colon as libc::c_int) == '\u{0}' as i32
    }) {
      ch = (buf[0] as libc::c_int & !0x20i32) as libc::c_uchar;
      if ch as libc::c_int == 'I' as i32 {
        if (*ptr_to_globals).index_page != index_html.as_ptr() {
          free((*ptr_to_globals).index_page as *mut libc::c_char as *mut libc::c_void);
        }
        (*ptr_to_globals).index_page = xstrdup(after_colon);
        continue;
      } else if flag == FIRST_PARSE as libc::c_int && ch as libc::c_int == 'H' as i32 {
        (*ptr_to_globals).home_httpd = xstrdup(after_colon);
        xchdir((*ptr_to_globals).home_httpd);
        continue;
      } else if ch as libc::c_int == 'A' as i32 || ch as libc::c_int == 'D' as i32 {
        let mut pip: *mut Htaccess_IP = 0 as *mut Htaccess_IP;
        if *after_colon as libc::c_int == '*' as i32 {
          if ch as libc::c_int == 'D' as i32 {
            /* do not allow jumping around using H in subdir's configs */
            /* memorize "deny all" */
            (*ptr_to_globals).flg_deny_all = 1i32 as smallint
          }
          continue;
        } else {
          /* store "allow/deny IP/mask" line */
          pip = xzalloc(::std::mem::size_of::<Htaccess_IP>() as libc::c_ulong) as *mut Htaccess_IP;
          if scan_ip_mask(after_colon, &mut (*pip).ip, &mut (*pip).mask) != 0 {
            /* IP{/mask} syntax error detected, protect all */
            ch = 'D' as i32 as libc::c_uchar;
            (*pip).mask = 0i32 as libc::c_uint
          }
          (*pip).allow_deny = ch as libc::c_int;
          if ch as libc::c_int == 'D' as i32 {
            /* Deny:from_IP - prepend */
            (*pip).next = (*ptr_to_globals).ip_a_d;
            (*ptr_to_globals).ip_a_d = pip
          } else {
            /* A:from_IP - append (thus all D's precedes A's) */
            let mut prev_IP: *mut Htaccess_IP = (*ptr_to_globals).ip_a_d; /* error status code */
            if prev_IP.is_null() {
              (*ptr_to_globals).ip_a_d = pip
            } else {
              while !(*prev_IP).next.is_null() {
                prev_IP = (*prev_IP).next
              }
              (*prev_IP).next = pip
            }
          }
          continue;
        }
      } else if flag == FIRST_PARSE as libc::c_int && ch as libc::c_int == 'E' as i32 {
        let mut i: libc::c_uint = 0;
        let mut status: libc::c_int = atoi(buf.as_mut_ptr().offset(1));
        if !(status < HTTP_CONTINUE as libc::c_int) {
          /* then error page; find matching status */
          i = 0i32 as libc::c_uint;
          while i
            < (::std::mem::size_of::<[u16; 11]>() as libc::c_ulong)
              .wrapping_div(::std::mem::size_of::<u16>() as libc::c_ulong)
              as libc::c_uint
          {
            if http_response_type[i as usize] as libc::c_int == status {
              /* We chdir to home_httpd, thus no need to
               * concat_path_file(home_httpd, after_colon)
               * here */
              (*ptr_to_globals).http_error_page[i as usize] = xstrdup(after_colon);
              break;
            } else {
              i = i.wrapping_add(1)
            }
          }
          continue;
        }
      } else if flag == FIRST_PARSE as libc::c_int && ch as libc::c_int == 'P' as i32 {
        /* P:/url:[http://]hostname[:port]/new/path */
        let mut url_from: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut host_port: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut url_to: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut proxy_entry: *mut Htaccess_Proxy = 0 as *mut Htaccess_Proxy;
        url_from = after_colon;
        host_port = strchr(after_colon, ':' as i32);
        if !host_port.is_null() {
          let fresh2 = host_port;
          host_port = host_port.offset(1);
          *fresh2 = '\u{0}' as i32 as libc::c_char;
          if !is_prefixed_with(
            host_port,
            b"http://\x00" as *const u8 as *const libc::c_char,
          )
          .is_null()
          {
            host_port = host_port.offset(7)
          }
          if !(*host_port as libc::c_int == '\u{0}' as i32) {
            url_to = strchr(host_port, '/' as i32);
            if !url_to.is_null() {
              *url_to = '\u{0}' as i32 as libc::c_char;
              proxy_entry = xzalloc(::std::mem::size_of::<Htaccess_Proxy>() as libc::c_ulong)
                as *mut Htaccess_Proxy;
              (*proxy_entry).url_from = xstrdup(url_from);
              (*proxy_entry).host_port = xstrdup(host_port);
              *url_to = '/' as i32 as libc::c_char;
              (*proxy_entry).url_to = xstrdup(url_to);
              (*proxy_entry).next = (*ptr_to_globals).proxy;
              (*ptr_to_globals).proxy = proxy_entry;
              continue;
            }
          }
        }
      } else {
        /* the rest of directives are non-alphabetic,
         * must avoid using "toupper'ed" ch */
        ch = buf[0] as libc::c_uchar;
        if ch as libc::c_int == '.' as i32
          || ch as libc::c_int == '*' as i32 && buf[1] as libc::c_int == '.' as i32
        {
          /* "*.php:/path/php" */
          let mut p_0: *mut libc::c_char = 0 as *mut libc::c_char;
          let mut cur: *mut Htaccess = 0 as *mut Htaccess;
          cur = xzalloc(
            (::std::mem::size_of::<Htaccess>() as libc::c_ulong)
              .wrapping_add(strlen_buf as libc::c_ulong),
          ) as *mut Htaccess;
          strcpy((*cur).before_colon.as_mut_ptr(), buf.as_mut_ptr());
          p_0 = (*cur)
            .before_colon
            .as_mut_ptr()
            .offset(after_colon.wrapping_offset_from(buf.as_mut_ptr()) as libc::c_long as isize);
          *p_0.offset(-1i32 as isize) = '\u{0}' as i32 as libc::c_char;
          (*cur).after_colon = p_0;
          if ch as libc::c_int == '.' as i32 {
            /* .mime line: prepend to mime_a list */
            (*cur).next = (*ptr_to_globals).mime_a;
            (*ptr_to_globals).mime_a = cur
          } else {
            /* script interpreter line: prepend to script_i list */
            (*cur).next = (*ptr_to_globals).script_i;
            (*ptr_to_globals).script_i = cur
          }
          continue;
        } else if ch as libc::c_int == '/' as i32 {
          /* "/file:user:pass" */
          let mut p_1: *mut libc::c_char = 0 as *mut libc::c_char;
          let mut cur_0: *mut Htaccess = 0 as *mut Htaccess;
          let mut file_len: libc::c_uint = 0;
          /* note: path is "" unless we are in SUBDIR parse,
           * otherwise it does NOT start with "/" */
          cur_0 = xzalloc(
            (::std::mem::size_of::<Htaccess>() as libc::c_ulong)
              .wrapping_add(1i32 as libc::c_ulong)
              .wrapping_add(strlen(path))
              .wrapping_add(strlen_buf as libc::c_ulong),
          ) as *mut Htaccess;
          /* form "/path/file" */
          sprintf(
            (*cur_0).before_colon.as_mut_ptr(),
            b"/%s%.*s\x00" as *const u8 as *const libc::c_char,
            path,
            (after_colon.wrapping_offset_from(buf.as_mut_ptr()) as libc::c_long - 1) as libc::c_int,
            buf.as_mut_ptr(),
          );
          /* canonicalize it */
          p_1 = bb_simplify_abs_path_inplace((*cur_0).before_colon.as_mut_ptr());
          file_len = p_1.wrapping_offset_from((*cur_0).before_colon.as_mut_ptr()) as libc::c_long
            as libc::c_uint;
          /* add "user:pass" after NUL */
          p_1 = p_1.offset(1);
          strcpy(p_1, after_colon);
          (*cur_0).after_colon = p_1;
          /* insert cur into g_auth */
          /* g_auth is sorted by decreased filename length */
          let mut auth: *mut Htaccess = 0 as *mut Htaccess;
          let mut authp: *mut *mut Htaccess = 0 as *mut *mut Htaccess;
          authp = &mut (*ptr_to_globals).g_auth;
          loop {
            auth = *authp;
            if auth.is_null() {
              break;
            }
            if file_len as libc::c_ulong >= strlen((*auth).before_colon.as_mut_ptr()) {
              /* insert cur before auth */
              (*cur_0).next = auth;
              break;
            } else {
              authp = &mut (*auth).next
            }
          }
          *authp = cur_0;
          continue;
        }
      }
    }
    bb_error_msg(
      b"config error \'%s\' in \'%s\'\x00" as *const u8 as *const libc::c_char,
      buf.as_mut_ptr(),
      filename,
    );
  }
  fclose(f);
}
/* BASIC_AUTH */
/* the line is not recognized */
/*
 * Given a string, html-encode special characters.
 * This is used for the -e command line option to provide an easy way
 * for scripts to encode result data without confusing browsers.  The
 * returned string pointer is memory allocated by malloc().
 *
 * Returns a pointer to the encoded string (malloced).
 */
unsafe extern "C" fn encodeString(mut string: *const libc::c_char) -> *mut libc::c_char {
  /* take the simple route and encode everything */
  /* could possibly scan once to get length.     */
  let mut len: libc::c_int = strlen(string) as libc::c_int;
  let mut out: *mut libc::c_char = xmalloc((len * 6i32 + 1i32) as size_t) as *mut libc::c_char;
  let mut p: *mut libc::c_char = out;
  let mut ch: libc::c_char = 0;
  loop {
    let fresh3 = string;
    string = string.offset(1);
    ch = *fresh3;
    if !(ch as libc::c_int != '\u{0}' as i32) {
      break;
    }
    /* very simple check for what to encode */
    if bb_ascii_isalnum(ch as libc::c_uchar) != 0 {
      let fresh4 = p;
      p = p.offset(1);
      *fresh4 = ch
    } else {
      p = p.offset(sprintf(
        p,
        b"&#%u;\x00" as *const u8 as *const libc::c_char,
        ch as libc::c_uchar as libc::c_int,
      ) as isize)
    }
  }
  *p = '\u{0}' as i32 as libc::c_char;
  return out;
}
/*
 * Decode a base64 data stream as per rfc1521.
 * Note that the rfc states that non base64 chars are to be ignored.
 * Since the decode always results in a shorter size than the input,
 * it is OK to pass the input arg as an output arg.
 * Parameter: a pointer to a base64 encoded string.
 * Decoded data is stored in-place.
 */
unsafe extern "C" fn decodeBase64(mut Data: *mut libc::c_char) {
  let mut in_0: *const libc::c_uchar = Data as *const libc::c_uchar;
  /* The decoded size will be at most 3/4 the size of the encoded */
  let mut ch: libc::c_uint = 0i32 as libc::c_uint;
  let mut i: libc::c_int = 0i32;
  while *in_0 != 0 {
    let fresh5 = in_0;
    in_0 = in_0.offset(1);
    let mut t: libc::c_int = *fresh5 as libc::c_int;
    if t >= '0' as i32 && t <= '9' as i32 {
      t = t - '0' as i32 + 52i32
    } else if t >= 'A' as i32 && t <= 'Z' as i32 {
      t = t - 'A' as i32
    } else if t >= 'a' as i32 && t <= 'z' as i32 {
      t = t - 'a' as i32 + 26i32
    } else if t == '+' as i32 {
      t = 62i32
    } else if t == '/' as i32 {
      t = 63i32
    } else {
      if !(t == '=' as i32) {
        continue;
      }
      t = 0i32
    }
    ch = ch << 6i32 | t as libc::c_uint;
    i += 1;
    if i == 4i32 {
      let fresh6 = Data;
      Data = Data.offset(1);
      *fresh6 = (ch >> 16i32) as libc::c_char;
      let fresh7 = Data;
      Data = Data.offset(1);
      *fresh7 = (ch >> 8i32) as libc::c_char;
      let fresh8 = Data;
      Data = Data.offset(1);
      *fresh8 = ch as libc::c_char;
      i = 0i32
    }
  }
  *Data = '\u{0}' as i32 as libc::c_char;
}
/*
 * Create a listen server socket on the designated port.
 */
unsafe extern "C" fn openServer() -> libc::c_int {
  let mut n: libc::c_uint = bb_strtou(
    (*ptr_to_globals).bind_addr_or_port,
    0 as *mut *mut libc::c_char,
    10i32,
  );
  if *bb_errno == 0 && n != 0 && n <= 0xffffi32 as libc::c_uint {
    n = create_and_bind_stream_or_die(0 as *const libc::c_char, n as libc::c_int) as libc::c_uint
  } else {
    n = create_and_bind_stream_or_die((*ptr_to_globals).bind_addr_or_port, 80i32) as libc::c_uint
  }
  xlisten(n as libc::c_int, 9i32);
  return n as libc::c_int;
}
/*
 * Log the connection closure and exit.
 */
unsafe extern "C" fn log_and_exit() -> ! {
  /* Paranoia. IE said to be buggy. It may send some extra data
   * or be confused by us just exiting without SHUT_WR. Oh well. */
  shutdown(1i32, SHUT_WR as libc::c_int);
  /* Why??
  (this also messes up stdin when user runs httpd -i from terminal)
  ndelay_on(0);
  while (read(STDIN_FILENO, iobuf, IOBUF_SIZE) > 0)
    continue;
  */
  if (*ptr_to_globals).verbose > 2i32 {
    bb_simple_error_msg(b"closed\x00" as *const u8 as *const libc::c_char);
  }
  _exit(xfunc_error_retval as libc::c_int);
}
/*
 * Create and send HTTP response headers.
 * The arguments are combined and sent as one write operation.  Note that
 * IE will puke big-time if the headers are not sent in one packet and the
 * second packet is delayed for any reason.
 * responseNum - the result code to send.
 */
unsafe extern "C" fn send_headers(mut responseNum: libc::c_uint) {
  static mut RFC1123FMT: [libc::c_char; 26] = [
    37, 97, 44, 32, 37, 100, 32, 37, 98, 32, 37, 89, 32, 37, 72, 58, 37, 77, 58, 37, 83, 32, 71,
    77, 84, 0,
  ];
  /* Fixed size 29-byte string. Example: Sun, 06 Nov 1994 08:49:37 GMT */
  let mut date_str: [libc::c_char; 40] = [0; 40]; /* using a bit larger buffer to paranoia reasons */
  let mut tm: tm =std::mem::zeroed();
  let mut responseString: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
  let mut infoString: *const libc::c_char = 0 as *const libc::c_char;
  let mut error_page: *const libc::c_char = 0 as *const libc::c_char;
  let mut len: libc::c_uint = 0;
  let mut i: libc::c_uint = 0;
  let mut timer: time_t = time(0 as *mut time_t);
  i = 0i32 as libc::c_uint;
  while i
    < (::std::mem::size_of::<[u16; 11]>() as libc::c_ulong)
      .wrapping_div(::std::mem::size_of::<u16>() as libc::c_ulong) as libc::c_uint
  {
    if http_response_type[i as usize] as libc::c_uint == responseNum {
      responseString = http_response[i as usize].name;
      infoString = http_response[i as usize].info;
      error_page = (*ptr_to_globals).http_error_page[i as usize];
      break;
    } else {
      i = i.wrapping_add(1)
    }
  }
  if (*ptr_to_globals).verbose != 0 {
    bb_error_msg(
      b"response:%u\x00" as *const u8 as *const libc::c_char,
      responseNum,
    );
  }
  /* We use sprintf, not snprintf (it's less code).
   * iobuf[] is several kbytes long and all headers we generate
   * always fit into those kbytes.
   */
  strftime(
    date_str.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong,
    RFC1123FMT.as_ptr(),
    gmtime_r(&mut timer, &mut tm),
  );
  /* ^^^ using gmtime_r() instead of gmtime() to not use static data */
  len = sprintf(
    (*ptr_to_globals).iobuf,
    b"HTTP/1.0 %u %s\r\nDate: %s\r\nConnection: close\r\n\x00" as *const u8 as *const libc::c_char,
    responseNum,
    responseString,
    date_str.as_mut_ptr(),
  ) as libc::c_uint;
  if responseNum != HTTP_OK as libc::c_int as libc::c_uint
    || !(*ptr_to_globals).found_mime_type.is_null()
  {
    len = len.wrapping_add(sprintf(
      (*ptr_to_globals).iobuf.offset(len as isize),
      b"Content-type: %s\r\n\x00" as *const u8 as *const libc::c_char,
      if responseNum != HTTP_OK as libc::c_int as libc::c_uint {
        b"text/html\x00" as *const u8 as *const libc::c_char
      } else {
        (*ptr_to_globals).found_mime_type
      },
    ) as libc::c_uint)
  }
  if responseNum == HTTP_UNAUTHORIZED as libc::c_int as libc::c_uint {
    len = len.wrapping_add(sprintf(
      (*ptr_to_globals).iobuf.offset(len as isize),
      b"WWW-Authenticate: Basic realm=\"%.999s\"\r\n\x00" as *const u8 as *const libc::c_char,
      (*ptr_to_globals).g_realm,
    ) as libc::c_uint)
  }
  if responseNum == HTTP_MOVED_TEMPORARILY as libc::c_int as libc::c_uint {
    /* Responding to "GET /dir" with
     * "HTTP/1.0 302 Found" "Location: /dir/"
     * - IOW, asking them to repeat with a slash.
     * Here, overflow IS possible, can't use sprintf:
     * mkdir test
     * python -c 'print("get /test?" + ("x" * 8192))' | busybox httpd -i -h .
     */
    len = len.wrapping_add(snprintf(
      (*ptr_to_globals).iobuf.offset(len as isize),
      ((8192i32 - 3i32) as libc::c_uint).wrapping_sub(len) as libc::c_ulong,
      b"Location: %s/%s%s\r\n\x00" as *const u8 as *const libc::c_char,
      (*ptr_to_globals).found_moved_temporarily,
      if !(*ptr_to_globals).g_query.is_null() {
        b"?\x00" as *const u8 as *const libc::c_char
      } else {
        b"\x00" as *const u8 as *const libc::c_char
      },
      if !(*ptr_to_globals).g_query.is_null() {
        (*ptr_to_globals).g_query
      } else {
        b"\x00" as *const u8 as *const libc::c_char
      },
    ) as libc::c_uint);
    if len > (8192i32 - 3i32) as libc::c_uint {
      len = (8192i32 - 3i32) as libc::c_uint
    }
  }
  if !error_page.is_null() && access(error_page, 4i32) == 0i32 {
    let fresh9 = len;
    len = len.wrapping_add(1);
    *(*ptr_to_globals).iobuf.offset(fresh9 as isize) = '\r' as i32 as libc::c_char;
    let fresh10 = len;
    len = len.wrapping_add(1);
    *(*ptr_to_globals).iobuf.offset(fresh10 as isize) = '\n' as i32 as libc::c_char;
    full_write(
      1i32,
      (*ptr_to_globals).iobuf as *const libc::c_void,
      len as size_t,
    );
    send_file_and_exit(error_page, SEND_BODY as libc::c_int);
  }
  if (*ptr_to_globals).file_size != -1i32 as libc::c_long {
    /* file */
    strftime(
      date_str.as_mut_ptr(),
      ::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong,
      RFC1123FMT.as_ptr(),
      gmtime_r(&mut (*ptr_to_globals).last_mod, &mut tm),
    );
    if responseNum == HTTP_PARTIAL_CONTENT as libc::c_int as libc::c_uint {
      len = len.wrapping_add(sprintf(
        (*ptr_to_globals).iobuf.offset(len as isize),
        b"Content-Range: bytes %lu-%lu/%lu\r\n\x00" as *const u8 as *const libc::c_char,
        (*ptr_to_globals).range_start,
        (*ptr_to_globals).range_end,
        (*ptr_to_globals).file_size,
      ) as libc::c_uint);
      (*ptr_to_globals).file_size = (*ptr_to_globals).range_end - (*ptr_to_globals).range_start + 1
    }
    //RFC 2616 4.4 Message Length
    // The transfer-length of a message is the length of the message-body as
    // it appears in the message; that is, after any transfer-codings have
    // been applied. When a message-body is included with a message, the
    // transfer-length of that body is determined by one of the following
    // (in order of precedence):
    // 1.Any response message which "MUST NOT" include a message-body (such
    //   as the 1xx, 204, and 304 responses and any response to a HEAD
    //   request) is always terminated by the first empty line after the
    //   header fields, regardless of the entity-header fields present in
    //   the message.
    // 2.If a Transfer-Encoding header field (section 14.41) is present and
    //   has any value other than "identity", then the transfer-length is
    //   defined by use of the "chunked" transfer-coding (section 3.6),
    //   unless the message is terminated by closing the connection.
    // 3.If a Content-Length header field (section 14.13) is present, its
    //   decimal value in OCTETs represents both the entity-length and the
    //   transfer-length. The Content-Length header field MUST NOT be sent
    //   if these two lengths are different (i.e., if a Transfer-Encoding
    //   header field is present). If a message is received with both a
    //   Transfer-Encoding header field and a Content-Length header field,
    //   the latter MUST be ignored.
    // 4.If the message uses the media type "multipart/byteranges" ...
    // 5.By the server closing the connection.
    //
    // (NB: standards do not define "Transfer-Length:" _header_,
    // transfer-length above is just a concept).
    len = len.wrapping_add(sprintf(
      (*ptr_to_globals).iobuf.offset(len as isize),
      b"Accept-Ranges: bytes\r\nLast-Modified: %s\r\nContent-Length: %lu\r\n\x00" as *const u8
        as *const libc::c_char,
      date_str.as_mut_ptr(),
      (*ptr_to_globals).file_size,
    ) as libc::c_uint)
  }
  /* This should be "Transfer-Encoding", not "Content-Encoding":
   * "data is compressed for transfer", not "data is an archive".
   * But many clients were not handling "Transfer-Encoding" correctly
   * (they were not uncompressing gzipped pages, tried to show
   * raw compressed data), and servers worked around it by using
   * "Content-Encoding" instead... and this become de-facto standard.
   * https://bugzilla.mozilla.org/show_bug.cgi?id=68517
   * https://bugs.chromium.org/p/chromium/issues/detail?id=94730
   */
  if (*ptr_to_globals).content_gzip != 0 {
    len = len.wrapping_add(sprintf(
      (*ptr_to_globals).iobuf.offset(len as isize),
      b"Content-Encoding: gzip\r\n\x00" as *const u8 as *const libc::c_char,
    ) as libc::c_uint)
  }
  let fresh11 = len;
  len = len.wrapping_add(1);
  *(*ptr_to_globals).iobuf.offset(fresh11 as isize) = '\r' as i32 as libc::c_char;
  let fresh12 = len;
  len = len.wrapping_add(1);
  *(*ptr_to_globals).iobuf.offset(fresh12 as isize) = '\n' as i32 as libc::c_char;
  if !infoString.is_null() {
    len = len.wrapping_add(sprintf(
      (*ptr_to_globals).iobuf.offset(len as isize),
      b"<HTML><HEAD><TITLE>%u %s</TITLE></HEAD>\n<BODY><H1>%u %s</H1>\n%s\n</BODY></HTML>\n\x00"
        as *const u8 as *const libc::c_char,
      responseNum,
      responseString,
      responseNum,
      responseString,
      infoString,
    ) as libc::c_uint)
  }
  if full_write(
    1,
    (*ptr_to_globals).iobuf as *const libc::c_void,
    len as size_t,
  ) != len as isize
  {
    if (*ptr_to_globals).verbose > 1i32 {
      bb_simple_perror_msg(b"error\x00" as *const u8 as *const libc::c_char);
    }
    log_and_exit();
  };
}
unsafe extern "C" fn send_headers_and_exit(mut responseNum: libc::c_int) -> ! {
  (*ptr_to_globals).content_gzip = 0i32 as smallint;
  send_headers(responseNum as libc::c_uint);
  log_and_exit();
}
/*
 * Read from the socket until '\n' or EOF.
 * '\r' chars are removed.
 * '\n' is replaced with NUL.
 * Return number of characters read or 0 if nothing is read
 * ('\r' and '\n' are not counted).
 * Data is returned in iobuf.
 */
unsafe extern "C" fn get_line() -> libc::c_uint {
  let mut count: libc::c_uint = 0;
  let mut c: libc::c_char = 0;
  count = 0i32 as libc::c_uint;
  loop {
    if (*ptr_to_globals).hdr_cnt <= 0i32 {
      alarm(60i32 as libc::c_uint);
      (*ptr_to_globals).hdr_cnt = safe_read(
        0i32,
        bb_common_bufsiz1.as_mut_ptr() as *mut libc::c_void,
        COMMON_BUFSIZE as libc::c_int as size_t,
      ) as libc::c_int;
      if (*ptr_to_globals).hdr_cnt <= 0i32 {
        break;
      }
      (*ptr_to_globals).hdr_ptr = bb_common_bufsiz1.as_mut_ptr()
    }
    (*ptr_to_globals).hdr_cnt -= 1;
    let fresh13 = (*ptr_to_globals).hdr_ptr;
    (*ptr_to_globals).hdr_ptr = (*ptr_to_globals).hdr_ptr.offset(1);
    c = *fresh13;
    if c as libc::c_int == '\r' as i32 {
      continue;
    }
    if c as libc::c_int == '\n' as i32 {
      break;
    }
    *(*ptr_to_globals).iobuf.offset(count as isize) = c;
    if count < (8192i32 - 1i32) as libc::c_uint {
      /* check overflow */
      count = count.wrapping_add(1)
    }
  }
  *(*ptr_to_globals).iobuf.offset(count as isize) = '\u{0}' as i32 as libc::c_char;
  return count;
}
/* gcc 4.2.1 fares better with NOINLINE */
#[inline(never)]
unsafe extern "C" fn cgi_io_loop_and_exit(
  mut fromCgi_rd: libc::c_int,
  mut toCgi_wr: libc::c_int,
  mut post_len: libc::c_int,
) -> ! {
  let mut pfd: [pollfd; 3] = [pollfd {
    fd: 0,
    events: 0,
    revents: 0,
  }; 3]; /* indexes in pfd[] */
  let mut out_cnt: libc::c_int = 0; /* we buffer a bit of initial CGI output */
  let mut count: libc::c_int = 0;
  /* iobuf is used for CGI -> network data,
   * hdr_buf is for network -> CGI data (POSTDATA) */
  /* If CGI dies, we still want to correctly finish reading its output
   * and send it to the peer. So please no SIGPIPEs! */
  signal(
    13i32,
    ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(1i32 as libc::intptr_t),
  );
  // We inconsistently handle a case when more POSTDATA from network
  // is coming than we expected. We may give *some part* of that
  // extra data to CGI.
  //if (hdr_cnt > post_len) {
  //	/* We got more POSTDATA from network than we expected */
  //	hdr_cnt = post_len;
  //}
  post_len -= (*ptr_to_globals).hdr_cnt;
  /* post_len - number of POST bytes not yet read from network */
  /* NB: breaking out of this loop jumps to log_and_exit() */
  out_cnt = 0i32; /* while (1) */
  pfd[FROM_CGI as libc::c_int as usize].fd = fromCgi_rd;
  pfd[FROM_CGI as libc::c_int as usize].events = 0x1i32 as libc::c_short;
  pfd[TO_CGI as libc::c_int as usize].fd = toCgi_wr;
  loop
  /* Note: even pfd[0].events == 0 won't prevent
   * revents == POLLHUP|POLLERR reports from closed stdin.
   * Setting fd to -1 works: */
  {
    pfd[0].fd = -1i32; /* probably not needed, paranoia */
    pfd[0].events = 0x1i32 as libc::c_short;
    pfd[0].revents = 0i32 as libc::c_short;
    /* We always poll this fd, thus kernel always sets revents: */
    /*pfd[FROM_CGI].events = POLLIN; - moved out of loop */
    /*pfd[FROM_CGI].revents = 0; - not needed */
    /* gcc-4.8.0 still doesnt fill two shorts with one insn :( */
    /* http://gcc.gnu.org/bugzilla/show_bug.cgi?id=47059 */
    /* hopefully one day it will... */
    pfd[TO_CGI as libc::c_int as usize].events = 0x4i32 as libc::c_short; /* needed! */
    pfd[TO_CGI as libc::c_int as usize].revents = 0i32 as libc::c_short;
    if toCgi_wr != 0 && (*ptr_to_globals).hdr_cnt <= 0i32 {
      if post_len > 0i32 {
        /* Expect more POST data from network */
        pfd[0].fd = 0i32
      } else {
        /* post_len <= 0 && hdr_cnt <= 0:
         * no more POST data to CGI,
         * let CGI see EOF on CGI's stdin */
        if toCgi_wr != fromCgi_rd {
          close(toCgi_wr);
        }
        toCgi_wr = 0i32
      }
    }
    /* Now wait on the set of sockets */
    count = safe_poll(
      pfd.as_mut_ptr(),
      if (*ptr_to_globals).hdr_cnt > 0i32 {
        (TO_CGI as libc::c_int) + 1i32
      } else {
        (FROM_CGI as libc::c_int) + 1i32
      } as nfds_t,
      -1i32,
    );
    if count <= 0i32 {
      break;
    }
    if pfd[TO_CGI as libc::c_int as usize].revents != 0 {
      /* hdr_cnt > 0 here due to the way poll() called */
      /* Have data from peer and can write to CGI */
      count = safe_write(
        toCgi_wr,
        (*ptr_to_globals).hdr_ptr as *const libc::c_void,
        (*ptr_to_globals).hdr_cnt as size_t,
      ) as libc::c_int;
      /* Doesn't happen, we dont use nonblocking IO here
       *if (count < 0 && errno == EAGAIN) {
       *	...
       *} else */
      if count > 0i32 {
        (*ptr_to_globals).hdr_ptr = (*ptr_to_globals).hdr_ptr.offset(count as isize);
        (*ptr_to_globals).hdr_cnt -= count
      } else {
        /* EOF/broken pipe to CGI, stop piping POST data */
        post_len = 0i32;
        (*ptr_to_globals).hdr_cnt = post_len
      }
    }
    if pfd[0].revents != 0 {
      /* post_len > 0 && hdr_cnt == 0 here */
      /* We expect data, prev data portion is eaten by CGI
       * and there *is* data to read from the peer
       * (POSTDATA) */
      //count = post_len > (int)sizeof_hdr_buf ? (int)sizeof_hdr_buf : post_len;
      //count = safe_read(STDIN_FILENO, hdr_buf, count);
      count = safe_read(
        0i32,
        bb_common_bufsiz1.as_mut_ptr() as *mut libc::c_void,
        COMMON_BUFSIZE as libc::c_int as size_t,
      ) as libc::c_int;
      if count > 0i32 {
        (*ptr_to_globals).hdr_cnt = count;
        (*ptr_to_globals).hdr_ptr = bb_common_bufsiz1.as_mut_ptr();
        post_len -= count
      } else {
        /* no more POST data can be read */
        post_len = 0i32
      }
    }
    if !(pfd[FROM_CGI as libc::c_int as usize].revents != 0) {
      continue;
    }
    /* There is something to read from CGI */
    let mut rbuf: *mut libc::c_char = (*ptr_to_globals).iobuf;
    /* Are we still buffering CGI output? */
    if out_cnt >= 0i32 {
      /* HTTP_200[] has single "\r\n" at the end.
       * According to http://hoohoo.ncsa.uiuc.edu/cgi/out.html,
       * CGI scripts MUST send their own header terminated by
       * empty line, then data. That's why we have only one
       * <cr><lf> pair here. We will output "200 OK" line
       * if needed, but CGI still has to provide blank line
       * between header and body */
      /* Must use safe_read, not full_read, because
       * CGI may output a few first bytes and then wait
       * for POSTDATA without closing stdout.
       * With full_read we may wait here forever. */
      count = safe_read(
        fromCgi_rd,
        rbuf.offset(out_cnt as isize) as *mut libc::c_void,
        (8192i32 - 8i32) as size_t,
      ) as libc::c_int;
      if count <= 0i32 {
        /* eof (or error) and there was no "HTTP",
         * send "HTTP/1.0 200 OK\r\n", then send received data */
        if out_cnt != 0 {
          full_write(
            1i32,
            HTTP_200.as_ptr() as *const libc::c_void,
            (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
              .wrapping_sub(1i32 as libc::c_ulong),
          );
          full_write(1i32, rbuf as *const libc::c_void, out_cnt as size_t);
        }
        break;
      /* CGI stdout is closed, exiting */
      } else {
        out_cnt += count;
        count = 0i32;
        /* "Status" header format is: "Status: 302 Redirected\r\n" */
        if out_cnt >= 8i32
          && memcmp(
            rbuf as *const libc::c_void,
            b"Status: \x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            8i32 as libc::c_ulong,
          ) == 0i32
        {
          /* send "HTTP/1.0 " */
          if full_write(1, HTTP_200.as_ptr() as *const libc::c_void, 9) != 9 {
            break;
          }
          /* skip "Status: " (including space, sending "HTTP/1.0  NNN" is wrong) */
          rbuf = rbuf.offset(8);
          count = out_cnt - 8i32;
          out_cnt = -1i32
        /* buffering off */
        } else if out_cnt >= 4i32 {
          /* Did CGI add "HTTP"? */
          if memcmp(
            rbuf as *const libc::c_void,
            HTTP_200.as_ptr() as *const libc::c_void,
            4i32 as libc::c_ulong,
          ) != 0i32
          {
            /* there is no "HTTP", do it ourself */
            if full_write(
              1i32,
              HTTP_200.as_ptr() as *const libc::c_void,
              (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
                .wrapping_sub(1i32 as libc::c_ulong),
            ) as libc::c_ulong
              != (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
                .wrapping_sub(1i32 as libc::c_ulong)
            {
              break;
            }
          }
          /* Commented out:
          if (!strstr(rbuf, "ontent-")) {
            full_write(s, "Content-type: text/plain\r\n\r\n", 28);
          }
           * Counter-example of valid CGI without Content-type:
           * echo -en "HTTP/1.0 302 Found\r\n"
           * echo -en "Location: http://www.busybox.net\r\n"
           * echo -en "\r\n"
           */
          count = out_cnt;
          out_cnt = -1i32
          /* buffering off */
        }
      }
    } else {
      count = safe_read(fromCgi_rd, rbuf as *mut libc::c_void, 8192i32 as size_t) as libc::c_int;
      if count <= 0i32 {
        break;
      }
      /* eof (or error) */
    }
    if full_write(1, rbuf as *const libc::c_void, count as size_t) != count as isize {
      break;
    }
  }
  log_and_exit();
}
unsafe extern "C" fn setenv1(mut name: *const libc::c_char, mut value: *const libc::c_char) {
  setenv(
    name,
    if !value.is_null() {
      value
    } else {
      b"\x00" as *const u8 as *const libc::c_char
    },
    1i32,
  );
}
/*
 * Spawn CGI script, forward CGI's stdin/out <=> network
 *
 * Environment variables are set up and the script is invoked with pipes
 * for stdin/stdout.  If a POST is being done the script is fed the POST
 * data in addition to setting the QUERY_STRING variable (for GETs or POSTs).
 *
 * Parameters:
 * const char *url              The requested URL (with leading /).
 * const char *orig_uri         The original URI before rewriting (if any)
 * int post_len                 Length of the POST body.
 */
unsafe extern "C" fn send_cgi_and_exit(
  mut url: *const libc::c_char,
  mut orig_uri: *const libc::c_char,
  mut request: *const libc::c_char,
  mut post_len: libc::c_int,
) -> ! {
  let mut fromCgi: fd_pair = fd_pair { rd: 0, wr: 0 }; /* CGI -> httpd pipe */
  let mut toCgi: fd_pair = fd_pair { rd: 0, wr: 0 }; /* httpd -> CGI pipe */
  let mut script: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut last_slash: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut pid: libc::c_int = 0;
  /* Make a copy. NB: caller guarantees:
   * url[0] == '/', url[1] != '/' */
  url = xstrdup(url);
  /*
   * We are mucking with environment _first_ and then vfork/exec,
   * this allows us to use vfork safely. Parent doesn't care about
   * these environment changes anyway.
   */
  /* Check for [dirs/]script.cgi/PATH_INFO */
  script = url as *mut libc::c_char;
  last_slash = script;
  loop {
    script = strchr(script.offset(1), '/' as i32);
    if script.is_null() {
      break;
    }
    let mut dir: libc::c_int = 0;
    *script = '\u{0}' as i32 as libc::c_char;
    dir = is_directory(url.offset(1), 1i32);
    *script = '/' as i32 as libc::c_char;
    if dir == 0 {
      break;
    }
    /* is directory, find next '/' */
    last_slash = script
  } /* set to /PATH_INFO or "" */
  setenv1(b"PATH_INFO\x00" as *const u8 as *const libc::c_char, script); /* cut off /PATH_INFO */
  setenv1(
    b"REQUEST_METHOD\x00" as *const u8 as *const libc::c_char,
    request,
  );
  if !(*ptr_to_globals).g_query.is_null() {
    putenv(xasprintf(
      b"%s=%s?%s\x00" as *const u8 as *const libc::c_char,
      b"REQUEST_URI\x00" as *const u8 as *const libc::c_char,
      orig_uri,
      (*ptr_to_globals).g_query,
    ));
  } else {
    setenv1(
      b"REQUEST_URI\x00" as *const u8 as *const libc::c_char,
      orig_uri,
    );
  }
  if !script.is_null() {
    *script = '\u{0}' as i32 as libc::c_char
  }
  /* SCRIPT_FILENAME is required by PHP in CGI mode */
  if *(*ptr_to_globals).home_httpd.offset(0) as libc::c_int == '/' as i32 {
    let mut fullpath: *mut libc::c_char = concat_path_file((*ptr_to_globals).home_httpd, url);
    setenv1(
      b"SCRIPT_FILENAME\x00" as *const u8 as *const libc::c_char,
      fullpath,
    );
  }
  /* set SCRIPT_NAME as full path: /cgi-bin/dirs/script.cgi */
  setenv1(b"SCRIPT_NAME\x00" as *const u8 as *const libc::c_char, url);
  /* http://hoohoo.ncsa.uiuc.edu/cgi/env.html:
   * QUERY_STRING: The information which follows the ? in the URL
   * which referenced this script. This is the query information.
   * It should not be decoded in any fashion. This variable
   * should always be set when there is query information,
   * regardless of command line decoding. */
  /* (Older versions of bbox seem to do some decoding) */
  setenv1(
    b"QUERY_STRING\x00" as *const u8 as *const libc::c_char,
    (*ptr_to_globals).g_query,
  );
  putenv(
    b"SERVER_SOFTWARE=busybox httpd/1.32.0.git\x00" as *const u8 as *const libc::c_char
      as *mut libc::c_char,
  );
  putenv(b"SERVER_PROTOCOL=HTTP/1.0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
  putenv(b"GATEWAY_INTERFACE=CGI/1.1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
  /* Having _separate_ variables for IP and port defeats
   * the purpose of having socket abstraction. Which "port"
   * are you using on Unix domain socket?
   * IOW - REMOTE_PEER="1.2.3.4:56" makes much more sense.
   * Oh well... */
  let mut p: *mut libc::c_char = if !(*ptr_to_globals).rmt_ip_str.is_null() {
    (*ptr_to_globals).rmt_ip_str
  } else {
    b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
  }; /* delete :PORT */
  let mut cp: *mut libc::c_char = strrchr(p, ':' as i32);
  if 1i32 != 0 && !cp.is_null() && !strchr(cp, ']' as i32).is_null() {
    cp = 0 as *mut libc::c_char
  }
  if !cp.is_null() {
    *cp = '\u{0}' as i32 as libc::c_char
  }
  setenv1(b"REMOTE_ADDR\x00" as *const u8 as *const libc::c_char, p);
  if !cp.is_null() {
    *cp = ':' as i32 as libc::c_char;
    setenv1(
      b"REMOTE_PORT\x00" as *const u8 as *const libc::c_char,
      cp.offset(1),
    );
  }
  if post_len != 0 {
    putenv(xasprintf(
      b"CONTENT_LENGTH=%u\x00" as *const u8 as *const libc::c_char,
      post_len,
    ));
  }
  if !(*ptr_to_globals).remoteuser.is_null() {
    setenv1(
      b"REMOTE_USER\x00" as *const u8 as *const libc::c_char,
      (*ptr_to_globals).remoteuser,
    );
    putenv(b"AUTH_TYPE=Basic\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
  }
  /* setenv1("SERVER_NAME", safe_gethostname()); - don't do this,
   * just run "env SERVER_NAME=xyz httpd ..." instead */
  xpipe(&mut fromCgi.rd);
  xpipe(&mut toCgi.rd);
  pid = vfork();
  if pid < 0i32 {
    /* TODO: log perror? */
    log_and_exit(); /* end child */
  }
  if pid == 0i32 {
    let mut current_block_69: u64;
    /* Child process */
    let mut argv: [*mut libc::c_char; 3] = [0 as *mut libc::c_char; 3];
    xfunc_error_retval = 242i32 as u8;
    /* NB: close _first_, then move fds! */
    close(toCgi.wr); /* replace stdin with the pipe */
    close(fromCgi.rd); /* replace stdout with the pipe */
    xmove_fd(toCgi.rd, 0i32);
    xmove_fd(fromCgi.wr, 1i32);
    /* User seeing stderr output can be a security problem.
     * If CGI really wants that, it can always do dup itself. */
    /* dup2(1, 2); */
    /* Chdiring to script's dir */
    script = last_slash;
    if script != url as *mut libc::c_char {
      /* paranoia */
      *script = '\u{0}' as i32 as libc::c_char;
      if chdir(url.offset(1)) != 0i32 {
        bb_perror_msg(
          b"can\'t change directory to \'%s\'\x00" as *const u8 as *const libc::c_char,
          url.offset(1),
        );
        current_block_69 = 12133829526579767928;
      } else {
        current_block_69 = 6560072651652764009;
      }
    // not needed: *script = '/';
    } else {
      current_block_69 = 6560072651652764009;
    }
    match current_block_69 {
      6560072651652764009 => {
        script = script.offset(1);
        /* set argv[0] to name without path */
        argv[0] = script;
        argv[1] = 0 as *mut libc::c_char;
        let mut suffix: *mut libc::c_char = strrchr(script, '.' as i32);
        if !suffix.is_null() {
          let mut cur: *mut Htaccess = 0 as *mut Htaccess;
          cur = (*ptr_to_globals).script_i;
          while !cur.is_null() {
            if strcmp((*cur).before_colon.as_mut_ptr().offset(1), suffix) == 0i32 {
              /* found interpreter name */
              argv[0] = (*cur).after_colon;
              argv[1] = script;
              argv[2] = 0 as *mut libc::c_char;
              break;
            } else {
              cur = (*cur).next
            }
          }
        }
        /* restore default signal dispositions for CGI process */
        bb_signals(0i32 | 1i32 << 17i32 | 1i32 << 13i32 | 1i32 << 1i32, None);
        /* _NOT_ execvp. We do not search PATH. argv[0] is a filename
         * without any dir components and will only match a file
         * in the current directory */
        execv(argv[0], argv.as_mut_ptr() as *const *mut libc::c_char);
        if (*ptr_to_globals).verbose != 0 {
          bb_perror_msg(
            b"can\'t execute \'%s\'\x00" as *const u8 as *const libc::c_char,
            argv[0],
          );
        }
      }
      _ => {}
    }
    /* send to stdout
     * (we are CGI here, our stdout is pumped to the net) */
    send_headers_and_exit(HTTP_NOT_FOUND as libc::c_int);
  }
  /* Parent process */
  /* Restore variables possibly changed by child */
  xfunc_error_retval = 0i32 as u8;
  /* Pump data */
  close(fromCgi.wr);
  close(toCgi.rd);
  cgi_io_loop_and_exit(fromCgi.rd, toCgi.wr, post_len);
}
/* FEATURE_HTTPD_CGI */
/*
 * Send a file response to a HTTP request, and exit
 *
 * Parameters:
 * const char *url  The requested URL (with leading /).
 * what             What to send (headers/body/both).
 */
#[inline(never)]
unsafe extern "C" fn send_file_and_exit(mut url: *const libc::c_char, mut what: libc::c_int) -> ! {
  let mut current_block: u64;
  let mut suffix: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut fd: libc::c_int = 0;
  let mut count: ssize_t = 0;
  if (*ptr_to_globals).content_gzip != 0 {
    /* does <url>.gz exist? Then use it instead */
    let mut gzurl: *mut libc::c_char =
      xasprintf(b"%s.gz\x00" as *const u8 as *const libc::c_char, url);
    fd = open(gzurl, 0i32);
    free(gzurl as *mut libc::c_void);
    if fd != -1i32 {
      let mut sb: stat = std::mem::zeroed();
      fstat(fd, &mut sb);
      (*ptr_to_globals).file_size = sb.st_size;
      (*ptr_to_globals).last_mod = sb.st_mtime
    } else {
      (*ptr_to_globals).content_gzip = 0i32 as smallint;
      fd = open(url, 0i32)
    }
  } else {
    fd = open(url, 0i32)
  }
  if fd < 0i32 {
    /* Error pages are sent by using send_file_and_exit(SEND_BODY).
     * IOW: it is unsafe to call send_headers_and_exit
     * if what is SEND_BODY! Can recurse! */
    if what != SEND_BODY as libc::c_int {
      send_headers_and_exit(HTTP_NOT_FOUND as libc::c_int);
    }
    log_and_exit();
  }
  /* If you want to know about EPIPE below
   * (happens if you abort downloads from local httpd): */
  signal(
    13i32,
    ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(1i32 as libc::intptr_t),
  );
  /* If not found, default is to not send "Content-type:" */
  /*found_mime_type = NULL; - already is */
  suffix = strrchr(url, '.' as i32);
  if !suffix.is_null() {
    static mut suffixTable: [libc::c_char; 277] = [
      46, 116, 120, 116, 46, 104, 46, 99, 46, 99, 99, 46, 99, 112, 112, 0, 116, 101, 120, 116, 47,
      112, 108, 97, 105, 110, 0, 46, 104, 116, 109, 46, 104, 116, 109, 108, 0, 116, 101, 120, 116,
      47, 104, 116, 109, 108, 0, 46, 106, 112, 103, 46, 106, 112, 101, 103, 0, 105, 109, 97, 103,
      101, 47, 106, 112, 101, 103, 0, 46, 103, 105, 102, 0, 105, 109, 97, 103, 101, 47, 103, 105,
      102, 0, 46, 112, 110, 103, 0, 105, 109, 97, 103, 101, 47, 112, 110, 103, 0, 46, 115, 118,
      103, 0, 105, 109, 97, 103, 101, 47, 115, 118, 103, 43, 120, 109, 108, 0, 46, 99, 115, 115, 0,
      116, 101, 120, 116, 47, 99, 115, 115, 0, 46, 106, 115, 0, 97, 112, 112, 108, 105, 99, 97,
      116, 105, 111, 110, 47, 106, 97, 118, 97, 115, 99, 114, 105, 112, 116, 0, 46, 119, 97, 118,
      0, 97, 117, 100, 105, 111, 47, 119, 97, 118, 0, 46, 97, 118, 105, 0, 118, 105, 100, 101, 111,
      47, 120, 45, 109, 115, 118, 105, 100, 101, 111, 0, 46, 113, 116, 46, 109, 111, 118, 0, 118,
      105, 100, 101, 111, 47, 113, 117, 105, 99, 107, 116, 105, 109, 101, 0, 46, 109, 112, 101, 46,
      109, 112, 101, 103, 0, 118, 105, 100, 101, 111, 47, 109, 112, 101, 103, 0, 46, 109, 105, 100,
      46, 109, 105, 100, 105, 0, 97, 117, 100, 105, 111, 47, 109, 105, 100, 105, 0, 46, 109, 112,
      51, 0, 97, 117, 100, 105, 111, 47, 109, 112, 101, 103, 0, 0,
    ];
    let mut cur: *mut Htaccess = 0 as *mut Htaccess;
    /* unpopular */
    /* compiler adds another "\0" here */
    let mut table: *const libc::c_char = suffixTable.as_ptr();
    let mut table_next: *const libc::c_char = 0 as *const libc::c_char;
    while *table != 0 {
      let mut try_suffix: *const libc::c_char = 0 as *const libc::c_char;
      let mut mime_type: *const libc::c_char = 0 as *const libc::c_char;
      mime_type = table.offset(strlen(table) as isize).offset(1);
      table_next = mime_type.offset(strlen(mime_type) as isize).offset(1);
      try_suffix = strstr(table, suffix);
      if try_suffix.is_null() {
        table = table_next
      } else {
        try_suffix = try_suffix.offset(strlen(suffix) as isize);
        if !(*try_suffix as libc::c_int == '\u{0}' as i32
          || *try_suffix as libc::c_int == '.' as i32)
        {
          break;
        }
        (*ptr_to_globals).found_mime_type = mime_type;
        break;
      }
    }
    cur = (*ptr_to_globals).mime_a;
    while !cur.is_null() {
      if strcmp((*cur).before_colon.as_mut_ptr(), suffix) == 0i32 {
        (*ptr_to_globals).found_mime_type = (*cur).after_colon;
        break;
      } else {
        cur = (*cur).next
      }
    }
  }
  if what == SEND_BODY as libc::c_int || (*ptr_to_globals).content_gzip as libc::c_int != 0 {
    /* Examine built-in table */
    /* ...then user's table */
    /* we are sending compressed page: can't do ranges */
    // /why?
    (*ptr_to_globals).range_start = -1i32 as off_t
  }
  (*ptr_to_globals).range_len = if -1i32 as off_t > 0 {
    -1i32 as off_t
  } else {
    !((1i32 as off_t)
      << (::std::mem::size_of::<off_t>() as libc::c_ulong)
        .wrapping_mul(8i32 as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong))
  };
  if (*ptr_to_globals).range_start >= 0 {
    if (*ptr_to_globals).range_end == 0
      || (*ptr_to_globals).range_end > (*ptr_to_globals).file_size - 1
    {
      (*ptr_to_globals).range_end = (*ptr_to_globals).file_size - 1
    }
    if (*ptr_to_globals).range_end < (*ptr_to_globals).range_start
      || lseek(fd, (*ptr_to_globals).range_start, 0i32) != (*ptr_to_globals).range_start
    {
      lseek(fd, 0i32 as off64_t, 0i32);
      (*ptr_to_globals).range_start = -1i32 as off_t
    } else {
      (*ptr_to_globals).range_len =
        (*ptr_to_globals).range_end - (*ptr_to_globals).range_start + 1i32 as libc::c_long;
      send_headers(HTTP_PARTIAL_CONTENT as libc::c_int as libc::c_uint);
      what = SEND_BODY as libc::c_int
    }
  }
  if what & SEND_HEADERS as libc::c_int != 0 {
    send_headers(HTTP_OK as libc::c_int as libc::c_uint);
  }
  let mut offset: off_t = (*ptr_to_globals).range_start;
  loop
  /* sz is rounded down to 64k */
  {
    let mut sz: ssize_t = (if -1 > 0 {
      -1
    } else {
      !(1
        << (::std::mem::size_of::<ssize_t>())
          .wrapping_mul(8)
          .wrapping_sub(1))
    }) - 0xffff; /* fall back to read/write loop */
    if sz > (*ptr_to_globals).range_len as isize {
      sz = (*ptr_to_globals).range_len as isize
    }
    count = sendfile(1i32, fd, &mut offset, sz as size_t);
    if count < 0 {
      if offset == (*ptr_to_globals).range_start {
        current_block = 4746626699541760585;
        break;
      } else {
        current_block = 13828257217690466024;
        break;
      }
    } else {
      (*ptr_to_globals).range_len -= count as i64;
      if count == 0 || (*ptr_to_globals).range_len == 0 {
        log_and_exit();
      }
    }
  }
  match current_block {
    4746626699541760585 => {
      loop {
        count = safe_read(
          fd,
          (*ptr_to_globals).iobuf as *mut libc::c_void,
          8192i32 as size_t,
        );
        if !(count > 0) {
          break;
        }
        let mut n: ssize_t = 0;
        if count > (*ptr_to_globals).range_len as isize {
          count = (*ptr_to_globals).range_len as isize
        }
        n = full_write(
          1i32,
          (*ptr_to_globals).iobuf as *const libc::c_void,
          count as size_t,
        );
        if count != n {
          break;
        }
        (*ptr_to_globals).range_len -= count as i64;
        if (*ptr_to_globals).range_len == 0 {
          break;
        }
      }
      if count < 0 {
        current_block = 13828257217690466024;
      } else {
        current_block = 10213293998891106930;
      }
    }
    _ => {}
  }
  match current_block {
    13828257217690466024 => {
      if (*ptr_to_globals).verbose > 1i32 {
        bb_simple_perror_msg(b"error\x00" as *const u8 as *const libc::c_char);
      }
    }
    _ => {}
  }
  log_and_exit();
}
unsafe extern "C" fn if_ip_denied_send_HTTP_FORBIDDEN_and_exit(mut remote_ip: libc::c_uint) {
  let mut cur: *mut Htaccess_IP = 0 as *mut Htaccess_IP;
  cur = (*ptr_to_globals).ip_a_d;
  while !cur.is_null() {
    if remote_ip & (*cur).mask == (*cur).ip {
      if (*cur).allow_deny == 'A' as i32 {
        return;
      }
      send_headers_and_exit(HTTP_FORBIDDEN as libc::c_int);
    }
    cur = (*cur).next
  }
  if (*ptr_to_globals).flg_deny_all != 0 {
    /* depends on whether we saw "D:*" */
    send_headers_and_exit(HTTP_FORBIDDEN as libc::c_int);
  };
}
/*
 * Config file entries are of the form "/<path>:<user>:<passwd>".
 * If config file has no prefix match for path, access is allowed.
 *
 * path                 The file path
 * user_and_passwd      "user:passwd" to validate
 *
 * Returns 1 if user_and_passwd is OK.
 */
unsafe extern "C" fn check_user_passwd(
  mut path: *const libc::c_char,
  mut user_and_passwd: *mut libc::c_char,
) -> libc::c_int {
  let mut encrypted: *mut libc::c_char = 0 as *mut libc::c_char; /* for */
  let mut current_block: u64;
  let mut cur: *mut Htaccess = 0 as *mut Htaccess;
  let mut prev: *const libc::c_char = 0 as *const libc::c_char;
  cur = (*ptr_to_globals).g_auth;
  while !cur.is_null() {
    let mut dir_prefix: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: size_t = 0;
    let mut r: libc::c_int = 0;
    dir_prefix = (*cur).before_colon.as_mut_ptr();
    /* WHY? */
    /* If already saw a match, don't accept other different matches */
    if !(!prev.is_null() && strcmp(prev, dir_prefix) != 0i32) {
      /* If it's not a prefix match, continue searching */
      len = strlen(dir_prefix);
      if !(len != 1i32 as libc::c_ulong
        && (strncmp(dir_prefix, path, len) != 0i32
          || *path.offset(len as isize) as libc::c_int != '/' as i32
            && *path.offset(len as isize) as libc::c_int != '\u{0}' as i32))
      {
        /* Path match found */
        prev = dir_prefix;
        let mut colon_after_user: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut passwd: *const libc::c_char = 0 as *const libc::c_char;
        let mut sp_buf: [libc::c_char; 256] = [0; 256];
        colon_after_user = strchr(user_and_passwd, ':' as i32);
        if colon_after_user.is_null() {
          current_block = 1910899846637949173;
        } else if *(*cur).after_colon.offset(0) as libc::c_int != '*' as i32
          && strncmp(
            (*cur).after_colon,
            user_and_passwd,
            (colon_after_user.wrapping_offset_from(user_and_passwd) as libc::c_long + 1)
              as libc::c_ulong,
          ) != 0i32
        {
          current_block = 16559507199688588974;
        } else {
          /* compare "user:" */
          /* this cfg entry is '*' or matches username from peer */
          passwd = strchr((*cur).after_colon, ':' as i32);
          if passwd.is_null() {
            current_block = 1910899846637949173;
          } else {
            passwd = passwd.offset(1);
            if *passwd.offset(0) as libc::c_int == '*' as i32 {
              /* Using _r function to avoid pulling in static buffers */
              let mut spw: spwd = spwd {
                sp_namp: 0 as *mut libc::c_char,
                sp_pwdp: 0 as *mut libc::c_char,
                sp_lstchg: 0,
                sp_min: 0,
                sp_max: 0,
                sp_warn: 0,
                sp_inact: 0,
                sp_expire: 0,
                sp_flag: 0,
              };
              let mut pw: *mut passwd = 0 as *mut passwd;
              *colon_after_user = '\u{0}' as i32 as libc::c_char;
              pw = bb_internal_getpwnam(user_and_passwd);
              *colon_after_user = ':' as i32 as libc::c_char;
              if pw.is_null() || (*pw).pw_passwd.is_null() {
                current_block = 16559507199688588974;
              } else {
                passwd = (*pw).pw_passwd;
                if (*passwd.offset(0) as libc::c_int == 'x' as i32
                  || *passwd.offset(0) as libc::c_int == '*' as i32)
                  && *passwd.offset(1) == 0
                {
                  /* getspnam_r may return 0 yet set result to NULL.
                   * At least glibc 2.4 does this. Be extra paranoid here. */
                  let mut result: *mut spwd = 0 as *mut spwd;
                  r = bb_internal_getspnam_r(
                    (*pw).pw_name,
                    &mut spw,
                    sp_buf.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                    &mut result,
                  );
                  if r == 0i32 && !result.is_null() {
                    passwd = (*result).sp_pwdp
                  }
                }
                current_block = 15736828544826234929;
              }
            /* ENABLE_PAM */
            } else if *passwd.offset(0) as libc::c_int == '$' as i32
              && (*passwd.offset(1) as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int
                <= 9i32
            {
              encrypted = 0 as *mut libc::c_char;
              current_block = 15736828544826234929;
            } else {
              /* Else: passwd is from httpd.conf, it is either plaintext or encrypted */
              /* local passwd is from httpd.conf and it's plaintext */
              r = strcmp(colon_after_user.offset(1), passwd);
              current_block = 14545652377200942025;
            }
            match current_block {
              16559507199688588974 => {}
              14545652377200942025 => {}
              _ =>
              /* In this case, passwd is ALWAYS encrypted:
               * it came from /etc/passwd or /etc/shadow!
               */
              /* encrypt pwd from peer and check match with local one */
              {
                encrypted = pw_encrypt(colon_after_user.offset(1), passwd, 0i32);
                r = strcmp(encrypted, passwd);
                free(encrypted as *mut libc::c_void);
                current_block = 14545652377200942025;
              }
            }
          }
        }
        match current_block {
          16559507199688588974 => {}
          _ => {
            match current_block {
              1910899846637949173 => {
                /* Comparing plaintext "user:pass" in one go */
                r = strcmp((*cur).after_colon, user_and_passwd)
              }
              _ => {}
            }
            if r == 0i32 {
              (*ptr_to_globals).remoteuser = xstrndup(
                user_and_passwd,
                strchrnul(user_and_passwd, ':' as i32).wrapping_offset_from(user_and_passwd)
                  as libc::c_long as libc::c_int,
              );
              return 1i32;
              /* Ok */
            }
          }
        }
      }
    }
    cur = (*cur).next
  }
  /* 0(bad) if prev is set: matches were found but passwd was wrong */
  return (prev == 0 as *mut libc::c_void as *const libc::c_char) as libc::c_int;
}
/* FEATURE_HTTPD_BASIC_AUTH */
unsafe extern "C" fn find_proxy_entry(mut url: *const libc::c_char) -> *mut Htaccess_Proxy {
  let mut p: *mut Htaccess_Proxy = 0 as *mut Htaccess_Proxy;
  p = (*ptr_to_globals).proxy;
  while !p.is_null() {
    if !is_prefixed_with(url, (*p).url_from).is_null() {
      return p;
    }
    p = (*p).next
  }
  return 0 as *mut Htaccess_Proxy;
}
/*
 * Handle timeouts
 */
unsafe extern "C" fn send_REQUEST_TIMEOUT_and_exit(mut _sig: libc::c_int) -> ! {
  send_headers_and_exit(HTTP_REQUEST_TIMEOUT as libc::c_int);
}
/*
 * Handle an incoming http request and exit.
 */
unsafe extern "C" fn handle_incoming_and_exit(mut fromAddr: *const len_and_sockaddr) -> ! {
  let mut current_block: u64;
  static mut request_GET: [libc::c_char; 4] = [71, 69, 84, 0];
  let mut sb: stat = std::mem::zeroed();
  let mut urlcopy: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut urlp: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut tptr: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut remote_ip: libc::c_uint = 0;
  let mut total_headers_len: libc::c_uint = 0;
  static mut request_HEAD: [libc::c_char; 5] = [72, 69, 65, 68, 0];
  let mut prequest: *const libc::c_char = 0 as *const libc::c_char;
  let mut length: libc::c_ulong = 0i32 as libc::c_ulong;
  let mut cgi_type: CGI_type = CGI_NONE;
  let mut authorized: smallint = -1i32 as smallint;
  let mut HTTP_slash: *mut libc::c_char = 0 as *mut libc::c_char;
  /* Allocation of iobuf is postponed until now
   * (IOW, server process doesn't need to waste 8k) */
  (*ptr_to_globals).iobuf = xmalloc(8192i32 as size_t) as *mut libc::c_char;
  remote_ip = 0i32 as libc::c_uint;
  if (*fromAddr).u.sa.sa_family as libc::c_int == 2i32 {
    remote_ip = {
      let mut __v: libc::c_uint = 0;
      let mut __x: libc::c_uint = (*fromAddr).u.sin.sin_addr.s_addr;
      if 0 != 0 {
        __v = (__x & 0xff000000u32) >> 24i32
          | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
          | (__x & 0xff00i32 as libc::c_uint) << 8i32
          | (__x & 0xffi32 as libc::c_uint) << 24i32
      } else {
        let fresh14 = &mut __v;
        let fresh15;
        let fresh16 = __x;
        asm!("bswap $0" : "=r" (fresh15) : "0"
                          (c2rust_asm_casts::AsmCast::cast_in(fresh14, fresh16))
                          :);
        c2rust_asm_casts::AsmCast::cast_out(fresh14, fresh16, fresh15);
      }
      __v
    }
  }
  if (*fromAddr).u.sa.sa_family as libc::c_int == 10i32
    && (*fromAddr).u.sin6.sin6_addr.__in6_u.__u6_addr32[0] == 0i32 as libc::c_uint
    && (*fromAddr).u.sin6.sin6_addr.__in6_u.__u6_addr32[1] == 0i32 as libc::c_uint
    && ({
      let mut __v: libc::c_uint = 0;
      let mut __x: libc::c_uint = (*fromAddr).u.sin6.sin6_addr.__in6_u.__u6_addr32[2];
      if 0 != 0 {
        __v = (__x & 0xff000000u32) >> 24i32
          | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
          | (__x & 0xff00i32 as libc::c_uint) << 8i32
          | (__x & 0xffi32 as libc::c_uint) << 24i32
      } else {
        let fresh17 = &mut __v;
        let fresh18;
        let fresh19 = __x;
        asm!("bswap $0" : "=r" (fresh18) : "0"
                         (c2rust_asm_casts::AsmCast::cast_in(fresh17, fresh19))
                         :);
        c2rust_asm_casts::AsmCast::cast_out(fresh17, fresh19, fresh18);
      }
      __v
    }) == 0xffffi32 as libc::c_uint
  {
    remote_ip = {
      let mut __v: libc::c_uint = 0;
      let mut __x: libc::c_uint = (*fromAddr).u.sin6.sin6_addr.__in6_u.__u6_addr32[3];
      if 0 != 0 {
        __v = (__x & 0xff000000u32) >> 24i32
          | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
          | (__x & 0xff00i32 as libc::c_uint) << 8i32
          | (__x & 0xffi32 as libc::c_uint) << 24i32
      } else {
        let fresh20 = &mut __v;
        let fresh21;
        let fresh22 = __x;
        asm!("bswap $0" : "=r" (fresh21) : "0"
                          (c2rust_asm_casts::AsmCast::cast_in(fresh20, fresh22))
                          :);
        c2rust_asm_casts::AsmCast::cast_out(fresh20, fresh22, fresh21);
      }
      __v
    }
  }
  if 1i32 != 0 || 0i32 != 0 || (*ptr_to_globals).verbose != 0 {
    /* NB: can be NULL (user runs httpd -i by hand?) */
    (*ptr_to_globals).rmt_ip_str = xmalloc_sockaddr2dotted(&(*fromAddr).u.sa)
  }
  if (*ptr_to_globals).verbose != 0 {
    /* this trick makes -v logging much simpler */
    if !(*ptr_to_globals).rmt_ip_str.is_null() {
      applet_name = (*ptr_to_globals).rmt_ip_str
    }
    if (*ptr_to_globals).verbose > 2i32 {
      bb_simple_error_msg(b"connected\x00" as *const u8 as *const libc::c_char);
    }
  }
  if_ip_denied_send_HTTP_FORBIDDEN_and_exit(remote_ip);
  /* Install timeout handler. get_line() needs it. */
  signal(
    14i32,
    ::std::mem::transmute::<Option<unsafe extern "C" fn(_: libc::c_int) -> !>, __sighandler_t>(
      Some(send_REQUEST_TIMEOUT_and_exit as unsafe extern "C" fn(_: libc::c_int) -> !),
    ),
  );
  if get_line() == 0 {
    /* EOF or error or empty line */
    send_headers_and_exit(HTTP_BAD_REQUEST as libc::c_int);
  }
  /* Determine type of request (GET/POST) */
  // rfc2616: method and URI is separated by exactly one space
  //urlp = strpbrk(iobuf, " \t"); - no, tab isn't allowed
  urlp = strchr((*ptr_to_globals).iobuf, ' ' as i32);
  if urlp.is_null() {
    send_headers_and_exit(HTTP_BAD_REQUEST as libc::c_int);
  }
  let fresh23 = urlp;
  urlp = urlp.offset(1);
  *fresh23 = '\u{0}' as i32 as libc::c_char;
  prequest = request_GET.as_ptr();
  if strcasecmp((*ptr_to_globals).iobuf, prequest) != 0i32 {
    prequest = request_HEAD.as_ptr();
    if strcasecmp((*ptr_to_globals).iobuf, prequest) != 0i32 {
      prequest = b"POST\x00" as *const u8 as *const libc::c_char;
      if strcasecmp((*ptr_to_globals).iobuf, prequest) != 0i32 {
        send_headers_and_exit(HTTP_NOT_IMPLEMENTED as libc::c_int);
      }
    }
  }
  // rfc2616: method and URI is separated by exactly one space
  //urlp = skip_whitespace(urlp); - should not be necessary
  if *urlp.offset(0) as libc::c_int != '/' as i32 {
    send_headers_and_exit(HTTP_BAD_REQUEST as libc::c_int);
  }
  /* Find end of URL */
  HTTP_slash = strchr(urlp, ' ' as i32);
  /* Is it " HTTP/"? */
  if HTTP_slash.is_null()
    || strncmp(
      HTTP_slash.offset(1),
      HTTP_200.as_ptr(),
      5i32 as libc::c_ulong,
    ) != 0i32
  {
    send_headers_and_exit(HTTP_BAD_REQUEST as libc::c_int);
  }
  let fresh24 = HTTP_slash;
  HTTP_slash = HTTP_slash.offset(1);
  *fresh24 = '\u{0}' as i32 as libc::c_char;
  /* Copy URL from after "GET "/"POST " to stack-allocated char[] */
  let mut fresh25 = ::std::vec::from_elem(
    0,
    ((HTTP_slash.wrapping_offset_from(urlp) as libc::c_long + 2i32 as libc::c_long)
      as libc::c_ulong)
      .wrapping_add(strlen((*ptr_to_globals).index_page)) as usize,
  );
  urlcopy = fresh25.as_mut_ptr() as *mut libc::c_char;
  /*if (urlcopy == NULL)
   *	send_headers_and_exit(HTTP_INTERNAL_SERVER_ERROR);*/
  strcpy(urlcopy, urlp);
  /* NB: urlcopy ptr is never changed after this */
  let mut proxy_fd: libc::c_int = 0;
  let mut lsa: *mut len_and_sockaddr = 0 as *mut len_and_sockaddr;
  let mut proxy_entry: *mut Htaccess_Proxy = find_proxy_entry(urlcopy);
  if !proxy_entry.is_null() {
    if (*ptr_to_globals).verbose > 1i32 {
      bb_error_msg(b"proxy:%s\x00" as *const u8 as *const libc::c_char, urlcopy);
    }
    lsa = host2sockaddr((*proxy_entry).host_port, 80i32);
    if lsa.is_null() {
      send_headers_and_exit(HTTP_INTERNAL_SERVER_ERROR as libc::c_int);
    }
    proxy_fd = socket(
      (*lsa).u.sa.sa_family as libc::c_int,
      SOCK_STREAM as libc::c_int,
      0i32,
    );
    if proxy_fd < 0i32 {
      send_headers_and_exit(HTTP_INTERNAL_SERVER_ERROR as libc::c_int);
    }
    if connect(
      proxy_fd,
      __CONST_SOCKADDR_ARG {
        __sockaddr__: &mut (*lsa).u.sa,
      },
      (*lsa).len,
    ) < 0i32
    {
      send_headers_and_exit(HTTP_INTERNAL_SERVER_ERROR as libc::c_int);
    }
    /* Disable peer header reading timeout */
    alarm(0i32 as libc::c_uint);
    /* Config directive was of the form:
     *   P:/url:[http://]hostname[:port]/new/path
     * When /urlSFX is requested, reverse proxy it
     * to http://hostname[:port]/new/pathSFX
     */
    dprintf(
      proxy_fd,
      b"%s %s%s %s\r\n\x00" as *const u8 as *const libc::c_char,
      prequest,
      (*proxy_entry).url_to,
      urlcopy.offset(strlen((*proxy_entry).url_from) as isize),
      HTTP_slash,
    );
    cgi_io_loop_and_exit(proxy_fd, proxy_fd, 2147483647i32);
  }
  /* Extract url args if present */
  (*ptr_to_globals).g_query = strchr(urlcopy, '?' as i32);
  if !(*ptr_to_globals).g_query.is_null() {
    let fresh26 = (*ptr_to_globals).g_query;
    (*ptr_to_globals).g_query = (*ptr_to_globals).g_query.offset(1);
    *fresh26 = '\u{0}' as i32 as libc::c_char
  }
  /* Decode URL escape sequences */
  tptr = percent_decode_in_place(urlcopy, 1i32);
  if tptr.is_null() {
    send_headers_and_exit(HTTP_BAD_REQUEST as libc::c_int);
  }
  if tptr == urlcopy.offset(1) {
    /* '/' or NUL is encoded */
    send_headers_and_exit(HTTP_NOT_FOUND as libc::c_int);
  }
  /* Canonicalize path */
  /* Algorithm stolen from libbb bb_simplify_path(),
   * but don't strdup, retain trailing slash, protect root */
  tptr = urlcopy;
  urlp = tptr;
  loop {
    if *urlp as libc::c_int == '/' as i32 {
      /* skip duplicate (or initial) slash */
      if *tptr as libc::c_int == '/' as i32 {
        current_block = 9975938041278959690;
      } else if *tptr as libc::c_int == '.' as i32 {
        if *tptr.offset(1) as libc::c_int == '.' as i32
          && (*tptr.offset(2) as libc::c_int == '/' as i32
            || *tptr.offset(2) as libc::c_int == '\u{0}' as i32)
        {
          /* "..": be careful */
          /* protect root */
          if urlp == urlcopy {
            send_headers_and_exit(HTTP_BAD_REQUEST as libc::c_int);
          }
          loop
          /* omit previous dir */
          {
            urlp = urlp.offset(-1);
            if !(*urlp as libc::c_int != '/' as i32) {
              break;
            }
          }
          /* skip to "./" or ".<NUL>" */
          tptr = tptr.offset(1)
        }
        if *tptr.offset(1) as libc::c_int == '/' as i32
          || *tptr.offset(1) as libc::c_int == '\u{0}' as i32
        {
          current_block = 9975938041278959690;
        } else {
          current_block = 17787701279558130514;
        }
      } else {
        current_block = 17787701279558130514;
      }
    } else {
      current_block = 17787701279558130514;
    }
    match current_block {
      17787701279558130514 => {
        urlp = urlp.offset(1);
        *urlp = *tptr;
        if *tptr as libc::c_int == '\u{0}' as i32 {
          break;
        }
      }
      _ => {}
    }
    /* skip extra "/./" */
    tptr = tptr.offset(1)
  }
  /* If URL is a directory, add '/' */
  if *urlp.offset(-1i32 as isize) as libc::c_int != '/' as i32 {
    if is_directory(urlcopy.offset(1), 1i32) != 0 {
      (*ptr_to_globals).found_moved_temporarily = urlcopy
    }
  }
  /* Log it */
  if (*ptr_to_globals).verbose > 1i32 {
    bb_error_msg(b"url:%s\x00" as *const u8 as *const libc::c_char, urlcopy);
  }
  tptr = urlcopy;
  loop {
    tptr = strchr(tptr.offset(1), '/' as i32);
    if tptr.is_null() {
      break;
    }
    /* have path1/path2 */
    *tptr = '\u{0}' as i32 as libc::c_char;
    if is_directory(urlcopy.offset(1), 1i32) != 0 {
      /* may have subdir config */
      parse_conf(urlcopy.offset(1), SUBDIR_PARSE as libc::c_int); /* skip first '/' */
      if_ip_denied_send_HTTP_FORBIDDEN_and_exit(remote_ip);
    }
    *tptr = '/' as i32 as libc::c_char
  }
  tptr = urlcopy.offset(1);
  if !is_prefixed_with(tptr, b"cgi-bin/\x00" as *const u8 as *const libc::c_char).is_null() {
    if *tptr.offset(8) as libc::c_int == '\u{0}' as i32 {
      /* protect listing "cgi-bin/" */
      send_headers_and_exit(HTTP_FORBIDDEN as libc::c_int);
    }
    cgi_type = CGI_NORMAL
  }
  if *urlp.offset(-1i32 as isize) as libc::c_int == '/' as i32 {
    /* When index_page string is appended to <dir>/ URL, it overwrites
     * the query string. If we fall back to call /cgi-bin/index.cgi,
     * query string would be lost and not available to the CGI.
     * Work around it by making a deep copy.
     */
    (*ptr_to_globals).g_query = xstrdup((*ptr_to_globals).g_query); /* ok for NULL too */
    strcpy(urlp, (*ptr_to_globals).index_page);
  }
  if stat(tptr, &mut sb) == 0i32 {
    let mut suffix: *mut libc::c_char = strrchr(tptr, '.' as i32);
    if !suffix.is_null() {
      let mut cur: *mut Htaccess = 0 as *mut Htaccess;
      cur = (*ptr_to_globals).script_i;
      while !cur.is_null() {
        if strcmp((*cur).before_colon.as_mut_ptr().offset(1), suffix) == 0i32 {
          cgi_type = CGI_INTERPRETER;
          break;
        } else {
          cur = (*cur).next
        }
      }
    }
    if (*ptr_to_globals).found_moved_temporarily.is_null() {
      (*ptr_to_globals).file_size = sb.st_size;
      (*ptr_to_globals).last_mod = sb.st_mtime
    }
  } else if *urlp.offset(-1i32 as isize) as libc::c_int == '/' as i32 {
    /* It's a dir URL and there is no index.html
     * Try cgi-bin/index.cgi */
    if access(
      (b"/cgi-bin/index.cgi\x00" as *const u8 as *const libc::c_char).offset(1),
      1i32,
    ) == 0i32
    {
      cgi_type = CGI_INDEX
    }
  }
  *urlp.offset(0) = '\u{0}' as i32 as libc::c_char;
  total_headers_len = 0i32 as libc::c_uint;
  loop
  /* Read until blank line */
  {
    let mut iobuf_len: libc::c_uint = get_line(); /* while extra header reading */
    if iobuf_len == 0 {
      break; /* EOF or error or empty line */
    }
    /* Prevent unlimited growth of HTTP_xyz envvars */
    total_headers_len = total_headers_len.wrapping_add(iobuf_len);
    if total_headers_len >= (32i32 * 1024i32) as libc::c_uint {
      send_headers_and_exit(HTTP_ENTITY_TOO_LARGE as libc::c_int);
    }
    /* Try and do our best to parse more lines */
    if strncasecmp(
      (*ptr_to_globals).iobuf,
      b"Content-Length:\x00" as *const u8 as *const libc::c_char,
      (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong),
    ) == 0i32
    {
      /* extra read only for POST */
      if prequest != request_GET.as_ptr() && prequest != request_HEAD.as_ptr() {
        tptr = skip_whitespace(
          (*ptr_to_globals)
            .iobuf
            .offset(::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as isize)
            .offset(-1),
        );
        if *tptr.offset(0) == 0 {
          send_headers_and_exit(HTTP_BAD_REQUEST as libc::c_int);
        }
        /* not using strtoul: it ignores leading minus! */
        length = bb_strtou(tptr, 0 as *mut *mut libc::c_char, 10i32) as libc::c_ulong;
        /* length is "ulong", but we need to pass it to int later */
        if *bb_errno != 0 || length > 2147483647i32 as libc::c_ulong {
          send_headers_and_exit(HTTP_BAD_REQUEST as libc::c_int);
        }
      }
    } else {
      if strncasecmp(
        (*ptr_to_globals).iobuf,
        b"Authorization:\x00" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
          .wrapping_sub(1i32 as libc::c_ulong),
      ) == 0i32
      {
        /* We only allow Basic credentials.
         * It shows up as "Authorization: Basic <user>:<passwd>" where
         * "<user>:<passwd>" is base64 encoded.
         */
        tptr = skip_whitespace(
          (*ptr_to_globals)
            .iobuf
            .offset(::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong as isize)
            .offset(-1),
        );
        if strncasecmp(
          tptr,
          b"Basic\x00" as *const u8 as *const libc::c_char,
          (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong),
        ) == 0i32
        {
          tptr = tptr.offset(
            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
              .wrapping_sub(1i32 as libc::c_ulong) as isize,
          );
          /* decodeBase64() skips whitespace itself */
          decodeBase64(tptr);
          authorized = check_user_passwd(urlcopy, tptr) as smallint;
          continue;
        }
      }
      if strncasecmp(
        (*ptr_to_globals).iobuf,
        b"Range:\x00" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
          .wrapping_sub(1i32 as libc::c_ulong),
      ) == 0i32
      {
        /* We know only bytes=NNN-[MMM] */
        let mut s: *mut libc::c_char = skip_whitespace(
          (*ptr_to_globals)
            .iobuf
            .offset(::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as isize)
            .offset(-1),
        );
        if !is_prefixed_with(s, b"bytes=\x00" as *const u8 as *const libc::c_char).is_null() {
          s = s.offset(
            (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
              .wrapping_sub(1i32 as libc::c_ulong) as isize,
          );
          (*ptr_to_globals).range_start = bb_strtoul(s, &mut s, 10i32) as off_t;
          if *s.offset(0) as libc::c_int != '-' as i32 || (*ptr_to_globals).range_start < 0 {
            (*ptr_to_globals).range_start = -1i32 as off_t
          } else if *s.offset(1) != 0 {
            (*ptr_to_globals).range_end =
              bb_strtoul(s.offset(1), 0 as *mut *mut libc::c_char, 10i32) as off_t;
            if *bb_errno != 0 || (*ptr_to_globals).range_end < (*ptr_to_globals).range_start {
              (*ptr_to_globals).range_start = -1i32 as off_t
            }
          }
        }
      } else if strncasecmp(
        (*ptr_to_globals).iobuf,
        b"Accept-Encoding:\x00" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
          .wrapping_sub(1i32 as libc::c_ulong),
      ) == 0i32
      {
        /* Note: we do not support "gzip;q=0"
         * method of _disabling_ gzip
         * delivery. No one uses that, though */
        let mut s_0: *const libc::c_char = strstr(
          (*ptr_to_globals).iobuf,
          b"gzip\x00" as *const u8 as *const libc::c_char,
        );
        if !s_0.is_null() {
          // want more thorough checks?
          //if (s[-1] == ' '
          // || s[-1] == ','
          // || s[-1] == ':'
          //) {
          (*ptr_to_globals).content_gzip = 1i32 as smallint
          //}
        }
      } else {
        if !(cgi_type as libc::c_uint != CGI_NONE as libc::c_int as libc::c_uint) {
          continue;
        }
        let mut ct: bool = strncasecmp(
          (*ptr_to_globals).iobuf,
          b"Content-Type:\x00" as *const u8 as *const libc::c_char,
          (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong),
        ) == 0i32;
        let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut colon: *mut libc::c_char = strchr((*ptr_to_globals).iobuf, ':' as i32);
        if colon.is_null() {
          continue;
        }
        cp = (*ptr_to_globals).iobuf;
        while cp < colon {
          /* a-z => A-Z, not-alnum => _ */
          let mut c: libc::c_char = (*cp as libc::c_int & !0x20i32) as libc::c_char; /* toupper for A-Za-z, undef for others */
          if (c as libc::c_int - 'A' as i32) as libc::c_uint
            <= ('Z' as i32 - 'A' as i32) as libc::c_uint
          {
            let fresh27 = cp;
            cp = cp.offset(1);
            *fresh27 = c
          } else {
            if !((*cp as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32) {
              *cp = '_' as i32 as libc::c_char
            }
            cp = cp.offset(1)
          }
        }
        /* "Content-Type:" gets no HTTP_ prefix, all others do */
        cp = xasprintf(
          if ct as libc::c_int != 0 {
            (b"HTTP_%.*s=%s\x00" as *const u8 as *const libc::c_char).offset(5)
          } else {
            b"HTTP_%.*s=%s\x00" as *const u8 as *const libc::c_char
          },
          colon.wrapping_offset_from((*ptr_to_globals).iobuf) as libc::c_long as libc::c_int,
          (*ptr_to_globals).iobuf,
          skip_whitespace(colon.offset(1)),
        );
        putenv(cp);
      }
    }
  }
  /* We are done reading headers, disable peer timeout */
  alarm(0i32 as libc::c_uint);
  if strcmp(bb_basename(urlcopy), HTTPD_CONF.as_ptr()) == 0i32 {
    /* protect listing [/path]/httpd.conf or IP deny */
    send_headers_and_exit(HTTP_FORBIDDEN as libc::c_int);
  }
  /* Case: no "Authorization:" was seen, but page might require passwd.
   * Check that with dummy user:pass */
  if (authorized as libc::c_int) < 0i32 {
    authorized = check_user_passwd(
      urlcopy,
      b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as smallint
  } /* skip first '/' */
  if authorized == 0 {
    send_headers_and_exit(HTTP_UNAUTHORIZED as libc::c_int);
  }
  if !(*ptr_to_globals).found_moved_temporarily.is_null() {
    send_headers_and_exit(HTTP_MOVED_TEMPORARILY as libc::c_int);
  }
  tptr = urlcopy.offset(1);
  if cgi_type as libc::c_uint != CGI_NONE as libc::c_int as libc::c_uint {
    send_cgi_and_exit(
      if cgi_type as libc::c_uint == CGI_INDEX as libc::c_int as libc::c_uint {
        b"/cgi-bin/index.cgi\x00" as *const u8 as *const libc::c_char
      } else {
        urlcopy
      },
      urlcopy,
      prequest,
      length as libc::c_int,
    );
  }
  if *urlp.offset(-1i32 as isize) as libc::c_int == '/' as i32 {
    strcpy(urlp, (*ptr_to_globals).index_page);
  }
  if prequest != request_GET.as_ptr() && prequest != request_HEAD.as_ptr() {
    /* POST for files does not make sense */
    send_headers_and_exit(HTTP_NOT_IMPLEMENTED as libc::c_int);
  }
  send_file_and_exit(
    tptr,
    if prequest != request_HEAD.as_ptr() {
      SEND_HEADERS_AND_BODY as libc::c_int
    } else {
      SEND_HEADERS as libc::c_int
    },
  );
}
/*
 * The main http server function.
 * Given a socket, listen for new connections and farm out
 * the processing as a [v]forked process.
 * Never returns.
 */
unsafe extern "C" fn mini_httpd(mut server_socket: libc::c_int) -> ! {
  loop
  /* NB: it's best to not use xfuncs in this loop before fork().
   * Otherwise server may die on transient errors (temporary
   * out-of-memory condition, etc), which is Bad(tm).
   * Try to do any dangerous calls after fork.
   */
  {
    let mut n: libc::c_int = 0;
    let mut fromAddr: len_and_sockaddr = len_and_sockaddr {
      len: 0,
      u: C2RustUnnamed_1 {
        sa: sockaddr {
          sa_family: 0,
          sa_data: [0; 14],
        },
      },
    };
    /* Wait for connections... */
    fromAddr.len = LSA_SIZEOF_SA as libc::c_int as socklen_t;
    n = accept(
      server_socket,
      __SOCKADDR_ARG {
        __sockaddr__: &mut fromAddr.u.sa as *mut sockaddr,
      },
      &mut fromAddr.len,
    );
    if n < 0i32 {
      continue;
    }
    /* set the KEEPALIVE option to cull dead connections */
    setsockopt_keepalive(n);
    if fork() == 0i32 {
      /* child */
      /* Do not reload config on HUP */
      signal(
        1i32,
        ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(1i32 as libc::intptr_t),
      );
      close(server_socket);
      xmove_fd(n, 0i32);
      xdup2(0i32, 1i32);
      handle_incoming_and_exit(&mut fromAddr);
    }
    /* parent, or fork failed */
    close(n);
  }
  /* while (1) */
  /* never reached */
}
/*
 * Process a HTTP connection on stdin/out.
 * Never returns.
 */
unsafe extern "C" fn mini_httpd_inetd() -> ! {
  let mut fromAddr: len_and_sockaddr = len_and_sockaddr {
    len: 0,
    u: C2RustUnnamed_1 {
      sa: sockaddr {
        sa_family: 0,
        sa_data: [0; 14],
      },
    },
  };
  memset(
    &mut fromAddr as *mut len_and_sockaddr as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<len_and_sockaddr>() as libc::c_ulong,
  );
  fromAddr.len = LSA_SIZEOF_SA as libc::c_int as socklen_t;
  /* NB: can fail if user runs it by hand and types in http cmds */
  getpeername(
    0i32,
    __SOCKADDR_ARG {
      __sockaddr__: &mut fromAddr.u.sa as *mut sockaddr,
    },
    &mut fromAddr.len,
  ); /* for gcc */
  handle_incoming_and_exit(&mut fromAddr);
}
unsafe extern "C" fn sighup_handler(mut _sig: libc::c_int) {
  let mut sv: libc::c_int = *bb_errno;
  parse_conf(
    DEFAULT_PATH_HTTPD_CONF.as_ptr(),
    SIGNALED_PARSE as libc::c_int,
  );
  *bb_errno = sv;
}
#[no_mangle]
pub unsafe extern "C" fn httpd_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut server_socket: libc::c_int = 0;
  server_socket = server_socket;
  let mut opt: libc::c_uint = 0;
  let mut url_for_decode: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut url_for_encode: *const libc::c_char = 0 as *const libc::c_char;
  let mut s_ugid: *const libc::c_char = 0 as *const libc::c_char;
  let mut ugid: bb_uidgid_t = bb_uidgid_t { uid: 0, gid: 0 };
  let mut pass: *const libc::c_char = 0 as *const libc::c_char;
  let ref mut fresh28 =
    *(not_const_pp(&ptr_to_globals as *const *mut globals as *const libc::c_void)
      as *mut *mut globals);
  *fresh28 = xzalloc(::std::mem::size_of::<globals>() as libc::c_ulong) as *mut globals;
  asm!("" : : : "memory" : "volatile");
  (*ptr_to_globals).g_realm = b"Web Server Authentication\x00" as *const u8 as *const libc::c_char;
  (*ptr_to_globals).range_start = -1i32 as off_t;
  (*ptr_to_globals).bind_addr_or_port = b"80\x00" as *const u8 as *const libc::c_char;
  (*ptr_to_globals).index_page = index_html.as_ptr();
  (*ptr_to_globals).file_size = -1i32 as off_t;
  (*ptr_to_globals).home_httpd = xrealloc_getcwd_or_warn(0 as *mut libc::c_char);
  /* We do not "absolutize" path given by -h (home) opt.
   * If user gives relative path in -h,
   * $SCRIPT_FILENAME will not be set. */
  opt = getopt32(
    argv,
    b"^c:d:h:e:r:m:u:p:ifv\x00vv:if\x00" as *const u8 as *const libc::c_char,
    &mut (*ptr_to_globals).opt_c_configFile as *mut *const libc::c_char,
    &mut url_for_decode as *mut *mut libc::c_char,
    &mut (*ptr_to_globals).home_httpd as *mut *const libc::c_char,
    &mut url_for_encode as *mut *const libc::c_char,
    &mut (*ptr_to_globals).g_realm as *mut *const libc::c_char,
    &mut pass as *mut *const libc::c_char,
    &mut s_ugid as *mut *const libc::c_char,
    &mut (*ptr_to_globals).bind_addr_or_port as *mut *const libc::c_char,
    &mut (*ptr_to_globals).verbose as *mut libc::c_int,
  );
  if opt & OPT_DECODE_URL as libc::c_int as libc::c_uint != 0 {
    fputs_unlocked(percent_decode_in_place(url_for_decode, 0i32), stdout);
    return 0i32;
  }
  if opt & OPT_ENCODE_URL as libc::c_int as libc::c_uint != 0 {
    fputs_unlocked(encodeString(url_for_encode), stdout);
    return 0i32;
  }
  if opt & OPT_MD5 as libc::c_int as libc::c_uint != 0 {
    let mut salt: [libc::c_char; 12] = [0; 12];
    salt[0] = '$' as i32 as libc::c_char;
    salt[1] = '1' as i32 as libc::c_char;
    salt[2] = '$' as i32 as libc::c_char;
    crypt_make_salt(salt.as_mut_ptr().offset(3), 4i32);
    puts(pw_encrypt(pass, salt.as_mut_ptr(), 0i32));
    return 0i32;
  }
  if opt & OPT_SETUID as libc::c_int as libc::c_uint != 0 {
    xget_uidgid(&mut ugid, s_ugid);
  }
  xchdir((*ptr_to_globals).home_httpd);
  if opt & OPT_INETD as libc::c_int as libc::c_uint == 0 {
    signal(
      17i32,
      ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(1i32 as libc::intptr_t),
    );
    server_socket = openServer();
    /* drop privileges */
    if opt & OPT_SETUID as libc::c_int as libc::c_uint != 0 {
      if ugid.gid != -1i32 as gid_t {
        if setgroups(1i32 as size_t, &mut ugid.gid) == -1i32 {
          bb_simple_perror_msg_and_die(b"setgroups\x00" as *const u8 as *const libc::c_char);
          /* never returns */
        } /* don't change current directory */
        xsetgid(ugid.gid);
      }
      xsetuid(ugid.uid);
    }
  }
  parse_conf(DEFAULT_PATH_HTTPD_CONF.as_ptr(), FIRST_PARSE as libc::c_int);
  if opt & OPT_INETD as libc::c_int as libc::c_uint == 0 {
    signal(
      1i32,
      Some(sighup_handler as unsafe extern "C" fn(_: libc::c_int) -> ()),
    );
  }
  xfunc_error_retval = 0i32 as u8;
  if opt & OPT_INETD as libc::c_int as libc::c_uint != 0 {
    mini_httpd_inetd();
  }
  if opt & OPT_FOREGROUND as libc::c_int as libc::c_uint == 0 {
    bb_daemonize_or_rexec(0i32);
  }
  mini_httpd(server_socket);
  /* never returns */
  /* return 0; */
}
