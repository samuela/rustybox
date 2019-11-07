use crate::librb::size_t;
use crate::librb::smallint;
use libc;
use libc::pid_t;
use libc::FILE;

extern "C" {
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn kill(__pid: pid_t, __sig: libc::c_int) -> libc::c_int;
  #[no_mangle]
  static mut stdout: *mut FILE;
  #[no_mangle]
  fn setbuf(__stream: *mut FILE, __buf: *mut libc::c_char);
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn puts(__s: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strchrnul(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn waitpid(__pid: pid_t, __stat_loc: *mut libc::c_int, __options: libc::c_int) -> pid_t;
  #[no_mangle]
  fn setmntent(__file: *const libc::c_char, __mode: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn getmntent_r(
    __stream: *mut FILE,
    __result: *mut mntent,
    __buffer: *mut libc::c_char,
    __bufsize: libc::c_int,
  ) -> *mut mntent;
  #[no_mangle]
  fn endmntent(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn skip_dev_pfx(tty_name: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xrealloc(old: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xrealloc_vector_helper(
    vector: *mut libc::c_void,
    sizeof_and_shift: libc::c_uint,
    idx: libc::c_int,
  ) -> *mut libc::c_void;
  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xstrndup(s: *const libc::c_char, n: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn is_prefixed_with(string: *const libc::c_char, key: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn signal_no_SA_RESTART_empty_mask(
    sig: libc::c_int,
    handler: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
  );
  #[no_mangle]
  static mut bb_got_signal: smallint;
  #[no_mangle]
  fn record_signo(signo: libc::c_int);
  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  #[no_mangle]
  fn xatoi(str: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn spawn(argv: *mut *mut libc::c_char) -> pid_t;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_simple_error_msg(s: *const libc::c_char);
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_perror_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);
  #[no_mangle]
  fn index_in_strings(strings: *const libc::c_char, key: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct mntent {
  pub mnt_fsname: *mut libc::c_char,
  pub mnt_dir: *mut libc::c_char,
  pub mnt_type: *mut libc::c_char,
  pub mnt_opts: *mut libc::c_char,
  pub mnt_freq: libc::c_int,
  pub mnt_passno: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub args: *mut *mut libc::c_char,
  pub num_args: libc::c_int,
  pub verbose: libc::c_int,
  pub fs_type_list: *mut *mut libc::c_char,
  pub fs_type_flag: *mut u8,
  pub fs_type_negated: smallint,
  pub noexecute: smallint,
  pub serialize: smallint,
  pub skip_root: smallint,
  pub parallel_root: smallint,
  pub force_all_parallel: smallint,
  pub kill_sent: smallint,
  pub num_running: libc::c_int,
  pub max_running: libc::c_int,
  pub fstype: *mut libc::c_char,
  pub filesys_info: *mut fs_info,
  pub filesys_last: *mut fs_info,
  pub instance_list: *mut fsck_instance,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fsck_instance {
  pub next: *mut fsck_instance,
  pub pid: libc::c_int,
  pub flags: libc::c_int,
  pub prog: *mut libc::c_char,
  pub device: *mut libc::c_char,
  pub base_device: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fs_info {
  pub next: *mut fs_info,
  pub device: *mut libc::c_char,
  pub mountpt: *mut libc::c_char,
  pub type_0: *mut libc::c_char,
  pub opts: *mut libc::c_char,
  pub passno: libc::c_int,
  pub flags: libc::c_int,
}
static mut ignored_types: [libc::c_char; 46] = [
  105, 103, 110, 111, 114, 101, 0, 105, 115, 111, 57, 54, 54, 48, 0, 110, 102, 115, 0, 112, 114,
  111, 99, 0, 115, 119, 0, 115, 119, 97, 112, 0, 116, 109, 112, 102, 115, 0, 100, 101, 118, 112,
  116, 115, 0, 0,
];
/*
 * Return the "base device" given a particular device; this is used to
 * assure that we only fsck one partition on a particular drive at any
 * one time.  Otherwise, the disk heads will be seeking all over the
 * place.  If the base device cannot be determined, return NULL.
 *
 * The base_device() function returns an allocated string which must
 * be freed.
 */
unsafe extern "C" fn base_device(mut device: *const libc::c_char) -> *mut libc::c_char {
  let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
  str = xstrdup(device);
  /* Skip over "/dev/"; if it's not present, give up */
  cp = skip_dev_pfx(str);
  if !(cp == str) {
    /*
     * For md devices, we treat them all as if they were all
     * on one disk, since we don't know how to parallelize them.
     */
    if *cp.offset(0) as libc::c_int == 'm' as i32 && *cp.offset(1) as libc::c_int == 'd' as i32 {
      *cp.offset(2) = 0i32 as libc::c_char;
      return str;
    }
    /* Handle DAC 960 devices */
    if !is_prefixed_with(cp, b"rd/\x00" as *const u8 as *const libc::c_char).is_null() {
      cp = cp.offset(3);
      if !(*cp.offset(0) as libc::c_int != 'c' as i32
        || !((*cp.offset(1) as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32)
        || *cp.offset(2) as libc::c_int != 'd' as i32
        || !((*cp.offset(3) as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32))
      {
        *cp.offset(4) = 0i32 as libc::c_char;
        return str;
      }
    } else if (*cp.offset(0) as libc::c_int == 'h' as i32
      || *cp.offset(0) as libc::c_int == 's' as i32)
      && *cp.offset(1) as libc::c_int == 'd' as i32
    {
      cp = cp.offset(2);
      /* Now let's handle /dev/hd* and /dev/sd* devices.... */
      /* If there's a single number after /dev/hd, skip it */
      if (*cp as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32 {
        cp = cp.offset(1)
      }
      /* What follows must be an alpha char, or give up */
      if ((*cp as libc::c_int | 0x20i32) - 'a' as i32) as libc::c_uchar as libc::c_int
        <= 'z' as i32 - 'a' as i32
      {
        *cp.offset(1) = 0i32 as libc::c_char;
        return str;
      }
    }
  }
  free(str as *mut libc::c_void);
  return 0 as *mut libc::c_char;
}
unsafe extern "C" fn free_instance(mut p: *mut fsck_instance) {
  free((*p).prog as *mut libc::c_void);
  free((*p).device as *mut libc::c_void);
  free((*p).base_device as *mut libc::c_void);
  free(p as *mut libc::c_void);
}
unsafe extern "C" fn create_fs_device(
  mut device: *const libc::c_char,
  mut mntpnt: *const libc::c_char,
  mut type_0: *const libc::c_char,
  mut opts: *const libc::c_char,
  mut passno: libc::c_int,
) -> *mut fs_info {
  let mut fs: *mut fs_info = 0 as *mut fs_info;
  fs = xzalloc(::std::mem::size_of::<fs_info>() as libc::c_ulong) as *mut fs_info;
  (*fs).device = xstrdup(device);
  (*fs).mountpt = xstrdup(mntpnt);
  if !strchr(type_0, ',' as i32).is_null() {
    type_0 = b"auto\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
  }
  (*fs).type_0 = xstrdup(type_0);
  (*fs).opts = xstrdup(if !opts.is_null() {
    opts
  } else {
    b"\x00" as *const u8 as *const libc::c_char
  });
  (*fs).passno = if passno < 0i32 { 1i32 } else { passno };
  /*fs->flags = 0; */
  /*fs->next = NULL; */
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .filesys_info
    .is_null()
  {
    let ref mut fresh0 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).filesys_info;
    *fresh0 = fs
  } else {
    let ref mut fresh1 = (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).filesys_last).next;
    *fresh1 = fs
  }
  let ref mut fresh2 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).filesys_last;
  *fresh2 = fs;
  return fs;
}
/* Load the filesystem database from /etc/fstab */
unsafe extern "C" fn load_fs_info(mut filename: *const libc::c_char) {
  let mut fstab: *mut FILE = 0 as *mut FILE;
  let mut mte: mntent = mntent {
    mnt_fsname: 0 as *mut libc::c_char,
    mnt_dir: 0 as *mut libc::c_char,
    mnt_type: 0 as *mut libc::c_char,
    mnt_opts: 0 as *mut libc::c_char,
    mnt_freq: 0,
    mnt_passno: 0,
  };
  let mut buf: [libc::c_char; 1024] = [0; 1024];
  fstab = setmntent(filename, b"r\x00" as *const u8 as *const libc::c_char);
  if fstab.is_null() {
    bb_perror_msg(
      b"can\'t read \'%s\'\x00" as *const u8 as *const libc::c_char,
      filename,
    );
    return;
  }
  // Loop through entries
  while !getmntent_r(
    fstab,
    &mut mte,
    buf.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
  )
  .is_null()
  {
    //bb_error_msg("CREATE[%s][%s][%s][%s][%d]", mte.mnt_fsname, mte.mnt_dir,
    //	mte.mnt_type, mte.mnt_opts,
    //	mte.mnt_passno);
    create_fs_device(
      mte.mnt_fsname,
      mte.mnt_dir,
      mte.mnt_type,
      mte.mnt_opts,
      mte.mnt_passno,
    );
  }
  endmntent(fstab);
}
/* Lookup filesys in /etc/fstab and return the corresponding entry. */
unsafe extern "C" fn lookup(mut filesys: *mut libc::c_char) -> *mut fs_info {
  let mut fs: *mut fs_info = 0 as *mut fs_info;
  fs = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).filesys_info;
  while !fs.is_null() {
    if strcmp(filesys, (*fs).device) == 0i32
      || !(*fs).mountpt.is_null() && strcmp(filesys, (*fs).mountpt) == 0i32
    {
      break;
    }
    fs = (*fs).next
  }
  return fs;
}
/*
 * Send a signal to all outstanding fsck child processes
 */
unsafe extern "C" fn kill_all_if_got_signal() {
  let mut inst: *mut fsck_instance = 0 as *mut fsck_instance;
  if bb_got_signal == 0
    || (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).kill_sent as libc::c_int != 0
  {
    return;
  }
  inst = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).instance_list;
  while !inst.is_null() {
    if !((*inst).flags & 1i32 != 0) {
      kill((*inst).pid, 15i32);
    }
    inst = (*inst).next
  }
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).kill_sent = 1i32 as smallint;
}
/*
 * Wait for one child process to exit; when it does, unlink it from
 * the list of executing child processes, free, and return its exit status.
 * If there is no exited child, return -1.
 */
unsafe extern "C" fn wait_one(mut flags: libc::c_int) -> libc::c_int {
  let mut status: libc::c_int = 0;
  let mut exitcode: libc::c_int = 0;
  let mut inst: *mut fsck_instance = 0 as *mut fsck_instance;
  let mut prev: *mut fsck_instance = 0 as *mut fsck_instance;
  let mut pid: pid_t = 0;
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .instance_list
    .is_null()
  {
    return -1i32;
  }
  's_25: loop
  /* if (G.noexecute) { already returned -1; } */
  {
    pid = waitpid(-1i32, &mut status, flags);
    kill_all_if_got_signal();
    if pid == 0i32 {
      /* flags == WNOHANG and no children exited */
      return -1i32;
    }
    if pid < 0i32 {
      if *bb_errno == 4i32 {
        continue;
      }
      if *bb_errno == 10i32 {
        /* paranoia */
        bb_simple_error_msg(b"wait: no more children\x00" as *const u8 as *const libc::c_char);
        return -1i32;
      }
      bb_simple_perror_msg(b"wait\x00" as *const u8 as *const libc::c_char);
    } else {
      prev = 0 as *mut fsck_instance;
      inst = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).instance_list;
      loop {
        if (*inst).pid == pid {
          break 's_25;
        }
        prev = inst;
        inst = (*inst).next;
        if inst.is_null() {
          break;
        }
      }
    }
  }
  exitcode = (status & 0xff00i32) >> 8i32;
  if ((status & 0x7fi32) + 1i32) as libc::c_schar as libc::c_int >> 1i32 > 0i32 {
    let mut sig: libc::c_uint = 0;
    sig = (status & 0x7fi32) as libc::c_uint;
    exitcode = 4i32;
    if sig != 2i32 as libc::c_uint {
      printf(
        b"Warning: %s %s terminated by signal %u\n\x00" as *const u8 as *const libc::c_char,
        (*inst).prog,
        (*inst).device,
        sig,
      );
      exitcode = 8i32
    }
  }
  if !prev.is_null() {
    (*prev).next = (*inst).next
  } else {
    let ref mut fresh3 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).instance_list;
    *fresh3 = (*inst).next
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).verbose > 1i32 {
    printf(
      b"Finished with %s (exit status %u)\n\x00" as *const u8 as *const libc::c_char,
      (*inst).device,
      exitcode,
    );
  }
  let ref mut fresh4 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).num_running;
  *fresh4 -= 1;
  free_instance(inst);
  return exitcode;
}
unsafe extern "C" fn wait_many(mut flags: libc::c_int) -> libc::c_int {
  let mut exit_status: libc::c_int = 0;
  let mut global_status: libc::c_int = 0i32;
  let mut wait_flags: libc::c_int = 0i32;
  loop {
    exit_status = wait_one(wait_flags);
    if !(exit_status != -1i32) {
      break;
    }
    global_status |= exit_status;
    wait_flags |= flags
  }
  return global_status;
}
/*
 * Execute a particular fsck program, and link it into the list of
 * child processes we are waiting for.
 */
unsafe extern "C" fn execute(
  mut type_0: *const libc::c_char,
  mut device: *const libc::c_char,
  mut mntpt: *const libc::c_char,
)
/*, int interactive */
{
  let mut i: libc::c_int = 0;
  let mut inst: *mut fsck_instance = 0 as *mut fsck_instance;
  let mut pid: pid_t = 0;
  let ref mut fresh5 = *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .args
    .offset(0);
  *fresh5 = xasprintf(b"fsck.%s\x00" as *const u8 as *const libc::c_char, type_0);
  let ref mut fresh6 = *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .args
    .offset(((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).num_args - 2i32) as isize);
  *fresh6 = device as *mut libc::c_char;
  /* G.args[G.num_args - 1] = NULL; - already is */
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).verbose != 0
    || (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).noexecute as libc::c_int != 0
  {
    printf(
      b"[%s (%d) -- %s]\x00" as *const u8 as *const libc::c_char,
      *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .args
        .offset(0),
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).num_running,
      if !mntpt.is_null() { mntpt } else { device },
    );
    i = 0i32;
    while !(*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .args
      .offset(i as isize))
    .is_null()
    {
      printf(
        b" %s\x00" as *const u8 as *const libc::c_char,
        *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
          .args
          .offset(i as isize),
      );
      i += 1
    }
    bb_putchar('\n' as i32);
  }
  /* Fork and execute the correct program. */
  pid = -1i32;
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).noexecute == 0 {
    pid = spawn((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).args);
    if pid < 0i32 {
      bb_simple_perror_msg(
        *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
          .args
          .offset(0),
      );
    }
  }
  /* No child, so don't record an instance */
  if pid <= 0i32 {
    free(
      *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .args
        .offset(0) as *mut libc::c_void,
    );
    return;
  }
  inst = xzalloc(::std::mem::size_of::<fsck_instance>() as libc::c_ulong) as *mut fsck_instance;
  (*inst).pid = pid;
  (*inst).prog = *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .args
    .offset(0);
  (*inst).device = xstrdup(device);
  (*inst).base_device = base_device(device);
  /* Add to the list of running fsck's.
   * (was adding to the end, but adding to the front is simpler...) */
  (*inst).next = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).instance_list;
  let ref mut fresh7 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).instance_list;
  *fresh7 = inst;
}
/*
 * Run the fsck program on a particular device
 *
 * If the type is specified using -t, and it isn't prefixed with "no"
 * (as in "noext2") and only one filesystem type is specified, then
 * use that type regardless of what is specified in /etc/fstab.
 *
 * If the type isn't specified by the user, then use either the type
 * specified in /etc/fstab, or "auto".
 */
unsafe extern "C" fn fsck_device(mut fs: *mut fs_info)
/*, int interactive */
{
  let mut type_0: *const libc::c_char = 0 as *const libc::c_char;
  if strcmp(
    (*fs).type_0,
    b"auto\x00" as *const u8 as *const libc::c_char,
  ) != 0i32
  {
    type_0 = (*fs).type_0;
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).verbose > 2i32 {
      printf(
        b"using filesystem type \'%s\' %s\n\x00" as *const u8 as *const libc::c_char,
        type_0,
        b"from fstab\x00" as *const u8 as *const libc::c_char,
      );
    }
  } else if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .fstype
    .is_null()
    && (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .fstype
      .offset(0) as libc::c_int
      != 'n' as i32
      || *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .fstype
        .offset(1) as libc::c_int
        != 'o' as i32)
    && is_prefixed_with(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).fstype,
      b"opts=\x00" as *const u8 as *const libc::c_char,
    )
    .is_null()
    && is_prefixed_with(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).fstype,
      b"loop\x00" as *const u8 as *const libc::c_char,
    )
    .is_null()
    && strchr(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).fstype,
      ',' as i32,
    )
    .is_null()
  {
    type_0 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).fstype;
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).verbose > 2i32 {
      printf(
        b"using filesystem type \'%s\' %s\n\x00" as *const u8 as *const libc::c_char,
        type_0,
        b"from -t\x00" as *const u8 as *const libc::c_char,
      );
    }
  } else {
    type_0 = b"auto\x00" as *const u8 as *const libc::c_char;
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).verbose > 2i32 {
      printf(
        b"using filesystem type \'%s\' %s\n\x00" as *const u8 as *const libc::c_char,
        type_0,
        b"(default)\x00" as *const u8 as *const libc::c_char,
      );
    }
  }
  let ref mut fresh8 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).num_running;
  *fresh8 += 1;
  execute(type_0, (*fs).device, (*fs).mountpt);
}
/*
 * Returns TRUE if a partition on the same disk is already being
 * checked.
 */
