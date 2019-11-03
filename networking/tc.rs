use libc;
extern "C" {
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn snprintf(
    _: *mut libc::c_char,
    _: libc::c_ulong,
    _: *const libc::c_char,
    _: ...
  ) -> libc::c_int;
  #[no_mangle]
  fn strtoul(
    __nptr: *const libc::c_char,
    __endptr: *mut *mut libc::c_char,
    __base: libc::c_int,
  ) -> libc::c_ulong;
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn index_in_strings(strings: *const libc::c_char, key: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn index_in_substrings(strings: *const libc::c_char, key: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
  #[no_mangle]
  fn xrtnl_open(rth: *mut rtnl_handle);
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
  fn parse_rtattr(tb: *mut *mut rtattr, max: libc::c_int, rta: *mut rtattr, len: libc::c_int);
  #[no_mangle]
  fn ll_init_map(rth: *mut rtnl_handle) -> libc::c_int;
  #[no_mangle]
  fn xll_name_to_index(name: *const libc::c_char) -> libc::c_int;
  //static: const char *ll_idx_n2a(int idx, char *buf) FAST_FUNC;
  #[no_mangle]
  fn ll_index_to_name(idx: libc::c_int) -> *const libc::c_char;
  #[no_mangle]
  fn next_arg(argv: *mut *mut libc::c_char) -> *mut *mut libc::c_char;
  #[no_mangle]
  fn get_u32(arg: *mut libc::c_char, errmsg: *const libc::c_char) -> uint32_t;
  #[no_mangle]
  fn invarg_1_to_2(_: *const libc::c_char, _: *const libc::c_char) -> !;
  #[no_mangle]
  fn duparg(_: *const libc::c_char, _: *const libc::c_char) -> !;
  #[no_mangle]
  fn duparg2(_: *const libc::c_char, _: *const libc::c_char) -> !;
  #[no_mangle]
  fn ll_proto_a2n(id: *mut libc::c_ushort, buf: *mut libc::c_char) -> libc::c_int;
}
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub filter_ifindex: libc::c_int,
  pub filter_qdisc: uint32_t,
  pub filter_parent: uint32_t,
  pub filter_prio: uint32_t,
  pub filter_proto: uint32_t,
}
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
pub struct tcmsg {
  pub tcm_family: libc::c_uchar,
  pub tcm__pad1: libc::c_uchar,
  pub tcm__pad2: libc::c_ushort,
  pub tcm_ifindex: libc::c_int,
  pub tcm_handle: __u32,
  pub tcm_parent: __u32,
  pub tcm_info: __u32,
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const __TCA_MAX: C2RustUnnamed_0 = 13;
pub const TCA_HW_OFFLOAD: C2RustUnnamed_0 = 12;
pub const TCA_CHAIN: C2RustUnnamed_0 = 11;
pub const TCA_DUMP_INVISIBLE: C2RustUnnamed_0 = 10;
pub const TCA_PAD: C2RustUnnamed_0 = 9;
pub const TCA_STAB: C2RustUnnamed_0 = 8;
pub const TCA_STATS2: C2RustUnnamed_0 = 7;
pub const TCA_FCNT: C2RustUnnamed_0 = 6;
pub const TCA_RATE: C2RustUnnamed_0 = 5;
pub const TCA_XSTATS: C2RustUnnamed_0 = 4;
pub const TCA_STATS: C2RustUnnamed_0 = 3;
pub const TCA_OPTIONS: C2RustUnnamed_0 = 2;
pub const TCA_KIND: C2RustUnnamed_0 = 1;
pub const TCA_UNSPEC: C2RustUnnamed_0 = 0;
/* vi: set sw=4 ts=4: */
/* We need linux/types.h because older kernels use __u32 etc
 * in linux/[rt]netlink.h. 2.6.19 seems to be ok, though */
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
pub struct tc_ratespec {
  pub cell_log: libc::c_uchar,
  pub linklayer: __u8,
  pub overhead: libc::c_ushort,
  pub cell_align: libc::c_short,
  pub mpu: libc::c_ushort,
  pub rate: __u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tc_prio_qopt {
  pub bands: libc::c_int,
  pub priomap: [__u8; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tc_cbq_lssopt {
  pub change: libc::c_uchar,
  pub flags: libc::c_uchar,
  pub ewma_log: libc::c_uchar,
  pub level: libc::c_uchar,
  pub maxidle: __u32,
  pub minidle: __u32,
  pub offtime: __u32,
  pub avpkt: __u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tc_cbq_wrropt {
  pub flags: libc::c_uchar,
  pub priority: libc::c_uchar,
  pub cpriority: libc::c_uchar,
  pub __reserved: libc::c_uchar,
  pub allot: __u32,
  pub weight: __u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tc_cbq_ovl {
  pub strategy: libc::c_uchar,
  pub priority2: libc::c_uchar,
  pub pad: __u16,
  pub penalty: __u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tc_cbq_fopt {
  pub split: __u32,
  pub defmap: __u32,
  pub defchange: __u32,
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub const __TCA_CBQ_MAX: C2RustUnnamed_1 = 8;
pub const TCA_CBQ_POLICE: C2RustUnnamed_1 = 7;
pub const TCA_CBQ_RTAB: C2RustUnnamed_1 = 6;
pub const TCA_CBQ_RATE: C2RustUnnamed_1 = 5;
pub const TCA_CBQ_OVL_STRATEGY: C2RustUnnamed_1 = 4;
pub const TCA_CBQ_FOPT: C2RustUnnamed_1 = 3;
pub const TCA_CBQ_WRROPT: C2RustUnnamed_1 = 2;
pub const TCA_CBQ_LSSOPT: C2RustUnnamed_1 = 1;
pub const TCA_CBQ_UNSPEC: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const __TCA_PRIO_MAX: C2RustUnnamed_2 = 2;
pub const TCA_PRIO_MQ: C2RustUnnamed_2 = 1;
pub const TCA_PRIO_UNSPEC: C2RustUnnamed_2 = 0;
pub const OBJ_class: C2RustUnnamed_5 = 1;
pub const OBJ_qdisc: C2RustUnnamed_5 = 0;
pub const OBJ_filter: C2RustUnnamed_5 = 2;
pub const CMD_show: C2RustUnnamed_4 = 5;
pub const ARG_proto: C2RustUnnamed_3 = 8;
pub const ARG_prio: C2RustUnnamed_3 = 7;
pub const ARG_pref: C2RustUnnamed_3 = 6;
pub const CMD_change: C2RustUnnamed_4 = 2;
pub const ARG_classid: C2RustUnnamed_3 = 5;
pub const ARG_handle: C2RustUnnamed_3 = 4;
pub const ARG_parent: C2RustUnnamed_3 = 2;
pub const ARG_root: C2RustUnnamed_3 = 1;
pub const ARG_qdisc: C2RustUnnamed_3 = 3;
pub const ARG_dev: C2RustUnnamed_3 = 0;
pub type C2RustUnnamed_3 = libc::c_uint;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const CMD_replace: C2RustUnnamed_4 = 4;
pub const CMD_link: C2RustUnnamed_4 = 3;
pub const CMD_del: C2RustUnnamed_4 = 1;
pub const CMD_add: C2RustUnnamed_4 = 0;
pub type C2RustUnnamed_5 = libc::c_uint;
/* Allocates a buffer containing the name of a class id.
 * The caller must free the returned memory.  */
unsafe extern "C" fn print_tc_classid(mut cid: uint32_t) -> *mut libc::c_char {
  /* IMPOSSIBLE */
  if cid == 0u32 {
    return xasprintf(b"none\x00" as *const u8 as *const libc::c_char);
  } else if cid & 0xffff0000u32 == 0i32 as libc::c_uint {
    return xasprintf(
      b":%x\x00" as *const u8 as *const libc::c_char,
      cid & 0xffffu32,
    );
  } else if cid & 0xffffu32 == 0i32 as libc::c_uint {
    return xasprintf(
      b"%x:\x00" as *const u8 as *const libc::c_char,
      (cid & 0xffff0000u32) >> 16i32,
    );
  } else {
    return xasprintf(
      b"%x:%x\x00" as *const u8 as *const libc::c_char,
      (cid & 0xffff0000u32) >> 16i32,
      cid & 0xffffu32,
    );
  };
}
/* Get a qdisc handle.  Return 0 on success, !0 otherwise.  */
unsafe extern "C" fn get_qdisc_handle(
  mut h: *mut uint32_t,
  mut str: *const libc::c_char,
) -> libc::c_int {
  let mut maj: uint32_t = 0;
  let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
  maj = 0u32;
  if !(strcmp(str, b"none\x00" as *const u8 as *const libc::c_char) == 0i32) {
    maj = strtoul(str, &mut p, 16i32) as uint32_t;
    if p == str as *mut libc::c_char {
      return 1i32;
    }
    maj <<= 16i32;
    if *p as libc::c_int != ':' as i32 && *p as libc::c_int != '\u{0}' as i32 {
      return 1i32;
    }
  }
  *h = maj;
  return 0i32;
}
/* Get class ID.  Return 0 on success, !0 otherwise.  */
unsafe extern "C" fn get_tc_classid(
  mut h: *mut uint32_t,
  mut str: *const libc::c_char,
) -> libc::c_int {
  let mut maj: uint32_t = 0;
  let mut min: uint32_t = 0;
  let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
  maj = 0xffffffffu32;
  if !(strcmp(str, b"root\x00" as *const u8 as *const libc::c_char) == 0i32) {
    maj = 0u32;
    if !(strcmp(str, b"none\x00" as *const u8 as *const libc::c_char) == 0i32) {
      maj = strtoul(str, &mut p, 16i32) as uint32_t;
      if p == str as *mut libc::c_char {
        if *p as libc::c_int != ':' as i32 {
          return 1i32;
        }
        maj = 0i32 as uint32_t
      }
      if *p as libc::c_int == ':' as i32 {
        if maj >= (1i32 << 16i32) as libc::c_uint {
          return 1i32;
        }
        maj <<= 16i32;
        str = p.offset(1);
        min = strtoul(str, &mut p, 16i32) as uint32_t;
        //FIXME: check for "" too?
        if *p as libc::c_int != '\u{0}' as i32 || min >= (1i32 << 16i32) as libc::c_uint {
          return 1i32;
        }
        maj |= min
      } else if *p as libc::c_int != 0i32 {
        return 1i32;
      }
    }
  }
  *h = maj;
  return 0i32;
}
unsafe extern "C" fn print_rate(
  mut buf: *mut libc::c_char,
  mut len: libc::c_int,
  mut rate: uint32_t,
) {
  let mut tmp: libc::c_double = rate as libc::c_double * 8i32 as libc::c_double;
  if tmp >= (1000i32 * 1000000i32) as libc::c_double {
    snprintf(
      buf,
      len as libc::c_ulong,
      b"%.0fMbit\x00" as *const u8 as *const libc::c_char,
      tmp / 1000000i32 as libc::c_double,
    );
  } else if tmp >= (1000i32 * 1000i32) as libc::c_double {
    snprintf(
      buf,
      len as libc::c_ulong,
      b"%.0fKbit\x00" as *const u8 as *const libc::c_char,
      tmp / 1000i32 as libc::c_double,
    );
  } else {
    snprintf(
      buf,
      len as libc::c_ulong,
      b"%.0fbit\x00" as *const u8 as *const libc::c_char,
      tmp,
    );
  };
}
unsafe extern "C" fn prio_print_opt(mut opt: *mut rtattr) -> libc::c_int {
  let mut i: libc::c_int = 0;
  let mut qopt: *mut tc_prio_qopt = 0 as *mut tc_prio_qopt;
  let mut tb: [*mut rtattr; 2] = [0 as *mut rtattr; 2];
  if opt.is_null() {
    return 0i32;
  }
  printf(
    b"bands %u priomap \x00" as *const u8 as *const libc::c_char,
    (*qopt).bands,
  );
  i = 0i32;
  while i <= 15i32 {
    printf(
      b" %d\x00" as *const u8 as *const libc::c_char,
      (*qopt).priomap[i as usize] as libc::c_int,
    );
    i += 1
  }
  if !tb[TCA_PRIO_MQ as libc::c_int as usize].is_null() {
    printf(
      b" multiqueue: o%s \x00" as *const u8 as *const libc::c_char,
      if *((tb[TCA_PRIO_MQ as libc::c_int as usize] as *mut libc::c_char).offset(
        ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
          .wrapping_add(4u32 as libc::c_ulong)
          .wrapping_sub(1i32 as libc::c_ulong)
          & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
          .wrapping_add(0i32 as libc::c_ulong) as isize,
      ) as *mut libc::c_void as *mut libc::c_uchar) as libc::c_int
        != 0
      {
        b"n\x00" as *const u8 as *const libc::c_char
      } else {
        b"ff\x00" as *const u8 as *const libc::c_char
      },
    );
  }
  return 0i32;
}
unsafe extern "C" fn cbq_print_opt(mut opt: *mut rtattr) -> libc::c_int {
  let mut tb: [*mut rtattr; 8] = [0 as *mut rtattr; 8];
  let mut r: *mut tc_ratespec = 0 as *mut tc_ratespec;
  let mut lss: *mut tc_cbq_lssopt = 0 as *mut tc_cbq_lssopt;
  let mut wrr: *mut tc_cbq_wrropt = 0 as *mut tc_cbq_wrropt;
  let mut fopt: *mut tc_cbq_fopt = 0 as *mut tc_cbq_fopt;
  let mut ovl: *mut tc_cbq_ovl = 0 as *mut tc_cbq_ovl;
  let error: *const libc::c_char = b"CBQ: too short %s opt\x00" as *const u8 as *const libc::c_char;
  let mut buf: [libc::c_char; 64] = [0; 64];
  if !opt.is_null() {
    parse_rtattr(
      tb.as_mut_ptr(),
      __TCA_CBQ_MAX as libc::c_int - 1i32,
      (opt as *mut libc::c_char).offset(
        ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
          .wrapping_add(4u32 as libc::c_ulong)
          .wrapping_sub(1i32 as libc::c_ulong)
          & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
          .wrapping_add(0i32 as libc::c_ulong) as isize,
      ) as *mut libc::c_void as *mut rtattr,
      ((*opt).rta_len as libc::c_int as libc::c_ulong).wrapping_sub(
        ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
          .wrapping_add(4u32 as libc::c_ulong)
          .wrapping_sub(1i32 as libc::c_ulong)
          & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
          .wrapping_add(0i32 as libc::c_ulong),
      ) as libc::c_int,
    );
    if !tb[TCA_CBQ_RATE as libc::c_int as usize].is_null() {
      if ((*tb[TCA_CBQ_RATE as libc::c_int as usize]).rta_len as libc::c_int as libc::c_ulong)
        .wrapping_sub(
          ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
            .wrapping_add(4u32 as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong)
            & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
            .wrapping_add(0i32 as libc::c_ulong),
        )
        < ::std::mem::size_of::<tc_ratespec>() as libc::c_ulong
      {
        bb_error_msg(error, b"rate\x00" as *const u8 as *const libc::c_char);
      } else {
        r = (tb[TCA_CBQ_RATE as libc::c_int as usize] as *mut libc::c_char).offset(
          ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
            .wrapping_add(4u32 as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong)
            & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
            .wrapping_add(0i32 as libc::c_ulong) as isize,
        ) as *mut libc::c_void as *mut tc_ratespec
      }
    }
    if !tb[TCA_CBQ_LSSOPT as libc::c_int as usize].is_null() {
      if ((*tb[TCA_CBQ_LSSOPT as libc::c_int as usize]).rta_len as libc::c_int as libc::c_ulong)
        .wrapping_sub(
          ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
            .wrapping_add(4u32 as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong)
            & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
            .wrapping_add(0i32 as libc::c_ulong),
        )
        < ::std::mem::size_of::<tc_cbq_lssopt>() as libc::c_ulong
      {
        bb_error_msg(error, b"lss\x00" as *const u8 as *const libc::c_char);
      } else {
        lss = (tb[TCA_CBQ_LSSOPT as libc::c_int as usize] as *mut libc::c_char).offset(
          ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
            .wrapping_add(4u32 as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong)
            & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
            .wrapping_add(0i32 as libc::c_ulong) as isize,
        ) as *mut libc::c_void as *mut tc_cbq_lssopt
      }
    }
    if !tb[TCA_CBQ_WRROPT as libc::c_int as usize].is_null() {
      if ((*tb[TCA_CBQ_WRROPT as libc::c_int as usize]).rta_len as libc::c_int as libc::c_ulong)
        .wrapping_sub(
          ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
            .wrapping_add(4u32 as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong)
            & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
            .wrapping_add(0i32 as libc::c_ulong),
        )
        < ::std::mem::size_of::<tc_cbq_wrropt>() as libc::c_ulong
      {
        bb_error_msg(error, b"wrr\x00" as *const u8 as *const libc::c_char);
      } else {
        wrr = (tb[TCA_CBQ_WRROPT as libc::c_int as usize] as *mut libc::c_char).offset(
          ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
            .wrapping_add(4u32 as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong)
            & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
            .wrapping_add(0i32 as libc::c_ulong) as isize,
        ) as *mut libc::c_void as *mut tc_cbq_wrropt
      }
    }
    if !tb[TCA_CBQ_FOPT as libc::c_int as usize].is_null() {
      if ((*tb[TCA_CBQ_FOPT as libc::c_int as usize]).rta_len as libc::c_int as libc::c_ulong)
        .wrapping_sub(
          ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
            .wrapping_add(4u32 as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong)
            & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
            .wrapping_add(0i32 as libc::c_ulong),
        )
        < ::std::mem::size_of::<tc_cbq_fopt>() as libc::c_ulong
      {
        bb_error_msg(error, b"fopt\x00" as *const u8 as *const libc::c_char);
      } else {
        fopt = (tb[TCA_CBQ_FOPT as libc::c_int as usize] as *mut libc::c_char).offset(
          ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
            .wrapping_add(4u32 as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong)
            & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
            .wrapping_add(0i32 as libc::c_ulong) as isize,
        ) as *mut libc::c_void as *mut tc_cbq_fopt
      }
    }
    if !tb[TCA_CBQ_OVL_STRATEGY as libc::c_int as usize].is_null() {
      if ((*tb[TCA_CBQ_OVL_STRATEGY as libc::c_int as usize]).rta_len as libc::c_int
        as libc::c_ulong)
        .wrapping_sub(
          ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
            .wrapping_add(4u32 as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong)
            & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
            .wrapping_add(0i32 as libc::c_ulong),
        )
        < ::std::mem::size_of::<tc_cbq_ovl>() as libc::c_ulong
      {
        bb_error_msg(
          b"CBQ: too short overlimit strategy %u/%u\x00" as *const u8 as *const libc::c_char,
          ((*tb[TCA_CBQ_OVL_STRATEGY as libc::c_int as usize]).rta_len as libc::c_int
            as libc::c_ulong)
            .wrapping_sub(
              ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
                .wrapping_add(4u32 as libc::c_ulong)
                .wrapping_sub(1i32 as libc::c_ulong)
                & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
                .wrapping_add(0i32 as libc::c_ulong),
            ) as libc::c_uint,
          ::std::mem::size_of::<tc_cbq_ovl>() as libc::c_ulong as libc::c_uint,
        );
      } else {
        ovl = (tb[TCA_CBQ_OVL_STRATEGY as libc::c_int as usize] as *mut libc::c_char).offset(
          ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
            .wrapping_add(4u32 as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong)
            & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
            .wrapping_add(0i32 as libc::c_ulong) as isize,
        ) as *mut libc::c_void as *mut tc_cbq_ovl
      }
    }
    if !r.is_null() {
      print_rate(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        (*r).rate,
      );
      printf(
        b"rate %s \x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
      );
    }
    if !lss.is_null() && (*lss).flags as libc::c_int != 0 {
      let mut comma: bool = 0i32 != 0;
      bb_putchar('(' as i32);
      if (*lss).flags as libc::c_int & 1i32 != 0 {
        printf(b"bounded\x00" as *const u8 as *const libc::c_char);
        comma = 1i32 != 0
      }
      if (*lss).flags as libc::c_int & 2i32 != 0 {
        if comma {
          bb_putchar(',' as i32);
        }
        printf(b"isolated\x00" as *const u8 as *const libc::c_char);
      }
      printf(b") \x00" as *const u8 as *const libc::c_char);
    }
    if !wrr.is_null() {
      if (*wrr).priority as libc::c_int != 8i32 {
        printf(
          b"prio %u\x00" as *const u8 as *const libc::c_char,
          (*wrr).priority as libc::c_int,
        );
      } else {
        printf(b"prio no-transmit\x00" as *const u8 as *const libc::c_char);
      }
    }
  }
  return 0i32;
}
unsafe extern "C" fn print_qdisc(
  mut _who: *const sockaddr_nl,
  mut hdr: *mut nlmsghdr,
  mut _arg: *mut libc::c_void,
) -> libc::c_int {
  let mut msg: *mut tcmsg = (hdr as *mut libc::c_char).offset(
    (0i32
      + ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
        .wrapping_add(4u32 as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong)
        & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong) as libc::c_int) as isize,
  ) as *mut libc::c_void as *mut tcmsg;
  let mut len: libc::c_int = (*hdr).nlmsg_len as libc::c_int;
  let mut tb: [*mut rtattr; 13] = [0 as *mut rtattr; 13];
  let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
  if (*hdr).nlmsg_type as libc::c_int != RTM_NEWQDISC as libc::c_int
    && (*hdr).nlmsg_type as libc::c_int != RTM_DELQDISC as libc::c_int
  {
    /* bb_error_msg("not a qdisc"); */
    return 0i32;
    /* ??? mimic upstream; should perhaps return -1 */
  }
  len = (len as libc::c_ulong).wrapping_sub(
    (::std::mem::size_of::<tcmsg>() as libc::c_ulong).wrapping_add(
      ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
        .wrapping_add(4u32 as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong)
        & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong) as libc::c_int
        as libc::c_ulong,
    ),
  ) as libc::c_int as libc::c_int;
  if len < 0i32 {
    /* bb_error_msg("wrong len %d", len); */
    return -1i32;
  }
  /* not the desired interface? */
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).filter_ifindex != 0
    && (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).filter_ifindex != (*msg).tcm_ifindex
  {
    return 0i32;
  }
  memset(
    tb.as_mut_ptr() as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<[*mut rtattr; 13]>() as libc::c_ulong,
  );
  parse_rtattr(
    tb.as_mut_ptr(),
    __TCA_MAX as libc::c_int - 1i32,
    (msg as *mut libc::c_char).offset(
      ((::std::mem::size_of::<tcmsg>() as libc::c_ulong)
        .wrapping_add(4u32 as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong)
        & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong) as isize,
    ) as *mut rtattr,
    len,
  );
  if tb[TCA_KIND as libc::c_int as usize].is_null() {
    /* bb_error_msg("%s: NULL kind", "qdisc"); */
    return -1i32;
  }
  if (*hdr).nlmsg_type as libc::c_int == RTM_DELQDISC as libc::c_int {
    printf(b"deleted \x00" as *const u8 as *const libc::c_char);
  }
  name = (tb[TCA_KIND as libc::c_int as usize] as *mut libc::c_char).offset(
    ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
      .wrapping_add(4u32 as libc::c_ulong)
      .wrapping_sub(1i32 as libc::c_ulong)
      & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
      .wrapping_add(0i32 as libc::c_ulong) as isize,
  ) as *mut libc::c_void as *mut libc::c_char;
  printf(
    b"qdisc %s %x: \x00" as *const u8 as *const libc::c_char,
    name,
    (*msg).tcm_handle >> 16i32,
  );
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).filter_ifindex == 0i32 {
    printf(
      b"dev %s \x00" as *const u8 as *const libc::c_char,
      ll_index_to_name((*msg).tcm_ifindex),
    );
  }
  if (*msg).tcm_parent == 0xffffffffu32 {
    printf(b"root \x00" as *const u8 as *const libc::c_char);
  } else if (*msg).tcm_parent != 0 {
    let mut classid: *mut libc::c_char = print_tc_classid((*msg).tcm_parent);
    printf(
      b"parent %s \x00" as *const u8 as *const libc::c_char,
      classid,
    );
  }
  if (*msg).tcm_info != 1i32 as libc::c_uint {
    printf(
      b"refcnt %d \x00" as *const u8 as *const libc::c_char,
      (*msg).tcm_info,
    );
  }
  if !tb[TCA_OPTIONS as libc::c_int as usize].is_null() {
    static mut _q_: [libc::c_char; 16] = [
      112, 102, 105, 102, 111, 95, 102, 97, 115, 116, 0, 99, 98, 113, 0, 0,
    ];
    let mut qqq: libc::c_int = index_in_strings(_q_.as_ptr(), name);
    if qqq == 0i32 {
      /* pfifo_fast aka prio */
      prio_print_opt(tb[TCA_OPTIONS as libc::c_int as usize]);
    } else if qqq == 1i32 {
      /* class based queuing */
      cbq_print_opt(tb[TCA_OPTIONS as libc::c_int as usize]);
    } else {
      bb_error_msg(b"unknown %s\x00" as *const u8 as *const libc::c_char, name);
    }
  }
  bb_putchar('\n' as i32);
  return 0i32;
}
unsafe extern "C" fn print_class(
  mut _who: *const sockaddr_nl,
  mut hdr: *mut nlmsghdr,
  mut _arg: *mut libc::c_void,
) -> libc::c_int {
  let mut msg: *mut tcmsg = (hdr as *mut libc::c_char).offset(
    (0i32
      + ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
        .wrapping_add(4u32 as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong)
        & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong) as libc::c_int) as isize,
  ) as *mut libc::c_void as *mut tcmsg;
  let mut len: libc::c_int = (*hdr).nlmsg_len as libc::c_int;
  let mut tb: [*mut rtattr; 13] = [0 as *mut rtattr; 13];
  let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut classid: *mut libc::c_char = 0 as *mut libc::c_char;
  /*XXX Eventually factor out common code */
  if (*hdr).nlmsg_type as libc::c_int != RTM_NEWTCLASS as libc::c_int
    && (*hdr).nlmsg_type as libc::c_int != RTM_DELTCLASS as libc::c_int
  {
    /* bb_error_msg("not a class"); */
    return 0i32;
    /* ??? mimic upstream; should perhaps return -1 */
  }
  len = (len as libc::c_ulong).wrapping_sub(
    (::std::mem::size_of::<tcmsg>() as libc::c_ulong).wrapping_add(
      ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
        .wrapping_add(4u32 as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong)
        & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong) as libc::c_int
        as libc::c_ulong,
    ),
  ) as libc::c_int as libc::c_int;
  if len < 0i32 {
    /* bb_error_msg("wrong len %d", len); */
    return -1i32;
  }
  /* not the desired interface? */
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).filter_qdisc != 0
    && ((*msg).tcm_handle ^ (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).filter_qdisc)
      & 0xffff0000u32
      != 0
  {
    return 0i32;
  }
  memset(
    tb.as_mut_ptr() as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<[*mut rtattr; 13]>() as libc::c_ulong,
  );
  parse_rtattr(
    tb.as_mut_ptr(),
    __TCA_MAX as libc::c_int - 1i32,
    (msg as *mut libc::c_char).offset(
      ((::std::mem::size_of::<tcmsg>() as libc::c_ulong)
        .wrapping_add(4u32 as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong)
        & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong) as isize,
    ) as *mut rtattr,
    len,
  );
  if tb[TCA_KIND as libc::c_int as usize].is_null() {
    /* bb_error_msg("%s: NULL kind", "class"); */
    return -1i32;
  }
  if (*hdr).nlmsg_type as libc::c_int == RTM_DELTCLASS as libc::c_int {
    printf(b"deleted \x00" as *const u8 as *const libc::c_char);
  }
  name = (tb[TCA_KIND as libc::c_int as usize] as *mut libc::c_char).offset(
    ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
      .wrapping_add(4u32 as libc::c_ulong)
      .wrapping_sub(1i32 as libc::c_ulong)
      & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
      .wrapping_add(0i32 as libc::c_ulong) as isize,
  ) as *mut libc::c_void as *mut libc::c_char;
  classid = if (*msg).tcm_handle == 0 {
    0 as *mut libc::c_char
  } else {
    print_tc_classid(
      if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).filter_qdisc != 0 {
        ((*msg).tcm_parent) & 0xffffu32
      } else {
        (*msg).tcm_parent
      },
    )
  };
  printf(
    b"class %s %s\x00" as *const u8 as *const libc::c_char,
    name,
    classid,
  );
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).filter_ifindex == 0i32 {
    printf(
      b"dev %s \x00" as *const u8 as *const libc::c_char,
      ll_index_to_name((*msg).tcm_ifindex),
    );
  }
  if (*msg).tcm_parent == 0xffffffffu32 {
    printf(b"root \x00" as *const u8 as *const libc::c_char);
  } else if (*msg).tcm_parent != 0 {
    classid = print_tc_classid(
      if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).filter_qdisc != 0 {
        ((*msg).tcm_parent) & 0xffffu32
      } else {
        (*msg).tcm_parent
      },
    );
    printf(
      b"parent %s \x00" as *const u8 as *const libc::c_char,
      classid,
    );
  }
  if (*msg).tcm_info != 0 {
    printf(
      b"leaf %x \x00" as *const u8 as *const libc::c_char,
      (*msg).tcm_info >> 16i32,
    );
  }
  /* Do that get_qdisc_kind(RTA_DATA(tb[TCA_KIND])).  */
  if !tb[TCA_OPTIONS as libc::c_int as usize].is_null() {
    static mut _q_: [libc::c_char; 16] = [
      112, 102, 105, 102, 111, 95, 102, 97, 115, 116, 0, 99, 98, 113, 0, 0,
    ];
    let mut qqq: libc::c_int = index_in_strings(_q_.as_ptr(), name);
    if !(qqq == 0i32) {
      if qqq == 1i32 {
        /* class based queuing */
        /* cbq_print_copt() is identical to cbq_print_opt(). */
        cbq_print_opt(tb[TCA_OPTIONS as libc::c_int as usize]);
      } else {
        bb_error_msg(b"unknown %s\x00" as *const u8 as *const libc::c_char, name);
      }
    }
  }
  bb_putchar('\n' as i32);
  return 0i32;
}
unsafe extern "C" fn print_filter(
  mut _who: *const sockaddr_nl,
  mut _hdr: *mut nlmsghdr,
  mut _arg: *mut libc::c_void,
) -> libc::c_int {
  return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn tc_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  static mut objects: [libc::c_char; 20] = [
    113, 100, 105, 115, 99, 0, 99, 108, 97, 115, 115, 0, 102, 105, 108, 116, 101, 114, 0, 0,
  ];
  static mut commands: [libc::c_char; 42] = [
    97, 100, 100, 0, 100, 101, 108, 101, 116, 101, 0, 99, 104, 97, 110, 103, 101, 0, 108, 105, 110,
    107, 0, 114, 101, 112, 108, 97, 99, 101, 0, 115, 104, 111, 119, 0, 108, 105, 115, 116, 0, 0,
  ];
  static mut args: [libc::c_char; 67] = [
    100, 101, 118, 0, 114, 111, 111, 116, 0, 112, 97, 114, 101, 110, 116, 0, 113, 100, 105, 115,
    99, 0, 104, 97, 110, 100, 108, 101, 0, 99, 108, 97, 115, 115, 105, 100, 0, 112, 114, 101, 102,
    101, 114, 101, 110, 99, 101, 0, 112, 114, 105, 111, 114, 105, 116, 121, 0, 112, 114, 111, 116,
    111, 99, 111, 108, 0, 0,
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
  let mut msg: tcmsg = tcmsg {
    tcm_family: 0,
    tcm__pad1: 0,
    tcm__pad2: 0,
    tcm_ifindex: 0,
    tcm_handle: 0,
    tcm_parent: 0,
    tcm_info: 0,
  };
  let mut ret: libc::c_int = 0;
  let mut obj: libc::c_int = 0;
  let mut cmd: libc::c_int = 0;
  let mut arg: libc::c_int = 0;
  let mut dev: *mut libc::c_char = 0 as *mut libc::c_char;
  argv = argv.offset(1);
  if (*argv).is_null() {
    bb_show_usage();
  }
  xrtnl_open(&mut rth);
  ret = 0i32;
  let fresh0 = argv;
  argv = argv.offset(1);
  obj = index_in_substrings(objects.as_ptr(), *fresh0);
  if obj < 0i32 {
    bb_show_usage();
  }
  if (*argv).is_null() {
    /* filter */
    cmd = CMD_show as libc::c_int
  } else {
    cmd = index_in_substrings(commands.as_ptr(), *argv); /* list is the default */
    if cmd < 0i32 {
      invarg_1_to_2(*argv, *argv.offset(-1i32 as isize));
    }
    argv = argv.offset(1)
  }
  memset(
    &mut msg as *mut tcmsg as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<tcmsg>() as libc::c_ulong,
  );
  if 0i32 != 0i32 {
    msg.tcm_family = 0i32 as libc::c_uchar
  }
  ll_init_map(&mut rth);
  while !(*argv).is_null() {
    arg = index_in_substrings(args.as_ptr(), *argv);
    if arg == ARG_dev as libc::c_int {
      argv = next_arg(argv);
      if !dev.is_null() {
        duparg2(b"dev\x00" as *const u8 as *const libc::c_char, *argv);
      }
      let fresh1 = argv;
      argv = argv.offset(1);
      dev = *fresh1;
      msg.tcm_ifindex = xll_name_to_index(dev);
      if cmd >= CMD_show as libc::c_int {
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).filter_ifindex = msg.tcm_ifindex
      }
    } else if arg == ARG_qdisc as libc::c_int
      && obj == OBJ_class as libc::c_int
      && cmd >= CMD_show as libc::c_int
      || arg == ARG_handle as libc::c_int
        && obj == OBJ_qdisc as libc::c_int
        && cmd == CMD_change as libc::c_int
    {
      argv = next_arg(argv);
      /* We don't care about duparg2("qdisc handle",*argv) for now */
      if get_qdisc_handle(
        &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).filter_qdisc,
        *argv,
      ) != 0
      {
        invarg_1_to_2(*argv, b"qdisc\x00" as *const u8 as *const libc::c_char);
      }
    } else if obj != OBJ_qdisc as libc::c_int
      && (arg == ARG_root as libc::c_int
        || arg == ARG_parent as libc::c_int
        || obj == OBJ_filter as libc::c_int && arg >= ARG_pref as libc::c_int)
    {
    } else {
      invarg_1_to_2(*argv, b"command\x00" as *const u8 as *const libc::c_char);
    }
    argv = next_arg(argv);
    if arg == ARG_root as libc::c_int {
      if msg.tcm_parent != 0 {
        duparg(b"parent\x00" as *const u8 as *const libc::c_char, *argv);
      }
      msg.tcm_parent = 0xffffffffu32;
      if obj == OBJ_filter as libc::c_int {
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).filter_parent = 0xffffffffu32
      }
    } else if arg == ARG_parent as libc::c_int {
      let mut handle: uint32_t = 0;
      if msg.tcm_parent != 0 {
        duparg(*argv, b"parent\x00" as *const u8 as *const libc::c_char);
      }
      if get_tc_classid(&mut handle, *argv) != 0 {
        invarg_1_to_2(*argv, b"parent\x00" as *const u8 as *const libc::c_char);
      }
      msg.tcm_parent = handle;
      if obj == OBJ_filter as libc::c_int {
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).filter_parent = handle
      }
    } else if arg == ARG_handle as libc::c_int {
      /* filter::list */
      if msg.tcm_handle != 0 {
        duparg(*argv, b"handle\x00" as *const u8 as *const libc::c_char);
      }
      /* if (slash) {if (get_u32(uint32_t &mask, slash+1, NULL)) inv mask; addattr32(n, MAX_MSG, TCA_FW_MASK, mask); */
      msg.tcm_handle = get_u32(*argv, b"handle\x00" as *const u8 as *const libc::c_char)
    } else if !(arg == ARG_classid as libc::c_int
      && obj == OBJ_class as libc::c_int
      && cmd == CMD_change as libc::c_int)
    {
      if arg == ARG_pref as libc::c_int || arg == ARG_prio as libc::c_int {
        /* reject LONG_MIN || LONG_MAX */
        /* TODO: for fw
        slash = strchr(handle, '/');
        if (slash != NULL)
          *slash = '\0';
         */
        /* filter::list */
        if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).filter_prio != 0 {
          duparg(*argv, b"priority\x00" as *const u8 as *const libc::c_char);
        }
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).filter_prio =
          get_u32(*argv, b"priority\x00" as *const u8 as *const libc::c_char)
      } else if arg == ARG_proto as libc::c_int {
        /* filter::list */
        let mut tmp: uint16_t = 0;
        if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).filter_proto != 0 {
          duparg(*argv, b"protocol\x00" as *const u8 as *const libc::c_char);
        }
        if ll_proto_a2n(&mut tmp, *argv) != 0 {
          invarg_1_to_2(*argv, b"protocol\x00" as *const u8 as *const libc::c_char);
        }
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).filter_proto = tmp as uint32_t
      }
    }
  }
  if cmd >= CMD_show as libc::c_int {
    /* show or list */
    if obj == OBJ_filter as libc::c_int {
      msg.tcm_info = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).filter_prio << 16i32
        & 0xffff0000u32
        | (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).filter_proto & 0xffffu32
    }
    if rtnl_dump_request(
      &mut rth,
      if obj == OBJ_qdisc as libc::c_int {
        RTM_GETQDISC as libc::c_int
      } else {
        (if obj == OBJ_class as libc::c_int {
          RTM_GETTCLASS as libc::c_int
        } else {
          RTM_GETTFILTER as libc::c_int
        })
      },
      &mut msg as *mut tcmsg as *mut libc::c_void,
      ::std::mem::size_of::<tcmsg>() as libc::c_ulong as libc::c_int,
    ) < 0i32
    {
      bb_simple_perror_msg_and_die(
        b"can\'t send dump request\x00" as *const u8 as *const libc::c_char,
      );
    }
    xrtnl_dump_filter(
      &mut rth,
      if obj == OBJ_qdisc as libc::c_int {
        Some(
          print_qdisc
            as unsafe extern "C" fn(
              _: *const sockaddr_nl,
              _: *mut nlmsghdr,
              _: *mut libc::c_void,
            ) -> libc::c_int,
        )
      } else if obj == OBJ_class as libc::c_int {
        Some(
          print_class
            as unsafe extern "C" fn(
              _: *const sockaddr_nl,
              _: *mut nlmsghdr,
              _: *mut libc::c_void,
            ) -> libc::c_int,
        )
      } else {
        Some(
          print_filter
            as unsafe extern "C" fn(
              _: *const sockaddr_nl,
              _: *mut nlmsghdr,
              _: *mut libc::c_void,
            ) -> libc::c_int,
        )
      },
      0 as *mut libc::c_void,
    );
  }
  return ret;
}
