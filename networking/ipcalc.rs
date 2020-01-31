use c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;

use libc;
use libc::printf;
extern "C" {
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn gethostbyaddr(
    __addr: *const libc::c_void,
    __len: __socklen_t,
    __type: libc::c_int,
  ) -> *mut hostent;

  #[no_mangle]
  fn inet_ntoa(__in: in_addr) -> *mut libc::c_char;
  #[no_mangle]
  fn inet_aton(__cp: *const libc::c_char, __inp: *mut in_addr) -> libc::c_int;

  #[no_mangle]
  static mut logmode: smallint;

}

pub type __socklen_t = libc::c_uint;
use crate::librb::smallint;

use libc::in_addr;
pub type in_addr_t = u32;

use libc::hostent;
pub type C2RustUnnamed = libc::c_uint;
pub const LOGMODE_BOTH: C2RustUnnamed = 3;
pub const LOGMODE_SYSLOG: C2RustUnnamed = 2;
pub const LOGMODE_STDIO: C2RustUnnamed = 1;
pub const LOGMODE_NONE: C2RustUnnamed = 0;
#[inline(always)]
unsafe fn xatoul_range(
  mut str: *const libc::c_char,
  mut l: libc::c_ulong,
  mut u: libc::c_ulong,
) -> libc::c_ulong {
  return crate::libbb::xatonum::xatoull_range(str, l as libc::c_ulonglong, u as libc::c_ulonglong)
    as libc::c_ulong;
}

