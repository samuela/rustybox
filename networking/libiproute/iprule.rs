use libc;



extern "C" {
  #[no_mangle]
  static mut stdout: *mut FILE;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  /* Guaranteed to NOT be a macro (smallest code). Saves nearly 2k on uclibc.
   * But potentially slow, don't use in one-billion-times loops */
  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_warn_ignoring_args(arg: *mut libc::c_char);
  #[no_mangle]
  fn index_in_substrings(strings: *const libc::c_char, key: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  static mut applet_name: *const libc::c_char;
  #[no_mangle]
  fn rtnl_rtrealm_n2a(id: libc::c_int) -> *const libc::c_char;
  #[no_mangle]
  fn rtnl_dsfield_n2a(id: libc::c_int) -> *const libc::c_char;
  #[no_mangle]
  fn rtnl_rttable_n2a(id: libc::c_int) -> *const libc::c_char;
  #[no_mangle]
  fn rtnl_dsfield_a2n(id: *mut u32, arg: *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn rtnl_rttable_a2n(id: *mut u32, arg: *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn rtnl_rtntype_n2a(id: libc::c_int) -> *const libc::c_char;
  #[no_mangle]
  fn rtnl_rtntype_a2n(id: *mut libc::c_int, arg: *mut libc::c_char) -> libc::c_int;

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
  fn parse_rtattr(tb: *mut *mut rtattr, max: libc::c_int, rta: *mut rtattr, len: libc::c_int);
  #[no_mangle]
  fn rt_addr_n2a(af: libc::c_int, addr: *mut libc::c_void) -> *const libc::c_char;
  #[no_mangle]
  fn rtnl_talk(rtnl: *mut rtnl_handle, n: *mut nlmsghdr, answer: *mut nlmsghdr) -> libc::c_int;
  #[no_mangle]
  fn invarg_1_to_2(_: *const libc::c_char, _: *const libc::c_char) -> !;
  #[no_mangle]
  fn next_arg(argv: *mut *mut libc::c_char) -> *mut *mut libc::c_char;
  #[no_mangle]
  fn get_addr32(name: *mut libc::c_char) -> u32;
  #[no_mangle]
  fn addattr32(
    n: *mut nlmsghdr,
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
  fn get_u32(arg: *mut libc::c_char, errmsg: *const libc::c_char) -> u32;
  #[no_mangle]
  fn get_rt_realms(realms: *mut u32, arg: *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn get_prefix(dst: *mut inet_prefix, arg: *mut libc::c_char, family: libc::c_int);
  #[no_mangle]
  fn xrtnl_wilddump_request(rth: *mut rtnl_handle, fam: libc::c_int, type_0: libc::c_int);
  #[no_mangle]
  static mut preferred_family: family_t;
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
  fn xrtnl_open(rth: *mut rtnl_handle);
}



use crate::librb::size_t;


pub type smalluint = libc::c_uchar;

use libc::FILE;
pub type family_t = i8;
pub type __u16 = libc::c_ushort;
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
pub struct rtnl_handle {
  pub fd: libc::c_int,
  pub local: sockaddr_nl,
  pub peer: sockaddr_nl,
  pub seq: u32,
  pub dump: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
  pub n: nlmsghdr,
  pub r: rtmsg,
  pub buf: [libc::c_char; 1024],
}
pub const ARG_help: C2RustUnnamed_2 = 18;
pub const ARG_type: C2RustUnnamed_2 = 17;
pub const ARG_map_to: C2RustUnnamed_2 = 16;
pub const ARG_nat: C2RustUnnamed_2 = 15;
pub const ARG_iif: C2RustUnnamed_2 = 14;
pub const ARG_dev: C2RustUnnamed_2 = 13;
pub const ARG_suppress_ifgroup: C2RustUnnamed_2 = 12;
pub const ARG_suppress_prefixlength: C2RustUnnamed_2 = 11;
pub const ARG_lookup: C2RustUnnamed_2 = 10;
pub const ARG_table: C2RustUnnamed_2 = 9;
pub const ARG_realms: C2RustUnnamed_2 = 8;
pub const ARG_fwmark: C2RustUnnamed_2 = 7;
pub const ARG_tos: C2RustUnnamed_2 = 6;
pub const ARG_priority: C2RustUnnamed_2 = 5;
pub const ARG_order: C2RustUnnamed_2 = 4;
pub const ARG_preference: C2RustUnnamed_2 = 3;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct inet_prefix {
  pub family: u8,
  pub bytelen: u8,
  pub bitlen: i16,
  pub data: [u32; 4],
}
pub const ARG_to: C2RustUnnamed_2 = 2;
pub const ARG_from: C2RustUnnamed_2 = 1;
pub type C2RustUnnamed_2 = libc::c_uint;
/* RTA_TABLE is not a define, can't test with ifdef. */
/* As a proxy, test which kernels toolchain expects: */
/* If you add stuff here, update iprule_full_usage */
static mut keywords: [libc::c_char; 134] = [
  102, 114, 111, 109, 0, 116, 111, 0, 112, 114, 101, 102, 101, 114, 101, 110, 99, 101, 0, 111, 114,
  100, 101, 114, 0, 112, 114, 105, 111, 114, 105, 116, 121, 0, 116, 111, 115, 0, 102, 119, 109, 97,
  114, 107, 0, 114, 101, 97, 108, 109, 115, 0, 116, 97, 98, 108, 101, 0, 108, 111, 111, 107, 117,
  112, 0, 115, 117, 112, 112, 114, 101, 115, 115, 95, 112, 114, 101, 102, 105, 120, 108, 101, 110,
  103, 116, 104, 0, 115, 117, 112, 112, 114, 101, 115, 115, 95, 105, 102, 103, 114, 111, 117, 112,
  0, 100, 101, 118, 0, 105, 105, 102, 0, 110, 97, 116, 0, 109, 97, 112, 45, 116, 111, 0, 116, 121,
  112, 101, 0, 104, 101, 108, 112, 0, 0,
];
unsafe extern "C" fn print_rule(
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
  let mut host_len: libc::c_int = -1i32;
  let mut tb: [*mut rtattr; 27] = [0 as *mut rtattr; 27];
  if (*n).nlmsg_type as libc::c_int != RTM_NEWRULE as libc::c_int {
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
    return -1i32;
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
  if (*r).rtm_family as libc::c_int == 2i32 {
    host_len = 32i32
  } else if (*r).rtm_family as libc::c_int == 10i32 {
    host_len = 128i32
  }
  /*	else if (r->rtm_family == AF_DECnet)
      host_len = 16;
    else if (r->rtm_family == AF_IPX)
      host_len = 80;
  */
  printf(
    b"%u:\t\x00" as *const u8 as *const libc::c_char,
    if !tb[RTA_PRIORITY as libc::c_int as usize].is_null() {
      *((tb[RTA_PRIORITY as libc::c_int as usize] as *mut libc::c_char).offset(
        ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
          .wrapping_add(4u32 as libc::c_ulong)
          .wrapping_sub(1i32 as libc::c_ulong)
          & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
          .wrapping_add(0i32 as libc::c_ulong) as isize,
      ) as *mut libc::c_void as *mut libc::c_uint)
    } else {
      0i32 as libc::c_uint
    },
  );
  printf(b"from \x00" as *const u8 as *const libc::c_char);
  if !tb[RTA_SRC as libc::c_int as usize].is_null() {
    if (*r).rtm_src_len as libc::c_int != host_len {
      printf(
        b"%s/%u\x00" as *const u8 as *const libc::c_char,
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
      fputs_unlocked(
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
        stdout,
      );
    }
  } else if (*r).rtm_src_len != 0 {
    printf(
      b"0/%d\x00" as *const u8 as *const libc::c_char,
      (*r).rtm_src_len as libc::c_int,
    );
  } else {
    printf(b"all\x00" as *const u8 as *const libc::c_char);
  }
  bb_putchar(' ' as i32);
  if !tb[RTA_DST as libc::c_int as usize].is_null() {
    if (*r).rtm_dst_len as libc::c_int != host_len {
      printf(
        b"to %s/%u \x00" as *const u8 as *const libc::c_char,
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
        b"to %s \x00" as *const u8 as *const libc::c_char,
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
      b"to 0/%d \x00" as *const u8 as *const libc::c_char,
      (*r).rtm_dst_len as libc::c_int,
    );
  }
  if (*r).rtm_tos != 0 {
    printf(
      b"tos %s \x00" as *const u8 as *const libc::c_char,
      rtnl_dsfield_n2a((*r).rtm_tos as libc::c_int),
    );
  }
  if !tb[RTA_PROTOINFO as libc::c_int as usize].is_null() {
    printf(
      b"fwmark %#x \x00" as *const u8 as *const libc::c_char,
      *((tb[RTA_PROTOINFO as libc::c_int as usize] as *mut libc::c_char).offset(
        ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
          .wrapping_add(4u32 as libc::c_ulong)
          .wrapping_sub(1i32 as libc::c_ulong)
          & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
          .wrapping_add(0i32 as libc::c_ulong) as isize,
      ) as *mut libc::c_void as *mut u32),
    );
  }
  if !tb[RTA_IIF as libc::c_int as usize].is_null() {
    printf(
      b"iif %s \x00" as *const u8 as *const libc::c_char,
      (tb[RTA_IIF as libc::c_int as usize] as *mut libc::c_char).offset(
        ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
          .wrapping_add(4u32 as libc::c_ulong)
          .wrapping_sub(1i32 as libc::c_ulong)
          & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
          .wrapping_add(0i32 as libc::c_ulong) as isize,
      ) as *mut libc::c_void as *mut libc::c_char,
    );
  }
  if !tb[RTA_TABLE as libc::c_int as usize].is_null() {
    printf(
      b"lookup %s \x00" as *const u8 as *const libc::c_char,
      rtnl_rttable_n2a(
        *((tb[RTA_TABLE as libc::c_int as usize] as *mut libc::c_char).offset(
          ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
            .wrapping_add(4u32 as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong)
            & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
            .wrapping_add(0i32 as libc::c_ulong) as isize,
        ) as *mut libc::c_void as *mut u32) as libc::c_int,
      ),
    );
  } else if (*r).rtm_table != 0 {
    printf(
      b"lookup %s \x00" as *const u8 as *const libc::c_char,
      rtnl_rttable_n2a((*r).rtm_table as libc::c_int),
    );
  }
  if !tb[14].is_null() {
    let mut pl: libc::c_int = *((tb[14] as *mut libc::c_char).offset(
      ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
        .wrapping_add(4u32 as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong)
        & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
        .wrapping_add(0i32 as libc::c_ulong) as isize,
    ) as *mut libc::c_void as *mut u32) as libc::c_int;
    if pl != -1i32 {
      printf(
        b"%s %d \x00" as *const u8 as *const libc::c_char,
        keywords
          .as_ptr()
          .offset(::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as isize)
          .offset(::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong as isize)
          .offset(::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong as isize)
          .offset(::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as isize)
          .offset(::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong as isize)
          .offset(::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong as isize)
          .offset(::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as isize)
          .offset(::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as isize)
          .offset(::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as isize)
          .offset(::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as isize),
        pl,
      );
    }
  }
  if !tb[13].is_null() {
    let mut grp: libc::c_int = *((tb[13] as *mut libc::c_char).offset(
      ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
        .wrapping_add(4u32 as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong)
        & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
        .wrapping_add(0i32 as libc::c_ulong) as isize,
    ) as *mut libc::c_void as *mut u32) as libc::c_int;
    if grp != -1i32 {
      printf(
        b"%s %d \x00" as *const u8 as *const libc::c_char,
        keywords
          .as_ptr()
          .offset(::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as isize)
          .offset(::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong as isize)
          .offset(::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong as isize)
          .offset(::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as isize)
          .offset(::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong as isize)
          .offset(::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong as isize)
          .offset(::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as isize)
          .offset(::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as isize)
          .offset(::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as isize)
          .offset(::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as isize)
          .offset(::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong as isize),
        grp,
      );
    }
  }
  if !tb[RTA_FLOW as libc::c_int as usize].is_null() {
    let mut to: u32 = *((tb[RTA_FLOW as libc::c_int as usize] as *mut libc::c_char).offset(
      ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
        .wrapping_add(4u32 as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong)
        & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
        .wrapping_add(0i32 as libc::c_ulong) as isize,
    ) as *mut libc::c_void as *mut u32);
    let mut from: u32 = to >> 16i32;
    to &= 0xffffi32 as libc::c_uint;
    if from != 0 {
      printf(
        b"realms %s/\x00" as *const u8 as *const libc::c_char,
        rtnl_rtrealm_n2a(from as libc::c_int),
      );
    }
    printf(
      b"%s \x00" as *const u8 as *const libc::c_char,
      rtnl_rtrealm_n2a(to as libc::c_int),
    );
  }
  if (*r).rtm_type as libc::c_int == RTN_NAT as libc::c_int {
    if !tb[RTA_GATEWAY as libc::c_int as usize].is_null() {
      printf(
        b"map-to %s \x00" as *const u8 as *const libc::c_char,
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
    } else {
      printf(b"masquerade\x00" as *const u8 as *const libc::c_char);
    }
  } else if (*r).rtm_type as libc::c_int != RTN_UNICAST as libc::c_int {
    fputs_unlocked(rtnl_rtntype_n2a((*r).rtm_type as libc::c_int), stdout);
  }
  bb_putchar('\n' as i32);
  /*fflush_all();*/
  return 0i32;
}
/* Return value becomes exitcode. It's okay to not return at all */
unsafe extern "C" fn iprule_list(mut argv: *mut *mut libc::c_char) -> libc::c_int {
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
  let mut af: libc::c_int = preferred_family as libc::c_int;
  if af == 0i32 {
    af = 2i32
  }
  if !(*argv).is_null() {
    //bb_error_msg("\"rule show\" needs no arguments");
    bb_warn_ignoring_args(*argv);
    return -1i32;
  }
  xrtnl_open(&mut rth);
  xrtnl_wilddump_request(&mut rth, af, RTM_GETRULE as libc::c_int);
  xrtnl_dump_filter(
    &mut rth,
    Some(
      print_rule
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
unsafe extern "C" fn iprule_modify(
  mut cmd: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut table_ok: bool = 0i32 != 0;
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
  let mut req: C2RustUnnamed_1 = C2RustUnnamed_1 {
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
  let mut key: smalluint = 0;
  memset(
    &mut req as *mut C2RustUnnamed_1 as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong,
  );
  req.n.nlmsg_type = cmd as __u16;
  req.n.nlmsg_len = (::std::mem::size_of::<rtmsg>() as libc::c_ulong).wrapping_add(
    ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
      .wrapping_add(4u32 as libc::c_ulong)
      .wrapping_sub(1i32 as libc::c_ulong)
      & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong) as libc::c_int
      as libc::c_ulong,
  ) as u32;
  req.n.nlmsg_flags = 0x1i32 as __u16;
  req.r.rtm_family = preferred_family as libc::c_uchar;
  req.r.rtm_protocol = 3i32 as libc::c_uchar;
  if RT_SCOPE_UNIVERSE as libc::c_int != 0i32 {
    req.r.rtm_scope = RT_SCOPE_UNIVERSE as libc::c_int as libc::c_uchar
  }
  /*req.r.rtm_table = 0; - already is */
  if RTN_UNSPEC as libc::c_int != 0i32 {
    req.r.rtm_type = RTN_UNSPEC as libc::c_int as libc::c_uchar
  }
  if cmd == RTM_NEWRULE as libc::c_int {
    req.n.nlmsg_flags = (req.n.nlmsg_flags as libc::c_int | (0x400i32 | 0x200i32)) as __u16;
    req.r.rtm_type = RTN_UNICAST as libc::c_int as libc::c_uchar
  }
  while !(*argv).is_null() {
    key = (index_in_substrings(keywords.as_ptr(), *argv) + 1i32) as smalluint;
    if key as libc::c_int == 0i32 {
      /* no match found in keywords array, bail out. */
      invarg_1_to_2(*argv, applet_name);
    }
    if key as libc::c_int == ARG_from as libc::c_int {
      let mut dst: inet_prefix = inet_prefix {
        family: 0,
        bytelen: 0,
        bitlen: 0,
        data: [0; 4],
      };
      argv = next_arg(argv);
      get_prefix(&mut dst, *argv, req.r.rtm_family as libc::c_int);
      req.r.rtm_src_len = dst.bitlen as libc::c_uchar;
      addattr_l(
        &mut req.n,
        ::std::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong as libc::c_int,
        RTA_SRC as libc::c_int,
        &mut dst.data as *mut [u32; 4] as *mut libc::c_void,
        dst.bytelen as libc::c_int,
      );
    } else if key as libc::c_int == ARG_to as libc::c_int {
      let mut dst_0: inet_prefix = inet_prefix {
        family: 0,
        bytelen: 0,
        bitlen: 0,
        data: [0; 4],
      };
      argv = next_arg(argv);
      get_prefix(&mut dst_0, *argv, req.r.rtm_family as libc::c_int);
      req.r.rtm_dst_len = dst_0.bitlen as libc::c_uchar;
      addattr_l(
        &mut req.n,
        ::std::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong as libc::c_int,
        RTA_DST as libc::c_int,
        &mut dst_0.data as *mut [u32; 4] as *mut libc::c_void,
        dst_0.bytelen as libc::c_int,
      );
    } else if key as libc::c_int == ARG_preference as libc::c_int
      || key as libc::c_int == ARG_order as libc::c_int
      || key as libc::c_int == ARG_priority as libc::c_int
    {
      let mut pref: u32 = 0;
      argv = next_arg(argv);
      pref = get_u32(
        *argv,
        keywords
          .as_ptr()
          .offset(::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as isize)
          .offset(::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong as isize),
      );
      addattr32(
        &mut req.n,
        ::std::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong as libc::c_int,
        RTA_PRIORITY as libc::c_int,
        pref,
      );
    } else if key as libc::c_int == ARG_tos as libc::c_int {
      let mut tos: u32 = 0;
      argv = next_arg(argv);
      if rtnl_dsfield_a2n(&mut tos, *argv) != 0 {
        invarg_1_to_2(*argv, b"TOS\x00" as *const u8 as *const libc::c_char);
      }
      req.r.rtm_tos = tos as libc::c_uchar
    } else if key as libc::c_int == ARG_fwmark as libc::c_int {
      let mut fwmark: u32 = 0;
      argv = next_arg(argv);
      fwmark = get_u32(
        *argv,
        keywords
          .as_ptr()
          .offset(::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as isize)
          .offset(::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong as isize)
          .offset(::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong as isize)
          .offset(::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as isize)
          .offset(::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong as isize)
          .offset(::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong as isize),
      );
      addattr32(
        &mut req.n,
        ::std::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong as libc::c_int,
        RTA_PROTOINFO as libc::c_int,
        fwmark,
      );
    } else if key as libc::c_int == ARG_realms as libc::c_int {
      let mut realm: u32 = 0;
      argv = next_arg(argv);
      if get_rt_realms(&mut realm, *argv) != 0 {
        invarg_1_to_2(
          *argv,
          keywords
            .as_ptr()
            .offset(::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as isize)
            .offset(::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong as isize)
            .offset(::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong as isize)
            .offset(::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as isize)
            .offset(::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong as isize)
            .offset(::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong as isize)
            .offset(::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as isize),
        );
      }
      addattr32(
        &mut req.n,
        ::std::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong as libc::c_int,
        RTA_FLOW as libc::c_int,
        realm,
      );
    } else if key as libc::c_int == ARG_table as libc::c_int
      || key as libc::c_int == ARG_lookup as libc::c_int
    {
      let mut tid: u32 = 0;
      argv = next_arg(argv);
      if rtnl_rttable_a2n(&mut tid, *argv) != 0 {
        invarg_1_to_2(*argv, b"table ID\x00" as *const u8 as *const libc::c_char);
      }
      if tid > 255i32 as libc::c_uint {
        req.r.rtm_table = RT_TABLE_UNSPEC as libc::c_int as libc::c_uchar;
        addattr32(
          &mut req.n,
          ::std::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong as libc::c_int,
          RTA_TABLE as libc::c_int,
          tid,
        );
      } else {
        req.r.rtm_table = tid as libc::c_uchar
      }
      table_ok = 1i32 != 0
    } else if key as libc::c_int == ARG_suppress_prefixlength as libc::c_int {
      let mut prefix_length: libc::c_int = 0;
      argv = next_arg(argv);
      prefix_length = get_u32(
        *argv,
        keywords
          .as_ptr()
          .offset(::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as isize)
          .offset(::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong as isize)
          .offset(::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong as isize)
          .offset(::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as isize)
          .offset(::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong as isize)
          .offset(::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong as isize)
          .offset(::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as isize)
          .offset(::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as isize)
          .offset(::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as isize)
          .offset(::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as isize),
      ) as libc::c_int;
      addattr32(
        &mut req.n,
        ::std::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong as libc::c_int,
        14i32,
        prefix_length as u32,
      );
    } else if key as libc::c_int == ARG_suppress_ifgroup as libc::c_int {
      let mut grp: libc::c_int = 0;
      argv = next_arg(argv);
      grp = get_u32(
        *argv,
        keywords
          .as_ptr()
          .offset(::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as isize)
          .offset(::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong as isize)
          .offset(::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong as isize)
          .offset(::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as isize)
          .offset(::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong as isize)
          .offset(::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong as isize)
          .offset(::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as isize)
          .offset(::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as isize)
          .offset(::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as isize)
          .offset(::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as isize)
          .offset(::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong as isize),
      ) as libc::c_int;
      addattr32(
        &mut req.n,
        ::std::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong as libc::c_int,
        13i32,
        grp as u32,
      );
    } else if key as libc::c_int == ARG_dev as libc::c_int
      || key as libc::c_int == ARG_iif as libc::c_int
    {
      argv = next_arg(argv);
      addattr_l(
        &mut req.n,
        ::std::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong as libc::c_int,
        RTA_IIF as libc::c_int,
        *argv as *mut libc::c_void,
        strlen(*argv).wrapping_add(1i32 as libc::c_ulong) as libc::c_int,
      );
    } else if key as libc::c_int == ARG_nat as libc::c_int
      || key as libc::c_int == ARG_map_to as libc::c_int
    {
      argv = next_arg(argv);
      addattr32(
        &mut req.n,
        ::std::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong as libc::c_int,
        RTA_GATEWAY as libc::c_int,
        get_addr32(*argv),
      );
      req.r.rtm_type = RTN_NAT as libc::c_int as libc::c_uchar
    } else {
      let mut type_0: libc::c_int = 0;
      if key as libc::c_int == ARG_type as libc::c_int {
        argv = next_arg(argv)
      }
      if key as libc::c_int == ARG_help as libc::c_int {
        bb_show_usage();
      }
      if rtnl_rtntype_a2n(&mut type_0, *argv) != 0 {
        invarg_1_to_2(*argv, b"type\x00" as *const u8 as *const libc::c_char);
      }
      req.r.rtm_type = type_0 as libc::c_uchar
    }
    argv = argv.offset(1)
  }
  if req.r.rtm_family as libc::c_int == 0i32 {
    req.r.rtm_family = 2i32 as libc::c_uchar
  }
  if !table_ok && cmd == RTM_NEWRULE as libc::c_int {
    req.r.rtm_table = RT_TABLE_MAIN as libc::c_int as libc::c_uchar
  }
  xrtnl_open(&mut rth);
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
pub unsafe extern "C" fn do_iprule(mut argv: *mut *mut libc::c_char) -> libc::c_int {
  static mut ip_rule_commands: [libc::c_char; 22] = [
    97, 100, 100, 0, 100, 101, 108, 101, 116, 101, 0, 108, 105, 115, 116, 0, 115, 104, 111, 119, 0,
    0,
  ];
  if !(*argv).is_null() {
    let mut cmd: libc::c_int = index_in_substrings(ip_rule_commands.as_ptr(), *argv);
    if cmd < 0i32 {
      invarg_1_to_2(*argv, applet_name);
    }
    argv = argv.offset(1);
    if cmd < 2i32 {
      return iprule_modify(
        if cmd == 0i32 {
          RTM_NEWRULE as libc::c_int
        } else {
          RTM_DELRULE as libc::c_int
        },
        argv,
      );
    }
  }
  return iprule_list(argv);
}
