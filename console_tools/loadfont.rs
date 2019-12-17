use crate::libbb::xfuncs_printf::xmalloc;
use libc;
use libc::chdir;
use libc::free;
extern "C" {

  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn get_console_fd_or_die() -> libc::c_int;
  #[no_mangle]
  fn xopen_nonblocking(pathname: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn xmalloc_read(fd: libc::c_int, maxsz_p: *mut size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xmalloc_open_zipped_read_close(
    fname: *const libc::c_char,
    maxsz_p: *mut size_t,
  ) -> *mut libc::c_void;
  #[no_mangle]
  fn xstrtoull(str: *const libc::c_char, b: libc::c_int) -> libc::c_ulonglong;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
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
  fn config_close(parser: *mut parser_t);
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
pub struct unipair {
  pub unicode: libc::c_ushort,
  pub fontpos: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unimapdesc {
  pub entry_ct: libc::c_ushort,
  pub entries: *mut unipair,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unimapinit {
  pub advised_hashsize: libc::c_ushort,
  pub advised_hashstep: libc::c_ushort,
  pub advised_hashlevel: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct console_font_op {
  pub op: libc::c_uint,
  pub flags: libc::c_uint,
  pub width: libc::c_uint,
  pub height: libc::c_uint,
  pub charcount: libc::c_uint,
  pub data: *mut libc::c_uchar,
}

/*
 * loadfont.c - Eugene Crosser & Andries Brouwer
 *
 * Version 0.96bb
 *
 * Loads the console font, and possibly the corresponding screen map(s).
 * (Adapted for busybox by Matej Vela.)
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
//config:config LOADFONT
//config:	bool "loadfont (5.2 kb)"
//config:	default y
//config:	select PLATFORM_LINUX
//config:	help
//config:	This program loads a console font from standard input.
//config:
//config:config SETFONT
//config:	bool "setfont (24 kb)"
//config:	default y
//config:	select PLATFORM_LINUX
//config:	help
//config:	Allows to load console screen map. Useful for i18n.
//config:
//config:config FEATURE_SETFONT_TEXTUAL_MAP
//config:	bool "Support reading textual screen maps"
//config:	default y
//config:	depends on SETFONT
//config:	help
//config:	Support reading textual screen maps.
//config:
//config:config DEFAULT_SETFONT_DIR
//config:	string "Default directory for console-tools files"
//config:	default ""
//config:	depends on SETFONT
//config:	help
//config:	Directory to use if setfont's params are simple filenames
//config:	(not /path/to/file or ./file). Default is "" (no default directory).
//config:
//config:comment "Common options for loadfont and setfont"
//config:	depends on LOADFONT || SETFONT
//config:
//config:config FEATURE_LOADFONT_PSF2
//config:	bool "Support PSF2 console fonts"
//config:	default y
//config:	depends on LOADFONT || SETFONT
//config:
//config:config FEATURE_LOADFONT_RAW
//config:	bool "Support old (raw) console fonts"
//config:	default y
//config:	depends on LOADFONT || SETFONT
//applet:IF_LOADFONT(APPLET_NOEXEC(loadfont, loadfont, BB_DIR_USR_SBIN, SUID_DROP, loadfont))
//applet:IF_SETFONT(APPLET_NOEXEC(setfont, setfont, BB_DIR_USR_SBIN, SUID_DROP, setfont))
//kbuild:lib-$(CONFIG_LOADFONT) += loadfont.o
//kbuild:lib-$(CONFIG_SETFONT) += loadfont.o
/* KDFONTOP */
pub type C2RustUnnamed_0 = libc::c_uint;
pub const PSF1_SEPARATOR: C2RustUnnamed_0 = 65535;
pub const PSF1_STARTSEQ: C2RustUnnamed_0 = 65534;
pub const PSF1_MAXMODE: C2RustUnnamed_0 = 5;
pub const PSF1_MODEHASSEQ: C2RustUnnamed_0 = 4;
pub const PSF1_MODEHASTAB: C2RustUnnamed_0 = 2;
pub const PSF1_MODE512: C2RustUnnamed_0 = 1;
pub const PSF1_MAGIC1: C2RustUnnamed_0 = 4;
pub const PSF1_MAGIC0: C2RustUnnamed_0 = 54;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct psf1_header {
  pub magic: [libc::c_uchar; 2],
  pub mode: libc::c_uchar,
  pub charsize: libc::c_uchar,
  /* Character size */
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub const PSF2_SEPARATOR: C2RustUnnamed_1 = 255;
pub const PSF2_STARTSEQ: C2RustUnnamed_1 = 254;
pub const PSF2_MAXVERSION: C2RustUnnamed_1 = 0;
pub const PSF2_HAS_UNICODE_TABLE: C2RustUnnamed_1 = 1;
pub const PSF2_MAGIC3: C2RustUnnamed_1 = 134;
pub const PSF2_MAGIC2: C2RustUnnamed_1 = 74;
pub const PSF2_MAGIC1: C2RustUnnamed_1 = 181;
pub const PSF2_MAGIC0: C2RustUnnamed_1 = 114;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct psf2_header {
  pub magic: [libc::c_uchar; 4],
  pub version: libc::c_uint,
  pub headersize: libc::c_uint,
  pub flags: libc::c_uint,
  pub length: libc::c_uint,
  pub charsize: libc::c_uint,
  pub height: libc::c_uint,
  pub width: libc::c_uint,
  /* charsize = height * ((width + 7) / 8) */
}
#[inline(always)]
unsafe extern "C" fn xstrtoul(mut str: *const libc::c_char, mut b: libc::c_int) -> libc::c_ulong {
  return xstrtoull(str, b) as libc::c_ulong;
}
/* ENABLE_FEATURE_LOADFONT_PSF2 */
unsafe extern "C" fn do_loadfont(
  mut fd: libc::c_int,
  mut inbuf: *mut libc::c_uchar,
  mut height: libc::c_int,
  mut width: libc::c_int,
  mut charsize: libc::c_int,
  mut fontsize: libc::c_int,
) {
  let mut buf: *mut libc::c_uchar = std::ptr::null_mut();
  let mut charwidth: libc::c_int = 32i32 * ((width + 7i32) / 8i32);
  let mut i: libc::c_int = 0;
  if height < 1i32 || height > 32i32 || width < 1i32 || width > 32i32 {
    bb_error_msg_and_die(
      b"bad character size %dx%d\x00" as *const u8 as *const libc::c_char,
      height,
      width,
    );
  }
  buf = xzalloc((charwidth * (if fontsize < 128i32 { 128i32 } else { fontsize })) as size_t)
    as *mut libc::c_uchar;
  i = 0;
  while i < fontsize {
    memcpy(
      buf.offset((i * charwidth) as isize) as *mut libc::c_void,
      inbuf.offset((i * charsize) as isize) as *const libc::c_void,
      charsize as libc::c_ulong,
    );
    i += 1
  }
  /* KDFONTOP */
  let mut cfo: console_font_op = console_font_op {
    op: 0,
    flags: 0,
    width: 0,
    height: 0,
    charcount: 0,
    data: 0 as *mut libc::c_uchar,
  };
  cfo.op = 0 as libc::c_uint;
  cfo.flags = 0 as libc::c_uint;
  cfo.width = width as libc::c_uint;
  cfo.height = height as libc::c_uint;
  cfo.charcount = fontsize as libc::c_uint;
  cfo.data = buf;
  bb_xioctl(
    fd,
    0x4b72i32 as libc::c_uint,
    &mut cfo as *mut console_font_op as *mut libc::c_void,
    b"KDFONTOP\x00" as *const u8 as *const libc::c_char,
  );
  free(buf as *mut libc::c_void);
}
/*
 * Format of the Unicode information:
 *
 * For each font position <uc>*<seq>*<term>
 * where <uc> is a 2-byte little endian Unicode value (PSF1)
 * or an UTF-8 coded value (PSF2),
 * <seq> = <ss><uc><uc>*, <ss> = psf1 ? 0xFFFE : 0xFE,
 * <term> = psf1 ? 0xFFFF : 0xFF.
 * and * denotes zero or more occurrences of the preceding item.
 *
 * Semantics:
 * The leading <uc>* part gives Unicode symbols that are all
 * represented by this font position. The following sequences
 * are sequences of Unicode symbols - probably a symbol
 * together with combining accents - also represented by
 * this font position.
 *
 * Example:
 * At the font position for a capital A-ring glyph, we
 * may have:
 *   00C5,212B,FFFE,0041,030A,FFFF
 * Some font positions may be described by sequences only,
 * namely when there is no precomposed Unicode value for the glyph.
 */
unsafe extern "C" fn do_loadtable(
  mut fd: libc::c_int,
  mut inbuf: *mut libc::c_uchar,
  mut tailsz: libc::c_int,
  mut fontsize: libc::c_int,
  mut psf2: libc::c_int,
) {
  let mut advice: unimapinit = unimapinit {
    advised_hashsize: 0,
    advised_hashstep: 0,
    advised_hashlevel: 0,
  }; /* more than enough */
  let mut ud: unimapdesc = unimapdesc {
    entry_ct: 0,
    entries: 0 as *mut unipair,
  }; /* PSF2 */
  let mut up: *mut unipair = std::ptr::null_mut();
  let mut ct: libc::c_int = 0;
  let mut maxct: libc::c_int = 0;
  let mut glyph: libc::c_int = 0;
  let mut unicode: u16 = 0;
  maxct = tailsz;
  up = xmalloc(
    (maxct as libc::c_ulong).wrapping_mul(::std::mem::size_of::<unipair>() as libc::c_ulong),
  ) as *mut unipair;
  glyph = 0;
  while glyph < fontsize {
    while tailsz > 0 {
      if psf2 == 0 {
        /* PSF1 */
        unicode = (((*inbuf.offset(1) as u16 as libc::c_int) << 8i32)
          + *inbuf.offset(0) as libc::c_int) as u16;
        tailsz -= 2i32;
        inbuf = inbuf.offset(2);
        if unicode as libc::c_int == PSF1_SEPARATOR as libc::c_int {
          break;
        }
      } else {
        tailsz -= 1;
        let fresh0 = inbuf;
        inbuf = inbuf.offset(1);
        unicode = *fresh0 as u16;
        if unicode as libc::c_int == PSF2_SEPARATOR as libc::c_int {
          break;
        }
        if unicode as libc::c_int == PSF2_STARTSEQ as libc::c_int {
          bb_simple_error_msg_and_die(
            b"unicode sequences not implemented\x00" as *const u8 as *const libc::c_char,
          );
        } else {
          if unicode as libc::c_int >= 0xc0i32 {
            if unicode as libc::c_int >= 0xfci32 {
              unicode = (unicode as libc::c_int & 0x1i32) as u16;
              maxct = 5i32
            } else if unicode as libc::c_int >= 0xf8i32 {
              unicode = (unicode as libc::c_int & 0x3i32) as u16;
              maxct = 4i32
            } else if unicode as libc::c_int >= 0xf0i32 {
              unicode = (unicode as libc::c_int & 0x7i32) as u16;
              maxct = 3i32
            } else if unicode as libc::c_int >= 0xe0i32 {
              unicode = (unicode as libc::c_int & 0xfi32) as u16;
              maxct = 2i32
            } else {
              unicode = (unicode as libc::c_int & 0x1fi32) as u16;
              maxct = 1i32
            }
            loop {
              if tailsz <= 0 || (*inbuf as libc::c_int) < 0x80i32 || *inbuf as libc::c_int > 0xbfi32
              {
                bb_simple_error_msg_and_die(
                  b"illegal UTF-8 character\x00" as *const u8 as *const libc::c_char,
                );
              }
              tailsz -= 1;
              let fresh1 = inbuf;
              inbuf = inbuf.offset(1);
              unicode =
                (((unicode as libc::c_int) << 6i32) + (*fresh1 as libc::c_int & 0x3fi32)) as u16;
              maxct -= 1;
              if !(maxct > 0) {
                break;
              }
            }
          } else if unicode as libc::c_int >= 0x80i32 {
            bb_simple_error_msg_and_die(
              b"illegal UTF-8 character\x00" as *const u8 as *const libc::c_char,
            );
          }
        }
      }
      (*up.offset(ct as isize)).unicode = unicode;
      (*up.offset(ct as isize)).fontpos = glyph as libc::c_ushort;
      ct += 1
    }
    glyph += 1
  }
  /* Note: after PIO_UNIMAPCLR and before PIO_UNIMAP
   * this printf did not work on many kernels */
  advice.advised_hashsize = 0 as libc::c_ushort;
  advice.advised_hashstep = 0 as libc::c_ushort;
  advice.advised_hashlevel = 0 as libc::c_ushort;
  bb_xioctl(
    fd,
    0x4b68i32 as libc::c_uint,
    &mut advice as *mut unimapinit as *mut libc::c_void,
    b"PIO_UNIMAPCLR\x00" as *const u8 as *const libc::c_char,
  );
  ud.entry_ct = ct as libc::c_ushort;
  ud.entries = up;
  bb_xioctl(
    fd,
    0x4b67i32 as libc::c_uint,
    &mut ud as *mut unimapdesc as *mut libc::c_void,
    b"PIO_UNIMAP\x00" as *const u8 as *const libc::c_char,
  );
}
unsafe extern "C" fn do_load(mut fd: libc::c_int, mut buffer: *mut libc::c_uchar, mut len: size_t) {
  let mut height: libc::c_int = 0;
  let mut width: libc::c_int = 8i32;
  let mut charsize: libc::c_int = 0;
  let mut fontsize: libc::c_int = 256i32;
  let mut has_table: libc::c_int = 0;
  let mut font: *mut libc::c_uchar = buffer;
  let mut table: *mut libc::c_uchar = std::ptr::null_mut();
  if len >= ::std::mem::size_of::<psf1_header>() as libc::c_ulong
    && ((*(buffer as *mut psf1_header)).magic[0] as libc::c_int == PSF1_MAGIC0 as libc::c_int
      && (*(buffer as *mut psf1_header)).magic[1] as libc::c_int == PSF1_MAGIC1 as libc::c_int)
  {
    if (*(buffer as *mut psf1_header)).mode as libc::c_int > PSF1_MAXMODE as libc::c_int {
      bb_simple_error_msg_and_die(
        b"unsupported psf file mode\x00" as *const u8 as *const libc::c_char,
      );
    }
    if (*(buffer as *mut psf1_header)).mode as libc::c_int & PSF1_MODE512 as libc::c_int != 0 {
      fontsize = 512i32
    }
    if (*(buffer as *mut psf1_header)).mode as libc::c_int & PSF1_MODEHASTAB as libc::c_int != 0 {
      has_table = 1i32
    }
    charsize = (*(buffer as *mut psf1_header)).charsize as libc::c_int;
    height = charsize;
    font = font.offset(::std::mem::size_of::<psf1_header>() as libc::c_ulong as isize)
  } else if len >= ::std::mem::size_of::<psf2_header>() as libc::c_ulong
    && ((*(buffer as *mut psf2_header)).magic[0] as libc::c_int == PSF2_MAGIC0 as libc::c_int
      && (*(buffer as *mut psf2_header)).magic[1] as libc::c_int == PSF2_MAGIC1 as libc::c_int
      && (*(buffer as *mut psf2_header)).magic[2] as libc::c_int == PSF2_MAGIC2 as libc::c_int
      && (*(buffer as *mut psf2_header)).magic[3] as libc::c_int == PSF2_MAGIC3 as libc::c_int)
  {
    if (*(buffer as *mut psf2_header)).version > PSF2_MAXVERSION as libc::c_int as libc::c_uint {
      bb_simple_error_msg_and_die(
        b"unsupported psf file version\x00" as *const u8 as *const libc::c_char,
      );
    }
    fontsize = (*(buffer as *mut psf2_header)).length as libc::c_int;
    if (*(buffer as *mut psf2_header)).flags & PSF2_HAS_UNICODE_TABLE as libc::c_int as libc::c_uint
      != 0
    {
      has_table = 2i32
    }
    charsize = (*(buffer as *mut psf2_header)).charsize as libc::c_int;
    height = (*(buffer as *mut psf2_header)).height as libc::c_int;
    width = (*(buffer as *mut psf2_header)).width as libc::c_int;
    font = font.offset((*(buffer as *mut psf2_header)).headersize as isize)
  } else if len == 9780i32 as libc::c_ulong {
    /* file with three code pages? */
    height = 16i32;
    charsize = height;
    font = font.offset(40)
  } else if len & 0o377i32 as libc::c_ulong == 0 as libc::c_ulong {
    /* bare font */
    height = len.wrapping_div(256i32 as libc::c_ulong) as libc::c_int;
    charsize = height
  } else {
    bb_simple_error_msg_and_die(
      b"input file: bad length or unsupported font type\x00" as *const u8 as *const libc::c_char,
    );
  }
  table = font.offset((fontsize * charsize) as isize);
  buffer = buffer.offset(len as isize);
  if table > buffer || has_table == 0 && table != buffer {
    bb_simple_error_msg_and_die(b"input file: bad length\x00" as *const u8 as *const libc::c_char);
  }
  do_loadfont(fd, font, height, width, charsize, fontsize);
  if has_table != 0 {
    do_loadtable(
      fd,
      table,
      buffer.wrapping_offset_from(table) as libc::c_long as libc::c_int,
      fontsize,
      has_table - 1i32,
    );
  };
}
//usage:#define loadfont_trivial_usage
//usage:       "< font"
//usage:#define loadfont_full_usage "\n\n"
//usage:       "Load a console font from stdin"
/* //usage:     "\n	-C TTY	Affect TTY instead of /dev/tty" */
//usage:
//usage:#define loadfont_example_usage
//usage:       "$ loadfont < /etc/i18n/fontname\n"
#[no_mangle]
pub unsafe extern "C" fn loadfont_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut len: size_t = 0;
  let mut buffer: *mut libc::c_uchar = std::ptr::null_mut();
  // no arguments allowed!
  getopt32(argv, b"^\x00=0\x00" as *const u8 as *const libc::c_char);
  /*
   * We used to look at the length of the input file
   * with stat(); now that we accept compressed files,
   * just read the entire file.
   * Len was 32k, but latarcyrheb-sun32.psfu is 34377 bytes
   * (it has largish Unicode map).
   */
  len = (128i32 * 1024i32) as size_t;
  buffer = xmalloc_read(0i32, &mut len) as *mut libc::c_uchar;
  // xmalloc_open_zipped_read_close(filename, &len);
  if buffer.is_null() {
    bb_simple_perror_msg_and_die(
      b"error reading input font\x00" as *const u8 as *const libc::c_char,
    );
  }
  do_load(get_console_fd_or_die(), buffer, len);
  return 0;
}
/* kbd-1.12:
setfont [-O font+umap.orig] [-o font.orig] [-om cmap.orig]
[-ou umap.orig] [-N] [font.new ...] [-m cmap] [-u umap] [-C console]
[-hNN] [-v] [-V]

-h NN  Override font height
-o file
       Save previous font in file
-O file
       Save previous font and Unicode map in file
-om file
       Store console map in file
-ou file
       Save previous Unicode map in file
-m file
       Load console map or Unicode console map from file
-u file
       Load Unicode table describing the font from file
       Example:
       # cp866
       0x00-0x7f       idem
       #
       0x80    U+0410  # CYRILLIC CAPITAL LETTER A
       0x81    U+0411  # CYRILLIC CAPITAL LETTER BE
       0x82    U+0412  # CYRILLIC CAPITAL LETTER VE
-C console
       Set the font for the indicated console
-v     Verbose
-V     Version
*/
//usage:#define setfont_trivial_usage
//usage:       "FONT [-m MAPFILE] [-C TTY]"
//usage:#define setfont_full_usage "\n\n"
//usage:       "Load a console font\n"
//usage:     "\n	-m MAPFILE	Load console screen map"
//usage:     "\n	-C TTY		Affect TTY instead of /dev/tty"
//usage:
//usage:#define setfont_example_usage
//usage:       "$ setfont -m koi8-r /etc/i18n/fontname\n"
unsafe extern "C" fn ctoi(mut s: *mut libc::c_char) -> libc::c_int {
  if *s.offset(0) as libc::c_int == '\'' as i32
    && *s.offset(1) as libc::c_int != '\u{0}' as i32
    && *s.offset(2) as libc::c_int == '\'' as i32
    && *s.offset(3) as libc::c_int == '\u{0}' as i32
  {
    return *s.offset(1) as libc::c_int;
  }
  // U+ means 0x
  if *s.offset(0) as libc::c_int == 'U' as i32 && *s.offset(1) as libc::c_int == '+' as i32 {
    *s.offset(0) = '0' as i32 as libc::c_char;
    *s.offset(1) = 'x' as i32 as libc::c_char
  }
  if !((*s.offset(0) as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32) {
    return -1i32;
  }
  return xstrtoul(s, 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn setfont_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut len: size_t = 0;
  let mut opts: libc::c_uint = 0;
  let mut fd: libc::c_int = 0;
  let mut buffer: *mut libc::c_uchar = std::ptr::null_mut();
  let mut mapfilename: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut tty_name: *const libc::c_char = b"/dev/tty\x00" as *const u8 as *const libc::c_char;
  opts = getopt32(
    argv,
    b"^m:C:\x00=1\x00" as *const u8 as *const libc::c_char,
    &mut mapfilename as *mut *mut libc::c_char,
    &mut tty_name as *mut *const libc::c_char,
  );
  argv = argv.offset(optind as isize);
  fd = xopen_nonblocking(tty_name);
  if ::std::mem::size_of::<[libc::c_char; 1]>() as libc::c_ulong > 1i32 as libc::c_ulong {
    // if not ""
    if **argv.offset(0) as libc::c_int != '/' as i32 {
      // goto default fonts location. don't die if doesn't exist
      chdir(b"/consolefonts\x00" as *const u8 as *const libc::c_char);
    }
  }
  // load font
  len = (128i32 * 1024i32) as size_t;
  buffer = xmalloc_open_zipped_read_close(*argv, &mut len) as *mut libc::c_uchar;
  if buffer.is_null() {
    bb_simple_perror_msg_and_die(*argv);
  }
  do_load(fd, buffer, len);
  // load the screen map, if any
  if opts & 1i32 as libc::c_uint != 0 {
    // -m
    let mut mode: libc::c_uint = 0x4b41i32 as libc::c_uint;
    let mut map: *mut libc::c_void = std::ptr::null_mut();
    if ::std::mem::size_of::<[libc::c_char; 1]>() as libc::c_ulong > 1i32 as libc::c_ulong {
      // if not ""
      if *mapfilename.offset(0) as libc::c_int != '/' as i32 {
        // goto default keymaps location
        chdir(b"/consoletrans\x00" as *const u8 as *const libc::c_char);
      }
    }
    // fetch keymap
    map = xmalloc_open_zipped_read_close(mapfilename, &mut len);
    if map.is_null() {
      bb_simple_perror_msg_and_die(mapfilename);
    }
    // file size is 256 or 512 bytes? -> assume binary map
    if len == 256i32 as libc::c_ulong || len == (2i32 * 256i32) as libc::c_ulong {
      if len == (2i32 * 256i32) as libc::c_ulong {
        mode = 0x4b6ai32 as libc::c_uint
      }
    } else {
      // assume textual Unicode console maps:
      // 0x00 U+0000  #  NULL (NUL)
      // 0x01 U+0001  #  START OF HEADING (SOH)
      // 0x02 U+0002  #  START OF TEXT (STX)
      // 0x03 U+0003  #  END OF TEXT (ETX)
      let mut i: libc::c_int = 0;
      let mut token: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2];
      let mut parser: *mut parser_t = std::ptr::null_mut();
      map = xmalloc(
        (256i32 as libc::c_ulong)
          .wrapping_mul(::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong),
      );
      // fill vanilla map
      i = 0;
      while i < 256i32 {
        *(map as *mut libc::c_ushort).offset(i as isize) = (0xf000i32 + i) as libc::c_ushort;
        i += 1
      }
      parser = config_open(mapfilename);
      while config_read(
        parser,
        token.as_mut_ptr(),
        (PARSE_NORMAL as libc::c_int
          | PARSE_MIN_DIE as libc::c_int
          | (2i32 & 0xffi32) << 8i32
          | 2i32 & 0xffi32) as libc::c_uint,
        b"# \t\x00" as *const u8 as *const libc::c_char,
      ) != 0
      {
        // parse code/value pair
        let mut a: libc::c_int = ctoi(token[0]);
        let mut b: libc::c_int = ctoi(token[1]);
        if a < 0 || a >= 256i32 || b < 0 || b > 65535i32 {
          bb_simple_error_msg_and_die(b"map format\x00" as *const u8 as *const libc::c_char);
        }
        // patch map
        *(map as *mut libc::c_ushort).offset(a as isize) = b as libc::c_ushort;
        // unicode character is met?
        if b > 255i32 {
          mode = 0x4b6ai32 as libc::c_uint
        }
      }
      if mode != 0x4b6ai32 as libc::c_uint {
        i = 0;
        while i < 256i32 {
          *(map as *mut libc::c_uchar).offset(i as isize) =
            *(map as *mut libc::c_ushort).offset(i as isize) as libc::c_uchar;
          i += 1
        }
      }
    }
    // ENABLE_FEATURE_SETFONT_TEXTUAL_MAP
    // do set screen map
    bb_xioctl(
      fd,
      mode,
      map,
      b"mode\x00" as *const u8 as *const libc::c_char,
    );
  }
  return 0;
}
