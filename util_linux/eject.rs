use libc;
use libc::ioctl;
extern "C" {

  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sg_io_hdr {
  pub interface_id: libc::c_int,
  pub dxfer_direction: libc::c_int,
  pub cmd_len: libc::c_uchar,
  pub mx_sb_len: libc::c_uchar,
  pub iovec_count: libc::c_ushort,
  pub dxfer_len: libc::c_uint,
  pub dxferp: *mut libc::c_void,
  pub cmdp: *mut libc::c_uchar,
  pub sbp: *mut libc::c_uchar,
  pub timeout: libc::c_uint,
  pub flags: libc::c_uint,
  pub pack_id: libc::c_int,
  pub usr_ptr: *mut libc::c_void,
  pub status: libc::c_uchar,
  pub masked_status: libc::c_uchar,
  pub msg_status: libc::c_uchar,
  pub sb_len_wr: libc::c_uchar,
  pub host_status: libc::c_ushort,
  pub driver_status: libc::c_ushort,
  pub resid: libc::c_int,
  pub duration: libc::c_uint,
  pub info: libc::c_uint,
}
pub type sg_io_hdr_t = sg_io_hdr;
/* Code taken from the original eject (http://eject.sourceforge.net/),
 * refactored it a bit for busybox (ne-bb@nicoerfurth.de) */
unsafe extern "C" fn eject_scsi(mut dev: *const libc::c_char) {
  static mut sg_commands: [[libc::c_char; 6]; 3] = [
    [
      0x1ei32 as libc::c_char,
      0 as libc::c_char,
      0 as libc::c_char,
      0 as libc::c_char,
      0 as libc::c_char,
      0 as libc::c_char,
    ],
    [
      0x1bi32 as libc::c_char,
      0 as libc::c_char,
      0 as libc::c_char,
      0 as libc::c_char,
      1i32 as libc::c_char,
      0 as libc::c_char,
    ],
    [
      0x1bi32 as libc::c_char,
      0 as libc::c_char,
      0 as libc::c_char,
      0 as libc::c_char,
      2i32 as libc::c_char,
      0 as libc::c_char,
    ],
  ];
  let mut i: libc::c_uint = 0;
  let mut sense_buffer: [libc::c_uchar; 32] = [0; 32];
  let mut inqBuff: [libc::c_uchar; 2] = [0; 2];
  let mut io_hdr: sg_io_hdr_t = std::mem::zeroed();
  if ioctl(
    3i32,
    0x2282i32 as libc::c_ulong,
    &mut i as *mut libc::c_uint,
  ) < 0
    || i < 30000i32 as libc::c_uint
  {
    crate::libbb::verror_msg::bb_simple_error_msg_and_die(
      b"not a sg device or old sg driver\x00" as *const u8 as *const libc::c_char,
    );
  }
  memset(
    &mut io_hdr as *mut sg_io_hdr_t as *mut libc::c_void,
    0,
    ::std::mem::size_of::<sg_io_hdr_t>() as libc::c_ulong,
  );
  io_hdr.interface_id = 'S' as i32;
  io_hdr.cmd_len = 6i32 as libc::c_uchar;
  io_hdr.mx_sb_len = ::std::mem::size_of::<[libc::c_uchar; 32]>() as libc::c_ulong as libc::c_uchar;
  io_hdr.dxfer_direction = -1i32;
  /* io_hdr.dxfer_len = 0; */
  io_hdr.dxferp = inqBuff.as_mut_ptr() as *mut libc::c_void;
  io_hdr.sbp = sense_buffer.as_mut_ptr();
  io_hdr.timeout = 2000i32 as libc::c_uint;
  i = 0 as libc::c_uint;
  while i < 3i32 as libc::c_uint {
    io_hdr.cmdp = sg_commands[i as usize].as_ptr() as *mut libc::c_void as *mut libc::c_uchar;
    crate::libbb::xfuncs_printf::ioctl_or_perror_and_die(
      3i32,
      0x2285i32 as libc::c_uint,
      &mut io_hdr as *mut sg_io_hdr_t as *mut libc::c_void,
      b"%s\x00" as *const u8 as *const libc::c_char,
      dev,
    );
    i = i.wrapping_add(1)
  }
  /* force kernel to reread partition table when new disc is inserted */
  ioctl(
    3i32,
    (0u32 << 0 + 8i32 + 8i32 + 14i32
      | (0x12i32 << 0 + 8i32) as libc::c_uint
      | (95i32 << 0) as libc::c_uint
      | (0i32 << 0 + 8i32 + 8i32) as libc::c_uint) as libc::c_ulong,
  );
}
unsafe extern "C" fn eject_cdrom(mut flags: libc::c_uint, mut dev: *const libc::c_char) {
  let mut cmd: libc::c_int = 0x5309i32;
  if flags & 1i32 as libc::c_uint != 0
    || flags & 2i32 as libc::c_uint != 0 && ioctl(3i32, 0x5326i32 as libc::c_ulong) == 2i32
  {
    cmd = 0x5319i32
  }
  crate::libbb::xfuncs_printf::ioctl_or_perror_and_die(
    3i32,
    cmd as libc::c_uint,
    0 as *mut libc::c_void,
    b"%s\x00" as *const u8 as *const libc::c_char,
    dev,
  );
}
#[no_mangle]
pub unsafe extern "C" fn eject_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut flags: libc::c_uint = 0;
  let mut device: *const libc::c_char = std::ptr::null();
  flags = crate::libbb::getopt32::getopt32(
    argv,
    b"^tTs\x00?1:t--T:T--t\x00" as *const u8 as *const libc::c_char,
  );
  device = if !(*argv.offset(optind as isize)).is_null() {
    *argv.offset(optind as isize)
  } else {
    b"/dev/cdrom\x00" as *const u8 as *const libc::c_char
  };
  /* We used to do "umount <device>" here, but it was buggy
     if something was mounted OVER cdrom and
     if cdrom is mounted many times.

     This works equally well (or better):
     #!/bin/sh
     umount /dev/cdrom
     eject /dev/cdrom
  */
  crate::libbb::xfuncs_printf::xmove_fd(
    crate::libbb::xfuncs_printf::xopen_nonblocking(device),
    3i32,
  );
  if 1i32 != 0 && flags & 4i32 as libc::c_uint != 0 {
    eject_scsi(device);
  } else {
    eject_cdrom(flags, device);
  }
  return 0;
}
