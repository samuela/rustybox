use libc;




extern "C" {
  #[no_mangle]
  fn putenv(__string: *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn putchar_unlocked(__c: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn puts(__s: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn mmap(
    __addr: *mut libc::c_void,
    __len: size_t,
    __prot: libc::c_int,
    __flags: libc::c_int,
    __fd: libc::c_int,
    __offset: off64_t,
  ) -> *mut libc::c_void;
  #[no_mangle]
  fn munmap(__addr: *mut libc::c_void, __len: size_t) -> libc::c_int;
  #[no_mangle]
  fn bb_unsetenv(key: *const libc::c_char);
  #[no_mangle]
  fn create_and_bind_to_netlink(
    proto: libc::c_int,
    grp: libc::c_int,
    rcvbuf: libc::c_uint,
  ) -> libc::c_int;
  #[no_mangle]
  fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn spawn_and_wait(argv: *mut *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
}
use libc::off64_t;

use crate::librb::size_t;
use libc::ssize_t;
pub type C2RustUnnamed = libc::c_uint;
pub const MAX_ENV: C2RustUnnamed = 127;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const RCVBUF: C2RustUnnamed_0 = 2097152;
#[no_mangle]
pub unsafe extern "C" fn uevent_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut fd: libc::c_int = 0;
  argv = argv.offset(1);
  /* sizeof(env[0]) instead of sizeof(char*)
   * makes gcc-6.3.0 emit "strict-aliasing" warning.
   */
  // Subscribe for UEVENT kernel messages.
  // Without a sufficiently big RCVBUF, a ton of simultaneous events
  // can trigger ENOBUFS on read, which is unrecoverable.
  // Reproducer:
  //	uevent mdev &
  // 	find /sys -name uevent -exec sh -c 'echo add >"{}"' ';'
  fd = create_and_bind_to_netlink(15i32, 1i32 << 0i32, RCVBUF as libc::c_int as libc::c_uint);
  loop {
    let mut netbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: ssize_t = 0;
    let mut idx: libc::c_int = 0;
    // In many cases, a system sits for *days* waiting
    // for a new uevent notification to come in.
    // We use a fresh mmap so that buffer is not allocated
    // until kernel actually starts filling it.
    netbuf = mmap(
      0 as *mut libc::c_void,
      (16i32 * 1024i32) as size_t,
      0x1i32 | 0x2i32,
      0x2i32 | 0x20i32,
      -1i32,
      0i32 as off64_t,
    ) as *mut libc::c_char;
    if netbuf == -1i32 as *mut libc::c_void as *mut libc::c_char {
      bb_simple_perror_msg_and_die(b"mmap\x00" as *const u8 as *const libc::c_char);
    }
    // Here we block, possibly for a very long time
    len = safe_read(
      fd,
      netbuf as *mut libc::c_void,
      (16i32 * 1024i32 - 1i32) as size_t,
    );
    if len < 0 {
      bb_simple_perror_msg_and_die(b"read\x00" as *const u8 as *const libc::c_char);
    }
    end = netbuf.offset(len as isize);
    *end = '\u{0}' as i32 as libc::c_char;
    // Each netlink message starts with "ACTION@/path"
    // (which we currently ignore),
    // followed by environment variables.
    if (*argv.offset(0)).is_null() {
      putchar_unlocked('\n' as i32);
    }
    idx = 0i32;
    s = netbuf;
    while s < end {
      if (*argv.offset(0)).is_null() {
        puts(s);
      }
      if !strchr(s, '=' as i32).is_null() && idx < MAX_ENV as libc::c_int {
        let fresh0 = idx;
        idx = idx + 1;
        let ref mut fresh1 =
          *(bb_common_bufsiz1.as_mut_ptr() as *mut *mut libc::c_char).offset(fresh0 as isize);
        *fresh1 = s
      }
      s = s.offset(strlen(s).wrapping_add(1i32 as libc::c_ulong) as isize)
    }
    let ref mut fresh2 =
      *(bb_common_bufsiz1.as_mut_ptr() as *mut *mut libc::c_char).offset(idx as isize);
    *fresh2 = 0 as *mut libc::c_char;
    if !(*argv.offset(0)).is_null() {
      idx = 0i32;
      while !(*(bb_common_bufsiz1.as_mut_ptr() as *mut *mut libc::c_char).offset(idx as isize))
        .is_null()
      {
        let fresh3 = idx;
        idx = idx + 1;
        putenv(*(bb_common_bufsiz1.as_mut_ptr() as *mut *mut libc::c_char).offset(fresh3 as isize));
      }
      spawn_and_wait(argv);
      idx = 0i32;
      while !(*(bb_common_bufsiz1.as_mut_ptr() as *mut *mut libc::c_char).offset(idx as isize))
        .is_null()
      {
        let fresh4 = idx;
        idx = idx + 1;
        bb_unsetenv(
          *(bb_common_bufsiz1.as_mut_ptr() as *mut *mut libc::c_char).offset(fresh4 as isize),
        );
      }
    }
    munmap(netbuf as *mut libc::c_void, (16i32 * 1024i32) as size_t);
  }
  // not reached
}
