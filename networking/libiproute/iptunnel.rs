use c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use c2rust_bitfields;
use c2rust_bitfields::BitfieldStruct;
use libc;
extern "C" {
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn fgets_unlocked(
    __s: *mut libc::c_char,
    __n: libc::c_int,
    __stream: *mut FILE,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  fn exit(_: libc::c_int) -> !;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn inet_ntop(
    __af: libc::c_int,
    __cp: *const libc::c_void,
    __buf: *mut libc::c_char,
    __len: socklen_t,
  ) -> *const libc::c_char;
  #[no_mangle]
  fn xstrndup(s: *const libc::c_char, n: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn xsocket(domain: libc::c_int, type_0: libc::c_int, protocol: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn strncpy_IFNAMSIZ(dst: *mut libc::c_char, src: *const libc::c_char) -> *mut libc::c_char;
  /* Guaranteed to NOT be a macro (smallest code). Saves nearly 2k on uclibc.
   * But potentially slow, don't use in one-billion-times loops */
  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;
  /* Prints warning to stderr and returns NULL on failure: */
  #[no_mangle]
  fn fopen_or_warn(filename: *const libc::c_char, mode: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_simple_error_msg(s: *const libc::c_char);
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn index_in_strings(strings: *const libc::c_char, key: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn index_in_substrings(strings: *const libc::c_char, key: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn bb_ioctl_or_warn(
    fd: libc::c_int,
    request: libc::c_uint,
    argp: *mut libc::c_void,
    ioctl_name: *const libc::c_char,
  ) -> libc::c_int;
  #[no_mangle]
  fn bb_xioctl(
    fd: libc::c_int,
    request: libc::c_uint,
    argp: *mut libc::c_void,
    ioctl_name: *const libc::c_char,
  ) -> libc::c_int;
  #[no_mangle]
  static mut applet_name: *const libc::c_char;
  #[no_mangle]
  fn rtnl_dsfield_n2a(id: libc::c_int) -> *const libc::c_char;
  #[no_mangle]
  fn rtnl_dsfield_a2n(id: *mut uint32_t, arg: *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn duparg2(_: *const libc::c_char, _: *const libc::c_char) -> !;
  #[no_mangle]
  fn next_arg(argv: *mut *mut libc::c_char) -> *mut *mut libc::c_char;
  #[no_mangle]
  fn invarg_1_to_2(_: *const libc::c_char, _: *const libc::c_char) -> !;
  #[no_mangle]
  fn get_unsigned(arg: *mut libc::c_char, errmsg: *const libc::c_char) -> libc::c_uint;
  #[no_mangle]
  fn get_addr32(name: *mut libc::c_char) -> uint32_t;
  #[no_mangle]
  fn rt_addr_n2a(af: libc::c_int, addr: *mut libc::c_void) -> *const libc::c_char;
  #[no_mangle]
  static mut _SL_: libc::c_char;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __caddr_t = *mut libc::c_char;
pub type __socklen_t = libc::c_uint;
pub type size_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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
pub type in_addr_t = uint32_t;
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct iphdr {
  #[bitfield(name = "ihl", ty = "libc::c_uint", bits = "0..=3")]
  #[bitfield(name = "version", ty = "libc::c_uint", bits = "4..=7")]
  pub ihl_version: [u8; 1],
  pub tos: uint8_t,
  pub tot_len: uint16_t,
  pub id: uint16_t,
  pub frag_off: uint16_t,
  pub ttl: uint8_t,
  pub protocol: uint8_t,
  pub check: uint16_t,
  pub saddr: uint32_t,
  pub daddr: uint32_t,
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
pub union C2RustUnnamed_1 {
  pub ifrn_name: [libc::c_char; 16],
}
//#define SIOCGETPRL      (SIOCDEVPRIVATE + 4)
//#define SIOCADDPRL      (SIOCDEVPRIVATE + 5)
//#define SIOCDELPRL      (SIOCDEVPRIVATE + 6)
//#define SIOCCHGPRL      (SIOCDEVPRIVATE + 7)
//#define GRE_ROUTING     __constant_htons(0x4000)
//#define GRE_STRICT      __constant_htons(0x0800)
//#define GRE_REC         __constant_htons(0x0700)
//#define GRE_FLAGS       __constant_htons(0x00F8)
//#define GRE_VERSION     __constant_htons(0x0007)
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ip_tunnel_parm {
  pub name: [libc::c_char; 16],
  pub link: libc::c_int,
  pub i_flags: uint16_t,
  pub o_flags: uint16_t,
  pub i_key: uint32_t,
  pub o_key: uint32_t,
  pub iph: iphdr,
}
use crate::librb::_IO_FILE;
pub type _IO_lock_t = ();
use crate::librb::_IO_marker;
use crate::librb::FILE;
pub const ARG_name: C2RustUnnamed_2 = 26;
pub const ARG_inherit: C2RustUnnamed_2 = 23;
pub const ARG_dsfield: C2RustUnnamed_2 = 25;
pub const ARG_tos: C2RustUnnamed_2 = 24;
pub const ARG_ttl: C2RustUnnamed_2 = 22;
pub const ARG_dev: C2RustUnnamed_2 = 21;
pub const ARG_any: C2RustUnnamed_2 = 19;
pub const ARG_local: C2RustUnnamed_2 = 20;
pub const ARG_remote: C2RustUnnamed_2 = 18;
pub const ARG_pmtudisc: C2RustUnnamed_2 = 17;
pub const ARG_nopmtudisc: C2RustUnnamed_2 = 16;
pub const ARG_ocsum: C2RustUnnamed_2 = 15;
pub const ARG_icsum: C2RustUnnamed_2 = 14;
pub const ARG_csum: C2RustUnnamed_2 = 13;
pub const ARG_oseq: C2RustUnnamed_2 = 12;
pub const ARG_iseq: C2RustUnnamed_2 = 11;
pub const ARG_seq: C2RustUnnamed_2 = 10;
pub const ARG_okey: C2RustUnnamed_2 = 9;
pub const ARG_ikey: C2RustUnnamed_2 = 8;
pub const ARG_key: C2RustUnnamed_2 = 7;
pub const ARG_ip6_ip: C2RustUnnamed_2 = 6;
pub const ARG_sit: C2RustUnnamed_2 = 5;
pub const ARG_gre_ip: C2RustUnnamed_2 = 4;
pub const ARG_gre: C2RustUnnamed_2 = 3;
pub const ARG_ip_ip: C2RustUnnamed_2 = 2;
pub const ARG_ipip: C2RustUnnamed_2 = 1;
pub const ARG_mode: C2RustUnnamed_2 = 0;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const ARG_del: C2RustUnnamed_3 = 2;
pub const ARG_change: C2RustUnnamed_3 = 1;
pub const ARG_add: C2RustUnnamed_3 = 0;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const ARG_lst: C2RustUnnamed_3 = 5;
pub const ARG_list: C2RustUnnamed_3 = 4;
pub const ARG_show: C2RustUnnamed_3 = 3;
/* SIT-mode i_flags */
//#define SIT_ISATAP 0x0001
//struct ip_tunnel_prl {
//	uint32_t          addr;
//	uint16_t          flags;
//	uint16_t          __reserved;
//	uint32_t          datalen;
//	uint32_t          __reserved2;
//	/* data follows */
//};
// /* PRL flags */
//#define PRL_DEFAULT 0x0001
/* Dies on error */
unsafe extern "C" fn do_ioctl_get_ifindex(mut dev: *mut libc::c_char) -> libc::c_int {
  let mut ifr: ifreq = ifreq {
    ifr_ifrn: C2RustUnnamed_1 { ifrn_name: [0; 16] },
    ifr_ifru: C2RustUnnamed_0 {
      ifru_addr: sockaddr {
        sa_family: 0,
        sa_data: [0; 14],
      },
    },
  };
  let mut fd: libc::c_int = 0;
  strncpy_IFNAMSIZ(ifr.ifr_ifrn.ifrn_name.as_mut_ptr(), dev);
  fd = xsocket(2i32, SOCK_DGRAM as libc::c_int, 0i32);
  bb_xioctl(
    fd,
    0x8933i32 as libc::c_uint,
    &mut ifr as *mut ifreq as *mut libc::c_void,
    b"SIOCGIFINDEX\x00" as *const u8 as *const libc::c_char,
  );
  close(fd);
  return ifr.ifr_ifru.ifru_ivalue;
}
unsafe extern "C" fn do_ioctl_get_iftype(mut dev: *mut libc::c_char) -> libc::c_int {
  let mut ifr: ifreq = ifreq {
    ifr_ifrn: C2RustUnnamed_1 { ifrn_name: [0; 16] },
    ifr_ifru: C2RustUnnamed_0 {
      ifru_addr: sockaddr {
        sa_family: 0,
        sa_data: [0; 14],
      },
    },
  };
  let mut fd: libc::c_int = 0;
  let mut err: libc::c_int = 0;
  strncpy_IFNAMSIZ(ifr.ifr_ifrn.ifrn_name.as_mut_ptr(), dev);
  fd = xsocket(2i32, SOCK_DGRAM as libc::c_int, 0i32);
  err = bb_ioctl_or_warn(
    fd,
    0x8927i32 as libc::c_uint,
    &mut ifr as *mut ifreq as *mut libc::c_void,
    b"SIOCGIFHWADDR\x00" as *const u8 as *const libc::c_char,
  );
  close(fd);
  return if err != 0 {
    -1i32
  } else {
    ifr.ifr_ifru.ifru_addr.sa_family as libc::c_int
  };
}
unsafe extern "C" fn do_ioctl_get_ifname(mut idx: libc::c_int) -> *mut libc::c_char {
  let mut ifr: ifreq = ifreq {
    ifr_ifrn: C2RustUnnamed_1 { ifrn_name: [0; 16] },
    ifr_ifru: C2RustUnnamed_0 {
      ifru_addr: sockaddr {
        sa_family: 0,
        sa_data: [0; 14],
      },
    },
  };
  let mut fd: libc::c_int = 0;
  let mut err: libc::c_int = 0;
  ifr.ifr_ifru.ifru_ivalue = idx;
  fd = xsocket(2i32, SOCK_DGRAM as libc::c_int, 0i32);
  err = bb_ioctl_or_warn(
    fd,
    0x8910i32 as libc::c_uint,
    &mut ifr as *mut ifreq as *mut libc::c_void,
    b"SIOCGIFNAME\x00" as *const u8 as *const libc::c_char,
  );
  close(fd);
  return if err != 0 {
    0 as *mut libc::c_char
  } else {
    xstrndup(
      ifr.ifr_ifrn.ifrn_name.as_mut_ptr(),
      ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as libc::c_int,
    )
  };
}
unsafe extern "C" fn do_get_ioctl(
  mut basedev: *const libc::c_char,
  mut p: *mut ip_tunnel_parm,
) -> libc::c_int {
  let mut ifr: ifreq = ifreq {
    ifr_ifrn: C2RustUnnamed_1 { ifrn_name: [0; 16] },
    ifr_ifru: C2RustUnnamed_0 {
      ifru_addr: sockaddr {
        sa_family: 0,
        sa_data: [0; 14],
      },
    },
  };
  let mut fd: libc::c_int = 0;
  let mut err: libc::c_int = 0;
  strncpy_IFNAMSIZ(ifr.ifr_ifrn.ifrn_name.as_mut_ptr(), basedev);
  ifr.ifr_ifru.ifru_data = p as *mut libc::c_void as __caddr_t;
  fd = xsocket(2i32, SOCK_DGRAM as libc::c_int, 0i32);
  err = bb_ioctl_or_warn(
    fd,
    (0x89f0i32 + 0i32) as libc::c_uint,
    &mut ifr as *mut ifreq as *mut libc::c_void,
    b"SIOCGETTUNNEL\x00" as *const u8 as *const libc::c_char,
  );
  close(fd);
  return err;
}
/* Dies on error, otherwise returns 0 */
unsafe extern "C" fn do_add_ioctl(
  mut cmd: libc::c_int,
  mut basedev: *const libc::c_char,
  mut p: *mut ip_tunnel_parm,
) -> libc::c_int {
  let mut ifr: ifreq = ifreq {
    ifr_ifrn: C2RustUnnamed_1 { ifrn_name: [0; 16] },
    ifr_ifru: C2RustUnnamed_0 {
      ifru_addr: sockaddr {
        sa_family: 0,
        sa_data: [0; 14],
      },
    },
  };
  let mut fd: libc::c_int = 0;
  if cmd == 0x89f0i32 + 3i32 && (*p).name[0] as libc::c_int != 0 {
    strncpy_IFNAMSIZ(ifr.ifr_ifrn.ifrn_name.as_mut_ptr(), (*p).name.as_mut_ptr());
  } else {
    strncpy_IFNAMSIZ(ifr.ifr_ifrn.ifrn_name.as_mut_ptr(), basedev);
  }
  ifr.ifr_ifru.ifru_data = p as *mut libc::c_void as __caddr_t;
  fd = xsocket(2i32, SOCK_DGRAM as libc::c_int, 0i32);
  /* #define magic will turn ioctl# into string */
  if cmd == 0x89f0i32 + 3i32 {
    bb_xioctl(
      fd,
      (0x89f0i32 + 3i32) as libc::c_uint,
      &mut ifr as *mut ifreq as *mut libc::c_void,
      b"SIOCCHGTUNNEL\x00" as *const u8 as *const libc::c_char,
    );
  } else {
    bb_xioctl(
      fd,
      (0x89f0i32 + 1i32) as libc::c_uint,
      &mut ifr as *mut ifreq as *mut libc::c_void,
      b"SIOCADDTUNNEL\x00" as *const u8 as *const libc::c_char,
    );
  }
  close(fd);
  return 0i32;
}
/* Dies on error, otherwise returns 0 */
unsafe extern "C" fn do_del_ioctl(
  mut basedev: *const libc::c_char,
  mut p: *mut ip_tunnel_parm,
) -> libc::c_int {
  let mut ifr: ifreq = ifreq {
    ifr_ifrn: C2RustUnnamed_1 { ifrn_name: [0; 16] },
    ifr_ifru: C2RustUnnamed_0 {
      ifru_addr: sockaddr {
        sa_family: 0,
        sa_data: [0; 14],
      },
    },
  };
  let mut fd: libc::c_int = 0;
  if (*p).name[0] != 0 {
    strncpy_IFNAMSIZ(ifr.ifr_ifrn.ifrn_name.as_mut_ptr(), (*p).name.as_mut_ptr());
  } else {
    strncpy_IFNAMSIZ(ifr.ifr_ifrn.ifrn_name.as_mut_ptr(), basedev);
  }
  ifr.ifr_ifru.ifru_data = p as *mut libc::c_void as __caddr_t;
  fd = xsocket(2i32, SOCK_DGRAM as libc::c_int, 0i32);
  bb_xioctl(
    fd,
    (0x89f0i32 + 2i32) as libc::c_uint,
    &mut ifr as *mut ifreq as *mut libc::c_void,
    b"SIOCDELTUNNEL\x00" as *const u8 as *const libc::c_char,
  );
  close(fd);
  return 0i32;
}
/* Dies on error */
unsafe extern "C" fn parse_args(
  mut argv: *mut *mut libc::c_char,
  mut cmd: libc::c_int,
  mut p: *mut ip_tunnel_parm,
) {
  static mut keywords: [libc::c_char; 155] = [
    109, 111, 100, 101, 0, 105, 112, 105, 112, 0, 105, 112, 47, 105, 112, 0, 103, 114, 101, 0, 103,
    114, 101, 47, 105, 112, 0, 115, 105, 116, 0, 105, 112, 118, 54, 47, 105, 112, 0, 107, 101, 121,
    0, 105, 107, 101, 121, 0, 111, 107, 101, 121, 0, 115, 101, 113, 0, 105, 115, 101, 113, 0, 111,
    115, 101, 113, 0, 99, 115, 117, 109, 0, 105, 99, 115, 117, 109, 0, 111, 99, 115, 117, 109, 0,
    110, 111, 112, 109, 116, 117, 100, 105, 115, 99, 0, 112, 109, 116, 117, 100, 105, 115, 99, 0,
    114, 101, 109, 111, 116, 101, 0, 97, 110, 121, 0, 108, 111, 99, 97, 108, 0, 100, 101, 118, 0,
    116, 116, 108, 0, 105, 110, 104, 101, 114, 105, 116, 0, 116, 111, 115, 0, 100, 115, 102, 105,
    101, 108, 100, 0, 110, 97, 109, 101, 0, 0,
  ];
  let mut count: libc::c_int = 0i32;
  let mut medium: [libc::c_char; 16] = [0; 16];
  let mut key: libc::c_int = 0;
  memset(
    p as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<ip_tunnel_parm>() as libc::c_ulong,
  );
  medium[0] = '\u{0}' as i32 as libc::c_char;
  (*p).iph.set_version(4i32 as libc::c_uint);
  (*p).iph.set_ihl(5i32 as libc::c_uint);
  (*p).iph.frag_off = {
    let mut __v: libc::c_ushort = 0;
    let mut __x: libc::c_ushort = 0x4000i32 as libc::c_ushort;
    if 0 != 0 {
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
  while !(*argv).is_null() {
    key = index_in_strings(keywords.as_ptr(), *argv);
    if key == ARG_mode as libc::c_int {
      argv = next_arg(argv);
      key = index_in_strings(keywords.as_ptr(), *argv);
      if key == ARG_ipip as libc::c_int || key == ARG_ip_ip as libc::c_int {
        if (*p).iph.protocol as libc::c_int != 0
          && (*p).iph.protocol as libc::c_int != IPPROTO_IPIP as libc::c_int
        {
          bb_error_msg_and_die(
            b"%s tunnel mode\x00" as *const u8 as *const libc::c_char,
            b"you managed to ask for more than one\x00" as *const u8 as *const libc::c_char,
          );
        }
        (*p).iph.protocol = IPPROTO_IPIP as libc::c_int as uint8_t
      } else if key == ARG_gre as libc::c_int || key == ARG_gre_ip as libc::c_int {
        if (*p).iph.protocol as libc::c_int != 0
          && (*p).iph.protocol as libc::c_int != IPPROTO_GRE as libc::c_int
        {
          bb_error_msg_and_die(
            b"%s tunnel mode\x00" as *const u8 as *const libc::c_char,
            b"you managed to ask for more than one\x00" as *const u8 as *const libc::c_char,
          );
        }
        (*p).iph.protocol = IPPROTO_GRE as libc::c_int as uint8_t
      } else if key == ARG_sit as libc::c_int || key == ARG_ip6_ip as libc::c_int {
        if (*p).iph.protocol as libc::c_int != 0
          && (*p).iph.protocol as libc::c_int != IPPROTO_IPV6 as libc::c_int
        {
          bb_error_msg_and_die(
            b"%s tunnel mode\x00" as *const u8 as *const libc::c_char,
            b"you managed to ask for more than one\x00" as *const u8 as *const libc::c_char,
          );
        }
        (*p).iph.protocol = IPPROTO_IPV6 as libc::c_int as uint8_t
      } else {
        bb_error_msg_and_die(
          b"%s tunnel mode\x00" as *const u8 as *const libc::c_char,
          b"can\'t guess\x00" as *const u8 as *const libc::c_char,
        );
      }
    } else if key == ARG_key as libc::c_int {
      let mut uval: libc::c_uint = 0;
      argv = next_arg(argv);
      (*p).i_flags = ((*p).i_flags as libc::c_int
        | ({
          let mut __v: libc::c_ushort = 0;
          let mut __x: libc::c_ushort = 0x2000i32 as libc::c_ushort;
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
        }) as libc::c_int) as uint16_t;
      (*p).o_flags = ((*p).o_flags as libc::c_int
        | ({
          let mut __v: libc::c_ushort = 0;
          let mut __x: libc::c_ushort = 0x2000i32 as libc::c_ushort;
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
        }) as libc::c_int) as uint16_t;
      if !strchr(*argv, '.' as i32).is_null() {
        (*p).o_key = get_addr32(*argv);
        (*p).i_key = (*p).o_key
      } else {
        uval = get_unsigned(*argv, b"key\x00" as *const u8 as *const libc::c_char);
        (*p).o_key = {
          let mut __v: libc::c_uint = 0;
          let mut __x: libc::c_uint = uval;
          if 0 != 0 {
            __v = (__x & 0xff000000u32) >> 24i32
              | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
              | (__x & 0xff00i32 as libc::c_uint) << 8i32
              | (__x & 0xffi32 as libc::c_uint) << 24i32
          } else {
            let fresh9 = &mut __v;
            let fresh10;
            let fresh11 = __x;
            asm!("bswap $0" : "=r" (fresh10) : "0"
                                  (c2rust_asm_casts::AsmCast::cast_in(fresh9, fresh11))
                                  :);
            c2rust_asm_casts::AsmCast::cast_out(fresh9, fresh11, fresh10);
          }
          __v
        };
        (*p).i_key = (*p).o_key
      }
    } else if key == ARG_ikey as libc::c_int {
      let mut uval_0: libc::c_uint = 0;
      argv = next_arg(argv);
      (*p).i_flags = ((*p).i_flags as libc::c_int
        | ({
          let mut __v: libc::c_ushort = 0;
          let mut __x: libc::c_ushort = 0x2000i32 as libc::c_ushort;
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
        }) as libc::c_int) as uint16_t;
      if !strchr(*argv, '.' as i32).is_null() {
        (*p).o_key = get_addr32(*argv)
      } else {
        uval_0 = get_unsigned(*argv, b"ikey\x00" as *const u8 as *const libc::c_char);
        (*p).i_key = {
          let mut __v: libc::c_uint = 0;
          let mut __x: libc::c_uint = uval_0;
          if 0 != 0 {
            __v = (__x & 0xff000000u32) >> 24i32
              | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
              | (__x & 0xff00i32 as libc::c_uint) << 8i32
              | (__x & 0xffi32 as libc::c_uint) << 24i32
          } else {
            let fresh15 = &mut __v;
            let fresh16;
            let fresh17 = __x;
            asm!("bswap $0" : "=r" (fresh16) : "0"
                                  (c2rust_asm_casts::AsmCast::cast_in(fresh15, fresh17))
                                  :);
            c2rust_asm_casts::AsmCast::cast_out(fresh15, fresh17, fresh16);
          }
          __v
        }
      }
    } else if key == ARG_okey as libc::c_int {
      let mut uval_1: libc::c_uint = 0;
      argv = next_arg(argv);
      (*p).o_flags = ((*p).o_flags as libc::c_int
        | ({
          let mut __v: libc::c_ushort = 0;
          let mut __x: libc::c_ushort = 0x2000i32 as libc::c_ushort;
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
        }) as libc::c_int) as uint16_t;
      if !strchr(*argv, '.' as i32).is_null() {
        (*p).o_key = get_addr32(*argv)
      } else {
        uval_1 = get_unsigned(*argv, b"okey\x00" as *const u8 as *const libc::c_char);
        (*p).o_key = {
          let mut __v: libc::c_uint = 0;
          let mut __x: libc::c_uint = uval_1;
          if 0 != 0 {
            __v = (__x & 0xff000000u32) >> 24i32
              | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
              | (__x & 0xff00i32 as libc::c_uint) << 8i32
              | (__x & 0xffi32 as libc::c_uint) << 24i32
          } else {
            let fresh21 = &mut __v;
            let fresh22;
            let fresh23 = __x;
            asm!("bswap $0" : "=r" (fresh22) : "0"
                                  (c2rust_asm_casts::AsmCast::cast_in(fresh21, fresh23))
                                  :);
            c2rust_asm_casts::AsmCast::cast_out(fresh21, fresh23, fresh22);
          }
          __v
        }
      }
    } else if key == ARG_seq as libc::c_int {
      (*p).i_flags = ((*p).i_flags as libc::c_int
        | ({
          let mut __v: libc::c_ushort = 0;
          let mut __x: libc::c_ushort = 0x1000i32 as libc::c_ushort;
          if 0 != 0 {
            __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
              as libc::c_ushort
          } else {
            let fresh24 = &mut __v;
            let fresh25;
            let fresh26 = __x;
            asm!("rorw $$8, ${0:w}" : "=r" (fresh25) : "0"
                                   (c2rust_asm_casts::AsmCast::cast_in(fresh24, fresh26))
                                   : "cc");
            c2rust_asm_casts::AsmCast::cast_out(fresh24, fresh26, fresh25);
          }
          __v
        }) as libc::c_int) as uint16_t;
      (*p).o_flags = ((*p).o_flags as libc::c_int
        | ({
          let mut __v: libc::c_ushort = 0;
          let mut __x: libc::c_ushort = 0x1000i32 as libc::c_ushort;
          if 0 != 0 {
            __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
              as libc::c_ushort
          } else {
            let fresh27 = &mut __v;
            let fresh28;
            let fresh29 = __x;
            asm!("rorw $$8, ${0:w}" : "=r" (fresh28) : "0"
                                   (c2rust_asm_casts::AsmCast::cast_in(fresh27, fresh29))
                                   : "cc");
            c2rust_asm_casts::AsmCast::cast_out(fresh27, fresh29, fresh28);
          }
          __v
        }) as libc::c_int) as uint16_t
    } else if key == ARG_iseq as libc::c_int {
      (*p).i_flags = ((*p).i_flags as libc::c_int
        | ({
          let mut __v: libc::c_ushort = 0;
          let mut __x: libc::c_ushort = 0x1000i32 as libc::c_ushort;
          if 0 != 0 {
            __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
              as libc::c_ushort
          } else {
            let fresh30 = &mut __v;
            let fresh31;
            let fresh32 = __x;
            asm!("rorw $$8, ${0:w}" : "=r" (fresh31) : "0"
                                   (c2rust_asm_casts::AsmCast::cast_in(fresh30, fresh32))
                                   : "cc");
            c2rust_asm_casts::AsmCast::cast_out(fresh30, fresh32, fresh31);
          }
          __v
        }) as libc::c_int) as uint16_t
    } else if key == ARG_oseq as libc::c_int {
      (*p).o_flags = ((*p).o_flags as libc::c_int
        | ({
          let mut __v: libc::c_ushort = 0;
          let mut __x: libc::c_ushort = 0x1000i32 as libc::c_ushort;
          if 0 != 0 {
            __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
              as libc::c_ushort
          } else {
            let fresh33 = &mut __v;
            let fresh34;
            let fresh35 = __x;
            asm!("rorw $$8, ${0:w}" : "=r" (fresh34) : "0"
                                   (c2rust_asm_casts::AsmCast::cast_in(fresh33, fresh35))
                                   : "cc");
            c2rust_asm_casts::AsmCast::cast_out(fresh33, fresh35, fresh34);
          }
          __v
        }) as libc::c_int) as uint16_t
    } else if key == ARG_csum as libc::c_int {
      (*p).i_flags = ((*p).i_flags as libc::c_int
        | ({
          let mut __v: libc::c_ushort = 0;
          let mut __x: libc::c_ushort = 0x8000i32 as libc::c_ushort;
          if 0 != 0 {
            __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
              as libc::c_ushort
          } else {
            let fresh36 = &mut __v;
            let fresh37;
            let fresh38 = __x;
            asm!("rorw $$8, ${0:w}" : "=r" (fresh37) : "0"
                                   (c2rust_asm_casts::AsmCast::cast_in(fresh36, fresh38))
                                   : "cc");
            c2rust_asm_casts::AsmCast::cast_out(fresh36, fresh38, fresh37);
          }
          __v
        }) as libc::c_int) as uint16_t;
      (*p).o_flags = ((*p).o_flags as libc::c_int
        | ({
          let mut __v: libc::c_ushort = 0;
          let mut __x: libc::c_ushort = 0x8000i32 as libc::c_ushort;
          if 0 != 0 {
            __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
              as libc::c_ushort
          } else {
            let fresh39 = &mut __v;
            let fresh40;
            let fresh41 = __x;
            asm!("rorw $$8, ${0:w}" : "=r" (fresh40) : "0"
                                   (c2rust_asm_casts::AsmCast::cast_in(fresh39, fresh41))
                                   : "cc");
            c2rust_asm_casts::AsmCast::cast_out(fresh39, fresh41, fresh40);
          }
          __v
        }) as libc::c_int) as uint16_t
    } else if key == ARG_icsum as libc::c_int {
      (*p).i_flags = ((*p).i_flags as libc::c_int
        | ({
          let mut __v: libc::c_ushort = 0;
          let mut __x: libc::c_ushort = 0x8000i32 as libc::c_ushort;
          if 0 != 0 {
            __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
              as libc::c_ushort
          } else {
            let fresh42 = &mut __v;
            let fresh43;
            let fresh44 = __x;
            asm!("rorw $$8, ${0:w}" : "=r" (fresh43) : "0"
                                   (c2rust_asm_casts::AsmCast::cast_in(fresh42, fresh44))
                                   : "cc");
            c2rust_asm_casts::AsmCast::cast_out(fresh42, fresh44, fresh43);
          }
          __v
        }) as libc::c_int) as uint16_t
    } else if key == ARG_ocsum as libc::c_int {
      (*p).o_flags = ((*p).o_flags as libc::c_int
        | ({
          let mut __v: libc::c_ushort = 0;
          let mut __x: libc::c_ushort = 0x8000i32 as libc::c_ushort;
          if 0 != 0 {
            __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
              as libc::c_ushort
          } else {
            let fresh45 = &mut __v;
            let fresh46;
            let fresh47 = __x;
            asm!("rorw $$8, ${0:w}" : "=r" (fresh46) : "0"
                                   (c2rust_asm_casts::AsmCast::cast_in(fresh45, fresh47))
                                   : "cc");
            c2rust_asm_casts::AsmCast::cast_out(fresh45, fresh47, fresh46);
          }
          __v
        }) as libc::c_int) as uint16_t
    } else if key == ARG_nopmtudisc as libc::c_int {
      (*p).iph.frag_off = 0i32 as uint16_t
    } else if key == ARG_pmtudisc as libc::c_int {
      (*p).iph.frag_off = {
        let mut __v: libc::c_ushort = 0;
        let mut __x: libc::c_ushort = 0x4000i32 as libc::c_ushort;
        if 0 != 0 {
          __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
            as libc::c_ushort
        } else {
          let fresh48 = &mut __v;
          let fresh49;
          let fresh50 = __x;
          asm!("rorw $$8, ${0:w}" : "=r" (fresh49) : "0"
                              (c2rust_asm_casts::AsmCast::cast_in(fresh48, fresh50))
                              : "cc");
          c2rust_asm_casts::AsmCast::cast_out(fresh48, fresh50, fresh49);
        }
        __v
      }
    } else if key == ARG_remote as libc::c_int {
      argv = next_arg(argv);
      key = index_in_strings(keywords.as_ptr(), *argv);
      if key != ARG_any as libc::c_int {
        (*p).iph.daddr = get_addr32(*argv)
      }
    } else if key == ARG_local as libc::c_int {
      argv = next_arg(argv);
      key = index_in_strings(keywords.as_ptr(), *argv);
      if key != ARG_any as libc::c_int {
        (*p).iph.saddr = get_addr32(*argv)
      }
    } else if key == ARG_dev as libc::c_int {
      argv = next_arg(argv);
      strncpy_IFNAMSIZ(medium.as_mut_ptr(), *argv);
    } else if key == ARG_ttl as libc::c_int {
      let mut uval_2: libc::c_uint = 0;
      argv = next_arg(argv);
      key = index_in_strings(keywords.as_ptr(), *argv);
      if key != ARG_inherit as libc::c_int {
        uval_2 = get_unsigned(*argv, b"TTL\x00" as *const u8 as *const libc::c_char);
        if uval_2 > 255i32 as libc::c_uint {
          invarg_1_to_2(*argv, b"TTL\x00" as *const u8 as *const libc::c_char);
        }
        (*p).iph.ttl = uval_2 as uint8_t
      }
    } else if key == ARG_tos as libc::c_int || key == ARG_dsfield as libc::c_int {
      let mut uval_3: uint32_t = 0;
      argv = next_arg(argv);
      key = index_in_strings(keywords.as_ptr(), *argv);
      if key != ARG_inherit as libc::c_int {
        if rtnl_dsfield_a2n(&mut uval_3, *argv) != 0 {
          invarg_1_to_2(*argv, b"TOS\x00" as *const u8 as *const libc::c_char);
        }
        (*p).iph.tos = uval_3 as uint8_t
      } else {
        (*p).iph.tos = 1i32 as uint8_t
      }
    } else {
      if key == ARG_name as libc::c_int {
        argv = next_arg(argv)
      }
      if (*p).name[0] != 0 {
        duparg2(b"name\x00" as *const u8 as *const libc::c_char, *argv);
      }
      strncpy_IFNAMSIZ((*p).name.as_mut_ptr(), *argv);
      if cmd == 0x89f0i32 + 3i32 && count == 0i32 {
        let mut old_p: ip_tunnel_parm = ip_tunnel_parm {
          name: [0; 16],
          link: 0,
          i_flags: 0,
          o_flags: 0,
          i_key: 0,
          o_key: 0,
          iph: iphdr {
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
        };
        memset(
          &mut old_p as *mut ip_tunnel_parm as *mut libc::c_void,
          0i32,
          ::std::mem::size_of::<ip_tunnel_parm>() as libc::c_ulong,
        );
        if do_get_ioctl(*argv, &mut old_p) != 0 {
          exit(1i32);
        }
        *p = old_p
      }
    }
    count += 1;
    argv = argv.offset(1)
  }
  if (*p).iph.protocol as libc::c_int == 0i32 {
    if memcmp(
      (*p).name.as_mut_ptr() as *const libc::c_void,
      b"gre\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
      3i32 as libc::c_ulong,
    ) == 0i32
    {
      (*p).iph.protocol = IPPROTO_GRE as libc::c_int as uint8_t
    } else if memcmp(
      (*p).name.as_mut_ptr() as *const libc::c_void,
      b"ipip\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
      4i32 as libc::c_ulong,
    ) == 0i32
    {
      (*p).iph.protocol = IPPROTO_IPIP as libc::c_int as uint8_t
    } else if memcmp(
      (*p).name.as_mut_ptr() as *const libc::c_void,
      b"sit\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
      3i32 as libc::c_ulong,
    ) == 0i32
    {
      (*p).iph.protocol = IPPROTO_IPV6 as libc::c_int as uint8_t
    }
  }
  if (*p).iph.protocol as libc::c_int == IPPROTO_IPIP as libc::c_int
    || (*p).iph.protocol as libc::c_int == IPPROTO_IPV6 as libc::c_int
  {
    if (*p).i_flags as libc::c_int
      & ({
        let mut __v: libc::c_ushort = 0;
        let mut __x: libc::c_ushort = 0x2000i32 as libc::c_ushort;
        if 0 != 0 {
          __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
            as libc::c_ushort
        } else {
          let fresh51 = &mut __v;
          let fresh52;
          let fresh53 = __x;
          asm!("rorw $$8, ${0:w}" : "=r" (fresh52) : "0"
                             (c2rust_asm_casts::AsmCast::cast_in(fresh51, fresh53))
                             : "cc");
          c2rust_asm_casts::AsmCast::cast_out(fresh51, fresh53, fresh52);
        }
        __v
      }) as libc::c_int
      != 0
      || (*p).o_flags as libc::c_int
        & ({
          let mut __v: libc::c_ushort = 0;
          let mut __x: libc::c_ushort = 0x2000i32 as libc::c_ushort;
          if 0 != 0 {
            __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
              as libc::c_ushort
          } else {
            let fresh54 = &mut __v;
            let fresh55;
            let fresh56 = __x;
            asm!("rorw $$8, ${0:w}" : "=r" (fresh55) : "0"
                                 (c2rust_asm_casts::AsmCast::cast_in(fresh54, fresh56))
                                 : "cc");
            c2rust_asm_casts::AsmCast::cast_out(fresh54, fresh56, fresh55);
          }
          __v
        }) as libc::c_int
        != 0
    {
      bb_simple_error_msg_and_die(
        b"keys are not allowed with ipip and sit\x00" as *const u8 as *const libc::c_char,
      );
    }
  }
  if medium[0] != 0 {
    (*p).link = do_ioctl_get_ifindex(medium.as_mut_ptr())
  }
  if (*p).i_key == 0i32 as libc::c_uint
    && ({
      let mut __v: libc::c_uint = 0;
      let mut __x: libc::c_uint = (*p).iph.daddr;
      if 0 != 0 {
        __v = (__x & 0xff000000u32) >> 24i32
          | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
          | (__x & 0xff00i32 as libc::c_uint) << 8i32
          | (__x & 0xffi32 as libc::c_uint) << 24i32
      } else {
        let fresh57 = &mut __v;
        let fresh58;
        let fresh59 = __x;
        asm!("bswap $0" : "=r" (fresh58) : "0"
                         (c2rust_asm_casts::AsmCast::cast_in(fresh57, fresh59))
                         :);
        c2rust_asm_casts::AsmCast::cast_out(fresh57, fresh59, fresh58);
      }
      __v
    }) & 0xf0000000u32
      == 0xe0000000u32
  {
    (*p).i_key = (*p).iph.daddr;
    (*p).i_flags = ((*p).i_flags as libc::c_int
      | ({
        let mut __v: libc::c_ushort = 0;
        let mut __x: libc::c_ushort = 0x2000i32 as libc::c_ushort;
        if 0 != 0 {
          __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
            as libc::c_ushort
        } else {
          let fresh60 = &mut __v;
          let fresh61;
          let fresh62 = __x;
          asm!("rorw $$8, ${0:w}" : "=r" (fresh61) : "0"
                               (c2rust_asm_casts::AsmCast::cast_in(fresh60, fresh62))
                               : "cc");
          c2rust_asm_casts::AsmCast::cast_out(fresh60, fresh62, fresh61);
        }
        __v
      }) as libc::c_int) as uint16_t
  }
  if (*p).o_key == 0i32 as libc::c_uint
    && ({
      let mut __v: libc::c_uint = 0;
      let mut __x: libc::c_uint = (*p).iph.daddr;
      if 0 != 0 {
        __v = (__x & 0xff000000u32) >> 24i32
          | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
          | (__x & 0xff00i32 as libc::c_uint) << 8i32
          | (__x & 0xffi32 as libc::c_uint) << 24i32
      } else {
        let fresh63 = &mut __v;
        let fresh64;
        let fresh65 = __x;
        asm!("bswap $0" : "=r" (fresh64) : "0"
                         (c2rust_asm_casts::AsmCast::cast_in(fresh63, fresh65))
                         :);
        c2rust_asm_casts::AsmCast::cast_out(fresh63, fresh65, fresh64);
      }
      __v
    }) & 0xf0000000u32
      == 0xe0000000u32
  {
    (*p).o_key = (*p).iph.daddr;
    (*p).o_flags = ((*p).o_flags as libc::c_int
      | ({
        let mut __v: libc::c_ushort = 0;
        let mut __x: libc::c_ushort = 0x2000i32 as libc::c_ushort;
        if 0 != 0 {
          __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
            as libc::c_ushort
        } else {
          let fresh66 = &mut __v;
          let fresh67;
          let fresh68 = __x;
          asm!("rorw $$8, ${0:w}" : "=r" (fresh67) : "0"
                               (c2rust_asm_casts::AsmCast::cast_in(fresh66, fresh68))
                               : "cc");
          c2rust_asm_casts::AsmCast::cast_out(fresh66, fresh68, fresh67);
        }
        __v
      }) as libc::c_int) as uint16_t
  }
  if ({
    let mut __v: libc::c_uint = 0;
    let mut __x: libc::c_uint = (*p).iph.daddr;
    if 0 != 0 {
      __v = (__x & 0xff000000u32) >> 24i32
        | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
        | (__x & 0xff00i32 as libc::c_uint) << 8i32
        | (__x & 0xffi32 as libc::c_uint) << 24i32
    } else {
      let fresh69 = &mut __v;
      let fresh70;
      let fresh71 = __x;
      asm!("bswap $0" : "=r" (fresh70) : "0"
                     (c2rust_asm_casts::AsmCast::cast_in(fresh69, fresh71))
                     :);
      c2rust_asm_casts::AsmCast::cast_out(fresh69, fresh71, fresh70);
    }
    __v
  }) & 0xf0000000u32
    == 0xe0000000u32
    && (*p).iph.saddr == 0
  {
    bb_simple_error_msg_and_die(
      b"broadcast tunnel requires a source address\x00" as *const u8 as *const libc::c_char,
    );
  };
}
/* Return value becomes exitcode. It's okay to not return at all */
unsafe extern "C" fn do_add(mut cmd: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
  let mut p: ip_tunnel_parm = ip_tunnel_parm {
    name: [0; 16],
    link: 0,
    i_flags: 0,
    o_flags: 0,
    i_key: 0,
    o_key: 0,
    iph: iphdr {
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
  };
  parse_args(argv, cmd, &mut p);
  if p.iph.ttl as libc::c_int != 0 && p.iph.frag_off as libc::c_int == 0i32 {
    bb_simple_error_msg_and_die(
      b"ttl != 0 and noptmudisc are incompatible\x00" as *const u8 as *const libc::c_char,
    );
  }
  match p.iph.protocol as libc::c_int {
    4 => {
      return do_add_ioctl(
        cmd,
        b"tunl0\x00" as *const u8 as *const libc::c_char,
        &mut p,
      )
    }
    47 => return do_add_ioctl(cmd, b"gre0\x00" as *const u8 as *const libc::c_char, &mut p),
    41 => return do_add_ioctl(cmd, b"sit0\x00" as *const u8 as *const libc::c_char, &mut p),
    _ => {
      bb_simple_error_msg_and_die(
        b"can\'t determine tunnel mode (ipip, gre or sit)\x00" as *const u8 as *const libc::c_char,
      );
    }
  };
}
/* Return value becomes exitcode. It's okay to not return at all */
unsafe extern "C" fn do_del(mut argv: *mut *mut libc::c_char) -> libc::c_int {
  let mut p: ip_tunnel_parm = ip_tunnel_parm {
    name: [0; 16],
    link: 0,
    i_flags: 0,
    o_flags: 0,
    i_key: 0,
    o_key: 0,
    iph: iphdr {
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
  };
  parse_args(argv, 0x89f0i32 + 2i32, &mut p);
  match p.iph.protocol as libc::c_int {
    4 => return do_del_ioctl(b"tunl0\x00" as *const u8 as *const libc::c_char, &mut p),
    47 => return do_del_ioctl(b"gre0\x00" as *const u8 as *const libc::c_char, &mut p),
    41 => return do_del_ioctl(b"sit0\x00" as *const u8 as *const libc::c_char, &mut p),
    _ => return do_del_ioctl(p.name.as_mut_ptr(), &mut p),
  };
}
unsafe extern "C" fn print_tunnel(mut p: *mut ip_tunnel_parm) {
  let mut s3: [libc::c_char; 16] = [0; 16];
  let mut s4: [libc::c_char; 16] = [0; 16];
  printf(
    b"%s: %s/ip  remote %s  local %s \x00" as *const u8 as *const libc::c_char,
    (*p).name.as_mut_ptr(),
    if (*p).iph.protocol as libc::c_int == IPPROTO_IPIP as libc::c_int {
      b"ip\x00" as *const u8 as *const libc::c_char
    } else if (*p).iph.protocol as libc::c_int == IPPROTO_GRE as libc::c_int {
      b"gre\x00" as *const u8 as *const libc::c_char
    } else if (*p).iph.protocol as libc::c_int == IPPROTO_IPV6 as libc::c_int {
      b"ipv6\x00" as *const u8 as *const libc::c_char
    } else {
      b"unknown\x00" as *const u8 as *const libc::c_char
    },
    if (*p).iph.daddr != 0 {
      rt_addr_n2a(
        2i32,
        &mut (*p).iph.daddr as *mut uint32_t as *mut libc::c_void,
      )
    } else {
      b"any\x00" as *const u8 as *const libc::c_char
    },
    if (*p).iph.saddr != 0 {
      rt_addr_n2a(
        2i32,
        &mut (*p).iph.saddr as *mut uint32_t as *mut libc::c_void,
      )
    } else {
      b"any\x00" as *const u8 as *const libc::c_char
    },
  );
  if (*p).link != 0 {
    let mut n: *mut libc::c_char = do_ioctl_get_ifname((*p).link);
    if !n.is_null() {
      printf(b" dev %s \x00" as *const u8 as *const libc::c_char, n);
      free(n as *mut libc::c_void);
    }
  }
  if (*p).iph.ttl != 0 {
    printf(
      b" ttl %d \x00" as *const u8 as *const libc::c_char,
      (*p).iph.ttl as libc::c_int,
    );
  } else {
    printf(b" ttl inherit \x00" as *const u8 as *const libc::c_char);
  }
  if (*p).iph.tos != 0 {
    printf(b" tos\x00" as *const u8 as *const libc::c_char);
    if (*p).iph.tos as libc::c_int & 1i32 != 0 {
      printf(b" inherit\x00" as *const u8 as *const libc::c_char);
    }
    if (*p).iph.tos as libc::c_int & !1i32 != 0 {
      printf(
        b"%c%s \x00" as *const u8 as *const libc::c_char,
        if (*p).iph.tos as libc::c_int & 1i32 != 0 {
          '/' as i32
        } else {
          ' ' as i32
        },
        rtnl_dsfield_n2a((*p).iph.tos as libc::c_int & !1i32),
      );
    }
  }
  if (*p).iph.frag_off as libc::c_int
    & ({
      let mut __v: libc::c_ushort = 0;
      let mut __x: libc::c_ushort = 0x4000i32 as libc::c_ushort;
      if 0 != 0 {
        __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
          as libc::c_ushort
      } else {
        let fresh72 = &mut __v;
        let fresh73;
        let fresh74 = __x;
        asm!("rorw $$8, ${0:w}" : "=r" (fresh73) : "0"
                         (c2rust_asm_casts::AsmCast::cast_in(fresh72, fresh74))
                         : "cc");
        c2rust_asm_casts::AsmCast::cast_out(fresh72, fresh74, fresh73);
      }
      __v
    }) as libc::c_int
    == 0
  {
    printf(b" nopmtudisc\x00" as *const u8 as *const libc::c_char);
  }
  inet_ntop(
    2i32,
    &mut (*p).i_key as *mut uint32_t as *const libc::c_void,
    s3.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as socklen_t,
  );
  inet_ntop(
    2i32,
    &mut (*p).o_key as *mut uint32_t as *const libc::c_void,
    s4.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as socklen_t,
  );
  if (*p).i_flags as libc::c_int
    & ({
      let mut __v: libc::c_ushort = 0;
      let mut __x: libc::c_ushort = 0x2000i32 as libc::c_ushort;
      if 0 != 0 {
        __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
          as libc::c_ushort
      } else {
        let fresh75 = &mut __v;
        let fresh76;
        let fresh77 = __x;
        asm!("rorw $$8, ${0:w}" : "=r" (fresh76) : "0"
                         (c2rust_asm_casts::AsmCast::cast_in(fresh75, fresh77))
                         : "cc");
        c2rust_asm_casts::AsmCast::cast_out(fresh75, fresh77, fresh76);
      }
      __v
    }) as libc::c_int
    != 0
    && (*p).o_flags as libc::c_int
      & ({
        let mut __v: libc::c_ushort = 0;
        let mut __x: libc::c_ushort = 0x2000i32 as libc::c_ushort;
        if 0 != 0 {
          __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
            as libc::c_ushort
        } else {
          let fresh78 = &mut __v;
          let fresh79;
          let fresh80 = __x;
          asm!("rorw $$8, ${0:w}" : "=r" (fresh79) : "0"
                             (c2rust_asm_casts::AsmCast::cast_in(fresh78, fresh80))
                             : "cc");
          c2rust_asm_casts::AsmCast::cast_out(fresh78, fresh80, fresh79);
        }
        __v
      }) as libc::c_int
      != 0
    && (*p).o_key == (*p).i_key
  {
    printf(
      b" key %s\x00" as *const u8 as *const libc::c_char,
      s3.as_mut_ptr(),
    );
  } else {
    if (*p).i_flags as libc::c_int
      & ({
        let mut __v: libc::c_ushort = 0;
        let mut __x: libc::c_ushort = 0x2000i32 as libc::c_ushort;
        if 0 != 0 {
          __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
            as libc::c_ushort
        } else {
          let fresh81 = &mut __v;
          let fresh82;
          let fresh83 = __x;
          asm!("rorw $$8, ${0:w}" : "=r" (fresh82) : "0"
                             (c2rust_asm_casts::AsmCast::cast_in(fresh81, fresh83))
                             : "cc");
          c2rust_asm_casts::AsmCast::cast_out(fresh81, fresh83, fresh82);
        }
        __v
      }) as libc::c_int
      != 0
    {
      printf(
        b" ikey %s \x00" as *const u8 as *const libc::c_char,
        s3.as_mut_ptr(),
      );
    }
    if (*p).o_flags as libc::c_int
      & ({
        let mut __v: libc::c_ushort = 0;
        let mut __x: libc::c_ushort = 0x2000i32 as libc::c_ushort;
        if 0 != 0 {
          __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
            as libc::c_ushort
        } else {
          let fresh84 = &mut __v;
          let fresh85;
          let fresh86 = __x;
          asm!("rorw $$8, ${0:w}" : "=r" (fresh85) : "0"
                             (c2rust_asm_casts::AsmCast::cast_in(fresh84, fresh86))
                             : "cc");
          c2rust_asm_casts::AsmCast::cast_out(fresh84, fresh86, fresh85);
        }
        __v
      }) as libc::c_int
      != 0
    {
      printf(
        b" okey %s \x00" as *const u8 as *const libc::c_char,
        s4.as_mut_ptr(),
      );
    }
  }
  if (*p).i_flags as libc::c_int
    & ({
      let mut __v: libc::c_ushort = 0;
      let mut __x: libc::c_ushort = 0x1000i32 as libc::c_ushort;
      if 0 != 0 {
        __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
          as libc::c_ushort
      } else {
        let fresh87 = &mut __v;
        let fresh88;
        let fresh89 = __x;
        asm!("rorw $$8, ${0:w}" : "=r" (fresh88) : "0"
                         (c2rust_asm_casts::AsmCast::cast_in(fresh87, fresh89))
                         : "cc");
        c2rust_asm_casts::AsmCast::cast_out(fresh87, fresh89, fresh88);
      }
      __v
    }) as libc::c_int
    != 0
  {
    printf(
      b"%c  Drop packets out of sequence.\n\x00" as *const u8 as *const libc::c_char,
      _SL_ as libc::c_int,
    );
  }
  if (*p).i_flags as libc::c_int
    & ({
      let mut __v: libc::c_ushort = 0;
      let mut __x: libc::c_ushort = 0x8000i32 as libc::c_ushort;
      if 0 != 0 {
        __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
          as libc::c_ushort
      } else {
        let fresh90 = &mut __v;
        let fresh91;
        let fresh92 = __x;
        asm!("rorw $$8, ${0:w}" : "=r" (fresh91) : "0"
                         (c2rust_asm_casts::AsmCast::cast_in(fresh90, fresh92))
                         : "cc");
        c2rust_asm_casts::AsmCast::cast_out(fresh90, fresh92, fresh91);
      }
      __v
    }) as libc::c_int
    != 0
  {
    printf(
      b"%c  Checksum in received packet is required.\x00" as *const u8 as *const libc::c_char,
      _SL_ as libc::c_int,
    );
  }
  if (*p).o_flags as libc::c_int
    & ({
      let mut __v: libc::c_ushort = 0;
      let mut __x: libc::c_ushort = 0x1000i32 as libc::c_ushort;
      if 0 != 0 {
        __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
          as libc::c_ushort
      } else {
        let fresh93 = &mut __v;
        let fresh94;
        let fresh95 = __x;
        asm!("rorw $$8, ${0:w}" : "=r" (fresh94) : "0"
                         (c2rust_asm_casts::AsmCast::cast_in(fresh93, fresh95))
                         : "cc");
        c2rust_asm_casts::AsmCast::cast_out(fresh93, fresh95, fresh94);
      }
      __v
    }) as libc::c_int
    != 0
  {
    printf(
      b"%c  Sequence packets on output.\x00" as *const u8 as *const libc::c_char,
      _SL_ as libc::c_int,
    );
  }
  if (*p).o_flags as libc::c_int
    & ({
      let mut __v: libc::c_ushort = 0;
      let mut __x: libc::c_ushort = 0x8000i32 as libc::c_ushort;
      if 0 != 0 {
        __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
          as libc::c_ushort
      } else {
        let fresh96 = &mut __v;
        let fresh97;
        let fresh98 = __x;
        asm!("rorw $$8, ${0:w}" : "=r" (fresh97) : "0"
                         (c2rust_asm_casts::AsmCast::cast_in(fresh96, fresh98))
                         : "cc");
        c2rust_asm_casts::AsmCast::cast_out(fresh96, fresh98, fresh97);
      }
      __v
    }) as libc::c_int
    != 0
  {
    printf(
      b"%c  Checksum output packets.\x00" as *const u8 as *const libc::c_char,
      _SL_ as libc::c_int,
    );
  };
}
unsafe extern "C" fn do_tunnels_list(mut p: *mut ip_tunnel_parm) {
  let mut name: [libc::c_char; 16] = [0; 16];
  let mut rx_bytes: libc::c_ulong = 0;
  let mut rx_packets: libc::c_ulong = 0;
  let mut rx_errs: libc::c_ulong = 0;
  let mut rx_drops: libc::c_ulong = 0;
  let mut rx_fifo: libc::c_ulong = 0;
  let mut rx_frame: libc::c_ulong = 0;
  let mut tx_bytes: libc::c_ulong = 0;
  let mut tx_packets: libc::c_ulong = 0;
  let mut tx_errs: libc::c_ulong = 0;
  let mut tx_drops: libc::c_ulong = 0;
  let mut tx_fifo: libc::c_ulong = 0;
  let mut tx_colls: libc::c_ulong = 0;
  let mut tx_carrier: libc::c_ulong = 0;
  let mut rx_multi: libc::c_ulong = 0;
  let mut type_0: libc::c_int = 0;
  let mut p1: ip_tunnel_parm = ip_tunnel_parm {
    name: [0; 16],
    link: 0,
    i_flags: 0,
    o_flags: 0,
    i_key: 0,
    o_key: 0,
    iph: iphdr {
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
  };
  let mut buf: [libc::c_char; 512] = [0; 512];
  let mut fp: *mut FILE = fopen_or_warn(
    b"/proc/net/dev\x00" as *const u8 as *const libc::c_char,
    b"r\x00" as *const u8 as *const libc::c_char,
  );
  if fp.is_null() {
    return;
  }
  /* skip headers */
  fgets_unlocked(
    buf.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong as libc::c_int,
    fp,
  );
  fgets_unlocked(
    buf.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong as libc::c_int,
    fp,
  );
  while !fgets_unlocked(
    buf.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong as libc::c_int,
    fp,
  )
  .is_null()
  {
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    /*buf[sizeof(buf) - 1] = 0; - fgets is safe anyway */
    ptr = strchr(buf.as_mut_ptr(), ':' as i32);
    if ptr.is_null() || {
      let fresh99 = ptr;
      ptr = ptr.offset(1);
      *fresh99 = 0i32 as libc::c_char;
      (sscanf(
        buf.as_mut_ptr(),
        b"%s\x00" as *const u8 as *const libc::c_char,
        name.as_mut_ptr(),
      )) != 1i32
    } {
      bb_simple_error_msg(b"wrong format of /proc/net/dev\x00" as *const u8 as *const libc::c_char);
      return;
    }
    if sscanf(
      ptr,
      b"%lu%lu%lu%lu%lu%lu%lu%*d%lu%lu%lu%lu%lu%lu%lu\x00" as *const u8 as *const libc::c_char,
      &mut rx_bytes as *mut libc::c_ulong,
      &mut rx_packets as *mut libc::c_ulong,
      &mut rx_errs as *mut libc::c_ulong,
      &mut rx_drops as *mut libc::c_ulong,
      &mut rx_fifo as *mut libc::c_ulong,
      &mut rx_frame as *mut libc::c_ulong,
      &mut rx_multi as *mut libc::c_ulong,
      &mut tx_bytes as *mut libc::c_ulong,
      &mut tx_packets as *mut libc::c_ulong,
      &mut tx_errs as *mut libc::c_ulong,
      &mut tx_drops as *mut libc::c_ulong,
      &mut tx_fifo as *mut libc::c_ulong,
      &mut tx_colls as *mut libc::c_ulong,
      &mut tx_carrier as *mut libc::c_ulong,
    ) != 14i32
    {
      continue;
    }
    if (*p).name[0] as libc::c_int != 0 && strcmp((*p).name.as_mut_ptr(), name.as_mut_ptr()) != 0 {
      continue;
    }
    type_0 = do_ioctl_get_iftype(name.as_mut_ptr());
    if type_0 == -1i32 {
      bb_error_msg(
        b"can\'t get type of [%s]\x00" as *const u8 as *const libc::c_char,
        name.as_mut_ptr(),
      );
    } else {
      if type_0 != 768i32 && type_0 != 778i32 && type_0 != 776i32 {
        continue;
      }
      memset(
        &mut p1 as *mut ip_tunnel_parm as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<ip_tunnel_parm>() as libc::c_ulong,
      );
      if do_get_ioctl(name.as_mut_ptr(), &mut p1) != 0 {
        continue;
      }
      if (*p).link != 0 && p1.link != (*p).link
        || (*p).name[0] as libc::c_int != 0
          && strcmp(p1.name.as_mut_ptr(), (*p).name.as_mut_ptr()) != 0
        || (*p).iph.daddr != 0 && p1.iph.daddr != (*p).iph.daddr
        || (*p).iph.saddr != 0 && p1.iph.saddr != (*p).iph.saddr
        || (*p).i_key != 0 && p1.i_key != (*p).i_key
      {
        continue;
      }
      print_tunnel(&mut p1);
      bb_putchar('\n' as i32);
    }
  }
}
/* Return value becomes exitcode. It's okay to not return at all */
unsafe extern "C" fn do_show(mut argv: *mut *mut libc::c_char) -> libc::c_int {
  let mut err: libc::c_int = 0;
  let mut p: ip_tunnel_parm = ip_tunnel_parm {
    name: [0; 16],
    link: 0,
    i_flags: 0,
    o_flags: 0,
    i_key: 0,
    o_key: 0,
    iph: iphdr {
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
  };
  parse_args(argv, 0x89f0i32 + 0i32, &mut p);
  match p.iph.protocol as libc::c_int {
    4 => {
      err = do_get_ioctl(
        if p.name[0] as libc::c_int != 0 {
          p.name.as_mut_ptr()
        } else {
          b"tunl0\x00" as *const u8 as *const libc::c_char
        },
        &mut p,
      )
    }
    47 => {
      err = do_get_ioctl(
        if p.name[0] as libc::c_int != 0 {
          p.name.as_mut_ptr()
        } else {
          b"gre0\x00" as *const u8 as *const libc::c_char
        },
        &mut p,
      )
    }
    41 => {
      err = do_get_ioctl(
        if p.name[0] as libc::c_int != 0 {
          p.name.as_mut_ptr()
        } else {
          b"sit0\x00" as *const u8 as *const libc::c_char
        },
        &mut p,
      )
    }
    _ => {
      do_tunnels_list(&mut p);
      return 0i32;
    }
  }
  if err != 0 {
    return -1i32;
  }
  print_tunnel(&mut p);
  bb_putchar('\n' as i32);
  return 0i32;
}
/* vi: set sw=4 ts=4: */
//int FAST_FUNC print_neigh(struct sockaddr_nl *who, struct nlmsghdr *n, void *arg);
//int FAST_FUNC iproute_monitor(char **argv);
//void FAST_FUNC ipneigh_reset_filter(void);
/* Return value becomes exitcode. It's okay to not return at all */
#[no_mangle]
pub unsafe extern "C" fn do_iptunnel(mut argv: *mut *mut libc::c_char) -> libc::c_int {
  static mut keywords: [libc::c_char; 33] = [
    97, 100, 100, 0, 99, 104, 97, 110, 103, 101, 0, 100, 101, 108, 101, 116, 101, 0, 115, 104, 111,
    119, 0, 108, 105, 115, 116, 0, 108, 115, 116, 0, 0,
  ];
  if !(*argv).is_null() {
    let mut key: libc::c_int = index_in_substrings(keywords.as_ptr(), *argv);
    if key < 0i32 {
      invarg_1_to_2(*argv, applet_name);
    }
    argv = argv.offset(1);
    if key == ARG_add as libc::c_int {
      return do_add(0x89f0i32 + 1i32, argv);
    }
    if key == ARG_change as libc::c_int {
      return do_add(0x89f0i32 + 3i32, argv);
    }
    if key == ARG_del as libc::c_int {
      return do_del(argv);
    }
  }
  return do_show(argv);
}
