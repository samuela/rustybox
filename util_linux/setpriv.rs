use libc;
use libc::chdir;
use libc::chmod;
use libc::chown;
use libc::closelog;
use libc::dup2;
use libc::fstat;
use libc::getenv;
use libc::geteuid;
use libc::getopt;
use libc::getpid;
use libc::isatty;
use libc::kill;
use libc::openlog;
use libc::sigaddset;
use libc::sigemptyset;
use libc::sigprocmask;
use libc::sleep;
use libc::sscanf;
use libc::strcasecmp;
use libc::strcpy;
use libc::symlink;
use libc::syscall;
use libc::syslog;
use libc::time;
use libc::access;
use libc::atoi;
use libc::fclose;
use libc::fprintf;
use libc::lstat;
use libc::printf;
use libc::puts;
use libc::rename;
use libc::rmdir;
use libc::sprintf;
use libc::strchr;
use libc::strcmp;
use libc::strrchr;
use libc::strstr;
use libc::system;





extern "C" {
  /*
   * setpriv implementation for busybox based on linux-utils-ng 2.29
   *
   * Copyright (C) 2017 by  <assafgordon@gmail.com>
   *
   * Licensed under GPLv2 or later, see file LICENSE in this source tree.
   */
  //config:config SETPRIV
  //config:	bool "setpriv (6.6 kb)"
  //config:	default y
  //config:	select PLATFORM_LINUX
  //config:	select LONG_OPTS
  //config:	help
  //config:	Run a program with different Linux privilege settings.
  //config:	Requires kernel >= 3.5
  //config:
  //config:config FEATURE_SETPRIV_DUMP
  //config:	bool "Support dumping current privilege state"
  //config:	default y
  //config:	depends on SETPRIV
  //config:	help
  //config:	Enables the "--dump" switch to print out the current privilege
  //config:	state. This is helpful for diagnosing problems.
  //config:
  //config:config FEATURE_SETPRIV_CAPABILITIES
  //config:	bool "Support capabilities"
  //config:	default y
  //config:	depends on SETPRIV
  //config:	help
  //config:	Capabilities can be used to grant processes additional rights
  //config:	without the necessity to always execute as the root user.
  //config:	Enabling this option enables "--dump" to show information on
  //config:	capabilities.
  //config:
  //config:config FEATURE_SETPRIV_CAPABILITY_NAMES
  //config:	bool "Support capability names"
  //config:	default y
  //config:	depends on SETPRIV && FEATURE_SETPRIV_CAPABILITIES
  //config:	help
  //config:	Capabilities can be either referenced via a human-readble name,
  //config:	e.g. "net_admin", or using their index, e.g. "cap_12". Enabling
  //config:	this option allows using the human-readable names in addition to
  //config:	the index-based names.
  //applet:IF_SETPRIV(APPLET(setpriv, BB_DIR_BIN, BB_SUID_DROP))
  //kbuild:lib-$(CONFIG_SETPRIV) += setpriv.o
  //usage:#define setpriv_trivial_usage
  //usage:	"[OPTIONS] PROG [ARGS]"
  //usage:#define setpriv_full_usage "\n\n"
  //usage:       "Run PROG with different privilege settings\n"
  //usage:	IF_FEATURE_SETPRIV_DUMP(
  //usage:     "\n-d,--dump		Show current capabilities"
  //usage:	)
  //usage:     "\n--nnp,--no-new-privs	Ignore setuid/setgid bits and file capabilities"
  //usage:	IF_FEATURE_SETPRIV_CAPABILITIES(
  //usage:     "\n--inh-caps CAP,CAP	Set inheritable capabilities"
  //usage:     "\n--ambient-caps CAP,CAP	Set ambient capabilities"
  //usage:	)
  //setpriv from util-linux 2.28:
  // -d, --dump               show current state (and do not exec anything)
  // --nnp, --no-new-privs    disallow granting new privileges
  // --inh-caps <caps,...>    set inheritable capabilities
  // --bounding-set <caps>    set capability bounding set
  // --ruid <uid>             set real uid
  // --euid <uid>             set effective uid
  // --rgid <gid>             set real gid
  // --egid <gid>             set effective gid
  // --reuid <uid>            set real and effective uid
  // --regid <gid>            set real and effective gid
  // --clear-groups           clear supplementary groups
  // --keep-groups            keep supplementary groups
  // --groups <group,...>     set supplementary groups
  // --securebits <bits>      set securebits
  // --selinux-label <label>  set SELinux label
  // --apparmor-profile <pr>  set AppArmor profile
  // #include <sys/capability.h>
  // This header is in libcap, but the functions are in libc.
  // Comment in the header says this above capset/capget:
  /* system calls - look to libc for function to system call mapping */

  #[no_mangle]
  fn capset(header: cap_user_header_t, data: cap_user_data_t) -> libc::c_int;

  #[no_mangle]
  fn prctl(__option: libc::c_int, _: ...) -> libc::c_int;



  #[no_mangle]
  fn getresuid(__ruid: *mut uid_t, __euid: *mut uid_t, __suid: *mut uid_t) -> libc::c_int;

  #[no_mangle]
  fn getresgid(__rgid: *mut gid_t, __egid: *mut gid_t, __sgid: *mut gid_t) -> libc::c_int;

  #[no_mangle]
  static mut optind: libc::c_int;



  #[no_mangle]
  fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;

  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn bb_getgroups(ngroups: *mut libc::c_int, group_array: *mut gid_t) -> *mut gid_t;

  #[no_mangle]
  fn BB_EXECVP_or_die(argv: *mut *mut libc::c_char) -> !;

  #[no_mangle]
  fn getopt32long(
    argv: *mut *mut libc::c_char,
    optstring: *const libc::c_char,
    longopts: *const libc::c_char,
    _: ...
  ) -> u32;

  #[no_mangle]
  fn bb_show_usage() -> !;

  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;

  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;

  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);

  #[no_mangle]
  fn bb_perror_msg_and_die(s: *const libc::c_char, _: ...) -> !;

  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;

  #[no_mangle]
  fn cap_name_to_number(cap: *const libc::c_char) -> libc::c_uint;

  #[no_mangle]
  fn printf_cap(pfx: *const libc::c_char, cap_no: libc::c_uint);

  #[no_mangle]
  fn getcaps(caps: *mut libc::c_void);
}

