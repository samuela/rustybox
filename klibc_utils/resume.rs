use libc;
extern "C" {
  #[no_mangle]
  fn gnu_dev_major(__dev: libc::dev_t) -> libc::c_uint;
  #[no_mangle]
  fn gnu_dev_minor(__dev: libc::dev_t) -> libc::c_uint;
  #[no_mangle]
  fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
  #[no_mangle]
  fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;

  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  #[no_mangle]
  fn open_read_close(
    filename: *const libc::c_char,
    buf: *mut libc::c_void,
    maxsz: size_t,
  ) -> ssize_t;
  #[no_mangle]
  fn xwrite_str(fd: libc::c_int, str: *const libc::c_char);
  #[no_mangle]
  fn xstrtoull(str: *const libc::c_char, b: libc::c_int) -> libc::c_ulonglong;
  /* Non-aborting kind of convertors: bb_strto[u][l]l */
  /* On exit: errno = 0 only if there was non-empty, '\0' terminated value
   * errno = EINVAL if value was not '\0' terminated, but otherwise ok
   *    Return value is still valid, caller should just check whether end[0]
   *    is a valid terminating char for particular case. OTOH, if caller
   *    requires '\0' terminated input, [s]he can just check errno == 0.
   * errno = ERANGE if value had alphanumeric terminating char ("1234abcg").
   * errno = ERANGE if value is out of range, missing, etc.
   * errno = ERANGE if value had minus sign for strtouXX (even "-0" is not ok )
   *    return value is all-ones in this case.
   */
  #[no_mangle]
  fn bb_strtoull(
    arg: *const libc::c_char,
    endp: *mut *mut libc::c_char,
    base: libc::c_int,
  ) -> libc::c_ulonglong;
  #[no_mangle]
  fn bb_strtou(
    arg: *const libc::c_char,
    endp: *mut *mut libc::c_char,
    base: libc::c_int,
  ) -> libc::c_uint;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_makedev(major: libc::c_uint, minor: libc::c_uint) -> libc::c_ulonglong;
}

use crate::librb::size_t;
use crate::librb::ssize_t;
use libc::stat;

#[inline(always)]
unsafe extern "C" fn bb_strtoul(
  mut arg: *const libc::c_char,
  mut endp: *mut *mut libc::c_char,
  mut base: libc::c_int,
) -> libc::c_ulong {
  return bb_strtoull(arg, endp, base) as libc::c_ulong;
}
/*
 * Copyright (c) 2017 Denys Vlasenko <vda.linux@googlemail.com>
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
//config:config RESUME
//config:	bool "resume (3.2 kb)"
//config:	default y
//config:	help
//config:	Resume from saved "suspend-to-disk" image
//applet:IF_RESUME(APPLET_NOEXEC(resume, resume, BB_DIR_BIN, BB_SUID_DROP, resume))
//kbuild:lib-$(CONFIG_RESUME) += resume.o
/* This is a NOEXEC applet. Be very careful! */
/* name_to_dev_t() in klibc-utils supports extended device name formats,
 * apart from the usual case where /dev/NAME already exists.
 *
 * - device number in hexadecimal represents itself (in libc::dev_t layout).
 * - device number in major:minor decimal represents itself.
 * - if block device (or partition) with this name is found in sysfs.
 * - if /dev/ prefix is not given, it is assumed.
 *
 * klibc-utils also recognizes these, but they don't work
 * for "resume" tool purposes (thus we don't support them (yet?)):
 * - /dev/nfs
 * - /dev/ram (alias to /dev/ram0)
 * - /dev/mtd
 */
