use libc;












































extern "C" {
  #[no_mangle]
  fn get_console_fd_or_die() -> libc::c_int;

  #[no_mangle]
  fn console_make_active(fd: libc::c_int, vt_num: libc::c_int);

  #[no_mangle]
  fn xatou_range(str: *const libc::c_char, l: libc::c_uint, u: libc::c_uint) -> libc::c_uint;

  #[no_mangle]
  fn single_argv(argv: *mut *mut libc::c_char) -> *mut libc::c_char;
}

#[no_mangle]
pub unsafe extern "C" fn chvt_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut num: libc::c_int = xatou_range(single_argv(argv), 1, 63) as libc::c_int;
  console_make_active(get_console_fd_or_die(), num);
  return 0;
}
