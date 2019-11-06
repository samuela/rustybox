use libc;
use libc::stat;
use libc::FILE;

extern "C" {
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);

  #[no_mangle]
  static mut applet_name: *const libc::c_char;

  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  fn fclose(__stream: *mut FILE) -> libc::c_int;

  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn strchrnul(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;

  #[no_mangle]
  fn getmntent(__stream: *mut FILE) -> *mut mntent;

  #[no_mangle]
  fn endmntent(__stream: *mut FILE) -> libc::c_int;

  #[no_mangle]
  fn hasmntopt(__mnt: *const mntent, __opt: *const libc::c_char) -> *mut libc::c_char;

  #[no_mangle]
  static bb_errno: *mut libc::c_int;

  #[no_mangle]
  fn xmalloc_fgetline(file: *mut FILE) -> *mut libc::c_char;

  #[no_mangle]
  fn fopen_for_read(path: *const libc::c_char) -> *mut FILE;

  #[no_mangle]
  fn xfopen_for_read(path: *const libc::c_char) -> *mut FILE;

  #[no_mangle]
  fn bb_strtou(
    arg: *const libc::c_char,
    endp: *mut *mut libc::c_char,
    base: libc::c_int,
  ) -> libc::c_uint;

  #[no_mangle]
  static mut option_mask32: uint32_t;

  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;

  #[no_mangle]
  fn bb_show_usage() -> !;

  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);

  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);

  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];

  #[no_mangle]
  fn swapon(__path: *const libc::c_char, __flags: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn swapoff(__path: *const libc::c_char) -> libc::c_int;

  /* Returns:
   * 0: no UUID= or LABEL= prefix found
   * 1: UUID= or LABEL= prefix found. In this case,
   *    *fsname is replaced if device with such UUID or LABEL is found
   */
  #[no_mangle]
  fn resolve_mount_spec(fsname: *mut *mut libc::c_char) -> libc::c_int;
}

use crate::librb::off_t;
use libc::uint32_t;

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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub flags: libc::c_int,
}

/* Command line options */
pub type C2RustUnnamed = libc::c_uint;
pub const OPT_p: C2RustUnnamed = 8;
pub const OPT_d: C2RustUnnamed = 4;
pub const OPT_e: C2RustUnnamed = 2;
/* -p priority */
pub const OPT_a: C2RustUnnamed = 1;
/* -d discard  */
// pub const OPTBIT_p: C2RustUnnamed = 3;
/* -e ifexists */
// pub const OPTBIT_d: C2RustUnnamed = 2;
/* -a all      */
// pub const OPTBIT_e: C2RustUnnamed = 1;
// pub const OPTBIT_a: C2RustUnnamed = 0;

