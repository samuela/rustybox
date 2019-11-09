use libc::passwd;
use crate::librb::size_t;
use crate::librb::smallint;
use libc;
use libc::close;
use libc::free;
use libc::ino64_t;
use libc::off64_t;
use libc::pid_t;
use libc::stat;
use libc::time_t;
use libc::FILE;

extern "C" {
  pub type __dirstream;
  #[no_mangle]
  fn strtol(
    __nptr: *const libc::c_char,
    __endptr: *mut *mut libc::c_char,
    __base: libc::c_int,
  ) -> libc::c_long;

  #[no_mangle]
  fn putenv(__string: *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn getpid() -> pid_t;
  #[no_mangle]
  fn setpgrp() -> libc::c_int;
  #[no_mangle]
  fn vfork() -> libc::c_int;
  #[no_mangle]
  fn unlink(__name: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn opendir(__name: *const libc::c_char) -> *mut DIR;
  #[no_mangle]
  fn closedir(__dirp: *mut DIR) -> libc::c_int;
  #[no_mangle]
  fn readdir(__dirp: *mut DIR) -> *mut dirent;
  #[no_mangle]
  fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
  #[no_mangle]
  static mut stderr: *mut FILE;
  #[no_mangle]
  fn rename(__old: *const libc::c_char, __new: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn fclose(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn snprintf(
    _: *mut libc::c_char,
    _: libc::c_ulong,
    _: *const libc::c_char,
    _: ...
  ) -> libc::c_int;
  #[no_mangle]
  fn dprintf(__fd: libc::c_int, __fmt: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  static mut applet_name: *const libc::c_char;
  #[no_mangle]
  fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn chdir(__path: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn sleep(__seconds: libc::c_uint) -> libc::c_uint;

  #[no_mangle]
  fn lseek(__fd: libc::c_int, __offset: off64_t, __whence: libc::c_int) -> off64_t;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn execlp(__file: *const libc::c_char, __arg: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn strncasecmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;

  #[no_mangle]
  fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
  #[no_mangle]
  fn waitpid(__pid: pid_t, __stat_loc: *mut libc::c_int, __options: libc::c_int) -> pid_t;
  #[no_mangle]
  fn time(__timer: *mut time_t) -> time_t;
  #[no_mangle]
  fn localtime(__timer: *const time_t) -> *mut tm;
  /* Search for an entry with a matching username.  */
  #[no_mangle]
  fn bb_internal_getpwnam(__name: *const libc::c_char) -> *mut passwd;
  #[no_mangle]
  fn skip_whitespace(_: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn skip_non_whitespace(_: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn is_prefixed_with(string: *const libc::c_char, key: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xmove_fd(_: libc::c_int, _: libc::c_int);
  #[no_mangle]
  fn xchdir(path: *const libc::c_char);
  #[no_mangle]
  fn bb_unsetenv_and_free(key: *mut libc::c_char);
  #[no_mangle]
  fn open_or_warn(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn bb_putchar_stderr(ch: libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  #[no_mangle]
  fn fopen_for_read(path: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn bb_daemonize_or_rexec(flags: libc::c_int);
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  fn write_pidfile_std_path_and_ext(path: *const libc::c_char);
  #[no_mangle]
  static mut logmode: smallint;
  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);
  #[no_mangle]
  fn bb_info_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_vinfo_msg(s: *const libc::c_char, p: ::std::ffi::VaList);
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
  fn change_identity(pw: *const passwd);
  #[no_mangle]
  fn get_shell_name() -> *const libc::c_char;
  #[no_mangle]
  fn fgets_unlocked(
    __s: *mut libc::c_char,
    __n: libc::c_int,
    __stream: *mut FILE,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn fileno_unlocked(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
  #[no_mangle]
  fn openlog(__ident: *const libc::c_char, __option: libc::c_int, __facility: libc::c_int);
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
  pub gp_offset: libc::c_uint,
  pub fp_offset: libc::c_uint,
  pub overflow_arg_area: *mut libc::c_void,
  pub reg_save_area: *mut libc::c_void,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
  pub d_ino: ino64_t,
  pub d_off: off64_t,
  pub d_reclen: libc::c_ushort,
  pub d_type: libc::c_uchar,
  pub d_name: [libc::c_char; 256],
}
pub type DIR = __dirstream;

pub type va_list = __builtin_va_list;
use libc::tm;

pub type C2RustUnnamed = libc::c_uint;
pub const DAEMON_ONLY_SANITIZE: C2RustUnnamed = 8;
pub const DAEMON_CLOSE_EXTRA_FDS: C2RustUnnamed = 4;
pub const DAEMON_DEVNULL_STDIO: C2RustUnnamed = 2;
pub const DAEMON_CHDIR_ROOT: C2RustUnnamed = 1;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const LOGMODE_BOTH: C2RustUnnamed_0 = 3;
pub const LOGMODE_SYSLOG: C2RustUnnamed_0 = 2;
pub const LOGMODE_STDIO: C2RustUnnamed_0 = 1;
pub const LOGMODE_NONE: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const PARSE_NORMAL: C2RustUnnamed_1 = 4653056;
pub const PARSE_WS_COMMENTS: C2RustUnnamed_1 = 16777216;
pub const PARSE_ALT_COMMENTS: C2RustUnnamed_1 = 8388608;
pub const PARSE_EOL_COMMENTS: C2RustUnnamed_1 = 4194304;
pub const PARSE_KEEP_COPY: C2RustUnnamed_1 = 2097152;
pub const PARSE_MIN_DIE: C2RustUnnamed_1 = 1048576;
pub const PARSE_GREEDY: C2RustUnnamed_1 = 262144;
pub const PARSE_TRIM: C2RustUnnamed_1 = 131072;
pub const PARSE_COLLAPSE: C2RustUnnamed_1 = 65536;
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
pub struct globals {
  pub log_level: libc::c_uint,
  pub crontab_dir_mtime: time_t,
  pub log_filename: *const libc::c_char,
  pub crontab_dir_name: *const libc::c_char,
  pub cron_files: *mut CronFile,
  pub default_shell: *mut libc::c_char,
  pub env_var_user: *mut libc::c_char,
  pub env_var_home: *mut libc::c_char,
  pub env_var_shell: *mut libc::c_char,
  pub env_var_logname: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CronFile {
  pub cf_next: *mut CronFile,
  pub cf_lines: *mut CronLine,
  pub cf_username: *mut libc::c_char,
  pub cf_wants_starting: smallint,
  pub cf_has_running: smallint,
  pub cf_deleted: smallint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CronLine {
  pub cl_next: *mut CronLine,
  pub cl_cmd: *mut libc::c_char,
  pub cl_pid: pid_t,
  pub cl_empty_mail_size: libc::c_int,
  pub cl_mailto: *mut libc::c_char,
  pub cl_shell: *mut libc::c_char,
  pub cl_Dow: [libc::c_char; 7],
  pub cl_Mons: [libc::c_char; 12],
  pub cl_Hrs: [libc::c_char; 24],
  pub cl_Days: [libc::c_char; 32],
  pub cl_Mins: [libc::c_char; 60],
}
pub type C2RustUnnamed_2 = libc::c_uint;
pub const OPT_d: C2RustUnnamed_2 = 64;
pub const OPT_c: C2RustUnnamed_2 = 32;
pub const OPT_S: C2RustUnnamed_2 = 16;
pub const OPT_b: C2RustUnnamed_2 = 8;
pub const OPT_f: C2RustUnnamed_2 = 4;
pub const OPT_L: C2RustUnnamed_2 = 2;
pub const OPT_l: C2RustUnnamed_2 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SpecialEntry {
  pub name: *const libc::c_char,
  pub tokens: [libc::c_char; 8],
}
/* Log levels:
 * 0 is the most verbose, default 8.
 * For some reason, in fact only 5, 7 and 8 are used.
 */
unsafe extern "C" fn crondlog(
  mut level: libc::c_uint,
  mut msg: *const libc::c_char,
  mut va: ::std::ffi::VaList,
) {
  if level >= (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).log_level {
    /*
     * We are called only for info meesages.
     * Warnings/errors use plain bb_[p]error_msg's, which
     * need not touch syslog_level
     * (they are ok with LOG_ERR default).
     */
    bb_vinfo_msg(msg, va.as_va_list());
  };
}
unsafe extern "C" fn log5(mut msg: *const libc::c_char, mut args: ...) {
  let mut va: ::std::ffi::VaListImpl;
  va = args.clone();
  crondlog(4i32 as libc::c_uint, msg, va.as_va_list());
}
unsafe extern "C" fn log7(mut msg: *const libc::c_char, mut args: ...) {
  let mut va: ::std::ffi::VaListImpl;
  va = args.clone();
  crondlog(7i32 as libc::c_uint, msg, va.as_va_list());
}
unsafe extern "C" fn log8(mut msg: *const libc::c_char, mut args: ...) {
  let mut va: ::std::ffi::VaListImpl;
  va = args.clone();
  crondlog(8i32 as libc::c_uint, msg, va.as_va_list());
}
static mut DowAry: [libc::c_char; 22] = [
  115, 117, 110, 109, 111, 110, 116, 117, 101, 119, 101, 100, 116, 104, 117, 102, 114, 105, 115,
  97, 116, 0,
];
static mut MonAry: [libc::c_char; 37] = [
  106, 97, 110, 102, 101, 98, 109, 97, 114, 97, 112, 114, 109, 97, 121, 106, 117, 110, 106, 117,
  108, 97, 117, 103, 115, 101, 112, 111, 99, 116, 110, 111, 118, 100, 101, 99, 0,
];
unsafe extern "C" fn ParseField(
  mut user: *mut libc::c_char,
  mut ary: *mut libc::c_char,
  mut modvalue: libc::c_int,
  mut off: libc::c_int,
  mut names: *const libc::c_char,
  mut ptr: *mut libc::c_char,
)
/* 'names' is a pointer to a set of 3-char abbreviations */
{
  let mut current_block: u64;
  let mut base: *mut libc::c_char = ptr;
  let mut n1: libc::c_int = -1i32;
  let mut n2: libc::c_int = -1i32;
  's_14: loop
  // this can't happen due to config_read()
  /*if (base == NULL)
  return;*/
  {
    let mut skip: libc::c_int = 0i32;
    /* Handle numeric digit or symbol or '*' */
    if *ptr as libc::c_int == '*' as i32 {
      n1 = 0i32; /* everything will be filled */
      n2 = modvalue - 1i32; /* gcc likes temp var for &endp */
      skip = 1i32;
      ptr = ptr.offset(1)
    } else if (*ptr as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32 {
      let mut endp: *mut libc::c_char = 0 as *mut libc::c_char;
      if n1 < 0i32 {
        n1 = (strtol(ptr, &mut endp, 10i32) + off as libc::c_long) as libc::c_int
      } else {
        n2 = (strtol(ptr, &mut endp, 10i32) + off as libc::c_long) as libc::c_int
      }
      ptr = endp;
      skip = 1i32
    } else if !names.is_null() {
      let mut i: libc::c_int = 0;
      i = 0i32;
      while *names.offset(i as isize) != 0 {
        /* was using strncmp before... */
        if strncasecmp(ptr, &*names.offset(i as isize), 3i32 as libc::c_ulong) == 0i32 {
          ptr = ptr.offset(3);
          if n1 < 0i32 {
            n1 = i / 3i32
          } else {
            n2 = i / 3i32
          }
          skip = 1i32;
          break;
        } else {
          i += 3i32
        }
      }
    }
    /* handle optional range '-' */
    if skip == 0i32 {
      current_block = 1165395826853565909;
      break;
    }
    if *ptr as libc::c_int == '-' as i32 && n2 < 0i32 {
      ptr = ptr.offset(1)
    } else {
      /*
       * collapse single-value ranges, handle skipmark, and fill
       * in the character array appropriately.
       */
      if n2 < 0i32 {
        n2 = n1
      }
      if *ptr as libc::c_int == '/' as i32 {
        let mut endp_0: *mut libc::c_char = 0 as *mut libc::c_char;
        skip = strtol(ptr.offset(1), &mut endp_0, 10i32) as libc::c_int;
        ptr = endp_0
        /* gcc likes temp var for &endp */
      }
      /*
       * fill array, using a failsafe is the easiest way to prevent
       * an endless loop
       */
      let mut s0: libc::c_int = 1i32;
      let mut failsafe: libc::c_int = 1024i32;
      n1 -= 1;
      loop {
        n1 = (n1 + 1i32) % modvalue;
        s0 -= 1;
        if s0 == 0i32 {
          *ary.offset((n1 % modvalue) as isize) = 1i32 as libc::c_char;
          s0 = skip
        }
        failsafe -= 1;
        if failsafe == 0i32 {
          current_block = 1165395826853565909;
          break 's_14;
        }
        if !(n1 != n2) {
          break;
        }
      }
      if *ptr as libc::c_int != ',' as i32 {
        current_block = 5159818223158340697;
        break;
      }
      ptr = ptr.offset(1);
      n1 = -1i32;
      n2 = -1i32
    }
  }
  match current_block {
    5159818223158340697 => {
      if !(*ptr != 0) {
        /* can't use log5 (it inserts newlines), open-coding it */
        if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).log_level <= 5i32 as libc::c_uint
          && logmode as libc::c_int != LOGMODE_SYSLOG as libc::c_int
        {
          let mut i_0: libc::c_int = 0;
          i_0 = 0i32;
          while i_0 < modvalue {
            fprintf(
              stderr,
              b"%d\x00" as *const u8 as *const libc::c_char,
              *ary.offset(i_0 as isize) as libc::c_uchar as libc::c_int,
            );
            i_0 += 1
          }
          bb_putchar_stderr('\n' as i32 as libc::c_char);
        }
        return;
      }
    }
    _ => {}
  }
  bb_error_msg(
    b"user %s: parse error at %s\x00" as *const u8 as *const libc::c_char,
    user,
    base,
  );
}
unsafe extern "C" fn FixDayDow(mut line: *mut CronLine) {
  let mut i: libc::c_uint = 0;
  let mut weekUsed: libc::c_int = 0i32;
  let mut daysUsed: libc::c_int = 0i32;
  i = 0i32 as libc::c_uint;
  while i
    < (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
      .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong) as libc::c_uint
  {
    if (*line).cl_Dow[i as usize] as libc::c_int == 0i32 {
      weekUsed = 1i32;
      break;
    } else {
      i = i.wrapping_add(1)
    }
  }
  i = 0i32 as libc::c_uint;
  while i
    < (::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
      .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong) as libc::c_uint
  {
    if (*line).cl_Days[i as usize] as libc::c_int == 0i32 {
      daysUsed = 1i32;
      break;
    } else {
      i = i.wrapping_add(1)
    }
  }
  if weekUsed != daysUsed {
    if weekUsed != 0 {
      memset(
        (*line).cl_Days.as_mut_ptr() as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
      );
    } else {
      /* daysUsed */
      memset(
        (*line).cl_Dow.as_mut_ptr() as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong,
      );
    }
  };
}
/*
 * delete_cronfile() - delete user database
 *
 * Note: multiple entries for same user may exist if we were unable to
 * completely delete a database due to running processes.
 */
//FIXME: we will start a new job even if the old job is running
//if crontab was reloaded: crond thinks that "new" job is different from "old"
//even if they are in fact completely the same. Example
//Crontab was:
// 0-59 * * * * job1
// 0-59 * * * * long_running_job2
//User edits crontab to:
// 0-59 * * * * job1_updated
// 0-59 * * * * long_running_job2
//Bug: crond can now start another long_running_job2 even if old one
//is still running.
//OTOH most other versions of cron do not wait for job termination anyway,
//they end up with multiple copies of jobs if they don't terminate soon enough.
unsafe extern "C" fn delete_cronfile(mut userName: *const libc::c_char) {
  let mut pfile: *mut *mut CronFile =
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).cron_files;
  let mut file: *mut CronFile = 0 as *mut CronFile;
  loop {
    file = *pfile;
    if file.is_null() {
      break;
    }
    if strcmp(userName, (*file).cf_username) == 0i32 {
      let mut pline: *mut *mut CronLine = &mut (*file).cf_lines;
      let mut line: *mut CronLine = 0 as *mut CronLine;
      (*file).cf_has_running = 0i32 as smallint;
      (*file).cf_deleted = 1i32 as smallint;
      loop {
        line = *pline;
        if line.is_null() {
          break;
        }
        if (*line).cl_pid > 0i32 {
          (*file).cf_has_running = 1i32 as smallint;
          pline = &mut (*line).cl_next
        } else {
          *pline = (*line).cl_next;
          free((*line).cl_cmd as *mut libc::c_void);
          free(line as *mut libc::c_void);
        }
      }
      if (*file).cf_has_running as libc::c_int == 0i32 {
        *pfile = (*file).cf_next;
        free((*file).cf_username as *mut libc::c_void);
        free(file as *mut libc::c_void);
        continue;
      }
    }
    pfile = &mut (*file).cf_next
  }
}
unsafe extern "C" fn load_crontab(mut fileName: *const libc::c_char) {
  let mut parser: *mut parser_t = 0 as *mut parser_t;
  let mut sbuf: stat = std::mem::zeroed();
  let mut maxLines: libc::c_int = 0;
  let mut tokens: [*mut libc::c_char; 6] = [0 as *mut libc::c_char; 6];
  let mut mailTo: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut shell: *mut libc::c_char = 0 as *mut libc::c_char;
  delete_cronfile(fileName);
  if bb_internal_getpwnam(fileName).is_null() {
    log7(
      b"ignoring file \'%s\' (no such user)\x00" as *const u8 as *const libc::c_char,
      fileName,
    );
    return;
  }
  parser = config_open(fileName);
  if parser.is_null() {
    return;
  }
  maxLines = if strcmp(fileName, b"root\x00" as *const u8 as *const libc::c_char) == 0i32 {
    65535i32
  } else {
    256i32
  };
  if fstat(fileno_unlocked((*parser).fp), &mut sbuf) == 0i32 && sbuf.st_uid == 0i32 as libc::c_uint
  {
    let mut file: *mut CronFile =
      xzalloc(::std::mem::size_of::<CronFile>() as libc::c_ulong) as *mut CronFile;
    let mut pline: *mut *mut CronLine = 0 as *mut *mut CronLine;
    let mut n: libc::c_int = 0;
    (*file).cf_username = xstrdup(fileName);
    pline = &mut (*file).cf_lines;
    's_80: loop {
      let mut line: *mut CronLine = 0 as *mut CronLine;
      maxLines -= 1;
      if maxLines == 0 {
        bb_error_msg(
          b"user %s: too many lines\x00" as *const u8 as *const libc::c_char,
          fileName,
        );
        break;
      } else {
        n = config_read(
          parser,
          tokens.as_mut_ptr(),
          (PARSE_NORMAL as libc::c_int
            | PARSE_KEEP_COPY as libc::c_int
            | (1i32 & 0xffi32) << 8i32
            | 6i32 & 0xffi32) as libc::c_uint,
          b"# \t\x00" as *const u8 as *const libc::c_char,
        );
        if n == 0 {
          break;
        }
        log5(
          b"user:%s entry:%s\x00" as *const u8 as *const libc::c_char,
          fileName,
          (*parser).data,
        );
        //bb_error_msg("M[%s]F[%s][%s][%s][%s][%s][%s]", mailTo, tokens[0], tokens[1], tokens[2], tokens[3], tokens[4], tokens[5]);
        /* check if line is setting MAILTO= */
        if !is_prefixed_with(
          tokens[0],
          b"MAILTO=\x00" as *const u8 as *const libc::c_char,
        )
        .is_null()
        {
          free(mailTo as *mut libc::c_void);
          mailTo = if *tokens[0].offset(7) as libc::c_int != 0 {
            xstrdup(&mut *(*tokens.as_mut_ptr().offset(0)).offset(7))
          } else {
            0 as *mut libc::c_char
          }
        } else if !is_prefixed_with(tokens[0], b"SHELL=\x00" as *const u8 as *const libc::c_char)
          .is_null()
        {
          free(shell as *mut libc::c_void);
          shell = xstrdup(&mut *(*tokens.as_mut_ptr().offset(0)).offset(6))
        } else {
          //TODO: handle HOME= too? "man crontab" says:
          //name = value
          //
          //where the spaces around the equal-sign (=) are optional, and any subsequent
          //non-leading spaces in value will be part of the value assigned to name.
          //The value string may be placed in quotes (single or double, but matching)
          //to preserve leading or trailing blanks.
          //
          //Several environment variables are set up automatically by the cron(8) daemon.
          //SHELL is set to /bin/sh, and LOGNAME and HOME are set from the /etc/passwd
          //line of the crontab's owner. HOME and SHELL may be overridden by settings
          //in the crontab; LOGNAME may not.
          if *tokens[0].offset(0) as libc::c_int == '@' as i32 {
            /*
             * "@daily /a/script/to/run PARAM1 PARAM2..."
             */
            static mut SpecAry: [SpecialEntry; 8] = [
              {
                let mut init = SpecialEntry {
                  name: b"yearly\x00" as *const u8 as *const libc::c_char,
                  tokens: [48, 0, 49, 0, 49, 0, 42, 0],
                };
                init
              },
              {
                let mut init = SpecialEntry {
                  name: b"annually\x00" as *const u8 as *const libc::c_char,
                  tokens: [48, 0, 49, 0, 49, 0, 42, 0],
                };
                init
              },
              {
                let mut init = SpecialEntry {
                  name: b"monthly\x00" as *const u8 as *const libc::c_char,
                  tokens: [48, 0, 49, 0, 42, 0, 42, 0],
                };
                init
              },
              {
                let mut init = SpecialEntry {
                  name: b"weekly\x00" as *const u8 as *const libc::c_char,
                  tokens: [48, 0, 42, 0, 42, 0, 48, 0],
                };
                init
              },
              {
                let mut init = SpecialEntry {
                  name: b"daily\x00" as *const u8 as *const libc::c_char,
                  tokens: [48, 0, 42, 0, 42, 0, 42, 0],
                };
                init
              },
              {
                let mut init = SpecialEntry {
                  name: b"midnight\x00" as *const u8 as *const libc::c_char,
                  tokens: [48, 0, 42, 0, 42, 0, 42, 0],
                };
                init
              },
              {
                let mut init = SpecialEntry {
                  name: b"hourly\x00" as *const u8 as *const libc::c_char,
                  tokens: [42, 0, 42, 0, 42, 0, 42, 0],
                };
                init
              },
              {
                let mut init = SpecialEntry {
                  name: b"reboot\x00" as *const u8 as *const libc::c_char,
                  tokens: [0, 0, 0, 0, 0, 0, 0, 0],
                };
                init
              },
            ];
            let mut e: *const SpecialEntry = SpecAry.as_ptr();
            if n < 2i32 {
              continue;
            }
            loop {
              if strcmp((*e).name, tokens[0].offset(1)) == 0i32 {
                /*
                 * tokens[1] is only the first word of command,
                 * can'r use it.
                 * find the entire command in unmodified string:
                 */
                tokens[5] = skip_whitespace(skip_non_whitespace(skip_whitespace((*parser).data)));
                if (*e).tokens[0] != 0 {
                  let mut et: *mut libc::c_char = (*e).tokens.as_ptr() as *mut libc::c_char;
                  /* minute is "0" for all specials */
                  tokens[0] = b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
                  tokens[1] = et;
                  tokens[2] = et.offset(2);
                  tokens[3] = et.offset(4);
                  tokens[4] = et.offset(6)
                }
                break;
              } else {
                if (*e).tokens[0] == 0 {
                  continue 's_80;
                }
                e = e.offset(1)
              }
            }
          /* bad line (unrecognized '@foo') */
          } else if n < 6i32
          /* check if a minimum of tokens is specified */
          {
            continue;
          }
          line = xzalloc(::std::mem::size_of::<CronLine>() as libc::c_ulong) as *mut CronLine;
          *pline = line;
          if *tokens[0].offset(0) as libc::c_int == '@' as i32 {
            /* "@reboot" line */
            (*file).cf_wants_starting = 1i32 as smallint;
            (*line).cl_pid = -2i32
          /* wants to start */
          /* line->cl_Mins/Hrs/etc stay zero: never match any time */
          } else {
            /* parse date ranges */
            ParseField(
              (*file).cf_username,
              (*line).cl_Mins.as_mut_ptr(),
              60i32,
              0i32,
              0 as *const libc::c_char,
              tokens[0],
            );
            ParseField(
              (*file).cf_username,
              (*line).cl_Hrs.as_mut_ptr(),
              24i32,
              0i32,
              0 as *const libc::c_char,
              tokens[1],
            );
            ParseField(
              (*file).cf_username,
              (*line).cl_Days.as_mut_ptr(),
              32i32,
              0i32,
              0 as *const libc::c_char,
              tokens[2],
            );
            ParseField(
              (*file).cf_username,
              (*line).cl_Mons.as_mut_ptr(),
              12i32,
              -1i32,
              MonAry.as_ptr(),
              tokens[3],
            );
            ParseField(
              (*file).cf_username,
              (*line).cl_Dow.as_mut_ptr(),
              7i32,
              0i32,
              DowAry.as_ptr(),
              tokens[4],
            );
            /*
             * fix days and dow - if one is not "*" and the other
             * is "*", the other is set to 0, and vise-versa
             */
            FixDayDow(line);
          }
          /* copy mailto (can be NULL) */
          (*line).cl_mailto = xstrdup(mailTo);
          (*line).cl_shell = xstrdup(shell);
          /* copy command */
          (*line).cl_cmd = xstrdup(tokens[5]);
          pline = &mut (*line).cl_next
        }
      }
    }
    *pline = 0 as *mut CronLine;
    (*file).cf_next = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).cron_files;
    let ref mut fresh0 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).cron_files;
    *fresh0 = file
  }
  config_close(parser);
  free(mailTo as *mut libc::c_void);
  free(shell as *mut libc::c_void);
}
unsafe extern "C" fn process_cron_update_file() {
  let mut fi: *mut FILE = 0 as *mut FILE;
  let mut buf: [libc::c_char; 256] = [0; 256];
  fi = fopen_for_read(b"cron.update\x00" as *const u8 as *const libc::c_char);
  if !fi.is_null() {
    unlink(b"cron.update\x00" as *const u8 as *const libc::c_char);
    while !fgets_unlocked(
      buf.as_mut_ptr(),
      ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
      fi,
    )
    .is_null()
    {
      /* use first word only */
      *skip_non_whitespace(buf.as_mut_ptr()).offset(0) = '\u{0}' as i32 as libc::c_char;
      load_crontab(buf.as_mut_ptr());
    }
    fclose(fi);
  };
}
unsafe extern "C" fn rescan_crontab_dir() {
  let mut file: *mut CronFile = 0 as *mut CronFile;
  's_5: loop
  /* Delete all files until we only have ones with running jobs (or none) */
  {
    file = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).cron_files;
    loop {
      if file.is_null() {
        break 's_5;
      }
      if (*file).cf_deleted == 0 {
        delete_cronfile((*file).cf_username);
        break;
      } else {
        file = (*file).cf_next
      }
    }
  }
  /* Remove cron update file */
  unlink(b"cron.update\x00" as *const u8 as *const libc::c_char);
  /* Re-chdir, in case directory was renamed & deleted */
  xchdir((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).crontab_dir_name);
  /* Scan directory and add associated users */
  let mut dir: *mut DIR = opendir(b".\x00" as *const u8 as *const libc::c_char);
  let mut den: *mut dirent = 0 as *mut dirent;
  /* xopendir exists, but "can't open '.'" is not informative */
  if dir.is_null() {
    bb_error_msg_and_die(
      b"can\'t open \'%s\'\x00" as *const u8 as *const libc::c_char,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).crontab_dir_name,
    );
  }
  loop {
    den = readdir(dir);
    if den.is_null() {
      break;
    }
    if !strchr((*den).d_name.as_mut_ptr(), '.' as i32).is_null() {
      continue;
    }
    load_crontab((*den).d_name.as_mut_ptr());
  }
  closedir(dir);
}
/* We set environment *before* vfork (because we want to use vfork),
 * so we cannot use setenv() - repeated calls to setenv() may leak memory!
 * Using putenv(), and freeing memory after unsetenv() won't leak */
unsafe extern "C" fn safe_setenv(
  mut pvar_val: *mut *mut libc::c_char,
  mut var: *const libc::c_char,
  mut val: *const libc::c_char,
) {
  let mut var_val: *mut libc::c_char = *pvar_val;
  if !var_val.is_null() {
    bb_unsetenv_and_free(var_val);
  }
  *pvar_val = xasprintf(b"%s=%s\x00" as *const u8 as *const libc::c_char, var, val);
  putenv(*pvar_val);
}
unsafe extern "C" fn set_env_vars(mut pas: *mut passwd, mut shell: *const libc::c_char) {
  /* POSIX requires crond to set up at least HOME, LOGNAME, PATH, SHELL.
   * We assume crond inherited suitable PATH.
   */
  safe_setenv(
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).env_var_logname,
    b"LOGNAME\x00" as *const u8 as *const libc::c_char,
    (*pas).pw_name,
  );
  safe_setenv(
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).env_var_user,
    b"USER\x00" as *const u8 as *const libc::c_char,
    (*pas).pw_name,
  );
  safe_setenv(
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).env_var_home,
    b"HOME\x00" as *const u8 as *const libc::c_char,
    (*pas).pw_dir,
  );
  safe_setenv(
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).env_var_shell,
    b"SHELL\x00" as *const u8 as *const libc::c_char,
    shell,
  );
}
unsafe extern "C" fn change_user(mut pas: *mut passwd) {
  /* careful: we're after vfork! */
  change_identity(pas); /* - initgroups, setgid, setuid */
  if chdir((*pas).pw_dir) < 0i32 {
    bb_error_msg(
      b"can\'t change directory to \'%s\'\x00" as *const u8 as *const libc::c_char,
      (*pas).pw_dir,
    );
    xchdir(b"/var/spool/cron\x00" as *const u8 as *const libc::c_char);
  };
}
// TODO: sendmail should be _run-time_ option, not compile-time!
unsafe extern "C" fn fork_job(
  mut user: *const libc::c_char,
  mut mailFd: libc::c_int,
  mut line: *mut CronLine,
  mut run_sendmail: bool,
) -> pid_t {
  let mut current_block: u64;
  let mut pas: *mut passwd = 0 as *mut passwd;
  let mut shell: *const libc::c_char = 0 as *const libc::c_char;
  let mut prog: *const libc::c_char = 0 as *const libc::c_char;
  let mut sv_logmode: smallint = 0;
  let mut pid: pid_t = 0;
  /* prepare things before vfork */
  pas = bb_internal_getpwnam(user);
  if pas.is_null() {
    bb_error_msg(
      b"can\'t get uid for %s\x00" as *const u8 as *const libc::c_char,
      user,
    );
    current_block = 12835897963350252569;
  } else {
    shell = if !(*line).cl_shell.is_null() {
      (*line).cl_shell
    } else {
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).default_shell
    };
    prog = if run_sendmail as libc::c_int != 0 {
      b"sendmail\x00" as *const u8 as *const libc::c_char
    } else {
      shell
    };
    set_env_vars(pas, shell);
    sv_logmode = logmode;
    pid = vfork();
    if pid == 0i32 {
      /* CHILD */
      /* initgroups, setgid, setuid, and chdir to home or CRON_DIR */
      change_user(pas);
      log5(
        b"child running %s\x00" as *const u8 as *const libc::c_char,
        prog,
      );
      if mailFd >= 0i32 {
        xmove_fd(
          mailFd,
          if run_sendmail as libc::c_int != 0 {
            0i32
          } else {
            1i32
          },
        );
        dup2(1i32, 2i32);
      }
      /* crond 3.0pl1-100 puts tasks in separate process groups */
      setpgrp();
      if !run_sendmail {
        execlp(
          prog,
          prog,
          b"-c\x00" as *const u8 as *const libc::c_char,
          (*line).cl_cmd,
          0 as *mut libc::c_void as *mut libc::c_char,
        );
      } else {
        execlp(
          prog,
          prog,
          b"-ti\x00" as *const u8 as *const libc::c_char,
          0 as *mut libc::c_void as *mut libc::c_char,
        );
      }
      /*
       * I want this error message on stderr too,
       * even if other messages go only to syslog:
       */
      logmode = (logmode as libc::c_int | LOGMODE_STDIO as libc::c_int) as smallint; /* else: PARENT, FORK SUCCESS */
      bb_error_msg_and_die(
        b"can\'t execute \'%s\' for user %s\x00" as *const u8 as *const libc::c_char,
        prog,
        user,
      );
    }
    logmode = sv_logmode;
    if pid < 0i32 {
      bb_simple_perror_msg(b"vfork\x00" as *const u8 as *const libc::c_char);
      current_block = 12835897963350252569;
    } else {
      current_block = 18386322304582297246;
    }
  }
  match current_block {
    12835897963350252569 => pid = 0i32,
    _ => {}
  }
  /*
   * Close the mail file descriptor.. we can't just leave it open in
   * a structure, closing it later, because we might run out of descriptors
   */
  if mailFd >= 0i32 {
    close(mailFd);
  }
  return pid;
}
unsafe extern "C" fn start_one_job(
  mut user: *const libc::c_char,
  mut line: *mut CronLine,
) -> pid_t {
  let mut mailFile: [libc::c_char; 128] = [0; 128];
  let mut mailFd: libc::c_int = -1i32;
  (*line).cl_pid = 0i32;
  (*line).cl_empty_mail_size = 0i32;
  if !(*line).cl_mailto.is_null() {
    /* Open mail file (owner is root so nobody can screw with it) */
    snprintf(
      mailFile.as_mut_ptr(),
      ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
      b"%s/cron.%s.%d\x00" as *const u8 as *const libc::c_char,
      b"/var/spool/cron\x00" as *const u8 as *const libc::c_char,
      user,
      getpid(),
    );
    mailFd = open(
      mailFile.as_mut_ptr(),
      0o100i32 | 0o1000i32 | 0o1i32 | 0o200i32 | 0o2000i32,
      0o600i32,
    );
    if mailFd >= 0i32 {
      dprintf(
        mailFd,
        b"To: %s\nSubject: cron: %s\n\n\x00" as *const u8 as *const libc::c_char,
        (*line).cl_mailto,
        (*line).cl_cmd,
      );
      (*line).cl_empty_mail_size = lseek(mailFd, 0i32 as off64_t, 1i32) as libc::c_int
    } else {
      bb_error_msg(
        b"can\'t create mail file %s for user %s, discarding output\x00" as *const u8
          as *const libc::c_char,
        mailFile.as_mut_ptr(),
        user,
      );
    }
  }
  (*line).cl_pid = fork_job(user, mailFd, line, 0i32 != 0);
  if mailFd >= 0i32 {
    if (*line).cl_pid <= 0i32 {
      unlink(mailFile.as_mut_ptr());
    } else {
      /* rename mail-file based on pid of process */
      let mut mailFile2: *mut libc::c_char = xasprintf(
        b"%s/cron.%s.%d\x00" as *const u8 as *const libc::c_char,
        b"/var/spool/cron\x00" as *const u8 as *const libc::c_char,
        user,
        (*line).cl_pid,
      ); // TODO: xrename?
      rename(mailFile.as_mut_ptr(), mailFile2);
      free(mailFile2 as *mut libc::c_void);
    }
  }
  return (*line).cl_pid;
}
/*
 * process_finished_job - called when job terminates and when mail terminates
 */
unsafe extern "C" fn process_finished_job(mut user: *const libc::c_char, mut line: *mut CronLine) {
  let mut pid: pid_t = 0;
  let mut mailFd: libc::c_int = 0;
  let mut mailFile: [libc::c_char; 128] = [0; 128];
  let mut sbuf: stat = std::mem::zeroed();
  pid = (*line).cl_pid;
  (*line).cl_pid = 0i32;
  if pid <= 0i32 {
    /* No job */
    return;
  }
  if (*line).cl_empty_mail_size <= 0i32 {
    /* End of job and no mail file, or end of sendmail job */
    return;
  }
  /*
   * End of primary job - check for mail file.
   * If size has changed and the file is still valid, we send it.
   */
  snprintf(
    mailFile.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
    b"%s/cron.%s.%d\x00" as *const u8 as *const libc::c_char,
    b"/var/spool/cron\x00" as *const u8 as *const libc::c_char,
    user,
    pid,
  );
  mailFd = open(mailFile.as_mut_ptr(), 0i32);
  unlink(mailFile.as_mut_ptr());
  if mailFd < 0i32 {
    return;
  }
  if fstat(mailFd, &mut sbuf) < 0i32
    || sbuf.st_uid != 0i32 as libc::c_uint
    || sbuf.st_nlink != 0i32 as libc::c_ulong
    || sbuf.st_size == (*line).cl_empty_mail_size as libc::c_long
    || !(sbuf.st_mode & 0o170000i32 as libc::c_uint == 0o100000i32 as libc::c_uint)
  {
    close(mailFd);
    return;
  }
  (*line).cl_empty_mail_size = 0i32;
  /* if (line->cl_mailto) - always true if cl_empty_mail_size was nonzero */
  (*line).cl_pid = fork_job(user, mailFd, line, 1i32 != 0);
}
/* !ENABLE_FEATURE_CROND_CALL_SENDMAIL */
/* !ENABLE_FEATURE_CROND_CALL_SENDMAIL */
/*
 * Determine which jobs need to be run.  Under normal conditions, the
 * period is about a minute (one scan).  Worst case it will be one
 * hour (60 scans).
 */
unsafe extern "C" fn flag_starting_jobs(mut t1: time_t, mut t2: time_t) {
  let mut t: time_t = 0;
  /* Find jobs > t1 and <= t2 */
  t = t1 - t1 % 60i32 as libc::c_long;
  while t <= t2 {
    let mut ptm: *mut tm = 0 as *mut tm;
    let mut file: *mut CronFile = 0 as *mut CronFile;
    let mut line: *mut CronLine = 0 as *mut CronLine;
    if !(t <= t1) {
      ptm = localtime(&mut t);
      file = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).cron_files;
      while !file.is_null() {
        log5(
          b"file %s:\x00" as *const u8 as *const libc::c_char,
          (*file).cf_username,
        );
        if !((*file).cf_deleted != 0) {
          line = (*file).cf_lines;
          while !line.is_null() {
            log5(
              b" line %s\x00" as *const u8 as *const libc::c_char,
              (*line).cl_cmd,
            );
            if (*line).cl_Mins[(*ptm).tm_min as usize] as libc::c_int != 0
              && (*line).cl_Hrs[(*ptm).tm_hour as usize] as libc::c_int != 0
              && ((*line).cl_Days[(*ptm).tm_mday as usize] as libc::c_int != 0
                || (*line).cl_Dow[(*ptm).tm_wday as usize] as libc::c_int != 0)
              && (*line).cl_Mons[(*ptm).tm_mon as usize] as libc::c_int != 0
            {
              log5(
                b" job: %d %s\x00" as *const u8 as *const libc::c_char,
                (*line).cl_pid,
                (*line).cl_cmd,
              );
              if (*line).cl_pid > 0i32 {
                log8(
                  b"user %s: process already running: %s\x00" as *const u8 as *const libc::c_char,
                  (*file).cf_username,
                  (*line).cl_cmd,
                );
              } else if (*line).cl_pid == 0i32 {
                (*line).cl_pid = -1i32;
                (*file).cf_wants_starting = 1i32 as smallint
              }
            }
            line = (*line).cl_next
          }
        }
        file = (*file).cf_next
      }
    }
    t += 60i32 as libc::c_long
  }
}
unsafe extern "C" fn touch_reboot_file() -> libc::c_int {
  let mut fd: libc::c_int = open(
    b"/var/run/crond.reboot\x00" as *const u8 as *const libc::c_char,
    0o1i32 | 0o100i32 | 0o200i32 | 0o1000i32,
    0i32,
  );
  if fd >= 0i32 {
    close(fd);
    return 1i32;
  }
  /* File (presumably) exists - this is not the first run after reboot */
  return 0i32;
}
unsafe extern "C" fn start_jobs(mut wants_start: libc::c_int) {
  let mut file: *mut CronFile = 0 as *mut CronFile;
  let mut line: *mut CronLine = 0 as *mut CronLine;
  file = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).cron_files;
  while !file.is_null() {
    if !((*file).cf_wants_starting == 0) {
      (*file).cf_wants_starting = 0i32 as smallint;
      line = (*file).cf_lines;
      while !line.is_null() {
        let mut pid: pid_t = 0;
        if !((*line).cl_pid != wants_start) {
          pid = start_one_job((*file).cf_username, line);
          log8(
            b"USER %s pid %3d cmd %s\x00" as *const u8 as *const libc::c_char,
            (*file).cf_username,
            pid,
            (*line).cl_cmd,
          );
          if pid < 0i32 {
            (*file).cf_wants_starting = 1i32 as smallint
          }
          if pid > 0i32 {
            (*file).cf_has_running = 1i32 as smallint
          }
        }
        line = (*line).cl_next
      }
    }
    file = (*file).cf_next
  }
}
/*
 * Check for job completion, return number of jobs still running after
 * all done.
 */
unsafe extern "C" fn check_completions() -> libc::c_int {
  let mut file: *mut CronFile = 0 as *mut CronFile;
  let mut line: *mut CronLine = 0 as *mut CronLine;
  let mut num_still_running: libc::c_int = 0i32;
  file = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).cron_files;
  while !file.is_null() {
    if !((*file).cf_has_running == 0) {
      (*file).cf_has_running = 0i32 as smallint;
      let mut current_block_5: u64;
      line = (*file).cf_lines;
      while !line.is_null() {
        let mut r: libc::c_int = 0;
        if !((*line).cl_pid <= 0i32) {
          r = waitpid((*line).cl_pid, 0 as *mut libc::c_int, 1i32);
          if r < 0i32 || r == (*line).cl_pid {
            process_finished_job((*file).cf_username, line);
            if (*line).cl_pid == 0i32 {
              current_block_5 = 2473556513754201174;
            } else {
              current_block_5 = 12349973810996921269;
            }
          /* else: sendmail was started, job is still running, fall thru */
          } else {
            current_block_5 = 12349973810996921269;
          }
          match current_block_5 {
            2473556513754201174 => {}
            _ => {
              /* else: r == 0: "process is still running" */
              (*file).cf_has_running = 1i32 as smallint
            }
          }
        }
        /* sendmail was not started for it */
        line = (*line).cl_next
      }
      //FIXME: if !file->cf_has_running && file->deleted: delete it!
      //otherwise deleted entries will stay forever, right?
      num_still_running += (*file).cf_has_running as libc::c_int
    }
    file = (*file).cf_next
  }
  return num_still_running;
}
unsafe extern "C" fn reopen_logfile_to_stderr() {
  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .log_filename
    .is_null()
  {
    let mut logfd: libc::c_int = open_or_warn(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).log_filename,
      0o1i32 | 0o100i32 | 0o2000i32,
    );
    if logfd >= 0i32 {
      xmove_fd(logfd, 2i32);
    }
  };
}
#[no_mangle]
pub unsafe extern "C" fn crond_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut t2: time_t = 0;
  let mut rescan: libc::c_uint = 0;
  let mut sleep_time: libc::c_uint = 0;
  let mut opts: libc::c_uint = 0;
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).log_level = 8i32 as libc::c_uint;
  let ref mut fresh1 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).crontab_dir_name;
  *fresh1 = b"/var/spool/cron/crontabs\x00" as *const u8 as *const libc::c_char;
  opts = getopt32(
    argv,
    b"^l:L:fbSc:d:\x00f-b:b-f:S-L:L-S:d-l:l+:d+\x00" as *const u8 as *const libc::c_char,
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).log_level as *mut libc::c_uint,
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).log_filename
      as *mut *const libc::c_char,
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).crontab_dir_name
      as *mut *const libc::c_char,
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).log_level as *mut libc::c_uint,
  );
  /* both -d N and -l N set the same variable: G.log_level */
  if opts & OPT_f as libc::c_int as libc::c_uint == 0 {
    /* close stdin, stdout, stderr.
     * close unused descriptors - don't need them. */
    bb_daemonize_or_rexec(DAEMON_CLOSE_EXTRA_FDS as libc::c_int);
  }
  if opts & OPT_d as libc::c_int as libc::c_uint == 0
    && (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .log_filename
      .is_null()
  {
    /* logging to syslog */
    openlog(applet_name, 0x2i32 | 0x1i32, 9i32 << 3i32);
    logmode = LOGMODE_SYSLOG as libc::c_int as smallint
  }
  //signal(SIGHUP, SIG_IGN); /* ? original crond dies on HUP... */
  reopen_logfile_to_stderr();
  xchdir((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).crontab_dir_name);
  /* $SHELL, or current UID's shell, or DEFAULT_SHELL */
  /* Useful on Android where DEFAULT_SHELL /bin/sh may not exist */
  let ref mut fresh2 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).default_shell; /* start @reboot entries, if any */
  *fresh2 = xstrdup(get_shell_name());
  log8(
    b"crond (busybox 1.32.0.git) started, log level %d\x00" as *const u8 as *const libc::c_char,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).log_level,
  );
  rescan_crontab_dir();
  write_pidfile_std_path_and_ext(b"crond\x00" as *const u8 as *const libc::c_char);
  if touch_reboot_file() != 0 {
    start_jobs(-2i32);
  }
  /* Main loop */
  t2 = time(0 as *mut time_t); /* for (;;) */
  rescan = 60i32 as libc::c_uint;
  sleep_time = 60i32 as libc::c_uint;
  loop
  /* else: time jumped back, do not run any jobs */
  {
    let mut sbuf: stat = std::mem::zeroed();
    let mut t1: time_t = 0;
    let mut dt: libc::c_long = 0;
    /* Synchronize to 1 minute, minimum 1 second */
    t1 = t2;
    sleep(
      (sleep_time as libc::c_long - time(0 as *mut time_t) % sleep_time as libc::c_long)
        as libc::c_uint,
    );
    t2 = time(0 as *mut time_t);
    dt = t2 - t1;
    reopen_logfile_to_stderr();
    /*
     * The file 'cron.update' is checked to determine new cron
     * jobs.  The directory is rescanned once an hour to deal
     * with any screwups.
     *
     * Check for time jump.  Disparities over an hour either way
     * result in resynchronization.  A negative disparity
     * less than an hour causes us to effectively sleep until we
     * match the original time (i.e. no re-execution of jobs that
     * have just been run).  A positive disparity less than
     * an hour causes intermediate jobs to be run, but only once
     * in the worst case.
     *
     * When running jobs, the inequality used is greater but not
     * equal to t1, and less then or equal to t2.
     */
    if stat(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).crontab_dir_name,
      &mut sbuf,
    ) != 0i32
    {
      sbuf.st_mtime = 0i32 as time_t
    } /* force update (once) if dir was deleted */
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).crontab_dir_mtime != sbuf.st_mtime {
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).crontab_dir_mtime = sbuf.st_mtime;
      rescan = 1i32 as libc::c_uint
    }
    rescan = rescan.wrapping_sub(1);
    if rescan == 0i32 as libc::c_uint {
      rescan = 60i32 as libc::c_uint;
      rescan_crontab_dir();
    }
    process_cron_update_file();
    log5(b"wakeup dt=%ld\x00" as *const u8 as *const libc::c_char, dt);
    if dt < (-60i32 * 60i32) as libc::c_long || dt > (60i32 * 60i32) as libc::c_long {
      bb_info_msg(
        b"time disparity of %ld minutes detected\x00" as *const u8 as *const libc::c_char,
        dt / 60i32 as libc::c_long,
      );
    /* and we do not run any jobs in this case */
    } else if dt > 0 {
      /* Usual case: time advances forward, as expected */
      flag_starting_jobs(t1, t2);
      start_jobs(-1i32);
      sleep_time = 60i32 as libc::c_uint;
      if check_completions() > 0i32 {
        /* some jobs are still running */
        sleep_time = 10i32 as libc::c_uint
      }
    }
  }
  /* not reached */
}
