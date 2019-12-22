use libc;
use libc::strcpy;
extern "C" {

  #[no_mangle]
  fn strnlen(__string: *const libc::c_char, __maxlen: size_t) -> size_t;

}

use crate::librb::size_t;
/* ubirename - port of the ubirename from the mtd-utils package
 *
 * A utility to rename one UBI volume.
 *
 * 2016-03-01 Sven Eisenberg <sven.eisenberg@novero.com>
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
//config:config UBIRENAME
//config:	bool "ubirename (2.4 kb)"
//config:	default y
//config:	select PLATFORM_LINUX
//config:	help
//config:	Utility to rename UBI volumes
//applet:IF_UBIRENAME(APPLET(ubirename, BB_DIR_USR_SBIN, SUID_DROP))
/* not NOEXEC: if flash operation stalls, use less memory in "hung" process */
//kbuild:lib-$(CONFIG_UBIRENAME) += ubirename.o
//usage:#define ubirename_trivial_usage
//usage:	"UBI_DEVICE OLD_VOLNAME NEW_VOLNAME [OLD2 NEW2]..."
//usage:#define ubirename_full_usage "\n\n"
//usage:	"Rename UBI volumes on UBI_DEVICE"
// from ubi-media.h
// end ubi-media.h
// from ubi-user.h
/* ioctl commands of UBI character devices */
/* Re-name volumes */
/* Maximum amount of UBI volumes that can be re-named at one go */

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ubi_rnvol_req {
  pub count: i32,
  pub padding1: [i8; 12],
  pub ents: [C2RustUnnamed; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed {
  pub vol_id: i32,
  pub name_len: i16,
  pub padding2: [i8; 2],
  pub name: [libc::c_char; 128],
}
// end ubi-user.h
#[no_mangle]
pub unsafe extern "C" fn ubirename_main(
  mut argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut rnvol: *mut ubi_rnvol_req = std::ptr::null_mut();
  let mut ubi_devname: *const libc::c_char = std::ptr::null();
  let mut ubi_devnum: libc::c_uint = 0;
  let mut n: libc::c_uint = 0;
  /* argc can be 4, 6, 8, ... */
  if argc & 1i32 != 0 || {
    argc >>= 1i32;
    (argc) < 2i32
  } {
    crate::libbb::appletlib::bb_show_usage();
  }
  rnvol =
    crate::libbb::xfuncs_printf::xzalloc(::std::mem::size_of::<ubi_rnvol_req>() as libc::c_ulong)
      as *mut ubi_rnvol_req;
  argc -= 1;
  (*rnvol).count = argc;
  if argc as libc::c_uint
    > (::std::mem::size_of::<[C2RustUnnamed; 32]>() as libc::c_ulong)
      .wrapping_div(::std::mem::size_of::<C2RustUnnamed>() as libc::c_ulong) as libc::c_uint
  {
    crate::libbb::verror_msg::bb_simple_error_msg_and_die(
      b"too many renames requested\x00" as *const u8 as *const libc::c_char,
    );
  }
  ubi_devname = *argv.offset(1);
  ubi_devnum = crate::libbb::ubi::ubi_devnum_from_devname(ubi_devname);
  n = 0 as libc::c_uint;
  argv = argv.offset(2);
  while !(*argv.offset(0)).is_null() {
    (*rnvol).ents[n as usize].vol_id =
      crate::libbb::ubi::ubi_get_volid_by_name(ubi_devnum, *argv.offset(0));
    /* strnlen avoids overflow of 16-bit field (paranoia) */
    (*rnvol).ents[n as usize].name_len = strnlen(
      *argv.offset(1),
      ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
    ) as i16;
    if (*rnvol).ents[n as usize].name_len as libc::c_ulong
      >= ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong
    {
      crate::libbb::verror_msg::bb_error_msg_and_die(
        b"new name \'%s\' is too long\x00" as *const u8 as *const libc::c_char,
        *argv.offset(1),
      );
    }
    strcpy((*rnvol).ents[n as usize].name.as_mut_ptr(), *argv.offset(1));
    n = n.wrapping_add(1);
    argv = argv.offset(2)
  }
  crate::libbb::xfuncs_printf::bb_xioctl(
    crate::libbb::xfuncs_printf::xopen(ubi_devname, 0),
    ((1u32 << 0 + 8i32 + 8i32 + 14i32
      | (('o' as i32) << 0 + 8i32) as libc::c_uint
      | (3i32 << 0) as libc::c_uint) as libc::c_ulong
      | (::std::mem::size_of::<ubi_rnvol_req>() as libc::c_ulong) << 0 + 8i32 + 8i32)
      as libc::c_uint,
    rnvol as *mut libc::c_void,
    b"UBI_IOCRNVOL\x00" as *const u8 as *const libc::c_char,
  );
  return 0;
}
