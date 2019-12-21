use libc;
use libc::alarm;
use libc::close;
use libc::fclose;
use libc::isatty;
use libc::prctl;
use libc::printf;
use libc::puts;
extern "C" {

  #[no_mangle]
  fn vfork() -> libc::c_int;

  #[no_mangle]
  static mut stdin: *mut FILE;

  #[no_mangle]
  static mut stdout: *mut FILE;

  #[no_mangle]
  fn fread(__ptr: *mut libc::c_void, __size: size_t, __n: size_t, __stream: *mut FILE) -> size_t;

  #[no_mangle]
  fn fwrite(__ptr: *const libc::c_void, __size: size_t, __n: size_t, __s: *mut FILE) -> size_t;

  #[no_mangle]
  static ptr_to_globals: *mut globals;

  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;

  #[no_mangle]
  static bb_uuenc_tbl_base64: [libc::c_char; 0];

}

use crate::librb::size_t;
use libc::pid_t;
use libc::ssize_t;
use libc::FILE;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct globals {
  pub helper_pid: pid_t,
  pub timeout: libc::c_uint,
  pub verbose: libc::c_uint,
  pub opts: libc::c_uint,
  pub user: *mut libc::c_char,
  pub pass: *mut libc::c_char,
  pub fp0: *mut FILE,
  pub opt_charset: *mut libc::c_char,
}

pub type C2RustUnnamed = libc::c_uint;
pub const SRC_BUF_SIZE: C2RustUnnamed = 57;
/* This *MUST* be a multiple of 3 */
pub const DST_BUF_SIZE: C2RustUnnamed = 76;

