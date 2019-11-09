use libc;
use libc::free;
extern "C" {

  #[no_mangle]
  fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
  #[no_mangle]
  fn mempcpy(
    __dest: *mut libc::c_void,
    __src: *const libc::c_void,
    __n: size_t,
  ) -> *mut libc::c_void;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn stpcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xmalloc(size: size_t) -> *mut libc::c_void;
}

use crate::librb::size_t;

/* name compression pointer flag */
/* Expand a RFC1035-compressed list of domain names "cstr", of length "clen";
 * returns a newly allocated string containing the space-separated domains,
 * prefixed with the contents of string pre, or NULL if an error occurs.
 */
#[no_mangle]
pub unsafe extern "C" fn dname_dec(
  mut cstr: *const u8,
  mut clen: libc::c_int,
  mut pre: *const libc::c_char,
) -> *mut libc::c_char {
  let mut ret: *mut libc::c_char = 0 as *mut libc::c_char; /* for compiler */
  ret = ret;
  let mut dst: *mut libc::c_char = 0 as *mut libc::c_char;
  /* We make two passes over the cstr string. First, we compute
   * how long the resulting string would be. Then we allocate a
   * new buffer of the required length, and fill it in with the
   * expanded content. The advantage of this approach is not
   * having to deal with requiring callers to supply their own
   * buffer, then having to check if it's sufficiently large, etc.
   */
  loop
  /* note: "return NULL" below are leak-safe since
   * dst isn't allocated yet */
  {
    let mut c: *const u8 = 0 as *const u8;
    let mut crtpos: libc::c_uint = 0;
    let mut retpos: libc::c_uint = 0;
    let mut depth: libc::c_uint = 0;
    let mut len: libc::c_uint = 0;
    len = 0i32 as libc::c_uint;
    depth = len;
    retpos = depth;
    crtpos = retpos;
    while crtpos < clen as libc::c_uint {
      c = cstr.offset(crtpos as isize);
      if *c as libc::c_int & 0xc0i32 == 0xc0i32 {
        /* pointer */
        if crtpos.wrapping_add(2i32 as libc::c_uint) > clen as libc::c_uint {
          /* no offset to jump to? abort */
          return 0 as *mut libc::c_char;
        }
        if retpos == 0i32 as libc::c_uint {
          /* jump */
          /* toplevel? save return spot */
          retpos = crtpos.wrapping_add(2i32 as libc::c_uint)
        }
        depth = depth.wrapping_add(1);
        crtpos = ((*c.offset(0) as libc::c_int & 0x3fi32) << 8i32 | *c.offset(1) as libc::c_int)
          as libc::c_uint
      } else if *c != 0 {
        /* label */
        if crtpos
          .wrapping_add(*c as libc::c_uint)
          .wrapping_add(1i32 as libc::c_uint)
          > clen as libc::c_uint
        {
          /* label too long? abort */
          return 0 as *mut libc::c_char;
        }
        if !dst.is_null() {
          /* \3com ---> "com." */
          *(mempcpy(
            dst.offset(len as isize) as *mut libc::c_void,
            c.offset(1) as *const libc::c_void,
            *c as size_t,
          ) as *mut libc::c_char)
            .offset(0) = '.' as i32 as libc::c_char
        }
        len = len.wrapping_add((*c as libc::c_int + 1i32) as libc::c_uint);
        crtpos = crtpos.wrapping_add((*c as libc::c_int + 1i32) as libc::c_uint)
      } else {
        /* NUL: end of current domain name */
        if retpos == 0i32 as libc::c_uint {
          /* toplevel? keep going */
          crtpos = crtpos.wrapping_add(1)
        } else {
          /* return to toplevel saved spot */
          crtpos = retpos;
          depth = 0i32 as libc::c_uint;
          retpos = depth
        }
        if !dst.is_null() && len != 0i32 as libc::c_uint {
          /* \4host\3com\0\4host and we are at \0:
           * \3com was converted to "com.", change dot to space.
           */
          *dst.offset(len.wrapping_sub(1i32 as libc::c_uint) as isize) = ' ' as i32 as libc::c_char
        }
      }
      if depth > 6i32 as libc::c_uint || len > (1025i32 * 6i32) as libc::c_uint {
        /* result too long? abort */
        return 0 as *mut libc::c_char;
      }
    }
    if len == 0 {
      /* expanded string has 0 length? abort */
      return 0 as *mut libc::c_char;
    }
    if dst.is_null() {
      /* first pass? */
      /* allocate dst buffer and copy pre */
      let mut plen: libc::c_uint = strlen(pre) as libc::c_uint;
      ret = xmalloc(plen.wrapping_add(len) as size_t) as *mut libc::c_char;
      dst = stpcpy(ret, pre)
    } else {
      *dst.offset(len.wrapping_sub(1i32 as libc::c_uint) as isize) = '\u{0}' as i32 as libc::c_char;
      break;
    }
  }
  return ret;
}
/* Convert a domain name (src) from human-readable "foo.blah.com" format into
 * RFC1035 encoding "\003foo\004blah\003com\000". Return allocated string, or
 * NULL if an error occurs.
 */
