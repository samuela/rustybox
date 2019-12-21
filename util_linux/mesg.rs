use libc;
use libc::fchmod;
use libc::isatty;
use libc::mode_t;
use libc::puts;
use libc::stat;

#[no_mangle]
pub unsafe extern "C" fn mesg_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut sb: stat = std::mem::zeroed();
  let mut m: mode_t = 0;
  let mut c: libc::c_char = 0i32 as libc::c_char;
  argv = argv.offset(1);
  if !(*argv.offset(0)).is_null()
    && (!(*argv.offset(1)).is_null() || {
      c = *(*argv.offset(0)).offset(0);
      (c as libc::c_int != 'y' as i32) && c as libc::c_int != 'n' as i32
    })
  {
    crate::libbb::appletlib::bb_show_usage();
  }
  /* We are a NOFORK applet.
   * (Not that it's very useful, but code is trivially NOFORK-safe).
   * Play nice. Do not leak anything.
   */
  if isatty(0i32) == 0 {
    crate::libbb::verror_msg::bb_simple_error_msg_and_die(
      b"not a tty\x00" as *const u8 as *const libc::c_char,
    );
  }
  crate::libbb::xfuncs_printf::xfstat(
    0i32,
    &mut sb,
    b"stdin\x00" as *const u8 as *const libc::c_char,
  );
  if c as libc::c_int == 0i32 {
    puts(
      if sb.st_mode & (0o200i32 >> 3i32 | 0o200i32 >> 3i32 >> 3i32) as libc::c_uint != 0 {
        b"is y\x00" as *const u8 as *const libc::c_char
      } else {
        b"is n\x00" as *const u8 as *const libc::c_char
      },
    );
    return 0i32;
  }
  m = if c as libc::c_int == 'y' as i32 {
    (sb.st_mode) | (0o200i32 >> 3i32) as libc::c_uint
  } else {
    (sb.st_mode) & !(0o200i32 >> 3i32 | 0o200i32 >> 3i32 >> 3i32) as libc::c_uint
  };
  if fchmod(0i32, m) != 0i32 {
    crate::libbb::perror_nomsg_and_die::bb_perror_nomsg_and_die();
  }
  return 0i32;
}
