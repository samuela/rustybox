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
  fn __cmsg_nxthdr(__mhdr: *mut msghdr, __cmsg: *mut cmsghdr) -> *mut cmsghdr;
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
  fn sendmsg(__fd: libc::c_int, __message: *const msghdr, __flags: libc::c_int) -> ssize_t;
  #[no_mangle]
  fn recvmsg(__fd: libc::c_int, __message: *mut msghdr, __flags: libc::c_int) -> ssize_t;
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn setsockopt_1(fd: libc::c_int, level: libc::c_int, optname: libc::c_int) -> libc::c_int;
}

pub type __socklen_t = libc::c_uint;
use crate::librb::size_t;
use crate::librb::ssize_t;
use libc::uint16_t;
use libc::uint32_t;
 use libc::uint8_t;
pub type socklen_t = __socklen_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iovec {
  pub iov_base: *mut libc::c_void,
  pub iov_len: size_t,
}
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
  pub sa_family: sa_family_t,
  pub sa_data: [libc::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct msghdr {
  pub msg_name: *mut libc::c_void,
  pub msg_namelen: socklen_t,
  pub msg_iov: *mut iovec,
  pub msg_iovlen: size_t,
  pub msg_control: *mut libc::c_void,
  pub msg_controllen: size_t,
  pub msg_flags: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmsghdr {
  pub cmsg_len: size_t,
  pub cmsg_level: libc::c_int,
  pub cmsg_type: libc::c_int,
  pub __cmsg_data: [libc::c_uchar; 0],
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
  pub __in6_u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_pktinfo {
  pub ipi_ifindex: libc::c_int,
  pub ipi_spec_dst: in_addr,
  pub ipi_addr: in_addr,
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const IPPROTO_MAX: C2RustUnnamed_0 = 256;
pub const IPPROTO_RAW: C2RustUnnamed_0 = 255;
pub const IPPROTO_MPLS: C2RustUnnamed_0 = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed_0 = 136;
pub const IPPROTO_SCTP: C2RustUnnamed_0 = 132;
pub const IPPROTO_COMP: C2RustUnnamed_0 = 108;
pub const IPPROTO_PIM: C2RustUnnamed_0 = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed_0 = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed_0 = 94;
pub const IPPROTO_MTP: C2RustUnnamed_0 = 92;
pub const IPPROTO_AH: C2RustUnnamed_0 = 51;
pub const IPPROTO_ESP: C2RustUnnamed_0 = 50;
pub const IPPROTO_GRE: C2RustUnnamed_0 = 47;
pub const IPPROTO_RSVP: C2RustUnnamed_0 = 46;
pub const IPPROTO_IPV6: C2RustUnnamed_0 = 41;
pub const IPPROTO_DCCP: C2RustUnnamed_0 = 33;
pub const IPPROTO_TP: C2RustUnnamed_0 = 29;
pub const IPPROTO_IDP: C2RustUnnamed_0 = 22;
pub const IPPROTO_UDP: C2RustUnnamed_0 = 17;
pub const IPPROTO_PUP: C2RustUnnamed_0 = 12;
pub const IPPROTO_EGP: C2RustUnnamed_0 = 8;
pub const IPPROTO_TCP: C2RustUnnamed_0 = 6;
pub const IPPROTO_IPIP: C2RustUnnamed_0 = 4;
pub const IPPROTO_IGMP: C2RustUnnamed_0 = 2;
pub const IPPROTO_ICMP: C2RustUnnamed_0 = 1;
pub const IPPROTO_IP: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_pktinfo {
  pub ipi6_addr: in6_addr,
  pub ipi6_ifindex: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
  pub cmsg: [libc::c_char; 32],
  pub cmsg6: [libc::c_char; 40],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
  pub cmsg: [libc::c_char; 32],
  pub cmsg6: [libc::c_char; 40],
}

/*
 * Utility routines.
 *
 * Copyright (C) 2007 Denys Vlasenko
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
/*
 * This asks kernel to let us know dst addr/port of incoming packets
 * We don't check for errors here. Not supported == won't be used
 */
#[no_mangle]
pub unsafe extern "C" fn socket_want_pktinfo(mut fd: libc::c_int) {
  setsockopt_1(fd, IPPROTO_IP as libc::c_int, 8i32);
  setsockopt_1(fd, IPPROTO_IPV6 as libc::c_int, 49i32);
}
#[no_mangle]
pub unsafe extern "C" fn send_to_from(
  mut fd: libc::c_int,
  mut buf: *mut libc::c_void,
  mut len: size_t,
  mut flags: libc::c_int,
  mut to: *const sockaddr,
  mut from: *const sockaddr,
  mut tolen: socklen_t,
) -> ssize_t {
  let mut iov: [iovec; 1] = [iovec {
    iov_base: 0 as *mut libc::c_void,
    iov_len: 0,
  }; 1];
  let mut msg: msghdr = msghdr {
    msg_name: 0 as *mut libc::c_void,
    msg_namelen: 0,
    msg_iov: 0 as *mut iovec,
    msg_iovlen: 0,
    msg_control: 0 as *mut libc::c_void,
    msg_controllen: 0,
    msg_flags: 0,
  };
  let mut u: C2RustUnnamed_1 = C2RustUnnamed_1 { cmsg: [0; 32] };
  let mut cmsgptr: *mut cmsghdr = 0 as *mut cmsghdr;
  if (*from).sa_family as libc::c_int != 2i32 && (*from).sa_family as libc::c_int != 10i32 {
    /* ANY local address */
    return sendto(
      fd,
      buf,
      len,
      flags,
      __CONST_SOCKADDR_ARG { __sockaddr__: to },
      tolen,
    );
  }
  /* man recvmsg and man cmsg is needed to make sense of code below */
  iov[0].iov_base = buf; /* or compiler will annoy us */
  iov[0].iov_len = len;
  memset(
    &mut u as *mut C2RustUnnamed_1 as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong,
  );
  memset(
    &mut msg as *mut msghdr as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<msghdr>() as libc::c_ulong,
  );
  msg.msg_name = to as *mut sockaddr as *mut libc::c_void;
  msg.msg_namelen = tolen;
  msg.msg_iov = iov.as_mut_ptr();
  msg.msg_iovlen = 1i32 as size_t;
  msg.msg_control = &mut u as *mut C2RustUnnamed_1 as *mut libc::c_void;
  msg.msg_controllen = ::std::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong;
  msg.msg_flags = flags;
  cmsgptr = if msg.msg_controllen >= ::std::mem::size_of::<cmsghdr>() as libc::c_ulong {
    msg.msg_control as *mut cmsghdr
  } else {
    0 as *mut cmsghdr
  };
  /*
   * Users report that to->sa_family can be AF_INET6 too,
   * if "to" was acquired by recv_from_to(). IOW: recv_from_to()
   * was seen showing IPv6 "from" even when the destination
   * of received packet (our local address) was IPv4.
   */
  if (*from).sa_family as libc::c_int == 2i32 {
    let mut pktptr: *mut in_pktinfo = 0 as *mut in_pktinfo;
    (*cmsgptr).cmsg_level = IPPROTO_IP as libc::c_int;
    (*cmsgptr).cmsg_type = 8i32;
    (*cmsgptr).cmsg_len = ((::std::mem::size_of::<cmsghdr>() as libc::c_ulong)
      .wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong)
      .wrapping_sub(1i32 as libc::c_ulong)
      & !(::std::mem::size_of::<size_t>() as libc::c_ulong).wrapping_sub(1i32 as libc::c_ulong))
    .wrapping_add(::std::mem::size_of::<in_pktinfo>() as libc::c_ulong);
    pktptr = (*cmsgptr).__cmsg_data.as_mut_ptr() as *mut in_pktinfo;
    /*pktptr->ipi_ifindex = 0; -- already done by memset(u...) */
    /* In general, CMSG_DATA() can be unaligned, but in this case
     * we know for sure it is sufficiently aligned:
     * CMSG_FIRSTHDR simply returns &u above,
     * and CMSG_DATA returns &u + size_t + int + int.
     * Thus direct assignment is ok:
     */
    (*pktptr).ipi_spec_dst = (*(from as *mut sockaddr_in)).sin_addr
  } else if (*from).sa_family as libc::c_int == 10i32 {
    let mut pktptr_0: *mut in6_pktinfo = 0 as *mut in6_pktinfo;
    (*cmsgptr).cmsg_level = IPPROTO_IPV6 as libc::c_int;
    (*cmsgptr).cmsg_type = 50i32;
    (*cmsgptr).cmsg_len = ((::std::mem::size_of::<cmsghdr>() as libc::c_ulong)
      .wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong)
      .wrapping_sub(1i32 as libc::c_ulong)
      & !(::std::mem::size_of::<size_t>() as libc::c_ulong).wrapping_sub(1i32 as libc::c_ulong))
    .wrapping_add(::std::mem::size_of::<in6_pktinfo>() as libc::c_ulong);
    pktptr_0 = (*cmsgptr).__cmsg_data.as_mut_ptr() as *mut in6_pktinfo;
    /* pktptr->ipi6_ifindex = 0; -- already done by memset(u...) */
    (*pktptr_0).ipi6_addr = (*(from as *mut sockaddr_in6)).sin6_addr
  }
  msg.msg_controllen = (*cmsgptr).cmsg_len;
  return sendmsg(fd, &mut msg, flags);
}

/*
 * Busybox main internal header file
 *
 * Based in part on code from sash, Copyright (c) 1999 by David I. Bell
 * Permission has been granted to redistribute this code under GPL.
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
/* TODO: and glibc? */
/* There are two incompatible basename's, let's not use them! */
/* See the dirname/basename man page for details */
/* dirname,basename */
/* Don't do this here:
 * #include <sys/sysinfo.h>
 * Some linux/ includes pull in conflicting definition
 * of struct sysinfo (only in some toolchanins), which breaks build.
 * Include sys/sysinfo.h only in those files which need it.
 */
/* Just in case libc doesn't define some of these... */
//This breaks on bionic:
//# if !defined(__socklen_t_defined) && !defined(_SOCKLEN_T_DECLARED)
// /* We #define socklen_t *after* includes, otherwise we get
// * typedef redefinition errors from system headers
// * (in case "is it defined already" detection above failed)
// */
//#  define socklen_t bb_socklen_t
//   typedef unsigned socklen_t;
//# endif
//if this is still needed, add a fix along the lines of
//  ifdef SPECIFIC_BROKEN_LIBC_CHECK / typedef socklen_t / endif
//in platform.h instead!
/*
 * Use '%m' to append error string on platforms that support it,
 * '%s' and strerror() on those that don't.
 */
/*nothing*/
/* Some libc's forget to declare these, do it ourself */
/* klogctl is in libc's klog.h, but we cheat and not #include that */
/* Busybox does not use threads, we can speed up stdio. */
/* Above functions are required by POSIX.1-2008, below ones are extensions */
/* musl <= 1.1.15 does not support fflush_unlocked(NULL) */
//# undef  fflush
//# define fflush(stream) fflush_unlocked(stream)
/* Make all declarations hidden (-fvisibility flag only affects definitions) */
/* (don't include system headers after this until corresponding pop!) */
/* Tested to work correctly with all int types (IIRC :]) */
/* Large file support */
/* Note that CONFIG_LFS=y forces bbox to be built with all common ops
 * (stat, lseek etc) mapped to "largefile" variants by libc.
 * Practically it means that open() automatically has O_LARGEFILE added
 * and all filesize/file_offset parameters and struct members are "large"
 * (in today's world - signed 64bit). For full support of large files,
 * we need a few helper #defines (below) and careful use of off_t
 * instead of int/ssize_t. No lseek64(), O_LARGEFILE etc necessary */
/* CONFIG_LFS is on */
/* "long" is long enough on this system */
/* usage: sz = BB_STRTOOFF(s, NULL, 10); if (errno || sz < 0) die(); */
/* usage: printf("size: %"OFF_FMT"d (%"OFF_FMT"x)\n", sz, sz); */
/* scary. better ideas? (but do *test* them first!) */
/* Users report bionic to use 32-bit off_t even if LARGEFILE support is requested.
 * We misdetected that. Don't let it build:
 */
/* Some useful definitions */
/* Macros for min/max.  */
/* buffer allocation schemes */
/* glibc uses __errno_location() to get a ptr to errno */
/* We can just memorize it once - no multithreading in busybox :) */
/* dmalloc will redefine these to it's own implementation. It is safe
 * to have the prototypes here unconditionally.  */
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
/* NB: this will never set port# in 'to'!
 * _Only_ IP/IPv6 address part of 'to' is _maybe_ modified.
 * Typical usage is to preinit 'to' with "default" value
 * before calling recv_from_to(). */
#[no_mangle]
pub unsafe extern "C" fn recv_from_to(
  mut fd: libc::c_int,
  mut buf: *mut libc::c_void,
  mut len: size_t,
  mut flags: libc::c_int,
  mut from: *mut sockaddr,
  mut to: *mut sockaddr,
  mut sa_size: socklen_t,
) -> ssize_t {
  /* man recvmsg and man cmsg is needed to make sense of code below */
  let mut iov: [iovec; 1] = [iovec {
    iov_base: 0 as *mut libc::c_void,
    iov_len: 0,
  }; 1];
  let mut u: C2RustUnnamed_2 = C2RustUnnamed_2 { cmsg: [0; 32] };
  let mut cmsgptr: *mut cmsghdr = 0 as *mut cmsghdr;
  let mut msg: msghdr = msghdr {
    msg_name: 0 as *mut libc::c_void,
    msg_namelen: 0,
    msg_iov: 0 as *mut iovec,
    msg_iovlen: 0,
    msg_control: 0 as *mut libc::c_void,
    msg_controllen: 0,
    msg_flags: 0,
  };
  let mut recv_length: ssize_t = 0;
  iov[0].iov_base = buf;
  iov[0].iov_len = len;
  memset(
    &mut msg as *mut msghdr as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<msghdr>() as libc::c_ulong,
  );
  msg.msg_name = from as *mut libc::c_void;
  msg.msg_namelen = sa_size;
  msg.msg_iov = iov.as_mut_ptr();
  msg.msg_iovlen = 1i32 as size_t;
  msg.msg_control = &mut u as *mut C2RustUnnamed_2 as *mut libc::c_void;
  msg.msg_controllen = ::std::mem::size_of::<C2RustUnnamed_2>() as libc::c_ulong;
  recv_length = recvmsg(fd, &mut msg, flags);
  if recv_length < 0i32 as libc::c_long {
    return recv_length;
  }
  /* Here we try to retrieve destination IP and memorize it */
  cmsgptr = if msg.msg_controllen >= ::std::mem::size_of::<cmsghdr>() as libc::c_ulong {
    msg.msg_control as *mut cmsghdr
  } else {
    0 as *mut cmsghdr
  };
  while !cmsgptr.is_null() {
    if (*cmsgptr).cmsg_level == IPPROTO_IP as libc::c_int && (*cmsgptr).cmsg_type == 8i32 {
      let IPI_ADDR_OFF: libc::c_int = 8u64 as libc::c_int;
      (*to).sa_family = 2i32 as sa_family_t;
      /*# define pktinfo(cmsgptr) ( (struct in_pktinfo*)(CMSG_DATA(cmsgptr)) )*/
      /*to4->sin_addr = pktinfo(cmsgptr)->ipi_addr; - may be unaligned */
      memcpy(
        &mut (*(to as *mut sockaddr_in)).sin_addr as *mut in_addr as *mut libc::c_void,
        ((*cmsgptr).__cmsg_data.as_mut_ptr() as *mut libc::c_char).offset(IPI_ADDR_OFF as isize)
          as *const libc::c_void,
        ::std::mem::size_of::<in_addr>() as libc::c_ulong,
      );
      /*to4->sin_port = 123; - this data is not supplied by kernel */
      break;
    } else if (*cmsgptr).cmsg_level == IPPROTO_IPV6 as libc::c_int && (*cmsgptr).cmsg_type == 50i32
    {
      let IPI6_ADDR_OFF: libc::c_int = 0u64 as libc::c_int;
      (*to).sa_family = 10i32 as sa_family_t;
      /*#  define pktinfo(cmsgptr) ( (struct in6_pktinfo*)(CMSG_DATA(cmsgptr)) )*/
      /*to6->sin6_addr = pktinfo(cmsgptr)->ipi6_addr; - may be unaligned */
      memcpy(
        &mut (*(to as *mut sockaddr_in6)).sin6_addr as *mut in6_addr as *mut libc::c_void,
        ((*cmsgptr).__cmsg_data.as_mut_ptr() as *mut libc::c_char).offset(IPI6_ADDR_OFF as isize)
          as *const libc::c_void,
        ::std::mem::size_of::<in6_addr>() as libc::c_ulong,
      );
      break;
    } else {
      cmsgptr = __cmsg_nxthdr(&mut msg, cmsgptr)
    }
  }
  return recv_length;
}
