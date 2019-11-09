use c2rust_bitfields;
use c2rust_bitfields::BitfieldStruct;
use libc;
use libc::open;
use libc::unlink;

use libc::free;
extern "C" {

  #[no_mangle]
  fn fnmatch(
    __pattern: *const libc::c_char,
    __name: *const libc::c_char,
    __flags: libc::c_int,
  ) -> libc::c_int;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn rmdir(__path: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn opendir(__name: *const libc::c_char) -> *mut DIR;
  #[no_mangle]
  fn closedir(__dirp: *mut DIR) -> libc::c_int;
  #[no_mangle]
  fn readdir(__dirp: *mut DIR) -> *mut dirent;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn puts(__s: *const libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn exit(_: libc::c_int) -> !;
  #[no_mangle]
  fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;

  #[no_mangle]
  fn time(__timer: *mut time_t) -> time_t;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
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
  fn bb_basename(name: *const libc::c_char) -> *const libc::c_char;
  #[no_mangle]
  fn xstat(pathname: *const libc::c_char, buf: *mut stat);
  #[no_mangle]
  fn count_strstr(str: *const libc::c_char, sub: *const libc::c_char) -> libc::c_uint;
  #[no_mangle]
  fn xmalloc_substitute_string(
    src: *const libc::c_char,
    count: libc::c_int,
    sub: *const libc::c_char,
    repl: *const libc::c_char,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_arg_max() -> libc::c_uint;
  #[no_mangle]
  fn xatoull_sfx(str: *const libc::c_char, sfx: *const suffix_mult) -> libc::c_ulonglong;
  #[no_mangle]
  fn xatoull(str: *const libc::c_char) -> libc::c_ulonglong;
  #[no_mangle]
  fn bb_strtou(
    arg: *const libc::c_char,
    endp: *mut *mut libc::c_char,
    base: libc::c_int,
  ) -> libc::c_uint;
  /* Specialized: */
  /* Using xatoi() instead of naive atoi() is not always convenient -
   * in many places people want *non-negative* values, but store them
   * in signed int. Therefore we need this one:
   * dies if input is not in [0, INT_MAX] range. Also will reject '-0' etc.
   * It should really be named xatoi_nonnegative (since it allows 0),
   * but that would be too long.
   */
  #[no_mangle]
  fn xatoi_positive(numstr: *const libc::c_char) -> libc::c_int;
  /* These parse entries in /etc/passwd and /etc/group.  This is desirable
   * for BusyBox since we want to avoid using the glibc NSS stuff, which
   * increases target size and is often not needed on embedded systems.  */
  #[no_mangle]
  fn xuname2uid(name: *const libc::c_char) -> libc::c_long;
  #[no_mangle]
  fn xgroup2gid(name: *const libc::c_char) -> libc::c_long;
  /* ***********************************************************************/
  /* spawn_and_wait/run_nofork_applet/run_applet_no_and_exit need to work */
  /* carefully together to reinit some global state while not disturbing  */
  /* other. Be careful if you change them. Consult docs/nofork_noexec.txt */
  /* ***********************************************************************/
  /* Same as wait4pid(spawn(argv)), but with NOFORK/NOEXEC if configured: */
  #[no_mangle]
  fn spawn_and_wait(argv: *mut *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);
  /* Returns -1 if input is invalid. current_mode is a base for e.g. "u+rw" */
  #[no_mangle]
  fn bb_parse_mode(s: *const libc::c_char, cur_mode: libc::c_uint) -> libc::c_int;
  #[no_mangle]
  fn index_in_strings(strings: *const libc::c_char, key: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  static bb_msg_requires_arg: [libc::c_char; 0];
  #[no_mangle]
  static bb_msg_invalid_arg_to: [libc::c_char; 0];
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
  #[no_mangle]
  fn xregcomp(preg: *mut regex_t, regex: *const libc::c_char, cflags: libc::c_int);
  #[no_mangle]
  fn regexec(
    __preg: *const regex_t,
    __string: *const libc::c_char,
    __nmatch: size_t,
    __pmatch: *mut regmatch_t,
    __eflags: libc::c_int,
  ) -> libc::c_int;
}







/* NB: unaligned parameter should be a pointer, aligned one -
 * a lvalue. This makes it more likely to not swap them by mistake
 */
/* #elif ... - add your favorite arch today! */
/* Unaligned, fixed-endian accessors */
/* unxz needs an aligned fixed-endian accessor.
 * (however, the compiler does not realize it's aligned, the cast is still necessary)
 */
/* ---- Size-saving "small" ints (arch-dependent) ----------- */
/* add other arches which benefit from this... */
use crate::librb::smallint;
pub type smalluint = libc::c_uchar;
use libc::gid_t;
 use libc::ino_t;
use libc::off_t;
use crate::librb::size_t;
use libc::uid_t;
use libc::dirent;
use libc::DIR;

use libc::mode_t;
use libc::stat;
use libc::time_t;

pub type C2RustUnnamed = libc::c_uint;
pub const ACTION_DANGLING_OK: C2RustUnnamed = 64;
pub const ACTION_QUIET: C2RustUnnamed = 32;
pub const ACTION_DEPTHFIRST: C2RustUnnamed = 8;
pub const ACTION_FOLLOWLINKS_L0: C2RustUnnamed = 4;
pub const ACTION_FOLLOWLINKS: C2RustUnnamed = 2;
pub const ACTION_RECURSE: C2RustUnnamed = 1;
pub type recurse_flags_t = u8;
/* Last element is marked by mult == 0 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct suffix_mult {
  pub suffix: [libc::c_char; 4],
  pub mult: libc::c_uint,
}
//extern const int const_int_1;
/* This struct is deliberately not defined. */
/* See docs/keep_data_small.txt */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub xdev_dev: *mut libc::dev_t,
  pub xdev_count: libc::c_int,
  pub minmaxdepth: [libc::c_int; 2],
  pub actions: *mut *mut *mut action,
  pub need_print: smallint,
  pub xdev_on: smallint,
  pub exitstatus: smalluint,
  pub recurse_flags: recurse_flags_t,
  pub max_argv_len: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct action {
  pub f: action_fp,
  pub invert: bool,
}
pub type action_fp = Option<
  unsafe extern "C" fn(_: *const libc::c_char, _: *const stat, _: *mut libc::c_void) -> libc::c_int,
>;
pub type reg_syntax_t = libc::c_ulong;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct re_pattern_buffer {
  pub buffer: *mut libc::c_uchar,
  pub allocated: libc::c_ulong,
  pub used: libc::c_ulong,
  pub syntax: reg_syntax_t,
  pub fastmap: *mut libc::c_char,
  pub translate: *mut libc::c_uchar,
  pub re_nsub: size_t,
  #[bitfield(name = "can_be_null", ty = "libc::c_uint", bits = "0..=0")]
  #[bitfield(name = "regs_allocated", ty = "libc::c_uint", bits = "1..=2")]
  #[bitfield(name = "fastmap_accurate", ty = "libc::c_uint", bits = "3..=3")]
  #[bitfield(name = "no_sub", ty = "libc::c_uint", bits = "4..=4")]
  #[bitfield(name = "not_bol", ty = "libc::c_uint", bits = "5..=5")]
  #[bitfield(name = "not_eol", ty = "libc::c_uint", bits = "6..=6")]
  #[bitfield(name = "newline_anchor", ty = "libc::c_uint", bits = "7..=7")]
  pub can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [u8; 1],
  #[bitfield(padding)]
  pub c2rust_padding: [u8; 7],
}
pub type regex_t = re_pattern_buffer;
pub type regoff_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regmatch_t {
  pub rm_so: regoff_t,
  pub rm_eo: regoff_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct action_print {
  pub a: action,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct action_name {
  pub a: action,
  pub pattern: *const libc::c_char,
  pub iname: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct action_path {
  pub a: action,
  pub pattern: *const libc::c_char,
  pub ipath: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct action_regex {
  pub a: action,
  pub compiled_pattern: regex_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct action_print0 {
  pub a: action,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct action_type {
  pub a: action,
  pub type_mask: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct action_executable {
  pub a: action,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct action_perm {
  pub a: action,
  pub perm_char: libc::c_char,
  pub perm_mask: mode_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct action_mtime {
  pub a: action,
  pub mtime_char: libc::c_char,
  pub mtime_days: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct action_mmin {
  pub a: action,
  pub mmin_char: libc::c_char,
  pub mmin_mins: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct action_newer {
  pub a: action,
  pub newer_mtime: time_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct action_inum {
  pub a: action,
  pub inode_num: ino_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct action_user {
  pub a: action,
  pub uid: uid_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct action_size {
  pub a: action,
  pub size_char: libc::c_char,
  pub size: off_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct action_paren {
  pub a: action,
  pub subexpr: *mut *mut *mut action,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct action_prune {
  pub a: action,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct action_quit {
  pub a: action,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct action_delete {
  pub a: action,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct action_empty {
  pub a: action,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct action_exec {
  pub a: action,
  pub exec_argv: *mut *mut libc::c_char,
  pub subst_count: *mut libc::c_uint,
  pub exec_argc: libc::c_int,
  pub filelist: *mut *mut libc::c_char,
  pub filelist_idx: libc::c_int,
  pub file_len: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct action_group {
  pub a: action,
  pub gid: gid_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct action_links {
  pub a: action,
  pub links_char: libc::c_char,
  pub links_count: libc::c_int,
}
/* Say no to GCCism */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pp_locals {
  pub appp: *mut *mut *mut action,
  pub cur_group: libc::c_uint,
  pub cur_action: libc::c_uint,
  pub invert_flag: bool,
}
pub const PARM_links: C2RustUnnamed_0 = 33;
pub const PARM_size: C2RustUnnamed_0 = 32;
pub const PARM_group: C2RustUnnamed_0 = 31;
pub const PARM_user: C2RustUnnamed_0 = 30;
pub const PARM_inum: C2RustUnnamed_0 = 29;
pub const PARM_newer: C2RustUnnamed_0 = 28;
pub const PARM_mmin: C2RustUnnamed_0 = 27;
pub const PARM_mtime: C2RustUnnamed_0 = 26;
pub const PARM_perm: C2RustUnnamed_0 = 25;
pub const PARM_executable: C2RustUnnamed_0 = 16;
pub const PARM_type: C2RustUnnamed_0 = 24;
pub const PARM_regex: C2RustUnnamed_0 = 23;
pub const PARM_ipath: C2RustUnnamed_0 = 22;
/* -wholename is a synonym for -path */
/* We support it because Linux kernel's "make tags" uses it */
pub const PARM_wholename: C2RustUnnamed_0 = 21;
pub const PARM_path: C2RustUnnamed_0 = 20;
pub const PARM_iname: C2RustUnnamed_0 = 19;
/* All options/actions starting from here require argument */
pub const PARM_name: C2RustUnnamed_0 = 18;
pub const PARM_char_brace: C2RustUnnamed_0 = 17;
pub const PARM_exec: C2RustUnnamed_0 = 15;
pub const PARM_empty: C2RustUnnamed_0 = 14;
pub const PARM_delete: C2RustUnnamed_0 = 13;
pub const PARM_quit: C2RustUnnamed_0 = 12;
pub const PARM_prune: C2RustUnnamed_0 = 11;
pub const PARM_print0: C2RustUnnamed_0 = 10;
pub const PARM_print: C2RustUnnamed_0 = 9;
pub const PARM_not: C2RustUnnamed_0 = 8;
pub const PARM_char_not: C2RustUnnamed_0 = 5;
pub const PARM_or: C2RustUnnamed_0 = 7;
pub const PARM_o: C2RustUnnamed_0 = 4;
pub const PARM_and: C2RustUnnamed_0 = 6;
pub const PARM_a: C2RustUnnamed_0 = 3;
pub const OPT_DEPTH: C2RustUnnamed_0 = 2;
pub const OPT_MINDEPTH: C2RustUnnamed_0 = 34;
pub const OPT_XDEV: C2RustUnnamed_0 = 1;
pub const OPT_FOLLOW: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const OPT_MAXDEPTH: C2RustUnnamed_0 = 35;
#[inline(always)]
unsafe extern "C" fn xatoul(mut str: *const libc::c_char) -> libc::c_ulong {
  return xatoull(str) as libc::c_ulong;
}
/* we have to zero it out because of NOEXEC */
/* Return values of ACTFs ('action functions') are a bit mask:
 * bit 1=1: prune (use SKIP constant for setting it)
 * bit 0=1: matched successfully (TRUE)
 */
unsafe extern "C" fn exec_actions(
  mut appp: *mut *mut *mut action,
  mut fileName: *const libc::c_char,
  mut statbuf: *const stat,
) -> libc::c_int {
  let mut cur_group: libc::c_int = 0;
  let mut cur_action: libc::c_int = 0;
  let mut rc: libc::c_int = 0i32;
  let mut app: *mut *mut action = 0 as *mut *mut action;
  let mut ap: *mut action = 0 as *mut action;
  /* "action group" is a set of actions ANDed together.
   * groups are ORed together.
   * We simply evaluate each group until we find one in which all actions
   * succeed. */
  /* -prune is special: if it is encountered, then we won't
   * descend into current directory. It doesn't matter whether
   * action group (in which -prune sits) will succeed or not:
   * find * -prune -name 'f*' -o -name 'm*' -- prunes every dir
   * find * -name 'f*' -o -prune -name 'm*' -- prunes all dirs
   *     not starting with 'f' */
  /* We invert TRUE bit (bit 0). Now 1 there means 'failure'.
   * and bitwise OR in "rc |= TRUE ^ ap->f()" will:
   * (1) make SKIP (-prune) bit stick; and (2) detect 'failure'.
   * On return, bit is restored.  */
  cur_group = -1i32; /* 'success' so far, clear TRUE bit */
  loop {
    cur_group += 1; /* restore TRUE bit */
    app = *appp.offset(cur_group as isize);
    if app.is_null() {
      break;
    }
    rc &= !1i32;
    cur_action = -1i32;
    loop {
      cur_action += 1;
      ap = *app.offset(cur_action as isize);
      if ap.is_null() {
        /* all actions in group were successful */
        return rc ^ 1i32;
      }
      rc |= 1i32
        ^ (*ap).f.expect("non-null function pointer")(fileName, statbuf, ap as *mut libc::c_void);
      if (*ap).invert {
        rc ^= 1i32
      }
      if rc & 1i32 != 0 {
        break;
      }
    }
  }
  return rc ^ 1i32;
  /* restore TRUE bit */
}
unsafe extern "C" fn func_name(
  mut fileName: *const libc::c_char,
  mut _statbuf: *const stat,
  mut ap: *mut action_name,
) -> libc::c_int {
  let mut r: libc::c_int = 0;
  let mut tmp: *const libc::c_char = bb_basename(fileName);
  /* GNU findutils: find DIR/ -name DIR
   * prints "DIR/" (DIR// prints "DIR//" etc).
   * Need to strip trailing "/".
   * Such names can come only from top-level names, but
   * we can't do this before recursive_action() call,
   * since then "find FILE/ -name FILE"
   * would also work (on non-directories), which is wrong.
   */
  let mut trunc_slash: *mut libc::c_char = 0 as *mut libc::c_char;
  if *tmp as libc::c_int == '\u{0}' as i32 {
    /* "foo/bar/[//...]" */
    while tmp != fileName && *tmp.offset(-1i32 as isize) as libc::c_int == '/' as i32 {
      tmp = tmp.offset(-1)
    }
    if tmp == fileName {
      /* entire fileName is "//.."? */
      /* yes, convert "//..." to "/"
       * Testcases:
       * find / -maxdepth 1 -name /: prints /
       * find // -maxdepth 1 -name /: prints //
       * find / -maxdepth 1 -name //: prints nothing
       * find // -maxdepth 1 -name //: prints nothing
       */
      if *tmp.offset(1) != 0 {
        trunc_slash = (tmp as *mut libc::c_char).offset(1)
      }
    } else {
      /* no, it's "foo/bar/[//...]", go back to 'b' */
      trunc_slash = tmp as *mut libc::c_char;
      while tmp != fileName && *tmp.offset(-1i32 as isize) as libc::c_int != '/' as i32 {
        tmp = tmp.offset(-1)
      }
    }
  }
  /* Was using FNM_PERIOD flag too,
   * but somewhere between 4.1.20 and 4.4.0 GNU find stopped using it.
   * find -name '*foo' should match .foo too:
   */
  if !trunc_slash.is_null() {
    *trunc_slash = '\u{0}' as i32 as libc::c_char
  } /* no match */
  r = fnmatch(
    (*ap).pattern,
    tmp,
    if (*ap).iname as libc::c_int != 0 {
      (1i32) << 4i32
    } else {
      0i32
    },
  ); /* match doesn't start at pos 0 */
  if !trunc_slash.is_null() {
    *trunc_slash = '/' as i32 as libc::c_char
  } /* match doesn't end exactly at end of pathname */
  return (r == 0i32) as libc::c_int;
}
unsafe extern "C" fn func_path(
  mut fileName: *const libc::c_char,
  mut _statbuf: *const stat,
  mut ap: *mut action_path,
) -> libc::c_int {
  return (fnmatch(
    (*ap).pattern,
    fileName,
    if (*ap).ipath as libc::c_int != 0 {
      (1i32) << 4i32
    } else {
      0i32
    },
  ) == 0i32) as libc::c_int;
}
unsafe extern "C" fn func_regex(
  mut fileName: *const libc::c_char,
  mut _statbuf: *const stat,
  mut ap: *mut action_regex,
) -> libc::c_int {
  let mut match_0: regmatch_t = regmatch_t { rm_so: 0, rm_eo: 0 };
  if regexec(
    &mut (*ap).compiled_pattern,
    fileName,
    1i32 as size_t,
    &mut match_0,
    0i32,
  ) != 0
  {
    return 0i32;
  }
  if match_0.rm_so != 0 {
    return 0i32;
  }
  if *fileName.offset(match_0.rm_eo as isize) != 0 {
    return 0i32;
  }
  return 1i32;
}
unsafe extern "C" fn func_type(
  mut _fileName: *const libc::c_char,
  mut statbuf: *const stat,
  mut ap: *mut action_type,
) -> libc::c_int {
  return ((*statbuf).st_mode & 0o170000i32 as libc::c_uint == (*ap).type_mask as libc::c_uint)
    as libc::c_int;
}
unsafe extern "C" fn func_executable(
  mut fileName: *const libc::c_char,
  mut _statbuf: *const stat,
  mut _ap: *mut action_executable,
) -> libc::c_int {
  return (access(fileName, 1i32) == 0i32) as libc::c_int;
}
unsafe extern "C" fn func_perm(
  mut _fileName: *const libc::c_char,
  mut statbuf: *const stat,
  mut ap: *mut action_perm,
) -> libc::c_int {
  /* -perm [+/]mode: at least one of perm_mask bits are set */
  if (*ap).perm_char as libc::c_int == '+' as i32 || (*ap).perm_char as libc::c_int == '/' as i32 {
    return ((*statbuf).st_mode & (*ap).perm_mask != 0i32 as libc::c_uint) as libc::c_int;
  }
  /* -perm -mode: all of perm_mask are set */
  if (*ap).perm_char as libc::c_int == '-' as i32 {
    return ((*statbuf).st_mode & (*ap).perm_mask == (*ap).perm_mask) as libc::c_int;
  }
  /* -perm mode: file mode must match perm_mask */
  return ((*statbuf).st_mode & 0o7777i32 as libc::c_uint == (*ap).perm_mask) as libc::c_int;
}
unsafe extern "C" fn func_mtime(
  mut _fileName: *const libc::c_char,
  mut statbuf: *const stat,
  mut ap: *mut action_mtime,
) -> libc::c_int {
  let mut file_age: time_t = time(0 as *mut time_t) - (*statbuf).st_mtime;
  let mut mtime_secs: time_t = (*ap)
    .mtime_days
    .wrapping_mul(24i32 as libc::c_uint)
    .wrapping_mul(60i32 as libc::c_uint)
    .wrapping_mul(60i32 as libc::c_uint) as time_t;
  if (*ap).mtime_char as libc::c_int == '+' as i32 {
    return (file_age >= mtime_secs + (24i32 * 60i32 * 60i32) as libc::c_long) as libc::c_int;
  }
  if (*ap).mtime_char as libc::c_int == '-' as i32 {
    return (file_age < mtime_secs) as libc::c_int;
  }
  /* just numeric mtime */
  return (file_age >= mtime_secs && file_age < mtime_secs + (24i32 * 60i32 * 60i32) as libc::c_long)
    as libc::c_int;
}
unsafe extern "C" fn func_mmin(
  mut _fileName: *const libc::c_char,
  mut statbuf: *const stat,
  mut ap: *mut action_mmin,
) -> libc::c_int {
  let mut file_age: time_t = time(0 as *mut time_t) - (*statbuf).st_mtime;
  let mut mmin_secs: time_t = (*ap).mmin_mins.wrapping_mul(60i32 as libc::c_uint) as time_t;
  if (*ap).mmin_char as libc::c_int == '+' as i32 {
    return (file_age >= mmin_secs + 60i32 as libc::c_long) as libc::c_int;
  }
  if (*ap).mmin_char as libc::c_int == '-' as i32 {
    return (file_age < mmin_secs) as libc::c_int;
  }
  /* just numeric mmin */
  return (file_age >= mmin_secs && file_age < mmin_secs + 60i32 as libc::c_long) as libc::c_int;
}
unsafe extern "C" fn func_newer(
  mut _fileName: *const libc::c_char,
  mut statbuf: *const stat,
  mut ap: *mut action_newer,
) -> libc::c_int {
  return ((*ap).newer_mtime < (*statbuf).st_mtime) as libc::c_int;
}
unsafe extern "C" fn func_inum(
  mut _fileName: *const libc::c_char,
  mut statbuf: *const stat,
  mut ap: *mut action_inum,
) -> libc::c_int {
  return ((*statbuf).st_ino == (*ap).inode_num) as libc::c_int;
}
unsafe extern "C" fn do_exec(
  mut ap: *mut action_exec,
  mut fileName: *const libc::c_char,
) -> libc::c_int {
  let mut i: libc::c_int = 0;
  let mut rc: libc::c_int = 0;
  let mut size: libc::c_int = (*ap).exec_argc + (*ap).filelist_idx + 1i32;
  /* gcc 4.3.1 generates smaller code: */
  let vla = size as usize;
  let mut argv: Vec<*mut libc::c_char> = ::std::vec::from_elem(0 as *mut libc::c_char, vla);
  let mut pp: *mut *mut libc::c_char = argv.as_mut_ptr();
  i = 0i32;
  while i < (*ap).exec_argc {
    let mut arg: *const libc::c_char = *(*ap).exec_argv.offset(i as isize);
    if !(*ap).filelist.is_null() {
      /* Handling "-exec +"
       * Only one exec_argv[i] has substitution in it.
       * Expand that one exec_argv[i] into file list.
       */
      if *(*ap).subst_count.offset(i as isize) == 0i32 as libc::c_uint {
        let fresh0 = pp;
        pp = pp.offset(1);
        *fresh0 = xstrdup(arg)
      } else {
        let mut j: libc::c_int = 0i32;
        while !(*(*ap).filelist.offset(j as isize)).is_null() {
          /* 2nd arg here should be ap->subst_count[i], but it is always 1: */
          let fresh1 = pp;
          pp = pp.offset(1);
          *fresh1 = xmalloc_substitute_string(
            arg,
            1i32,
            b"{}\x00" as *const u8 as *const libc::c_char,
            *(*ap).filelist.offset(j as isize),
          );
          free(*(*ap).filelist.offset(j as isize) as *mut libc::c_void);
          j += 1
        }
      }
    } else {
      /* Handling "-exec ;" */
      let fresh2 = pp; /* terminate the list */
      pp = pp.offset(1);
      *fresh2 = xmalloc_substitute_string(
        arg,
        *(*ap).subst_count.offset(i as isize) as libc::c_int,
        b"{}\x00" as *const u8 as *const libc::c_char,
        fileName,
      )
    }
    i += 1
  }
  *pp = 0 as *mut libc::c_char;
  if !(*ap).filelist.is_null() {
    let ref mut fresh3 = *(*ap).filelist.offset(0);
    *fresh3 = 0 as *mut libc::c_char;
    (*ap).filelist_idx = 0i32;
    (*ap).file_len = 0i32
  }
  rc = spawn_and_wait(argv.as_mut_ptr());
  if rc < 0i32 {
    bb_simple_perror_msg(*argv.as_mut_ptr().offset(0));
  }
  i = 0i32;
  while !(*argv.as_mut_ptr().offset(i as isize)).is_null() {
    let fresh4 = i;
    i = i + 1;
    free(*argv.as_mut_ptr().offset(fresh4 as isize) as *mut libc::c_void);
  }
  return (rc == 0i32) as libc::c_int;
  /* return 1 if exitcode 0 */
}
unsafe extern "C" fn func_exec(
  mut fileName: *const libc::c_char,
  mut _statbuf: *const stat,
  mut ap: *mut action_exec,
) -> libc::c_int {
  if !(*ap).filelist.is_null() {
    let mut rc: libc::c_int = 0;
    (*ap).filelist = xrealloc_vector_helper(
      (*ap).filelist as *mut libc::c_void,
      ((::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong) << 8i32)
        .wrapping_add(8i32 as libc::c_ulong) as libc::c_uint,
      (*ap).filelist_idx,
    ) as *mut *mut libc::c_char;
    let fresh5 = (*ap).filelist_idx;
    (*ap).filelist_idx = (*ap).filelist_idx + 1;
    let ref mut fresh6 = *(*ap).filelist.offset(fresh5 as isize);
    *fresh6 = xstrdup(fileName);
    (*ap).file_len = ((*ap).file_len as libc::c_ulong).wrapping_add(
      strlen(fileName)
        .wrapping_add(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
        .wrapping_add(1i32 as libc::c_ulong),
    ) as libc::c_int as libc::c_int;
    /* If we have lots of files already, exec the command */
    rc = 1i32;
    if (*ap).file_len as libc::c_uint
      >= (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).max_argv_len
    {
      rc = do_exec(ap, 0 as *const libc::c_char)
    }
    return rc;
  }
  return do_exec(ap, fileName);
}
unsafe extern "C" fn flush_exec_plus() -> libc::c_int {
  let mut ap: *mut action = 0 as *mut action;
  let mut app: *mut *mut action = 0 as *mut *mut action;
  let mut appp: *mut *mut *mut action = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).actions;
  loop {
    let fresh7 = appp;
    appp = appp.offset(1);
    app = *fresh7;
    if app.is_null() {
      break;
    }
    loop {
      let fresh8 = app;
      app = app.offset(1);
      ap = *fresh8;
      if ap.is_null() {
        break;
      }
      if (*ap).f
        == ::std::mem::transmute::<
          Option<
            unsafe extern "C" fn(
              _: *const libc::c_char,
              _: *const stat,
              _: *mut action_exec,
            ) -> libc::c_int,
          >,
          action_fp,
        >(Some(
          func_exec
            as unsafe extern "C" fn(
              _: *const libc::c_char,
              _: *const stat,
              _: *mut action_exec,
            ) -> libc::c_int,
        ))
      {
        let mut ae: *mut action_exec = ap as *mut libc::c_void as *mut action_exec;
        if (*ae).filelist_idx != 0i32 {
          let mut rc: libc::c_int = do_exec(ae, 0 as *const libc::c_char);
          if (*ap).invert {
            rc = (rc == 0) as libc::c_int
          }
          if rc == 0i32 {
            return 1i32;
          }
        }
      }
    }
  }
  return 0i32;
}
unsafe extern "C" fn func_user(
  mut _fileName: *const libc::c_char,
  mut statbuf: *const stat,
  mut ap: *mut action_user,
) -> libc::c_int {
  return ((*statbuf).st_uid == (*ap).uid) as libc::c_int;
}
unsafe extern "C" fn func_group(
  mut _fileName: *const libc::c_char,
  mut statbuf: *const stat,
  mut ap: *mut action_group,
) -> libc::c_int {
  return ((*statbuf).st_gid == (*ap).gid) as libc::c_int;
}
unsafe extern "C" fn func_print0(
  mut fileName: *const libc::c_char,
  mut _statbuf: *const stat,
  mut _ap: *mut action_print0,
) -> libc::c_int {
  printf(
    b"%s%c\x00" as *const u8 as *const libc::c_char,
    fileName,
    '\u{0}' as i32,
  );
  return 1i32;
}
unsafe extern "C" fn func_print(
  mut fileName: *const libc::c_char,
  mut _statbuf: *const stat,
  mut _ap: *mut action_print,
) -> libc::c_int {
  puts(fileName);
  return 1i32;
}
unsafe extern "C" fn func_paren(
  mut fileName: *const libc::c_char,
  mut statbuf: *const stat,
  mut ap: *mut action_paren,
) -> libc::c_int {
  return exec_actions((*ap).subexpr, fileName, statbuf);
}
unsafe extern "C" fn func_size(
  mut _fileName: *const libc::c_char,
  mut statbuf: *const stat,
  mut ap: *mut action_size,
) -> libc::c_int {
  if (*ap).size_char as libc::c_int == '+' as i32 {
    return ((*statbuf).st_size > (*ap).size) as libc::c_int;
  }
  if (*ap).size_char as libc::c_int == '-' as i32 {
    return ((*statbuf).st_size < (*ap).size) as libc::c_int;
  }
  return ((*statbuf).st_size == (*ap).size) as libc::c_int;
}
/*
 * -prune: if -depth is not given, return true and do not descend
 * current dir; if -depth is given, return false with no effect.
 * Example:
 * find dir -name 'asm-*' -prune -o -name '*.[chS]' -print
 */
unsafe extern "C" fn func_prune(
  mut _fileName: *const libc::c_char,
  mut _statbuf: *const stat,
  mut _ap: *mut action_prune,
) -> libc::c_int {
  return 2i32 + 1i32;
}
unsafe extern "C" fn func_quit(
  mut _fileName: *const libc::c_char,
  mut _statbuf: *const stat,
  mut _ap: *mut action_quit,
) -> libc::c_int {
  exit((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).exitstatus as libc::c_int);
}
unsafe extern "C" fn func_delete(
  mut fileName: *const libc::c_char,
  mut statbuf: *const stat,
  mut _ap: *mut action_delete,
) -> libc::c_int {
  let mut rc: libc::c_int = 0;
  if (*statbuf).st_mode & 0o170000i32 as libc::c_uint == 0o40000i32 as libc::c_uint {
    /* "find . -delete" skips rmdir(".") */
    rc = 0i32;
    if *fileName.offset(0) as libc::c_int != '.' as i32 || *fileName.offset(1) as libc::c_int != 0 {
      rc = rmdir(fileName)
    }
  } else {
    rc = unlink(fileName)
  }
  if rc < 0i32 {
    bb_simple_perror_msg(fileName);
  }
  return 1i32;
}
unsafe extern "C" fn func_empty(
  mut fileName: *const libc::c_char,
  mut statbuf: *const stat,
  mut _ap: *mut action_empty,
) -> libc::c_int {
  if (*statbuf).st_mode & 0o170000i32 as libc::c_uint == 0o40000i32 as libc::c_uint {
    let mut dir: *mut DIR = 0 as *mut DIR;
    let mut dent: *mut dirent = 0 as *mut dirent;
    dir = opendir(fileName);
    if dir.is_null() {
      bb_simple_perror_msg(fileName);
      return 0i32;
    }
    loop {
      dent = readdir(dir);
      if !(!dent.is_null()
        && ((*dent).d_name[0] as libc::c_int == '.' as i32
          && ((*dent).d_name[1] == 0
            || (*dent).d_name[1] as libc::c_int == '.' as i32 && (*dent).d_name[2] == 0)))
      {
        break;
      }
    }
    closedir(dir);
    return (dent == 0 as *mut libc::c_void as *mut dirent) as libc::c_int;
  }
  return ((*statbuf).st_mode & 0o170000i32 as libc::c_uint == 0o100000i32 as libc::c_uint
    && (*statbuf).st_size ==0) as libc::c_int;
}
unsafe extern "C" fn func_links(
  mut _fileName: *const libc::c_char,
  mut statbuf: *const stat,
  mut ap: *mut action_links,
) -> libc::c_int {
  match (*ap).links_char as libc::c_int {
    45 => return ((*statbuf).st_nlink < (*ap).links_count as libc::c_ulong) as libc::c_int,
    43 => return ((*statbuf).st_nlink > (*ap).links_count as libc::c_ulong) as libc::c_int,
    _ => return ((*statbuf).st_nlink == (*ap).links_count as libc::c_ulong) as libc::c_int,
  };
}
unsafe extern "C" fn fileAction(
  mut fileName: *const libc::c_char,
  mut statbuf: *mut stat,
  mut _userData: *mut libc::c_void,
  mut depth: libc::c_int,
) -> libc::c_int {
  let mut r: libc::c_int = 0;
  let mut same_fs: libc::c_int = 1i32;
  if (*statbuf).st_mode & 0o170000i32 as libc::c_uint == 0o40000i32 as libc::c_uint
    && (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).xdev_count != 0
  {
    let mut current_block_2: u64;
    let mut i: libc::c_int = 0;
    i = 0i32;
    loop {
      if !(i < (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).xdev_count) {
        current_block_2 = 6483416627284290920;
        break;
      }
      if *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .xdev_dev
        .offset(i as isize)
        == (*statbuf).st_dev
      {
        current_block_2 = 8515828400728868193;
        break;
      }
      i += 1
    }
    match current_block_2 {
      6483416627284290920 => {
        //bb_error_msg("'%s': not same fs", fileName);
        same_fs = 0i32
      }
      _ => {}
    }
  } /* skip this, continue recursing */
  if depth < (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).minmaxdepth[0] {
    if same_fs != 0 {
      return 1i32;
    }
    return 2i32;
    /* stop recursing */
  } /* stop recursing */
  if depth > (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).minmaxdepth[1] {
    return 2i32;
  }
  r = exec_actions(
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).actions,
    fileName,
    statbuf,
  );
  /* Had no explicit -print[0] or -exec? then print */
  if r & 1i32 != 0
    && (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).need_print as libc::c_int != 0
  {
    puts(fileName);
  }
  if (*statbuf).st_mode & 0o170000i32 as libc::c_uint == 0o40000i32 as libc::c_uint {
    if depth == (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).minmaxdepth[1] {
      return 2i32;
    }
  }
  /* -xdev stops on mountpoints, but AFTER mountpoit itself
   * is processed as usual */
  if same_fs == 0 {
    return 2i32;
  }
  /* Cannot return 0: our caller, recursive_action(),
   * will perror() and skip dirs (if called on dir) */
  return if r & 2i32 != 0 { 2i32 } else { 1i32 }; /* appp[0],[1] == NULL */
}
unsafe extern "C" fn find_type(mut type_0: *const libc::c_char) -> libc::c_int {
  let mut mask: libc::c_int = 0i32;
  if *type_0 as libc::c_int == 'b' as i32 {
    mask = 0o60000i32
  } else if *type_0 as libc::c_int == 'c' as i32 {
    mask = 0o20000i32
  } else if *type_0 as libc::c_int == 'd' as i32 {
    mask = 0o40000i32
  } else if *type_0 as libc::c_int == 'p' as i32 {
    mask = 0o10000i32
  } else if *type_0 as libc::c_int == 'f' as i32 {
    mask = 0o100000i32
  } else if *type_0 as libc::c_int == 'l' as i32 {
    mask = 0o120000i32
  } else if *type_0 as libc::c_int == 's' as i32 {
    mask = 0o140000i32
  }
  if mask == 0i32 || *type_0.offset(1) as libc::c_int != '\u{0}' as i32 {
    bb_error_msg_and_die(
      bb_msg_invalid_arg_to.as_ptr(),
      type_0,
      b"-type\x00" as *const u8 as *const libc::c_char,
    );
  }
  return mask;
}
unsafe extern "C" fn plus_minus_num(mut str: *const libc::c_char) -> *const libc::c_char {
  if *str as libc::c_int == '-' as i32 || *str as libc::c_int == '+' as i32 {
    str = str.offset(1)
  }
  return str;
}
unsafe extern "C" fn alloc_action(
  mut ppl: *mut pp_locals,
  mut sizeof_struct: libc::c_int,
  mut f: action_fp,
) -> *mut action {
  let mut ap: *mut action = xzalloc(sizeof_struct as size_t) as *mut action;
  let mut app: *mut *mut action = 0 as *mut *mut action;
  let mut group: *mut *mut *mut action =
    &mut *(*ppl).appp.offset((*ppl).cur_group as isize) as *mut *mut *mut action;
  app = xrealloc(
    *group as *mut libc::c_void,
    ((*ppl).cur_action.wrapping_add(2i32 as libc::c_uint) as libc::c_ulong)
      .wrapping_mul(::std::mem::size_of::<*mut action>() as libc::c_ulong),
  ) as *mut *mut action;
  *group = app;
  let fresh9 = (*ppl).cur_action;
  (*ppl).cur_action = (*ppl).cur_action.wrapping_add(1);
  let ref mut fresh10 = *app.offset(fresh9 as isize);
  *fresh10 = ap;
  let ref mut fresh11 = *app.offset((*ppl).cur_action as isize);
  *fresh11 = 0 as *mut action;
  (*ap).f = f;
  (*ap).invert = (*ppl).invert_flag;
  (*ppl).invert_flag = 0i32 != 0;
  return ap;
}
unsafe extern "C" fn parse_params(mut argv: *mut *mut libc::c_char) -> *mut *mut *mut action {
  static mut params: [libc::c_char; 235] = [
    45, 102, 111, 108, 108, 111, 119, 0, 45, 120, 100, 101, 118, 0, 45, 100, 101, 112, 116, 104, 0,
    45, 97, 0, 45, 111, 0, 33, 0, 45, 97, 110, 100, 0, 45, 111, 114, 0, 45, 110, 111, 116, 0, 45,
    112, 114, 105, 110, 116, 0, 45, 112, 114, 105, 110, 116, 48, 0, 45, 112, 114, 117, 110, 101, 0,
    45, 113, 117, 105, 116, 0, 45, 100, 101, 108, 101, 116, 101, 0, 45, 101, 109, 112, 116, 121, 0,
    45, 101, 120, 101, 99, 0, 45, 101, 120, 101, 99, 117, 116, 97, 98, 108, 101, 0, 40, 0, 45, 110,
    97, 109, 101, 0, 45, 105, 110, 97, 109, 101, 0, 45, 112, 97, 116, 104, 0, 45, 119, 104, 111,
    108, 101, 110, 97, 109, 101, 0, 45, 105, 112, 97, 116, 104, 0, 45, 114, 101, 103, 101, 120, 0,
    45, 116, 121, 112, 101, 0, 45, 112, 101, 114, 109, 0, 45, 109, 116, 105, 109, 101, 0, 45, 109,
    109, 105, 110, 0, 45, 110, 101, 119, 101, 114, 0, 45, 105, 110, 117, 109, 0, 45, 117, 115, 101,
    114, 0, 45, 103, 114, 111, 117, 112, 0, 45, 115, 105, 122, 101, 0, 45, 108, 105, 110, 107, 115,
    0, 45, 109, 105, 110, 100, 101, 112, 116, 104, 0, 45, 109, 97, 120, 100, 101, 112, 116, 104, 0,
    0,
  ];
  let mut ppl: pp_locals = pp_locals {
    appp: 0 as *mut *mut *mut action,
    cur_group: 0,
    cur_action: 0,
    invert_flag: false,
  };
  ppl.cur_group = 0i32 as libc::c_uint;
  ppl.cur_action = 0i32 as libc::c_uint;
  ppl.invert_flag = 0i32 != 0;
  ppl.appp = xzalloc(
    (2i32 as libc::c_ulong)
      .wrapping_mul(::std::mem::size_of::<*mut *mut action>() as libc::c_ulong),
  ) as *mut *mut *mut action;
  while !(*argv).is_null() {
    let mut arg: *const libc::c_char = *argv.offset(0);
    let mut parm: libc::c_int = index_in_strings(params.as_ptr(), arg);
    let mut arg1: *const libc::c_char = *argv.offset(1);
    if parm >= PARM_name as libc::c_int {
      /* All options/actions starting from -name require argument */
      if arg1.is_null() {
        bb_error_msg_and_die(bb_msg_requires_arg.as_ptr(), arg);
      }
      argv = argv.offset(1)
    }
    /* We can use big switch() here, but on i386
     * it doesn't give smaller code. Other arches? */
    /* Options always return true. They always take effect
     * rather than being processed only when their place in the
     * expression is reached.
     */
    /* Options */
    if parm == OPT_FOLLOW as libc::c_int {
      let ref mut fresh12 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).recurse_flags;
      *fresh12 = (*fresh12 as libc::c_int
        | (ACTION_FOLLOWLINKS as libc::c_int | ACTION_DANGLING_OK as libc::c_int))
        as recurse_flags_t
    } else if parm == OPT_XDEV as libc::c_int {
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).xdev_on = 1i32 as smallint
    } else if parm == OPT_MINDEPTH as libc::c_int || parm == OPT_MINDEPTH as libc::c_int + 1i32 {
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).minmaxdepth
        [(parm - OPT_MINDEPTH as libc::c_int) as usize] = xatoi_positive(arg1)
    } else if parm == OPT_DEPTH as libc::c_int {
      let ref mut fresh13 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).recurse_flags;
      *fresh13 = (*fresh13 as libc::c_int | ACTION_DEPTHFIRST as libc::c_int) as recurse_flags_t
    } else if !(parm == PARM_a as libc::c_int || parm == PARM_and as libc::c_int) {
      if parm == PARM_o as libc::c_int || parm == PARM_or as libc::c_int {
        /* Actions are grouped by operators
         * ( expr )              Force precedence
         * ! expr                True if expr is false
         * -not expr             Same as ! expr
         * expr1 [-a[nd]] expr2  And; expr2 is not evaluated if expr1 is false
         * expr1 -o[r] expr2     Or; expr2 is not evaluated if expr1 is true
         * expr1 , expr2         List; both expr1 and expr2 are always evaluated
         * We implement: (), -a, -o
         */
        /* Operators */
        /* start new OR group */
        ppl.cur_group = ppl.cur_group.wrapping_add(1);
        ppl.appp = xrealloc(
          ppl.appp as *mut libc::c_void,
          (ppl.cur_group.wrapping_add(2i32 as libc::c_uint) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut *mut action>() as libc::c_ulong),
        ) as *mut *mut *mut action;
        /*appp[cur_group] = NULL; - already NULL */
        let ref mut fresh14 = *ppl
          .appp
          .offset(ppl.cur_group.wrapping_add(1i32 as libc::c_uint) as isize);
        *fresh14 = 0 as *mut *mut action;
        ppl.cur_action = 0i32 as libc::c_uint
      } else if parm == PARM_char_not as libc::c_int || parm == PARM_not as libc::c_int {
        /* also handles "find ! ! -name 'foo*'" */
        ppl.invert_flag = (ppl.invert_flag as libc::c_int ^ 1i32) != 0
      } else if parm == PARM_print as libc::c_int {
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).need_print = 0i32 as smallint;
        alloc_action(
          &mut ppl,
          ::std::mem::size_of::<action_print>() as libc::c_ulong as libc::c_int,
          ::std::mem::transmute::<
            Option<
              unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *const stat,
                _: *mut action_print,
              ) -> libc::c_int,
            >,
            action_fp,
          >(Some(
            func_print
              as unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *const stat,
                _: *mut action_print,
              ) -> libc::c_int,
          )),
        );
      } else if parm == PARM_print0 as libc::c_int {
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).need_print = 0i32 as smallint;
        alloc_action(
          &mut ppl,
          ::std::mem::size_of::<action_print0>() as libc::c_ulong as libc::c_int,
          ::std::mem::transmute::<
            Option<
              unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *const stat,
                _: *mut action_print0,
              ) -> libc::c_int,
            >,
            action_fp,
          >(Some(
            func_print0
              as unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *const stat,
                _: *mut action_print0,
              ) -> libc::c_int,
          )),
        );
      } else if parm == PARM_prune as libc::c_int {
        alloc_action(
          &mut ppl,
          ::std::mem::size_of::<action_prune>() as libc::c_ulong as libc::c_int,
          ::std::mem::transmute::<
            Option<
              unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *const stat,
                _: *mut action_prune,
              ) -> libc::c_int,
            >,
            action_fp,
          >(Some(
            func_prune
              as unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *const stat,
                _: *mut action_prune,
              ) -> libc::c_int,
          )),
        );
      } else if parm == PARM_quit as libc::c_int {
        alloc_action(
          &mut ppl,
          ::std::mem::size_of::<action_quit>() as libc::c_ulong as libc::c_int,
          ::std::mem::transmute::<
            Option<
              unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *const stat,
                _: *mut action_quit,
              ) -> libc::c_int,
            >,
            action_fp,
          >(Some(
            func_quit
              as unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *const stat,
                _: *mut action_quit,
              ) -> libc::c_int,
          )),
        );
      } else if parm == PARM_delete as libc::c_int {
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).need_print = 0i32 as smallint;
        let ref mut fresh15 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).recurse_flags;
        *fresh15 = (*fresh15 as libc::c_int | ACTION_DEPTHFIRST as libc::c_int) as recurse_flags_t;
        alloc_action(
          &mut ppl,
          ::std::mem::size_of::<action_delete>() as libc::c_ulong as libc::c_int,
          ::std::mem::transmute::<
            Option<
              unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *const stat,
                _: *mut action_delete,
              ) -> libc::c_int,
            >,
            action_fp,
          >(Some(
            func_delete
              as unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *const stat,
                _: *mut action_delete,
              ) -> libc::c_int,
          )),
        );
      } else if parm == PARM_empty as libc::c_int {
        alloc_action(
          &mut ppl,
          ::std::mem::size_of::<action_empty>() as libc::c_ulong as libc::c_int,
          ::std::mem::transmute::<
            Option<
              unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *const stat,
                _: *mut action_empty,
              ) -> libc::c_int,
            >,
            action_fp,
          >(Some(
            func_empty
              as unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *const stat,
                _: *mut action_empty,
              ) -> libc::c_int,
          )),
        );
      } else if parm == PARM_exec as libc::c_int {
        let mut i: libc::c_int = 0;
        let mut ap: *mut action_exec = 0 as *mut action_exec;
        let mut all_subst: libc::c_int = 0i32;
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).need_print = 0i32 as smallint;
        ap = alloc_action(
          &mut ppl,
          ::std::mem::size_of::<action_exec>() as libc::c_ulong as libc::c_int,
          ::std::mem::transmute::<
            Option<
              unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *const stat,
                _: *mut action_exec,
              ) -> libc::c_int,
            >,
            action_fp,
          >(Some(
            func_exec
              as unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *const stat,
                _: *mut action_exec,
              ) -> libc::c_int,
          )),
        ) as *mut action_exec;
        /* Actions */
        argv = argv.offset(1); /* first arg after -exec */
        (*ap).exec_argv = argv;
        loop
        /*ap->exec_argc = 0; - ALLOC_ACTION did it */
        {
          if (*argv).is_null() {
            /* did not see ';' or '+' until end */
            bb_error_msg_and_die(
              bb_msg_requires_arg.as_ptr(),
              b"-exec\x00" as *const u8 as *const libc::c_char,
            );
          }
          // find -exec echo Foo ">{}<" ";"
          // executes "echo Foo >FILENAME<",
          // find -exec echo Foo ">{}<" "+"
          // executes "echo Foo FILENAME1 FILENAME2 FILENAME3...".
          if (*(*argv.offset(0)).offset(0) as libc::c_int == ';' as i32
            || *(*argv.offset(0)).offset(0) as libc::c_int == '+' as i32)
            && *(*argv.offset(0)).offset(1) as libc::c_int == '\u{0}' as i32
          {
            if *(*argv.offset(0)).offset(0) as libc::c_int == '+' as i32 {
              (*ap).filelist = xzalloc(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                as *mut *mut libc::c_char
            }
            break;
          } else {
            argv = argv.offset(1);
            (*ap).exec_argc += 1
          }
        }
        if (*ap).exec_argc == 0i32 {
          bb_error_msg_and_die(bb_msg_requires_arg.as_ptr(), arg);
        }
        (*ap).subst_count = xmalloc(
          ((*ap).exec_argc as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_uint;
        i = (*ap).exec_argc;
        loop {
          let fresh16 = i;
          i = i - 1;
          if !(fresh16 != 0) {
            break;
          }
          *(*ap).subst_count.offset(i as isize) = count_strstr(
            *(*ap).exec_argv.offset(i as isize),
            b"{}\x00" as *const u8 as *const libc::c_char,
          );
          all_subst = (all_subst as libc::c_uint)
            .wrapping_add(*(*ap).subst_count.offset(i as isize))
            as libc::c_int as libc::c_int
        }
        /*
         * coreutils expects {} to appear only once in "-exec +"
         */
        if all_subst != 1i32 && !(*ap).filelist.is_null() {
          bb_simple_error_msg_and_die(
            b"only one \'{}\' allowed for -exec +\x00" as *const u8 as *const libc::c_char,
          ); /* restore NULLed parameter */
        }
      } else if parm == PARM_char_brace as libc::c_int {
        let mut ap_0: *mut action_paren = 0 as *mut action_paren;
        let mut endarg: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        let mut nested: libc::c_uint = 1i32 as libc::c_uint;
        endarg = argv;
        loop {
          endarg = endarg.offset(1);
          if (*endarg).is_null() {
            bb_simple_error_msg_and_die(b"unpaired \'(\'\x00" as *const u8 as *const libc::c_char);
          }
          if *(*endarg).offset(0) as libc::c_int == '(' as i32 && *(*endarg).offset(1) == 0 {
            nested = nested.wrapping_add(1)
          } else {
            if !(*(*endarg).offset(0) as libc::c_int == ')' as i32
              && *(*endarg).offset(1) == 0
              && {
                nested = nested.wrapping_sub(1);
                (nested) == 0
              })
            {
              continue;
            }
            *endarg = 0 as *mut libc::c_char;
            break;
          }
        }
        ap_0 = alloc_action(
          &mut ppl,
          ::std::mem::size_of::<action_paren>() as libc::c_ulong as libc::c_int,
          ::std::mem::transmute::<
            Option<
              unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *const stat,
                _: *mut action_paren,
              ) -> libc::c_int,
            >,
            action_fp,
          >(Some(
            func_paren
              as unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *const stat,
                _: *mut action_paren,
              ) -> libc::c_int,
          )),
        ) as *mut action_paren;
        (*ap_0).subexpr = parse_params(argv.offset(1));
        *endarg = b")\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
        argv = endarg
      } else if parm == PARM_name as libc::c_int || parm == PARM_iname as libc::c_int {
        let mut ap_1: *mut action_name = 0 as *mut action_name;
        ap_1 = alloc_action(
          &mut ppl,
          ::std::mem::size_of::<action_name>() as libc::c_ulong as libc::c_int,
          ::std::mem::transmute::<
            Option<
              unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *const stat,
                _: *mut action_name,
              ) -> libc::c_int,
            >,
            action_fp,
          >(Some(
            func_name
              as unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *const stat,
                _: *mut action_name,
              ) -> libc::c_int,
          )),
        ) as *mut action_name;
        (*ap_1).pattern = arg1;
        (*ap_1).iname = parm == PARM_iname as libc::c_int
      } else if parm == PARM_path as libc::c_int
        || parm == PARM_wholename as libc::c_int
        || parm == PARM_ipath as libc::c_int
      {
        let mut ap_2: *mut action_path = 0 as *mut action_path;
        ap_2 = alloc_action(
          &mut ppl,
          ::std::mem::size_of::<action_path>() as libc::c_ulong as libc::c_int,
          ::std::mem::transmute::<
            Option<
              unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *const stat,
                _: *mut action_path,
              ) -> libc::c_int,
            >,
            action_fp,
          >(Some(
            func_path
              as unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *const stat,
                _: *mut action_path,
              ) -> libc::c_int,
          )),
        ) as *mut action_path;
        (*ap_2).pattern = arg1;
        (*ap_2).ipath = parm == PARM_ipath as libc::c_int
      } else if parm == PARM_regex as libc::c_int {
        let mut ap_3: *mut action_regex = 0 as *mut action_regex;
        ap_3 = alloc_action(
          &mut ppl,
          ::std::mem::size_of::<action_regex>() as libc::c_ulong as libc::c_int,
          ::std::mem::transmute::<
            Option<
              unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *const stat,
                _: *mut action_regex,
              ) -> libc::c_int,
            >,
            action_fp,
          >(Some(
            func_regex
              as unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *const stat,
                _: *mut action_regex,
              ) -> libc::c_int,
          )),
        ) as *mut action_regex;
        xregcomp(&mut (*ap_3).compiled_pattern, arg1, 0i32);
      } else if parm == PARM_type as libc::c_int {
        let mut ap_4: *mut action_type = 0 as *mut action_type;
        ap_4 = alloc_action(
          &mut ppl,
          ::std::mem::size_of::<action_type>() as libc::c_ulong as libc::c_int,
          ::std::mem::transmute::<
            Option<
              unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *const stat,
                _: *mut action_type,
              ) -> libc::c_int,
            >,
            action_fp,
          >(Some(
            func_type
              as unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *const stat,
                _: *mut action_type,
              ) -> libc::c_int,
          )),
        ) as *mut action_type;
        (*ap_4).type_mask = find_type(arg1)
      } else if parm == PARM_executable as libc::c_int {
        alloc_action(
          &mut ppl,
          ::std::mem::size_of::<action_executable>() as libc::c_ulong as libc::c_int,
          ::std::mem::transmute::<
            Option<
              unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *const stat,
                _: *mut action_executable,
              ) -> libc::c_int,
            >,
            action_fp,
          >(Some(
            func_executable
              as unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *const stat,
                _: *mut action_executable,
              ) -> libc::c_int,
          )),
        );
      } else if parm == PARM_perm as libc::c_int {
        let mut ap_5: *mut action_perm = 0 as *mut action_perm;
        ap_5 = alloc_action(
          &mut ppl,
          ::std::mem::size_of::<action_perm>() as libc::c_ulong as libc::c_int,
          ::std::mem::transmute::<
            Option<
              unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *const stat,
                _: *mut action_perm,
              ) -> libc::c_int,
            >,
            action_fp,
          >(Some(
            func_perm
              as unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *const stat,
                _: *mut action_perm,
              ) -> libc::c_int,
          )),
        ) as *mut action_perm;
        (*ap_5).perm_char = *arg1.offset(0);
        arg1 = if *arg1.offset(0) as libc::c_int == '/' as i32 {
          arg1.offset(1)
        } else {
          plus_minus_num(arg1)
        };
        /* -perm BITS   File's mode bits are exactly BITS (octal or symbolic).
         *              Symbolic modes use mode 0 as a point of departure.
         * -perm -BITS  All of the BITS are set in file's mode.
         * -perm [+/]BITS  At least one of the BITS is set in file's mode.
         */
        /*ap->perm_mask = 0; - ALLOC_ACTION did it */
        (*ap_5).perm_mask = bb_parse_mode(arg1, (*ap_5).perm_mask) as mode_t;
        if (*ap_5).perm_mask == -1i32 as mode_t {
          bb_error_msg_and_die(
            b"invalid mode \'%s\'\x00" as *const u8 as *const libc::c_char,
            arg1,
          );
        }
      } else if parm == PARM_mtime as libc::c_int {
        let mut ap_6: *mut action_mtime = 0 as *mut action_mtime;
        ap_6 = alloc_action(
          &mut ppl,
          ::std::mem::size_of::<action_mtime>() as libc::c_ulong as libc::c_int,
          ::std::mem::transmute::<
            Option<
              unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *const stat,
                _: *mut action_mtime,
              ) -> libc::c_int,
            >,
            action_fp,
          >(Some(
            func_mtime
              as unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *const stat,
                _: *mut action_mtime,
              ) -> libc::c_int,
          )),
        ) as *mut action_mtime;
        (*ap_6).mtime_char = *arg1.offset(0);
        (*ap_6).mtime_days = xatoul(plus_minus_num(arg1)) as libc::c_uint
      } else if parm == PARM_mmin as libc::c_int {
        let mut ap_7: *mut action_mmin = 0 as *mut action_mmin;
        ap_7 = alloc_action(
          &mut ppl,
          ::std::mem::size_of::<action_mmin>() as libc::c_ulong as libc::c_int,
          ::std::mem::transmute::<
            Option<
              unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *const stat,
                _: *mut action_mmin,
              ) -> libc::c_int,
            >,
            action_fp,
          >(Some(
            func_mmin
              as unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *const stat,
                _: *mut action_mmin,
              ) -> libc::c_int,
          )),
        ) as *mut action_mmin;
        (*ap_7).mmin_char = *arg1.offset(0);
        (*ap_7).mmin_mins = xatoul(plus_minus_num(arg1)) as libc::c_uint
      } else if parm == PARM_newer as libc::c_int {
        let mut stat_newer: stat = std::mem::zeroed();
        let mut ap_8: *mut action_newer = 0 as *mut action_newer;
        ap_8 = alloc_action(
          &mut ppl,
          ::std::mem::size_of::<action_newer>() as libc::c_ulong as libc::c_int,
          ::std::mem::transmute::<
            Option<
              unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *const stat,
                _: *mut action_newer,
              ) -> libc::c_int,
            >,
            action_fp,
          >(Some(
            func_newer
              as unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *const stat,
                _: *mut action_newer,
              ) -> libc::c_int,
          )),
        ) as *mut action_newer;
        xstat(arg1, &mut stat_newer);
        (*ap_8).newer_mtime = stat_newer.st_mtime
      } else if parm == PARM_inum as libc::c_int {
        let mut ap_9: *mut action_inum = 0 as *mut action_inum;
        ap_9 = alloc_action(
          &mut ppl,
          ::std::mem::size_of::<action_inum>() as libc::c_ulong as libc::c_int,
          ::std::mem::transmute::<
            Option<
              unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *const stat,
                _: *mut action_inum,
              ) -> libc::c_int,
            >,
            action_fp,
          >(Some(
            func_inum
              as unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *const stat,
                _: *mut action_inum,
              ) -> libc::c_int,
          )),
        ) as *mut action_inum;
        (*ap_9).inode_num = xatoul(arg1)
      } else if parm == PARM_user as libc::c_int {
        let mut ap_10: *mut action_user = 0 as *mut action_user;
        ap_10 = alloc_action(
          &mut ppl,
          ::std::mem::size_of::<action_user>() as libc::c_ulong as libc::c_int,
          ::std::mem::transmute::<
            Option<
              unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *const stat,
                _: *mut action_user,
              ) -> libc::c_int,
            >,
            action_fp,
          >(Some(
            func_user
              as unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *const stat,
                _: *mut action_user,
              ) -> libc::c_int,
          )),
        ) as *mut action_user;
        (*ap_10).uid = bb_strtou(arg1, 0 as *mut *mut libc::c_char, 10i32);
        if *bb_errno != 0 {
          (*ap_10).uid = xuname2uid(arg1) as uid_t
        }
      } else if parm == PARM_group as libc::c_int {
        let mut ap_11: *mut action_group = 0 as *mut action_group;
        ap_11 = alloc_action(
          &mut ppl,
          ::std::mem::size_of::<action_group>() as libc::c_ulong as libc::c_int,
          ::std::mem::transmute::<
            Option<
              unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *const stat,
                _: *mut action_group,
              ) -> libc::c_int,
            >,
            action_fp,
          >(Some(
            func_group
              as unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *const stat,
                _: *mut action_group,
              ) -> libc::c_int,
          )),
        ) as *mut action_group;
        (*ap_11).gid = bb_strtou(arg1, 0 as *mut *mut libc::c_char, 10i32);
        if *bb_errno != 0 {
          (*ap_11).gid = xgroup2gid(arg1) as gid_t
        }
      } else if parm == PARM_size as libc::c_int {
        /* -size n[bckw]: file uses n units of space
         * b (default): units are 512-byte blocks
         * c: 1 byte
         * k: kilobytes
         * w: 2-byte words
         */
        static mut find_suffixes: [suffix_mult; 6] = [
          {
            let mut init = suffix_mult {
              suffix: [99, 0, 0, 0],
              mult: 1i32 as libc::c_uint,
            };
            init
          },
          {
            let mut init = suffix_mult {
              suffix: [119, 0, 0, 0],
              mult: 2i32 as libc::c_uint,
            };
            init
          },
          {
            let mut init = suffix_mult {
              suffix: [0, 0, 0, 0],
              mult: 512i32 as libc::c_uint,
            };
            init
          },
          {
            let mut init = suffix_mult {
              suffix: [98, 0, 0, 0],
              mult: 512i32 as libc::c_uint,
            };
            init
          },
          {
            let mut init = suffix_mult {
              suffix: [107, 0, 0, 0],
              mult: 1024i32 as libc::c_uint,
            };
            init
          },
          {
            let mut init = suffix_mult {
              suffix: [0, 0, 0, 0],
              mult: 0i32 as libc::c_uint,
            };
            init
          },
        ];
        let mut ap_12: *mut action_size = 0 as *mut action_size;
        ap_12 = alloc_action(
          &mut ppl,
          ::std::mem::size_of::<action_size>() as libc::c_ulong as libc::c_int,
          ::std::mem::transmute::<
            Option<
              unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *const stat,
                _: *mut action_size,
              ) -> libc::c_int,
            >,
            action_fp,
          >(Some(
            func_size
              as unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *const stat,
                _: *mut action_size,
              ) -> libc::c_int,
          )),
        ) as *mut action_size;
        (*ap_12).size_char = *arg1.offset(0);
        (*ap_12).size = xatoull_sfx(plus_minus_num(arg1), find_suffixes.as_ptr()) as off_t
      } else if parm == PARM_links as libc::c_int {
        let mut ap_13: *mut action_links = 0 as *mut action_links;
        ap_13 = alloc_action(
          &mut ppl,
          ::std::mem::size_of::<action_links>() as libc::c_ulong as libc::c_int,
          ::std::mem::transmute::<
            Option<
              unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *const stat,
                _: *mut action_links,
              ) -> libc::c_int,
            >,
            action_fp,
          >(Some(
            func_links
              as unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *const stat,
                _: *mut action_links,
              ) -> libc::c_int,
          )),
        ) as *mut action_links;
        (*ap_13).links_char = *arg1.offset(0);
        (*ap_13).links_count = xatoul(plus_minus_num(arg1)) as libc::c_int
      } else {
        bb_error_msg(
          b"unrecognized: %s\x00" as *const u8 as *const libc::c_char,
          arg,
        );
        bb_show_usage();
      }
    }
    argv = argv.offset(1)
  }
  return ppl.appp;
}
#[no_mangle]
pub unsafe extern "C" fn find_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut i: libc::c_int = 0;
  let mut firstopt: libc::c_int = 0;
  let mut past_HLP: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  let mut saved: *mut libc::c_char = 0 as *mut libc::c_char;
  memset(
    bb_common_bufsiz1.as_mut_ptr() as *mut globals as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<globals>() as libc::c_ulong,
  );
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).minmaxdepth[1] = 2147483647i32;
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).max_argv_len =
    bb_arg_max().wrapping_sub(2048i32 as libc::c_uint);
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).need_print = 1i32 as smallint;
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).recurse_flags =
    ACTION_RECURSE as libc::c_int as recurse_flags_t;
  /* "find -type f" + getopt("+HLP") => disaster.
   * Need to avoid getopt running into a non-HLP option.
   * Do this by temporarily storing NULL there:
   */
  past_HLP = argv; /* it is "-" */
  loop {
    past_HLP = past_HLP.offset(1);
    saved = *past_HLP;
    if saved.is_null() {
      break;
    }
    if *saved.offset(0) as libc::c_int != '-' as i32 {
      break;
    }
    if *saved.offset(1) == 0 {
      break;
    }
    if *saved.offset(1) as libc::c_int == '-' as i32 && *saved.offset(2) == 0 {
      /* it is "--" */
      /* Try: find -- /dev/null */
      past_HLP = past_HLP.offset(1);
      saved = *past_HLP;
      break;
    } else if *saved.offset(1).offset(strspn(
      saved.offset(1),
      b"HLP\x00" as *const u8 as *const libc::c_char,
    ) as isize) as libc::c_int
      != '\u{0}' as i32
    {
      break;
    }
  }
  *past_HLP = 0 as *mut libc::c_char;
  /* "+": stop on first non-option */
  i = getopt32(argv, b"+HLP\x00" as *const u8 as *const libc::c_char) as libc::c_int;
  if i & 1i32 << 0i32 != 0 {
    let ref mut fresh17 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).recurse_flags;
    *fresh17 = (*fresh17 as libc::c_int
      | (ACTION_FOLLOWLINKS_L0 as libc::c_int | ACTION_DANGLING_OK as libc::c_int))
      as recurse_flags_t
  }
  if i & 1i32 << 1i32 != 0 {
    let ref mut fresh18 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).recurse_flags;
    *fresh18 = (*fresh18 as libc::c_int
      | (ACTION_FOLLOWLINKS as libc::c_int | ACTION_DANGLING_OK as libc::c_int))
      as recurse_flags_t
  }
  /* -P is default and is ignored */
  argv = past_HLP; /* same result as "argv += optind;" */
  *past_HLP = saved;
  firstopt = 0i32;
  while !(*argv.offset(firstopt as isize)).is_null() {
    if *(*argv.offset(firstopt as isize)).offset(0) as libc::c_int == '-' as i32 {
      break;
    }
    if 1i32 != 0
      && (*(*argv.offset(firstopt as isize)).offset(0) as libc::c_int == '!' as i32
        && *(*argv.offset(firstopt as isize)).offset(1) == 0)
    {
      break;
    }
    if 1i32 != 0
      && (*(*argv.offset(firstopt as isize)).offset(0) as libc::c_int == '(' as i32
        && *(*argv.offset(firstopt as isize)).offset(1) == 0)
    {
      break;
    }
    firstopt += 1
  }
  if firstopt == 0i32 {
    argv = argv.offset(-1);
    *argv = b".\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    firstopt += 1
  }
  let ref mut fresh19 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).actions;
  *fresh19 = parse_params(&mut *argv.offset(firstopt as isize));
  let ref mut fresh20 = *argv.offset(firstopt as isize);
  *fresh20 = 0 as *mut libc::c_char;
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).xdev_on != 0 {
    let mut stbuf: stat = std::mem::zeroed();
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).xdev_count = firstopt;
    let ref mut fresh21 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).xdev_dev;
    *fresh21 = xzalloc(
      ((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).xdev_count as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<libc::dev_t>() as libc::c_ulong),
    ) as *mut libc::dev_t;
    i = 0i32;
    while !(*argv.offset(i as isize)).is_null() {
      /* not xstat(): shouldn't bomb out on
       * "find not_exist exist -xdev" */
      if stat(*argv.offset(i as isize), &mut stbuf) == 0i32 {
        *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
          .xdev_dev
          .offset(i as isize) = stbuf.st_dev
      }
      i += 1
      /* else G.xdev_dev[i] stays 0 and
       * won't match any real device libc::dev_t
       */
    }
  }
  i = 0i32;
  while !(*argv.offset(i as isize)).is_null() {
    if recursive_action(
      *argv.offset(i as isize),
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).recurse_flags as libc::c_uint,
      Some(
        fileAction
          as unsafe extern "C" fn(
            _: *const libc::c_char,
            _: *mut stat,
            _: *mut libc::c_void,
            _: libc::c_int,
          ) -> libc::c_int,
      ),
      Some(
        fileAction
          as unsafe extern "C" fn(
            _: *const libc::c_char,
            _: *mut stat,
            _: *mut libc::c_void,
            _: libc::c_int,
          ) -> libc::c_int,
      ),
      0 as *mut libc::c_void,
      0i32 as libc::c_uint,
    ) == 0
    {
      /* depth */
      let ref mut fresh22 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).exitstatus;
      *fresh22 = (*fresh22 as libc::c_int | 1i32) as smalluint
    }
    i += 1
  }
  let ref mut fresh23 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).exitstatus;
  *fresh23 = (*fresh23 as libc::c_int | flush_exec_plus()) as smalluint;
  return (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).exitstatus as libc::c_int;
}
