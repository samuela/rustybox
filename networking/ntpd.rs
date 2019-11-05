use c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use c2rust_bitfields;
use c2rust_bitfields::BitfieldStruct;
use libc;
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
  /* Some useful definitions */
  /* Macros for min/max.  */
  /* buffer allocation schemes */
  /* glibc uses __errno_location() to get a ptr to errno */
  /* We can just memorize it once - no multithreading in busybox :) */
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn bb_signals(sigs: libc::c_int, f: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>);
  #[no_mangle]
  fn kill_myself_with_sig(sig: libc::c_int) -> !;
  /* Standard handler which just records signo */
  #[no_mangle]
  static mut bb_got_signal: smallint;
  #[no_mangle]
  fn record_signo(signo: libc::c_int);
  #[no_mangle]
  fn strftime_YYYYMMDDHHMMSS(
    buf: *mut libc::c_char,
    len: libc::c_uint,
    tp: *mut time_t,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn xbind(sockfd: libc::c_int, my_addr: *mut sockaddr, addrlen: socklen_t);
  #[no_mangle]
  fn setsockopt_int(
    fd: libc::c_int,
    level: libc::c_int,
    optname: libc::c_int,
    optval: libc::c_int,
  ) -> libc::c_int;
  #[no_mangle]
  fn setsockopt_bindtodevice(fd: libc::c_int, iface: *const libc::c_char) -> libc::c_int;
  /* Create stream socket, and allocate suitable lsa.
   * (lsa of correct size and lsa->sa.sa_family (AF_INET/AF_INET6))
   * af == AF_UNSPEC will result in trying to create IPv6 socket,
   * and if kernel doesn't support it, fall back to IPv4.
   * This is useful if you plan to bind to resulting local lsa.
   */
  #[no_mangle]
  fn xsocket_type(
    lsap: *mut *mut len_and_sockaddr,
    af: libc::c_int,
    sock_type: libc::c_int,
  ) -> libc::c_int;
  #[no_mangle]
  fn create_and_bind_dgram_or_die(bindaddr: *const libc::c_char, port: libc::c_int) -> libc::c_int;
  /* Get local address of bound or accepted socket */
  #[no_mangle]
  fn get_sock_lsa(fd: libc::c_int) -> *mut len_and_sockaddr;
  /* Return malloc'ed len_and_sockaddr with socket address of host:port
   * Currently will return IPv4 or IPv6 sockaddrs only
   * (depending on host), but in theory nothing prevents e.g.
   * UNIX socket address being returned, IPX sockaddr etc...
   * On error does bb_error_msg and returns NULL */
  #[no_mangle]
  fn host2sockaddr(host: *const libc::c_char, port: libc::c_int) -> *mut len_and_sockaddr;
  #[no_mangle]
  fn xmalloc_sockaddr2dotted_noport(sa: *const sockaddr) -> *mut libc::c_char;
  #[no_mangle]
  fn socket_want_pktinfo(fd: libc::c_int);
  #[no_mangle]
  fn send_to_from(
    fd: libc::c_int,
    buf: *mut libc::c_void,
    len: size_t,
    flags: libc::c_int,
    to: *const sockaddr,
    from: *const sockaddr,
    tolen: socklen_t,
  ) -> ssize_t;
  #[no_mangle]
  fn recv_from_to(
    fd: libc::c_int,
    buf: *mut libc::c_void,
    len: size_t,
    flags: libc::c_int,
    from: *mut sockaddr,
    to: *mut sockaddr,
    sa_size: socklen_t,
  ) -> ssize_t;
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  /* Reverse */
  #[no_mangle]
  fn hex2bin(
    dst: *mut libc::c_char,
    src: *const libc::c_char,
    count: libc::c_int,
  ) -> *mut libc::c_char;
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
  fn rand() -> libc::c_int;
  #[no_mangle]
  fn srand(__seed: libc::c_uint);
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  fn exit(_: libc::c_int) -> !;
  #[no_mangle]
  fn putenv(__string: *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn unsetenv(__name: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t, __compar: __compar_fn_t);
  #[no_mangle]
  fn abs(_: libc::c_int) -> libc::c_int;
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
  fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
  #[no_mangle]
  fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn strnlen(__string: *const libc::c_char, __maxlen: size_t) -> size_t;
  #[no_mangle]
  fn strncasecmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
  #[no_mangle]
  fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn setpriority(__which: __priority_which_t, __who: id_t, __prio: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn gettimeofday(__tv: *mut timeval, __tz: __timezone_ptr_t) -> libc::c_int;
  #[no_mangle]
  fn settimeofday(__tv: *const timeval, __tz: *const timezone) -> libc::c_int;
  #[no_mangle]
  fn getuid() -> __uid_t;
  #[no_mangle]
  fn getpid() -> __pid_t;
  #[no_mangle]
  fn alarm(__seconds: libc::c_uint) -> libc::c_uint;
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xatou_range(str: *const libc::c_char, l: libc::c_uint, u: libc::c_uint) -> libc::c_uint;
  #[no_mangle]
  fn spawn(argv: *mut *mut libc::c_char) -> pid_t;
  #[no_mangle]
  fn bb_daemonize_or_rexec(flags: libc::c_int);
  #[no_mangle]
  static mut option_mask32: uint32_t;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;
  #[no_mangle]
  fn llist_add_to(old_head: *mut *mut llist_t, data: *mut libc::c_void);
  #[no_mangle]
  fn llist_pop(elm: *mut *mut llist_t) -> *mut libc::c_void;
  #[no_mangle]
  fn write_pidfile_std_path_and_ext(path: *const libc::c_char);
  #[no_mangle]
  fn remove_pidfile_std_path_and_ext(path: *const libc::c_char);
  #[no_mangle]
  static mut logmode: smallint;
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
  fn bb_simple_perror_msg(s: *const libc::c_char);
  #[no_mangle]
  fn bb_perror_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_info_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn config_open(filename: *const libc::c_char) -> *mut parser_t;
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
  fn md5_begin(ctx: *mut md5_ctx_t);
  #[no_mangle]
  fn md5_hash(ctx: *mut md5_ctx_t, buffer: *const libc::c_void, len: size_t);
  #[no_mangle]
  fn md5_end(ctx: *mut md5_ctx_t, resbuf: *mut libc::c_void) -> libc::c_uint;
  #[no_mangle]
  fn sha1_begin(ctx: *mut sha1_ctx_t);
  #[no_mangle]
  fn sha1_end(ctx: *mut sha1_ctx_t, resbuf: *mut libc::c_void) -> libc::c_uint;
  #[no_mangle]
  static bb_msg_you_must_be_root: [libc::c_char; 0];
  /* '*const' ptr makes gcc optimize code much better.
   * Magic prevents ptr_to_globals from going into rodata.
   * If you want to assign a value, use SET_PTR_TO_GLOBALS(x) */
  #[no_mangle]
  static ptr_to_globals: *mut globals;
  #[no_mangle]
  fn sqrt(_: libc::c_double) -> libc::c_double;
  #[no_mangle]
  fn fabs(_: libc::c_double) -> libc::c_double;
  #[no_mangle]
  fn adjtimex(__ntx: *mut timex) -> libc::c_int;
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __id_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type int8_t = __int8_t;
pub type int32_t = __int32_t;
use crate::librb::uint8_t;
pub type uint16_t = __uint16_t;
use crate::librb::uint32_t;
use crate::librb::uint64_t;
use crate::librb::smallint;
/* vi: set sw=4 ts=4: */
/*
 * Copyright 2006, Bernhard Reutner-Fischer
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/* Convenience macros to test the version of gcc. */
/* __restrict is known in EGCS 1.2 and above. */
/* "The malloc attribute is used to tell the compiler that a function
 * may be treated as if any non-NULL pointer it returns cannot alias
 * any other pointer valid when the function returns. This will often
 * improve optimization. Standard functions with this property include
 * malloc and calloc. realloc-like functions have this property as long
 * as the old pointer is never referred to (including comparing it
 * to the new pointer) after the function returns a non-NULL value."
 */
/* __NO_INLINE__: some gcc's do not honor inlining! :( */
/* I've seen a toolchain where I needed __noinline__ instead of noinline */
/* used by unit test machinery to run registration functions before calling main() */
/* -fwhole-program makes all symbols local. The attribute externally_visible
 * forces a symbol global.  */
//__attribute__ ((__externally_visible__))
/* At 4.4 gcc become much more anal about this, need to use "aliased" types */
/* We use __extension__ in some places to suppress -pedantic warnings
 * about GCC extensions.  This feature didn't work properly before
 * gcc 2.8.  */
/* FAST_FUNC is a qualifier which (possibly) makes function call faster
 * and/or smaller by using modified ABI. It is usually only needed
 * on non-static, busybox internal functions. Recent versions of gcc
 * optimize statics automatically. FAST_FUNC on static is required
 * only if you need to match a function pointer's type.
 * FAST_FUNC may not work well with -flto so allow user to disable this.
 * (-DFAST_FUNC= )
 */
/* Make all declarations hidden (-fvisibility flag only affects definitions) */
/* (don't include system headers after this until corresponding pop!) */
/* gcc-2.95 had no va_copy but only __va_copy. */
/* ---- Endian Detection ------------------------------------ */
/* SWAP_LEnn means "convert CPU<->little_endian by swapping bytes" */
/* ---- Unaligned access ------------------------------------ */
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
pub type smalluint = libc::c_uchar;
use crate::librb::ssize_t;
pub type size_t = libc::c_ulong;
use crate::librb::pid_t;
pub type socklen_t = __socklen_t;
pub type id_t = __id_t;
use crate::librb::time_t;

use crate::librb::timeval;

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
pub struct sockaddr_in6 {
  pub sin6_family: sa_family_t,
  pub sin6_port: in_port_t,
  pub sin6_flowinfo: uint32_t,
  pub sin6_addr: in6_addr,
  pub sin6_scope_id: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
  pub __in6_u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
  pub __u6_addr8: [uint8_t; 16],
  pub __u6_addr16: [uint16_t; 8],
  pub __u6_addr32: [uint32_t; 4],
}
pub type in_port_t = uint16_t;
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
pub type in_addr_t = uint32_t;
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

pub type __sighandler_t = Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;

use crate::librb::FILE;
use crate::librb::__compar_fn_t;
pub type nfds_t = libc::c_ulong;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
  pub fd: libc::c_int,
  pub events: libc::c_short,
  pub revents: libc::c_short,
}
pub type __priority_which = libc::c_uint;
pub const PRIO_USER: __priority_which = 2;
pub const PRIO_PGRP: __priority_which = 1;
pub const PRIO_PROCESS: __priority_which = 0;
pub type __priority_which_t = __priority_which;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timezone {
  pub tz_minuteswest: libc::c_int,
  pub tz_dsttime: libc::c_int,
}
pub type __timezone_ptr_t = *mut timezone;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct timex {
  pub modes: libc::c_uint,
  pub offset: __syscall_slong_t,
  pub freq: __syscall_slong_t,
  pub maxerror: __syscall_slong_t,
  pub esterror: __syscall_slong_t,
  pub status: libc::c_int,
  pub constant: __syscall_slong_t,
  pub precision: __syscall_slong_t,
  pub tolerance: __syscall_slong_t,
  pub time: timeval,
  pub tick: __syscall_slong_t,
  pub ppsfreq: __syscall_slong_t,
  pub jitter: __syscall_slong_t,
  pub shift: libc::c_int,
  pub stabil: __syscall_slong_t,
  pub jitcnt: __syscall_slong_t,
  pub calcnt: __syscall_slong_t,
  pub errcnt: __syscall_slong_t,
  pub stbcnt: __syscall_slong_t,
  pub tai: libc::c_int,
  #[bitfield(name = "c2rust_unnamed", ty = "libc::c_int", bits = "0..=31")]
  #[bitfield(name = "c2rust_unnamed_0", ty = "libc::c_int", bits = "32..=63")]
  #[bitfield(name = "c2rust_unnamed_1", ty = "libc::c_int", bits = "64..=95")]
  #[bitfield(name = "c2rust_unnamed_2", ty = "libc::c_int", bits = "96..=127")]
  #[bitfield(name = "c2rust_unnamed_3", ty = "libc::c_int", bits = "128..=159")]
  #[bitfield(name = "c2rust_unnamed_4", ty = "libc::c_int", bits = "160..=191")]
  #[bitfield(name = "c2rust_unnamed_5", ty = "libc::c_int", bits = "192..=223")]
  #[bitfield(name = "c2rust_unnamed_6", ty = "libc::c_int", bits = "224..=255")]
  #[bitfield(name = "c2rust_unnamed_7", ty = "libc::c_int", bits = "256..=287")]
  #[bitfield(name = "c2rust_unnamed_8", ty = "libc::c_int", bits = "288..=319")]
  #[bitfield(name = "c2rust_unnamed_9", ty = "libc::c_int", bits = "320..=351")]
  pub c2rust_unnamed_c2rust_unnamed_0_c2rust_unnamed_1_c2rust_unnamed_2_c2rust_unnamed_3_c2rust_unnamed_4_c2rust_unnamed_5_c2rust_unnamed_6_c2rust_unnamed_7_c2rust_unnamed_8_c2rust_unnamed_9:
    [u8; 44],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct len_and_sockaddr {
  pub len: socklen_t,
  pub u: C2RustUnnamed_2,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
  pub sa: sockaddr,
  pub sin: sockaddr_in,
  pub sin6: sockaddr_in6,
}

pub type C2RustUnnamed_3 = libc::c_uint;
pub const DAEMON_ONLY_SANITIZE: C2RustUnnamed_3 = 8;
pub const DAEMON_CLOSE_EXTRA_FDS: C2RustUnnamed_3 = 4;
pub const DAEMON_DEVNULL_STDIO: C2RustUnnamed_3 = 2;
pub const DAEMON_CHDIR_ROOT: C2RustUnnamed_3 = 1;
use crate::libbb::llist::llist_t;

pub type C2RustUnnamed_4 = libc::c_uint;
pub const LOGMODE_BOTH: C2RustUnnamed_4 = 3;
pub const LOGMODE_SYSLOG: C2RustUnnamed_4 = 2;
pub const LOGMODE_STDIO: C2RustUnnamed_4 = 1;
pub const LOGMODE_NONE: C2RustUnnamed_4 = 0;

pub type C2RustUnnamed_5 = libc::c_uint;
pub const PARSE_NORMAL: C2RustUnnamed_5 = 4653056;
pub const PARSE_WS_COMMENTS: C2RustUnnamed_5 = 16777216;
pub const PARSE_ALT_COMMENTS: C2RustUnnamed_5 = 8388608;
pub const PARSE_EOL_COMMENTS: C2RustUnnamed_5 = 4194304;
pub const PARSE_KEEP_COPY: C2RustUnnamed_5 = 2097152;
pub const PARSE_MIN_DIE: C2RustUnnamed_5 = 1048576;
pub const PARSE_GREEDY: C2RustUnnamed_5 = 262144;
pub const PARSE_TRIM: C2RustUnnamed_5 = 131072;
pub const PARSE_COLLAPSE: C2RustUnnamed_5 = 65536;

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
use crate::librb::md5_ctx_t;
use crate::librb::sha1_ctx_t;
//extern const int const_int_1;
/* This struct is deliberately not defined. */
/* See docs/keep_data_small.txt */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub cur_time: libc::c_double,
  pub rootdelay: libc::c_double,
  pub reftime: libc::c_double,
  pub rootdisp: libc::c_double,
  pub last_script_run: libc::c_double,
  pub script_name: *mut libc::c_char,
  pub ntp_peers: *mut llist_t,
  pub listen_fd: libc::c_int,
  pub if_name: *mut libc::c_char,
  pub verbose: libc::c_uint,
  pub peer_cnt: libc::c_uint,
  pub refid: uint32_t,
  pub ntp_status: uint8_t,
  pub stratum: uint8_t,
  pub discipline_state: uint8_t,
  pub poll_exp: uint8_t,
  pub polladj_count: libc::c_int,
  pub FREQHOLD_cnt: libc::c_int,
  pub kernel_freq_drift: libc::c_long,
  pub last_update_peer: *mut peer_t,
  pub last_update_offset: libc::c_double,
  pub last_update_recv_time: libc::c_double,
  pub discipline_jitter: libc::c_double,
  pub offset_to_jitter_ratio: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct peer_t {
  pub p_lsa: *mut len_and_sockaddr,
  pub p_dotted: *mut libc::c_char,
  pub key_entry: *mut key_entry_t,
  pub p_fd: libc::c_int,
  pub datapoint_idx: libc::c_int,
  pub lastpkt_refid: uint32_t,
  pub lastpkt_status: uint8_t,
  pub lastpkt_stratum: uint8_t,
  pub reachable_bits: uint8_t,
  pub dns_errors: uint8_t,
  pub next_action_time: libc::c_double,
  pub p_xmttime: libc::c_double,
  pub p_raw_delay: libc::c_double,
  pub lastpkt_recv_time: libc::c_double,
  pub lastpkt_delay: libc::c_double,
  pub lastpkt_rootdelay: libc::c_double,
  pub lastpkt_rootdisp: libc::c_double,
  pub filter_offset: libc::c_double,
  pub filter_dispersion: libc::c_double,
  pub filter_jitter: libc::c_double,
  pub filter_datapoint: [datapoint_t; 8],
  pub p_xmt_msg: msg_t,
  pub p_hostname: [libc::c_char; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct msg_t {
  pub m_status: uint8_t,
  pub m_stratum: uint8_t,
  pub m_ppoll: uint8_t,
  pub m_precision_exp: int8_t,
  pub m_rootdelay: s_fixedpt_t,
  pub m_rootdisp: s_fixedpt_t,
  pub m_refid: uint32_t,
  pub m_reftime: l_fixedpt_t,
  pub m_orgtime: l_fixedpt_t,
  pub m_rectime: l_fixedpt_t,
  pub m_xmttime: l_fixedpt_t,
  pub m_keyid: uint32_t,
  pub m_digest: [uint8_t; 20],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_fixedpt_t {
  pub int_partl: uint32_t,
  pub fractionl: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct s_fixedpt_t {
  pub int_parts: uint16_t,
  pub fractions: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct datapoint_t {
  pub d_offset: libc::c_double,
  pub d_recv_time: libc::c_double,
  pub d_dispersion: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct key_entry_t {
  pub id: libc::c_uint,
  pub type_0: smalluint,
  pub msg_size: smalluint,
  pub key_length: smalluint,
  pub key: [libc::c_char; 0],
}
pub type C2RustUnnamed_6 = libc::c_uint;
pub const MODE_RES2: C2RustUnnamed_6 = 7;
pub const MODE_RES1: C2RustUnnamed_6 = 6;
pub const MODE_BROADCAST: C2RustUnnamed_6 = 5;
pub const MODE_SERVER: C2RustUnnamed_6 = 4;
pub const MODE_CLIENT: C2RustUnnamed_6 = 3;
pub const MODE_SYM_PAS: C2RustUnnamed_6 = 2;
pub const MODE_SYM_ACT: C2RustUnnamed_6 = 1;
pub const MODE_RES0: C2RustUnnamed_6 = 0;
pub const LI_ALARM: C2RustUnnamed_6 = 192;
pub const LI_MINUSSEC: C2RustUnnamed_6 = 128;
pub const LI_PLUSSEC: C2RustUnnamed_6 = 64;
pub const LI_NOWARNING: C2RustUnnamed_6 = 0;
pub const LI_MASK: C2RustUnnamed_6 = 192;
pub const VERSION_SHIFT: C2RustUnnamed_6 = 3;
pub const VERSION_MASK: C2RustUnnamed_6 = 56;
pub const MODE_MASK: C2RustUnnamed_6 = 7;
pub const NTP_MSGSIZE_SHA1_AUTH: C2RustUnnamed_6 = 72;
pub const NTP_SHA1_DIGESTSIZE: C2RustUnnamed_6 = 20;
pub const NTP_MSGSIZE_MD5_AUTH: C2RustUnnamed_6 = 68;
pub const NTP_MSGSIZE_NOAUTH: C2RustUnnamed_6 = 48;
pub const NTP_MD5_DIGESTSIZE: C2RustUnnamed_6 = 16;
pub const NTP_MAXSTRATUM: C2RustUnnamed_6 = 15;
pub const NTP_VERSION: C2RustUnnamed_6 = 4;
pub type C2RustUnnamed_7 = libc::c_uint;
pub const HASH_SHA1: C2RustUnnamed_7 = 1;
pub const HASH_MD5: C2RustUnnamed_7 = 0;
pub type C2RustUnnamed_8 = libc::c_int;
pub const OPT_qq: C2RustUnnamed_8 = -2147483648;
pub const OPT_I: C2RustUnnamed_8 = 512;
pub const OPT_l: C2RustUnnamed_8 = 256;
pub const OPT_S: C2RustUnnamed_8 = 128;
pub const OPT_p: C2RustUnnamed_8 = 64;
pub const OPT_w: C2RustUnnamed_8 = 32;
pub const OPT_k: C2RustUnnamed_8 = 16;
pub const OPT_x: C2RustUnnamed_8 = 8;
pub const OPT_N: C2RustUnnamed_8 = 4;
pub const OPT_q: C2RustUnnamed_8 = 2;
pub const OPT_n: C2RustUnnamed_8 = 1;

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
  pub f: libc::c_float,
  pub i: int32_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
  pub m: md5_ctx_t,
  pub s: sha1_ctx_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct point_t {
  pub p: *mut peer_t,
  pub type_0: libc::c_int,
  pub edge: libc::c_double,
  pub opt_rd: libc::c_double,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct survivor_t {
  pub p: *mut peer_t,
  pub metric: libc::c_double,
}

#[inline(always)]
unsafe extern "C" fn not_const_pp(mut p: *const libc::c_void) -> *mut libc::c_void {
  return p as *mut libc::c_void;
}

//double   cluster_offset;        // s.offset
//double   cluster_jitter;        // s.jitter
unsafe extern "C" fn LOG2D(mut a: libc::c_int) -> libc::c_double {
  if a < 0i32 {
    return 1.0f64 / (1u64 << -a) as libc::c_double;
  }
  return (1u64 << a) as libc::c_double;
}

#[inline(always)]
unsafe extern "C" fn SQUARE(mut x: libc::c_double) -> libc::c_double {
  return x * x;
}

#[inline(always)]
unsafe extern "C" fn MAXD(mut a: libc::c_double, mut b: libc::c_double) -> libc::c_double {
  if a > b {
    return a;
  }
  return b;
}
#[inline(never)]
unsafe extern "C" fn my_SQRT(mut X: libc::c_double) -> libc::c_double {
  let mut v: C2RustUnnamed_9 = C2RustUnnamed_9 { f: 0. };
  let mut invsqrt: libc::c_double = 0.;
  let mut Xhalf: libc::c_double = X * 0.5f64;
  /* Fast and good approximation to 1/sqrt(X), black magic */
  v.f = X as libc::c_float;
  /*v.i = 0x5f3759df - (v.i >> 1);*/
  v.i = 0x5f375a86i32 - (v.i >> 1i32); /* - this constant is slightly better */
  invsqrt = v.f as libc::c_double; /* better than 0.2% accuracy */
  /* Refining it using Newton's method: x1 = x0 - f(x0)/f'(x0)
   * f(x) = 1/(x*x) - X  (f==0 when x = 1/sqrt(X))
   * f'(x) = -2/(x*x*x)
   * f(x)/f'(x) = (X - 1/(x*x)) / (2/(x*x*x)) = X*x*x*x/2 - x/2
   * x1 = x0 - (X*x0*x0*x0/2 - x0/2) = 1.5*x0 - X*x0*x0*x0/2 = x0*(1.5 - (X/2)*x0*x0)
   */
  invsqrt = invsqrt * (1.5f64 - Xhalf * invsqrt * invsqrt); /* ~0.05% accuracy */
  /* invsqrt = invsqrt * (1.5 - Xhalf * invsqrt * invsqrt); 2nd iter: ~0.0001% accuracy */
  /* With 4 iterations, more than half results will be exact,
   * at 6th iterations result stabilizes with about 72% results exact.
   * We are well satisfied with 0.05% accuracy.
   */
  return X * invsqrt;
  /* X * 1/sqrt(X) ~= sqrt(X) */
}

#[inline(always)]
unsafe extern "C" fn SQRT(mut X: libc::c_double) -> libc::c_double {
  /* If this arch doesn't use IEEE 754 floats, fall back to using libm */
  if ::std::mem::size_of::<libc::c_float>() as libc::c_ulong != 4i32 as libc::c_ulong {
    return sqrt(X);
  }
  /* This avoids needing libm, saves about 0.5k on x86-32 */
  return my_SQRT(X); /* never fails */
}

unsafe extern "C" fn gettime1900d() -> libc::c_double {
  let mut tv: timeval = timeval {
    tv_sec: 0,
    tv_usec: 0,
  };
  gettimeofday(&mut tv, 0 as *mut timezone);
  (*ptr_to_globals).cur_time = tv.tv_sec as libc::c_double
    + 1.0e-6f64 * tv.tv_usec as libc::c_double
    + 2208988800u64 as libc::c_double;
  return (*ptr_to_globals).cur_time;
}

unsafe extern "C" fn d_to_tv(mut d: libc::c_double, mut tv: *mut timeval) {
  (*tv).tv_sec = d as libc::c_long;
  (*tv).tv_usec =
    ((d - (*tv).tv_sec as libc::c_double) * 1000000i32 as libc::c_double) as __suseconds_t;
}

unsafe extern "C" fn lfp_to_d(mut lfp: l_fixedpt_t) -> libc::c_double {
  let mut ret: libc::c_double = 0.;
  lfp.int_partl = {
    let mut __v: libc::c_uint = 0;
    let mut __x: libc::c_uint = lfp.int_partl;
    if 0 != 0 {
      __v = (__x & 0xff000000u32) >> 24i32
        | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
        | (__x & 0xff00i32 as libc::c_uint) << 8i32
        | (__x & 0xffi32 as libc::c_uint) << 24i32
    } else {
      let fresh0 = &mut __v;
      let fresh1;
      let fresh2 = __x;
      asm!("bswap $0" : "=r" (fresh1) : "0"
                      (c2rust_asm_casts::AsmCast::cast_in(fresh0, fresh2)) :);
      c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
    }
    __v
  };
  lfp.fractionl = {
    let mut __v: libc::c_uint = 0;
    let mut __x: libc::c_uint = lfp.fractionl;
    if 0 != 0 {
      __v = (__x & 0xff000000u32) >> 24i32
        | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
        | (__x & 0xff00i32 as libc::c_uint) << 8i32
        | (__x & 0xffi32 as libc::c_uint) << 24i32
    } else {
      let fresh3 = &mut __v;
      let fresh4;
      let fresh5 = __x;
      asm!("bswap $0" : "=r" (fresh4) : "0"
                      (c2rust_asm_casts::AsmCast::cast_in(fresh3, fresh5)) :);
      c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh5, fresh4);
    }
    __v
  };
  ret = lfp.int_partl as libc::c_double
    + lfp.fractionl as libc::c_double
      / (2147483647i32 as libc::c_uint)
        .wrapping_mul(2u32)
        .wrapping_add(1u32) as libc::c_double;
  return ret;
}

unsafe extern "C" fn sfp_to_d(mut sfp: s_fixedpt_t) -> libc::c_double {
  let mut ret: libc::c_double = 0.;
  sfp.int_parts = {
    let mut __v: libc::c_ushort = 0;
    let mut __x: libc::c_ushort = sfp.int_parts;
    if 0 != 0 {
      __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
        as libc::c_ushort
    } else {
      let fresh6 = &mut __v;
      let fresh7;
      let fresh8 = __x;
      asm!("rorw $$8, ${0:w}" : "=r" (fresh7) : "0"
                      (c2rust_asm_casts::AsmCast::cast_in(fresh6, fresh8)) :
                      "cc");
      c2rust_asm_casts::AsmCast::cast_out(fresh6, fresh8, fresh7);
    }
    __v
  };
  sfp.fractions = {
    let mut __v: libc::c_ushort = 0;
    let mut __x: libc::c_ushort = sfp.fractions;
    if 0 != 0 {
      __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
        as libc::c_ushort
    } else {
      let fresh9 = &mut __v;
      let fresh10;
      let fresh11 = __x;
      asm!("rorw $$8, ${0:w}" : "=r" (fresh10) : "0"
                      (c2rust_asm_casts::AsmCast::cast_in(fresh9, fresh11)) :
                      "cc");
      c2rust_asm_casts::AsmCast::cast_out(fresh9, fresh11, fresh10);
    }
    __v
  };
  ret = sfp.int_parts as libc::c_double
    + sfp.fractions as libc::c_double / (32767i32 * 2i32 + 1i32) as libc::c_double;
  return ret;
}

unsafe extern "C" fn d_to_lfp(mut d: libc::c_double) -> l_fixedpt_t {
  let mut lfp: l_fixedpt_t = l_fixedpt_t {
    int_partl: 0,
    fractionl: 0,
  };
  lfp.int_partl = d as uint32_t;
  lfp.fractionl = ((d - lfp.int_partl as libc::c_double)
    * (2147483647i32 as libc::c_uint)
      .wrapping_mul(2u32)
      .wrapping_add(1u32) as libc::c_double) as uint32_t;
  lfp.int_partl = {
    let mut __v: libc::c_uint = 0;
    let mut __x: libc::c_uint = lfp.int_partl;
    if 0 != 0 {
      __v = (__x & 0xff000000u32) >> 24i32
        | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
        | (__x & 0xff00i32 as libc::c_uint) << 8i32
        | (__x & 0xffi32 as libc::c_uint) << 24i32
    } else {
      let fresh12 = &mut __v;
      let fresh13;
      let fresh14 = __x;
      asm!("bswap $0" : "=r" (fresh13) : "0"
                      (c2rust_asm_casts::AsmCast::cast_in(fresh12, fresh14))
                      :);
      c2rust_asm_casts::AsmCast::cast_out(fresh12, fresh14, fresh13);
    }
    __v
  };
  lfp.fractionl = {
    let mut __v: libc::c_uint = 0;
    let mut __x: libc::c_uint = lfp.fractionl;
    if 0 != 0 {
      __v = (__x & 0xff000000u32) >> 24i32
        | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
        | (__x & 0xff00i32 as libc::c_uint) << 8i32
        | (__x & 0xffi32 as libc::c_uint) << 24i32
    } else {
      let fresh15 = &mut __v;
      let fresh16;
      let fresh17 = __x;
      asm!("bswap $0" : "=r" (fresh16) : "0"
                      (c2rust_asm_casts::AsmCast::cast_in(fresh15, fresh17))
                      :);
      c2rust_asm_casts::AsmCast::cast_out(fresh15, fresh17, fresh16);
    }
    __v
  };
  return lfp;
}
unsafe extern "C" fn d_to_sfp(mut d: libc::c_double) -> s_fixedpt_t {
  let mut sfp: s_fixedpt_t = s_fixedpt_t {
    int_parts: 0,
    fractions: 0,
  };
  sfp.int_parts = d as uint16_t;
  sfp.fractions = ((d - sfp.int_parts as libc::c_int as libc::c_double)
    * (32767i32 * 2i32 + 1i32) as libc::c_double) as uint16_t;
  sfp.int_parts = {
    let mut __v: libc::c_ushort = 0;
    let mut __x: libc::c_ushort = sfp.int_parts;
    if 0 != 0 {
      __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
        as libc::c_ushort
    } else {
      let fresh18 = &mut __v;
      let fresh19;
      let fresh20 = __x;
      asm!("rorw $$8, ${0:w}" : "=r" (fresh19) : "0"
                      (c2rust_asm_casts::AsmCast::cast_in(fresh18, fresh20)) :
                      "cc");
      c2rust_asm_casts::AsmCast::cast_out(fresh18, fresh20, fresh19);
    }
    __v
  };
  sfp.fractions = {
    let mut __v: libc::c_ushort = 0;
    let mut __x: libc::c_ushort = sfp.fractions;
    if 0 != 0 {
      __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
        as libc::c_ushort
    } else {
      let fresh21 = &mut __v;
      let fresh22;
      let fresh23 = __x;
      asm!("rorw $$8, ${0:w}" : "=r" (fresh22) : "0"
                      (c2rust_asm_casts::AsmCast::cast_in(fresh21, fresh23)) :
                      "cc");
      c2rust_asm_casts::AsmCast::cast_out(fresh21, fresh23, fresh22);
    }
    __v
  };
  return sfp;
}
unsafe extern "C" fn dispersion(mut dp: *const datapoint_t) -> libc::c_double {
  return (*dp).d_dispersion + 0.000015f64 * ((*ptr_to_globals).cur_time - (*dp).d_recv_time);
}
unsafe extern "C" fn root_distance(mut p: *mut peer_t) -> libc::c_double {
  /* The root synchronization distance is the maximum error due to
   * all causes of the local clock relative to the primary server.
   * It is defined as half the total delay plus total dispersion
   * plus peer jitter.
   */
  return MAXD(0.01f64, (*p).lastpkt_rootdelay + (*p).lastpkt_delay) / 2i32 as libc::c_double
    + (*p).lastpkt_rootdisp
    + (*p).filter_dispersion
    + 0.000015f64 * ((*ptr_to_globals).cur_time - (*p).lastpkt_recv_time)
    + (*p).filter_jitter;
}
unsafe extern "C" fn set_next(mut p: *mut peer_t, mut t: libc::c_uint) {
  (*p).next_action_time = (*ptr_to_globals).cur_time + t as libc::c_double;
}
/*
 * Peer clock filter and its helpers
 */
unsafe extern "C" fn filter_datapoints(mut p: *mut peer_t) {
  let mut i: libc::c_int = 0; /* most recent datapoint's index */
  let mut idx: libc::c_int = 0;
  let mut sum: libc::c_double = 0.;
  let mut wavg: libc::c_double = 0.;
  let mut fdp: *mut datapoint_t = 0 as *mut datapoint_t;
  fdp = (*p).filter_datapoint.as_mut_ptr();
  idx = (*p).datapoint_idx;
  /* filter_offset: simply use the most recent value */
  (*p).filter_offset = (*fdp.offset(idx as isize)).d_offset;
  /*                     n-1
   *                     ---    dispersion(i)
   * filter_dispersion =  \     -------------
   *                      /       (i+1)
   *                     ---     2
   *                     i=0
   */
  wavg = 0i32 as libc::c_double;
  sum = 0i32 as libc::c_double;
  i = 0i32;
  while i < 8i32 {
    sum += dispersion(&mut *fdp.offset(idx as isize)) / (2i32 << i) as libc::c_double;
    wavg += (*fdp.offset(idx as isize)).d_offset;
    idx = idx - 1i32 & 8i32 - 1i32;
    i += 1
  }
  wavg /= 8i32 as libc::c_double;
  (*p).filter_dispersion = sum;
  /*                  +-----                 -----+ ^ 1/2
   *                  |       n-1                 |
   *                  |       ---                 |
   *                  |  1    \                2  |
   * filter_jitter =  | --- * /  (avg-offset_j)   |
   *                  |  n    ---                 |
   *                  |       j=0                 |
   *                  +-----                 -----+
   * where n is the number of valid datapoints in the filter (n > 1);
   * if filter_jitter < precision then filter_jitter = precision
   */
  sum = 0i32 as libc::c_double;
  i = 0i32;
  while i < 8i32 {
    sum += SQUARE(wavg - (*fdp.offset(i as isize)).d_offset);
    i += 1
  }
  sum = SQRT(sum / 8i32 as libc::c_double);
  (*p).filter_jitter = if sum > 0.002f64 { sum } else { 0.002f64 };
  if 3i32 >= 4i32 && (*ptr_to_globals).verbose >= 4i32 as libc::c_uint {
    bb_error_msg(
      b"filter offset:%+f disp:%f jitter:%f\x00" as *const u8 as *const libc::c_char,
      (*p).filter_offset,
      (*p).filter_dispersion,
      (*p).filter_jitter,
    );
  };
}
unsafe extern "C" fn reset_peer_stats(mut p: *mut peer_t, mut offset: libc::c_double) {
  let mut i: libc::c_int = 0;
  let mut small_ofs: bool = fabs(offset) < 1i32 as libc::c_double;
  /* Used to set p->filter_datapoint[i].d_dispersion = MAXDISP
   * and clear reachable bits, but this proved to be too aggressive:
   * after step (tested with suspending laptop for ~30 secs),
   * this caused all previous data to be considered invalid,
   * making us needing to collect full ~8 datapoints per peer
   * after step in order to start trusting them.
   * In turn, this was making poll interval decrease even after
   * step was done. (Poll interval decreases already before step
   * in this scenario, because we see large offsets and end up with
   * no good peer to select).
   */
  i = 0i32;
  while i < 8i32 {
    if small_ofs {
      (*p).filter_datapoint[i as usize].d_recv_time += offset;
      if (*p).filter_datapoint[i as usize].d_offset != 0i32 as libc::c_double {
        (*p).filter_datapoint[i as usize].d_offset -= offset
        //bb_error_msg("p->filter_datapoint[%d].d_offset %f -> %f",
        //	i,
        //	p->filter_datapoint[i].d_offset + offset,
        //	p->filter_datapoint[i].d_offset);
      }
    } else {
      (*p).filter_datapoint[i as usize].d_recv_time = (*ptr_to_globals).cur_time;
      (*p).filter_datapoint[i as usize].d_offset = 0i32 as libc::c_double
      /*p->filter_datapoint[i].d_dispersion = MAXDISP;*/
    }
    i += 1
  }
  if small_ofs {
    (*p).lastpkt_recv_time += offset
  } else {
    /*p->reachable_bits = 0;*/
    (*p).lastpkt_recv_time = (*ptr_to_globals).cur_time
  } /* recalc p->filter_xxx */
  filter_datapoints(p); /* = set_next(p, 0); */
  if 3i32 >= 6i32 && (*ptr_to_globals).verbose >= 6i32 as libc::c_uint {
    bb_error_msg(
      b"%s->lastpkt_recv_time=%f\x00" as *const u8 as *const libc::c_char,
      (*p).p_dotted,
      (*p).lastpkt_recv_time,
    );
  };
}
unsafe extern "C" fn resolve_peer_hostname(mut p: *mut peer_t) -> *mut len_and_sockaddr {
  let mut lsa: *mut len_and_sockaddr = host2sockaddr((*p).p_hostname.as_mut_ptr(), 123i32);
  if !lsa.is_null() {
    free((*p).p_lsa as *mut libc::c_void);
    free((*p).p_dotted as *mut libc::c_void);
    (*p).p_lsa = lsa;
    (*p).p_dotted = xmalloc_sockaddr2dotted_noport(&mut (*lsa).u.sa);
    if 3i32 != 0 && (*ptr_to_globals).verbose != 0 {
      if strcmp((*p).p_hostname.as_mut_ptr(), (*p).p_dotted) != 0i32 {
        bb_error_msg(
          b"\'%s\' is %s\x00" as *const u8 as *const libc::c_char,
          (*p).p_hostname.as_mut_ptr(),
          (*p).p_dotted,
        );
      }
    }
    (*p).dns_errors = 0i32 as uint8_t;
    return lsa;
  }
  (*p).dns_errors = ((((*p).dns_errors as libc::c_int) << 1i32 | 1i32) & 0x3fi32) as uint8_t;
  return lsa;
}
unsafe extern "C" fn add_peers(mut s: *const libc::c_char, mut key_entry: *mut key_entry_t) {
  let mut item: *mut llist_t = 0 as *mut llist_t;
  let mut p: *mut peer_t = 0 as *mut peer_t;
  p = xzalloc((::std::mem::size_of::<peer_t>() as libc::c_ulong).wrapping_add(strlen(s)))
    as *mut peer_t;
  strcpy((*p).p_hostname.as_mut_ptr(), s);
  (*p).p_fd = -1i32;
  (*p).p_xmt_msg.m_status =
    (MODE_CLIENT as libc::c_int | (NTP_VERSION as libc::c_int) << 3i32) as uint8_t;
  (*p).next_action_time = (*ptr_to_globals).cur_time;
  reset_peer_stats(p, 1i32 as libc::c_double);
  /* Names like N.<country2chars>.pool.ntp.org are randomly resolved
   * to a pool of machines. Sometimes different N's resolve to the same IP.
   * It is not useful to have two peers with same IP. We skip duplicates.
   */
  if !resolve_peer_hostname(p).is_null() {
    item = (*ptr_to_globals).ntp_peers;
    while !item.is_null() {
      let mut pp: *mut peer_t = (*item).data as *mut peer_t;
      if !(*pp).p_dotted.is_null() && strcmp((*p).p_dotted, (*pp).p_dotted) == 0i32 {
        bb_error_msg(
          b"duplicate peer %s (%s)\x00" as *const u8 as *const libc::c_char,
          s,
          (*p).p_dotted,
        );
        free((*p).p_lsa as *mut libc::c_void);
        free((*p).p_dotted as *mut libc::c_void);
        free(p as *mut libc::c_void);
        return;
      }
      item = (*item).link
    }
  }
  (*p).key_entry = key_entry;
  llist_add_to(&mut (*ptr_to_globals).ntp_peers, p as *mut libc::c_void);
  (*ptr_to_globals).peer_cnt = (*ptr_to_globals).peer_cnt.wrapping_add(1);
}
unsafe extern "C" fn do_sendto(
  mut fd: libc::c_int,
  mut from: *const sockaddr,
  mut to: *const sockaddr,
  mut addrlen: socklen_t,
  mut msg: *mut msg_t,
  mut len: ssize_t,
) -> libc::c_int {
  let mut ret: ssize_t = 0;
  *bb_errno = 0i32;
  if from.is_null() {
    ret = sendto(
      fd,
      msg as *const libc::c_void,
      len as size_t,
      MSG_DONTWAIT as libc::c_int,
      __CONST_SOCKADDR_ARG { __sockaddr__: to },
      addrlen,
    )
  } else {
    ret = send_to_from(
      fd,
      msg as *mut libc::c_void,
      len as size_t,
      MSG_DONTWAIT as libc::c_int,
      to,
      from,
      addrlen,
    )
  }
  if ret != len {
    bb_simple_perror_msg(b"send failed\x00" as *const u8 as *const libc::c_char);
    return -1i32;
  }
  return 0i32;
}
unsafe extern "C" fn hash(
  mut key_entry: *mut key_entry_t,
  mut msg: *const msg_t,
  mut output: *mut uint8_t,
) {
  let mut ctx: C2RustUnnamed_10 = C2RustUnnamed_10 {
    m: md5_ctx_t {
      wbuffer: [0; 64],
      process_block: None,
      total64: 0,
      hash: [0; 8],
    },
  };
  let mut hash_size: libc::c_uint = (::std::mem::size_of::<msg_t>() as libc::c_ulong)
    .wrapping_sub(::std::mem::size_of::<uint32_t>() as libc::c_ulong)
    .wrapping_sub(::std::mem::size_of::<[uint8_t; 20]>() as libc::c_ulong)
    as libc::c_uint;
  match (*key_entry).type_0 as libc::c_int {
    0 => {
      md5_begin(&mut ctx.m);
      md5_hash(
        &mut ctx.m,
        (*key_entry).key.as_mut_ptr() as *const libc::c_void,
        (*key_entry).key_length as size_t,
      );
      md5_hash(&mut ctx.m, msg as *const libc::c_void, hash_size as size_t);
      md5_end(&mut ctx.m, output as *mut libc::c_void);
    }
    _ => {
      /* it's HASH_SHA1 */
      sha1_begin(&mut ctx.s);
      md5_hash(
        &mut ctx.s,
        (*key_entry).key.as_mut_ptr() as *const libc::c_void,
        (*key_entry).key_length as size_t,
      );
      md5_hash(&mut ctx.s, msg as *const libc::c_void, hash_size as size_t);
      sha1_end(&mut ctx.s, output as *mut libc::c_void);
    }
  };
}
unsafe extern "C" fn hash_peer(mut p: *mut peer_t) {
  (*p).p_xmt_msg.m_keyid = {
    let mut __v: libc::c_uint = 0;
    let mut __x: libc::c_uint = (*(*p).key_entry).id;
    if 0 != 0 {
      __v = (__x & 0xff000000u32) >> 24i32
        | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
        | (__x & 0xff00i32 as libc::c_uint) << 8i32
        | (__x & 0xffi32 as libc::c_uint) << 24i32
    } else {
      let fresh24 = &mut __v;
      let fresh25;
      let fresh26 = __x;
      asm!("bswap $0" : "=r" (fresh25) : "0"
                      (c2rust_asm_casts::AsmCast::cast_in(fresh24, fresh26))
                      :);
      c2rust_asm_casts::AsmCast::cast_out(fresh24, fresh26, fresh25);
    }
    __v
  };
  hash(
    (*p).key_entry,
    &mut (*p).p_xmt_msg,
    (*p).p_xmt_msg.m_digest.as_mut_ptr(),
  );
}
unsafe extern "C" fn hashes_differ(mut p: *mut peer_t, mut msg: *const msg_t) -> libc::c_int {
  let mut digest: [uint8_t; 20] = [0; 20];
  hash((*p).key_entry, msg, digest.as_mut_ptr());
  return memcmp(
    digest.as_mut_ptr() as *const libc::c_void,
    (*msg).m_digest.as_ptr() as *const libc::c_void,
    (((*(*p).key_entry).msg_size as libc::c_int - NTP_MSGSIZE_NOAUTH as libc::c_int)
      as libc::c_ulong)
      .wrapping_sub(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
  );
}
unsafe extern "C" fn send_query_to_peer(mut p: *mut peer_t) {
  if (*p).p_lsa.is_null() {
    return;
  }
  /* Why do we need to bind()?
   * See what happens when we don't bind:
   *
   * socket(PF_INET, SOCK_DGRAM, IPPROTO_IP) = 3
   * setsockopt(3, SOL_IP, IP_TOS, [16], 4) = 0
   * gettimeofday({1259071266, 327885}, NULL) = 0
   * sendto(3, "xxx", 48, MSG_DONTWAIT, {sa_family=AF_INET, sin_port=htons(123), sin_addr=inet_addr("10.34.32.125")}, 16) = 48
   * ^^^ we sent it from some source port picked by kernel.
   * time(NULL)              = 1259071266
   * write(2, "ntpd: entering poll 15 secs\n", 28) = 28
   * poll([{fd=3, events=POLLIN}], 1, 15000) = 1 ([{fd=3, revents=POLLIN}])
   * recv(3, "yyy", 68, MSG_DONTWAIT) = 48
   * ^^^ this recv will receive packets to any local port!
   *
   * Uncomment this and use strace to see it in action:
   */
  /* { len_and_sockaddr lsa; lsa.len = LSA_SIZEOF_SA; getsockname(p->query.fd, &lsa.u.sa, &lsa.len); } */
  if (*p).p_fd == -1i32 {
    let mut fd: libc::c_int = 0;
    let mut family: libc::c_int = 0;
    let mut local_lsa: *mut len_and_sockaddr = 0 as *mut len_and_sockaddr;
    family = (*(*p).p_lsa).u.sa.sa_family as libc::c_int;
    fd = xsocket_type(&mut local_lsa, family, SOCK_DGRAM as libc::c_int);
    (*p).p_fd = fd;
    /* local_lsa has "null" address and port 0 now.
     * bind() ensures we have a *particular port* selected by kernel
     * and remembered in p->p_fd, thus later recv(p->p_fd)
     * receives only packets sent to this port.
     */
    xbind(fd, &mut (*local_lsa).u.sa, (*local_lsa).len);
    if family == 2i32 {
      setsockopt_int(fd, IPPROTO_IP as libc::c_int, 1i32, 0x48i32);
    }
    free(local_lsa as *mut libc::c_void);
  }
  /* Emit message _before_ attempted send. Think of a very short
   * roundtrip networks: we need to go back to recv loop ASAP,
   * to reduce delay. Printing messages after send works against that.
   */
  if 3i32 != 0 && (*ptr_to_globals).verbose != 0 {
    bb_error_msg(
      b"sending query to %s\x00" as *const u8 as *const libc::c_char,
      (*p).p_dotted,
    );
  }
  /*
   * Send out a random 64-bit number as our transmit time.  The NTP
   * server will copy said number into the originate field on the
   * response that it sends us.  This is totally legal per the SNTP spec.
   *
   * The impact of this is two fold: we no longer send out the current
   * system time for the world to see (which may aid an attacker), and
   * it gives us a (not very secure) way of knowing that we're not
   * getting spoofed by an attacker that can't capture our traffic
   * but can spoof packets from the NTP server we're communicating with.
   *
   * Save the real transmit timestamp locally.
   */
  (*p).p_xmt_msg.m_xmttime.int_partl = rand() as uint32_t;
  (*p).p_xmt_msg.m_xmttime.fractionl = rand() as uint32_t;
  (*p).p_xmttime = gettime1900d();
  /* Were doing it only if sendto worked, but
   * loss of sync detection needs reachable_bits updated
   * even if sending fails *locally*:
   * "network is unreachable" because cable was pulled?
   * We still need to declare "unsync" if this condition persists.
   */
  (*p).reachable_bits = (((*p).reachable_bits as libc::c_int) << 1i32) as uint8_t;
  if !(*p).key_entry.is_null() {
    hash_peer(p);
  }
  if do_sendto(
    (*p).p_fd,
    0 as *const sockaddr,
    &mut (*(*p).p_lsa).u.sa,
    (*(*p).p_lsa).len,
    &mut (*p).p_xmt_msg,
    (if (*p).key_entry.is_null() {
      NTP_MSGSIZE_NOAUTH as libc::c_int
    } else {
      (*(*p).key_entry).msg_size as libc::c_int
    }) as ssize_t,
  ) == -1i32
  {
    close((*p).p_fd);
    (*p).p_fd = -1i32;
    /*
     * We know that we sent nothing.
     * We can retry *soon* without fearing
     * that we are flooding the peer.
     */
    set_next(p, 32i32 as libc::c_uint);
    return;
  }
  set_next(p, 16i32 as libc::c_uint);
}
/* Note that there is no provision to prevent several run_scripts
 * to be started in quick succession. In fact, it happens rather often
 * if initial syncronization results in a step.
 * You will see "step" and then "stratum" script runs, sometimes
 * as close as only 0.002 seconds apart.
 * Script should be ready to deal with this.
 */
unsafe extern "C" fn run_script(mut action: *const libc::c_char, mut offset: libc::c_double) {
  let mut argv: [*mut libc::c_char; 3] = [0 as *mut libc::c_char; 3];
  let mut env1: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut env2: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut env3: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut env4: *mut libc::c_char = 0 as *mut libc::c_char;
  (*ptr_to_globals).last_script_run = (*ptr_to_globals).cur_time;
  if (*ptr_to_globals).script_name.is_null() {
    return;
  }
  argv[0] = (*ptr_to_globals).script_name;
  argv[1] = action as *mut libc::c_char;
  argv[2] = 0 as *mut libc::c_char;
  if 3i32 != 0 && (*ptr_to_globals).verbose != 0 {
    bb_error_msg(
      b"executing \'%s %s\'\x00" as *const u8 as *const libc::c_char,
      (*ptr_to_globals).script_name,
      action,
    );
  }
  env1 = xasprintf(
    b"%s=%u\x00" as *const u8 as *const libc::c_char,
    b"stratum\x00" as *const u8 as *const libc::c_char,
    (*ptr_to_globals).stratum as libc::c_int,
  );
  putenv(env1);
  env2 = xasprintf(
    b"%s=%ld\x00" as *const u8 as *const libc::c_char,
    b"freq_drift_ppm\x00" as *const u8 as *const libc::c_char,
    (*ptr_to_globals).kernel_freq_drift,
  );
  putenv(env2);
  env3 = xasprintf(
    b"%s=%u\x00" as *const u8 as *const libc::c_char,
    b"poll_interval\x00" as *const u8 as *const libc::c_char,
    1i32 << (*ptr_to_globals).poll_exp as libc::c_int,
  );
  putenv(env3);
  env4 = xasprintf(
    b"%s=%f\x00" as *const u8 as *const libc::c_char,
    b"offset\x00" as *const u8 as *const libc::c_char,
    offset,
  );
  putenv(env4);
  /* Other items of potential interest: selected peer,
   * rootdelay, reftime, rootdisp, refid, ntp_status,
   * last_update_offset, last_update_recv_time, discipline_jitter,
   * how many peers have reachable_bits = 0?
   */
  /* Don't want to wait: it may run hwclock --systohc, and that
   * may take some time (seconds): */
  /*spawn_and_wait(argv);*/
  spawn(argv.as_mut_ptr());
  unsetenv(b"stratum\x00" as *const u8 as *const libc::c_char);
  unsetenv(b"freq_drift_ppm\x00" as *const u8 as *const libc::c_char);
  unsetenv(b"poll_interval\x00" as *const u8 as *const libc::c_char);
  unsetenv(b"offset\x00" as *const u8 as *const libc::c_char);
  free(env1 as *mut libc::c_void);
  free(env2 as *mut libc::c_void);
  free(env3 as *mut libc::c_void);
  free(env4 as *mut libc::c_void);
}
#[inline(never)]
unsafe extern "C" fn step_time(mut offset: libc::c_double) {
  let mut item: *mut llist_t = 0 as *mut llist_t;
  let mut dtime: libc::c_double = 0.;
  let mut tvc: timeval = timeval {
    tv_sec: 0,
    tv_usec: 0,
  };
  let mut tvn: timeval = timeval {
    tv_sec: 0,
    tv_usec: 0,
  };
  let mut buf: [libc::c_char; 24] = [0; 24];
  let mut tval: time_t = 0;
  /*paranoia:*/
  gettimeofday(&mut tvc, 0 as *mut timezone); /* never fails */
  dtime = tvc.tv_sec as libc::c_double + 1.0e-6f64 * tvc.tv_usec as libc::c_double + offset;
  d_to_tv(dtime, &mut tvn);
  if settimeofday(&mut tvn, 0 as *const timezone) == -1i32 {
    bb_simple_perror_msg_and_die(b"settimeofday\x00" as *const u8 as *const libc::c_char);
  }
  if 3i32 >= 2i32 && (*ptr_to_globals).verbose >= 2i32 as libc::c_uint {
    tval = tvc.tv_sec;
    strftime_YYYYMMDDHHMMSS(
      buf.as_mut_ptr(),
      ::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong as libc::c_uint,
      &mut tval,
    );
    bb_error_msg(
      b"current time is %s.%06u\x00" as *const u8 as *const libc::c_char,
      buf.as_mut_ptr(),
      tvc.tv_usec as libc::c_uint,
    );
  }
  tval = tvn.tv_sec;
  strftime_YYYYMMDDHHMMSS(
    buf.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong as libc::c_uint,
    &mut tval,
  );
  bb_info_msg(
    b"setting time to %s.%06u (offset %+fs)\x00" as *const u8 as *const libc::c_char,
    buf.as_mut_ptr(),
    tvn.tv_usec as libc::c_uint,
    offset,
  );
  //maybe? G.FREQHOLD_cnt = 0;
  /* Correct various fields which contain time-relative values: */
  /* Globals: */
  (*ptr_to_globals).cur_time += offset;
  (*ptr_to_globals).last_update_recv_time += offset;
  (*ptr_to_globals).last_script_run += offset;
  /* p->lastpkt_recv_time, p->next_action_time and such: */
  item = (*ptr_to_globals).ntp_peers;
  while !item.is_null() {
    let mut pp: *mut peer_t = (*item).data as *mut peer_t;
    reset_peer_stats(pp, offset);
    //bb_error_msg("offset:%+f pp->next_action_time:%f -> %f",
    //	offset, pp->next_action_time, pp->next_action_time + offset);
    (*pp).next_action_time += offset;
    if (*pp).p_fd >= 0i32 {
      /* We wait for reply from this peer too.
       * But due to step we are doing, reply's data is no longer
       * useful (in fact, it'll be bogus). Stop waiting for it.
       */
      close((*pp).p_fd);
      (*pp).p_fd = -1i32;
      set_next(pp, 32i32 as libc::c_uint);
    }
    item = (*item).link
  }
}
unsafe extern "C" fn clamp_pollexp_and_set_MAXSTRAT() {
  if ((*ptr_to_globals).poll_exp as libc::c_int) < 5i32 {
    (*ptr_to_globals).poll_exp = 5i32 as uint8_t
  }
  if (*ptr_to_globals).poll_exp as libc::c_int > 9i32 {
    (*ptr_to_globals).poll_exp = 9i32 as uint8_t
  }
  (*ptr_to_globals).polladj_count = 0i32;
  (*ptr_to_globals).stratum = 16i32 as uint8_t;
}
unsafe extern "C" fn compare_point_edge(
  mut aa: *const libc::c_void,
  mut bb: *const libc::c_void,
) -> libc::c_int {
  let mut a: *const point_t = aa as *const point_t;
  let mut b: *const point_t = bb as *const point_t;
  if (*a).edge < (*b).edge {
    return -1i32;
  }
  return ((*a).edge > (*b).edge) as libc::c_int;
}
unsafe extern "C" fn compare_survivor_metric(
  mut aa: *const libc::c_void,
  mut bb: *const libc::c_void,
) -> libc::c_int {
  let mut a: *const survivor_t = aa as *const survivor_t;
  let mut b: *const survivor_t = bb as *const survivor_t;
  if (*a).metric < (*b).metric {
    return -1i32;
  }
  return ((*a).metric > (*b).metric) as libc::c_int;
}
unsafe extern "C" fn fit(mut p: *mut peer_t, mut rd: libc::c_double) -> libc::c_int {
  if (*p).reachable_bits as libc::c_int & (*p).reachable_bits as libc::c_int - 1i32 == 0i32 {
    /* One or zero bits in reachable_bits */
    if 3i32 >= 4i32 && (*ptr_to_globals).verbose >= 4i32 as libc::c_uint {
      bb_error_msg(
        b"peer %s unfit for selection: unreachable\x00" as *const u8 as *const libc::c_char,
        (*p).p_dotted,
      );
    }
    return 0i32;
  }
  /* we filter out such packets earlier */
  /* rd is root_distance(p) */
  if rd
    > 1i32 as libc::c_double
      + 0.000015f64 * (1i32 << (*ptr_to_globals).poll_exp as libc::c_int) as libc::c_double
  {
    if 3i32 >= 3i32 && (*ptr_to_globals).verbose >= 3i32 as libc::c_uint {
      bb_error_msg(
        b"peer %s unfit for selection: root distance %f too high, jitter:%f\x00" as *const u8
          as *const libc::c_char,
        (*p).p_dotted,
        rd,
        (*p).filter_jitter,
      );
    }
    return 0i32;
  }
  //TODO
  //	/* Do we have a loop? */
  //	if (p->refid == p->dstaddr || p->refid == s.refid)
  //		return 0;
  return 1i32;
}
unsafe extern "C" fn select_and_cluster() -> *mut peer_t {
  let mut current_block: u64;
  let mut p: *mut peer_t = 0 as *mut peer_t;
  let mut item: *mut llist_t = 0 as *mut llist_t;
  let mut i: libc::c_int = 0;
  let mut j: libc::c_int = 0;
  let mut size: libc::c_int =
    (3i32 as libc::c_uint).wrapping_mul((*ptr_to_globals).peer_cnt) as libc::c_int;
  /* for selection algorithm */
  let vla = size as usize;
  let mut point: Vec<point_t> = ::std::vec::from_elem(
    point_t {
      p: 0 as *mut peer_t,
      type_0: 0,
      edge: 0.,
      opt_rd: 0.,
    },
    vla,
  );
  let mut num_points: libc::c_uint = 0;
  let mut num_candidates: libc::c_uint = 0;
  let mut low: libc::c_double = 0.;
  let mut high: libc::c_double = 0.;
  let mut num_falsetickers: libc::c_uint = 0;
  /* for cluster algorithm */
  let vla_0 = size as usize;
  let mut survivor: Vec<survivor_t> = ::std::vec::from_elem(
    survivor_t {
      p: 0 as *mut peer_t,
      metric: 0.,
    },
    vla_0,
  );
  let mut num_survivors: libc::c_uint = 0;
  /* Selection */
  num_points = 0i32 as libc::c_uint;
  item = (*ptr_to_globals).ntp_peers;
  while !item.is_null() {
    let mut rd: libc::c_double = 0.;
    let mut offset: libc::c_double = 0.;
    p = (*item).data as *mut peer_t;
    rd = root_distance(p);
    offset = (*p).filter_offset;
    if fit(p, rd) == 0 {
      item = (*item).link
    } else {
      if 3i32 >= 5i32 && (*ptr_to_globals).verbose >= 5i32 as libc::c_uint {
        bb_error_msg(
          b"interval: [%f %f %f] %s\x00" as *const u8 as *const libc::c_char,
          offset - rd,
          offset,
          offset + rd,
          (*p).p_dotted,
        );
      }
      let ref mut fresh27 = (*point.as_mut_ptr().offset(num_points as isize)).p;
      *fresh27 = p;
      (*point.as_mut_ptr().offset(num_points as isize)).type_0 = -1i32;
      (*point.as_mut_ptr().offset(num_points as isize)).edge = offset - rd;
      (*point.as_mut_ptr().offset(num_points as isize)).opt_rd = rd;
      num_points = num_points.wrapping_add(1);
      let ref mut fresh28 = (*point.as_mut_ptr().offset(num_points as isize)).p;
      *fresh28 = p;
      (*point.as_mut_ptr().offset(num_points as isize)).type_0 = 0i32;
      (*point.as_mut_ptr().offset(num_points as isize)).edge = offset;
      (*point.as_mut_ptr().offset(num_points as isize)).opt_rd = rd;
      num_points = num_points.wrapping_add(1);
      let ref mut fresh29 = (*point.as_mut_ptr().offset(num_points as isize)).p;
      *fresh29 = p;
      (*point.as_mut_ptr().offset(num_points as isize)).type_0 = 1i32;
      (*point.as_mut_ptr().offset(num_points as isize)).edge = offset + rd;
      (*point.as_mut_ptr().offset(num_points as isize)).opt_rd = rd;
      num_points = num_points.wrapping_add(1);
      item = (*item).link
    }
  }
  num_candidates = num_points.wrapping_div(3i32 as libc::c_uint);
  if num_candidates == 0i32 as libc::c_uint {
    if 3i32 >= 3i32 && (*ptr_to_globals).verbose >= 3i32 as libc::c_uint {
      bb_error_msg(
        b"no valid datapoints%s\x00" as *const u8 as *const libc::c_char,
        b", no peer selected\x00" as *const u8 as *const libc::c_char,
      );
    }
    return 0 as *mut peer_t;
  }
  //TODO: sorting does not seem to be done in reference code
  qsort(
    point.as_mut_ptr() as *mut libc::c_void,
    num_points as size_t,
    ::std::mem::size_of::<point_t>() as libc::c_ulong,
    Some(
      compare_point_edge
        as unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> libc::c_int,
    ),
  );
  /* Start with the assumption that there are no falsetickers.
   * Attempt to find a nonempty intersection interval containing
   * the midpoints of all truechimers.
   * If a nonempty interval cannot be found, increase the number
   * of assumed falsetickers by one and try again.
   * If a nonempty interval is found and the number of falsetickers
   * is less than the number of truechimers, a majority has been found
   * and the midpoint of each truechimer represents
   * the candidates available to the cluster algorithm.
   */
  num_falsetickers = 0i32 as libc::c_uint;
  loop {
    let mut c: libc::c_int = 0;
    let mut num_midpoints: libc::c_uint = 0i32 as libc::c_uint;
    low = (1i32 << 9i32) as libc::c_double;
    high = -(1i32 << 9i32) as libc::c_double;
    c = 0i32;
    i = 0i32;
    while (i as libc::c_uint) < num_points {
      /* We want to do:
       * if (point[i].type == -1) c++;
       * if (point[i].type == 1) c--;
       * and it's simpler to do it this way:
       */
      c -= (*point.as_mut_ptr().offset(i as isize)).type_0;
      if c as libc::c_uint >= num_candidates.wrapping_sub(num_falsetickers) {
        /* If it was c++ and it got big enough... */
        low = (*point.as_mut_ptr().offset(i as isize)).edge;
        break;
      } else {
        if (*point.as_mut_ptr().offset(i as isize)).type_0 == 0i32 {
          num_midpoints = num_midpoints.wrapping_add(1)
        }
        i += 1
      }
    }
    c = 0i32;
    i = num_points.wrapping_sub(1i32 as libc::c_uint) as libc::c_int;
    while i >= 0i32 {
      c += (*point.as_mut_ptr().offset(i as isize)).type_0;
      if c as libc::c_uint >= num_candidates.wrapping_sub(num_falsetickers) {
        high = (*point.as_mut_ptr().offset(i as isize)).edge;
        break;
      } else {
        if (*point.as_mut_ptr().offset(i as isize)).type_0 == 0i32 {
          num_midpoints = num_midpoints.wrapping_add(1)
        }
        i -= 1
      }
    }
    /* If the number of midpoints is greater than the number
     * of allowed falsetickers, the intersection contains at
     * least one truechimer with no midpoint - bad.
     * Also, interval should be nonempty.
     */
    if num_midpoints <= num_falsetickers && low < high {
      break;
    }
    num_falsetickers = num_falsetickers.wrapping_add(1);
    if num_falsetickers.wrapping_mul(2i32 as libc::c_uint) >= num_candidates {
      if 3i32 >= 3i32 && (*ptr_to_globals).verbose >= 3i32 as libc::c_uint {
        bb_error_msg(
          b"falsetickers:%d, candidates:%d%s\x00" as *const u8 as *const libc::c_char,
          num_falsetickers,
          num_candidates,
          b", no peer selected\x00" as *const u8 as *const libc::c_char,
        );
      }
      return 0 as *mut peer_t;
    }
  }
  if 3i32 >= 4i32 && (*ptr_to_globals).verbose >= 4i32 as libc::c_uint {
    bb_error_msg(
      b"selected interval: [%f, %f]; candidates:%d falsetickers:%d\x00" as *const u8
        as *const libc::c_char,
      low,
      high,
      num_candidates,
      num_falsetickers,
    );
  }
  /* Clustering */
  /* Construct a list of survivors (p, metric)
   * from the chime list, where metric is dominated
   * first by stratum and then by root distance.
   * All other things being equal, this is the order of preference.
   */
  num_survivors = 0i32 as libc::c_uint;
  i = 0i32;
  while (i as libc::c_uint) < num_points {
    if !((*point.as_mut_ptr().offset(i as isize)).edge < low
      || (*point.as_mut_ptr().offset(i as isize)).edge > high)
    {
      p = (*point.as_mut_ptr().offset(i as isize)).p;
      let ref mut fresh30 = (*survivor.as_mut_ptr().offset(num_survivors as isize)).p;
      *fresh30 = p;
      /* x.opt_rd == root_distance(p); */
      (*survivor.as_mut_ptr().offset(num_survivors as isize)).metric =
        (1i32 * (*p).lastpkt_stratum as libc::c_int) as libc::c_double
          + (*point.as_mut_ptr().offset(i as isize)).opt_rd;
      if 3i32 >= 5i32 && (*ptr_to_globals).verbose >= 5i32 as libc::c_uint {
        bb_error_msg(
          b"survivor[%d] metric:%f peer:%s\x00" as *const u8 as *const libc::c_char,
          num_survivors,
          (*survivor.as_mut_ptr().offset(num_survivors as isize)).metric,
          (*p).p_dotted,
        );
      }
      num_survivors = num_survivors.wrapping_add(1)
    }
    i += 1
  }
  /* There must be at least MIN_SELECTED survivors to satisfy the
   * correctness assertions. Ordinarily, the Byzantine criteria
   * require four survivors, but for the demonstration here, one
   * is acceptable.
   */
  if num_survivors < 1i32 as libc::c_uint {
    if 3i32 >= 3i32 && (*ptr_to_globals).verbose >= 3i32 as libc::c_uint {
      bb_error_msg(
        b"survivors:%d%s\x00" as *const u8 as *const libc::c_char,
        num_survivors,
        b", no peer selected\x00" as *const u8 as *const libc::c_char,
      );
    }
    return 0 as *mut peer_t;
  }
  //looks like this is ONLY used by the fact that later we pick survivor[0].
  //we can avoid sorting then, just find the minimum once!
  qsort(
    survivor.as_mut_ptr() as *mut libc::c_void,
    num_survivors as size_t,
    ::std::mem::size_of::<survivor_t>() as libc::c_ulong,
    Some(
      compare_survivor_metric
        as unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> libc::c_int,
    ),
  );
  loop
  /* For each association p in turn, calculate the selection
   * jitter p->sjitter as the square root of the sum of squares
   * (p->offset - q->offset) over all q associations. The idea is
   * to repeatedly discard the survivor with maximum selection
   * jitter until a termination condition is met.
   */
  {
    let mut max_idx: libc::c_uint = 0;
    max_idx = max_idx;
    let mut max_selection_jitter: libc::c_double = 0.;
    max_selection_jitter = max_selection_jitter;
    let mut min_jitter: libc::c_double = 0.;
    min_jitter = min_jitter;
    if num_survivors <= 3i32 as libc::c_uint {
      if 3i32 >= 4i32 && (*ptr_to_globals).verbose >= 4i32 as libc::c_uint {
        bb_error_msg(
          b"num_survivors %d <= %d, not discarding more\x00" as *const u8 as *const libc::c_char,
          num_survivors,
          3i32,
        );
      }
      break;
    } else {
      /* To make sure a few survivors are left
       * for the clustering algorithm to chew on,
       * we stop if the number of survivors
       * is less than or equal to MIN_CLUSTERED (3).
       */
      i = 0i32;
      while (i as libc::c_uint) < num_survivors {
        let mut selection_jitter_sq: libc::c_double = 0.;
        p = (*survivor.as_mut_ptr().offset(i as isize)).p;
        if i == 0i32 || (*p).filter_jitter < min_jitter {
          min_jitter = (*p).filter_jitter
        }
        selection_jitter_sq = 0i32 as libc::c_double;
        j = 0i32;
        while (j as libc::c_uint) < num_survivors {
          let mut q: *mut peer_t = (*survivor.as_mut_ptr().offset(j as isize)).p;
          selection_jitter_sq += SQUARE((*p).filter_offset - (*q).filter_offset);
          j += 1
        }
        if i == 0i32 || selection_jitter_sq > max_selection_jitter {
          max_selection_jitter = selection_jitter_sq;
          max_idx = i as libc::c_uint
        }
        if 3i32 >= 6i32 && (*ptr_to_globals).verbose >= 6i32 as libc::c_uint {
          bb_error_msg(
            b"survivor %d selection_jitter^2:%f\x00" as *const u8 as *const libc::c_char,
            i,
            selection_jitter_sq,
          );
        }
        i += 1
      }
      max_selection_jitter = SQRT(max_selection_jitter / num_survivors as libc::c_double);
      if 3i32 >= 5i32 && (*ptr_to_globals).verbose >= 5i32 as libc::c_uint {
        bb_error_msg(
          b"max_selection_jitter (at %d):%f min_jitter:%f\x00" as *const u8 as *const libc::c_char,
          max_idx,
          max_selection_jitter,
          min_jitter,
        );
      }
      /* If the maximum selection jitter is less than the
       * minimum peer jitter, then tossing out more survivors
       * will not lower the minimum peer jitter, so we might
       * as well stop.
       */
      if max_selection_jitter < min_jitter {
        if 3i32 >= 4i32 && (*ptr_to_globals).verbose >= 4i32 as libc::c_uint {
          bb_error_msg(
            b"max_selection_jitter:%f < min_jitter:%f, num_survivors:%d, not discarding more\x00"
              as *const u8 as *const libc::c_char,
            max_selection_jitter,
            min_jitter,
            num_survivors,
          );
        }
        break;
      } else {
        /* Delete survivor[max_idx] from the list
         * and go around again.
         */
        if 3i32 >= 6i32 && (*ptr_to_globals).verbose >= 6i32 as libc::c_uint {
          bb_error_msg(
            b"dropping survivor %d\x00" as *const u8 as *const libc::c_char,
            max_idx,
          );
        }
        num_survivors = num_survivors.wrapping_sub(1);
        while max_idx < num_survivors {
          *survivor.as_mut_ptr().offset(max_idx as isize) = *survivor
            .as_mut_ptr()
            .offset(max_idx.wrapping_add(1i32 as libc::c_uint) as isize);
          max_idx = max_idx.wrapping_add(1)
        }
      }
    }
  }
  /* Pick the best clock. If the old system peer is on the list
   * and at the same stratum as the first survivor on the list,
   * then don't do a clock hop. Otherwise, select the first
   * survivor on the list as the new system peer.
   */
  p = (*survivor.as_mut_ptr().offset(0)).p;
  if !(*ptr_to_globals).last_update_peer.is_null()
    && (*(*ptr_to_globals).last_update_peer).lastpkt_stratum as libc::c_int
      <= (*p).lastpkt_stratum as libc::c_int
  {
    /* Starting from 1 is ok here */
    i = 1i32;
    loop {
      if !((i as libc::c_uint) < num_survivors) {
        current_block = 92352228884877657;
        break;
      }
      if (*ptr_to_globals).last_update_peer == (*survivor.as_mut_ptr().offset(i as isize)).p {
        if 3i32 >= 5i32 && (*ptr_to_globals).verbose >= 5i32 as libc::c_uint {
          bb_simple_error_msg(b"keeping old synced peer\x00" as *const u8 as *const libc::c_char);
        }
        p = (*ptr_to_globals).last_update_peer;
        current_block = 16970629020640858593;
        break;
      } else {
        i += 1
      }
    }
  } else {
    current_block = 92352228884877657;
  }
  match current_block {
    92352228884877657 => (*ptr_to_globals).last_update_peer = p,
    _ => {}
  }
  if 3i32 >= 4i32 && (*ptr_to_globals).verbose >= 4i32 as libc::c_uint {
    bb_error_msg(
      b"selected peer %s filter_offset:%+f age:%f\x00" as *const u8 as *const libc::c_char,
      (*p).p_dotted,
      (*p).filter_offset,
      (*ptr_to_globals).cur_time - (*p).lastpkt_recv_time,
    );
  }
  return p;
}
/*
 * Local clock discipline and its helpers
 */
unsafe extern "C" fn set_new_values(
  mut disc_state: libc::c_int,
  mut offset: libc::c_double,
  mut recv_time: libc::c_double,
) {
  /* Enter new state and set state variables. Note we use the time
   * of the last clock filter sample, which must be earlier than
   * the current time.
   */
  if 3i32 >= 4i32 && (*ptr_to_globals).verbose >= 4i32 as libc::c_uint {
    bb_error_msg(
      b"disc_state=%d last update offset=%f recv_time=%f\x00" as *const u8 as *const libc::c_char,
      disc_state,
      offset,
      recv_time,
    );
  }
  (*ptr_to_globals).discipline_state = disc_state as uint8_t;
  (*ptr_to_globals).last_update_offset = offset;
  (*ptr_to_globals).last_update_recv_time = recv_time;
}
/* Return: -1: decrease poll interval, 0: leave as is, 1: increase */
#[inline(never)]
unsafe extern "C" fn update_local_clock(mut p: *mut peer_t) -> libc::c_int {
  let mut rc: libc::c_int = 0;
  let mut tmx: timex =
        timex{modes: 0,
              offset: 0,
              freq: 0,
              maxerror: 0,
              esterror: 0,
              status: 0,
              constant: 0,
              precision: 0,
              tolerance: 0,
              time: timeval{tv_sec: 0, tv_usec: 0,},
              tick: 0,
              ppsfreq: 0,
              jitter: 0,
              shift: 0,
              stabil: 0,
              jitcnt: 0,
              calcnt: 0,
              errcnt: 0,
              stbcnt: 0,
              tai: 0,
              c2rust_unnamed_c2rust_unnamed_0_c2rust_unnamed_1_c2rust_unnamed_2_c2rust_unnamed_3_c2rust_unnamed_4_c2rust_unnamed_5_c2rust_unnamed_6_c2rust_unnamed_7_c2rust_unnamed_8_c2rust_unnamed_9:
                  [0; 44],};
  /* Note: can use G.cluster_offset instead: */
  let mut offset: libc::c_double = (*p).filter_offset;
  let mut recv_time: libc::c_double = (*p).lastpkt_recv_time;
  let mut abs_offset: libc::c_double = 0.;
  let mut etemp: libc::c_double = 0.;
  let mut dtemp: libc::c_double = 0.;
  abs_offset = fabs(offset);
  /* If this is an old update, for instance as the result
   * of a system peer change, avoid it. We never use
   * an old sample or the same sample twice.
   */
  if recv_time <= (*ptr_to_globals).last_update_recv_time {
    if 3i32 >= 3i32 && (*ptr_to_globals).verbose >= 3i32 as libc::c_uint {
      bb_error_msg(
        b"update from %s: same or older datapoint, not using it\x00" as *const u8
          as *const libc::c_char,
        (*p).p_dotted,
      );
    }
    return 0i32;
    /* "leave poll interval as is" */
  }
  /* Clock state machine transition function. This is where the
   * action is and defines how the system reacts to large time
   * and frequency errors.
   */
  /* There are two main regimes: when the
   * offset exceeds the step threshold and when it does not.
   */
  if abs_offset > 1i32 as libc::c_double {
    if 3i32 >= 4i32 && (*ptr_to_globals).verbose >= 4i32 as libc::c_uint {
      bb_error_msg(
        b"stepping time by %+f; poll_exp=MINPOLL\x00" as *const u8 as *const libc::c_char,
        offset,
      ); /* abs_offset <= STEP_THRESHOLD */
    }
    step_time(offset);
    if option_mask32 & OPT_q as libc::c_int as libc::c_uint != 0 {
      /* Step the time and clamp down the poll interval.
       *
       * In NSET state an initial frequency correction is
       * not available, usually because the frequency file has
       * not yet been written. Since the time is outside the
       * capture range, the clock is stepped. The frequency
       * will be set directly following the stepout interval.
       *
       * In FSET state the initial frequency has been set
       * from the frequency file. Since the time is outside
       * the capture range, the clock is stepped immediately,
       * rather than after the stepout interval. Guys get
       * nervous if it takes 17 minutes to set the clock for
       * the first time.
       *
       * In SPIK state the stepout threshold has expired and
       * the phase is still above the step threshold. Note
       * that a single spike greater than the step threshold
       * is always suppressed, even at the longer poll
       * intervals.
       */
      /* We were only asked to set time once. Done. */
      exit(0i32);
    }
    clamp_pollexp_and_set_MAXSTRAT();
    run_script(b"step\x00" as *const u8 as *const libc::c_char, offset);
    recv_time += offset;
    offset = 0i32 as libc::c_double;
    abs_offset = offset;
    set_new_values(4i32, offset, recv_time);
  } else {
    /* The ratio is calculated before jitter is updated to make
     * poll adjust code more sensitive to large offsets.
     */
    (*ptr_to_globals).offset_to_jitter_ratio =
      (abs_offset / (*ptr_to_globals).discipline_jitter) as libc::c_uint;
    /* Compute the clock jitter as the RMS of exponentially
     * weighted offset differences. Used by the poll adjust code.
     */
    etemp = SQUARE((*ptr_to_globals).discipline_jitter);
    dtemp = SQUARE(offset - (*ptr_to_globals).last_update_offset);
    (*ptr_to_globals).discipline_jitter = SQRT(etemp + (dtemp - etemp) / 4i32 as libc::c_double);
    if (*ptr_to_globals).discipline_jitter < 0.002f64 {
      (*ptr_to_globals).discipline_jitter = 0.002f64
    }
    match (*ptr_to_globals).discipline_state as libc::c_int {
      0 => {
        if option_mask32 & OPT_q as libc::c_int as libc::c_uint != 0 {
          /* We were only asked to set time once.
           * The clock is precise enough, no need to step.
           */
          exit(0i32); /* "leave poll interval as is" */
        }
        set_new_values(4i32, offset, recv_time);
        if 3i32 >= 4i32 && (*ptr_to_globals).verbose >= 4i32 as libc::c_uint {
          bb_simple_error_msg(
            b"transitioning to FREQ, datapoint ignored\x00" as *const u8 as *const libc::c_char,
          );
        }
        return 0i32;
      }
      _ => {
        /* this is dead code for now */
        set_new_values(4i32, offset, recv_time); // SQRT(SQUARE(p->filter_jitter) + SQUARE(G.cluster_jitter));
      }
    }
    if (*ptr_to_globals).stratum as libc::c_int != (*p).lastpkt_stratum as libc::c_int + 1i32 {
      (*ptr_to_globals).stratum = ((*p).lastpkt_stratum as libc::c_int + 1i32) as uint8_t;
      run_script(b"stratum\x00" as *const u8 as *const libc::c_char, offset);
    }
  }
  (*ptr_to_globals).reftime = (*ptr_to_globals).cur_time;
  (*ptr_to_globals).ntp_status = (*p).lastpkt_status;
  (*ptr_to_globals).refid = (*p).lastpkt_refid;
  (*ptr_to_globals).rootdelay = (*p).lastpkt_rootdelay + (*p).lastpkt_delay;
  dtemp = (*p).filter_jitter;
  dtemp += MAXD(
    (*p).filter_dispersion
      + 0.000015f64 * ((*ptr_to_globals).cur_time - (*p).lastpkt_recv_time)
      + abs_offset,
    0.01f64,
  );
  (*ptr_to_globals).rootdisp = (*p).lastpkt_rootdisp + dtemp;
  if 3i32 >= 4i32 && (*ptr_to_globals).verbose >= 4i32 as libc::c_uint {
    bb_error_msg(
      b"updating leap/refid/reftime/rootdisp from peer %s\x00" as *const u8 as *const libc::c_char,
      (*p).p_dotted,
    );
  }
  /* We are in STATE_SYNC now, but did not do adjtimex yet.
   * (Any other state does not reach this, they all return earlier)
   * By this time, freq_drift and offset are set
   * to values suitable for adjtimex.
   */
  if 3i32 >= 4i32 && (*ptr_to_globals).verbose >= 4i32 as libc::c_uint {
    memset(
      &mut tmx as *mut timex as *mut libc::c_void,
      0i32,
      ::std::mem::size_of::<timex>() as libc::c_ulong,
    ); // | ADJ_MAXERROR | ADJ_ESTERROR;
    if adjtimex(&mut tmx) < 0i32 {
      bb_simple_perror_msg_and_die(b"adjtimex\x00" as *const u8 as *const libc::c_char);
      /* usec */
    }
    bb_error_msg(
      b"p adjtimex freq:%ld offset:%+ld status:0x%x tc:%ld\x00" as *const u8 as *const libc::c_char,
      tmx.freq,
      tmx.offset,
      tmx.status,
      tmx.constant,
    );
  }
  memset(
    &mut tmx as *mut timex as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<timex>() as libc::c_ulong,
  );
  tmx.modes = (0x1i32 | 0x10i32 | 0x20i32) as libc::c_uint;
  tmx.offset = (offset * 1000000i32 as libc::c_double) as libc::c_long;
  if 0.5f64 < 1i32 as libc::c_double {
    if tmx.offset > (0.5f64 * 1000000i32 as libc::c_double) as libc::c_long {
      tmx.offset = (0.5f64 * 1000000i32 as libc::c_double) as libc::c_long
    }
    if tmx.offset < -((0.5f64 * 1000000i32 as libc::c_double) as libc::c_long) {
      tmx.offset = -((0.5f64 * 1000000i32 as libc::c_double) as libc::c_long)
    }
  }
  tmx.status = 0x1i32;
  if (*ptr_to_globals).FREQHOLD_cnt != 0i32 {
    /* man adjtimex on STA_FREQHOLD:
     * "Normally adjustments made via ADJ_OFFSET result in dampened
     * frequency adjustments also being made.
     * This flag prevents the small frequency adjustment from being
     * made when correcting for an ADJ_OFFSET value."
     *
     * Use this flag for a few first adjustments at the beginning
     * of ntpd execution, otherwise even relatively small initial
     * offset tend to cause largish changes to in-kernel tmx.freq.
     * If ntpd was restarted due to e.g. switch to another network,
     * this destroys already well-established tmx.freq value.
     */
    if (*ptr_to_globals).FREQHOLD_cnt < 0i32 {
      /* Initialize it */
      // Example: a laptop whose clock runs slower when hibernated,
      // after wake up it still has good tmx.freq, but accumulated ~0.5 sec offset:
      // Run with code where initial G.FREQHOLD_cnt was always 8:
      //15:17:52.947 no valid datapoints, no peer selected
      //15:17:56.515 update from:<IP> offset:+0.485133 delay:0.157762 jitter:0.209310 clock drift:-1.393ppm tc:4
      //15:17:57.719 update from:<IP> offset:+0.483825 delay:0.158070 jitter:0.181159 clock drift:-1.393ppm tc:4
      //15:17:59.925 update from:<IP> offset:+0.479504 delay:0.158147 jitter:0.156657 clock drift:-1.393ppm tc:4
      //15:18:33.322 update from:<IP> offset:+0.428119 delay:0.158317 jitter:0.138071 clock drift:-1.393ppm tc:4
      //15:19:06.718 update from:<IP> offset:+0.376932 delay:0.158276 jitter:0.122075 clock drift:-1.393ppm tc:4
      //15:19:39.114 update from:<IP> offset:+0.327022 delay:0.158384 jitter:0.108538 clock drift:-1.393ppm tc:4
      //15:20:12.715 update from:<IP> offset:+0.275596 delay:0.158297 jitter:0.097292 clock drift:-1.393ppm tc:4
      //15:20:45.111 update from:<IP> offset:+0.225715 delay:0.158271 jitter:0.087841 clock drift:-1.393ppm tc:4
      // If allowed to continue, it would start increasing tmx.freq now.
      // Instead, it was ^Ced, and started anew:
      //15:21:15.043 no valid datapoints, no peer selected
      //15:21:17.408 update from:<IP> offset:+0.175910 delay:0.158314 jitter:0.076683 clock drift:-1.393ppm tc:4
      //15:21:19.774 update from:<IP> offset:+0.171784 delay:0.158401 jitter:0.066436 clock drift:-1.393ppm tc:4
      //15:21:22.140 update from:<IP> offset:+0.171660 delay:0.158592 jitter:0.057536 clock drift:-1.393ppm tc:4
      //15:21:22.140 update from:<IP> offset:+0.167126 delay:0.158507 jitter:0.049792 clock drift:-1.393ppm tc:4
      //15:21:55.696 update from:<IP> offset:+0.115223 delay:0.158277 jitter:0.050240 clock drift:-1.393ppm tc:4
      //15:22:29.093 update from:<IP> offset:+0.068051 delay:0.158243 jitter:0.049405 clock drift:-1.393ppm tc:5
      //15:23:02.490 update from:<IP> offset:+0.051632 delay:0.158215 jitter:0.043545 clock drift:-1.393ppm tc:5
      //15:23:34.726 update from:<IP> offset:+0.039984 delay:0.158157 jitter:0.038106 clock drift:-1.393ppm tc:5
      // STA_FREQHOLD no longer set, started increasing tmx.freq now:
      //15:24:06.961 update from:<IP> offset:+0.030968 delay:0.158190 jitter:0.033306 clock drift:+2.387ppm tc:5
      //15:24:40.357 update from:<IP> offset:+0.023648 delay:0.158211 jitter:0.029072 clock drift:+5.454ppm tc:5
      //15:25:13.774 update from:<IP> offset:+0.018068 delay:0.157660 jitter:0.025288 clock drift:+7.728ppm tc:5
      //15:26:19.173 update from:<IP> offset:+0.010057 delay:0.157969 jitter:0.022255 clock drift:+8.361ppm tc:6
      //15:27:26.602 update from:<IP> offset:+0.006737 delay:0.158103 jitter:0.019316 clock drift:+8.792ppm tc:6
      //15:28:33.030 update from:<IP> offset:+0.004513 delay:0.158294 jitter:0.016765 clock drift:+9.080ppm tc:6
      //15:29:40.617 update from:<IP> offset:+0.002787 delay:0.157745 jitter:0.014543 clock drift:+9.258ppm tc:6
      //15:30:47.045 update from:<IP> offset:+0.001324 delay:0.157709 jitter:0.012594 clock drift:+9.342ppm tc:6
      //15:31:53.473 update from:<IP> offset:+0.000007 delay:0.158142 jitter:0.010922 clock drift:+9.343ppm tc:6
      //15:32:58.902 update from:<IP> offset:-0.000728 delay:0.158222 jitter:0.009454 clock drift:+9.298ppm tc:6
      /*
       * This expression would choose MIN_FREQHOLD + 14 in the above example
       * (off_032 is +1 for each 0.032768 seconds of offset).
       */
      let mut off_032: libc::c_uint = abs((tmx.offset >> 15i32) as libc::c_int) as libc::c_uint;
      (*ptr_to_globals).FREQHOLD_cnt =
        ((1i32 + 10i32) as libc::c_uint).wrapping_add(off_032) as libc::c_int
    }
    (*ptr_to_globals).FREQHOLD_cnt -= 1;
    tmx.status |= 0x80i32
  }
  if (*ptr_to_globals).ntp_status as libc::c_int & LI_PLUSSEC as libc::c_int != 0 {
    tmx.status |= 0x10i32
  }
  if (*ptr_to_globals).ntp_status as libc::c_int & LI_MINUSSEC as libc::c_int != 0 {
    tmx.status |= 0x20i32
  }
  tmx.constant = ((*ptr_to_globals).poll_exp as libc::c_int - 4i32) as __syscall_slong_t;
  /* EXPERIMENTAL.
   * The below if statement should be unnecessary, but...
   * It looks like Linux kernel's PLL is far too gentle in changing
   * tmx.freq in response to clock offset. Offset keeps growing
   * and eventually we fall back to smaller poll intervals.
   * We can make correction more aggressive (about x2) by supplying
   * PLL time constant which is one less than the real one.
   * To be on a safe side, let's do it only if offset is significantly
   * larger than jitter.
   */
  if (*ptr_to_globals).offset_to_jitter_ratio >= 2i32 as libc::c_uint {
    tmx.constant -= 1
  }
  if tmx.constant < 0i32 as libc::c_long {
    tmx.constant = 0i32 as __syscall_slong_t
  }
  //tmx.esterror = (uint32_t)(clock_jitter * 1e6);
  //tmx.maxerror = (uint32_t)((sys_rootdelay / 2 + sys_rootdisp) * 1e6);
  rc = adjtimex(&mut tmx);
  if rc < 0i32 {
    bb_simple_perror_msg_and_die(b"adjtimex\x00" as *const u8 as *const libc::c_char);
  }
  /* NB: here kernel returns constant == G.poll_exp, not == G.poll_exp - 4.
   * Not sure why. Perhaps it is normal.
   */
  if 3i32 >= 4i32 && (*ptr_to_globals).verbose >= 4i32 as libc::c_uint {
    bb_error_msg(
      b"adjtimex:%d freq:%ld offset:%+ld status:0x%x\x00" as *const u8 as *const libc::c_char,
      rc,
      tmx.freq,
      tmx.offset,
      tmx.status,
    );
  }
  (*ptr_to_globals).kernel_freq_drift = tmx.freq / 65536i32 as libc::c_long;
  if 3i32 >= 2i32 && (*ptr_to_globals).verbose >= 2i32 as libc::c_uint {
    bb_error_msg(
      b"update from:%s offset:%+f delay:%f jitter:%f clock drift:%+.3fppm tc:%d\x00" as *const u8
        as *const libc::c_char,
      (*p).p_dotted,
      offset,
      (*p).p_raw_delay,
      (*ptr_to_globals).discipline_jitter,
      tmx.freq as libc::c_double / 65536i32 as libc::c_double,
      tmx.constant as libc::c_int,
    );
  }
  return 1i32;
  /* "ok to increase poll interval" */
}
/*
 * We've got a new reply packet from a peer, process it
 * (helpers first)
 */
unsafe extern "C" fn poll_interval(mut upper_bound: libc::c_int) -> libc::c_uint {
  let mut interval: libc::c_uint = 0; /* ~ random(0..1) * interval/16 */
  let mut r: libc::c_uint = 0;
  let mut mask: libc::c_uint = 0;
  interval = (1i32 << (*ptr_to_globals).poll_exp as libc::c_int) as libc::c_uint;
  if interval > upper_bound as libc::c_uint {
    interval = upper_bound as libc::c_uint
  }
  mask = interval.wrapping_sub(1i32 as libc::c_uint) >> 4i32 | 1i32 as libc::c_uint;
  r = rand() as libc::c_uint;
  interval = interval.wrapping_add(r & mask);
  if 3i32 >= 4i32 && (*ptr_to_globals).verbose >= 4i32 as libc::c_uint {
    bb_error_msg(
      b"chose poll interval:%u (poll_exp:%d)\x00" as *const u8 as *const libc::c_char,
      interval,
      (*ptr_to_globals).poll_exp as libc::c_int,
    );
  }
  return interval;
}
unsafe extern "C" fn adjust_poll(mut count: libc::c_int) {
  (*ptr_to_globals).polladj_count += count;
  if (*ptr_to_globals).polladj_count > 40i32 {
    (*ptr_to_globals).polladj_count = 0i32;
    if ((*ptr_to_globals).poll_exp as libc::c_int) < 12i32 {
      (*ptr_to_globals).poll_exp = (*ptr_to_globals).poll_exp.wrapping_add(1);
      if 3i32 >= 4i32 && (*ptr_to_globals).verbose >= 4i32 as libc::c_uint {
        bb_error_msg(
          b"polladj: discipline_jitter:%f ++poll_exp=%d\x00" as *const u8 as *const libc::c_char,
          (*ptr_to_globals).discipline_jitter,
          (*ptr_to_globals).poll_exp as libc::c_int,
        );
      }
    }
  } else if (*ptr_to_globals).polladj_count < -40i32
    || count < 0i32 && (*ptr_to_globals).poll_exp as libc::c_int > 9i32
  {
    (*ptr_to_globals).polladj_count = 0i32;
    if (*ptr_to_globals).poll_exp as libc::c_int > 5i32 {
      let mut item: *mut llist_t = 0 as *mut llist_t;
      (*ptr_to_globals).poll_exp = (*ptr_to_globals).poll_exp.wrapping_sub(1);
      /* Correct p->next_action_time in each peer
       * which waits for sending, so that they send earlier.
       * Old pp->next_action_time are on the order
       * of t + (1 << old_poll_exp) + small_random,
       * we simply need to subtract ~half of that.
       */
      item = (*ptr_to_globals).ntp_peers;
      while !item.is_null() {
        let mut pp: *mut peer_t = (*item).data as *mut peer_t;
        if (*pp).p_fd < 0i32 {
          (*pp).next_action_time -=
            (1i32 << (*ptr_to_globals).poll_exp as libc::c_int) as libc::c_double
        }
        item = (*item).link
      }
      if 3i32 >= 4i32 && (*ptr_to_globals).verbose >= 4i32 as libc::c_uint {
        bb_error_msg(
          b"polladj: discipline_jitter:%f --poll_exp=%d\x00" as *const u8 as *const libc::c_char,
          (*ptr_to_globals).discipline_jitter,
          (*ptr_to_globals).poll_exp as libc::c_int,
        );
      }
    }
  } else if 3i32 >= 4i32 && (*ptr_to_globals).verbose >= 4i32 as libc::c_uint {
    bb_error_msg(
      b"polladj: count:%d\x00" as *const u8 as *const libc::c_char,
      (*ptr_to_globals).polladj_count,
    );
  };
}
#[inline(never)]
unsafe extern "C" fn recv_and_process_peer_pkt(mut p: *mut peer_t) {
  let mut current_block: u64;
  let mut rc: libc::c_int = 0;
  let mut size: ssize_t = 0;
  let mut msg: msg_t = msg_t {
    m_status: 0,
    m_stratum: 0,
    m_ppoll: 0,
    m_precision_exp: 0,
    m_rootdelay: s_fixedpt_t {
      int_parts: 0,
      fractions: 0,
    },
    m_rootdisp: s_fixedpt_t {
      int_parts: 0,
      fractions: 0,
    },
    m_refid: 0,
    m_reftime: l_fixedpt_t {
      int_partl: 0,
      fractionl: 0,
    },
    m_orgtime: l_fixedpt_t {
      int_partl: 0,
      fractionl: 0,
    },
    m_rectime: l_fixedpt_t {
      int_partl: 0,
      fractionl: 0,
    },
    m_xmttime: l_fixedpt_t {
      int_partl: 0,
      fractionl: 0,
    },
    m_keyid: 0,
    m_digest: [0; 20],
  };
  let mut T1: libc::c_double = 0.;
  let mut T2: libc::c_double = 0.;
  let mut T3: libc::c_double = 0.;
  let mut T4: libc::c_double = 0.;
  let mut offset: libc::c_double = 0.;
  let mut prev_delay: libc::c_double = 0.;
  let mut delay: libc::c_double = 0.;
  let mut interval: libc::c_uint = 0;
  let mut datapoint: *mut datapoint_t = 0 as *mut datapoint_t;
  let mut q: *mut peer_t = 0 as *mut peer_t;
  offset = 0i32 as libc::c_double;
  /* We can recvfrom here and check from.IP, but some multihomed
   * ntp servers reply from their *other IP*.
   * TODO: maybe we should check at least what we can: from.port == 123?
   */
  loop
  /* Signal caught */
  {
    size = recv(
      (*p).p_fd,
      &mut msg as *mut msg_t as *mut libc::c_void,
      ::std::mem::size_of::<msg_t>() as libc::c_ulong,
      MSG_DONTWAIT as libc::c_int,
    );
    if size < 0i32 as libc::c_long {
      if *bb_errno == 4i32 {
        continue;
      }
      if *bb_errno == 11i32 {
        /* There was no packet after all
         * (poll() returning POLLIN for a fd
         * is not a ironclad guarantee that data is there)
         */
        return;
      }
      /*
       * If you need a different handling for a specific
       * errno, always explain it in comment.
       */
      bb_perror_msg_and_die(
        b"recv(%s) error\x00" as *const u8 as *const libc::c_char,
        (*p).p_dotted,
      );
    } else {
      if size != NTP_MSGSIZE_NOAUTH as libc::c_int as libc::c_long
        && size != NTP_MSGSIZE_MD5_AUTH as libc::c_int as libc::c_long
        && size != NTP_MSGSIZE_SHA1_AUTH as libc::c_int as libc::c_long
      {
        bb_error_msg(
          b"malformed packet received from %s: size %u\x00" as *const u8 as *const libc::c_char,
          (*p).p_dotted,
          size as libc::c_int,
        );
        return;
      }
      if !(*p).key_entry.is_null() && hashes_differ(p, &mut msg) != 0 {
        bb_error_msg(
          b"invalid cryptographic hash received from %s\x00" as *const u8 as *const libc::c_char,
          (*p).p_dotted,
        );
        return;
      }
      if msg.m_orgtime.int_partl != (*p).p_xmt_msg.m_xmttime.int_partl
        || msg.m_orgtime.fractionl != (*p).p_xmt_msg.m_xmttime.fractionl
      {
        /* Somebody else's packet */
        return;
      }
      /* We do not expect any more packets from this peer for now.
       * Closing the socket informs kernel about it.
       * We open a new socket when we send a new query.
       */
      close((*p).p_fd);
      (*p).p_fd = -1i32;
      if msg.m_status as libc::c_int & LI_ALARM as libc::c_int == LI_ALARM as libc::c_int
        || msg.m_stratum as libc::c_int == 0i32
        || msg.m_stratum as libc::c_int > NTP_MAXSTRATUM as libc::c_int
      {
        current_block = 5634871135123216486;
        break;
      } else {
        current_block = 2232869372362427478;
        break;
      }
    }
  }
  match current_block {
    5634871135123216486 => {
      bb_error_msg(
        b"reply from %s: peer is unsynced\x00" as *const u8 as *const libc::c_char,
        (*p).p_dotted,
      );
      /*
       * Stratum 0 responses may have commands in 32-bit m_refid field:
       * "DENY", "RSTR" - peer does not like us at all,
       * "RATE" - peer is overloaded, reduce polling freq.
       * If poll interval is small, increase it.
       */
      if ((*ptr_to_globals).poll_exp as libc::c_int) < 9i32 {
        current_block = 8536721194329258740;
      } else {
        current_block = 7833897186683197378;
      }
    }
    _ => {
      //	/* Verify valid root distance */
      //	if (msg.m_rootdelay / 2 + msg.m_rootdisp >= MAXDISP || p->lastpkt_reftime > msg.m_xmt)
      //		return;                 /* invalid header values */
      /*
       * From RFC 2030 (with a correction to the delay math):
       *
       * Timestamp Name          ID   When Generated
       * ------------------------------------------------------------
       * Originate Timestamp     T1   time request sent by client
       * Receive Timestamp       T2   time request received by server
       * Transmit Timestamp      T3   time reply sent by server
       * Destination Timestamp   T4   time reply received by client
       *
       * The roundtrip delay and local clock offset are defined as
       *
       * delay = (T4 - T1) - (T3 - T2); offset = ((T2 - T1) + (T3 - T4)) / 2
       */
      T1 = (*p).p_xmttime;
      T2 = lfp_to_d(msg.m_rectime);
      T3 = lfp_to_d(msg.m_xmttime);
      T4 = (*ptr_to_globals).cur_time;
      delay = T4 - T1 - (T3 - T2);
      /*
       * If this packet's delay is much bigger than the last one,
       * it's better to just ignore it than use its much less precise value.
       */
      prev_delay = (*p).p_raw_delay;
      (*p).p_raw_delay = if delay < 0i32 as libc::c_double {
        0.0f64
      } else {
        delay
      };
      if (*p).reachable_bits as libc::c_int != 0
        && delay > prev_delay * 4i32 as libc::c_double
        && delay > 1.0f64 / (8i32 * 1024i32) as libc::c_double
      {
        /* larger than ~0.000122 */
        bb_error_msg(
          b"reply from %s: delay %f is too high, ignoring\x00" as *const u8 as *const libc::c_char,
          (*p).p_dotted,
          delay,
        );
        current_block = 7833897186683197378;
      } else {
        /* The delay calculation is a special case. In cases where the
         * server and client clocks are running at different rates and
         * with very fast networks, the delay can appear negative. In
         * order to avoid violating the Principle of Least Astonishment,
         * the delay is clamped not less than the system precision.
         */
        if delay < 0.002f64 {
          delay = 0.002f64
        }
        (*p).lastpkt_delay = delay;
        (*p).lastpkt_recv_time = T4;
        if 3i32 >= 6i32 && (*ptr_to_globals).verbose >= 6i32 as libc::c_uint {
          bb_error_msg(
            b"%s->lastpkt_recv_time=%f\x00" as *const u8 as *const libc::c_char,
            (*p).p_dotted,
            (*p).lastpkt_recv_time,
          );
        }
        (*p).lastpkt_status = msg.m_status;
        (*p).lastpkt_stratum = msg.m_stratum;
        (*p).lastpkt_rootdelay = sfp_to_d(msg.m_rootdelay);
        (*p).lastpkt_rootdisp = sfp_to_d(msg.m_rootdisp);
        (*p).lastpkt_refid = msg.m_refid;
        (*p).datapoint_idx = if (*p).reachable_bits as libc::c_int != 0 {
          ((*p).datapoint_idx + 1i32) % 8i32
        } else {
          0i32
        };
        datapoint = &mut *(*p)
          .filter_datapoint
          .as_mut_ptr()
          .offset((*p).datapoint_idx as isize) as *mut datapoint_t;
        (*datapoint).d_recv_time = T4;
        offset = (T2 - T1 + (T3 - T4)) / 2i32 as libc::c_double;
        (*datapoint).d_offset = offset;
        (*datapoint).d_dispersion = LOG2D(msg.m_precision_exp as libc::c_int) + 0.002f64;
        if (*p).reachable_bits == 0 {
          /* 1st datapoint ever - replicate offset in every element */
          let mut i: libc::c_int = 0;
          i = 0i32;
          while i < 8i32 {
            (*p).filter_datapoint[i as usize].d_offset = offset;
            i += 1
          }
        }
        (*p).reachable_bits = ((*p).reachable_bits as libc::c_int | 1i32) as uint8_t;
        if 3i32 != 0 && (*ptr_to_globals).verbose != 0
          || option_mask32 & OPT_w as libc::c_int as libc::c_uint != 0
        {
          bb_info_msg(b"reply from %s: offset:%+f delay:%f status:0x%02x strat:%d refid:0x%08x rootdelay:%f reach:0x%02x\x00"
                                    as *const u8 as *const libc::c_char,
                                (*p).p_dotted, offset, (*p).p_raw_delay,
                                (*p).lastpkt_status as libc::c_int,
                                (*p).lastpkt_stratum as libc::c_int,
                                (*p).lastpkt_refid, (*p).lastpkt_rootdelay,
                                (*p).reachable_bits as libc::c_int);
        }
        /* Muck with statictics and update the clock */
        filter_datapoints(p);
        q = select_and_cluster();
        rc = 0i32;
        if !q.is_null() {
          if option_mask32 & OPT_w as libc::c_int as libc::c_uint == 0 {
            rc = update_local_clock(q)
          }
          current_block = 15970011996474399071;
        } else if ((*ptr_to_globals).poll_exp as libc::c_int) < 9i32 {
          current_block = 8536721194329258740;
        } else {
          current_block = 15970011996474399071;
        }
        match current_block {
          8536721194329258740 => {}
          _ => {
            if rc != 0i32 {
              /* No peer selected.
               * If poll interval is small, increase it.
               */
              /* Adjust the poll interval by comparing the current offset
               * with the clock jitter. If the offset is less than
               * the clock jitter times a constant, then the averaging interval
               * is increased, otherwise it is decreased. A bit of hysteresis
               * helps calm the dance. Works best using burst mode.
               */
              if rc > 0i32 && (*ptr_to_globals).offset_to_jitter_ratio <= 4i32 as libc::c_uint {
                current_block = 8536721194329258740;
              } else {
                if 3i32 >= 3i32 && (*ptr_to_globals).verbose >= 3i32 as libc::c_uint {
                  if rc > 0i32 {
                    bb_error_msg(
                      b"want smaller interval: offset/jitter = %u\x00" as *const u8
                        as *const libc::c_char,
                      (*ptr_to_globals).offset_to_jitter_ratio,
                    );
                  }
                }
                adjust_poll(-((*ptr_to_globals).poll_exp as libc::c_int) * 2i32);
                current_block = 7833897186683197378;
              }
            } else {
              current_block = 7833897186683197378;
            }
          }
        }
      }
    }
  }
  match current_block {
    8536721194329258740 =>
    /* was += G.poll_exp but it is a bit
     * too optimistic for my taste at high poll_exp's */
    {
      adjust_poll(5i32);
    }
    _ => {}
  }
  /* Decide when to send new query for this peer */
  interval = poll_interval(2147483647i32);
  if fabs(offset) >= 1i32 as libc::c_double && interval > (1i32 << 7i32) as libc::c_uint {
    /* If we are synced, offsets are less than SLEW_THRESHOLD,
     * or at the very least not much larger than it.
     * Now we see a largish one.
     * Either this peer is feeling bad, or packet got corrupted,
     * or _our_ clock is wrong now and _all_ peers will show similar
     * largish offsets too.
     * I observed this with laptop suspend stopping clock.
     * In any case, it makes sense to make next request soonish:
     * cases 1 and 2: get a better datapoint,
     * case 3: allows to resync faster.
     */
    interval = (1i32 << 7i32) as libc::c_uint
  }
  set_next(p, interval);
}
#[inline(never)]
unsafe extern "C" fn recv_and_process_client_pkt()
/*int fd*/
{
  let mut size: ssize_t = 0;
  //uint8_t          version;
  let mut to: *mut len_and_sockaddr = 0 as *mut len_and_sockaddr;
  let mut from: *mut sockaddr = 0 as *mut sockaddr;
  let mut msg: msg_t = msg_t {
    m_status: 0,
    m_stratum: 0,
    m_ppoll: 0,
    m_precision_exp: 0,
    m_rootdelay: s_fixedpt_t {
      int_parts: 0,
      fractions: 0,
    },
    m_rootdisp: s_fixedpt_t {
      int_parts: 0,
      fractions: 0,
    },
    m_refid: 0,
    m_reftime: l_fixedpt_t {
      int_partl: 0,
      fractionl: 0,
    },
    m_orgtime: l_fixedpt_t {
      int_partl: 0,
      fractionl: 0,
    },
    m_rectime: l_fixedpt_t {
      int_partl: 0,
      fractionl: 0,
    },
    m_xmttime: l_fixedpt_t {
      int_partl: 0,
      fractionl: 0,
    },
    m_keyid: 0,
    m_digest: [0; 20],
  };
  let mut query_status: uint8_t = 0;
  let mut query_xmttime: l_fixedpt_t = l_fixedpt_t {
    int_partl: 0,
    fractionl: 0,
  };
  to = get_sock_lsa((*ptr_to_globals).listen_fd);
  from = xzalloc((*to).len as size_t) as *mut sockaddr;
  size = recv_from_to(
    (*ptr_to_globals).listen_fd,
    &mut msg as *mut msg_t as *mut libc::c_void,
    ::std::mem::size_of::<msg_t>() as libc::c_ulong,
    MSG_DONTWAIT as libc::c_int,
    from,
    &mut (*to).u.sa,
    (*to).len,
  );
  /* "ntpq -p" (4.2.8p13) sends a 12-byte NTPv2 request:
   * m_status is 0x16: leap:0 version:2 mode:6(reserved1)
   *  https://docs.ntpsec.org/latest/mode6.html
   * We don't support this.
   */
  if size != NTP_MSGSIZE_NOAUTH as libc::c_int as libc::c_long
    && size != NTP_MSGSIZE_MD5_AUTH as libc::c_int as libc::c_long
    && size != NTP_MSGSIZE_SHA1_AUTH as libc::c_int as libc::c_long
  {
    let mut addr: *mut libc::c_char = 0 as *mut libc::c_char;
    if size < 0i32 as libc::c_long {
      if !(*bb_errno == 11i32) {
        bb_simple_perror_msg_and_die(b"recv\x00" as *const u8 as *const libc::c_char);
      }
    } else {
      addr = xmalloc_sockaddr2dotted_noport(from);
      bb_error_msg(
        b"malformed packet received from %s: size %u\x00" as *const u8 as *const libc::c_char,
        addr,
        size as libc::c_int,
      );
      free(addr as *mut libc::c_void);
    }
  } else if !(msg.m_status as libc::c_int & MODE_MASK as libc::c_int != MODE_CLIENT as libc::c_int
    && msg.m_status as libc::c_int & MODE_MASK as libc::c_int != MODE_SYM_ACT as libc::c_int)
  {
    query_status = msg.m_status;
    query_xmttime = msg.m_xmttime;
    /* Respond only to client and symmetric active packets */
    /* Build a reply packet */
    memset(
      &mut msg as *mut msg_t as *mut libc::c_void,
      0i32,
      ::std::mem::size_of::<msg_t>() as libc::c_ulong,
    );
    msg.m_status = if ((*ptr_to_globals).stratum as libc::c_int) < 16i32 {
      ((*ptr_to_globals).ntp_status as libc::c_int) & LI_MASK as libc::c_int
    } else {
      LI_ALARM as libc::c_int
    } as uint8_t;
    msg.m_status = (msg.m_status as libc::c_int
      | query_status as libc::c_int & VERSION_MASK as libc::c_int) as uint8_t;
    msg.m_status = (msg.m_status as libc::c_int
      | if query_status as libc::c_int & MODE_MASK as libc::c_int == MODE_CLIENT as libc::c_int {
        MODE_SERVER as libc::c_int
      } else {
        MODE_SYM_PAS as libc::c_int
      }) as uint8_t;
    msg.m_stratum = (*ptr_to_globals).stratum;
    msg.m_ppoll = (*ptr_to_globals).poll_exp;
    msg.m_precision_exp = -9i32 as int8_t;
    /* this time was obtained between poll() and recv() */
    msg.m_rectime = d_to_lfp((*ptr_to_globals).cur_time); /* this instant */
    msg.m_xmttime = d_to_lfp(gettime1900d());
    if (*ptr_to_globals).peer_cnt == 0i32 as libc::c_uint {
      /* we have no peers: "stratum 1 server" mode. reftime = our own time */
      (*ptr_to_globals).reftime = (*ptr_to_globals).cur_time
    }
    msg.m_reftime = d_to_lfp((*ptr_to_globals).reftime);
    msg.m_orgtime = query_xmttime;
    msg.m_rootdelay = d_to_sfp((*ptr_to_globals).rootdelay);
    //simple code does not do this, fix simple code!
    msg.m_rootdisp = d_to_sfp((*ptr_to_globals).rootdisp);
    //version = (query_status & VERSION_MASK); /* ... >> VERSION_SHIFT - done below instead */
    msg.m_refid = (*ptr_to_globals).refid; // (version > (3 << VERSION_SHIFT)) ? G.refid : G.refid3;
                                           /* We reply from the local address packet was sent to,
                                            * this makes to/from look swapped here: */
    do_sendto(
      (*ptr_to_globals).listen_fd,
      &mut (*to).u.sa,
      from,
      (*to).len,
      &mut msg,
      size,
    );
  }
  free(to as *mut libc::c_void);
  free(from as *mut libc::c_void);
}
/* Upstream ntpd's options:
 *
 * -4   Force DNS resolution of host names to the IPv4 namespace.
 * -6   Force DNS resolution of host names to the IPv6 namespace.
 * -a   Require cryptographic authentication for broadcast client,
 *      multicast client and symmetric passive associations.
 *      This is the default.
 * -A   Do not require cryptographic authentication for broadcast client,
 *      multicast client and symmetric passive associations.
 *      This is almost never a good idea.
 * -b   Enable the client to synchronize to broadcast servers.
 * -c conffile
 *      Specify the name and path of the configuration file,
 *      default /etc/ntp.conf
 * -d   Specify debugging mode. This option may occur more than once,
 *      with each occurrence indicating greater detail of display.
 * -D level
 *      Specify debugging level directly.
 * -f driftfile
 *      Specify the name and path of the frequency file.
 *      This is the same operation as the "driftfile FILE"
 *      configuration command.
 * -g   Normally, ntpd exits with a message to the system log
 *      if the offset exceeds the panic threshold, which is 1000 s
 *      by default. This option allows the time to be set to any value
 *      without restriction; however, this can happen only once.
 *      If the threshold is exceeded after that, ntpd will exit
 *      with a message to the system log. This option can be used
 *      with the -q and -x options. See the tinker command for other options.
 * -i jaildir
 *      Chroot the server to the directory jaildir. This option also implies
 *      that the server attempts to drop root privileges at startup
 *      (otherwise, chroot gives very little additional security).
 *      You may need to also specify a -u option.
 * -k keyfile
 *      Specify the name and path of the symmetric key file,
 *      default /etc/ntp/keys. This is the same operation
 *      as the "keys FILE" configuration command.
 * -l logfile
 *      Specify the name and path of the log file. The default
 *      is the system log file. This is the same operation as
 *      the "logfile FILE" configuration command.
 * -L   Do not listen to virtual IPs. The default is to listen.
 * -n   Don't fork.
 * -N   To the extent permitted by the operating system,
 *      run the ntpd at the highest priority.
 * -p pidfile
 *      Specify the name and path of the file used to record the ntpd
 *      process ID. This is the same operation as the "pidfile FILE"
 *      configuration command.
 * -P priority
 *      To the extent permitted by the operating system,
 *      run the ntpd at the specified priority.
 * -q   Exit the ntpd just after the first time the clock is set.
 *      This behavior mimics that of the ntpdate program, which is
 *      to be retired. The -g and -x options can be used with this option.
 *      Note: The kernel time discipline is disabled with this option.
 * -r broadcastdelay
 *      Specify the default propagation delay from the broadcast/multicast
 *      server to this client. This is necessary only if the delay
 *      cannot be computed automatically by the protocol.
 * -s statsdir
 *      Specify the directory path for files created by the statistics
 *      facility. This is the same operation as the "statsdir DIR"
 *      configuration command.
 * -t key
 *      Add a key number to the trusted key list. This option can occur
 *      more than once.
 * -u user[:group]
 *      Specify a user, and optionally a group, to switch to.
 * -v variable
 * -V variable
 *      Add a system variable listed by default.
 * -x   Normally, the time is slewed if the offset is less than the step
 *      threshold, which is 128 ms by default, and stepped if above
 *      the threshold. This option sets the threshold to 600 s, which is
 *      well within the accuracy window to set the clock manually.
 *      Note: since the slew rate of typical Unix kernels is limited
 *      to 0.5 ms/s, each second of adjustment requires an amortization
 *      interval of 2000 s. Thus, an adjustment as much as 600 s
 *      will take almost 14 days to complete. This option can be used
 *      with the -g and -q options. See the tinker command for other options.
 *      Note: The kernel time discipline is disabled with this option.
 */
unsafe extern "C" fn find_key_entry(
  mut key_entries: *mut llist_t,
  mut id: libc::c_uint,
) -> *mut key_entry_t {
  while !key_entries.is_null() {
    let mut cur: *mut key_entry_t = (*key_entries).data as *mut key_entry_t;
    if (*cur).id == id {
      return cur;
    }
    key_entries = (*key_entries).link
  }
  bb_error_msg_and_die(
    b"key %u is not defined\x00" as *const u8 as *const libc::c_char,
    id,
  );
}
/* By doing init in a separate function we decrease stack usage
 * in main loop.
 */
#[inline(never)]
unsafe extern "C" fn ntp_init(mut argv: *mut *mut libc::c_char) {
  let mut current_block: u64;
  let mut opts: libc::c_uint = 0;
  let mut peers: *mut llist_t = 0 as *mut llist_t;
  let mut key_entries: *mut llist_t = 0 as *mut llist_t;
  let mut key_file_path: *mut libc::c_char = 0 as *mut libc::c_char;
  srand(getpid() as libc::c_uint);
  if getuid() != 0 {
    bb_simple_error_msg_and_die(bb_msg_you_must_be_root.as_ptr());
  }
  /* Set some globals */
  (*ptr_to_globals).discipline_jitter = 0.002f64; /* speeds up initial sync */
  (*ptr_to_globals).stratum = 16i32 as uint8_t; /* sets G.cur_time too */
  if 0i32 != 0i32 {
    (*ptr_to_globals).poll_exp = 0i32 as uint8_t
  }
  (*ptr_to_globals).last_update_recv_time = gettime1900d();
  (*ptr_to_globals).reftime = (*ptr_to_globals).last_update_recv_time;
  (*ptr_to_globals).last_script_run = (*ptr_to_globals).reftime;
  (*ptr_to_globals).FREQHOLD_cnt = -1i32;
  /* Parse options */
  peers = 0 as *mut llist_t;
  key_entries = 0 as *mut llist_t;
  opts = getopt32(
    argv,
    b"^nqNxk:wp:*S:lI:d46aAbgL\x00=0:dd:wn:Il\x00" as *const u8 as *const libc::c_char,
    &mut key_file_path as *mut *mut libc::c_char,
    &mut peers as *mut *mut llist_t,
    &mut (*ptr_to_globals).script_name as *mut *mut libc::c_char,
    &mut (*ptr_to_globals).if_name as *mut *mut libc::c_char,
    &mut (*ptr_to_globals).verbose as *mut libc::c_uint,
  );
  //	if (opts & OPT_x) /* disable stepping, only slew is allowed */
  //		G.time_was_stepped = 1;
  (*ptr_to_globals).listen_fd = -1i32;
  if opts & OPT_l as libc::c_int as libc::c_uint != 0 {
    (*ptr_to_globals).listen_fd = create_and_bind_dgram_or_die(0 as *const libc::c_char, 123i32);
    if !(*ptr_to_globals).if_name.is_null() {
      if setsockopt_bindtodevice((*ptr_to_globals).listen_fd, (*ptr_to_globals).if_name) != 0 {
        xfunc_die();
      }
    }
    socket_want_pktinfo((*ptr_to_globals).listen_fd);
    setsockopt_int(
      (*ptr_to_globals).listen_fd,
      IPPROTO_IP as libc::c_int,
      1i32,
      0x48i32,
    );
  }
  /* I hesitate to set -20 prio. -15 should be high enough for timekeeping */
  if opts & OPT_N as libc::c_int as libc::c_uint != 0 {
    setpriority(PRIO_PROCESS, 0i32 as id_t, -15i32);
  }
  if opts & OPT_n as libc::c_int as libc::c_uint == 0 {
    bb_daemonize_or_rexec(DAEMON_DEVNULL_STDIO as libc::c_int);
    logmode = LOGMODE_NONE as libc::c_int as smallint
  }
  if opts & OPT_k as libc::c_int as libc::c_uint != 0 {
    let mut tokens: [*mut libc::c_char; 4] = [0 as *mut libc::c_char; 4];
    let mut parser: *mut parser_t = 0 as *mut parser_t;
    parser = config_open(key_file_path);
    while config_read(
      parser,
      tokens.as_mut_ptr(),
      (PARSE_NORMAL as libc::c_int
        | PARSE_MIN_DIE as libc::c_int
        | (3i32 & 0xffi32) << 8i32
        | 4i32 & 0xffi32) as libc::c_uint,
      b"# \t\x00" as *const u8 as *const libc::c_char,
    ) == 3i32
    {
      let mut key_entry: *mut key_entry_t = 0 as *mut key_entry_t;
      let mut buffer: [libc::c_char; 40] = [0; 40];
      let mut hash_type: smalluint = 0;
      let mut msg_size: smalluint = 0;
      let mut key_length: smalluint = 0;
      let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
      if *tokens[1].offset(0) as libc::c_int | 0x20i32 == 'm' as i32 {
        /* supports 'M' and 'md5' formats */
        hash_type = HASH_MD5 as libc::c_int as smalluint
      } else if strncasecmp(
        tokens[1],
        b"sha\x00" as *const u8 as *const libc::c_char,
        3i32 as libc::c_ulong,
      ) == 0i32
      {
        /* supports 'sha' and 'sha1' formats */
        hash_type = HASH_SHA1 as libc::c_int as smalluint
      } else {
        bb_simple_error_msg_and_die(
          b"only MD5 and SHA1 keys supported\x00" as *const u8 as *const libc::c_char,
        );
      }
      /* man ntp.keys:
       *  MD5    The key is 1 to 16 printable characters terminated by an EOL,
       *         whitespace, or a # (which is the "start of comment" character).
       *  SHA
       *  SHA1
       *  RMD160 The key is a hex-encoded ASCII string of 40 characters, which
       *         is truncated as necessary.
       */
      key_length = strnlen(
        tokens[2],
        (::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong)
          .wrapping_add(1i32 as libc::c_ulong),
      ) as smalluint; /* it's hash_type == HASH_SHA1 */
      if !(key_length as libc::c_ulong
        >= (::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong)
          .wrapping_add(1i32 as libc::c_ulong))
      {
        if hash_type as libc::c_int == HASH_MD5 as libc::c_int {
          key = tokens[2];
          msg_size = NTP_MSGSIZE_MD5_AUTH as libc::c_int as smalluint;
          current_block = 14945149239039849694;
        } else if key_length as libc::c_int & 1i32 == 0 {
          key_length = (key_length as libc::c_int >> 1i32) as smalluint;
          if hex2bin(buffer.as_mut_ptr(), tokens[2], key_length as libc::c_int).is_null() {
            current_block = 8105424810531593847;
          } else {
            key = buffer.as_mut_ptr();
            msg_size = NTP_MSGSIZE_SHA1_AUTH as libc::c_int as smalluint;
            current_block = 14945149239039849694;
          }
        } else {
          current_block = 8105424810531593847;
        }
        match current_block {
          8105424810531593847 => {}
          _ => {
            key_entry = xzalloc(
              (::std::mem::size_of::<key_entry_t>() as libc::c_ulong)
                .wrapping_add(key_length as libc::c_ulong),
            ) as *mut key_entry_t;
            (*key_entry).type_0 = hash_type;
            (*key_entry).msg_size = msg_size;
            (*key_entry).key_length = key_length;
            memcpy(
              (*key_entry).key.as_mut_ptr() as *mut libc::c_void,
              key as *const libc::c_void,
              key_length as libc::c_ulong,
            );
            (*key_entry).id =
              xatou_range(tokens[0], 1i32 as libc::c_uint, 65535i32 as libc::c_uint);
            llist_add_to(&mut key_entries, key_entry as *mut libc::c_void);
            continue;
          }
        }
      }
      bb_error_msg_and_die(
        b"malformed key at line %u\x00" as *const u8 as *const libc::c_char,
        (*parser).lineno,
      );
    }
    config_close(parser);
  }
  if !peers.is_null() {
    while !peers.is_null() {
      let mut peer: *mut libc::c_char = llist_pop(&mut peers) as *mut libc::c_char;
      let mut key_entry_0: *mut key_entry_t = 0 as *mut key_entry_t;
      if strncmp(
        peer,
        b"keyno:\x00" as *const u8 as *const libc::c_char,
        6i32 as libc::c_ulong,
      ) == 0i32
      {
        let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut key_id: libc::c_int = 0;
        peer = peer.offset(6);
        end = strchr(peer, ':' as i32);
        if end.is_null() {
          bb_show_usage();
        }
        *end = '\u{0}' as i32 as libc::c_char;
        key_id = xatou_range(peer, 1i32 as libc::c_uint, 65535i32 as libc::c_uint) as libc::c_int;
        *end = ':' as i32 as libc::c_char;
        key_entry_0 = find_key_entry(key_entries, key_id as libc::c_uint);
        peer = end.offset(1)
      }
      add_peers(peer, key_entry_0);
    }
  } else {
    let mut parser_0: *mut parser_t = 0 as *mut parser_t;
    let mut token: [*mut libc::c_char; 5] = [0 as *mut libc::c_char; 5];
    parser_0 = config_open(b"/etc/ntp.conf\x00" as *const u8 as *const libc::c_char);
    while config_read(
      parser_0,
      token.as_mut_ptr(),
      (PARSE_NORMAL as libc::c_int | (1i32 & 0xffi32) << 8i32 | 3i32 + 2i32 * 1i32 & 0xffi32)
        as libc::c_uint,
      b"# \t\x00" as *const u8 as *const libc::c_char,
    ) != 0
    {
      if strcmp(token[0], b"server\x00" as *const u8 as *const libc::c_char) == 0i32
        && !token[1].is_null()
      {
        let mut key_entry_1: *mut key_entry_t = 0 as *mut key_entry_t;
        if !token[2].is_null()
          && !token[3].is_null()
          && strcmp(token[2], b"key\x00" as *const u8 as *const libc::c_char) == 0i32
        {
          let mut key_id_0: libc::c_uint =
            xatou_range(token[3], 1i32 as libc::c_uint, 65535i32 as libc::c_uint);
          key_entry_1 = find_key_entry(key_entries, key_id_0)
        }
        add_peers(token[1], key_entry_1);
      } else {
        bb_error_msg(
          b"skipping %s:%u: unimplemented command \'%s\'\x00" as *const u8 as *const libc::c_char,
          b"/etc/ntp.conf\x00" as *const u8 as *const libc::c_char,
          (*parser_0).lineno,
          token[0],
        );
      }
    }
    config_close(parser_0);
  }
  if (*ptr_to_globals).peer_cnt == 0i32 as libc::c_uint {
    if opts & OPT_l as libc::c_int as libc::c_uint == 0 {
      bb_show_usage();
    }
    /* -l but no peers: "stratum 1 server" mode */
    (*ptr_to_globals).stratum = 1i32 as uint8_t
  }
  if opts & OPT_n as libc::c_int as libc::c_uint == 0 {
    /* only if backgrounded: */
    write_pidfile_std_path_and_ext(b"ntpd\x00" as *const u8 as *const libc::c_char);
  }
  /* If network is up, syncronization occurs in ~10 seconds.
   * We give "ntpd -q" 10 seconds to get first reply,
   * then another 50 seconds to finish syncing.
   *
   * I tested ntpd 4.2.6p1 and apparently it never exits
   * (will try forever), but it does not feel right.
   * The goal of -q is to act like ntpdate: set time
   * after a reasonably small period of polling, or fail.
   */
  if opts & OPT_q as libc::c_int as libc::c_uint != 0 {
    option_mask32 |= OPT_qq as libc::c_int as libc::c_uint;
    alarm(10i32 as libc::c_uint);
  }
  bb_signals(
    0i32 | 1i32 << 15i32 | 1i32 << 2i32 | 1i32 << 14i32,
    Some(record_signo as unsafe extern "C" fn(_: libc::c_int) -> ()),
  );
  bb_signals(
    0i32 | 1i32 << 13i32 | 1i32 << 17i32,
    ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(1i32 as libc::intptr_t),
  );
  //TODO: free unused elements of key_entries?
}
#[no_mangle]
pub unsafe extern "C" fn ntpd_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut current_block: u64;
  let mut G: globals = globals {
    cur_time: 0.,
    rootdelay: 0.,
    reftime: 0.,
    rootdisp: 0.,
    last_script_run: 0.,
    script_name: 0 as *mut libc::c_char,
    ntp_peers: 0 as *mut llist_t,
    listen_fd: 0,
    if_name: 0 as *mut libc::c_char,
    verbose: 0,
    peer_cnt: 0,
    refid: 0,
    ntp_status: 0,
    stratum: 0,
    discipline_state: 0,
    poll_exp: 0,
    polladj_count: 0,
    FREQHOLD_cnt: 0,
    kernel_freq_drift: 0,
    last_update_peer: 0 as *mut peer_t,
    last_update_offset: 0.,
    last_update_recv_time: 0.,
    discipline_jitter: 0.,
    offset_to_jitter_ratio: 0,
  };
  let mut pfd: *mut pollfd = 0 as *mut pollfd;
  let mut idx2peer: *mut *mut peer_t = 0 as *mut *mut peer_t;
  let mut cnt: libc::c_uint = 0;
  memset(
    &mut G as *mut globals as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<globals>() as libc::c_ulong,
  );
  let ref mut fresh31 =
    *(not_const_pp(&ptr_to_globals as *const *mut globals as *const libc::c_void)
      as *mut *mut globals);
  *fresh31 = &mut G as *mut globals as *mut libc::c_void as *mut globals;
  asm!("" : : : "memory" : "volatile");
  ntp_init(argv);
  /* If ENABLE_FEATURE_NTPD_SERVER, + 1 for listen_fd: */
  cnt = G.peer_cnt.wrapping_add(1i32 as libc::c_uint);
  idx2peer = xzalloc(
    (::std::mem::size_of::<*mut peer_t>() as libc::c_ulong).wrapping_mul(cnt as libc::c_ulong),
  ) as *mut *mut peer_t;
  pfd =
    xzalloc((::std::mem::size_of::<pollfd>() as libc::c_ulong).wrapping_mul(cnt as libc::c_ulong))
      as *mut pollfd;
  /* Countdown: we never sync before we sent INITIAL_SAMPLES+1
   * packets to each peer.
   * NB: if some peer is not responding, we may end up sending
   * fewer packets to it and more to other peers.
   * NB2: sync usually happens using INITIAL_SAMPLES packets,
   * since last reply does not come back instantaneously.
   */
  cnt = G.peer_cnt.wrapping_mul((4i32 + 1i32) as libc::c_uint); /* while (!bb_got_signal) */
  while bb_got_signal == 0 {
    let mut item: *mut llist_t = 0 as *mut llist_t;
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut nfds: libc::c_int = 0;
    let mut timeout: libc::c_int = 0;
    let mut nextaction: libc::c_double = 0.;
    /* Nothing between here and poll() blocks for any significant time */
    nextaction = G.last_script_run + (11i32 * 60i32) as libc::c_double;
    if nextaction < G.cur_time + 1i32 as libc::c_double {
      nextaction = G.cur_time + 1i32 as libc::c_double
    }
    i = 0i32 as libc::c_uint;
    if G.listen_fd != -1i32 {
      (*pfd.offset(0)).fd = G.listen_fd;
      (*pfd.offset(0)).events = 0x1i32 as libc::c_short;
      i = i.wrapping_add(1)
    }
    /* Pass over peer list, send requests, time out on receives */
    item = G.ntp_peers;
    while !item.is_null() {
      let mut p: *mut peer_t = (*item).data as *mut peer_t;
      if (*p).next_action_time <= G.cur_time {
        if (*p).p_fd == -1i32 {
          /* Time to send new req */
          cnt = cnt.wrapping_sub(1);
          if cnt == 0i32 as libc::c_uint {
            if 3i32 >= 4i32 && G.verbose >= 4i32 as libc::c_uint {
              bb_simple_error_msg(b"disabling burst mode\x00" as *const u8 as *const libc::c_char);
            }
            G.polladj_count = 0i32;
            G.poll_exp = 5i32 as uint8_t
          }
          send_query_to_peer(p);
        } else {
          /* Timed out waiting for reply */
          close((*p).p_fd);
          (*p).p_fd = -1i32;
          /* If poll interval is small, increase it */
          if (G.poll_exp as libc::c_int) < 9i32 {
            adjust_poll(5i32);
          }
          timeout = poll_interval(512i32) as libc::c_int;
          bb_error_msg(
            b"timed out waiting for %s, reach 0x%02x, next query in %us\x00" as *const u8
              as *const libc::c_char,
            (*p).p_dotted,
            (*p).reachable_bits as libc::c_int,
            timeout,
          );
          /* What if don't see it because it changed its IP? */
          if (*p).reachable_bits as libc::c_int == 0i32 {
            resolve_peer_hostname(p);
          }
          set_next(p, timeout as libc::c_uint);
        }
      }
      if (*p).next_action_time < nextaction {
        nextaction = (*p).next_action_time
      }
      if (*p).p_fd >= 0i32 {
        /* Wait for reply from this peer */
        (*pfd.offset(i as isize)).fd = (*p).p_fd; /* (nextaction - G.cur_time) rounds down, compensating */
        (*pfd.offset(i as isize)).events = 0x1i32 as libc::c_short;
        let ref mut fresh32 = *idx2peer.offset(i as isize);
        *fresh32 = p;
        i = i.wrapping_add(1)
      }
      item = (*item).link
    }
    timeout = (nextaction - G.cur_time) as libc::c_int;
    if timeout < 0i32 {
      timeout = 0i32
    }
    timeout += 1;
    /* Here we may block */
    if 3i32 >= 2i32 && G.verbose >= 2i32 as libc::c_uint {
      if i > (1i32 != 0 && G.listen_fd != -1i32) as libc::c_int as libc::c_uint {
        /* We wait for at least one reply.
         * Poll for it, without wasting time for message.
         * Since replies often come under 1 second, this also
         * reduces clutter in logs.
         */
        nfds = poll(pfd, i as nfds_t, 1000i32); /* sets G.cur_time */
        if nfds != 0i32 {
          current_block = 4356998877126854527; /* poll was interrupted by a signal */
        } else {
          timeout -= 1;
          if timeout <= 0i32 {
            current_block = 4356998877126854527;
          } else {
            current_block = 17836213544692497527;
          }
        }
      } else {
        current_block = 17836213544692497527;
      }
      match current_block {
        4356998877126854527 => {}
        _ => {
          bb_error_msg(
            b"poll:%us sockets:%u interval:%us\x00" as *const u8 as *const libc::c_char,
            timeout,
            i,
            1i32 << G.poll_exp as libc::c_int,
          );
          current_block = 3392087639489470149;
        }
      }
    } else {
      current_block = 3392087639489470149;
    }
    match current_block {
      3392087639489470149 => nfds = poll(pfd, i as nfds_t, timeout * 1000i32),
      _ => {}
    }
    gettime1900d();
    if nfds <= 0i32 {
      let mut ct: libc::c_double = 0.;
      let mut dns_error: libc::c_int = 0;
      if bb_got_signal != 0 {
        break;
      }
      if G.cur_time - G.last_script_run > (11i32 * 60i32) as libc::c_double {
        /* Useful for updating battery-backed RTC and such */
        run_script(
          b"periodic\x00" as *const u8 as *const libc::c_char,
          G.last_update_offset,
        );
        gettime1900d();
        /* sets G.cur_time */
      }
      /* Resolve peer names to IPs, if not resolved yet.
       * We do it only when poll timed out:
       * this way, we almost never overlap DNS resolution with
       * "request-reply" packet round trip.
       */
      dns_error = 0i32;
      ct = G.cur_time;
      item = G.ntp_peers;
      while !item.is_null() {
        let mut p_0: *mut peer_t = (*item).data as *mut peer_t;
        if (*p_0).next_action_time <= ct && (*p_0).p_lsa.is_null() {
          /* This can take up to ~10 sec per each DNS query */
          dns_error |= resolve_peer_hostname(p_0).is_null() as libc::c_int
        }
        item = (*item).link
      }
      if !(dns_error == 0) {
        /* Set next time for those which are still not resolved */
        gettime1900d(); /* sets G.cur_time (needed for set_next()) */
        item = G.ntp_peers;
        while !item.is_null() {
          let mut p_1: *mut peer_t = (*item).data as *mut peer_t;
          if (*p_1).next_action_time <= ct && (*p_1).p_lsa.is_null() {
            set_next(
              p_1,
              (4i32 * (*p_1).dns_errors as libc::c_int) as libc::c_uint,
            );
          }
          item = (*item).link
        }
      }
    } else {
      /* Process any received packets */
      j = 0i32 as libc::c_uint;
      if G.listen_fd != -1i32 {
        if (*pfd.offset(0)).revents != 0 {
          /* & (POLLIN|POLLERR)*/
          nfds -= 1;
          recv_and_process_client_pkt();
          gettime1900d();
          /* sets G.cur_time */
        }
        j = 1i32 as libc::c_uint
      }
      while nfds != 0i32 && j < i {
        if (*pfd.offset(j as isize)).revents != 0 {
          /* & (POLLIN|POLLERR)*/
          /*
           * At init, alarm was set to 10 sec.
           * Now we did get a reply.
           * Increase timeout to 50 seconds to finish syncing.
           */
          if option_mask32 & OPT_qq as libc::c_int as libc::c_uint != 0 {
            option_mask32 &= !(OPT_qq as libc::c_int) as libc::c_uint;
            alarm(50i32 as libc::c_uint);
          }
          nfds -= 1;
          recv_and_process_peer_pkt(*idx2peer.offset(j as isize));
          gettime1900d();
        }
        j = j.wrapping_add(1)
      }
    }
    if !G.ntp_peers.is_null() && G.stratum as libc::c_int != 16i32 {
      let mut current_block_101: u64;
      item = G.ntp_peers;
      loop {
        if item.is_null() {
          current_block_101 = 1677945370889843322;
          break;
        }
        let mut p_2: *mut peer_t = (*item).data as *mut peer_t;
        if (*p_2).reachable_bits != 0 {
          current_block_101 = 6665878751423064961;
          break;
        }
        item = (*item).link
      }
      match current_block_101 {
        1677945370889843322 => {
          /* No peer responded for last 8 packets, panic */
          clamp_pollexp_and_set_MAXSTRAT();
          run_script(b"unsync\x00" as *const u8 as *const libc::c_char, 0.0f64);
        }
        _ => {}
      }
    }
  }
  remove_pidfile_std_path_and_ext(b"ntpd\x00" as *const u8 as *const libc::c_char);
  kill_myself_with_sig(bb_got_signal as libc::c_int);
}
/* ** ntp-4.2.6/ntpd/ntp_loopfilter.c - adjtimex usage ***/
/* ** openntpd-4.6 uses only adjtime, not adjtimex ***/
