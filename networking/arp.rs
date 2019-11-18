use crate::libbb::ptr_to_globals::bb_errno;

use libc;
use libc::ioctl;
use libc::printf;
use libc::sscanf;
use libc::strcmp;
extern "C" {
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  fn fgets_unlocked(
    __s: *mut libc::c_char,
    __n: libc::c_int,
    __stream: *mut FILE,
  ) -> *mut libc::c_char;

  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xmove_fd(_: libc::c_int, _: libc::c_int);
  #[no_mangle]
  fn xsocket(domain: libc::c_int, type_0: libc::c_int, protocol: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn strncpy_IFNAMSIZ(dst: *mut libc::c_char, src: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xfopen_for_read(path: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  static mut option_mask32: u32;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_simple_error_msg(s: *const libc::c_char);
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_simple_herror_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn get_aftype(name: *const libc::c_char) -> *const aftype;
  #[no_mangle]
  fn get_hwtype(name: *const libc::c_char) -> *const hwtype;
  #[no_mangle]
  fn get_hwntype(type_0: libc::c_int) -> *const hwtype;
  #[no_mangle]
  fn index_in_strings(strings: *const libc::c_char, key: *const libc::c_char) -> libc::c_int;
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
  fn print_flags_separated(
    masks: *const libc::c_int,
    labels: *const libc::c_char,
    flags: libc::c_int,
    separator: *const libc::c_char,
  ) -> libc::c_int;
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
}

pub type __caddr_t = *mut libc::c_char;

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
use libc::FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aftype {
  pub name: *const libc::c_char,
  pub title: *const libc::c_char,
  pub af: libc::c_int,
  pub alen: libc::c_int,
  pub print: Option<unsafe extern "C" fn(_: *mut libc::c_uchar) -> *mut libc::c_char>,
  pub sprint: Option<unsafe extern "C" fn(_: *mut sockaddr, _: libc::c_int) -> *const libc::c_char>,
  pub input: Option<unsafe extern "C" fn(_: *const libc::c_char, _: *mut sockaddr) -> libc::c_int>,
  pub herror: Option<unsafe extern "C" fn(_: *mut libc::c_char) -> ()>,
  pub rprint: Option<unsafe extern "C" fn(_: libc::c_int) -> libc::c_int>,
  pub rinput: Option<
    unsafe extern "C" fn(_: libc::c_int, _: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
  >,
  pub getmask: Option<
    unsafe extern "C" fn(
      _: *mut libc::c_char,
      _: *mut sockaddr,
      _: *mut libc::c_char,
    ) -> libc::c_int,
  >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hwtype {
  pub name: *const libc::c_char,
  pub title: *const libc::c_char,
  pub type_0: libc::c_int,
  pub alen: libc::c_int,
  pub print: Option<unsafe extern "C" fn(_: *mut libc::c_uchar) -> *mut libc::c_char>,
  pub input: Option<unsafe extern "C" fn(_: *const libc::c_char, _: *mut sockaddr) -> libc::c_int>,
  pub activate: Option<unsafe extern "C" fn(_: libc::c_int) -> libc::c_int>,
  pub suppress_null_addr: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub ap: *const aftype,
  pub hw: *const hwtype,
  pub device: *const libc::c_char,
  pub hw_set: smallint,
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
pub struct ifreq {
  pub ifr_ifrn: C2RustUnnamed_0,
  pub ifr_ifru: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
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
pub union C2RustUnnamed_0 {
  pub ifrn_name: [libc::c_char; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arpreq {
  pub arp_pa: sockaddr,
  pub arp_ha: sockaddr,
  pub arp_flags: libc::c_int,
  pub arp_netmask: sockaddr,
  pub arp_dev: [libc::c_char; 16],
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub const ARP_OPT_v: C2RustUnnamed_1 = 0;
pub const ARP_OPT_s: C2RustUnnamed_1 = 512;
pub const ARP_OPT_D: C2RustUnnamed_1 = 256;
pub const ARP_OPT_n: C2RustUnnamed_1 = 128;
pub const ARP_OPT_d: C2RustUnnamed_1 = 64;
pub const ARP_OPT_a: C2RustUnnamed_1 = 32;
pub const ARP_OPT_i: C2RustUnnamed_1 = 16;
pub const ARP_OPT_t: C2RustUnnamed_1 = 8;
pub const ARP_OPT_H: C2RustUnnamed_1 = 4;
pub const ARP_OPT_p: C2RustUnnamed_1 = 2;
pub const ARP_OPT_A: C2RustUnnamed_1 = 1;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const sockfd: C2RustUnnamed_2 = 3;
/* flag if hw-type was set (-H) */
static mut options: [libc::c_char; 46] = [
  112, 117, 98, 0, 112, 114, 105, 118, 0, 116, 101, 109, 112, 0, 116, 114, 97, 105, 108, 0, 100,
  111, 110, 116, 112, 117, 98, 0, 97, 117, 116, 111, 0, 100, 101, 118, 0, 110, 101, 116, 109, 97,
  115, 107, 0, 0,
];
/* Delete an entry from the ARP cache. */
/* Called only from main, once */
unsafe extern "C" fn arp_del(mut args: *mut *mut libc::c_char) -> libc::c_int {
  let mut current_block: u64;
  let mut host: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut req: arpreq = arpreq {
    arp_pa: sockaddr {
      sa_family: 0,
      sa_data: [0; 14],
    },
    arp_ha: sockaddr {
      sa_family: 0,
      sa_data: [0; 14],
    },
    arp_flags: 0,
    arp_netmask: sockaddr {
      sa_family: 0,
      sa_data: [0; 14],
    },
    arp_dev: [0; 16],
  };
  let mut sa: sockaddr = sockaddr {
    sa_family: 0,
    sa_data: [0; 14],
  };
  let mut flags: libc::c_int = 0i32;
  let mut err: libc::c_int = 0;
  memset(
    &mut req as *mut arpreq as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<arpreq>() as libc::c_ulong,
  );
  /* Resolve the host name. */
  host = *args;
  if (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).ap)
    .input
    .expect("non-null function pointer")(host, &mut sa)
    < 0i32
  {
    bb_simple_herror_msg_and_die(host);
  }
  /* If a host has more than one address, use the correct one! */
  memcpy(
    &mut req.arp_pa as *mut sockaddr as *mut libc::c_void,
    &mut sa as *mut sockaddr as *const libc::c_void,
    ::std::mem::size_of::<sockaddr>() as libc::c_ulong,
  );
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hw_set != 0 {
    req.arp_ha.sa_family =
      (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hw).type_0 as sa_family_t
  }
  req.arp_flags = 0x4i32;
  args = args.offset(1);
  while !(*args).is_null() {
    match index_in_strings(options.as_ptr(), *args) {
      0 => {
        /* "pub" */
        flags |= 1i32;
        args = args.offset(1)
      }
      1 => {
        /* "priv" */
        flags |= 2i32;
        args = args.offset(1)
      }
      2 => {
        /* "temp" */
        req.arp_flags &= !0x4i32;
        args = args.offset(1)
      }
      3 => {
        /* "trail" */
        req.arp_flags |= 0x10i32;
        args = args.offset(1)
      }
      4 => {
        /* "dontpub" */
        bb_simple_error_msg(
          b"feature ATF_DONTPUB is not supported\x00" as *const u8 as *const libc::c_char,
        );
        args = args.offset(1)
      }
      5 => {
        /* "auto" */
        bb_simple_error_msg(
          b"feature ATF_MAGIC is not supported\x00" as *const u8 as *const libc::c_char,
        );
        args = args.offset(1)
      }
      6 => {
        /* "dev" */
        args = args.offset(1);
        if (*args).is_null() {
          bb_show_usage();
        }
        let ref mut fresh0 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).device;
        *fresh0 = *args;
        args = args.offset(1)
      }
      7 => {
        /* "netmask" */
        args = args.offset(1);
        if (*args).is_null() {
          bb_show_usage();
        }
        if strcmp(
          *args,
          b"255.255.255.255\x00" as *const u8 as *const libc::c_char,
        ) != 0i32
        {
          host = *args;
          if (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).ap)
            .input
            .expect("non-null function pointer")(host, &mut sa)
            < 0i32
          {
            bb_simple_herror_msg_and_die(host);
          }
          memcpy(
            &mut req.arp_netmask as *mut sockaddr as *mut libc::c_void,
            &mut sa as *mut sockaddr as *const libc::c_void,
            ::std::mem::size_of::<sockaddr>() as libc::c_ulong,
          );
          req.arp_flags |= 0x20i32
        }
        args = args.offset(1)
      }
      _ => {
        bb_show_usage();
      }
    }
  }
  if flags == 0i32 {
    flags = 3i32
  }
  strncpy_IFNAMSIZ(
    req.arp_dev.as_mut_ptr(),
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).device,
  );
  err = -1i32;
  /* Call the kernel. */
  if flags & 2i32 != 0 {
    if option_mask32 & ARP_OPT_v as libc::c_int as libc::c_uint != 0 {
      bb_simple_error_msg(b"SIOCDARP(nopub)\x00" as *const u8 as *const libc::c_char);
    }
    err = ioctl(
      sockfd as libc::c_int,
      0x8953i32 as libc::c_ulong,
      &mut req as *mut arpreq,
    );
    if err < 0i32 {
      if *bb_errno == 6i32 {
        if !(flags & 1i32 != 0) {
          printf(
            b"No ARP entry for %s\n\x00" as *const u8 as *const libc::c_char,
            host,
          );
          return -1i32;
        }
      } else {
        bb_simple_perror_msg_and_die(b"SIOCDARP(priv)\x00" as *const u8 as *const libc::c_char);
      }
      current_block = 4254989650113872746;
    } else {
      current_block = 5159818223158340697;
    }
  } else {
    current_block = 5159818223158340697;
  }
  match current_block {
    5159818223158340697 => {
      if flags & 1i32 != 0 && err != 0 {
        current_block = 4254989650113872746;
      } else {
        current_block = 10067844863897285902;
      }
    }
    _ => {}
  }
  match current_block {
    4254989650113872746 => {
      req.arp_flags |= 0x8i32;
      if option_mask32 & ARP_OPT_v as libc::c_int as libc::c_uint != 0 {
        bb_simple_error_msg(b"SIOCDARP(pub)\x00" as *const u8 as *const libc::c_char);
      }
      if ioctl(
        sockfd as libc::c_int,
        0x8953i32 as libc::c_ulong,
        &mut req as *mut arpreq,
      ) < 0i32
      {
        if *bb_errno == 6i32 {
          printf(
            b"No ARP entry for %s\n\x00" as *const u8 as *const libc::c_char,
            host,
          );
          return -1i32;
        }
        bb_simple_perror_msg_and_die(b"SIOCDARP(pub)\x00" as *const u8 as *const libc::c_char);
      }
    }
    _ => {}
  }
  return 0i32;
}
/* Get the hardware address to a specified interface name */
unsafe extern "C" fn arp_getdevhw(mut ifname: *mut libc::c_char, mut sa: *mut sockaddr) {
  let mut ifr: ifreq = ifreq {
    ifr_ifrn: C2RustUnnamed_0 { ifrn_name: [0; 16] },
    ifr_ifru: C2RustUnnamed {
      ifru_addr: sockaddr {
        sa_family: 0,
        sa_data: [0; 14],
      },
    },
  };
  let mut xhw: *const hwtype = 0 as *const hwtype;
  strncpy_IFNAMSIZ(ifr.ifr_ifrn.ifrn_name.as_mut_ptr(), ifname);
  ioctl_or_perror_and_die(
    sockfd as libc::c_int,
    0x8927i32 as libc::c_uint,
    &mut ifr as *mut ifreq as *mut libc::c_void,
    b"can\'t get HW-Address for \'%s\'\x00" as *const u8 as *const libc::c_char,
    ifname,
  );
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hw_set as libc::c_int != 0
    && ifr.ifr_ifru.ifru_hwaddr.sa_family as libc::c_int
      != (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hw).type_0
  {
    bb_simple_error_msg_and_die(b"protocol type mismatch\x00" as *const u8 as *const libc::c_char);
  }
  memcpy(
    sa as *mut libc::c_void,
    &mut ifr.ifr_ifru.ifru_hwaddr as *mut sockaddr as *const libc::c_void,
    ::std::mem::size_of::<sockaddr>() as libc::c_ulong,
  );
  if option_mask32 & ARP_OPT_v as libc::c_int as libc::c_uint != 0 {
    xhw = get_hwntype(ifr.ifr_ifru.ifru_hwaddr.sa_family as libc::c_int);
    if xhw.is_null() || (*xhw).print.is_none() {
      xhw = get_hwntype(-1i32)
    }
    bb_error_msg(
      b"device \'%s\' has HW address %s \'%s\'\x00" as *const u8 as *const libc::c_char,
      ifname,
      (*xhw).name,
      (*xhw).print.expect("non-null function pointer")(
        &mut ifr.ifr_ifru.ifru_hwaddr.sa_data as *mut [libc::c_char; 14] as *mut libc::c_uchar,
      ),
    );
  };
}
/* Set an entry in the ARP cache. */
/* Called only from main, once */
unsafe extern "C" fn arp_set(mut args: *mut *mut libc::c_char) -> libc::c_int {
  let mut host: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut req: arpreq = arpreq {
    arp_pa: sockaddr {
      sa_family: 0,
      sa_data: [0; 14],
    },
    arp_ha: sockaddr {
      sa_family: 0,
      sa_data: [0; 14],
    },
    arp_flags: 0,
    arp_netmask: sockaddr {
      sa_family: 0,
      sa_data: [0; 14],
    },
    arp_dev: [0; 16],
  };
  let mut sa: sockaddr = sockaddr {
    sa_family: 0,
    sa_data: [0; 14],
  };
  let mut flags: libc::c_int = 0;
  memset(
    &mut req as *mut arpreq as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<arpreq>() as libc::c_ulong,
  );
  let fresh1 = args;
  args = args.offset(1);
  host = *fresh1;
  if (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).ap)
    .input
    .expect("non-null function pointer")(host, &mut sa)
    < 0i32
  {
    bb_simple_herror_msg_and_die(host);
  }
  /* If a host has more than one address, use the correct one! */
  memcpy(
    &mut req.arp_pa as *mut sockaddr as *mut libc::c_void,
    &mut sa as *mut sockaddr as *const libc::c_void,
    ::std::mem::size_of::<sockaddr>() as libc::c_ulong,
  );
  /* Fetch the hardware address. */
  if (*args).is_null() {
    bb_simple_error_msg_and_die(b"need hardware address\x00" as *const u8 as *const libc::c_char);
  }
  if option_mask32 & ARP_OPT_D as libc::c_int as libc::c_uint != 0 {
    let fresh2 = args;
    args = args.offset(1);
    arp_getdevhw(*fresh2, &mut req.arp_ha);
  } else {
    let fresh3 = args;
    args = args.offset(1);
    if (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hw)
      .input
      .expect("non-null function pointer")(*fresh3, &mut req.arp_ha)
      < 0i32
    {
      bb_simple_error_msg_and_die(
        b"invalid hardware address\x00" as *const u8 as *const libc::c_char,
      );
    }
  }
  /* Check out any modifiers. */
  flags = 0x4i32 | 0x2i32;
  while !(*args).is_null() {
    match index_in_strings(options.as_ptr(), *args) {
      0 => {
        /* "pub" */
        flags |= 0x8i32;
        args = args.offset(1)
      }
      1 => {
        /* "priv" */
        flags &= !0x8i32;
        args = args.offset(1)
      }
      2 => {
        /* "temp" */
        flags &= !0x4i32;
        args = args.offset(1)
      }
      3 => {
        /* "trail" */
        flags |= 0x10i32;
        args = args.offset(1)
      }
      4 => {
        /* "dontpub" */
        bb_simple_error_msg(
          b"feature ATF_DONTPUB is not supported\x00" as *const u8 as *const libc::c_char,
        );
        args = args.offset(1)
      }
      5 => {
        /* "auto" */
        bb_simple_error_msg(
          b"feature ATF_MAGIC is not supported\x00" as *const u8 as *const libc::c_char,
        );
        args = args.offset(1)
      }
      6 => {
        /* "dev" */
        args = args.offset(1);
        if (*args).is_null() {
          bb_show_usage();
        }
        let ref mut fresh4 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).device;
        *fresh4 = *args;
        args = args.offset(1)
      }
      7 => {
        /* "netmask" */
        args = args.offset(1);
        if (*args).is_null() {
          bb_show_usage();
        }
        if strcmp(
          *args,
          b"255.255.255.255\x00" as *const u8 as *const libc::c_char,
        ) != 0i32
        {
          host = *args;
          if (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).ap)
            .input
            .expect("non-null function pointer")(host, &mut sa)
            < 0i32
          {
            bb_simple_herror_msg_and_die(host);
          }
          memcpy(
            &mut req.arp_netmask as *mut sockaddr as *mut libc::c_void,
            &mut sa as *mut sockaddr as *const libc::c_void,
            ::std::mem::size_of::<sockaddr>() as libc::c_ulong,
          );
          flags |= 0x20i32
        }
        args = args.offset(1)
      }
      _ => {
        bb_show_usage();
      }
    }
  }
  /* Fill in the remainder of the request. */
  req.arp_flags = flags;
  strncpy_IFNAMSIZ(
    req.arp_dev.as_mut_ptr(),
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).device,
  );
  /* Call the kernel. */
  if option_mask32 & ARP_OPT_v as libc::c_int as libc::c_uint != 0 {
    bb_simple_error_msg(b"SIOCSARP()\x00" as *const u8 as *const libc::c_char);
  }
  bb_xioctl(
    sockfd as libc::c_int,
    0x8955i32 as libc::c_uint,
    &mut req as *mut arpreq as *mut libc::c_void,
    b"SIOCSARP\x00" as *const u8 as *const libc::c_char,
  );
  return 0i32;
}
/* Print the contents of an ARP request block. */
unsafe extern "C" fn arp_disp(
  mut name: *const libc::c_char,
  mut ip: *mut libc::c_char,
  mut type_0: libc::c_int,
  mut arp_flags: libc::c_int,
  mut hwa: *mut libc::c_char,
  mut mask: *mut libc::c_char,
  mut dev: *mut libc::c_char,
) {
  static mut arp_masks: [libc::c_int; 3] = [0x4i32, 0x8i32, 0x10i32];
  static mut arp_labels: [libc::c_char; 16] =
    [80, 69, 82, 77, 0, 80, 85, 80, 0, 84, 82, 65, 73, 76, 0, 0];
  let mut xhw: *const hwtype = 0 as *const hwtype;
  xhw = get_hwntype(type_0);
  if xhw.is_null() {
    xhw = get_hwtype(b"ether\x00" as *const u8 as *const libc::c_char)
  }
  printf(
    b"%s (%s) at \x00" as *const u8 as *const libc::c_char,
    name,
    ip,
  );
  if arp_flags & 0x2i32 == 0 {
    if arp_flags & 0x8i32 != 0 {
      printf(b"* \x00" as *const u8 as *const libc::c_char);
    } else {
      printf(b"<incomplete> \x00" as *const u8 as *const libc::c_char);
    }
  } else {
    printf(
      b"%s [%s] \x00" as *const u8 as *const libc::c_char,
      hwa,
      (*xhw).name,
    );
  }
  if arp_flags & 0x20i32 != 0 {
    printf(b"netmask %s \x00" as *const u8 as *const libc::c_char, mask);
  }
  print_flags_separated(
    arp_masks.as_ptr(),
    arp_labels.as_ptr(),
    arp_flags,
    b" \x00" as *const u8 as *const libc::c_char,
  );
  printf(b" on %s\n\x00" as *const u8 as *const libc::c_char, dev);
}
/* Display the contents of the ARP cache in the kernel. */
/* Called only from main, once */
unsafe extern "C" fn arp_show(mut name: *mut libc::c_char) -> libc::c_int {
  let mut host: *const libc::c_char = 0 as *const libc::c_char;
  let mut hostname: *const libc::c_char = 0 as *const libc::c_char;
  let mut fp: *mut FILE = 0 as *mut FILE;
  let mut sa: sockaddr = sockaddr {
    sa_family: 0,
    sa_data: [0; 14],
  };
  let mut type_0: libc::c_int = 0;
  let mut flags: libc::c_int = 0;
  let mut num: libc::c_int = 0;
  let mut entries: libc::c_uint = 0i32 as libc::c_uint;
  let mut shown: libc::c_uint = 0i32 as libc::c_uint;
  let mut ip: [libc::c_char; 128] = [0; 128];
  let mut hwa: [libc::c_char; 128] = [0; 128];
  let mut mask: [libc::c_char; 128] = [0; 128];
  let mut line: [libc::c_char; 128] = [0; 128];
  let mut dev: [libc::c_char; 128] = [0; 128];
  host = 0 as *const libc::c_char;
  if !name.is_null() {
    /* Resolve the host name. */
    if (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).ap)
      .input
      .expect("non-null function pointer")(name, &mut sa)
      < 0i32
    {
      bb_simple_herror_msg_and_die(name);
    }
    host = xstrdup((*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).ap)
      .sprint
      .expect("non-null function pointer")(&mut sa, 1i32))
  }
  fp = xfopen_for_read(b"/proc/net/arp\x00" as *const u8 as *const libc::c_char);
  /* Bypass header -- read one line */
  fgets_unlocked(
    line.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
    fp,
  );
  /* Read the ARP cache entries. */
  while !fgets_unlocked(
    line.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
    fp,
  )
  .is_null()
  {
    mask[0] = '-' as i32 as libc::c_char;
    mask[1] = '\u{0}' as i32 as libc::c_char;
    dev[0] = '-' as i32 as libc::c_char;
    dev[1] = '\u{0}' as i32 as libc::c_char;
    /* All these strings can't overflow
     * because fgets above reads limited amount of data */
    num = sscanf(
      line.as_mut_ptr(),
      b"%s 0x%x 0x%x %s %s %s\n\x00" as *const u8 as *const libc::c_char,
      ip.as_mut_ptr(),
      &mut type_0 as *mut libc::c_int,
      &mut flags as *mut libc::c_int,
      hwa.as_mut_ptr(),
      mask.as_mut_ptr(),
      dev.as_mut_ptr(),
    );
    if num < 4i32 {
      break;
    }
    entries = entries.wrapping_add(1);
    /* if the user specified hw-type differs, skip it */
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hw_set as libc::c_int != 0
      && type_0 != (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hw).type_0
    {
      continue;
    }
    /* if the user specified address differs, skip it */
    if !host.is_null() && strcmp(ip.as_mut_ptr(), host) != 0i32 {
      continue;
    }
    /* if the user specified device differs, skip it */
    if *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .device
      .offset(0) as libc::c_int
      != 0
      && strcmp(
        dev.as_mut_ptr(),
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).device,
      ) != 0i32
    {
      continue;
    }
    shown = shown.wrapping_add(1);
    /* This IS ugly but it works -be */
    hostname = b"?\x00" as *const u8 as *const libc::c_char;
    if option_mask32 & ARP_OPT_n as libc::c_int as libc::c_uint == 0 {
      if (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).ap)
        .input
        .expect("non-null function pointer")(ip.as_mut_ptr(), &mut sa)
        < 0i32
      {
        hostname = ip.as_mut_ptr()
      } else {
        hostname = (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).ap)
          .sprint
          .expect("non-null function pointer")(
          &mut sa,
          (option_mask32 & ARP_OPT_n as libc::c_int as libc::c_uint | 0x8000i32 as libc::c_uint)
            as libc::c_int,
        )
      }
      if strcmp(hostname, ip.as_mut_ptr()) == 0i32 {
        hostname = b"?\x00" as *const u8 as *const libc::c_char
      }
    }
    arp_disp(
      hostname,
      ip.as_mut_ptr(),
      type_0,
      flags,
      hwa.as_mut_ptr(),
      mask.as_mut_ptr(),
      dev.as_mut_ptr(),
    );
  }
  if option_mask32 & ARP_OPT_v as libc::c_int as libc::c_uint != 0 {
    printf(
      b"Entries: %u\tSkipped: %u\tFound: %u\n\x00" as *const u8 as *const libc::c_char,
      entries,
      entries.wrapping_sub(shown),
      shown,
    );
  }
  if shown == 0 {
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hw_set as libc::c_int != 0
      || !host.is_null()
      || *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .device
        .offset(0) as libc::c_int
        != 0
    {
      printf(
        b"No match found in %u entries\n\x00" as *const u8 as *const libc::c_char,
        entries,
      );
    }
  }
  return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn arp_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut hw_type: *const libc::c_char = 0 as *const libc::c_char;
  let mut protocol: *const libc::c_char = 0 as *const libc::c_char;
  let mut opts: libc::c_uint = 0;
  let ref mut fresh5 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).device;
  *fresh5 = b"\x00" as *const u8 as *const libc::c_char;
  xmove_fd(
    xsocket(2i32, SOCK_DGRAM as libc::c_int, 0i32),
    sockfd as libc::c_int,
  );
  let ref mut fresh6 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).ap;
  *fresh6 = get_aftype(b"inet\x00" as *const u8 as *const libc::c_char);
  /* Defaults are always supported */
  //if (!ap)
  //	bb_error_msg_and_die("%s: %s not supported", DFLT_AF, "address family");
  let ref mut fresh7 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hw;
  *fresh7 = get_hwtype(b"ether\x00" as *const u8 as *const libc::c_char);
  //if (!hw)
  //	bb_error_msg_and_die("%s: %s not supported", DFLT_HW, "hardware type");
  opts = getopt32(
    argv,
    b"A:p:H:t:i:adnDsv\x00" as *const u8 as *const libc::c_char,
    &mut protocol as *mut *const libc::c_char,
    &mut protocol as *mut *const libc::c_char,
    &mut hw_type as *mut *const libc::c_char,
    &mut hw_type as *mut *const libc::c_char,
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).device as *mut *const libc::c_char,
  );
  argv = argv.offset(optind as isize);
  if opts & (ARP_OPT_A as libc::c_int | ARP_OPT_p as libc::c_int) as libc::c_uint != 0 {
    let ref mut fresh8 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).ap;
    *fresh8 = get_aftype(protocol);
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .ap
      .is_null()
    {
      bb_error_msg_and_die(
        b"%s: unknown %s\x00" as *const u8 as *const libc::c_char,
        protocol,
        b"address family\x00" as *const u8 as *const libc::c_char,
      );
    }
  }
  if opts & (ARP_OPT_H as libc::c_int | ARP_OPT_t as libc::c_int) as libc::c_uint != 0 {
    let ref mut fresh9 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hw;
    *fresh9 = get_hwtype(hw_type);
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .hw
      .is_null()
    {
      bb_error_msg_and_die(
        b"%s: unknown %s\x00" as *const u8 as *const libc::c_char,
        hw_type,
        b"hardware type\x00" as *const u8 as *const libc::c_char,
      );
    }
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hw_set = 1i32 as smallint
  }
  //if (opts & ARP_OPT_i)... -i
  if (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).ap).af != 2i32 {
    bb_error_msg_and_die(
      b"%s: kernel only supports \'inet\'\x00" as *const u8 as *const libc::c_char,
      (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).ap).name,
    );
  }
  if (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hw).alen <= 0i32 {
    bb_error_msg_and_die(
      b"%s: %s without ARP support\x00" as *const u8 as *const libc::c_char,
      (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hw).name,
      b"hardware type\x00" as *const u8 as *const libc::c_char,
    );
  }
  /* Now see what we have to do here... */
  if opts & (ARP_OPT_d as libc::c_int | ARP_OPT_s as libc::c_int) as libc::c_uint != 0 {
    if (*argv.offset(0)).is_null() {
      bb_simple_error_msg_and_die(b"need host name\x00" as *const u8 as *const libc::c_char);
    }
    if opts & ARP_OPT_s as libc::c_int as libc::c_uint != 0 {
      return arp_set(argv);
    }
    return arp_del(argv);
  }
  //if (opts & ARP_OPT_a) - default
  return arp_show(*argv.offset(0));
}
