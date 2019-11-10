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

  #[no_mangle]
  fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
  #[no_mangle]
  static mut stderr: *mut FILE;


  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  fn open_or_warn(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xsendto(
    s: libc::c_int,
    buf: *const libc::c_void,
    len: size_t,
    to: *const sockaddr,
    tolen: socklen_t,
  ) -> ssize_t;
  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xfunc_die() -> !;
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn index_in_strings(strings: *const libc::c_char, key: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn index_in_substrings(strings: *const libc::c_char, key: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  static mut applet_name: *const libc::c_char;
  #[no_mangle]
  static bb_msg_requires_arg: [libc::c_char; 0];
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
  #[no_mangle]
  fn rtnl_rtscope_n2a(id: libc::c_int) -> *const libc::c_char;
  #[no_mangle]
  fn rtnl_rttable_n2a(id: libc::c_int) -> *const libc::c_char;
  #[no_mangle]
  fn rtnl_rtprot_a2n(id: *mut u32, arg: *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn rtnl_rtscope_a2n(id: *mut u32, arg: *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn rtnl_rttable_a2n(id: *mut u32, arg: *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn ll_init_map(rth: *mut rtnl_handle) -> libc::c_int;
  #[no_mangle]
  fn xll_name_to_index(name: *const libc::c_char) -> libc::c_int;

  /* UNUSED */
  /* UNUSED */
  /* UNUSED */
  /* UNUSED */
  /*void get_prefix_1(inet_prefix *dst, char *arg, int family) FAST_FUNC;*/
  #[no_mangle]
  fn inet_addr_match(
    a: *const inet_prefix,
    b: *const inet_prefix,
    bits: libc::c_int,
  ) -> libc::c_int;
  #[no_mangle]
  static mut _SL_: libc::c_char;

  /* We need linux/types.h because older kernels use u32 etc
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
  fn addattr32(
    n: *mut nlmsghdr,
    maxlen: libc::c_int,
    type_0: libc::c_int,
    data: u32,
  ) -> libc::c_int;
  #[no_mangle]
  fn rta_addattr32(
    rta: *mut rtattr,
    maxlen: libc::c_int,
    type_0: libc::c_int,
    data: u32,
  ) -> libc::c_int;
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
  fn duparg2(_: *const libc::c_char, _: *const libc::c_char) -> !;
  #[no_mangle]
  fn rt_addr_n2a(af: libc::c_int, addr: *mut libc::c_void) -> *const libc::c_char;

  #[no_mangle]
  fn rtnl_rtntype_n2a(id: libc::c_int) -> *const libc::c_char;
  #[no_mangle]
  fn rtnl_send_check(
    rth: *mut rtnl_handle,
    buf: *const libc::c_void,
    len: libc::c_int,
  ) -> libc::c_int;
  #[no_mangle]
  fn parse_rtattr(tb: *mut *mut rtattr, max: libc::c_int, rta: *mut rtattr, len: libc::c_int);
  #[no_mangle]
  fn rtnl_rtntype_a2n(id: *mut libc::c_int, arg: *mut libc::c_char) -> libc::c_int;
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
  //static: const char *ll_idx_n2a(int idx, char *buf) FAST_FUNC;
  #[no_mangle]
  fn ll_index_to_name(idx: libc::c_int) -> *const libc::c_char;
  #[no_mangle]
  fn xrtnl_wilddump_request(rth: *mut rtnl_handle, fam: libc::c_int, type_0: libc::c_int);
  #[no_mangle]
  fn next_arg(argv: *mut *mut libc::c_char) -> *mut *mut libc::c_char;
  #[no_mangle]
  fn xrtnl_open(rth: *mut rtnl_handle);
  #[no_mangle]
  static mut preferred_family: family_t;
  #[no_mangle]
  fn get_addr(dst: *mut inet_prefix, arg: *mut libc::c_char, family: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn get_u32(arg: *mut libc::c_char, errmsg: *const libc::c_char) -> u32;
  #[no_mangle]
  fn get_unsigned(arg: *mut libc::c_char, errmsg: *const libc::c_char) -> libc::c_uint;
  #[no_mangle]
  fn get_prefix(dst: *mut inet_prefix, arg: *mut libc::c_char, family: libc::c_int);
  #[no_mangle]
  fn invarg_1_to_2(_: *const libc::c_char, _: *const libc::c_char) -> !;
  //const char *dnet_ntop(int af, const void *addr, char *str, size_t len);
  //int dnet_pton(int af, const char *src, void *addr);
  //const char *ipx_ntop(int af, const void *addr, char *str, size_t len);
  //int ipx_pton(int af, const char *src, void *addr);
  #[no_mangle]
  fn get_hz() -> libc::c_uint;
}

pub type __socklen_t = libc::c_uint;

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
use crate::librb::size_t;
use libc::ssize_t;
pub type socklen_t = __socklen_t;

use libc::sockaddr;

use libc::FILE;
pub type family_t = i8;
pub type __u16 = libc::c_ushort;
pub type __s32 = libc::c_int;
pub type u32 = libc::c_uint;
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
pub type C2RustUnnamed = libc::c_uint;
pub const __RTM_MAX: C2RustUnnamed = 97;
pub const RTM_NEWCACHEREPORT: C2RustUnnamed = 96;
pub const RTM_GETSTATS: C2RustUnnamed = 94;
pub const RTM_NEWSTATS: C2RustUnnamed = 92;
pub const RTM_GETNSID: C2RustUnnamed = 90;
pub const RTM_DELNSID: C2RustUnnamed = 89;
pub const RTM_NEWNSID: C2RustUnnamed = 88;
pub const RTM_GETMDB: C2RustUnnamed = 86;
pub const RTM_DELMDB: C2RustUnnamed = 85;
pub const RTM_NEWMDB: C2RustUnnamed = 84;
pub const RTM_GETNETCONF: C2RustUnnamed = 82;
pub const RTM_DELNETCONF: C2RustUnnamed = 81;
pub const RTM_NEWNETCONF: C2RustUnnamed = 80;
pub const RTM_SETDCB: C2RustUnnamed = 79;
pub const RTM_GETDCB: C2RustUnnamed = 78;
pub const RTM_GETADDRLABEL: C2RustUnnamed = 74;
pub const RTM_DELADDRLABEL: C2RustUnnamed = 73;
pub const RTM_NEWADDRLABEL: C2RustUnnamed = 72;
pub const RTM_NEWNDUSEROPT: C2RustUnnamed = 68;
pub const RTM_SETNEIGHTBL: C2RustUnnamed = 67;
pub const RTM_GETNEIGHTBL: C2RustUnnamed = 66;
pub const RTM_NEWNEIGHTBL: C2RustUnnamed = 64;
pub const RTM_GETANYCAST: C2RustUnnamed = 62;
pub const RTM_GETMULTICAST: C2RustUnnamed = 58;
pub const RTM_NEWPREFIX: C2RustUnnamed = 52;
pub const RTM_GETACTION: C2RustUnnamed = 50;
pub const RTM_DELACTION: C2RustUnnamed = 49;
pub const RTM_NEWACTION: C2RustUnnamed = 48;
pub const RTM_GETTFILTER: C2RustUnnamed = 46;
pub const RTM_DELTFILTER: C2RustUnnamed = 45;
pub const RTM_NEWTFILTER: C2RustUnnamed = 44;
pub const RTM_GETTCLASS: C2RustUnnamed = 42;
pub const RTM_DELTCLASS: C2RustUnnamed = 41;
pub const RTM_NEWTCLASS: C2RustUnnamed = 40;
pub const RTM_GETQDISC: C2RustUnnamed = 38;
pub const RTM_DELQDISC: C2RustUnnamed = 37;
pub const RTM_NEWQDISC: C2RustUnnamed = 36;
pub const RTM_GETRULE: C2RustUnnamed = 34;
pub const RTM_DELRULE: C2RustUnnamed = 33;
pub const RTM_NEWRULE: C2RustUnnamed = 32;
pub const RTM_GETNEIGH: C2RustUnnamed = 30;
pub const RTM_DELNEIGH: C2RustUnnamed = 29;
pub const RTM_NEWNEIGH: C2RustUnnamed = 28;
pub const RTM_GETROUTE: C2RustUnnamed = 26;
pub const RTM_DELROUTE: C2RustUnnamed = 25;
pub const RTM_NEWROUTE: C2RustUnnamed = 24;
pub const RTM_GETADDR: C2RustUnnamed = 22;
pub const RTM_DELADDR: C2RustUnnamed = 21;
pub const RTM_NEWADDR: C2RustUnnamed = 20;
pub const RTM_SETLINK: C2RustUnnamed = 19;
pub const RTM_GETLINK: C2RustUnnamed = 18;
pub const RTM_DELLINK: C2RustUnnamed = 17;
pub const RTM_NEWLINK: C2RustUnnamed = 16;
pub const RTM_BASE: C2RustUnnamed = 16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rtattr {
  pub rta_len: libc::c_ushort,
  pub rta_type: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rtmsg {
  pub rtm_family: libc::c_uchar,
  pub rtm_dst_len: libc::c_uchar,
  pub rtm_src_len: libc::c_uchar,
  pub rtm_tos: libc::c_uchar,
  pub rtm_table: libc::c_uchar,
  pub rtm_protocol: libc::c_uchar,
  pub rtm_scope: libc::c_uchar,
  pub rtm_type: libc::c_uchar,
  pub rtm_flags: libc::c_uint,
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const __RTN_MAX: C2RustUnnamed_0 = 12;
pub const RTN_XRESOLVE: C2RustUnnamed_0 = 11;
pub const RTN_NAT: C2RustUnnamed_0 = 10;
pub const RTN_THROW: C2RustUnnamed_0 = 9;
pub const RTN_PROHIBIT: C2RustUnnamed_0 = 8;
pub const RTN_UNREACHABLE: C2RustUnnamed_0 = 7;
pub const RTN_BLACKHOLE: C2RustUnnamed_0 = 6;
pub const RTN_MULTICAST: C2RustUnnamed_0 = 5;
pub const RTN_ANYCAST: C2RustUnnamed_0 = 4;
pub const RTN_BROADCAST: C2RustUnnamed_0 = 3;
pub const RTN_LOCAL: C2RustUnnamed_0 = 2;
pub const RTN_UNICAST: C2RustUnnamed_0 = 1;
pub const RTN_UNSPEC: C2RustUnnamed_0 = 0;
pub type rt_scope_t = libc::c_uint;
pub const RT_SCOPE_NOWHERE: rt_scope_t = 255;
pub const RT_SCOPE_HOST: rt_scope_t = 254;
pub const RT_SCOPE_LINK: rt_scope_t = 253;
pub const RT_SCOPE_SITE: rt_scope_t = 200;
pub const RT_SCOPE_UNIVERSE: rt_scope_t = 0;
pub type rt_class_t = libc::c_uint;
pub const RT_TABLE_MAX: rt_class_t = 4294967295;
pub const RT_TABLE_LOCAL: rt_class_t = 255;
pub const RT_TABLE_MAIN: rt_class_t = 254;
pub const RT_TABLE_DEFAULT: rt_class_t = 253;
pub const RT_TABLE_COMPAT: rt_class_t = 252;
pub const RT_TABLE_UNSPEC: rt_class_t = 0;
pub type rtattr_type_t = libc::c_uint;
pub const __RTA_MAX: rtattr_type_t = 27;
pub const RTA_TTL_PROPAGATE: rtattr_type_t = 26;
pub const RTA_UID: rtattr_type_t = 25;
pub const RTA_PAD: rtattr_type_t = 24;
pub const RTA_EXPIRES: rtattr_type_t = 23;
pub const RTA_ENCAP: rtattr_type_t = 22;
pub const RTA_ENCAP_TYPE: rtattr_type_t = 21;
pub const RTA_PREF: rtattr_type_t = 20;
pub const RTA_NEWDST: rtattr_type_t = 19;
pub const RTA_VIA: rtattr_type_t = 18;
pub const RTA_MFC_STATS: rtattr_type_t = 17;
pub const RTA_MARK: rtattr_type_t = 16;
pub const RTA_TABLE: rtattr_type_t = 15;
pub const RTA_MP_ALGO: rtattr_type_t = 14;
pub const RTA_SESSION: rtattr_type_t = 13;
pub const RTA_CACHEINFO: rtattr_type_t = 12;
pub const RTA_FLOW: rtattr_type_t = 11;
pub const RTA_PROTOINFO: rtattr_type_t = 10;
pub const RTA_MULTIPATH: rtattr_type_t = 9;
pub const RTA_METRICS: rtattr_type_t = 8;
pub const RTA_PREFSRC: rtattr_type_t = 7;
pub const RTA_PRIORITY: rtattr_type_t = 6;
pub const RTA_GATEWAY: rtattr_type_t = 5;
pub const RTA_OIF: rtattr_type_t = 4;
pub const RTA_IIF: rtattr_type_t = 3;
pub const RTA_SRC: rtattr_type_t = 2;
pub const RTA_DST: rtattr_type_t = 1;
pub const RTA_UNSPEC: rtattr_type_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rta_cacheinfo {
  pub rta_clntref: u32,
  pub rta_lastuse: u32,
  pub rta_expires: __s32,
  pub rta_error: u32,
  pub rta_used: u32,
  pub rta_id: u32,
  pub rta_ts: u32,
  pub rta_tsage: u32,
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub const __RTAX_MAX: C2RustUnnamed_1 = 18;
pub const RTAX_FASTOPEN_NO_COOKIE: C2RustUnnamed_1 = 17;
pub const RTAX_CC_ALGO: C2RustUnnamed_1 = 16;
pub const RTAX_QUICKACK: C2RustUnnamed_1 = 15;
pub const RTAX_INITRWND: C2RustUnnamed_1 = 14;
pub const RTAX_RTO_MIN: C2RustUnnamed_1 = 13;
pub const RTAX_FEATURES: C2RustUnnamed_1 = 12;
pub const RTAX_INITCWND: C2RustUnnamed_1 = 11;
pub const RTAX_HOPLIMIT: C2RustUnnamed_1 = 10;
pub const RTAX_REORDERING: C2RustUnnamed_1 = 9;
pub const RTAX_ADVMSS: C2RustUnnamed_1 = 8;
pub const RTAX_CWND: C2RustUnnamed_1 = 7;
pub const RTAX_SSTHRESH: C2RustUnnamed_1 = 6;
pub const RTAX_RTTVAR: C2RustUnnamed_1 = 5;
pub const RTAX_RTT: C2RustUnnamed_1 = 4;
pub const RTAX_WINDOW: C2RustUnnamed_1 = 3;
pub const RTAX_MTU: C2RustUnnamed_1 = 2;
pub const RTAX_LOCK: C2RustUnnamed_1 = 1;
pub const RTAX_UNSPEC: C2RustUnnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
  pub n: nlmsghdr,
  pub r: rtmsg,
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
pub const gw_ok: C2RustUnnamed_3 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct inet_prefix {
  pub family: u8,
  pub bytelen: u8,
  pub bitlen: i16,
  pub data: [u32; 4],
}
pub const dst_ok: C2RustUnnamed_3 = 2;
pub const type_ok: C2RustUnnamed_3 = 8;
pub const ARG_to: C2RustUnnamed_4 = 9;
pub const ARG_onlink: C2RustUnnamed_4 = 11;
pub const ARG_metric: C2RustUnnamed_4 = 10;
pub const ARG_oif: C2RustUnnamed_4 = 8;
pub const ARG_dev: C2RustUnnamed_4 = 7;
pub const ARG_table: C2RustUnnamed_4 = 6;
pub const proto_ok: C2RustUnnamed_3 = 4;
pub const ARG_protocol: C2RustUnnamed_4 = 5;
pub const ARG_scope: C2RustUnnamed_4 = 4;
pub const ARG_advmss: C2RustUnnamed_4 = 3;
pub const ARG_mtu: C2RustUnnamed_4 = 2;
pub const ARG_via: C2RustUnnamed_4 = 1;
pub const ARG_src: C2RustUnnamed_4 = 0;
pub type C2RustUnnamed_3 = libc::c_uint;
pub type C2RustUnnamed_4 = libc::c_uint;

/*
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 *
 * Authors: Alexey Kuznetsov, <kuznet@ms2.inr.ac.ru>
 *
 * Changes:
 *
 * Rani Assaf <rani@magic.metawire.com> 980929: resolve addresses
 * Kunihiro Ishiguro <kunihiro@zebra.org> 001102: rtnh_ifindex was not initialized
 */
/* RTA_TABLE is not a define, can't test with ifdef. */
/* As a proxy, test which kernels toolchain expects: */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct filter_t {
  pub tb: libc::c_int,
  pub flushed: smallint,
  pub flushb: *mut libc::c_char,
  pub flushp: libc::c_int,
  pub flushe: libc::c_int,
  pub rth: *mut rtnl_handle,
  pub scope: libc::c_int,
  pub scopemask: libc::c_int,
  pub iif: libc::c_int,
  pub oif: libc::c_int,
  pub rvia: inet_prefix,
  pub rdst: inet_prefix,
  pub mdst: inet_prefix,
  pub rsrc: inet_prefix,
  pub msrc: inet_prefix,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
  pub nlh: nlmsghdr,
  pub rtm: rtmsg,
}
pub const KW_exact: C2RustUnnamed_6 = 13;
pub const KW_match: C2RustUnnamed_6 = 12;
pub const KW_root: C2RustUnnamed_6 = 11;
pub const KW_to: C2RustUnnamed_6 = 8;
pub const KW_from: C2RustUnnamed_6 = 7;
pub const KW_scope: C2RustUnnamed_6 = 9;
pub const KW_cache: C2RustUnnamed_6 = 6;
pub const KW_main: C2RustUnnamed_6 = 14;
/* */
pub const KW_all: C2RustUnnamed_6 = 10;
pub const KW_table: C2RustUnnamed_6 = 5;
pub const KW_via: C2RustUnnamed_6 = 4;
pub const KW_iif: C2RustUnnamed_6 = 3;
pub const KW_oif: C2RustUnnamed_6 = 2;
pub const KW_dev: C2RustUnnamed_6 = 1;
pub const KW_proto: C2RustUnnamed_6 = 0;
pub type C2RustUnnamed_6 = libc::c_uint;
pub const CMD_flush: C2RustUnnamed_8 = 12;
pub const CMD_test: C2RustUnnamed_8 = 11;
pub const CMD_replace: C2RustUnnamed_8 = 10;
pub const CMD_prepend: C2RustUnnamed_8 = 9;
pub const CMD_show: C2RustUnnamed_8 = 8;
pub const CMD_list: C2RustUnnamed_8 = 7;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
  pub n: nlmsghdr,
  pub r: rtmsg,
  pub buf: [libc::c_char; 1024],
}
pub const CMD_get: C2RustUnnamed_8 = 6;
pub const CMD_delete: C2RustUnnamed_8 = 5;
pub const CMD_chg: C2RustUnnamed_8 = 4;
pub const CMD_change: C2RustUnnamed_8 = 3;
pub const CMD_append: C2RustUnnamed_8 = 2;
pub const CMD_add: C2RustUnnamed_8 = 1;
pub const CMD_a: C2RustUnnamed_8 = 0;
pub type C2RustUnnamed_8 = libc::c_uint;
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
unsafe extern "C" fn print_route(
  mut _who: *const sockaddr_nl,
  mut n: *mut nlmsghdr,
  mut _arg: *mut libc::c_void,
) -> libc::c_int {
  let mut r: *mut rtmsg = (n as *mut libc::c_char).offset(
    (0i32
      + ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
        .wrapping_add(4u32 as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong)
        & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong) as libc::c_int) as isize,
  ) as *mut libc::c_void as *mut rtmsg;
  let mut len: libc::c_int = (*n).nlmsg_len as libc::c_int;
  let mut tb: [*mut rtattr; 27] = [0 as *mut rtattr; 27];
  let mut dst: inet_prefix = inet_prefix {
    family: 0,
    bytelen: 0,
    bitlen: 0,
    data: [0; 4],
  };
  let mut src: inet_prefix = inet_prefix {
    family: 0,
    bytelen: 0,
    bitlen: 0,
    data: [0; 4],
  };
  let mut host_len: libc::c_int = -1i32;
  let mut tid: u32 = 0;
  if (*n).nlmsg_type as libc::c_int != RTM_NEWROUTE as libc::c_int
    && (*n).nlmsg_type as libc::c_int != RTM_DELROUTE as libc::c_int
  {
    fprintf(
      stderr,
      b"Not a route: %08x %08x %08x\n\x00" as *const u8 as *const libc::c_char,
      (*n).nlmsg_len,
      (*n).nlmsg_type as libc::c_int,
      (*n).nlmsg_flags as libc::c_int,
    );
    return 0i32;
  }
  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t))
    .flushb
    .is_null()
    && (*n).nlmsg_type as libc::c_int != RTM_NEWROUTE as libc::c_int
  {
    return 0i32;
  }
  len = (len as libc::c_ulong).wrapping_sub(
    (::std::mem::size_of::<rtmsg>() as libc::c_ulong).wrapping_add(
      ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
        .wrapping_add(4u32 as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong)
        & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong) as libc::c_int
        as libc::c_ulong,
    ),
  ) as libc::c_int as libc::c_int;
  if len < 0i32 {
    bb_error_msg_and_die(
      b"wrong nlmsg len %d\x00" as *const u8 as *const libc::c_char,
      len,
    );
  }
  //memset(tb, 0, sizeof(tb)); - parse_rtattr does this
  parse_rtattr(
    tb.as_mut_ptr(),
    __RTA_MAX as libc::c_int - 1i32,
    (r as *mut libc::c_char).offset(
      ((::std::mem::size_of::<rtmsg>() as libc::c_ulong)
        .wrapping_add(4u32 as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong)
        & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong) as isize,
    ) as *mut rtattr,
    len,
  );
  if !tb[RTA_TABLE as libc::c_int as usize].is_null() {
    tid = *((tb[RTA_TABLE as libc::c_int as usize] as *mut libc::c_char).offset(
      ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
        .wrapping_add(4u32 as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong)
        & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
        .wrapping_add(0i32 as libc::c_ulong) as isize,
    ) as *mut libc::c_void as *mut u32)
  } else {
    tid = (*r).rtm_table as u32
  }
  if (*r).rtm_family as libc::c_int == 10i32 {
    host_len = 128i32
  } else if (*r).rtm_family as libc::c_int == 2i32 {
    host_len = 32i32
  }
  if (*r).rtm_family as libc::c_int == 10i32 {
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).tb != 0 {
      if (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).tb < 0i32 {
        if (*r).rtm_flags & 0x200i32 as libc::c_uint == 0 {
          return 0i32;
        }
      } else {
        if (*r).rtm_flags & 0x200i32 as libc::c_uint != 0 {
          return 0i32;
        }
        if (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).tb == RT_TABLE_LOCAL as libc::c_int
        {
          if (*r).rtm_type as libc::c_int != RTN_LOCAL as libc::c_int {
            return 0i32;
          }
        } else if (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).tb
          == RT_TABLE_MAIN as libc::c_int
        {
          if (*r).rtm_type as libc::c_int == RTN_LOCAL as libc::c_int {
            return 0i32;
          }
        } else {
          return 0i32;
        }
      }
    }
  } else if (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).tb > 0i32
    && (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).tb as libc::c_uint != tid
  {
    return 0i32;
  }
  if ((*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).scope ^ (*r).rtm_scope as libc::c_int)
    & (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).scopemask
    != 0
  {
    return 0i32;
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t))
    .rdst
    .family as libc::c_int
    != 0
    && ((*r).rtm_family as libc::c_int
      != (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t))
        .rdst
        .family as libc::c_int
      || (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t))
        .rdst
        .bitlen as libc::c_int
        > (*r).rtm_dst_len as libc::c_int)
  {
    return 0i32;
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t))
    .mdst
    .family as libc::c_int
    != 0
    && ((*r).rtm_family as libc::c_int
      != (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t))
        .mdst
        .family as libc::c_int
      || (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t))
        .mdst
        .bitlen as libc::c_int
        >= 0i32
        && ((*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t))
          .mdst
          .bitlen as libc::c_int)
          < (*r).rtm_dst_len as libc::c_int)
  {
    return 0i32;
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t))
    .rsrc
    .family as libc::c_int
    != 0
    && ((*r).rtm_family as libc::c_int
      != (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t))
        .rsrc
        .family as libc::c_int
      || (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t))
        .rsrc
        .bitlen as libc::c_int
        > (*r).rtm_src_len as libc::c_int)
  {
    return 0i32;
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t))
    .msrc
    .family as libc::c_int
    != 0
    && ((*r).rtm_family as libc::c_int
      != (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t))
        .msrc
        .family as libc::c_int
      || (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t))
        .msrc
        .bitlen as libc::c_int
        >= 0i32
        && ((*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t))
          .msrc
          .bitlen as libc::c_int)
          < (*r).rtm_src_len as libc::c_int)
  {
    return 0i32;
  }
  memset(
    &mut src as *mut inet_prefix as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<inet_prefix>() as libc::c_ulong,
  );
  memset(
    &mut dst as *mut inet_prefix as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<inet_prefix>() as libc::c_ulong,
  );
  if !tb[RTA_SRC as libc::c_int as usize].is_null() {
    src.bitlen = (*r).rtm_src_len as i16;
    src.bytelen = if (*r).rtm_family as libc::c_int == 10i32 {
      16i32
    } else {
      4i32
    } as u8;
    memcpy(
      src.data.as_mut_ptr() as *mut libc::c_void,
      (tb[RTA_SRC as libc::c_int as usize] as *mut libc::c_char).offset(
        ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
          .wrapping_add(4u32 as libc::c_ulong)
          .wrapping_sub(1i32 as libc::c_ulong)
          & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
          .wrapping_add(0i32 as libc::c_ulong) as isize,
      ) as *mut libc::c_void,
      src.bytelen as libc::c_ulong,
    );
  }
  if !tb[RTA_DST as libc::c_int as usize].is_null() {
    dst.bitlen = (*r).rtm_dst_len as i16;
    dst.bytelen = if (*r).rtm_family as libc::c_int == 10i32 {
      16i32
    } else {
      4i32
    } as u8;
    memcpy(
      dst.data.as_mut_ptr() as *mut libc::c_void,
      (tb[RTA_DST as libc::c_int as usize] as *mut libc::c_char).offset(
        ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
          .wrapping_add(4u32 as libc::c_ulong)
          .wrapping_sub(1i32 as libc::c_ulong)
          & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
          .wrapping_add(0i32 as libc::c_ulong) as isize,
      ) as *mut libc::c_void,
      dst.bytelen as libc::c_ulong,
    );
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t))
    .rdst
    .family as libc::c_int
    != 0
    && inet_addr_match(
      &mut dst,
      &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).rdst,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t))
        .rdst
        .bitlen as libc::c_int,
    ) != 0
  {
    return 0i32;
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t))
    .mdst
    .family as libc::c_int
    != 0
    && (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t))
      .mdst
      .bitlen as libc::c_int
      >= 0i32
    && inet_addr_match(
      &mut dst,
      &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).mdst,
      (*r).rtm_dst_len as libc::c_int,
    ) != 0
  {
    return 0i32;
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t))
    .rsrc
    .family as libc::c_int
    != 0
    && inet_addr_match(
      &mut src,
      &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).rsrc,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t))
        .rsrc
        .bitlen as libc::c_int,
    ) != 0
  {
    return 0i32;
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t))
    .msrc
    .family as libc::c_int
    != 0
    && (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t))
      .msrc
      .bitlen as libc::c_int
      >= 0i32
    && inet_addr_match(
      &mut src,
      &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).msrc,
      (*r).rtm_src_len as libc::c_int,
    ) != 0
  {
    return 0i32;
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).oif != 0i32 {
    if tb[RTA_OIF as libc::c_int as usize].is_null() {
      return 0i32;
    }
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).oif
      != *((tb[RTA_OIF as libc::c_int as usize] as *mut libc::c_char).offset(
        ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
          .wrapping_add(4u32 as libc::c_ulong)
          .wrapping_sub(1i32 as libc::c_ulong)
          & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
          .wrapping_add(0i32 as libc::c_ulong) as isize,
      ) as *mut libc::c_void as *mut libc::c_int)
    {
      return 0i32;
    }
  }
  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t))
    .flushb
    .is_null()
  {
    let mut fn_0: *mut nlmsghdr = 0 as *mut nlmsghdr;
    /* We are creating route flush commands */
    if (*r).rtm_family as libc::c_int == 10i32
      && (*r).rtm_dst_len as libc::c_int == 0i32
      && (*r).rtm_type as libc::c_int == RTN_UNREACHABLE as libc::c_int
      && !tb[RTA_PRIORITY as libc::c_int as usize].is_null()
      && *((tb[RTA_PRIORITY as libc::c_int as usize] as *mut libc::c_char).offset(
        ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
          .wrapping_add(4u32 as libc::c_ulong)
          .wrapping_sub(1i32 as libc::c_ulong)
          & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
          .wrapping_add(0i32 as libc::c_ulong) as isize,
      ) as *mut libc::c_void as *mut libc::c_int)
        == -1i32
    {
      return 0i32;
    }
    if (((*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).flushp as libc::c_uint)
      .wrapping_add(4u32)
      .wrapping_sub(1i32 as libc::c_uint)
      & !4u32.wrapping_sub(1i32 as libc::c_uint))
    .wrapping_add((*n).nlmsg_len)
      > (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).flushe as libc::c_uint
    {
      if flush_update() != 0 {
        xfunc_die();
      }
    }
    fn_0 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t))
      .flushb
      .offset(
        (((*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).flushp as libc::c_uint)
          .wrapping_add(4u32)
          .wrapping_sub(1i32 as libc::c_uint)
          & !4u32.wrapping_sub(1i32 as libc::c_uint)) as isize,
      ) as *mut libc::c_void as *mut nlmsghdr;
    memcpy(
      fn_0 as *mut libc::c_void,
      n as *const libc::c_void,
      (*n).nlmsg_len as libc::c_ulong,
    );
    (*fn_0).nlmsg_type = RTM_DELROUTE as libc::c_int as __u16;
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
  /* We are printing routes */
  if (*n).nlmsg_type as libc::c_int == RTM_DELROUTE as libc::c_int {
    printf(b"Deleted \x00" as *const u8 as *const libc::c_char);
  }
  if (*r).rtm_type as libc::c_int != RTN_UNICAST as libc::c_int {
    /* && !G_filter.type - always 0 */
    printf(
      b"%s \x00" as *const u8 as *const libc::c_char,
      rtnl_rtntype_n2a((*r).rtm_type as libc::c_int),
    );
  }
  if !tb[RTA_DST as libc::c_int as usize].is_null() {
    if (*r).rtm_dst_len as libc::c_int != host_len {
      printf(
        b"%s/%u \x00" as *const u8 as *const libc::c_char,
        rt_addr_n2a(
          (*r).rtm_family as libc::c_int,
          (tb[RTA_DST as libc::c_int as usize] as *mut libc::c_char).offset(
            ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
              .wrapping_add(4u32 as libc::c_ulong)
              .wrapping_sub(1i32 as libc::c_ulong)
              & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
              .wrapping_add(0i32 as libc::c_ulong) as isize,
          ) as *mut libc::c_void,
        ),
        (*r).rtm_dst_len as libc::c_int,
      );
    } else {
      printf(
        b"%s \x00" as *const u8 as *const libc::c_char,
        rt_addr_n2a(
          (*r).rtm_family as libc::c_int,
          (tb[RTA_DST as libc::c_int as usize] as *mut libc::c_char).offset(
            ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
              .wrapping_add(4u32 as libc::c_ulong)
              .wrapping_sub(1i32 as libc::c_ulong)
              & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
              .wrapping_add(0i32 as libc::c_ulong) as isize,
          ) as *mut libc::c_void,
        ),
      );
    }
  } else if (*r).rtm_dst_len != 0 {
    printf(
      b"0/%d \x00" as *const u8 as *const libc::c_char,
      (*r).rtm_dst_len as libc::c_int,
    );
  } else {
    printf(b"default \x00" as *const u8 as *const libc::c_char);
  }
  if !tb[RTA_SRC as libc::c_int as usize].is_null() {
    if (*r).rtm_src_len as libc::c_int != host_len {
      printf(
        b"from %s/%u \x00" as *const u8 as *const libc::c_char,
        rt_addr_n2a(
          (*r).rtm_family as libc::c_int,
          (tb[RTA_SRC as libc::c_int as usize] as *mut libc::c_char).offset(
            ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
              .wrapping_add(4u32 as libc::c_ulong)
              .wrapping_sub(1i32 as libc::c_ulong)
              & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
              .wrapping_add(0i32 as libc::c_ulong) as isize,
          ) as *mut libc::c_void,
        ),
        (*r).rtm_src_len as libc::c_int,
      );
    } else {
      printf(
        b"from %s \x00" as *const u8 as *const libc::c_char,
        rt_addr_n2a(
          (*r).rtm_family as libc::c_int,
          (tb[RTA_SRC as libc::c_int as usize] as *mut libc::c_char).offset(
            ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
              .wrapping_add(4u32 as libc::c_ulong)
              .wrapping_sub(1i32 as libc::c_ulong)
              & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
              .wrapping_add(0i32 as libc::c_ulong) as isize,
          ) as *mut libc::c_void,
        ),
      );
    }
  } else if (*r).rtm_src_len != 0 {
    printf(
      b"from 0/%u \x00" as *const u8 as *const libc::c_char,
      (*r).rtm_src_len as libc::c_int,
    );
  }
  if !tb[RTA_GATEWAY as libc::c_int as usize].is_null()
    && (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t))
      .rvia
      .bitlen as libc::c_int
      != host_len
  {
    printf(
      b"via %s \x00" as *const u8 as *const libc::c_char,
      rt_addr_n2a(
        (*r).rtm_family as libc::c_int,
        (tb[RTA_GATEWAY as libc::c_int as usize] as *mut libc::c_char).offset(
          ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
            .wrapping_add(4u32 as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong)
            & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
            .wrapping_add(0i32 as libc::c_ulong) as isize,
        ) as *mut libc::c_void,
      ),
    );
  }
  if !tb[RTA_OIF as libc::c_int as usize].is_null() {
    printf(
      b"dev %s \x00" as *const u8 as *const libc::c_char,
      ll_index_to_name(
        *((tb[RTA_OIF as libc::c_int as usize] as *mut libc::c_char).offset(
          ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
            .wrapping_add(4u32 as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong)
            & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
            .wrapping_add(0i32 as libc::c_ulong) as isize,
        ) as *mut libc::c_void as *mut libc::c_int),
      ),
    );
  }
  if tid != 0
    && tid != RT_TABLE_MAIN as libc::c_int as libc::c_uint
    && (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).tb == 0
  {
    printf(
      b"table %s \x00" as *const u8 as *const libc::c_char,
      rtnl_rttable_n2a(tid as libc::c_int),
    );
  }
  /* Todo: parse & show "proto kernel" here */
  if (*r).rtm_flags & 0x200i32 as libc::c_uint == 0 {
    if (*r).rtm_scope as libc::c_int != RT_SCOPE_UNIVERSE as libc::c_int
      && (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).scopemask != -1i32
    {
      printf(
        b"scope %s \x00" as *const u8 as *const libc::c_char,
        rtnl_rtscope_n2a((*r).rtm_scope as libc::c_int),
      );
    }
  }
  if !tb[RTA_PREFSRC as libc::c_int as usize].is_null() && 0i32 != host_len {
    /* Do not use format_host(). It is our local addr
      and symbolic name will not be useful.
    */
    printf(
      b" src %s \x00" as *const u8 as *const libc::c_char,
      rt_addr_n2a(
        (*r).rtm_family as libc::c_int,
        (tb[RTA_PREFSRC as libc::c_int as usize] as *mut libc::c_char).offset(
          ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
            .wrapping_add(4u32 as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong)
            & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
            .wrapping_add(0i32 as libc::c_ulong) as isize,
        ) as *mut libc::c_void,
      ),
    );
  }
  if !tb[RTA_PRIORITY as libc::c_int as usize].is_null() {
    printf(
      b" metric %d \x00" as *const u8 as *const libc::c_char,
      *((tb[RTA_PRIORITY as libc::c_int as usize] as *mut libc::c_char).offset(
        ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
          .wrapping_add(4u32 as libc::c_ulong)
          .wrapping_sub(1i32 as libc::c_ulong)
          & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
          .wrapping_add(0i32 as libc::c_ulong) as isize,
      ) as *mut libc::c_void as *mut u32),
    );
  }
  if (*r).rtm_flags & 1i32 as libc::c_uint != 0 {
    printf(b"dead \x00" as *const u8 as *const libc::c_char);
  }
  if (*r).rtm_flags & 4i32 as libc::c_uint != 0 {
    printf(b"onlink \x00" as *const u8 as *const libc::c_char);
  }
  if (*r).rtm_flags & 2i32 as libc::c_uint != 0 {
    printf(b"pervasive \x00" as *const u8 as *const libc::c_char);
  }
  if (*r).rtm_flags & 0x100i32 as libc::c_uint != 0 {
    printf(b"notify \x00" as *const u8 as *const libc::c_char);
  }
  if (*r).rtm_family as libc::c_int == 10i32 {
    let mut ci: *mut rta_cacheinfo = 0 as *mut rta_cacheinfo;
    if !tb[RTA_CACHEINFO as libc::c_int as usize].is_null() {
      ci = (tb[RTA_CACHEINFO as libc::c_int as usize] as *mut libc::c_char).offset(
        ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
          .wrapping_add(4u32 as libc::c_ulong)
          .wrapping_sub(1i32 as libc::c_ulong)
          & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
          .wrapping_add(0i32 as libc::c_ulong) as isize,
      ) as *mut libc::c_void as *mut rta_cacheinfo
    }
    if (*r).rtm_flags & 0x200i32 as libc::c_uint != 0 || !ci.is_null() && (*ci).rta_expires != 0 {
      if (*r).rtm_flags & 0x200i32 as libc::c_uint != 0 {
        printf(
          b"%c    cache \x00" as *const u8 as *const libc::c_char,
          _SL_ as libc::c_int,
        );
      }
      if (*ci).rta_expires != 0 {
        printf(
          b" expires %dsec\x00" as *const u8 as *const libc::c_char,
          ((*ci).rta_expires as libc::c_uint).wrapping_div(get_hz()),
        );
      }
      if (*ci).rta_error != 0i32 as libc::c_uint {
        printf(
          b" error %d\x00" as *const u8 as *const libc::c_char,
          (*ci).rta_error,
        );
      }
    } else if !ci.is_null() {
      if (*ci).rta_error != 0i32 as libc::c_uint {
        printf(
          b" error %d\x00" as *const u8 as *const libc::c_char,
          (*ci).rta_error,
        );
      }
    }
  }
  if !tb[RTA_IIF as libc::c_int as usize].is_null()
    && (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).iif == 0i32
  {
    printf(
      b" iif %s\x00" as *const u8 as *const libc::c_char,
      ll_index_to_name(
        *((tb[RTA_IIF as libc::c_int as usize] as *mut libc::c_char).offset(
          ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
            .wrapping_add(4u32 as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong)
            & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
            .wrapping_add(0i32 as libc::c_ulong) as isize,
        ) as *mut libc::c_void as *mut libc::c_int),
      ),
    );
  }
  bb_putchar('\n' as i32);
  return 0i32;
}
unsafe extern "C" fn str_is_lock(mut str: *const libc::c_char) -> libc::c_int {
  return (strcmp(str, b"lock\x00" as *const u8 as *const libc::c_char) == 0i32) as libc::c_int;
}
/* Return value becomes exitcode. It's okay to not return at all */
unsafe extern "C" fn iproute_modify(
  mut cmd: libc::c_int,
  mut flags: libc::c_uint,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  /* If you add stuff here, update iproute_full_usage */
  static mut keywords: [libc::c_char; 66] = [
    115, 114, 99, 0, 118, 105, 97, 0, 109, 116, 117, 0, 97, 100, 118, 109, 115, 115, 0, 115, 99,
    111, 112, 101, 0, 112, 114, 111, 116, 111, 99, 111, 108, 0, 116, 97, 98, 108, 101, 0, 100, 101,
    118, 0, 111, 105, 102, 0, 116, 111, 0, 109, 101, 116, 114, 105, 99, 0, 111, 110, 108, 105, 110,
    107, 0, 0,
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
  let mut req: C2RustUnnamed_2 = C2RustUnnamed_2 {
    n: nlmsghdr {
      nlmsg_len: 0,
      nlmsg_type: 0,
      nlmsg_flags: 0,
      nlmsg_seq: 0,
      nlmsg_pid: 0,
    },
    r: rtmsg {
      rtm_family: 0,
      rtm_dst_len: 0,
      rtm_src_len: 0,
      rtm_tos: 0,
      rtm_table: 0,
      rtm_protocol: 0,
      rtm_scope: 0,
      rtm_type: 0,
      rtm_flags: 0,
    },
    buf: [0; 1024],
  };
  let mut mxbuf: [libc::c_char; 256] = [0; 256];
  let mut mxrta: *mut rtattr = mxbuf.as_mut_ptr() as *mut libc::c_void as *mut rtattr;
  let mut mxlock: libc::c_uint = 0i32 as libc::c_uint;
  let mut d: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut ok: smalluint = 0i32 as smalluint;
  let mut scope_ok: smalluint = 0i32 as smalluint;
  let mut arg: libc::c_int = 0;
  memset(
    &mut req as *mut C2RustUnnamed_2 as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<C2RustUnnamed_2>() as libc::c_ulong,
  );
  req.n.nlmsg_len = (::std::mem::size_of::<rtmsg>() as libc::c_ulong).wrapping_add(
    ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
      .wrapping_add(4u32 as libc::c_ulong)
      .wrapping_sub(1i32 as libc::c_ulong)
      & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong) as libc::c_int
      as libc::c_ulong,
  ) as u32;
  req.n.nlmsg_flags = (0x1i32 as libc::c_uint | flags) as __u16;
  req.n.nlmsg_type = cmd as __u16;
  req.r.rtm_family = preferred_family as libc::c_uchar;
  if RT_TABLE_MAIN as libc::c_int != 0i32 {
    /* if it is zero, memset already did it */
    req.r.rtm_table = RT_TABLE_MAIN as libc::c_int as libc::c_uchar
  }
  if RT_SCOPE_NOWHERE as libc::c_int != 0i32 {
    req.r.rtm_scope = RT_SCOPE_NOWHERE as libc::c_int as libc::c_uchar
  }
  if cmd != RTM_DELROUTE as libc::c_int {
    req.r.rtm_scope = RT_SCOPE_UNIVERSE as libc::c_int as libc::c_uchar;
    if 3i32 != 0i32 {
      req.r.rtm_protocol = 3i32 as libc::c_uchar
    }
    if RTN_UNICAST as libc::c_int != 0i32 {
      req.r.rtm_type = RTN_UNICAST as libc::c_int as libc::c_uchar
    }
  }
  (*mxrta).rta_type = RTA_METRICS as libc::c_int as libc::c_ushort;
  (*mxrta).rta_len = ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
    .wrapping_add(4u32 as libc::c_ulong)
    .wrapping_sub(1i32 as libc::c_ulong)
    & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
    .wrapping_add(0i32 as libc::c_ulong) as libc::c_ushort;
  while !(*argv).is_null() {
    arg = index_in_substrings(keywords.as_ptr(), *argv);
    if arg == ARG_src as libc::c_int {
      let mut addr: inet_prefix = inet_prefix {
        family: 0,
        bytelen: 0,
        bitlen: 0,
        data: [0; 4],
      };
      argv = next_arg(argv);
      get_addr(&mut addr, *argv, req.r.rtm_family as libc::c_int);
      if req.r.rtm_family as libc::c_int == 0i32 {
        req.r.rtm_family = addr.family
      }
      addattr_l(
        &mut req.n,
        ::std::mem::size_of::<C2RustUnnamed_2>() as libc::c_ulong as libc::c_int,
        RTA_PREFSRC as libc::c_int,
        &mut addr.data as *mut [u32; 4] as *mut libc::c_void,
        addr.bytelen as libc::c_int,
      );
    } else if arg == ARG_via as libc::c_int {
      let mut addr_0: inet_prefix = inet_prefix {
        family: 0,
        bytelen: 0,
        bitlen: 0,
        data: [0; 4],
      };
      ok = (ok as libc::c_int | gw_ok as libc::c_int) as smalluint;
      argv = next_arg(argv);
      get_addr(&mut addr_0, *argv, req.r.rtm_family as libc::c_int);
      if req.r.rtm_family as libc::c_int == 0i32 {
        req.r.rtm_family = addr_0.family
      }
      addattr_l(
        &mut req.n,
        ::std::mem::size_of::<C2RustUnnamed_2>() as libc::c_ulong as libc::c_int,
        RTA_GATEWAY as libc::c_int,
        &mut addr_0.data as *mut [u32; 4] as *mut libc::c_void,
        addr_0.bytelen as libc::c_int,
      );
    } else if arg == ARG_mtu as libc::c_int {
      let mut mtu: libc::c_uint = 0;
      argv = next_arg(argv);
      if str_is_lock(*argv) != 0 {
        mxlock |= (1i32 << RTAX_MTU as libc::c_int) as libc::c_uint;
        argv = next_arg(argv)
      }
      mtu = get_unsigned(
        *argv,
        keywords
          .as_ptr()
          .offset(::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong as isize)
          .offset(::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong as isize),
      );
      rta_addattr32(
        mxrta,
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
        RTAX_MTU as libc::c_int,
        mtu,
      );
    } else if arg == ARG_advmss as libc::c_int {
      let mut mss: libc::c_uint = 0;
      argv = next_arg(argv);
      if str_is_lock(*argv) != 0 {
        mxlock |= (1i32 << RTAX_ADVMSS as libc::c_int) as libc::c_uint;
        argv = next_arg(argv)
      }
      mss = get_unsigned(
        *argv,
        keywords
          .as_ptr()
          .offset(::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong as isize)
          .offset(::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong as isize)
          .offset(::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong as isize),
      );
      rta_addattr32(
        mxrta,
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
        RTAX_ADVMSS as libc::c_int,
        mss,
      );
    } else if arg == ARG_scope as libc::c_int {
      let mut scope: u32 = 0;
      argv = next_arg(argv);
      if rtnl_rtscope_a2n(&mut scope, *argv) != 0 {
        invarg_1_to_2(
          *argv,
          keywords
            .as_ptr()
            .offset(::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong as isize)
            .offset(::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong as isize)
            .offset(::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong as isize)
            .offset(::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as isize),
        );
      }
      req.r.rtm_scope = scope as libc::c_uchar;
      scope_ok = 1i32 as smalluint
    } else if arg == ARG_protocol as libc::c_int {
      let mut prot: u32 = 0;
      argv = next_arg(argv);
      if rtnl_rtprot_a2n(&mut prot, *argv) != 0 {
        invarg_1_to_2(
          *argv,
          keywords
            .as_ptr()
            .offset(::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong as isize)
            .offset(::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong as isize)
            .offset(::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong as isize)
            .offset(::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as isize)
            .offset(::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as isize),
        );
      }
      req.r.rtm_protocol = prot as libc::c_uchar;
      ok = (ok as libc::c_int | proto_ok as libc::c_int) as smalluint
    } else if arg == ARG_table as libc::c_int {
      let mut tid: u32 = 0;
      argv = next_arg(argv);
      if rtnl_rttable_a2n(&mut tid, *argv) != 0 {
        invarg_1_to_2(
          *argv,
          keywords
            .as_ptr()
            .offset(::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong as isize)
            .offset(::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong as isize)
            .offset(::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong as isize)
            .offset(::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as isize)
            .offset(::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as isize)
            .offset(::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong as isize),
        );
      }
      if tid > 255i32 as libc::c_uint {
        req.r.rtm_table = RT_TABLE_UNSPEC as libc::c_int as libc::c_uchar;
        addattr32(
          &mut req.n,
          ::std::mem::size_of::<C2RustUnnamed_2>() as libc::c_ulong as libc::c_int,
          RTA_TABLE as libc::c_int,
          tid,
        );
      } else {
        req.r.rtm_table = tid as libc::c_uchar
      }
    } else if arg == ARG_dev as libc::c_int || arg == ARG_oif as libc::c_int {
      argv = next_arg(argv);
      d = *argv
    } else if arg == ARG_metric as libc::c_int {
      //TODO: "metric", "priority" and "preference" are synonyms
      let mut metric: u32 = 0;
      argv = next_arg(argv);
      metric = get_u32(*argv, b"metric\x00" as *const u8 as *const libc::c_char);
      addattr32(
        &mut req.n,
        ::std::mem::size_of::<C2RustUnnamed_2>() as libc::c_ulong as libc::c_int,
        RTA_PRIORITY as libc::c_int,
        metric,
      );
    } else if arg == ARG_onlink as libc::c_int {
      req.r.rtm_flags |= 4i32 as libc::c_uint
    } else {
      let mut type_0: libc::c_int = 0;
      let mut dst: inet_prefix = inet_prefix {
        family: 0,
        bytelen: 0,
        bitlen: 0,
        data: [0; 4],
      };
      if arg == ARG_to as libc::c_int {
        argv = next_arg(argv)
      }
      if ((**argv as libc::c_int) < '0' as i32 || **argv as libc::c_int > '9' as i32)
        && rtnl_rtntype_a2n(&mut type_0, *argv) == 0i32
      {
        argv = next_arg(argv);
        req.r.rtm_type = type_0 as libc::c_uchar;
        ok = (ok as libc::c_int | type_ok as libc::c_int) as smalluint
      }
      if ok as libc::c_int & dst_ok as libc::c_int != 0 {
        duparg2(b"to\x00" as *const u8 as *const libc::c_char, *argv);
      }
      get_prefix(&mut dst, *argv, req.r.rtm_family as libc::c_int);
      if req.r.rtm_family as libc::c_int == 0i32 {
        req.r.rtm_family = dst.family
      }
      req.r.rtm_dst_len = dst.bitlen as libc::c_uchar;
      ok = (ok as libc::c_int | dst_ok as libc::c_int) as smalluint;
      if dst.bytelen != 0 {
        addattr_l(
          &mut req.n,
          ::std::mem::size_of::<C2RustUnnamed_2>() as libc::c_ulong as libc::c_int,
          RTA_DST as libc::c_int,
          &mut dst.data as *mut [u32; 4] as *mut libc::c_void,
          dst.bytelen as libc::c_int,
        );
      }
    }
    /* Other keywords recognized by iproute2-3.19.0: */
    argv = argv.offset(1)
  }
  xrtnl_open(&mut rth);
  if !d.is_null() {
    let mut idx: libc::c_int = 0;
    ll_init_map(&mut rth);
    if !d.is_null() {
      idx = xll_name_to_index(d);
      addattr32(
        &mut req.n,
        ::std::mem::size_of::<C2RustUnnamed_2>() as libc::c_ulong as libc::c_int,
        RTA_OIF as libc::c_int,
        idx as u32,
      );
    }
  }
  if (*mxrta).rta_len as libc::c_ulong
    > ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
      .wrapping_add(4u32 as libc::c_ulong)
      .wrapping_sub(1i32 as libc::c_ulong)
      & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
      .wrapping_add(0i32 as libc::c_ulong)
  {
    if mxlock != 0 {
      rta_addattr32(
        mxrta,
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
        RTAX_LOCK as libc::c_int,
        mxlock,
      );
    }
    addattr_l(
      &mut req.n,
      ::std::mem::size_of::<C2RustUnnamed_2>() as libc::c_ulong as libc::c_int,
      RTA_METRICS as libc::c_int,
      (mxrta as *mut libc::c_char).offset(
        ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
          .wrapping_add(4u32 as libc::c_ulong)
          .wrapping_sub(1i32 as libc::c_ulong)
          & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
          .wrapping_add(0i32 as libc::c_ulong) as isize,
      ) as *mut libc::c_void,
      ((*mxrta).rta_len as libc::c_int as libc::c_ulong).wrapping_sub(
        ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
          .wrapping_add(4u32 as libc::c_ulong)
          .wrapping_sub(1i32 as libc::c_ulong)
          & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
          .wrapping_add(0i32 as libc::c_ulong),
      ) as libc::c_int,
    );
  }
  if scope_ok == 0 {
    if req.r.rtm_type as libc::c_int == RTN_LOCAL as libc::c_int
      || req.r.rtm_type as libc::c_int == RTN_NAT as libc::c_int
    {
      req.r.rtm_scope = RT_SCOPE_HOST as libc::c_int as libc::c_uchar
    } else if req.r.rtm_type as libc::c_int == RTN_BROADCAST as libc::c_int
      || req.r.rtm_type as libc::c_int == RTN_MULTICAST as libc::c_int
      || req.r.rtm_type as libc::c_int == RTN_ANYCAST as libc::c_int
    {
      req.r.rtm_scope = RT_SCOPE_LINK as libc::c_int as libc::c_uchar
    } else if req.r.rtm_type as libc::c_int == RTN_UNICAST as libc::c_int
      || req.r.rtm_type as libc::c_int == RTN_UNSPEC as libc::c_int
    {
      if cmd == RTM_DELROUTE as libc::c_int {
        req.r.rtm_scope = RT_SCOPE_NOWHERE as libc::c_int as libc::c_uchar
      } else if ok as libc::c_int & gw_ok as libc::c_int == 0 {
        req.r.rtm_scope = RT_SCOPE_LINK as libc::c_int as libc::c_uchar
      }
    }
  }
  if req.r.rtm_family as libc::c_int == 0i32 {
    req.r.rtm_family = 2i32 as libc::c_uchar
  }
  if rtnl_talk(&mut rth, &mut req.n, 0 as *mut nlmsghdr) < 0i32 {
    return 2i32;
  }
  return 0i32;
}
unsafe extern "C" fn rtnl_rtcache_request(
  mut rth: *mut rtnl_handle,
  mut family: libc::c_int,
) -> libc::c_int {
  let mut req: C2RustUnnamed_5 = C2RustUnnamed_5 {
    nlh: nlmsghdr {
      nlmsg_len: 0,
      nlmsg_type: 0,
      nlmsg_flags: 0,
      nlmsg_seq: 0,
      nlmsg_pid: 0,
    },
    rtm: rtmsg {
      rtm_family: 0,
      rtm_dst_len: 0,
      rtm_src_len: 0,
      rtm_tos: 0,
      rtm_table: 0,
      rtm_protocol: 0,
      rtm_scope: 0,
      rtm_type: 0,
      rtm_flags: 0,
    },
  };
  let mut nladdr: sockaddr_nl = sockaddr_nl {
    nl_family: 0,
    nl_pad: 0,
    nl_pid: 0,
    nl_groups: 0,
  };
  memset(
    &mut nladdr as *mut sockaddr_nl as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<sockaddr_nl>() as libc::c_ulong,
  );
  memset(
    &mut req as *mut C2RustUnnamed_5 as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<C2RustUnnamed_5>() as libc::c_ulong,
  );
  nladdr.nl_family = 16i32 as __kernel_sa_family_t;
  req.nlh.nlmsg_len = ::std::mem::size_of::<C2RustUnnamed_5>() as libc::c_ulong as u32;
  if RTM_GETROUTE as libc::c_int != 0 {
    req.nlh.nlmsg_type = RTM_GETROUTE as libc::c_int as __u16
  }
  if 0x100i32 | 0x1i32 != 0 {
    req.nlh.nlmsg_flags = (0x100i32 | 0x1i32) as __u16
  }
  /*req.nlh.nlmsg_pid = 0; - memset did it already */
  (*rth).seq = (*rth).seq.wrapping_add(1);
  (*rth).dump = (*rth).seq;
  req.nlh.nlmsg_seq = (*rth).dump;
  req.rtm.rtm_family = family as libc::c_uchar;
  req.rtm.rtm_flags = 0x200i32 as libc::c_uint;
  return xsendto(
    (*rth).fd,
    &mut req as *mut C2RustUnnamed_5 as *mut libc::c_void,
    ::std::mem::size_of::<C2RustUnnamed_5>() as libc::c_ulong,
    &mut nladdr as *mut sockaddr_nl as *mut sockaddr,
    ::std::mem::size_of::<sockaddr_nl>() as libc::c_ulong as socklen_t,
  ) as libc::c_int;
}
unsafe extern "C" fn iproute_flush_cache() {
  static mut fn_0: [libc::c_char; 31] = [
    47, 112, 114, 111, 99, 47, 115, 121, 115, 47, 110, 101, 116, 47, 105, 112, 118, 52, 47, 114,
    111, 117, 116, 101, 47, 102, 108, 117, 115, 104, 0,
  ];
  let mut flush_fd: libc::c_int = open_or_warn(fn_0.as_ptr(), 0o1i32);
  if flush_fd < 0i32 {
    return;
  }
  if write(
    flush_fd,
    b"-1\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
    2i32 as size_t,
  ) < 2
  {
    bb_simple_perror_msg(b"can\'t flush routing cache\x00" as *const u8 as *const libc::c_char);
    return;
  }
  close(flush_fd);
}
unsafe extern "C" fn iproute_reset_filter() {
  memset(
    bb_common_bufsiz1.as_mut_ptr() as *mut filter_t as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<filter_t>() as libc::c_ulong,
  );
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t))
    .mdst
    .bitlen = -1i32 as i16;
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t))
    .msrc
    .bitlen = -1i32 as i16;
}
/* Return value becomes exitcode. It's okay to not return at all */
unsafe extern "C" fn iproute_list_or_flush(
  mut argv: *mut *mut libc::c_char,
  mut flush: libc::c_int,
) -> libc::c_int {
  let mut do_ipv6: libc::c_int = preferred_family as libc::c_int;
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
  let mut id: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut od: *mut libc::c_char = 0 as *mut libc::c_char;
  static mut keywords: [libc::c_char; 78] = [
    112, 114, 111, 116, 111, 99, 111, 108, 0, 100, 101, 118, 0, 111, 105, 102, 0, 105, 105, 102, 0,
    118, 105, 97, 0, 116, 97, 98, 108, 101, 0, 99, 97, 99, 104, 101, 0, 102, 114, 111, 109, 0, 116,
    111, 0, 115, 99, 111, 112, 101, 0, 97, 108, 108, 0, 114, 111, 111, 116, 0, 109, 97, 116, 99,
    104, 0, 101, 120, 97, 99, 116, 0, 109, 97, 105, 110, 0, 0,
  ];
  let mut arg: libc::c_int = 0;
  let mut parm: libc::c_int = 0;
  iproute_reset_filter();
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).tb = RT_TABLE_MAIN as libc::c_int;
  if flush != 0 && (*argv).is_null() {
    bb_error_msg_and_die(
      bb_msg_requires_arg.as_ptr(),
      b"\"ip route flush\"\x00" as *const u8 as *const libc::c_char,
    );
  }
  while !(*argv).is_null() {
    arg = index_in_substrings(keywords.as_ptr(), *argv);
    if arg == KW_proto as libc::c_int {
      let mut prot: u32 = 0i32 as u32;
      argv = next_arg(argv);
      //G_filter.protocol = prot;
      if rtnl_rtprot_a2n(&mut prot, *argv) != 0 {
        if index_in_strings(keywords.as_ptr(), *argv) != KW_all as libc::c_int {
          invarg_1_to_2(*argv, b"protocol\x00" as *const u8 as *const libc::c_char);
        }
        prot = 0i32 as u32
        //G_filter.protocolmask = -1;
        //G_filter.protocolmask = 0;
      }
    } else if arg == KW_dev as libc::c_int || arg == KW_oif as libc::c_int {
      argv = next_arg(argv);
      od = *argv
    } else if arg == KW_iif as libc::c_int {
      argv = next_arg(argv);
      id = *argv
    } else if arg == KW_via as libc::c_int {
      argv = next_arg(argv);
      get_prefix(
        &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).rvia,
        *argv,
        do_ipv6,
      );
    } else if arg == KW_table as libc::c_int {
      /* table all/cache/main */
      argv = next_arg(argv);
      parm = index_in_substrings(keywords.as_ptr(), *argv);
      if parm == KW_cache as libc::c_int {
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).tb = -1i32
      } else if parm == KW_all as libc::c_int {
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).tb = 0i32
      } else if parm != KW_main as libc::c_int {
        let mut tid: u32 = 0;
        if rtnl_rttable_a2n(&mut tid, *argv) != 0 {
          invarg_1_to_2(*argv, b"table\x00" as *const u8 as *const libc::c_char);
        }
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).tb = tid as libc::c_int
      }
    } else if arg == KW_cache as libc::c_int {
      /* The command 'ip route flush cache' is used by OpenSWAN.
       * Assuming it's a synonym for 'ip route flush table cache' */
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).tb = -1i32
    } else if arg == KW_scope as libc::c_int {
      let mut scope: u32 = 0; /* "to" is the default parameter */
      argv = next_arg(argv);
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).scopemask = -1i32;
      if rtnl_rtscope_a2n(&mut scope, *argv) != 0 {
        if strcmp(*argv, b"all\x00" as *const u8 as *const libc::c_char) != 0i32 {
          invarg_1_to_2(*argv, b"scope\x00" as *const u8 as *const libc::c_char);
        }
        scope = RT_SCOPE_NOWHERE as libc::c_int as u32;
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).scopemask = 0i32
      }
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).scope = scope as libc::c_int
    } else if arg == KW_from as libc::c_int {
      argv = next_arg(argv);
      parm = index_in_substrings(keywords.as_ptr(), *argv);
      if parm == KW_root as libc::c_int {
        argv = next_arg(argv);
        get_prefix(
          &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).rsrc,
          *argv,
          do_ipv6,
        );
      } else if parm == KW_match as libc::c_int {
        argv = next_arg(argv);
        get_prefix(
          &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).msrc,
          *argv,
          do_ipv6,
        );
      } else {
        if parm == KW_exact as libc::c_int {
          argv = next_arg(argv)
        }
        get_prefix(
          &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).msrc,
          *argv,
          do_ipv6,
        );
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).rsrc =
          (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).msrc
      }
    } else {
      if arg == KW_to as libc::c_int {
        argv = next_arg(argv);
        arg = index_in_substrings(keywords.as_ptr(), *argv)
      }
      /* parm = arg; - would be more plausible, but we reuse 'arg' here */
      if arg == KW_root as libc::c_int {
        argv = next_arg(argv); /* "to exact" is the default */
        get_prefix(
          &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).rdst,
          *argv,
          do_ipv6,
        );
      } else if arg == KW_match as libc::c_int {
        argv = next_arg(argv);
        get_prefix(
          &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).mdst,
          *argv,
          do_ipv6,
        );
      } else {
        if arg == KW_exact as libc::c_int {
          argv = next_arg(argv)
        }
        get_prefix(
          &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).mdst,
          *argv,
          do_ipv6,
        );
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).rdst =
          (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).mdst
      }
    }
    argv = argv.offset(1)
  }
  if do_ipv6 == 0i32 && (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).tb != 0 {
    do_ipv6 = 2i32
  }
  xrtnl_open(&mut rth);
  ll_init_map(&mut rth);
  if !id.is_null() || !od.is_null() {
    let mut idx: libc::c_int = 0;
    if !id.is_null() {
      idx = xll_name_to_index(id);
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).iif = idx
    }
    if !od.is_null() {
      idx = xll_name_to_index(od);
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).oif = idx
    }
  }
  if flush != 0 {
    let mut flushb: [libc::c_char; 3584] = [0; 3584];
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).tb == -1i32 {
      /* "flush table cache" */
      if do_ipv6 != 10i32 {
        iproute_flush_cache();
      }
      if do_ipv6 == 2i32 {
        return 0i32;
      }
    }
    let ref mut fresh1 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).flushb;
    *fresh1 = flushb.as_mut_ptr();
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).flushp = 0i32;
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).flushe =
      ::std::mem::size_of::<[libc::c_char; 3584]>() as libc::c_ulong as libc::c_int;
    let ref mut fresh2 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).rth;
    *fresh2 = &mut rth;
    loop {
      xrtnl_wilddump_request(&mut rth, do_ipv6, RTM_GETROUTE as libc::c_int);
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).flushed = 0i32 as smallint;
      xrtnl_dump_filter(
        &mut rth,
        Some(
          print_route
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
      if flush_update() != 0 {
        return 1i32;
      }
    }
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).tb != -1i32 {
    xrtnl_wilddump_request(&mut rth, do_ipv6, RTM_GETROUTE as libc::c_int);
  } else if rtnl_rtcache_request(&mut rth, do_ipv6) < 0i32 {
    bb_simple_perror_msg_and_die(
      b"can\'t send dump request\x00" as *const u8 as *const libc::c_char,
    );
  }
  xrtnl_dump_filter(
    &mut rth,
    Some(
      print_route
        as unsafe extern "C" fn(
          _: *const sockaddr_nl,
          _: *mut nlmsghdr,
          _: *mut libc::c_void,
        ) -> libc::c_int,
    ),
    0 as *mut libc::c_void,
  );
  return 0i32;
}
/* Return value becomes exitcode. It's okay to not return at all */
unsafe extern "C" fn iproute_get(mut argv: *mut *mut libc::c_char) -> libc::c_int {
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
    r: rtmsg {
      rtm_family: 0,
      rtm_dst_len: 0,
      rtm_src_len: 0,
      rtm_tos: 0,
      rtm_table: 0,
      rtm_protocol: 0,
      rtm_scope: 0,
      rtm_type: 0,
      rtm_flags: 0,
    },
    buf: [0; 1024],
  };
  let mut idev: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut odev: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut connected: bool = 0i32 != 0;
  let mut from_ok: bool = 0i32 != 0;
  static mut options: [libc::c_char; 38] = [
    102, 114, 111, 109, 0, 105, 105, 102, 0, 111, 105, 102, 0, 100, 101, 118, 0, 110, 111, 116,
    105, 102, 121, 0, 99, 111, 110, 110, 101, 99, 116, 101, 100, 0, 116, 111, 0, 0,
  ];
  memset(
    &mut req as *mut C2RustUnnamed_7 as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<C2RustUnnamed_7>() as libc::c_ulong,
  );
  iproute_reset_filter();
  req.n.nlmsg_len = (::std::mem::size_of::<rtmsg>() as libc::c_ulong).wrapping_add(
    ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
      .wrapping_add(4u32 as libc::c_ulong)
      .wrapping_sub(1i32 as libc::c_ulong)
      & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong) as libc::c_int
      as libc::c_ulong,
  ) as u32;
  req.n.nlmsg_flags = 0x1i32 as __u16;
  if RTM_GETROUTE as libc::c_int != 0 {
    req.n.nlmsg_type = RTM_GETROUTE as libc::c_int as __u16
  }
  req.r.rtm_family = preferred_family as libc::c_uchar;
  /*req.r.rtm_table = 0; - memset did this already */
  /*req.r.rtm_protocol = 0;*/
  /*req.r.rtm_scope = 0;*/
  /*req.r.rtm_type = 0;*/
  /*req.r.rtm_src_len = 0;*/
  /*req.r.rtm_dst_len = 0;*/
  /*req.r.rtm_tos = 0;*/
  while !(*argv).is_null() {
    let mut current_block_41: u64;
    match index_in_strings(options.as_ptr(), *argv) {
      0 => {
        /* from */
        let mut addr: inet_prefix = inet_prefix {
          family: 0,
          bytelen: 0,
          bitlen: 0,
          data: [0; 4],
        };
        argv = next_arg(argv);
        from_ok = 1i32 != 0;
        get_prefix(&mut addr, *argv, req.r.rtm_family as libc::c_int);
        if req.r.rtm_family as libc::c_int == 0i32 {
          req.r.rtm_family = addr.family
        }
        if addr.bytelen != 0 {
          addattr_l(
            &mut req.n,
            ::std::mem::size_of::<C2RustUnnamed_7>() as libc::c_ulong as libc::c_int,
            RTA_SRC as libc::c_int,
            &mut addr.data as *mut [u32; 4] as *mut libc::c_void,
            addr.bytelen as libc::c_int,
          );
        }
        req.r.rtm_src_len = addr.bitlen as libc::c_uchar;
        current_block_41 = 7990025728955927862;
      }
      1 => {
        /* iif */
        argv = next_arg(argv);
        idev = *argv;
        current_block_41 = 7990025728955927862;
      }
      2 | 3 => {
        /* oif */
        /* dev */
        argv = next_arg(argv);
        odev = *argv;
        current_block_41 = 7990025728955927862;
      }
      4 => {
        /* notify */
        req.r.rtm_flags |= 0x100i32 as libc::c_uint;
        current_block_41 = 7990025728955927862;
      }
      5 => {
        /* connected */
        connected = 1i32 != 0;
        current_block_41 = 7990025728955927862;
      }
      6 => {
        /* to */
        argv = next_arg(argv);
        current_block_41 = 521360392864492715;
      }
      _ => {
        current_block_41 = 521360392864492715;
      }
    }
    match current_block_41 {
      521360392864492715 => {
        let mut addr_0: inet_prefix = inet_prefix {
          family: 0,
          bytelen: 0,
          bitlen: 0,
          data: [0; 4],
        };
        get_prefix(&mut addr_0, *argv, req.r.rtm_family as libc::c_int);
        if req.r.rtm_family as libc::c_int == 0i32 {
          req.r.rtm_family = addr_0.family
        }
        if addr_0.bytelen != 0 {
          addattr_l(
            &mut req.n,
            ::std::mem::size_of::<C2RustUnnamed_7>() as libc::c_ulong as libc::c_int,
            RTA_DST as libc::c_int,
            &mut addr_0.data as *mut [u32; 4] as *mut libc::c_void,
            addr_0.bytelen as libc::c_int,
          );
        }
        req.r.rtm_dst_len = addr_0.bitlen as libc::c_uchar
      }
      _ => {}
    }
    argv = argv.offset(1)
  }
  if req.r.rtm_dst_len as libc::c_int == 0i32 {
    bb_simple_error_msg_and_die(
      b"need at least destination address\x00" as *const u8 as *const libc::c_char,
    );
  }
  xrtnl_open(&mut rth);
  ll_init_map(&mut rth);
  if !idev.is_null() || !odev.is_null() {
    let mut idx: libc::c_int = 0;
    if !idev.is_null() {
      idx = xll_name_to_index(idev);
      addattr32(
        &mut req.n,
        ::std::mem::size_of::<C2RustUnnamed_7>() as libc::c_ulong as libc::c_int,
        RTA_IIF as libc::c_int,
        idx as u32,
      );
    }
    if !odev.is_null() {
      idx = xll_name_to_index(odev);
      addattr32(
        &mut req.n,
        ::std::mem::size_of::<C2RustUnnamed_7>() as libc::c_ulong as libc::c_int,
        RTA_OIF as libc::c_int,
        idx as u32,
      );
    }
  }
  if req.r.rtm_family as libc::c_int == 0i32 {
    req.r.rtm_family = 2i32 as libc::c_uchar
  }
  if rtnl_talk(&mut rth, &mut req.n, &mut req.n) < 0i32 {
    return 2i32;
  }
  if connected as libc::c_int != 0 && !from_ok {
    let mut r: *mut rtmsg = (&mut req.n as *mut nlmsghdr as *mut libc::c_char).offset(
      (0i32
        + ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
          .wrapping_add(4u32 as libc::c_ulong)
          .wrapping_sub(1i32 as libc::c_ulong)
          & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong) as libc::c_int)
        as isize,
    ) as *mut libc::c_void as *mut rtmsg;
    let mut len: libc::c_int = req.n.nlmsg_len as libc::c_int;
    let mut tb: [*mut rtattr; 27] = [0 as *mut rtattr; 27];
    print_route(0 as *const sockaddr_nl, &mut req.n, 0 as *mut libc::c_void);
    if req.n.nlmsg_type as libc::c_int != RTM_NEWROUTE as libc::c_int {
      bb_simple_error_msg_and_die(b"not a route?\x00" as *const u8 as *const libc::c_char);
    }
    len = (len as libc::c_ulong).wrapping_sub(
      (::std::mem::size_of::<rtmsg>() as libc::c_ulong).wrapping_add(
        ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
          .wrapping_add(4u32 as libc::c_ulong)
          .wrapping_sub(1i32 as libc::c_ulong)
          & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong) as libc::c_int
          as libc::c_ulong,
      ),
    ) as libc::c_int as libc::c_int;
    if len < 0i32 {
      bb_error_msg_and_die(b"wrong len %d\x00" as *const u8 as *const libc::c_char, len);
    }
    //memset(tb, 0, sizeof(tb)); - parse_rtattr does this
    parse_rtattr(
      tb.as_mut_ptr(),
      __RTA_MAX as libc::c_int - 1i32,
      (r as *mut libc::c_char).offset(
        ((::std::mem::size_of::<rtmsg>() as libc::c_ulong)
          .wrapping_add(4u32 as libc::c_ulong)
          .wrapping_sub(1i32 as libc::c_ulong)
          & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong) as isize,
      ) as *mut rtattr,
      len,
    );
    if !tb[RTA_PREFSRC as libc::c_int as usize].is_null() {
      (*tb[RTA_PREFSRC as libc::c_int as usize]).rta_type =
        RTA_SRC as libc::c_int as libc::c_ushort;
      (*r).rtm_src_len = (8i32 as libc::c_ulong).wrapping_mul(
        ((*tb[RTA_PREFSRC as libc::c_int as usize]).rta_len as libc::c_int as libc::c_ulong)
          .wrapping_sub(
            ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
              .wrapping_add(4u32 as libc::c_ulong)
              .wrapping_sub(1i32 as libc::c_ulong)
              & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
              .wrapping_add(0i32 as libc::c_ulong),
          ),
      ) as libc::c_uchar
    } else if tb[RTA_SRC as libc::c_int as usize].is_null() {
      bb_simple_error_msg_and_die(
        b"can\'t connect the route\x00" as *const u8 as *const libc::c_char,
      );
    }
    if odev.is_null() && !tb[RTA_OIF as libc::c_int as usize].is_null() {
      (*tb[RTA_OIF as libc::c_int as usize]).rta_type = 0i32 as libc::c_ushort
    }
    if !tb[RTA_GATEWAY as libc::c_int as usize].is_null() {
      (*tb[RTA_GATEWAY as libc::c_int as usize]).rta_type = 0i32 as libc::c_ushort
    }
    if idev.is_null() && !tb[RTA_IIF as libc::c_int as usize].is_null() {
      (*tb[RTA_IIF as libc::c_int as usize]).rta_type = 0i32 as libc::c_ushort
    }
    req.n.nlmsg_flags = 0x1i32 as __u16;
    req.n.nlmsg_type = RTM_GETROUTE as libc::c_int as __u16;
    if rtnl_talk(&mut rth, &mut req.n, &mut req.n) < 0i32 {
      return 2i32;
    }
  }
  print_route(0 as *const sockaddr_nl, &mut req.n, 0 as *mut libc::c_void);
  return 0i32;
}

