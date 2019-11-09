use libc;


extern "C" {

  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  static ptr_to_globals: *mut globals;
  #[no_mangle]
  fn fclose(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn fgets_unlocked(
    __s: *mut libc::c_char,
    __n: libc::c_int,
    __stream: *mut FILE,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn puts(__s: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> *mut libc::c_char;
  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn time(__timer: *mut time_t) -> time_t;
  #[no_mangle]
  fn strftime(
    __s: *mut libc::c_char,
    __maxsize: size_t,
    __format: *const libc::c_char,
    __tp: *const tm,
  ) -> size_t;
  #[no_mangle]
  fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
  #[no_mangle]
  fn skip_whitespace(_: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn skip_non_whitespace(_: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn skip_dev_pfx(tty_name: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn bb_clk_tck() -> libc::c_uint;
  #[no_mangle]
  fn xfopen_for_read(path: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn xatoi_positive(numstr: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  static mut option_mask32: u32;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  fn llist_add_to(old_head: *mut *mut llist_t, data: *mut libc::c_void);
  #[no_mangle]
  fn llist_free(
    elm: *mut llist_t,
    freeit: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
  );
  #[no_mangle]
  fn llist_find_str(first: *mut llist_t, str: *const libc::c_char) -> *mut llist_t;
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn starts_with_cpu(str: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn get_cpu_count() -> libc::c_uint;
  #[no_mangle]
  fn uname(__name: *mut utsname) -> libc::c_int;
}

use crate::librb::size_t;
use crate::librb::smallint;
use libc::time_t;

use libc::FILE;
use libc::tm;
use crate::libbb::llist::llist_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub show_all: smallint,
  pub total_cpus: libc::c_uint,
  pub clk_tck: libc::c_uint,
  pub dev_name_list: *mut llist_t,
  pub stats_dev_list: *mut stats_dev_t,
  pub tmtime: tm,
  pub unit: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
  pub str_0: *const libc::c_char,
  pub div: libc::c_uint,
}
pub type stats_dev_t = stats_dev;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stats_dev {
  pub next: *mut stats_dev,
  pub dname: [libc::c_char; 13],
  pub prev_data: stats_dev_data_t,
  pub curr_data: stats_dev_data_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stats_dev_data_t {
  pub rd_sectors: libc::c_ulonglong,
  pub wr_sectors: libc::c_ulonglong,
  pub rd_ops: libc::c_ulong,
  pub wr_ops: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utsname {
  pub sysname: [libc::c_char; 65],
  pub nodename: [libc::c_char; 65],
  pub release: [libc::c_char; 65],
  pub version: [libc::c_char; 65],
  pub machine: [libc::c_char; 65],
  pub domainname: [libc::c_char; 65],
}
pub type cputime_t = libc::c_ulonglong;
pub type icputime_t = libc::c_longlong;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const N_STATS_CPU: C2RustUnnamed_0 = 11;
pub const SMP_UPTIME: C2RustUnnamed_0 = 10;
pub const GLOBAL_UPTIME: C2RustUnnamed_0 = 9;
pub const STATS_CPU_GUEST: C2RustUnnamed_0 = 8;
pub const STATS_CPU_STEAL: C2RustUnnamed_0 = 7;
pub const STATS_CPU_SOFTIRQ: C2RustUnnamed_0 = 6;
pub const STATS_CPU_IRQ: C2RustUnnamed_0 = 5;
pub const STATS_CPU_IOWAIT: C2RustUnnamed_0 = 4;
pub const STATS_CPU_IDLE: C2RustUnnamed_0 = 3;
pub const STATS_CPU_SYSTEM: C2RustUnnamed_0 = 2;
pub const STATS_CPU_NICE: C2RustUnnamed_0 = 1;
pub const STATS_CPU_USER: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stats_cpu_t {
  pub vector: [cputime_t; 11],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stats_cpu_pair_t {
  pub prev: *mut stats_cpu_t,
  pub curr: *mut stats_cpu_t,
  pub itv: cputime_t,
}
/* Must match option string! */
pub type C2RustUnnamed_1 = libc::c_uint;
pub const OPT_m: C2RustUnnamed_1 = 32;
pub const OPT_k: C2RustUnnamed_1 = 16;
pub const OPT_z: C2RustUnnamed_1 = 8;
pub const OPT_t: C2RustUnnamed_1 = 4;
pub const OPT_d: C2RustUnnamed_1 = 2;
pub const OPT_c: C2RustUnnamed_1 = 1;
#[inline(always)]
unsafe extern "C" fn not_const_pp(mut p: *const libc::c_void) -> *mut libc::c_void {
  return p as *mut libc::c_void; /* never fails */
}
#[inline(always)]
unsafe extern "C" fn this_is_smp() -> libc::c_int {
  return ((*ptr_to_globals).total_cpus > 1i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn print_header() {
  let mut buf: [libc::c_char; 32] = [0; 32];
  let mut uts: utsname = utsname {
    sysname: [0; 65],
    nodename: [0; 65],
    release: [0; 65],
    version: [0; 65],
    machine: [0; 65],
    domainname: [0; 65],
  };
  uname(&mut uts);
  /* Date representation for the current locale */
  strftime(
    buf.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
    b"%x\x00" as *const u8 as *const libc::c_char,
    &mut (*ptr_to_globals).tmtime,
  );
  printf(
    b"%s %s (%s) \t%s \t_%s_\t(%u CPU)\n\n\x00" as *const u8 as *const libc::c_char,
    uts.sysname.as_mut_ptr(),
    uts.release.as_mut_ptr(),
    uts.nodename.as_mut_ptr(),
    buf.as_mut_ptr(),
    uts.machine.as_mut_ptr(),
    (*ptr_to_globals).total_cpus,
  );
}
unsafe extern "C" fn get_localtime(mut ptm: *mut tm) {
  let mut timer: time_t = 0;
  time(&mut timer);
  localtime_r(&mut timer, ptm);
}
unsafe extern "C" fn print_timestamp() {
  let mut buf: [libc::c_char; 64] = [0; 64];
  /* %x: date representation for the current locale */
  /* %X: time representation for the current locale */
  strftime(
    buf.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
    b"%x %X\x00" as *const u8 as *const libc::c_char,
    &mut (*ptr_to_globals).tmtime,
  );
  puts(buf.as_mut_ptr());
}
unsafe extern "C" fn get_smp_uptime() -> cputime_t {
  let mut fp: *mut FILE = 0 as *mut FILE;
  let mut sec: libc::c_ulong = 0;
  let mut dec: libc::c_ulong = 0;
  fp = xfopen_for_read(b"/proc/uptime\x00" as *const u8 as *const libc::c_char);
  if fscanf(
    fp,
    b"%lu.%lu\x00" as *const u8 as *const libc::c_char,
    &mut sec as *mut libc::c_ulong,
    &mut dec as *mut libc::c_ulong,
  ) != 2i32
  {
    bb_error_msg_and_die(
      b"can\'t read \'%s\'\x00" as *const u8 as *const libc::c_char,
      b"/proc/uptime\x00" as *const u8 as *const libc::c_char,
    );
  }
  fclose(fp);
  return (sec as cputime_t)
    .wrapping_mul((*ptr_to_globals).clk_tck as libc::c_ulonglong)
    .wrapping_add(
      dec
        .wrapping_mul((*ptr_to_globals).clk_tck as libc::c_ulong)
        .wrapping_div(100i32 as libc::c_ulong) as libc::c_ulonglong,
    );
}
/* Fetch CPU statistics from /proc/stat */
unsafe extern "C" fn get_cpu_statistics(mut sc: *mut stats_cpu_t) {
  let mut fp: *mut FILE = 0 as *mut FILE;
  let mut buf: [libc::c_char; 1024] = [0; 1024];
  fp = xfopen_for_read(b"/proc/stat\x00" as *const u8 as *const libc::c_char);
  memset(
    sc as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<stats_cpu_t>() as libc::c_ulong,
  );
  while !fgets_unlocked(
    buf.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    fp,
  )
  .is_null()
  {
    let mut i: libc::c_int = 0;
    let mut ibuf: *mut libc::c_char = 0 as *mut libc::c_char;
    /* Does the line start with "cpu "? */
    if starts_with_cpu(buf.as_mut_ptr()) == 0 || buf[3] as libc::c_int != ' ' as i32 {
      continue;
    }
    ibuf = buf.as_mut_ptr().offset(4);
    i = STATS_CPU_USER as libc::c_int;
    while i <= STATS_CPU_GUEST as libc::c_int {
      ibuf = skip_whitespace(ibuf);
      sscanf(
        ibuf,
        b"%llu\x00" as *const u8 as *const libc::c_char,
        &mut *(*sc).vector.as_mut_ptr().offset(i as isize) as *mut cputime_t,
      );
      if i != STATS_CPU_GUEST as libc::c_int {
        (*sc).vector[GLOBAL_UPTIME as libc::c_int as usize] =
          ((*sc).vector[GLOBAL_UPTIME as libc::c_int as usize] as libc::c_ulonglong)
            .wrapping_add((*sc).vector[i as usize]) as cputime_t as cputime_t
      }
      ibuf = skip_non_whitespace(ibuf);
      i += 1
    }
    break;
  }
  if this_is_smp() != 0 {
    (*sc).vector[SMP_UPTIME as libc::c_int as usize] = get_smp_uptime()
  }
  fclose(fp);
}
#[inline(always)]
unsafe extern "C" fn get_interval(mut old: cputime_t, mut new: cputime_t) -> cputime_t {
  let mut itv: cputime_t = new.wrapping_sub(old);
  return if itv == 0i32 as libc::c_ulonglong {
    1i32 as libc::c_ulonglong
  } else {
    itv
  };
}
/*
 * Handle overflow conditions properly for counters which can have
 * less bits than cputime_t, depending on the kernel version.
 */
/* Surprisingly, on 32bit inlining is a size win */
#[inline(always)]
unsafe extern "C" fn overflow_safe_sub(mut prev: cputime_t, mut curr: cputime_t) -> cputime_t {
  let mut v: cputime_t = curr.wrapping_sub(prev);
  if (v as icputime_t) < 0i32 as libc::c_longlong && prev <= 0xffffffffu32 as libc::c_ulonglong {
    /* kernel uses 32bit value for the counter? */
    /* Add 33th bit set to 1 to curr, compensating for the overflow */
    /* double shift defeats "warning: left shift count >= width of type" */
    v = (v as libc::c_ulonglong).wrapping_add((1i32 as cputime_t) << 16i32 << 16i32) as cputime_t
      as cputime_t
  }
  return v;
}
unsafe extern "C" fn percent_value(
  mut prev: cputime_t,
  mut curr: cputime_t,
  mut itv: cputime_t,
) -> libc::c_double {
  return overflow_safe_sub(prev, curr) as libc::c_double / itv as libc::c_double
    * 100i32 as libc::c_double;
}
unsafe extern "C" fn print_stats_cpu_struct(mut stats: *mut stats_cpu_pair_t) {
  let mut p: *mut cputime_t = (*(*stats).prev).vector.as_mut_ptr();
  let mut c: *mut cputime_t = (*(*stats).curr).vector.as_mut_ptr();
  printf(
    b"        %7.2f %7.2f %7.2f %7.2f %7.2f %7.2f\n\x00" as *const u8 as *const libc::c_char,
    percent_value(
      *p.offset(STATS_CPU_USER as libc::c_int as isize),
      *c.offset(STATS_CPU_USER as libc::c_int as isize),
      (*stats).itv,
    ),
    percent_value(
      *p.offset(STATS_CPU_NICE as libc::c_int as isize),
      *c.offset(STATS_CPU_NICE as libc::c_int as isize),
      (*stats).itv,
    ),
    percent_value(
      (*p.offset(STATS_CPU_SYSTEM as libc::c_int as isize))
        .wrapping_add(*p.offset(STATS_CPU_SOFTIRQ as libc::c_int as isize))
        .wrapping_add(*p.offset(STATS_CPU_IRQ as libc::c_int as isize)),
      (*c.offset(STATS_CPU_SYSTEM as libc::c_int as isize))
        .wrapping_add(*c.offset(STATS_CPU_SOFTIRQ as libc::c_int as isize))
        .wrapping_add(*c.offset(STATS_CPU_IRQ as libc::c_int as isize)),
      (*stats).itv,
    ),
    percent_value(
      *p.offset(STATS_CPU_IOWAIT as libc::c_int as isize),
      *c.offset(STATS_CPU_IOWAIT as libc::c_int as isize),
      (*stats).itv,
    ),
    percent_value(
      *p.offset(STATS_CPU_STEAL as libc::c_int as isize),
      *c.offset(STATS_CPU_STEAL as libc::c_int as isize),
      (*stats).itv,
    ),
    percent_value(
      *p.offset(STATS_CPU_IDLE as libc::c_int as isize),
      *c.offset(STATS_CPU_IDLE as libc::c_int as isize),
      (*stats).itv,
    ),
  );
}
unsafe extern "C" fn cpu_report(mut stats: *mut stats_cpu_pair_t) {
  /* Always print a header */
  puts(
    b"avg-cpu:  %user   %nice %system %iowait  %steal   %idle\x00" as *const u8
      as *const libc::c_char,
  );
  /* Print current statistics */
  print_stats_cpu_struct(stats);
}
unsafe extern "C" fn print_stats_dev_struct(
  mut stats_dev: *mut stats_dev_t,
  mut itv: cputime_t,
) {
  let mut p: *mut stats_dev_data_t = &mut (*stats_dev).prev_data;
  let mut c: *mut stats_dev_data_t = &mut (*stats_dev).curr_data;
  if option_mask32 & OPT_z as libc::c_int as libc::c_uint != 0 {
    if (*p).rd_ops == (*c).rd_ops && (*p).wr_ops == (*c).wr_ops {
      return;
    }
  }
  printf(
    b"%-13s %8.2f %12.2f %12.2f %10llu %10llu\n\x00" as *const u8 as *const libc::c_char,
    (*stats_dev).dname.as_mut_ptr(),
    percent_value(
      (*p).rd_ops.wrapping_add((*p).wr_ops) as cputime_t,
      (*c).rd_ops.wrapping_add((*c).wr_ops) as cputime_t,
      itv,
    ),
    percent_value((*p).rd_sectors, (*c).rd_sectors, itv)
      / (*ptr_to_globals).unit.div as libc::c_double,
    percent_value((*p).wr_sectors, (*c).wr_sectors, itv)
      / (*ptr_to_globals).unit.div as libc::c_double,
    (*c)
      .rd_sectors
      .wrapping_sub((*p).rd_sectors)
      .wrapping_div((*ptr_to_globals).unit.div as libc::c_ulonglong),
    (*c)
      .wr_sectors
      .wrapping_sub((*p).wr_sectors)
      .wrapping_div((*ptr_to_globals).unit.div as libc::c_ulonglong),
  );
}
unsafe extern "C" fn print_devstat_header() {
  printf(
    b"Device:%15s%6s%s/s%6s%s/s%6s%s%6s%s\n\x00" as *const u8 as *const libc::c_char,
    b"tps\x00" as *const u8 as *const libc::c_char,
    (*ptr_to_globals).unit.str_0,
    b"_read\x00" as *const u8 as *const libc::c_char,
    (*ptr_to_globals).unit.str_0,
    b"_wrtn\x00" as *const u8 as *const libc::c_char,
    (*ptr_to_globals).unit.str_0,
    b"_read\x00" as *const u8 as *const libc::c_char,
    (*ptr_to_globals).unit.str_0,
    b"_wrtn\x00" as *const u8 as *const libc::c_char,
  );
}
/*
 * Is input partition of format [sdaN]?
 */
unsafe extern "C" fn is_partition(mut dev: *const libc::c_char) -> libc::c_int {
  /* Ok, this is naive... */
  return (*dev.offset(0) as libc::c_int - 's' as i32
    | *dev.offset(1) as libc::c_int - 'd' as i32
    | *dev.offset(2) as libc::c_int - 'a' as i32
    == 0i32
    && (*dev.offset(3) as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32)
    as libc::c_int;
}
unsafe extern "C" fn stats_dev_find_or_new(
  mut dev_name: *const libc::c_char,
) -> *mut stats_dev_t {
  let mut curr: *mut *mut stats_dev_t = &mut (*ptr_to_globals).stats_dev_list;
  while !(*curr).is_null() {
    if strcmp((**curr).dname.as_mut_ptr(), dev_name) == 0i32 {
      return *curr;
    }
    curr = &mut (**curr).next
  }
  *curr =
    xzalloc(::std::mem::size_of::<stats_dev_t>() as libc::c_ulong) as *mut stats_dev_t;
  strncpy(
    (**curr).dname.as_mut_ptr(),
    dev_name,
    12i32 as libc::c_ulong,
  );
  return *curr;
}

unsafe extern "C" fn do_disk_statistics(mut itv: cputime_t) {
  let mut buf: [libc::c_char; 128] = [0; 128];
  let mut dev_name: [libc::c_char; 13] = [0; 13];
  let mut rd_sec_or_dummy: libc::c_ulonglong = 0;
  let mut wr_sec_or_dummy: libc::c_ulonglong = 0;
  let mut curr_data: *mut stats_dev_data_t = 0 as *mut stats_dev_data_t;
  let mut stats_dev: *mut stats_dev_t = 0 as *mut stats_dev_t;
  let mut fp: *mut FILE = 0 as *mut FILE;
  let mut rc: libc::c_int = 0;
  fp = xfopen_for_read(b"/proc/diskstats\x00" as *const u8 as *const libc::c_char);
  /* Read and possibly print stats from /proc/diskstats */
  while !fgets_unlocked(
    buf.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
    fp,
  )
  .is_null()
  {
    sscanf(
      buf.as_mut_ptr(),
      b"%*s %*s %12s\x00" as *const u8 as *const libc::c_char,
      dev_name.as_mut_ptr(),
    );
    if !(*ptr_to_globals).dev_name_list.is_null() {
      /* Is device name in list? */
      if llist_find_str((*ptr_to_globals).dev_name_list, dev_name.as_mut_ptr()).is_null() {
        continue;
      }
    } else if is_partition(dev_name.as_mut_ptr()) != 0 {
      continue;
    }
    stats_dev = stats_dev_find_or_new(dev_name.as_mut_ptr());
    curr_data = &mut (*stats_dev).curr_data;
    rc = sscanf(
      buf.as_mut_ptr(),
      b"%*s %*s %*s %lu %llu %llu %llu %lu %*s %llu\x00" as *const u8 as *const libc::c_char,
      &mut (*curr_data).rd_ops as *mut libc::c_ulong,
      &mut rd_sec_or_dummy as *mut libc::c_ulonglong,
      &mut (*curr_data).rd_sectors as *mut libc::c_ulonglong,
      &mut wr_sec_or_dummy as *mut libc::c_ulonglong,
      &mut (*curr_data).wr_ops as *mut libc::c_ulong,
      &mut (*curr_data).wr_sectors as *mut libc::c_ulonglong,
    );
    if rc != 6i32 {
      (*curr_data).rd_sectors = rd_sec_or_dummy;
      (*curr_data).wr_sectors = wr_sec_or_dummy;
      //curr_data->rd_ops = ;
      (*curr_data).wr_ops = (*curr_data).rd_sectors as libc::c_ulong
    }
    if !((*ptr_to_globals).dev_name_list.is_null()
      && (*ptr_to_globals).show_all == 0
      && (*curr_data).rd_ops == 0i32 as libc::c_ulong
      && (*curr_data).wr_ops == 0i32 as libc::c_ulong)
    {
      /* Print current statistics */
      print_stats_dev_struct(stats_dev, itv);
      (*stats_dev).prev_data = *curr_data
    }
  }
  fclose(fp);
}
unsafe extern "C" fn dev_report(mut itv: cputime_t) {
  /* Always print a header */
  print_devstat_header();
  /* Fetch current disk statistics */
  do_disk_statistics(itv);
}
//usage:#define iostat_trivial_usage
//usage:       "[-c] [-d] [-t] [-z] [-k|-m] [ALL|BLOCKDEV...] [INTERVAL [COUNT]]"
//usage:#define iostat_full_usage "\n\n"
//usage:       "Report CPU and I/O statistics\n"
//usage:     "\n	-c	Show CPU utilization"
//usage:     "\n	-d	Show device utilization"
//usage:     "\n	-t	Print current time"
//usage:     "\n	-z	Omit devices with no activity"
//usage:     "\n	-k	Use kb/s"
//usage:     "\n	-m	Use Mb/s"
#[no_mangle]
pub unsafe extern "C" fn iostat_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut opt: libc::c_int = 0;
  let mut interval: libc::c_uint = 0;
  let mut count: libc::c_int = 0;
  let mut stats_data: [stats_cpu_t; 2] = [stats_cpu_t { vector: [0; 11] }; 2];
  let mut current_stats: smallint = 0;
  let ref mut fresh0 = *(not_const_pp(&ptr_to_globals as *const *mut globals as *const libc::c_void)
    as *mut *mut globals);
  *fresh0 = xzalloc(::std::mem::size_of::<globals>() as libc::c_ulong) as *mut globals;
  asm!("" : : : "memory" : "volatile");
  (*ptr_to_globals).unit.str_0 = b"Blk\x00" as *const u8 as *const libc::c_char;
  (*ptr_to_globals).unit.div = 1i32 as libc::c_uint;
  memset(
    &mut stats_data as *mut [stats_cpu_t; 2] as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<[stats_cpu_t; 2]>() as libc::c_ulong,
  );
  /* Get number of clock ticks per sec */
  (*ptr_to_globals).clk_tck = bb_clk_tck();
  /* Determine number of CPUs */
  (*ptr_to_globals).total_cpus = get_cpu_count();
  if (*ptr_to_globals).total_cpus == 0i32 as libc::c_uint {
    (*ptr_to_globals).total_cpus = 1i32 as libc::c_uint
  }
  /* Parse and process arguments */
  /* -k and -m are mutually exclusive */
  opt = getopt32(
    argv,
    b"^cdtzkm\x00k--m:m--k\x00" as *const u8 as *const libc::c_char,
  ) as libc::c_int;
  if opt & OPT_c as libc::c_int + OPT_d as libc::c_int == 0 {
    /* Default is -cd */
    opt |= OPT_c as libc::c_int + OPT_d as libc::c_int
  }
  argv = argv.offset(optind as isize);
  /* Store device names into device list */
  while !(*argv).is_null()
    && !((**argv.offset(0) as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32)
  {
    if strcmp(*argv, b"ALL\x00" as *const u8 as *const libc::c_char) != 0i32 {
      /* If not ALL, save device name */
      let mut dev_name: *mut libc::c_char = skip_dev_pfx(*argv);
      if llist_find_str((*ptr_to_globals).dev_name_list, dev_name).is_null() {
        llist_add_to(
          &mut (*ptr_to_globals).dev_name_list,
          dev_name as *mut libc::c_void,
        );
      }
    } else {
      (*ptr_to_globals).show_all = 1i32 as smallint
    }
    argv = argv.offset(1)
  }
  interval = 0i32 as libc::c_uint;
  count = 1i32;
  if !(*argv).is_null() {
    /* Get interval */
    interval = xatoi_positive(*argv) as libc::c_uint;
    count = if interval != 0i32 as libc::c_uint {
      -1i32
    } else {
      1i32
    };
    argv = argv.offset(1);
    if !(*argv).is_null() {
      /* Get count value */
      count = xatoi_positive(*argv)
    }
  }
  if opt & OPT_m as libc::c_int != 0 {
    (*ptr_to_globals).unit.str_0 = b" MB\x00" as *const u8 as *const libc::c_char;
    (*ptr_to_globals).unit.div = 2048i32 as libc::c_uint
  }
  if opt & OPT_k as libc::c_int != 0 {
    (*ptr_to_globals).unit.str_0 = b" kB\x00" as *const u8 as *const libc::c_char;
    (*ptr_to_globals).unit.div = 2i32 as libc::c_uint
  }
  get_localtime(&mut (*ptr_to_globals).tmtime);
  /* Display header */
  print_header();
  current_stats = 0i32 as smallint;
  loop
  /* Main loop */
  {
    let mut stats: stats_cpu_pair_t = stats_cpu_pair_t {
      prev: 0 as *mut stats_cpu_t,
      curr: 0 as *mut stats_cpu_t,
      itv: 0,
    };
    stats.prev = &mut *stats_data
      .as_mut_ptr()
      .offset((current_stats as libc::c_int ^ 1i32) as isize) as *mut stats_cpu_t;
    stats.curr = &mut *stats_data.as_mut_ptr().offset(current_stats as isize) as *mut stats_cpu_t;
    /* Fill the time structure */
    get_localtime(&mut (*ptr_to_globals).tmtime);
    /* Fetch current CPU statistics */
    get_cpu_statistics(stats.curr);
    /* Get interval */
    stats.itv = get_interval(
      (*stats.prev).vector[GLOBAL_UPTIME as libc::c_int as usize],
      (*stats.curr).vector[GLOBAL_UPTIME as libc::c_int as usize],
    );
    if opt & OPT_t as libc::c_int != 0 {
      print_timestamp();
    }
    if opt & OPT_c as libc::c_int != 0 {
      cpu_report(&mut stats);
      if opt & OPT_d as libc::c_int != 0 {
        /* Separate outputs by a newline */
        bb_putchar('\n' as i32);
      }
    }
    if opt & OPT_d as libc::c_int != 0 {
      if this_is_smp() != 0 {
        stats.itv = get_interval(
          (*stats.prev).vector[SMP_UPTIME as libc::c_int as usize],
          (*stats.curr).vector[SMP_UPTIME as libc::c_int as usize],
        )
      }
      dev_report(stats.itv);
    }
    bb_putchar('\n' as i32);
    if count > 0i32 {
      count -= 1;
      if count == 0i32 {
        break;
      }
    }
    /* Swap stats */
    current_stats = (current_stats as libc::c_int ^ 1i32) as smallint;
    sleep(interval);
  }
  return 0i32;
}
