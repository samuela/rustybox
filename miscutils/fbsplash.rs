use libc;
use libc::unlink;
use libc::close;
use libc::free;
extern "C" {
  #[no_mangle]
  fn atoi(__nptr: *const libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
  #[no_mangle]
  static mut stdin: *mut FILE;
  #[no_mangle]
  fn fclose(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn fgets_unlocked(
    __s: *mut libc::c_char,
    __n: libc::c_int,
    __stream: *mut FILE,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn fread(__ptr: *mut libc::c_void, __size: size_t, __n: size_t, __stream: *mut FILE) -> size_t;

  #[no_mangle]
  fn strchrnul(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn mmap(
    __addr: *mut libc::c_void,
    __len: size_t,
    __prot: libc::c_int,
    __flags: libc::c_int,
    __fd: libc::c_int,
    __offset: off64_t,
  ) -> *mut libc::c_void;
  #[no_mangle]
  fn xmalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn is_prefixed_with(string: *const libc::c_char, key: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn open_zipped(fname: *const libc::c_char, fail_if_not_compressed: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn full_write(fd: libc::c_int, buf: *const libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn xmalloc_fgetline(file: *mut FILE) -> *mut libc::c_char;
  #[no_mangle]
  fn xfopen_stdin(filename: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn xfdopen_for_read(fd: libc::c_int) -> *mut FILE;
  #[no_mangle]
  fn xatoi_positive(numstr: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn config_open2(
    filename: *const libc::c_char,
    fopen_func: Option<unsafe extern "C" fn(_: *const libc::c_char) -> *mut FILE>,
  ) -> *mut parser_t;
  #[no_mangle]
  fn config_read(
    parser: *mut parser_t,
    tokens: *mut *mut libc::c_char,
    flags: libc::c_uint,
    delims: *const libc::c_char,
  ) -> libc::c_int;
  #[no_mangle]
  fn config_close(parser: *mut parser_t);
  #[no_mangle]
  fn index_in_strings(strings: *const libc::c_char, key: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn bb_xioctl(
    fd: libc::c_int,
    request: libc::c_uint,
    argp: *mut libc::c_void,
    ioctl_name: *const libc::c_char,
  ) -> libc::c_int;
  #[no_mangle]
  static ptr_to_globals: *mut globals;
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
}

use libc::off64_t;

use crate::librb::size_t;
use libc::ssize_t;



use libc::FILE;
pub type C2RustUnnamed = libc::c_uint;
pub const PARSE_NORMAL: C2RustUnnamed = 4653056;
pub const PARSE_WS_COMMENTS: C2RustUnnamed = 16777216;
pub const PARSE_ALT_COMMENTS: C2RustUnnamed = 8388608;
pub const PARSE_EOL_COMMENTS: C2RustUnnamed = 4194304;
pub const PARSE_KEEP_COPY: C2RustUnnamed = 2097152;
pub const PARSE_MIN_DIE: C2RustUnnamed = 1048576;
pub const PARSE_GREEDY: C2RustUnnamed = 262144;
pub const PARSE_TRIM: C2RustUnnamed = 131072;
pub const PARSE_COLLAPSE: C2RustUnnamed = 65536;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub addr: *mut libc::c_uchar,
  pub ns: [libc::c_uint; 9],
  pub image_filename: *const libc::c_char,
  pub scr_var: fb_var_screeninfo,
  pub scr_fix: fb_fix_screeninfo,
  pub bytes_per_pixel: libc::c_uint,
  pub red_shift: libc::c_uint,
  pub green_shift: libc::c_uint,
  pub blue_shift: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fb_fix_screeninfo {
  pub id: [libc::c_char; 16],
  pub smem_start: libc::c_ulong,
  pub smem_len: u32,
  pub type_0: u32,
  pub type_aux: u32,
  pub visual: u32,
  pub xpanstep: __u16,
  pub ypanstep: __u16,
  pub ywrapstep: __u16,
  pub line_length: u32,
  pub mmio_start: libc::c_ulong,
  pub mmio_len: u32,
  pub accel: u32,
  pub capabilities: __u16,
  pub reserved: [__u16; 2],
}
pub type __u16 = libc::c_ushort;
pub type u32 = libc::c_uint;
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
  pub rotate: u32,
  pub colorspace: u32,
  pub reserved: [u32; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fb_bitfield {
  pub offset: u32,
  pub length: u32,
  pub msb_right: u32,
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const COMMON_BUFSIZE: C2RustUnnamed_0 = 1024;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fb_cmap {
  pub start: u32,
  pub len: u32,
  pub red: *mut __u16,
  pub green: *mut __u16,
  pub blue: *mut __u16,
  pub transp: *mut __u16,
}
#[inline(always)]
unsafe extern "C" fn not_const_pp(mut p: *const libc::c_void) -> *mut libc::c_void {
  return p as *mut libc::c_void;
}
// progress bar width
// progress bar height
// progress bar horizontal position
// progress bar vertical position
// progress bar color red component
// progress bar color green component
// progress bar color blue component
// image horizontal position
// image vertical position
/* *
 * Configure palette for RGB:332
 */
unsafe extern "C" fn fb_setpal(mut fd: libc::c_int) {
  let mut cmap: fb_cmap = fb_cmap {
    start: 0,
    len: 0,
    red: 0 as *mut __u16,
    green: 0 as *mut __u16,
    blue: 0 as *mut __u16,
    transp: 0 as *mut __u16,
  };
  /* fb colors are 16 bit */
  let mut red: [libc::c_ushort; 256] = [0; 256];
  let mut green: [libc::c_ushort; 256] = [0; 256];
  let mut blue: [libc::c_ushort; 256] = [0; 256];
  let mut i: libc::c_uint = 0;
  /* RGB:332 */
  i = 0i32 as libc::c_uint;
  while i < 256i32 as libc::c_uint {
    /* Color is encoded in pixel value as rrrgggbb.
     * 3-bit color is mapped to 16-bit one as:
     * 000 -> 00000000 00000000
     * 001 -> 00100100 10010010
     * ...
     * 011 -> 01101101 10110110
     * 100 -> 10010010 01001001
     * ...
     * 111 -> 11111111 11111111
     */
    red[i as usize] =
      ((i >> 5i32).wrapping_mul(0x9249i32 as libc::c_uint) >> 2i32) as libc::c_ushort; // rrr * 00 10010010 01001001 >> 2
                                                                                       // bb * 01010101 01010101
    green[i as usize] = ((i >> 2i32 & 0x7i32 as libc::c_uint)
      .wrapping_mul(0x9249i32 as libc::c_uint)
      >> 2i32) as libc::c_ushort; // ggg * 00 10010010 01001001 >> 2
    blue[i as usize] =
      (i & 0x3i32 as libc::c_uint).wrapping_mul(0x5555i32 as libc::c_uint) as libc::c_ushort;
    i = i.wrapping_add(1)
  }
  cmap.start = 0i32 as u32;
  cmap.len = 256i32 as u32;
  cmap.red = red.as_mut_ptr();
  cmap.green = green.as_mut_ptr();
  cmap.blue = blue.as_mut_ptr();
  cmap.transp = 0 as *mut __u16;
  bb_xioctl(
    fd,
    0x4605i32 as libc::c_uint,
    &mut cmap as *mut fb_cmap as *mut libc::c_void,
    b"FBIOPUTCMAP\x00" as *const u8 as *const libc::c_char,
  );
}
/* 2-bit color is easier: */
/* *
 * Open and initialize the framebuffer device
 * \param *strfb_device pointer to framebuffer device
 */
unsafe extern "C" fn fb_open(mut strfb_device: *const libc::c_char) {
  let mut fbfd: libc::c_int = xopen(strfb_device, 0o2i32);
  // framebuffer properties
  bb_xioctl(
    fbfd,
    0x4600i32 as libc::c_uint,
    &mut (*ptr_to_globals).scr_var as *mut fb_var_screeninfo as *mut libc::c_void,
    b"FBIOGET_VSCREENINFO\x00" as *const u8 as *const libc::c_char,
  );
  bb_xioctl(
    fbfd,
    0x4602i32 as libc::c_uint,
    &mut (*ptr_to_globals).scr_fix as *mut fb_fix_screeninfo as *mut libc::c_void,
    b"FBIOGET_FSCREENINFO\x00" as *const u8 as *const libc::c_char,
  );
  match (*ptr_to_globals).scr_var.bits_per_pixel {
    8 => {
      fb_setpal(fbfd);
    }
    16 | 24 | 32 => {}
    _ => {
      bb_error_msg_and_die(
        b"unsupported %u bpp\x00" as *const u8 as *const libc::c_char,
        (*ptr_to_globals).scr_var.bits_per_pixel as libc::c_int,
      );
    }
  }
  (*ptr_to_globals).red_shift =
    (8i32 as libc::c_uint).wrapping_sub((*ptr_to_globals).scr_var.red.length);
  (*ptr_to_globals).green_shift =
    (8i32 as libc::c_uint).wrapping_sub((*ptr_to_globals).scr_var.green.length);
  (*ptr_to_globals).blue_shift =
    (8i32 as libc::c_uint).wrapping_sub((*ptr_to_globals).scr_var.blue.length);
  (*ptr_to_globals).bytes_per_pixel = (*ptr_to_globals)
    .scr_var
    .bits_per_pixel
    .wrapping_add(7i32 as libc::c_uint)
    >> 3i32;
  // map the device in memory
  (*ptr_to_globals).addr = mmap(
    0 as *mut libc::c_void,
    if (*ptr_to_globals).scr_var.yres_virtual != 0 {
      (*ptr_to_globals).scr_var.yres_virtual
    } else {
      (*ptr_to_globals).scr_var.yres
    }
    .wrapping_mul((*ptr_to_globals).scr_fix.line_length) as size_t,
    0x2i32,
    0x1i32,
    fbfd,
    0i32 as off64_t,
  ) as *mut libc::c_uchar;
  if (*ptr_to_globals).addr == -1i32 as *mut libc::c_void as *mut libc::c_uchar {
    bb_simple_perror_msg_and_die(b"mmap\x00" as *const u8 as *const libc::c_char);
  }
  // point to the start of the visible screen
  (*ptr_to_globals).addr = (*ptr_to_globals).addr.offset(
    (*ptr_to_globals)
      .scr_var
      .yoffset
      .wrapping_mul((*ptr_to_globals).scr_fix.line_length)
      .wrapping_add(
        (*ptr_to_globals)
          .scr_var
          .xoffset
          .wrapping_mul((*ptr_to_globals).bytes_per_pixel),
      ) as isize,
  );
  close(fbfd);
}
/* *
 * Return pixel value of the passed RGB color.
 * This is performance critical fn.
 */
unsafe extern "C" fn fb_pixel_value(
  mut r: libc::c_uint,
  mut g: libc::c_uint,
  mut b: libc::c_uint,
) -> libc::c_uint {
  /* We assume that the r,g,b values are <= 255 */
  if (*ptr_to_globals).bytes_per_pixel == 1i32 as libc::c_uint {
    r = r & 0xe0i32 as libc::c_uint; // 3-bit red
    g = g >> 3i32 & 0x1ci32 as libc::c_uint; // 3-bit green
    b = b >> 6i32; // 2-bit blue
    return r.wrapping_add(g).wrapping_add(b);
  }
  if (*ptr_to_globals).bytes_per_pixel == 2i32 as libc::c_uint {
    // ARM PL110 on Integrator/CP has RGBA5551 bit arrangement.
    // We want to support bit locations like that.
    //
    // First shift out unused bits
    r = r >> (*ptr_to_globals).red_shift;
    g = g >> (*ptr_to_globals).green_shift;
    b = b >> (*ptr_to_globals).blue_shift;
    // Then shift the remaining bits to their offset
    return (r << (*ptr_to_globals).scr_var.red.offset)
      .wrapping_add(g << (*ptr_to_globals).scr_var.green.offset)
      .wrapping_add(b << (*ptr_to_globals).scr_var.blue.offset);
  }
  // RGB 888
  return b.wrapping_add(g << 8i32).wrapping_add(r << 16i32);
}
/* *
 * Draw pixel on framebuffer
 */
unsafe extern "C" fn fb_write_pixel(mut addr: *mut libc::c_uchar, mut pixel: libc::c_uint) {
  match (*ptr_to_globals).bytes_per_pixel {
    1 => *addr = pixel as libc::c_uchar,
    2 => *(addr as *mut u16) = pixel as u16,
    4 => *(addr as *mut u32) = pixel,
    _ => {
      // 24 bits per pixel
      *addr.offset(0) = pixel as libc::c_uchar;
      *addr.offset(1) = (pixel >> 8i32) as libc::c_uchar;
      *addr.offset(2) = (pixel >> 16i32) as libc::c_uchar
    }
  };
}
/* *
 * Draw hollow rectangle on framebuffer
 */
unsafe extern "C" fn fb_drawrectangle() {
  let mut cnt: libc::c_int = 0;
  let mut thispix: libc::c_uint = 0;
  let mut ptr1: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
  let mut ptr2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
  let mut nred: libc::c_uchar =
    (*ptr_to_globals).ns[4].wrapping_div(2i32 as libc::c_uint) as libc::c_uchar;
  let mut ngreen: libc::c_uchar =
    (*ptr_to_globals).ns[5].wrapping_div(2i32 as libc::c_uint) as libc::c_uchar;
  let mut nblue: libc::c_uchar =
    (*ptr_to_globals).ns[6].wrapping_div(2i32 as libc::c_uint) as libc::c_uchar;
  thispix = fb_pixel_value(
    nred as libc::c_uint,
    ngreen as libc::c_uint,
    nblue as libc::c_uint,
  );
  // horizontal lines
  ptr1 = (*ptr_to_globals)
    .addr
    .offset((*ptr_to_globals).ns[3].wrapping_mul((*ptr_to_globals).scr_fix.line_length) as isize)
    .offset((*ptr_to_globals).ns[2].wrapping_mul((*ptr_to_globals).bytes_per_pixel) as isize);
  ptr2 = (*ptr_to_globals)
    .addr
    .offset(
      (*ptr_to_globals).ns[3]
        .wrapping_add((*ptr_to_globals).ns[1])
        .wrapping_sub(1i32 as libc::c_uint)
        .wrapping_mul((*ptr_to_globals).scr_fix.line_length) as isize,
    )
    .offset((*ptr_to_globals).ns[2].wrapping_mul((*ptr_to_globals).bytes_per_pixel) as isize);
  cnt = (*ptr_to_globals).ns[0].wrapping_sub(1i32 as libc::c_uint) as libc::c_int;
  loop {
    fb_write_pixel(ptr1, thispix);
    fb_write_pixel(ptr2, thispix);
    ptr1 = ptr1.offset((*ptr_to_globals).bytes_per_pixel as isize);
    ptr2 = ptr2.offset((*ptr_to_globals).bytes_per_pixel as isize);
    cnt -= 1;
    if !(cnt >= 0i32) {
      break;
    }
  }
  // vertical lines
  ptr1 = (*ptr_to_globals)
    .addr
    .offset((*ptr_to_globals).ns[3].wrapping_mul((*ptr_to_globals).scr_fix.line_length) as isize)
    .offset((*ptr_to_globals).ns[2].wrapping_mul((*ptr_to_globals).bytes_per_pixel) as isize);
  ptr2 = (*ptr_to_globals)
    .addr
    .offset((*ptr_to_globals).ns[3].wrapping_mul((*ptr_to_globals).scr_fix.line_length) as isize)
    .offset(
      (*ptr_to_globals).ns[2]
        .wrapping_add((*ptr_to_globals).ns[0])
        .wrapping_sub(1i32 as libc::c_uint)
        .wrapping_mul((*ptr_to_globals).bytes_per_pixel) as isize,
    );
  cnt = (*ptr_to_globals).ns[1].wrapping_sub(1i32 as libc::c_uint) as libc::c_int;
  loop {
    fb_write_pixel(ptr1, thispix);
    fb_write_pixel(ptr2, thispix);
    ptr1 = ptr1.offset((*ptr_to_globals).scr_fix.line_length as isize);
    ptr2 = ptr2.offset((*ptr_to_globals).scr_fix.line_length as isize);
    cnt -= 1;
    if !(cnt >= 0i32) {
      break;
    }
  }
}
/* *
 * Draw filled rectangle on framebuffer
 * \param nx1pos,ny1pos upper left position
 * \param nx2pos,ny2pos down right position
 * \param nred,ngreen,nblue rgb color
 */
unsafe extern "C" fn fb_drawfullrectangle(
  mut nx1pos: libc::c_int,
  mut ny1pos: libc::c_int,
  mut nx2pos: libc::c_int,
  mut ny2pos: libc::c_int,
  mut nred: libc::c_uchar,
  mut ngreen: libc::c_uchar,
  mut nblue: libc::c_uchar,
) {
  let mut cnt1: libc::c_int = 0;
  let mut cnt2: libc::c_int = 0;
  let mut nypos: libc::c_int = 0;
  let mut thispix: libc::c_uint = 0;
  let mut ptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
  thispix = fb_pixel_value(
    nred as libc::c_uint,
    ngreen as libc::c_uint,
    nblue as libc::c_uint,
  );
  cnt1 = ny2pos - ny1pos;
  nypos = ny1pos;
  loop {
    ptr = (*ptr_to_globals)
      .addr
      .offset((nypos as libc::c_uint).wrapping_mul((*ptr_to_globals).scr_fix.line_length) as isize)
      .offset((nx1pos as libc::c_uint).wrapping_mul((*ptr_to_globals).bytes_per_pixel) as isize);
    cnt2 = nx2pos - nx1pos;
    loop {
      fb_write_pixel(ptr, thispix);
      ptr = ptr.offset((*ptr_to_globals).bytes_per_pixel as isize);
      cnt2 -= 1;
      if !(cnt2 >= 0i32) {
        break;
      }
    }
    nypos += 1;
    cnt1 -= 1;
    if !(cnt1 >= 0i32) {
      break;
    }
  }
}
/* *
 * Draw a progress bar on framebuffer
 * \param percent percentage of loading
 */
unsafe extern "C" fn fb_drawprogressbar(mut percent: libc::c_uint) {
  let mut left_x: libc::c_int = 0;
  let mut top_y: libc::c_int = 0;
  let mut pos_x: libc::c_int = 0;
  let mut width: libc::c_uint = 0;
  let mut height: libc::c_uint = 0;
  // outer box
  left_x = (*ptr_to_globals).ns[2] as libc::c_int;
  top_y = (*ptr_to_globals).ns[3] as libc::c_int;
  width = (*ptr_to_globals).ns[0].wrapping_sub(1i32 as libc::c_uint);
  height = (*ptr_to_globals).ns[1].wrapping_sub(1i32 as libc::c_uint);
  if ((height | width) as libc::c_int) < 0i32 {
    return;
  }
  // NB: "width" of 1 actually makes rect with width of 2!
  fb_drawrectangle();
  // inner "empty" rectangle
  left_x += 1;
  top_y += 1;
  width = width.wrapping_sub(2i32 as libc::c_uint);
  height = height.wrapping_sub(2i32 as libc::c_uint);
  if ((height | width) as libc::c_int) < 0i32 {
    return;
  }
  pos_x = left_x;
  if percent > 0i32 as libc::c_uint {
    let mut i: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    // actual progress bar
    pos_x = (pos_x as libc::c_uint).wrapping_add(
      width
        .wrapping_mul(percent)
        .wrapping_div(100i32 as libc::c_uint),
    ) as libc::c_int as libc::c_int; // divide by 0 is bad
    y = top_y;
    i = height as libc::c_int;
    if height == 0i32 as libc::c_uint {
      height = height.wrapping_add(1)
    }
    while i >= 0i32 {
      // draw one-line thick "rectangle"
      // top line will have gray lvl 200, bottom one 100
      let mut gray_level: libc::c_uint = (100i32 as libc::c_uint).wrapping_add(
        (i as libc::c_uint)
          .wrapping_mul(100i32 as libc::c_uint)
          .wrapping_div(height),
      );
      fb_drawfullrectangle(
        left_x,
        y,
        pos_x,
        y,
        gray_level as libc::c_uchar,
        gray_level as libc::c_uchar,
        gray_level as libc::c_uchar,
      );
      y += 1;
      i -= 1
    }
  }
  fb_drawfullrectangle(
    pos_x,
    top_y,
    (left_x as libc::c_uint).wrapping_add(width) as libc::c_int,
    (top_y as libc::c_uint).wrapping_add(height) as libc::c_int,
    (*ptr_to_globals).ns[4] as libc::c_uchar,
    (*ptr_to_globals).ns[5] as libc::c_uchar,
    (*ptr_to_globals).ns[6] as libc::c_uchar,
  );
}
/* *
 * Draw image from PPM file
 */
unsafe extern "C" fn fb_drawimage() {
  let mut theme_file: *mut FILE = 0 as *mut FILE;
  let mut read_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut pixline: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
  let mut i: libc::c_uint = 0;
  let mut j: libc::c_uint = 0;
  let mut width: libc::c_uint = 0;
  let mut height: libc::c_uint = 0;
  let mut line_size: libc::c_uint = 0;
  if *(*ptr_to_globals).image_filename.offset(0) as libc::c_int == '-' as i32
    && *(*ptr_to_globals).image_filename.offset(1) == 0
  {
    theme_file = stdin
  } else {
    let mut fd: libc::c_int = open_zipped((*ptr_to_globals).image_filename, 0i32);
    if fd < 0i32 {
      bb_simple_perror_msg_and_die((*ptr_to_globals).image_filename);
    }
    theme_file = xfdopen_for_read(fd)
  }
  /* Parse ppm header:
   * - Magic: two characters "P6".
   * - Whitespace (blanks, TABs, CRs, LFs).
   * - A width, formatted as ASCII characters in decimal.
   * - Whitespace.
   * - A height, ASCII decimal.
   * - Whitespace.
   * - The maximum color value, ASCII decimal, in 0..65535
   * - Newline or other single whitespace character.
   *   (we support newline only)
   * - A raster of Width * Height pixels in triplets of rgb
   *   in pure binary by 1 or 2 bytes. (we support only 1 byte)
   */
  read_ptr = bb_common_bufsiz1.as_mut_ptr(); /* ignore #comments */
  loop {
    let mut w: libc::c_int = 0; /* w is on stack, width may be in register */
    let mut h: libc::c_int = 0;
    let mut max_color_val: libc::c_int = 0;
    let mut rem: libc::c_int = bb_common_bufsiz1
      .as_mut_ptr()
      .offset(COMMON_BUFSIZE as libc::c_int as isize)
      .wrapping_offset_from(read_ptr) as libc::c_long as libc::c_int;
    if rem < 2i32 || fgets_unlocked(read_ptr, rem, theme_file).is_null() {
      bb_error_msg_and_die(
        b"bad PPM file \'%s\'\x00" as *const u8 as *const libc::c_char,
        (*ptr_to_globals).image_filename,
      );
    }
    read_ptr = strchrnul(read_ptr, '#' as i32);
    *read_ptr = '\u{0}' as i32 as libc::c_char;
    if !(sscanf(
      bb_common_bufsiz1.as_mut_ptr(),
      b"P6 %u %u %u\x00" as *const u8 as *const libc::c_char,
      &mut w as *mut libc::c_int,
      &mut h as *mut libc::c_int,
      &mut max_color_val as *mut libc::c_int,
    ) == 3i32
      && max_color_val <= 255i32)
    {
      continue;
    }
    width = w as libc::c_uint;
    height = h as libc::c_uint;
    break;
  }
  line_size = width.wrapping_mul(3i32 as libc::c_uint);
  pixline = xmalloc(line_size as size_t) as *mut libc::c_uchar;
  if width.wrapping_add((*ptr_to_globals).ns[7]) > (*ptr_to_globals).scr_var.xres {
    width = (*ptr_to_globals)
      .scr_var
      .xres
      .wrapping_sub((*ptr_to_globals).ns[7])
  }
  if height.wrapping_add((*ptr_to_globals).ns[8]) > (*ptr_to_globals).scr_var.yres {
    height = (*ptr_to_globals)
      .scr_var
      .yres
      .wrapping_sub((*ptr_to_globals).ns[8])
  }
  j = 0i32 as libc::c_uint;
  while j < height {
    let mut pixel: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut src: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if fread(
      pixline as *mut libc::c_void,
      1i32 as size_t,
      line_size as size_t,
      theme_file,
    ) != line_size as libc::c_ulong
    {
      bb_error_msg_and_die(
        b"bad PPM file \'%s\'\x00" as *const u8 as *const libc::c_char,
        (*ptr_to_globals).image_filename,
      );
    }
    pixel = pixline;
    src = (*ptr_to_globals)
      .addr
      .offset(
        (*ptr_to_globals).ns[8]
          .wrapping_add(j)
          .wrapping_mul((*ptr_to_globals).scr_fix.line_length) as isize,
      )
      .offset((*ptr_to_globals).ns[7].wrapping_mul((*ptr_to_globals).bytes_per_pixel) as isize);
    i = 0i32 as libc::c_uint;
    while i < width {
      let mut thispix: libc::c_uint = fb_pixel_value(
        *pixel.offset(0) as libc::c_uint,
        *pixel.offset(1) as libc::c_uint,
        *pixel.offset(2) as libc::c_uint,
      );
      fb_write_pixel(src, thispix);
      src = src.offset((*ptr_to_globals).bytes_per_pixel as isize);
      pixel = pixel.offset(3);
      i = i.wrapping_add(1)
    }
    j = j.wrapping_add(1)
  }
  free(pixline as *mut libc::c_void);
  fclose(theme_file);
}
/* *
 * Parse configuration file
 * \param *cfg_filename name of the configuration file
 */
unsafe extern "C" fn init(mut cfg_filename: *const libc::c_char) {
  static mut param_names: [libc::c_char; 74] = [
    66, 65, 82, 95, 87, 73, 68, 84, 72, 0, 66, 65, 82, 95, 72, 69, 73, 71, 72, 84, 0, 66, 65, 82,
    95, 76, 69, 70, 84, 0, 66, 65, 82, 95, 84, 79, 80, 0, 66, 65, 82, 95, 82, 0, 66, 65, 82, 95,
    71, 0, 66, 65, 82, 95, 66, 0, 73, 77, 71, 95, 76, 69, 70, 84, 0, 73, 77, 71, 95, 84, 79, 80, 0,
    0,
  ]; // for compiler
  let mut token: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2];
  let mut parser: *mut parser_t = config_open2(
    cfg_filename,
    Some(xfopen_stdin as unsafe extern "C" fn(_: *const libc::c_char) -> *mut FILE),
  );
  while config_read(
    parser,
    token.as_mut_ptr(),
    ((PARSE_NORMAL as libc::c_int | PARSE_MIN_DIE as libc::c_int)
      & !(PARSE_TRIM as libc::c_int | PARSE_COLLAPSE as libc::c_int)
      | (2i32 & 0xffi32) << 8i32
      | 2i32 & 0xffi32) as libc::c_uint,
    b"#=\x00" as *const u8 as *const libc::c_char,
  ) != 0
  {
    let mut val: libc::c_uint = xatoi_positive(token[1]) as libc::c_uint;
    let mut i: libc::c_int = index_in_strings(param_names.as_ptr(), token[0]);
    if i < 0i32 {
      bb_error_msg_and_die(
        b"syntax error: %s\x00" as *const u8 as *const libc::c_char,
        token[0],
      );
    }
    if i >= 0i32 && i < 9i32 {
      (*ptr_to_globals).ns[i as usize] = val
    }
  }
  config_close(parser);
}
#[no_mangle]
pub unsafe extern "C" fn fbsplash_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut fb_device: *const libc::c_char = 0 as *const libc::c_char;
  let mut cfg_filename: *const libc::c_char = 0 as *const libc::c_char;
  let mut fifo_filename: *const libc::c_char = 0 as *const libc::c_char;
  let mut fp: *mut FILE = 0 as *mut FILE;
  fp = fp;
  let mut num_buf: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut num: libc::c_uint = 0;
  let mut bCursorOff: bool = false;
  let ref mut fresh0 = *(not_const_pp(&ptr_to_globals as *const *mut globals as *const libc::c_void)
    as *mut *mut globals);
  *fresh0 = xzalloc(::std::mem::size_of::<globals>() as libc::c_ulong) as *mut globals;
  asm!("" : : : "memory" : "volatile");
  // parse command line options
  fb_device = b"/dev/fb0\x00" as *const u8 as *const libc::c_char;
  cfg_filename = 0 as *const libc::c_char;
  fifo_filename = 0 as *const libc::c_char;
  bCursorOff = 1i32 as libc::c_uint
    & getopt32(
      argv,
      b"cs:d:i:f:\x00" as *const u8 as *const libc::c_char,
      &mut (*ptr_to_globals).image_filename as *mut *const libc::c_char,
      &mut fb_device as *mut *const libc::c_char,
      &mut cfg_filename as *mut *const libc::c_char,
      &mut fifo_filename as *mut *const libc::c_char,
    )
    != 0;
  // parse configuration file
  if !cfg_filename.is_null() {
    init(cfg_filename);
  }
  // We must have -s IMG
  if (*ptr_to_globals).image_filename.is_null() {
    bb_show_usage();
  }
  fb_open(fb_device);
  if !fifo_filename.is_null() && bCursorOff as libc::c_int != 0 {
    // hide cursor (BEFORE any fb ops)
    full_write(
      1i32,
      b"\x1b[?25l\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
      6i32 as size_t,
    );
  }
  fb_drawimage();
  if fifo_filename.is_null() {
    return 0i32;
  }
  fp = xfopen_stdin(fifo_filename);
  if fp != stdin {
    // For named pipes, we want to support this:
    //  mkfifo cmd_pipe
    //  fbsplash -f cmd_pipe .... &
    //  ...
    //  echo 33 >cmd_pipe
    //  ...
    //  echo 66 >cmd_pipe
    // This means that we don't want fbsplash to get EOF
    // when last writer closes input end.
    // The simplest way is to open fifo for writing too
    // and become an additional writer :)
    open(fifo_filename, 0o1i32);
    // errors are ignored
  }
  fb_drawprogressbar(0i32 as libc::c_uint);
  loop
  // Block on read, waiting for some input.
  // Use of <stdio.h> style I/O allows to correctly
  // handle a case when we have many buffered lines
  // already in the pipe
  {
    num_buf = xmalloc_fgetline(fp);
    if num_buf.is_null() {
      break;
    }
    if !is_prefixed_with(num_buf, b"exit\x00" as *const u8 as *const libc::c_char).is_null() {
      break;
    }
    num = atoi(num_buf) as libc::c_uint;
    if (*num_buf.offset(0) as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32
      && num <= 100i32 as libc::c_uint
    {
      fb_drawprogressbar(num);
    }
    free(num_buf as *mut libc::c_void);
  }
  if bCursorOff {
    // restore cursor
    full_write(
      1i32,
      b"\x1b[?25h\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
      6i32 as size_t,
    );
  }
  return 0i32;
}
