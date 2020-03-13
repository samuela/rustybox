use crate::libbb::ptr_to_globals::bb_errno;
use crate::librb::in6_addr;
use crate::librb::size_t;
use crate::librb::smallint;
use crate::librb::socklen_t;
use c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use libc;
use libc::fclose;
use libc::free;
use libc::printf;
use libc::sa_family_t;
use libc::sockaddr;
use libc::sockaddr_in;
use libc::sockaddr_in6;
use libc::sscanf;
use libc::stat;
use libc::strcpy;
extern "C" {

  #[no_mangle]
  fn getservbyport(__port: libc::c_int, __proto: *const libc::c_char) -> *mut servent;
  #[no_mangle]
  static mut stdout: *mut FILE;

  #[no_mangle]
  fn snprintf(
    _: *mut libc::c_char,
    _: libc::c_ulong,
    _: *const libc::c_char,
    _: ...
  ) -> libc::c_int;

  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn inet_pton(
    __af: libc::c_int,
    __cp: *const libc::c_char,
    __buf: *mut libc::c_void,
  ) -> libc::c_int;
  #[no_mangle]
  fn inet_ntop(
    __af: libc::c_int,
    __cp: *const libc::c_void,
    __buf: *mut libc::c_char,
    __len: socklen_t,
  ) -> *const libc::c_char;
  /* Some useful definitions */
  /* Macros for min/max.  */
  /* buffer allocation schemes */
  /* glibc uses __errno_location() to get a ptr to errno */
  /* We can just memorize it once - no multithreading in busybox :) */

  /* Simpler version: does not special case "/" string */

  /* This one doesn't append :PORTNUM */

  /* Guaranteed to NOT be a macro (smallest code). Saves nearly 2k on uclibc.
   * But potentially slow, don't use in one-billion-times loops */

  /* Reads up to (and including) TERMINATING_STRING: */

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

  /* { "-", NULL } */
  #[no_mangle]
  static mut option_mask32: u32;

  /* Concatenate path and filename to new allocated buffer.
   * Add "/" only as needed (no duplicate "//" are produced).
   * If path is NULL, it is assumed to be "/".
   * filename should not be NULL. */

  /* '*const' ptr makes gcc optimize code much better.
   * Magic prevents ptr_to_globals from going into rodata.
   * If you want to assign a value, use SET_PTR_TO_GLOBALS(x) */
  #[no_mangle]
  static ptr_to_globals: *mut globals;
}

pub type __socklen_t = libc::c_uint;

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
pub union C2RustUnnamed {
  pub __u6_addr8: [u8; 16],
  pub __u6_addr16: [u16; 8],
  pub __u6_addr32: [u32; 4],
}
pub type in_port_t = u16;

pub type in_addr_t = u32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct servent {
  pub s_name: *mut libc::c_char,
  pub s_aliases: *mut *mut libc::c_char,
  pub s_port: libc::c_int,
  pub s_proto: *mut libc::c_char,
}

