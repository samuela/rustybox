use libc;











































extern "C" {
  #[no_mangle]
  fn gnu_dev_makedev(__major: libc::c_uint, __minor: libc::c_uint) -> libc::dev_t;
}

/*
 * Utility routines.
 *
 * Copyright (C) 2006 Denys Vlasenko
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
/* We do not include libbb.h - #define makedev() is there! */
/* Different Unixes want different headers for makedev */
/* At least glibc has horrendously large inline for this, so wrap it. */
/* uclibc people please check - do we need "&& !__UCLIBC__" above? */
/* Suppress gcc "no previous prototype" warning */
#[no_mangle]
pub unsafe extern "C" fn bb_makedev(
  mut major: libc::c_uint,
  mut minor: libc::c_uint,
) -> libc::c_ulonglong {
  return gnu_dev_makedev(major, minor) as libc::c_ulonglong;
}
