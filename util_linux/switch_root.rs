use libc;
use libc::stat;

extern "C" {
  pub type __dirstream;

  #[no_mangle]
  fn statfs(__file: *const libc::c_char, __buf: *mut statfs) -> libc::c_int;

  #[no_mangle]
  fn mount(
    __special_file: *const libc::c_char,
    __dir: *const libc::c_char,
    __fstype: *const libc::c_char,
    __rwflag: libc::c_ulong,
    __data: *const libc::c_void,
  ) -> libc::c_int;

  #[no_mangle]
  fn prctl(__option: libc::c_int, _: ...) -> libc::c_int;

  /*
   * Copyright 2005 Rob Landley <rob@landley.net>
   *
   * Switch from rootfs to another filesystem as the root of the mount tree.
   *
   * Licensed under GPLv2, see file LICENSE in this source tree.
   */
  //config:config SWITCH_ROOT
  //config:	bool "switch_root (5.5 kb)"
  //config:	default y
  //config:	select PLATFORM_LINUX
  //config:	help
  //config:	The switch_root utility is used from initramfs to select a new
  //config:	root device. Under initramfs, you have to use this instead of
  //config:	pivot_root. (Stop reading here if you don't care why.)
  //config:
  //config:	Booting with initramfs extracts a gzipped cpio archive into rootfs
  //config:	(which is a variant of ramfs/tmpfs). Because rootfs can't be moved
  //config:	or unmounted*, pivot_root will not work from initramfs. Instead,
  //config:	switch_root deletes everything out of rootfs (including itself),
  //config:	does a mount --move that overmounts rootfs with the new root, and
  //config:	then execs the specified init program.
  //config:
  //config:	* Because the Linux kernel uses rootfs internally as the starting
  //config:	and ending point for searching through the kernel's doubly linked
  //config:	list of active mount points. That's why.
  //config:
  // RUN_INIT config item is in klibc-utils
  //applet:IF_SWITCH_ROOT(APPLET(switch_root, BB_DIR_SBIN, BB_SUID_DROP))
  //                      APPLET_ODDNAME:name      main         location     suid_type     help
  //applet:IF_RUN_INIT(   APPLET_ODDNAME(run-init, switch_root, BB_DIR_SBIN, BB_SUID_DROP, run_init))
  //kbuild:lib-$(CONFIG_SWITCH_ROOT) += switch_root.o
  //kbuild:lib-$(CONFIG_RUN_INIT)    += switch_root.o
  // #include <sys/capability.h>
  // This header is in libcap, but the functions are in libc.
  // Comment in the header says this above capset/capget:
  /* system calls - look to libc for function to system call mapping */
  #[no_mangle]
  fn capset(header: cap_user_header_t, data: cap_user_data_t) -> libc::c_int;

  #[no_mangle]
  fn getpid() -> __pid_t;

  #[no_mangle]
  fn unlink(__name: *const libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn rmdir(__path: *const libc::c_char) -> libc::c_int;

  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  fn opendir(__name: *const libc::c_char) -> *mut DIR;

  #[no_mangle]
  fn closedir(__dirp: *mut DIR) -> libc::c_int;

  #[no_mangle]
  fn readdir(__dirp: *mut DIR) -> *mut dirent;

  #[no_mangle]
  fn dprintf(__fd: libc::c_int, __fmt: *const libc::c_char, _: ...) -> libc::c_int;

  #[no_mangle]
  fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;

  #[no_mangle]
  fn execv(__path: *const libc::c_char, __argv: *const *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);

  #[no_mangle]
  fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;

  #[no_mangle]
  fn lstat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;

  #[no_mangle]
  fn xdup2(_: libc::c_int, _: libc::c_int);

  #[no_mangle]
  fn xmove_fd(_: libc::c_int, _: libc::c_int);

  #[no_mangle]
  fn xchdir(path: *const libc::c_char);

  #[no_mangle]
  fn xchroot(path: *const libc::c_char);

  #[no_mangle]
  fn xstat(pathname: *const libc::c_char, buf: *mut stat);

  #[no_mangle]
  fn open_or_warn(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn open_read_close(
    filename: *const libc::c_char,
    buf: *mut libc::c_void,
    maxsz: size_t,
  ) -> ssize_t;

  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;

  #[no_mangle]
  fn bb_show_usage() -> !;

  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);

  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;

  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;

  #[no_mangle]
  fn bb_perror_msg_and_die(s: *const libc::c_char, _: ...) -> !;

  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;

  #[no_mangle]
  fn concat_path_file(
    path: *const libc::c_char,
    filename: *const libc::c_char,
  ) -> *mut libc::c_char;

  #[no_mangle]
  fn cap_name_to_number(cap: *const libc::c_char) -> libc::c_uint;

  #[no_mangle]
  fn getcaps(caps: *mut libc::c_void);

  #[no_mangle]
  static mut applet_name: *const libc::c_char;
}

use libc::ino64_t;

use crate::librb::__off64_t;
use crate::librb::__pid_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __fsid_t {
  pub __val: [libc::c_int; 2],
}

pub type __fsblkcnt64_t = libc::c_ulong;
pub type __fsfilcnt64_t = libc::c_ulong;
pub type __fsword_t = libc::c_long;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct statfs {
  pub f_type: __fsword_t,
  pub f_bsize: __fsword_t,
  pub f_blocks: __fsblkcnt64_t,
  pub f_bfree: __fsblkcnt64_t,
  pub f_bavail: __fsblkcnt64_t,
  pub f_files: __fsfilcnt64_t,
  pub f_ffree: __fsfilcnt64_t,
  pub f_fsid: __fsid_t,
  pub f_namelen: __fsword_t,
  pub f_frsize: __fsword_t,
  pub f_flags: __fsword_t,
  pub f_spare: [__fsword_t; 4],
}

pub type C2RustUnnamed = libc::c_int;
// pub const MS_NOUSER: C2RustUnnamed = -2147483648;
// pub const MS_ACTIVE: C2RustUnnamed = 1073741824;
// pub const MS_LAZYTIME: C2RustUnnamed = 33554432;
// pub const MS_STRICTATIME: C2RustUnnamed = 16777216;
// pub const MS_I_VERSION: C2RustUnnamed = 8388608;
// pub const MS_KERNMOUNT: C2RustUnnamed = 4194304;
// pub const MS_RELATIME: C2RustUnnamed = 2097152;
// pub const MS_SHARED: C2RustUnnamed = 1048576;
// pub const MS_SLAVE: C2RustUnnamed = 524288;
// pub const MS_PRIVATE: C2RustUnnamed = 262144;
// pub const MS_UNBINDABLE: C2RustUnnamed = 131072;
// pub const MS_POSIXACL: C2RustUnnamed = 65536;
// pub const MS_SILENT: C2RustUnnamed = 32768;
// pub const MS_REC: C2RustUnnamed = 16384;
pub const MS_MOVE: C2RustUnnamed = 8192;
// pub const MS_BIND: C2RustUnnamed = 4096;
// pub const MS_NODIRATIME: C2RustUnnamed = 2048;
// pub const MS_NOATIME: C2RustUnnamed = 1024;
// pub const MS_DIRSYNC: C2RustUnnamed = 128;
// pub const MS_MANDLOCK: C2RustUnnamed = 64;
// pub const MS_REMOUNT: C2RustUnnamed = 32;
// pub const MS_SYNCHRONOUS: C2RustUnnamed = 16;
// pub const MS_NOEXEC: C2RustUnnamed = 8;
// pub const MS_NODEV: C2RustUnnamed = 4;
// pub const MS_NOSUID: C2RustUnnamed = 2;
// pub const MS_RDONLY: C2RustUnnamed = 1;

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
use crate::librb::size_t;
use crate::librb::ssize_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
  pub d_ino: ino64_t,
  pub d_off: __off64_t,
  pub d_reclen: libc::c_ushort,
  pub d_type: libc::c_uchar,
  pub d_name: [libc::c_char; 256],
}

