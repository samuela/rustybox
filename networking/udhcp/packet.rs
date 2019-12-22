use crate::librb::size_t;
use crate::librb::socklen_t;
use c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use c2rust_bitfields;
use c2rust_bitfields::BitfieldStruct;
use libc;
use libc::close;
use libc::in_addr;
use libc::sa_family_t;
use libc::sockaddr;
use libc::sockaddr_in;
use libc::sockaddr_in6;
use libc::ssize_t;
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
  #[no_mangle]
  static mut dhcp_verbose: libc::c_uint;

}

pub type __socklen_t = libc::c_uint;
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct udphdr {
  pub c2rust_unnamed: C2RustUnnamed_1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_1 {
  pub c2rust_unnamed: C2RustUnnamed_3,
  pub c2rust_unnamed_0: C2RustUnnamed_2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_2 {
  pub source: u16,
  pub dest: u16,
  pub len: u16,
  pub check: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_3 {
  pub uh_sport: u16,
  pub uh_dport: u16,
  pub uh_ulen: u16,
  pub uh_sum: u16,
}

#[repr(C)]
#[derive(Copy, Clone, BitfieldStruct)]
pub struct iphdr {
  #[bitfield(name = "ihl", ty = "libc::c_uint", bits = "0..=3")]
  #[bitfield(name = "version", ty = "libc::c_uint", bits = "4..=7")]
  pub ihl_version: [u8; 1],
  pub tos: u8,
  pub tot_len: u16,
  pub id: u16,
  pub frag_off: u16,
  pub ttl: u8,
  pub protocol: u8,
  pub check: u16,
  pub saddr: u32,
  pub daddr: u32,
}

use crate::networking::udhcp::common::dhcp_packet;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ip_udp_dhcp_packet {
  pub ip: iphdr,
  pub udp: udphdr,
  pub data: dhcp_packet,
}
pub type C2RustUnnamed_4 = libc::c_uint;
pub const DHCP_SIZE: C2RustUnnamed_4 = 548;
pub const UDP_DHCP_SIZE: C2RustUnnamed_4 = 556;
pub const IP_UDP_DHCP_SIZE: C2RustUnnamed_4 = 576;

#[repr(C)]
#[derive(Copy, Clone)]
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
 * Packet ops
 *
 * Rewrite by Russ Dill <Russ.Dill@asu.edu> July 2001
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
#[no_mangle]
pub unsafe extern "C" fn udhcp_init_header(mut packet: *mut dhcp_packet, mut type_0: libc::c_char) {
  memset(
    packet as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<dhcp_packet>() as libc::c_ulong,
  ); /* if client to a server */
  (*packet).op = 1i32 as u8;
  match type_0 as libc::c_int {
    2 | 5 | 6 => {
      (*packet).op = 2i32 as u8
      /* if server to client */
    }
    _ => {}
  } /* ethernet */
  (*packet).htype = 1i32 as u8;
  (*packet).hlen = 6i32 as u8;
  (*packet).cookie = {
    let mut __v: libc::c_uint = 0;
    let mut __x: libc::c_uint = 0x63825363i32 as libc::c_uint;
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
  };
  if 0xffi32 != 0i32 {
    (*packet).options[0] = 0xffi32 as u8
  }
  crate::networking::udhcp::common::udhcp_add_simple_option(packet, 0x35i32 as u8, type_0 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn udhcp_dump_packet(mut packet: *mut dhcp_packet) {
  let mut buf: [libc::c_char; 33] = [0; 33];
  if dhcp_verbose < 2i32 as libc::c_uint {
    return;
  }
  crate::libbb::verror_msg::bb_info_msg(
    b" hlen %x xid %x ciaddr %x yiaddr %x siaddr %x giaddr %x\x00" as *const u8
      as *const libc::c_char,
    (*packet).hlen as libc::c_int,
    (*packet).xid,
    (*packet).ciaddr,
    (*packet).yiaddr,
    (*packet).siaddr_nip,
    (*packet).gateway_nip,
  );
  *crate::libbb::xfuncs::bin2hex(
    buf.as_mut_ptr(),
    (*packet).chaddr.as_mut_ptr() as *mut libc::c_void as *const libc::c_char,
    ::std::mem::size_of::<[u8; 16]>() as libc::c_ulong as libc::c_int,
  ) = '\u{0}' as i32 as libc::c_char;
  crate::libbb::verror_msg::bb_info_msg(
    b" chaddr %s\x00" as *const u8 as *const libc::c_char,
    buf.as_mut_ptr(),
  );
}
/* Read a packet from socket fd, return -1 on read error, -2 on packet error */
#[no_mangle]
pub unsafe extern "C" fn udhcp_recv_kernel_packet(
  mut packet: *mut dhcp_packet,
  mut fd: libc::c_int,
) -> libc::c_int {
  let mut bytes: libc::c_int = 0;
  memset(
    packet as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<dhcp_packet>() as libc::c_ulong,
  );
  bytes = crate::libbb::read::safe_read(
    fd,
    packet as *mut libc::c_void,
    ::std::mem::size_of::<dhcp_packet>() as libc::c_ulong,
  ) as libc::c_int;
  if bytes < 0i32 {
    if dhcp_verbose >= 1i32 as libc::c_uint {
      crate::libbb::verror_msg::bb_simple_info_msg(
        b"packet read error, ignoring\x00" as *const u8 as *const libc::c_char,
      );
    }
    return bytes;
    /* returns -1 */
  }
  if (bytes as libc::c_ulong) < 240u64
    || (*packet).cookie
      != ({
        let mut __v: libc::c_uint = 0;
        let mut __x: libc::c_uint = 0x63825363i32 as libc::c_uint;
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
      })
  {
    crate::libbb::verror_msg::bb_simple_info_msg(
      b"packet with bad magic, ignoring\x00" as *const u8 as *const libc::c_char,
    );
    return -2i32;
  }
  if dhcp_verbose >= 1i32 as libc::c_uint {
    crate::libbb::verror_msg::bb_info_msg(
      b"received %s\x00" as *const u8 as *const libc::c_char,
      b"a packet\x00" as *const u8 as *const libc::c_char,
    );
  }
  udhcp_dump_packet(packet);
  return bytes;
}
/* Construct a ip/udp header for a packet, send packet */
#[no_mangle]
pub unsafe extern "C" fn udhcp_send_raw_packet(
  mut dhcp_pkt: *mut dhcp_packet,
  mut source_nip: u32,
  mut source_port: libc::c_int,
  mut dest_nip: u32,
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
  let mut packet: ip_udp_dhcp_packet = ip_udp_dhcp_packet {
    ip: iphdr {
      ihl_version: [0; 1],
      tos: 0,
      tot_len: 0,
      id: 0,
      frag_off: 0,
      ttl: 0,
      protocol: 0,
      check: 0,
      saddr: 0,
      daddr: 0,
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
    data: dhcp_packet {
      op: 0,
      htype: 0,
      hlen: 0,
      hops: 0,
      xid: 0,
      secs: 0,
      flags: 0,
      ciaddr: 0,
      yiaddr: 0,
      siaddr_nip: 0,
      gateway_nip: 0,
      chaddr: [0; 16],
      sname: [0; 64],
      file: [0; 128],
      cookie: 0,
      options: [0; 388],
    },
  };
  let mut padding: libc::c_uint = 0;
  let mut fd: libc::c_int = 0;
  let mut result: libc::c_int = -1i32;
  let mut msg: *const libc::c_char = std::ptr::null();
  fd = socket(
    17i32,
    SOCK_DGRAM as libc::c_int,
    ({
      let mut __v: libc::c_ushort = 0;
      let mut __x: libc::c_ushort = 0x800i32 as libc::c_ushort;
      if false {
        __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
          as libc::c_ushort
      } else {
        let fresh6 = &mut __v;
        let fresh7;
        let fresh8 = __x;
        asm!("rorw $$8, ${0:w}" : "=r" (fresh7) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh6, fresh8)) : "cc");
        c2rust_asm_casts::AsmCast::cast_out(fresh6, fresh8, fresh7);
      }
      __v
    }) as libc::c_int,
  );
  if fd < 0i32 {
    msg = b"socket(%s)\x00" as *const u8 as *const libc::c_char;
    current_block = 2404584983316808095;
  } else {
    memset(
      &mut dest_sll as *mut sockaddr_ll as *mut libc::c_void,
      0i32,
      ::std::mem::size_of::<sockaddr_ll>() as libc::c_ulong,
    );
    memset(
      &mut packet as *mut ip_udp_dhcp_packet as *mut libc::c_void,
      0i32,
      28u64,
    );
    packet.data = *dhcp_pkt;
    dest_sll.sll_family = 17i32 as libc::c_ushort;
    dest_sll.sll_protocol = {
      let mut __v: libc::c_ushort = 0;
      let mut __x: libc::c_ushort = 0x800i32 as libc::c_ushort;
      if false {
        __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
          as libc::c_ushort
      } else {
        let fresh9 = &mut __v;
        let fresh10;
        let fresh11 = __x;
        asm!("rorw $$8, ${0:w}" : "=r" (fresh10) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh9, fresh11)) : "cc");
        c2rust_asm_casts::AsmCast::cast_out(fresh9, fresh11, fresh10);
      }
      __v
    };
    dest_sll.sll_ifindex = ifindex;
    /*dest_sll.sll_hatype = ARPHRD_???;*/
    /*dest_sll.sll_pkttype = PACKET_???;*/
    dest_sll.sll_halen = 6i32 as libc::c_uchar;
    memcpy(
      dest_sll.sll_addr.as_mut_ptr() as *mut libc::c_void,
      dest_arp as *const libc::c_void,
      6i32 as libc::c_ulong,
    );
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
      /* We were sending full-sized DHCP packets (zero padded),
       * but some badly configured servers were seen dropping them.
       * Apparently they drop all DHCP packets >576 *ethernet* octets big,
       * whereas they may only drop packets >576 *IP* octets big
       * (which for typical Ethernet II means 590 octets: 6+6+2 + 576).
       *
       * In order to work with those buggy servers,
       * we truncate packets after end option byte.
       *
       * However, RFC 1542 says "The IP Total Length and UDP Length
       * must be large enough to contain the minimal BOOTP header of 300 octets".
       * Thus, we retain enough padding to not go below 300 BOOTP bytes.
       * Some devices have filters which drop DHCP packets shorter than that.
       */
      padding = (308i32
        - 1i32
        - crate::networking::udhcp::common::udhcp_end_option(packet.data.options.as_mut_ptr()))
        as libc::c_uint;
      if padding > (DHCP_SIZE as libc::c_int - 300i32) as libc::c_uint {
        padding = (DHCP_SIZE as libc::c_int - 300i32) as libc::c_uint
      }
      packet.ip.protocol = IPPROTO_UDP as libc::c_int as u8;
      packet.ip.saddr = source_nip;
      packet.ip.daddr = dest_nip;
      packet.udp.c2rust_unnamed.c2rust_unnamed_0.source = {
        let mut __v: libc::c_ushort = 0;
        let mut __x: libc::c_ushort = source_port as libc::c_ushort;
        if false {
          __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
            as libc::c_ushort
        } else {
          let fresh12 = &mut __v;
          let fresh13;
          let fresh14 = __x;
          asm!("rorw $$8, ${0:w}" : "=r" (fresh13) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh12, fresh14)) : "cc");
          c2rust_asm_casts::AsmCast::cast_out(fresh12, fresh14, fresh13);
        }
        __v
      };
      packet.udp.c2rust_unnamed.c2rust_unnamed_0.dest = {
        let mut __v: libc::c_ushort = 0;
        let mut __x: libc::c_ushort = dest_port as libc::c_ushort;
        if false {
          __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
            as libc::c_ushort
        } else {
          let fresh15 = &mut __v;
          let fresh16;
          let fresh17 = __x;
          asm!("rorw $$8, ${0:w}" : "=r" (fresh16) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh15, fresh17)) : "cc");
          c2rust_asm_casts::AsmCast::cast_out(fresh15, fresh17, fresh16);
        }
        __v
      };
      /* size, excluding IP header: */
      packet.udp.c2rust_unnamed.c2rust_unnamed_0.len = {
        let mut __v: libc::c_ushort = 0;
        let mut __x: libc::c_ushort =
          (UDP_DHCP_SIZE as libc::c_int as libc::c_uint).wrapping_sub(padding) as libc::c_ushort;
        if false {
          __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
            as libc::c_ushort
        } else {
          let fresh18 = &mut __v;
          let fresh19;
          let fresh20 = __x;
          asm!("rorw $$8, ${0:w}" : "=r" (fresh19) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh18, fresh20)) : "cc");
          c2rust_asm_casts::AsmCast::cast_out(fresh18, fresh20, fresh19);
        }
        __v
      };
      /* for UDP checksumming, ip.len is set to UDP packet len */
      packet.ip.tot_len = packet.udp.c2rust_unnamed.c2rust_unnamed_0.len;
      packet.udp.c2rust_unnamed.c2rust_unnamed_0.check = crate::libbb::inet_cksum::inet_cksum(
        &mut packet as *mut ip_udp_dhcp_packet as *mut u16,
        (IP_UDP_DHCP_SIZE as libc::c_int as libc::c_uint).wrapping_sub(padding) as libc::c_int,
      );
      /* but for sending, it is set to IP packet len */
      packet.ip.tot_len = {
        let mut __v: libc::c_ushort = 0;
        let mut __x: libc::c_ushort =
          (IP_UDP_DHCP_SIZE as libc::c_int as libc::c_uint).wrapping_sub(padding) as libc::c_ushort;
        if false {
          __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
            as libc::c_ushort
        } else {
          let fresh21 = &mut __v;
          let fresh22;
          let fresh23 = __x;
          asm!("rorw $$8, ${0:w}" : "=r" (fresh22) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh21, fresh23)) : "cc");
          c2rust_asm_casts::AsmCast::cast_out(fresh21, fresh23, fresh22);
        }
        __v
      };
      packet
        .ip
        .set_ihl((::std::mem::size_of::<iphdr>() as libc::c_ulong >> 2i32) as libc::c_uint);
      packet.ip.set_version(4i32 as libc::c_uint);
      packet.ip.ttl = 64i32 as u8;
      packet.ip.check = crate::libbb::inet_cksum::inet_cksum(
        &mut packet.ip as *mut iphdr as *mut u16,
        ::std::mem::size_of::<iphdr>() as libc::c_ulong as libc::c_int,
      );
      udhcp_dump_packet(dhcp_pkt);
      result = sendto(
        fd,
        &mut packet as *mut ip_udp_dhcp_packet as *const libc::c_void,
        (IP_UDP_DHCP_SIZE as libc::c_int as libc::c_uint).wrapping_sub(padding) as size_t,
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
      current_block = 2404584983316808095;
    } else {
      current_block = 2873832966593178012;
    }
  }
  match current_block {
    2404584983316808095 => {
      crate::libbb::perror_msg::bb_perror_msg(
        msg,
        b"PACKET\x00" as *const u8 as *const libc::c_char,
      );
    }
    _ => {}
  }
  return result;
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
/* Let the kernel do all the work for packet generation */
#[no_mangle]
pub unsafe extern "C" fn udhcp_send_kernel_packet(
  mut dhcp_pkt: *mut dhcp_packet,
  mut source_nip: u32,
  mut source_port: libc::c_int,
  mut dest_nip: u32,
  mut dest_port: libc::c_int,
) -> libc::c_int {
  let mut current_block: u64;
  let mut sa: sockaddr_in = sockaddr_in {
    sin_family: 0,
    sin_port: 0,
    sin_addr: in_addr { s_addr: 0 },
    sin_zero: [0; 8],
  };
  let mut padding: libc::c_uint = 0;
  let mut fd: libc::c_int = 0;
  let mut result: libc::c_int = -1i32;
  let mut msg: *const libc::c_char = std::ptr::null();
  fd = socket(2i32, SOCK_DGRAM as libc::c_int, IPPROTO_UDP as libc::c_int);
  if fd < 0i32 {
    msg = b"socket(%s)\x00" as *const u8 as *const libc::c_char;
    current_block = 840687141290369799;
  } else {
    crate::libbb::xconnect::setsockopt_reuseaddr(fd);
    memset(
      &mut sa as *mut sockaddr_in as *mut libc::c_void,
      0i32,
      ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong,
    );
    sa.sin_family = 2i32 as sa_family_t;
    sa.sin_port = {
      let mut __v: libc::c_ushort = 0;
      let mut __x: libc::c_ushort = source_port as libc::c_ushort;
      if false {
        __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
          as libc::c_ushort
      } else {
        let fresh24 = &mut __v;
        let fresh25;
        let fresh26 = __x;
        asm!("rorw $$8, ${0:w}" : "=r" (fresh25) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh24, fresh26)) : "cc");
        c2rust_asm_casts::AsmCast::cast_out(fresh24, fresh26, fresh25);
      }
      __v
    };
    sa.sin_addr.s_addr = source_nip;
    if bind(
      fd,
      __CONST_SOCKADDR_ARG {
        __sockaddr__: &mut sa as *mut sockaddr_in as *mut sockaddr,
      },
      ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
    ) == -1i32
    {
      msg = b"bind(%s)\x00" as *const u8 as *const libc::c_char
    } else {
      memset(
        &mut sa as *mut sockaddr_in as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong,
      );
      sa.sin_family = 2i32 as sa_family_t;
      sa.sin_port = {
        let mut __v: libc::c_ushort = 0;
        let mut __x: libc::c_ushort = dest_port as libc::c_ushort;
        if false {
          __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
            as libc::c_ushort
        } else {
          let fresh27 = &mut __v;
          let fresh28;
          let fresh29 = __x;
          asm!("rorw $$8, ${0:w}" : "=r" (fresh28) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh27, fresh29)) : "cc");
          c2rust_asm_casts::AsmCast::cast_out(fresh27, fresh29, fresh28);
        }
        __v
      };
      sa.sin_addr.s_addr = dest_nip;
      if connect(
        fd,
        __CONST_SOCKADDR_ARG {
          __sockaddr__: &mut sa as *mut sockaddr_in as *mut sockaddr,
        },
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
      ) == -1i32
      {
        msg = b"connect\x00" as *const u8 as *const libc::c_char
      } else {
        udhcp_dump_packet(dhcp_pkt);
        padding = (308i32
          - 1i32
          - crate::networking::udhcp::common::udhcp_end_option((*dhcp_pkt).options.as_mut_ptr()))
          as libc::c_uint;
        if padding > (DHCP_SIZE as libc::c_int - 300i32) as libc::c_uint {
          padding = (DHCP_SIZE as libc::c_int - 300i32) as libc::c_uint
        }
        result = crate::libbb::safe_write::safe_write(
          fd,
          dhcp_pkt as *const libc::c_void,
          (DHCP_SIZE as libc::c_int as libc::c_uint).wrapping_sub(padding) as size_t,
        ) as libc::c_int;
        msg = b"write\x00" as *const u8 as *const libc::c_char
      }
    }
    close(fd);
    if result < 0i32 {
      current_block = 840687141290369799;
    } else {
      current_block = 15925075030174552612;
    }
  }
  match current_block {
    840687141290369799 => {
      crate::libbb::perror_msg::bb_perror_msg(msg, b"UDP\x00" as *const u8 as *const libc::c_char);
    }
    _ => {}
  }
  return result;
}
