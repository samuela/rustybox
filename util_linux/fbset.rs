use libc;
use libc::free;

extern "C" {
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;

  #[no_mangle]
  fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;

  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;

  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;

  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn xatoull(str: *const libc::c_char) -> libc::c_ulonglong;

  #[no_mangle]
  fn xatou(str: *const libc::c_char) -> libc::c_uint;

  /* Specialized */
  #[no_mangle]
  fn bb_show_usage() -> !;

  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;

  #[no_mangle]
  fn config_open(filename: *const libc::c_char) -> *mut parser_t;

  #[no_mangle]
  fn config_read(
    parser: *mut parser_t,
    tokens: *mut *mut libc::c_char,
    flags: libc::c_uint,
    delims: *const libc::c_char,
  ) -> libc::c_int;

  #[no_mangle]
  fn index_in_strings(strings: *const libc::c_char, key: *const libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn bb_xioctl(
    fd: libc::c_int,
    request: libc::c_uint,
    argp: *mut libc::c_void,
    ioctl_name: *const libc::c_char,
  ) -> libc::c_int;
}


use crate::librb::size_t;


use libc::FILE;

pub type C2RustUnnamed = libc::c_uint;
pub const PARSE_NORMAL: C2RustUnnamed = 4653056;
// pub const PARSE_WS_COMMENTS: C2RustUnnamed = 16777216;
// pub const PARSE_ALT_COMMENTS: C2RustUnnamed = 8388608;
// pub const PARSE_EOL_COMMENTS: C2RustUnnamed = 4194304;
// pub const PARSE_KEEP_COPY: C2RustUnnamed = 2097152;
// pub const PARSE_MIN_DIE: C2RustUnnamed = 1048576;
// pub const PARSE_GREEDY: C2RustUnnamed = 262144;
// pub const PARSE_TRIM: C2RustUnnamed = 131072;
// pub const PARSE_COLLAPSE: C2RustUnnamed = 65536;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct parser_t {
  pub fp: *mut FILE,
  pub data: *mut libc::c_char,
  pub line: *mut libc::c_char,
  pub nline: *mut libc::c_char,
  pub line_alloc: size_t,
  pub nline_alloc: size_t,
  pub lineno: libc::c_int,
}

