use crate::libbb::ptr_to_globals::bb_errno;
use crate::libbb::xfuncs_printf::xmalloc;
use crate::librb::bb_uidgid_t;
use crate::librb::size_t;
use crate::librb::smallint;
use libc;
use libc::closedir;
use libc::fclose;
use libc::free;
use libc::fscanf;
use libc::gid_t;
use libc::kill;
use libc::passwd;
use libc::pid_t;
use libc::printf;
use libc::puts;
use libc::readdir;
use libc::setsid;
use libc::sprintf;
use libc::ssize_t;
use libc::stat;
use libc::strchr;
use libc::strcmp;
use libc::strcpy;
use libc::strrchr;
use libc::uid_t;
use libc::FILE;
extern "C" {

  #[no_mangle]
  fn execvp(__file: *const libc::c_char, __argv: *const *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn _exit(_: libc::c_int) -> !;

  #[no_mangle]
  fn vfork() -> libc::c_int;
  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn getpriority(__which: __priority_which_t, __who: id_t) -> libc::c_int;
  #[no_mangle]
  fn setpriority(__which: __priority_which_t, __who: id_t, __prio: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn setgroups(__n: size_t, __groups: *const gid_t) -> libc::c_int;

  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xopendir(path: *const libc::c_char) -> *mut DIR;
  #[no_mangle]
  fn xmalloc_readlink(path: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xsetgid(gid: gid_t);
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  #[no_mangle]
  fn open_read_close(
    filename: *const libc::c_char,
    buf: *mut libc::c_void,
    maxsz: size_t,
  ) -> ssize_t;
  #[no_mangle]
  fn fopen_for_read(path: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn xatoi_range(str: *const libc::c_char, l: libc::c_int, u: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn bb_strtou(
    arg: *const libc::c_char,
    endp: *mut *mut libc::c_char,
    base: libc::c_int,
  ) -> libc::c_uint;
  #[no_mangle]
  fn xuname2uid(name: *const libc::c_char) -> libc::c_long;
  #[no_mangle]
  fn parse_chown_usergroup_or_die(u: *mut bb_uidgid_t, user_group: *mut libc::c_char);
  #[no_mangle]
  fn xgetpwuid(uid: uid_t) -> *mut passwd;
  #[no_mangle]
  fn bb_daemonize_or_rexec(flags: libc::c_int);
  #[no_mangle]
  static mut option_mask32: u32;
  #[no_mangle]
  fn getopt32long(
    argv: *mut *mut libc::c_char,
    optstring: *const libc::c_char,
    longopts: *const libc::c_char,
    _: ...
  ) -> u32;
  #[no_mangle]
  fn write_pidfile(path: *const libc::c_char);
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_perror_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_perror_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn get_signum(name: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn change_identity(pw: *const passwd);
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
}

pub type __id_t = libc::c_uint;

use libc::dirent;
use libc::DIR;
pub type id_t = __id_t;

pub type __priority_which = libc::c_uint;
pub const PRIO_USER: __priority_which = 2;
pub const PRIO_PGRP: __priority_which = 1;
pub const PRIO_PROCESS: __priority_which = 0;
pub type __priority_which_t = __priority_which;
pub type C2RustUnnamed = libc::c_uint;
pub const DAEMON_ONLY_SANITIZE: C2RustUnnamed = 8;
pub const DAEMON_CLOSE_EXTRA_FDS: C2RustUnnamed = 4;
pub const DAEMON_DEVNULL_STDIO: C2RustUnnamed = 2;
pub const DAEMON_CHDIR_ROOT: C2RustUnnamed = 1;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const COMM_LEN: C2RustUnnamed_0 = 16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub found_procs: *mut pid_list,
  pub userspec: *mut libc::c_char,
  pub cmdname: *mut libc::c_char,
  pub execname: *mut libc::c_char,
  pub pidfile: *mut libc::c_char,
  pub execname_cmpbuf: *mut libc::c_char,
  pub execname_sizeof: libc::c_uint,
  pub user_id: libc::c_int,
  pub signal_nr: smallint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pid_list {
  pub next: *mut pid_list,
  pub pid: pid_t,
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub const OPT_NICELEVEL: C2RustUnnamed_1 = 32768;
pub const OPT_VERBOSE: C2RustUnnamed_1 = 16384;
pub const OPT_OKNODO: C2RustUnnamed_1 = 8192;
pub const OPT_p: C2RustUnnamed_1 = 4096;
pub const OPT_x: C2RustUnnamed_1 = 2048;
pub const OPT_c: C2RustUnnamed_1 = 1024;
pub const OPT_u: C2RustUnnamed_1 = 512;
pub const OPT_s: C2RustUnnamed_1 = 256;
pub const OPT_n: C2RustUnnamed_1 = 128;
pub const OPT_a: C2RustUnnamed_1 = 64;
pub const OPT_MAKEPID: C2RustUnnamed_1 = 32;
pub const OPT_TEST: C2RustUnnamed_1 = 16;
pub const OPT_QUIET: C2RustUnnamed_1 = 8;
pub const OPT_BACKGROUND: C2RustUnnamed_1 = 4;
pub const CTX_START: C2RustUnnamed_1 = 2;
pub const CTX_STOP: C2RustUnnamed_1 = 1;
unsafe extern "C" fn pid_is_exec(mut pid: pid_t) -> libc::c_int {
  let mut bytes: ssize_t = 0;
  let mut buf: [libc::c_char; 29] = [0; 29];
  let mut procname: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut exelink: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut match_0: libc::c_int = 0;
  procname = buf
    .as_mut_ptr()
    .offset(sprintf(
      buf.as_mut_ptr(),
      b"/proc/%u/exe\x00" as *const u8 as *const libc::c_char,
      pid as libc::c_uint,
    ) as isize)
    .offset(-3);
  exelink = xmalloc_readlink(buf.as_mut_ptr());
  match_0 = (!exelink.is_null()
    && strcmp(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).execname,
      exelink,
    ) == 0) as libc::c_int;
  free(exelink as *mut libc::c_void);
  if match_0 != 0 {
    return match_0;
  }
  strcpy(procname, b"cmdline\x00" as *const u8 as *const libc::c_char);
  bytes = open_read_close(
    buf.as_mut_ptr(),
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).execname_cmpbuf as *mut libc::c_void,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).execname_sizeof as size_t,
  );
  if bytes > 0 {
    *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .execname_cmpbuf
      .offset(bytes as isize) = '\u{0}' as i32 as libc::c_char;
    return (strcmp(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).execname,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).execname_cmpbuf,
    ) == 0) as libc::c_int;
  }
  return 0;
}
unsafe extern "C" fn pid_is_name(mut pid: pid_t) -> libc::c_int {
  /* /proc/PID/stat is "PID (comm_15_bytes_max) ..." */
  let mut buf: [libc::c_char; 32] = [0; 32]; /* should be enough */
  let mut p: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>(); /* paranoia */
  let mut pe: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  sprintf(
    buf.as_mut_ptr(),
    b"/proc/%u/stat\x00" as *const u8 as *const libc::c_char,
    pid as libc::c_uint,
  );
  if open_read_close(
    buf.as_mut_ptr(),
    buf.as_mut_ptr() as *mut libc::c_void,
    (::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
      .wrapping_sub(1i32 as libc::c_ulong),
  ) < 0
  {
    return 0;
  }
  buf[(::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
    .wrapping_sub(1i32 as libc::c_ulong) as usize] = '\u{0}' as i32 as libc::c_char;
  p = strchr(buf.as_mut_ptr(), '(' as i32);
  if p.is_null() {
    return 0;
  }
  p = p.offset(1);
  pe = strrchr(p, ')' as i32);
  if pe.is_null() {
    return 0;
  }
  *pe = '\u{0}' as i32 as libc::c_char;
  /* we require comm to match and to not be truncated */
  /* in Linux, if comm is 15 chars, it may be a truncated
   * name, so we don't allow that to match */
  if strlen(p) >= (COMM_LEN as libc::c_int - 1i32) as libc::c_ulong {
    /* COMM_LEN is 16 */
    return 0;
  }
  return (strcmp(
    p,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).cmdname,
  ) == 0) as libc::c_int;
}
unsafe extern "C" fn pid_is_user(mut pid: libc::c_int) -> libc::c_int {
  let mut sb: stat = std::mem::zeroed();
  let mut buf: [libc::c_char; 19] = [0; 19];
  sprintf(
    buf.as_mut_ptr(),
    b"/proc/%u\x00" as *const u8 as *const libc::c_char,
    pid as libc::c_uint,
  );
  if stat(buf.as_mut_ptr(), &mut sb) != 0 {
    return 0;
  }
  return (sb.st_uid == (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).user_id as uid_t)
    as libc::c_int;
}
unsafe extern "C" fn check(mut pid: libc::c_int) {
  let mut p: *mut pid_list = std::ptr::null_mut();
  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .execname
    .is_null()
    && pid_is_exec(pid) == 0
  {
    return;
  }
  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .cmdname
    .is_null()
    && pid_is_name(pid) == 0
  {
    return;
  }
  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .userspec
    .is_null()
    && pid_is_user(pid) == 0
  {
    return;
  }
  p = xmalloc(::std::mem::size_of::<pid_list>() as libc::c_ulong) as *mut pid_list;
  (*p).next = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).found_procs;
  (*p).pid = pid;
  let ref mut fresh0 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).found_procs;
  *fresh0 = p;
}
unsafe extern "C" fn do_pidfile() {
  let mut f: *mut FILE = std::ptr::null_mut();
  let mut pid: libc::c_uint = 0;
  f = fopen_for_read((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).pidfile);
  if !f.is_null() {
    if fscanf(
      f,
      b"%u\x00" as *const u8 as *const libc::c_char,
      &mut pid as *mut libc::c_uint,
    ) == 1i32
    {
      check(pid as libc::c_int);
    }
    fclose(f);
  } else if *bb_errno != 2i32 {
    bb_perror_msg_and_die(
      b"open pidfile %s\x00" as *const u8 as *const libc::c_char,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).pidfile,
    );
  };
}
unsafe extern "C" fn do_procinit() {
  let mut procdir: *mut DIR = std::ptr::null_mut();
  let mut entry: *mut dirent = std::ptr::null_mut();
  let mut pid: libc::c_int = 0;
  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .pidfile
    .is_null()
  {
    do_pidfile();
    return;
  }
  procdir = xopendir(b"/proc\x00" as *const u8 as *const libc::c_char);
  pid = 0;
  loop
  /* Stale entry, process has died after opendir */
  {
    *bb_errno = 0; /* clear any previous error */
    entry = readdir(procdir);
    // TODO: this check is too generic, it's better
    // to check for exact errno(s) which mean that we got stale entry
    if *bb_errno != 0 {
      continue;
    }
    if entry.is_null() {
      break;
    }
    pid = bb_strtou(
      (*entry).d_name.as_mut_ptr(),
      0 as *mut *mut libc::c_char,
      10i32,
    ) as libc::c_int;
    if !(*bb_errno != 0) {
      check(pid);
    }
  }
  closedir(procdir);
  if pid == 0 {
    bb_simple_error_msg_and_die(
      b"nothing in /proc - not mounted?\x00" as *const u8 as *const libc::c_char,
    );
  };
}
unsafe extern "C" fn do_stop() -> libc::c_int {
  let mut current_block: u64;
  let mut what: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut p: *mut pid_list = std::ptr::null_mut();
  let mut killed: libc::c_int = 0;
  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .cmdname
    .is_null()
  {
    if 0 == 0 {
      what = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).cmdname
    }
  } else if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .execname
    .is_null()
  {
    if 0 == 0 {
      what = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).execname
    }
  } else if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .pidfile
    .is_null()
  {
    what = xasprintf(
      b"process in pidfile \'%s\'\x00" as *const u8 as *const libc::c_char,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).pidfile,
    )
  } else if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .userspec
    .is_null()
  {
    what = xasprintf(
      b"process(es) owned by \'%s\'\x00" as *const u8 as *const libc::c_char,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).userspec,
    )
  } else {
    bb_simple_error_msg_and_die(
      b"internal error, please report\x00" as *const u8 as *const libc::c_char,
    );
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .found_procs
    .is_null()
  {
    if option_mask32 & OPT_QUIET as libc::c_int as libc::c_uint == 0 {
      printf(
        b"no %s found; none killed\n\x00" as *const u8 as *const libc::c_char,
        what,
      );
    }
    killed = -1i32
  } else {
    p = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).found_procs;
    loop {
      if p.is_null() {
        current_block = 11385396242402735691;
        break;
      }
      if kill(
        (*p).pid,
        if option_mask32 & OPT_TEST as libc::c_int as libc::c_uint != 0 {
          0
        } else {
          (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).signal_nr as libc::c_int
        },
      ) == 0
      {
        killed += 1
      } else {
        bb_perror_msg(
          b"warning: killing process %u\x00" as *const u8 as *const libc::c_char,
          (*p).pid as libc::c_uint,
        );
        (*p).pid = 0;
        if option_mask32 & OPT_TEST as libc::c_int as libc::c_uint != 0 {
          /* Example: -K --test --pidfile PIDFILE detected
           * that PIDFILE's pid doesn't exist */
          killed = -1i32;
          current_block = 1356832168064818221;
          break;
        }
      }
      p = (*p).next
    }
    match current_block {
      1356832168064818221 => {}
      _ => {
        if option_mask32 & OPT_QUIET as libc::c_int as libc::c_uint == 0 && killed != 0 {
          printf(
            b"stopped %s (pid\x00" as *const u8 as *const libc::c_char,
            what,
          );
          p = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).found_procs;
          while !p.is_null() {
            if (*p).pid != 0 {
              printf(
                b" %u\x00" as *const u8 as *const libc::c_char,
                (*p).pid as libc::c_uint,
              );
            }
            p = (*p).next
          }
          puts(b")\x00" as *const u8 as *const libc::c_char);
        }
      }
    }
  }
  return killed;
}
static mut start_stop_daemon_longopts: [libc::c_char; 156] = [
  115, 116, 111, 112, 0, 0, 75, 115, 116, 97, 114, 116, 0, 0, 83, 98, 97, 99, 107, 103, 114, 111,
  117, 110, 100, 0, 0, 98, 113, 117, 105, 101, 116, 0, 0, 113, 116, 101, 115, 116, 0, 0, 116, 109,
  97, 107, 101, 45, 112, 105, 100, 102, 105, 108, 101, 0, 0, 109, 111, 107, 110, 111, 100, 111, 0,
  0, 111, 118, 101, 114, 98, 111, 115, 101, 0, 0, 118, 110, 105, 99, 101, 108, 101, 118, 101, 108,
  0, 1, 78, 115, 116, 97, 114, 116, 97, 115, 0, 1, 97, 110, 97, 109, 101, 0, 1, 110, 115, 105, 103,
  110, 97, 108, 0, 1, 115, 117, 115, 101, 114, 0, 1, 117, 99, 104, 117, 105, 100, 0, 1, 99, 101,
  120, 101, 99, 0, 1, 120, 112, 105, 100, 102, 105, 108, 101, 0, 1, 112, 114, 101, 116, 114, 121,
  0, 1, 82, 0,
];
#[no_mangle]
pub unsafe extern "C" fn start_stop_daemon_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut opt: libc::c_uint = 0;
  let mut signame: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut startas: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut chuid: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  //	char *retry_arg = NULL;
  //	int retries = -1;
  let mut opt_N: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).user_id = -1i32;
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).signal_nr = 15i32 as smallint;
  opt = getopt32long(
    argv,
    b"^KSbqtma:n:s:u:c:x:p:ovN:R:\x00K:S:K--S:S--K:m?p:K?xpunq-v\x00" as *const u8
      as *const libc::c_char,
    start_stop_daemon_longopts.as_ptr(),
    &mut startas as *mut *mut libc::c_char,
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).cmdname as *mut *mut libc::c_char,
    &mut signame as *mut *mut libc::c_char,
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).userspec as *mut *mut libc::c_char,
    &mut chuid as *mut *mut libc::c_char,
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).execname as *mut *mut libc::c_char,
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).pidfile as *mut *mut libc::c_char,
    &mut opt_N as *mut *mut libc::c_char,
    0 as *mut libc::c_void,
  );
  if opt & OPT_s as libc::c_int as libc::c_uint != 0 {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).signal_nr = get_signum(signame) as smallint;
    if ((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).signal_nr as libc::c_int) < 0 {
      bb_show_usage();
    }
  }
  //argc -= optind;
  argv = argv.offset(optind as isize);
  // ARGS contains zeroth arg if -x/-a is not given, else it starts with 1st arg.
  // These will try to execute "[/bin/]sleep 5":
  // "start-stop-daemon -S               -- sleep 5"
  // "start-stop-daemon -S -x /bin/sleep -- 5"
  // "start-stop-daemon -S -a sleep      -- 5"
  // NB: -n option does _not_ behave in this way: this will try to execute "5":
  // "start-stop-daemon -S -n sleep      -- 5"
  if opt & CTX_START as libc::c_int as libc::c_uint != 0 {
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .execname
      .is_null()
    {
      /* -x is not given */
      let ref mut fresh1 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).execname;
      *fresh1 = startas;
      if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .execname
        .is_null()
      {
        /* neither -x nor -a is given */
        let ref mut fresh2 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).execname;
        *fresh2 = *argv.offset(0);
        if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
          .execname
          .is_null()
        {
          bb_show_usage();
        }
        argv = argv.offset(1)
      }
    }
    if startas.is_null() {
      /* -a is not given: use -x EXECUTABLE or argv[0] */
      startas = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).execname
    }
    argv = argv.offset(-1);
    *argv = startas
  }
  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .execname
    .is_null()
  {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).execname_sizeof =
      strlen((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).execname)
        .wrapping_add(1i32 as libc::c_ulong) as libc::c_uint;
    let ref mut fresh3 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).execname_cmpbuf;
    *fresh3 = xmalloc(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .execname_sizeof
        .wrapping_add(1i32 as libc::c_uint) as size_t,
    ) as *mut libc::c_char
  }
  //	IF_FEATURE_START_STOP_DAEMON_FANCY(
  //		if (retry_arg)
  //			retries = xatoi_positive(retry_arg);
  //	)
  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .userspec
    .is_null()
  {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).user_id = bb_strtou(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).userspec,
      0 as *mut *mut libc::c_char,
      10i32,
    ) as libc::c_int;
    if *bb_errno != 0 {
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).user_id =
        xuname2uid((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).userspec) as libc::c_int
    }
  }
  /* Both start and stop need to know current processes */
  do_procinit();
  if opt & CTX_STOP as libc::c_int as libc::c_uint != 0 {
    let mut i: libc::c_int = do_stop();
    return if opt & OPT_OKNODO as libc::c_int as libc::c_uint != 0 {
      0
    } else {
      (i <= 0) as libc::c_int
    };
  }
  /* else: CTX_START (-S). execname can't be NULL. */
  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .found_procs
    .is_null()
  {
    if option_mask32 & OPT_QUIET as libc::c_int as libc::c_uint == 0 {
      printf(
        b"%s is already running\n\x00" as *const u8 as *const libc::c_char,
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).execname,
      );
    }
    return (opt & OPT_OKNODO as libc::c_int as libc::c_uint == 0) as libc::c_int;
  }
  if opt & OPT_BACKGROUND as libc::c_int as libc::c_uint != 0 {
    /* Daemons usually call bb_daemonize_or_rexec(), but SSD can do
     * without: SSD is not itself a daemon, it _execs_ a daemon.
     * The usual NOMMU problem of "child can't run indefinitely,
     * it must exec" does not bite us: we exec anyway.
     *
     * bb_daemonize(DAEMON_DEVNULL_STDIO | DAEMON_CLOSE_EXTRA_FDS | DAEMON_DOUBLE_FORK)
     * can be used on MMU systems, but use of vfork()
     * is preferable since we want to create pidfile
     * _before_ parent returns, and vfork() on Linux
     * ensures that (by blocking parent until exec in the child).
     */
    let mut pid: pid_t = {
      let mut bb__xvfork_pid: pid_t = vfork();
      if bb__xvfork_pid < 0 {
        bb_simple_perror_msg_and_die(b"vfork\x00" as *const u8 as *const libc::c_char);
      }
      bb__xvfork_pid
    };
    if pid != 0 {
      /* Parent */
      /* why _exit? the child may have changed the stack,
       * so "return 0" may do bad things
       */
      _exit(0i32);
    }
    /* Child */
    setsid(); /* detach from controlling tty */
    /* Redirect stdio to /dev/null, close extra FDs */
    bb_daemonize_or_rexec(
      DAEMON_DEVNULL_STDIO as libc::c_int + DAEMON_CLOSE_EXTRA_FDS as libc::c_int
        | DAEMON_ONLY_SANITIZE as libc::c_int,
    );
    /* On Linux, session leader can acquire ctty
     * unknowingly, by opening a tty.
     * Prevent this: stop being a session leader.
     */
    pid = {
      let mut bb__xvfork_pid: pid_t = vfork();
      if bb__xvfork_pid < 0 {
        bb_simple_perror_msg_and_die(b"vfork\x00" as *const u8 as *const libc::c_char);
      }
      bb__xvfork_pid
    };
    if pid != 0 {
      _exit(0i32);
    }
  }
  if opt & OPT_MAKEPID as libc::c_int as libc::c_uint != 0 {
    /* User wants _us_ to make the pidfile */
    write_pidfile((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).pidfile);
  }
  if opt & OPT_NICELEVEL as libc::c_int as libc::c_uint != 0 {
    /* Set process priority (must be before OPT_c) */
    let mut prio: libc::c_int = getpriority(PRIO_PROCESS, 0 as id_t)
      + xatoi_range(opt_N, (-2147483647i32 - 1i32) / 2i32, 2147483647i32 / 2i32);
    if setpriority(PRIO_PROCESS, 0 as id_t, prio) < 0 {
      bb_perror_msg_and_die(
        b"setpriority(%d)\x00" as *const u8 as *const libc::c_char,
        prio,
      );
    }
  }
  if opt & OPT_c as libc::c_int as libc::c_uint != 0 {
    let mut ugid: bb_uidgid_t = bb_uidgid_t { uid: 0, gid: 0 };
    parse_chown_usergroup_or_die(&mut ugid, chuid);
    if ugid.uid != -1i64 as uid_t {
      let mut pw: *mut passwd = xgetpwuid(ugid.uid);
      if ugid.gid != -1i64 as gid_t {
        (*pw).pw_gid = ugid.gid
      }
      /* initgroups, setgid, setuid: */
      change_identity(pw);
    } else if ugid.gid != -1i64 as gid_t {
      xsetgid(ugid.gid);
      setgroups(1i32 as size_t, &mut ugid.gid);
    }
  }
  /* Try:
   * strace -oLOG start-stop-daemon -S -x /bin/usleep -a qwerty 500000
   * should exec "/bin/usleep", but argv[0] should be "qwerty":
   */
  execvp(
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).execname,
    argv as *const *mut libc::c_char,
  );
  bb_perror_msg_and_die(
    b"can\'t execute \'%s\'\x00" as *const u8 as *const libc::c_char,
    startas,
  );
}
