use crate::libbb::ptr_to_globals::bb_errno;
use crate::libbb::xfuncs_printf::xmalloc;
use crate::librb::len_and_sockaddr;
use crate::librb::size_t;
use crate::librb::socklen_t;
use c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use libc;
use libc::free;
use libc::getpid;
use libc::sa_family_t;
use libc::sockaddr;
use libc::strchr;
use libc::strrchr;
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
  fn socket(__domain: libc::c_int, __type: libc::c_int, __protocol: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn getsockname(__fd: libc::c_int, __addr: __SOCKADDR_ARG, __len: *mut socklen_t) -> libc::c_int;
  #[no_mangle]
  fn connect(__fd: libc::c_int, __addr: __CONST_SOCKADDR_ARG, __len: socklen_t) -> libc::c_int;
  #[no_mangle]
  fn getpeername(__fd: libc::c_int, __addr: __SOCKADDR_ARG, __len: *mut socklen_t) -> libc::c_int;
  #[no_mangle]
  fn setsockopt(
    __fd: libc::c_int,
    __level: libc::c_int,
    __optname: libc::c_int,
    __optval: *const libc::c_void,
    __optlen: socklen_t,
  ) -> libc::c_int;
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  fn getservbyname(__name: *const libc::c_char, __proto: *const libc::c_char) -> *mut servent;
  #[no_mangle]
  fn getaddrinfo(
    __name: *const libc::c_char,
    __service: *const libc::c_char,
    __req: *const addrinfo,
    __pai: *mut *mut addrinfo,
  ) -> libc::c_int;
  #[no_mangle]
  fn freeaddrinfo(__ai: *mut addrinfo);
  #[no_mangle]
  fn getnameinfo(
    __sa: *const sockaddr,
    __salen: socklen_t,
    __host: *mut libc::c_char,
    __hostlen: socklen_t,
    __serv: *mut libc::c_char,
    __servlen: socklen_t,
    __flags: libc::c_int,
  ) -> libc::c_int;

  #[no_mangle]
  fn inet_ntoa(__in: in_addr) -> *mut libc::c_char;
  #[no_mangle]
  fn inet_pton(
    __af: libc::c_int,
    __cp: *const libc::c_char,
    __buf: *mut libc::c_void,
  ) -> libc::c_int;
  #[no_mangle]
  fn inet_aton(__cp: *const libc::c_char, __inp: *mut in_addr) -> libc::c_int;

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
/*u8 *server_write_MAC_key;*/
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

/*
 * ascii-to-numbers implementations for busybox
 *
 * Copyright (C) 2003  Manuel Novoa III  <mjn3@codepoet.org>
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
/* Provides extern declarations of functions */
/* Unsigned long long functions always exist */
/* Provides inline definitions of functions */
/* (useful for mapping them to the type of the same width) */
/* If long == long long, then just map them one-to-one */
/* Same for int -> [long] long */
/* Specialized */
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

}

pub type __caddr_t = *mut libc::c_char;
pub type __socklen_t = libc::c_uint;

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

use libc::sockaddr_in6;

use crate::librb::in6_addr;

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed {
  pub __u6_addr8: [u8; 16],
  pub __u6_addr16: [u16; 8],
  pub __u6_addr32: [u32; 4],
}

pub type in_port_t = u16;

use libc::sockaddr_in;

