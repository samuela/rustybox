use c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use libc;
extern "C" {
  #[no_mangle]
  fn fclose(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn strtoul(
    __nptr: *const libc::c_char,
    __endptr: *mut *mut libc::c_char,
    __base: libc::c_int,
  ) -> libc::c_ulong;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
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
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn bb_strtou(
    arg: *const libc::c_char,
    endp: *mut *mut libc::c_char,
    base: libc::c_int,
  ) -> libc::c_uint;
  #[no_mangle]
  fn auto_string(str: *mut libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn fopen_for_read(path: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn bb_clk_tck() -> libc::c_uint;
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  static bb_msg_invalid_arg_to: [libc::c_char; 0];
}
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type int16_t = __int16_t;
use crate::librb::uint8_t;
pub type uint16_t = __uint16_t;
use crate::librb::uint32_t;
pub type size_t = libc::c_ulong;
pub type socklen_t = __socklen_t;



use crate::librb::FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct inet_prefix {
  pub family: uint8_t,
  pub bytelen: uint8_t,
  pub bitlen: int16_t,
  pub data: [uint32_t; 4],
}
//const char *dnet_ntop(int af, const void *addr, char *str, size_t len);
//int dnet_pton(int af, const char *src, void *addr);
//const char *ipx_ntop(int af, const void *addr, char *str, size_t len);
//int ipx_pton(int af, const char *src, void *addr);
/* vi: set sw=4 ts=4: */
/*
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 *
 * Authors: Alexey Kuznetsov, <kuznet@ms2.inr.ac.ru>
 *
 * Changes:
 *
 * Rani Assaf <rani@magic.metawire.com> 980929: resolve addresses
 */
#[no_mangle]
pub unsafe extern "C" fn get_hz() -> libc::c_uint {
  static mut hz_internal: libc::c_uint = 0;
  let mut fp: *mut FILE = 0 as *mut FILE;
  if hz_internal != 0 {
    return hz_internal;
  }
  fp = fopen_for_read(b"/proc/net/psched\x00" as *const u8 as *const libc::c_char);
  if !fp.is_null() {
    let mut nom: libc::c_uint = 0;
    let mut denom: libc::c_uint = 0;
    if fscanf(
      fp,
      b"%*08x%*08x%08x%08x\x00" as *const u8 as *const libc::c_char,
      &mut nom as *mut libc::c_uint,
      &mut denom as *mut libc::c_uint,
    ) == 2i32
    {
      if nom == 1000000i32 as libc::c_uint {
        hz_internal = denom
      }
    }
    fclose(fp);
  }
  if hz_internal == 0 {
    hz_internal = bb_clk_tck()
  }
  return hz_internal;
}
#[no_mangle]
pub unsafe extern "C" fn get_unsigned(
  mut arg: *mut libc::c_char,
  mut errmsg: *const libc::c_char,
) -> libc::c_uint {
  let mut res: libc::c_ulong = 0;
  let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
  if *arg != 0 {
    res = strtoul(arg, &mut ptr, 0i32);
    //FIXME: "" will be accepted too, is it correct?!
    if *ptr == 0
      && res
        <= (2147483647i32 as libc::c_uint)
          .wrapping_mul(2u32)
          .wrapping_add(1u32) as libc::c_ulong
    {
      return res as libc::c_uint;
    }
  }
  invarg_1_to_2(arg, errmsg);
  /* does not return */
}
#[no_mangle]
pub unsafe extern "C" fn get_u32(
  mut arg: *mut libc::c_char,
  mut errmsg: *const libc::c_char,
) -> uint32_t {
  let mut res: libc::c_ulong = 0;
  let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
  if *arg != 0 {
    res = strtoul(arg, &mut ptr, 0i32);
    //FIXME: "" will be accepted too, is it correct?!
    if *ptr == 0 && res <= 0xffffffffu64 {
      return res as uint32_t;
    }
  }
  invarg_1_to_2(arg, errmsg);
  /* does not return */
}
#[no_mangle]
pub unsafe extern "C" fn get_u16(
  mut arg: *mut libc::c_char,
  mut errmsg: *const libc::c_char,
) -> uint16_t {
  let mut res: libc::c_ulong = 0;
  let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
  if *arg != 0 {
    res = strtoul(arg, &mut ptr, 0i32);
    //FIXME: "" will be accepted too, is it correct?!
    if *ptr == 0 && res <= 0xffffi32 as libc::c_ulong {
      return res as uint16_t;
    }
  }
  invarg_1_to_2(arg, errmsg);
  /* does not return */
}
#[no_mangle]
pub unsafe extern "C" fn get_addr_1(
  mut addr: *mut inet_prefix,
  mut name: *mut libc::c_char,
  mut family: libc::c_int,
) -> libc::c_int {
  memset(
    addr as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<inet_prefix>() as libc::c_ulong,
  );
  if strcmp(name, b"default\x00" as *const u8 as *const libc::c_char) == 0i32
    || strcmp(name, b"all\x00" as *const u8 as *const libc::c_char) == 0i32
    || strcmp(name, b"any\x00" as *const u8 as *const libc::c_char) == 0i32
  {
    (*addr).family = family as uint8_t;
    (*addr).bytelen = if family == 10i32 { 16i32 } else { 4i32 } as uint8_t;
    (*addr).bitlen = -1i32 as int16_t;
    return 0i32;
  }
  if !strchr(name, ':' as i32).is_null() {
    (*addr).family = 10i32 as uint8_t;
    if family != 0i32 && family != 10i32 {
      return -1i32;
    }
    if inet_pton(10i32, name, (*addr).data.as_mut_ptr() as *mut libc::c_void) <= 0i32 {
      return -1i32;
    }
    (*addr).bytelen = 16i32 as uint8_t;
    (*addr).bitlen = -1i32 as int16_t;
    return 0i32;
  }
  if family != 0i32 && family != 2i32 {
    return -1i32;
  }
  /* Try to parse it as IPv4 */
  (*addr).family = 2i32 as uint8_t;
  /* Doesn't handle e.g. "10.10", for example, "ip r l root 10.10/16" */
  let mut i: libc::c_uint = 0i32 as libc::c_uint;
  let mut n: libc::c_uint = 0i32 as libc::c_uint;
  let mut cp: *const libc::c_char = name.offset(-1);
  loop {
    cp = cp.offset(1);
    if !(*cp != 0) {
      break;
    }
    if (*cp as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32 {
      n = (10i32 as libc::c_uint)
        .wrapping_mul(n)
        .wrapping_add((*cp as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_uint);
      if n >= 256i32 as libc::c_uint {
        return -1i32;
      }
      *((*addr).data.as_mut_ptr() as *mut uint8_t).offset(i as isize) = n as uint8_t
    } else if *cp as libc::c_int == '.' as i32 && {
      i = i.wrapping_add(1);
      (i) <= 3i32 as libc::c_uint
    } {
      n = 0i32 as libc::c_uint
    } else {
      return -1i32;
    }
  }
  (*addr).bytelen = 4i32 as uint8_t;
  (*addr).bitlen = -1i32 as int16_t;
  return 0i32;
}
unsafe extern "C" fn get_prefix_1(
  mut dst: *mut inet_prefix,
  mut arg: *mut libc::c_char,
  mut family: libc::c_int,
) {
  let mut current_block: u64;
  let mut slash: *mut libc::c_char = 0 as *mut libc::c_char;
  memset(
    dst as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<inet_prefix>() as libc::c_ulong,
  );
  if strcmp(arg, b"default\x00" as *const u8 as *const libc::c_char) == 0i32
    || strcmp(arg, b"all\x00" as *const u8 as *const libc::c_char) == 0i32
    || strcmp(arg, b"any\x00" as *const u8 as *const libc::c_char) == 0i32
  {
    (*dst).family = family as uint8_t;
    /*dst->bytelen = 0; - done by memset */
    /*dst->bitlen = 0;*/
    return;
  }
  slash = strchr(arg, '/' as i32);
  if !slash.is_null() {
    *slash = '\u{0}' as i32 as libc::c_char
  }
  if get_addr_1(dst, arg, family) == 0i32 {
    (*dst).bitlen = if (*dst).family as libc::c_int == 10i32 {
      128i32
    } else {
      32i32
    } as int16_t;
    if !slash.is_null() {
      let mut plen: libc::c_uint = 0;
      let mut netmask_pfx: inet_prefix = inet_prefix {
        family: 0,
        bytelen: 0,
        bitlen: 0,
        data: [0; 4],
      };
      netmask_pfx.family = 0i32 as uint8_t;
      plen = bb_strtou(slash.offset(1), 0 as *mut *mut libc::c_char, 0i32);
      if (*bb_errno != 0 || plen > (*dst).bitlen as libc::c_uint)
        && get_addr_1(&mut netmask_pfx, slash.offset(1), family) != 0i32
      {
        current_block = 5733981528044815378;
      } else {
        if netmask_pfx.family as libc::c_int == 2i32 {
          /* fill in prefix length of dotted quad */
          let mut mask: uint32_t = {
            let mut __v: libc::c_uint = 0;
            let mut __x: libc::c_uint = netmask_pfx.data[0];
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
                                      (c2rust_asm_casts::AsmCast::cast_in(fresh0, fresh2))
                                      :);
              c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
            }
            __v
          };
          let mut host: uint32_t = !mask;
          /* a valid netmask must be 2^n - 1 */
          if host & host.wrapping_add(1i32 as libc::c_uint) != 0 {
            current_block = 5733981528044815378;
          } else {
            plen = 0i32 as libc::c_uint;
            while mask != 0 {
              plen = plen.wrapping_add(1);
              mask <<= 1i32
            }
            if plen > (*dst).bitlen as libc::c_uint {
              current_block = 5733981528044815378;
            } else {
              current_block = 11042950489265723346;
            }
          }
        /* dst->flags |= PREFIXLEN_SPECIFIED; */
        } else {
          current_block = 11042950489265723346;
        }
        match current_block {
          5733981528044815378 => {}
          _ => {
            (*dst).bitlen = plen as int16_t;
            current_block = 17478428563724192186;
          }
        }
      }
      match current_block {
        17478428563724192186 => {}
        _ => {
          bb_error_msg_and_die(
            b"an %s %s is expected rather than \"%s\"\x00" as *const u8 as *const libc::c_char,
            b"inet\x00" as *const u8 as *const libc::c_char,
            b"prefix\x00" as *const u8 as *const libc::c_char,
            arg,
          );
        }
      }
    }
  }
  if !slash.is_null() {
    *slash = '/' as i32 as libc::c_char
  };
}
#[no_mangle]
pub unsafe extern "C" fn get_addr(
  mut dst: *mut inet_prefix,
  mut arg: *mut libc::c_char,
  mut family: libc::c_int,
) -> libc::c_int {
  if family == 17i32 {
    bb_error_msg_and_die(
      b"\"%s\" may be inet %s, but it is not allowed in this context\x00" as *const u8
        as *const libc::c_char,
      arg,
      b"address\x00" as *const u8 as *const libc::c_char,
    );
  }
  if get_addr_1(dst, arg, family) != 0 {
    bb_error_msg_and_die(
      b"an %s %s is expected rather than \"%s\"\x00" as *const u8 as *const libc::c_char,
      b"inet\x00" as *const u8 as *const libc::c_char,
      b"address\x00" as *const u8 as *const libc::c_char,
      arg,
    );
  }
  return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn get_prefix(
  mut dst: *mut inet_prefix,
  mut arg: *mut libc::c_char,
  mut family: libc::c_int,
) {
  if family == 17i32 {
    bb_error_msg_and_die(
      b"\"%s\" may be inet %s, but it is not allowed in this context\x00" as *const u8
        as *const libc::c_char,
      arg,
      b"prefix\x00" as *const u8 as *const libc::c_char,
    );
  }
  get_prefix_1(dst, arg, family);
}
#[no_mangle]
pub unsafe extern "C" fn get_addr32(mut name: *mut libc::c_char) -> uint32_t {
  let mut addr: inet_prefix = inet_prefix {
    family: 0,
    bytelen: 0,
    bitlen: 0,
    data: [0; 4],
  };
  if get_addr_1(&mut addr, name, 2i32) != 0 {
    bb_error_msg_and_die(
      b"an %s %s is expected rather than \"%s\"\x00" as *const u8 as *const libc::c_char,
      b"IP\x00" as *const u8 as *const libc::c_char,
      b"address\x00" as *const u8 as *const libc::c_char,
      name,
    );
  }
  return addr.data[0];
}
#[no_mangle]
pub unsafe extern "C" fn next_arg(mut argv: *mut *mut libc::c_char) -> *mut *mut libc::c_char {
  argv = argv.offset(1);
  if (*argv).is_null() {
    bb_simple_error_msg_and_die(
      b"command line is not complete, try \"help\"\x00" as *const u8 as *const libc::c_char,
    );
  }
  return argv;
}
#[no_mangle]
pub unsafe extern "C" fn invarg_1_to_2(
  mut arg: *const libc::c_char,
  mut opt: *const libc::c_char,
) -> ! {
  bb_error_msg_and_die(bb_msg_invalid_arg_to.as_ptr(), arg, opt);
}
#[no_mangle]
pub unsafe extern "C" fn duparg(mut key: *const libc::c_char, mut arg: *const libc::c_char) -> ! {
  bb_error_msg_and_die(
    b"duplicate \"%s\": \"%s\" is the second value\x00" as *const u8 as *const libc::c_char,
    key,
    arg,
  );
}
#[no_mangle]
pub unsafe extern "C" fn duparg2(mut key: *const libc::c_char, mut arg: *const libc::c_char) -> ! {
  bb_error_msg_and_die(
    b"either \"%s\" is duplicate, or \"%s\" is garbage\x00" as *const u8 as *const libc::c_char,
    key,
    arg,
  );
}
#[no_mangle]
pub unsafe extern "C" fn inet_addr_match(
  mut a: *const inet_prefix,
  mut b: *const inet_prefix,
  mut bits: libc::c_int,
) -> libc::c_int {
  let mut a1: *const uint32_t = (*a).data.as_ptr();
  let mut a2: *const uint32_t = (*b).data.as_ptr();
  let mut words: libc::c_int = bits >> 5i32;
  bits &= 0x1fi32;
  if words != 0 {
    if memcmp(
      a1 as *const libc::c_void,
      a2 as *const libc::c_void,
      (words << 2i32) as libc::c_ulong,
    ) != 0
    {
      return -1i32;
    }
  }
  if bits != 0 {
    let mut w1: uint32_t = 0;
    let mut w2: uint32_t = 0;
    let mut mask: uint32_t = 0;
    w1 = *a1.offset(words as isize);
    w2 = *a2.offset(words as isize);
    mask = {
      let mut __v: libc::c_uint = 0;
      let mut __x: libc::c_uint = 0xffffffffu32 << 0x20i32 - bits;
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
                          (c2rust_asm_casts::AsmCast::cast_in(fresh3, fresh5))
                          :);
        c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh5, fresh4);
      }
      __v
    };
    if (w1 ^ w2) & mask != 0 {
      return 1i32;
    }
  }
  return 0i32;
}
/* vi: set sw=4 ts=4: */
/* UNUSED */
/* UNUSED */
/* UNUSED */
/* UNUSED */
/*void get_prefix_1(inet_prefix *dst, char *arg, int family) FAST_FUNC;*/
#[no_mangle]
pub unsafe extern "C" fn rt_addr_n2a(
  mut af: libc::c_int,
  mut addr: *mut libc::c_void,
) -> *const libc::c_char {
  match af {
    2 | 10 => {
      return inet_ntop(
        af,
        addr,
        auto_string(xzalloc(46i32 as size_t) as *mut libc::c_char),
        46i32 as socklen_t,
      )
    }
    _ => return b"???\x00" as *const u8 as *const libc::c_char,
  };
}
