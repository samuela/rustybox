use libc;
extern "C" {
  #[no_mangle]
  fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn fchown(__fd: libc::c_int, __owner: __uid_t, __group: __gid_t) -> libc::c_int;
  #[no_mangle]
  fn execlp(__file: *const libc::c_char, __arg: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn getpid() -> __pid_t;
  #[no_mangle]
  fn getuid() -> __uid_t;
  #[no_mangle]
  fn vfork() -> libc::c_int;
  #[no_mangle]
  fn unlink(__name: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
  #[no_mangle]
  fn dprintf(__fd: libc::c_int, __fmt: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
  #[no_mangle]
  fn bb_copyfd_eof(fd1: libc::c_int, fd2: libc::c_int) -> off_t;
  #[no_mangle]
  fn close_on_exec_on(fd: libc::c_int);
  #[no_mangle]
  fn xchdir(path: *const libc::c_char);
  #[no_mangle]
  fn xopen3(pathname: *const libc::c_char, flags: libc::c_int, mode: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xopen_as_uid_gid(
    pathname: *const libc::c_char,
    flags: libc::c_int,
    u: uid_t,
    g: gid_t,
  ) -> libc::c_int;
  #[no_mangle]
  fn xrename(oldpath: *const libc::c_char, newpath: *const libc::c_char);
  #[no_mangle]
  fn xlseek(fd: libc::c_int, offset: off_t, whence: libc::c_int) -> off_t;
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  #[no_mangle]
  fn xgetpwnam(name: *const libc::c_char) -> *mut passwd;
  #[no_mangle]
  fn xgetpwuid(uid: uid_t) -> *mut passwd;
  #[no_mangle]
  fn wait4pid(pid: pid_t) -> libc::c_int;
  #[no_mangle]
  fn sanitize_env_if_suid() -> libc::c_int;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_perror_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_cat(argv: *mut *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn change_identity(pw: *const passwd);
  #[no_mangle]
  fn setup_environment(shell: *const libc::c_char, flags: libc::c_int, pw: *const passwd);
  #[no_mangle]
  static bb_msg_you_must_be_root: [libc::c_char; 0];
}
pub type __uint32_t = libc::c_uint;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type uint32_t = __uint32_t;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type off_t = __off64_t;
pub type pid_t = __pid_t;
use crate::librb::timespec;
use crate::librb::stat;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
  pub pw_name: *mut libc::c_char,
  pub pw_passwd: *mut libc::c_char,
  pub pw_uid: __uid_t,
  pub pw_gid: __gid_t,
  pub pw_gecos: *mut libc::c_char,
  pub pw_dir: *mut libc::c_char,
  pub pw_shell: *mut libc::c_char,
}
pub const OPT_e: C2RustUnnamed = 8;
pub const OPT_l: C2RustUnnamed = 4;
pub const OPT_ler: C2RustUnnamed = 28;
pub const OPT_u: C2RustUnnamed = 1;
pub const OPT_c: C2RustUnnamed = 2;
pub type C2RustUnnamed = libc::c_uint;
pub const OPT_r: C2RustUnnamed = 16;
unsafe extern "C" fn edit_file(mut pas: *const passwd, mut file: *const libc::c_char) {
  let mut ptr: *const libc::c_char = 0 as *const libc::c_char;
  let mut pid: pid_t = 0;
  pid = {
    let mut bb__xvfork_pid: pid_t = vfork();
    if bb__xvfork_pid < 0i32 {
      bb_simple_perror_msg_and_die(b"vfork\x00" as *const u8 as *const libc::c_char);
    }
    bb__xvfork_pid
  };
  if pid != 0 {
    /* parent */
    wait4pid(pid);
    return;
  }
  /* CHILD - change user and run editor */
  /* initgroups, setgid, setuid */
  change_identity(pas); /* -u USER */
  setup_environment((*pas).pw_shell, 1i32 << 0i32 | 1i32 << 2i32, pas);
  ptr = getenv(b"VISUAL\x00" as *const u8 as *const libc::c_char);
  if ptr.is_null() {
    ptr = getenv(b"EDITOR\x00" as *const u8 as *const libc::c_char);
    if ptr.is_null() {
      ptr = b"vi\x00" as *const u8 as *const libc::c_char
    }
  }
  execlp(ptr, ptr, file, 0 as *mut libc::c_void);
  bb_perror_msg_and_die(
    b"can\'t execute \'%s\'\x00" as *const u8 as *const libc::c_char,
    ptr,
  );
}
#[no_mangle]
pub unsafe extern "C" fn crontab_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut pas: *const passwd = 0 as *const passwd;
  let mut crontab_dir: *const libc::c_char =
    b"/var/spool/cron/crontabs\x00" as *const u8 as *const libc::c_char;
  let mut tmp_fname: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut new_fname: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut user_name: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut fd: libc::c_int = 0;
  let mut src_fd: libc::c_int = 0;
  let mut opt_ler: libc::c_int = 0;
  /* file [opts]     Replace crontab from file
   * - [opts]        Replace crontab from stdin
   * -u user         User
   * -c dir          Crontab directory
   * -l              List crontab for user
   * -e              Edit crontab for user
   * -r              Delete crontab for user
   * bbox also supports -d == -r, but most other crontab
   * implementations do not. Deprecated.
   */
  opt_ler = getopt32(
    argv,
    b"^u:c:lerd\x00?1:dr\x00" as *const u8 as *const libc::c_char,
    &mut user_name as *mut *mut libc::c_char,
    &mut crontab_dir as *mut *const libc::c_char,
  ) as libc::c_int;
  argv = argv.offset(optind as isize);
  if sanitize_env_if_suid() != 0 {
    /* Clears dangerous stuff, sets PATH */
    /* Run by non-root */
    if opt_ler & (OPT_u as libc::c_int | OPT_c as libc::c_int) != 0 {
      bb_simple_error_msg_and_die(bb_msg_you_must_be_root.as_ptr());
    }
  }
  if opt_ler & OPT_u as libc::c_int != 0 {
    pas = xgetpwnam(user_name)
  } else {
    pas = xgetpwuid(getuid())
  }
  /* From now on, keep only -l, -e, -r bits */
  opt_ler &= OPT_ler as libc::c_int;
  if opt_ler - 1i32 & opt_ler != 0 {
    /* more than one bit set? */
    bb_show_usage();
  }
  /* Read replacement file under user's UID/GID/group vector */
  src_fd = 0i32;
  if opt_ler == 0 {
    /* Replace? */
    if (*argv.offset(0)).is_null() {
      bb_show_usage();
    }
    if *(*argv.offset(0)).offset(0) as libc::c_int != '-' as i32
      || *(*argv.offset(0)).offset(1) as libc::c_int != 0
    {
      src_fd = xopen_as_uid_gid(*argv.offset(0), 0i32, (*pas).pw_uid, (*pas).pw_gid)
    }
  }
  /* cd to our crontab directory */
  xchdir(crontab_dir);
  tmp_fname = 0 as *mut libc::c_char;
  let mut current_block_48: u64;
  /* Handle requested operation */
  match opt_ler {
    4 => {
      /* switch */
      /* List */
      let mut args: [*mut libc::c_char; 2] = [(*pas).pw_name, 0 as *mut libc::c_char];
      return bb_cat(args.as_mut_ptr());
      /* list exits,
       * the rest go play with cron update file */
    }
    8 => {
      /* Edit */
      tmp_fname = xasprintf(
        b"%s.%u\x00" as *const u8 as *const libc::c_char,
        crontab_dir,
        getpid() as libc::c_uint,
      );
      /* No O_EXCL: we don't want to be stuck if earlier crontabs
       * were killed, leaving stale temp file behind */
      src_fd = xopen3(tmp_fname, 0o2i32 | 0o100i32 | 0o1000i32, 0o600i32); /* don't want editor to see this fd */
      fchown(src_fd, (*pas).pw_uid, (*pas).pw_gid);
      fd = open((*pas).pw_name, 0i32);
      if fd >= 0i32 {
        bb_copyfd_eof(fd, src_fd);
        close(fd);
        xlseek(src_fd, 0i32 as off_t, 0i32);
      }
      close_on_exec_on(src_fd);
      edit_file(pas, tmp_fname);
      current_block_48 = 16302727479442519837;
    }
    0 => {
      current_block_48 = 16302727479442519837;
    }
    _ => {
      /* case OPT_r: Delete */
      unlink((*pas).pw_name);
      current_block_48 = 5141539773904409130;
      /*free(tmp_fname);*/
      /*free(new_fname);*/
    }
  }
  match current_block_48 {
    16302727479442519837 =>
    /* fall through */
    /* Replace (no -l, -e, or -r were given) */
    {
      new_fname = xasprintf(
        b"%s.new\x00" as *const u8 as *const libc::c_char,
        (*pas).pw_name,
      );
      fd = open(
        new_fname,
        0o1i32 | 0o100i32 | 0o1000i32 | 0o2000i32,
        0o600i32,
      );
      if fd >= 0i32 {
        bb_copyfd_eof(src_fd, fd);
        close(fd);
        xrename(new_fname, (*pas).pw_name);
      } else {
        bb_error_msg(
          b"can\'t create %s/%s\x00" as *const u8 as *const libc::c_char,
          crontab_dir,
          new_fname,
        );
      }
      if !tmp_fname.is_null() {
        unlink(tmp_fname);
      }
    }
    _ => {}
  }
  loop
  /* Bump notification file.  Handle window where crond picks file up
   * before we can write our entry out.
   */
  {
    fd = open(
      b"cron.update\x00" as *const u8 as *const libc::c_char,
      0o1i32 | 0o100i32 | 0o2000i32,
      0o600i32,
    );
    if !(fd >= 0i32) {
      break;
    }
    let mut st: stat = stat {
      st_dev: 0,
      st_ino: 0,
      st_nlink: 0,
      st_mode: 0,
      st_uid: 0,
      st_gid: 0,
      __pad0: 0,
      st_rdev: 0,
      st_size: 0,
      st_blksize: 0,
      st_blocks: 0,
      st_atim: timespec {
        tv_sec: 0,
        tv_nsec: 0,
      },
      st_mtim: timespec {
        tv_sec: 0,
        tv_nsec: 0,
      },
      st_ctim: timespec {
        tv_sec: 0,
        tv_nsec: 0,
      },
      __glibc_reserved: [0; 3],
    };
    dprintf(
      fd,
      b"%s\n\x00" as *const u8 as *const libc::c_char,
      (*pas).pw_name,
    );
    if fstat(fd, &mut st) != 0i32 || st.st_nlink != 0i32 as libc::c_ulong {
      break;
    }
    /* loop */
    close(fd);
  }
  if fd < 0i32 {
    bb_error_msg(
      b"can\'t append to %s/%s\x00" as *const u8 as *const libc::c_char,
      crontab_dir,
      b"cron.update\x00" as *const u8 as *const libc::c_char,
    );
  }
  return 0i32;
}
/* st.st_nlink == 0:
 * file was deleted, maybe crond missed our notification */
