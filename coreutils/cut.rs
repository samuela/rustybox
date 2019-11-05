use libc;
extern "C" {
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  static mut stdout: *mut FILE;
  #[no_mangle]
  fn putchar_unlocked(__c: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn puts(__s: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t, __compar: __compar_fn_t);
  #[no_mangle]
  fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn strsep(__stringp: *mut *mut libc::c_char, __delim: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xrealloc_vector_helper(
    vector: *mut libc::c_void,
    sizeof_and_shift: libc::c_uint,
    idx: libc::c_int,
  ) -> *mut libc::c_void;
  #[no_mangle]
  fn xmalloc_fgetline(file: *mut FILE) -> *mut libc::c_char;
  #[no_mangle]
  fn fflush_stdout_and_exit(retval: libc::c_int) -> !;
  #[no_mangle]
  fn fclose_if_not_stdin(file: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn fopen_or_warn_stdin(filename: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn xatoi_positive(numstr: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  static mut option_mask32: uint32_t;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
}

use crate::librb::__compar_fn_t;

use crate::librb::size_t;
use crate::librb::uint32_t;
use crate::librb::FILE;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct cut_list {
  pub startpos: libc::c_int,
  pub endpos: libc::c_int,
}
pub type C2RustUnnamed = libc::c_int;
pub const NON_RANGE: C2RustUnnamed = -1;
pub const EOL: C2RustUnnamed = 2147483647;
pub const BOL: C2RustUnnamed = 0;
unsafe extern "C" fn cmpfunc(
  mut a: *const libc::c_void,
  mut b: *const libc::c_void,
) -> libc::c_int {
  return (*(a as *mut cut_list)).startpos - (*(b as *mut cut_list)).startpos; /* keep these zero-based to be consistent */
}
unsafe extern "C" fn cut_file(
  mut file: *mut FILE,
  mut delim: libc::c_char,
  mut cut_lists: *const cut_list,
  mut nlists: libc::c_uint,
) {
  let mut current_block: u64;
  let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut linenum: libc::c_uint = 0i32 as libc::c_uint;
  loop
  /* go through every line in the file */
  {
    line = xmalloc_fgetline(file);
    if line.is_null() {
      break;
    }
    /* set up a list so we can keep track of what's been printed */
    let mut linelen: libc::c_int = strlen(line) as libc::c_int;
    let mut printed: *mut libc::c_char = xzalloc((linelen + 1i32) as size_t) as *mut libc::c_char;
    let mut orig_line: *mut libc::c_char = line;
    let mut cl_pos: libc::c_uint = 0i32 as libc::c_uint;
    let mut spos: libc::c_int = 0;
    /* cut based on chars/bytes XXX: only works when sizeof(char) == byte */
    if option_mask32 & (1i32 << 1i32 | 1i32 << 0i32) as libc::c_uint != 0 {
      /* print the chars specified in each cut list */
      while cl_pos < nlists {
        spos = (*cut_lists.offset(cl_pos as isize)).startpos; /* cut by fields */
        while spos < linelen {
          if *printed.offset(spos as isize) == 0 {
            *printed.offset(spos as isize) = 'X' as i32 as libc::c_char;
            putchar_unlocked(*line.offset(spos as isize) as libc::c_int);
          }
          spos += 1;
          if spos > (*cut_lists.offset(cl_pos as isize)).endpos {
            break;
          }
        }
        cl_pos = cl_pos.wrapping_add(1)
      }
      current_block = 7343950298149844727;
    } else if delim as libc::c_int == '\n' as i32 {
      /* cut by lines */
      spos = (*cut_lists.offset(cl_pos as isize)).startpos;
      /* get out if we have no more lists to process or if the lines
       * are lower than what we're interested in */
      if (linenum as libc::c_int) < spos || cl_pos >= nlists {
        current_block = 4372395669998863707;
      } else {
        loop
        /* if the line we're looking for is lower than the one we were
         * passed, it means we displayed it already, so move on */
        {
          if !(spos < linenum as libc::c_int) {
            current_block = 4775909272756257391;
            break;
          }
          spos += 1;
          /* go to the next list if we're at the end of this one */
          if !(spos > (*cut_lists.offset(cl_pos as isize)).endpos
            || (*cut_lists.offset(cl_pos as isize)).endpos == NON_RANGE as libc::c_int)
          {
            continue;
          }
          cl_pos = cl_pos.wrapping_add(1);
          /* get out if there's no more lists to process */
          if cl_pos >= nlists {
            current_block = 4372395669998863707;
            break;
          }
          spos = (*cut_lists.offset(cl_pos as isize)).startpos;
          /* get out if the current line is lower than the one
           * we just became interested in */
          if (linenum as libc::c_int) < spos {
            current_block = 4372395669998863707;
            break;
          }
        }
        match current_block {
          4372395669998863707 => {}
          _ => {
            /* If we made it here, it means we've found the line we're
             * looking for, so print it */
            puts(line); /* zero-based / one-based problem */
            current_block = 4372395669998863707;
          }
        }
      }
    } else {
      let mut ndelim: libc::c_int = -1i32;
      let mut nfields_printed: libc::c_int = 0i32;
      let mut field: *mut libc::c_char = 0 as *mut libc::c_char;
      let mut delimiter: [libc::c_char; 2] = [0; 2];
      delimiter[0] = delim;
      delimiter[1] = 0i32 as libc::c_char;
      /* does this line contain any delimiters? */
      if strchr(line, delim as libc::c_int).is_null() {
        if option_mask32 & (1i32 << 4i32) as libc::c_uint == 0 {
          puts(line);
        }
        current_block = 4372395669998863707;
      } else {
        /* process each list on this line, for as long as we've got
         * a line to process */
        while cl_pos < nlists && !line.is_null() {
          spos = (*cut_lists.offset(cl_pos as isize)).startpos;
          loop {
            /* find the field we're looking for */
            while !line.is_null() && ndelim < spos {
              field = strsep(&mut line, delimiter.as_mut_ptr());
              ndelim += 1
            }
            /* keep going as long as we have a line to work with,
             * this is a list, and we're not at the end of that
             * list */
            if !field.is_null() && ndelim == spos && *printed.offset(ndelim as isize) == 0 {
              /* we found it, and it hasn't been printed yet */
              /* if this isn't our first time through, we need to
               * print the delimiter after the last field that was
               * printed */
              if nfields_printed > 0i32 {
                putchar_unlocked(delim as libc::c_int);
              }
              fputs_unlocked(field, stdout);
              *printed.offset(ndelim as isize) = 'X' as i32 as libc::c_char;
              nfields_printed += 1
              /* shouldn't overflow.. */
            }
            spos += 1;
            if !(spos <= (*cut_lists.offset(cl_pos as isize)).endpos
              && !line.is_null()
              && (*cut_lists.offset(cl_pos as isize)).endpos != NON_RANGE as libc::c_int)
            {
              break;
            }
          }
          cl_pos = cl_pos.wrapping_add(1)
        }
        current_block = 7343950298149844727;
      }
    }
    match current_block {
      7343950298149844727 => {
        /* if we printed anything at all, we need to finish it with a
         * newline cuz we were handed a chomped line */
        putchar_unlocked('\n' as i32);
      }
      _ => {}
    }
    linenum = linenum.wrapping_add(1);
    free(printed as *mut libc::c_void);
    free(orig_line as *mut libc::c_void);
  }
}
#[no_mangle]
pub unsafe extern "C" fn cut_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  /* growable array holding a series of lists */
  let mut cut_lists: *mut cut_list = 0 as *mut cut_list; /* number of elements in above list */
  let mut nlists: libc::c_uint = 0i32 as libc::c_uint; /* delimiter, default is tab */
  let mut delim: libc::c_char = '\t' as i32 as libc::c_char;
  let mut sopt: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut ltok: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut opt: libc::c_uint = 0;
  opt = getopt32(
    argv,
    b"^b:c:f:d:sn\x00b--bcf:c--bcf:f--bcf\x00" as *const u8 as *const libc::c_char,
    &mut sopt as *mut *mut libc::c_char,
    &mut sopt as *mut *mut libc::c_char,
    &mut sopt as *mut *mut libc::c_char,
    &mut ltok as *mut *mut libc::c_char,
  );
  //	argc -= optind;
  argv = argv.offset(optind as isize);
  if opt & (1i32 << 0i32 | 1i32 << 1i32 | 1i32 << 2i32) as libc::c_uint == 0 {
    bb_simple_error_msg_and_die(
      b"expected a list of bytes, characters, or fields\x00" as *const u8 as *const libc::c_char,
    );
  }
  if opt & (1i32 << 3i32) as libc::c_uint != 0 {
    if *ltok.offset(0) as libc::c_int != 0 && *ltok.offset(1) as libc::c_int != 0 {
      /* more than 1 char? */
      bb_simple_error_msg_and_die(
        b"the delimiter must be a single character\x00" as *const u8 as *const libc::c_char,
      );
    }
    delim = *ltok.offset(0)
  }
  /*  non-field (char or byte) cutting has some special handling */
  if opt & (1i32 << 2i32) as libc::c_uint == 0 {
    static mut _op_on_field: [libc::c_char; 31] = [
      32, 111, 110, 108, 121, 32, 119, 104, 101, 110, 32, 111, 112, 101, 114, 97, 116, 105, 110,
      103, 32, 111, 110, 32, 102, 105, 101, 108, 100, 115, 0,
    ];
    if opt & (1i32 << 4i32) as libc::c_uint != 0 {
      bb_error_msg_and_die(
        b"suppressing non-delimited lines makes sense%s\x00" as *const u8 as *const libc::c_char,
        _op_on_field.as_ptr(),
      );
    }
    if delim as libc::c_int != '\t' as i32 {
      bb_error_msg_and_die(
        b"a delimiter may be specified%s\x00" as *const u8 as *const libc::c_char,
        _op_on_field.as_ptr(),
      );
    }
  }
  /*
   * parse list and put values into startpos and endpos.
   * valid list formats: N, N-, N-M, -M
   * more than one list can be separated by commas
   */
  let mut ntok: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut s: libc::c_int = 0i32;
  let mut e: libc::c_int = 0i32;
  loop
  /* take apart the lists, one by one (they are separated with commas) */
  {
    ltok = strsep(&mut sopt, b",\x00" as *const u8 as *const libc::c_char);
    if ltok.is_null() {
      break;
    }
    /* it's actually legal to pass an empty list */
    if *ltok.offset(0) == 0 {
      continue;
    }
    /* get the start pos */
    ntok = strsep(&mut ltok, b"-\x00" as *const u8 as *const libc::c_char);
    if *ntok.offset(0) == 0 {
      s = BOL as libc::c_int
    } else {
      s = xatoi_positive(ntok);
      /* account for the fact that arrays are zero based, while
       * the user expects the first char on the line to be char #1 */
      if s != 0i32 {
        s -= 1
      }
    }
    /* get the end pos */
    if ltok.is_null() {
      e = NON_RANGE as libc::c_int
    } else if *ltok.offset(0) == 0 {
      e = EOL as libc::c_int
    } else {
      e = xatoi_positive(ltok);
      /* if the user specified and end position of 0,
       * that means "til the end of the line" */
      if e == 0i32 {
        e = EOL as libc::c_int
      } /* again, arrays are zero based, lines are 1 based */
      e -= 1;
      if e == s {
        e = NON_RANGE as libc::c_int
      }
    }
    /* add the new list */
    cut_lists = xrealloc_vector_helper(
      cut_lists as *mut libc::c_void,
      ((::std::mem::size_of::<cut_list>() as libc::c_ulong) << 8i32)
        .wrapping_add(4i32 as libc::c_ulong) as libc::c_uint,
      nlists as libc::c_int,
    ) as *mut cut_list;
    /* NB: startpos is always >= 0,
     * while endpos may be = NON_RANGE (-1) */
    (*cut_lists.offset(nlists as isize)).startpos = s;
    (*cut_lists.offset(nlists as isize)).endpos = e;
    nlists = nlists.wrapping_add(1)
  }
  /* make sure we got some cut positions out of all that */
  if nlists == 0i32 as libc::c_uint {
    bb_simple_error_msg_and_die(
      b"missing list of positions\x00" as *const u8 as *const libc::c_char,
    );
  }
  /* now that the lists are parsed, we need to sort them to make life
   * easier on us when it comes time to print the chars / fields / lines
   */
  qsort(
    cut_lists as *mut libc::c_void,
    nlists as size_t,
    ::std::mem::size_of::<cut_list>() as libc::c_ulong,
    Some(
      cmpfunc
        as unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> libc::c_int,
    ),
  );
  let mut retval: libc::c_int = 0i32;
  if (*argv).is_null() {
    argv = argv.offset(-1);
    *argv = b"-\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
  }
  loop {
    let mut file: *mut FILE = fopen_or_warn_stdin(*argv);
    if file.is_null() {
      retval = 1i32
    } else {
      cut_file(file, delim, cut_lists, nlists);
      fclose_if_not_stdin(file);
    }
    argv = argv.offset(1);
    if (*argv).is_null() {
      break;
    }
  }
  fflush_stdout_and_exit(retval);
}
