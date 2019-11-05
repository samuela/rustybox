use libc;

extern "C" {
  #[no_mangle]
  fn unshare(__flags: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn mount(
    __special_file: *const libc::c_char,
    __dir: *const libc::c_char,
    __fstype: *const libc::c_char,
    __rwflag: libc::c_ulong,
    __data: *const libc::c_void,
  ) -> libc::c_int;

  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;

  #[no_mangle]
  fn getegid() -> __gid_t;

  #[no_mangle]
  fn geteuid() -> __uid_t;

  #[no_mangle]
  fn getpid() -> __pid_t;

  #[no_mangle]
  fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;

  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn xpipe(filedes: *mut libc::c_int);

  #[no_mangle]
  fn xopen_xwrite_close(file: *const libc::c_char, str: *const libc::c_char);

  #[no_mangle]
  fn exec_prog_or_SHELL(argv: *mut *mut libc::c_char) -> !;

  /* xvfork() can't be a _function_, return after vfork in child mangles stack
   * in the parent. It must be a macro. */
  #[no_mangle]
  fn xfork() -> pid_t;

  #[no_mangle]
  fn xvfork_parent_waits_and_exits();

  #[no_mangle]
  fn wait_for_exitstatus(pid: pid_t) -> libc::c_int;

  #[no_mangle]
  fn getopt32long(
    argv: *mut *mut libc::c_char,
    optstring: *const libc::c_char,
    longopts: *const libc::c_char,
    _: ...
  ) -> uint32_t;

  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;

  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;

  #[no_mangle]
  fn bb_perror_msg_and_die(s: *const libc::c_char, _: ...) -> !;

  #[no_mangle]
  fn index_in_strings(strings: *const libc::c_char, key: *const libc::c_char) -> libc::c_int;
}

use crate::librb::__gid_t;
use crate::librb::__pid_t;
use crate::librb::__uid_t;

use crate::librb::pid_t;
use crate::librb::size_t;

pub type C2RustUnnamed = libc::c_int;
// pub const MS_NOUSER: C2RustUnnamed = -2147483648;
// pub const MS_ACTIVE: C2RustUnnamed = 1073741824;
// pub const MS_LAZYTIME: C2RustUnnamed = 33554432;
// pub const MS_STRICTATIME: C2RustUnnamed = 16777216;
// pub const MS_I_VERSION: C2RustUnnamed = 8388608;
// pub const MS_KERNMOUNT: C2RustUnnamed = 4194304;
// pub const MS_RELATIME: C2RustUnnamed = 2097152;
pub const MS_SHARED: C2RustUnnamed = 1048576;
pub const MS_SLAVE: C2RustUnnamed = 524288;
pub const MS_PRIVATE: C2RustUnnamed = 262144;
// pub const MS_UNBINDABLE: C2RustUnnamed = 131072;
// pub const MS_POSIXACL: C2RustUnnamed = 65536;
// pub const MS_SILENT: C2RustUnnamed = 32768;
pub const MS_REC: C2RustUnnamed = 16384;
// pub const MS_MOVE: C2RustUnnamed = 8192;
pub const MS_BIND: C2RustUnnamed = 4096;
// pub const MS_NODIRATIME: C2RustUnnamed = 2048;
// pub const MS_NOATIME: C2RustUnnamed = 1024;
// pub const MS_DIRSYNC: C2RustUnnamed = 128;
// pub const MS_MANDLOCK: C2RustUnnamed = 64;
// pub const MS_REMOUNT: C2RustUnnamed = 32;
// pub const MS_SYNCHRONOUS: C2RustUnnamed = 16;
pub const MS_NOEXEC: C2RustUnnamed = 8;
pub const MS_NODEV: C2RustUnnamed = 4;
pub const MS_NOSUID: C2RustUnnamed = 2;
// pub const MS_RDONLY: C2RustUnnamed = 1;

use crate::librb::uint32_t;
pub type uintptr_t = libc::c_ulong;
use crate::librb::gid_t;
use crate::librb::ssize_t;
use crate::librb::uid_t;

use crate::librb::fd_pair;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct namespace_descr {
  pub flag: libc::c_int,
  pub nsfile4: [libc::c_char; 4],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct namespace_ctx {
  pub path: *mut libc::c_char,
}

pub type C2RustUnnamed_0 = libc::c_uint;
// pub const OPT_setgroups: C2RustUnnamed_0 = 1024;
// pub const OPT_propagation: C2RustUnnamed_0 = 512;
pub const OPT_mount_proc: C2RustUnnamed_0 = 256;
pub const OPT_map_root: C2RustUnnamed_0 = 128;
/* OPT_user, NS_USR_POS, and ns_list[] index must match! */
pub const OPT_fork: C2RustUnnamed_0 = 64;
// pub const OPT_user: C2RustUnnamed_0 = 32;
// pub const OPT_pid: C2RustUnnamed_0 = 16;
// pub const OPT_net: C2RustUnnamed_0 = 8;
// pub const OPT_ipc: C2RustUnnamed_0 = 4;
// pub const OPT_uts: C2RustUnnamed_0 = 2;
pub const OPT_mount: C2RustUnnamed_0 = 1;

pub type C2RustUnnamed_1 = libc::c_uint;
/* OPT_user, NS_USR_POS, and ns_list[] index must match! */
pub const NS_COUNT: C2RustUnnamed_1 = 6;
pub const NS_USR_POS: C2RustUnnamed_1 = 5;
pub const NS_PID_POS: C2RustUnnamed_1 = 4;
pub const NS_NET_POS: C2RustUnnamed_1 = 3;
pub const NS_IPC_POS: C2RustUnnamed_1 = 2;
pub const NS_UTS_POS: C2RustUnnamed_1 = 1;
pub const NS_MNT_POS: C2RustUnnamed_1 = 0;

/*
 * Mini unshare implementation for busybox.
 *
 * Copyright (C) 2016 by Bartosz Golaszewski <bartekgola@gmail.com>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config UNSHARE
//config:	bool "unshare (7.2 kb)"
//config:	default y
//config:	depends on !NOMMU
//config:	select PLATFORM_LINUX
//config:	select LONG_OPTS
//config:	help
//config:	Run program with some namespaces unshared from parent.
// needs LONG_OPTS: it is awkward to exclude code which handles --propagation
// and --setgroups based on LONG_OPTS, so instead applet requires LONG_OPTS.
// depends on !NOMMU: we need fork()
//applet:IF_UNSHARE(APPLET(unshare, BB_DIR_USR_BIN, BB_SUID_DROP))
//kbuild:lib-$(CONFIG_UNSHARE) += unshare.o
//usage:#define unshare_trivial_usage
//usage:       "[OPTIONS] [PROG [ARGS]]"
//usage:#define unshare_full_usage "\n"
//usage:     "\n	-m,--mount[=FILE]	Unshare mount namespace"
//usage:     "\n	-u,--uts[=FILE]		Unshare UTS namespace (hostname etc.)"
//usage:     "\n	-i,--ipc[=FILE]		Unshare System V IPC namespace"
//usage:     "\n	-n,--net[=FILE]		Unshare network namespace"
//usage:     "\n	-p,--pid[=FILE]		Unshare PID namespace"
//usage:     "\n	-U,--user[=FILE]	Unshare user namespace"
//usage:     "\n	-f,--fork		Fork before execing PROG"
//usage:     "\n	-r,--map-root-user	Map current user to root (implies -U)"
//usage:     "\n	--mount-proc[=DIR]	Mount /proc filesystem first (implies -m)"
//usage:     "\n	--propagation slave|shared|private|unchanged"
//usage:     "\n				Modify mount propagation in mount namespace"
//usage:     "\n	--setgroups allow|deny	Control the setgroups syscall in user namespaces"
unsafe extern "C" fn mount_or_die(
  mut source: *const libc::c_char,
  mut target: *const libc::c_char,
  mut fstype: *const libc::c_char,
  mut mountflags: libc::c_ulong,
) {
  if mount(source, target, fstype, mountflags, 0 as *const libc::c_void) != 0 {
    bb_perror_msg_and_die(
      b"can\'t mount %s on %s (flags:0x%lx)\x00" as *const u8 as *const libc::c_char,
      source,
      target,
      mountflags,
    );
    /* fstype is always either NULL or "proc".
     * "proc" is only used to mount /proc.
     * No need to clutter up error message with fstype,
     * it is easily deductible.
     */
  };
}
static mut ns_list: [namespace_descr; 6] = [
  {
    let mut init = namespace_descr {
      flag: 0x20000i32,
      nsfile4: [109, 110, 116, 0],
    };
    init
  },
  {
    let mut init = namespace_descr {
      flag: 0x4000000i32,
      nsfile4: [117, 116, 115, 0],
    };
    init
  },
  {
    let mut init = namespace_descr {
      flag: 0x8000000i32,
      nsfile4: [105, 112, 99, 0],
    };
    init
  },
  {
    let mut init = namespace_descr {
      flag: 0x40000000i32,
      nsfile4: [110, 101, 116, 0],
    };
    init
  },
  {
    let mut init = namespace_descr {
      flag: 0x20000000i32,
      nsfile4: [112, 105, 100, 0],
    };
    init
  },
  {
    let mut init = namespace_descr {
      flag: 0x10000000i32,
      nsfile4: [117, 115, 101, 114],
    };
    init
  },
];
/*
 * Upstream unshare doesn't support short options for --mount-proc,
 * --propagation, --setgroups.
 * Optional arguments (namespace mountpoints) exist only for long opts,
 * we are forced to use "fake" letters for them.
 * '+': stop at first non-option.
 */
