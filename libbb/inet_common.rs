use libc;












































use libc::strcpy;















use libc::strcmp;







extern "C" {
  #[no_mangle]
  fn gethostbyname(__name: *const libc::c_char) -> *mut hostent;
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
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;


  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn inet_aton(__cp: *const libc::c_char, __inp: *mut in_addr) -> libc::c_int;
  /* Some useful definitions */
  /* Macros for min/max.  */
  /* buffer allocation schemes */
  /* glibc uses __errno_location() to get a ptr to errno */
  /* We can just memorize it once - no multithreading in busybox :) */
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn xmalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  /* This one doesn't append :PORTNUM */
  #[no_mangle]
  fn xmalloc_sockaddr2host_noport(sa: *const sockaddr) -> *mut libc::c_char;
  #[no_mangle]
  fn xmalloc_sockaddr2dotted_noport(sa: *const sockaddr) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);
}

pub type __socklen_t = libc::c_uint;
use crate::librb::size_t;
use crate::librb::smallint;



pub type socklen_t = __socklen_t;
use libc::sa_family_t;
use libc::sockaddr;
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
  pub __in6_u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
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
pub struct hostent {
  pub h_name: *mut libc::c_char,
  pub h_aliases: *mut *mut libc::c_char,
  pub h_addrtype: libc::c_int,
  pub h_length: libc::c_int,
  pub h_addr_list: *mut *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct addr {
  pub next: *mut addr,
  pub nip: u32,
  pub is_host: smallint,
  pub name: [libc::c_char; 1],
}

/*
 * stolen from net-tools-1.59 and stripped down for busybox by
 *                      Erik Andersen <andersen@codepoet.org>
 *
 * Heavily modified by Manuel Novoa III       Mar 12, 2001
 *
 */
/* hostfirst!=0 If we expect this to be a hostname,
  try hostname database first
*/

/*
 * stolen from net-tools-1.59 and stripped down for busybox by
 *                      Erik Andersen <andersen@codepoet.org>
 *
 * Heavily modified by Manuel Novoa III       Mar 12, 2001
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
#[no_mangle]
pub unsafe extern "C" fn INET_resolve(
  mut name: *const libc::c_char,
  mut s_in: *mut sockaddr_in,
  mut hostfirst: libc::c_int,
) -> libc::c_int {
  let mut hp: *mut hostent = 0 as *mut hostent;
  /* Grmpf. -FvK */
  (*s_in).sin_family = 2i32 as sa_family_t;
  (*s_in).sin_port = 0i32 as in_port_t;
  /* Default is special, meaning 0.0.0.0. */
  if strcmp(name, b"default\x00" as *const u8 as *const libc::c_char) == 0i32 {
    (*s_in).sin_addr.s_addr = 0i32 as in_addr_t;
    return 1i32;
  }
  /* Look to see if it's a dotted quad. */
  if inet_aton(name, &mut (*s_in).sin_addr) != 0 {
    return 0i32;
  }
  /* If we expect this to be a hostname, try hostname database first */
  if hostfirst != 0 {
    hp = gethostbyname(name);
    if !hp.is_null() {
      memcpy(
        &mut (*s_in).sin_addr as *mut in_addr as *mut libc::c_void,
        *(*hp).h_addr_list.offset(0) as *const libc::c_void,
        ::std::mem::size_of::<in_addr>() as libc::c_ulong,
      );
      return 0i32;
    }
  }
  if hostfirst != 0 {
    /* Don't try again */
    return -1i32;
  }
  hp = gethostbyname(name);
  if hp.is_null() {
    return -1i32;
  }
  memcpy(
    &mut (*s_in).sin_addr as *mut in_addr as *mut libc::c_void,
    *(*hp).h_addr_list.offset(0) as *const libc::c_void,
    ::std::mem::size_of::<in_addr>() as libc::c_ulong,
  );
  return 0i32;
}
/* These return malloced string */
/* numeric: & 0x8000: "default" instead of "*",
 *          & 0x4000: host instead of net,
 *          & 0x0fff: don't resolve
 */
