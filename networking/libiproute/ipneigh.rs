use libc;


extern "C" {
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
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
  fn index_in_substrings(strings: *const libc::c_char, key: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  static mut applet_name: *const libc::c_char;
  #[no_mangle]
  static bb_msg_requires_arg: [libc::c_char; 0];
  #[no_mangle]
  static bb_msg_invalid_arg_to: [libc::c_char; 0];
  #[no_mangle]
  fn puts(__s: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
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
  fn next_arg(argv: *mut *mut libc::c_char) -> *mut *mut libc::c_char;

  #[no_mangle]
  fn ll_init_map(rth: *mut rtnl_handle) -> libc::c_int;
  #[no_mangle]
  fn xrtnl_open(rth: *mut rtnl_handle);
  #[no_mangle]
  fn xll_name_to_index(name: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn xrtnl_wilddump_request(rth: *mut rtnl_handle, fam: libc::c_int, type_0: libc::c_int);
  #[no_mangle]
  fn rtnl_dump_request(
    rth: *mut rtnl_handle,
    type_0: libc::c_int,
    req: *mut libc::c_void,
    len: libc::c_int,
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
  fn rt_addr_n2a(af: libc::c_int, addr: *mut libc::c_void) -> *const libc::c_char;
  //static: const char *ll_idx_n2a(int idx, char *buf) FAST_FUNC;
  #[no_mangle]
  fn ll_index_to_name(idx: libc::c_int) -> *const libc::c_char;
  //const char *dnet_ntop(int af, const void *addr, char *str, size_t len);
  //int dnet_pton(int af, const char *src, void *addr);
  //const char *ipx_ntop(int af, const void *addr, char *str, size_t len);
  //int ipx_pton(int af, const char *src, void *addr);
  #[no_mangle]
  fn get_hz() -> libc::c_uint;
  #[no_mangle]
  fn invarg_1_to_2(_: *const libc::c_char, _: *const libc::c_char) -> !;
  #[no_mangle]
  fn get_prefix(dst: *mut inet_prefix, arg: *mut libc::c_char, family: libc::c_int);
}

pub type family_t = i8;
pub type __u8 = libc::c_uchar;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ndmsg {
  pub ndm_family: __u8,
  pub ndm_pad1: __u8,
  pub ndm_pad2: __u16,
  pub ndm_ifindex: __s32,
  pub ndm_state: __u16,
  pub ndm_flags: __u8,
  pub ndm_type: __u8,
}
pub type C2RustUnnamed = libc::c_uint;
pub const __NDA_MAX: C2RustUnnamed = 12;
pub const NDA_SRC_VNI: C2RustUnnamed = 11;
pub const NDA_LINK_NETNSID: C2RustUnnamed = 10;
pub const NDA_MASTER: C2RustUnnamed = 9;
pub const NDA_IFINDEX: C2RustUnnamed = 8;
pub const NDA_VNI: C2RustUnnamed = 7;
pub const NDA_PORT: C2RustUnnamed = 6;
pub const NDA_VLAN: C2RustUnnamed = 5;
pub const NDA_PROBES: C2RustUnnamed = 4;
pub const NDA_CACHEINFO: C2RustUnnamed = 3;
pub const NDA_LLADDR: C2RustUnnamed = 2;
pub const NDA_DST: C2RustUnnamed = 1;
pub const NDA_UNSPEC: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nda_cacheinfo {
  pub ndm_confirmed: u32,
  pub ndm_used: u32,
  pub ndm_updated: u32,
  pub ndm_refcnt: u32,
}
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
pub const xshow_stats: C2RustUnnamed_2 = 3;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct filter_t {
  pub family: libc::c_int,
  pub index: libc::c_int,
  pub state: libc::c_int,
  pub unused_only: libc::c_int,
  pub pfx: inet_prefix,
  pub flushed: libc::c_int,
  pub flushb: *mut libc::c_char,
  pub flushp: libc::c_int,
  pub flushe: libc::c_int,
  pub rth: *mut rtnl_handle,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct inet_prefix {
  pub family: u8,
  pub bytelen: u8,
  pub bitlen: i16,
  pub data: [u32; 4],
}
pub const KW_to: C2RustUnnamed_1 = 0;
pub const KW_nud: C2RustUnnamed_1 = 2;
pub const KW_dev: C2RustUnnamed_1 = 1;
pub type C2RustUnnamed_1 = libc::c_uint;

/*
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 *
 * Authors: Alexey Kuznetsov, <kuznet@ms2.inr.ac.ru>
 *
 * Ported to Busybox by:  Curt Brune <curt@cumulusnetworks.com>
 */
//static int xshow_stats = 3;
pub type C2RustUnnamed_2 = libc::c_uint;
#[inline]
unsafe extern "C" fn rta_getattr_u32(mut rta: *const rtattr) -> u32 {
  return *((rta as *mut libc::c_char).offset(
    ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
      .wrapping_add(4u32 as libc::c_ulong)
      .wrapping_sub(1i32 as libc::c_ulong)
      & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
      .wrapping_add(0i32 as libc::c_ulong) as isize,
  ) as *mut libc::c_void as *mut u32);
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
unsafe extern "C" fn nud_state_a2n(mut arg: *mut libc::c_char) -> libc::c_uint {
  static mut keywords: [libc::c_char; 68] = [
    112, 101, 114, 109, 97, 110, 101, 110, 116, 0, 114, 101, 97, 99, 104, 97, 98, 108, 101, 0, 110,
    111, 97, 114, 112, 0, 110, 111, 110, 101, 0, 115, 116, 97, 108, 101, 0, 105, 110, 99, 111, 109,
    112, 108, 101, 116, 101, 0, 100, 101, 108, 97, 121, 0, 112, 114, 111, 98, 101, 0, 102, 97, 105,
    108, 101, 100, 0, 0,
  ];
  static mut nuds: [u8; 9] = [
    0x80i32 as u8,
    0x2i32 as u8,
    0x40i32 as u8,
    0i32 as u8,
    0x4i32 as u8,
    0x1i32 as u8,
    0x8i32 as u8,
    0x10i32 as u8,
    0x20i32 as u8,
  ];
  let mut id: libc::c_int = 0;
  id = index_in_substrings(keywords.as_ptr(), arg);
  if id < 0i32 {
    bb_error_msg_and_die(
      bb_msg_invalid_arg_to.as_ptr(),
      arg,
      b"nud state\x00" as *const u8 as *const libc::c_char,
    );
  }
  return nuds[id as usize] as libc::c_uint;
}
unsafe extern "C" fn print_neigh(
  mut _who: *const sockaddr_nl,
  mut n: *mut nlmsghdr,
  mut _arg: *mut libc::c_void,
) -> libc::c_int {
  let mut r: *mut ndmsg = (n as *mut libc::c_char).offset(
    (0i32
      + ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
        .wrapping_add(4u32 as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong)
        & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong) as libc::c_int) as isize,
  ) as *mut libc::c_void as *mut ndmsg;
  let mut len: libc::c_int = (*n).nlmsg_len as libc::c_int;
  let mut tb: [*mut rtattr; 12] = [0 as *mut rtattr; 12];
  if (*n).nlmsg_type as libc::c_int != RTM_NEWNEIGH as libc::c_int
    && (*n).nlmsg_type as libc::c_int != RTM_DELNEIGH as libc::c_int
  {
    bb_error_msg_and_die(
      b"not RTM_NEWNEIGH: %08x %08x %08x\x00" as *const u8 as *const libc::c_char,
      (*n).nlmsg_len,
      (*n).nlmsg_type as libc::c_int,
      (*n).nlmsg_flags as libc::c_int,
    );
  }
  len = (len as libc::c_ulong).wrapping_sub(
    (::std::mem::size_of::<ndmsg>() as libc::c_ulong).wrapping_add(
      ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
        .wrapping_add(4u32 as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong)
        & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong) as libc::c_int
        as libc::c_ulong,
    ),
  ) as libc::c_int as libc::c_int;
  if len < 0i32 {
    bb_error_msg_and_die(
      b"BUG: wrong nlmsg len %d\x00" as *const u8 as *const libc::c_char,
      len,
    );
  }
  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t))
    .flushb
    .is_null()
    && (*n).nlmsg_type as libc::c_int != RTM_NEWNEIGH as libc::c_int
  {
    return 0i32;
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).family != 0
    && (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).family != (*r).ndm_family as libc::c_int
  {
    return 0i32;
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).index != 0
    && (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).index != (*r).ndm_ifindex
  {
    return 0i32;
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).state & (*r).ndm_state as libc::c_int == 0
    && (*r).ndm_flags as libc::c_int & 0x8i32 == 0
    && ((*r).ndm_state as libc::c_int != 0
      || (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).state & 0x100i32 == 0)
    && (*r).ndm_family as libc::c_int != 12i32
  {
    return 0i32;
  }
  parse_rtattr(
    tb.as_mut_ptr(),
    __NDA_MAX as libc::c_int - 1i32,
    (r as *mut libc::c_char).offset(
      ((::std::mem::size_of::<ndmsg>() as libc::c_ulong)
        .wrapping_add(4u32 as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong)
        & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong) as isize,
    ) as *mut rtattr,
    ((*n).nlmsg_len as libc::c_ulong).wrapping_sub(
      (::std::mem::size_of::<ndmsg>() as libc::c_ulong).wrapping_add(
        ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
          .wrapping_add(4u32 as libc::c_ulong)
          .wrapping_sub(1i32 as libc::c_ulong)
          & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong) as libc::c_int
          as libc::c_ulong,
      ),
    ) as libc::c_int,
  );
  if !tb[NDA_DST as libc::c_int as usize].is_null() {
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t))
      .pfx
      .family
      != 0
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
      dst.family = (*r).ndm_family;
      memcpy(
        &mut dst.data as *mut [u32; 4] as *mut libc::c_void,
        (tb[NDA_DST as libc::c_int as usize] as *mut libc::c_char).offset(
          ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
            .wrapping_add(4u32 as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong)
            & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
            .wrapping_add(0i32 as libc::c_ulong) as isize,
        ) as *mut libc::c_void,
        ((*tb[NDA_DST as libc::c_int as usize]).rta_len as libc::c_int as libc::c_ulong)
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
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).unused_only != 0
    && !tb[NDA_CACHEINFO as libc::c_int as usize].is_null()
  {
    let mut ci: *mut nda_cacheinfo =
      (tb[NDA_CACHEINFO as libc::c_int as usize] as *mut libc::c_char).offset(
        ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
          .wrapping_add(4u32 as libc::c_ulong)
          .wrapping_sub(1i32 as libc::c_ulong)
          & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
          .wrapping_add(0i32 as libc::c_ulong) as isize,
      ) as *mut libc::c_void as *mut nda_cacheinfo;
    if (*ci).ndm_refcnt != 0 {
      return 0i32;
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
    (*fn_0).nlmsg_type = RTM_DELNEIGH as libc::c_int as __u16;
    (*fn_0).nlmsg_flags = 0x1i32 as __u16;
    let ref mut fresh0 = (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).rth).seq;
    *fresh0 = (*fresh0).wrapping_add(1);
    (*fn_0).nlmsg_seq = *fresh0;
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).flushp = (fn_0 as *mut libc::c_char)
      .offset((*n).nlmsg_len as isize)
      .wrapping_offset_from((*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).flushb)
      as libc::c_long
      as libc::c_int;
    let ref mut fresh1 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).flushed;
    *fresh1 += 1;
    if (xshow_stats as libc::c_int) < 2i32 {
      return 0i32;
    }
  }
  if !tb[NDA_DST as libc::c_int as usize].is_null() {
    printf(
      b"%s \x00" as *const u8 as *const libc::c_char,
      rt_addr_n2a(
        (*r).ndm_family as libc::c_int,
        (tb[NDA_DST as libc::c_int as usize] as *mut libc::c_char).offset(
          ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
            .wrapping_add(4u32 as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong)
            & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
            .wrapping_add(0i32 as libc::c_ulong) as isize,
        ) as *mut libc::c_void,
      ),
    );
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).index == 0 && (*r).ndm_ifindex != 0 {
    printf(
      b"dev %s \x00" as *const u8 as *const libc::c_char,
      ll_index_to_name((*r).ndm_ifindex),
    );
  }
  if !tb[NDA_LLADDR as libc::c_int as usize].is_null() {
    let mut b1: [libc::c_char; 64] = [0; 64];
    printf(
      b"lladdr %s\x00" as *const u8 as *const libc::c_char,
      ll_addr_n2a(
        (tb[NDA_LLADDR as libc::c_int as usize] as *mut libc::c_char).offset(
          ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
            .wrapping_add(4u32 as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong)
            & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
            .wrapping_add(0i32 as libc::c_ulong) as isize,
        ) as *mut libc::c_void as *mut libc::c_uchar,
        ((*tb[NDA_LLADDR as libc::c_int as usize]).rta_len as libc::c_int as libc::c_ulong)
          .wrapping_sub(
            ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
              .wrapping_add(4u32 as libc::c_ulong)
              .wrapping_sub(1i32 as libc::c_ulong)
              & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
              .wrapping_add(0i32 as libc::c_ulong),
          ) as libc::c_int,
        1i32,
        b1.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
      ),
    );
  }
  if (*r).ndm_flags as libc::c_int & 0x80i32 != 0 {
    printf(b" router\x00" as *const u8 as *const libc::c_char);
  }
  if (*r).ndm_flags as libc::c_int & 0x8i32 != 0 {
    printf(b" proxy\x00" as *const u8 as *const libc::c_char);
  }
  if !tb[NDA_CACHEINFO as libc::c_int as usize].is_null() && xshow_stats as libc::c_int != 0 {
    let mut ci_0: *mut nda_cacheinfo =
      (tb[NDA_CACHEINFO as libc::c_int as usize] as *mut libc::c_char).offset(
        ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
          .wrapping_add(4u32 as libc::c_ulong)
          .wrapping_sub(1i32 as libc::c_ulong)
          & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
          .wrapping_add(0i32 as libc::c_ulong) as isize,
      ) as *mut libc::c_void as *mut nda_cacheinfo;
    let mut hz: libc::c_int = get_hz() as libc::c_int;
    if (*ci_0).ndm_refcnt != 0 {
      printf(
        b" ref %d\x00" as *const u8 as *const libc::c_char,
        (*ci_0).ndm_refcnt,
      );
    }
    printf(
      b" used %d/%d/%d\x00" as *const u8 as *const libc::c_char,
      (*ci_0).ndm_used.wrapping_div(hz as libc::c_uint),
      (*ci_0).ndm_confirmed.wrapping_div(hz as libc::c_uint),
      (*ci_0).ndm_updated.wrapping_div(hz as libc::c_uint),
    );
  }
  if !tb[NDA_PROBES as libc::c_int as usize].is_null() && xshow_stats as libc::c_int != 0 {
    let mut p: u32 = rta_getattr_u32(tb[NDA_PROBES as libc::c_int as usize]);
    printf(b" probes %u\x00" as *const u8 as *const libc::c_char, p);
  }
  /*if (r->ndm_state)*/
  let mut nud: libc::c_int = (*r).ndm_state as libc::c_int;
  let mut c: libc::c_char = ' ' as i32 as libc::c_char;
  if nud & 0x1i32 != 0 {
    printf(
      b"%cINCOMPLETE\x00" as *const u8 as *const libc::c_char,
      c as libc::c_int,
    );
    c = ',' as i32 as libc::c_char
  }
  if nud & 0x2i32 != 0 {
    printf(
      b"%cREACHABLE\x00" as *const u8 as *const libc::c_char,
      c as libc::c_int,
    );
    c = ',' as i32 as libc::c_char
  }
  if nud & 0x4i32 != 0 {
    printf(
      b"%cSTALE\x00" as *const u8 as *const libc::c_char,
      c as libc::c_int,
    );
    c = ',' as i32 as libc::c_char
  }
  if nud & 0x8i32 != 0 {
    printf(
      b"%cDELAY\x00" as *const u8 as *const libc::c_char,
      c as libc::c_int,
    );
    c = ',' as i32 as libc::c_char
  }
  if nud & 0x10i32 != 0 {
    printf(
      b"%cPROBE\x00" as *const u8 as *const libc::c_char,
      c as libc::c_int,
    );
    c = ',' as i32 as libc::c_char
  }
  if nud & 0x20i32 != 0 {
    printf(
      b"%cFAILED\x00" as *const u8 as *const libc::c_char,
      c as libc::c_int,
    );
    c = ',' as i32 as libc::c_char
  }
  if nud & 0x40i32 != 0 {
    printf(
      b"%cNOARP\x00" as *const u8 as *const libc::c_char,
      c as libc::c_int,
    );
    c = ',' as i32 as libc::c_char
  }
  if nud & 0x80i32 != 0 {
    printf(
      b"%cPERMANENT\x00" as *const u8 as *const libc::c_char,
      c as libc::c_int,
    );
    c = ',' as i32 as libc::c_char
  }
  bb_putchar('\n' as i32);
  return 0i32;
}
unsafe extern "C" fn ipneigh_reset_filter() {
  memset(
    bb_common_bufsiz1.as_mut_ptr() as *mut filter_t as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<filter_t>() as libc::c_ulong,
  );
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).state = !0i32;
}
/* Return value becomes exitcode. It's okay to not return at all */
unsafe extern "C" fn ipneigh_list_or_flush(
  mut argv: *mut *mut libc::c_char,
  mut flush: libc::c_int,
) -> libc::c_int {
  static mut keywords: [libc::c_char; 12] = [116, 111, 0, 100, 101, 118, 0, 110, 117, 100, 0, 0];
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
  let mut ndm: ndmsg = {
    let mut init = ndmsg {
      ndm_family: 0i32 as __u8,
      ndm_pad1: 0,
      ndm_pad2: 0,
      ndm_ifindex: 0,
      ndm_state: 0,
      ndm_flags: 0,
      ndm_type: 0,
    };
    init
  };
  let mut filter_dev: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut state_given: libc::c_int = 0i32;
  let mut arg: libc::c_int = 0;
  ipneigh_reset_filter();
  if flush != 0 && (*argv).is_null() {
    bb_error_msg_and_die(
      bb_msg_requires_arg.as_ptr(),
      b"\"ip neigh flush\"\x00" as *const u8 as *const libc::c_char,
    );
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).family == 0 {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).family = preferred_family as libc::c_int
  }
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).state = if flush != 0 {
    !(0x80i32 | 0x40i32)
  } else {
    (0xffi32) & !0x40i32
  };
  while !(*argv).is_null() {
    arg = index_in_substrings(keywords.as_ptr(), *argv);
    if arg == KW_dev as libc::c_int {
      argv = next_arg(argv);
      filter_dev = *argv
    } else if arg == KW_nud as libc::c_int {
      let mut state: libc::c_uint = 0;
      argv = next_arg(argv);
      if state_given == 0 {
        state_given = 1i32;
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).state = 0i32
      }
      if strcmp(*argv, b"all\x00" as *const u8 as *const libc::c_char) == 0i32 {
        state = !0i32 as libc::c_uint;
        if flush != 0 {
          state &= !0x40i32 as libc::c_uint
        }
      } else {
        state = nud_state_a2n(*argv)
      }
      if state == 0i32 as libc::c_uint {
        state = 0x100i32 as libc::c_uint
      }
      let ref mut fresh2 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).state;
      *fresh2 = (*fresh2 as libc::c_uint | state) as libc::c_int
    } else {
      if arg == KW_to as libc::c_int {
        argv = next_arg(argv)
      }
      get_prefix(
        &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).pfx,
        *argv,
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).family,
      );
      if (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).family == 0i32 {
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).family =
          (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t))
            .pfx
            .family as libc::c_int
      }
    }
    argv = argv.offset(1)
  }
  xrtnl_open(&mut rth);
  ll_init_map(&mut rth);
  if !filter_dev.is_null() {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).index = xll_name_to_index(filter_dev);
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).index == 0i32 {
      bb_error_msg_and_die(
        b"can\'t find device \'%s\'\x00" as *const u8 as *const libc::c_char,
        filter_dev,
      );
    }
  }
  if flush != 0 {
    let mut round: libc::c_int = 0i32;
    let mut flushb: [libc::c_char; 3584] = [0; 3584];
    let ref mut fresh3 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).flushb;
    *fresh3 = flushb.as_mut_ptr();
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).flushp = 0i32;
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).flushe =
      ::std::mem::size_of::<[libc::c_char; 3584]>() as libc::c_ulong as libc::c_int;
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).state &= !0x20i32;
    let ref mut fresh4 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).rth;
    *fresh4 = &mut rth;
    while round < 10i32 {
      xrtnl_wilddump_request(
        &mut rth,
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).family,
        RTM_GETNEIGH as libc::c_int,
      );
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).flushed = 0i32;
      if xrtnl_dump_filter(
        &mut rth,
        Some(
          print_neigh
            as unsafe extern "C" fn(
              _: *const sockaddr_nl,
              _: *mut nlmsghdr,
              _: *mut libc::c_void,
            ) -> libc::c_int,
        ),
        0 as *mut libc::c_void,
      ) < 0i32
      {
        bb_simple_perror_msg_and_die(b"flush terminated\x00" as *const u8 as *const libc::c_char);
      }
      if (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).flushed == 0i32 {
        if round == 0i32 {
          puts(b"Nothing to flush\x00" as *const u8 as *const libc::c_char);
        } else {
          printf(
            b"*** Flush is complete after %d round(s) ***\n\x00" as *const u8
              as *const libc::c_char,
            round,
          );
        }
        return 0i32;
      }
      round += 1;
      if flush_update() < 0i32 {
        xfunc_die();
      }
      printf(
        b"\n*** Round %d, deleting %d entries ***\n\x00" as *const u8 as *const libc::c_char,
        round,
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).flushed,
      );
    }
    bb_error_msg_and_die(
      b"*** Flush not complete bailing out after %d rounds\x00" as *const u8 as *const libc::c_char,
      10i32,
    );
  }
  ndm.ndm_family = (*(bb_common_bufsiz1.as_mut_ptr() as *mut filter_t)).family as __u8;
  if rtnl_dump_request(
    &mut rth,
    RTM_GETNEIGH as libc::c_int,
    &mut ndm as *mut ndmsg as *mut libc::c_void,
    ::std::mem::size_of::<ndmsg>() as libc::c_ulong as libc::c_int,
  ) < 0i32
  {
    bb_simple_perror_msg_and_die(
      b"can\'t send dump request\x00" as *const u8 as *const libc::c_char,
    );
  }
  if xrtnl_dump_filter(
    &mut rth,
    Some(
      print_neigh
        as unsafe extern "C" fn(
          _: *const sockaddr_nl,
          _: *mut nlmsghdr,
          _: *mut libc::c_void,
        ) -> libc::c_int,
    ),
    0 as *mut libc::c_void,
  ) < 0i32
  {
    bb_simple_error_msg_and_die(b"dump terminated\x00" as *const u8 as *const libc::c_char);
  }
  return 0i32;
}

//int FAST_FUNC print_neigh(struct sockaddr_nl *who, struct nlmsghdr *n, void *arg);
//int FAST_FUNC iproute_monitor(char **argv);
//void FAST_FUNC ipneigh_reset_filter(void);
/* Return value becomes exitcode. It's okay to not return at all */
#[no_mangle]
pub unsafe extern "C" fn do_ipneigh(mut argv: *mut *mut libc::c_char) -> libc::c_int {
  static mut ip_neigh_commands: [libc::c_char; 12] =
    [115, 104, 111, 119, 0, 102, 108, 117, 115, 104, 0, 0];
  let mut command_num: libc::c_int = 0;
  if (*argv).is_null() {
    return ipneigh_list_or_flush(argv, 0i32);
  }
  command_num = index_in_substrings(ip_neigh_commands.as_ptr(), *argv);
  match command_num {
    0 => {
      /* show */
      return ipneigh_list_or_flush(argv.offset(1), 0i32);
    }
    1 => {
      /* flush */
      return ipneigh_list_or_flush(argv.offset(1), 1i32);
    }
    _ => {}
  }
  invarg_1_to_2(*argv, applet_name);
}
