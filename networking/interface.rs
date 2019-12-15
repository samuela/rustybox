use crate::libbb::ptr_to_globals::bb_errno;
use crate::libbb::skip_whitespace::skip_whitespace;
use crate::libbb::xfuncs_printf::xmalloc;
use libc;
use libc::close;
use libc::fclose;
use libc::free;
use libc::fscanf;
use libc::ioctl;
use libc::printf;
use libc::puts;
use libc::sprintf;
use libc::sscanf;
use libc::strcmp;
use libc::strstr;
extern "C" {
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  fn fgets_unlocked(
    __s: *mut libc::c_char,
    __n: libc::c_int,
    __stream: *mut FILE,
  ) -> *mut libc::c_char;

  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;

  #[no_mangle]
  fn strerror(_: libc::c_int) -> *mut libc::c_char;

  #[no_mangle]
  fn inet_pton(
    __af: libc::c_int,
    __cp: *const libc::c_char,
    __buf: *mut libc::c_void,
  ) -> libc::c_int;

  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xrealloc(old: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xsocket(domain: libc::c_int, type_0: libc::c_int, protocol: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn strncpy_IFNAMSIZ(dst: *mut libc::c_char, src: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  #[no_mangle]
  fn auto_string(str: *mut libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn fopen_or_warn(filename: *const libc::c_char, mode: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn fopen_for_read(path: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn hex2bin(
    dst: *mut libc::c_char,
    src: *const libc::c_char,
    count: libc::c_int,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_xioctl(
    fd: libc::c_int,
    request: libc::c_uint,
    argp: *mut libc::c_void,
    ioctl_name: *const libc::c_char,
  ) -> libc::c_int;
  #[no_mangle]
  fn in_ether(bufp: *const libc::c_char, sap: *mut sockaddr) -> libc::c_int;

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
  #[no_mangle]
  fn INET_resolve(
    name: *const libc::c_char,
    s_in: *mut sockaddr_in,
    hostfirst: libc::c_int,
  ) -> libc::c_int;
  /* numeric: & 0x8000: "default" instead of "*",
   *          & 0x4000: host instead of net,
   *          & 0x0fff: don't resolve
   */
  #[no_mangle]
  fn INET6_resolve(name: *const libc::c_char, sin6: *mut sockaddr_in6) -> libc::c_int;
  /* These return malloced string */
  #[no_mangle]
  fn INET_rresolve(s_in: *mut sockaddr_in, numeric: libc::c_int, netmask: u32)
    -> *mut libc::c_char;
  #[no_mangle]
  fn INET6_rresolve(sin6: *mut sockaddr_in6, numeric: libc::c_int) -> *mut libc::c_char;
}

pub type __caddr_t = *mut libc::c_char;

pub type intptr_t = libc::c_long;
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

use libc::FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aftype {
  pub name: *const libc::c_char,
  pub title: *const libc::c_char,
  pub af: libc::c_int,
  pub alen: libc::c_int,
  pub print: Option<unsafe extern "C" fn(_: *mut libc::c_uchar) -> *mut libc::c_char>,
  pub sprint: Option<unsafe extern "C" fn(_: *mut sockaddr, _: libc::c_int) -> *const libc::c_char>,
  pub input: Option<unsafe extern "C" fn(_: *const libc::c_char, _: *mut sockaddr) -> libc::c_int>,
  pub herror: Option<unsafe extern "C" fn(_: *mut libc::c_char) -> ()>,
  pub rprint: Option<unsafe extern "C" fn(_: libc::c_int) -> libc::c_int>,
  pub rinput: Option<
    unsafe extern "C" fn(_: libc::c_int, _: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
  >,
  pub getmask: Option<
    unsafe extern "C" fn(
      _: *mut libc::c_char,
      _: *mut sockaddr,
      _: *mut libc::c_char,
    ) -> libc::c_int,
  >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hwtype {
  pub name: *const libc::c_char,
  pub title: *const libc::c_char,
  pub type_0: libc::c_int,
  pub alen: libc::c_int,
  pub print: Option<unsafe extern "C" fn(_: *mut libc::c_uchar) -> *mut libc::c_char>,
  pub input: Option<unsafe extern "C" fn(_: *const libc::c_char, _: *mut sockaddr) -> libc::c_int>,
  pub activate: Option<unsafe extern "C" fn(_: libc::c_int) -> libc::c_int>,
  pub suppress_null_addr: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct interface {
  pub next: *mut interface,
  pub prev: *mut interface,
  pub name: [libc::c_char; 16],
  pub type_0: libc::c_short,
  pub flags: libc::c_short,
  pub tx_queue_len: libc::c_int,
  pub metric: libc::c_int,
  pub mtu: libc::c_int,
  pub map: ifmap,
  pub addr: sockaddr,
  pub dstaddr: sockaddr,
  pub broadaddr: sockaddr,
  pub netmask: sockaddr,
  pub hwaddr: [libc::c_char; 32],
  pub has_ip: smallint,
  pub statistics_valid: smallint,
  pub stats: user_net_device_stats,
  /* statistics            */
  /* UNUSED */
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct user_net_device_stats {
  pub rx_packets: libc::c_ulonglong,
  pub tx_packets: libc::c_ulonglong,
  pub rx_bytes: libc::c_ulonglong,
  pub tx_bytes: libc::c_ulonglong,
  pub rx_errors: libc::c_ulong,
  pub tx_errors: libc::c_ulong,
  pub rx_dropped: libc::c_ulong,
  pub tx_dropped: libc::c_ulong,
  pub rx_multicast: libc::c_ulong,
  pub rx_compressed: libc::c_ulong,
  pub tx_compressed: libc::c_ulong,
  pub collisions: libc::c_ulong,
  pub rx_length_errors: libc::c_ulong,
  pub rx_over_errors: libc::c_ulong,
  pub rx_crc_errors: libc::c_ulong,
  pub rx_frame_errors: libc::c_ulong,
  pub rx_fifo_errors: libc::c_ulong,
  pub rx_missed_errors: libc::c_ulong,
  pub tx_aborted_errors: libc::c_ulong,
  pub tx_carrier_errors: libc::c_ulong,
  pub tx_fifo_errors: libc::c_ulong,
  pub tx_heartbeat_errors: libc::c_ulong,
  pub tx_window_errors: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifmap {
  pub mem_start: libc::c_ulong,
  pub mem_end: libc::c_ulong,
  pub base_addr: libc::c_ushort,
  pub irq: libc::c_uchar,
  pub dma: libc::c_uchar,
  pub port: libc::c_uchar,
}
pub const IFF_MULTICAST: C2RustUnnamed_3 = 4096;
pub const IFF_MASTER: C2RustUnnamed_3 = 1024;
pub const IFF_SLAVE: C2RustUnnamed_3 = 2048;
pub const IFF_ALLMULTI: C2RustUnnamed_3 = 512;
pub const IFF_PROMISC: C2RustUnnamed_3 = 256;
pub const IFF_NOARP: C2RustUnnamed_3 = 128;
pub const IFF_RUNNING: C2RustUnnamed_3 = 64;
pub const IFF_NOTRAILERS: C2RustUnnamed_3 = 32;
pub const IFF_POINTOPOINT: C2RustUnnamed_3 = 16;
pub const IFF_LOOPBACK: C2RustUnnamed_3 = 8;
pub const IFF_DEBUG: C2RustUnnamed_3 = 4;
pub const IFF_BROADCAST: C2RustUnnamed_3 = 2;
pub const IFF_UP: C2RustUnnamed_3 = 1;
pub const IFF_AUTOMEDIA: C2RustUnnamed_3 = 16384;
pub const IFF_PORTSEL: C2RustUnnamed_3 = 8192;
#[derive(Copy, Clone)]
#[repr(C)]
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifreq {
  pub ifr_ifrn: C2RustUnnamed_1,
  pub ifr_ifru: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
  pub ifrn_name: [libc::c_char; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iface_list {
  pub int_list: *mut interface,
  pub int_last: *mut interface,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
  pub ifcu_buf: __caddr_t,
  pub ifcu_req: *mut ifreq,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifconf {
  pub ifc_len: libc::c_int,
  pub ifc_ifcu: C2RustUnnamed_2,
}
pub type C2RustUnnamed_3 = libc::c_uint;
pub const IFF_DYNAMIC: C2RustUnnamed_3 = 32768;
/* HAVE_AFINET6 */
/* Defines for glibc2.0 users. */
/* ifr_qlen is ifru_ivalue, but it isn't present in 2.0 kernel headers */
/* Display an Internet socket address. */
unsafe extern "C" fn INET_sprint(
  mut sap: *mut sockaddr,
  mut numeric: libc::c_int,
) -> *const libc::c_char {
  if (*sap).sa_family as libc::c_int == 0xffffi32 || (*sap).sa_family as libc::c_int == 0i32 {
    return b"[NONE SET]\x00" as *const u8 as *const libc::c_char;
  }
  return auto_string(INET_rresolve(
    sap as *mut sockaddr_in,
    numeric,
    0xffffff00u32,
  ));
}
unsafe extern "C" fn INET_input(
  mut bufp: *const libc::c_char,
  mut sap: *mut sockaddr,
) -> libc::c_int {
  return INET_resolve(bufp, sap as *mut sockaddr_in, 0i32);
  /*
    switch (type) {
    case 1:
      return (INET_getsock(bufp, sap));
    case 256:
      return (INET_resolve(bufp, (struct sockaddr_in *) sap, 1));
    default:
      return (INET_resolve(bufp, (struct sockaddr_in *) sap, 0));
    }
  */
}

static mut inet_aftype: aftype = {
  let mut init = aftype {
    name: b"inet\x00" as *const u8 as *const libc::c_char,
    title: b"DARPA Internet\x00" as *const u8 as *const libc::c_char,
    af: 2i32,
    alen: 4i32,
    print: None,
    sprint: Some(
      INET_sprint as unsafe extern "C" fn(_: *mut sockaddr, _: libc::c_int) -> *const libc::c_char,
    ),
    input: Some(
      INET_input as unsafe extern "C" fn(_: *const libc::c_char, _: *mut sockaddr) -> libc::c_int,
    ),
    herror: None,
    rprint: None,
    rinput: None,
    getmask: None,
  };
  init
};

/* Display an Internet socket address. */
/* dirty! struct sockaddr usually doesn't suffer for inet6 addresses, fst. */
unsafe extern "C" fn INET6_sprint(
  mut sap: *mut sockaddr,
  mut numeric: libc::c_int,
) -> *const libc::c_char {
  if (*sap).sa_family as libc::c_int == 0xffffi32 || (*sap).sa_family as libc::c_int == 0i32 {
    return b"[NONE SET]\x00" as *const u8 as *const libc::c_char;
  }
  return auto_string(INET6_rresolve(sap as *mut sockaddr_in6, numeric));
}

unsafe extern "C" fn INET6_input(
  mut bufp: *const libc::c_char,
  mut sap: *mut sockaddr,
) -> libc::c_int {
  return INET6_resolve(bufp, sap as *mut sockaddr_in6);
  /*
    switch (type) {
    case 1:
      return (INET6_getsock(bufp, sap));
    default:
      return (INET6_resolve(bufp, (struct sockaddr_in6 *) sap));
    }
  */
}

static mut inet6_aftype: aftype = {
  let mut init = aftype {
    name: b"inet6\x00" as *const u8 as *const libc::c_char,
    title: b"IPv6\x00" as *const u8 as *const libc::c_char,
    af: 10i32,
    alen: ::std::mem::size_of::<in6_addr>() as libc::c_ulong as libc::c_int,
    print: None,
    sprint: Some(
      INET6_sprint as unsafe extern "C" fn(_: *mut sockaddr, _: libc::c_int) -> *const libc::c_char,
    ),
    input: Some(
      INET6_input as unsafe extern "C" fn(_: *const libc::c_char, _: *mut sockaddr) -> libc::c_int,
    ),
    herror: None,
    rprint: None,
    rinput: None,
    getmask: None,
  };
  init
};

/* HAVE_AFINET6 */
/* Display an UNSPEC address. */
unsafe extern "C" fn UNSPEC_print(mut ptr: *mut libc::c_uchar) -> *mut libc::c_char {
  let mut buff: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut pos: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut i: libc::c_uint = 0;
  buff = auto_string(xmalloc(
    (::std::mem::size_of::<sockaddr>() as libc::c_ulong)
      .wrapping_mul(3i32 as libc::c_ulong)
      .wrapping_add(1i32 as libc::c_ulong),
  ) as *mut libc::c_char);
  pos = buff;
  i = 0i32 as libc::c_uint;
  while (i as libc::c_ulong) < ::std::mem::size_of::<sockaddr>() as libc::c_ulong {
    /* careful -- not every libc's sprintf returns # bytes written */
    let fresh0 = ptr;
    ptr = ptr.offset(1);
    sprintf(
      pos,
      b"%02X-\x00" as *const u8 as *const libc::c_char,
      *fresh0 as libc::c_int,
    );
    pos = pos.offset(3);
    i = i.wrapping_add(1)
  }
  /* Erase trailing "-".  Works as long as sizeof(struct sockaddr) != 0 */
  pos = pos.offset(-1);
  *pos = '\u{0}' as i32 as libc::c_char;
  return buff;
}
/* Display an UNSPEC socket address. */
unsafe extern "C" fn UNSPEC_sprint(
  mut sap: *mut sockaddr,
  mut _numeric: libc::c_int,
) -> *const libc::c_char {
  if (*sap).sa_family as libc::c_int == 0xffffi32 || (*sap).sa_family as libc::c_int == 0i32 {
    return b"[NONE SET]\x00" as *const u8 as *const libc::c_char;
  }
  return UNSPEC_print((*sap).sa_data.as_mut_ptr() as *mut libc::c_uchar);
}

static mut unspec_aftype: aftype = {
  let mut init = aftype {
    name: b"unspec\x00" as *const u8 as *const libc::c_char,
    title: b"UNSPEC\x00" as *const u8 as *const libc::c_char,
    af: 0i32,
    alen: 0i32,
    print: Some(UNSPEC_print as unsafe extern "C" fn(_: *mut libc::c_uchar) -> *mut libc::c_char),
    sprint: Some(
      UNSPEC_sprint
        as unsafe extern "C" fn(_: *mut sockaddr, _: libc::c_int) -> *const libc::c_char,
    ),
    input: None,
    herror: None,
    rprint: None,
    rinput: None,
    getmask: None,
  };
  init
};

static mut aftypes: [*const aftype; 4] = unsafe {
  [
    &inet_aftype as *const aftype,
    &inet6_aftype as *const aftype,
    &unspec_aftype as *const aftype,
    0 as *const aftype,
  ]
};
/* Check our protocol family table for this family. */
#[no_mangle]
pub unsafe extern "C" fn get_aftype(mut name: *const libc::c_char) -> *const aftype {
  let mut afp: *const *const aftype = 0 as *const *const aftype;
  afp = aftypes.as_ptr();
  while !(*afp).is_null() {
    if strcmp((**afp).name, name) == 0i32 {
      return *afp;
    }
    afp = afp.offset(1)
  }
  return 0 as *const aftype;
}
/* Check our protocol family table for this family. */
unsafe extern "C" fn get_afntype(mut af: libc::c_int) -> *const aftype {
  let mut afp: *const *const aftype = 0 as *const *const aftype;
  afp = aftypes.as_ptr();
  while !(*afp).is_null() {
    if (**afp).af == af {
      return *afp;
    }
    afp = afp.offset(1)
  }
  return 0 as *const aftype;
}
unsafe extern "C" fn add_interface(
  mut ilist: *mut iface_list,
  mut name: *mut libc::c_char,
) -> *mut interface {
  let mut ife: *mut interface = std::ptr::null_mut();
  let mut nextp: *mut *mut interface = std::ptr::null_mut();
  let mut new: *mut interface = std::ptr::null_mut();
  ife = (*ilist).int_last;
  while !ife.is_null() {
    let mut n: libc::c_int = strcmp((*ife).name.as_mut_ptr(), name);
    if n == 0i32 {
      return ife;
    }
    if n < 0i32 {
      break;
    }
    ife = (*ife).prev
  }
  new = xzalloc(::std::mem::size_of::<interface>() as libc::c_ulong) as *mut interface;
  strncpy_IFNAMSIZ((*new).name.as_mut_ptr(), name);
  nextp = if !ife.is_null() {
    &mut (*ife).next
  } else {
    &mut (*ilist).int_list
  };
  (*new).prev = ife;
  (*new).next = *nextp;
  if !(*new).next.is_null() {
    (*(*new).next).prev = new
  } else {
    (*ilist).int_last = new
  }
  *nextp = new;
  return new;
}
unsafe extern "C" fn get_name(
  mut name: *mut libc::c_char,
  mut p: *mut libc::c_char,
) -> *mut libc::c_char {
  /* Extract NAME from nul-terminated p of the form "<whitespace>NAME:"
   * If match is not made, set NAME to "" and return unchanged p.
   */
  let mut nameend: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>(); /* interface name too large - return "" */
  let mut namestart: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  namestart = skip_whitespace(p);
  nameend = namestart;
  while !(nameend.wrapping_offset_from(namestart) as libc::c_long >= 16i32 as libc::c_long) {
    if *nameend as libc::c_int == ':' as i32 {
      memcpy(
        name as *mut libc::c_void,
        namestart as *const libc::c_void,
        nameend.wrapping_offset_from(namestart) as libc::c_long as libc::c_ulong,
      );
      *name.offset(nameend.wrapping_offset_from(namestart) as libc::c_long as isize) =
        '\u{0}' as i32 as libc::c_char;
      return nameend.offset(1);
    }
    nameend = nameend.offset(1);
    /* isspace, NUL, any control char? */
    if *nameend as libc::c_uchar as libc::c_int <= ' ' as i32 as libc::c_uchar as libc::c_int {
      break;
    }
  }
  *name.offset(0) = '\u{0}' as i32 as libc::c_char;
  return p;
}
/* If scanf supports size qualifiers for %n conversions, then we can
 * use a modified fmt that simply stores the position in the fields
 * having no associated fields in the proc string.  Of course, we need
 * to zero them again when we're done.  But that is smaller than the
 * old approach of multiple scanf occurrences with large numbers of
 * args. */
/* static const char *const ss_fmt[] = { */
/*	"%lln%llu%lu%lu%lu%lu%ln%ln%lln%llu%lu%lu%lu%lu%lu", */
/*	"%llu%llu%lu%lu%lu%lu%ln%ln%llu%llu%lu%lu%lu%lu%lu", */
/*	"%llu%llu%lu%lu%lu%lu%lu%lu%llu%llu%lu%lu%lu%lu%lu%lu" */
/* }; */
/* We use %n for unavailable data in older versions of /proc/net/dev formats.
 * This results in bogus stores to ife->FOO members corresponding to
 * %n specifiers (even the size of integers may not match).
 */
static mut ss_fmt: [*const libc::c_char; 3] = [
  b"%n%llu%lu%lu%lu%lu%n%n%n%llu%lu%lu%lu%lu%lu\x00" as *const u8 as *const libc::c_char,
  b"%llu%llu%lu%lu%lu%lu%n%n%llu%llu%lu%lu%lu%lu%lu\x00" as *const u8 as *const libc::c_char,
  b"%llu%llu%lu%lu%lu%lu%lu%lu%llu%llu%lu%lu%lu%lu%lu%lu\x00" as *const u8 as *const libc::c_char,
];
unsafe extern "C" fn get_dev_fields(
  mut bp: *mut libc::c_char,
  mut ife: *mut interface,
  mut procnetdev_vsn: libc::c_int,
) {
  memset(
    &mut (*ife).stats as *mut user_net_device_stats as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<user_net_device_stats>() as libc::c_ulong,
  );
  sscanf(
    bp,
    ss_fmt[procnetdev_vsn as usize],
    &mut (*ife).stats.rx_bytes as *mut libc::c_ulonglong,
    &mut (*ife).stats.rx_packets as *mut libc::c_ulonglong,
    &mut (*ife).stats.rx_errors as *mut libc::c_ulong,
    &mut (*ife).stats.rx_dropped as *mut libc::c_ulong,
    &mut (*ife).stats.rx_fifo_errors as *mut libc::c_ulong,
    &mut (*ife).stats.rx_frame_errors as *mut libc::c_ulong,
    &mut (*ife).stats.rx_compressed as *mut libc::c_ulong,
    &mut (*ife).stats.rx_multicast as *mut libc::c_ulong,
    &mut (*ife).stats.tx_bytes as *mut libc::c_ulonglong,
    &mut (*ife).stats.tx_packets as *mut libc::c_ulonglong,
    &mut (*ife).stats.tx_errors as *mut libc::c_ulong,
    &mut (*ife).stats.tx_dropped as *mut libc::c_ulong,
    &mut (*ife).stats.tx_fifo_errors as *mut libc::c_ulong,
    &mut (*ife).stats.collisions as *mut libc::c_ulong,
    &mut (*ife).stats.tx_carrier_errors as *mut libc::c_ulong,
    &mut (*ife).stats.tx_compressed as *mut libc::c_ulong,
  );
  if procnetdev_vsn <= 1i32 {
    if procnetdev_vsn == 0i32 {
      (*ife).stats.rx_bytes = 0i32 as libc::c_ulonglong;
      (*ife).stats.tx_bytes = 0i32 as libc::c_ulonglong
    }
    (*ife).stats.rx_multicast = 0i32 as libc::c_ulong;
    (*ife).stats.rx_compressed = 0i32 as libc::c_ulong;
    (*ife).stats.tx_compressed = 0i32 as libc::c_ulong
  };
}
unsafe extern "C" fn procnetdev_version(mut buf: *mut libc::c_char) -> libc::c_int {
  if !strstr(buf, b"compressed\x00" as *const u8 as *const libc::c_char).is_null() {
    return 2i32;
  }
  if !strstr(buf, b"bytes\x00" as *const u8 as *const libc::c_char).is_null() {
    return 1i32;
  }
  return 0i32;
}
unsafe extern "C" fn if_readconf(mut ilist: *mut iface_list) {
  let mut numreqs: libc::c_int = 30i32;
  let mut ifc: ifconf = ifconf {
    ifc_len: 0,
    ifc_ifcu: C2RustUnnamed_2 {
      ifcu_buf: std::ptr::null_mut::<libc::c_char>(),
    },
  };
  let mut ifr: *mut ifreq = std::ptr::null_mut();
  let mut n: libc::c_int = 0;
  let mut skfd: libc::c_int = 0;
  ifc.ifc_ifcu.ifcu_buf = 0 as __caddr_t;
  /* SIOCGIFCONF currently seems to only work properly on AF_INET sockets
  (as of 2.1.128) */
  skfd = xsocket(2i32, SOCK_DGRAM as libc::c_int, 0i32);
  loop {
    ifc.ifc_len = (::std::mem::size_of::<ifreq>() as libc::c_ulong)
      .wrapping_mul(numreqs as libc::c_ulong) as libc::c_int;
    ifc.ifc_ifcu.ifcu_buf = xrealloc(
      ifc.ifc_ifcu.ifcu_buf as *mut libc::c_void,
      ifc.ifc_len as size_t,
    ) as __caddr_t;
    bb_xioctl(
      skfd,
      0x8912i32 as libc::c_uint,
      &mut ifc as *mut ifconf as *mut libc::c_void,
      b"SIOCGIFCONF\x00" as *const u8 as *const libc::c_char,
    );
    if !(ifc.ifc_len
      == (::std::mem::size_of::<ifreq>() as libc::c_ulong).wrapping_mul(numreqs as libc::c_ulong)
        as libc::c_int)
    {
      break;
    }
    /* assume it overflowed and try again */
    numreqs += 10i32
  }
  ifr = ifc.ifc_ifcu.ifcu_req;
  n = 0i32;
  while n < ifc.ifc_len {
    add_interface(ilist, (*ifr).ifr_ifrn.ifrn_name.as_mut_ptr());
    ifr = ifr.offset(1);
    n = (n as libc::c_ulong).wrapping_add(::std::mem::size_of::<ifreq>() as libc::c_ulong)
      as libc::c_int as libc::c_int
  }
  close(skfd);
  free(ifc.ifc_ifcu.ifcu_buf as *mut libc::c_void);
}
unsafe extern "C" fn if_readlist_proc(
  mut ilist: *mut iface_list,
  mut ifname: *mut libc::c_char,
) -> libc::c_int {
  let mut fh: *mut FILE = std::ptr::null_mut();
  let mut buf: [libc::c_char; 512] = [0; 512];
  let mut ife: *mut interface = std::ptr::null_mut();
  let mut procnetdev_vsn: libc::c_int = 0;
  let mut ret: libc::c_int = 0;
  fh = fopen_or_warn(
    b"/proc/net/dev\x00" as *const u8 as *const libc::c_char,
    b"r\x00" as *const u8 as *const libc::c_char,
  );
  if fh.is_null() {
    return 0i32;
    /* "not found" */
  } /* eat line */
  fgets_unlocked(
    buf.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong as libc::c_int,
    fh,
  ); /* found */
  fgets_unlocked(
    buf.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong as libc::c_int,
    fh,
  );
  procnetdev_vsn = procnetdev_version(buf.as_mut_ptr());
  ret = 0i32;
  while !fgets_unlocked(
    buf.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong as libc::c_int,
    fh,
  )
  .is_null()
  {
    let mut s: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut name: [libc::c_char; 16] = [0; 16];
    s = get_name(name.as_mut_ptr(), buf.as_mut_ptr());
    ife = add_interface(ilist, name.as_mut_ptr());
    get_dev_fields(s, ife, procnetdev_vsn);
    (*ife).statistics_valid = 1i32 as smallint;
    if !(!ifname.is_null() && strcmp(ifname, name.as_mut_ptr()) == 0i32) {
      continue;
    }
    ret = 1i32;
    break;
  }
  fclose(fh);
  return ret;
}
unsafe extern "C" fn if_readlist(mut ilist: *mut iface_list, mut ifname: *mut libc::c_char) {
  let mut found: libc::c_int = if_readlist_proc(ilist, ifname);
  /* Needed in order to get ethN:M aliases */
  if found == 0 {
    if_readconf(ilist);
  };
}
/* Fetch the interface configuration from the kernel. */
unsafe extern "C" fn if_fetch(mut ife: *mut interface) -> libc::c_int {
  let mut ifr: ifreq = ifreq {
    ifr_ifrn: C2RustUnnamed_1 { ifrn_name: [0; 16] },
    ifr_ifru: C2RustUnnamed_0 {
      ifru_addr: sockaddr {
        sa_family: 0,
        sa_data: [0; 14],
      },
    },
  };
  let mut ifname: *mut libc::c_char = (*ife).name.as_mut_ptr();
  let mut skfd: libc::c_int = 0;
  skfd = xsocket(2i32, SOCK_DGRAM as libc::c_int, 0i32);
  strncpy_IFNAMSIZ(ifr.ifr_ifrn.ifrn_name.as_mut_ptr(), ifname);
  if ioctl(skfd, 0x8913i32 as libc::c_ulong, &mut ifr as *mut ifreq) < 0i32 {
    close(skfd);
    return -1i32;
  }
  (*ife).flags = ifr.ifr_ifru.ifru_flags;
  /* set up default values if ioctl's would fail */
  (*ife).tx_queue_len = -1i32; /* unknown value */
  memset(
    &mut (*ife).metric as *mut libc::c_int as *mut libc::c_void,
    0i32,
    136u64
      .wrapping_sub(40u64)
      .wrapping_add(::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong),
  );
  strncpy_IFNAMSIZ(ifr.ifr_ifrn.ifrn_name.as_mut_ptr(), ifname);
  if ioctl(skfd, 0x8927i32 as libc::c_ulong, &mut ifr as *mut ifreq) >= 0i32 {
    memcpy(
      (*ife).hwaddr.as_mut_ptr() as *mut libc::c_void,
      ifr.ifr_ifru.ifru_hwaddr.sa_data.as_mut_ptr() as *const libc::c_void,
      8i32 as libc::c_ulong,
    );
  }
  //er.... why this _isnt_ inside if()?
  (*ife).type_0 = ifr.ifr_ifru.ifru_hwaddr.sa_family as libc::c_short;
  strncpy_IFNAMSIZ(ifr.ifr_ifrn.ifrn_name.as_mut_ptr(), ifname);
  if ioctl(skfd, 0x891di32 as libc::c_ulong, &mut ifr as *mut ifreq) >= 0i32 {
    (*ife).metric = ifr.ifr_ifru.ifru_ivalue
  }
  strncpy_IFNAMSIZ(ifr.ifr_ifrn.ifrn_name.as_mut_ptr(), ifname);
  if ioctl(skfd, 0x8921i32 as libc::c_ulong, &mut ifr as *mut ifreq) >= 0i32 {
    (*ife).mtu = ifr.ifr_ifru.ifru_mtu
  }
  strncpy_IFNAMSIZ(ifr.ifr_ifrn.ifrn_name.as_mut_ptr(), ifname);
  if ioctl(skfd, 0x8970i32 as libc::c_ulong, &mut ifr as *mut ifreq) == 0i32 {
    (*ife).map = ifr.ifr_ifru.ifru_map
  }
  strncpy_IFNAMSIZ(ifr.ifr_ifrn.ifrn_name.as_mut_ptr(), ifname);
  if ioctl(skfd, 0x8942i32 as libc::c_ulong, &mut ifr as *mut ifreq) >= 0i32 {
    (*ife).tx_queue_len = ifr.ifr_ifru.ifru_ivalue
  }
  strncpy_IFNAMSIZ(ifr.ifr_ifrn.ifrn_name.as_mut_ptr(), ifname);
  ifr.ifr_ifru.ifru_addr.sa_family = 2i32 as sa_family_t;
  if ioctl(skfd, 0x8915i32 as libc::c_ulong, &mut ifr as *mut ifreq) == 0i32 {
    (*ife).has_ip = 1i32 as smallint;
    (*ife).addr = ifr.ifr_ifru.ifru_addr;
    strncpy_IFNAMSIZ(ifr.ifr_ifrn.ifrn_name.as_mut_ptr(), ifname);
    if ioctl(skfd, 0x8917i32 as libc::c_ulong, &mut ifr as *mut ifreq) >= 0i32 {
      (*ife).dstaddr = ifr.ifr_ifru.ifru_dstaddr
    }
    strncpy_IFNAMSIZ(ifr.ifr_ifrn.ifrn_name.as_mut_ptr(), ifname);
    if ioctl(skfd, 0x8919i32 as libc::c_ulong, &mut ifr as *mut ifreq) >= 0i32 {
      (*ife).broadaddr = ifr.ifr_ifru.ifru_broadaddr
    }
    strncpy_IFNAMSIZ(ifr.ifr_ifrn.ifrn_name.as_mut_ptr(), ifname);
    if ioctl(skfd, 0x891bi32 as libc::c_ulong, &mut ifr as *mut ifreq) >= 0i32 {
      (*ife).netmask = ifr.ifr_ifru.ifru_netmask
    }
  }
  close(skfd);
  return 0i32;
}
unsafe extern "C" fn do_if_fetch(mut ife: *mut interface) -> libc::c_int {
  if if_fetch(ife) < 0i32 {
    let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
    if *bb_errno == 19i32 {
      /* Give better error message for this case. */
      errmsg = b"Device not found\x00" as *const u8 as *const libc::c_char
    } else {
      errmsg = strerror(*bb_errno)
    }
    bb_error_msg(
      b"%s: error fetching interface information: %s\x00" as *const u8 as *const libc::c_char,
      (*ife).name.as_mut_ptr(),
      errmsg,
    );
    return -1i32;
  }
  return 0i32;
}

static mut unspec_hwtype: hwtype = {
  let mut init = hwtype {
    name: b"unspec\x00" as *const u8 as *const libc::c_char,
    title: b"UNSPEC\x00" as *const u8 as *const libc::c_char,
    type_0: -1i32,
    alen: 0,
    print: Some(UNSPEC_print as unsafe extern "C" fn(_: *mut libc::c_uchar) -> *mut libc::c_char),
    input: None,
    activate: None,
    suppress_null_addr: 0,
  };
  init
};

static mut loop_hwtype: hwtype = {
  let mut init = hwtype {
    name: b"loop\x00" as *const u8 as *const libc::c_char,
    title: b"Local Loopback\x00" as *const u8 as *const libc::c_char,
    type_0: 772i32,
    alen: 0,
    print: None,
    input: None,
    activate: None,
    suppress_null_addr: 0,
  };
  init
};
/* Display an Ethernet address in readable format. */
unsafe extern "C" fn ether_print(mut ptr: *mut libc::c_uchar) -> *mut libc::c_char {
  let mut buff: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  buff = xasprintf(
    b"%02X:%02X:%02X:%02X:%02X:%02X\x00" as *const u8 as *const libc::c_char,
    *ptr.offset(0) as libc::c_int,
    *ptr.offset(1) as libc::c_int,
    *ptr.offset(2) as libc::c_int,
    *ptr.offset(3) as libc::c_int,
    *ptr.offset(4) as libc::c_int,
    *ptr.offset(5) as libc::c_int,
  );
  return auto_string(buff);
}

static mut ether_hwtype: hwtype = {
  let mut init = hwtype {
    name: b"ether\x00" as *const u8 as *const libc::c_char,
    title: b"Ethernet\x00" as *const u8 as *const libc::c_char,
    type_0: 1i32,
    alen: 6i32,
    print: Some(ether_print as unsafe extern "C" fn(_: *mut libc::c_uchar) -> *mut libc::c_char),
    input: Some(
      in_ether as unsafe extern "C" fn(_: *const libc::c_char, _: *mut sockaddr) -> libc::c_int,
    ),
    activate: None,
    suppress_null_addr: 0,
  };
  init
};

static mut ppp_hwtype: hwtype = {
  let mut init = hwtype {
    name: b"ppp\x00" as *const u8 as *const libc::c_char,
    title: b"Point-to-Point Protocol\x00" as *const u8 as *const libc::c_char,
    type_0: 512i32,
    alen: 0,
    print: None,
    input: None,
    activate: None,
    suppress_null_addr: 0,
  };
  init
};

static mut sit_hwtype: hwtype = {
  let mut init = hwtype {
    name: b"sit\x00" as *const u8 as *const libc::c_char,
    title: b"IPv6-in-IPv4\x00" as *const u8 as *const libc::c_char,
    type_0: 776i32,
    alen: 0,
    print: Some(UNSPEC_print as unsafe extern "C" fn(_: *mut libc::c_uchar) -> *mut libc::c_char),
    input: None,
    activate: None,
    suppress_null_addr: 1i32,
  };
  init
};

static mut ib_hwtype: hwtype = {
  let mut init = hwtype {
    name: b"infiniband\x00" as *const u8 as *const libc::c_char,
    title: b"InfiniBand\x00" as *const u8 as *const libc::c_char,
    type_0: 32i32,
    alen: 20i32,
    print: Some(UNSPEC_print as unsafe extern "C" fn(_: *mut libc::c_uchar) -> *mut libc::c_char),
    input: Some(
      in_ib as unsafe extern "C" fn(_: *const libc::c_char, _: *mut sockaddr) -> libc::c_int,
    ),
    activate: None,
    suppress_null_addr: 0,
  };
  init
};

static mut hwtypes: [*const hwtype; 7] = unsafe {
  [
    &loop_hwtype as *const hwtype,
    &ether_hwtype as *const hwtype,
    &ppp_hwtype as *const hwtype,
    &unspec_hwtype as *const hwtype,
    &sit_hwtype as *const hwtype,
    &ib_hwtype as *const hwtype,
    0 as *const hwtype,
  ]
};
static mut if_port_text: [*const libc::c_char; 8] = [
  b"unknown\x00" as *const u8 as *const libc::c_char,
  b"10base2\x00" as *const u8 as *const libc::c_char,
  b"10baseT\x00" as *const u8 as *const libc::c_char,
  b"AUI\x00" as *const u8 as *const libc::c_char,
  b"100baseT\x00" as *const u8 as *const libc::c_char,
  b"100baseTX\x00" as *const u8 as *const libc::c_char,
  b"100baseFX\x00" as *const u8 as *const libc::c_char,
  0 as *const libc::c_char,
];
/* Check our hardware type table for this type. */
#[no_mangle]
pub unsafe extern "C" fn get_hwtype(mut name: *const libc::c_char) -> *const hwtype {
  let mut hwp: *const *const hwtype = 0 as *const *const hwtype;
  hwp = hwtypes.as_ptr();
  while !(*hwp).is_null() {
    if strcmp((**hwp).name, name) == 0i32 {
      return *hwp;
    }
    hwp = hwp.offset(1)
  }
  return 0 as *const hwtype;
}
/* Check our hardware type table for this type. */
#[no_mangle]
pub unsafe extern "C" fn get_hwntype(mut type_0: libc::c_int) -> *const hwtype {
  let mut hwp: *const *const hwtype = 0 as *const *const hwtype;
  hwp = hwtypes.as_ptr();
  while !(*hwp).is_null() {
    if (**hwp).type_0 == type_0 {
      return *hwp;
    }
    hwp = hwp.offset(1)
  }
  return 0 as *const hwtype;
}
/* return 1 if address is all zeros */
unsafe extern "C" fn hw_null_address(
  mut hw: *const hwtype,
  mut ap: *mut libc::c_void,
) -> libc::c_int {
  let mut i: libc::c_int = 0;
  let mut address: *mut libc::c_uchar = ap as *mut libc::c_uchar;
  i = 0i32;
  while i < (*hw).alen {
    if *address.offset(i as isize) != 0 {
      return 0i32;
    }
    i += 1
  }
  return 1i32;
}
static mut TRext: [libc::c_char; 15] = [0, 0, 0, 75, 105, 0, 77, 105, 0, 71, 105, 0, 84, 105, 0];
unsafe extern "C" fn print_bytes_scaled(mut ull: libc::c_ulonglong, mut end: *const libc::c_char) {
  let mut int_part: libc::c_ulonglong = 0;
  let mut ext: *const libc::c_char = 0 as *const libc::c_char;
  let mut frac_part: libc::c_uint = 0;
  let mut i: libc::c_int = 0;
  frac_part = 0i32 as libc::c_uint;
  ext = TRext.as_ptr();
  int_part = ull;
  i = 4i32;
  loop {
    if int_part >= 1024i32 as libc::c_ulonglong {
      frac_part = (int_part as libc::c_uint & (1024i32 - 1i32) as libc::c_uint)
        .wrapping_mul(10i32 as libc::c_uint)
        .wrapping_div(1024i32 as libc::c_uint);
      int_part = int_part.wrapping_div(1024i32 as libc::c_ulonglong);
      ext = ext.offset(3)
      /* KiB, MiB, GiB, TiB */
    }
    i -= 1;
    if !(i != 0) {
      break;
    }
  }
  printf(
    b"X bytes:%llu (%llu.%u %sB)%s\x00" as *const u8 as *const libc::c_char,
    ull,
    int_part,
    frac_part,
    ext,
    end,
  );
}
/* reserved address space */
unsafe extern "C" fn ife_print6(mut ptr: *mut interface) {
  let mut f: *mut FILE = std::ptr::null_mut();
  let mut addr6: [libc::c_char; 40] = [0; 40];
  let mut devname: [libc::c_char; 21] = [0; 21];
  let mut sap: sockaddr_in6 = sockaddr_in6 {
    sin6_family: 0,
    sin6_port: 0,
    sin6_flowinfo: 0,
    sin6_addr: in6_addr {
      __in6_u: C2RustUnnamed {
        __u6_addr8: [0; 16],
      },
    },
    sin6_scope_id: 0,
  };
  let mut plen: libc::c_int = 0;
  let mut scope: libc::c_int = 0;
  let mut dad_status: libc::c_int = 0;
  let mut if_idx: libc::c_int = 0;
  let mut addr6p: [[libc::c_char; 5]; 8] = [[0; 5]; 8];
  f = fopen_for_read(b"/proc/net/if_inet6\x00" as *const u8 as *const libc::c_char);
  if f.is_null() {
    return;
  }
  while fscanf(
    f,
    b"%4s%4s%4s%4s%4s%4s%4s%4s %08x %02x %02x %02x %20s\n\x00" as *const u8 as *const libc::c_char,
    addr6p[0].as_mut_ptr(),
    addr6p[1].as_mut_ptr(),
    addr6p[2].as_mut_ptr(),
    addr6p[3].as_mut_ptr(),
    addr6p[4].as_mut_ptr(),
    addr6p[5].as_mut_ptr(),
    addr6p[6].as_mut_ptr(),
    addr6p[7].as_mut_ptr(),
    &mut if_idx as *mut libc::c_int,
    &mut plen as *mut libc::c_int,
    &mut scope as *mut libc::c_int,
    &mut dad_status as *mut libc::c_int,
    devname.as_mut_ptr(),
  ) != -1i32
  {
    if strcmp(devname.as_mut_ptr(), (*ptr).name.as_mut_ptr()) == 0i32 {
      sprintf(
        addr6.as_mut_ptr(),
        b"%s:%s:%s:%s:%s:%s:%s:%s\x00" as *const u8 as *const libc::c_char,
        addr6p[0].as_mut_ptr(),
        addr6p[1].as_mut_ptr(),
        addr6p[2].as_mut_ptr(),
        addr6p[3].as_mut_ptr(),
        addr6p[4].as_mut_ptr(),
        addr6p[5].as_mut_ptr(),
        addr6p[6].as_mut_ptr(),
        addr6p[7].as_mut_ptr(),
      );
      memset(
        &mut sap as *mut sockaddr_in6 as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<sockaddr_in6>() as libc::c_ulong,
      );
      inet_pton(
        10i32,
        addr6.as_mut_ptr(),
        &mut sap.sin6_addr as *mut in6_addr as *mut sockaddr as *mut libc::c_void,
      );
      sap.sin6_family = 10i32 as sa_family_t;
      printf(
        b"          inet6 addr: %s/%d\x00" as *const u8 as *const libc::c_char,
        INET6_sprint(&mut sap as *mut sockaddr_in6 as *mut sockaddr, 1i32),
        plen,
      );
      printf(b" Scope:\x00" as *const u8 as *const libc::c_char);
      match scope as libc::c_uint & 0xf0u32 {
        0 => {
          puts(b"Global\x00" as *const u8 as *const libc::c_char);
        }
        32 => {
          puts(b"Link\x00" as *const u8 as *const libc::c_char);
        }
        64 => {
          puts(b"Site\x00" as *const u8 as *const libc::c_char);
        }
        128 => {
          puts(b"Compat\x00" as *const u8 as *const libc::c_char);
        }
        16 => {
          puts(b"Host\x00" as *const u8 as *const libc::c_char);
        }
        _ => {
          puts(b"Unknown\x00" as *const u8 as *const libc::c_char);
        }
      }
    }
  }
  fclose(f);
}
unsafe extern "C" fn ife_print(mut ptr: *mut interface) {
  let mut ap: *const aftype = 0 as *const aftype;
  let mut hw: *const hwtype = 0 as *const hwtype;
  let mut hf: libc::c_int = 0;
  let mut can_compress: libc::c_int = 0i32;
  ap = get_afntype((*ptr).addr.sa_family as libc::c_int);
  if ap.is_null() {
    ap = get_afntype(0i32)
  }
  hf = (*ptr).type_0 as libc::c_int;
  if hf == 257i32 || hf == 259i32 {
    can_compress = 1i32
  }
  hw = get_hwntype(hf);
  if hw.is_null() {
    hw = get_hwntype(-1i32)
  }
  printf(
    b"%-9s Link encap:%s  \x00" as *const u8 as *const libc::c_char,
    (*ptr).name.as_mut_ptr(),
    (*hw).title,
  );
  /* For some hardware types (eg Ash, ATM) we don't print the
  hardware address if it's null.  */
  if (*hw).print.is_some()
    && !(hw_null_address(hw, (*ptr).hwaddr.as_mut_ptr() as *mut libc::c_void) != 0
      && (*hw).suppress_null_addr != 0)
  {
    printf(
      b"HWaddr %s  \x00" as *const u8 as *const libc::c_char,
      (*hw).print.expect("non-null function pointer")(
        (*ptr).hwaddr.as_mut_ptr() as *mut libc::c_uchar
      ),
    );
  }
  if (*ptr).flags as libc::c_int & IFF_PORTSEL as libc::c_int != 0 {
    printf(
      b"Media:%s\x00" as *const u8 as *const libc::c_char,
      if_port_text[(*ptr).map.port as usize],
    );
    if (*ptr).flags as libc::c_int & IFF_AUTOMEDIA as libc::c_int != 0 {
      printf(b"(auto)\x00" as *const u8 as *const libc::c_char);
    }
  }
  bb_putchar('\n' as i32);
  if (*ptr).has_ip != 0 {
    printf(
      b"          %s addr:%s \x00" as *const u8 as *const libc::c_char,
      (*ap).name,
      (*ap).sprint.expect("non-null function pointer")(&mut (*ptr).addr, 1i32),
    );
    if (*ptr).flags as libc::c_int & IFF_POINTOPOINT as libc::c_int != 0 {
      printf(
        b" P-t-P:%s \x00" as *const u8 as *const libc::c_char,
        (*ap).sprint.expect("non-null function pointer")(&mut (*ptr).dstaddr, 1i32),
      );
    }
    if (*ptr).flags as libc::c_int & IFF_BROADCAST as libc::c_int != 0 {
      printf(
        b" Bcast:%s \x00" as *const u8 as *const libc::c_char,
        (*ap).sprint.expect("non-null function pointer")(&mut (*ptr).broadaddr, 1i32),
      );
    }
    printf(
      b" Mask:%s\n\x00" as *const u8 as *const libc::c_char,
      (*ap).sprint.expect("non-null function pointer")(&mut (*ptr).netmask, 1i32),
    );
  }
  ife_print6(ptr);
  printf(b"          \x00" as *const u8 as *const libc::c_char);
  /* DONT FORGET TO ADD THE FLAGS IN ife_print_short, too */
  if (*ptr).flags as libc::c_int == 0i32 {
    printf(b"[NO FLAGS] \x00" as *const u8 as *const libc::c_char);
  } else {
    static mut ife_print_flags_strs: [libc::c_char; 106] = [
      85, 80, 0, 66, 82, 79, 65, 68, 67, 65, 83, 84, 0, 68, 69, 66, 85, 71, 0, 76, 79, 79, 80, 66,
      65, 67, 75, 0, 80, 79, 73, 78, 84, 79, 80, 79, 73, 78, 84, 0, 78, 79, 84, 82, 65, 73, 76, 69,
      82, 83, 0, 82, 85, 78, 78, 73, 78, 71, 0, 78, 79, 65, 82, 80, 0, 80, 82, 79, 77, 73, 83, 67,
      0, 65, 76, 76, 77, 85, 76, 84, 73, 0, 83, 76, 65, 86, 69, 0, 77, 65, 83, 84, 69, 82, 0, 77,
      85, 76, 84, 73, 67, 65, 83, 84, 0, 0,
    ];
    static mut ife_print_flags_mask: [libc::c_ushort; 13] = [
      IFF_UP as libc::c_int as libc::c_ushort,
      IFF_BROADCAST as libc::c_int as libc::c_ushort,
      IFF_DEBUG as libc::c_int as libc::c_ushort,
      IFF_LOOPBACK as libc::c_int as libc::c_ushort,
      IFF_POINTOPOINT as libc::c_int as libc::c_ushort,
      IFF_NOTRAILERS as libc::c_int as libc::c_ushort,
      IFF_RUNNING as libc::c_int as libc::c_ushort,
      IFF_NOARP as libc::c_int as libc::c_ushort,
      IFF_PROMISC as libc::c_int as libc::c_ushort,
      IFF_ALLMULTI as libc::c_int as libc::c_ushort,
      IFF_SLAVE as libc::c_int as libc::c_ushort,
      IFF_MASTER as libc::c_int as libc::c_ushort,
      IFF_MULTICAST as libc::c_int as libc::c_ushort,
    ];
    let mut mask: *const libc::c_ushort = ife_print_flags_mask.as_ptr();
    let mut str: *const libc::c_char = ife_print_flags_strs.as_ptr();
    loop {
      if (*ptr).flags as libc::c_int & *mask as libc::c_int != 0 {
        printf(b"%s \x00" as *const u8 as *const libc::c_char, str);
      }
      mask = mask.offset(1);
      str = str.offset(strlen(str).wrapping_add(1i32 as libc::c_ulong) as isize);
      if !(*str != 0) {
        break;
      }
    }
  }
  /* DONT FORGET TO ADD THE FLAGS IN ife_print_short */
  printf(
    b" MTU:%d  Metric:%d\x00" as *const u8 as *const libc::c_char,
    (*ptr).mtu,
    if (*ptr).metric != 0 {
      (*ptr).metric
    } else {
      1i32
    },
  );
  bb_putchar('\n' as i32);
  /* If needed, display the interface statistics. */
  if (*ptr).statistics_valid != 0 {
    /* XXX: statistics are currently only printed for the primary address,
     *      not for the aliases, although strictly speaking they're shared
     *      by all addresses.
     */
    printf(b"          \x00" as *const u8 as *const libc::c_char);
    printf(
      b"RX packets:%llu errors:%lu dropped:%lu overruns:%lu frame:%lu\n\x00" as *const u8
        as *const libc::c_char,
      (*ptr).stats.rx_packets,
      (*ptr).stats.rx_errors,
      (*ptr).stats.rx_dropped,
      (*ptr).stats.rx_fifo_errors,
      (*ptr).stats.rx_frame_errors,
    );
    if can_compress != 0 {
      printf(
        b"             compressed:%lu\n\x00" as *const u8 as *const libc::c_char,
        (*ptr).stats.rx_compressed,
      );
    }
    printf(b"          \x00" as *const u8 as *const libc::c_char);
    printf(
      b"TX packets:%llu errors:%lu dropped:%lu overruns:%lu carrier:%lu\n\x00" as *const u8
        as *const libc::c_char,
      (*ptr).stats.tx_packets,
      (*ptr).stats.tx_errors,
      (*ptr).stats.tx_dropped,
      (*ptr).stats.tx_fifo_errors,
      (*ptr).stats.tx_carrier_errors,
    );
    printf(
      b"          collisions:%lu \x00" as *const u8 as *const libc::c_char,
      (*ptr).stats.collisions,
    );
    if can_compress != 0 {
      printf(
        b"compressed:%lu \x00" as *const u8 as *const libc::c_char,
        (*ptr).stats.tx_compressed,
      );
    }
    if (*ptr).tx_queue_len != -1i32 {
      printf(
        b"txqueuelen:%d \x00" as *const u8 as *const libc::c_char,
        (*ptr).tx_queue_len,
      );
    }
    printf(b"\n          R\x00" as *const u8 as *const libc::c_char);
    print_bytes_scaled(
      (*ptr).stats.rx_bytes,
      b"  T\x00" as *const u8 as *const libc::c_char,
    );
    print_bytes_scaled(
      (*ptr).stats.tx_bytes,
      b"\n\x00" as *const u8 as *const libc::c_char,
    );
  }
  if (*ptr).map.irq as libc::c_int != 0
    || (*ptr).map.mem_start != 0
    || (*ptr).map.dma as libc::c_int != 0
    || (*ptr).map.base_addr as libc::c_int != 0
  {
    printf(b"          \x00" as *const u8 as *const libc::c_char);
    if (*ptr).map.irq != 0 {
      printf(
        b"Interrupt:%d \x00" as *const u8 as *const libc::c_char,
        (*ptr).map.irq as libc::c_int,
      );
    }
    if (*ptr).map.base_addr as libc::c_int >= 0x100i32 {
      /* Only print devices using it for I/O maps */
      printf(
        b"Base address:0x%lx \x00" as *const u8 as *const libc::c_char,
        (*ptr).map.base_addr as libc::c_ulong,
      );
    }
    if (*ptr).map.mem_start != 0 {
      printf(
        b"Memory:%lx-%lx \x00" as *const u8 as *const libc::c_char,
        (*ptr).map.mem_start,
        (*ptr).map.mem_end,
      );
    }
    if (*ptr).map.dma != 0 {
      printf(
        b"DMA chan:%x \x00" as *const u8 as *const libc::c_char,
        (*ptr).map.dma as libc::c_int,
      );
    }
    bb_putchar('\n' as i32);
  }
  bb_putchar('\n' as i32);
}
unsafe extern "C" fn do_if_print(
  mut ife: *mut interface,
  mut show_downed_too: libc::c_int,
) -> libc::c_int {
  let mut res: libc::c_int = 0;
  res = do_if_fetch(ife);
  if res >= 0i32 {
    if (*ife).flags as libc::c_int & IFF_UP as libc::c_int != 0 || show_downed_too != 0 {
      ife_print(ife);
    }
  }
  return res;
}
#[no_mangle]
pub unsafe extern "C" fn display_interfaces(mut ifname: *mut libc::c_char) -> libc::c_int {
  let mut current_block: u64;
  let mut ife: *mut interface = std::ptr::null_mut();
  let mut res: libc::c_int = 0;
  let mut ilist: iface_list = iface_list {
    int_list: 0 as *mut interface,
    int_last: 0 as *mut interface,
  };
  ilist.int_list = std::ptr::null_mut();
  ilist.int_last = std::ptr::null_mut();
  if_readlist(
    &mut ilist,
    if ifname != 1i32 as intptr_t as *mut libc::c_char {
      ifname
    } else {
      std::ptr::null_mut::<libc::c_char>()
    },
  );
  if ifname.is_null() || ifname == 1i32 as intptr_t as *mut libc::c_char {
    ife = ilist.int_list;
    loop {
      if ife.is_null() {
        current_block = 13109137661213826276;
        break;
      }
      res = do_if_print(ife, ifname as intptr_t as libc::c_int);
      if res < 0i32 {
        current_block = 3896182693824010770;
        break;
      }
      ife = (*ife).next
    }
    match current_block {
      3896182693824010770 => {}
      _ => return 0i32,
    }
  } else {
    ife = add_interface(&mut ilist, ifname);
    res = do_if_print(ife, 1i32)
  }
  return (res < 0i32) as libc::c_int;
  /* status < 0 == 1 -- error */
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
/* We need to export XXX_main from libbusybox
 * only if we build "individual" binaries
 */
/* Embedded script support */
/* Applets which are useful from another applets */
/* If shell needs them, they exist even if not enabled as applets */
/* Similar, but used by chgrp, not shell */
/* Used by ftpd */
/* Don't need IF_xxx() guard for these */
/* Networking */
/* This structure defines protocol families and their handlers. */
/*int type,*/
/* may modify src */
/* This structure defines hardware protocols and their handlers. */
/* Input an Infiniband address and convert to binary. */
#[no_mangle]
pub unsafe extern "C" fn in_ib(
  mut bufp: *const libc::c_char,
  mut sap: *mut sockaddr,
) -> libc::c_int {
  (*sap).sa_family = ib_hwtype.type_0 as sa_family_t;
  //TODO: error check?
  hex2bin((*sap).sa_data.as_mut_ptr(), bufp, 20i32);
  return 0i32;
}
