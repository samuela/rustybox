use crate::libbb::appletlib::applet_name;
use crate::libbb::parse_config::parser_t;
use crate::libbb::ptr_to_globals::bb_errno;
use crate::libbb::xfuncs_printf::xmalloc;
use crate::libpwdgrp::pwd_grp::bb_internal_getpwnam;
use crate::librb::len_and_sockaddr;
use crate::librb::signal::__sighandler_t;
use crate::librb::signal::sigaction;
use crate::librb::size_t;
use crate::librb::smallint;
use crate::librb::socklen_t;
use c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use libc;
use libc::alarm;
use libc::close;
use libc::free;
use libc::getgid;
use libc::getuid;
use libc::gid_t;
use libc::group;
use libc::openlog;
use libc::passwd;
use libc::pid_t;
use libc::sa_family_t;
use libc::setsid;
use libc::sigaddset;
use libc::sigemptyset;
use libc::siginfo_t;
use libc::sigprocmask;
use libc::sigset_t;
use libc::sigval;
use libc::sleep;
use libc::sockaddr;
use libc::sockaddr_in;
use libc::sockaddr_in6;
use libc::sprintf;
use libc::ssize_t;
use libc::strchr;
use libc::strcmp;
use libc::strrchr;
use libc::time;
use libc::time_t;
use libc::timeval;
use libc::uid_t;
use libc::unlink;
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
  fn sigaction(__sig: libc::c_int, __act: *const sigaction, __oact: *mut sigaction) -> libc::c_int;

  #[no_mangle]
  fn getrlimit(__resource: __rlimit_resource_t, __rlimits: *mut rlimit) -> libc::c_int;
  #[no_mangle]
  fn setrlimit(__resource: __rlimit_resource_t, __rlimits: *const rlimit) -> libc::c_int;
  #[no_mangle]
  fn select(
    __nfds: libc::c_int,
    __readfds: *mut fd_set,
    __writefds: *mut fd_set,
    __exceptfds: *mut fd_set,
    __timeout: *mut timeval,
  ) -> libc::c_int;
  #[no_mangle]
  fn socket(__domain: libc::c_int, __type: libc::c_int, __protocol: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn bind(__fd: libc::c_int, __addr: __CONST_SOCKADDR_ARG, __len: socklen_t) -> libc::c_int;
  #[no_mangle]
  fn connect(__fd: libc::c_int, __addr: __CONST_SOCKADDR_ARG, __len: socklen_t) -> libc::c_int;
  #[no_mangle]
  fn recv(
    __fd: libc::c_int,
    __buf: *mut libc::c_void,
    __n: size_t,
    __flags: libc::c_int,
  ) -> ssize_t;
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
  fn recvfrom(
    __fd: libc::c_int,
    __buf: *mut libc::c_void,
    __n: size_t,
    __flags: libc::c_int,
    __addr: __SOCKADDR_ARG,
    __addr_len: *mut socklen_t,
  ) -> ssize_t;
  #[no_mangle]
  fn listen(__fd: libc::c_int, __n: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn accept(__fd: libc::c_int, __addr: __SOCKADDR_ARG, __addr_len: *mut socklen_t) -> libc::c_int;
  #[no_mangle]
  fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;

  #[no_mangle]
  fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;

  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn _exit(_: libc::c_int) -> !;

  #[no_mangle]
  fn fork() -> pid_t;
  #[no_mangle]
  fn vfork() -> libc::c_int;

  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn getservbyname(__name: *const libc::c_char, __proto: *const libc::c_char) -> *mut servent;

  #[no_mangle]
  fn dprintf(__fd: libc::c_int, __fmt: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn execvp(__file: *const libc::c_char, __argv: *const *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn exit(_: libc::c_int) -> !;
  #[no_mangle]
  fn gettimeofday(__tv: *mut timeval, __tz: __timezone_ptr_t) -> libc::c_int;

  #[no_mangle]
  fn ctime(__timer: *const time_t) -> *mut libc::c_char;
  #[no_mangle]
  fn setgroups(__n: size_t, __groups: *const gid_t) -> libc::c_int;
  /* Search for an entry with a matching username.  */

  /* Search for an entry with a matching group name.  */

  /* Some useful definitions */
  /* Macros for min/max.  */
  /* buffer allocation schemes */
  /* glibc uses __errno_location() to get a ptr to errno */
  /* We can just memorize it once - no multithreading in busybox :) */

  /* NB: can violate const-ness (similarly to strchr) */

  /* Will do sigaction(signum, act, NULL): */

  /* Return old set in the same set: */

  /* not FAST_FUNC! */

  /* SO_REUSEADDR allows a server to rebind to an address that is already
   * "in use" by old connections to e.g. previous server instance which is
   * killed or crashed. Without it bind will fail until all such connections
   * time out. Linux does not allow multiple live binds on same ip:port
   * regardless of SO_REUSEADDR (unlike some other flavors of Unix).
   * Turn it on before you call bind(). */

  /* Same, useful if you want to force family (e.g. IPv6) */

  /* Assign sin[6]_port member if the socket is an AF_INET[6] one,
   * otherwise no-op. Useful for ftp.
   * NB: does NOT do htons() internally, just direct assignment. */

  // NB: will return short write on error, not -1,
  // if some data was written before error occurred

  #[no_mangle]
  static mut logmode: smallint;

  /* delims[0] is a comment char (use '\0' to disable), the rest are token delimiters */

  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
}

pub type __rlim64_t = libc::c_ulong;
pub type __socklen_t = libc::c_uint;

pub type __rlimit_resource = libc::c_uint;
pub const __RLIM_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_RTTIME: __rlimit_resource = 15;
pub const __RLIMIT_RTPRIO: __rlimit_resource = 14;
pub const __RLIMIT_NICE: __rlimit_resource = 13;
pub const __RLIMIT_MSGQUEUE: __rlimit_resource = 12;
pub const __RLIMIT_SIGPENDING: __rlimit_resource = 11;
pub const __RLIMIT_LOCKS: __rlimit_resource = 10;
pub const __RLIMIT_MEMLOCK: __rlimit_resource = 8;
pub const __RLIMIT_NPROC: __rlimit_resource = 6;
pub const RLIMIT_AS: __rlimit_resource = 9;
pub const __RLIMIT_OFILE: __rlimit_resource = 7;
pub const RLIMIT_NOFILE: __rlimit_resource = 7;
pub const __RLIMIT_RSS: __rlimit_resource = 5;
pub const RLIMIT_CORE: __rlimit_resource = 4;
pub const RLIMIT_STACK: __rlimit_resource = 3;
pub const RLIMIT_DATA: __rlimit_resource = 2;
pub const RLIMIT_FSIZE: __rlimit_resource = 1;
pub const RLIMIT_CPU: __rlimit_resource = 0;

pub type rlim_t = __rlim64_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rlimit {
  pub rlim_cur: rlim_t,
  pub rlim_max: rlim_t,
}

pub type __rlimit_resource_t = __rlimit_resource;
pub type __fd_mask = libc::c_long;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fd_set {
  pub fds_bits: [__fd_mask; 16],
}
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

#[repr(C)]
#[derive(Copy, Clone)]
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_un {
  pub sun_family: sa_family_t,
  pub sun_path: [libc::c_char; 108],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_0 {
  pub __u6_addr8: [u8; 16],
  pub __u6_addr16: [u16; 8],
  pub __u6_addr32: [u32; 4],
}

pub type in_port_t = u16;

pub type in_addr_t = u32;

#[repr(C)]
#[derive(Copy, Clone)]
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
pub type C2RustUnnamed_1 = libc::c_uint;
pub const IPPROTO_MAX: C2RustUnnamed_1 = 256;
pub const IPPROTO_RAW: C2RustUnnamed_1 = 255;
pub const IPPROTO_MPLS: C2RustUnnamed_1 = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed_1 = 136;
pub const IPPROTO_SCTP: C2RustUnnamed_1 = 132;
pub const IPPROTO_COMP: C2RustUnnamed_1 = 108;
pub const IPPROTO_PIM: C2RustUnnamed_1 = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed_1 = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed_1 = 94;
pub const IPPROTO_MTP: C2RustUnnamed_1 = 92;
pub const IPPROTO_AH: C2RustUnnamed_1 = 51;
pub const IPPROTO_ESP: C2RustUnnamed_1 = 50;
pub const IPPROTO_GRE: C2RustUnnamed_1 = 47;
pub const IPPROTO_RSVP: C2RustUnnamed_1 = 46;
pub const IPPROTO_IPV6: C2RustUnnamed_1 = 41;
pub const IPPROTO_DCCP: C2RustUnnamed_1 = 33;
pub const IPPROTO_TP: C2RustUnnamed_1 = 29;
pub const IPPROTO_IDP: C2RustUnnamed_1 = 22;
pub const IPPROTO_UDP: C2RustUnnamed_1 = 17;
pub const IPPROTO_PUP: C2RustUnnamed_1 = 12;
pub const IPPROTO_EGP: C2RustUnnamed_1 = 8;
pub const IPPROTO_TCP: C2RustUnnamed_1 = 6;
pub const IPPROTO_IPIP: C2RustUnnamed_1 = 4;
pub const IPPROTO_IGMP: C2RustUnnamed_1 = 2;
pub const IPPROTO_ICMP: C2RustUnnamed_1 = 1;
pub const IPPROTO_IP: C2RustUnnamed_1 = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct servent {
  pub s_name: *mut libc::c_char,
  pub s_aliases: *mut *mut libc::c_char,
  pub s_port: libc::c_int,
  pub s_proto: *mut libc::c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_2 {
  pub _pad: [libc::c_int; 28],
  pub _kill: C2RustUnnamed_11,
  pub _timer: C2RustUnnamed_10,
  pub _rt: C2RustUnnamed_9,
  pub _sigchld: C2RustUnnamed_8,
  pub _sigfault: C2RustUnnamed_5,
  pub _sigpoll: C2RustUnnamed_4,
  pub _sigsys: C2RustUnnamed_3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_3 {
  pub _call_addr: *mut libc::c_void,
  pub _syscall: libc::c_int,
  pub _arch: libc::c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_4 {
  pub si_band: libc::c_long,
  pub si_fd: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_5 {
  pub si_addr: *mut libc::c_void,
  pub si_addr_lsb: libc::c_short,
  pub _bounds: C2RustUnnamed_6,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_6 {
  pub _addr_bnd: C2RustUnnamed_7,
  pub _pkey: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_7 {
  pub _lower: *mut libc::c_void,
  pub _upper: *mut libc::c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_8 {
  pub si_pid: pid_t,
  pub si_uid: uid_t,
  pub si_status: libc::c_int,
  pub si_utime: libc::clock_t,
  pub si_stime: libc::clock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_9 {
  pub si_pid: pid_t,
  pub si_uid: uid_t,
  pub si_sigval: sigval,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_10 {
  pub si_tid: libc::c_int,
  pub si_overrun: libc::c_int,
  pub si_sigval: sigval,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_11 {
  pub si_pid: pid_t,
  pub si_uid: uid_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_12 {
  pub sa_handler: __sighandler_t,
  pub sa_sigaction:
    Option<unsafe extern "C" fn(_: libc::c_int, _: *mut siginfo_t, _: *mut libc::c_void) -> ()>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timezone {
  pub tz_minuteswest: libc::c_int,
  pub tz_dsttime: libc::c_int,
}
pub type __timezone_ptr_t = *mut timezone;
/* Useful for having small structure members/global variables */
pub type socktype_t = i8;
pub type family_t = i8;

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_13 {
  pub sa: sockaddr,
  pub sin: sockaddr_in,
  pub sin6: sockaddr_in6,
}
pub type C2RustUnnamed_14 = libc::c_uint;
pub const LSA_SIZEOF_SA: C2RustUnnamed_14 = 28;
pub const LSA_LEN_SIZE: C2RustUnnamed_14 = 4;
pub type C2RustUnnamed_15 = libc::c_uint;
pub const LOGMODE_BOTH: C2RustUnnamed_15 = 3;
pub const LOGMODE_SYSLOG: C2RustUnnamed_15 = 2;
pub const LOGMODE_STDIO: C2RustUnnamed_15 = 1;
pub const LOGMODE_NONE: C2RustUnnamed_15 = 0;
/*
 * Config file parser
 */
pub type C2RustUnnamed_16 = libc::c_uint;
// comments are recognized even if there is whitespace before
// ("line start><space><tab><space>#comment" is also comment, not only "line start>#comment")
// NORMAL is:
// * remove leading and trailing delimiters and collapse
//   multiple delimiters into one
// * warn and continue if less than mintokens delimiters found
// * grab everything into last token
// * comments are recognized even if they aren't the first char
pub const PARSE_NORMAL: C2RustUnnamed_16 = 4653056;
// delim[0] and delim[1] are two different allowed comment chars
// (so far, delim[0] will only work as comment char for full-line comment)
// (IOW: it works as if PARSE_EOL_COMMENTS is not set. sysctl applet is okay with this)
pub const PARSE_WS_COMMENTS: C2RustUnnamed_16 = 16777216;
// comments are recognized even if they aren't the first char
pub const PARSE_ALT_COMMENTS: C2RustUnnamed_16 = 8388608;
pub const PARSE_EOL_COMMENTS: C2RustUnnamed_16 = 4194304;
// die if < min tokens found
// keep a copy of current line
pub const PARSE_KEEP_COPY: C2RustUnnamed_16 = 2097152;
// last token takes entire remainder of the line
pub const PARSE_MIN_DIE: C2RustUnnamed_16 = 1048576;
// trim leading and trailing delimiters
// TODO: COLLAPSE and TRIM seem to always go in pair
pub const PARSE_GREEDY: C2RustUnnamed_16 = 262144;
// treat consecutive delimiters as one
pub const PARSE_TRIM: C2RustUnnamed_16 = 131072;
pub const PARSE_COLLAPSE: C2RustUnnamed_16 = 65536;

//extern const int const_int_1;
/* This struct is deliberately not defined. */
/* See docs/keep_data_small.txt */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct globals {
  pub rlim_ofile_cur: rlim_t,
  pub rlim_ofile: rlimit,
  pub serv_list: *mut servtab_t,
  pub global_queuelen: libc::c_int,
  pub maxsock: libc::c_int,
  pub prev_maxsock: libc::c_int,
  pub max_concurrency: libc::c_uint,
  pub alarm_armed: smallint,
  pub real_uid: uid_t,
  pub config_filename: *const libc::c_char,
  pub parser: *mut parser_t,
  pub default_local_hostname: *mut libc::c_char,
  pub end_ring: *mut libc::c_char,
  pub ring_pos: *mut libc::c_char,
  pub ring: [libc::c_char; 128],
  pub allsock: fd_set,
  pub line: [libc::c_char; 256],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct servtab_t {
  pub se_fd: libc::c_int,
  pub se_local_hostname: *mut libc::c_char,
  pub se_service: *mut libc::c_char,
  pub se_proto: *mut libc::c_char,
  pub se_wait: pid_t,
  pub se_socktype: socktype_t,
  pub se_family: family_t,
  pub se_proto_no: smallint,
  pub se_checked: smallint,
  pub se_max: libc::c_uint,
  pub se_count: libc::c_uint,
  pub se_time: libc::c_uint,
  pub se_user: *mut libc::c_char,
  pub se_group: *mut libc::c_char,
  pub se_builtin: *const builtin,
  pub se_next: *mut servtab_t,
  pub se_lsa: *mut len_and_sockaddr,
  pub se_program: *mut libc::c_char,
  pub se_argv: [*mut libc::c_char; 21],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct builtin {
  pub bi_service7: [libc::c_char; 7],
  pub bi_fork: u8,
  pub bi_stream_fn: Option<unsafe extern "C" fn(_: libc::c_int, _: *mut servtab_t) -> ()>,
  pub bi_dgram_fn: Option<unsafe extern "C" fn(_: libc::c_int, _: *mut servtab_t) -> ()>,
}
pub const LINE_SIZE: C2RustUnnamed_18 = 672;
pub const BUFSIZE: C2RustUnnamed_17 = 12288;
pub type C2RustUnnamed_17 = libc::c_uint;
/* _at least_ 256, see LINE_SIZE */
pub type C2RustUnnamed_18 = libc::c_uint;
pub const FD_CHUNK: C2RustUnnamed_19 = 32;
pub type C2RustUnnamed_19 = libc::c_uint;

static mut builtins: [builtin; 5] = [
  {
    let mut init = builtin {
      bi_service7: [101, 99, 104, 111, 0, 0, 0],
      bi_fork: 1i32 as u8,
      bi_stream_fn: Some(
        echo_stream as unsafe extern "C" fn(_: libc::c_int, _: *mut servtab_t) -> (),
      ),
      bi_dgram_fn: Some(echo_dg as unsafe extern "C" fn(_: libc::c_int, _: *mut servtab_t) -> ()),
    };
    init
  },
  {
    let mut init = builtin {
      bi_service7: [100, 105, 115, 99, 97, 114, 100],
      bi_fork: 1i32 as u8,
      bi_stream_fn: Some(
        discard_stream as unsafe extern "C" fn(_: libc::c_int, _: *mut servtab_t) -> (),
      ),
      bi_dgram_fn: Some(
        discard_dg as unsafe extern "C" fn(_: libc::c_int, _: *mut servtab_t) -> (),
      ),
    };
    init
  },
  {
    let mut init = builtin {
      bi_service7: [99, 104, 97, 114, 103, 101, 110],
      bi_fork: 1i32 as u8,
      bi_stream_fn: Some(
        chargen_stream as unsafe extern "C" fn(_: libc::c_int, _: *mut servtab_t) -> (),
      ),
      bi_dgram_fn: Some(
        chargen_dg as unsafe extern "C" fn(_: libc::c_int, _: *mut servtab_t) -> (),
      ),
    };
    init
  },
  {
    let mut init = builtin {
      bi_service7: [116, 105, 109, 101, 0, 0, 0],
      bi_fork: 0 as u8,
      bi_stream_fn: Some(
        machtime_stream as unsafe extern "C" fn(_: libc::c_int, _: *mut servtab_t) -> (),
      ),
      bi_dgram_fn: Some(
        machtime_dg as unsafe extern "C" fn(_: libc::c_int, _: *mut servtab_t) -> (),
      ),
    };
    init
  },
  {
    let mut init = builtin {
      bi_service7: [100, 97, 121, 116, 105, 109, 101],
      bi_fork: 0 as u8,
      bi_stream_fn: Some(
        daytime_stream as unsafe extern "C" fn(_: libc::c_int, _: *mut servtab_t) -> (),
      ),
      bi_dgram_fn: Some(
        daytime_dg as unsafe extern "C" fn(_: libc::c_int, _: *mut servtab_t) -> (),
      ),
    };
    init
  },
];

unsafe extern "C" fn maybe_close(mut fd: libc::c_int) {
  if fd >= 0 {
    close(fd);
  };
}
// TODO: move to libbb?
unsafe extern "C" fn xzalloc_lsa(mut family: libc::c_int) -> *mut len_and_sockaddr {
  let mut lsa: *mut len_and_sockaddr = std::ptr::null_mut();
  let mut sz: libc::c_int = 0;
  sz = ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as libc::c_int;
  if family == 1i32 {
    sz = ::std::mem::size_of::<sockaddr_un>() as libc::c_ulong as libc::c_int
  }
  if family == 10i32 {
    sz = ::std::mem::size_of::<sockaddr_in6>() as libc::c_ulong as libc::c_int
  }
  lsa = crate::libbb::xfuncs_printf::xzalloc((LSA_LEN_SIZE as libc::c_int + sz) as size_t)
    as *mut len_and_sockaddr;
  (*lsa).len = sz as socklen_t;
  (*lsa).u.sa.sa_family = family as sa_family_t;
  return lsa;
}
unsafe extern "C" fn rearm_alarm() {
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).alarm_armed == 0 {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).alarm_armed = 1i32 as smallint;
    alarm(60i32 as libc::c_uint);
  };
}
unsafe extern "C" fn block_CHLD_HUP_ALRM(mut m: *mut sigset_t) {
  sigemptyset(m);
  sigaddset(m, 17i32);
  sigaddset(m, 1i32);
  sigaddset(m, 14i32);
  crate::libbb::signals::sigprocmask2(0i32, m);
  /* old sigmask is stored in m */
}
unsafe extern "C" fn restore_sigmask(mut m: *mut sigset_t) {
  sigprocmask(2i32, m, 0 as *mut sigset_t);
}
/* FEATURE_INETD_RPC */
unsafe extern "C" fn bump_nofile() {
  let mut rl: rlimit = rlimit {
    rlim_cur: 0,
    rlim_max: 0,
  };
  /* Never fails under Linux (except if you pass it bad arguments) */
  getrlimit(RLIMIT_NOFILE, &mut rl);
  rl.rlim_cur = if rl.rlim_max
    < rl
      .rlim_cur
      .wrapping_add(FD_CHUNK as libc::c_int as libc::c_ulong)
  {
    rl.rlim_max
  } else {
    rl.rlim_cur
      .wrapping_add(FD_CHUNK as libc::c_int as libc::c_ulong)
  };
  rl.rlim_cur = if (1024i32 as libc::c_ulong)
    < rl
      .rlim_cur
      .wrapping_add(FD_CHUNK as libc::c_int as libc::c_ulong)
  {
    1024i32 as libc::c_ulong
  } else {
    rl.rlim_cur
      .wrapping_add(FD_CHUNK as libc::c_int as libc::c_ulong)
  };
  if rl.rlim_cur <= (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).rlim_ofile_cur {
    crate::libbb::verror_msg::bb_error_msg(
      b"can\'t extend file limit, max = %d\x00" as *const u8 as *const libc::c_char,
      rl.rlim_cur as libc::c_int,
    );
    return;
  }
  if setrlimit(RLIMIT_NOFILE, &mut rl) < 0 {
    crate::libbb::perror_msg::bb_simple_perror_msg(
      b"setrlimit\x00" as *const u8 as *const libc::c_char,
    );
    return;
  }
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).rlim_ofile_cur = rl.rlim_cur;
}
unsafe extern "C" fn remove_fd_from_set(mut fd: libc::c_int) {
  if fd >= 0 {
    let ref mut fresh0 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .allsock
      .fds_bits
      [(fd / (8i32 * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int)) as usize];
    *fresh0 &= !((1u64
      << fd % (8i32 * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
      as __fd_mask);
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).maxsock = -1i32
  };
}
unsafe extern "C" fn add_fd_to_set(mut fd: libc::c_int) {
  if fd >= 0 {
    let ref mut fresh1 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .allsock
      .fds_bits
      [(fd / (8i32 * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int)) as usize];
    *fresh1 |= (1u64
      << fd % (8i32 * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
      as __fd_mask;
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).maxsock >= 0
      && fd > (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).maxsock
    {
      let ref mut fresh2 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).maxsock;
      *fresh2 = fd;
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).prev_maxsock = *fresh2;
      if fd as rlim_t
        > (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
          .rlim_ofile_cur
          .wrapping_sub(8i32 as libc::c_ulong)
      {
        bump_nofile();
      }
    }
  };
}
unsafe extern "C" fn recalculate_maxsock() {
  let mut fd: libc::c_int = 0;
  /* We may have no services, in this case maxsock should still be >= 0
   * (code elsewhere is not happy with maxsock == -1) */
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).maxsock = 0; /* not a string in fact */
  while fd <= (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).prev_maxsock {
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .allsock
      .fds_bits
      [(fd / (8i32 * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int)) as usize]
      & (1u64 << fd % (8i32 * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
        as __fd_mask
      != 0
    {
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).maxsock = fd
    } /* paranoia */
    fd += 1
  } /* struct copy */
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).prev_maxsock =
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).maxsock;
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).maxsock as rlim_t
    > (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .rlim_ofile_cur
      .wrapping_sub(8i32 as libc::c_ulong)
  {
    bump_nofile();
  };
}
unsafe extern "C" fn prepare_socket_fd(mut sep: *mut servtab_t) {
  let mut r: libc::c_int = 0;
  let mut fd: libc::c_int = 0;
  fd = socket(
    (*sep).se_family as libc::c_int,
    (*sep).se_socktype as libc::c_int,
    0,
  );
  if fd < 0 {
    crate::libbb::perror_msg::bb_simple_perror_msg(
      b"socket\x00" as *const u8 as *const libc::c_char,
    );
    return;
  }
  crate::libbb::xconnect::setsockopt_reuseaddr(fd);
  if (*sep).se_family as libc::c_int == 1i32 {
    let mut sun: *mut sockaddr_un = std::ptr::null_mut();
    sun = &mut (*(*sep).se_lsa).u.sa as *mut sockaddr as *mut sockaddr_un;
    unlink((*sun).sun_path.as_mut_ptr());
  }
  r = bind(
    fd,
    __CONST_SOCKADDR_ARG {
      __sockaddr__: &mut (*(*sep).se_lsa).u.sa,
    },
    (*(*sep).se_lsa).len,
  );
  if r < 0 {
    crate::libbb::perror_msg::bb_perror_msg(
      b"%s/%s: bind\x00" as *const u8 as *const libc::c_char,
      (*sep).se_service,
      (*sep).se_proto,
    );
    close(fd);
    rearm_alarm();
    return;
  }
  if (*sep).se_socktype as libc::c_int == SOCK_STREAM as libc::c_int {
    listen(
      fd,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).global_queuelen,
    );
  }
  add_fd_to_set(fd);
  (*sep).se_fd = fd;
}
unsafe extern "C" fn reopen_config_file() -> libc::c_int {
  free(
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).default_local_hostname as *mut libc::c_void,
  );
  let ref mut fresh3 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).default_local_hostname;
  *fresh3 = crate::libbb::xfuncs_printf::xstrdup(b"*\x00" as *const u8 as *const libc::c_char);
  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .parser
    .is_null()
  {
    crate::libbb::parse_config::config_close(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).parser,
    );
  }
  let ref mut fresh4 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).parser;
  *fresh4 = crate::libbb::parse_config::config_open(
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).config_filename,
  );
  return ((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).parser != std::ptr::null_mut())
    as libc::c_int;
}
unsafe extern "C" fn close_config_file() {
  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .parser
    .is_null()
  {
    crate::libbb::parse_config::config_close(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).parser,
    );
    let ref mut fresh5 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).parser;
    *fresh5 = std::ptr::null_mut()
  };
}
unsafe extern "C" fn free_servtab_strings(mut cp: *mut servtab_t) {
  let mut i: libc::c_int = 0;
  free((*cp).se_local_hostname as *mut libc::c_void);
  free((*cp).se_service as *mut libc::c_void);
  free((*cp).se_proto as *mut libc::c_void);
  free((*cp).se_user as *mut libc::c_void);
  free((*cp).se_group as *mut libc::c_void);
  free((*cp).se_lsa as *mut libc::c_void);
  free((*cp).se_program as *mut libc::c_void);
  i = 0;
  while i < 20i32 {
    free((*cp).se_argv[i as usize] as *mut libc::c_void);
    i += 1
  }
}
unsafe extern "C" fn new_servtab() -> *mut servtab_t {
  let mut newtab: *mut servtab_t = crate::libbb::xfuncs_printf::xzalloc(::std::mem::size_of::<
    servtab_t,
  >() as libc::c_ulong) as *mut servtab_t;
  (*newtab).se_fd = -1i32;
  return newtab;
}
unsafe extern "C" fn dup_servtab(mut sep: *mut servtab_t) -> *mut servtab_t {
  let mut newtab: *mut servtab_t = std::ptr::null_mut();
  let mut argc: libc::c_int = 0;
  newtab = new_servtab();
  *newtab = *sep;
  /* deep-copying strings */
  (*newtab).se_service = crate::libbb::xfuncs_printf::xstrdup((*newtab).se_service);
  (*newtab).se_proto = crate::libbb::xfuncs_printf::xstrdup((*newtab).se_proto);
  (*newtab).se_user = crate::libbb::xfuncs_printf::xstrdup((*newtab).se_user);
  (*newtab).se_group = crate::libbb::xfuncs_printf::xstrdup((*newtab).se_group);
  (*newtab).se_program = crate::libbb::xfuncs_printf::xstrdup((*newtab).se_program);
  argc = 0;
  while argc <= 20i32 {
    (*newtab).se_argv[argc as usize] =
      crate::libbb::xfuncs_printf::xstrdup((*newtab).se_argv[argc as usize]);
    argc += 1
  }
  /* NB: se_fd, se_hostaddr and se_next are always
   * overwrittend by callers, so we don't bother resetting them
   * to NULL/0/-1 etc */
  return newtab;
}
/* gcc generates much more code if this is inlined */
#[inline(never)]
unsafe extern "C" fn parse_one_line() -> *mut servtab_t {
  let mut current_block: u64;
  let mut argc: libc::c_int = 0;
  let mut token: [*mut libc::c_char; 26] = [0 as *mut libc::c_char; 26];
  let mut p: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut arg: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut hostdelim: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut sep: *mut servtab_t = std::ptr::null_mut();
  let mut nsep: *mut servtab_t = std::ptr::null_mut();
  loop {
    sep = new_servtab();
    loop
    /*sep->se_local_hostname = NULL; - redundant */
    /* (we'll overwrite this field anyway) */
    {
      argc = crate::libbb::parse_config::config_read(
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).parser,
        token.as_mut_ptr(),
        (PARSE_NORMAL as libc::c_int | (1i32 & 0xffi32) << 8i32 | 6i32 + 20i32 & 0xffi32)
          as libc::c_uint,
        b"# \t\x00" as *const u8 as *const libc::c_char,
      );
      if argc == 0 {
        free(sep as *mut libc::c_void);
        return std::ptr::null_mut();
      }
      /* [host:]service socktype proto wait user[:group] prog [args] */
      /* Check for "host:...." line */
      arg = token[0];
      hostdelim = strrchr(arg, ':' as i32);
      if !hostdelim.is_null() {
        *hostdelim = '\u{0}' as i32 as libc::c_char;
        (*sep).se_local_hostname = crate::libbb::xfuncs_printf::xstrdup(arg);
        arg = hostdelim.offset(1);
        if !(*arg as libc::c_int == '\u{0}' as i32 && argc == 1i32) {
          break;
        }
        /* Line has just "host:", change the
         * default host for the following lines. */
        free(
          (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).default_local_hostname
            as *mut libc::c_void,
        );
        let ref mut fresh6 =
          (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).default_local_hostname;
        *fresh6 = (*sep).se_local_hostname
      } else {
        (*sep).se_local_hostname = crate::libbb::xfuncs_printf::xstrdup(
          (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).default_local_hostname,
        );
        break;
      }
    }
    /* service socktype proto wait user[:group] prog [args] */
    (*sep).se_service = crate::libbb::xfuncs_printf::xstrdup(arg);
    /* socktype proto wait user[:group] prog [args] */
    if !(argc < 6i32) {
      static mut SOCK_xxx: [i8; 6] = [
        -1i32 as i8,
        SOCK_STREAM as libc::c_int as i8,
        SOCK_DGRAM as libc::c_int as i8,
        SOCK_RDM as libc::c_int as i8,
        SOCK_SEQPACKET as libc::c_int as i8,
        SOCK_RAW as libc::c_int as i8,
      ];
      (*sep).se_socktype = SOCK_xxx[(1i32
        + crate::libbb::compare_string_array::index_in_strings(
          b"stream\x00dgram\x00rdm\x00seqpacket\x00raw\x00\x00" as *const u8 as *const libc::c_char,
          token[1],
        )) as usize];
      /* {unix,[rpc/]{tcp,udp}[6]} wait user[:group] prog [args] */
      arg = crate::libbb::xfuncs_printf::xstrdup(token[2]);
      (*sep).se_proto = arg;
      if strcmp(arg, b"unix\x00" as *const u8 as *const libc::c_char) == 0 {
        (*sep).se_family = 1i32 as family_t;
        current_block = 11626999923138678822;
      } else {
        let mut six: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
        (*sep).se_family = 2i32 as family_t;
        six = crate::libbb::last_char_is::last_char_is(arg, '6' as i32);
        if !six.is_null() {
          *six = '\u{0}' as i32 as libc::c_char;
          (*sep).se_family = 10i32 as family_t
        }
        if !crate::libbb::compare_string_array::is_prefixed_with(
          arg,
          b"rpc/\x00" as *const u8 as *const libc::c_char,
        )
        .is_null()
        {
          crate::libbb::verror_msg::bb_simple_error_msg(
            b"no support for rpc services\x00" as *const u8 as *const libc::c_char,
          );
          current_block = 16152178309243456251;
        } else {
          /* we don't really need getprotobyname()! */
          if strcmp(arg, b"tcp\x00" as *const u8 as *const libc::c_char) == 0 {
            (*sep).se_proto_no = IPPROTO_TCP as libc::c_int as smallint
          } /* = 6 */
          if strcmp(arg, b"udp\x00" as *const u8 as *const libc::c_char) == 0 {
            (*sep).se_proto_no = IPPROTO_UDP as libc::c_int as smallint
          } /* = 17 */
          if !six.is_null() {
            *six = '6' as i32 as libc::c_char
          }
          if (*sep).se_proto_no == 0 {
            current_block = 16152178309243456251;
          } else {
            current_block = 11626999923138678822;
          }
        }
      }
      match current_block {
        16152178309243456251 => {}
        _ => {
          /* [no]wait[.max] user[:group] prog [args] */
          arg = token[3];
          (*sep).se_max = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).max_concurrency;
          p = strchr(arg, '.' as i32);
          if !p.is_null() {
            let fresh7 = p;
            p = p.offset(1);
            *fresh7 = '\u{0}' as i32 as libc::c_char;
            (*sep).se_max =
              crate::libbb::bb_strtonum::bb_strtou(p, 0 as *mut *mut libc::c_char, 10i32);
            if *bb_errno != 0 {
              current_block = 16152178309243456251;
            } else {
              current_block = 5892776923941496671;
            }
          } else {
            current_block = 5892776923941496671;
          }
          match current_block {
            16152178309243456251 => {}
            _ => {
              (*sep).se_wait = (*arg.offset(0) as libc::c_int != 'n' as i32
                || *arg.offset(1) as libc::c_int != 'o' as i32)
                as libc::c_int;
              if (*sep).se_wait == 0 {
                /* "no" seen */
                arg = arg.offset(2)
              }
              if !(strcmp(arg, b"wait\x00" as *const u8 as *const libc::c_char) != 0) {
                /* user[:group] prog [args] */
                (*sep).se_user = crate::libbb::xfuncs_printf::xstrdup(token[4]);
                arg = strchr((*sep).se_user, '.' as i32);
                if arg.is_null() {
                  arg = strchr((*sep).se_user, ':' as i32)
                }
                if !arg.is_null() {
                  let fresh8 = arg;
                  arg = arg.offset(1);
                  *fresh8 = '\u{0}' as i32 as libc::c_char;
                  (*sep).se_group = crate::libbb::xfuncs_printf::xstrdup(arg)
                }
                /* prog [args] */
                (*sep).se_program = crate::libbb::xfuncs_printf::xstrdup(token[5]);
                if strcmp(
                  (*sep).se_program,
                  b"internal\x00" as *const u8 as *const libc::c_char,
                ) == 0
                  && strlen((*sep).se_service) <= 7i32 as libc::c_ulong
                  && ((*sep).se_socktype as libc::c_int == SOCK_STREAM as libc::c_int
                    || (*sep).se_socktype as libc::c_int == SOCK_DGRAM as libc::c_int)
                {
                  let mut i: libc::c_uint = 0;
                  i = 0 as libc::c_uint;
                  loop {
                    if !(i
                      < (::std::mem::size_of::<[builtin; 5]>() as libc::c_ulong)
                        .wrapping_div(::std::mem::size_of::<builtin>() as libc::c_ulong)
                        as libc::c_uint)
                    {
                      current_block = 4216521074440650966;
                      break;
                    }
                    if strncmp(
                      builtins[i as usize].bi_service7.as_ptr(),
                      (*sep).se_service,
                      7i32 as libc::c_ulong,
                    ) == 0
                    {
                      current_block = 3513661652909667213;
                      break;
                    }
                    i = i.wrapping_add(1)
                  }
                  match current_block {
                    4216521074440650966 => {
                      crate::libbb::verror_msg::bb_error_msg(
                        b"unknown internal service %s\x00" as *const u8 as *const libc::c_char,
                        (*sep).se_service,
                      );
                      current_block = 16152178309243456251;
                    }
                    _ => {
                      (*sep).se_builtin = &*builtins.as_ptr().offset(i as isize) as *const builtin;
                      /* stream builtins must be "nowait", dgram must be "wait" */
                      if (*sep).se_wait
                        != ((*sep).se_socktype as libc::c_int == SOCK_DGRAM as libc::c_int)
                          as libc::c_int
                      {
                        current_block = 16152178309243456251;
                      } else {
                        current_block = 6367734732029634840;
                      }
                    }
                  }
                } else {
                  current_block = 6367734732029634840;
                }
                match current_block {
                  16152178309243456251 => {}
                  _ => {
                    argc = 0;
                    while argc < 20i32 && {
                      arg = token[(6i32 + argc) as usize];
                      !arg.is_null()
                    } {
                      let fresh9 = argc;
                      argc = argc + 1;
                      (*sep).se_argv[fresh9 as usize] = crate::libbb::xfuncs_printf::xstrdup(arg)
                    }
                    /* Some inetd.conf files have no argv's, not even argv[0].
                     * Fix them up.
                     * (Technically, programs can be execed with argv[0] = NULL,
                     * but many programs do not like that at all) */
                    if argc == 0 {
                      (*sep).se_argv[0] = crate::libbb::xfuncs_printf::xstrdup((*sep).se_program)
                    }
                    /* catch mixups. "<service> stream udp ..." == wtf */
                    if (*sep).se_socktype as libc::c_int == SOCK_STREAM as libc::c_int {
                      if (*sep).se_proto_no as libc::c_int == IPPROTO_UDP as libc::c_int {
                        current_block = 16152178309243456251;
                      } else {
                        current_block = 993425571616822999;
                      }
                    } else {
                      current_block = 993425571616822999;
                    }
                    match current_block {
                      16152178309243456251 => {}
                      _ => {
                        if !((*sep).se_socktype as libc::c_int == SOCK_DGRAM as libc::c_int) {
                          break;
                        }
                        if !((*sep).se_proto_no as libc::c_int == IPPROTO_TCP as libc::c_int) {
                          break;
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
    /* not tcp/udp?? */
    crate::libbb::verror_msg::bb_error_msg(
      b"parse error on line %u, line is ignored\x00" as *const u8 as *const libc::c_char,
      (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).parser).lineno,
    );
    /* Just "goto more" can make sep to carry over e.g.
     * "rpc"-ness (by having se_rpcver_lo != 0).
     * We will be more paranoid: */
    free_servtab_strings(sep);
    free(sep as *mut libc::c_void);
  }
  loop
  //bb_error_msg(
  //	"ENTRY[%s][%s][%s][%d][%d][%d][%d][%d][%s][%s][%s]",
  //	sep->se_local_hostname, sep->se_service, sep->se_proto, sep->se_wait, sep->se_proto_no,
  //	sep->se_max, sep->se_count, sep->se_time, sep->se_user, sep->se_group, sep->se_program);
  /* check if the hostname specifier is a comma separated list
   * of hostnames. we'll make new entries for each address. */
  {
    hostdelim = strrchr((*sep).se_local_hostname, ',' as i32);
    if hostdelim.is_null() {
      break;
    }
    nsep = dup_servtab(sep);
    /* NUL terminate the hostname field of the existing entry,
     * and make a dup for the new entry. */
    let fresh10 = hostdelim;
    hostdelim = hostdelim.offset(1);
    *fresh10 = '\u{0}' as i32 as libc::c_char;
    (*nsep).se_local_hostname = crate::libbb::xfuncs_printf::xstrdup(hostdelim);
    (*nsep).se_next = (*sep).se_next;
    (*sep).se_next = nsep
  }
  /* was doing it here: */
  /* DNS resolution, create copies for each IP address */
  /* IPv6-ization destroyed it :( */
  return sep; /* struct copy */
}
unsafe extern "C" fn insert_in_servlist(mut cp: *mut servtab_t) -> *mut servtab_t {
  let mut sep: *mut servtab_t = std::ptr::null_mut();
  let mut omask: sigset_t = std::mem::zeroed();
  sep = new_servtab();
  *sep = *cp;
  (*sep).se_fd = -1i32;
  block_CHLD_HUP_ALRM(&mut omask);
  (*sep).se_next = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).serv_list;
  let ref mut fresh11 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).serv_list;
  *fresh11 = sep;
  restore_sigmask(&mut omask);
  return sep;
}
unsafe extern "C" fn same_serv_addr_proto(
  mut old: *mut servtab_t,
  mut new: *mut servtab_t,
) -> libc::c_int {
  if strcmp((*old).se_local_hostname, (*new).se_local_hostname) != 0 {
    return 0;
  }
  if strcmp((*old).se_service, (*new).se_service) != 0 {
    return 0;
  }
  if strcmp((*old).se_proto, (*new).se_proto) != 0 {
    return 0;
  }
  return 1i32;
}
unsafe extern "C" fn reread_config_file(mut _sig: libc::c_int) {
  let mut sun: *mut sockaddr_un = std::ptr::null_mut();
  let mut current_block: u64;
  let mut sep: *mut servtab_t = std::ptr::null_mut();
  let mut cp: *mut servtab_t = std::ptr::null_mut();
  let mut sepp: *mut *mut servtab_t = std::ptr::null_mut();
  let mut lsa: *mut len_and_sockaddr = std::ptr::null_mut();
  let mut omask: sigset_t = std::mem::zeroed();
  let mut n: libc::c_uint = 0;
  let mut port: u16 = 0;
  let mut save_errno: libc::c_int = *bb_errno;
  if !(reopen_config_file() == 0) {
    sep = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).serv_list;
    while !sep.is_null() {
      (*sep).se_checked = 0 as smallint;
      sep = (*sep).se_next
    }
    loop {
      cp = parse_one_line();
      if cp.is_null() {
        break;
      }
      loop {
        sep = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).serv_list;
        loop {
          if sep.is_null() {
            current_block = 8831408221741692167;
            break;
          }
          if same_serv_addr_proto(sep, cp) != 0 {
            current_block = 4429232765501753682;
            break;
          }
          sep = (*sep).se_next
        }
        match current_block {
          4429232765501753682 => {
            let mut i: libc::c_int = 0;
            block_CHLD_HUP_ALRM(&mut omask);
            if (*cp).se_wait == 0 {
              /* New config says "nowait". If old one
               * was "wait", we currently may be waiting
               * for a child (and not accepting connects).
               * Stop waiting, start listening again.
               * (if it's not true, this op is harmless) */
              add_fd_to_set((*sep).se_fd);
            }
            (*sep).se_wait = (*cp).se_wait;
            (*sep).se_max = (*cp).se_max;
            /* string fields need more love - we don't want to leak them */
            let mut c: *mut libc::c_char = (*sep).se_user;
            (*sep).se_user = (*cp).se_user;
            (*cp).se_user = c;
            let mut c_0: *mut libc::c_char = (*sep).se_group;
            (*sep).se_group = (*cp).se_group;
            (*cp).se_group = c_0;
            let mut c_1: *mut libc::c_char = (*sep).se_program;
            (*sep).se_program = (*cp).se_program;
            (*cp).se_program = c_1;
            i = 0;
            while i < 20i32 {
              let mut c_2: *mut libc::c_char = (*sep).se_argv[i as usize];
              (*sep).se_argv[i as usize] = (*cp).se_argv[i as usize];
              (*cp).se_argv[i as usize] = c_2;
              i += 1
            }
            restore_sigmask(&mut omask);
            free_servtab_strings(cp);
          }
          _ => {
            /* not an "equal" servtab */
            sep = insert_in_servlist(cp)
          }
        }
        /* cp->string_fields are consumed by insert_in_servlist()
         * or freed at this point, cp itself is not yet freed. */
        (*sep).se_checked = 1i32 as smallint;
        /* create new len_and_sockaddr */
        match (*sep).se_family as libc::c_int {
          1 => {
            lsa = xzalloc_lsa(1i32); /* end of "switch (sep->se_family)" */
            sun = &mut (*lsa).u.sa as *mut sockaddr as *mut sockaddr_un;
            crate::libbb::safe_strncpy::safe_strncpy(
              (*sun).sun_path.as_mut_ptr(),
              (*sep).se_service,
              ::std::mem::size_of::<[libc::c_char; 108]>() as libc::c_ulong,
            );
            current_block = 7330218953828964527;
          }
          _ => {
            /* case AF_INET, case AF_INET6 */
            n = crate::libbb::bb_strtonum::bb_strtou(
              (*sep).se_service,
              0 as *mut *mut libc::c_char,
              10i32,
            );
            /* what port to listen on? */
            port = {
              let mut __v: libc::c_ushort = 0;
              let mut __x: libc::c_ushort = n as libc::c_ushort;
              if false {
                __v = (__x as libc::c_int >> 8i32 & 0xffi32
                  | (__x as libc::c_int & 0xffi32) << 8i32) as libc::c_ushort
              } else {
                let fresh12 = &mut __v;
                let fresh13;
                let fresh14 = __x;
                asm!("rorw $$8, ${0:w}" : "=r" (fresh13) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh12, fresh14)) : "cc");
                c2rust_asm_casts::AsmCast::cast_out(fresh12, fresh14, fresh13);
              }
              __v
            };
            if *bb_errno != 0 || n > 0xffffi32 as libc::c_uint {
              /* se_service is not numeric */
              let mut protoname: [libc::c_char; 4] = [0; 4];
              let mut sp: *mut servent = std::ptr::null_mut();
              /* can result only in "tcp" or "udp": */
              crate::libbb::safe_strncpy::safe_strncpy(
                protoname.as_mut_ptr(),
                (*sep).se_proto,
                4i32 as size_t,
              );
              sp = getservbyname((*sep).se_service, protoname.as_mut_ptr());
              if sp.is_null() {
                crate::libbb::verror_msg::bb_error_msg(
                  b"%s/%s: unknown service\x00" as *const u8 as *const libc::c_char,
                  (*sep).se_service,
                  (*sep).se_proto,
                );
                current_block = 15695848964033577090;
              } else {
                port = (*sp).s_port as u16;
                current_block = 2706659501864706830;
              }
            } else {
              current_block = 2706659501864706830;
            }
            match current_block {
              15695848964033577090 => {}
              _ => {
                if *(*sep).se_local_hostname.offset(0) as libc::c_int == '*' as i32
                  && *(*sep).se_local_hostname.offset(1) == 0
                {
                  lsa = xzalloc_lsa((*sep).se_family as libc::c_int);
                  crate::libbb::xconnect::set_nport(&mut (*lsa).u.sa, port as libc::c_uint);
                  current_block = 7330218953828964527;
                } else {
                  lsa = crate::libbb::xconnect::host_and_af2sockaddr(
                    (*sep).se_local_hostname,
                    ({
                      let mut __v: libc::c_ushort = 0;
                      let mut __x: libc::c_ushort = port;
                      if false {
                        __v = (__x as libc::c_int >> 8i32 & 0xffi32
                          | (__x as libc::c_int & 0xffi32) << 8i32)
                          as libc::c_ushort
                      } else {
                        let fresh15 = &mut __v;
                        let fresh16;
                        let fresh17 = __x;
                        asm!("rorw $$8, ${0:w}" : "=r" (fresh16) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh15, fresh17)) : "cc");
                        c2rust_asm_casts::AsmCast::cast_out(fresh15, fresh17, fresh16);
                      }
                      __v
                    }) as libc::c_int,
                    (*sep).se_family as sa_family_t,
                  );
                  if lsa.is_null() {
                    crate::libbb::verror_msg::bb_error_msg(
                      b"%s/%s: unknown host \'%s\'\x00" as *const u8 as *const libc::c_char,
                      (*sep).se_service,
                      (*sep).se_proto,
                      (*sep).se_local_hostname,
                    );
                    current_block = 15695848964033577090;
                  } else {
                    current_block = 7330218953828964527;
                  }
                }
              }
            }
          }
        }
        match current_block {
          7330218953828964527 => {
            /* did lsa change? Then close/open */
            if (*sep).se_lsa.is_null()
              || (*lsa).len != (*(*sep).se_lsa).len
              || memcmp(
                &mut (*lsa).u.sa as *mut sockaddr as *const libc::c_void,
                &mut (*(*sep).se_lsa).u.sa as *mut sockaddr as *const libc::c_void,
                (*lsa).len as libc::c_ulong,
              ) != 0
            {
              remove_fd_from_set((*sep).se_fd); /* end of "while (1) parse lines" */
              maybe_close((*sep).se_fd);
              free((*sep).se_lsa as *mut libc::c_void);
              (*sep).se_lsa = lsa;
              (*sep).se_fd = -1i32
            } else {
              free(lsa as *mut libc::c_void);
            }
            if (*sep).se_fd == -1i32 {
              prepare_socket_fd(sep);
            }
          }
          _ => {}
        }
        sep = (*cp).se_next;
        free(cp as *mut libc::c_void);
        cp = sep;
        if cp.is_null() {
          break;
        }
      }
    }
    close_config_file();
    /* Purge anything not looked at above - these are stale entries,
     * new config file doesnt have them. */
    block_CHLD_HUP_ALRM(&mut omask);
    sepp = &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).serv_list;
    loop {
      sep = *sepp;
      if sep.is_null() {
        break;
      }
      if (*sep).se_checked != 0 {
        sepp = &mut (*sep).se_next
      } else {
        *sepp = (*sep).se_next;
        remove_fd_from_set((*sep).se_fd);
        maybe_close((*sep).se_fd);
        if (*sep).se_family as libc::c_int == 1i32 {
          unlink((*sep).se_service);
        }
        free_servtab_strings(sep);
        free(sep as *mut libc::c_void);
      }
    }
    restore_sigmask(&mut omask);
  }
  *bb_errno = save_errno;
}
unsafe extern "C" fn reap_child(mut _sig: libc::c_int) {
  let mut pid: pid_t = 0;
  let mut status: libc::c_int = 0;
  let mut sep: *mut servtab_t = std::ptr::null_mut();
  let mut save_errno: libc::c_int = *bb_errno;
  loop {
    pid = crate::libbb::xfuncs::wait_any_nohang(&mut status);
    if pid <= 0 {
      break;
    }
    sep = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).serv_list;
    while !sep.is_null() {
      if (*sep).se_wait != pid {
        sep = (*sep).se_next
      } else {
        /* One of our "wait" services */
        if status & 0x7fi32 == 0 && (status & 0xff00i32) >> 8i32 != 0 {
          crate::libbb::verror_msg::bb_error_msg(
            b"%s: exit status %u\x00" as *const u8 as *const libc::c_char,
            (*sep).se_program,
            (status & 0xff00i32) >> 8i32,
          );
        } else if ((status & 0x7fi32) + 1i32) as libc::c_schar as libc::c_int >> 1i32 > 0 {
          crate::libbb::verror_msg::bb_error_msg(
            b"%s: exit signal %u\x00" as *const u8 as *const libc::c_char,
            (*sep).se_program,
            status & 0x7fi32,
          );
        }
        (*sep).se_wait = 1i32;
        add_fd_to_set((*sep).se_fd);
        break;
      }
    }
  }
  *bb_errno = save_errno;
}
unsafe extern "C" fn retry_network_setup(mut _sig: libc::c_int) {
  let mut save_errno: libc::c_int = *bb_errno;
  let mut sep: *mut servtab_t = std::ptr::null_mut();
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).alarm_armed = 0 as smallint;
  sep = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).serv_list;
  while !sep.is_null() {
    if (*sep).se_fd == -1i32 {
      prepare_socket_fd(sep);
    }
    sep = (*sep).se_next
  }
  *bb_errno = save_errno;
}
unsafe extern "C" fn clean_up_and_exit(mut _sig: libc::c_int) {
  let mut sep: *mut servtab_t = std::ptr::null_mut();
  /* XXX signal race walking sep list */
  sep = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).serv_list; /* for compiler */
  while !sep.is_null() {
    if !((*sep).se_fd == -1i32) {
      match (*sep).se_family as libc::c_int {
        1 => {
          unlink((*sep).se_service);
        }
        _ => {}
      }
    }
    sep = (*sep).se_next
  }
  crate::libbb::pidfile::remove_pidfile_std_path_and_ext(
    b"inetd\x00" as *const u8 as *const libc::c_char,
  );
  exit(0i32);
}
pub unsafe fn inetd_main(mut _argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
  let mut current_block: u64;
  let mut sa: sigaction = std::mem::zeroed();
  let mut saved_pipe_handler: sigaction = std::mem::zeroed();
  let mut sep: *mut servtab_t = std::ptr::null_mut();
  let mut sep2: *mut servtab_t = std::ptr::null_mut();
  let mut pwd: *mut passwd = std::ptr::null_mut();
  let mut grp: *mut group = std::ptr::null_mut();
  grp = grp;
  let mut opt: libc::c_int = 0;
  let mut pid: pid_t = 0;
  let mut omask: sigset_t = std::mem::zeroed();
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).rlim_ofile_cur = 64i32 as rlim_t;
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).global_queuelen = 128i32;
  let ref mut fresh18 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).config_filename;
  *fresh18 = b"/etc/inetd.conf\x00" as *const u8 as *const libc::c_char;
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).real_uid = getuid();
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).real_uid != 0 as libc::c_uint {
    /* run by non-root user */
    let ref mut fresh19 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).config_filename;
    *fresh19 = std::ptr::null()
  }
  /* -q N, -R N */
  opt = crate::libbb::getopt32::getopt32(
    argv,
    b"R:+feq:+\x00" as *const u8 as *const libc::c_char,
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).max_concurrency as *mut libc::c_uint,
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).global_queuelen as *mut libc::c_int,
  ) as libc::c_int;
  argv = argv.offset(optind as isize);
  //argc -= optind;
  if !(*argv.offset(0)).is_null() {
    let ref mut fresh20 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).config_filename;
    *fresh20 = *argv.offset(0)
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .config_filename
    .is_null()
  {
    crate::libbb::verror_msg::bb_simple_error_msg_and_die(
      b"non-root must specify config file\x00" as *const u8 as *const libc::c_char,
    );
  }
  if opt & 2i32 == 0 {
    crate::libbb::vfork_daemon_rexec::bb_daemonize_or_rexec(0i32);
  } else {
    crate::libbb::vfork_daemon_rexec::bb_sanitize_stdio();
  }
  if opt & 4i32 == 0 {
    /* LOG_NDELAY: connect to syslog daemon NOW.
     * Otherwise, we may open syslog socket
     * in vforked child, making opened fds and syslog()
     * internal state inconsistent.
     * This was observed to leak file descriptors. */
    openlog(applet_name, 0x1i32 | 0x8i32, 3i32 << 3i32);
    logmode = LOGMODE_SYSLOG as libc::c_int as smallint
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).real_uid == 0 as libc::c_uint {
    /* run by root, ensure groups vector gets trashed */
    let mut gid: gid_t = getgid();
    setgroups(1i32 as size_t, &mut gid);
  }
  crate::libbb::pidfile::write_pidfile_std_path_and_ext(
    b"inetd\x00" as *const u8 as *const libc::c_char,
  );
  /* never fails under Linux (except if you pass it bad arguments) */
  getrlimit(
    RLIMIT_NOFILE,
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).rlim_ofile,
  );
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).rlim_ofile_cur =
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .rlim_ofile
      .rlim_cur;
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).rlim_ofile_cur as libc::c_ulonglong
    == 0xffffffffffffffffu64
  {
    /* ! */
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).rlim_ofile_cur = 64i32 as rlim_t
  }
  memset(
    &mut sa as *mut sigaction as *mut libc::c_void,
    0,
    ::std::mem::size_of::<sigaction>() as libc::c_ulong,
  );
  /*sigemptyset(&sa.sa_mask); - memset did it */
  sigaddset(&mut sa.sa_mask, 14i32);
  sigaddset(&mut sa.sa_mask, 17i32);
  sigaddset(&mut sa.sa_mask, 1i32);
  //FIXME: explain why no SA_RESTART
  //FIXME: retry_network_setup is unsafe to run in signal handler (many reasons)!
  sa.__sigaction_handler.sa_handler =
    Some(retry_network_setup as unsafe extern "C" fn(_: libc::c_int) -> ());
  crate::libbb::signals::sigaction_set(14i32, &mut sa);
  //FIXME: reread_config_file is unsafe to run in signal handler(many reasons)!
  sa.__sigaction_handler.sa_handler =
    Some(reread_config_file as unsafe extern "C" fn(_: libc::c_int) -> ());
  crate::libbb::signals::sigaction_set(1i32, &mut sa);
  //FIXME: reap_child is unsafe to run in signal handler (uses stdio)!
  sa.__sigaction_handler.sa_handler =
    Some(reap_child as unsafe extern "C" fn(_: libc::c_int) -> ());
  crate::libbb::signals::sigaction_set(17i32, &mut sa);
  //FIXME: clean_up_and_exit is unsafe to run in signal handler (uses stdio)!
  sa.__sigaction_handler.sa_handler =
    Some(clean_up_and_exit as unsafe extern "C" fn(_: libc::c_int) -> ()); /* load config from file */
  crate::libbb::signals::sigaction_set(15i32, &mut sa);
  sa.__sigaction_handler.sa_handler =
    Some(clean_up_and_exit as unsafe extern "C" fn(_: libc::c_int) -> ());
  crate::libbb::signals::sigaction_set(2i32, &mut sa);
  sa.__sigaction_handler.sa_handler =
    ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(1i32 as libc::intptr_t);
  sigaction(13i32, &mut sa, &mut saved_pipe_handler);
  reread_config_file(1i32);
  loop {
    let mut ready_fd_cnt: libc::c_int = 0;
    let mut ctrl: libc::c_int = 0;
    let mut accepted_fd: libc::c_int = 0;
    let mut new_udp_fd: libc::c_int = 0;
    let mut readable: fd_set = fd_set { fds_bits: [0; 16] };
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).maxsock < 0 {
      recalculate_maxsock();
    }
    /* for (sep = servtab...) */
    readable = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).allsock; /* struct copy */
    /* if there are no fds to wait on, we will block
     * until signal wakes us up (maxsock == 0, but readable
     * never contains fds 0 and 1...) */
    ready_fd_cnt = select(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).maxsock + 1i32,
      &mut readable,
      0 as *mut fd_set,
      0 as *mut fd_set,
      0 as *mut timeval,
    );
    if ready_fd_cnt < 0 {
      if *bb_errno != 4i32 {
        crate::libbb::perror_msg::bb_simple_perror_msg(
          b"select\x00" as *const u8 as *const libc::c_char,
        );
        sleep(1i32 as libc::c_uint);
      }
    } else {
      sep = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).serv_list;
      while ready_fd_cnt != 0 && !sep.is_null() {
        if !((*sep).se_fd == -1i32
          || !(readable.fds_bits[((*sep).se_fd
            / (8i32 * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            & (1u64
              << (*sep).se_fd
                % (8i32 * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
              as __fd_mask
            != 0))
        {
          ready_fd_cnt -= 1;
          ctrl = (*sep).se_fd;
          accepted_fd = -1i32;
          new_udp_fd = -1i32;
          if (*sep).se_wait == 0 {
            if (*sep).se_socktype as libc::c_int == SOCK_STREAM as libc::c_int {
              accepted_fd = accept(
                (*sep).se_fd,
                __SOCKADDR_ARG {
                  __sockaddr__: std::ptr::null_mut(),
                },
                0 as *mut socklen_t,
              );
              ctrl = accepted_fd;
              if ctrl < 0 {
                if *bb_errno != 4i32 {
                  crate::libbb::perror_msg::bb_perror_msg(
                    b"accept (for %s)\x00" as *const u8 as *const libc::c_char,
                    (*sep).se_service,
                  );
                }
                current_block = 168769493162332264;
              } else {
                current_block = 16029476503615101993;
              }
            } else {
              current_block = 16029476503615101993;
            }
            match current_block {
              168769493162332264 => {}
              _ =>
              /* "nowait" udp */
              {
                if (*sep).se_socktype as libc::c_int == SOCK_DGRAM as libc::c_int
                  && (*sep).se_family as libc::c_int != 1i32
                {
                  /* How udp "nowait" works:
                   * child peeks at (received and buffered by kernel) UDP packet,
                   * performs connect() on the socket so that it is linked only
                   * to this peer. But this also affects parent, because descriptors
                   * are shared after fork() a-la dup(). When parent performs
                   * select(), it will see this descriptor connected to the peer (!)
                   * and still readable, will act on it and mess things up
                   * (can create many copies of same child, etc).
                   * Parent must create and use new socket instead. */
                  new_udp_fd = socket(
                    (*sep).se_family as libc::c_int,
                    SOCK_DGRAM as libc::c_int,
                    0,
                  );
                  if new_udp_fd < 0 {
                    current_block = 3055274896829046098;
                  } else {
                    crate::libbb::xconnect::setsockopt_reuseaddr(new_udp_fd);
                    /* TODO: better do bind after fork in parent,
                     * so that we don't have two wildcard bound sockets
                     * even for a brief moment? */
                    if bind(
                      new_udp_fd,
                      __CONST_SOCKADDR_ARG {
                        __sockaddr__: &mut (*(*sep).se_lsa).u.sa,
                      },
                      (*(*sep).se_lsa).len,
                    ) < 0
                    {
                      close(new_udp_fd);
                      current_block = 3055274896829046098;
                    } else {
                      current_block = 15855550149339537395;
                    }
                  }
                  match current_block {
                    15855550149339537395 => {}
                    _ =>
                    /* error: eat packet, forget about it */
                    {
                      recv(
                        (*sep).se_fd,
                        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                          .line
                          .as_mut_ptr() as *mut libc::c_void,
                        LINE_SIZE as libc::c_int as size_t,
                        MSG_DONTWAIT as libc::c_int,
                      );
                      current_block = 168769493162332264;
                    }
                  }
                } else {
                  current_block = 15855550149339537395;
                }
              }
            }
          } else {
            current_block = 15855550149339537395;
          }
          match current_block {
            168769493162332264 => {}
            _ => {
              block_CHLD_HUP_ALRM(&mut omask);
              pid = 0;
              /* do we need to fork? */
              if (*sep).se_builtin.is_null()
                || (*sep).se_socktype as libc::c_int == SOCK_STREAM as libc::c_int
                  && (*(*sep).se_builtin).bi_fork as libc::c_int != 0
              {
                if (*sep).se_max != 0 as libc::c_uint {
                  (*sep).se_count = (*sep).se_count.wrapping_add(1);
                  if (*sep).se_count == 1i32 as libc::c_uint {
                    (*sep).se_time = crate::libbb::time::monotonic_sec();
                    current_block = 5706507068631705000;
                  } else if (*sep).se_count >= (*sep).se_max {
                    let mut now: libc::c_uint = crate::libbb::time::monotonic_sec();
                    /* did we accumulate se_max connects too quickly? */
                    if now.wrapping_sub((*sep).se_time) <= 60i32 as libc::c_uint {
                      crate::libbb::verror_msg::bb_error_msg(
                        b"%s/%s: too many connections, pausing\x00" as *const u8
                          as *const libc::c_char,
                        (*sep).se_service,
                        (*sep).se_proto,
                      );
                      remove_fd_from_set((*sep).se_fd);
                      close((*sep).se_fd);
                      (*sep).se_fd = -1i32;
                      (*sep).se_count = 0 as libc::c_uint;
                      /* -> check next fd in fd set */
                      rearm_alarm(); /* will revive it in RETRYTIME sec */
                      restore_sigmask(&mut omask);
                      maybe_close(new_udp_fd);
                      maybe_close(accepted_fd);
                      current_block = 168769493162332264;
                    } else {
                      (*sep).se_count = 0 as libc::c_uint;
                      current_block = 5706507068631705000;
                    }
                  } else {
                    current_block = 5706507068631705000;
                  }
                } else {
                  current_block = 5706507068631705000;
                }
                match current_block {
                  168769493162332264 => {}
                  _ => {
                    /* -1: "we did fork and we are child" */
                    /* on NOMMU, streamed chargen
                     * builtin wouldn't work, but it is
                     * not allowed on NOMMU (ifdefed out) */
                    if 1i32 != 0 && !(*sep).se_builtin.is_null() {
                      pid = fork()
                    } else {
                      pid = vfork()
                    }
                    if pid < 0 {
                      /* fork error */
                      crate::libbb::perror_msg::bb_simple_perror_msg(
                        (b"vfork\x00" as *const u8 as *const libc::c_char).offset(1),
                      );
                      sleep(1i32 as libc::c_uint);
                      restore_sigmask(&mut omask);
                      maybe_close(new_udp_fd);
                      maybe_close(accepted_fd);
                      current_block = 168769493162332264;
                    /* -> check next fd in fd set */
                    } else {
                      if pid == 0 {
                        pid -= 1
                      }
                      current_block = 16972322153429435017;
                    }
                  }
                }
              } else {
                current_block = 16972322153429435017;
              }
              match current_block {
                168769493162332264 => {}
                _ =>
                /* if pid == 0 here, we didn't fork */
                {
                  if pid > 0 {
                    /* parent */
                    if (*sep).se_wait != 0 {
                      /* wait: we passed socket to child,
                       * will wait for child to terminate */
                      (*sep).se_wait = pid;
                      remove_fd_from_set((*sep).se_fd);
                    }
                    if new_udp_fd >= 0 {
                      /* -> check next fd in fd set */
                      /* udp nowait: child connected the socket,
                       * we created and will use new, unconnected one */
                      crate::libbb::xfuncs_printf::xmove_fd(new_udp_fd, (*sep).se_fd);
                    }
                    restore_sigmask(&mut omask);
                    maybe_close(accepted_fd);
                  } else if !(*sep).se_builtin.is_null() {
                    if pid != 0 {
                      /* we are either child or didn't fork at all */
                      /* "pid" is -1: we did fork */
                      close((*sep).se_fd);
                      logmode = LOGMODE_NONE as libc::c_int as smallint /* listening socket */
                      /* make xwrite etc silent */
                    }
                    restore_sigmask(&mut omask);
                    if (*sep).se_socktype as libc::c_int == SOCK_STREAM as libc::c_int {
                      (*(*sep).se_builtin)
                        .bi_stream_fn
                        .expect("non-null function pointer")(ctrl, sep);
                    } else {
                      (*(*sep).se_builtin)
                        .bi_dgram_fn
                        .expect("non-null function pointer")(ctrl, sep);
                    }
                    if pid != 0 {
                      /* -> check next fd in fd set */
                      /* we did fork */
                      _exit(1i32);
                    }
                    maybe_close(accepted_fd);
                  } else {
                    /* child */
                    setsid();
                    /* "nowait" udp */
                    if new_udp_fd >= 0 {
                      let mut lsa: *mut len_and_sockaddr = std::ptr::null_mut();
                      let mut r: libc::c_int = 0;
                      close(new_udp_fd);
                      lsa = xzalloc_lsa((*sep).se_family as libc::c_int);
                      /* peek at the packet and remember peer addr */
                      r = recvfrom(
                        ctrl,
                        0 as *mut libc::c_void,
                        0 as size_t,
                        MSG_PEEK as libc::c_int | MSG_DONTWAIT as libc::c_int,
                        __SOCKADDR_ARG {
                          __sockaddr__: &mut (*lsa).u.sa as *mut sockaddr,
                        },
                        &mut (*lsa).len,
                      ) as libc::c_int;
                      if r < 0 {
                        current_block = 8654587244565057016;
                      } else {
                        /* make this socket "connected" to peer addr:
                         * only packets from this peer will be recv'ed,
                         * and bare write()/send() will work on it */
                        connect(
                          ctrl,
                          __CONST_SOCKADDR_ARG {
                            __sockaddr__: &mut (*lsa).u.sa,
                          },
                          (*lsa).len,
                        );
                        free(lsa as *mut libc::c_void);
                        current_block = 12099607619007264150;
                      }
                    } else {
                      current_block = 12099607619007264150;
                    }
                    match current_block {
                      12099607619007264150 => {
                        /* prepare env and exec program */
                        pwd = bb_internal_getpwnam((*sep).se_user);
                        if pwd.is_null() {
                          crate::libbb::verror_msg::bb_error_msg(
                            b"%s: no such %s\x00" as *const u8 as *const libc::c_char,
                            (*sep).se_user,
                            b"user\x00" as *const u8 as *const libc::c_char,
                          );
                        } else if !(*sep).se_group.is_null() && {
                          grp = crate::libpwdgrp::pwd_grp::bb_internal_getgrnam((*sep).se_group);
                          grp.is_null()
                        } {
                          crate::libbb::verror_msg::bb_error_msg(
                            b"%s: no such %s\x00" as *const u8 as *const libc::c_char,
                            (*sep).se_group,
                            b"group\x00" as *const u8 as *const libc::c_char,
                          );
                        } else if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).real_uid
                          != 0 as libc::c_uint
                          && (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).real_uid
                            != (*pwd).pw_uid
                        {
                          /* a user running private inetd */
                          crate::libbb::verror_msg::bb_simple_error_msg(
                            b"non-root must run services as himself\x00" as *const u8
                              as *const libc::c_char,
                          );
                        } else {
                          if (*pwd).pw_uid
                            != (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).real_uid
                          {
                            if !(*sep).se_group.is_null() {
                              (*pwd).pw_gid = (*grp).gr_gid
                            }
                            /* initgroups, setgid, setuid: */
                            crate::libbb::change_identity::change_identity(pwd);
                          } else if !(*sep).se_group.is_null() {
                            crate::libbb::xfuncs_printf::xsetgid((*grp).gr_gid);
                            setgroups(1i32 as size_t, &mut (*grp).gr_gid);
                          }
                          if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                            .rlim_ofile
                            .rlim_cur
                            != (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).rlim_ofile_cur
                          {
                            if setrlimit(
                              RLIMIT_NOFILE,
                              &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).rlim_ofile,
                            ) < 0
                            {
                              crate::libbb::perror_msg::bb_simple_perror_msg(
                                b"setrlimit\x00" as *const u8 as *const libc::c_char,
                              );
                            }
                          }
                          /* closelog(); - WRONG. we are after vfork,
                           * this may confuse syslog() internal state.
                           * Let's hope libc sets syslog fd to CLOEXEC...
                           */
                          crate::libbb::xfuncs_printf::xmove_fd(ctrl, 0);
                          crate::libbb::xfuncs_printf::xdup2(0i32, 1i32);
                          /* manpages of inetd I managed to find either say
                           * that stderr is also redirected to the network,
                           * or do not talk about redirection at all (!) */
                          if (*sep).se_wait == 0 {
                            /* only for usual "tcp nowait" */
                            crate::libbb::xfuncs_printf::xdup2(0i32, 2i32);
                          }
                          /* NB: among others, this loop closes listening sockets
                           * for nowait stream children */
                          sep2 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).serv_list;
                          while !sep2.is_null() {
                            if (*sep2).se_fd != ctrl {
                              maybe_close((*sep2).se_fd);
                            }
                            sep2 = (*sep2).se_next
                          }
                          crate::libbb::signals::sigaction_set(13i32, &mut saved_pipe_handler);
                          restore_sigmask(&mut omask);
                          execvp(
                            (*sep).se_program,
                            (*sep).se_argv.as_mut_ptr() as *const *mut libc::c_char,
                          );
                          crate::libbb::perror_msg::bb_perror_msg(
                            b"can\'t execute \'%s\'\x00" as *const u8 as *const libc::c_char,
                            (*sep).se_program,
                          );
                        }
                      }
                      _ => {}
                    }
                    /* eat packet in udp case */
                    if (*sep).se_socktype as libc::c_int != SOCK_STREAM as libc::c_int {
                      recv(
                        0,
                        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                          .line
                          .as_mut_ptr() as *mut libc::c_void,
                        LINE_SIZE as libc::c_int as size_t,
                        MSG_DONTWAIT as libc::c_int,
                      );
                    }
                    _exit(1i32);
                  }
                }
              }
            }
          }
        }
        sep = (*sep).se_next
      }
    }
  }
  /* for (;;) */
}
/*
 * Internet services provided internally by inetd:
 */
