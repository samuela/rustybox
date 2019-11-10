use libc;










































use libc::sscanf;

























extern "C" {
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn xsocket(domain: libc::c_int, type_0: libc::c_int, protocol: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xsendto(
    s: libc::c_int,
    buf: *const libc::c_void,
    len: size_t,
    to: *const sockaddr,
    tolen: socklen_t,
  ) -> ssize_t;
  #[no_mangle]
  fn setsockopt_broadcast(fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn strncpy_IFNAMSIZ(dst: *mut libc::c_char, src: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_simple_error_msg(s: *const libc::c_char);
  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);
  #[no_mangle]
  fn ioctl_or_perror_and_die(
    fd: libc::c_int,
    request: libc::c_uint,
    argp: *mut libc::c_void,
    fmt: *const libc::c_char,
    _: ...
  ) -> libc::c_int;
  #[no_mangle]
  fn bb_xioctl(
    fd: libc::c_int,
    request: libc::c_uint,
    argp: *mut libc::c_void,
    ioctl_name: *const libc::c_char,
  ) -> libc::c_int;
  #[no_mangle]
  fn ether_aton_r(__asc: *const libc::c_char, __addr: *mut ether_addr) -> *mut ether_addr;
  #[no_mangle]
  fn ether_hostton(__hostname: *const libc::c_char, __addr: *mut ether_addr) -> libc::c_int;
}

pub type __socklen_t = libc::c_uint;
use crate::librb::size_t;
use libc::ssize_t;


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

use libc::sockaddr;
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
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ether_addr {
  pub ether_addr_octet: [u8; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sync_serial_settings {
  pub clock_rate: libc::c_uint,
  pub clock_type: libc::c_uint,
  pub loopback: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct te1_settings {
  pub clock_rate: libc::c_uint,
  pub clock_type: libc::c_uint,
  pub loopback: libc::c_ushort,
  pub slot_map: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct raw_hdlc_proto {
  pub encoding: libc::c_ushort,
  pub parity: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fr_proto {
  pub t391: libc::c_uint,
  pub t392: libc::c_uint,
  pub n391: libc::c_uint,
  pub n392: libc::c_uint,
  pub n393: libc::c_uint,
  pub lmi: libc::c_ushort,
  pub dce: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fr_proto_pvc {
  pub dlci: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fr_proto_pvc_info {
  pub dlci: libc::c_uint,
  pub master: [libc::c_char; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cisco_proto {
  pub interval: libc::c_uint,
  pub timeout: libc::c_uint,
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
pub struct if_settings {
  pub type_0: libc::c_uint,
  pub size: libc::c_uint,
  pub ifs_ifsu: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
  pub raw_hdlc: *mut raw_hdlc_proto,
  pub cisco: *mut cisco_proto,
  pub fr: *mut fr_proto,
  pub fr_pvc: *mut fr_proto_pvc,
  pub fr_pvc_info: *mut fr_proto_pvc_info,
  pub sync: *mut sync_serial_settings,
  pub te1: *mut te1_settings,
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
  pub ifru_data: *mut libc::c_void,
  pub ifru_settings: if_settings,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
  pub ifrn_name: [libc::c_char; 16],
}

/*
 * ether-wake.c - Send a magic packet to wake up sleeping machines.
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 *
 * Author:      Donald Becker, http://www.scyld.com/"; http://www.scyld.com/wakeonlan.html
 * Busybox port: Christian Volkmann <haveaniceday@online.de>
 *               Used version of ether-wake.c: v1.09 11/12/2003 Donald Becker, http://www.scyld.com/";
 */
/* full usage according Donald Becker
 * usage: ether-wake [-i <ifname>] [-p aa:bb:cc:dd[:ee:ff]] 00:11:22:33:44:55\n"
 *
 *	This program generates and transmits a Wake-On-LAN (WOL)\n"
 *	\"Magic Packet\", used for restarting machines that have been\n"
 *	soft-powered-down (ACPI D3-warm state).\n"
 *	It currently generates the standard AMD Magic Packet format, with\n"
 *	an optional password appended.\n"
 *
 *	The single required parameter is the Ethernet MAC (station) address\n"
 *	of the machine to wake or a host ID with known NSS 'ethers' entry.\n"
 *	The MAC address may be found with the 'arp' program while the target\n"
 *	machine is awake.\n"
 *
 *	Options:\n"
 *		-b	Send wake-up packet to the broadcast address.\n"
 *		-D	Increase the debug level.\n"
 *		-i ifname	Use interface IFNAME instead of the default 'eth0'.\n"
 *		-p <pw>		Append the four or six byte password PW to the packet.\n"
 *					A password is only required for a few adapter types.\n"
 *					The password may be specified in ethernet hex format\n"
 *					or dotted decimal (Internet address)\n"
 *		-p 00:22:44:66:88:aa\n"
 *		-p 192.168.1.1\n";
 *
 *
 *	This program generates and transmits a Wake-On-LAN (WOL) "Magic Packet",
 *	used for restarting machines that have been soft-powered-down
 *	(ACPI D3-warm state).  It currently generates the standard AMD Magic Packet
 *	format, with an optional password appended.
 *
 *	This software may be used and distributed according to the terms
 *	of the GNU Public License, incorporated herein by reference.
 *	Contact the author for use under other terms.
 *
 *	This source file was originally part of the network tricks package, and
 *	is now distributed to support the Scyld Beowulf system.
 *	Copyright 1999-2003 Donald Becker and Scyld Computing Corporation.
 *
 *	The author may be reached as becker@scyld, or C/O
 *	Scyld Computing Corporation
 *	914 Bay Ridge Road, Suite 220
 *	Annapolis MD 21403
 *
 *   Notes:
 *   On some systems dropping root capability allows the process to be
 *   dumped, traced or debugged.
 *   If someone traces this program, they get control of a raw socket.
 *   Linux handles this safely, but beware when porting this program.
 *
 *   An alternative to needing 'root' is using a UDP broadcast socket, however
 *   doing so only works with adapters configured for unicast+broadcast Rx
 *   filter.  That configuration consumes more power.
 */
//config:config ETHER_WAKE
//config:	bool "ether-wake (4.9 kb)"
//config:	default y
//config:	select PLATFORM_LINUX
//config:	help
//config:	Send a magic packet to wake up sleeping machines.
//applet:IF_ETHER_WAKE(APPLET_ODDNAME(ether-wake, ether_wake, BB_DIR_USR_SBIN, BB_SUID_DROP, ether_wake))
//kbuild:lib-$(CONFIG_ETHER_WAKE) += ether-wake.o
//usage:#define ether_wake_trivial_usage
//usage:       "[-b] [-i IFACE] [-p aa:bb:cc:dd[:ee:ff]/a.b.c.d] MAC"
//usage:#define ether_wake_full_usage "\n\n"
//usage:       "Send a magic packet to wake up sleeping machines.\n"
//usage:       "MAC must be a station address (00:11:22:33:44:55) or\n"
//usage:       "a hostname with a known 'ethers' entry.\n"
//usage:     "\n	-b		Broadcast the packet"
//usage:     "\n	-i IFACE	Interface to use (default eth0)"
//usage:     "\n	-p PASSWORD	Append four or six byte PASSWORD to the packet"
/* Note: PF_INET, SOCK_DGRAM, IPPROTO_UDP would allow SIOCGIFHWADDR to
 * work as non-root, but we need SOCK_PACKET to specify the Ethernet
 * destination address.
 */
/* Convert the host ID string to a MAC address.
 * The string may be a:
 *    Host name
 *    IP address string
 *    MAC address string
 */
unsafe extern "C" fn get_dest_addr(mut hostid: *const libc::c_char, mut eaddr: *mut ether_addr) {
  let mut eap: *mut ether_addr = 0 as *mut ether_addr; /* 6 */
  eap = ether_aton_r(hostid, eaddr); /* 12 */
  /* Or 0x0806 for ARP, 0x8035 for RARP */
  if eap.is_null() {
    if ether_hostton(hostid, eaddr) == 0i32 {
    } else {
      bb_show_usage(); /* 13 */
    }
  }; /* 14 */
}
unsafe extern "C" fn fill_pkt_header(
  mut pkt: *mut libc::c_uchar,
  mut eaddr: *mut ether_addr,
  mut broadcast: libc::c_int,
) -> libc::c_int {
  let mut i: libc::c_int = 0;
  let mut station_addr: *mut libc::c_uchar = (*eaddr).ether_addr_octet.as_mut_ptr();
  memset(pkt as *mut libc::c_void, 0xffi32, 6i32 as libc::c_ulong);
  if broadcast == 0 {
    memcpy(
      pkt as *mut libc::c_void,
      station_addr as *const libc::c_void,
      6i32 as libc::c_ulong,
    );
  }
  pkt = pkt.offset(6);
  memcpy(
    pkt as *mut libc::c_void,
    station_addr as *const libc::c_void,
    6i32 as libc::c_ulong,
  );
  pkt = pkt.offset(6);
  let fresh0 = pkt;
  pkt = pkt.offset(1);
  *fresh0 = 0x8i32 as libc::c_uchar;
  let fresh1 = pkt;
  pkt = pkt.offset(1);
  *fresh1 = 0x42i32 as libc::c_uchar;
  memset(pkt as *mut libc::c_void, 0xffi32, 6i32 as libc::c_ulong);
  i = 0i32;
  while i < 16i32 {
    pkt = pkt.offset(6);
    memcpy(
      pkt as *mut libc::c_void,
      station_addr as *const libc::c_void,
      6i32 as libc::c_ulong,
    );
    i += 1
    /* 20,26,32,... */
  }
  return 20i32 + 16i32 * 6i32;
  /* length of packet */
}
unsafe extern "C" fn get_wol_pw(
  mut ethoptarg: *const libc::c_char,
  mut wol_passwd: *mut libc::c_uchar,
) -> libc::c_int {
  let mut passwd: [libc::c_uint; 6] = [0; 6];
  let mut byte_cnt: libc::c_int = 0;
  let mut i: libc::c_int = 0;
  /* handle MAC format */
  byte_cnt = sscanf(
    ethoptarg,
    b"%2x:%2x:%2x:%2x:%2x:%2x\x00" as *const u8 as *const libc::c_char,
    &mut *passwd.as_mut_ptr().offset(0) as *mut libc::c_uint,
    &mut *passwd.as_mut_ptr().offset(1) as *mut libc::c_uint,
    &mut *passwd.as_mut_ptr().offset(2) as *mut libc::c_uint,
    &mut *passwd.as_mut_ptr().offset(3) as *mut libc::c_uint,
    &mut *passwd.as_mut_ptr().offset(4) as *mut libc::c_uint,
    &mut *passwd.as_mut_ptr().offset(5) as *mut libc::c_uint,
  );
  /* handle IP format */
  // FIXME: why < 4?? should it be < 6?
  if byte_cnt < 4i32 {
    byte_cnt = sscanf(
      ethoptarg,
      b"%u.%u.%u.%u\x00" as *const u8 as *const libc::c_char,
      &mut *passwd.as_mut_ptr().offset(0) as *mut libc::c_uint,
      &mut *passwd.as_mut_ptr().offset(1) as *mut libc::c_uint,
      &mut *passwd.as_mut_ptr().offset(2) as *mut libc::c_uint,
      &mut *passwd.as_mut_ptr().offset(3) as *mut libc::c_uint,
    )
  }
  if byte_cnt < 4i32 {
    bb_simple_error_msg(b"can\'t read Wake-On-LAN pass\x00" as *const u8 as *const libc::c_char);
    return 0i32;
  }
  // TODO: check invalid numbers >255??
  i = 0i32; /* Raw socket */
  while i < byte_cnt {
    *wol_passwd.offset(i as isize) = passwd[i as usize] as libc::c_uchar;
    i += 1
  }
  return byte_cnt;
}
#[no_mangle]
pub unsafe extern "C" fn ether_wake_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut ifname: *const libc::c_char = b"eth0\x00" as *const u8 as *const libc::c_char;
  let mut pass: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut flags: libc::c_uint = 0;
  let mut wol_passwd: [libc::c_uchar; 6] = [0; 6];
  let mut wol_passwd_sz: libc::c_int = 0i32;
  let mut s: libc::c_int = 0;
  let mut pktsize: libc::c_int = 0;
  let mut outpack: [libc::c_uchar; 138] = [0; 138];
  let mut eaddr: ether_addr = ether_addr {
    ether_addr_octet: [0; 6],
  };
  /* max passwd size */
  /* paranoia */
  let mut whereto: sockaddr_ll = sockaddr_ll {
    sll_family: 0,
    sll_protocol: 0,
    sll_ifindex: 0,
    sll_hatype: 0,
    sll_pkttype: 0,
    sll_halen: 0,
    sll_addr: [0; 8],
  }; /* who to wake up */
  /* handle misc user options */
  flags = getopt32(
    argv,
    b"^bi:p:\x00=1\x00" as *const u8 as *const libc::c_char,
    &mut ifname as *mut *const libc::c_char,
    &mut pass as *mut *mut libc::c_char,
  );
  if flags & 4i32 as libc::c_uint != 0 {
    /* -p */
    wol_passwd_sz = get_wol_pw(pass, wol_passwd.as_mut_ptr())
  } /* we further interested only in -b [bcast] flag */
  flags &= 1i32 as libc::c_uint;
  /* create the raw socket */
  s = xsocket(17i32, SOCK_RAW as libc::c_int, 0i32);
  /* now that we have a raw socket we can drop root */
  /* xsetuid(getuid()); - but save on code size... */
  /* look up the dest mac address */
  get_dest_addr(*argv.offset(optind as isize), &mut eaddr);
  /* fill out the header of the packet */
  pktsize = fill_pkt_header(outpack.as_mut_ptr(), &mut eaddr, flags as libc::c_int);
  /* Fill in the source address, if possible. */
  let mut if_hwaddr: ifreq = ifreq {
    ifr_ifrn: C2RustUnnamed_1 { ifrn_name: [0; 16] },
    ifr_ifru: C2RustUnnamed_0 {
      ifru_addr: sockaddr {
        sa_family: 0,
        sa_data: [0; 14],
      },
    },
  };
  strncpy_IFNAMSIZ(if_hwaddr.ifr_ifrn.ifrn_name.as_mut_ptr(), ifname);
  ioctl_or_perror_and_die(
    s,
    0x8927i32 as libc::c_uint,
    &mut if_hwaddr as *mut ifreq as *mut libc::c_void,
    b"SIOCGIFHWADDR on %s failed\x00" as *const u8 as *const libc::c_char,
    ifname,
  );
  memcpy(
    outpack.as_mut_ptr().offset(6) as *mut libc::c_void,
    if_hwaddr.ifr_ifru.ifru_hwaddr.sa_data.as_mut_ptr() as *const libc::c_void,
    6i32 as libc::c_ulong,
  );
  /* __linux__ */
  /* append the password if specified */
  if wol_passwd_sz > 0i32 {
    memcpy(
      outpack.as_mut_ptr().offset(pktsize as isize) as *mut libc::c_void,
      wol_passwd.as_mut_ptr() as *const libc::c_void,
      wol_passwd_sz as libc::c_ulong,
    );
    pktsize += wol_passwd_sz
  }
  /* This is necessary for broadcasts to work */
  if flags != 0 {
    /* & 1 OPT_BROADCAST */
    if setsockopt_broadcast(s) != 0i32 {
      bb_simple_perror_msg(b"SO_BROADCAST\x00" as *const u8 as *const libc::c_char);
    }
  }
  let mut ifr: ifreq = ifreq {
    ifr_ifrn: C2RustUnnamed_1 { ifrn_name: [0; 16] },
    ifr_ifru: C2RustUnnamed_0 {
      ifru_addr: sockaddr {
        sa_family: 0,
        sa_data: [0; 14],
      },
    },
  };
  strncpy_IFNAMSIZ(ifr.ifr_ifrn.ifrn_name.as_mut_ptr(), ifname);
  bb_xioctl(
    s,
    0x8933i32 as libc::c_uint,
    &mut ifr as *mut ifreq as *mut libc::c_void,
    b"SIOCGIFINDEX\x00" as *const u8 as *const libc::c_char,
  );
  memset(
    &mut whereto as *mut sockaddr_ll as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<sockaddr_ll>() as libc::c_ulong,
  );
  whereto.sll_family = 17i32 as libc::c_ushort;
  whereto.sll_ifindex = ifr.ifr_ifru.ifru_ivalue;
  /* The manual page incorrectly claims the address must be filled.
  We do so because the code may change to match the docs. */
  whereto.sll_halen = 6i32 as libc::c_uchar;
  memcpy(
    whereto.sll_addr.as_mut_ptr() as *mut libc::c_void,
    outpack.as_mut_ptr() as *const libc::c_void,
    6i32 as libc::c_ulong,
  );
  xsendto(
    s,
    outpack.as_mut_ptr() as *const libc::c_void,
    pktsize as size_t,
    &mut whereto as *mut sockaddr_ll as *mut sockaddr,
    ::std::mem::size_of::<sockaddr_ll>() as libc::c_ulong as socklen_t,
  );
  return 0i32;
}
