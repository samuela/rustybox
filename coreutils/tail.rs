use crate::libbb::xfuncs_printf::xmalloc;
use libc;
use libc::close;
use libc::fstat;
use libc::open;
use libc::sleep;
extern "C" {

  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  static bb_msg_standard_input: [libc::c_char; 0];

  #[no_mangle]
  fn dprintf(__fd: libc::c_int, __fmt: *const libc::c_char, _: ...) -> libc::c_int;

  #[no_mangle]
  fn lseek(__fd: libc::c_int, __offset: off64_t, __whence: libc::c_int) -> off64_t;
  #[no_mangle]
  fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  fn xrealloc(old: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn bb_copyfd_size(fd1: libc::c_int, fd2: libc::c_int, size: off_t) -> off_t;
  #[no_mangle]
  fn open_or_warn_stdin(pathname: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn xlseek(fd: libc::c_int, offset: off_t, whence: libc::c_int) -> off_t;
  #[no_mangle]
  fn full_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn xwrite(fd: libc::c_int, buf: *const libc::c_void, count: size_t);
  #[no_mangle]
  static bkm_suffixes: [suffix_mult; 0];
  #[no_mangle]
  fn xatou_sfx(str: *const libc::c_char, sfx: *const suffix_mult) -> libc::c_uint;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_perror_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);
  #[no_mangle]
  fn bb_perror_nomsg_and_die() -> !;
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
}

use crate::librb::size_t;
use libc::off64_t;
use libc::off_t;
use libc::ssize_t;
use libc::stat;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct suffix_mult {
  pub suffix: [libc::c_char; 4],
  pub mult: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub from_top: bool,
  pub exitcode: bool,
}
unsafe extern "C" fn tail_xprint_header(
  mut fmt: *const libc::c_char,
  mut filename: *const libc::c_char,
) {
  if dprintf(1i32, fmt, filename) < 0i32 {
    bb_perror_nomsg_and_die();
  };
}
unsafe extern "C" fn tail_read(
  mut fd: libc::c_int,
  mut buf: *mut libc::c_char,
  mut count: size_t,
) -> ssize_t {
  let mut r: ssize_t = 0;
  r = full_read(fd, buf as *mut libc::c_void, count);
  if r < 0 {
    bb_simple_perror_msg(b"read error\x00" as *const u8 as *const libc::c_char);
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).exitcode = 1i32 != 0
  }
  return r;
}
unsafe extern "C" fn eat_num(mut p: *const libc::c_char) -> libc::c_uint {
  if *p as libc::c_int == '-' as i32 {
    p = p.offset(1)
  } else if *p as libc::c_int == '+' as i32 {
    p = p.offset(1);
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).from_top = 1i32 != 0
  }
  return xatou_sfx(p, bkm_suffixes.as_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn tail_main(
  mut argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut count: libc::c_uint = 10i32 as libc::c_uint;
  let mut sleep_period: libc::c_uint = 1i32 as libc::c_uint;
  let mut str_c: *const libc::c_char = 0 as *const libc::c_char;
  let mut str_n: *const libc::c_char = 0 as *const libc::c_char;
  let mut tailbuf: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut tailbufsize: size_t = 0;
  let mut header_threshhold: libc::c_uint = 1i32 as libc::c_uint;
  let mut nfiles: libc::c_uint = 0;
  let mut i: libc::c_int = 0;
  let mut opt: libc::c_int = 0;
  let mut fds: *mut libc::c_int = std::ptr::null_mut();
  let mut fmt: *const libc::c_char = 0 as *const libc::c_char;
  let mut prev_fd: libc::c_int = 0;
  /* Allow legacy syntax of an initial numeric option without -n. */
  if !(*argv.offset(1)).is_null()
    && (*(*argv.offset(1)).offset(0) as libc::c_int == '+' as i32
      || *(*argv.offset(1)).offset(0) as libc::c_int == '-' as i32)
    && (*(*argv.offset(1)).offset(1) as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int
      <= 9i32
  {
    count = eat_num(*argv.offset(1));
    argv = argv.offset(1);
    argc -= 1
  }
  /* -s NUM, -F imlies -f */
  opt = getopt32(
    argv,
    b"^fc:n:qs:+vF\x00Ff\x00" as *const u8 as *const libc::c_char,
    &mut str_c as *mut *const libc::c_char,
    &mut str_n as *mut *const libc::c_char,
    &mut sleep_period as *mut libc::c_uint,
  ) as libc::c_int;
  //if (opt & 0x1) // -f
  if opt & 0x2i32 != 0 {
    count = eat_num(str_c)
  } // -c
  if opt & 0x4i32 != 0 {
    count = eat_num(str_n)
  } // -n
    /* q: make it impossible for nfiles to be > header_threshhold */
  if opt & 0x8i32 != 0 {
    header_threshhold = (2147483647i32 as libc::c_uint)
      .wrapping_mul(2u32)
      .wrapping_add(1u32)
  } // -q
    //if (opt & 0x10) // -s
  if opt & 0x20i32 != 0 {
    header_threshhold = 0i32 as libc::c_uint
  } // -v
  argc -= optind;
  argv = argv.offset(optind as isize);
  /* open all the files */
  fds = xmalloc(
    (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
      .wrapping_mul((argc + 1i32) as libc::c_ulong),
  ) as *mut libc::c_int;
  if (*argv.offset(0)).is_null() {
    let mut statbuf: stat = std::mem::zeroed();
    if fstat(0i32, &mut statbuf) == 0i32
      && statbuf.st_mode & 0o170000i32 as libc::c_uint == 0o10000i32 as libc::c_uint
    {
      opt &= !1i32
      /* clear FOLLOW */
    }
    let ref mut fresh0 = *argv.offset(0);
    *fresh0 = bb_msg_standard_input.as_ptr() as *mut libc::c_char
  }
  i = 0i32;
  nfiles = i as libc::c_uint;
  loop {
    let mut fd: libc::c_int = open_or_warn_stdin(*argv.offset(i as isize));
    if fd < 0i32 && opt & 0x40i32 == 0 {
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).exitcode = 1i32 != 0
    } else {
      *fds.offset(nfiles as isize) = fd;
      let fresh1 = nfiles;
      nfiles = nfiles.wrapping_add(1);
      let ref mut fresh2 = *argv.offset(fresh1 as isize);
      *fresh2 = *argv.offset(i as isize)
    }
    i += 1;
    if !(i < argc) {
      break;
    }
  }
  if nfiles == 0 {
    bb_simple_error_msg_and_die(b"no files\x00" as *const u8 as *const libc::c_char);
  }
  /* prepare the buffer */
  tailbufsize = 8192i32 as size_t;
  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).from_top && opt & 0x2i32 != 0 {
    if tailbufsize < count.wrapping_add(8192i32 as libc::c_uint) as libc::c_ulong {
      tailbufsize = count.wrapping_add(8192i32 as libc::c_uint) as size_t
    }
  }
  /* tail -c1024m REGULAR_FILE doesn't really need 1G mem block.
   * (In fact, it doesn't need ANY memory). So delay allocation.
   */
  tailbuf = std::ptr::null_mut::<libc::c_char>();
  /* tail the files */
  fmt = (b"\n==> %s <==\n\x00" as *const u8 as *const libc::c_char).offset(1); /* skip leading newline in the header on the first output */
  i = 0i32; /* may happen with -F */
  let mut current_block_118: u64;
  loop {
    let mut buf: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut taillen: libc::c_int = 0;
    let mut newlines_seen: libc::c_int = 0;
    let mut seen: libc::c_uint = 0;
    let mut nread: libc::c_int = 0;
    let mut fd_0: libc::c_int = *fds.offset(i as isize);
    if !(1i32 != 0 && fd_0 < 0i32) {
      if nfiles > header_threshhold {
        tail_xprint_header(fmt, *argv.offset(i as isize));
        fmt = b"\n==> %s <==\n\x00" as *const u8 as *const libc::c_char
      }
      if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).from_top {
        let mut current: off_t = lseek(fd_0, 0i32 as off64_t, 2i32);
        if current > 0 {
          let mut off: libc::c_uint = 0;
          if opt & 0x2i32 != 0 {
            /* Optimizing count-bytes case if the file is seekable.
             * Beware of backing up too far.
             * Also we exclude files with size 0 (because of /proc/xxx) */
            if count == 0i32 as libc::c_uint {
              current_block_118 = 11763295167351361500; /* showing zero bytes is easy :) */
            } else {
              current -= count as libc::c_long;
              if current < 0 {
                current = 0i32 as off_t
              }
              xlseek(fd_0, current, 0i32);
              bb_copyfd_size(fd_0, 1i32, count as off_t);
              current_block_118 = 11763295167351361500;
            }
          } else {
            /* This is technically incorrect for *LONG* strings, but very useful */
            /* Optimizing count-lines case if the file is seekable.
             * We assume the lines are <64k.
             * (Users complain that tail takes too long
             * on multi-gigabyte files) */
            off = count | 0xfi32 as libc::c_uint; /* for small counts, be more paranoid */
            if off > (2147483647i32 / (64i32 * 1024i32)) as libc::c_uint {
              off = (2147483647i32 / (64i32 * 1024i32)) as libc::c_uint
            }
            current -= off.wrapping_mul((64i32 * 1024i32) as libc::c_uint) as libc::c_long;
            if current < 0 {
              current = 0i32 as off_t
            }
            xlseek(fd_0, current, 0i32);
            current_block_118 = 5873035170358615968;
          }
        } else {
          current_block_118 = 5873035170358615968;
        }
      } else {
        current_block_118 = 5873035170358615968;
      }
      match current_block_118 {
        11763295167351361500 => {}
        _ => {
          if tailbuf.is_null() {
            tailbuf = xmalloc(tailbufsize) as *mut libc::c_char
          }
          buf = tailbuf;
          taillen = 0i32;
          /* "We saw 1st line/byte".
           * Used only by +N code ("start from Nth", 1-based): */
          seen = 1i32 as libc::c_uint; /* while (tail_read() > 0) */
          newlines_seen = 0i32;
          loop {
            nread = tail_read(
              fd_0,
              buf,
              tailbufsize.wrapping_sub(taillen as libc::c_ulong),
            ) as libc::c_int;
            if !(nread > 0i32) {
              break;
            }
            if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).from_top {
              let mut nwrite: libc::c_int = nread;
              if seen < count {
                /* We need to skip a few more bytes/lines */
                if opt & 0x2i32 != 0 {
                  nwrite = (nwrite as libc::c_uint).wrapping_sub(count.wrapping_sub(seen))
                    as libc::c_int as libc::c_int;
                  seen = seen.wrapping_add(nread as libc::c_uint)
                } else {
                  let mut s: *mut libc::c_char = buf;
                  loop {
                    nwrite -= 1;
                    let fresh3 = s;
                    s = s.offset(1);
                    if *fresh3 as libc::c_int == '\n' as i32 && {
                      seen = seen.wrapping_add(1);
                      (seen) == count
                    } {
                      break;
                    }
                    if !(nwrite != 0) {
                      break;
                    }
                  }
                }
              }
              if nwrite > 0i32 {
                xwrite(
                  1i32,
                  buf.offset(nread as isize).offset(-(nwrite as isize)) as *const libc::c_void,
                  nwrite as size_t,
                );
              }
            } else if count != 0 {
              if opt & 0x2i32 != 0 {
                taillen += nread;
                if taillen > count as libc::c_int {
                  memmove(
                    tailbuf as *mut libc::c_void,
                    tailbuf.offset(taillen as isize).offset(-(count as isize))
                      as *const libc::c_void,
                    count as libc::c_ulong,
                  );
                  taillen = count as libc::c_int
                }
              } else {
                let mut k: libc::c_int = nread;
                let mut newlines_in_buf: libc::c_int = 0i32;
                loop {
                  /* count '\n' in last read */
                  k -= 1; /* while (1) */
                  if *buf.offset(k as isize) as libc::c_int == '\n' as i32 {
                    newlines_in_buf += 1
                  }
                  if !(k != 0) {
                    break;
                  }
                }
                if newlines_seen + newlines_in_buf < count as libc::c_int {
                  newlines_seen += newlines_in_buf;
                  taillen += nread
                } else {
                  let mut extra: libc::c_int = (*buf.offset((nread - 1i32) as isize) as libc::c_int
                    != '\n' as i32) as libc::c_int;
                  let mut s_0: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
                  k = ((newlines_seen + newlines_in_buf + extra) as libc::c_uint)
                    .wrapping_sub(count) as libc::c_int;
                  s_0 = tailbuf;
                  while k != 0 {
                    if *s_0 as libc::c_int == '\n' as i32 {
                      k -= 1
                    }
                    s_0 = s_0.offset(1)
                  }
                  taillen = (taillen as libc::c_long
                    + (nread as libc::c_long - s_0.wrapping_offset_from(tailbuf) as libc::c_long))
                    as libc::c_int;
                  memmove(
                    tailbuf as *mut libc::c_void,
                    s_0 as *const libc::c_void,
                    taillen as libc::c_ulong,
                  );
                  newlines_seen = count.wrapping_sub(extra as libc::c_uint) as libc::c_int
                }
                if tailbufsize < (taillen as size_t).wrapping_add(8192i32 as libc::c_ulong) {
                  tailbufsize = (taillen + 8192i32) as size_t;
                  tailbuf = xrealloc(tailbuf as *mut libc::c_void, tailbufsize) as *mut libc::c_char
                }
              }
              buf = tailbuf.offset(taillen as isize)
            }
          }
          if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).from_top {
            xwrite(1i32, tailbuf as *const libc::c_void, taillen as size_t);
          }
        }
      }
    }
    i += 1;
    if !((i as libc::c_uint) < nfiles) {
      break;
    }
  }
  prev_fd = *fds.offset((i - 1i32) as isize);
  tailbuf = xrealloc(tailbuf as *mut libc::c_void, 8192i32 as size_t) as *mut libc::c_char;
  fmt = 0 as *const libc::c_char;
  if opt & 0x1i32 != 0 {
    loop {
      sleep(sleep_period);
      i = 0i32;
      loop {
        let mut nread_0: libc::c_int = 0;
        let mut filename: *const libc::c_char = *argv.offset(i as isize);
        let mut fd_1: libc::c_int = *fds.offset(i as isize);
        if opt & 0x40i32 != 0 {
          let mut sbuf: stat = std::mem::zeroed();
          let mut fsbuf: stat = std::mem::zeroed();
          if fd_1 < 0i32
            || fstat(fd_1, &mut fsbuf) < 0i32
            || stat(filename, &mut sbuf) < 0i32
            || fsbuf.st_dev != sbuf.st_dev
            || fsbuf.st_ino != sbuf.st_ino
          {
            let mut new_fd: libc::c_int = 0;
            if fd_1 >= 0i32 {
              close(fd_1);
            }
            new_fd = open(filename, 0i32);
            if new_fd >= 0i32 {
              bb_error_msg(
                b"%s has %s; following end of new file\x00" as *const u8 as *const libc::c_char,
                filename,
                if fd_1 < 0i32 {
                  b"appeared\x00" as *const u8 as *const libc::c_char
                } else {
                  b"been replaced\x00" as *const u8 as *const libc::c_char
                },
              );
            } else if fd_1 >= 0i32 {
              bb_perror_msg(
                b"%s has become inaccessible\x00" as *const u8 as *const libc::c_char,
                filename,
              );
            }
            fd_1 = new_fd;
            *fds.offset(i as isize) = fd_1
          }
        }
        if !(1i32 != 0 && fd_1 < 0i32) {
          if nfiles > header_threshhold {
            fmt = b"\n==> %s <==\n\x00" as *const u8 as *const libc::c_char
          }
          loop
          /* tail -f keeps following files even if they are truncated */
          {
            let mut sbuf_0: stat = std::mem::zeroed();
            /* /proc files report zero st_size, don't lseek them */
            if fstat(fd_1, &mut sbuf_0) == 0i32 && sbuf_0.st_size > 0 {
              let mut current_0: off_t = lseek(fd_1, 0i32 as off64_t, 1i32);
              if sbuf_0.st_size < current_0 {
                xlseek(fd_1, 0i32 as off_t, 0i32);
              }
            }
            nread_0 = tail_read(fd_1, tailbuf, 8192i32 as size_t) as libc::c_int;
            if nread_0 <= 0i32 {
              break;
            }
            if !fmt.is_null() && fd_1 != prev_fd {
              tail_xprint_header(fmt, filename);
              fmt = 0 as *const libc::c_char;
              prev_fd = fd_1
            }
            xwrite(1i32, tailbuf as *const libc::c_void, nread_0 as size_t);
          }
        }
        i += 1;
        if !((i as libc::c_uint) < nfiles) {
          break;
        }
      }
    }
  }
  return (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).exitcode as libc::c_int;
}
