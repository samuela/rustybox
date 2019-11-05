use libc;
extern "C" {
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
  #[no_mangle]
  fn xmalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn get_console_fd_or_die() -> libc::c_int;
  #[no_mangle]
  fn is_prefixed_with(string: *const libc::c_char, key: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xread(fd: libc::c_int, buf: *mut libc::c_void, count: size_t);
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
}


use crate::librb::size_t;
use crate::librb::uint16_t;

/* From <linux/kd.h> */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kbentry {
  pub kb_table: libc::c_uchar,
  pub kb_index: libc::c_uchar,
  pub kb_value: libc::c_ushort,
}
#[no_mangle]
pub unsafe extern "C" fn loadkmap_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut ke: kbentry = kbentry {
    kb_table: 0,
    kb_index: 0,
    kb_value: 0,
  };
  let mut i: libc::c_int = 0;
  let mut j: libc::c_int = 0;
  let mut fd: libc::c_int = 0;
  let mut ibuff: [uint16_t; 128] = [0; 128];
  /*	const char *tty_name = CURRENT_TTY; */
  let mut flags: *mut libc::c_char = xmalloc(256i32 as size_t) as *mut libc::c_char;
  /* When user accidentally runs "loadkmap FILE"
   * instead of "loadkmap <FILE", we end up waiting for input from tty.
   * Let's prevent it: */
  if !(*argv.offset(1)).is_null() {
    bb_show_usage();
  }
  /* bb_warn_ignoring_args(argv[1]); */
  fd = get_console_fd_or_die();
  /* or maybe:
    opt = getopt32(argv, "C:", &tty_name);
    fd = xopen_nonblocking(tty_name);
  */
  xread(0i32, flags as *mut libc::c_void, 7i32 as size_t);
  if is_prefixed_with(flags, b"bkeymap\x00" as *const u8 as *const libc::c_char).is_null() {
    bb_simple_error_msg_and_die(
      b"not a valid binary keymap\x00" as *const u8 as *const libc::c_char,
    );
  }
  xread(0i32, flags as *mut libc::c_void, 256i32 as size_t);
  i = 0i32;
  while i < 256i32 {
    if !(*flags.offset(i as isize) as libc::c_int != 1i32) {
      xread(
        0i32,
        ibuff.as_mut_ptr() as *mut libc::c_void,
        (128i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<uint16_t>() as libc::c_ulong),
      );
      j = 0i32;
      while j < 128i32 {
        ke.kb_index = j as libc::c_uchar;
        ke.kb_table = i as libc::c_uchar;
        ke.kb_value = ibuff[j as usize];
        /*
         * Note: table[idx:0] can contain special value
         * K_ALLOCATED (marks allocated tables in kernel).
         * dumpkmap saves the value as-is; but attempts
         * to load it here fail, since it isn't a valid
         * key value: it is K(KT_SPEC,126) == 2<<8 + 126,
         * whereas last valid KT_SPEC is
         * K_BARENUMLOCK == K(KT_SPEC,19).
         * So far we just ignore these errors:
         */
        ioctl(fd, 0x4b47i32 as libc::c_ulong, &mut ke as *mut kbentry);
        j += 1
      }
    }
    i += 1
  }
  return 0i32;
}
