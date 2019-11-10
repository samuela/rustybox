use crate::libbb::llist::llist_t;
use crate::librb::signal::__sighandler_t;
use crate::librb::size_t;
use crate::librb::smallint;
use libc;
use libc::alarm;
use libc::atoi;
use libc::close;
use libc::free;
use libc::pollfd;
use libc::sleep;
use libc::ssize_t;
use libc::strcmp;
use libc::useconds_t;
extern "C" {

  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
  #[no_mangle]
  fn usleep(__useconds: useconds_t) -> libc::c_int;

  #[no_mangle]
  fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;

  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn tcsendbreak(__fd: libc::c_int, __duration: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn trim(s: *mut libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_process_escape_sequence(ptr: *mut *const libc::c_char) -> libc::c_char;
  #[no_mangle]
  fn bb_signals(sigs: libc::c_int, f: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>);
  #[no_mangle]
  static mut bb_got_signal: smallint;
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn safe_read(fd: libc::c_int, buf_0: *mut libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn xmalloc_xopen_read_close(
    filename: *const libc::c_char,
    maxsz_p: *mut size_t,
  ) -> *mut libc::c_void;
  #[no_mangle]
  fn safe_write(fd: libc::c_int, buf_0: *const libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn full_write(fd: libc::c_int, buf_0: *const libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn xwrite(fd: libc::c_int, buf_0: *const libc::c_void, count: size_t);
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  fn llist_add_to_end(list_head: *mut *mut llist_t, data: *mut libc::c_void);
  #[no_mangle]
  fn llist_unlink(head: *mut *mut llist_t, elm: *mut llist_t);
  #[no_mangle]
  static mut xfunc_error_retval: u8;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_simple_error_msg(s: *const libc::c_char);
  #[no_mangle]
  fn index_in_strings(strings: *const libc::c_char, key: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
}

pub type nfds_t = libc::c_ulong;

pub type C2RustUnnamed = libc::c_uint;
pub const COMMON_BUFSIZE: C2RustUnnamed = 1024;
// max length of "abort string",
// i.e. device reply which causes termination
// possible exit codes
pub type C2RustUnnamed_0 = libc::c_uint;
// first abort condition was met
//	ERR_ABORT2,     // second abort condition was met
//	...
// timed out while expecting
pub const ERR_ABORT: C2RustUnnamed_0 = 4;
// signalled or I/O error
pub const ERR_TIMEOUT: C2RustUnnamed_0 = 3;
// read too much while expecting
pub const ERR_IO: C2RustUnnamed_0 = 2;
// all's well
pub const ERR_MEM: C2RustUnnamed_0 = 1;
pub const ERR_OK: C2RustUnnamed_0 = 0;
pub const DIR_SAY: C2RustUnnamed_1 = 5;
pub const DIR_RECORD: C2RustUnnamed_1 = 6;
pub const DIR_ECHO: C2RustUnnamed_1 = 4;
pub const DIR_TIMEOUT: C2RustUnnamed_1 = 3;
pub const DIR_CLR_ABORT: C2RustUnnamed_1 = 2;
pub const DIR_ABORT: C2RustUnnamed_1 = 1;
pub const DIR_HANGUP: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_1 = libc::c_uint;
// exit code
// trap for critical signals
unsafe extern "C" fn signal_handler(mut _signo: libc::c_int) {
  // report I/O error condition
  bb_got_signal = ERR_IO as libc::c_int as smallint;
}
unsafe extern "C" fn unescape(mut s: *mut libc::c_char, mut nocr: *mut libc::c_int) -> size_t {
  let mut start: *mut libc::c_char = s;
  let mut p: *mut libc::c_char = s;
  while *s != 0 {
    let mut current_block_17: u64;
    let mut c: libc::c_char = *s;
    // do we need special processing?
    // standard escapes + \s for space and \N for \0
    // \c inhibits terminating \r for commands and is noop for expects
    if '\\' as i32 == c as libc::c_int {
      s = s.offset(1);
      c = *s;
      if c != 0 {
        if 'c' as i32 == c as libc::c_int {
          *nocr = 1i32;
          current_block_17 = 4372395669998863707;
        } else {
          if 'N' as i32 == c as libc::c_int {
            c = '\u{0}' as i32 as libc::c_char
          } else if 's' as i32 == c as libc::c_int {
            c = ' ' as i32 as libc::c_char
          // ^A becomes \001, ^B -- \002 and so on...
          // unescape leading dash only
          // TODO: and only for expect, not command string
          } else if !('-' as i32 == c as libc::c_int && start.offset(1) == s) {
            c = bb_process_escape_sequence(
              &mut s as *mut *mut libc::c_char as *mut *const libc::c_char,
            );
            s = s.offset(-1)
          }
          current_block_17 = 11042950489265723346;
        }
      } else {
        current_block_17 = 11042950489265723346;
      }
    } else {
      if '^' as i32 == c as libc::c_int {
        s = s.offset(1);
        c = (*s as libc::c_int - '@' as i32) as libc::c_char
      }
      current_block_17 = 11042950489265723346;
    }
    match current_block_17 {
      11042950489265723346 => {
        // put unescaped char
        let fresh0 = p;
        p = p.offset(1);
        *fresh0 = c
      }
      _ => {}
    }
    // next char
    s = s.offset(1)
  }
  *p = '\u{0}' as i32 as libc::c_char;
  return p.wrapping_offset_from(start) as libc::c_long as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn chat_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut current_block: u64;
  let mut record_fd: libc::c_int = -1i32;
  let mut echo: bool = 0i32 != 0;
  // collection of device replies which cause unconditional termination
  let mut aborts: *mut llist_t = 0 as *mut llist_t;
  // inactivity period
  let mut timeout: libc::c_int = 45i32 * 1000i32;
  // maximum length of abort string
  let mut max_abort_len: size_t = 0i32 as size_t;
  // directive names
  // make x* functions fail with correct exitcode
  xfunc_error_retval = ERR_IO as libc::c_int as u8;
  // trap vanilla signals to prevent process from being killed suddenly
  bb_signals(
    0i32 + (1i32 << 1i32) + (1i32 << 2i32) + (1i32 << 15i32) + (1i32 << 13i32),
    Some(signal_handler as unsafe extern "C" fn(_: libc::c_int) -> ()),
  );
  getopt32(argv, b"vVsSE\x00" as *const u8 as *const libc::c_char);
  argv = argv.offset(optind as isize);
  // handle chat expect-send pairs
  while !(*argv).is_null() {
    /* while (*argv) */
    // directive given? process it
    let mut key: libc::c_int = index_in_strings(
      b"HANGUP\x00ABORT\x00CLR_ABORT\x00TIMEOUT\x00ECHO\x00SAY\x00RECORD\x00\x00" as *const u8
        as *const libc::c_char,
      *argv,
    );
    if key >= 0i32 {
      let mut onoff: bool = false;
      // ordinary expect-send pair!
      argv = argv.offset(1);
      let mut arg: *mut libc::c_char = *argv;
      if arg.is_null() {
        bb_show_usage();
      }
      onoff = 0i32 != strcmp(b"OFF\x00" as *const u8 as *const libc::c_char, arg);
      if DIR_HANGUP as libc::c_int == key {
        // cache directive value
        // OFF -> 0, anything else -> 1
        // process directive
        // turn SIGHUP on/off
        signal(
          1i32,
          if onoff as libc::c_int != 0 {
            Some(signal_handler as unsafe extern "C" fn(_: libc::c_int) -> ())
          } else {
            ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(1i32 as libc::intptr_t)
          },
        );
      } else if DIR_ABORT as libc::c_int == key {
        // append the string to abort conditions
        let mut len: size_t = strlen(arg);
        if len > max_abort_len {
          max_abort_len = len
        }
        llist_add_to_end(&mut aborts, arg as *mut libc::c_void);
      } else if DIR_CLR_ABORT as libc::c_int == key {
        let mut l: *mut llist_t = 0 as *mut llist_t;
        // remove the string from abort conditions
        // N.B. gotta refresh maximum length too...
        max_abort_len = 0i32 as size_t;
        l = aborts;
        while !l.is_null() {
          let mut len_0: size_t = strlen((*l).data);
          if strcmp(arg, (*l).data) == 0i32 {
            llist_unlink(&mut aborts, l);
          } else if len_0 > max_abort_len {
            max_abort_len = len_0
          }
          l = (*l).link
        }
      } else if DIR_TIMEOUT as libc::c_int == key {
        // set new timeout
        // -1 means OFF
        timeout = atoi(arg) * 1000i32;
        // 0 means default
        // >0 means value in msecs
        if timeout == 0 {
          timeout = 45i32 * 1000i32
        }
      } else if DIR_ECHO as libc::c_int == key {
        // turn echo on/off
        // N.B. echo means dumping device input/output to stderr
        echo = onoff
      } else if DIR_RECORD as libc::c_int == key {
        // turn record on/off
        // N.B. record means dumping device input to a file
        // close previous record_fd
        if record_fd > 0i32 {
          close(record_fd);
        }
        // N.B. do we have to die here on open error?
        record_fd = if onoff as libc::c_int != 0 {
          xopen(arg, 0o1i32 | 0o100i32 | 0o1000i32)
        } else {
          -1i32
        }
      } else if DIR_SAY as libc::c_int == key {
        // just print argument verbatim
        // TODO: should we use full_write() to avoid unistd/stdio conflict?
        bb_simple_error_msg(arg);
      }
      argv = argv.offset(1)
    } else {
      // next, please!
      //-----------------------
      // do expect
      //-----------------------
      let mut expect_len: libc::c_int = 0;
      let mut buf_len: size_t = 0i32 as size_t;
      let mut max_len: size_t = max_abort_len;
      let mut pfd: pollfd = pollfd {
        fd: 0,
        events: 0,
        revents: 0,
      };
      let mut nofail: libc::c_int = 0i32;
      let fresh1 = argv;
      argv = argv.offset(1);
      let mut expect: *mut libc::c_char = *fresh1;
      // sanity check: shall we really expect something?
      if !expect.is_null() {
        // if expect starts with -
        if '-' as i32 == *expect as libc::c_int {
          // swallow -
          expect = expect.offset(1);
          // and enter nofail mode
          nofail += 1
        }
        // test behaviour with a small buffer
        // expand escape sequences in expect
        expect_len = unescape(expect, &mut expect_len) as libc::c_int;
        if expect_len as libc::c_ulong > max_len {
          max_len = expect_len as size_t
        }
        // sanity check:
        // we should expect more than nothing but not more than input buffer
        // TODO: later we'll get rid of fixed-size buffer
        if !(expect_len == 0) {
          if max_len >= COMMON_BUFSIZE as libc::c_int as libc::c_ulong {
            bb_got_signal = ERR_MEM as libc::c_int as smallint
          } else {
            // get reply
            pfd.fd = 0i32; /* while (have data) */
            pfd.events = 0x1i32 as libc::c_short;
            's_332: loop {
              if !(bb_got_signal == 0
                && poll(&mut pfd, 1i32 as nfds_t, timeout) > 0i32
                && pfd.revents as libc::c_int & 0x1i32 != 0)
              {
                current_block = 11869735117417356968;
                break;
              }
              let mut l_0: *mut llist_t = 0 as *mut llist_t;
              let mut delta: ssize_t = 0;
              // read next char from device
              if safe_read(
                0i32,
                bb_common_bufsiz1.as_mut_ptr().offset(buf_len as isize) as *mut libc::c_void,
                1i32 as size_t,
              ) > 0
              {
                // dump device input if RECORD fname
                if record_fd > 0i32 {
                  full_write(
                    record_fd,
                    bb_common_bufsiz1.as_mut_ptr().offset(buf_len as isize) as *const libc::c_void,
                    1i32 as size_t,
                  );
                }
                // dump device input if ECHO ON
                if echo {
                  //						if (buf[buf_len] < ' ') {
                  //							full_write(STDERR_FILENO, "^", 1);
                  //							buf[buf_len] += '@';
                  //						}
                  full_write(
                    2i32,
                    bb_common_bufsiz1.as_mut_ptr().offset(buf_len as isize) as *const libc::c_void,
                    1i32 as size_t,
                  );
                }
                buf_len = buf_len.wrapping_add(1);
                // move input frame if we've reached higher bound
                if buf_len > COMMON_BUFSIZE as libc::c_int as libc::c_ulong {
                  memmove(
                    bb_common_bufsiz1.as_mut_ptr() as *mut libc::c_void,
                    bb_common_bufsiz1
                      .as_mut_ptr()
                      .offset(buf_len as isize)
                      .offset(-(max_len as isize)) as *const libc::c_void,
                    max_len,
                  );
                  buf_len = max_len
                }
              }
              // N.B. rule of thumb: values being looked for can
              // be found only at the end of input buffer
              // this allows to get rid of strstr() and memmem()
              // TODO: make expect and abort strings processed uniformly
              // abort condition is met? -> bail out
              l_0 = aborts;
              bb_got_signal = ERR_ABORT as libc::c_int as smallint;
              while !l_0.is_null() {
                let mut len_1: size_t = strlen((*l_0).data);
                delta = buf_len.wrapping_sub(len_1) as ssize_t;
                if delta >= 0
                  && memcmp(
                    bb_common_bufsiz1.as_mut_ptr().offset(delta as isize) as *const libc::c_void,
                    (*l_0).data as *const libc::c_void,
                    len_1,
                  ) == 0
                {
                  current_block = 9816136611746998877;
                  break 's_332;
                }
                l_0 = (*l_0).link;
                bb_got_signal += 1
              }
              bb_got_signal = ERR_OK as libc::c_int as smallint;
              // expected reply received? -> goto next command
              delta = buf_len.wrapping_sub(expect_len as libc::c_ulong) as ssize_t;
              if delta >= 0
                && memcmp(
                  bb_common_bufsiz1.as_mut_ptr().offset(delta as isize) as *const libc::c_void,
                  expect as *const libc::c_void,
                  expect_len as libc::c_ulong,
                ) == 0
              {
                current_block = 9816136611746998877;
                break;
              }
            }
            match current_block {
              9816136611746998877 => {}
              _ => {
                // device timed out or unexpected reply received
                bb_got_signal = ERR_TIMEOUT as libc::c_int as smallint
              }
            }
          }
        }
      }
      // on success and when in nofail mode
      // we should skip following subsend-subexpect pairs
      if nofail != 0 {
        if bb_got_signal == 0 {
          // find last send before non-dashed expect
          while !(*argv).is_null()
            && !(*argv.offset(1)).is_null()
            && '-' as i32 == *(*argv.offset(1)).offset(0) as libc::c_int
          {
            argv = argv.offset(2)
          }
          // skip the pair
          // N.B. do we really need this?!
          let fresh2 = argv;
          argv = argv.offset(1);
          if (*fresh2).is_null() || {
            let fresh3 = argv;
            argv = argv.offset(1);
            (*fresh3).is_null()
          } {
            break;
          }
        }
        // nofail mode also clears all but IO errors (or signals)
        if ERR_IO as libc::c_int != bb_got_signal as libc::c_int {
          bb_got_signal = ERR_OK as libc::c_int as smallint
        }
      }
      // bail out unless we expected successfully
      if bb_got_signal != 0 {
        break;
      }
      //-----------------------
      // do send
      //-----------------------
      if (*argv).is_null() {
        continue; // inhibit terminating command with \r
      } // loaded command
      let mut nocr: libc::c_int = 0i32;
      let mut loaded: *mut libc::c_char = 0 as *mut libc::c_char;
      let mut len_2: size_t = 0;
      let fresh4 = argv;
      argv = argv.offset(1);
      let mut buf_0: *mut libc::c_char = *fresh4;
      // if command starts with @
      // load "real" command from file named after @
      if '@' as i32 == *buf_0 as libc::c_int {
        // skip the @ and any following white-space
        buf_0 = buf_0.offset(1);
        trim(buf_0);
        loaded = xmalloc_xopen_read_close(buf_0, 0 as *mut size_t) as *mut libc::c_char;
        buf_0 = loaded
      }
      // expand escape sequences in command
      len_2 = unescape(buf_0, &mut nocr);
      // send command
      alarm(timeout as libc::c_uint); /* while (can write) */
      pfd.fd = 1i32;
      pfd.events = 0x4i32 as libc::c_short;
      while len_2 != 0
        && bb_got_signal == 0
        && poll(&mut pfd, 1i32 as nfds_t, -1i32) > 0i32
        && pfd.revents as libc::c_int & 0x4i32 != 0
      {
        // "\\d" means 1 sec delay, "\\p" means 0.01 sec delay
        // "\\K" means send BREAK
        let mut c: libc::c_char = *buf_0;
        if '\\' as i32 == c as libc::c_int {
          buf_0 = buf_0.offset(1);
          c = *buf_0;
          if 'd' as i32 == c as libc::c_int {
            sleep(1i32 as libc::c_uint);
            len_2 = len_2.wrapping_sub(1);
            continue;
          } else if 'p' as i32 == c as libc::c_int {
            usleep(10000i32 as useconds_t);
            len_2 = len_2.wrapping_sub(1);
            continue;
          } else if 'K' as i32 == c as libc::c_int {
            tcsendbreak(1i32, 0i32);
            len_2 = len_2.wrapping_sub(1);
            continue;
          } else {
            buf_0 = buf_0.offset(-1)
          }
        }
        if safe_write(1i32, buf_0 as *const libc::c_void, 1i32 as size_t) != 1 {
          break;
        }
        len_2 = len_2.wrapping_sub(1);
        buf_0 = buf_0.offset(1)
      }
      alarm(0i32 as libc::c_uint);
      // report I/O error if there still exists at least one non-sent char
      if len_2 != 0 {
        bb_got_signal = ERR_IO as libc::c_int as smallint
      }
      // free loaded command (if any)
      if !loaded.is_null() {
        free(loaded as *mut libc::c_void);
      } else if nocr == 0 {
        xwrite(
          1i32,
          b"\r\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
          1i32 as size_t,
        );
      }
      // or terminate command with \r (if not inhibited)
      // bail out unless we sent command successfully
      if bb_got_signal != 0 {
        break;
      }
      /* if (*argv) */
    }
  }
  return bb_got_signal as libc::c_int;
}
