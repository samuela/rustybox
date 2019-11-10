use libc;
use libc::putenv;
use libc::umask;
use libc::mknod;
use libc::fchmod;
use libc::fscanf;
use libc::alarm;
use libc::sync;
use libc::setsid;
use libc::ioctl;
use libc::statfs;
use libc::mount;
use libc::prctl;
use libc::opendir;
use libc::closedir;
use libc::readdir;
use libc::strtok;
use libc::putchar_unlocked;
use libc::endmntent;
use libc::setmntent;
use libc::umount2;
use libc::getegid;
use libc::getuid;
use libc::getgid;
use libc::setutxent;
use libc::endutxent;







use libc::geteuid;





















use libc::printf;
use libc::puts;











extern "C" {

  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn strncpy_IFNAMSIZ(dst: *mut libc::c_char, src: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xuname2uid(name: *const libc::c_char) -> libc::c_long;
  #[no_mangle]
  fn xgroup2gid(name: *const libc::c_char) -> libc::c_long;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  fn ioctl_or_perror_and_die(
    fd: libc::c_int,
    request: libc::c_uint,
    argp: *mut libc::c_void,
    fmt: *const libc::c_char,
    _: ...
  ) -> libc::c_int;
}

pub type __caddr_t = *mut libc::c_char;

use libc::sockaddr;
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
pub type uintptr_t = libc::c_ulong;
// set new interface group
pub const OPT_b: C2RustUnnamed_1 = 32;
// delete named interface
pub const OPT_u: C2RustUnnamed_1 = 8;
// set new interface owner
pub const OPT_g: C2RustUnnamed_1 = 16;
// create named interface
pub const OPT_d: C2RustUnnamed_1 = 4;
pub type C2RustUnnamed_1 = libc::c_uint;
// brief output
// control device name (/dev/net/tun)
pub const OPT_t: C2RustUnnamed_1 = 2;
pub const OPT_f: C2RustUnnamed_1 = 1;

/*
 * tun devices controller
 *
 * Copyright (C) 2008 by Vladimir Dronnikov <dronnikov@gmail.com>
 *
 * Original code:
 *      Jeff Dike
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
//config:config TUNCTL
//config:	bool "tunctl (6.2 kb)"
//config:	default y
//config:	select PLATFORM_LINUX
//config:	help
//config:	tunctl creates or deletes tun devices.
//config:
//config:config FEATURE_TUNCTL_UG
//config:	bool "Support owner:group assignment"
//config:	default y
//config:	depends on TUNCTL
//config:	help
//config:	Allow to specify owner and group of newly created interface.
//config:	340 bytes of pure bloat. Say no here.
//applet:IF_TUNCTL(APPLET_NOEXEC(tunctl, tunctl, BB_DIR_SBIN, BB_SUID_DROP, tunctl))
//kbuild:lib-$(CONFIG_TUNCTL) += tunctl.o
//usage:#define tunctl_trivial_usage
//usage:       "[-f device] ([-t name] | -d name)" IF_FEATURE_TUNCTL_UG(" [-u owner] [-g group] [-b]")
//usage:#define tunctl_full_usage "\n\n"
//usage:       "Create or delete tun interfaces\n"
//usage:     "\n	-f name		tun device (/dev/net/tun)"
//usage:     "\n	-t name		Create iface 'name'"
//usage:     "\n	-d name		Delete iface 'name'"
//usage:	IF_FEATURE_TUNCTL_UG(
//usage:     "\n	-u owner	Set iface owner"
//usage:     "\n	-g group	Set iface group"
//usage:     "\n	-b		Brief output"
//usage:	)
//usage:
//usage:#define tunctl_example_usage
//usage:       "# tunctl\n"
//usage:       "# tunctl -d tun0\n"
/* TUNSETGROUP appeared in 2.6.23 */
#[no_mangle]
pub unsafe extern "C" fn tunctl_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut ifr: ifreq = ifreq {
    ifr_ifrn: C2RustUnnamed_0 { ifrn_name: [0; 16] },
    ifr_ifru: C2RustUnnamed {
      ifru_addr: sockaddr {
        sa_family: 0,
        sa_data: [0; 14],
      },
    },
  };
  let mut fd: libc::c_int = 0;
  let mut opt_name: *const libc::c_char = b"tap%d\x00" as *const u8 as *const libc::c_char;
  let mut opt_device: *const libc::c_char = b"/dev/net/tun\x00" as *const u8 as *const libc::c_char;
  let mut opt_user: *const libc::c_char = 0 as *const libc::c_char;
  let mut opt_group: *const libc::c_char = 0 as *const libc::c_char;
  let mut user: libc::c_long = -1i32 as libc::c_long;
  let mut group: libc::c_long = -1i32 as libc::c_long;
  let mut opts: libc::c_uint = 0;
  opts = getopt32(
    argv,
    b"^f:t:d:u:g:b\x00=0:t--d:d--t\x00" as *const u8 as *const libc::c_char,
    &mut opt_device as *mut *const libc::c_char,
    &mut opt_name as *mut *const libc::c_char,
    &mut opt_name as *mut *const libc::c_char,
    &mut opt_user as *mut *const libc::c_char,
    &mut opt_group as *mut *const libc::c_char,
  );
  // select device
  memset(
    &mut ifr as *mut ifreq as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<ifreq>() as libc::c_ulong,
  );
  ifr.ifr_ifru.ifru_flags = (0x2i32 | 0x1000i32) as libc::c_short;
  strncpy_IFNAMSIZ(ifr.ifr_ifrn.ifrn_name.as_mut_ptr(), opt_name);
  // open device
  fd = xopen(opt_device, 0o2i32);
  ioctl_or_perror_and_die(
    fd,
    ((1u32 << 0i32 + 8i32 + 8i32 + 14i32
      | (('T' as i32) << 0i32 + 8i32) as libc::c_uint
      | (202i32 << 0i32) as libc::c_uint) as libc::c_ulong
      | (::std::mem::size_of::<libc::c_int>() as libc::c_ulong) << 0i32 + 8i32 + 8i32)
      as libc::c_uint,
    &mut ifr as *mut ifreq as *mut libc::c_void,
    0 as *const libc::c_char,
  );
  // delete?
  if opts & OPT_d as libc::c_int as libc::c_uint != 0 {
    ioctl_or_perror_and_die(
      fd,
      ((1u32 << 0i32 + 8i32 + 8i32 + 14i32
        | (('T' as i32) << 0i32 + 8i32) as libc::c_uint
        | (203i32 << 0i32) as libc::c_uint) as libc::c_ulong
        | (::std::mem::size_of::<libc::c_int>() as libc::c_ulong) << 0i32 + 8i32 + 8i32)
        as libc::c_uint,
      0 as *mut libc::c_void,
      0 as *const libc::c_char,
    );
    printf(
      b"Set \'%s\' nonpersistent\n\x00" as *const u8 as *const libc::c_char,
      ifr.ifr_ifrn.ifrn_name.as_mut_ptr(),
    );
    return 0i32;
  }
  // create
  if opts & OPT_g as libc::c_int as libc::c_uint != 0 {
    group = xgroup2gid(opt_group);
    ioctl_or_perror_and_die(
      fd,
      ((1u32 << 0i32 + 8i32 + 8i32 + 14i32
        | (('T' as i32) << 0i32 + 8i32) as libc::c_uint
        | (206i32 << 0i32) as libc::c_uint) as libc::c_ulong
        | (::std::mem::size_of::<libc::c_int>() as libc::c_ulong) << 0i32 + 8i32 + 8i32)
        as libc::c_uint,
      group as uintptr_t as *mut libc::c_void,
      0 as *const libc::c_char,
    );
  } else {
    user = geteuid() as libc::c_long
  }
  if opts & OPT_u as libc::c_int as libc::c_uint != 0 {
    user = xuname2uid(opt_user)
  }
  ioctl_or_perror_and_die(
    fd,
    ((1u32 << 0i32 + 8i32 + 8i32 + 14i32
      | (('T' as i32) << 0i32 + 8i32) as libc::c_uint
      | (204i32 << 0i32) as libc::c_uint) as libc::c_ulong
      | (::std::mem::size_of::<libc::c_int>() as libc::c_ulong) << 0i32 + 8i32 + 8i32)
      as libc::c_uint,
    user as uintptr_t as *mut libc::c_void,
    0 as *const libc::c_char,
  );
  ioctl_or_perror_and_die(
    fd,
    ((1u32 << 0i32 + 8i32 + 8i32 + 14i32
      | (('T' as i32) << 0i32 + 8i32) as libc::c_uint
      | (203i32 << 0i32) as libc::c_uint) as libc::c_ulong
      | (::std::mem::size_of::<libc::c_int>() as libc::c_ulong) << 0i32 + 8i32 + 8i32)
      as libc::c_uint,
    1i32 as uintptr_t as *mut libc::c_void,
    0 as *const libc::c_char,
  );
  // show info
  if opts & OPT_b as libc::c_int as libc::c_uint != 0 {
    puts(ifr.ifr_ifrn.ifrn_name.as_mut_ptr());
  } else {
    printf(
      b"Set \'%s\' %spersistent\x00" as *const u8 as *const libc::c_char,
      ifr.ifr_ifrn.ifrn_name.as_mut_ptr(),
      b"\x00" as *const u8 as *const libc::c_char,
    );
    printf(
      b" and owned by uid %ld\x00" as *const u8 as *const libc::c_char,
      user,
    );
    if group != -1i32 as libc::c_long {
      printf(b" gid %ld\x00" as *const u8 as *const libc::c_char, group);
    }
    bb_putchar('\n' as i32);
  }
  return 0i32;
}
