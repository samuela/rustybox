use c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use libc;
extern "C" {
  #[no_mangle]
  fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
  #[no_mangle]
  fn flock(__fd: libc::c_int, __operation: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn unlink(__name: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn readlink(__path: *const libc::c_char, __buf: *mut libc::c_char, __len: size_t) -> ssize_t;
  #[no_mangle]
  fn vfork() -> libc::c_int;
  #[no_mangle]
  fn _exit(_: libc::c_int) -> !;
  #[no_mangle]
  fn execl(__path: *const libc::c_char, __arg: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn execv(__path: *const libc::c_char, __argv: *const *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn fchdir(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
  #[no_mangle]
  fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
  #[no_mangle]
  fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn stpcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
  #[no_mangle]
  fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
  #[no_mangle]
  fn mkfifo(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
  #[no_mangle]
  fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn ndelay_on(fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn close_on_exec_on(fd: libc::c_int);
  #[no_mangle]
  fn xdup2(_: libc::c_int, _: libc::c_int);
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
  fn xchdir(path: *const libc::c_char);
  #[no_mangle]
  fn xopen3(pathname: *const libc::c_char, flags: libc::c_int, mode: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn rename_or_warn(oldpath: *const libc::c_char, newpath: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn xpipe(filedes: *mut libc::c_int);
  #[no_mangle]
  fn safe_waitpid(pid: pid_t, wstat: *mut libc::c_int, options: libc::c_int) -> pid_t;
  #[no_mangle]
  fn wait_any_nohang(wstat: *mut libc::c_int) -> pid_t;
  #[no_mangle]
  fn single_argv(argv: *mut *mut libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_perror_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_perror_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn utoa_to_buf(
    n: libc::c_uint,
    buf: *mut libc::c_char,
    buflen: libc::c_uint,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn utoa(n: libc::c_uint) -> *mut libc::c_char;
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
}
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type size_t = libc::c_ulong;
use crate::librb::pid_t;
use crate::librb::stat;
use crate::librb::timespec;
use crate::librb::uint8_t;
use crate::librb::uint32_t;
use crate::librb::uint64_t;
use crate::librb::smallint;
use crate::librb::ssize_t;
pub type clockid_t = __clockid_t;
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
pub struct fd_pair {
  pub rd: libc::c_int,
  pub wr: libc::c_int,
}
//extern const int const_int_1;
/* This struct is deliberately not defined. */
/* See docs/keep_data_small.txt */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub haslog: smallint,
  pub sigterm: smallint,
  pub pidchanged: smallint,
  pub selfpipe: fd_pair,
  pub logpipe: fd_pair,
  pub dir: *mut libc::c_char,
  pub svd: [svdir; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct svdir {
  pub pid: libc::c_int,
  pub state: smallint,
  pub ctrl: smallint,
  pub sd_want: smallint,
  pub islog: smallint,
  pub start: timespec,
  pub fdlock: libc::c_int,
  pub fdcontrol: libc::c_int,
  pub fdcontrolwrite: libc::c_int,
  pub wstat: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct svstatus_t {
  pub time_be64: uint64_t,
  pub time_nsec_be32: uint32_t,
  pub pid_le32: uint32_t,
  pub paused: uint8_t,
  pub want: uint8_t,
  pub got_term: uint8_t,
  pub run_or_finish: uint8_t,
}
unsafe extern "C" fn gettimeofday_ns(mut ts: *mut timespec) {
  clock_gettime(0i32, ts);
}
unsafe extern "C" fn fatal2_cannot(mut m1: *const libc::c_char, mut m2: *const libc::c_char) {
  bb_perror_msg_and_die(
    b"%s: fatal: can\'t %s%s\x00" as *const u8 as *const libc::c_char,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).dir,
    m1,
    m2,
  );
  /* was exiting 111 */
}
unsafe extern "C" fn fatal_cannot(mut m: *const libc::c_char) {
  fatal2_cannot(m, b"\x00" as *const u8 as *const libc::c_char);
  /* was exiting 111 */
}
unsafe extern "C" fn fatal2x_cannot(mut m1: *const libc::c_char, mut m2: *const libc::c_char) {
  bb_error_msg_and_die(
    b"%s: fatal: can\'t %s%s\x00" as *const u8 as *const libc::c_char,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).dir,
    m1,
    m2,
  );
  /* was exiting 111 */
}
unsafe extern "C" fn warn2_cannot(mut m1: *const libc::c_char, mut m2: *const libc::c_char) {
  bb_perror_msg(
    b"%s: warning: can\'t %s%s\x00" as *const u8 as *const libc::c_char,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).dir,
    m1,
    m2,
  );
}
unsafe extern "C" fn warn_cannot(mut m: *const libc::c_char) {
  warn2_cannot(m, b"\x00" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn s_child(mut _sig_no: libc::c_int) {
  write(
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .selfpipe
      .wr,
    b"\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
    1i32 as size_t,
  );
}
unsafe extern "C" fn s_term(mut _sig_no: libc::c_int) {
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).sigterm = 1i32 as smallint;
  write(
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .selfpipe
      .wr,
    b"\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
    1i32 as size_t,
  );
  /* XXX */
}
unsafe extern "C" fn open_trunc_or_warn(mut name: *const libc::c_char) -> libc::c_int {
  /* Why O_NDELAY? */
  let mut fd: libc::c_int = open(name, 0o1i32 | 0o4000i32 | 0o1000i32 | 0o100i32, 0o644i32);
  if fd < 0i32 {
    bb_perror_msg(
      b"%s: warning: cannot open %s\x00" as *const u8 as *const libc::c_char,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).dir,
      name,
    );
  }
  return fd;
}
unsafe extern "C" fn update_status(mut s: *mut svdir) {
  let mut sz: ssize_t = 0;
  let mut fd: libc::c_int = 0;
  let mut status: svstatus_t = svstatus_t {
    time_be64: 0,
    time_nsec_be32: 0,
    pid_le32: 0,
    paused: 0,
    want: 0,
    got_term: 0,
    run_or_finish: 0,
  };
  let mut fstatus: *const libc::c_char =
    b"log/supervise/status\x00" as *const u8 as *const libc::c_char;
  let mut fstatusnew: *const libc::c_char =
    b"log/supervise/status.new\x00" as *const u8 as *const libc::c_char;
  let mut f_stat: *const libc::c_char =
    b"log/supervise/stat\x00" as *const u8 as *const libc::c_char;
  let mut fstatnew: *const libc::c_char =
    b"log/supervise/stat.new\x00" as *const u8 as *const libc::c_char;
  let mut fpid: *const libc::c_char = b"log/supervise/pid\x00" as *const u8 as *const libc::c_char;
  let mut fpidnew: *const libc::c_char =
    b"log/supervise/pid.new\x00" as *const u8 as *const libc::c_char;
  if (*s).islog == 0 {
    fstatus = fstatus.offset(4);
    fstatusnew = fstatusnew.offset(4);
    f_stat = f_stat.offset(4);
    fstatnew = fstatnew.offset(4);
    fpid = fpid.offset(4);
    fpidnew = fpidnew.offset(4)
  }
  /* pid */
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).pidchanged != 0 {
    fd = open_trunc_or_warn(fpidnew);
    if fd < 0i32 {
      return;
    }
    if (*s).pid != 0 {
      let mut spid: [libc::c_char; 14] = [0; 14];
      let mut size: libc::c_int = sprintf(
        spid.as_mut_ptr(),
        b"%u\n\x00" as *const u8 as *const libc::c_char,
        (*s).pid as libc::c_uint,
      );
      write(fd, spid.as_mut_ptr() as *const libc::c_void, size as size_t);
    }
    close(fd);
    if rename_or_warn(fpidnew, fpid) != 0 {
      return;
    }
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).pidchanged = 0i32 as smallint
  }
  /* stat */
  fd = open_trunc_or_warn(fstatnew);
  if fd < -1i32 {
    return;
  }
  let mut stat_buf: [libc::c_char; 37] = [0; 37];
  let mut p: *mut libc::c_char = stat_buf.as_mut_ptr();
  match (*s).state as libc::c_int {
    0 => p = stpcpy(p, b"down\x00" as *const u8 as *const libc::c_char),
    1 => p = stpcpy(p, b"run\x00" as *const u8 as *const libc::c_char),
    2 => p = stpcpy(p, b"finish\x00" as *const u8 as *const libc::c_char),
    _ => {}
  }
  if (*s).ctrl as libc::c_int & 2i32 != 0 {
    p = stpcpy(p, b", paused\x00" as *const u8 as *const libc::c_char)
  }
  if (*s).ctrl as libc::c_int & 1i32 != 0 {
    p = stpcpy(p, b", got TERM\x00" as *const u8 as *const libc::c_char)
  }
  if (*s).state as libc::c_int != 0i32 {
    match (*s).sd_want as libc::c_int {
      1 => p = stpcpy(p, b", want down\x00" as *const u8 as *const libc::c_char),
      2 => p = stpcpy(p, b", want exit\x00" as *const u8 as *const libc::c_char),
      _ => {}
    }
  }
  let fresh0 = p;
  p = p.offset(1);
  *fresh0 = '\n' as i32 as libc::c_char;
  write(
    fd,
    stat_buf.as_mut_ptr() as *const libc::c_void,
    p.wrapping_offset_from(stat_buf.as_mut_ptr()) as libc::c_long as size_t,
  );
  close(fd);
  rename_or_warn(fstatnew, f_stat);
  /* supervise compatibility */
  memset(
    &mut status as *mut svstatus_t as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<svstatus_t>() as libc::c_ulong,
  ); /* replace '?' */
  status.time_be64 = {
    let mut __v: __uint64_t = 0;
    let mut __x: __uint64_t =
      ((*s).start.tv_sec as libc::c_ulonglong).wrapping_add(0x400000000000000au64) as __uint64_t;
    if 0 != 0 {
      __v = ((__x as libc::c_ulonglong & 0xff00000000000000u64) >> 56i32
        | (__x as libc::c_ulonglong & 0xff000000000000u64) >> 40i32
        | (__x as libc::c_ulonglong & 0xff0000000000u64) >> 24i32
        | (__x as libc::c_ulonglong & 0xff00000000u64) >> 8i32
        | (__x as libc::c_ulonglong & 0xff000000u64) << 8i32
        | (__x as libc::c_ulonglong & 0xff0000u64) << 24i32
        | (__x as libc::c_ulonglong & 0xff00u64) << 40i32
        | (__x as libc::c_ulonglong & 0xffu64) << 56i32) as __uint64_t
    } else {
      let fresh1 = &mut __v;
      let fresh2;
      let fresh3 = __x;
      asm!("bswap ${0:q}" : "=r" (fresh2) : "0"
                      (c2rust_asm_casts::AsmCast::cast_in(fresh1, fresh3)) :);
      c2rust_asm_casts::AsmCast::cast_out(fresh1, fresh3, fresh2);
    }
    __v
  };
  status.time_nsec_be32 = {
    let mut __v: libc::c_uint = 0;
    let mut __x: libc::c_uint = (*s).start.tv_nsec as libc::c_uint;
    if 0 != 0 {
      __v = (__x & 0xff000000u32) >> 24i32
        | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
        | (__x & 0xff00i32 as libc::c_uint) << 8i32
        | (__x & 0xffi32 as libc::c_uint) << 24i32
    } else {
      let fresh4 = &mut __v;
      let fresh5;
      let fresh6 = __x;
      asm!("bswap $0" : "=r" (fresh5) : "0"
                      (c2rust_asm_casts::AsmCast::cast_in(fresh4, fresh6)) :);
      c2rust_asm_casts::AsmCast::cast_out(fresh4, fresh6, fresh5);
    }
    __v
  };
  status.pid_le32 = (*s).pid as uint32_t;
  if (*s).ctrl as libc::c_int & 2i32 != 0 {
    status.paused = 1i32 as uint8_t
  }
  if (*s).sd_want as libc::c_int == 0i32 {
    status.want = 'u' as i32 as uint8_t
  } else {
    status.want = 'd' as i32 as uint8_t
  }
  if (*s).ctrl as libc::c_int & 1i32 != 0 {
    status.got_term = 1i32 as uint8_t
  }
  status.run_or_finish = (*s).state as uint8_t;
  fd = open_trunc_or_warn(fstatusnew);
  if fd < 0i32 {
    return;
  }
  sz = write(
    fd,
    &mut status as *mut svstatus_t as *const libc::c_void,
    ::std::mem::size_of::<svstatus_t>() as libc::c_ulong,
  );
  close(fd);
  if sz as libc::c_ulong != ::std::mem::size_of::<svstatus_t>() as libc::c_ulong {
    warn2_cannot(
      b"write \x00" as *const u8 as *const libc::c_char,
      fstatusnew,
    );
    unlink(fstatusnew);
    return;
  }
  rename_or_warn(fstatusnew, fstatus);
}
unsafe extern "C" fn custom(mut s: *mut svdir, mut c: libc::c_char) -> libc::c_uint {
  let mut pid: pid_t = 0;
  let mut w: libc::c_int = 0;
  let mut a: [libc::c_char; 10] = [0; 10];
  let mut st: stat = stat {
    st_dev: 0,
    st_ino: 0,
    st_nlink: 0,
    st_mode: 0,
    st_uid: 0,
    st_gid: 0,
    __pad0: 0,
    st_rdev: 0,
    st_size: 0,
    st_blksize: 0,
    st_blocks: 0,
    st_atim: timespec {
      tv_sec: 0,
      tv_nsec: 0,
    },
    st_mtim: timespec {
      tv_sec: 0,
      tv_nsec: 0,
    },
    st_ctim: timespec {
      tv_sec: 0,
      tv_nsec: 0,
    },
    __glibc_reserved: [0; 3],
  };
  if (*s).islog != 0 {
    return 0i32 as libc::c_uint;
  }
  strcpy(
    a.as_mut_ptr(),
    b"control/?\x00" as *const u8 as *const libc::c_char,
  );
  a[8] = c;
  if stat(a.as_mut_ptr(), &mut st) == 0i32 {
    if st.st_mode & 0o100i32 as libc::c_uint != 0 {
      pid = vfork();
      if pid == -1i32 {
        warn2_cannot(
          b"vfork for \x00" as *const u8 as *const libc::c_char,
          a.as_mut_ptr(),
        );
        return 0i32 as libc::c_uint;
      }
      if pid == 0i32 {
        /* child */
        if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).haslog as libc::c_int != 0
          && dup2(
            (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
              .logpipe
              .wr,
            1i32,
          ) == -1i32
        {
          warn2_cannot(
            b"setup stdout for \x00" as *const u8 as *const libc::c_char,
            a.as_mut_ptr(),
          );
        }
        execl(
          a.as_mut_ptr(),
          a.as_mut_ptr(),
          0 as *mut libc::c_void as *mut libc::c_char,
        );
        fatal2_cannot(
          b"run \x00" as *const u8 as *const libc::c_char,
          a.as_mut_ptr(),
        );
      }
      /* parent */
      if safe_waitpid(pid, &mut w, 0i32) == -1i32 {
        warn2_cannot(
          b"wait for child \x00" as *const u8 as *const libc::c_char,
          a.as_mut_ptr(),
        );
        return 0i32 as libc::c_uint;
      }
      return ((w & 0xff00i32) >> 8i32 == 0i32) as libc::c_int as libc::c_uint;
    }
  } else if *bb_errno != 2i32 {
    warn2_cannot(
      b"stat \x00" as *const u8 as *const libc::c_char,
      a.as_mut_ptr(),
    );
  }
  return 0i32 as libc::c_uint;
}
unsafe extern "C" fn stopservice(mut s: *mut svdir) {
  if (*s).pid != 0 && custom(s, 't' as i32 as libc::c_char) == 0 {
    kill((*s).pid, 15i32);
    (*s).ctrl = ((*s).ctrl as libc::c_int | 1i32) as smallint;
    update_status(s);
  }
  if (*s).sd_want as libc::c_int == 1i32 {
    kill((*s).pid, 18i32);
    custom(s, 'd' as i32 as libc::c_char);
    return;
  }
  if (*s).sd_want as libc::c_int == 2i32 {
    kill((*s).pid, 18i32);
    custom(s, 'x' as i32 as libc::c_char);
  };
}
unsafe extern "C" fn startservice(mut s: *mut svdir) {
  let mut p: libc::c_int = 0;
  let mut arg: [*const libc::c_char; 4] = [0 as *const libc::c_char; 4];
  let mut exitcode: [libc::c_char; 14] = [0; 14];
  if (*s).state as libc::c_int == 2i32 {
    /* Two arguments are given to ./finish. The first one is ./run exit code,
     * or -1 if ./run didnt exit normally. The second one is
     * the least significant byte of the exit status as determined by waitpid;
     * for instance it is 0 if ./run exited normally, and the signal number
     * if ./run was terminated by a signal. If runsv cannot start ./run
     * for some reason, the exit code is 111 and the status is 0.
     */
    arg[0] = b"./finish\x00" as *const u8 as *const libc::c_char;
    arg[1] = b"-1\x00" as *const u8 as *const libc::c_char;
    if (*s).wstat & 0x7fi32 == 0i32 {
      *utoa_to_buf(
        (((*s).wstat & 0xff00i32) >> 8i32) as libc::c_uint,
        exitcode.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong as libc::c_uint,
      ) = '\u{0}' as i32 as libc::c_char;
      arg[1] = exitcode.as_mut_ptr()
    }
    //arg[2] = "0";
    //if (WIFSIGNALED(s->wstat)) {
    arg[2] = utoa(((*s).wstat & 0x7fi32) as libc::c_uint);
    //}
    arg[3] = 0 as *const libc::c_char
  } else {
    arg[0] = b"./run\x00" as *const u8 as *const libc::c_char; /* should never happen */
    arg[1] = 0 as *const libc::c_char;
    custom(s, 'u' as i32 as libc::c_char);
  }
  if (*s).pid != 0i32 {
    stopservice(s);
  }
  loop {
    p = vfork();
    if !(p == -1i32) {
      break;
    }
    warn_cannot(b"vfork, sleeping\x00" as *const u8 as *const libc::c_char);
    sleep(5i32 as libc::c_uint);
  }
  if p == 0i32 {
    /* child */
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).haslog != 0 {
      /* NB: bug alert! right order is close, then dup2 */
      if (*s).islog != 0 {
        xchdir(b"./log\x00" as *const u8 as *const libc::c_char);
        close(
          (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
            .logpipe
            .wr,
        );
        xdup2(
          (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
            .logpipe
            .rd,
          0i32,
        );
      } else {
        close(
          (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
            .logpipe
            .rd,
        );
        xdup2(
          (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
            .logpipe
            .wr,
          1i32,
        );
      }
    }
    /* Non-ignored signals revert to SIG_DFL on exec anyway */
    /*bb_signals(0
    + (1 << SIGCHLD)
    + (1 << SIGTERM)
    , SIG_DFL);*/
    sig_unblock(17i32);
    sig_unblock(15i32);
    execv(
      arg[0],
      arg.as_mut_ptr() as *mut *mut libc::c_char as *const *mut libc::c_char,
    );
    fatal2_cannot(
      if (*s).islog as libc::c_int != 0 {
        b"start log/\x00" as *const u8 as *const libc::c_char
      } else {
        b"start \x00" as *const u8 as *const libc::c_char
      },
      arg[0],
    );
  }
  /* parent */
  if (*s).state as libc::c_int != 2i32 {
    gettimeofday_ns(&mut (*s).start);
    (*s).state = 1i32 as smallint
  }
  (*s).pid = p;
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).pidchanged = 1i32 as smallint;
  (*s).ctrl = 0i32 as smallint;
  update_status(s);
}
unsafe extern "C" fn ctrl(mut s: *mut svdir, mut c: libc::c_char) -> libc::c_int {
  let mut current_block: u64;
  let mut sig: libc::c_int = 0;
  match c as libc::c_int {
    100 => {
      /* down */
      (*s).sd_want = 1i32 as smallint;
      update_status(s);
      if (*s).state as libc::c_int == 1i32 {
        stopservice(s);
      }
      current_block = 18435049525520518667;
    }
    117 => {
      /* up */
      (*s).sd_want = 0i32 as smallint;
      update_status(s);
      if (*s).state as libc::c_int == 0i32 {
        startservice(s);
      }
      current_block = 18435049525520518667;
    }
    120 => {
      /* exit */
      if (*s).islog != 0 {
        current_block = 18435049525520518667;
      } else {
        (*s).sd_want = 2i32 as smallint;
        update_status(s);
        current_block = 9959655061420637642;
      }
    }
    116 => {
      current_block = 9959655061420637642;
    }
    107 => {
      /* sig kill */
      if (*s).state as libc::c_int == 1i32 && custom(s, c) == 0 {
        kill((*s).pid, 9i32);
      }
      (*s).state = 0i32 as smallint;
      current_block = 18435049525520518667;
    }
    112 => {
      /* sig pause */
      if (*s).state as libc::c_int == 1i32 && custom(s, c) == 0 {
        kill((*s).pid, 19i32);
      }
      (*s).ctrl = ((*s).ctrl as libc::c_int | 2i32) as smallint;
      update_status(s);
      current_block = 18435049525520518667;
    }
    99 => {
      /* sig cont */
      if (*s).state as libc::c_int == 1i32 && custom(s, c) == 0 {
        kill((*s).pid, 18i32);
      }
      (*s).ctrl = ((*s).ctrl as libc::c_int & !2i32) as smallint;
      update_status(s);
      current_block = 18435049525520518667;
    }
    111 => {
      /* once */
      (*s).sd_want = 1i32 as smallint;
      update_status(s);
      if (*s).state as libc::c_int == 0i32 {
        startservice(s);
      }
      current_block = 18435049525520518667;
    }
    97 => {
      /* sig alarm */
      sig = 14i32;
      current_block = 16913877891475873216;
    }
    104 => {
      /* sig hup */
      sig = 1i32;
      current_block = 16913877891475873216;
    }
    105 => {
      /* sig int */
      sig = 2i32;
      current_block = 16913877891475873216;
    }
    113 => {
      /* sig quit */
      sig = 3i32;
      current_block = 16913877891475873216;
    }
    49 => {
      /* sig usr1 */
      sig = 10i32;
      current_block = 16913877891475873216;
    }
    50 => {
      /* sig usr2 */
      sig = 12i32;
      current_block = 16913877891475873216;
    }
    _ => {
      current_block = 18435049525520518667;
    }
  }
  match current_block {
    16913877891475873216 => {
      if (*s).state as libc::c_int == 1i32 && custom(s, c) == 0 {
        kill((*s).pid, sig);
      }
      return 1i32;
    }
    9959655061420637642 =>
    /* FALLTHROUGH */
    /* sig term */
    {
      if (*s).state as libc::c_int == 1i32 {
        stopservice(s);
      }
    }
    _ => {}
  }
  return 1i32;
}
unsafe extern "C" fn open_control(mut f: *const libc::c_char, mut s: *mut svdir) {
  let mut st: stat = stat {
    st_dev: 0,
    st_ino: 0,
    st_nlink: 0,
    st_mode: 0,
    st_uid: 0,
    st_gid: 0,
    __pad0: 0,
    st_rdev: 0,
    st_size: 0,
    st_blksize: 0,
    st_blocks: 0,
    st_atim: timespec {
      tv_sec: 0,
      tv_nsec: 0,
    },
    st_mtim: timespec {
      tv_sec: 0,
      tv_nsec: 0,
    },
    st_ctim: timespec {
      tv_sec: 0,
      tv_nsec: 0,
    },
    __glibc_reserved: [0; 3],
  };
  mkfifo(f, 0o600i32 as __mode_t);
  if stat(f, &mut st) == -1i32 {
    fatal2_cannot(b"stat \x00" as *const u8 as *const libc::c_char, f);
  }
  if !(st.st_mode & 0o170000i32 as libc::c_uint == 0o10000i32 as libc::c_uint) {
    bb_error_msg_and_die(
      b"%s: fatal: %s exists but is not a fifo\x00" as *const u8 as *const libc::c_char,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).dir,
      f,
    );
  }
  (*s).fdcontrol = xopen(f, 0i32 | 0o4000i32);
  close_on_exec_on((*s).fdcontrol);
  (*s).fdcontrolwrite = xopen(f, 0o1i32 | 0o4000i32);
  close_on_exec_on((*s).fdcontrolwrite);
  update_status(s);
}
#[no_mangle]
pub unsafe extern "C" fn runsv_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut s: stat = stat {
    st_dev: 0,
    st_ino: 0,
    st_nlink: 0,
    st_mode: 0,
    st_uid: 0,
    st_gid: 0,
    __pad0: 0,
    st_rdev: 0,
    st_size: 0,
    st_blksize: 0,
    st_blocks: 0,
    st_atim: timespec {
      tv_sec: 0,
      tv_nsec: 0,
    },
    st_mtim: timespec {
      tv_sec: 0,
      tv_nsec: 0,
    },
    st_ctim: timespec {
      tv_sec: 0,
      tv_nsec: 0,
    },
    __glibc_reserved: [0; 3],
  };
  let mut fd: libc::c_int = 0;
  let mut r: libc::c_int = 0;
  let mut buf: [libc::c_char; 256] = [0; 256];
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).pidchanged = 1i32 as smallint;
  let ref mut fresh7 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).dir;
  *fresh7 = single_argv(argv);
  xpipe(
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .selfpipe
      .rd,
  );
  close_on_exec_on(
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .selfpipe
      .rd,
  );
  close_on_exec_on(
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .selfpipe
      .wr,
  );
  ndelay_on(
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .selfpipe
      .rd,
  );
  ndelay_on(
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .selfpipe
      .wr,
  );
  sig_block(17i32);
  bb_signals_recursive_norestart(
    1i32 << 17i32,
    Some(s_child as unsafe extern "C" fn(_: libc::c_int) -> ()),
  );
  sig_block(15i32);
  bb_signals_recursive_norestart(
    1i32 << 15i32,
    Some(s_term as unsafe extern "C" fn(_: libc::c_int) -> ()),
  );
  xchdir((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).dir);
  /* bss: svd[0].pid = 0; */
  /* otherwise already 0 (bss) */
  /* bss: svd[0].islog = 0; */
  /* bss: svd[1].pid = 0; */
  gettimeofday_ns(
    &mut (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .svd
      .as_mut_ptr()
      .offset(0))
    .start,
  ); /* for (;;) */
  if stat(b"down\x00" as *const u8 as *const libc::c_char, &mut s) != -1i32 {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svd[0].sd_want = 1i32 as smallint
  }
  if stat(b"log\x00" as *const u8 as *const libc::c_char, &mut s) == -1i32 {
    if *bb_errno != 2i32 {
      warn_cannot(b"stat ./log\x00" as *const u8 as *const libc::c_char);
    }
  } else if !(s.st_mode & 0o170000i32 as libc::c_uint == 0o40000i32 as libc::c_uint) {
    *bb_errno = 0i32;
    warn_cannot(b"stat log/down: log is not a directory\x00" as *const u8 as *const libc::c_char);
  } else {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).haslog = 1i32 as smallint;
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svd[1].state = 0i32 as smallint;
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svd[1].ctrl = 0i32 as smallint;
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svd[1].sd_want = 0i32 as smallint;
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svd[1].islog = 1i32 as smallint;
    gettimeofday_ns(
      &mut (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .svd
        .as_mut_ptr()
        .offset(1))
      .start,
    );
    if stat(b"log/down\x00" as *const u8 as *const libc::c_char, &mut s) != -1i32 {
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svd[1].sd_want = 1i32 as smallint
    }
    xpipe(
      &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .logpipe
        .rd,
    );
    close_on_exec_on(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .logpipe
        .rd,
    );
    close_on_exec_on(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .logpipe
        .wr,
    );
  }
  if mkdir(
    b"supervise\x00" as *const u8 as *const libc::c_char,
    0o700i32 as __mode_t,
  ) == -1i32
  {
    r = readlink(
      b"supervise\x00" as *const u8 as *const libc::c_char,
      buf.as_mut_ptr(),
      ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
    ) as libc::c_int;
    if r != -1i32 {
      if r as libc::c_ulong == ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong {
        fatal2x_cannot(
          b"readlink ./supervise\x00" as *const u8 as *const libc::c_char,
          b": name too long\x00" as *const u8 as *const libc::c_char,
        );
      }
      buf[r as usize] = 0i32 as libc::c_char;
      mkdir(buf.as_mut_ptr(), 0o700i32 as __mode_t);
    } else if *bb_errno != 2i32 && *bb_errno != 22i32 {
      fatal_cannot(b"readlink ./supervise\x00" as *const u8 as *const libc::c_char);
    }
  }
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svd[0].fdlock = xopen3(
    (b"log/supervise/lock\x00" as *const u8 as *const libc::c_char).offset(4),
    0o1i32 | 0o4000i32 | 0o2000i32 | 0o100i32,
    0o600i32,
  );
  if flock(
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svd[0].fdlock,
    2i32 | 4i32,
  ) == -1i32
  {
    fatal_cannot(b"lock supervise/lock\x00" as *const u8 as *const libc::c_char);
  }
  close_on_exec_on((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svd[0].fdlock);
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).haslog != 0 {
    if mkdir(
      b"log/supervise\x00" as *const u8 as *const libc::c_char,
      0o700i32 as __mode_t,
    ) == -1i32
    {
      r = readlink(
        b"log/supervise\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        256i32 as size_t,
      ) as libc::c_int;
      if r != -1i32 {
        if r == 256i32 {
          fatal2x_cannot(
            b"readlink ./log/supervise\x00" as *const u8 as *const libc::c_char,
            b": name too long\x00" as *const u8 as *const libc::c_char,
          );
        }
        buf[r as usize] = 0i32 as libc::c_char;
        fd = xopen(
          b".\x00" as *const u8 as *const libc::c_char,
          0i32 | 0o4000i32,
        );
        xchdir(b"./log\x00" as *const u8 as *const libc::c_char);
        mkdir(buf.as_mut_ptr(), 0o700i32 as __mode_t);
        if fchdir(fd) == -1i32 {
          fatal_cannot(b"change back to service directory\x00" as *const u8 as *const libc::c_char);
        }
        close(fd);
      } else if *bb_errno != 2i32 && *bb_errno != 22i32 {
        fatal_cannot(b"readlink ./log/supervise\x00" as *const u8 as *const libc::c_char);
      }
    }
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svd[1].fdlock = xopen3(
      b"log/supervise/lock\x00" as *const u8 as *const libc::c_char,
      0o1i32 | 0o4000i32 | 0o2000i32 | 0o100i32,
      0o600i32,
    );
    if flock(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svd[1].fdlock,
      2i32,
    ) == -1i32
    {
      fatal_cannot(b"lock log/supervise/lock\x00" as *const u8 as *const libc::c_char);
    }
    close_on_exec_on((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svd[1].fdlock);
  }
  open_control(
    (b"log/supervise/control\x00" as *const u8 as *const libc::c_char).offset(4),
    &mut *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .svd
      .as_mut_ptr()
      .offset(0),
  );
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).haslog != 0 {
    open_control(
      b"log/supervise/control\x00" as *const u8 as *const libc::c_char,
      &mut *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .svd
        .as_mut_ptr()
        .offset(1),
    );
  }
  mkfifo(
    (b"log/supervise/ok\x00" as *const u8 as *const libc::c_char).offset(4),
    0o600i32 as __mode_t,
  );
  fd = xopen(
    (b"log/supervise/ok\x00" as *const u8 as *const libc::c_char).offset(4),
    0i32 | 0o4000i32,
  );
  close_on_exec_on(fd);
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).haslog != 0 {
    mkfifo(
      b"log/supervise/ok\x00" as *const u8 as *const libc::c_char,
      0o600i32 as __mode_t,
    );
    fd = xopen(
      b"log/supervise/ok\x00" as *const u8 as *const libc::c_char,
      0i32 | 0o4000i32,
    );
    close_on_exec_on(fd);
  }
  loop {
    let mut x: [pollfd; 3] = [pollfd {
      fd: 0,
      events: 0,
      revents: 0,
    }; 3];
    let mut deadline: libc::c_uint = 0;
    let mut ch: libc::c_char = 0;
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).haslog != 0 {
      if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svd[1].pid == 0
        && (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svd[1].sd_want as libc::c_int == 0i32
      {
        startservice(
          &mut *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
            .svd
            .as_mut_ptr()
            .offset(1),
        );
      }
    }
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svd[0].pid == 0 {
      if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svd[0].sd_want as libc::c_int == 0i32
        || (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svd[0].state as libc::c_int == 2i32
      {
        startservice(
          &mut *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
            .svd
            .as_mut_ptr()
            .offset(0),
        );
      }
    }
    x[0].fd = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .selfpipe
      .rd;
    x[0].events = 0x1i32 as libc::c_short;
    x[1].fd = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svd[0].fdcontrol;
    x[1].events = 0x1i32 as libc::c_short;
    /* x[2] is used only if haslog == 1 */
    x[2].fd = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svd[1].fdcontrol; /* for (;;) */
    x[2].events = 0x1i32 as libc::c_short;
    sig_unblock(15i32);
    sig_unblock(17i32);
    poll(
      x.as_mut_ptr(),
      (2i32 + (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).haslog as libc::c_int) as nfds_t,
      3600i32 * 1000i32,
    );
    sig_block(15i32);
    sig_block(17i32);
    while read(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .selfpipe
        .rd,
      &mut ch as *mut libc::c_char as *mut libc::c_void,
      1i32 as size_t,
    ) == 1i32 as libc::c_long
    {}
    loop {
      let mut child: pid_t = 0;
      let mut wstat: libc::c_int = 0;
      child = wait_any_nohang(&mut wstat);
      if child == 0 {
        break;
      }
      if child == -1i32 && *bb_errno != 4i32 {
        break;
      }
      if child == (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svd[0].pid {
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svd[0].wstat = wstat;
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svd[0].pid = 0i32;
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).pidchanged = 1i32 as smallint;
        let ref mut fresh8 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svd[0].ctrl;
        *fresh8 = (*fresh8 as libc::c_int & !1i32) as smallint;
        if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svd[0].state as libc::c_int != 2i32 {
          fd = open(
            b"finish\x00" as *const u8 as *const libc::c_char,
            0i32 | 0o4000i32,
          );
          if fd != -1i32 {
            close(fd);
            (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svd[0].state = 2i32 as smallint;
            update_status(
              &mut *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                .svd
                .as_mut_ptr()
                .offset(0),
            );
            continue;
          }
        }
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svd[0].state = 0i32 as smallint;
        deadline = ((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svd[0]
          .start
          .tv_sec
          + 1i32 as libc::c_long) as libc::c_uint;
        gettimeofday_ns(
          &mut (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
            .svd
            .as_mut_ptr()
            .offset(0))
          .start,
        );
        update_status(
          &mut *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
            .svd
            .as_mut_ptr()
            .offset(0),
        );
        if deadline.wrapping_sub(
          (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svd[0]
            .start
            .tv_sec as libc::c_uint,
        ) as libc::c_int
          > 0i32
        {
          sleep(1i32 as libc::c_uint);
        }
      }
      if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).haslog != 0 {
        if child == (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svd[1].pid {
          (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svd[0].wstat = wstat;
          (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svd[1].pid = 0i32;
          (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).pidchanged = 1i32 as smallint;
          (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svd[1].state = 0i32 as smallint;
          let ref mut fresh9 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svd[1].ctrl;
          *fresh9 = (*fresh9 as libc::c_int & !1i32) as smallint;
          deadline = ((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svd[1]
            .start
            .tv_sec
            + 1i32 as libc::c_long) as libc::c_uint;
          gettimeofday_ns(
            &mut (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
              .svd
              .as_mut_ptr()
              .offset(1))
            .start,
          );
          update_status(
            &mut *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
              .svd
              .as_mut_ptr()
              .offset(1),
          );
          if deadline.wrapping_sub(
            (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svd[1]
              .start
              .tv_sec as libc::c_uint,
          ) as libc::c_int
            > 0i32
          {
            sleep(1i32 as libc::c_uint);
          }
        }
      }
    }
    if read(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svd[0].fdcontrol,
      &mut ch as *mut libc::c_char as *mut libc::c_void,
      1i32 as size_t,
    ) == 1i32 as libc::c_long
    {
      ctrl(
        &mut *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
          .svd
          .as_mut_ptr()
          .offset(0),
        ch,
      );
    }
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).haslog != 0 {
      if read(
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svd[1].fdcontrol,
        &mut ch as *mut libc::c_char as *mut libc::c_void,
        1i32 as size_t,
      ) == 1i32 as libc::c_long
      {
        ctrl(
          &mut *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
            .svd
            .as_mut_ptr()
            .offset(1),
          ch,
        );
      }
    }
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).sigterm != 0 {
      ctrl(
        &mut *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
          .svd
          .as_mut_ptr()
          .offset(0),
        'x' as i32 as libc::c_char,
      );
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).sigterm = 0i32 as smallint
    }
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svd[0].sd_want as libc::c_int == 2i32
      && (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svd[0].state as libc::c_int == 0i32
    {
      if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svd[1].pid == 0i32 {
        _exit(0i32);
      }
      if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svd[1].sd_want as libc::c_int != 2i32 {
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svd[1].sd_want = 2i32 as smallint;
        /* stopservice(&svd[1]); */
        update_status(
          &mut *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
            .svd
            .as_mut_ptr()
            .offset(1),
        );
        close(
          (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
            .logpipe
            .wr,
        );
        close(
          (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
            .logpipe
            .rd,
        );
      }
    }
  }
}
