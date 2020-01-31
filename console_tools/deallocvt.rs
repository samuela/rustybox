use libc;
use libc::ptrdiff_t;

/*
 * Disallocate virtual terminal(s)
 *
 * Copyright (C) 2003 by Tito Ragusa <farmatito@tiscali.it>
 * Copyright (C) 1999-2004 by Erik Andersen <andersen@codepoet.org>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config DEALLOCVT
//config:	bool "deallocvt (1.9 kb)"
//config:	default y
//config:	select PLATFORM_LINUX
//config:	help
//config:	This program deallocates unused virtual consoles.
//applet:IF_DEALLOCVT(APPLET_NOEXEC(deallocvt, deallocvt, BB_DIR_USR_BIN, SUID_DROP, deallocvt))
//kbuild:lib-$(CONFIG_DEALLOCVT) += deallocvt.o
//usage:#define deallocvt_trivial_usage
//usage:       "[N]"
//usage:#define deallocvt_full_usage "\n\n"
//usage:       "Deallocate unused virtual terminal /dev/ttyN"
/* From <linux/vt.h> */
pub type C2RustUnnamed = libc::c_uint;
pub const VT_DISALLOCATE: C2RustUnnamed = 22024;
/* free memory associated to vt */

pub unsafe fn deallocvt_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  /* num = 0 deallocate all unused consoles */
  let mut num: libc::c_int = 0;
  if !(*argv.offset(1)).is_null() {
    if !(*argv.offset(2)).is_null() {
      crate::libbb::appletlib::bb_show_usage();
    }
    num = crate::libbb::xatonum::xatou_range(
      *argv.offset(1),
      1i32 as libc::c_uint,
      63i32 as libc::c_uint,
    ) as libc::c_int
  }
  /* double cast suppresses "cast to ptr from int of different size" */
  crate::libbb::xfuncs_printf::bb_xioctl(
    crate::libbb::get_console::get_console_fd_or_die(),
    VT_DISALLOCATE as libc::c_int as libc::c_uint,
    num as ptrdiff_t as *mut libc::c_void,
    b"VT_DISALLOCATE\x00" as *const u8 as *const libc::c_char,
  );
  return 0;
}
