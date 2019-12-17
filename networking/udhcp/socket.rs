use c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;

use libc;
use libc::close;
use libc::strrchr;
extern "C" {

  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  fn inet_ntoa(__in: in_addr) -> *mut libc::c_char;
  #[no_mangle]
  fn xsocket(domain: libc::c_int, type_0: libc::c_int, protocol: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xbind(sockfd: libc::c_int, my_addr: *mut sockaddr, addrlen: socklen_t);
  #[no_mangle]
  fn setsockopt_reuseaddr(fd: libc::c_int);
  #[no_mangle]
  fn setsockopt_broadcast(fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn setsockopt_bindtodevice(fd: libc::c_int, iface: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn strncpy_IFNAMSIZ(dst: *mut libc::c_char, src: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xfunc_die() -> !;
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_info_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn ioctl_or_perror(
    fd: libc::c_int,
    request: libc::c_uint,
    argp: *mut libc::c_void,
    fmt: *const libc::c_char,
    _: ...
  ) -> libc::c_int;
  #[no_mangle]
  fn bb_ioctl_or_warn(
    fd: libc::c_int,
    request: libc::c_uint,
    argp: *mut libc::c_void,
    ioctl_name: *const libc::c_char,
  ) -> libc::c_int;
  #[no_mangle]
  static mut dhcp_verbose: libc::c_uint;
}

pub type __caddr_t = *mut libc::c_char;
pub type __socklen_t = libc::c_uint;

pub type socklen_t = __socklen_t;
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
pub type C2RustUnnamed = libc::c_uint;
pub const IPPROTO_MAX: C2RustUnnamed = 256;
pub const IPPROTO_RAW: C2RustUnnamed = 255;
pub const IPPROTO_MPLS: C2RustUnnamed = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed = 136;
pub const IPPROTO_SCTP: C2RustUnnamed = 132;
pub const IPPROTO_COMP: C2RustUnnamed = 108;
pub const IPPROTO_PIM: C2RustUnnamed = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed = 94;
pub const IPPROTO_MTP: C2RustUnnamed = 92;
pub const IPPROTO_AH: C2RustUnnamed = 51;
pub const IPPROTO_ESP: C2RustUnnamed = 50;
pub const IPPROTO_GRE: C2RustUnnamed = 47;
pub const IPPROTO_RSVP: C2RustUnnamed = 46;
pub const IPPROTO_IPV6: C2RustUnnamed = 41;
pub const IPPROTO_DCCP: C2RustUnnamed = 33;
pub const IPPROTO_TP: C2RustUnnamed = 29;
pub const IPPROTO_IDP: C2RustUnnamed = 22;
pub const IPPROTO_UDP: C2RustUnnamed = 17;
pub const IPPROTO_PUP: C2RustUnnamed = 12;
pub const IPPROTO_EGP: C2RustUnnamed = 8;
pub const IPPROTO_TCP: C2RustUnnamed = 6;
pub const IPPROTO_IPIP: C2RustUnnamed = 4;
pub const IPPROTO_IGMP: C2RustUnnamed = 2;
pub const IPPROTO_ICMP: C2RustUnnamed = 1;
pub const IPPROTO_IP: C2RustUnnamed = 0;
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
pub struct ifmap {
  pub mem_start: libc::c_ulong,
  pub mem_end: libc::c_ulong,
  pub base_addr: libc::c_ushort,
  pub irq: libc::c_uchar,
  pub dma: libc::c_uchar,
  pub port: libc::c_uchar,
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

/*
 * DHCP server client/server socket creation
 *
 * udhcp client/server
 * Copyright (C) 1999 Matthew Ramsay <matthewr@moreton.com.au>
 *			Chris Trew <ctrew@moreton.com.au>
 *
 * Rewrite by Russ Dill <Russ.Dill@asu.edu> July 2001
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 675 Mass Ave, Cambridge, MA 02139, USA.
 */
#[no_mangle]
pub unsafe extern "C" fn udhcp_read_interface(
  mut interface: *const libc::c_char,
  mut ifindex: *mut libc::c_int,
  mut nip: *mut u32,
  mut mac: *mut u8,
) -> libc::c_int {
  /* char buffer instead of bona-fide struct avoids aliasing warning */
  let mut ifr_buf: [libc::c_char; 40] = [0; 40];
  let ifr: *mut ifreq = ifr_buf.as_mut_ptr() as *mut libc::c_void as *mut ifreq;
  let mut fd: libc::c_int = 0;
  let mut our_ip: *mut sockaddr_in = std::ptr::null_mut();
  memset(
    ifr as *mut libc::c_void,
    0,
    ::std::mem::size_of::<ifreq>() as libc::c_ulong,
  );
  fd = xsocket(2i32, SOCK_RAW as libc::c_int, IPPROTO_RAW as libc::c_int);
  (*ifr).ifr_ifru.ifru_addr.sa_family = 2i32 as sa_family_t;
  strncpy_IFNAMSIZ((*ifr).ifr_ifrn.ifrn_name.as_mut_ptr(), interface);
  if !nip.is_null() {
    if ioctl_or_perror(
      fd,
      0x8915i32 as libc::c_uint,
      ifr as *mut libc::c_void,
      b"is interface %s up and configured?\x00" as *const u8 as *const libc::c_char,
      interface,
    ) != 0
    {
      close(fd);
      return -1i32;
    }
    our_ip = &mut (*ifr).ifr_ifru.ifru_addr as *mut sockaddr as *mut sockaddr_in;
    *nip = (*our_ip).sin_addr.s_addr;
    if dhcp_verbose >= 1i32 as libc::c_uint {
      bb_info_msg(
        b"IP %s\x00" as *const u8 as *const libc::c_char,
        inet_ntoa((*our_ip).sin_addr),
      );
    }
  }
  if !ifindex.is_null() {
    if bb_ioctl_or_warn(
      fd,
      0x8933i32 as libc::c_uint,
      ifr as *mut libc::c_void,
      b"SIOCGIFINDEX\x00" as *const u8 as *const libc::c_char,
    ) != 0
    {
      close(fd);
      return -1i32;
    }
    if dhcp_verbose >= 2i32 as libc::c_uint {
      bb_info_msg(
        b"ifindex %d\x00" as *const u8 as *const libc::c_char,
        (*ifr).ifr_ifru.ifru_ivalue,
      );
    }
    *ifindex = (*ifr).ifr_ifru.ifru_ivalue
  }
  if !mac.is_null() {
    if bb_ioctl_or_warn(
      fd,
      0x8927i32 as libc::c_uint,
      ifr as *mut libc::c_void,
      b"SIOCGIFHWADDR\x00" as *const u8 as *const libc::c_char,
    ) != 0
    {
      close(fd);
      return -1i32;
    }
    memcpy(
      mac as *mut libc::c_void,
      (*ifr).ifr_ifru.ifru_hwaddr.sa_data.as_mut_ptr() as *const libc::c_void,
      6i32 as libc::c_ulong,
    );
    if dhcp_verbose >= 2i32 as libc::c_uint {
      bb_info_msg(
        b"MAC %02x:%02x:%02x:%02x:%02x:%02x\x00" as *const u8 as *const libc::c_char,
        *mac.offset(0) as libc::c_int,
        *mac.offset(1) as libc::c_int,
        *mac.offset(2) as libc::c_int,
        *mac.offset(3) as libc::c_int,
        *mac.offset(4) as libc::c_int,
        *mac.offset(5) as libc::c_int,
      );
    }
  }
  close(fd);
  return 0;
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
/* 1. None of the callers expects it to ever fail */
/* 2. ip was always INADDR_ANY */
#[no_mangle]
pub unsafe extern "C" fn udhcp_listen_socket(
  mut port: libc::c_int,
  mut inf: *const libc::c_char,
) -> libc::c_int {
  let mut fd: libc::c_int = 0;
  let mut addr: sockaddr_in = sockaddr_in {
    sin_family: 0,
    sin_port: 0,
    sin_addr: in_addr { s_addr: 0 },
    sin_zero: [0; 8],
  };
  let mut colon: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  if dhcp_verbose >= 1i32 as libc::c_uint {
    bb_info_msg(
      b"opening listen socket on *:%d %s\x00" as *const u8 as *const libc::c_char,
      port,
      inf,
    );
  }
  fd = xsocket(2i32, SOCK_DGRAM as libc::c_int, IPPROTO_UDP as libc::c_int);
  setsockopt_reuseaddr(fd);
  if setsockopt_broadcast(fd) == -1i32 {
    bb_simple_perror_msg_and_die(b"SO_BROADCAST\x00" as *const u8 as *const libc::c_char);
  }
  /* SO_BINDTODEVICE doesn't work on ethernet aliases (ethN:M) */
  colon = strrchr(inf, ':' as i32); /* warning is already printed */
  if !colon.is_null() {
    *colon = '\u{0}' as i32 as libc::c_char
  }
  if setsockopt_bindtodevice(fd, inf) != 0 {
    xfunc_die();
  }
  if !colon.is_null() {
    *colon = ':' as i32 as libc::c_char
  }
  memset(
    &mut addr as *mut sockaddr_in as *mut libc::c_void,
    0,
    ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong,
  );
  addr.sin_family = 2i32 as sa_family_t;
  addr.sin_port = {
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
                      (c2rust_asm_casts::AsmCast::cast_in(fresh0, fresh2)) :
                      "cc");
      c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
    }
    __v
  };
  /* addr.sin_addr.s_addr = ip; - all-zeros is INADDR_ANY */
  xbind(
    fd,
    &mut addr as *mut sockaddr_in as *mut sockaddr,
    ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
  );
  return fd;
}