static mut unshare_longopts: [libc::c_char; 102] = [
  109, 111, 117, 110, 116, 0, 2, -16, 117, 116, 115, 0, 2, -15, 105, 112, 99, 0, 2, -14, 110, 101,
  116, 0, 2, -13, 112, 105, 100, 0, 2, -12, 117, 115, 101, 114, 0, 2, -11, 102, 111, 114, 107, 0,
  0, 102, 109, 97, 112, 45, 114, 111, 111, 116, 45, 117, 115, 101, 114, 0, 0, 114, 109, 111, 117,
  110, 116, 45, 112, 114, 111, 99, 0, 2, -3, 112, 114, 111, 112, 97, 103, 97, 116, 105, 111, 110,
  0, 1, -2, 115, 101, 116, 103, 114, 111, 117, 112, 115, 0, 1, -1, 0,
];
unsafe extern "C" fn parse_propagation(mut prop_str: *const libc::c_char) -> libc::c_ulong {
  let mut i: libc::c_int = index_in_strings(
    b"private\x00unchanged\x00shared\x00slave\x00\x00" as *const u8 as *const libc::c_char,
    prop_str,
  ); /* for compiler */
  if i < 0i32 {
    bb_error_msg_and_die(
      b"unrecognized: --%s=%s\x00" as *const u8 as *const libc::c_char,
      b"propagation\x00" as *const u8 as *const libc::c_char,
      prop_str,
    );
  }
  if i == 0i32 {
    return (MS_REC as libc::c_int | MS_PRIVATE as libc::c_int) as libc::c_ulong;
  }
  if i == 1i32 {
    return 0i32 as libc::c_ulong;
  }
  if i == 2i32 {
    return (MS_REC as libc::c_int | MS_SHARED as libc::c_int) as libc::c_ulong;
  }
  return (MS_REC as libc::c_int | MS_SLAVE as libc::c_int) as libc::c_ulong;
}
unsafe extern "C" fn mount_namespaces(mut pid: pid_t, mut ns_ctx_list: *mut namespace_ctx) {
  let mut ns: *const namespace_descr = 0 as *const namespace_descr;
  let mut ns_ctx: *mut namespace_ctx = 0 as *mut namespace_ctx;
  let mut i: libc::c_int = 0;
  i = 0i32;
  while i < NS_COUNT as libc::c_int {
    let mut nsf: [libc::c_char; 29] = [0; 29];
    ns = &*ns_list.as_ptr().offset(i as isize) as *const namespace_descr;
    ns_ctx = &mut *ns_ctx_list.offset(i as isize) as *mut namespace_ctx;
    if !(*ns_ctx).path.is_null() {
      sprintf(
        nsf.as_mut_ptr(),
        b"/proc/%u/ns/%.4s\x00" as *const u8 as *const libc::c_char,
        pid as libc::c_uint,
        (*ns).nsfile4.as_ptr(),
      );
      mount_or_die(
        nsf.as_mut_ptr(),
        (*ns_ctx).path,
        0 as *const libc::c_char,
        MS_BIND as libc::c_int as libc::c_ulong,
      );
    }
    i += 1
  }
}
#[no_mangle]
pub unsafe extern "C" fn unshare_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut i: libc::c_int = 0;
  let mut opts: libc::c_uint = 0;
  let mut unsflags: libc::c_int = 0;
  let mut need_mount: uintptr_t = 0;
  let mut proc_mnt_target: *const libc::c_char = 0 as *const libc::c_char;
  let mut prop_str: *const libc::c_char = 0 as *const libc::c_char;
  let mut setgrp_str: *const libc::c_char = 0 as *const libc::c_char;
  let mut prop_flags: libc::c_ulong = 0;
  let mut reuid: uid_t = geteuid();
  let mut regid: gid_t = getegid();
  let mut fdp: fd_pair = fd_pair { rd: 0, wr: 0 };
  let mut child: pid_t = 0;
  child = child;
  let mut ns_ctx_list: [namespace_ctx; 6] = [namespace_ctx {
    path: 0 as *mut libc::c_char,
  }; 6];
  memset(
    ns_ctx_list.as_mut_ptr() as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<[namespace_ctx; 6]>() as libc::c_ulong,
  );
  proc_mnt_target = b"/proc\x00" as *const u8 as *const libc::c_char;
  prop_str = b"private\x00unchanged\x00shared\x00slave\x00\x00" as *const u8 as *const libc::c_char;
  setgrp_str = 0 as *const libc::c_char;
  opts = getopt32long(
    argv,
    b"^+muinpUfr\xfd::\xfe:\xff:\x00\xf0m:\xf1u:\xf2i:\xf3n:\xf4p:\xf5U:rU:\xfdm\x00" as *const u8
      as *const libc::c_char,
    unshare_longopts.as_ptr(),
    &mut proc_mnt_target as *mut *const libc::c_char,
    &mut prop_str as *mut *const libc::c_char,
    &mut setgrp_str as *mut *const libc::c_char,
    &mut (*ns_ctx_list
      .as_mut_ptr()
      .offset(NS_MNT_POS as libc::c_int as isize))
    .path as *mut *mut libc::c_char,
    &mut (*ns_ctx_list
      .as_mut_ptr()
      .offset(NS_UTS_POS as libc::c_int as isize))
    .path as *mut *mut libc::c_char,
    &mut (*ns_ctx_list
      .as_mut_ptr()
      .offset(NS_IPC_POS as libc::c_int as isize))
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
      .offset(NS_USR_POS as libc::c_int as isize))
    .path as *mut *mut libc::c_char,
  );
  argv = argv.offset(optind as isize);
  //bb_error_msg("opts:0x%x", opts);
  //bb_error_msg("mount:%s", ns_ctx_list[NS_MNT_POS].path);
  //bb_error_msg("proc_mnt_target:%s", proc_mnt_target);
  //bb_error_msg("prop_str:%s", prop_str);
  //bb_error_msg("setgrp_str:%s", setgrp_str);
  //exit(1);
  if !setgrp_str.is_null() {
    if strcmp(setgrp_str, b"allow\x00" as *const u8 as *const libc::c_char) == 0i32 {
      if opts & OPT_map_root as libc::c_int as libc::c_uint != 0 {
        bb_simple_error_msg_and_die(
          b"--setgroups=allow and --map-root-user are mutually exclusive\x00" as *const u8
            as *const libc::c_char,
        );
      }
    } else if strcmp(setgrp_str, b"deny\x00" as *const u8 as *const libc::c_char) != 0i32 {
      bb_error_msg_and_die(
        b"unrecognized: --%s=%s\x00" as *const u8 as *const libc::c_char,
        b"setgroups\x00" as *const u8 as *const libc::c_char,
        setgrp_str,
      );
    }
  }
  unsflags = 0i32;
  need_mount = 0i32 as uintptr_t;
  i = 0i32;
  while i < NS_COUNT as libc::c_int {
    let mut ns: *const namespace_descr =
      &*ns_list.as_ptr().offset(i as isize) as *const namespace_descr;
    let mut ns_ctx: *mut namespace_ctx =
      &mut *ns_ctx_list.as_mut_ptr().offset(i as isize) as *mut namespace_ctx;
    if opts & (1i32 << i) as libc::c_uint != 0 {
      unsflags |= (*ns).flag
    }
    need_mount |= (*ns_ctx).path as uintptr_t;
    i += 1
  }
  /* It's not "allow", must be "deny" */
  /* need_mount != 0 if at least one FILE was given */
  prop_flags = (MS_REC as libc::c_int | MS_PRIVATE as libc::c_int) as libc::c_ulong;
  /* Silently ignore --propagation if --mount is not requested. */
  if opts & OPT_mount as libc::c_int as libc::c_uint != 0 {
    prop_flags = parse_propagation(prop_str)
  }
  /*
   * Special case: if we were requested to unshare the mount namespace
   * AND to make any namespace persistent (by bind mounting it) we need
   * to spawn a child process which will wait for the parent to call
   * unshare(), then mount parent's namespaces while still in the
   * previous namespace.
   */
  fdp.wr = -1i32;
  if need_mount != 0 && opts & OPT_mount as libc::c_int as libc::c_uint != 0 {
    /*
     * Can't use getppid() in child, as we can be unsharing the
     * pid namespace.
     */
    let mut ppid: pid_t = getpid();
    xpipe(&mut fdp.rd);
    child = xfork();
    if child == 0i32 {
      /* Child */
      close(fdp.wr);
      /* Wait until parent calls unshare() */
      read(
        fdp.rd,
        ns_ctx_list.as_mut_ptr() as *mut libc::c_void,
        1i32 as size_t,
      ); /* ...using bogus buffer */
      /*close(fdp.rd);*/
      /* Mount parent's unshared namespaces. */
      mount_namespaces(ppid, ns_ctx_list.as_mut_ptr()); /* Release child */
      return 0i32;
    }
  }
  if unshare(unsflags) != 0i32 {
    bb_perror_msg_and_die(
      b"unshare(0x%x)\x00" as *const u8 as *const libc::c_char,
      unsflags,
    );
  }
  if fdp.wr >= 0i32 {
    close(fdp.wr);
    close(fdp.rd);
    /* should close fd, to not confuse exec'ed PROG */
  }
  if need_mount != 0 {
    /* Wait for the child to finish mounting the namespaces. */
    if opts & OPT_mount as libc::c_int as libc::c_uint != 0 {
      let mut exit_status: libc::c_int = wait_for_exitstatus(child);
      if exit_status & 0x7fi32 == 0i32 && (exit_status & 0xff00i32) >> 8i32 != 0i32 {
        return (exit_status & 0xff00i32) >> 8i32;
      }
    } else {
      /*
       * Regular way - we were requested to mount some other
       * namespaces: mount them after the call to unshare().
       */
      mount_namespaces(getpid(), ns_ctx_list.as_mut_ptr());
    }
  }
  /*
   * When we're unsharing the pid namespace, it's not the process that
   * calls unshare() that is put into the new namespace, but its first
   * child. The user may want to use this option to spawn a new process
   * that'll become PID 1 in this new namespace.
   */
  if opts & OPT_fork as libc::c_int as libc::c_uint != 0 {
    xvfork_parent_waits_and_exits();
    /* Child continues */
  }
  if opts & OPT_map_root as libc::c_int as libc::c_uint != 0 {
    let mut uidmap_buf: [libc::c_char; 19] = [0; 19];
    /*
     * Since Linux 3.19 unprivileged writing of /proc/self/gid_map
     * has been disabled unless /proc/self/setgroups is written
     * first to permanently disable the ability to call setgroups
     * in that user namespace.
     */
    xopen_xwrite_close(
      b"/proc/self/setgroups\x00" as *const u8 as *const libc::c_char,
      b"deny\x00" as *const u8 as *const libc::c_char,
    );
    sprintf(
      uidmap_buf.as_mut_ptr(),
      b"0 %u 1\x00" as *const u8 as *const libc::c_char,
      reuid,
    );
    xopen_xwrite_close(
      b"/proc/self/uid_map\x00" as *const u8 as *const libc::c_char,
      uidmap_buf.as_mut_ptr(),
    );
    sprintf(
      uidmap_buf.as_mut_ptr(),
      b"0 %u 1\x00" as *const u8 as *const libc::c_char,
      regid,
    );
    xopen_xwrite_close(
      b"/proc/self/gid_map\x00" as *const u8 as *const libc::c_char,
      uidmap_buf.as_mut_ptr(),
    );
  } else if !setgrp_str.is_null() {
    /* Write "allow" or "deny" */
    xopen_xwrite_close(
      b"/proc/self/setgroups\x00" as *const u8 as *const libc::c_char,
      setgrp_str,
    );
  }
  if opts & OPT_mount as libc::c_int as libc::c_uint != 0 {
    mount_or_die(
      b"none\x00" as *const u8 as *const libc::c_char,
      b"/\x00" as *const u8 as *const libc::c_char,
      0 as *const libc::c_char,
      prop_flags,
    );
  }
  if opts & OPT_mount_proc as libc::c_int as libc::c_uint != 0 {
    /*
     * When creating a new pid namespace, we might want the pid
     * subdirectories in /proc to remain consistent with the new
     * process IDs. Without --mount-proc the pids in /proc would
     * still reflect the old pid namespace. This is why we make
     * /proc private here and then do a fresh mount.
     */
    mount_or_die(
      b"none\x00" as *const u8 as *const libc::c_char,
      proc_mnt_target,
      0 as *const libc::c_char,
      (MS_PRIVATE as libc::c_int | MS_REC as libc::c_int) as libc::c_ulong,
    );
    mount_or_die(
      b"proc\x00" as *const u8 as *const libc::c_char,
      proc_mnt_target,
      b"proc\x00" as *const u8 as *const libc::c_char,
      (MS_NOSUID as libc::c_int | MS_NOEXEC as libc::c_int | MS_NODEV as libc::c_int)
        as libc::c_ulong,
    );
  }
  exec_prog_or_SHELL(argv);
}
