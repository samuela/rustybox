use libc;
extern "C" {
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  fn exit(_: libc::c_int) -> !;
  #[no_mangle]
  fn unlink(__name: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn syscall(__sysno: libc::c_long, _: ...) -> libc::c_long;
  #[no_mangle]
  fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
  #[no_mangle]
  fn strerror(_: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn rename(__old: *const libc::c_char, __new: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn fclose(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn ferror_unlocked(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn usleep(__useconds: __useconds_t) -> libc::c_int;
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memchr(_: *const libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
  #[no_mangle]
  fn strchrnul(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn strrstr(haystack: *const libc::c_char, needle: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xmalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xrealloc(old: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xrealloc_vector_helper(
    vector: *mut libc::c_void,
    sizeof_and_shift: libc::c_uint,
    idx: libc::c_int,
  ) -> *mut libc::c_void;
  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn recursive_action(
    fileName: *const libc::c_char,
    flags: libc::c_uint,
    fileAction_0: Option<
      unsafe extern "C" fn(
        _: *const libc::c_char,
        _: *mut stat,
        _: *mut libc::c_void,
        _: libc::c_int,
      ) -> libc::c_int,
    >,
    dirAction: Option<
      unsafe extern "C" fn(
        _: *const libc::c_char,
        _: *mut stat,
        _: *mut libc::c_void,
        _: libc::c_int,
      ) -> libc::c_int,
    >,
    userData: *mut libc::c_void,
    depth: libc::c_uint,
  ) -> libc::c_int;
  #[no_mangle]
  fn bb_get_last_path_component_nostrip(path: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn is_prefixed_with(string: *const libc::c_char, key: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xchdir(path: *const libc::c_char);
  #[no_mangle]
  fn open_or_warn(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  #[no_mangle]
  fn xmalloc_open_read_close(
    filename: *const libc::c_char,
    maxsz_p: *mut size_t,
  ) -> *mut libc::c_void;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn xmalloc_open_zipped_read_close(
    fname: *const libc::c_char,
    maxsz_p: *mut size_t,
  ) -> *mut libc::c_void;
  /* Reads and prints to stdout till eof, then closes FILE. Exits on error: */
  #[no_mangle]
  fn xprint_and_close_file(file: *mut FILE);
  /* Chops off '\n' from the end, unlike fgets: */
  #[no_mangle]
  fn xmalloc_fgetline(file: *mut FILE) -> *mut libc::c_char;
  #[no_mangle]
  fn fopen_for_read(path: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn xfopen_for_read(path: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn xfdopen_for_write(fd: libc::c_int) -> *mut FILE;
  /* { "-", NULL } */
  #[no_mangle]
  static mut option_mask32: u32;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_perror_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_perror_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  static mut applet_name: *const libc::c_char;
  /* '*const' ptr makes gcc optimize code much better.
   * Magic prevents ptr_to_globals from going into rodata.
   * If you want to assign a value, use SET_PTR_TO_GLOBALS(x) */
  #[no_mangle]
  static ptr_to_globals: *mut globals;
  #[no_mangle]
  fn uname(__name: *mut utsname) -> libc::c_int;
  #[no_mangle]
  fn fnmatch(
    __pattern: *const libc::c_char,
    __name: *const libc::c_char,
    __flags: libc::c_int,
  ) -> libc::c_int;
}

use crate::librb::__useconds_t;

use crate::librb::size_t;
use crate::librb::smallint;
use libc::ssize_t;


use libc::stat;

use libc::FILE;
pub type C2RustUnnamed = libc::c_uint;
pub const ACTION_DANGLING_OK: C2RustUnnamed = 64;
pub const ACTION_QUIET: C2RustUnnamed = 32;
pub const ACTION_DEPTHFIRST: C2RustUnnamed = 8;
pub const ACTION_FOLLOWLINKS_L0: C2RustUnnamed = 4;
pub const ACTION_FOLLOWLINKS: C2RustUnnamed = 2;
pub const ACTION_RECURSE: C2RustUnnamed = 1;
//extern const int const_int_1;
/* This struct is deliberately not defined. */
/* See docs/keep_data_small.txt */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub modinfo: *mut module_info,
  pub module_load_options: *mut libc::c_char,
  pub dep_bb_seen: smallint,
  pub wrote_dep_bb_ok: smallint,
  pub module_count: libc::c_uint,
  pub module_found_idx: libc::c_int,
  pub stringbuf_idx: libc::c_uint,
  pub stringbuf_size: libc::c_uint,
  pub stringbuf: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct module_info {
  pub pathname: *mut libc::c_char,
  pub aliases: *mut libc::c_char,
  pub deps: *mut libc::c_char,
  pub open_read_failed: smallint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utsname {
  pub sysname: [libc::c_char; 65],
  pub nodename: [libc::c_char; 65],
  pub release: [libc::c_char; 65],
  pub version: [libc::c_char; 65],
  pub machine: [libc::c_char; 65],
  pub domainname: [libc::c_char; 65],
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const OPT_r: C2RustUnnamed_0 = 2;
pub const OPT_q: C2RustUnnamed_0 = 1;
pub const DEPMOD_OPT_n: C2RustUnnamed_0 = 1;
#[inline(always)]
unsafe extern "C" fn not_const_pp(mut p: *const libc::c_void) -> *mut libc::c_void {
  return p as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn lsmod_main(
  mut _argc: libc::c_int,
  mut _argv: *mut *mut libc::c_char,
) -> libc::c_int {
  xprint_and_close_file(xfopen_for_read(
    b"/proc/modules\x00" as *const u8 as *const libc::c_char,
  ));
  return 0i32;
}
/* some modules have lots of stuff */
/* for example, drivers/media/video/saa7134/saa7134.ko */
/* therefore having a fixed biggish buffer is not wise */
unsafe extern "C" fn append(mut s: *const libc::c_char) {
  let mut len: libc::c_uint = strlen(s) as libc::c_uint;
  if (*ptr_to_globals)
    .stringbuf_idx
    .wrapping_add(len)
    .wrapping_add(15i32 as libc::c_uint)
    > (*ptr_to_globals).stringbuf_size
  {
    (*ptr_to_globals).stringbuf_size = (*ptr_to_globals)
      .stringbuf_idx
      .wrapping_add(len)
      .wrapping_add(127i32 as libc::c_uint);
    (*ptr_to_globals).stringbuf = xrealloc(
      (*ptr_to_globals).stringbuf as *mut libc::c_void,
      (*ptr_to_globals).stringbuf_size as size_t,
    ) as *mut libc::c_char
  }
  memcpy(
    (*ptr_to_globals)
      .stringbuf
      .offset((*ptr_to_globals).stringbuf_idx as isize) as *mut libc::c_void,
    s as *const libc::c_void,
    len as libc::c_ulong,
  );
  (*ptr_to_globals).stringbuf_idx = (*ptr_to_globals).stringbuf_idx.wrapping_add(len);
}
unsafe extern "C" fn appendc(mut c: libc::c_char) {
  /* We appendc() only after append(), + 15 trick in append()
   * makes it unnecessary to check for overflow here */
  let fresh0 = (*ptr_to_globals).stringbuf_idx; /* terminating NUL */
  (*ptr_to_globals).stringbuf_idx = (*ptr_to_globals).stringbuf_idx.wrapping_add(1);
  *(*ptr_to_globals).stringbuf.offset(fresh0 as isize) = c;
}
unsafe extern "C" fn bksp() {
  if (*ptr_to_globals).stringbuf_idx != 0 {
    (*ptr_to_globals).stringbuf_idx = (*ptr_to_globals).stringbuf_idx.wrapping_sub(1)
  };
}
unsafe extern "C" fn reset_stringbuf() {
  (*ptr_to_globals).stringbuf_idx = 0i32 as libc::c_uint;
}
unsafe extern "C" fn copy_stringbuf() -> *mut libc::c_char {
  let mut copy: *mut libc::c_char = xzalloc(
    (*ptr_to_globals)
      .stringbuf_idx
      .wrapping_add(1i32 as libc::c_uint) as size_t,
  ) as *mut libc::c_char;
  return memcpy(
    copy as *mut libc::c_void,
    (*ptr_to_globals).stringbuf as *const libc::c_void,
    (*ptr_to_globals).stringbuf_idx as libc::c_ulong,
  ) as *mut libc::c_char;
}
unsafe extern "C" fn find_keyword(
  mut ptr: *mut libc::c_char,
  mut len: size_t,
  mut word: *const libc::c_char,
) -> *mut libc::c_char {
  if ptr.is_null() {
    /* happens if xmalloc_open_zipped_read_close cannot read it */
    return 0 as *mut libc::c_char;
  }
  len = (len as libc::c_ulong).wrapping_sub(strlen(word).wrapping_sub(1i32 as libc::c_ulong))
    as size_t as size_t;
  while len as ssize_t >0{
    let mut old: *mut libc::c_char = ptr;
    let mut after_word: *mut libc::c_char = 0 as *mut libc::c_char;
    /* search for the first char in word */
    ptr = memchr(
      ptr as *const libc::c_void,
      *word.offset(0) as libc::c_int,
      len,
    ) as *mut libc::c_char; /* found, return ptr past it */
    if ptr.is_null() {
      break;
    }
    after_word = is_prefixed_with(ptr, word);
    if !after_word.is_null() {
      return after_word;
    }
    ptr = ptr.offset(1);
    len = (len as libc::c_ulong)
      .wrapping_sub(ptr.wrapping_offset_from(old) as libc::c_long as libc::c_ulong)
      as size_t as size_t
  }
  return 0 as *mut libc::c_char;
}
unsafe extern "C" fn replace(
  mut s: *mut libc::c_char,
  mut what: libc::c_char,
  mut with: libc::c_char,
) {
  while *s != 0 {
    if what as libc::c_int == *s as libc::c_int {
      *s = with
    }
    s = s.offset(1)
  }
}
unsafe extern "C" fn filename2modname(
  mut filename: *const libc::c_char,
  mut modname: *mut libc::c_char,
) -> *mut libc::c_char {
  let mut i: libc::c_int = 0;
  let mut from: *const libc::c_char = 0 as *const libc::c_char;
  // Disabled since otherwise "modprobe dir/name" would work
  // as if it is "modprobe name". It is unclear why
  // 'basenamization' was here in the first place.
  //from = bb_get_last_path_component_nostrip(filename);
  from = filename;
  i = 0i32;
  while i < 64i32 - 1i32
    && *from.offset(i as isize) as libc::c_int != '\u{0}' as i32
    && *from.offset(i as isize) as libc::c_int != '.' as i32
  {
    *modname.offset(i as isize) = if *from.offset(i as isize) as libc::c_int == '-' as i32 {
      '_' as i32
    } else {
      *from.offset(i as isize) as libc::c_int
    } as libc::c_char;
    i += 1
  }
  *modname.offset(i as isize) = '\u{0}' as i32 as libc::c_char;
  return modname;
}
unsafe extern "C" fn pathname_matches_modname(
  mut pathname: *const libc::c_char,
  mut modname: *const libc::c_char,
) -> libc::c_int {
  let mut r: libc::c_int = 0;
  let mut name: [libc::c_char; 64] = [0; 64];
  filename2modname(
    bb_get_last_path_component_nostrip(pathname),
    name.as_mut_ptr(),
  );
  r = (strcmp(name.as_mut_ptr(), modname) == 0i32) as libc::c_int;
  return r;
}
/* Take "word word", return malloced "word",NUL,"word",NUL,NUL */
unsafe extern "C" fn str_2_list(mut str: *const libc::c_char) -> *mut libc::c_char {
  let mut len: libc::c_int = strlen(str).wrapping_add(1i32 as libc::c_ulong) as libc::c_int;
  let mut dst: *mut libc::c_char = xmalloc((len + 1i32) as size_t) as *mut libc::c_char;
  *dst.offset(len as isize) = '\u{0}' as i32 as libc::c_char;
  memcpy(
    dst as *mut libc::c_void,
    str as *const libc::c_void,
    len as libc::c_ulong,
  );
  //TODO: protect against 2+ spaces: "word  word"
  replace(
    dst,
    ' ' as i32 as libc::c_char,
    '\u{0}' as i32 as libc::c_char,
  );
  return dst;
}
/* We use error numbers in a loose translation... */
unsafe extern "C" fn moderror(mut err: libc::c_int) -> *const libc::c_char {
  match err {
    8 => return b"invalid module format\x00" as *const u8 as *const libc::c_char,
    2 => {
      return b"unknown symbol in module or invalid parameter\x00" as *const u8
        as *const libc::c_char
    }
    3 => return b"module has wrong symbol version\x00" as *const u8 as *const libc::c_char,
    22 => {
      /* "invalid parameter" */
      return (b"unknown symbol in module or invalid parameter\x00" as *const u8
        as *const libc::c_char)
        .offset(::std::mem::size_of::<[libc::c_char; 28]>() as libc::c_ulong as isize);
    }
    _ => return strerror(err),
  };
}
unsafe extern "C" fn load_module(
  mut fname: *const libc::c_char,
  mut options: *const libc::c_char,
) -> libc::c_int {
  let mut r: libc::c_int = 0;
  let mut len: size_t = if -1i32 as ssize_t >0{
    -1i32 as ssize_t
  } else {
    !((1i32 as ssize_t)
      << (::std::mem::size_of::<ssize_t>() as libc::c_ulong)
        .wrapping_mul(8i32 as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong))
  } as size_t;
  let mut module_image: *mut libc::c_char = 0 as *mut libc::c_char;
  if options.is_null() {
    options = b"\x00" as *const u8 as *const libc::c_char
  }
  /*
   * First we try finit_module if available.  Some kernels are configured
   * to only allow loading of modules off of secure storage (like a read-
   * only rootfs) which needs the finit_module call.  If it fails, we fall
   * back to normal module loading to support compressed modules.
   */
  r = 1i32;
  let mut fd: libc::c_int = open(fname, 0i32 | 0o2000000i32);
  if fd >= 0i32 {
    r = (syscall(313i32 as libc::c_long, fd, options, 0i32) !=0) as libc::c_int;
    close(fd);
  }
  if r != 0i32 {
    module_image = xmalloc_open_zipped_read_close(fname, &mut len) as *mut libc::c_char;
    r = (module_image.is_null()
      || syscall(175i32 as libc::c_long, module_image, len, options) !=0)
      as libc::c_int;
    free(module_image as *mut libc::c_void);
  }
  return r;
  /* 0 = success */
}
/* Returns !0 if open/read was unsuccessful */
unsafe extern "C" fn parse_module(
  mut info: *mut module_info,
  mut pathname: *const libc::c_char,
) -> libc::c_int {
  let mut current_block: u64;
  let mut module_image: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut len: size_t = 0;
  let mut pos: size_t = 0;
  /* Read (possibly compressed) module */
  *bb_errno = 0i32; /* 64 Mb at most */
  len = (64i32 * 1024i32 * 1024i32) as size_t;
  module_image = xmalloc_open_zipped_read_close(pathname, &mut len) as *mut libc::c_char;
  /* module_image == NULL is ok here, find_keyword handles it */
  //TODO: optimize redundant module body reads
  /* "alias1 symbol:sym1 alias2 symbol:sym2" */
  reset_stringbuf();
  pos = 0i32 as size_t;
  loop {
    let mut start: libc::c_uint = (*ptr_to_globals).stringbuf_idx;
    ptr = find_keyword(
      module_image.offset(pos as isize),
      len.wrapping_sub(pos),
      b"alias=\x00" as *const u8 as *const libc::c_char,
    );
    if ptr.is_null() {
      ptr = find_keyword(
        module_image.offset(pos as isize),
        len.wrapping_sub(pos),
        b"__ksymtab_\x00" as *const u8 as *const libc::c_char,
      );
      if ptr.is_null() {
        break;
      }
      /* DOCME: __ksymtab_gpl and __ksymtab_strings occur
       * in many modules. What do they mean? */
      if strcmp(ptr, b"gpl\x00" as *const u8 as *const libc::c_char) == 0i32
        || strcmp(ptr, b"strings\x00" as *const u8 as *const libc::c_char) == 0i32
      {
        current_block = 14188675094891468690;
      } else {
        append(b"symbol:\x00" as *const u8 as *const libc::c_char);
        current_block = 17833034027772472439;
      }
    } else {
      current_block = 17833034027772472439;
    }
    match current_block {
      17833034027772472439 => {
        append(ptr);
        appendc(' ' as i32 as libc::c_char);
        /*
         * Don't add redundant aliases, such as:
         * libcrc32c.ko symbol:crc32c symbol:crc32c
         */
        if start != 0 {
          /* "if we aren't the first alias" */
          let mut found: *mut libc::c_char = 0 as *mut libc::c_char;
          let mut last: *mut libc::c_char = 0 as *mut libc::c_char;
          *(*ptr_to_globals)
            .stringbuf
            .offset((*ptr_to_globals).stringbuf_idx as isize) = '\u{0}' as i32 as libc::c_char;
          last = (*ptr_to_globals).stringbuf.offset(start as isize);
          /*
           * String at last-1 is " symbol:crc32c "
           * (with both leading and trailing spaces).
           */
          if strncmp(
            (*ptr_to_globals).stringbuf,
            last,
            (*ptr_to_globals).stringbuf_idx.wrapping_sub(start) as libc::c_ulong,
          ) == 0i32
          {
            /* First alias matches us */
            found = (*ptr_to_globals).stringbuf
          } else {
            /* Does any other alias match? */
            found = strstr((*ptr_to_globals).stringbuf, last.offset(-1))
          }
          if found < last.offset(-1) {
            /* There is absolutely the same string before us */
            (*ptr_to_globals).stringbuf_idx = start
          }
        }
      }
      _ => {}
    } /* remove last ' ' */
    pos = ptr.wrapping_offset_from(module_image) as libc::c_long as size_t
  }
  bksp();
  (*info).aliases = copy_stringbuf();
  replace(
    (*info).aliases,
    '-' as i32 as libc::c_char,
    '_' as i32 as libc::c_char,
  );
  /* "dependency1 depandency2" */
  reset_stringbuf(); /* skip "./" */
  ptr = find_keyword(
    module_image,
    len,
    b"depends=\x00" as *const u8 as *const libc::c_char,
  );
  if !ptr.is_null() && *ptr as libc::c_int != 0 {
    replace(ptr, ',' as i32 as libc::c_char, ' ' as i32 as libc::c_char);
    replace(ptr, '-' as i32 as libc::c_char, '_' as i32 as libc::c_char);
    append(ptr);
  }
  free(module_image as *mut libc::c_void);
  (*info).deps = copy_stringbuf();
  (*info).open_read_failed =
    (module_image == 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int as smallint;
  return (*info).open_read_failed as libc::c_int;
}
unsafe extern "C" fn fileAction(
  mut pathname: *const libc::c_char,
  mut _sb: *mut stat,
  mut modname_to_match: *mut libc::c_void,
  mut _depth: libc::c_int,
) -> libc::c_int {
  let mut cur: libc::c_int = 0;
  let mut fname: *const libc::c_char = 0 as *const libc::c_char;
  let mut is_remove: bool = 1i32 != 0 && 1i32 + 1i32 + 1i32 + 1i32 == 1i32
    || (1i32 != 0 || 1i32 != 0) && option_mask32 & OPT_r as libc::c_int as libc::c_uint != 0;
  pathname = pathname.offset(2);
  fname = bb_get_last_path_component_nostrip(pathname);
  if strrstr(fname, b".ko\x00" as *const u8 as *const libc::c_char).is_null() {
    return 1i32;
    /* not a module, continue search */
  }
  let fresh1 = (*ptr_to_globals).module_count;
  (*ptr_to_globals).module_count = (*ptr_to_globals).module_count.wrapping_add(1);
  cur = fresh1 as libc::c_int;
  (*ptr_to_globals).modinfo = xrealloc_vector_helper(
    (*ptr_to_globals).modinfo as *mut libc::c_void,
    ((::std::mem::size_of::<module_info>() as libc::c_ulong) << 8i32)
      .wrapping_add(12i32 as libc::c_ulong) as libc::c_uint,
    cur,
  ) as *mut module_info;
  let ref mut fresh2 = (*(*ptr_to_globals).modinfo.offset(cur as isize)).pathname;
  *fresh2 = xstrdup(pathname);
  /*modinfo[cur].aliases = NULL; - xrealloc_vector did it */
  /*modinfo[cur+1].pathname = NULL;*/
  if pathname_matches_modname(fname, modname_to_match as *const libc::c_char) == 0 {
    return 1i32;
    /* module name doesn't match, continue search */
  } /* failed to open/read it, no point in trying loading */
  (*ptr_to_globals).module_found_idx = cur;
  if parse_module(
    &mut *(*ptr_to_globals).modinfo.offset(cur as isize),
    pathname,
  ) != 0i32
  {
    return 1i32;
  }
  if !is_remove {
    if load_module(pathname, (*ptr_to_globals).module_load_options) == 0i32 {
      /* Load was successful, there is nothing else to do.
       * This can happen ONLY for "top-level" module load,
       * not a dep, because deps don't do dirscan. */
      exit(0i32);
    }
  }
  return 1i32;
}
unsafe extern "C" fn load_dep_bb() -> libc::c_int {
  let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut fp: *mut FILE = fopen_for_read(b"modules.dep.bb\x00" as *const u8 as *const libc::c_char);
  if fp.is_null() {
    return 0i32;
  }
  (*ptr_to_globals).dep_bb_seen = 1i32 as smallint;
  /* Why? There is a rare scenario: we did not find modprobe.dep.bb,
   * we scanned the dir and found no module by name, then we search
   * for alias (full scan), and we decided to generate modprobe.dep.bb.
   * But we see modprobe.dep.bb.new! Other modprobe is at work!
   * We wait and other modprobe renames it to modprobe.dep.bb.
   * Now we can use it.
   * But we already have modinfo[] filled, and "module_count = 0"
   * makes us start anew. Yes, we leak modinfo[].xxx pointers -
   * there is not much of data there anyway. */
  (*ptr_to_globals).module_count = 0i32 as libc::c_uint;
  memset(
    &mut *(*ptr_to_globals).modinfo.offset(0) as *mut module_info as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<module_info>() as libc::c_ulong,
  );
  loop {
    line = xmalloc_fgetline(fp);
    if line.is_null() {
      break;
    }
    let mut space: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut linebuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cur: libc::c_int = 0;
    if *line.offset(0) == 0 {
      free(line as *mut libc::c_void);
    } else {
      space = strchrnul(line, ' ' as i32);
      let fresh3 = (*ptr_to_globals).module_count;
      (*ptr_to_globals).module_count = (*ptr_to_globals).module_count.wrapping_add(1);
      cur = fresh3 as libc::c_int;
      (*ptr_to_globals).modinfo = xrealloc_vector_helper(
        (*ptr_to_globals).modinfo as *mut libc::c_void,
        ((::std::mem::size_of::<module_info>() as libc::c_ulong) << 8i32)
          .wrapping_add(12i32 as libc::c_ulong) as libc::c_uint,
        cur,
      ) as *mut module_info;
      /*modinfo[cur+1].pathname = NULL; - xrealloc_vector did it */
      let ref mut fresh4 = (*(*ptr_to_globals).modinfo.offset(cur as isize)).pathname; /* we take ownership of malloced block here */
      *fresh4 = line;
      if *space != 0 {
        let fresh5 = space;
        space = space.offset(1);
        *fresh5 = '\u{0}' as i32 as libc::c_char
      }
      let ref mut fresh6 = (*(*ptr_to_globals).modinfo.offset(cur as isize)).aliases;
      *fresh6 = space;
      linebuf = xmalloc_fgetline(fp);
      let ref mut fresh7 = (*(*ptr_to_globals).modinfo.offset(cur as isize)).deps;
      *fresh7 = if !linebuf.is_null() {
        linebuf as *mut libc::c_void
      } else {
        xzalloc(1i32 as size_t)
      } as *mut libc::c_char;
      if *(*(*ptr_to_globals).modinfo.offset(cur as isize))
        .deps
        .offset(0)
        != 0
      {
        /* deps are not "", so next line must be empty */
        line = xmalloc_fgetline(fp);
        /* Refuse to work with damaged config file */
        if !line.is_null() && *line.offset(0) as libc::c_int != 0 {
          bb_error_msg_and_die(
            b"error in %s at \'%s\'\x00" as *const u8 as *const libc::c_char,
            b"modules.dep.bb\x00" as *const u8 as *const libc::c_char,
            line,
          );
        }
        free(line as *mut libc::c_void);
      }
    }
  }
  return 1i32;
}
unsafe extern "C" fn start_dep_bb_writeout() -> libc::c_int {
  let mut fd: libc::c_int = 0;
  /* depmod -n: write result to stdout */
  if 1i32 != 0
    && (1i32 + 1i32 + 1i32 + 1i32 == 1i32 || *applet_name.offset(0) as libc::c_int == 'd' as i32)
    && option_mask32 & DEPMOD_OPT_n as libc::c_int as libc::c_uint != 0
  {
    return 1i32;
  }
  fd = open(
    b"modules.dep.bb.new\x00" as *const u8 as *const libc::c_char,
    0o1i32 | 0o100i32 | 0o1000i32 | 0o200i32,
    0o644i32,
  );
  if fd < 0i32 {
    if *bb_errno == 17i32 {
      let mut count: libc::c_int = 5i32 * 20i32;
      loop {
        usleep((1000i32 * 1000i32 / 20i32) as __useconds_t);
        if load_dep_bb() != 0 {
          return -2i32;
          /* magic number */
        }
        count -= 1;
        if count == 0 {
          break;
        }
      }
      bb_error_msg(
        b"deleting stale %s\x00" as *const u8 as *const libc::c_char,
        b"modules.dep.bb.new\x00" as *const u8 as *const libc::c_char,
      );
      fd = open_or_warn(
        b"modules.dep.bb.new\x00" as *const u8 as *const libc::c_char,
        0o1i32 | 0o100i32 | 0o1000i32,
      )
    }
  }
  return fd;
}
unsafe extern "C" fn write_out_dep_bb(mut fd: libc::c_int) {
  let mut current_block: u64;
  let mut i: libc::c_int = 0;
  let mut fp: *mut FILE = 0 as *mut FILE;
  /* We want good error reporting. fdprintf is not good enough. */
  fp = xfdopen_for_write(fd);
  i = 0i32;
  while !(*(*ptr_to_globals).modinfo.offset(i as isize))
    .pathname
    .is_null()
  {
    fprintf(
      fp,
      b"%s%s%s\n%s%s\n\x00" as *const u8 as *const libc::c_char,
      (*(*ptr_to_globals).modinfo.offset(i as isize)).pathname,
      if *(*(*ptr_to_globals).modinfo.offset(i as isize))
        .aliases
        .offset(0) as libc::c_int
        != 0
      {
        b" \x00" as *const u8 as *const libc::c_char
      } else {
        b"\x00" as *const u8 as *const libc::c_char
      },
      (*(*ptr_to_globals).modinfo.offset(i as isize)).aliases,
      (*(*ptr_to_globals).modinfo.offset(i as isize)).deps,
      if *(*(*ptr_to_globals).modinfo.offset(i as isize))
        .deps
        .offset(0) as libc::c_int
        != 0
      {
        b"\n\x00" as *const u8 as *const libc::c_char
      } else {
        b"\x00" as *const u8 as *const libc::c_char
      },
    );
    i += 1
  }
  /* Badly formatted depfile is a no-no. Be paranoid. */
  *bb_errno = 0i32;
  if ferror_unlocked(fp) | fclose(fp) != 0 {
    current_block = 14742036289623448043;
  } else {
    if fd == 1i32 {
      current_block = 7367305048794505069;
    } else if rename(
      b"modules.dep.bb.new\x00" as *const u8 as *const libc::c_char,
      b"modules.dep.bb\x00" as *const u8 as *const libc::c_char,
    ) != 0i32
    {
      current_block = 14742036289623448043;
    } else {
      current_block = 7367305048794505069;
    }
    match current_block {
      14742036289623448043 => {}
      _ =>
      /* it was depmod -n */
      {
        (*ptr_to_globals).wrote_dep_bb_ok = 1i32 as smallint;
        current_block = 1054647088692577877;
      }
    }
  }
  match current_block {
    14742036289623448043 =>
    /* | instead of || is intended */
    {
      bb_perror_msg(
        b"can\'t create \'%s\'\x00" as *const u8 as *const libc::c_char,
        b"modules.dep.bb\x00" as *const u8 as *const libc::c_char,
      );
      unlink(b"modules.dep.bb.new\x00" as *const u8 as *const libc::c_char);
    }
    _ => {}
  };
}
unsafe extern "C" fn find_alias(mut alias: *const libc::c_char) -> *mut *mut module_info {
  let mut i: libc::c_int = 0;
  let mut dep_bb_fd: libc::c_int = 0;
  let mut infoidx: libc::c_int = 0;
  let mut infovec: *mut *mut module_info = 0 as *mut *mut module_info;
  loop
  /* modprobe.dep.bb appeared? */
  /* First try to find by name (cheaper) */
  {
    i = 0i32;
    while !(*(*ptr_to_globals).modinfo.offset(i as isize))
      .pathname
      .is_null()
    {
      if pathname_matches_modname(
        (*(*ptr_to_globals).modinfo.offset(i as isize)).pathname,
        alias,
      ) != 0
      {
        if (*(*ptr_to_globals).modinfo.offset(i as isize))
          .aliases
          .is_null()
        {
          parse_module(
            &mut *(*ptr_to_globals).modinfo.offset(i as isize),
            (*(*ptr_to_globals).modinfo.offset(i as isize)).pathname,
          );
        }
        infovec = xzalloc(
          (2i32 as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut module_info>() as libc::c_ulong),
        ) as *mut *mut module_info;
        let ref mut fresh8 = *infovec.offset(0);
        *fresh8 = &mut *(*ptr_to_globals).modinfo.offset(i as isize) as *mut module_info;
        return infovec;
      }
      i += 1
    }
    /* Ok, we definitely have to scan module bodies. This is a good
     * moment to generate modprobe.dep.bb, if it does not exist yet */
    dep_bb_fd = if (*ptr_to_globals).dep_bb_seen as libc::c_int != 0 {
      -1i32
    } else {
      start_dep_bb_writeout()
    };
    if !(dep_bb_fd == -2i32) {
      break;
    }
  }
  /* Scan all module bodies, extract modinfo (it contains aliases) */
  i = 0i32;
  infoidx = 0i32;
  infovec = 0 as *mut *mut module_info;
  while !(*(*ptr_to_globals).modinfo.offset(i as isize))
    .pathname
    .is_null()
  {
    let mut desc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*(*ptr_to_globals).modinfo.offset(i as isize))
      .aliases
      .is_null()
    {
      parse_module(
        &mut *(*ptr_to_globals).modinfo.offset(i as isize),
        (*(*ptr_to_globals).modinfo.offset(i as isize)).pathname,
      );
    }
    /* "alias1 symbol:sym1 alias2 symbol:sym2" */
    desc = str_2_list((*(*ptr_to_globals).modinfo.offset(i as isize)).aliases);
    /* Does matching substring exist? */
    s = desc;
    while *s != 0 {
      /* Aliases in module bodies can be defined with
       * shell patterns. Example:
       * "pci:v000010DEd000000D9sv*sd*bc*sc*i*".
       * Plain strcmp() won't catch that */
      if fnmatch(s, alias, 0i32) == 0i32 {
        infovec = xrealloc_vector_helper(
          infovec as *mut libc::c_void,
          ((::std::mem::size_of::<*mut module_info>() as libc::c_ulong) << 8i32)
            .wrapping_add(1i32 as libc::c_ulong) as libc::c_uint,
          infoidx,
        ) as *mut *mut module_info;
        let fresh9 = infoidx;
        infoidx = infoidx + 1;
        let ref mut fresh10 = *infovec.offset(fresh9 as isize);
        *fresh10 = &mut *(*ptr_to_globals).modinfo.offset(i as isize) as *mut module_info;
        break;
      } else {
        s = s.offset(strlen(s).wrapping_add(1i32 as libc::c_ulong) as isize)
      }
    }
    free(desc as *mut libc::c_void);
    i += 1
  }
  /* Create module.dep.bb if needed */
  if dep_bb_fd >= 0i32 {
    write_out_dep_bb(dep_bb_fd);
  }
  return infovec;
}
// TODO: open only once, invent config_rewind()
unsafe extern "C" fn already_loaded(mut name: *const libc::c_char) -> libc::c_int {
  let mut ret: libc::c_int = 0;
  let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut fp: *mut FILE = 0 as *mut FILE;
  ret = 5i32 * 2i32;
  'c_11409: loop {
    fp = fopen_for_read(b"/proc/modules\x00" as *const u8 as *const libc::c_char);
    if fp.is_null() {
      return 0i32;
    }
    loop {
      line = xmalloc_fgetline(fp);
      if line.is_null() {
        break 'c_11409;
      }
      let mut live: *mut libc::c_char = 0 as *mut libc::c_char;
      let mut after_name: *mut libc::c_char = 0 as *mut libc::c_char;
      // Examples from kernel 3.14.6:
      //pcspkr 12718 0 - Live 0xffffffffa017e000
      //snd_timer 28690 2 snd_seq,snd_pcm, Live 0xffffffffa025e000
      //i915 801405 2 - Live 0xffffffffa0096000
      after_name = is_prefixed_with(line, name);
      if after_name.is_null() || *after_name as libc::c_int != ' ' as i32 {
        free(line as *mut libc::c_void);
      } else {
        live = strstr(line, b" Live\x00" as *const u8 as *const libc::c_char);
        free(line as *mut libc::c_void);
        if live.is_null() {
          /* State can be Unloading, Loading, or Live.
           * modprobe must not return prematurely if we see "Loading":
           * it can cause further programs to assume load completed,
           * but it did not (yet)!
           * Wait up to 5*20 ms for it to resolve.
           */
          ret -= 2i32; /* huh? report as "not loaded" */
          if ret == 0i32 {
            break 'c_11409;
          }
          fclose(fp);
          usleep((20i32 * 1000i32) as __useconds_t);
          break;
        } else {
          ret = 1i32;
          break 'c_11409;
        }
      }
    }
  }
  fclose(fp);
  return ret & 1i32;
}
unsafe extern "C" fn rmmod(mut filename: *const libc::c_char) -> libc::c_int {
  let mut r: libc::c_int = 0;
  let mut modname: [libc::c_char; 64] = [0; 64];
  filename2modname(filename, modname.as_mut_ptr());
  r = syscall(
    176i32 as libc::c_long,
    modname.as_mut_ptr(),
    0o4000i32 | 0o200i32,
  ) as libc::c_int;
  if r != 0i32 && option_mask32 & OPT_q as libc::c_int as libc::c_uint == 0 {
    bb_perror_msg(
      b"remove \'%s\'\x00" as *const u8 as *const libc::c_char,
      modname.as_mut_ptr(),
    );
  }
  return r;
}
/*
 * Given modules definition and module name (or alias, or symbol)
 * load/remove the module respecting dependencies.
 * NB: also called by depmod with bogus name "/",
 * just in order to force modprobe.dep.bb creation.
*/
unsafe extern "C" fn process_module(
  mut name: *mut libc::c_char,
  mut cmdline_options: *const libc::c_char,
) -> libc::c_int {
  let mut current_block: u64;
  let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut deps: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut options: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut infovec: *mut *mut module_info = 0 as *mut *mut module_info;
  let mut info: *mut module_info = 0 as *mut module_info;
  let mut infoidx: libc::c_int = 0;
  let mut is_remove: bool = 1i32 != 0 && 1i32 + 1i32 + 1i32 + 1i32 == 1i32
    || (1i32 != 0 || 1i32 != 0) && option_mask32 & OPT_r as libc::c_int as libc::c_uint != 0;
  let mut exitcode: libc::c_int = 0i32;
  replace(name, '-' as i32 as libc::c_char, '_' as i32 as libc::c_char);
  if 1i32 != 0
    && (1i32 + 1i32 + 1i32 + 1i32 == 1i32 || *applet_name.offset(0) as libc::c_int == 'r' as i32)
  {
    /* Does not remove dependencies, no need to scan, just remove.
     * (compat note: this allows and strips .ko suffix)
     */
    rmmod(name);
    return 0i32;
  }
  /*
   * We used to have "is_remove != already_loaded(name)" check here, but
   *  modprobe -r pci:v00008086d00007010sv00000000sd00000000bc01sc01i80
   * won't unload modules (there are more than one)
   * which have this alias.
   */
  if !is_remove && already_loaded(name) != 0 {
    return 0i32;
  }
  options = 0 as *mut libc::c_char;
  if !is_remove {
    let mut opt_filename: *mut libc::c_char = xasprintf(
      b"/etc/modules/%s\x00" as *const u8 as *const libc::c_char,
      name,
    );
    options = xmalloc_open_read_close(opt_filename, 0 as *mut size_t) as *mut libc::c_char;
    if !options.is_null() {
      replace(
        options,
        '\n' as i32 as libc::c_char,
        ' ' as i32 as libc::c_char,
      );
    }
    if !cmdline_options.is_null() {
      /* NB: cmdline_options always have one leading ' '
       * (see main()), we remove it here */
      let mut op: *mut libc::c_char = xasprintf(
        if !options.is_null() {
          b"%s %s\x00" as *const u8 as *const libc::c_char
        } else {
          (b"%s %s\x00" as *const u8 as *const libc::c_char).offset(3)
        },
        cmdline_options.offset(1),
        options,
      );
      free(options as *mut libc::c_void);
      options = op
    }
    free(opt_filename as *mut libc::c_void);
    (*ptr_to_globals).module_load_options = options
  }
  if (*ptr_to_globals).module_count == 0 {
    /* Scan module directory. This is done only once.
     * It will attempt module load, and will exit(EXIT_SUCCESS)
     * on success.
     */
    (*ptr_to_globals).module_found_idx = -1i32;
    recursive_action(
      b".\x00" as *const u8 as *const libc::c_char,
      ACTION_RECURSE as libc::c_int as libc::c_uint,
      Some(
        fileAction
          as unsafe extern "C" fn(
            _: *const libc::c_char,
            _: *mut stat,
            _: *mut libc::c_void,
            _: libc::c_int,
          ) -> libc::c_int,
      ),
      None,
      name as *mut libc::c_void,
      0i32 as libc::c_uint,
    );
    /* Module was not found, or load failed, or is_remove */
    if (*ptr_to_globals).module_found_idx >= 0i32 {
      infovec = xzalloc(
        (2i32 as libc::c_ulong)
          .wrapping_mul(::std::mem::size_of::<*mut module_info>() as libc::c_ulong),
      ) as *mut *mut module_info; /* search for alias, not a plain module name */
      let ref mut fresh11 = *infovec.offset(0);
      *fresh11 = &mut *(*ptr_to_globals)
        .modinfo
        .offset((*ptr_to_globals).module_found_idx as isize) as *mut module_info
    } else {
      infovec = find_alias(name)
    }
  } else {
    infovec = find_alias(name)
  }
  if infovec.is_null() {
    /* module was found */
    /* both dirscan and find_alias found nothing */
    if !is_remove
      && !(1i32 != 0
        && (1i32 + 1i32 + 1i32 + 1i32 == 1i32
          || *applet_name.offset(0) as libc::c_int == 'd' as i32))
    {
      /* it wasn't rmmod or depmod */
      bb_error_msg(
        b"module \'%s\' not found\x00" as *const u8 as *const libc::c_char,
        name,
      );
      //TODO: _and_die()? or should we continue (un)loading modules listed on cmdline?
      /* "modprobe non-existing-module; echo $?" must print 1 */
      exitcode = 1i32
    }
  } else {
    /* There can be more than one module for the given alias. For example,
     * "pci:v00008086d00007010sv00000000sd00000000bc01sc01i80" matches
     * ata_piix because it has alias "pci:v00008086d00007010sv*sd*bc*sc*i*"
     * and ata_generic, it has alias "pci:v*d*sv*sd*bc01sc01i*"
     * Standard modprobe loads them both. We achieve it by returning
     * a *list* of modinfo pointers from find_alias().
     */
    /* modprobe -r? unload module(s) */
    if is_remove {
      infoidx = 0i32;
      loop {
        let fresh12 = infoidx;
        infoidx = infoidx + 1;
        info = *infovec.offset(fresh12 as isize);
        if info.is_null() {
          current_block = 2989495919056355252;
          break;
        }
        let mut r: libc::c_int = rmmod(bb_get_last_path_component_nostrip((*info).pathname));
        if r != 0i32 {
          current_block = 13810939424077242397;
          break;
        }
      }
    /* modprobe -r: we do not stop here -
     * continue to unload modules on which the module depends:
     * "-r --remove: option causes modprobe to remove a module.
     * If the modules it depends on are also unused, modprobe
     * will try to remove them, too."
     */
    } else {
      current_block = 2989495919056355252;
    }
    match current_block {
      13810939424077242397 => {}
      _ => {
        infoidx = 0i32;
        loop {
          let fresh13 = infoidx;
          infoidx = infoidx + 1;
          info = *infovec.offset(fresh13 as isize);
          if info.is_null() {
            break;
          }
          /* Iterate thru dependencies, trying to (un)load them */
          deps = str_2_list((*info).deps);
          s = deps;
          while *s != 0 {
            //if (strcmp(name, s) != 0) // N.B. do loops exist?
            process_module(s, 0 as *const libc::c_char);
            s = s.offset(strlen(s).wrapping_add(1i32 as libc::c_ulong) as isize)
          }
          free(deps as *mut libc::c_void);
          if is_remove {
            continue;
          }
          /* We are modprobe: load it */
          if !options.is_null()
            && !strstr(
              options,
              b"blacklist\x00" as *const u8 as *const libc::c_char,
            )
            .is_null()
          {
            continue;
          }
          if (*info).open_read_failed != 0 {
            /* We already tried it, didn't work. Don't try load again */
            exitcode = 1i32
          } else {
            *bb_errno = 0i32;
            if load_module((*info).pathname, options) != 0i32 {
              if 17i32 != *bb_errno {
                bb_error_msg(
                  b"\'%s\': %s\x00" as *const u8 as *const libc::c_char,
                  (*info).pathname,
                  moderror(*bb_errno),
                );
              }
              exitcode = 1i32
            }
          }
        }
      }
    }
  }
  free(infovec as *mut libc::c_void);
  free(options as *mut libc::c_void);
  return exitcode;
}
/* error */
/* For reference, module-init-tools v3.4 options:

# insmod
Usage: insmod filename [args]

# rmmod --help
Usage: rmmod [-fhswvV] modulename ...
 -f (or --force) forces a module unload, and may crash your
    machine. This requires the Forced Module Removal option
    when the kernel was compiled.
 -h (or --help) prints this help text
 -s (or --syslog) says use syslog, not stderr
 -v (or --verbose) enables more messages
 -V (or --version) prints the version code
 -w (or --wait) begins module removal even if it is used
    and will stop new users from accessing the module (so it
    should eventually fall to zero).

# modprobe
Usage: modprobe [-v] [-V] [-C config-file] [-d <dirname> ] [-n] [-i] [-q]
    [-b] [-o <modname>] [ --dump-modversions ] <modname> [parameters...]
modprobe -r [-n] [-i] [-v] <modulename> ...
modprobe -l -t <dirname> [ -a <modulename> ...]

# depmod --help
depmod 3.13 -- part of module-init-tools
depmod -[aA] [-n -e -v -q -V -r -u -w -m]
      [-b basedirectory] [forced_version]
depmod [-n -e -v -q -r -u -w] [-F kernelsyms] module1.ko module2.ko ...
If no arguments (except options) are given, "depmod -a" is assumed.
depmod will output a dependency list suitable for the modprobe utility.
Options:
    -a, --all           Probe all modules
    -A, --quick         Only does the work if there's a new module
    -e, --errsyms       Report not supplied symbols
    -m, --map           Create the legacy map files
    -n, --show          Write the dependency file on stdout only
    -P, --symbol-prefix Architecture symbol prefix
    -V, --version       Print the release version
    -v, --verbose       Enable verbose mode
    -w, --warn          Warn on duplicates
    -h, --help          Print this usage message
The following options are useful for people managing distributions:
    -b basedirectory
    --basedir basedirectory
                        Use an image of a module tree
    -F kernelsyms
    --filesyms kernelsyms
                        Use the file instead of the current kernel symbols
    -E Module.symvers
    --symvers Module.symvers
                        Use Module.symvers file to check symbol versions
*/
//usage:#if ENABLE_MODPROBE_SMALL
//usage:#define depmod_trivial_usage "[-n]"
//usage:#define depmod_full_usage "\n\n"
//usage:       "Generate modules.dep.bb"
//usage:     "\n"
//usage:     "\n	-n	Dry run: print file to stdout"
//usage:#define insmod_trivial_usage
//usage:	"FILE" IF_FEATURE_CMDLINE_MODULE_OPTIONS(" [SYMBOL=VALUE]...")
//usage:#define insmod_full_usage "\n\n"
//usage:       "Load kernel module"
//usage:#define rmmod_trivial_usage
//usage:       "MODULE..."
//usage:#define rmmod_full_usage "\n\n"
//usage:       "Unload kernel modules"
//usage:#define modprobe_trivial_usage
//usage:	"[-rq] MODULE" IF_FEATURE_CMDLINE_MODULE_OPTIONS(" [SYMBOL=VALUE]...")
//usage:#define modprobe_full_usage "\n\n"
//usage:       "	-r	Remove MODULE"
//usage:     "\n	-q	Quiet"
//usage:#endif
#[no_mangle]
pub unsafe extern "C" fn modprobe_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut exitcode: libc::c_int = 0;
  let mut uts: utsname = utsname {
    sysname: [0; 65],
    nodename: [0; 65],
    release: [0; 65],
    version: [0; 65],
    machine: [0; 65],
    domainname: [0; 65],
  };
  let mut options: *mut libc::c_char = 0 as *mut libc::c_char;
  let ref mut fresh14 =
    *(not_const_pp(&ptr_to_globals as *const *mut globals as *const libc::c_void)
      as *mut *mut globals);
  *fresh14 = xzalloc(::std::mem::size_of::<globals>() as libc::c_ulong) as *mut globals;
  asm!("" : : : "memory" : "volatile");
  /* Prevent ugly corner cases with no modules at all */
  (*ptr_to_globals).modinfo =
    xzalloc(::std::mem::size_of::<module_info>() as libc::c_ulong) as *mut module_info;
  if 1i32 + 1i32 + 1i32 + 1i32 == 2i32 && 1i32 != 0 && 1i32 != 0
    || 1i32 != 0
      && (1i32 + 1i32 + 1i32 + 1i32 == 1i32 || *applet_name.offset(0) as libc::c_int == 'd' as i32)
    || 1i32 != 0
      && (1i32 + 1i32 + 1i32 + 1i32 == 1i32 || *applet_name.offset(0) as libc::c_int == 'm' as i32)
  {
    /* Goto modules directory */
    xchdir(b"/lib/modules\x00" as *const u8 as *const libc::c_char);
    uname(&mut uts);
    /* never fails */
  }
  /* depmod? */
  if 1i32 != 0
    && (1i32 + 1i32 + 1i32 + 1i32 == 1i32 || *applet_name.offset(0) as libc::c_int == 'd' as i32)
  {
    /* Supported:
     * -n: print result to stdout
     * -a: process all modules (default)
     * optional VERSION parameter
     * Ignored:
     * -A: do work only if a module is newer than depfile
     * -e: report any symbols which a module needs
     *  which are not supplied by other modules or the kernel
     * -F FILE: System.map (symbols for -e)
     * -q, -r, -u: noop
     * Not supported:
     * -b BASEDIR: (TODO!) modules are in
     *  $BASEDIR/lib/modules/$VERSION
     * -m: create legacy "modules.*map" files (deprecated; in
     *  kmod's depmod, prints a warning message and continues)
     * -v: human readable deps to stdout
     * -V: version (don't want to support it - people may depend
     *  on it as an indicator of "standard" depmod)
     * -h: help (well duh)
     * module1.o module2.o parameters (just ignored for now)
     */
    getopt32(
      argv,
      b"naAeF:qru\x00" as *const u8 as *const libc::c_char,
      0 as *mut libc::c_void,
    );
    argv = argv.offset(optind as isize);
    /* if (argv[0] && argv[1]) bb_show_usage(); */
    /* Goto $VERSION directory */
    xchdir(if !(*argv.offset(0)).is_null() {
      *argv.offset(0)
    } else {
      uts.release.as_mut_ptr()
    });
    /* Force full module scan by asking to find a bogus module.
     * This will generate modules.dep.bb as a side effect. */
    process_module(
      b"/\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
      0 as *const libc::c_char,
    );
    return ((*ptr_to_globals).wrote_dep_bb_ok == 0) as libc::c_int;
  }
  /* modprobe, insmod, rmmod require at least one argument */
  /* only -q (quiet) and -r (rmmod),
   * the rest are accepted and ignored (compat) */
  getopt32(
    argv,
    b"^qrfsvwb\x00-1\x00" as *const u8 as *const libc::c_char,
  );
  argv = argv.offset(optind as isize);
  if 1i32 != 0
    && (1i32 + 1i32 + 1i32 + 1i32 == 1i32 || *applet_name.offset(0) as libc::c_int == 'm' as i32)
  {
    /* Goto $VERSION directory */
    xchdir(uts.release.as_mut_ptr());
  }
  /* are we rmmod? -> simulate modprobe -r, but don't bother the flag if
   * there're no other applets here */
  if 1i32 != 0
    && (1i32 + 1i32 + 1i32 + 1i32 == 1i32 || *applet_name.offset(0) as libc::c_int == 'r' as i32)
  {
    if !(1i32 + 1i32 + 1i32 + 1i32 == 1i32) {
      option_mask32 |= OPT_r as libc::c_int as libc::c_uint
    }
  } else if 1i32 == 0 || option_mask32 & OPT_r as libc::c_int as libc::c_uint == 0 {
    /* If not rmmod/-r, parse possible module options given on command line.
     * insmod/modprobe takes one module name, the rest are parameters. */
    let mut arg: *mut *mut libc::c_char = argv;
    loop {
      arg = arg.offset(1);
      if (*arg).is_null() {
        break;
      }
      /* Enclose options in quotes */
      let mut s: *mut libc::c_char = options;
      options = xasprintf(
        b"%s \"%s\"\x00" as *const u8 as *const libc::c_char,
        if !s.is_null() {
          s
        } else {
          b"\x00" as *const u8 as *const libc::c_char
        },
        *arg,
      );
      free(s as *mut libc::c_void);
      *arg = 0 as *mut libc::c_char
    }
  }
  if 1i32 != 0
    && (1i32 + 1i32 + 1i32 + 1i32 == 1i32 || *applet_name.offset(0) as libc::c_int == 'i' as i32)
  {
    let mut len: size_t = 0;
    let mut map: *mut libc::c_void = 0 as *mut libc::c_void;
    len = if -1i32 as ssize_t >0{
      -1i32 as ssize_t
    } else {
      !((1i32 as ssize_t)
        << (::std::mem::size_of::<ssize_t>() as libc::c_ulong)
          .wrapping_mul(8i32 as libc::c_ulong)
          .wrapping_sub(1i32 as libc::c_ulong))
    } as size_t;
    map = xmalloc_open_zipped_read_close(*argv, &mut len);
    if map.is_null() {
      bb_perror_msg_and_die(
        b"can\'t read \'%s\'\x00" as *const u8 as *const libc::c_char,
        *argv,
      );
    }
    if syscall(
      175i32 as libc::c_long,
      map,
      len,
      if !options.is_null() {
        options
      } else {
        b"\x00" as *const u8 as *const libc::c_char
      },
    ) != 0     {
      bb_error_msg_and_die(
        b"can\'t insert \'%s\': %s\x00" as *const u8 as *const libc::c_char,
        *argv,
        moderror(*bb_errno),
      );
    }
    return 0i32;
  }
  /* Try to load modprobe.dep.bb */
  if !(1i32 != 0
    && (1i32 + 1i32 + 1i32 + 1i32 == 1i32 || *applet_name.offset(0) as libc::c_int == 'r' as i32))
  {
    load_dep_bb();
  }
  /* Load/remove modules.
   * Only rmmod/modprobe -r loops here, insmod/modprobe has only argv[0] */
  exitcode = 0i32;
  loop {
    exitcode |= process_module(*argv, options);
    argv = argv.offset(1);
    if (*argv).is_null() {
      break;
    }
  }
  return exitcode;
  /* MODPROBE || INSMOD || RMMOD */
}
/* MOD_APPLET_CNT > 0 */
