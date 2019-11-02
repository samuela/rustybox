use libc;
extern "C" {
  pub type __dirstream;
  #[no_mangle]
  fn isatty(__fd: libc::c_int) -> libc::c_int;
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
  fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
  #[no_mangle]
  fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn xmalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xrealloc_vector_helper(
    vector: *mut libc::c_void,
    sizeof_and_shift: libc::c_uint,
    idx: libc::c_int,
  ) -> *mut libc::c_void;
  #[no_mangle]
  fn safe_strncpy(
    dst: *mut libc::c_char,
    src: *const libc::c_char,
    size: size_t,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn smart_ulltoa4(
    ul: libc::c_ulonglong,
    buf: *mut libc::c_char,
    scale: *const libc::c_char,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn get_cached_username(uid: uid_t) -> *const libc::c_char;
  #[no_mangle]
  fn get_cached_groupname(gid: gid_t) -> *const libc::c_char;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;
  #[no_mangle]
  fn llist_pop(elm: *mut *mut llist_t) -> *mut libc::c_void;
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn get_terminal_width(fd: libc::c_int) -> libc::c_int;
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
  static mut bb_common_bufsiz1: [libc::c_char; 0];
  #[no_mangle]
  fn sysinfo(__info: *mut sysinfo) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type DIR = __dirstream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct llist_t {
  pub link: *mut llist_t,
  pub data: *mut libc::c_char,
}
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
  pub shift_pages_to_bytes: uint8_t,
  pub shift_pages_to_kb: uint8_t,
  pub argv_len: uint16_t,
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
pub type C2RustUnnamed = libc::c_uint;
pub const PSSCAN_TASKS: C2RustUnnamed = 4194304;
pub const PSSCAN_RUIDGID: C2RustUnnamed = 2097152;
pub const PSSCAN_NICE: C2RustUnnamed = 1048576;
pub const PSSCAN_CPU: C2RustUnnamed = 524288;
pub const PSSCAN_START_TIME: C2RustUnnamed = 262144;
pub const PSSCAN_CONTEXT: C2RustUnnamed = 0;
pub const PSSCAN_ARGVN: C2RustUnnamed = 65536;
pub const PSSCAN_SMAPS: C2RustUnnamed = 32768;
pub const PSSCAN_TTY: C2RustUnnamed = 16384;
pub const PSSCAN_UTIME: C2RustUnnamed = 8192;
pub const PSSCAN_STIME: C2RustUnnamed = 4096;
pub const PSSCAN_RSS: C2RustUnnamed = 2048;
pub const PSSCAN_VSZ: C2RustUnnamed = 1024;
pub const PSSCAN_STATE: C2RustUnnamed = 512;
pub const PSSCAN_EXE: C2RustUnnamed = 256;
pub const PSSCAN_ARGV0: C2RustUnnamed = 128;
pub const PSSCAN_COMM: C2RustUnnamed = 32;
pub const PSSCAN_UIDGID: C2RustUnnamed = 16;
pub const PSSCAN_SID: C2RustUnnamed = 8;
pub const PSSCAN_PGID: C2RustUnnamed = 4;
pub const PSSCAN_PPID: C2RustUnnamed = 2;
pub const PSSCAN_PID: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub out: *mut ps_out_t,
  pub out_cnt: libc::c_int,
  pub print_header: libc::c_int,
  pub need_flags: libc::c_int,
  pub buffer: *mut libc::c_char,
  pub terminal_width: libc::c_uint,
  pub seconds_since_boot: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ps_out_t {
  pub width: uint16_t,
  pub name6: [libc::c_char; 6],
  pub header: *const libc::c_char,
  pub f: Option<
    unsafe extern "C" fn(_: *mut libc::c_char, _: libc::c_int, _: *const procps_status_t) -> (),
  >,
  pub ps_flags: libc::c_int,
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const MAX_WIDTH: C2RustUnnamed_0 = 2048;
pub const OPT_T: C2RustUnnamed_1 = 256;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const OPT_l: C2RustUnnamed_1 = 128;
pub const OPT_f: C2RustUnnamed_1 = 64;
pub const OPT_e: C2RustUnnamed_1 = 32;
pub const OPT_d: C2RustUnnamed_1 = 16;
pub const OPT_A: C2RustUnnamed_1 = 8;
pub const OPT_a: C2RustUnnamed_1 = 4;
pub const OPT_o: C2RustUnnamed_1 = 2;
pub const OPT_Z: C2RustUnnamed_1 = 1;
unsafe extern "C" fn get_uptime() -> libc::c_ulong {
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
  if sysinfo(&mut info) < 0i32 {
    return 0i32 as libc::c_ulong;
  }
  return info.uptime as libc::c_ulong;
}
/* non-ancient Linux standardized on 100 for "times" freq */
/* Print value to buf, max size+1 chars (including trailing '\0') */
unsafe extern "C" fn func_user(
  mut buf: *mut libc::c_char,
  mut size: libc::c_int,
  mut ps: *const procps_status_t,
) {
  safe_strncpy(buf, get_cached_username((*ps).uid), (size + 1i32) as size_t);
}
unsafe extern "C" fn func_group(
  mut buf: *mut libc::c_char,
  mut size: libc::c_int,
  mut ps: *const procps_status_t,
) {
  safe_strncpy(
    buf,
    get_cached_groupname((*ps).gid),
    (size + 1i32) as size_t,
  );
}
unsafe extern "C" fn func_comm(
  mut buf: *mut libc::c_char,
  mut size: libc::c_int,
  mut ps: *const procps_status_t,
) {
  safe_strncpy(buf, (*ps).comm.as_ptr(), (size + 1i32) as size_t);
}
unsafe extern "C" fn func_state(
  mut buf: *mut libc::c_char,
  mut size: libc::c_int,
  mut ps: *const procps_status_t,
) {
  safe_strncpy(buf, (*ps).state.as_ptr(), (size + 1i32) as size_t);
}
unsafe extern "C" fn func_args(
  mut buf: *mut libc::c_char,
  mut size: libc::c_int,
  mut ps: *const procps_status_t,
) {
  read_cmdline(buf, size + 1i32, (*ps).pid, (*ps).comm.as_ptr());
}
unsafe extern "C" fn func_pid(
  mut buf: *mut libc::c_char,
  mut size: libc::c_int,
  mut ps: *const procps_status_t,
) {
  sprintf(
    buf,
    b"%*u\x00" as *const u8 as *const libc::c_char,
    size,
    (*ps).pid,
  );
}
unsafe extern "C" fn func_ppid(
  mut buf: *mut libc::c_char,
  mut size: libc::c_int,
  mut ps: *const procps_status_t,
) {
  sprintf(
    buf,
    b"%*u\x00" as *const u8 as *const libc::c_char,
    size,
    (*ps).ppid,
  );
}
unsafe extern "C" fn func_pgid(
  mut buf: *mut libc::c_char,
  mut size: libc::c_int,
  mut ps: *const procps_status_t,
) {
  sprintf(
    buf,
    b"%*u\x00" as *const u8 as *const libc::c_char,
    size,
    (*ps).pgid,
  );
}
unsafe extern "C" fn func_sid(
  mut buf: *mut libc::c_char,
  mut size: libc::c_int,
  mut ps: *const procps_status_t,
) {
  sprintf(
    buf,
    b"%*u\x00" as *const u8 as *const libc::c_char,
    size,
    (*ps).sid,
  );
}
unsafe extern "C" fn put_lu(
  mut buf: *mut libc::c_char,
  mut size: libc::c_int,
  mut u: libc::c_ulong,
) {
  let mut buf4: [libc::c_char; 5] = [0; 5];
  /* see http://en.wikipedia.org/wiki/Tera */
  *smart_ulltoa4(
    u as libc::c_ulonglong,
    buf4.as_mut_ptr(),
    b" mgtpezy\x00" as *const u8 as *const libc::c_char,
  )
  .offset(0) = '\u{0}' as i32 as libc::c_char;
  sprintf(
    buf,
    b"%.*s\x00" as *const u8 as *const libc::c_char,
    size,
    buf4.as_mut_ptr(),
  );
}
unsafe extern "C" fn func_vsz(
  mut buf: *mut libc::c_char,
  mut size: libc::c_int,
  mut ps: *const procps_status_t,
) {
  put_lu(buf, size, (*ps).vsz);
}
unsafe extern "C" fn func_rss(
  mut buf: *mut libc::c_char,
  mut size: libc::c_int,
  mut ps: *const procps_status_t,
) {
  put_lu(buf, size, (*ps).rss);
}
unsafe extern "C" fn func_tty(
  mut buf: *mut libc::c_char,
  mut size: libc::c_int,
  mut ps: *const procps_status_t,
) {
  *buf.offset(0) = '?' as i32 as libc::c_char;
  *buf.offset(1) = '\u{0}' as i32 as libc::c_char;
  if (*ps).tty_major != 0 {
    /* tty field of "0" means "no tty" */
    snprintf(
      buf,
      (size + 1i32) as libc::c_ulong,
      b"%u,%u\x00" as *const u8 as *const libc::c_char,
      (*ps).tty_major,
      (*ps).tty_minor,
    );
  };
}
unsafe extern "C" fn func_rgroup(
  mut buf: *mut libc::c_char,
  mut size: libc::c_int,
  mut ps: *const procps_status_t,
) {
  safe_strncpy(
    buf,
    get_cached_groupname((*ps).rgid),
    (size + 1i32) as size_t,
  );
}
unsafe extern "C" fn func_ruser(
  mut buf: *mut libc::c_char,
  mut size: libc::c_int,
  mut ps: *const procps_status_t,
) {
  safe_strncpy(
    buf,
    get_cached_username((*ps).ruid),
    (size + 1i32) as size_t,
  );
}
unsafe extern "C" fn func_nice(
  mut buf: *mut libc::c_char,
  mut size: libc::c_int,
  mut ps: *const procps_status_t,
) {
  sprintf(
    buf,
    b"%*d\x00" as *const u8 as *const libc::c_char,
    size,
    (*ps).niceness,
  );
}
unsafe extern "C" fn format_time(
  mut buf: *mut libc::c_char,
  mut size: libc::c_int,
  mut tt: libc::c_ulong,
) {
  let mut ff: libc::c_uint = 0;
  /* Used to show "14453:50" if tt is large. Ugly.
   * procps-ng 3.3.10 uses "[[dd-]hh:]mm:ss" format.
   * TODO: switch to that?
   */
  /* Formatting for 5-char TIME column.
   * NB: "size" is not always 5: ELAPSED is wider (7),
   * not taking advantage of that (yet?).
   */
  ff = tt.wrapping_rem(60i32 as libc::c_ulong) as libc::c_uint;
  tt = tt.wrapping_div(60i32 as libc::c_ulong);
  if tt < 60i32 as libc::c_ulong {
    snprintf(
      buf,
      (size + 1i32) as libc::c_ulong,
      b"%2u:%02u\x00" as *const u8 as *const libc::c_char,
      tt as libc::c_uint,
      ff,
    );
    return;
  }
  ff = tt.wrapping_rem(60i32 as libc::c_ulong) as libc::c_uint;
  tt = tt.wrapping_div(60i32 as libc::c_ulong);
  if tt < 24i32 as libc::c_ulong {
    snprintf(
      buf,
      (size + 1i32) as libc::c_ulong,
      b"%2uh%02u\x00" as *const u8 as *const libc::c_char,
      tt as libc::c_uint,
      ff,
    );
    return;
  }
  ff = tt.wrapping_rem(24i32 as libc::c_ulong) as libc::c_uint;
  tt = tt.wrapping_div(24i32 as libc::c_ulong);
  if tt < 100i32 as libc::c_ulong {
    snprintf(
      buf,
      (size + 1i32) as libc::c_ulong,
      b"%2ud%02u\x00" as *const u8 as *const libc::c_char,
      tt as libc::c_uint,
      ff,
    );
    return;
  }
  snprintf(
    buf,
    (size + 1i32) as libc::c_ulong,
    b"%4lud\x00" as *const u8 as *const libc::c_char,
    tt,
  );
}
unsafe extern "C" fn func_etime(
  mut buf: *mut libc::c_char,
  mut size: libc::c_int,
  mut ps: *const procps_status_t,
) {
  /* elapsed time [[dd-]hh:]mm:ss; here only mm:ss */
  let mut mm: libc::c_ulong = 0;
  mm = (*ps)
    .start_time
    .wrapping_div(100i32 as libc::c_uint as libc::c_ulong);
  mm = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .seconds_since_boot
    .wrapping_sub(mm);
  format_time(buf, size, mm);
}
unsafe extern "C" fn func_time(
  mut buf: *mut libc::c_char,
  mut size: libc::c_int,
  mut ps: *const procps_status_t,
) {
  /* cumulative time [[dd-]hh:]mm:ss; here only mm:ss */
  let mut mm: libc::c_ulong = 0;
  mm = (*ps)
    .utime
    .wrapping_add((*ps).stime)
    .wrapping_div(100i32 as libc::c_uint as libc::c_ulong);
  format_time(buf, size, mm);
}
/*
static void func_pcpu(char *buf, int size, const procps_status_t *ps)
{
}
*/
// Initialized in run_static_initializers
static mut out_spec: [ps_out_t; 17] = [ps_out_t {
  width: 0,
  name6: [0; 6],
  header: 0 as *const libc::c_char,
  f: None,
  ps_flags: 0,
}; 17];
unsafe extern "C" fn new_out_t() -> *mut ps_out_t {
  let ref mut fresh0 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).out;
  *fresh0 = xrealloc_vector_helper(
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).out as *mut libc::c_void,
    ((::std::mem::size_of::<ps_out_t>() as libc::c_ulong) << 8i32)
      .wrapping_add(2i32 as libc::c_ulong) as libc::c_uint,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).out_cnt,
  ) as *mut ps_out_t;
  let ref mut fresh1 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).out_cnt;
  let fresh2 = *fresh1;
  *fresh1 = *fresh1 + 1;
  return &mut *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .out
    .offset(fresh2 as isize) as *mut ps_out_t;
}
unsafe extern "C" fn find_out_spec(mut name: *const libc::c_char) -> *const ps_out_t {
  let mut i: libc::c_uint = 0;
  let mut buf: [libc::c_char; 120] = [0; 120];
  let mut p: *mut libc::c_char = buf.as_mut_ptr();
  i = 0i32 as libc::c_uint;
  while i
    < (::std::mem::size_of::<[ps_out_t; 17]>() as libc::c_ulong)
      .wrapping_div(::std::mem::size_of::<ps_out_t>() as libc::c_ulong) as libc::c_uint
  {
    if strncmp(
      name,
      out_spec[i as usize].name6.as_ptr(),
      6i32 as libc::c_ulong,
    ) == 0i32
    {
      return &*out_spec.as_ptr().offset(i as isize) as *const ps_out_t;
    }
    p = p.offset(sprintf(
      p,
      b"%.6s,\x00" as *const u8 as *const libc::c_char,
      out_spec[i as usize].name6.as_ptr(),
    ) as isize);
    i = i.wrapping_add(1)
  }
  *p.offset(-1i32 as isize) = '\u{0}' as i32 as libc::c_char;
  bb_error_msg_and_die(
    b"bad -o argument \'%s\', supported arguments: %s\x00" as *const u8 as *const libc::c_char,
    name,
    buf.as_mut_ptr(),
  );
}
unsafe extern "C" fn parse_o(mut opt: *mut libc::c_char) {
  let mut new: *mut ps_out_t = 0 as *mut ps_out_t;
  // POSIX: "-o is blank- or comma-separated list" (FIXME)
  let mut comma: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut equal: *mut libc::c_char = 0 as *mut libc::c_char;
  loop {
    comma = strchr(opt, ',' as i32);
    equal = strchr(opt, '=' as i32);
    if !(!comma.is_null() && (equal.is_null() || equal > comma)) {
      break;
    }
    *comma = '\u{0}' as i32 as libc::c_char;
    *new_out_t() = *find_out_spec(opt);
    *comma = ',' as i32 as libc::c_char;
    opt = comma.offset(1)
  }
  // opt points to last spec in comma separated list.
  // This one can have =HEADER part.
  new = new_out_t();
  if !equal.is_null() {
    *equal = '\u{0}' as i32 as libc::c_char
  }
  *new = *find_out_spec(opt);
  if !equal.is_null() {
    *equal = '=' as i32 as libc::c_char;
    (*new).header = equal.offset(1);
    // POSIX: the field widths shall be ... at least as wide as
    // the header text (default or overridden value).
    // If the header text is null, such as -o user=,
    // the field width shall be at least as wide as the
    // default header text
    if *(*new).header.offset(0) != 0 {
      (*new).width = strlen((*new).header) as uint16_t; /* "FIELD " */
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).print_header = 1i32
    }
  } else {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).print_header = 1i32
  };
}
unsafe extern "C" fn alloc_line_buffer() {
  let mut i: libc::c_int = 0;
  let mut width: libc::c_int = 0i32;
  i = 0i32;
  while i < (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).out_cnt {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).need_flags |=
      (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .out
        .offset(i as isize))
      .ps_flags;
    if *(*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .out
      .offset(i as isize))
    .header
    .offset(0)
      != 0
    {
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).print_header = 1i32
    }
    width += (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .out
      .offset(i as isize))
    .width as libc::c_int
      + 1i32;
    if (width as libc::c_uint)
      .wrapping_sub((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).terminal_width)
      as libc::c_int
      > 0i32
    {
      /* The rest does not fit on the screen */
      //out[i].width -= (width - terminal_width - 1);
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).out_cnt = i + 1i32;
      break;
    } else {
      i += 1
    }
  }
  let ref mut fresh3 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).buffer;
  *fresh3 = xmalloc((width + 1i32) as size_t) as *mut libc::c_char;
  /* for trailing \0 */
}
unsafe extern "C" fn format_header() {
  let mut i: libc::c_int = 0;
  let mut op: *mut ps_out_t = 0 as *mut ps_out_t;
  let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).print_header == 0 {
    return;
  }
  p = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).buffer;
  i = 0i32;
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).out_cnt != 0 {
    loop {
      op = &mut *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .out
        .offset(i as isize) as *mut ps_out_t;
      i += 1;
      if i == (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).out_cnt {
        break;
      }
      p = p.offset(sprintf(
        p,
        b"%-*s \x00" as *const u8 as *const libc::c_char,
        (*op).width as libc::c_int,
        (*op).header,
      ) as isize)
    }
    strcpy(p, (*op).header);
  }
  printf(
    b"%.*s\n\x00" as *const u8 as *const libc::c_char,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).terminal_width,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).buffer,
  );
}
unsafe extern "C" fn format_process(mut ps: *const procps_status_t) {
  let mut i: libc::c_int = 0;
  let mut len: libc::c_int = 0;
  let mut p: *mut libc::c_char = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).buffer;
  i = 0i32;
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).out_cnt != 0 {
    loop {
      (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .out
        .offset(i as isize))
      .f
      .expect("non-null function pointer")(
        p,
        (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
          .out
          .offset(i as isize))
        .width as libc::c_int,
        ps,
      );
      /* " ", not "", to ensure separation of fields */
      // POSIX: Any field need not be meaningful in all
      // implementations. In such a case a hyphen ( '-' )
      // should be output in place of the field value.
      if *p.offset(0) == 0 {
        *p.offset(0) = '-' as i32 as libc::c_char;
        *p.offset(1) = '\u{0}' as i32 as libc::c_char
      }
      len = strlen(p) as libc::c_int;
      p = p.offset(len as isize);
      len = (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .out
        .offset(i as isize))
      .width as libc::c_int
        - len
        + 1i32;
      i += 1;
      if i == (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).out_cnt {
        break;
      }
      p = p.offset(sprintf(
        p,
        b"%*s\x00" as *const u8 as *const libc::c_char,
        len,
        b" \x00" as *const u8 as *const libc::c_char,
      ) as isize)
    }
  }
  printf(
    b"%.*s\n\x00" as *const u8 as *const libc::c_char,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).terminal_width,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).buffer,
  );
}
#[no_mangle]
pub unsafe extern "C" fn ps_main(
  mut argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut p: *mut procps_status_t = 0 as *mut procps_status_t;
  let mut opt_o: *mut llist_t = 0 as *mut llist_t;
  let mut default_o: [libc::c_char; 19] = [0; 19];
  let mut opt: libc::c_int = 0;
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).seconds_since_boot = get_uptime();
  // POSIX:
  // -a  Write information for all processes associated with terminals
  //     Implementations may omit session leaders from this list
  // -A  Write information for all processes
  // -d  Write information for all processes, except session leaders
  // -e  Write information for all processes (equivalent to -A)
  // -f  Generate a full listing
  // -l  Generate a long listing
  // -o col1,col2,col3=header
  //     Select which columns to display
  /* We allow (and ignore) most of the above. FIXME.
   * -T is picked for threads (POSIX hasn't standardized it).
   * procps v3.2.7 supports -T and shows tids as SPID column,
   * it also supports -L where it shows tids as LWP column.
   */
  opt = getopt32(
    argv,
    b"Zo:*aAdeflT\x00" as *const u8 as *const libc::c_char,
    &mut opt_o as *mut *mut llist_t,
  ) as libc::c_int;
  if !opt_o.is_null() {
    loop {
      parse_o(llist_pop(&mut opt_o) as *mut libc::c_char);
      if opt_o.is_null() {
        break;
      }
    }
  } else {
    /* Below: parse_o() needs char*, NOT const char*,
     * can't pass it constant string. Need to make a copy first.
     */
    strcpy(
      default_o.as_mut_ptr(),
      b"pid,user,time,args\x00" as *const u8 as *const libc::c_char,
    );
    parse_o(default_o.as_mut_ptr());
  }
  if opt & OPT_T as libc::c_int != 0 {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).need_flags |= PSSCAN_TASKS as libc::c_int
  }
  /* Was INT_MAX, but some libc's go belly up with printf("%.*s")
   * and such large widths */
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).terminal_width =
    MAX_WIDTH as libc::c_int as libc::c_uint;
  if isatty(1i32) != 0 {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).terminal_width =
      get_terminal_width(0i32) as libc::c_uint;
    let ref mut fresh4 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).terminal_width;
    *fresh4 = (*fresh4).wrapping_sub(1);
    if *fresh4 > MAX_WIDTH as libc::c_int as libc::c_uint {
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).terminal_width =
        MAX_WIDTH as libc::c_int as libc::c_uint
    }
  }
  alloc_line_buffer();
  format_header();
  p = 0 as *mut procps_status_t;
  loop {
    p = procps_scan(
      p,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).need_flags,
    );
    if p.is_null() {
      break;
    }
    format_process(p);
  }
  return 0i32;
}
unsafe extern "C" fn run_static_initializers() {
  out_spec = [
    {
      let mut init = ps_out_t {
        width: 8i32 as uint16_t,
        name6: *::std::mem::transmute::<&[u8; 6], &mut [libc::c_char; 6]>(b"user\x00\x00"),
        header: b"USER\x00" as *const u8 as *const libc::c_char,
        f: Some(
          func_user
            as unsafe extern "C" fn(
              _: *mut libc::c_char,
              _: libc::c_int,
              _: *const procps_status_t,
            ) -> (),
        ),
        ps_flags: PSSCAN_UIDGID as libc::c_int,
      };
      init
    },
    {
      let mut init = ps_out_t {
        width: 8i32 as uint16_t,
        name6: *::std::mem::transmute::<&[u8; 6], &mut [libc::c_char; 6]>(b"group\x00"),
        header: b"GROUP\x00" as *const u8 as *const libc::c_char,
        f: Some(
          func_group
            as unsafe extern "C" fn(
              _: *mut libc::c_char,
              _: libc::c_int,
              _: *const procps_status_t,
            ) -> (),
        ),
        ps_flags: PSSCAN_UIDGID as libc::c_int,
      };
      init
    },
    {
      let mut init = ps_out_t {
        width: 16i32 as uint16_t,
        name6: *::std::mem::transmute::<&[u8; 6], &mut [libc::c_char; 6]>(b"comm\x00\x00"),
        header: b"COMMAND\x00" as *const u8 as *const libc::c_char,
        f: Some(
          func_comm
            as unsafe extern "C" fn(
              _: *mut libc::c_char,
              _: libc::c_int,
              _: *const procps_status_t,
            ) -> (),
        ),
        ps_flags: PSSCAN_COMM as libc::c_int,
      };
      init
    },
    {
      let mut init = ps_out_t {
        width: MAX_WIDTH as libc::c_int as uint16_t,
        name6: *::std::mem::transmute::<&[u8; 6], &mut [libc::c_char; 6]>(b"args\x00\x00"),
        header: b"COMMAND\x00" as *const u8 as *const libc::c_char,
        f: Some(
          func_args
            as unsafe extern "C" fn(
              _: *mut libc::c_char,
              _: libc::c_int,
              _: *const procps_status_t,
            ) -> (),
        ),
        ps_flags: PSSCAN_COMM as libc::c_int,
      };
      init
    },
    {
      let mut init = ps_out_t {
        width: 5i32 as uint16_t,
        name6: *::std::mem::transmute::<&[u8; 6], &mut [libc::c_char; 6]>(b"pid\x00\x00\x00"),
        header: b"PID\x00" as *const u8 as *const libc::c_char,
        f: Some(
          func_pid
            as unsafe extern "C" fn(
              _: *mut libc::c_char,
              _: libc::c_int,
              _: *const procps_status_t,
            ) -> (),
        ),
        ps_flags: PSSCAN_PID as libc::c_int,
      };
      init
    },
    {
      let mut init = ps_out_t {
        width: 5i32 as uint16_t,
        name6: *::std::mem::transmute::<&[u8; 6], &mut [libc::c_char; 6]>(b"ppid\x00\x00"),
        header: b"PPID\x00" as *const u8 as *const libc::c_char,
        f: Some(
          func_ppid
            as unsafe extern "C" fn(
              _: *mut libc::c_char,
              _: libc::c_int,
              _: *const procps_status_t,
            ) -> (),
        ),
        ps_flags: PSSCAN_PPID as libc::c_int,
      };
      init
    },
    {
      let mut init = ps_out_t {
        width: 5i32 as uint16_t,
        name6: *::std::mem::transmute::<&[u8; 6], &mut [libc::c_char; 6]>(b"pgid\x00\x00"),
        header: b"PGID\x00" as *const u8 as *const libc::c_char,
        f: Some(
          func_pgid
            as unsafe extern "C" fn(
              _: *mut libc::c_char,
              _: libc::c_int,
              _: *const procps_status_t,
            ) -> (),
        ),
        ps_flags: PSSCAN_PGID as libc::c_int,
      };
      init
    },
    {
      let mut init = ps_out_t {
        width: (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
          .wrapping_sub(1i32 as libc::c_ulong) as uint16_t,
        name6: *::std::mem::transmute::<&[u8; 6], &mut [libc::c_char; 6]>(b"etime\x00"),
        header: b"ELAPSED\x00" as *const u8 as *const libc::c_char,
        f: Some(
          func_etime
            as unsafe extern "C" fn(
              _: *mut libc::c_char,
              _: libc::c_int,
              _: *const procps_status_t,
            ) -> (),
        ),
        ps_flags: PSSCAN_START_TIME as libc::c_int,
      };
      init
    },
    {
      let mut init = ps_out_t {
        width: 5i32 as uint16_t,
        name6: *::std::mem::transmute::<&[u8; 6], &mut [libc::c_char; 6]>(b"nice\x00\x00"),
        header: b"NI\x00" as *const u8 as *const libc::c_char,
        f: Some(
          func_nice
            as unsafe extern "C" fn(
              _: *mut libc::c_char,
              _: libc::c_int,
              _: *const procps_status_t,
            ) -> (),
        ),
        ps_flags: PSSCAN_NICE as libc::c_int,
      };
      init
    },
    {
      let mut init = ps_out_t {
        width: 8i32 as uint16_t,
        name6: *::std::mem::transmute::<&[u8; 6], &mut [libc::c_char; 6]>(b"rgroup"),
        header: b"RGROUP\x00" as *const u8 as *const libc::c_char,
        f: Some(
          func_rgroup
            as unsafe extern "C" fn(
              _: *mut libc::c_char,
              _: libc::c_int,
              _: *const procps_status_t,
            ) -> (),
        ),
        ps_flags: PSSCAN_RUIDGID as libc::c_int,
      };
      init
    },
    {
      let mut init = ps_out_t {
        width: 8i32 as uint16_t,
        name6: *::std::mem::transmute::<&[u8; 6], &mut [libc::c_char; 6]>(b"ruser\x00"),
        header: b"RUSER\x00" as *const u8 as *const libc::c_char,
        f: Some(
          func_ruser
            as unsafe extern "C" fn(
              _: *mut libc::c_char,
              _: libc::c_int,
              _: *const procps_status_t,
            ) -> (),
        ),
        ps_flags: PSSCAN_RUIDGID as libc::c_int,
      };
      init
    },
    {
      let mut init = ps_out_t {
        width: 5i32 as uint16_t,
        name6: *::std::mem::transmute::<&[u8; 6], &mut [libc::c_char; 6]>(b"time\x00\x00"),
        header: b"TIME\x00" as *const u8 as *const libc::c_char,
        f: Some(
          func_time
            as unsafe extern "C" fn(
              _: *mut libc::c_char,
              _: libc::c_int,
              _: *const procps_status_t,
            ) -> (),
        ),
        ps_flags: PSSCAN_STIME as libc::c_int | PSSCAN_UTIME as libc::c_int,
      };
      init
    },
    {
      let mut init = ps_out_t {
        width: 6i32 as uint16_t,
        name6: *::std::mem::transmute::<&[u8; 6], &mut [libc::c_char; 6]>(b"tty\x00\x00\x00"),
        header: b"TT\x00" as *const u8 as *const libc::c_char,
        f: Some(
          func_tty
            as unsafe extern "C" fn(
              _: *mut libc::c_char,
              _: libc::c_int,
              _: *const procps_status_t,
            ) -> (),
        ),
        ps_flags: PSSCAN_TTY as libc::c_int,
      };
      init
    },
    {
      let mut init = ps_out_t {
        width: 4i32 as uint16_t,
        name6: *::std::mem::transmute::<&[u8; 6], &mut [libc::c_char; 6]>(b"vsz\x00\x00\x00"),
        header: b"VSZ\x00" as *const u8 as *const libc::c_char,
        f: Some(
          func_vsz
            as unsafe extern "C" fn(
              _: *mut libc::c_char,
              _: libc::c_int,
              _: *const procps_status_t,
            ) -> (),
        ),
        ps_flags: PSSCAN_VSZ as libc::c_int,
      };
      init
    },
    {
      let mut init = ps_out_t {
        width: 5i32 as uint16_t,
        name6: *::std::mem::transmute::<&[u8; 6], &mut [libc::c_char; 6]>(b"sid\x00\x00\x00"),
        header: b"SID\x00" as *const u8 as *const libc::c_char,
        f: Some(
          func_sid
            as unsafe extern "C" fn(
              _: *mut libc::c_char,
              _: libc::c_int,
              _: *const procps_status_t,
            ) -> (),
        ),
        ps_flags: PSSCAN_SID as libc::c_int,
      };
      init
    },
    {
      let mut init = ps_out_t {
        width: 4i32 as uint16_t,
        name6: *::std::mem::transmute::<&[u8; 6], &mut [libc::c_char; 6]>(b"stat\x00\x00"),
        header: b"STAT\x00" as *const u8 as *const libc::c_char,
        f: Some(
          func_state
            as unsafe extern "C" fn(
              _: *mut libc::c_char,
              _: libc::c_int,
              _: *const procps_status_t,
            ) -> (),
        ),
        ps_flags: PSSCAN_STATE as libc::c_int,
      };
      init
    },
    {
      let mut init = ps_out_t {
        width: 4i32 as uint16_t,
        name6: *::std::mem::transmute::<&[u8; 6], &mut [libc::c_char; 6]>(b"rss\x00\x00\x00"),
        header: b"RSS\x00" as *const u8 as *const libc::c_char,
        f: Some(
          func_rss
            as unsafe extern "C" fn(
              _: *mut libc::c_char,
              _: libc::c_int,
              _: *const procps_status_t,
            ) -> (),
        ),
        ps_flags: PSSCAN_RSS as libc::c_int,
      };
      init
    },
  ]
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
/* !ENABLE_DESKTOP */
/* !ENABLE_DESKTOP */
