use crate::librb::size_t;
use crate::librb::smallint;
use libc;
use libc::mode_t;
use libc::ssize_t;
use libc::FILE;

extern "C" {
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);

  #[no_mangle]
  fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;

  #[no_mangle]
  fn creat(__file: *const libc::c_char, __mode: mode_t) -> libc::c_int;

  #[no_mangle]
  static ptr_to_globals: *mut globals;

  #[no_mangle]
  static mut stdout: *mut FILE;

  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;

  #[no_mangle]
  fn puts(__s: *const libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;

  #[no_mangle]
  fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;

  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;

  #[no_mangle]
  fn memchr(_: *const libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;

  #[no_mangle]
  fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;

  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;

  #[no_mangle]
  fn skip_whitespace(_: *const libc::c_char) -> *mut libc::c_char;

  #[no_mangle]
  fn xmalloc(size: size_t) -> *mut libc::c_void;

  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;

  #[no_mangle]
  fn xrealloc(old: *mut libc::c_void, size: size_t) -> *mut libc::c_void;

  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;

  #[no_mangle]
  fn fputc_printable(ch: libc::c_int, file: *mut FILE);

  #[no_mangle]
  fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;

  #[no_mangle]
  fn full_write(fd: libc::c_int, buf: *const libc::c_void, count: size_t) -> ssize_t;

  #[no_mangle]
  fn fflush_all() -> libc::c_int;

  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);

  #[no_mangle]
  fn bb_simple_error_msg(s: *const libc::c_char);

  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);

  #[no_mangle]
  fn read_line_input(
    st: *mut line_input_t,
    prompt: *const libc::c_char,
    command: *mut libc::c_char,
    maxsize: libc::c_int,
  ) -> libc::c_int;

  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
}

