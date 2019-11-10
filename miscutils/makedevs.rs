use libc::mode_t;

use crate::librb::size_t;

use libc;
use libc::chdir;
use libc::chmod;
use libc::chown;
use libc::closelog;
use libc::dup2;
use libc::fstat;
use libc::getenv;
use libc::geteuid;
use libc::getopt;
use libc::getpid;
use libc::isatty;
use libc::kill;
use libc::openlog;
use libc::sigaddset;
use libc::sigemptyset;
use libc::sigprocmask;
use libc::sleep;
use libc::sscanf;
use libc::strcasecmp;
use libc::strcpy;
use libc::symlink;
use libc::syscall;
use libc::syslog;
use libc::time;
use libc::access;
use libc::atoi;
use libc::fclose;
use libc::fprintf;
use libc::lstat;
use libc::printf;
use libc::puts;
use libc::rename;
use libc::rmdir;
use libc::sprintf;
use libc::strchr;
use libc::strcmp;
use libc::strrchr;
use libc::strstr;
use libc::system;

use libc::free;
use libc::gid_t;
use libc::stat;
use libc::uid_t;
use libc::FILE;

extern "C" {

  #[no_mangle]
  fn getuid() -> uid_t;
  #[no_mangle]
  fn getgid() -> gid_t;
  #[no_mangle]
  static mut optind: libc::c_int;



  #[no_mangle]
  fn umask(__mask: mode_t) -> mode_t;
  #[no_mangle]
  fn mknod(__path: *const libc::c_char, __mode: mode_t, __dev: libc::dev_t) -> libc::c_int;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn xchdir(path: *const libc::c_char);
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  #[no_mangle]
  fn xuname2uid(name: *const libc::c_char) -> libc::c_long;
  #[no_mangle]
  fn xgroup2gid(name: *const libc::c_char) -> libc::c_long;
  #[no_mangle]
  fn get_ug_id(
    s: *const libc::c_char,
    xname2id: Option<unsafe extern "C" fn(_: *const libc::c_char) -> libc::c_long>,
  ) -> libc::c_ulong;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_perror_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn config_open(filename: *const libc::c_char) -> *mut parser_t;
  #[no_mangle]
  fn config_read(
    parser: *mut parser_t,
    tokens: *mut *mut libc::c_char,
    flags: libc::c_uint,
    delims: *const libc::c_char,
  ) -> libc::c_int;
  #[no_mangle]
  fn config_close(parser: *mut parser_t);
  #[no_mangle]
  fn bb_make_directory(
    path: *mut libc::c_char,
    mode: libc::c_long,
    flags: libc::c_int,
  ) -> libc::c_int;
  #[no_mangle]
  fn bb_makedev(major: libc::c_uint, minor: libc::c_uint) -> libc::c_ulonglong;
}

pub type C2RustUnnamed = libc::c_int;
pub const FILEUTILS_IGNORE_CHMOD_ERR: C2RustUnnamed = -2147483648;
pub const FILEUTILS_REFLINK_ALWAYS: C2RustUnnamed = 262144;
pub const FILEUTILS_REFLINK: C2RustUnnamed = 131072;
pub const FILEUTILS_RMDEST: C2RustUnnamed = 32768;
pub const FILEUTILS_NO_TARGET_DIR: C2RustUnnamed = 16384;
pub const FILEUTILS_UPDATE: C2RustUnnamed = 8192;
pub const FILEUTILS_VERBOSE: C2RustUnnamed = 4096;
pub const FILEUTILS_DEREFERENCE_L0: C2RustUnnamed = 256;
pub const FILEUTILS_DEREF_SOFTLINK: C2RustUnnamed = 128;
pub const FILEUTILS_MAKE_SOFTLINK: C2RustUnnamed = 64;
pub const FILEUTILS_MAKE_HARDLINK: C2RustUnnamed = 32;
pub const FILEUTILS_INTERACTIVE: C2RustUnnamed = 16;
pub const FILEUTILS_FORCE: C2RustUnnamed = 8;
pub const FILEUTILS_RECUR: C2RustUnnamed = 4;
pub const FILEUTILS_DEREFERENCE: C2RustUnnamed = 2;
pub const FILEUTILS_PRESERVE_STATUS: C2RustUnnamed = 1;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const PARSE_NORMAL: C2RustUnnamed_0 = 4653056;
pub const PARSE_WS_COMMENTS: C2RustUnnamed_0 = 16777216;
pub const PARSE_ALT_COMMENTS: C2RustUnnamed_0 = 8388608;
pub const PARSE_EOL_COMMENTS: C2RustUnnamed_0 = 4194304;
pub const PARSE_KEEP_COPY: C2RustUnnamed_0 = 2097152;
pub const PARSE_MIN_DIE: C2RustUnnamed_0 = 1048576;
pub const PARSE_GREEDY: C2RustUnnamed_0 = 262144;
pub const PARSE_TRIM: C2RustUnnamed_0 = 131072;
pub const PARSE_COLLAPSE: C2RustUnnamed_0 = 65536;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct parser_t {
  pub fp: *mut FILE,
  pub data: *mut libc::c_char,
  pub line: *mut libc::c_char,
  pub nline: *mut libc::c_char,
  pub line_alloc: size_t,
  pub nline_alloc: size_t,
  pub lineno: libc::c_int,
}

