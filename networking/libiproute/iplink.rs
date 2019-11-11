use c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;

use libc;
use libc::close;
use libc::strcmp;
extern "C" {
  #[no_mangle]
  fn socket(__domain: libc::c_int, __type: libc::c_int, __protocol: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  fn exit(_: libc::c_int) -> !;

  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn xsocket(domain: libc::c_int, type_0: libc::c_int, protocol: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xbind(sockfd: libc::c_int, my_addr: *mut sockaddr, addrlen: socklen_t);
  #[no_mangle]
  fn bb_getsockname(
    sockfd: libc::c_int,
    addr: *mut libc::c_void,
    addrlen: socklen_t,
  ) -> libc::c_int;
  #[no_mangle]
  fn strncpy_IFNAMSIZ(dst: *mut libc::c_char, src: *const libc::c_char) -> *mut libc::c_char;
  /* Convert each alpha char in str to lower-case */
  #[no_mangle]
  fn str_tolower(str: *mut libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  static mut xfunc_error_retval: u8;
  #[no_mangle]
  fn xfunc_die() -> !;
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn index_in_strings(strings: *const libc::c_char, key: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn index_in_substrings(strings: *const libc::c_char, key: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn nth_string(strings: *const libc::c_char, n: libc::c_int) -> *const libc::c_char;
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
  static bb_msg_requires_arg: [libc::c_char; 0];
  #[no_mangle]
  fn ipaddr_list_or_flush(argv: *mut *mut libc::c_char, flush: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn ll_addr_a2n(
    lladdr: *mut libc::c_uchar,
    len: libc::c_int,
    arg: *mut libc::c_char,
  ) -> libc::c_int;
  #[no_mangle]
  fn ll_init_map(rth: *mut rtnl_handle) -> libc::c_int;
  #[no_mangle]
  fn xll_name_to_index(name: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  static mut preferred_family: family_t;

  /* We need linux/types.h because older kernels use u32 etc
   * in linux/[rt]netlink.h. 2.6.19 seems to be ok, though */
  #[no_mangle]
  fn xrtnl_open(rth: *mut rtnl_handle);
  #[no_mangle]
  fn addattr_l(
    n: *mut nlmsghdr,
    maxlen: libc::c_int,
    type_0: libc::c_int,
    data: *mut libc::c_void,
    alen: libc::c_int,
  ) -> libc::c_int;
  #[no_mangle]
  fn rtnl_talk(rtnl: *mut rtnl_handle, n: *mut nlmsghdr, answer: *mut nlmsghdr) -> libc::c_int;
  #[no_mangle]
  fn next_arg(argv: *mut *mut libc::c_char) -> *mut *mut libc::c_char;
  #[no_mangle]
  fn get_unsigned(arg: *mut libc::c_char, errmsg: *const libc::c_char) -> libc::c_uint;
  #[no_mangle]
  fn get_u32(arg: *mut libc::c_char, errmsg: *const libc::c_char) -> u32;
  #[no_mangle]
  fn get_u16(arg: *mut libc::c_char, errmsg: *const libc::c_char) -> u16;
  #[no_mangle]
  fn invarg_1_to_2(_: *const libc::c_char, _: *const libc::c_char) -> !;
  #[no_mangle]
  fn duparg(_: *const libc::c_char, _: *const libc::c_char) -> !;
  #[no_mangle]
  fn duparg2(_: *const libc::c_char, _: *const libc::c_char) -> !;
}

pub type __caddr_t = *mut libc::c_char;
pub type __socklen_t = libc::c_uint;

use crate::librb::size_t;
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
pub type C2RustUnnamed = libc::c_uint;
pub const IFF_DYNAMIC: C2RustUnnamed = 32768;
pub const IFF_AUTOMEDIA: C2RustUnnamed = 16384;
pub const IFF_PORTSEL: C2RustUnnamed = 8192;
pub const IFF_MULTICAST: C2RustUnnamed = 4096;
pub const IFF_SLAVE: C2RustUnnamed = 2048;
pub const IFF_MASTER: C2RustUnnamed = 1024;
pub const IFF_ALLMULTI: C2RustUnnamed = 512;
pub const IFF_PROMISC: C2RustUnnamed = 256;
pub const IFF_NOARP: C2RustUnnamed = 128;
pub const IFF_RUNNING: C2RustUnnamed = 64;
pub const IFF_NOTRAILERS: C2RustUnnamed = 32;
pub const IFF_POINTOPOINT: C2RustUnnamed = 16;
pub const IFF_LOOPBACK: C2RustUnnamed = 8;
pub const IFF_DEBUG: C2RustUnnamed = 4;
pub const IFF_BROADCAST: C2RustUnnamed = 2;
pub const IFF_UP: C2RustUnnamed = 1;
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
pub type __u16 = libc::c_ushort;
pub type u32 = libc::c_uint;
/* NB: unaligned parameter should be a pointer, aligned one -
 * a lvalue. This makes it more likely to not swap them by mistake
 */
/* #elif ... - add your favorite arch today! */
/* Unaligned, fixed-endian accessors */
/* unxz needs an aligned fixed-endian accessor.
 * (however, the compiler does not realize it's aligned, the cast is still necessary)
 */
/* ---- Size-saving "small" ints (arch-dependent) ----------- */
/* add other arches which benefit from this... */
pub type smalluint = libc::c_uchar;
pub type family_t = i8;
pub type __kernel_sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_nl {
  pub nl_family: __kernel_sa_family_t,
  pub nl_pad: libc::c_ushort,
  pub nl_pid: u32,
  pub nl_groups: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nlmsghdr {
  pub nlmsg_len: u32,
  pub nlmsg_type: __u16,
  pub nlmsg_flags: __u16,
  pub nlmsg_seq: u32,
  pub nlmsg_pid: u32,
}
pub type C2RustUnnamed_2 = libc::c_uint;
pub const __IFLA_MAX: C2RustUnnamed_2 = 50;
pub const IFLA_NEW_IFINDEX: C2RustUnnamed_2 = 49;
pub const IFLA_CARRIER_DOWN_COUNT: C2RustUnnamed_2 = 48;
pub const IFLA_CARRIER_UP_COUNT: C2RustUnnamed_2 = 47;
pub const IFLA_IF_NETNSID: C2RustUnnamed_2 = 46;
pub const IFLA_NEW_NETNSID: C2RustUnnamed_2 = 45;
pub const IFLA_EVENT: C2RustUnnamed_2 = 44;
pub const IFLA_XDP: C2RustUnnamed_2 = 43;
pub const IFLA_PAD: C2RustUnnamed_2 = 42;
pub const IFLA_GSO_MAX_SIZE: C2RustUnnamed_2 = 41;
pub const IFLA_GSO_MAX_SEGS: C2RustUnnamed_2 = 40;
pub const IFLA_PROTO_DOWN: C2RustUnnamed_2 = 39;
pub const IFLA_PHYS_PORT_NAME: C2RustUnnamed_2 = 38;
pub const IFLA_LINK_NETNSID: C2RustUnnamed_2 = 37;
pub const IFLA_PHYS_SWITCH_ID: C2RustUnnamed_2 = 36;
pub const IFLA_CARRIER_CHANGES: C2RustUnnamed_2 = 35;
pub const IFLA_PHYS_PORT_ID: C2RustUnnamed_2 = 34;
pub const IFLA_CARRIER: C2RustUnnamed_2 = 33;
pub const IFLA_NUM_RX_QUEUES: C2RustUnnamed_2 = 32;
pub const IFLA_NUM_TX_QUEUES: C2RustUnnamed_2 = 31;
pub const IFLA_PROMISCUITY: C2RustUnnamed_2 = 30;
pub const IFLA_EXT_MASK: C2RustUnnamed_2 = 29;
pub const IFLA_NET_NS_FD: C2RustUnnamed_2 = 28;
pub const IFLA_GROUP: C2RustUnnamed_2 = 27;
pub const IFLA_AF_SPEC: C2RustUnnamed_2 = 26;
pub const IFLA_PORT_SELF: C2RustUnnamed_2 = 25;
pub const IFLA_VF_PORTS: C2RustUnnamed_2 = 24;
pub const IFLA_STATS64: C2RustUnnamed_2 = 23;
pub const IFLA_VFINFO_LIST: C2RustUnnamed_2 = 22;
pub const IFLA_NUM_VF: C2RustUnnamed_2 = 21;
pub const IFLA_IFALIAS: C2RustUnnamed_2 = 20;
pub const IFLA_NET_NS_PID: C2RustUnnamed_2 = 19;
pub const IFLA_LINKINFO: C2RustUnnamed_2 = 18;
pub const IFLA_LINKMODE: C2RustUnnamed_2 = 17;
pub const IFLA_OPERSTATE: C2RustUnnamed_2 = 16;
pub const IFLA_WEIGHT: C2RustUnnamed_2 = 15;
pub const IFLA_MAP: C2RustUnnamed_2 = 14;
pub const IFLA_TXQLEN: C2RustUnnamed_2 = 13;
pub const IFLA_PROTINFO: C2RustUnnamed_2 = 12;
pub const IFLA_WIRELESS: C2RustUnnamed_2 = 11;
pub const IFLA_MASTER: C2RustUnnamed_2 = 10;
pub const IFLA_PRIORITY: C2RustUnnamed_2 = 9;
pub const IFLA_COST: C2RustUnnamed_2 = 8;
pub const IFLA_STATS: C2RustUnnamed_2 = 7;
pub const IFLA_QDISC: C2RustUnnamed_2 = 6;
pub const IFLA_LINK: C2RustUnnamed_2 = 5;
pub const IFLA_MTU: C2RustUnnamed_2 = 4;
pub const IFLA_IFNAME: C2RustUnnamed_2 = 3;
pub const IFLA_BROADCAST: C2RustUnnamed_2 = 2;
pub const IFLA_ADDRESS: C2RustUnnamed_2 = 1;
pub const IFLA_UNSPEC: C2RustUnnamed_2 = 0;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const __IFLA_INFO_MAX: C2RustUnnamed_3 = 6;
pub const IFLA_INFO_SLAVE_DATA: C2RustUnnamed_3 = 5;
pub const IFLA_INFO_SLAVE_KIND: C2RustUnnamed_3 = 4;
pub const IFLA_INFO_XSTATS: C2RustUnnamed_3 = 3;
pub const IFLA_INFO_DATA: C2RustUnnamed_3 = 2;
pub const IFLA_INFO_KIND: C2RustUnnamed_3 = 1;
pub const IFLA_INFO_UNSPEC: C2RustUnnamed_3 = 0;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const __IFLA_VLAN_MAX: C2RustUnnamed_4 = 6;
pub const IFLA_VLAN_PROTOCOL: C2RustUnnamed_4 = 5;
pub const IFLA_VLAN_INGRESS_QOS: C2RustUnnamed_4 = 4;
pub const IFLA_VLAN_EGRESS_QOS: C2RustUnnamed_4 = 3;
pub const IFLA_VLAN_FLAGS: C2RustUnnamed_4 = 2;
pub const IFLA_VLAN_ID: C2RustUnnamed_4 = 1;
pub const IFLA_VLAN_UNSPEC: C2RustUnnamed_4 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifla_vlan_flags {
  pub flags: u32,
  pub mask: u32,
}
pub type C2RustUnnamed_5 = libc::c_uint;
pub const __IFLA_VRF_MAX: C2RustUnnamed_5 = 2;
pub const IFLA_VRF_TABLE: C2RustUnnamed_5 = 1;
pub const IFLA_VRF_UNSPEC: C2RustUnnamed_5 = 0;
pub type C2RustUnnamed_6 = libc::c_uint;
pub const __RTM_MAX: C2RustUnnamed_6 = 97;
pub const RTM_NEWCACHEREPORT: C2RustUnnamed_6 = 96;
pub const RTM_GETSTATS: C2RustUnnamed_6 = 94;
pub const RTM_NEWSTATS: C2RustUnnamed_6 = 92;
pub const RTM_GETNSID: C2RustUnnamed_6 = 90;
pub const RTM_DELNSID: C2RustUnnamed_6 = 89;
pub const RTM_NEWNSID: C2RustUnnamed_6 = 88;
pub const RTM_GETMDB: C2RustUnnamed_6 = 86;
pub const RTM_DELMDB: C2RustUnnamed_6 = 85;
pub const RTM_NEWMDB: C2RustUnnamed_6 = 84;
pub const RTM_GETNETCONF: C2RustUnnamed_6 = 82;
pub const RTM_DELNETCONF: C2RustUnnamed_6 = 81;
pub const RTM_NEWNETCONF: C2RustUnnamed_6 = 80;
pub const RTM_SETDCB: C2RustUnnamed_6 = 79;
pub const RTM_GETDCB: C2RustUnnamed_6 = 78;
pub const RTM_GETADDRLABEL: C2RustUnnamed_6 = 74;
pub const RTM_DELADDRLABEL: C2RustUnnamed_6 = 73;
pub const RTM_NEWADDRLABEL: C2RustUnnamed_6 = 72;
pub const RTM_NEWNDUSEROPT: C2RustUnnamed_6 = 68;
pub const RTM_SETNEIGHTBL: C2RustUnnamed_6 = 67;
pub const RTM_GETNEIGHTBL: C2RustUnnamed_6 = 66;
pub const RTM_NEWNEIGHTBL: C2RustUnnamed_6 = 64;
pub const RTM_GETANYCAST: C2RustUnnamed_6 = 62;
pub const RTM_GETMULTICAST: C2RustUnnamed_6 = 58;
pub const RTM_NEWPREFIX: C2RustUnnamed_6 = 52;
pub const RTM_GETACTION: C2RustUnnamed_6 = 50;
pub const RTM_DELACTION: C2RustUnnamed_6 = 49;
pub const RTM_NEWACTION: C2RustUnnamed_6 = 48;
pub const RTM_GETTFILTER: C2RustUnnamed_6 = 46;
pub const RTM_DELTFILTER: C2RustUnnamed_6 = 45;
pub const RTM_NEWTFILTER: C2RustUnnamed_6 = 44;
pub const RTM_GETTCLASS: C2RustUnnamed_6 = 42;
pub const RTM_DELTCLASS: C2RustUnnamed_6 = 41;
pub const RTM_NEWTCLASS: C2RustUnnamed_6 = 40;
pub const RTM_GETQDISC: C2RustUnnamed_6 = 38;
pub const RTM_DELQDISC: C2RustUnnamed_6 = 37;
pub const RTM_NEWQDISC: C2RustUnnamed_6 = 36;
pub const RTM_GETRULE: C2RustUnnamed_6 = 34;
pub const RTM_DELRULE: C2RustUnnamed_6 = 33;
pub const RTM_NEWRULE: C2RustUnnamed_6 = 32;
pub const RTM_GETNEIGH: C2RustUnnamed_6 = 30;
pub const RTM_DELNEIGH: C2RustUnnamed_6 = 29;
pub const RTM_NEWNEIGH: C2RustUnnamed_6 = 28;
pub const RTM_GETROUTE: C2RustUnnamed_6 = 26;
pub const RTM_DELROUTE: C2RustUnnamed_6 = 25;
pub const RTM_NEWROUTE: C2RustUnnamed_6 = 24;
pub const RTM_GETADDR: C2RustUnnamed_6 = 22;
pub const RTM_DELADDR: C2RustUnnamed_6 = 21;
pub const RTM_NEWADDR: C2RustUnnamed_6 = 20;
pub const RTM_SETLINK: C2RustUnnamed_6 = 19;
pub const RTM_GETLINK: C2RustUnnamed_6 = 18;
pub const RTM_DELLINK: C2RustUnnamed_6 = 17;
pub const RTM_NEWLINK: C2RustUnnamed_6 = 16;
pub const RTM_BASE: C2RustUnnamed_6 = 16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rtattr {
  pub rta_len: libc::c_ushort,
  pub rta_type: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifinfomsg {
  pub ifi_family: libc::c_uchar,
  pub __ifi_pad: libc::c_uchar,
  pub ifi_type: libc::c_ushort,
  pub ifi_index: libc::c_int,
  pub ifi_flags: libc::c_uint,
  pub ifi_change: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
  pub n: nlmsghdr,
  pub i: ifinfomsg,
  pub buf: [libc::c_char; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rtnl_handle {
  pub fd: libc::c_int,
  pub local: sockaddr_nl,
  pub peer: sockaddr_nl,
  pub seq: u32,
  pub dump: u32,
}
pub const PARM_on: C2RustUnnamed_8 = 0;
pub const ARG_promisc: C2RustUnnamed_9 = 7;
pub const ARG_arp: C2RustUnnamed_9 = 6;
pub const ARG_multicast: C2RustUnnamed_9 = 5;
pub const ARG_dev: C2RustUnnamed_9 = 11;
pub const ARG_nomaster: C2RustUnnamed_9 = 10;
pub const ARG_master: C2RustUnnamed_9 = 9;
pub const ARG_addr: C2RustUnnamed_9 = 8;
pub const ARG_qlen: C2RustUnnamed_9 = 4;
pub const ARG_mtu: C2RustUnnamed_9 = 3;
pub const ARG_name: C2RustUnnamed_9 = 2;
pub const ARG_down: C2RustUnnamed_9 = 1;
pub const ARG_up: C2RustUnnamed_9 = 0;
pub type C2RustUnnamed_8 = libc::c_uint;
pub const PARM_off: C2RustUnnamed_8 = 1;
pub type C2RustUnnamed_9 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
  pub n: nlmsghdr,
  pub i: ifinfomsg,
  pub buf: [libc::c_char; 1024],
}
pub const PARM_on_0: C2RustUnnamed_11 = 0;
pub const ARG_mvrp: C2RustUnnamed_13 = 4;
pub const ARG_gvrp: C2RustUnnamed_13 = 3;
pub const ARG_reorder_hdr: C2RustUnnamed_13 = 2;
pub const PROTO_8021AD: C2RustUnnamed_12 = 1;
pub const PROTO_8021Q: C2RustUnnamed_12 = 0;
pub const ARG_protocol: C2RustUnnamed_13 = 1;
pub const ARG_id: C2RustUnnamed_13 = 0;
pub type C2RustUnnamed_11 = libc::c_uint;
pub const PARM_off_0: C2RustUnnamed_11 = 1;
pub type C2RustUnnamed_12 = libc::c_uint;
pub type C2RustUnnamed_13 = libc::c_uint;
pub const ARG_loose_binding: C2RustUnnamed_13 = 5;
pub const ARG_dev_0: C2RustUnnamed_14 = 3;
pub const ARG_address: C2RustUnnamed_14 = 4;
pub const ARG_name_0: C2RustUnnamed_14 = 1;
pub const ARG_link: C2RustUnnamed_14 = 0;
pub const ARG_type: C2RustUnnamed_14 = 2;
pub type C2RustUnnamed_14 = libc::c_uint;
/* Exits on error */
unsafe extern "C" fn get_ctl_fd() -> libc::c_int {
  let mut fd: libc::c_int = 0;
  fd = socket(2i32, SOCK_DGRAM as libc::c_int, 0i32);
  if fd >= 0i32 {
    return fd;
  }
  fd = socket(17i32, SOCK_DGRAM as libc::c_int, 0i32);
  if fd >= 0i32 {
    return fd;
  }
  return xsocket(10i32, SOCK_DGRAM as libc::c_int, 0i32);
}
/* Exits on error */
unsafe extern "C" fn do_chflags(mut dev: *mut libc::c_char, mut flags: u32, mut mask: u32) {
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
  fd = get_ctl_fd();
  bb_xioctl(
    fd,
    0x8913i32 as libc::c_uint,
    &mut ifr as *mut ifreq as *mut libc::c_void,
    b"SIOCGIFFLAGS\x00" as *const u8 as *const libc::c_char,
  );
  if (ifr.ifr_ifru.ifru_flags as libc::c_uint ^ flags) & mask != 0 {
    ifr.ifr_ifru.ifru_flags = (ifr.ifr_ifru.ifru_flags as libc::c_uint & !mask) as libc::c_short;
    ifr.ifr_ifru.ifru_flags =
      (ifr.ifr_ifru.ifru_flags as libc::c_uint | mask & flags) as libc::c_short;
    bb_xioctl(
      fd,
      0x8914i32 as libc::c_uint,
      &mut ifr as *mut ifreq as *mut libc::c_void,
      b"SIOCSIFFLAGS\x00" as *const u8 as *const libc::c_char,
    );
  }
  close(fd);
}
/* Exits on error */
unsafe extern "C" fn do_changename(mut dev: *mut libc::c_char, mut newdev: *mut libc::c_char) {
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
  strncpy_IFNAMSIZ(ifr.ifr_ifru.ifru_newname.as_mut_ptr(), newdev);
  fd = get_ctl_fd();
  bb_xioctl(
    fd,
    0x8923i32 as libc::c_uint,
    &mut ifr as *mut ifreq as *mut libc::c_void,
    b"SIOCSIFNAME\x00" as *const u8 as *const libc::c_char,
  );
  close(fd);
}
/* Exits on error */
unsafe extern "C" fn set_qlen(mut dev: *mut libc::c_char, mut qlen: libc::c_int) {
  let mut ifr: ifreq = ifreq {
    ifr_ifrn: C2RustUnnamed_1 { ifrn_name: [0; 16] },
    ifr_ifru: C2RustUnnamed_0 {
      ifru_addr: sockaddr {
        sa_family: 0,
        sa_data: [0; 14],
      },
    },
  };
  let mut s: libc::c_int = 0;
  s = get_ctl_fd();
  memset(
    &mut ifr as *mut ifreq as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<ifreq>() as libc::c_ulong,
  );
  strncpy_IFNAMSIZ(ifr.ifr_ifrn.ifrn_name.as_mut_ptr(), dev);
  ifr.ifr_ifru.ifru_ivalue = qlen;
  bb_xioctl(
    s,
    0x8943i32 as libc::c_uint,
    &mut ifr as *mut ifreq as *mut libc::c_void,
    b"SIOCSIFTXQLEN\x00" as *const u8 as *const libc::c_char,
  );
  close(s);
}
/* Exits on error */
unsafe extern "C" fn set_mtu(mut dev: *mut libc::c_char, mut mtu: libc::c_int) {
  let mut ifr: ifreq = ifreq {
    ifr_ifrn: C2RustUnnamed_1 { ifrn_name: [0; 16] },
    ifr_ifru: C2RustUnnamed_0 {
      ifru_addr: sockaddr {
        sa_family: 0,
        sa_data: [0; 14],
      },
    },
  };
  let mut s: libc::c_int = 0;
  s = get_ctl_fd();
  memset(
    &mut ifr as *mut ifreq as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<ifreq>() as libc::c_ulong,
  );
  strncpy_IFNAMSIZ(ifr.ifr_ifrn.ifrn_name.as_mut_ptr(), dev);
  ifr.ifr_ifru.ifru_mtu = mtu;
  bb_xioctl(
    s,
    0x8922i32 as libc::c_uint,
    &mut ifr as *mut ifreq as *mut libc::c_void,
    b"SIOCSIFMTU\x00" as *const u8 as *const libc::c_char,
  );
  close(s);
}
/* Exits on error */
unsafe extern "C" fn set_master(mut dev: *mut libc::c_char, mut master: libc::c_int) {
  let mut rth: rtnl_handle = rtnl_handle {
    fd: 0,
    local: sockaddr_nl {
      nl_family: 0,
      nl_pad: 0,
      nl_pid: 0,
      nl_groups: 0,
    },
    peer: sockaddr_nl {
      nl_family: 0,
      nl_pad: 0,
      nl_pid: 0,
      nl_groups: 0,
    },
    seq: 0,
    dump: 0,
  };
  let mut req: C2RustUnnamed_7 = C2RustUnnamed_7 {
    n: nlmsghdr {
      nlmsg_len: 0,
      nlmsg_type: 0,
      nlmsg_flags: 0,
      nlmsg_seq: 0,
      nlmsg_pid: 0,
    },
    i: ifinfomsg {
      ifi_family: 0,
      __ifi_pad: 0,
      ifi_type: 0,
      ifi_index: 0,
      ifi_flags: 0,
      ifi_change: 0,
    },
    buf: [0; 1024],
  };
  memset(
    &mut req as *mut C2RustUnnamed_7 as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<C2RustUnnamed_7>() as libc::c_ulong,
  );
  req.n.nlmsg_len = (::std::mem::size_of::<ifinfomsg>() as libc::c_ulong).wrapping_add(
    ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
      .wrapping_add(4u32 as libc::c_ulong)
      .wrapping_sub(1i32 as libc::c_ulong)
      & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong) as libc::c_int
      as libc::c_ulong,
  ) as u32;
  req.n.nlmsg_flags = 0x1i32 as __u16;
  req.n.nlmsg_type = RTM_NEWLINK as libc::c_int as __u16;
  req.i.ifi_family = preferred_family as libc::c_uchar;
  xrtnl_open(&mut rth);
  req.i.ifi_index = xll_name_to_index(dev);
  //printf("master %i for %i\n", master, req.i.ifi_index);
  addattr_l(
    &mut req.n,
    ::std::mem::size_of::<C2RustUnnamed_7>() as libc::c_ulong as libc::c_int,
    IFLA_MASTER as libc::c_int,
    &mut master as *mut libc::c_int as *mut libc::c_void,
    4i32,
  );
  if rtnl_talk(&mut rth, &mut req.n, 0 as *mut nlmsghdr) < 0i32 {
    xfunc_die();
  };
}
/* Exits on error */
unsafe extern "C" fn get_address(
  mut dev: *mut libc::c_char,
  mut htype: *mut libc::c_int,
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
  let mut me: sockaddr_ll = sockaddr_ll {
    sll_family: 0,
    sll_protocol: 0,
    sll_ifindex: 0,
    sll_hatype: 0,
    sll_pkttype: 0,
    sll_halen: 0,
    sll_addr: [0; 8],
  };
  let mut s: libc::c_int = 0;
  s = xsocket(17i32, SOCK_DGRAM as libc::c_int, 0i32);
  /*memset(&ifr, 0, sizeof(ifr)); - SIOCGIFINDEX does not need to clear all */
  strncpy_IFNAMSIZ(ifr.ifr_ifrn.ifrn_name.as_mut_ptr(), dev);
  bb_xioctl(
    s,
    0x8933i32 as libc::c_uint,
    &mut ifr as *mut ifreq as *mut libc::c_void,
    b"SIOCGIFINDEX\x00" as *const u8 as *const libc::c_char,
  );
  memset(
    &mut me as *mut sockaddr_ll as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<sockaddr_ll>() as libc::c_ulong,
  );
  me.sll_family = 17i32 as libc::c_ushort;
  me.sll_ifindex = ifr.ifr_ifru.ifru_ivalue;
  me.sll_protocol = {
    let mut __v: libc::c_ushort = 0;
    let mut __x: libc::c_ushort = 0x60i32 as libc::c_ushort;
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
  xbind(
    s,
    &mut me as *mut sockaddr_ll as *mut sockaddr,
    ::std::mem::size_of::<sockaddr_ll>() as libc::c_ulong as socklen_t,
  );
  bb_getsockname(
    s,
    &mut me as *mut sockaddr_ll as *mut sockaddr as *mut libc::c_void,
    ::std::mem::size_of::<sockaddr_ll>() as libc::c_ulong as socklen_t,
  );
  //never happens:
  //if (getsockname(s, (struct sockaddr*)&me, &alen) == -1)
  //	bb_perror_msg_and_die("getsockname");
  close(s);
  *htype = me.sll_hatype as libc::c_int;
  return me.sll_halen as libc::c_int;
}
/* Exits on error */
unsafe extern "C" fn parse_address(
  mut dev: *mut libc::c_char,
  mut hatype: libc::c_int,
  mut halen: libc::c_int,
  mut lla: *mut libc::c_char,
  mut ifr: *mut ifreq,
) {
  let mut alen: libc::c_int = 0; /*INFINIBAND_HLEN*/
  memset(
    ifr as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<ifreq>() as libc::c_ulong,
  );
  strncpy_IFNAMSIZ((*ifr).ifr_ifrn.ifrn_name.as_mut_ptr(), dev);
  (*ifr).ifr_ifru.ifru_hwaddr.sa_family = hatype as sa_family_t;
  alen = if hatype == 1i32 { 14i32 } else { 19i32 };
  alen = ll_addr_a2n(
    (*ifr).ifr_ifru.ifru_hwaddr.sa_data.as_mut_ptr() as *mut libc::c_uchar,
    alen,
    lla,
  );
  if alen < 0i32 {
    exit(1i32);
  }
  if alen != halen {
    bb_error_msg_and_die(
      b"wrong address (%s) length: expected %d bytes\x00" as *const u8 as *const libc::c_char,
      lla,
      halen,
    );
  };
}
/* Exits on error */
unsafe extern "C" fn set_address(mut ifr: *mut ifreq, mut brd: libc::c_int) {
  let mut s: libc::c_int = 0;
  s = get_ctl_fd();
  if brd != 0 {
    bb_xioctl(
      s,
      0x8937i32 as libc::c_uint,
      ifr as *mut libc::c_void,
      b"SIOCSIFHWBROADCAST\x00" as *const u8 as *const libc::c_char,
    );
  } else {
    bb_xioctl(
      s,
      0x8924i32 as libc::c_uint,
      ifr as *mut libc::c_void,
      b"SIOCSIFHWADDR\x00" as *const u8 as *const libc::c_char,
    );
  }
  close(s);
}
unsafe extern "C" fn die_must_be_on_off(mut msg: *const libc::c_char) -> ! {
  bb_error_msg_and_die(
    b"argument of \"%s\" must be \"on\" or \"off\"\x00" as *const u8 as *const libc::c_char,
    msg,
  );
}
/* Return value becomes exitcode. It's okay to not return at all */
unsafe extern "C" fn do_set(mut argv: *mut *mut libc::c_char) -> libc::c_int {
  let mut dev: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut mask: u32 = 0i32 as u32;
  let mut flags: u32 = 0i32 as u32;
  let mut qlen: libc::c_int = -1i32;
  let mut mtu: libc::c_int = -1i32;
  let mut master: libc::c_int = -1i32;
  let mut newaddr: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut newbrd: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut ifr0: ifreq = ifreq {
    ifr_ifrn: C2RustUnnamed_1 { ifrn_name: [0; 16] },
    ifr_ifru: C2RustUnnamed_0 {
      ifru_addr: sockaddr {
        sa_family: 0,
        sa_data: [0; 14],
      },
    },
  };
  let mut ifr1: ifreq = ifreq {
    ifr_ifrn: C2RustUnnamed_1 { ifrn_name: [0; 16] },
    ifr_ifru: C2RustUnnamed_0 {
      ifru_addr: sockaddr {
        sa_family: 0,
        sa_data: [0; 14],
      },
    },
  };
  let mut newname: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut htype: libc::c_int = 0;
  let mut halen: libc::c_int = 0;
  /* If you add stuff here, update iplink_full_usage */
  static mut keywords: [libc::c_char; 73] = [
    117, 112, 0, 100, 111, 119, 110, 0, 110, 97, 109, 101, 0, 109, 116, 117, 0, 113, 108, 101, 110,
    0, 109, 117, 108, 116, 105, 99, 97, 115, 116, 0, 97, 114, 112, 0, 112, 114, 111, 109, 105, 115,
    99, 0, 97, 100, 100, 114, 101, 115, 115, 0, 109, 97, 115, 116, 101, 114, 0, 110, 111, 109, 97,
    115, 116, 101, 114, 0, 100, 101, 118, 0, 0,
  ];
  let mut key: smalluint = 0;
  while !(*argv).is_null() {
    /* must be last */
    /* substring search ensures that e.g. "addr" and "address"
     * are both accepted */
    key = index_in_substrings(keywords.as_ptr(), *argv) as smalluint;
    if key as libc::c_int == ARG_up as libc::c_int {
      mask |= IFF_UP as libc::c_int as libc::c_uint;
      flags |= IFF_UP as libc::c_int as libc::c_uint
    } else if key as libc::c_int == ARG_down as libc::c_int {
      mask |= IFF_UP as libc::c_int as libc::c_uint;
      flags &= !(IFF_UP as libc::c_int) as libc::c_uint
    } else if key as libc::c_int == ARG_name as libc::c_int {
      argv = next_arg(argv);
      newname = *argv
    } else if key as libc::c_int == ARG_mtu as libc::c_int {
      argv = next_arg(argv);
      if mtu != -1i32 {
        duparg(b"mtu\x00" as *const u8 as *const libc::c_char, *argv);
      }
      mtu = get_unsigned(*argv, b"mtu\x00" as *const u8 as *const libc::c_char) as libc::c_int
    } else if key as libc::c_int == ARG_qlen as libc::c_int {
      //TODO: txqueuelen, txqlen are synonyms to qlen
      argv = next_arg(argv);
      if qlen != -1i32 {
        duparg(b"qlen\x00" as *const u8 as *const libc::c_char, *argv);
      }
      qlen = get_unsigned(*argv, b"qlen\x00" as *const u8 as *const libc::c_char) as libc::c_int
    } else if key as libc::c_int == ARG_addr as libc::c_int {
      argv = next_arg(argv);
      newaddr = *argv
    } else if key as libc::c_int == ARG_master as libc::c_int {
      argv = next_arg(argv);
      master = xll_name_to_index(*argv)
    } else if key as libc::c_int == ARG_nomaster as libc::c_int {
      master = 0i32
    } else if key as libc::c_int >= ARG_dev as libc::c_int {
      /* ^^^^^^ ">=" here results in "dev IFACE" treated as default */
      if key as libc::c_int == ARG_dev as libc::c_int {
        argv = next_arg(argv)
      }
      if !dev.is_null() {
        duparg2(b"dev\x00" as *const u8 as *const libc::c_char, *argv);
      }
      dev = *argv
    } else {
      /* "on|off" options */
      let mut param: libc::c_int = 0;
      argv = next_arg(argv);
      param = index_in_strings(
        b"on\x00off\x00\x00" as *const u8 as *const libc::c_char,
        *argv,
      );
      if key as libc::c_int == ARG_multicast as libc::c_int {
        if param < 0i32 {
          die_must_be_on_off(b"multicast\x00" as *const u8 as *const libc::c_char);
        }
        mask |= IFF_MULTICAST as libc::c_int as libc::c_uint;
        if param == PARM_on as libc::c_int {
          flags |= IFF_MULTICAST as libc::c_int as libc::c_uint
        } else {
          flags &= !(IFF_MULTICAST as libc::c_int) as libc::c_uint
        }
      } else if key as libc::c_int == ARG_arp as libc::c_int {
        if param < 0i32 {
          die_must_be_on_off(b"arp\x00" as *const u8 as *const libc::c_char);
        }
        mask |= IFF_NOARP as libc::c_int as libc::c_uint;
        if param == PARM_on as libc::c_int {
          flags &= !(IFF_NOARP as libc::c_int) as libc::c_uint
        } else {
          flags |= IFF_NOARP as libc::c_int as libc::c_uint
        }
      } else if key as libc::c_int == ARG_promisc as libc::c_int {
        if param < 0i32 {
          die_must_be_on_off(b"promisc\x00" as *const u8 as *const libc::c_char);
        }
        mask |= IFF_PROMISC as libc::c_int as libc::c_uint;
        if param == PARM_on as libc::c_int {
          flags |= IFF_PROMISC as libc::c_int as libc::c_uint
        } else {
          flags &= !(IFF_PROMISC as libc::c_int) as libc::c_uint
        }
      }
    }
    /* Other keywords recognized by iproute2-3.12.0: */
    argv = argv.offset(1)
  } /*if (arg == ARG_loose_binding) */
  if dev.is_null() {
    bb_error_msg_and_die(
      bb_msg_requires_arg.as_ptr(),
      b"\"dev\"\x00" as *const u8 as *const libc::c_char,
    );
  }
  if !newaddr.is_null() || !newbrd.is_null() {
    halen = get_address(dev, &mut htype);
    if !newaddr.is_null() {
      parse_address(dev, htype, halen, newaddr, &mut ifr0);
      set_address(&mut ifr0, 0i32);
    }
    if !newbrd.is_null() {
      parse_address(dev, htype, halen, newbrd, &mut ifr1);
      set_address(&mut ifr1, 1i32);
    }
  }
  if !newname.is_null() && strcmp(dev, newname) != 0 {
    do_changename(dev, newname);
    dev = newname
  }
  if qlen != -1i32 {
    set_qlen(dev, qlen);
  }
  if mtu != -1i32 {
    set_mtu(dev, mtu);
  }
  if master != -1i32 {
    set_master(dev, master);
  }
  if mask != 0 {
    do_chflags(dev, flags, mask);
  }
  return 0i32;
}
unsafe extern "C" fn ipaddr_list_link(mut argv: *mut *mut libc::c_char) -> libc::c_int {
  preferred_family = 17i32 as family_t;
  return ipaddr_list_or_flush(argv, 0i32);
}
unsafe extern "C" fn vlan_parse_opt(
  mut argv: *mut *mut libc::c_char,
  mut n: *mut nlmsghdr,
  mut size: libc::c_uint,
) {
  static mut keywords: [libc::c_char; 49] = [
    105, 100, 0, 112, 114, 111, 116, 111, 99, 111, 108, 0, 114, 101, 111, 114, 100, 101, 114, 95,
    104, 100, 114, 0, 103, 118, 114, 112, 0, 109, 118, 114, 112, 0, 108, 111, 111, 115, 101, 95,
    98, 105, 110, 100, 105, 110, 103, 0, 0,
  ];
  static mut protocols: [libc::c_char; 16] = [
    56, 48, 50, 46, 49, 113, 0, 56, 48, 50, 46, 49, 97, 100, 0, 0,
  ];
  let mut arg: libc::c_int = 0;
  let mut id: u16 = 0;
  let mut proto: u16 = 0;
  let mut flags: ifla_vlan_flags = {
    let mut init = ifla_vlan_flags { flags: 0, mask: 0 };
    init
  };
  while !(*argv).is_null() {
    arg = index_in_substrings(keywords.as_ptr(), *argv);
    if arg < 0i32 {
      invarg_1_to_2(*argv, b"type vlan\x00" as *const u8 as *const libc::c_char);
    }
    argv = next_arg(argv);
    if arg == ARG_id as libc::c_int {
      id = get_u16(*argv, b"id\x00" as *const u8 as *const libc::c_char);
      addattr_l(
        n,
        size as libc::c_int,
        IFLA_VLAN_ID as libc::c_int,
        &mut id as *mut u16 as *mut libc::c_void,
        ::std::mem::size_of::<u16>() as libc::c_ulong as libc::c_int,
      );
    } else if arg == ARG_protocol as libc::c_int {
      arg = index_in_substrings(protocols.as_ptr(), str_tolower(*argv));
      if arg == PROTO_8021Q as libc::c_int {
        proto = {
          let mut __v: libc::c_ushort = 0;
          let mut __x: libc::c_ushort = 0x8100i32 as libc::c_ushort;
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
        }
      } else if arg == PROTO_8021AD as libc::c_int {
        proto = {
          let mut __v: libc::c_ushort = 0;
          let mut __x: libc::c_ushort = 0x88a8i32 as libc::c_ushort;
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
        }
      } else {
        bb_error_msg_and_die(
          b"unknown VLAN encapsulation protocol \'%s\'\x00" as *const u8 as *const libc::c_char,
          *argv,
        );
      }
      addattr_l(
        n,
        size as libc::c_int,
        5i32,
        &mut proto as *mut u16 as *mut libc::c_void,
        ::std::mem::size_of::<u16>() as libc::c_ulong as libc::c_int,
      );
    } else {
      let mut param: libc::c_int = index_in_strings(
        b"on\x00off\x00\x00" as *const u8 as *const libc::c_char,
        *argv,
      );
      if param < 0i32 {
        die_must_be_on_off(nth_string(keywords.as_ptr(), arg));
      }
      if arg == ARG_reorder_hdr as libc::c_int {
        flags.mask |= 0x1i32 as libc::c_uint;
        flags.flags &= !0x1i32 as libc::c_uint;
        if param == PARM_on_0 as libc::c_int {
          flags.flags |= 0x1i32 as libc::c_uint
        }
      } else if arg == ARG_gvrp as libc::c_int {
        flags.mask |= 0x2i32 as libc::c_uint;
        flags.flags &= !0x2i32 as libc::c_uint;
        if param == PARM_on_0 as libc::c_int {
          flags.flags |= 0x2i32 as libc::c_uint
        }
      } else if arg == ARG_mvrp as libc::c_int {
        flags.mask |= 0x8i32 as libc::c_uint;
        flags.flags &= !0x8i32 as libc::c_uint;
        if param == PARM_on_0 as libc::c_int {
          flags.flags |= 0x8i32 as libc::c_uint
        }
      } else {
        flags.mask |= 0x4i32 as libc::c_uint;
        flags.flags &= !0x4i32 as libc::c_uint;
        if param == PARM_on_0 as libc::c_int {
          flags.flags |= 0x4i32 as libc::c_uint
        }
      }
    }
    argv = argv.offset(1)
  }
  if flags.mask != 0 {
    addattr_l(
      n,
      size as libc::c_int,
      IFLA_VLAN_FLAGS as libc::c_int,
      &mut flags as *mut ifla_vlan_flags as *mut libc::c_void,
      ::std::mem::size_of::<ifla_vlan_flags>() as libc::c_ulong as libc::c_int,
    );
  };
}
unsafe extern "C" fn vrf_parse_opt(
  mut argv: *mut *mut libc::c_char,
  mut n: *mut nlmsghdr,
  mut size: libc::c_uint,
) {
  /* IFLA_VRF_TABLE is an enum, not a define -
   * can't test "defined(IFLA_VRF_TABLE)".
   */
  let mut table: u32 = 0;
  if strcmp(*argv, b"table\x00" as *const u8 as *const libc::c_char) != 0i32 {
    invarg_1_to_2(*argv, b"type vrf\x00" as *const u8 as *const libc::c_char);
  }
  argv = next_arg(argv);
  table = get_u32(*argv, b"table\x00" as *const u8 as *const libc::c_char);
  addattr_l(
    n,
    size as libc::c_int,
    IFLA_VRF_TABLE as libc::c_int,
    &mut table as *mut u32 as *mut libc::c_void,
    ::std::mem::size_of::<u32>() as libc::c_ulong as libc::c_int,
  );
}
/* Return value becomes exitcode. It's okay to not return at all */
unsafe extern "C" fn do_add_or_delete(
  mut argv: *mut *mut libc::c_char,
  rtm: libc::c_uint,
) -> libc::c_int {
  static mut keywords: [libc::c_char; 28] = [
    108, 105, 110, 107, 0, 110, 97, 109, 101, 0, 116, 121, 112, 101, 0, 100, 101, 118, 0, 97, 100,
    100, 114, 101, 115, 115, 0, 0,
  ];
  let mut rth: rtnl_handle = rtnl_handle {
    fd: 0,
    local: sockaddr_nl {
      nl_family: 0,
      nl_pad: 0,
      nl_pid: 0,
      nl_groups: 0,
    },
    peer: sockaddr_nl {
      nl_family: 0,
      nl_pad: 0,
      nl_pid: 0,
      nl_groups: 0,
    },
    seq: 0,
    dump: 0,
  };
  let mut req: C2RustUnnamed_10 = C2RustUnnamed_10 {
    n: nlmsghdr {
      nlmsg_len: 0,
      nlmsg_type: 0,
      nlmsg_flags: 0,
      nlmsg_seq: 0,
      nlmsg_pid: 0,
    },
    i: ifinfomsg {
      ifi_family: 0,
      __ifi_pad: 0,
      ifi_type: 0,
      ifi_index: 0,
      ifi_flags: 0,
      ifi_change: 0,
    },
    buf: [0; 1024],
  };
  let mut arg: smalluint = 0;
  let mut name_str: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut link_str: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut type_str: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut dev_str: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut address_str: *mut libc::c_char = 0 as *mut libc::c_char;
  memset(
    &mut req as *mut C2RustUnnamed_10 as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<C2RustUnnamed_10>() as libc::c_ulong,
  );
  req.n.nlmsg_len = (::std::mem::size_of::<ifinfomsg>() as libc::c_ulong).wrapping_add(
    ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
      .wrapping_add(4u32 as libc::c_ulong)
      .wrapping_sub(1i32 as libc::c_ulong)
      & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong) as libc::c_int
      as libc::c_ulong,
  ) as u32;
  req.n.nlmsg_flags = 0x1i32 as __u16;
  req.n.nlmsg_type = rtm as __u16;
  req.i.ifi_family = preferred_family as libc::c_uchar;
  if rtm == RTM_NEWLINK as libc::c_int as libc::c_uint {
    req.n.nlmsg_flags = (req.n.nlmsg_flags as libc::c_int | (0x400i32 | 0x200i32)) as __u16
  }
  /* NB: update iplink_full_usage if you extend this code */
  while !(*argv).is_null() {
    arg = index_in_substrings(keywords.as_ptr(), *argv) as smalluint;
    if arg as libc::c_int == ARG_type as libc::c_int {
      argv = next_arg(argv);
      let fresh9 = argv;
      argv = argv.offset(1);
      type_str = *fresh9;
      break;
    } else {
      if arg as libc::c_int == ARG_link as libc::c_int {
        argv = next_arg(argv);
        link_str = *argv
      } else if arg as libc::c_int == ARG_name_0 as libc::c_int {
        argv = next_arg(argv);
        name_str = *argv
      } else if arg as libc::c_int == ARG_address as libc::c_int {
        argv = next_arg(argv);
        address_str = *argv
      } else {
        if arg as libc::c_int == ARG_dev_0 as libc::c_int {
          if !dev_str.is_null() {
            duparg(*argv, b"dev\x00" as *const u8 as *const libc::c_char);
          }
          argv = next_arg(argv)
        }
        dev_str = *argv
      }
      argv = argv.offset(1)
    }
  }
  xrtnl_open(&mut rth);
  ll_init_map(&mut rth);
  if !type_str.is_null() {
    let mut linkinfo: *mut rtattr = (&mut req.n as *mut nlmsghdr as *mut libc::c_void).offset(
      (req
        .n
        .nlmsg_len
        .wrapping_add(4u32)
        .wrapping_sub(1i32 as libc::c_uint)
        & !4u32.wrapping_sub(1i32 as libc::c_uint)) as isize,
    ) as *mut rtattr;
    addattr_l(
      &mut req.n,
      ::std::mem::size_of::<C2RustUnnamed_10>() as libc::c_ulong as libc::c_int,
      IFLA_LINKINFO as libc::c_int,
      0 as *mut libc::c_void,
      0i32,
    );
    addattr_l(
      &mut req.n,
      ::std::mem::size_of::<C2RustUnnamed_10>() as libc::c_ulong as libc::c_int,
      IFLA_INFO_KIND as libc::c_int,
      type_str as *mut libc::c_void,
      strlen(type_str) as libc::c_int,
    );
    if !(*argv).is_null() {
      let mut data: *mut rtattr = (&mut req.n as *mut nlmsghdr as *mut libc::c_void).offset(
        (req
          .n
          .nlmsg_len
          .wrapping_add(4u32)
          .wrapping_sub(1i32 as libc::c_uint)
          & !4u32.wrapping_sub(1i32 as libc::c_uint)) as isize,
      ) as *mut rtattr;
      addattr_l(
        &mut req.n,
        ::std::mem::size_of::<C2RustUnnamed_10>() as libc::c_ulong as libc::c_int,
        IFLA_INFO_DATA as libc::c_int,
        0 as *mut libc::c_void,
        0i32,
      );
      if strcmp(type_str, b"vlan\x00" as *const u8 as *const libc::c_char) == 0i32 {
        vlan_parse_opt(
          argv,
          &mut req.n,
          ::std::mem::size_of::<C2RustUnnamed_10>() as libc::c_ulong as libc::c_uint,
        );
      } else if strcmp(type_str, b"vrf\x00" as *const u8 as *const libc::c_char) == 0i32 {
        vrf_parse_opt(
          argv,
          &mut req.n,
          ::std::mem::size_of::<C2RustUnnamed_10>() as libc::c_ulong as libc::c_uint,
        );
      }
      (*data).rta_len = ((&mut req.n as *mut nlmsghdr as *mut libc::c_void).offset(
        (req
          .n
          .nlmsg_len
          .wrapping_add(4u32)
          .wrapping_sub(1i32 as libc::c_uint)
          & !4u32.wrapping_sub(1i32 as libc::c_uint)) as isize,
      ) as *mut rtattr as *mut libc::c_void)
        .wrapping_offset_from(data as *mut libc::c_void) as libc::c_long
        as libc::c_ushort
    }
    (*linkinfo).rta_len = ((&mut req.n as *mut nlmsghdr as *mut libc::c_void).offset(
      (req
        .n
        .nlmsg_len
        .wrapping_add(4u32)
        .wrapping_sub(1i32 as libc::c_uint)
        & !4u32.wrapping_sub(1i32 as libc::c_uint)) as isize,
    ) as *mut rtattr as *mut libc::c_void)
      .wrapping_offset_from(linkinfo as *mut libc::c_void) as libc::c_long
      as libc::c_ushort
  }
  /* Allow "ip link add dev" and "ip link add name" */
  if name_str.is_null() {
    name_str = dev_str
  } else if dev_str.is_null() {
    dev_str = name_str
  }
  /* else if (!strcmp(name_str, dev_str))
  name_str = dev_str; */
  if rtm != RTM_NEWLINK as libc::c_int as libc::c_uint {
    if dev_str.is_null() {
      return 1i32;
    } /* Need a device to delete */
    req.i.ifi_index = xll_name_to_index(dev_str)
  } else {
    if !link_str.is_null() {
      let mut idx: libc::c_int = xll_name_to_index(link_str);
      addattr_l(
        &mut req.n,
        ::std::mem::size_of::<C2RustUnnamed_10>() as libc::c_ulong as libc::c_int,
        IFLA_LINK as libc::c_int,
        &mut idx as *mut libc::c_int as *mut libc::c_void,
        4i32,
      );
    }
    if !address_str.is_null() {
      let mut abuf: [libc::c_uchar; 32] = [0; 32];
      let mut len: libc::c_int = ll_addr_a2n(
        abuf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_uchar; 32]>() as libc::c_ulong as libc::c_int,
        address_str,
      );
      if len < 0i32 {
        return -1i32;
      }
      addattr_l(
        &mut req.n,
        ::std::mem::size_of::<C2RustUnnamed_10>() as libc::c_ulong as libc::c_int,
        IFLA_ADDRESS as libc::c_int,
        abuf.as_mut_ptr() as *mut libc::c_void,
        len,
      );
    }
  }
  if !name_str.is_null() {
    let name_len: size_t = strlen(name_str).wrapping_add(1i32 as libc::c_ulong);
    if name_len < 2i32 as libc::c_ulong || name_len > 16i32 as libc::c_ulong {
      invarg_1_to_2(name_str, b"name\x00" as *const u8 as *const libc::c_char);
    }
    addattr_l(
      &mut req.n,
      ::std::mem::size_of::<C2RustUnnamed_10>() as libc::c_ulong as libc::c_int,
      IFLA_IFNAME as libc::c_int,
      name_str as *mut libc::c_void,
      name_len as libc::c_int,
    );
  }
  if rtnl_talk(&mut rth, &mut req.n, 0 as *mut nlmsghdr) < 0i32 {
    return 2i32;
  }
  return 0i32;
}

//int FAST_FUNC print_neigh(struct sockaddr_nl *who, struct nlmsghdr *n, void *arg);
//int FAST_FUNC iproute_monitor(char **argv);
//void FAST_FUNC ipneigh_reset_filter(void);
/* Return value becomes exitcode. It's okay to not return at all */
#[no_mangle]
pub unsafe extern "C" fn do_iplink(mut argv: *mut *mut libc::c_char) -> libc::c_int {
  static mut keywords: [libc::c_char; 30] = [
    97, 100, 100, 0, 100, 101, 108, 101, 116, 101, 0, 115, 101, 116, 0, 115, 104, 111, 119, 0, 108,
    115, 116, 0, 108, 105, 115, 116, 0, 0,
  ]; //TODO: move up to "ip"? Is it the common rule for all "ip" tools?
  xfunc_error_retval = 2i32 as u8;
  if !(*argv).is_null() {
    let mut key: libc::c_int = index_in_substrings(keywords.as_ptr(), *argv);
    if key < 0i32 {
      /* invalid argument */
      invarg_1_to_2(*argv, applet_name);
    }
    argv = argv.offset(1);
    if key <= 1i32 {
      /* add/delete */
      return do_add_or_delete(
        argv,
        if key != 0 {
          RTM_DELLINK as libc::c_int
        } else {
          RTM_NEWLINK as libc::c_int
        } as libc::c_uint,
      );
    }
    if key == 2i32 {
      /* set */
      return do_set(argv);
    }
  }
  /* show, lst, list */
  return ipaddr_list_link(argv);
}
