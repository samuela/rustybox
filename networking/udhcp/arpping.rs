use c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use libc;
use libc::open;

use libc::close;

extern "C" {
  pub type sockaddr_x25;
  pub type sockaddr_un;
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
  fn sendto(
    __fd: libc::c_int,
    __buf: *const libc::c_void,
    __n: size_t,
    __flags: libc::c_int,
    __addr: __CONST_SOCKADDR_ARG,
    __addr_len: socklen_t,
  ) -> ssize_t;

  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
  #[no_mangle]
  fn monotonic_ms() -> libc::c_ulonglong;
  #[no_mangle]
  fn setsockopt_broadcast(fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn safe_strncpy(
    dst: *mut libc::c_char,
    src: *const libc::c_char,
    size: size_t,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;
  /* Wrapper which restarts poll on EINTR or ENOMEM.
   * On other errors complains [perror("poll")] and returns.
   * Warning! May take (much) longer than timeout_ms to return!
   * If this is a problem, use bare poll and open-code EINTR/ENOMEM handling */
  #[no_mangle]
  fn safe_poll(ufds: *mut pollfd, nfds: nfds_t, timeout_ms: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);
  #[no_mangle]
  fn bb_info_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  static bb_msg_can_not_create_raw_socket: [libc::c_char; 0];
  #[no_mangle]
  static mut dhcp_verbose: libc::c_uint;
}

pub type __socklen_t = libc::c_uint;
use crate::librb::size_t;
use libc::ssize_t;



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
pub type nfds_t = libc::c_ulong;
use libc::pollfd;

/*
 * Mostly stolen from: dhcpcd - DHCP client daemon
 * by Yoichi Hariguchi <yoichi@fore.com>
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct arpMsg {
  pub h_dest: [u8; 6],
  pub h_source: [u8; 6],
  pub h_proto: u16,
  pub htype: u16,
  pub ptype: u16,
  pub hlen: u8,
  pub plen: u8,
  pub operation: u16,
  pub sHaddr: [u8; 6],
  pub sInaddr: [u8; 4],
  pub tHaddr: [u8; 6],
  pub tInaddr: [u8; 4],
  pub pad: [u8; 18],
  /* 2a pad for min. ethernet payload (60 bytes) */
}
pub type aliased_u32 = u32;
pub const ARP_MSG_SIZE: C2RustUnnamed_0 = 42;
pub type C2RustUnnamed_0 = libc::c_uint;

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
/* Returns 1 if no reply received */
#[no_mangle]
pub unsafe extern "C" fn arpping(
  mut test_nip: u32,
  mut safe_mac: *const u8,
  mut from_ip: u32,
  mut from_mac: *mut u8,
  mut interface: *const libc::c_char,
  mut timeo: libc::c_uint,
) -> libc::c_int {
  let mut timeout_ms: libc::c_int = 0;
  let mut pfd: [pollfd; 1] = [pollfd {
    fd: 0,
    events: 0,
    revents: 0,
  }; 1];
  /* socket */
  let mut rv: libc::c_int = 1i32; /* "no reply received" yet */
  let mut addr: sockaddr = sockaddr {
    sa_family: 0,
    sa_data: [0; 14],
  }; /* for interface name */
  let mut arp: arpMsg = arpMsg {
    h_dest: [0; 6],
    h_source: [0; 6],
    h_proto: 0,
    htype: 0,
    ptype: 0,
    hlen: 0,
    plen: 0,
    operation: 0,
    sHaddr: [0; 6],
    sInaddr: [0; 4],
    tHaddr: [0; 6],
    tInaddr: [0; 4],
    pad: [0; 18],
  };
  if timeo == 0 {
    return 1i32;
  }
  pfd[0].fd = socket(
    17i32,
    SOCK_PACKET as libc::c_int,
    ({
      let mut __v: libc::c_ushort = 0;
      let mut __x: libc::c_ushort = 0x806i32 as libc::c_ushort;
      if 0 != 0 {
        __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
          as libc::c_ushort
      } else {
        let fresh0 = &mut __v;
        let fresh1;
        let fresh2 = __x;
        asm!("rorw $$8, ${0:w}" : "=r" (fresh1) : "0"
                             (c2rust_asm_casts::AsmCast::cast_in(fresh0, fresh2))
                             : "cc");
        c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
      }
      __v
    }) as libc::c_int,
  );
  if pfd[0].fd == -1i32 {
    bb_simple_perror_msg(bb_msg_can_not_create_raw_socket.as_ptr());
    return -1i32;
  }
  if setsockopt_broadcast(pfd[0].fd) == -1i32 {
    bb_simple_perror_msg(
      b"can\'t enable bcast on raw socket\x00" as *const u8 as *const libc::c_char,
    );
  } else {
    /* send arp request */
    memset(
      &mut arp as *mut arpMsg as *mut libc::c_void,
      0i32,
      ::std::mem::size_of::<arpMsg>() as libc::c_ulong,
    ); /* MAC DA */
    memset(
      arp.h_dest.as_mut_ptr() as *mut libc::c_void,
      0xffi32,
      6i32 as libc::c_ulong,
    ); /* MAC SA */
    memcpy(
      arp.h_source.as_mut_ptr() as *mut libc::c_void,
      from_mac as *const libc::c_void,
      6i32 as libc::c_ulong,
    ); /* protocol type (Ethernet) */
    arp.h_proto = {
      let mut __v: libc::c_ushort = 0; /* hardware type */
      let mut __x: libc::c_ushort = 0x806i32 as libc::c_ushort; /* protocol type (ARP message) */
      if 0 != 0 {
        __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
          as libc::c_ushort
      } else {
        let fresh3 = &mut __v; /* hardware address length */
        let fresh4; /* protocol address length */
        let fresh5 = __x; /* ARP op code */
        asm!("rorw $$8, ${0:w}" : "=r" (fresh4) : "0"
                          (c2rust_asm_casts::AsmCast::cast_in(fresh3, fresh5))
                          : "cc"); /* source hardware address */
        c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh5, fresh4); /* source IP address */
      }
      __v
    };
    arp.htype = {
      let mut __v: libc::c_ushort = 0;
      let mut __x: libc::c_ushort = 1i32 as libc::c_ushort;
      if 0 != 0 {
        __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
          as libc::c_ushort
      } else {
        let fresh6 = &mut __v;
        let fresh7;
        let fresh8 = __x;
        asm!("rorw $$8, ${0:w}" : "=r" (fresh7) : "0"
                          (c2rust_asm_casts::AsmCast::cast_in(fresh6, fresh8))
                          : "cc");
        c2rust_asm_casts::AsmCast::cast_out(fresh6, fresh8, fresh7);
      }
      __v
    };
    arp.ptype = {
      let mut __v: libc::c_ushort = 0;
      let mut __x: libc::c_ushort = 0x800i32 as libc::c_ushort;
      if 0 != 0 {
        __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
          as libc::c_ushort
      } else {
        let fresh9 = &mut __v;
        let fresh10;
        let fresh11 = __x;
        asm!("rorw $$8, ${0:w}" : "=r" (fresh10) : "0"
                          (c2rust_asm_casts::AsmCast::cast_in(fresh9, fresh11))
                          : "cc");
        c2rust_asm_casts::AsmCast::cast_out(fresh9, fresh11, fresh10);
      }
      __v
    };
    arp.hlen = 6i32 as u8;
    arp.plen = 4i32 as u8;
    arp.operation = {
      let mut __v: libc::c_ushort = 0;
      let mut __x: libc::c_ushort = 1i32 as libc::c_ushort;
      if 0 != 0 {
        __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
          as libc::c_ushort
      } else {
        let fresh12 = &mut __v;
        let fresh13;
        let fresh14 = __x;
        asm!("rorw $$8, ${0:w}" : "=r" (fresh13) : "0"
                          (c2rust_asm_casts::AsmCast::cast_in(fresh12, fresh14))
                          : "cc");
        c2rust_asm_casts::AsmCast::cast_out(fresh12, fresh14, fresh13);
      }
      __v
    };
    memcpy(
      arp.sHaddr.as_mut_ptr() as *mut libc::c_void,
      from_mac as *const libc::c_void,
      6i32 as libc::c_ulong,
    );
    memcpy(
      arp.sInaddr.as_mut_ptr() as *mut libc::c_void,
      &mut from_ip as *mut u32 as *const libc::c_void,
      ::std::mem::size_of::<u32>() as libc::c_ulong,
    );
    /* tHaddr is zero-filled */
    /* target hardware address */
    memcpy(
      arp.tInaddr.as_mut_ptr() as *mut libc::c_void,
      &mut test_nip as *mut u32 as *const libc::c_void,
      ::std::mem::size_of::<u32>() as libc::c_ulong,
    ); /* target IP address */
    memset(
      &mut addr as *mut sockaddr as *mut libc::c_void,
      0i32,
      ::std::mem::size_of::<sockaddr>() as libc::c_ulong,
    );
    safe_strncpy(
      addr.sa_data.as_mut_ptr(),
      interface,
      ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong,
    );
    if !(sendto(
      pfd[0].fd,
      &mut arp as *mut arpMsg as *const libc::c_void,
      ::std::mem::size_of::<arpMsg>() as libc::c_ulong,
      0i32,
      __CONST_SOCKADDR_ARG {
        __sockaddr__: &mut addr,
      },
      ::std::mem::size_of::<sockaddr>() as libc::c_ulong as socklen_t,
    ) <0)
    {
      /* wait for arp reply, and check it */
      timeout_ms = timeo as libc::c_int;
      loop {
        let mut r: libc::c_int = 0;
        let mut prevTime: libc::c_uint = monotonic_ms() as libc::c_uint;
        pfd[0].events = 0x1i32 as libc::c_short;
        r = safe_poll(pfd.as_mut_ptr(), 1i32 as nfds_t, timeout_ms);
        if r < 0i32 {
          break;
        }
        if r != 0 {
          r = safe_read(
            pfd[0].fd,
            &mut arp as *mut arpMsg as *mut libc::c_void,
            ::std::mem::size_of::<arpMsg>() as libc::c_ulong,
          ) as libc::c_int;
          if r < 0i32 {
            break;
          }
          //log3("sHaddr %02x:%02x:%02x:%02x:%02x:%02x",
          //	arp.sHaddr[0], arp.sHaddr[1], arp.sHaddr[2],
          //	arp.sHaddr[3], arp.sHaddr[4], arp.sHaddr[5]);
          if r >= ARP_MSG_SIZE as libc::c_int
            && arp.operation as libc::c_int
              == ({
                let mut __v: libc::c_ushort = 0;
                let mut __x: libc::c_ushort = 2i32 as libc::c_ushort;
                if 0 != 0 {
                  __v = (__x as libc::c_int >> 8i32 & 0xffi32
                    | (__x as libc::c_int & 0xffi32) << 8i32)
                    as libc::c_ushort
                } else {
                  let fresh15 = &mut __v;
                  let fresh16;
                  let fresh17 = __x;
                  asm!("rorw $$8, ${0:w}" : "=r"
                                             (fresh16) : "0"
                                             (c2rust_asm_casts::AsmCast::cast_in(fresh15, fresh17))
                                             : "cc");
                  c2rust_asm_casts::AsmCast::cast_out(fresh15, fresh17, fresh16);
                }
                __v
              }) as libc::c_int
            && *(arp.sInaddr.as_mut_ptr() as *mut aliased_u32) == test_nip
          {
            /* if ARP source MAC matches safe_mac
             * (which is client's MAC), then it's not a conflict
             * (client simply already has this IP and replies to ARPs!)
             */
            if safe_mac.is_null()
              || memcmp(
                safe_mac as *const libc::c_void,
                arp.sHaddr.as_mut_ptr() as *const libc::c_void,
                6i32 as libc::c_ulong,
              ) != 0i32
            {
              rv = 0i32
            }
            break;
          }
        }
        timeout_ms = (timeout_ms as libc::c_uint).wrapping_sub(
          (monotonic_ms() as libc::c_uint)
            .wrapping_sub(prevTime)
            .wrapping_add(1i32 as libc::c_uint),
        ) as libc::c_int as libc::c_int;
        if !(timeout_ms as libc::c_uint <= timeo) {
          break;
        }
        /* We used to check "timeout_ms > 0", but
         * this is more under/overflow-resistant
         * (people did see overflows here when system time jumps):
         */
      }
    }
  }
  // TODO: error message? caller didn't expect us to fail,
  // just returning 1 "no reply received" misleads it.
  close(pfd[0].fd);
  if dhcp_verbose >= 1i32 as libc::c_uint {
    bb_info_msg(
      b"%srp reply received for this address\x00" as *const u8 as *const libc::c_char,
      if rv != 0 {
        b"no a\x00" as *const u8 as *const libc::c_char
      } else {
        b"A\x00" as *const u8 as *const libc::c_char
      },
    );
  }
  return rv;
}