unsafe extern "C" fn swap_enable_disable(mut device: *mut libc::c_char) -> libc::c_int {
  let mut err: libc::c_int = 0i32;
  let mut quiet: libc::c_int = 0i32;
  resolve_mount_spec(&mut device);
  if *applet_name.offset(5) as libc::c_int == 'f' as i32 {
    err = swapoff(device);
    /* Don't complain on OPT_ALL if not a swap device or if it doesn't exist */
    quiet = (option_mask32 & OPT_a as libc::c_int as libc::c_uint != 0
      && (*bb_errno == 22i32 || *bb_errno == 2i32)) as libc::c_int
  } else {
    /* swapon */
    let mut st: stat = std::mem::zeroed();
    err = stat(device, &mut st);
    if err == 0 {
      if 1i32 != 0 && st.st_mode & 0o170000i32 as libc::c_uint == 0o100000i32 as libc::c_uint {
        if (st.st_blocks * 512i32 as off_t) < st.st_size {
          bb_error_msg(
            b"%s: file has holes\x00" as *const u8 as *const libc::c_char,
            device,
          );
          return 1i32;
        }
      }
      err = swapon(
        device,
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).flags,
      );
      /* Don't complain on swapon -a if device is already in use */
      quiet = (option_mask32 & OPT_a as libc::c_int as libc::c_uint != 0 && *bb_errno == 16i32)
        as libc::c_int
    }
    /* Don't complain if file does not exist with -e option */
    if err != 0 && option_mask32 & OPT_e as libc::c_int as libc::c_uint != 0 && *bb_errno == 2i32 {
      err = 0i32
    }
  }
  if err != 0 && quiet == 0 {
    bb_simple_perror_msg(device);
    return 1i32;
  }
  return 0i32;
}
unsafe extern "C" fn set_discard_flag(mut s: *mut libc::c_char) {
  /* Unset the flag first to allow fstab options to override */
  /* options set on the command line */
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).flags =
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).flags
      & !(0x10000i32 | 0x20000i32 | 0x40000i32)
      | 0x10000i32;
  if s.is_null() {
    /* No optional policy value on the commandline */
    return;
  }
  /* Skip prepended '=' */
  if *s as libc::c_int == '=' as i32 {
    s = s.offset(1)
  }
  /* For fstab parsing: remove other appended options */
  *strchrnul(s, ',' as i32) = '\u{0}' as i32 as libc::c_char;
  if strcmp(s, b"once\x00" as *const u8 as *const libc::c_char) == 0i32 {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).flags |= 0x20000i32
  }
  if strcmp(s, b"pages\x00" as *const u8 as *const libc::c_char) == 0i32 {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).flags |= 0x40000i32
  };
}
unsafe extern "C" fn set_priority_flag(mut s: *mut libc::c_char) {
  let mut prio: libc::c_uint = 0;
  /* For fstab parsing: remove other appended options */
  *strchrnul(s, ',' as i32) = '\u{0}' as i32 as libc::c_char;
  /* Max allowed 32767 (== SWAP_FLAG_PRIO_MASK) */
  prio = bb_strtou(s, 0 as *mut *mut libc::c_char, 10i32);
  if *bb_errno == 0 {
    /* Unset the flag first to allow fstab options to override */
    /* options set on the command line */
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).flags =
      (((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).flags & !0x7fffi32 | 0x8000i32)
        as libc::c_uint
        | (if prio < 0x7fffi32 as libc::c_uint {
          prio
        } else {
          0x7fffi32 as libc::c_uint
        })) as libc::c_int
  };
}
unsafe extern "C" fn do_em_all_in_fstab() -> libc::c_int {
  let mut m: *mut mntent = 0 as *mut mntent;
  let mut err: libc::c_int = 0i32;
  let mut f: *mut FILE = xfopen_for_read(b"/etc/fstab\x00" as *const u8 as *const libc::c_char);
  loop {
    m = getmntent(f);
    if m.is_null() {
      break;
    }
    if strcmp(
      (*m).mnt_type,
      b"swap\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
      /* swapon -a should ignore entries with noauto,
       * but swapoff -a should process them
       */
      if *applet_name.offset(5) as libc::c_int == 'f' as i32
        || hasmntopt(m, b"noauto\x00" as *const u8 as *const libc::c_char).is_null()
      {
        /* each swap space might have different flags */
        /* save global flags for the next round */
        let mut save_g_flags: libc::c_int =
          (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).flags;
        let mut p: *mut libc::c_char =
          hasmntopt(m, b"discard\x00" as *const u8 as *const libc::c_char);
        if !p.is_null() {
          /* move to '=' or to end of string */
          p = p.offset(7);
          set_discard_flag(p);
        }
        let mut p_0: *mut libc::c_char =
          hasmntopt(m, b"pri\x00" as *const u8 as *const libc::c_char);
        if !p_0.is_null() {
          set_priority_flag(p_0.offset(4));
        }
        err |= swap_enable_disable((*m).mnt_fsname);
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).flags = save_g_flags
      }
    }
  }
  return err;
}
unsafe extern "C" fn do_all_in_proc_swaps() -> libc::c_int {
  let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut err: libc::c_int = 0i32;
  let mut f: *mut FILE = fopen_for_read(b"/proc/swaps\x00" as *const u8 as *const libc::c_char);
  /* Don't complain if missing */
  if !f.is_null() {
    loop {
      line = xmalloc_fgetline(f);
      if line.is_null() {
        break;
      }
      if *line.offset(0) as libc::c_int == '/' as i32 {
        *strchrnul(line, ' ' as i32) = '\u{0}' as i32 as libc::c_char;
        err |= swap_enable_disable(line)
      }
      free(line as *mut libc::c_void);
    }
  }
  return err;
}
#[no_mangle]
pub unsafe extern "C" fn swap_on_off_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut prio: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut discard: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut ret: libc::c_int = 0i32;
  getopt32(
    argv,
    if *applet_name.offset(5) as libc::c_int == 'f' as i32 {
      b"ae\x00" as *const u8 as *const libc::c_char
    } else {
      b"aed::p:\x00" as *const u8 as *const libc::c_char
    },
    &mut discard as *mut *mut libc::c_char,
    &mut prio as *mut *mut libc::c_char,
  );
  argv = argv.offset(optind as isize);
  if option_mask32 & OPT_d as libc::c_int as libc::c_uint != 0 {
    set_discard_flag(discard);
  }
  if option_mask32 & OPT_p as libc::c_int as libc::c_uint != 0 {
    set_priority_flag(prio);
  }
  if option_mask32 & OPT_a as libc::c_int as libc::c_uint != 0 {
    /* swapoff -a does also /proc/swaps */
    if *applet_name.offset(5) as libc::c_int == 'f' as i32 {
      ret = do_all_in_proc_swaps()
    }
    ret |= do_em_all_in_fstab()
  } else if (*argv).is_null() {
    /* if not -a we need at least one arg */
    bb_show_usage();
  }
  /* Unset -a now to allow for more messages in swap_enable_disable */
  option_mask32 = option_mask32 & !(OPT_a as libc::c_int) as libc::c_uint;
  /* Now process devices on the commandline if any */
  while !(*argv).is_null() {
    let fresh0 = argv;
    argv = argv.offset(1);
    ret |= swap_enable_disable(*fresh0)
  }
  return ret;
}
