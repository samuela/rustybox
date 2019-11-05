use c2rust_bitfields;
use c2rust_bitfields::BitfieldStruct;
use libc;
extern "C" {
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn strtoul(
    __nptr: *const libc::c_char,
    __endptr: *mut *mut libc::c_char,
    __base: libc::c_int,
  ) -> libc::c_ulong;
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn xsocket(domain: libc::c_int, type_0: libc::c_int, protocol: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xhost2sockaddr(host: *const libc::c_char, port: libc::c_int) -> *mut len_and_sockaddr;
  #[no_mangle]
  fn strncpy_IFNAMSIZ(dst: *mut libc::c_char, src: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xatou_range(str: *const libc::c_char, l: libc::c_uint, u: libc::c_uint) -> libc::c_uint;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn display_interfaces(ifname: *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn in_ether(bufp: *const libc::c_char, sap: *mut sockaddr) -> libc::c_int;
  #[no_mangle]
  fn in_ib(bufp: *const libc::c_char, sap: *mut sockaddr) -> libc::c_int;
  #[no_mangle]
  fn index_in_substrings(strings: *const libc::c_char, key: *const libc::c_char) -> libc::c_int;
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
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __caddr_t = *mut libc::c_char;
pub type __socklen_t = libc::c_uint;
use crate::librb::uint8_t;
pub type uint16_t = __uint16_t;
use crate::librb::uint32_t;
pub type intptr_t = libc::c_long;
pub type smalluint = libc::c_uchar;
pub type socklen_t = __socklen_t;
pub type caddr_t = __caddr_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
  pub sin6_family: sa_family_t,
  pub sin6_port: in_port_t,
  pub sin6_flowinfo: uint32_t,
  pub sin6_addr: in6_addr,
  pub sin6_scope_id: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
  pub __in6_u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
  pub __u6_addr8: [uint8_t; 16],
  pub __u6_addr16: [uint16_t; 8],
  pub __u6_addr32: [uint32_t; 4],
}
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
  pub sin_family: sa_family_t,
  pub sin_port: in_port_t,
  pub sin_addr: in_addr,
  pub sin_zero: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
  pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct len_and_sockaddr {
  pub len: socklen_t,
  pub u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
  pub sa: sockaddr,
  pub sin: sockaddr_in,
  pub sin6: sockaddr_in6,
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub const IFF_DYNAMIC: C2RustUnnamed_1 = 32768;
pub const IFF_AUTOMEDIA: C2RustUnnamed_1 = 16384;
pub const IFF_PORTSEL: C2RustUnnamed_1 = 8192;
pub const IFF_MULTICAST: C2RustUnnamed_1 = 4096;
pub const IFF_SLAVE: C2RustUnnamed_1 = 2048;
pub const IFF_MASTER: C2RustUnnamed_1 = 1024;
pub const IFF_ALLMULTI: C2RustUnnamed_1 = 512;
pub const IFF_PROMISC: C2RustUnnamed_1 = 256;
pub const IFF_NOARP: C2RustUnnamed_1 = 128;
pub const IFF_RUNNING: C2RustUnnamed_1 = 64;
pub const IFF_NOTRAILERS: C2RustUnnamed_1 = 32;
pub const IFF_POINTOPOINT: C2RustUnnamed_1 = 16;
pub const IFF_LOOPBACK: C2RustUnnamed_1 = 8;
pub const IFF_DEBUG: C2RustUnnamed_1 = 4;
pub const IFF_BROADCAST: C2RustUnnamed_1 = 2;
pub const IFF_UP: C2RustUnnamed_1 = 1;
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
  pub ifr_ifrn: C2RustUnnamed_3,
  pub ifr_ifru: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
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
pub union C2RustUnnamed_3 {
  pub ifrn_name: [libc::c_char; 16],
}
/* vi: set sw=4 ts=4: */
/*
 * ifconfig
 *
 * Similar to the standard Unix ifconfig, but with only the necessary
 * parts for AF_INET, and without any printing of if info (for now).
 *
 * Bjorn Wesen, Axis Communications AB
 *
 * Authors of the original ifconfig was:
 *              Fred N. van Kempen, <waltje@uwalt.nl.mugnet.org>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/*
 * Heavily modified by Manuel Novoa III       Mar 6, 2001
 *
 * From initial port to busybox, removed most of the redundancy by
 * converting to a table-driven approach.  Added several (optional)
 * args missing from initial port.
 *
 * Still missing:  media, tunnel.
 *
 * 2002-04-20
 * IPV6 support added by Bart Visscher <magick@linux-fan.com>
 */
//config:config IFCONFIG
//config:	bool "ifconfig (12 kb)"
//config:	default y
//config:	select PLATFORM_LINUX
//config:	help
//config:	Ifconfig is used to configure the kernel-resident network interfaces.
//config:
//config:config FEATURE_IFCONFIG_STATUS
//config:	bool "Enable status reporting output (+7k)"
//config:	default y
//config:	depends on IFCONFIG
//config:	help
//config:	If ifconfig is called with no arguments it will display the status
//config:	of the currently active interfaces.
//config:
//config:config FEATURE_IFCONFIG_SLIP
//config:	bool "Enable slip-specific options \"keepalive\" and \"outfill\""
//config:	default y
//config:	depends on IFCONFIG
//config:	help
//config:	Allow "keepalive" and "outfill" support for SLIP. If you're not
//config:	planning on using serial lines, leave this unchecked.
//config:
//config:config FEATURE_IFCONFIG_MEMSTART_IOADDR_IRQ
//config:	bool "Enable options \"mem_start\", \"io_addr\", and \"irq\""
//config:	default y
//config:	depends on IFCONFIG
//config:	help
//config:	Allow the start address for shared memory, start address for I/O,
//config:	and/or the interrupt line used by the specified device.
//config:
//config:config FEATURE_IFCONFIG_HW
//config:	bool "Enable option \"hw\" (ether only)"
//config:	default y
//config:	depends on IFCONFIG
//config:	help
//config:	Set the hardware address of this interface, if the device driver
//config:	supports  this  operation. Currently, we only support the 'ether'
//config:	class.
//config:
//config:config FEATURE_IFCONFIG_BROADCAST_PLUS
//config:	bool "Set the broadcast automatically"
//config:	default y
//config:	depends on IFCONFIG
//config:	help
//config:	Setting this will make ifconfig attempt to find the broadcast
//config:	automatically if the value '+' is used.
//applet:IF_IFCONFIG(APPLET(ifconfig, BB_DIR_SBIN, BB_SUID_DROP))
//kbuild:lib-$(CONFIG_IFCONFIG) += ifconfig.o interface.o
//usage:#define ifconfig_trivial_usage
//usage:	IF_FEATURE_IFCONFIG_STATUS("[-a]") " interface [address]"
//usage:#define ifconfig_full_usage "\n\n"
//usage:       "Configure a network interface\n"
//usage:     "\n"
//usage:	IF_FEATURE_IPV6(
//usage:       "	[add ADDRESS[/PREFIXLEN]]\n")
//usage:	IF_FEATURE_IPV6(
//usage:       "	[del ADDRESS[/PREFIXLEN]]\n")
//usage:       "	[[-]broadcast [ADDRESS]] [[-]pointopoint [ADDRESS]]\n"
//usage:       "	[netmask ADDRESS] [dstaddr ADDRESS]\n"
//usage:	IF_FEATURE_IFCONFIG_SLIP(
//usage:       "	[outfill NN] [keepalive NN]\n")
//usage:       "	" IF_FEATURE_IFCONFIG_HW("[hw ether" IF_FEATURE_HWIB("|infiniband")" ADDRESS] ") "[metric NN] [mtu NN]\n"
//usage:       "	[[-]trailers] [[-]arp] [[-]allmulti]\n"
//usage:       "	[multicast] [[-]promisc] [txqueuelen NN] [[-]dynamic]\n"
//usage:	IF_FEATURE_IFCONFIG_MEMSTART_IOADDR_IRQ(
//usage:       "	[mem_start NN] [io_addr NN] [irq NN]\n")
//usage:       "	[up|down] ..."
/* I don't know if this is needed for busybox or not.  Anyone? */
/* Defines for glibc2.0 users. */
/* ifr_qlen is ifru_ivalue, but it isn't present in 2.0 kernel headers */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_ifreq {
  pub ifr6_addr: in6_addr,
  pub ifr6_prefixlen: uint32_t,
  pub ifr6_ifindex: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arg1opt {
  pub name: *const libc::c_char,
  pub selector: libc::c_ushort,
  pub ifr_offset: libc::c_ushort,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct options {
  pub name: *const libc::c_char,
  #[bitfield(name = "flags", ty = "libc::c_uint", bits = "0..=5")]
  #[bitfield(name = "arg_flags", ty = "libc::c_uint", bits = "6..=15")]
  pub flags_arg_flags: [u8; 2],
  pub selector: libc::c_ushort,
}
/*
 * Set up the tables.  Warning!  They must have corresponding order!
 */
static mut Arg1Opt: [arg1opt; 16] = [
  {
    let mut init = arg1opt {
      name: b"SIFMETRIC\x00" as *const u8 as *const libc::c_char,
      selector: 0x891ei32 as libc::c_ushort,
      ifr_offset: 16u64 as libc::c_ushort,
    };
    init
  },
  {
    let mut init = arg1opt {
      name: b"SIFMTU\x00" as *const u8 as *const libc::c_char,
      selector: 0x8922i32 as libc::c_ushort,
      ifr_offset: 16u64 as libc::c_ushort,
    };
    init
  },
  {
    let mut init = arg1opt {
      name: b"SIFTXQLEN\x00" as *const u8 as *const libc::c_char,
      selector: 0x8943i32 as libc::c_ushort,
      ifr_offset: 16u64 as libc::c_ushort,
    };
    init
  },
  {
    let mut init = arg1opt {
      name: b"SIFDSTADDR\x00" as *const u8 as *const libc::c_char,
      selector: 0x8918i32 as libc::c_ushort,
      ifr_offset: 16u64 as libc::c_ushort,
    };
    init
  },
  {
    let mut init = arg1opt {
      name: b"SIFNETMASK\x00" as *const u8 as *const libc::c_char,
      selector: 0x891ci32 as libc::c_ushort,
      ifr_offset: 16u64 as libc::c_ushort,
    };
    init
  },
  {
    let mut init = arg1opt {
      name: b"SIFBRDADDR\x00" as *const u8 as *const libc::c_char,
      selector: 0x891ai32 as libc::c_ushort,
      ifr_offset: 16u64 as libc::c_ushort,
    };
    init
  },
  {
    let mut init = arg1opt {
      name: b"SIFHWADDR\x00" as *const u8 as *const libc::c_char,
      selector: 0x8924i32 as libc::c_ushort,
      ifr_offset: 16u64 as libc::c_ushort,
    };
    init
  },
  {
    let mut init = arg1opt {
      name: b"SIFDSTADDR\x00" as *const u8 as *const libc::c_char,
      selector: 0x8918i32 as libc::c_ushort,
      ifr_offset: 16u64 as libc::c_ushort,
    };
    init
  },
  {
    let mut init = arg1opt {
      name: b"SKEEPALIVE\x00" as *const u8 as *const libc::c_char,
      selector: 0x89f0i32 as libc::c_ushort,
      ifr_offset: 16u64 as libc::c_ushort,
    };
    init
  },
  {
    let mut init = arg1opt {
      name: b"SOUTFILL\x00" as *const u8 as *const libc::c_char,
      selector: (0x89f0i32 + 2i32) as libc::c_ushort,
      ifr_offset: 16u64 as libc::c_ushort,
    };
    init
  },
  {
    let mut init = arg1opt {
      name: b"SIFMAP\x00" as *const u8 as *const libc::c_char,
      selector: 0x8971i32 as libc::c_ushort,
      ifr_offset: 16u64 as libc::c_ushort,
    };
    init
  },
  {
    let mut init = arg1opt {
      name: b"SIFMAP\x00" as *const u8 as *const libc::c_char,
      selector: 0x8971i32 as libc::c_ushort,
      ifr_offset: 32u64 as libc::c_ushort,
    };
    init
  },
  {
    let mut init = arg1opt {
      name: b"SIFMAP\x00" as *const u8 as *const libc::c_char,
      selector: 0x8971i32 as libc::c_ushort,
      ifr_offset: 34u64 as libc::c_ushort,
    };
    init
  },
  {
    let mut init = arg1opt {
      name: b"SIFADDR\x00" as *const u8 as *const libc::c_char,
      selector: 0x8916i32 as libc::c_ushort,
      ifr_offset: 16u64 as libc::c_ushort,
    };
    init
  },
  {
    let mut init = arg1opt {
      name: b"DIFADDR\x00" as *const u8 as *const libc::c_char,
      selector: 0x8936i32 as libc::c_ushort,
      ifr_offset: 16u64 as libc::c_ushort,
    };
    init
  },
  {
    let mut init = arg1opt {
      name: b"SIFADDR\x00" as *const u8 as *const libc::c_char,
      selector: 0x8916i32 as libc::c_ushort,
      ifr_offset: 16u64 as libc::c_ushort,
    };
    init
  },
];
// Initialized in run_static_initializers
static mut OptArray: [options; 24] = [options {
  name: 0 as *const libc::c_char,
  flags_arg_flags: [0; 2],
  selector: 0,
}; 24];
#[no_mangle]
pub unsafe extern "C" fn ifconfig_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut ifr: ifreq = ifreq {
    ifr_ifrn: C2RustUnnamed_3 { ifrn_name: [0; 16] },
    ifr_ifru: C2RustUnnamed_2 {
      ifru_addr: sockaddr {
        sa_family: 0,
        sa_data: [0; 14],
      },
    },
  }; /* socket fd we use to manipulate stuff with */
  let mut sai: sockaddr_in = sockaddr_in {
    sin_family: 0,
    sin_port: 0,
    sin_addr: in_addr { s_addr: 0 },
    sin_zero: [0; 8],
  };
  let mut sa: sockaddr = sockaddr {
    sa_family: 0,
    sa_data: [0; 14],
  };
  let mut a1op: *const arg1opt = 0 as *const arg1opt;
  let mut op: *const options = 0 as *const options;
  let mut sockfd: libc::c_int = 0;
  let mut selector: libc::c_int = 0;
  let mut mask: libc::c_uint = 0;
  let mut did_flags: libc::c_uint = 0;
  let mut sai_hostname: libc::c_uint = 0;
  let mut sai_netmask: libc::c_uint = 0;
  let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
  /*char host[128];*/
  let mut host: *const libc::c_char = 0 as *const libc::c_char; /* make gcc happy */
  let mut show_all_param: *mut libc::c_char = 0 as *mut libc::c_char;
  did_flags = 0i32 as libc::c_uint;
  sai_hostname = 0i32 as libc::c_uint;
  sai_netmask = 0i32 as libc::c_uint;
  /* skip argv[0] */
  argv = argv.offset(1);
  show_all_param = 0 as *mut libc::c_char;
  if !(*argv.offset(0)).is_null()
    && *(*argv.offset(0)).offset(0) as libc::c_int == '-' as i32
    && *(*argv.offset(0)).offset(1) as libc::c_int == 'a' as i32
    && *(*argv.offset(0)).offset(2) == 0
  {
    argv = argv.offset(1);
    show_all_param = 1i32 as intptr_t as *mut libc::c_char
  }
  if (*argv.offset(0)).is_null() || (*argv.offset(1)).is_null() {
    /* one or no args */
    return display_interfaces(if !(*argv.offset(0)).is_null() {
      *argv.offset(0)
    } else {
      show_all_param
    });
  }
  /* Create a channel to the NET kernel. */
  sockfd = xsocket(2i32, SOCK_DGRAM as libc::c_int, 0i32);
  /* get interface name */
  strncpy_IFNAMSIZ(ifr.ifr_ifrn.ifrn_name.as_mut_ptr(), *argv);
  let mut current_block_97: u64;
  loop
  /* Process the remaining arguments. */
  {
    argv = argv.offset(1); /* while () */
    if (*argv).is_null() {
      break;
    }
    p = *argv;
    mask = (0x1i32 | 0x4i32 | 0x10i32) as libc::c_uint;
    if *p as libc::c_int == '-' as i32 {
      /* If the arg starts with '-'... */
      p = p.offset(1);
      mask = (0x2i32 | 0x8i32 | 0x20i32) as libc::c_uint /*    advance past it and */
      /*    set the appropriate mask. */
    }
    op = OptArray.as_ptr();
    loop {
      if (*op).name.is_null() {
        current_block_97 = 652864300344834934;
        break;
      }
      /* Find table entry. */
      if strcmp(p, (*op).name) == 0i32 {
        /* If name matches... */
        mask &= (*op).flags();
        if mask != 0 {
          /* set the mask and go. */
          current_block_97 = 4949694742072023634;
          break;
        } else {
          /* If we get here, there was a valid arg with an */
          /* invalid '-' prefix. */
          bb_error_msg_and_die(
            b"bad: \'%s\'\x00" as *const u8 as *const libc::c_char,
            p.offset(-1),
          );
        }
      } else {
        op = op.offset(1)
      }
    }
    match current_block_97 {
      652864300344834934 => {
        /* We fell through, so treat as possible hostname. */
        a1op = Arg1Opt
          .as_ptr()
          .offset(
            (::std::mem::size_of::<[arg1opt; 16]>() as libc::c_ulong)
              .wrapping_div(::std::mem::size_of::<arg1opt>() as libc::c_ulong)
              as libc::c_uint as isize,
          )
          .offset(-1); /* if (mask & ARG_MASK) */
        mask = (*op).arg_flags();
        current_block_97 = 15570335776229543684;
      }
      _ => {
        if mask & (0x20i32 | 0x10i32) as libc::c_uint != 0 {
          mask = (*op).arg_flags();
          if mask & 0x20i32 as libc::c_uint & did_flags != 0 {
            bb_show_usage();
          }
          a1op = Arg1Opt
            .as_ptr()
            .offset(op.wrapping_offset_from(OptArray.as_ptr()) as libc::c_long as isize);
          argv = argv.offset(1);
          if (*argv).is_null() {
            if mask & 0x10i32 as libc::c_uint != 0 {
              bb_show_usage();
            }
            argv = argv.offset(-1);
            mask &= 0x40i32 as libc::c_uint;
            current_block_97 = 1677945370889843322;
          /* just for broadcast */
          } else {
            current_block_97 = 15570335776229543684; /* A_CAST_HOST_COPY_IN_ETHER */
          }
        } else {
          current_block_97 = 18002345992382212654; /* compat stuff */
        }
      }
    }
    match current_block_97 {
      15570335776229543684 => {
        did_flags |= mask & (0x20i32 | 0x100i32) as libc::c_uint;
        if mask & 0x2i32 as libc::c_uint != 0 {
          if mask & 0x1i32 as libc::c_uint != 0 {
            host = *argv;
            if strcmp(host, b"inet\x00" as *const u8 as *const libc::c_char) == 0i32 {
              continue;
            }
            sai.sin_family = 2i32 as sa_family_t;
            sai.sin_port = 0i32 as in_port_t;
            if strcmp(host, b"default\x00" as *const u8 as *const libc::c_char) == 0i32 {
              /* Default is special, meaning 0.0.0.0. */
              sai.sin_addr.s_addr = 0i32 as in_addr_t
            } else if *host.offset(0) as libc::c_int == '+' as i32
              && *host.offset(1) == 0
              && mask & 0x200i32 as libc::c_uint != 0
              && did_flags & (0x20i32 | 0x100i32) as libc::c_uint
                == (0x20i32 | 0x100i32) as libc::c_uint
            {
              /* + is special, meaning broadcast is derived. */
              sai.sin_addr.s_addr = !sai_netmask | sai_hostname & sai_netmask
            } else {
              let mut lsa: *mut len_and_sockaddr = 0 as *mut len_and_sockaddr;
              let mut prefix: *mut libc::c_char = 0 as *mut libc::c_char;
              let mut prefix_len: libc::c_int = 0i32;
              prefix = strchr(host, '/' as i32);
              if !prefix.is_null() {
                prefix_len = xatou_range(
                  prefix.offset(1),
                  0i32 as libc::c_uint,
                  128i32 as libc::c_uint,
                ) as libc::c_int;
                *prefix = '\u{0}' as i32 as libc::c_char
              }
              loop {
                lsa = xhost2sockaddr(host, 0i32);
                if !((*lsa).u.sa.sa_family as libc::c_int != 10i32 && !prefix.is_null()) {
                  break;
                }
                /* TODO: we do not support "ifconfig eth0 up 1.2.3.4/17".
                 * For now, just make it fail instead of silently ignoring "/17" part:
                 */
                *prefix = '/' as i32 as libc::c_char
              }
              if (*lsa).u.sa.sa_family as libc::c_int == 10i32 {
                let mut sockfd6: libc::c_int = 0;
                let mut ifr6: in6_ifreq = in6_ifreq {
                  ifr6_addr: in6_addr {
                    __in6_u: C2RustUnnamed {
                      __u6_addr8: [0; 16],
                    },
                  },
                  ifr6_prefixlen: 0,
                  ifr6_ifindex: 0,
                };
                sockfd6 = xsocket(10i32, SOCK_DGRAM as libc::c_int, 0i32);
                bb_xioctl(
                  sockfd6,
                  0x8933i32 as libc::c_uint,
                  &mut ifr as *mut ifreq as *mut libc::c_void,
                  b"SIOCGIFINDEX\x00" as *const u8 as *const libc::c_char,
                );
                ifr6.ifr6_ifindex = ifr.ifr_ifru.ifru_ivalue;
                ifr6.ifr6_prefixlen = prefix_len as uint32_t;
                memcpy(
                  &mut ifr6.ifr6_addr as *mut in6_addr as *mut libc::c_void,
                  &mut (*lsa).u.sin6.sin6_addr as *mut in6_addr as *const libc::c_void,
                  ::std::mem::size_of::<in6_addr>() as libc::c_ulong,
                );
                ioctl_or_perror_and_die(
                  sockfd6,
                  (*a1op).selector as libc::c_uint,
                  &mut ifr6 as *mut in6_ifreq as *mut libc::c_void,
                  b"SIOC%s\x00" as *const u8 as *const libc::c_char,
                  (*a1op).name,
                );
                continue;
              } else {
                sai.sin_addr = (*lsa).u.sin.sin_addr
              }
            }
            if mask & 0x100i32 as libc::c_uint != 0 {
              sai_hostname = sai.sin_addr.s_addr
            }
            if mask & 0x20i32 as libc::c_uint != 0 {
              sai_netmask = sai.sin_addr.s_addr
            }
            p = &mut sai as *mut sockaddr_in as *mut libc::c_char
          } else {
            /* This is the "hw" arg case. */
            let mut hw_class: smalluint = (index_in_substrings(
              b"ether\x00infiniband\x00\x00" as *const u8 as *const libc::c_char,
              *argv,
            ) + 1i32) as smalluint;
            if hw_class == 0 || {
              argv = argv.offset(1);
              (*argv).is_null()
            } {
              bb_show_usage();
            }
            host = *argv;
            if if hw_class as libc::c_int == 1i32 {
              in_ether(host, &mut sa)
            } else {
              in_ib(host, &mut sa)
            } != 0
            {
              bb_error_msg_and_die(
                b"invalid hw-addr %s\x00" as *const u8 as *const libc::c_char,
                host,
              );
            }
            p = &mut sa as *mut sockaddr as *mut libc::c_char
          }
          memcpy(
            (&mut ifr as *mut ifreq as *mut libc::c_char)
              .offset((*a1op).ifr_offset as libc::c_int as isize) as *mut libc::c_void,
            p as *const libc::c_void,
            ::std::mem::size_of::<sockaddr>() as libc::c_ulong,
          );
        } else {
          /* FIXME: error check?? */
          let mut i: libc::c_ulong = strtoul(*argv, 0 as *mut *mut libc::c_char, 0i32);
          p = (&mut ifr as *mut ifreq as *mut libc::c_char)
            .offset((*a1op).ifr_offset as libc::c_int as isize);
          if mask & 0xci32 as libc::c_uint != 0 {
            bb_xioctl(
              sockfd,
              0x8970i32 as libc::c_uint,
              &mut ifr as *mut ifreq as *mut libc::c_void,
              b"SIOCGIFMAP\x00" as *const u8 as *const libc::c_char,
            );
            if mask & 0xci32 as libc::c_uint == 0xci32 as libc::c_uint {
              *(p as *mut libc::c_uchar) = i as libc::c_uchar
            } else if mask & 0x8i32 as libc::c_uint != 0 {
              *(p as *mut libc::c_ushort) = i as libc::c_ushort
            } else {
              *(p as *mut libc::c_ulong) = i
            }
          } else if mask & 0x1i32 as libc::c_uint != 0 {
            let ref mut fresh0 = *(p as *mut caddr_t);
            *fresh0 = i as caddr_t
          } else {
            /* A_CAST_INT */
            *(p as *mut libc::c_int) = i as libc::c_int
          }
        }
        ioctl_or_perror_and_die(
          sockfd,
          (*a1op).selector as libc::c_uint,
          &mut ifr as *mut ifreq as *mut libc::c_void,
          b"SIOC%s\x00" as *const u8 as *const libc::c_char,
          (*a1op).name,
        );
        if mask & 0x80i32 as libc::c_uint != 0 {
          /*
           * Don't do the set_flag() if the address is an alias with
           * a '-' at the end, since it's deleted already! - Roman
           *
           * Should really use regex.h here, not sure though how well
           * it'll go with the cross-platform support etc.
           */
          let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
          let mut found_colon: libc::c_short = 0i32 as libc::c_short;
          ptr = ifr.ifr_ifrn.ifrn_name.as_mut_ptr();
          while *ptr != 0 {
            if *ptr as libc::c_int == ':' as i32 {
              found_colon += 1
            }
            ptr = ptr.offset(1)
          }
          if found_colon as libc::c_int != 0
            && *ptr.offset(-1i32 as isize) as libc::c_int == '-' as i32
          {
            continue;
          }
          current_block_97 = 1677945370889843322;
        } else {
          current_block_97 = 1677945370889843322;
        }
      }
      _ => {}
    }
    match current_block_97 {
      1677945370889843322 => {
        if mask & 0x40i32 as libc::c_uint == 0 {
          continue;
        }
        mask = 0x4i32 as libc::c_uint
      }
      _ => {}
    }
    bb_xioctl(
      sockfd,
      0x8913i32 as libc::c_uint,
      &mut ifr as *mut ifreq as *mut libc::c_void,
      b"SIOCGIFFLAGS\x00" as *const u8 as *const libc::c_char,
    );
    selector = (*op).selector as libc::c_int;
    if mask & (0x4i32 | 0x8i32) as libc::c_uint != 0 {
      ifr.ifr_ifru.ifru_flags = (ifr.ifr_ifru.ifru_flags as libc::c_int | selector) as libc::c_short
    } else {
      ifr.ifr_ifru.ifru_flags =
        (ifr.ifr_ifru.ifru_flags as libc::c_int & !selector) as libc::c_short
    }
    bb_xioctl(
      sockfd,
      0x8914i32 as libc::c_uint,
      &mut ifr as *mut ifreq as *mut libc::c_void,
      b"SIOCSIFFLAGS\x00" as *const u8 as *const libc::c_char,
    );
  }
  return 0i32;
}
unsafe extern "C" fn run_static_initializers() {
  OptArray = [
    {
      let mut init = options {
        flags_arg_flags: [0; 2],
        name: b"metric\x00" as *const u8 as *const libc::c_char,
        selector: 0i32 as libc::c_ushort,
      };
      init.set_flags(0x10i32 as libc::c_uint);
      init.set_arg_flags(0x10i32 as libc::c_uint);
      init
    },
    {
      let mut init = options {
        flags_arg_flags: [0; 2],
        name: b"mtu\x00" as *const u8 as *const libc::c_char,
        selector: 0i32 as libc::c_ushort,
      };
      init.set_flags(0x10i32 as libc::c_uint);
      init.set_arg_flags(0x10i32 as libc::c_uint);
      init
    },
    {
      let mut init = options {
        flags_arg_flags: [0; 2],
        name: b"txqueuelen\x00" as *const u8 as *const libc::c_char,
        selector: 0i32 as libc::c_ushort,
      };
      init.set_flags(0x10i32 as libc::c_uint);
      init.set_arg_flags(0x10i32 as libc::c_uint);
      init
    },
    {
      let mut init = options {
        flags_arg_flags: [0; 2],
        name: b"dstaddr\x00" as *const u8 as *const libc::c_char,
        selector: 0i32 as libc::c_ushort,
      };
      init.set_flags(0x10i32 as libc::c_uint);
      init.set_arg_flags((0x10i32 | (0x2i32 | 0x1i32)) as libc::c_uint);
      init
    },
    {
      let mut init = options {
        flags_arg_flags: [0; 2],
        name: b"netmask\x00" as *const u8 as *const libc::c_char,
        selector: 0i32 as libc::c_ushort,
      };
      init.set_flags(0x10i32 as libc::c_uint);
      init.set_arg_flags((0x10i32 | (0x2i32 | 0x1i32) | 0x20i32) as libc::c_uint);
      init
    },
    {
      let mut init = options {
        flags_arg_flags: [0; 2],
        name: b"broadcast\x00" as *const u8 as *const libc::c_char,
        selector: IFF_BROADCAST as libc::c_int as libc::c_ushort,
      };
      init.set_flags((0x10i32 | 0x2i32) as libc::c_uint);
      init.set_arg_flags((0x10i32 | (0x2i32 | 0x1i32) | 0x40i32 | 0x200i32) as libc::c_uint);
      init
    },
    {
      let mut init = options {
        flags_arg_flags: [0; 2],
        name: b"hw\x00" as *const u8 as *const libc::c_char,
        selector: 0i32 as libc::c_ushort,
      };
      init.set_flags(0x10i32 as libc::c_uint);
      init.set_arg_flags((0x10i32 | 0x2i32) as libc::c_uint);
      init
    },
    {
      let mut init = options {
        flags_arg_flags: [0; 2],
        name: b"pointopoint\x00" as *const u8 as *const libc::c_char,
        selector: IFF_POINTOPOINT as libc::c_int as libc::c_ushort,
      };
      init.set_flags((0x10i32 | 0x2i32) as libc::c_uint);
      init.set_arg_flags((0x10i32 | (0x2i32 | 0x1i32) | 0x40i32) as libc::c_uint);
      init
    },
    {
      let mut init = options {
        flags_arg_flags: [0; 2],
        name: b"keepalive\x00" as *const u8 as *const libc::c_char,
        selector: 0i32 as libc::c_ushort,
      };
      init.set_flags(0x10i32 as libc::c_uint);
      init.set_arg_flags((0x10i32 | 0x1i32) as libc::c_uint);
      init
    },
    {
      let mut init = options {
        flags_arg_flags: [0; 2],
        name: b"outfill\x00" as *const u8 as *const libc::c_char,
        selector: 0i32 as libc::c_ushort,
      };
      init.set_flags(0x10i32 as libc::c_uint);
      init.set_arg_flags((0x10i32 | 0x1i32) as libc::c_uint);
      init
    },
    {
      let mut init = options {
        flags_arg_flags: [0; 2],
        name: b"mem_start\x00" as *const u8 as *const libc::c_char,
        selector: 0i32 as libc::c_ushort,
      };
      init.set_flags(0x10i32 as libc::c_uint);
      init.set_arg_flags((0x10i32 | 0x4i32) as libc::c_uint);
      init
    },
    {
      let mut init = options {
        flags_arg_flags: [0; 2],
        name: b"io_addr\x00" as *const u8 as *const libc::c_char,
        selector: 0i32 as libc::c_ushort,
      };
      init.set_flags(0x10i32 as libc::c_uint);
      init.set_arg_flags((0x10i32 | 0x4i32) as libc::c_uint);
      init
    },
    {
      let mut init = options {
        flags_arg_flags: [0; 2],
        name: b"irq\x00" as *const u8 as *const libc::c_char,
        selector: 0i32 as libc::c_ushort,
      };
      init.set_flags(0x10i32 as libc::c_uint);
      init.set_arg_flags((0x10i32 | 0xci32) as libc::c_uint);
      init
    },
    {
      let mut init = options {
        flags_arg_flags: [0; 2],
        name: b"add\x00" as *const u8 as *const libc::c_char,
        selector: 0i32 as libc::c_ushort,
      };
      init.set_flags(0x10i32 as libc::c_uint);
      init.set_arg_flags((0x2i32 | 0x1i32 | 0x40i32) as libc::c_uint);
      init
    },
    {
      let mut init = options {
        flags_arg_flags: [0; 2],
        name: b"del\x00" as *const u8 as *const libc::c_char,
        selector: 0i32 as libc::c_ushort,
      };
      init.set_flags(0x10i32 as libc::c_uint);
      init.set_arg_flags((0x2i32 | 0x1i32 | 0x40i32) as libc::c_uint);
      init
    },
    {
      let mut init = options {
        flags_arg_flags: [0; 2],
        name: b"arp\x00" as *const u8 as *const libc::c_char,
        selector: IFF_NOARP as libc::c_int as libc::c_ushort,
      };
      init.set_flags((0x1i32 | 0x8i32) as libc::c_uint);
      init.set_arg_flags(0i32 as libc::c_uint);
      init
    },
    {
      let mut init = options {
        flags_arg_flags: [0; 2],
        name: b"trailers\x00" as *const u8 as *const libc::c_char,
        selector: IFF_NOTRAILERS as libc::c_int as libc::c_ushort,
      };
      init.set_flags((0x1i32 | 0x8i32) as libc::c_uint);
      init.set_arg_flags(0i32 as libc::c_uint);
      init
    },
    {
      let mut init = options {
        flags_arg_flags: [0; 2],
        name: b"promisc\x00" as *const u8 as *const libc::c_char,
        selector: IFF_PROMISC as libc::c_int as libc::c_ushort,
      };
      init.set_flags((0x4i32 | 0x2i32) as libc::c_uint);
      init.set_arg_flags(0i32 as libc::c_uint);
      init
    },
    {
      let mut init = options {
        flags_arg_flags: [0; 2],
        name: b"multicast\x00" as *const u8 as *const libc::c_char,
        selector: IFF_MULTICAST as libc::c_int as libc::c_ushort,
      };
      init.set_flags((0x4i32 | 0x2i32) as libc::c_uint);
      init.set_arg_flags(0i32 as libc::c_uint);
      init
    },
    {
      let mut init = options {
        flags_arg_flags: [0; 2],
        name: b"allmulti\x00" as *const u8 as *const libc::c_char,
        selector: IFF_ALLMULTI as libc::c_int as libc::c_ushort,
      };
      init.set_flags((0x4i32 | 0x2i32) as libc::c_uint);
      init.set_arg_flags(0i32 as libc::c_uint);
      init
    },
    {
      let mut init = options {
        flags_arg_flags: [0; 2],
        name: b"dynamic\x00" as *const u8 as *const libc::c_char,
        selector: IFF_DYNAMIC as libc::c_int as libc::c_ushort,
      };
      init.set_flags((0x4i32 | 0x2i32) as libc::c_uint);
      init.set_arg_flags(0i32 as libc::c_uint);
      init
    },
    {
      let mut init = options {
        flags_arg_flags: [0; 2],
        name: b"up\x00" as *const u8 as *const libc::c_char,
        selector: (IFF_UP as libc::c_int | IFF_RUNNING as libc::c_int) as libc::c_ushort,
      };
      init.set_flags(0x4i32 as libc::c_uint);
      init.set_arg_flags(0i32 as libc::c_uint);
      init
    },
    {
      let mut init = options {
        flags_arg_flags: [0; 2],
        name: b"down\x00" as *const u8 as *const libc::c_char,
        selector: IFF_UP as libc::c_int as libc::c_ushort,
      };
      init.set_flags(0x1i32 as libc::c_uint);
      init.set_arg_flags(0i32 as libc::c_uint);
      init
    },
    {
      let mut init = options {
        flags_arg_flags: [0; 2],
        name: 0 as *const libc::c_char,
        selector: (IFF_UP as libc::c_int | IFF_RUNNING as libc::c_int) as libc::c_ushort,
      };
      init.set_flags(0i32 as libc::c_uint);
      init.set_arg_flags((0x2i32 | 0x1i32 | 0x40i32 | 0x80i32 | 0x100i32) as libc::c_uint);
      init
    },
  ]
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