/* Echo service -- echo data back. */
/* ARGSUSED */
unsafe extern "C" fn echo_stream(mut s: libc::c_int, mut _sep: *mut servtab_t) {
  loop {
    let mut sz: ssize_t = crate::libbb::read::safe_read(
      s,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .line
        .as_mut_ptr() as *mut libc::c_void,
      LINE_SIZE as libc::c_int as size_t,
    ); /* for jumbo sized packets! :) */
    if sz <= 0 {
      break; /* too big for stack */
    }
    crate::libbb::xfuncs_printf::xwrite(
      s,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .line
        .as_mut_ptr() as *const libc::c_void,
      sz as size_t,
    );
  }
}
unsafe extern "C" fn echo_dg(mut s: libc::c_int, mut sep: *mut servtab_t) {
  let mut buf: *mut libc::c_char = xmalloc(BUFSIZE as libc::c_int as size_t) as *mut libc::c_char;
  let mut sz: libc::c_int = 0;
  let mut fresh21 = ::std::vec::from_elem(
    0,
    (LSA_LEN_SIZE as libc::c_int as libc::c_uint).wrapping_add((*(*sep).se_lsa).len)
      as libc::c_ulong as usize,
  );
  let mut lsa: *mut len_and_sockaddr = fresh21.as_mut_ptr() as *mut len_and_sockaddr;
  (*lsa).len = (*(*sep).se_lsa).len;
  /* dgram builtins are non-forking - DONT BLOCK! */
  sz = recvfrom(
    s,
    buf as *mut libc::c_void,
    BUFSIZE as libc::c_int as size_t,
    MSG_DONTWAIT as libc::c_int,
    __SOCKADDR_ARG {
      __sockaddr__: &mut (*lsa).u.sa as *mut sockaddr,
    },
    &mut (*lsa).len,
  ) as libc::c_int;
  if sz > 0 {
    sendto(
      s,
      buf as *const libc::c_void,
      sz as size_t,
      0,
      __CONST_SOCKADDR_ARG {
        __sockaddr__: &mut (*lsa).u.sa,
      },
      (*lsa).len,
    );
  }
  free(buf as *mut libc::c_void);
}
/* FEATURE_INETD_SUPPORT_BUILTIN_ECHO */
/* Discard service -- ignore data. */
/* ARGSUSED */
unsafe extern "C" fn discard_stream(mut s: libc::c_int, mut _sep: *mut servtab_t) {
  while crate::libbb::read::safe_read(
    s,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .line
      .as_mut_ptr() as *mut libc::c_void,
    LINE_SIZE as libc::c_int as size_t,
  ) > 0
  {}
}
/* ARGSUSED */
unsafe extern "C" fn discard_dg(mut s: libc::c_int, mut _sep: *mut servtab_t) {
  /* dgram builtins are non-forking - DONT BLOCK! */
  recv(
    s,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .line
      .as_mut_ptr() as *mut libc::c_void,
    LINE_SIZE as libc::c_int as size_t,
    MSG_DONTWAIT as libc::c_int,
  );
}
unsafe extern "C" fn init_ring() {
  let mut i: libc::c_int = 0;
  let ref mut fresh22 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).end_ring;
  *fresh22 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .ring
    .as_mut_ptr();
  i = ' ' as i32;
  while i < 127i32 {
    let ref mut fresh23 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).end_ring;
    let fresh24 = *fresh23;
    *fresh23 = (*fresh23).offset(1);
    *fresh24 = i as libc::c_char;
    i += 1
  }
}
/* Character generator. MMU arches only. */
/* ARGSUSED */
unsafe extern "C" fn chargen_stream(mut s: libc::c_int, mut _sep: *mut servtab_t) {
  let mut rs: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut len: libc::c_int = 0;
  let mut text: [libc::c_char; 74] = [0; 74];
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .end_ring
    .is_null()
  {
    init_ring();
    rs = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .ring
      .as_mut_ptr()
  }
  text[72] = '\r' as i32 as libc::c_char;
  text[(72i32 + 1i32) as usize] = '\n' as i32 as libc::c_char;
  rs = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .ring
    .as_mut_ptr();
  loop {
    len = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .end_ring
      .wrapping_offset_from(rs) as libc::c_long as libc::c_int;
    if len >= 72i32 {
      memmove(
        text.as_mut_ptr() as *mut libc::c_void,
        rs as *const libc::c_void,
        72i32 as libc::c_ulong,
      );
    } else {
      memmove(
        text.as_mut_ptr() as *mut libc::c_void,
        rs as *const libc::c_void,
        len as libc::c_ulong,
      );
      memmove(
        text.as_mut_ptr().offset(len as isize) as *mut libc::c_void,
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
          .ring
          .as_mut_ptr() as *const libc::c_void,
        (72i32 - len) as libc::c_ulong,
      );
    }
    rs = rs.offset(1);
    if rs == (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).end_ring {
      rs = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .ring
        .as_mut_ptr()
    }
    crate::libbb::xfuncs_printf::xwrite(
      s,
      text.as_mut_ptr() as *const libc::c_void,
      ::std::mem::size_of::<[libc::c_char; 74]>() as libc::c_ulong,
    );
  }
}
/* ARGSUSED */
unsafe extern "C" fn chargen_dg(mut s: libc::c_int, mut sep: *mut servtab_t) {
  let mut len: libc::c_int = 0;
  let mut text: [libc::c_char; 74] = [0; 74];
  let mut fresh25 = ::std::vec::from_elem(
    0,
    (LSA_LEN_SIZE as libc::c_int as libc::c_uint).wrapping_add((*(*sep).se_lsa).len)
      as libc::c_ulong as usize,
  );
  let mut lsa: *mut len_and_sockaddr = fresh25.as_mut_ptr() as *mut len_and_sockaddr;
  /* Eat UDP packet which started it all */
  /* dgram builtins are non-forking - DONT BLOCK! */
  (*lsa).len = (*(*sep).se_lsa).len;
  if recvfrom(
    s,
    text.as_mut_ptr() as *mut libc::c_void,
    ::std::mem::size_of::<[libc::c_char; 74]>() as libc::c_ulong,
    MSG_DONTWAIT as libc::c_int,
    __SOCKADDR_ARG {
      __sockaddr__: &mut (*lsa).u.sa as *mut sockaddr,
    },
    &mut (*lsa).len,
  ) < 0
  {
    return;
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .end_ring
    .is_null()
  {
    init_ring();
    let ref mut fresh26 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).ring_pos;
    *fresh26 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .ring
      .as_mut_ptr()
  }
  len = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .end_ring
    .wrapping_offset_from((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).ring_pos)
    as libc::c_long as libc::c_int;
  if len >= 72i32 {
    memmove(
      text.as_mut_ptr() as *mut libc::c_void,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).ring_pos as *const libc::c_void,
      72i32 as libc::c_ulong,
    );
  } else {
    memmove(
      text.as_mut_ptr() as *mut libc::c_void,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).ring_pos as *const libc::c_void,
      len as libc::c_ulong,
    );
    memmove(
      text.as_mut_ptr().offset(len as isize) as *mut libc::c_void,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .ring
        .as_mut_ptr() as *const libc::c_void,
      (72i32 - len) as libc::c_ulong,
    );
  }
  let ref mut fresh27 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).ring_pos;
  *fresh27 = (*fresh27).offset(1);
  if *fresh27 == (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).end_ring {
    let ref mut fresh28 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).ring_pos;
    *fresh28 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .ring
      .as_mut_ptr()
  }
  text[72] = '\r' as i32 as libc::c_char;
  text[(72i32 + 1i32) as usize] = '\n' as i32 as libc::c_char;
  sendto(
    s,
    text.as_mut_ptr() as *const libc::c_void,
    ::std::mem::size_of::<[libc::c_char; 74]>() as libc::c_ulong,
    0,
    __CONST_SOCKADDR_ARG {
      __sockaddr__: &mut (*lsa).u.sa,
    },
    (*lsa).len,
  );
}
/* FEATURE_INETD_SUPPORT_BUILTIN_CHARGEN */
/*
 * Return a machine readable date and time, in the form of the
 * number of seconds since midnight, Jan 1, 1900.  Since gettimeofday
 * returns the number of seconds since midnight, Jan 1, 1970,
 * we must add 2208988800 seconds to this figure to make up for
 * some seventy years Bell Labs was asleep.
 */
