use libc;
extern "C" {
  #[no_mangle]
  fn strtoul(
    __nptr: *const libc::c_char,
    __endptr: *mut *mut libc::c_char,
    __base: libc::c_int,
  ) -> libc::c_ulong;
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
  #[no_mangle]
  static ptr_to_globals: *mut globals;
  #[no_mangle]
  static mut stdout: *mut _IO_FILE;
  #[no_mangle]
  fn fclose(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn setbuf(__stream: *mut FILE, __buf: *mut libc::c_char);
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
  fn pause() -> libc::c_int;
  #[no_mangle]
  fn alarm(__seconds: libc::c_uint) -> libc::c_uint;
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
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
  fn xmalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn is_prefixed_with(string: *const libc::c_char, key: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn safe_strncpy(
    dst: *mut libc::c_char,
    src: *const libc::c_char,
    size: size_t,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn bb_clk_tck() -> libc::c_uint;
  #[no_mangle]
  fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
  #[no_mangle]
  fn fopen_for_read(path: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn xfopen_for_read(path: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn xatoi_positive(numstr: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn index_in_strings(strings: *const libc::c_char, key: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn starts_with_cpu(str: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn get_cpu_count() -> libc::c_uint;
  #[no_mangle]
  fn uname(__name: *mut utsname) -> libc::c_int;
}
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type uint32_t = __uint32_t;
pub type smallint = libc::c_schar;
pub type size_t = libc::c_ulong;
pub type time_t = __time_t;
pub type __sighandler_t = Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
use crate::librb::_IO_FILE;
pub type _IO_lock_t = ();
use crate::librb::_IO_marker;
use crate::librb::FILE;
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
pub struct globals {
  pub interval: libc::c_int,
  pub count: libc::c_int,
  pub cpu_nr: libc::c_uint,
  pub irqcpu_nr: libc::c_uint,
  pub softirqcpu_nr: libc::c_uint,
  pub options: libc::c_uint,
  pub hz: libc::c_uint,
  pub cpu_bitmap_len: libc::c_uint,
  pub p_option: smallint,
  pub cpu_bitmap: *mut libc::c_uchar,
  pub global_uptime: [data_t; 3],
  pub per_cpu_uptime: [data_t; 3],
  pub st_cpu: [*mut stats_cpu; 3],
  pub st_irq: [*mut stats_irq; 3],
  pub st_irqcpu: [*mut stats_irqcpu; 3],
  pub st_softirqcpu: [*mut stats_irqcpu; 3],
  pub timestamp: [tm; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stats_irqcpu {
  pub interrupts: libc::c_uint,
  pub irq_name: [libc::c_char; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stats_irq {
  pub irq_nr: data_t,
}
pub type data_t = libc::c_ulonglong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stats_cpu {
  pub cpu_user: data_t,
  pub cpu_nice: data_t,
  pub cpu_system: data_t,
  pub cpu_idle: data_t,
  pub cpu_iowait: data_t,
  pub cpu_steal: data_t,
  pub cpu_irq: data_t,
  pub cpu_softirq: data_t,
  pub cpu_guest: data_t,
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
pub type idata_t = libc::c_longlong;
/* The selected interrupts statistics (bits in G.options) */
pub type C2RustUnnamed = libc::c_uint;
pub const D_SOFTIRQS: C2RustUnnamed = 8;
pub const D_IRQ_CPU: C2RustUnnamed = 4;
pub const D_IRQ_SUM: C2RustUnnamed = 2;
pub const D_CPU: C2RustUnnamed = 1;
/* -I */
pub const OPT_SETCPU: C2RustUnnamed_0 = 4;
/* -P */
pub const OPT_UTIL: C2RustUnnamed_0 = 8;
/* -A */
pub const OPT_INTS: C2RustUnnamed_0 = 2;
pub const OPT_ALL: C2RustUnnamed_0 = 1;
pub type C2RustUnnamed_0 = libc::c_uint;
#[inline(always)]
unsafe extern "C" fn not_const_pp(mut p: *const libc::c_void) -> *mut libc::c_void {
  return p as *mut libc::c_void;
}
/* -u */
/* Is option on? */
#[inline(always)]
unsafe extern "C" fn display_opt(mut opt: libc::c_int) -> libc::c_int {
  return (opt as libc::c_uint & (*ptr_to_globals).options) as libc::c_int;
}
/*
 * Handle overflow conditions properly for counters which can have
 * less bits than data_t, depending on the kernel version.
 */
/* Surprisingly, on 32bit inlining is a size win */
#[inline(always)]
unsafe extern "C" fn overflow_safe_sub(mut prev: data_t, mut curr: data_t) -> data_t {
  let mut v: data_t = curr.wrapping_sub(prev);
  if (v as idata_t) < 0i32 as libc::c_longlong && prev <= 0xffffffffu32 as libc::c_ulonglong {
    /* kernel uses 32bit value for the counter? */
    /* Add 33th bit set to 1 to curr, compensating for the overflow */
    /* double shift defeats "warning: left shift count >= width of type" */
    v =
      (v as libc::c_ulonglong).wrapping_add((1i32 as data_t) << 16i32 << 16i32) as data_t as data_t
  }
  return v;
}
unsafe extern "C" fn percent_value(
  mut prev: data_t,
  mut curr: data_t,
  mut itv: data_t,
) -> libc::c_double {
  return overflow_safe_sub(prev, curr) as libc::c_double / itv as libc::c_double
    * 100i32 as libc::c_double;
}
unsafe extern "C" fn hz_value(
  mut prev: data_t,
  mut curr: data_t,
  mut itv: data_t,
) -> libc::c_double {
  //bb_error_msg("curr:%lld prev:%lld G.hz:%u", curr, prev, G.hz);
  return overflow_safe_sub(prev, curr) as libc::c_double / itv as libc::c_double
    * (*ptr_to_globals).hz as libc::c_double;
}
#[inline(always)]
unsafe extern "C" fn jiffies_diff(mut old: data_t, mut new: data_t) -> data_t {
  let mut diff: data_t = new.wrapping_sub(old);
  return if diff == 0i32 as libc::c_ulonglong {
    1i32 as libc::c_ulonglong
  } else {
    diff
  };
}
unsafe extern "C" fn is_cpu_in_bitmap(mut cpu: libc::c_uint) -> libc::c_int {
  return *(*ptr_to_globals).cpu_bitmap.offset((cpu >> 3i32) as isize) as libc::c_int
    & 1i32 << (cpu & 7i32 as libc::c_uint);
}
unsafe extern "C" fn write_irqcpu_stats(
  mut per_cpu_stats: *mut *mut stats_irqcpu,
  mut total_irqs: libc::c_int,
  mut itv: data_t,
  mut prev: libc::c_int,
  mut current: libc::c_int,
  mut prev_str: *const libc::c_char,
  mut current_str: *const libc::c_char,
) {
  let mut j: libc::c_int = 0;
  let mut offset: libc::c_int = 0;
  let mut cpu: libc::c_int = 0;
  let mut p0: *mut stats_irqcpu = 0 as *mut stats_irqcpu;
  let mut q0: *mut stats_irqcpu = 0 as *mut stats_irqcpu;
  /* Check if number of IRQs has changed */
  if (*ptr_to_globals).interval != 0i32 {
    j = 0i32;
    while j <= total_irqs {
      p0 = &mut *(*per_cpu_stats.offset(current as isize)).offset(j as isize) as *mut stats_irqcpu;
      if (*p0).irq_name[0] as libc::c_int != '\u{0}' as i32 {
        q0 = &mut *(*per_cpu_stats.offset(prev as isize)).offset(j as isize) as *mut stats_irqcpu;
        if strcmp((*p0).irq_name.as_mut_ptr(), (*q0).irq_name.as_mut_ptr()) != 0i32 {
          break;
        }
      }
      j += 1
    }
  }
  /* Print header */
  printf(
    b"\n%-11s  CPU\x00" as *const u8 as *const libc::c_char,
    prev_str,
  );
  /* A bit complex code to "buy back" space if one header is too wide.
   * Here's how it looks like. BLOCK_IOPOLL eats too much space,
   * and latter headers use smaller width to compensate:
   * ...BLOCK/s BLOCK_IOPOLL/s TASKLET/s SCHED/s HRTIMER/s  RCU/s
   * ...   2.32      0.00      0.01     17.58      0.14    141.96
   */
  let mut expected_len: libc::c_int = 0i32;
  let mut printed_len: libc::c_int = 0i32;
  j = 0i32;
  while j < total_irqs {
    p0 = &mut *(*per_cpu_stats.offset(current as isize)).offset(j as isize) as *mut stats_irqcpu;
    if (*p0).irq_name[0] as libc::c_int != '\u{0}' as i32 {
      let mut n: libc::c_int = 10i32 - 3i32 - (printed_len - expected_len);
      printed_len += printf(
        b" %*s/s\x00" as *const u8 as *const libc::c_char,
        if n > 0i32 { n } else { 0i32 },
        skip_whitespace((*p0).irq_name.as_mut_ptr()),
      );
      expected_len += 10i32
    }
    j += 1
  }
  bb_putchar('\n' as i32);
  cpu = 1i32;
  while cpu as libc::c_uint <= (*ptr_to_globals).cpu_nr {
    /* Check if we want stats about this CPU */
    if !(is_cpu_in_bitmap(cpu as libc::c_uint) == 0
      && (*ptr_to_globals).p_option as libc::c_int != 0)
    {
      printf(
        b"%-11s %4u\x00" as *const u8 as *const libc::c_char,
        current_str,
        cpu - 1i32,
      );
      j = 0i32;
      while j < total_irqs {
        /* IRQ field set only for proc 0 */
        p0 =
          &mut *(*per_cpu_stats.offset(current as isize)).offset(j as isize) as *mut stats_irqcpu;
        /*
         * An empty string for irq name means that
         * interrupt is no longer used.
         */
        if (*p0).irq_name[0] as libc::c_int != '\u{0}' as i32 {
          offset = j;
          q0 = &mut *(*per_cpu_stats.offset(prev as isize)).offset(offset as isize)
            as *mut stats_irqcpu;
          /*
           * If we want stats for the time since boot
           * we have p0->irq != q0->irq.
           */
          if strcmp((*p0).irq_name.as_mut_ptr(), (*q0).irq_name.as_mut_ptr()) != 0i32
            && (*ptr_to_globals).interval != 0i32
          {
            if j != 0 {
              offset = j - 1i32;
              q0 = &mut *(*per_cpu_stats.offset(prev as isize)).offset(offset as isize)
                as *mut stats_irqcpu
            }
            if strcmp((*p0).irq_name.as_mut_ptr(), (*q0).irq_name.as_mut_ptr()) != 0i32
              && j + 1i32 < total_irqs
            {
              offset = j + 1i32;
              q0 = &mut *(*per_cpu_stats.offset(prev as isize)).offset(offset as isize)
                as *mut stats_irqcpu
            }
          }
          if strcmp((*p0).irq_name.as_mut_ptr(), (*q0).irq_name.as_mut_ptr()) == 0i32
            || (*ptr_to_globals).interval == 0i32
          {
            let mut p: *mut stats_irqcpu = 0 as *mut stats_irqcpu;
            let mut q: *mut stats_irqcpu = 0 as *mut stats_irqcpu;
            p = &mut *(*per_cpu_stats.offset(current as isize))
              .offset(((cpu - 1i32) * total_irqs + j) as isize)
              as *mut stats_irqcpu;
            q = &mut *(*per_cpu_stats.offset(prev as isize))
              .offset(((cpu - 1i32) * total_irqs + offset) as isize)
              as *mut stats_irqcpu;
            printf(
              b"%10.2f\x00" as *const u8 as *const libc::c_char,
              (*p).interrupts.wrapping_sub((*q).interrupts) as libc::c_double
                / itv as libc::c_double
                * (*ptr_to_globals).hz as libc::c_double,
            );
          } else {
            printf(b"        N/A\x00" as *const u8 as *const libc::c_char);
          }
        }
        j += 1
      }
      bb_putchar('\n' as i32);
    }
    cpu += 1
  }
}
unsafe extern "C" fn get_per_cpu_interval(
  mut scc: *const stats_cpu,
  mut scp: *const stats_cpu,
) -> data_t {
  return (*scc)
    .cpu_user
    .wrapping_add((*scc).cpu_nice)
    .wrapping_add((*scc).cpu_system)
    .wrapping_add((*scc).cpu_iowait)
    .wrapping_add((*scc).cpu_idle)
    .wrapping_add((*scc).cpu_steal)
    .wrapping_add((*scc).cpu_irq)
    .wrapping_add((*scc).cpu_softirq)
    .wrapping_sub(
      (*scp)
        .cpu_user
        .wrapping_add((*scp).cpu_nice)
        .wrapping_add((*scp).cpu_system)
        .wrapping_add((*scp).cpu_iowait)
        .wrapping_add((*scp).cpu_idle)
        .wrapping_add((*scp).cpu_steal)
        .wrapping_add((*scp).cpu_irq)
        .wrapping_add((*scp).cpu_softirq),
    );
}
unsafe extern "C" fn print_stats_cpu_struct(
  mut p: *const stats_cpu,
  mut c: *const stats_cpu,
  mut itv: data_t,
) {
  printf(
    b" %7.2f %7.2f %7.2f %7.2f %7.2f %7.2f %7.2f %7.2f %7.2f\n\x00" as *const u8
      as *const libc::c_char,
    percent_value(
      (*p).cpu_user.wrapping_sub((*p).cpu_guest),
      (*c).cpu_user.wrapping_sub((*c).cpu_guest),
      itv,
    ),
    percent_value((*p).cpu_nice, (*c).cpu_nice, itv),
    percent_value((*p).cpu_system, (*c).cpu_system, itv),
    percent_value((*p).cpu_iowait, (*c).cpu_iowait, itv),
    percent_value((*p).cpu_irq, (*c).cpu_irq, itv),
    percent_value((*p).cpu_softirq, (*c).cpu_softirq, itv),
    percent_value((*p).cpu_steal, (*c).cpu_steal, itv),
    percent_value((*p).cpu_guest, (*c).cpu_guest, itv),
    percent_value((*p).cpu_idle, (*c).cpu_idle, itv),
  );
}
unsafe extern "C" fn write_stats_core(
  mut prev: libc::c_int,
  mut current: libc::c_int,
  mut prev_str: *const libc::c_char,
  mut current_str: *const libc::c_char,
) {
  let mut scc: *mut stats_cpu = 0 as *mut stats_cpu;
  let mut scp: *mut stats_cpu = 0 as *mut stats_cpu;
  let mut itv: data_t = 0;
  let mut global_itv: data_t = 0;
  let mut cpu: libc::c_int = 0;
  /* Compute time interval */
  global_itv = jiffies_diff(
    (*ptr_to_globals).global_uptime[prev as usize],
    (*ptr_to_globals).global_uptime[current as usize],
  );
  itv = global_itv;
  /* Reduce interval to one CPU */
  if (*ptr_to_globals).cpu_nr > 1i32 as libc::c_uint {
    itv = jiffies_diff(
      (*ptr_to_globals).per_cpu_uptime[prev as usize],
      (*ptr_to_globals).per_cpu_uptime[current as usize],
    )
  }
  /* Print CPU stats */
  if display_opt(D_CPU as libc::c_int) != 0 {
    // /* This is done exactly once */
    //if (!G.header_done) {
    printf(b"\n%-11s  CPU    %%usr   %%nice    %%sys %%iowait    %%irq   %%soft  %%steal  %%guest   %%idle\n\x00"
                   as *const u8 as *const libc::c_char, prev_str);
    let mut current_block_14: u64;
    //	G.header_done = 1;
    //}
    cpu = 0i32;
    while cpu as libc::c_uint <= (*ptr_to_globals).cpu_nr {
      let mut per_cpu_itv: data_t = 0;
      /* Print stats about this particular CPU? */
      if !(is_cpu_in_bitmap(cpu as libc::c_uint) == 0) {
        scc = &mut *(*(*ptr_to_globals)
          .st_cpu
          .as_mut_ptr()
          .offset(current as isize))
        .offset(cpu as isize) as *mut stats_cpu;
        scp = &mut *(*(*ptr_to_globals).st_cpu.as_mut_ptr().offset(prev as isize))
          .offset(cpu as isize) as *mut stats_cpu;
        per_cpu_itv = global_itv;
        printf(
          if cpu != 0 {
            b"%-11s %4u\x00" as *const u8 as *const libc::c_char
          } else {
            b"%-11s  all\x00" as *const u8 as *const libc::c_char
          },
          current_str,
          cpu - 1i32,
        );
        if cpu != 0 {
          let mut idle: libc::c_double = 0.;
          /*
           * If the CPU is offline, then it isn't in /proc/stat,
           * so all values are 0.
           * NB: Guest time is already included in user time.
           */
          if (*scc).cpu_user
            | (*scc).cpu_nice
            | (*scc).cpu_system
            | (*scc).cpu_iowait
            | (*scc).cpu_idle
            | (*scc).cpu_steal
            | (*scc).cpu_irq
            | (*scc).cpu_softirq
            == 0i32 as libc::c_ulonglong
          {
            /*
             * Set current struct fields to values from prev.
             * iteration. Then their values won't jump from
             * zero, when the CPU comes back online.
             */
            *scc = *scp;
            idle = 0.0f64;
            current_block_14 = 3502820671955397158;
          } else {
            /* Compute interval again for current proc */
            per_cpu_itv = get_per_cpu_interval(scc, scp);
            if per_cpu_itv == 0i32 as libc::c_ulonglong {
              /*
               * If the CPU is tickless then there is no change in CPU values
               * but the sum of values is not zero.
               */
              idle = 100.0f64;
              current_block_14 = 3502820671955397158;
            } else {
              current_block_14 = 14401909646449704462;
            }
          }
          match current_block_14 {
            14401909646449704462 => {}
            _ => {
              printf(
                b" %7.2f %7.2f %7.2f %7.2f %7.2f %7.2f %7.2f %7.2f %7.2f\n\x00" as *const u8
                  as *const libc::c_char,
                0.0f64,
                0.0f64,
                0.0f64,
                0.0f64,
                0.0f64,
                0.0f64,
                0.0f64,
                0.0f64,
                idle,
              );
              current_block_14 = 4906268039856690917;
            }
          }
        } else {
          current_block_14 = 14401909646449704462;
        }
        match current_block_14 {
          4906268039856690917 => {}
          _ => {
            print_stats_cpu_struct(scp, scc, per_cpu_itv);
          }
        }
      }
      cpu += 1
    }
  }
  /* Print total number of IRQs per CPU */
  if display_opt(D_IRQ_SUM as libc::c_int) != 0 {
    // /* Print average header, this is done exactly once */
    //if (!G.avg_header_done) {
    printf(
      b"\n%-11s  CPU    intr/s\n\x00" as *const u8 as *const libc::c_char,
      prev_str,
    );
    let mut current_block_26: u64;
    //	G.avg_header_done = 1;
    //}
    cpu = 0i32;
    while cpu as libc::c_uint <= (*ptr_to_globals).cpu_nr {
      let mut per_cpu_itv_0: data_t = 0;
      /* Print stats about this CPU? */
      if !(is_cpu_in_bitmap(cpu as libc::c_uint) == 0) {
        per_cpu_itv_0 = itv;
        printf(
          if cpu != 0 {
            b"%-11s %4u\x00" as *const u8 as *const libc::c_char
          } else {
            b"%-11s  all\x00" as *const u8 as *const libc::c_char
          },
          current_str,
          cpu - 1i32,
        );
        if cpu != 0 {
          scc = &mut *(*(*ptr_to_globals)
            .st_cpu
            .as_mut_ptr()
            .offset(current as isize))
          .offset(cpu as isize) as *mut stats_cpu;
          scp = &mut *(*(*ptr_to_globals).st_cpu.as_mut_ptr().offset(prev as isize))
            .offset(cpu as isize) as *mut stats_cpu;
          /* Compute interval again for current proc */
          per_cpu_itv_0 = get_per_cpu_interval(scc, scp);
          if per_cpu_itv_0 == 0i32 as libc::c_ulonglong {
            printf(b" %9.2f\n\x00" as *const u8 as *const libc::c_char, 0.0f64);
            current_block_26 = 1608152415753874203;
          } else {
            current_block_26 = 9007357115414505193;
          }
        } else {
          current_block_26 = 9007357115414505193;
        }
        match current_block_26 {
          1608152415753874203 => {}
          _ => {
            //bb_error_msg("G.st_irq[%u][%u].irq_nr:%lld - G.st_irq[%u][%u].irq_nr:%lld",
            // current, cpu, G.st_irq[prev][cpu].irq_nr, prev, cpu, G.st_irq[current][cpu].irq_nr);
            printf(
              b" %9.2f\n\x00" as *const u8 as *const libc::c_char,
              hz_value(
                (*(*ptr_to_globals).st_irq[prev as usize].offset(cpu as isize)).irq_nr,
                (*(*ptr_to_globals).st_irq[current as usize].offset(cpu as isize)).irq_nr,
                per_cpu_itv_0,
              ),
            );
          }
        }
      }
      cpu += 1
    }
  }
  if display_opt(D_IRQ_CPU as libc::c_int) != 0 {
    write_irqcpu_stats(
      (*ptr_to_globals).st_irqcpu.as_mut_ptr(),
      (*ptr_to_globals).irqcpu_nr as libc::c_int,
      itv,
      prev,
      current,
      prev_str,
      current_str,
    );
  }
  if display_opt(D_SOFTIRQS as libc::c_int) != 0 {
    write_irqcpu_stats(
      (*ptr_to_globals).st_softirqcpu.as_mut_ptr(),
      (*ptr_to_globals).softirqcpu_nr as libc::c_int,
      itv,
      prev,
      current,
      prev_str,
      current_str,
    );
  };
}
/*
 * Print the statistics
 */
unsafe extern "C" fn write_stats(mut current: libc::c_int) {
  let mut prev_time: [libc::c_char; 16] = [0; 16];
  let mut curr_time: [libc::c_char; 16] = [0; 16];
  strftime(
    prev_time.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
    b"%X\x00" as *const u8 as *const libc::c_char,
    &mut *(*ptr_to_globals)
      .timestamp
      .as_mut_ptr()
      .offset((current == 0) as libc::c_int as isize),
  );
  strftime(
    curr_time.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
    b"%X\x00" as *const u8 as *const libc::c_char,
    &mut *(*ptr_to_globals)
      .timestamp
      .as_mut_ptr()
      .offset(current as isize),
  );
  write_stats_core(
    (current == 0) as libc::c_int,
    current,
    prev_time.as_mut_ptr(),
    curr_time.as_mut_ptr(),
  );
}
unsafe extern "C" fn write_stats_avg(mut current: libc::c_int) {
  write_stats_core(
    2i32,
    current,
    b"Average:\x00" as *const u8 as *const libc::c_char,
    b"Average:\x00" as *const u8 as *const libc::c_char,
  );
}
/*
 * Read CPU statistics
 */
unsafe extern "C" fn get_cpu_statistics(
  mut cpu: *mut stats_cpu,
  mut up: *mut data_t,
  mut up0: *mut data_t,
) {
  let mut fp: *mut FILE = 0 as *mut FILE; /* not "cpu" */
  let mut buf: [libc::c_char; 1024] = [0; 1024]; /* for "cpu " case */
  fp = xfopen_for_read(b"/proc/stat\x00" as *const u8 as *const libc::c_char);
  while !fgets_unlocked(
    buf.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    fp,
  )
  .is_null()
  {
    let mut sum: data_t = 0;
    let mut cpu_number: libc::c_uint = 0;
    let mut cp: *mut stats_cpu = 0 as *mut stats_cpu;
    if starts_with_cpu(buf.as_mut_ptr()) == 0 {
      continue;
    }
    cp = cpu;
    if buf[3] as libc::c_int != ' ' as i32 {
      /* "cpuN " */
      if (*ptr_to_globals).cpu_nr == 0i32 as libc::c_uint
        || sscanf(
          buf.as_mut_ptr().offset(3),
          b"%u \x00" as *const u8 as *const libc::c_char,
          &mut cpu_number as *mut libc::c_uint,
        ) != 1i32
        || cpu_number >= (*ptr_to_globals).cpu_nr
      {
        continue;
      }
      cp =
        &mut *cpu.offset(cpu_number.wrapping_add(1i32 as libc::c_uint) as isize) as *mut stats_cpu
    }
    /* Read the counters, save them */
    /* Not all fields have to be present */
    memset(
      cp as *mut libc::c_void,
      0i32,
      ::std::mem::size_of::<stats_cpu>() as libc::c_ulong,
    );
    sscanf(
      buf.as_mut_ptr(),
      b"%*s %llu %llu %llu %llu %llu %llu %llu %llu %llu\x00" as *const u8 as *const libc::c_char,
      &mut (*cp).cpu_user as *mut data_t,
      &mut (*cp).cpu_nice as *mut data_t,
      &mut (*cp).cpu_system as *mut data_t,
      &mut (*cp).cpu_idle as *mut data_t,
      &mut (*cp).cpu_iowait as *mut data_t,
      &mut (*cp).cpu_irq as *mut data_t,
      &mut (*cp).cpu_softirq as *mut data_t,
      &mut (*cp).cpu_steal as *mut data_t,
      &mut (*cp).cpu_guest as *mut data_t,
    );
    /*
     * Compute uptime in jiffies (1/HZ), it'll be the sum of
     * individual CPU's uptimes.
     * NB: We have to omit cpu_guest, because cpu_user includes it.
     */
    sum = (*cp)
      .cpu_user
      .wrapping_add((*cp).cpu_nice)
      .wrapping_add((*cp).cpu_system)
      .wrapping_add((*cp).cpu_idle)
      .wrapping_add((*cp).cpu_iowait)
      .wrapping_add((*cp).cpu_irq)
      .wrapping_add((*cp).cpu_softirq)
      .wrapping_add((*cp).cpu_steal);
    if buf[3] as libc::c_int == ' ' as i32 {
      /* "cpu " */
      *up = sum
    } else if cpu_number == 0i32 as libc::c_uint && *up0 != 0i32 as libc::c_ulonglong {
      /* "cpuN " */
      /* Compute uptime of single CPU */
      *up0 = sum
    }
  }
  fclose(fp);
}
/*
 * Read IRQs from /proc/stat
 */
unsafe extern "C" fn get_irqs_from_stat(mut irq: *mut stats_irq) {
  let mut fp: *mut FILE = 0 as *mut FILE;
  let mut buf: [libc::c_char; 1024] = [0; 1024];
  fp = xfopen_for_read(b"/proc/stat\x00" as *const u8 as *const libc::c_char);
  while !fgets_unlocked(
    buf.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    fp,
  )
  .is_null()
  {
    //bb_error_msg("/proc/stat:'%s'", buf);
    if !is_prefixed_with(
      buf.as_mut_ptr(),
      b"intr \x00" as *const u8 as *const libc::c_char,
    )
    .is_null()
    {
      /* Read total number of IRQs since system boot */
      sscanf(
        buf.as_mut_ptr().offset(5),
        b"%llu\x00" as *const u8 as *const libc::c_char,
        &mut (*irq).irq_nr as *mut data_t,
      );
    }
  }
  fclose(fp);
}
/*
 * Read stats from /proc/interrupts or /proc/softirqs
 */
unsafe extern "C" fn get_irqs_from_interrupts(
  mut fname: *const libc::c_char,
  mut per_cpu_stats: *mut *mut stats_irqcpu,
  mut irqs_per_cpu: libc::c_int,
  mut current: libc::c_int,
) {
  let mut fp: *mut FILE = 0 as *mut FILE;
  let mut irq_i: *mut stats_irq = 0 as *mut stats_irq;
  let mut ic: *mut stats_irqcpu = 0 as *mut stats_irqcpu;
  let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut buflen: libc::c_uint = 0;
  let mut cpu: libc::c_uint = 0;
  let mut irq: libc::c_uint = 0;
  let vla = (*ptr_to_globals).cpu_nr as usize;
  let mut cpu_index: Vec<libc::c_int> = ::std::vec::from_elem(0, vla);
  let mut iindex: libc::c_int = 0;
  // Moved to caller.
  // Otherwise reading of /proc/softirqs
  // was resetting counts to 0 after we painstakingly collected them from
  // /proc/interrupts. Which resulted in:
  // 01:32:47 PM  CPU    intr/s
  // 01:32:47 PM  all    591.47
  // 01:32:47 PM    0      0.00 <= ???
  // 01:32:47 PM    1      0.00 <= ???
  //	for (cpu = 1; cpu <= G.cpu_nr; cpu++) {
  //		G.st_irq[current][cpu].irq_nr = 0;
  //		//bb_error_msg("G.st_irq[%u][%u].irq_nr=0", current, cpu);
  //	}
  fp = fopen_for_read(fname);
  if fp.is_null() {
    return;
  }
  buflen = (64i32 as libc::c_uint)
    .wrapping_add((16i32 as libc::c_uint).wrapping_mul((*ptr_to_globals).cpu_nr));
  buf = xmalloc(buflen as size_t) as *mut libc::c_char;
  /* Parse header and determine, which CPUs are online */
  iindex = 0i32;
  while !fgets_unlocked(buf, buflen as libc::c_int, fp).is_null() {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
    next = buf;
    loop {
      cp = strstr(next, b"CPU\x00" as *const u8 as *const libc::c_char);
      if !(!cp.is_null() && (iindex as libc::c_uint) < (*ptr_to_globals).cpu_nr) {
        break;
      }
      cpu = strtoul(cp.offset(3), &mut next, 10i32) as libc::c_uint;
      let fresh0 = iindex;
      iindex = iindex + 1;
      *cpu_index.as_mut_ptr().offset(fresh0 as isize) = cpu as libc::c_int
    }
    if iindex != 0 {
      break;
    }
  }
  irq = 0i32 as libc::c_uint;
  while !fgets_unlocked(buf, buflen as libc::c_int, fp).is_null()
    && irq < irqs_per_cpu as libc::c_uint
  {
    let mut len: libc::c_int = 0;
    let mut last_char: libc::c_char = 0;
    let mut cp_0: *mut libc::c_char = 0 as *mut libc::c_char;
    /* Skip over "IRQNAME:" */
    cp_0 = strchr(buf, ':' as i32);
    if cp_0.is_null() {
      continue;
    }
    last_char = *cp_0.offset(-1i32 as isize);
    ic = &mut *(*per_cpu_stats.offset(current as isize)).offset(irq as isize) as *mut stats_irqcpu;
    len = cp_0.wrapping_offset_from(buf) as libc::c_long as libc::c_int;
    if len as libc::c_ulong >= ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong {
      len = (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong) as libc::c_int
    }
    safe_strncpy((*ic).irq_name.as_mut_ptr(), buf, (len + 1i32) as size_t);
    //bb_error_msg("%s: irq%d:'%s' buf:'%s'", fname, irq, ic->irq_name, buf);
    cp_0 = cp_0.offset(1);
    cpu = 0i32 as libc::c_uint;
    while cpu < iindex as libc::c_uint {
      let mut next_0: *mut libc::c_char = 0 as *mut libc::c_char;
      ic = &mut *(*per_cpu_stats.offset(current as isize)).offset(
        ((*cpu_index.as_mut_ptr().offset(cpu as isize) * irqs_per_cpu) as libc::c_uint)
          .wrapping_add(irq) as isize,
      ) as *mut stats_irqcpu;
      irq_i = &mut *(*(*ptr_to_globals)
        .st_irq
        .as_mut_ptr()
        .offset(current as isize))
      .offset((*cpu_index.as_mut_ptr().offset(cpu as isize) + 1i32) as isize)
        as *mut stats_irq;
      (*ic).interrupts = strtoul(cp_0, &mut next_0, 10i32) as libc::c_uint;
      /* Count only numerical IRQs */
      if (last_char as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32 {
        (*irq_i).irq_nr = ((*irq_i).irq_nr as libc::c_ulonglong)
          .wrapping_add((*ic).interrupts as libc::c_ulonglong) as data_t
          as data_t
        //bb_error_msg("G.st_irq[%u][%u].irq_nr + %u = %lld",
        // current, cpu_index[cpu] + 1, ic->interrupts, irq_i->irq_nr);
      }
      cp_0 = next_0;
      cpu = cpu.wrapping_add(1)
    }
    irq = irq.wrapping_add(1)
  }
  fclose(fp);
  free(buf as *mut libc::c_void);
  while irq < irqs_per_cpu as libc::c_uint {
    /* Number of interrupts per CPU has changed */
    ic = &mut *(*per_cpu_stats.offset(current as isize)).offset(irq as isize) as *mut stats_irqcpu; /* False interrupt */
    (*ic).irq_name[0] = '\u{0}' as i32 as libc::c_char; /* enough for long.long */
    irq = irq.wrapping_add(1)
  }
}
unsafe extern "C" fn get_uptime(mut uptime: *mut data_t) {
  let mut fp: *mut FILE = 0 as *mut FILE;
  let mut buf: [libc::c_char; 52] = [0; 52];
  let mut uptime_sec: libc::c_ulong = 0;
  let mut decimal: libc::c_ulong = 0;
  fp = xfopen_for_read(b"/proc/uptime\x00" as *const u8 as *const libc::c_char);
  if !fgets_unlocked(
    buf.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_char; 52]>() as libc::c_ulong as libc::c_int,
    fp,
  )
  .is_null()
  {
    if sscanf(
      buf.as_mut_ptr(),
      b"%lu.%lu\x00" as *const u8 as *const libc::c_char,
      &mut uptime_sec as *mut libc::c_ulong,
      &mut decimal as *mut libc::c_ulong,
    ) == 2i32
    {
      *uptime = (uptime_sec as data_t)
        .wrapping_mul((*ptr_to_globals).hz as libc::c_ulonglong)
        .wrapping_add(
          decimal
            .wrapping_mul((*ptr_to_globals).hz as libc::c_ulong)
            .wrapping_div(100i32 as libc::c_ulong) as libc::c_ulonglong,
        )
    }
  }
  fclose(fp);
}
unsafe extern "C" fn get_localtime(mut tm: *mut tm) {
  let mut timer: time_t = 0;
  time(&mut timer);
  localtime_r(&mut timer, tm);
}
unsafe extern "C" fn alarm_handler(mut _sig: libc::c_int) {
  signal(
    14i32,
    Some(alarm_handler as unsafe extern "C" fn(_: libc::c_int) -> ()),
  );
  alarm((*ptr_to_globals).interval as libc::c_uint);
}
unsafe extern "C" fn main_loop() {
  let mut current: libc::c_uint = 0;
  let mut cpus: libc::c_uint = 0;
  /* Read the stats */
  if (*ptr_to_globals).cpu_nr > 1i32 as libc::c_uint {
    (*ptr_to_globals).per_cpu_uptime[0] = 0i32 as data_t;
    get_uptime(&mut *(*ptr_to_globals).per_cpu_uptime.as_mut_ptr().offset(0));
  }
  get_cpu_statistics(
    (*ptr_to_globals).st_cpu[0],
    &mut *(*ptr_to_globals).global_uptime.as_mut_ptr().offset(0),
    &mut *(*ptr_to_globals).per_cpu_uptime.as_mut_ptr().offset(0),
  );
  if display_opt(D_IRQ_SUM as libc::c_int) != 0 {
    get_irqs_from_stat((*ptr_to_globals).st_irq[0]);
  }
  if display_opt(D_IRQ_SUM as libc::c_int | D_IRQ_CPU as libc::c_int) != 0 {
    get_irqs_from_interrupts(
      b"/proc/interrupts\x00" as *const u8 as *const libc::c_char,
      (*ptr_to_globals).st_irqcpu.as_mut_ptr(),
      (*ptr_to_globals).irqcpu_nr as libc::c_int,
      0i32,
    );
  }
  if display_opt(D_SOFTIRQS as libc::c_int) != 0 {
    get_irqs_from_interrupts(
      b"/proc/softirqs\x00" as *const u8 as *const libc::c_char,
      (*ptr_to_globals).st_softirqcpu.as_mut_ptr(),
      (*ptr_to_globals).softirqcpu_nr as libc::c_int,
      0i32,
    );
  }
  if (*ptr_to_globals).interval == 0i32 {
    /* Display since boot time */
    cpus = (*ptr_to_globals).cpu_nr.wrapping_add(1i32 as libc::c_uint);
    (*ptr_to_globals).timestamp[1] = (*ptr_to_globals).timestamp[0];
    memset(
      (*ptr_to_globals).st_cpu[1] as *mut libc::c_void,
      0i32,
      (::std::mem::size_of::<stats_cpu>() as libc::c_ulong).wrapping_mul(cpus as libc::c_ulong),
    );
    memset(
      (*ptr_to_globals).st_irq[1] as *mut libc::c_void,
      0i32,
      (::std::mem::size_of::<stats_irq>() as libc::c_ulong).wrapping_mul(cpus as libc::c_ulong),
    );
    memset(
      (*ptr_to_globals).st_irqcpu[1] as *mut libc::c_void,
      0i32,
      (::std::mem::size_of::<stats_irqcpu>() as libc::c_ulong)
        .wrapping_mul(cpus as libc::c_ulong)
        .wrapping_mul((*ptr_to_globals).irqcpu_nr as libc::c_ulong),
    );
    memset(
      (*ptr_to_globals).st_softirqcpu[1] as *mut libc::c_void,
      0i32,
      (::std::mem::size_of::<stats_irqcpu>() as libc::c_ulong)
        .wrapping_mul(cpus as libc::c_ulong)
        .wrapping_mul((*ptr_to_globals).softirqcpu_nr as libc::c_ulong),
    );
    write_stats(0i32);
    /* And we're done */
    return;
  }
  /* Set a handler for SIGALRM */
  alarm_handler(0i32);
  /* Save the stats we already have. We need them to compute the average */
  (*ptr_to_globals).timestamp[2] = (*ptr_to_globals).timestamp[0];
  (*ptr_to_globals).global_uptime[2] = (*ptr_to_globals).global_uptime[0];
  (*ptr_to_globals).per_cpu_uptime[2] = (*ptr_to_globals).per_cpu_uptime[0];
  cpus = (*ptr_to_globals).cpu_nr.wrapping_add(1i32 as libc::c_uint);
  memcpy(
    (*ptr_to_globals).st_cpu[2] as *mut libc::c_void,
    (*ptr_to_globals).st_cpu[0] as *const libc::c_void,
    (::std::mem::size_of::<stats_cpu>() as libc::c_ulong).wrapping_mul(cpus as libc::c_ulong),
  );
  memcpy(
    (*ptr_to_globals).st_irq[2] as *mut libc::c_void,
    (*ptr_to_globals).st_irq[0] as *const libc::c_void,
    (::std::mem::size_of::<stats_irq>() as libc::c_ulong).wrapping_mul(cpus as libc::c_ulong),
  );
  memcpy(
    (*ptr_to_globals).st_irqcpu[2] as *mut libc::c_void,
    (*ptr_to_globals).st_irqcpu[0] as *const libc::c_void,
    (::std::mem::size_of::<stats_irqcpu>() as libc::c_ulong)
      .wrapping_mul(cpus as libc::c_ulong)
      .wrapping_mul((*ptr_to_globals).irqcpu_nr as libc::c_ulong),
  );
  if display_opt(D_SOFTIRQS as libc::c_int) != 0 {
    memcpy(
      (*ptr_to_globals).st_softirqcpu[2] as *mut libc::c_void,
      (*ptr_to_globals).st_softirqcpu[0] as *const libc::c_void,
      (::std::mem::size_of::<stats_irqcpu>() as libc::c_ulong)
        .wrapping_mul(cpus as libc::c_ulong)
        .wrapping_mul((*ptr_to_globals).softirqcpu_nr as libc::c_ulong),
    );
  }
  current = 1i32 as libc::c_uint;
  loop {
    /* Suspend until a signal is received */
    pause();
    /* Set structures to 0 to distinguish off/online CPUs */
    memset(
      &mut *(*(*ptr_to_globals)
        .st_cpu
        .as_mut_ptr()
        .offset(current as isize))
      .offset(1) as *mut stats_cpu as *mut libc::c_void,
      0i32,
      (::std::mem::size_of::<stats_cpu>() as libc::c_ulong)
        .wrapping_mul((*ptr_to_globals).cpu_nr as libc::c_ulong),
    );
    get_localtime(
      &mut *(*ptr_to_globals)
        .timestamp
        .as_mut_ptr()
        .offset(current as isize),
    );
    /* Read stats */
    if (*ptr_to_globals).cpu_nr > 1i32 as libc::c_uint {
      (*ptr_to_globals).per_cpu_uptime[current as usize] = 0i32 as data_t;
      get_uptime(
        &mut *(*ptr_to_globals)
          .per_cpu_uptime
          .as_mut_ptr()
          .offset(current as isize),
      );
    }
    get_cpu_statistics(
      (*ptr_to_globals).st_cpu[current as usize],
      &mut *(*ptr_to_globals)
        .global_uptime
        .as_mut_ptr()
        .offset(current as isize),
      &mut *(*ptr_to_globals)
        .per_cpu_uptime
        .as_mut_ptr()
        .offset(current as isize),
    );
    if display_opt(D_IRQ_SUM as libc::c_int) != 0 {
      get_irqs_from_stat((*ptr_to_globals).st_irq[current as usize]);
    }
    if display_opt(D_IRQ_SUM as libc::c_int | D_IRQ_CPU as libc::c_int) != 0 {
      let mut cpu: libc::c_int = 0;
      cpu = 1i32;
      while cpu as libc::c_uint <= (*ptr_to_globals).cpu_nr {
        (*(*ptr_to_globals).st_irq[current as usize].offset(cpu as isize)).irq_nr = 0i32 as data_t;
        cpu += 1
      }
      /* accumulates .irq_nr */
      get_irqs_from_interrupts(
        b"/proc/interrupts\x00" as *const u8 as *const libc::c_char,
        (*ptr_to_globals).st_irqcpu.as_mut_ptr(),
        (*ptr_to_globals).irqcpu_nr as libc::c_int,
        current as libc::c_int,
      );
    }
    if display_opt(D_SOFTIRQS as libc::c_int) != 0 {
      get_irqs_from_interrupts(
        b"/proc/softirqs\x00" as *const u8 as *const libc::c_char,
        (*ptr_to_globals).st_softirqcpu.as_mut_ptr(),
        (*ptr_to_globals).softirqcpu_nr as libc::c_int,
        current as libc::c_int,
      );
    }
    write_stats(current as libc::c_int);
    if (*ptr_to_globals).count > 0i32 {
      (*ptr_to_globals).count -= 1;
      if (*ptr_to_globals).count == 0i32 {
        break;
      }
    }
    current ^= 1i32 as libc::c_uint
  }
  /* Print average statistics */
  write_stats_avg(current as libc::c_int);
}
/* Initialization */
unsafe extern "C" fn alloc_struct(mut cpus: libc::c_int) {
  let mut i: libc::c_int = 0;
  i = 0i32;
  while i < 3i32 {
    (*ptr_to_globals).st_cpu[i as usize] = xzalloc(
      (::std::mem::size_of::<stats_cpu>() as libc::c_ulong).wrapping_mul(cpus as libc::c_ulong),
    ) as *mut stats_cpu;
    (*ptr_to_globals).st_irq[i as usize] = xzalloc(
      (::std::mem::size_of::<stats_irq>() as libc::c_ulong).wrapping_mul(cpus as libc::c_ulong),
    ) as *mut stats_irq;
    (*ptr_to_globals).st_irqcpu[i as usize] = xzalloc(
      (::std::mem::size_of::<stats_irqcpu>() as libc::c_ulong)
        .wrapping_mul(cpus as libc::c_ulong)
        .wrapping_mul((*ptr_to_globals).irqcpu_nr as libc::c_ulong),
    ) as *mut stats_irqcpu;
    (*ptr_to_globals).st_softirqcpu[i as usize] = xzalloc(
      (::std::mem::size_of::<stats_irqcpu>() as libc::c_ulong)
        .wrapping_mul(cpus as libc::c_ulong)
        .wrapping_mul((*ptr_to_globals).softirqcpu_nr as libc::c_ulong),
    ) as *mut stats_irqcpu;
    i += 1
  }
  (*ptr_to_globals).cpu_bitmap_len = ((cpus >> 3i32) + 1i32) as libc::c_uint;
  (*ptr_to_globals).cpu_bitmap =
    xzalloc((*ptr_to_globals).cpu_bitmap_len as size_t) as *mut libc::c_uchar;
}
unsafe extern "C" fn print_header(mut t: *mut tm) {
  let mut cur_date: [libc::c_char; 16] = [0; 16];
  let mut uts: utsname = utsname {
    sysname: [0; 65],
    nodename: [0; 65],
    release: [0; 65],
    version: [0; 65],
    machine: [0; 65],
    domainname: [0; 65],
  };
  /* Get system name, release number and hostname */
  uname(&mut uts);
  strftime(
    cur_date.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
    b"%x\x00" as *const u8 as *const libc::c_char,
    t,
  );
  printf(
    b"%s %s (%s)\t%s\t_%s_\t(%u CPU)\n\x00" as *const u8 as *const libc::c_char,
    uts.sysname.as_mut_ptr(),
    uts.release.as_mut_ptr(),
    uts.nodename.as_mut_ptr(),
    cur_date.as_mut_ptr(),
    uts.machine.as_mut_ptr(),
    (*ptr_to_globals).cpu_nr,
  );
}
/*
 * Get number of interrupts available per processor
 */
unsafe extern "C" fn get_irqcpu_nr(
  mut f: *const libc::c_char,
  mut max_irqs: libc::c_int,
) -> libc::c_int {
  let mut fp: *mut FILE = 0 as *mut FILE;
  let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut linelen: libc::c_uint = 0;
  let mut irq: libc::c_uint = 0;
  fp = fopen_for_read(f);
  if fp.is_null() {
    /* No interrupts file */
    return 0i32;
  }
  linelen = (64i32 as libc::c_uint)
    .wrapping_add((16i32 as libc::c_uint).wrapping_mul((*ptr_to_globals).cpu_nr));
  line = xmalloc(linelen as size_t) as *mut libc::c_char;
  irq = 0i32 as libc::c_uint;
  while !fgets_unlocked(line, linelen as libc::c_int, fp).is_null()
    && irq < max_irqs as libc::c_uint
  {
    let mut p: libc::c_int =
      strcspn(line, b":\x00" as *const u8 as *const libc::c_char) as libc::c_int;
    if p > 0i32 && p < 16i32 {
      irq = irq.wrapping_add(1)
    }
  }
  fclose(fp);
  free(line as *mut libc::c_void);
  return irq as libc::c_int;
}
//usage:#define mpstat_trivial_usage
//usage:       "[-A] [-I SUM|CPU|ALL|SCPU] [-u] [-P num|ALL] [INTERVAL [COUNT]]"
//usage:#define mpstat_full_usage "\n\n"
//usage:       "Per-processor statistics\n"
//usage:     "\n	-A			Same as -I ALL -u -P ALL"
//usage:     "\n	-I SUM|CPU|ALL|SCPU	Report interrupt statistics"
//usage:     "\n	-P num|ALL		Processor to monitor"
//usage:     "\n	-u			Report CPU utilization"
#[no_mangle]
pub unsafe extern "C" fn mpstat_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut opt_irq_fmt: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut opt_set_cpu: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut i: libc::c_int = 0;
  let mut opt: libc::c_int = 0;
  /* Dont buffer data if redirected to a pipe */
  setbuf(stdout, 0 as *mut libc::c_char);
  let ref mut fresh1 = *(not_const_pp(&ptr_to_globals as *const *mut globals as *const libc::c_void)
    as *mut *mut globals);
  *fresh1 = xzalloc(::std::mem::size_of::<globals>() as libc::c_ulong) as *mut globals;
  asm!("" : : : "memory" : "volatile");
  (*ptr_to_globals).interval = -1i32;
  /* Get number of processors */
  (*ptr_to_globals).cpu_nr = get_cpu_count();
  /* Get number of clock ticks per sec */
  (*ptr_to_globals).hz = bb_clk_tck();
  /* Calculate number of interrupts per processor */
  (*ptr_to_globals).irqcpu_nr = (get_irqcpu_nr(
    b"/proc/interrupts\x00" as *const u8 as *const libc::c_char,
    256i32,
  ) + 3i32) as libc::c_uint;
  /* Calculate number of soft interrupts per processor */
  (*ptr_to_globals).softirqcpu_nr = (get_irqcpu_nr(
    b"/proc/softirqs\x00" as *const u8 as *const libc::c_char,
    256i32,
  ) + 3i32) as libc::c_uint;
  /* Allocate space for structures. + 1 for global structure. */
  alloc_struct((*ptr_to_globals).cpu_nr.wrapping_add(1i32 as libc::c_uint) as libc::c_int);
  /* Parse and process arguments */
  opt = getopt32(
    argv,
    b"AI:P:u\x00" as *const u8 as *const libc::c_char,
    &mut opt_irq_fmt as *mut *mut libc::c_char,
    &mut opt_set_cpu as *mut *mut libc::c_char,
  ) as libc::c_int;
  argv = argv.offset(optind as isize);
  if !(*argv).is_null() {
    /* Get interval */
    (*ptr_to_globals).interval = xatoi_positive(*argv);
    (*ptr_to_globals).count = -1i32;
    argv = argv.offset(1);
    if !(*argv).is_null() {
      /* Get count value */
      if (*ptr_to_globals).interval == 0i32 {
        bb_show_usage();
      }
      (*ptr_to_globals).count = xatoi_positive(*argv)
      //if (*++argv)
      //	bb_show_usage();
    }
  }
  if (*ptr_to_globals).interval < 0i32 {
    (*ptr_to_globals).interval = 0i32
  }
  if opt & OPT_ALL as libc::c_int != 0 {
    (*ptr_to_globals).p_option = 1i32 as smallint;
    (*ptr_to_globals).options |= (D_CPU as libc::c_int
      + D_IRQ_SUM as libc::c_int
      + D_IRQ_CPU as libc::c_int
      + D_SOFTIRQS as libc::c_int) as libc::c_uint;
    /* Select every CPU */
    memset(
      (*ptr_to_globals).cpu_bitmap as *mut libc::c_void,
      0xffi32,
      (*ptr_to_globals).cpu_bitmap_len as libc::c_ulong,
    );
  }
  if opt & OPT_INTS as libc::c_int != 0 {
    static mut v: [libc::c_char; 4] = [
      D_IRQ_CPU as libc::c_int as libc::c_char,
      D_IRQ_SUM as libc::c_int as libc::c_char,
      D_SOFTIRQS as libc::c_int as libc::c_char,
      (D_IRQ_SUM as libc::c_int + D_IRQ_CPU as libc::c_int + D_SOFTIRQS as libc::c_int)
        as libc::c_char,
    ];
    i = index_in_strings(
      b"CPU\x00SUM\x00SCPU\x00ALL\x00\x00" as *const u8 as *const libc::c_char,
      opt_irq_fmt,
    );
    if i == -1i32 {
      bb_show_usage();
    }
    (*ptr_to_globals).options |= v[i as usize] as libc::c_uint
  }
  if opt & OPT_UTIL as libc::c_int != 0 || (*ptr_to_globals).options == 0i32 as libc::c_uint {
    /* nothing? (use default then) */
    (*ptr_to_globals).options |= D_CPU as libc::c_int as libc::c_uint
  }
  if opt & OPT_SETCPU as libc::c_int != 0 {
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    (*ptr_to_globals).p_option = 1i32 as smallint;
    t = strtok(opt_set_cpu, b",\x00" as *const u8 as *const libc::c_char);
    while !t.is_null() {
      if strcmp(t, b"ALL\x00" as *const u8 as *const libc::c_char) == 0i32 {
        /* Select every CPU */
        memset(
          (*ptr_to_globals).cpu_bitmap as *mut libc::c_void,
          0xffi32,
          (*ptr_to_globals).cpu_bitmap_len as libc::c_ulong,
        );
      } else {
        /* Get CPU number */
        let mut n: libc::c_uint = xatoi_positive(t) as libc::c_uint;
        if n >= (*ptr_to_globals).cpu_nr {
          bb_simple_error_msg_and_die(
            b"not that many processors\x00" as *const u8 as *const libc::c_char,
          );
        }
        n = n.wrapping_add(1);
        let ref mut fresh2 = *(*ptr_to_globals).cpu_bitmap.offset((n >> 3i32) as isize);
        *fresh2 = (*fresh2 as libc::c_int | 1i32 << (n & 7i32 as libc::c_uint)) as libc::c_uchar
      }
      t = strtok(
        0 as *mut libc::c_char,
        b",\x00" as *const u8 as *const libc::c_char,
      )
    }
  }
  if (*ptr_to_globals).p_option == 0 {
    /* Display global stats */
    *(*ptr_to_globals).cpu_bitmap.offset(0) = 1i32 as libc::c_uchar
  }
  /* Get time */
  get_localtime(&mut *(*ptr_to_globals).timestamp.as_mut_ptr().offset(0));
  /* Display header */
  print_header(&mut *(*ptr_to_globals).timestamp.as_mut_ptr().offset(0));
  /* The main loop */
  main_loop();
  return 0i32;
}
