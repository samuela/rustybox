use libc;
extern "C" {
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn puts(__s: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xatoull(str: *const libc::c_char) -> libc::c_ulonglong;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn query_loop(device: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn get_free_loop() -> libc::c_int;
  #[no_mangle]
  fn del_loop(device: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn set_loop(
    devname: *mut *mut libc::c_char,
    file: *const libc::c_char,
    offset: libc::c_ulonglong,
    flags: libc::c_uint,
  ) -> libc::c_int;
  #[no_mangle]
  fn bb_xioctl(
    fd: libc::c_int,
    request: libc::c_uint,
    argp: *mut libc::c_void,
    ioctl_name: *const libc::c_char,
  ) -> libc::c_int;
}
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub const OPT_r: C2RustUnnamed = 64;
pub const OPT_P: C2RustUnnamed = 4;
pub const OPT_f: C2RustUnnamed = 16;
pub const OPT_o: C2RustUnnamed = 8;
pub const OPT_a: C2RustUnnamed = 32;
pub const OPT_d: C2RustUnnamed = 2;
pub const OPT_c: C2RustUnnamed = 1;
pub type C2RustUnnamed = libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn losetup_main(
  mut argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut opt: libc::c_uint = 0;
  let mut opt_o: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut dev: [libc::c_char; 23] = [0; 23];
  opt = getopt32(
    argv,
    b"^cdPo:far\x00?2:d--Pofar:a--Pofr\x00" as *const u8 as *const libc::c_char,
    &mut opt_o as *mut *mut libc::c_char,
  );
  argv = argv.offset(optind as isize);
  /* LOOPDEV */
  if opt == 0 && !(*argv.offset(0)).is_null() && (*argv.offset(1)).is_null() {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    s = query_loop(*argv.offset(0));
    if s.is_null() {
      bb_simple_perror_msg_and_die(*argv.offset(0));
    }
    printf(
      b"%s: %s\n\x00" as *const u8 as *const libc::c_char,
      *argv.offset(0),
      s,
    );
    return 0i32;
  }
  /* -c LOOPDEV */
  if opt == OPT_c as libc::c_int as libc::c_uint && !(*argv.offset(0)).is_null() {
    let mut fd: libc::c_int = xopen(*argv.offset(0), 0i32);
    bb_xioctl(
      fd,
      0x4c07i32 as libc::c_uint,
      0 as *mut libc::c_void,
      b"LOOP_SET_CAPACITY\x00" as *const u8 as *const libc::c_char,
    );
    return 0i32;
  }
  /* -d LOOPDEV */
  if opt == OPT_d as libc::c_int as libc::c_uint && !(*argv.offset(0)).is_null() {
    if del_loop(*argv.offset(0)) != 0 {
      bb_simple_perror_msg_and_die(*argv.offset(0));
    }
    return 0i32;
  }
  /* -a */
  if opt == OPT_a as libc::c_int as libc::c_uint {
    let mut n: libc::c_int = 0;
    n = 0i32;
    while n < 1023i32 {
      let mut s_0: *mut libc::c_char = 0 as *mut libc::c_char;
      sprintf(
        dev.as_mut_ptr(),
        b"/dev/loop%u\x00" as *const u8 as *const libc::c_char,
        n,
      );
      s_0 = query_loop(dev.as_mut_ptr());
      if !s_0.is_null() {
        printf(
          b"%s: %s\n\x00" as *const u8 as *const libc::c_char,
          dev.as_mut_ptr(),
          s_0,
        );
        free(s_0 as *mut libc::c_void);
      }
      n += 1
    }
    return 0i32;
  }
  /* contains -f */
  if opt & OPT_f as libc::c_int as libc::c_uint != 0 {
    let mut s_1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n_0: libc::c_int = 0;
    n_0 = get_free_loop();
    if n_0 == -1i32 {
      bb_simple_error_msg_and_die(b"no free loop devices\x00" as *const u8 as *const libc::c_char);
    }
    if n_0 < 0i32 {
      /* n == -2: no /dev/loop-control, use legacy method */
      n_0 = 0i32
    }
    loop
    /* or: n >= 0: the number of next free loopdev, just verify it */
    {
      if n_0 > 1023i32 {
        bb_simple_error_msg_and_die(
          b"no free loop devices\x00" as *const u8 as *const libc::c_char,
        );
      }
      let fresh0 = n_0;
      n_0 = n_0 + 1;
      sprintf(
        dev.as_mut_ptr(),
        b"/dev/loop%u\x00" as *const u8 as *const libc::c_char,
        fresh0,
      );
      s_1 = query_loop(dev.as_mut_ptr());
      free(s_1 as *mut libc::c_void);
      if s_1.is_null() {
        break;
      }
    }
    /* now: dev is next free "/dev/loopN" */
    if opt == OPT_f as libc::c_int as libc::c_uint && (*argv.offset(0)).is_null() {
      puts(dev.as_mut_ptr());
      return 0i32;
    }
  }
  /* [-rP] [-o OFS] {-f|LOOPDEV} FILE */
  if !(*argv.offset(0)).is_null()
    && (opt & OPT_f as libc::c_int as libc::c_uint != 0 || !(*argv.offset(1)).is_null())
  {
    let mut offset: libc::c_ulonglong = 0i32 as libc::c_ulonglong;
    let mut d: *mut libc::c_char = dev.as_mut_ptr();
    if opt & OPT_o as libc::c_int as libc::c_uint != 0 {
      offset = xatoull(opt_o)
    }
    if opt & OPT_f as libc::c_int as libc::c_uint == 0 {
      let fresh1 = argv;
      argv = argv.offset(1);
      d = *fresh1
    }
    if !(*argv.offset(0)).is_null() {
      let mut flags: libc::c_uint = if opt & OPT_r as libc::c_int as libc::c_uint != 0 {
        1i32
      } else {
        0i32
      } as libc::c_uint;
      if opt & OPT_P as libc::c_int as libc::c_uint != 0 {
        flags |= 8i32 as libc::c_uint
      }
      if set_loop(&mut d, *argv.offset(0), offset, flags) < 0i32 {
        bb_simple_perror_msg_and_die(*argv.offset(0));
      }
      return 0i32;
    }
  }
  /* TODO: util-linux 2.28 shows this when run w/o params:
   * NAME       SIZELIMIT OFFSET AUTOCLEAR RO BACK-FILE     DIO
   * /dev/loop0         0      0         1  0 /PATH/TO/FILE   0
   *
   * implemented by reading /sys:
   *
   * open("/sys/block", O_RDONLY|O_NONBLOCK|O_DIRECTORY|O_CLOEXEC) = 3
   * newfstatat(3, "loop0/loop/backing_file", {st_mode=S_IFREG|0444, st_size=4096, ...}, 0) = 0
   * stat("/dev/loop0", {st_mode=S_IFBLK|0660, st_rdev=makedev(7, 0), ...}) = 0
   * open("/sys/dev/block/7:0/loop/offset", O_RDONLY|O_CLOEXEC) = 5
   * read(5, "0\n", 4096)                    = 2
   * open("/sys/dev/block/7:0/loop/sizelimit", O_RDONLY|O_CLOEXEC) = 5
   * read(5, "0\n", 4096)                    = 2
   * open("/sys/dev/block/7:0/loop/offset", O_RDONLY|O_CLOEXEC) = 5
   * read(5, "0\n", 4096)                    = 2
   * open("/sys/dev/block/7:0/loop/autoclear", O_RDONLY|O_CLOEXEC) = 5
   * read(5, "1\n", 4096)                    = 2
   * open("/sys/dev/block/7:0/ro", O_RDONLY|O_CLOEXEC)     = 5
   * read(5, "0\n", 4096)                    = 2
   * open("/sys/dev/block/7:0/loop/backing_file", O_RDONLY|O_CLOEXEC) = 5
   * read(5, "/PATH/TO/FILE", 4096) = 37
   * open("/sys/dev/block/7:0/loop/dio", O_RDONLY|O_CLOEXEC) = 5
   * read(5, "0\n", 4096)                    = 2
   */
  bb_show_usage();
  /* does not return */
  /*return EXIT_FAILURE;*/
}
