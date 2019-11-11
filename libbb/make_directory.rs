use crate::libbb::ptr_to_globals::bb_errno;
use libc;
use libc::chmod;
use libc::mode_t;
use libc::printf;
use libc::stat;
use libc::umask;
extern "C" {

  #[no_mangle]
  fn mkdir(__path: *const libc::c_char, __mode: mode_t) -> libc::c_int;



  #[no_mangle]
  fn bb_perror_msg(s: *const libc::c_char, _: ...);
}

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

#[no_mangle]
pub unsafe extern "C" fn bb_make_directory(
  mut path: *mut libc::c_char,
  mut mode: libc::c_long,
  mut flags: libc::c_int,
) -> libc::c_int {
  let mut current_block: u64;
  let mut cur_mask: mode_t = 0;
  let mut org_mask: mode_t = 0;
  let mut fail_msg: *const libc::c_char = 0 as *const libc::c_char;
  let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut c: libc::c_char = 0;
  let mut st: stat = std::mem::zeroed();
  /* "path" can be a result of dirname().
   * dirname("no_slashes") returns ".", possibly read-only.
   * musl dirname() can return read-only "/" too.
   * We need writable string. And for "/", "." (and ".."?)
   * nothing needs to be created anyway.
   */
  if *path.offset(0) as libc::c_int == '/' as i32 && *path.offset(1) == 0 {
    return 0i32;
  }
  if *path.offset(0) as libc::c_int == '.' as i32 {
    if *path.offset(1) as libc::c_int == '\u{0}' as i32 {
      return 0i32;
    }
    /* "." */
    //		if (path[1] == '.' && path[2] == '\0')
    //			return 0; /* ".." */
  } /* while (1) */
  cur_mask = -1i64 as mode_t;
  org_mask = cur_mask;
  s = path;
  loop {
    c = '\u{0}' as i32 as libc::c_char;
    if flags & FILEUTILS_RECUR as libc::c_int != 0 {
      /* Get the parent */
      /* Bypass leading non-'/'s and then subsequent '/'s */
      while *s != 0 {
        if *s as libc::c_int == '/' as i32 {
          loop {
            s = s.offset(1); /* Save the current char */
            if !(*s as libc::c_int == '/' as i32) {
              break; /* and replace it with nul */
            }
          }
          c = *s;
          *s = '\u{0}' as i32 as libc::c_char;
          break;
        } else {
          s = s.offset(1)
        }
      }
    }
    if c as libc::c_int != '\u{0}' as i32 {
      /* Intermediate dirs: must have wx for user */
      if cur_mask == -1i64 as mode_t {
        /* wasn't done yet? */
        let mut new_mask: mode_t = 0;
        org_mask = umask(0i32 as mode_t);
        cur_mask = 0i32 as mode_t;
        /* Clear u=wx in umask - this ensures
         * they won't be cleared on mkdir */
        new_mask = org_mask & !(0o300i32 as mode_t);
        //bb_error_msg("org_mask:%o cur_mask:%o", org_mask, new_mask);
        if new_mask != cur_mask {
          cur_mask = new_mask;
          umask(new_mask);
        }
      }
    } else if org_mask != cur_mask {
      cur_mask = org_mask;
      umask(org_mask);
    }
    /* Last component: uses original umask */
    //bb_error_msg("1 org_mask:%o", org_mask);
    //bb_error_msg("mkdir '%s'", path);
    if mkdir(path, 0o777i32 as mode_t) < 0i32 {
      /* If we failed for any other reason than the directory
       * already exists, output a diagnostic and return -1 */
      if *bb_errno != 17i32 && *bb_errno != 21i32
        || flags & FILEUTILS_RECUR as libc::c_int == 0
        || (stat(path, &mut st) < 0i32
          || !(st.st_mode & 0o170000i32 as libc::c_uint == 0o40000i32 as libc::c_uint))
      {
        fail_msg = b"create\x00" as *const u8 as *const libc::c_char;
        current_block = 6560072651652764009;
        break;
      } else if c == 0
      /* Since the directory exists, don't attempt to change
       * permissions if it was the full target.  Note that
       * this is not an error condition. */
      {
        current_block = 7995648291273452906;
        break;
      }
    } else if flags & FILEUTILS_VERBOSE as libc::c_int != 0 {
      printf(
        b"created directory: \'%s\'\n\x00" as *const u8 as *const libc::c_char,
        path,
      );
    }
    if c == 0 {
      /* Done.  If necessary, update perms on the newly
       * created directory.  Failure to update here _is_
       * an error. */
      if !(mode != -1i32 as libc::c_long) {
        current_block = 7995648291273452906;
        break;
      }
      //bb_error_msg("chmod 0%03lo mkdir '%s'", mode, path);
      if !(chmod(path, mode as mode_t) < 0i32) {
        current_block = 7995648291273452906;
        break;
      }
      fail_msg = b"set permissions of\x00" as *const u8 as *const libc::c_char;
      if !(flags & FILEUTILS_IGNORE_CHMOD_ERR as libc::c_int != 0) {
        current_block = 6560072651652764009;
        break;
      }
      flags = 0i32;
      current_block = 14469951070566118589;
      break;
    } else {
      /* Remove any inserted nul from the path (recursive mode) */
      *s = c
    }
  }
  match current_block {
    6560072651652764009 => {
      flags = -1i32;
      current_block = 14469951070566118589;
    }
    7995648291273452906 => {
      flags = 0i32;
      current_block = 3427267834250323188;
    }
    _ => {}
  }
  match current_block {
    14469951070566118589 => {
      bb_perror_msg(
        b"can\'t %s directory \'%s\'\x00" as *const u8 as *const libc::c_char,
        fail_msg,
        path,
      );
    }
    _ => {}
  }
  //bb_error_msg("2 org_mask:%o", org_mask);
  if org_mask != cur_mask {
    umask(org_mask);
  }
  return flags;
}
