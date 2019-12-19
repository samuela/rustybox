use crate::libbb::xfuncs_printf::xmalloc;
use libc;
use libc::close;
use libc::sprintf;
use libc::strchr;
use libc::strstr;
extern "C" {
  #[no_mangle]
  fn strtoul(
    __nptr: *const libc::c_char,
    __endptr: *mut *mut libc::c_char,
    __base: libc::c_int,
  ) -> libc::c_ulong;
  #[no_mangle]
  fn strtoull(
    __nptr: *const libc::c_char,
    __endptr: *mut *mut libc::c_char,
    __base: libc::c_int,
  ) -> libc::c_ulonglong;
  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  static ptr_to_globals: *mut globals;
  #[no_mangle]
  fn usleep(__useconds: useconds_t) -> libc::c_int;
  #[no_mangle]
  fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;

  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> *mut libc::c_char;
  #[no_mangle]
  fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;

  #[no_mangle]
  fn strchrnul(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;

  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn gettimeofday(__tv: *mut timeval, __tz: __timezone_ptr_t) -> libc::c_int;
  #[no_mangle]
  fn localtime(__timer: *const time_t) -> *mut tm;

  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xrealloc(old: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xchdir(path: *const libc::c_char);
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn overlapping_strcpy(dst: *mut libc::c_char, src: *const libc::c_char);
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  #[no_mangle]
  fn open_read_close(
    filename: *const libc::c_char,
    buf: *mut libc::c_void,
    maxsz: size_t,
  ) -> ssize_t;
  #[no_mangle]
  fn xwrite(fd: libc::c_int, buf: *const libc::c_void, count: size_t);
  #[no_mangle]
  fn smart_ulltoa4(
    ul: libc::c_ulonglong,
    buf: *mut libc::c_char,
    scale_0: *const libc::c_char,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn xatoi(str: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn xatoi_positive(numstr: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
}

use crate::librb::size_t;
use crate::librb::smallint;
use libc::ssize_t;
use libc::time_t;
use libc::timeval;
use libc::useconds_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timezone {
  pub tz_minuteswest: libc::c_int,
  pub tz_dsttime: libc::c_int,
}
pub type __timezone_ptr_t = *mut timezone;
use libc::tm;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub gen: smallint,
  pub is26: smallint,
  pub need_seconds: smallint,
  pub final_char: libc::c_char,
  pub cur_outbuf: *mut libc::c_char,
  pub delta: libc::c_int,
  pub deltanz: libc::c_uint,
  pub tv: timeval,
  pub proc_stat: proc_file,
  pub proc_loadavg: proc_file,
  pub proc_net_dev: proc_file,
  pub proc_meminfo: proc_file,
  pub proc_diskstats: proc_file,
  pub proc_sys_fs_filenr: proc_file,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct proc_file {
  pub file: *mut libc::c_char,
  pub file_sz: libc::c_int,
  pub last_gen: smallint,
}
pub type C2RustUnnamed = libc::c_uint;
pub const COMMON_BUFSIZE: C2RustUnnamed = 1024;
pub type ullong = libc::c_ulonglong;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const PROC_MAX_FILE_SIZE: C2RustUnnamed_0 = 16384;
pub const PROC_MIN_FILE_SIZE: C2RustUnnamed_0 = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct s_stat {
  pub next: *mut s_stat,
  pub collect: Option<unsafe extern "C" fn(_: *mut s_stat) -> ()>,
  pub label: *const libc::c_char,
}
pub type conv_type = libc::c_uint;
pub const conv_slash: conv_type = 1;
pub const conv_decimal: conv_type = 0;
//     user nice system idle  iowait irq  softirq (last 3 only in 2.6)
//cpu  649369 0 341297 4336769 11640 7122 1183
//cpuN 649369 0 341297 4336769 11640 7122 1183
pub type C2RustUnnamed_1 = libc::c_uint;
pub const CPU_FIELDCNT: C2RustUnnamed_1 = 7;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpu_stat {
  pub next: *mut s_stat,
  pub collect: Option<unsafe extern "C" fn(_: *mut cpu_stat) -> ()>,
  pub label: *const libc::c_char,
  pub old: [ullong; 7],
  pub bar_sz: libc::c_uint,
  pub bar: [libc::c_char; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct int_stat {
  pub next: *mut s_stat,
  pub collect: Option<unsafe extern "C" fn(_: *mut int_stat) -> ()>,
  pub label: *const libc::c_char,
  pub old: ullong,
  pub no: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ctx_stat {
  pub next: *mut s_stat,
  pub collect: Option<unsafe extern "C" fn(_: *mut ctx_stat) -> ()>,
  pub label: *const libc::c_char,
  pub old: ullong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct blk_stat {
  pub next: *mut s_stat,
  pub collect: Option<unsafe extern "C" fn(_: *mut blk_stat) -> ()>,
  pub label: *const libc::c_char,
  pub lookfor: *const libc::c_char,
  pub old: [ullong; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fork_stat {
  pub next: *mut s_stat,
  pub collect: Option<unsafe extern "C" fn(_: *mut fork_stat) -> ()>,
  pub label: *const libc::c_char,
  pub old: ullong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct if_stat {
  pub next: *mut s_stat,
  pub collect: Option<unsafe extern "C" fn(_: *mut if_stat) -> ()>,
  pub label: *const libc::c_char,
  pub old: [ullong; 4],
  pub device: *const libc::c_char,
  pub device_colon: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mem_stat {
  pub next: *mut s_stat,
  pub collect: Option<unsafe extern "C" fn(_: *mut mem_stat) -> ()>,
  pub label: *const libc::c_char,
  pub opt: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct swp_stat {
  pub next: *mut s_stat,
  pub collect: Option<unsafe extern "C" fn(_: *mut swp_stat) -> ()>,
  pub label: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_stat {
  pub next: *mut s_stat,
  pub collect: Option<unsafe extern "C" fn(_: *mut fd_stat) -> ()>,
  pub label: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct time_stat {
  pub next: *mut s_stat,
  pub collect: Option<unsafe extern "C" fn(_: *mut time_stat) -> ()>,
  pub label: *const libc::c_char,
  pub prec: libc::c_uint,
  pub scale: libc::c_uint,
}
pub type init_func = unsafe extern "C" fn(_: *const libc::c_char) -> *mut s_stat;
#[inline(always)]
unsafe extern "C" fn not_const_pp(mut p: *const libc::c_void) -> *mut libc::c_void {
  return p as *mut libc::c_void;
}
static mut proc_name: [*const libc::c_char; 6] = [
  b"stat\x00" as *const u8 as *const libc::c_char,
  b"loadavg\x00" as *const u8 as *const libc::c_char,
  b"net/dev\x00" as *const u8 as *const libc::c_char,
  b"meminfo\x00" as *const u8 as *const libc::c_char,
  b"diskstats\x00" as *const u8 as *const libc::c_char,
  b"sys/fs/file-nr\x00" as *const u8 as *const libc::c_char,
];
#[inline]
unsafe extern "C" fn reset_outbuf() {
  (*ptr_to_globals).cur_outbuf = bb_common_bufsiz1.as_mut_ptr();
}
unsafe extern "C" fn print_outbuf() {
  let mut sz: libc::c_int = (*ptr_to_globals)
    .cur_outbuf
    .wrapping_offset_from(bb_common_bufsiz1.as_mut_ptr())
    as libc::c_long as libc::c_int;
  if sz > 0 {
    xwrite(
      1i32,
      bb_common_bufsiz1.as_mut_ptr() as *const libc::c_void,
      sz as size_t,
    );
    (*ptr_to_globals).cur_outbuf = bb_common_bufsiz1.as_mut_ptr()
  };
}
unsafe extern "C" fn put(mut s: *const libc::c_char) {
  let mut p: *mut libc::c_char = (*ptr_to_globals).cur_outbuf;
  let mut sz: libc::c_int = bb_common_bufsiz1
    .as_mut_ptr()
    .offset(COMMON_BUFSIZE as libc::c_int as isize)
    .wrapping_offset_from(p) as libc::c_long as libc::c_int;
  while *s as libc::c_int != 0 && {
    sz -= 1;
    (sz) >= 0
  } {
    let fresh0 = s;
    s = s.offset(1);
    let fresh1 = p;
    p = p.offset(1);
    *fresh1 = *fresh0
  }
  (*ptr_to_globals).cur_outbuf = p;
}
unsafe extern "C" fn put_c(mut c: libc::c_char) {
  if (*ptr_to_globals).cur_outbuf
    < bb_common_bufsiz1
      .as_mut_ptr()
      .offset(COMMON_BUFSIZE as libc::c_int as isize)
  {
    let fresh2 = (*ptr_to_globals).cur_outbuf;
    (*ptr_to_globals).cur_outbuf = (*ptr_to_globals).cur_outbuf.offset(1);
    *fresh2 = c
  };
}
unsafe extern "C" fn put_question_marks(mut count: libc::c_int) {
  loop {
    let fresh3 = count;
    count = count - 1;
    if !(fresh3 != 0) {
      break;
    }
    put_c('?' as i32 as libc::c_char);
  }
}
unsafe extern "C" fn readfile_z(mut pf: *mut proc_file, mut fname: *const libc::c_char) {
  // open_read_close() will do two reads in order to be sure we are at EOF,
  // and we don't need/want that.
  let mut fd: libc::c_int = 0;
  let mut sz: libc::c_int = 0;
  let mut rdsz: libc::c_int = 0;
  let mut buf: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  sz = (*pf).file_sz;
  buf = (*pf).file;
  if buf.is_null() {
    buf = xmalloc(PROC_MIN_FILE_SIZE as libc::c_int as size_t) as *mut libc::c_char;
    sz = PROC_MIN_FILE_SIZE as libc::c_int
  }
  loop {
    fd = xopen(fname, 0);
    *buf.offset(0) = '\u{0}' as i32 as libc::c_char;
    rdsz = read(fd, buf as *mut libc::c_void, (sz - 1i32) as size_t) as libc::c_int;
    close(fd);
    if !(rdsz > 0) {
      break;
    }
    if rdsz == sz - 1i32 && sz < PROC_MAX_FILE_SIZE as libc::c_int {
      sz *= 2i32;
      buf = xrealloc(buf as *mut libc::c_void, sz as size_t) as *mut libc::c_char
    } else {
      *buf.offset(rdsz as isize) = '\u{0}' as i32 as libc::c_char;
      break;
    }
  }
  (*pf).file_sz = sz;
  (*pf).file = buf;
}
unsafe extern "C" fn get_file(mut pf: *mut proc_file) -> *const libc::c_char {
  if (*pf).last_gen as libc::c_int != (*ptr_to_globals).gen as libc::c_int {
    (*pf).last_gen = (*ptr_to_globals).gen;
    readfile_z(
      pf,
      proc_name[pf.wrapping_offset_from(&mut (*ptr_to_globals).proc_stat) as libc::c_long as usize],
    );
  }
  return (*pf).file;
}
unsafe extern "C" fn read_after_slash(mut p: *const libc::c_char) -> ullong {
  p = strchr(p, '/' as i32);
  if p.is_null() {
    return 0 as ullong;
  }
  return strtoull(p.offset(1), 0 as *mut *mut libc::c_char, 10i32);
}
// Reads decimal values from line. Values start after key, for example:
// "cpu  649369 0 341297 4336769..." - key is "cpu" here.
// Values are stored in vec[].
// posbits is a bit lit of positions we are interested in.
// for example: 00100110 - we want 1st, 2nd and 5th value.
// posbits.bit0 encodes conversion type.
unsafe extern "C" fn rdval(
  mut p: *const libc::c_char,
  mut key: *const libc::c_char,
  mut vec: *mut ullong,
  mut posbits: libc::c_long,
) -> libc::c_int {
  let mut curpos: libc::c_uint = 0;
  p = strstr(p, key);
  if p.is_null() {
    return 1i32;
  }
  p = p.offset(strlen(key) as isize);
  curpos = (1i32 << 1i32) as libc::c_uint;
  loop {
    while *p as libc::c_int == ' ' as i32 || *p as libc::c_int == '\t' as i32 {
      p = p.offset(1)
    }
    if *p as libc::c_int == '\n' as i32 || *p as libc::c_int == '\u{0}' as i32 {
      break;
    }
    if curpos as libc::c_long & posbits != 0 {
      // read this value
      let fresh4 = vec;
      vec = vec.offset(1);
      *fresh4 = if posbits & 1 == conv_decimal as libc::c_int as libc::c_long {
        strtoull(p, 0 as *mut *mut libc::c_char, 10i32)
      } else {
        read_after_slash(p)
      };
      posbits -= curpos as libc::c_long;
      if posbits <= 1 {
        return 0;
      }
    }
    while *p as libc::c_int > ' ' as i32 {
      // skip over the value
      p = p.offset(1)
    }
    curpos <<= 1i32
  }
  return 0;
}
// Parses files with lines like "... ... ... 3/148 ...."
unsafe extern "C" fn rdval_loadavg(
  mut p: *const libc::c_char,
  mut vec: *mut ullong,
  mut posbits: libc::c_long,
) -> libc::c_int {
  let mut result: libc::c_int = 0;
  result = rdval(
    p,
    b"\x00" as *const u8 as *const libc::c_char,
    vec,
    posbits | conv_slash as libc::c_int as libc::c_long,
  );
  return result;
}
// Parses /proc/diskstats
//   1  2 3   4     5     6(rd)  7      8     9     10(wr) 11     12 13     14
//   3  0 hda 51292 14441 841783 926052 25717 79650 843256 3029804 0 148459 3956933
//   3  1 hda1 0 0 0 0 <- ignore if only 4 fields
// Linux 3.0 (maybe earlier) started printing full stats for hda1 too.
// Had to add code which skips such devices.
unsafe extern "C" fn rdval_diskstats(
  mut p: *const libc::c_char,
  mut vec: *mut ullong,
) -> libc::c_int {
  let mut devname: [libc::c_char; 32] = [0; 32];
  let mut devname_len: libc::c_uint = 0 as libc::c_uint;
  let mut value_idx: libc::c_int = 0;
  *vec.offset(0) = 0 as ullong;
  *vec.offset(1) = 0 as ullong;
  let mut current_block_19: u64;
  loop {
    value_idx += 1;
    while *p as libc::c_int == ' ' as i32 || *p as libc::c_int == '\t' as i32 {
      p = p.offset(1)
    }
    if *p as libc::c_int == '\u{0}' as i32 {
      break;
    }
    if *p as libc::c_int == '\n' as i32 {
      value_idx = 0;
      p = p.offset(1)
    } else {
      if value_idx == 3i32 {
        let mut end: *mut libc::c_char = strchrnul(p, ' ' as i32);
        /* If this a hda1-like device (same prefix as last one + digit)? */
        if devname_len != 0
          && strncmp(devname.as_mut_ptr(), p, devname_len as libc::c_ulong) == 0
          && (*p.offset(devname_len as isize) as libc::c_int - '0' as i32) as libc::c_uchar
            as libc::c_int
            <= 9i32
        {
          p = end;
          current_block_19 = 4839426858821150325;
        /* skip entire line */
        } else {
          /* It is not. Remember the name for future checks */
          devname_len = end.wrapping_offset_from(p) as libc::c_long as libc::c_uint;
          if devname_len as libc::c_ulong
            > (::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
              .wrapping_sub(1i32 as libc::c_ulong)
          {
            devname_len = (::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
              .wrapping_sub(1i32 as libc::c_ulong) as libc::c_uint
          }
          strncpy(devname.as_mut_ptr(), p, devname_len as libc::c_ulong);
          /* devname[devname_len] = '\0'; - not really needed */
          p = end;
          current_block_19 = 14648156034262866959;
        }
      } else if value_idx == 6i32 {
        // TODO: *sectorsize (don't know how to find out sectorsize)
        let ref mut fresh5 = *vec.offset(0);
        *fresh5 = (*fresh5 as libc::c_ulonglong).wrapping_add(strtoull(
          p,
          0 as *mut *mut libc::c_char,
          10i32,
        )) as ullong as ullong;
        current_block_19 = 14648156034262866959;
      } else if value_idx == 10i32 {
        // TODO: *sectorsize (don't know how to find out sectorsize)
        let ref mut fresh6 = *vec.offset(1);
        *fresh6 = (*fresh6 as libc::c_ulonglong).wrapping_add(strtoull(
          p,
          0 as *mut *mut libc::c_char,
          10i32,
        )) as ullong as ullong;
        current_block_19 = 4839426858821150325;
      } else {
        current_block_19 = 14648156034262866959;
      }
      match current_block_19 {
        14648156034262866959 => {
          while *p as libc::c_uchar as libc::c_int > ' ' as i32 {
            // skip over value
            p = p.offset(1)
          }
        }
        _ => {
          while *p as libc::c_int != '\n' as i32 && *p as libc::c_int != '\u{0}' as i32 {
            p = p.offset(1)
          }
        }
      }
    }
  }
  return 0;
}
unsafe extern "C" fn scale(mut ul: ullong) {
  let mut buf: [libc::c_char; 5] = [0; 5];
  /* see http://en.wikipedia.org/wiki/Tera */
  *smart_ulltoa4(
    ul,
    buf.as_mut_ptr(),
    b" kmgtpezy\x00" as *const u8 as *const libc::c_char,
  )
  .offset(0) = '\u{0}' as i32 as libc::c_char; //sanitize
  put(buf.as_mut_ptr());
}
unsafe extern "C" fn collect_literal(mut _s: *mut s_stat) {}
unsafe extern "C" fn init_literal() -> *mut s_stat {
  let mut s: *mut s_stat = xzalloc(::std::mem::size_of::<s_stat>() as libc::c_ulong) as *mut s_stat;
  (*s).collect = Some(collect_literal as unsafe extern "C" fn(_: *mut s_stat) -> ());
  return s;
}
unsafe extern "C" fn init_cr(mut _param: *const libc::c_char) -> *mut s_stat {
  (*ptr_to_globals).final_char = '\r' as i32 as libc::c_char;
  return std::ptr::null_mut();
}
unsafe extern "C" fn collect_cpu(mut s: *mut cpu_stat) {
  let mut data: [ullong; 7] = [
    0 as ullong,
    0 as ullong,
    0 as ullong,
    0 as ullong,
    0 as ullong,
    0 as ullong,
    0 as ullong,
  ];
  let mut frac: [libc::c_uint; 7] = [
    0 as libc::c_uint,
    0 as libc::c_uint,
    0 as libc::c_uint,
    0 as libc::c_uint,
    0 as libc::c_uint,
    0 as libc::c_uint,
    0 as libc::c_uint,
  ];
  let mut all: ullong = 0 as ullong;
  let mut norm_all: libc::c_uint = 0 as libc::c_uint;
  let mut bar_sz: libc::c_uint = (*s).bar_sz;
  let mut bar: *mut libc::c_char = (*s).bar.as_mut_ptr();
  let mut i: libc::c_int = 0;
  if rdval(
    get_file(&mut (*ptr_to_globals).proc_stat),
    b"cpu \x00" as *const u8 as *const libc::c_char,
    data.as_mut_ptr(),
    (0i32
      | 1i32 << 1i32
      | 1i32 << 2i32
      | 1i32 << 3i32
      | 1i32 << 4i32
      | 1i32 << 5i32
      | 1i32 << 6i32
      | 1i32 << 7i32) as libc::c_long,
  ) != 0
  {
    put_question_marks(bar_sz as libc::c_int);
    return;
  }
  i = 0;
  while i < CPU_FIELDCNT as libc::c_int {
    let mut old: ullong = (*s).old[i as usize];
    if data[i as usize] < old {
      old = data[i as usize]
    }
    (*s).old[i as usize] = data[i as usize];
    data[i as usize] =
      (data[i as usize] as libc::c_ulonglong).wrapping_sub(old) as ullong as ullong;
    all = (all as libc::c_ulonglong).wrapping_add(data[i as usize]) as ullong as ullong;
    i += 1
  }
  if all != 0 {
    i = 0;
    while i < CPU_FIELDCNT as libc::c_int {
      let mut t: ullong = (bar_sz as libc::c_ulonglong).wrapping_mul(data[i as usize]);
      data[i as usize] = t.wrapping_div(all);
      norm_all = (norm_all as libc::c_ulonglong).wrapping_add(data[i as usize]) as libc::c_uint
        as libc::c_uint;
      frac[i as usize] = t.wrapping_rem(all) as libc::c_uint;
      i += 1
    }
    while norm_all < bar_sz {
      let mut max: libc::c_uint = frac[0];
      let mut pos: libc::c_int = 0;
      i = 1i32;
      while i < CPU_FIELDCNT as libc::c_int {
        if frac[i as usize] > max {
          max = frac[i as usize];
          pos = i
        }
        i += 1
      }
      //softirq
      frac[pos as usize] = 0 as libc::c_uint; //avoid bumping up same value twice
      data[pos as usize] = data[pos as usize].wrapping_add(1); //sys
      norm_all = norm_all.wrapping_add(1)
    } //usr
    memset(
      bar as *mut libc::c_void,
      '.' as i32,
      bar_sz as libc::c_ulong,
    ); //nice
    memset(
      bar as *mut libc::c_void,
      'S' as i32,
      data[2] as libc::c_ulong,
    ); //iowait
    bar = bar.offset(data[2] as isize); //irq
    memset(
      bar as *mut libc::c_void,
      'U' as i32,
      data[0] as libc::c_ulong,
    );
    bar = bar.offset(data[0] as isize);
    memset(
      bar as *mut libc::c_void,
      'N' as i32,
      data[1] as libc::c_ulong,
    );
    bar = bar.offset(data[1] as isize);
    memset(
      bar as *mut libc::c_void,
      'D' as i32,
      data[4] as libc::c_ulong,
    );
    bar = bar.offset(data[4] as isize);
    memset(
      bar as *mut libc::c_void,
      'I' as i32,
      data[5] as libc::c_ulong,
    );
    bar = bar.offset(data[5] as isize);
    memset(
      bar as *mut libc::c_void,
      'i' as i32,
      data[6] as libc::c_ulong,
    );
    bar = bar.offset(data[6] as isize)
  } else {
    memset(
      bar as *mut libc::c_void,
      '?' as i32,
      bar_sz as libc::c_ulong,
    );
  }
  put((*s).bar.as_mut_ptr());
}
unsafe extern "C" fn init_cpu(mut param: *const libc::c_char) -> *mut s_stat {
  let mut sz: libc::c_int = 0;
  let mut s: *mut cpu_stat = std::ptr::null_mut();
  sz = if *param.offset(0) as libc::c_int != 0 {
    strtoul(param, 0 as *mut *mut libc::c_char, 0)
  } else {
    10i32 as libc::c_ulong
  } as libc::c_int;
  if sz <= 0 {
    sz = 1i32
  }
  if sz > 1000i32 {
    sz = 1000i32
  }
  s =
    xzalloc((::std::mem::size_of::<cpu_stat>() as libc::c_ulong).wrapping_add(sz as libc::c_ulong))
      as *mut cpu_stat;
  /*s->bar[sz] = '\0'; - xzalloc did it */
  (*s).bar_sz = sz as libc::c_uint; //sanitize
  (*s).collect = Some(collect_cpu as unsafe extern "C" fn(_: *mut cpu_stat) -> ()); //sanitize
  return s as *mut s_stat;
}
unsafe extern "C" fn collect_int(mut s: *mut int_stat) {
  let mut data: [ullong; 1] = [0; 1];
  let mut old: ullong = 0;
  if rdval(
    get_file(&mut (*ptr_to_globals).proc_stat),
    b"intr\x00" as *const u8 as *const libc::c_char,
    data.as_mut_ptr(),
    (1i32 << (*s).no) as libc::c_long,
  ) != 0
  {
    put_question_marks(4i32);
    return;
  }
  old = (*s).old;
  if data[0] < old {
    old = data[0]
  }
  (*s).old = data[0];
  scale(data[0].wrapping_sub(old));
}
unsafe extern "C" fn init_int(mut param: *const libc::c_char) -> *mut s_stat {
  let mut s: *mut int_stat =
    xzalloc(::std::mem::size_of::<int_stat>() as libc::c_ulong) as *mut int_stat;
  (*s).collect = Some(collect_int as unsafe extern "C" fn(_: *mut int_stat) -> ());
  if *param.offset(0) as libc::c_int == '\u{0}' as i32 {
    (*s).no = 1i32
  } else {
    let mut n: libc::c_int = xatoi_positive(param);
    (*s).no = n + 2i32
  }
  return s as *mut s_stat;
}
unsafe extern "C" fn collect_ctx(mut s: *mut ctx_stat) {
  let mut data: [ullong; 1] = [0; 1];
  let mut old: ullong = 0;
  if rdval(
    get_file(&mut (*ptr_to_globals).proc_stat),
    b"ctxt\x00" as *const u8 as *const libc::c_char,
    data.as_mut_ptr(),
    (1i32 << 1i32) as libc::c_long,
  ) != 0
  {
    put_question_marks(4i32);
    return;
  }
  old = (*s).old;
  if data[0] < old {
    old = data[0]
  }
  (*s).old = data[0];
  scale(data[0].wrapping_sub(old));
}
unsafe extern "C" fn init_ctx(mut _param: *const libc::c_char) -> *mut s_stat {
  let mut s: *mut ctx_stat =
    xzalloc(::std::mem::size_of::<ctx_stat>() as libc::c_ulong) as *mut ctx_stat;
  (*s).collect = Some(collect_ctx as unsafe extern "C" fn(_: *mut ctx_stat) -> ());
  return s as *mut s_stat;
}
unsafe extern "C" fn collect_blk(mut s: *mut blk_stat) {
  let mut data: [ullong; 2] = [0; 2];
  let mut i: libc::c_int = 0;
  if (*ptr_to_globals).is26 != 0 {
    i = rdval_diskstats(
      get_file(&mut (*ptr_to_globals).proc_diskstats),
      data.as_mut_ptr(),
    )
  } else {
    i = rdval(
      get_file(&mut (*ptr_to_globals).proc_stat),
      (*s).lookfor,
      data.as_mut_ptr(),
      (0i32 | 1i32 << 1i32 | 1i32 << 2i32) as libc::c_long,
    );
    // Linux 2.4 reports bio in Kbytes, convert to sectors:
    data[0] =
      (data[0] as libc::c_ulonglong).wrapping_mul(2i32 as libc::c_ulonglong) as ullong as ullong; //sanitize
    data[1] =
      (data[1] as libc::c_ulonglong).wrapping_mul(2i32 as libc::c_ulonglong) as ullong as ullong
  } // TODO: *sectorsize
  if i != 0 {
    put_question_marks(9i32); //sanitize
    return;
  } //sanitize
  i = 0;
  while i < 2i32 {
    let mut old: ullong = (*s).old[i as usize];
    if data[i as usize] < old {
      old = data[i as usize]
    }
    (*s).old[i as usize] = data[i as usize];
    data[i as usize] =
      (data[i as usize] as libc::c_ulonglong).wrapping_sub(old) as ullong as ullong;
    i += 1
  }
  scale(data[0].wrapping_mul(512i32 as libc::c_ulonglong));
  put_c(' ' as i32 as libc::c_char);
  scale(data[1].wrapping_mul(512i32 as libc::c_ulonglong));
}
unsafe extern "C" fn init_blk(mut _param: *const libc::c_char) -> *mut s_stat {
  let mut s: *mut blk_stat =
    xzalloc(::std::mem::size_of::<blk_stat>() as libc::c_ulong) as *mut blk_stat;
  (*s).collect = Some(collect_blk as unsafe extern "C" fn(_: *mut blk_stat) -> ());
  (*s).lookfor = b"page\x00" as *const u8 as *const libc::c_char;
  return s as *mut s_stat;
}
unsafe extern "C" fn collect_thread_nr(mut _s: *mut fork_stat) {
  let mut data: [ullong; 1] = [0; 1];
  if rdval_loadavg(
    get_file(&mut (*ptr_to_globals).proc_loadavg),
    data.as_mut_ptr(),
    (1i32 << 4i32) as libc::c_long,
  ) != 0
  {
    put_question_marks(4i32);
    return;
  }
  scale(data[0]);
}
unsafe extern "C" fn collect_fork(mut s: *mut fork_stat) {
  let mut data: [ullong; 1] = [0; 1];
  let mut old: ullong = 0;
  if rdval(
    get_file(&mut (*ptr_to_globals).proc_stat),
    b"processes\x00" as *const u8 as *const libc::c_char,
    data.as_mut_ptr(),
    (1i32 << 1i32) as libc::c_long,
  ) != 0
  {
    put_question_marks(4i32);
    return;
  }
  old = (*s).old;
  if data[0] < old {
    old = data[0]
  }
  (*s).old = data[0];
  scale(data[0].wrapping_sub(old));
}
unsafe extern "C" fn init_fork(mut param: *const libc::c_char) -> *mut s_stat {
  let mut s: *mut fork_stat =
    xzalloc(::std::mem::size_of::<fork_stat>() as libc::c_ulong) as *mut fork_stat;
  if *param as libc::c_int == 'n' as i32 {
    (*s).collect = Some(collect_thread_nr as unsafe extern "C" fn(_: *mut fork_stat) -> ())
  } else {
    (*s).collect = Some(collect_fork as unsafe extern "C" fn(_: *mut fork_stat) -> ())
  }
  return s as *mut s_stat;
}
unsafe extern "C" fn collect_if(mut s: *mut if_stat) {
  let mut data: [ullong; 4] = [0; 4];
  let mut i: libc::c_int = 0;
  if rdval(
    get_file(&mut (*ptr_to_globals).proc_net_dev),
    (*s).device_colon,
    data.as_mut_ptr(),
    (0i32 | 1i32 << 1i32 | 1i32 << 3i32 | 1i32 << 9i32 | 1i32 << 11i32) as libc::c_long,
  ) != 0
  {
    put_question_marks(10i32);
    return;
  }
  i = 0;
  while i < 4i32 {
    let mut old: ullong = (*s).old[i as usize];
    if data[i as usize] < old {
      old = data[i as usize]
    }
    (*s).old[i as usize] = data[i as usize];
    data[i as usize] =
      (data[i as usize] as libc::c_ulonglong).wrapping_sub(old) as ullong as ullong;
    i += 1
  }
  put_c(if data[1] != 0 { '*' as i32 } else { ' ' as i32 } as libc::c_char);
  scale(data[0]);
  put_c(if data[3] != 0 { '*' as i32 } else { ' ' as i32 } as libc::c_char);
  scale(data[2]);
}
unsafe extern "C" fn init_if(mut device: *const libc::c_char) -> *mut s_stat {
  let mut s: *mut if_stat =
    xzalloc(::std::mem::size_of::<if_stat>() as libc::c_ulong) as *mut if_stat;
  if device.is_null() || *device.offset(0) == 0 {
    bb_show_usage();
  }
  (*s).collect = Some(collect_if as unsafe extern "C" fn(_: *mut if_stat) -> ());
  (*s).device = device;
  (*s).device_colon = xasprintf(b"%s:\x00" as *const u8 as *const libc::c_char, device);
  return s as *mut s_stat;
}
// "Memory" value should not include any caches.
// IOW: neither "ls -laR /" nor heavy read/write activity
//      should affect it. We'd like to also include any
//      long-term allocated kernel-side mem, but it is hard
//      to figure out. For now, bufs, cached & slab are
//      counted as "free" memory
//2.6.16:
//MemTotal:       773280 kB
//MemFree:         25912 kB - genuinely free
//Buffers:        320672 kB - cache
//Cached:         146396 kB - cache
//SwapCached:          0 kB
//Active:         183064 kB
//Inactive:       356892 kB
//HighTotal:           0 kB
//HighFree:            0 kB
//LowTotal:       773280 kB
//LowFree:         25912 kB
//SwapTotal:      131064 kB
//SwapFree:       131064 kB
//Dirty:              48 kB
//Writeback:           0 kB
//Mapped:          96620 kB
//Slab:           200668 kB - takes 7 Mb on my box fresh after boot,
//                            but includes dentries and inodes
//                            (== can take arbitrary amount of mem)
//CommitLimit:    517704 kB
//Committed_AS:   236776 kB
//PageTables:       1248 kB
//VmallocTotal:   516052 kB
//VmallocUsed:      3852 kB
//VmallocChunk:   512096 kB
//HugePages_Total:     0
//HugePages_Free:      0
//Hugepagesize:     4096 kB
unsafe extern "C" fn collect_mem(mut s: *mut mem_stat) {
  let mut m_total: ullong = 0 as ullong;
  let mut m_free: ullong = 0 as ullong;
  let mut m_bufs: ullong = 0 as ullong;
  let mut m_cached: ullong = 0 as ullong;
  let mut m_slab: ullong = 0 as ullong;
  if rdval(
    get_file(&mut (*ptr_to_globals).proc_meminfo),
    b"MemTotal:\x00" as *const u8 as *const libc::c_char,
    &mut m_total,
    (1i32 << 1i32) as libc::c_long,
  ) != 0
  {
    put_question_marks(4i32);
    return;
  }
  if (*s).opt as libc::c_int == 't' as i32 {
    scale(m_total << 10i32);
    return;
  }
  if rdval(
    (*ptr_to_globals).proc_meminfo.file,
    b"MemFree:\x00" as *const u8 as *const libc::c_char,
    &mut m_free,
    (1i32 << 1i32) as libc::c_long,
  ) != 0
    || rdval(
      (*ptr_to_globals).proc_meminfo.file,
      b"Buffers:\x00" as *const u8 as *const libc::c_char,
      &mut m_bufs,
      (1i32 << 1i32) as libc::c_long,
    ) != 0
    || rdval(
      (*ptr_to_globals).proc_meminfo.file,
      b"Cached:\x00" as *const u8 as *const libc::c_char,
      &mut m_cached,
      (1i32 << 1i32) as libc::c_long,
    ) != 0
    || rdval(
      (*ptr_to_globals).proc_meminfo.file,
      b"Slab:\x00" as *const u8 as *const libc::c_char,
      &mut m_slab,
      (1i32 << 1i32) as libc::c_long,
    ) != 0
  {
    put_question_marks(4i32);
    return;
  }
  m_free = (m_free as libc::c_ulonglong)
    .wrapping_add(m_bufs.wrapping_add(m_cached).wrapping_add(m_slab)) as ullong
    as ullong;
  match (*s).opt as libc::c_int {
    102 => {
      scale(m_free << 10i32);
    }
    _ => {
      scale(m_total.wrapping_sub(m_free) << 10i32);
    }
  };
}
unsafe extern "C" fn init_mem(mut param: *const libc::c_char) -> *mut s_stat {
  let mut s: *mut mem_stat =
    xzalloc(::std::mem::size_of::<mem_stat>() as libc::c_ulong) as *mut mem_stat;
  (*s).collect = Some(collect_mem as unsafe extern "C" fn(_: *mut mem_stat) -> ());
  (*s).opt = *param.offset(0);
  return s as *mut s_stat;
}
unsafe extern "C" fn collect_swp(mut _s: *mut swp_stat) {
  let mut s_total: [ullong; 1] = [0; 1];
  let mut s_free: [ullong; 1] = [0; 1];
  if rdval(
    get_file(&mut (*ptr_to_globals).proc_meminfo),
    b"SwapTotal:\x00" as *const u8 as *const libc::c_char,
    s_total.as_mut_ptr(),
    (1i32 << 1i32) as libc::c_long,
  ) != 0
    || rdval(
      (*ptr_to_globals).proc_meminfo.file,
      b"SwapFree:\x00" as *const u8 as *const libc::c_char,
      s_free.as_mut_ptr(),
      (1i32 << 1i32) as libc::c_long,
    ) != 0
  {
    put_question_marks(4i32);
    return;
  }
  scale(s_total[0].wrapping_sub(s_free[0]) << 10i32);
}
unsafe extern "C" fn init_swp(mut _param: *const libc::c_char) -> *mut s_stat {
  let mut s: *mut swp_stat =
    xzalloc(::std::mem::size_of::<swp_stat>() as libc::c_ulong) as *mut swp_stat;
  (*s).collect = Some(collect_swp as unsafe extern "C" fn(_: *mut swp_stat) -> ());
  return s as *mut s_stat;
}
unsafe extern "C" fn collect_fd(mut _s: *mut fd_stat) {
  let mut data: [ullong; 2] = [0; 2];
  if rdval(
    get_file(&mut (*ptr_to_globals).proc_sys_fs_filenr),
    b"\x00" as *const u8 as *const libc::c_char,
    data.as_mut_ptr(),
    (0i32 | 1i32 << 1i32 | 1i32 << 2i32) as libc::c_long,
  ) != 0
  {
    put_question_marks(4i32);
    return;
  }
  scale(data[0].wrapping_sub(data[1]));
}
unsafe extern "C" fn init_fd(mut _param: *const libc::c_char) -> *mut s_stat {
  let mut s: *mut fd_stat =
    xzalloc(::std::mem::size_of::<fd_stat>() as libc::c_ulong) as *mut fd_stat;
  (*s).collect = Some(collect_fd as unsafe extern "C" fn(_: *mut fd_stat) -> ());
  return s as *mut s_stat;
}
unsafe extern "C" fn collect_time(mut s: *mut time_stat) {
  let mut buf: [libc::c_char; 16] = [0; 16];
  let mut tm: *mut tm = std::ptr::null_mut();
  let mut us: libc::c_uint = ((*ptr_to_globals).tv.tv_usec
    + (*s).scale.wrapping_div(2i32 as libc::c_uint) as libc::c_long)
    as libc::c_uint;
  let mut t: time_t = (*ptr_to_globals).tv.tv_sec;
  if us >= 1000000i32 as libc::c_uint {
    t += 1;
    us = us.wrapping_sub(1000000i32 as libc::c_uint)
  }
  tm = localtime(&mut t);
  sprintf(
    buf.as_mut_ptr(),
    b"%02d:%02d:%02d\x00" as *const u8 as *const libc::c_char,
    (*tm).tm_hour,
    (*tm).tm_min,
    (*tm).tm_sec,
  );
  if (*s).prec != 0 {
    sprintf(
      buf.as_mut_ptr().offset(8),
      b".%0*d\x00" as *const u8 as *const libc::c_char,
      (*s).prec,
      us.wrapping_div((*s).scale),
    );
  }
  put(buf.as_mut_ptr());
}
unsafe extern "C" fn init_time(mut param: *const libc::c_char) -> *mut s_stat {
  let mut prec: libc::c_int = 0;
  let mut s: *mut time_stat =
    xzalloc(::std::mem::size_of::<time_stat>() as libc::c_ulong) as *mut time_stat;
  (*s).collect = Some(collect_time as unsafe extern "C" fn(_: *mut time_stat) -> ());
  prec = *param.offset(0) as libc::c_int - '0' as i32;
  if prec < 0 {
    prec = 0
  } else if prec > 6i32 {
    prec = 6i32
  }
  (*s).prec = prec as libc::c_uint;
  (*s).scale = 1i32 as libc::c_uint;
  loop {
    let fresh7 = prec;
    prec = prec + 1;
    if !(fresh7 < 6i32) {
      break;
    }
    (*s).scale = (*s).scale.wrapping_mul(10i32 as libc::c_uint)
  }
  return s as *mut s_stat;
}
unsafe extern "C" fn collect_info(mut s: *mut s_stat) {
  (*ptr_to_globals).gen = ((*ptr_to_globals).gen as libc::c_int ^ 1i32) as smallint;
  while !s.is_null() {
    put((*s).label);
    (*s).collect.expect("non-null function pointer")(s);
    s = (*s).next
  }
}
static mut options: [libc::c_char; 12] = [110, 99, 109, 115, 102, 105, 120, 112, 116, 98, 114, 0];

static mut init_functions: [Option<unsafe extern "C" fn(_: *const libc::c_char) -> *mut s_stat>;
  11] = [
  Some(init_if as unsafe extern "C" fn(_: *const libc::c_char) -> *mut s_stat),
  Some(init_cpu as unsafe extern "C" fn(_: *const libc::c_char) -> *mut s_stat),
  Some(init_mem as unsafe extern "C" fn(_: *const libc::c_char) -> *mut s_stat),
  Some(init_swp as unsafe extern "C" fn(_: *const libc::c_char) -> *mut s_stat),
  Some(init_fd as unsafe extern "C" fn(_: *const libc::c_char) -> *mut s_stat),
  Some(init_int as unsafe extern "C" fn(_: *const libc::c_char) -> *mut s_stat),
  Some(init_ctx as unsafe extern "C" fn(_: *const libc::c_char) -> *mut s_stat),
  Some(init_fork as unsafe extern "C" fn(_: *const libc::c_char) -> *mut s_stat),
  Some(init_time as unsafe extern "C" fn(_: *const libc::c_char) -> *mut s_stat),
  Some(init_blk as unsafe extern "C" fn(_: *const libc::c_char) -> *mut s_stat),
  Some(init_cr as unsafe extern "C" fn(_: *const libc::c_char) -> *mut s_stat),
];

#[no_mangle]
pub unsafe extern "C" fn nmeter_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut buf: [libc::c_char; 32] = [0; 32];
  let mut first: *mut s_stat = std::ptr::null_mut();
  let mut last: *mut s_stat = std::ptr::null_mut();
  let mut s: *mut s_stat = std::ptr::null_mut();
  let mut opt_d: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut cur: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut prev: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let ref mut fresh8 = *(not_const_pp(&ptr_to_globals as *const *mut globals as *const libc::c_void)
    as *mut *mut globals);
  *fresh8 = xzalloc(::std::mem::size_of::<globals>() as libc::c_ulong) as *mut globals;
  asm!("" : : : "memory" : "volatile");
  (*ptr_to_globals).cur_outbuf = bb_common_bufsiz1.as_mut_ptr();
  (*ptr_to_globals).final_char = '\n' as i32 as libc::c_char;
  (*ptr_to_globals).delta = 1000000i32;
  (*ptr_to_globals).deltanz = (*ptr_to_globals).delta as libc::c_uint;
  xchdir(b"/proc\x00" as *const u8 as *const libc::c_char);
  if open_read_close(
    b"version\x00" as *const u8 as *const libc::c_char,
    buf.as_mut_ptr() as *mut libc::c_void,
    (::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
      .wrapping_sub(1i32 as libc::c_ulong),
  ) > 0
  {
    buf[(::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
      .wrapping_sub(1i32 as libc::c_ulong) as usize] = '\u{0}' as i32 as libc::c_char;
    (*ptr_to_globals).is26 = (strstr(
      buf.as_mut_ptr(),
      b" 2.4.\x00" as *const u8 as *const libc::c_char,
    ) == std::ptr::null_mut()) as libc::c_int as smallint
  }
  if getopt32(
    argv,
    b"d:\x00" as *const u8 as *const libc::c_char,
    &mut opt_d as *mut *mut libc::c_char,
  ) != 0
  {
    (*ptr_to_globals).delta = xatoi(opt_d) * 1000i32;
    (*ptr_to_globals).deltanz = if (*ptr_to_globals).delta > 0 {
      (*ptr_to_globals).delta
    } else {
      1i32
    } as libc::c_uint;
    (*ptr_to_globals).need_seconds = ((1000000i32 as libc::c_uint)
      .wrapping_rem((*ptr_to_globals).deltanz)
      != 0 as libc::c_uint) as libc::c_int as smallint
  }
  argv = argv.offset(optind as isize);
  if (*argv.offset(0)).is_null() {
    bb_show_usage();
  }
  // Can use argv[0] directly, but this will mess up
  // parameters as seen by e.g. ps. Making a copy...
  cur = xstrdup(*argv.offset(0));
  's_129: loop {
    let mut param: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut p: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    prev = cur;
    loop {
      cur = strchr(cur, '%' as i32);
      if cur.is_null() {
        break 's_129;
      }
      if !(*cur.offset(1) as libc::c_int == '%' as i32) {
        break;
      }
      // %%
      overlapping_strcpy(cur, cur.offset(1)); // overwrite %
      cur = cur.offset(1)
    }
    let fresh9 = cur;
    cur = cur.offset(1);
    *fresh9 = '\u{0}' as i32 as libc::c_char;
    if *cur.offset(0) as libc::c_int == '[' as i32 {
      // format: %[foptstring]
      cur = cur.offset(1);
      p = strchr(options.as_ptr(), *cur.offset(0) as libc::c_int);
      param = cur.offset(1);
      while *cur.offset(0) as libc::c_int != ']' as i32 {
        if *cur.offset(0) == 0 {
          bb_show_usage();
        }
        cur = cur.offset(1)
      }
      let fresh10 = cur;
      cur = cur.offset(1);
      *fresh10 = '\u{0}' as i32 as libc::c_char
    // overwrite [
    } else {
      // format: %NNNNNNf
      param = cur;
      while *cur.offset(0) as libc::c_int >= '0' as i32
        && *cur.offset(0) as libc::c_int <= '9' as i32
      {
        cur = cur.offset(1)
      }
      if *cur.offset(0) == 0 {
        bb_show_usage();
      }
      p = strchr(options.as_ptr(), *cur.offset(0) as libc::c_int);
      let fresh11 = cur;
      cur = cur.offset(1);
      *fresh11 = '\u{0}' as i32 as libc::c_char
      // overwrite format char
    }
    if p.is_null() {
      bb_show_usage();
    }
    s = init_functions[p.wrapping_offset_from(options.as_ptr()) as libc::c_long as usize]
      .expect("non-null function pointer")(param);
    if !s.is_null() {
      (*s).label = prev;
      /*s->next = NULL; - all initXXX funcs use xzalloc */
      if first.is_null() {
        first = s
      } else {
        (*last).next = s
      }
      last = s
    } else {
      // %r option. remove it from string
      overlapping_strcpy(prev.offset(strlen(prev) as isize), cur);
      cur = prev
    }
  }
  if *prev.offset(0) != 0 {
    s = init_literal();
    (*s).label = prev;
    /*s->next = NULL; - all initXXX funcs use xzalloc */
    if first.is_null() {
      first = s
    } else {
      (*last).next = s
    }
    last = s
  }
  // Generate first samples but do not print them, they're bogus
  collect_info(first);
  reset_outbuf();
  if (*ptr_to_globals).delta >= 0 {
    gettimeofday(&mut (*ptr_to_globals).tv, 0 as *mut timezone);
    usleep(if (*ptr_to_globals).delta > 1000000i32 {
      1000000i32 as libc::c_long
    } else {
      ((*ptr_to_globals).delta as libc::c_long)
        - (*ptr_to_globals).tv.tv_usec % (*ptr_to_globals).deltanz as libc::c_long
    } as useconds_t);
  }
  loop {
    gettimeofday(&mut (*ptr_to_globals).tv, 0 as *mut timezone);
    collect_info(first);
    put_c((*ptr_to_globals).final_char);
    print_outbuf();
    // Negative delta -> no usleep at all
    // This will hog the CPU but you can have REALLY GOOD
    // time resolution ;)
    // TODO: detect and avoid useless updates
    // (like: nothing happens except time)
    if (*ptr_to_globals).delta >= 0 {
      let mut rem: libc::c_int = 0;
      // can be commented out, will sacrifice sleep time precision a bit
      gettimeofday(&mut (*ptr_to_globals).tv, 0 as *mut timezone);
      if (*ptr_to_globals).need_seconds != 0 {
        rem = ((*ptr_to_globals).delta as libc::c_ulonglong).wrapping_sub(
          ((*ptr_to_globals).tv.tv_sec as ullong)
            .wrapping_mul(1000000i32 as libc::c_ulonglong)
            .wrapping_add((*ptr_to_globals).tv.tv_usec as libc::c_ulonglong)
            .wrapping_rem((*ptr_to_globals).deltanz as libc::c_ulonglong),
        ) as libc::c_int
      } else {
        rem = ((*ptr_to_globals).delta as libc::c_uint).wrapping_sub(
          ((*ptr_to_globals).tv.tv_usec as libc::c_uint).wrapping_rem((*ptr_to_globals).deltanz),
        ) as libc::c_int
      }
      // Sometimes kernel wakes us up just a tiny bit earlier than asked
      // Do not go to very short sleep in this case
      if (rem as libc::c_uint)
        < ((*ptr_to_globals).delta as libc::c_uint).wrapping_div(128i32 as libc::c_uint)
      {
        rem += (*ptr_to_globals).delta
      }
      usleep(rem as useconds_t);
    }
  }
  /*return 0;*/
}
