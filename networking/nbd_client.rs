use c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use libc;
extern "C" {
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
  #[no_mangle]
  fn fork() -> __pid_t;
  #[no_mangle]
  static mut optarg: *mut libc::c_char;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn daemon(__nochdir: libc::c_int, __noclose: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
  #[no_mangle]
  fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
  #[no_mangle]
  fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn setsockopt_1(fd: libc::c_int, level: libc::c_int, optname: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn create_and_connect_stream_or_die(peer: *const libc::c_char, port: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xread(fd: libc::c_int, buf: *mut libc::c_void, count: size_t);
  #[no_mangle]
  fn xwrite(fd: libc::c_int, buf: *const libc::c_void, count: size_t);
  #[no_mangle]
  fn xatou(str: *const libc::c_char) -> libc::c_uint;
  #[no_mangle]
  fn xatou16(numstr: *const libc::c_char) -> uint16_t;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  static const_int_0: libc::c_int;
  #[no_mangle]
  fn getopt_long_only(
    ___argc: libc::c_int,
    ___argv: *const *mut libc::c_char,
    __shortopts: *const libc::c_char,
    __longopts: *const option,
    __longind: *mut libc::c_int,
  ) -> libc::c_int;
}
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __pid_t = libc::c_int;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type size_t = libc::c_ulong;
pub type C2RustUnnamed = libc::c_uint;
pub const IPPROTO_MAX: C2RustUnnamed = 256;
pub const IPPROTO_RAW: C2RustUnnamed = 255;
pub const IPPROTO_MPLS: C2RustUnnamed = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed = 136;
pub const IPPROTO_SCTP: C2RustUnnamed = 132;
pub const IPPROTO_COMP: C2RustUnnamed = 108;
pub const IPPROTO_PIM: C2RustUnnamed = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed = 94;
pub const IPPROTO_MTP: C2RustUnnamed = 92;
pub const IPPROTO_AH: C2RustUnnamed = 51;
pub const IPPROTO_ESP: C2RustUnnamed = 50;
pub const IPPROTO_GRE: C2RustUnnamed = 47;
pub const IPPROTO_RSVP: C2RustUnnamed = 46;
pub const IPPROTO_IPV6: C2RustUnnamed = 41;
pub const IPPROTO_DCCP: C2RustUnnamed = 33;
pub const IPPROTO_TP: C2RustUnnamed = 29;
pub const IPPROTO_IDP: C2RustUnnamed = 22;
pub const IPPROTO_UDP: C2RustUnnamed = 17;
pub const IPPROTO_PUP: C2RustUnnamed = 12;
pub const IPPROTO_EGP: C2RustUnnamed = 8;
pub const IPPROTO_TCP: C2RustUnnamed = 6;
pub const IPPROTO_IPIP: C2RustUnnamed = 4;
pub const IPPROTO_IGMP: C2RustUnnamed = 2;
pub const IPPROTO_ICMP: C2RustUnnamed = 1;
pub const IPPROTO_IP: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
  pub name: *const libc::c_char,
  pub has_arg: libc::c_int,
  pub flag: *mut libc::c_int,
  pub val: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct new_nbd_header_t {
  pub devsize: uint64_t,
  pub transmission_flags: uint16_t,
  pub data: [libc::c_char; 124],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nbd_opt_t {
  pub magic: uint64_t,
  pub opt: uint32_t,
  pub len: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct old_nbd_header_t {
  pub devsize: uint64_t,
  pub flags: uint32_t,
  pub data: [libc::c_char; 124],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nbd_header_t {
  pub magic1: uint64_t,
  pub magic2: uint64_t,
  // old style: 0x420281861253 big endian
  //               // new style: 0x49484156454F5054 (IHAVEOPT)
}
//usage:#define nbdclient_trivial_usage
//usage:       "{ [-b BLKSIZE] [-N NAME] [-t SEC] [-p] HOST [PORT] | -d } BLOCKDEV"
//usage:#define nbdclient_full_usage "\n\n"
//usage:       "Connect to HOST and provide network block device on BLOCKDEV"
//TODO: more compat with nbd-client version 3.17 -
//nbd-client host [ port ] nbd-device [ -connections num ] [ -sdp ] [ -swap ]
//	[ -persist ] [ -nofork ] [ -nonetlink ] [ -systemd-mark ]
//	[ -block-size block size ] [ -timeout seconds ] [ -name name ]
//	[ -certfile certfile ] [ -keyfile keyfile ] [ -cacertfile cacertfile ]
//	[ -tlshostname hostname ]
//nbd-client -unix path nbd-device [ -connections num ] [ -sdp ] [ -swap ]
//	[ -persist ] [ -nofork ] [ -nonetlink ] [ -systemd-mark ]
//	[ -block-size block size ] [ -timeout seconds ] [ -name name ]
//nbd-client nbd-device
//nbd-client -d nbd-device
//nbd-client -c nbd-device
//nbd-client -l host [ port ]
//nbd-client [ -netlink ] -l host
//
//Default value for blocksize is 4096
//Allowed values for blocksize are 512,1024,2048,4096
#[no_mangle]
pub unsafe extern "C" fn nbdclient_main(
  mut argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut nofork: bool = false;
  let mut opt_d: bool = false;
  let mut opt_p: bool = false;
  let mut host: *const libc::c_char = 0 as *const libc::c_char;
  let mut port: *const libc::c_char = 0 as *const libc::c_char;
  let mut device: *const libc::c_char = 0 as *const libc::c_char;
  let mut name: *const libc::c_char = 0 as *const libc::c_char;
  let mut blksize: libc::c_uint = 0;
  let mut size_blocks: libc::c_uint = 0;
  let mut timeout: libc::c_uint = 0;
  let mut ch: libc::c_int = 0;
  let mut nbd_header: nbd_header_t = nbd_header_t {
    magic1: 0,
    magic2: 0,
  };
  let mut old_nbd_header: old_nbd_header_t = old_nbd_header_t {
    devsize: 0,
    flags: 0,
    data: [0; 124],
  };
  let mut new_nbd_header: new_nbd_header_t = new_nbd_header_t {
    devsize: 0,
    transmission_flags: 0,
    data: [0; 124],
  };
  let mut nbd_opts: nbd_opt_t = nbd_opt_t {
    magic: 0,
    opt: 0,
    len: 0,
  };
  static mut long_options: [option; 5] = [
    {
      let mut init = option {
        name: b"block-size\x00" as *const u8 as *const libc::c_char,
        has_arg: 1i32,
        flag: 0 as *const libc::c_int as *mut libc::c_int,
        val: 'b' as i32,
      };
      init
    },
    {
      let mut init = option {
        name: b"timeout\x00" as *const u8 as *const libc::c_char,
        has_arg: 1i32,
        flag: 0 as *const libc::c_int as *mut libc::c_int,
        val: 't' as i32,
      };
      init
    },
    {
      let mut init = option {
        name: b"name\x00" as *const u8 as *const libc::c_char,
        has_arg: 1i32,
        flag: 0 as *const libc::c_int as *mut libc::c_int,
        val: 'n' as i32,
      };
      init
    },
    {
      let mut init = option {
        name: b"persist\x00" as *const u8 as *const libc::c_char,
        has_arg: 0i32,
        flag: 0 as *const libc::c_int as *mut libc::c_int,
        val: 'p' as i32,
      };
      init
    },
    {
      let mut init = option {
        name: 0 as *const libc::c_char,
        has_arg: 0,
        flag: 0 as *const libc::c_int as *mut libc::c_int,
        val: 0,
      };
      init
    },
  ];
  // Parse args. nbd-client uses stupid "one-dash long options" style :(
  // Even though short forms (-b,-t,-N,-p) exist for all long opts,
  // older manpages only contained long forms, which probably resulted
  // in many scripts using them.
  blksize = 4096i32 as libc::c_uint; // use of "" instead of NULL simplifies strlen() later
  timeout = 0i32 as libc::c_uint;
  name = b"\x00" as *const u8 as *const libc::c_char;
  opt_p = 0i32 != 0;
  opt_d = opt_p;
  loop {
    ch = getopt_long_only(
      argc,
      argv,
      b"dN:\x00" as *const u8 as *const libc::c_char,
      long_options.as_ptr(),
      0 as *mut libc::c_int,
    );
    if !(ch != -1i32) {
      break;
    }
    match ch {
      112 => {
        // -persist
        opt_p = 1i32 != 0
      }
      100 => {
        // -d
        opt_d = 1i32 != 0
      }
      98 => {
        // -block-size
        blksize = xatou(optarg)
      }
      116 => {
        // -timeout
        timeout = xatou(optarg)
      }
      78 | 110 => {
        // -N
        // -name
        name = optarg
      }
      _ => {
        bb_show_usage();
      }
    }
  }
  argv = argv.offset(optind as isize);
  if opt_d {
    // -d
    if !(*argv.offset(0)).is_null() && (*argv.offset(1)).is_null() {
      let mut nbd: libc::c_int = xopen(*argv.offset(0), 0o2i32);
      ioctl(
        nbd,
        (0u32 << 0i32 + 8i32 + 8i32 + 14i32
          | (0xabi32 << 0i32 + 8i32) as libc::c_uint
          | (8i32 << 0i32) as libc::c_uint
          | (0i32 << 0i32 + 8i32 + 8i32) as libc::c_uint) as libc::c_ulong,
      );
      ioctl(
        nbd,
        (0u32 << 0i32 + 8i32 + 8i32 + 14i32
          | (0xabi32 << 0i32 + 8i32) as libc::c_uint
          | (4i32 << 0i32) as libc::c_uint
          | (0i32 << 0i32 + 8i32 + 8i32) as libc::c_uint) as libc::c_ulong,
      );
      return 0i32;
    }
    bb_show_usage();
  }
  // Allow only argv[] of: HOST [PORT] BLOCKDEV
  if (*argv.offset(0)).is_null()
    || (*argv.offset(1)).is_null()
    || !(*argv.offset(2)).is_null() && !(*argv.offset(3)).is_null()
  {
    bb_show_usage();
  }
  host = *argv.offset(0);
  port = if !(*argv.offset(2)).is_null() {
    *argv.offset(1)
  } else {
    b"10809\x00" as *const u8 as *const libc::c_char
  };
  device = if !(*argv.offset(2)).is_null() {
    *argv.offset(2)
  } else {
    *argv.offset(1)
  };
  // Repeat until spanked if -persist
  nofork = 0i32 != 0; // 0 for old, 1 for new
  loop {
    let mut sock: libc::c_int = 0;
    let mut nbd_0: libc::c_int = 0;
    let mut ro: libc::c_int = 0;
    let mut proto_new: libc::c_int = 0;
    let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
    // Make sure BLOCKDEV exists
    nbd_0 = xopen(device, 0o2i32);
    // Find and connect to server
    sock = create_and_connect_stream_or_die(host, xatou16(port) as libc::c_int);
    setsockopt_1(sock, IPPROTO_TCP as libc::c_int, 1i32);
    // Log on to the server
    xread(
      sock,
      &mut nbd_header as *mut nbd_header_t as *mut libc::c_void,
      (8i32 + 8i32) as size_t,
    ); // client_flags
    if memcmp(
      &mut nbd_header.magic1 as *mut uint64_t as *const libc::c_void,
      b"NBDMAGIC\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
      ::std::mem::size_of::<uint64_t>() as libc::c_ulong,
    ) != 0i32
    {
      bb_simple_error_msg_and_die(b"login failed\x00" as *const u8 as *const libc::c_char);
      // NBD_OPT_EXPORT_NAME
    }
    if memcmp(
      &mut nbd_header.magic2 as *mut uint64_t as *const libc::c_void,
      b"\x00\x00B\x02\x81\x86\x12S\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
      ::std::mem::size_of::<uint64_t>() as libc::c_ulong,
    ) == 0i32
    {
      proto_new = 0i32
    } else if memcmp(
      &mut nbd_header.magic2 as *mut uint64_t as *const libc::c_void,
      b"IHAVEOPT\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
      8i32 as libc::c_ulong,
    ) == 0i32
    {
      proto_new = 1i32
    } else {
      bb_simple_error_msg_and_die(b"login failed\x00" as *const u8 as *const libc::c_char);
    }
    if proto_new == 0 {
      xread(
        sock,
        &mut old_nbd_header as *mut old_nbd_header_t as *mut libc::c_void,
        (::std::mem::size_of::<uint64_t>() as libc::c_ulong)
          .wrapping_add(::std::mem::size_of::<uint32_t>() as libc::c_ulong)
          .wrapping_add(::std::mem::size_of::<[libc::c_char; 124]>() as libc::c_ulong),
      );
      size_blocks = ({
        let mut __v: __uint64_t = 0;
        let mut __x: __uint64_t = old_nbd_header.devsize;
        if 0 != 0 {
          __v = ((__x as libc::c_ulonglong & 0xff00000000000000u64) >> 56i32
            | (__x as libc::c_ulonglong & 0xff000000000000u64) >> 40i32
            | (__x as libc::c_ulonglong & 0xff0000000000u64) >> 24i32
            | (__x as libc::c_ulonglong & 0xff00000000u64) >> 8i32
            | (__x as libc::c_ulonglong & 0xff000000u64) << 8i32
            | (__x as libc::c_ulonglong & 0xff0000u64) << 24i32
            | (__x as libc::c_ulonglong & 0xff00u64) << 40i32
            | (__x as libc::c_ulonglong & 0xffu64) << 56i32) as __uint64_t
        } else {
          let fresh0 = &mut __v;
          let fresh1;
          let fresh2 = __x;
          asm!("bswap ${0:q}" : "=r" (fresh1) : "0"
                              (c2rust_asm_casts::AsmCast::cast_in(fresh0, fresh2))
                              :);
          c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
        }
        __v
      })
      .wrapping_div(blksize as libc::c_ulong) as libc::c_uint;
      ioctl(
        nbd_0,
        (0u32 << 0i32 + 8i32 + 8i32 + 14i32
          | (0xabi32 << 0i32 + 8i32) as libc::c_uint
          | (1i32 << 0i32) as libc::c_uint
          | (0i32 << 0i32 + 8i32 + 8i32) as libc::c_uint) as libc::c_ulong,
        blksize as libc::c_ulong,
      );
      ioctl(
        nbd_0,
        (0u32 << 0i32 + 8i32 + 8i32 + 14i32
          | (0xabi32 << 0i32 + 8i32) as libc::c_uint
          | (7i32 << 0i32) as libc::c_uint
          | (0i32 << 0i32 + 8i32 + 8i32) as libc::c_uint) as libc::c_ulong,
        size_blocks,
      );
      ioctl(
        nbd_0,
        (0u32 << 0i32 + 8i32 + 8i32 + 14i32
          | (0xabi32 << 0i32 + 8i32) as libc::c_uint
          | (4i32 << 0i32) as libc::c_uint
          | (0i32 << 0i32 + 8i32 + 8i32) as libc::c_uint) as libc::c_ulong,
      );
      ro = (old_nbd_header.flags
        & ({
          let mut __v: libc::c_ushort = 0;
          let mut __x: libc::c_ushort = 2i32 as libc::c_ushort;
          if 0 != 0 {
            __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
              as libc::c_ushort
          } else {
            let fresh3 = &mut __v;
            let fresh4;
            let fresh5 = __x;
            asm!("rorw $$8, ${0:w}" : "=r" (fresh4) : "0"
                                   (c2rust_asm_casts::AsmCast::cast_in(fresh3, fresh5))
                                   : "cc");
            c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh5, fresh4);
          }
          __v
        }) as libc::c_uint
        != 0) as libc::c_int;
      data = old_nbd_header.data.as_mut_ptr()
    } else {
      let mut namelen: libc::c_uint = 0;
      let mut handshake_flags: uint16_t = 0;
      xread(
        sock,
        &mut handshake_flags as *mut uint16_t as *mut libc::c_void,
        ::std::mem::size_of::<uint16_t>() as libc::c_ulong,
      );
      xwrite(
        sock,
        &const_int_0 as *const libc::c_int as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
      );
      memcpy(
        &mut nbd_opts.magic as *mut uint64_t as *mut libc::c_void,
        b"IHAVEOPT\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        ::std::mem::size_of::<uint64_t>() as libc::c_ulong,
      );
      nbd_opts.opt = {
        let mut __v: libc::c_uint = 0;
        let mut __x: libc::c_uint = 1i32 as libc::c_uint;
        if 0 != 0 {
          __v = (__x & 0xff000000u32) >> 24i32
            | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
            | (__x & 0xff00i32 as libc::c_uint) << 8i32
            | (__x & 0xffi32 as libc::c_uint) << 24i32
        } else {
          let fresh6 = &mut __v;
          let fresh7;
          let fresh8 = __x;
          asm!("bswap $0" : "=r" (fresh7) : "0"
                              (c2rust_asm_casts::AsmCast::cast_in(fresh6, fresh8))
                              :);
          c2rust_asm_casts::AsmCast::cast_out(fresh6, fresh8, fresh7);
        }
        __v
      };
      namelen = strlen(name) as libc::c_uint;
      nbd_opts.len = {
        let mut __v: libc::c_uint = 0;
        let mut __x: libc::c_uint = namelen;
        if 0 != 0 {
          __v = (__x & 0xff000000u32) >> 24i32
            | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
            | (__x & 0xff00i32 as libc::c_uint) << 8i32
            | (__x & 0xffi32 as libc::c_uint) << 24i32
        } else {
          let fresh9 = &mut __v;
          let fresh10;
          let fresh11 = __x;
          asm!("bswap $0" : "=r" (fresh10) : "0"
                              (c2rust_asm_casts::AsmCast::cast_in(fresh9, fresh11))
                              :);
          c2rust_asm_casts::AsmCast::cast_out(fresh9, fresh11, fresh10);
        }
        __v
      };
      xwrite(
        sock,
        &mut nbd_opts as *mut nbd_opt_t as *const libc::c_void,
        (::std::mem::size_of::<uint64_t>() as libc::c_ulong)
          .wrapping_add(::std::mem::size_of::<uint32_t>() as libc::c_ulong)
          .wrapping_add(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
      );
      xwrite(sock, name as *const libc::c_void, namelen as size_t);
      xread(
        sock,
        &mut new_nbd_header as *mut new_nbd_header_t as *mut libc::c_void,
        (::std::mem::size_of::<uint64_t>() as libc::c_ulong)
          .wrapping_add(::std::mem::size_of::<uint16_t>() as libc::c_ulong)
          .wrapping_add(::std::mem::size_of::<[libc::c_char; 124]>() as libc::c_ulong),
      );
      size_blocks = ({
        let mut __v: __uint64_t = 0;
        let mut __x: __uint64_t = new_nbd_header.devsize;
        if 0 != 0 {
          __v = ((__x as libc::c_ulonglong & 0xff00000000000000u64) >> 56i32
            | (__x as libc::c_ulonglong & 0xff000000000000u64) >> 40i32
            | (__x as libc::c_ulonglong & 0xff0000000000u64) >> 24i32
            | (__x as libc::c_ulonglong & 0xff00000000u64) >> 8i32
            | (__x as libc::c_ulonglong & 0xff000000u64) << 8i32
            | (__x as libc::c_ulonglong & 0xff0000u64) << 24i32
            | (__x as libc::c_ulonglong & 0xff00u64) << 40i32
            | (__x as libc::c_ulonglong & 0xffu64) << 56i32) as __uint64_t
        } else {
          let fresh12 = &mut __v;
          let fresh13;
          let fresh14 = __x;
          asm!("bswap ${0:q}" : "=r" (fresh13) : "0"
                              (c2rust_asm_casts::AsmCast::cast_in(fresh12, fresh14))
                              :);
          c2rust_asm_casts::AsmCast::cast_out(fresh12, fresh14, fresh13);
        }
        __v
      })
      .wrapping_div(blksize as libc::c_ulong) as libc::c_uint;
      ioctl(
        nbd_0,
        (0u32 << 0i32 + 8i32 + 8i32 + 14i32
          | (0xabi32 << 0i32 + 8i32) as libc::c_uint
          | (1i32 << 0i32) as libc::c_uint
          | (0i32 << 0i32 + 8i32 + 8i32) as libc::c_uint) as libc::c_ulong,
        blksize as libc::c_ulong,
      );
      ioctl(
        nbd_0,
        (0u32 << 0i32 + 8i32 + 8i32 + 14i32
          | (0xabi32 << 0i32 + 8i32) as libc::c_uint
          | (7i32 << 0i32) as libc::c_uint
          | (0i32 << 0i32 + 8i32 + 8i32) as libc::c_uint) as libc::c_ulong,
        size_blocks,
      );
      ioctl(
        nbd_0,
        (0u32 << 0i32 + 8i32 + 8i32 + 14i32
          | (0xabi32 << 0i32 + 8i32) as libc::c_uint
          | (4i32 << 0i32) as libc::c_uint
          | (0i32 << 0i32 + 8i32 + 8i32) as libc::c_uint) as libc::c_ulong,
      );
      ioctl(
        nbd_0,
        (0u32 << 0i32 + 8i32 + 8i32 + 14i32
          | (0xabi32 << 0i32 + 8i32) as libc::c_uint
          | (10i32 << 0i32) as libc::c_uint
          | (0i32 << 0i32 + 8i32 + 8i32) as libc::c_uint) as libc::c_ulong,
        ({
          let mut __v: libc::c_ushort = 0;
          let mut __x: libc::c_ushort = new_nbd_header.transmission_flags;
          if 0 != 0 {
            __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
              as libc::c_ushort
          } else {
            let fresh15 = &mut __v;
            let fresh16;
            let fresh17 = __x;
            asm!("rorw $$8, ${0:w}" : "=r" (fresh16) : "0"
                                (c2rust_asm_casts::AsmCast::cast_in(fresh15, fresh17))
                                : "cc");
            c2rust_asm_casts::AsmCast::cast_out(fresh15, fresh17, fresh16);
          }
          __v
        }) as libc::c_int,
      );
      ro = (new_nbd_header.transmission_flags as libc::c_int
        & ({
          let mut __v: libc::c_ushort = 0;
          let mut __x: libc::c_ushort = 2i32 as libc::c_ushort;
          if 0 != 0 {
            __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
              as libc::c_ushort
          } else {
            let fresh18 = &mut __v;
            let fresh19;
            let fresh20 = __x;
            asm!("rorw $$8, ${0:w}" : "=r" (fresh19) : "0"
                                   (c2rust_asm_casts::AsmCast::cast_in(fresh18, fresh20))
                                   : "cc");
            c2rust_asm_casts::AsmCast::cast_out(fresh18, fresh20, fresh19);
          }
          __v
        }) as libc::c_int
        != 0) as libc::c_int;
      data = new_nbd_header.data.as_mut_ptr()
    }
    if ioctl(
      nbd_0,
      (0u32 << 0i32 + 8i32 + 8i32 + 14i32
        | (0x12i32 << 0i32 + 8i32) as libc::c_uint
        | (93i32 << 0i32) as libc::c_uint
        | (0i32 << 0i32 + 8i32 + 8i32) as libc::c_uint) as libc::c_ulong,
      &mut ro as *mut libc::c_int,
    ) < 0i32
    {
      bb_simple_perror_msg_and_die(b"BLKROSET\x00" as *const u8 as *const libc::c_char);
    }
    if timeout != 0 {
      if ioctl(
        nbd_0,
        (0u32 << 0i32 + 8i32 + 8i32 + 14i32
          | (0xabi32 << 0i32 + 8i32) as libc::c_uint
          | (9i32 << 0i32) as libc::c_uint
          | (0i32 << 0i32 + 8i32 + 8i32) as libc::c_uint) as libc::c_ulong,
        timeout as libc::c_ulong,
      ) != 0
      {
        bb_simple_perror_msg_and_die(b"NBD_SET_TIMEOUT\x00" as *const u8 as *const libc::c_char);
      }
    }
    if ioctl(
      nbd_0,
      (0u32 << 0i32 + 8i32 + 8i32 + 14i32
        | (0xabi32 << 0i32 + 8i32) as libc::c_uint
        | (0i32 << 0i32) as libc::c_uint
        | (0i32 << 0i32 + 8i32 + 8i32) as libc::c_uint) as libc::c_ulong,
      sock,
    ) != 0
    {
      bb_simple_perror_msg_and_die(b"NBD_SET_SOCK\x00" as *const u8 as *const libc::c_char);
    }
    //if (swap) mlockall(MCL_CURRENT|MCL_FUTURE);
    // Open the device to force reread of the partition table.
    // Need to do it in a separate process, since open(device)
    // needs some other process to sit in ioctl(nbd, NBD_DO_IT).
    if fork() == 0i32 {
      /* child */
      let mut s: *mut libc::c_char = strrchr(device, '/' as i32);
      sprintf(
        data,
        b"/sys/block/%.32s/pid\x00" as *const u8 as *const libc::c_char,
        if !s.is_null() { s.offset(1) } else { device },
      );
      loop
      // Is it up yet?
      {
        let mut fd: libc::c_int = open(data, 0i32);
        if fd >= 0i32 {
          break;
        }
        sleep(1i32 as libc::c_uint);
      }
      open(device, 0i32);
      return 0i32;
    }
    // Daemonize here
    if !nofork {
      daemon(0i32, 0i32);
      nofork = 1i32 != 0
    }
    // This turns us (the process that calls this ioctl)
    // into a dedicated NBD request handler.
    // We block here for a long time.
    // When exactly ioctl returns? On a signal,
    // or if someone does ioctl(NBD_DISCONNECT) [nbd-client -d].
    if ioctl(
      nbd_0,
      (0u32 << 0i32 + 8i32 + 8i32 + 14i32
        | (0xabi32 << 0i32 + 8i32) as libc::c_uint
        | (3i32 << 0i32) as libc::c_uint
        | (0i32 << 0i32 + 8i32 + 8i32) as libc::c_uint) as libc::c_ulong,
    ) >= 0i32
      || *bb_errno == 53i32
    {
      // Flush queue and exit
      ioctl(
        nbd_0,
        (0u32 << 0i32 + 8i32 + 8i32 + 14i32
          | (0xabi32 << 0i32 + 8i32) as libc::c_uint
          | (5i32 << 0i32) as libc::c_uint
          | (0i32 << 0i32 + 8i32 + 8i32) as libc::c_uint) as libc::c_ulong,
      );
      ioctl(
        nbd_0,
        (0u32 << 0i32 + 8i32 + 8i32 + 14i32
          | (0xabi32 << 0i32 + 8i32) as libc::c_uint
          | (4i32 << 0i32) as libc::c_uint
          | (0i32 << 0i32 + 8i32 + 8i32) as libc::c_uint) as libc::c_ulong,
      );
      break;
    } else {
      close(sock);
      close(nbd_0);
      if !opt_p {
        break;
      }
    }
  }
  return 0i32;
}
