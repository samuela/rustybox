use crate::libbb::parse_config::parser_t;
use crate::libbb::skip_whitespace::skip_whitespace;
use libc;
use libc::access;
use libc::close;
use libc::free;
use libc::getenv;
use libc::isatty;
use libc::putenv;
use libc::puts;
use libc::strchr;
use libc::strcmp;
use libc::strcpy;
use libc::strrchr;
use libc::system;
extern "C" {

  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  fn strchrnul(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;

  /* After v = xrealloc_vector(v, SHIFT, idx) it's ok to use
   * at least v[idx] and v[idx+1], for all idx values.
   * SHIFT specifies how many new elements are added (1:2, 2:4, ..., 8:256...)
   * when all elements are used up. New elements are zeroed out.
   * xrealloc_vector(v, SHIFT, idx) *MUST* be called with consecutive IDXs -
   * skipping an index is a bad bug - it may miss a realloc!
   */

  /* Autodetects .gz etc */

  /* { "-", NULL } */
  #[no_mangle]
  static mut option_mask32: u32;

  /* delims[0] is a comment char (use '\0' to disable), the rest are token delimiters */

  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
}

use crate::librb::size_t;
use libc::stat;
use libc::FILE;
/*
 * Config file parser
 */
pub type C2RustUnnamed = libc::c_uint;
// comments are recognized even if there is whitespace before
// ("line start><space><tab><space>#comment" is also comment, not only "line start>#comment")
// NORMAL is:
// * remove leading and trailing delimiters and collapse
//   multiple delimiters into one
// * warn and continue if less than mintokens delimiters found
// * grab everything into last token
// * comments are recognized even if they aren't the first char
pub const PARSE_NORMAL: C2RustUnnamed = 4653056;
// delim[0] and delim[1] are two different allowed comment chars
// (so far, delim[0] will only work as comment char for full-line comment)
// (IOW: it works as if PARSE_EOL_COMMENTS is not set. sysctl applet is okay with this)
pub const PARSE_WS_COMMENTS: C2RustUnnamed = 16777216;
// comments are recognized even if they aren't the first char
pub const PARSE_ALT_COMMENTS: C2RustUnnamed = 8388608;
pub const PARSE_EOL_COMMENTS: C2RustUnnamed = 4194304;
// die if < min tokens found
// keep a copy of current line
pub const PARSE_KEEP_COPY: C2RustUnnamed = 2097152;
// last token takes entire remainder of the line
pub const PARSE_MIN_DIE: C2RustUnnamed = 1048576;
// trim leading and trailing delimiters
// TODO: COLLAPSE and TRIM seem to always go in pair
pub const PARSE_GREEDY: C2RustUnnamed = 262144;
// treat consecutive delimiters as one
pub const PARSE_TRIM: C2RustUnnamed = 131072;
pub const PARSE_COLLAPSE: C2RustUnnamed = 65536;

//extern const int const_int_1;
/* This struct is deliberately not defined. */
/* See docs/keep_data_small.txt */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct globals {
  pub col: *const libc::c_char,
  pub tbl: *const libc::c_char,
  pub nroff: *const libc::c_char,
  pub pager: *const libc::c_char,
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const OPT_w: C2RustUnnamed_0 = 2;
pub const OPT_a: C2RustUnnamed_0 = 1;
unsafe extern "C" fn run_pipe(
  mut man_filename: *mut libc::c_char,
  mut man: libc::c_int,
  mut level: libc::c_int,
) -> libc::c_int {
  let mut current_block: u64;
  let mut cmd: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  /* Prevent man page link loops */
  if level > 10i32 {
    return 0i32;
  }
  if access(man_filename, 4i32) != 0i32 {
    return 0i32;
  }
  if option_mask32 & OPT_w as libc::c_int as libc::c_uint != 0 {
    puts(man_filename);
    return 1i32;
  }
  if man != 0 {
    /* man page, not cat page */
    /* Is this a link to another manpage? */
    /* The link has the following on the first line: */
    /* ".so another_man_page" */
    let mut sb: stat = std::mem::zeroed();
    let mut line: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut linkname: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut p: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    /* On my system:
     * man1/genhostid.1.gz: 203 bytes - smallest real manpage
     * man2/path_resolution.2.gz: 114 bytes - largest link
     */
    crate::libbb::xfuncs_printf::xstat(man_filename, &mut sb);
    if !(sb.st_size > 300i32 as libc::c_long) {
      line = crate::archival::libarchive::open_transformer::xmalloc_open_zipped_read_close(
        man_filename,
        std::ptr::null_mut::<size_t>(),
      ) as *mut libc::c_char;
      if line.is_null()
        || crate::libbb::compare_string_array::is_prefixed_with(
          line,
          b".so \x00" as *const u8 as *const libc::c_char,
        )
        .is_null()
      {
        free(line as *mut libc::c_void);
      } else {
        /* Example: man2/path_resolution.2.gz contains
         * ".so man7/path_resolution.7\n<junk>"
         */
        *strchrnul(line, '\n' as i32) = '\u{0}' as i32 as libc::c_char;
        linkname = skip_whitespace(&mut *line.offset(4));
        /* If link has no slashes, we just replace man page name.
         * If link has slashes (however many), we go back *once*.
         * ".so zzz/ggg/page.3" does NOT go back two levels. */
        p = strrchr(man_filename, '/' as i32);
        if !p.is_null() {
          *p = '\u{0}' as i32 as libc::c_char;
          if !strchr(linkname, '/' as i32).is_null() {
            p = strrchr(man_filename, '/' as i32);
            if p.is_null() {
              current_block = 14270494255691478666;
            } else {
              *p = '\u{0}' as i32 as libc::c_char;
              current_block = 11298138898191919651;
            }
          } else {
            current_block = 11298138898191919651;
          }
          match current_block {
            14270494255691478666 => {}
            _ => {
              /* Links do not have .gz extensions, even if manpage
               * is compressed */
              man_filename = crate::libbb::xfuncs_printf::xasprintf(
                b"%s/%s\x00" as *const u8 as *const libc::c_char,
                man_filename,
                linkname,
              );
              free(line as *mut libc::c_void);
              /* Note: we leak "new" man_filename string as well... */
              if show_manpage(man_filename, man, level + 1i32) != 0 {
                return 1i32;
              }
            }
          }
        }
      }
    }
    /* else: show the link, it's better than nothing */
  }
  /* err on the safe side */
  close(0i32); /* guaranteed to use fd 0 (STDIN_FILENO) */
  crate::archival::libarchive::open_transformer::open_zipped(man_filename, 0i32);
  if man != 0 {
    let mut w: libc::c_int = crate::libbb::xfuncs::get_terminal_width(-1i32);
    if w > 10i32 {
      w -= 2i32
    }
    /* "2>&1" is added so that nroff errors are shown in pager too.
     * Otherwise it may show just empty screen.
     */
    cmd = crate::libbb::xfuncs_printf::xasprintf(
      b"%s | %s -rLL=%un -rLT=%un 2>&1 | %s\x00" as *const u8 as *const libc::c_char,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).tbl,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nroff,
      w,
      w,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).pager,
    )
  } else {
    cmd = crate::libbb::xfuncs_printf::xstrdup(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).pager,
    )
  }
  system(cmd);
  free(cmd as *mut libc::c_void);
  return 1i32;
}
/* Removed -Tlatin1. Assuming system nroff has suitable default */
/* man_filename is of the form "/dir/dir/dir/name.s" */
unsafe extern "C" fn show_manpage(
  mut man_filename: *mut libc::c_char,
  mut man: libc::c_int,
  mut level: libc::c_int,
) -> libc::c_int {
  /* We leak this allocation... */
  let mut filename_with_zext: *mut libc::c_char = crate::libbb::xfuncs_printf::xasprintf(
    b"%s.lzma\x00" as *const u8 as *const libc::c_char,
    man_filename,
  );
  let mut ext: *mut libc::c_char = strrchr(filename_with_zext, '.' as i32).offset(1);
  if run_pipe(filename_with_zext, man, level) != 0 {
    return 1i32;
  }
  strcpy(ext, b"xz\x00" as *const u8 as *const libc::c_char);
  if run_pipe(filename_with_zext, man, level) != 0 {
    return 1i32;
  }
  strcpy(ext, b"bz2\x00" as *const u8 as *const libc::c_char);
  if run_pipe(filename_with_zext, man, level) != 0 {
    return 1i32;
  }
  strcpy(ext, b"gz\x00" as *const u8 as *const libc::c_char);
  if run_pipe(filename_with_zext, man, level) != 0 {
    return 1i32;
  }
  return run_pipe(man_filename, man, level);
}
unsafe extern "C" fn add_MANPATH(
  mut man_path_list: *mut *mut libc::c_char,
  mut count_mp: *mut libc::c_int,
  mut path: *mut libc::c_char,
) -> *mut *mut libc::c_char {
  if !path.is_null() {
    let mut current_block_8: u64;
    while *path != 0 {
      let mut next_path: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
      let mut path_element: *mut *mut libc::c_char = std::ptr::null_mut();
      next_path = strchr(path, ':' as i32);
      if !next_path.is_null() {
        if next_path == path {
          current_block_8 = 10067683555244574201;
        } else {
          *next_path = '\u{0}' as i32 as libc::c_char;
          current_block_8 = 4906268039856690917;
        }
      } else {
        current_block_8 = 4906268039856690917;
      }
      match current_block_8 {
        4906268039856690917 => {
          /* Do we already have path? */
          path_element = man_path_list;
          if !path_element.is_null() {
            current_block_8 = 17216689946888361452;
          } else {
            current_block_8 = 2979737022853876585;
          }
          loop {
            match current_block_8 {
              17216689946888361452 => {
                if (*path_element).is_null() {
                  current_block_8 = 2979737022853876585;
                  continue;
                }
                if strcmp(*path_element, path) == 0i32 {
                  break;
                }
                path_element = path_element.offset(1);
                current_block_8 = 17216689946888361452;
              }
              _ => {
                man_path_list = crate::libbb::xrealloc_vector::xrealloc_vector_helper(
                  man_path_list as *mut libc::c_void,
                  ((::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong) << 8i32)
                    .wrapping_add(4i32 as libc::c_ulong) as libc::c_uint,
                  *count_mp,
                ) as *mut *mut libc::c_char;
                let ref mut fresh0 = *man_path_list.offset(*count_mp as isize);
                *fresh0 = crate::libbb::xfuncs_printf::xstrdup(path);
                *count_mp += 1;
                break;
              }
            }
          }
          /* man_path_list is NULL terminated */
          /* man_path_list[*count_mp] = NULL; - xrealloc_vector did it */
          if next_path.is_null() {
            break;
          }
          /* "path" may be a result of getenv(), be nice and don't mangle it */
          *next_path = ':' as i32 as libc::c_char
        }
        _ => {}
      }
      /* "::"? */
      path = next_path.offset(1)
    }
  }
  return man_path_list;
}
unsafe extern "C" fn if_redefined(
  mut var: *const libc::c_char,
  mut key: *const libc::c_char,
  mut line: *const libc::c_char,
) -> *const libc::c_char {
  if crate::libbb::compare_string_array::is_prefixed_with(line, key).is_null() {
    return var;
  }
  line = line.offset(strlen(key) as isize);
  if ({
    let mut bb__isspace: libc::c_uchar = (*line.offset(0) as libc::c_int - 9i32) as libc::c_uchar;
    (bb__isspace as libc::c_int == ' ' as i32 - 9i32 || bb__isspace as libc::c_int <= 13i32 - 9i32)
      as libc::c_int
  }) == 0
  {
    return var;
  }
  return crate::libbb::xfuncs_printf::xstrdup(skip_whitespace(line));
}
#[no_mangle]
pub unsafe extern "C" fn man_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut parser: *mut parser_t = std::ptr::null_mut();
  let mut sec_list: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut man_path_list: *mut *mut libc::c_char = std::ptr::null_mut();
  let mut count_mp: libc::c_int = 0;
  let mut opt: libc::c_int = 0;
  let mut not_found: libc::c_int = 0;
  let mut token: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2];
  let ref mut fresh1 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).col;
  *fresh1 = b"col\x00" as *const u8 as *const libc::c_char;
  let ref mut fresh2 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).tbl;
  *fresh2 = b"tbl\x00" as *const u8 as *const libc::c_char;
  let ref mut fresh3 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nroff;
  *fresh3 = b"nroff -mandoc\x00" as *const u8 as *const libc::c_char;
  let ref mut fresh4 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).pager;
  *fresh4 = if 1i32 != 0 {
    b"less\x00" as *const u8 as *const libc::c_char
  } else {
    b"more\x00" as *const u8 as *const libc::c_char
  };
  opt =
    crate::libbb::getopt32::getopt32(argv, b"^+aw\x00-1\x00" as *const u8 as *const libc::c_char)
      as libc::c_int;
  argv = argv.offset(optind as isize);
  sec_list = crate::libbb::xfuncs_printf::xstrdup(
    b"0p:1:1p:2:3:3p:4:5:6:7:8:9\x00" as *const u8 as *const libc::c_char,
  );
  count_mp = 0i32;
  man_path_list = add_MANPATH(
    0 as *mut *mut libc::c_char,
    &mut count_mp,
    getenv((b"MANDATORY_MANPATH\x00" as *const u8 as *const libc::c_char).offset(10)),
  );
  /* Parse man.conf[ig] or man_db.conf */
  /* man version 1.6f uses man.config */
  /* man-db implementation of man uses man_db.conf */
  parser = crate::libbb::parse_config::config_open2(
    b"/etc/man.config\x00" as *const u8 as *const libc::c_char,
    Some(
      crate::libbb::wfopen::fopen_for_read
        as unsafe extern "C" fn(_: *const libc::c_char) -> *mut FILE,
    ),
  );
  if parser.is_null() {
    parser = crate::libbb::parse_config::config_open2(
      b"/etc/man.conf\x00" as *const u8 as *const libc::c_char,
      Some(
        crate::libbb::wfopen::fopen_for_read
          as unsafe extern "C" fn(_: *const libc::c_char) -> *mut FILE,
      ),
    )
  }
  if parser.is_null() {
    parser = crate::libbb::parse_config::config_open2(
      b"/etc/man_db.conf\x00" as *const u8 as *const libc::c_char,
      Some(
        crate::libbb::wfopen::fopen_for_read
          as unsafe extern "C" fn(_: *const libc::c_char) -> *mut FILE,
      ),
    )
  }
  while crate::libbb::parse_config::config_read(
    parser,
    token.as_mut_ptr(),
    (PARSE_NORMAL as libc::c_int | (0i32 & 0xffi32) << 8i32 | 2i32 & 0xffi32) as libc::c_uint,
    b"# \t\x00" as *const u8 as *const libc::c_char,
  ) != 0
  {
    if token[1].is_null() {
      continue;
    }
    if strcmp(b"DEFINE\x00" as *const u8 as *const libc::c_char, token[0]) == 0i32 {
      let ref mut fresh5 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).col;
      *fresh5 = if_redefined(
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).col,
        b"col\x00" as *const u8 as *const libc::c_char,
        token[1],
      );
      let ref mut fresh6 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).tbl;
      *fresh6 = if_redefined(
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).tbl,
        b"tbl\x00" as *const u8 as *const libc::c_char,
        token[1],
      );
      let ref mut fresh7 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nroff;
      *fresh7 = if_redefined(
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nroff,
        b"nroff\x00" as *const u8 as *const libc::c_char,
        token[1],
      );
      let ref mut fresh8 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).pager;
      *fresh8 = if_redefined(
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).pager,
        b"pager\x00" as *const u8 as *const libc::c_char,
        token[1],
      )
    } else if strcmp(
      (b"MANDATORY_MANPATH\x00" as *const u8 as *const libc::c_char).offset(10),
      token[0],
    ) == 0i32
      || strcmp(
        b"MANDATORY_MANPATH\x00" as *const u8 as *const libc::c_char,
        token[0],
      ) == 0i32
    {
      man_path_list = add_MANPATH(man_path_list, &mut count_mp, token[1])
    }
    if strcmp(b"MANSECT\x00" as *const u8 as *const libc::c_char, token[0]) == 0i32 {
      free(sec_list as *mut libc::c_void);
      sec_list = crate::libbb::xfuncs_printf::xstrdup(token[1])
    }
  }
  crate::libbb::parse_config::config_close(parser);
  if man_path_list.is_null() {
    static mut mpl: [*const libc::c_char; 3] = [
      b"/usr/man\x00" as *const u8 as *const libc::c_char,
      b"/usr/share/man\x00" as *const u8 as *const libc::c_char,
      0 as *const libc::c_char,
    ];
    man_path_list = mpl.as_ptr() as *mut *mut libc::c_char
    /*count_mp = 2; - not used below anyway */
  }
  /* environment overrides setting from man.config */
  let mut env_pager: *mut libc::c_char =
    getenv(b"MANPAGER\x00" as *const u8 as *const libc::c_char);
  if env_pager.is_null() {
    env_pager = getenv(b"PAGER\x00" as *const u8 as *const libc::c_char)
  }
  if !env_pager.is_null() {
    let ref mut fresh9 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).pager;
    *fresh9 = env_pager
  }
  if isatty(1i32) == 0 {
    putenv(b"GROFF_NO_SGR=1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
    let ref mut fresh10 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).pager;
    *fresh10 = crate::libbb::xfuncs_printf::xasprintf(
      b"%s -b -p -x\x00" as *const u8 as *const libc::c_char,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).col,
    )
  }
  not_found = 0i32;
  loop {
    let mut current_block_61: u64;
    /* for each argv[] */
    let mut cur_path: *const libc::c_char = std::ptr::null();
    let mut cur_mp: libc::c_int = 0;
    let mut found: libc::c_int = 0i32;
    if !strchr(*argv, '/' as i32).is_null() {
      found = show_manpage(*argv, 1i32, 0i32);
      current_block_61 = 13839692391726842101;
    } else {
      cur_mp = 0i32;
      's_267: loop {
        let fresh11 = cur_mp;
        cur_mp = cur_mp + 1;
        cur_path = *man_path_list.offset(fresh11 as isize);
        if cur_path.is_null() {
          current_block_61 = 13839692391726842101;
          break;
        }
        /* for each MANPATH */
        let mut cur_sect: *const libc::c_char = sec_list;
        loop
        /* for each section */
        {
          let mut next_sect: *mut libc::c_char = strchrnul(cur_sect, ':' as i32);
          let mut sect_len: libc::c_int =
            next_sect.wrapping_offset_from(cur_sect) as libc::c_long as libc::c_int;
          let mut man_filename: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
          let mut cat0man1: libc::c_int = 0i32;
          /* Search for cat, then man page */
          while cat0man1 < 2i32 {
            let mut found_here: libc::c_int = 0;
            man_filename = crate::libbb::xfuncs_printf::xasprintf(
              b"%s/%s%.*s/%s.%.*s\x00" as *const u8 as *const libc::c_char,
              cur_path,
              (b"cat\x00man\x00" as *const u8 as *const libc::c_char)
                .offset((cat0man1 * 4i32) as isize),
              sect_len,
              cur_sect,
              *argv,
              sect_len,
              cur_sect,
            );
            found_here = show_manpage(man_filename, cat0man1, 0i32);
            found |= found_here;
            cat0man1 += found_here + 1i32;
            free(man_filename as *mut libc::c_void);
          }
          if found != 0 && opt & OPT_a as libc::c_int == 0 {
            current_block_61 = 5677601115122216434;
            break 's_267;
          }
          cur_sect = next_sect;
          while *cur_sect as libc::c_int == ':' as i32 {
            cur_sect = cur_sect.offset(1)
          }
          if !(*cur_sect != 0) {
            break;
          }
        }
      }
    }
    match current_block_61 {
      13839692391726842101 => {
        if found == 0 {
          crate::libbb::verror_msg::bb_error_msg(
            b"no manual entry for \'%s\'\x00" as *const u8 as *const libc::c_char,
            *argv,
          );
          not_found = 1i32
        }
      }
      _ => {}
    }
    argv = argv.offset(1);
    if (*argv).is_null() {
      break;
    }
  }
  return not_found;
}
