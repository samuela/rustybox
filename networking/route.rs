use crate::librb::in6_addr;
use c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use libc;
use libc::fclose;
use libc::free;
use libc::fscanf;
use libc::in_addr;
use libc::printf;
use libc::sa_family_t;
use libc::sockaddr;
use libc::sockaddr_in;
use libc::sockaddr_in6;
use libc::strchr;
use libc::strcmp;
extern "C" {

  #[no_mangle]
  fn snprintf(
    _: *mut libc::c_char,
    _: libc::c_ulong,
    _: *const libc::c_char,
    _: ...
  ) -> libc::c_int;

  #[no_mangle]
  fn feof_unlocked(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  fn inet_ntoa(__in: in_addr) -> *mut libc::c_char;
  #[no_mangle]
  fn inet_pton(
    __af: libc::c_int,
    __cp: *const libc::c_char,
    __buf: *mut libc::c_void,
  ) -> libc::c_int;

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

/* numeric: & 0x8000: "default" instead of "*",
 *          & 0x4000: host instead of net,
 *          & 0x0fff: don't resolve
 */

/* These return malloced string */

}

pub type __caddr_t = *mut libc::c_char;
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
pub struct rtentry {
  pub rt_pad1: libc::c_ulong,
  pub rt_dst: sockaddr,
  pub rt_gateway: sockaddr,
  pub rt_genmask: sockaddr,
  pub rt_flags: libc::c_ushort,
  pub rt_pad2: libc::c_short,
  pub rt_pad3: libc::c_ulong,
  pub rt_tos: libc::c_uchar,
  pub rt_class: libc::c_uchar,
  pub rt_pad4: [libc::c_short; 3],
  pub rt_metric: libc::c_short,
  pub rt_dev: *mut libc::c_char,
  pub rt_mtu: libc::c_ulong,
  pub rt_window: libc::c_ulong,
  pub rt_irtt: libc::c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct in6_rtmsg {
  pub rtmsg_dst: in6_addr,
  pub rtmsg_src: in6_addr,
  pub rtmsg_gateway: in6_addr,
  pub rtmsg_type: u32,
  pub rtmsg_dst_len: u16,
  pub rtmsg_src_len: u16,
  pub rtmsg_metric: u32,
  pub rtmsg_info: libc::c_ulong,
  pub rtmsg_flags: u32,
  pub rtmsg_ifindex: libc::c_int,
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

use libc::FILE;
#[inline(always)]
unsafe extern "C" fn xatoul_range(
  mut str: *const libc::c_char,
  mut l: libc::c_ulong,
  mut u: libc::c_ulong,
) -> libc::c_ulong {
  return crate::libbb::xatonum::xatoull_range(str, l as libc::c_ulonglong, u as libc::c_ulonglong)
    as libc::c_ulong;
}
#[inline(always)]
unsafe extern "C" fn xatoul(mut str: *const libc::c_char) -> libc::c_ulong {
  return crate::libbb::xatonum::xatoull(str) as libc::c_ulong;
}
/* We remap '-' to '#' to avoid problems with getopt. */
static mut tbl_hash_net_host: [libc::c_char; 15] =
  [7, 1, 35, 110, 101, 116, 0, 7, 2, 35, 104, 111, 115, 116, 0];
static mut tbl_ipvx: [libc::c_char; 104] = [
  9, 16, 109, 101, 116, 114, 105, 99, 0, 10, 17, 110, 101, 116, 109, 97, 115, 107, 0, 5, 18, 103,
  119, 0, 10, 18, 103, 97, 116, 101, 119, 97, 121, 0, 6, 19, 109, 115, 115, 0, 9, 20, 119, 105,
  110, 100, 111, 119, 0, 7, 21, 105, 114, 116, 116, 0, 6, 22, 100, 101, 118, 0, 9, 22, 100, 101,
  118, 105, 99, 101, 0, 9, 32, 114, 101, 106, 101, 99, 116, 0, 6, 33, 109, 111, 100, 0, 6, 34, 100,
  121, 110, 0, 11, 35, 114, 101, 105, 110, 115, 116, 97, 116, 101, 0,
];
/* Since last, we can save a byte. */
static mut flags_ipvx: [u16; 4] = [
  0x200i32 as u16,
  0x20i32 as u16,
  0x10i32 as u16,
  0x8i32 as u16,
];
unsafe extern "C" fn kw_lookup(
  mut kwtbl: *const libc::c_char,
  mut pargs: *mut *mut *mut libc::c_char,
) -> libc::c_int {
  if !(**pargs).is_null() {
    loop {
      if strcmp(kwtbl.offset(2), **pargs) == 0 {
        /* Found a match. */
        *pargs = (*pargs).offset(1);
        if *kwtbl.offset(1) as libc::c_int & 0o20i32 != 0 {
          if (**pargs).is_null() {
            /* No more args! */
            crate::libbb::appletlib::bb_show_usage();
          }
          *pargs = (*pargs).offset(1)
          /* Calling routine will use args[-1]. */
        }
        return *kwtbl.offset(1) as libc::c_int;
      }
      kwtbl = kwtbl.offset(*kwtbl as libc::c_int as isize);
      if !(*kwtbl != 0) {
        break;
      }
    }
  }
  return 0;
}
/* Add or delete a route, depending on action. */
#[inline(never)]
unsafe extern "C" fn INET_setroute(mut action: libc::c_int, mut args: *mut *mut libc::c_char) {
  /* char buffer instead of bona-fide struct avoids aliasing warning */
  let mut rt_buf: [libc::c_char; 120] = [0; 120];
  let rt: *mut rtentry = rt_buf.as_mut_ptr() as *mut libc::c_void as *mut rtentry;
  let mut netmask: *const libc::c_char = std::ptr::null();
  let mut skfd: libc::c_int = 0;
  let mut isnet: libc::c_int = 0;
  let mut xflag: libc::c_int = 0;
  /* Grab the -net or -host options.  Remember they were transformed. */
  xflag = kw_lookup(tbl_hash_net_host.as_ptr(), &mut args);
  /* If we did grab -net or -host, make sure we still have an arg left. */
  if (*args).is_null() {
    crate::libbb::appletlib::bb_show_usage();
  }
  /* Clean out the RTREQ structure. */
  memset(
    rt as *mut libc::c_void,
    0,
    ::std::mem::size_of::<rtentry>() as libc::c_ulong,
  );
  let fresh0 = args;
  args = args.offset(1);
  let mut target: *const libc::c_char = *fresh0;
  let mut prefix: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  /* recognize x.x.x.x/mask format. */
  prefix = strchr(target, '/' as i32);
  if !prefix.is_null() {
    let mut prefix_len: libc::c_int = 0;
    prefix_len = xatoul_range(
      prefix.offset(1),
      0 as libc::c_ulong,
      32i32 as libc::c_ulong,
    ) as libc::c_int;
    (*(&mut (*rt).rt_genmask as *mut sockaddr as *mut sockaddr_in))
      .sin_addr
      .s_addr = {
      let mut __v: libc::c_uint = 0;
      let mut __x: libc::c_uint = !(0xffffffffu64 >> prefix_len) as libc::c_uint;
      if false {
        __v = (__x & 0xff000000u32) >> 24i32
          | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
          | (__x & 0xff00i32 as libc::c_uint) << 8i32
          | (__x & 0xffi32 as libc::c_uint) << 24i32
      } else {
        let fresh1 = &mut __v;
        let fresh2;
        let fresh3 = __x;
        asm!("bswap $0" : "=r" (fresh2) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh1, fresh3)) :);
        c2rust_asm_casts::AsmCast::cast_out(fresh1, fresh3, fresh2);
      }
      __v
    };
    *prefix = '\u{0}' as i32 as libc::c_char;
    (*rt).rt_genmask.sa_family = 2i32 as sa_family_t
  } else {
    /* Default netmask. */
    netmask = b"default\x00" as *const u8 as *const libc::c_char
  }
  /* Prefer hostname lookup is -host flag (xflag==1) was given. */
  isnet = crate::libbb::inet_common::INET_resolve(
    target,
    &mut (*rt).rt_dst as *mut sockaddr as *mut sockaddr_in,
    xflag & 2i32,
  );
  if isnet < 0 {
    crate::libbb::verror_msg::bb_error_msg_and_die(
      b"resolving %s\x00" as *const u8 as *const libc::c_char,
      target,
    );
  }
  if !prefix.is_null() {
    /* do not destroy prefix for process args */
    *prefix = '/' as i32 as libc::c_char
  }
  if xflag != 0 {
    /* Reinit isnet if -net or -host was specified. */
    isnet = xflag & 1i32
  }
  /* Fill in the other fields. */
  (*rt).rt_flags = if isnet != 0 {
    0x1i32
  } else {
    (0x1i32) | 0x4i32
  } as libc::c_ushort;
  /* FIXME: do we need to check anything of this? */
  while !(*args).is_null() {
    let mut k: libc::c_int = kw_lookup(tbl_ipvx.as_ptr(), &mut args);
    let mut args_m1: *const libc::c_char = *args.offset(-1i32 as isize);
    if k & 0o40i32 != 0 {
      (*rt).rt_flags = ((*rt).rt_flags as libc::c_int
        | flags_ipvx[(k & 3i32) as usize] as libc::c_int) as libc::c_ushort
    } else if k == 0o20i32 {
      (*rt).rt_metric = xatoul(args_m1).wrapping_add(1i32 as libc::c_ulong) as libc::c_short
    } else if k == 0o21i32 {
      let mut mask: sockaddr = sockaddr {
        sa_family: 0,
        sa_data: [0; 14],
      };
      if (*(&mut (*rt).rt_genmask as *mut sockaddr as *mut sockaddr_in))
        .sin_addr
        .s_addr
        != 0
      {
        crate::libbb::appletlib::bb_show_usage();
      }
      netmask = args_m1;
      isnet = crate::libbb::inet_common::INET_resolve(
        netmask,
        &mut mask as *mut sockaddr as *mut sockaddr_in,
        0,
      );
      if isnet < 0 {
        crate::libbb::verror_msg::bb_error_msg_and_die(
          b"resolving %s\x00" as *const u8 as *const libc::c_char,
          netmask,
        );
      }
      (*rt).rt_genmask = mask
    } else if k == 0o22i32 {
      if (*rt).rt_flags as libc::c_int & 0x2i32 != 0 {
        crate::libbb::appletlib::bb_show_usage();
      }
      isnet = crate::libbb::inet_common::INET_resolve(
        args_m1,
        &mut (*rt).rt_gateway as *mut sockaddr as *mut sockaddr_in,
        1i32,
      );
      (*rt).rt_flags = ((*rt).rt_flags as libc::c_int | 0x2i32) as libc::c_ushort;
      if isnet != 0 {
        if isnet < 0 {
          crate::libbb::verror_msg::bb_error_msg_and_die(
            b"resolving %s\x00" as *const u8 as *const libc::c_char,
            args_m1,
          );
        }
        crate::libbb::verror_msg::bb_error_msg_and_die(
          b"gateway %s is a NETWORK\x00" as *const u8 as *const libc::c_char,
          args_m1,
        );
      }
    } else if k == 0o23i32 {
      /* Check valid MSS bounds. */
      (*rt).rt_flags = ((*rt).rt_flags as libc::c_int | 0x40i32) as libc::c_ushort;
      (*rt).rt_mtu = xatoul_range(args_m1, 64i32 as libc::c_ulong, 32768i32 as libc::c_ulong)
    } else if k == 0o24i32 {
      /* Check valid window bounds. */
      (*rt).rt_flags = ((*rt).rt_flags as libc::c_int | 0x80i32) as libc::c_ushort; /* FIXME */
      (*rt).rt_window = xatoul_range(
        args_m1,
        128i32 as libc::c_ulong,
        2147483647i32 as libc::c_ulong,
      )
    } else if k == 0o25i32 {
      (*rt).rt_flags = ((*rt).rt_flags as libc::c_int | 0x100i32) as libc::c_ushort;
      (*rt).rt_irtt = xatoul(args_m1) as libc::c_ushort;
      (*rt).rt_irtt = ((*rt).rt_irtt as libc::c_uint)
        .wrapping_mul(crate::libbb::sysconf::bb_clk_tck().wrapping_div(100i32 as libc::c_uint))
        as libc::c_ushort as libc::c_ushort
    } else if (*rt).rt_dev.is_null()
      && (k == 0o26i32
        || k == 0 && {
          args = args.offset(1);
          (*args).is_null()
        })
    {
      /* Device is special in that it can be the last arg specified
       * and doesn't require the dev/device keyword in that case. */
      /* Don't use args_m1 here since args may have changed! */
      (*rt).rt_dev = *args.offset(-1i32 as isize)
    } else {
      /* Nothing matched. */
      crate::libbb::appletlib::bb_show_usage();
    }
  }
  if (*rt).rt_flags as libc::c_int & 0x200i32 != 0 && (*rt).rt_dev.is_null() {
    (*rt).rt_dev = b"lo\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
  }
  /* sanity checks.. */
  if (*(&mut (*rt).rt_genmask as *mut sockaddr as *mut sockaddr_in))
    .sin_addr
    .s_addr
    != 0
  {
    let mut mask_0: u32 = (*(&mut (*rt).rt_genmask as *mut sockaddr as *mut sockaddr_in))
      .sin_addr
      .s_addr;
    mask_0 = !({
      let mut __v: libc::c_uint = 0;
      let mut __x: libc::c_uint = mask_0;
      if false {
        __v = (__x & 0xff000000u32) >> 24i32
          | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
          | (__x & 0xff00i32 as libc::c_uint) << 8i32
          | (__x & 0xffi32 as libc::c_uint) << 24i32
      } else {
        let fresh4 = &mut __v;
        let fresh5;
        let fresh6 = __x;
        asm!("bswap $0" : "=r" (fresh5) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh4, fresh6)) :);
        c2rust_asm_casts::AsmCast::cast_out(fresh4, fresh6, fresh5);
      }
      __v
    });
    if (*rt).rt_flags as libc::c_int & 0x4i32 != 0 && mask_0 != 0xffffffffu32 {
      crate::libbb::verror_msg::bb_error_msg_and_die(
        b"netmask %.8x and host route conflict\x00" as *const u8 as *const libc::c_char,
        mask_0,
      );
    }
    if mask_0 & mask_0.wrapping_add(1i32 as libc::c_uint) != 0 {
      crate::libbb::verror_msg::bb_error_msg_and_die(
        b"bogus netmask %s\x00" as *const u8 as *const libc::c_char,
        netmask,
      );
    }
    mask_0 = (*(&mut (*rt).rt_dst as *mut sockaddr as *mut sockaddr_in))
      .sin_addr
      .s_addr;
    if mask_0
      & !(*(&mut (*rt).rt_genmask as *mut sockaddr as *mut sockaddr_in))
        .sin_addr
        .s_addr
      != 0
    {
      crate::libbb::verror_msg::bb_simple_error_msg_and_die(
        b"netmask and route address conflict\x00" as *const u8 as *const libc::c_char,
      );
    }
  }
  /* Fill out netmask if still unset */
  if action == 1i32 && (*rt).rt_flags as libc::c_int & 0x4i32 != 0 {
    (*(&mut (*rt).rt_genmask as *mut sockaddr as *mut sockaddr_in))
      .sin_addr
      .s_addr = 0xffffffffu32
  }
  /* Create a socket to the INET kernel. */
  skfd = crate::libbb::xfuncs_printf::xsocket(2i32, SOCK_DGRAM as libc::c_int, 0);
  if action == 1i32 {
    crate::libbb::xfuncs_printf::bb_xioctl(
      skfd,
      0x890bi32 as libc::c_uint,
      rt as *mut libc::c_void,
      b"SIOCADDRT\x00" as *const u8 as *const libc::c_char,
    );
  } else {
    crate::libbb::xfuncs_printf::bb_xioctl(
      skfd,
      0x890ci32 as libc::c_uint,
      rt as *mut libc::c_void,
      b"SIOCDELRT\x00" as *const u8 as *const libc::c_char,
    );
  };
}
#[inline(never)]
unsafe extern "C" fn INET6_setroute(mut action: libc::c_int, mut args: *mut *mut libc::c_char) {
  let mut sa6: sockaddr_in6 = std::mem::zeroed();
  let mut rt: in6_rtmsg = std::mem::zeroed();
  let mut prefix_len: libc::c_int = 0;
  let mut skfd: libc::c_int = 0;
  let mut devname: *const libc::c_char = std::ptr::null();
  /* We know args isn't NULL from the check in route_main. */
  let fresh7 = args; /* Yes... const to non is ok. */
  args = args.offset(1);
  let mut target: *const libc::c_char = *fresh7;
  if strcmp(target, b"default\x00" as *const u8 as *const libc::c_char) == 0 {
    prefix_len = 0;
    memset(
      &mut sa6 as *mut sockaddr_in6 as *mut libc::c_void,
      0,
      ::std::mem::size_of::<sockaddr_in6>() as libc::c_ulong,
    );
  } else {
    let mut cp: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    cp = strchr(target, '/' as i32);
    if !cp.is_null() {
      *cp = '\u{0}' as i32 as libc::c_char;
      prefix_len =
        xatoul_range(cp.offset(1), 0 as libc::c_ulong, 128i32 as libc::c_ulong) as libc::c_int
    } else {
      prefix_len = 128i32
    }
    if crate::libbb::inet_common::INET6_resolve(target, &mut sa6 as *mut sockaddr_in6) < 0 {
      crate::libbb::verror_msg::bb_error_msg_and_die(
        b"resolving %s\x00" as *const u8 as *const libc::c_char,
        target,
      );
    }
  }
  /* Clean out the RTREQ structure. */
  memset(
    &mut rt as *mut in6_rtmsg as *mut libc::c_void,
    0,
    ::std::mem::size_of::<in6_rtmsg>() as libc::c_ulong,
  );
  memcpy(
    &mut rt.rtmsg_dst as *mut in6_addr as *mut libc::c_void,
    sa6.sin6_addr.s6_addr.as_mut_ptr() as *const libc::c_void,
    ::std::mem::size_of::<in6_addr>() as libc::c_ulong,
  );
  /* Fill in the other fields. */
  rt.rtmsg_dst_len = prefix_len as u16;
  rt.rtmsg_flags = if prefix_len == 128i32 {
    (0x1i32) | 0x4i32
  } else {
    0x1i32
  } as u32;
  rt.rtmsg_metric = 1i32 as u32;
  devname = std::ptr::null();
  while !(*args).is_null() {
    let mut k: libc::c_int = kw_lookup(tbl_ipvx.as_ptr(), &mut args);
    let mut args_m1: *const libc::c_char = *args.offset(-1i32 as isize);
    if k == 0o41i32 || k == 0o42i32 {
      rt.rtmsg_flags |= flags_ipvx[(k & 3i32) as usize] as libc::c_uint
    } else if k == 0o20i32 {
      rt.rtmsg_metric = xatoul(args_m1) as u32
    } else if k == 0o22i32 {
      if rt.rtmsg_flags & 0x2i32 as libc::c_uint != 0 {
        crate::libbb::appletlib::bb_show_usage();
      }
      if crate::libbb::inet_common::INET6_resolve(args_m1, &mut sa6 as *mut sockaddr_in6) < 0 {
        crate::libbb::verror_msg::bb_error_msg_and_die(
          b"resolving %s\x00" as *const u8 as *const libc::c_char,
          args_m1,
        );
      }
      memcpy(
        &mut rt.rtmsg_gateway as *mut in6_addr as *mut libc::c_void,
        sa6.sin6_addr.s6_addr.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<in6_addr>() as libc::c_ulong,
      );
      rt.rtmsg_flags |= 0x2i32 as libc::c_uint
    } else if devname.is_null()
      && (k == 0o26i32
        || k == 0 && {
          args = args.offset(1);
          (*args).is_null()
        })
    {
      /* Device is special in that it can be the last arg specified
       * and doesn't require the dev/device keyword in that case. */
      /* Don't use args_m1 here since args may have changed! */
      devname = *args.offset(-1i32 as isize)
    } else {
      /* Nothing matched. */
      crate::libbb::appletlib::bb_show_usage();
    }
  }
  /* Create a socket to the INET6 kernel. */
  skfd = crate::libbb::xfuncs_printf::xsocket(10i32, SOCK_DGRAM as libc::c_int, 0);
  rt.rtmsg_ifindex = 0;
  if !devname.is_null() {
    let mut ifr: ifreq = ifreq {
      ifr_ifrn: C2RustUnnamed_1 { ifrn_name: [0; 16] },
      ifr_ifru: C2RustUnnamed_0 {
        ifru_addr: sockaddr {
          sa_family: 0,
          sa_data: [0; 14],
        },
      },
    };
    /*memset(&ifr, 0, sizeof(ifr)); - SIOCGIFINDEX does not need to clear all */
    crate::libbb::xfuncs::strncpy_IFNAMSIZ(ifr.ifr_ifrn.ifrn_name.as_mut_ptr(), devname);
    crate::libbb::xfuncs_printf::bb_xioctl(
      skfd,
      0x8933i32 as libc::c_uint,
      &mut ifr as *mut ifreq as *mut libc::c_void,
      b"SIOCGIFINDEX\x00" as *const u8 as *const libc::c_char,
    );
    rt.rtmsg_ifindex = ifr.ifr_ifru.ifru_ivalue
  }
  /* Tell the kernel to accept this route. */
  if action == 1i32 {
    crate::libbb::xfuncs_printf::bb_xioctl(
      skfd,
      0x890bi32 as libc::c_uint,
      &mut rt as *mut in6_rtmsg as *mut libc::c_void,
      b"SIOCADDRT\x00" as *const u8 as *const libc::c_char,
    );
  } else {
    crate::libbb::xfuncs_printf::bb_xioctl(
      skfd,
      0x890ci32 as libc::c_uint,
      &mut rt as *mut in6_rtmsg as *mut libc::c_void,
      b"SIOCDELRT\x00" as *const u8 as *const libc::c_char,
    );
  };
}
static mut flagvals: [libc::c_uint; 11] = [
  0x1i32 as libc::c_uint,
  0x2i32 as libc::c_uint,
  0x4i32 as libc::c_uint,
  0x8i32 as libc::c_uint,
  0x10i32 as libc::c_uint,
  0x20i32 as libc::c_uint,
  0x10000i32 as libc::c_uint,
  0x40000i32 as libc::c_uint,
  0x1000000i32 as libc::c_uint,
  0x200i32 as libc::c_uint,
  0x200000i32 as libc::c_uint,
];
/* Must agree with flagvals[]. */
static mut flagchars: [libc::c_char; 12] = [85, 71, 72, 82, 68, 77, 68, 65, 67, 33, 110, 0];
unsafe extern "C" fn set_flags(mut flagstr: *mut libc::c_char, mut flags: libc::c_int) {
  let mut i: libc::c_int = 0;
  i = 0;
  loop {
    *flagstr = flagchars[i as usize];
    if !(*flagstr as libc::c_int != 0) {
      break;
    }
    if flags as libc::c_uint & flagvals[i as usize] != 0 {
      flagstr = flagstr.offset(1)
    }
    i += 1
  }
}
/* also used in netstat */
#[no_mangle]
pub unsafe extern "C" fn bb_displayroutes(mut noresolve: libc::c_int, mut netstatfmt: libc::c_int) {
  let mut current_block: u64;
  let mut devname: [libc::c_char; 64] = [0; 64];
  let mut flags: [libc::c_char; 16] = [0; 16];
  let mut sdest: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut sgw: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut d: libc::c_ulong = 0;
  let mut g: libc::c_ulong = 0;
  let mut m: libc::c_ulong = 0;
  let mut r: libc::c_int = 0;
  let mut flgs: libc::c_int = 0;
  let mut ref_0: libc::c_int = 0;
  let mut use_0: libc::c_int = 0;
  let mut metric: libc::c_int = 0;
  let mut mtu: libc::c_int = 0;
  let mut win: libc::c_int = 0;
  let mut ir: libc::c_int = 0;
  let mut s_addr: sockaddr_in = sockaddr_in {
    sin_family: 0,
    sin_port: 0,
    sin_addr: in_addr { s_addr: 0 },
    sin_zero: [0; 8],
  };
  let mut mask: in_addr = in_addr { s_addr: 0 };
  let mut fp: *mut FILE = crate::libbb::wfopen::xfopen_for_read(
    b"/proc/net/route\x00" as *const u8 as *const libc::c_char,
  );
  printf(
    b"Kernel IP routing table\nDestination     Gateway         Genmask         Flags %s Iface\n\x00"
      as *const u8 as *const libc::c_char,
    if netstatfmt != 0 {
      b"  MSS Window  irtt\x00" as *const u8 as *const libc::c_char
    } else {
      b"Metric Ref    Use\x00" as *const u8 as *const libc::c_char
    },
  );
  /* Skip the first line. */
  r = fscanf(fp, b"%*[^\n]\n\x00" as *const u8 as *const libc::c_char);
  if r < 0 {
    current_block = 3405502113825085733;
  } else {
    current_block = 17216689946888361452;
  }
  loop {
    match current_block {
      3405502113825085733 =>
      /* Empty line, read error, or EOF. Yes, if routing table
       * is completely empty, /proc/net/route has no header.
       */
      {
        if r < 0 && feof_unlocked(fp) != 0 {
          break;
        }
        crate::libbb::perror_msg::bb_simple_perror_msg_and_die(
          b"read error\x00" as *const u8 as *const libc::c_char,
        );
      }
      _ =>
      /* Skip interfaces that are down. */
      {
        r = fscanf(
          fp,
          b"%63s%lx%lx%X%d%d%d%lx%d%d%d\n\x00" as *const u8 as *const libc::c_char,
          devname.as_mut_ptr(),
          &mut d as *mut libc::c_ulong,
          &mut g as *mut libc::c_ulong,
          &mut flgs as *mut libc::c_int,
          &mut ref_0 as *mut libc::c_int,
          &mut use_0 as *mut libc::c_int,
          &mut metric as *mut libc::c_int,
          &mut m as *mut libc::c_ulong,
          &mut mtu as *mut libc::c_int,
          &mut win as *mut libc::c_int,
          &mut ir as *mut libc::c_int,
        ); /* 'default' instead of '*' */
        if r != 11i32 {
          current_block = 3405502113825085733; /* Host instead of net */
          continue;
        }
        if flgs & 0x1i32 == 0 {
          current_block = 17216689946888361452;
          continue;
        }
        set_flags(
          flags.as_mut_ptr(),
          flgs & (0x1i32 | 0x2i32 | 0x4i32 | 0x8i32 | 0x10i32 | 0x20i32),
        );
        if flgs & 0x200i32 != 0 {
          flags[0] = '!' as i32 as libc::c_char
        }
        memset(
          &mut s_addr as *mut sockaddr_in as *mut libc::c_void,
          0,
          ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong,
        );
        s_addr.sin_family = 2i32 as sa_family_t;
        s_addr.sin_addr.s_addr = d as in_addr_t;
        sdest =
          crate::libbb::inet_common::INET_rresolve(&mut s_addr, noresolve | 0x8000i32, m as u32);
        s_addr.sin_addr.s_addr = g as in_addr_t;
        sgw =
          crate::libbb::inet_common::INET_rresolve(&mut s_addr, noresolve | 0x4000i32, m as u32);
        mask.s_addr = m as in_addr_t;
        /* "%15.15s" truncates hostnames, do we really want that? */
        printf(
          b"%-15.15s %-15.15s %-16s%-6s\x00" as *const u8 as *const libc::c_char,
          sdest,
          sgw,
          inet_ntoa(mask),
          flags.as_mut_ptr(),
        );
        free(sdest as *mut libc::c_void);
        free(sgw as *mut libc::c_void);
        if netstatfmt != 0 {
          printf(
            b"%5d %-5d %6d %s\n\x00" as *const u8 as *const libc::c_char,
            mtu,
            win,
            ir,
            devname.as_mut_ptr(),
          );
        } else {
          printf(
            b"%-6d %-2d %7d %s\n\x00" as *const u8 as *const libc::c_char,
            metric,
            ref_0,
            use_0,
            devname.as_mut_ptr(),
          );
        }
        current_block = 17216689946888361452;
      }
    }
  }
  /* EOF with no (nonspace) chars read. */
  fclose(fp);
}
unsafe extern "C" fn INET6_displayroutes() {
  let mut addr6: [libc::c_char; 128] = [0; 128];
  let mut naddr6: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  /* In addr6x, we store both 40-byte ':'-delimited ipv6 addresses.
   * We read the non-delimited strings into the tail of the buffer
   * using fscanf and then modify the buffer by shifting forward
   * while inserting ':'s and the nul terminator for the first string.
   * Hence the strings are at addr6x and addr6x+40.  This generates
   * _much_ less code than the previous (upstream) approach. */
  let mut addr6x: [libc::c_char; 80] = [0; 80];
  let mut iface: [libc::c_char; 16] = [0; 16];
  let mut flags: [libc::c_char; 16] = [0; 16];
  let mut iflags: libc::c_int = 0;
  let mut metric: libc::c_int = 0;
  let mut refcnt: libc::c_int = 0;
  let mut use_0: libc::c_int = 0;
  let mut prefix_len: libc::c_int = 0;
  let mut slen: libc::c_int = 0;
  let mut snaddr6: sockaddr_in6 = std::mem::zeroed();
  let mut fp: *mut FILE = crate::libbb::wfopen::xfopen_for_read(
    b"/proc/net/ipv6_route\x00" as *const u8 as *const libc::c_char,
  );
  printf(
    b"Kernel IPv6 routing table\n%-44s%-40sFlags Metric Ref    Use Iface\n\x00" as *const u8
      as *const libc::c_char,
    b"Destination\x00" as *const u8 as *const libc::c_char,
    b"Next Hop\x00" as *const u8 as *const libc::c_char,
  );
  let mut current_block_21: u64;
  loop {
    let mut r: libc::c_int = 0;
    r = fscanf(
      fp,
      b"%32s%x%*s%x%32s%x%x%x%x%s\n\x00" as *const u8 as *const libc::c_char,
      addr6x.as_mut_ptr().offset(14),
      &mut prefix_len as *mut libc::c_int,
      &mut slen as *mut libc::c_int,
      addr6x.as_mut_ptr().offset(40).offset(7),
      &mut metric as *mut libc::c_int,
      &mut refcnt as *mut libc::c_int,
      &mut use_0 as *mut libc::c_int,
      &mut iflags as *mut libc::c_int,
      iface.as_mut_ptr(),
    );
    if r != 9i32 {
      if r < 0 && feof_unlocked(fp) != 0 {
        break;
      }
    } else {
      /* Do the addr6x shift-and-insert changes to ':'-delimit addresses.
       * For now, always do this to validate the proc route format, even
       * if the interface is down. */
      let mut i: libc::c_int = 0;
      let mut p: *mut libc::c_char = addr6x.as_mut_ptr().offset(14);
      loop {
        if *p == 0 {
          if !(i == 40i32) {
            current_block_21 = 9578924276388461147;
            break;
          }
          /* nul terminator for 1st address? */
          addr6x[39] = 0 as libc::c_char; /* Fixup... need 0 instead of ':'. */
          p = p.offset(1)
        } else {
          let fresh8 = p; /* Skip and continue. */
          p = p.offset(1); /* 2nd pass */
          let fresh9 = i;
          i = i + 1;
          addr6x[fresh9 as usize] = *fresh8;
          if (i + 1i32) % 5i32 == 0 {
            let fresh10 = i;
            i = i + 1;
            addr6x[fresh10 as usize] = ':' as i32 as libc::c_char
          }
        }
        if !(i < 40i32 + 28i32 + 7i32) {
          current_block_21 = 5634871135123216486;
          break;
        }
      }
      match current_block_21 {
        9578924276388461147 => {}
        _ => {
          set_flags(
            flags.as_mut_ptr(),
            iflags
              & (0x1i32
                | 0x2i32
                | 0x4i32
                | 0x10000i32
                | 0x40000i32
                | 0x1000000i32
                | 0x200i32
                | 0x200000i32),
          );
          r = 0;
          loop {
            memset(
              &mut snaddr6 as *mut sockaddr_in6 as *mut libc::c_void,
              0,
              ::std::mem::size_of::<sockaddr_in6>() as libc::c_ulong,
            );
            inet_pton(
              10i32,
              addr6x.as_mut_ptr().offset(r as isize),
              &mut snaddr6.sin6_addr as *mut libc::in6_addr as *mut sockaddr as *mut libc::c_void,
            );
            snaddr6.sin6_family = 10i32 as sa_family_t;
            naddr6 = crate::libbb::inet_common::INET6_rresolve(
              &mut snaddr6 as *mut sockaddr_in6,
              0xfffi32,
            );
            if r == 0 {
              /* 1st pass */
              snprintf(
                addr6.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
                b"%s/%d\x00" as *const u8 as *const libc::c_char,
                naddr6,
                prefix_len,
              );
              r += 40i32;
              free(naddr6 as *mut libc::c_void);
            } else {
              /* Print the info. */
              printf(
                b"%-43s %-39s %-5s %-6d %-2d %7d %-8s\n\x00" as *const u8 as *const libc::c_char,
                addr6.as_mut_ptr(),
                naddr6,
                flags.as_mut_ptr(),
                metric,
                refcnt,
                use_0,
                iface.as_mut_ptr(),
              );
              free(naddr6 as *mut libc::c_void);
              break;
            }
          }
          continue;
        }
      }
    }
    crate::libbb::perror_msg::bb_simple_perror_msg_and_die(
      b"read error\x00" as *const u8 as *const libc::c_char,
    );
  }
  fclose(fp);
}
/* Not an actual option. See below. */
/* 1st byte is offset to next entry offset.  2nd byte is return value. */
/* 2nd byte matches RTACTION_* code */
static mut tbl_verb: [libc::c_char; 21] = [
  6, 1, 97, 100, 100, 0, 6, 2, 100, 101, 108, 0, 8, 2, 100, 101, 108, 101, 116, 101, 0,
];
/* Since it's last, we can save a byte. */
#[no_mangle]
pub unsafe extern "C" fn route_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut opt: libc::c_uint = 0;
  let mut what: libc::c_int = 0;
  let mut family: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut p: *mut *mut libc::c_char = std::ptr::null_mut();
  /* First, remap '-net' and '-host' to avoid getopt problems. */
  p = argv;
  loop {
    p = p.offset(1);
    if (*p).is_null() {
      break;
    }
    if strcmp(*p, b"-net\x00" as *const u8 as *const libc::c_char) == 0
      || strcmp(*p, b"-host\x00" as *const u8 as *const libc::c_char) == 0
    {
      *(*p.offset(0)).offset(0) = '#' as i32 as libc::c_char
    }
  }
  opt = crate::libbb::getopt32::getopt32(
    argv,
    b"A:ne\x00" as *const u8 as *const libc::c_char,
    &mut family as *mut *mut libc::c_char,
  );
  if opt & 0x1i32 as libc::c_uint != 0
    && strcmp(family, b"inet\x00" as *const u8 as *const libc::c_char) != 0
  {
    if strcmp(family, b"inet6\x00" as *const u8 as *const libc::c_char) == 0 {
      opt |= 0x8i32 as libc::c_uint
    /* Set flag for ipv6. */
    } else {
      crate::libbb::appletlib::bb_show_usage();
    }
  }
  argv = argv.offset(optind as isize);
  /* No more args means display the routing table. */
  if (*argv).is_null() {
    let mut noresolve: libc::c_int = if opt & 0x2i32 as libc::c_uint != 0 {
      0xfffi32
    } else {
      0
    };
    if opt & 0x8i32 as libc::c_uint != 0 {
      INET6_displayroutes();
    } else {
      bb_displayroutes(noresolve, (opt & 0x4i32 as libc::c_uint) as libc::c_int);
    }
    crate::libbb::fflush_stdout_and_exit::fflush_stdout_and_exit(0i32);
  }
  /* Check verb.  At the moment, must be add, del, or delete. */
  what = kw_lookup(tbl_verb.as_ptr(), &mut argv);
  if what == 0 || (*argv).is_null() {
    /* Unknown verb or no more args. */
    crate::libbb::appletlib::bb_show_usage();
  }
  if opt & 0x8i32 as libc::c_uint != 0 {
    INET6_setroute(what, argv);
  } else {
    INET_setroute(what, argv);
  }
  return 0;
}
