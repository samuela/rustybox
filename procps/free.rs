use libc;
extern "C" {
  #[no_mangle]
  fn fclose(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn fgets_unlocked(
    __s: *mut libc::c_char,
    __n: libc::c_int,
    __stream: *mut FILE,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn xfopen_for_read(path: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn sysinfo(__info: *mut sysinfo) -> libc::c_int;
}

use crate::librb::uint8_t;

use libc::FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub mem_unit: libc::c_uint,
  pub unit_steps: uint8_t,
  pub cached_kb: libc::c_ulong,
  pub available_kb: libc::c_ulong,
  pub reclaimable_kb: libc::c_ulong,
}
pub type __u16 = libc::c_ushort;
pub type __u32 = libc::c_uint;
pub type __kernel_long_t = libc::c_long;
pub type __kernel_ulong_t = libc::c_ulong;
#[derive(Copy, Clone)]
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
  pub mem_unit: __u32,
  pub _f: [libc::c_char; 0],
}
/* Because of NOFORK, "globals" are not in global data */
unsafe extern "C" fn scale(mut g: *mut globals, mut d: libc::c_ulong) -> libc::c_ulonglong {
  return (d as libc::c_ulonglong).wrapping_mul((*g).mem_unit as libc::c_ulonglong)
    >> (*g).unit_steps as libc::c_int;
}
/* NOINLINE reduces main() stack usage, which makes code smaller (on x86 at least) */
#[inline(never)]
unsafe extern "C" fn parse_meminfo(mut g: *mut globals) -> libc::c_uint {
  let mut buf: [libc::c_char; 60] = [0; 60]; /* actual lines we expect are ~30 chars or less */
  let mut fp: *mut FILE = 0 as *mut FILE;
  let mut seen_cached_and_available_and_reclaimable: libc::c_int = 0;
  fp = xfopen_for_read(b"/proc/meminfo\x00" as *const u8 as *const libc::c_char);
  (*g).reclaimable_kb = 0i32 as libc::c_ulong;
  (*g).available_kb = (*g).reclaimable_kb;
  (*g).cached_kb = (*g).available_kb;
  seen_cached_and_available_and_reclaimable = 3i32;
  while !fgets_unlocked(
    buf.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_char; 60]>() as libc::c_ulong as libc::c_int,
    fp,
  )
  .is_null()
  {
    if sscanf(
      buf.as_mut_ptr(),
      b"Cached: %lu %*s\n\x00" as *const u8 as *const libc::c_char,
      &mut (*g).cached_kb as *mut libc::c_ulong,
    ) == 1i32
    {
      seen_cached_and_available_and_reclaimable -= 1;
      if seen_cached_and_available_and_reclaimable == 0i32 {
        break;
      }
    }
    if sscanf(
      buf.as_mut_ptr(),
      b"MemAvailable: %lu %*s\n\x00" as *const u8 as *const libc::c_char,
      &mut (*g).available_kb as *mut libc::c_ulong,
    ) == 1i32
    {
      seen_cached_and_available_and_reclaimable -= 1;
      if seen_cached_and_available_and_reclaimable == 0i32 {
        break;
      }
    }
    if !(sscanf(
      buf.as_mut_ptr(),
      b"SReclaimable: %lu %*s\n\x00" as *const u8 as *const libc::c_char,
      &mut (*g).reclaimable_kb as *mut libc::c_ulong,
    ) == 1i32)
    {
      continue;
    }
    seen_cached_and_available_and_reclaimable -= 1;
    if seen_cached_and_available_and_reclaimable == 0i32 {
      break;
    }
  }
  /* Have to close because of NOFORK */
  fclose(fp);
  return (seen_cached_and_available_and_reclaimable == 0i32) as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn free_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut G: globals = globals {
    mem_unit: 0,
    unit_steps: 0,
    cached_kb: 0,
    available_kb: 0,
    reclaimable_kb: 0,
  };
  let mut info: sysinfo = sysinfo {
    uptime: 0,
    loads: [0; 3],
    totalram: 0,
    freeram: 0,
    sharedram: 0,
    bufferram: 0,
    totalswap: 0,
    freeswap: 0,
    procs: 0,
    pad: 0,
    totalhigh: 0,
    freehigh: 0,
    mem_unit: 0,
    _f: [0; 0],
  };
  let mut cached: libc::c_ulonglong = 0;
  let mut cached_plus_free: libc::c_ulonglong = 0;
  let mut available: libc::c_ulonglong = 0;
  let mut seen_available: libc::c_int = 0;
  G.unit_steps = 10i32 as uint8_t;
  if !(*argv.offset(1)).is_null() && *(*argv.offset(1)).offset(0) as libc::c_int == '-' as i32 {
    match *(*argv.offset(1)).offset(1) as libc::c_int {
      98 => G.unit_steps = 0i32 as uint8_t,
      107 => {}
      109 => {
        /* 2^(2*10) */
        G.unit_steps = 20i32 as uint8_t
      }
      103 => {
        /* 2^(3*10) */
        G.unit_steps = 30i32 as uint8_t
      }
      _ => {
        bb_show_usage();
      }
    }
  }
  printf(
    b"       %12s%12s%12s%12s%12s%12s\nMem:   \x00" as *const u8 as *const libc::c_char,
    b"total\x00" as *const u8 as *const libc::c_char,
    b"used\x00" as *const u8 as *const libc::c_char,
    b"free\x00" as *const u8 as *const libc::c_char,
    b"shared\x00" as *const u8 as *const libc::c_char,
    b"buff/cache\x00" as *const u8 as *const libc::c_char,
    b"available\x00" as *const u8 as *const libc::c_char,
  );
  sysinfo(&mut info);
  /* Kernels prior to 2.4.x will return info.mem_unit==0, so cope... */
  G.mem_unit = if info.mem_unit != 0 {
    info.mem_unit
  } else {
    1i32 as libc::c_uint
  };
  /* Extract cached and memavailable from /proc/meminfo and convert to mem_units */
  seen_available = parse_meminfo(&mut G) as libc::c_int;
  available = (G.available_kb as libc::c_ulonglong)
    .wrapping_mul(1024i32 as libc::c_ulonglong)
    .wrapping_div(G.mem_unit as libc::c_ulonglong);
  cached = (G.cached_kb as libc::c_ulonglong)
    .wrapping_mul(1024i32 as libc::c_ulonglong)
    .wrapping_div(G.mem_unit as libc::c_ulonglong);
  cached = cached.wrapping_add(info.bufferram as libc::c_ulonglong);
  cached = cached.wrapping_add(
    (G.reclaimable_kb as libc::c_ulonglong)
      .wrapping_mul(1024i32 as libc::c_ulonglong)
      .wrapping_div(G.mem_unit as libc::c_ulonglong),
  );
  cached_plus_free = cached.wrapping_add(info.freeram as libc::c_ulonglong);
  printf(
    b"%12llu %11llu %11llu %11llu %11llu %11llu\n\x00" as *const u8 as *const libc::c_char,
    scale(&mut G, info.totalram),
    scale(
      &mut G,
      (info.totalram as libc::c_ulonglong).wrapping_sub(cached_plus_free) as libc::c_ulong,
    ),
    scale(&mut G, info.freeram),
    scale(&mut G, info.sharedram),
    scale(&mut G, cached as libc::c_ulong),
    scale(&mut G, available as libc::c_ulong),
  );
  /* On kernels < 3.14, MemAvailable is not provided.
   * Show alternate, more meaningful busy/free numbers by counting
   * buffer cache as free memory. */
  if seen_available == 0 {
    printf(b"-/+ buffers/cache: \x00" as *const u8 as *const libc::c_char);
    printf(
      (b"%12llu %11llu %11llu %11llu %11llu %11llu\n\x00" as *const u8 as *const libc::c_char)
        .offset(6)
        .offset(7)
        .offset(7)
        .offset(7),
      scale(
        &mut G,
        (info.totalram as libc::c_ulonglong).wrapping_sub(cached_plus_free) as libc::c_ulong,
      ),
      scale(&mut G, cached_plus_free as libc::c_ulong),
    );
  }
  printf(b"Swap:  \x00" as *const u8 as *const libc::c_char);
  printf(
    (b"%12llu %11llu %11llu %11llu %11llu %11llu\n\x00" as *const u8 as *const libc::c_char)
      .offset(6)
      .offset(7)
      .offset(7),
    scale(&mut G, info.totalswap),
    scale(&mut G, info.totalswap.wrapping_sub(info.freeswap)),
    scale(&mut G, info.freeswap),
  );
  return 0i32;
}
