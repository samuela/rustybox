use crate::libbb::appletlib::applet_name;
use libc;

#[no_mangle]
pub unsafe extern "C" fn freeramdisk_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut fd: libc::c_int = 0;
  fd = crate::libbb::xfuncs_printf::xopen(crate::libbb::single_argv::single_argv(argv), 0o2i32);
  // Act like freeramdisk, fdflush, or both depending on configuration.
  crate::libbb::xfuncs_printf::ioctl_or_perror_and_die(
    fd,
    if 1i32 != 0 && *applet_name.offset(1) as libc::c_int == 'r' as i32 || 1i32 == 0 {
      (0u32 << 0 + 8i32 + 8i32 + 14i32
        | (0x12i32 << 0 + 8i32) as libc::c_uint
        | (97i32 << 0) as libc::c_uint)
        | (0i32 << 0 + 8i32 + 8i32) as libc::c_uint
    } else {
      (0u32 << 0 + 8i32 + 8i32 + 14i32
        | (2i32 << 0 + 8i32) as libc::c_uint
        | (0x4bi32 << 0) as libc::c_uint)
        | (0i32 << 0 + 8i32 + 8i32) as libc::c_uint
    },
    0 as *mut libc::c_void,
    b"%s\x00" as *const u8 as *const libc::c_char,
    *argv.offset(1),
  );
  return 0;
}
