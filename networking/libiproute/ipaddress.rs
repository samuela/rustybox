use c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use libc;
extern "C" {
  #[no_mangle]
  fn fnmatch(
    __pattern: *const libc::c_char,
    __name: *const libc::c_char,
    __flags: libc::c_int,
  ) -> libc::c_int;
  #[no_mangle]
  fn socket(__domain: libc::c_int, __type: libc::c_int, __protocol: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  static mut stdout: *mut FILE;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn is_prefixed_with(string: *const libc::c_char, key: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn strncpy_IFNAMSIZ(dst: *mut libc::c_char, src: *const libc::c_char) -> *mut libc::c_char;
  /* Guaranteed to NOT be a macro (smallest code). Saves nearly 2k on uclibc.
   * But potentially slow, don't use in one-billion-times loops */
  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_simple_error_msg(s: *const libc::c_char);
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);
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
  fn print_flags_separated(
    masks: *const libc::c_int,
    labels: *const libc::c_char,
    flags: libc::c_int,
    separator: *const libc::c_char,
  ) -> libc::c_int;
  #[no_mangle]
  static mut applet_name: *const libc::c_char;
  #[no_mangle]
  static bb_msg_requires_arg: [libc::c_char; 0];
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
  #[no_mangle]
  fn rtnl_rtscope_n2a(id: libc::c_int) -> *const libc::c_char;
  #[no_mangle]
  fn rtnl_rtscope_a2n(id: *mut uint32_t, arg: *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn ll_type_n2a(type_0: libc::c_int, buf: *mut libc::c_char) -> *const libc::c_char;
  #[no_mangle]
  fn ll_addr_n2a(
    addr: *mut libc::c_uchar,
    alen: libc::c_int,
    type_0: libc::c_int,
    buf: *mut libc::c_char,
    blen: libc::c_int,
  ) -> *const libc::c_char;

  #[no_mangle]
  static mut preferred_family: family_t;
  /* UNUSED */
  #[no_mangle]
  static mut oneline: smallint;
  #[no_mangle]
  static mut _SL_: libc::c_char;
  #[no_mangle]
  fn next_arg(argv: *mut *mut libc::c_char) -> *mut *mut libc::c_char;
  /*void get_prefix_1(inet_prefix *dst, char *arg, int family) FAST_FUNC;*/
  #[no_mangle]
  fn get_addr(dst: *mut inet_prefix, arg: *mut libc::c_char, family: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn get_prefix(dst: *mut inet_prefix, arg: *mut libc::c_char, family: libc::c_int);
  #[no_mangle]
  fn rt_addr_n2a(af: libc::c_int, addr: *mut libc::c_void) -> *const libc::c_char;

  #[no_mangle]
  fn ll_init_map(rth: *mut rtnl_handle) -> libc::c_int;

  /* We need linux/types.h because older kernels use __u32 etc
   * in linux/[rt]netlink.h. 2.6.19 seems to be ok, though */
  /* bbox doesn't use parameters no. 3, 4, 6, 7, stub them out */
  //TODO: pass rth->fd instead of full rth?
  // Used to be:
  //struct sockaddr_nl nladdr;
  //memset(&nladdr, 0, sizeof(nladdr));
  //nladdr.nl_family = AF_NETLINK;
  //return xsendto(rth->fd, buf, len, (struct sockaddr*)&nladdr, sizeof(nladdr));
  // iproute2-4.2.0 simplified the above to:
  //return send(rth->fd, buf, len, 0);
  // We are using even shorter:
  // and convert to void, inline.
  #[no_mangle]
  fn parse_rtattr(tb: *mut *mut rtattr, max: libc::c_int, rta: *mut rtattr, len: libc::c_int);
  #[no_mangle]
  fn addattr_l(
    n: *mut nlmsghdr,
    maxlen: libc::c_int,
    type_0: libc::c_int,
    data: *mut libc::c_void,
    alen: libc::c_int,
  ) -> libc::c_int;
  #[no_mangle]
  fn inet_addr_match(
    a: *const inet_prefix,
    b: *const inet_prefix,
    bits: libc::c_int,
  ) -> libc::c_int;
  #[no_mangle]
  fn rtnl_send_check(
    rth: *mut rtnl_handle,
    buf: *const libc::c_void,
    len: libc::c_int,
  ) -> libc::c_int;
  #[no_mangle]
  fn duparg(_: *const libc::c_char, _: *const libc::c_char) -> !;
  #[no_mangle]
  fn rtnl_talk(rtnl: *mut rtnl_handle, n: *mut nlmsghdr, answer: *mut nlmsghdr) -> libc::c_int;
  #[no_mangle]
  fn ll_remember_index(
    who: *const sockaddr_nl,
    n: *mut nlmsghdr,
    arg: *mut libc::c_void,
  ) -> libc::c_int;
  #[no_mangle]
  fn xrtnl_dump_filter(
    rth: *mut rtnl_handle,
    filter: Option<
      unsafe extern "C" fn(
        _: *const sockaddr_nl,
        _: *mut nlmsghdr,
        _: *mut libc::c_void,
      ) -> libc::c_int,
    >,
    arg1: *mut libc::c_void,
  ) -> libc::c_int;
  #[no_mangle]
  fn invarg_1_to_2(_: *const libc::c_char, _: *const libc::c_char) -> !;
  #[no_mangle]
  fn duparg2(_: *const libc::c_char, _: *const libc::c_char) -> !;
  #[no_mangle]
  fn xrtnl_open(rth: *mut rtnl_handle);
  #[no_mangle]
  fn xrtnl_wilddump_request(rth: *mut rtnl_handle, fam: libc::c_int, type_0: libc::c_int);
  #[no_mangle]
  fn xll_name_to_index(name: *const libc::c_char) -> libc::c_int;
  //static: const char *ll_idx_n2a(int idx, char *buf) FAST_FUNC;
  #[no_mangle]
  fn ll_index_to_name(idx: libc::c_int) -> *const libc::c_char;
  /* int ll_index_to_type(int idx); */
  #[no_mangle]
  fn ll_index_to_flags(idx: libc::c_int) -> libc::c_uint;
}

pub type __caddr_t = *mut libc::c_char;
use crate::librb::int16_t;
use crate::librb::int8_t;
use crate::librb::size_t;
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
use libc::uint32_t;
 use libc::uint8_t;
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
use crate::librb::smallint;
pub type smalluint = libc::c_uchar;

use libc::FILE;
pub type family_t = int8_t;
pub type __u8 = libc::c_uchar;
pub type __u16 = libc::c_ushort;
pub type __u32 = libc::c_uint;
pub type __kernel_sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_nl {
  pub nl_family: __kernel_sa_family_t,
  pub nl_pad: libc::c_ushort,
  pub nl_pid: __u32,
  pub nl_groups: __u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nlmsghdr {
  pub nlmsg_len: __u32,
  pub nlmsg_type: __u16,
  pub nlmsg_flags: __u16,
  pub nlmsg_seq: __u32,
  pub nlmsg_pid: __u32,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifaddrmsg {
  pub ifa_family: __u8,
  pub ifa_prefixlen: __u8,
  pub ifa_flags: __u8,
  pub ifa_scope: __u8,
  pub ifa_index: __u32,
}
pub type C2RustUnnamed_3 = libc::c_uint;
pub const __IFA_MAX: C2RustUnnamed_3 = 9;
pub const IFA_FLAGS: C2RustUnnamed_3 = 8;
pub const IFA_MULTICAST: C2RustUnnamed_3 = 7;
pub const IFA_CACHEINFO: C2RustUnnamed_3 = 6;
pub const IFA_ANYCAST: C2RustUnnamed_3 = 5;
pub const IFA_BROADCAST: C2RustUnnamed_3 = 4;
pub const IFA_LABEL: C2RustUnnamed_3 = 3;
pub const IFA_LOCAL: C2RustUnnamed_3 = 2;
pub const IFA_ADDRESS: C2RustUnnamed_3 = 1;
pub const IFA_UNSPEC: C2RustUnnamed_3 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifa_cacheinfo {
  pub ifa_prefered: __u32,
  pub ifa_valid: __u32,
  pub cstamp: __u32,
  pub tstamp: __u32,
}
pub type C2RustUnnamed_4 = libc::c_uint;
pub const __RTM_MAX: C2RustUnnamed_4 = 97;
pub const RTM_NEWCACHEREPORT: C2RustUnnamed_4 = 96;
pub const RTM_GETSTATS: C2RustUnnamed_4 = 94;
pub const RTM_NEWSTATS: C2RustUnnamed_4 = 92;
pub const RTM_GETNSID: C2RustUnnamed_4 = 90;
pub const RTM_DELNSID: C2RustUnnamed_4 = 89;
pub const RTM_NEWNSID: C2RustUnnamed_4 = 88;
pub const RTM_GETMDB: C2RustUnnamed_4 = 86;
pub const RTM_DELMDB: C2RustUnnamed_4 = 85;
pub const RTM_NEWMDB: C2RustUnnamed_4 = 84;
pub const RTM_GETNETCONF: C2RustUnnamed_4 = 82;
pub const RTM_DELNETCONF: C2RustUnnamed_4 = 81;
pub const RTM_NEWNETCONF: C2RustUnnamed_4 = 80;
pub const RTM_SETDCB: C2RustUnnamed_4 = 79;
pub const RTM_GETDCB: C2RustUnnamed_4 = 78;
pub const RTM_GETADDRLABEL: C2RustUnnamed_4 = 74;
pub const RTM_DELADDRLABEL: C2RustUnnamed_4 = 73;
pub const RTM_NEWADDRLABEL: C2RustUnnamed_4 = 72;
pub const RTM_NEWNDUSEROPT: C2RustUnnamed_4 = 68;
pub const RTM_SETNEIGHTBL: C2RustUnnamed_4 = 67;
pub const RTM_GETNEIGHTBL: C2RustUnnamed_4 = 66;
pub const RTM_NEWNEIGHTBL: C2RustUnnamed_4 = 64;
pub const RTM_GETANYCAST: C2RustUnnamed_4 = 62;
pub const RTM_GETMULTICAST: C2RustUnnamed_4 = 58;
pub const RTM_NEWPREFIX: C2RustUnnamed_4 = 52;
pub const RTM_GETACTION: C2RustUnnamed_4 = 50;
pub const RTM_DELACTION: C2RustUnnamed_4 = 49;
pub const RTM_NEWACTION: C2RustUnnamed_4 = 48;
pub const RTM_GETTFILTER: C2RustUnnamed_4 = 46;
pub const RTM_DELTFILTER: C2RustUnnamed_4 = 45;
pub const RTM_NEWTFILTER: C2RustUnnamed_4 = 44;
pub const RTM_GETTCLASS: C2RustUnnamed_4 = 42;
pub const RTM_DELTCLASS: C2RustUnnamed_4 = 41;
pub const RTM_NEWTCLASS: C2RustUnnamed_4 = 40;
pub const RTM_GETQDISC: C2RustUnnamed_4 = 38;
pub const RTM_DELQDISC: C2RustUnnamed_4 = 37;
pub const RTM_NEWQDISC: C2RustUnnamed_4 = 36;
pub const RTM_GETRULE: C2RustUnnamed_4 = 34;
pub const RTM_DELRULE: C2RustUnnamed_4 = 33;
pub const RTM_NEWRULE: C2RustUnnamed_4 = 32;
pub const RTM_GETNEIGH: C2RustUnnamed_4 = 30;
pub const RTM_DELNEIGH: C2RustUnnamed_4 = 29;
pub const RTM_NEWNEIGH: C2RustUnnamed_4 = 28;
pub const RTM_GETROUTE: C2RustUnnamed_4 = 26;
pub const RTM_DELROUTE: C2RustUnnamed_4 = 25;
pub const RTM_NEWROUTE: C2RustUnnamed_4 = 24;
pub const RTM_GETADDR: C2RustUnnamed_4 = 22;
pub const RTM_DELADDR: C2RustUnnamed_4 = 21;
pub const RTM_NEWADDR: C2RustUnnamed_4 = 20;
pub const RTM_SETLINK: C2RustUnnamed_4 = 19;
pub const RTM_GETLINK: C2RustUnnamed_4 = 18;
pub const RTM_DELLINK: C2RustUnnamed_4 = 17;
pub const RTM_NEWLINK: C2RustUnnamed_4 = 16;
pub const RTM_BASE: C2RustUnnamed_4 = 16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rtattr {
  pub rta_len: libc::c_ushort,
  pub rta_type: libc::c_ushort,
}
pub type rt_scope_t = libc::c_uint;
pub const RT_SCOPE_NOWHERE: rt_scope_t = 255;
pub const RT_SCOPE_HOST: rt_scope_t = 254;
pub const RT_SCOPE_LINK: rt_scope_t = 253;
pub const RT_SCOPE_SITE: rt_scope_t = 200;
pub const RT_SCOPE_UNIVERSE: rt_scope_t = 0;
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
pub struct nlmsg_list {
  pub next: *mut nlmsg_list,
  pub h: nlmsghdr,
}
/* driver signals L1 up */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct filter_t {
  pub label: *mut libc::c_char,
  pub flushb: *mut libc::c_char,
  pub rth: *mut rtnl_handle,
  pub scope: libc::c_int,
  pub scopemask: libc::c_int,
  pub flags: libc::c_int,
  pub flagmask: libc::c_int,
  pub flushp: libc::c_int,
  pub flushe: libc::c_int,
  pub ifindex: libc::c_int,
  pub family: family_t,
  pub showqueue: smallint,
  pub oneline: smallint,
  pub up: smallint,
  pub flushed: smallint,
  pub pfx: inet_prefix,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct inet_prefix {
  pub family: uint8_t,
  pub bytelen: uint8_t,
  pub bitlen: int16_t,
  pub data: [uint32_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rtnl_handle {
  pub fd: libc::c_int,
  pub local: sockaddr_nl,
  pub peer: sockaddr_nl,
  pub seq: uint32_t,
  pub dump: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
  pub n: nlmsghdr,
  pub ifa: ifaddrmsg,
  pub buf: [libc::c_char; 256],
}
unsafe extern "C" fn print_link_flags(mut flags: libc::c_uint, mut mdown: libc::c_uint) {
  static mut flag_masks: [libc::c_int; 7] = [
    IFF_LOOPBACK as libc::c_int,
    IFF_BROADCAST as libc::c_int,
    IFF_POINTOPOINT as libc::c_int,
    IFF_MULTICAST as libc::c_int,
    IFF_NOARP as libc::c_int,
    IFF_UP as libc::c_int,
    0x10000i32,
  ];
  static mut flag_labels: [libc::c_char; 60] = [
    76, 79, 79, 80, 66, 65, 67, 75, 0, 66, 82, 79, 65, 68, 67, 65, 83, 84, 0, 80, 79, 73, 78, 84,
    79, 80, 79, 73, 78, 84, 0, 77, 85, 76, 84, 73, 67, 65, 83, 84, 0, 78, 79, 65, 82, 80, 0, 85,
    80, 0, 76, 79, 87, 69, 82, 95, 85, 80, 0, 0,
  ];
  bb_putchar('<' as i32);
  if flags & IFF_UP as libc::c_int as libc::c_uint != 0
    && flags & IFF_RUNNING as libc::c_int as libc::c_uint == 0
  {
    printf(b"NO-CARRIER,\x00" as *const u8 as *const libc::c_char);
  }
  flags &= !(IFF_RUNNING as libc::c_int) as libc::c_uint;
  flags = print_flags_separated(
    flag_masks.as_ptr(),
    flag_labels.as_ptr(),
    flags as libc::c_int,
    b",\x00" as *const u8 as *const libc::c_char,
  ) as libc::c_uint;
  if flags != 0 {
    printf(b"%x\x00" as *const u8 as *const libc::c_char, flags);
  }
  if mdown != 0 {
    printf(b",M-DOWN\x00" as *const u8 as *const libc::c_char);
  }
  printf(b"> \x00" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn print_queuelen(mut name: *mut libc::c_char) {
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
  s = socket(2i32, SOCK_STREAM as libc::c_int, 0i32);
  if s < 0i32 {
    return;
  }
  memset(
    &mut ifr as *mut ifreq as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<ifreq>() as libc::c_ulong,
  );
  strncpy_IFNAMSIZ(ifr.ifr_ifrn.ifrn_name.as_mut_ptr(), name);
  if bb_ioctl_or_warn(
    s,
    0x8942i32 as libc::c_uint,
    &mut ifr as *mut ifreq as *mut libc::c_void,
    b"SIOCGIFTXQLEN\x00" as *const u8 as *const libc::c_char,
  ) < 0i32
  {
    close(s);
    return;
  }
  close(s);
  if ifr.ifr_ifru.ifru_ivalue != 0 {
    printf(
      b"qlen %d\x00" as *const u8 as *const libc::c_char,
      ifr.ifr_ifru.ifru_ivalue,
    );
  };
}
#[inline(never)]
unsafe extern "C" fn print_linkinfo(mut n: *const nlmsghdr) -> libc::c_int {
  let mut ifi: *mut ifinfomsg = (n as *mut libc::c_char).offset(
    (0i32
      + ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
        .wrapping_add(4u32 as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong)
        & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong) as libc::c_int) as isize,
  ) as *mut libc::c_void as *mut ifinfomsg;
  let mut tb: [*mut rtattr; 50] = [0 as *mut rtattr; 50];
  let mut len: libc::c_int = (*n).nlmsg_len as libc::c_int;
  if (*n).nlmsg_type as libc::c_int != RTM_NEWLINK as libc::c_int
    && (*n).nlmsg_type as libc::c_int != RTM_DELLINK as libc::c_int
  {
    return 0i32;
  }
  len = (len as libc::c_ulong).wrapping_sub(
    (::std::mem::size_of::<ifinfomsg>() as libc::c_ulong).wrapping_add(
      ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
        .wrapping_add(4u32 as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong)
        & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong) as libc::c_int
        as libc::c_ulong,
    ),
  ) as libc::c_int as libc::c_int;
  if len < 0i32 {
    return -1i32;
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).ifindex != 0
    && (*ifi).ifi_index != (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).ifindex
  {
    return 0i32;
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).up as libc::c_int != 0
    && (*ifi).ifi_flags & IFF_UP as libc::c_int as libc::c_uint == 0
  {
    return 0i32;
  }
  //memset(tb, 0, sizeof(tb)); - parse_rtattr does this
  parse_rtattr(
    tb.as_mut_ptr(),
    __IFLA_MAX as libc::c_int - 1i32,
    (ifi as *mut libc::c_char).offset(
      ((::std::mem::size_of::<ifinfomsg>() as libc::c_ulong)
        .wrapping_add(4u32 as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong)
        & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong) as isize,
    ) as *mut rtattr,
    len,
  );
  if tb[IFLA_IFNAME as libc::c_int as usize].is_null() {
    bb_simple_error_msg(b"nil ifname\x00" as *const u8 as *const libc::c_char);
    return -1i32;
  }
  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t))
    .label
    .is_null()
    && ((*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).family == 0
      || (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).family as libc::c_int == 17i32)
    && fnmatch(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).label,
      (tb[IFLA_IFNAME as libc::c_int as usize] as *mut libc::c_char).offset(
        ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
          .wrapping_add(4u32 as libc::c_ulong)
          .wrapping_sub(1i32 as libc::c_ulong)
          & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
          .wrapping_add(0i32 as libc::c_ulong) as isize,
      ) as *mut libc::c_void as *const libc::c_char,
      0i32,
    ) != 0
  {
    return 0i32;
  }
  if (*n).nlmsg_type as libc::c_int == RTM_DELLINK as libc::c_int {
    printf(b"Deleted \x00" as *const u8 as *const libc::c_char);
  }
  printf(
    b"%d: %s\x00" as *const u8 as *const libc::c_char,
    (*ifi).ifi_index,
    (tb[IFLA_IFNAME as libc::c_int as usize] as *mut libc::c_char).offset(
      ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
        .wrapping_add(4u32 as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong)
        & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
        .wrapping_add(0i32 as libc::c_ulong) as isize,
    ) as *mut libc::c_void as *mut libc::c_char,
  );
  let mut m_flag: libc::c_uint = 0i32 as libc::c_uint;
  if !tb[IFLA_LINK as libc::c_int as usize].is_null() {
    let mut iflink: libc::c_int = *((tb[IFLA_LINK as libc::c_int as usize] as *mut libc::c_char)
      .offset(
        ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
          .wrapping_add(4u32 as libc::c_ulong)
          .wrapping_sub(1i32 as libc::c_ulong)
          & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
          .wrapping_add(0i32 as libc::c_ulong) as isize,
      ) as *mut libc::c_void as *mut libc::c_int);
    if iflink == 0i32 {
      printf(b"@NONE: \x00" as *const u8 as *const libc::c_char);
    } else {
      printf(
        b"@%s: \x00" as *const u8 as *const libc::c_char,
        ll_index_to_name(iflink),
      );
      m_flag = ll_index_to_flags(iflink);
      m_flag = (m_flag & IFF_UP as libc::c_int as libc::c_uint == 0) as libc::c_int as libc::c_uint
    }
  } else {
    printf(b": \x00" as *const u8 as *const libc::c_char);
  }
  print_link_flags((*ifi).ifi_flags, m_flag);
  if !tb[IFLA_MTU as libc::c_int as usize].is_null() {
    printf(
      b"mtu %u \x00" as *const u8 as *const libc::c_char,
      *((tb[IFLA_MTU as libc::c_int as usize] as *mut libc::c_char).offset(
        ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
          .wrapping_add(4u32 as libc::c_ulong)
          .wrapping_sub(1i32 as libc::c_ulong)
          & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
          .wrapping_add(0i32 as libc::c_ulong) as isize,
      ) as *mut libc::c_void as *mut libc::c_int),
    );
  }
  if !tb[IFLA_QDISC as libc::c_int as usize].is_null() {
    printf(
      b"qdisc %s \x00" as *const u8 as *const libc::c_char,
      (tb[IFLA_QDISC as libc::c_int as usize] as *mut libc::c_char).offset(
        ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
          .wrapping_add(4u32 as libc::c_ulong)
          .wrapping_sub(1i32 as libc::c_ulong)
          & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
          .wrapping_add(0i32 as libc::c_ulong) as isize,
      ) as *mut libc::c_void as *mut libc::c_char,
    );
  }
  if !tb[IFLA_MASTER as libc::c_int as usize].is_null() {
    printf(
      b"master %s \x00" as *const u8 as *const libc::c_char,
      ll_index_to_name(
        *((tb[IFLA_MASTER as libc::c_int as usize] as *mut libc::c_char).offset(
          ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
            .wrapping_add(4u32 as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong)
            & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
            .wrapping_add(0i32 as libc::c_ulong) as isize,
        ) as *mut libc::c_void as *mut libc::c_int),
      ),
    );
  }
  /* IFLA_OPERSTATE was added to kernel with the same commit as IFF_DORMANT */
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).showqueue != 0 {
    print_queuelen(
      (tb[IFLA_IFNAME as libc::c_int as usize] as *mut libc::c_char).offset(
        ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
          .wrapping_add(4u32 as libc::c_ulong)
          .wrapping_sub(1i32 as libc::c_ulong)
          & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
          .wrapping_add(0i32 as libc::c_ulong) as isize,
      ) as *mut libc::c_void as *mut libc::c_char,
    );
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).family == 0
    || (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).family as libc::c_int == 17i32
  {
    let mut b1: [libc::c_char; 64] = [0; 64];
    printf(
      b"%c    link/%s \x00" as *const u8 as *const libc::c_char,
      _SL_ as libc::c_int,
      ll_type_n2a((*ifi).ifi_type as libc::c_int, b1.as_mut_ptr()),
    );
    if !tb[IFLA_ADDRESS as libc::c_int as usize].is_null() {
      fputs_unlocked(
        ll_addr_n2a(
          (tb[IFLA_ADDRESS as libc::c_int as usize] as *mut libc::c_char).offset(
            ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
              .wrapping_add(4u32 as libc::c_ulong)
              .wrapping_sub(1i32 as libc::c_ulong)
              & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
              .wrapping_add(0i32 as libc::c_ulong) as isize,
          ) as *mut libc::c_void as *mut libc::c_uchar,
          ((*tb[IFLA_ADDRESS as libc::c_int as usize]).rta_len as libc::c_int as libc::c_ulong)
            .wrapping_sub(
              ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
                .wrapping_add(4u32 as libc::c_ulong)
                .wrapping_sub(1i32 as libc::c_ulong)
                & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
                .wrapping_add(0i32 as libc::c_ulong),
            ) as libc::c_int,
          (*ifi).ifi_type as libc::c_int,
          b1.as_mut_ptr(),
          ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        ),
        stdout,
      );
    }
    if !tb[IFLA_BROADCAST as libc::c_int as usize].is_null() {
      if (*ifi).ifi_flags & IFF_POINTOPOINT as libc::c_int as libc::c_uint != 0 {
        printf(b" peer \x00" as *const u8 as *const libc::c_char);
      } else {
        printf(b" brd \x00" as *const u8 as *const libc::c_char);
      }
      fputs_unlocked(
        ll_addr_n2a(
          (tb[IFLA_BROADCAST as libc::c_int as usize] as *mut libc::c_char).offset(
            ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
              .wrapping_add(4u32 as libc::c_ulong)
              .wrapping_sub(1i32 as libc::c_ulong)
              & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
              .wrapping_add(0i32 as libc::c_ulong) as isize,
          ) as *mut libc::c_void as *mut libc::c_uchar,
          ((*tb[IFLA_BROADCAST as libc::c_int as usize]).rta_len as libc::c_int as libc::c_ulong)
            .wrapping_sub(
              ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
                .wrapping_add(4u32 as libc::c_ulong)
                .wrapping_sub(1i32 as libc::c_ulong)
                & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
                .wrapping_add(0i32 as libc::c_ulong),
            ) as libc::c_int,
          (*ifi).ifi_type as libc::c_int,
          b1.as_mut_ptr(),
          ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        ),
        stdout,
      );
    }
  }
  bb_putchar('\n' as i32);
  /*fflush_all();*/
  return 0i32;
}
unsafe extern "C" fn flush_update() -> libc::c_int {
  if rtnl_send_check(
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).rth,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).flushb as *const libc::c_void,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).flushp,
  ) < 0i32
  {
    bb_simple_perror_msg(b"can\'t send flush request\x00" as *const u8 as *const libc::c_char);
    return -1i32;
  }
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).flushp = 0i32;
  return 0i32;
}
unsafe extern "C" fn print_addrinfo(
  mut _who: *const sockaddr_nl,
  mut n: *mut nlmsghdr,
  mut _arg: *mut libc::c_void,
) -> libc::c_int {
  let mut ifa: *mut ifaddrmsg = (n as *mut libc::c_char).offset(
    (0i32
      + ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
        .wrapping_add(4u32 as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong)
        & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong) as libc::c_int) as isize,
  ) as *mut libc::c_void as *mut ifaddrmsg;
  let mut len: libc::c_int = (*n).nlmsg_len as libc::c_int;
  let mut rta_tb: [*mut rtattr; 9] = [0 as *mut rtattr; 9];
  if (*n).nlmsg_type as libc::c_int != RTM_NEWADDR as libc::c_int
    && (*n).nlmsg_type as libc::c_int != RTM_DELADDR as libc::c_int
  {
    return 0i32;
  }
  len = (len as libc::c_ulong).wrapping_sub(
    (::std::mem::size_of::<ifaddrmsg>() as libc::c_ulong).wrapping_add(
      ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
        .wrapping_add(4u32 as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong)
        & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong) as libc::c_int
        as libc::c_ulong,
    ),
  ) as libc::c_int as libc::c_int;
  if len < 0i32 {
    bb_error_msg(
      b"wrong nlmsg len %d\x00" as *const u8 as *const libc::c_char,
      len,
    );
    return -1i32;
  }
  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t))
    .flushb
    .is_null()
    && (*n).nlmsg_type as libc::c_int != RTM_NEWADDR as libc::c_int
  {
    return 0i32;
  }
  //memset(rta_tb, 0, sizeof(rta_tb)); - parse_rtattr does this
  parse_rtattr(
    rta_tb.as_mut_ptr(),
    __IFA_MAX as libc::c_int - 1i32,
    (ifa as *mut libc::c_char).offset(
      ((::std::mem::size_of::<ifaddrmsg>() as libc::c_ulong)
        .wrapping_add(4u32 as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong)
        & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong) as isize,
    ) as *mut rtattr,
    ((*n).nlmsg_len as libc::c_ulong).wrapping_sub(
      (::std::mem::size_of::<ifaddrmsg>() as libc::c_ulong).wrapping_add(
        ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
          .wrapping_add(4u32 as libc::c_ulong)
          .wrapping_sub(1i32 as libc::c_ulong)
          & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong) as libc::c_int
          as libc::c_ulong,
      ),
    ) as libc::c_int,
  );
  if rta_tb[IFA_LOCAL as libc::c_int as usize].is_null() {
    rta_tb[IFA_LOCAL as libc::c_int as usize] = rta_tb[IFA_ADDRESS as libc::c_int as usize]
  }
  if rta_tb[IFA_ADDRESS as libc::c_int as usize].is_null() {
    rta_tb[IFA_ADDRESS as libc::c_int as usize] = rta_tb[IFA_LOCAL as libc::c_int as usize]
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).ifindex != 0
    && (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).ifindex as libc::c_uint
      != (*ifa).ifa_index
  {
    return 0i32;
  }
  if ((*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).scope ^ (*ifa).ifa_scope as libc::c_int)
    & (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).scopemask
    != 0
  {
    return 0i32;
  }
  if ((*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).flags ^ (*ifa).ifa_flags as libc::c_int)
    & (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).flagmask
    != 0
  {
    return 0i32;
  }
  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t))
    .label
    .is_null()
  {
    let mut label: *const libc::c_char = 0 as *const libc::c_char;
    if !rta_tb[IFA_LABEL as libc::c_int as usize].is_null() {
      label = (rta_tb[IFA_LABEL as libc::c_int as usize] as *mut libc::c_char).offset(
        ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
          .wrapping_add(4u32 as libc::c_ulong)
          .wrapping_sub(1i32 as libc::c_ulong)
          & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
          .wrapping_add(0i32 as libc::c_ulong) as isize,
      ) as *mut libc::c_void as *const libc::c_char
    } else {
      label = ll_index_to_name((*ifa).ifa_index as libc::c_int)
    }
    if fnmatch(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).label,
      label,
      0i32,
    ) != 0i32
    {
      return 0i32;
    }
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t))
    .pfx
    .family
    != 0
  {
    if !rta_tb[IFA_LOCAL as libc::c_int as usize].is_null() {
      let mut dst: inet_prefix = inet_prefix {
        family: 0,
        bytelen: 0,
        bitlen: 0,
        data: [0; 4],
      };
      memset(
        &mut dst as *mut inet_prefix as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<inet_prefix>() as libc::c_ulong,
      );
      dst.family = (*ifa).ifa_family;
      memcpy(
        &mut dst.data as *mut [uint32_t; 4] as *mut libc::c_void,
        (rta_tb[IFA_LOCAL as libc::c_int as usize] as *mut libc::c_char).offset(
          ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
            .wrapping_add(4u32 as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong)
            & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
            .wrapping_add(0i32 as libc::c_ulong) as isize,
        ) as *mut libc::c_void,
        ((*rta_tb[IFA_LOCAL as libc::c_int as usize]).rta_len as libc::c_int as libc::c_ulong)
          .wrapping_sub(
            ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
              .wrapping_add(4u32 as libc::c_ulong)
              .wrapping_sub(1i32 as libc::c_ulong)
              & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
              .wrapping_add(0i32 as libc::c_ulong),
          ),
      );
      if inet_addr_match(
        &mut dst,
        &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).pfx,
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t))
          .pfx
          .bitlen as libc::c_int,
      ) != 0
      {
        return 0i32;
      }
    }
  }
  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t))
    .flushb
    .is_null()
  {
    let mut fn_0: *mut nlmsghdr = 0 as *mut nlmsghdr;
    if (((*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).flushp as libc::c_uint)
      .wrapping_add(4u32)
      .wrapping_sub(1i32 as libc::c_uint)
      & !4u32.wrapping_sub(1i32 as libc::c_uint))
    .wrapping_add((*n).nlmsg_len)
      > (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).flushe as libc::c_uint
    {
      if flush_update() != 0 {
        return -1i32;
      }
    }
    fn_0 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t))
      .flushb
      .offset(
        (((*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).flushp as libc::c_uint)
          .wrapping_add(4u32)
          .wrapping_sub(1i32 as libc::c_uint)
          & !4u32.wrapping_sub(1i32 as libc::c_uint)) as isize,
      ) as *mut nlmsghdr;
    memcpy(
      fn_0 as *mut libc::c_void,
      n as *const libc::c_void,
      (*n).nlmsg_len as libc::c_ulong,
    );
    (*fn_0).nlmsg_type = RTM_DELADDR as libc::c_int as __u16;
    (*fn_0).nlmsg_flags = 0x1i32 as __u16;
    let ref mut fresh0 = (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).rth).seq;
    *fresh0 = (*fresh0).wrapping_add(1);
    (*fn_0).nlmsg_seq = *fresh0;
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).flushp = (fn_0 as *mut libc::c_char)
      .offset((*n).nlmsg_len as isize)
      .wrapping_offset_from((*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).flushb)
      as libc::c_long
      as libc::c_int;
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).flushed = 1i32 as smallint;
    return 0i32;
  }
  if (*n).nlmsg_type as libc::c_int == RTM_DELADDR as libc::c_int {
    printf(b"Deleted \x00" as *const u8 as *const libc::c_char);
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).oneline != 0 {
    printf(
      b"%u: %s\x00" as *const u8 as *const libc::c_char,
      (*ifa).ifa_index,
      ll_index_to_name((*ifa).ifa_index as libc::c_int),
    );
  }
  if (*ifa).ifa_family as libc::c_int == 2i32 {
    printf(b"    inet \x00" as *const u8 as *const libc::c_char);
  } else if (*ifa).ifa_family as libc::c_int == 10i32 {
    printf(b"    inet6 \x00" as *const u8 as *const libc::c_char);
  } else {
    printf(
      b"    family %d \x00" as *const u8 as *const libc::c_char,
      (*ifa).ifa_family as libc::c_int,
    );
  }
  if !rta_tb[IFA_LOCAL as libc::c_int as usize].is_null() {
    fputs_unlocked(
      rt_addr_n2a(
        (*ifa).ifa_family as libc::c_int,
        (rta_tb[IFA_LOCAL as libc::c_int as usize] as *mut libc::c_char).offset(
          ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
            .wrapping_add(4u32 as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong)
            & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
            .wrapping_add(0i32 as libc::c_ulong) as isize,
        ) as *mut libc::c_void,
      ),
      stdout,
    );
    if rta_tb[IFA_ADDRESS as libc::c_int as usize].is_null()
      || memcmp(
        (rta_tb[IFA_ADDRESS as libc::c_int as usize] as *mut libc::c_char).offset(
          ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
            .wrapping_add(4u32 as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong)
            & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
            .wrapping_add(0i32 as libc::c_ulong) as isize,
        ) as *mut libc::c_void,
        (rta_tb[IFA_LOCAL as libc::c_int as usize] as *mut libc::c_char).offset(
          ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
            .wrapping_add(4u32 as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong)
            & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
            .wrapping_add(0i32 as libc::c_ulong) as isize,
        ) as *mut libc::c_void,
        4i32 as libc::c_ulong,
      ) == 0i32
    {
      printf(
        b"/%d \x00" as *const u8 as *const libc::c_char,
        (*ifa).ifa_prefixlen as libc::c_int,
      );
    } else {
      printf(
        b" peer %s/%d \x00" as *const u8 as *const libc::c_char,
        rt_addr_n2a(
          (*ifa).ifa_family as libc::c_int,
          (rta_tb[IFA_ADDRESS as libc::c_int as usize] as *mut libc::c_char).offset(
            ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
              .wrapping_add(4u32 as libc::c_ulong)
              .wrapping_sub(1i32 as libc::c_ulong)
              & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
              .wrapping_add(0i32 as libc::c_ulong) as isize,
          ) as *mut libc::c_void,
        ),
        (*ifa).ifa_prefixlen as libc::c_int,
      );
    }
  }
  if !rta_tb[IFA_BROADCAST as libc::c_int as usize].is_null() {
    printf(
      b"brd %s \x00" as *const u8 as *const libc::c_char,
      rt_addr_n2a(
        (*ifa).ifa_family as libc::c_int,
        (rta_tb[IFA_BROADCAST as libc::c_int as usize] as *mut libc::c_char).offset(
          ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
            .wrapping_add(4u32 as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong)
            & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
            .wrapping_add(0i32 as libc::c_ulong) as isize,
        ) as *mut libc::c_void,
      ),
    );
  }
  if !rta_tb[IFA_ANYCAST as libc::c_int as usize].is_null() {
    printf(
      b"any %s \x00" as *const u8 as *const libc::c_char,
      rt_addr_n2a(
        (*ifa).ifa_family as libc::c_int,
        (rta_tb[IFA_ANYCAST as libc::c_int as usize] as *mut libc::c_char).offset(
          ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
            .wrapping_add(4u32 as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong)
            & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
            .wrapping_add(0i32 as libc::c_ulong) as isize,
        ) as *mut libc::c_void,
      ),
    );
  }
  printf(
    b"scope %s \x00" as *const u8 as *const libc::c_char,
    rtnl_rtscope_n2a((*ifa).ifa_scope as libc::c_int),
  );
  if (*ifa).ifa_flags as libc::c_int & 0x1i32 != 0 {
    (*ifa).ifa_flags = ((*ifa).ifa_flags as libc::c_int & !0x1i32) as __u8;
    printf(b"secondary \x00" as *const u8 as *const libc::c_char);
  }
  if (*ifa).ifa_flags as libc::c_int & 0x40i32 != 0 {
    (*ifa).ifa_flags = ((*ifa).ifa_flags as libc::c_int & !0x40i32) as __u8;
    printf(b"tentative \x00" as *const u8 as *const libc::c_char);
  }
  if (*ifa).ifa_flags as libc::c_int & 0x8i32 != 0 {
    (*ifa).ifa_flags = ((*ifa).ifa_flags as libc::c_int & !0x8i32) as __u8;
    printf(b"dadfailed \x00" as *const u8 as *const libc::c_char);
  }
  if (*ifa).ifa_flags as libc::c_int & 0x20i32 != 0 {
    (*ifa).ifa_flags = ((*ifa).ifa_flags as libc::c_int & !0x20i32) as __u8;
    printf(b"deprecated \x00" as *const u8 as *const libc::c_char);
  }
  if (*ifa).ifa_flags as libc::c_int & 0x80i32 == 0 {
    printf(b"dynamic \x00" as *const u8 as *const libc::c_char);
  } else {
    (*ifa).ifa_flags = ((*ifa).ifa_flags as libc::c_int & !0x80i32) as __u8
  }
  if (*ifa).ifa_flags != 0 {
    printf(
      b"flags %02x \x00" as *const u8 as *const libc::c_char,
      (*ifa).ifa_flags as libc::c_int,
    );
  }
  if !rta_tb[IFA_LABEL as libc::c_int as usize].is_null() {
    fputs_unlocked(
      (rta_tb[IFA_LABEL as libc::c_int as usize] as *mut libc::c_char).offset(
        ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
          .wrapping_add(4u32 as libc::c_ulong)
          .wrapping_sub(1i32 as libc::c_ulong)
          & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
          .wrapping_add(0i32 as libc::c_ulong) as isize,
      ) as *mut libc::c_void as *mut libc::c_char,
      stdout,
    );
  }
  if !rta_tb[IFA_CACHEINFO as libc::c_int as usize].is_null() {
    let mut ci: *mut ifa_cacheinfo =
      (rta_tb[IFA_CACHEINFO as libc::c_int as usize] as *mut libc::c_char).offset(
        ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
          .wrapping_add(4u32 as libc::c_ulong)
          .wrapping_sub(1i32 as libc::c_ulong)
          & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
          .wrapping_add(0i32 as libc::c_ulong) as isize,
      ) as *mut libc::c_void as *mut ifa_cacheinfo;
    let mut buf: [libc::c_char; 128] = [0; 128];
    bb_putchar(_SL_ as libc::c_int);
    if (*ci).ifa_valid == 0xffffffffu32 {
      sprintf(
        buf.as_mut_ptr(),
        b"valid_lft forever\x00" as *const u8 as *const libc::c_char,
      );
    } else {
      sprintf(
        buf.as_mut_ptr(),
        b"valid_lft %dsec\x00" as *const u8 as *const libc::c_char,
        (*ci).ifa_valid,
      );
    }
    if (*ci).ifa_prefered == 0xffffffffu32 {
      sprintf(
        buf.as_mut_ptr().offset(strlen(buf.as_mut_ptr()) as isize),
        b" preferred_lft forever\x00" as *const u8 as *const libc::c_char,
      );
    } else {
      sprintf(
        buf.as_mut_ptr().offset(strlen(buf.as_mut_ptr()) as isize),
        b" preferred_lft %dsec\x00" as *const u8 as *const libc::c_char,
        (*ci).ifa_prefered,
      );
    }
    printf(
      b"       %s\x00" as *const u8 as *const libc::c_char,
      buf.as_mut_ptr(),
    );
  }
  bb_putchar('\n' as i32);
  /*fflush_all();*/
  return 0i32;
}
unsafe extern "C" fn print_selected_addrinfo(
  mut ifindex: libc::c_int,
  mut ainfo: *mut nlmsg_list,
) -> libc::c_int {
  while !ainfo.is_null() {
    let mut n: *mut nlmsghdr = &mut (*ainfo).h;
    let mut ifa: *mut ifaddrmsg = (n as *mut libc::c_char).offset(
      (0i32
        + ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
          .wrapping_add(4u32 as libc::c_ulong)
          .wrapping_sub(1i32 as libc::c_ulong)
          & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong) as libc::c_int)
        as isize,
    ) as *mut libc::c_void as *mut ifaddrmsg;
    if !((*n).nlmsg_type as libc::c_int != RTM_NEWADDR as libc::c_int) {
      if ((*n).nlmsg_len as libc::c_ulong)
        < (::std::mem::size_of::<*mut ifaddrmsg>() as libc::c_ulong).wrapping_add(
          ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
            .wrapping_add(4u32 as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong)
            & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong) as libc::c_int
            as libc::c_ulong,
        )
      {
        return -1i32;
      }
      if !((*ifa).ifa_index != ifindex as libc::c_uint
        || (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).family as libc::c_int != 0
          && (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).family as libc::c_int
            != (*ifa).ifa_family as libc::c_int)
      {
        print_addrinfo(0 as *const sockaddr_nl, n, 0 as *mut libc::c_void);
      }
    }
    ainfo = (*ainfo).next
  }
  return 0i32;
}
unsafe extern "C" fn store_nlmsg(
  mut who: *const sockaddr_nl,
  mut n: *mut nlmsghdr,
  mut arg: *mut libc::c_void,
) -> libc::c_int {
  let mut linfo: *mut *mut nlmsg_list = arg as *mut *mut nlmsg_list;
  let mut h: *mut nlmsg_list = 0 as *mut nlmsg_list;
  let mut lp: *mut *mut nlmsg_list = 0 as *mut *mut nlmsg_list;
  h = xzalloc(
    ((*n).nlmsg_len as libc::c_ulong)
      .wrapping_add(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong),
  ) as *mut nlmsg_list;
  memcpy(
    &mut (*h).h as *mut nlmsghdr as *mut libc::c_void,
    n as *const libc::c_void,
    (*n).nlmsg_len as libc::c_ulong,
  );
  /*h->next = NULL; - xzalloc did it */
  lp = linfo;
  while !(*lp).is_null() {
    lp = &mut (**lp).next
  }
  *lp = h;
  ll_remember_index(who, n, 0 as *mut libc::c_void);
  return 0i32;
}
unsafe extern "C" fn ipaddr_reset_filter(mut _oneline: libc::c_int) {
  memset(
    bb_common_bufsiz1.as_mut_ptr() as *mut filter_t as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<filter_t>() as libc::c_ulong,
  );
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).oneline = _oneline as smallint;
}
/* Return value becomes exitcode. It's okay to not return at all */
#[no_mangle]
pub unsafe extern "C" fn ipaddr_list_or_flush(
  mut argv: *mut *mut libc::c_char,
  mut flush: libc::c_int,
) -> libc::c_int {
  static mut option: [libc::c_char; 23] = [
    116, 111, 0, 115, 99, 111, 112, 101, 0, 117, 112, 0, 108, 97, 98, 101, 108, 0, 100, 101, 118,
    0, 0,
  ];
  let mut linfo: *mut nlmsg_list = 0 as *mut nlmsg_list;
  let mut ainfo: *mut nlmsg_list = 0 as *mut nlmsg_list;
  let mut l: *mut nlmsg_list = 0 as *mut nlmsg_list;
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
  let mut filter_dev: *mut libc::c_char = 0 as *mut libc::c_char;
  ipaddr_reset_filter(oneline as libc::c_int);
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).showqueue = 1i32 as smallint;
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).family as libc::c_int == 0i32 {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).family = preferred_family
  }
  if flush != 0 {
    if (*argv).is_null() {
      bb_error_msg_and_die(
        bb_msg_requires_arg.as_ptr(),
        b"flush\x00" as *const u8 as *const libc::c_char,
      );
    }
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).family as libc::c_int == 17i32 {
      bb_simple_error_msg_and_die(
        b"can\'t flush link addresses\x00" as *const u8 as *const libc::c_char,
      );
    }
  }
  while !(*argv).is_null() {
    let key: smalluint = index_in_strings(option.as_ptr(), *argv) as smalluint;
    if key as libc::c_int == 0i32 {
      /* to */
      argv = next_arg(argv);
      get_prefix(
        &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).pfx,
        *argv,
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).family as libc::c_int,
      );
      if (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).family as libc::c_int == 0i32 {
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).family =
          (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t))
            .pfx
            .family as family_t
      }
    } else if key as libc::c_int == 1i32 {
      /* scope */
      let mut scope: uint32_t = 0i32 as uint32_t;
      argv = next_arg(argv);
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).scopemask = -1i32;
      if rtnl_rtscope_a2n(&mut scope, *argv) != 0 {
        if strcmp(*argv, b"all\x00" as *const u8 as *const libc::c_char) != 0i32 {
          invarg_1_to_2(*argv, b"scope\x00" as *const u8 as *const libc::c_char);
        }
        scope = RT_SCOPE_NOWHERE as libc::c_int as uint32_t;
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).scopemask = 0i32
      }
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).scope = scope as libc::c_int
    } else if key as libc::c_int == 2i32 {
      /* up */
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).up = 1i32 as smallint
    } else if key as libc::c_int == 3i32 {
      /* label */
      argv = next_arg(argv);
      let ref mut fresh1 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).label;
      *fresh1 = *argv
    } else {
      if key as libc::c_int == 4i32 {
        /* dev */
        argv = next_arg(argv)
      }
      if !filter_dev.is_null() {
        duparg2(b"dev\x00" as *const u8 as *const libc::c_char, *argv);
      }
      filter_dev = *argv
    }
    argv = argv.offset(1)
  }
  xrtnl_open(&mut rth);
  xrtnl_wilddump_request(
    &mut rth,
    preferred_family as libc::c_int,
    RTM_GETLINK as libc::c_int,
  );
  xrtnl_dump_filter(
    &mut rth,
    Some(
      store_nlmsg
        as unsafe extern "C" fn(
          _: *const sockaddr_nl,
          _: *mut nlmsghdr,
          _: *mut libc::c_void,
        ) -> libc::c_int,
    ),
    &mut linfo as *mut *mut nlmsg_list as *mut libc::c_void,
  );
  if !filter_dev.is_null() {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).ifindex = xll_name_to_index(filter_dev)
  }
  if flush != 0 {
    let mut flushb: [libc::c_char; 3584] = [0; 3584];
    let ref mut fresh2 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).flushb;
    *fresh2 = flushb.as_mut_ptr();
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).flushp = 0i32;
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).flushe =
      ::std::mem::size_of::<[libc::c_char; 3584]>() as libc::c_ulong as libc::c_int;
    let ref mut fresh3 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).rth;
    *fresh3 = &mut rth;
    loop {
      xrtnl_wilddump_request(
        &mut rth,
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).family as libc::c_int,
        RTM_GETADDR as libc::c_int,
      );
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).flushed = 0i32 as smallint;
      xrtnl_dump_filter(
        &mut rth,
        Some(
          print_addrinfo
            as unsafe extern "C" fn(
              _: *const sockaddr_nl,
              _: *mut nlmsghdr,
              _: *mut libc::c_void,
            ) -> libc::c_int,
        ),
        0 as *mut libc::c_void,
      );
      if (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).flushed as libc::c_int == 0i32 {
        return 0i32;
      }
      if flush_update() < 0i32 {
        return 1i32;
      }
    }
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).family as libc::c_int != 17i32 {
    xrtnl_wilddump_request(
      &mut rth,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).family as libc::c_int,
      RTM_GETADDR as libc::c_int,
    );
    xrtnl_dump_filter(
      &mut rth,
      Some(
        store_nlmsg
          as unsafe extern "C" fn(
            _: *const sockaddr_nl,
            _: *mut nlmsghdr,
            _: *mut libc::c_void,
          ) -> libc::c_int,
      ),
      &mut ainfo as *mut *mut nlmsg_list as *mut libc::c_void,
    );
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).family as libc::c_int != 0
    && (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).family as libc::c_int != 17i32
  {
    let mut lp: *mut *mut nlmsg_list = 0 as *mut *mut nlmsg_list;
    lp = &mut linfo;
    loop {
      l = *lp;
      if l.is_null() {
        break;
      }
      let mut ok: libc::c_int = 0i32;
      let mut ifi: *mut ifinfomsg = (&mut (*l).h as *mut nlmsghdr as *mut libc::c_char).offset(
        (0i32
          + ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
            .wrapping_add(4u32 as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong)
            & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong) as libc::c_int)
          as isize,
      ) as *mut libc::c_void as *mut ifinfomsg;
      let mut a: *mut nlmsg_list = 0 as *mut nlmsg_list;
      let mut current_block_93: u64;
      a = ainfo;
      while !a.is_null() {
        let mut n: *mut nlmsghdr = &mut (*a).h;
        let mut ifa: *mut ifaddrmsg = (n as *mut libc::c_char).offset(
          (0i32
            + ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
              .wrapping_add(4u32 as libc::c_ulong)
              .wrapping_sub(1i32 as libc::c_ulong)
              & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
              as libc::c_int) as isize,
        ) as *mut libc::c_void as *mut ifaddrmsg;
        if !((*ifa).ifa_index != (*ifi).ifi_index as libc::c_uint
          || (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).family as libc::c_int != 0
            && (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).family as libc::c_int
              != (*ifa).ifa_family as libc::c_int)
        {
          if !(((*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).scope
            ^ (*ifa).ifa_scope as libc::c_int)
            & (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).scopemask
            != 0)
          {
            if !(((*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).flags
              ^ (*ifa).ifa_flags as libc::c_int)
              & (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).flagmask
              != 0)
            {
              if (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t))
                .pfx
                .family as libc::c_int
                != 0
                || !(*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t))
                  .label
                  .is_null()
              {
                let mut tb: [*mut rtattr; 9] = [0 as *mut rtattr; 9];
                //memset(tb, 0, sizeof(tb)); - parse_rtattr does this
                parse_rtattr(
                  tb.as_mut_ptr(),
                  __IFA_MAX as libc::c_int - 1i32,
                  (ifa as *mut libc::c_char).offset(
                    ((::std::mem::size_of::<ifaddrmsg>() as libc::c_ulong)
                      .wrapping_add(4u32 as libc::c_ulong)
                      .wrapping_sub(1i32 as libc::c_ulong)
                      & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
                      as isize,
                  ) as *mut rtattr,
                  ((*n).nlmsg_len as libc::c_ulong).wrapping_sub(
                    (::std::mem::size_of::<ifaddrmsg>() as libc::c_ulong)
                      .wrapping_add(
                        ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
                          .wrapping_add(4u32 as libc::c_ulong)
                          .wrapping_sub(1i32 as libc::c_ulong)
                          & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
                          as libc::c_int as libc::c_ulong,
                      )
                      .wrapping_add(4u32 as libc::c_ulong)
                      .wrapping_sub(1i32 as libc::c_ulong)
                      & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong,
                  ) as libc::c_int,
                );
                if tb[IFA_LOCAL as libc::c_int as usize].is_null() {
                  tb[IFA_LOCAL as libc::c_int as usize] = tb[IFA_ADDRESS as libc::c_int as usize]
                }
                if (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t))
                  .pfx
                  .family as libc::c_int
                  != 0
                  && !tb[IFA_LOCAL as libc::c_int as usize].is_null()
                {
                  let mut dst: inet_prefix = inet_prefix {
                    family: 0,
                    bytelen: 0,
                    bitlen: 0,
                    data: [0; 4],
                  };
                  memset(
                    &mut dst as *mut inet_prefix as *mut libc::c_void,
                    0i32,
                    ::std::mem::size_of::<inet_prefix>() as libc::c_ulong,
                  );
                  dst.family = (*ifa).ifa_family;
                  memcpy(
                    &mut dst.data as *mut [uint32_t; 4] as *mut libc::c_void,
                    (tb[IFA_LOCAL as libc::c_int as usize] as *mut libc::c_char).offset(
                      ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
                        .wrapping_add(4u32 as libc::c_ulong)
                        .wrapping_sub(1i32 as libc::c_ulong)
                        & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
                        .wrapping_add(0i32 as libc::c_ulong) as isize,
                    ) as *mut libc::c_void,
                    ((*tb[IFA_LOCAL as libc::c_int as usize]).rta_len as libc::c_int
                      as libc::c_ulong)
                      .wrapping_sub(
                        ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
                          .wrapping_add(4u32 as libc::c_ulong)
                          .wrapping_sub(1i32 as libc::c_ulong)
                          & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
                          .wrapping_add(0i32 as libc::c_ulong),
                      ),
                  );
                  if inet_addr_match(
                    &mut dst,
                    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).pfx,
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t))
                      .pfx
                      .bitlen as libc::c_int,
                  ) != 0
                  {
                    current_block_93 = 2310077433060450808;
                  } else {
                    current_block_93 = 4899250571165509867;
                  }
                } else {
                  current_block_93 = 4899250571165509867;
                }
                match current_block_93 {
                  2310077433060450808 => {}
                  _ => {
                    if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t))
                      .label
                      .is_null()
                    {
                      let mut label: *const libc::c_char = 0 as *const libc::c_char;
                      if !tb[IFA_LABEL as libc::c_int as usize].is_null() {
                        label = (tb[IFA_LABEL as libc::c_int as usize] as *mut libc::c_char).offset(
                          ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
                            .wrapping_add(4u32 as libc::c_ulong)
                            .wrapping_sub(1i32 as libc::c_ulong)
                            & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
                            .wrapping_add(0i32 as libc::c_ulong) as isize,
                        ) as *mut libc::c_void
                          as *const libc::c_char
                      } else {
                        label = ll_index_to_name((*ifa).ifa_index as libc::c_int)
                      }
                      if fnmatch(
                        (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).label,
                        label,
                        0i32,
                      ) != 0i32
                      {
                        current_block_93 = 2310077433060450808;
                      } else {
                        current_block_93 = 5697748000427295508;
                      }
                    } else {
                      current_block_93 = 5697748000427295508;
                    }
                  }
                }
              } else {
                current_block_93 = 5697748000427295508;
              }
              match current_block_93 {
                2310077433060450808 => {}
                _ => {
                  ok = 1i32;
                  break;
                }
              }
            }
          }
        }
        a = (*a).next
      }
      if ok == 0 {
        *lp = (*l).next
      } else {
        lp = &mut (*l).next
      }
    }
  }
  l = linfo;
  while !l.is_null() {
    if oneline as libc::c_int != 0
      && (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).family as libc::c_int != 17i32
      || print_linkinfo(&mut (*l).h) == 0i32
    {
      let mut ifi_0: *mut ifinfomsg = (&mut (*l).h as *mut nlmsghdr as *mut libc::c_char).offset(
        (0i32
          + ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
            .wrapping_add(4u32 as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong)
            & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong) as libc::c_int)
          as isize,
      ) as *mut libc::c_void as *mut ifinfomsg;
      if (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).family as libc::c_int != 17i32 {
        print_selected_addrinfo((*ifi_0).ifi_index, ainfo);
      }
    }
    l = (*l).next
  }
  return 0i32;
}
unsafe extern "C" fn default_scope(mut lcl: *mut inet_prefix) -> libc::c_int {
  if (*lcl).family as libc::c_int == 2i32 {
    if (*lcl).bytelen as libc::c_int >= 1i32
      && *(&mut (*lcl).data as *mut [uint32_t; 4] as *mut uint8_t) as libc::c_int == 127i32
    {
      return RT_SCOPE_HOST as libc::c_int;
    }
  }
  return 0i32;
}
/* Return value becomes exitcode. It's okay to not return at all */
unsafe extern "C" fn ipaddr_modify(
  mut cmd: libc::c_int,
  mut flags: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  /* If you add stuff here, update ipaddr_full_usage */
  static mut option: [libc::c_char; 57] = [
    112, 101, 101, 114, 0, 114, 101, 109, 111, 116, 101, 0, 98, 114, 111, 97, 100, 99, 97, 115,
    116, 0, 98, 114, 100, 0, 97, 110, 121, 99, 97, 115, 116, 0, 115, 99, 111, 112, 101, 0, 100,
    101, 118, 0, 108, 97, 98, 101, 108, 0, 108, 111, 99, 97, 108, 0, 0,
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
  let mut req: C2RustUnnamed_5 = C2RustUnnamed_5 {
    n: nlmsghdr {
      nlmsg_len: 0,
      nlmsg_type: 0,
      nlmsg_flags: 0,
      nlmsg_seq: 0,
      nlmsg_pid: 0,
    },
    ifa: ifaddrmsg {
      ifa_family: 0,
      ifa_prefixlen: 0,
      ifa_flags: 0,
      ifa_scope: 0,
      ifa_index: 0,
    },
    buf: [0; 256],
  };
  let mut d: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut l: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut lcl: inet_prefix = inet_prefix {
    family: 0,
    bytelen: 0,
    bitlen: 0,
    data: [0; 4],
  };
  let mut peer: inet_prefix = inet_prefix {
    family: 0,
    bytelen: 0,
    bitlen: 0,
    data: [0; 4],
  };
  let mut local_len: libc::c_int = 0i32;
  let mut peer_len: libc::c_int = 0i32;
  let mut brd_len: libc::c_int = 0i32;
  let mut any_len: libc::c_int = 0i32;
  let mut scoped: bool = 0i32 != 0;
  memset(
    &mut req as *mut C2RustUnnamed_5 as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<C2RustUnnamed_5>() as libc::c_ulong,
  );
  req.n.nlmsg_len = (::std::mem::size_of::<ifaddrmsg>() as libc::c_ulong).wrapping_add(
    ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
      .wrapping_add(4u32 as libc::c_ulong)
      .wrapping_sub(1i32 as libc::c_ulong)
      & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong) as libc::c_int
      as libc::c_ulong,
  ) as __u32;
  req.n.nlmsg_flags = (0x1i32 | flags) as __u16;
  req.n.nlmsg_type = cmd as __u16;
  req.ifa.ifa_family = preferred_family as __u8;
  while !(*argv).is_null() {
    let mut arg: libc::c_uint = index_in_strings(option.as_ptr(), *argv) as libc::c_uint;
    /* if search fails, "local" is assumed */
    if arg as libc::c_int >= 0i32 {
      argv = next_arg(argv)
    }
    if arg <= 1i32 as libc::c_uint {
      /* peer, remote */
      if peer_len != 0 {
        duparg(option.as_ptr(), *argv);
      }
      get_prefix(&mut peer, *argv, req.ifa.ifa_family as libc::c_int);
      peer_len = peer.bytelen as libc::c_int;
      if req.ifa.ifa_family as libc::c_int == 0i32 {
        req.ifa.ifa_family = peer.family
      }
      addattr_l(
        &mut req.n,
        ::std::mem::size_of::<C2RustUnnamed_5>() as libc::c_ulong as libc::c_int,
        IFA_ADDRESS as libc::c_int,
        &mut peer.data as *mut [uint32_t; 4] as *mut libc::c_void,
        peer.bytelen as libc::c_int,
      );
      req.ifa.ifa_prefixlen = peer.bitlen as __u8
    } else if arg <= 3i32 as libc::c_uint {
      /* broadcast, brd */
      let mut addr: inet_prefix = inet_prefix {
        family: 0,
        bytelen: 0,
        bitlen: 0,
        data: [0; 4],
      };
      if brd_len != 0 {
        duparg(
          option
            .as_ptr()
            .offset(::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as isize)
            .offset(::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as isize),
          *argv,
        );
      }
      if *(*argv).offset(0) as libc::c_int == '+' as i32 && *(*argv).offset(1) == 0 {
        brd_len = -1i32
      } else if *(*argv).offset(0) as libc::c_int == '-' as i32 && *(*argv).offset(1) == 0 {
        brd_len = -2i32
      } else {
        get_addr(&mut addr, *argv, req.ifa.ifa_family as libc::c_int);
        if req.ifa.ifa_family as libc::c_int == 0i32 {
          req.ifa.ifa_family = addr.family
        }
        addattr_l(
          &mut req.n,
          ::std::mem::size_of::<C2RustUnnamed_5>() as libc::c_ulong as libc::c_int,
          IFA_BROADCAST as libc::c_int,
          &mut addr.data as *mut [uint32_t; 4] as *mut libc::c_void,
          addr.bytelen as libc::c_int,
        );
        brd_len = addr.bytelen as libc::c_int
      }
    } else if arg == 4i32 as libc::c_uint {
      /* anycast */
      let mut addr_0: inet_prefix = inet_prefix {
        family: 0,
        bytelen: 0,
        bitlen: 0,
        data: [0; 4],
      };
      if any_len != 0 {
        duparg(
          option
            .as_ptr()
            .offset(::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as isize)
            .offset(::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as isize)
            .offset(::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong as isize)
            .offset(::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong as isize),
          *argv,
        );
      }
      get_addr(&mut addr_0, *argv, req.ifa.ifa_family as libc::c_int);
      if req.ifa.ifa_family as libc::c_int == 0i32 {
        req.ifa.ifa_family = addr_0.family
      }
      addattr_l(
        &mut req.n,
        ::std::mem::size_of::<C2RustUnnamed_5>() as libc::c_ulong as libc::c_int,
        IFA_ANYCAST as libc::c_int,
        &mut addr_0.data as *mut [uint32_t; 4] as *mut libc::c_void,
        addr_0.bytelen as libc::c_int,
      );
      any_len = addr_0.bytelen as libc::c_int
    } else if arg == 5i32 as libc::c_uint {
      /* scope */
      let mut scope: uint32_t = 0i32 as uint32_t;
      if rtnl_rtscope_a2n(&mut scope, *argv) != 0 {
        invarg_1_to_2(*argv, b"scope\x00" as *const u8 as *const libc::c_char);
      }
      req.ifa.ifa_scope = scope as __u8;
      scoped = 1i32 != 0
    } else if arg == 6i32 as libc::c_uint {
      /* dev */
      d = *argv
    } else if arg == 7i32 as libc::c_uint {
      /* label */
      l = *argv;
      addattr_l(
        &mut req.n,
        ::std::mem::size_of::<C2RustUnnamed_5>() as libc::c_ulong as libc::c_int,
        IFA_LABEL as libc::c_int,
        l as *mut libc::c_void,
        strlen(l).wrapping_add(1i32 as libc::c_ulong) as libc::c_int,
      );
    } else {
      /* local (specified or assumed) */
      if local_len != 0 {
        duparg2(b"local\x00" as *const u8 as *const libc::c_char, *argv);
      }
      get_prefix(&mut lcl, *argv, req.ifa.ifa_family as libc::c_int);
      if req.ifa.ifa_family as libc::c_int == 0i32 {
        req.ifa.ifa_family = lcl.family
      }
      addattr_l(
        &mut req.n,
        ::std::mem::size_of::<C2RustUnnamed_5>() as libc::c_ulong as libc::c_int,
        IFA_LOCAL as libc::c_int,
        &mut lcl.data as *mut [uint32_t; 4] as *mut libc::c_void,
        lcl.bytelen as libc::c_int,
      );
      local_len = lcl.bytelen as libc::c_int
    }
    argv = argv.offset(1)
  }
  if d.is_null() {
    /* There was no "dev IFACE", but we need that */
    bb_simple_error_msg_and_die(b"need \"dev IFACE\"\x00" as *const u8 as *const libc::c_char);
  }
  if !l.is_null() && is_prefixed_with(l, d).is_null() {
    bb_error_msg_and_die(
      b"\"dev\" (%s) must match \"label\" (%s)\x00" as *const u8 as *const libc::c_char,
      d,
      l,
    );
  }
  if peer_len == 0i32 && local_len != 0 && cmd != RTM_DELADDR as libc::c_int {
    peer = lcl;
    addattr_l(
      &mut req.n,
      ::std::mem::size_of::<C2RustUnnamed_5>() as libc::c_ulong as libc::c_int,
      IFA_ADDRESS as libc::c_int,
      &mut lcl.data as *mut [uint32_t; 4] as *mut libc::c_void,
      lcl.bytelen as libc::c_int,
    );
  }
  if req.ifa.ifa_prefixlen as libc::c_int == 0i32 {
    req.ifa.ifa_prefixlen = lcl.bitlen as __u8
  }
  if brd_len < 0i32 && cmd != RTM_DELADDR as libc::c_int {
    let mut brd: inet_prefix = inet_prefix {
      family: 0,
      bytelen: 0,
      bitlen: 0,
      data: [0; 4],
    };
    let mut i: libc::c_int = 0;
    if req.ifa.ifa_family as libc::c_int != 2i32 {
      bb_simple_error_msg_and_die(
        b"broadcast can be set only for IPv4 addresses\x00" as *const u8 as *const libc::c_char,
      );
    }
    brd = peer;
    if brd.bitlen as libc::c_int <= 30i32 {
      i = 31i32;
      while i >= brd.bitlen as libc::c_int {
        if brd_len == -1i32 {
          brd.data[0] |= {
            let mut __v: libc::c_uint = 0;
            let mut __x: libc::c_uint = (1i32 << 31i32 - i) as libc::c_uint;
            if 0 != 0 {
              __v = (__x & 0xff000000u32) >> 24i32
                | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
                | (__x & 0xff00i32 as libc::c_uint) << 8i32
                | (__x & 0xffi32 as libc::c_uint) << 24i32
            } else {
              let fresh4 = &mut __v;
              let fresh5;
              let fresh6 = __x;
              asm!("bswap $0" : "=r" (fresh5) : "0"
                                      (c2rust_asm_casts::AsmCast::cast_in(fresh4, fresh6))
                                      :);
              c2rust_asm_casts::AsmCast::cast_out(fresh4, fresh6, fresh5);
            }
            __v
          }
        } else {
          brd.data[0] &= !({
            let mut __v: libc::c_uint = 0;
            let mut __x: libc::c_uint = (1i32 << 31i32 - i) as libc::c_uint;
            if 0 != 0 {
              __v = (__x & 0xff000000u32) >> 24i32
                | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
                | (__x & 0xff00i32 as libc::c_uint) << 8i32
                | (__x & 0xffi32 as libc::c_uint) << 24i32
            } else {
              let fresh7 = &mut __v;
              let fresh8;
              let fresh9 = __x;
              asm!("bswap $0" : "=r" (fresh8) : "0"
                                       (c2rust_asm_casts::AsmCast::cast_in(fresh7, fresh9))
                                       :);
              c2rust_asm_casts::AsmCast::cast_out(fresh7, fresh9, fresh8);
            }
            __v
          })
        }
        i -= 1
      }
      addattr_l(
        &mut req.n,
        ::std::mem::size_of::<C2RustUnnamed_5>() as libc::c_ulong as libc::c_int,
        IFA_BROADCAST as libc::c_int,
        &mut brd.data as *mut [uint32_t; 4] as *mut libc::c_void,
        brd.bytelen as libc::c_int,
      );
      brd_len = brd.bytelen as libc::c_int
    }
  }
  if !scoped && cmd != RTM_DELADDR as libc::c_int {
    req.ifa.ifa_scope = default_scope(&mut lcl) as __u8
  }
  xrtnl_open(&mut rth);
  ll_init_map(&mut rth);
  req.ifa.ifa_index = xll_name_to_index(d) as __u32;
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
pub unsafe extern "C" fn do_ipaddr(mut argv: *mut *mut libc::c_char) -> libc::c_int {
  static mut commands: [libc::c_char; 51] = [
    97, 100, 100, 0, 99, 104, 97, 110, 103, 101, 0, 99, 104, 103, 0, 114, 101, 112, 108, 97, 99,
    101, 0, 100, 101, 108, 101, 116, 101, 0, 108, 105, 115, 116, 0, 115, 104, 111, 119, 0, 108,
    115, 116, 0, 102, 108, 117, 115, 104, 0, 0,
  ];
  let mut cmd: libc::c_int = 2i32;
  if !(*argv).is_null() {
    cmd = index_in_substrings(commands.as_ptr(), *argv);
    if cmd < 0i32 {
      invarg_1_to_2(*argv, applet_name);
    }
    argv = argv.offset(1);
    if cmd <= 4i32 {
      return ipaddr_modify(
        if cmd == 4i32 {
          RTM_DELADDR as libc::c_int
        } else {
          RTM_NEWADDR as libc::c_int
        },
        if cmd == 0i32 {
          (0x400i32) | 0x200i32
        } else if cmd == 1i32 || cmd == 2i32 {
          0x100i32
        } else if cmd == 3i32 {
          (0x400i32) | 0x100i32
        } else {
          0i32
        },
        argv,
      );
    }
  }
  return ipaddr_list_or_flush(argv, (cmd == 8i32) as libc::c_int);
}