pub type C2RustUnnamed = libc::c_uint;
pub const PRINTABLE_META: C2RustUnnamed = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct line_input_t {
  pub flags: libc::c_int,
  pub timeout: libc::c_int,
  pub path_lookup: *const libc::c_char,
  pub cnt_history: libc::c_int,
  pub cur_history: libc::c_int,
  pub max_history: libc::c_int,
  pub cnt_history_in_file: libc::c_uint,
  pub hist_file: *const libc::c_char,
  pub history: [*mut libc::c_char; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub curNum: libc::c_int,
  pub lastNum: libc::c_int,
  pub bufUsed: libc::c_int,
  pub bufSize: libc::c_int,
  pub curLine: *mut LINE,
  pub bufBase: *mut libc::c_char,
  pub bufPtr: *mut libc::c_char,
  pub fileName: *mut libc::c_char,
  pub lines: LINE,
  pub dirty: smallint,
  pub marks: [libc::c_int; 26],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LINE {
  pub next: *mut LINE,
  pub prev: *mut LINE,
  pub len: libc::c_int,
  pub data: [libc::c_char; 1],
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const INITBUF_SIZE: C2RustUnnamed_0 = 1024;
pub const USERSIZE: C2RustUnnamed_0 = 1023;
#[inline(always)]
unsafe extern "C" fn not_const_pp(mut p: *const libc::c_void) -> *mut libc::c_void {
  return p as *mut libc::c_void;
}
unsafe extern "C" fn bad_nums(
  mut num1: libc::c_int,
  mut num2: libc::c_int,
  mut for_what: *const libc::c_char,
) -> libc::c_int {
  if num1 < 1i32 || num2 > (*ptr_to_globals).lastNum || num1 > num2 {
    bb_error_msg(
      b"bad line range for %s\x00" as *const u8 as *const libc::c_char,
      for_what,
    );
    return 1i32;
  }
  return 0i32;
}
/*
 * Return a pointer to the specified line number.
 */
unsafe extern "C" fn findLine(mut num: libc::c_int) -> *mut LINE {
  let mut lp: *mut LINE = 0 as *mut LINE;
  let mut lnum: libc::c_int = 0;
  if num < 1i32 || num > (*ptr_to_globals).lastNum {
    bb_error_msg(
      b"line number %d does not exist\x00" as *const u8 as *const libc::c_char,
      num,
    );
    return 0 as *mut LINE;
  }
  if (*ptr_to_globals).curNum <= 0i32 {
    (*ptr_to_globals).curNum = 1i32;
    (*ptr_to_globals).curLine = (*ptr_to_globals).lines.next
  }
  if num == (*ptr_to_globals).curNum {
    return (*ptr_to_globals).curLine;
  }
  lp = (*ptr_to_globals).curLine;
  lnum = (*ptr_to_globals).curNum;
  if num < (*ptr_to_globals).curNum / 2i32 {
    lp = (*ptr_to_globals).lines.next;
    lnum = 1i32
  } else if num > ((*ptr_to_globals).curNum + (*ptr_to_globals).lastNum) / 2i32 {
    lp = (*ptr_to_globals).lines.prev;
    lnum = (*ptr_to_globals).lastNum
  }
  while lnum < num {
    lp = (*lp).next;
    lnum += 1
  }
  while lnum > num {
    lp = (*lp).prev;
    lnum -= 1
  }
  return lp;
}
/*
 * Search a line for the specified string starting at the specified
 * offset in the line.  Returns the offset of the found string, or -1.
 */
unsafe extern "C" fn findString(
  mut lp: *const LINE,
  mut str: *const libc::c_char,
  mut len: libc::c_int,
  mut offset: libc::c_int,
) -> libc::c_int {
  let mut left: libc::c_int = 0;
  let mut cp: *const libc::c_char = 0 as *const libc::c_char;
  let mut ncp: *const libc::c_char = 0 as *const libc::c_char;
  cp = &*(*lp).data.as_ptr().offset(offset as isize) as *const libc::c_char;
  left = (*lp).len - offset - len;
  while left >= 0i32 {
    ncp = memchr(
      cp as *const libc::c_void,
      *str.offset(0) as libc::c_int,
      (left + 1i32) as libc::c_ulong,
    ) as *const libc::c_char;
    if ncp.is_null() {
      return -1i32;
    }
    left = (left as libc::c_long - ncp.wrapping_offset_from(cp) as libc::c_long) as libc::c_int;
    cp = ncp;
    if memcmp(
      cp as *const libc::c_void,
      str as *const libc::c_void,
      len as libc::c_ulong,
    ) == 0i32
    {
      return cp.wrapping_offset_from((*lp).data.as_ptr()) as libc::c_long as libc::c_int;
    }
    cp = cp.offset(1);
    left -= 1
  }
  return -1i32;
}
/*
 * Search for a line which contains the specified string.
 * If the string is "", then the previously searched for string
 * is used.  The currently searched for string is saved for future use.
 * Returns the line number which matches, or 0 if there was no match
 * with an error printed.
 */
#[inline(never)]
unsafe extern "C" fn searchLines(
  mut str: *const libc::c_char,
  mut num1: libc::c_int,
  mut num2: libc::c_int,
) -> libc::c_int {
  let mut lp: *const LINE = 0 as *const LINE;
  let mut len: libc::c_int = 0;
  if bad_nums(
    num1,
    num2,
    b"search\x00" as *const u8 as *const libc::c_char,
  ) != 0
  {
    return 0i32;
  }
  if *str as libc::c_int == '\u{0}' as i32 {
    if *bb_common_bufsiz1.as_mut_ptr().offset(0) as libc::c_int == '\u{0}' as i32 {
      bb_simple_error_msg(b"no previous search string\x00" as *const u8 as *const libc::c_char);
      return 0i32;
    }
    str = bb_common_bufsiz1.as_mut_ptr()
  }
  if str != bb_common_bufsiz1.as_mut_ptr() {
    strcpy(bb_common_bufsiz1.as_mut_ptr(), str);
  }
  len = strlen(str) as libc::c_int;
  lp = findLine(num1);
  if lp.is_null() {
    return 0i32;
  }
  while num1 <= num2 {
    if findString(lp, str, len, 0i32) >= 0i32 {
      return num1;
    }
    num1 += 1;
    lp = (*lp).next
  }
  bb_error_msg(
    b"can\'t find string \"%s\"\x00" as *const u8 as *const libc::c_char,
    str,
  );
  return 0i32;
}
/*
 * Parse a line number argument if it is present.  This is a sum
 * or difference of numbers, ".", "$", "'c", or a search string.
 * Returns pointer which stopped the scan if successful
 * (whether or not there was a number).
 * Returns NULL if there was a parsing error, with a message output.
 * Whether there was a number is returned indirectly, as is the number.
 */
unsafe extern "C" fn getNum(
  mut cp: *const libc::c_char,
  mut retHaveNum: *mut smallint,
  mut retNum: *mut libc::c_int,
) -> *const libc::c_char {
  let mut endStr: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut str: [libc::c_char; 1023] = [0; 1023];
  let mut value: libc::c_int = 0;
  let mut num: libc::c_int = 0;
  let mut haveNum: smallint = 0;
  let mut minus: smallint = 0;
  value = 0i32;
  haveNum = 0i32 as smallint;
  minus = 0i32 as smallint;
  while 1i32 != 0 {
    cp = skip_whitespace(cp);
    match *cp as libc::c_int {
      46 => {
        haveNum = 1i32 as smallint;
        num = (*ptr_to_globals).curNum;
        cp = cp.offset(1)
      }
      36 => {
        haveNum = 1i32 as smallint;
        num = (*ptr_to_globals).lastNum;
        cp = cp.offset(1)
      }
      39 => {
        cp = cp.offset(1);
        if (*cp as libc::c_int - 'a' as i32) as libc::c_uint >= 26i32 as libc::c_uint {
          bb_simple_error_msg(b"bad mark name\x00" as *const u8 as *const libc::c_char);
          return 0 as *const libc::c_char;
        }
        haveNum = 1i32 as smallint;
        num = (*ptr_to_globals).marks[(*cp as libc::c_int - 'a' as i32) as libc::c_uint as usize];
        cp = cp.offset(1)
      }
      47 => {
        cp = cp.offset(1);
        strcpy(str.as_mut_ptr(), cp);
        endStr = strchr(str.as_mut_ptr(), '/' as i32);
        if !endStr.is_null() {
          let fresh0 = endStr;
          endStr = endStr.offset(1);
          *fresh0 = '\u{0}' as i32 as libc::c_char;
          cp = cp.offset(endStr.wrapping_offset_from(str.as_mut_ptr()) as libc::c_long as isize)
        } else {
          cp = b"\x00" as *const u8 as *const libc::c_char
        }
        num = searchLines(
          str.as_mut_ptr(),
          (*ptr_to_globals).curNum,
          (*ptr_to_globals).lastNum,
        );
        if num == 0i32 {
          return 0 as *const libc::c_char;
        }
        haveNum = 1i32 as smallint
      }
      _ => {
        if !((*cp as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32) {
          *retHaveNum = haveNum;
          *retNum = value;
          return cp;
        }
        num = 0i32;
        while (*cp as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32 {
          let fresh1 = cp;
          cp = cp.offset(1);
          num = num * 10i32 + *fresh1 as libc::c_int - '0' as i32
        }
        haveNum = 1i32 as smallint
      }
    }
    value += if minus as libc::c_int != 0 { -num } else { num };
    cp = skip_whitespace(cp);
    match *cp as libc::c_int {
      45 => {
        minus = 1i32 as smallint;
        cp = cp.offset(1)
      }
      43 => {
        minus = 0i32 as smallint;
        cp = cp.offset(1)
      }
      _ => {
        *retHaveNum = haveNum;
        *retNum = value;
        return cp;
      }
    }
  }
  panic!("Reached end of non-void function without returning");
}
/*
 * Set the current line number.
 * Returns TRUE if successful.
 */
unsafe extern "C" fn setCurNum(mut num: libc::c_int) -> libc::c_int {
  let mut lp: *mut LINE = 0 as *mut LINE;
  lp = findLine(num);
  if lp.is_null() {
    return 0i32;
  }
  (*ptr_to_globals).curNum = num;
  (*ptr_to_globals).curLine = lp;
  return 1i32;
}
/*
 * Insert a new line with the specified text.
 * The line is inserted so as to become the specified line,
 * thus pushing any existing and further lines down one.
 * The inserted line is also set to become the current line.
 * Returns TRUE if successful.
 */
unsafe extern "C" fn insertLine(
  mut num: libc::c_int,
  mut data: *const libc::c_char,
  mut len: libc::c_int,
) -> libc::c_int {
  let mut newLp: *mut LINE = 0 as *mut LINE;
  let mut lp: *mut LINE = 0 as *mut LINE;
  if num < 1i32 || num > (*ptr_to_globals).lastNum + 1i32 {
    bb_simple_error_msg(b"inserting at bad line number\x00" as *const u8 as *const libc::c_char);
    return 0i32;
  }
  newLp = xmalloc(
    (::std::mem::size_of::<LINE>() as libc::c_ulong)
      .wrapping_add(len as libc::c_ulong)
      .wrapping_sub(1i32 as libc::c_ulong),
  ) as *mut LINE;
  memcpy(
    (*newLp).data.as_mut_ptr() as *mut libc::c_void,
    data as *const libc::c_void,
    len as libc::c_ulong,
  );
  (*newLp).len = len;
  if num > (*ptr_to_globals).lastNum {
    lp = &mut (*ptr_to_globals).lines
  } else {
    lp = findLine(num);
    if lp.is_null() {
      free(newLp as *mut libc::c_char as *mut libc::c_void);
      return 0i32;
    }
  }
  (*newLp).next = lp;
  (*newLp).prev = (*lp).prev;
  (*(*lp).prev).next = newLp;
  (*lp).prev = newLp;
  (*ptr_to_globals).lastNum += 1;
  (*ptr_to_globals).dirty = 1i32 as smallint;
  return setCurNum(num);
}
/*
 * Add lines which are typed in by the user.
 * The lines are inserted just before the specified line number.
 * The lines are terminated by a line containing a single dot (ugly!),
 * or by an end of file.
 */
unsafe extern "C" fn addLines(mut num: libc::c_int) {
  let mut len: libc::c_int = 0;
  let mut buf: [libc::c_char; 1024] = [0; 1024];
  loop {
    /* Returns:
     * -1 on read errors or EOF, or on bare Ctrl-D.
     * 0  on ctrl-C,
     * >0 length of input string, including terminating '\n'
     */
    len = read_line_input(
      0 as *mut line_input_t,
      b"\x00" as *const u8 as *const libc::c_char,
      buf.as_mut_ptr(),
      ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    if len <= 0i32 {
      /* Previously, ctrl-C was exiting to shell.
       * Now we exit to ed prompt. Is in important? */
      return;
    }
    if buf[0] as libc::c_int == '.' as i32
      && buf[1] as libc::c_int == '\n' as i32
      && buf[2] as libc::c_int == '\u{0}' as i32
    {
      return;
    }
    let fresh2 = num;
    num = num + 1;
    if insertLine(fresh2, buf.as_mut_ptr(), len) == 0 {
      return;
    }
  }
}
/*
 * Read lines from a file at the specified line number.
 * Returns TRUE if the file was successfully read.
 */
unsafe extern "C" fn readLines(mut file: *const libc::c_char, mut num: libc::c_int) -> libc::c_int {
  let mut fd: libc::c_int = 0;
  let mut cc: libc::c_int = 0;
  let mut len: libc::c_int = 0;
  let mut lineCount: libc::c_int = 0;
  let mut charCount: libc::c_int = 0;
  let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
  if num < 1i32 || num > (*ptr_to_globals).lastNum + 1i32 {
    bb_simple_error_msg(b"bad line for read\x00" as *const u8 as *const libc::c_char);
    return 0i32;
  }
  fd = open(file, 0i32);
  if fd < 0i32 {
    bb_simple_perror_msg(file);
    return 0i32;
  }
  (*ptr_to_globals).bufPtr = (*ptr_to_globals).bufBase;
  (*ptr_to_globals).bufUsed = 0i32;
  lineCount = 0i32;
  charCount = 0i32;
  cc = 0i32;
  printf(b"\"%s\", \x00" as *const u8 as *const libc::c_char, file);
  fflush_all();
  loop {
    cp = memchr(
      (*ptr_to_globals).bufPtr as *const libc::c_void,
      '\n' as i32,
      (*ptr_to_globals).bufUsed as libc::c_ulong,
    ) as *mut libc::c_char;
    if !cp.is_null() {
      len = (cp.wrapping_offset_from((*ptr_to_globals).bufPtr) as libc::c_long + 1) as libc::c_int;
      if insertLine(num, (*ptr_to_globals).bufPtr, len) == 0 {
        close(fd);
        return 0i32;
      }
      (*ptr_to_globals).bufPtr = (*ptr_to_globals).bufPtr.offset(len as isize);
      (*ptr_to_globals).bufUsed -= len;
      charCount += len;
      lineCount += 1;
      num += 1
    } else {
      if (*ptr_to_globals).bufPtr != (*ptr_to_globals).bufBase {
        memcpy(
          (*ptr_to_globals).bufBase as *mut libc::c_void,
          (*ptr_to_globals).bufPtr as *const libc::c_void,
          (*ptr_to_globals).bufUsed as libc::c_ulong,
        );
        (*ptr_to_globals).bufPtr = (*ptr_to_globals)
          .bufBase
          .offset((*ptr_to_globals).bufUsed as isize)
      }
      if (*ptr_to_globals).bufUsed >= (*ptr_to_globals).bufSize {
        len = (*ptr_to_globals).bufSize * 3i32 / 2i32;
        cp = xrealloc(
          (*ptr_to_globals).bufBase as *mut libc::c_void,
          len as size_t,
        ) as *mut libc::c_char;
        (*ptr_to_globals).bufBase = cp;
        (*ptr_to_globals).bufPtr = (*ptr_to_globals)
          .bufBase
          .offset((*ptr_to_globals).bufUsed as isize);
        (*ptr_to_globals).bufSize = len
      }
      cc = safe_read(
        fd,
        (*ptr_to_globals).bufPtr as *mut libc::c_void,
        ((*ptr_to_globals).bufSize - (*ptr_to_globals).bufUsed) as size_t,
      ) as libc::c_int;
      (*ptr_to_globals).bufUsed += cc;
      (*ptr_to_globals).bufPtr = (*ptr_to_globals).bufBase
    }
    if !(cc > 0i32) {
      break;
    }
  }
  if cc < 0i32 {
    bb_simple_perror_msg(file);
    close(fd);
    return 0i32;
  }
  if (*ptr_to_globals).bufUsed != 0 {
    if insertLine(num, (*ptr_to_globals).bufPtr, (*ptr_to_globals).bufUsed) == 0 {
      close(fd);
      return -1i32;
    }
    lineCount += 1;
    charCount += (*ptr_to_globals).bufUsed
  }
  close(fd);
  printf(
    b"%d lines%s, %d chars\n\x00" as *const u8 as *const libc::c_char,
    lineCount,
    if (*ptr_to_globals).bufUsed != 0 {
      b" (incomplete)\x00" as *const u8 as *const libc::c_char
    } else {
      b"\x00" as *const u8 as *const libc::c_char
    },
    charCount,
  );
  return 1i32;
}
/*
 * Write the specified lines out to the specified file.
 * Returns TRUE if successful, or FALSE on an error with a message output.
 */
unsafe extern "C" fn writeLines(
  mut file: *const libc::c_char,
  mut num1: libc::c_int,
  mut num2: libc::c_int,
) -> libc::c_int {
  let mut lp: *mut LINE = 0 as *mut LINE;
  let mut fd: libc::c_int = 0;
  let mut lineCount: libc::c_int = 0;
  let mut charCount: libc::c_int = 0;
  if bad_nums(num1, num2, b"write\x00" as *const u8 as *const libc::c_char) != 0 {
    return 0i32;
  }
  lineCount = 0i32;
  charCount = 0i32;
  fd = creat(file, 0o666i32 as mode_t);
  if fd < 0i32 {
    bb_simple_perror_msg(file);
    return 0i32;
  }
  printf(b"\"%s\", \x00" as *const u8 as *const libc::c_char, file);
  fflush_all();
  lp = findLine(num1);
  if lp.is_null() {
    close(fd);
    return 0i32;
  }
  loop {
    let fresh3 = num1;
    num1 = num1 + 1;
    if !(fresh3 <= num2) {
      break;
    }
    if full_write(
      fd,
      (*lp).data.as_mut_ptr() as *const libc::c_void,
      (*lp).len as size_t,
    ) != (*lp).len as isize
    {
      bb_simple_perror_msg(file);
      close(fd);
      return 0i32;
    }
    charCount += (*lp).len;
    lineCount += 1;
    lp = (*lp).next
  }
  if close(fd) < 0i32 {
    bb_simple_perror_msg(file);
    return 0i32;
  }
  printf(
    b"%d lines, %d chars\n\x00" as *const u8 as *const libc::c_char,
    lineCount,
    charCount,
  );
  return 1i32;
}
/*
 * Print lines in a specified range.
 * The last line printed becomes the current line.
 * If expandFlag is TRUE, then the line is printed specially to
 * show magic characters.
 */
unsafe extern "C" fn printLines(
  mut num1: libc::c_int,
  mut num2: libc::c_int,
  mut expandFlag: libc::c_int,
) -> libc::c_int {
  let mut lp: *const LINE = 0 as *const LINE;
  let mut cp: *const libc::c_char = 0 as *const libc::c_char;
  let mut ch: libc::c_int = 0;
  let mut count: libc::c_int = 0;
  if bad_nums(num1, num2, b"print\x00" as *const u8 as *const libc::c_char) != 0 {
    return 0i32;
  }
  lp = findLine(num1);
  if lp.is_null() {
    return 0i32;
  }
  while num1 <= num2 {
    if expandFlag == 0 {
      write(
        1i32,
        (*lp).data.as_ptr() as *const libc::c_void,
        (*lp).len as size_t,
      );
      let fresh4 = num1;
      num1 = num1 + 1;
      setCurNum(fresh4);
      lp = (*lp).next
    } else {
      /*
       * Show control characters and characters with the
       * high bit set specially.
       */
      cp = (*lp).data.as_ptr();
      count = (*lp).len;
      if count > 0i32 && *cp.offset((count - 1i32) as isize) as libc::c_int == '\n' as i32 {
        count -= 1
      }
      loop {
        let fresh5 = count;
        count = count - 1;
        if !(fresh5 > 0i32) {
          break;
        }
        let fresh6 = cp;
        cp = cp.offset(1);
        ch = *fresh6 as libc::c_uchar as libc::c_int;
        fputc_printable(ch | PRINTABLE_META as libc::c_int, stdout);
      }
      fputs_unlocked(b"$\n\x00" as *const u8 as *const libc::c_char, stdout);
      let fresh7 = num1;
      num1 = num1 + 1;
      setCurNum(fresh7);
      lp = (*lp).next
    }
  }
  return 1i32;
}
/*
 * Delete lines from the given range.
 */
unsafe extern "C" fn deleteLines(mut num1: libc::c_int, mut num2: libc::c_int) {
  let mut lp: *mut LINE = 0 as *mut LINE;
  let mut nlp: *mut LINE = 0 as *mut LINE;
  let mut plp: *mut LINE = 0 as *mut LINE;
  let mut count: libc::c_int = 0;
  if bad_nums(
    num1,
    num2,
    b"delete\x00" as *const u8 as *const libc::c_char,
  ) != 0
  {
    return;
  }
  lp = findLine(num1);
  if lp.is_null() {
    return;
  }
  if (*ptr_to_globals).curNum >= num1 && (*ptr_to_globals).curNum <= num2 {
    if num2 < (*ptr_to_globals).lastNum {
      setCurNum(num2 + 1i32);
    } else if num1 > 1i32 {
      setCurNum(num1 - 1i32);
    } else {
      (*ptr_to_globals).curNum = 0i32
    }
  }
  count = num2 - num1 + 1i32;
  if (*ptr_to_globals).curNum > num2 {
    (*ptr_to_globals).curNum -= count
  }
  (*ptr_to_globals).lastNum -= count;
  loop {
    let fresh8 = count;
    count = count - 1;
    if !(fresh8 > 0i32) {
      break;
    }
    nlp = (*lp).next;
    plp = (*lp).prev;
    (*plp).next = nlp;
    (*nlp).prev = plp;
    free(lp as *mut libc::c_void);
    lp = nlp
  }
  (*ptr_to_globals).dirty = 1i32 as smallint;
}
/*
 * Do the substitute command.
 * The current line is set to the last substitution done.
 */
unsafe extern "C" fn subCommand(
  mut cmd: *const libc::c_char,
  mut num1: libc::c_int,
  mut num2: libc::c_int,
) {
  let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut oldStr: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut newStr: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut buf: [libc::c_char; 1023] = [0; 1023];
  let mut delim: libc::c_int = 0;
  let mut oldLen: libc::c_int = 0;
  let mut newLen: libc::c_int = 0;
  let mut deltaLen: libc::c_int = 0;
  let mut offset: libc::c_int = 0;
  let mut lp: *mut LINE = 0 as *mut LINE;
  let mut nlp: *mut LINE = 0 as *mut LINE;
  let mut globalFlag: libc::c_int = 0;
  let mut printFlag: libc::c_int = 0;
  let mut didSub: libc::c_int = 0;
  let mut needPrint: libc::c_int = 0;
  if bad_nums(
    num1,
    num2,
    b"substitute\x00" as *const u8 as *const libc::c_char,
  ) != 0
  {
    return;
  }
  globalFlag = 0i32;
  printFlag = 0i32;
  didSub = 0i32;
  needPrint = 0i32;
  /*
   * Copy the command so we can modify it.
   */
  strcpy(buf.as_mut_ptr(), cmd);
  cp = buf.as_mut_ptr();
  if ({
    let mut bb__isblank: libc::c_uchar = *cp as libc::c_uchar;
    (bb__isblank as libc::c_int == ' ' as i32 || bb__isblank as libc::c_int == '\t' as i32)
      as libc::c_int
  }) != 0
    || *cp as libc::c_int == '\u{0}' as i32
  {
    bb_simple_error_msg(b"bad delimiter for substitute\x00" as *const u8 as *const libc::c_char);
    return;
  }
  let fresh9 = cp;
  cp = cp.offset(1);
  delim = *fresh9 as libc::c_int;
  oldStr = cp;
  cp = strchr(cp, delim);
  if cp.is_null() {
    bb_simple_error_msg(
      b"missing 2nd delimiter for substitute\x00" as *const u8 as *const libc::c_char,
    );
    return;
  }
  let fresh10 = cp;
  cp = cp.offset(1);
  *fresh10 = '\u{0}' as i32 as libc::c_char;
  newStr = cp;
  cp = strchr(cp, delim);
  if !cp.is_null() {
    let fresh11 = cp;
    cp = cp.offset(1);
    *fresh11 = '\u{0}' as i32 as libc::c_char
  } else {
    cp = b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
  }
  while *cp != 0 {
    let fresh12 = cp;
    cp = cp.offset(1);
    match *fresh12 as libc::c_int {
      103 => globalFlag = 1i32,
      112 => printFlag = 1i32,
      _ => {
        bb_simple_error_msg(
          b"unknown option for substitute\x00" as *const u8 as *const libc::c_char,
        );
        return;
      }
    }
  }
  if *oldStr as libc::c_int == '\u{0}' as i32 {
    if *bb_common_bufsiz1.as_mut_ptr().offset(0) as libc::c_int == '\u{0}' as i32 {
      bb_simple_error_msg(b"no previous search string\x00" as *const u8 as *const libc::c_char);
      return;
    }
    oldStr = bb_common_bufsiz1.as_mut_ptr()
  }
  if oldStr != bb_common_bufsiz1.as_mut_ptr() {
    strcpy(bb_common_bufsiz1.as_mut_ptr(), oldStr);
  }
  lp = findLine(num1);
  if lp.is_null() {
    return;
  }
  oldLen = strlen(oldStr) as libc::c_int;
  newLen = strlen(newStr) as libc::c_int;
  deltaLen = newLen - oldLen;
  offset = 0i32;
  nlp = 0 as *mut LINE;
  while num1 <= num2 {
    offset = findString(lp, oldStr, oldLen, offset);
    if offset < 0i32 {
      if needPrint != 0 {
        printLines(num1, num1, 0i32);
        needPrint = 0i32
      }
      offset = 0i32;
      lp = (*lp).next;
      num1 += 1
    } else {
      needPrint = printFlag;
      didSub = 1i32;
      (*ptr_to_globals).dirty = 1i32 as smallint;
      /*
       * If the replacement string is the same size or shorter
       * than the old string, then the substitution is easy.
       */
      if deltaLen <= 0i32 {
        memcpy(
          &mut *(*lp).data.as_mut_ptr().offset(offset as isize) as *mut libc::c_char
            as *mut libc::c_void,
          newStr as *const libc::c_void,
          newLen as libc::c_ulong,
        );
        if deltaLen != 0 {
          memcpy(
            &mut *(*lp).data.as_mut_ptr().offset((offset + newLen) as isize) as *mut libc::c_char
              as *mut libc::c_void,
            &mut *(*lp).data.as_mut_ptr().offset((offset + oldLen) as isize) as *mut libc::c_char
              as *const libc::c_void,
            ((*lp).len - offset - oldLen) as libc::c_ulong,
          );
          (*lp).len += deltaLen
        }
        offset += newLen;
        if globalFlag != 0 {
          continue;
        }
        if needPrint != 0 {
          printLines(num1, num1, 0i32);
          needPrint = 0i32
        }
        lp = (*lp).next;
        num1 += 1
      } else {
        /*
         * The new string is larger, so allocate a new line
         * structure and use that.  Link it in place of
         * the old line structure.
         */
        nlp = xmalloc(
          (::std::mem::size_of::<LINE>() as libc::c_ulong)
            .wrapping_add((*lp).len as libc::c_ulong)
            .wrapping_add(deltaLen as libc::c_ulong),
        ) as *mut LINE;
        (*nlp).len = (*lp).len + deltaLen;
        memcpy(
          (*nlp).data.as_mut_ptr() as *mut libc::c_void,
          (*lp).data.as_mut_ptr() as *const libc::c_void,
          offset as libc::c_ulong,
        );
        memcpy(
          &mut *(*nlp).data.as_mut_ptr().offset(offset as isize) as *mut libc::c_char
            as *mut libc::c_void,
          newStr as *const libc::c_void,
          newLen as libc::c_ulong,
        );
        memcpy(
          &mut *(*nlp).data.as_mut_ptr().offset((offset + newLen) as isize) as *mut libc::c_char
            as *mut libc::c_void,
          &mut *(*lp).data.as_mut_ptr().offset((offset + oldLen) as isize) as *mut libc::c_char
            as *const libc::c_void,
          ((*lp).len - offset - oldLen) as libc::c_ulong,
        );
        (*nlp).next = (*lp).next;
        (*nlp).prev = (*lp).prev;
        (*(*nlp).prev).next = nlp;
        (*(*nlp).next).prev = nlp;
        if (*ptr_to_globals).curLine == lp {
          (*ptr_to_globals).curLine = nlp
        }
        free(lp as *mut libc::c_void);
        lp = nlp;
        offset += newLen;
        if globalFlag != 0 {
          continue;
        }
        if needPrint != 0 {
          printLines(num1, num1, 0i32);
          needPrint = 0i32
        }
        lp = (*lp).next;
        num1 += 1
      }
    }
  }
  if didSub == 0 {
    bb_error_msg(
      b"no substitutions found for \"%s\"\x00" as *const u8 as *const libc::c_char,
      oldStr,
    );
  };
}
/*
 * Read commands until we are told to stop.
 */
unsafe extern "C" fn doCommands() {
  while 1i32 != 0 {
    let mut buf: [libc::c_char; 1023] = [0; 1023];
    let mut cp: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut num1: libc::c_int = 0;
    let mut num2: libc::c_int = 0;
    let mut h: smallint = 0;
    let mut have1: smallint = 0;
    let mut have2: smallint = 0;
    /* Returns:
     * -1 on read errors or EOF, or on bare Ctrl-D.
     * 0  on ctrl-C,
     * >0 length of input string, including terminating '\n'
     */
    len = read_line_input(
      0 as *mut line_input_t,
      b": \x00" as *const u8 as *const libc::c_char,
      buf.as_mut_ptr(),
      ::std::mem::size_of::<[libc::c_char; 1023]>() as libc::c_ulong as libc::c_int,
    );
    if len <= 0i32 {
      return;
    }
    while len != 0
      && ({
        len -= 1;
        let mut bb__isspace: libc::c_uchar =
          (buf[len as usize] as libc::c_int - 9i32) as libc::c_uchar;
        (bb__isspace as libc::c_int == ' ' as i32 - 9i32
          || bb__isspace as libc::c_int <= 13i32 - 9i32) as libc::c_int
      }) != 0
    {
      buf[len as usize] = '\u{0}' as i32 as libc::c_char
    }
    if (*ptr_to_globals).curNum == 0i32 && (*ptr_to_globals).lastNum > 0i32 {
      (*ptr_to_globals).curNum = 1i32;
      (*ptr_to_globals).curLine = (*ptr_to_globals).lines.next
    }
    have1 = 0i32 as smallint;
    have2 = 0i32 as smallint;
    /* Don't pass &haveN, &numN to getNum() since this forces
     * compiler to keep them on stack, not in registers,
     * which is usually quite suboptimal.
     * Using intermediate variables shrinks code by ~150 bytes.
     */
    cp = getNum(skip_whitespace(buf.as_mut_ptr()), &mut h, &mut n);
    if cp.is_null() {
      continue;
    }
    have1 = h;
    num1 = n;
    cp = skip_whitespace(cp);
    if *cp as libc::c_int == ',' as i32 {
      cp = getNum(cp.offset(1), &mut h, &mut n);
      if cp.is_null() {
        continue;
      }
      num2 = n;
      if have1 == 0 {
        num1 = 1i32
      }
      if h == 0 {
        num2 = (*ptr_to_globals).lastNum
      }
      have1 = 1i32 as smallint;
      have2 = 1i32 as smallint
    }
    if have1 == 0 {
      num1 = (*ptr_to_globals).curNum
    }
    if have2 == 0 {
      num2 = num1
    }
    let mut current_block_86: u64;
    let fresh13 = cp;
    cp = cp.offset(1);
    match *fresh13 as libc::c_int {
      97 => {
        addLines(num1 + 1i32);
      }
      99 => {
        deleteLines(num1, num2);
        addLines(num1);
      }
      100 => {
        deleteLines(num1, num2);
      }
      102 => {
        if *cp as libc::c_int != '\u{0}' as i32 && *cp as libc::c_int != ' ' as i32 {
          bb_simple_error_msg(b"bad file command\x00" as *const u8 as *const libc::c_char);
        } else {
          cp = skip_whitespace(cp);
          if *cp as libc::c_int == '\u{0}' as i32 {
            if !(*ptr_to_globals).fileName.is_null() {
              printf(
                b"\"%s\"\n\x00" as *const u8 as *const libc::c_char,
                (*ptr_to_globals).fileName,
              );
            } else {
              puts(b"No file name\x00" as *const u8 as *const libc::c_char);
            }
          } else {
            free((*ptr_to_globals).fileName as *mut libc::c_void);
            (*ptr_to_globals).fileName = xstrdup(cp)
          }
        }
      }
      105 => {
        if have1 == 0 && (*ptr_to_globals).lastNum == 0i32 {
          num1 = 1i32
        }
        addLines(num1);
      }
      107 => {
        cp = skip_whitespace(cp);
        if (*cp as libc::c_int - 'a' as i32) as libc::c_uint >= 26i32 as libc::c_uint
          || *cp.offset(1) as libc::c_int != 0
        {
          bb_simple_error_msg(b"bad mark name\x00" as *const u8 as *const libc::c_char);
        } else {
          (*ptr_to_globals).marks[(*cp as libc::c_int - 'a' as i32) as libc::c_uint as usize] = num2
        }
      }
      108 => {
        printLines(num1, num2, 1i32);
      }
      112 => {
        printLines(num1, num2, 0i32);
      }
      113 => {
        cp = skip_whitespace(cp);
        if have1 as libc::c_int != 0 || *cp as libc::c_int != 0 {
          bb_simple_error_msg(b"bad quit command\x00" as *const u8 as *const libc::c_char);
        } else {
          if (*ptr_to_globals).dirty == 0 {
            return;
          }
          len = read_line_input(
            0 as *mut line_input_t,
            b"Really quit? \x00" as *const u8 as *const libc::c_char,
            buf.as_mut_ptr(),
            16i32,
          );
          /* read error/EOF - no way to continue */
          if len < 0i32 {
            return;
          }
          cp = skip_whitespace(buf.as_mut_ptr());
          if *cp as libc::c_int | 0x20i32 == 'y' as i32 {
            /* Y or y */
            return;
          }
        }
      }
      114 => {
        if *cp as libc::c_int != '\u{0}' as i32 && *cp as libc::c_int != ' ' as i32 {
          bb_simple_error_msg(b"bad read command\x00" as *const u8 as *const libc::c_char);
        } else {
          cp = skip_whitespace(cp);
          if *cp as libc::c_int == '\u{0}' as i32 {
            bb_simple_error_msg(b"no file name\x00" as *const u8 as *const libc::c_char);
          } else {
            if have1 == 0 {
              num1 = (*ptr_to_globals).lastNum
            }
            if !(readLines(cp, num1 + 1i32) != 0) {
              if (*ptr_to_globals).fileName.is_null() {
                (*ptr_to_globals).fileName = xstrdup(cp)
              }
            }
          }
        }
      }
      115 => {
        subCommand(cp, num1, num2);
      }
      119 => {
        if *cp as libc::c_int != '\u{0}' as i32 && *cp as libc::c_int != ' ' as i32 {
          bb_simple_error_msg(b"bad write command\x00" as *const u8 as *const libc::c_char);
        } else {
          cp = skip_whitespace(cp);
          if *cp as libc::c_int == '\u{0}' as i32 {
            cp = (*ptr_to_globals).fileName;
            if cp.is_null() {
              bb_simple_error_msg(
                b"no file name specified\x00" as *const u8 as *const libc::c_char,
              );
              current_block_86 = 13505557363059842426;
            } else {
              current_block_86 = 7072655752890836508;
            }
          } else {
            current_block_86 = 7072655752890836508;
          }
          match current_block_86 {
            13505557363059842426 => {}
            _ => {
              if have1 == 0 {
                num1 = 1i32;
                num2 = (*ptr_to_globals).lastNum;
                (*ptr_to_globals).dirty = 0i32 as smallint
              }
              writeLines(cp, num1, num2);
            }
          }
        }
      }
      122 => match *cp as libc::c_int {
        45 => {
          printLines(
            (*ptr_to_globals).curNum - 21i32,
            (*ptr_to_globals).curNum,
            0i32,
          );
        }
        46 => {
          printLines(
            (*ptr_to_globals).curNum - 11i32,
            (*ptr_to_globals).curNum + 10i32,
            0i32,
          );
        }
        _ => {
          printLines(
            (*ptr_to_globals).curNum,
            (*ptr_to_globals).curNum + 21i32,
            0i32,
          );
        }
      },
      46 => {
        if have1 != 0 {
          bb_simple_error_msg(b"no arguments allowed\x00" as *const u8 as *const libc::c_char);
        } else {
          printLines((*ptr_to_globals).curNum, (*ptr_to_globals).curNum, 0i32);
        }
      }
      45 => {
        if setCurNum((*ptr_to_globals).curNum - 1i32) != 0 {
          printLines((*ptr_to_globals).curNum, (*ptr_to_globals).curNum, 0i32);
        }
      }
      61 => {
        printf(b"%d\n\x00" as *const u8 as *const libc::c_char, num1);
      }
      0 => {
        if have1 != 0 {
          printLines(num2, num2, 0i32);
        } else if setCurNum((*ptr_to_globals).curNum + 1i32) != 0 {
          printLines((*ptr_to_globals).curNum, (*ptr_to_globals).curNum, 0i32);
        }
      }
      _ => {
        bb_simple_error_msg(b"unimplemented command\x00" as *const u8 as *const libc::c_char);
      }
    }
  }
}
#[no_mangle]
pub unsafe extern "C" fn ed_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let ref mut fresh14 =
    *(not_const_pp(&ptr_to_globals as *const *mut globals as *const libc::c_void)
      as *mut *mut globals);
  *fresh14 = xzalloc(::std::mem::size_of::<globals>() as libc::c_ulong) as *mut globals;
  asm!("" : : : "memory" : "volatile");
  (*ptr_to_globals).bufSize = INITBUF_SIZE as libc::c_int;
  (*ptr_to_globals).bufBase = xmalloc((*ptr_to_globals).bufSize as size_t) as *mut libc::c_char;
  (*ptr_to_globals).bufPtr = (*ptr_to_globals).bufBase;
  (*ptr_to_globals).lines.next = &mut (*ptr_to_globals).lines;
  (*ptr_to_globals).lines.prev = &mut (*ptr_to_globals).lines;
  if !(*argv.offset(1)).is_null() {
    (*ptr_to_globals).fileName = xstrdup(*argv.offset(1));
    if readLines((*ptr_to_globals).fileName, 1i32) == 0 {
      return 0i32;
    }
    if (*ptr_to_globals).lastNum != 0 {
      setCurNum(1i32);
    }
    (*ptr_to_globals).dirty = 0i32 as smallint
  }
  doCommands();
  return 0i32;
}
