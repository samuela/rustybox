use libc;
use libc::getutxent;
use libc::localtime;
use libc::printf;
use libc::time;

extern "C" {
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  fn sysinfo(__info: *mut sysinfo) -> libc::c_int;
}

pub type __u16 = libc::c_ushort;
pub type u32 = libc::c_uint;
pub type __kernel_long_t = libc::c_long;
pub type __kernel_ulong_t = libc::c_ulong;
#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct sysinfo {
  pub uptime: __kernel_long_t,
  pub loads: [__kernel_ulong_t; 3],
  pub totalram: __kernel_ulong_t,
  pub freeram: __kernel_ulong_t,
  pub sharedram: __kernel_ulong_t,
  pub bufferram: __kernel_ulong_t,
  pub totalswap: __kernel_ulong_t,
  pub freeswap: __kernel_ulong_t,
  pub procs: __u16,
  pub pad: __u16,
  pub totalhigh: __kernel_ulong_t,
  pub freehigh: __kernel_ulong_t,
  pub mem_unit: u32,
  pub _f: [libc::c_char; 0],
}
/* nr of bits of precision */
/* 1.0 as fixed-point */
#[no_mangle]
pub unsafe extern "C" fn uptime_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let opts = getopt32(argv, b"s\x00" as *const u8 as *const libc::c_char);

  let mut current_secs = 0;
  time(&mut current_secs);

  let mut info = Default::default();
  sysinfo(&mut info);

  if opts != 0 {
    // -s
    current_secs -= info.uptime
  }

  let mut current_time = *localtime(&mut current_secs);
  if opts != 0 {
    // -s
    println!(
      "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
      current_time.tm_year + 1900,
      current_time.tm_mon + 1,
      current_time.tm_mday,
      current_time.tm_hour,
      current_time.tm_min,
      current_time.tm_sec,
    );
    /* The above way of calculating boot time is wobbly,
     * info.uptime has only 1 second precision, which makes
     * "uptime -s" wander +- one second.
     * /proc/uptime may be better, it has 0.01s precision.
     */
    return 0;
  }

  printf(
    b" %02u:%02u:%02u up \x00" as *const u8 as *const libc::c_char,
    current_time.tm_hour,
    current_time.tm_min,
    current_time.tm_sec,
  );

  let updays = info.uptime.wrapping_div(60 * 60 * 24);
  if updays != 0 {
    printf(
      b"%u day%s, \x00" as *const u8 as *const libc::c_char,
      updays,
      if updays != 1 {
        b"s\x00" as *const u8 as *const libc::c_char
      } else {
        b"\x00" as *const u8 as *const libc::c_char
      },
    );
  }

  let mut upminutes = info.uptime.wrapping_div(60);
  let uphours = upminutes.wrapping_div(60).wrapping_rem(24);
  upminutes = upminutes.wrapping_rem(60);
  if uphours != 0 {
    printf(
      b"%2u:%02u\x00" as *const u8 as *const libc::c_char,
      uphours,
      upminutes,
    );
  } else {
    printf(b"%u min\x00" as *const u8 as *const libc::c_char, upminutes);
  }

  let mut users = 0;
  loop {
    let ut = getutxent();
    if ut.is_null() {
      break;
    }
    if (*ut).ut_type == 7 && (*ut).ut_user[0] as libc::c_int != '\u{0}' as i32 {
      users += 1
    }
  }

  printf(
    b",  %u users\x00" as *const u8 as *const libc::c_char,
    users,
  );

  printf(
    b",  load average: %u.%02u, %u.%02u, %u.%02u\n\x00" as *const u8 as *const libc::c_char,
    (info.loads[0] >> 16i32) as libc::c_uint,
    ((info.loads[0] & ((1i32 << 16i32) - 1i32) as libc::c_ulong)
      .wrapping_mul(100i32 as libc::c_ulong)
      >> 16i32) as libc::c_uint,
    (info.loads[1] >> 16i32) as libc::c_uint,
    ((info.loads[1] & ((1i32 << 16i32) - 1i32) as libc::c_ulong)
      .wrapping_mul(100i32 as libc::c_ulong)
      >> 16i32) as libc::c_uint,
    (info.loads[2] >> 16i32) as libc::c_uint,
    ((info.loads[2] & ((1i32 << 16i32) - 1i32) as libc::c_ulong)
      .wrapping_mul(100i32 as libc::c_ulong)
      >> 16i32) as libc::c_uint,
  );

  0
}
