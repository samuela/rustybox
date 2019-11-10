use crate::librb::__compar_fn_t;

use crate::librb::size_t;
use crate::librb::smallint;
use libc;












use libc::opendir;
use libc::closedir;
use libc::readdir;

















use libc::geteuid;











use libc::strcpy;






use libc::fclose;


use libc::printf;
use libc::puts;


use libc::sprintf;
use libc::ssize_t;
use libc::strchr;
use libc::strcmp;

use libc::strstr;

use libc::termios;

use libc::free;

use libc::FILE;

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
  fn _exit(_: libc::c_int) -> !;

  #[no_mangle]
  static ptr_to_globals: *mut globals;



  #[no_mangle]
  static mut stdout: *mut FILE;

  #[no_mangle]
  fn fflush(__stream: *mut FILE) -> libc::c_int;

  #[no_mangle]
  fn fgets_unlocked(
    __s: *mut libc::c_char,
    __n: libc::c_int,
    __stream: *mut FILE,
  ) -> *mut libc::c_char;

  #[no_mangle]
  fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn chomp(s: *mut libc::c_char);
  #[no_mangle]
  fn skip_whitespace(_: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn skip_non_whitespace(_: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xrealloc_vector_helper(
    vector: *mut libc::c_void,
    sizeof_and_shift: libc::c_uint,
    idx: libc::c_int,
  ) -> *mut libc::c_void;
  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn is_prefixed_with(string: *const libc::c_char, key: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_signals(sigs: libc::c_int, f: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>);
  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn fopen_for_read(path: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn xfopen_for_read(path: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn fopen_for_write(path: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn safe_poll(ufds: *mut pollfd, nfds: nfds_t, timeout_ms: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn smart_ulltoa5(
    ul: libc::c_ulonglong,
    buf: *mut libc::c_char,
    scale: *const libc::c_char,
  ) -> *mut libc::c_char;
  /* Non-aborting kind of convertors: bb_strto[u][l]l */
  /* On exit: errno = 0 only if there was non-empty, '\0' terminated value
   * errno = EINVAL if value was not '\0' terminated, but otherwise ok
   *    Return value is still valid, caller should just check whether end[0]
   *    is a valid terminating char for particular case. OTOH, if caller
   *    requires '\0' terminated input, [s]he can just check errno == 0.
   * errno = ERANGE if value had alphanumeric terminating char ("1234abcg").
   * errno = ERANGE if value is out of range, missing, etc.
   * errno = ERANGE if value had minus sign for strtouXX (even "-0" is not ok )
   *    return value is all-ones in this case.
   */
  #[no_mangle]
  fn bb_strtoull(
    arg: *const libc::c_char,
    endp: *mut *mut libc::c_char,
    base: libc::c_int,
  ) -> libc::c_ulonglong;
  #[no_mangle]
  fn bb_strtou(
    arg: *const libc::c_char,
    endp: *mut *mut libc::c_char,
    base: libc::c_int,
  ) -> libc::c_uint;
  #[no_mangle]
  static mut die_func: Option<unsafe extern "C" fn() -> ()>;
  #[no_mangle]
  fn bb_simple_error_msg(s: *const libc::c_char);
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
  fn get_cpu_count() -> libc::c_uint;
}

use libc::dirent;
use libc::DIR;

pub type nfds_t = libc::c_ulong;
use libc::pollfd;

pub type C2RustUnnamed = libc::c_uint;
pub const BB_FATAL_SIGS: C2RustUnnamed = 117503054;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub lines: *mut line,
  pub lines_cnt: libc::c_int,
  pub lines_cumulative_count: libc::c_int,
  pub maxcstate: libc::c_int,
  pub total_cpus: libc::c_uint,
  pub cant_enable_timer_stats: smallint,
  pub interrupt_0: libc::c_int,
  pub total_interrupt: libc::c_int,
  pub interrupts: [irqdata; 40],
  pub start_usage: [ullong; 8],
  pub last_usage: [ullong; 8],
  pub start_duration: [ullong; 8],
  pub last_duration: [ullong; 8],
  pub init_settings: termios,
}
pub type ullong = libc::c_ulonglong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct irqdata {
  pub active: smallint,
  pub number: libc::c_int,
  pub count: ullong,
  pub irq_desc: [libc::c_char; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct line {
  pub string: *mut libc::c_char,
  pub count: libc::c_int,
}
#[inline(always)]
unsafe extern "C" fn not_const_pp(mut p: *const libc::c_void) -> *mut libc::c_void {
  return p as *mut libc::c_void;
}
unsafe extern "C" fn reset_term() {
  tcsetattr_stdin_TCSANOW(&mut (*ptr_to_globals).init_settings);
}
unsafe extern "C" fn sig_handler(mut _signo: libc::c_int) {
  reset_term();
  _exit(1i32);
}
unsafe extern "C" fn write_str_to_file(
  mut fname: *const libc::c_char,
  mut str: *const libc::c_char,
) -> libc::c_int {
  let mut fp: *mut FILE = fopen_for_write(fname);
  if fp.is_null() {
    return 1i32;
  }
  fputs_unlocked(str, fp);
  fclose(fp);
  return 0i32;
}
/* Make it more readable */
#[inline(never)]
unsafe extern "C" fn clear_lines() {
  let mut i: libc::c_int = 0;
  if !(*ptr_to_globals).lines.is_null() {
    i = 0i32;
    while i < (*ptr_to_globals).lines_cnt {
      free((*(*ptr_to_globals).lines.offset(i as isize)).string as *mut libc::c_void);
      i += 1
    }
    free((*ptr_to_globals).lines as *mut libc::c_void);
    (*ptr_to_globals).lines_cnt = 0i32;
    (*ptr_to_globals).lines = 0 as *mut line
  };
}
unsafe extern "C" fn update_lines_cumulative_count() {
  let mut i: libc::c_int = 0;
  i = 0i32;
  while i < (*ptr_to_globals).lines_cnt {
    (*ptr_to_globals).lines_cumulative_count += (*(*ptr_to_globals).lines.offset(i as isize)).count;
    i += 1
  }
}
unsafe extern "C" fn line_compare(
  mut p1: *const libc::c_void,
  mut p2: *const libc::c_void,
) -> libc::c_int {
  let mut a: *const line = p1 as *const line;
  let mut b: *const line = p2 as *const line;
  return (*b).count - (*a).count;
}
unsafe extern "C" fn sort_lines() {
  qsort(
    (*ptr_to_globals).lines as *mut libc::c_void,
    (*ptr_to_globals).lines_cnt as size_t,
    ::std::mem::size_of::<line>() as libc::c_ulong,
    Some(
      line_compare
        as unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> libc::c_int,
    ),
  );
}
/* Save C-state usage and duration. Also update maxcstate. */
unsafe extern "C" fn read_cstate_counts(mut usage: *mut ullong, mut duration: *mut ullong) {
  let mut dir: *mut DIR = 0 as *mut DIR; /* "CPUnn" */
  let mut d: *mut dirent = 0 as *mut dirent;
  dir = opendir(b"/proc/acpi/processor\x00" as *const u8 as *const libc::c_char);
  if dir.is_null() {
    return;
  }
  loop {
    d = readdir(dir);
    if d.is_null() {
      break;
    }
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut buf: [libc::c_char; 192] = [0; 192];
    let mut level: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    len = strlen((*d).d_name.as_mut_ptr()) as libc::c_int;
    if len < 3i32 || len > 16i32 {
      continue;
    }
    sprintf(
      buf.as_mut_ptr(),
      b"%s/%s/power\x00" as *const u8 as *const libc::c_char,
      b"/proc/acpi/processor\x00" as *const u8 as *const libc::c_char,
      (*d).d_name.as_mut_ptr(),
    );
    fp = fopen_for_read(buf.as_mut_ptr());
    if fp.is_null() {
      continue;
    }
    // Example file contents:
    // active state:            C0
    // max_cstate:              C8
    // maximum allowed latency: 2000000000 usec
    // states:
    //     C1:                  type[C1] promotion[--] demotion[--] latency[001] usage[00006173] duration[00000000000000000000]
    //     C2:                  type[C2] promotion[--] demotion[--] latency[001] usage[00085191] duration[00000000000083024907]
    //     C3:                  type[C3] promotion[--] demotion[--] latency[017] usage[01017622] duration[00000000017921327182]
    level = 0i32;
    while !fgets_unlocked(
      buf.as_mut_ptr(),
      ::std::mem::size_of::<[libc::c_char; 192]>() as libc::c_ulong as libc::c_int,
      fp,
    )
    .is_null()
    {
      let mut p: *mut libc::c_char = strstr(
        buf.as_mut_ptr(),
        b"age[\x00" as *const u8 as *const libc::c_char,
      );
      if p.is_null() {
        continue;
      }
      p = p.offset(4);
      let ref mut fresh0 = *usage.offset(level as isize);
      *fresh0 = (*fresh0 as libc::c_ulonglong).wrapping_add(
        bb_strtoull(p, 0 as *mut *mut libc::c_char, 10i32).wrapping_add(1i32 as libc::c_ulonglong),
      ) as ullong as ullong;
      p = strstr(
        buf.as_mut_ptr(),
        b"ation[\x00" as *const u8 as *const libc::c_char,
      );
      if p.is_null() {
        continue;
      }
      p = p.offset(6);
      let ref mut fresh1 = *duration.offset(level as isize);
      *fresh1 = (*fresh1 as libc::c_ulonglong).wrapping_add(bb_strtoull(
        p,
        0 as *mut *mut libc::c_char,
        10i32,
      )) as ullong as ullong;
      if level >= 8i32 - 1i32 {
        break;
      }
      level += 1;
      if level > (*ptr_to_globals).maxcstate {
        /* update maxcstate */
        (*ptr_to_globals).maxcstate = level
      }
    }
    fclose(fp);
  }
  closedir(dir);
}
/* Add line and/or update count */
unsafe extern "C" fn save_line(mut string: *const libc::c_char, mut count: libc::c_int) {
  let mut i: libc::c_int = 0;
  i = 0i32;
  while i < (*ptr_to_globals).lines_cnt {
    if strcmp(string, (*(*ptr_to_globals).lines.offset(i as isize)).string) == 0i32 {
      /* It's already there, only update count */
      (*(*ptr_to_globals).lines.offset(i as isize)).count += count;
      return;
    }
    i += 1
  }
  /* Add new line */
  (*ptr_to_globals).lines = xrealloc_vector_helper(
    (*ptr_to_globals).lines as *mut libc::c_void,
    ((::std::mem::size_of::<line>() as libc::c_ulong) << 8i32).wrapping_add(4i32 as libc::c_ulong)
      as libc::c_uint,
    (*ptr_to_globals).lines_cnt,
  ) as *mut line;
  let ref mut fresh2 = (*(*ptr_to_globals)
    .lines
    .offset((*ptr_to_globals).lines_cnt as isize))
  .string;
  *fresh2 = xstrdup(string);
  (*(*ptr_to_globals)
    .lines
    .offset((*ptr_to_globals).lines_cnt as isize))
  .count = count;
  /*G.lines[G.lines_cnt].disk_count = 0;*/
  (*ptr_to_globals).lines_cnt += 1;
}
unsafe extern "C" fn is_hpet_irq(mut name: *const libc::c_char) -> libc::c_int {
  let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
  //TODO: optimize
  p = strstr(name, b"hpet\x00" as *const u8 as *const libc::c_char);
  if p.is_null() {
    return 0i32;
  }
  p = p.offset(4);
  if !((*p as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32) {
    return 0i32;
  }
  return 1i32;
}
/* Save new IRQ count, return delta from old one */
unsafe extern "C" fn save_irq_count(mut irq: libc::c_int, mut count: ullong) -> libc::c_int {
  let mut unused: libc::c_int = 40i32;
  let mut i: libc::c_int = 0;
  i = 0i32;
  while i < 40i32 {
    if (*ptr_to_globals).interrupts[i as usize].active as libc::c_int != 0
      && (*ptr_to_globals).interrupts[i as usize].number == irq
    {
      let mut old: ullong = (*ptr_to_globals).interrupts[i as usize].count;
      (*ptr_to_globals).interrupts[i as usize].count = count;
      return count.wrapping_sub(old) as libc::c_int;
    }
    if (*ptr_to_globals).interrupts[i as usize].active == 0 && unused > i {
      unused = i
    }
    i += 1
  }
  if unused < 40i32 {
    (*ptr_to_globals).interrupts[unused as usize].active = 1i32 as smallint;
    (*ptr_to_globals).interrupts[unused as usize].count = count;
    (*ptr_to_globals).interrupts[unused as usize].number = irq
  }
  return count as libc::c_int;
}
/* Read /proc/interrupts, save IRQ counts and IRQ description */
unsafe extern "C" fn process_irq_counts() {
  let mut fp: *mut FILE = 0 as *mut FILE;
  let mut buf: [libc::c_char; 128] = [0; 128];
  /* Reset values */
  (*ptr_to_globals).interrupt_0 = 0i32;
  (*ptr_to_globals).total_interrupt = 0i32;
  fp = xfopen_for_read(b"/proc/interrupts\x00" as *const u8 as *const libc::c_char);
  while !fgets_unlocked(
    buf.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
    fp,
  )
  .is_null()
  {
    let mut irq_desc: [libc::c_char; 147] = [0; 147];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut nr: libc::c_int = 0;
    let mut count: ullong = 0;
    let mut delta: ullong = 0;
    p = strchr(buf.as_mut_ptr(), ':' as i32);
    if p.is_null() {
      continue;
    }
    /*  0:  143646045  153901007   IO-APIC-edge      timer
     *   ^
     */
    *p = '\u{0}' as i32 as libc::c_char;
    /* Deal with non-maskable interrupts -- make up fake numbers */
    nr = index_in_strings(
      b"NMI\x00RES\x00CAL\x00TLB\x00TRM\x00THR\x00SPU\x00\x00" as *const u8 as *const libc::c_char,
      buf.as_mut_ptr(),
    );
    if nr >= 0i32 {
      nr += 20000i32
    } else {
      /* bb_strtou doesn't eat leading spaces, using strtoul */
      *bb_errno = 0i32;
      nr = strtoul(buf.as_mut_ptr(), 0 as *mut *mut libc::c_char, 10i32) as libc::c_int;
      if *bb_errno != 0 {
        continue;
      }
    }
    p = p.offset(1);
    /*  0:  143646045  153901007   IO-APIC-edge      timer
     *    ^
     */
    /* Sum counts for this IRQ */
    count = 0i32 as ullong;
    loop {
      let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
      p = skip_whitespace(p);
      if !((*p as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32) {
        break;
      }
      count = (count as libc::c_ulonglong).wrapping_add(bb_strtoull(p, &mut tmp, 10i32)) as ullong
        as ullong;
      p = tmp
    }
    /*   0:  143646045  153901007   IO-APIC-edge      timer
     * NMI:          1          2   Non-maskable interrupts
     *                              ^
     */
    if nr < 20000i32 {
      /* Skip to the interrupt name, e.g. 'timer' */
      p = strchr(p, ' ' as i32);
      if p.is_null() {
        continue;
      }
      p = skip_whitespace(p)
    }
    name = p;
    chomp(p);
    /* Save description of the interrupt */
    if nr >= 20000i32 {
      sprintf(
        irq_desc.as_mut_ptr(),
        b"   <kernel IPI> : %s\x00" as *const u8 as *const libc::c_char,
        name,
      );
    } else {
      sprintf(
        irq_desc.as_mut_ptr(),
        b"    <interrupt> : %s\x00" as *const u8 as *const libc::c_char,
        name,
      );
    }
    delta = save_irq_count(nr, count) as ullong;
    /* Skip per CPU timer interrupts */
    if is_hpet_irq(name) != 0 {
      continue;
    }
    if nr != 0i32 && delta != 0i32 as libc::c_ulonglong {
      save_line(irq_desc.as_mut_ptr(), delta as libc::c_int);
    }
    if nr == 0i32 {
      (*ptr_to_globals).interrupt_0 = delta as libc::c_int
    } else {
      (*ptr_to_globals).total_interrupt = ((*ptr_to_globals).total_interrupt as libc::c_ulonglong)
        .wrapping_add(delta) as libc::c_int as libc::c_int
    }
  }
  fclose(fp);
}
/* !ENABLE_FEATURE_POWERTOP_PROCIRQ */
#[inline(never)]
unsafe extern "C" fn process_timer_stats() -> libc::c_int {
  let mut buf: [libc::c_char; 128] = [0; 128];
  let mut line: [libc::c_char; 146] = [0; 146];
  let mut n: libc::c_int = 0;
  let mut fp: *mut FILE = 0 as *mut FILE;
  buf[0] = '\u{0}' as i32 as libc::c_char;
  n = 0i32;
  fp = 0 as *mut FILE;
  if (*ptr_to_globals).cant_enable_timer_stats == 0 {
    fp = fopen_for_read(b"/proc/timer_stats\x00" as *const u8 as *const libc::c_char)
  }
  if !fp.is_null() {
    // Example file contents:
    // Timer Stats Version: v0.2
    // Sample period: 1.329 s
    //    76,     0 swapper          hrtimer_start_range_ns (tick_sched_timer)
    //    88,     0 swapper          hrtimer_start_range_ns (tick_sched_timer)
    //    24,  3787 firefox          hrtimer_start_range_ns (hrtimer_wakeup)
    //   46D,  1136 kondemand/1      do_dbs_timer (delayed_work_timer_fn)
    // ...
    //     1,  1656 Xorg             hrtimer_start_range_ns (hrtimer_wakeup)
    //     1,  2159 udisks-daemon    hrtimer_start_range_ns (hrtimer_wakeup)
    // 331 total events, 249.059 events/sec
    's_39: while !fgets_unlocked(
      buf.as_mut_ptr(),
      ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
      fp,
    )
    .is_null()
    {
      let mut count: *const libc::c_char = 0 as *const libc::c_char; /* deferred */
      let mut process: *const libc::c_char = 0 as *const libc::c_char; /* points to pid now */
      let mut func: *const libc::c_char = 0 as *const libc::c_char;
      let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
      let mut idx: libc::c_int = 0;
      let mut cnt: libc::c_uint = 0;
      count = skip_whitespace(buf.as_mut_ptr());
      p = strchr(count, ',' as i32);
      if p.is_null() {
        continue;
      }
      let fresh3 = p;
      p = p.offset(1);
      *fresh3 = '\u{0}' as i32 as libc::c_char;
      cnt = bb_strtou(count, 0 as *mut *mut libc::c_char, 10i32);
      if strcmp(
        skip_non_whitespace(count),
        b" total events\x00" as *const u8 as *const libc::c_char,
      ) == 0i32
      {
        n = cnt.wrapping_div((*ptr_to_globals).total_cpus) as libc::c_int;
        if n > 0i32 && n < (*ptr_to_globals).interrupt_0 {
          sprintf(
            line.as_mut_ptr(),
            b"    <interrupt> : %s\x00" as *const u8 as *const libc::c_char,
            b"extra timer interrupt\x00" as *const u8 as *const libc::c_char,
          );
          save_line(line.as_mut_ptr(), (*ptr_to_globals).interrupt_0 - n);
        }
        break;
      } else {
        if !strchr(count, 'D' as i32).is_null() {
          continue;
        }
        p = skip_whitespace(p);
        process = 0 as *const libc::c_char;
        loop {
          p = strchr(p, ' ' as i32);
          if p.is_null() {
            continue 's_39;
          }
          let fresh4 = p;
          p = p.offset(1);
          *fresh4 = '\u{0}' as i32 as libc::c_char;
          p = skip_whitespace(p);
          if !process.is_null() {
            break;
          }
          process = p
        }
        func = p;
        //if (strcmp(process, "swapper") == 0
        // && strcmp(func, "hrtimer_start_range_ns (tick_sched_timer)\n") == 0
        //) {
        //	process = "[kernel scheduler]";
        //	func = "Load balancing tick";
        //}
        if !is_prefixed_with(func, b"tick_nohz_\x00" as *const u8 as *const libc::c_char).is_null()
        {
          continue;
        }
        if !is_prefixed_with(
          func,
          b"tick_setup_sched_timer\x00" as *const u8 as *const libc::c_char,
        )
        .is_null()
        {
          continue;
        }
        //if (strcmp(process, "powertop") == 0)
        //	continue;
        idx = index_in_strings(
          b"insmod\x00modprobe\x00swapper\x00\x00" as *const u8 as *const libc::c_char,
          process,
        );
        if idx != -1i32 {
          process = if idx < 2i32 {
            b"[kernel module]\x00" as *const u8 as *const libc::c_char
          } else {
            b"<kernel core>\x00" as *const u8 as *const libc::c_char
          }
        }
        chomp(p);
        // 46D\01136\0kondemand/1\0do_dbs_timer (delayed_work_timer_fn)
        // ^          ^            ^
        // count      process      func
        //if (strchr(process, '['))
        sprintf(
          line.as_mut_ptr(),
          b"%15.15s : %s\x00" as *const u8 as *const libc::c_char,
          process,
          func,
        );
        //else
        //	sprintf(line, "%s", process);
        save_line(line.as_mut_ptr(), cnt as libc::c_int);
      }
    }
    fclose(fp);
  }
  return n;
}
unsafe extern "C" fn show_timerstats() {
  let mut lines: libc::c_uint = 0;
  /* Get terminal height */
  get_terminal_width_height(1i32, 0 as *mut libc::c_uint, &mut lines);
  /* We don't have whole terminal just for timerstats */
  lines = lines.wrapping_sub(12i32 as libc::c_uint);
  if (*ptr_to_globals).cant_enable_timer_stats == 0 {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0i32;
    let mut strbuf6: [libc::c_char; 6] = [0; 6];
    puts(b"\nTop causes for wakeups:\x00" as *const u8 as *const libc::c_char);
    i = 0i32;
    while i < (*ptr_to_globals).lines_cnt {
      if (*(*ptr_to_globals).lines.offset(i as isize)).count > 0i32 && {
        let fresh5 = n;
        n = n + 1;
        (fresh5 as libc::c_uint) < lines
      } {
        /* NB: upstream powertop prints "(wakeups/sec)",
         * we print just "(wakeup counts)".
         */
        /*char c = ' ';
        if (G.lines[i].disk_count)
          c = 'D';*/
        *smart_ulltoa5(
          (*(*ptr_to_globals).lines.offset(i as isize)).count as libc::c_ulonglong,
          strbuf6.as_mut_ptr(),
          b" KMGTPEZY\x00" as *const u8 as *const libc::c_char,
        )
        .offset(0) = '\u{0}' as i32 as libc::c_char;
        printf(
          b" %5.1f%% (%s)   %s\n\x00" as *const u8 as *const libc::c_char,
          (*(*ptr_to_globals).lines.offset(i as isize)).count as libc::c_double * 100.0f64
            / (*ptr_to_globals).lines_cumulative_count as libc::c_double,
          strbuf6.as_mut_ptr(),
          (*(*ptr_to_globals).lines.offset(i as isize)).string,
        );
      }
      i += 1
    }
  } else {
    bb_putchar('\n' as i32);
    bb_simple_error_msg(
      b"no stats available; run as root or enable the timer_stats module\x00" as *const u8
        as *const libc::c_char,
    );
  };
}
// Example display from powertop version 1.11
// Cn                Avg residency       P-states (frequencies)
// C0 (cpu running)        ( 0.5%)         2.00 Ghz     0.0%
// polling           0.0ms ( 0.0%)         1.67 Ghz     0.0%
// C1 mwait          0.0ms ( 0.0%)         1333 Mhz     0.1%
// C2 mwait          0.1ms ( 0.1%)         1000 Mhz    99.9%
// C3 mwait         12.1ms (99.4%)
//
// Wakeups-from-idle per second : 93.6     interval: 15.0s
// no ACPI power usage estimate available
//
// Top causes for wakeups:
//   32.4% ( 26.7)       <interrupt> : extra timer interrupt
//   29.0% ( 23.9)     <kernel core> : hrtimer_start_range_ns (tick_sched_timer)
//    9.0% (  7.5)     <kernel core> : hrtimer_start (tick_sched_timer)
//    6.5% (  5.3)       <interrupt> : ata_piix
//    5.0% (  4.1)             inetd : hrtimer_start_range_ns (hrtimer_wakeup)
//usage:#define powertop_trivial_usage
//usage:       ""
//usage:#define powertop_full_usage "\n\n"
//usage:       "Analyze power consumption on Intel-based laptops"
#[no_mangle]
pub unsafe extern "C" fn powertop_main(
  mut _argc: libc::c_int,
  mut _argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut cur_usage: [ullong; 8] = [0; 8];
  let mut cur_duration: [ullong; 8] = [0; 8];
  let mut cstate_lines: [[libc::c_char; 64]; 10] = [[0; 64]; 10];
  let mut pfd: [pollfd; 1] = [pollfd {
    fd: 0,
    events: 0,
    revents: 0,
  }; 1];
  pfd[0].fd = 0i32;
  pfd[0].events = 0x1i32 as libc::c_short;
  let ref mut fresh6 = *(not_const_pp(&ptr_to_globals as *const *mut globals as *const libc::c_void)
    as *mut *mut globals);
  *fresh6 = xzalloc(::std::mem::size_of::<globals>() as libc::c_ulong) as *mut globals;
  asm!("" : : : "memory" : "volatile");
  /* Print warning when we don't have superuser privileges */
  if geteuid() != 0i32 as libc::c_uint {
    bb_simple_error_msg(
      b"run as root to collect enough information\x00" as *const u8 as *const libc::c_char,
    );
  }
  /* Get number of CPUs */
  (*ptr_to_globals).total_cpus = get_cpu_count();
  puts(b"Collecting data for 10 seconds\x00" as *const u8 as *const libc::c_char);
  /* Turn on unbuffered input; turn off echoing, ^C ^Z etc */
  set_termios_to_raw(0i32, &mut (*ptr_to_globals).init_settings, 1i32 << 0i32);
  bb_signals(
    BB_FATAL_SIGS as libc::c_int,
    Some(sig_handler as unsafe extern "C" fn(_: libc::c_int) -> ()),
  );
  /* So we don't forget to reset term settings */
  die_func = Some(reset_term as unsafe extern "C" fn() -> ());
  /* Collect initial data */
  process_irq_counts();
  /* Read initial usage and duration */
  read_cstate_counts(
    (*ptr_to_globals).start_usage.as_mut_ptr(),
    (*ptr_to_globals).start_duration.as_mut_ptr(),
  );
  /* Copy them to "last" */
  memcpy(
    (*ptr_to_globals).last_usage.as_mut_ptr() as *mut libc::c_void,
    (*ptr_to_globals).start_usage.as_mut_ptr() as *const libc::c_void,
    ::std::mem::size_of::<[ullong; 8]>() as libc::c_ulong,
  );
  memcpy(
    (*ptr_to_globals).last_duration.as_mut_ptr() as *mut libc::c_void,
    (*ptr_to_globals).start_duration.as_mut_ptr() as *const libc::c_void,
    ::std::mem::size_of::<[ullong; 8]>() as libc::c_ulong,
  );
  /* Display C-states */
  (*ptr_to_globals).cant_enable_timer_stats = ((*ptr_to_globals).cant_enable_timer_stats
    as libc::c_int
    | write_str_to_file(
      b"/proc/timer_stats\x00" as *const u8 as *const libc::c_char,
      b"0\n\x00" as *const u8 as *const libc::c_char,
    )) as smallint; /* 1 on error */
  /* The main loop */
  loop
  /* for (;;) */
  //double maxsleep = 0.0;
  {
    let mut totalticks: ullong = 0; /* 1 on error */
    let mut totalevents: ullong = 0; /* EOF/error */
    let mut i: libc::c_int = 0; /* ^C */
    (*ptr_to_globals).cant_enable_timer_stats = ((*ptr_to_globals).cant_enable_timer_stats
      as libc::c_int
      | write_str_to_file(
        b"/proc/timer_stats\x00" as *const u8 as *const libc::c_char,
        b"1\n\x00" as *const u8 as *const libc::c_char,
      )) as smallint; /* 1 on error */
    if safe_poll(pfd.as_mut_ptr(), 1i32 as nfds_t, 10i32 * 1000i32) > 0i32 {
      let mut c: libc::c_uchar = 0;
      if safe_read(
        0i32,
        &mut c as *mut libc::c_uchar as *mut libc::c_void,
        1i32 as size_t,
      ) != 1
      {
        break;
      }
      if c as libc::c_int == (*ptr_to_globals).init_settings.c_cc[0] as libc::c_int {
        break;
      }
      if c as libc::c_int | 0x20i32 == 'q' as i32 {
        break;
      }
    }
    (*ptr_to_globals).cant_enable_timer_stats = ((*ptr_to_globals).cant_enable_timer_stats
      as libc::c_int
      | write_str_to_file(
        b"/proc/timer_stats\x00" as *const u8 as *const libc::c_char,
        b"0\n\x00" as *const u8 as *const libc::c_char,
      )) as smallint;
    clear_lines();
    process_irq_counts();
    /* Clear the stats */
    memset(
      cur_duration.as_mut_ptr() as *mut libc::c_void,
      0i32,
      ::std::mem::size_of::<[ullong; 8]>() as libc::c_ulong,
    );
    memset(
      cur_usage.as_mut_ptr() as *mut libc::c_void,
      0i32,
      ::std::mem::size_of::<[ullong; 8]>() as libc::c_ulong,
    );
    /* Read them */
    read_cstate_counts(cur_usage.as_mut_ptr(), cur_duration.as_mut_ptr());
    /* Count totalticks and totalevents */
    totalevents = 0i32 as ullong;
    totalticks = totalevents;
    i = 0i32;
    while i < 8i32 {
      if cur_usage[i as usize] != 0i32 as libc::c_ulonglong {
        totalticks = (totalticks as libc::c_ulonglong).wrapping_add(
          cur_duration[i as usize].wrapping_sub((*ptr_to_globals).last_duration[i as usize]),
        ) as ullong as ullong;
        totalevents = (totalevents as libc::c_ulonglong).wrapping_add(
          cur_usage[i as usize].wrapping_sub((*ptr_to_globals).last_usage[i as usize]),
        ) as ullong as ullong
      }
      i += 1
    }
    /* Home; clear screen */
    printf(b"\x1b[H\x1b[J\x00" as *const u8 as *const libc::c_char);
    /* Clear C-state lines */
    memset(
      &mut cstate_lines as *mut [[libc::c_char; 64]; 10] as *mut libc::c_void,
      0i32,
      ::std::mem::size_of::<[[libc::c_char; 64]; 10]>() as libc::c_ulong,
    );
    if totalevents == 0i32 as libc::c_ulonglong && (*ptr_to_globals).maxcstate <= 1i32 {
      /* This should not happen */
      strcpy(
        cstate_lines[0].as_mut_ptr(),
        b"C-state information is not available\n\x00" as *const u8 as *const libc::c_char,
      );
    } else {
      let mut percentage: libc::c_double = 0.;
      let mut newticks: libc::c_uint = 0;
      newticks = ((*ptr_to_globals)
        .total_cpus
        .wrapping_mul(10i32 as libc::c_uint)
        .wrapping_mul(3579545i32 as libc::c_uint) as libc::c_ulonglong)
        .wrapping_sub(totalticks) as libc::c_uint;
      /* Handle rounding errors: do not display negative values */
      if (newticks as libc::c_int) < 0i32 {
        newticks = 0i32 as libc::c_uint
      }
      sprintf(
        cstate_lines[0].as_mut_ptr(),
        b"Cn\t\t  Avg residency\n\x00" as *const u8 as *const libc::c_char,
      );
      percentage = newticks as libc::c_double * 100.0f64
        / (*ptr_to_globals)
          .total_cpus
          .wrapping_mul(10i32 as libc::c_uint)
          .wrapping_mul(3579545i32 as libc::c_uint) as libc::c_double;
      sprintf(
        cstate_lines[1].as_mut_ptr(),
        b"C0 (cpu running)        (%4.1f%%)\n\x00" as *const u8 as *const libc::c_char,
        percentage,
      );
      /* Compute values for individual C-states */
      i = 0i32;
      while i < 8i32 {
        if cur_usage[i as usize] != 0i32 as libc::c_ulonglong {
          let mut slept: libc::c_double = 0.;
          slept = cur_duration[i as usize].wrapping_sub((*ptr_to_globals).last_duration[i as usize])
            as libc::c_double
            / (cur_usage[i as usize].wrapping_sub((*ptr_to_globals).last_usage[i as usize])
              as libc::c_double
              + 0.1f64)
            / 3579.545f64;
          percentage = cur_duration[i as usize]
            .wrapping_sub((*ptr_to_globals).last_duration[i as usize])
            .wrapping_mul(100i32 as libc::c_ulonglong)
            .wrapping_div(
              (*ptr_to_globals)
                .total_cpus
                .wrapping_mul(10i32 as libc::c_uint)
                .wrapping_mul(3579545i32 as libc::c_uint) as libc::c_ulonglong,
            ) as libc::c_double;
          sprintf(
            cstate_lines[(i + 2i32) as usize].as_mut_ptr(),
            b"C%u\t\t%5.1fms (%4.1f%%)\n\x00" as *const u8 as *const libc::c_char,
            i + 1i32,
            slept,
            percentage,
          );
          //if (maxsleep < slept)
          //	maxsleep = slept;
        }
        i += 1
      }
    }
    i = 0i32;
    while i < 8i32 + 2i32 {
      if cstate_lines[i as usize][0] != 0 {
        fputs_unlocked(cstate_lines[i as usize].as_mut_ptr(), stdout);
      }
      i += 1
    }
    i = process_timer_stats();
    if totalevents == 0i32 as libc::c_ulonglong {
      /* No C-state info available, use timerstats */
      totalevents = (i as libc::c_uint)
        .wrapping_mul((*ptr_to_globals).total_cpus)
        .wrapping_add((*ptr_to_globals).total_interrupt as libc::c_uint)
        as ullong;
      if i < 0i32 {
        totalevents = (totalevents as libc::c_ulonglong)
          .wrapping_add(((*ptr_to_globals).interrupt_0 - i) as libc::c_ulonglong)
          as ullong as ullong
      }
    }
    /* Upstream powertop prints wakeups per sec per CPU,
     * we print just raw wakeup counts.
     */
    //TODO: show real seconds (think about manual refresh)
    printf(
      b"\nWakeups-from-idle in %u seconds: %llu\n\x00" as *const u8 as *const libc::c_char,
      10i32,
      totalevents,
    );
    update_lines_cumulative_count();
    sort_lines();
    show_timerstats();
    fflush(stdout);
    /* Clear the stats */
    memset(
      cur_duration.as_mut_ptr() as *mut libc::c_void,
      0i32,
      ::std::mem::size_of::<[ullong; 8]>() as libc::c_ulong,
    );
    memset(
      cur_usage.as_mut_ptr() as *mut libc::c_void,
      0i32,
      ::std::mem::size_of::<[ullong; 8]>() as libc::c_ulong,
    );
    /* Get new values */
    read_cstate_counts(cur_usage.as_mut_ptr(), cur_duration.as_mut_ptr());
    /* Save them */
    memcpy(
      (*ptr_to_globals).last_usage.as_mut_ptr() as *mut libc::c_void,
      cur_usage.as_mut_ptr() as *const libc::c_void,
      ::std::mem::size_of::<[ullong; 8]>() as libc::c_ulong,
    );
    memcpy(
      (*ptr_to_globals).last_duration.as_mut_ptr() as *mut libc::c_void,
      cur_duration.as_mut_ptr() as *const libc::c_void,
      ::std::mem::size_of::<[ullong; 8]>() as libc::c_ulong,
    );
  }
  bb_putchar('\n' as i32);
  reset_term();
  return 0i32;
}
