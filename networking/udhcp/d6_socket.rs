use c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use libc;
extern "C" {
  #[no_mangle]
  fn rand() -> libc::c_int;
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
  #[no_mangle]
  fn xsocket(domain: libc::c_int, type_0: libc::c_int, protocol: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xbind(sockfd: libc::c_int, my_addr: *mut sockaddr, addrlen: socklen_t);
  /* SO_REUSEADDR allows a server to rebind to an address that is already
   * "in use" by old connections to e.g. previous server instance which is
   * killed or crashed. Without it bind will fail until all such connections
   * time out. Linux does not allow multiple live binds on same ip:port
   * regardless of SO_REUSEADDR (unlike some other flavors of Unix).
   * Turn it on before you call bind(). */
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
  fn bb_error_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_info_msg(s: *const libc::c_char, _: ...);
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
  #[no_mangle]
  static mut dhcp_verbose: libc::c_uint;
  #[no_mangle]
  fn getifaddrs(__ifap: *mut *mut ifaddrs) -> libc::c_int;
  #[no_mangle]
  fn freeifaddrs(__ifa: *mut ifaddrs);
}
use crate::librb::__uint8_t;
pub type __uint16_t = libc::c_ushort;
use crate::librb::__uint32_t;
pub type __caddr_t = *mut libc::c_char;
pub type __socklen_t = libc::c_uint;
use crate::librb::uint8_t;
pub type uint16_t = __uint16_t;
use crate::librb::uint32_t;
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
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
  pub sa_family: sa_family_t,
  pub sa_data: [libc::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
  pub sin6_family: sa_family_t,
  pub sin6_port: in_port_t,
  pub sin6_flowinfo: uint32_t,
  pub sin6_addr: in6_addr,
  pub sin6_scope_id: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
  pub __in6_u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
  pub __u6_addr8: [uint8_t; 16],
  pub __u6_addr16: [uint16_t; 8],
  pub __u6_addr32: [uint32_t; 4],
}
pub type in_port_t = uint16_t;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const IPPROTO_MAX: C2RustUnnamed_0 = 256;
pub const IPPROTO_RAW: C2RustUnnamed_0 = 255;
pub const IPPROTO_MPLS: C2RustUnnamed_0 = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed_0 = 136;
pub const IPPROTO_SCTP: C2RustUnnamed_0 = 132;
pub const IPPROTO_COMP: C2RustUnnamed_0 = 108;
pub const IPPROTO_PIM: C2RustUnnamed_0 = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed_0 = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed_0 = 94;
pub const IPPROTO_MTP: C2RustUnnamed_0 = 92;
pub const IPPROTO_AH: C2RustUnnamed_0 = 51;
pub const IPPROTO_ESP: C2RustUnnamed_0 = 50;
pub const IPPROTO_GRE: C2RustUnnamed_0 = 47;
pub const IPPROTO_RSVP: C2RustUnnamed_0 = 46;
pub const IPPROTO_IPV6: C2RustUnnamed_0 = 41;
pub const IPPROTO_DCCP: C2RustUnnamed_0 = 33;
pub const IPPROTO_TP: C2RustUnnamed_0 = 29;
pub const IPPROTO_IDP: C2RustUnnamed_0 = 22;
pub const IPPROTO_UDP: C2RustUnnamed_0 = 17;
pub const IPPROTO_PUP: C2RustUnnamed_0 = 12;
pub const IPPROTO_EGP: C2RustUnnamed_0 = 8;
pub const IPPROTO_TCP: C2RustUnnamed_0 = 6;
pub const IPPROTO_IPIP: C2RustUnnamed_0 = 4;
pub const IPPROTO_IGMP: C2RustUnnamed_0 = 2;
pub const IPPROTO_ICMP: C2RustUnnamed_0 = 1;
pub const IPPROTO_IP: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
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
  pub ifr_ifrn: C2RustUnnamed_2,
  pub ifr_ifru: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
  pub ifrn_name: [libc::c_char; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifaddrs {
  pub ifa_next: *mut ifaddrs,
  pub ifa_name: *mut libc::c_char,
  pub ifa_flags: libc::c_uint,
  pub ifa_addr: *mut sockaddr,
  pub ifa_netmask: *mut sockaddr,
  pub ifa_ifu: C2RustUnnamed_3,
  pub ifa_data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
  pub ifu_broadaddr: *mut sockaddr,
  pub ifu_dstaddr: *mut sockaddr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_ll {
  pub sll_family: libc::c_ushort,
  pub sll_protocol: libc::c_ushort,
  pub sll_ifindex: libc::c_int,
  pub sll_hatype: libc::c_ushort,
  pub sll_pkttype: libc::c_uchar,
  pub sll_halen: libc::c_uchar,
  pub sll_addr: [libc::c_uchar; 8],
}

/*
 * Copyright (C) 2011 Denys Vlasenko.
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
#[no_mangle]
pub unsafe extern "C" fn d6_read_interface(
  mut interface: *const libc::c_char,
  mut ifindex: *mut libc::c_int,
  mut nip6: *mut in6_addr,
  mut mac: *mut uint8_t,
) -> libc::c_int {
  let mut retval: libc::c_int = 3i32;
  let mut ifap: *mut ifaddrs = 0 as *mut ifaddrs;
  let mut ifa: *mut ifaddrs = 0 as *mut ifaddrs;
  getifaddrs(&mut ifap);
  ifa = ifap;
  while !ifa.is_null() {
    let mut sip6: *mut sockaddr_in6 = 0 as *mut sockaddr_in6;
    if !((*ifa).ifa_addr.is_null() || strcmp((*ifa).ifa_name, interface) != 0i32) {
      if (*(*ifa).ifa_addr).sa_family as libc::c_int == 17i32 {
        let mut sll: *mut sockaddr_ll = (*ifa).ifa_addr as *mut libc::c_void as *mut sockaddr_ll;
        memcpy(
          mac as *mut libc::c_void,
          (*sll).sll_addr.as_mut_ptr() as *const libc::c_void,
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
        *ifindex = (*sll).sll_ifindex;
        if dhcp_verbose >= 2i32 as libc::c_uint {
          bb_info_msg(
            b"ifindex %d\x00" as *const u8 as *const libc::c_char,
            *ifindex,
          );
        }
        retval &= 3i32 - (1i32 << 0i32)
      }
      /* RFC 3315
       * 16. Client Source Address and Interface Selection
       *
       * "When a client sends a DHCP message to the
       * All_DHCP_Relay_Agents_and_Servers address, ... ... The client
       * MUST use a link-local address assigned to the interface for which it
       * is requesting configuration information as the source address in the
       * header of the IP datagram."
       */
      sip6 = (*ifa).ifa_addr as *mut libc::c_void as *mut sockaddr_in6; /* struct copy */
      if (*(*ifa).ifa_addr).sa_family as libc::c_int == 10i32
        && ({
          let mut __a: *const in6_addr = &mut (*sip6).sin6_addr as *mut in6_addr as *const in6_addr;
          ((*__a).__in6_u.__u6_addr32[0]
            & ({
              let mut __v: libc::c_uint = 0;
              let mut __x: libc::c_uint = 0xffc00000u32;
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
            })
            == ({
              let mut __v: libc::c_uint = 0;
              let mut __x: libc::c_uint = 0xfe800000u32;
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
            })) as libc::c_int
        }) != 0
      {
        *nip6 = (*sip6).sin6_addr;
        if dhcp_verbose >= 1i32 as libc::c_uint {
          bb_info_msg(
            b"IPv6 %02x%02x:%02x%02x:%02x%02x:%02x%02x:%02x%02x:%02x%02x:%02x%02x:%02x%02x\x00"
              as *const u8 as *const libc::c_char,
            (*nip6).__in6_u.__u6_addr8[0] as libc::c_int,
            (*nip6).__in6_u.__u6_addr8[1] as libc::c_int,
            (*nip6).__in6_u.__u6_addr8[2] as libc::c_int,
            (*nip6).__in6_u.__u6_addr8[3] as libc::c_int,
            (*nip6).__in6_u.__u6_addr8[4] as libc::c_int,
            (*nip6).__in6_u.__u6_addr8[5] as libc::c_int,
            (*nip6).__in6_u.__u6_addr8[6] as libc::c_int,
            (*nip6).__in6_u.__u6_addr8[7] as libc::c_int,
            (*nip6).__in6_u.__u6_addr8[8] as libc::c_int,
            (*nip6).__in6_u.__u6_addr8[9] as libc::c_int,
            (*nip6).__in6_u.__u6_addr8[10] as libc::c_int,
            (*nip6).__in6_u.__u6_addr8[11] as libc::c_int,
            (*nip6).__in6_u.__u6_addr8[12] as libc::c_int,
            (*nip6).__in6_u.__u6_addr8[13] as libc::c_int,
            (*nip6).__in6_u.__u6_addr8[14] as libc::c_int,
            (*nip6).__in6_u.__u6_addr8[15] as libc::c_int,
          );
        }
        retval &= 3i32 - (1i32 << 1i32)
      }
    }
    ifa = (*ifa).ifa_next
  }
  freeifaddrs(ifap);
  if retval & 1i32 << 0i32 != 0 {
    /* This iface has no MAC (e.g. ppp), generate a random one */
    let mut ifr: ifreq = ifreq {
      ifr_ifrn: C2RustUnnamed_2 { ifrn_name: [0; 16] },
      ifr_ifru: C2RustUnnamed_1 {
        ifru_addr: sockaddr {
          sa_family: 0,
          sa_data: [0; 14],
        },
      },
    };
    let mut fd: libc::c_int = 0;
    /*memset(&ifr, 0, sizeof(ifr)); - SIOCGIFINDEX does not need to clear all */
    strncpy_IFNAMSIZ(ifr.ifr_ifrn.ifrn_name.as_mut_ptr(), interface);
    fd = xsocket(10i32, SOCK_RAW as libc::c_int, IPPROTO_RAW as libc::c_int);
    if ioctl(fd, 0x8933i32 as libc::c_ulong, &mut ifr as *mut ifreq) == 0i32 {
      *ifindex = ifr.ifr_ifru.ifru_ivalue;
      if dhcp_verbose >= 2i32 as libc::c_uint {
        bb_info_msg(
          b"ifindex %d\x00" as *const u8 as *const libc::c_char,
          *ifindex,
        );
      }
      if *(mac as *mut uint32_t).offset(0) == 0i32 as libc::c_uint {
        /* invent a fictitious MAC (once) */
        *(mac as *mut uint32_t).offset(0) = rand() as uint32_t;
        *(mac as *mut uint16_t).offset(2) = rand() as uint16_t;
        let ref mut fresh6 = *mac.offset(0);
        *fresh6 = (*fresh6 as libc::c_int & 0xfci32) as uint8_t
        /* make sure it's not bcast */
      }
      retval &= 3i32 - (1i32 << 0i32)
    }
    close(fd);
  }
  if retval == 0i32 {
    return retval;
  }
  if retval & 1i32 << 0i32 != 0 {
    bb_error_msg(
      b"can\'t get %s\x00" as *const u8 as *const libc::c_char,
      b"MAC\x00" as *const u8 as *const libc::c_char,
    );
  }
  if retval & 1i32 << 1i32 != 0 {
    bb_error_msg(
      b"can\'t get %s\x00" as *const u8 as *const libc::c_char,
      b"link-local IPv6 address\x00" as *const u8 as *const libc::c_char,
    );
  }
  return retval;
}
#[no_mangle]
pub unsafe extern "C" fn d6_listen_socket(
  mut port: libc::c_int,
  mut inf: *const libc::c_char,
) -> libc::c_int {
  let mut fd: libc::c_int = 0;
  let mut addr: sockaddr_in6 = sockaddr_in6 {
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
  if dhcp_verbose >= 1i32 as libc::c_uint {
    bb_info_msg(
      b"opening listen socket on *:%d %s\x00" as *const u8 as *const libc::c_char,
      port,
      inf,
    );
  }
  fd = xsocket(10i32, SOCK_DGRAM as libc::c_int, IPPROTO_UDP as libc::c_int);
  setsockopt_reuseaddr(fd);
  if setsockopt_broadcast(fd) == -1i32 {
    bb_simple_perror_msg_and_die(b"SO_BROADCAST\x00" as *const u8 as *const libc::c_char);
  }
  /* NB: bug 1032 says this doesn't work on ethernet aliases (ethN:M) */
  if setsockopt_bindtodevice(fd, inf) != 0 {
    xfunc_die(); /* warning is already printed */
  }
  memset(
    &mut addr as *mut sockaddr_in6 as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<sockaddr_in6>() as libc::c_ulong,
  );
  addr.sin6_family = 10i32 as sa_family_t;
  addr.sin6_port = {
    let mut __v: libc::c_ushort = 0;
    let mut __x: libc::c_ushort = port as libc::c_ushort;
    if 0 != 0 {
      __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
        as libc::c_ushort
    } else {
      let fresh7 = &mut __v;
      let fresh8;
      let fresh9 = __x;
      asm!("rorw $$8, ${0:w}" : "=r" (fresh8) : "0"
                      (c2rust_asm_casts::AsmCast::cast_in(fresh7, fresh9)) :
                      "cc");
      c2rust_asm_casts::AsmCast::cast_out(fresh7, fresh9, fresh8);
    }
    __v
  };
  /* addr.sin6_addr is all-zeros */
  xbind(
    fd,
    &mut addr as *mut sockaddr_in6 as *mut sockaddr,
    ::std::mem::size_of::<sockaddr_in6>() as libc::c_ulong as socklen_t,
  );
  return fd;
}