use libc::in_addr;
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifmap {
  pub mem_start: libc::c_ulong,
  pub mem_end: libc::c_ulong,
  pub base_addr: libc::c_ushort,
  pub irq: libc::c_uchar,
  pub dma: libc::c_uchar,
  pub port: libc::c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifreq {
  pub ifr_ifrn: C2RustUnnamed_1,
  pub ifr_ifru: C2RustUnnamed_0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_0 {
  pub ifru_addr: sockaddr,
  pub ifru_dstaddr: sockaddr,
  pub ifru_broadaddr: sockaddr,
  pub ifru_netmask: sockaddr,
  pub ifru_hwaddr: sockaddr,
  pub ifru_flags: libc::c_short,
  pub ifru_ivalue: libc::c_int,
  pub ifru_mtu: libc::c_int,
  pub ifru_map: ifmap,
  pub ifru_slave: [libc::c_char; 16],
  pub ifru_newname: [libc::c_char; 16],
  pub ifru_data: __caddr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_1 {
  pub ifrn_name: [libc::c_char; 16],
}
pub type u32 = libc::c_uint;
pub type __kernel_sa_family_t = libc::c_ushort;

use libc::sockaddr_nl;

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
pub struct addrinfo {
  pub ai_flags: libc::c_int,
  pub ai_family: libc::c_int,
  pub ai_socktype: libc::c_int,
  pub ai_protocol: libc::c_int,
  pub ai_addrlen: socklen_t,
  pub ai_addr: *mut sockaddr,
  pub ai_canonname: *mut libc::c_char,
  pub ai_next: *mut addrinfo,
}

pub type C2RustUnnamed_3 = libc::c_uint;
pub const LSA_SIZEOF_SA: C2RustUnnamed_3 = 28;
pub const LSA_LEN_SIZE: C2RustUnnamed_3 = 4;

/*
 * Utility routines.
 *
 * Connect to host at port using address resolution from getaddrinfo
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
/* netinet/in.h needs it */
pub unsafe fn setsockopt_int(
  mut fd: libc::c_int,
  mut level: libc::c_int,
  mut optname: libc::c_int,
  mut optval: libc::c_int,
) -> libc::c_int {
  return setsockopt(
    fd,
    level,
    optname,
    &mut optval as *mut libc::c_int as *const libc::c_void,
    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
  );
}
pub unsafe fn setsockopt_1(
  mut fd: libc::c_int,
  mut level: libc::c_int,
  mut optname: libc::c_int,
) -> libc::c_int {
  return setsockopt_int(fd, level, optname, 1i32);
}
pub unsafe fn setsockopt_SOL_SOCKET_int(
  mut fd: libc::c_int,
  mut optname: libc::c_int,
  mut optval: libc::c_int,
) -> libc::c_int {
  return setsockopt_int(fd, 1i32, optname, optval);
}
pub unsafe fn setsockopt_SOL_SOCKET_1(
  mut fd: libc::c_int,
  mut optname: libc::c_int,
) -> libc::c_int {
  return setsockopt_SOL_SOCKET_int(fd, optname, 1i32);
}
pub unsafe fn setsockopt_reuseaddr(mut fd: libc::c_int) {
  setsockopt_SOL_SOCKET_1(fd, 2i32);
}
pub unsafe fn setsockopt_broadcast(mut fd: libc::c_int) -> libc::c_int {
  return setsockopt_SOL_SOCKET_1(fd, 6i32);
}
pub unsafe fn setsockopt_keepalive(mut fd: libc::c_int) -> libc::c_int {
  return setsockopt_SOL_SOCKET_1(fd, 9i32);
}
pub unsafe fn setsockopt_bindtodevice(
  mut fd: libc::c_int,
  mut iface: *const libc::c_char,
) -> libc::c_int {
  let mut r: libc::c_int = 0;
  let mut ifr: ifreq = ifreq {
    ifr_ifrn: C2RustUnnamed_1 { ifrn_name: [0; 16] },
    ifr_ifru: C2RustUnnamed_0 {
      ifru_addr: sockaddr {
        sa_family: 0,
        sa_data: [0; 14],
      },
    },
  };
  crate::libbb::xfuncs::strncpy_IFNAMSIZ(ifr.ifr_ifrn.ifrn_name.as_mut_ptr(), iface);
  /* NB: passing (iface, strlen(iface) + 1) does not work!
   * (maybe it works on _some_ kernels, but not on 2.6.26)
   * Actually, ifr_name is at offset 0, and in practice
   * just giving char[IFNAMSIZ] instead of struct ifreq works too.
   * But just in case it's not true on some obscure arch... */
  r = setsockopt(
    fd,
    1i32,
    25i32,
    &mut ifr as *mut ifreq as *const libc::c_void,
    ::std::mem::size_of::<ifreq>() as libc::c_ulong as socklen_t,
  );
  if r != 0 {
    crate::libbb::perror_msg::bb_perror_msg(
      b"can\'t bind to interface %s\x00" as *const u8 as *const libc::c_char,
      iface,
    );
  }
  return r;
}
unsafe fn get_lsa(
  mut fd: libc::c_int,
  mut get_name: Option<
    unsafe extern "C" fn(_: libc::c_int, _: *mut sockaddr, _: *mut socklen_t) -> libc::c_int,
  >,
) -> *mut len_and_sockaddr {
  let mut lsa: len_and_sockaddr = std::mem::zeroed();
  let mut lsa_ptr: *mut len_and_sockaddr = std::ptr::null_mut();
  lsa.len = LSA_SIZEOF_SA as libc::c_int as socklen_t;
  if get_name.expect("non-null function pointer")(fd, &mut lsa.u.sa, &mut lsa.len) != 0 {
    return std::ptr::null_mut();
  }
  lsa_ptr = crate::libbb::xfuncs_printf::xzalloc(
    (LSA_LEN_SIZE as libc::c_int as libc::c_uint).wrapping_add(lsa.len) as size_t,
  ) as *mut len_and_sockaddr;
  if lsa.len > LSA_SIZEOF_SA as libc::c_int as libc::c_uint {
    /* rarely (if ever) happens */
    (*lsa_ptr).len = lsa.len;
    get_name.expect("non-null function pointer")(fd, &mut (*lsa_ptr).u.sa, &mut (*lsa_ptr).len);
  } else {
    memcpy(
      lsa_ptr as *mut libc::c_void,
      &mut lsa as *mut len_and_sockaddr as *const libc::c_void,
      (LSA_LEN_SIZE as libc::c_int as libc::c_uint).wrapping_add(lsa.len) as libc::c_ulong,
    );
  }
  return lsa_ptr;
}
pub unsafe fn get_sock_lsa(mut fd: libc::c_int) -> *mut len_and_sockaddr {
  return get_lsa(
    fd,
    ::std::mem::transmute::<
      Option<
        unsafe extern "C" fn(_: libc::c_int, _: __SOCKADDR_ARG, _: *mut socklen_t) -> libc::c_int,
      >,
      Option<
        unsafe extern "C" fn(_: libc::c_int, _: *mut sockaddr, _: *mut socklen_t) -> libc::c_int,
      >,
    >(Some(getsockname)),
  );
}
pub unsafe fn get_peer_lsa(mut fd: libc::c_int) -> *mut len_and_sockaddr {
  return get_lsa(
    fd,
    ::std::mem::transmute::<
      Option<
        unsafe extern "C" fn(_: libc::c_int, _: __SOCKADDR_ARG, _: *mut socklen_t) -> libc::c_int,
      >,
      Option<
        unsafe extern "C" fn(_: libc::c_int, _: *mut sockaddr, _: *mut socklen_t) -> libc::c_int,
      >,
    >(Some(getpeername)),
  );
}
pub unsafe fn xconnect(mut s: libc::c_int, mut s_addr: *const sockaddr, mut addrlen: socklen_t) {
  if connect(
    s,
    __CONST_SOCKADDR_ARG {
      __sockaddr__: s_addr,
    },
    addrlen,
  ) < 0
  {
    if (*s_addr).sa_family as libc::c_int == 2i32 {
      crate::libbb::perror_msg::bb_perror_msg_and_die(
        b"%s (%s)\x00" as *const u8 as *const libc::c_char,
        b"can\'t connect to remote host\x00" as *const u8 as *const libc::c_char,
        inet_ntoa((*(s_addr as *mut sockaddr_in)).sin_addr),
      );
    }
    crate::libbb::perror_msg::bb_simple_perror_msg_and_die(
      b"can\'t connect to remote host\x00" as *const u8 as *const libc::c_char,
    );
  };
}
/* Return port number for a service.
 * If "port" is a number use it as the port.
 * If "port" is a name it is looked up in /etc/services,
 * if it isnt found return default_port
 */
pub unsafe fn bb_lookup_port(
  mut port: *const libc::c_char,
  mut protocol: *const libc::c_char,
  mut default_port: libc::c_uint,
) -> libc::c_uint {
  let mut port_nr: libc::c_uint = default_port;
  if !port.is_null() {
    let mut old_errno: libc::c_int = 0;
    /* Since this is a lib function, we're not allowed to reset errno to 0.
     * Doing so could break an app that is deferring checking of errno. */
    old_errno = *bb_errno;
    port_nr = crate::libbb::bb_strtonum::bb_strtou(port, 0 as *mut *mut libc::c_char, 10i32);
    if *bb_errno != 0 || port_nr > 65535i32 as libc::c_uint {
      let mut tserv: *mut servent = getservbyname(port, protocol);
      port_nr = default_port;
      if !tserv.is_null() {
        port_nr = ({
          let mut __v: libc::c_ushort = 0;
          let mut __x: libc::c_ushort = (*tserv).s_port as libc::c_ushort;
          if false {
            __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
              as libc::c_ushort
          } else {
            let fresh0 = &mut __v;
            let fresh1;
            let fresh2 = __x;
            asm!("rorw $$8, ${0:w}" : "=r" (fresh1) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh0, fresh2)) : "cc");
            c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
          }
          __v
        }) as libc::c_uint
      }
      //FIXME: else: port string was garbage, but we don't report that???
    }
    *bb_errno = old_errno
  }
  return port_nr as u16 as libc::c_uint;
}
/* "New" networking API */
pub unsafe fn get_nport(mut sa: *const sockaddr) -> libc::c_int {
  if (*sa).sa_family as libc::c_int == 10i32 {
    return (*(sa as *mut sockaddr_in6)).sin6_port as libc::c_int;
  }
  if (*sa).sa_family as libc::c_int == 2i32 {
    return (*(sa as *mut sockaddr_in)).sin_port as libc::c_int;
  }
  /* What? UNIX socket? IPX?? :) */
  return -1i32;
}
pub unsafe fn set_nport(mut sa: *mut sockaddr, mut port: libc::c_uint) {
  if (*sa).sa_family as libc::c_int == 10i32 {
    let mut sin6: *mut sockaddr_in6 = sa as *mut libc::c_void as *mut sockaddr_in6;
    (*sin6).sin6_port = port as in_port_t;
    return;
  }
  if (*sa).sa_family as libc::c_int == 2i32 {
    let mut sin: *mut sockaddr_in = sa as *mut libc::c_void as *mut sockaddr_in;
    (*sin).sin_port = port as in_port_t;
    return;
  };
  /* What? UNIX socket? IPX?? :) */
}
/* host: "1.2.3.4[:port]", "www.google.com[:port]"
 * port: if neither of above specifies port # */
unsafe fn str2sockaddr(
  mut host: *const libc::c_char,
  mut port: libc::c_int,
  mut af: sa_family_t,
  mut ai_flags: libc::c_int,
) -> *mut len_and_sockaddr {
  let mut current_block: u64; /* only for error msg */
  let mut rc: libc::c_int = 0;
  let mut r: *mut len_and_sockaddr = std::ptr::null_mut();
  let mut result: *mut addrinfo = std::ptr::null_mut();
  let mut used_res: *mut addrinfo = std::ptr::null_mut();
  let mut org_host: *const libc::c_char = host;
  let mut cp: *const libc::c_char = std::ptr::null();
  let mut hint: addrinfo = std::mem::zeroed();
  if 0 != 0
    && !crate::libbb::compare_string_array::is_prefixed_with(
      host,
      b"local:\x00" as *const u8 as *const libc::c_char,
    )
    .is_null()
  {
    let mut sun: *mut sockaddr_un = std::ptr::null_mut();
    r = crate::libbb::xfuncs_printf::xzalloc(
      (LSA_LEN_SIZE as libc::c_int as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<sockaddr_un>() as libc::c_ulong),
    ) as *mut len_and_sockaddr;
    (*r).len = ::std::mem::size_of::<sockaddr_un>() as libc::c_ulong as socklen_t;
    (*r).u.sa.sa_family = 1i32 as sa_family_t;
    sun = &mut (*r).u.sa as *mut sockaddr as *mut sockaddr_un;
    crate::libbb::safe_strncpy::safe_strncpy(
      (*sun).sun_path.as_mut_ptr(),
      host.offset(6),
      ::std::mem::size_of::<[libc::c_char; 108]>() as libc::c_ulong,
    );
    return r;
  }
  r = std::ptr::null_mut();
  /* Ugly parsing of host:addr */
  if 1i32 != 0 && *host.offset(0) as libc::c_int == '[' as i32 {
    /* Even uglier parsing of [xx]:nn */
    host = host.offset(1);
    cp = strchr(host, ']' as i32);
    if cp.is_null()
      || *cp.offset(1) as libc::c_int != ':' as i32
        && *cp.offset(1) as libc::c_int != '\u{0}' as i32
    {
      /* Malformed: must be [xx]:nn or [xx] */
      crate::libbb::verror_msg::bb_error_msg(
        b"bad address \'%s\'\x00" as *const u8 as *const libc::c_char,
        org_host,
      );
      if ai_flags & 0x2i32 != 0 {
        crate::libbb::xfunc_die::xfunc_die();
      }
      return std::ptr::null_mut();
    }
  } else {
    cp = strrchr(host, ':' as i32);
    if 1i32 != 0 && !cp.is_null() && strchr(host, ':' as i32) != cp as *mut libc::c_char {
      /* There is more than one ':' (e.g. "::1") */
      cp = std::ptr::null()
      /* it's not a port spec */
    }
  }
  if !cp.is_null() {
    let mut current_block_34: u64;
    /* points to ":" or "]:" */
    let mut sz: libc::c_int = (cp.wrapping_offset_from(host) as libc::c_long + 1) as libc::c_int; /* skip ']' */
    let mut fresh3 = ::std::vec::from_elem(0, sz as libc::c_ulong as usize); /* skip ':' */
    host = crate::libbb::safe_strncpy::safe_strncpy(
      fresh3.as_mut_ptr() as *mut libc::c_char,
      host,
      sz as size_t,
    );
    if 1i32 != 0 && *cp as libc::c_int != ':' as i32 {
      cp = cp.offset(1);
      if *cp as libc::c_int == '\u{0}' as i32 {
        current_block_34 = 7659304154607701039;
      } else {
        current_block_34 = 8693738493027456495;
      }
    } else {
      current_block_34 = 8693738493027456495;
    }
    match current_block_34 {
      8693738493027456495 => {
        cp = cp.offset(1);
        port = crate::libbb::bb_strtonum::bb_strtou(cp, 0 as *mut *mut libc::c_char, 10i32)
          as libc::c_int;
        if *bb_errno != 0 || port as libc::c_uint > 0xffffi32 as libc::c_uint {
          crate::libbb::verror_msg::bb_error_msg(
            b"bad port spec \'%s\'\x00" as *const u8 as *const libc::c_char,
            org_host,
          );
          if ai_flags & 0x2i32 != 0 {
            crate::libbb::xfunc_die::xfunc_die();
          }
          return std::ptr::null_mut();
        }
      }
      _ => {}
    }
  }
  /* Next two if blocks allow to skip getaddrinfo()
   * in case host name is a numeric IP(v6) address.
   * getaddrinfo() initializes DNS resolution machinery,
   * scans network config and such - tens of syscalls.
   */
  /* If we were not asked specifically for IPv6,
   * check whether this is a numeric IPv4 */
  if af as libc::c_int != 10i32 {
    let mut in4: in_addr = in_addr { s_addr: 0 };
    if inet_aton(host, &mut in4) != 0 {
      r = crate::libbb::xfuncs_printf::xzalloc(
        (LSA_LEN_SIZE as libc::c_int as libc::c_ulong)
          .wrapping_add(::std::mem::size_of::<sockaddr_in>() as libc::c_ulong),
      ) as *mut len_and_sockaddr;
      (*r).len = ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t;
      (*r).u.sa.sa_family = 2i32 as sa_family_t;
      (*r).u.sin.sin_addr = in4;
      current_block = 696780632340298417;
    } else {
      current_block = 1345366029464561491;
    }
  } else {
    current_block = 1345366029464561491;
  }
  match current_block {
    1345366029464561491 =>
    /* If we were not asked specifically for IPv4,
     * check whether this is a numeric IPv6 */
    {
      if af != 2 {
        let mut in6: in6_addr = std::mem::zeroed();
        if inet_pton(10, host, &mut in6 as *mut in6_addr as *mut libc::c_void) > 0 {
          r = crate::libbb::xfuncs_printf::xzalloc(
            (LSA_LEN_SIZE as libc::c_ulong)
              .wrapping_add(::std::mem::size_of::<sockaddr_in6>() as libc::c_ulong),
          ) as *mut len_and_sockaddr;
          (*r).len = ::std::mem::size_of::<sockaddr_in6>() as socklen_t;
          (*r).u.sa.sa_family = 10 as sa_family_t;
          (*r).u.sin6.sin6_addr = libc::in6_addr {
            s6_addr: std::mem::transmute(in6.__in6_u),
          };
          current_block = 696780632340298417;
        } else {
          current_block = 1724319918354933278;
        }
      } else {
        current_block = 1724319918354933278;
      }
      match current_block {
        696780632340298417 => {}
        _ => {
          memset(
            &mut hint as *mut addrinfo as *mut libc::c_void,
            0,
            ::std::mem::size_of::<addrinfo>() as libc::c_ulong,
          );
          hint.ai_family = af as libc::c_int;
          /* Need SOCK_STREAM, or else we get each address thrice (or more)
           * for each possible socket type (tcp,udp,raw...): */
          hint.ai_socktype = SOCK_STREAM as libc::c_int;
          hint.ai_flags = ai_flags & !0x2i32;
          rc = getaddrinfo(host, 0 as *const libc::c_char, &mut hint, &mut result);
          if rc != 0 || result.is_null() {
            crate::libbb::verror_msg::bb_error_msg(
              b"bad address \'%s\'\x00" as *const u8 as *const libc::c_char,
              org_host,
            );
            if ai_flags & 0x2 != 0 {
              crate::libbb::xfunc_die::xfunc_die();
            }
            current_block = 3073517295034021775;
          } else {
            used_res = result;
            while !((*used_res).ai_family == 2) {
              used_res = (*used_res).ai_next;
              if !used_res.is_null() {
                continue;
              }
              used_res = result;
              break;
            }
            r = xmalloc(
              (LSA_LEN_SIZE as libc::c_uint).wrapping_add((*used_res).ai_addrlen) as size_t,
            ) as *mut len_and_sockaddr;
            (*r).len = (*used_res).ai_addrlen;
            memcpy(
              &mut (*r).u.sa as *mut sockaddr as *mut libc::c_void,
              (*used_res).ai_addr as *const libc::c_void,
              (*used_res).ai_addrlen as libc::c_ulong,
            );
            current_block = 696780632340298417;
          }
        }
      }
    }
    _ => {}
  }
  match current_block {
    696780632340298417 => {
      set_nport(
        &mut (*r).u.sa,
        ({
          let mut __v: libc::c_ushort = 0;
          let mut __x: libc::c_ushort = port as libc::c_ushort;
          if false {
            __v =
              (__x as libc::c_int >> 8 & 0xff | (__x as libc::c_int & 0xff) << 8) as libc::c_ushort
          } else {
            let fresh4 = &mut __v;
            let fresh5;
            let fresh6 = __x;
            asm!("rorw $$8, ${0:w}" : "=r" (fresh5) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh4, fresh6)) : "cc");
            c2rust_asm_casts::AsmCast::cast_out(fresh4, fresh6, fresh5);
          }
          __v
        }) as libc::c_uint,
      );
    }
    _ => {}
  }
  if !result.is_null() {
    freeaddrinfo(result);
  }
  return r;
}
pub unsafe fn host_and_af2sockaddr(
  mut host: *const libc::c_char,
  mut port: libc::c_int,
  mut af: sa_family_t,
) -> *mut len_and_sockaddr {
  return str2sockaddr(host, port, af, 0);
}
pub unsafe fn xhost_and_af2sockaddr(
  mut host: *const libc::c_char,
  mut port: libc::c_int,
  mut af: sa_family_t,
) -> *mut len_and_sockaddr {
  return str2sockaddr(host, port, af, 0x2i32);
}
pub unsafe fn host2sockaddr(
  mut host: *const libc::c_char,
  mut port: libc::c_int,
) -> *mut len_and_sockaddr {
  return str2sockaddr(host, port, 0 as sa_family_t, 0);
}
pub unsafe fn xhost2sockaddr(
  mut host: *const libc::c_char,
  mut port: libc::c_int,
) -> *mut len_and_sockaddr {
  return str2sockaddr(host, port, 0 as sa_family_t, 0x2i32);
}
pub unsafe fn xdotted2sockaddr(
  mut host: *const libc::c_char,
  mut port: libc::c_int,
) -> *mut len_and_sockaddr {
  return str2sockaddr(host, port, 0 as sa_family_t, 0x4i32 | 0x2i32);
}
pub unsafe fn xsocket_type(
  mut lsap: *mut *mut len_and_sockaddr,
  mut family: libc::c_int,
  mut sock_type: libc::c_int,
) -> libc::c_int {
  let mut current_block: u64;
  let mut lsa: *mut len_and_sockaddr = std::ptr::null_mut();
  let mut fd: libc::c_int = 0;
  let mut len: libc::c_int = 0;
  if family == 0 {
    fd = socket(10, sock_type, 0);
    if fd >= 0 {
      family = 10;
      current_block = 17911962374867333381;
    } else {
      family = 2;
      current_block = 7351195479953500246;
    }
  } else {
    current_block = 7351195479953500246;
  }
  match current_block {
    7351195479953500246 => {
      fd = crate::libbb::xfuncs_printf::xsocket(family, sock_type, 0);
      len = ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as libc::c_int;
      if family == 1 {
        len = ::std::mem::size_of::<sockaddr_un>() as libc::c_ulong as libc::c_int
      }
      if family == 10 {
        current_block = 17911962374867333381;
      } else {
        current_block = 10599921512955367680;
      }
    }
    _ => {}
  }
  match current_block {
    17911962374867333381 => {
      len = ::std::mem::size_of::<sockaddr_in6>() as libc::c_ulong as libc::c_int
    }
    _ => {}
  }
  lsa = crate::libbb::xfuncs_printf::xzalloc((LSA_LEN_SIZE as libc::c_int + len) as size_t)
    as *mut len_and_sockaddr;
  (*lsa).len = len as socklen_t;
  (*lsa).u.sa.sa_family = family as sa_family_t;
  *lsap = lsa;
  return fd;
}
pub unsafe fn xsocket_stream(mut lsap: *mut *mut len_and_sockaddr) -> libc::c_int {
  return xsocket_type(lsap, 0, SOCK_STREAM as libc::c_int);
}
unsafe fn create_and_bind_or_die(
  mut bindaddr: *const libc::c_char,
  mut port: libc::c_int,
  mut sock_type: libc::c_int,
) -> libc::c_int {
  let mut fd: libc::c_int = 0;
  let mut lsa: *mut len_and_sockaddr = std::ptr::null_mut();
  if !bindaddr.is_null() && *bindaddr.offset(0) as libc::c_int != 0 {
    lsa = xdotted2sockaddr(bindaddr, port);
    /* user specified bind addr dictates family */
    fd = crate::libbb::xfuncs_printf::xsocket((*lsa).u.sa.sa_family as libc::c_int, sock_type, 0)
  } else {
    fd = xsocket_type(&mut lsa, 0, sock_type);
    set_nport(
      &mut (*lsa).u.sa,
      ({
        let mut __v: libc::c_ushort = 0;
        let mut __x: libc::c_ushort = port as libc::c_ushort;
        if false {
          __v =
            (__x as libc::c_int >> 8 & 0xff | (__x as libc::c_int & 0xff) << 8) as libc::c_ushort
        } else {
          let fresh7 = &mut __v;
          let fresh8;
          let fresh9 = __x;
          asm!("rorw $$8, ${0:w}" : "=r" (fresh8) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh7, fresh9)) : "cc");
          c2rust_asm_casts::AsmCast::cast_out(fresh7, fresh9, fresh8);
        }
        __v
      }) as libc::c_uint,
    );
  }
  setsockopt_reuseaddr(fd);
  crate::libbb::xfuncs_printf::xbind(fd, &mut (*lsa).u.sa, (*lsa).len);
  free(lsa as *mut libc::c_void);
  return fd;
}
pub unsafe fn create_and_bind_stream_or_die(
  mut bindaddr: *const libc::c_char,
  mut port: libc::c_int,
) -> libc::c_int {
  return create_and_bind_or_die(bindaddr, port, SOCK_STREAM as libc::c_int);
}
pub unsafe fn create_and_bind_dgram_or_die(
  mut bindaddr: *const libc::c_char,
  mut port: libc::c_int,
) -> libc::c_int {
  return create_and_bind_or_die(bindaddr, port, SOCK_DGRAM as libc::c_int);
}
pub unsafe fn create_and_bind_to_netlink(
  mut proto: libc::c_int,
  mut grp: libc::c_int,
  mut rcvbuf: libc::c_uint,
) -> libc::c_int {
  let mut sa: sockaddr_nl = std::mem::zeroed();
  let mut fd: libc::c_int = 0;
  memset(
    &mut sa as *mut sockaddr_nl as *mut libc::c_void,
    0,
    ::std::mem::size_of::<sockaddr_nl>() as libc::c_ulong,
  );
  sa.nl_family = 16 as __kernel_sa_family_t;
  sa.nl_pid = getpid() as u32;
  sa.nl_groups = grp as u32;
  fd = crate::libbb::xfuncs_printf::xsocket(16, SOCK_DGRAM as libc::c_int, proto);
  crate::libbb::xfuncs_printf::xbind(
    fd,
    &mut sa as *mut sockaddr_nl as *mut sockaddr,
    ::std::mem::size_of::<sockaddr_nl>() as libc::c_ulong as socklen_t,
  );
  crate::libbb::xfuncs::close_on_exec_on(fd);
  if rcvbuf != 0 as libc::c_uint {
    // SO_RCVBUFFORCE (root only) can go above net.core.rmem_max sysctl
    setsockopt_SOL_SOCKET_int(fd, 8, rcvbuf as libc::c_int);
    setsockopt_SOL_SOCKET_int(fd, 33, rcvbuf as libc::c_int);
  }
  return fd;
}
pub unsafe fn create_and_connect_stream_or_die(
  mut peer: *const libc::c_char,
  mut port: libc::c_int,
) -> libc::c_int {
  let mut fd: libc::c_int = 0;
  let mut lsa: *mut len_and_sockaddr = std::ptr::null_mut();
  lsa = xhost2sockaddr(peer, port);
  fd = crate::libbb::xfuncs_printf::xsocket(
    (*lsa).u.sa.sa_family as libc::c_int,
    SOCK_STREAM as libc::c_int,
    0,
  );
  setsockopt_reuseaddr(fd);
  xconnect(fd, &mut (*lsa).u.sa, (*lsa).len);
  free(lsa as *mut libc::c_void);
  return fd;
}
pub unsafe fn xconnect_stream(mut lsa: *const len_and_sockaddr) -> libc::c_int {
  let mut fd: libc::c_int = crate::libbb::xfuncs_printf::xsocket(
    (*lsa).u.sa.sa_family as libc::c_int,
    SOCK_STREAM as libc::c_int,
    0,
  );
  xconnect(fd, &(*lsa).u.sa, (*lsa).len);
  return fd;
}
unsafe fn sockaddr2str(mut sa: *const sockaddr, mut flags: libc::c_int) -> *mut libc::c_char {
  let mut host: [libc::c_char; 128] = [0; 128];
  let mut serv: [libc::c_char; 16] = [0; 16];
  let mut rc: libc::c_int = 0;
  let mut salen: socklen_t = 0;
  if false && (*sa).sa_family as libc::c_int == 1 {
    let mut sun: *mut sockaddr_un = sa as *mut sockaddr_un;
    return crate::libbb::xfuncs_printf::xasprintf(
      b"local:%.*s\x00" as *const u8 as *const libc::c_char,
      ::std::mem::size_of::<[libc::c_char; 108]>() as libc::c_ulong as libc::c_int,
      (*sun).sun_path.as_mut_ptr(),
    );
  }
  salen = LSA_SIZEOF_SA as libc::c_int as socklen_t;
  if (*sa).sa_family as libc::c_int == 2 {
    salen = ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t
  }
  if (*sa).sa_family as libc::c_int == 10 {
    salen = ::std::mem::size_of::<sockaddr_in6>() as libc::c_ulong as socklen_t
  }
  rc = getnameinfo(
    sa,
    salen,
    host.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as socklen_t,
    serv.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as socklen_t,
    flags | 2,
  );
  if rc != 0 {
    return std::ptr::null_mut::<libc::c_char>();
  }
  if flags & 2 != 0 {
    return crate::libbb::xfuncs_printf::xstrdup(host.as_mut_ptr());
  }
  if (*sa).sa_family == 10 {
    if !strchr(host.as_mut_ptr(), ':' as i32).is_null() {
      /* heh, it's not a resolved hostname */
      return crate::libbb::xfuncs_printf::xasprintf(
        b"[%s]:%s\x00" as *const u8 as *const libc::c_char,
        host.as_mut_ptr(),
        serv.as_mut_ptr(),
      );
    }
    /*return xasprintf("%s:%s", host, serv);*/
    /* - fall through instead */
  }
  /* For now we don't support anything else, so it has to be INET */
  /*if (sa->sa_family == AF_INET)*/
  return crate::libbb::xfuncs_printf::xasprintf(
    b"%s:%s\x00" as *const u8 as *const libc::c_char,
    host.as_mut_ptr(),
    serv.as_mut_ptr(),
  );
  /*return xstrdup(host);*/
}
pub unsafe fn xmalloc_sockaddr2host(mut sa: *const sockaddr) -> *mut libc::c_char {
  return sockaddr2str(sa, 0);
}
pub unsafe fn xmalloc_sockaddr2host_noport(mut sa: *const sockaddr) -> *mut libc::c_char {
  return sockaddr2str(sa, 2i32);
}
pub unsafe fn xmalloc_sockaddr2hostonly_noport(mut sa: *const sockaddr) -> *mut libc::c_char {
  return sockaddr2str(sa, 8i32 | 2i32);
}
pub unsafe fn xmalloc_sockaddr2dotted(mut sa: *const sockaddr) -> *mut libc::c_char {
  return sockaddr2str(sa, 1i32 | 0);
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
pub unsafe fn xmalloc_sockaddr2dotted_noport(mut sa: *const sockaddr) -> *mut libc::c_char {
  return sockaddr2str(sa, 1 | 0 | 2);
}
