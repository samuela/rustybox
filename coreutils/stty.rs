use libc;
extern "C" {
  #[no_mangle]
  static mut stdout: *mut FILE;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn vsnprintf(
    _: *mut libc::c_char,
    _: libc::c_ulong,
    _: *const libc::c_char,
    _: ::std::ffi::VaList,
  ) -> libc::c_int;
  #[no_mangle]
  fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
  #[no_mangle]
  fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
  #[no_mangle]
  fn cfgetospeed(__termios_p: *const termios) -> speed_t;
  #[no_mangle]
  fn cfgetispeed(__termios_p: *const termios) -> speed_t;
  #[no_mangle]
  fn cfsetospeed(__termios_p: *mut termios, __speed: speed_t) -> libc::c_int;
  #[no_mangle]
  fn cfsetispeed(__termios_p: *mut termios, __speed: speed_t) -> libc::c_int;
  #[no_mangle]
  fn tcgetattr(__fd: libc::c_int, __termios_p: *mut termios) -> libc::c_int;
  #[no_mangle]
  fn tcsetattr(
    __fd: libc::c_int,
    __optional_actions: libc::c_int,
    __termios_p: *const termios,
  ) -> libc::c_int;
  /* Some useful definitions */
  /* Macros for min/max.  */
  /* buffer allocation schemes */
  /* glibc uses __errno_location() to get a ptr to errno */
  /* We can just memorize it once - no multithreading in busybox :) */
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn ndelay_off(fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xmove_fd(_: libc::c_int, _: libc::c_int);
  #[no_mangle]
  fn xopen_nonblocking(pathname: *const libc::c_char) -> libc::c_int;
  /* Guaranteed to NOT be a macro (smallest code). Saves nearly 2k on uclibc.
   * But potentially slow, don't use in one-billion-times loops */
  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn visible(ch: libc::c_uint, buf: *mut libc::c_char, flags: libc::c_int);
  #[no_mangle]
  fn xatoull_range_sfx(
    str: *const libc::c_char,
    l: libc::c_ulonglong,
    u: libc::c_ulonglong,
    sfx: *const suffix_mult,
  ) -> libc::c_ulonglong;
  #[no_mangle]
  fn xatoull_sfx(str: *const libc::c_char, sfx: *const suffix_mult) -> libc::c_ulonglong;
  #[no_mangle]
  fn xatou(str: *const libc::c_char) -> libc::c_uint;
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_perror_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_perror_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn tty_baud_to_value(speed: speed_t) -> libc::c_uint;
  #[no_mangle]
  fn tty_value_to_baud(value: libc::c_uint) -> speed_t;
  #[no_mangle]
  fn index_in_strings(strings: *const libc::c_char, key: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn nth_string(strings: *const libc::c_char, n: libc::c_int) -> *const libc::c_char;
  /* NB: typically you want to pass fd 0, not 1. Think 'applet | grep something' */
  #[no_mangle]
  fn get_terminal_width_height(
    fd: libc::c_int,
    width: *mut libc::c_uint,
    height: *mut libc::c_uint,
  ) -> libc::c_int;
  #[no_mangle]
  fn get_terminal_width(fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  static bb_msg_requires_arg: [libc::c_char; 0];
  #[no_mangle]
  static bb_msg_standard_input: [libc::c_char; 0];
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
  pub gp_offset: libc::c_uint,
  pub fp_offset: libc::c_uint,
  pub overflow_arg_area: *mut libc::c_void,
  pub reg_save_area: *mut libc::c_void,
}

use crate::librb::uint16_t;
use crate::librb::uint8_t;

use crate::librb::FILE;
pub type va_list = __builtin_va_list;
use crate::librb::cc_t;
use crate::librb::speed_t;
use crate::librb::tcflag_t;
use crate::librb::termios;
use crate::librb::winsize;
/* Last element is marked by mult == 0 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct suffix_mult {
  pub suffix: [libc::c_char; 4],
  pub mult: libc::c_uint,
}
//extern const int const_int_1;
/* This struct is deliberately not defined. */
/* See docs/keep_data_small.txt */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub device_name: *const libc::c_char,
  pub max_col: libc::c_uint,
  pub current_col: libc::c_uint,
}
pub type speed_setting = libc::c_uint;
pub const both_speeds: speed_setting = 2;
pub const output_speed: speed_setting = 1;
pub const input_speed: speed_setting = 0;
pub type C2RustUnnamed = libc::c_uint;
pub const combination: C2RustUnnamed = 4;
pub const local: C2RustUnnamed = 3;
pub const output: C2RustUnnamed = 2;
pub const input: C2RustUnnamed = 1;
pub const control: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct control_info {
  pub saneval: uint8_t,
  pub offset: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mode_info {
  pub type_0: uint8_t,
  pub flags: uint8_t,
  pub mask: uint16_t,
  pub bits: tcflag_t,
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const IDX_LCASE: C2RustUnnamed_0 = 16;
pub const IDX_lcase: C2RustUnnamed_0 = 15;
pub const IDX_tabs: C2RustUnnamed_0 = 14;
pub const IDX_decctlq: C2RustUnnamed_0 = 13;
pub const IDX_dec: C2RustUnnamed_0 = 12;
pub const IDX_crt: C2RustUnnamed_0 = 11;
pub const IDX_cbreak: C2RustUnnamed_0 = 10;
pub const IDX_litout: C2RustUnnamed_0 = 9;
pub const IDX_pass8: C2RustUnnamed_0 = 8;
pub const IDX_raw: C2RustUnnamed_0 = 7;
pub const IDX_cooked: C2RustUnnamed_0 = 6;
pub const IDX_sane: C2RustUnnamed_0 = 5;
pub const IDX_ek: C2RustUnnamed_0 = 4;
pub const IDX_nl: C2RustUnnamed_0 = 3;
pub const IDX_oddp: C2RustUnnamed_0 = 2;
pub const IDX_parity: C2RustUnnamed_0 = 1;
pub const IDX_evenp: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const NUM_mode_info: C2RustUnnamed_1 = 89;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const CIDX_time: C2RustUnnamed_2 = 16;
pub const CIDX_min: C2RustUnnamed_2 = 15;
pub const CIDX_flush: C2RustUnnamed_2 = 14;
pub const CIDX_lnext: C2RustUnnamed_2 = 13;
pub const CIDX_werase: C2RustUnnamed_2 = 12;
pub const CIDX_rprnt: C2RustUnnamed_2 = 11;
pub const CIDX_susp: C2RustUnnamed_2 = 10;
pub const CIDX_stop: C2RustUnnamed_2 = 9;
pub const CIDX_start: C2RustUnnamed_2 = 8;
pub const CIDX_swtch: C2RustUnnamed_2 = 7;
pub const CIDX_eol2: C2RustUnnamed_2 = 6;
pub const CIDX_eol: C2RustUnnamed_2 = 5;
pub const CIDX_eof: C2RustUnnamed_2 = 4;
pub const CIDX_kill: C2RustUnnamed_2 = 3;
pub const CIDX_erase: C2RustUnnamed_2 = 2;
pub const CIDX_quit: C2RustUnnamed_2 = 1;
pub const CIDX_intr: C2RustUnnamed_2 = 0;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const NUM_control_info: C2RustUnnamed_3 = 17;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const param_ospeed: C2RustUnnamed_4 = 136;
pub const param_ispeed: C2RustUnnamed_4 = 135;
pub const param_speed: C2RustUnnamed_4 = 6;
pub const param_size: C2RustUnnamed_4 = 5;
pub const param_columns: C2RustUnnamed_4 = 132;
pub const param_cols: C2RustUnnamed_4 = 131;
pub const param_rows: C2RustUnnamed_4 = 130;
pub const param_line: C2RustUnnamed_4 = 129;
pub const param_need_arg: C2RustUnnamed_4 = 128;
#[inline(always)]
unsafe extern "C" fn xatoul_range_sfx(
  mut str: *const libc::c_char,
  mut l: libc::c_ulong,
  mut u: libc::c_ulong,
  mut sfx: *const suffix_mult,
) -> libc::c_ulong {
  return xatoull_range_sfx(str, l as libc::c_ulonglong, u as libc::c_ulonglong, sfx)
    as libc::c_ulong;
}
#[inline(always)]
unsafe extern "C" fn xatoul_sfx(
  mut str: *const libc::c_char,
  mut sfx: *const suffix_mult,
) -> libc::c_ulong {
  return xatoull_sfx(str, sfx) as libc::c_ulong;
}
unsafe extern "C" fn get_ptr_to_tcflag(
  mut type_0: libc::c_uint,
  mut mode: *const termios,
) -> *mut tcflag_t {
  static mut tcflag_offsets: [uint8_t; 4] = [
    8u64 as uint8_t,
    0u64 as uint8_t,
    4u64 as uint8_t,
    12u64 as uint8_t,
  ];
  if type_0 <= local as libc::c_int as libc::c_uint {
    return (mode as *mut libc::c_char)
      .offset(tcflag_offsets[type_0 as usize] as libc::c_int as isize) as *mut tcflag_t;
  }
  return 0 as *mut tcflag_t;
}
static mut mode_name: [libc::c_char; 524] = [
  101, 118, 101, 110, 112, 0, 112, 97, 114, 105, 116, 121, 0, 111, 100, 100, 112, 0, 110, 108, 0,
  101, 107, 0, 115, 97, 110, 101, 0, 99, 111, 111, 107, 101, 100, 0, 114, 97, 119, 0, 112, 97, 115,
  115, 56, 0, 108, 105, 116, 111, 117, 116, 0, 99, 98, 114, 101, 97, 107, 0, 99, 114, 116, 0, 100,
  101, 99, 0, 100, 101, 99, 99, 116, 108, 113, 0, 116, 97, 98, 115, 0, 108, 99, 97, 115, 101, 0,
  76, 67, 65, 83, 69, 0, 112, 97, 114, 101, 110, 98, 0, 112, 97, 114, 111, 100, 100, 0, 99, 109,
  115, 112, 97, 114, 0, 99, 115, 53, 0, 99, 115, 54, 0, 99, 115, 55, 0, 99, 115, 56, 0, 104, 117,
  112, 99, 108, 0, 104, 117, 112, 0, 99, 115, 116, 111, 112, 98, 0, 99, 114, 101, 97, 100, 0, 99,
  108, 111, 99, 97, 108, 0, 99, 114, 116, 115, 99, 116, 115, 0, 105, 103, 110, 98, 114, 107, 0, 98,
  114, 107, 105, 110, 116, 0, 105, 103, 110, 112, 97, 114, 0, 112, 97, 114, 109, 114, 107, 0, 105,
  110, 112, 99, 107, 0, 105, 115, 116, 114, 105, 112, 0, 105, 110, 108, 99, 114, 0, 105, 103, 110,
  99, 114, 0, 105, 99, 114, 110, 108, 0, 105, 120, 111, 110, 0, 105, 120, 111, 102, 102, 0, 116,
  97, 110, 100, 101, 109, 0, 105, 117, 99, 108, 99, 0, 105, 120, 97, 110, 121, 0, 105, 109, 97,
  120, 98, 101, 108, 0, 105, 117, 116, 102, 56, 0, 111, 112, 111, 115, 116, 0, 111, 108, 99, 117,
  99, 0, 111, 99, 114, 110, 108, 0, 111, 110, 108, 99, 114, 0, 111, 110, 111, 99, 114, 0, 111, 110,
  108, 114, 101, 116, 0, 111, 102, 105, 108, 108, 0, 111, 102, 100, 101, 108, 0, 110, 108, 49, 0,
  110, 108, 48, 0, 99, 114, 51, 0, 99, 114, 50, 0, 99, 114, 49, 0, 99, 114, 48, 0, 116, 97, 98, 51,
  0, 116, 97, 98, 50, 0, 116, 97, 98, 49, 0, 116, 97, 98, 48, 0, 98, 115, 49, 0, 98, 115, 48, 0,
  118, 116, 49, 0, 118, 116, 48, 0, 102, 102, 49, 0, 102, 102, 48, 0, 105, 115, 105, 103, 0, 105,
  99, 97, 110, 111, 110, 0, 105, 101, 120, 116, 101, 110, 0, 101, 99, 104, 111, 0, 101, 99, 104,
  111, 101, 0, 99, 114, 116, 101, 114, 97, 115, 101, 0, 101, 99, 104, 111, 107, 0, 101, 99, 104,
  111, 110, 108, 0, 110, 111, 102, 108, 115, 104, 0, 120, 99, 97, 115, 101, 0, 116, 111, 115, 116,
  111, 112, 0, 101, 99, 104, 111, 112, 114, 116, 0, 112, 114, 116, 101, 114, 97, 115, 101, 0, 101,
  99, 104, 111, 99, 116, 108, 0, 99, 116, 108, 101, 99, 104, 111, 0, 101, 99, 104, 111, 107, 101,
  0, 99, 114, 116, 107, 105, 108, 108, 0, 102, 108, 117, 115, 104, 111, 0, 101, 120, 116, 112, 114,
  111, 99, 0, 0,
];
static mut mode_info: [mode_info; 89] = [
  {
    let mut init = mode_info {
      type_0: combination as libc::c_int as uint8_t,
      flags: (4i32 | 8i32) as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: combination as libc::c_int as uint8_t,
      flags: (4i32 | 8i32) as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: combination as libc::c_int as uint8_t,
      flags: (4i32 | 8i32) as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: combination as libc::c_int as uint8_t,
      flags: (4i32 | 8i32) as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: combination as libc::c_int as uint8_t,
      flags: 8i32 as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: combination as libc::c_int as uint8_t,
      flags: 8i32 as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: combination as libc::c_int as uint8_t,
      flags: (4i32 | 8i32) as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: combination as libc::c_int as uint8_t,
      flags: (4i32 | 8i32) as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: combination as libc::c_int as uint8_t,
      flags: (4i32 | 8i32) as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: combination as libc::c_int as uint8_t,
      flags: (4i32 | 8i32) as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: combination as libc::c_int as uint8_t,
      flags: (4i32 | 8i32) as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: combination as libc::c_int as uint8_t,
      flags: 8i32 as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: combination as libc::c_int as uint8_t,
      flags: 8i32 as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: combination as libc::c_int as uint8_t,
      flags: (4i32 | 8i32) as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: combination as libc::c_int as uint8_t,
      flags: (4i32 | 8i32) as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: combination as libc::c_int as uint8_t,
      flags: (4i32 | 8i32) as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: combination as libc::c_int as uint8_t,
      flags: (4i32 | 8i32) as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: control as libc::c_int as uint8_t,
      flags: 4i32 as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0o400i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: control as libc::c_int as uint8_t,
      flags: 4i32 as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0o1000i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: control as libc::c_int as uint8_t,
      flags: 4i32 as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0o10000000000i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: control as libc::c_int as uint8_t,
      flags: 0i32 as uint8_t,
      mask: 0o60i32 as uint16_t,
      bits: 0i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: control as libc::c_int as uint8_t,
      flags: 0i32 as uint8_t,
      mask: 0o60i32 as uint16_t,
      bits: 0o20i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: control as libc::c_int as uint8_t,
      flags: 0i32 as uint8_t,
      mask: 0o60i32 as uint16_t,
      bits: 0o40i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: control as libc::c_int as uint8_t,
      flags: 0i32 as uint8_t,
      mask: 0o60i32 as uint16_t,
      bits: 0o60i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: control as libc::c_int as uint8_t,
      flags: 4i32 as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0o2000i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: control as libc::c_int as uint8_t,
      flags: (4i32 | 8i32) as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0o2000i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: control as libc::c_int as uint8_t,
      flags: 4i32 as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0o100i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: control as libc::c_int as uint8_t,
      flags: (1i32 | 4i32) as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0o200i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: control as libc::c_int as uint8_t,
      flags: 4i32 as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0o4000i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: control as libc::c_int as uint8_t,
      flags: 4i32 as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0o20000000000u32,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: input as libc::c_int as uint8_t,
      flags: (2i32 | 4i32) as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0o1i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: input as libc::c_int as uint8_t,
      flags: (1i32 | 4i32) as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0o2i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: input as libc::c_int as uint8_t,
      flags: 4i32 as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0o4i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: input as libc::c_int as uint8_t,
      flags: 4i32 as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0o10i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: input as libc::c_int as uint8_t,
      flags: 4i32 as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0o20i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: input as libc::c_int as uint8_t,
      flags: 4i32 as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0o40i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: input as libc::c_int as uint8_t,
      flags: (2i32 | 4i32) as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0o100i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: input as libc::c_int as uint8_t,
      flags: (2i32 | 4i32) as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0o200i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: input as libc::c_int as uint8_t,
      flags: (1i32 | 4i32) as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0o400i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: input as libc::c_int as uint8_t,
      flags: 4i32 as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0o2000i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: input as libc::c_int as uint8_t,
      flags: (2i32 | 4i32) as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0o10000i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: input as libc::c_int as uint8_t,
      flags: (8i32 | 4i32) as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0o10000i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: input as libc::c_int as uint8_t,
      flags: (2i32 | 4i32) as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0o1000i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: input as libc::c_int as uint8_t,
      flags: (2i32 | 4i32) as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0o4000i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: input as libc::c_int as uint8_t,
      flags: (1i32 | 4i32) as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0o20000i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: input as libc::c_int as uint8_t,
      flags: (2i32 | 4i32) as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0o40000i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: output as libc::c_int as uint8_t,
      flags: (1i32 | 4i32) as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0o1i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: output as libc::c_int as uint8_t,
      flags: (2i32 | 4i32) as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0o2i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: output as libc::c_int as uint8_t,
      flags: (2i32 | 4i32) as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0o10i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: output as libc::c_int as uint8_t,
      flags: (1i32 | 4i32) as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0o4i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: output as libc::c_int as uint8_t,
      flags: (2i32 | 4i32) as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0o20i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: output as libc::c_int as uint8_t,
      flags: (2i32 | 4i32) as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0o40i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: output as libc::c_int as uint8_t,
      flags: (2i32 | 4i32) as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0o100i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: output as libc::c_int as uint8_t,
      flags: (2i32 | 4i32) as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0o200i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: output as libc::c_int as uint8_t,
      flags: 2i32 as uint8_t,
      mask: 0o400i32 as uint16_t,
      bits: 0o400i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: output as libc::c_int as uint8_t,
      flags: 1i32 as uint8_t,
      mask: 0o400i32 as uint16_t,
      bits: 0i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: output as libc::c_int as uint8_t,
      flags: 2i32 as uint8_t,
      mask: 0o3000i32 as uint16_t,
      bits: 0o3000i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: output as libc::c_int as uint8_t,
      flags: 2i32 as uint8_t,
      mask: 0o3000i32 as uint16_t,
      bits: 0o2000i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: output as libc::c_int as uint8_t,
      flags: 2i32 as uint8_t,
      mask: 0o3000i32 as uint16_t,
      bits: 0o1000i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: output as libc::c_int as uint8_t,
      flags: 1i32 as uint8_t,
      mask: 0o3000i32 as uint16_t,
      bits: 0i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: output as libc::c_int as uint8_t,
      flags: 2i32 as uint8_t,
      mask: 0o14000i32 as uint16_t,
      bits: 0o14000i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: output as libc::c_int as uint8_t,
      flags: 2i32 as uint8_t,
      mask: 0o14000i32 as uint16_t,
      bits: 0o10000i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: output as libc::c_int as uint8_t,
      flags: 2i32 as uint8_t,
      mask: 0o14000i32 as uint16_t,
      bits: 0o4000i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: output as libc::c_int as uint8_t,
      flags: 1i32 as uint8_t,
      mask: 0o14000i32 as uint16_t,
      bits: 0i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: output as libc::c_int as uint8_t,
      flags: 2i32 as uint8_t,
      mask: 0o20000i32 as uint16_t,
      bits: 0o20000i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: output as libc::c_int as uint8_t,
      flags: 1i32 as uint8_t,
      mask: 0o20000i32 as uint16_t,
      bits: 0i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: output as libc::c_int as uint8_t,
      flags: 2i32 as uint8_t,
      mask: 0o40000i32 as uint16_t,
      bits: 0o40000i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: output as libc::c_int as uint8_t,
      flags: 1i32 as uint8_t,
      mask: 0o40000i32 as uint16_t,
      bits: 0i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: output as libc::c_int as uint8_t,
      flags: 2i32 as uint8_t,
      mask: 0o100000i32 as uint16_t,
      bits: 0o100000i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: output as libc::c_int as uint8_t,
      flags: 1i32 as uint8_t,
      mask: 0o100000i32 as uint16_t,
      bits: 0i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: local as libc::c_int as uint8_t,
      flags: (1i32 | 4i32) as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0o1i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: local as libc::c_int as uint8_t,
      flags: (1i32 | 4i32) as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0o2i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: local as libc::c_int as uint8_t,
      flags: (1i32 | 4i32) as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0o100000i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: local as libc::c_int as uint8_t,
      flags: (1i32 | 4i32) as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0o10i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: local as libc::c_int as uint8_t,
      flags: (1i32 | 4i32) as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0o20i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: local as libc::c_int as uint8_t,
      flags: (8i32 | 4i32) as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0o20i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: local as libc::c_int as uint8_t,
      flags: (1i32 | 4i32) as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0o40i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: local as libc::c_int as uint8_t,
      flags: (2i32 | 4i32) as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0o100i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: local as libc::c_int as uint8_t,
      flags: (2i32 | 4i32) as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0o200i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: local as libc::c_int as uint8_t,
      flags: (2i32 | 4i32) as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0o4i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: local as libc::c_int as uint8_t,
      flags: (2i32 | 4i32) as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0o400i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: local as libc::c_int as uint8_t,
      flags: (2i32 | 4i32) as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0o2000i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: local as libc::c_int as uint8_t,
      flags: (8i32 | 4i32) as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0o2000i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: local as libc::c_int as uint8_t,
      flags: (1i32 | 4i32) as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0o1000i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: local as libc::c_int as uint8_t,
      flags: (8i32 | 4i32) as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0o1000i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: local as libc::c_int as uint8_t,
      flags: (1i32 | 4i32) as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0o4000i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: local as libc::c_int as uint8_t,
      flags: (8i32 | 4i32) as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0o4000i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: local as libc::c_int as uint8_t,
      flags: (2i32 | 4i32) as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0o10000i32 as tcflag_t,
    };
    init
  },
  {
    let mut init = mode_info {
      type_0: local as libc::c_int as uint8_t,
      flags: (2i32 | 4i32) as uint8_t,
      mask: 0i32 as uint16_t,
      bits: 0o200000i32 as tcflag_t,
    };
    init
  },
];
static mut control_name: [libc::c_char; 91] = [
  105, 110, 116, 114, 0, 113, 117, 105, 116, 0, 101, 114, 97, 115, 101, 0, 107, 105, 108, 108, 0,
  101, 111, 102, 0, 101, 111, 108, 0, 101, 111, 108, 50, 0, 115, 119, 116, 99, 104, 0, 115, 116,
  97, 114, 116, 0, 115, 116, 111, 112, 0, 115, 117, 115, 112, 0, 114, 112, 114, 110, 116, 0, 119,
  101, 114, 97, 115, 101, 0, 108, 110, 101, 120, 116, 0, 102, 108, 117, 115, 104, 0, 109, 105, 110,
  0, 116, 105, 109, 101, 0, 0,
];
static mut control_info: [control_info; 17] = [
  {
    let mut init = control_info {
      saneval: ('c' as i32 & 0o37i32) as uint8_t,
      offset: 0i32 as uint8_t,
    };
    init
  },
  {
    let mut init = control_info {
      saneval: 0o34i32 as uint8_t,
      offset: 1i32 as uint8_t,
    };
    init
  },
  {
    let mut init = control_info {
      saneval: 0o177i32 as uint8_t,
      offset: 2i32 as uint8_t,
    };
    init
  },
  {
    let mut init = control_info {
      saneval: ('u' as i32 & 0o37i32) as uint8_t,
      offset: 3i32 as uint8_t,
    };
    init
  },
  {
    let mut init = control_info {
      saneval: ('d' as i32 & 0o37i32) as uint8_t,
      offset: 4i32 as uint8_t,
    };
    init
  },
  {
    let mut init = control_info {
      saneval: '\u{0}' as i32 as uint8_t,
      offset: 11i32 as uint8_t,
    };
    init
  },
  {
    let mut init = control_info {
      saneval: '\u{0}' as i32 as uint8_t,
      offset: 16i32 as uint8_t,
    };
    init
  },
  {
    let mut init = control_info {
      saneval: '\u{0}' as i32 as uint8_t,
      offset: 7i32 as uint8_t,
    };
    init
  },
  {
    let mut init = control_info {
      saneval: ('q' as i32 & 0o37i32) as uint8_t,
      offset: 8i32 as uint8_t,
    };
    init
  },
  {
    let mut init = control_info {
      saneval: ('s' as i32 & 0o37i32) as uint8_t,
      offset: 9i32 as uint8_t,
    };
    init
  },
  {
    let mut init = control_info {
      saneval: ('z' as i32 & 0o37i32) as uint8_t,
      offset: 10i32 as uint8_t,
    };
    init
  },
  {
    let mut init = control_info {
      saneval: ('r' as i32 & 0o37i32) as uint8_t,
      offset: 12i32 as uint8_t,
    };
    init
  },
  {
    let mut init = control_info {
      saneval: ('w' as i32 & 0o37i32) as uint8_t,
      offset: 14i32 as uint8_t,
    };
    init
  },
  {
    let mut init = control_info {
      saneval: ('v' as i32 & 0o37i32) as uint8_t,
      offset: 15i32 as uint8_t,
    };
    init
  },
  {
    let mut init = control_info {
      saneval: ('o' as i32 & 0x1fi32) as uint8_t,
      offset: 13i32 as uint8_t,
    };
    init
  },
  {
    let mut init = control_info {
      saneval: 1i32 as uint8_t,
      offset: 6i32 as uint8_t,
    };
    init
  },
  {
    let mut init = control_info {
      saneval: 0i32 as uint8_t,
      offset: 5i32 as uint8_t,
    };
    init
  },
];
/* we are noexec, must clear */
unsafe extern "C" fn set_speed_or_die(
  mut type_0: speed_setting,
  mut arg: *const libc::c_char,
  mut mode: *mut termios,
) {
  let mut baud: speed_t = 0;
  baud = tty_value_to_baud(xatou(arg));
  if type_0 as libc::c_uint != output_speed as libc::c_int as libc::c_uint {
    /* either input or both */
    cfsetispeed(mode, baud);
  }
  if type_0 as libc::c_uint != input_speed as libc::c_int as libc::c_uint {
    /* either output or both */
    cfsetospeed(mode, baud);
  };
}
unsafe extern "C" fn perror_on_device_and_die(mut fmt: *const libc::c_char) -> ! {
  bb_perror_msg_and_die(
    fmt,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).device_name,
  );
}
unsafe extern "C" fn perror_on_device(mut fmt: *const libc::c_char) {
  bb_perror_msg(
    fmt,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).device_name,
  );
}
/* Print format string MESSAGE and optional args.
Wrap to next line first if it won't fit.
Print a space first unless MESSAGE will start a new line */
unsafe extern "C" fn wrapf(mut message: *const libc::c_char, mut args: ...) {
  let mut buf: [libc::c_char; 128] = [0; 128];
  let mut args_0: ::std::ffi::VaListImpl;
  let mut buflen: libc::c_uint = 0;
  args_0 = args.clone();
  buflen = vsnprintf(
    buf.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
    message,
    args_0.as_va_list(),
  ) as libc::c_uint;
  /* We seem to be called only with suitable lengths, but check if
  somebody failed to adhere to this assumption just to be sure.  */
  if buflen == 0
    || buflen as libc::c_ulong >= ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong
  {
    return;
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).current_col > 0i32 as libc::c_uint {
    let ref mut fresh0 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).current_col;
    *fresh0 = (*fresh0).wrapping_add(1);
    if buf[0] as libc::c_int != '\n' as i32 {
      if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .current_col
        .wrapping_add(buflen)
        >= (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).max_col
      {
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).current_col = 0i32 as libc::c_uint;
        bb_putchar('\n' as i32);
      } else {
        bb_putchar(' ' as i32);
      }
    }
  }
  fputs_unlocked(buf.as_mut_ptr(), stdout);
  let ref mut fresh1 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).current_col;
  *fresh1 = (*fresh1).wrapping_add(buflen);
  if buf[buflen.wrapping_sub(1i32 as libc::c_uint) as usize] as libc::c_int == '\n' as i32 {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).current_col = 0i32 as libc::c_uint
  };
}
unsafe extern "C" fn newline() {
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).current_col != 0i32 as libc::c_uint {
    wrapf(b"\n\x00" as *const u8 as *const libc::c_char);
  };
}
unsafe extern "C" fn set_window_size(mut rows: libc::c_int, mut cols: libc::c_int) {
  let mut current_block: u64;
  let mut win: winsize = {
    let mut init = winsize {
      ws_row: 0i32 as libc::c_ushort,
      ws_col: 0i32 as libc::c_ushort,
      ws_xpixel: 0i32 as libc::c_ushort,
      ws_ypixel: 0i32 as libc::c_ushort,
    };
    init
  };
  if ioctl(0i32, 0x5413i32 as libc::c_ulong, &mut win as *mut winsize) != 0 {
    if *bb_errno != 22i32 {
      current_block = 17025510460101745201;
    } else {
      memset(
        &mut win as *mut winsize as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<winsize>() as libc::c_ulong,
      );
      current_block = 735147466149431745;
    }
  } else {
    current_block = 735147466149431745;
  }
  match current_block {
    735147466149431745 => {
      if rows >= 0i32 {
        win.ws_row = rows as libc::c_ushort
      }
      if cols >= 0i32 {
        win.ws_col = cols as libc::c_ushort
      }
      if ioctl(
        0i32,
        0x5414i32 as libc::c_ulong,
        &mut win as *mut winsize as *mut libc::c_char,
      ) != 0
      {
        current_block = 17025510460101745201;
      } else {
        current_block = 1917311967535052937;
      }
    }
    _ => {}
  }
  match current_block {
    17025510460101745201 => {
      perror_on_device(b"%s\x00" as *const u8 as *const libc::c_char);
    }
    _ => {}
  };
}
unsafe extern "C" fn display_window_size(mut fancy: libc::c_int) {
  let mut fmt_str: *const libc::c_char =
    b"%s\x00%s: no size information for this device\x00" as *const u8 as *const libc::c_char;
  let mut width: libc::c_uint = 0;
  let mut height: libc::c_uint = 0;
  if get_terminal_width_height(0i32, &mut width, &mut height) != 0 {
    if *bb_errno != 22i32 || {
      fmt_str = fmt_str.offset(2);
      (fancy) == 0
    } {
      perror_on_device(fmt_str);
    }
  } else {
    wrapf(
      if fancy != 0 {
        b"rows %u; columns %u;\x00" as *const u8 as *const libc::c_char
      } else {
        b"%u %u\n\x00" as *const u8 as *const libc::c_char
      },
      height,
      width,
    );
  };
}
static mut stty_suffixes: [suffix_mult; 4] = [
  {
    let mut init = suffix_mult {
      suffix: [98, 0, 0, 0],
      mult: 512i32 as libc::c_uint,
    };
    init
  },
  {
    let mut init = suffix_mult {
      suffix: [107, 0, 0, 0],
      mult: 1024i32 as libc::c_uint,
    };
    init
  },
  {
    let mut init = suffix_mult {
      suffix: [66, 0, 0, 0],
      mult: 1024i32 as libc::c_uint,
    };
    init
  },
  {
    let mut init = suffix_mult {
      suffix: [0, 0, 0, 0],
      mult: 0i32 as libc::c_uint,
    };
    init
  },
];
unsafe extern "C" fn find_mode(mut name: *const libc::c_char) -> *const mode_info {
  let mut i: libc::c_int = index_in_strings(mode_name.as_ptr(), name);
  return if i >= 0i32 {
    &*mode_info.as_ptr().offset(i as isize) as *const mode_info
  } else {
    0 as *const mode_info
  };
}
unsafe extern "C" fn find_control(mut name: *const libc::c_char) -> *const control_info {
  let mut i: libc::c_int = index_in_strings(control_name.as_ptr(), name);
  return if i >= 0i32 {
    &*control_info.as_ptr().offset(i as isize) as *const control_info
  } else {
    0 as *const control_info
  };
}
unsafe extern "C" fn find_param(mut name: *const libc::c_char) -> libc::c_int {
  static mut params: [libc::c_char; 49] = [
    108, 105, 110, 101, 0, 114, 111, 119, 115, 0, 99, 111, 108, 115, 0, 99, 111, 108, 117, 109,
    110, 115, 0, 115, 105, 122, 101, 0, 115, 112, 101, 101, 100, 0, 105, 115, 112, 101, 101, 100,
    0, 111, 115, 112, 101, 101, 100, 0, 0,
  ];
  let mut i: libc::c_int = index_in_strings(params.as_ptr(), name) + 1i32;
  if i == 0i32 {
    return 0i32;
  }
  if i != 5i32 && i != 6i32 {
    i |= 0x80i32
  }
  return i;
}
unsafe extern "C" fn recover_mode(
  mut arg: *const libc::c_char,
  mut mode: *mut termios,
) -> libc::c_int {
  let mut i: libc::c_int = 0;
  let mut n: libc::c_int = 0;
  let mut chr: libc::c_uint = 0;
  let mut iflag: libc::c_ulong = 0;
  let mut oflag: libc::c_ulong = 0;
  let mut cflag: libc::c_ulong = 0;
  let mut lflag: libc::c_ulong = 0;
  /* Scan into temporaries since it is too much trouble to figure out
  the right format for 'tcflag_t' */
  if sscanf(
    arg,
    b"%lx:%lx:%lx:%lx%n\x00" as *const u8 as *const libc::c_char,
    &mut iflag as *mut libc::c_ulong,
    &mut oflag as *mut libc::c_ulong,
    &mut cflag as *mut libc::c_ulong,
    &mut lflag as *mut libc::c_ulong,
    &mut n as *mut libc::c_int,
  ) != 4i32
  {
    return 0i32;
  }
  (*mode).c_iflag = iflag as tcflag_t;
  (*mode).c_oflag = oflag as tcflag_t;
  (*mode).c_cflag = cflag as tcflag_t;
  (*mode).c_lflag = lflag as tcflag_t;
  arg = arg.offset(n as isize);
  i = 0i32;
  while i < 32i32 {
    if sscanf(
      arg,
      b":%x%n\x00" as *const u8 as *const libc::c_char,
      &mut chr as *mut libc::c_uint,
      &mut n as *mut libc::c_int,
    ) != 1i32
    {
      return 0i32;
    }
    (*mode).c_cc[i as usize] = chr as cc_t;
    arg = arg.offset(n as isize);
    i += 1
  }
  /* Fail if there are too many fields */
  if *arg as libc::c_int != '\u{0}' as i32 {
    return 0i32;
  }
  return 1i32;
}
unsafe extern "C" fn display_recoverable(mut mode: *const termios, mut _dummy: libc::c_int) {
  let mut i: libc::c_int = 0;
  printf(
    b"%lx:%lx:%lx:%lx\x00" as *const u8 as *const libc::c_char,
    (*mode).c_iflag as libc::c_ulong,
    (*mode).c_oflag as libc::c_ulong,
    (*mode).c_cflag as libc::c_ulong,
    (*mode).c_lflag as libc::c_ulong,
  );
  i = 0i32;
  while i < 32i32 {
    printf(
      b":%x\x00" as *const u8 as *const libc::c_char,
      (*mode).c_cc[i as usize] as libc::c_uint,
    );
    i += 1
  }
  bb_putchar('\n' as i32);
}
unsafe extern "C" fn display_speed(mut mode: *const termios, mut fancy: libc::c_int) {
  //____________________ 01234567 8 9
  let mut fmt_str: *const libc::c_char =
    b"%lu %lu\n\x00ispeed %lu baud; ospeed %lu baud;\x00" as *const u8 as *const libc::c_char; /* in case ispeed was 0 */
  let mut ispeed: libc::c_ulong = 0;
  let mut ospeed: libc::c_ulong = 0;
  ispeed = cfgetispeed(mode) as libc::c_ulong;
  ospeed = cfgetospeed(mode) as libc::c_ulong;
  if ispeed == 0i32 as libc::c_ulong || ispeed == ospeed {
    ispeed = ospeed;
    //________ 0123 4 5 6 7 8 9
    fmt_str = b"%lu\n\x00\x00\x00\x00\x00speed %lu baud;\x00" as *const u8 as *const libc::c_char
  }
  if fancy != 0 {
    fmt_str = fmt_str.offset(9)
  }
  wrapf(
    fmt_str,
    tty_baud_to_value(ispeed as speed_t),
    tty_baud_to_value(ospeed as speed_t),
  );
}
unsafe extern "C" fn do_display(mut mode: *const termios, mut all: libc::c_int) {
  let mut i: libc::c_int = 0;
  let mut bitsp: *mut tcflag_t = 0 as *mut tcflag_t;
  let mut mask: libc::c_ulong = 0;
  let mut prev_type: libc::c_int = control as libc::c_int;
  display_speed(mode, 1i32);
  if all != 0 {
    display_window_size(1i32);
  }
  wrapf(
    b"line = %u;\n\x00" as *const u8 as *const libc::c_char,
    (*mode).c_line as libc::c_int,
  );
  i = 0i32;
  while i != CIDX_min as libc::c_int {
    let mut ch: libc::c_char = 0;
    let mut buf10: [libc::c_char; 10] = [0; 10];
    /* If swtch is the same as susp, don't print both */
    /* If eof uses the same slot as min, only print whichever applies */
    ch = (*mode).c_cc[control_info[i as usize].offset as usize] as libc::c_char;
    if ch as libc::c_int == '\u{0}' as i32 {
      strcpy(
        buf10.as_mut_ptr(),
        b"<undef>\x00" as *const u8 as *const libc::c_char,
      );
    } else {
      visible(ch as libc::c_uint, buf10.as_mut_ptr(), 0i32);
    }
    wrapf(
      b"%s = %s;\x00" as *const u8 as *const libc::c_char,
      nth_string(control_name.as_ptr(), i),
      buf10.as_mut_ptr(),
    );
    i += 1
  }
  wrapf(
    b"min = %u; time = %u;\x00" as *const u8 as *const libc::c_char,
    (*mode).c_cc[6] as libc::c_int,
    (*mode).c_cc[5] as libc::c_int,
  );
  newline();
  i = 0i32;
  while i < NUM_mode_info as libc::c_int {
    if !(mode_info[i as usize].flags as libc::c_int & 8i32 != 0) {
      if mode_info[i as usize].type_0 as libc::c_int != prev_type {
        newline();
        prev_type = mode_info[i as usize].type_0 as libc::c_int
      }
      bitsp = get_ptr_to_tcflag(mode_info[i as usize].type_0 as libc::c_uint, mode);
      mask = if mode_info[i as usize].mask as libc::c_int != 0 {
        mode_info[i as usize].mask as libc::c_uint
      } else {
        mode_info[i as usize].bits
      } as libc::c_ulong;
      if *bitsp as libc::c_ulong & mask == mode_info[i as usize].bits as libc::c_ulong {
        if all != 0 || mode_info[i as usize].flags as libc::c_int & 2i32 != 0 {
          wrapf(
            (b"-%s\x00" as *const u8 as *const libc::c_char).offset(1),
            nth_string(mode_name.as_ptr(), i),
          );
        }
      } else if all != 0 && mode_info[i as usize].flags as libc::c_int & 4i32 != 0
        || all == 0 && mode_info[i as usize].flags as libc::c_int & (1i32 | 4i32) == 1i32 | 4i32
      {
        wrapf(
          b"-%s\x00" as *const u8 as *const libc::c_char,
          nth_string(mode_name.as_ptr(), i),
        );
      }
    }
    i += 1
  }
  newline();
}
unsafe extern "C" fn sane_mode(mut mode: *mut termios) {
  let mut i: libc::c_int = 0;
  i = 0i32;
  while i < NUM_control_info as libc::c_int {
    (*mode).c_cc[control_info[i as usize].offset as usize] = control_info[i as usize].saneval;
    i += 1
  }
  i = 0i32;
  while i < NUM_mode_info as libc::c_int {
    let mut val: tcflag_t = 0;
    let mut bitsp: *mut tcflag_t =
      get_ptr_to_tcflag(mode_info[i as usize].type_0 as libc::c_uint, mode);
    if !bitsp.is_null() {
      val = (*bitsp as libc::c_ulong & !(mode_info[i as usize].mask as libc::c_ulong)) as tcflag_t;
      if mode_info[i as usize].flags as libc::c_int & 1i32 != 0 {
        *bitsp = val | mode_info[i as usize].bits
      } else if mode_info[i as usize].flags as libc::c_int & 2i32 != 0 {
        *bitsp = val & !mode_info[i as usize].bits
      }
    }
    i += 1
  }
}
unsafe extern "C" fn set_mode(
  mut info: *const mode_info,
  mut reversed: libc::c_int,
  mut mode: *mut termios,
) {
  let mut bitsp: *mut tcflag_t = 0 as *mut tcflag_t;
  bitsp = get_ptr_to_tcflag((*info).type_0 as libc::c_uint, mode);
  if !bitsp.is_null() {
    let mut val: tcflag_t = *bitsp & !((*info).mask as libc::c_int) as libc::c_uint;
    if reversed != 0 {
      *bitsp = val & !(*info).bits
    } else {
      *bitsp = val | (*info).bits
    }
    return;
  }
  /* !bitsp - it's a "combination" mode */
  if info == &*mode_info.as_ptr().offset(IDX_evenp as libc::c_int as isize) as *const mode_info
    || info
      == &*mode_info
        .as_ptr()
        .offset(IDX_parity as libc::c_int as isize) as *const mode_info
  {
    if reversed != 0 {
      (*mode).c_cflag = (*mode).c_cflag & !0o400i32 as libc::c_uint & !0o60i32 as libc::c_uint
        | 0o60i32 as libc::c_uint
    } else {
      (*mode).c_cflag = (*mode).c_cflag & !0o1000i32 as libc::c_uint & !0o60i32 as libc::c_uint
        | 0o400i32 as libc::c_uint
        | 0o40i32 as libc::c_uint
    }
  } else if info
    == &*mode_info.as_ptr().offset(IDX_oddp as libc::c_int as isize) as *const mode_info
  {
    if reversed != 0 {
      (*mode).c_cflag = (*mode).c_cflag & !0o400i32 as libc::c_uint & !0o60i32 as libc::c_uint
        | 0o60i32 as libc::c_uint
    } else {
      (*mode).c_cflag = (*mode).c_cflag & !0o60i32 as libc::c_uint
        | 0o40i32 as libc::c_uint
        | 0o1000i32 as libc::c_uint
        | 0o400i32 as libc::c_uint
    }
  } else if info == &*mode_info.as_ptr().offset(IDX_nl as libc::c_int as isize) as *const mode_info
  {
    if reversed != 0 {
      (*mode).c_iflag = ((*mode).c_iflag | 0o400i32 as libc::c_uint)
        & !0o100i32 as libc::c_uint
        & !0o200i32 as libc::c_uint;
      (*mode).c_oflag = ((*mode).c_oflag | 0o4i32 as libc::c_uint)
        & !0o10i32 as libc::c_uint
        & !0o40i32 as libc::c_uint
    } else {
      (*mode).c_iflag = (*mode).c_iflag & !0o400i32 as libc::c_uint;
      (*mode).c_oflag = (*mode).c_oflag & !0o4i32 as libc::c_uint
    }
  } else if info == &*mode_info.as_ptr().offset(IDX_ek as libc::c_int as isize) as *const mode_info
  {
    (*mode).c_cc[2] = 0o177i32 as cc_t;
    (*mode).c_cc[3] = ('u' as i32 & 0o37i32) as cc_t
  } else if info
    == &*mode_info.as_ptr().offset(IDX_sane as libc::c_int as isize) as *const mode_info
  {
    sane_mode(mode);
  } else if info
    == &*mode_info
      .as_ptr()
      .offset(IDX_cbreak as libc::c_int as isize) as *const mode_info
  {
    if reversed != 0 {
      (*mode).c_lflag |= 0o2i32 as libc::c_uint
    } else {
      (*mode).c_lflag &= !0o2i32 as libc::c_uint
    }
  } else if info
    == &*mode_info.as_ptr().offset(IDX_pass8 as libc::c_int as isize) as *const mode_info
  {
    if reversed != 0 {
      (*mode).c_cflag = (*mode).c_cflag & !0o60i32 as libc::c_uint
        | 0o40i32 as libc::c_uint
        | 0o400i32 as libc::c_uint;
      (*mode).c_iflag |= 0o40i32 as libc::c_uint
    } else {
      (*mode).c_cflag = (*mode).c_cflag & !0o400i32 as libc::c_uint & !0o60i32 as libc::c_uint
        | 0o60i32 as libc::c_uint;
      (*mode).c_iflag &= !0o40i32 as libc::c_uint
    }
  } else if info
    == &*mode_info
      .as_ptr()
      .offset(IDX_litout as libc::c_int as isize) as *const mode_info
  {
    if reversed != 0 {
      (*mode).c_cflag = (*mode).c_cflag & !0o60i32 as libc::c_uint
        | 0o40i32 as libc::c_uint
        | 0o400i32 as libc::c_uint;
      (*mode).c_iflag |= 0o40i32 as libc::c_uint;
      (*mode).c_oflag |= 0o1i32 as libc::c_uint
    } else {
      (*mode).c_cflag = (*mode).c_cflag & !0o400i32 as libc::c_uint & !0o60i32 as libc::c_uint
        | 0o60i32 as libc::c_uint;
      (*mode).c_iflag &= !0o40i32 as libc::c_uint;
      (*mode).c_oflag &= !0o1i32 as libc::c_uint
    }
  } else if info == &*mode_info.as_ptr().offset(IDX_raw as libc::c_int as isize) as *const mode_info
    || info
      == &*mode_info
        .as_ptr()
        .offset(IDX_cooked as libc::c_int as isize) as *const mode_info
  {
    if info == &*mode_info.as_ptr().offset(IDX_raw as libc::c_int as isize) as *const mode_info
      && reversed != 0
      || info
        == &*mode_info
          .as_ptr()
          .offset(IDX_cooked as libc::c_int as isize) as *const mode_info
        && reversed == 0
    {
      /* Cooked mode */
      (*mode).c_iflag |= (0o2i32 | 0o4i32 | 0o40i32 | 0o400i32 | 0o2000i32) as libc::c_uint;
      (*mode).c_oflag |= 0o1i32 as libc::c_uint;
      (*mode).c_lflag |= (0o1i32 | 0o2i32) as libc::c_uint
    } else {
      /* Raw mode */
      (*mode).c_iflag = 0i32 as tcflag_t; /* ^C */
      (*mode).c_oflag &= !0o1i32 as libc::c_uint; /* DEL */
      (*mode).c_lflag &= !(0o1i32 | 0o2i32 | 0o4i32) as libc::c_uint; /* ^U */
      (*mode).c_cc[6] = 1i32 as cc_t;
      (*mode).c_cc[5] = 0i32 as cc_t
    }
  } else if info
    == &*mode_info
      .as_ptr()
      .offset(IDX_decctlq as libc::c_int as isize) as *const mode_info
  {
    if reversed != 0 {
      (*mode).c_iflag |= 0o4000i32 as libc::c_uint
    } else {
      (*mode).c_iflag &= !0o4000i32 as libc::c_uint
    }
  } else if info
    == &*mode_info.as_ptr().offset(IDX_tabs as libc::c_int as isize) as *const mode_info
  {
    if reversed != 0 {
      (*mode).c_oflag = (*mode).c_oflag & !0o14000i32 as libc::c_uint | 0o14000i32 as libc::c_uint
    } else {
      (*mode).c_oflag = (*mode).c_oflag & !0o14000i32 as libc::c_uint | 0i32 as libc::c_uint
    }
  } else if info
    == &*mode_info.as_ptr().offset(IDX_lcase as libc::c_int as isize) as *const mode_info
    || info == &*mode_info.as_ptr().offset(IDX_LCASE as libc::c_int as isize) as *const mode_info
  {
    if reversed != 0 {
      (*mode).c_lflag &= !0o4i32 as libc::c_uint;
      (*mode).c_iflag &= !0o1000i32 as libc::c_uint;
      (*mode).c_oflag &= !0o2i32 as libc::c_uint
    } else {
      (*mode).c_lflag |= 0o4i32 as libc::c_uint;
      (*mode).c_iflag |= 0o1000i32 as libc::c_uint;
      (*mode).c_oflag |= 0o2i32 as libc::c_uint
    }
  } else if info == &*mode_info.as_ptr().offset(IDX_crt as libc::c_int as isize) as *const mode_info
  {
    (*mode).c_lflag |= (0o20i32 | 0o1000i32 | 0o4000i32) as libc::c_uint
  } else if info == &*mode_info.as_ptr().offset(IDX_dec as libc::c_int as isize) as *const mode_info
  {
    (*mode).c_cc[0] = 3i32 as cc_t;
    (*mode).c_cc[2] = 127i32 as cc_t;
    (*mode).c_cc[3] = 21i32 as cc_t;
    (*mode).c_lflag |= (0o20i32 | 0o1000i32 | 0o4000i32) as libc::c_uint;
    (*mode).c_iflag &= !0o4000i32 as libc::c_uint
  };
}
unsafe extern "C" fn set_control_char_or_die(
  mut info: *const control_info,
  mut arg: *const libc::c_char,
  mut mode: *mut termios,
) {
  let mut value: libc::c_uchar = 0;
  if info
    == &*control_info
      .as_ptr()
      .offset(CIDX_min as libc::c_int as isize) as *const control_info
    || info
      == &*control_info
        .as_ptr()
        .offset(CIDX_time as libc::c_int as isize) as *const control_info
  {
    value = xatoul_range_sfx(
      arg,
      0i32 as libc::c_ulong,
      0xffi32 as libc::c_ulong,
      stty_suffixes.as_ptr(),
    ) as libc::c_uchar
  } else if *arg.offset(0) as libc::c_int == '\u{0}' as i32
    || *arg.offset(1) as libc::c_int == '\u{0}' as i32
  {
    value = *arg.offset(0) as libc::c_uchar
  } else if strcmp(arg, b"^-\x00" as *const u8 as *const libc::c_char) == 0i32
    || strcmp(arg, b"undef\x00" as *const u8 as *const libc::c_char) == 0i32
  {
    value = '\u{0}' as i32 as libc::c_uchar
  } else if *arg.offset(0) as libc::c_int == '^' as i32 {
    /* Ignore any trailing junk (^Cjunk) */
    value = (*arg.offset(1) as libc::c_int & 0x1fi32) as libc::c_uchar; /* Non-letters get weird results */
    if *arg.offset(1) as libc::c_int == '?' as i32 {
      value = 127i32 as libc::c_uchar
    }
  } else {
    value = xatoul_range_sfx(
      arg,
      0i32 as libc::c_ulong,
      0xffi32 as libc::c_ulong,
      stty_suffixes.as_ptr(),
    ) as libc::c_uchar
  }
  (*mode).c_cc[(*info).offset as usize] = value;
}
#[no_mangle]
pub unsafe extern "C" fn stty_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut current_block: u64;
  let mut mode: termios = termios {
    c_iflag: 0,
    c_oflag: 0,
    c_cflag: 0,
    c_lflag: 0,
    c_line: 0,
    c_cc: [0; 32],
    c_ispeed: 0,
    c_ospeed: 0,
  };
  let mut output_func: Option<unsafe extern "C" fn(_: *const termios, _: libc::c_int) -> ()> = None;
  let mut file_name: *const libc::c_char = 0 as *const libc::c_char;
  let mut display_all: libc::c_int = 0i32;
  let mut stty_state: libc::c_int = 0;
  let mut k: libc::c_int = 0;
  let ref mut fresh2 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).device_name;
  *fresh2 = bb_msg_standard_input.as_ptr();
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).max_col = 80i32 as libc::c_uint;
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).current_col = 0i32 as libc::c_uint;
  stty_state = 1i32 << 4i32;
  output_func = Some(do_display as unsafe extern "C" fn(_: *const termios, _: libc::c_int) -> ());
  /* First pass: only parse/verify command line params */
  k = 0i32;
  's_59: loop {
    k += 1;
    if (*argv.offset(k as isize)).is_null() {
      break;
    }
    let mut mp: *const mode_info = 0 as *const mode_info;
    let mut cp: *const control_info = 0 as *const control_info;
    let mut arg: *const libc::c_char = *argv.offset(k as isize);
    let mut argnext: *const libc::c_char = *argv.offset((k + 1i32) as isize);
    let mut param: libc::c_int = 0;
    if *arg.offset(0) as libc::c_int == '-' as i32 {
      let mut i: libc::c_int = 0;
      mp = find_mode(arg.offset(1));
      if !mp.is_null() {
        if !((*mp).flags as libc::c_int & 4i32 == 0) {
          stty_state &= !(1i32 << 4i32);
          continue;
        }
      } else {
        /* It is an option - parse it */
        i = 0i32; /* "-Fdevice" ? */
        loop {
          i += 1;
          if !(*arg.offset(i as isize) != 0) {
            continue 's_59;
          }
          match *arg.offset(i as isize) as libc::c_int {
            97 => {
              stty_state |= 1i32 << 2i32;
              output_func =
                Some(do_display as unsafe extern "C" fn(_: *const termios, _: libc::c_int) -> ());
              display_all = 1i32
            }
            103 => {
              stty_state |= 1i32 << 3i32;
              output_func = Some(
                display_recoverable
                  as unsafe extern "C" fn(_: *const termios, _: libc::c_int) -> (),
              )
            }
            70 => {
              if !file_name.is_null() {
                bb_simple_error_msg_and_die(
                  b"only one device may be specified\x00" as *const u8 as *const libc::c_char,
                );
              }
              file_name = &*arg.offset((i + 1i32) as isize) as *const libc::c_char;
              if *file_name.offset(0) == 0 {
                /* nope, "-F device" */
                let mut p: libc::c_int = k + 1i32; /* argv[p] is argnext */
                file_name = argnext;
                if file_name.is_null() {
                  bb_error_msg_and_die(
                    bb_msg_requires_arg.as_ptr(),
                    b"-F\x00" as *const u8 as *const libc::c_char,
                  );
                }
                /* remove -F param from arg[vc] */
                while !(*argv.offset(p as isize)).is_null() {
                  let ref mut fresh3 = *argv.offset(p as isize);
                  *fresh3 = *argv.offset((p + 1i32) as isize);
                  p += 1
                }
              }
              continue 's_59;
            }
            _ => {
              break;
            }
          }
        }
      }
    } else {
      mp = find_mode(arg);
      if !mp.is_null() {
        stty_state &= !(1i32 << 4i32);
        continue;
      } else {
        cp = find_control(arg);
        if !cp.is_null() {
          if argnext.is_null() {
            bb_error_msg_and_die(bb_msg_requires_arg.as_ptr(), arg);
          }
          /* called for the side effect of xfunc death only */
          set_control_char_or_die(cp, argnext, &mut mode);
          stty_state &= !(1i32 << 4i32);
          k += 1;
          continue;
        } else {
          param = find_param(arg);
          if param & param_need_arg as libc::c_int != 0 {
            if argnext.is_null() {
              bb_error_msg_and_die(bb_msg_requires_arg.as_ptr(), arg);
            }
            k += 1
          }
          match param {
            129 | 130 | 131 | 132 => {
              /* else fall-through */
              xatoul_range_sfx(
                argnext,
                1i32 as libc::c_ulong,
                2147483647i32 as libc::c_ulong,
                stty_suffixes.as_ptr(),
              );
              current_block = 7178192492338286402;
            }
            5 | 6 => {
              current_block = 7178192492338286402;
            }
            135 => {
              /* called for the side effect of xfunc death only */
              set_speed_or_die(input_speed, argnext, &mut mode);
              current_block = 7178192492338286402;
            }
            136 => {
              /* called for the side effect of xfunc death only */
              set_speed_or_die(output_speed, argnext, &mut mode);
              current_block = 7178192492338286402;
            }
            _ => {
              if recover_mode(arg, &mut mode) == 1i32 {
                current_block = 7178192492338286402;
              } else if tty_value_to_baud(xatou(arg)) != -1i32 as speed_t {
                current_block = 7178192492338286402;
              } else {
                current_block = 14462319228209588966;
              }
            }
          }
          match current_block {
            14462319228209588966 => {}
            _ => {
              stty_state &= !(1i32 << 4i32);
              continue;
            }
          }
        }
      }
    }
    bb_error_msg_and_die(
      b"invalid argument \'%s\'\x00" as *const u8 as *const libc::c_char,
      arg,
    );
  }
  /* Specifying both -a and -g is an error */
  if stty_state & (1i32 << 2i32 | 1i32 << 3i32) == 1i32 << 2i32 | 1i32 << 3i32 {
    bb_simple_error_msg_and_die(
      b"-a and -g are mutually exclusive\x00" as *const u8 as *const libc::c_char,
    );
  }
  /* Specifying -a or -g with non-options is an error */
  if stty_state & (1i32 << 2i32 | 1i32 << 3i32) != 0 && stty_state & 1i32 << 4i32 == 0 {
    bb_simple_error_msg_and_die(
      b"modes may not be set when -a or -g is used\x00" as *const u8 as *const libc::c_char,
    );
  }
  /* Now it is safe to start doing things */
  if !file_name.is_null() {
    let ref mut fresh4 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).device_name;
    *fresh4 = file_name;
    xmove_fd(
      xopen_nonblocking((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).device_name),
      0i32,
    );
    ndelay_off(0i32);
  }
  /* Initialize to all zeroes so there is no risk memcmp will report a
  spurious difference in an uninitialized portion of the structure */
  memset(
    &mut mode as *mut termios as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<termios>() as libc::c_ulong,
  );
  if tcgetattr(0i32, &mut mode) != 0 {
    perror_on_device_and_die(b"%s\x00" as *const u8 as *const libc::c_char);
  }
  if stty_state & (1i32 << 2i32 | 1i32 << 3i32 | 1i32 << 4i32) != 0 {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).max_col =
      get_terminal_width(1i32) as libc::c_uint;
    output_func.expect("non-null function pointer")(&mut mode, display_all);
    return 0i32;
  }
  /* Second pass: perform actions */
  k = 0i32;
  loop
  /* It is an option - already parsed. Skip it */
  {
    k += 1;
    if (*argv.offset(k as isize)).is_null() {
      break;
    }
    let mut mp_0: *const mode_info = 0 as *const mode_info;
    let mut cp_0: *const control_info = 0 as *const control_info;
    let mut arg_0: *const libc::c_char = *argv.offset(k as isize);
    let mut argnext_0: *const libc::c_char = *argv.offset((k + 1i32) as isize);
    let mut param_0: libc::c_int = 0;
    if *arg_0.offset(0) as libc::c_int == '-' as i32 {
      mp_0 = find_mode(arg_0.offset(1));
      if !mp_0.is_null() {
        set_mode(mp_0, 1i32, &mut mode);
        stty_state |= 1i32 << 0i32
      }
    } else {
      mp_0 = find_mode(arg_0);
      if !mp_0.is_null() {
        set_mode(mp_0, 0i32, &mut mode);
        stty_state |= 1i32 << 0i32
      } else {
        cp_0 = find_control(arg_0);
        if !cp_0.is_null() {
          k += 1;
          set_control_char_or_die(cp_0, argnext_0, &mut mode);
          stty_state |= 1i32 << 0i32
        } else {
          param_0 = find_param(arg_0);
          if param_0 & param_need_arg as libc::c_int != 0 {
            k += 1
          }
          match param_0 {
            129 => {
              mode.c_line = xatoul_sfx(argnext_0, stty_suffixes.as_ptr()) as cc_t;
              stty_state |= 1i32 << 0i32
              /* else - impossible (caught in the first pass):
              bb_error_msg_and_die("invalid argument '%s'", arg); */
            }
            131 | 132 => {
              set_window_size(
                -1i32,
                xatoul_sfx(argnext_0, stty_suffixes.as_ptr()) as libc::c_int,
              );
            }
            5 => {
              display_window_size(0i32);
            }
            130 => {
              set_window_size(
                xatoul_sfx(argnext_0, stty_suffixes.as_ptr()) as libc::c_int,
                -1i32,
              );
            }
            6 => {
              display_speed(&mut mode, 0i32);
            }
            135 => {
              set_speed_or_die(input_speed, argnext_0, &mut mode);
              stty_state |= 1i32 << 0i32 | 1i32 << 1i32
            }
            136 => {
              set_speed_or_die(output_speed, argnext_0, &mut mode);
              stty_state |= 1i32 << 0i32 | 1i32 << 1i32
            }
            _ => {
              if recover_mode(arg_0, &mut mode) == 1i32 {
                stty_state |= 1i32 << 0i32
              } else {
                /* true: if (tty_value_to_baud(xatou(arg)) != (speed_t) -1) */
                set_speed_or_die(both_speeds, arg_0, &mut mode);
                stty_state |= 1i32 << 0i32 | 1i32 << 1i32
              }
            }
          }
        }
      }
    }
  }
  if stty_state & 1i32 << 0i32 != 0 {
    let mut new_mode: termios = termios {
      c_iflag: 0,
      c_oflag: 0,
      c_cflag: 0,
      c_lflag: 0,
      c_line: 0,
      c_cc: [0; 32],
      c_ispeed: 0,
      c_ospeed: 0,
    };
    if tcsetattr(0i32, 1i32, &mut mode) != 0 {
      perror_on_device_and_die(b"%s\x00" as *const u8 as *const libc::c_char);
    }
    /* POSIX (according to Zlotnick's book) tcsetattr returns zero if
    it performs *any* of the requested operations.  This means it
    can report 'success' when it has actually failed to perform
    some proper subset of the requested operations.  To detect
    this partial failure, get the current terminal attributes and
    compare them to the requested ones */
    /* Initialize to all zeroes so there is no risk memcmp will report a
    spurious difference in an uninitialized portion of the structure */
    memset(
      &mut new_mode as *mut termios as *mut libc::c_void,
      0i32,
      ::std::mem::size_of::<termios>() as libc::c_ulong,
    );
    if tcgetattr(0i32, &mut new_mode) != 0 {
      perror_on_device_and_die(b"%s\x00" as *const u8 as *const libc::c_char);
    }
    if memcmp(
      &mut mode as *mut termios as *const libc::c_void,
      &mut new_mode as *mut termios as *const libc::c_void,
      ::std::mem::size_of::<termios>() as libc::c_ulong,
    ) != 0i32
    {
      /*
       * I think the below chunk is not necessary on Linux.
       * If you are deleting it, also delete STTY_speed_was_set bit -
       * it is only ever checked here.
       */
      /* was "if CIBAUD" */
      perror_on_device_and_die(
        b"%s: cannot perform all requested operations\x00" as *const u8 as *const libc::c_char,
      );
    }
  }
  return 0i32;
}