pub type u32 = libc::c_uint;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __user_cap_header_struct {
  pub version: u32,
  pub pid: libc::c_int,
}

pub type cap_user_header_t = *mut __user_cap_header_struct;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __user_cap_data_struct {
  pub effective: u32,
  pub permitted: u32,
  pub inheritable: u32,
}

pub type cap_user_data_t = *mut __user_cap_data_struct;


use libc::gid_t;
use libc::uid_t;

// so for bbox, let's just repeat the declarations.
// This way, libcap needs not be installed in build environment.

pub type C2RustUnnamed = libc::c_uint;
pub const OPT_NNP: C2RustUnnamed = 8;
pub const OPT_AMB: C2RustUnnamed = 4;
pub const OPT_INH: C2RustUnnamed = 2;
pub const OPT_DUMP: C2RustUnnamed = 1;
// pub const OPTBIT_NNP: C2RustUnnamed = 3;
// pub const OPTBIT_AMB: C2RustUnnamed = 2;
// pub const OPTBIT_INH: C2RustUnnamed = 1;
// pub const OPTBIT_DUMP: C2RustUnnamed = 0;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct caps {
  pub header: __user_cap_header_struct,
  pub u32s: libc::c_uint,
  pub data: [__user_cap_data_struct; 2],
}
unsafe extern "C" fn parse_cap(mut cap: *const libc::c_char) -> libc::c_uint {
  match *cap.offset(0) as libc::c_int {
    45 | 43 => {}
    _ => {
      bb_error_msg_and_die(
        b"invalid capability \'%s\'\x00" as *const u8 as *const libc::c_char,
        cap,
      );
    }
  }
  cap = cap.offset(1);
  return cap_name_to_number(cap);
}
unsafe extern "C" fn set_inh_caps(mut capstring: *mut libc::c_char) {
  let mut caps: caps = caps {
    header: __user_cap_header_struct { version: 0, pid: 0 },
    u32s: 0,
    data: [__user_cap_data_struct {
      effective: 0,
      permitted: 0,
      inheritable: 0,
    }; 2],
  };
  getcaps(&mut caps as *mut caps as *mut libc::c_void);
  capstring = strtok(capstring, b",\x00" as *const u8 as *const libc::c_char);
  while !capstring.is_null() {
    let mut cap: libc::c_uint = 0;
    cap = parse_cap(capstring);
    if cap >> 5i32 >= caps.u32s {
      bb_error_msg_and_die(
        b"invalid capability \'%s\'\x00" as *const u8 as *const libc::c_char,
        capstring,
      );
    }
    if *capstring.offset(0) as libc::c_int == '+' as i32 {
      caps.data[(cap >> 5i32) as usize].inheritable |=
        (1i32 << (cap & 31i32 as libc::c_uint)) as libc::c_uint
    } else {
      caps.data[(cap >> 5i32) as usize].inheritable &=
        !(1i32 << (cap & 31i32 as libc::c_uint)) as libc::c_uint
    }
    capstring = strtok(
      0 as *mut libc::c_char,
      b",\x00" as *const u8 as *const libc::c_char,
    )
  }
  if capset(&mut caps.header, caps.data.as_mut_ptr()) != 0i32 {
    bb_simple_perror_msg_and_die(b"capset\x00" as *const u8 as *const libc::c_char);
  };
}
unsafe extern "C" fn set_ambient_caps(mut string: *mut libc::c_char) {
  let mut cap: *mut libc::c_char = 0 as *mut libc::c_char;
  cap = strtok(string, b",\x00" as *const u8 as *const libc::c_char);
  while !cap.is_null() {
    let mut idx: libc::c_uint = 0;
    idx = parse_cap(cap);
    if *cap.offset(0) as libc::c_int == '+' as i32 {
      if prctl(47i32, 2i32, idx, 0i32, 0i32) < 0i32 {
        bb_simple_perror_msg(b"cap_ambient_raise\x00" as *const u8 as *const libc::c_char);
      }
    } else if prctl(47i32, 3i32, idx, 0i32, 0i32) < 0i32 {
      bb_simple_perror_msg(b"cap_ambient_lower\x00" as *const u8 as *const libc::c_char);
    }
    cap = strtok(
      0 as *mut libc::c_char,
      b",\x00" as *const u8 as *const libc::c_char,
    )
  }
}
/* FEATURE_SETPRIV_CAPABILITIES */
unsafe extern "C" fn dump() -> libc::c_int {
  let mut caps: caps = caps {
    header: __user_cap_header_struct { version: 0, pid: 0 },
    u32s: 0,
    data: [__user_cap_data_struct {
      effective: 0,
      permitted: 0,
      inheritable: 0,
    }; 2],
  }; /* never fails in Linux */
  let mut fmt: *const libc::c_char = 0 as *const libc::c_char; /* never fails in Linux */
  let mut ruid: uid_t = 0; /* never fails in Linux */
  let mut euid: uid_t = 0;
  let mut suid: uid_t = 0;
  let mut rgid: gid_t = 0;
  let mut egid: gid_t = 0;
  let mut sgid: gid_t = 0;
  let mut gids: *mut gid_t = 0 as *mut gid_t;
  let mut i: libc::c_int = 0;
  let mut ngids: libc::c_int = 0;
  let mut nnp: libc::c_int = 0;
  getresuid(&mut ruid, &mut euid, &mut suid);
  getresgid(&mut rgid, &mut egid, &mut sgid);
  ngids = 0i32;
  gids = bb_getgroups(&mut ngids, 0 as *mut gid_t);
  nnp = prctl(39i32, 0i32, 0i32, 0i32, 0i32);
  if nnp < 0i32 {
    bb_perror_msg_and_die(
      b"prctl: %s\x00" as *const u8 as *const libc::c_char,
      b"GET_NO_NEW_PRIVS\x00" as *const u8 as *const libc::c_char,
    );
  }
  printf(b"uid: %u\n\x00" as *const u8 as *const libc::c_char, ruid);
  printf(b"euid: %u\n\x00" as *const u8 as *const libc::c_char, euid);
  printf(b"gid: %u\n\x00" as *const u8 as *const libc::c_char, rgid);
  printf(b"egid: %u\n\x00" as *const u8 as *const libc::c_char, egid);
  printf(b"Supplementary groups: \x00" as *const u8 as *const libc::c_char);
  if ngids == 0i32 {
    printf(b"[none]\x00" as *const u8 as *const libc::c_char);
  } else {
    fmt = (b",%u\x00" as *const u8 as *const libc::c_char).offset(1);
    i = 0i32;
    while i < ngids {
      printf(fmt, *gids.offset(i as isize));
      fmt = b",%u\x00" as *const u8 as *const libc::c_char;
      i += 1
    }
  }
  printf(
    b"\nno_new_privs: %d\n\x00" as *const u8 as *const libc::c_char,
    nnp,
  );
  getcaps(&mut caps as *mut caps as *mut libc::c_void);
  printf(b"Inheritable capabilities: \x00" as *const u8 as *const libc::c_char);
  fmt = b"\x00" as *const u8 as *const libc::c_char;
  i = 0i32;
  while i >= 0i32 && i <= 37i32 {
    let mut idx: libc::c_uint = (i >> 5i32) as libc::c_uint;
    if idx >= caps.u32s {
      printf(
        b"\nindex: %u u32s: %u capability: %u\n\x00" as *const u8 as *const libc::c_char,
        idx,
        caps.u32s,
        i,
      );
      bb_simple_error_msg_and_die(
        b"unsupported capability\x00" as *const u8 as *const libc::c_char,
      );
    }
    if caps.data[idx as usize].inheritable & (1i32 << (i & 31i32)) as libc::c_uint != 0 {
      printf_cap(fmt, i as libc::c_uint);
      fmt = b",\x00" as *const u8 as *const libc::c_char
    }
    i += 1
  }
  if *fmt.offset(0) == 0 {
    printf(b"[none]\x00" as *const u8 as *const libc::c_char);
  }
  printf(b"\nAmbient capabilities: \x00" as *const u8 as *const libc::c_char);
  fmt = b"\x00" as *const u8 as *const libc::c_char;
  i = 0i32;
  while i >= 0i32 && i <= 37i32 {
    let mut ret: libc::c_int = prctl(47i32, 1i32, i as libc::c_ulong, 0u64, 0u64);
    if ret < 0i32 {
      bb_perror_msg_and_die(
        b"prctl: %s\x00" as *const u8 as *const libc::c_char,
        b"CAP_AMBIENT_IS_SET\x00" as *const u8 as *const libc::c_char,
      );
    }
    if ret != 0 {
      printf_cap(fmt, i as libc::c_uint);
      fmt = b",\x00" as *const u8 as *const libc::c_char
    }
    i += 1
  }
  if i == 0i32 {
    printf(b"[unsupported]\x00" as *const u8 as *const libc::c_char);
  } else if *fmt.offset(0) == 0 {
    printf(b"[none]\x00" as *const u8 as *const libc::c_char);
  }
  printf(b"\nCapability bounding set: \x00" as *const u8 as *const libc::c_char);
  fmt = b"\x00" as *const u8 as *const libc::c_char;
  i = 0i32;
  while i >= 0i32 && i <= 37i32 {
    let mut ret_0: libc::c_int = prctl(23i32, i as libc::c_ulong, 0u64, 0u64, 0u64);
    if ret_0 < 0i32 {
      bb_perror_msg_and_die(
        b"prctl: %s\x00" as *const u8 as *const libc::c_char,
        b"CAPBSET_READ\x00" as *const u8 as *const libc::c_char,
      );
    }
    if ret_0 != 0 {
      printf_cap(fmt, i as libc::c_uint);
      fmt = b",\x00" as *const u8 as *const libc::c_char
    }
    i += 1
  }
  if *fmt.offset(0) == 0 {
    printf(b"[none]\x00" as *const u8 as *const libc::c_char);
  }
  bb_putchar('\n' as i32);
  return 0i32;
}
/* FEATURE_SETPRIV_DUMP */
#[no_mangle]
pub unsafe extern "C" fn setpriv_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  static mut setpriv_longopts: [libc::c_char; 55] = [
    100, 117, 109, 112, 0, 0, 100, 110, 110, 112, 0, 0, -1, 110, 111, 45, 110, 101, 119, 45, 112,
    114, 105, 118, 115, 0, 0, -1, 105, 110, 104, 45, 99, 97, 112, 115, 0, 1, -2, 97, 109, 98, 105,
    101, 110, 116, 45, 99, 97, 112, 115, 0, 1, -3, 0,
  ];
  let mut opts: libc::c_int = 0;
  let mut inh_caps: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut ambient_caps: *mut libc::c_char = 0 as *mut libc::c_char;
  opts = getopt32long(
    argv,
    b"+d\xfe:\xfd:\x00" as *const u8 as *const libc::c_char,
    setpriv_longopts.as_ptr(),
    &mut inh_caps as *mut *mut libc::c_char,
    &mut ambient_caps as *mut *mut libc::c_char,
  ) as libc::c_int;
  argv = argv.offset(optind as isize);
  if opts & OPT_DUMP as libc::c_int != 0 {
    if !(*argv.offset(0)).is_null() || opts - OPT_DUMP as libc::c_int != 0i32 {
      bb_show_usage();
    }
    return dump();
  }
  if opts & OPT_NNP as libc::c_int != 0 {
    if prctl(38i32, 1i32, 0i32, 0i32, 0i32) != 0 {
      bb_perror_msg_and_die(
        b"prctl: %s\x00" as *const u8 as *const libc::c_char,
        b"SET_NO_NEW_PRIVS\x00" as *const u8 as *const libc::c_char,
      );
    }
  }
  if opts & OPT_INH as libc::c_int != 0 {
    set_inh_caps(inh_caps);
  }
  if opts & OPT_AMB as libc::c_int != 0 {
    set_ambient_caps(ambient_caps);
  }
  if (*argv.offset(0)).is_null() {
    bb_show_usage();
  }
  BB_EXECVP_or_die(argv);
}
