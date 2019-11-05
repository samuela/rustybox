use libc;
extern "C" {

  #[no_mangle]
  fn setns(__fd: libc::c_int, __nstype: libc::c_int) -> libc::c_int;

  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  fn setgroups(__n: size_t, __groups: *const __gid_t) -> libc::c_int;

  #[no_mangle]
  fn xsetgid(gid: gid_t);

  #[no_mangle]
  fn xsetuid(uid: uid_t);

  #[no_mangle]
  fn xfchdir(fd: libc::c_int);

  #[no_mangle]
  fn xchroot(path: *const libc::c_char);

  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn exec_prog_or_SHELL(argv: *mut *mut libc::c_char) -> !;

  #[no_mangle]
  fn xvfork_parent_waits_and_exits();

  #[no_mangle]
  fn getopt32long(
    argv: *mut *mut libc::c_char,
    optstring: *const libc::c_char,
    longopts: *const libc::c_char,
    _: ...
  ) -> uint32_t;

  #[no_mangle]
  fn bb_show_usage() -> !;

  #[no_mangle]
  fn bb_perror_msg_and_die(s: *const libc::c_char, _: ...) -> !;

  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;

  #[no_mangle]
  fn snprintf(
    _: *mut libc::c_char,
    _: libc::c_ulong,
    _: *const libc::c_char,
    _: ...
  ) -> libc::c_int;
}


pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;

use crate::librb::gid_t;
use crate::librb::pid_t;
use crate::librb::size_t;
use crate::librb::uid_t;
use crate::librb::uint32_t;

/*
 * Mini nsenter implementation for busybox.
 *
 * Copyright (C) 2016 by Bartosz Golaszewski <bartekgola@gmail.com>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config NSENTER
//config:	bool "nsenter (6.5 kb)"
//config:	default y
//config:	select PLATFORM_LINUX
//config:	help
//config:	Run program with namespaces of other processes.
//applet:IF_NSENTER(APPLET(nsenter, BB_DIR_USR_BIN, BB_SUID_DROP))
//kbuild:lib-$(CONFIG_NSENTER) += nsenter.o
//usage:#define nsenter_trivial_usage
//usage:       "[OPTIONS] [PROG [ARGS]]"
//usage:#define nsenter_full_usage "\n"
//usage:     "\n	-t PID		Target process to get namespaces from"
//usage:     "\n	-m[FILE]	Enter mount namespace"
//usage:     "\n	-u[FILE]	Enter UTS namespace (hostname etc)"
//usage:     "\n	-i[FILE]	Enter System V IPC namespace"
//usage:     "\n	-n[FILE]	Enter network namespace"
//usage:     "\n	-p[FILE]	Enter pid namespace"
//usage:     "\n	-U[FILE]	Enter user namespace"
//usage:     "\n	-S UID		Set uid in entered namespace"
//usage:     "\n	-G GID		Set gid in entered namespace"
//usage:	IF_LONG_OPTS(
//usage:     "\n	--preserve-credentials	Don't touch uids or gids"
//usage:	)
//usage:     "\n	-r[DIR]		Set root directory"
//usage:     "\n	-w[DIR]		Set working directory"
//usage:     "\n	-F		Don't fork before exec'ing PROG"
#[derive(Copy, Clone)]
#[repr(C)]
pub struct namespace_descr {
  pub flag: libc::c_int,
  pub ns_nsfile8: [libc::c_char; 8],
  /* "ns/" + namespace file in process' procfs entry */
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct namespace_ctx {
  pub path: *mut libc::c_char,
  pub fd: libc::c_int,
  /* opened namespace file descriptor */
}