//int FAST_FUNC print_neigh(struct sockaddr_nl *who, struct nlmsghdr *n, void *arg);
//int FAST_FUNC iproute_monitor(char **argv);
//void FAST_FUNC ipneigh_reset_filter(void);
/* Return value becomes exitcode. It's okay to not return at all */
#[no_mangle]
pub unsafe extern "C" fn do_iproute(mut argv: *mut *mut libc::c_char) -> libc::c_int {
  static mut ip_route_commands: [libc::c_char; 73] = [
    97, 0, 97, 100, 100, 0, 97, 112, 112, 101, 110, 100, 0, 99, 104, 97, 110, 103, 101, 0, 99, 104,
    103, 0, 100, 101, 108, 101, 116, 101, 0, 103, 101, 116, 0, 108, 105, 115, 116, 0, 115, 104,
    111, 119, 0, 112, 114, 101, 112, 101, 110, 100, 0, 114, 101, 112, 108, 97, 99, 101, 0, 116,
    101, 115, 116, 0, 102, 108, 117, 115, 104, 0, 0,
  ];
  let mut command_num: libc::c_int = 0;
  let mut flags: libc::c_uint = 0i32 as libc::c_uint;
  let mut cmd: libc::c_int = RTM_NEWROUTE as libc::c_int;
  if (*argv).is_null() {
    return iproute_list_or_flush(argv, 0i32);
  }
  /* "Standard" 'ip r a' treats 'a' as 'add', not 'append' */
  /* It probably means that it is using "first match" rule */
  command_num = index_in_substrings(ip_route_commands.as_ptr(), *argv);
  match command_num {
    0 | 1 => flags = (0x400i32 | 0x200i32) as libc::c_uint,
    2 => flags = (0x400i32 | 0x800i32) as libc::c_uint,
    3 | 4 => flags = 0x100i32 as libc::c_uint,
    5 => cmd = RTM_DELROUTE as libc::c_int,
    6 => return iproute_get(argv.offset(1)),
    7 | 8 => return iproute_list_or_flush(argv.offset(1), 0i32),
    9 => flags = 0x400i32 as libc::c_uint,
    10 => flags = (0x400i32 | 0x100i32) as libc::c_uint,
    11 => flags = 0x200i32 as libc::c_uint,
    12 => return iproute_list_or_flush(argv.offset(1), 1i32),
    _ => {
      invarg_1_to_2(*argv, applet_name);
    }
  }
  return iproute_modify(cmd, flags, argv.offset(1));
}
