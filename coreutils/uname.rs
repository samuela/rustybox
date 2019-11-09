use libc;
use libc::open;



extern "C" {
  #[no_mangle]
  static mut applet_name: *const libc::c_char;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn puts(__s: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn fflush_stdout_and_exit(retval: libc::c_int) -> !;
  #[no_mangle]
  fn getopt32long(
    argv: *mut *mut libc::c_char,
    optstring: *const libc::c_char,
    longopts: *const libc::c_char,
    _: ...
  ) -> u32;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn uname(__name: *mut utsname) -> libc::c_int;
}


#[derive(Copy, Clone)]
#[repr(C)]
pub struct utsname {
  pub sysname: [libc::c_char; 65],
  pub nodename: [libc::c_char; 65],
  pub release: [libc::c_char; 65],
  pub version: [libc::c_char; 65],
  pub machine: [libc::c_char; 65],
  pub domainname: [libc::c_char; 65],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uname_info_t {
  pub name: utsname,
  pub processor: [libc::c_char; 65],
  pub platform: [libc::c_char; 65],
  pub os: [libc::c_char; 10],
}
static mut utsname_offset: [libc::c_ushort; 8] = [
  0u64 as libc::c_ushort,
  65u64 as libc::c_ushort,
  130u64 as libc::c_ushort,
  195u64 as libc::c_ushort,
  260u64 as libc::c_ushort,
  390u64 as libc::c_ushort,
  455u64 as libc::c_ushort,
  520u64 as libc::c_ushort,
];
#[no_mangle]
pub unsafe extern "C" fn uname_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut uname_info: uname_info_t = uname_info_t {
    name: utsname {
      sysname: [0; 65],
      nodename: [0; 65],
      release: [0; 65],
      version: [0; 65],
      machine: [0; 65],
      domainname: [0; 65],
    },
    processor: [0; 65],
    platform: [0; 65],
    os: [0; 10],
  }; /* "arch" = "uname -m" */
  let mut unknown_str: *const libc::c_char = b"unknown\x00" as *const u8 as *const libc::c_char; /* if "uname" */
  let mut toprint: libc::c_uint = (1i32 << 4i32) as libc::c_uint;
  if 1i32 == 0 || *applet_name.offset(0) as libc::c_int == 'u' as i32 {
    static mut uname_longopts: [libc::c_char; 137] = [
      97, 108, 108, 0, 0, 97, 107, 101, 114, 110, 101, 108, 45, 110, 97, 109, 101, 0, 0, 115, 110,
      111, 100, 101, 110, 97, 109, 101, 0, 0, 110, 107, 101, 114, 110, 101, 108, 45, 114, 101, 108,
      101, 97, 115, 101, 0, 0, 114, 114, 101, 108, 101, 97, 115, 101, 0, 0, 114, 107, 101, 114,
      110, 101, 108, 45, 118, 101, 114, 115, 105, 111, 110, 0, 0, 118, 109, 97, 99, 104, 105, 110,
      101, 0, 0, 109, 112, 114, 111, 99, 101, 115, 115, 111, 114, 0, 0, 112, 104, 97, 114, 100,
      119, 97, 114, 101, 45, 112, 108, 97, 116, 102, 111, 114, 109, 0, 0, 105, 111, 112, 101, 114,
      97, 116, 105, 110, 103, 45, 115, 121, 115, 116, 101, 109, 0, 0, 111, 0,
    ];
    toprint = getopt32long(
      argv,
      b"snrvmpioa\x00" as *const u8 as *const libc::c_char,
      uname_longopts.as_ptr(),
    );
    if !(*argv.offset(optind as isize)).is_null() {
      /* coreutils-6.9 compat */
      bb_show_usage();
    }
    if toprint & (1i32 << 8i32) as libc::c_uint != 0 {
      /* -a => all opts on */
      toprint = ((1i32 << 8i32) - 1i32) as libc::c_uint;
      unknown_str = b"\x00" as *const u8 as *const libc::c_char
      /* -a does not print unknown fields */
    }
    if toprint == 0i32 as libc::c_uint {
      /* no opts => -s (sysname) */
      toprint = 1i32 as libc::c_uint
    }
  } /* never fails */
  uname(&mut uname_info.name);
  if 1i32 != 0 && (1i32 == 0 || *applet_name.offset(0) as libc::c_int == 'a' as i32) {
    puts(uname_info.name.machine.as_mut_ptr());
  } else {
    /* "uname" */
    let mut fmt: *const libc::c_char = 0 as *const libc::c_char;
    let mut delta: *const libc::c_ushort = 0 as *const libc::c_ushort;
    strcpy(uname_info.processor.as_mut_ptr(), unknown_str);
    strcpy(uname_info.platform.as_mut_ptr(), unknown_str);
    strcpy(
      uname_info.os.as_mut_ptr(),
      b"GNU/Linux\x00" as *const u8 as *const libc::c_char,
    );
    delta = utsname_offset.as_ptr();
    fmt = (b" %s\x00" as *const u8 as *const libc::c_char).offset(1);
    loop {
      if toprint & 1i32 as libc::c_uint != 0 {
        let mut p: *const libc::c_char = (&mut uname_info as *mut uname_info_t
          as *mut libc::c_char)
          .offset(*delta as libc::c_int as isize);
        if *p.offset(0) != 0 {
          printf(fmt, p);
          fmt = b" %s\x00" as *const u8 as *const libc::c_char
        }
      }
      delta = delta.offset(1);
      toprint >>= 1i32;
      if !(toprint != 0) {
        break;
      }
    }
    bb_putchar('\n' as i32);
  }
  fflush_stdout_and_exit(0i32);
  /* coreutils-6.9 compat */
}