pub type C2RustUnnamed = libc::c_uint;
pub const OPT_prescred: C2RustUnnamed = 4096;
pub const OPT_nofork: C2RustUnnamed = 2048;
pub const OPT_wd: C2RustUnnamed = 1024;
pub const OPT_root: C2RustUnnamed = 512;
pub const OPT_setgid: C2RustUnnamed = 256;
pub const OPT_setuid: C2RustUnnamed = 128;
// pub const OPT_target: C2RustUnnamed = 64;
// pub const OPT_mount: C2RustUnnamed = 32;
pub const OPT_pid: C2RustUnnamed = 16;
// pub const OPT_network: C2RustUnnamed = 8;
// pub const OPT_uts: C2RustUnnamed = 4;
// pub const OPT_ipc: C2RustUnnamed = 2;
pub const OPT_user: C2RustUnnamed = 1;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const NS_COUNT: C2RustUnnamed_0 = 6;
pub const NS_MNT_POS: C2RustUnnamed_0 = 5;
pub const NS_PID_POS: C2RustUnnamed_0 = 4;
pub const NS_NET_POS: C2RustUnnamed_0 = 3;
pub const NS_UTS_POS: C2RustUnnamed_0 = 2;
pub const NS_IPC_POS: C2RustUnnamed_0 = 1;
pub const NS_USR_POS: C2RustUnnamed_0 = 0;

/*
 * The order is significant in nsenter.
 * The user namespace comes first, so that it is entered first.
 * This gives an unprivileged user the potential to enter other namespaces.
 */
static mut ns_list: [namespace_descr; 6] = [
  {
    let mut init = namespace_descr {
      flag: 0x10000000i32,
      ns_nsfile8: [110, 115, 47, 117, 115, 101, 114, 0],
    };
    init
  },
  {
    let mut init = namespace_descr {
      flag: 0x8000000i32,
      ns_nsfile8: [110, 115, 47, 105, 112, 99, 0, 0],
    };
    init
  },
  {
    let mut init = namespace_descr {
      flag: 0x4000000i32,
      ns_nsfile8: [110, 115, 47, 117, 116, 115, 0, 0],
    };
    init
  },
  {
    let mut init = namespace_descr {
      flag: 0x40000000i32,
      ns_nsfile8: [110, 115, 47, 110, 101, 116, 0, 0],
    };
    init
  },
  {
    let mut init = namespace_descr {
      flag: 0x20000000i32,
      ns_nsfile8: [110, 115, 47, 112, 105, 100, 0, 0],
    };
    init
  },
  {
    let mut init = namespace_descr {
      flag: 0x20000i32,
      ns_nsfile8: [110, 115, 47, 109, 110, 116, 0, 0],
    };
    init
  },
];
/*
 * Upstream nsenter doesn't support the short option for --preserve-credentials
 */
static mut opt_str: [libc::c_char; 35] = [
  85, 58, 58, 105, 58, 58, 117, 58, 58, 110, 58, 58, 112, 58, 58, 109, 58, 58, 116, 58, 43, 83, 58,
  43, 71, 58, 43, 114, 58, 58, 119, 58, 58, 70, 0,
];
static mut nsenter_longopts: [libc::c_char; 112] = [
  117, 115, 101, 114, 0, 2, 85, 105, 112, 99, 0, 2, 105, 117, 116, 115, 0, 2, 117, 110, 101, 116,
  0, 2, 110, 112, 105, 100, 0, 2, 112, 109, 111, 117, 110, 116, 0, 2, 109, 116, 97, 114, 103, 101,
  116, 0, 1, 116, 115, 101, 116, 117, 105, 100, 0, 1, 83, 115, 101, 116, 103, 105, 100, 0, 1, 71,
  114, 111, 111, 116, 0, 2, 114, 119, 100, 0, 2, 119, 110, 111, 45, 102, 111, 114, 107, 0, 0, 70,
  112, 114, 101, 115, 101, 114, 118, 101, 45, 99, 114, 101, 100, 101, 110, 116, 105, 97, 108, 115,
  0, 0, -1, 0,
];
/*
 * Open a file and return the new descriptor. If a full path is provided in
 * fs_path, then the file to which it points is opened. Otherwise (fd_path is
 * NULL) the routine builds a path to a procfs file using the following
 * template: '/proc/<target_pid>/<target_file>'.
 */
