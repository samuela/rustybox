use libc;

use libc::free;
extern "C" {


  #[no_mangle]
  fn kill(__pid: pid_t, __sig: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;

  #[no_mangle]
  fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;

  #[no_mangle]
  fn snprintf(
    _: *mut libc::c_char,
    _: libc::c_ulong,
    _: *const libc::c_char,
    _: ...
  ) -> libc::c_int;



  #[no_mangle]
  fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;

  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;

  #[no_mangle]
  fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;

  #[no_mangle]
  fn time(__timer: *mut time_t) -> time_t;

  #[no_mangle]
  fn ctime(__timer: *const time_t) -> *mut libc::c_char;

  #[no_mangle]
  static bb_errno: *mut libc::c_int;

  #[no_mangle]
  fn xmemdup(s: *const libc::c_void, n: libc::c_int) -> *mut libc::c_void;

  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn xlseek(fd: libc::c_int, offset: off_t, whence: libc::c_int) -> off_t;

  #[no_mangle]
  fn safe_strncpy(
    dst: *mut libc::c_char,
    src: *const libc::c_char,
    size: size_t,
  ) -> *mut libc::c_char;

  #[no_mangle]
  fn xread(fd: libc::c_int, buf: *mut libc::c_void, count: size_t);

  #[no_mangle]
  fn fflush_stdout_and_exit(retval: libc::c_int) -> !;

  #[no_mangle]
  static mut option_mask32: u32;

  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;

  #[no_mangle]
  fn llist_add_to(old_head: *mut *mut llist_t, data: *mut libc::c_void);

  #[no_mangle]
  fn llist_unlink(head: *mut *mut llist_t, elm: *mut llist_t);

  #[no_mangle]
  fn llist_free(
    elm: *mut llist_t,
    freeit: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
  );
}



use libc::pid_t;

use libc::off_t;
use crate::librb::size_t;
use crate::librb::smallint;




use libc::stat;

use libc::time_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __exit_status {
  pub e_termination: libc::c_short,
  pub e_exit: libc::c_short,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct utmpx {
  pub ut_type: libc::c_short,
  pub ut_pid: pid_t,
  pub ut_line: [libc::c_char; 32],
  pub ut_id: [libc::c_char; 4],
  pub ut_user: [libc::c_char; 32],
  pub ut_host: [libc::c_char; 256],
  pub ut_exit: __exit_status,
  pub ut_session: i32,
  pub ut_tv: C2RustUnnamed,
  pub ut_addr_v6: [i32; 4],
  pub __glibc_reserved: [libc::c_char; 20],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
  pub tv_sec: i32,
  pub tv_usec: i32,
}

use crate::libbb::llist::llist_t;

pub type C2RustUnnamed_0 = libc::c_uint;
pub const GONE: C2RustUnnamed_0 = 5;
pub const CRASH: C2RustUnnamed_0 = 4;
pub const REBOOT: C2RustUnnamed_0 = 3;
pub const DOWN: C2RustUnnamed_0 = 2;
pub const LOGGED: C2RustUnnamed_0 = 1;
pub const NORMAL: C2RustUnnamed_0 = 0;

pub type C2RustUnnamed_1 = libc::c_uint;
/* -H header          */
/* -f input file      */
// pub const LAST_OPT_H: C2RustUnnamed_1 = 4;
/* -W wide            */
// pub const LAST_OPT_f: C2RustUnnamed_1 = 2;
pub const LAST_OPT_W: C2RustUnnamed_1 = 1;

unsafe extern "C" fn show_entry(mut ut: *mut utmpx, mut state: libc::c_int, mut dur_secs: time_t) {
  let mut days: libc::c_uint = 0;
  let mut hours: libc::c_uint = 0;
  let mut mins: libc::c_uint = 0;
  let mut duration: [libc::c_char; 23] = [0; 23];
  let mut login_time: [libc::c_char; 17] = [0; 17];
  let mut logout_time: [libc::c_char; 8] = [0; 8];
  let mut logout_str: *const libc::c_char = 0 as *const libc::c_char;
  let mut duration_str: *const libc::c_char = 0 as *const libc::c_char;
  let mut tmp: time_t = 0;
  /* manpages say ut_tv.tv_sec *is* time_t,
   * but some systems have it wrong */
  tmp = (*ut).ut_tv.tv_sec as time_t;
  safe_strncpy(login_time.as_mut_ptr(), ctime(&mut tmp), 17i32 as size_t);
  tmp = dur_secs;
  snprintf(
    logout_time.as_mut_ptr(),
    8i32 as libc::c_ulong,
    b"- %s\x00" as *const u8 as *const libc::c_char,
    ctime(&mut tmp).offset(11),
  );
  dur_secs = if dur_secs - (*ut).ut_tv.tv_sec as time_t > 0i32 as time_t {
    (dur_secs) - (*ut).ut_tv.tv_sec as time_t
  } else {
    0i32 as time_t
  };
  /* unsigned int is easier to divide than time_t (which may be signed long) */
  mins = (dur_secs / 60i32 as libc::c_long) as libc::c_uint;
  days = mins.wrapping_div((24i32 * 60i32) as libc::c_uint);
  mins = mins.wrapping_rem((24i32 * 60i32) as libc::c_uint);
  hours = mins.wrapping_div(60i32 as libc::c_uint);
  mins = mins.wrapping_rem(60i32 as libc::c_uint);
  //	if (days) {
  sprintf(
    duration.as_mut_ptr(),
    b"(%u+%02u:%02u)\x00" as *const u8 as *const libc::c_char,
    days,
    hours,
    mins,
  );
  //	} else {
  //		sprintf(duration, " (%02u:%02u)", hours, mins);
  //	}
  logout_str = logout_time.as_mut_ptr();
  duration_str = duration.as_mut_ptr();
  match state {
    1 => {
      logout_str = b"  still\x00" as *const u8 as *const libc::c_char;
      duration_str = b"logged in\x00" as *const u8 as *const libc::c_char
    }
    2 => logout_str = b"- down \x00" as *const u8 as *const libc::c_char,
    4 => logout_str = b"- crash\x00" as *const u8 as *const libc::c_char,
    5 => {
      logout_str = b"   gone\x00" as *const u8 as *const libc::c_char;
      duration_str = b"- no logout\x00" as *const u8 as *const libc::c_char
    }
    0 | 3 | _ => {}
  }
  printf(
    b"%-8.8s %-12.12s %-*.*s %-16.16s %-7.7s %s\n\x00" as *const u8 as *const libc::c_char,
    (*ut).ut_user.as_mut_ptr(),
    (*ut).ut_line.as_mut_ptr(),
    if option_mask32 & LAST_OPT_W as libc::c_int as libc::c_uint != 0 {
      46i32
    } else {
      16i32
    },
    if option_mask32 & LAST_OPT_W as libc::c_int as libc::c_uint != 0 {
      46i32
    } else {
      16i32
    },
    (*ut).ut_host.as_mut_ptr(),
    login_time.as_mut_ptr(),
    logout_str,
    duration_str,
  );
}
unsafe extern "C" fn get_ut_type(mut ut: *mut utmpx) -> libc::c_int {
  if (*ut).ut_line[0] as libc::c_int == '~' as i32 {
    if strcmp(
      (*ut).ut_user.as_mut_ptr(),
      b"shutdown\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
      return 254i32;
    }
    if strcmp(
      (*ut).ut_user.as_mut_ptr(),
      b"reboot\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
      return 2i32;
    }
    if strcmp(
      (*ut).ut_user.as_mut_ptr(),
      b"runlevel\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
      return 1i32;
    }
    return (*ut).ut_type as libc::c_int;
  }
  if (*ut).ut_user[0] as libc::c_int == 0i32 {
    return 8i32;
  }
  if (*ut).ut_type as libc::c_int != 8i32
    && strcmp(
      (*ut).ut_user.as_mut_ptr(),
      b"LOGIN\x00" as *const u8 as *const libc::c_char,
    ) != 0i32
    && (*ut).ut_user[0] as libc::c_int != 0
    && (*ut).ut_line[0] as libc::c_int != 0
  {
    (*ut).ut_type = 7i32 as libc::c_short
  }
  if strcmp(
    (*ut).ut_user.as_mut_ptr(),
    b"date\x00" as *const u8 as *const libc::c_char,
  ) == 0i32
  {
    if (*ut).ut_line[0] as libc::c_int == '|' as i32 {
      return 4i32;
    }
    if (*ut).ut_line[0] as libc::c_int == '{' as i32 {
      return 3i32;
    }
  }
  return (*ut).ut_type as libc::c_int;
}
unsafe extern "C" fn is_runlevel_shutdown(mut ut: *mut utmpx) -> libc::c_int {
  if (*ut).ut_pid & 255i32 == '0' as i32 || (*ut).ut_pid & 255i32 == '6' as i32 {
    return 1i32;
  }
  return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn last_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut ut: utmpx = utmpx {
    ut_type: 0,
    ut_pid: 0,
    ut_line: [0; 32],
    ut_id: [0; 4],
    ut_user: [0; 32],
    ut_host: [0; 256],
    ut_exit: __exit_status {
      e_termination: 0,
      e_exit: 0,
    },
    ut_session: 0,
    ut_tv: C2RustUnnamed {
      tv_sec: 0,
      tv_usec: 0,
    },
    ut_addr_v6: [0; 4],
    __glibc_reserved: [0; 20],
  };
  let mut filename: *const libc::c_char = b"/var/log/wtmp\x00" as *const u8 as *const libc::c_char;
  let mut zlist: *mut llist_t = 0 as *mut llist_t;
  let mut pos: off_t = 0;
  let mut start_time: time_t = 0;
  let mut boot_time: time_t = 0;
  let mut down_time: time_t = 0;
  let mut file: libc::c_int = 0;
  let mut going_down: smallint = 0;
  let mut boot_down: smallint = 0;
  /*opt =*/
  getopt32(
    argv,
    b"Wf:\x00" as *const u8 as *const libc::c_char,
    &mut filename as *mut *const libc::c_char,
  );
  file = xopen(filename, 0i32);
  /* in case the file is empty... */
  let mut st: stat = std::mem::zeroed(); /* 0 */
  fstat(file, &mut st);
  start_time = st.st_ctime;
  time(&mut down_time);
  going_down = 0i32 as smallint;
  boot_down = NORMAL as libc::c_int as smallint;
  zlist = 0 as *mut llist_t;
  boot_time = 0i32 as time_t;
  /* get file size, rounding down to last full record */
  pos = (xlseek(file, 0i32 as off_t, 2i32) as libc::c_ulong)
    .wrapping_div(::std::mem::size_of::<utmpx>() as libc::c_ulong)
    .wrapping_mul(::std::mem::size_of::<utmpx>() as libc::c_ulong) as off_t;
  loop {
    pos -= ::std::mem::size_of::<utmpx>() as libc::c_ulong as off_t;
    if pos < 0 {
      break;
    }
    xlseek(file, pos, 0i32);
    xread(
      file,
      &mut ut as *mut utmpx as *mut libc::c_void,
      ::std::mem::size_of::<utmpx>() as libc::c_ulong,
    );
    /* rewritten by each record, eventially will have
     * first record's ut_tv.tv_sec: */
    start_time = ut.ut_tv.tv_sec as time_t;
    match get_ut_type(&mut ut) {
      254 => {
        down_time = ut.ut_tv.tv_sec as time_t;
        boot_down = DOWN as libc::c_int as smallint;
        going_down = 1i32 as smallint
      }
      1 => {
        if is_runlevel_shutdown(&mut ut) != 0 {
          down_time = ut.ut_tv.tv_sec as time_t;
          going_down = 1i32 as smallint;
          boot_down = DOWN as libc::c_int as smallint
        }
      }
      2 => {
        strcpy(
          ut.ut_line.as_mut_ptr(),
          b"system boot\x00" as *const u8 as *const libc::c_char,
        );
        show_entry(&mut ut, REBOOT as libc::c_int, down_time);
        boot_down = CRASH as libc::c_int as smallint;
        going_down = 1i32 as smallint
      }
      8 => {
        if !(ut.ut_line[0] == 0) {
          /* add_entry */
          llist_add_to(
            &mut zlist,
            xmemdup(
              &mut ut as *mut utmpx as *const libc::c_void,
              ::std::mem::size_of::<utmpx>() as libc::c_ulong as libc::c_int,
            ),
          );
        }
      }
      7 => {
        let mut show: libc::c_int = 0;
        if !(ut.ut_line[0] == 0) {
          /* find_entry */
          show = 1i32;
          let mut el: *mut llist_t = 0 as *mut llist_t;
          let mut next: *mut llist_t = 0 as *mut llist_t;
          el = zlist;
          while !el.is_null() {
            let mut up: *mut utmpx = (*el).data as *mut utmpx;
            next = (*el).link;
            if strncmp(
              (*up).ut_line.as_mut_ptr(),
              ut.ut_line.as_mut_ptr(),
              32i32 as libc::c_ulong,
            ) == 0i32
            {
              if show != 0 {
                show_entry(&mut ut, NORMAL as libc::c_int, (*up).ut_tv.tv_sec as time_t);
                show = 0i32
              }
              llist_unlink(&mut zlist, el);
              free((*el).data as *mut libc::c_void);
              free(el as *mut libc::c_void);
            }
            el = next
          }
          if show != 0 {
            let mut state: libc::c_int = boot_down as libc::c_int;
            if boot_time ==0{
              state = LOGGED as libc::c_int;
              /* Check if the process is alive */
              if ut.ut_pid > 0i32 && kill(ut.ut_pid, 0i32) != 0i32 && *bb_errno == 3i32 {
                state = GONE as libc::c_int
              }
            }
            show_entry(&mut ut, state, boot_time);
          }
          /* add_entry */
          llist_add_to(
            &mut zlist,
            xmemdup(
              &mut ut as *mut utmpx as *const libc::c_void,
              ::std::mem::size_of::<utmpx>() as libc::c_ulong as libc::c_int,
            ),
          );
        }
      }
      _ => {}
    }
    if going_down != 0 {
      boot_time = ut.ut_tv.tv_sec as time_t;
      llist_free(
        zlist,
        Some(free as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
      );
      zlist = 0 as *mut llist_t;
      going_down = 0i32 as smallint
    }
  }
  printf(
    b"\nwtmp begins %s\x00" as *const u8 as *const libc::c_char,
    ctime(&mut start_time),
  );
  fflush_stdout_and_exit(0i32);
}
