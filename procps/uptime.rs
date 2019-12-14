use libc;
use libc::getutxent;
use libc::localtime;
use libc::time;
use libc::time_t;

extern "C" {
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  fn sysinfo(__info: *mut sysinfo) -> libc::c_int;
}

#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct sysinfo {
  pub uptime: libc::c_long,
  pub loads: [libc::c_ulong; 3],
  pub totalram: libc::c_ulong,
  pub freeram: libc::c_ulong,
  pub sharedram: libc::c_ulong,
  pub bufferram: libc::c_ulong,
  pub totalswap: libc::c_ulong,
  pub freeswap: libc::c_ulong,
  pub procs: libc::c_ushort,
  pub pad: libc::c_ushort,
  pub totalhigh: libc::c_ulong,
  pub freehigh: libc::c_ulong,
  pub mem_unit: libc::c_uint,
  pub _f: [libc::c_char; 0],
}

fn get_secs() -> time_t {
  unsafe { time(std::ptr::null_mut()) }
}

fn get_users() -> u32 {
  let mut users = 0;
  unsafe {
    loop {
      let ut = getutxent();
      if ut.is_null() {
        break;
      }
      if (*ut).ut_type == 7 && (*ut).ut_user[0] != 0 {
        users += 1
      }
    }
  }
  users
}

/* nr of bits of precision */
/* 1.0 as fixed-point */
#[no_mangle]
pub extern "C" fn uptime_main(mut _argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int {
  let opts = unsafe { getopt32(argv, b"s\x00" as *const u8 as *const libc::c_char) };

  let mut current_secs = get_secs();

  let mut info = Default::default();
  unsafe {
    sysinfo(&mut info);
  }

  if opts != 0 {
    // -s
    current_secs -= info.uptime
  }

  let current_time = unsafe { *localtime(&current_secs) };
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

  print!(
    " {:02}:{:02}:{:02} up ",
    current_time.tm_hour, current_time.tm_min, current_time.tm_sec,
  );

  let updays = info.uptime / (60 * 60 * 24);
  if updays > 0 {
    print!("{} day{}, ", updays, if updays != 1 { "s" } else { "" },);
  }

  let mut upminutes = info.uptime / 60;
  let uphours = (upminutes / 60).wrapping_rem(24);
  upminutes = upminutes.wrapping_rem(60);
  if uphours > 0 {
    print!("{:2}:{:02}", uphours, upminutes);
  } else {
    print!("{} min", upminutes);
  }

  println!(
    ",  {} users,  load average: {}.{:02}, {}.{:02}, {}.{:02}",
    get_users(),
    info.loads[0] >> 16,
    (info.loads[0] & ((1 << 16) - 1)).wrapping_mul(100) >> 16,
    info.loads[1] >> 16,
    (info.loads[1] & ((1 << 16) - 1)).wrapping_mul(100) >> 16,
    info.loads[2] >> 16,
    (info.loads[2] & ((1 << 16) - 1)).wrapping_mul(100) >> 16,
  );

  0
}
