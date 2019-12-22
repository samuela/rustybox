use crate::libbb::xfuncs_printf::xmalloc;
use crate::librb::size_t;
use libc;
use libc::ioctl;

/* From <linux/kd.h> */

#[repr(C)]
#[derive(Copy, Clone)]
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
  let mut ibuff: [u16; 128] = [0; 128];
  /*	const char *tty_name = CURRENT_TTY; */
  let mut flags: *mut libc::c_char = xmalloc(256i32 as size_t) as *mut libc::c_char;
  /* When user accidentally runs "loadkmap FILE"
   * instead of "loadkmap <FILE", we end up waiting for input from tty.
   * Let's prevent it: */
  if !(*argv.offset(1)).is_null() {
    crate::libbb::appletlib::bb_show_usage();
  }
  /* bb_warn_ignoring_args(argv[1]); */
  fd = crate::libbb::get_console::get_console_fd_or_die();
  /* or maybe:
    opt = getopt32(argv, "C:", &tty_name);
    fd = xopen_nonblocking(tty_name);
  */
  crate::libbb::read_printf::xread(0i32, flags as *mut libc::c_void, 7i32 as size_t);
  if crate::libbb::compare_string_array::is_prefixed_with(
    flags,
    b"bkeymap\x00" as *const u8 as *const libc::c_char,
  )
  .is_null()
  {
    crate::libbb::verror_msg::bb_simple_error_msg_and_die(
      b"not a valid binary keymap\x00" as *const u8 as *const libc::c_char,
    );
  }
  crate::libbb::read_printf::xread(0i32, flags as *mut libc::c_void, 256i32 as size_t);
  i = 0;
  while i < 256i32 {
    if !(*flags.offset(i as isize) as libc::c_int != 1i32) {
      crate::libbb::read_printf::xread(
        0,
        ibuff.as_mut_ptr() as *mut libc::c_void,
        (128i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<u16>() as libc::c_ulong),
      );
      j = 0;
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
  return 0;
}
