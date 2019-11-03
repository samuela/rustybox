use libc;
extern "C" {
  #[no_mangle]
  fn rand() -> libc::c_int;
  #[no_mangle]
  fn srand(__seed: libc::c_uint);
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  static mut stdout: *mut _IO_FILE;
  #[no_mangle]
  fn freopen(
    __filename: *const libc::c_char,
    __modes: *const libc::c_char,
    __stream: *mut FILE,
  ) -> *mut FILE;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn puts(__s: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn monotonic_us() -> libc::c_ulonglong;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn bb_get_last_path_component_strip(path: *mut libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;
  #[no_mangle]
  static ptr_to_globals: *mut globals;
  #[no_mangle]
  fn printfile_base64(fname: *const libc::c_char);
}
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
pub type pid_t = __pid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
  pub _flags: libc::c_int,
  pub _IO_read_ptr: *mut libc::c_char,
  pub _IO_read_end: *mut libc::c_char,
  pub _IO_read_base: *mut libc::c_char,
  pub _IO_write_base: *mut libc::c_char,
  pub _IO_write_ptr: *mut libc::c_char,
  pub _IO_write_end: *mut libc::c_char,
  pub _IO_buf_base: *mut libc::c_char,
  pub _IO_buf_end: *mut libc::c_char,
  pub _IO_save_base: *mut libc::c_char,
  pub _IO_backup_base: *mut libc::c_char,
  pub _IO_save_end: *mut libc::c_char,
  pub _markers: *mut _IO_marker,
  pub _chain: *mut _IO_FILE,
  pub _fileno: libc::c_int,
  pub _flags2: libc::c_int,
  pub _old_offset: __off_t,
  pub _cur_column: libc::c_ushort,
  pub _vtable_offset: libc::c_schar,
  pub _shortbuf: [libc::c_char; 1],
  pub _lock: *mut libc::c_void,
  pub _offset: __off64_t,
  pub __pad1: *mut libc::c_void,
  pub __pad2: *mut libc::c_void,
  pub __pad3: *mut libc::c_void,
  pub __pad4: *mut libc::c_void,
  pub __pad5: size_t,
  pub _mode: libc::c_int,
  pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
  pub _next: *mut _IO_marker,
  pub _sbuf: *mut _IO_FILE,
  pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct llist_t {
  pub link: *mut llist_t,
  pub data: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub helper_pid: pid_t,
  pub timeout: libc::c_uint,
  pub verbose: libc::c_uint,
  pub opts: libc::c_uint,
  pub user: *mut libc::c_char,
  pub pass: *mut libc::c_char,
  pub fp0: *mut FILE,
  pub opt_charset: *mut libc::c_char,
}
// Content-Transfer-Encoding. Ignored. Assumed base64
pub const OPT_o: C2RustUnnamed = 4;
pub type C2RustUnnamed = libc::c_uint;
// additional headers
//OPT_m = 1 << 6,         // create mutipart section
//OPT_j = 1 << 7,         // join section to multipart section
// COMPAT
pub const OPT_a: C2RustUnnamed = 32;
// charset
pub const OPT_N: C2RustUnnamed = 16;
// output to
pub const OPT_C: C2RustUnnamed = 8;
// create (non-multipart) section
pub const OPT_e: C2RustUnnamed = 2;
pub const OPT_c: C2RustUnnamed = 1;
#[inline(always)]
unsafe extern "C" fn not_const_pp(mut p: *const libc::c_void) -> *mut libc::c_void {
  return p as *mut libc::c_void;
}
/* vi: set sw=4 ts=4: */
/*
 * makemime: create MIME-encoded message
 *
 * Copyright (C) 2008 by Vladimir Dronnikov <dronnikov@gmail.com>
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
//config:config MAKEMIME
//config:	bool "makemime (5.4 kb)"
//config:	default y
//config:	help
//config:	Create MIME-formatted messages.
//applet:IF_MAKEMIME(APPLET(makemime, BB_DIR_BIN, BB_SUID_DROP))
//kbuild:lib-$(CONFIG_MAKEMIME) += makemime.o mail.o
/*
  makemime -c type [-o file] [-e encoding] [-C charset] [-N name] \
                   [-a "Header: Contents"] file
           -m [ type ] [-o file] [-e encoding] [-a "Header: Contents"] file
           -j [-o file] file1 file2
           @file

   file:  filename    - read or write from filename
          -           - read or write from stdin or stdout
          &n          - read or write from file descriptor n
          \( opts \)  - read from child process, that generates [ opts ]

Options:
  -c type         - create a new MIME section from "file" with this
                    Content-Type: (default is application/octet-stream).
  -C charset      - MIME charset of a new text/plain section.
  -N name         - MIME content name of the new mime section.
  -m [ type ]     - create a multipart mime section from "file" of this
                    Content-Type: (default is multipart/mixed).
  -e encoding     - use the given encoding (7bit, 8bit, quoted-printable,
                    or base64), instead of guessing.  Omit "-e" and use
                    -c auto to set Content-Type: to text/plain or
                    application/octet-stream based on picked encoding.
  -j file1 file2  - join mime section file2 to multipart section file1.
  -o file         - write the result to file, instead of stdout (not
                    allowed in child processes).
  -a header       - prepend an additional header to the output.

  @file - read all of the above options from file, one option or
          value on each line.
  {which version of makemime is this? What do we support?}
*/
/* man makemime:

* -c TYPE: create a (non-multipart) MIME section with Content-Type: TYPE
* makemime -c TYPE [-e ENCODING] [-o OUTFILE] [-C CHARSET] [-N NAME] [-a HEADER...] FILE
* The -C option sets the MIME charset attribute for text/plain content.
* The -N option sets the name attribute for Content-Type:
* Encoding must be one of the following: 7bit, 8bit, quoted-printable, or base64.

* -m multipart/TYPE: create a multipart MIME collection with Content-Type: multipart/TYPE
* makemime -m multipart/TYPE [-e ENCODING] [-o OUTFILE] [-a HEADER...] FILE
* Type must be either "multipart/mixed", "multipart/alternative", or some other MIME multipart content type.
* Additionally, encoding can only be "7bit" or "8bit", and will default to "8bit" if not specified.
* Finally, filename must be a MIME-formatted section, NOT a regular file.
* The -m option creates an initial multipart MIME collection, that contains only one MIME section, taken from filename.
* The collection is written to standard output, or the pipe or to outputfile.

* -j FILE1: add a section to a multipart MIME collection
* makemime -j FILE1 [-o OUTFILE] FILE2
* FILE1 must be a MIME collection that was previously created by the -m option.
* FILE2 must be a MIME section that was previously created by the -c option.
* The -j options adds the MIME section in FILE2 to the MIME collection in FILE1.
*/
/* In busybox 1.15.0.svn, makemime generates output like this
 * (empty lines are shown exactly!):
{headers added with -a HDR}
Mime-Version: 1.0
Content-Type: multipart/mixed; boundary="24269534-2145583448-1655890676"

--24269534-2145583448-1655890676
Content-Type: {set by -c, e.g. text/plain}; charset={set by -C, e.g. us-ascii}
Content-Disposition: inline; filename="A"
Content-Transfer-Encoding: base64

...file A contents...
--24269534-2145583448-1655890676
Content-Type: {set by -c, e.g. text/plain}; charset={set by -C, e.g. us-ascii}
Content-Disposition: inline; filename="B"
Content-Transfer-Encoding: base64

...file B contents...
--24269534-2145583448-1655890676--

 *
 * For reference: here is an example email to LKML which has
 * 1st unnamed part (so it serves as an email body)
 * and one attached file:
...other headers...
Content-Type: multipart/mixed; boundary="=-tOfTf3byOS0vZgxEWcX+"
...other headers...
Mime-Version: 1.0
...other headers...


--=-tOfTf3byOS0vZgxEWcX+
Content-Type: text/plain
Content-Transfer-Encoding: 7bit

...email text...
...email text...


--=-tOfTf3byOS0vZgxEWcX+
Content-Disposition: attachment; filename="xyz"
Content-Type: text/plain; name="xyz"; charset="UTF-8"
Content-Transfer-Encoding: 7bit

...file contents...
...file contents...

--=-tOfTf3byOS0vZgxEWcX+--

...random junk added by mailing list robots and such...
*/
//usage:#define makemime_trivial_usage
//usage:       "[OPTIONS] [FILE]..."
//usage:#define makemime_full_usage "\n\n"
//usage:       "Create multipart MIME-encoded message from FILEs\n"
/* //usage:    "Transfer encoding is base64, disposition is inline (not attachment)\n" */
//usage:     "\n	-o FILE	Output. Default: stdout"
//usage:     "\n	-a HDR	Add header(s). Examples:"
//usage:     "\n		\"From: user@host.org\", \"Date: `date -R`\""
//usage:     "\n	-c CT	Content type. Default: application/octet-stream"
//usage:     "\n	-C CS	Charset. Default: " CONFIG_FEATURE_MIME_CHARSET
/* //usage:  "\n	-e ENC	Transfer encoding. Ignored. base64 is assumed" */
//usage:     "\n"
//usage:     "\nOther options are silently ignored"
/*
 * -c [Content-Type] should create just one MIME section
 * with "Content-Type:", "Content-Transfer-Encoding:", and HDRs from "-a HDR".
 * NB: without "Content-Disposition:" auto-added, unlike we do now
 * NB2: -c has *optional* param which nevertheless _can_ be specified after a space :(
 *
 * -m [multipart/mixed] should create multipart MIME section
 * with "Content-Type:", "Content-Transfer-Encoding:", and HDRs from "-a HDR",
 * and add FILE to it _verbatim_:
 *  HEADERS
 *
 *  --=_1_1321709112_1605
 *  FILE_CONTENTS
 *  --=_1_1321709112_1605
 * without any encoding of FILE_CONTENTS. (Basically, it expects that FILE
 * is the result of "makemime -c").
 *
 * -j MULTIPART_FILE1 SINGLE_FILE2 should output MULTIPART_FILE1 + SINGLE_FILE2
 *
 * Our current behavior is a mutant "-m + -c + -j" one: we create multipart MIME
 * and we put "-c" encoded FILEs into many multipart sections.
 */
#[no_mangle]
pub unsafe extern "C" fn makemime_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut opt_headers: *mut llist_t = 0 as *mut llist_t;
  let mut l: *mut llist_t = 0 as *mut llist_t;
  let mut opt_output: *const libc::c_char = 0 as *const libc::c_char;
  let mut content_type: *const libc::c_char =
    b"application/octet-stream\x00" as *const u8 as *const libc::c_char;
  let ref mut fresh0 = *(not_const_pp(&ptr_to_globals as *const *mut globals as *const libc::c_void)
    as *mut *mut globals);
  *fresh0 = xzalloc(::std::mem::size_of::<globals>() as libc::c_ulong) as *mut globals;
  asm!("" : : : "memory" : "volatile");
  (*ptr_to_globals).opt_charset =
    b"us-ascii\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
  // parse options
  (*ptr_to_globals).opts = getopt32(
    argv,
    b"c:e:o:C:N:a:*\x00" as *const u8 as *const libc::c_char,
    &mut content_type as *mut *const libc::c_char,
    0 as *mut libc::c_void,
    &mut opt_output as *mut *const libc::c_char,
    &mut (*ptr_to_globals).opt_charset as *mut *mut libc::c_char,
    0 as *mut libc::c_void,
    &mut opt_headers as *mut *mut llist_t,
  );
  //argc -= optind;
  argv = argv.offset(optind as isize);
  // respect -o output
  if (*ptr_to_globals).opts & OPT_o as libc::c_int as libc::c_uint != 0 {
    freopen(
      opt_output,
      b"w\x00" as *const u8 as *const libc::c_char,
      stdout,
    );
  }
  // no files given on command line? -> use stdin
  if (*argv).is_null() {
    argv = argv.offset(-1);
    *argv = b"-\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
  }
  // put additional headers
  l = opt_headers;
  while !l.is_null() {
    puts((*l).data);
    l = (*l).link
  }
  // make a random string -- it will delimit message parts
  srand(monotonic_us() as libc::c_uint);
  opt_output = xasprintf(
    b"%u-%u-%u\x00" as *const u8 as *const libc::c_char,
    rand() as libc::c_uint,
    rand() as libc::c_uint,
    rand() as libc::c_uint,
  );
  // put multipart header
  printf(
    b"Mime-Version: 1.0\nContent-Type: multipart/mixed; boundary=\"%s\"\n\x00" as *const u8
      as *const libc::c_char,
    opt_output,
  );
  // put attachments
  while !(*argv).is_null() {
    printf(b"\n--%s\nContent-Type: %s; charset=%s\nContent-Disposition: inline; filename=\"%s\"\nContent-Transfer-Encoding: base64\n\x00"
                   as *const u8 as *const libc::c_char, opt_output,
               content_type, (*ptr_to_globals).opt_charset,
               bb_get_last_path_component_strip(*argv));
    let fresh1 = argv;
    argv = argv.offset(1);
    printfile_base64(*fresh1);
  }
  // put multipart footer
  printf(
    b"\n--%s--\n\n\x00" as *const u8 as *const libc::c_char,
    opt_output,
  );
  return 0i32;
}
