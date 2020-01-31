use libc;

pub unsafe fn chvt_main(mut _argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
  let mut num: libc::c_int =
    crate::libbb::xatonum::xatou_range(crate::libbb::single_argv::single_argv(argv), 1, 63)
      as libc::c_int;
  crate::libbb::get_console::console_make_active(
    crate::libbb::get_console::get_console_fd_or_die(),
    num,
  );
  return 0;
}
