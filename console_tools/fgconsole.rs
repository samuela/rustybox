use libc;
use libc::printf;

/*
 * Mini fgconsole implementation for busybox
 *
 * Copyright (C) 2010 by Grigory Batalov <bga@altlinux.org>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config FGCONSOLE
//config:	bool "fgconsole (1.5 kb)"
//config:	default y
//config:	select PLATFORM_LINUX
//config:	help
//config:	This program prints active (foreground) console number.
//applet:IF_FGCONSOLE(APPLET_NOEXEC(fgconsole, fgconsole, BB_DIR_USR_BIN, SUID_DROP, fgconsole))
//kbuild:lib-$(CONFIG_FGCONSOLE) += fgconsole.o
//usage:#define fgconsole_trivial_usage
//usage:	""
//usage:#define fgconsole_full_usage "\n\n"
//usage:	"Get active console"
/* From <linux/vt.h> */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vt_stat {
  pub v_active: libc::c_ushort,
  pub v_signal: libc::c_ushort,
  pub v_state: libc::c_ushort,
  /* vt bitmask */
}
pub type C2RustUnnamed = libc::c_uint;
pub const VT_GETSTATE: C2RustUnnamed = 22019;
/* get global vt state info */
#[no_mangle]
pub unsafe extern "C" fn fgconsole_main(
  mut _argc: libc::c_int,
  mut _argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut vtstat: vt_stat = vt_stat {
    v_active: 0,
    v_signal: 0,
    v_state: 0,
  };
  vtstat.v_active = 0i32 as libc::c_ushort;
  crate::libbb::xfuncs_printf::bb_xioctl(
    crate::libbb::get_console::get_console_fd_or_die(),
    VT_GETSTATE as libc::c_int as libc::c_uint,
    &mut vtstat as *mut vt_stat as *mut libc::c_void,
    b"VT_GETSTATE\x00" as *const u8 as *const libc::c_char,
  );
  printf(
    b"%d\n\x00" as *const u8 as *const libc::c_char,
    vtstat.v_active as libc::c_int,
  );
  return 0i32;
}