#[no_mangle]
pub unsafe extern "C" fn INET_rresolve(
  mut s_in: *mut sockaddr_in,
  mut numeric: libc::c_int,
  mut netmask: u32,
) -> *mut libc::c_char {
  /* addr-to-name cache */
  static mut cache: *mut addr = 0 as *const addr as *mut addr; /* no '+ 1', it's already accounted for */
  let mut pn: *mut addr = 0 as *mut addr;
  let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut nip: u32 = 0;
  let mut is_host: smallint = 0;
  if (*s_in).sin_family as libc::c_int != 2i32 {
    *bb_errno = 97i32;
    return 0 as *mut libc::c_char;
  }
  nip = (*s_in).sin_addr.s_addr;
  if numeric & 0xfffi32 != 0 {
    return xmalloc_sockaddr2dotted_noport(s_in as *mut libc::c_void as *const sockaddr);
  }
  if nip == 0i32 as in_addr_t {
    if numeric & 0x8000i32 != 0 {
      return xstrdup(b"default\x00" as *const u8 as *const libc::c_char);
    }
    return xstrdup(b"*\x00" as *const u8 as *const libc::c_char);
  }
  is_host =
    (nip & !netmask != 0i32 as libc::c_uint || numeric & 0x4000i32 != 0) as libc::c_int as smallint;
  pn = cache;
  while !pn.is_null() {
    if (*pn).nip == nip && (*pn).is_host as libc::c_int == is_host as libc::c_int {
      return xstrdup((*pn).name.as_mut_ptr());
    }
    pn = (*pn).next
  }
  name = 0 as *mut libc::c_char;
  if is_host != 0 {
    name = xmalloc_sockaddr2host_noport(s_in as *mut libc::c_void as *const sockaddr)
  }
  if name.is_null() {
    name = xmalloc_sockaddr2dotted_noport(s_in as *mut libc::c_void as *const sockaddr)
  }
  pn = xmalloc((::std::mem::size_of::<addr>() as libc::c_ulong).wrapping_add(strlen(name)))
    as *mut addr;
  (*pn).next = cache;
  (*pn).nip = nip;
  (*pn).is_host = is_host;
  strcpy((*pn).name.as_mut_ptr(), name);
  cache = pn;
  return name;
}
/* numeric: & 0x8000: "default" instead of "*",
 *          & 0x4000: host instead of net,
 *          & 0x0fff: don't resolve
 */
#[no_mangle]
pub unsafe extern "C" fn INET6_resolve(
  mut name: *const libc::c_char,
  mut sin6: *mut sockaddr_in6,
) -> libc::c_int {
  let mut req: addrinfo = addrinfo {
    ai_flags: 0,
    ai_family: 0,
    ai_socktype: 0,
    ai_protocol: 0,
    ai_addrlen: 0,
    ai_addr: 0 as *mut sockaddr,
    ai_canonname: 0 as *mut libc::c_char,
    ai_next: 0 as *mut addrinfo,
  };
  let mut ai: *mut addrinfo = 0 as *mut addrinfo;
  let mut s: libc::c_int = 0;
  memset(
    &mut req as *mut addrinfo as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<addrinfo>() as libc::c_ulong,
  );
  req.ai_family = 10i32;
  s = getaddrinfo(name, 0 as *const libc::c_char, &mut req, &mut ai);
  if s != 0i32 {
    bb_error_msg(
      b"getaddrinfo: %s: %d\x00" as *const u8 as *const libc::c_char,
      name,
      s,
    );
    return -1i32;
  }
  memcpy(
    sin6 as *mut libc::c_void,
    (*ai).ai_addr as *const libc::c_void,
    ::std::mem::size_of::<sockaddr_in6>() as libc::c_ulong,
  );
  freeaddrinfo(ai);
  return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn INET6_rresolve(
  mut sin6: *mut sockaddr_in6,
  mut numeric: libc::c_int,
) -> *mut libc::c_char {
  if (*sin6).sin6_family as libc::c_int != 10i32 {
    *bb_errno = 97i32;
    return 0 as *mut libc::c_char;
  }
  if numeric & 0x7fffi32 != 0 {
    return xmalloc_sockaddr2dotted_noport(sin6 as *mut libc::c_void as *const sockaddr);
  }
  if ({
    let mut __a: *const in6_addr = &mut (*sin6).sin6_addr as *mut in6_addr as *const in6_addr;
    ((*__a).__in6_u.__u6_addr32[0] == 0i32 as libc::c_uint
      && (*__a).__in6_u.__u6_addr32[1] == 0i32 as libc::c_uint
      && (*__a).__in6_u.__u6_addr32[2] == 0i32 as libc::c_uint
      && (*__a).__in6_u.__u6_addr32[3] == 0i32 as libc::c_uint) as libc::c_int
  }) != 0
  {
    if numeric & 0x8000i32 != 0 {
      return xstrdup(b"default\x00" as *const u8 as *const libc::c_char);
    }
    return xstrdup(b"*\x00" as *const u8 as *const libc::c_char);
  }
  return xmalloc_sockaddr2host_noport(sin6 as *mut libc::c_void as *const sockaddr);
}
/* CONFIG_FEATURE_IPV6 */
