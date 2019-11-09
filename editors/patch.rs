use libc;
use libc::close;
use libc::free;
extern "C" {
  #[no_mangle]
  fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn strtol(
    __nptr: *const libc::c_char,
    __endptr: *mut *mut libc::c_char,
    __base: libc::c_int,
  ) -> libc::c_long;

  #[no_mangle]
  static ptr_to_globals: *mut globals;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  static mut stdin: *mut FILE;
  #[no_mangle]
  fn rename(__old: *const libc::c_char, __new: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn dprintf(__fd: libc::c_int, __fmt: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
  #[no_mangle]
  fn fchmod(__fd: libc::c_int, __mode: mode_t) -> libc::c_int;
  #[no_mangle]
  fn xmalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_copyfd_eof(fd1: libc::c_int, fd2: libc::c_int) -> off_t;
  #[no_mangle]
  fn is_prefixed_with(string: *const libc::c_char, key: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xmove_fd(_: libc::c_int, _: libc::c_int);
  #[no_mangle]
  fn xunlink(pathname: *const libc::c_char);
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xopen_stdin(pathname: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn xmkstemp(template: *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  #[no_mangle]
  fn xmalloc_reads(fd: libc::c_int, maxsz_p: *mut size_t) -> *mut libc::c_char;
  #[no_mangle]
  fn xclose(fd: libc::c_int);
  #[no_mangle]
  fn xmalloc_fgetline(file: *mut FILE) -> *mut libc::c_char;
  #[no_mangle]
  fn xatoi(str: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  static mut option_mask32: u32;
  #[no_mangle]
  fn getopt32long(
    argv: *mut *mut libc::c_char,
    optstring: *const libc::c_char,
    longopts: *const libc::c_char,
    _: ...
  ) -> u32;
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_make_directory(
    path: *mut libc::c_char,
    mode: libc::c_long,
    flags: libc::c_int,
  ) -> libc::c_int;

  #[no_mangle]
  fn unlink(__name: *const libc::c_char) -> libc::c_int;
}

use libc::mode_t;

use libc::off_t;
use crate::librb::size_t;


use libc::stat;

use libc::FILE;
pub type C2RustUnnamed = libc::c_int;
pub const FILEUTILS_IGNORE_CHMOD_ERR: C2RustUnnamed = -2147483648;
pub const FILEUTILS_REFLINK_ALWAYS: C2RustUnnamed = 262144;
pub const FILEUTILS_REFLINK: C2RustUnnamed = 131072;
pub const FILEUTILS_RMDEST: C2RustUnnamed = 32768;
pub const FILEUTILS_NO_TARGET_DIR: C2RustUnnamed = 16384;
pub const FILEUTILS_UPDATE: C2RustUnnamed = 8192;
pub const FILEUTILS_VERBOSE: C2RustUnnamed = 4096;
pub const FILEUTILS_DEREFERENCE_L0: C2RustUnnamed = 256;
pub const FILEUTILS_DEREF_SOFTLINK: C2RustUnnamed = 128;
pub const FILEUTILS_MAKE_SOFTLINK: C2RustUnnamed = 64;
pub const FILEUTILS_MAKE_HARDLINK: C2RustUnnamed = 32;
pub const FILEUTILS_INTERACTIVE: C2RustUnnamed = 16;
pub const FILEUTILS_FORCE: C2RustUnnamed = 8;
pub const FILEUTILS_RECUR: C2RustUnnamed = 4;
pub const FILEUTILS_DEREFERENCE: C2RustUnnamed = 2;
pub const FILEUTILS_PRESERVE_STATUS: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub infile: *mut libc::c_char,
  pub prefix: libc::c_long,
  pub current_hunk: *mut double_list,
  pub oldline: libc::c_long,
  pub oldlen: libc::c_long,
  pub newline: libc::c_long,
  pub newlen: libc::c_long,
  pub linenum: libc::c_long,
  pub context: libc::c_int,
  pub state: libc::c_int,
  pub hunknum: libc::c_int,
  pub filein: libc::c_int,
  pub fileout: libc::c_int,
  pub tempname: *mut libc::c_char,
  pub exitval: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct double_list {
  pub next: *mut double_list,
  pub prev: *mut double_list,
  pub data: *mut libc::c_char,
}
#[inline(always)]
unsafe extern "C" fn not_const_pp(mut p: *const libc::c_void) -> *mut libc::c_void {
  return p as *mut libc::c_void;
}
unsafe extern "C" fn dlist_free(
  mut list: *mut double_list,
  mut freeit: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
) {
  while !list.is_null() {
    let mut pop: *mut libc::c_void = list as *mut libc::c_void;
    list = (*list).next;
    freeit.expect("non-null function pointer")(pop);
    if list == pop as *mut double_list {
      break;
    }
  }
}
unsafe extern "C" fn dlist_add(
  mut list: *mut *mut double_list,
  mut data: *mut libc::c_char,
) -> *mut double_list {
  let mut llist: *mut double_list = 0 as *mut double_list;
  let mut line: *mut double_list =
    xmalloc(::std::mem::size_of::<double_list>() as libc::c_ulong) as *mut double_list;
  (*line).data = data;
  llist = *list;
  if !llist.is_null() {
    let mut p: *mut double_list = 0 as *mut double_list;
    (*line).next = llist;
    (*line).prev = (*llist).prev;
    p = (*line).prev;
    (*p).next = line;
    (*llist).prev = line
  } else {
    (*line).prev = line;
    (*line).next = (*line).prev;
    *list = (*line).next
  }
  return line;
}
// Dispose of a line of input, either by writing it out or discarding it.
// state < 2: just free
// state = 2: write whole line to stderr
// state = 3: write whole line to fileout
// state > 3: write line+1 to fileout when *line != state
unsafe extern "C" fn do_line(mut data: *mut libc::c_void) {
  let mut dlist: *mut double_list = data as *mut double_list;
  if (*ptr_to_globals).state > 1i32 && *(*dlist).data as libc::c_int != (*ptr_to_globals).state {
    dprintf(
      if (*ptr_to_globals).state == 2i32 {
        2i32
      } else {
        (*ptr_to_globals).fileout
      },
      b"%s\n\x00" as *const u8 as *const libc::c_char,
      (*dlist).data.offset(
        (if (*ptr_to_globals).state > 3i32 {
          1i32
        } else {
          0i32
        }) as isize,
      ),
    );
  }
  free((*dlist).data as *mut libc::c_void);
  free(dlist as *mut libc::c_void);
}
unsafe extern "C" fn finish_oldfile() {
  if !(*ptr_to_globals).tempname.is_null() {
    // Copy the rest of the data and replace the original with the copy.
    let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*ptr_to_globals).filein != -1i32 {
      bb_copyfd_eof((*ptr_to_globals).filein, (*ptr_to_globals).fileout);
      xclose((*ptr_to_globals).filein);
    }
    xclose((*ptr_to_globals).fileout);
    if 1i32 == 0 || *(*ptr_to_globals).tempname.offset(0) as libc::c_int != 0 {
      /* not --dry-run? */
      temp = xstrdup((*ptr_to_globals).tempname);
      *temp.offset(strlen(temp).wrapping_sub(6i32 as libc::c_ulong) as isize) =
        '\u{0}' as i32 as libc::c_char;
      rename((*ptr_to_globals).tempname, temp);
      free(temp as *mut libc::c_void);
      free((*ptr_to_globals).tempname as *mut libc::c_void);
    }
    (*ptr_to_globals).tempname = 0 as *mut libc::c_char
  }
  (*ptr_to_globals).filein = -1i32;
  (*ptr_to_globals).fileout = (*ptr_to_globals).filein;
}
unsafe extern "C" fn fail_hunk() {
  if (*ptr_to_globals).current_hunk.is_null() {
    return;
  }
  dprintf(
    2i32,
    b"Hunk %d FAILED %ld/%ld.\n\x00" as *const u8 as *const libc::c_char,
    (*ptr_to_globals).hunknum,
    (*ptr_to_globals).oldline,
    (*ptr_to_globals).newline,
  );
  (*ptr_to_globals).exitval = 1i32;
  // If we got to this point, we've seeked to the end.  Discard changes to
  // this file and advance to next file.
  (*ptr_to_globals).state = 2i32;
  (*(*(*ptr_to_globals).current_hunk).prev).next = 0 as *mut double_list;
  dlist_free(
    (*ptr_to_globals).current_hunk,
    Some(do_line as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
  );
  (*ptr_to_globals).current_hunk = 0 as *mut double_list;
  // Abort the copy and delete the temporary file.
  close((*ptr_to_globals).filein);
  close((*ptr_to_globals).fileout);
  if 1i32 == 0 || *(*ptr_to_globals).tempname.offset(0) as libc::c_int != 0 {
    /* not --dry-run? */
    unlink((*ptr_to_globals).tempname);
    free((*ptr_to_globals).tempname as *mut libc::c_void);
  }
  (*ptr_to_globals).tempname = 0 as *mut libc::c_char;
  (*ptr_to_globals).state = 0i32;
}
// Given a hunk of a unified diff, make the appropriate change to the file.
// This does not use the location information, but instead treats a hunk
// as a sort of regex.  Copies data from input to output until it finds
// the change to be made, then outputs the changed data and returns.
// (Finding EOF first is an error.)  This is a single pass operation, so
// multiple hunks must occur in order in the file.
unsafe extern "C" fn apply_one_hunk() -> libc::c_int {
  let mut current_block: u64;
  let mut plist: *mut double_list = 0 as *mut double_list;
  let mut buf: *mut double_list = 0 as *mut double_list;
  let mut check: *mut double_list = 0 as *mut double_list;
  let mut matcheof: libc::c_int = 0i32;
  let mut reverse: libc::c_int = (option_mask32 & (1i32 << 0i32) as libc::c_uint) as libc::c_int;
  let mut backwarn: libc::c_int = 0i32;
  /* Do we try "dummy" revert to check whether
   * to silently skip this hunk? Used to implement -N.
   */
  let mut dummy_revert: libc::c_int = 0i32;
  // Break doubly linked list so we can use singly linked traversal function.
  (*(*(*ptr_to_globals).current_hunk).prev).next = 0 as *mut double_list;
  // Match EOF if there aren't as many ending context lines as beginning
  plist = (*ptr_to_globals).current_hunk;
  while !plist.is_null() {
    if *(*plist).data.offset(0) as libc::c_int == ' ' as i32 {
      matcheof += 1
    } else {
      matcheof = 0i32
    }
    plist = (*plist).next
  }
  matcheof = (matcheof == 0 || matcheof < (*ptr_to_globals).context) as libc::c_int;
  // Loop through input data searching for this hunk.  Match all context
  // lines and all lines to be removed until we've found the end of a
  // complete hunk.
  plist = (*ptr_to_globals).current_hunk;
  buf = 0 as *mut double_list;
  if if reverse != 0 {
    (*ptr_to_globals).oldlen
  } else {
    (*ptr_to_globals).newlen
  } != 0
  {
    current_block = 1054647088692577877;
  } else {
    current_block = 183457795823665471;
  }
  loop {
    match current_block {
      183457795823665471 => {
        // We have a match.  Emit changed data.
        (*ptr_to_globals).state =
          (*::std::mem::transmute::<&[u8; 3], &[libc::c_char; 3]>(b"-+\x00"))
            [(reverse ^ dummy_revert) as usize] as libc::c_int;
        dlist_free(
          (*ptr_to_globals).current_hunk,
          Some(do_line as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
        );
        (*ptr_to_globals).current_hunk = 0 as *mut double_list;
        (*ptr_to_globals).state = 1i32;
        break;
      }
      _ =>
      //FIXME: this performs 1-byte reads:
      {
        let mut data: *mut libc::c_char = xmalloc_reads((*ptr_to_globals).filein, 0 as *mut size_t);
        (*ptr_to_globals).linenum += 1;
        // Figure out which line of hunk to compare with next.  (Skip lines
        // of the hunk we'd be adding.)
        while !plist.is_null()
          && *(*plist).data as libc::c_int
            == (*::std::mem::transmute::<&[u8; 3], &[libc::c_char; 3]>(b"+-\x00"))[reverse as usize]
              as libc::c_int
        {
          if !data.is_null() && strcmp(data, (*plist).data.offset(1)) == 0i32 {
            if backwarn == 0 {
              backwarn = (*ptr_to_globals).linenum as libc::c_int;
              if option_mask32 & (1i32 << 4i32) as libc::c_uint != 0 {
                dummy_revert = 1i32;
                reverse ^= 1i32;
                continue;
              }
            }
          }
          plist = (*plist).next
        }
        // Is this EOF?
        if data.is_null() {
          // Does this hunk need to match EOF?
          if plist.is_null() && matcheof != 0 {
            current_block = 183457795823665471;
            continue;
          }
          if backwarn != 0 {
            dprintf(
              2i32,
              b"Possibly reversed hunk %d at %ld\n\x00" as *const u8 as *const libc::c_char,
              (*ptr_to_globals).hunknum,
              (*ptr_to_globals).linenum,
            );
          }
          // File ended before we found a place for this hunk.
          fail_hunk();
          break;
        } else {
          check = dlist_add(&mut buf, data);
          loop
          // Compare this line with next expected line of hunk.
          // todo: teach the strcmp() to ignore whitespace.
          // A match can fail because the next line doesn't match, or because
          // we hit the end of a hunk that needed EOF, and this isn't EOF.
          // If match failed, flush first line of buffered data and
          // recheck buffered data for a new match until we find one or run
          // out of buffer.
          {
            while !plist.is_null()
              && *(*plist).data as libc::c_int
                == (*::std::mem::transmute::<&[u8; 3], &[libc::c_char; 3]>(b"+-\x00"))
                  [reverse as usize] as libc::c_int
            {
              if strcmp((*check).data, (*plist).data.offset(1)) == 0i32 && backwarn == 0 {
                backwarn = (*ptr_to_globals).linenum as libc::c_int;
                if option_mask32 & (1i32 << 4i32) as libc::c_uint != 0 {
                  dummy_revert = 1i32;
                  reverse ^= 1i32
                }
              }
              plist = (*plist).next
            }
            if plist.is_null() || strcmp((*check).data, (*plist).data.offset(1)) != 0 {
              // Match failed.  Write out first line of buffered data and
              // recheck remaining buffered data for a new match.
              (*ptr_to_globals).state = 3i32;
              check = buf;
              buf = (*buf).next;
              (*(*check).prev).next = buf;
              (*buf).prev = (*check).prev;
              do_line(check as *mut libc::c_void);
              plist = (*ptr_to_globals).current_hunk;
              // If we've reached the end of the buffer without confirming a
              // match, read more lines.
              if check == buf {
                buf = 0 as *mut double_list;
                current_block = 1054647088692577877;
                break;
              } else {
                check = buf
              }
            } else {
              // This line matches.  Advance plist, detect successful match.
              plist = (*plist).next;
              if plist.is_null() && matcheof == 0 {
                current_block = 183457795823665471;
                break;
              }
              check = (*check).next;
              if check == buf {
                current_block = 1054647088692577877;
                break;
              }
            }
          }
        }
      }
    }
  }
  if !buf.is_null() {
    (*(*buf).prev).next = 0 as *mut double_list;
    dlist_free(
      buf,
      Some(do_line as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
    );
  }
  return (*ptr_to_globals).state;
}
// Read a patch file and find hunks, opening/creating/deleting files.
// Call apply_one_hunk() on each hunk.
// state 0: Not in a hunk, look for +++.
// state 1: Found +++ file indicator, look for @@
// state 2: In hunk: counting initial context lines
// state 3: In hunk: getting body
// Like GNU patch, we don't require a --- line before the +++, and
// also allow the --- after the +++ line.
#[no_mangle]
pub unsafe extern "C" fn patch_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut opts: libc::c_int = 0; /* for compiler */
  let mut reverse: libc::c_int = 0; /* for compiler */
  let mut state: libc::c_int = 0i32;
  let mut oldname: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut newname: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut opt_p: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut opt_i: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut oldlen: libc::c_long = 0;
  oldlen = oldlen;
  let mut newlen: libc::c_long = 0;
  newlen = newlen;
  static mut patch_longopts: [libc::c_char; 137] = [
    114, 101, 118, 101, 114, 115, 101, 0, 0, 82, 117, 110, 105, 102, 105, 101, 100, 0, 0, 117, 115,
    116, 114, 105, 112, 0, 1, 112, 105, 110, 112, 117, 116, 0, 1, 105, 102, 111, 114, 119, 97, 114,
    100, 0, 0, 78, 114, 101, 109, 111, 118, 101, 45, 101, 109, 112, 116, 121, 45, 102, 105, 108,
    101, 115, 0, 0, 69, 102, 111, 114, 99, 101, 0, 0, 102, 103, 101, 116, 0, 1, 103, 100, 114, 121,
    45, 114, 117, 110, 0, 0, -3, 98, 97, 99, 107, 117, 112, 45, 105, 102, 45, 109, 105, 115, 109,
    97, 116, 99, 104, 0, 0, -2, 110, 111, 45, 98, 97, 99, 107, 117, 112, 45, 105, 102, 45, 109,
    105, 115, 109, 97, 116, 99, 104, 0, 0, -1, 0,
  ];
  let ref mut fresh0 = *(not_const_pp(&ptr_to_globals as *const *mut globals as *const libc::c_void)
    as *mut *mut globals);
  *fresh0 = xzalloc(::std::mem::size_of::<globals>() as libc::c_ulong) as *mut globals;
  asm!("" : : : "memory" : "volatile");
  opts = getopt32long(
    argv,
    b"Rup:i:NEfg\x00" as *const u8 as *const libc::c_char,
    patch_longopts.as_ptr(),
    &mut opt_p as *mut *mut libc::c_char,
    &mut opt_i as *mut *mut libc::c_char,
  ) as libc::c_int;
  /*ignored*/
  //bb_error_msg_and_die("opts:%x", opts);
  argv = argv.offset(optind as isize); // can be negative!
  reverse = opts & 1i32 << 0i32;
  (*ptr_to_globals).prefix = if opts & 1i32 << 2i32 != 0 {
    xatoi(opt_p)
  } else {
    0i32
  } as libc::c_long;
  (*ptr_to_globals).fileout = -1i32;
  (*ptr_to_globals).filein = (*ptr_to_globals).fileout;
  if opts & 1i32 << 3i32 != 0 {
    xmove_fd(xopen_stdin(opt_i), 0i32);
  } else if !(*argv.offset(0)).is_null() && !(*argv.offset(1)).is_null() {
    xmove_fd(xopen_stdin(*argv.offset(1)), 0i32);
  }
  loop
  // Loop through the lines in the patch
  {
    let mut patchline: *mut libc::c_char = 0 as *mut libc::c_char;
    patchline = xmalloc_fgetline(stdin);
    if patchline.is_null() {
      break;
    }
    // Other versions of patch accept damaged patches,
    // so we need to also.
    if *patchline == 0 {
      free(patchline as *mut libc::c_void);
      patchline = xstrdup(b" \x00" as *const u8 as *const libc::c_char)
    }
    // Are we assembling a hunk?
    if state >= 2i32 {
      if *patchline as libc::c_int == ' ' as i32
        || *patchline as libc::c_int == '+' as i32
        || *patchline as libc::c_int == '-' as i32
      {
        dlist_add(&mut (*ptr_to_globals).current_hunk, patchline);
        if *patchline as libc::c_int != '+' as i32 {
          oldlen -= 1
        }
        if *patchline as libc::c_int != '-' as i32 {
          newlen -= 1
        }
        // Context line?
        if *patchline as libc::c_int == ' ' as i32 && state == 2i32 {
          (*ptr_to_globals).context += 1
        } else {
          state = 3i32
        }
        // If we've consumed all expected hunk lines, apply the hunk.
        if oldlen == 0 && newlen == 0 {
          state = apply_one_hunk()
        }
      } else {
        fail_hunk();
        state = 0i32
      }
    } else {
      // Open a new file?
      if !is_prefixed_with(patchline, b"--- \x00" as *const u8 as *const libc::c_char).is_null()
        || !is_prefixed_with(patchline, b"+++ \x00" as *const u8 as *const libc::c_char).is_null()
      {
        let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut name: *mut *mut libc::c_char = if reverse != 0 {
          &mut newname
        } else {
          &mut oldname
        };
        let mut i: libc::c_int = 0;
        if *patchline as libc::c_int == '+' as i32 {
          name = if reverse != 0 {
            &mut oldname
          } else {
            &mut newname
          };
          state = 1i32
        }
        finish_oldfile();
        if (*argv.offset(0)).is_null() {
          free(*name as *mut libc::c_void);
          // We defer actually opening the file because svn produces broken
          // patches that don't signal they want to create a new file the
          // way the patch man page says, so you have to read the first hunk
          // and _guess_.
          // Start a new hunk?  Usually @@ -oldline,oldlen +newline,newlen @@
          // but a missing ,value means the value is 1.
          // Trim date from end of filename (if any).  We don't care.
          s = patchline.offset(4);
          while *s as libc::c_int != 0 && *s as libc::c_int != '\t' as i32 {
            if *s as libc::c_int == '\\' as i32 && *s.offset(1) as libc::c_int != 0 {
              s = s.offset(1)
            }
            s = s.offset(1)
          }
          i = atoi(s);
          if i > 1900i32 && i <= 1970i32 {
            *name = xstrdup(b"/dev/null\x00" as *const u8 as *const libc::c_char)
          } else {
            *s = 0i32 as libc::c_char;
            *name = xstrdup(patchline.offset(4))
          }
        }
      } else if state == 1i32
        && !is_prefixed_with(patchline, b"@@ -\x00" as *const u8 as *const libc::c_char).is_null()
      {
        let mut i_0: libc::c_int = 0;
        let mut s_0: *mut libc::c_char = patchline.offset(4);
        // Read oldline[,oldlen] +newline[,newlen]
        newlen = 1i32 as libc::c_long;
        (*ptr_to_globals).newlen = newlen;
        oldlen = (*ptr_to_globals).newlen;
        (*ptr_to_globals).oldlen = oldlen;
        (*ptr_to_globals).oldline = strtol(s_0, &mut s_0, 10i32);
        if *s_0 as libc::c_int == ',' as i32 {
          oldlen = strtol(s_0.offset(1), &mut s_0, 10i32);
          (*ptr_to_globals).oldlen = oldlen
        }
        (*ptr_to_globals).newline = strtol(s_0.offset(2), &mut s_0, 10i32);
        if *s_0 as libc::c_int == ',' as i32 {
          newlen = strtol(s_0.offset(1), &mut s_0, 10i32);
          (*ptr_to_globals).newlen = newlen
        }
        if oldlen <1&& newlen <1{
          bb_error_msg_and_die(
            b"Really? %s\x00" as *const u8 as *const libc::c_char,
            patchline,
          );
        }
        (*ptr_to_globals).context = 0i32;
        state = 2i32;
        // If the --- line is missing or malformed, either oldname
        // or (for -R) newname could be NULL -- but not both.  Like
        // GNU patch, proceed based on the +++ line, and avoid SEGVs.
        if oldname.is_null() {
          oldname = xstrdup(b"MISSING_FILENAME\x00" as *const u8 as *const libc::c_char)
        }
        if newname.is_null() {
          newname = xstrdup(b"MISSING_FILENAME\x00" as *const u8 as *const libc::c_char)
        }
        // If this is the first hunk, open the file.
        if (*ptr_to_globals).filein == -1i32 {
          let mut oldsum: libc::c_int = 0;
          let mut newsum: libc::c_int = 0;
          let mut empty: libc::c_int = 0i32;
          let mut name_0: *mut libc::c_char = 0 as *mut libc::c_char;
          oldsum = ((*ptr_to_globals).oldline + oldlen) as libc::c_int;
          newsum = ((*ptr_to_globals).newline + newlen) as libc::c_int;
          name_0 = if reverse != 0 { oldname } else { newname };
          // We're deleting oldname if new file is /dev/null (before -p)
          // or if new hunk is empty (zero context) after patching
          if strcmp(name_0, b"/dev/null\x00" as *const u8 as *const libc::c_char) == 0i32
            || (if reverse != 0 { oldsum } else { newsum }) == 0
          {
            name_0 = if reverse != 0 { newname } else { oldname };
            empty = 1i32
          }
          // Handle -p path truncation.
          i_0 = 0i32;
          s_0 = name_0;
          while *s_0 != 0 {
            if option_mask32 & (1i32 << 2i32) as libc::c_uint != 0
              && (*ptr_to_globals).prefix == i_0 as libc::c_long
            {
              break;
            }
            let fresh1 = s_0;
            s_0 = s_0.offset(1);
            if *fresh1 as libc::c_int != '/' as i32 {
              continue;
            }
            while *s_0 as libc::c_int == '/' as i32 {
              s_0 = s_0.offset(1)
            }
            i_0 += 1;
            name_0 = s_0
          }
          // If "patch FILE_TO_PATCH", completely ignore name from patch
          if !(*argv.offset(0)).is_null() {
            name_0 = *argv.offset(0)
          }
          if empty != 0 {
            // File is empty after the patches have been applied
            state = 0i32;
            if option_mask32 & (1i32 << 5i32) as libc::c_uint != 0 {
              // If we've got a file to open, do so.
              // If flag -E or --remove-empty-files is set
              printf(
                b"removing %s\n\x00" as *const u8 as *const libc::c_char,
                name_0,
              );
              if opts & (1i32 << 8i32) * 1i32 == 0 {
                xunlink(name_0);
              }
            } else {
              printf(
                b"patching file %s\n\x00" as *const u8 as *const libc::c_char,
                name_0,
              );
              if opts & (1i32 << 8i32) * 1i32 == 0 {
                xclose(xopen(name_0, 0o1i32 | 0o1000i32));
              }
            }
          } else if option_mask32 & (1i32 << 2i32) as libc::c_uint == 0
            || i_0 as libc::c_long <= (*ptr_to_globals).prefix
          {
            let mut statbuf: stat = std::mem::zeroed();
            // If the old file was null, we're creating a new one.
            if strcmp(
              oldname,
              b"/dev/null\x00" as *const u8 as *const libc::c_char,
            ) == 0i32
              || oldsum == 0
            {
              printf(
                b"creating %s\n\x00" as *const u8 as *const libc::c_char,
                name_0,
              );
              if opts & (1i32 << 8i32) * 1i32 == 0 {
                s_0 = strrchr(name_0, '/' as i32);
                if !s_0.is_null() {
                  *s_0 = '\u{0}' as i32 as libc::c_char;
                  bb_make_directory(
                    name_0,
                    -1i32 as libc::c_long,
                    FILEUTILS_RECUR as libc::c_int,
                  );
                  *s_0 = '/' as i32 as libc::c_char
                }
                (*ptr_to_globals).filein = xopen(name_0, 0o100i32 | 0o200i32 | 0o2i32)
              } else {
                (*ptr_to_globals).filein =
                  xopen(b"/dev/null\x00" as *const u8 as *const libc::c_char, 0i32)
              }
            } else {
              printf(
                b"patching file %s\n\x00" as *const u8 as *const libc::c_char,
                name_0,
              );
              (*ptr_to_globals).filein = xopen(name_0, 0i32)
            }
            if opts & (1i32 << 8i32) * 1i32 == 0 {
              (*ptr_to_globals).tempname =
                xasprintf(b"%sXXXXXX\x00" as *const u8 as *const libc::c_char, name_0);
              (*ptr_to_globals).fileout = xmkstemp((*ptr_to_globals).tempname);
              // Set permissions of output file
              fstat((*ptr_to_globals).filein, &mut statbuf);
              fchmod((*ptr_to_globals).fileout, statbuf.st_mode);
            } else {
              (*ptr_to_globals).tempname =
                b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
              (*ptr_to_globals).fileout =
                xopen(b"/dev/null\x00" as *const u8 as *const libc::c_char, 0o1i32)
            }
            (*ptr_to_globals).linenum = 0;
            (*ptr_to_globals).hunknum = 0i32
          }
        }
        (*ptr_to_globals).hunknum += 1;
        continue;
      }
      // If we didn't continue above, discard this line.
      free(patchline as *mut libc::c_void);
    }
  }
  finish_oldfile();
  return (*ptr_to_globals).exitval;
}
