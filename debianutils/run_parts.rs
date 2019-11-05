use libc;
extern "C" {
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn puts(__s: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t, __compar: __compar_fn_t);
  #[no_mangle]
  fn umask(__mask: __mode_t) -> __mode_t;
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
    fileAction: Option<
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
  fn bb_basename(name: *const libc::c_char) -> *const libc::c_char;
  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn xstrtou_range(
    str: *const libc::c_char,
    b: libc::c_int,
    l: libc::c_uint,
    u: libc::c_uint,
  ) -> libc::c_uint;
  #[no_mangle]
  fn spawn_and_wait(argv: *mut *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  static mut option_mask32: uint32_t;
  #[no_mangle]
  fn getopt32long(
    argv: *mut *mut libc::c_char,
    optstring: *const libc::c_char,
    longopts: *const libc::c_char,
    _: ...
  ) -> uint32_t;
  #[no_mangle]
  fn llist_pop(elm: *mut *mut llist_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xfunc_die() -> !;
  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_perror_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
}

pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
use crate::librb::uint32_t;
use crate::librb::size_t;

use crate::librb::stat;
use crate::librb::__compar_fn_t;
pub type C2RustUnnamed = libc::c_uint;
pub const ACTION_DANGLING_OK: C2RustUnnamed = 64;
pub const ACTION_QUIET: C2RustUnnamed = 32;
pub const ACTION_DEPTHFIRST: C2RustUnnamed = 8;
pub const ACTION_FOLLOWLINKS_L0: C2RustUnnamed = 4;
pub const ACTION_FOLLOWLINKS: C2RustUnnamed = 2;
pub const ACTION_RECURSE: C2RustUnnamed = 1;
use crate::libbb::llist::llist_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub names: *mut *mut libc::c_char,
  pub cur: libc::c_int,
  pub cmd: [*mut libc::c_char; 2],
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const NUM_CMD: C2RustUnnamed_0 = 123;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const OPT_l: C2RustUnnamed_1 = 32;
pub const OPT_e: C2RustUnnamed_1 = 16;
pub const OPT_t: C2RustUnnamed_1 = 8;
pub const OPT_r: C2RustUnnamed_1 = 4;
pub const OPT_u: C2RustUnnamed_1 = 2;
pub const OPT_a: C2RustUnnamed_1 = 1;
#[inline(always)]
unsafe extern "C" fn bb_ascii_isalnum(mut a: libc::c_uchar) -> libc::c_int {
  let mut b: libc::c_uchar = (a as libc::c_int - '0' as i32) as libc::c_uchar;
  if b as libc::c_int <= 9i32 {
    return (b as libc::c_int <= 9i32) as libc::c_int;
  }
  b = ((a as libc::c_int | 0x20i32) - 'a' as i32) as libc::c_uchar;
  return (b as libc::c_int <= 'z' as i32 - 'a' as i32) as libc::c_int;
}
/* Is this a valid filename (upper/lower alpha, digits,
 * underscores, and hyphens only?)
 */
unsafe extern "C" fn invalid_name(mut c: *const libc::c_char) -> bool {
  c = bb_basename(c);
  while *c as libc::c_int != 0
    && (bb_ascii_isalnum(*c as libc::c_uchar) != 0
      || *c as libc::c_int == '_' as i32
      || *c as libc::c_int == '-' as i32)
  {
    c = c.offset(1)
  }
  return *c != 0;
  /* TRUE (!0) if terminating NUL is not reached */
}
unsafe extern "C" fn bb_alphasort(
  mut p1: *const libc::c_void,
  mut p2: *const libc::c_void,
) -> libc::c_int {
  let mut r: libc::c_int = strcmp(
    *(p1 as *mut *mut libc::c_char),
    *(p2 as *mut *mut libc::c_char),
  );
  return if option_mask32 & OPT_r as libc::c_int as libc::c_uint != 0 {
    -r
  } else {
    r
  };
}
unsafe extern "C" fn act(
  mut file: *const libc::c_char,
  mut statbuf: *mut stat,
  mut _args: *mut libc::c_void,
  mut depth: libc::c_int,
) -> libc::c_int {
  if depth == 1i32 {
    return 1i32;
  }
  if depth == 2i32
    && ((*statbuf).st_mode & (0o100000i32 | 0o120000i32) as libc::c_uint == 0
      || invalid_name(file) as libc::c_int != 0
      || option_mask32 & OPT_l as libc::c_int as libc::c_uint == 0 && access(file, 1i32) != 0i32)
  {
    return 2i32;
  }
  let ref mut fresh0 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).names;
  *fresh0 = xrealloc_vector_helper(
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).names as *mut libc::c_void,
    ((::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong) << 8i32)
      .wrapping_add(4i32 as libc::c_ulong) as libc::c_uint,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).cur,
  ) as *mut *mut libc::c_char;
  let ref mut fresh1 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).cur;
  let fresh2 = *fresh1;
  *fresh1 = *fresh1 + 1;
  let ref mut fresh3 = *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .names
    .offset(fresh2 as isize);
  *fresh3 = xstrdup(file);
  /*names[cur] = NULL; - xrealloc_vector did it */
  return 1i32;
}
static mut runparts_longopts: [libc::c_char; 55] = [
  97, 114, 103, 0, 1, 97, 117, 109, 97, 115, 107, 0, 1, 117, 114, 101, 118, 101, 114, 115, 101, 0,
  0, -16, 116, 101, 115, 116, 0, 0, -15, 101, 120, 105, 116, 45, 111, 110, 45, 101, 114, 114, 111,
  114, 0, 0, -14, 108, 105, 115, 116, 0, 0, -13, 0,
];
#[no_mangle]
pub unsafe extern "C" fn run_parts_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut umask_p: *const libc::c_char = b"22\x00" as *const u8 as *const libc::c_char;
  let mut arg_list: *mut llist_t = 0 as *mut llist_t;
  let mut n: libc::c_uint = 0;
  let mut ret: libc::c_int = 0;
  /* We require exactly one argument: the directory name */
  getopt32long(
    argv,
    b"^a:*u:\x00=1\x00" as *const u8 as *const libc::c_char,
    runparts_longopts.as_ptr(),
    &mut arg_list as *mut *mut llist_t,
    &mut umask_p as *mut *const libc::c_char,
  );
  umask(xstrtou_range(
    umask_p,
    8i32,
    0i32 as libc::c_uint,
    0o7777i32 as libc::c_uint,
  ));
  n = 1i32 as libc::c_uint;
  while !arg_list.is_null() && n < NUM_CMD as libc::c_int as libc::c_uint {
    let fresh4 = n;
    n = n.wrapping_add(1);
    let ref mut fresh5 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).cmd[fresh4 as usize];
    *fresh5 = llist_pop(&mut arg_list) as *mut libc::c_char
  }
  /* cmd[n] = NULL; - is already zeroed out */
  /* run-parts has to sort executables by name before running them */
  recursive_action(
    *argv.offset(optind as isize),
    (ACTION_RECURSE as libc::c_int | ACTION_FOLLOWLINKS as libc::c_int) as libc::c_uint,
    Some(
      act
        as unsafe extern "C" fn(
          _: *const libc::c_char,
          _: *mut stat,
          _: *mut libc::c_void,
          _: libc::c_int,
        ) -> libc::c_int,
    ),
    Some(
      act
        as unsafe extern "C" fn(
          _: *const libc::c_char,
          _: *mut stat,
          _: *mut libc::c_void,
          _: libc::c_int,
        ) -> libc::c_int,
    ),
    0 as *mut libc::c_void,
    1i32 as libc::c_uint,
  );
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .names
    .is_null()
  {
    return 0i32;
  }
  qsort(
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).names as *mut libc::c_void,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).cur as size_t,
    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
    Some(
      bb_alphasort
        as unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> libc::c_int,
    ),
  );
  n = 0i32 as libc::c_uint;
  loop {
    let ref mut fresh6 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).names;
    let fresh7 = *fresh6;
    *fresh6 = (*fresh6).offset(1);
    let mut name: *mut libc::c_char = *fresh7;
    if name.is_null() {
      break;
    }
    if option_mask32 & (OPT_t as libc::c_int | OPT_l as libc::c_int) as libc::c_uint != 0 {
      puts(name);
    } else {
      let ref mut fresh8 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).cmd[0];
      *fresh8 = name;
      ret = spawn_and_wait(
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
          .cmd
          .as_mut_ptr(),
      );
      if ret == 0i32 {
        continue;
      }
      n = 1i32 as libc::c_uint;
      if ret < 0i32 {
        bb_perror_msg(
          b"can\'t execute \'%s\'\x00" as *const u8 as *const libc::c_char,
          name,
        );
      } else {
        /* ret > 0 */
        bb_error_msg(
          b"%s: exit status %u\x00" as *const u8 as *const libc::c_char,
          name,
          ret & 0xffi32,
        );
      }
      if option_mask32 & OPT_e as libc::c_int as libc::c_uint != 0 {
        xfunc_die();
      }
    }
  }
  return n as libc::c_int;
}