unsafe extern "C" fn name_to_dev_t(mut devname: *const libc::c_char) -> libc::dev_t {
  let mut devfile: [libc::c_char; 28] = [0; 28];
  let mut sysname: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut major_num: libc::c_uint = 0;
  let mut minor_num: libc::c_uint = 0;
  let mut st: stat = std::mem::zeroed();
  let mut r: libc::c_int = 0;
  if strncmp(
    devname,
    b"/dev/\x00" as *const u8 as *const libc::c_char,
    5i32 as libc::c_ulong,
  ) != 0i32
  {
    let mut cptr: *mut libc::c_char = 0 as *mut libc::c_char;
    cptr = strchr(devname, ':' as i32);
    if !cptr.is_null() {
      /* Colon-separated decimal device number? */
      *cptr = '\u{0}' as i32 as libc::c_char;
      major_num = bb_strtou(devname, 0 as *mut *mut libc::c_char, 10i32);
      if *bb_errno == 0 {
        minor_num = bb_strtou(cptr.offset(1), 0 as *mut *mut libc::c_char, 10i32)
      }
      *cptr = ':' as i32 as libc::c_char;
      if *bb_errno == 0 {
        return bb_makedev(major_num, minor_num) as libc::dev_t;
      }
    } else {
      /* Hexadecimal device number? */
      let mut res: libc::dev_t = bb_strtoul(devname, 0 as *mut *mut libc::c_char, 16i32);
      if *bb_errno == 0 {
        return res;
      }
    }
    devname = xasprintf(b"/dev/%s\x00" as *const u8 as *const libc::c_char, devname)
  }
  /* Now devname is always "/dev/FOO" */
  if stat(devname, &mut st) == 0i32
    && st.st_mode & 0o170000i32 as libc::c_uint == 0o60000i32 as libc::c_uint
  {
    return st.st_rdev;
  }
  /* Full blockdevs as well as partitions may be visible
   * in /sys/class/block/ even if /dev is not populated.
   */
  sysname = xasprintf(
    b"/sys/class/block/%s/dev\x00" as *const u8 as *const libc::c_char,
    devname.offset(5),
  );
  r = open_read_close(
    sysname,
    devfile.as_mut_ptr() as *mut libc::c_void,
    (::std::mem::size_of::<[libc::c_char; 28]>() as libc::c_ulong)
      .wrapping_sub(1i32 as libc::c_ulong),
  ) as libc::c_int;
  //free(sysname);
  if r > 0i32 {
    devfile[r as usize] = '\u{0}' as i32 as libc::c_char;
    if sscanf(
      devfile.as_mut_ptr(),
      b"%u:%u\x00" as *const u8 as *const libc::c_char,
      &mut major_num as *mut libc::c_uint,
      &mut minor_num as *mut libc::c_uint,
    ) == 2i32
    {
      return bb_makedev(major_num, minor_num) as libc::dev_t;
    }
  }
  return 0i32 as libc::dev_t;
}
//usage:#define resume_trivial_usage
//usage:       "BLOCKDEV [OFFSET]"
//usage:#define resume_full_usage "\n"
//usage:   "\n""Restore system state from 'suspend-to-disk' data in BLOCKDEV"
#[no_mangle]
pub unsafe extern "C" fn resume_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut ofs: libc::c_ulonglong = 0;
  let mut resume_device: libc::dev_t = 0;
  let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut fd: libc::c_int = 0;
  argv = argv.offset(1);
  if (*argv.offset(0)).is_null() {
    bb_show_usage();
  }
  resume_device = name_to_dev_t(*argv.offset(0));
  if gnu_dev_major(resume_device) == 0i32 as libc::c_uint {
    bb_error_msg_and_die(
      b"invalid resume device: %s\x00" as *const u8 as *const libc::c_char,
      *argv.offset(0),
    );
  }
  ofs = if !(*argv.offset(1)).is_null() {
    xstrtoull(*argv.offset(1), 0i32)
  } else {
    0i32 as libc::c_ulonglong
  };
  fd = xopen(
    b"/sys/power/resume\x00" as *const u8 as *const libc::c_char,
    0o1i32,
  );
  s = xasprintf(
    b"%u:%u:%llu\x00" as *const u8 as *const libc::c_char,
    gnu_dev_major(resume_device),
    gnu_dev_minor(resume_device),
    ofs,
  );
  xwrite_str(fd, s);
  /* if write() returns, resume did not succeed */
  return 1i32;
  /* klibc-utils exits -1 aka 255 */
}