unsafe extern "C" fn device_already_active(mut device: *mut libc::c_char) -> libc::c_int {
  let mut inst: *mut fsck_instance = 0 as *mut fsck_instance;
  let mut base: *mut libc::c_char = 0 as *mut libc::c_char;
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).force_all_parallel != 0 {
    return 0i32;
  }
  /* Don't check a soft raid disk with any other disk */
  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .instance_list
    .is_null()
    && (!is_prefixed_with(
      (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).instance_list).device,
      b"/dev/md\x00" as *const u8 as *const libc::c_char,
    )
    .is_null()
      || !is_prefixed_with(device, b"/dev/md\x00" as *const u8 as *const libc::c_char).is_null())
  {
    return 1i32;
  }
  base = base_device(device);
  /*
   * If we don't know the base device, assume that the device is
   * already active if there are any fsck instances running.
   */
  if base.is_null() {
    return ((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).instance_list
      != 0 as *mut libc::c_void as *mut fsck_instance) as libc::c_int;
  }
  inst = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).instance_list;
  while !inst.is_null() {
    if (*inst).base_device.is_null() || strcmp(base, (*inst).base_device) == 0i32 {
      free(base as *mut libc::c_void);
      return 1i32;
    }
    inst = (*inst).next
  }
  free(base as *mut libc::c_void);
  return 0i32;
}
/*
 * This function returns true if a particular option appears in a
 * comma-delimited options list
 */
