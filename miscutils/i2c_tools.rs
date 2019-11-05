use libc;
extern "C" {
  pub type __dirstream;
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn opendir(__name: *const libc::c_char) -> *mut DIR;
  #[no_mangle]
  fn readdir(__dirp: *mut DIR) -> *mut dirent;
  #[no_mangle]
  fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
  #[no_mangle]
  static mut stderr: *mut FILE;
  #[no_mangle]
  fn fclose(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn fopen(__filename: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn snprintf(
    _: *mut libc::c_char,
    _: libc::c_ulong,
    _: *const libc::c_char,
    _: ...
  ) -> libc::c_int;
  #[no_mangle]
  fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn fgets_unlocked(
    __s: *mut libc::c_char,
    __n: libc::c_int,
    __stream: *mut FILE,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn puts(__s: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn strtol(
    __nptr: *const libc::c_char,
    __endptr: *mut *mut libc::c_char,
    __base: libc::c_int,
  ) -> libc::c_long;
  #[no_mangle]
  fn strtoul(
    __nptr: *const libc::c_char,
    __endptr: *mut *mut libc::c_char,
    __base: libc::c_int,
  ) -> libc::c_ulong;
  #[no_mangle]
  fn exit(_: libc::c_int) -> !;
  #[no_mangle]
  fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
  /* Some useful definitions */
  /* Macros for min/max.  */
  /* buffer allocation schemes */
  /* glibc uses __errno_location() to get a ptr to errno */
  /* We can just memorize it once - no multithreading in busybox :) */
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn is_prefixed_with(string: *const libc::c_char, key: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xopendir(path: *const libc::c_char) -> *mut DIR;
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  /* Guaranteed to NOT be a macro (smallest code). Saves nearly 2k on uclibc.
   * But potentially slow, don't use in one-billion-times loops */
  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn fflush_all() -> libc::c_int;
  #[no_mangle]
  fn xstrtou_range(
    str: *const libc::c_char,
    b: libc::c_int,
    l: libc::c_uint,
    u: libc::c_uint,
  ) -> libc::c_uint;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_simple_error_msg(s: *const libc::c_char);
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_perror_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_ask_y_confirmation() -> libc::c_int;
  #[no_mangle]
  fn ioctl_or_perror_and_die(
    fd: libc::c_int,
    request: libc::c_uint,
    argp: *mut libc::c_void,
    fmt: *const libc::c_char,
    _: ...
  ) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __ino64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type int32_t = __int32_t;
use crate::librb::uint8_t;
pub type uint16_t = __uint16_t;
use crate::librb::uint32_t;
pub type intptr_t = libc::c_long;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
  pub d_ino: __ino64_t,
  pub d_off: __off64_t,
  pub d_reclen: libc::c_ushort,
  pub d_type: libc::c_uchar,
  pub d_name: [libc::c_char; 256],
}
pub type DIR = __dirstream;



use crate::librb::FILE;
pub type __u8 = libc::c_uchar;
pub type __u16 = libc::c_ushort;
pub type __u32 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct i2c_msg {
  pub addr: __u16,
  pub flags: __u16,
  pub len: __u16,
  pub buf: *mut __u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union i2c_smbus_data {
  pub byte: __u8,
  pub word: __u16,
  pub block: [__u8; 34],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct i2c_smbus_ioctl_data {
  pub read_write: __u8,
  pub command: __u8,
  pub size: __u32,
  pub data: *mut i2c_smbus_data,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct i2c_rdwr_ioctl_data {
  pub msgs: *mut i2c_msg,
  pub nmsgs: __u32,
  /* number of i2c_msgs */
}
/* ENABLE_I2CDUMP */
pub type adapter_type = libc::c_uint;
pub const ADT_SMBUS: adapter_type = 3;
pub const ADT_I2C: adapter_type = 2;
pub const ADT_ISA: adapter_type = 1;
pub const ADT_DUMMY: adapter_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct adap_desc {
  pub funcs: *const libc::c_char,
  pub algo: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct i2c_func {
  pub value: libc::c_long,
  pub name: *const libc::c_char,
}
pub const opt_y: C2RustUnnamed = 2;
pub const opt_f: C2RustUnnamed = 1;
pub const opt_a: C2RustUnnamed = 4;
pub type C2RustUnnamed = libc::c_uint;
/* end linux/i2c-dev.h */
/*
 * This is needed for ioctl_or_perror_and_die() since it only accepts pointers.
 */
#[inline(always)]
unsafe extern "C" fn itoptr(mut i: libc::c_int) -> *mut libc::c_void {
  return i as intptr_t as *mut libc::c_void;
}
unsafe extern "C" fn i2c_smbus_access(
  mut fd: libc::c_int,
  mut read_write: libc::c_char,
  mut cmd: uint8_t,
  mut size: libc::c_int,
  mut data: *mut i2c_smbus_data,
) -> int32_t {
  let mut args: i2c_smbus_ioctl_data = i2c_smbus_ioctl_data {
    read_write: 0,
    command: 0,
    size: 0,
    data: 0 as *mut i2c_smbus_data,
  };
  args.read_write = read_write as __u8;
  args.command = cmd;
  args.size = size as __u32;
  args.data = data;
  return ioctl(
    fd,
    0x720i32 as libc::c_ulong,
    &mut args as *mut i2c_smbus_ioctl_data,
  );
}
unsafe extern "C" fn i2c_smbus_read_byte(mut fd: libc::c_int) -> int32_t {
  let mut data: i2c_smbus_data = i2c_smbus_data { byte: 0 };
  let mut err: libc::c_int = 0;
  err = i2c_smbus_access(fd, 1i32 as libc::c_char, 0i32 as uint8_t, 1i32, &mut data);
  if err < 0i32 {
    return err;
  }
  return data.byte as int32_t;
}
unsafe extern "C" fn i2c_smbus_write_byte(mut fd: libc::c_int, mut val: uint8_t) -> int32_t {
  return i2c_smbus_access(
    fd,
    0i32 as libc::c_char,
    val,
    1i32,
    0 as *mut i2c_smbus_data,
  );
}
unsafe extern "C" fn i2c_smbus_read_byte_data(mut fd: libc::c_int, mut cmd: uint8_t) -> int32_t {
  let mut data: i2c_smbus_data = i2c_smbus_data { byte: 0 };
  let mut err: libc::c_int = 0;
  err = i2c_smbus_access(fd, 1i32 as libc::c_char, cmd, 2i32, &mut data);
  if err < 0i32 {
    return err;
  }
  return data.byte as int32_t;
}
unsafe extern "C" fn i2c_smbus_read_word_data(mut fd: libc::c_int, mut cmd: uint8_t) -> int32_t {
  let mut data: i2c_smbus_data = i2c_smbus_data { byte: 0 };
  let mut err: libc::c_int = 0;
  err = i2c_smbus_access(fd, 1i32 as libc::c_char, cmd, 3i32, &mut data);
  if err < 0i32 {
    return err;
  }
  return data.word as int32_t;
}
/* ENABLE_I2CGET || ENABLE_I2CSET || ENABLE_I2CDUMP */
unsafe extern "C" fn i2c_smbus_write_byte_data(
  mut file: libc::c_int,
  mut cmd: uint8_t,
  mut value: uint8_t,
) -> int32_t {
  let mut data: i2c_smbus_data = i2c_smbus_data { byte: 0 };
  data.byte = value;
  return i2c_smbus_access(file, 0i32 as libc::c_char, cmd, 2i32, &mut data);
}
unsafe extern "C" fn i2c_smbus_write_word_data(
  mut file: libc::c_int,
  mut cmd: uint8_t,
  mut value: uint16_t,
) -> int32_t {
  let mut data: i2c_smbus_data = i2c_smbus_data { byte: 0 };
  data.word = value;
  return i2c_smbus_access(file, 0i32 as libc::c_char, cmd, 3i32, &mut data);
}
unsafe extern "C" fn i2c_smbus_write_block_data(
  mut file: libc::c_int,
  mut cmd: uint8_t,
  mut length: uint8_t,
  mut values: *const uint8_t,
) -> int32_t {
  let mut data: i2c_smbus_data = i2c_smbus_data { byte: 0 };
  if length as libc::c_int > 32i32 {
    length = 32i32 as uint8_t
  }
  memcpy(
    data.block.as_mut_ptr().offset(1) as *mut libc::c_void,
    values as *const libc::c_void,
    length as libc::c_ulong,
  );
  data.block[0] = length;
  return i2c_smbus_access(file, 0i32 as libc::c_char, cmd, 5i32, &mut data);
}
unsafe extern "C" fn i2c_smbus_write_i2c_block_data(
  mut file: libc::c_int,
  mut cmd: uint8_t,
  mut length: uint8_t,
  mut values: *const uint8_t,
) -> int32_t {
  let mut data: i2c_smbus_data = i2c_smbus_data { byte: 0 };
  if length as libc::c_int > 32i32 {
    length = 32i32 as uint8_t
  }
  memcpy(
    data.block.as_mut_ptr().offset(1) as *mut libc::c_void,
    values as *const libc::c_void,
    length as libc::c_ulong,
  );
  data.block[0] = length;
  return i2c_smbus_access(file, 0i32 as libc::c_char, cmd, 6i32, &mut data);
}
/* ENABLE_I2CSET */
/*
 * Returns the number of bytes read, vals must hold at
 * least I2C_SMBUS_BLOCK_MAX bytes.
 */
unsafe extern "C" fn i2c_smbus_read_block_data(
  mut fd: libc::c_int,
  mut cmd: uint8_t,
  mut vals: *mut uint8_t,
) -> int32_t {
  let mut data: i2c_smbus_data = i2c_smbus_data { byte: 0 };
  let mut i: libc::c_int = 0;
  let mut err: libc::c_int = 0;
  err = i2c_smbus_access(fd, 1i32 as libc::c_char, cmd, 5i32, &mut data);
  if err < 0i32 {
    return err;
  }
  i = 1i32;
  while i <= data.block[0] as libc::c_int {
    let fresh0 = vals;
    vals = vals.offset(1);
    *fresh0 = data.block[i as usize];
    i += 1
  }
  return data.block[0] as int32_t;
}
unsafe extern "C" fn i2c_smbus_read_i2c_block_data(
  mut fd: libc::c_int,
  mut cmd: uint8_t,
  mut len: uint8_t,
  mut vals: *mut uint8_t,
) -> int32_t {
  let mut data: i2c_smbus_data = i2c_smbus_data { byte: 0 };
  let mut i: libc::c_int = 0;
  let mut err: libc::c_int = 0;
  if len as libc::c_int > 32i32 {
    len = 32i32 as uint8_t
  }
  data.block[0] = len;
  err = i2c_smbus_access(
    fd,
    1i32 as libc::c_char,
    cmd,
    if len as libc::c_int == 32i32 {
      6i32
    } else {
      8i32
    },
    &mut data,
  );
  if err < 0i32 {
    return err;
  }
  i = 1i32;
  while i <= data.block[0] as libc::c_int {
    let fresh1 = vals;
    vals = vals.offset(1);
    *fresh1 = data.block[i as usize];
    i += 1
  }
  return data.block[0] as int32_t;
}
/* ENABLE_I2CDUMP */
unsafe extern "C" fn i2c_smbus_write_quick(mut fd: libc::c_int, mut val: uint8_t) -> int32_t {
  return i2c_smbus_access(
    fd,
    val as libc::c_char,
    0i32 as uint8_t,
    0i32,
    0 as *mut i2c_smbus_data,
  );
}
/* ENABLE_I2CDETECT */
unsafe extern "C" fn i2c_bus_lookup(mut bus_str: *const libc::c_char) -> libc::c_int {
  return xstrtou_range(
    bus_str,
    10i32,
    0i32 as libc::c_uint,
    0xfffffi32 as libc::c_uint,
  ) as libc::c_int;
}
unsafe extern "C" fn i2c_parse_bus_addr(mut addr_str: *const libc::c_char) -> libc::c_int {
  /* Slave address must be in range 0x03 - 0x77. */
  return xstrtou_range(
    addr_str,
    16i32,
    0x3i32 as libc::c_uint,
    0x77i32 as libc::c_uint,
  ) as libc::c_int;
}
unsafe extern "C" fn i2c_set_pec(mut fd: libc::c_int, mut pec: libc::c_int) {
  ioctl_or_perror_and_die(
    fd,
    0x708i32 as libc::c_uint,
    itoptr(if pec != 0 { 1i32 } else { 0i32 }),
    b"can\'t set PEC\x00" as *const u8 as *const libc::c_char,
  );
}
unsafe extern "C" fn i2c_set_slave_addr(
  mut fd: libc::c_int,
  mut addr: libc::c_int,
  mut force: libc::c_int,
) {
  ioctl_or_perror_and_die(
    fd,
    if force != 0 { 0x706i32 } else { 0x703i32 } as libc::c_uint,
    itoptr(addr),
    b"can\'t set address to 0x%02x\x00" as *const u8 as *const libc::c_char,
    addr,
  );
}
/* ENABLE_I2CGET || ENABLE_I2CSET || ENABLE_I2CDUMP */
unsafe extern "C" fn i2c_parse_data_addr(mut data_addr: *const libc::c_char) -> libc::c_int {
  /* Data address must be an 8 bit integer. */
  return xstrtou_range(
    data_addr,
    16i32,
    0i32 as libc::c_uint,
    0xffi32 as libc::c_uint,
  ) as libc::c_int;
}
/* ENABLE_I2CGET || ENABLE_I2CSET */
/*
 * Opens the device file associated with given i2c bus.
 *
 * Upstream i2c-tools also support opening devices by i2c bus name
 * but we drop it here for size reduction.
 */
unsafe extern "C" fn i2c_dev_open(mut i2cbus: libc::c_int) -> libc::c_int {
  let mut filename: [libc::c_char; 24] = [0; 24]; /* change to "/dev/i2c/%d" */
  let mut fd: libc::c_int = 0;
  sprintf(
    filename.as_mut_ptr(),
    b"/dev/i2c-%d\x00" as *const u8 as *const libc::c_char,
    i2cbus,
  );
  fd = open(filename.as_mut_ptr(), 0o2i32);
  if fd < 0i32 {
    if *bb_errno == 2i32 {
      filename[8] = '/' as i32 as libc::c_char;
      fd = xopen(filename.as_mut_ptr(), 0o2i32)
    } else {
      bb_perror_msg_and_die(
        b"can\'t open \'%s\'\x00" as *const u8 as *const libc::c_char,
        filename.as_mut_ptr(),
      );
    }
  }
  return fd;
}
/* Size reducing helpers for xxx_check_funcs(). */
unsafe extern "C" fn get_funcs_matrix(mut fd: libc::c_int, mut funcs: *mut libc::c_ulong) {
  ioctl_or_perror_and_die(
    fd,
    0x705i32 as libc::c_uint,
    funcs as *mut libc::c_void,
    b"can\'t get adapter functionality matrix\x00" as *const u8 as *const libc::c_char,
  );
}
unsafe extern "C" fn check_funcs_test_end(
  mut funcs: libc::c_int,
  mut pec: libc::c_int,
  mut err: *const libc::c_char,
) {
  if pec != 0 && funcs & (0x8i32 | 0x1i32) == 0 {
    bb_simple_error_msg(
      b"warning: adapter does not support PEC\x00" as *const u8 as *const libc::c_char,
    );
  }
  if !err.is_null() {
    bb_error_msg_and_die(
      b"adapter has no %s capability\x00" as *const u8 as *const libc::c_char,
      err,
    );
  };
}
/* ENABLE_I2CGET || ENABLE_I2CSET || ENABLE_I2CDUMP */
/*
 * The below functions emit an error message and exit if the adapter doesn't
 * support desired functionalities.
 */
unsafe extern "C" fn check_read_funcs(
  mut fd: libc::c_int,
  mut mode: libc::c_int,
  mut data_addr: libc::c_int,
  mut pec: libc::c_int,
) {
  let mut funcs: libc::c_ulong = 0;
  let mut err: *const libc::c_char = 0 as *const libc::c_char;
  get_funcs_matrix(fd, &mut funcs);
  match mode {
    1 => {
      if funcs & 0x20000i32 as libc::c_ulong == 0 {
        err = b"SMBus receive byte\x00" as *const u8 as *const libc::c_char
      } else if data_addr >= 0i32 && funcs & 0x40000i32 as libc::c_ulong == 0 {
        err = b"SMBus send byte\x00" as *const u8 as *const libc::c_char
      }
    }
    2 => {
      if funcs & 0x80000i32 as libc::c_ulong == 0 {
        err = b"SMBus read byte\x00" as *const u8 as *const libc::c_char
      }
    }
    3 => {
      if funcs & 0x200000i32 as libc::c_ulong == 0 {
        err = b"SMBus read word\x00" as *const u8 as *const libc::c_char
      }
    }
    5 => {
      if funcs & 0x1000000i32 as libc::c_ulong == 0 {
        err = b"SMBus block read\x00" as *const u8 as *const libc::c_char
      }
    }
    8 => {
      if funcs & 0x4000000i32 as libc::c_ulong == 0 {
        err = b"I2C block read\x00" as *const u8 as *const libc::c_char
      }
    }
    _ => {
      /* ENABLE_I2CDUMP */
      bb_simple_error_msg_and_die(b"internal error\x00" as *const u8 as *const libc::c_char);
    }
  }
  check_funcs_test_end(funcs as libc::c_int, pec, err);
}
/* ENABLE_I2CGET || ENABLE_I2CDUMP */
unsafe extern "C" fn check_write_funcs(
  mut fd: libc::c_int,
  mut mode: libc::c_int,
  mut pec: libc::c_int,
) {
  let mut funcs: libc::c_ulong = 0;
  let mut err: *const libc::c_char = 0 as *const libc::c_char;
  get_funcs_matrix(fd, &mut funcs);
  match mode {
    1 => {
      if funcs & 0x40000i32 as libc::c_ulong == 0 {
        err = b"SMBus send byte\x00" as *const u8 as *const libc::c_char
      }
    }
    2 => {
      if funcs & 0x100000i32 as libc::c_ulong == 0 {
        err = b"SMBus write byte\x00" as *const u8 as *const libc::c_char
      }
    }
    3 => {
      if funcs & 0x400000i32 as libc::c_ulong == 0 {
        err = b"SMBus write word\x00" as *const u8 as *const libc::c_char
      }
    }
    5 => {
      if funcs & 0x2000000i32 as libc::c_ulong == 0 {
        err = b"SMBus block write\x00" as *const u8 as *const libc::c_char
      }
    }
    8 => {
      if funcs & 0x8000000i32 as libc::c_ulong == 0 {
        err = b"I2C block write\x00" as *const u8 as *const libc::c_char
      }
    }
    _ => {}
  }
  check_funcs_test_end(funcs as libc::c_int, pec, err);
}
/* ENABLE_I2CSET */
unsafe extern "C" fn confirm_or_abort() {
  fprintf(
    stderr,
    b"Continue? [y/N] \x00" as *const u8 as *const libc::c_char,
  );
  if bb_ask_y_confirmation() == 0 {
    bb_simple_error_msg_and_die(b"aborting\x00" as *const u8 as *const libc::c_char);
  };
}
/*
 * Return only if user confirms the action, abort otherwise.
 *
 * The messages displayed here are much less elaborate than their i2c-tools
 * counterparts - this is done for size reduction.
 */
unsafe extern "C" fn confirm_action(
  mut bus_addr: libc::c_int,
  mut mode: libc::c_int,
  mut data_addr: libc::c_int,
  mut pec: libc::c_int,
) {
  bb_simple_error_msg(
    b"WARNING! This program can confuse your I2C bus\x00" as *const u8 as *const libc::c_char,
  );
  /* Don't let the user break his/her EEPROMs */
  if bus_addr >= 0x50i32 && bus_addr <= 0x57i32 && pec != 0 {
    bb_simple_error_msg_and_die(
      b"this is I2C not smbus - using PEC on I2C devices may result in data loss, aborting\x00"
        as *const u8 as *const libc::c_char,
    );
  }
  if mode == 1i32 && data_addr >= 0i32 && pec != 0 {
    bb_simple_error_msg(
      b"WARNING! May interpret a write byte command with PEC as a write byte data command\x00"
        as *const u8 as *const libc::c_char,
    );
  }
  if pec != 0 {
    bb_simple_error_msg(b"PEC checking enabled\x00" as *const u8 as *const libc::c_char);
  }
  confirm_or_abort();
}
//usage:#define i2cget_trivial_usage
//usage:       "[-fy] BUS CHIP-ADDRESS [DATA-ADDRESS [MODE]]"
//usage:#define i2cget_full_usage "\n\n"
//usage:       "Read from I2C/SMBus chip registers"
//usage:     "\n"
//usage:     "\n	I2CBUS	I2C bus number"
//usage:     "\n	ADDRESS	0x03-0x77"
//usage:     "\nMODE is:"
//usage:     "\n	b	Read byte data (default)"
//usage:     "\n	w	Read word data"
//usage:     "\n	c	Write byte/read byte"
//usage:     "\n	Append p for SMBus PEC"
//usage:     "\n"
//usage:     "\n	-f	Force access"
//usage:     "\n	-y	Disable interactive mode"
#[no_mangle]
pub unsafe extern "C" fn i2cget_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let opt_f_0: libc::c_uint = (1i32 << 0i32) as libc::c_uint;
  let opt_y_0: libc::c_uint = (1i32 << 1i32) as libc::c_uint;
  let mut bus_num: libc::c_int = 0;
  let mut bus_addr: libc::c_int = 0;
  let mut data_addr: libc::c_int = -1i32;
  let mut status: libc::c_int = 0;
  let mut mode: libc::c_int = 1i32;
  let mut pec: libc::c_int = 0i32;
  let mut fd: libc::c_int = 0;
  let mut opts: libc::c_uint = 0;
  opts = getopt32(
    argv,
    b"^fy\x00-2:?4\x00" as *const u8 as *const libc::c_char,
  );
  argv = argv.offset(optind as isize);
  bus_num = i2c_bus_lookup(*argv.offset(0));
  bus_addr = i2c_parse_bus_addr(*argv.offset(1));
  if !(*argv.offset(2)).is_null() {
    data_addr = i2c_parse_data_addr(*argv.offset(2));
    mode = 2i32;
    if !(*argv.offset(3)).is_null() {
      match *(*argv.offset(3)).offset(0) as libc::c_int {
        98 => {}
        119 => mode = 3i32,
        99 => mode = 1i32,
        _ => {
          bb_simple_error_msg(b"invalid mode\x00" as *const u8 as *const libc::c_char);
          bb_show_usage();
        }
      }
      pec = (*(*argv.offset(3)).offset(1) as libc::c_int == 'p' as i32) as libc::c_int
    }
  }
  fd = i2c_dev_open(bus_num);
  check_read_funcs(fd, mode, data_addr, pec);
  i2c_set_slave_addr(fd, bus_addr, (opts & opt_f_0) as libc::c_int);
  if opts & opt_y_0 == 0 {
    confirm_action(bus_addr, mode, data_addr, pec);
  }
  if pec != 0 {
    i2c_set_pec(fd, 1i32);
  }
  match mode {
    1 => {
      if data_addr >= 0i32 {
        status = i2c_smbus_write_byte(fd, data_addr as uint8_t);
        if status < 0i32 {
          bb_simple_error_msg(b"warning - write failed\x00" as *const u8 as *const libc::c_char);
        }
      }
      status = i2c_smbus_read_byte(fd)
    }
    3 => status = i2c_smbus_read_word_data(fd, data_addr as uint8_t),
    _ => {
      /* I2C_SMBUS_BYTE_DATA */
      status = i2c_smbus_read_byte_data(fd, data_addr as uint8_t)
    }
  }
  close(fd);
  if status < 0i32 {
    bb_simple_perror_msg_and_die(b"read failed\x00" as *const u8 as *const libc::c_char);
  }
  printf(
    b"0x%0*x\n\x00" as *const u8 as *const libc::c_char,
    if mode == 3i32 { 4i32 } else { 2i32 },
    status,
  );
  return 0i32;
}
/* ENABLE_I2CGET */
//usage:#define i2cset_trivial_usage
//usage:       "[-fy] [-m MASK] BUS CHIP-ADDRESS DATA-ADDRESS [VALUE] ... [MODE]"
//usage:#define i2cset_full_usage "\n\n"
//usage:       "Set I2C registers"
//usage:     "\n"
//usage:     "\n	I2CBUS	I2C bus number"
//usage:     "\n	ADDRESS	0x03-0x77"
//usage:     "\nMODE is:"
//usage:     "\n	c	Byte, no value"
//usage:     "\n	b	Byte data (default)"
//usage:     "\n	w	Word data"
//usage:     "\n	i	I2C block data"
//usage:     "\n	s	SMBus block data"
//usage:     "\n	Append p for SMBus PEC"
//usage:     "\n"
//usage:     "\n	-f	Force access"
//usage:     "\n	-y	Disable interactive mode"
//usage:     "\n	-r	Read back and compare the result"
//usage:     "\n	-m MASK	Mask specifying which bits to write"
#[no_mangle]
pub unsafe extern "C" fn i2cset_main(
  mut argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let opt_f_0: libc::c_uint = (1i32 << 0i32) as libc::c_uint; /* now argv[argc] is last arg */
  let opt_y_0: libc::c_uint = (1i32 << 1i32) as libc::c_uint;
  let opt_m: libc::c_uint = (1i32 << 2i32) as libc::c_uint;
  let opt_r: libc::c_uint = (1i32 << 3i32) as libc::c_uint;
  let mut bus_num: libc::c_int = 0;
  let mut bus_addr: libc::c_int = 0;
  let mut data_addr: libc::c_int = 0;
  let mut mode: libc::c_int = 1i32;
  let mut pec: libc::c_int = 0i32;
  let mut val: libc::c_int = 0;
  let mut blen: libc::c_int = 0;
  let mut mask: libc::c_int = 0;
  let mut fd: libc::c_int = 0;
  let mut status: libc::c_int = 0;
  let mut block: [libc::c_uchar; 32] = [0; 32];
  let mut opt_m_arg: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut opts: libc::c_uint = 0;
  opts = getopt32(
    argv,
    b"^fym:r\x00-3\x00" as *const u8 as *const libc::c_char,
    &mut opt_m_arg as *mut *mut libc::c_char,
  );
  argv = argv.offset(optind as isize);
  argc -= optind;
  argc -= 1;
  bus_num = i2c_bus_lookup(*argv.offset(0));
  bus_addr = i2c_parse_bus_addr(*argv.offset(1));
  data_addr = i2c_parse_data_addr(*argv.offset(2));
  if !(*argv.offset(3)).is_null() {
    if (*argv.offset(4)).is_null() && *(*argv.offset(3)).offset(0) as libc::c_int != 'c' as i32 {
      mode = 2i32
    /* Implicit b */
    } else {
      match *(*argv.offset(argc as isize)).offset(0) as libc::c_int {
        99 => {}
        98 => mode = 2i32,
        119 => mode = 3i32,
        115 => mode = 5i32,
        105 => mode = 8i32,
        _ => {
          bb_simple_error_msg(b"invalid mode\x00" as *const u8 as *const libc::c_char);
          bb_show_usage();
        }
      }
      pec = (*(*argv.offset(argc as isize)).offset(1) as libc::c_int == 'p' as i32) as libc::c_int;
      if mode == 5i32 || mode == 8i32 {
        if pec != 0 && mode == 8i32 {
          bb_simple_error_msg_and_die(
            b"PEC not supported for I2C block writes\x00" as *const u8 as *const libc::c_char,
          );
        }
        if opts & opt_m != 0 {
          bb_simple_error_msg_and_die(
            b"mask not supported for block writes\x00" as *const u8 as *const libc::c_char,
          );
        }
      }
    }
  }
  /* Prepare the value(s) to be written according to current mode. */
  mask = 0i32;
  blen = 0i32;
  match mode {
    2 => {
      val = xstrtou_range(
        *argv.offset(3),
        0i32,
        0i32 as libc::c_uint,
        0xffi32 as libc::c_uint,
      ) as libc::c_int
    }
    3 => {
      val = xstrtou_range(
        *argv.offset(3),
        0i32,
        0i32 as libc::c_uint,
        0xffffi32 as libc::c_uint,
      ) as libc::c_int
    }
    5 | 8 => {
      blen = 3i32;
      while blen < argc {
        block[(blen - 3i32) as usize] = xstrtou_range(
          *argv.offset(blen as isize),
          0i32,
          0i32 as libc::c_uint,
          0xffi32 as libc::c_uint,
        ) as libc::c_uchar;
        blen += 1
      }
      blen -= 3i32;
      val = -1i32
    }
    _ => val = -1i32,
  }
  if opts & opt_m != 0 {
    mask = xstrtou_range(
      opt_m_arg,
      0i32,
      0i32 as libc::c_uint,
      if mode == 1i32 || mode == 2i32 {
        0xffi32
      } else {
        0xffffi32
      } as libc::c_uint,
    ) as libc::c_int
  }
  fd = i2c_dev_open(bus_num);
  check_write_funcs(fd, mode, pec);
  i2c_set_slave_addr(fd, bus_addr, (opts & opt_f_0) as libc::c_int);
  if opts & opt_y_0 == 0 {
    confirm_action(bus_addr, mode, data_addr, pec);
  }
  /*
   * If we're using mask - read the current value here and adjust the
   * value to be written.
   */
  if opts & opt_m != 0 {
    let mut tmpval: libc::c_int = 0;
    match mode {
      1 => tmpval = i2c_smbus_read_byte(fd),
      3 => tmpval = i2c_smbus_read_word_data(fd, data_addr as uint8_t),
      _ => tmpval = i2c_smbus_read_byte_data(fd, data_addr as uint8_t),
    }
    if tmpval < 0i32 {
      bb_simple_perror_msg_and_die(
        b"can\'t read old value\x00" as *const u8 as *const libc::c_char,
      );
    }
    val = val & mask | tmpval & !mask;
    if opts & opt_y_0 == 0 {
      bb_error_msg(
        b"old value 0x%0*x, write mask 0x%0*x, will write 0x%0*x to register 0x%02x\x00"
          as *const u8 as *const libc::c_char,
        if mode == 3i32 { 4i32 } else { 2i32 },
        tmpval,
        if mode == 3i32 { 4i32 } else { 2i32 },
        mask,
        if mode == 3i32 { 4i32 } else { 2i32 },
        val,
        data_addr,
      );
      confirm_or_abort();
    }
  }
  if pec != 0 {
    i2c_set_pec(fd, 1i32);
  }
  match mode {
    1 => status = i2c_smbus_write_byte(fd, data_addr as uint8_t),
    3 => status = i2c_smbus_write_word_data(fd, data_addr as uint8_t, val as uint16_t),
    5 => {
      status = i2c_smbus_write_block_data(
        fd,
        data_addr as uint8_t,
        blen as uint8_t,
        block.as_mut_ptr(),
      )
    }
    8 => {
      status = i2c_smbus_write_i2c_block_data(
        fd,
        data_addr as uint8_t,
        blen as uint8_t,
        block.as_mut_ptr(),
      )
    }
    _ => {
      /* I2C_SMBUS_BYTE_DATA */
      status = i2c_smbus_write_byte_data(fd, data_addr as uint8_t, val as uint8_t)
    }
  } /* Clear PEC. */
  if status < 0i32 {
    bb_simple_perror_msg_and_die(b"write failed\x00" as *const u8 as *const libc::c_char);
  }
  if pec != 0 {
    i2c_set_pec(fd, 0i32);
  }
  /* No readback required - we're done. */
  if opts & opt_r == 0 {
    return 0i32;
  }
  match mode {
    1 => {
      status = i2c_smbus_read_byte(fd);
      val = data_addr
    }
    3 => status = i2c_smbus_read_word_data(fd, data_addr as uint8_t),
    _ => {
      /* I2C_SMBUS_BYTE_DATA */
      status = i2c_smbus_read_byte_data(fd, data_addr as uint8_t)
    }
  }
  if status < 0i32 {
    puts(b"Warning - readback failed\x00" as *const u8 as *const libc::c_char);
  } else if status != val {
    printf(
      b"Warning - data mismatch - wrote 0x%0*x, read back 0x%0*x\n\x00" as *const u8
        as *const libc::c_char,
      if mode == 3i32 { 4i32 } else { 2i32 },
      val,
      if mode == 3i32 { 4i32 } else { 2i32 },
      status,
    );
  } else {
    printf(
      b"Value 0x%0*x written, readback matched\n\x00" as *const u8 as *const libc::c_char,
      if mode == 3i32 { 4i32 } else { 2i32 },
      val,
    );
  }
  return 0i32;
}
/* ENABLE_I2CSET */
unsafe extern "C" fn read_block_data(
  mut buf_fd: libc::c_int,
  mut mode: libc::c_int,
  mut block: *mut libc::c_int,
) -> libc::c_int {
  let mut current_block: u64;
  let mut cblock: [uint8_t; 288] = [0; 288];
  let mut res: libc::c_int = 0;
  let mut blen: libc::c_int = 0i32;
  let mut tmp: libc::c_int = 0;
  let mut i: libc::c_int = 0;
  if mode == 5i32 {
    blen = i2c_smbus_read_block_data(buf_fd, 0i32 as uint8_t, cblock.as_mut_ptr());
    if blen <= 0i32 {
      current_block = 11502256174207680615;
    } else {
      current_block = 26972500619410423;
    }
  } else {
    res = 0i32;
    loop {
      if !(res < 256i32) {
        current_block = 11650488183268122163;
        break;
      }
      tmp = i2c_smbus_read_i2c_block_data(
        buf_fd,
        res as uint8_t,
        32i32 as uint8_t,
        cblock.as_mut_ptr().offset(res as isize),
      );
      if tmp <= 0i32 {
        blen = tmp;
        current_block = 11502256174207680615;
        break;
      } else {
        res += tmp
      }
    }
    match current_block {
      11502256174207680615 => {}
      _ => {
        if res >= 256i32 {
          res = 256i32
        }
        i = 0i32;
        while i < res {
          *block.offset(i as isize) = cblock[i as usize] as libc::c_int;
          i += 1
        }
        if mode != 5i32 {
          i = res;
          while i < 256i32 {
            *block.offset(i as isize) = -1i32;
            i += 1
          }
        }
        current_block = 26972500619410423;
      }
    }
  }
  match current_block {
    11502256174207680615 => {
      bb_error_msg_and_die(
        b"block read failed: %d\x00" as *const u8 as *const libc::c_char,
        blen,
      );
    }
    _ => return blen,
  };
}
/* Dump all but word data. */
unsafe extern "C" fn dump_data(
  mut bus_fd: libc::c_int,
  mut mode: libc::c_int,
  mut first: libc::c_uint,
  mut last: libc::c_uint,
  mut block: *mut libc::c_int,
  mut blen: libc::c_int,
) {
  let mut i: libc::c_int = 0;
  let mut j: libc::c_int = 0;
  let mut res: libc::c_int = 0;
  puts(
    b"     0  1  2  3  4  5  6  7  8  9  a  b  c  d  e  f    0123456789abcdef\x00" as *const u8
      as *const libc::c_char,
  );
  i = 0i32;
  while i < 256i32 {
    if mode == 5i32 && i >= blen {
      break;
    }
    if !(((i / 16i32) as libc::c_uint) < first.wrapping_div(16i32 as libc::c_uint)) {
      if (i / 16i32) as libc::c_uint > last.wrapping_div(16i32 as libc::c_uint) {
        break;
      }
      printf(b"%02x: \x00" as *const u8 as *const libc::c_char, i);
      j = 0i32;
      while j < 16i32 {
        fflush_all();
        /* Skip unwanted registers */
        if ((i + j) as libc::c_uint) < first || (i + j) as libc::c_uint > last {
          printf(b"   \x00" as *const u8 as *const libc::c_char);
          if mode == 3i32 {
            printf(b"   \x00" as *const u8 as *const libc::c_char);
            j += 1
          }
        } else {
          match mode {
            2 => {
              res = i2c_smbus_read_byte_data(bus_fd, (i + j) as uint8_t);
              *block.offset((i + j) as isize) = res
            }
            3 => {
              res = i2c_smbus_read_word_data(bus_fd, (i + j) as uint8_t);
              if res < 0i32 {
                *block.offset((i + j) as isize) = res;
                *block.offset((i + j + 1i32) as isize) = res
              } else {
                *block.offset((i + j) as isize) = res & 0xffi32;
                *block.offset((i + j + 1i32) as isize) = res >> 8i32
              }
            }
            1 => {
              res = i2c_smbus_read_byte(bus_fd);
              *block.offset((i + j) as isize) = res
            }
            _ => res = *block.offset((i + j) as isize),
          }
          if mode == 5i32 && i + j >= blen {
            printf(b"   \x00" as *const u8 as *const libc::c_char);
          } else if res < 0i32 {
            printf(b"XX \x00" as *const u8 as *const libc::c_char);
            if mode == 3i32 {
              printf(b"XX \x00" as *const u8 as *const libc::c_char);
            }
          } else {
            printf(
              b"%02x \x00" as *const u8 as *const libc::c_char,
              *block.offset((i + j) as isize),
            );
            if mode == 3i32 {
              printf(
                b"%02x \x00" as *const u8 as *const libc::c_char,
                *block.offset((i + j + 1i32) as isize),
              );
            }
          }
          if mode == 3i32 {
            j += 1
          }
        }
        j += 1
      }
      printf(b"   \x00" as *const u8 as *const libc::c_char);
      j = 0i32;
      while j < 16i32 {
        if mode == 5i32 && i + j >= blen {
          break;
        }
        /* Skip unwanted registers */
        if ((i + j) as libc::c_uint) < first || (i + j) as libc::c_uint > last {
          bb_putchar(' ' as i32);
        } else {
          res = *block.offset((i + j) as isize);
          if res < 0i32 {
            bb_putchar('X' as i32);
          } else if res == 0i32 || res == 0xffi32 {
            bb_putchar('.' as i32);
          } else if res < 32i32 || res >= 127i32 {
            bb_putchar('?' as i32);
          } else {
            bb_putchar(res);
          }
        }
        j += 1
      }
      bb_putchar('\n' as i32);
    }
    i += 0x10i32
  }
}
unsafe extern "C" fn dump_word_data(
  mut bus_fd: libc::c_int,
  mut first: libc::c_uint,
  mut last: libc::c_uint,
) {
  let mut i: libc::c_int = 0;
  let mut j: libc::c_int = 0;
  let mut rv: libc::c_int = 0;
  /* Word data. */
  puts(b"     0,8  1,9  2,a  3,b  4,c  5,d  6,e  7,f\x00" as *const u8 as *const libc::c_char);
  i = 0i32;
  while i < 256i32 {
    if !(((i / 8i32) as libc::c_uint) < first.wrapping_div(8i32 as libc::c_uint)) {
      if (i / 8i32) as libc::c_uint > last.wrapping_div(8i32 as libc::c_uint) {
        break;
      }
      printf(b"%02x: \x00" as *const u8 as *const libc::c_char, i);
      j = 0i32;
      while j < 8i32 {
        /* Skip unwanted registers. */
        if ((i + j) as libc::c_uint) < first || (i + j) as libc::c_uint > last {
          printf(b"     \x00" as *const u8 as *const libc::c_char);
        } else {
          rv = i2c_smbus_read_word_data(bus_fd, (i + j) as uint8_t);
          if rv < 0i32 {
            printf(b"XXXX \x00" as *const u8 as *const libc::c_char);
          } else {
            printf(
              b"%04x \x00" as *const u8 as *const libc::c_char,
              rv & 0xffffi32,
            );
          }
        }
        j += 1
      }
      bb_putchar('\n' as i32);
    }
    i += 8i32
  }
}
//usage:#define i2cdump_trivial_usage
//usage:       "[-fy] [-r FIRST-LAST] BUS ADDR [MODE]"
//usage:#define i2cdump_full_usage "\n\n"
//usage:       "Examine I2C registers"
//usage:     "\n"
//usage:     "\n	I2CBUS	I2C bus number"
//usage:     "\n	ADDRESS	0x03-0x77"
//usage:     "\nMODE is:"
//usage:     "\n	b	Byte (default)"
//usage:     "\n	w	Word"
//usage:     "\n	W	Word on even register addresses"
//usage:     "\n	i	I2C block"
//usage:     "\n	s	SMBus block"
//usage:     "\n	c	Consecutive byte"
//usage:     "\n	Append p for SMBus PEC"
//usage:     "\n"
//usage:     "\n	-f	Force access"
//usage:     "\n	-y	Disable interactive mode"
//usage:     "\n	-r	Limit the number of registers being accessed"
#[no_mangle]
pub unsafe extern "C" fn i2cdump_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let opt_f_0: libc::c_uint = (1i32 << 0i32) as libc::c_uint;
  let opt_y_0: libc::c_uint = (1i32 << 1i32) as libc::c_uint;
  let opt_r: libc::c_uint = (1i32 << 2i32) as libc::c_uint;
  let mut bus_num: libc::c_int = 0;
  let mut bus_addr: libc::c_int = 0;
  let mut mode: libc::c_int = 2i32;
  let mut even: libc::c_int = 0i32;
  let mut pec: libc::c_int = 0i32;
  let mut first: libc::c_uint = 0i32 as libc::c_uint;
  let mut last: libc::c_uint = 0xffi32 as libc::c_uint;
  let mut opts: libc::c_uint = 0;
  let mut block: [libc::c_int; 256] = [0; 256];
  let mut opt_r_str: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut dash: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut fd: libc::c_int = 0;
  let mut res: libc::c_int = 0;
  opts = getopt32(
    argv,
    b"^fyr:\x00-2:?3\x00" as *const u8 as *const libc::c_char,
    &mut opt_r_str as *mut *mut libc::c_char,
  );
  argv = argv.offset(optind as isize);
  bus_num = i2c_bus_lookup(*argv.offset(0));
  bus_addr = i2c_parse_bus_addr(*argv.offset(1));
  if !(*argv.offset(2)).is_null() {
    match *(*argv.offset(2)).offset(0) as libc::c_int {
      98 => {}
      99 => mode = 1i32,
      119 => mode = 3i32,
      87 => {
        mode = 3i32;
        even = 1i32
      }
      115 => mode = 5i32,
      105 => mode = 8i32,
      _ => {
        bb_simple_error_msg_and_die(b"invalid mode\x00" as *const u8 as *const libc::c_char);
      }
    }
    if *(*argv.offset(2)).offset(1) as libc::c_int == 'p' as i32 {
      if *(*argv.offset(2)).offset(0) as libc::c_int == 'W' as i32
        || *(*argv.offset(2)).offset(0) as libc::c_int == 'i' as i32
      {
        bb_simple_error_msg_and_die(
          b"pec not supported for -W and -i\x00" as *const u8 as *const libc::c_char,
        );
      } else {
        pec = 1i32
      }
    }
  }
  if opts & opt_r != 0 {
    first = strtol(opt_r_str, &mut dash, 0i32) as libc::c_uint;
    if dash == opt_r_str || *dash as libc::c_int != '-' as i32 || first > 0xffi32 as libc::c_uint {
      bb_simple_error_msg_and_die(b"invalid range\x00" as *const u8 as *const libc::c_char);
    }
    dash = dash.offset(1);
    last = xstrtou_range(dash, 0i32, first, 0xffi32 as libc::c_uint);
    let mut current_block_25: u64;
    /* Range is not available for every mode. */
    match mode {
      1 | 2 => {
        current_block_25 = 7333393191927787629;
      }
      3 => {
        if even == 0
          || first.wrapping_rem(2i32 as libc::c_uint) == 0
            && last.wrapping_rem(2i32 as libc::c_uint) != 0
        {
          current_block_25 = 7333393191927787629;
        } else {
          current_block_25 = 12719898589429693100;
        }
      }
      _ => {
        current_block_25 = 12719898589429693100;
      }
    }
    match current_block_25 {
      12719898589429693100 =>
      /* Fall through */
      {
        bb_simple_error_msg_and_die(
          b"range not compatible with selected mode\x00" as *const u8 as *const libc::c_char,
        );
      }
      _ => {}
    }
  }
  fd = i2c_dev_open(bus_num);
  check_read_funcs(fd, mode, -1i32, pec);
  i2c_set_slave_addr(fd, bus_addr, (opts & opt_f_0) as libc::c_int);
  if pec != 0 {
    i2c_set_pec(fd, 1i32);
  }
  if opts & opt_y_0 == 0 {
    confirm_action(bus_addr, mode, -1i32, pec);
  }
  /* All but word data. */
  if mode != 3i32 || even != 0 {
    let mut blen: libc::c_int = 0i32;
    if mode == 5i32 || mode == 8i32 {
      blen = read_block_data(fd, mode, block.as_mut_ptr())
    }
    if mode == 1i32 {
      res = i2c_smbus_write_byte(fd, first as uint8_t);
      if res < 0i32 {
        bb_simple_perror_msg_and_die(
          b"write start address\x00" as *const u8 as *const libc::c_char,
        );
      }
    }
    dump_data(fd, mode, first, last, block.as_mut_ptr(), blen);
  } else {
    dump_word_data(fd, first, last);
  }
  return 0i32;
}
static mut adap_descs: [adap_desc; 4] = [
  {
    let mut init = adap_desc {
      funcs: b"dummy\x00" as *const u8 as *const libc::c_char,
      algo: b"Dummy bus\x00" as *const u8 as *const libc::c_char,
    };
    init
  },
  {
    let mut init = adap_desc {
      funcs: b"isa\x00" as *const u8 as *const libc::c_char,
      algo: b"ISA bus\x00" as *const u8 as *const libc::c_char,
    };
    init
  },
  {
    let mut init = adap_desc {
      funcs: b"i2c\x00" as *const u8 as *const libc::c_char,
      algo: b"I2C adapter\x00" as *const u8 as *const libc::c_char,
    };
    init
  },
  {
    let mut init = adap_desc {
      funcs: b"smbus\x00" as *const u8 as *const libc::c_char,
      algo: b"SMBus adapter\x00" as *const u8 as *const libc::c_char,
    };
    init
  },
];
static mut i2c_funcs_tab: [i2c_func; 16] = [
  {
    let mut init = i2c_func {
      value: 0x1i32 as libc::c_long,
      name: b"I2C\x00" as *const u8 as *const libc::c_char,
    };
    init
  },
  {
    let mut init = i2c_func {
      value: 0x10000i32 as libc::c_long,
      name: b"SMBus quick command\x00" as *const u8 as *const libc::c_char,
    };
    init
  },
  {
    let mut init = i2c_func {
      value: 0x40000i32 as libc::c_long,
      name: b"SMBus send byte\x00" as *const u8 as *const libc::c_char,
    };
    init
  },
  {
    let mut init = i2c_func {
      value: 0x20000i32 as libc::c_long,
      name: b"SMBus receive byte\x00" as *const u8 as *const libc::c_char,
    };
    init
  },
  {
    let mut init = i2c_func {
      value: 0x100000i32 as libc::c_long,
      name: b"SMBus write byte\x00" as *const u8 as *const libc::c_char,
    };
    init
  },
  {
    let mut init = i2c_func {
      value: 0x80000i32 as libc::c_long,
      name: b"SMBus read byte\x00" as *const u8 as *const libc::c_char,
    };
    init
  },
  {
    let mut init = i2c_func {
      value: 0x400000i32 as libc::c_long,
      name: b"SMBus write word\x00" as *const u8 as *const libc::c_char,
    };
    init
  },
  {
    let mut init = i2c_func {
      value: 0x200000i32 as libc::c_long,
      name: b"SMBus read word\x00" as *const u8 as *const libc::c_char,
    };
    init
  },
  {
    let mut init = i2c_func {
      value: 0x800000i32 as libc::c_long,
      name: b"SMBus process call\x00" as *const u8 as *const libc::c_char,
    };
    init
  },
  {
    let mut init = i2c_func {
      value: 0x2000000i32 as libc::c_long,
      name: b"SMBus block write\x00" as *const u8 as *const libc::c_char,
    };
    init
  },
  {
    let mut init = i2c_func {
      value: 0x1000000i32 as libc::c_long,
      name: b"SMBus block read\x00" as *const u8 as *const libc::c_char,
    };
    init
  },
  {
    let mut init = i2c_func {
      value: 0x8000i32 as libc::c_long,
      name: b"SMBus block process call\x00" as *const u8 as *const libc::c_char,
    };
    init
  },
  {
    let mut init = i2c_func {
      value: 0x8i32 as libc::c_long,
      name: b"SMBus PEC\x00" as *const u8 as *const libc::c_char,
    };
    init
  },
  {
    let mut init = i2c_func {
      value: 0x8000000i32 as libc::c_long,
      name: b"I2C block write\x00" as *const u8 as *const libc::c_char,
    };
    init
  },
  {
    let mut init = i2c_func {
      value: 0x4000000i32 as libc::c_long,
      name: b"I2C block read\x00" as *const u8 as *const libc::c_char,
    };
    init
  },
  {
    let mut init = i2c_func {
      value: 0i32 as libc::c_long,
      name: 0 as *const libc::c_char,
    };
    init
  },
];
unsafe extern "C" fn i2cdetect_get_funcs(mut bus: libc::c_int) -> adapter_type {
  let mut ret: adapter_type = ADT_DUMMY;
  let mut funcs: libc::c_ulong = 0;
  let mut fd: libc::c_int = 0;
  fd = i2c_dev_open(bus);
  get_funcs_matrix(fd, &mut funcs);
  if funcs & 0x1i32 as libc::c_ulong != 0 {
    ret = ADT_I2C
  } else if funcs
    & (0x20000i32 | 0x40000i32 | (0x80000i32 | 0x100000i32) | (0x200000i32 | 0x400000i32))
      as libc::c_ulong
    != 0
  {
    ret = ADT_SMBUS
  } else {
    ret = ADT_DUMMY
  }
  close(fd);
  return ret;
}
unsafe extern "C" fn list_i2c_busses_and_exit() -> ! {
  let i2cdev_path: *const libc::c_char =
    b"/sys/class/i2c-dev\x00" as *const u8 as *const libc::c_char;
  let mut path: [libc::c_char; 255] = [0; 255];
  let mut name: [libc::c_char; 128] = [0; 128];
  let mut de: *mut dirent = 0 as *mut dirent;
  let mut subde: *mut dirent = 0 as *mut dirent;
  let mut adt: adapter_type = ADT_DUMMY;
  let mut dir: *mut DIR = 0 as *mut DIR;
  let mut subdir: *mut DIR = 0 as *mut DIR;
  let mut rv: libc::c_int = 0;
  let mut bus: libc::c_int = 0;
  let mut pos: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut fp: *mut FILE = 0 as *mut FILE;
  /*
   * XXX Upstream i2cdetect also looks for i2c bus info in /proc/bus/i2c,
   * but we won't bother since it's only useful on older kernels (before
   * 2.6.5). We expect sysfs to be present and mounted at /sys/.
   */
  dir = xopendir(i2cdev_path);
  loop {
    de = readdir(dir);
    if de.is_null() {
      break;
    }
    if (*de).d_name[0] as libc::c_int == '.' as i32 {
      continue;
    }
    /* Simple version for ISA chips. */
    snprintf(
      path.as_mut_ptr(),
      255i32 as libc::c_ulong,
      b"%s/%s/name\x00" as *const u8 as *const libc::c_char,
      i2cdev_path,
      (*de).d_name.as_mut_ptr(),
    );
    fp = fopen(
      path.as_mut_ptr(),
      b"r\x00" as *const u8 as *const libc::c_char,
    );
    if fp.is_null() {
      snprintf(
        path.as_mut_ptr(),
        255i32 as libc::c_ulong,
        b"%s/%s/device/name\x00" as *const u8 as *const libc::c_char,
        i2cdev_path,
        (*de).d_name.as_mut_ptr(),
      );
      fp = fopen(
        path.as_mut_ptr(),
        b"r\x00" as *const u8 as *const libc::c_char,
      )
    }
    /* Non-ISA chips require the hard-way. */
    if fp.is_null() {
      snprintf(
        path.as_mut_ptr(),
        255i32 as libc::c_ulong,
        b"%s/%s/device/name\x00" as *const u8 as *const libc::c_char,
        i2cdev_path,
        (*de).d_name.as_mut_ptr(),
      );
      subdir = opendir(path.as_mut_ptr());
      if subdir.is_null() {
        continue;
      }
      loop {
        subde = readdir(subdir);
        if subde.is_null() {
          break;
        }
        if (*subde).d_name[0] as libc::c_int == '.' as i32 {
          continue;
        }
        if is_prefixed_with(
          (*subde).d_name.as_mut_ptr(),
          b"i2c-\x00" as *const u8 as *const libc::c_char,
        )
        .is_null()
        {
          continue;
        }
        snprintf(
          path.as_mut_ptr(),
          255i32 as libc::c_ulong,
          b"%s/%s/device/%s/name\x00" as *const u8 as *const libc::c_char,
          i2cdev_path,
          (*de).d_name.as_mut_ptr(),
          (*subde).d_name.as_mut_ptr(),
        );
        fp = fopen(
          path.as_mut_ptr(),
          b"r\x00" as *const u8 as *const libc::c_char,
        );
        break;
      }
    }
    if fp.is_null() {
      continue;
    }
    /*
     * Get the rest of the info and display a line
     * for a single bus.
     */
    memset(
      name.as_mut_ptr() as *mut libc::c_void,
      0i32,
      ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
    );
    pos = fgets_unlocked(
      name.as_mut_ptr(),
      ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
      fp,
    );
    fclose(fp);
    if pos.is_null() {
      continue;
    }
    pos = strchr(name.as_mut_ptr(), '\n' as i32);
    if !pos.is_null() {
      *pos = '\u{0}' as i32 as libc::c_char
    }
    rv = sscanf(
      (*de).d_name.as_mut_ptr(),
      b"i2c-%d\x00" as *const u8 as *const libc::c_char,
      &mut bus as *mut libc::c_int,
    );
    if rv != 1i32 {
      continue;
    }
    if !is_prefixed_with(
      name.as_mut_ptr(),
      b"ISA\x00" as *const u8 as *const libc::c_char,
    )
    .is_null()
    {
      adt = ADT_ISA
    } else {
      adt = i2cdetect_get_funcs(bus)
    }
    printf(
      b"i2c-%d\t%-10s\t%-32s\t%s\n\x00" as *const u8 as *const libc::c_char,
      bus,
      adap_descs[adt as usize].funcs,
      name.as_mut_ptr(),
      adap_descs[adt as usize].algo,
    );
  }
  exit(0i32);
}
unsafe extern "C" fn no_support(mut cmd: *const libc::c_char) -> ! {
  bb_error_msg_and_die(
    b"bus doesn\'t support %s\x00" as *const u8 as *const libc::c_char,
    cmd,
  );
}
unsafe extern "C" fn will_skip(mut cmd: *const libc::c_char) {
  bb_error_msg(
    b"warning: can\'t use %s command, will skip some addresses\x00" as *const u8
      as *const libc::c_char,
    cmd,
  );
}
//usage:#define i2cdetect_trivial_usage
//usage:       "-l | -F I2CBUS | [-ya] [-q|-r] I2CBUS [FIRST LAST]"
//usage:#define i2cdetect_full_usage "\n\n"
//usage:       "Detect I2C chips"
//usage:     "\n"
//usage:     "\n	-l	List installed buses"
//usage:     "\n	-F BUS#	List functionalities on this bus"
//usage:     "\n	-y	Disable interactive mode"
//usage:     "\n	-a	Force scanning of non-regular addresses"
//usage:     "\n	-q	Use smbus quick write commands for probing (default)"
//usage:     "\n	-r	Use smbus read byte commands for probing"
//usage:     "\n	FIRST and LAST limit probing range"
#[no_mangle]
pub unsafe extern "C" fn i2cdetect_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let opt_y_0: libc::c_uint = (1i32 << 0i32) as libc::c_uint;
  let opt_a_0: libc::c_uint = (1i32 << 1i32) as libc::c_uint;
  let opt_q: libc::c_uint = (1i32 << 2i32) as libc::c_uint;
  let opt_r: libc::c_uint = (1i32 << 3i32) as libc::c_uint;
  let opt_F: libc::c_uint = (1i32 << 4i32) as libc::c_uint;
  let opt_l: libc::c_uint = (1i32 << 5i32) as libc::c_uint;
  let mut fd: libc::c_int = 0;
  let mut bus_num: libc::c_int = 0;
  let mut i: libc::c_int = 0;
  let mut j: libc::c_int = 0;
  let mut mode: libc::c_int = 0i32;
  let mut status: libc::c_int = 0;
  let mut cmd: libc::c_int = 0;
  let mut first: libc::c_uint = 0x3i32 as libc::c_uint;
  let mut last: libc::c_uint = 0x77i32 as libc::c_uint;
  let mut opts: libc::c_uint = 0;
  let mut funcs: libc::c_ulong = 0;
  opts = getopt32(
    argv,
    b"^yaqrFl\x00q--r:r--q:?3\x00" as *const u8 as *const libc::c_char,
  );
  argv = argv.offset(optind as isize);
  if opts & opt_l != 0 {
    list_i2c_busses_and_exit();
  }
  if (*argv.offset(0)).is_null() {
    bb_show_usage();
  }
  bus_num = i2c_bus_lookup(*argv.offset(0));
  fd = i2c_dev_open(bus_num);
  get_funcs_matrix(fd, &mut funcs);
  if opts & opt_F != 0 {
    /* Only list the functionalities. */
    printf(
      b"Functionalities implemented by bus #%d\n\x00" as *const u8 as *const libc::c_char,
      bus_num,
    );
    i = 0i32;
    while i2c_funcs_tab[i as usize].value != 0 {
      printf(
        b"%-32s %s\n\x00" as *const u8 as *const libc::c_char,
        i2c_funcs_tab[i as usize].name,
        if funcs & i2c_funcs_tab[i as usize].value as libc::c_ulong != 0 {
          b"yes\x00" as *const u8 as *const libc::c_char
        } else {
          b"no\x00" as *const u8 as *const libc::c_char
        },
      );
      i += 1
    }
    return 0i32;
  }
  if opts & opt_r != 0 {
    mode = 2i32
  } else if opts & opt_q != 0 {
    mode = 1i32
  }
  if opts & opt_a_0 != 0 {
    first = 0i32 as libc::c_uint;
    last = 0x7fi32 as libc::c_uint
  }
  /* Read address range. */
  if !(*argv.offset(1)).is_null() {
    first = xstrtou_range(*argv.offset(1), 16i32, first, last);
    if !(*argv.offset(2)).is_null() {
      last = xstrtou_range(*argv.offset(2), 16i32, first, last)
    }
  }
  if funcs & (0x10000i32 | 0x20000i32) as libc::c_ulong == 0 {
    no_support(b"detection commands\x00" as *const u8 as *const libc::c_char);
  } else {
    if mode == 1i32 && funcs & 0x10000i32 as libc::c_ulong == 0 {
      no_support(b"SMBus quick write\x00" as *const u8 as *const libc::c_char);
    } else {
      if mode == 2i32 && funcs & 0x20000i32 as libc::c_ulong == 0 {
        no_support(b"SMBus receive byte\x00" as *const u8 as *const libc::c_char);
      }
    }
  }
  if mode == 0i32 {
    if funcs & 0x10000i32 as libc::c_ulong == 0 {
      will_skip(b"SMBus quick write\x00" as *const u8 as *const libc::c_char);
    }
    if funcs & 0x20000i32 as libc::c_ulong == 0 {
      will_skip(b"SMBus receive byte\x00" as *const u8 as *const libc::c_char);
    }
  }
  if opts & opt_y_0 == 0 {
    confirm_action(-1i32, -1i32, -1i32, 0i32);
  }
  puts(
    b"     0  1  2  3  4  5  6  7  8  9  a  b  c  d  e  f\x00" as *const u8 as *const libc::c_char,
  );
  i = 0i32;
  while i < 128i32 {
    printf(b"%02x: \x00" as *const u8 as *const libc::c_char, i);
    j = 0i32;
    while j < 16i32 {
      fflush_all();
      cmd = mode;
      if mode == 0i32 {
        if i + j >= 0x30i32 && i + j <= 0x37i32 || i + j >= 0x50i32 && i + j <= 0x5fi32 {
          cmd = 2i32
        } else {
          cmd = 1i32
        }
      }
      /* Skip unwanted addresses. */
      if ((i + j) as libc::c_uint) < first
        || (i + j) as libc::c_uint > last
        || cmd == 2i32 && funcs & 0x20000i32 as libc::c_ulong == 0
        || cmd == 1i32 && funcs & 0x10000i32 as libc::c_ulong == 0
      {
        printf(b"   \x00" as *const u8 as *const libc::c_char);
      } else {
        status = ioctl(fd, 0x703i32 as libc::c_ulong, itoptr(i + j));
        if status < 0i32 {
          if *bb_errno == 16i32 {
            printf(b"UU \x00" as *const u8 as *const libc::c_char);
          } else {
            bb_perror_msg_and_die(
              b"can\'t set address to 0x%02x\x00" as *const u8 as *const libc::c_char,
              i + j,
            );
          }
        } else {
          match cmd {
            2 => {
              /*
               * This is known to lock SMBus on various
               * write-only chips (mainly clock chips).
               */
              status = i2c_smbus_read_byte(fd)
            }
            _ => {
              /* I2CDETECT_MODE_QUICK: */
              /*
               * This is known to corrupt the Atmel
               * AT24RF08 EEPROM.
               */
              status = i2c_smbus_write_quick(fd, 0i32 as uint8_t)
            }
          }
          if status < 0i32 {
            printf(b"-- \x00" as *const u8 as *const libc::c_char);
          } else {
            printf(b"%02x \x00" as *const u8 as *const libc::c_char, i + j);
          }
        }
      }
      j += 1
    }
    bb_putchar('\n' as i32);
    i += 16i32
  }
  return 0i32;
}
/* ENABLE_I2CDETECT */
unsafe extern "C" fn check_i2c_func(mut fd: libc::c_int) {
  let mut funcs: libc::c_ulong = 0;
  get_funcs_matrix(fd, &mut funcs);
  if funcs & 0x1i32 as libc::c_ulong == 0 {
    bb_simple_error_msg_and_die(
      b"adapter does not support I2C transfers\x00" as *const u8 as *const libc::c_char,
    );
  };
}
//usage:#define i2ctransfer_trivial_usage
//usage:       "[-fay] I2CBUS {rLENGTH[@ADDR] | wLENGTH[@ADDR] DATA...}..."
//usage:#define i2ctransfer_full_usage "\n\n"
//usage:       "Read/write I2C data in one transfer"
//usage:     "\n"
//usage:     "\n	-f	Force access to busy addresses"
//usage:     "\n	-a	Force access to non-regular addresses"
//usage:     "\n	-y	Disable interactive mode"
#[no_mangle]
pub unsafe extern "C" fn i2ctransfer_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut bus_num: libc::c_int = 0;
  let mut bus_addr: libc::c_int = 0;
  let mut fd: libc::c_int = 0;
  let mut opts: libc::c_uint = 0;
  let mut first: libc::c_uint = 0;
  let mut last: libc::c_uint = 0;
  let mut nmsgs: libc::c_int = 0;
  let mut nmsgs_sent: libc::c_int = 0;
  let mut i: libc::c_int = 0;
  let mut msgs: [i2c_msg; 42] = [i2c_msg {
    addr: 0,
    flags: 0,
    len: 0,
    buf: 0 as *mut __u8,
  }; 42];
  let mut rdwr: i2c_rdwr_ioctl_data = i2c_rdwr_ioctl_data {
    msgs: 0 as *mut i2c_msg,
    nmsgs: 0,
  };
  memset(
    msgs.as_mut_ptr() as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<[i2c_msg; 42]>() as libc::c_ulong,
  );
  opts = getopt32(argv, b"^fya\x00-2\x00" as *const u8 as *const libc::c_char);
  first = 0x3i32 as libc::c_uint;
  last = 0x77i32 as libc::c_uint;
  if opts & opt_a as libc::c_int as libc::c_uint != 0 {
    first = 0i32 as libc::c_uint;
    last = 0x7fi32 as libc::c_uint
  }
  argv = argv.offset(optind as isize);
  bus_num = i2c_bus_lookup(*argv.offset(0));
  fd = i2c_dev_open(bus_num);
  check_i2c_func(fd);
  bus_addr = -1i32;
  nmsgs = 0i32;
  loop {
    argv = argv.offset(1);
    if (*argv).is_null() {
      break;
    }
    let mut arg_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_uint = 0;
    let mut flags: uint16_t = 0;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    if nmsgs >= 42i32 {
      bb_simple_error_msg_and_die(
        b"too many messages, max: 42\x00" as *const u8 as *const libc::c_char,
      );
    }
    flags = 0i32 as uint16_t;
    arg_ptr = *argv;
    let fresh2 = arg_ptr;
    arg_ptr = arg_ptr.offset(1);
    match *fresh2 as libc::c_int {
      114 => flags = (flags as libc::c_int | 0x1i32) as uint16_t,
      119 => {}
      _ => {
        bb_show_usage();
      }
    }
    end = strchr(arg_ptr, '@' as i32);
    if !end.is_null() {
      *end = '\u{0}' as i32 as libc::c_char
    }
    len = xstrtou_range(
      arg_ptr,
      0i32,
      0i32 as libc::c_uint,
      0xffffi32 as libc::c_uint,
    );
    if !end.is_null() {
      bus_addr = xstrtou_range(end.offset(1), 0i32, first, last) as libc::c_int;
      i2c_set_slave_addr(
        fd,
        bus_addr,
        (opts & opt_f as libc::c_int as libc::c_uint) as libc::c_int,
      );
    } else if bus_addr < 0i32 {
      bb_error_msg_and_die(
        b"no address given in \'%s\'\x00" as *const u8 as *const libc::c_char,
        *argv,
      );
    }
    msgs[nmsgs as usize].addr = bus_addr as __u16;
    msgs[nmsgs as usize].flags = flags;
    msgs[nmsgs as usize].len = len as __u16;
    if len != 0 {
      msgs[nmsgs as usize].buf = xzalloc(len as size_t) as *mut __u8
    }
    if flags as libc::c_int & 0x1i32 == 0 {
      /* Reuse last address if possible */
      /* Consume DATA arg(s) */
      let mut buf_idx: libc::c_uint = 0i32 as libc::c_uint;
      while buf_idx < len {
        let mut data8: uint8_t = 0;
        let mut data: libc::c_ulong = 0;
        argv = argv.offset(1);
        arg_ptr = *argv;
        if arg_ptr.is_null() {
          bb_show_usage();
        }
        data = strtoul(arg_ptr, &mut end, 0i32);
        if data > 0xffi32 as libc::c_ulong || arg_ptr == end {
          bb_error_msg_and_die(
            b"invalid data byte \'%s\'\x00" as *const u8 as *const libc::c_char,
            *argv,
          );
        }
        data8 = data as uint8_t;
        while buf_idx < len {
          let fresh3 = buf_idx;
          buf_idx = buf_idx.wrapping_add(1);
          *msgs[nmsgs as usize].buf.offset(fresh3 as isize) = data8;
          if *end == 0 {
            break;
          }
          match *end as libc::c_int {
            112 => {
              /* Pseudo randomness (8 bit AXR with a=13 and b=27) */
              data8 = ((data8 as libc::c_int ^ 27i32) + 13i32) as uint8_t;
              data8 = ((data8 as libc::c_int) << 1i32 | data8 as libc::c_int >> 7i32) as uint8_t
            }
            43 => data8 = data8.wrapping_add(1),
            45 => data8 = data8.wrapping_sub(1),
            61 => {}
            _ => {
              bb_error_msg_and_die(
                b"invalid data byte suffix: \'%s\'\x00" as *const u8 as *const libc::c_char,
                *argv,
              );
            }
          }
        }
      }
    }
    nmsgs += 1
  }
  if opts & opt_y as libc::c_int as libc::c_uint == 0 {
    confirm_action(bus_addr, 0i32, 0i32, 0i32);
  }
  rdwr.msgs = msgs.as_mut_ptr();
  rdwr.nmsgs = nmsgs as __u32;
  nmsgs_sent = ioctl_or_perror_and_die(
    fd,
    0x707i32 as libc::c_uint,
    &mut rdwr as *mut i2c_rdwr_ioctl_data as *mut libc::c_void,
    b"I2C_RDWR\x00" as *const u8 as *const libc::c_char,
  );
  if nmsgs_sent < nmsgs {
    bb_error_msg(
      b"warning: only %u/%u messages sent\x00" as *const u8 as *const libc::c_char,
      nmsgs_sent,
      nmsgs,
    );
  }
  i = 0i32;
  while i < nmsgs_sent {
    if msgs[i as usize].len as libc::c_int != 0i32
      && msgs[i as usize].flags as libc::c_int & 0x1i32 != 0
    {
      let mut j: libc::c_int = 0;
      j = 0i32;
      while j < msgs[i as usize].len as libc::c_int - 1i32 {
        printf(
          b"0x%02x \x00" as *const u8 as *const libc::c_char,
          *msgs[i as usize].buf.offset(j as isize) as libc::c_int,
        );
        j += 1
      }
      /* Print final byte with newline */
      printf(
        b"0x%02x\n\x00" as *const u8 as *const libc::c_char,
        *msgs[i as usize].buf.offset(j as isize) as libc::c_int,
      );
    }
    i += 1
  }
  return 0i32;
}
/* ENABLE_I2CTRANSFER */
