use crate::libbb::ptr_to_globals::bb_errno;
use crate::libbb::xfuncs_printf::xmalloc;
use crate::librb::len_and_sockaddr;
use crate::librb::size_t;
use c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use libc;
use libc::free;
use libc::sa_family_t;
use libc::sockaddr;
use libc::sockaddr_in;
use libc::sockaddr_in6;
use libc::sprintf;
use libc::strchr;
use libc::strtok;
extern "C" {

  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;

  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;

  /* Some useful definitions */
  /* Macros for min/max.  */
  /* buffer allocation schemes */
  /* glibc uses __errno_location() to get a ptr to errno */
  /* We can just memorize it once - no multithreading in busybox :) */

  /* NB: can violate const-ness (similarly to strchr) */

  /* Same, useful if you want to force family (e.g. IPv6) */

  /* Put a string of hex bytes ("1b2e66fe"...), return advanced pointer */

  /* Reverse */

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

  // #[no_mangle]
  // fn BUG_bb_strtou32_unimplemented() -> u32;

  #[no_mangle]
  fn strnlen(__string: *const libc::c_char, __maxlen: size_t) -> size_t;
}

pub type __socklen_t = libc::c_uint;
pub type bb__aliased_u32 = u32;
pub type in_port_t = u16;
pub type in_addr_t = u32;

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_0 {
  pub sa: sockaddr,
  pub sin: sockaddr_in,
  pub sin6: sockaddr_in6,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct dhcp_packet {
  pub op: u8,
  pub htype: u8,
  pub hlen: u8,
  pub hops: u8,
  pub xid: u32,
  pub secs: u16,
  pub flags: u16,
  pub ciaddr: u32,
  pub yiaddr: u32,
  pub siaddr_nip: u32,
  pub gateway_nip: u32,
  pub chaddr: [u8; 16],
  pub sname: [u8; 64],
  pub file: [u8; 128],
  pub cookie: u32,
  pub options: [u8; 388],
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub const OPTION_LIST: C2RustUnnamed_1 = 32;
pub const OPTION_REQ: C2RustUnnamed_1 = 16;
pub const OPTION_TYPE_MASK: C2RustUnnamed_1 = 15;
pub const OPTION_SIP_SERVERS: C2RustUnnamed_1 = 13;
pub const OPTION_DNS_STRING: C2RustUnnamed_1 = 12;
pub const OPTION_6RD: C2RustUnnamed_1 = 11;
pub const OPTION_STATIC_ROUTES: C2RustUnnamed_1 = 10;
pub const OPTION_BIN: C2RustUnnamed_1 = 9;
pub const OPTION_S32: C2RustUnnamed_1 = 8;
pub const OPTION_U32: C2RustUnnamed_1 = 7;
pub const OPTION_U16: C2RustUnnamed_1 = 6;
pub const OPTION_U8: C2RustUnnamed_1 = 5;
pub const OPTION_STRING_HOST: C2RustUnnamed_1 = 4;
pub const OPTION_STRING: C2RustUnnamed_1 = 3;
pub const OPTION_IP_PAIR: C2RustUnnamed_1 = 2;
pub const OPTION_IP: C2RustUnnamed_1 = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dhcp_optflag {
  pub flags: u8,
  pub code: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct option_set {
  pub data: *mut u8,
  pub next: *mut option_set,
}
pub const SNAME_FIELD101: C2RustUnnamed_2 = 514;
pub const FILE_FIELD101: C2RustUnnamed_2 = 257;
pub type C2RustUnnamed_2 = libc::c_uint;

pub fn BUG_bb_strtou32_unimplemented() -> u32 {
  panic!("BUG_bb_strtou32_unimplemented")
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
unsafe extern "C" fn bb_strtol(
  mut arg: *const libc::c_char,
  mut endp: *mut *mut libc::c_char,
  mut base: libc::c_int,
) -> libc::c_long {
  return crate::libbb::bb_strtonum::bb_strtoll(arg, endp, base) as libc::c_long;
}
#[inline(always)]
unsafe extern "C" fn bb_strtou32(
  mut arg: *const libc::c_char,
  mut endp: *mut *mut libc::c_char,
  mut base: libc::c_int,
) -> u32 {
  if ::std::mem::size_of::<u32>() as libc::c_ulong
    == ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong
  {
    return crate::libbb::bb_strtonum::bb_strtou(arg, endp, base);
  }
  if ::std::mem::size_of::<u32>() as libc::c_ulong
    == ::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong
  {
    return bb_strtoul(arg, endp, base) as u32;
  }
  return BUG_bb_strtou32_unimplemented();
}
#[inline(always)]
unsafe extern "C" fn bb_strtoi32(
  mut arg: *const libc::c_char,
  mut endp: *mut *mut libc::c_char,
  mut base: libc::c_int,
) -> i32 {
  if ::std::mem::size_of::<i32>() as libc::c_ulong
    == ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
  {
    return crate::libbb::bb_strtonum::bb_strtoi(arg, endp, base);
  }
  if ::std::mem::size_of::<i32>() as libc::c_ulong
    == ::std::mem::size_of::<libc::c_long>() as libc::c_ulong
  {
    return bb_strtol(arg, endp, base) as i32;
  }
  return BUG_bb_strtou32_unimplemented() as i32;
}

/*
 * Rewrite by Russ Dill <Russ.Dill@asu.edu> July 2001
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
#[no_mangle]
pub static mut dhcp_verbose: libc::c_uint = 0;
#[no_mangle]
pub static mut MAC_BCAST_ADDR: [u8; 6] = [
  0xffi32 as u8,
  0xffi32 as u8,
  0xffi32 as u8,
  0xffi32 as u8,
  0xffi32 as u8,
  0xffi32 as u8,
];
/* Supported options are easily added here.
 * See RFC2132 for more options.
 * OPTION_REQ: these options are requested by udhcpc (unless -o).
 */
#[no_mangle]
pub static mut dhcp_optflags: [dhcp_optflag; 40] = [
  {
    let mut init = dhcp_optflag {
      flags: (OPTION_IP as libc::c_int | OPTION_REQ as libc::c_int) as u8,
      code: 0x1i32 as u8,
    };
    init
  },
  {
    let mut init = dhcp_optflag {
      flags: OPTION_S32 as libc::c_int as u8,
      code: 0x2i32 as u8,
    };
    init
  },
  {
    let mut init = dhcp_optflag {
      flags: (OPTION_IP as libc::c_int | OPTION_LIST as libc::c_int | OPTION_REQ as libc::c_int)
        as u8,
      code: 0x3i32 as u8,
    };
    init
  },
  {
    let mut init = dhcp_optflag {
      flags: (OPTION_IP as libc::c_int | OPTION_LIST as libc::c_int | OPTION_REQ as libc::c_int)
        as u8,
      code: 0x6i32 as u8,
    };
    init
  },
  {
    let mut init = dhcp_optflag {
      flags: (OPTION_IP as libc::c_int | OPTION_LIST as libc::c_int) as u8,
      code: 0x9i32 as u8,
    };
    init
  },
  {
    let mut init = dhcp_optflag {
      flags: (OPTION_STRING_HOST as libc::c_int | OPTION_REQ as libc::c_int) as u8,
      code: 0xci32 as u8,
    };
    init
  },
  {
    let mut init = dhcp_optflag {
      flags: OPTION_U16 as libc::c_int as u8,
      code: 0xdi32 as u8,
    };
    init
  },
  {
    let mut init = dhcp_optflag {
      flags: (OPTION_STRING_HOST as libc::c_int | OPTION_REQ as libc::c_int) as u8,
      code: 0xfi32 as u8,
    };
    init
  },
  {
    let mut init = dhcp_optflag {
      flags: OPTION_IP as libc::c_int as u8,
      code: 0x10i32 as u8,
    };
    init
  },
  {
    let mut init = dhcp_optflag {
      flags: OPTION_STRING as libc::c_int as u8,
      code: 0x11i32 as u8,
    };
    init
  },
  {
    let mut init = dhcp_optflag {
      flags: OPTION_U8 as libc::c_int as u8,
      code: 0x17i32 as u8,
    };
    init
  },
  {
    let mut init = dhcp_optflag {
      flags: OPTION_U16 as libc::c_int as u8,
      code: 0x1ai32 as u8,
    };
    init
  },
  {
    let mut init = dhcp_optflag {
      flags: (OPTION_IP as libc::c_int | OPTION_REQ as libc::c_int) as u8,
      code: 0x1ci32 as u8,
    };
    init
  },
  {
    let mut init = dhcp_optflag {
      flags: (OPTION_IP_PAIR as libc::c_int | OPTION_LIST as libc::c_int) as u8,
      code: 0x21i32 as u8,
    };
    init
  },
  {
    let mut init = dhcp_optflag {
      flags: OPTION_STRING_HOST as libc::c_int as u8,
      code: 0x28i32 as u8,
    };
    init
  },
  {
    let mut init = dhcp_optflag {
      flags: (OPTION_IP as libc::c_int | OPTION_LIST as libc::c_int) as u8,
      code: 0x29i32 as u8,
    };
    init
  },
  {
    let mut init = dhcp_optflag {
      flags: (OPTION_IP as libc::c_int | OPTION_LIST as libc::c_int | OPTION_REQ as libc::c_int)
        as u8,
      code: 0x2ai32 as u8,
    };
    init
  },
  {
    let mut init = dhcp_optflag {
      flags: (OPTION_IP as libc::c_int | OPTION_LIST as libc::c_int) as u8,
      code: 0x2ci32 as u8,
    };
    init
  },
  {
    let mut init = dhcp_optflag {
      flags: OPTION_U32 as libc::c_int as u8,
      code: 0x33i32 as u8,
    };
    init
  },
  {
    let mut init = dhcp_optflag {
      flags: OPTION_IP as libc::c_int as u8,
      code: 0x36i32 as u8,
    };
    init
  },
  {
    let mut init = dhcp_optflag {
      flags: OPTION_STRING as libc::c_int as u8,
      code: 0x38i32 as u8,
    };
    init
  },
  {
    let mut init = dhcp_optflag {
      flags: OPTION_STRING_HOST as libc::c_int as u8,
      code: 0x42i32 as u8,
    };
    init
  },
  {
    let mut init = dhcp_optflag {
      flags: OPTION_STRING as libc::c_int as u8,
      code: 0x43i32 as u8,
    };
    init
  },
  {
    let mut init = dhcp_optflag {
      flags: OPTION_STRING as libc::c_int as u8,
      code: 0x64i32 as u8,
    };
    init
  },
  {
    let mut init = dhcp_optflag {
      flags: OPTION_STRING as libc::c_int as u8,
      code: 0x65i32 as u8,
    };
    init
  },
  {
    let mut init = dhcp_optflag {
      flags: (OPTION_DNS_STRING as libc::c_int | OPTION_LIST as libc::c_int) as u8,
      code: 0x77i32 as u8,
    };
    init
  },
  {
    let mut init = dhcp_optflag {
      flags: OPTION_SIP_SERVERS as libc::c_int as u8,
      code: 0x78i32 as u8,
    };
    init
  },
  {
    let mut init = dhcp_optflag {
      flags: (OPTION_STATIC_ROUTES as libc::c_int | OPTION_LIST as libc::c_int) as u8,
      code: 0x79i32 as u8,
    };
    init
  },
  {
    let mut init = dhcp_optflag {
      flags: OPTION_U16 as libc::c_int as u8,
      code: 0x84i32 as u8,
    };
    init
  },
  {
    let mut init = dhcp_optflag {
      flags: OPTION_U8 as libc::c_int as u8,
      code: 0x85i32 as u8,
    };
    init
  },
  {
    let mut init = dhcp_optflag {
      flags: OPTION_STRING as libc::c_int as u8,
      code: 0xd1i32 as u8,
    };
    init
  },
  {
    let mut init = dhcp_optflag {
      flags: OPTION_STRING as libc::c_int as u8,
      code: 0xd2i32 as u8,
    };
    init
  },
  {
    let mut init = dhcp_optflag {
      flags: OPTION_U32 as libc::c_int as u8,
      code: 0xd3i32 as u8,
    };
    init
  },
  {
    let mut init = dhcp_optflag {
      flags: OPTION_6RD as libc::c_int as u8,
      code: 0xd4i32 as u8,
    };
    init
  },
  {
    let mut init = dhcp_optflag {
      flags: (OPTION_STATIC_ROUTES as libc::c_int | OPTION_LIST as libc::c_int) as u8,
      code: 0xf9i32 as u8,
    };
    init
  },
  {
    let mut init = dhcp_optflag {
      flags: OPTION_STRING as libc::c_int as u8,
      code: 0xfci32 as u8,
    };
    init
  },
  {
    let mut init = dhcp_optflag {
      flags: OPTION_IP as libc::c_int as u8,
      code: 0x32i32 as u8,
    };
    init
  },
  {
    let mut init = dhcp_optflag {
      flags: OPTION_U8 as libc::c_int as u8,
      code: 0x35i32 as u8,
    };
    init
  },
  {
    let mut init = dhcp_optflag {
      flags: OPTION_U16 as libc::c_int as u8,
      code: 0x39i32 as u8,
    };
    init
  },
  {
    let mut init = dhcp_optflag {
      flags: 0i32 as u8,
      code: 0i32 as u8,
    };
    init
  },
];
/* Used for converting options from incoming packets to env variables
 * for udhcpc script, and for setting options for udhcpd via
 * "opt OPTION_NAME OPTION_VALUE" directives in udhcpd.conf file.
 */
/* Must match dhcp_optflags[] order */
#[no_mangle]
pub static mut dhcp_option_strings: [libc::c_char; 294] = [
  115, 117, 98, 110, 101, 116, 0, 116, 105, 109, 101, 122, 111, 110, 101, 0, 114, 111, 117, 116,
  101, 114, 0, 100, 110, 115, 0, 108, 112, 114, 115, 114, 118, 0, 104, 111, 115, 116, 110, 97, 109,
  101, 0, 98, 111, 111, 116, 115, 105, 122, 101, 0, 100, 111, 109, 97, 105, 110, 0, 115, 119, 97,
  112, 115, 114, 118, 0, 114, 111, 111, 116, 112, 97, 116, 104, 0, 105, 112, 116, 116, 108, 0, 109,
  116, 117, 0, 98, 114, 111, 97, 100, 99, 97, 115, 116, 0, 114, 111, 117, 116, 101, 115, 0, 110,
  105, 115, 100, 111, 109, 97, 105, 110, 0, 110, 105, 115, 115, 114, 118, 0, 110, 116, 112, 115,
  114, 118, 0, 119, 105, 110, 115, 0, 108, 101, 97, 115, 101, 0, 115, 101, 114, 118, 101, 114, 105,
  100, 0, 109, 101, 115, 115, 97, 103, 101, 0, 116, 102, 116, 112, 0, 98, 111, 111, 116, 102, 105,
  108, 101, 0, 116, 122, 115, 116, 114, 0, 116, 122, 100, 98, 115, 116, 114, 0, 115, 101, 97, 114,
  99, 104, 0, 115, 105, 112, 115, 114, 118, 0, 115, 116, 97, 116, 105, 99, 114, 111, 117, 116, 101,
  115, 0, 118, 108, 97, 110, 105, 100, 0, 118, 108, 97, 110, 112, 114, 105, 111, 114, 105, 116,
  121, 0, 112, 120, 101, 99, 111, 110, 102, 102, 105, 108, 101, 0, 112, 120, 101, 112, 97, 116,
  104, 112, 114, 101, 102, 105, 120, 0, 114, 101, 98, 111, 111, 116, 116, 105, 109, 101, 0, 105,
  112, 54, 114, 100, 0, 109, 115, 115, 116, 97, 116, 105, 99, 114, 111, 117, 116, 101, 115, 0, 119,
  112, 97, 100, 0, 0,
];
/* DHCP_WPAD            */
/* Lengths of the option types in binary form.
 * Used by:
 * udhcp_str2optset: to determine how many bytes to allocate.
 * xmalloc_optname_optval: to estimate string length
 * from binary option length: (option[LEN] / dhcp_option_lengths[opt_type])
 * is the number of elements, multiply it by one element's string width
 * (len_of_option_as_string[opt_type]) and you know how wide string you need.
 */
#[no_mangle]
pub static mut dhcp_option_lengths: [u8; 14] = [
  0,
  4i32 as u8,
  8i32 as u8,
  1i32 as u8,
  1i32 as u8,
  1i32 as u8,
  2i32 as u8,
  4i32 as u8,
  4i32 as u8,
  0,
  5i32 as u8,
  12i32 as u8,
  1i32 as u8,
  1i32 as u8,
];
unsafe extern "C" fn log_option(mut pfx: *const libc::c_char, mut opt: *const u8) {
  if dhcp_verbose >= 2i32 as libc::c_uint {
    let mut buf: [libc::c_char; 514] = [0; 514];
    *crate::libbb::xfuncs::bin2hex(
      buf.as_mut_ptr(),
      opt.offset(2) as *mut libc::c_void as *const libc::c_char,
      *opt.offset(1) as libc::c_int,
    ) = '\u{0}' as i32 as libc::c_char;
    crate::libbb::verror_msg::bb_info_msg(
      b"%s: 0x%02x %s\x00" as *const u8 as *const libc::c_char,
      pfx,
      *opt.offset(0) as libc::c_int,
      buf.as_mut_ptr(),
    );
  };
}
#[no_mangle]
pub unsafe extern "C" fn udhcp_option_idx(
  mut name: *const libc::c_char,
  mut option_strings: *const libc::c_char,
) -> libc::c_uint {
  let mut n: libc::c_int =
    crate::libbb::compare_string_array::index_in_strings(option_strings, name);
  if n >= 0i32 {
    return n as libc::c_uint;
  }
  let mut buf: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut d: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut s: *const libc::c_char = std::ptr::null();
  s = option_strings;
  while *s != 0 {
    s = s.offset(strlen(s).wrapping_add(1i32 as libc::c_ulong) as isize)
  }
  buf = crate::libbb::xfuncs_printf::xzalloc(
    s.wrapping_offset_from(option_strings) as libc::c_long as size_t
  ) as *mut libc::c_char;
  d = buf;
  s = option_strings;
  while !(*s as libc::c_int == '\u{0}' as i32 && *s.offset(1) as libc::c_int == '\u{0}' as i32) {
    let fresh0 = d;
    d = d.offset(1);
    *fresh0 = if *s as libc::c_int == '\u{0}' as i32 {
      ' ' as i32
    } else {
      *s as libc::c_int
    } as libc::c_char;
    s = s.offset(1)
  }
  crate::libbb::verror_msg::bb_error_msg_and_die(
    b"unknown option \'%s\', known options: %s\x00" as *const u8 as *const libc::c_char,
    name,
    buf,
  );
}
/* Get an option with bounds checking (warning, result is not aligned) */
#[no_mangle]
pub unsafe extern "C" fn udhcp_get_option(
  mut packet: *mut dhcp_packet,
  mut code: libc::c_int,
) -> *mut u8 {
  let mut optionptr: *mut u8 = std::ptr::null_mut();
  let mut len: libc::c_int = 0;
  let mut rem: libc::c_int = 0;
  let mut overload: libc::c_int = 0i32;
  /* option bytes: [code][len][data1][data2]..[dataLEN] */
  optionptr = (*packet).options.as_mut_ptr();
  rem = ::std::mem::size_of::<[u8; 388]>() as libc::c_ulong as libc::c_int;
  loop {
    if !(rem <= 0i32) {
      /* DHCP_PADDING and DHCP_END have no [len] byte */
      if *optionptr.offset(0) as libc::c_int == 0i32 {
        rem -= 1;
        optionptr = optionptr.offset(1);
        continue;
      } else if *optionptr.offset(0) as libc::c_int == 0xffi32 {
        if overload & FILE_FIELD101 as libc::c_int == 1i32 {
          /* can use packet->file, and didn't look at it yet */
          overload |= FILE_FIELD101 as libc::c_int; /* "we looked at it" */
          optionptr = (*packet).file.as_mut_ptr();
          rem = ::std::mem::size_of::<[u8; 128]>() as libc::c_ulong as libc::c_int;
          continue;
        } else {
          if !(overload & SNAME_FIELD101 as libc::c_int == 2i32) {
            break;
          }
          /* can use packet->sname, and didn't look at it yet */
          overload |= SNAME_FIELD101 as libc::c_int; /* "we looked at it" */
          optionptr = (*packet).sname.as_mut_ptr(); /* complain and return NULL */
          rem = ::std::mem::size_of::<[u8; 64]>() as libc::c_ulong as libc::c_int; /* complain and return NULL */
          continue;
        }
      } else if !(rem <= 1i32) {
        len = 2i32 + *optionptr.offset(1) as libc::c_int;
        rem -= len;
        if !(rem < 0i32) {
          if *optionptr.offset(0) as libc::c_int == code {
            if !(*optionptr.offset(1) as libc::c_int == 0i32) {
              log_option(
                b"option found\x00" as *const u8 as *const libc::c_char,
                optionptr,
              );
              return optionptr.offset(2);
            }
          } else {
            if *optionptr.offset(0) as libc::c_int == 0x34i32 {
              if len >= 3i32 {
                overload |= *optionptr.offset(2) as libc::c_int
              }
              /* fall through */
            }
            optionptr = optionptr.offset(len as isize);
            continue;
          }
        }
      }
    }
    /* So far no valid option with length 0 known.
     * Having this check means that searching
     * for DHCP_MESSAGE_TYPE need not worry
     * that returned pointer might be unsafe
     * to dereference.
     */
    crate::libbb::verror_msg::bb_simple_error_msg(
      b"bad packet, malformed option field\x00" as *const u8 as *const libc::c_char,
    );
    return 0 as *mut u8;
  }
  /* log3 because udhcpc uses it a lot - very noisy */
  return 0 as *mut u8;
}
#[no_mangle]
pub unsafe extern "C" fn udhcp_get_option32(
  mut packet: *mut dhcp_packet,
  mut code: libc::c_int,
) -> *mut u8 {
  let mut r: *mut u8 = udhcp_get_option(packet, code);
  if !r.is_null() {
    if *r.offset((-2i32 + 1i32) as isize) as libc::c_int != 4i32 {
      r = std::ptr::null_mut()
    }
  }
  return r;
}
/* Return the position of the 'end' option (no bounds checking) */
#[no_mangle]
pub unsafe extern "C" fn udhcp_end_option(mut optionptr: *mut u8) -> libc::c_int {
  let mut i: libc::c_int = 0i32;
  while *optionptr.offset(i as isize) as libc::c_int != 0xffi32 {
    if *optionptr.offset(i as isize) as libc::c_int != 0i32 {
      i += *optionptr.offset((i + 1i32) as isize) as libc::c_int + 2i32 - 1i32
    }
    i += 1
  }
  return i;
}
/* Add an option (supplied in binary form) to the options.
 * Option format: [code][len][data1][data2]..[dataLEN]
 */
#[no_mangle]
pub unsafe extern "C" fn udhcp_add_binary_option(
  mut packet: *mut dhcp_packet,
  mut addopt: *mut u8,
) {
  let mut len: libc::c_uint = 0;
  let mut optionptr: *mut u8 = (*packet).options.as_mut_ptr();
  let mut end: libc::c_uint = udhcp_end_option(optionptr) as libc::c_uint;
  len = (2i32 + *addopt.offset(1) as libc::c_int) as libc::c_uint;
  /* end position + (option code/length + addopt length) + end option */
  if end.wrapping_add(len).wrapping_add(1i32 as libc::c_uint) >= 308i32 as libc::c_uint {
    //TODO: learn how to use overflow option if we exhaust packet->options[]
    crate::libbb::verror_msg::bb_error_msg(
      b"option 0x%02x did not fit into the packet\x00" as *const u8 as *const libc::c_char,
      *addopt.offset(0) as libc::c_int,
    );
    return;
  }
  log_option(
    b"adding option\x00" as *const u8 as *const libc::c_char,
    addopt,
  );
  memcpy(
    optionptr.offset(end as isize) as *mut libc::c_void,
    addopt as *const libc::c_void,
    len as libc::c_ulong,
  );
  *optionptr.offset(end.wrapping_add(len) as isize) = 0xffi32 as u8;
}
/* Add an one to four byte option to a packet */
#[no_mangle]
pub unsafe extern "C" fn udhcp_add_simple_option(
  mut packet: *mut dhcp_packet,
  mut code: u8,
  mut data: u32,
) {
  let mut dh: *const dhcp_optflag = std::ptr::null();
  dh = dhcp_optflags.as_ptr();
  while (*dh).code != 0 {
    if (*dh).code as libc::c_int == code as libc::c_int {
      let mut option: [u8; 6] = [0; 6];
      let mut len: u8 = 0;
      option[0] = code;
      len = dhcp_option_lengths
        [((*dh).flags as libc::c_int & OPTION_TYPE_MASK as libc::c_int) as usize];
      option[1] = len;
      /* Assignment is unaligned! */
      *(&mut *option.as_mut_ptr().offset(2) as *mut u8 as *mut bb__aliased_u32) = data;
      udhcp_add_binary_option(packet, option.as_mut_ptr());
      return;
    }
    dh = dh.offset(1)
  }
  crate::libbb::verror_msg::bb_error_msg(
    b"can\'t add option 0x%02x\x00" as *const u8 as *const libc::c_char,
    code as libc::c_int,
  );
}
/* Find option 'code' in opt_list */
#[no_mangle]
pub unsafe extern "C" fn udhcp_find_option(
  mut opt_list: *mut option_set,
  mut code: u8,
) -> *mut option_set {
  while !opt_list.is_null() && (*(*opt_list).data.offset(0) as libc::c_int) < code as libc::c_int {
    opt_list = (*opt_list).next
  }
  if !opt_list.is_null() && *(*opt_list).data.offset(0) as libc::c_int == code as libc::c_int {
    return opt_list;
  }
  return 0 as *mut option_set;
}
/* Parse string to IP in network order */
#[no_mangle]
pub unsafe extern "C" fn udhcp_str2nip(
  mut str: *const libc::c_char,
  mut arg: *mut libc::c_void,
) -> libc::c_int {
  let mut lsa: *mut len_and_sockaddr = std::ptr::null_mut();
  lsa = crate::libbb::xconnect::host_and_af2sockaddr(str, 0i32, 2i32 as sa_family_t);
  if lsa.is_null() {
    return 0i32;
  }
  /* arg maybe unaligned */
  *(arg as *mut u32 as *mut bb__aliased_u32) = (*lsa).u.sin.sin_addr.s_addr;
  free(lsa as *mut libc::c_void);
  return 1i32;
}
/* udhcp_str2optset:
 * Parse string option representation to binary form and add it to opt_list.
 * Called to parse "udhcpc -x OPTNAME:OPTVAL"
 * and to parse udhcpd.conf's "opt OPTNAME OPTVAL" directives.
 */
/* helper: add an option to the opt_list */
#[inline(never)]
unsafe extern "C" fn attach_option(
  mut opt_list: *mut *mut option_set,
  mut optflag: *const dhcp_optflag,
  mut buffer: *mut libc::c_char,
  mut length: libc::c_int,
  mut dhcpv6: bool,
) {
  let mut existing: *mut option_set = std::ptr::null_mut(); /* more than enough */
  let mut allocated: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  if (*optflag).flags as libc::c_int & OPTION_TYPE_MASK as libc::c_int == OPTION_BIN as libc::c_int
  {
    let mut end: *const libc::c_char = std::ptr::null();
    allocated = crate::libbb::xfuncs_printf::xstrdup(buffer);
    end = crate::libbb::xfuncs::hex2bin(allocated, buffer, 255i32);
    if *bb_errno != 0 {
      crate::libbb::verror_msg::bb_error_msg_and_die(
        b"malformed hex string \'%s\'\x00" as *const u8 as *const libc::c_char,
        buffer,
      );
    }
    length = end.wrapping_offset_from(allocated) as libc::c_long as libc::c_int;
    buffer = allocated
  }
  if (*optflag).flags as libc::c_int & OPTION_TYPE_MASK as libc::c_int
    == OPTION_DNS_STRING as libc::c_int
  {
    /* reuse buffer and length for RFC1035-formatted string */
    buffer =
      crate::networking::udhcp::domain_codec::dname_enc(0 as *const u8, 0i32, buffer, &mut length)
        as *mut libc::c_char;
    allocated = buffer
  }
  existing = udhcp_find_option(*opt_list, (*optflag).code);
  if existing.is_null() {
    let mut new: *mut option_set = std::ptr::null_mut();
    let mut curr: *mut *mut option_set = std::ptr::null_mut();
    /* make a new option */
    if dhcp_verbose >= 2i32 as libc::c_uint {
      crate::libbb::verror_msg::bb_info_msg(
        b"attaching option %02x to list\x00" as *const u8 as *const libc::c_char,
        (*optflag).code as libc::c_int,
      ); /* else, ignore the new data */
    }
    new = xmalloc(::std::mem::size_of::<option_set>() as libc::c_ulong) as *mut option_set;
    if !dhcpv6 {
      (*new).data = xmalloc((length + 2i32) as size_t) as *mut u8;
      *(*new).data.offset(0) = (*optflag).code;
      *(*new).data.offset(1) = length as u8;
      memcpy(
        (*new).data.offset(2) as *mut libc::c_void,
        buffer as *const libc::c_void,
        length as libc::c_ulong,
      );
    } else {
      (*new).data = xmalloc((length + 4i32) as size_t) as *mut u8;
      *(*new).data.offset(0) = ((*optflag).code as libc::c_int >> 8i32) as u8;
      *(*new).data.offset((0i32 + 1i32) as isize) =
        ((*optflag).code as libc::c_int & 0xffi32) as u8;
      *(*new).data.offset(2) = (length >> 8i32) as u8;
      *(*new).data.offset((2i32 + 1i32) as isize) = (length & 0xffi32) as u8;
      memcpy(
        (*new).data.offset(4) as *mut libc::c_void,
        buffer as *const libc::c_void,
        length as libc::c_ulong,
      );
    }
    curr = opt_list;
    while !(*curr).is_null()
      && (*(**curr).data.offset(0) as libc::c_int) < (*optflag).code as libc::c_int
    {
      curr = &mut (**curr).next
    }
    (*new).next = *curr;
    *curr = new
  } else if (*optflag).flags as libc::c_int & OPTION_LIST as libc::c_int != 0 {
    let mut old_len: libc::c_uint = 0;
    /* else, ignore the data, we could put this in a second option in the future */
    if dhcp_verbose >= 2i32 as libc::c_uint {
      crate::libbb::verror_msg::bb_info_msg(
        b"attaching option %02x to existing member of list\x00" as *const u8 as *const libc::c_char,
        (*optflag).code as libc::c_int,
      );
    }
    old_len = *(*existing).data.offset(1) as libc::c_uint;
    if old_len.wrapping_add(length as libc::c_uint) < 255i32 as libc::c_uint {
      /* add it to an existing option */
      /* actually 255 is ok too, but adding a space can overlow it */
      (*existing).data = crate::libbb::xfuncs_printf::xrealloc(
        (*existing).data as *mut libc::c_void,
        ((2i32 + 1i32) as libc::c_uint)
          .wrapping_add(old_len)
          .wrapping_add(length as libc::c_uint) as size_t,
      ) as *mut u8;
      // So far dhcp_optflags[] has no OPTION_STRING[_HOST] | OPTION_LIST items
      memcpy(
        (*existing).data.offset(2).offset(old_len as isize) as *mut libc::c_void,
        buffer as *const libc::c_void,
        length as libc::c_ulong,
      );
      *(*existing).data.offset(1) = old_len.wrapping_add(length as libc::c_uint) as u8
    }
  }
  free(allocated as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn udhcp_str2optset(
  mut const_str: *const libc::c_char,
  mut arg: *mut libc::c_void,
  mut optflags: *const dhcp_optflag,
  mut option_strings: *const libc::c_char,
  mut dhcpv6: bool,
) -> libc::c_int {
  let mut opt_list: *mut *mut option_set = arg as *mut *mut option_set;
  let mut opt: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut str: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut optflag: *const dhcp_optflag = std::ptr::null();
  let mut userdef_optflag: dhcp_optflag = dhcp_optflag { flags: 0, code: 0 };
  let mut optcode: libc::c_uint = 0;
  let mut retval: libc::c_int = 0;
  /* IP_PAIR needs 8 bytes, STATIC_ROUTES needs 9 max */
  let mut buffer: [libc::c_char; 9] = [0; 9];
  let mut result_u16: *mut u16 = buffer.as_mut_ptr() as *mut u16;
  let mut result_u32: *mut u32 = buffer.as_mut_ptr() as *mut u32;
  /* Cheat, the only *const* str possible is "" */
  str = const_str as *mut libc::c_char;
  opt = strtok(str, b" \t=:\x00" as *const u8 as *const libc::c_char);
  if opt.is_null() {
    return 0i32;
  }
  optcode = crate::libbb::bb_strtonum::bb_strtou(opt, 0 as *mut *mut libc::c_char, 0i32);
  if *bb_errno == 0 && optcode < 255i32 as libc::c_uint {
    /* Raw (numeric) option code.
     * Initially assume binary (hex-str), but if "str" or 'str'
     * is seen later, switch to STRING.
     */
    userdef_optflag.flags = OPTION_BIN as libc::c_int as u8;
    userdef_optflag.code = optcode as u8;
    optflag = &mut userdef_optflag
  } else {
    optflag = &*optflags.offset((udhcp_option_idx
      as unsafe extern "C" fn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_uint)(
      opt,
      option_strings,
    ) as isize) as *const dhcp_optflag
  }
  /* Loop to handle OPTION_LIST case, else execute just once */
  retval = 0i32; /* do not split "'q w e'" */
  loop {
    let mut length: libc::c_int = 0; /* new meaning for variable opt */
    let mut val: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    if (*optflag).flags as libc::c_int == OPTION_BIN as libc::c_int {
      val = strtok(
        std::ptr::null_mut::<libc::c_char>(),
        b"\x00" as *const u8 as *const libc::c_char,
      );
      if !val.is_null() {
        crate::libbb::trim::trim(val);
      }
    } else {
      val = strtok(
        std::ptr::null_mut::<libc::c_char>(),
        b", \t\x00" as *const u8 as *const libc::c_char,
      )
    }
    if val.is_null() {
      break;
    }
    length = dhcp_option_lengths
      [((*optflag).flags as libc::c_int & OPTION_TYPE_MASK as libc::c_int) as usize]
      as libc::c_int;
    retval = 0i32;
    opt = buffer.as_mut_ptr();
    let mut current_block_58: u64;
    match (*optflag).flags as libc::c_int & OPTION_TYPE_MASK as libc::c_int {
      1 => {
        retval = udhcp_str2nip(val, buffer.as_mut_ptr() as *mut libc::c_void);
        current_block_58 = 10778260831612459202;
      }
      2 => {
        retval = udhcp_str2nip(val, buffer.as_mut_ptr() as *mut libc::c_void);
        val = strtok(
          std::ptr::null_mut::<libc::c_char>(),
          b", \t/-\x00" as *const u8 as *const libc::c_char,
        );
        if val.is_null() {
          retval = 0i32
        }
        if retval != 0 {
          retval = udhcp_str2nip(val, buffer.as_mut_ptr().offset(4) as *mut libc::c_void)
        }
        current_block_58 = 10778260831612459202;
      }
      3 | 4 | 12 => {
        current_block_58 = 17106587497971209820;
      }
      5 => {
        //		case OPTION_BOOLEAN: {
        //			static const char no_yes[] ALIGN1 = "no\0yes\0";
        //			buffer[0] = retval = index_in_strings(no_yes, val);
        //			retval++; /* 0 - bad; 1: "no" 2: "yes" */
        //			break;
        //		}
        buffer[0] = bb_strtou32(val, 0 as *mut *mut libc::c_char, 0i32) as libc::c_char;
        retval = (*bb_errno == 0i32) as libc::c_int;
        current_block_58 = 10778260831612459202;
      }
      6 => {
        /* htonX are macros in older libc's, using temp var
         * in code below for safety */
        /* TODO: use bb_strtoX? */
        let mut tmp: u32 = bb_strtou32(val, 0 as *mut *mut libc::c_char, 0i32);
        *result_u16 = {
          let mut __v: libc::c_ushort = 0;
          let mut __x: libc::c_ushort = tmp as libc::c_ushort;
          if false {
            __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
              as libc::c_ushort
          } else {
            let fresh1 = &mut __v;
            let fresh2;
            let fresh3 = __x;
            asm!("rorw $$8, ${0:w}" : "=r" (fresh2) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh1, fresh3)) : "cc");
            c2rust_asm_casts::AsmCast::cast_out(fresh1, fresh3, fresh2);
          }
          __v
        };
        retval = (*bb_errno == 0i32) as libc::c_int;
        current_block_58 = 10778260831612459202;
      }
      7 => {
        //		case OPTION_S16: {
        //			long tmp = bb_strtoi32(val, NULL, 0);
        //			*result_u16 = htons(tmp);
        //			retval = (errno == 0);
        //			break;
        //		}
        let mut tmp_0: u32 = bb_strtou32(val, 0 as *mut *mut libc::c_char, 0i32);
        *result_u32 = {
          let mut __v: libc::c_uint = 0;
          let mut __x: libc::c_uint = tmp_0;
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
        };
        retval = (*bb_errno == 0i32) as libc::c_int;
        current_block_58 = 10778260831612459202;
      }
      8 => {
        let mut tmp_1: i32 = bb_strtoi32(val, 0 as *mut *mut libc::c_char, 0i32);
        *result_u32 = {
          let mut __v: libc::c_uint = 0;
          let mut __x: libc::c_uint = tmp_1 as libc::c_uint;
          if false {
            __v = (__x & 0xff000000u32) >> 24i32
              | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
              | (__x & 0xff00i32 as libc::c_uint) << 8i32
              | (__x & 0xffi32 as libc::c_uint) << 24i32
          } else {
            let fresh7 = &mut __v;
            let fresh8;
            let fresh9 = __x;
            asm!("bswap $0" : "=r" (fresh8) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh7, fresh9)) :);
            c2rust_asm_casts::AsmCast::cast_out(fresh7, fresh9, fresh8);
          }
          __v
        };
        retval = (*bb_errno == 0i32) as libc::c_int;
        current_block_58 = 10778260831612459202;
      }
      10 => {
        /* Input: "a.b.c.d/m" */
        /* Output: mask(1 byte),pfx(0-4 bytes),gw(4 bytes) */
        let mut mask: libc::c_uint = 0;
        let mut slash: *mut libc::c_char = strchr(val, '/' as i32);
        if !slash.is_null() {
          *slash = '\u{0}' as i32 as libc::c_char;
          retval = udhcp_str2nip(val, buffer.as_mut_ptr().offset(1) as *mut libc::c_void);
          mask = crate::libbb::bb_strtonum::bb_strtou(
            slash.offset(1),
            0 as *mut *mut libc::c_char,
            10i32,
          );
          buffer[0] = mask as libc::c_char;
          val = strtok(
            std::ptr::null_mut::<libc::c_char>(),
            b", \t/-\x00" as *const u8 as *const libc::c_char,
          );
          if val.is_null() || mask > 32i32 as libc::c_uint || *bb_errno != 0 {
            retval = 0i32
          }
          if retval != 0 {
            length = (mask.wrapping_add(7i32 as libc::c_uint) >> 3i32)
              .wrapping_add(5i32 as libc::c_uint) as libc::c_int;
            retval = udhcp_str2nip(
              val,
              buffer.as_mut_ptr().offset((length - 4i32) as isize) as *mut libc::c_void,
            )
          }
        }
        current_block_58 = 10778260831612459202;
      }
      9 => {
        /* Raw (numeric) option code. Is it a string? */
        if *val.offset(0) as libc::c_int == '\"' as i32
          || *val.offset(0) as libc::c_int == '\'' as i32
        {
          let mut delim: libc::c_char = *val.offset(0);
          let mut end: *mut libc::c_char =
            crate::libbb::last_char_is::last_char_is(val.offset(1), delim as libc::c_int);
          if !end.is_null() {
            *end = '\u{0}' as i32 as libc::c_char;
            val = val.offset(1);
            userdef_optflag.flags = OPTION_STRING as libc::c_int as u8;
            current_block_58 = 17106587497971209820;
          } else {
            current_block_58 = 919954187481050311;
          }
        } else {
          current_block_58 = 919954187481050311;
        }
        match current_block_58 {
          17106587497971209820 => {}
          _ => {
            /* No: hex-str option, handled in attach_option() */
            opt = val;
            retval = 1i32;
            current_block_58 = 10778260831612459202;
          }
        }
      }
      _ => {
        current_block_58 = 10778260831612459202;
      }
    }
    match current_block_58 {
      17106587497971209820 => {
        length = strnlen(val, 254i32 as size_t) as libc::c_int;
        if length > 0i32 {
          opt = val;
          retval = 1i32
        }
      }
      _ => {}
    }
    if retval != 0 {
      attach_option(opt_list, optflag, opt, length, dhcpv6);
    }
    if !(retval != 0 && (*optflag).flags as libc::c_int & OPTION_LIST as libc::c_int != 0) {
      break;
    }
  }
  return retval;
}

/*
 * Russ Dill <Russ.Dill@asu.edu> September 2001
 * Rewritten by Vladimir Oleynik <dzo@simtreas.ru> (C) 2003
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/* six all-ones */
/* ** DHCP packet ***/
/* DHCP protocol. See RFC 2131 */
//TODO: rename ciaddr/yiaddr/chaddr
/* BOOTREQUEST or BOOTREPLY */
/* hardware address type. 1 = 10mb ethernet */
/* hardware address length */
/* used by relay agents only */
/* unique id */
/* elapsed since client began acquisition/renewal */
/* only one flag so far: */
/* "I need broadcast replies" */
/* client IP (if client is in BOUND, RENEW or REBINDING state) */
/* 'your' (client) IP address */
/* IP address of next server to use in bootstrap, returned in DHCPOFFER, DHCPACK by server */
/* aka 'giaddr': relay agent IP address */
/* link-layer client hardware address (MAC) */
/* server host name (ASCIZ) */
/* boot file name (ASCIZ) */
/* fixed first four option bytes (99,130,83,99 dec) */
/* Let's see whether compiler understood us right */
/* ** Options ***/
/* Opts of STRING_HOST type will be sanitized before they are passed
 * to udhcpc script's environment: */
//	OPTION_BOOLEAN,
//	OPTION_S16,
/* RFC1035 compressed domain name list */
/* Client requests this option by default */
/* There can be a list of 1 or more of these */
/* DHCP option codes (partial list). See RFC 2132 and
 * http://www.iana.org/assignments/bootp-dhcp-parameters/
 * Commented out options are handled by common option machinery,
 * uncommented ones have special cases (grep for them to see).
 */
//#define DHCP_TIME_OFFSET      0x02 /* (localtime - UTC_time) in seconds. signed */
//#define DHCP_ROUTER           0x03
//#define DHCP_TIME_SERVER      0x04 /* RFC 868 time server (32-bit, 0 = 1.1.1900) */
//#define DHCP_NAME_SERVER      0x05 /* IEN 116 _really_ ancient kind of NS */
//#define DHCP_DNS_SERVER       0x06
//#define DHCP_LOG_SERVER       0x07 /* port 704 UDP log (not syslog) */
//#define DHCP_COOKIE_SERVER    0x08 /* "quote of the day" server */
//#define DHCP_LPR_SERVER       0x09
/* 12: either client informs server or server gives name to client */
//#define DHCP_BOOT_SIZE        0x0d
//#define DHCP_DOMAIN_NAME      0x0f /* 15: server gives domain suffix */
//#define DHCP_SWAP_SERVER      0x10
//#define DHCP_ROOT_PATH        0x11
//#define DHCP_IP_TTL           0x17
//#define DHCP_MTU              0x1a
//#define DHCP_BROADCAST        0x1c
//#define DHCP_ROUTES           0x21
//#define DHCP_NIS_DOMAIN       0x28
//#define DHCP_NIS_SERVER       0x29
//#define DHCP_NTP_SERVER       0x2a
//#define DHCP_WINS_SERVER      0x2c
/* 50: sent by client if specific IP is wanted */
/* 51: */
/* 52: */
/* 53: */
/* 54: server's IP */
/* 55: list of options client wants */
//#define DHCP_ERR_MESSAGE      0x38 /* 56: error message when sending NAK etc */
/* 57: */
/* 60: client's vendor (a string) */
/* 61: by default client's MAC addr, but may be arbitrarily long */
//#define DHCP_TFTP_SERVER_NAME 0x42 /* 66: same as 'sname' field */
//#define DHCP_BOOT_FILE        0x43 /* 67: same as 'file' field */
//#define DHCP_USER_CLASS       0x4d /* 77: RFC 3004. set of LASCII strings. "I am a printer" etc */
/* 81: client asks to update DNS to map its FQDN to its new IP */
//#define DHCP_PCODE            0x64 /* 100: RFC 4833. IEEE 1003.1 TZ string */
//#define DHCP_TCODE            0x65 /* 101: RFC 4833. Reference to the TZ database string */
//#define DHCP_DOMAIN_SEARCH    0x77 /* 119: RFC 3397. set of ASCIZ string, DNS-style compressed */
//#define DHCP_SIP_SERVERS      0x78 /* 120: RFC 3361. flag byte, then: 0: domain names, 1: IP addrs */
//#define DHCP_STATIC_ROUTES    0x79 /* 121: RFC 3442. (mask,ip,router) tuples */
//#define DHCP_VLAN_ID          0x84 /* 132: 802.1P VLAN ID */
//#define DHCP_VLAN_PRIORITY    0x85 /* 133: 802.1Q VLAN priority */
//#define DHCP_PXE_CONF_FILE    0xd1 /* 209: RFC 5071 Configuration file */
//#define DHCP_PXE_PATH_PREFIX  0xd2 /* 210: RFC 5071 Path prefix */
//#define DHCP_REBOOT_TIME      0xd3 /* 211: RFC 5071 Reboot time */
//#define DHCP_MS_STATIC_ROUTES 0xf9 /* 249: Microsoft's pre-RFC 3442 code for 0x79? */
//#define DHCP_WPAD             0xfc /* 252: MSIE's Web Proxy Autodiscovery Protocol */
/* 255: */
/* Offsets in option byte sequence */
/* Offsets in option byte sequence for DHCPv6 */
/* Bits in "overload" option */
/* DHCP_MESSAGE_TYPE values */
/* client -> server */
/* client <- server */
/* client -> server */
/* client -> server */
/* client <- server */
/* client <- server */
/* client -> server */
/* client -> server */
/* Same as above + ensures that option length is 4 bytes
 * (returns NULL if size is different)
 */
// RFC 2131  Table 5: Fields and options used by DHCP clients
//
// Fields 'hops', 'yiaddr', 'siaddr', 'giaddr' are always zero, 'chaddr' is always client's MAC
//
// Field      DHCPDISCOVER          DHCPINFORM            DHCPREQUEST           DHCPDECLINE         DHCPRELEASE
// -----      ------------          ------------          -----------           -----------         -----------
// 'xid'      selected by client    selected by client    'xid' from server     selected by client  selected by client
//                                                        DHCPOFFER message
// 'secs'     0 or seconds since    0 or seconds since    0 or seconds since    0                   0
//            DHCP process started  DHCP process started  DHCP process started
// 'flags'    Set 'BROADCAST'       Set 'BROADCAST'       Set 'BROADCAST'       0                   0
//            flag if client needs  flag if client needs  flag if client needs
//            broadcast reply       broadcast reply       broadcast reply
// 'ciaddr'   0                     client's IP           0 or client's IP      0                   client's IP
//                                                        (BOUND/RENEW/REBIND)
// 'sname'    options or sname      options or sname      options or sname      (unused)            (unused)
// 'file'     options or file       options or file       options or file       (unused)            (unused)
// 'options'  options               options               options               message type opt    message type opt
//
// Option                     DHCPDISCOVER  DHCPINFORM  DHCPREQUEST             DHCPDECLINE  DHCPRELEASE
// ------                     ------------  ----------  -----------             -----------  -----------
// Requested IP address       MAY           MUST NOT    MUST (in SELECTING      MUST         MUST NOT
//                                                      or INIT-REBOOT)
//                                                      MUST NOT (in BOUND
//                                                      or RENEWING)
// IP address lease time      MAY           MUST NOT    MAY                     MUST NOT     MUST NOT
// Use 'file'/'sname' fields  MAY           MAY         MAY                     MAY          MAY
// Client identifier          MAY           MAY         MAY                     MAY          MAY
// Vendor class identifier    MAY           MAY         MAY                     MUST NOT     MUST NOT
// Server identifier          MUST NOT      MUST NOT    MUST (after SELECTING)  MUST         MUST
//                                                      MUST NOT (after
//                                                      INIT-REBOOT, BOUND,
//                                                      RENEWING or REBINDING)
// Parameter request list     MAY           MAY         MAY                     MUST NOT     MUST NOT
// Maximum message size       MAY           MAY         MAY                     MUST NOT     MUST NOT
// Message                    SHOULD NOT    SHOULD NOT  SHOULD NOT              SHOULD       SHOULD
// Site-specific              MAY           MAY         MAY                     MUST NOT     MUST NOT
// All others                 MAY           MAY         MAY                     MUST NOT     MUST NOT
/* ** Logging ***/
/* ** Other shared functions ***/
/* 2nd param is "u32*" */
/* 2nd param is "struct option_set**" */
/*u32 ip,*/
/* Returns 1 if no reply received */
/* note: ip is a pointer to an IPv6 in network order, possibly misaliged */
/* note: ip is a pointer to an IPv6 in network order, possibly misaliged */
#[no_mangle]
pub unsafe extern "C" fn sprint_nip6(
  mut dest: *mut libc::c_char,
  mut ip: *const u8,
) -> libc::c_int {
  let mut hexstrbuf: [libc::c_char; 32] = [0; 32];
  crate::libbb::xfuncs::bin2hex(
    hexstrbuf.as_mut_ptr(),
    ip as *mut libc::c_void as *const libc::c_char,
    16i32,
  );
  return sprintf(
    dest,
    b"%.4s:%.4s:%.4s:%.4s:%.4s:%.4s:%.4s:%.4s\x00" as *const u8 as *const libc::c_char,
    hexstrbuf.as_mut_ptr().offset((0i32 * 4i32) as isize),
    hexstrbuf.as_mut_ptr().offset((1i32 * 4i32) as isize),
    hexstrbuf.as_mut_ptr().offset((2i32 * 4i32) as isize),
    hexstrbuf.as_mut_ptr().offset((3i32 * 4i32) as isize),
    hexstrbuf.as_mut_ptr().offset((4i32 * 4i32) as isize),
    hexstrbuf.as_mut_ptr().offset((5i32 * 4i32) as isize),
    hexstrbuf.as_mut_ptr().offset((6i32 * 4i32) as isize),
    hexstrbuf.as_mut_ptr().offset((7i32 * 4i32) as isize),
  );
}
