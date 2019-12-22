use crate::libbb::ptr_to_globals::bb_errno;
use crate::librb::size_t;
use crate::librb::smallint;
use c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use libc;
use libc::chdir;
use libc::close;
use libc::getenv;
use libc::open;
use libc::printf;
use libc::ssize_t;
use libc::stat;
use libc::strcmp;
use libc::time;
use libc::time_t;
use libc::useconds_t;
extern "C" {

  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  fn _exit(_: libc::c_int) -> !;
  #[no_mangle]
  fn fchdir(__fd: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn usleep(__useconds: useconds_t) -> libc::c_int;
  #[no_mangle]
  fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
  #[no_mangle]
  fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;

  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;

  /* Some useful definitions */
  /* Macros for min/max.  */
  /* buffer allocation schemes */
  /* glibc uses __errno_location() to get a ptr to errno */
  /* We can just memorize it once - no multithreading in busybox :) */

  /* NB: can violate const-ness (similarly to strchr) */

  /* Guaranteed to NOT be a macro (smallest code). Saves nearly 2k on uclibc.
   * But potentially slow, don't use in one-billion-times loops */

  /* NOMMU friendy fork+exec: */

  #[no_mangle]
  static mut xfunc_error_retval: u8;

  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
}

//extern const int const_int_1;
/* This struct is deliberately not defined. */
/* See docs/keep_data_small.txt */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct globals {
  pub acts: *const libc::c_char,
  pub service: *mut *mut libc::c_char,
  pub rc: libc::c_uint,
  pub tstart: u64,
  pub tnow: u64,
  pub svstatus: svstatus_t,
  pub islog: smallint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct svstatus_t {
  pub time_be64: u64,
  pub time_nsec_be32: u32,
  pub pid_le32: u32,
  pub paused: u8,
  pub want: u8,
  pub got_term: u8,
  pub run_or_finish: u8,
}
/* need to zero out, svc calls sv() repeatedly */
unsafe extern "C" fn fatal_cannot(mut m1: *const libc::c_char) -> ! {
  crate::libbb::perror_msg::bb_perror_msg(
    b"fatal: can\'t %s\x00" as *const u8 as *const libc::c_char,
    m1,
  );
  _exit(151i32);
}
unsafe extern "C" fn out(mut p: *const libc::c_char, mut m1: *const libc::c_char) {
  printf(
    b"%s%s%s: %s\x00" as *const u8 as *const libc::c_char,
    p,
    *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).service,
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).islog as libc::c_int != 0 {
      b"/log\x00" as *const u8 as *const libc::c_char
    } else {
      b"\x00" as *const u8 as *const libc::c_char
    },
    m1,
  );
  if *bb_errno != 0 {
    printf(b": %m\x00" as *const u8 as *const libc::c_char);
  }
  crate::libbb::xfuncs_printf::bb_putchar('\n' as i32);
  /* will also flush the output */
}
unsafe extern "C" fn fail(mut m1: *const libc::c_char) {
  let ref mut fresh0 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).rc;
  *fresh0 = (*fresh0).wrapping_add(1);
  out(b"fail: \x00" as *const u8 as *const libc::c_char, m1);
}
unsafe extern "C" fn failx(mut m1: *const libc::c_char) {
  *bb_errno = 0i32;
  fail(m1);
}
unsafe extern "C" fn warn(mut m1: *const libc::c_char) {
  let ref mut fresh1 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).rc;
  *fresh1 = (*fresh1).wrapping_add(1);
  /* "warning: <service>: <m1>\n" */
  out(b"warning: \x00" as *const u8 as *const libc::c_char, m1);
}
unsafe extern "C" fn ok(mut m1: *const libc::c_char) {
  *bb_errno = 0i32;
  out(b"ok: \x00" as *const u8 as *const libc::c_char, m1);
}
unsafe extern "C" fn svstatus_get() -> libc::c_int {
  let mut fd: libc::c_int = 0;
  let mut r: libc::c_int = 0;
  fd = open(
    b"supervise/ok\x00" as *const u8 as *const libc::c_char,
    0o1i32 | 0o4000i32,
  );
  if fd == -1i32 {
    if *bb_errno == 19i32 {
      if *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).acts as libc::c_int == 'x' as i32 {
        ok(b"runsv not running\x00" as *const u8 as *const libc::c_char);
      } else {
        failx(b"runsv not running\x00" as *const u8 as *const libc::c_char);
      };
      return 0i32;
    }
    warn(b"can\'t open supervise/ok\x00" as *const u8 as *const libc::c_char);
    return -1i32;
  }
  close(fd);
  fd = open(
    b"supervise/status\x00" as *const u8 as *const libc::c_char,
    0i32 | 0o4000i32,
  );
  if fd == -1i32 {
    warn(b"can\'t open supervise/status\x00" as *const u8 as *const libc::c_char);
    return -1i32;
  }
  r = read(
    fd,
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svstatus as *mut svstatus_t
      as *mut libc::c_void,
    20i32 as size_t,
  ) as libc::c_int;
  close(fd);
  match r {
    20 => {}
    -1 => {
      warn(b"can\'t read supervise/status\x00" as *const u8 as *const libc::c_char);
      return -1i32;
    }
    _ => {
      *bb_errno = 0i32;
      warn(b"can\'t read supervise/status: bad format\x00" as *const u8 as *const libc::c_char);
      return -1i32;
    }
  }
  return 1i32;
}
unsafe extern "C" fn svstatus_print(mut m: *const libc::c_char) -> libc::c_uint {
  let mut diff: libc::c_int = 0;
  let mut pid: libc::c_int = 0;
  let mut normallyup: libc::c_int = 0i32;
  let mut s: stat = std::mem::zeroed();
  let mut timestamp: u64 = 0;
  if stat(b"down\x00" as *const u8 as *const libc::c_char, &mut s) == -1i32 {
    if *bb_errno != 2i32 {
      crate::libbb::perror_msg::bb_perror_msg(
        b"warning: can\'t stat %s/down\x00" as *const u8 as *const libc::c_char,
        *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).service,
      );
      return 0i32 as libc::c_uint;
    }
    normallyup = 1i32
  }
  pid = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .svstatus
    .pid_le32 as libc::c_int;
  timestamp = {
    let mut __v: u64 = 0;
    let mut __x: u64 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .svstatus
      .time_be64;
    if false {
      __v = ((__x as libc::c_ulonglong & 0xff00000000000000u64) >> 56i32
        | (__x as libc::c_ulonglong & 0xff000000000000u64) >> 40i32
        | (__x as libc::c_ulonglong & 0xff0000000000u64) >> 24i32
        | (__x as libc::c_ulonglong & 0xff00000000u64) >> 8i32
        | (__x as libc::c_ulonglong & 0xff000000u64) << 8i32
        | (__x as libc::c_ulonglong & 0xff0000u64) << 24i32
        | (__x as libc::c_ulonglong & 0xff00u64) << 40i32
        | (__x as libc::c_ulonglong & 0xffu64) << 56i32) as u64
    } else {
      let fresh2 = &mut __v;
      let fresh3;
      let fresh4 = __x;
      asm!("bswap ${0:q}" : "=r" (fresh3) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh2, fresh4)) :);
      c2rust_asm_casts::AsmCast::cast_out(fresh2, fresh4, fresh3);
    }
    __v
  };
  match (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .svstatus
    .run_or_finish as libc::c_int
  {
    0 => {
      printf(b"down: \x00" as *const u8 as *const libc::c_char);
    }
    1 => {
      printf(b"run: \x00" as *const u8 as *const libc::c_char);
    }
    2 => {
      printf(b"finish: \x00" as *const u8 as *const libc::c_char);
    }
    _ => {}
  }
  printf(b"%s: \x00" as *const u8 as *const libc::c_char, m);
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .svstatus
    .run_or_finish
    != 0
  {
    printf(b"(pid %d) \x00" as *const u8 as *const libc::c_char, pid);
  }
  diff = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .tnow
    .wrapping_sub(timestamp) as libc::c_int;
  printf(
    b"%us\x00" as *const u8 as *const libc::c_char,
    if diff < 0i32 { 0i32 } else { diff },
  );
  if pid != 0 {
    if normallyup == 0 {
      printf(b", normally down\x00" as *const u8 as *const libc::c_char);
    }
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .svstatus
      .paused
      != 0
    {
      printf(b", paused\x00" as *const u8 as *const libc::c_char);
    }
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .svstatus
      .want as libc::c_int
      == 'd' as i32
    {
      printf(b", want down\x00" as *const u8 as *const libc::c_char);
    }
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .svstatus
      .got_term
      != 0
    {
      printf(b", got TERM\x00" as *const u8 as *const libc::c_char);
    }
  } else {
    if normallyup != 0 {
      printf(b", normally up\x00" as *const u8 as *const libc::c_char);
    }
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .svstatus
      .want as libc::c_int
      == 'u' as i32
    {
      printf(b", want up\x00" as *const u8 as *const libc::c_char);
    }
  }
  return if pid != 0 { 1i32 } else { 2i32 } as libc::c_uint;
}
unsafe extern "C" fn status(mut _unused: *const libc::c_char) -> libc::c_int {
  let mut r: libc::c_int = 0;
  if svstatus_get() <= 0i32 {
    return 0i32;
  }
  r = svstatus_print(*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).service) as libc::c_int;
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).islog = 1i32 as smallint;
  if chdir(b"log\x00" as *const u8 as *const libc::c_char) == -1i32 {
    if *bb_errno != 2i32 {
      printf(b"; \x00" as *const u8 as *const libc::c_char);
      warn(b"can\'t change directory\x00" as *const u8 as *const libc::c_char);
    } else {
      crate::libbb::xfuncs_printf::bb_putchar('\n' as i32);
    }
  } else {
    printf(b"; \x00" as *const u8 as *const libc::c_char);
    if svstatus_get() != 0 {
      r = svstatus_print(b"log\x00" as *const u8 as *const libc::c_char) as libc::c_int;
      crate::libbb::xfuncs_printf::bb_putchar('\n' as i32);
    }
  }
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).islog = 0i32 as smallint;
  return r;
}
unsafe extern "C" fn checkscript() -> libc::c_int {
  let mut prog: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2];
  let mut s: stat = std::mem::zeroed();
  let mut pid: libc::c_int = 0;
  let mut w: libc::c_int = 0;
  if stat(b"check\x00" as *const u8 as *const libc::c_char, &mut s) == -1i32 {
    if *bb_errno == 2i32 {
      return 1i32;
    }
    crate::libbb::perror_msg::bb_perror_msg(
      b"warning: can\'t stat %s/check\x00" as *const u8 as *const libc::c_char,
      *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).service,
    );
    return 0i32;
  }
  /* if (!(s.st_mode & S_IXUSR)) return 1; */
  prog[0] = b"./check\x00" as *const u8 as *const libc::c_char as *mut libc::c_char; /* will also flush the output */
  prog[1] = std::ptr::null_mut::<libc::c_char>();
  pid = crate::libbb::vfork_daemon_rexec::spawn(prog.as_mut_ptr());
  if pid <= 0i32 {
    crate::libbb::perror_msg::bb_perror_msg(
      b"warning: can\'t %s child %s/check\x00" as *const u8 as *const libc::c_char,
      b"run\x00" as *const u8 as *const libc::c_char,
      *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).service,
    );
    return 0i32;
  }
  if crate::libbb::xfuncs::safe_waitpid(pid, &mut w, 0i32) == -1i32 {
    crate::libbb::perror_msg::bb_perror_msg(
      b"warning: can\'t %s child %s/check\x00" as *const u8 as *const libc::c_char,
      b"wait for\x00" as *const u8 as *const libc::c_char,
      *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).service,
    );
    return 0i32;
  }
  return ((w & 0xff00i32) >> 8i32 == 0i32) as libc::c_int;
}
unsafe extern "C" fn check(mut a: *const libc::c_char) -> libc::c_int {
  let mut r: libc::c_int = 0;
  let mut pid_le32: libc::c_uint = 0;
  let mut timestamp: u64 = 0;
  r = svstatus_get();
  if r == -1i32 {
    return -1i32;
  }
  while *a != 0 {
    if r == 0i32 {
      if *a as libc::c_int == 'x' as i32 {
        return 1i32;
      }
      return -1i32;
    }
    pid_le32 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .svstatus
      .pid_le32;
    match *a as libc::c_int {
      120 => return 0i32,
      117 => {
        if pid_le32 == 0
          || (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
            .svstatus
            .run_or_finish as libc::c_int
            != 1i32
        {
          return 0i32;
        }
        if checkscript() == 0 {
          return 0i32;
        }
      }
      100 => {
        if pid_le32 != 0
          || (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
            .svstatus
            .run_or_finish as libc::c_int
            != 0i32
        {
          return 0i32;
        }
      }
      67 => {
        if pid_le32 != 0 && checkscript() == 0 {
          return 0i32;
        }
      }
      116 | 107 => {
        if !(pid_le32 == 0
          && (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
            .svstatus
            .want as libc::c_int
            == 'd' as i32)
        {
          timestamp = {
            let mut __v: u64 = 0;
            let mut __x: u64 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
              .svstatus
              .time_be64;
            if false {
              __v = ((__x as libc::c_ulonglong & 0xff00000000000000u64) >> 56i32
                | (__x as libc::c_ulonglong & 0xff000000000000u64) >> 40i32
                | (__x as libc::c_ulonglong & 0xff0000000000u64) >> 24i32
                | (__x as libc::c_ulonglong & 0xff00000000u64) >> 8i32
                | (__x as libc::c_ulonglong & 0xff000000u64) << 8i32
                | (__x as libc::c_ulonglong & 0xff0000u64) << 24i32
                | (__x as libc::c_ulonglong & 0xff00u64) << 40i32
                | (__x as libc::c_ulonglong & 0xffu64) << 56i32) as u64
            } else {
              let fresh5 = &mut __v;
              let fresh6;
              let fresh7 = __x;
              asm!("bswap ${0:q}" : "=r" (fresh6) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh5, fresh7)) :);
              c2rust_asm_casts::AsmCast::cast_out(fresh5, fresh7, fresh6);
            }
            __v
          };
          if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).tstart > timestamp
            || pid_le32 == 0
            || (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
              .svstatus
              .got_term as libc::c_int
              != 0
            || checkscript() == 0
          {
            return 0i32;
          }
        }
      }
      111 => {
        timestamp = {
          let mut __v: u64 = 0;
          let mut __x: u64 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
            .svstatus
            .time_be64;
          if false {
            __v = ((__x as libc::c_ulonglong & 0xff00000000000000u64) >> 56i32
              | (__x as libc::c_ulonglong & 0xff000000000000u64) >> 40i32
              | (__x as libc::c_ulonglong & 0xff0000000000u64) >> 24i32
              | (__x as libc::c_ulonglong & 0xff00000000u64) >> 8i32
              | (__x as libc::c_ulonglong & 0xff000000u64) << 8i32
              | (__x as libc::c_ulonglong & 0xff0000u64) << 24i32
              | (__x as libc::c_ulonglong & 0xff00u64) << 40i32
              | (__x as libc::c_ulonglong & 0xffu64) << 56i32) as u64
          } else {
            let fresh8 = &mut __v;
            let fresh9;
            let fresh10 = __x;
            asm!("bswap ${0:q}" : "=r" (fresh9) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh8, fresh10)) :);
            c2rust_asm_casts::AsmCast::cast_out(fresh8, fresh10, fresh9);
          }
          __v
        };
        if pid_le32 == 0 && (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).tstart > timestamp
          || pid_le32 != 0
            && (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
              .svstatus
              .want as libc::c_int
              != 'd' as i32
        {
          return 0i32;
        }
      }
      112 => {
        if pid_le32 != 0
          && (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
            .svstatus
            .paused
            == 0
        {
          return 0i32;
        }
      }
      99 => {
        if pid_le32 != 0
          && (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
            .svstatus
            .paused as libc::c_int
            != 0
        {
          return 0i32;
        }
      }
      _ => {}
    }
    a = a.offset(1)
  }
  printf(b"ok: \x00" as *const u8 as *const libc::c_char);
  svstatus_print(*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).service);
  crate::libbb::xfuncs_printf::bb_putchar('\n' as i32);
  return 1i32;
}
unsafe extern "C" fn control(mut a: *const libc::c_char) -> libc::c_int {
  let mut fd: libc::c_int = 0;
  let mut r: libc::c_int = 0;
  let mut l: libc::c_int = 0;
  if svstatus_get() <= 0i32 {
    return -1i32;
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .svstatus
    .want as libc::c_int
    == *a as libc::c_int
    && (*a as libc::c_int != 'd' as i32
      || (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .svstatus
        .got_term as libc::c_int
        == 1i32)
  {
    return 0i32;
  }
  fd = open(
    b"supervise/control\x00" as *const u8 as *const libc::c_char,
    0o1i32 | 0o4000i32,
  );
  if fd == -1i32 {
    if *bb_errno != 19i32 {
      warn(b"can\'t open supervise/control\x00" as *const u8 as *const libc::c_char);
    } else {
      if *a as libc::c_int == 'x' as i32 {
        ok(b"runsv not running\x00" as *const u8 as *const libc::c_char);
      } else {
        failx(b"runsv not running\x00" as *const u8 as *const libc::c_char);
      };
    }
    return -1i32;
  }
  l = strlen(a) as libc::c_int;
  r = write(fd, a as *const libc::c_void, l as size_t) as libc::c_int;
  close(fd);
  if r != l {
    warn(b"can\'t write to supervise/control\x00" as *const u8 as *const libc::c_char);
    return -1i32;
  }
  return 1i32;
}
//usage:#define sv_trivial_usage
//usage:       "[-v] [-w SEC] CMD SERVICE_DIR..."
//usage:#define sv_full_usage "\n\n"
//usage:       "Control services monitored by runsv supervisor.\n"
//usage:       "Commands (only first character is enough):\n"
//usage:       "\n"
//usage:       "status: query service status\n"
//usage:       "up: if service isn't running, start it. If service stops, restart it\n"
//usage:       "once: like 'up', but if service stops, don't restart it\n"
//usage:       "down: send TERM and CONT signals. If ./run exits, start ./finish\n"
//usage:       "	if it exists. After it stops, don't restart service\n"
//usage:       "exit: send TERM and CONT signals to service and log service. If they exit,\n"
//usage:       "	runsv exits too\n"
//usage:       "pause, cont, hup, alarm, interrupt, quit, 1, 2, term, kill: send\n"
//usage:       "STOP, CONT, HUP, ALRM, INT, QUIT, USR1, USR2, TERM, KILL signal to service"
unsafe extern "C" fn sv(mut argv: *mut *mut libc::c_char) -> libc::c_int {
  let mut current_block: u64;
  let mut x: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut action: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut varservice: *const libc::c_char = b"/var/service\x00" as *const u8 as *const libc::c_char;
  let mut waitsec: libc::c_uint = 7i32 as libc::c_uint;
  let mut kll: smallint = 0i32 as smallint;
  let mut verbose: libc::c_int = 0i32;
  let mut act: Option<unsafe extern "C" fn(_: *const libc::c_char) -> libc::c_int> = None;
  let mut cbk: Option<unsafe extern "C" fn(_: *const libc::c_char) -> libc::c_int> = None;
  let mut curdir: libc::c_int = 0;
  memset(
    bb_common_bufsiz1.as_mut_ptr() as *mut globals as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<globals>() as libc::c_ulong,
  );
  xfunc_error_retval = 100i32 as u8;
  x = getenv(b"SVDIR\x00" as *const u8 as *const libc::c_char);
  if !x.is_null() {
    varservice = x
  }
  x = getenv(b"SVWAIT\x00" as *const u8 as *const libc::c_char);
  if !x.is_null() {
    waitsec = crate::libbb::xatonum::xatou(x)
  }
  crate::libbb::getopt32::getopt32(
    argv,
    b"^w:+v\x00vv\x00" as *const u8 as *const libc::c_char,
    &mut waitsec as *mut libc::c_uint,
    &mut verbose as *mut libc::c_int,
  );
  argv = argv.offset(optind as isize);
  let fresh11 = argv;
  argv = argv.offset(1);
  action = *fresh11;
  if action.is_null() || (*argv).is_null() {
    crate::libbb::appletlib::bb_show_usage();
  }
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).tnow =
    (time(0 as *mut time_t) as libc::c_ulonglong).wrapping_add(0x400000000000000au64) as u64;
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).tstart =
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).tnow;
  curdir = open(
    b".\x00" as *const u8 as *const libc::c_char,
    0i32 | 0o4000i32,
  );
  if curdir == -1i32 {
    fatal_cannot(b"open current directory\x00" as *const u8 as *const libc::c_char);
  }
  act = Some(control as unsafe extern "C" fn(_: *const libc::c_char) -> libc::c_int);
  let ref mut fresh12 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).acts;
  *fresh12 = b"s\x00" as *const u8 as *const libc::c_char;
  cbk = Some(check as unsafe extern "C" fn(_: *const libc::c_char) -> libc::c_int);
  let mut current_block_56: u64;
  match *action as libc::c_int {
    120 | 101 => {
      let ref mut fresh13 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).acts;
      *fresh13 = b"x\x00" as *const u8 as *const libc::c_char;
      if verbose == 0 {
        cbk = None
      }
      current_block_56 = 14294131666767243020;
    }
    88 | 69 => {
      let ref mut fresh14 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).acts;
      *fresh14 = b"x\x00" as *const u8 as *const libc::c_char;
      kll = 1i32 as smallint;
      current_block_56 = 14294131666767243020;
    }
    68 => {
      let ref mut fresh15 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).acts;
      *fresh15 = b"d\x00" as *const u8 as *const libc::c_char;
      kll = 1i32 as smallint;
      current_block_56 = 14294131666767243020;
    }
    84 => {
      let ref mut fresh16 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).acts;
      *fresh16 = b"tc\x00" as *const u8 as *const libc::c_char;
      kll = 1i32 as smallint;
      current_block_56 = 14294131666767243020;
    }
    116 => {
      if strcmp(
        action,
        b"try-restart\x00" as *const u8 as *const libc::c_char,
      ) == 0i32
      {
        let ref mut fresh17 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).acts;
        *fresh17 = b"tc\x00" as *const u8 as *const libc::c_char;
        current_block_56 = 14294131666767243020;
      } else {
        current_block_56 = 9005509770874399122;
      }
    }
    99 => {
      current_block_56 = 9005509770874399122;
    }
    117 | 100 | 111 | 112 | 104 | 97 | 105 | 107 | 113 | 49 | 50 => {
      current_block_56 = 14309576813620330651;
    }
    115 => {
      if strcmp(action, b"shutdown\x00" as *const u8 as *const libc::c_char) == 0i32 {
        let ref mut fresh20 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).acts;
        *fresh20 = b"x\x00" as *const u8 as *const libc::c_char
      } else if strcmp(action, b"start\x00" as *const u8 as *const libc::c_char) == 0i32 {
        let ref mut fresh21 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).acts;
        *fresh21 = b"u\x00" as *const u8 as *const libc::c_char
      } else if strcmp(action, b"stop\x00" as *const u8 as *const libc::c_char) == 0i32 {
        let ref mut fresh22 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).acts;
        *fresh22 = b"d\x00" as *const u8 as *const libc::c_char
      } else {
        /* "status" */
        act = Some(status as unsafe extern "C" fn(_: *const libc::c_char) -> libc::c_int);
        cbk = None
      }
      current_block_56 = 14294131666767243020;
    }
    114 => {
      if strcmp(action, b"restart\x00" as *const u8 as *const libc::c_char) == 0i32 {
        let ref mut fresh23 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).acts;
        *fresh23 = b"tcu\x00" as *const u8 as *const libc::c_char
      } else if strcmp(action, b"reload\x00" as *const u8 as *const libc::c_char) == 0i32 {
        let ref mut fresh24 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).acts;
        *fresh24 = b"h\x00" as *const u8 as *const libc::c_char
      } else {
        crate::libbb::appletlib::bb_show_usage();
      }
      current_block_56 = 14294131666767243020;
    }
    102 => {
      if strcmp(
        action,
        b"force-reload\x00" as *const u8 as *const libc::c_char,
      ) == 0i32
      {
        let ref mut fresh25 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).acts;
        *fresh25 = b"tc\x00" as *const u8 as *const libc::c_char;
        kll = 1i32 as smallint;
        current_block_56 = 14294131666767243020;
      } else if strcmp(
        action,
        b"force-restart\x00" as *const u8 as *const libc::c_char,
      ) == 0i32
      {
        let ref mut fresh26 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).acts;
        *fresh26 = b"tcu\x00" as *const u8 as *const libc::c_char;
        kll = 1i32 as smallint;
        current_block_56 = 14294131666767243020;
      } else if strcmp(
        action,
        b"force-shutdown\x00" as *const u8 as *const libc::c_char,
      ) == 0i32
      {
        let ref mut fresh27 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).acts;
        *fresh27 = b"x\x00" as *const u8 as *const libc::c_char;
        kll = 1i32 as smallint;
        current_block_56 = 14294131666767243020;
      } else if strcmp(
        action,
        b"force-stop\x00" as *const u8 as *const libc::c_char,
      ) == 0i32
      {
        let ref mut fresh28 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).acts;
        *fresh28 = b"d\x00" as *const u8 as *const libc::c_char;
        kll = 1i32 as smallint;
        current_block_56 = 14294131666767243020;
      } else {
        current_block_56 = 796904743902263619;
      }
    }
    _ => {
      current_block_56 = 796904743902263619;
    }
  }
  match current_block_56 {
    9005509770874399122 => {
      if strcmp(action, b"check\x00" as *const u8 as *const libc::c_char) == 0i32 {
        act = None;
        let ref mut fresh18 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).acts;
        *fresh18 = b"C\x00" as *const u8 as *const libc::c_char;
        current_block_56 = 14294131666767243020;
      } else {
        current_block_56 = 14309576813620330651;
      }
    }
    796904743902263619 => {
      crate::libbb::appletlib::bb_show_usage();
    }
    _ => {}
  }
  match current_block_56 {
    14309576813620330651 => {
      *action.offset(1) = '\u{0}' as i32 as libc::c_char;
      let ref mut fresh19 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).acts;
      *fresh19 = action;
      if verbose == 0 {
        cbk = None
      }
    }
    _ => {}
  }
  let ref mut fresh29 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).service;
  *fresh29 = argv;
  loop {
    x = *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).service;
    if x.is_null() {
      break;
    }
    let mut current_block_63: u64;
    if *x.offset(0) as libc::c_int != '/' as i32
      && *x.offset(0) as libc::c_int != '.' as i32
      && crate::libbb::last_char_is::last_char_is(x, '/' as i32).is_null()
    {
      if chdir(varservice) == -1i32 {
        current_block_63 = 18071914750955744041;
      } else {
        current_block_63 = 2750570471926810434;
      }
    } else {
      current_block_63 = 2750570471926810434;
    }
    match current_block_63 {
      2750570471926810434 => {
        if chdir(x) == -1i32 {
          current_block_63 = 18071914750955744041;
        } else if act.is_some()
          && act.expect("non-null function pointer")(
            (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).acts,
          ) == -1i32
        {
          current_block_63 = 3992760240680818840;
        } else {
          current_block_63 = 6540614962658479183;
        }
      }
      _ => {}
    }
    match current_block_63 {
      18071914750955744041 => {
        fail(b"can\'t change to service directory\x00" as *const u8 as *const libc::c_char);
        current_block_63 = 3992760240680818840;
      }
      _ => {}
    }
    match current_block_63 {
      3992760240680818840 => {
        let ref mut fresh30 = *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).service;
        *fresh30 = -1i64 as *mut libc::c_char
        /* "dead" */
      }
      _ => {}
    }
    if fchdir(curdir) == -1i32 {
      fatal_cannot(b"change to original directory\x00" as *const u8 as *const libc::c_char);
    }
    let ref mut fresh31 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).service;
    *fresh31 = (*fresh31).offset(1)
  }
  if cbk.is_some() {
    loop {
      let mut want_exit: libc::c_int = 0;
      let mut diff: libc::c_int = 0;
      diff = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .tnow
        .wrapping_sub((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).tstart)
        as libc::c_int;
      let ref mut fresh32 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).service;
      *fresh32 = argv;
      want_exit = 1i32;
      loop {
        x = *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).service;
        if x.is_null() {
          break;
        }
        if !(x == -1i64 as *mut libc::c_char) {
          if *x.offset(0) as libc::c_int != '/' as i32 && *x.offset(0) as libc::c_int != '.' as i32
          {
            if chdir(varservice) == -1i32 {
              current_block = 18133677720280896402;
            } else {
              current_block = 16593409533420678784;
            }
          } else {
            current_block = 16593409533420678784;
          }
          match current_block {
            16593409533420678784 => {
              if chdir(x) == -1i32 {
                current_block = 18133677720280896402;
              } else if cbk.expect("non-null function pointer")(
                (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).acts,
              ) != 0i32
              {
                current_block = 17181890625510875756;
              } else {
                want_exit = 0i32;
                if diff as libc::c_uint >= waitsec {
                  printf(if kll as libc::c_int != 0 {
                    b"kill: \x00" as *const u8 as *const libc::c_char
                  } else {
                    b"timeout: \x00" as *const u8 as *const libc::c_char
                  });
                  if svstatus_get() > 0i32 {
                    svstatus_print(x);
                    let ref mut fresh33 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).rc;
                    *fresh33 = (*fresh33).wrapping_add(1)
                  }
                  /* "dead" */
                  crate::libbb::xfuncs_printf::bb_putchar('\n' as i32); /* will also flush the output */
                  if kll != 0 {
                    control(b"k\x00" as *const u8 as *const libc::c_char);
                  }
                  current_block = 17181890625510875756;
                } else {
                  current_block = 1930794479672247912;
                }
              }
            }
            _ => {}
          }
          match current_block {
            18133677720280896402 => {
              fail(b"can\'t change to service directory\x00" as *const u8 as *const libc::c_char);
              current_block = 17181890625510875756;
            }
            _ => {}
          }
          match current_block {
            17181890625510875756 => {
              let ref mut fresh34 = *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).service;
              *fresh34 = -1i64 as *mut libc::c_char
            }
            _ => {}
          }
          if fchdir(curdir) == -1i32 {
            fatal_cannot(b"change to original directory\x00" as *const u8 as *const libc::c_char);
          }
        }
        /* "dead" */
        let ref mut fresh35 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).service;
        *fresh35 = (*fresh35).offset(1)
      }
      if want_exit != 0 {
        break;
      }
      usleep(420000i32 as useconds_t);
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).tnow =
        (time(0 as *mut time_t) as libc::c_ulonglong).wrapping_add(0x400000000000000au64) as u64
    }
  }
  return if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).rc > 99i32 as libc::c_uint {
    99i32 as libc::c_uint
  } else {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).rc
  } as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sv_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  return sv(argv);
}
//usage:#define svc_trivial_usage
//usage:       "[-udopchaitkx] SERVICE_DIR..."
//usage:#define svc_full_usage "\n\n"
//usage:       "Control services monitored by runsv supervisor"
//usage:   "\n"
//usage:   "\n""	-u	If service is not running, start it; restart if it stops"
//usage:   "\n""	-d	If service is running, send TERM+CONT signals; do not restart it"
//usage:   "\n""	-o	Once: if service is not running, start it; do not restart it"
//usage:   "\n""	-pchaitk Send STOP, CONT, HUP, ALRM, INT, TERM, KILL signal to service"
//usage:   "\n""	-x	Exit: runsv will exit as soon as the service is down"
#[no_mangle]
pub unsafe extern "C" fn svc_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut command: [libc::c_char; 2] = [0; 2];
  let mut optstring: *const libc::c_char = std::ptr::null();
  let mut opts: libc::c_uint = 0;
  optstring = b"udopchaitkx\x00" as *const u8 as *const libc::c_char;
  opts = crate::libbb::getopt32::getopt32(argv, optstring);
  argv = argv.offset(optind as isize);
  if (*argv.offset(0)).is_null() || opts == 0 {
    crate::libbb::appletlib::bb_show_usage();
  }
  argv = argv.offset(-2);
  if optind > 2i32 {
    argv = argv.offset(-1);
    let ref mut fresh36 = *argv.offset(2);
    *fresh36 = b"--\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
  }
  let ref mut fresh37 = *argv.offset(0);
  *fresh37 = b"sv\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
  let ref mut fresh38 = *argv.offset(1);
  *fresh38 = command.as_mut_ptr();
  command[1] = '\u{0}' as i32 as libc::c_char;
  loop {
    if opts & 1i32 as libc::c_uint != 0 {
      let mut r: libc::c_int = 0;
      command[0] = *optstring;
      /* getopt() was already called by getopt32():
       * reset the libc getopt() function's internal state.
       */
      optind = 0i32;
      r = sv(argv);
      if r != 0 {
        return 1i32;
      }
    }
    optstring = optstring.offset(1);
    opts >>= 1i32;
    if !(opts != 0) {
      break;
    }
  }
  return 0i32;
}
//usage:#define svok_trivial_usage
//usage:       "SERVICE_DIR"
//usage:#define svok_full_usage "\n\n"
//usage:       "Check whether runsv supervisor is running.\n"
//usage:       "Exit code is 0 if it does, 100 if it does not,\n"
//usage:       "111 (with error message) if SERVICE_DIR does not exist."
#[no_mangle]
pub unsafe extern "C" fn svok_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut dir: *const libc::c_char = *argv.offset(1);
  if dir.is_null() {
    crate::libbb::appletlib::bb_show_usage();
  }
  xfunc_error_retval = 111i32 as u8;
  /*
   * daemontools has no concept of "default service dir", runit does.
   * Let's act as runit.
   */
  if *dir.offset(0) as libc::c_int != '/' as i32
    && *dir.offset(0) as libc::c_int != '.' as i32
    && crate::libbb::last_char_is::last_char_is(dir, '/' as i32).is_null()
  {
    crate::libbb::xfuncs_printf::xchdir(b"/var/service\x00" as *const u8 as *const libc::c_char);
  }
  crate::libbb::xfuncs_printf::xchdir(dir);
  if open(
    b"supervise/ok\x00" as *const u8 as *const libc::c_char,
    0o1i32,
  ) < 0i32
  {
    if *bb_errno == 2i32 || *bb_errno == 6i32 {
      return 100i32;
    }
    crate::libbb::perror_msg::bb_perror_msg_and_die(
      b"can\'t open \'%s\'\x00" as *const u8 as *const libc::c_char,
      b"supervise/ok\x00" as *const u8 as *const libc::c_char,
    );
  }
  return 0i32;
}
