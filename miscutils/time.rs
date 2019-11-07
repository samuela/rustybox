use crate::librb::__syscall_slong_t;
use crate::librb::signal::__sighandler_t;
use libc;
use libc::pid_t;
use libc::timeval;

extern "C" {
  #[no_mangle]
  fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn vfork() -> libc::c_int;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn getpagesize() -> libc::c_int;
  #[no_mangle]
  fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
  #[no_mangle]
  fn wait3(__stat_loc: *mut libc::c_int, __options: libc::c_int, __usage: *mut rusage) -> pid_t;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn monotonic_ms() -> libc::c_ulonglong;
  #[no_mangle]
  fn close_on_exec_on(fd: libc::c_int);
  #[no_mangle]
  fn xdup2(_: libc::c_int, _: libc::c_int);
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn fflush_stdout_and_exit(retval: libc::c_int) -> !;
  #[no_mangle]
  fn BB_EXECVP_or_die(argv: *mut *mut libc::c_char) -> !;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct rusage {
  pub ru_utime: timeval,
  pub ru_stime: timeval,
  pub c2rust_unnamed: C2RustUnnamed_12,
  pub c2rust_unnamed_0: C2RustUnnamed_11,
  pub c2rust_unnamed_1: C2RustUnnamed_10,
  pub c2rust_unnamed_2: C2RustUnnamed_9,
  pub c2rust_unnamed_3: C2RustUnnamed_8,
  pub c2rust_unnamed_4: C2RustUnnamed_7,
  pub c2rust_unnamed_5: C2RustUnnamed_6,
  pub c2rust_unnamed_6: C2RustUnnamed_5,
  pub c2rust_unnamed_7: C2RustUnnamed_4,
  pub c2rust_unnamed_8: C2RustUnnamed_3,
  pub c2rust_unnamed_9: C2RustUnnamed_2,
  pub c2rust_unnamed_10: C2RustUnnamed_1,
  pub c2rust_unnamed_11: C2RustUnnamed_0,
  pub c2rust_unnamed_12: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
  pub ru_nivcsw: libc::c_long,
  pub __ru_nivcsw_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
  pub ru_nvcsw: libc::c_long,
  pub __ru_nvcsw_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
  pub ru_nsignals: libc::c_long,
  pub __ru_nsignals_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
  pub ru_msgrcv: libc::c_long,
  pub __ru_msgrcv_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
  pub ru_msgsnd: libc::c_long,
  pub __ru_msgsnd_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
  pub ru_oublock: libc::c_long,
  pub __ru_oublock_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
  pub ru_inblock: libc::c_long,
  pub __ru_inblock_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
  pub ru_nswap: libc::c_long,
  pub __ru_nswap_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
  pub ru_majflt: libc::c_long,
  pub __ru_majflt_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
  pub ru_minflt: libc::c_long,
  pub __ru_minflt_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
  pub ru_isrss: libc::c_long,
  pub __ru_isrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
  pub ru_idrss: libc::c_long,
  pub __ru_idrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
  pub ru_ixrss: libc::c_long,
  pub __ru_ixrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
  pub ru_maxrss: libc::c_long,
  pub __ru_maxrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct resource_t {
  pub waitstatus: libc::c_int,
  pub ru: rusage,
  pub elapsed_ms: libc::c_uint,
}
pub const OPT_a: C2RustUnnamed_13 = 4;
pub const OPT_o: C2RustUnnamed_13 = 8;
pub const OPT_p: C2RustUnnamed_13 = 2;
pub const OPT_v: C2RustUnnamed_13 = 1;
pub type C2RustUnnamed_13 = libc::c_uint;
pub const OPT_f: C2RustUnnamed_13 = 16;
/* msec = milliseconds = 1/1,000 (1*10e-3) second.
usec = microseconds = 1/1,000,000 (1*10e-6) second.  */
static mut default_format: [libc::c_char; 23] = [
  114, 101, 97, 108, 9, 37, 69, 10, 117, 115, 101, 114, 9, 37, 117, 10, 115, 121, 115, 9, 37, 84, 0,
];
/* The output format for the -p option .*/
static mut posix_format: [libc::c_char; 23] = [
  114, 101, 97, 108, 32, 37, 101, 10, 117, 115, 101, 114, 32, 37, 85, 10, 115, 121, 115, 32, 37,
  83, 0,
];
/* Format string for printing all statistics verbosely.
Keep this output to 24 lines so users on terminals can see it all.*/
static mut long_format: [libc::c_char; 715] = [
  9, 67, 111, 109, 109, 97, 110, 100, 32, 98, 101, 105, 110, 103, 32, 116, 105, 109, 101, 100, 58,
  32, 34, 37, 67, 34, 10, 9, 85, 115, 101, 114, 32, 116, 105, 109, 101, 32, 40, 115, 101, 99, 111,
  110, 100, 115, 41, 58, 32, 37, 85, 10, 9, 83, 121, 115, 116, 101, 109, 32, 116, 105, 109, 101,
  32, 40, 115, 101, 99, 111, 110, 100, 115, 41, 58, 32, 37, 83, 10, 9, 80, 101, 114, 99, 101, 110,
  116, 32, 111, 102, 32, 67, 80, 85, 32, 116, 104, 105, 115, 32, 106, 111, 98, 32, 103, 111, 116,
  58, 32, 37, 80, 10, 9, 69, 108, 97, 112, 115, 101, 100, 32, 40, 119, 97, 108, 108, 32, 99, 108,
  111, 99, 107, 41, 32, 116, 105, 109, 101, 32, 40, 104, 58, 109, 109, 58, 115, 115, 32, 111, 114,
  32, 109, 58, 115, 115, 41, 58, 32, 37, 69, 10, 9, 65, 118, 101, 114, 97, 103, 101, 32, 115, 104,
  97, 114, 101, 100, 32, 116, 101, 120, 116, 32, 115, 105, 122, 101, 32, 40, 107, 98, 121, 116,
  101, 115, 41, 58, 32, 37, 88, 10, 9, 65, 118, 101, 114, 97, 103, 101, 32, 117, 110, 115, 104, 97,
  114, 101, 100, 32, 100, 97, 116, 97, 32, 115, 105, 122, 101, 32, 40, 107, 98, 121, 116, 101, 115,
  41, 58, 32, 37, 68, 10, 9, 65, 118, 101, 114, 97, 103, 101, 32, 115, 116, 97, 99, 107, 32, 115,
  105, 122, 101, 32, 40, 107, 98, 121, 116, 101, 115, 41, 58, 32, 37, 112, 10, 9, 65, 118, 101,
  114, 97, 103, 101, 32, 116, 111, 116, 97, 108, 32, 115, 105, 122, 101, 32, 40, 107, 98, 121, 116,
  101, 115, 41, 58, 32, 37, 75, 10, 9, 77, 97, 120, 105, 109, 117, 109, 32, 114, 101, 115, 105,
  100, 101, 110, 116, 32, 115, 101, 116, 32, 115, 105, 122, 101, 32, 40, 107, 98, 121, 116, 101,
  115, 41, 58, 32, 37, 77, 10, 9, 65, 118, 101, 114, 97, 103, 101, 32, 114, 101, 115, 105, 100,
  101, 110, 116, 32, 115, 101, 116, 32, 115, 105, 122, 101, 32, 40, 107, 98, 121, 116, 101, 115,
  41, 58, 32, 37, 116, 10, 9, 77, 97, 106, 111, 114, 32, 40, 114, 101, 113, 117, 105, 114, 105,
  110, 103, 32, 73, 47, 79, 41, 32, 112, 97, 103, 101, 32, 102, 97, 117, 108, 116, 115, 58, 32, 37,
  70, 10, 9, 77, 105, 110, 111, 114, 32, 40, 114, 101, 99, 108, 97, 105, 109, 105, 110, 103, 32,
  97, 32, 102, 114, 97, 109, 101, 41, 32, 112, 97, 103, 101, 32, 102, 97, 117, 108, 116, 115, 58,
  32, 37, 82, 10, 9, 86, 111, 108, 117, 110, 116, 97, 114, 121, 32, 99, 111, 110, 116, 101, 120,
  116, 32, 115, 119, 105, 116, 99, 104, 101, 115, 58, 32, 37, 119, 10, 9, 73, 110, 118, 111, 108,
  117, 110, 116, 97, 114, 121, 32, 99, 111, 110, 116, 101, 120, 116, 32, 115, 119, 105, 116, 99,
  104, 101, 115, 58, 32, 37, 99, 10, 9, 83, 119, 97, 112, 115, 58, 32, 37, 87, 10, 9, 70, 105, 108,
  101, 32, 115, 121, 115, 116, 101, 109, 32, 105, 110, 112, 117, 116, 115, 58, 32, 37, 73, 10, 9,
  70, 105, 108, 101, 32, 115, 121, 115, 116, 101, 109, 32, 111, 117, 116, 112, 117, 116, 115, 58,
  32, 37, 79, 10, 9, 83, 111, 99, 107, 101, 116, 32, 109, 101, 115, 115, 97, 103, 101, 115, 32,
  115, 101, 110, 116, 58, 32, 37, 115, 10, 9, 83, 111, 99, 107, 101, 116, 32, 109, 101, 115, 115,
  97, 103, 101, 115, 32, 114, 101, 99, 101, 105, 118, 101, 100, 58, 32, 37, 114, 10, 9, 83, 105,
  103, 110, 97, 108, 115, 32, 100, 101, 108, 105, 118, 101, 114, 101, 100, 58, 32, 37, 107, 10, 9,
  80, 97, 103, 101, 32, 115, 105, 122, 101, 32, 40, 98, 121, 116, 101, 115, 41, 58, 32, 37, 90, 10,
  9, 69, 120, 105, 116, 32, 115, 116, 97, 116, 117, 115, 58, 32, 37, 120, 0,
];
/* Wait for and fill in data on child process PID.
Return 0 on error, 1 if ok.  */
/* pid_t is short on BSDI, so don't try to promote it.  */
unsafe extern "C" fn resuse_end(mut pid: pid_t, mut resp: *mut resource_t) {
  let mut caught: pid_t = 0;
  loop
  /* Ignore signals, but don't ignore the children.  When wait3
   * returns the child process, set the time the command finished. */
  {
    caught = wait3(&mut (*resp).waitstatus, 0i32, &mut (*resp).ru);
    if !(caught != pid) {
      break;
    }
    if caught == -1i32 && *bb_errno != 4i32 {
      bb_simple_perror_msg(b"wait\x00" as *const u8 as *const libc::c_char);
      return;
    }
  }
  (*resp).elapsed_ms =
    monotonic_ms().wrapping_sub((*resp).elapsed_ms as libc::c_ulonglong) as libc::c_uint;
}
unsafe extern "C" fn printargv(mut argv: *const *mut libc::c_char) {
  let mut fmt: *const libc::c_char = (b" %s\x00" as *const u8 as *const libc::c_char).offset(1);
  loop {
    printf(fmt, *argv);
    fmt = b" %s\x00" as *const u8 as *const libc::c_char;
    argv = argv.offset(1);
    if (*argv).is_null() {
      break;
    }
  }
}
/* Return the number of kilobytes corresponding to a number of pages PAGES.
(Actually, we use it to convert pages*ticks into kilobytes*ticks.)

Try to do arithmetic so that the risk of overflow errors is minimized.
This is funky since the pagesize could be less than 1K.
Note: Some machines express getrusage statistics in terms of K,
others in terms of pages.  */
unsafe extern "C" fn ptok(pagesize: libc::c_uint, pages: libc::c_ulong) -> libc::c_ulong {
  let mut tmp: libc::c_ulong = 0;
  /* Conversion.  */
  if pages > (9223372036854775807i64 / pagesize as libc::c_long) as libc::c_ulong {
    /* Could overflow.  */
    tmp = pages.wrapping_div(1024i32 as libc::c_ulong);
    return tmp.wrapping_mul(pagesize as libc::c_ulong); /* Smaller first, */
    /* then larger.  */
  }
  /* Could underflow.  */
  tmp = pages.wrapping_mul(pagesize as libc::c_ulong); /* Larger first, */
  return tmp.wrapping_div(1024i32 as libc::c_ulong);
  /* then smaller.  */
}
unsafe extern "C" fn summarize(
  mut fmt: *const libc::c_char,
  mut command: *mut *mut libc::c_char,
  mut resp: *mut resource_t,
) {
  let mut vv_ms: libc::c_uint = 0; /* Elapsed virtual (CPU) milliseconds */
  let mut cpu_ticks: libc::c_uint = 0; /* Same, in "CPU ticks" */
  let mut pagesize: libc::c_uint = getpagesize() as libc::c_uint;
  /* Impossible: we do not use WUNTRACED flag in wait()...
  if (WIFSTOPPED(resp->waitstatus))
    printf("Command stopped by signal %u\n",
        WSTOPSIG(resp->waitstatus));
  else */
  if (((*resp).waitstatus & 0x7fi32) + 1i32) as libc::c_schar as libc::c_int >> 1i32 > 0i32 {
    printf(
      b"Command terminated by signal %u\n\x00" as *const u8 as *const libc::c_char,
      (*resp).waitstatus & 0x7fi32,
    );
  } else if (*resp).waitstatus & 0x7fi32 == 0i32 && ((*resp).waitstatus & 0xff00i32) >> 8i32 != 0 {
    printf(
      b"Command exited with non-zero status %u\n\x00" as *const u8 as *const libc::c_char,
      ((*resp).waitstatus & 0xff00i32) >> 8i32,
    );
  }
  vv_ms = (((*resp).ru.ru_utime.tv_sec + (*resp).ru.ru_stime.tv_sec) * 1000i32 as libc::c_long
    + ((*resp).ru.ru_utime.tv_usec + (*resp).ru.ru_stime.tv_usec) / 1000i32 as libc::c_long)
    as libc::c_uint;
  /* 1000 is exactly divisible by TICKS_PER_SEC (typical) */
  cpu_ticks = vv_ms.wrapping_div((1000i32 / 100i32) as libc::c_uint); /* we divide by it, must be nonzero */
  if cpu_ticks == 0 {
    cpu_ticks = 1i32 as libc::c_uint
  }
  while *fmt != 0 {
    /* Handle leading literal part */
    let mut n: libc::c_int =
      strcspn(fmt, b"%\\\x00" as *const u8 as *const libc::c_char) as libc::c_int;
    if n != 0 {
      printf(b"%.*s\x00" as *const u8 as *const libc::c_char, n, fmt);
      fmt = fmt.offset(n as isize)
    } else {
      match *fmt as libc::c_int {
        37 => {
          fmt = fmt.offset(1);
          match *fmt as libc::c_int {
            67 => {
              /* The command that got timed.  */
              printargv(command);
            }
            68 => {
              /* Average unshared data size.  */
              printf(
                b"%lu\x00" as *const u8 as *const libc::c_char,
                ptok(
                  pagesize,
                  (*resp).ru.c2rust_unnamed_1.ru_idrss as libc::c_ulong,
                )
                .wrapping_add(ptok(
                  pagesize,
                  (*resp).ru.c2rust_unnamed_2.ru_isrss as libc::c_ulong,
                ))
                .wrapping_div(cpu_ticks as libc::c_ulong),
              );
            }
            69 => {
              /* Elapsed real (wall clock) time.  */
              let mut seconds: libc::c_uint =
                (*resp).elapsed_ms.wrapping_div(1000i32 as libc::c_uint);
              if seconds >= 3600i32 as libc::c_uint {
                /* One hour -> h:m:s.  */
                printf(
                  b"%uh %um %02us\x00" as *const u8 as *const libc::c_char,
                  seconds.wrapping_div(3600i32 as libc::c_uint),
                  seconds
                    .wrapping_rem(3600i32 as libc::c_uint)
                    .wrapping_div(60i32 as libc::c_uint),
                  seconds.wrapping_rem(60i32 as libc::c_uint),
                );
              } else {
                printf(
                  b"%um %u.%02us\x00" as *const u8 as *const libc::c_char,
                  seconds.wrapping_div(60i32 as libc::c_uint),
                  seconds.wrapping_rem(60i32 as libc::c_uint),
                  (*resp)
                    .elapsed_ms
                    .wrapping_div(10i32 as libc::c_uint)
                    .wrapping_rem(100i32 as libc::c_uint),
                );
              }
            }
            70 => {
              /* Major page faults.  */
              printf(
                b"%lu\x00" as *const u8 as *const libc::c_char,
                (*resp).ru.c2rust_unnamed_4.ru_majflt,
              );
            }
            73 => {
              /* Inputs.  */
              printf(
                b"%lu\x00" as *const u8 as *const libc::c_char,
                (*resp).ru.c2rust_unnamed_6.ru_inblock,
              );
            }
            75 => {
              /* Average mem usage == data+stack+text.  */
              printf(
                b"%lu\x00" as *const u8 as *const libc::c_char,
                ptok(
                  pagesize,
                  (*resp).ru.c2rust_unnamed_1.ru_idrss as libc::c_ulong,
                )
                .wrapping_add(ptok(
                  pagesize,
                  (*resp).ru.c2rust_unnamed_2.ru_isrss as libc::c_ulong,
                ))
                .wrapping_add(ptok(
                  pagesize,
                  (*resp).ru.c2rust_unnamed_0.ru_ixrss as libc::c_ulong,
                ))
                .wrapping_div(cpu_ticks as libc::c_ulong),
              );
            }
            77 => {
              /* Maximum resident set size.  */
              printf(
                b"%lu\x00" as *const u8 as *const libc::c_char,
                ptok(
                  pagesize,
                  (*resp).ru.c2rust_unnamed.ru_maxrss as libc::c_ulong,
                ),
              );
            }
            79 => {
              /* Outputs.  */
              printf(
                b"%lu\x00" as *const u8 as *const libc::c_char,
                (*resp).ru.c2rust_unnamed_7.ru_oublock,
              );
            }
            80 => {
              /* Percent of CPU this job got.  */
              /* % cpu is (total cpu time)/(elapsed time).  */
              if (*resp).elapsed_ms > 0i32 as libc::c_uint {
                printf(
                  b"%u%%\x00" as *const u8 as *const libc::c_char,
                  vv_ms
                    .wrapping_mul(100i32 as libc::c_uint)
                    .wrapping_div((*resp).elapsed_ms),
                );
              } else {
                printf(b"?%%\x00" as *const u8 as *const libc::c_char);
              }
            }
            82 => {
              /* Minor page faults (reclaims).  */
              printf(
                b"%lu\x00" as *const u8 as *const libc::c_char,
                (*resp).ru.c2rust_unnamed_3.ru_minflt,
              );
            }
            83 => {
              /* System time.  */
              printf(
                b"%u.%02u\x00" as *const u8 as *const libc::c_char,
                (*resp).ru.ru_stime.tv_sec as libc::c_uint,
                ((*resp).ru.ru_stime.tv_usec / 10000i32 as libc::c_long) as libc::c_uint,
              );
            }
            84 => {
              /* System time.  */
              if (*resp).ru.ru_stime.tv_sec >= 3600i32 as libc::c_long {
                /* One hour -> h:m:s.  */
                printf(
                  b"%uh %um %02us\x00" as *const u8 as *const libc::c_char,
                  ((*resp).ru.ru_stime.tv_sec / 3600i32 as libc::c_long) as libc::c_uint,
                  (((*resp).ru.ru_stime.tv_sec % 3600i32 as libc::c_long) as libc::c_uint)
                    .wrapping_div(60i32 as libc::c_uint),
                  ((*resp).ru.ru_stime.tv_sec % 60i32 as libc::c_long) as libc::c_uint,
                );
              } else {
                printf(
                  b"%um %u.%02us\x00" as *const u8 as *const libc::c_char,
                  ((*resp).ru.ru_stime.tv_sec / 60i32 as libc::c_long) as libc::c_uint,
                  ((*resp).ru.ru_stime.tv_sec % 60i32 as libc::c_long) as libc::c_uint,
                  ((*resp).ru.ru_stime.tv_usec / 10000i32 as libc::c_long) as libc::c_uint,
                );
              }
            }
            85 => {
              /* User time.  */
              printf(
                b"%u.%02u\x00" as *const u8 as *const libc::c_char,
                (*resp).ru.ru_utime.tv_sec as libc::c_uint,
                ((*resp).ru.ru_utime.tv_usec / 10000i32 as libc::c_long) as libc::c_uint,
              );
            }
            117 => {
              /* User time.  */
              if (*resp).ru.ru_utime.tv_sec >= 3600i32 as libc::c_long {
                /* One hour -> h:m:s.  */
                printf(
                  b"%uh %um %02us\x00" as *const u8 as *const libc::c_char,
                  ((*resp).ru.ru_utime.tv_sec / 3600i32 as libc::c_long) as libc::c_uint,
                  (((*resp).ru.ru_utime.tv_sec % 3600i32 as libc::c_long) as libc::c_uint)
                    .wrapping_div(60i32 as libc::c_uint),
                  ((*resp).ru.ru_utime.tv_sec % 60i32 as libc::c_long) as libc::c_uint,
                );
              } else {
                printf(
                  b"%um %u.%02us\x00" as *const u8 as *const libc::c_char,
                  ((*resp).ru.ru_utime.tv_sec / 60i32 as libc::c_long) as libc::c_uint,
                  ((*resp).ru.ru_utime.tv_sec % 60i32 as libc::c_long) as libc::c_uint,
                  ((*resp).ru.ru_utime.tv_usec / 10000i32 as libc::c_long) as libc::c_uint,
                );
              }
            }
            87 => {
              /* Times swapped out.  */
              printf(
                b"%lu\x00" as *const u8 as *const libc::c_char,
                (*resp).ru.c2rust_unnamed_5.ru_nswap,
              );
            }
            88 => {
              /* Average shared text size.  */
              printf(
                b"%lu\x00" as *const u8 as *const libc::c_char,
                ptok(
                  pagesize,
                  (*resp).ru.c2rust_unnamed_0.ru_ixrss as libc::c_ulong,
                )
                .wrapping_div(cpu_ticks as libc::c_ulong),
              );
            }
            90 => {
              /* Page size.  */
              printf(b"%u\x00" as *const u8 as *const libc::c_char, pagesize);
            }
            99 => {
              /* Involuntary context switches.  */
              printf(
                b"%lu\x00" as *const u8 as *const libc::c_char,
                (*resp).ru.c2rust_unnamed_12.ru_nivcsw,
              );
            }
            101 => {
              /* Elapsed real time in seconds.  */
              printf(
                b"%u.%02u\x00" as *const u8 as *const libc::c_char,
                (*resp).elapsed_ms.wrapping_div(1000i32 as libc::c_uint),
                (*resp)
                  .elapsed_ms
                  .wrapping_div(10i32 as libc::c_uint)
                  .wrapping_rem(100i32 as libc::c_uint),
              );
            }
            107 => {
              /* Signals delivered.  */
              printf(
                b"%lu\x00" as *const u8 as *const libc::c_char,
                (*resp).ru.c2rust_unnamed_10.ru_nsignals,
              );
            }
            112 => {
              /* Average stack segment.  */
              printf(
                b"%lu\x00" as *const u8 as *const libc::c_char,
                ptok(
                  pagesize,
                  (*resp).ru.c2rust_unnamed_2.ru_isrss as libc::c_ulong,
                )
                .wrapping_div(cpu_ticks as libc::c_ulong),
              );
            }
            114 => {
              /* Incoming socket messages received.  */
              printf(
                b"%lu\x00" as *const u8 as *const libc::c_char,
                (*resp).ru.c2rust_unnamed_9.ru_msgrcv,
              );
            }
            115 => {
              /* Outgoing socket messages sent.  */
              printf(
                b"%lu\x00" as *const u8 as *const libc::c_char,
                (*resp).ru.c2rust_unnamed_8.ru_msgsnd,
              );
            }
            116 => {
              /* Average resident set size.  */
              printf(
                b"%lu\x00" as *const u8 as *const libc::c_char,
                ptok(
                  pagesize,
                  (*resp).ru.c2rust_unnamed_1.ru_idrss as libc::c_ulong,
                )
                .wrapping_div(cpu_ticks as libc::c_ulong),
              );
            }
            119 => {
              /* Voluntary context switches.  */
              printf(
                b"%lu\x00" as *const u8 as *const libc::c_char,
                (*resp).ru.c2rust_unnamed_11.ru_nvcsw,
              );
            }
            120 => {
              /* Exit status.  */
              printf(
                b"%u\x00" as *const u8 as *const libc::c_char,
                ((*resp).waitstatus & 0xff00i32) >> 8i32,
              );
            }
            _ => {}
          }
        }
        _ => {}
      }
      fmt = fmt.offset(1)
    }
  }
  /* ret: */
  bb_putchar('\n' as i32);
}
/* Run command CMD and return statistics on it.
Put the statistics in *RESP.  */
unsafe extern "C" fn run_command(mut cmd: *const *mut libc::c_char, mut resp: *mut resource_t) {
  let mut pid: pid_t = 0;
  let mut interrupt_signal: Option<unsafe extern "C" fn(_: libc::c_int) -> ()> = None;
  let mut quit_signal: Option<unsafe extern "C" fn(_: libc::c_int) -> ()> = None;
  (*resp).elapsed_ms = monotonic_ms() as libc::c_uint;
  pid = {
    let mut bb__xvfork_pid: pid_t = vfork();
    if bb__xvfork_pid < 0i32 {
      bb_simple_perror_msg_and_die(b"vfork\x00" as *const u8 as *const libc::c_char);
    }
    bb__xvfork_pid
  };
  if pid == 0i32 {
    /* Child */
    BB_EXECVP_or_die(cmd as *mut *mut libc::c_char);
  }
  /* Have signals kill the child but not self (if possible).  */
  //TODO: just block all sigs? and re-enable them in the very end in main?
  interrupt_signal = signal(
    2i32,
    ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(1i32 as libc::intptr_t),
  );
  quit_signal = signal(
    3i32,
    ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(1i32 as libc::intptr_t),
  );
  resuse_end(pid, resp);
  /* Re-enable signals.  */
  signal(2i32, interrupt_signal);
  signal(3i32, quit_signal);
}
#[no_mangle]
pub unsafe extern "C" fn time_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut res: resource_t = resource_t {
    waitstatus: 0,
    ru: rusage {
      ru_utime: timeval {
        tv_sec: 0,
        tv_usec: 0,
      },
      ru_stime: timeval {
        tv_sec: 0,
        tv_usec: 0,
      },
      c2rust_unnamed: C2RustUnnamed_12 { ru_maxrss: 0 },
      c2rust_unnamed_0: C2RustUnnamed_11 { ru_ixrss: 0 },
      c2rust_unnamed_1: C2RustUnnamed_10 { ru_idrss: 0 },
      c2rust_unnamed_2: C2RustUnnamed_9 { ru_isrss: 0 },
      c2rust_unnamed_3: C2RustUnnamed_8 { ru_minflt: 0 },
      c2rust_unnamed_4: C2RustUnnamed_7 { ru_majflt: 0 },
      c2rust_unnamed_5: C2RustUnnamed_6 { ru_nswap: 0 },
      c2rust_unnamed_6: C2RustUnnamed_5 { ru_inblock: 0 },
      c2rust_unnamed_7: C2RustUnnamed_4 { ru_oublock: 0 },
      c2rust_unnamed_8: C2RustUnnamed_3 { ru_msgsnd: 0 },
      c2rust_unnamed_9: C2RustUnnamed_2 { ru_msgrcv: 0 },
      c2rust_unnamed_10: C2RustUnnamed_1 { ru_nsignals: 0 },
      c2rust_unnamed_11: C2RustUnnamed_0 { ru_nvcsw: 0 },
      c2rust_unnamed_12: C2RustUnnamed { ru_nivcsw: 0 },
    },
    elapsed_ms: 0,
  };
  /* $TIME has lowest prio (-v,-p,-f FMT overrride it) */
  let ref mut fresh0 = getenv(b"TIME\x00" as *const u8 as *const libc::c_char);
  let mut output_format: *const libc::c_char = if !(*fresh0).is_null() {
    *fresh0
  } else {
    default_format.as_ptr()
  };
  let mut output_filename: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut output_fd: libc::c_int = 0;
  let mut opt: libc::c_int = 0;
  let mut ex: libc::c_int = 0;
  /* "+": stop on first non-option */
  opt = getopt32(
    argv,
    b"^+vpao:f:\x00-1\x00" as *const u8 as *const libc::c_char,
    &mut output_filename as *mut *mut libc::c_char,
    &mut output_format as *mut *const libc::c_char,
  ) as libc::c_int;
  argv = argv.offset(optind as isize);
  if opt & OPT_v as libc::c_int != 0 {
    output_format = long_format.as_ptr()
  }
  if opt & OPT_p as libc::c_int != 0 {
    output_format = posix_format.as_ptr()
  }
  output_fd = 2i32;
  if opt & OPT_o as libc::c_int != 0 {
    output_fd = xopen(
      output_filename,
      if opt & OPT_a as libc::c_int != 0 {
        (0o100i32 | 0o1i32 | 0o2000000i32) | 0o2000i32
      } else {
        (0o100i32 | 0o1i32 | 0o2000000i32) | 0o1000i32
      },
    );
    if 0o2000000i32 == 0 {
      close_on_exec_on(output_fd);
    }
  }
  run_command(argv, &mut res);
  /* Cheat. printf's are shorter :) */
  xdup2(output_fd, 1i32);
  summarize(output_format, argv, &mut res);
  ex = (res.waitstatus & 0xff00i32) >> 8i32;
  /* Impossible: we do not use WUNTRACED flag in wait()...
  if (WIFSTOPPED(res.waitstatus))
    ex = WSTOPSIG(res.waitstatus);
  */
  if ((res.waitstatus & 0x7fi32) + 1i32) as libc::c_schar as libc::c_int >> 1i32 > 0i32 {
    ex = res.waitstatus & 0x7fi32
  }
  fflush_stdout_and_exit(ex);
}