unsafe extern "C" fn convert_dname(mut src: *const libc::c_char) -> *mut u8 {
  let mut c: u8 = 0;
  let mut res: *mut u8 = 0 as *mut u8;
  let mut lenptr: *mut u8 = 0 as *mut u8;
  let mut dst: *mut u8 = 0 as *mut u8;
  let mut len: libc::c_int = 0;
  res = xmalloc(strlen(src).wrapping_add(2i32 as libc::c_ulong)) as *mut u8;
  lenptr = res;
  dst = lenptr;
  dst = dst.offset(1);
  loop {
    let fresh0 = src;
    src = src.offset(1);
    c = *fresh0 as u8;
    if c as libc::c_int == '.' as i32 || c as libc::c_int == '\u{0}' as i32 {
      /* end of label */
      len =
        (dst.wrapping_offset_from(lenptr) as libc::c_long - 1) as libc::c_int;
      /* label too long, too short, or two '.'s in a row? abort */
      if len > 63i32
        || len == 0i32
        || c as libc::c_int == '.' as i32 && *src as libc::c_int == '.' as i32
      {
        free(res as *mut libc::c_void);
        return 0 as *mut u8;
      }
      *lenptr = len as u8;
      if c as libc::c_int == '\u{0}' as i32 || *src as libc::c_int == '\u{0}' as i32 {
        break;
      }
      let fresh1 = dst;
      dst = dst.offset(1);
      lenptr = fresh1
    } else {
      if c as libc::c_int >= 'A' as i32 && c as libc::c_int <= 'Z' as i32 {
        /* uppercase? convert to lower */
        c = (c as libc::c_int + ('a' as i32 - 'A' as i32)) as u8
      }
      let fresh2 = dst;
      dst = dst.offset(1);
      *fresh2 = c
    }
  }
  if dst.wrapping_offset_from(res) as libc::c_long >= 255i32 as libc::c_long {
    /* dname too long? abort */
    free(res as *mut libc::c_void);
    return 0 as *mut u8;
  }
  *dst = 0i32 as u8;
  return res;
}
/* Returns the offset within cstr at which dname can be found, or -1 */
unsafe extern "C" fn find_offset(
  mut cstr: *const u8,
  mut clen: libc::c_int,
  mut dname: *const u8,
) -> libc::c_int {
  let mut c: *const u8 = 0 as *const u8;
  let mut d: *const u8 = 0 as *const u8;
  let mut off: libc::c_int = 0;
  /* find all labels in cstr */
  off = 0i32;
  while off < clen {
    c = cstr.offset(off as isize);
    if *c as libc::c_int & 0xc0i32 == 0xc0i32 {
      /* pointer, skip */
      off += 2i32
    } else if *c != 0 {
      /* label, try matching dname */
      d = dname;
      loop {
        let mut len1: libc::c_uint = (*c as libc::c_int + 1i32) as libc::c_uint;
        if memcmp(
          c as *const libc::c_void,
          d as *const libc::c_void,
          len1 as libc::c_ulong,
        ) != 0i32
        {
          break;
        }
        if len1 == 1i32 as libc::c_uint {
          /* at terminating NUL - match, return offset */
          return off;
        }
        d = d.offset(len1 as isize);
        c = c.offset(len1 as isize);
        if *c as libc::c_int & 0xc0i32 == 0xc0i32 {
          /* pointer, jump */
          c = cstr.offset(
            ((*c.offset(0) as libc::c_int & 0x3fi32) << 8i32 | *c.offset(1) as libc::c_int)
              as isize,
          )
        }
      }
      off += *cstr.offset(off as isize) as libc::c_int + 1i32
    } else {
      /* NUL, skip */
      off += 1
    }
  }
  return -1i32;
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
/* Computes string to be appended to cstr so that src would be added to
 * the compression (best case, it's a 2-byte pointer to some offset within
 * cstr; worst case, it's all of src, converted to <4>host<3>com<0> format).
 * The computed string is returned directly; its length is returned via retlen;
 * NULL and 0, respectively, are returned if an error occurs.
 */
#[no_mangle]
pub unsafe extern "C" fn dname_enc(
  mut cstr: *const u8,
  mut clen: libc::c_int,
  mut src: *const libc::c_char,
  mut retlen: *mut libc::c_int,
) -> *mut u8 {
  let mut d: *mut u8 = 0 as *mut u8;
  let mut dname: *mut u8 = 0 as *mut u8;
  let mut off: libc::c_int = 0;
  dname = convert_dname(src);
  if dname.is_null() {
    *retlen = 0i32;
    return 0 as *mut u8;
  }
  d = dname;
  while *d != 0 {
    if !cstr.is_null() {
      off = find_offset(cstr, clen, d);
      if off >= 0i32 {
        /* found a match, add pointer and return */
        let fresh3 = d;
        d = d.offset(1);
        *fresh3 = (0xc0i32 | off >> 8i32) as u8;
        *d = off as u8;
        break;
      }
    }
    d = d.offset((*d as libc::c_int + 1i32) as isize)
  }
  *retlen = (d.wrapping_offset_from(dname) as libc::c_long + 1) as libc::c_int;
  return dname;
}