/*
 * helper routines
 *
 * Copyright (C) 2008 by Vladimir Dronnikov <dronnikov@gmail.com>
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
// generic signal handler
unsafe extern "C" fn signal_handler(mut signo: libc::c_int) {
  if 14i32 == signo {
    crate::libbb::verror_msg::bb_simple_error_msg_and_die(
      b"timed out\x00" as *const u8 as *const libc::c_char,
    );
  }
  // SIGCHLD. reap zombies
  if crate::libbb::xfuncs::safe_waitpid((*ptr_to_globals).helper_pid, &mut signo, 1i32) > 0i32 {
    if ((signo & 0x7fi32) + 1i32) as libc::c_schar as libc::c_int >> 1i32 > 0i32 {
      crate::libbb::verror_msg::bb_error_msg_and_die(
        b"helper killed by signal %u\x00" as *const u8 as *const libc::c_char,
        signo & 0x7fi32,
      );
    }
    if signo & 0x7fi32 == 0i32 {
      (*ptr_to_globals).helper_pid = 0i32;
      if (signo & 0xff00i32) >> 8i32 != 0 {
        crate::libbb::verror_msg::bb_error_msg_and_die(
          b"helper exited (%u)\x00" as *const u8 as *const libc::c_char,
          (signo & 0xff00i32) >> 8i32,
        );
      }
    }
  };
}
//char FAST_FUNC *parse_url(char *url, char **user, char **pass);
#[no_mangle]
pub unsafe extern "C" fn launch_helper(mut argv: *mut *const libc::c_char) {
  // setup vanilla unidirectional pipes interchange
  let mut i: libc::c_int = 0;
  let mut pipes: [libc::c_int; 4] = [0; 4];
  crate::libbb::xfuncs_printf::xpipe(pipes.as_mut_ptr());
  crate::libbb::xfuncs_printf::xpipe(pipes.as_mut_ptr().offset(2));
  // NB: handler must be installed before vfork
  crate::libbb::signals::bb_signals(
    0i32 + (1i32 << 17i32) + (1i32 << 14i32),
    Some(signal_handler as unsafe extern "C" fn(_: libc::c_int) -> ()),
  ); // for parent:0, for child:2
  (*ptr_to_globals).helper_pid = {
    let mut bb__xvfork_pid: pid_t = vfork(); // 1 or 3 - closing one write end
    if bb__xvfork_pid < 0i32 {
      crate::libbb::perror_msg::bb_simple_perror_msg_and_die(
        b"vfork\x00" as *const u8 as *const libc::c_char,
      );
      // 2 or 0 - closing one read end
    } // 0 or 2 - using other read end
    bb__xvfork_pid
  }; // 3 or 1 - using other write end
  i = ((*ptr_to_globals).helper_pid == 0) as libc::c_int * 2i32;
  close(pipes[(i + 1i32) as usize]);
  close(pipes[(2i32 - i) as usize]);
  crate::libbb::xfuncs_printf::xmove_fd(pipes[i as usize], 0i32);
  crate::libbb::xfuncs_printf::xmove_fd(pipes[(3i32 - i) as usize], 1i32);
  // End result:
  // parent stdout [3] -> child stdin [2]
  // child stdout [1] -> parent stdin [0]
  if (*ptr_to_globals).helper_pid == 0 {
    // child
    // if parent dies, get SIGTERM
    prctl(1i32, 15i32, 0i32, 0i32, 0i32);
    // try to execute connection helper
    // NB: SIGCHLD & SIGALRM revert to SIG_DFL on exec
    crate::libbb::executable::BB_EXECVP_or_die(argv as *mut *mut libc::c_char);
  };
  // parent goes on
}
#[no_mangle]
pub unsafe extern "C" fn send_mail_command(
  mut fmt: *const libc::c_char,
  mut param: *const libc::c_char,
) -> *mut libc::c_char {
  let mut msg: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  if (*ptr_to_globals).timeout != 0 {
    alarm((*ptr_to_globals).timeout);
  }
  msg = fmt as *mut libc::c_char;
  if !fmt.is_null() {
    msg = crate::libbb::xfuncs_printf::xasprintf(fmt, param);
    if (*ptr_to_globals).verbose != 0 {
      crate::libbb::verror_msg::bb_error_msg(
        b"send:\'%s\'\x00" as *const u8 as *const libc::c_char,
        msg,
      );
    }
    printf(b"%s\r\n\x00" as *const u8 as *const libc::c_char, msg);
  }
  crate::libbb::xfuncs_printf::fflush_all();
  return msg;
}
// NB: parse_url can modify url[] (despite const), but only if '@' is there
/*
static char* FAST_FUNC parse_url(char *url, char **user, char **pass)
{
  // parse [user[:pass]@]host
  // return host
  char *s = strchr(url, '@');
  *user = *pass = NULL;
  if (s) {
    *s++ = '\0';
    *user = url;
    url = s;
    s = strchr(*user, ':');
    if (s) {
      *s++ = '\0';
      *pass = s;
    }
  }
  return url;
}
*/
unsafe extern "C" fn encode_n_base64(
  mut fname: *const libc::c_char,
  mut text: *const libc::c_char,
  mut len: size_t,
) {
  let mut src: [libc::c_char; 57] = [0; 57];
  let mut fp: *mut FILE = 0 as *mut FILE;
  fp = fp;
  let mut dst_buf: [libc::c_char; 77] = [0; 77];
  if !fname.is_null() {
    fp = if *fname.offset(0) as libc::c_int != '-' as i32 || *fname.offset(1) as libc::c_int != 0 {
      crate::libbb::wfopen::xfopen_for_read(fname)
    } else {
      stdin
    };
    text = src.as_mut_ptr()
  }
  loop {
    let mut size: size_t = 0;
    if !fname.is_null() {
      size = fread(
        text as *mut libc::c_char as *mut libc::c_void,
        1i32 as size_t,
        SRC_BUF_SIZE as libc::c_int as size_t,
        fp,
      );
      if (size as ssize_t) < 0 {
        crate::libbb::perror_msg::bb_simple_perror_msg_and_die(
          b"read error\x00" as *const u8 as *const libc::c_char,
        );
      }
    } else {
      size = len;
      if len > SRC_BUF_SIZE as libc::c_int as libc::c_ulong {
        size = SRC_BUF_SIZE as libc::c_int as size_t
      }
    }
    if size == 0 {
      break;
    }
    // encode the buffer we just read in
    crate::libbb::uuencode::bb_uuencode(
      dst_buf.as_mut_ptr(),
      text as *const libc::c_void,
      size as libc::c_int,
      bb_uuenc_tbl_base64.as_ptr(),
    );
    if !fname.is_null() {
      puts(b"\x00" as *const u8 as *const libc::c_char);
    } else {
      text = text.offset(size as isize);
      len = (len as libc::c_ulong).wrapping_sub(size) as size_t as size_t
    }
    fwrite(
      dst_buf.as_mut_ptr() as *const libc::c_void,
      1i32 as size_t,
      (4i32 as libc::c_ulong).wrapping_mul(
        size
          .wrapping_add(2i32 as libc::c_ulong)
          .wrapping_div(3i32 as libc::c_ulong),
      ),
      stdout,
    );
  }
  if !fname.is_null()
    && (*fname.offset(0) as libc::c_int != '-' as i32 || *fname.offset(1) as libc::c_int != 0)
  {
    fclose(fp);
  };
}
#[no_mangle]
pub unsafe extern "C" fn printstr_base64(mut text: *const libc::c_char) {
  encode_n_base64(0 as *const libc::c_char, text, strlen(text));
}
#[no_mangle]
pub unsafe extern "C" fn printbuf_base64(mut text: *const libc::c_char, mut len: libc::c_uint) {
  encode_n_base64(0 as *const libc::c_char, text, len as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn printfile_base64(mut fname: *const libc::c_char) {
  encode_n_base64(fname, 0 as *const libc::c_char, 0i32 as size_t);
}
/*
 * get username and password from a file descriptor
 */
#[no_mangle]
pub unsafe extern "C" fn get_cred_or_die(mut fd: libc::c_int) {
  if isatty(fd) != 0 {
    (*ptr_to_globals).user = crate::libbb::bb_askpass::bb_ask_noecho(
      fd,
      0i32,
      b"User: \x00" as *const u8 as *const libc::c_char,
    );
    (*ptr_to_globals).pass = crate::libbb::bb_askpass::bb_ask_noecho(
      fd,
      0i32,
      b"Password: \x00" as *const u8 as *const libc::c_char,
    )
  } else {
    (*ptr_to_globals).user =
      crate::libbb::read_printf::xmalloc_reads(fd, std::ptr::null_mut::<size_t>());
    (*ptr_to_globals).pass =
      crate::libbb::read_printf::xmalloc_reads(fd, std::ptr::null_mut::<size_t>())
  }
  if (*ptr_to_globals).user.is_null()
    || *(*ptr_to_globals).user == 0
    || (*ptr_to_globals).pass.is_null()
  {
    crate::libbb::verror_msg::bb_simple_error_msg_and_die(
      b"no username or password\x00" as *const u8 as *const libc::c_char,
    );
  };
}
