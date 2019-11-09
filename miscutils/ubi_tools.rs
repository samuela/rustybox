use libc;



extern "C" {

  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;

  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn strnlen(__string: *const libc::c_char, __maxlen: size_t) -> size_t;
  #[no_mangle]
  fn xmalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xmove_fd(_: libc::c_int, _: libc::c_int);
  #[no_mangle]
  fn xfstat(fd: libc::c_int, buf: *mut stat, errmsg: *const libc::c_char);
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn full_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn open_read_close(
    filename: *const libc::c_char,
    buf: *mut libc::c_void,
    maxsz: size_t,
  ) -> ssize_t;
  #[no_mangle]
  fn xwrite(fd: libc::c_int, buf: *const libc::c_void, count: size_t);
  #[no_mangle]
  fn xatoull_sfx(str: *const libc::c_char, sfx: *const suffix_mult) -> libc::c_ulonglong;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_perror_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_xioctl(
    fd: libc::c_int,
    request: libc::c_uint,
    argp: *mut libc::c_void,
    ioctl_name: *const libc::c_char,
  ) -> libc::c_int;
  #[no_mangle]
  fn ubi_devnum_from_devname(str: *const libc::c_char) -> libc::c_uint;
  #[no_mangle]
  fn ubi_get_volid_by_name(ubi_devnum: libc::c_uint, vol_name: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  static mut applet_name: *const libc::c_char;
}

pub type __int64_t = libc::c_long;


pub type int64_t = __int64_t;
use crate::librb::size_t;
use libc::ssize_t;
use libc::stat;


#[derive(Copy, Clone)]
#[repr(C)]
pub struct suffix_mult {
  pub suffix: [libc::c_char; 4],
  pub mult: libc::c_uint,
}
pub type __s8 = libc::c_schar;
pub type __s16 = libc::c_short;
pub type __s32 = libc::c_int;
pub type __s64 = libc::c_longlong;
pub type C2RustUnnamed = libc::c_uint;
pub const UBI_STATIC_VOLUME: C2RustUnnamed = 4;
pub const UBI_DYNAMIC_VOLUME: C2RustUnnamed = 3;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ubi_attach_req {
  pub ubi_num: __s32,
  pub mtd_num: __s32,
  pub vid_hdr_offset: __s32,
  pub max_beb_per1024: __s16,
  pub padding: [__s8; 10],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ubi_mkvol_req {
  pub vol_id: __s32,
  pub alignment: __s32,
  pub bytes: __s64,
  pub vol_type: __s8,
  pub padding1: __s8,
  pub name_len: __s16,
  pub padding2: [__s8; 4],
  pub name: [libc::c_char; 128],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ubi_rsvol_req {
  pub bytes: __s64,
  pub vol_id: __s32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
  pub attach_req: ubi_attach_req,
  pub mkvol_req: ubi_mkvol_req,
  pub rsvol_req: ubi_rsvol_req,
}
unsafe extern "C" fn get_num_from_file(
  mut path: *const libc::c_char,
  mut max: libc::c_uint,
) -> libc::c_uint {
  let mut buf: [libc::c_char; 24] = [0; 24];
  let mut num: libc::c_ulonglong = 0;
  if open_read_close(
    path,
    buf.as_mut_ptr() as *mut libc::c_void,
    ::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong,
  ) < 0   {
    bb_perror_msg_and_die(
      b"can\'t open \'%s\'\x00" as *const u8 as *const libc::c_char,
      path,
    );
  }
  /* It can be \n terminated, xatoull won't work well */
  if sscanf(
    buf.as_mut_ptr(),
    b"%llu\x00" as *const u8 as *const libc::c_char,
    &mut num as *mut libc::c_ulonglong,
  ) != 1i32
    || num > max as libc::c_ulonglong
  {
    bb_error_msg_and_die(
      b"number in \'%s\' is malformed or too large\x00" as *const u8 as *const libc::c_char,
      path,
    ); /* for compiler */
  }
  return num as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn ubi_tools_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  static mut size_suffixes: [suffix_mult; 4] = [
    {
      let mut init = suffix_mult {
        suffix: [75, 105, 66, 0],
        mult: 1024i32 as libc::c_uint,
      };
      init
    },
    {
      let mut init = suffix_mult {
        suffix: [77, 105, 66, 0],
        mult: (1024i32 * 1024i32) as libc::c_uint,
      };
      init
    },
    {
      let mut init = suffix_mult {
        suffix: [71, 105, 66, 0],
        mult: (1024i32 * 1024i32 * 1024i32) as libc::c_uint,
      };
      init
    },
    {
      let mut init = suffix_mult {
        suffix: [0, 0, 0, 0],
        mult: 0i32 as libc::c_uint,
      };
      init
    },
  ];
  let mut opts: libc::c_uint = 0;
  let mut ubi_ctrl: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut fd: libc::c_int = 0;
  let mut mtd_num: libc::c_int = 0;
  let mut dev_num: libc::c_int = -1i32;
  let mut vol_id: libc::c_int = -1i32;
  let mut vid_hdr_offset: libc::c_int = 0i32;
  let mut vol_name: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut size_bytes: libc::c_ulonglong = 0;
  size_bytes = size_bytes;
  let mut size_bytes_str: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut alignment: libc::c_int = 1i32;
  let mut type_0: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut req_structs: C2RustUnnamed_0 = C2RustUnnamed_0 {
    attach_req: ubi_attach_req {
      ubi_num: 0,
      mtd_num: 0,
      vid_hdr_offset: 0,
      max_beb_per1024: 0,
      padding: [0; 10],
    },
  };
  let mut path: [libc::c_char; 79] = [0; 79];
  strcpy(
    path.as_mut_ptr(),
    b"/sys/class/ubi/ubi\x00" as *const u8 as *const libc::c_char,
  );
  memset(
    &mut req_structs as *mut C2RustUnnamed_0 as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<C2RustUnnamed_0>() as libc::c_ulong,
  );
  if 1i32 != 0
    && (0i32 + 1i32 + 1i32 + 1i32 + 1i32 + 1i32 + 1i32 == 1i32
      || *applet_name.offset(4) as libc::c_int == 'k' as i32)
  {
    opts = getopt32(
      argv,
      b"^md:+n:+N:s:a:+t:O:+\x00-1\x00" as *const u8 as *const libc::c_char,
      &mut dev_num as *mut libc::c_int,
      &mut vol_id as *mut libc::c_int,
      &mut vol_name as *mut *mut libc::c_char,
      &mut size_bytes_str as *mut *mut libc::c_char,
      &mut alignment as *mut libc::c_int,
      &mut type_0 as *mut *mut libc::c_char,
      &mut vid_hdr_offset as *mut libc::c_int,
    )
  } else if 1i32 != 0
    && (0i32 + 1i32 + 1i32 + 1i32 + 1i32 + 1i32 + 1i32 == 1i32
      || *applet_name.offset(4) as libc::c_int == 'p' as i32)
  {
    opts = getopt32(
      argv,
      b"^s:at\x00-1\x00" as *const u8 as *const libc::c_char,
      &mut size_bytes_str as *mut *mut libc::c_char,
    );
    opts = opts.wrapping_mul((1i32 << 4i32) as libc::c_uint)
  } else {
    opts = getopt32(
      argv,
      b"^m:+d:+n:+N:s:a:+t:\x00-1\x00" as *const u8 as *const libc::c_char,
      &mut mtd_num as *mut libc::c_int,
      &mut dev_num as *mut libc::c_int,
      &mut vol_id as *mut libc::c_int,
      &mut vol_name as *mut *mut libc::c_char,
      &mut size_bytes_str as *mut *mut libc::c_char,
      &mut alignment as *mut libc::c_int,
      &mut type_0 as *mut *mut libc::c_char,
    )
  }
  if opts & (1i32 << 4i32) as libc::c_uint != 0 {
    size_bytes = xatoull_sfx(size_bytes_str, size_suffixes.as_ptr())
  }
  argv = argv.offset(optind as isize);
  let fresh0 = argv;
  argv = argv.offset(1);
  ubi_ctrl = *fresh0;
  fd = xopen(ubi_ctrl, 0o2i32);
  /*just in case:*/
  //xfstat(fd, &st, ubi_ctrl);
  //if (!S_ISCHR(st.st_mode))
  //	bb_error_msg_and_die("%s: not a char device", ubi_ctrl);
  //usage:#define ubiattach_trivial_usage
  //usage:       "-m MTD_NUM [-d UBI_NUM] [-O VID_HDR_OFF] UBI_CTRL_DEV"
  //usage:#define ubiattach_full_usage "\n\n"
  //usage:       "Attach MTD device to UBI\n"
  //usage:     "\n	-m MTD_NUM	MTD device number to attach"
  //usage:     "\n	-d UBI_NUM	UBI device number to assign"
  //usage:     "\n	-O VID_HDR_OFF	VID header offset"
  if 1i32 != 0
    && (0i32 + 1i32 + 1i32 + 1i32 + 1i32 + 1i32 + 1i32 == 1i32
      || *applet_name.offset(4) as libc::c_int == 't' as i32)
  {
    if opts & (1i32 << 0i32) as libc::c_uint == 0 {
      bb_error_msg_and_die(
        b"%s device not specified\x00" as *const u8 as *const libc::c_char,
        b"MTD\x00" as *const u8 as *const libc::c_char,
      );
    }
    req_structs.attach_req.mtd_num = mtd_num;
    req_structs.attach_req.ubi_num = dev_num;
    req_structs.attach_req.vid_hdr_offset = vid_hdr_offset;
    bb_xioctl(
      fd,
      ((1u32 << 0i32 + 8i32 + 8i32 + 14i32
        | (('o' as i32) << 0i32 + 8i32) as libc::c_uint
        | (64i32 << 0i32) as libc::c_uint) as libc::c_ulong
        | (::std::mem::size_of::<ubi_attach_req>() as libc::c_ulong) << 0i32 + 8i32 + 8i32)
        as libc::c_uint,
      &mut req_structs.attach_req as *mut ubi_attach_req as *mut libc::c_void,
      b"UBI_IOCATT\x00" as *const u8 as *const libc::c_char,
    );
  } else if 1i32 != 0
    && (0i32 + 1i32 + 1i32 + 1i32 + 1i32 + 1i32 + 1i32 == 1i32
      || *applet_name.offset(4) as libc::c_int == 'e' as i32)
  {
    if opts & (1i32 << 1i32) as libc::c_uint == 0 {
      bb_error_msg_and_die(
        b"%s device not specified\x00" as *const u8 as *const libc::c_char,
        b"UBI\x00" as *const u8 as *const libc::c_char,
      );
    }
    //usage:#define ubidetach_trivial_usage
    //usage:       "-d UBI_NUM UBI_CTRL_DEV"
    //usage:#define ubidetach_full_usage "\n\n"
    //usage:       "Detach MTD device from UBI\n"
    //usage:     "\n	-d UBI_NUM	UBI device number"
    /* FIXME? kernel expects i32* here: */
    bb_xioctl(
      fd,
      ((1u32 << 0i32 + 8i32 + 8i32 + 14i32
        | (('o' as i32) << 0i32 + 8i32) as libc::c_uint
        | (65i32 << 0i32) as libc::c_uint) as libc::c_ulong
        | (::std::mem::size_of::<__s32>() as libc::c_ulong) << 0i32 + 8i32 + 8i32)
        as libc::c_uint,
      &mut dev_num as *mut libc::c_int as *mut libc::c_void,
      b"UBI_IOCDET\x00" as *const u8 as *const libc::c_char,
    );
  } else if 1i32 != 0
    && (0i32 + 1i32 + 1i32 + 1i32 + 1i32 + 1i32 + 1i32 == 1i32
      || *applet_name.offset(4) as libc::c_int == 'k' as i32)
  {
    if opts & (1i32 << 0i32) as libc::c_uint != 0 {
      let mut leb_avail: libc::c_uint = 0;
      let mut leb_size: libc::c_uint = 0;
      let mut num: libc::c_uint = 0;
      let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
      num = ubi_devnum_from_devname(ubi_ctrl);
      p = path
        .as_mut_ptr()
        .offset(::std::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong as isize)
        .offset(-1)
        .offset(sprintf(
          path
            .as_mut_ptr()
            .offset(::std::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong as isize)
            .offset(-1),
          b"%u/\x00" as *const u8 as *const libc::c_char,
          num,
        ) as isize);
      strcpy(
        p,
        b"avail_eraseblocks\x00" as *const u8 as *const libc::c_char,
      );
      leb_avail = get_num_from_file(
        path.as_mut_ptr(),
        (2147483647i32 as libc::c_uint)
          .wrapping_mul(2u32)
          .wrapping_add(1u32),
      );
      strcpy(
        p,
        b"eraseblock_size\x00" as *const u8 as *const libc::c_char,
      );
      leb_size = get_num_from_file(
        path.as_mut_ptr(),
        (16i32 * 1024i32 * 1024i32) as libc::c_uint,
      );
      size_bytes = (leb_avail as libc::c_ulonglong).wrapping_mul(leb_size as libc::c_ulonglong)
    //usage:#define ubimkvol_trivial_usage
    //usage:       "-N NAME [-s SIZE | -m] UBI_DEVICE"
    //usage:#define ubimkvol_full_usage "\n\n"
    //usage:       "Create UBI volume\n"
    //usage:     "\n	-a ALIGNMENT	Volume alignment (default 1)"
    //usage:     "\n	-m		Set volume size to maximum available"
    //usage:     "\n	-n VOLID	Volume ID. If not specified,"
    //usage:     "\n			assigned automatically"
    //usage:     "\n	-N NAME		Volume name"
    //usage:     "\n	-s SIZE		Size in bytes"
    //usage:     "\n	-t TYPE		Volume type (static|dynamic)"
    //if (size_bytes <= 0)
    //	bb_error_msg_and_die("%s invalid maximum size calculated", "UBI");
    } else if opts & (1i32 << 4i32) as libc::c_uint == 0 {
      bb_simple_error_msg_and_die(b"size not specified\x00" as *const u8 as *const libc::c_char);
    }
    if opts & (1i32 << 3i32) as libc::c_uint == 0 {
      bb_simple_error_msg_and_die(b"name not specified\x00" as *const u8 as *const libc::c_char);
    }
    /* the structure is memset(0) above */
    req_structs.mkvol_req.vol_id = vol_id; /* signed int64_t */
    req_structs.mkvol_req.vol_type = UBI_DYNAMIC_VOLUME as libc::c_int as __s8;
    if opts & (1i32 << 6i32) as libc::c_uint != 0 && *type_0.offset(0) as libc::c_int == 's' as i32
    {
      req_structs.mkvol_req.vol_type = UBI_STATIC_VOLUME as libc::c_int as __s8
    }
    req_structs.mkvol_req.alignment = alignment;
    req_structs.mkvol_req.bytes = size_bytes as __s64;
    /* strnlen avoids overflow of 16-bit field (paranoia) */
    req_structs.mkvol_req.name_len = strnlen(vol_name, (127i32 + 1i32) as size_t) as __s16;
    if req_structs.mkvol_req.name_len as libc::c_int > 127i32 {
      bb_error_msg_and_die(
        b"volume name too long: \'%s\'\x00" as *const u8 as *const libc::c_char,
        vol_name,
      );
    }
    /* this is safe: .name[] is UBI_MAX_VOLUME_NAME+1 bytes */
    strcpy(req_structs.mkvol_req.name.as_mut_ptr(), vol_name);
    bb_xioctl(
      fd,
      ((1u32 << 0i32 + 8i32 + 8i32 + 14i32
        | (('o' as i32) << 0i32 + 8i32) as libc::c_uint
        | (0i32 << 0i32) as libc::c_uint) as libc::c_ulong
        | (::std::mem::size_of::<ubi_mkvol_req>() as libc::c_ulong) << 0i32 + 8i32 + 8i32)
        as libc::c_uint,
      &mut req_structs.mkvol_req as *mut ubi_mkvol_req as *mut libc::c_void,
      b"UBI_IOCMKVOL\x00" as *const u8 as *const libc::c_char,
    );
  } else if 1i32 != 0
    && (0i32 + 1i32 + 1i32 + 1i32 + 1i32 + 1i32 + 1i32 == 1i32
      || *applet_name.offset(4) as libc::c_int == 'm' as i32)
  {
    if opts & (1i32 << 2i32 | 1i32 << 3i32) as libc::c_uint == 0 {
      bb_simple_error_msg_and_die(
        b"volume id not specified\x00" as *const u8 as *const libc::c_char,
      );
    }
    if opts & (1i32 << 3i32) as libc::c_uint != 0 {
      let mut num_0: libc::c_uint = ubi_devnum_from_devname(ubi_ctrl);
      vol_id = ubi_get_volid_by_name(num_0, vol_name)
    }
    if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong != 4i32 as libc::c_ulong {
      //usage:#define ubirmvol_trivial_usage
      //usage:       "-n VOLID / -N VOLNAME UBI_DEVICE"
      //usage:#define ubirmvol_full_usage "\n\n"
      //usage:       "Remove UBI volume\n"
      //usage:     "\n	-n VOLID	Volume ID"
      //usage:     "\n	-N VOLNAME	Volume name"
      /* kernel expects i32* in this ioctl */
      let mut t: i32 = vol_id;
      bb_xioctl(
        fd,
        ((1u32 << 0i32 + 8i32 + 8i32 + 14i32
          | (('o' as i32) << 0i32 + 8i32) as libc::c_uint
          | (1i32 << 0i32) as libc::c_uint) as libc::c_ulong
          | (::std::mem::size_of::<__s32>() as libc::c_ulong) << 0i32 + 8i32 + 8i32)
          as libc::c_uint,
        &mut t as *mut i32 as *mut libc::c_void,
        b"UBI_IOCRMVOL\x00" as *const u8 as *const libc::c_char,
      );
    } else {
      bb_xioctl(
        fd,
        ((1u32 << 0i32 + 8i32 + 8i32 + 14i32
          | (('o' as i32) << 0i32 + 8i32) as libc::c_uint
          | (1i32 << 0i32) as libc::c_uint) as libc::c_ulong
          | (::std::mem::size_of::<__s32>() as libc::c_ulong) << 0i32 + 8i32 + 8i32)
          as libc::c_uint,
        &mut vol_id as *mut libc::c_int as *mut libc::c_void,
        b"UBI_IOCRMVOL\x00" as *const u8 as *const libc::c_char,
      );
    }
  } else if 1i32 != 0
    && (0i32 + 1i32 + 1i32 + 1i32 + 1i32 + 1i32 + 1i32 == 1i32
      || *applet_name.offset(4) as libc::c_int == 's' as i32)
  {
    if opts & (1i32 << 4i32) as libc::c_uint == 0 {
      bb_simple_error_msg_and_die(b"size not specified\x00" as *const u8 as *const libc::c_char);
    }
    if opts & (1i32 << 2i32) as libc::c_uint == 0 {
      bb_simple_error_msg_and_die(
        b"volume id not specified\x00" as *const u8 as *const libc::c_char,
      );
    }
    //usage:#define ubirsvol_trivial_usage
    //usage:       "-n VOLID -s SIZE UBI_DEVICE"
    //usage:#define ubirsvol_full_usage "\n\n"
    //usage:       "Resize UBI volume\n"
    //usage:     "\n	-n VOLID	Volume ID"
    //usage:     "\n	-s SIZE		Size in bytes"
    req_structs.rsvol_req.bytes = size_bytes as __s64; /* signed int64_t */
    req_structs.rsvol_req.vol_id = vol_id;
    bb_xioctl(
      fd,
      ((1u32 << 0i32 + 8i32 + 8i32 + 14i32
        | (('o' as i32) << 0i32 + 8i32) as libc::c_uint
        | (2i32 << 0i32) as libc::c_uint) as libc::c_ulong
        | (::std::mem::size_of::<ubi_rsvol_req>() as libc::c_ulong) << 0i32 + 8i32 + 8i32)
        as libc::c_uint,
      &mut req_structs.rsvol_req as *mut ubi_rsvol_req as *mut libc::c_void,
      b"UBI_IOCRSVOL\x00" as *const u8 as *const libc::c_char,
    );
  } else if 1i32 != 0
    && (0i32 + 1i32 + 1i32 + 1i32 + 1i32 + 1i32 + 1i32 == 1i32
      || *applet_name.offset(4) as libc::c_int == 'p' as i32)
  {
    let mut bytes64: int64_t = 0;
    if opts & (1i32 << 6i32) as libc::c_uint != 0 {
      //usage:#define ubiupdatevol_trivial_usage
      //usage:       "-t UBI_DEVICE | [-s SIZE] UBI_DEVICE IMG_FILE"
      //usage:#define ubiupdatevol_full_usage "\n\n"
      //usage:       "Update UBI volume\n"
      //usage:     "\n	-t	Truncate to zero size"
      //usage:     "\n	-s SIZE	Size in bytes to resize to"
      /* truncate the volume by starting an update for size 0 */
      bytes64 = 0i32 as int64_t;
      /* this ioctl expects int64_t* parameter */
      bb_xioctl(
        fd,
        ((1u32 << 0i32 + 8i32 + 8i32 + 14i32
          | (('O' as i32) << 0i32 + 8i32) as libc::c_uint
          | (0i32 << 0i32) as libc::c_uint) as libc::c_ulong
          | (::std::mem::size_of::<__s64>() as libc::c_ulong) << 0i32 + 8i32 + 8i32)
          as libc::c_uint,
        &mut bytes64 as *mut int64_t as *mut libc::c_void,
        b"UBI_IOCVOLUP\x00" as *const u8 as *const libc::c_char,
      );
    } else {
      let mut ubinum: libc::c_uint = 0;
      let mut volnum: libc::c_uint = 0;
      let mut leb_size_0: libc::c_uint = 0;
      let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
      /* Assume that device is in normal format. */
      /* Removes need for scanning sysfs tree as full libubi does. */
      if sscanf(
        ubi_ctrl,
        b"/dev/ubi%u_%u\x00" as *const u8 as *const libc::c_char,
        &mut ubinum as *mut libc::c_uint,
        &mut volnum as *mut libc::c_uint,
      ) != 2i32
      {
        bb_error_msg_and_die(
          b"UBI device name \'%s\' is not /dev/ubiN_M\x00" as *const u8 as *const libc::c_char,
          ubi_ctrl,
        );
      }
      sprintf(
        path
          .as_mut_ptr()
          .offset(::std::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong as isize)
          .offset(-1),
        b"%u_%u/usable_eb_size\x00" as *const u8 as *const libc::c_char,
        ubinum,
        volnum,
      );
      leb_size_0 = get_num_from_file(
        path.as_mut_ptr(),
        (16i32 * 1024i32 * 1024i32) as libc::c_uint,
      );
      if (*argv).is_null() {
        bb_show_usage();
      }
      if *(*argv).offset(0) as libc::c_int != '-' as i32 || *(*argv).offset(1) as libc::c_int != 0 {
        /* mtd-utils supports "-" as stdin */
        xmove_fd(xopen(*argv, 0i32), 0i32);
      }
      if opts & (1i32 << 4i32) as libc::c_uint == 0 {
        let mut st: stat = std::mem::zeroed();
        xfstat(0i32, &mut st, *argv);
        size_bytes = st.st_size as libc::c_ulonglong
      }
      bytes64 = size_bytes as int64_t;
      /* this ioctl expects signed int64_t* parameter */
      bb_xioctl(
        fd,
        ((1u32 << 0i32 + 8i32 + 8i32 + 14i32
          | (('O' as i32) << 0i32 + 8i32) as libc::c_uint
          | (0i32 << 0i32) as libc::c_uint) as libc::c_ulong
          | (::std::mem::size_of::<__s64>() as libc::c_ulong) << 0i32 + 8i32 + 8i32)
          as libc::c_uint,
        &mut bytes64 as *mut int64_t as *mut libc::c_void,
        b"UBI_IOCVOLUP\x00" as *const u8 as *const libc::c_char,
      );
      /* can't use bb_copyfd_exact_size(): copy in blocks of exactly leb_size */
      buf = xmalloc(leb_size_0 as size_t) as *mut libc::c_char;
      while size_bytes != 0i32 as libc::c_ulonglong {
        let mut len: libc::c_int =
          full_read(0i32, buf as *mut libc::c_void, leb_size_0 as size_t) as libc::c_int;
        if len <= 0i32 {
          if len < 0i32 {
            bb_perror_msg_and_die(
              b"read error from \'%s\'\x00" as *const u8 as *const libc::c_char,
              *argv,
            );
          }
          break;
        } else {
          if len as libc::c_uint as libc::c_ulonglong > size_bytes {
            /* for this case: "ubiupdatevol -s 1024000 $UBIDEV /dev/urandom" */
            len = size_bytes as libc::c_int
          }
          xwrite(fd, buf as *const libc::c_void, len as size_t);
          size_bytes = size_bytes.wrapping_sub(len as libc::c_ulonglong)
        }
      }
    }
  }
  return 0i32;
}
