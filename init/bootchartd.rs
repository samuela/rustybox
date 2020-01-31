use crate::libbb::parse_config::parser_t;
use crate::libbb::ptr_to_globals::bb_errno;
use crate::librb::size_t;
use crate::librb::smallint;
use libc;
use libc::close;
use libc::closedir;
use libc::fclose;
use libc::fprintf;
use libc::getenv;
use libc::getpid;
use libc::kill;
use libc::mount;
use libc::open;
use libc::opendir;
use libc::pid_t;
use libc::putenv;
use libc::readdir;
use libc::rmdir;
use libc::sprintf;
use libc::strchr;
use libc::strcmp;
use libc::strstr;
use libc::system;
use libc::time;
use libc::time_t;
use libc::umount2;
use libc::unlink;
use libc::useconds_t;
use libc::FILE;
extern "C" {

  #[no_mangle]
  fn atof(__nptr: *const libc::c_char) -> libc::c_double;

  #[no_mangle]
  fn mkdtemp(__template: *mut libc::c_char) -> *mut libc::c_char;

  #[no_mangle]
  fn execl(__path: *const libc::c_char, __arg: *const libc::c_char, _: ...) -> libc::c_int;

  #[no_mangle]
  fn getppid() -> pid_t;
  #[no_mangle]
  fn vfork() -> libc::c_int;

  #[no_mangle]
  fn acct(__name: *const libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn raise(__sig: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn fflush(__stream: *mut FILE) -> libc::c_int;

  #[no_mangle]
  fn putc_unlocked(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn fileno_unlocked(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn usleep(__useconds: useconds_t) -> libc::c_int;

  #[no_mangle]
  fn strchrnul(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;

  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn waitpid(__pid: pid_t, __stat_loc: *mut libc::c_int, __options: libc::c_int) -> pid_t;

  #[no_mangle]
  fn strftime(
    __s: *mut libc::c_char,
    __maxsize: size_t,
    __format: *const libc::c_char,
    __tp: *const tm,
  ) -> size_t;
  #[no_mangle]
  fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;

  #[no_mangle]
  static mut bb_got_signal: smallint;

  #[no_mangle]
  static bb_PATH_root_path: [libc::c_char; 0];
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
  #[no_mangle]
  fn uname(__name: *mut utsname) -> libc::c_int;

}

use libc::dirent;
use libc::tm;
use libc::DIR;
pub type C2RustUnnamed = libc::c_uint;
pub const PARSE_NORMAL: C2RustUnnamed = 4653056;
pub const PARSE_WS_COMMENTS: C2RustUnnamed = 16777216;
pub const PARSE_ALT_COMMENTS: C2RustUnnamed = 8388608;
pub const PARSE_EOL_COMMENTS: C2RustUnnamed = 4194304;
pub const PARSE_KEEP_COPY: C2RustUnnamed = 2097152;
pub const PARSE_MIN_DIE: C2RustUnnamed = 1048576;
pub const PARSE_GREEDY: C2RustUnnamed = 262144;
pub const PARSE_TRIM: C2RustUnnamed = 131072;
pub const PARSE_COLLAPSE: C2RustUnnamed = 65536;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct globals {
  pub jiffy_line: [libc::c_char; 1024],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct utsname {
  pub sysname: [libc::c_char; 65],
  pub nodename: [libc::c_char; 65],
  pub release: [libc::c_char; 65],
  pub version: [libc::c_char; 65],
  pub machine: [libc::c_char; 65],
  pub domainname: [libc::c_char; 65],
}
pub type C2RustUnnamed_0 = libc::c_int;
pub const MS_NOUSER: C2RustUnnamed_0 = -2147483648;
pub const MS_ACTIVE: C2RustUnnamed_0 = 1073741824;
pub const MS_LAZYTIME: C2RustUnnamed_0 = 33554432;
pub const MS_STRICTATIME: C2RustUnnamed_0 = 16777216;
pub const MS_I_VERSION: C2RustUnnamed_0 = 8388608;
pub const MS_KERNMOUNT: C2RustUnnamed_0 = 4194304;
pub const MS_RELATIME: C2RustUnnamed_0 = 2097152;
pub const MS_SHARED: C2RustUnnamed_0 = 1048576;
pub const MS_SLAVE: C2RustUnnamed_0 = 524288;
pub const MS_PRIVATE: C2RustUnnamed_0 = 262144;
pub const MS_UNBINDABLE: C2RustUnnamed_0 = 131072;
pub const MS_POSIXACL: C2RustUnnamed_0 = 65536;
pub const MS_SILENT: C2RustUnnamed_0 = 32768;
pub const MS_REC: C2RustUnnamed_0 = 16384;
pub const MS_MOVE: C2RustUnnamed_0 = 8192;
pub const MS_BIND: C2RustUnnamed_0 = 4096;
pub const MS_NODIRATIME: C2RustUnnamed_0 = 2048;
pub const MS_NOATIME: C2RustUnnamed_0 = 1024;
pub const MS_DIRSYNC: C2RustUnnamed_0 = 128;
pub const MS_MANDLOCK: C2RustUnnamed_0 = 64;
pub const MS_REMOUNT: C2RustUnnamed_0 = 32;
pub const MS_SYNCHRONOUS: C2RustUnnamed_0 = 16;
pub const MS_NOEXEC: C2RustUnnamed_0 = 8;
pub const MS_NODEV: C2RustUnnamed_0 = 4;
pub const MS_NOSUID: C2RustUnnamed_0 = 2;
pub const MS_RDONLY: C2RustUnnamed_0 = 1;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const UMOUNT_NOFOLLOW: C2RustUnnamed_1 = 8;
pub const MNT_EXPIRE: C2RustUnnamed_1 = 4;
pub const MNT_DETACH: C2RustUnnamed_1 = 2;
pub const MNT_FORCE: C2RustUnnamed_1 = 1;
pub const CMD_START: C2RustUnnamed_2 = 1;
pub const CMD_PID1: C2RustUnnamed_2 = 3;
pub const CMD_STOP: C2RustUnnamed_2 = 0;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const CMD_INIT: C2RustUnnamed_2 = 2;
unsafe fn dump_file(mut fp: *mut FILE, mut filename: *const libc::c_char) {
  let mut fd: libc::c_int = open(filename, 0);
  if fd >= 0 {
    fputs_unlocked(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .jiffy_line
        .as_mut_ptr(),
      fp,
    );
    fflush(fp);
    crate::libbb::copyfd::bb_copyfd_eof(fd, fileno_unlocked(fp));
    close(fd);
    putc_unlocked('\n' as i32, fp);
  };
}
unsafe fn dump_procs(mut fp: *mut FILE, mut look_for_login_process: libc::c_int) -> libc::c_int {
  let mut entry: *mut dirent = std::ptr::null_mut();
  let mut dir: *mut DIR = opendir(b"/proc\x00" as *const u8 as *const libc::c_char);
  let mut found_login_process: libc::c_int = 0;
  fputs_unlocked(
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .jiffy_line
      .as_mut_ptr(),
    fp,
  );
  loop {
    entry = readdir(dir);
    if entry.is_null() {
      break;
    }
    let mut name: [libc::c_char; 29] = [0; 29];
    let mut stat_fd: libc::c_int = 0;
    let mut pid: libc::c_uint = crate::libbb::bb_strtonum::bb_strtou(
      (*entry).d_name.as_mut_ptr(),
      0 as *mut *mut libc::c_char,
      10i32,
    );
    if *bb_errno != 0 {
      continue;
    }
    /* used to mark pid 1 case */
    /* Android's version reads /proc/PID/cmdline and extracts
     * non-truncated process name. Do we want to do that? */
    sprintf(
      name.as_mut_ptr(),
      b"/proc/%u/stat\x00" as *const u8 as *const libc::c_char,
      pid,
    );
    stat_fd = open(name.as_mut_ptr(), 0);
    if !(stat_fd >= 0) {
      continue;
    }
    let mut p: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut stat_line: [libc::c_char; 4096] = [0; 4096];
    let mut rd: libc::c_int = crate::libbb::read::safe_read(
      stat_fd,
      stat_line.as_mut_ptr() as *mut libc::c_void,
      (::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong)
        .wrapping_sub(2i32 as libc::c_ulong),
    ) as libc::c_int;
    close(stat_fd);
    if rd < 0 {
      continue;
    }
    stat_line[rd as usize] = '\u{0}' as i32 as libc::c_char;
    p = strchrnul(stat_line.as_mut_ptr(), '\n' as i32);
    let fresh0 = p;
    p = p.offset(1);
    *fresh0 = '\n' as i32 as libc::c_char;
    *p = '\u{0}' as i32 as libc::c_char;
    fputs_unlocked(stat_line.as_mut_ptr(), fp);
    if look_for_login_process == 0 {
      continue;
    }
    p = strchr(stat_line.as_mut_ptr(), '(' as i32);
    if p.is_null() {
      continue;
    }
    p = p.offset(1);
    *strchrnul(p, ')' as i32).offset(0) = '\u{0}' as i32 as libc::c_char;
    /* Is it gdm, kdm or a getty? */
    if (*p.offset(0) as libc::c_int == 'g' as i32
      || *p.offset(0) as libc::c_int == 'k' as i32
      || *p.offset(0) as libc::c_int == 'x' as i32)
      && *p.offset(1) as libc::c_int == 'd' as i32
      && *p.offset(2) as libc::c_int == 'm' as i32
      && *p.offset(3) as libc::c_int == '\u{0}' as i32
      || !strstr(p, b"getty\x00" as *const u8 as *const libc::c_char).is_null()
    {
      found_login_process = 1i32
    }
  }
  closedir(dir);
  putc_unlocked('\n' as i32, fp);
  return found_login_process;
}
unsafe fn make_tempdir() -> *mut libc::c_char {
  let mut template: [libc::c_char; 22] =
    *::std::mem::transmute::<&[u8; 22], &mut [libc::c_char; 22]>(b"/tmp/bootchart.XXXXXX\x00");
  let mut tempdir: *mut libc::c_char =
    crate::libbb::xfuncs_printf::xstrdup(mkdtemp(template.as_mut_ptr()));
  if tempdir.is_null() {
    /* /tmp is not writable (happens when we are used as init).
     * Try to mount a tmpfs, then cd and lazily unmount it.
     * Since we unmount it at once, we can mount it anywhere.
     * Try a few locations which are likely ti exist.
     */
    static mut dirs: [libc::c_char; 23] = [
      47, 109, 110, 116, 0, 47, 116, 109, 112, 0, 47, 98, 111, 111, 116, 0, 47, 112, 114, 111, 99,
      0, 0,
    ];
    let mut try_dir: *const libc::c_char = dirs.as_ptr();
    while mount(
      b"none\x00" as *const u8 as *const libc::c_char,
      try_dir,
      b"tmpfs\x00" as *const u8 as *const libc::c_char,
      MS_SILENT as libc::c_int as libc::c_ulong,
      b"size=16m\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
    ) != 0
    {
      try_dir = try_dir.offset(strlen(try_dir).wrapping_add(1i32 as libc::c_ulong) as isize);
      if *try_dir.offset(0) == 0 {
        crate::libbb::perror_msg::bb_perror_msg_and_die(
          b"can\'t %smount tmpfs\x00" as *const u8 as *const libc::c_char,
          b"\x00" as *const u8 as *const libc::c_char,
        );
      }
    }
    //bb_error_msg("mounted tmpfs on %s", try_dir);
    crate::libbb::xfuncs_printf::xchdir(try_dir);
    if umount2(try_dir, MNT_DETACH as libc::c_int) != 0 {
      crate::libbb::perror_msg::bb_perror_msg_and_die(
        b"can\'t %smount tmpfs\x00" as *const u8 as *const libc::c_char,
        b"un\x00" as *const u8 as *const libc::c_char,
      );
    }
  } else {
    crate::libbb::xfuncs_printf::xchdir(tempdir);
  }
  return tempdir;
}
unsafe fn do_logging(mut sample_period_us: libc::c_uint, mut process_accounting: libc::c_int) {
  let mut proc_stat: *mut FILE = crate::libbb::xfuncs_printf::xfopen(
    b"proc_stat.log\x00" as *const u8 as *const libc::c_char,
    b"w\x00" as *const u8 as *const libc::c_char,
  );
  let mut proc_diskstats: *mut FILE = crate::libbb::xfuncs_printf::xfopen(
    b"proc_diskstats.log\x00" as *const u8 as *const libc::c_char,
    b"w\x00" as *const u8 as *const libc::c_char,
  );
  //FILE *proc_netdev = xfopen("proc_netdev.log", "w");
  let mut proc_ps: *mut FILE = crate::libbb::xfuncs_printf::xfopen(
    b"proc_ps.log\x00" as *const u8 as *const libc::c_char,
    b"w\x00" as *const u8 as *const libc::c_char,
  ); /* ~1 minute */
  let mut look_for_login_process: libc::c_int = (getppid() == 1i32) as libc::c_int;
  let mut count: libc::c_uint =
    ((60i32 * 1000i32 * 1000i32) as libc::c_uint).wrapping_div(sample_period_us);
  if process_accounting != 0 {
    close(crate::libbb::xfuncs_printf::xopen(
      b"kernel_pacct\x00" as *const u8 as *const libc::c_char,
      0o1i32 | 0o100i32 | 0o1000i32,
    ));
    acct(b"kernel_pacct\x00" as *const u8 as *const libc::c_char);
  }
  loop {
    count = count.wrapping_sub(1);
    if !(count != 0 && bb_got_signal == 0) {
      break;
    }
    let mut p: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut len: libc::c_int = crate::libbb::read::open_read_close(
      b"/proc/uptime\x00" as *const u8 as *const libc::c_char,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .jiffy_line
        .as_mut_ptr() as *mut libc::c_void,
      (::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
        .wrapping_sub(2i32 as libc::c_ulong),
    ) as libc::c_int;
    if !(len < 0) {
      /* /proc/uptime has format "NNNNNN.MM NNNNNNN.MM" */
      /* we convert it to "NNNNNNMM\n" (using first value) */
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).jiffy_line[len as usize] =
        '\u{0}' as i32 as libc::c_char;
      p = strchr(
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
          .jiffy_line
          .as_mut_ptr(),
        '.' as i32,
      );
      if !p.is_null() {
        loop {
          p = p.offset(1);
          if !((*p as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32) {
            break;
          }
          *p.offset(-1i32 as isize) = *p
        }
        *p.offset(-1i32 as isize) = '\n' as i32 as libc::c_char;
        *p.offset(0) = '\u{0}' as i32 as libc::c_char;
        dump_file(
          proc_stat,
          b"/proc/stat\x00" as *const u8 as *const libc::c_char,
        );
        dump_file(
          proc_diskstats,
          b"/proc/diskstats\x00" as *const u8 as *const libc::c_char,
        );
        //dump_file(proc_netdev, "/proc/net/dev");
        if dump_procs(proc_ps, look_for_login_process) != 0 {
          /* dump_procs saw a getty or {g,k,x}dm
           * stop logging in 2 seconds:
           */
          if count > ((2i32 * 1000i32 * 1000i32) as libc::c_uint).wrapping_div(sample_period_us) {
            count = ((2i32 * 1000i32 * 1000i32) as libc::c_uint).wrapping_div(sample_period_us)
          }
        }
        crate::libbb::xfuncs_printf::fflush_all();
      }
    }
    usleep(sample_period_us);
  }
}
unsafe fn finalize(
  mut tempdir: *mut libc::c_char,
  mut prog: *const libc::c_char,
  mut process_accounting: libc::c_int,
) {
  //# Stop process accounting if configured
  //local pacct=
  //[ -e kernel_pacct ] && pacct=kernel_pacct
  let mut header_fp: *mut FILE = crate::libbb::xfuncs_printf::xfopen(
    b"header\x00" as *const u8 as *const libc::c_char,
    b"w\x00" as *const u8 as *const libc::c_char,
  );
  if process_accounting != 0 {
    acct(0 as *const libc::c_char);
  }
  if !prog.is_null() {
    fprintf(
      header_fp,
      b"profile.process = %s\n\x00" as *const u8 as *const libc::c_char,
      prog,
    );
  }
  fputs_unlocked(
    b"version = 0.8\n\x00" as *const u8 as *const libc::c_char,
    header_fp,
  );
  let mut hostname: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut kcmdline: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut t: time_t = 0;
  let mut tm_time: tm = std::mem::zeroed();
  /* x2 for possible localized weekday/month names */
  let mut date_buf: [libc::c_char; 60] = [0; 60]; /* never fails */
  let mut unamebuf: utsname = utsname {
    sysname: [0; 65],
    nodename: [0; 65],
    release: [0; 65],
    version: [0; 65],
    machine: [0; 65],
    domainname: [0; 65],
  };
  hostname = crate::libbb::safe_gethostname::safe_gethostname();
  time(&mut t);
  localtime_r(&mut t, &mut tm_time);
  strftime(
    date_buf.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_char; 60]>() as libc::c_ulong,
    b"%a %b %e %H:%M:%S %Z %Y\x00" as *const u8 as *const libc::c_char,
    &mut tm_time,
  );
  fprintf(
    header_fp,
    b"title = Boot chart for %s (%s)\n\x00" as *const u8 as *const libc::c_char,
    hostname,
    date_buf.as_mut_ptr(),
  );
  uname(&mut unamebuf);
  /* same as uname -srvm */
  fprintf(
    header_fp,
    b"system.uname = %s %s %s %s\n\x00" as *const u8 as *const libc::c_char,
    unamebuf.sysname.as_mut_ptr(),
    unamebuf.release.as_mut_ptr(),
    unamebuf.version.as_mut_ptr(),
    unamebuf.machine.as_mut_ptr(),
  );
  //system.release = `cat /etc/DISTRO-release`
  //system.cpu = `grep '^model name' /proc/cpuinfo | head -1` ($cpucount)
  kcmdline = crate::libbb::read_printf::xmalloc_open_read_close(
    b"/proc/cmdline\x00" as *const u8 as *const libc::c_char,
    std::ptr::null_mut::<size_t>(),
  ) as *mut libc::c_char;
  /* kcmdline includes trailing "\n" */
  fprintf(
    header_fp,
    b"system.kernel.options = %s\x00" as *const u8 as *const libc::c_char,
    kcmdline,
  );
  fclose(header_fp);
  /* Package log files */
  system(crate::libbb::xfuncs_printf::xasprintf(
    b"tar -zcf /var/log/bootlog.tgz header %s *.log\x00" as *const u8 as *const libc::c_char,
    if process_accounting != 0 {
      b"kernel_pacct\x00" as *const u8 as *const libc::c_char
    } else {
      b"\x00" as *const u8 as *const libc::c_char
    },
  ));
  /* Clean up (if we are not in detached tmpfs) */
  if !tempdir.is_null() {
    unlink(b"header\x00" as *const u8 as *const libc::c_char);
    unlink(b"proc_stat.log\x00" as *const u8 as *const libc::c_char);
    unlink(b"proc_diskstats.log\x00" as *const u8 as *const libc::c_char);
    //unlink("proc_netdev.log");
    unlink(b"proc_ps.log\x00" as *const u8 as *const libc::c_char);
    if process_accounting != 0 {
      unlink(b"kernel_pacct\x00" as *const u8 as *const libc::c_char);
    }
    rmdir(tempdir);
  };
  /* shell-based bootchartd tries to run /usr/bin/bootchart if $AUTO_RENDER=yes:
   * /usr/bin/bootchart -o "$AUTO_RENDER_DIR" -f $AUTO_RENDER_FORMAT "$BOOTLOG_DEST"
   */
}
//usage:#define bootchartd_trivial_usage
//usage:       "start [PROG ARGS]|stop|init"
//usage:#define bootchartd_full_usage "\n\n"
//usage:       "Create /var/log/bootchart.tgz with boot chart data\n"
//usage:     "\nstart: start background logging; with PROG, run PROG, then kill logging with USR1"
//usage:     "\nstop: send USR1 to all bootchartd processes"
//usage:     "\ninit: start background logging; stop when getty/xdm is seen (for init scripts)"
//usage:     "\nUnder PID 1: as init, then exec $bootchart_init, /init, /sbin/init"
pub unsafe fn bootchartd_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut sample_period_us: libc::c_uint = 0;
  let mut parent_pid: pid_t = 0;
  let mut logger_pid: pid_t = 0;
  let mut cmd: smallint = 0;
  let mut process_accounting: libc::c_int = 0;
  parent_pid = getpid();
  if !(*argv.offset(1)).is_null() {
    cmd = crate::libbb::compare_string_array::index_in_strings(
      b"stop\x00start\x00init\x00\x00" as *const u8 as *const libc::c_char,
      *argv.offset(1),
    ) as smallint;
    if (cmd as libc::c_int) < 0 {
      crate::libbb::appletlib::bb_show_usage();
    }
    if cmd as libc::c_int == CMD_STOP as libc::c_int {
      let mut pidList: *mut pid_t = crate::libbb::find_pid_by_name::find_pid_by_name(
        b"bootchartd\x00" as *const u8 as *const libc::c_char,
      );
      while *pidList != 0 {
        if *pidList != parent_pid {
          kill(*pidList, 10i32);
        }
        pidList = pidList.offset(1)
      }
      return 0;
    }
  } else {
    if parent_pid != 1i32 {
      crate::libbb::appletlib::bb_show_usage();
    }
    cmd = CMD_PID1 as libc::c_int as smallint
  }
  /* Here we are in START, INIT or CMD_PID1 state */
  /* Read config file: */
  sample_period_us = (200i32 * 1000i32) as libc::c_uint;
  process_accounting = 0;
  let mut token: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2];
  let mut parser: *mut parser_t = crate::libbb::parse_config::config_open2(
    (b"/etc/bootchartd.conf\x00" as *const u8 as *const libc::c_char).offset(5),
    Some(crate::libbb::wfopen::fopen_for_read as unsafe fn(_: *const libc::c_char) -> *mut FILE),
  );
  if parser.is_null() {
    parser = crate::libbb::parse_config::config_open2(
      b"/etc/bootchartd.conf\x00" as *const u8 as *const libc::c_char,
      Some(crate::libbb::wfopen::fopen_for_read as unsafe fn(_: *const libc::c_char) -> *mut FILE),
    )
  }
  while crate::libbb::parse_config::config_read(
    parser,
    token.as_mut_ptr(),
    (PARSE_NORMAL as libc::c_int & !(PARSE_COLLAPSE as libc::c_int)
      | (0i32 & 0xffi32) << 8i32
      | 2i32 & 0xffi32) as libc::c_uint,
    b"#=\x00" as *const u8 as *const libc::c_char,
  ) != 0
  {
    if strcmp(
      token[0],
      b"SAMPLE_PERIOD\x00" as *const u8 as *const libc::c_char,
    ) == 0
      && !token[1].is_null()
    {
      sample_period_us = (atof(token[1]) * 1000000i32 as libc::c_double) as libc::c_uint
    }
    if strcmp(
      token[0],
      b"PROCESS_ACCOUNTING\x00" as *const u8 as *const libc::c_char,
    ) == 0
      && !token[1].is_null()
      && (strcmp(token[1], b"on\x00" as *const u8 as *const libc::c_char) == 0
        || strcmp(token[1], b"yes\x00" as *const u8 as *const libc::c_char) == 0)
    {
      process_accounting = 1i32
    }
  }
  crate::libbb::parse_config::config_close(parser);
  if sample_period_us as libc::c_int <= 0 {
    sample_period_us = 1i32 as libc::c_uint
  }
  /* prevent division by 0 */
  /* Create logger child: */
  logger_pid = crate::libbb::xfuncs_printf::xfork();
  if logger_pid == 0 {
    /* child */
    let mut tempdir: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    crate::libbb::signals::bb_signals(
      0 + (1i32 << 10i32)
        + (1i32 << 12i32)
        + (1i32 << 15i32)
        + (1i32 << 3i32)
        + (1i32 << 2i32)
        + (1i32 << 1i32),
      Some(crate::libbb::signals::record_signo),
    );
    /* Inform parent that we are ready */
    raise(19i32);
    /* If we are started by kernel, PATH might be unset.
     * In order to find "tar", let's set some sane PATH:
     */
    if cmd as libc::c_int == CMD_PID1 as libc::c_int
      && getenv(b"PATH\x00" as *const u8 as *const libc::c_char).is_null()
    {
      putenv(bb_PATH_root_path.as_ptr() as *mut libc::c_char);
    }
    tempdir = make_tempdir();
    do_logging(sample_period_us, process_accounting);
    finalize(
      tempdir,
      if cmd as libc::c_int == CMD_START as libc::c_int {
        *argv.offset(2)
      } else {
        std::ptr::null_mut::<libc::c_char>()
      },
      process_accounting,
    );
    return 0;
  }
  /* parent */
  /* undo fork_or_rexec() damage */
  /* Wait for logger child to set handlers, then unpause it.
   * Otherwise with short-lived PROG (e.g. "bootchartd start true")
   * we might send SIGUSR1 before logger sets its handler.
   */
  waitpid(logger_pid, 0 as *mut libc::c_int, 2i32);
  kill(logger_pid, 18i32);
  if cmd as libc::c_int == CMD_PID1 as libc::c_int {
    let mut bootchart_init: *mut libc::c_char =
      getenv(b"bootchart_init\x00" as *const u8 as *const libc::c_char);
    if !bootchart_init.is_null() {
      execl(bootchart_init, bootchart_init, 0 as *mut libc::c_void);
    }
    execl(
      b"/init\x00" as *const u8 as *const libc::c_char,
      b"init\x00" as *const u8 as *const libc::c_char,
      0 as *mut libc::c_void,
    );
    execl(
      b"/sbin/init\x00" as *const u8 as *const libc::c_char,
      b"init\x00" as *const u8 as *const libc::c_char,
      0 as *mut libc::c_void,
    );
    crate::libbb::perror_msg::bb_perror_msg_and_die(
      b"can\'t execute \'%s\'\x00" as *const u8 as *const libc::c_char,
      b"/sbin/init\x00" as *const u8 as *const libc::c_char,
    );
  }
  if cmd as libc::c_int == CMD_START as libc::c_int && !(*argv.offset(2)).is_null() {
    /* "start PROG ARGS" */
    let mut pid: pid_t = {
      let mut bb__xvfork_pid: pid_t = vfork();
      if bb__xvfork_pid < 0 {
        crate::libbb::perror_msg::bb_simple_perror_msg_and_die(
          b"vfork\x00" as *const u8 as *const libc::c_char,
        );
      }
      bb__xvfork_pid
    };
    if pid == 0 {
      /* child */
      argv = argv.offset(2);
      crate::libbb::executable::BB_EXECVP_or_die(argv);
    }
    /* parent */
    waitpid(pid, 0 as *mut libc::c_int, 0);
    kill(logger_pid, 10i32);
  }
  return 0;
}
