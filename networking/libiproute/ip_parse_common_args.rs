use crate::librb::smallint;
use libc;

/* NB: unaligned parameter should be a pointer, aligned one -
 * a lvalue. This makes it more likely to not swap them by mistake
 */
/* #elif ... - add your favorite arch today! */
/* Unaligned, fixed-endian accessors */
/* unxz needs an aligned fixed-endian accessor.
 * (however, the compiler does not realize it's aligned, the cast is still necessary)
 */
/* ---- Size-saving "small" ints (arch-dependent) ----------- */
/* add other arches which benefit from this... */
pub type family_t = i8;
pub const ARG_IPv4: C2RustUnnamed = 2;
pub const ARG_family: C2RustUnnamed = 1;
pub const ARG_oneline: C2RustUnnamed = 0;
pub type C2RustUnnamed = libc::c_uint;
pub const ARG_packet: C2RustUnnamed = 4;
pub const ARG_IPv6: C2RustUnnamed = 3;

/*
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License
 * as published by the Free Software Foundation; either version
 * 2 of the License, or (at your option) any later version.
 *
 * Authors: Alexey Kuznetsov, <kuznet@ms2.inr.ac.ru>
 *
 * Changes:
 *
 * Rani Assaf <rani@magic.metawire.com> 980929: resolve addresses
 */
#[no_mangle]
pub static mut preferred_family: family_t = 0 as family_t;
#[no_mangle]
pub static mut oneline: smallint = 0;
#[no_mangle]
pub static mut _SL_: libc::c_char = 0;

pub unsafe fn ip_parse_common_args(mut argv: *mut *mut libc::c_char) -> *mut *mut libc::c_char {
  static mut ip_common_commands: [libc::c_char; 22] = [
    111, 110, 101, 108, 105, 110, 101, 0, 102, 97, 109, 105, 108, 121, 0, 52, 0, 54, 0, 48, 0, 0,
  ];
  static mut af_numbers: [family_t; 3] = [2i32 as family_t, 10i32 as family_t, 17i32 as family_t];
  let mut arg: libc::c_int = 0;
  while !(*argv).is_null() {
    let mut opt: *mut libc::c_char = *argv;
    if *opt.offset(0) as libc::c_int != '-' as i32 {
      break;
    }
    opt = opt.offset(1);
    if *opt.offset(0) as libc::c_int == '-' as i32 {
      opt = opt.offset(1);
      if *opt.offset(0) == 0 {
        /* "--" */
        argv = argv.offset(1);
        break;
      }
    }
    arg = crate::libbb::compare_string_array::index_in_substrings(ip_common_commands.as_ptr(), opt);
    if arg < 0 {
      crate::libbb::appletlib::bb_show_usage();
    }
    if arg == ARG_oneline as libc::c_int {
      oneline = 1i32 as smallint;
      argv = argv.offset(1)
    } else {
      if arg == ARG_family as libc::c_int {
        static mut families: [libc::c_char; 17] = [
          105, 110, 101, 116, 0, 105, 110, 101, 116, 54, 0, 108, 105, 110, 107, 0, 0,
        ];
        argv = argv.offset(1);
        if (*argv).is_null() {
          crate::libbb::appletlib::bb_show_usage();
        }
        arg = crate::libbb::compare_string_array::index_in_strings(families.as_ptr(), *argv);
        if arg < 0 {
          crate::networking::libiproute::utils::invarg_1_to_2(
            *argv,
            b"family\x00" as *const u8 as *const libc::c_char,
          );
        }
      /* now arg == 0, 1 or 2 */
      } else {
        arg -= ARG_IPv4 as libc::c_int
        /* now arg == 0, 1 or 2 */
      }
      preferred_family = af_numbers[arg as usize];
      argv = argv.offset(1)
    }
  }
  _SL_ = if oneline as libc::c_int != 0 {
    '\\' as i32
  } else {
    '\n' as i32
  } as libc::c_char;
  return argv;
}
