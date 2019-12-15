use crate::libbb::ptr_to_globals::bb_errno;
use crate::librb::size_t;

use libc;
use libc::endmntent;
use libc::free;
use libc::mount;
use libc::setmntent;
use libc::strcmp;
use libc::umount2;
use libc::FILE;
extern "C" {

  #[no_mangle]
  fn getmntent_r(
    __stream: *mut FILE,
    __result: *mut mntent,
    __buffer: *mut libc::c_char,
    __bufsize: libc::c_int,
  ) -> *mut mntent;

  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;

  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;

  #[no_mangle]
  fn xmalloc_realpath(path: *const libc::c_char) -> *mut libc::c_char;

  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;

  #[no_mangle]
  fn bb_show_usage() -> !;

  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);

  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;

  #[no_mangle]
  fn bb_perror_msg(s: *const libc::c_char, _: ...);

  #[no_mangle]
  fn fstype_matches(fstype: *const libc::c_char, comma_list: *const libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn del_loop(device: *const libc::c_char) -> libc::c_int;

  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct mntent {
  pub mnt_fsname: *mut libc::c_char,
  pub mnt_dir: *mut libc::c_char,
  pub mnt_type: *mut libc::c_char,
  pub mnt_opts: *mut libc::c_char,
  pub mnt_freq: libc::c_int,
  pub mnt_passno: libc::c_int,
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
// pub const MS_MOVE: C2RustUnnamed = 8192;
// pub const MS_BIND: C2RustUnnamed = 4096;
// pub const MS_NODIRATIME: C2RustUnnamed = 2048;
// pub const MS_NOATIME: C2RustUnnamed = 1024;
// pub const MS_DIRSYNC: C2RustUnnamed = 128;
// pub const MS_MANDLOCK: C2RustUnnamed = 64;
pub const MS_REMOUNT: C2RustUnnamed = 32;
// pub const MS_SYNCHRONOUS: C2RustUnnamed = 16;
// pub const MS_NOEXEC: C2RustUnnamed = 8;
// pub const MS_NODEV: C2RustUnnamed = 4;
// pub const MS_NOSUID: C2RustUnnamed = 2;
pub const MS_RDONLY: C2RustUnnamed = 1;

pub type C2RustUnnamed_0 = libc::c_uint;
pub const COMMON_BUFSIZE: C2RustUnnamed_0 = 1024;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct mtab_list {
  pub dir: *mut libc::c_char,
  pub device: *mut libc::c_char,
  pub next: *mut mtab_list,
}

#[no_mangle]
pub unsafe extern "C" fn umount_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut doForce: libc::c_int = 0;
  let mut me: mntent = mntent {
    mnt_fsname: std::ptr::null_mut::<libc::c_char>(),
    mnt_dir: std::ptr::null_mut::<libc::c_char>(),
    mnt_type: std::ptr::null_mut::<libc::c_char>(),
    mnt_opts: std::ptr::null_mut::<libc::c_char>(),
    mnt_freq: 0,
    mnt_passno: 0,
  };
  let mut fp: *mut FILE = std::ptr::null_mut();
  let mut fstype: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut status: libc::c_int = 0i32;
  let mut opt: libc::c_uint = 0;
  let mut mtl: *mut mtab_list = std::ptr::null_mut();
  let mut m: *mut mtab_list = std::ptr::null_mut();
  opt = getopt32(
    argv,
    b"fldnrat:cvi\x00" as *const u8 as *const libc::c_char,
    &mut fstype as *mut *mut libc::c_char,
  );
  //argc -= optind;
  argv = argv.offset(optind as isize);
  // MNT_FORCE and MNT_DETACH (from linux/fs.h) must match
  // OPT_FORCE and OPT_LAZY.
  doForce = (opt & (1i32 << 0i32 | 1i32 << 1i32) as libc::c_uint) as libc::c_int;
  /* Get a list of mount points from mtab.  We read them all in now mostly
   * for umount -a (so we don't have to worry about the list changing while
   * we iterate over it, or about getting stuck in a loop on the same failing
   * entry.  Notice that this also naturally reverses the list so that -a
   * umounts the most recent entries first. */
  mtl = std::ptr::null_mut();
  m = mtl;
  // If we're umounting all, then m points to the start of the list and
  // the argument list should be empty (which will match all).
  fp = setmntent(
    b"/proc/mounts\x00" as *const u8 as *const libc::c_char,
    b"r\x00" as *const u8 as *const libc::c_char,
  );
  if fp.is_null() {
    if opt & (if 1i32 != 0 { (1i32) << 5i32 } else { 0i32 }) as libc::c_uint != 0 {
      bb_error_msg_and_die(
        b"can\'t open \'%s\'\x00" as *const u8 as *const libc::c_char,
        b"/proc/mounts\x00" as *const u8 as *const libc::c_char,
      );
    }
  } else {
    while !getmntent_r(
      fp,
      &mut me,
      bb_common_bufsiz1.as_mut_ptr(),
      COMMON_BUFSIZE as libc::c_int,
    )
    .is_null()
    {
      /* Match fstype (fstype==NULL matches always) */
      if fstype_matches(me.mnt_type, fstype) == 0 {
        continue;
      }
      m = xzalloc(::std::mem::size_of::<mtab_list>() as libc::c_ulong) as *mut mtab_list;
      (*m).next = mtl;
      (*m).device = xstrdup(me.mnt_fsname);
      (*m).dir = xstrdup(me.mnt_dir);
      mtl = m
    }
    endmntent(fp);
  }
  // If we're not umounting all, we need at least one argument.
  // Note: "-t FSTYPE" does not imply -a.
  if opt & (if 1i32 != 0 { (1i32) << 5i32 } else { 0i32 }) as libc::c_uint == 0 {
    if (*argv.offset(0)).is_null() {
      bb_show_usage();
    }
    m = std::ptr::null_mut()
  }
  loop
  // Loop through everything we're supposed to umount, and do so.
  {
    let mut curstat: libc::c_int = 0;
    let mut zapit: *mut libc::c_char = *argv;
    let mut path: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    // Do we already know what to umount this time through the loop?
    if !m.is_null() {
      path = xstrdup((*m).dir)
    } else {
      // For umount -a, end of mtab means time to exit.
      if opt & (if 1i32 != 0 { (1i32) << 5i32 } else { 0i32 }) as libc::c_uint != 0 {
        break;
      }
      // Use command line argument (and look it up in mtab list)
      if zapit.is_null() {
        break;
      }
      argv = argv.offset(1);
      path = xmalloc_realpath(zapit);
      if !path.is_null() {
        m = mtl;
        while !m.is_null() {
          if strcmp(path, (*m).dir) == 0i32 || strcmp(path, (*m).device) == 0i32 {
            break;
          }
          m = (*m).next
        }
      }
    }
    // If we couldn't find this sucker in /etc/mtab, punt by passing our
    // command line argument straight to the umount syscall.  Otherwise,
    // umount the directory even if we were given the block device.
    if !m.is_null() {
      zapit = (*m).dir
    }
    // umount from util-linux 2.22.2 does not do this:
    // umount -f uses umount2(MNT_FORCE) immediately,
    // not trying umount() first.
    // (Strangely, umount -fl ignores -f: it is equivalent to umount -l.
    // We do pass both flags in this case)
    curstat = umount2(zapit, doForce);
    // If still can't umount, maybe remount read-only?
    if curstat != 0 {
      if opt & (1i32 << 4i32) as libc::c_uint != 0 && *bb_errno == 16i32 && !m.is_null() {
        // Note! Even if we succeed here, later we should not
        // free loop device or erase mtab entry!
        let mut msg: *const libc::c_char =
          b"%s busy - remounted read-only\x00" as *const u8 as *const libc::c_char;
        curstat = mount(
          (*m).device,
          zapit,
          0 as *const libc::c_char,
          (MS_REMOUNT as libc::c_int | MS_RDONLY as libc::c_int) as libc::c_ulong,
          0 as *const libc::c_void,
        );
        if curstat != 0 {
          msg = b"can\'t remount %s read-only\x00" as *const u8 as *const libc::c_char;
          status = 1i32
        }
        bb_error_msg(msg, (*m).device);
      } else {
        status = 1i32;
        bb_perror_msg(
          b"can\'t unmount %s\x00" as *const u8 as *const libc::c_char,
          zapit,
        );
      }
    } else {
      // De-allocate the loop device.  This ioctl should be ignored on
      // any non-loop block devices.
      // This was originally
      //    if (ENABLE_FEATURE_MOUNT_LOOP && (opt & OPT_FREELOOP) && m)
      // in the C code.
      if 1i32 != 0 && opt & (1i32 << 2i32) as libc::c_uint != 0 && !m.is_null() {
        del_loop((*m).device);
      }

      // This was originally
      //    if (ENABLE_FEATURE_MTAB_SUPPORT && !(opt & OPT_NO_MTAB) && m)
      // in the C code.
      // }
    }
    // Find next matching mtab entry for -a or umount /dev
    // Note this means that "umount /dev/blah" will unmount all instances
    // of /dev/blah, not just the most recent.
    if !m.is_null() {
      loop {
        m = (*m).next;
        if m.is_null() {
          break;
        }
        // NB: if m is non-NULL, path is non-NULL as well
        if opt & (if 1i32 != 0 { (1i32) << 5i32 } else { 0i32 }) as libc::c_uint != 0
          || strcmp(path, (*m).device) == 0i32
        {
          break;
        }
      }
    }
    free(path as *mut libc::c_void);
  }
  // Free mtab list if necessary
  return status;
}