/*
 * public domain -- Dave 'Kill a Cop' Cinege <dcinege@psychosis.com>
 *
 * makedevs
 * Make ranges of device files quickly.
 * known bugs: can't deal with alpha ranges
 */
//config:config MAKEDEVS
//config:	bool "makedevs (9.2 kb)"
//config:	default y
//config:	help
//config:	'makedevs' is a utility used to create a batch of devices with
//config:	one command.
//config:
//config:	There are two choices for command line behaviour, the interface
//config:	as used by LEAF/Linux Router Project, or a device table file.
//config:
//config:	'leaf' is traditionally what busybox follows, it allows multiple
//config:	devices of a particluar type to be created per command.
//config:	e.g. /dev/hda[0-9]
//config:	Device properties are passed as command line arguments.
//config:
//config:	'table' reads device properties from a file or stdin, allowing
//config:	a batch of unrelated devices to be made with one command.
//config:	User/group names are allowed as an alternative to uid/gid.
//config:
//config:choice
//config:	prompt "Choose makedevs behaviour"
//config:	depends on MAKEDEVS
//config:	default FEATURE_MAKEDEVS_TABLE
//config:
//config:config FEATURE_MAKEDEVS_LEAF
//config:	bool "leaf"
//config:
//config:config FEATURE_MAKEDEVS_TABLE
//config:	bool "table"
//config:
//config:endchoice
//applet:IF_MAKEDEVS(APPLET_NOEXEC(makedevs, makedevs, BB_DIR_SBIN, BB_SUID_DROP, makedevs))
//kbuild:lib-$(CONFIG_MAKEDEVS) += makedevs.o
//usage:#if ENABLE_FEATURE_MAKEDEVS_LEAF
//usage:#define makedevs_trivial_usage
//usage:       "NAME TYPE MAJOR MINOR FIRST LAST [s]"
//usage:#define makedevs_full_usage "\n\n"
//usage:       "Create a range of block or character special files"
//usage:     "\n"
//usage:     "\nTYPE is:"
//usage:     "\n	b	Block device"
//usage:     "\n	c	Character device"
//usage:     "\n	f	FIFO, MAJOR and MINOR are ignored"
//usage:     "\n"
//usage:     "\nFIRST..LAST specify numbers appended to NAME."
//usage:     "\nIf 's' is the last argument, the base device is created as well."
//usage:     "\n"
//usage:     "\nExamples:"
//usage:     "\n	makedevs /dev/ttyS c 4 66 2 63   ->  ttyS2-ttyS63"
//usage:     "\n	makedevs /dev/hda b 3 0 0 8 s    ->  hda,hda1-hda8"
//usage:
//usage:#define makedevs_example_usage
//usage:       "# makedevs /dev/ttyS c 4 66 2 63\n"
//usage:       "[creates ttyS2-ttyS63]\n"
//usage:       "# makedevs /dev/hda b 3 0 0 8 s\n"
//usage:       "[creates hda,hda1-hda8]\n"
//usage:#endif
//usage:
//usage:#if ENABLE_FEATURE_MAKEDEVS_TABLE
//usage:#define makedevs_trivial_usage
//usage:       "[-d device_table] rootdir"
//usage:#define makedevs_full_usage "\n\n"
//usage:       "Create a range of special files as specified in a device table.\n"
//usage:       "Device table entries take the form of:\n"
//usage:       "<name> <type> <mode> <uid> <gid> <major> <minor> <start> <inc> <count>\n"
//usage:       "Where name is the file name, type can be one of:\n"
//usage:       "	f	Regular file\n"
//usage:       "	d	Directory\n"
//usage:       "	c	Character device\n"
//usage:       "	b	Block device\n"
//usage:       "	p	Fifo (named pipe)\n"
//usage:       "uid is the user id for the target file, gid is the group id for the\n"
//usage:       "target file. The rest of the entries (major, minor, etc) apply to\n"
//usage:       "to device special files. A '-' may be used for blank entries."
//usage:
//usage:#define makedevs_example_usage
//usage:       "For example:\n"
//usage:       "<name>    <type> <mode><uid><gid><major><minor><start><inc><count>\n"
//usage:       "/dev         d   755    0    0    -      -      -      -    -\n"
//usage:       "/dev/console c   666    0    0    5      1      -      -    -\n"
//usage:       "/dev/null    c   666    0    0    1      3      0      0    -\n"
//usage:       "/dev/zero    c   666    0    0    1      5      0      0    -\n"
//usage:       "/dev/hda     b   640    0    0    3      0      0      0    -\n"
//usage:       "/dev/hda     b   640    0    0    3      1      1      1    15\n\n"
//usage:       "Will Produce:\n"
//usage:       "/dev\n"
//usage:       "/dev/console\n"
//usage:       "/dev/null\n"
//usage:       "/dev/zero\n"
//usage:       "/dev/hda\n"
//usage:       "/dev/hda[0-15]\n"
//usage:#endif
/* Licensed under GPLv2 or later, see file LICENSE in this source tree. */
#[no_mangle]
pub unsafe extern "C" fn makedevs_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut parser: *mut parser_t = 0 as *mut parser_t; /* ensure root dir exists */
  let mut line: *mut libc::c_char =
    b"-\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
  let mut ret: libc::c_int = 0i32;
  getopt32(
    argv,
    b"^d:\x00=1\x00" as *const u8 as *const libc::c_char,
    &mut line as *mut *mut libc::c_char,
  );
  argv = argv.offset(optind as isize);
  xchdir(*argv);
  umask(0i32 as mode_t);
  printf(
    b"rootdir=%s\ntable=\x00" as *const u8 as *const libc::c_char,
    *argv,
  );
  if *line.offset(0) as libc::c_int != '-' as i32 || *line.offset(1) as libc::c_int != 0 {
    printf(b"\'%s\'\n\x00" as *const u8 as *const libc::c_char, line);
  } else {
    puts(b"<stdin>\x00" as *const u8 as *const libc::c_char);
  }
  parser = config_open(line);
  let mut current_block_56: u64;
  while config_read(
    parser,
    &mut line,
    (PARSE_NORMAL as libc::c_int | (1i32 & 0xffi32) << 8i32 | 1i32 & 0xffi32) as libc::c_uint,
    b"# \t\x00" as *const u8 as *const libc::c_char,
  ) != 0
  {
    let mut linenum: libc::c_int = 0;
    let mut type_0: libc::c_char = 0;
    let mut mode: libc::c_uint = 0o755i32 as libc::c_uint;
    let mut major: libc::c_uint = 0i32 as libc::c_uint;
    let mut minor: libc::c_uint = 0i32 as libc::c_uint;
    let mut count: libc::c_uint = 0i32 as libc::c_uint;
    let mut increment: libc::c_uint = 0i32 as libc::c_uint;
    let mut start: libc::c_uint = 0i32 as libc::c_uint;
    let mut user: [libc::c_char; 41] = [0; 41];
    let mut group: [libc::c_char; 41] = [0; 41];
    let mut full_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name_len: libc::c_int = 0;
    let mut uid: uid_t = 0;
    let mut gid: gid_t = 0;
    linenum = (*parser).lineno;
    if 1i32
      > sscanf(
        line,
        b"%*s%n %c %o %40s %40s %u %u %u %u %u\x00" as *const u8 as *const libc::c_char,
        &mut name_len as *mut libc::c_int,
        &mut type_0 as *mut libc::c_char,
        &mut mode as *mut libc::c_uint,
        user.as_mut_ptr(),
        group.as_mut_ptr(),
        &mut major as *mut libc::c_uint,
        &mut minor as *mut libc::c_uint,
        &mut start as *mut libc::c_uint,
        &mut increment as *mut libc::c_uint,
        &mut count as *mut libc::c_uint,
      )
      || major | minor | start | count | increment > 255i32 as libc::c_uint
    {
      bb_error_msg(
        b"invalid line %d: \'%s\'\x00" as *const u8 as *const libc::c_char,
        linenum,
        line,
      );
      ret = 1i32
    } else {
      gid = if *group.as_mut_ptr() as libc::c_int != 0 {
        get_ug_id(
          group.as_mut_ptr(),
          Some(xgroup2gid as unsafe extern "C" fn(_: *const libc::c_char) -> libc::c_long),
        )
      } else {
        getgid() as libc::c_ulong
      } as gid_t;
      uid = if *user.as_mut_ptr() as libc::c_int != 0 {
        get_ug_id(
          user.as_mut_ptr(),
          Some(xuname2uid as unsafe extern "C" fn(_: *const libc::c_char) -> libc::c_long),
        )
      } else {
        getuid() as libc::c_ulong
      } as uid_t;
      *line.offset(name_len as isize) = '\u{0}' as i32 as libc::c_char;
      full_name = line;
      /* We are already in the right root dir,
       * so make absolute paths relative */
      if '/' as i32 == *full_name.offset(0) as libc::c_int {
        full_name = full_name.offset(1)
      }
      if type_0 as libc::c_int == 'd' as i32 {
        bb_make_directory(
          full_name,
          (mode | 0o40000i32 as libc::c_uint) as libc::c_long,
          FILEUTILS_RECUR as libc::c_int,
        );
        if chown(full_name, uid, gid) == -1i32 {
          current_block_56 = 6367029485986847337;
        } else {
          if !(chmod(full_name, mode) < 0i32) {
            continue;
          }
          current_block_56 = 1050926138102375056;
        }
      } else if type_0 as libc::c_int == 'f' as i32 {
        let mut st: stat = std::mem::zeroed();
        if stat(full_name, &mut st) < 0i32
          || !(st.st_mode & 0o170000i32 as libc::c_uint == 0o100000i32 as libc::c_uint)
        {
          bb_perror_msg(
            b"line %d: regular file \'%s\' does not exist\x00" as *const u8 as *const libc::c_char,
            linenum,
            full_name,
          );
          ret = 1i32;
          continue;
        } else if chown(full_name, uid, gid) < 0i32 {
          current_block_56 = 6367029485986847337;
        } else {
          if !(chmod(full_name, mode) < 0i32) {
            continue;
          }
          current_block_56 = 1050926138102375056;
        }
      } else {
        let mut i: libc::c_uint = 0;
        if type_0 as libc::c_int == 'p' as i32 {
          mode |= 0o10000i32 as libc::c_uint
        } else if type_0 as libc::c_int == 'c' as i32 {
          mode |= 0o20000i32 as libc::c_uint
        } else if type_0 as libc::c_int == 'b' as i32 {
          mode |= 0o60000i32 as libc::c_uint
        } else {
          bb_error_msg(
            b"line %d: unsupported file type %c\x00" as *const u8 as *const libc::c_char,
            linenum,
            type_0 as libc::c_int,
          );
          ret = 1i32;
          continue;
        }
        if count != 0i32 as libc::c_uint {
          count = count.wrapping_sub(1)
        }
        i = 0i32 as libc::c_uint;
        while i <= count {
          let mut rdev: libc::dev_t = 0;
          let mut nameN: *mut libc::c_char = full_name;
          if count != 0i32 as libc::c_uint {
            nameN = xasprintf(
              b"%s%u\x00" as *const u8 as *const libc::c_char,
              full_name,
              start.wrapping_add(i),
            )
          }
          rdev = bb_makedev(major, minor.wrapping_add(i.wrapping_mul(increment))) as libc::dev_t;
          if mknod(nameN, mode, rdev) != 0i32 && *bb_errno != 17i32 {
            bb_perror_msg(
              b"line %d: can\'t create node %s\x00" as *const u8 as *const libc::c_char,
              linenum,
              nameN,
            );
            ret = 1i32
          } else if chown(nameN, uid, gid) < 0i32 {
            bb_perror_msg(
              b"line %d: can\'t chown %s\x00" as *const u8 as *const libc::c_char,
              linenum,
              nameN,
            );
            ret = 1i32
          } else if chmod(nameN, mode) < 0i32 {
            bb_perror_msg(
              b"line %d: can\'t chmod %s\x00" as *const u8 as *const libc::c_char,
              linenum,
              nameN,
            );
            ret = 1i32
          }
          if count != 0i32 as libc::c_uint {
            free(nameN as *mut libc::c_void);
          }
          i = i.wrapping_add(1)
        }
        continue;
      }
      match current_block_56 {
        1050926138102375056 => {
          bb_perror_msg(
            b"line %d: can\'t chmod %s\x00" as *const u8 as *const libc::c_char,
            linenum,
            full_name,
          );
          ret = 1i32
        }
        _ => {
          bb_perror_msg(
            b"line %d: can\'t chown %s\x00" as *const u8 as *const libc::c_char,
            linenum,
            full_name,
          );
          ret = 1i32
        }
      }
    }
  }
  return ret;
}
