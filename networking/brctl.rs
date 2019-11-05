use libc;
extern "C" {
  pub type __dirstream;
  #[no_mangle]
  fn strtod(__nptr: *const libc::c_char, __endptr: *mut *mut libc::c_char) -> libc::c_double;
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t, __compar: __compar_fn_t);
  #[no_mangle]
  fn opendir(__name: *const libc::c_char) -> *mut DIR;
  #[no_mangle]
  fn closedir(__dirp: *mut DIR) -> libc::c_int;
  #[no_mangle]
  fn readdir(__dirp: *mut DIR) -> *mut dirent;
  #[no_mangle]
  fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
  #[no_mangle]
  static mut stdout: *mut FILE;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn puts(__s: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
  #[no_mangle]
  fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xrealloc_vector_helper(
    vector: *mut libc::c_void,
    sizeof_and_shift: libc::c_uint,
    idx: libc::c_int,
  ) -> *mut libc::c_void;
  #[no_mangle]
  fn xopendir(path: *const libc::c_char) -> *mut DIR;
  #[no_mangle]
  fn xchdir(path: *const libc::c_char);
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xsocket(domain: libc::c_int, type_0: libc::c_int, protocol: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn strncpy_IFNAMSIZ(dst: *mut libc::c_char, src: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn full_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn open_read_close(
    filename: *const libc::c_char,
    buf: *mut libc::c_void,
    maxsz: size_t,
  ) -> ssize_t;
  #[no_mangle]
  fn utoa(n: libc::c_uint) -> *mut libc::c_char;
  #[no_mangle]
  fn xstrtoull(str: *const libc::c_char, b: libc::c_int) -> libc::c_ulonglong;
  #[no_mangle]
  fn xstrtou(str: *const libc::c_char, b: libc::c_int) -> libc::c_uint;
  #[no_mangle]
  fn xatoi_positive(numstr: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_perror_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn index_in_strings(strings: *const libc::c_char, key: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn nth_string(strings: *const libc::c_char, n: libc::c_int) -> *const libc::c_char;
  #[no_mangle]
  fn ioctl_or_perror_and_die(
    fd: libc::c_int,
    request: libc::c_uint,
    argp: *mut libc::c_void,
    fmt: *const libc::c_char,
    _: ...
  ) -> libc::c_int;
  #[no_mangle]
  static mut applet_name: *const libc::c_char;
  #[no_mangle]
  static bb_msg_invalid_arg_to: [libc::c_char; 0];
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
  #[no_mangle]
  fn if_nametoindex(__ifname: *const libc::c_char) -> libc::c_uint;
}

use crate::librb::__uint16_t;

use crate::librb::__ino64_t;
use crate::librb::__off_t;
use crate::librb::__off64_t;

pub type __caddr_t = *mut libc::c_char;
use crate::librb::uint8_t;
use crate::librb::uint16_t;
use crate::librb::uint32_t;
use crate::librb::ssize_t;
use crate::librb::size_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
  pub d_ino: __ino64_t,
  pub d_off: __off64_t,
  pub d_reclen: libc::c_ushort,
  pub d_type: libc::c_uchar,
  pub d_name: [libc::c_char; 256],
}
pub type DIR = __dirstream;
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



use crate::librb::FILE;
use crate::librb::__compar_fn_t;
pub type C2RustUnnamed = libc::c_uint;
pub const COMMON_BUFSIZE: C2RustUnnamed = 1024;
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
  pub ifru_data: __caddr_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
  pub ifrn_name: [libc::c_char; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fdb_entry {
  pub mac_addr: [uint8_t; 6],
  pub port_no: uint8_t,
  pub is_local: uint8_t,
  pub ageing_timer_value: uint32_t,
  pub port_hi: uint8_t,
  pub pad0: uint8_t,
  pub unused: uint16_t,
}
pub const ARG_addif: C2RustUnnamed_2 = 2;
pub const ARG_setpathcost: C2RustUnnamed_2 = 10;
pub const ARG_setportprio: C2RustUnnamed_2 = 11;
pub const ARG_setbridgeprio: C2RustUnnamed_2 = 12;
pub const ARG_setageing: C2RustUnnamed_2 = 6;
pub const ARG_stp: C2RustUnnamed_2 = 4;
pub const ARG_showstp: C2RustUnnamed_2 = 5;
pub const ARG_showmacs: C2RustUnnamed_2 = 13;
pub const ARG_addbr: C2RustUnnamed_2 = 0;
pub const ARG_delbr: C2RustUnnamed_2 = 1;
pub const ARG_show: C2RustUnnamed_2 = 14;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const ARG_setmaxage: C2RustUnnamed_2 = 9;
pub const ARG_sethello: C2RustUnnamed_2 = 8;
pub const ARG_setfd: C2RustUnnamed_2 = 7;
pub const ARG_delif: C2RustUnnamed_2 = 3;

/*
 * Small implementation of brctl for busybox.
 *
 * Copyright (C) 2008 by Bernhard Reutner-Fischer
 *
 * Some helper functions from bridge-utils are
 * Copyright (C) 2000 Lennert Buytenhek
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config BRCTL
//config:	bool "brctl (4.7 kb)"
//config:	default y
//config:	select PLATFORM_LINUX
//config:	help
//config:	Manage ethernet bridges.
//config:	Supports addbr/delbr and addif/delif.
//config:
//config:config FEATURE_BRCTL_FANCY
//config:	bool "Fancy options"
//config:	default y
//config:	depends on BRCTL
//config:	help
//config:	Add support for extended option like:
//config:		setageing, setfd, sethello, setmaxage,
//config:		setpathcost, setportprio, setbridgeprio,
//config:		stp
//config:	This adds about 600 bytes.
//config:
//config:config FEATURE_BRCTL_SHOW
//config:	bool "Support show"
//config:	default y
//config:	depends on BRCTL && FEATURE_BRCTL_FANCY
//config:	help
//config:	Add support for option which prints the current config:
//config:		show
//applet:IF_BRCTL(APPLET_NOEXEC(brctl, brctl, BB_DIR_USR_SBIN, BB_SUID_DROP, brctl))
//kbuild:lib-$(CONFIG_BRCTL) += brctl.o
//usage:#define brctl_trivial_usage
//usage:       "COMMAND [BRIDGE [ARGS]]"
//usage:#define brctl_full_usage "\n\n"
//usage:       "Manage ethernet bridges"
//usage:     "\nCommands:"
//usage:	IF_FEATURE_BRCTL_SHOW(
//usage:     "\n	show [BRIDGE]...	Show bridges"
//usage:	)
//usage:     "\n	addbr BRIDGE		Create BRIDGE"
//usage:     "\n	delbr BRIDGE		Delete BRIDGE"
//usage:     "\n	addif BRIDGE IFACE	Add IFACE to BRIDGE"
//usage:     "\n	delif BRIDGE IFACE	Delete IFACE from BRIDGE"
//usage:	IF_FEATURE_BRCTL_FANCY(
//usage:     "\n	showmacs BRIDGE			List MAC addresses"
//usage:     "\n	showstp	BRIDGE			Show STP info"
//usage:     "\n	stp BRIDGE 1/yes/on|0/no/off	Set STP on/off"
//usage:     "\n	setageing BRIDGE SECONDS	Set ageing time"
//usage:     "\n	setfd BRIDGE SECONDS		Set bridge forward delay"
//usage:     "\n	sethello BRIDGE SECONDS		Set hello time"
//usage:     "\n	setmaxage BRIDGE SECONDS	Set max message age"
//usage:     "\n	setbridgeprio BRIDGE PRIO	Set bridge priority"
//usage:     "\n	setportprio BRIDGE IFACE PRIO	Set port priority"
//usage:     "\n	setpathcost BRIDGE IFACE COST	Set path cost"
//usage:	)
// Not yet implemented:
//			hairpin BRIDGE IFACE on|off	Set hairpin on/off
unsafe extern "C" fn str_to_jiffies(mut time_str: *const libc::c_char) -> libc::c_uint {
  let mut dd: libc::c_double = 0.;
  let mut endptr: *mut libc::c_char = 0 as *mut libc::c_char;
  dd = strtod(time_str, &mut endptr);
  if endptr == time_str as *mut libc::c_char || dd < 0i32 as libc::c_double {
    bb_error_msg_and_die(
      bb_msg_invalid_arg_to.as_ptr(),
      time_str,
      b"timespec\x00" as *const u8 as *const libc::c_char,
    );
  }
  dd *= 100i32 as libc::c_double;
  /* For purposes of brctl,
   * capping SECONDS by ~20 million seconds is quite enough:
   */
  if dd > 2147483647i32 as libc::c_double {
    dd = 2147483647i32 as libc::c_double
  }
  return dd as libc::c_uint;
}
unsafe extern "C" fn read_file(mut name: *const libc::c_char) -> libc::c_int {
  let mut n: libc::c_int = open_read_close(
    name,
    bb_common_bufsiz1.as_mut_ptr() as *mut libc::c_void,
    (COMMON_BUFSIZE as libc::c_int - 1i32) as size_t,
  ) as libc::c_int;
  if n < 0i32 {
    *bb_common_bufsiz1.as_mut_ptr().offset(0) = '\u{0}' as i32 as libc::c_char
  } else {
    *bb_common_bufsiz1.as_mut_ptr().offset(n as isize) = '\u{0}' as i32 as libc::c_char;
    if n != 0i32
      && *bb_common_bufsiz1.as_mut_ptr().offset((n - 1i32) as isize) as libc::c_int == '\n' as i32
    {
      n -= 1;
      *bb_common_bufsiz1.as_mut_ptr().offset(n as isize) = '\u{0}' as i32 as libc::c_char
    }
  }
  return n;
}
/* NB: we are in /sys/class/net
 */
unsafe extern "C" fn show_bridge(
  mut name: *const libc::c_char,
  mut need_hdr: libc::c_int,
) -> libc::c_int {
  /* Output:
   *bridge name	bridge id		STP enabled	interfaces
   *br0		8000.000000000000	no		eth0
   */
  let mut pathbuf: [libc::c_char; 42] = [0; 42]; /* this iface is not a bridge */
  let mut tabs: libc::c_int = 0;
  let mut ifaces: *mut DIR = 0 as *mut DIR;
  let mut ent: *mut dirent = 0 as *mut dirent;
  let mut sfx: *mut libc::c_char = 0 as *mut libc::c_char;
  sfx = pathbuf.as_mut_ptr().offset(sprintf(
    pathbuf.as_mut_ptr(),
    b"%.16s/bridge/\x00" as *const u8 as *const libc::c_char,
    name,
  ) as isize);
  strcpy(sfx, b"bridge_id\x00" as *const u8 as *const libc::c_char);
  if read_file(pathbuf.as_mut_ptr()) < 0i32 {
    return -1i32;
  }
  if need_hdr != 0 {
    puts(
      b"bridge name\tbridge id\t\tSTP enabled\tinterfaces\x00" as *const u8 as *const libc::c_char,
    );
  }
  printf(
    b"%s\t\t%s\t\x00" as *const u8 as *const libc::c_char,
    name,
    bb_common_bufsiz1.as_mut_ptr(),
  );
  strcpy(sfx, b"stp_state\x00" as *const u8 as *const libc::c_char);
  read_file(pathbuf.as_mut_ptr());
  if *bb_common_bufsiz1.as_mut_ptr().offset(0) as libc::c_int == '0' as i32
    && *bb_common_bufsiz1.as_mut_ptr().offset(1) == 0
  {
    strcpy(
      bb_common_bufsiz1.as_mut_ptr(),
      b"no\x00" as *const u8 as *const libc::c_char,
    );
  } else if *bb_common_bufsiz1.as_mut_ptr().offset(0) as libc::c_int == '1' as i32
    && *bb_common_bufsiz1.as_mut_ptr().offset(1) == 0
  {
    strcpy(
      bb_common_bufsiz1.as_mut_ptr(),
      b"yes\x00" as *const u8 as *const libc::c_char,
    );
  }
  fputs_unlocked(bb_common_bufsiz1.as_mut_ptr(), stdout);
  /* sfx points past "BR/bridge/", turn it into "BR/brif": */
  *sfx.offset(-4i32 as isize) = 'f' as i32 as libc::c_char; /* . or .. */
  *sfx.offset(-3i32 as isize) = '\u{0}' as i32 as libc::c_char;
  tabs = 0i32;
  ifaces = opendir(pathbuf.as_mut_ptr());
  if !ifaces.is_null() {
    loop {
      ent = readdir(ifaces);
      if ent.is_null() {
        break;
      }
      if (*ent).d_name[0] as libc::c_int == '.' as i32
        && ((*ent).d_name[1] == 0
          || (*ent).d_name[1] as libc::c_int == '.' as i32 && (*ent).d_name[2] == 0)
      {
        continue;
      }
      if tabs != 0 {
        printf(b"\t\t\t\t\t\x00" as *const u8 as *const libc::c_char);
      } else {
        tabs = 1i32
      }
      printf(
        b"\t\t%s\n\x00" as *const u8 as *const libc::c_char,
        (*ent).d_name.as_mut_ptr(),
      );
    }
    closedir(ifaces);
  }
  if tabs == 0 {
    /* bridge has no interfaces */
    bb_putchar('\n' as i32);
  }
  return 0i32;
}
unsafe extern "C" fn write_uint(
  mut name: *const libc::c_char,
  mut leaf: *const libc::c_char,
  mut val: libc::c_uint,
) {
  let mut pathbuf: [libc::c_char; 66] = [0; 66];
  let mut fd: libc::c_int = 0;
  let mut n: libc::c_int = 0;
  sprintf(
    pathbuf.as_mut_ptr(),
    b"%.16s/%s\x00" as *const u8 as *const libc::c_char,
    name,
    leaf,
  );
  fd = xopen(pathbuf.as_mut_ptr(), 0o1i32);
  n = sprintf(
    bb_common_bufsiz1.as_mut_ptr(),
    b"%u\n\x00" as *const u8 as *const libc::c_char,
    val,
  );
  if write(
    fd,
    bb_common_bufsiz1.as_mut_ptr() as *const libc::c_void,
    n as size_t,
  ) < 0i32 as libc::c_long
  {
    bb_simple_perror_msg_and_die(name);
  };
}
unsafe extern "C" fn compare_fdbs(
  mut _f0: *const libc::c_void,
  mut _f1: *const libc::c_void,
) -> libc::c_int {
  let mut f0: *const fdb_entry = _f0 as *const fdb_entry;
  let mut f1: *const fdb_entry = _f1 as *const fdb_entry;
  return memcmp(
    (*f0).mac_addr.as_ptr() as *const libc::c_void,
    (*f1).mac_addr.as_ptr() as *const libc::c_void,
    6i32 as libc::c_ulong,
  );
}
unsafe extern "C" fn read_bridge_forward_db(
  mut name: *const libc::c_char,
  mut _fdb: *mut *mut fdb_entry,
) -> size_t {
  let mut pathbuf: [libc::c_char; 35] = [0; 35];
  let mut fdb: *mut fdb_entry = 0 as *mut fdb_entry;
  let mut nentries: size_t = 0;
  let mut fd: libc::c_int = 0;
  let mut cc: ssize_t = 0;
  sprintf(
    pathbuf.as_mut_ptr(),
    b"%.16s/brforward\x00" as *const u8 as *const libc::c_char,
    name,
  );
  fd = open(pathbuf.as_mut_ptr(), 0i32);
  if fd < 0i32 {
    bb_error_msg_and_die(
      b"bridge %s does not exist\x00" as *const u8 as *const libc::c_char,
      name,
    );
  }
  fdb = 0 as *mut fdb_entry;
  nentries = 0i32 as size_t;
  loop {
    fdb = xrealloc_vector_helper(
      fdb as *mut libc::c_void,
      ((::std::mem::size_of::<fdb_entry>() as libc::c_ulong) << 8i32)
        .wrapping_add(4i32 as libc::c_ulong) as libc::c_uint,
      nentries as libc::c_int,
    ) as *mut fdb_entry;
    cc = full_read(
      fd,
      &mut *fdb.offset(nentries as isize) as *mut fdb_entry as *mut libc::c_void,
      ::std::mem::size_of::<fdb_entry>() as libc::c_ulong,
    );
    if cc == 0i32 as libc::c_long {
      break;
    }
    if cc as libc::c_ulong != ::std::mem::size_of::<fdb_entry>() as libc::c_ulong {
      bb_perror_msg_and_die(
        b"can\'t read bridge %s forward db\x00" as *const u8 as *const libc::c_char,
        name,
      );
    }
    nentries = nentries.wrapping_add(1)
  }
  qsort(
    fdb as *mut libc::c_void,
    nentries,
    ::std::mem::size_of::<fdb_entry>() as libc::c_ulong,
    Some(
      compare_fdbs
        as unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> libc::c_int,
    ),
  );
  *_fdb = fdb;
  return nentries;
}
unsafe extern "C" fn show_bridge_macs(mut name: *const libc::c_char) {
  let mut fdb: *mut fdb_entry = 0 as *mut fdb_entry;
  let mut nentries: size_t = 0;
  let mut i: size_t = 0;
  nentries = read_bridge_forward_db(name, &mut fdb);
  printf(b"port no\tmac addr\t\tis local?\tageing timer\n\x00" as *const u8 as *const libc::c_char);
  i = 0i32 as size_t;
  while i < nentries {
    let mut f: *const fdb_entry = &mut *fdb.offset(i as isize) as *mut fdb_entry;
    let mut tv_sec: libc::c_uint = (*f).ageing_timer_value.wrapping_div(100i32 as libc::c_uint);
    let mut tv_csec: libc::c_uint = (*f).ageing_timer_value.wrapping_rem(100i32 as libc::c_uint);
    printf(
      b"%3u\t%.2x:%.2x:%.2x:%.2x:%.2x:%.2x\t%s\t\t%4u.%.2u\n\x00" as *const u8
        as *const libc::c_char,
      (*f).port_no as libc::c_int,
      (*f).mac_addr[0] as libc::c_int,
      (*f).mac_addr[1] as libc::c_int,
      (*f).mac_addr[2] as libc::c_int,
      (*f).mac_addr[3] as libc::c_int,
      (*f).mac_addr[4] as libc::c_int,
      (*f).mac_addr[5] as libc::c_int,
      if (*f).is_local as libc::c_int != 0 {
        b"yes\x00" as *const u8 as *const libc::c_char
      } else {
        b"no\x00" as *const u8 as *const libc::c_char
      },
      tv_sec,
      tv_csec,
    );
    i = i.wrapping_add(1)
  }
}
unsafe extern "C" fn show_bridge_timer(mut msg: *const libc::c_char) {
  let mut centisec: libc::c_ulonglong = xstrtoull(bb_common_bufsiz1.as_mut_ptr(), 0i32);
  let mut tv_sec: libc::c_uint = centisec.wrapping_div(100i32 as libc::c_ulonglong) as libc::c_uint;
  let mut tv_csec: libc::c_uint =
    centisec.wrapping_rem(100i32 as libc::c_ulonglong) as libc::c_uint;
  printf(
    b"%s%4u.%.2u\x00" as *const u8 as *const libc::c_char,
    msg,
    tv_sec,
    tv_csec,
  );
}
unsafe extern "C" fn show_bridge_state(mut state: libc::c_uint) -> *const libc::c_char {
  /* See linux/if_bridge.h, BR_STATE_ constants */
  static mut state_names: [libc::c_char; 48] = [
    100, 105, 115, 97, 98, 108, 101, 100, 0, 108, 105, 115, 116, 101, 110, 105, 110, 103, 0, 108,
    101, 97, 114, 110, 105, 110, 103, 0, 102, 111, 114, 119, 97, 114, 100, 105, 110, 103, 0, 98,
    108, 111, 99, 107, 105, 110, 103, 0,
  ];
  if state < 5i32 as libc::c_uint {
    return nth_string(state_names.as_ptr(), state as libc::c_int);
  }
  return utoa(state);
}
unsafe extern "C" fn printf_xstrtou(mut fmt: *const libc::c_char) {
  printf(fmt, xstrtou(bb_common_bufsiz1.as_mut_ptr(), 0i32));
}
unsafe extern "C" fn show_bridge_port(mut name: *const libc::c_char) {
  let mut pathbuf: [libc::c_char; 52] = [0; 52];
  let mut sfx: *mut libc::c_char = 0 as *mut libc::c_char;
  sfx = pathbuf.as_mut_ptr().offset(sprintf(
    pathbuf.as_mut_ptr(),
    b"%.16s/brport/\x00" as *const u8 as *const libc::c_char,
    name,
  ) as isize);
  strcpy(sfx, b"port_no\x00" as *const u8 as *const libc::c_char);
  read_file(pathbuf.as_mut_ptr());
  printf(
    b"%s (%u)\n\x00" as *const u8 as *const libc::c_char,
    name,
    xstrtou(bb_common_bufsiz1.as_mut_ptr(), 0i32),
  );
  //BR_STATE_BLOCKING   4
  strcpy(sfx.offset(5), b"id\x00" as *const u8 as *const libc::c_char); // "port_id"
  read_file(pathbuf.as_mut_ptr()); // "root_port"
  printf_xstrtou(b" port id\t\t%.4x\x00" as *const u8 as *const libc::c_char); // "root_path_cost"
  strcpy(sfx, b"state\x00" as *const u8 as *const libc::c_char);
  read_file(pathbuf.as_mut_ptr());
  printf(
    b"\t\t\tstate\t\t%15s\n\x00" as *const u8 as *const libc::c_char,
    show_bridge_state(xstrtou(bb_common_bufsiz1.as_mut_ptr(), 0i32)),
  );
  strcpy(
    sfx,
    b"designated_root\x00" as *const u8 as *const libc::c_char,
  );
  read_file(pathbuf.as_mut_ptr());
  printf(
    b" designated root\t%s\x00" as *const u8 as *const libc::c_char,
    bb_common_bufsiz1.as_mut_ptr(),
  );
  strcpy(sfx, b"path_cost\x00" as *const u8 as *const libc::c_char);
  read_file(pathbuf.as_mut_ptr());
  printf_xstrtou(b"\tpath cost\t\t%4u\n\x00" as *const u8 as *const libc::c_char);
  strcpy(
    sfx,
    b"designated_bridge\x00" as *const u8 as *const libc::c_char,
  );
  read_file(pathbuf.as_mut_ptr());
  printf(
    b" designated bridge\t%s\x00" as *const u8 as *const libc::c_char,
    bb_common_bufsiz1.as_mut_ptr(),
  );
  strcpy(
    sfx,
    b"message_age_timer\x00" as *const u8 as *const libc::c_char,
  );
  read_file(pathbuf.as_mut_ptr());
  show_bridge_timer(b"\tmessage age timer\t\x00" as *const u8 as *const libc::c_char);
  strcpy(
    sfx,
    b"designated_port\x00" as *const u8 as *const libc::c_char,
  );
  read_file(pathbuf.as_mut_ptr());
  printf_xstrtou(b"\n designated port\t%.4x\x00" as *const u8 as *const libc::c_char);
  strcpy(
    sfx,
    b"forward_delay_timer\x00" as *const u8 as *const libc::c_char,
  );
  read_file(pathbuf.as_mut_ptr());
  show_bridge_timer(b"\t\t\tforward delay timer\t\x00" as *const u8 as *const libc::c_char);
  strcpy(
    sfx,
    b"designated_cost\x00" as *const u8 as *const libc::c_char,
  );
  read_file(pathbuf.as_mut_ptr());
  printf_xstrtou(b"\n designated cost\t%4u\x00" as *const u8 as *const libc::c_char);
  strcpy(sfx, b"hold_timer\x00" as *const u8 as *const libc::c_char);
  read_file(pathbuf.as_mut_ptr());
  show_bridge_timer(b"\t\t\thold timer\t\t\x00" as *const u8 as *const libc::c_char);
  printf(b"\n flags\t\t\t\x00" as *const u8 as *const libc::c_char);
  strcpy(
    sfx,
    b"config_pending\x00" as *const u8 as *const libc::c_char,
  );
  read_file(pathbuf.as_mut_ptr());
  if !(*bb_common_bufsiz1.as_mut_ptr().offset(0) as libc::c_int == '0' as i32
    && *bb_common_bufsiz1.as_mut_ptr().offset(1) == 0)
  {
    printf(b"CONFIG_PENDING \x00" as *const u8 as *const libc::c_char);
  }
  strcpy(sfx, b"change_ack\x00" as *const u8 as *const libc::c_char);
  read_file(pathbuf.as_mut_ptr());
  if !(*bb_common_bufsiz1.as_mut_ptr().offset(0) as libc::c_int == '0' as i32
    && *bb_common_bufsiz1.as_mut_ptr().offset(1) == 0)
  {
    printf(b"TOPOLOGY_CHANGE_ACK \x00" as *const u8 as *const libc::c_char);
  }
  strcpy(sfx, b"hairpin_mode\x00" as *const u8 as *const libc::c_char);
  read_file(pathbuf.as_mut_ptr());
  if !(*bb_common_bufsiz1.as_mut_ptr().offset(0) as libc::c_int == '0' as i32
    && *bb_common_bufsiz1.as_mut_ptr().offset(1) == 0)
  {
    printf_xstrtou(b"\n hairpin mode\t\t%4u\x00" as *const u8 as *const libc::c_char);
  }
  printf(b"\n\n\x00" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn show_bridge_stp(mut name: *const libc::c_char) {
  let mut pathbuf: [libc::c_char; 54] = [0; 54];
  let mut sfx: *mut libc::c_char = 0 as *mut libc::c_char;
  sfx = pathbuf.as_mut_ptr().offset(sprintf(
    pathbuf.as_mut_ptr(),
    b"%.16s/bridge/\x00" as *const u8 as *const libc::c_char,
    name,
  ) as isize);
  strcpy(sfx, b"bridge_id\x00" as *const u8 as *const libc::c_char);
  if read_file(pathbuf.as_mut_ptr()) < 0i32 {
    bb_error_msg_and_die(
      b"bridge %s does not exist\x00" as *const u8 as *const libc::c_char,
      name,
    );
  }
  printf(
    b"%s\n bridge id\t\t%s\x00" as *const u8 as *const libc::c_char,
    name,
    bb_common_bufsiz1.as_mut_ptr(),
  );
  strcpy(sfx, b"root_id\x00" as *const u8 as *const libc::c_char);
  read_file(pathbuf.as_mut_ptr());
  printf(
    b"\n designated root\t%s\x00" as *const u8 as *const libc::c_char,
    bb_common_bufsiz1.as_mut_ptr(),
  );
  strcpy(
    sfx.offset(5),
    b"port\x00" as *const u8 as *const libc::c_char,
  );
  read_file(pathbuf.as_mut_ptr());
  printf_xstrtou(b"\n root port\t\t%4u\t\t\t\x00" as *const u8 as *const libc::c_char);
  strcpy(
    sfx.offset(6),
    b"ath_cost\x00" as *const u8 as *const libc::c_char,
  );
  read_file(pathbuf.as_mut_ptr());
  printf_xstrtou(b"path cost\t\t%4u\n\x00" as *const u8 as *const libc::c_char);
  strcpy(sfx, b"max_age\x00" as *const u8 as *const libc::c_char);
  read_file(pathbuf.as_mut_ptr());
  show_bridge_timer(b" max age\t\t\x00" as *const u8 as *const libc::c_char);
  show_bridge_timer(b"\t\t\tbridge max age\t\t\x00" as *const u8 as *const libc::c_char);
  strcpy(sfx, b"hello_time\x00" as *const u8 as *const libc::c_char);
  read_file(pathbuf.as_mut_ptr());
  show_bridge_timer(b"\n hello time\t\t\x00" as *const u8 as *const libc::c_char);
  show_bridge_timer(b"\t\t\tbridge hello time\t\x00" as *const u8 as *const libc::c_char);
  strcpy(
    sfx,
    b"forward_delay\x00" as *const u8 as *const libc::c_char,
  );
  read_file(pathbuf.as_mut_ptr());
  show_bridge_timer(b"\n forward delay\t\t\x00" as *const u8 as *const libc::c_char);
  show_bridge_timer(b"\t\t\tbridge forward delay\t\x00" as *const u8 as *const libc::c_char);
  strcpy(sfx, b"ageing_time\x00" as *const u8 as *const libc::c_char);
  read_file(pathbuf.as_mut_ptr());
  show_bridge_timer(b"\n ageing time\t\t\x00" as *const u8 as *const libc::c_char);
  strcpy(sfx, b"hello_timer\x00" as *const u8 as *const libc::c_char);
  read_file(pathbuf.as_mut_ptr());
  show_bridge_timer(b"\n hello timer\t\t\x00" as *const u8 as *const libc::c_char);
  strcpy(sfx, b"tcn_timer\x00" as *const u8 as *const libc::c_char);
  read_file(pathbuf.as_mut_ptr());
  show_bridge_timer(b"\t\t\ttcn timer\t\t\x00" as *const u8 as *const libc::c_char);
  strcpy(
    sfx,
    b"topology_change_timer\x00" as *const u8 as *const libc::c_char,
  );
  read_file(pathbuf.as_mut_ptr());
  show_bridge_timer(b"\n topology change timer\t\x00" as *const u8 as *const libc::c_char);
  strcpy(sfx, b"gc_timer\x00" as *const u8 as *const libc::c_char);
  read_file(pathbuf.as_mut_ptr());
  show_bridge_timer(b"\t\t\tgc timer\t\t\x00" as *const u8 as *const libc::c_char);
  printf(b"\n flags\t\t\t\x00" as *const u8 as *const libc::c_char);
  strcpy(
    sfx,
    b"topology_change\x00" as *const u8 as *const libc::c_char,
  );
  read_file(pathbuf.as_mut_ptr());
  if !(*bb_common_bufsiz1.as_mut_ptr().offset(0) as libc::c_int == '0' as i32
    && *bb_common_bufsiz1.as_mut_ptr().offset(1) == 0)
  {
    printf(b"TOPOLOGY_CHANGE \x00" as *const u8 as *const libc::c_char);
  }
  strcpy(
    sfx,
    b"topology_change_detected\x00" as *const u8 as *const libc::c_char,
  );
  read_file(pathbuf.as_mut_ptr());
  if !(*bb_common_bufsiz1.as_mut_ptr().offset(0) as libc::c_int == '0' as i32
    && *bb_common_bufsiz1.as_mut_ptr().offset(1) == 0)
  {
    printf(b"TOPOLOGY_CHANGE_DETECTED \x00" as *const u8 as *const libc::c_char);
  }
  printf(b"\n\n\n\x00" as *const u8 as *const libc::c_char);
  /* Show bridge ports */
  let mut ifaces: *mut DIR = 0 as *mut DIR;
  /* sfx points past "BR/bridge/", turn it into "BR/brif": */
  *sfx.offset(-4i32 as isize) = 'f' as i32 as libc::c_char; /* . or .. */
  *sfx.offset(-3i32 as isize) = '\u{0}' as i32 as libc::c_char;
  ifaces = opendir(pathbuf.as_mut_ptr());
  if !ifaces.is_null() {
    let mut ent: *mut dirent = 0 as *mut dirent;
    loop {
      ent = readdir(ifaces);
      if ent.is_null() {
        break;
      }
      if (*ent).d_name[0] as libc::c_int == '.' as i32
        && ((*ent).d_name[1] == 0
          || (*ent).d_name[1] as libc::c_int == '.' as i32 && (*ent).d_name[2] == 0)
      {
        continue;
      }
      show_bridge_port((*ent).d_name.as_mut_ptr());
    }
  };
}
#[no_mangle]
pub unsafe extern "C" fn brctl_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  static mut keywords: [libc::c_char; 124] = [
    97, 100, 100, 98, 114, 0, 100, 101, 108, 98, 114, 0, 97, 100, 100, 105, 102, 0, 100, 101, 108,
    105, 102, 0, 115, 116, 112, 0, 115, 104, 111, 119, 115, 116, 112, 0, 115, 101, 116, 97, 103,
    101, 105, 110, 103, 0, 115, 101, 116, 102, 100, 0, 115, 101, 116, 104, 101, 108, 108, 111, 0,
    115, 101, 116, 109, 97, 120, 97, 103, 101, 0, 115, 101, 116, 112, 97, 116, 104, 99, 111, 115,
    116, 0, 115, 101, 116, 112, 111, 114, 116, 112, 114, 105, 111, 0, 115, 101, 116, 98, 114, 105,
    100, 103, 101, 112, 114, 105, 111, 0, 115, 104, 111, 119, 109, 97, 99, 115, 0, 115, 104, 111,
    119, 0, 0,
  ];
  let mut key: libc::c_int = 0;
  let mut br: *mut libc::c_char = 0 as *mut libc::c_char;
  argv = argv.offset(1);
  if (*argv).is_null() {
    /* bare "brctl" shows --help */
    bb_show_usage();
  }
  xchdir(b"/sys/class/net\x00" as *const u8 as *const libc::c_char);
  key = index_in_strings(keywords.as_ptr(), *argv);
  if key == -1i32 {
    /* no match found in keywords array, bail out. */
    bb_error_msg_and_die(bb_msg_invalid_arg_to.as_ptr(), *argv, applet_name);
  }
  argv = argv.offset(1);
  if key == ARG_show as libc::c_int {
    /* show [BR]... */
    let mut net: *mut DIR = 0 as *mut DIR;
    let mut ent: *mut dirent = 0 as *mut dirent;
    let mut need_hdr: libc::c_int = 1i32;
    let mut exitcode: libc::c_int = 0i32;
    if !(*argv).is_null() {
      loop
      /* "show BR1 BR2 BR3" */
      {
        if show_bridge(*argv, need_hdr) >= 0i32 {
          need_hdr = 0i32
        } else {
          bb_error_msg(
            b"bridge %s does not exist\x00" as *const u8 as *const libc::c_char,
            *argv,
          );
          //TODO: if device exists, but is not a BR, brctl from bridge-utils 1.6
          //says this instead: "device eth0 is not a bridge"
          exitcode = 1i32
        }
        argv = argv.offset(1);
        if (*argv).is_null() {
          break;
        }
      }
      return exitcode;
    }
    /* "show" (if no ifaces, shows nothing, not even header) */
    net = xopendir(b".\x00" as *const u8 as *const libc::c_char); /* . or .. */
    loop {
      ent = readdir(net);
      if ent.is_null() {
        break;
      }
      if (*ent).d_name[0] as libc::c_int == '.' as i32
        && ((*ent).d_name[1] == 0
          || (*ent).d_name[1] as libc::c_int == '.' as i32 && (*ent).d_name[2] == 0)
      {
        continue;
      }
      if show_bridge((*ent).d_name.as_mut_ptr(), need_hdr) >= 0i32 {
        need_hdr = 0i32
      }
    }
    return exitcode;
  }
  if (*argv).is_null() {
    /* All of the below need at least one argument */
    bb_show_usage();
  }
  let fresh0 = argv;
  argv = argv.offset(1);
  br = *fresh0;
  if key == ARG_addbr as libc::c_int || key == ARG_delbr as libc::c_int {
    /* brctl from bridge-utils 1.6 still uses ioctl
     * for SIOCBRADDBR / SIOCBRDELBR, not /sys accesses
     */
    let mut fd: libc::c_int = xsocket(2i32, SOCK_STREAM as libc::c_int, 0i32);
    ioctl_or_perror_and_die(
      fd,
      if key == ARG_addbr as libc::c_int {
        0x89a0i32
      } else {
        0x89a1i32
      } as libc::c_uint,
      br as *mut libc::c_void,
      b"bridge %s\x00" as *const u8 as *const libc::c_char,
      br,
    );
    //close(fd);
    //goto done;
    /* bridge-utils 1.6 simply ignores trailing args:
     * "brctl addbr BR1 ARGS" ignores ARGS
     */
    return 0i32;
  }
  if key == ARG_showmacs as libc::c_int {
    show_bridge_macs(br);
    return 0i32;
  }
  if key == ARG_showstp as libc::c_int {
    show_bridge_stp(br);
    return 0i32;
  }
  if (*argv).is_null() {
    /* All of the below need at least two arguments */
    bb_show_usage(); /* 4 .. 7 */
  }
  if key == ARG_stp as libc::c_int {
    static mut no_yes: [libc::c_char; 23] = [
      48, 0, 111, 102, 102, 0, 110, 0, 110, 111, 0, 49, 0, 111, 110, 0, 121, 0, 121, 101, 115, 0, 0,
    ];
    let mut onoff: libc::c_int = index_in_strings(no_yes.as_ptr(), *argv);
    if onoff < 0i32 {
      bb_error_msg_and_die(bb_msg_invalid_arg_to.as_ptr(), *argv, applet_name);
    }
    onoff = (onoff as libc::c_uint).wrapping_div(4i32 as libc::c_uint) as libc::c_int;
    write_uint(
      br,
      b"bridge/stp_state\x00" as *const u8 as *const libc::c_char,
      onoff as libc::c_uint,
    );
    return 0i32;
  }
  if ((key - ARG_setageing as libc::c_int) as libc::c_uint) < 4i32 as libc::c_uint {
    /* time related ops */
    /* setageing BR N: "N*100\n" to /sys/class/net/BR/bridge/ageing_time
     * setfd BR N:     "N*100\n" to /sys/class/net/BR/bridge/forward_delay
     * sethello BR N:  "N*100\n" to /sys/class/net/BR/bridge/hello_time
     * setmaxage BR N: "N*100\n" to /sys/class/net/BR/bridge/max_age
     */
    write_uint(
      br,
      nth_string(
        b"bridge/ageing_time\x00bridge/forward_delay\x00bridge/hello_time\x00bridge/max_age\x00"
          as *const u8 as *const libc::c_char,
        key - ARG_setageing as libc::c_int,
      ),
      str_to_jiffies(*argv),
    );
    return 0i32;
  }
  if key == ARG_setbridgeprio as libc::c_int {
    write_uint(
      br,
      b"bridge/priority\x00" as *const u8 as *const libc::c_char,
      xatoi_positive(*argv) as libc::c_uint,
    );
    return 0i32;
  }
  if key == ARG_setpathcost as libc::c_int || key == ARG_setportprio as libc::c_int {
    if (*argv.offset(1)).is_null() {
      bb_show_usage();
    }
    /* BR is not used (and ignored!) for these commands:
     * "setpathcost BR PORT N" writes "N\n" to
     * /sys/class/net/PORT/brport/path_cost
     * "setportprio BR PORT N" writes "N\n" to
     * /sys/class/net/PORT/brport/priority
     */
    write_uint(
      *argv.offset(0),
      nth_string(
        b"brport/path_cost\x00brport/priority\x00" as *const u8 as *const libc::c_char,
        key - ARG_setpathcost as libc::c_int,
      ),
      xatoi_positive(*argv.offset(1)) as libc::c_uint,
    );
    return 0i32;
  }
  /* always true: if (key == ARG_addif || key == ARG_delif) */
  let mut ifr: ifreq = ifreq {
    ifr_ifrn: C2RustUnnamed_1 { ifrn_name: [0; 16] },
    ifr_ifru: C2RustUnnamed_0 {
      ifru_addr: sockaddr {
        sa_family: 0,
        sa_data: [0; 14],
      },
    },
  };
  let mut fd_0: libc::c_int = xsocket(2i32, SOCK_STREAM as libc::c_int, 0i32);
  strncpy_IFNAMSIZ(ifr.ifr_ifrn.ifrn_name.as_mut_ptr(), br);
  ifr.ifr_ifru.ifru_ivalue = if_nametoindex(*argv) as libc::c_int;
  if ifr.ifr_ifru.ifru_ivalue == 0i32 {
    bb_perror_msg_and_die(b"iface %s\x00" as *const u8 as *const libc::c_char, *argv);
  }
  ioctl_or_perror_and_die(
    fd_0,
    if key == ARG_addif as libc::c_int {
      0x89a2i32
    } else {
      0x89a3i32
    } as libc::c_uint,
    &mut ifr as *mut ifreq as *mut libc::c_void,
    b"bridge %s\x00" as *const u8 as *const libc::c_char,
    br,
  );
  return 0i32;
}
