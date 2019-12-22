use crate::libbb::parse_config::parser_t;
use libc;
use libc::free;
use libc::printf;
use libc::stat;
use libc::strcmp;
use libc::FILE;

pub type C2RustUnnamed = libc::c_uint;
// pub const ACTION_DANGLING_OK: C2RustUnnamed = 64;
// pub const ACTION_QUIET: C2RustUnnamed = 32;
// pub const ACTION_DEPTHFIRST: C2RustUnnamed = 8;
// pub const ACTION_FOLLOWLINKS_L0: C2RustUnnamed = 4;
// pub const ACTION_FOLLOWLINKS: C2RustUnnamed = 2;
pub const ACTION_RECURSE: C2RustUnnamed = 1;

pub type C2RustUnnamed_0 = libc::c_uint;
pub const PARSE_NORMAL: C2RustUnnamed_0 = 4653056;
// pub const PARSE_WS_COMMENTS: C2RustUnnamed_0 = 16777216;
// pub const PARSE_ALT_COMMENTS: C2RustUnnamed_0 = 8388608;
// pub const PARSE_EOL_COMMENTS: C2RustUnnamed_0 = 4194304;
// pub const PARSE_KEEP_COPY: C2RustUnnamed_0 = 2097152;
// pub const PARSE_MIN_DIE: C2RustUnnamed_0 = 1048576;
// pub const PARSE_GREEDY: C2RustUnnamed_0 = 262144;
// pub const PARSE_TRIM: C2RustUnnamed_0 = 131072;
// pub const PARSE_COLLAPSE: C2RustUnnamed_0 = 65536;

/*
 * lsusb implementation for busybox
 *
 * Copyright (C) 2009  Malek Degachi <malek-degachi@laposte.net>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config LSUSB
//config:	bool "lsusb (4.2 kb)"
//config:	default y
//config:	#select PLATFORM_LINUX
//config:	help
//config:	lsusb is a utility for displaying information about USB buses in the
//config:	system and devices connected to them.
//config:
//config:	This version uses sysfs (/sys/bus/usb/devices) only.
//applet:IF_LSUSB(APPLET_NOEXEC(lsusb, lsusb, BB_DIR_USR_BIN, SUID_DROP, lsusb))
//kbuild:lib-$(CONFIG_LSUSB) += lsusb.o
//usage:#define lsusb_trivial_usage NOUSAGE_STR
//usage:#define lsusb_full_usage ""
unsafe extern "C" fn fileAction(
  mut fileName: *const libc::c_char,
  mut _statbuf: *mut stat,
  mut _userData: *mut libc::c_void,
  mut _depth: libc::c_int,
) -> libc::c_int {
  let mut parser: *mut parser_t = std::ptr::null_mut();
  let mut tokens: [*mut libc::c_char; 4] = [0 as *mut libc::c_char; 4];
  let mut busnum: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut devnum: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut product_vid: libc::c_int = 0;
  let mut product_did: libc::c_int = 0;
  let mut uevent_filename: *mut libc::c_char = crate::libbb::concat_path_file::concat_path_file(
    fileName,
    b"/uevent\x00" as *const u8 as *const libc::c_char,
  );
  parser = crate::libbb::parse_config::config_open2(
    uevent_filename,
    Some(
      crate::libbb::wfopen::fopen_for_read
        as unsafe extern "C" fn(_: *const libc::c_char) -> *mut FILE,
    ),
  );
  free(uevent_filename as *mut libc::c_void);
  while crate::libbb::parse_config::config_read(
    parser,
    tokens.as_mut_ptr(),
    (PARSE_NORMAL as libc::c_int | (2i32 & 0xffi32) << 8i32 | 4i32 & 0xffi32) as libc::c_uint,
    b"\\/=\x00" as *const u8 as *const libc::c_char,
  ) != 0
  {
    if (*parser).lineno == 1i32
      && strcmp(
        tokens[0],
        b"DEVTYPE\x00" as *const u8 as *const libc::c_char,
      ) == 0
    {
      break;
    }
    if strcmp(
      tokens[0],
      b"PRODUCT\x00" as *const u8 as *const libc::c_char,
    ) == 0
    {
      product_vid = crate::libbb::xatonum::xstrtou(tokens[1], 16i32) as libc::c_int;
      product_did = crate::libbb::xatonum::xstrtou(tokens[2], 16i32) as libc::c_int
    } else if strcmp(tokens[0], b"BUSNUM\x00" as *const u8 as *const libc::c_char) == 0 {
      busnum = crate::libbb::xfuncs_printf::xstrdup(tokens[1])
    } else {
      if !(strcmp(tokens[0], b"DEVNUM\x00" as *const u8 as *const libc::c_char) == 0) {
        continue;
      }
      devnum = crate::libbb::xfuncs_printf::xstrdup(tokens[1])
    }
  }
  crate::libbb::parse_config::config_close(parser);
  if !busnum.is_null() {
    printf(
      b"Bus %s Device %s: ID %04x:%04x\n\x00" as *const u8 as *const libc::c_char,
      busnum,
      devnum,
      product_vid,
      product_did,
    );
    free(busnum as *mut libc::c_void);
    free(devnum as *mut libc::c_void);
  }
  return 1i32;
}

#[no_mangle]
pub unsafe extern "C" fn lsusb_main(
  mut _argc: libc::c_int,
  mut _argv: *mut *mut libc::c_char,
) -> libc::c_int {
  /* no options, no getopt */
  crate::libbb::recursive_action::recursive_action(
    b"/sys/bus/usb/devices\x00" as *const u8 as *const libc::c_char,
    ACTION_RECURSE as libc::c_int as libc::c_uint,
    Some(
      fileAction
        as unsafe extern "C" fn(
          _: *const libc::c_char,
          _: *mut stat,
          _: *mut libc::c_void,
          _: libc::c_int,
        ) -> libc::c_int,
    ),
    None,
    0 as *mut libc::c_void,
    0 as libc::c_uint,
  );
  return 0;
}