pub type C2RustUnnamed_0 = libc::c_uint;
pub const FBIOPUT_VSCREENINFO: C2RustUnnamed_0 = 17921;
pub const FBIOGET_VSCREENINFO: C2RustUnnamed_0 = 17920;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct fb_bitfield {
  pub offset: u32,
  pub length: u32,
  pub msb_right: u32,
  /* !=0: Most significant bit is right */
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct fb_var_screeninfo {
  pub xres: u32,
  pub yres: u32,
  pub xres_virtual: u32,
  pub yres_virtual: u32,
  pub xoffset: u32,
  pub yoffset: u32,
  pub bits_per_pixel: u32,
  pub grayscale: u32,
  pub red: fb_bitfield,
  pub green: fb_bitfield,
  pub blue: fb_bitfield,
  pub transp: fb_bitfield,
  pub nonstd: u32,
  pub activate: u32,
  pub height: u32,
  pub width: u32,
  pub accel_flags: u32,
  pub pixclock: u32,
  pub left_margin: u32,
  pub right_margin: u32,
  pub upper_margin: u32,
  pub lower_margin: u32,
  pub hsync_len: u32,
  pub vsync_len: u32,
  pub sync: u32,
  pub vmode: u32,
  pub reserved: [u32; 6],
  /* Reserved for future compatibility */
}

pub type C2RustUnnamed_1 = libc::c_uint;
pub const CMD_MOVE: C2RustUnnamed_1 = 119;
pub const CMD_STEP: C2RustUnnamed_1 = 118;
pub const CMD_RGBA: C2RustUnnamed_1 = 117;
pub const CMD_BCAST: C2RustUnnamed_1 = 116;
pub const CMD_EXTSYNC: C2RustUnnamed_1 = 115;
pub const CMD_GSYNC: C2RustUnnamed_1 = 114;
pub const CMD_CSYNC: C2RustUnnamed_1 = 113;
pub const CMD_VSLEN: C2RustUnnamed_1 = 112;
pub const CMD_HSLEN: C2RustUnnamed_1 = 111;
pub const CMD_LOWER: C2RustUnnamed_1 = 110;
pub const CMD_UPPER: C2RustUnnamed_1 = 109;
pub const CMD_RIGHT: C2RustUnnamed_1 = 108;
pub const CMD_LEFT: C2RustUnnamed_1 = 107;
pub const CMD_PIXCLOCK: C2RustUnnamed_1 = 106;
pub const CMD_MATCH: C2RustUnnamed_1 = 105;
pub const CMD_DEPTH: C2RustUnnamed_1 = 104;
pub const CMD_VYRES: C2RustUnnamed_1 = 103;
pub const CMD_VXRES: C2RustUnnamed_1 = 102;
pub const CMD_YRES: C2RustUnnamed_1 = 101;
pub const CMD_XRES: C2RustUnnamed_1 = 100;
// pub const CMD_CHANGE: C2RustUnnamed_1 = 14;
pub const CMD_SHOW: C2RustUnnamed_1 = 13;
pub const CMD_INFO: C2RustUnnamed_1 = 12;
/*	CMD_XCOMPAT =     10, */
pub const CMD_ALL: C2RustUnnamed_1 = 11;
pub const CMD_DOUBLE: C2RustUnnamed_1 = 9;
pub const CMD_LACED: C2RustUnnamed_1 = 8;
pub const CMD_VSYNC: C2RustUnnamed_1 = 7;
pub const CMD_HSYNC: C2RustUnnamed_1 = 6;
pub const CMD_ACCEL: C2RustUnnamed_1 = 5;
pub const CMD_TIMING: C2RustUnnamed_1 = 4;
pub const CMD_GEOMETRY: C2RustUnnamed_1 = 3;
pub const CMD_DB: C2RustUnnamed_1 = 2;
pub const CMD_FB: C2RustUnnamed_1 = 1;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmdoptions_t {
  pub name: [libc::c_char; 9],
  pub param_count: libc::c_uchar,
  pub code: libc::c_uchar,
}

/* taken from linux/fb.h */
pub type C2RustUnnamed_2 = libc::c_uint;
/* composite sync high active */
/* external sync */
pub const FB_SYNC_COMP_HIGH_ACT: C2RustUnnamed_2 = 8;
/* double scan */
pub const FB_SYNC_EXT: C2RustUnnamed_2 = 4;
/* interlaced */
pub const FB_VMODE_DOUBLE: C2RustUnnamed_2 = 2;
/* vertical sync high active */
pub const FB_VMODE_INTERLACED: C2RustUnnamed_2 = 1;
/* horizontal sync high active */
pub const FB_SYNC_VERT_HIGH_ACT: C2RustUnnamed_2 = 2;
pub const FB_SYNC_HOR_HIGH_ACT: C2RustUnnamed_2 = 1;
pub const OPT_SHOW: C2RustUnnamed_3 = 2;
pub const OPT_ALL: C2RustUnnamed_3 = 8;
pub const OPT_CHANGE: C2RustUnnamed_3 = 1;
pub const OPT_READMODE: C2RustUnnamed_3 = 4;
pub type C2RustUnnamed_3 = libc::c_uint;

fn BUG_xatou32_unimplemented() -> u32 {
  panic!("BUG_xatou32_unimplemented")
}

#[inline(always)]
unsafe extern "C" fn xatoul(mut str: *const libc::c_char) -> libc::c_ulong {
  return xatoull(str) as libc::c_ulong;
}

#[inline(always)]
unsafe extern "C" fn xatou32(mut numstr: *const libc::c_char) -> u32 {
  if (2147483647i32 as libc::c_uint)
    .wrapping_mul(2u32)
    .wrapping_add(1u32)
    == 0xffffffffu32
  {
    return xatou(numstr);
  }
  if (9223372036854775807i64 as libc::c_ulong)
    .wrapping_mul(2u64)
    .wrapping_add(1u64)
    == 0xffffffffu32 as libc::c_ulong
  {
    return xatoul(numstr) as u32;
  }
  return BUG_xatou32_unimplemented();
}

unsafe extern "C" fn copy_if_gt0(
  mut src: *mut u32,
  mut dst: *mut u32,
  mut cnt: libc::c_uint,
) {
  loop {
    if *src as i32 > 0i32 {
      *dst = *src
    }
    src = src.offset(1);
    dst = dst.offset(1);
    cnt = cnt.wrapping_sub(1);
    if !(cnt != 0) {
      break;
    }
  }
}

#[inline(never)]
unsafe extern "C" fn copy_changed_values(
  mut base: *mut fb_var_screeninfo,
  mut set: *mut fb_var_screeninfo,
) {
  //if ((i32) set->xres > 0) base->xres = set->xres;
  //if ((i32) set->yres > 0) base->yres = set->yres;
  //if ((i32) set->xres_virtual > 0)   base->xres_virtual = set->xres_virtual;
  //if ((i32) set->yres_virtual > 0)   base->yres_virtual = set->yres_virtual;
  copy_if_gt0(&mut (*set).xres, &mut (*base).xres, 4i32 as libc::c_uint);
  if (*set).bits_per_pixel as i32 > 0i32 {
    (*base).bits_per_pixel = (*set).bits_per_pixel
  }
  //copy_if_gt0(&set->bits_per_pixel, &base->bits_per_pixel, 1);
  //if ((i32) set->pixclock > 0)       base->pixclock = set->pixclock;
  //if ((i32) set->left_margin > 0)    base->left_margin = set->left_margin;
  //if ((i32) set->right_margin > 0)   base->right_margin = set->right_margin;
  //if ((i32) set->upper_margin > 0)   base->upper_margin = set->upper_margin;
  //if ((i32) set->lower_margin > 0)   base->lower_margin = set->lower_margin;
  //if ((i32) set->hsync_len > 0) base->hsync_len = set->hsync_len;
  //if ((i32) set->vsync_len > 0) base->vsync_len = set->vsync_len;
  //if ((i32) set->sync > 0)  base->sync = set->sync;
  //if ((i32) set->vmode > 0) base->vmode = set->vmode;
  copy_if_gt0(
    &mut (*set).pixclock,
    &mut (*base).pixclock,
    9i32 as libc::c_uint,
  );
}
static mut g_cmdoptions: [cmdoptions_t; 36] = [
  {
    let mut init = cmdoptions_t {
      name: [102, 98, 0, 0, 0, 0, 0, 0, 0],
      param_count: 1i32 as libc::c_uchar,
      code: CMD_FB as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = cmdoptions_t {
      name: [100, 98, 0, 0, 0, 0, 0, 0, 0],
      param_count: 1i32 as libc::c_uchar,
      code: CMD_DB as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = cmdoptions_t {
      name: [97, 0, 0, 0, 0, 0, 0, 0, 0],
      param_count: 0i32 as libc::c_uchar,
      code: CMD_ALL as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = cmdoptions_t {
      name: [105, 0, 0, 0, 0, 0, 0, 0, 0],
      param_count: 0i32 as libc::c_uchar,
      code: CMD_INFO as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = cmdoptions_t {
      name: [103, 0, 0, 0, 0, 0, 0, 0, 0],
      param_count: 5i32 as libc::c_uchar,
      code: CMD_GEOMETRY as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = cmdoptions_t {
      name: [116, 0, 0, 0, 0, 0, 0, 0, 0],
      param_count: 7i32 as libc::c_uchar,
      code: CMD_TIMING as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = cmdoptions_t {
      name: [97, 99, 99, 101, 108, 0, 0, 0, 0],
      param_count: 1i32 as libc::c_uchar,
      code: CMD_ACCEL as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = cmdoptions_t {
      name: [104, 115, 121, 110, 99, 0, 0, 0, 0],
      param_count: 1i32 as libc::c_uchar,
      code: CMD_HSYNC as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = cmdoptions_t {
      name: [118, 115, 121, 110, 99, 0, 0, 0, 0],
      param_count: 1i32 as libc::c_uchar,
      code: CMD_VSYNC as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = cmdoptions_t {
      name: [108, 97, 99, 101, 100, 0, 0, 0, 0],
      param_count: 1i32 as libc::c_uchar,
      code: CMD_LACED as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = cmdoptions_t {
      name: [100, 111, 117, 98, 108, 101, 0, 0, 0],
      param_count: 1i32 as libc::c_uchar,
      code: CMD_DOUBLE as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = cmdoptions_t {
      name: [115, 104, 111, 119, 0, 0, 0, 0, 0],
      param_count: 0i32 as libc::c_uchar,
      code: CMD_SHOW as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = cmdoptions_t {
      name: [115, 0, 0, 0, 0, 0, 0, 0, 0],
      param_count: 0i32 as libc::c_uchar,
      code: CMD_SHOW as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = cmdoptions_t {
      name: [97, 108, 108, 0, 0, 0, 0, 0, 0],
      param_count: 0i32 as libc::c_uchar,
      code: CMD_ALL as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = cmdoptions_t {
      name: [120, 114, 101, 115, 0, 0, 0, 0, 0],
      param_count: 1i32 as libc::c_uchar,
      code: CMD_XRES as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = cmdoptions_t {
      name: [121, 114, 101, 115, 0, 0, 0, 0, 0],
      param_count: 1i32 as libc::c_uchar,
      code: CMD_YRES as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = cmdoptions_t {
      name: [118, 120, 114, 101, 115, 0, 0, 0, 0],
      param_count: 1i32 as libc::c_uchar,
      code: CMD_VXRES as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = cmdoptions_t {
      name: [118, 121, 114, 101, 115, 0, 0, 0, 0],
      param_count: 1i32 as libc::c_uchar,
      code: CMD_VYRES as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = cmdoptions_t {
      name: [100, 101, 112, 116, 104, 0, 0, 0, 0],
      param_count: 1i32 as libc::c_uchar,
      code: CMD_DEPTH as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = cmdoptions_t {
      name: [109, 97, 116, 99, 104, 0, 0, 0, 0],
      param_count: 0i32 as libc::c_uchar,
      code: CMD_MATCH as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = cmdoptions_t {
      name: [103, 101, 111, 109, 101, 116, 114, 121, 0],
      param_count: 5i32 as libc::c_uchar,
      code: CMD_GEOMETRY as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = cmdoptions_t {
      name: [112, 105, 120, 99, 108, 111, 99, 107, 0],
      param_count: 1i32 as libc::c_uchar,
      code: CMD_PIXCLOCK as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = cmdoptions_t {
      name: [108, 101, 102, 116, 0, 0, 0, 0, 0],
      param_count: 1i32 as libc::c_uchar,
      code: CMD_LEFT as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = cmdoptions_t {
      name: [114, 105, 103, 104, 116, 0, 0, 0, 0],
      param_count: 1i32 as libc::c_uchar,
      code: CMD_RIGHT as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = cmdoptions_t {
      name: [117, 112, 112, 101, 114, 0, 0, 0, 0],
      param_count: 1i32 as libc::c_uchar,
      code: CMD_UPPER as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = cmdoptions_t {
      name: [108, 111, 119, 101, 114, 0, 0, 0, 0],
      param_count: 1i32 as libc::c_uchar,
      code: CMD_LOWER as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = cmdoptions_t {
      name: [104, 115, 108, 101, 110, 0, 0, 0, 0],
      param_count: 1i32 as libc::c_uchar,
      code: CMD_HSLEN as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = cmdoptions_t {
      name: [118, 115, 108, 101, 110, 0, 0, 0, 0],
      param_count: 1i32 as libc::c_uchar,
      code: CMD_VSLEN as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = cmdoptions_t {
      name: [116, 105, 109, 105, 110, 103, 115, 0, 0],
      param_count: 7i32 as libc::c_uchar,
      code: CMD_TIMING as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = cmdoptions_t {
      name: [99, 115, 121, 110, 99, 0, 0, 0, 0],
      param_count: 1i32 as libc::c_uchar,
      code: CMD_CSYNC as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = cmdoptions_t {
      name: [103, 115, 121, 110, 99, 0, 0, 0, 0],
      param_count: 1i32 as libc::c_uchar,
      code: CMD_GSYNC as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = cmdoptions_t {
      name: [101, 120, 116, 115, 121, 110, 99, 0, 0],
      param_count: 1i32 as libc::c_uchar,
      code: CMD_EXTSYNC as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = cmdoptions_t {
      name: [98, 99, 97, 115, 116, 0, 0, 0, 0],
      param_count: 1i32 as libc::c_uchar,
      code: CMD_BCAST as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = cmdoptions_t {
      name: [114, 103, 98, 97, 0, 0, 0, 0, 0],
      param_count: 1i32 as libc::c_uchar,
      code: CMD_RGBA as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = cmdoptions_t {
      name: [115, 116, 101, 112, 0, 0, 0, 0, 0],
      param_count: 1i32 as libc::c_uchar,
      code: CMD_STEP as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = cmdoptions_t {
      name: [109, 111, 118, 101, 0, 0, 0, 0, 0],
      param_count: 1i32 as libc::c_uchar,
      code: CMD_MOVE as libc::c_int as libc::c_uchar,
    };
    init
  },
];

unsafe extern "C" fn ss(
  mut x: *mut u32,
  mut flag: u32,
  mut buf: *mut libc::c_char,
  mut what: *const libc::c_char,
) {
  if strcmp(buf, what) == 0i32 {
    *x &= !flag
  } else {
    *x |= flag
  };
}

/* Mode db file contains mode definitions like this:
 * mode "800x600-48-lace"
 *     # D: 36.00 MHz, H: 33.835 kHz, V: 96.39 Hz
 *     geometry 800 600 800 600 8
 *     timings 27778 56 80 79 11 128 12
 *     laced true
 *     hsync high
 *     vsync high
 * endmode
 */
unsafe extern "C" fn read_mode_db(
  mut base: *mut fb_var_screeninfo,
  mut fn_0: *const libc::c_char,
  mut mode: *const libc::c_char,
) -> libc::c_int {
  let mut token: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2];
  let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut parser: *mut parser_t = config_open(fn_0);
  while config_read(
    parser,
    token.as_mut_ptr(),
    (PARSE_NORMAL as libc::c_int | (1i32 & 0xffi32) << 8i32 | 2i32 & 0xffi32) as libc::c_uint,
    b"# \t\r\x00" as *const u8 as *const libc::c_char,
  ) != 0
  {
    if strcmp(token[0], b"mode\x00" as *const u8 as *const libc::c_char) != 0i32
      || token[1].is_null()
    {
      continue;
    }
    p = strstr(token[1], mode);
    if p.is_null() {
      continue;
    }
    s = p.offset(strlen(mode) as isize);
    //bb_error_msg("CHECK[%s][%s][%d]", mode, p-1, *s);
    /* exact match? */
    if !((*s == 0
      || ({
        let mut bb__isspace: libc::c_uchar = (*s as libc::c_int - 9i32) as libc::c_uchar;
        (bb__isspace as libc::c_int == ' ' as i32 - 9i32
          || bb__isspace as libc::c_int <= 13i32 - 9i32) as libc::c_int
      }) != 0)
      && '\"' as i32 != *s.offset(-1i32 as isize) as libc::c_int
      || '\"' as i32 == *s as libc::c_int
        && '\"' as i32 == *p.offset(-1i32 as isize) as libc::c_int)
    {
      continue;
    }
    /* ends with " but starts with " too! */
    break;
  }
  if token[0].is_null() {
    return 0i32;
  }
  while config_read(
    parser,
    token.as_mut_ptr(),
    (PARSE_NORMAL as libc::c_int | (1i32 & 0xffi32) << 8i32 | 2i32 & 0xffi32) as libc::c_uint,
    b"# \t\x00" as *const u8 as *const libc::c_char,
  ) != 0
  {
    let mut i: libc::c_int = 0;
    //bb_error_msg("???[%s][%s]", token[0], token[1]);
    if strcmp(token[0], b"endmode\x00" as *const u8 as *const libc::c_char) == 0i32 {
      //bb_error_msg("OK[%s]", mode);
      return 1i32;
    } /* for compiler */
    p = token[1]; /* set all to -1 */
    i =
            index_in_strings(b"geometry\x00timings\x00interlaced\x00double\x00vsync\x00hsync\x00csync\x00extsync\x00rgba\x00\x00"
                                 as *const u8 as *const libc::c_char,
                             token[0]);
    match i {
      0 => {
        if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
          == ::std::mem::size_of::<u32>() as libc::c_ulong
        {
          sscanf(
            p,
            b"%d %d %d %d %d\x00" as *const u8 as *const libc::c_char,
            &mut (*base).xres as *mut u32,
            &mut (*base).yres as *mut u32,
            &mut (*base).xres_virtual as *mut u32,
            &mut (*base).yres_virtual as *mut u32,
            &mut (*base).bits_per_pixel as *mut u32,
          );
        } else {
          let mut base_xres: libc::c_int = 0;
          let mut base_yres: libc::c_int = 0;
          let mut base_xres_virtual: libc::c_int = 0;
          let mut base_yres_virtual: libc::c_int = 0;
          let mut base_bits_per_pixel: libc::c_int = 0;
          sscanf(
            p,
            b"%d %d %d %d %d\x00" as *const u8 as *const libc::c_char,
            &mut base_xres as *mut libc::c_int,
            &mut base_yres as *mut libc::c_int,
            &mut base_xres_virtual as *mut libc::c_int,
            &mut base_yres_virtual as *mut libc::c_int,
            &mut base_bits_per_pixel as *mut libc::c_int,
          );
          (*base).xres = base_xres as u32;
          (*base).yres = base_yres as u32;
          (*base).xres_virtual = base_xres_virtual as u32;
          (*base).yres_virtual = base_yres_virtual as u32;
          (*base).bits_per_pixel = base_bits_per_pixel as u32
        }
      }
      1 => {
        if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
          == ::std::mem::size_of::<u32>() as libc::c_ulong
        {
          sscanf(
            p,
            b"%d %d %d %d %d %d %d\x00" as *const u8 as *const libc::c_char,
            &mut (*base).pixclock as *mut u32,
            &mut (*base).left_margin as *mut u32,
            &mut (*base).right_margin as *mut u32,
            &mut (*base).upper_margin as *mut u32,
            &mut (*base).lower_margin as *mut u32,
            &mut (*base).hsync_len as *mut u32,
            &mut (*base).vsync_len as *mut u32,
          );
        } else {
          let mut base_pixclock: libc::c_int = 0;
          let mut base_left_margin: libc::c_int = 0;
          let mut base_right_margin: libc::c_int = 0;
          let mut base_upper_margin: libc::c_int = 0;
          let mut base_lower_margin: libc::c_int = 0;
          let mut base_hsync_len: libc::c_int = 0;
          let mut base_vsync_len: libc::c_int = 0;
          sscanf(
            p,
            b"%d %d %d %d %d %d %d\x00" as *const u8 as *const libc::c_char,
            &mut base_pixclock as *mut libc::c_int,
            &mut base_left_margin as *mut libc::c_int,
            &mut base_right_margin as *mut libc::c_int,
            &mut base_upper_margin as *mut libc::c_int,
            &mut base_lower_margin as *mut libc::c_int,
            &mut base_hsync_len as *mut libc::c_int,
            &mut base_vsync_len as *mut libc::c_int,
          );
          (*base).pixclock = base_pixclock as u32;
          (*base).left_margin = base_left_margin as u32;
          (*base).right_margin = base_right_margin as u32;
          (*base).upper_margin = base_upper_margin as u32;
          (*base).lower_margin = base_lower_margin as u32;
          (*base).hsync_len = base_hsync_len as u32;
          (*base).vsync_len = base_vsync_len as u32
        }
      }
      2 | 3 => {
        static mut syncs: [u32; 2] = [
          FB_VMODE_INTERLACED as libc::c_int as u32,
          FB_VMODE_DOUBLE as libc::c_int as u32,
        ];
        ss(
          &mut (*base).vmode,
          syncs[(i - 2i32) as usize],
          p,
          b"false\x00" as *const u8 as *const libc::c_char,
        );
      }
      4 | 5 | 6 => {
        static mut syncs_0: [u32; 3] = [
          FB_SYNC_VERT_HIGH_ACT as libc::c_int as u32,
          FB_SYNC_HOR_HIGH_ACT as libc::c_int as u32,
          FB_SYNC_COMP_HIGH_ACT as libc::c_int as u32,
        ];
        ss(
          &mut (*base).sync,
          syncs_0[(i - 4i32) as usize],
          p,
          b"low\x00" as *const u8 as *const libc::c_char,
        );
      }
      7 => {
        ss(
          &mut (*base).sync,
          FB_SYNC_EXT as libc::c_int as u32,
          p,
          b"false\x00" as *const u8 as *const libc::c_char,
        );
      }
      8 => {
        let mut red_offset: libc::c_int = 0;
        let mut red_length: libc::c_int = 0;
        let mut green_offset: libc::c_int = 0;
        let mut green_length: libc::c_int = 0;
        let mut blue_offset: libc::c_int = 0;
        let mut blue_length: libc::c_int = 0;
        let mut transp_offset: libc::c_int = 0;
        let mut transp_length: libc::c_int = 0;
        sscanf(
          p,
          b"%d/%d,%d/%d,%d/%d,%d/%d\x00" as *const u8 as *const libc::c_char,
          &mut red_length as *mut libc::c_int,
          &mut red_offset as *mut libc::c_int,
          &mut green_length as *mut libc::c_int,
          &mut green_offset as *mut libc::c_int,
          &mut blue_length as *mut libc::c_int,
          &mut blue_offset as *mut libc::c_int,
          &mut transp_length as *mut libc::c_int,
          &mut transp_offset as *mut libc::c_int,
        );
        (*base).red.offset = red_offset as u32;
        (*base).red.length = red_length as u32;
        (*base).red.msb_right = 0i32 as u32;
        (*base).green.offset = green_offset as u32;
        (*base).green.length = green_length as u32;
        (*base).green.msb_right = 0i32 as u32;
        (*base).blue.offset = blue_offset as u32;
        (*base).blue.length = blue_length as u32;
        (*base).blue.msb_right = 0i32 as u32;
        (*base).transp.offset = transp_offset as u32;
        (*base).transp.length = transp_length as u32;
        (*base).transp.msb_right = 0i32 as u32
      }
      _ => {}
    }
  }
  return 0i32;
}

#[inline(never)]
unsafe extern "C" fn showmode(mut v: *mut fb_var_screeninfo) {
  let mut drate: libc::c_double = 0i32 as libc::c_double;
  let mut hrate: libc::c_double = 0i32 as libc::c_double;
  let mut vrate: libc::c_double = 0i32 as libc::c_double;
  if (*v).pixclock != 0 {
    drate = 1e12f64 / (*v).pixclock as libc::c_double;
    hrate = drate
      / (*v)
        .left_margin
        .wrapping_add((*v).xres)
        .wrapping_add((*v).right_margin)
        .wrapping_add((*v).hsync_len) as libc::c_double;
    vrate = hrate
      / (*v)
        .upper_margin
        .wrapping_add((*v).yres)
        .wrapping_add((*v).lower_margin)
        .wrapping_add((*v).vsync_len) as libc::c_double
  }
  printf(b"\nmode \"%ux%u-%u\"\n\t# D: %.3f MHz, H: %.3f kHz, V: %.3f Hz\n\tgeometry %u %u %u %u %u\n\ttimings %u %u %u %u %u %u %u\n\taccel %s\n\trgba %u/%u,%u/%u,%u/%u,%u/%u\nendmode\n\n\x00"
               as *const u8 as *const libc::c_char, (*v).xres, (*v).yres,
           (vrate + 0.5f64) as libc::c_int, drate / 1e6f64, hrate / 1e3f64,
           vrate, (*v).xres, (*v).yres, (*v).xres_virtual, (*v).yres_virtual,
           (*v).bits_per_pixel, (*v).pixclock, (*v).left_margin,
           (*v).right_margin, (*v).upper_margin, (*v).lower_margin,
           (*v).hsync_len, (*v).vsync_len,
           if (*v).accel_flags > 0i32 as libc::c_uint {
               b"true\x00" as *const u8 as *const libc::c_char
           } else { b"false\x00" as *const u8 as *const libc::c_char },
           (*v).red.length, (*v).red.offset, (*v).green.length,
           (*v).green.offset, (*v).blue.length, (*v).blue.offset,
           (*v).transp.length, (*v).transp.offset);
}

#[no_mangle]
pub unsafe extern "C" fn fbset_main(
  mut argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut current_block: u64;
  let mut var_old: fb_var_screeninfo = fb_var_screeninfo {
    xres: 0,
    yres: 0,
    xres_virtual: 0,
    yres_virtual: 0,
    xoffset: 0,
    yoffset: 0,
    bits_per_pixel: 0,
    grayscale: 0,
    red: fb_bitfield {
      offset: 0,
      length: 0,
      msb_right: 0,
    },
    green: fb_bitfield {
      offset: 0,
      length: 0,
      msb_right: 0,
    },
    blue: fb_bitfield {
      offset: 0,
      length: 0,
      msb_right: 0,
    },
    transp: fb_bitfield {
      offset: 0,
      length: 0,
      msb_right: 0,
    },
    nonstd: 0,
    activate: 0,
    height: 0,
    width: 0,
    accel_flags: 0,
    pixclock: 0,
    left_margin: 0,
    right_margin: 0,
    upper_margin: 0,
    lower_margin: 0,
    hsync_len: 0,
    vsync_len: 0,
    sync: 0,
    vmode: 0,
    reserved: [0; 6],
  };
  let mut var_set: fb_var_screeninfo = fb_var_screeninfo {
    xres: 0,
    yres: 0,
    xres_virtual: 0,
    yres_virtual: 0,
    xoffset: 0,
    yoffset: 0,
    bits_per_pixel: 0,
    grayscale: 0,
    red: fb_bitfield {
      offset: 0,
      length: 0,
      msb_right: 0,
    },
    green: fb_bitfield {
      offset: 0,
      length: 0,
      msb_right: 0,
    },
    blue: fb_bitfield {
      offset: 0,
      length: 0,
      msb_right: 0,
    },
    transp: fb_bitfield {
      offset: 0,
      length: 0,
      msb_right: 0,
    },
    nonstd: 0,
    activate: 0,
    height: 0,
    width: 0,
    accel_flags: 0,
    pixclock: 0,
    left_margin: 0,
    right_margin: 0,
    upper_margin: 0,
    lower_margin: 0,
    hsync_len: 0,
    vsync_len: 0,
    sync: 0,
    vmode: 0,
    reserved: [0; 6],
  };
  let mut fh: libc::c_int = 0;
  let mut i: libc::c_int = 0;
  let mut options: libc::c_uint = 0i32 as libc::c_uint;
  let mut fbdev: *const libc::c_char = b"/dev/fb0\x00" as *const u8 as *const libc::c_char;
  let mut modefile: *const libc::c_char = b"/etc/fb.modes\x00" as *const u8 as *const libc::c_char;
  let mut thisarg: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut mode: *mut libc::c_char = 0 as *mut libc::c_char;
  mode = mode;
  memset(
    &mut var_set as *mut fb_var_screeninfo as *mut libc::c_void,
    0xffi32,
    ::std::mem::size_of::<fb_var_screeninfo>() as libc::c_ulong,
  );
  /* parse cmd args.... why do they have to make things so difficult? */
  argv = argv.offset(1);
  argc -= 1;
  while argc > 0i32 && {
    thisarg = *argv;
    !thisarg.is_null()
  } {
    if *thisarg.offset(0) as libc::c_int != '-' as i32 {
      if 1i32 == 0 || argc != 1i32 {
        bb_show_usage();
      }
      mode = thisarg;
      options |= OPT_READMODE as libc::c_int as libc::c_uint
    } else {
      i = 0i32;
      loop {
        if !((i as libc::c_uint)
          < (::std::mem::size_of::<[cmdoptions_t; 36]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<cmdoptions_t>() as libc::c_ulong)
            as libc::c_uint)
        {
          current_block = 13321564401369230990;
          break;
        }
        if strcmp(thisarg.offset(1), g_cmdoptions[i as usize].name.as_ptr()) != 0i32 {
          i += 1
        } else {
          if argc <= g_cmdoptions[i as usize].param_count as libc::c_int {
            bb_show_usage();
          }
          match g_cmdoptions[i as usize].code as libc::c_int {
            1 => fbdev = *argv.offset(1),
            2 => modefile = *argv.offset(1),
            11 => options |= OPT_ALL as libc::c_int as libc::c_uint,
            13 => options |= OPT_SHOW as libc::c_int as libc::c_uint,
            3 => {
              var_set.xres = xatou32(*argv.offset(1));
              var_set.yres = xatou32(*argv.offset(2));
              var_set.xres_virtual = xatou32(*argv.offset(3));
              var_set.yres_virtual = xatou32(*argv.offset(4));
              var_set.bits_per_pixel = xatou32(*argv.offset(5))
            }
            4 => {
              var_set.pixclock = xatou32(*argv.offset(1));
              var_set.left_margin = xatou32(*argv.offset(2));
              var_set.right_margin = xatou32(*argv.offset(3));
              var_set.upper_margin = xatou32(*argv.offset(4));
              var_set.lower_margin = xatou32(*argv.offset(5));
              var_set.hsync_len = xatou32(*argv.offset(6));
              var_set.vsync_len = xatou32(*argv.offset(7))
            }
            6 => var_set.sync |= FB_SYNC_HOR_HIGH_ACT as libc::c_int as libc::c_uint,
            7 => var_set.sync |= FB_SYNC_VERT_HIGH_ACT as libc::c_int as libc::c_uint,
            100 => var_set.xres = xatou32(*argv.offset(1)),
            101 => var_set.yres = xatou32(*argv.offset(1)),
            104 => var_set.bits_per_pixel = xatou32(*argv.offset(1)),
            5 | _ => {}
          }
          match g_cmdoptions[i as usize].code as libc::c_int {
            1 | 2 | 11 | 13 => {}
            _ => {
              /* other commands imply changes */
              options |= OPT_CHANGE as libc::c_int as libc::c_uint
            }
          }
          argc -= g_cmdoptions[i as usize].param_count as libc::c_int;
          argv = argv.offset(g_cmdoptions[i as usize].param_count as libc::c_int as isize);
          current_block = 10886091980245723256;
          break;
        }
      }
      match current_block {
        10886091980245723256 => {}
        _ => {
          bb_show_usage();
        }
      }
    }
    argc -= 1;
    argv = argv.offset(1)
  }
  fh = xopen(fbdev, 0i32);
  bb_xioctl(
    fh,
    FBIOGET_VSCREENINFO as libc::c_int as libc::c_uint,
    &mut var_old as *mut fb_var_screeninfo as *mut libc::c_void,
    b"FBIOGET_VSCREENINFO\x00" as *const u8 as *const libc::c_char,
  );
  if options & OPT_READMODE as libc::c_int as libc::c_uint != 0 {
    if read_mode_db(&mut var_old, modefile, mode) == 0 {
      bb_error_msg_and_die(
        b"unknown video mode \'%s\'\x00" as *const u8 as *const libc::c_char,
        mode,
      );
    }
    options |= OPT_CHANGE as libc::c_int as libc::c_uint
  }
  if options & OPT_CHANGE as libc::c_int as libc::c_uint != 0 {
    copy_changed_values(&mut var_old, &mut var_set);
    if options & OPT_ALL as libc::c_int as libc::c_uint != 0 {
      var_old.activate = 64i32 as u32
    }
    bb_xioctl(
      fh,
      FBIOPUT_VSCREENINFO as libc::c_int as libc::c_uint,
      &mut var_old as *mut fb_var_screeninfo as *mut libc::c_void,
      b"FBIOPUT_VSCREENINFO\x00" as *const u8 as *const libc::c_char,
    );
  }
  if options == 0i32 as libc::c_uint || options & OPT_SHOW as libc::c_int as libc::c_uint != 0 {
    showmode(&mut var_old);
  }
  return 0i32;
}
