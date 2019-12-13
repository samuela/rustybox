use crate::libbb::ptr_to_globals::bb_errno;
use libc;
use libc::free;
use libc::lstat;
use libc::printf;
use libc::rename;
use libc::stat;
use libc::symlink;
use libc::unlink;
extern "C" {

  #[no_mangle]
  fn link(__from: *const libc::c_char, __to: *const libc::c_char) -> libc::c_int;

  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn is_directory(name: *const libc::c_char, followLinks: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn bb_get_last_path_component_strip(path: *mut libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);
  #[no_mangle]
  fn concat_path_file(
    path: *const libc::c_char,
    filename: *const libc::c_char,
  ) -> *mut libc::c_char;
}

#[no_mangle]
pub unsafe extern "C" fn ln_main(
  mut argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut status: libc::c_int = 0i32;
  let mut opts: libc::c_int = 0;
  let mut last: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut src_name: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut src: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut suffix: *mut libc::c_char =
    b"~\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
  let mut statbuf: stat = std::mem::zeroed();
  let mut link_func: Option<
    unsafe extern "C" fn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int,
  > = None;
  opts = getopt32(
    argv,
    b"^sfnbS:vT\x00-1\x00" as *const u8 as *const libc::c_char,
    &mut suffix as *mut *mut libc::c_char,
  ) as libc::c_int;
  /*
   -s, --symbolic		make symbolic links instead of hard links
   -f, --force		remove existing destination files
   -n, --no-dereference	treat LINK_NAME as a normal file if it is a symbolic link to a directory
   -b			like --backup but does not accept an argument
   --backup[=CONTROL]	make a backup of each existing destination file
   -S, --suffix=SUFFIX	override the usual backup suffix
   -v, --verbose
   -T, --no-target-directory
   -d, -F, --directory	allow the superuser to attempt to hard link directories
   -i, --interactive	prompt whether to remove destinations
   -L, --logical		dereference TARGETs that are symbolic links
   -P, --physical		make hard links directly to symbolic links
   -r, --relative		create symbolic links relative to link location
   -t, --target-directory=DIRECTORY	specify the DIRECTORY in which to create the links
  */
  last = *argv.offset((argc - 1i32) as isize);
  argv = argv.offset(optind as isize);
  argc -= optind;
  if opts & 1i32 << 6i32 != 0 && argc > 2i32 {
    bb_simple_error_msg_and_die(b"-T accepts 2 args max\x00" as *const u8 as *const libc::c_char);
  }
  if (*argv.offset(1)).is_null() {
    /* "ln PATH/TO/FILE" -> "ln PATH/TO/FILE FILE" */
    argv = argv.offset(-1);
    *argv = last;
    /* xstrdup is needed: "ln -s PATH/TO/FILE/" is equivalent to
     * "ln -s PATH/TO/FILE/ FILE", not "ln -s PATH/TO/FILE FILE"
     */
    last = bb_get_last_path_component_strip(xstrdup(last))
  }
  let mut current_block_45: u64;
  loop {
    src_name = std::ptr::null_mut::<libc::c_char>();
    src = last;
    if is_directory(
      src,
      (opts & (1i32 << 2i32 | 1i32 << 6i32) == 0) as libc::c_int,
    ) != 0
    {
      if opts & 1i32 << 6i32 != 0 {
        bb_error_msg_and_die(
          b"\'%s\' is a directory\x00" as *const u8 as *const libc::c_char,
          src,
        );
      }
      src_name = xstrdup(*argv);
      src = concat_path_file(src, bb_get_last_path_component_strip(src_name));
      free(src_name as *mut libc::c_void);
      src_name = src
    }
    if opts & 1i32 << 0i32 == 0 && stat(*argv, &mut statbuf) != 0 {
      // coreutils: "ln dangling_symlink new_hardlink" works
      if lstat(*argv, &mut statbuf) != 0
        || !(statbuf.st_mode & 0o170000i32 as libc::c_uint == 0o120000i32 as libc::c_uint)
      {
        bb_simple_perror_msg(*argv);
        status = 1i32;
        free(src_name as *mut libc::c_void);
        current_block_45 = 12349973810996921269;
      } else {
        current_block_45 = 5689316957504528238;
      }
    } else {
      current_block_45 = 5689316957504528238;
    }
    match current_block_45 {
      5689316957504528238 => {
        if opts & 1i32 << 3i32 != 0 {
          let mut backup: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
          backup = xasprintf(b"%s%s\x00" as *const u8 as *const libc::c_char, src, suffix);
          if rename(src, backup) < 0i32 && *bb_errno != 2i32 {
            bb_simple_perror_msg(src);
            status = 1i32;
            free(backup as *mut libc::c_void);
            current_block_45 = 12349973810996921269;
          } else {
            free(backup as *mut libc::c_void);
            /*
             * When the source and dest are both hard links to the same
             * inode, a rename may succeed even though nothing happened.
             * Therefore, always unlink().
             */
            unlink(src);
            current_block_45 = 17784502470059252271;
          }
        } else {
          if opts & 1i32 << 1i32 != 0 {
            unlink(src);
          }
          current_block_45 = 17784502470059252271;
        }
        match current_block_45 {
          12349973810996921269 => {}
          _ => {
            link_func = Some(
              link
                as unsafe extern "C" fn(
                  _: *const libc::c_char,
                  _: *const libc::c_char,
                ) -> libc::c_int,
            );
            if opts & 1i32 << 0i32 != 0 {
              link_func = Some(
                symlink
                  as unsafe extern "C" fn(
                    _: *const libc::c_char,
                    _: *const libc::c_char,
                  ) -> libc::c_int,
              )
            }
            if opts & 1i32 << 5i32 != 0 {
              printf(
                b"\'%s\' -> \'%s\'\n\x00" as *const u8 as *const libc::c_char,
                src,
                *argv,
              );
            }
            if link_func.expect("non-null function pointer")(*argv, src) != 0i32 {
              bb_simple_perror_msg(src);
              status = 1i32
            }
            free(src_name as *mut libc::c_void);
          }
        }
      }
      _ => {}
    }
    argv = argv.offset(1);
    if (*argv.offset(1)).is_null() {
      break;
    }
  }
  return status;
}