use libc::FILE;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const ACTION_DANGLING_OK: C2RustUnnamed_0 = 64;
/*ACTION_REVERSE      = (1 << 4), - unused */
pub const ACTION_QUIET: C2RustUnnamed_0 = 32;
pub const ACTION_DEPTHFIRST: C2RustUnnamed_0 = 8;
pub const ACTION_FOLLOWLINKS_L0: C2RustUnnamed_0 = 4;
pub const ACTION_FOLLOWLINKS: C2RustUnnamed_0 = 2;
pub const ACTION_RECURSE: C2RustUnnamed_0 = 1;
//extern const int const_int_1;
/* This struct is deliberately not defined. */
/* See docs/keep_data_small.txt */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct globals {
  pub flags: smalluint,
  pub prg_cache_loaded: smallint,
  pub prg_hash: [*mut prg_node; 211],
  pub progname_banner: *const libc::c_char,
  pub addr_width: libc::c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prg_node {
  pub next: *mut prg_node,
  pub inode: libc::c_long,
  pub name: [libc::c_char; 20],
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub const OPT_prg: C2RustUnnamed_1 = 1024;
pub const OPT_wide: C2RustUnnamed_1 = 512;
pub const OPT_route: C2RustUnnamed_1 = 256;
pub const OPTBIT_PRG: C2RustUnnamed_1 = 10;
pub const OPTBIT_WIDE: C2RustUnnamed_1 = 9;
pub const OPTBIT_ROUTE: C2RustUnnamed_1 = 8;
pub const OPTBIT_x: C2RustUnnamed_1 = 7;
pub const OPT_sock_unix: C2RustUnnamed_1 = 128;
pub const OPT_sock_raw: C2RustUnnamed_1 = 64;
pub const OPT_sock_udp: C2RustUnnamed_1 = 32;
pub const OPT_sock_tcp: C2RustUnnamed_1 = 16;
pub const OPT_noresolve: C2RustUnnamed_1 = 8;
pub const OPT_extended: C2RustUnnamed_1 = 4;
pub const OPT_sock_all: C2RustUnnamed_1 = 2;
pub const OPT_sock_listen: C2RustUnnamed_1 = 1;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const TCP_CLOSING: C2RustUnnamed_2 = 11;
pub const TCP_LISTEN: C2RustUnnamed_2 = 10;
pub const TCP_LAST_ACK: C2RustUnnamed_2 = 9;
pub const TCP_CLOSE_WAIT: C2RustUnnamed_2 = 8;
pub const TCP_CLOSE: C2RustUnnamed_2 = 7;
pub const TCP_TIME_WAIT: C2RustUnnamed_2 = 6;
pub const TCP_FIN_WAIT2: C2RustUnnamed_2 = 5;
pub const TCP_FIN_WAIT1: C2RustUnnamed_2 = 4;
pub const TCP_SYN_RECV: C2RustUnnamed_2 = 3;
pub const TCP_SYN_SENT: C2RustUnnamed_2 = 2;
pub const TCP_ESTABLISHED: C2RustUnnamed_2 = 1;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const SS_DISCONNECTING: C2RustUnnamed_3 = 4;
pub const SS_CONNECTED: C2RustUnnamed_3 = 3;
pub const SS_CONNECTING: C2RustUnnamed_3 = 2;
pub const SS_UNCONNECTED: C2RustUnnamed_3 = 1;
pub const SS_FREE: C2RustUnnamed_3 = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet_params {
  pub local_port: libc::c_int,
  pub rem_port: libc::c_int,
  pub state: libc::c_int,
  pub uid: libc::c_int,
  pub localaddr: C2RustUnnamed_4,
  pub remaddr: C2RustUnnamed_4,
  pub rxq: libc::c_ulong,
  pub txq: libc::c_ulong,
  pub inode: libc::c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_4 {
  pub sa: sockaddr,
  pub sin: sockaddr_in,
  pub sin6: sockaddr_in6,
}
#[inline(always)]
unsafe extern "C" fn bb_strtoul(
  mut arg: *const libc::c_char,
  mut endp: *mut *mut libc::c_char,
  mut base: libc::c_int,
) -> libc::c_ulong {
  return crate::libbb::bb_strtonum::bb_strtoull(arg, endp, base) as libc::c_ulong;
}
#[inline(always)]
unsafe extern "C" fn not_const_pp(mut p: *const libc::c_void) -> *mut libc::c_void {
  return p as *mut libc::c_void;
}
static mut tcp_state: [*const libc::c_char; 12] = [
  b"\x00" as *const u8 as *const libc::c_char,
  b"ESTABLISHED\x00" as *const u8 as *const libc::c_char,
  b"SYN_SENT\x00" as *const u8 as *const libc::c_char,
  b"SYN_RECV\x00" as *const u8 as *const libc::c_char,
  b"FIN_WAIT1\x00" as *const u8 as *const libc::c_char,
  b"FIN_WAIT2\x00" as *const u8 as *const libc::c_char,
  b"TIME_WAIT\x00" as *const u8 as *const libc::c_char,
  b"CLOSE\x00" as *const u8 as *const libc::c_char,
  b"CLOSE_WAIT\x00" as *const u8 as *const libc::c_char,
  b"LAST_ACK\x00" as *const u8 as *const libc::c_char,
  b"LISTEN\x00" as *const u8 as *const libc::c_char,
  b"CLOSING\x00" as *const u8 as *const libc::c_char,
];
/* Deliberately truncating long to unsigned *int* */
unsafe extern "C" fn prg_cache_add(mut inode: libc::c_long, mut name: *mut libc::c_char) {
  let mut hi: libc::c_uint = (inode as libc::c_uint).wrapping_rem(211i32 as libc::c_uint);
  let mut pnp: *mut *mut prg_node = std::ptr::null_mut();
  let mut pn: *mut prg_node = std::ptr::null_mut();
  (*ptr_to_globals).prg_cache_loaded = 2i32 as smallint;
  pnp = (*ptr_to_globals).prg_hash.as_mut_ptr().offset(hi as isize);
  loop {
    pn = *pnp;
    if pn.is_null() {
      break;
    }
    if (*pn).inode == inode {
      /* Some warning should be appropriate here
       * as we got multiple processes for one i-node */
      return;
    }
    pnp = &mut (*pn).next
  }
  *pnp = crate::libbb::xfuncs_printf::xzalloc(::std::mem::size_of::<prg_node>() as libc::c_ulong)
    as *mut prg_node;
  pn = *pnp;
  (*pn).inode = inode;
  crate::libbb::safe_strncpy::safe_strncpy((*pn).name.as_mut_ptr(), name, 20i32 as size_t);
}
unsafe extern "C" fn prg_cache_get(mut inode: libc::c_long) -> *const libc::c_char {
  let mut hi: libc::c_uint = (inode as libc::c_uint).wrapping_rem(211i32 as libc::c_uint);
  let mut pn: *mut prg_node = std::ptr::null_mut();
  pn = (*ptr_to_globals).prg_hash[hi as usize];
  while !pn.is_null() {
    if (*pn).inode == inode {
      return (*pn).name.as_mut_ptr();
    }
    pn = (*pn).next
  }
  return b"-\x00" as *const u8 as *const libc::c_char;
}
unsafe extern "C" fn extract_socket_inode(mut lname: *const libc::c_char) -> libc::c_long {
  let mut inode: libc::c_long = -1i32 as libc::c_long;
  if !crate::libbb::compare_string_array::is_prefixed_with(
    lname,
    b"socket:[\x00" as *const u8 as *const libc::c_char,
  )
  .is_null()
  {
    /* "socket:[12345]", extract the "12345" as inode */
    inode = bb_strtoul(
      lname
        .offset(::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong as isize)
        .offset(-1),
      &mut lname as *mut *const libc::c_char as *mut *mut libc::c_char,
      0,
    ) as libc::c_long;
    if *lname as libc::c_int != ']' as i32 {
      inode = -1i32 as libc::c_long
    }
  } else if !crate::libbb::compare_string_array::is_prefixed_with(
    lname,
    b"[0000]:\x00" as *const u8 as *const libc::c_char,
  )
  .is_null()
  {
    /* "[0000]:12345", extract the "12345" as inode */
    inode = bb_strtoul(
      lname
        .offset(::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as isize)
        .offset(-1),
      0 as *mut *mut libc::c_char,
      0,
    ) as libc::c_long;
    if *bb_errno != 0 {
      /* not NUL terminated? */
      inode = -1i32 as libc::c_long
    }
  }
  /* bb_strtol returns all-ones bit pattern on ERANGE anyway */
  return inode; /* continue looking one level below /proc */
}
unsafe fn add_to_prg_cache_if_socket(
  mut fileName: *const libc::c_char,
  mut _statbuf: *mut stat,
  mut pid_slash_progname: *mut libc::c_void,
  mut _depth: libc::c_int,
) -> libc::c_int {
  let mut linkname: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut inode: libc::c_long = 0;
  linkname = crate::libbb::xreadlink::xmalloc_readlink(fileName);
  if !linkname.is_null() {
    inode = extract_socket_inode(linkname);
    free(linkname as *mut libc::c_void);
    if inode >= 0 {
      prg_cache_add(inode, pid_slash_progname as *mut libc::c_char);
    }
  }
  return 1i32;
}
unsafe fn dir_act(
  mut fileName: *const libc::c_char,
  mut _statbuf: *mut stat,
  mut _userData: *mut libc::c_void,
  mut depth: libc::c_int,
) -> libc::c_int {
  let mut pid: *const libc::c_char = std::ptr::null();
  let mut pid_slash_progname: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut proc_pid_fname: [libc::c_char; 41] = [0; 41];
  let mut cmdline_buf: [libc::c_char; 512] = [0; 512];
  let mut n: libc::c_int = 0;
  let mut len: libc::c_int = 0;
  if depth == 0 {
    /* "/proc" itself */
    return 1i32;
  } /* point after "/proc/" */
  pid = fileName
    .offset(::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as isize)
    .offset(-1);
  if !((*pid.offset(0) as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32) {
    /* skip /proc entries which aren't processes */
    return 2i32;
  }
  len = snprintf(
    proc_pid_fname.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_char; 41]>() as libc::c_ulong,
    b"%s/cmdline\x00" as *const u8 as *const libc::c_char,
    fileName,
  );
  n = crate::libbb::read::open_read_close(
    proc_pid_fname.as_mut_ptr(),
    cmdline_buf.as_mut_ptr() as *mut libc::c_void,
    (::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong)
      .wrapping_sub(1i32 as libc::c_ulong),
  ) as libc::c_int;
  if n < 0 {
    return 0;
  }
  cmdline_buf[n as usize] = '\u{0}' as i32 as libc::c_char;
  /* go through all files in /proc/PID/fd and check whether they are sockets */
  strcpy(
    proc_pid_fname.as_mut_ptr().offset(len as isize).offset(
      -((::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong) as isize),
    ),
    b"fd\x00" as *const u8 as *const libc::c_char,
  ); /* "PID/argv0" */
  pid_slash_progname = crate::libbb::concat_path_file::concat_path_file(
    pid,
    crate::libbb::get_last_path_component::bb_basename(cmdline_buf.as_mut_ptr()),
  ); /* signal permissions error to caller */
  n = crate::libbb::recursive_action::recursive_action(
    proc_pid_fname.as_mut_ptr(),
    (ACTION_RECURSE as libc::c_int | ACTION_QUIET as libc::c_int) as libc::c_uint,
    Some(add_to_prg_cache_if_socket),
    None,
    pid_slash_progname as *mut libc::c_void,
    0 as libc::c_uint,
  );
  free(pid_slash_progname as *mut libc::c_void);
  if n == 0 {
    return 0;
  }
  return 2i32;
  /* caller should not recurse further into this dir */
}
unsafe extern "C" fn prg_cache_load() {
  let mut load_ok: libc::c_int = 0;
  (*ptr_to_globals).prg_cache_loaded = 1i32 as smallint;
  load_ok = crate::libbb::recursive_action::recursive_action(
    b"/proc\x00" as *const u8 as *const libc::c_char,
    (ACTION_RECURSE as libc::c_int | ACTION_QUIET as libc::c_int) as libc::c_uint,
    None,
    Some(dir_act),
    0 as *mut libc::c_void,
    0 as libc::c_uint,
  );
  if load_ok != 0 {
    return;
  }
  if (*ptr_to_globals).prg_cache_loaded as libc::c_int == 1i32 {
    crate::libbb::verror_msg::bb_simple_error_msg(
      b"can\'t scan /proc - are you root?\x00" as *const u8 as *const libc::c_char,
    );
  } else {
    crate::libbb::verror_msg::bb_simple_error_msg(
      b"showing only processes with your user ID\x00" as *const u8 as *const libc::c_char,
    );
  };
}
//ENABLE_FEATURE_NETSTAT_PRG
unsafe extern "C" fn build_ipv6_addr(
  mut local_addr: *mut libc::c_char,
  mut localaddr: *mut sockaddr_in6,
) {
  let mut addr6: [libc::c_char; 46] = [0; 46];
  let mut in6: in6_addr = std::mem::zeroed();
  sscanf(
    local_addr,
    b"%08X%08X%08X%08X\x00" as *const u8 as *const libc::c_char,
    &mut *in6.__in6_u.__u6_addr32.as_mut_ptr().offset(0) as *mut u32,
    &mut *in6.__in6_u.__u6_addr32.as_mut_ptr().offset(1) as *mut u32,
    &mut *in6.__in6_u.__u6_addr32.as_mut_ptr().offset(2) as *mut u32,
    &mut *in6.__in6_u.__u6_addr32.as_mut_ptr().offset(3) as *mut u32,
  );
  inet_ntop(
    10i32,
    &mut in6 as *mut in6_addr as *const libc::c_void,
    addr6.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong as socklen_t,
  );
  inet_pton(
    10i32,
    addr6.as_mut_ptr(),
    &mut (*localaddr).sin6_addr as *mut libc::in6_addr as *mut libc::c_void,
  );
  (*localaddr).sin6_family = 10i32 as sa_family_t;
}
unsafe extern "C" fn build_ipv4_addr(
  mut local_addr: *mut libc::c_char,
  mut localaddr: *mut sockaddr_in,
) {
  sscanf(
    local_addr,
    b"%X\x00" as *const u8 as *const libc::c_char,
    &mut (*localaddr).sin_addr.s_addr as *mut in_addr_t,
  );
  (*localaddr).sin_family = 2i32 as sa_family_t;
}
unsafe extern "C" fn get_sname(
  mut port: libc::c_int,
  mut proto: *const libc::c_char,
  mut numeric: libc::c_int,
) -> *const libc::c_char {
  if port == 0 {
    return b"*\x00" as *const u8 as *const libc::c_char;
  }
  if numeric == 0 {
    let mut se: *mut servent = getservbyport(port, proto);
    if !se.is_null() {
      return (*se).s_name;
    }
  }
  /* hummm, we may return static buffer here!! */
  return crate::libbb::xfuncs::itoa(
    ({
      let mut __v: libc::c_ushort = 0;
      let mut __x: libc::c_ushort = port as libc::c_ushort;
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
    }) as libc::c_int,
  );
}
unsafe extern "C" fn ip_port_str(
  mut addr: *mut sockaddr,
  mut port: libc::c_int,
  mut proto: *const libc::c_char,
  mut numeric: libc::c_int,
) -> *mut libc::c_char {
  let mut host: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut host_port: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  /* Code which used "*" for INADDR_ANY is removed: it's ambiguous
   * in IPv6, while "0.0.0.0" is not. */
  host = std::ptr::null_mut::<libc::c_char>();
  if numeric == 0 {
    host = crate::libbb::xconnect::xmalloc_sockaddr2host_noport(addr)
  }
  if host.is_null() {
    host = crate::libbb::xconnect::xmalloc_sockaddr2dotted_noport(addr)
  }
  host_port = crate::libbb::xfuncs_printf::xasprintf(
    b"%s:%s\x00" as *const u8 as *const libc::c_char,
    host,
    get_sname(
      ({
        let mut __v: libc::c_ushort = 0;
        let mut __x: libc::c_ushort = port as libc::c_ushort;
        if false {
          __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
            as libc::c_ushort
        } else {
          let fresh3 = &mut __v;
          let fresh4;
          let fresh5 = __x;
          asm!("rorw $$8, ${0:w}" : "=r" (fresh4) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh3, fresh5)) : "cc");
          c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh5, fresh4);
        }
        __v
      }) as libc::c_int,
      proto,
      numeric,
    ),
  );
  free(host as *mut libc::c_void);
  return host_port;
}
unsafe extern "C" fn scan_inet_proc_line(
  mut param: *mut inet_params,
  mut line: *mut libc::c_char,
) -> libc::c_int {
  let mut num: libc::c_int = 0;
  /* IPv6 /proc files use 32-char hex representation
   * of IPv6 address, followed by :PORT_IN_HEX
   */
  let mut local_addr: [libc::c_char; 33] = [0; 33]; /* 32 + 1 for NUL */
  let mut rem_addr: [libc::c_char; 33] = [0; 33];
  num = sscanf(
    line,
    b"%*d: %32[0-9A-Fa-f]:%X %32[0-9A-Fa-f]:%X %X %lX:%lX %*X:%*X %*X %d %*d %lu \x00" as *const u8
      as *const libc::c_char,
    local_addr.as_mut_ptr(),
    &mut (*param).local_port as *mut libc::c_int,
    rem_addr.as_mut_ptr(),
    &mut (*param).rem_port as *mut libc::c_int,
    &mut (*param).state as *mut libc::c_int,
    &mut (*param).txq as *mut libc::c_ulong,
    &mut (*param).rxq as *mut libc::c_ulong,
    &mut (*param).uid as *mut libc::c_int,
    &mut (*param).inode as *mut libc::c_ulong,
  );
  if num < 9i32 {
    return 1i32;
    /* error */
  } /* otherwise we display garbage IPv6 scope_ids */
  if strlen(local_addr.as_mut_ptr()) > 8i32 as libc::c_ulong {
    build_ipv6_addr(local_addr.as_mut_ptr(), &mut (*param).localaddr.sin6);
    build_ipv6_addr(rem_addr.as_mut_ptr(), &mut (*param).remaddr.sin6);
  } else {
    build_ipv4_addr(local_addr.as_mut_ptr(), &mut (*param).localaddr.sin);
    build_ipv4_addr(rem_addr.as_mut_ptr(), &mut (*param).remaddr.sin);
  }
  return 0;
}
unsafe extern "C" fn print_inet_line(
  mut param: *mut inet_params,
  mut state_str: *const libc::c_char,
  mut proto: *const libc::c_char,
  mut is_connected: libc::c_int,
) {
  if is_connected != 0 && (*ptr_to_globals).flags as libc::c_int & 0x1i32 != 0
    || is_connected == 0 && (*ptr_to_globals).flags as libc::c_int & 0x2i32 != 0
  {
    let mut l: *mut libc::c_char = ip_port_str(
      &mut (*param).localaddr.sa,
      (*param).local_port,
      proto,
      (*ptr_to_globals).flags as libc::c_int & 0x4i32,
    );
    let mut r: *mut libc::c_char = ip_port_str(
      &mut (*param).remaddr.sa,
      (*param).rem_port,
      proto,
      (*ptr_to_globals).flags as libc::c_int & 0x4i32,
    );
    printf(
      b"%s   %6lu %6lu %-*s %-*s %-12s\x00" as *const u8 as *const libc::c_char,
      proto,
      (*param).rxq,
      (*param).txq,
      (*ptr_to_globals).addr_width,
      l,
      (*ptr_to_globals).addr_width,
      r,
      state_str,
    );
    if option_mask32 & OPT_prg as libc::c_int as libc::c_uint != 0 {
      printf(
        b"%.20s\x00" as *const u8 as *const libc::c_char,
        prg_cache_get((*param).inode as libc::c_long),
      );
    }
    crate::libbb::xfuncs_printf::bb_putchar('\n' as i32);
    free(l as *mut libc::c_void);
    free(r as *mut libc::c_void);
  };
}
unsafe extern "C" fn tcp_do_one(mut line: *mut libc::c_char) -> libc::c_int {
  let mut param: inet_params = inet_params {
    local_port: 0,
    rem_port: 0,
    state: 0,
    uid: 0,
    localaddr: C2RustUnnamed_4 {
      sa: sockaddr {
        sa_family: 0,
        sa_data: [0; 14],
      },
    },
    remaddr: C2RustUnnamed_4 {
      sa: sockaddr {
        sa_family: 0,
        sa_data: [0; 14],
      },
    },
    rxq: 0,
    txq: 0,
    inode: 0,
  };
  memset(
    &mut param as *mut inet_params as *mut libc::c_void,
    0,
    ::std::mem::size_of::<inet_params>() as libc::c_ulong,
  );
  if scan_inet_proc_line(&mut param, line) != 0 {
    return 1i32;
  }
  print_inet_line(
    &mut param,
    tcp_state[param.state as usize],
    b"tcp\x00" as *const u8 as *const libc::c_char,
    param.rem_port,
  );
  return 0;
}
unsafe extern "C" fn udp_do_one(mut line: *mut libc::c_char) -> libc::c_int {
  let mut have_remaddr: libc::c_int = 0;
  let mut state_str: *const libc::c_char = std::ptr::null();
  let mut param: inet_params = inet_params {
    local_port: 0,
    rem_port: 0,
    state: 0,
    uid: 0,
    localaddr: C2RustUnnamed_4 {
      sa: sockaddr {
        sa_family: 0,
        sa_data: [0; 14],
      },
    },
    remaddr: C2RustUnnamed_4 {
      sa: sockaddr {
        sa_family: 0,
        sa_data: [0; 14],
      },
    },
    rxq: 0,
    txq: 0,
    inode: 0,
  };
  memset(
    &mut param as *mut inet_params as *mut libc::c_void,
    0,
    ::std::mem::size_of::<inet_params>() as libc::c_ulong,
  );
  if scan_inet_proc_line(&mut param, line) != 0 {
    return 1i32;
  }
  state_str = b"UNKNOWN\x00" as *const u8 as *const libc::c_char;
  match param.state {
    1 => state_str = b"ESTABLISHED\x00" as *const u8 as *const libc::c_char,
    7 => state_str = b"\x00" as *const u8 as *const libc::c_char,
    _ => {}
  }
  have_remaddr = (param.remaddr.sa.sa_family == 10
    && param.remaddr.sin6.sin6_addr.s6_addr != [0; 16]
    || param.remaddr.sa.sa_family == 2 && param.remaddr.sin.sin_addr.s_addr != 0)
    as libc::c_int;
  print_inet_line(
    &mut param,
    state_str,
    b"udp\x00" as *const u8 as *const libc::c_char,
    have_remaddr,
  );
  return 0;
}
unsafe extern "C" fn raw_do_one(mut line: *mut libc::c_char) -> libc::c_int {
  let mut have_remaddr: libc::c_int = 0;
  let mut param: inet_params = std::mem::zeroed();
  if scan_inet_proc_line(&mut param, line) != 0 {
    return 1i32;
  }
  have_remaddr = (param.remaddr.sa.sa_family as libc::c_int == 10i32
    && param.remaddr.sin6.sin6_addr.s6_addr != [0; 16]
    || param.remaddr.sa.sa_family as libc::c_int == 2i32
      && param.remaddr.sin.sin_addr.s_addr != 0 as libc::c_uint) as libc::c_int;
  print_inet_line(
    &mut param,
    crate::libbb::xfuncs::itoa(param.state),
    b"raw\x00" as *const u8 as *const libc::c_char,
    have_remaddr,
  );
  return 0;
}
unsafe extern "C" fn unix_do_one(mut line: *mut libc::c_char) -> libc::c_int {
  let mut refcnt: libc::c_ulong = 0;
  let mut proto: libc::c_ulong = 0;
  let mut unix_flags: libc::c_ulong = 0;
  let mut inode: libc::c_ulong = 0;
  let mut type_0: libc::c_int = 0;
  let mut state: libc::c_int = 0;
  let mut num: libc::c_int = 0;
  let mut path_ofs: libc::c_int = 0;
  let mut ss_proto: *const libc::c_char = std::ptr::null();
  let mut ss_state: *const libc::c_char = std::ptr::null();
  let mut ss_type: *const libc::c_char = std::ptr::null();
  let mut ss_flags: [libc::c_char; 32] = [0; 32];
  /* 2.6.15 may report lines like "... @/tmp/fam-user-^@^@^@^@^@^@^@..."
   * Other users report long lines filled by NUL bytes.
   * (those ^@ are NUL bytes too). We see them as empty lines. */
  if *line.offset(0) == 0 {
    return 0;
  } /* paranoia */
  path_ofs = 0;
  num = sscanf(
    line,
    b"%*p: %lX %lX %lX %X %X %lu %n\x00" as *const u8 as *const libc::c_char,
    &mut refcnt as *mut libc::c_ulong,
    &mut proto as *mut libc::c_ulong,
    &mut unix_flags as *mut libc::c_ulong,
    &mut type_0 as *mut libc::c_int,
    &mut state as *mut libc::c_int,
    &mut inode as *mut libc::c_ulong,
    &mut path_ofs as *mut libc::c_int,
  );
  if num < 6i32 {
    return 1i32;
    /* error */
  }
  if (*ptr_to_globals).flags as libc::c_int & (0x2i32 | 0x1i32) != 0x2i32 | 0x1i32 {
    if state == SS_UNCONNECTED as libc::c_int && unix_flags & (1i32 << 16i32) as libc::c_ulong != 0
    {
      if (*ptr_to_globals).flags as libc::c_int & 0x2i32 == 0 {
        return 0;
      }
    } else if (*ptr_to_globals).flags as libc::c_int & 0x1i32 == 0 {
      return 0;
    }
  }
  match proto {
    0 => ss_proto = b"unix\x00" as *const u8 as *const libc::c_char,
    _ => ss_proto = b"??\x00" as *const u8 as *const libc::c_char,
  }
  match type_0 {
    1 => ss_type = b"STREAM\x00" as *const u8 as *const libc::c_char,
    2 => ss_type = b"DGRAM\x00" as *const u8 as *const libc::c_char,
    3 => ss_type = b"RAW\x00" as *const u8 as *const libc::c_char,
    4 => ss_type = b"RDM\x00" as *const u8 as *const libc::c_char,
    5 => ss_type = b"SEQPACKET\x00" as *const u8 as *const libc::c_char,
    _ => ss_type = b"UNKNOWN\x00" as *const u8 as *const libc::c_char,
  }
  match state {
    0 => ss_state = b"FREE\x00" as *const u8 as *const libc::c_char,
    1 => {
      /*
       * Unconnected sockets may be listening
       * for something.
       */
      if unix_flags & (1i32 << 16i32) as libc::c_ulong != 0 {
        ss_state = b"LISTENING\x00" as *const u8 as *const libc::c_char
      } else {
        ss_state = b"\x00" as *const u8 as *const libc::c_char
      }
    }
    2 => ss_state = b"CONNECTING\x00" as *const u8 as *const libc::c_char,
    3 => ss_state = b"CONNECTED\x00" as *const u8 as *const libc::c_char,
    4 => ss_state = b"DISCONNECTING\x00" as *const u8 as *const libc::c_char,
    _ => ss_state = b"UNKNOWN\x00" as *const u8 as *const libc::c_char,
  }
  strcpy(
    ss_flags.as_mut_ptr(),
    b"[ \x00" as *const u8 as *const libc::c_char,
  );
  if unix_flags & (1i32 << 16i32) as libc::c_ulong != 0 {
    strcat(
      ss_flags.as_mut_ptr(),
      b"ACC \x00" as *const u8 as *const libc::c_char,
    );
  }
  if unix_flags & (1i32 << 17i32) as libc::c_ulong != 0 {
    strcat(
      ss_flags.as_mut_ptr(),
      b"W \x00" as *const u8 as *const libc::c_char,
    );
  }
  if unix_flags & (1i32 << 18i32) as libc::c_ulong != 0 {
    strcat(
      ss_flags.as_mut_ptr(),
      b"N \x00" as *const u8 as *const libc::c_char,
    );
  }
  strcat(
    ss_flags.as_mut_ptr(),
    b"]\x00" as *const u8 as *const libc::c_char,
  );
  printf(
    b"%-5s %-6lu %-11s %-10s %-13s %6lu \x00" as *const u8 as *const libc::c_char,
    ss_proto,
    refcnt,
    ss_flags.as_mut_ptr(),
    ss_type,
    ss_state,
    inode,
  );
  if option_mask32 & OPT_prg as libc::c_int as libc::c_uint != 0 {
    printf(
      b"%-20s\x00" as *const u8 as *const libc::c_char,
      prg_cache_get(inode as libc::c_long),
    );
  }
  /* TODO: currently we stop at first NUL byte. Is it a problem? */
  line = line.offset(path_ofs as isize);
  crate::libbb::chomp::chomp(line);
  while *line != 0 {
    let fresh6 = line;
    line = line.offset(1);
    crate::libbb::printable::fputc_printable(*fresh6 as libc::c_int, stdout);
  }
  crate::libbb::xfuncs_printf::bb_putchar('\n' as i32);
  return 0;
}
unsafe extern "C" fn do_info(
  mut file: *const libc::c_char,
  mut proc_0: Option<unsafe extern "C" fn(_: *mut libc::c_char) -> libc::c_int>,
) {
  let mut lnr: libc::c_int = 0;
  let mut procinfo: *mut FILE = std::ptr::null_mut();
  let mut buffer: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  /* _stdin is just to save "r" param */
  procinfo = crate::libbb::wfopen_input::fopen_or_warn_stdin(file);
  if procinfo.is_null() {
    return;
  }
  lnr = 0;
  loop
  /* Why xmalloc_fgets_str? because it doesn't stop on NULs */
  {
    buffer = crate::libbb::fgets_str::xmalloc_fgets_str(
      procinfo,
      b"\n\x00" as *const u8 as *const libc::c_char,
    );
    if buffer.is_null() {
      break;
    }
    /* line 0 is skipped */
    if lnr != 0 && proc_0.expect("non-null function pointer")(buffer) != 0 {
      crate::libbb::verror_msg::bb_error_msg(
        b"%s: bogus data on line %d\x00" as *const u8 as *const libc::c_char,
        file,
        lnr + 1i32,
      );
    }
    lnr += 1;
    free(buffer as *mut libc::c_void);
  }
  fclose(procinfo);
}
pub unsafe fn netstat_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut opt: libc::c_uint = 0;
  let ref mut fresh7 = *(not_const_pp(&ptr_to_globals as *const *mut globals as *const libc::c_void)
    as *mut *mut globals);
  *fresh7 = crate::libbb::xfuncs_printf::xzalloc(::std::mem::size_of::<globals>() as libc::c_ulong)
    as *mut globals;
  asm!("" : : : "memory" : "volatile");
  (*ptr_to_globals).flags = (0x1i32 | (0x10i32 | 0x20i32 | 0x40i32 | 0x80i32)) as smalluint;
  /* Option string must match NETSTAT_xxx constants */
  opt =
    crate::libbb::getopt32::getopt32(argv, b"laentuwxrWp\x00" as *const u8 as *const libc::c_char);
  if opt & OPT_sock_listen as libc::c_int as libc::c_uint != 0 {
    // -l
    (*ptr_to_globals).flags = ((*ptr_to_globals).flags as libc::c_int & !0x1i32) as smalluint; // -a
    (*ptr_to_globals).flags = ((*ptr_to_globals).flags as libc::c_int | 0x2i32) as smalluint
  }
  if opt & OPT_sock_all as libc::c_int as libc::c_uint != 0 {
    (*ptr_to_globals).flags =
      ((*ptr_to_globals).flags as libc::c_int | (0x2i32 | 0x1i32)) as smalluint
  }
  //if (opt & OPT_extended) // -e
  if opt & OPT_noresolve as libc::c_int as libc::c_uint != 0 {
    (*ptr_to_globals).flags = ((*ptr_to_globals).flags as libc::c_int | 0x4i32) as smalluint
  } // -n
    //if (opt & OPT_sock_tcp) // -t: NETSTAT_TCP
    //if (opt & OPT_sock_udp) // -u: NETSTAT_UDP
    //if (opt & OPT_sock_raw) // -w: NETSTAT_RAW
    //if (opt & OPT_sock_unix) // -x: NETSTAT_UNIX
  if opt & OPT_route as libc::c_int as libc::c_uint != 0 {
    // -r
    crate::networking::route::bb_displayroutes(
      (*ptr_to_globals).flags as libc::c_int & 0x4i32,
      (opt & OPT_extended as libc::c_int as libc::c_uint == 0) as libc::c_int,
    );
    return 0;
  }
  (*ptr_to_globals).addr_width = 23i32 as libc::c_uint;
  if opt & OPT_wide as libc::c_int as libc::c_uint != 0 {
    // -W
    (*ptr_to_globals).addr_width = 51i32 as libc::c_uint
  }
  (*ptr_to_globals).progname_banner = b"\x00" as *const u8 as *const libc::c_char;
  if opt & OPT_prg as libc::c_int as libc::c_uint != 0 {
    // -p
    (*ptr_to_globals).progname_banner =
      b"PID/Program name    \x00" as *const u8 as *const libc::c_char; /* xxx */
    prg_cache_load();
  }
  opt &= (0x10i32 | 0x20i32 | 0x40i32 | 0x80i32) as libc::c_uint;
  if opt != 0 {
    (*ptr_to_globals).flags = ((*ptr_to_globals).flags as libc::c_int
      & !(0x10i32 | 0x20i32 | 0x40i32 | 0x80i32)) as smalluint;
    (*ptr_to_globals).flags = ((*ptr_to_globals).flags as libc::c_uint | opt) as smalluint
  }
  if (*ptr_to_globals).flags as libc::c_int & (0x10i32 | 0x20i32 | 0x40i32) != 0 {
    printf(b"Active Internet connections \x00" as *const u8 as *const libc::c_char);
    if (*ptr_to_globals).flags as libc::c_int & (0x2i32 | 0x1i32) == 0x2i32 | 0x1i32 {
      printf(b"(servers and established)\x00" as *const u8 as *const libc::c_char);
    } else if (*ptr_to_globals).flags as libc::c_int & 0x2i32 != 0 {
      printf(b"(only servers)\x00" as *const u8 as *const libc::c_char);
    } else {
      printf(b"(w/o servers)\x00" as *const u8 as *const libc::c_char);
    }
    printf(
      b"\nProto Recv-Q Send-Q %-*s %-*s State       %s\n\x00" as *const u8 as *const libc::c_char,
      (*ptr_to_globals).addr_width,
      b"Local Address\x00" as *const u8 as *const libc::c_char,
      (*ptr_to_globals).addr_width,
      b"Foreign Address\x00" as *const u8 as *const libc::c_char,
      (*ptr_to_globals).progname_banner,
    );
  }
  if (*ptr_to_globals).flags as libc::c_int & 0x10i32 != 0 {
    do_info(
      b"/proc/net/tcp\x00" as *const u8 as *const libc::c_char,
      Some(tcp_do_one as unsafe extern "C" fn(_: *mut libc::c_char) -> libc::c_int),
    );
    do_info(
      b"/proc/net/tcp6\x00" as *const u8 as *const libc::c_char,
      Some(tcp_do_one as unsafe extern "C" fn(_: *mut libc::c_char) -> libc::c_int),
    );
  }
  if (*ptr_to_globals).flags as libc::c_int & 0x20i32 != 0 {
    do_info(
      b"/proc/net/udp\x00" as *const u8 as *const libc::c_char,
      Some(udp_do_one as unsafe extern "C" fn(_: *mut libc::c_char) -> libc::c_int),
    );
    do_info(
      b"/proc/net/udp6\x00" as *const u8 as *const libc::c_char,
      Some(udp_do_one as unsafe extern "C" fn(_: *mut libc::c_char) -> libc::c_int),
    );
  }
  if (*ptr_to_globals).flags as libc::c_int & 0x40i32 != 0 {
    do_info(
      b"/proc/net/raw\x00" as *const u8 as *const libc::c_char,
      Some(raw_do_one as unsafe extern "C" fn(_: *mut libc::c_char) -> libc::c_int),
    );
    do_info(
      b"/proc/net/raw6\x00" as *const u8 as *const libc::c_char,
      Some(raw_do_one as unsafe extern "C" fn(_: *mut libc::c_char) -> libc::c_int),
    );
  }
  if (*ptr_to_globals).flags as libc::c_int & 0x80i32 != 0 {
    printf(b"Active UNIX domain sockets \x00" as *const u8 as *const libc::c_char);
    if (*ptr_to_globals).flags as libc::c_int & (0x2i32 | 0x1i32) == 0x2i32 | 0x1i32 {
      printf(b"(servers and established)\x00" as *const u8 as *const libc::c_char);
    } else if (*ptr_to_globals).flags as libc::c_int & 0x2i32 != 0 {
      printf(b"(only servers)\x00" as *const u8 as *const libc::c_char);
    } else {
      printf(b"(w/o servers)\x00" as *const u8 as *const libc::c_char);
    }
    printf(
      b"\nProto RefCnt Flags       Type       State         I-Node %sPath\n\x00" as *const u8
        as *const libc::c_char,
      (*ptr_to_globals).progname_banner,
    );
    do_info(
      b"/proc/net/unix\x00" as *const u8 as *const libc::c_char,
      Some(unix_do_one as unsafe extern "C" fn(_: *mut libc::c_char) -> libc::c_int),
    );
  }
  return 0;
}
