use c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use libc;
extern "C" {
  pub type __dirstream;
  #[no_mangle]
  fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
  #[no_mangle]
  fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
  #[no_mangle]
  fn flock(__fd: libc::c_int, __operation: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  fn fsync(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn opendir(__name: *const libc::c_char) -> *mut DIR;
  #[no_mangle]
  fn closedir(__dirp: *mut DIR) -> libc::c_int;
  #[no_mangle]
  fn readdir(__dirp: *mut DIR) -> *mut dirent;
  #[no_mangle]
  fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
  #[no_mangle]
  fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn sigprocmask(__how: libc::c_int, __set: *const sigset_t, __oset: *mut sigset_t) -> libc::c_int;
  #[no_mangle]
  static mut stderr: *mut FILE;
  #[no_mangle]
  fn rename(__old: *const libc::c_char, __new: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn fclose(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn fflush(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn setvbuf(
    __stream: *mut FILE,
    __buf: *mut libc::c_char,
    __modes: libc::c_int,
    __n: size_t,
  ) -> libc::c_int;
  #[no_mangle]
  fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn asprintf(__ptr: *mut *mut libc::c_char, __fmt: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn fwrite(__ptr: *const libc::c_void, __size: size_t, __n: size_t, __s: *mut FILE) -> size_t;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn unlink(__name: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn vfork() -> libc::c_int;
  #[no_mangle]
  fn execl(__path: *const libc::c_char, __arg: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn fchdir(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memchr(_: *const libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memrchr(__s: *const libc::c_void, __c: libc::c_int, __n: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn chmod(__file: *const libc::c_char, __mode: mode_t) -> libc::c_int;
  #[no_mangle]
  fn fchmod(__fd: libc::c_int, __mode: mode_t) -> libc::c_int;
  #[no_mangle]
  fn gettimeofday(__tv: *mut timeval, __tz: __timezone_ptr_t) -> libc::c_int;
  #[no_mangle]
  fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn monotonic_sec() -> libc::c_uint;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn close_on_exec_on(fd: libc::c_int);
  #[no_mangle]
  fn xmove_fd(_: libc::c_int, _: libc::c_int);
  #[no_mangle]
  fn bb_signals_recursive_norestart(
    sigs: libc::c_int,
    f: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
  );
  #[no_mangle]
  fn sig_block(sig: libc::c_int);
  #[no_mangle]
  fn sig_unblock(sig: libc::c_int);
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn open_read_close(
    filename: *const libc::c_char,
    buf: *mut libc::c_void,
    maxsz: size_t,
  ) -> ssize_t;
  #[no_mangle]
  fn xatou_range(str: *const libc::c_char, l: libc::c_uint, u: libc::c_uint) -> libc::c_uint;
  #[no_mangle]
  fn xatou_sfx(str: *const libc::c_char, sfx: *const suffix_mult) -> libc::c_uint;
  #[no_mangle]
  static bkm_suffixes: [suffix_mult; 0];
  #[no_mangle]
  fn bin2hex(
    dst: *mut libc::c_char,
    src: *const libc::c_char,
    count: libc::c_int,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn xatoi_positive(numstr: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn safe_waitpid(pid: pid_t, wstat: *mut libc::c_int, options: libc::c_int) -> pid_t;
  #[no_mangle]
  fn wait_any_nohang(wstat: *mut libc::c_int) -> pid_t;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_simple_error_msg(s: *const libc::c_char);
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_perror_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_perror_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn fflush_all() -> libc::c_int;
  #[no_mangle]
  fn get_shell_name() -> *const libc::c_char;
  /* '*const' ptr makes gcc optimize code much better.
   * Magic prevents ptr_to_globals from going into rodata.
   * If you want to assign a value, use SET_PTR_TO_GLOBALS(x) */
  #[no_mangle]
  static ptr_to_globals: *mut globals;
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
}

use libc::ino64_t;
use libc::mode_t;

use libc::off64_t;

use crate::librb::pid_t;
use crate::librb::size_t;
use crate::librb::smallint;
use crate::librb::ssize_t;
use libc::stat;


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

use libc::time_t;
use crate::librb::signal::sigset_t;
 use libc::timeval;

use libc::FILE;
pub type nfds_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
  pub fd: libc::c_int,
  pub events: libc::c_short,
  pub revents: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timezone {
  pub tz_minuteswest: libc::c_int,
  pub tz_dsttime: libc::c_int,
}
pub type __timezone_ptr_t = *mut timezone;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
  pub tm_sec: libc::c_int,
  pub tm_min: libc::c_int,
  pub tm_hour: libc::c_int,
  pub tm_mday: libc::c_int,
  pub tm_mon: libc::c_int,
  pub tm_year: libc::c_int,
  pub tm_wday: libc::c_int,
  pub tm_yday: libc::c_int,
  pub tm_isdst: libc::c_int,
  pub tm_gmtoff: libc::c_long,
  pub tm_zone: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct suffix_mult {
  pub suffix: [libc::c_char; 4],
  pub mult: libc::c_uint,
}
/* Some older linkers don't perform string merging, we used to have common strings
 * as global arrays to do it by hand. But:
 * (1) newer linkers do it themselves,
 * (2) however, they DONT merge string constants with global arrays,
 * even if the value is the same (!). Thus global arrays actually
 * increased size a bit: for example, "/etc/passwd" string from libc
 * wasn't merged with bb_path_passwd_file[] array!
 * Therefore now we use #defines.
 */
/* "BusyBox vN.N.N (timestamp or extra_version)" */
/* NB: (bb_hexdigits_upcase[i] | 0x20) -> lowercase hex digit */
/* Busybox mount uses either /proc/mounts or /etc/mtab to
 * get the list of currently mounted filesystems */
/* allow default system PATH to be extended via CFLAGS */
/* BB_PATH_ROOT_PATH */
/* util-linux manpage says /sbin:/bin:/usr/sbin:/usr/bin,
 * but I want to save a few bytes here:
 */
//extern const int const_int_1;
/* This struct is deliberately not defined. */
/* See docs/keep_data_small.txt */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub dir: *mut logdir,
  pub verbose: libc::c_uint,
  pub linemax: libc::c_int,
  pub linelen: libc::c_int,
  pub fdwdir: libc::c_int,
  pub fndir: *mut *mut libc::c_char,
  pub wstat: libc::c_int,
  pub nearest_rotate: libc::c_uint,
  pub memRchr: Option<
    unsafe extern "C" fn(_: *const libc::c_void, _: libc::c_int, _: size_t) -> *mut libc::c_void,
  >,
  pub shell: *mut libc::c_char,
  pub exitasap: smallint,
  pub rotateasap: smallint,
  pub reopenasap: smallint,
  pub linecomplete: smallint,
  pub tmaxflag: smallint,
  pub repl: libc::c_char,
  pub replace: *const libc::c_char,
  pub fl_flag_0: libc::c_int,
  pub dirn: libc::c_uint,
  pub blocked_sigset: sigset_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct logdir {
  pub inst: *mut libc::c_char,
  pub processor: *mut libc::c_char,
  pub name: *mut libc::c_char,
  pub size: libc::c_uint,
  pub sizemax: libc::c_uint,
  pub nmax: libc::c_uint,
  pub nmin: libc::c_uint,
  pub rotate_period: libc::c_uint,
  pub ppid: libc::c_int,
  pub fddir: libc::c_int,
  pub fdcur: libc::c_int,
  pub filecur: *mut FILE,
  pub fdlock: libc::c_int,
  pub next_rotate: libc::c_uint,
  pub fnsave: [libc::c_char; 30],
  pub match_0: libc::c_char,
  pub matcherr: libc::c_char,
}
pub type C2RustUnnamed = libc::c_uint;
pub const COMMON_BUFSIZE: C2RustUnnamed = 1024;
#[inline(always)]
unsafe extern "C" fn not_const_pp(mut p: *const libc::c_void) -> *mut libc::c_void {
  return p as *mut libc::c_void;
}
/*buflen = 1024;*/
unsafe extern "C" fn fatalx(mut m0: *const libc::c_char) {
  bb_error_msg_and_die(b"fatal: %s\x00" as *const u8 as *const libc::c_char, m0);
}
unsafe extern "C" fn warn(mut m0: *const libc::c_char) {
  bb_perror_msg(b"warning: %s\x00" as *const u8 as *const libc::c_char, m0);
}
unsafe extern "C" fn warn2(mut m0: *const libc::c_char, mut m1: *const libc::c_char) {
  bb_perror_msg(
    b"warning: %s: %s\x00" as *const u8 as *const libc::c_char,
    m0,
    m1,
  );
}
unsafe extern "C" fn warnx(mut m0: *const libc::c_char, mut m1: *const libc::c_char) {
  bb_error_msg(
    b"warning: %s: %s\x00" as *const u8 as *const libc::c_char,
    m0,
    m1,
  );
}
unsafe extern "C" fn pause_nomem() {
  bb_simple_error_msg(b"pausing: out of memory\x00" as *const u8 as *const libc::c_char);
  sleep(3i32 as libc::c_uint);
}
unsafe extern "C" fn pause1cannot(mut m0: *const libc::c_char) {
  bb_perror_msg(
    b"pausing: can\'t %s\x00" as *const u8 as *const libc::c_char,
    m0,
  );
  sleep(3i32 as libc::c_uint);
}
unsafe extern "C" fn pause2cannot(mut m0: *const libc::c_char, mut m1: *const libc::c_char) {
  bb_perror_msg(
    b"pausing: can\'t %s %s\x00" as *const u8 as *const libc::c_char,
    m0,
    m1,
  );
  sleep(3i32 as libc::c_uint);
}
unsafe extern "C" fn wstrdup(mut str: *const libc::c_char) -> *mut libc::c_char {
  let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
  loop {
    s = strdup(str);
    if !s.is_null() {
      break;
    }
    pause_nomem();
  }
  return s;
}
unsafe extern "C" fn pmatch(
  mut p: *const libc::c_char,
  mut s: *const libc::c_char,
  mut len: libc::c_uint,
) -> libc::c_uint {
  loop {
    let fresh0 = p;
    p = p.offset(1);
    let mut c: libc::c_char = *fresh0;
    if c == 0 {
      return (len == 0) as libc::c_int as libc::c_uint;
    }
    match c as libc::c_int {
      42 => {
        c = *p;
        if c == 0 {
          return 1i32 as libc::c_uint;
        }
        loop {
          if len == 0 {
            return 0i32 as libc::c_uint;
          }
          if *s as libc::c_int == c as libc::c_int {
            break;
          }
          s = s.offset(1);
          len = len.wrapping_sub(1)
        }
      }
      43 => {
        let fresh1 = p;
        p = p.offset(1);
        c = *fresh1;
        if c as libc::c_int != *s as libc::c_int {
          return 0i32 as libc::c_uint;
        }
        loop {
          if len == 0 {
            return 1i32 as libc::c_uint;
          }
          if *s as libc::c_int != c as libc::c_int {
            break;
          }
          s = s.offset(1);
          len = len.wrapping_sub(1)
        }
      }
      _ => {
        /*
        case '?':
          if (*p == '?') {
            if (*s != '?') return 0;
            ++p;
          }
          ++s; --len;
          continue;
          */
        if len == 0 {
          return 0i32 as libc::c_uint;
        }
        if *s as libc::c_int != c as libc::c_int {
          return 0i32 as libc::c_uint;
        }
        s = s.offset(1);
        len = len.wrapping_sub(1)
      }
    }
  }
}
/* ** ex fmt_ptime.[ch] ***/
/* NUL terminated */
unsafe extern "C" fn fmt_time_human_30nul(mut s: *mut libc::c_char, mut dt_delim: libc::c_char) {
  let mut tm: tm = tm {
    tm_sec: 0,
    tm_min: 0,
    tm_hour: 0,
    tm_mday: 0,
    tm_mon: 0,
    tm_year: 0,
    tm_wday: 0,
    tm_yday: 0,
    tm_isdst: 0,
    tm_gmtoff: 0,
    tm_zone: 0 as *const libc::c_char,
  };
  let mut ptm: *mut tm = 0 as *mut tm;
  let mut tv: timeval = timeval {
    tv_sec: 0,
    tv_usec: 0,
  };
  gettimeofday(&mut tv, 0 as *mut timezone);
  ptm = gmtime_r(&mut tv.tv_sec, &mut tm);
  /* ^^^ using gmtime_r() instead of gmtime() to not use static data */
  sprintf(
    s,
    b"%04u-%02u-%02u%c%02u:%02u:%02u.%06u000\x00" as *const u8 as *const libc::c_char,
    (1900i32 + (*ptm).tm_year) as libc::c_uint,
    ((*ptm).tm_mon + 1i32) as libc::c_uint,
    (*ptm).tm_mday as libc::c_uint,
    dt_delim as libc::c_int,
    (*ptm).tm_hour as libc::c_uint,
    (*ptm).tm_min as libc::c_uint,
    (*ptm).tm_sec as libc::c_uint,
    tv.tv_usec as libc::c_uint,
  );
  /* 4+1 + 2+1 + 2+1 + 2+1 + 2+1 + 2+1 + 9 = */
  /* 5   + 3   + 3   + 3   + 3   + 3   + 9 = */
  /* 20 (up to '.' inclusive) + 9 (not including '\0') */
}
/* NOT terminated! */
unsafe extern "C" fn fmt_time_bernstein_25(mut s: *mut libc::c_char) {
  let mut pack: [u32; 3] = [0; 3];
  let mut tv: timeval = timeval {
    tv_sec: 0,
    tv_usec: 0,
  };
  let mut sec_hi: libc::c_uint = 0;
  gettimeofday(&mut tv, 0 as *mut timezone);
  sec_hi =
    (0x400000000000000au64.wrapping_add(tv.tv_sec as libc::c_ulonglong) >> 32i32) as libc::c_uint;
  tv.tv_sec = 0x400000000000000au64 as time_t + tv.tv_sec;
  tv.tv_usec *= 1000i32 as libc::c_long;
  /* Network order is big-endian: most significant byte first.
   * This is exactly what we want here */
  pack[0] = {
    let mut __v: libc::c_uint = 0;
    let mut __x: libc::c_uint = sec_hi;
    if 0 != 0 {
      __v = (__x & 0xff000000u32) >> 24i32
        | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
        | (__x & 0xff00i32 as libc::c_uint) << 8i32
        | (__x & 0xffi32 as libc::c_uint) << 24i32
    } else {
      let fresh2 = &mut __v;
      let fresh3;
      let fresh4 = __x;
      asm!("bswap $0" : "=r" (fresh3) : "0"
                      (c2rust_asm_casts::AsmCast::cast_in(fresh2, fresh4)) :);
      c2rust_asm_casts::AsmCast::cast_out(fresh2, fresh4, fresh3);
    }
    __v
  };
  pack[1] = {
    let mut __v: libc::c_uint = 0;
    let mut __x: libc::c_uint = tv.tv_sec as libc::c_uint;
    if 0 != 0 {
      __v = (__x & 0xff000000u32) >> 24i32
        | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
        | (__x & 0xff00i32 as libc::c_uint) << 8i32
        | (__x & 0xffi32 as libc::c_uint) << 24i32
    } else {
      let fresh5 = &mut __v;
      let fresh6;
      let fresh7 = __x;
      asm!("bswap $0" : "=r" (fresh6) : "0"
                      (c2rust_asm_casts::AsmCast::cast_in(fresh5, fresh7)) :);
      c2rust_asm_casts::AsmCast::cast_out(fresh5, fresh7, fresh6);
    }
    __v
  };
  pack[2] = {
    let mut __v: libc::c_uint = 0;
    let mut __x: libc::c_uint = tv.tv_usec as libc::c_uint;
    if 0 != 0 {
      __v = (__x & 0xff000000u32) >> 24i32
        | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
        | (__x & 0xff00i32 as libc::c_uint) << 8i32
        | (__x & 0xffi32 as libc::c_uint) << 24i32
    } else {
      let fresh8 = &mut __v;
      let fresh9;
      let fresh10 = __x;
      asm!("bswap $0" : "=r" (fresh9) : "0"
                      (c2rust_asm_casts::AsmCast::cast_in(fresh8, fresh10))
                      :);
      c2rust_asm_casts::AsmCast::cast_out(fresh8, fresh10, fresh9);
    }
    __v
  };
  let fresh11 = s;
  s = s.offset(1);
  *fresh11 = '@' as i32 as libc::c_char;
  bin2hex(s, pack.as_mut_ptr() as *mut libc::c_char, 12i32);
}
unsafe extern "C" fn processorstart(mut ld: *mut logdir) {
  let mut sv_ch: libc::c_char = 0;
  let mut pid: libc::c_int = 0;
  if (*ld).processor.is_null() {
    return;
  }
  if (*ld).ppid != 0 {
    warnx(
      b"processor already running\x00" as *const u8 as *const libc::c_char,
      (*ld).name,
    );
    return;
  }
  /* vfork'ed child trashes this byte, save... */
  sv_ch = (*ld).fnsave[26];
  if (*ptr_to_globals).shell.is_null() {
    (*ptr_to_globals).shell = xstrdup(get_shell_name())
  }
  loop {
    pid = vfork();
    if !(pid == -1i32) {
      break;
    }
    pause2cannot(
      b"vfork for processor\x00" as *const u8 as *const libc::c_char,
      (*ld).name,
    );
  }
  if pid == 0 {
    let mut fd: libc::c_int = 0;
    /* child */
    /* Non-ignored signals revert to SIG_DFL on exec anyway */
    /*bb_signals(0
    + (1 << SIGTERM)
    + (1 << SIGALRM)
    + (1 << SIGHUP)
    , SIG_DFL);*/
    sig_unblock(15i32); /* <- that's why we need sv_ch! */
    sig_unblock(14i32); /* ...restore */
    sig_unblock(1i32);
    if (*ptr_to_globals).verbose != 0 {
      bb_error_msg(
        b"info: processing: %s/%s\x00" as *const u8 as *const libc::c_char,
        (*ld).name,
        (*ld).fnsave.as_mut_ptr(),
      );
    }
    fd = xopen((*ld).fnsave.as_mut_ptr(), 0i32 | 0o4000i32);
    xmove_fd(fd, 0i32);
    (*ld).fnsave[26] = 't' as i32 as libc::c_char;
    fd = xopen(
      (*ld).fnsave.as_mut_ptr(),
      0o1i32 | 0o4000i32 | 0o1000i32 | 0o100i32,
    );
    xmove_fd(fd, 1i32);
    fd = open(
      b"state\x00" as *const u8 as *const libc::c_char,
      0i32 | 0o4000i32,
    );
    if fd == -1i32 {
      if *bb_errno != 2i32 {
        bb_perror_msg_and_die(
          b"fatal: can\'t %s processor %s\x00" as *const u8 as *const libc::c_char,
          b"open state for\x00" as *const u8 as *const libc::c_char,
          (*ld).name,
        );
      }
      close(xopen(
        b"state\x00" as *const u8 as *const libc::c_char,
        0o1i32 | 0o4000i32 | 0o1000i32 | 0o100i32,
      ));
      fd = xopen(
        b"state\x00" as *const u8 as *const libc::c_char,
        0i32 | 0o4000i32,
      )
    }
    xmove_fd(fd, 4i32);
    fd = xopen(
      b"newstate\x00" as *const u8 as *const libc::c_char,
      0o1i32 | 0o4000i32 | 0o1000i32 | 0o100i32,
    );
    xmove_fd(fd, 5i32);
    execl(
      (*ptr_to_globals).shell,
      (*ptr_to_globals).shell,
      b"-c\x00" as *const u8 as *const libc::c_char,
      (*ld).processor,
      0 as *mut libc::c_void as *mut libc::c_char,
    );
    bb_perror_msg_and_die(
      b"fatal: can\'t %s processor %s\x00" as *const u8 as *const libc::c_char,
      b"run\x00" as *const u8 as *const libc::c_char,
      (*ld).name,
    );
  }
  (*ld).fnsave[26] = sv_ch;
  (*ld).ppid = pid;
}
unsafe extern "C" fn processorstop(mut ld: *mut logdir) -> libc::c_uint {
  let mut f: [libc::c_char; 28] = [0; 28];
  if (*ld).ppid != 0 {
    sig_unblock(1i32);
    while safe_waitpid((*ld).ppid, &mut (*ptr_to_globals).wstat, 0i32) == -1i32 {
      pause2cannot(
        b"wait for processor\x00" as *const u8 as *const libc::c_char,
        (*ld).name,
      );
    }
    sig_block(1i32);
    (*ld).ppid = 0i32
  }
  if (*ld).fddir == -1i32 {
    return 1i32 as libc::c_uint;
  }
  while fchdir((*ld).fddir) == -1i32 {
    pause2cannot(
      b"change directory, want processor\x00" as *const u8 as *const libc::c_char,
      (*ld).name,
    );
  }
  if ((*ptr_to_globals).wstat & 0xff00i32) >> 8i32 != 0i32 {
    warnx(
      b"processor failed, restart\x00" as *const u8 as *const libc::c_char,
      (*ld).name,
    );
    (*ld).fnsave[26] = 't' as i32 as libc::c_char;
    unlink((*ld).fnsave.as_mut_ptr());
    (*ld).fnsave[26] = 'u' as i32 as libc::c_char;
    processorstart(ld);
    while fchdir((*ptr_to_globals).fdwdir) == -1i32 {
      pause1cannot(b"change to initial working directory\x00" as *const u8 as *const libc::c_char);
    }
    return if !(*ld).processor.is_null() {
      0i32
    } else {
      1i32
    } as libc::c_uint;
  }
  (*ld).fnsave[26] = 't' as i32 as libc::c_char;
  memcpy(
    f.as_mut_ptr() as *mut libc::c_void,
    (*ld).fnsave.as_mut_ptr() as *const libc::c_void,
    26i32 as libc::c_ulong,
  );
  f[26] = 's' as i32 as libc::c_char;
  f[27] = '\u{0}' as i32 as libc::c_char;
  while rename((*ld).fnsave.as_mut_ptr(), f.as_mut_ptr()) == -1i32 {
    pause2cannot(
      b"rename processed\x00" as *const u8 as *const libc::c_char,
      (*ld).name,
    );
  }
  while chmod(f.as_mut_ptr(), 0o744i32 as mode_t) == -1i32 {
    pause2cannot(
      b"set mode of processed\x00" as *const u8 as *const libc::c_char,
      (*ld).name,
    );
  }
  (*ld).fnsave[26] = 'u' as i32 as libc::c_char;
  if unlink((*ld).fnsave.as_mut_ptr()) == -1i32 {
    bb_error_msg(
      b"warning: can\'t unlink: %s/%s\x00" as *const u8 as *const libc::c_char,
      (*ld).name,
      (*ld).fnsave.as_mut_ptr(),
    );
  }
  while rename(
    b"newstate\x00" as *const u8 as *const libc::c_char,
    b"state\x00" as *const u8 as *const libc::c_char,
  ) == -1i32
  {
    pause2cannot(
      b"rename state\x00" as *const u8 as *const libc::c_char,
      (*ld).name,
    );
  }
  if (*ptr_to_globals).verbose != 0 {
    bb_error_msg(
      b"info: processed: %s/%s\x00" as *const u8 as *const libc::c_char,
      (*ld).name,
      f.as_mut_ptr(),
    );
  }
  while fchdir((*ptr_to_globals).fdwdir) == -1i32 {
    pause1cannot(b"change to initial working directory\x00" as *const u8 as *const libc::c_char);
  }
  return 1i32 as libc::c_uint;
}
unsafe extern "C" fn rmoldest(mut ld: *mut logdir) {
  let mut d: *mut DIR = 0 as *mut DIR;
  let mut f: *mut dirent = 0 as *mut dirent;
  let mut oldest: [libc::c_char; 30] = [0; 30];
  let mut n: libc::c_int = 0i32;
  oldest[0] = 'A' as i32 as libc::c_char;
  oldest[27] = 0i32 as libc::c_char;
  oldest[1] = oldest[27];
  loop {
    d = opendir(b".\x00" as *const u8 as *const libc::c_char);
    if !d.is_null() {
      break;
    }
    pause2cannot(
      b"open directory, want rotate\x00" as *const u8 as *const libc::c_char,
      (*ld).name,
    );
  }
  *bb_errno = 0i32;
  loop {
    f = readdir(d);
    if f.is_null() {
      break;
    }
    if (*f).d_name[0] as libc::c_int == '@' as i32
      && strlen((*f).d_name.as_mut_ptr()) == 27i32 as libc::c_ulong
    {
      if (*f).d_name[26] as libc::c_int == 't' as i32 {
        if unlink((*f).d_name.as_mut_ptr()) == -1i32 {
          warn2(
            b"can\'t unlink processor leftover\x00" as *const u8 as *const libc::c_char,
            (*f).d_name.as_mut_ptr(),
          );
        }
      } else {
        n += 1;
        if strcmp((*f).d_name.as_mut_ptr(), oldest.as_mut_ptr()) < 0i32 {
          memcpy(
            oldest.as_mut_ptr() as *mut libc::c_void,
            (*f).d_name.as_mut_ptr() as *const libc::c_void,
            27i32 as libc::c_ulong,
          );
        }
      }
      *bb_errno = 0i32
    }
  }
  if *bb_errno != 0 {
    warn2(
      b"can\'t read directory\x00" as *const u8 as *const libc::c_char,
      (*ld).name,
    );
  }
  closedir(d);
  if (*ld).nmax != 0 && n as libc::c_uint > (*ld).nmax {
    if (*ptr_to_globals).verbose != 0 {
      bb_error_msg(
        b"info: delete: %s/%s\x00" as *const u8 as *const libc::c_char,
        (*ld).name,
        oldest.as_mut_ptr(),
      );
    }
    if *oldest.as_mut_ptr() as libc::c_int == '@' as i32 && unlink(oldest.as_mut_ptr()) == -1i32 {
      warn2(
        b"can\'t unlink oldest logfile\x00" as *const u8 as *const libc::c_char,
        (*ld).name,
      );
    }
  };
}
unsafe extern "C" fn rotate(mut ld: *mut logdir) -> libc::c_uint {
  let mut st: stat = std::mem::zeroed();
  let mut now: libc::c_uint = 0;
  if (*ld).fddir == -1i32 {
    (*ld).rotate_period = 0i32 as libc::c_uint;
    return 0i32 as libc::c_uint;
  }
  if (*ld).ppid != 0 {
    while processorstop(ld) == 0 {}
  }
  while fchdir((*ld).fddir) == -1i32 {
    pause2cannot(
      b"change directory, want rotate\x00" as *const u8 as *const libc::c_char,
      (*ld).name,
    );
  }
  /* create new filename */
  (*ld).fnsave[25] = '.' as i32 as libc::c_char;
  (*ld).fnsave[26] = 's' as i32 as libc::c_char;
  if !(*ld).processor.is_null() {
    (*ld).fnsave[26] = 'u' as i32 as libc::c_char
  }
  (*ld).fnsave[27] = '\u{0}' as i32 as libc::c_char;
  loop {
    fmt_time_bernstein_25((*ld).fnsave.as_mut_ptr());
    *bb_errno = 0i32;
    stat((*ld).fnsave.as_mut_ptr(), &mut st);
    if !(*bb_errno != 2i32) {
      break;
    }
  }
  now = monotonic_sec();
  if (*ld).rotate_period != 0 && now.wrapping_sub((*ld).next_rotate) as libc::c_int > 0i32 {
    (*ld).next_rotate = now.wrapping_add((*ld).rotate_period);
    if (*ptr_to_globals)
      .nearest_rotate
      .wrapping_sub((*ld).next_rotate) as libc::c_int
      > 0i32
    {
      (*ptr_to_globals).nearest_rotate = (*ld).next_rotate
    }
  }
  if (*ld).size > 0i32 as libc::c_uint {
    while fflush((*ld).filecur) != 0 || fsync((*ld).fdcur) == -1i32 {
      pause2cannot(
        b"fsync current logfile\x00" as *const u8 as *const libc::c_char,
        (*ld).name,
      );
    }
    while fchmod((*ld).fdcur, 0o744i32 as mode_t) == -1i32 {
      pause2cannot(
        b"set mode of current\x00" as *const u8 as *const libc::c_char,
        (*ld).name,
      );
    }
    // //close(ld->fdcur);
    fclose((*ld).filecur); /* very unlikely */
    if (*ptr_to_globals).verbose != 0 {
      bb_error_msg(
        b"info: rename: %s/current %s %u\x00" as *const u8 as *const libc::c_char,
        (*ld).name,
        (*ld).fnsave.as_mut_ptr(),
        (*ld).size,
      );
    }
    while rename(
      b"current\x00" as *const u8 as *const libc::c_char,
      (*ld).fnsave.as_mut_ptr(),
    ) == -1i32
    {
      pause2cannot(
        b"rename current\x00" as *const u8 as *const libc::c_char,
        (*ld).name,
      );
    }
    loop {
      (*ld).fdcur = open(
        b"current\x00" as *const u8 as *const libc::c_char,
        0o1i32 | 0o4000i32 | 0o2000i32 | 0o100i32,
        0o600i32,
      );
      if !((*ld).fdcur == -1i32) {
        break;
      }
      pause2cannot(
        b"create new current\x00" as *const u8 as *const libc::c_char,
        (*ld).name,
      );
    }
    loop {
      (*ld).filecur = fdopen((*ld).fdcur, b"a\x00" as *const u8 as *const libc::c_char);
      if !(*ld).filecur.is_null() {
        break;
      }
      // //
      pause2cannot(
        b"create new current\x00" as *const u8 as *const libc::c_char,
        (*ld).name,
      ); // //
    }
    setvbuf(
      (*ld).filecur,
      0 as *mut libc::c_char,
      0i32,
      (*ptr_to_globals).linelen as size_t,
    );
    close_on_exec_on((*ld).fdcur);
    (*ld).size = 0i32 as libc::c_uint;
    while fchmod((*ld).fdcur, 0o644i32 as mode_t) == -1i32 {
      pause2cannot(
        b"set mode of current\x00" as *const u8 as *const libc::c_char,
        (*ld).name,
      );
    }
    rmoldest(ld);
    processorstart(ld);
  }
  while fchdir((*ptr_to_globals).fdwdir) == -1i32 {
    pause1cannot(b"change to initial working directory\x00" as *const u8 as *const libc::c_char);
  }
  return 1i32 as libc::c_uint;
}
unsafe extern "C" fn buffer_pwrite(
  mut n: libc::c_int,
  mut s: *mut libc::c_char,
  mut len: libc::c_uint,
) -> libc::c_int {
  let mut i: libc::c_int = 0;
  let mut ld: *mut logdir = &mut *(*ptr_to_globals).dir.offset(n as isize) as *mut logdir;
  if (*ld).sizemax != 0 {
    if (*ld).size >= (*ld).sizemax {
      rotate(ld);
    }
    if len > (*ld).sizemax.wrapping_sub((*ld).size) {
      len = (*ld).sizemax.wrapping_sub((*ld).size)
    }
  }
  loop {
    // //i = full_write(ld->fdcur, s, len);
    // //if (i != -1) break;
    i = fwrite(
      s as *const libc::c_void,
      1i32 as size_t,
      len as size_t,
      (*ld).filecur,
    ) as libc::c_int; /* impossible */
    if i as libc::c_uint == len {
      break;
    }
    if *bb_errno == 28i32 && (*ld).nmin < (*ld).nmax {
      let mut d: *mut DIR = 0 as *mut DIR;
      let mut f: *mut dirent = 0 as *mut dirent;
      let mut oldest: [libc::c_char; 30] = [0; 30];
      let mut j: libc::c_int = 0i32;
      while fchdir((*ld).fddir) == -1i32 {
        pause2cannot(
          b"change directory, want remove old logfile\x00" as *const u8 as *const libc::c_char,
          (*ld).name,
        );
      }
      oldest[0] = 'A' as i32 as libc::c_char;
      oldest[27] = '\u{0}' as i32 as libc::c_char;
      oldest[1] = oldest[27];
      loop {
        d = opendir(b".\x00" as *const u8 as *const libc::c_char);
        if !d.is_null() {
          break;
        }
        pause2cannot(
          b"open directory, want remove old logfile\x00" as *const u8 as *const libc::c_char,
          (*ld).name,
        );
      }
      *bb_errno = 0i32;
      loop {
        f = readdir(d);
        if f.is_null() {
          break;
        }
        if (*f).d_name[0] as libc::c_int == '@' as i32
          && strlen((*f).d_name.as_mut_ptr()) == 27i32 as libc::c_ulong
        {
          j += 1;
          if strcmp((*f).d_name.as_mut_ptr(), oldest.as_mut_ptr()) < 0i32 {
            memcpy(
              oldest.as_mut_ptr() as *mut libc::c_void,
              (*f).d_name.as_mut_ptr() as *const libc::c_void,
              27i32 as libc::c_ulong,
            );
          }
        }
      }
      if *bb_errno != 0 {
        warn2(
          b"can\'t read directory, want remove old logfile\x00" as *const u8 as *const libc::c_char,
          (*ld).name,
        );
      }
      closedir(d);
      *bb_errno = 28i32;
      if j as libc::c_uint > (*ld).nmin {
        if *oldest.as_mut_ptr() as libc::c_int == '@' as i32 {
          bb_error_msg(
            b"warning: out of disk space, delete: %s/%s\x00" as *const u8 as *const libc::c_char,
            (*ld).name,
            oldest.as_mut_ptr(),
          );
          *bb_errno = 0i32;
          if unlink(oldest.as_mut_ptr()) == -1i32 {
            warn2(
              b"can\'t unlink oldest logfile\x00" as *const u8 as *const libc::c_char,
              (*ld).name,
            );
            *bb_errno = 28i32
          }
          while fchdir((*ptr_to_globals).fdwdir) == -1i32 {
            pause1cannot(
              b"change to initial working directory\x00" as *const u8 as *const libc::c_char,
            );
          }
        }
      }
    }
    if *bb_errno != 0 {
      pause2cannot(
        b"write to current\x00" as *const u8 as *const libc::c_char,
        (*ld).name,
      );
    }
  }
  (*ld).size = (*ld).size.wrapping_add(i as libc::c_uint);
  if (*ld).sizemax != 0 {
    if *s.offset((i - 1i32) as isize) as libc::c_int == '\n' as i32 {
      if (*ld).size
        >= (*ld)
          .sizemax
          .wrapping_sub((*ptr_to_globals).linemax as libc::c_uint)
      {
        rotate(ld);
      }
    }
  }
  return i;
}
unsafe extern "C" fn logdir_close(mut ld: *mut logdir) {
  if (*ld).fddir == -1i32 {
    return;
  }
  if (*ptr_to_globals).verbose != 0 {
    bb_error_msg(
      b"info: close: %s\x00" as *const u8 as *const libc::c_char,
      (*ld).name,
    );
  }
  close((*ld).fddir);
  (*ld).fddir = -1i32;
  if (*ld).fdcur == -1i32 {
    return;
  }
  while fflush((*ld).filecur) != 0 || fsync((*ld).fdcur) == -1i32 {
    pause2cannot(
      b"fsync current logfile\x00" as *const u8 as *const libc::c_char,
      (*ld).name,
    );
  }
  while fchmod((*ld).fdcur, 0o744i32 as mode_t) == -1i32 {
    pause2cannot(
      b"set mode of current\x00" as *const u8 as *const libc::c_char,
      (*ld).name,
    );
  }
  // //close(ld->fdcur);
  fclose((*ld).filecur); /* impossible */
  (*ld).fdcur = -1i32;
  if (*ld).fdlock == -1i32 {
    return;
  }
  close((*ld).fdlock);
  (*ld).fdlock = -1i32;
  free((*ld).processor as *mut libc::c_void);
  (*ld).processor = 0 as *mut libc::c_char;
}
#[inline(never)]
unsafe extern "C" fn logdir_open(
  mut ld: *mut logdir,
  mut fn_0: *const libc::c_char,
) -> libc::c_uint {
  let mut buf: [libc::c_char; 128] = [0; 128];
  let mut now: libc::c_uint = 0;
  let mut new: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut np: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut i: libc::c_int = 0;
  let mut st: stat = std::mem::zeroed();
  now = monotonic_sec();
  (*ld).fddir = open(fn_0, 0i32 | 0o4000i32);
  if (*ld).fddir == -1i32 {
    warn2(
      b"can\'t open log directory\x00" as *const u8 as *const libc::c_char,
      fn_0 as *mut libc::c_char,
    );
    return 0i32 as libc::c_uint;
  }
  close_on_exec_on((*ld).fddir);
  if fchdir((*ld).fddir) == -1i32 {
    logdir_close(ld);
    warn2(
      b"can\'t change directory\x00" as *const u8 as *const libc::c_char,
      fn_0 as *mut libc::c_char,
    );
    return 0i32 as libc::c_uint;
  }
  (*ld).fdlock = open(
    b"lock\x00" as *const u8 as *const libc::c_char,
    0o1i32 | 0o4000i32 | 0o2000i32 | 0o100i32,
    0o600i32,
  );
  if (*ld).fdlock == -1i32 || flock((*ld).fdlock, 2i32 | 4i32) == -1i32 {
    logdir_close(ld);
    warn2(
      b"can\'t lock directory\x00" as *const u8 as *const libc::c_char,
      fn_0 as *mut libc::c_char,
    );
    while fchdir((*ptr_to_globals).fdwdir) == -1i32 {
      pause1cannot(b"change to initial working directory\x00" as *const u8 as *const libc::c_char);
    }
    return 0i32 as libc::c_uint;
  }
  close_on_exec_on((*ld).fdlock);
  (*ld).size = 0i32 as libc::c_uint;
  (*ld).sizemax = 1000000i32 as libc::c_uint;
  (*ld).nmin = 10i32 as libc::c_uint;
  (*ld).nmax = (*ld).nmin;
  (*ld).rotate_period = 0i32 as libc::c_uint;
  (*ld).name = fn_0 as *mut libc::c_char;
  (*ld).ppid = 0i32;
  (*ld).match_0 = '+' as i32 as libc::c_char;
  free((*ld).inst as *mut libc::c_void);
  (*ld).inst = 0 as *mut libc::c_char;
  free((*ld).processor as *mut libc::c_void);
  (*ld).processor = 0 as *mut libc::c_char;
  /* read config */
  i = open_read_close(
    b"config\x00" as *const u8 as *const libc::c_char,
    buf.as_mut_ptr() as *mut libc::c_void,
    (::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong)
      .wrapping_sub(1i32 as libc::c_ulong),
  ) as libc::c_int;
  if i < 0i32 && *bb_errno != 2i32 {
    bb_perror_msg(
      b"warning: %s/config\x00" as *const u8 as *const libc::c_char,
      (*ld).name,
    );
  }
  if i > 0i32 {
    buf[i as usize] = '\u{0}' as i32 as libc::c_char;
    if (*ptr_to_globals).verbose != 0 {
      bb_error_msg(
        b"info: read: %s/config\x00" as *const u8 as *const libc::c_char,
        (*ld).name,
      );
    }
    s = buf.as_mut_ptr();
    while !s.is_null() {
      np = strchr(s, '\n' as i32);
      if !np.is_null() {
        let fresh12 = np;
        np = np.offset(1);
        *fresh12 = '\u{0}' as i32 as libc::c_char
      }
      match *s.offset(0) as libc::c_int {
        43 | 45 | 101 | 69 => {
          /* Filtering requires one-line buffering,
           * resetting the "find newline" function
           * accordingly */
          (*ptr_to_globals).memRchr = Some(
            memchr
              as unsafe extern "C" fn(
                _: *const libc::c_void,
                _: libc::c_int,
                _: libc::c_ulong,
              ) -> *mut libc::c_void,
          );
          loop
          /* Add '\n'-terminated line to ld->inst */
          {
            let mut l: libc::c_int = asprintf(
              &mut new as *mut *mut libc::c_char,
              b"%s%s\n\x00" as *const u8 as *const libc::c_char,
              if !(*ld).inst.is_null() {
                (*ld).inst
              } else {
                b"\x00" as *const u8 as *const libc::c_char
              },
              s,
            );
            if l >= 0i32 && !new.is_null() {
              break;
            }
            pause_nomem();
          }
          free((*ld).inst as *mut libc::c_void);
          (*ld).inst = new
        }
        115 => (*ld).sizemax = xatou_sfx(&mut *s.offset(1), bkm_suffixes.as_ptr().offset(1)),
        110 => (*ld).nmax = xatoi_positive(&mut *s.offset(1)) as libc::c_uint,
        78 => (*ld).nmin = xatoi_positive(&mut *s.offset(1)) as libc::c_uint,
        116 => {
          static mut mh_suffixes: [suffix_mult; 3] = [
            {
              let mut init = suffix_mult {
                suffix: [109, 0, 0, 0],
                mult: 60i32 as libc::c_uint,
              };
              init
            },
            {
              let mut init = suffix_mult {
                suffix: [104, 0, 0, 0],
                mult: (60i32 * 60i32) as libc::c_uint,
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
          (*ld).rotate_period = xatou_sfx(&mut *s.offset(1), mh_suffixes.as_ptr());
          if (*ld).rotate_period != 0 {
            (*ld).next_rotate = now.wrapping_add((*ld).rotate_period);
            if (*ptr_to_globals).tmaxflag == 0
              || (*ptr_to_globals)
                .nearest_rotate
                .wrapping_sub((*ld).next_rotate) as libc::c_int
                > 0i32
            {
              (*ptr_to_globals).nearest_rotate = (*ld).next_rotate
            }
            (*ptr_to_globals).tmaxflag = 1i32 as smallint
          }
        }
        33 => {
          if *s.offset(1) != 0 {
            free((*ld).processor as *mut libc::c_void);
            (*ld).processor = wstrdup(&mut *s.offset(1))
          }
        }
        _ => {}
      }
      s = np
    }
    /* Convert "aa\nbb\ncc\n\0" to "aa\0bb\0cc\0\0" */
    s = (*ld).inst;
    while !s.is_null() {
      np = strchr(s, '\n' as i32);
      if !np.is_null() {
        let fresh13 = np;
        np = np.offset(1);
        *fresh13 = '\u{0}' as i32 as libc::c_char
      }
      s = np
    }
  }
  /* open current */
  i = stat(b"current\x00" as *const u8 as *const libc::c_char, &mut st);
  if i != -1i32 {
    if st.st_size != 0 && st.st_mode & 0o100i32 as libc::c_uint == 0 {
      (*ld).fnsave[25] = '.' as i32 as libc::c_char;
      (*ld).fnsave[26] = 'u' as i32 as libc::c_char;
      (*ld).fnsave[27] = '\u{0}' as i32 as libc::c_char;
      loop {
        fmt_time_bernstein_25((*ld).fnsave.as_mut_ptr());
        *bb_errno = 0i32;
        stat((*ld).fnsave.as_mut_ptr(), &mut st);
        if !(*bb_errno != 2i32) {
          break;
        }
      }
      while rename(
        b"current\x00" as *const u8 as *const libc::c_char,
        (*ld).fnsave.as_mut_ptr(),
      ) == -1i32
      {
        pause2cannot(
          b"rename current\x00" as *const u8 as *const libc::c_char,
          (*ld).name,
        );
      }
      rmoldest(ld);
      i = -1i32
    } else {
      /* st.st_size can be not just bigger, but WIDER!
       * This code is safe: if st.st_size > 4GB, we select
       * ld->sizemax (because it's "unsigned") */
      (*ld).size = if st.st_size > (*ld).sizemax as libc::c_long {
        (*ld).sizemax as libc::c_long
      } else {
        st.st_size
      } as libc::c_uint
    }
  } else if *bb_errno != 2i32 {
    logdir_close(ld); // //
    warn2(
      b"can\'t stat current\x00" as *const u8 as *const libc::c_char,
      (*ld).name,
    ); // //
    while fchdir((*ptr_to_globals).fdwdir) == -1i32 {
      pause1cannot(b"change to initial working directory\x00" as *const u8 as *const libc::c_char);
    }
    return 0i32 as libc::c_uint;
  }
  loop {
    (*ld).fdcur = open(
      b"current\x00" as *const u8 as *const libc::c_char,
      0o1i32 | 0o4000i32 | 0o2000i32 | 0o100i32,
      0o600i32,
    );
    if !((*ld).fdcur == -1i32) {
      break;
    }
    pause2cannot(
      b"open current\x00" as *const u8 as *const libc::c_char,
      (*ld).name,
    );
  }
  loop {
    (*ld).filecur = fdopen((*ld).fdcur, b"a\x00" as *const u8 as *const libc::c_char);
    if !(*ld).filecur.is_null() {
      break;
    }
    pause2cannot(
      b"open current\x00" as *const u8 as *const libc::c_char,
      (*ld).name,
    );
  }
  setvbuf(
    (*ld).filecur,
    0 as *mut libc::c_char,
    0i32,
    (*ptr_to_globals).linelen as size_t,
  );
  close_on_exec_on((*ld).fdcur);
  while fchmod((*ld).fdcur, 0o644i32 as mode_t) == -1i32 {
    pause2cannot(
      b"set mode of current\x00" as *const u8 as *const libc::c_char,
      (*ld).name,
    );
  }
  if (*ptr_to_globals).verbose != 0 {
    if i == 0i32 {
      bb_error_msg(
        b"info: append: %s/current\x00" as *const u8 as *const libc::c_char,
        (*ld).name,
      );
    } else {
      bb_error_msg(
        b"info: new: %s/current\x00" as *const u8 as *const libc::c_char,
        (*ld).name,
      );
    }
  }
  while fchdir((*ptr_to_globals).fdwdir) == -1i32 {
    pause1cannot(b"change to initial working directory\x00" as *const u8 as *const libc::c_char);
  }
  return 1i32 as libc::c_uint;
}
unsafe extern "C" fn logdirs_reopen() {
  let mut l: libc::c_int = 0;
  let mut ok: libc::c_int = 0i32;
  (*ptr_to_globals).tmaxflag = 0i32 as smallint;
  l = 0i32;
  while (l as libc::c_uint) < (*ptr_to_globals).dirn {
    logdir_close(&mut *(*ptr_to_globals).dir.offset(l as isize));
    if logdir_open(
      &mut *(*ptr_to_globals).dir.offset(l as isize),
      *(*ptr_to_globals).fndir.offset(l as isize),
    ) != 0
    {
      ok = 1i32
    }
    l += 1
  }
  if ok == 0 {
    fatalx(b"no functional log directories\x00" as *const u8 as *const libc::c_char);
  };
}
/* Will look good in libbb one day */
unsafe extern "C" fn ndelay_read(
  mut fd: libc::c_int,
  mut buf: *mut libc::c_void,
  mut count: size_t,
) -> ssize_t {
  if (*ptr_to_globals).fl_flag_0 & 0o4000i32 == 0 {
    fcntl(fd, 4i32, (*ptr_to_globals).fl_flag_0 | 0o4000i32);
  }
  count = safe_read(fd, buf, count) as size_t;
  if (*ptr_to_globals).fl_flag_0 & 0o4000i32 == 0 {
    fcntl(fd, 4i32, (*ptr_to_globals).fl_flag_0);
  }
  return count as ssize_t;
}
/* Used for reading stdin */
unsafe extern "C" fn buffer_pread(mut s: *mut libc::c_char, mut len: libc::c_uint) -> libc::c_int {
  let mut now: libc::c_uint = 0;
  let mut input: pollfd = pollfd {
    fd: 0,
    events: 0,
    revents: 0,
  };
  let mut i: libc::c_int = 0;
  input.fd = 0i32;
  input.events = 0x1i32 as libc::c_short;
  loop {
    if (*ptr_to_globals).rotateasap != 0 {
      i = 0i32;
      while (i as libc::c_uint) < (*ptr_to_globals).dirn {
        rotate((*ptr_to_globals).dir.offset(i as isize));
        i += 1
      }
      (*ptr_to_globals).rotateasap = 0i32 as smallint
    }
    if (*ptr_to_globals).exitasap != 0 {
      if (*ptr_to_globals).linecomplete != 0 {
        return 0i32;
      }
      len = 1i32 as libc::c_uint
    }
    if (*ptr_to_globals).reopenasap != 0 {
      logdirs_reopen();
      (*ptr_to_globals).reopenasap = 0i32 as smallint
    }
    now = monotonic_sec();
    (*ptr_to_globals).nearest_rotate = now.wrapping_add((45i32 * 60i32 + 45i32) as libc::c_uint);
    i = 0i32;
    while (i as libc::c_uint) < (*ptr_to_globals).dirn {
      if (*(*ptr_to_globals).dir.offset(i as isize)).rotate_period != 0 {
        if now.wrapping_sub((*(*ptr_to_globals).dir.offset(i as isize)).next_rotate) as libc::c_int
          > 0i32
        {
          rotate((*ptr_to_globals).dir.offset(i as isize));
        }
        if (*ptr_to_globals)
          .nearest_rotate
          .wrapping_sub((*(*ptr_to_globals).dir.offset(i as isize)).next_rotate)
          as libc::c_int
          > 0i32
        {
          (*ptr_to_globals).nearest_rotate = (*(*ptr_to_globals).dir.offset(i as isize)).next_rotate
        }
      }
      i += 1
    }
    sigprocmask(
      1i32,
      &mut (*ptr_to_globals).blocked_sigset,
      0 as *mut sigset_t,
    );
    i = (*ptr_to_globals).nearest_rotate.wrapping_sub(now) as libc::c_int;
    if i > 1000000i32 {
      i = 1000000i32
    }
    if i <= 0i32 {
      i = 1i32
    }
    poll(&mut input, 1i32 as nfds_t, i * 1000i32);
    sigprocmask(
      0i32,
      &mut (*ptr_to_globals).blocked_sigset,
      0 as *mut sigset_t,
    );
    i = ndelay_read(0i32, s as *mut libc::c_void, len as size_t) as libc::c_int;
    if i >= 0i32 {
      break;
    }
    if !(*bb_errno == 4i32) {
      if *bb_errno != 11i32 {
        warn(b"can\'t read standard input\x00" as *const u8 as *const libc::c_char);
        break;
      }
    }
    if !((*ptr_to_globals).exitasap == 0) {
      break;
    }
    /* else: EAGAIN - normal, repeat silently */
  }
  if i > 0i32 {
    let mut cnt: libc::c_int = 0;
    (*ptr_to_globals).linecomplete =
      (*s.offset((i - 1i32) as isize) as libc::c_int == '\n' as i32) as libc::c_int as smallint;
    if (*ptr_to_globals).repl == 0 {
      return i;
    }
    cnt = i;
    loop {
      cnt -= 1;
      if !(cnt >= 0i32) {
        break;
      }
      let mut ch: libc::c_char = *s;
      if ch as libc::c_int != '\n' as i32 {
        if (ch as libc::c_int) < 32i32 || ch as libc::c_int > 126i32 {
          *s = (*ptr_to_globals).repl
        } else {
          let mut j: libc::c_int = 0;
          j = 0i32;
          while *(*ptr_to_globals).replace.offset(j as isize) != 0 {
            if ch as libc::c_int == *(*ptr_to_globals).replace.offset(j as isize) as libc::c_int {
              *s = (*ptr_to_globals).repl;
              break;
            } else {
              j += 1
            }
          }
        }
      }
      s = s.offset(1)
    }
  }
  return i;
}
unsafe extern "C" fn sig_term_handler(mut _sig_no: libc::c_int) {
  if (*ptr_to_globals).verbose != 0 {
    bb_error_msg(
      b"info: sig%s received\x00" as *const u8 as *const libc::c_char,
      b"term\x00" as *const u8 as *const libc::c_char,
    );
  }
  (*ptr_to_globals).exitasap = 1i32 as smallint;
}
unsafe extern "C" fn sig_child_handler(mut _sig_no: libc::c_int) {
  let mut pid: pid_t = 0;
  let mut l: libc::c_int = 0;
  if (*ptr_to_globals).verbose != 0 {
    bb_error_msg(
      b"info: sig%s received\x00" as *const u8 as *const libc::c_char,
      b"child\x00" as *const u8 as *const libc::c_char,
    );
  }
  loop {
    pid = wait_any_nohang(&mut (*ptr_to_globals).wstat);
    if !(pid > 0i32) {
      break;
    }
    l = 0i32;
    while (l as libc::c_uint) < (*ptr_to_globals).dirn {
      if (*(*ptr_to_globals).dir.offset(l as isize)).ppid == pid {
        (*(*ptr_to_globals).dir.offset(l as isize)).ppid = 0i32;
        processorstop(&mut *(*ptr_to_globals).dir.offset(l as isize));
        break;
      } else {
        l += 1
      }
    }
  }
}
unsafe extern "C" fn sig_alarm_handler(mut _sig_no: libc::c_int) {
  if (*ptr_to_globals).verbose != 0 {
    bb_error_msg(
      b"info: sig%s received\x00" as *const u8 as *const libc::c_char,
      b"alarm\x00" as *const u8 as *const libc::c_char,
    );
  }
  (*ptr_to_globals).rotateasap = 1i32 as smallint;
}
unsafe extern "C" fn sig_hangup_handler(mut _sig_no: libc::c_int) {
  if (*ptr_to_globals).verbose != 0 {
    bb_error_msg(
      b"info: sig%s received\x00" as *const u8 as *const libc::c_char,
      b"hangup\x00" as *const u8 as *const libc::c_char,
    );
  }
  (*ptr_to_globals).reopenasap = 1i32 as smallint;
}
unsafe extern "C" fn logmatch(
  mut ld: *mut logdir,
  mut lineptr: *mut libc::c_char,
  mut lineptr_len: libc::c_int,
) {
  let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
  (*ld).match_0 = '+' as i32 as libc::c_char;
  (*ld).matcherr = 'E' as i32 as libc::c_char;
  s = (*ld).inst;
  while !s.is_null() && *s.offset(0) as libc::c_int != 0 {
    match *s.offset(0) as libc::c_int {
      43 | 45 => {
        if pmatch(s.offset(1), lineptr, lineptr_len as libc::c_uint) != 0 {
          (*ld).match_0 = *s.offset(0)
        }
      }
      101 | 69 => {
        if pmatch(s.offset(1), lineptr, lineptr_len as libc::c_uint) != 0 {
          (*ld).matcherr = *s.offset(0)
        }
      }
      _ => {}
    }
    s = s.offset(strlen(s).wrapping_add(1i32 as libc::c_ulong) as isize)
  }
}
#[no_mangle]
pub unsafe extern "C" fn svlogd_main(
  mut argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut r: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut l: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut b: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut stdin_cnt: ssize_t = 0i32 as ssize_t;
  let mut i: libc::c_int = 0;
  let mut opt: libc::c_uint = 0;
  let mut timestamp: libc::c_uint = 0i32 as libc::c_uint;
  let ref mut fresh14 =
    *(not_const_pp(&ptr_to_globals as *const *mut globals as *const libc::c_void)
      as *mut *mut globals);
  *fresh14 = xzalloc(::std::mem::size_of::<globals>() as libc::c_ulong) as *mut globals;
  asm!("" : : : "memory" : "volatile");
  (*ptr_to_globals).linemax = 1000i32;
  (*ptr_to_globals).linecomplete = 1i32 as smallint;
  (*ptr_to_globals).replace = b"\x00" as *const u8 as *const libc::c_char;
  opt = getopt32(
    argv,
    b"^r:R:l:b:tv\x00tt:vv\x00" as *const u8 as *const libc::c_char,
    &mut r as *mut *mut libc::c_char,
    &mut (*ptr_to_globals).replace as *mut *const libc::c_char,
    &mut l as *mut *mut libc::c_char,
    &mut b as *mut *mut libc::c_char,
    &mut timestamp as *mut libc::c_uint,
    &mut (*ptr_to_globals).verbose as *mut libc::c_uint,
  );
  if opt & 1i32 as libc::c_uint != 0 {
    // -r
    (*ptr_to_globals).repl = *r.offset(0); // -R
    if (*ptr_to_globals).repl == 0 || *r.offset(1) as libc::c_int != 0 {
      bb_show_usage();
    }
  }
  if opt & 2i32 as libc::c_uint != 0 {
    if (*ptr_to_globals).repl == 0 {
      (*ptr_to_globals).repl = '_' as i32 as libc::c_char
    }
  }
  if opt & 4i32 as libc::c_uint != 0 {
    // -l
    (*ptr_to_globals).linemax = xatou_range(
      l,
      0i32 as libc::c_uint,
      (COMMON_BUFSIZE as libc::c_int - 26i32) as libc::c_uint,
    ) as libc::c_int;
    if (*ptr_to_globals).linemax == 0i32 {
      (*ptr_to_globals).linemax = COMMON_BUFSIZE as libc::c_int - 26i32
    }
    if (*ptr_to_globals).linemax < 256i32 {
      (*ptr_to_globals).linemax = 256i32
    }
  }
  // //if (opt & 8) { // -b
  // //	buflen = xatoi_positive(b);
  // //	if (buflen == 0) buflen = 1024;
  // //}
  //if (opt & 0x10) timestamp++; // -t
  //if (opt & 0x20) verbose++; // -v
  //if (timestamp > 2) timestamp = 2;
  argv = argv.offset(optind as isize);
  argc -= optind;
  (*ptr_to_globals).dirn = argc as libc::c_uint;
  if (*ptr_to_globals).dirn <= 0i32 as libc::c_uint {
    bb_show_usage();
  }
  // //if (buflen <= linemax) bb_show_usage();
  (*ptr_to_globals).fdwdir = xopen(
    b".\x00" as *const u8 as *const libc::c_char,
    0i32 | 0o4000i32,
  );
  close_on_exec_on((*ptr_to_globals).fdwdir);
  (*ptr_to_globals).dir = xzalloc(
    ((*ptr_to_globals).dirn as libc::c_ulong)
      .wrapping_mul(::std::mem::size_of::<logdir>() as libc::c_ulong),
  ) as *mut logdir;
  i = 0i32;
  while (i as libc::c_uint) < (*ptr_to_globals).dirn {
    (*(*ptr_to_globals).dir.offset(i as isize)).fddir = -1i32;
    (*(*ptr_to_globals).dir.offset(i as isize)).fdcur = -1i32;
    i += 1
    // //dir[i].btmp = xmalloc(buflen);
    /*dir[i].ppid = 0;*/
  }
  /* line = xmalloc(linemax + (timestamp ? 26 : 0)); */
  (*ptr_to_globals).fndir = argv;
  /* We cannot set NONBLOCK on fd #0 permanently - this setting
   * _isn't_ per-process! It is shared among all other processes
   * with the same stdin */
  (*ptr_to_globals).fl_flag_0 = fcntl(0i32, 3i32);
  sigemptyset(&mut (*ptr_to_globals).blocked_sigset);
  sigaddset(&mut (*ptr_to_globals).blocked_sigset, 15i32);
  sigaddset(&mut (*ptr_to_globals).blocked_sigset, 17i32);
  sigaddset(&mut (*ptr_to_globals).blocked_sigset, 14i32);
  sigaddset(&mut (*ptr_to_globals).blocked_sigset, 1i32);
  sigprocmask(
    0i32,
    &mut (*ptr_to_globals).blocked_sigset,
    0 as *mut sigset_t,
  );
  bb_signals_recursive_norestart(
    1i32 << 15i32,
    Some(sig_term_handler as unsafe extern "C" fn(_: libc::c_int) -> ()),
  );
  bb_signals_recursive_norestart(
    1i32 << 17i32,
    Some(sig_child_handler as unsafe extern "C" fn(_: libc::c_int) -> ()),
  );
  bb_signals_recursive_norestart(
    1i32 << 14i32,
    Some(sig_alarm_handler as unsafe extern "C" fn(_: libc::c_int) -> ()),
  );
  bb_signals_recursive_norestart(
    1i32 << 1i32,
    Some(sig_hangup_handler as unsafe extern "C" fn(_: libc::c_int) -> ()),
  );
  /* Without timestamps, we don't have to print each line
   * separately, so we can look for _last_ newline, not first,
   * thus batching writes. If filtering is enabled in config,
   * logdirs_reopen resets it to memchr.
   */
  (*ptr_to_globals).memRchr = if timestamp != 0 {
    Some(
      memchr
        as unsafe extern "C" fn(
          _: *const libc::c_void,
          _: libc::c_int,
          _: libc::c_ulong,
        ) -> *mut libc::c_void,
    )
  } else {
    Some(
      memrchr
        as unsafe extern "C" fn(
          _: *const libc::c_void,
          _: libc::c_int,
          _: size_t,
        ) -> *mut libc::c_void,
    )
  };
  logdirs_reopen();
  setvbuf(
    stderr,
    0 as *mut libc::c_char,
    0i32,
    (*ptr_to_globals).linelen as size_t,
  );
  let mut current_block_116: u64;
  loop
  /* Each iteration processes one or more lines */
  {
    let mut stamp: [libc::c_char; 30] = [0; 30];
    let mut lineptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut printptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut np: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut printlen: libc::c_int = 0;
    let mut ch: libc::c_char = 0;
    lineptr = bb_common_bufsiz1.as_mut_ptr();
    if timestamp != 0 {
      lineptr = lineptr.offset(26)
    }
    /* lineptr[0..linemax-1] - buffer for stdin */
    /* (possibly has some unprocessed data from prev loop) */
    /* Refill the buffer if needed */
    np = (*ptr_to_globals)
      .memRchr
      .expect("non-null function pointer")(
      lineptr as *const libc::c_void,
      '\n' as i32,
      stdin_cnt as size_t,
    ) as *mut libc::c_char; /* avail. bytes at tail */
    if np.is_null() && (*ptr_to_globals).exitasap == 0 {
      i = ((*ptr_to_globals).linemax as libc::c_long - stdin_cnt) as libc::c_int;
      if i >= 128i32 {
        i = buffer_pread(lineptr.offset(stdin_cnt as isize), i as libc::c_uint);
        if i <= 0i32 {
          /* EOF or error on stdin */
          (*ptr_to_globals).exitasap = 1i32 as smallint
        } else {
          np = (*ptr_to_globals)
            .memRchr
            .expect("non-null function pointer")(
            lineptr.offset(stdin_cnt as isize) as *const libc::c_void,
            '\n' as i32,
            i as size_t,
          ) as *mut libc::c_char;
          stdin_cnt += i as libc::c_long
        }
      }
    }
    if stdin_cnt <= 0i32 as libc::c_long && (*ptr_to_globals).exitasap as libc::c_int != 0 {
      break;
    }
    /* Search for '\n' (in fact, np already holds the result) */
    (*ptr_to_globals).linelen = stdin_cnt as libc::c_int;
    if !np.is_null() {
      current_block_116 = 368077705793071303;
    } else {
      current_block_116 = 12961834331865314435;
    }
    loop {
      match current_block_116 {
        368077705793071303 => {
          /* NB: starting from here lineptr may point
           * farther out into line[] */
          (*ptr_to_globals).linelen = (np.wrapping_offset_from(lineptr) as libc::c_long
            + 1i32 as libc::c_long) as libc::c_int;
          current_block_116 = 12961834331865314435;
        }
        _ => {
          /* linelen == no of chars incl. '\n' (or == stdin_cnt) */
          ch = *lineptr.offset(((*ptr_to_globals).linelen - 1i32) as isize);
          /* Biggest performance hit was coming from the fact
           * that we did not buffer writes. We were reading many lines
           * in one read() above, but wrote one line per write().
           * We are using stdio to fix that */
          /* write out lineptr[0..linelen-1] to each log destination
           * (or lineptr[-26..linelen-1] if timestamping) */
          printlen = (*ptr_to_globals).linelen;
          printptr = lineptr;
          if timestamp != 0 {
            if timestamp == 1i32 as libc::c_uint {
              fmt_time_bernstein_25(stamp.as_mut_ptr());
            } else {
              /* 2+: */
              fmt_time_human_30nul(
                stamp.as_mut_ptr(),
                if timestamp == 2i32 as libc::c_uint {
                  '_' as i32
                } else {
                  'T' as i32
                } as libc::c_char,
              );
            }
            printlen += 26i32;
            printptr = printptr.offset(-26);
            memcpy(
              printptr as *mut libc::c_void,
              stamp.as_mut_ptr() as *const libc::c_void,
              25i32 as libc::c_ulong,
            );
            *printptr.offset(25) = ' ' as i32 as libc::c_char
          }
          i = 0i32;
          while (i as libc::c_uint) < (*ptr_to_globals).dirn {
            let mut ld: *mut logdir = &mut *(*ptr_to_globals).dir.offset(i as isize) as *mut logdir;
            if !((*ld).fddir == -1i32) {
              if !(*ld).inst.is_null() {
                logmatch(ld, lineptr, (*ptr_to_globals).linelen);
              }
              if (*ld).matcherr as libc::c_int == 'e' as i32 {
                /* runit-1.8.0 compat: if timestamping, do it on stderr too */
                // //full_write(STDERR_FILENO, printptr, printlen);
                fwrite(
                  printptr as *const libc::c_void,
                  1i32 as size_t,
                  printlen as size_t,
                  stderr,
                );
              }
              if !((*ld).match_0 as libc::c_int != '+' as i32) {
                buffer_pwrite(i, printptr, printlen as libc::c_uint);
              }
            }
            i += 1
          }
          /* If we didn't see '\n' (long input line), */
          /* read/write repeatedly until we see it */
          while ch as libc::c_int != '\n' as i32 {
            /* lineptr is emptied now, safe to use as buffer */
            stdin_cnt = if (*ptr_to_globals).exitasap as libc::c_int != 0 {
              -1i32
            } else {
              buffer_pread(lineptr, (*ptr_to_globals).linemax as libc::c_uint)
            } as ssize_t;
            if stdin_cnt <= 0i32 as libc::c_long {
              /* EOF or error on stdin */
              (*ptr_to_globals).exitasap = 1i32 as smallint;
              ch = '\n' as i32 as libc::c_char;
              *lineptr.offset(0) = ch;
              (*ptr_to_globals).linelen = 1i32;
              stdin_cnt = 1i32 as ssize_t
            } else {
              (*ptr_to_globals).linelen = stdin_cnt as libc::c_int;
              np = (*ptr_to_globals)
                .memRchr
                .expect("non-null function pointer")(
                lineptr as *const libc::c_void,
                '\n' as i32,
                stdin_cnt as size_t,
              ) as *mut libc::c_char;
              if !np.is_null() {
                (*ptr_to_globals).linelen = (np.wrapping_offset_from(lineptr) as libc::c_long
                  + 1i32 as libc::c_long) as libc::c_int
              }
              ch = *lineptr.offset(((*ptr_to_globals).linelen - 1i32) as isize)
            }
            /* linelen == no of chars incl. '\n' (or == stdin_cnt) */
            i = 0i32;
            while (i as libc::c_uint) < (*ptr_to_globals).dirn {
              if !((*(*ptr_to_globals).dir.offset(i as isize)).fddir == -1i32) {
                if (*(*ptr_to_globals).dir.offset(i as isize)).matcherr as libc::c_int == 'e' as i32
                {
                  // //full_write(STDERR_FILENO, lineptr, linelen);
                  fwrite(
                    lineptr as *const libc::c_void,
                    1i32 as size_t,
                    (*ptr_to_globals).linelen as size_t,
                    stderr,
                  );
                }
                if !((*(*ptr_to_globals).dir.offset(i as isize)).match_0 as libc::c_int
                  != '+' as i32)
                {
                  buffer_pwrite(i, lineptr, (*ptr_to_globals).linelen as libc::c_uint);
                }
              }
              i += 1
            }
          }
          stdin_cnt -= (*ptr_to_globals).linelen as libc::c_long;
          if !(stdin_cnt > 0i32 as libc::c_long) {
            break;
          }
          lineptr = lineptr.offset((*ptr_to_globals).linelen as isize);
          /* If we see another '\n', we don't need to read
           * next piece of input: can print what we have */
          np = (*ptr_to_globals)
            .memRchr
            .expect("non-null function pointer")(
            lineptr as *const libc::c_void,
            '\n' as i32,
            stdin_cnt as size_t,
          ) as *mut libc::c_char;
          if !np.is_null() {
            current_block_116 = 368077705793071303;
            continue;
          }
          /* Move unprocessed data to the front of line */
          memmove(
            if timestamp != 0 {
              bb_common_bufsiz1.as_mut_ptr().offset(26)
            } else {
              bb_common_bufsiz1.as_mut_ptr()
            } as *mut libc::c_void,
            lineptr as *const libc::c_void,
            stdin_cnt as libc::c_ulong,
          );
          break;
        }
      }
    }
    fflush_all();
  }
  i = 0i32;
  while (i as libc::c_uint) < (*ptr_to_globals).dirn {
    if (*(*ptr_to_globals).dir.offset(i as isize)).ppid != 0 {
      while processorstop(&mut *(*ptr_to_globals).dir.offset(i as isize)) == 0 {}
    }
    logdir_close(&mut *(*ptr_to_globals).dir.offset(i as isize));
    i += 1
  }
  return 0i32;
}
