use c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use libc;
use libc::access;
use libc::atoi;
use libc::fclose;
use libc::fprintf;
use libc::lstat;
use libc::printf;
use libc::puts;
use libc::rename;
use libc::rmdir;
use libc::sprintf;
use libc::strchr;
use libc::strcmp;
use libc::strrchr;
use libc::strstr;
use libc::system;


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
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  fn socket(__domain: libc::c_int, __type: libc::c_int, __protocol: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn bind(__fd: libc::c_int, __addr: __CONST_SOCKADDR_ARG, __len: socklen_t) -> libc::c_int;
  #[no_mangle]
  fn connect(__fd: libc::c_int, __addr: __CONST_SOCKADDR_ARG, __len: socklen_t) -> libc::c_int;
  #[no_mangle]
  fn sendto(
    __fd: libc::c_int,
    __buf: *const libc::c_void,
    __n: size_t,
    __flags: libc::c_int,
    __addr: __CONST_SOCKADDR_ARG,
    __addr_len: socklen_t,
  ) -> ssize_t;
  /* SO_REUSEADDR allows a server to rebind to an address that is already
   * "in use" by old connections to e.g. previous server instance which is
   * killed or crashed. Without it bind will fail until all such connections
   * time out. Linux does not allow multiple live binds on same ip:port
   * regardless of SO_REUSEADDR (unlike some other flavors of Unix).
   * Turn it on before you call bind(). */
  #[no_mangle]
  fn setsockopt_reuseaddr(fd: libc::c_int);
  #[no_mangle]
  fn inet_cksum(addr: *mut u16, len: libc::c_int) -> u16;
  #[no_mangle]
  fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn safe_write(fd: libc::c_int, buf: *const libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn bb_perror_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_info_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_simple_info_msg(s: *const libc::c_char);
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
pub struct udphdr {
  pub c2rust_unnamed: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
  pub c2rust_unnamed: C2RustUnnamed_3,
  pub c2rust_unnamed_0: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
  pub source: u16,
  pub dest: u16,
  pub len: u16,
  pub check: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
  pub uh_sport: u16,
  pub uh_dport: u16,
  pub uh_ulen: u16,
  pub uh_sum: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ip6_hdr {
  pub ip6_ctlun: C2RustUnnamed_4,
  pub ip6_src: in6_addr,
  pub ip6_dst: in6_addr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
  pub ip6_un1: ip6_hdrctl,
  pub ip6_un2_vfc: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ip6_hdrctl {
  pub ip6_un1_flow: u32,
  pub ip6_un1_plen: u16,
  pub ip6_un1_nxt: u8,
  pub ip6_un1_hlim: u8,
}

/*
 * Copyright (C) 2011 Denys Vlasenko.
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
/* ** DHCPv6 packet ***/
/* DHCPv6 protocol. See RFC 3315 */
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct d6_packet {
  pub d6_u: C2RustUnnamed_5,
  pub d6_options: [u8; 604],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
  pub d6_msg_type: u8,
  pub d6_xid32: u32,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ip6_udp_d6_packet {
  pub ip6: ip6_hdr,
  pub udp: udphdr,
  pub data: d6_packet,
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
pub unsafe extern "C" fn d6_dump_packet(mut packet: *mut d6_packet) {
  if dhcp_verbose < 2i32 as libc::c_uint {
    return;
  }
  bb_info_msg(
    b" xid %x\x00" as *const u8 as *const libc::c_char,
    (*packet).d6_u.d6_xid32,
  );
  //*bin2hex(buf, (void *) packet->chaddr, sizeof(packet->chaddr)) = '\0';
  //bb_error_msg(" chaddr %s", buf);
}
#[no_mangle]
pub unsafe extern "C" fn d6_recv_kernel_packet(
  mut _peer_ipv6: *mut in6_addr,
  mut packet: *mut d6_packet,
  mut fd: libc::c_int,
) -> libc::c_int {
  let mut bytes: libc::c_int = 0;
  memset(
    packet as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<d6_packet>() as libc::c_ulong,
  );
  bytes = safe_read(
    fd,
    packet as *mut libc::c_void,
    ::std::mem::size_of::<d6_packet>() as libc::c_ulong,
  ) as libc::c_int;
  if bytes < 0i32 {
    if dhcp_verbose >= 1i32 as libc::c_uint {
      bb_simple_info_msg(b"packet read error, ignoring\x00" as *const u8 as *const libc::c_char);
    }
    return bytes;
    /* returns -1 */
  }
  if (bytes as libc::c_ulong) < 4u64 {
    bb_simple_info_msg(b"packet with bad magic, ignoring\x00" as *const u8 as *const libc::c_char);
    return -2i32;
  }
  if dhcp_verbose >= 1i32 as libc::c_uint {
    bb_info_msg(
      b"received %s\x00" as *const u8 as *const libc::c_char,
      b"a packet\x00" as *const u8 as *const libc::c_char,
    );
  }
  d6_dump_packet(packet);
  return bytes;
}
/* Construct a ipv6+udp header for a packet, send packet */
#[no_mangle]
pub unsafe extern "C" fn d6_send_raw_packet(
  mut d6_pkt: *mut d6_packet,
  mut d6_pkt_size: libc::c_uint,
  mut src_ipv6: *mut in6_addr,
  mut source_port: libc::c_int,
  mut dst_ipv6: *mut in6_addr,
  mut dest_port: libc::c_int,
  mut dest_arp: *const u8,
  mut ifindex: libc::c_int,
) -> libc::c_int {
  let mut current_block: u64; /* struct copy */
  let mut dest_sll: sockaddr_ll = sockaddr_ll {
    sll_family: 0,
    sll_protocol: 0,
    sll_ifindex: 0,
    sll_hatype: 0,
    sll_pkttype: 0,
    sll_halen: 0,
    sll_addr: [0; 8],
  };
  let mut packet: ip6_udp_d6_packet = ip6_udp_d6_packet {
    ip6: ip6_hdr {
      ip6_ctlun: C2RustUnnamed_4 {
        ip6_un1: ip6_hdrctl {
          ip6_un1_flow: 0,
          ip6_un1_plen: 0,
          ip6_un1_nxt: 0,
          ip6_un1_hlim: 0,
        },
      },
      ip6_src: in6_addr {
        __in6_u: C2RustUnnamed {
          __u6_addr8: [0; 16],
        },
      },
      ip6_dst: in6_addr {
        __in6_u: C2RustUnnamed {
          __u6_addr8: [0; 16],
        },
      },
    },
    udp: udphdr {
      c2rust_unnamed: C2RustUnnamed_1 {
        c2rust_unnamed: C2RustUnnamed_3 {
          uh_sport: 0,
          uh_dport: 0,
          uh_ulen: 0,
          uh_sum: 0,
        },
      },
    },
    data: d6_packet {
      d6_u: C2RustUnnamed_5 { d6_msg_type: 0 },
      d6_options: [0; 604],
    },
  };
  let mut fd: libc::c_int = 0;
  let mut result: libc::c_int = -1i32;
  let mut msg: *const libc::c_char = 0 as *const libc::c_char;
  fd = socket(
    17i32,
    SOCK_DGRAM as libc::c_int,
    ({
      let mut __v: libc::c_ushort = 0;
      let mut __x: libc::c_ushort = 0x86ddi32 as libc::c_ushort;
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
  if fd < 0i32 {
    msg = b"socket(%s)\x00" as *const u8 as *const libc::c_char;
    current_block = 6439760897239081377;
  } else {
    memset(
      &mut dest_sll as *mut sockaddr_ll as *mut libc::c_void,
      0i32,
      ::std::mem::size_of::<sockaddr_ll>() as libc::c_ulong,
    );
    memset(
      &mut packet as *mut ip6_udp_d6_packet as *mut libc::c_void,
      0i32,
      48u64,
    );
    packet.data = *d6_pkt;
    dest_sll.sll_family = 17i32 as libc::c_ushort;
    dest_sll.sll_protocol = {
      let mut __v: libc::c_ushort = 0;
      let mut __x: libc::c_ushort = 0x86ddi32 as libc::c_ushort;
      if 0 != 0 {
        __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
          as libc::c_ushort
      } else {
        let fresh3 = &mut __v;
        let fresh4;
        let fresh5 = __x;
        asm!("rorw $$8, ${0:w}" : "=r" (fresh4) : "0"
                          (c2rust_asm_casts::AsmCast::cast_in(fresh3, fresh5))
                          : "cc");
        c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh5, fresh4);
      }
      __v
    };
    dest_sll.sll_ifindex = ifindex;
    /*dest_sll.sll_hatype = ARPHRD_???;*/
    /*dest_sll.sll_pkttype = PACKET_???;*/
    dest_sll.sll_halen = 6i32 as libc::c_uchar; /* 4 bits version, top 4 bits of tclass */
    memcpy(
      dest_sll.sll_addr.as_mut_ptr() as *mut libc::c_void,
      dest_arp as *const libc::c_void,
      6i32 as libc::c_ulong,
    ); /* struct copy */
    if bind(
      fd,
      __CONST_SOCKADDR_ARG {
        __sockaddr__: &mut dest_sll as *mut sockaddr_ll as *mut sockaddr,
      },
      ::std::mem::size_of::<sockaddr_ll>() as libc::c_ulong as socklen_t,
    ) < 0i32
    {
      msg = b"bind(%s)\x00" as *const u8 as *const libc::c_char
    } else {
      packet.ip6.ip6_ctlun.ip6_un2_vfc = (6i32 << 4i32) as u8; /* struct copy */
      if !src_ipv6.is_null() {
        packet.ip6.ip6_src = *src_ipv6
      }
      packet.ip6.ip6_dst = *dst_ipv6;
      packet.udp.c2rust_unnamed.c2rust_unnamed_0.source = {
        let mut __v: libc::c_ushort = 0;
        let mut __x: libc::c_ushort = source_port as libc::c_ushort;
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
      packet.udp.c2rust_unnamed.c2rust_unnamed_0.dest = {
        let mut __v: libc::c_ushort = 0;
        let mut __x: libc::c_ushort = dest_port as libc::c_ushort;
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
      /* size, excluding IP header: */
      packet.udp.c2rust_unnamed.c2rust_unnamed_0.len = {
        let mut __v: libc::c_ushort = 0;
        let mut __x: libc::c_ushort = (::std::mem::size_of::<udphdr>() as libc::c_ulong)
          .wrapping_add(d6_pkt_size as libc::c_ulong)
          as libc::c_ushort;
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
      packet.ip6.ip6_ctlun.ip6_un1.ip6_un1_plen = packet.udp.c2rust_unnamed.c2rust_unnamed_0.len;
      /*
       * Someone was smoking weed (at least) while inventing UDP checksumming:
       * UDP checksum skips first four bytes of IPv6 header.
       * 'next header' field should be summed as if it is one more byte
       * to the right, therefore we write its value (IPPROTO_UDP)
       * into ip6_hlim, and its 'real' location remains zero-filled for now.
       */
      packet.ip6.ip6_ctlun.ip6_un1.ip6_un1_hlim = IPPROTO_UDP as libc::c_int as u8;
      packet.udp.c2rust_unnamed.c2rust_unnamed_0.check = inet_cksum(
        (&mut packet as *mut ip6_udp_d6_packet as *mut u16).offset(2),
        48u64
          .wrapping_sub(4i32 as libc::c_ulong)
          .wrapping_add(d6_pkt_size as libc::c_ulong) as libc::c_int,
      );
      /* fix 'hop limit' and 'next header' after UDP checksumming */
      packet.ip6.ip6_ctlun.ip6_un1.ip6_un1_hlim = 1i32 as u8; /* observed Windows machines to use hlim=1 */
      packet.ip6.ip6_ctlun.ip6_un1.ip6_un1_nxt = IPPROTO_UDP as libc::c_int as u8;
      d6_dump_packet(d6_pkt);
      result = sendto(
        fd,
        &mut packet as *mut ip6_udp_d6_packet as *const libc::c_void,
        48u64.wrapping_add(d6_pkt_size as libc::c_ulong),
        0i32,
        __CONST_SOCKADDR_ARG {
          __sockaddr__: &mut dest_sll as *mut sockaddr_ll as *mut sockaddr,
        },
        ::std::mem::size_of::<sockaddr_ll>() as libc::c_ulong as socklen_t,
      ) as libc::c_int;
      msg = b"sendto\x00" as *const u8 as *const libc::c_char
    }
    close(fd);
    if result < 0i32 {
      current_block = 6439760897239081377;
    } else {
      current_block = 17281240262373992796;
    }
  }
  match current_block {
    6439760897239081377 => {
      bb_perror_msg(msg, b"PACKET\x00" as *const u8 as *const libc::c_char);
    }
    _ => {}
  }
  return result;
}
/* Let the kernel do all the work for packet generation */
#[no_mangle]
pub unsafe extern "C" fn d6_send_kernel_packet(
  mut d6_pkt: *mut d6_packet,
  mut d6_pkt_size: libc::c_uint,
  mut src_ipv6: *mut in6_addr,
  mut source_port: libc::c_int,
  mut dst_ipv6: *mut in6_addr,
  mut dest_port: libc::c_int,
  mut ifindex: libc::c_int,
) -> libc::c_int {
  let mut current_block: u64; /* struct copy */
  let mut sa: sockaddr_in6 = sockaddr_in6 {
    sin6_family: 0,
    sin6_port: 0,
    sin6_flowinfo: 0,
    sin6_addr: in6_addr {
      __in6_u: C2RustUnnamed {
        __u6_addr8: [0; 16],
      },
    },
    sin6_scope_id: 0,
  }; /* struct copy */
  let mut fd: libc::c_int = 0;
  let mut result: libc::c_int = -1i32;
  let mut msg: *const libc::c_char = 0 as *const libc::c_char;
  fd = socket(10i32, SOCK_DGRAM as libc::c_int, IPPROTO_UDP as libc::c_int);
  if fd < 0i32 {
    msg = b"socket(%s)\x00" as *const u8 as *const libc::c_char;
    current_block = 9009192957559530970;
  } else {
    setsockopt_reuseaddr(fd);
    memset(
      &mut sa as *mut sockaddr_in6 as *mut libc::c_void,
      0i32,
      ::std::mem::size_of::<sockaddr_in6>() as libc::c_ulong,
    );
    sa.sin6_family = 10i32 as sa_family_t;
    sa.sin6_port = {
      let mut __v: libc::c_ushort = 0;
      let mut __x: libc::c_ushort = source_port as libc::c_ushort;
      if 0 != 0 {
        __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
          as libc::c_ushort
      } else {
        let fresh15 = &mut __v;
        let fresh16;
        let fresh17 = __x;
        asm!("rorw $$8, ${0:w}" : "=r" (fresh16) : "0"
                          (c2rust_asm_casts::AsmCast::cast_in(fresh15, fresh17))
                          : "cc");
        c2rust_asm_casts::AsmCast::cast_out(fresh15, fresh17, fresh16);
      }
      __v
    };
    sa.sin6_addr = *src_ipv6;
    if bind(
      fd,
      __CONST_SOCKADDR_ARG {
        __sockaddr__: &mut sa as *mut sockaddr_in6 as *mut sockaddr,
      },
      ::std::mem::size_of::<sockaddr_in6>() as libc::c_ulong as socklen_t,
    ) == -1i32
    {
      msg = b"bind(%s)\x00" as *const u8 as *const libc::c_char
    } else {
      memset(
        &mut sa as *mut sockaddr_in6 as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<sockaddr_in6>() as libc::c_ulong,
      );
      sa.sin6_family = 10i32 as sa_family_t;
      sa.sin6_port = {
        let mut __v: libc::c_ushort = 0;
        let mut __x: libc::c_ushort = dest_port as libc::c_ushort;
        if 0 != 0 {
          __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
            as libc::c_ushort
        } else {
          let fresh18 = &mut __v;
          let fresh19;
          let fresh20 = __x;
          asm!("rorw $$8, ${0:w}" : "=r" (fresh19) : "0"
                              (c2rust_asm_casts::AsmCast::cast_in(fresh18, fresh20))
                              : "cc");
          c2rust_asm_casts::AsmCast::cast_out(fresh18, fresh20, fresh19);
        }
        __v
      };
      sa.sin6_addr = *dst_ipv6;
      sa.sin6_scope_id = ifindex as u32;
      if connect(
        fd,
        __CONST_SOCKADDR_ARG {
          __sockaddr__: &mut sa as *mut sockaddr_in6 as *mut sockaddr,
        },
        ::std::mem::size_of::<sockaddr_in6>() as libc::c_ulong as socklen_t,
      ) == -1i32
      {
        msg = b"connect\x00" as *const u8 as *const libc::c_char
      } else {
        d6_dump_packet(d6_pkt);
        result =
          safe_write(fd, d6_pkt as *const libc::c_void, d6_pkt_size as size_t) as libc::c_int;
        msg = b"write\x00" as *const u8 as *const libc::c_char
      }
    }
    close(fd);
    if result < 0i32 {
      current_block = 9009192957559530970;
    } else {
      current_block = 9828876828309294594;
    }
  }
  match current_block {
    9009192957559530970 => {
      bb_perror_msg(msg, b"UDP\x00" as *const u8 as *const libc::c_char);
    }
    _ => {}
  }
  return result;
}
