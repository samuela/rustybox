use crate::librb::smallint;
use crate::libbb::ptr_to_globals::bb_errno;
use libc;
use libc::chdir;
use libc::closedir;
use libc::getpid;
use libc::ino_t;
use libc::kill;
use libc::open;
use libc::opendir;
use libc::pid_t;
use libc::readdir;
use libc::setsid;
use libc::sleep;
use libc::stat;
use libc::time;
use libc::useconds_t;
extern "C" {

  #[no_mangle]
  fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  fn vfork() -> libc::c_int;

  #[no_mangle]
  fn execlp(__file: *const libc::c_char, __arg: *const libc::c_char, _: ...) -> libc::c_int;

  #[no_mangle]
  fn fchdir(__fd: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn usleep(__useconds: useconds_t) -> libc::c_int;

  #[no_mangle]
  fn wait(__stat_loc: *mut libc::c_int) -> pid_t;



  #[no_mangle]
  fn monotonic_sec() -> libc::c_uint;

  #[no_mangle]
  fn close_on_exec_on(fd: libc::c_int);

  #[no_mangle]
  fn bb_signals(sigs: libc::c_int, f: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>);

  #[no_mangle]
  static mut bb_got_signal: smallint;

  #[no_mangle]
  fn record_signo(signo: libc::c_int);

  #[no_mangle]
  fn spawn(argv: *mut *mut libc::c_char) -> pid_t;

  #[no_mangle]
  fn wait_any_nohang(wstat: *mut libc::c_int) -> pid_t;

  #[no_mangle]
  static mut option_mask32: u32;

  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;

  #[no_mangle]
  fn utoa(n: libc::c_uint) -> *mut libc::c_char;

  #[no_mangle]
  fn bb_perror_msg_and_die(s: *const libc::c_char, _: ...) -> !;

  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);

  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
}

