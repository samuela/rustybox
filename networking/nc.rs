use c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
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
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  fn vfork() -> libc::c_int;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn getsockname(__fd: libc::c_int, __addr: __SOCKADDR_ARG, __len: *mut socklen_t) -> libc::c_int;
  #[no_mangle]
  fn connect(__fd: libc::c_int, __addr: __CONST_SOCKADDR_ARG, __len: socklen_t) -> libc::c_int;
  #[no_mangle]
  fn getsockopt(
    __fd: libc::c_int,
    __level: libc::c_int,
    __optname: libc::c_int,
    __optval: *mut libc::c_void,
    __optlen: *mut socklen_t,
  ) -> libc::c_int;
  #[no_mangle]
  fn accept(__fd: libc::c_int, __addr: __SOCKADDR_ARG, __addr_len: *mut socklen_t) -> libc::c_int;
  #[no_mangle]
  fn shutdown(__fd: libc::c_int, __how: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
  #[no_mangle]
  fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
  #[no_mangle]
  fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
  #[no_mangle]
  static mut stderr: *mut FILE;
  #[no_mangle]
  fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
  #[no_mangle]
  fn alarm(__seconds: libc::c_uint) -> libc::c_uint;
  #[no_mangle]
  fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
  #[no_mangle]
  fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
  #[no_mangle]
  fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: libc::c_int) -> libc::c_int;
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
  fn xmove_fd(_: libc::c_int, _: libc::c_int);
  #[no_mangle]
  fn bb_signals(sigs: libc::c_int, f: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>);
  #[no_mangle]
  fn kill_myself_with_sig(sig: libc::c_int) -> !;
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xsocket(domain: libc::c_int, type_0: libc::c_int, protocol: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xbind(sockfd: libc::c_int, my_addr: *mut sockaddr, addrlen: socklen_t);
  #[no_mangle]
  fn xlisten(s: libc::c_int, backlog: libc::c_int);
  #[no_mangle]
  fn xconnect(s: libc::c_int, s_addr: *const sockaddr, addrlen: socklen_t);
  /* SO_REUSEADDR allows a server to rebind to an address that is already
   * "in use" by old connections to e.g. previous server instance which is
   * killed or crashed. Without it bind will fail until all such connections
   * time out. Linux does not allow multiple live binds on same ip:port
   * regardless of SO_REUSEADDR (unlike some other flavors of Unix).
   * Turn it on before you call bind(). */
  #[no_mangle]
  fn setsockopt_reuseaddr(fd: libc::c_int);
  /* NB: returns port in host byte order */
  #[no_mangle]
  fn bb_lookup_port(
    port: *const libc::c_char,
    protocol: *const libc::c_char,
    default_port: libc::c_uint,
  ) -> libc::c_uint;
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
  /* Version which dies on error */
  #[no_mangle]
  fn xhost2sockaddr(host: *const libc::c_char, port: libc::c_int) -> *mut len_and_sockaddr;
  /* Assign sin[6]_port member if the socket is an AF_INET[6] one,
   * otherwise no-op. Useful for ftp.
   * NB: does NOT do htons() internally, just direct assignment. */
  #[no_mangle]
  fn set_nport(sa: *mut sockaddr, port: libc::c_uint);
  /* Retrieve sin[6]_port or return -1 for non-INET[6] lsa's */
  #[no_mangle]
  fn get_nport(sa: *const sockaddr) -> libc::c_int;
  /* Reverse DNS. Returns NULL on failure. */
  #[no_mangle]
  fn xmalloc_sockaddr2host(sa: *const sockaddr) -> *mut libc::c_char;
  /* inet_[ap]ton on steroids */
  #[no_mangle]
  fn xmalloc_sockaddr2dotted(sa: *const sockaddr) -> *mut libc::c_char;
  #[no_mangle]
  fn socket_want_pktinfo(fd: libc::c_int);
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
  fn xwrite(fd: libc::c_int, buf: *const libc::c_void, count: size_t);
  /* Put a string of hex bytes ("1b2e66fe"...), return advanced pointer */
  #[no_mangle]
  fn bin2hex(
    dst: *mut libc::c_char,
    src: *const libc::c_char,
    count: libc::c_int,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn xatou_range(str: *const libc::c_char, l: libc::c_uint, u: libc::c_uint) -> libc::c_uint;
  /* BB_EXECxx always execs (it's not doing NOFORK/NOEXEC stuff),
   * but it may exec busybox and call applet instead of searching PATH.
   */
  #[no_mangle]
  fn BB_EXECVP_or_die(argv: *mut *mut libc::c_char) -> !;
  /* { "-", NULL } */
  #[no_mangle]
  static mut option_mask32: uint32_t;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;
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
  /* NB: (bb_hexdigits_upcase[i] | 0x20) -> lowercase hex digit */
  #[no_mangle]
  static bb_hexdigits_upcase: [libc::c_char; 0];
  /* '*const' ptr makes gcc optimize code much better.
   * Magic prevents ptr_to_globals from going into rodata.
   * If you want to assign a value, use SET_PTR_TO_GLOBALS(x) */
  #[no_mangle]
  static ptr_to_globals: *mut globals;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type pid_t = __pid_t;
pub type socklen_t = __socklen_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
  pub __val: [libc::c_ulong; 16],
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const SHUT_RDWR: C2RustUnnamed_0 = 2;
pub const SHUT_WR: C2RustUnnamed_0 = 1;
pub const SHUT_RD: C2RustUnnamed_0 = 0;
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
  pub sin6_flowinfo: uint32_t,
  pub sin6_addr: in6_addr,
  pub sin6_scope_id: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
  pub __in6_u: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
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
pub type C2RustUnnamed_2 = libc::c_uint;
pub const IPPROTO_MAX: C2RustUnnamed_2 = 256;
pub const IPPROTO_RAW: C2RustUnnamed_2 = 255;
pub const IPPROTO_MPLS: C2RustUnnamed_2 = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed_2 = 136;
pub const IPPROTO_SCTP: C2RustUnnamed_2 = 132;
pub const IPPROTO_COMP: C2RustUnnamed_2 = 108;
pub const IPPROTO_PIM: C2RustUnnamed_2 = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed_2 = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed_2 = 94;
pub const IPPROTO_MTP: C2RustUnnamed_2 = 92;
pub const IPPROTO_AH: C2RustUnnamed_2 = 51;
pub const IPPROTO_ESP: C2RustUnnamed_2 = 50;
pub const IPPROTO_GRE: C2RustUnnamed_2 = 47;
pub const IPPROTO_RSVP: C2RustUnnamed_2 = 46;
pub const IPPROTO_IPV6: C2RustUnnamed_2 = 41;
pub const IPPROTO_DCCP: C2RustUnnamed_2 = 33;
pub const IPPROTO_TP: C2RustUnnamed_2 = 29;
pub const IPPROTO_IDP: C2RustUnnamed_2 = 22;
pub const IPPROTO_UDP: C2RustUnnamed_2 = 17;
pub const IPPROTO_PUP: C2RustUnnamed_2 = 12;
pub const IPPROTO_EGP: C2RustUnnamed_2 = 8;
pub const IPPROTO_TCP: C2RustUnnamed_2 = 6;
pub const IPPROTO_IPIP: C2RustUnnamed_2 = 4;
pub const IPPROTO_IGMP: C2RustUnnamed_2 = 2;
pub const IPPROTO_ICMP: C2RustUnnamed_2 = 1;
pub const IPPROTO_IP: C2RustUnnamed_2 = 0;
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
  pub __jmpbuf: __jmp_buf,
  pub __mask_was_saved: libc::c_int,
  pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
pub type __sighandler_t = Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;



use crate::librb::FILE;
pub type nfds_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
  pub fd: libc::c_int,
  pub events: libc::c_short,
  pub revents: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct len_and_sockaddr {
  pub len: socklen_t,
  pub u: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
  pub sa: sockaddr,
  pub sin: sockaddr_in,
  pub sin6: sockaddr_in6,
}
pub type C2RustUnnamed_4 = libc::c_uint;
pub const LSA_SIZEOF_SA: C2RustUnnamed_4 = 28;
pub const LSA_LEN_SIZE: C2RustUnnamed_4 = 4;
//extern const int const_int_1;
/* This struct is deliberately not defined. */
/* See docs/keep_data_small.txt */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub o_verbose: libc::c_uint,
  pub o_wait: libc::c_uint,
  pub o_interval: libc::c_uint,
  pub wrote_out: libc::c_ulonglong,
  pub wrote_net: libc::c_ulonglong,
  pub proggie0saved: *mut libc::c_char,
  pub ouraddr: *mut len_and_sockaddr,
  pub themaddr: *mut len_and_sockaddr,
  pub remend: len_and_sockaddr,
  pub jbuf: jmp_buf,
  pub bigbuf_in: [libc::c_char; 8192],
  pub bigbuf_net: [libc::c_char; 8192],
}
pub type C2RustUnnamed_5 = libc::c_uint;
pub const ofd: C2RustUnnamed_5 = 4;
pub const netfd: C2RustUnnamed_5 = 3;
pub const BIGSIZ: C2RustUnnamed_5 = 8192;
pub const SLEAZE_PORT: C2RustUnnamed_5 = 31337;
/* Must match getopt32 call! */
pub type C2RustUnnamed_6 = libc::c_uint;
pub const OPT_z: C2RustUnnamed_6 = 1024;
pub const OPT_o: C2RustUnnamed_6 = 512;
pub const OPT_i: C2RustUnnamed_6 = 256;
pub const OPT_k: C2RustUnnamed_6 = 128;
pub const OPT_l: C2RustUnnamed_6 = 64;
pub const OPT_w: C2RustUnnamed_6 = 32;
pub const OPT_v: C2RustUnnamed_6 = 16;
pub const OPT_u: C2RustUnnamed_6 = 8;
pub const OPT_s: C2RustUnnamed_6 = 4;
pub const OPT_p: C2RustUnnamed_6 = 2;
pub const OPT_n: C2RustUnnamed_6 = 1;
#[inline(always)]
unsafe extern "C" fn not_const_pp(mut p: *const libc::c_void) -> *mut libc::c_void {
  return p as *mut libc::c_void;
}
/* Debug: squirt whatever message and sleep a bit so we can see it go by. */
/* Beware: writes to stdOUT... */
/* catch: no-brainer interrupt handler */
unsafe extern "C" fn catch(mut sig: libc::c_int) {
  if (*ptr_to_globals).o_verbose > 1i32 as libc::c_uint {
    /* normally we don't care */
    fprintf(
      stderr,
      b"sent %llu, rcvd %llu\n\x00" as *const u8 as *const libc::c_char,
      (*ptr_to_globals).wrote_net,
      (*ptr_to_globals).wrote_out,
    );
  }
  fprintf(stderr, b"punt!\n\x00" as *const u8 as *const libc::c_char);
  kill_myself_with_sig(sig);
}
/* unarm  */
unsafe extern "C" fn unarm() {
  signal(
    14i32,
    ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(1i32 as libc::intptr_t),
  );
  alarm(0i32 as libc::c_uint);
}
/* timeout and other signal handling cruft */
unsafe extern "C" fn tmtravel(mut _sig: libc::c_int) {
  unarm();
  longjmp((*ptr_to_globals).jbuf.as_mut_ptr(), 1i32);
}
/* arm: set the timer.  */
unsafe extern "C" fn arm(mut secs: libc::c_uint) {
  signal(
    14i32,
    Some(tmtravel as unsafe extern "C" fn(_: libc::c_int) -> ()),
  );
  alarm(secs);
}
/* findline:
find the next newline in a buffer; return inclusive size of that "line",
or the entire buffer size, so the caller knows how much to then write().
Not distinguishing \n vs \r\n for the nonce; it just works as is... */
unsafe extern "C" fn findline(mut buf: *mut libc::c_char, mut siz: libc::c_uint) -> libc::c_uint {
  let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut x: libc::c_int = 0;
  if buf.is_null() {
    /* various sanity checks... */
    return 0i32 as libc::c_uint;
  } /* for */
  if siz > BIGSIZ as libc::c_int as libc::c_uint {
    return 0i32 as libc::c_uint;
  } /* 'sokay if it points just past the end! */
  x = siz as libc::c_int;
  p = buf;
  while x > 0i32 {
    if *p as libc::c_int == '\n' as i32 {
      x = p.wrapping_offset_from(buf) as libc::c_long as libc::c_int;
      x += 1;
      return x as libc::c_uint;
    }
    p = p.offset(1);
    x -= 1
  }
  return siz;
}
/* findline */
/* doexec:
fiddle all the file descriptors around, and hand off to another prog.  Sort
of like a one-off "poor man's inetd".  This is the only section of code
that would be security-critical, which is why it's ifdefed out by default.
Use at your own hairy risk; if you leave shells lying around behind open
listening ports you deserve to lose!! */
unsafe extern "C" fn doexec(mut proggie: *mut *mut libc::c_char) -> ! {
  if !(*ptr_to_globals).proggie0saved.is_null() {
    let ref mut fresh0 = *proggie.offset(0);
    *fresh0 = (*ptr_to_globals).proggie0saved
  }
  xmove_fd(netfd as libc::c_int, 0i32);
  dup2(0i32, 1i32);
  /* dup2(0, 2); - do we *really* want this? NO!
   * exec'ed prog can do it yourself, if needed */
  BB_EXECVP_or_die(proggie);
}
/* connect_w_timeout:
return an fd for one of
an open outbound TCP connection, a UDP stub-socket thingie, or
an unconnected TCP or UDP socket to listen on.
Examines various global o_blah flags to figure out what to do.
lad can be NULL, then socket is not bound to any local ip[:port] */
unsafe extern "C" fn connect_w_timeout(mut fd: libc::c_int) -> libc::c_int {
  let mut rr: libc::c_int = 0;
  /* wrap connect inside a timer, and hit it */
  arm((*ptr_to_globals).o_wait); /* setjmp: connect failed... */
  if _setjmp((*ptr_to_globals).jbuf.as_mut_ptr()) == 0i32 {
    rr = connect(
      fd,
      __CONST_SOCKADDR_ARG {
        __sockaddr__: &mut (*(*ptr_to_globals).themaddr).u.sa,
      },
      (*(*ptr_to_globals).themaddr).len,
    );
    unarm();
  } else {
    rr = -1i32;
    *bb_errno = 110i32
    /* fake it */
  }
  return rr;
}
/* dolisten:
listens for
incoming and returns an open connection *from* someplace.  If we were
given host/port args, any connections from elsewhere are rejected.  This
in conjunction with local-address binding should limit things nicely... */
unsafe extern "C" fn dolisten(mut is_persistent: libc::c_int, mut proggie: *mut *mut libc::c_char) {
  let mut rr: libc::c_int = 0; /* TCP: gotta listen() before we can get */
  if option_mask32 & OPT_u as libc::c_int as libc::c_uint == 0 {
    xlisten(netfd as libc::c_int, 1i32);
  }
  /* Various things that follow temporarily trash bigbuf_net, which might contain
  a copy of any recvfrom()ed packet, but we'll read() another copy later. */
  /* I can't believe I have to do all this to get my own goddamn bound address
  and port number.  It should just get filled in during bind() or something.
  All this is only useful if we didn't say -p for listening, since if we
  said -p we *know* what port we're listening on.  At any rate we won't bother
  with it all unless we wanted to see it, although listening quietly on a
  random unknown port is probably not very useful without "netstat". */
  if (*ptr_to_globals).o_verbose != 0 {
    let mut addr: *mut libc::c_char = 0 as *mut libc::c_char;
    getsockname(
      netfd as libc::c_int,
      __SOCKADDR_ARG {
        __sockaddr__: &mut (*(*ptr_to_globals).ouraddr).u.sa as *mut sockaddr,
      },
      &mut (*(*ptr_to_globals).ouraddr).len,
    );
    //if (rr < 0)
    //	bb_perror_msg_and_die("getsockname after bind");
    addr = xmalloc_sockaddr2dotted(&mut (*(*ptr_to_globals).ouraddr).u.sa);
    fprintf(
      stderr,
      b"listening on %s ...\n\x00" as *const u8 as *const libc::c_char,
      addr,
    );
    free(addr as *mut libc::c_void);
  }
  if option_mask32 & OPT_u as libc::c_int as libc::c_uint != 0 {
    /* UDP is a speeeeecial case -- we have to do I/O *and* get the calling
    party's particulars all at once, listen() and accept() don't apply.
    At least in the BSD universe, however, recvfrom/PEEK is enough to tell
    us something came in, and we can set things up so straight read/write
    actually does work after all.  Yow.  YMMV on strange platforms!  */
    /* I'm not completely clear on how this works -- BSD seems to make UDP
    just magically work in a connect()ed context, but we'll undoubtedly run
    into systems this deal doesn't work on.  For now, we apparently have to
    issue a connect() on our just-tickled socket so we can write() back.
    Again, why the fuck doesn't it just get filled in and taken care of?!
    This hack is anything but optimal.  Basically, if you want your listener
    to also be able to send data back, you need this connect() line, which
    also has the side effect that now anything from a different source or even a
    different port on the other end won't show up and will cause ICMP errors.
    I guess that's what they meant by "connect".
    Let's try to remember what the "U" is *really* for, eh? */
    /* If peer address is specified, connect to it */
    (*ptr_to_globals).remend.len = LSA_SIZEOF_SA as libc::c_int as socklen_t;
    if !(*ptr_to_globals).themaddr.is_null() {
      (*ptr_to_globals).remend = *(*ptr_to_globals).themaddr;
      xconnect(
        netfd as libc::c_int,
        &mut (*(*ptr_to_globals).themaddr).u.sa,
        (*(*ptr_to_globals).themaddr).len,
      );
    }
    /* peek first packet and remember peer addr */
    arm((*ptr_to_globals).o_wait); /* might as well timeout this, too */
    if _setjmp((*ptr_to_globals).jbuf.as_mut_ptr()) == 0i32 {
      /* do timeout for initial connect */
      /* (*ouraddr) is prefilled with "default" address */
      /* and here we block... */
      rr = recv_from_to(
        netfd as libc::c_int,
        0 as *mut libc::c_void,
        0i32 as size_t,
        MSG_PEEK as libc::c_int,
        &mut (*ptr_to_globals).remend.u.sa,
        &mut (*(*ptr_to_globals).ouraddr).u.sa,
        (*(*ptr_to_globals).ouraddr).len,
      ) as libc::c_int;
      if rr < 0i32 {
        bb_simple_perror_msg_and_die(b"recvfrom\x00" as *const u8 as *const libc::c_char);
      }
      unarm();
    } else {
      bb_simple_error_msg_and_die(b"timeout\x00" as *const u8 as *const libc::c_char);
    }
    /* Now we learned *to which IP* peer has connected, and we want to anchor
    our socket on it, so that our outbound packets will have correct local IP.
    Unfortunately, bind() on already bound socket will fail now (EINVAL):
      xbind(netfd, &ouraddr->u.sa, ouraddr->len);
    Need to read the packet, save data, close this socket and
    create new one, and bind() it. TODO */
    if (*ptr_to_globals).themaddr.is_null() {
      xconnect(
        netfd as libc::c_int,
        &mut (*ptr_to_globals).remend.u.sa,
        (*(*ptr_to_globals).ouraddr).len,
      );
    }
  } else {
    loop {
      arm((*ptr_to_globals).o_wait);
      if _setjmp((*ptr_to_globals).jbuf.as_mut_ptr()) == 0i32 {
        loop {
          (*ptr_to_globals).remend.len = LSA_SIZEOF_SA as libc::c_int as socklen_t;
          rr = accept(
            netfd as libc::c_int,
            __SOCKADDR_ARG {
              __sockaddr__: &mut (*ptr_to_globals).remend.u.sa as *mut sockaddr,
            },
            &mut (*ptr_to_globals).remend.len,
          );
          if rr < 0i32 {
            bb_simple_perror_msg_and_die(b"accept\x00" as *const u8 as *const libc::c_char);
          }
          if (*ptr_to_globals).themaddr.is_null() {
            break;
          }
          let mut sv_port: libc::c_int = 0;
          let mut port: libc::c_int = 0;
          let mut r: libc::c_int = 0;
          //if (rr < 0)
          //	bb_perror_msg_and_die("getsockname after accept");
          sv_port = get_nport(&mut (*ptr_to_globals).remend.u.sa); /* save */
          port = get_nport(&mut (*(*ptr_to_globals).themaddr).u.sa);
          if port == 0i32 {
            /* "nc -nl -p LPORT RHOST" (w/o RPORT!):
             * we should accept any remote port */
            set_nport(&mut (*ptr_to_globals).remend.u.sa, 0i32 as libc::c_uint);
            /* blot out remote port# */
          } /* restore */
          r = memcmp(
            &mut (*ptr_to_globals).remend.u.sa as *mut sockaddr as *const libc::c_void,
            &mut (*(*ptr_to_globals).themaddr).u.sa as *mut sockaddr as *const libc::c_void,
            (*ptr_to_globals).remend.len as libc::c_ulong,
          );
          set_nport(&mut (*ptr_to_globals).remend.u.sa, sv_port as libc::c_uint);
          if !(r != 0i32) {
            break;
          }
          /* nc 1.10 bails out instead, and its error message
           * is not suppressed by o_verbose */
          if (*ptr_to_globals).o_verbose != 0 {
            let mut remaddr: *mut libc::c_char =
              xmalloc_sockaddr2dotted(&mut (*ptr_to_globals).remend.u.sa);
            bb_error_msg(
              b"connect from wrong ip/port %s ignored\x00" as *const u8 as *const libc::c_char,
              remaddr,
            );
            free(remaddr as *mut libc::c_void);
          }
          close(rr);
        }
        unarm();
      } else {
        bb_simple_error_msg_and_die(b"timeout\x00" as *const u8 as *const libc::c_char);
      }
      if !(is_persistent != 0 && !proggie.is_null()) {
        break;
      }
      /* -l -k -e PROG */
      signal(
        17i32,
        ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(1i32 as libc::intptr_t),
      ); /* no zombies please */
      if ({
        let mut bb__xvfork_pid: pid_t = vfork();
        if bb__xvfork_pid < 0i32 {
          bb_simple_perror_msg_and_die(b"vfork\x00" as *const u8 as *const libc::c_char);
        }
        bb__xvfork_pid
      }) != 0i32
      {
        /* parent: go back and accept more connections */
        close(rr);
      } else {
        /* child */
        signal(17i32, None); /* dump the old socket, here's our new one */
        break;
      }
    }
    xmove_fd(rr, netfd as libc::c_int);
    /* find out what address the connection was *to* on our end, in case we're
    doing a listen-on-any on a multihomed machine.  This allows one to
    offer different services via different alias addresses, such as the
    "virtual web site" hack. */
    getsockname(
      netfd as libc::c_int,
      __SOCKADDR_ARG {
        __sockaddr__: &mut (*(*ptr_to_globals).ouraddr).u.sa as *mut sockaddr,
      },
      &mut (*(*ptr_to_globals).ouraddr).len,
    );
  }
  if (*ptr_to_globals).o_verbose != 0 {
    let mut lcladdr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut remaddr_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut remhostname: *mut libc::c_char = 0 as *mut libc::c_char;
    /* If we can, look for any IP options.  Useful for testing the receiving end of
    such things, and is a good exercise in dealing with it.  We do this before
    the connect message, to ensure that the connect msg is uniformly the LAST
    thing to emerge after all the intervening crud.  Doesn't work for UDP on
    any machines I've tested, but feel free to surprise me. */
    let mut optbuf: [libc::c_char; 40] = [0; 40];
    let mut x: socklen_t =
      ::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong as socklen_t;
    rr = getsockopt(
      netfd as libc::c_int,
      IPPROTO_IP as libc::c_int,
      4i32,
      optbuf.as_mut_ptr() as *mut libc::c_void,
      &mut x,
    );
    if rr >= 0i32 && x != 0 {
      /* we've got options, lessee em... */
      *bin2hex(
        (*ptr_to_globals).bigbuf_net.as_mut_ptr(),
        optbuf.as_mut_ptr(),
        x as libc::c_int,
      ) = '\u{0}' as i32 as libc::c_char;
      fprintf(
        stderr,
        b"IP options: %s\n\x00" as *const u8 as *const libc::c_char,
        (*ptr_to_globals).bigbuf_net.as_mut_ptr(),
      );
    }
    /* now check out who it is.  We don't care about mismatched DNS names here,
    but any ADDR and PORT we specified had better fucking well match the caller.
    Converting from addr to inet_ntoa and back again is a bit of a kludge, but
    gethostpoop wants a string and there's much gnarlier code out there already,
    so I don't feel bad.
    The *real* question is why BFD sockets wasn't designed to allow listens for
    connections *from* specific hosts/ports, instead of requiring the caller to
    accept the connection and then reject undesirable ones by closing.
    In other words, we need a TCP MSG_PEEK. */
    /* bbox: removed most of it */
    lcladdr = xmalloc_sockaddr2dotted(&mut (*(*ptr_to_globals).ouraddr).u.sa);
    remaddr_0 = xmalloc_sockaddr2dotted(&mut (*ptr_to_globals).remend.u.sa);
    remhostname = if option_mask32 & OPT_n as libc::c_int as libc::c_uint != 0 {
      remaddr_0
    } else {
      xmalloc_sockaddr2host(&mut (*ptr_to_globals).remend.u.sa)
    };
    fprintf(
      stderr,
      b"connect to %s from %s (%s)\n\x00" as *const u8 as *const libc::c_char,
      lcladdr,
      remhostname,
      remaddr_0,
    );
    free(lcladdr as *mut libc::c_void);
    free(remaddr_0 as *mut libc::c_void);
    if option_mask32 & OPT_n as libc::c_int as libc::c_uint == 0 {
      free(remhostname as *mut libc::c_void);
    }
  }
  if !proggie.is_null() {
    doexec(proggie);
  };
}
/* udptest:
fire a couple of packets at a UDP target port, just to see if it's really
there.  On BSD kernels, ICMP host/port-unreachable errors get delivered to
our socket as ECONNREFUSED write errors.  On SV kernels, we lose; we'll have
to collect and analyze raw ICMP ourselves a la satan's probe_udp_ports
backend.  Guess where one could swipe the appropriate code from...

Use the time delay between writes if given, otherwise use the "tcp ping"
trick for getting the RTT.  [I got that idea from pluvius, and warped it.]
Return either the original fd, or clean up and return -1. */
unsafe extern "C" fn udptest() -> libc::c_int {
  let mut rr: libc::c_int = 0; // can be interrupted! while (t) nanosleep(&t)?
  rr = write(
    netfd as libc::c_int,
    (*ptr_to_globals).bigbuf_in.as_mut_ptr() as *const libc::c_void,
    1i32 as size_t,
  ) as libc::c_int;
  if rr != 1i32 {
    bb_simple_perror_msg(b"udptest first write\x00" as *const u8 as *const libc::c_char);
  }
  if (*ptr_to_globals).o_wait != 0 {
    sleep((*ptr_to_globals).o_wait);
  } else {
    /* use the tcp-ping trick: try connecting to a normally refused port, which
    causes us to block for the time that SYN gets there and RST gets back.
    Not completely reliable, but it *does* mostly work. */
    /* Set a temporary connect timeout, so packet filtration doesn't cause
    us to hang forever, and hit it */
    (*ptr_to_globals).o_wait = 5i32 as libc::c_uint;
    rr = xsocket(
      (*(*ptr_to_globals).ouraddr).u.sa.sa_family as libc::c_int,
      SOCK_STREAM as libc::c_int,
      0i32,
    );
    set_nport(
      &mut (*(*ptr_to_globals).themaddr).u.sa,
      ({
        let mut __v: libc::c_ushort = 0; /* enough that we'll notice?? */
        let mut __x: libc::c_ushort = SLEAZE_PORT as libc::c_int as libc::c_ushort;
        if 0 != 0 {
          __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
            as libc::c_ushort
        } else {
          let fresh1 = &mut __v;
          let fresh2;
          let fresh3 = __x;
          asm!("rorw $$8, ${0:w}" : "=r" (fresh2) : "0"
                                (c2rust_asm_casts::AsmCast::cast_in(fresh1, fresh3))
                                : "cc");
          c2rust_asm_casts::AsmCast::cast_out(fresh1, fresh3, fresh2);
        }
        __v
      }) as libc::c_uint,
    );
    connect_w_timeout(rr);
    /* restore */
    close(rr);
    (*ptr_to_globals).o_wait = 0i32 as libc::c_uint
  }
  rr = write(
    netfd as libc::c_int,
    (*ptr_to_globals).bigbuf_in.as_mut_ptr() as *const libc::c_void,
    1i32 as size_t,
  ) as libc::c_int;
  return (rr != 1i32) as libc::c_int;
  /* don't need to restore themaddr's port, it's not used anymore */
  /* if rr == 1, return 0 (success) */
}
/* oprint:
Hexdump bytes shoveled either way to a running logfile, in the format:
D offset       -  - - - --- 16 bytes --- - - -  -     # .... ascii .....
where "which" sets the direction indicator, D:
0 -- sent to network, or ">"
1 -- rcvd and printed to stdout, or "<"
and "buf" and "n" are data-block and length.  If the current block generates
a partial line, so be it; we *want* that lockstep indication of who sent
what when.  Adapted from dgaudet's original example -- but must be ripping
*fast*, since we don't want to be too disk-bound... */
unsafe extern "C" fn oprint(
  mut direction: libc::c_int,
  mut p: *mut libc::c_uchar,
  mut bc: libc::c_uint,
) {
  let mut obc: libc::c_uint = 0; /* current "global" offset */
  let mut x: libc::c_uint = 0; /* out hexdump ptr */
  let mut op: *mut libc::c_uchar = 0 as *mut libc::c_uchar; /* out asc-dump ptr */
  let mut ap: *mut libc::c_uchar = 0 as *mut libc::c_uchar; /* use the globals! */
  let mut stage: [libc::c_uchar; 100] = [0; 100]; /* preload separator */
  if bc == 0i32 as libc::c_uint {
    return;
  }
  obc = (*ptr_to_globals).wrote_net as libc::c_uint;
  if direction == '<' as i32 {
    obc = (*ptr_to_globals).wrote_out as libc::c_uint
  }
  stage[0] = direction as libc::c_uchar;
  stage[59] = '#' as i32 as libc::c_uchar;
  stage[60] = ' ' as i32 as libc::c_uchar;
  loop {
    /* for chunk-o-data ... */
    x = 16i32 as libc::c_uint;
    if bc < 16i32 as libc::c_uint {
      /* memset(&stage[bc*3 + 11], ' ', 16*3 - bc*3); */
      memset(
        &mut *stage.as_mut_ptr().offset(11) as *mut libc::c_uchar as *mut libc::c_void,
        ' ' as i32,
        (16i32 * 3i32) as libc::c_ulong,
      ); /* xxx: still slow? */
      x = bc
    } /* fix current count */
    sprintf(
      &mut *stage.as_mut_ptr().offset(1) as *mut libc::c_uchar as *mut libc::c_char,
      b" %8.8x \x00" as *const u8 as *const libc::c_char,
      obc,
    ); /* fix current offset */
    bc = bc.wrapping_sub(x); /* where hex starts */
    obc = obc.wrapping_add(x); /* where ascii starts */
    op = &mut *stage.as_mut_ptr().offset(11) as *mut libc::c_uchar;
    ap = &mut *stage.as_mut_ptr().offset(61) as *mut libc::c_uchar;
    loop {
      /* for line of dump, however long ... */
      let fresh4 = op; /* nonprinting, loose def */
      op = op.offset(1); /* printing */
      *fresh4 = (0x20i32
        | *bb_hexdigits_upcase
          .as_ptr()
          .offset((*p as libc::c_int >> 4i32) as isize) as libc::c_int)
        as libc::c_uchar; /* finish the line */
      let fresh5 = op;
      op = op.offset(1);
      *fresh5 = (0x20i32
        | *bb_hexdigits_upcase
          .as_ptr()
          .offset((*p as libc::c_int & 0xfi32) as isize) as libc::c_int)
        as libc::c_uchar;
      let fresh6 = op;
      op = op.offset(1);
      *fresh6 = ' ' as i32 as libc::c_uchar;
      if *p as libc::c_int > 31i32 && (*p as libc::c_int) < 127i32 {
        *ap = *p
      } else {
        *ap = '.' as i32 as libc::c_uchar
      }
      ap = ap.offset(1);
      p = p.offset(1);
      x = x.wrapping_sub(1);
      if !(x != 0) {
        break;
      }
    }
    let fresh7 = ap;
    ap = ap.offset(1);
    *fresh7 = '\n' as i32 as libc::c_uchar;
    xwrite(
      ofd as libc::c_int,
      stage.as_mut_ptr() as *const libc::c_void,
      ap.wrapping_offset_from(stage.as_mut_ptr()) as libc::c_long as size_t,
    );
    if !(bc != 0) {
      break;
    }
  }
}
/* readwrite:
handle stdin/stdout/network I/O.  Bwahaha!! -- the i/o loop from hell.
In this instance, return what might become our exit status. */
unsafe extern "C" fn readwrite() -> libc::c_int {
  let mut zp: *mut libc::c_char = 0 as *mut libc::c_char; /* gcc */
  /* stdin buf ptr */
  zp = zp; /* net-in buf ptr */
  let mut np: *mut libc::c_char = 0 as *mut libc::c_char; /* net-read retry counter */
  np = np; /* pause *before* sending stuff, too */
  let mut rzleft: libc::c_uint = 0;
  let mut rnleft: libc::c_uint = 0;
  let mut netretry: libc::c_uint = 0;
  let mut fds_open: libc::c_uint = 0;
  let mut pfds: [pollfd; 2] = [pollfd {
    fd: 0,
    events: 0,
    revents: 0,
  }; 2];
  pfds[0].fd = 0i32;
  pfds[0].events = 0x1i32 as libc::c_short;
  pfds[1].fd = netfd as libc::c_int;
  pfds[1].events = 0x1i32 as libc::c_short;
  fds_open = 2i32 as libc::c_uint;
  netretry = 2i32 as libc::c_uint;
  rnleft = 0i32 as libc::c_uint;
  rzleft = rnleft;
  if (*ptr_to_globals).o_interval != 0 {
    sleep((*ptr_to_globals).o_interval);
  }
  /* and now the big ol' shoveling loop ... */
  /* nc 1.10 has "while (FD_ISSET(netfd)" here */
  's_60: while fds_open != 0 {
    let mut rr: libc::c_int = 0; /* while (fds_open) */
    let mut poll_tmout_ms: libc::c_int = 0; /* net-write sanity counter */
    let mut wretry: libc::c_uint = 8200i32 as libc::c_uint;
    poll_tmout_ms = -1i32;
    if (*ptr_to_globals).o_wait != 0 {
      poll_tmout_ms = 2147483647i32;
      if (*ptr_to_globals).o_wait < (2147483647i32 / 1000i32) as libc::c_uint {
        poll_tmout_ms = (*ptr_to_globals)
          .o_wait
          .wrapping_mul(1000i32 as libc::c_uint) as libc::c_int
      }
    }
    rr = poll(pfds.as_mut_ptr(), 2i32 as nfds_t, poll_tmout_ms);
    if rr < 0i32 && *bb_errno != 4i32 {
      /* might have gotten ^Zed, etc */
      if (*ptr_to_globals).o_verbose != 0 {
        bb_simple_perror_msg(b"poll\x00" as *const u8 as *const libc::c_char);
      }
      close(netfd as libc::c_int);
      return 1i32;
    }
    /* if we have a timeout AND stdin is closed AND we haven't heard anything
    from the net during that time, assume it's dead and close it too. */
    if rr == 0i32 {
      if pfds[0].revents == 0 {
        /* timeout */
        netretry = netretry.wrapping_sub(1); /* we actually try a coupla times. */
        if netretry == 0 {
          if (*ptr_to_globals).o_verbose > 1i32 as libc::c_uint {
            /* normally we don't care */
            fprintf(
              stderr,
              b"net timeout\n\x00" as *const u8 as *const libc::c_char,
            );
          }
          /* not an error! */
          return 0i32;
        }
      }
    }
    /*close(netfd); - redundant, exit will do it */
    /* Ding!!  Something arrived, go check all the incoming hoppers, net first */
    if pfds[1].revents != 0 {
      /* net:ding */
      /* net: ding! */
      rr = read(
        netfd as libc::c_int,
        (*ptr_to_globals).bigbuf_net.as_mut_ptr() as *mut libc::c_void,
        BIGSIZ as libc::c_int as size_t,
      ) as libc::c_int;
      if rr <= 0i32 {
        if rr < 0i32 && (*ptr_to_globals).o_verbose > 1i32 as libc::c_uint {
          /* nc 1.10 doesn't do this */
          bb_simple_perror_msg(b"net read\x00" as *const u8 as *const libc::c_char);
        }
        /* can't write anymore: broken pipe */
        pfds[1].fd = -1i32; /* don't poll for netfd anymore */
        fds_open = fds_open.wrapping_sub(1);
        rzleft = 0i32 as libc::c_uint
      } else {
        rnleft = rr as libc::c_uint;
        np = (*ptr_to_globals).bigbuf_net.as_mut_ptr()
      }
    }
    /* if we're in "slowly" mode there's probably still stuff in the stdin
    buffer, so don't read unless we really need MORE INPUT!  MORE INPUT! */
    if !(rzleft != 0) {
      /* okay, suck more stdin */
      if pfds[0].revents != 0 {
        /* stdin:ding */
        /* stdin: ding! */
        rr = read(
          0i32,
          (*ptr_to_globals).bigbuf_in.as_mut_ptr() as *mut libc::c_void,
          BIGSIZ as libc::c_int as size_t,
        ) as libc::c_int;
        /* Considered making reads here smaller for UDP mode, but 8192-byte
        mobygrams are kinda fun and exercise the reassembler. */
        if rr <= 0i32 {
          /* at end, or fukt, or ... */
          pfds[0].fd = -1i32; /* disable stdin */
          /*close(STDIN_FILENO); - not really necessary */
          /* Let peer know we have no more data */
          /* nc 1.10 doesn't do this: */
          shutdown(netfd as libc::c_int, SHUT_WR as libc::c_int); /* rnleft */
          fds_open = fds_open.wrapping_sub(1)
        } else {
          rzleft = rr as libc::c_uint;
          zp = (*ptr_to_globals).bigbuf_in.as_mut_ptr()
        }
      }
    }
    loop
    /* now that we've dingdonged all our thingdings, send off the results.
    Geez, why does this look an awful lot like the big loop in "rsh"? ...
    not sure if the order of this matters, but write net -> stdout first. */
    {
      if rnleft != 0 {
        rr = write(1i32, np as *const libc::c_void, rnleft as size_t) as libc::c_int;
        if rr > 0i32 {
          if option_mask32 & OPT_o as libc::c_int as libc::c_uint != 0 {
            /* log the stdout */
            oprint('<' as i32, np as *mut libc::c_uchar, rr as libc::c_uint);
          }
          np = np.offset(rr as isize);
          rnleft = rnleft.wrapping_sub(rr as libc::c_uint);
          (*ptr_to_globals).wrote_out = (*ptr_to_globals)
            .wrote_out
            .wrapping_add(rr as libc::c_ulonglong)
          /* global count */
        }
      } /* rzleft */
      if rzleft != 0 {
        if (*ptr_to_globals).o_interval != 0 {
          /* in "slowly" mode ?? */
          rr = findline(zp, rzleft) as libc::c_int
        } else {
          rr = rzleft as libc::c_int
        } /* one line, or the whole buffer */
        rr = write(
          netfd as libc::c_int,
          zp as *const libc::c_void,
          rr as size_t,
        ) as libc::c_int;
        if rr > 0i32 {
          if option_mask32 & OPT_o as libc::c_int as libc::c_uint != 0 {
            /* log what got sent */
            oprint('>' as i32, zp as *mut libc::c_uchar, rr as libc::c_uint);
          }
          zp = zp.offset(rr as isize);
          rzleft = rzleft.wrapping_sub(rr as libc::c_uint);
          (*ptr_to_globals).wrote_net = (*ptr_to_globals)
            .wrote_net
            .wrapping_add(rr as libc::c_ulonglong)
          /* global count */
        }
      }
      if (*ptr_to_globals).o_interval != 0 {
        break;
      }
      if !(rzleft != 0 || rnleft != 0) {
        continue 's_60;
      }
      /* shovel that shit till they ain't */
      wretry = wretry.wrapping_sub(1); /* none left, and get another load */
      /* net write retries sometimes happen on UDP connections */
      if wretry == 0 {
        /* is something hung? */
        if (*ptr_to_globals).o_verbose != 0 {
          bb_simple_error_msg(b"too many output retries\x00" as *const u8 as *const libc::c_char);
        }
        return 1i32;
      }
    }
    /* cycle between slow lines, or ... */
    sleep((*ptr_to_globals).o_interval);
    /* ...with hairy loop... */
  }
  /* XXX: maybe want a more graceful shutdown() here, or screw around with
  linger times??  I suspect that I don't need to since I'm always doing
  blocking reads and writes and my own manual "last ditch" efforts to read
  the net again after a timeout.  I haven't seen any screwups yet, but it's
  not like my test network is particularly busy... */
  close(netfd as libc::c_int);
  return 0i32;
}
/* readwrite */
/* main: now we pull it all together... */
#[no_mangle]
pub unsafe extern "C" fn nc_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut current_block: u64; /* for compiler */
  let mut str_p: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut str_s: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut str_i: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut str_o: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut themdotted: *mut libc::c_char = 0 as *mut libc::c_char;
  themdotted = themdotted;
  let mut proggie: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  let mut x: libc::c_int = 0;
  let mut cnt_l: libc::c_uint = 0i32 as libc::c_uint;
  let mut o_lport: libc::c_uint = 0i32 as libc::c_uint;
  let ref mut fresh8 = *(not_const_pp(&ptr_to_globals as *const *mut globals as *const libc::c_void)
    as *mut *mut globals);
  *fresh8 = xzalloc(::std::mem::size_of::<globals>() as libc::c_ulong) as *mut globals;
  asm!("" : : : "memory" : "volatile");
  /* catch a signal or two for cleanup */
  bb_signals(
    0i32 + (1i32 << 2i32) + (1i32 << 3i32) + (1i32 << 15i32),
    Some(catch as unsafe extern "C" fn(_: libc::c_int) -> ()),
  );
  /* and suppress others... */
  bb_signals(
    0i32 + (1i32 << 23i32) + (1i32 << 13i32),
    ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(1i32 as libc::intptr_t),
  );
  proggie = argv;
  loop {
    proggie = proggie.offset(1);
    if (*proggie).is_null() {
      current_block = 7172762164747879670;
      break;
    }
    if strcmp(*proggie, b"-e\x00" as *const u8 as *const libc::c_char) == 0i32 {
      *proggie = 0 as *mut libc::c_char;
      proggie = proggie.offset(1);
      current_block = 10887457417680102994;
      break;
    } else {
      /* -<other_opts>e PROG [ARGS] ? */
      /* (aboriginal linux uses this form) */
      if !(*(*proggie.offset(0)).offset(0) as libc::c_int == '-' as i32) {
        continue;
      }
      let mut optpos: *mut libc::c_char = (*proggie).offset(1);
      /* Skip all valid opts w/o params */
      optpos =
        optpos.offset(strspn(optpos, b"nuvlkz\x00" as *const u8 as *const libc::c_char) as isize); /* terminate argv for getopt32 */
      if !(*optpos as libc::c_int == 'e' as i32 && *optpos.offset(1) == 0) {
        continue;
      }
      *optpos = '\u{0}' as i32 as libc::c_char;
      proggie = proggie.offset(1);
      (*ptr_to_globals).proggie0saved = *proggie;
      *proggie = 0 as *mut libc::c_char;
      current_block = 10887457417680102994;
      break;
    }
  }
  match current_block {
    7172762164747879670 => proggie = 0 as *mut *mut libc::c_char,
    _ => {}
  }
  // -g -G -t -r deleted, unimplemented -a deleted too
  getopt32(
    argv,
    b"^np:s:uvw:+lki:o:z\x00?2:vv:ll\x00" as *const u8 as *const libc::c_char,
    &mut str_p as *mut *mut libc::c_char,
    &mut str_s as *mut *mut libc::c_char,
    &mut (*ptr_to_globals).o_wait as *mut libc::c_uint,
    &mut str_i as *mut *mut libc::c_char,
    &mut str_o as *mut *mut libc::c_char,
    &mut (*ptr_to_globals).o_verbose as *mut libc::c_uint,
    &mut cnt_l as *mut libc::c_uint,
  );
  argv = argv.offset(optind as isize);
  if option_mask32 & OPT_i as libc::c_int as libc::c_uint != 0 {
    /* line-interval time */
    (*ptr_to_globals).o_interval =
      xatou_range(str_i, 1i32 as libc::c_uint, 0xffffi32 as libc::c_uint)
  }
  //if (option_mask32 & OPT_l) /* listen mode */
  if option_mask32 & OPT_k as libc::c_int as libc::c_uint != 0 {
    /* persistent server mode */
    cnt_l = 2i32 as libc::c_uint
  }
  //if (option_mask32 & OPT_n) /* numeric-only, no DNS lookups */
  //if (option_mask32 & OPT_o) /* hexdump log */
  if option_mask32 & OPT_p as libc::c_int as libc::c_uint != 0 {
    /* local source port */
    o_lport = bb_lookup_port(
      str_p,
      if option_mask32 & OPT_u as libc::c_int as libc::c_uint != 0 {
        b"udp\x00" as *const u8 as *const libc::c_char
      } else {
        b"tcp\x00" as *const u8 as *const libc::c_char
      },
      0i32 as libc::c_uint,
    );
    if o_lport == 0 {
      bb_error_msg_and_die(
        b"bad local port \'%s\'\x00" as *const u8 as *const libc::c_char,
        str_p,
      );
    }
  }
  //if (option_mask32 & OPT_r) /* randomize various things */
  //if (option_mask32 & OPT_u) /* use UDP */
  //if (option_mask32 & OPT_v) /* verbose */
  //if (option_mask32 & OPT_w) /* wait time */
  //if (option_mask32 & OPT_z) /* little or no data xfer */
  /* We manage our fd's so that they are never 0,1,2 */
  /*bb_sanitize_stdio(); - not needed */
  if !(*argv.offset(0)).is_null() {
    (*ptr_to_globals).themaddr = xhost2sockaddr(
      *argv.offset(0),
      if !(*argv.offset(1)).is_null() {
        bb_lookup_port(
          *argv.offset(1),
          if option_mask32 & OPT_u as libc::c_int as libc::c_uint != 0 {
            b"udp\x00" as *const u8 as *const libc::c_char
          } else {
            b"tcp\x00" as *const u8 as *const libc::c_char
          },
          0i32 as libc::c_uint,
        )
      } else {
        0i32 as libc::c_uint
      } as libc::c_int,
    )
  }
  /* create & bind network socket */
  x = if option_mask32 & OPT_u as libc::c_int as libc::c_uint != 0 {
    SOCK_DGRAM as libc::c_int
  } else {
    SOCK_STREAM as libc::c_int
  };
  if option_mask32 & OPT_s as libc::c_int as libc::c_uint != 0 {
    /* local address */
    /* if o_lport is still 0, then we will use random port */
    (*ptr_to_globals).ouraddr = xhost2sockaddr(str_s, o_lport as libc::c_int);
    x = xsocket(
      (*(*ptr_to_globals).ouraddr).u.sa.sa_family as libc::c_int,
      x,
      0i32,
    )
  } else {
    /* We try IPv6, then IPv4, unless addr family is
     * implicitly set by way of remote addr/port spec */
    x = xsocket_type(
      &mut (*ptr_to_globals).ouraddr,
      if !(*ptr_to_globals).themaddr.is_null() {
        (*(*ptr_to_globals).themaddr).u.sa.sa_family as libc::c_int
      } else {
        0i32
      },
      x,
    ); /* won't need stdin */
    if o_lport != 0 {
      set_nport(
        &mut (*(*ptr_to_globals).ouraddr).u.sa,
        ({
          let mut __v: libc::c_ushort = 0;
          let mut __x: libc::c_ushort = o_lport as libc::c_ushort;
          if 0 != 0 {
            __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
              as libc::c_ushort
          } else {
            let fresh9 = &mut __v;
            let fresh10;
            let fresh11 = __x;
            asm!("rorw $$8, ${0:w}" : "=r" (fresh10) : "0"
                                    (c2rust_asm_casts::AsmCast::cast_in(fresh9, fresh11))
                                    : "cc");
            c2rust_asm_casts::AsmCast::cast_out(fresh9, fresh11, fresh10);
          }
          __v
        }) as libc::c_uint,
      );
    }
  }
  xmove_fd(x, netfd as libc::c_int);
  setsockopt_reuseaddr(netfd as libc::c_int);
  if option_mask32 & OPT_u as libc::c_int as libc::c_uint != 0 {
    socket_want_pktinfo(netfd as libc::c_int);
  }
  if 0i32 == 0
    || cnt_l != 0i32 as libc::c_uint
    || (*(*ptr_to_globals).ouraddr).u.sa.sa_family as libc::c_int != 1i32
  {
    xbind(
      netfd as libc::c_int,
      &mut (*(*ptr_to_globals).ouraddr).u.sa,
      (*(*ptr_to_globals).ouraddr).len,
    );
  }
  if !proggie.is_null() {
    close(0i32);
    option_mask32 &= !(OPT_o as libc::c_int) as libc::c_uint
    /* -o with -e is meaningless! */
  }
  if option_mask32 & OPT_o as libc::c_int as libc::c_uint != 0 {
    xmove_fd(
      xopen(str_o, 0o1i32 | 0o100i32 | 0o1000i32),
      ofd as libc::c_int,
    );
  }
  if cnt_l != 0i32 as libc::c_uint {
    dolisten(
      cnt_l.wrapping_sub(1i32 as libc::c_uint) as libc::c_int,
      proggie,
    );
    /* it even works with UDP! */
    x = readwrite()
  } else {
    /* dolisten does its own connect reporting */
    /* Outbound connects.  Now we're more picky about args... */
    if (*ptr_to_globals).themaddr.is_null() {
      bb_show_usage();
    }
    (*ptr_to_globals).remend = *(*ptr_to_globals).themaddr;
    if (*ptr_to_globals).o_verbose != 0 {
      themdotted = xmalloc_sockaddr2dotted(&mut (*(*ptr_to_globals).themaddr).u.sa)
    }
    x = connect_w_timeout(netfd as libc::c_int);
    if option_mask32 & OPT_z as libc::c_int as libc::c_uint != 0
      && x == 0i32
      && option_mask32 & OPT_u as libc::c_int as libc::c_uint != 0
    {
      /* if UDP scanning... */
      x = udptest()
    } /* connect or udptest wasn't successful */
    if x == 0i32 {
      /* Yow, are we OPEN YET?! */
      if (*ptr_to_globals).o_verbose != 0 {
        fprintf(
          stderr,
          b"%s (%s) open\n\x00" as *const u8 as *const libc::c_char,
          *argv.offset(0),
          themdotted,
        );
      }
      if !proggie.is_null() {
        /* exec is valid for outbound, too */
        doexec(proggie); /* exit status */
      }
      if option_mask32 & OPT_z as libc::c_int as libc::c_uint == 0 {
        x = readwrite()
      }
    } else {
      x = 1i32;
      /* if we're scanning at a "one -v" verbosity level, don't print refusals.
      Give it another -v if you want to see everything. */
      if (*ptr_to_globals).o_verbose > 1i32 as libc::c_uint
        || (*ptr_to_globals).o_verbose != 0 && *bb_errno != 111i32
      {
        bb_perror_msg(
          b"%s (%s)\x00" as *const u8 as *const libc::c_char,
          *argv.offset(0),
          themdotted,
        );
      }
    }
  }
  if (*ptr_to_globals).o_verbose > 1i32 as libc::c_uint {
    /* normally we don't care */
    fprintf(
      stderr,
      b"sent %llu, rcvd %llu\n\x00" as *const u8 as *const libc::c_char,
      (*ptr_to_globals).wrote_net,
      (*ptr_to_globals).wrote_out,
    );
  }
  return x;
}
//kbuild:lib-$(CONFIG_NC) += nc.o
//kbuild:lib-$(CONFIG_NETCAT) += nc.o
//applet:IF_NC(APPLET(nc, BB_DIR_USR_BIN, BB_SUID_DROP))
//                 APPLET_ODDNAME:name    main location        suid_type     help
//applet:IF_NETCAT(APPLET_ODDNAME(netcat, nc,  BB_DIR_USR_BIN, BB_SUID_DROP, nc))
/* vi: set sw=4 ts=4: */
/*
 * nc: mini-netcat - built from the ground up for LRP
 *
 * Copyright (C) 1998, 1999  Charles P. Wright
 * Copyright (C) 1998  Dave Cinege
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config NC
//config:	bool "nc (11 kb)"
//config:	default y
//config:	help
//config:	A simple Unix utility which reads and writes data across network
//config:	connections.
//config:
//config:config NETCAT
//config:	bool "netcat (11 kb)"
//config:	default n
//config:	help
//config:	Alias to nc.
//config:
//config:config NC_SERVER
//config:	bool "Netcat server options (-l)"
//config:	default y
//config:	depends on NC || NETCAT
//config:	help
//config:	Allow netcat to act as a server.
//config:
//config:config NC_EXTRA
//config:	bool "Netcat extensions (-eiw and -f FILE)"
//config:	default y
//config:	depends on NC || NETCAT
//config:	help
//config:	Add -e (support for executing the rest of the command line after
//config:	making or receiving a successful connection), -i (delay interval for
//config:	lines sent), -w (timeout for initial connection).
//config:
//config:config NC_110_COMPAT
//config:	bool "Netcat 1.10 compatibility (+2.5k)"
//config:	default y
//config:	depends on NC || NETCAT
//config:	help
//config:	This option makes nc closely follow original nc-1.10.
//config:	The code is about 2.5k bigger. It enables
//config:	-s ADDR, -n, -u, -v, -o FILE, -z options, but loses
//config:	busybox-specific extensions: -f FILE.
