use libc;
extern "C" {
  #[no_mangle]
  fn if_nametoindex(__ifname: *const libc::c_char) -> libc::c_uint;
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xmalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  #[no_mangle]
  fn auto_string(str: *mut libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  /* vi: set sw=4 ts=4: */
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
  fn xrtnl_wilddump_request(rth: *mut rtnl_handle, fam: libc::c_int, type_0: libc::c_int);
}
pub type __uint32_t = libc::c_uint;
pub type size_t = libc::c_ulong;
use crate::librb::uint32_t;
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
pub type C2RustUnnamed = libc::c_uint;
pub const __IFLA_MAX: C2RustUnnamed = 50;
pub const IFLA_NEW_IFINDEX: C2RustUnnamed = 49;
pub const IFLA_CARRIER_DOWN_COUNT: C2RustUnnamed = 48;
pub const IFLA_CARRIER_UP_COUNT: C2RustUnnamed = 47;
pub const IFLA_IF_NETNSID: C2RustUnnamed = 46;
pub const IFLA_NEW_NETNSID: C2RustUnnamed = 45;
pub const IFLA_EVENT: C2RustUnnamed = 44;
pub const IFLA_XDP: C2RustUnnamed = 43;
pub const IFLA_PAD: C2RustUnnamed = 42;
pub const IFLA_GSO_MAX_SIZE: C2RustUnnamed = 41;
pub const IFLA_GSO_MAX_SEGS: C2RustUnnamed = 40;
pub const IFLA_PROTO_DOWN: C2RustUnnamed = 39;
pub const IFLA_PHYS_PORT_NAME: C2RustUnnamed = 38;
pub const IFLA_LINK_NETNSID: C2RustUnnamed = 37;
pub const IFLA_PHYS_SWITCH_ID: C2RustUnnamed = 36;
pub const IFLA_CARRIER_CHANGES: C2RustUnnamed = 35;
pub const IFLA_PHYS_PORT_ID: C2RustUnnamed = 34;
pub const IFLA_CARRIER: C2RustUnnamed = 33;
pub const IFLA_NUM_RX_QUEUES: C2RustUnnamed = 32;
pub const IFLA_NUM_TX_QUEUES: C2RustUnnamed = 31;
pub const IFLA_PROMISCUITY: C2RustUnnamed = 30;
pub const IFLA_EXT_MASK: C2RustUnnamed = 29;
pub const IFLA_NET_NS_FD: C2RustUnnamed = 28;
pub const IFLA_GROUP: C2RustUnnamed = 27;
pub const IFLA_AF_SPEC: C2RustUnnamed = 26;
pub const IFLA_PORT_SELF: C2RustUnnamed = 25;
pub const IFLA_VF_PORTS: C2RustUnnamed = 24;
pub const IFLA_STATS64: C2RustUnnamed = 23;
pub const IFLA_VFINFO_LIST: C2RustUnnamed = 22;
pub const IFLA_NUM_VF: C2RustUnnamed = 21;
pub const IFLA_IFALIAS: C2RustUnnamed = 20;
pub const IFLA_NET_NS_PID: C2RustUnnamed = 19;
pub const IFLA_LINKINFO: C2RustUnnamed = 18;
pub const IFLA_LINKMODE: C2RustUnnamed = 17;
pub const IFLA_OPERSTATE: C2RustUnnamed = 16;
pub const IFLA_WEIGHT: C2RustUnnamed = 15;
pub const IFLA_MAP: C2RustUnnamed = 14;
pub const IFLA_TXQLEN: C2RustUnnamed = 13;
pub const IFLA_PROTINFO: C2RustUnnamed = 12;
pub const IFLA_WIRELESS: C2RustUnnamed = 11;
pub const IFLA_MASTER: C2RustUnnamed = 10;
pub const IFLA_PRIORITY: C2RustUnnamed = 9;
pub const IFLA_COST: C2RustUnnamed = 8;
pub const IFLA_STATS: C2RustUnnamed = 7;
pub const IFLA_QDISC: C2RustUnnamed = 6;
pub const IFLA_LINK: C2RustUnnamed = 5;
pub const IFLA_MTU: C2RustUnnamed = 4;
pub const IFLA_IFNAME: C2RustUnnamed = 3;
pub const IFLA_BROADCAST: C2RustUnnamed = 2;
pub const IFLA_ADDRESS: C2RustUnnamed = 1;
pub const IFLA_UNSPEC: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const __RTM_MAX: C2RustUnnamed_0 = 97;
pub const RTM_NEWCACHEREPORT: C2RustUnnamed_0 = 96;
pub const RTM_GETSTATS: C2RustUnnamed_0 = 94;
pub const RTM_NEWSTATS: C2RustUnnamed_0 = 92;
pub const RTM_GETNSID: C2RustUnnamed_0 = 90;
pub const RTM_DELNSID: C2RustUnnamed_0 = 89;
pub const RTM_NEWNSID: C2RustUnnamed_0 = 88;
pub const RTM_GETMDB: C2RustUnnamed_0 = 86;
pub const RTM_DELMDB: C2RustUnnamed_0 = 85;
pub const RTM_NEWMDB: C2RustUnnamed_0 = 84;
pub const RTM_GETNETCONF: C2RustUnnamed_0 = 82;
pub const RTM_DELNETCONF: C2RustUnnamed_0 = 81;
pub const RTM_NEWNETCONF: C2RustUnnamed_0 = 80;
pub const RTM_SETDCB: C2RustUnnamed_0 = 79;
pub const RTM_GETDCB: C2RustUnnamed_0 = 78;
pub const RTM_GETADDRLABEL: C2RustUnnamed_0 = 74;
pub const RTM_DELADDRLABEL: C2RustUnnamed_0 = 73;
pub const RTM_NEWADDRLABEL: C2RustUnnamed_0 = 72;
pub const RTM_NEWNDUSEROPT: C2RustUnnamed_0 = 68;
pub const RTM_SETNEIGHTBL: C2RustUnnamed_0 = 67;
pub const RTM_GETNEIGHTBL: C2RustUnnamed_0 = 66;
pub const RTM_NEWNEIGHTBL: C2RustUnnamed_0 = 64;
pub const RTM_GETANYCAST: C2RustUnnamed_0 = 62;
pub const RTM_GETMULTICAST: C2RustUnnamed_0 = 58;
pub const RTM_NEWPREFIX: C2RustUnnamed_0 = 52;
pub const RTM_GETACTION: C2RustUnnamed_0 = 50;
pub const RTM_DELACTION: C2RustUnnamed_0 = 49;
pub const RTM_NEWACTION: C2RustUnnamed_0 = 48;
pub const RTM_GETTFILTER: C2RustUnnamed_0 = 46;
pub const RTM_DELTFILTER: C2RustUnnamed_0 = 45;
pub const RTM_NEWTFILTER: C2RustUnnamed_0 = 44;
pub const RTM_GETTCLASS: C2RustUnnamed_0 = 42;
pub const RTM_DELTCLASS: C2RustUnnamed_0 = 41;
pub const RTM_NEWTCLASS: C2RustUnnamed_0 = 40;
pub const RTM_GETQDISC: C2RustUnnamed_0 = 38;
pub const RTM_DELQDISC: C2RustUnnamed_0 = 37;
pub const RTM_NEWQDISC: C2RustUnnamed_0 = 36;
pub const RTM_GETRULE: C2RustUnnamed_0 = 34;
pub const RTM_DELRULE: C2RustUnnamed_0 = 33;
pub const RTM_NEWRULE: C2RustUnnamed_0 = 32;
pub const RTM_GETNEIGH: C2RustUnnamed_0 = 30;
pub const RTM_DELNEIGH: C2RustUnnamed_0 = 29;
pub const RTM_NEWNEIGH: C2RustUnnamed_0 = 28;
pub const RTM_GETROUTE: C2RustUnnamed_0 = 26;
pub const RTM_DELROUTE: C2RustUnnamed_0 = 25;
pub const RTM_NEWROUTE: C2RustUnnamed_0 = 24;
pub const RTM_GETADDR: C2RustUnnamed_0 = 22;
pub const RTM_DELADDR: C2RustUnnamed_0 = 21;
pub const RTM_NEWADDR: C2RustUnnamed_0 = 20;
pub const RTM_SETLINK: C2RustUnnamed_0 = 19;
pub const RTM_GETLINK: C2RustUnnamed_0 = 18;
pub const RTM_DELLINK: C2RustUnnamed_0 = 17;
pub const RTM_NEWLINK: C2RustUnnamed_0 = 16;
pub const RTM_BASE: C2RustUnnamed_0 = 16;
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
pub struct rtnl_handle {
  pub fd: libc::c_int,
  pub local: sockaddr_nl,
  pub peer: sockaddr_nl,
  pub seq: uint32_t,
  pub dump: uint32_t,
}
/* vi: set sw=4 ts=4: */
/*
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License
 * as published by the Free Software Foundation; either version
 * 2 of the License, or (at your option) any later version.
 *
 * Authors: Alexey Kuznetsov, <kuznet@ms2.inr.ac.ru>
 */
/* struct ifreq and co. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct idxmap {
  pub next: *mut idxmap,
  pub index: libc::c_int,
  pub type_0: libc::c_int,
  pub alen: libc::c_int,
  pub flags: libc::c_uint,
  pub addr: [libc::c_uchar; 8],
  pub name: [libc::c_char; 16],
}
static mut idxmap: *mut *mut idxmap = 0 as *const *mut idxmap as *mut *mut idxmap;
/* treat as *idxmap[16] */
unsafe extern "C" fn find_by_index(mut idx: libc::c_int) -> *mut idxmap {
  let mut im: *mut idxmap = 0 as *mut idxmap;
  if !idxmap.is_null() {
    im = *idxmap.offset((idx & 0xfi32) as isize);
    while !im.is_null() {
      if (*im).index == idx {
        return im;
      }
      im = (*im).next
    }
  }
  return 0 as *mut idxmap;
}
/* vi: set sw=4 ts=4: */
#[no_mangle]
pub unsafe extern "C" fn ll_remember_index(
  mut _who: *const sockaddr_nl,
  mut n: *mut nlmsghdr,
  mut _arg: *mut libc::c_void,
) -> libc::c_int {
  let mut current_block: u64;
  let mut h: libc::c_int = 0;
  let mut ifi: *mut ifinfomsg = (n as *mut libc::c_char).offset(
    (0i32
      + ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
        .wrapping_add(4u32 as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong)
        & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong) as libc::c_int) as isize,
  ) as *mut libc::c_void as *mut ifinfomsg;
  let mut im: *mut idxmap = 0 as *mut idxmap;
  let mut imp: *mut *mut idxmap = 0 as *mut *mut idxmap;
  let mut tb: [*mut rtattr; 50] = [0 as *mut rtattr; 50];
  if (*n).nlmsg_type as libc::c_int != RTM_NEWLINK as libc::c_int {
    return 0i32;
  }
  if ((*n).nlmsg_len as libc::c_ulong)
    < (::std::mem::size_of::<*mut ifinfomsg>() as libc::c_ulong).wrapping_add(
      ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
        .wrapping_add(4u32 as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong)
        & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong) as libc::c_int
        as libc::c_ulong,
    )
  {
    return -1i32;
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
    ((*n).nlmsg_len as libc::c_ulong).wrapping_sub(
      (::std::mem::size_of::<ifinfomsg>() as libc::c_ulong)
        .wrapping_add(
          ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
            .wrapping_add(4u32 as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong)
            & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong) as libc::c_int
            as libc::c_ulong,
        )
        .wrapping_add(4u32 as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong)
        & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong,
    ) as libc::c_int,
  );
  if tb[IFLA_IFNAME as libc::c_int as usize].is_null() {
    return 0i32;
  }
  if idxmap.is_null() {
    idxmap = xzalloc(
      (::std::mem::size_of::<*mut idxmap>() as libc::c_ulong).wrapping_mul(16i32 as libc::c_ulong),
    ) as *mut *mut idxmap
  }
  h = (*ifi).ifi_index & 0xfi32;
  imp = &mut *idxmap.offset(h as isize) as *mut *mut idxmap;
  loop {
    im = *imp;
    if im.is_null() {
      current_block = 10599921512955367680;
      break;
    }
    if (*im).index == (*ifi).ifi_index {
      current_block = 3282573736978658212;
      break;
    }
    imp = &mut (*im).next
  }
  match current_block {
    10599921512955367680 => {
      im = xmalloc(::std::mem::size_of::<idxmap>() as libc::c_ulong) as *mut idxmap;
      (*im).next = *imp;
      (*im).index = (*ifi).ifi_index;
      *imp = im
    }
    _ => {}
  }
  (*im).type_0 = (*ifi).ifi_type as libc::c_int;
  (*im).flags = (*ifi).ifi_flags;
  if !tb[IFLA_ADDRESS as libc::c_int as usize].is_null() {
    let mut alen: libc::c_int = 0;
    alen = ((*tb[IFLA_ADDRESS as libc::c_int as usize]).rta_len as libc::c_int as libc::c_ulong)
      .wrapping_sub(
        ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
          .wrapping_add(4u32 as libc::c_ulong)
          .wrapping_sub(1i32 as libc::c_ulong)
          & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
          .wrapping_add(0i32 as libc::c_ulong),
      ) as libc::c_int;
    (*im).alen = alen;
    if alen > ::std::mem::size_of::<[libc::c_uchar; 8]>() as libc::c_ulong as libc::c_int {
      alen = ::std::mem::size_of::<[libc::c_uchar; 8]>() as libc::c_ulong as libc::c_int
    }
    memcpy(
      (*im).addr.as_mut_ptr() as *mut libc::c_void,
      (tb[IFLA_ADDRESS as libc::c_int as usize] as *mut libc::c_char).offset(
        ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
          .wrapping_add(4u32 as libc::c_ulong)
          .wrapping_sub(1i32 as libc::c_ulong)
          & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
          .wrapping_add(0i32 as libc::c_ulong) as isize,
      ) as *mut libc::c_void,
      alen as libc::c_ulong,
    );
  } else {
    (*im).alen = 0i32;
    memset(
      (*im).addr.as_mut_ptr() as *mut libc::c_void,
      0i32,
      ::std::mem::size_of::<[libc::c_uchar; 8]>() as libc::c_ulong,
    );
  }
  strcpy(
    (*im).name.as_mut_ptr(),
    (tb[IFLA_IFNAME as libc::c_int as usize] as *mut libc::c_char).offset(
      ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
        .wrapping_add(4u32 as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong)
        & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
        .wrapping_add(0i32 as libc::c_ulong) as isize,
    ) as *mut libc::c_void as *const libc::c_char,
  );
  return 0i32;
}
unsafe extern "C" fn ll_idx_n2a(mut idx: libc::c_int) -> *const libc::c_char
/*, char *buf*/ {
  let mut im: *mut idxmap = 0 as *mut idxmap;
  if idx == 0i32 {
    return b"*\x00" as *const u8 as *const libc::c_char;
  }
  im = find_by_index(idx);
  if !im.is_null() {
    return (*im).name.as_mut_ptr();
  }
  //snprintf(buf, 16, "if%d", idx);
  //return buf;
  return auto_string(xasprintf(
    b"if%d\x00" as *const u8 as *const libc::c_char,
    idx,
  ));
}
//static: const char *ll_idx_n2a(int idx, char *buf) FAST_FUNC;
#[no_mangle]
pub unsafe extern "C" fn ll_index_to_name(mut idx: libc::c_int) -> *const libc::c_char {
  //static char nbuf[16];
  return ll_idx_n2a(idx);
}
/* int ll_index_to_type(int idx); */
#[no_mangle]
pub unsafe extern "C" fn ll_index_to_flags(mut idx: libc::c_int) -> libc::c_uint {
  let mut im: *mut idxmap = 0 as *mut idxmap;
  if idx == 0i32 {
    return 0i32 as libc::c_uint;
  }
  im = find_by_index(idx);
  if !im.is_null() {
    return (*im).flags;
  }
  return 0i32 as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn xll_name_to_index(mut name: *const libc::c_char) -> libc::c_int {
  let mut ret: libc::c_int = 0i32;
  /* caching is not warranted - no users which repeatedly call it */
  ret = if_nametoindex(name) as libc::c_int;
  /* out:*/
  if ret <= 0i32 {
    bb_error_msg_and_die(
      b"can\'t find device \'%s\'\x00" as *const u8 as *const libc::c_char,
      name,
    );
  }
  return ret;
}
#[no_mangle]
pub unsafe extern "C" fn ll_init_map(mut rth: *mut rtnl_handle) -> libc::c_int {
  xrtnl_wilddump_request(rth, 0i32, RTM_GETLINK as libc::c_int);
  xrtnl_dump_filter(
    rth,
    Some(
      ll_remember_index
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