unsafe extern "C" fn opt_in_list(
  mut opt: *mut libc::c_char,
  mut optlist: *mut libc::c_char,
) -> libc::c_int {
  let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut len: libc::c_int = 0;
  if optlist.is_null() {
    return 0i32;
  }
  len = strlen(opt) as libc::c_int;
  s = optlist.offset(-1);
  loop {
    s = strstr(s.offset(1), opt);
    if s.is_null() {
      return 0i32;
    }
    /* neither "opt.." nor "xxx,opt.."? */
    if s != optlist && *s.offset(-1i32 as isize) as libc::c_int != ',' as i32 {
      continue;
    }
    /* neither "..opt" nor "..opt,xxx"? */
    if *s.offset(len as isize) as libc::c_int != '\u{0}' as i32
      && *s.offset(len as isize) as libc::c_int != ',' as i32
    {
      continue;
    }
    return 1i32;
  }
}
/* See if the filesystem matches the criteria given by the -t option */
unsafe extern "C" fn fs_match(mut fs: *mut fs_info) -> libc::c_int {
  let mut n: libc::c_int = 0;
  let mut ret: libc::c_int = 0;
  let mut checked_type: libc::c_int = 0;
  let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .fs_type_list
    .is_null()
  {
    return 1i32;
  }
  ret = 0i32;
  checked_type = 0i32;
  n = 0i32;
  loop {
    cp = *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .fs_type_list
      .offset(n as isize);
    if cp.is_null() {
      break;
    }
    match *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .fs_type_flag
      .offset(n as isize) as libc::c_int
    {
      0 => {
        checked_type += 1;
        if strcmp(cp, (*fs).type_0) == 0i32 {
          ret = 1i32
        }
      }
      2 => {
        if opt_in_list(cp, (*fs).opts) != 0 {
          return 0i32;
        }
      }
      1 => {
        if opt_in_list(cp, (*fs).opts) == 0 {
          return 0i32;
        }
      }
      _ => {}
    }
    n += 1
  }
  if checked_type == 0i32 {
    return 1i32;
  }
  return if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).fs_type_negated as libc::c_int != 0
  {
    (ret == 0) as libc::c_int
  } else {
    ret
  };
}
/* Check if we should ignore this filesystem. */
unsafe extern "C" fn ignore(mut fs: *mut fs_info) -> libc::c_int {
  /*
   * If the pass number is 0, ignore it.
   */
  if (*fs).passno == 0i32 {
    return 1i32;
  }
  /*
   * If a specific fstype is specified, and it doesn't match,
   * ignore it.
   */
  if fs_match(fs) == 0 {
    return 1i32;
  }
  /* Are we ignoring this type? */
  if index_in_strings(ignored_types.as_ptr(), (*fs).type_0) >= 0i32 {
    return 1i32;
  }
  /* We can and want to check this file system type. */
  return 0i32;
}
/* Check all file systems, using the /etc/fstab table. */
unsafe extern "C" fn check_all() -> libc::c_int {
  let mut fs: *mut fs_info = 0 as *mut fs_info;
  let mut status: libc::c_int = 0i32;
  let mut not_done_yet: smallint = 0;
  let mut pass_done: smallint = 0;
  let mut passno: libc::c_int = 0;
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).verbose != 0 {
    puts(b"Checking all filesystems\x00" as *const u8 as *const libc::c_char);
  }
  /*
   * Do an initial scan over the filesystem; mark filesystems
   * which should be ignored as done, and resolve any "auto"
   * filesystem types (done as a side-effect of calling ignore()).
   */
  fs = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).filesys_info;
  while !fs.is_null() {
    if ignore(fs) != 0 {
      (*fs).flags |= 1i32
    }
    fs = (*fs).next
  }
  /*
   * Find and check the root filesystem.
   */
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).parallel_root == 0 {
    fs = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).filesys_info;
    while !fs.is_null() {
      if *(*fs).mountpt.offset(0) as libc::c_int == '/' as i32 && *(*fs).mountpt.offset(1) == 0 {
        if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).skip_root == 0 && ignore(fs) == 0 {
          fsck_device(fs);
          status |= wait_many(0i32);
          if status > 1i32 {
            return status;
          }
        }
        (*fs).flags |= 1i32;
        break;
      } else {
        fs = (*fs).next
      }
    }
  }
  /*
   * This is for the bone-headed user who has root
   * filesystem listed twice.
   * "Skip root" will skip _all_ root entries.
   */
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).skip_root != 0 {
    fs = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).filesys_info;
    while !fs.is_null() {
      if *(*fs).mountpt.offset(0) as libc::c_int == '/' as i32 && *(*fs).mountpt.offset(1) == 0 {
        (*fs).flags |= 1i32
      }
      fs = (*fs).next
    }
  }
  not_done_yet = 1i32 as smallint;
  passno = 1i32;
  while not_done_yet != 0 {
    not_done_yet = 0i32 as smallint;
    pass_done = 1i32 as smallint;
    fs = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).filesys_info;
    while !fs.is_null() {
      if bb_got_signal != 0 {
        break;
      }
      if !((*fs).flags & 1i32 != 0) {
        /*
         * If the filesystem's pass number is higher
         * than the current pass number, then we didn't
         * do it yet.
         */
        if (*fs).passno > passno {
          not_done_yet = 1i32 as smallint
        } else if device_already_active((*fs).device) != 0 {
          pass_done = 0i32 as smallint
        } else {
          /*
           * If a filesystem on a particular device has
           * already been spawned, then we need to defer
           * this to another pass.
           */
          /*
           * Spawn off the fsck process
           */
          fsck_device(fs);
          (*fs).flags |= 1i32;
          /*
           * Only do one filesystem at a time, or if we
           * have a limit on the number of fsck's extant
           * at one time, apply that limit.
           */
          if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).serialize as libc::c_int != 0
            || (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).num_running
              >= (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).max_running
          {
            pass_done = 0i32 as smallint;
            break;
          }
        }
      }
      fs = (*fs).next
    }
    if bb_got_signal != 0 {
      break;
    }
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).verbose > 1i32 {
      printf(
        b"--waiting-- (pass %d)\n\x00" as *const u8 as *const libc::c_char,
        passno,
      );
    }
    status |= wait_many(if pass_done as libc::c_int != 0 {
      0i32
    } else {
      1i32
    });
    if pass_done != 0 {
      if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).verbose > 1i32 {
        puts(b"----------------------------------\x00" as *const u8 as *const libc::c_char);
      }
      passno += 1
    } else {
      not_done_yet = 1i32 as smallint
    }
  }
  kill_all_if_got_signal();
  status |= wait_many(1i32);
  return status;
}
/*
 * Deal with the fsck -t argument.
 * Huh, for mount "-t novfat,nfs" means "neither vfat nor nfs"!
 * Why here we require "-t novfat,nonfs" ??
 */
