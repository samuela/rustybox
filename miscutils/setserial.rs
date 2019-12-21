use libc;
use libc::close;
use libc::ioctl;
use libc::printf;
use libc::puts;
use libc::strcasecmp;
extern "C" {
  #[no_mangle]
  fn exit(_: libc::c_int) -> !;
  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  static bb_msg_requires_arg: [libc::c_char; 0];

  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
}

use crate::librb::size_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct serial_struct {
  pub type_0: libc::c_int,
  pub line: libc::c_int,
  pub port: libc::c_uint,
  pub irq: libc::c_int,
  pub flags: libc::c_int,
  pub xmit_fifo_size: libc::c_int,
  pub custom_divisor: libc::c_int,
  pub baud_base: libc::c_int,
  pub close_delay: libc::c_ushort,
  pub io_type: libc::c_char,
  pub reserved_char: [libc::c_char; 1],
  pub hub6: libc::c_int,
  pub closing_wait: libc::c_ushort,
  pub closing_wait2: libc::c_ushort,
  pub iomem_base: *mut libc::c_uchar,
  pub iomem_reg_shift: libc::c_ushort,
  pub port_high: libc::c_uint,
  pub iomap_base: libc::c_ulong,
  /* cookie passed into ioremap */
}
pub type print_mode = libc::c_uint;
pub const PRINT_ALL: print_mode = 4;
pub const PRINT_FEDBACK: print_mode = 2;
pub const PRINT_SUMMARY: print_mode = 1;
pub const PRINT_NORMAL: print_mode = 0;
pub type C2RustUnnamed = libc::c_uint;
pub const CMD_FLAG_LAST: C2RustUnnamed = 15;
pub const CMD_FLAG_FIRST: C2RustUnnamed = 6;
pub const CMD_AUTOCONFIG: C2RustUnnamed = 23;
pub const CMD_WAIT: C2RustUnnamed = 22;
pub const CMD_DELAY: C2RustUnnamed = 21;
pub const CMD_BASE: C2RustUnnamed = 20;
pub const CMD_UART: C2RustUnnamed = 19;
pub const CMD_DIVISOR: C2RustUnnamed = 18;
pub const CMD_IRQ: C2RustUnnamed = 17;
pub const CMD_PORT: C2RustUnnamed = 16;
pub const CMD_FLAG_LOW_LATENCY: C2RustUnnamed = 15;
pub const CMD_FLAG_CALLOUT_NOHUP: C2RustUnnamed = 14;
pub const CMD_FLAG_PGRP_LOCKOUT: C2RustUnnamed = 13;
pub const CMD_FLAG_SESSION_LOCKOUT: C2RustUnnamed = 12;
pub const CMD_FLAG_SPLIT_TERMIOS: C2RustUnnamed = 11;
pub const CMD_FLAG_AUTO_IRQ: C2RustUnnamed = 10;
pub const CMD_FLAG_SKIP_TEST: C2RustUnnamed = 9;
pub const CMD_FLAG_NUP_NOTIFY: C2RustUnnamed = 8;
pub const CMD_FLAG_FOURPORT: C2RustUnnamed = 7;
pub const CMD_FLAG_SAK: C2RustUnnamed = 6;
pub const CMD_SPD_CUST: C2RustUnnamed = 5;
pub const CMD_SPD_WARP: C2RustUnnamed = 4;
pub const CMD_SPD_SHI: C2RustUnnamed = 3;
pub const CMD_SPD_VHI: C2RustUnnamed = 2;
pub const CMD_SPD_HI: C2RustUnnamed = 1;
pub const CMD_SPD_NORMAL: C2RustUnnamed = 0;
#[inline(always)]
unsafe extern "C" fn bb_strtol(
  mut arg: *const libc::c_char,
  mut endp: *mut *mut libc::c_char,
  mut base: libc::c_int,
) -> libc::c_long {
  return crate::libbb::bb_strtonum::bb_strtoll(arg, endp, base) as libc::c_long;
}
static mut serial_types: [libc::c_char; 132] = [
  117, 110, 107, 110, 111, 119, 110, 0, 56, 50, 53, 48, 0, 49, 54, 52, 53, 48, 0, 49, 54, 53, 53,
  48, 0, 49, 54, 53, 53, 48, 65, 0, 67, 105, 114, 114, 117, 115, 0, 49, 54, 54, 53, 48, 0, 49, 54,
  54, 53, 48, 86, 50, 0, 49, 54, 55, 53, 48, 0, 49, 54, 57, 53, 48, 0, 49, 54, 57, 53, 52, 0, 49,
  54, 54, 53, 52, 0, 49, 54, 56, 53, 48, 0, 82, 83, 65, 0, 78, 83, 49, 54, 53, 53, 48, 65, 0, 88,
  83, 67, 65, 76, 69, 0, 82, 77, 57, 48, 48, 48, 0, 79, 67, 84, 69, 79, 78, 0, 65, 82, 55, 0, 85,
  54, 95, 49, 54, 53, 53, 48, 65, 0, 0,
];
static mut commands: [libc::c_char; 233] = [
  115, 112, 100, 95, 110, 111, 114, 109, 97, 108, 0, 115, 112, 100, 95, 104, 105, 0, 115, 112, 100,
  95, 118, 104, 105, 0, 115, 112, 100, 95, 115, 104, 105, 0, 115, 112, 100, 95, 119, 97, 114, 112,
  0, 115, 112, 100, 95, 99, 117, 115, 116, 0, 115, 97, 107, 0, 102, 111, 117, 114, 112, 111, 114,
  116, 0, 104, 117, 112, 95, 110, 111, 116, 105, 102, 121, 0, 115, 107, 105, 112, 95, 116, 101,
  115, 116, 0, 97, 117, 116, 111, 95, 105, 114, 113, 0, 115, 112, 108, 105, 116, 95, 116, 101, 114,
  109, 105, 111, 115, 0, 115, 101, 115, 115, 105, 111, 110, 95, 108, 111, 99, 107, 111, 117, 116,
  0, 112, 103, 114, 112, 95, 108, 111, 99, 107, 111, 117, 116, 0, 99, 97, 108, 108, 111, 117, 116,
  95, 110, 111, 104, 117, 112, 0, 108, 111, 119, 95, 108, 97, 116, 101, 110, 99, 121, 0, 112, 111,
  114, 116, 0, 105, 114, 113, 0, 100, 105, 118, 105, 115, 111, 114, 0, 117, 97, 114, 116, 0, 98,
  97, 117, 100, 95, 98, 97, 115, 101, 0, 99, 108, 111, 115, 101, 95, 100, 101, 108, 97, 121, 0, 99,
  108, 111, 115, 105, 110, 103, 95, 119, 97, 105, 116, 0, 97, 117, 116, 111, 99, 111, 110, 102,
  105, 103, 0, 0,
];
unsafe extern "C" fn cmd_noprint(mut cmd: libc::c_int) -> bool {
  return cmd >= CMD_FLAG_SKIP_TEST as libc::c_int && cmd <= CMD_FLAG_CALLOUT_NOHUP as libc::c_int;
}
unsafe extern "C" fn cmd_is_flag(mut cmd: libc::c_int) -> bool {
  return cmd >= CMD_FLAG_FIRST as libc::c_int && cmd <= CMD_FLAG_LAST as libc::c_int;
}
unsafe extern "C" fn cmd_needs_arg(mut cmd: libc::c_int) -> bool {
  return cmd >= CMD_PORT as libc::c_int && cmd <= CMD_WAIT as libc::c_int;
}
static mut setbits: [u16; 16] = [
  0i32 as u16,
  (1u32 << 4i32) as u16,
  (1u32 << 5i32) as u16,
  (1u32 << 12i32) as u16,
  (1u32 << 4i32 | 1u32 << 12i32) as u16,
  (1u32 << 4i32 | 1u32 << 5i32) as u16,
  (1u32 << 2i32) as u16,
  (1u32 << 1i32) as u16,
  (1u32 << 0i32) as u16,
  (1u32 << 6i32) as u16,
  (1u32 << 7i32) as u16,
  (1u32 << 3i32) as u16,
  (1u32 << 8i32) as u16,
  (1u32 << 9i32) as u16,
  (1u32 << 10i32) as u16,
  (1u32 << 13i32) as u16,
];
unsafe extern "C" fn uart_type(mut type_0: libc::c_int) -> *const libc::c_char {
  if type_0 > 19i32 {
    return b"undefined\x00" as *const u8 as *const libc::c_char;
  }
  return crate::libbb::compare_string_array::nth_string(serial_types.as_ptr(), type_0);
}
/* libbb candidate */
unsafe extern "C" fn index_in_strings_case_insensitive(
  mut strings: *const libc::c_char,
  mut key: *const libc::c_char,
) -> libc::c_int {
  let mut idx: libc::c_int = 0i32; /* skip NUL */
  while *strings != 0 {
    if strcasecmp(strings, key) == 0i32 {
      return idx;
    }
    strings = strings.offset(strlen(strings).wrapping_add(1i32 as libc::c_ulong) as isize);
    idx += 1
  }
  return -1i32;
}
unsafe extern "C" fn uart_id(mut name: *const libc::c_char) -> libc::c_int {
  return index_in_strings_case_insensitive(serial_types.as_ptr(), name);
}
unsafe extern "C" fn get_spd(mut flags: libc::c_int, mut mode: print_mode) -> *const libc::c_char {
  let mut idx: libc::c_int = 0;
  match flags as libc::c_uint & (1u32 << 4i32 | 1u32 << 5i32 | 1u32 << 12i32) {
    16 => idx = CMD_SPD_HI as libc::c_int,
    32 => idx = CMD_SPD_VHI as libc::c_int,
    4096 => idx = CMD_SPD_SHI as libc::c_int,
    4112 => idx = CMD_SPD_WARP as libc::c_int,
    48 => idx = CMD_SPD_CUST as libc::c_int,
    _ => {
      if (mode as libc::c_uint) < PRINT_FEDBACK as libc::c_int as libc::c_uint {
        return 0 as *const libc::c_char;
      }
      idx = CMD_SPD_NORMAL as libc::c_int
    }
  }
  return crate::libbb::compare_string_array::nth_string(commands.as_ptr(), idx);
}
unsafe extern "C" fn get_numeric(mut arg: *const libc::c_char) -> libc::c_int {
  return bb_strtol(arg, 0 as *mut *mut libc::c_char, 0i32) as libc::c_int;
}
unsafe extern "C" fn get_wait(mut arg: *const libc::c_char) -> libc::c_int {
  if strcasecmp(arg, b"none\x00" as *const u8 as *const libc::c_char) == 0i32 {
    return 65535i32;
  }
  if strcasecmp(arg, b"infinite\x00" as *const u8 as *const libc::c_char) == 0i32 {
    return 0i32;
  }
  return get_numeric(arg);
}
unsafe extern "C" fn get_uart(mut arg: *const libc::c_char) -> libc::c_int {
  let mut uart: libc::c_int = uart_id(arg);
  if uart < 0i32 {
    crate::libbb::verror_msg::bb_error_msg_and_die(
      b"illegal UART type: %s\x00" as *const u8 as *const libc::c_char,
      arg,
    );
  }
  return uart;
}
unsafe extern "C" fn serial_open(mut dev: *const libc::c_char, mut quiet: bool) -> libc::c_int {
  let mut fd: libc::c_int = 0;
  fd = crate::libbb::device_open::device_open(dev, 0o2i32 | 0o4000i32);
  if fd < 0i32 && !quiet {
    crate::libbb::perror_msg::bb_simple_perror_msg(dev);
  }
  return fd;
}
unsafe extern "C" fn serial_ctl(
  mut fd: libc::c_int,
  mut ops: libc::c_int,
  mut serinfo: *mut serial_struct,
) -> libc::c_int {
  let mut current_block: u64;
  let mut ret: libc::c_int = 0i32;
  let mut err: *const libc::c_char = 0 as *const libc::c_char;
  if ops & 1i32 << 0i32 != 0 {
    ret = ioctl(fd, 0x541fi32 as libc::c_ulong, serinfo);
    if ret < 0i32 {
      err = b"can\'t set serial info\x00" as *const u8 as *const libc::c_char;
      current_block = 183908852989203104;
    } else {
      current_block = 11875828834189669668;
    }
  } else {
    current_block = 11875828834189669668;
  }
  match current_block {
    11875828834189669668 => {
      if ops & 1i32 << 1i32 != 0 {
        ret = ioctl(fd, 0x5453i32 as libc::c_ulong);
        if ret < 0i32 {
          err = b"can\'t autoconfigure port\x00" as *const u8 as *const libc::c_char;
          current_block = 183908852989203104;
        } else {
          current_block = 13536709405535804910;
        }
      } else {
        current_block = 13536709405535804910;
      }
      match current_block {
        183908852989203104 => {}
        _ => {
          if ops & 1i32 << 2i32 != 0 {
            ret = ioctl(fd, 0x541ei32 as libc::c_ulong, serinfo);
            if ret < 0i32 {
              err = b"can\'t get serial info\x00" as *const u8 as *const libc::c_char;
              current_block = 183908852989203104;
            } else {
              current_block = 1858477412628151261;
            }
          } else {
            current_block = 1858477412628151261;
          }
        }
      }
    }
    _ => {}
  }
  match current_block {
    183908852989203104 => {
      crate::libbb::perror_msg::bb_simple_perror_msg(err);
      if !(ops & 1i32 << 4i32 != 0) {
        exit(1i32);
      }
    }
    _ => {}
  }
  if ops & 1i32 << 3i32 != 0 {
    close(fd);
  }
  return ret;
}
unsafe extern "C" fn print_flag(
  mut prefix: *mut *const libc::c_char,
  mut flag: *const libc::c_char,
) {
  printf(
    b"%s%s\x00" as *const u8 as *const libc::c_char,
    *prefix,
    flag,
  );
  *prefix = b" \x00" as *const u8 as *const libc::c_char;
}
unsafe extern "C" fn print_serial_flags(
  mut serial_flags: libc::c_int,
  mut mode: print_mode,
  mut prefix: *const libc::c_char,
  mut postfix: *const libc::c_char,
) {
  let mut i: libc::c_int = 0;
  let mut spd: *const libc::c_char = 0 as *const libc::c_char;
  let mut pr: *const libc::c_char = 0 as *const libc::c_char;
  pr = prefix;
  spd = get_spd(serial_flags, mode);
  if !spd.is_null() {
    print_flag(&mut pr, spd);
  }
  i = CMD_FLAG_FIRST as libc::c_int;
  while i <= CMD_FLAG_LAST as libc::c_int {
    if serial_flags & setbits[i as usize] as libc::c_int != 0
      && (mode as libc::c_uint > PRINT_SUMMARY as libc::c_int as libc::c_uint || !cmd_noprint(i))
    {
      print_flag(
        &mut pr,
        crate::libbb::compare_string_array::nth_string(commands.as_ptr(), i),
      );
    }
    i += 1
  }
  puts(if pr == prefix {
    b"\x00" as *const u8 as *const libc::c_char
  } else {
    postfix
  });
}
unsafe extern "C" fn print_closing_wait(mut closing_wait: libc::c_uint) {
  match closing_wait {
    65535 => {
      puts(b"none\x00" as *const u8 as *const libc::c_char);
    }
    0 => {
      puts(b"infinite\x00" as *const u8 as *const libc::c_char);
    }
    _ => {
      printf(
        b"%u\n\x00" as *const u8 as *const libc::c_char,
        closing_wait,
      );
    }
  };
}
unsafe extern "C" fn serial_get(mut device: *const libc::c_char, mut mode: print_mode) {
  let mut fd: libc::c_int = 0;
  let mut ret: libc::c_int = 0;
  let mut uart: *const libc::c_char = 0 as *const libc::c_char;
  let mut prefix: *const libc::c_char = 0 as *const libc::c_char;
  let mut postfix: *const libc::c_char = 0 as *const libc::c_char;
  let mut serinfo: serial_struct = serial_struct {
    type_0: 0,
    line: 0,
    port: 0,
    irq: 0,
    flags: 0,
    xmit_fifo_size: 0,
    custom_divisor: 0,
    baud_base: 0,
    close_delay: 0,
    io_type: 0,
    reserved_char: [0; 1],
    hub6: 0,
    closing_wait: 0,
    closing_wait2: 0,
    iomem_base: 0 as *mut libc::c_uchar,
    iomem_reg_shift: 0,
    port_high: 0,
    iomap_base: 0,
  };
  fd = serial_open(
    device,
    mode as libc::c_uint == PRINT_SUMMARY as libc::c_int as libc::c_uint,
  );
  if fd < 0i32 {
    return;
  }
  ret = serial_ctl(fd, 1i32 << 2i32 | 1i32 << 3i32 | 1i32 << 4i32, &mut serinfo);
  if ret < 0i32 {
    return;
  }
  uart = uart_type(serinfo.type_0);
  prefix = b", Flags: \x00" as *const u8 as *const libc::c_char;
  postfix = b"\x00" as *const u8 as *const libc::c_char;
  match mode as libc::c_uint {
    0 => {
      printf(
        b"%s, UART: %s, Port: 0x%.4x, IRQ: %d\x00" as *const u8 as *const libc::c_char,
        device,
        uart,
        serinfo.port,
        serinfo.irq,
      );
    }
    1 => {
      if serinfo.type_0 == 0 {
        return;
      }
      printf(
        b"%s at 0x%.4x (irq = %d) is a %s\x00" as *const u8 as *const libc::c_char,
        device,
        serinfo.port,
        serinfo.irq,
        uart,
      );
      prefix = b" (\x00" as *const u8 as *const libc::c_char;
      postfix = b")\x00" as *const u8 as *const libc::c_char
    }
    2 => {
      printf(
        b"%s uart %s port 0x%.4x irq %d baud_base %d\x00" as *const u8 as *const libc::c_char,
        device,
        uart,
        serinfo.port,
        serinfo.irq,
        serinfo.baud_base,
      );
      prefix = b" \x00" as *const u8 as *const libc::c_char
    }
    4 => {
      printf(
        b"%s, Line %d, UART: %s, Port: 0x%.4x, IRQ: %d\n\x00" as *const u8 as *const libc::c_char,
        device,
        serinfo.line,
        uart,
        serinfo.port,
        serinfo.irq,
      );
      printf(
        b"\tBaud_base: %d, close_delay: %u, divisor: %d\n\x00" as *const u8 as *const libc::c_char,
        serinfo.baud_base,
        serinfo.close_delay as libc::c_int,
        serinfo.custom_divisor,
      );
      printf(b"\tclosing_wait: \x00" as *const u8 as *const libc::c_char);
      print_closing_wait(serinfo.closing_wait as libc::c_uint);
      prefix = b"\tFlags: \x00" as *const u8 as *const libc::c_char;
      postfix = b"\n\x00" as *const u8 as *const libc::c_char
    }
    _ => {}
  }
  print_serial_flags(serinfo.flags, mode, prefix, postfix);
}
unsafe extern "C" fn find_cmd(mut cmd: *const libc::c_char) -> libc::c_int {
  let mut idx: libc::c_int = 0;
  idx = index_in_strings_case_insensitive(commands.as_ptr(), cmd);
  if idx < 0i32 {
    crate::libbb::verror_msg::bb_error_msg_and_die(
      b"invalid flag: %s\x00" as *const u8 as *const libc::c_char,
      cmd,
    );
  }
  return idx;
}
unsafe extern "C" fn serial_set(mut arg: *mut *mut libc::c_char, mut opts: libc::c_int) {
  let mut serinfo: serial_struct = serial_struct {
    type_0: 0,
    line: 0,
    port: 0,
    irq: 0,
    flags: 0,
    xmit_fifo_size: 0,
    custom_divisor: 0,
    baud_base: 0,
    close_delay: 0,
    io_type: 0,
    reserved_char: [0; 1],
    hub6: 0,
    closing_wait: 0,
    closing_wait2: 0,
    iomem_base: 0 as *mut libc::c_uchar,
    iomem_reg_shift: 0,
    port_high: 0,
    iomap_base: 0,
  };
  let mut fd: libc::c_int = 0;
  fd = serial_open(*arg, 0i32 != 0);
  if fd < 0i32 {
    exit(201i32);
  }
  serial_ctl(fd, 1i32 << 2i32, &mut serinfo);
  if opts & 1i32 << 4i32 != 0 {
    serinfo.flags = 0i32
  }
  loop {
    arg = arg.offset(1);
    if (*arg).is_null() {
      break;
    }
    let mut word: *const libc::c_char = 0 as *const libc::c_char;
    let mut invert: libc::c_int = 0;
    let mut cmd: libc::c_int = 0;
    word = *arg;
    invert = (*word.offset(0) as libc::c_int == '^' as i32) as libc::c_int;
    word = word.offset(invert as isize);
    cmd = find_cmd(word);
    if cmd_needs_arg(cmd) {
      arg = arg.offset(1);
      if (*arg).is_null() {
        crate::libbb::verror_msg::bb_error_msg_and_die(bb_msg_requires_arg.as_ptr(), word);
      }
    }
    if invert != 0 && !cmd_is_flag(cmd) {
      crate::libbb::verror_msg::bb_error_msg_and_die(
        b"can\'t invert %s\x00" as *const u8 as *const libc::c_char,
        word,
      );
    }
    let mut current_block_27: u64;
    match cmd {
      0 | 1 | 2 | 3 | 4 | 5 => {
        serinfo.flags = (serinfo.flags as libc::c_uint
          & !(1u32 << 4i32 | 1u32 << 5i32 | 1u32 << 12i32)) as libc::c_int;
        current_block_27 = 2860297398562693489;
      }
      6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 => {
        current_block_27 = 2860297398562693489;
      }
      16 => {
        serinfo.port = get_numeric(*arg) as libc::c_uint;
        current_block_27 = 12199444798915819164;
      }
      17 => {
        serinfo.irq = get_numeric(*arg);
        current_block_27 = 12199444798915819164;
      }
      18 => {
        serinfo.custom_divisor = get_numeric(*arg);
        current_block_27 = 12199444798915819164;
      }
      19 => {
        serinfo.type_0 = get_uart(*arg);
        current_block_27 = 12199444798915819164;
      }
      20 => {
        serinfo.baud_base = get_numeric(*arg);
        current_block_27 = 12199444798915819164;
      }
      21 => {
        serinfo.close_delay = get_numeric(*arg) as libc::c_ushort;
        current_block_27 = 12199444798915819164;
      }
      22 => {
        serinfo.closing_wait = get_wait(*arg) as libc::c_ushort;
        current_block_27 = 12199444798915819164;
      }
      23 => {
        serial_ctl(fd, 1i32 << 0i32 | 1i32 << 1i32 | 1i32 << 2i32, &mut serinfo);
        current_block_27 = 12199444798915819164;
      }
      _ => {
        current_block_27 = 12199444798915819164;
      }
    }
    match current_block_27 {
      2860297398562693489 =>
      /* fallthrough */
      {
        if invert != 0 {
          serinfo.flags &= !(setbits[cmd as usize] as libc::c_int)
        } else {
          serinfo.flags |= setbits[cmd as usize] as libc::c_int
        }
      }
      _ => {}
    }
  } /* force display */
  serial_ctl(fd, 1i32 << 0i32 | 1i32 << 3i32, &mut serinfo);
}
#[no_mangle]
pub unsafe extern "C" fn setserial_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut opts: libc::c_int = 0;
  opts = crate::libbb::getopt32::getopt32(
    argv,
    b"^bGavzgq\x00-1:b-aG:G-ab:a-bG\x00" as *const u8 as *const libc::c_char,
  ) as libc::c_int;
  argv = argv.offset(optind as isize);
  if (*argv.offset(1)).is_null() {
    /* one arg only? (nothing to change?) */
    opts |= 1i32 << 5i32
  }
  if opts & 1i32 << 5i32 == 0 {
    serial_set(argv, opts);
    let ref mut fresh0 = *argv.offset(1);
    *fresh0 = std::ptr::null_mut::<libc::c_char>()
  }
  /* -v effect: "after setting params, do not be silent, show them" */
  if opts & (1i32 << 3i32 | 1i32 << 5i32) != 0 {
    loop {
      serial_get(
        *argv,
        (opts & (1i32 << 2i32 | 1i32 << 0i32 | 1i32 << 1i32)) as print_mode,
      );
      argv = argv.offset(1);
      if (*argv).is_null() {
        break;
      }
    }
  }
  return 0i32;
}
