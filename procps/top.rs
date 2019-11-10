use libc;
use libc::access;
use libc::atoi;
use libc::fclose;
use libc::fprintf;
use libc::lstat;
use libc::printf;
use libc::puts;
use libc::rename;
use libc::rmdir;
use libc::sprintf;
use libc::strchr;
use libc::strcmp;
use libc::strrchr;
use libc::strstr;
use libc::system;



use libc::free;
extern "C" {

  #[no_mangle]
  fn strtoul(
    __nptr: *const libc::c_char,
    __endptr: *mut *mut libc::c_char,
    __base: libc::c_int,
  ) -> libc::c_ulong;

  #[no_mangle]
  fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t, __compar: __compar_fn_t);
  #[no_mangle]
  fn div(__numer: libc::c_int, __denom: libc::c_int) -> div_t;
  #[no_mangle]
  static mut stdout: *mut FILE;


  #[no_mangle]
  fn snprintf(
    _: *mut libc::c_char,
    _: libc::c_ulong,
    _: *const libc::c_char,
    _: ...
  ) -> libc::c_int;
  #[no_mangle]
  fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn fgets_unlocked(
    __s: *mut libc::c_char,
    __n: libc::c_int,
    __stream: *mut FILE,
  ) -> *mut libc::c_char;

  #[no_mangle]
  fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn usleep(__useconds: useconds_t) -> libc::c_int;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;

  #[no_mangle]
  fn strchrnul(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn stpcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn xmalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xrealloc_vector_helper(
    vector: *mut libc::c_void,
    sizeof_and_shift: libc::c_uint,
    idx: libc::c_int,
  ) -> *mut libc::c_void;
  #[no_mangle]
  fn bb_signals(sigs: libc::c_int, f: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>);
  #[no_mangle]
  fn kill_myself_with_sig(sig: libc::c_int) -> !;
  #[no_mangle]
  fn xchdir(path: *const libc::c_char);
  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn open_read_close(
    filename: *const libc::c_char,
    buf: *mut libc::c_void,
    maxsz: size_t,
  ) -> ssize_t;
  #[no_mangle]
  fn fflush_all() -> libc::c_int;
  #[no_mangle]
  fn xfopen_for_read(path: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn utoa(n: libc::c_uint) -> *mut libc::c_char;
  #[no_mangle]
  fn smart_ulltoa5(
    ul: libc::c_ulonglong,
    buf: *mut libc::c_char,
    scale: *const libc::c_char,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn xatou(str: *const libc::c_char) -> libc::c_uint;
  #[no_mangle]
  fn sleep_for_duration(duration: duration_t);
  #[no_mangle]
  fn parse_duration_str(str: *mut libc::c_char) -> duration_t;
  #[no_mangle]
  fn get_cached_username(uid: uid_t) -> *const libc::c_char;
  #[no_mangle]
  fn clear_username_cache();
  #[no_mangle]
  fn make_all_argv_opts(argv: *mut *mut libc::c_char);
  #[no_mangle]
  static mut option_mask32: u32;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  static mut die_func: Option<unsafe extern "C" fn() -> ()>;
  #[no_mangle]
  fn bb_simple_error_msg(s: *const libc::c_char);
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn index_in_strings(strings: *const libc::c_char, key: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn get_terminal_width_height(
    fd: libc::c_int,
    width: *mut libc::c_uint,
    height: *mut libc::c_uint,
  ) -> libc::c_int;
  #[no_mangle]
  fn tcsetattr_stdin_TCSANOW(tp: *const termios) -> libc::c_int;
  #[no_mangle]
  fn set_termios_to_raw(fd: libc::c_int, oldterm: *mut termios, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn read_key(fd: libc::c_int, buffer: *mut libc::c_char, timeout: libc::c_int) -> int64_t;
  #[no_mangle]
  fn procps_scan(sp: *mut procps_status_t, flags: libc::c_int) -> *mut procps_status_t;
  #[no_mangle]
  fn read_cmdline(
    buf: *mut libc::c_char,
    size: libc::c_int,
    pid: libc::c_uint,
    comm: *const libc::c_char,
  );
  #[no_mangle]
  static ptr_to_globals: *mut globals;
}

pub type __int64_t = libc::c_long;

use libc::useconds_t;

pub type int64_t = __int64_t;
use crate::librb::size_t;
use crate::librb::smallint;
use libc::pid_t;
use libc::ssize_t;
use libc::uid_t;

use libc::DIR;

use libc::FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct div_t {
  pub quot: libc::c_int,
  pub rem: libc::c_int,
}
use crate::librb::__compar_fn_t;

use libc::termios;
pub type C2RustUnnamed = libc::c_uint;
pub const BB_FATAL_SIGS: C2RustUnnamed = 117503054;
pub type duration_t = libc::c_double;
pub type C2RustUnnamed_0 = libc::c_int;
pub const KEYCODE_BUFFER_SIZE: C2RustUnnamed_0 = 16;
pub const KEYCODE_CURSOR_POS: C2RustUnnamed_0 = -256;
pub const KEYCODE_ALT_D: C2RustUnnamed_0 = -45;
pub const KEYCODE_ALT_BACKSPACE: C2RustUnnamed_0 = -44;
pub const KEYCODE_ALT_LEFT: C2RustUnnamed_0 = -37;
pub const KEYCODE_ALT_RIGHT: C2RustUnnamed_0 = -36;
pub const KEYCODE_CTRL_LEFT: C2RustUnnamed_0 = -69;
pub const KEYCODE_CTRL_RIGHT: C2RustUnnamed_0 = -68;
pub const KEYCODE_D: C2RustUnnamed_0 = -13;
pub const KEYCODE_BACKSPACE: C2RustUnnamed_0 = -12;
pub const KEYCODE_PAGEDOWN: C2RustUnnamed_0 = -11;
pub const KEYCODE_PAGEUP: C2RustUnnamed_0 = -10;
pub const KEYCODE_DELETE: C2RustUnnamed_0 = -9;
pub const KEYCODE_INSERT: C2RustUnnamed_0 = -8;
pub const KEYCODE_END: C2RustUnnamed_0 = -7;
pub const KEYCODE_HOME: C2RustUnnamed_0 = -6;
pub const KEYCODE_LEFT: C2RustUnnamed_0 = -5;
pub const KEYCODE_RIGHT: C2RustUnnamed_0 = -4;
pub const KEYCODE_DOWN: C2RustUnnamed_0 = -3;
pub const KEYCODE_UP: C2RustUnnamed_0 = -2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct smaprec {
  pub mapped_rw: libc::c_ulong,
  pub mapped_ro: libc::c_ulong,
  pub shared_clean: libc::c_ulong,
  pub shared_dirty: libc::c_ulong,
  pub private_clean: libc::c_ulong,
  pub private_dirty: libc::c_ulong,
  pub stack: libc::c_ulong,
  pub smap_pss: libc::c_ulong,
  pub smap_swap: libc::c_ulong,
  pub smap_size: libc::c_ulong,
  pub smap_start: libc::c_ulonglong,
  pub smap_mode: [libc::c_char; 5],
  pub smap_name: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct procps_status_t {
  pub dir: *mut DIR,
  pub task_dir: *mut DIR,
  pub shift_pages_to_bytes: u8,
  pub shift_pages_to_kb: u8,
  pub argv_len: u16,
  pub argv0: *mut libc::c_char,
  pub exe: *mut libc::c_char,
  pub main_thread_pid: libc::c_uint,
  pub vsz: libc::c_ulong,
  pub rss: libc::c_ulong,
  pub stime: libc::c_ulong,
  pub utime: libc::c_ulong,
  pub start_time: libc::c_ulong,
  pub pid: libc::c_uint,
  pub ppid: libc::c_uint,
  pub pgid: libc::c_uint,
  pub sid: libc::c_uint,
  pub uid: libc::c_uint,
  pub gid: libc::c_uint,
  pub ruid: libc::c_uint,
  pub rgid: libc::c_uint,
  pub niceness: libc::c_int,
  pub tty_major: libc::c_uint,
  pub tty_minor: libc::c_uint,
  pub smaps: smaprec,
  pub state: [libc::c_char; 4],
  pub comm: [libc::c_char; 16],
  pub last_seen_on_cpu: libc::c_int,
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub const PSSCAN_TASKS: C2RustUnnamed_1 = 4194304;
pub const PSSCAN_RUIDGID: C2RustUnnamed_1 = 2097152;
pub const PSSCAN_NICE: C2RustUnnamed_1 = 1048576;
pub const PSSCAN_CPU: C2RustUnnamed_1 = 524288;
pub const PSSCAN_START_TIME: C2RustUnnamed_1 = 262144;
pub const PSSCAN_CONTEXT: C2RustUnnamed_1 = 0;
pub const PSSCAN_ARGVN: C2RustUnnamed_1 = 65536;
pub const PSSCAN_SMAPS: C2RustUnnamed_1 = 32768;
pub const PSSCAN_TTY: C2RustUnnamed_1 = 16384;
pub const PSSCAN_UTIME: C2RustUnnamed_1 = 8192;
pub const PSSCAN_STIME: C2RustUnnamed_1 = 4096;
pub const PSSCAN_RSS: C2RustUnnamed_1 = 2048;
pub const PSSCAN_VSZ: C2RustUnnamed_1 = 1024;
pub const PSSCAN_STATE: C2RustUnnamed_1 = 512;
pub const PSSCAN_EXE: C2RustUnnamed_1 = 256;
pub const PSSCAN_ARGV0: C2RustUnnamed_1 = 128;
pub const PSSCAN_COMM: C2RustUnnamed_1 = 32;
pub const PSSCAN_UIDGID: C2RustUnnamed_1 = 16;
pub const PSSCAN_SID: C2RustUnnamed_1 = 8;
pub const PSSCAN_PGID: C2RustUnnamed_1 = 4;
pub const PSSCAN_PPID: C2RustUnnamed_1 = 2;
pub const PSSCAN_PID: C2RustUnnamed_1 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub top: *mut top_status_t,
  pub ntop: libc::c_int,
  pub inverted: smallint,
  pub sort_field: smallint,
  pub smp_cpu_info: smallint,
  pub lines: libc::c_uint,
  pub initial_settings: termios,
  pub scroll_ofs: libc::c_int,
  pub sort_function: [cmp_funcp; 3],
  pub prev_hist: *mut save_hist,
  pub prev_hist_count: libc::c_uint,
  pub cur_jif: jiffy_counts_t,
  pub prev_jif: jiffy_counts_t,
  pub total_pcpu: libc::c_uint,
  pub cpu_jif: *mut jiffy_counts_t,
  pub cpu_prev_jif: *mut jiffy_counts_t,
  pub num_cpus: libc::c_uint,
  pub kbd_input: [libc::c_char; 16],
  pub line_buf: [libc::c_char; 448],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jiffy_counts_t {
  pub usr: libc::c_ulonglong,
  pub nic: libc::c_ulonglong,
  pub sys: libc::c_ulonglong,
  pub idle: libc::c_ulonglong,
  pub iowait: libc::c_ulonglong,
  pub irq: libc::c_ulonglong,
  pub softirq: libc::c_ulonglong,
  pub steal: libc::c_ulonglong,
  pub total: libc::c_ulonglong,
  pub busy: libc::c_ulonglong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct save_hist {
  pub ticks: libc::c_ulong,
  pub pid: pid_t,
}
pub type cmp_funcp =
  Option<unsafe extern "C" fn(_: *mut top_status_t, _: *mut top_status_t) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct top_status_t {
  pub vsz: libc::c_ulong,
  pub ticks: libc::c_ulong,
  pub pcpu: libc::c_uint,
  pub pid: libc::c_uint,
  pub ppid: libc::c_uint,
  pub uid: libc::c_uint,
  pub state: [libc::c_char; 4],
  pub comm: [libc::c_char; 16],
  pub last_seen_on_cpu: libc::c_int,
}
pub type C2RustUnnamed_2 = libc::c_uint;
pub const SORT_DEPTH: C2RustUnnamed_2 = 3;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const LINE_BUF_SIZE: C2RustUnnamed_3 = 448;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const OPT_EOF: C2RustUnnamed_4 = 32;
pub const OPT_m: C2RustUnnamed_4 = 16;
pub const OPT_H: C2RustUnnamed_4 = 8;
pub const OPT_b: C2RustUnnamed_4 = 4;
pub const OPT_n: C2RustUnnamed_4 = 2;
pub const OPT_d: C2RustUnnamed_4 = 1;
/* pseudo: "we saw EOF in stdin" */
/* !ENABLE_FEATURE_TOP_CPU_GLOBAL_PERCENTS */
pub type C2RustUnnamed_5 = libc::c_uint;
pub const MI_MAX: C2RustUnnamed_5 = 13;
pub const MI_SLAB: C2RustUnnamed_5 = 12;
pub const MI_MAPPED: C2RustUnnamed_5 = 11;
pub const MI_ANONPAGES: C2RustUnnamed_5 = 10;
pub const MI_WRITEBACK: C2RustUnnamed_5 = 9;
pub const MI_DIRTY: C2RustUnnamed_5 = 8;
pub const MI_SWAPFREE: C2RustUnnamed_5 = 7;
pub const MI_SWAPTOTAL: C2RustUnnamed_5 = 6;
pub const MI_CACHED: C2RustUnnamed_5 = 5;
pub const MI_BUFFERS: C2RustUnnamed_5 = 4;
pub const MI_SHMEM: C2RustUnnamed_5 = 3;
pub const MI_MEMSHARED: C2RustUnnamed_5 = 2;
pub const MI_MEMFREE: C2RustUnnamed_5 = 1;
pub const MI_MEMTOTAL: C2RustUnnamed_5 = 0;
pub const BITS_PER_INT: C2RustUnnamed_6 = 32;
pub type C2RustUnnamed_6 = libc::c_uint;
/* FEATURE_TOP_INTERACTIVE */
/*
 * TOPMEM support
 */
pub type mem_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct topmem_status_t {
  pub pid: libc::c_uint,
  pub comm: [libc::c_char; 16],
  pub vsz: mem_t,
  pub vszrw: mem_t,
  pub rss: mem_t,
  pub rss_sh: mem_t,
  pub dirty: mem_t,
  pub dirty_sh: mem_t,
  pub stack: mem_t,
}
pub type C2RustUnnamed_7 = libc::c_uint;
pub const NUM_SORT_FIELD: C2RustUnnamed_7 = 7;
/* TOPMEM */
/*
 * end TOPMEM support
 */
pub type C2RustUnnamed_8 = libc::c_uint;
pub const NO_RESCAN_MASK: C2RustUnnamed_8 = 4294967295;
pub const EXIT_MASK: C2RustUnnamed_8 = 0;
pub const TOPMEM_MASK: C2RustUnnamed_8 = 32801;
pub const TOP_MASK: C2RustUnnamed_8 = 538163;
#[inline(always)]
unsafe extern "C" fn not_const_pp(mut p: *const libc::c_void) -> *mut libc::c_void {
  return p as *mut libc::c_void;
}
unsafe extern "C" fn pid_sort(mut P: *mut top_status_t, mut Q: *mut top_status_t) -> libc::c_int {
  /* Buggy wrt pids with high bit set */
  /* (linux pids are in [1..2^15-1]) */
  return (*Q).pid.wrapping_sub((*P).pid) as libc::c_int;
}
unsafe extern "C" fn mem_sort(mut P: *mut top_status_t, mut Q: *mut top_status_t) -> libc::c_int {
  /* We want to avoid unsigned->signed and truncation errors */
  if (*Q).vsz < (*P).vsz {
    return -1i32;
  }
  return ((*Q).vsz != (*P).vsz) as libc::c_int;
  /* 0 if ==, 1 if > */
}
unsafe extern "C" fn pcpu_sort(mut P: *mut top_status_t, mut Q: *mut top_status_t) -> libc::c_int {
  /* Buggy wrt ticks with high bit set */
  /* Affects only processes for which ticks overflow */
  return (*Q).pcpu as libc::c_int - (*P).pcpu as libc::c_int;
}
unsafe extern "C" fn time_sort(mut P: *mut top_status_t, mut Q: *mut top_status_t) -> libc::c_int {
  /* We want to avoid unsigned->signed and truncation errors */
  if (*Q).ticks < (*P).ticks {
    return -1i32;
  }
  return ((*Q).ticks != (*P).ticks) as libc::c_int;
  /* 0 if ==, 1 if > */
}
unsafe extern "C" fn mult_lvl_cmp(
  mut a: *mut libc::c_void,
  mut b: *mut libc::c_void,
) -> libc::c_int {
  let mut i: libc::c_int = 0;
  let mut cmp_val: libc::c_int = 0;
  i = 0i32;
  while i < SORT_DEPTH as libc::c_int {
    cmp_val =
      Some(
        (*(*ptr_to_globals)
          .sort_function
          .as_mut_ptr()
          .offset(i as isize))
        .expect("non-null function pointer"),
      )
      .expect("non-null function pointer")(a as *mut top_status_t, b as *mut top_status_t);
    if cmp_val != 0i32 {
      break;
    }
    i += 1
  }
  return if (*ptr_to_globals).inverted as libc::c_int != 0 {
    -cmp_val
  } else {
    cmp_val
  };
}
#[inline(never)]
unsafe extern "C" fn read_cpu_jiffy(
  mut fp: *mut FILE,
  mut p_jif: *mut jiffy_counts_t,
) -> libc::c_int {
  static mut fmt: [libc::c_char; 46] = [
    99, 112, 37, 42, 115, 32, 37, 108, 108, 117, 32, 37, 108, 108, 117, 32, 37, 108, 108, 117, 32,
    37, 108, 108, 117, 32, 37, 108, 108, 117, 32, 37, 108, 108, 117, 32, 37, 108, 108, 117, 32, 37,
    108, 108, 117, 0,
  ];
  let mut ret: libc::c_int = 0;
  if fgets_unlocked(
    (*ptr_to_globals).line_buf.as_mut_ptr(),
    LINE_BUF_SIZE as libc::c_int,
    fp,
  )
  .is_null()
    || (*ptr_to_globals).line_buf[0] as libc::c_int != 'c' as i32
  {
    /* not "cpu" */
    return 0i32;
  }
  ret = sscanf(
    (*ptr_to_globals).line_buf.as_mut_ptr(),
    fmt.as_ptr(),
    &mut (*p_jif).usr as *mut libc::c_ulonglong,
    &mut (*p_jif).nic as *mut libc::c_ulonglong,
    &mut (*p_jif).sys as *mut libc::c_ulonglong,
    &mut (*p_jif).idle as *mut libc::c_ulonglong,
    &mut (*p_jif).iowait as *mut libc::c_ulonglong,
    &mut (*p_jif).irq as *mut libc::c_ulonglong,
    &mut (*p_jif).softirq as *mut libc::c_ulonglong,
    &mut (*p_jif).steal as *mut libc::c_ulonglong,
  );
  if ret >= 4i32 {
    (*p_jif).total = (*p_jif)
      .usr
      .wrapping_add((*p_jif).nic)
      .wrapping_add((*p_jif).sys)
      .wrapping_add((*p_jif).idle)
      .wrapping_add((*p_jif).iowait)
      .wrapping_add((*p_jif).irq)
      .wrapping_add((*p_jif).softirq)
      .wrapping_add((*p_jif).steal);
    /* procps 2.x does not count iowait as busy time */
    (*p_jif).busy = (*p_jif)
      .total
      .wrapping_sub((*p_jif).idle)
      .wrapping_sub((*p_jif).iowait)
  }
  return ret;
}
unsafe extern "C" fn get_jiffy_counts() {
  let mut fp: *mut FILE = xfopen_for_read(b"stat\x00" as *const u8 as *const libc::c_char);
  /* We need to parse cumulative counts even if SMP CPU display is on,
   * they are used to calculate per process CPU% */
  (*ptr_to_globals).prev_jif = (*ptr_to_globals).cur_jif; /* Non first time invocation */
  if read_cpu_jiffy(fp, &mut (*ptr_to_globals).cur_jif) < 4i32 {
    bb_error_msg_and_die(
      b"can\'t read \'%s\'\x00" as *const u8 as *const libc::c_char,
      b"/proc/stat\x00" as *const u8 as *const libc::c_char,
    );
  }
  if (*ptr_to_globals).smp_cpu_info == 0 {
    fclose(fp);
    return;
  }
  if (*ptr_to_globals).num_cpus == 0 {
    loop
    /* First time here. How many CPUs?
     * There will be at least 1 /proc/stat line with cpu%d
     */
    {
      (*ptr_to_globals).cpu_jif = xrealloc_vector_helper(
        (*ptr_to_globals).cpu_jif as *mut libc::c_void,
        ((::std::mem::size_of::<jiffy_counts_t>() as libc::c_ulong) << 8i32)
          .wrapping_add(1i32 as libc::c_ulong) as libc::c_uint,
        (*ptr_to_globals).num_cpus as libc::c_int,
      ) as *mut jiffy_counts_t;
      if read_cpu_jiffy(
        fp,
        &mut *(*ptr_to_globals)
          .cpu_jif
          .offset((*ptr_to_globals).num_cpus as isize),
      ) <= 4i32
      {
        break;
      }
      (*ptr_to_globals).num_cpus = (*ptr_to_globals).num_cpus.wrapping_add(1)
    }
    if (*ptr_to_globals).num_cpus == 0i32 as libc::c_uint {
      /* /proc/stat with only "cpu ..." line?! */
      (*ptr_to_globals).smp_cpu_info = 0i32 as smallint
    }
    (*ptr_to_globals).cpu_prev_jif = xzalloc(
      (::std::mem::size_of::<jiffy_counts_t>() as libc::c_ulong)
        .wrapping_mul((*ptr_to_globals).num_cpus as libc::c_ulong),
    ) as *mut jiffy_counts_t;
    /* Otherwise the first per cpu display shows all 100% idles */
    usleep(50000i32 as useconds_t);
  } else {
    let mut tmp: *mut jiffy_counts_t = 0 as *mut jiffy_counts_t;
    let mut i: libc::c_int = 0;
    /* First switch the sample pointers: no need to copy */
    tmp = (*ptr_to_globals).cpu_prev_jif;
    (*ptr_to_globals).cpu_prev_jif = (*ptr_to_globals).cpu_jif;
    (*ptr_to_globals).cpu_jif = tmp;
    /* Get the new samples */
    i = 0i32;
    while (i as libc::c_uint) < (*ptr_to_globals).num_cpus {
      read_cpu_jiffy(fp, &mut *(*ptr_to_globals).cpu_jif.offset(i as isize));
      i += 1
    }
  }
  fclose(fp);
}
unsafe extern "C" fn do_stats() {
  let mut cur: *mut top_status_t = 0 as *mut top_status_t;
  let mut pid: pid_t = 0;
  let mut n: libc::c_int = 0;
  let mut i: libc::c_uint = 0;
  let mut last_i: libc::c_uint = 0;
  let mut new_hist: *mut save_hist = 0 as *mut save_hist;
  get_jiffy_counts();
  (*ptr_to_globals).total_pcpu = 0i32 as libc::c_uint;
  /* total_vsz = 0; */
  new_hist = xmalloc(
    (::std::mem::size_of::<save_hist>() as libc::c_ulong)
      .wrapping_mul((*ptr_to_globals).ntop as libc::c_ulong),
  ) as *mut save_hist;
  /*
   * Make a pass through the data to get stats.
   */
  /* hist_iterations = 0; */
  i = 0i32 as libc::c_uint;
  n = 0i32;
  while n < (*ptr_to_globals).ntop {
    cur = (*ptr_to_globals).top.offset(n as isize);
    /* total_vsz += cur->vsz; */
    pid = (*cur).pid as pid_t;
    (*new_hist.offset(n as isize)).ticks = (*cur).ticks;
    (*new_hist.offset(n as isize)).pid = pid;
    (*cur).pcpu = 0i32 as libc::c_uint;
    last_i = i;
    if (*ptr_to_globals).prev_hist_count != 0 {
      loop {
        if (*(*ptr_to_globals).prev_hist.offset(i as isize)).pid == pid {
          (*cur).pcpu = (*cur)
            .ticks
            .wrapping_sub((*(*ptr_to_globals).prev_hist.offset(i as isize)).ticks)
            as libc::c_uint;
          (*ptr_to_globals).total_pcpu = (*ptr_to_globals).total_pcpu.wrapping_add((*cur).pcpu);
          break;
        } else {
          i = i
            .wrapping_add(1i32 as libc::c_uint)
            .wrapping_rem((*ptr_to_globals).prev_hist_count);
          if !(i != last_i) {
            break;
          }
        }
        /*
         * Calculate time in cur process.  Time is sum of user time
         * and system time
         */
        /* find matching entry from previous pass */
        /* do not start at index 0, continue at last used one
         * (brought hist_iterations from ~14000 down to 172) */
        /* hist_iterations++; */
      }
    }
    n += 1
  }
  /*
   * Save cur frame's information.
   */
  free((*ptr_to_globals).prev_hist as *mut libc::c_void);
  (*ptr_to_globals).prev_hist = new_hist;
  (*ptr_to_globals).prev_hist_count = (*ptr_to_globals).ntop as libc::c_uint;
}
/* FEATURE_TOP_CPU_USAGE_PERCENTAGE */
/* formats 7 char string (8 with terminating NUL) */
unsafe extern "C" fn fmt_100percent_8(
  mut pbuf: *mut libc::c_char,
  mut value: libc::c_uint,
  mut total: libc::c_uint,
) -> *mut libc::c_char {
  let mut t: libc::c_uint = 0;
  if value >= total {
    /* 100% ? */
    strcpy(pbuf, b"  100% \x00" as *const u8 as *const libc::c_char);
    return pbuf;
  }
  /* else generate " [N/space]N.N% " string */
  value = (1000i32 as libc::c_uint)
    .wrapping_mul(value)
    .wrapping_div(total);
  t = value.wrapping_div(100i32 as libc::c_uint);
  value = value.wrapping_rem(100i32 as libc::c_uint);
  *pbuf.offset(0) = ' ' as i32 as libc::c_char;
  *pbuf.offset(1) = if t != 0 {
    t.wrapping_add('0' as i32 as libc::c_uint)
  } else {
    ' ' as i32 as libc::c_uint
  } as libc::c_char;
  *pbuf.offset(2) = ('0' as i32 as libc::c_uint)
    .wrapping_add(value.wrapping_div(10i32 as libc::c_uint)) as libc::c_char;
  *pbuf.offset(3) = '.' as i32 as libc::c_char;
  *pbuf.offset(4) = ('0' as i32 as libc::c_uint)
    .wrapping_add(value.wrapping_rem(10i32 as libc::c_uint)) as libc::c_char;
  *pbuf.offset(5) = '%' as i32 as libc::c_char;
  *pbuf.offset(6) = ' ' as i32 as libc::c_char;
  *pbuf.offset(7) = '\u{0}' as i32 as libc::c_char;
  return pbuf;
}
unsafe extern "C" fn display_cpus(
  mut scr_width: libc::c_int,
  mut scrbuf: *mut libc::c_char,
  mut lines_rem_p: *mut libc::c_int,
) {
  /*
   * xxx% = (cur_jif.xxx - prev_jif.xxx) / (cur_jif.total - prev_jif.total) * 100%
   */
  let mut total_diff: libc::c_uint = 0;
  let mut p_jif: *mut jiffy_counts_t = 0 as *mut jiffy_counts_t;
  let mut p_prev_jif: *mut jiffy_counts_t = 0 as *mut jiffy_counts_t;
  let mut i: libc::c_int = 0;
  let mut n_cpu_lines: libc::c_int = 0;
  /* using (unsigned) casts to make operations cheaper */
  /* Loop thru CPU(s) */
  n_cpu_lines = if (*ptr_to_globals).smp_cpu_info as libc::c_int != 0 {
    (*ptr_to_globals).num_cpus
  } else {
    1i32 as libc::c_uint
  } as libc::c_int;
  if n_cpu_lines > *lines_rem_p {
    n_cpu_lines = *lines_rem_p
  }
  i = 0i32;
  while i < n_cpu_lines {
    p_jif = &mut *(*ptr_to_globals).cpu_jif.offset(i as isize) as *mut jiffy_counts_t;
    p_prev_jif = &mut *(*ptr_to_globals).cpu_prev_jif.offset(i as isize) as *mut jiffy_counts_t;
    total_diff = (*p_jif).total.wrapping_sub((*p_prev_jif).total) as libc::c_uint;
    if total_diff == 0i32 as libc::c_uint {
      total_diff = 1i32 as libc::c_uint
    }
    /* Need a block: CALC_STAT are declarations */
    let mut usr: [libc::c_char; 8] = [0; 8];
    let mut sys: [libc::c_char; 8] = [0; 8];
    let mut nic: [libc::c_char; 8] = [0; 8];
    let mut idle: [libc::c_char; 8] = [0; 8];
    let mut iowait: [libc::c_char; 8] = [0; 8];
    let mut irq: [libc::c_char; 8] = [0; 8];
    let mut softirq: [libc::c_char; 8] = [0; 8];
    /*CALC_STAT(steal);*/
    snprintf(
      scrbuf,
      scr_width as libc::c_ulong,
      b"CPU%s:%susr%ssys%snic%sidle%sio%sirq%ssirq\x00" as *const u8 as *const libc::c_char,
      if (*ptr_to_globals).smp_cpu_info as libc::c_int != 0 {
        utoa(i as libc::c_uint)
      } else {
        b"\x00" as *const u8 as *const libc::c_char
      },
      fmt_100percent_8(
        usr.as_mut_ptr(),
        (*p_jif).usr.wrapping_sub((*p_prev_jif).usr) as libc::c_uint,
        total_diff,
      ),
      fmt_100percent_8(
        sys.as_mut_ptr(),
        (*p_jif).sys.wrapping_sub((*p_prev_jif).sys) as libc::c_uint,
        total_diff,
      ),
      fmt_100percent_8(
        nic.as_mut_ptr(),
        (*p_jif).nic.wrapping_sub((*p_prev_jif).nic) as libc::c_uint,
        total_diff,
      ),
      fmt_100percent_8(
        idle.as_mut_ptr(),
        (*p_jif).idle.wrapping_sub((*p_prev_jif).idle) as libc::c_uint,
        total_diff,
      ),
      fmt_100percent_8(
        iowait.as_mut_ptr(),
        (*p_jif).iowait.wrapping_sub((*p_prev_jif).iowait) as libc::c_uint,
        total_diff,
      ),
      fmt_100percent_8(
        irq.as_mut_ptr(),
        (*p_jif).irq.wrapping_sub((*p_prev_jif).irq) as libc::c_uint,
        total_diff,
      ),
      fmt_100percent_8(
        softirq.as_mut_ptr(),
        (*p_jif).softirq.wrapping_sub((*p_prev_jif).softirq) as libc::c_uint,
        total_diff,
      ),
    ); /* actual lines we expect are ~30 chars or less */
    puts(scrbuf); /* [80] was a bit too low on 8Gb ram box */
    i += 1
  }
  *lines_rem_p -= i;
}
unsafe extern "C" fn parse_meminfo(mut meminfo: *mut libc::c_ulong) {
  static mut fields: [libc::c_char; 106] = [
    77, 101, 109, 84, 111, 116, 97, 108, 0, 77, 101, 109, 70, 114, 101, 101, 0, 77, 101, 109, 83,
    104, 97, 114, 101, 100, 0, 83, 104, 109, 101, 109, 0, 66, 117, 102, 102, 101, 114, 115, 0, 67,
    97, 99, 104, 101, 100, 0, 83, 119, 97, 112, 84, 111, 116, 97, 108, 0, 83, 119, 97, 112, 70,
    114, 101, 101, 0, 68, 105, 114, 116, 121, 0, 87, 114, 105, 116, 101, 98, 97, 99, 107, 0, 65,
    110, 111, 110, 80, 97, 103, 101, 115, 0, 77, 97, 112, 112, 101, 100, 0, 83, 108, 97, 98, 0, 0,
  ];
  let mut buf: [libc::c_char; 60] = [0; 60];
  let mut f: *mut FILE = 0 as *mut FILE;
  let mut i: libc::c_int = 0;
  memset(
    meminfo as *mut libc::c_void,
    0i32,
    (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
      .wrapping_mul(MI_MAX as libc::c_int as libc::c_ulong),
  );
  f = xfopen_for_read(b"meminfo\x00" as *const u8 as *const libc::c_char);
  while !fgets_unlocked(
    buf.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_char; 60]>() as libc::c_ulong as libc::c_int,
    f,
  )
  .is_null()
  {
    let mut c: *mut libc::c_char = strchr(buf.as_mut_ptr(), ':' as i32);
    if c.is_null() {
      continue;
    }
    *c = '\u{0}' as i32 as libc::c_char;
    i = index_in_strings(fields.as_ptr(), buf.as_mut_ptr());
    if i >= 0i32 {
      *meminfo.offset(i as isize) = strtoul(c.offset(1), 0 as *mut *mut libc::c_char, 10i32)
    }
  }
  fclose(f);
}
unsafe extern "C" fn display_header(
  mut scr_width: libc::c_int,
  mut lines_rem_p: *mut libc::c_int,
) -> libc::c_ulong {
  let mut scrbuf: [libc::c_char; 100] = [0; 100];
  let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut meminfo: [libc::c_ulong; 13] = [0; 13];
  parse_meminfo(meminfo.as_mut_ptr());
  /* Output memory info */
  if scr_width > ::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong as libc::c_int {
    scr_width = ::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong as libc::c_int
  }
  snprintf(
    scrbuf.as_mut_ptr(),
    scr_width as libc::c_ulong,
    b"Mem: %luK used, %luK free, %luK shrd, %luK buff, %luK cached\x00" as *const u8
      as *const libc::c_char,
    meminfo[MI_MEMTOTAL as libc::c_int as usize]
      .wrapping_sub(meminfo[MI_MEMFREE as libc::c_int as usize]),
    meminfo[MI_MEMFREE as libc::c_int as usize],
    meminfo[MI_MEMSHARED as libc::c_int as usize]
      .wrapping_add(meminfo[MI_SHMEM as libc::c_int as usize]),
    meminfo[MI_BUFFERS as libc::c_int as usize],
    meminfo[MI_CACHED as libc::c_int as usize],
  );
  /* Go to top & clear to the end of screen */
  printf(
    if option_mask32 & OPT_b as libc::c_int as libc::c_uint != 0 {
      b"%s\n\x00" as *const u8 as *const libc::c_char
    } else {
      b"\x1b[H\x1b[J%s\n\x00" as *const u8 as *const libc::c_char
    },
    scrbuf.as_mut_ptr(),
  );
  *lines_rem_p -= 1;
  /* Display CPU time split as percentage of total time.
   * This displays either a cumulative line or one line per CPU.
   */
  display_cpus(scr_width, scrbuf.as_mut_ptr(), lines_rem_p);
  /* Read load average as a string */
  buf = stpcpy(
    scrbuf.as_mut_ptr(),
    b"Load average: \x00" as *const u8 as *const libc::c_char,
  ); /* or use total_vsz? */
  open_read_close(
    b"loadavg\x00" as *const u8 as *const libc::c_char,
    buf as *mut libc::c_void,
    (::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong)
      .wrapping_sub(::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong),
  );
  scrbuf[(scr_width - 1i32) as usize] = '\u{0}' as i32 as libc::c_char;
  *strchrnul(buf, '\n' as i32).offset(0) = '\u{0}' as i32 as libc::c_char;
  puts(scrbuf.as_mut_ptr());
  *lines_rem_p -= 1;
  return meminfo[MI_MEMTOTAL as libc::c_int as usize];
}
#[inline(never)]
unsafe extern "C" fn display_process_list(mut lines_rem: libc::c_int, mut scr_width: libc::c_int) {
  let mut s: *mut top_status_t = 0 as *mut top_status_t;
  let mut total_memory: libc::c_ulong = display_header(scr_width, &mut lines_rem);
  /* xxx_shift and xxx_scale variables allow us to replace
   * expensive divides with multiply and shift */
  let mut pmem_shift: libc::c_uint = 0;
  let mut pmem_scale: libc::c_uint = 0;
  let mut pmem_half: libc::c_uint = 0;
  let mut tmp_unsigned: libc::c_uint = 0;
  let mut pcpu_shift: libc::c_uint = 0;
  let mut pcpu_scale: libc::c_uint = 0;
  let mut pcpu_half: libc::c_uint = 0;
  let mut busy_jifs: libc::c_uint = 0;
  /* what info of the processes is shown */
  printf(
    if option_mask32 & OPT_b as libc::c_int as libc::c_uint != 0 {
      b"%.*s\x00" as *const u8 as *const libc::c_char
    } else {
      b"\x1b[7m%.*s\x1b[m\x00" as *const u8 as *const libc::c_char
    },
    scr_width,
    b"  PID  PPID USER     STAT   VSZ %VSZ CPU %CPU COMMAND\x00" as *const u8
      as *const libc::c_char,
  );
  lines_rem -= 1;
  /*
   * %VSZ = s->vsz/MemTotal
   */
  pmem_shift = (BITS_PER_INT as libc::c_int - 11i32) as libc::c_uint;
  pmem_scale = ((1000i32 as libc::c_uint).wrapping_mul(1u32 << BITS_PER_INT as libc::c_int - 11i32)
    as libc::c_ulong)
    .wrapping_div(total_memory) as libc::c_uint;
  /* s->vsz is in kb. we want (s->vsz * pmem_scale) to never overflow */
  while pmem_scale >= 512i32 as libc::c_uint {
    pmem_scale = pmem_scale.wrapping_div(4i32 as libc::c_uint);
    pmem_shift = pmem_shift.wrapping_sub(2i32 as libc::c_uint)
  }
  pmem_half =
    (1u32 << pmem_shift).wrapping_div((if 1i32 != 0 { 20i32 } else { 2i32 }) as libc::c_uint);
  busy_jifs = (*ptr_to_globals)
    .cur_jif
    .busy
    .wrapping_sub((*ptr_to_globals).prev_jif.busy) as libc::c_uint;
  /* This happens if there were lots of short-lived processes
   * between two top updates (e.g. compilation) */
  if (*ptr_to_globals).total_pcpu < busy_jifs {
    (*ptr_to_globals).total_pcpu = busy_jifs
  }
  /*
   * CPU% = s->pcpu/sum(s->pcpu) * busy_cpu_ticks/total_cpu_ticks
   * (pcpu is delta of sys+user time between samples)
   */
  /* (cur_jif.xxx - prev_jif.xxx) and s->pcpu are
   * in 0..~64000 range (HZ*update_interval).
   * we assume that unsigned is at least 32-bit.
   */
  pcpu_shift = 6i32 as libc::c_uint;
  pcpu_scale = (1000i32 * 64i32 * busy_jifs as u16 as libc::c_int) as libc::c_uint;
  if pcpu_scale == 0i32 as libc::c_uint {
    pcpu_scale = 1i32 as libc::c_uint
  }
  while pcpu_scale < 1u32 << BITS_PER_INT as libc::c_int - 2i32 {
    pcpu_scale = pcpu_scale.wrapping_mul(4i32 as libc::c_uint);
    pcpu_shift = pcpu_shift.wrapping_add(2i32 as libc::c_uint)
  }
  tmp_unsigned = ((*ptr_to_globals)
    .cur_jif
    .total
    .wrapping_sub((*ptr_to_globals).prev_jif.total) as u16 as libc::c_uint)
    .wrapping_mul((*ptr_to_globals).total_pcpu);
  if tmp_unsigned != 0i32 as libc::c_uint {
    pcpu_scale = pcpu_scale.wrapping_div(tmp_unsigned)
  }
  /* we want (s->pcpu * pcpu_scale) to never overflow */
  while pcpu_scale >= 1024i32 as libc::c_uint {
    pcpu_scale = pcpu_scale.wrapping_div(4i32 as libc::c_uint);
    pcpu_shift = pcpu_shift.wrapping_sub(2i32 as libc::c_uint)
  }
  pcpu_half =
    (1u32 << pcpu_shift).wrapping_div((if 1i32 != 0 { 20i32 } else { 2i32 }) as libc::c_uint);
  /* printf(" pmem_scale=%u pcpu_scale=%u ", pmem_scale, pcpu_scale); */
  /* Ok, all preliminary data is ready, go through the list */
  scr_width += 2i32; /* account for leading '\n' and trailing NUL */
  if lines_rem > (*ptr_to_globals).ntop - (*ptr_to_globals).scroll_ofs {
    lines_rem = (*ptr_to_globals).ntop - (*ptr_to_globals).scroll_ofs
  }
  s = (*ptr_to_globals)
    .top
    .offset((*ptr_to_globals).scroll_ofs as isize);
  loop {
    lines_rem -= 1;
    if !(lines_rem >= 0i32) {
      break;
    }
    let mut vsz_str_buf: [libc::c_char; 8] = [0; 8];
    let mut col: libc::c_uint = 0;
    let mut pmem: div_t = div(
      ((*s)
        .vsz
        .wrapping_mul(pmem_scale as libc::c_ulong)
        .wrapping_add(pmem_half as libc::c_ulong)
        >> pmem_shift) as libc::c_int,
      10i32,
    );
    let mut pcpu: div_t = div(
      ((*s).pcpu.wrapping_mul(pcpu_scale).wrapping_add(pcpu_half) >> pcpu_shift) as libc::c_int,
      10i32,
    );
    smart_ulltoa5(
      (*s).vsz as libc::c_ulonglong,
      vsz_str_buf.as_mut_ptr(),
      b" mgtpezy\x00" as *const u8 as *const libc::c_char,
    );
    /* PID PPID USER STAT VSZ %VSZ [%CPU] COMMAND */
    col = snprintf(
      (*ptr_to_globals).line_buf.as_mut_ptr(),
      scr_width as libc::c_ulong,
      b"\n%5u%6u %-8.8s %s  %.5s%3u.%c %3d%3u.%c \x00" as *const u8 as *const libc::c_char,
      (*s).pid,
      (*s).ppid,
      get_cached_username((*s).uid),
      (*s).state.as_mut_ptr(),
      vsz_str_buf.as_mut_ptr(),
      pmem.quot,
      '0' as i32 + pmem.rem,
      (*s).last_seen_on_cpu,
      pcpu.quot,
      '0' as i32 + pcpu.rem,
    ) as libc::c_uint;
    if (scr_width as libc::c_uint).wrapping_sub(col) as libc::c_int > 1i32 {
      read_cmdline(
        (*ptr_to_globals).line_buf.as_mut_ptr().offset(col as isize),
        (scr_width as libc::c_uint).wrapping_sub(col) as libc::c_int,
        (*s).pid,
        (*s).comm.as_mut_ptr(),
      );
    }
    fputs_unlocked((*ptr_to_globals).line_buf.as_mut_ptr(), stdout);
    /* printf(" %d/%d %lld/%lld", s->pcpu, total_pcpu,
    cur_jif.busy - prev_jif.busy, cur_jif.total - prev_jif.total); */
    s = s.offset(1)
  }
  /* printf(" %d", hist_iterations); */
  bb_putchar(
    if option_mask32 & OPT_b as libc::c_int as libc::c_uint != 0 {
      '\n' as i32
    } else {
      '\r' as i32
    },
  );
  fflush_all();
}
unsafe extern "C" fn clearmems() {
  clear_username_cache();
  free((*ptr_to_globals).top as *mut libc::c_void);
  (*ptr_to_globals).top = 0 as *mut top_status_t;
}
unsafe extern "C" fn reset_term() {
  if option_mask32 & OPT_b as libc::c_int as libc::c_uint == 0 {
    tcsetattr_stdin_TCSANOW(&mut (*ptr_to_globals).initial_settings);
  };
}
unsafe extern "C" fn sig_catcher(mut sig: libc::c_int) {
  reset_term();
  kill_myself_with_sig(sig);
}
unsafe extern "C" fn topmem_sort(
  mut a: *mut libc::c_char,
  mut b: *mut libc::c_char,
) -> libc::c_int {
  let mut n: libc::c_int = 0;
  let mut l: mem_t = 0;
  let mut r: mem_t = 0;
  n = 24u64.wrapping_add(
    ((*ptr_to_globals).sort_field as libc::c_ulong)
      .wrapping_mul(::std::mem::size_of::<mem_t>() as libc::c_ulong),
  ) as libc::c_int;
  l = *(a.offset(n as isize) as *mut mem_t);
  r = *(b.offset(n as isize) as *mut mem_t);
  if l == r {
    l = (*(a as *mut topmem_status_t)).dirty;
    r = (*(b as *mut topmem_status_t)).dirty
  }
  /* We want to avoid unsigned->signed and truncation errors */
  /* l>r: -1, l=r: 0, l<r: 1 */
  n = if l > r {
    -1i32
  } else {
    (l != r) as libc::c_int
  };
  return if (*ptr_to_globals).inverted as libc::c_int != 0 {
    -n
  } else {
    n
  };
}
/* display header info (meminfo / loadavg) */
unsafe extern "C" fn display_topmem_header(
  mut scr_width: libc::c_int,
  mut lines_rem_p: *mut libc::c_int,
) {
  let mut meminfo: [libc::c_ulong; 13] = [0; 13];
  parse_meminfo(meminfo.as_mut_ptr());
  snprintf(
    (*ptr_to_globals).line_buf.as_mut_ptr(),
    LINE_BUF_SIZE as libc::c_int as libc::c_ulong,
    b"Mem total:%lu anon:%lu map:%lu free:%lu\x00" as *const u8 as *const libc::c_char,
    meminfo[MI_MEMTOTAL as libc::c_int as usize],
    meminfo[MI_ANONPAGES as libc::c_int as usize],
    meminfo[MI_MAPPED as libc::c_int as usize],
    meminfo[MI_MEMFREE as libc::c_int as usize],
  );
  printf(
    if option_mask32 & OPT_b as libc::c_int as libc::c_uint != 0 {
      b"%.*s\n\x00" as *const u8 as *const libc::c_char
    } else {
      b"\x1b[H\x1b[J%.*s\n\x00" as *const u8 as *const libc::c_char
    },
    scr_width,
    (*ptr_to_globals).line_buf.as_mut_ptr(),
  );
  snprintf(
    (*ptr_to_globals).line_buf.as_mut_ptr(),
    LINE_BUF_SIZE as libc::c_int as libc::c_ulong,
    b" slab:%lu buf:%lu cache:%lu dirty:%lu write:%lu\x00" as *const u8 as *const libc::c_char,
    meminfo[MI_SLAB as libc::c_int as usize],
    meminfo[MI_BUFFERS as libc::c_int as usize],
    meminfo[MI_CACHED as libc::c_int as usize],
    meminfo[MI_DIRTY as libc::c_int as usize],
    meminfo[MI_WRITEBACK as libc::c_int as usize],
  );
  printf(
    b"%.*s\n\x00" as *const u8 as *const libc::c_char,
    scr_width,
    (*ptr_to_globals).line_buf.as_mut_ptr(),
  );
  snprintf(
    (*ptr_to_globals).line_buf.as_mut_ptr(),
    LINE_BUF_SIZE as libc::c_int as libc::c_ulong,
    b"Swap total:%lu free:%lu\x00" as *const u8 as *const libc::c_char,
    meminfo[MI_SWAPTOTAL as libc::c_int as usize],
    meminfo[MI_SWAPFREE as libc::c_int as usize],
  );
  printf(
    b"%.*s\n\x00" as *const u8 as *const libc::c_char,
    scr_width,
    (*ptr_to_globals).line_buf.as_mut_ptr(),
  );
  *lines_rem_p -= 3i32;
}
unsafe extern "C" fn ulltoa6_and_space(mut ul: libc::c_ulonglong, mut buf: *mut libc::c_char) {
  /* see http://en.wikipedia.org/wiki/Tera */
  *smart_ulltoa5(ul, buf, b" mgtpezy\x00" as *const u8 as *const libc::c_char).offset(0) =
    ' ' as i32 as libc::c_char;
}
#[inline(never)]
unsafe extern "C" fn display_topmem_process_list(
  mut lines_rem: libc::c_int,
  mut scr_width: libc::c_int,
) {
  let mut s: *const topmem_status_t =
    ((*ptr_to_globals).top as *mut topmem_status_t).offset((*ptr_to_globals).scroll_ofs as isize);
  let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut ch: libc::c_char = 0;
  display_topmem_header(scr_width, &mut lines_rem);
  strcpy(
    (*ptr_to_globals).line_buf.as_mut_ptr(),
    b"  PID   VSZ VSZRW   RSS (SHR) DIRTY (SHR) STACK COMMAND\x00" as *const u8
      as *const libc::c_char,
  );
  /* Mark the ^FIELD^ we sort by */
  cp = &mut *(*ptr_to_globals)
    .line_buf
    .as_mut_ptr()
    .offset((5i32 + (*ptr_to_globals).sort_field as libc::c_int * 6i32) as isize)
    as *mut libc::c_char;
  ch = (*::std::mem::transmute::<&[u8; 3], &[libc::c_char; 3]>(b"^_\x00"))
    [(*ptr_to_globals).inverted as usize];
  *cp.offset(6) = ch;
  loop {
    let fresh0 = cp;
    cp = cp.offset(1);
    *fresh0 = ch;
    if !(*cp as libc::c_int == ' ' as i32) {
      break;
    }
  }
  printf(
    if option_mask32 & OPT_b as libc::c_int as libc::c_uint != 0 {
      b"%.*s\x00" as *const u8 as *const libc::c_char
    } else {
      b"\x1b[7m%.*s\x1b[m\x00" as *const u8 as *const libc::c_char
    },
    scr_width,
    (*ptr_to_globals).line_buf.as_mut_ptr(),
  );
  lines_rem -= 1;
  if lines_rem > (*ptr_to_globals).ntop - (*ptr_to_globals).scroll_ofs {
    lines_rem = (*ptr_to_globals).ntop - (*ptr_to_globals).scroll_ofs
  }
  loop {
    lines_rem -= 1;
    if !(lines_rem >= 0i32) {
      break;
    }
    /* PID VSZ VSZRW RSS (SHR) DIRTY (SHR) COMMAND */
    ulltoa6_and_space(
      (*s).pid as libc::c_ulonglong,
      &mut *(*ptr_to_globals)
        .line_buf
        .as_mut_ptr()
        .offset((0i32 * 6i32) as isize),
    );
    ulltoa6_and_space(
      (*s).vsz as libc::c_ulonglong,
      &mut *(*ptr_to_globals)
        .line_buf
        .as_mut_ptr()
        .offset((1i32 * 6i32) as isize),
    );
    ulltoa6_and_space(
      (*s).vszrw as libc::c_ulonglong,
      &mut *(*ptr_to_globals)
        .line_buf
        .as_mut_ptr()
        .offset((2i32 * 6i32) as isize),
    );
    ulltoa6_and_space(
      (*s).rss as libc::c_ulonglong,
      &mut *(*ptr_to_globals)
        .line_buf
        .as_mut_ptr()
        .offset((3i32 * 6i32) as isize),
    );
    ulltoa6_and_space(
      (*s).rss_sh as libc::c_ulonglong,
      &mut *(*ptr_to_globals)
        .line_buf
        .as_mut_ptr()
        .offset((4i32 * 6i32) as isize),
    );
    ulltoa6_and_space(
      (*s).dirty as libc::c_ulonglong,
      &mut *(*ptr_to_globals)
        .line_buf
        .as_mut_ptr()
        .offset((5i32 * 6i32) as isize),
    );
    ulltoa6_and_space(
      (*s).dirty_sh as libc::c_ulonglong,
      &mut *(*ptr_to_globals)
        .line_buf
        .as_mut_ptr()
        .offset((6i32 * 6i32) as isize),
    );
    ulltoa6_and_space(
      (*s).stack as libc::c_ulonglong,
      &mut *(*ptr_to_globals)
        .line_buf
        .as_mut_ptr()
        .offset((7i32 * 6i32) as isize),
    );
    (*ptr_to_globals).line_buf[(8i32 * 6i32) as usize] = '\u{0}' as i32 as libc::c_char;
    if scr_width > ::std::mem::size_of::<[libc::c_char; 48]>() as libc::c_ulong as libc::c_int {
      read_cmdline(
        &mut *(*ptr_to_globals)
          .line_buf
          .as_mut_ptr()
          .offset((8i32 * 6i32) as isize),
        (scr_width as libc::c_ulong)
          .wrapping_sub(::std::mem::size_of::<[libc::c_char; 48]>() as libc::c_ulong)
          as libc::c_int,
        (*s).pid,
        (*s).comm.as_ptr(),
      );
    }
    printf(
      b"\n%.*s\x00" as *const u8 as *const libc::c_char,
      scr_width,
      (*ptr_to_globals).line_buf.as_mut_ptr(),
    );
    s = s.offset(1)
  }
  bb_putchar(
    if option_mask32 & OPT_b as libc::c_int as libc::c_uint != 0 {
      '\n' as i32
    } else {
      '\r' as i32
    },
  );
  fflush_all();
}
unsafe extern "C" fn handle_input(
  mut scan_mask: libc::c_uint,
  mut interval: duration_t,
) -> libc::c_uint {
  if option_mask32 & OPT_EOF as libc::c_int as libc::c_uint != 0 {
    /* EOF on stdin ("top </dev/null") */
    sleep_for_duration(interval);
    return scan_mask;
  }
  loop {
    let mut c: i32 = 0;
    c = read_key(
      0i32,
      (*ptr_to_globals).kbd_input.as_mut_ptr(),
      (interval * 1000i32 as libc::c_double) as libc::c_int,
    ) as i32;
    if c == -1i32 && *bb_errno != 11i32 {
      /* unknown key -> force refresh */
      /* error/EOF */
      option_mask32 |= OPT_EOF as libc::c_int as libc::c_uint; /* lowercase */
      break;
    } else {
      interval = 0i32 as duration_t;
      if c == (*ptr_to_globals).initial_settings.c_cc[0] as libc::c_int {
        return EXIT_MASK as libc::c_int as libc::c_uint;
      }
      if c == (*ptr_to_globals).initial_settings.c_cc[4] as libc::c_int {
        return EXIT_MASK as libc::c_int as libc::c_uint;
      }
      if c == KEYCODE_UP as libc::c_int {
        (*ptr_to_globals).scroll_ofs -= 1
      } else if c == KEYCODE_DOWN as libc::c_int {
        (*ptr_to_globals).scroll_ofs += 1
      } else if c == KEYCODE_HOME as libc::c_int {
        (*ptr_to_globals).scroll_ofs = 0i32
      } else if c == KEYCODE_END as libc::c_int {
        (*ptr_to_globals).scroll_ofs = ((*ptr_to_globals).ntop as libc::c_uint)
          .wrapping_sub((*ptr_to_globals).lines.wrapping_div(2i32 as libc::c_uint))
          as libc::c_int
      } else if c == KEYCODE_PAGEUP as libc::c_int {
        (*ptr_to_globals).scroll_ofs = ((*ptr_to_globals).scroll_ofs as libc::c_uint)
          .wrapping_sub((*ptr_to_globals).lines.wrapping_div(2i32 as libc::c_uint))
          as libc::c_int as libc::c_int
      } else if c == KEYCODE_PAGEDOWN as libc::c_int {
        (*ptr_to_globals).scroll_ofs = ((*ptr_to_globals).scroll_ofs as libc::c_uint)
          .wrapping_add((*ptr_to_globals).lines.wrapping_div(2i32 as libc::c_uint))
          as libc::c_int as libc::c_int
      } else {
        c |= 0x20i32;
        if c == 'q' as i32 {
          return EXIT_MASK as libc::c_int as libc::c_uint;
        }
        if c == 'n' as i32 {
          scan_mask = TOP_MASK as libc::c_int as libc::c_uint;
          (*ptr_to_globals).sort_function[0] = Some(
            pid_sort
              as unsafe extern "C" fn(_: *mut top_status_t, _: *mut top_status_t) -> libc::c_int,
          );
          continue;
        } else if c == 'm' as i32 {
          scan_mask = TOP_MASK as libc::c_int as libc::c_uint;
          (*ptr_to_globals).sort_function[0] = Some(
            mem_sort
              as unsafe extern "C" fn(_: *mut top_status_t, _: *mut top_status_t) -> libc::c_int,
          );
          (*ptr_to_globals).sort_function[1] = Some(
            pcpu_sort
              as unsafe extern "C" fn(_: *mut top_status_t, _: *mut top_status_t) -> libc::c_int,
          );
          (*ptr_to_globals).sort_function[2] = Some(
            time_sort
              as unsafe extern "C" fn(_: *mut top_status_t, _: *mut top_status_t) -> libc::c_int,
          );
          continue;
        } else if c == 'h' as i32 && scan_mask != TOPMEM_MASK as libc::c_int as libc::c_uint {
          scan_mask ^= PSSCAN_TASKS as libc::c_int as libc::c_uint;
          free((*ptr_to_globals).prev_hist as *mut libc::c_void);
          (*ptr_to_globals).prev_hist = 0 as *mut save_hist;
          (*ptr_to_globals).prev_hist_count = 0i32 as libc::c_uint;
          continue;
        } else if c == 'p' as i32 {
          scan_mask = TOP_MASK as libc::c_int as libc::c_uint;
          (*ptr_to_globals).sort_function[0] = Some(
            pcpu_sort
              as unsafe extern "C" fn(_: *mut top_status_t, _: *mut top_status_t) -> libc::c_int,
          );
          (*ptr_to_globals).sort_function[1] = Some(
            mem_sort
              as unsafe extern "C" fn(_: *mut top_status_t, _: *mut top_status_t) -> libc::c_int,
          );
          (*ptr_to_globals).sort_function[2] = Some(
            time_sort
              as unsafe extern "C" fn(_: *mut top_status_t, _: *mut top_status_t) -> libc::c_int,
          );
          continue;
        } else if c == 't' as i32 {
          scan_mask = TOP_MASK as libc::c_int as libc::c_uint;
          (*ptr_to_globals).sort_function[0] = Some(
            time_sort
              as unsafe extern "C" fn(_: *mut top_status_t, _: *mut top_status_t) -> libc::c_int,
          );
          (*ptr_to_globals).sort_function[1] = Some(
            mem_sort
              as unsafe extern "C" fn(_: *mut top_status_t, _: *mut top_status_t) -> libc::c_int,
          );
          (*ptr_to_globals).sort_function[2] = Some(
            pcpu_sort
              as unsafe extern "C" fn(_: *mut top_status_t, _: *mut top_status_t) -> libc::c_int,
          );
          continue;
        } else if c == 's' as i32 {
          scan_mask = TOPMEM_MASK as libc::c_int as libc::c_uint;
          (*ptr_to_globals).sort_field = (((*ptr_to_globals).sort_field as libc::c_int + 1i32)
            % NUM_SORT_FIELD as libc::c_int) as smallint;
          free((*ptr_to_globals).prev_hist as *mut libc::c_void);
          (*ptr_to_globals).prev_hist = 0 as *mut save_hist;
          (*ptr_to_globals).prev_hist_count = 0i32 as libc::c_uint;
          continue;
        } else if c == 'r' as i32 {
          (*ptr_to_globals).inverted =
            ((*ptr_to_globals).inverted as libc::c_int ^ 1i32) as smallint;
          continue;
        } else {
          /* procps-2.0.18 uses 'C', 3.2.7 uses '1' */
          if !(c == 'c' as i32 || c == '1' as i32) {
            break;
          }
          /* User wants to toggle per cpu <> aggregate */
          if (*ptr_to_globals).smp_cpu_info != 0 {
            free((*ptr_to_globals).cpu_prev_jif as *mut libc::c_void);
            free((*ptr_to_globals).cpu_jif as *mut libc::c_void);
            (*ptr_to_globals).cpu_jif = &mut (*ptr_to_globals).cur_jif;
            (*ptr_to_globals).cpu_prev_jif = &mut (*ptr_to_globals).prev_jif
          } else {
            /* Prepare for xrealloc() */
            (*ptr_to_globals).cpu_prev_jif = 0 as *mut jiffy_counts_t;
            (*ptr_to_globals).cpu_jif = (*ptr_to_globals).cpu_prev_jif
          }
          (*ptr_to_globals).num_cpus = 0i32 as libc::c_uint;
          (*ptr_to_globals).smp_cpu_info =
            ((*ptr_to_globals).smp_cpu_info == 0) as libc::c_int as smallint;
          get_jiffy_counts();
          continue;
        }
      }
      if (*ptr_to_globals).scroll_ofs >= (*ptr_to_globals).ntop {
        (*ptr_to_globals).scroll_ofs = (*ptr_to_globals).ntop - 1i32
      }
      if (*ptr_to_globals).scroll_ofs < 0i32 {
        (*ptr_to_globals).scroll_ofs = 0i32
      }
      return NO_RESCAN_MASK as libc::c_uint;
    }
  }
  return scan_mask;
}
//usage:#if ENABLE_FEATURE_SHOW_THREADS || ENABLE_FEATURE_TOP_SMP_CPU
//usage:# define IF_SHOW_THREADS_OR_TOP_SMP(...) __VA_ARGS__
//usage:#else
//usage:# define IF_SHOW_THREADS_OR_TOP_SMP(...)
//usage:#endif
//usage:#define top_trivial_usage
//usage:       "[-b"IF_FEATURE_TOPMEM("m")IF_FEATURE_SHOW_THREADS("H")"]"
//usage:       " [-n COUNT] [-d SECONDS]"
//usage:#define top_full_usage "\n\n"
//usage:       "Provide a view of process activity in real time."
//usage:   "\n""Read the status of all processes from /proc each SECONDS"
//usage:   "\n""and display a screenful of them."
//usage:   "\n"
//usage:	IF_FEATURE_TOP_INTERACTIVE(
//usage:       "Keys:"
//usage:   "\n""	N/M"
//usage:                IF_FEATURE_TOP_CPU_USAGE_PERCENTAGE("/P")
//usage:                IF_FEATURE_TOP_CPU_USAGE_PERCENTAGE("/T")
//usage:           ": " IF_FEATURE_TOPMEM("show CPU usage, ") "sort by pid/mem"
//usage:                IF_FEATURE_TOP_CPU_USAGE_PERCENTAGE("/cpu")
//usage:                IF_FEATURE_TOP_CPU_USAGE_PERCENTAGE("/time")
//usage:	IF_FEATURE_TOPMEM(
//usage:   "\n""	S: show memory"
//usage:	)
//usage:   "\n""	R: reverse sort"
//usage:	IF_SHOW_THREADS_OR_TOP_SMP(
//usage:   "\n""	"
//usage:                IF_FEATURE_SHOW_THREADS("H: toggle threads")
//usage:                IF_FEATURE_SHOW_THREADS(IF_FEATURE_TOP_SMP_CPU(", "))
//usage:                IF_FEATURE_TOP_SMP_CPU("1: toggle SMP")
//usage:	)
//usage:   "\n""	Q,^C: exit"
//usage:   "\n""Options:"
//usage:	)
//usage:   "\n""	-b	Batch mode"
//usage:   "\n""	-n N	Exit after N iterations"
//usage:   "\n""	-d SEC	Delay between updates"
//usage:	IF_FEATURE_TOPMEM(
//usage:   "\n""	-m	Same as 's' key"
//usage:	)
//usage:	IF_FEATURE_SHOW_THREADS(
//usage:   "\n""	-H	Show threads"
//usage:	)
/* Interactive testing:
 * echo sss | ./busybox top
 * - shows memory screen
 * echo sss | ./busybox top -bn1 >mem
 * - saves memory screen - the *whole* list, not first NROWS processes!
 * echo .m.s.s.s.s.s.s.q | ./busybox top -b >z
 * - saves several different screens, and exits
 *
 * TODO: -i STRING param as a better alternative?
 */
#[no_mangle]
pub unsafe extern "C" fn top_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut interval: duration_t = 0.; /* default update interval is 5 seconds */
  let mut iterations: libc::c_int = 0; /* infinite */
  let mut col: libc::c_uint = 0;
  let mut str_interval: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut str_iterations: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut scan_mask: libc::c_uint = TOP_MASK as libc::c_int as libc::c_uint;
  let ref mut fresh1 = *(not_const_pp(&ptr_to_globals as *const *mut globals as *const libc::c_void)
    as *mut *mut globals);
  *fresh1 = xzalloc(::std::mem::size_of::<globals>() as libc::c_ulong) as *mut globals;
  asm!("" : : : "memory" : "volatile");
  interval = 5i32 as duration_t;
  iterations = 0i32;
  /*num_cpus = 0;*/
  /*smp_cpu_info = 0;*/
  /* to start with show aggregate */
  (*ptr_to_globals).cpu_jif = &mut (*ptr_to_globals).cur_jif;
  (*ptr_to_globals).cpu_prev_jif = &mut (*ptr_to_globals).prev_jif;
  /* all args are options; -n NUM */
  make_all_argv_opts(argv); /* options can be specified w/o dash */
  col = getopt32(
    argv,
    b"d:n:bHm\x00" as *const u8 as *const libc::c_char,
    &mut str_interval as *mut *mut libc::c_char,
    &mut str_iterations as *mut *mut libc::c_char,
  );
  /* NB: -m and -H are accepted even if not configured */
  if col & OPT_m as libc::c_int as libc::c_uint != 0 {
    /* -m (busybox specific) */
    scan_mask = TOPMEM_MASK as libc::c_int as libc::c_uint
  }
  if col & OPT_d as libc::c_int as libc::c_uint != 0 {
    /* work around for "-d 1" -> "-d -1" done by make_all_argv_opts() */
    if *str_interval.offset(0) as libc::c_int == '-' as i32 {
      str_interval = str_interval.offset(1)
    }
    interval = parse_duration_str(str_interval);
    /* Need to limit it to not overflow poll timeout */
    if interval > (2147483647i32 / 1000i32) as libc::c_double {
      interval = (2147483647i32 / 1000i32) as duration_t
    }
  }
  if col & OPT_n as libc::c_int as libc::c_uint != 0 {
    if *str_iterations.offset(0) as libc::c_int == '-' as i32 {
      str_iterations = str_iterations.offset(1)
    }
    iterations = xatou(str_iterations) as libc::c_int
  }
  if col & OPT_H as libc::c_int as libc::c_uint != 0 {
    scan_mask |= PSSCAN_TASKS as libc::c_int as libc::c_uint
  }
  /* change to /proc */
  xchdir(b"/proc\x00" as *const u8 as *const libc::c_char);
  (*ptr_to_globals).sort_function[0] = Some(
    pcpu_sort as unsafe extern "C" fn(_: *mut top_status_t, _: *mut top_status_t) -> libc::c_int,
  );
  (*ptr_to_globals).sort_function[1] = Some(
    mem_sort as unsafe extern "C" fn(_: *mut top_status_t, _: *mut top_status_t) -> libc::c_int,
  );
  (*ptr_to_globals).sort_function[2] = Some(
    time_sort as unsafe extern "C" fn(_: *mut top_status_t, _: *mut top_status_t) -> libc::c_int,
  );
  if option_mask32 & OPT_b as libc::c_int as libc::c_uint != 0 {
    option_mask32 |= OPT_EOF as libc::c_int as libc::c_uint
  } else {
    /* Turn on unbuffered input; turn off echoing, ^C ^Z etc */
    set_termios_to_raw(0i32, &mut (*ptr_to_globals).initial_settings, 1i32 << 0i32);
    die_func = Some(reset_term as unsafe extern "C" fn() -> ())
  }
  bb_signals(
    BB_FATAL_SIGS as libc::c_int,
    Some(sig_catcher as unsafe extern "C" fn(_: libc::c_int) -> ()),
  );
  /* Eat initial input, if any */
  scan_mask = handle_input(scan_mask, 0i32 as duration_t); /* end of "while (not Q)" */
  's_204: while scan_mask != EXIT_MASK as libc::c_int as libc::c_uint {
    let mut new_mask: libc::c_uint = 0;
    let mut p: *mut procps_status_t = 0 as *mut procps_status_t;
    if option_mask32 & OPT_b as libc::c_int as libc::c_uint != 0 {
      (*ptr_to_globals).lines = 2147483647i32 as libc::c_uint;
      col = (LINE_BUF_SIZE as libc::c_int - 2i32) as libc::c_uint
    /* +2 bytes for '\n', NUL */
    } else {
      (*ptr_to_globals).lines = 24i32 as libc::c_uint; /* default */
      col = 79i32 as libc::c_uint;
      /* We output to stdout, we need size of stdout (not stdin)! */
      get_terminal_width_height(1i32, &mut col, &mut (*ptr_to_globals).lines);
      if (*ptr_to_globals).lines < 5i32 as libc::c_uint || col < 10i32 as libc::c_uint {
        sleep_for_duration(interval);
        continue;
      } else if col > (LINE_BUF_SIZE as libc::c_int - 2i32) as libc::c_uint {
        col = (LINE_BUF_SIZE as libc::c_int - 2i32) as libc::c_uint
      }
    }
    /* read process IDs & status for all the processes */
    (*ptr_to_globals).ntop = 0i32; /* end of "while we read /proc" */
    loop {
      p = procps_scan(p, scan_mask as libc::c_int);
      if p.is_null() {
        break;
      }
      let mut n: libc::c_int = 0;
      if scan_mask != TOPMEM_MASK as libc::c_int as libc::c_uint {
        n = (*ptr_to_globals).ntop;
        let fresh2 = (*ptr_to_globals).ntop;
        (*ptr_to_globals).ntop = (*ptr_to_globals).ntop + 1;
        (*ptr_to_globals).top = xrealloc_vector_helper(
          (*ptr_to_globals).top as *mut libc::c_void,
          ((::std::mem::size_of::<top_status_t>() as libc::c_ulong) << 8i32)
            .wrapping_add(6i32 as libc::c_ulong) as libc::c_uint,
          fresh2,
        ) as *mut top_status_t;
        (*(*ptr_to_globals).top.offset(n as isize)).pid = (*p).pid;
        (*(*ptr_to_globals).top.offset(n as isize)).ppid = (*p).ppid;
        (*(*ptr_to_globals).top.offset(n as isize)).vsz = (*p).vsz;
        (*(*ptr_to_globals).top.offset(n as isize)).ticks = (*p).stime.wrapping_add((*p).utime);
        (*(*ptr_to_globals).top.offset(n as isize)).uid = (*p).uid;
        strcpy(
          (*(*ptr_to_globals).top.offset(n as isize))
            .state
            .as_mut_ptr(),
          (*p).state.as_mut_ptr(),
        );
        strcpy(
          (*(*ptr_to_globals).top.offset(n as isize))
            .comm
            .as_mut_ptr(),
          (*p).comm.as_mut_ptr(),
        );
        (*(*ptr_to_globals).top.offset(n as isize)).last_seen_on_cpu = (*p).last_seen_on_cpu
      } else {
        /* TOPMEM */
        if (*p).smaps.mapped_ro | (*p).smaps.mapped_rw == 0 {
          continue; /* kernel threads are ignored */
        }
        n = (*ptr_to_globals).ntop;
        /* No bug here - top and topmem are the same */
        let fresh3 = (*ptr_to_globals).ntop;
        (*ptr_to_globals).ntop = (*ptr_to_globals).ntop + 1;
        (*ptr_to_globals).top = xrealloc_vector_helper(
          (*ptr_to_globals).top as *mut topmem_status_t as *mut libc::c_void,
          ((::std::mem::size_of::<topmem_status_t>() as libc::c_ulong) << 8i32)
            .wrapping_add(6i32 as libc::c_ulong) as libc::c_uint,
          fresh3,
        ) as *mut top_status_t;
        strcpy(
          (*((*ptr_to_globals).top as *mut topmem_status_t).offset(n as isize))
            .comm
            .as_mut_ptr(),
          (*p).comm.as_mut_ptr(),
        );
        (*((*ptr_to_globals).top as *mut topmem_status_t).offset(n as isize)).pid = (*p).pid;
        (*((*ptr_to_globals).top as *mut topmem_status_t).offset(n as isize)).vsz =
          (*p).smaps.mapped_rw.wrapping_add((*p).smaps.mapped_ro);
        (*((*ptr_to_globals).top as *mut topmem_status_t).offset(n as isize)).vszrw =
          (*p).smaps.mapped_rw;
        (*((*ptr_to_globals).top as *mut topmem_status_t).offset(n as isize)).rss_sh = (*p)
          .smaps
          .shared_clean
          .wrapping_add((*p).smaps.shared_dirty);
        (*((*ptr_to_globals).top as *mut topmem_status_t).offset(n as isize)).rss = (*p)
          .smaps
          .private_clean
          .wrapping_add((*p).smaps.private_dirty)
          .wrapping_add(
            (*((*ptr_to_globals).top as *mut topmem_status_t).offset(n as isize)).rss_sh,
          );
        (*((*ptr_to_globals).top as *mut topmem_status_t).offset(n as isize)).dirty = (*p)
          .smaps
          .private_dirty
          .wrapping_add((*p).smaps.shared_dirty);
        (*((*ptr_to_globals).top as *mut topmem_status_t).offset(n as isize)).dirty_sh =
          (*p).smaps.shared_dirty;
        (*((*ptr_to_globals).top as *mut topmem_status_t).offset(n as isize)).stack =
          (*p).smaps.stack
      }
    }
    if (*ptr_to_globals).ntop == 0i32 {
      bb_simple_error_msg(b"no process info in /proc\x00" as *const u8 as *const libc::c_char);
      break;
    } else {
      if scan_mask != TOPMEM_MASK as libc::c_int as libc::c_uint {
        if (*ptr_to_globals).prev_hist_count == 0 {
          do_stats();
          usleep(100000i32 as useconds_t);
          clearmems();
          continue;
        } else {
          do_stats();
          /* TODO: we don't need to sort all 10000 processes, we need to find top 24! */
          qsort(
            (*ptr_to_globals).top as *mut libc::c_void,
            (*ptr_to_globals).ntop as size_t,
            ::std::mem::size_of::<top_status_t>() as libc::c_ulong,
            ::std::mem::transmute::<*mut libc::c_void, __compar_fn_t>(::std::mem::transmute::<
              Option<
                unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void) -> libc::c_int,
              >,
              *mut libc::c_void,
            >(Some(
              mult_lvl_cmp
                as unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void) -> libc::c_int,
            ))),
          );
        }
      } else {
        /* TOPMEM */
        qsort(
          (*ptr_to_globals).top as *mut topmem_status_t as *mut libc::c_void,
          (*ptr_to_globals).ntop as size_t,
          ::std::mem::size_of::<topmem_status_t>() as libc::c_ulong,
          ::std::mem::transmute::<*mut libc::c_void, __compar_fn_t>(::std::mem::transmute::<
            Option<unsafe extern "C" fn(_: *mut libc::c_char, _: *mut libc::c_char) -> libc::c_int>,
            *mut libc::c_void,
          >(Some(
            topmem_sort
              as unsafe extern "C" fn(_: *mut libc::c_char, _: *mut libc::c_char) -> libc::c_int,
          ))),
        );
      }
      loop {
        if scan_mask != TOPMEM_MASK as libc::c_int as libc::c_uint {
          display_process_list((*ptr_to_globals).lines as libc::c_int, col as libc::c_int);
        } else {
          /* TOPMEM */
          display_topmem_process_list((*ptr_to_globals).lines as libc::c_int, col as libc::c_int);
        }
        if iterations >= 0i32 && {
          iterations -= 1;
          (iterations) == 0
        } {
          break 's_204;
        }
        new_mask = handle_input(scan_mask, interval);
        if !(new_mask == NO_RESCAN_MASK as libc::c_uint) {
          break;
        }
      }
      scan_mask = new_mask;
      clearmems();
    }
  }
  bb_putchar('\n' as i32);
  reset_term();
  return 0i32;
}