pub type DIR = __dirstream;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct caps {
  pub header: __user_cap_header_struct,
  pub u32s: libc::c_uint,
  pub data: [__user_cap_data_struct; 2],
}

// Recursively delete contents of rootfs
unsafe extern "C" fn delete_contents(mut directory: *const libc::c_char, mut rootdev: libc::dev_t) {
  let mut dir: *mut DIR = 0 as *mut DIR;
  let mut d: *mut dirent = 0 as *mut dirent;
  let mut st: stat = std::mem::zeroed();
  // Don't descend into other filesystems
  if lstat(directory, &mut st) != 0 || st.st_dev != rootdev {
    return;
  }
  // Recursively delete the contents of directories
  if st.st_mode & 0o170000i32 as libc::c_uint == 0o40000i32 as libc::c_uint {
    dir = opendir(directory);
    if !dir.is_null() {
      loop {
        d = readdir(dir);
        if d.is_null() {
          break;
        }
        let mut newdir: *mut libc::c_char = (*d).d_name.as_mut_ptr();
        // Skip . and ..
        if *newdir.offset(0) as libc::c_int == '.' as i32
          && (*newdir.offset(1) == 0
            || *newdir.offset(1) as libc::c_int == '.' as i32 && *newdir.offset(2) == 0)
        {
          continue;
        }
        // Recurse to delete contents
        newdir = concat_path_file(directory, newdir);
        delete_contents(newdir, rootdev);
        free(newdir as *mut libc::c_void);
      }
      closedir(dir);
      // Directory should now be empty, zap it
      rmdir(directory);
    }
  } else {
    // It wasn't a directory, zap it
    unlink(directory); /* assuming files do not exist */
  };
}
unsafe extern "C" fn drop_capset(mut cap_idx: libc::c_int) {
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
  caps.data[(cap_idx >> 5i32) as usize].inheritable &= !(1i32 << (cap_idx & 31i32)) as libc::c_uint;
  if capset(&mut caps.header, caps.data.as_mut_ptr()) != 0i32 {
    bb_simple_perror_msg_and_die(b"capset\x00" as *const u8 as *const libc::c_char);
  };
}
unsafe extern "C" fn drop_bounding_set(mut cap_idx: libc::c_int) {
  let mut ret: libc::c_int = 0;
  ret = prctl(23i32, cap_idx, 0i32, 0i32, 0i32);
  if ret < 0i32 {
    bb_perror_msg_and_die(
      b"prctl: %s\x00" as *const u8 as *const libc::c_char,
      b"PR_CAPBSET_READ\x00" as *const u8 as *const libc::c_char,
    );
  }
  if ret == 1i32 {
    ret = prctl(24i32, cap_idx, 0i32, 0i32, 0i32);
    if ret != 0i32 {
      bb_perror_msg_and_die(
        b"prctl: %s\x00" as *const u8 as *const libc::c_char,
        b"PR_CAPBSET_DROP\x00" as *const u8 as *const libc::c_char,
      );
    }
  };
}
unsafe extern "C" fn drop_usermodehelper(
  mut filename: *const libc::c_char,
  mut cap_idx: libc::c_int,
) {
  let mut lo: libc::c_uint = 0;
  let mut hi: libc::c_uint = 0;
  let mut buf: [libc::c_char; 32] = [0; 32];
  let mut fd: libc::c_int = 0;
  let mut ret: libc::c_int = 0;
  ret = open_read_close(
    filename,
    buf.as_mut_ptr() as *mut libc::c_void,
    (::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
      .wrapping_sub(1i32 as libc::c_ulong),
  ) as libc::c_int;
  if ret < 0i32 {
    return;
  }
  buf[ret as usize] = '\u{0}' as i32 as libc::c_char;
  ret = sscanf(
    buf.as_mut_ptr(),
    b"%u %u\x00" as *const u8 as *const libc::c_char,
    &mut lo as *mut libc::c_uint,
    &mut hi as *mut libc::c_uint,
  );
  if ret != 2i32 {
    bb_perror_msg_and_die(
      b"can\'t parse file \'%s\'\x00" as *const u8 as *const libc::c_char,
      filename,
    );
  }
  if cap_idx < 32i32 {
    lo &= !(1i32 << cap_idx) as libc::c_uint
  } else {
    hi &= !(1i32 << cap_idx - 32i32) as libc::c_uint
  }
  fd = xopen(filename, 0o1i32);
  dprintf(fd, b"%u %u\x00" as *const u8 as *const libc::c_char, lo, hi);
  close(fd);
}
unsafe extern "C" fn drop_capabilities(mut string: *mut libc::c_char) {
  let mut cap: *mut libc::c_char = 0 as *mut libc::c_char;
  cap = strtok(string, b",\x00" as *const u8 as *const libc::c_char);
  while !cap.is_null() {
    let mut cap_idx: libc::c_uint = 0;
    cap_idx = cap_name_to_number(cap);
    drop_usermodehelper(
      b"/proc/sys/kernel/usermodehelper/bset\x00" as *const u8 as *const libc::c_char,
      cap_idx as libc::c_int,
    );
    drop_usermodehelper(
      b"/proc/sys/kernel/usermodehelper/inheritable\x00" as *const u8 as *const libc::c_char,
      cap_idx as libc::c_int,
    );
    drop_bounding_set(cap_idx as libc::c_int);
    drop_capset(cap_idx as libc::c_int);
    bb_error_msg(
      b"dropped capability: %s\x00" as *const u8 as *const libc::c_char,
      cap,
    );
    cap = strtok(
      0 as *mut libc::c_char,
      b",\x00" as *const u8 as *const libc::c_char,
    )
  }
}
#[no_mangle]
pub unsafe extern "C" fn switch_root_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut newroot: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut console: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut st: stat = std::mem::zeroed();
  let mut stfs: statfs = statfs {
    f_type: 0,
    f_bsize: 0,
    f_blocks: 0,
    f_bfree: 0,
    f_bavail: 0,
    f_files: 0,
    f_ffree: 0,
    f_fsid: __fsid_t { __val: [0; 2] },
    f_namelen: 0,
    f_frsize: 0,
    f_flags: 0,
    f_spare: [0; 4],
  };
  let mut dry_run: libc::c_uint = 0i32 as libc::c_uint;
  let mut rootdev: libc::dev_t = 0;
  // Parse args. '+': stop at first non-option
  if 1i32 != 0 && (1i32 == 0 || *applet_name.offset(0) as libc::c_int == 's' as i32) {
    //usage:#define switch_root_trivial_usage
    //usage:       "[-c CONSOLE_DEV] NEW_ROOT NEW_INIT [ARGS]"
    //usage:#define switch_root_full_usage "\n\n"
    //usage:       "Free initramfs and switch to another root fs:\n"
    //usage:       "chroot to NEW_ROOT, delete all in /, move NEW_ROOT to /,\n"
    //usage:       "execute NEW_INIT. PID must be 1. NEW_ROOT must be a mountpoint.\n"
    //usage:     "\n	-c DEV	Reopen stdio to DEV after switch"
    getopt32(
      argv,
      b"^+c:\x00-2\x00" as *const u8 as *const libc::c_char,
      &mut console as *mut *mut libc::c_char,
    );
  } else {
    //usage:#define run_init_trivial_usage
    //usage:       "[-d CAP,CAP...] [-n] [-c CONSOLE_DEV] NEW_ROOT NEW_INIT [ARGS]"
    //usage:#define run_init_full_usage "\n\n"
    //usage:       "Free initramfs and switch to another root fs:\n"
    //usage:       "chroot to NEW_ROOT, delete all in /, move NEW_ROOT to /,\n"
    //usage:       "execute NEW_INIT. PID must be 1. NEW_ROOT must be a mountpoint.\n"
    //usage:     "\n	-c DEV	Reopen stdio to DEV after switch"
    //usage:     "\n	-d CAPS	Drop capabilities"
    //usage:     "\n	-n	Dry run"
    let mut cap_list: *mut libc::c_char = 0 as *mut libc::c_char; // -n
    dry_run = getopt32(
      argv,
      b"^+c:d:n\x00-2\x00" as *const u8 as *const libc::c_char,
      &mut console as *mut *mut libc::c_char,
      &mut cap_list as *mut *mut libc::c_char,
    );
    dry_run >>= 2i32;
    if !cap_list.is_null() {
      drop_capabilities(cap_list);
    }
  }
  argv = argv.offset(optind as isize);
  let fresh0 = argv;
  argv = argv.offset(1);
  newroot = *fresh0;
  // Change to new root directory and verify it's a different fs
  xchdir(newroot);
  xstat(b"/\x00" as *const u8 as *const libc::c_char, &mut st);
  rootdev = st.st_dev;
  xstat(b".\x00" as *const u8 as *const libc::c_char, &mut st);
  if st.st_dev == rootdev {
    // Show usage, it says new root must be a mountpoint
    bb_show_usage();
  }
  if dry_run == 0 && getpid() != 1i32 {
    // Show usage, it says we must be PID 1
    bb_show_usage();
  }
  // Additional sanity checks: we're about to rm -rf /, so be REALLY SURE
  // we mean it. I could make this a CONFIG option, but I would get email
  // from all the people who WILL destroy their filesystems.
  if stat(b"/init\x00" as *const u8 as *const libc::c_char, &mut st) != 0i32
    || !(st.st_mode & 0o170000i32 as libc::c_uint == 0o100000i32 as libc::c_uint)
  {
    bb_error_msg_and_die(
      b"\'%s\' is not a regular file\x00" as *const u8 as *const libc::c_char,
      b"/init\x00" as *const u8 as *const libc::c_char,
    ); // this never fails
  }
  statfs(b"/\x00" as *const u8 as *const libc::c_char, &mut stfs);
  if stfs.f_type as libc::c_uint != 0x858458f6u32
    && stfs.f_type as libc::c_uint != 0x1021994i32 as libc::c_uint
  {
    bb_simple_error_msg_and_die(
      b"root filesystem is not ramfs/tmpfs\x00" as *const u8 as *const libc::c_char,
    );
  }
  if dry_run == 0 {
    // Zap everything out of rootdev
    delete_contents(b"/\x00" as *const u8 as *const libc::c_char, rootdev);
    // Overmount / with newdir and chroot into it
    if mount(
      b".\x00" as *const u8 as *const libc::c_char,
      b"/\x00" as *const u8 as *const libc::c_char,
      0 as *const libc::c_char,
      MS_MOVE as libc::c_int as libc::c_ulong,
      0 as *const libc::c_void,
    ) != 0
    {
      // For example, fails when newroot is not a mountpoint
      bb_simple_perror_msg_and_die(b"error moving root\x00" as *const u8 as *const libc::c_char);
    }
  }
  xchroot(b".\x00" as *const u8 as *const libc::c_char);
  // The chdir is needed to recalculate "." and ".." links
  /*xchdir("/"); - done in xchroot */
  // If a new console specified, redirect stdin/stdout/stderr to it
  if !console.is_null() {
    let mut fd: libc::c_int = open_or_warn(console, 0o2i32);
    if fd >= 0i32 {
      xmove_fd(fd, 0i32);
      xdup2(0i32, 1i32);
      xdup2(0i32, 2i32);
    }
  }
  if dry_run != 0 {
    // Does NEW_INIT look like it can be executed?
    //xstat(argv[0], &st);
    //if (!S_ISREG(st.st_mode))
    //	bb_perror_msg_and_die("'%s' is not a regular file", argv[0]);
    if access(*argv.offset(0), 1i32) == 0i32 {
      return 0i32;
    }
  } else {
    // Exec NEW_INIT
    execv(*argv.offset(0), argv as *const *mut libc::c_char);
  }
  bb_perror_msg_and_die(
    b"can\'t execute \'%s\'\x00" as *const u8 as *const libc::c_char,
    *argv.offset(0),
  );
}
/*
From: Rob Landley <rob@landley.net>
Date: Tue, Jun 16, 2009 at 7:47 PM
Subject: Re: switch_root...

...
...
...

If you're _not_ running out of init_ramfs (if for example you're using initrd
instead), you probably shouldn't use switch_root because it's the wrong tool.

Basically what the sucker does is something like the following shell script:

 find / -xdev | xargs rm -rf
 cd "$1"
 shift
 mount --move . /
 exec chroot . "$@"

There are a couple reasons that won't work as a shell script:

1) If you delete the commands out of your $PATH, your shell scripts can't run
more commands, but you can't start using dynamically linked _new_ commands
until after you do the chroot because the path to the dynamic linker is wrong.
So there's a step that needs to be sort of atomic but can't be as a shell
script.  (You can work around this with static linking or very carefully laid
out paths and sequencing, but it's brittle, ugly, and non-obvious.)

2) The "find | rm" bit will actually delete everything because the mount points
still show up (even if their contents don't), and rm -rf will then happily zap
that.  So the first line is an oversimplification of what you need to do _not_
to descend into other filesystems and delete their contents.

The reason we do this is to free up memory, by the way.  Since initramfs is a
ramfs, deleting its contents frees up the memory it uses.  (We leave it with
one remaining dentry for the new mount point, but that's ok.)

Note that you cannot ever umount rootfs, for approximately the same reason you
can't kill PID 1.  The kernel tracks mount points as a doubly linked list, and
the pointer to the start/end of that list always points to an entry that's
known to be there (rootfs), so it never has to worry about moving that pointer
and it never has to worry about the list being empty.  (Back around 2.6.13
there _was_ a bug that let you umount rootfs, and the system locked hard the
instant you did so endlessly looping to find the end of the mount list and
never stopping.  They fixed it.)

Oh, and the reason we mount --move _and_ do the chroot is due to the way "/"
works.  Each process has two special symlinks, ".", and "/".  Each of them
points to the dentry of a directory, and give you a location paths can start
from.  (Historically ".." was also special, because you could enter a
directory via a symlink so backing out to the directory you came from doesn't
necessarily mean the one physically above where "." points to.  These days I
think it's just handed off to the filesystem.)

Anyway, path resolution starts with "." or "/" (although the "./" at the start
of the path may be implicit), meaning it's relative to one of those two
directories.  Your current directory, and your current root directory.  The
chdir() syscall changes where "." points to, and the chroot() syscall changes
where "/" points to.  (Again, both are per-process which is why chroot only
affects your current process and its child processes.)

Note that chroot() does _not_ change where "." points to, and back before they
put crazy security checks into the kernel your current directory could be
somewhere you could no longer access after the chroot.  (The command line
chroot does a cd as well, the chroot _syscall_ is what I'm talking about.)

The reason mounting something new over / has no obvious effect is the same
reason mounting something over your current directory has no obvious effect:
the . and / links aren't recalculated after a mount, so they still point to
the same dentry they did before, even if that dentry is no longer accessible
by other means.  Note that "cd ." is a NOP, and "chroot /" is a nop; both look
up the cached dentry and set it right back.  They don't re-parse any paths,
because they're what all paths your process uses would be relative to.

That's why the careful sequencing above: we cd into the new mount point before
we do the mount --move.  Moving the mount point would otherwise make it
totally inaccessible to us because cd-ing to the old path wouldn't give it to
us anymore, and cd "/" just gives us the cached dentry from when the process
was created (in this case the old initramfs one).  But the "." symlink gives
us the dentry of the filesystem we just moved, so we can then "chroot ." to
copy that dentry to "/" and get the new filesystem.  If we _didn't_ save that
dentry in "." we couldn't get it back after the mount --move.

(Yes, this is all screwy and I had to email questions to Linus Torvalds to get
it straight myself.  I keep meaning to write up a "how mount actually works"
document someday...)
*/
