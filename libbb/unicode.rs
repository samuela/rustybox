

use libc;
extern "C" {
  #[no_mangle]
  fn strchrnul(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn xmalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xrealloc(old: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xstrndup(s: *const libc::c_char, n: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn printable_string2(stats: *mut uni_stat_t, str: *const libc::c_char) -> *const libc::c_char;
}

use crate::librb::size_t;
pub type wchar_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uni_stat_t {
  pub byte_count: libc::c_uint,
  pub unicode_count: libc::c_uint,
  pub unicode_width: libc::c_uint,
}

/*
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
pub type C2RustUnnamed = libc::c_uint;
pub const UNICODE_ON: C2RustUnnamed = 2;
pub const UNICODE_OFF: C2RustUnnamed = 1;
pub const UNICODE_UNKNOWN: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const UNI_FLAG_PAD: C2RustUnnamed_0 = 1;
pub type bb_wint_t = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bb_mbstate_t {
  pub bogus: libc::c_char,
}
#[inline(always)]
unsafe extern "C" fn bb_ascii_isalnum(mut a: libc::c_uchar) -> libc::c_int {
  let mut b: libc::c_uchar = (a as libc::c_int - '0' as i32) as libc::c_uchar;
  if b as libc::c_int <= 9i32 {
    return (b as libc::c_int <= 9i32) as libc::c_int;
  }
  b = ((a as libc::c_int | 0x20i32) - 'a' as i32) as libc::c_uchar;
  return (b as libc::c_int <= 'z' as i32 - 'a' as i32) as libc::c_int;
}

/*
 * Unicode support routines.
 *
 * Copyright (C) 2009 Denys Vlasenko
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
/* If it's not #defined as a constant in unicode.h... */
/* This file is compiled only if UNICODE_SUPPORT is on.
 * We check other options and decide whether to use libc support
 * via locale, or use our own logic:
 */
/* Homegrown Unicode support. It knows only C and Unicode locales. */
unsafe extern "C" fn wcrtomb_internal(mut s: *mut libc::c_char, mut wc: wchar_t) -> size_t {
  let mut n: libc::c_int = 0;
  let mut i: libc::c_int = 0;
  let mut v: u32 = wc as u32;
  if v <= 0x7fi32 as libc::c_uint {
    *s = v as libc::c_char;
    return 1i32 as size_t;
  }
  /* RFC 3629 says that Unicode ends at 10FFFF,
   * but we cover entire 32 bits */
  /* 4000000-FFFFFFFF -> 111111tt 10tttttt 10zzzzzz 10zzyyyy 10yyyyxx 10xxxxxx */
  /* 200000-3FFFFFF -> 111110tt 10zzzzzz 10zzyyyy 10yyyyxx 10xxxxxx */
  /* 10000-1FFFFF -> 11110zzz 10zzyyyy 10yyyyxx 10xxxxxx */
  /* 800-FFFF -> 1110yyyy 10yyyyxx 10xxxxxx */
  /* 80-7FF -> 110yyyxx 10xxxxxx */
  /* How many bytes do we need? */
  n = 2i32;
  /* (0x80000000+ would result in n = 7, limiting n to 6) */
  while v >= 0x800i32 as libc::c_uint && n < 6i32 {
    v >>= 5i32;
    n += 1
  }
  /* Fill bytes n-1..1 */
  i = n;
  loop {
    i -= 1;
    if !(i != 0) {
      break;
    }
    *s.offset(i as isize) = (wc & 0x3fi32 | 0x80i32) as libc::c_char;
    wc >>= 6i32
  }
  /* Fill byte 0 */
  *s.offset(0) = (wc | (0x3f00i32 >> n) as u8 as libc::c_int) as libc::c_char;
  return n as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn bb_wcrtomb(
  mut s: *mut libc::c_char,
  mut wc: wchar_t,
  mut _ps: *mut bb_mbstate_t,
) -> size_t {
  if UNICODE_ON as libc::c_int != UNICODE_ON as libc::c_int {
    *s = wc as libc::c_char;
    return 1i32 as size_t;
  }
  return wcrtomb_internal(s, wc);
}
#[no_mangle]
pub unsafe extern "C" fn bb_wcstombs(
  mut dest: *mut libc::c_char,
  mut src: *const wchar_t,
  mut n: size_t,
) -> size_t {
  let mut org_n: size_t = n;
  if UNICODE_ON as libc::c_int != UNICODE_ON as libc::c_int {
    while n != 0 {
      let fresh0 = src;
      src = src.offset(1);
      let mut c: wchar_t = *fresh0;
      let fresh1 = dest;
      dest = dest.offset(1);
      *fresh1 = c as libc::c_char;
      if c == 0i32 {
        break;
      }
      n = n.wrapping_sub(1)
    }
    return org_n.wrapping_sub(n);
  }
  while n >= 6i32 as libc::c_ulong {
    let fresh2 = src;
    src = src.offset(1);
    let mut wc: wchar_t = *fresh2;
    let mut len: size_t = wcrtomb_internal(dest, wc);
    if wc == '\u{0}' as i32 {
      return org_n.wrapping_sub(n);
    }
    dest = dest.offset(len as isize);
    n = (n as libc::c_ulong).wrapping_sub(len) as size_t as size_t
  }
  while n != 0 {
    let mut tbuf: [libc::c_char; 6] = [0; 6];
    let fresh3 = src;
    src = src.offset(1);
    let mut wc_0: wchar_t = *fresh3;
    let mut len_0: size_t = wcrtomb_internal(tbuf.as_mut_ptr(), wc_0);
    if len_0 > n {
      break;
    }
    memcpy(
      dest as *mut libc::c_void,
      tbuf.as_mut_ptr() as *const libc::c_void,
      len_0,
    );
    if wc_0 == '\u{0}' as i32 {
      return org_n.wrapping_sub(n);
    }
    dest = dest.offset(len_0 as isize);
    n = (n as libc::c_ulong).wrapping_sub(len_0) as size_t as size_t
  }
  return org_n.wrapping_sub(n);
}
unsafe extern "C" fn mbstowc_internal(
  mut res: *mut wchar_t,
  mut src: *const libc::c_char,
) -> *const libc::c_char {
  let mut bytes: libc::c_int = 0;
  let fresh4 = src;
  src = src.offset(1);
  let mut c: libc::c_uint = *fresh4 as libc::c_uchar as libc::c_uint;
  if c <= 0x7fi32 as libc::c_uint {
    *res = c as wchar_t;
    return src;
  }
  /* 80-7FF -> 110yyyxx 10xxxxxx */
  /* 800-FFFF -> 1110yyyy 10yyyyxx 10xxxxxx */
  /* 10000-1FFFFF -> 11110zzz 10zzyyyy 10yyyyxx 10xxxxxx */
  /* 200000-3FFFFFF -> 111110tt 10zzzzzz 10zzyyyy 10yyyyxx 10xxxxxx */
  /* 4000000-FFFFFFFF -> 111111tt 10tttttt 10zzzzzz 10zzyyyy 10yyyyxx 10xxxxxx */
  bytes = 0i32;
  loop {
    c <<= 1i32;
    bytes += 1;
    if !(c & 0x80i32 as libc::c_uint != 0 && bytes < 6i32) {
      break;
    }
  }
  if bytes == 1i32 {
    /* A bare "continuation" byte. Say, 80 */
    *res = !0i32;
    return src;
  }
  c = (c as u8 as libc::c_int >> bytes) as libc::c_uint;
  loop {
    bytes -= 1;
    if !(bytes != 0) {
      break;
    }
    let mut ch: libc::c_uint = *src as libc::c_uchar as libc::c_uint;
    if ch & 0xc0i32 as libc::c_uint != 0x80i32 as libc::c_uint {
      /* Missing "continuation" byte. Example: e0 80 */
      *res = !0i32;
      return src;
    }
    c = (c << 6i32).wrapping_add(ch & 0x3fi32 as libc::c_uint);
    src = src.offset(1)
  }
  /* TODO */
  /* Need to check that c isn't produced by overlong encoding */
  /* Example: 11000000 10000000 converts to NUL */
  /* 11110000 10000000 10000100 10000000 converts to 0x100 */
  /* correct encoding: 11000100 10000000 */
  if c <= 0x7fi32 as libc::c_uint {
    /* crude check */
    *res = !0i32;
    return src;
  }
  *res = c as wchar_t;
  return src;
}
#[no_mangle]
pub unsafe extern "C" fn bb_mbstowcs(
  mut dest: *mut wchar_t,
  mut src: *const libc::c_char,
  mut n: size_t,
) -> size_t {
  let mut org_n: size_t = n;
  if UNICODE_ON as libc::c_int != UNICODE_ON as libc::c_int {
    while n != 0 {
      let fresh5 = src;
      src = src.offset(1);
      let mut c: libc::c_uchar = *fresh5 as libc::c_uchar;
      if !dest.is_null() {
        let fresh6 = dest;
        dest = dest.offset(1);
        *fresh6 = c as wchar_t
      }
      if c as libc::c_int == 0i32 {
        break;
      }
      n = n.wrapping_sub(1)
    }
    return org_n.wrapping_sub(n);
  }
  while n != 0 {
    let mut wc: wchar_t = 0;
    src = mbstowc_internal(&mut wc, src);
    if wc == !0i32 {
      /* error */
      return -1i64 as size_t;
    }
    if !dest.is_null() {
      let fresh7 = dest;
      dest = dest.offset(1);
      *fresh7 = wc
    }
    if wc == 0i32 {
      break;
    }
    n = n.wrapping_sub(1)
  }
  return org_n.wrapping_sub(n);
}
#[no_mangle]
pub unsafe extern "C" fn bb_iswspace(mut wc: bb_wint_t) -> libc::c_int {
  return (wc as libc::c_uint <= 0x7fi32 as libc::c_uint
    && ({
      let mut bb__isspace: libc::c_uchar = (wc - 9i32) as libc::c_uchar;
      (bb__isspace as libc::c_int == ' ' as i32 - 9i32
        || bb__isspace as libc::c_int <= 13i32 - 9i32) as libc::c_int
    }) != 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn bb_iswalnum(mut wc: bb_wint_t) -> libc::c_int {
  return (wc as libc::c_uint <= 0x7fi32 as libc::c_uint
    && bb_ascii_isalnum(wc as libc::c_uchar) != 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn bb_iswpunct(mut wc: bb_wint_t) -> libc::c_int {
  return (wc as libc::c_uint <= 0x7fi32 as libc::c_uint
    && *strchrnul(
      b"!\"#$%&\'()*+,-./:;<=>?@[\\]^_`{|}~\x00" as *const u8 as *const libc::c_char,
      wc,
    )
    .offset(0) as libc::c_int
      != 0) as libc::c_int;
}
/*
 * This is an implementation of wcwidth() and wcswidth() (defined in
 * IEEE Std 1002.1-2001) for Unicode.
 *
 * http://www.opengroup.org/onlinepubs/007904975/functions/wcwidth.html
 * http://www.opengroup.org/onlinepubs/007904975/functions/wcswidth.html
 *
 * In fixed-width output devices, Latin characters all occupy a single
 * "cell" position of equal width, whereas ideographic CJK characters
 * occupy two such cells. Interoperability between terminal-line
 * applications and (teletype-style) character terminals using the
 * UTF-8 encoding requires agreement on which character should advance
 * the cursor by how many cell positions. No established formal
 * standards exist at present on which Unicode character shall occupy
 * how many cell positions on character terminals. These routines are
 * a first attempt of defining such behavior based on simple rules
 * applied to data provided by the Unicode Consortium.
 *
 * For some graphical characters, the Unicode standard explicitly
 * defines a character-cell width via the definition of the East Asian
 * FullWidth (F), Wide (W), Half-width (H), and Narrow (Na) classes.
 * In all these cases, there is no ambiguity about which width a
 * terminal shall use. For characters in the East Asian Ambiguous (A)
 * class, the width choice depends purely on a preference of backward
 * compatibility with either historic CJK or Western practice.
 * Choosing single-width for these characters is easy to justify as
 * the appropriate long-term solution, as the CJK practice of
 * displaying these characters as double-width comes from historic
 * implementation simplicity (8-bit encoded characters were displayed
 * single-width and 16-bit ones double-width, even for Greek,
 * Cyrillic, etc.) and not any typographic considerations.
 *
 * Much less clear is the choice of width for the Not East Asian
 * (Neutral) class. Existing practice does not dictate a width for any
 * of these characters. It would nevertheless make sense
 * typographically to allocate two character cells to characters such
 * as for instance EM SPACE or VOLUME INTEGRAL, which cannot be
 * represented adequately with a single-width glyph. The following
 * routines at present merely assign a single-cell width to all
 * neutral characters, in the interest of simplicity. This is not
 * entirely satisfactory and should be reconsidered before
 * establishing a formal standard in this area. At the moment, the
 * decision which Not East Asian (Neutral) characters should be
 * represented by double-width glyphs cannot yet be answered by
 * applying a simple rule from the Unicode database content. Setting
 * up a proper standard for the behavior of UTF-8 character terminals
 * will require a careful analysis not only of each Unicode character,
 * but also of each presentation form, something the author of these
 * routines has avoided to do so far.
 *
 * http://www.unicode.org/unicode/reports/tr11/
 *
 * Markus Kuhn -- 2007-05-26 (Unicode 5.0)
 *
 * Permission to use, copy, modify, and distribute this software
 * for any purpose and without fee is hereby granted. The author
 * disclaims all warranties with regard to this software.
 *
 * Latest version: http://www.cl.cam.ac.uk/~mgk25/ucs/wcwidth.c
 */
/* Assigned Unicode character ranges:
 * Plane Range
 * 0       0000–FFFF   Basic Multilingual Plane
 * 1      10000–1FFFF  Supplementary Multilingual Plane
 * 2      20000–2FFFF  Supplementary Ideographic Plane
 * 3      30000-3FFFF  Tertiary Ideographic Plane (no chars assigned yet)
 * 4-13   40000–DFFFF  currently unassigned
 * 14     E0000–EFFFF  Supplementary Special-purpose Plane
 * 15     F0000–FFFFF  Supplementary Private Use Area-A
 * 16    100000–10FFFF Supplementary Private Use Area-B
 *
 * "Supplementary Special-purpose Plane currently contains non-graphical
 * characters in two blocks of 128 and 240 characters. The first block
 * is for language tag characters for use when language cannot be indicated
 * through other protocols (such as the xml:lang  attribute in XML).
 * The other block contains glyph variation selectors to indicate
 * an alternate glyph for a character that cannot be determined by context."
 *
 * In simpler terms: it is a tool to fix the "Han unification" mess
 * created by Unicode committee, to select Chinese/Japanese/Korean/Taiwan
 * version of a character. (They forgot that the whole purpose of the Unicode
 * was to be able to write all chars in one charset without such tricks).
 * Until East Asian users say it is actually necessary to support these
 * code points in console applications like busybox
 * (i.e. do these chars ever appear in filenames, hostnames, text files
 * and such?), we are treating these code points as invalid.
 *
 * Tertiary Ideographic Plane is also ignored for now,
 * until Unicode committee assigns something there.
 */
/* The following two functions define the column width of an ISO 10646
 * character as follows:
 *
 *    - The null character (U+0000) has a column width of 0.
 *
 *    - Other C0/C1 control characters and DEL will lead to a return
 *      value of -1.
 *
 *    - Non-spacing and enclosing combining characters (general
 *      category code Mn or Me in the Unicode database) have a
 *      column width of 0.
 *
 *    - SOFT HYPHEN (U+00AD) has a column width of 1.
 *
 *    - Other format characters (general category code Cf in the Unicode
 *      database) and ZERO WIDTH SPACE (U+200B) have a column width of 0.
 *
 *    - Hangul Jamo medial vowels and final consonants (U+1160-U+11FF)
 *      have a column width of 0.
 *
 *    - Spacing characters in the East Asian Wide (W) or East Asian
 *      Full-width (F) category as defined in Unicode Technical
 *      Report #11 have a column width of 2.
 *
 *    - All remaining characters (including all printable
 *      ISO 8859-1 and WGL4 characters, Unicode control characters,
 *      etc.) have a column width of 1.
 *
 * This implementation assumes that wchar_t characters are encoded
 * in ISO 10646.
 */
#[no_mangle]
pub unsafe extern "C" fn bb_wcwidth(mut ucs: libc::c_uint) -> libc::c_int {
  if ucs == 0i32 as libc::c_uint {
    return 0i32;
  }
  /* Test for 8-bit control characters (00-1f, 80-9f, 7f) */
  if (ucs & !0x80i32 as libc::c_uint) < 0x20i32 as libc::c_uint || ucs == 0x7fi32 as libc::c_uint {
    return -1i32;
  }
  /* Quick abort if it is an obviously invalid char */
  if ucs > 767i32 as libc::c_uint {
    return -1i32;
  }
  /* Optimization: no combining chars below 0x300 */
  if 767i32 < 0x300i32 || ucs < 0x300i32 as libc::c_uint {
    return 1i32;
  }
  panic!("Reached end of non-void function without returning");
  /* >= 0x300 */
}
/* Number of unicode chars. Falls back to strlen() on invalid unicode */
/* UNICODE_BIDI_SUPPORT */
/* Homegrown Unicode support */
/* The rest is mostly same for libc and for "homegrown" support */
#[no_mangle]
pub unsafe extern "C" fn unicode_strlen(mut string: *const libc::c_char) -> size_t {
  let mut width: size_t = bb_mbstowcs(0 as *mut wchar_t, string, 2147483647i32 as size_t);
  if width == -1i64 as size_t {
    return strlen(string);
  }
  return width;
}
/* Width on terminal */
#[no_mangle]
pub unsafe extern "C" fn unicode_strwidth(mut string: *const libc::c_char) -> size_t {
  let mut uni_stat: uni_stat_t = uni_stat_t {
    byte_count: 0,
    unicode_count: 0,
    unicode_width: 0,
  };
  printable_string2(&mut uni_stat, string);
  return uni_stat.unicode_width as size_t;
}
unsafe extern "C" fn unicode_conv_to_printable2(
  mut stats: *mut uni_stat_t,
  mut src: *const libc::c_char,
  mut width: libc::c_uint,
  mut flags: libc::c_int,
) -> *mut libc::c_char {
  let mut current_block: u64;
  let mut dst: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut dst_len: libc::c_uint = 0;
  let mut uni_count: libc::c_uint = 0;
  let mut uni_width: libc::c_uint = 0;
  if UNICODE_ON as libc::c_int != UNICODE_ON as libc::c_int {
    let mut d: *mut libc::c_char = 0 as *mut libc::c_char;
    if flags & UNI_FLAG_PAD as libc::c_int != 0 {
      dst = xmalloc(width.wrapping_add(1i32 as libc::c_uint) as size_t) as *mut libc::c_char;
      d = dst;
      loop {
        width = width.wrapping_sub(1);
        if !(width as libc::c_int >= 0i32) {
          break;
        }
        let mut c: libc::c_uchar = *src as libc::c_uchar;
        if c as libc::c_int == '\u{0}' as i32 {
          loop {
            let fresh8 = d;
            d = d.offset(1);
            *fresh8 = ' ' as i32 as libc::c_char;
            width = width.wrapping_sub(1);
            if !(width as libc::c_int >= 0i32) {
              break;
            }
          }
          break;
        } else {
          let fresh9 = d;
          d = d.offset(1);
          *fresh9 = if c as libc::c_int >= ' ' as i32 && (c as libc::c_int) < 0x7fi32 {
            c as libc::c_int
          } else {
            '?' as i32
          } as libc::c_char;
          src = src.offset(1)
        }
      }
      *d = '\u{0}' as i32 as libc::c_char
    } else {
      dst = xstrndup(src, width as libc::c_int);
      d = dst;
      while *d != 0 {
        let mut c_0: libc::c_uchar = *d as libc::c_uchar;
        if (c_0 as libc::c_int) < ' ' as i32 || c_0 as libc::c_int >= 0x7fi32 {
          *d = '?' as i32 as libc::c_char
        }
        d = d.offset(1)
      }
    }
    if !stats.is_null() {
      (*stats).byte_count = d.wrapping_offset_from(dst) as libc::c_long as libc::c_uint;
      (*stats).unicode_count = d.wrapping_offset_from(dst) as libc::c_long as libc::c_uint;
      (*stats).unicode_width = d.wrapping_offset_from(dst) as libc::c_long as libc::c_uint
    }
    return dst;
  }
  dst = 0 as *mut libc::c_char;
  uni_width = 0i32 as libc::c_uint;
  uni_count = uni_width;
  dst_len = 0i32 as libc::c_uint;
  loop {
    let mut w: libc::c_int = 0;
    let mut wc: wchar_t = 0;
    src = mbstowc_internal(&mut wc, src);
    /* src is advanced to next mb char
     * wc == ERROR_WCHAR: invalid sequence is seen
     * else: wc is set
     */
    if wc == !0i32 {
      current_block = 6651542264556950584;
    } else {
      if wc == 0i32 {
        break;
      }
      if 767i32 != 0 && wc > 767i32 {
        current_block = 6651542264556950584;
      } else {
        w = bb_wcwidth(wc as libc::c_uint);
        if 0i32 != 0 && w < 0i32 || 0i32 == 0 && w <= 0i32 || 0i32 == 0 && w > 1i32 {
          current_block = 6651542264556950584;
        } else {
          current_block = 12199444798915819164;
        }
      }
    }
    match current_block {
      6651542264556950584 =>
      /* error */
      {
        wc = 63i32;
        w = 1i32
      }
      _ => {}
    }
    width = width.wrapping_sub(w as libc::c_uint);
    /* Note: if width == 0, we still may add more chars,
     * they may be zero-width or combining ones */
    if (width as libc::c_int) < 0i32 {
      /* can't add this wc, string would become longer than width */
      width = width.wrapping_add(w as libc::c_uint);
      break;
    } else {
      uni_count = uni_count.wrapping_add(1);
      uni_width = uni_width.wrapping_add(w as libc::c_uint);
      dst = xrealloc(
        dst as *mut libc::c_void,
        dst_len.wrapping_add(6i32 as libc::c_uint) as size_t,
      ) as *mut libc::c_char;
      dst_len = (dst_len as libc::c_ulong)
        .wrapping_add(wcrtomb_internal(&mut *dst.offset(dst_len as isize), wc))
        as libc::c_uint as libc::c_uint
    }
  }
  /* end-of-string */
  /* Pad to remaining width */
  if flags & UNI_FLAG_PAD as libc::c_int != 0 {
    dst = xrealloc(
      dst as *mut libc::c_void,
      dst_len
        .wrapping_add(width)
        .wrapping_add(1i32 as libc::c_uint) as size_t,
    ) as *mut libc::c_char;
    uni_count = uni_count.wrapping_add(width);
    uni_width = uni_width.wrapping_add(width);
    loop {
      width = width.wrapping_sub(1);
      if !(width as libc::c_int >= 0i32) {
        break;
      }
      let fresh10 = dst_len;
      dst_len = dst_len.wrapping_add(1);
      *dst.offset(fresh10 as isize) = ' ' as i32 as libc::c_char
    }
  }
  if dst.is_null() {
    /* for example, if input was "" */
    dst = xzalloc(1i32 as size_t) as *mut libc::c_char
  }
  *dst.offset(dst_len as isize) = '\u{0}' as i32 as libc::c_char;
  if !stats.is_null() {
    (*stats).byte_count = dst_len;
    (*stats).unicode_count = uni_count;
    (*stats).unicode_width = uni_width
  }
  return dst;
}
//UNUSED: unsigned FAST_FUNC unicode_padding_to_width(unsigned width, const char *src);
//UNUSED: char* FAST_FUNC unicode_conv_to_printable2(uni_stat_t *stats, const char *src, unsigned width, int flags);
#[no_mangle]
pub unsafe extern "C" fn unicode_conv_to_printable(
  mut stats: *mut uni_stat_t,
  mut src: *const libc::c_char,
) -> *mut libc::c_char {
  return unicode_conv_to_printable2(stats, src, 2147483647i32 as libc::c_uint, 0i32);
}
//UNUSED: char* FAST_FUNC unicode_conv_to_printable_maxwidth(uni_stat_t *stats, const char *src, unsigned maxwidth);
#[no_mangle]
pub unsafe extern "C" fn unicode_conv_to_printable_fixedwidth(
  mut src: *const libc::c_char,
  mut width: libc::c_uint,
) -> *mut libc::c_char {
  return unicode_conv_to_printable2(
    0 as *mut uni_stat_t,
    src,
    width,
    UNI_FLAG_PAD as libc::c_int,
  );
}
