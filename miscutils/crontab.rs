use libc;
use libc::close;
use libc::fstat;
use libc::getenv;
use libc::getpid;
use libc::getuid;
use libc::gid_t;
use libc::off_t;
use libc::open;
use libc::passwd;
use libc::pid_t;
use libc::stat;
use libc::uid_t;
use libc::unlink;
extern "C" {

  #[no_mangle]
  fn fchown(__fd: libc::c_int, __owner: uid_t, __group: gid_t) -> libc::c_int;
  #[no_mangle]
  fn execlp(__file: *const libc::c_char, __arg: *const libc::c_char, _: ...) -> libc::c_int;

  #[no_mangle]
  fn vfork() -> libc::c_int;

  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  fn dprintf(__fd: libc::c_int, __fmt: *const libc::c_char, _: ...) -> libc::c_int;

  #[no_mangle]
  static bb_msg_you_must_be_root: [libc::c_char; 0];
}

pub type C2RustUnnamed = libc::c_uint;
pub const OPT_e: C2RustUnnamed = 8;
pub const OPT_l: C2RustUnnamed = 4;
pub const OPT_ler: C2RustUnnamed = 28;
pub const OPT_u: C2RustUnnamed = 1;
pub const OPT_c: C2RustUnnamed = 2;
pub const OPT_r: C2RustUnnamed = 16;

unsafe extern "C" fn edit_file(mut pas: *const passwd, mut file: *const libc::c_char) {
  let mut ptr: *const libc::c_char = 0 as *const libc::c_char;
  let mut pid: pid_t = 0;
  pid = {
    let mut bb__xvfork_pid: pid_t = vfork();
    if bb__xvfork_pid < 0i32 {
      crate::libbb::perror_msg::bb_simple_perror_msg_and_die(
        b"vfork\x00" as *const u8 as *const libc::c_char,
      );
    }
    bb__xvfork_pid
  };
  if pid != 0 {
    /* parent */
    crate::libbb::xfuncs::wait4pid(pid);
    return;
  }
  /* CHILD - change user and run editor */
  /* initgroups, setgid, setuid */
  crate::libbb::change_identity::change_identity(pas); /* -u USER */
  crate::libbb::setup_environment::setup_environment(
    (*pas).pw_shell,
    1i32 << 0i32 | 1i32 << 2i32,
    pas,
  );
  ptr = getenv(b"VISUAL\x00" as *const u8 as *const libc::c_char);
  if ptr.is_null() {
    ptr = getenv(b"EDITOR\x00" as *const u8 as *const libc::c_char);
    if ptr.is_null() {
      ptr = b"vi\x00" as *const u8 as *const libc::c_char
    }
  }
  execlp(ptr, ptr, file, 0 as *mut libc::c_void);
  crate::libbb::perror_msg::bb_perror_msg_and_die(
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
  let mut tmp_fname: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut new_fname: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut user_name: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
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
  opt_ler = crate::libbb::getopt32::getopt32(
    argv,
    b"^u:c:lerd\x00?1:dr\x00" as *const u8 as *const libc::c_char,
    &mut user_name as *mut *mut libc::c_char,
    &mut crontab_dir as *mut *const libc::c_char,
  ) as libc::c_int;
  argv = argv.offset(optind as isize);
  if crate::libbb::login::sanitize_env_if_suid() != 0 {
    /* Clears dangerous stuff, sets PATH */
    /* Run by non-root */
    if opt_ler & (OPT_u as libc::c_int | OPT_c as libc::c_int) != 0 {
      crate::libbb::verror_msg::bb_simple_error_msg_and_die(bb_msg_you_must_be_root.as_ptr());
    }
  }
  if opt_ler & OPT_u as libc::c_int != 0 {
    pas = crate::libbb::bb_pwd::xgetpwnam(user_name)
  } else {
    pas = crate::libbb::bb_pwd::xgetpwuid(getuid())
  }
  /* From now on, keep only -l, -e, -r bits */
  opt_ler &= OPT_ler as libc::c_int;
  if opt_ler - 1i32 & opt_ler != 0 {
    /* more than one bit set? */
    crate::libbb::appletlib::bb_show_usage();
  }
  /* Read replacement file under user's UID/GID/group vector */
  src_fd = 0i32;
  if opt_ler == 0 {
    /* Replace? */
    if (*argv.offset(0)).is_null() {
      crate::libbb::appletlib::bb_show_usage();
    }
    if *(*argv.offset(0)).offset(0) as libc::c_int != '-' as i32
      || *(*argv.offset(0)).offset(1) as libc::c_int != 0
    {
      src_fd = crate::libbb::xfuncs_printf::xopen_as_uid_gid(
        *argv.offset(0),
        0i32,
        (*pas).pw_uid,
        (*pas).pw_gid,
      )
    }
  }
  /* cd to our crontab directory */
  crate::libbb::xfuncs_printf::xchdir(crontab_dir);
  tmp_fname = std::ptr::null_mut::<libc::c_char>();
  let mut current_block_48: u64;
  /* Handle requested operation */
  match opt_ler {
    4 => {
      /* switch */
      /* List */
      let mut args: [*mut libc::c_char; 2] = [(*pas).pw_name, std::ptr::null_mut::<libc::c_char>()];
      return crate::libbb::bb_cat::bb_cat(args.as_mut_ptr());
      /* list exits,
       * the rest go play with cron update file */
    }
    8 => {
      /* Edit */
      tmp_fname = crate::libbb::xfuncs_printf::xasprintf(
        b"%s.%u\x00" as *const u8 as *const libc::c_char,
        crontab_dir,
        getpid() as libc::c_uint,
      );
      /* No O_EXCL: we don't want to be stuck if earlier crontabs
       * were killed, leaving stale temp file behind */
      src_fd =
        crate::libbb::xfuncs_printf::xopen3(tmp_fname, 0o2i32 | 0o100i32 | 0o1000i32, 0o600i32); /* don't want editor to see this fd */
      fchown(src_fd, (*pas).pw_uid, (*pas).pw_gid);
      fd = open((*pas).pw_name, 0i32);
      if fd >= 0i32 {
        crate::libbb::copyfd::bb_copyfd_eof(fd, src_fd);
        close(fd);
        crate::libbb::xfuncs_printf::xlseek(src_fd, 0i32 as off_t, 0i32);
      }
      crate::libbb::xfuncs::close_on_exec_on(src_fd);
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
      new_fname = crate::libbb::xfuncs_printf::xasprintf(
        b"%s.new\x00" as *const u8 as *const libc::c_char,
        (*pas).pw_name,
      );
      fd = open(
        new_fname,
        0o1i32 | 0o100i32 | 0o1000i32 | 0o2000i32,
        0o600i32,
      );
      if fd >= 0i32 {
        crate::libbb::copyfd::bb_copyfd_eof(src_fd, fd);
        close(fd);
        crate::libbb::xfuncs_printf::xrename(new_fname, (*pas).pw_name);
      } else {
        crate::libbb::verror_msg::bb_error_msg(
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
    let mut st: stat = std::mem::zeroed();
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
    crate::libbb::verror_msg::bb_error_msg(
      b"can\'t append to %s/%s\x00" as *const u8 as *const libc::c_char,
      crontab_dir,
      b"cron.update\x00" as *const u8 as *const libc::c_char,
    );
  }
  return 0i32;
}
/* st.st_nlink == 0:
 * file was deleted, maybe crond missed our notification */