unsafe extern "C" fn compile_fs_type(mut fs_type: *mut libc::c_char) {
  let mut s: *mut libc::c_char = 0 as *mut libc::c_char; /* not yet known is it negated or not */
  let mut num: libc::c_int = 2i32;
  let mut negate: smallint = 0;
  s = fs_type;
  loop {
    s = strchr(s, ',' as i32);
    if s.is_null() {
      break;
    }
    num += 1;
    s = s.offset(1)
  }
  let ref mut fresh9 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).fs_type_list;
  *fresh9 = xzalloc(
    (num as libc::c_ulong)
      .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
  ) as *mut *mut libc::c_char;
  let ref mut fresh10 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).fs_type_flag;
  *fresh10 =
    xzalloc((num as libc::c_ulong).wrapping_mul(::std::mem::size_of::<u8>() as libc::c_ulong))
      as *mut u8;
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).fs_type_negated = -1i32 as smallint;
  num = 0i32;
  s = fs_type;
  let mut current_block_28: u64;
  loop {
    let mut comma: *mut libc::c_char = 0 as *mut libc::c_char;
    negate = 0i32 as smallint;
    if *s.offset(0) as libc::c_int == 'n' as i32 && *s.offset(1) as libc::c_int == 'o' as i32 {
      /* "no.." */
      s = s.offset(2);
      negate = 1i32 as smallint
    } else if *s.offset(0) as libc::c_int == '!' as i32 {
      s = s.offset(1);
      negate = 1i32 as smallint
    }
    if strcmp(s, b"loop\x00" as *const u8 as *const libc::c_char) == 0i32 {
      current_block_28 = 11934833676784627535;
    } else if !is_prefixed_with(s, b"opts=\x00" as *const u8 as *const libc::c_char).is_null() {
      s = s.offset(5);
      current_block_28 = 11934833676784627535;
    } else {
      if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).fs_type_negated as libc::c_int == -1i32
      {
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).fs_type_negated = negate
      }
      if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).fs_type_negated as libc::c_int
        != negate as libc::c_int
      {
        bb_simple_error_msg_and_die(b"either all or none of the filesystem types passed to -t must be prefixed with \'no\' or \'!\'\x00"
                                                as *const u8 as
                                                *const libc::c_char);
      }
      current_block_28 = 16924917904204750491;
    }
    match current_block_28 {
      11934833676784627535 =>
      /* loop is really short-hand for opts=loop */
      {
        *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
          .fs_type_flag
          .offset(num as isize) = if negate as libc::c_int != 0 {
          2i32
        } else {
          1i32
        } as u8
      }
      _ => {}
    }
    comma = strchrnul(s, ',' as i32);
    let fresh11 = num;
    num = num + 1;
    let ref mut fresh12 = *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .fs_type_list
      .offset(fresh11 as isize);
    *fresh12 = xstrndup(
      s,
      comma.wrapping_offset_from(s) as libc::c_long as libc::c_int,
    );
    if *comma as libc::c_int == '\u{0}' as i32 {
      break;
    }
    s = comma.offset(1)
  }
}
unsafe extern "C" fn new_args() -> *mut *mut libc::c_char {
  let ref mut fresh13 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).args;
  *fresh13 = xrealloc_vector_helper(
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).args as *mut libc::c_void,
    ((::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong) << 8i32)
      .wrapping_add(2i32 as libc::c_ulong) as libc::c_uint,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).num_args,
  ) as *mut *mut libc::c_char;
  let ref mut fresh14 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).num_args;
  let fresh15 = *fresh14;
  *fresh14 = *fresh14 + 1;
  return &mut *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .args
    .offset(fresh15 as isize) as *mut *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn fsck_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut i: libc::c_int = 0;
  let mut status: libc::c_int = 0;
  /*int interactive;*/
  let mut fs: *mut fs_info = 0 as *mut fs_info;
  let mut fstab: *const libc::c_char = 0 as *const libc::c_char;
  let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut devices: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  let mut num_devices: libc::c_int = 0;
  let mut opts_for_fsck: smallint = 0;
  let mut doall: smallint = 0;
  let mut notitle: smallint = 0;
  /* we want wait() to be interruptible */
  signal_no_SA_RESTART_empty_mask(
    2i32,
    Some(record_signo as unsafe extern "C" fn(_: libc::c_int) -> ()),
  ); /* G.args[0] = NULL, will be replaced by fsck.<type> */
  signal_no_SA_RESTART_empty_mask(
    15i32,
    Some(record_signo as unsafe extern "C" fn(_: libc::c_int) -> ()),
  );
  setbuf(stdout, 0 as *mut libc::c_char);
  notitle = 0i32 as smallint;
  doall = notitle;
  opts_for_fsck = doall;
  devices = 0 as *mut *mut libc::c_char;
  num_devices = 0i32;
  new_args();
  loop
  /* G.instance_list = NULL; - in bss, so already zeroed */
  {
    argv = argv.offset(1);
    if (*argv).is_null() {
      break;
    }
    let mut j: libc::c_int = 0;
    let mut optpos: libc::c_int = 0;
    let mut options: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut arg: *mut libc::c_char = *argv;
    /* "/dev/blk" or "/path" or "UUID=xxx" or "LABEL=xxx" */
    if *arg.offset(0) as libc::c_int == '/' as i32 && opts_for_fsck == 0
      || !strchr(arg, '=' as i32).is_null()
    {
      // FIXME: must check that arg is a blkdev, or resolve
      // "/path", "UUID=xxx" or "LABEL=xxx" into block device name
      // ("UUID=xxx"/"LABEL=xxx" can probably shifted to fsck.auto duties)
      devices = xrealloc_vector_helper(
        devices as *mut libc::c_void,
        ((::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong) << 8i32)
          .wrapping_add(2i32 as libc::c_ulong) as libc::c_uint,
        num_devices,
      ) as *mut *mut libc::c_char;
      let fresh16 = num_devices;
      num_devices = num_devices + 1;
      let ref mut fresh17 = *devices.offset(fresh16 as isize);
      *fresh17 = arg
    } else if *arg.offset(0) as libc::c_int != '-' as i32 || opts_for_fsck as libc::c_int != 0 {
      let ref mut fresh18 = *new_args();
      *fresh18 = arg
    } else if *arg.offset(1).offset(0) as libc::c_int == '-' as i32 && *arg.offset(1).offset(1) == 0
    {
      /* "--" ? */
      opts_for_fsck = 1i32 as smallint
    } else {
      optpos = 0i32;
      options = 0 as *mut libc::c_char;
      j = 1i32;
      while *arg.offset(j as isize) != 0 {
        match *arg.offset(j as isize) as libc::c_int {
          65 => doall = 1i32 as smallint,
          86 => {
            let ref mut fresh19 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).verbose;
            *fresh19 += 1
          }
          78 => (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).noexecute = 1i32 as smallint,
          82 => (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).skip_root = 1i32 as smallint,
          84 => notitle = 1i32 as smallint,
          80 => {
            /*			case 'M':
            like_mount = 1;
            break; */
            (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).parallel_root = 1i32 as smallint
          }
          115 => (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).serialize = 1i32 as smallint,
          116 => {
            if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
              .fstype
              .is_null()
            {
              bb_show_usage();
            }
            j += 1;
            if *arg.offset(j as isize) != 0 {
              tmp = &mut *arg.offset(j as isize) as *mut libc::c_char
            } else {
              argv = argv.offset(1);
              if !(*argv).is_null() {
                tmp = *argv
              } else {
                bb_show_usage();
              }
            }
            let ref mut fresh20 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).fstype;
            *fresh20 = xstrdup(tmp);
            compile_fs_type((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).fstype);
            break;
          }
          63 => {
            bb_show_usage();
          }
          _ => {
            optpos += 1;
            /* one extra for '\0' */
            options = xrealloc(options as *mut libc::c_void, (optpos + 2i32) as size_t)
              as *mut libc::c_char; /* G.args[G.num_args - 2] will be replaced by <device> */
            *options.offset(optpos as isize) = *arg.offset(j as isize)
          }
        } /* G.args[G.num_args - 1] is the last, NULL element */
        j += 1
      }
      if optpos != 0 {
        *options.offset(0) = '-' as i32 as libc::c_char;
        *options.offset((optpos + 1i32) as isize) = '\u{0}' as i32 as libc::c_char;
        let ref mut fresh21 = *new_args();
        *fresh21 = options
      }
    }
  }
  if !getenv(b"FSCK_FORCE_ALL_PARALLEL\x00" as *const u8 as *const libc::c_char).is_null() {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).force_all_parallel = 1i32 as smallint
  }
  tmp = getenv(b"FSCK_MAX_INST\x00" as *const u8 as *const libc::c_char);
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).max_running = 2147483647i32;
  if !tmp.is_null() {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).max_running = xatoi(tmp)
  }
  new_args();
  new_args();
  if notitle == 0 {
    puts(b"fsck (busybox 1.32.0.git)\x00" as *const u8 as *const libc::c_char);
  }
  /* Even plain "fsck /dev/hda1" needs fstab to get fs type,
   * so we are scanning it anyway */
  fstab = getenv(b"FSTAB_FILE\x00" as *const u8 as *const libc::c_char);
  if fstab.is_null() {
    fstab = b"/etc/fstab\x00" as *const u8 as *const libc::c_char
  }
  load_fs_info(fstab);
  /*interactive = (num_devices == 1) | G.serialize;*/
  if num_devices == 0i32 {
    /*interactive =*/
    doall = 1i32 as smallint;
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).serialize = doall
  }
  if doall != 0 {
    return check_all();
  }
  status = 0i32;
  i = 0i32;
  while i < num_devices {
    if bb_got_signal != 0 {
      kill_all_if_got_signal();
      break;
    } else {
      fs = lookup(*devices.offset(i as isize));
      if fs.is_null() {
        fs = create_fs_device(
          *devices.offset(i as isize),
          b"\x00" as *const u8 as *const libc::c_char,
          b"auto\x00" as *const u8 as *const libc::c_char,
          0 as *const libc::c_char,
          -1i32,
        )
      }
      fsck_device(fs);
      if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).serialize as libc::c_int != 0
        || (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).num_running
          >= (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).max_running
      {
        let mut exit_status: libc::c_int = wait_one(0i32);
        if exit_status >= 0i32 {
          status |= exit_status
        }
        if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).verbose > 1i32 {
          puts(b"----------------------------------\x00" as *const u8 as *const libc::c_char);
        }
      }
      i += 1
    }
  }
  status |= wait_many(0i32);
  return status;
}