use libc::dirent;
use libc::time_t;
use libc::DIR;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub sv: *mut service,
  pub svdir: *mut libc::c_char,
  pub svnum: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct service {
  pub ino: ino_t,
  pub pid: pid_t,
  pub isgone: smallint,
}
unsafe extern "C" fn fatal2_cannot(mut m1: *const libc::c_char, mut m2: *const libc::c_char) {
  bb_perror_msg_and_die(
    b"%s: fatal: can\'t %s%s\x00" as *const u8 as *const libc::c_char,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svdir,
    m1,
    m2,
  );
  /* was exiting 100 */
}
unsafe extern "C" fn warn3x(
  mut m1: *const libc::c_char,
  mut m2: *const libc::c_char,
  mut m3: *const libc::c_char,
) {
  bb_error_msg(
    b"%s: warning: %s%s%s\x00" as *const u8 as *const libc::c_char,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svdir,
    m1,
    m2,
    m3,
  );
}
unsafe extern "C" fn warn2_cannot(mut m1: *const libc::c_char, mut m2: *const libc::c_char) {
  warn3x(b"can\'t \x00" as *const u8 as *const libc::c_char, m1, m2);
}
/* inlining + vfork -> bigger code */
#[inline(never)]
unsafe extern "C" fn runsv(mut name: *const libc::c_char) -> pid_t {
  let mut pid: pid_t = 0;
  /* If we got signaled, stop spawning children at once! */
  if bb_got_signal != 0 {
    return 0i32;
  }
  pid = vfork();
  if pid == -1i32 {
    warn2_cannot(
      b"vfork\x00" as *const u8 as *const libc::c_char,
      b"\x00" as *const u8 as *const libc::c_char,
    );
    return 0i32;
  }
  if pid == 0i32 {
    /* child */
    if option_mask32 & 1i32 as libc::c_uint != 0 {
      /* -P option? */
      setsid();
    }
    /* man execv:
     * "Signals set to be caught by the calling process image
     *  shall be set to the default action in the new process image."
     * Therefore, we do not need this: */
    execlp(
      b"runsv\x00" as *const u8 as *const libc::c_char,
      b"runsv\x00" as *const u8 as *const libc::c_char,
      name,
      0 as *mut libc::c_void as *mut libc::c_char,
    );
    fatal2_cannot(
      b"start runsv \x00" as *const u8 as *const libc::c_char,
      name,
    );
  }
  return pid;
}
/* gcc 4.3.0 does better with NOINLINE */
#[inline(never)]
unsafe extern "C" fn do_rescan() -> libc::c_int {
  let mut dir: *mut DIR = 0 as *mut DIR;
  let mut d: *mut dirent = 0 as *mut dirent;
  let mut i: libc::c_int = 0;
  let mut s: stat = std::mem::zeroed();
  let mut need_rescan: libc::c_int = 0i32;
  dir = opendir(b".\x00" as *const u8 as *const libc::c_char);
  if dir.is_null() {
    warn2_cannot(
      b"open directory \x00" as *const u8 as *const libc::c_char,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svdir,
    );
    return 1i32;
    /* need to rescan again soon */
  }
  i = 0i32;
  while i < (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svnum {
    (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .sv
      .offset(i as isize))
    .isgone = 1i32 as smallint;
    i += 1
  }
  let mut svnew: *mut service = 0 as *mut service;
  let mut current_block_20: u64;
  's_55: loop {
    *bb_errno = 0i32;
    d = readdir(dir);
    if d.is_null() {
      break;
    }
    if (*d).d_name[0] as libc::c_int == '.' as i32 {
      continue;
    }
    if stat((*d).d_name.as_mut_ptr(), &mut s) == -1i32 {
      warn2_cannot(
        b"stat \x00" as *const u8 as *const libc::c_char,
        (*d).d_name.as_mut_ptr(),
      );
    } else {
      if !(s.st_mode & 0o170000i32 as libc::c_uint == 0o40000i32 as libc::c_uint) {
        continue;
      }
      /* Do we have this service listed already? */
      i = 0i32; /* "we still see you" */
      loop {
        if !(i < (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svnum) {
          current_block_20 = 7172762164747879670;
          break;
        }
        if (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
          .sv
          .offset(i as isize))
        .ino
          == s.st_ino
        {
          if (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
            .sv
            .offset(i as isize))
          .pid
            == 0i32
          {
            current_block_20 = 14858922589835423217;
            break;
          }
          (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
            .sv
            .offset(i as isize))
          .isgone = 0i32 as smallint;
          continue 's_55;
        } else {
          i += 1
        }
      }
      match current_block_20 {
        7172762164747879670 =>
        /* Not found, make new service */
        {
          svnew = realloc(
            (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).sv as *mut libc::c_void,
            ((i + 1i32) as libc::c_ulong)
              .wrapping_mul(::std::mem::size_of::<service>() as libc::c_ulong),
          ) as *mut service;
          if svnew.is_null() {
            warn2_cannot(
              b"start runsv \x00" as *const u8 as *const libc::c_char,
              (*d).d_name.as_mut_ptr(),
            );
            need_rescan = 1i32;
            continue;
          } else {
            let ref mut fresh0 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).sv;
            *fresh0 = svnew;
            let ref mut fresh1 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svnum;
            *fresh1 += 1;
            (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
              .sv
              .offset(i as isize))
            .ino = s.st_ino
          }
        }
        _ => {}
      }
      /* restart if it has died */
      (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .sv
        .offset(i as isize))
      .pid = runsv((*d).d_name.as_mut_ptr());
      (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .sv
        .offset(i as isize))
      .isgone = 0i32 as smallint
    }
  }
  i = *bb_errno;
  closedir(dir);
  if i != 0 {
    /* readdir failed */
    warn2_cannot(
      b"read directory \x00" as *const u8 as *const libc::c_char,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svdir,
    );
    return 1i32;
    /* need to rescan again soon */
  }
  /* Send SIGTERM to runsv whose directories
   * were no longer found (-> must have been removed) */
  i = 0i32;
  while i < (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svnum {
    if !((*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .sv
      .offset(i as isize))
    .isgone
      == 0)
    {
      if (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .sv
        .offset(i as isize))
      .pid
        != 0
      {
        kill(
          (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
            .sv
            .offset(i as isize))
          .pid,
          15i32,
        );
      }
      let ref mut fresh2 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svnum;
      *fresh2 -= 1;
      *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .sv
        .offset(i as isize) = *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .sv
        .offset((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svnum as isize);
      i -= 1
    }
    i += 1
    /* so that we don't skip new sv[i] (bug was here!) */
  } /* for gcc */
  return need_rescan; /* for gcc */
}
#[no_mangle]
pub unsafe extern "C" fn runsvdir_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut s: stat = std::mem::zeroed();
  let mut last_dev: libc::dev_t = 0;
  last_dev = last_dev;
  let mut last_ino: ino_t = 0;
  last_ino = last_ino;
  let mut last_mtime: time_t = 0;
  let mut curdir: libc::c_int = 0;
  let mut stampcheck: libc::c_uint = 0;
  let mut i: libc::c_int = 0;
  let mut need_rescan: libc::c_int = 0;
  let mut i_am_init: bool = false;
  let mut opt_s_argv: [*mut libc::c_char; 3] = [0 as *mut libc::c_char; 3];
  opt_s_argv[0] = 0 as *mut libc::c_char;
  opt_s_argv[2] = 0 as *mut libc::c_char;
  getopt32(
    argv,
    b"^Ps:\x00-1\x00" as *const u8 as *const libc::c_char,
    &mut *opt_s_argv.as_mut_ptr().offset(0) as *mut *mut libc::c_char,
  );
  argv = argv.offset(optind as isize);
  i_am_init = getpid() == 1i32;
  bb_signals(
    0i32
      | 1i32 << 15i32
      | 1i32 << 1i32
      | (if i_am_init as libc::c_int != 0 {
        (1i32 << 10i32 | 1i32 << 12i32) | 1i32 << 2i32
      } else {
        0i32
      }),
    Some(record_signo as unsafe extern "C" fn(_: libc::c_int) -> ()),
  );
  let fresh3 = argv;
  argv = argv.offset(1);
  let ref mut fresh4 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svdir;
  *fresh4 = *fresh3;
  curdir = open(
    b".\x00" as *const u8 as *const libc::c_char,
    0i32 | 0o4000i32,
  );
  if curdir == -1i32 {
    fatal2_cannot(
      b"open current directory\x00" as *const u8 as *const libc::c_char,
      b"\x00" as *const u8 as *const libc::c_char,
    );
  }
  close_on_exec_on(curdir);
  stampcheck = monotonic_sec();
  need_rescan = 1i32;
  last_mtime = 0i32 as time_t;
  loop {
    let mut now: libc::c_uint = 0;
    let mut sig: libc::c_uint = 0;
    loop
    /* init continues to monitor services forever */
    /* collect children */
    {
      let mut pid: pid_t = wait_any_nohang(0 as *mut libc::c_int);
      if pid <= 0i32 {
        break;
      }
      i = 0i32;
      while i < (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svnum {
        if pid
          == (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
            .sv
            .offset(i as isize))
          .pid
        {
          /* runsv has died */
          (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
            .sv
            .offset(i as isize))
          .pid = 0i32;
          need_rescan = 1i32
        }
        i += 1
      }
    }
    now = monotonic_sec();
    if now.wrapping_sub(stampcheck) as libc::c_int >= 0i32 {
      /* wait at least a second */
      stampcheck = now.wrapping_add(1i32 as libc::c_uint);
      if stat(
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svdir,
        &mut s,
      ) != -1i32
      {
        if need_rescan != 0
          || s.st_mtime != last_mtime
          || s.st_ino != last_ino
          || s.st_dev != last_dev
        {
          /* svdir modified */
          if chdir((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svdir) != -1i32 {
            last_mtime = s.st_mtime;
            last_dev = s.st_dev;
            last_ino = s.st_ino;
            /* if the svdir changed this very second, wait until the
             * next second, because we won't be able to detect more
             * changes within this second */
            while time(0 as *mut time_t) == last_mtime {
              usleep(100000i32 as useconds_t);
            }
            need_rescan = do_rescan();
            while fchdir(curdir) == -1i32 {
              warn2_cannot(
                b"change directory, pausing\x00" as *const u8 as *const libc::c_char,
                b"\x00" as *const u8 as *const libc::c_char,
              );
              sleep(5i32 as libc::c_uint);
            }
          } else {
            warn2_cannot(
              b"change directory to \x00" as *const u8 as *const libc::c_char,
              (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svdir,
            );
          }
        }
      } else {
        warn2_cannot(
          b"stat \x00" as *const u8 as *const libc::c_char,
          (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svdir,
        );
      }
    }
    let mut deadline: libc::c_uint = if need_rescan != 0 { 1i32 } else { 5i32 } as libc::c_uint;
    sleep(deadline);
    sig = bb_got_signal as libc::c_uint;
    if sig == 0 {
      continue;
    }
    bb_got_signal = 0i32 as smallint;
    /* -s SCRIPT: useful if we are init.
     * In this case typically script never returns,
     * it halts/powers off/reboots the system. */
    if !opt_s_argv[0].is_null() {
      let mut pid_0: pid_t = 0;
      /* Single parameter: signal# */
      opt_s_argv[1] = utoa(sig);
      pid_0 = spawn(opt_s_argv.as_mut_ptr());
      if pid_0 > 0i32 {
        /* Remembering to wait for _any_ children,
         * not just pid */
        while wait(0 as *mut libc::c_int) != pid_0 {}
      }
    }
    if sig == 1i32 as libc::c_uint {
      i = 0i32;
      while i < (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).svnum {
        if (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
          .sv
          .offset(i as isize))
        .pid
          != 0
        {
          kill(
            (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
              .sv
              .offset(i as isize))
            .pid,
            15i32,
          );
        }
        i += 1
      }
    }
    if !i_am_init {
      return if 1i32 as libc::c_uint == sig {
        111i32
      } else {
        0i32
      };
    }
  }
  /* SIGHUP or SIGTERM (or SIGUSRn if we are init) */
  /* Exit unless we are init */
  /* for (;;) */
}