/*
 * Mini ipcalc implementation for busybox
 *
 * By Jordan Crouse <jordan@cosmicpenguin.net>
 *    Stephan Linz  <linz@li-pro.net>
 *
 * This is a complete reimplementation of the ipcalc program
 * from Red Hat.  I didn't look at their source code, but there
 * is no denying that this is a loving reimplementation
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config IPCALC
//config:	bool "ipcalc (4.4 kb)"
//config:	default y
//config:	help
//config:	ipcalc takes an IP address and netmask and calculates the
//config:	resulting broadcast, network, and host range.
//config:
//config:config FEATURE_IPCALC_LONG_OPTIONS
//config:	bool "Enable long options"
//config:	default y
//config:	depends on IPCALC && LONG_OPTS
//config:
//config:config FEATURE_IPCALC_FANCY
//config:	bool "Fancy IPCALC, more options, adds 1 kbyte"
//config:	default y
//config:	depends on IPCALC
//config:	help
//config:	Adds the options hostname, prefix and silent to the output of
//config:	"ipcalc".
//applet:IF_IPCALC(APPLET_NOEXEC(ipcalc, ipcalc, BB_DIR_BIN, SUID_DROP, ipcalc))
//kbuild:lib-$(CONFIG_IPCALC) += ipcalc.o
//usage:#define ipcalc_trivial_usage
//usage:       "[OPTIONS] ADDRESS"
//usage:       IF_FEATURE_IPCALC_FANCY("[/PREFIX]") " [NETMASK]"
//usage:#define ipcalc_full_usage "\n\n"
//usage:       "Calculate and display network settings from IP address\n"
//usage:     "\n	-b	Broadcast address"
//usage:     "\n	-n	Network address"
//usage:     "\n	-m	Default netmask for IP"
//usage:	IF_FEATURE_IPCALC_FANCY(
//usage:     "\n	-p	Prefix for IP/NETMASK"
//usage:     "\n	-h	Resolved host name"
//usage:     "\n	-s	No error messages"
//usage:	)
/* After libbb.h, because on some systems it needs other includes */
unsafe fn get_netmask(mut ipaddr: libc::c_ulong) -> libc::c_ulong {
  ipaddr = ({
    let mut __v: libc::c_uint = 0;
    let mut __x: libc::c_uint = ipaddr as libc::c_uint;
    if false {
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
  }) as libc::c_ulong;
  if ipaddr & 0xc0000000u32 as libc::c_ulong == 0xc0000000u32 as libc::c_ulong {
    return ({
      let mut __v: libc::c_uint = 0;
      let mut __x: libc::c_uint = 0xffffff00u32;
      if false {
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
    }) as libc::c_ulong;
  } else if ipaddr & 0x80000000u32 as libc::c_ulong == 0x80000000u32 as libc::c_ulong {
    return ({
      let mut __v: libc::c_uint = 0;
      let mut __x: libc::c_uint = 0xffff0000u32;
      if false {
        __v = (__x & 0xff000000u32) >> 24i32
          | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
          | (__x & 0xff00i32 as libc::c_uint) << 8i32
          | (__x & 0xffi32 as libc::c_uint) << 24i32
      } else {
        let fresh6 = &mut __v;
        let fresh7;
        let fresh8 = __x;
        asm!("bswap $0" : "=r" (fresh7) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh6, fresh8)) :);
        c2rust_asm_casts::AsmCast::cast_out(fresh6, fresh8, fresh7);
      }
      __v
    }) as libc::c_ulong;
  } else if ipaddr & 0x80000000u32 as libc::c_ulong == 0 as libc::c_ulong {
    return ({
      let mut __v: libc::c_uint = 0;
      let mut __x: libc::c_uint = 0xff000000u32;
      if false {
        __v = (__x & 0xff000000u32) >> 24i32
          | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
          | (__x & 0xff00i32 as libc::c_uint) << 8i32
          | (__x & 0xffi32 as libc::c_uint) << 24i32
      } else {
        let fresh9 = &mut __v;
        let fresh10;
        let fresh11 = __x;
        asm!("bswap $0" : "=r" (fresh10) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh9, fresh11)) :);
        c2rust_asm_casts::AsmCast::cast_out(fresh9, fresh11, fresh10);
      }
      __v
    }) as libc::c_ulong;
  } else {
    return 0 as libc::c_ulong;
  };
}
unsafe fn get_prefix(mut netmask: libc::c_ulong) -> libc::c_int {
  let mut msk: libc::c_ulong = 0x80000000u32 as libc::c_ulong;
  let mut ret: libc::c_int = 0;
  netmask = ({
    let mut __v: libc::c_uint = 0;
    let mut __x: libc::c_uint = netmask as libc::c_uint;
    if false {
      __v = (__x & 0xff000000u32) >> 24i32
        | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
        | (__x & 0xff00i32 as libc::c_uint) << 8i32
        | (__x & 0xffi32 as libc::c_uint) << 24i32
    } else {
      let fresh12 = &mut __v;
      let fresh13;
      let fresh14 = __x;
      asm!("bswap $0" : "=r" (fresh13) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh12, fresh14)) :);
      c2rust_asm_casts::AsmCast::cast_out(fresh12, fresh14, fresh13);
    }
    __v
  }) as libc::c_ulong;
  while msk != 0 {
    if netmask & msk != 0 {
      ret += 1
    }
    msk >>= 1i32
  }
  return ret;
}
static mut ipcalc_longopts: [libc::c_char; 62] = [
  110, 101, 116, 109, 97, 115, 107, 0, 0, 109, 98, 114, 111, 97, 100, 99, 97, 115, 116, 0, 0, 98,
  110, 101, 116, 119, 111, 114, 107, 0, 0, 110, 112, 114, 101, 102, 105, 120, 0, 0, 112, 104, 111,
  115, 116, 110, 97, 109, 101, 0, 0, 104, 115, 105, 108, 101, 110, 116, 0, 0, 115, 0,
];
pub unsafe fn ipcalc_main(mut _argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
  let mut opt: libc::c_uint = 0;
  let mut have_netmask: bool = 0 != 0;
  let mut s_netmask: in_addr = in_addr { s_addr: 0 };
  let mut s_broadcast: in_addr = in_addr { s_addr: 0 };
  let mut s_network: in_addr = in_addr { s_addr: 0 };
  let mut s_ipaddr: in_addr = in_addr { s_addr: 0 };
  /* struct in_addr { in_addr_t s_addr; }  and  in_addr_t
   * (which in turn is just a typedef to u32)
   * are essentially the same type. A few macros for less verbosity: */
  let mut ipstr: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>(); /* suppress error_msg() output */
  opt = crate::libbb::getopt32::getopt32long(
    argv,
    b"^mbnphs\x00-1:?2\x00" as *const u8 as *const libc::c_char,
    ipcalc_longopts.as_ptr(),
  );
  argv = argv.offset(optind as isize);
  if opt & 0x20i32 as libc::c_uint != 0 {
    logmode = LOGMODE_NONE as libc::c_int as smallint
  }
  opt &= !0x20i32 as libc::c_uint;
  if opt & (0x2i32 | 0x4i32 | 0x8i32) as libc::c_uint == 0 {
    /* if no options at all or
     * (no broadcast,network,prefix) and (two args)... */
    if opt == 0 || !(*argv.offset(1)).is_null() {
      crate::libbb::appletlib::bb_show_usage();
    }
  }
  ipstr = *argv.offset(0);
  let mut netprefix: libc::c_ulong = 0 as libc::c_ulong;
  let mut prefixstr: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  prefixstr = ipstr;
  while *prefixstr != 0 {
    if *prefixstr as libc::c_int == '/' as i32 {
      let fresh15 = prefixstr;
      prefixstr = prefixstr.offset(1);
      *fresh15 = '\u{0}' as i32 as libc::c_char;
      if *prefixstr != 0 {
        let mut msk: libc::c_uint = 0;
        netprefix = xatoul_range(prefixstr, 0 as libc::c_ulong, 32i32 as libc::c_ulong);
        s_netmask.s_addr = 0 as in_addr_t;
        msk = 0x80000000u32;
        while netprefix > 0 as libc::c_ulong {
          s_netmask.s_addr |= msk;
          msk >>= 1i32;
          netprefix = netprefix.wrapping_sub(1)
        }
        s_netmask.s_addr = {
          let mut __v: libc::c_uint = 0;
          let mut __x: libc::c_uint = s_netmask.s_addr;
          if false {
            __v = (__x & 0xff000000u32) >> 24i32
              | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
              | (__x & 0xff00i32 as libc::c_uint) << 8i32
              | (__x & 0xffi32 as libc::c_uint) << 24i32
          } else {
            let fresh16 = &mut __v;
            let fresh17;
            let fresh18 = __x;
            asm!("bswap $0" : "=r" (fresh17) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh16, fresh18)) :);
            c2rust_asm_casts::AsmCast::cast_out(fresh16, fresh18, fresh17);
          }
          __v
        };
        /* Even if it was 0, we will signify that we have a netmask. This allows */
        /* for specification of default routes, etc which have a 0 netmask/prefix */
        have_netmask = 1i32 != 0
      }
      break;
    } else {
      prefixstr = prefixstr.offset(1)
    }
  }
  if inet_aton(ipstr, &mut s_ipaddr) == 0 {
    crate::libbb::verror_msg::bb_error_msg_and_die(
      b"bad IP address: %s\x00" as *const u8 as *const libc::c_char,
      *argv.offset(0),
    );
  }
  if !(*argv.offset(1)).is_null() {
    if 1i32 != 0 && have_netmask as libc::c_int != 0 {
      crate::libbb::verror_msg::bb_simple_error_msg_and_die(
        b"use prefix or netmask, not both\x00" as *const u8 as *const libc::c_char,
      );
    }
    if inet_aton(*argv.offset(1), &mut s_netmask) == 0 {
      crate::libbb::verror_msg::bb_error_msg_and_die(
        b"bad netmask: %s\x00" as *const u8 as *const libc::c_char,
        *argv.offset(1),
      );
    }
  } else if 1i32 == 0 || !have_netmask {
    s_netmask.s_addr = get_netmask(s_ipaddr.s_addr as libc::c_ulong) as in_addr_t
  }
  if opt & 0x1i32 as libc::c_uint != 0 {
    printf(
      b"NETMASK=%s\n\x00" as *const u8 as *const libc::c_char,
      inet_ntoa(s_netmask),
    );
  }
  if opt & 0x2i32 as libc::c_uint != 0 {
    s_broadcast.s_addr = s_ipaddr.s_addr & s_netmask.s_addr | !s_netmask.s_addr;
    printf(
      b"BROADCAST=%s\n\x00" as *const u8 as *const libc::c_char,
      inet_ntoa(s_broadcast),
    );
  }
  if opt & 0x4i32 as libc::c_uint != 0 {
    s_network.s_addr = s_ipaddr.s_addr & s_netmask.s_addr;
    printf(
      b"NETWORK=%s\n\x00" as *const u8 as *const libc::c_char,
      inet_ntoa(s_network),
    );
  }
  if opt & 0x8i32 as libc::c_uint != 0 {
    printf(
      b"PREFIX=%i\n\x00" as *const u8 as *const libc::c_char,
      get_prefix(s_netmask.s_addr as libc::c_ulong),
    );
  }
  if opt & 0x10i32 as libc::c_uint != 0 {
    let mut hostinfo: *mut hostent = std::ptr::null_mut();
    hostinfo = gethostbyaddr(
      &mut s_ipaddr.s_addr as *mut in_addr_t as *mut libc::c_char as *const libc::c_void,
      ::std::mem::size_of::<in_addr_t>() as libc::c_ulong as __socklen_t,
      2i32,
    );
    if hostinfo.is_null() {
      crate::libbb::herror_msg::bb_herror_msg_and_die(
        b"can\'t find hostname for %s\x00" as *const u8 as *const libc::c_char,
        *argv.offset(0),
      );
    }
    crate::libbb::str_tolower::str_tolower((*hostinfo).h_name);
    printf(
      b"HOSTNAME=%s\n\x00" as *const u8 as *const libc::c_char,
      (*hostinfo).h_name,
    );
  }
  return 0;
}
/* JHC - If the netmask wasn't provided then calculate it */