unsafe extern "C" fn open_by_path_or_target(
  mut path: *const libc::c_char,
  mut target_pid: pid_t,
  mut target_file: *const libc::c_char,
) -> libc::c_int {
  let mut proc_path_buf: [libc::c_char; 32] = [0; 32];
  if path.is_null() {
    if target_pid == 0i32 {
      /* Example:
       * "nsenter -p PROG" - neither -pFILE nor -tPID given.
       */
      bb_show_usage();
    }
    snprintf(
      proc_path_buf.as_mut_ptr(),
      ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
      b"/proc/%u/%s\x00" as *const u8 as *const libc::c_char,
      target_pid as libc::c_uint,
      target_file,
    );
    path = proc_path_buf.as_mut_ptr()
  }
  return xopen(path, 0i32);
}

#[no_mangle]
pub unsafe extern "C" fn nsenter_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut i: libc::c_int = 0;
  let mut opts: libc::c_uint = 0;
  let mut root_dir_str: *const libc::c_char = 0 as *const libc::c_char;
  let mut wd_str: *const libc::c_char = 0 as *const libc::c_char;
  let mut ns_ctx_list: [namespace_ctx; 6] = [namespace_ctx {
    path: 0 as *mut libc::c_char,
    fd: 0,
  }; 6];
  let mut setgroups_failed: libc::c_int = 0;
  let mut root_fd: libc::c_int = 0;
  let mut wd_fd: libc::c_int = 0;
  let mut target_pid: libc::c_int = 0i32;
  let mut uid: libc::c_int = 0i32;
  let mut gid: libc::c_int = 0i32;
  memset(
    ns_ctx_list.as_mut_ptr() as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<[namespace_ctx; 6]>() as libc::c_ulong,
  );
  opts = getopt32long(
    argv,
    opt_str.as_ptr(),
    nsenter_longopts.as_ptr(),
    &mut (*ns_ctx_list
      .as_mut_ptr()
      .offset(NS_USR_POS as libc::c_int as isize))
    .path as *mut *mut libc::c_char,
    &mut (*ns_ctx_list
      .as_mut_ptr()
      .offset(NS_IPC_POS as libc::c_int as isize))
    .path as *mut *mut libc::c_char,
    &mut (*ns_ctx_list
      .as_mut_ptr()
      .offset(NS_UTS_POS as libc::c_int as isize))
    .path as *mut *mut libc::c_char,
    &mut (*ns_ctx_list
      .as_mut_ptr()
      .offset(NS_NET_POS as libc::c_int as isize))
    .path as *mut *mut libc::c_char,
    &mut (*ns_ctx_list
      .as_mut_ptr()
      .offset(NS_PID_POS as libc::c_int as isize))
    .path as *mut *mut libc::c_char,
    &mut (*ns_ctx_list
      .as_mut_ptr()
      .offset(NS_MNT_POS as libc::c_int as isize))
    .path as *mut *mut libc::c_char,
    &mut target_pid as *mut libc::c_int,
    &mut uid as *mut libc::c_int,
    &mut gid as *mut libc::c_int,
    &mut root_dir_str as *mut *const libc::c_char,
    &mut wd_str as *mut *const libc::c_char,
  );
  argv = argv.offset(optind as isize);
  wd_fd = -1i32;
  root_fd = wd_fd;
  if opts & OPT_root as libc::c_int as libc::c_uint != 0 {
    root_fd = open_by_path_or_target(
      root_dir_str,
      target_pid,
      b"root\x00" as *const u8 as *const libc::c_char,
    )
  }
  if opts & OPT_wd as libc::c_int as libc::c_uint != 0 {
    wd_fd = open_by_path_or_target(
      wd_str,
      target_pid,
      b"cwd\x00" as *const u8 as *const libc::c_char,
    )
  }
  i = 0i32;
  while i < NS_COUNT as libc::c_int {
    let mut ns: *const namespace_descr =
      &*ns_list.as_ptr().offset(i as isize) as *const namespace_descr;
    let mut ns_ctx: *mut namespace_ctx =
      &mut *ns_ctx_list.as_mut_ptr().offset(i as isize) as *mut namespace_ctx;
    (*ns_ctx).fd = -1i32;
    if opts & (1i32 << i) as libc::c_uint != 0 {
      (*ns_ctx).fd = open_by_path_or_target((*ns_ctx).path, target_pid, (*ns).ns_nsfile8.as_ptr())
    }
    i += 1
  }
  /*
   * Entering the user namespace without --preserve-credentials implies
   * --setuid & --setgid and clearing root's groups.
   */
  setgroups_failed = 0i32;
  if opts & OPT_user as libc::c_int as libc::c_uint != 0
    && opts & OPT_prescred as libc::c_int as libc::c_uint == 0
  {
    opts |= (OPT_setuid as libc::c_int | OPT_setgid as libc::c_int) as libc::c_uint;
    /*
     * We call setgroups() before and after setns() and only
     * bail-out if it fails twice.
     */
    setgroups_failed = (setgroups(0i32 as size_t, 0 as *const __gid_t) < 0i32) as libc::c_int
  }
  i = 0i32;
  while i < NS_COUNT as libc::c_int {
    let mut ns_0: *const namespace_descr =
      &*ns_list.as_ptr().offset(i as isize) as *const namespace_descr;
    let mut ns_ctx_0: *mut namespace_ctx =
      &mut *ns_ctx_list.as_mut_ptr().offset(i as isize) as *mut namespace_ctx;
    if !((*ns_ctx_0).fd < 0i32) {
      if setns((*ns_ctx_0).fd, (*ns_0).flag) != 0 {
        bb_perror_msg_and_die(
          b"setns(): can\'t reassociate to namespace \'%s\'\x00" as *const u8
            as *const libc::c_char,
          (*ns_0).ns_nsfile8.as_ptr().offset(3),
        );
      }
      close((*ns_ctx_0).fd);
    }
    i += 1
    /* should close fds, to not confuse exec'ed PROG */
    /*ns_ctx->fd = -1;*/
  }
  if root_fd >= 0i32 {
    if wd_fd < 0i32 {
      /*
       * Save the current working directory if we're not
       * changing it.
       */
      wd_fd = xopen(b".\x00" as *const u8 as *const libc::c_char, 0i32)
    }
    xfchdir(root_fd);
    xchroot(b".\x00" as *const u8 as *const libc::c_char);
    close(root_fd);
    /*root_fd = -1;*/
  }
  if wd_fd >= 0i32 {
    xfchdir(wd_fd);
    close(wd_fd);
    /*wd_fd = -1;*/
  }
  /*
   * Entering the pid namespace implies forking unless it's been
   * explicitly requested by the user not to.
   */
  if opts & OPT_nofork as libc::c_int as libc::c_uint == 0
    && opts & OPT_pid as libc::c_int as libc::c_uint != 0
  {
    xvfork_parent_waits_and_exits();
    /* Child continues */
  }
  if opts & OPT_setgid as libc::c_int as libc::c_uint != 0 {
    if setgroups(0i32 as size_t, 0 as *const __gid_t) < 0i32 && setgroups_failed != 0 {
      bb_simple_perror_msg_and_die(b"setgroups\x00" as *const u8 as *const libc::c_char);
    }
    xsetgid(gid as gid_t);
  }
  if opts & OPT_setuid as libc::c_int as libc::c_uint != 0 {
    xsetuid(uid as uid_t);
  }
  exec_prog_or_SHELL(argv);
}