unsafe extern "C" fn machtime() -> u32 {
  let mut tv: timeval = timeval {
    tv_sec: 0,
    tv_usec: 0,
  };
  gettimeofday(&mut tv, 0 as *mut timezone);
  return {
    let mut __v: libc::c_uint = 0;
    let mut __x: libc::c_uint = (tv.tv_sec + 2208988800u32 as libc::c_long) as u32;
    if false {
      __v = (__x & 0xff000000u32) >> 24i32
        | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
        | (__x & 0xff00i32 as libc::c_uint) << 8i32
        | (__x & 0xffi32 as libc::c_uint) << 24i32
    } else {
      let fresh29 = &mut __v;
      let fresh30;
      let fresh31 = __x;
      asm!("bswap $0" : "=r" (fresh30) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh29, fresh31)) :);
      c2rust_asm_casts::AsmCast::cast_out(fresh29, fresh31, fresh30);
    }
    __v
  };
}
/* ARGSUSED */
unsafe extern "C" fn machtime_stream(mut s: libc::c_int, mut _sep: *mut servtab_t) {
  let mut result: u32 = 0;
  result = machtime();
  crate::libbb::full_write::full_write(
    s,
    &mut result as *mut u32 as *const libc::c_void,
    ::std::mem::size_of::<u32>() as libc::c_ulong,
  );
}
unsafe extern "C" fn machtime_dg(mut s: libc::c_int, mut sep: *mut servtab_t) {
  let mut result: u32 = 0;
  let mut fresh32 = ::std::vec::from_elem(
    0,
    (LSA_LEN_SIZE as libc::c_int as libc::c_uint).wrapping_add((*(*sep).se_lsa).len)
      as libc::c_ulong as usize,
  );
  let mut lsa: *mut len_and_sockaddr = fresh32.as_mut_ptr() as *mut len_and_sockaddr;
  (*lsa).len = (*(*sep).se_lsa).len;
  if recvfrom(
    s,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .line
      .as_mut_ptr() as *mut libc::c_void,
    LINE_SIZE as libc::c_int as size_t,
    MSG_DONTWAIT as libc::c_int,
    __SOCKADDR_ARG {
      __sockaddr__: &mut (*lsa).u.sa as *mut sockaddr,
    },
    &mut (*lsa).len,
  ) < 0
  {
    return;
  }
  result = machtime();
  sendto(
    s,
    &mut result as *mut u32 as *const libc::c_void,
    ::std::mem::size_of::<u32>() as libc::c_ulong,
    0,
    __CONST_SOCKADDR_ARG {
      __sockaddr__: &mut (*lsa).u.sa,
    },
    (*lsa).len,
  );
}
/* FEATURE_INETD_SUPPORT_BUILTIN_TIME */
/* Return human-readable time of day */
/* ARGSUSED */
unsafe extern "C" fn daytime_stream(mut s: libc::c_int, mut _sep: *mut servtab_t) {
  let mut t: time_t = 0;
  time(&mut t);
  dprintf(
    s,
    b"%.24s\r\n\x00" as *const u8 as *const libc::c_char,
    ctime(&mut t),
  );
}
unsafe extern "C" fn daytime_dg(mut s: libc::c_int, mut sep: *mut servtab_t) {
  let mut t: time_t = 0;
  let mut fresh33 = ::std::vec::from_elem(
    0,
    (LSA_LEN_SIZE as libc::c_int as libc::c_uint).wrapping_add((*(*sep).se_lsa).len)
      as libc::c_ulong as usize,
  );
  let mut lsa: *mut len_and_sockaddr = fresh33.as_mut_ptr() as *mut len_and_sockaddr;
  (*lsa).len = (*(*sep).se_lsa).len;
  if recvfrom(
    s,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .line
      .as_mut_ptr() as *mut libc::c_void,
    LINE_SIZE as libc::c_int as size_t,
    MSG_DONTWAIT as libc::c_int,
    __SOCKADDR_ARG {
      __sockaddr__: &mut (*lsa).u.sa as *mut sockaddr,
    },
    &mut (*lsa).len,
  ) < 0
  {
    return;
  }
  t = time(0 as *mut time_t);
  sprintf(
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .line
      .as_mut_ptr(),
    b"%.24s\r\n\x00" as *const u8 as *const libc::c_char,
    ctime(&mut t),
  );
  sendto(
    s,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .line
      .as_mut_ptr() as *const libc::c_void,
    strlen(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .line
        .as_mut_ptr(),
    ),
    0,
    __CONST_SOCKADDR_ARG {
      __sockaddr__: &mut (*lsa).u.sa,
    },
    (*lsa).len,
  );
}
/* FEATURE_INETD_SUPPORT_BUILTIN_DAYTIME */
