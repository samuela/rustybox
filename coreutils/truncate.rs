use libc;




extern "C" {
  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  fn ftruncate(__fd: libc::c_int, __length: off64_t) -> libc::c_int;

  #[no_mangle]
  fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;

  #[no_mangle]
  static bb_errno: *mut libc::c_int;

  /* Close fd, but check for failures (some types of write errors) */
  #[no_mangle]
  fn xclose(fd: libc::c_int);

  #[no_mangle]
  static cwbkMG_suffixes: [suffix_mult; 0];

  #[no_mangle]
  fn xatoull_sfx(str: *const libc::c_char, sfx: *const suffix_mult) -> libc::c_ulonglong;

  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;

  #[no_mangle]
  fn bb_perror_msg(s: *const libc::c_char, _: ...);
}

use libc::off64_t;
use libc::off_t;


/* Last element is marked by mult == 0 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct suffix_mult {
  pub suffix: [libc::c_char; 4],
  pub mult: libc::c_uint,
}

pub const OPT_NOCREATE: C2RustUnnamed = 1;
pub type C2RustUnnamed = libc::c_uint;
pub const OPT_SIZE: C2RustUnnamed = 2;
/* This is a NOFORK applet. Be very careful! */

#[no_mangle]
pub unsafe extern "C" fn truncate_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut opts: libc::c_uint = 0;
  let mut flags: libc::c_int = 0o1i32 | 0o4000i32;
  let mut ret: libc::c_int = 0i32;
  let mut size_str: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut size: off_t = 0;
  opts = getopt32(
    argv,
    b"^cs:\x00s:-1\x00" as *const u8 as *const libc::c_char,
    &mut size_str as *mut *mut libc::c_char,
  );
  if opts & OPT_NOCREATE as libc::c_int as libc::c_uint == 0 {
    flags |= 0o100i32
  }
  // TODO: coreutils 8.17 also support "m" (lowercase) suffix
  // with truncate, but not with dd!
  // We share kMG_suffixes[], so we can't make both tools
  // compatible at once...
  size = xatoull_sfx(size_str, cwbkMG_suffixes.as_ptr().offset(3)) as off_t;
  argv = argv.offset(optind as isize);
  while !(*argv).is_null() {
    let mut fd: libc::c_int = open(*argv, flags, 0o666i32);
    if fd < 0i32 {
      if *bb_errno != 2i32 || opts & OPT_NOCREATE as libc::c_int as libc::c_uint == 0 {
        bb_perror_msg(b"%s: open\x00" as *const u8 as *const libc::c_char, *argv);
        ret = 1i32
      }
    /* else: ENOENT && OPT_NOCREATE:
     * do not report error, exitcode is also 0.
     */
    } else {
      if ftruncate(fd, size) == -1i32 {
        bb_perror_msg(
          b"%s: truncate\x00" as *const u8 as *const libc::c_char,
          *argv,
        );
        ret = 1i32
      }
      xclose(fd);
    }
    argv = argv.offset(1)
  }
  return ret;
}
