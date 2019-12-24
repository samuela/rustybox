use libc;
use libc::strcasecmp;
extern "C" {

  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;

}
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

/*
 * vconfig implementation for busybox
 *
 * Copyright (C) 2001  Manuel Novoa III  <mjn3@codepoet.org>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config VCONFIG
//config:	bool "vconfig (2.3 kb)"
//config:	default y
//config:	select PLATFORM_LINUX
//config:	help
//config:	Creates, removes, and configures VLAN interfaces
//applet:IF_VCONFIG(APPLET_NOEXEC(vconfig, vconfig, BB_DIR_SBIN, SUID_DROP, vconfig))
//kbuild:lib-$(CONFIG_VCONFIG) += vconfig.o
//usage:#define vconfig_trivial_usage
//usage:       "COMMAND [OPTIONS]"
//usage:#define vconfig_full_usage "\n\n"
//usage:       "Create and remove virtual ethernet devices\n"
//usage:     "\n	add		IFACE VLAN_ID"
//usage:     "\n	rem		VLAN_NAME"
//usage:     "\n	set_flag	IFACE 0|1 VLAN_QOS"
//usage:     "\n	set_egress_map	VLAN_NAME SKB_PRIO VLAN_QOS"
//usage:     "\n	set_ingress_map	VLAN_NAME SKB_PRIO VLAN_QOS"
//usage:     "\n	set_name_type	NAME_TYPE"
/* BB_AUDIT SUSv3 N/A */
/* Stuff from linux/if_vlan.h, kernel version 2.4.23 */
pub type vlan_ioctl_cmds = libc::c_uint;
pub const SET_VLAN_FLAG_CMD: vlan_ioctl_cmds = 7;
pub const SET_VLAN_NAME_TYPE_CMD: vlan_ioctl_cmds = 6;
pub const GET_VLAN_EGRESS_PRIORITY_CMD: vlan_ioctl_cmds = 5;
pub const GET_VLAN_INGRESS_PRIORITY_CMD: vlan_ioctl_cmds = 4;
pub const SET_VLAN_EGRESS_PRIORITY_CMD: vlan_ioctl_cmds = 3;
pub const SET_VLAN_INGRESS_PRIORITY_CMD: vlan_ioctl_cmds = 2;
pub const DEL_VLAN_CMD: vlan_ioctl_cmds = 1;
pub const ADD_VLAN_CMD: vlan_ioctl_cmds = 0;
pub type vlan_name_types = libc::c_uint;
/* Name will look like:  eth0.5 */
pub const VLAN_NAME_TYPE_HIGHEST: vlan_name_types = 4;
/* Name will look like:  vlan5 */
pub const VLAN_NAME_TYPE_RAW_PLUS_VID_NO_PAD: vlan_name_types = 3;
/* name will look like:  eth1.0005 */
pub const VLAN_NAME_TYPE_PLUS_VID_NO_PAD: vlan_name_types = 2;
/* Name will look like:  vlan0005 */
pub const VLAN_NAME_TYPE_RAW_PLUS_VID: vlan_name_types = 1;
pub const VLAN_NAME_TYPE_PLUS_VID: vlan_name_types = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vlan_ioctl_args {
  pub cmd: libc::c_int,
  pub device1: [libc::c_char; 24],
  pub u: C2RustUnnamed,
  pub vlan_qos: libc::c_short,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed {
  pub device2: [libc::c_char; 24],
  pub VID: libc::c_int,
  pub skb_priority: libc::c_uint,
  pub name_type: libc::c_uint,
  pub bind_type: libc::c_uint,
  pub flag: libc::c_uint,
}
/* Set 802.1Q VLAN options */
/* On entry, table points to the length of the current string
 * plus NUL terminator plus data length for the subsequent entry.
 * The return value is the last data entry for the matching string. */
unsafe extern "C" fn xfind_str(
  mut table: *const libc::c_char,
  mut str: *const libc::c_char,
) -> *const libc::c_char {
  while strcasecmp(str, table.offset(1)) != 0 {
    if *table.offset(0) == 0 {
      crate::libbb::appletlib::bb_show_usage();
    }
    table = table.offset(*table.offset(0) as libc::c_int as isize)
  }
  return table.offset(-1);
}
static mut cmds: [libc::c_char; 80] = [
  4i32 as libc::c_char,
  ADD_VLAN_CMD as libc::c_int as libc::c_char,
  7i32 as libc::c_char,
  'a' as i32 as libc::c_char,
  'd' as i32 as libc::c_char,
  'd' as i32 as libc::c_char,
  0 as libc::c_char,
  3i32 as libc::c_char,
  DEL_VLAN_CMD as libc::c_int as libc::c_char,
  7i32 as libc::c_char,
  'r' as i32 as libc::c_char,
  'e' as i32 as libc::c_char,
  'm' as i32 as libc::c_char,
  0 as libc::c_char,
  3i32 as libc::c_char,
  SET_VLAN_NAME_TYPE_CMD as libc::c_int as libc::c_char,
  17i32 as libc::c_char,
  's' as i32 as libc::c_char,
  'e' as i32 as libc::c_char,
  't' as i32 as libc::c_char,
  '_' as i32 as libc::c_char,
  'n' as i32 as libc::c_char,
  'a' as i32 as libc::c_char,
  'm' as i32 as libc::c_char,
  'e' as i32 as libc::c_char,
  '_' as i32 as libc::c_char,
  't' as i32 as libc::c_char,
  'y' as i32 as libc::c_char,
  'p' as i32 as libc::c_char,
  'e' as i32 as libc::c_char,
  0 as libc::c_char,
  5i32 as libc::c_char,
  SET_VLAN_FLAG_CMD as libc::c_int as libc::c_char,
  12i32 as libc::c_char,
  's' as i32 as libc::c_char,
  'e' as i32 as libc::c_char,
  't' as i32 as libc::c_char,
  '_' as i32 as libc::c_char,
  'f' as i32 as libc::c_char,
  'l' as i32 as libc::c_char,
  'a' as i32 as libc::c_char,
  'g' as i32 as libc::c_char,
  0 as libc::c_char,
  5i32 as libc::c_char,
  SET_VLAN_EGRESS_PRIORITY_CMD as libc::c_int as libc::c_char,
  18i32 as libc::c_char,
  's' as i32 as libc::c_char,
  'e' as i32 as libc::c_char,
  't' as i32 as libc::c_char,
  '_' as i32 as libc::c_char,
  'e' as i32 as libc::c_char,
  'g' as i32 as libc::c_char,
  'r' as i32 as libc::c_char,
  'e' as i32 as libc::c_char,
  's' as i32 as libc::c_char,
  's' as i32 as libc::c_char,
  '_' as i32 as libc::c_char,
  'm' as i32 as libc::c_char,
  'a' as i32 as libc::c_char,
  'p' as i32 as libc::c_char,
  0 as libc::c_char,
  5i32 as libc::c_char,
  SET_VLAN_INGRESS_PRIORITY_CMD as libc::c_int as libc::c_char,
  0 as libc::c_char,
  's' as i32 as libc::c_char,
  'e' as i32 as libc::c_char,
  't' as i32 as libc::c_char,
  '_' as i32 as libc::c_char,
  'i' as i32 as libc::c_char,
  'n' as i32 as libc::c_char,
  'g' as i32 as libc::c_char,
  'r' as i32 as libc::c_char,
  'e' as i32 as libc::c_char,
  's' as i32 as libc::c_char,
  's' as i32 as libc::c_char,
  '_' as i32 as libc::c_char,
  'm' as i32 as libc::c_char,
  'a' as i32 as libc::c_char,
  'p' as i32 as libc::c_char,
  0 as libc::c_char,
];
static mut name_types: [libc::c_char; 76] = [
  VLAN_NAME_TYPE_PLUS_VID as libc::c_int as libc::c_char,
  16i32 as libc::c_char,
  'V' as i32 as libc::c_char,
  'L' as i32 as libc::c_char,
  'A' as i32 as libc::c_char,
  'N' as i32 as libc::c_char,
  '_' as i32 as libc::c_char,
  'P' as i32 as libc::c_char,
  'L' as i32 as libc::c_char,
  'U' as i32 as libc::c_char,
  'S' as i32 as libc::c_char,
  '_' as i32 as libc::c_char,
  'V' as i32 as libc::c_char,
  'I' as i32 as libc::c_char,
  'D' as i32 as libc::c_char,
  0 as libc::c_char,
  VLAN_NAME_TYPE_PLUS_VID_NO_PAD as libc::c_int as libc::c_char,
  22i32 as libc::c_char,
  'V' as i32 as libc::c_char,
  'L' as i32 as libc::c_char,
  'A' as i32 as libc::c_char,
  'N' as i32 as libc::c_char,
  '_' as i32 as libc::c_char,
  'P' as i32 as libc::c_char,
  'L' as i32 as libc::c_char,
  'U' as i32 as libc::c_char,
  'S' as i32 as libc::c_char,
  '_' as i32 as libc::c_char,
  'V' as i32 as libc::c_char,
  'I' as i32 as libc::c_char,
  'D' as i32 as libc::c_char,
  '_' as i32 as libc::c_char,
  'N' as i32 as libc::c_char,
  'O' as i32 as libc::c_char,
  '_' as i32 as libc::c_char,
  'P' as i32 as libc::c_char,
  'A' as i32 as libc::c_char,
  'D' as i32 as libc::c_char,
  0 as libc::c_char,
  VLAN_NAME_TYPE_RAW_PLUS_VID as libc::c_int as libc::c_char,
  15i32 as libc::c_char,
  'D' as i32 as libc::c_char,
  'E' as i32 as libc::c_char,
  'V' as i32 as libc::c_char,
  '_' as i32 as libc::c_char,
  'P' as i32 as libc::c_char,
  'L' as i32 as libc::c_char,
  'U' as i32 as libc::c_char,
  'S' as i32 as libc::c_char,
  '_' as i32 as libc::c_char,
  'V' as i32 as libc::c_char,
  'I' as i32 as libc::c_char,
  'D' as i32 as libc::c_char,
  0 as libc::c_char,
  VLAN_NAME_TYPE_RAW_PLUS_VID_NO_PAD as libc::c_int as libc::c_char,
  0 as libc::c_char,
  'D' as i32 as libc::c_char,
  'E' as i32 as libc::c_char,
  'V' as i32 as libc::c_char,
  '_' as i32 as libc::c_char,
  'P' as i32 as libc::c_char,
  'L' as i32 as libc::c_char,
  'U' as i32 as libc::c_char,
  'S' as i32 as libc::c_char,
  '_' as i32 as libc::c_char,
  'V' as i32 as libc::c_char,
  'I' as i32 as libc::c_char,
  'D' as i32 as libc::c_char,
  '_' as i32 as libc::c_char,
  'N' as i32 as libc::c_char,
  'O' as i32 as libc::c_char,
  '_' as i32 as libc::c_char,
  'P' as i32 as libc::c_char,
  'A' as i32 as libc::c_char,
  'D' as i32 as libc::c_char,
  0 as libc::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn vconfig_main(
  mut argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut ifr: vlan_ioctl_args = vlan_ioctl_args {
    cmd: 0,
    device1: [0; 24],
    u: C2RustUnnamed { device2: [0; 24] },
    vlan_qos: 0,
  };
  let mut p: *const libc::c_char = std::ptr::null();
  let mut fd: libc::c_int = 0;
  memset(
    &mut ifr as *mut vlan_ioctl_args as *mut libc::c_void,
    0,
    ::std::mem::size_of::<vlan_ioctl_args>() as libc::c_ulong,
  );
  argv = argv.offset(1);
  if (*argv.offset(0)).is_null() {
    crate::libbb::appletlib::bb_show_usage();
  }
  p = xfind_str(cmds.as_ptr().offset(2), *argv.offset(0));
  ifr.cmd = *p as libc::c_int;
  if argc != *p.offset(-1i32 as isize) as libc::c_int {
    crate::libbb::appletlib::bb_show_usage();
  }
  if ifr.cmd == SET_VLAN_NAME_TYPE_CMD as libc::c_int {
    /* set_name_type */
    ifr.u.name_type = *xfind_str(name_types.as_ptr().offset(1), *argv.offset(1)) as libc::c_uint
  } else {
    crate::libbb::xfuncs::strncpy_IFNAMSIZ(ifr.device1.as_mut_ptr(), *argv.offset(1));
    p = *argv.offset(2);
    /* I suppose one could try to combine some of the function calls below,
     * since ifr.u.flag, ifr.u.VID, and ifr.u.skb_priority are all same-sized
     * (unsigned) int members of a unions.  But because of the range checking,
     * doing so wouldn't save that much space and would also make maintenance
     * more of a pain.
     */
    if ifr.cmd == SET_VLAN_FLAG_CMD as libc::c_int {
      /* set_flag */
      ifr.u.flag = crate::libbb::xatonum::xatou_range(p, 0 as libc::c_uint, 1i32 as libc::c_uint);
      /* DM: in order to set reorder header, qos must be set */
      ifr.vlan_qos =
        crate::libbb::xatonum::xatou_range(*argv.offset(3), 0 as libc::c_uint, 7i32 as libc::c_uint)
          as libc::c_short
    } else if ifr.cmd == ADD_VLAN_CMD as libc::c_int {
      /* add */
      ifr.u.VID =
        crate::libbb::xatonum::xatou_range(p, 0 as libc::c_uint, (4096i32 - 1i32) as libc::c_uint)
          as libc::c_int
    } else if ifr.cmd != DEL_VLAN_CMD as libc::c_int {
      /* set_{egress|ingress}_map */
      ifr.u.skb_priority = crate::libbb::xatonum::xatou(p);
      ifr.vlan_qos =
        crate::libbb::xatonum::xatou_range(*argv.offset(3), 0 as libc::c_uint, 7i32 as libc::c_uint)
          as libc::c_short
    }
  }
  fd = crate::libbb::xfuncs_printf::xsocket(2i32, SOCK_STREAM as libc::c_int, 0);
  crate::libbb::xfuncs_printf::ioctl_or_perror_and_die(
    fd,
    0x8983i32 as libc::c_uint,
    &mut ifr as *mut vlan_ioctl_args as *mut libc::c_void,
    b"ioctl error for %s\x00" as *const u8 as *const libc::c_char,
    *argv.offset(0),
  );
  return 0;
}
