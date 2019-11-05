use c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use libc;
extern "C" {
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
  #[no_mangle]
  static mut stderr: *mut FILE;
  #[no_mangle]
  fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn putc_unlocked(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  fn mempcpy(
    __dest: *mut libc::c_void,
    __src: *const libc::c_void,
    __n: size_t,
  ) -> *mut libc::c_void;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn strnlen(__string: *const libc::c_char, __maxlen: size_t) -> size_t;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xrealloc(old: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn pstm_clear(a: *mut pstm_int);
  #[no_mangle]
  fn xfunc_die() -> !;
  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn md5_hash(ctx: *mut md5_ctx_t, buffer: *const libc::c_void, len: size_t);
  #[no_mangle]
  fn itoa(n: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  /*
   * Copyright (C) 2017 Denys Vlasenko
   *
   * Licensed under GPLv2, see file LICENSE in this source tree.
   *
   * Selected few declarations for AES.
   */
  #[no_mangle]
  fn aes_cbc_decrypt(
    aes: *mut tls_aes,
    iv: *mut libc::c_void,
    data: *const libc::c_void,
    len: size_t,
    dst: *mut libc::c_void,
  );
  #[no_mangle]
  fn aes_encrypt_one_block(aes: *mut tls_aes, data: *const libc::c_void, dst: *mut libc::c_void);
  #[no_mangle]
  fn bb_perror_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn xwrite(fd: libc::c_int, buf: *const libc::c_void, count: size_t);
  /*
   * Copyright (C) 2018 Denys Vlasenko
   *
   * Licensed under GPLv2, see file LICENSE in this source tree.
   */
  #[no_mangle]
  fn aesgcm_GHASH(
    h: *mut uint8_t,
    a: *const uint8_t,
    c: *const uint8_t,
    cSz: libc::c_uint,
    s: *mut uint8_t,
  );
  #[no_mangle]
  fn aes_cbc_encrypt(
    aes: *mut tls_aes,
    iv: *mut libc::c_void,
    data: *const libc::c_void,
    len: size_t,
    dst: *mut libc::c_void,
  );
  #[no_mangle]
  fn open_read_close(
    filename: *const libc::c_char,
    buf: *mut libc::c_void,
    maxsz: size_t,
  ) -> ssize_t;
  #[no_mangle]
  fn sha1_end(ctx: *mut sha1_ctx_t, resbuf: *mut libc::c_void) -> libc::c_uint;
  #[no_mangle]
  fn sha256_begin(ctx: *mut sha256_ctx_t);
  #[no_mangle]
  fn aes_setkey(aes: *mut tls_aes, key: *const libc::c_void, key_len: libc::c_uint);
  #[no_mangle]
  fn curve25519(result: *mut uint8_t, e: *const uint8_t, q: *const uint8_t);
  #[no_mangle]
  fn bb_simple_error_msg(s: *const libc::c_char);
  #[no_mangle]
  fn psRsaEncryptPub(
    key: *mut psRsaKey_t,
    in_0: *mut libc::c_uchar,
    inlen: uint32,
    out: *mut libc::c_uchar,
    outlen: uint32,
  ) -> int32;
  #[no_mangle]
  fn pstm_unsigned_bin_size(a: *mut pstm_int) -> int32;
  #[no_mangle]
  fn pstm_read_unsigned_bin(a: *mut pstm_int, b: *mut libc::c_uchar, c: int32) -> int32;
  #[no_mangle]
  fn pstm_init_for_read_unsigned_bin(a: *mut pstm_int, len: uint32) -> int32;
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn safe_poll(ufds: *mut pollfd, nfds: nfds_t, timeout_ms: libc::c_int) -> libc::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
  pub gp_offset: libc::c_uint,
  pub fp_offset: libc::c_uint,
  pub overflow_arg_area: *mut libc::c_void,
  pub reg_save_area: *mut libc::c_void,
}

use crate::librb::__uint16_t;
use crate::librb::__int32_t;

use crate::librb::__uint64_t;
use crate::librb::__off_t;
use crate::librb::__off64_t;

use crate::librb::int32_t;
use crate::librb::uint8_t;
use crate::librb::uint16_t;
use crate::librb::uint32_t;
use crate::librb::uint64_t;
pub type bb__aliased_uint32_t = uint32_t;
pub type bb__aliased_uint64_t = uint64_t;
use crate::librb::ssize_t;
use crate::librb::size_t;

use crate::librb::FILE;
pub type va_list = __builtin_va_list;
pub type nfds_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
  pub fd: libc::c_int,
  pub events: libc::c_short,
  pub revents: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tls_aes {
  pub key: [uint32_t; 60],
  pub rounds: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tls_handshake_data {
  pub handshake_hash_ctx: md5sha_ctx_t,
  pub client_and_server_rand32: [uint8_t; 64],
  pub master_secret: [uint8_t; 48],
  pub server_rsa_pub_key: psRsaKey_t,
  pub ecc_pub_key32: [uint8_t; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct psRsaKey_t {
  pub e: pstm_int,
  pub d: pstm_int,
  pub N: pstm_int,
  pub qP: pstm_int,
  pub dP: pstm_int,
  pub dQ: pstm_int,
  pub p: pstm_int,
  pub q: pstm_int,
  pub size: uint32,
  pub optimized: int32,
}
pub type int32 = int32_t;
pub type uint32 = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pstm_int {
  pub used: libc::c_int,
  pub alloc: libc::c_int,
  pub sign: libc::c_int,
  pub dp: *mut pstm_digit,
}
pub type pstm_digit = uint32;
/* 0 if argv[0] is NULL: */
/* Guaranteed to NOT be a macro (smallest code). Saves nearly 2k on uclibc.
 * But potentially slow, don't use in one-billion-times loops */
/* Note: does not use stdio, writes to fd 2 directly */
// gcc-4.1.1 still isn't good enough at optimizing it
// (+200 bytes compared to macro)
//static ALWAYS_INLINE
//int LONE_DASH(const char *s) { return s[0] == '-' && !s[1]; }
//static ALWAYS_INLINE
//int NOT_LONE_DASH(const char *s) { return s[0] != '-' || s[1]; }
/* Returns a string with unprintable chars replaced by '?' or
 * SUBST_WCHAR. This function is unicode-aware. */
/* Prints unprintable char ch as ^C or M-c to file
 * (M-c is used only if ch is ORed with PRINTABLE_META),
 * else it is printed as-is (except for ch = 0x9b) */
/* Return a string that is the printable representation of character ch.
 * Buffer must hold at least four characters. */
// NB: will return short read on error, not -1,
// if some data was read before error occurred
// Reads one line a-la fgets (but doesn't save terminating '\n').
// Reads byte-by-byte. Useful when it is important to not read ahead.
// Bytes are appended to pfx (which must be malloced, or NULL).
/* Reads block up to *maxsz_p (default: INT_MAX - 4095) */
/* Returns NULL if file can't be opened (default max size: INT_MAX - 4095) */
/* Never returns NULL */
/* Else use variable one (a bit more expensive) */
/* Autodetects gzip/bzip2 formats. fd may be in the middle of the file! */
/* Autodetects .gz etc */
/* lzma has no signature, need a little helper. NB: exist only for ENABLE_FEATURE_SEAMLESS_LZMA=y */
// NB: will return short write on error, not -1,
// if some data was written before error occurred
/* Close fd, but check for failures (some types of write errors) */
/* Reads and prints to stdout till eof, then closes FILE. Exits on error: */
/* Reads a line from a text file, up to a newline or NUL byte, inclusive.
 * Returns malloc'ed char*. If end is NULL '\n' isn't considered
 * end of line. If end isn't NULL, length of the chunk is stored in it.
 * Returns NULL if EOF/error.
 */
/* Reads up to (and including) TERMINATING_STRING: */
/* Same, with limited max size, and returns the length (excluding NUL): */
/* Chops off TERMINATING_STRING from the end: */
/* Reads up to (and including) "\n" or NUL byte: */
/* Chops off '\n' from the end, unlike fgets: */
/* Same, but doesn't try to conserve space (may have some slack after the end) */
/* extern char *xmalloc_fgetline_fast(FILE *file) FAST_FUNC RETURNS_MALLOC; */
/* Prints warning to stderr and returns NULL on failure: */
/* "Opens" stdin if filename is special, else just opens file: */
/* not FAST_FUNC! */
/* Wrapper which restarts poll on EINTR or ENOMEM.
 * On other errors complains [perror("poll")] and returns.
 * Warning! May take (much) longer than timeout_ms to return!
 * If this is a problem, use bare poll and open-code EINTR/ENOMEM handling */
/* Convert each alpha char in str to lower-case */
/* Returns a pointer past the formatted number, does NOT null-terminate */
/* Intelligent formatters of bignums */
/* If block_size == 0, display size without fractional part,
 * else display (size * block_size) with one decimal digit.
 * If display_unit == 0, show value no bigger than 1024 with suffix (K,M,G...),
 * else divide by display_unit and do not use suffix. */
/* "1024.0G" */
//TODO: provide pointer to buf (avoid statics)?
/* Put a string of hex bytes ("1b2e66fe"...), return advanced pointer */
/* Reverse */
/* Generate a UUID */
/* Last element is marked by mult == 0 */
/* Specialized: */
/* Using xatoi() instead of naive atoi() is not always convenient -
 * in many places people want *non-negative* values, but store them
 * in signed int. Therefore we need this one:
 * dies if input is not in [0, INT_MAX] range. Also will reject '-0' etc.
 * It should really be named xatoi_nonnegative (since it allows 0),
 * but that would be too long.
 */
/* Useful for reading port numbers */
/* These parse entries in /etc/passwd and /etc/group.  This is desirable
 * for BusyBox since we want to avoid using the glibc NSS stuff, which
 * increases target size and is often not needed on embedded systems.  */
/* wrapper: allows string to contain numeric uid or gid */
/* always sets uid and gid; returns 0 on failure */
/* always sets uid and gid; exits on failure */
/* chown-like handling of "user[:[group]" */
/* versions which cache results (useful for ps, ls etc) */
/* internally usernames are saved in fixed-sized char[] buffers */
/*
 * Returns (-1) terminated malloced result of getgroups().
 * Reallocs group_array (useful for repeated calls).
 * ngroups is an initial size of array. It is rounded up to 32 for realloc.
 * ngroups is updated on return.
 * ngroups can be NULL: bb_getgroups(NULL, NULL) is valid usage.
 * Dies on errors (on Linux, only xrealloc can cause this, not internal getgroups call).
 */
/* BB_EXECxx always execs (it's not doing NOFORK/NOEXEC stuff),
 * but it may exec busybox and call applet instead of searching PATH.
 */
/* xvfork() can't be a _function_, return after vfork in child mangles stack
 * in the parent. It must be a macro. */
/* NOMMU friendy fork+exec: */
/* wait4pid: unlike waitpid, waits ONLY for one process.
 * Returns sig + 0x180 if child is killed by signal.
 * It's safe to pass negative 'pids' from failed [v]fork -
 * wait4pid will return -1 (and will not clobber [v]fork's errno).
 * IOW: rc = wait4pid(spawn(argv));
 *      if (rc < 0) bb_perror_msg("%s", argv[0]);
 *      if (rc > 0) bb_error_msg("exit code: %d", rc & 0xff);
 */
/* ***********************************************************************/
/* spawn_and_wait/run_nofork_applet/run_applet_no_and_exit need to work */
/* carefully together to reinit some global state while not disturbing  */
/* other. Be careful if you change them. Consult docs/nofork_noexec.txt */
/* ***********************************************************************/
/* Same as wait4pid(spawn(argv)), but with NOFORK/NOEXEC if configured: */
/* Does NOT check that applet is NOFORK, just blindly runs it */
/* Helpers for daemonization.
 *
 * bb_daemonize(flags) = daemonize, does not compile on NOMMU
 *
 * bb_daemonize_or_rexec(flags, argv) = daemonizes on MMU (and ignores argv),
 *      rexec's itself on NOMMU with argv passed as command line.
 * Thus bb_daemonize_or_rexec may cause your <applet>_main() to be re-executed
 * from the start. (It will detect it and not reexec again second time).
 * You have to audit carefully that you don't do something twice as a result
 * (opening files/sockets, parsing config files etc...)!
 *
 * Both of the above will redirect fd 0,1,2 to /dev/null and drop ctty
 * (will do setsid()).
 *
 * fork_or_rexec(argv) = bare-bones fork on MMU,
 *      "vfork + re-exec ourself" on NOMMU. No fd redirection, no setsid().
 *      On MMU ignores argv.
 *
 * Helper for network daemons in foreground mode:
 *
 * bb_sanitize_stdio() = make sure that fd 0,1,2 are opened by opening them
 * to /dev/null if they are not.
 */
/* internal use */
//DAEMON_DOUBLE_FORK     = 1 << 4, /* double fork to avoid controlling tty */
/* Clear dangerous stuff, set PATH. Return 1 if was run by different user. */
/* For top, ps. Some argv[i] are replaced by malloced "-opt" strings */
/* { "-", NULL } */
/* BSD-derived getopt() functions require that optind be set to 1 in
 * order to reset getopt() state.  This used to be generally accepted
 * way of resetting getopt().  However, glibc's getopt()
 * has additional getopt() state beyond optind (specifically, glibc
 * extensions such as '+' and '-' at the start of the string), and requires
 * that optind be set to zero to reset its state.  BSD-derived versions
 * of getopt() misbehaved if optind is set to 0 in order to reset getopt(),
 * and glibc's getopt() used to coredump if optind is set 1 in order
 * to reset getopt().
 * Then BSD introduced additional variable "optreset" which should be
 * set to 1 in order to reset getopt().  Sigh.  Standards, anyone?
 *
 * By ~2008, OpenBSD 3.4 was changed to survive glibc-like optind = 0
 * (to interpret it as if optreset was set).
 */
/*def __GLIBC__*/
/* BSD style */
/* Having next pointer as a first member allows easy creation
 * of "llist-compatible" structs, and using llist_FOO functions
 * on them.
 */
/* BTW, surprisingly, changing API to
 *   llist_t *llist_add_to(llist_t *old_head, void *data)
 * etc does not result in smaller code... */
/* start_stop_daemon and udhcpc are special - they want
 * to create pidfiles regardless of FEATURE_PIDFILE */
/* True only if we created pidfile which is *file*, not /dev/null etc */
/* We need to export XXX_main from libbusybox
 * only if we build "individual" binaries
 */
/* Embedded script support */
/* Applets which are useful from another applets */
/* If shell needs them, they exist even if not enabled as applets */
/* Similar, but used by chgrp, not shell */
/* Used by ftpd */
/* Don't need IF_xxx() guard for these */
/* Networking */
/* This structure defines protocol families and their handlers. */
/*int type,*/
/* may modify src */
/* This structure defines hardware protocols and their handlers. */
/*
 * If *devname is not NULL, use that name, otherwise try to find free one,
 * malloc and return it in *devname.
 * return value is the opened fd to the loop device, or < on error
 */
/* These constants match linux/loop.h (without BB_ prefix): */
/* Returns malloced str */
/* Like bb_ask_noecho, but asks on stdin with no timeout.  */
/* Returns -1 if input is invalid. current_mode is a base for e.g. "u+rw" */
/*
 * Config file parser
 */
// treat consecutive delimiters as one
// trim leading and trailing delimiters
// TODO: COLLAPSE and TRIM seem to always go in pair
// last token takes entire remainder of the line
// die if < min tokens found
// keep a copy of current line
// comments are recognized even if they aren't the first char
// delim[0] and delim[1] are two different allowed comment chars
// (so far, delim[0] will only work as comment char for full-line comment)
// (IOW: it works as if PARSE_EOL_COMMENTS is not set. sysctl applet is okay with this)
// comments are recognized even if there is whitespace before
// ("line start><space><tab><space>#comment" is also comment, not only "line start>#comment")
// NORMAL is:
// * remove leading and trailing delimiters and collapse
//   multiple delimiters into one
// * warn and continue if less than mintokens delimiters found
// * grab everything into last token
// * comments are recognized even if they aren't the first char
/* delims[0] is a comment char (use '\0' to disable), the rest are token delimiters */
/* Concatenate path and filename to new allocated buffer.
 * Add "/" only as needed (no duplicate "//" are produced).
 * If path is NULL, it is assumed to be "/".
 * filename should not be NULL. */
/* Returns NULL on . and .. */
/* Returns ptr to NUL */
/* Returns $SHELL, getpwuid(getuid())->pw_shell, or DEFAULT_SHELL.
 * Note that getpwuid result might need xstrdup'ing
 * if there is a possibility of intervening getpwxxx() calls.
 */
/* Structures inside "struct caps" are Linux-specific and libcap-specific: */
/* setup_environment:
 * if chdir pw->pw_dir: ok: else if to_tmp == 1: goto /tmp else: goto / or die
 * if clear_env = 1: cd(pw->pw_dir), clear environment, then set
 *   TERM=(old value)
 *   USER=pw->pw_name, LOGNAME=pw->pw_name
 *   PATH=bb_default_[root_]path
 *   HOME=pw->pw_dir
 *   SHELL=shell
 * else if change_env = 1:
 *   if not root (if pw->pw_uid != 0):
 *     USER=pw->pw_name, LOGNAME=pw->pw_name
 *   HOME=pw->pw_dir
 *   SHELL=shell
 * else does nothing
 *
 * NB: CHANGEENV and CLEARENV use setenv() - this leaks memory!
 * If setup_environment() is used is vforked child, this leaks memory _in parent too_!
 */
/* Returns a malloced string */
/*
 * rnd is additional random input. New one is returned.
 * Useful if you call crypt_make_salt many times in a row:
 * rnd = crypt_make_salt(buf1, 4, 0);
 * rnd = crypt_make_salt(buf2, 4, rnd);
 * rnd = crypt_make_salt(buf3, 4, rnd);
 * (otherwise we risk having same salt generated)
 */
/*, int rnd*/
/* "$N$" + sha_salt_16_bytes + NUL */
/* Returns number of lines changed, or -1 on error */
/* NB: typically you want to pass fd 0, not 1. Think 'applet | grep something' */
/* NB: "unsigned request" is crucial! "int request" will break some arches! */
/* At least glibc has horrendously large inline for this, so wrap it */
/* "Keycodes" that report an escape sequence.
 * We use something which fits into signed char,
 * yet doesn't represent any valid Unicode character.
 * Also, -1 is reserved for error indication and we don't use it. */
/* Used only if Alt/Ctrl/Shifted */
/* Used only if Alted */
/* ^^^^^ Be sure that last defined value is small enough.
 * Current read_key() code allows going up to -32 (0xfff..fffe0).
 * This gives three upper bits in LSB to play with:
 * KEYCODE_foo values are 0xfff..fffXX, lowest XX bits are: scavvvvv,
 * s=0 if SHIFT, c=0 if CTRL, a=0 if ALT,
 * vvvvv bits are the same for same key regardless of "shift bits".
 */
//KEYCODE_SHIFT_...   = KEYCODE_...   & ~0x80,
/* 0xfff..fff00 */
/* How long is the longest ESC sequence we know?
 * We want it big enough to be able to contain
 * cursor position sequence "ESC [ 9999 ; 9999 R"
 */
/* Note: fd may be in blocking or non-blocking mode, both make sense.
 * For one, less uses non-blocking mode.
 * Only the first read syscall inside read_key may block indefinitely
 * (unless fd is in non-blocking mode),
 * subsequent reads will time out after a few milliseconds.
 * Return of -1 means EOF or error (errno == 0 on EOF).
 * buffer[0] is used as a counter of buffered chars and must be 0
 * on first call.
 * timeout:
 * -2: do not poll(-1) for input - read() it, return on EAGAIN at once
 * -1: poll(-1) (i.e. block even on NONBLOCKed fd)
 * >=0: poll() for TIMEOUT milliseconds, return -1/EAGAIN on timeout
 */
/* It's NOT just ENABLEd or disabled. It's a number: */
/* must never be <= 0 */
/* meaning of this field depends on FEATURE_EDITING_SAVE_ON_EXIT:
 * if !FEATURE_EDITING_SAVE_ON_EXIT: "how many lines are
 * in on-disk history"
 * if FEATURE_EDITING_SAVE_ON_EXIT: "how many in-memory lines are
 * also in on-disk history (and thus need to be skipped on save)"
 */
/*
 * maxsize must be >= 2.
 * Returns:
 * -1 on read errors or EOF, or on bare Ctrl-D,
 * 0  on ctrl-C (the line entered is still returned in 'command'),
 * >0 length of input string, including terminating '\n'
 */
/* synchronize with sizeof(task_struct.comm) in /usr/include/linux/sched.h */
// For mixed 32/64 userspace, 32-bit pmap still needs
// 64-bit field here to correctly show 64-bit processes:
// (strictly speaking, other fields need to be wider too,
// but they are in kbytes, not bytes, and they hold sizes,
// not start addresses, sizes tend to be less than 4 terabytes)
/* Fields are set to 0/NULL if failed to determine (or not requested) */
/* Everything below must contain no ptrs to malloc'ed data:
 * it is memset(0) for each process in procps_scan() */
/* we round it to kbytes */
/* basename of executable in exec(2), read from /proc/N/stat
 * (if executable is symlink or script, it is NOT replaced
 * by link target or interpreter name) */
/* user/group? - use passwd/group parsing functions */
/* flag bits for procps_scan(xx, flags) calls */
/* PSSCAN_CMD      = 1 << 6, - use read_cmdline instead */
/* NB: used by find_pid_by_name(). Any applet using it
 * needs to be mentioned here. */
//procps_status_t* alloc_procps_scan(void) FAST_FUNC;
/* Format cmdline (up to col chars) into char buf[size] */
/* Puts [comm] if cmdline is empty (-> process is a kernel thread) */
/* Use strict=1 if you process input from untrusted source:
 * it will return NULL on invalid %xx (bad hex chars)
 * and str + 1 if decoded char is / or NUL.
 * In non-strict mode, it always succeeds (returns str),
 * and also it additionally decoded '+' to space.
 */
/* Sign-extends to a value which never matches fgetc result: */
/* always correctly aligned for uint64_t */
/* must be directly before hash[] */
/* 4 elements for md5, 5 for sha1, 8 for sha256 */
/* must be directly before hash[] */
/* always correctly aligned for uint64_t */
/* TLS benefits from knowing that sha1 and sha256 share these. Give them "agnostic" names too */
pub type md5sha_ctx_t = md5_ctx_t;
use crate::librb::md5_ctx_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tls_state {
  pub flags: libc::c_uint,
  pub ofd: libc::c_int,
  pub ifd: libc::c_int,
  pub min_encrypted_len_on_read: libc::c_uint,
  pub cipher_id: uint16_t,
  pub MAC_size: libc::c_uint,
  pub key_size: libc::c_uint,
  pub IV_size: libc::c_uint,
  pub outbuf: *mut uint8_t,
  pub outbuf_size: libc::c_int,
  pub inbuf_size: libc::c_int,
  pub ofs_to_buffered: libc::c_int,
  pub buffered_size: libc::c_int,
  pub inbuf: *mut uint8_t,
  pub hsd: *mut tls_handshake_data,
  pub write_seq64_be: uint64_t,
  pub client_write_key: *mut uint8_t,
  pub server_write_key: *mut uint8_t,
  pub client_write_IV: *mut uint8_t,
  pub server_write_IV: *mut uint8_t,
  pub client_write_MAC_key: [uint8_t; 32],
  pub server_write_MAC_k__: [uint8_t; 32],
  pub client_write_k__: [uint8_t; 32],
  pub server_write_k__: [uint8_t; 32],
  pub client_write_I_: [uint8_t; 4],
  pub server_write_I_: [uint8_t; 4],
  pub aes_encrypt: tls_aes,
  pub aes_decrypt: tls_aes,
  pub H: [uint8_t; 16],
}
pub type tls_state_t = tls_state;
pub const RECHDR_LEN: C2RustUnnamed = 5;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct record_hdr {
  pub type_0: uint8_t,
  pub proto_maj: uint8_t,
  pub proto_min: uint8_t,
  pub len16_hi: uint8_t,
  pub len16_lo: uint8_t,
}
pub const SHA256_OUTSIZE: C2RustUnnamed = 32;
pub const ENCRYPTION_AESGCM: C2RustUnnamed = 16;
pub const MAX_INBUF: C2RustUnnamed = 18437;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct finished {
  pub type_0: uint8_t,
  pub len24_hi: uint8_t,
  pub len24_mid: uint8_t,
  pub len24_lo: uint8_t,
  pub prf_result: [uint8_t; 12],
}
pub const OUTBUF_PFX: C2RustUnnamed = 24;
pub const OUTBUF_SFX: C2RustUnnamed = 48;
// RFC 2104:
// HMAC(key, text) based on a hash H (say, sha256) is:
// ipad = [0x36 x INSIZE]
// opad = [0x5c x INSIZE]
// HMAC(key, text) = H((key XOR opad) + H((key XOR ipad) + text))
//
// H(key XOR opad) and H(key XOR ipad) can be precomputed
// if we often need HMAC hmac with the same key.
//
// text is often given in disjoint pieces.
pub type hmac_precomputed_t = hmac_precomputed;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hmac_precomputed {
  pub hashed_key_xor_ipad: md5sha_ctx_t,
  pub hashed_key_xor_opad: md5sha_ctx_t,
}
use crate::librb::sha1_ctx_t;
pub const SHA_INSIZE: C2RustUnnamed = 64;
use crate::librb::sha256_ctx_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct handshake_hdr {
  pub type_0: uint8_t,
  pub len24_hi: uint8_t,
  pub len24_mid: uint8_t,
  pub len24_lo: uint8_t,
}
pub const ENCRYPT_ON_WRITE: C2RustUnnamed = 32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct client_key_exchange {
  pub type_0: uint8_t,
  pub len24_hi: uint8_t,
  pub len24_mid: uint8_t,
  pub len24_lo: uint8_t,
  pub key: [uint8_t; 4098],
  // size??
}
pub const GOT_EC_KEY: C2RustUnnamed = 8;
pub const GOT_CERT_RSA_KEY_ALG: C2RustUnnamed = 2;
pub const NEED_EC_KEY: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct client_empty_cert {
  pub type_0: uint8_t,
  pub len24_hi: uint8_t,
  pub len24_mid: uint8_t,
  pub len24_lo: uint8_t,
  pub cert_chain_len24_hi: uint8_t,
  pub cert_chain_len24_mid: uint8_t,
  pub cert_chain_len24_lo: uint8_t,
}
pub const SHA1_OUTSIZE: C2RustUnnamed = 20;
pub const AES128_KEYSIZE: C2RustUnnamed = 16;
pub const AES256_KEYSIZE: C2RustUnnamed = 32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct server_hello {
  pub xhdr: record_hdr,
  pub type_0: uint8_t,
  pub len24_hi: uint8_t,
  pub len24_mid: uint8_t,
  pub len24_lo: uint8_t,
  pub proto_maj: uint8_t,
  pub proto_min: uint8_t,
  pub rand32: [uint8_t; 32],
  pub session_id_len: uint8_t,
  pub session_id: [uint8_t; 32],
  pub cipherid_hi: uint8_t,
  pub cipherid_lo: uint8_t,
  pub comprtype: uint8_t,
  /* extensions may follow, but only those which client offered in its Hello */
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct client_hello {
  pub type_0: uint8_t,
  pub len24_hi: uint8_t,
  pub len24_mid: uint8_t,
  pub len24_lo: uint8_t,
  pub proto_maj: uint8_t,
  pub proto_min: uint8_t,
  pub rand32: [uint8_t; 32],
  pub session_id_len: uint8_t,
  pub cipherid_len16_hi: uint8_t,
  pub cipherid_len16_lo: uint8_t,
  pub cipherid: [uint8_t; 16],
  pub comprtypes_len: uint8_t,
  pub comprtypes: [uint8_t; 1],
  /* actually variable */
  /* Extensions (SNI shown):
   * hi,lo // len of all extensions
   *   00,00 // extension_type: "Server Name"
   *   00,0e // list len (there can be more than one SNI)
   *     00,0c // len of 1st Server Name Indication
   *       00    // name type: host_name
   *       00,09   // name len
   *       "localhost" // name
   */
  // GNU Wget 1.18 to cdn.kernel.org sends these extensions:
  // 0055
  //   0005 0005 0100000000 - status_request
  //   0000 0013 0011 00 000e 63646e 2e 6b65726e656c 2e 6f7267 - server_name
  //   ff01 0001 00 - renegotiation_info
  //   0023 0000 - session_ticket
  //   000a 0008 0006001700180019 - supported_groups
  //   000b 0002 0100 - ec_point_formats
  //   000d 0016 0014 0401 0403 0501 0503 0601 0603 0301 0303 0201 0203 - signature_algorithms
  // wolfssl library sends this option, RFC 7627 (closes a security weakness, some servers may require it. TODO?):
  //   0017 0000 - extended master secret
}
pub type C2RustUnnamed = libc::c_uint;
pub const GOT_CERT_ECDSA_KEY_ALG: C2RustUnnamed = 4;
pub const RSA_PREMASTER_SIZE: C2RustUnnamed = 48;
#[inline(always)]
unsafe extern "C" fn psRsaKey_clear(mut key: *mut psRsaKey_t) {
  pstm_clear(&mut (*key).N);
  pstm_clear(&mut (*key).e);
  pstm_clear(&mut (*key).d);
  pstm_clear(&mut (*key).p);
  pstm_clear(&mut (*key).q);
  pstm_clear(&mut (*key).dP);
  pstm_clear(&mut (*key).dQ);
  pstm_clear(&mut (*key).qP);
}
/* HANDSHAKE HASH: */
//unsigned saved_client_hello_size;
//uint8_t saved_client_hello[1];
unsafe extern "C" fn get24be(mut p: *const uint8_t) -> libc::c_uint {
  return (0x100i32 * (0x100i32 * *p.offset(0) as libc::c_int + *p.offset(1) as libc::c_int)
    + *p.offset(2) as libc::c_int) as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn tls_get_random(mut buf: *mut libc::c_void, mut len: libc::c_uint) {
  if len as libc::c_long
    != open_read_close(
      b"/dev/urandom\x00" as *const u8 as *const libc::c_char,
      buf,
      len as size_t,
    )
  {
    xfunc_die();
  };
}
unsafe extern "C" fn xorbuf3(
  mut dst: *mut libc::c_void,
  mut src1: *const libc::c_void,
  mut src2: *const libc::c_void,
  mut count: libc::c_uint,
) {
  let mut d: *mut uint8_t = dst as *mut uint8_t;
  let mut s1: *const uint8_t = src1 as *const uint8_t;
  let mut s2: *const uint8_t = src2 as *const uint8_t;
  loop {
    let fresh0 = count;
    count = count.wrapping_sub(1);
    if !(fresh0 != 0) {
      break;
    }
    let fresh1 = s1;
    s1 = s1.offset(1);
    let fresh2 = s2;
    s2 = s2.offset(1);
    let fresh3 = d;
    d = d.offset(1);
    *fresh3 = (*fresh1 as libc::c_int ^ *fresh2 as libc::c_int) as uint8_t
  }
}
#[no_mangle]
pub unsafe extern "C" fn xorbuf(
  mut dst: *mut libc::c_void,
  mut src: *const libc::c_void,
  mut count: libc::c_uint,
) {
  xorbuf3(dst, dst, src, count);
}
/*
 * Copyright (C) 2017 Denys Vlasenko
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
/* Interface glue between bbox code and minimally tweaked matrixssl
 * code. All C files (matrixssl and bbox (ones which need TLS))
 * include this file, and guaranteed to see a consistent API,
 * defines, types, etc.
 */
/* Config tweaks */
/* pstm: multiprecision numbers */
//#if defined(__GNUC__) && defined(__x86_64__)
//  /* PSTM_X86_64 works correctly, but +782 bytes. */
//  /* Looks like most of the growth is because of PSTM_64BIT. */
//# define PSTM_64BIT
//# define PSTM_X86_64
//#endif
//#if SOME_COND #define PSTM_MIPS, #define PSTM_32BIT
//#if SOME_COND #define PSTM_ARM,  #define PSTM_32BIT
/* Failure due to bad function param */
/* Failure as a result of system call error */
/* Failure to allocate requested memory */
/* Failure on sanity/limit tests */
//typedef char psPool_t;
//#ifdef PS_PUBKEY_OPTIMIZE_FOR_SMALLER_RAM
//#ifdef PS_PUBKEY_OPTIMIZE_FOR_FASTER_SPEED
//#define PS_EXPTMOD_WINSIZE 5
#[no_mangle]
pub unsafe extern "C" fn xorbuf_aligned_AES_BLOCK_SIZE(
  mut dst: *mut libc::c_void,
  mut src: *const libc::c_void,
) {
  let mut d: *mut libc::c_ulong = dst as *mut libc::c_ulong;
  let mut s: *const libc::c_ulong = src as *const libc::c_ulong;
  *d.offset(0) ^= *s.offset(0);
  *d.offset(1) ^= *s.offset(1);
}
unsafe extern "C" fn hash_handshake(
  mut tls: *mut tls_state_t,
  mut buffer: *const libc::c_void,
  mut len: libc::c_uint,
) {
  md5_hash(&mut (*(*tls).hsd).handshake_hash_ctx, buffer, len as size_t);
}
unsafe extern "C" fn hmac_begin(
  mut pre: *mut hmac_precomputed_t,
  mut key: *mut uint8_t,
  mut key_size: libc::c_uint,
) {
  let mut key_xor_ipad: [uint8_t; 64] = [0; 64];
  let mut key_xor_opad: [uint8_t; 64] = [0; 64];
  //	uint8_t tempkey[SHA1_OUTSIZE < SHA256_OUTSIZE ? SHA256_OUTSIZE : SHA1_OUTSIZE];
  let mut i: libc::c_uint = 0;
  // "The authentication key can be of any length up to INSIZE, the
  // block length of the hash function.  Applications that use keys longer
  // than INSIZE bytes will first hash the key using H and then use the
  // resultant OUTSIZE byte string as the actual key to HMAC."
  if key_size > SHA_INSIZE as libc::c_int as libc::c_uint {
    bb_simple_error_msg_and_die(b"HMAC key>64\x00" as *const u8 as *const libc::c_char);
    //does not happen (yet?)
    //		md5sha_ctx_t ctx;
    //		begin(&ctx);
    //		md5sha_hash(&ctx, key, key_size);
    //		key_size = sha_end(&ctx, tempkey);
    //		//key = tempkey; - right? RIGHT? why does it work without this?
    //		// because SHA_INSIZE is 64, but hmac() is always called with
    //		// key_size = tls->MAC_size = SHA1/256_OUTSIZE (20 or 32),
    //		// and prf_hmac_sha256() -> hmac_sha256() key sizes are:
    //		// - RSA_PREMASTER_SIZE is 48
    //		// - CURVE25519_KEYSIZE is 32
    //		// - master_secret[] is 48
  }
  i = 0i32 as libc::c_uint;
  while i < key_size {
    key_xor_ipad[i as usize] = (*key.offset(i as isize) as libc::c_int ^ 0x36i32) as uint8_t;
    key_xor_opad[i as usize] = (*key.offset(i as isize) as libc::c_int ^ 0x5ci32) as uint8_t;
    i = i.wrapping_add(1)
  }
  while i < SHA_INSIZE as libc::c_int as libc::c_uint {
    key_xor_ipad[i as usize] = 0x36i32 as uint8_t;
    key_xor_opad[i as usize] = 0x5ci32 as uint8_t;
    i = i.wrapping_add(1)
  }
  sha256_begin(&mut (*pre).hashed_key_xor_ipad);
  sha256_begin(&mut (*pre).hashed_key_xor_opad);
  md5_hash(
    &mut (*pre).hashed_key_xor_ipad,
    key_xor_ipad.as_mut_ptr() as *const libc::c_void,
    SHA_INSIZE as libc::c_int as size_t,
  );
  md5_hash(
    &mut (*pre).hashed_key_xor_opad,
    key_xor_opad.as_mut_ptr() as *const libc::c_void,
    SHA_INSIZE as libc::c_int as size_t,
  );
}
unsafe extern "C" fn hmac_sha_precomputed_v(
  mut pre: *mut hmac_precomputed_t,
  mut out: *mut uint8_t,
  mut va: ::std::ffi::VaList,
) -> libc::c_uint {
  let mut text: *mut uint8_t = 0 as *mut uint8_t;
  let mut len: libc::c_uint = 0;
  loop
  /* pre->hashed_key_xor_ipad contains unclosed "H((key XOR ipad) +" state */
  /* pre->hashed_key_xor_opad contains unclosed "H((key XOR opad) +" state */
  /* calculate out = H((key XOR ipad) + text) */
  {
    text = va.arg::<*mut uint8_t>();
    if text.is_null() {
      break;
    }
    let mut text_size: libc::c_uint = va.arg::<libc::c_uint>();
    md5_hash(
      &mut (*pre).hashed_key_xor_ipad,
      text as *const libc::c_void,
      text_size as size_t,
    );
  }
  len = sha1_end(&mut (*pre).hashed_key_xor_ipad, out as *mut libc::c_void);
  /* out = H((key XOR opad) + out) */
  md5_hash(
    &mut (*pre).hashed_key_xor_opad,
    out as *const libc::c_void,
    len as size_t,
  ); /* struct copy */
  return sha1_end(&mut (*pre).hashed_key_xor_opad, out as *mut libc::c_void);
}
unsafe extern "C" fn hmac_sha_precomputed(
  mut pre_init: *mut hmac_precomputed_t,
  mut out: *mut uint8_t,
  mut args: ...
) -> libc::c_uint {
  let mut pre: hmac_precomputed_t = hmac_precomputed_t {
    hashed_key_xor_ipad: md5sha_ctx_t {
      wbuffer: [0; 64],
      process_block: None,
      total64: 0,
      hash: [0; 8],
    },
    hashed_key_xor_opad: md5sha_ctx_t {
      wbuffer: [0; 64],
      process_block: None,
      total64: 0,
      hash: [0; 8],
    },
  };
  let mut va: ::std::ffi::VaListImpl;
  let mut len: libc::c_uint = 0;
  va = args.clone();
  pre = *pre_init;
  len = hmac_sha_precomputed_v(&mut pre, out, va.as_va_list());
  return len;
}
unsafe extern "C" fn hmac(
  mut out: *mut uint8_t,
  mut key: *mut uint8_t,
  mut key_size: libc::c_uint,
  mut args: ...
) -> libc::c_uint {
  let mut pre: hmac_precomputed_t = hmac_precomputed_t {
    hashed_key_xor_ipad: md5sha_ctx_t {
      wbuffer: [0; 64],
      process_block: None,
      total64: 0,
      hash: [0; 8],
    },
    hashed_key_xor_opad: md5sha_ctx_t {
      wbuffer: [0; 64],
      process_block: None,
      total64: 0,
      hash: [0; 8],
    },
  };
  let mut va: ::std::ffi::VaListImpl;
  let mut len: libc::c_uint = 0;
  va = args.clone();
  hmac_begin(&mut pre, key, key_size);
  len = hmac_sha_precomputed_v(&mut pre, out, va.as_va_list());
  return len;
}
// RFC 5246:
// 5.  HMAC and the Pseudorandom Function
//...
// In this section, we define one PRF, based on HMAC.  This PRF with the
// SHA-256 hash function is used for all cipher suites defined in this
// document and in TLS documents published prior to this document when
// TLS 1.2 is negotiated.
// ^^^^^^^^^^^^^ IMPORTANT!
//               PRF uses sha256 regardless of cipher for all ciphers
//               defined by RFC 5246. It's not sha1 for AES_128_CBC_SHA!
//               However, for _SHA384 ciphers, it's sha384. See RFC 5288,5289.
//...
//    P_hash(secret, seed) = HMAC_hash(secret, A(1) + seed) +
//                           HMAC_hash(secret, A(2) + seed) +
//                           HMAC_hash(secret, A(3) + seed) + ...
// where + indicates concatenation.
// A() is defined as:
//    A(0) = seed
//    A(1) = HMAC_hash(secret, A(0)) = HMAC_hash(secret, seed)
//    A(i) = HMAC_hash(secret, A(i-1))
// P_hash can be iterated as many times as necessary to produce the
// required quantity of data.  For example, if P_SHA256 is being used to
// create 80 bytes of data, it will have to be iterated three times
// (through A(3)), creating 96 bytes of output data; the last 16 bytes
// of the final iteration will then be discarded, leaving 80 bytes of
// output data.
//
// TLS's PRF is created by applying P_hash to the secret as:
//
//    PRF(secret, label, seed) = P_<hash>(secret, label + seed)
//
// The label is an ASCII string.
//
// RFC 5288:
// For cipher suites ending with _SHA256, the PRF is the TLS PRF
// with SHA-256 as the hash function.
// For cipher suites ending with _SHA384, the PRF is the TLS PRF
// with SHA-384 as the hash function.
unsafe extern "C" fn prf_hmac_sha256(
  mut outbuf: *mut uint8_t,
  mut outbuf_size: libc::c_uint,
  mut secret: *mut uint8_t,
  mut secret_size: libc::c_uint,
  mut label: *const libc::c_char,
  mut seed: *mut uint8_t,
  mut seed_size: libc::c_uint,
) {
  let mut pre: hmac_precomputed_t = hmac_precomputed_t {
    hashed_key_xor_ipad: md5sha_ctx_t {
      wbuffer: [0; 64],
      process_block: None,
      total64: 0,
      hash: [0; 8],
    },
    hashed_key_xor_opad: md5sha_ctx_t {
      wbuffer: [0; 64],
      process_block: None,
      total64: 0,
      hash: [0; 8],
    },
  };
  let mut a: [uint8_t; 32] = [0; 32];
  let mut out_p: *mut uint8_t = outbuf;
  let mut label_size: libc::c_uint = strlen(label) as libc::c_uint;
  let mut MAC_size: libc::c_uint = SHA256_OUTSIZE as libc::c_int as libc::c_uint;
  /* In P_hash() calculation, "seed" is "label + seed": */
  hmac_begin(&mut pre, secret, secret_size);
  /* A(1) = HMAC_hash(secret, seed) */
  hmac_sha_precomputed(
    &mut pre as *mut hmac_precomputed_t,
    a.as_mut_ptr(),
    label,
    label_size,
    seed,
    seed_size,
    0 as *mut libc::c_void,
  );
  loop {
    /* HMAC_hash(secret, A(1) + seed) */
    if outbuf_size <= MAC_size {
      /* Last, possibly incomplete, block */
      /* (use a[] as temp buffer) */
      hmac_sha_precomputed(
        &mut pre as *mut hmac_precomputed_t,
        a.as_mut_ptr(),
        a.as_mut_ptr(),
        MAC_size,
        label,
        label_size,
        seed,
        seed_size,
        0 as *mut libc::c_void,
      );
      memcpy(
        out_p as *mut libc::c_void,
        a.as_mut_ptr() as *const libc::c_void,
        outbuf_size as libc::c_ulong,
      );
      return;
    }
    /* Not last block. Store directly to result buffer */
    hmac_sha_precomputed(
      &mut pre as *mut hmac_precomputed_t,
      out_p,
      a.as_mut_ptr(),
      MAC_size,
      label,
      label_size,
      seed,
      seed_size,
      0 as *mut libc::c_void,
    );
    out_p = out_p.offset(MAC_size as isize);
    outbuf_size = outbuf_size.wrapping_sub(MAC_size);
    /* A(2) = HMAC_hash(secret, A(1)) */
    hmac_sha_precomputed(
      &mut pre as *mut hmac_precomputed_t,
      a.as_mut_ptr(),
      a.as_mut_ptr(),
      MAC_size,
      0 as *mut libc::c_void,
    ); /* don't flood, a few lines should be enough */
  }
}
unsafe extern "C" fn bad_record_die(
  mut tls: *mut tls_state_t,
  mut expected: *const libc::c_char,
  mut len: libc::c_int,
) {
  bb_error_msg(
    b"got bad TLS record (len:%d) while expecting %s\x00" as *const u8 as *const libc::c_char,
    len,
    expected,
  );
  if len > 0i32 {
    let mut p: *mut uint8_t = (*tls).inbuf;
    if len > 99i32 {
      len = 99i32
    }
    loop {
      let fresh4 = p;
      p = p.offset(1);
      fprintf(
        stderr,
        b" %02x\x00" as *const u8 as *const libc::c_char,
        *fresh4 as libc::c_int,
      );
      len -= 1;
      if !(len != 0i32) {
        break;
      }
    }
    putc_unlocked('\n' as i32, stderr);
  }
  xfunc_die();
}
unsafe extern "C" fn tls_error_die(mut tls: *mut tls_state_t, mut line: libc::c_int) {
  bb_error_msg_and_die(
    b"tls error at line %d cipher:%04x\x00" as *const u8 as *const libc::c_char,
    line,
    (*tls).cipher_id as libc::c_int,
  );
}
//UNUSED
unsafe extern "C" fn tls_free_outbuf(mut tls: *mut tls_state_t) {
  free((*tls).outbuf as *mut libc::c_void);
  (*tls).outbuf_size = 0i32;
  (*tls).outbuf = 0 as *mut uint8_t;
}
unsafe extern "C" fn tls_get_outbuf(
  mut tls: *mut tls_state_t,
  mut len: libc::c_int,
) -> *mut libc::c_void {
  if len > 1i32 << 14i32 {
    xfunc_die();
  }
  len += OUTBUF_PFX as libc::c_int + OUTBUF_SFX as libc::c_int;
  if (*tls).outbuf_size < len {
    (*tls).outbuf_size = len;
    (*tls).outbuf = xrealloc((*tls).outbuf as *mut libc::c_void, len as size_t) as *mut uint8_t
  }
  return (*tls).outbuf.offset(OUTBUF_PFX as libc::c_int as isize) as *mut libc::c_void;
}
unsafe extern "C" fn tls_get_zeroed_outbuf(
  mut tls: *mut tls_state_t,
  mut len: libc::c_int,
) -> *mut libc::c_void {
  let mut record: *mut libc::c_void = tls_get_outbuf(tls, len);
  memset(record, 0i32, len as libc::c_ulong);
  return record;
}
unsafe extern "C" fn xwrite_encrypted_and_hmac_signed(
  mut tls: *mut tls_state_t,
  mut size: libc::c_uint,
  mut type_0: libc::c_uint,
) {
  let mut buf: *mut uint8_t = (*tls).outbuf.offset(OUTBUF_PFX as libc::c_int as isize);
  let mut xhdr: *mut record_hdr = 0 as *mut record_hdr;
  let mut padding_length: uint8_t = 0;
  xhdr = buf.offset(-(RECHDR_LEN as libc::c_int as isize)) as *mut libc::c_void as *mut record_hdr;
  if 0i32 == 0 || (*tls).cipher_id as libc::c_int != 0x3bi32 {
    /* or if it wasn't selected */
    xhdr = buf
      .offset(-(RECHDR_LEN as libc::c_int as isize))
      .offset(-16) as *mut libc::c_void as *mut record_hdr
    /* place for IV */
  }
  (*xhdr).type_0 = type_0 as uint8_t;
  (*xhdr).proto_maj = 3i32 as uint8_t;
  (*xhdr).proto_min = 3i32 as uint8_t;
  /* fake unencrypted record len for MAC calculation */
  (*xhdr).len16_hi = (size >> 8i32) as uint8_t;
  (*xhdr).len16_lo = (size & 0xffi32 as libc::c_uint) as uint8_t;
  /* Calculate MAC signature */
  hmac(
    buf.offset(size as isize),
    (*tls).client_write_MAC_key.as_mut_ptr(),
    SHA256_OUTSIZE as libc::c_int as libc::c_uint,
    &mut (*tls).write_seq64_be as *mut uint64_t,
    ::std::mem::size_of::<uint64_t>() as libc::c_ulong,
    xhdr,
    RECHDR_LEN as libc::c_int,
    buf,
    size,
    0 as *mut libc::c_void,
  );
  (*tls).write_seq64_be = {
    let mut __v: __uint64_t = 0;
    let mut __x: __uint64_t =
                 (1i32 as
                      libc::c_ulong).wrapping_add({
                                                       let mut __v_0:
                                                               __uint64_t = 0;
                                                       let mut __x_0:
                                                               __uint64_t =
                                                           (*tls).write_seq64_be;
                                                       if 0 != 0 {
                                                           __v_0 =
                                                               ((__x_0 as
                                                                     libc::c_ulonglong
                                                                     &
                                                                     0xff00000000000000u64)
                                                                    >> 56i32 |
                                                                    (__x_0 as
                                                                         libc::c_ulonglong
                                                                         &
                                                                         0xff000000000000u64)
                                                                        >>
                                                                        40i32
                                                                    |
                                                                    (__x_0 as
                                                                         libc::c_ulonglong
                                                                         &
                                                                         0xff0000000000u64)
                                                                        >>
                                                                        24i32
                                                                    |
                                                                    (__x_0 as
                                                                         libc::c_ulonglong
                                                                         &
                                                                         0xff00000000u64)
                                                                        >>
                                                                        8i32 |
                                                                    (__x_0 as
                                                                         libc::c_ulonglong
                                                                         &
                                                                         0xff000000u64)
                                                                        <<
                                                                        8i32 |
                                                                    (__x_0 as
                                                                         libc::c_ulonglong
                                                                         &
                                                                         0xff0000u64)
                                                                        <<
                                                                        24i32
                                                                    |
                                                                    (__x_0 as
                                                                         libc::c_ulonglong
                                                                         &
                                                                         0xff00u64)
                                                                        <<
                                                                        40i32
                                                                    |
                                                                    (__x_0 as
                                                                         libc::c_ulonglong
                                                                         &
                                                                         0xffu64)
                                                                        <<
                                                                        56i32)
                                                                   as
                                                                   __uint64_t
                                                       } else {
                                                           let fresh5 =
                                                               &mut __v_0;
                                                           let fresh6;
                                                           let fresh7 = __x_0;
                                                           asm!("bswap ${0:q}"
                                                                : "=r"
                                                                (fresh6) : "0"
                                                                (c2rust_asm_casts::AsmCast::cast_in(fresh5, fresh7))
                                                                :);
                                                           c2rust_asm_casts::AsmCast::cast_out(fresh5,
                                                                                               fresh7,
                                                                                               fresh6);
                                                       }
                                                       __v_0
                                                   });
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
      let fresh8 = &mut __v;
      let fresh9;
      let fresh10 = __x;
      asm!("bswap ${0:q}" : "=r" (fresh9) : "0"
                      (c2rust_asm_casts::AsmCast::cast_in(fresh8, fresh10))
                      :);
      c2rust_asm_casts::AsmCast::cast_out(fresh8, fresh10, fresh9);
    }
    __v
  };
  size = size.wrapping_add(SHA256_OUTSIZE as libc::c_int as libc::c_uint);
  // RFC 5246:
  // 6.2.3.1.  Null or Standard Stream Cipher
  //
  // Stream ciphers (including BulkCipherAlgorithm.null; see Appendix A.6)
  // convert TLSCompressed.fragment structures to and from stream
  // TLSCiphertext.fragment structures.
  //
  //    stream-ciphered struct {
  //        opaque content[TLSCompressed.length];
  //        opaque MAC[SecurityParameters.mac_length];
  //    } GenericStreamCipher;
  //
  // The MAC is generated as:
  //    MAC(MAC_write_key, seq_num +
  //                          TLSCompressed.type +
  //                          TLSCompressed.version +
  //                          TLSCompressed.length +
  //                          TLSCompressed.fragment);
  // where "+" denotes concatenation.
  // seq_num
  //    The sequence number for this record.
  // MAC
  //    The MAC algorithm specified by SecurityParameters.mac_algorithm.
  //
  // Note that the MAC is computed before encryption.  The stream cipher
  // encrypts the entire block, including the MAC.
  //...
  // Appendix C.  Cipher Suite Definitions
  //...
  // MAC       Algorithm    mac_length  mac_key_length
  // --------  -----------  ----------  --------------
  // SHA       HMAC-SHA1       20            20
  // SHA256    HMAC-SHA256     32            32
  if 0i32 != 0 && (*tls).cipher_id as libc::c_int == 0x3bi32 {
    /* No encryption, only signing */
    (*xhdr).len16_hi = (size >> 8i32) as uint8_t;
    (*xhdr).len16_lo = (size & 0xffi32 as libc::c_uint) as uint8_t;
    xwrite(
      (*tls).ofd,
      xhdr as *const libc::c_void,
      (RECHDR_LEN as libc::c_int as libc::c_uint).wrapping_add(size) as size_t,
    );
    return;
  }
  // 6.2.3.2.  CBC Block Cipher
  // For block ciphers (such as 3DES or AES), the encryption and MAC
  // functions convert TLSCompressed.fragment structures to and from block
  // TLSCiphertext.fragment structures.
  //    struct {
  //        opaque IV[SecurityParameters.record_iv_length];
  //        block-ciphered struct {
  //            opaque content[TLSCompressed.length];
  //            opaque MAC[SecurityParameters.mac_length];
  //            uint8 padding[GenericBlockCipher.padding_length];
  //            uint8 padding_length;
  //        };
  //    } GenericBlockCipher;
  //...
  // IV
  //    The Initialization Vector (IV) SHOULD be chosen at random, and
  //    MUST be unpredictable.  Note that in versions of TLS prior to 1.1,
  //    there was no IV field (...).  For block ciphers, the IV length is
  //    of length SecurityParameters.record_iv_length, which is equal to the
  //    SecurityParameters.block_size.
  // padding
  //    Padding that is added to force the length of the plaintext to be
  //    an integral multiple of the block cipher's block length.
  // padding_length
  //    The padding length MUST be such that the total size of the
  //    GenericBlockCipher structure is a multiple of the cipher's block
  //    length.  Legal values range from zero to 255, inclusive.
  //...
  // Appendix C.  Cipher Suite Definitions
  //...
  //                         Key      IV   Block
  // Cipher        Type    Material  Size  Size
  // ------------  ------  --------  ----  -----
  // AES_128_CBC   Block      16      16     16
  // AES_256_CBC   Block      32      16     16
  tls_get_random(buf.offset(-16) as *mut libc::c_void, 16i32 as libc::c_uint); /* IV */
  /* Fill IV and padding in outbuf */
  // RFC is talking nonsense:
  //    "Padding that is added to force the length of the plaintext to be
  //    an integral multiple of the block cipher's block length."
  // WRONG. _padding+padding_length_, not just _padding_,
  // pads the data.
  // IOW: padding_length is the last byte of padding[] array,
  // contrary to what RFC depicts.
  //
  // What actually happens is that there is always padding.
  // If you need one byte to reach BLOCKSIZE, this byte is 0x00.
  // If you need two bytes, they are both 0x01.
  // If you need three, they are 0x02,0x02,0x02. And so on.
  // If you need no bytes to reach BLOCKSIZE, you have to pad a full
  // BLOCKSIZE with bytes of value (BLOCKSIZE-1).
  // It's ok to have more than minimum padding, but we do minimum.
  padding_length = (!size & (16i32 - 1i32) as libc::c_uint) as uint8_t;
  loop {
    let fresh11 = size;
    size = size.wrapping_add(1);
    *buf.offset(fresh11 as isize) = padding_length;
    if !(size & (16i32 - 1i32) as libc::c_uint != 0i32 as libc::c_uint) {
      break;
    }
    /* padding */
  }
  /* Encrypt content+MAC+padding in place */
  aes_cbc_encrypt(
    &mut (*tls).aes_encrypt,
    buf.offset(-16) as *mut libc::c_void,
    buf as *const libc::c_void,
    size as size_t,
    buf as *mut libc::c_void,
  );
  /* Write out */
  size = size.wrapping_add(16i32 as libc::c_uint); /* + IV */
  (*xhdr).len16_hi = (size >> 8i32) as uint8_t;
  (*xhdr).len16_lo = (size & 0xffi32 as libc::c_uint) as uint8_t;
  xwrite(
    (*tls).ofd,
    xhdr as *const libc::c_void,
    (RECHDR_LEN as libc::c_int as libc::c_uint).wrapping_add(size) as size_t,
  );
}
/* Example how GCM encryption combines nonce, aad, input and generates
 * "header | exp_nonce | encrypted output | tag":
 * nonce:0d 6a 26 31 00 00 00 00 00 00 00 01 (implicit 4 bytes (derived from master secret), then explicit 8 bytes)
 * aad:  00 00 00 00 00 00 00 01 17 03 03 00 1c
 * in:   47 45 54 20 2f 69 6e 64 65 78 2e 68 74 6d 6c 20 48 54 54 50 2f 31 2e 30 0d 0a 0d 0a "GET /index.html HTTP/1.0\r\n\r\n" (0x1c bytes)
 * out:  f7 8a b2 8f 78 0e f6 d5 76 17 2e b5 6d 46 59 56 8b 46 9f 0b d9 2c 35 28 13 66 19 be
 * tag:  c2 86 ce 4a 50 4a d0 aa 50 b3 76 5c 49 2a 3f 33
 * sent: 17 03 03 00 34|00 00 00 00 00 00 00 01|f7 8a b2 8f 78 0e f6 d5 76 17 2e b5 6d 46 59 56 8b 46 9f 0b d9 2c 35 28 13 66 19 be|c2 86 ce 4a 50 4a d0 aa 50 b3 76 5c 49 2a 3f 33
 * .............................................^^ buf points here
 */
unsafe extern "C" fn xwrite_encrypted_aesgcm(
  mut tls: *mut tls_state_t,
  mut size: libc::c_uint,
  mut type_0: libc::c_uint,
) {
  let mut aad: [uint8_t; 16] = [0; 16]; /* +3 creates [16] buffer, simplifying GHASH() */
  let mut nonce: [uint8_t; 16] = [0; 16]; /* +4 creates space for AES block counter */
  let mut scratch: [uint8_t; 16] = [0; 16]; //[16]
  let mut authtag: [uint8_t; 16] = [0; 16]; //[16]
  let mut buf: *mut uint8_t = 0 as *mut uint8_t; /* see above for the byte it points to */
  let mut xhdr: *mut record_hdr = 0 as *mut record_hdr; /* do it here so that "type" param no longer used */
  let mut remaining: libc::c_uint = 0;
  let mut cnt: libc::c_uint = 0;
  let mut t64: uint64_t = 0;
  buf = (*tls).outbuf.offset(OUTBUF_PFX as libc::c_int as isize);
  xhdr = buf.offset(-8).offset(-(RECHDR_LEN as libc::c_int as isize)) as *mut libc::c_void
    as *mut record_hdr;
  (*xhdr).type_0 = type_0 as uint8_t;
  aad[8] = type_0 as uint8_t;
  aad[9] = 3i32 as uint8_t;
  aad[10] = 3i32 as uint8_t;
  aad[11] = (size >> 8i32) as uint8_t;
  /* set aad[12], and clear aad[13..15] */
  *(aad.as_mut_ptr().offset(12) as *mut uint32_t) = size & 0xffi32 as libc::c_uint;
  memcpy(
    nonce.as_mut_ptr() as *mut libc::c_void,
    (*tls).client_write_IV as *const libc::c_void,
    4i32 as libc::c_ulong,
  );
  t64 = (*tls).write_seq64_be;
  *(nonce.as_mut_ptr().offset(4) as *mut bb__aliased_uint64_t) = t64;
  *(aad.as_mut_ptr() as *mut bb__aliased_uint64_t) = t64;
  *(buf.offset(-8) as *mut bb__aliased_uint64_t) = t64;
  /* seq64 is not used later in this func, can increment here */
  (*tls).write_seq64_be = {
    let mut __v: __uint64_t = 0; /* yes, first cnt here is 2 (!) */
    let mut __x: __uint64_t =
                 (1i32 as
                      libc::c_ulong).wrapping_add({
                                                       let mut __v_0:
                                                               __uint64_t = 0;
                                                       let mut __x_0:
                                                               __uint64_t =
                                                           t64;
                                                       if 0 != 0 {
                                                           __v_0 =
                                                               ((__x_0 as
                                                                     libc::c_ulonglong
                                                                     &
                                                                     0xff00000000000000u64)
                                                                    >> 56i32 |
                                                                    (__x_0 as
                                                                         libc::c_ulonglong
                                                                         &
                                                                         0xff000000000000u64)
                                                                        >>
                                                                        40i32
                                                                    |
                                                                    (__x_0 as
                                                                         libc::c_ulonglong
                                                                         &
                                                                         0xff0000000000u64)
                                                                        >>
                                                                        24i32
                                                                    |
                                                                    (__x_0 as
                                                                         libc::c_ulonglong
                                                                         &
                                                                         0xff00000000u64)
                                                                        >>
                                                                        8i32 |
                                                                    (__x_0 as
                                                                         libc::c_ulonglong
                                                                         &
                                                                         0xff000000u64)
                                                                        <<
                                                                        8i32 |
                                                                    (__x_0 as
                                                                         libc::c_ulonglong
                                                                         &
                                                                         0xff0000u64)
                                                                        <<
                                                                        24i32
                                                                    |
                                                                    (__x_0 as
                                                                         libc::c_ulonglong
                                                                         &
                                                                         0xff00u64)
                                                                        <<
                                                                        40i32
                                                                    |
                                                                    (__x_0 as
                                                                         libc::c_ulonglong
                                                                         &
                                                                         0xffu64)
                                                                        <<
                                                                        56i32)
                                                                   as
                                                                   __uint64_t
                                                       } else {
                                                           let fresh12 =
                                                               &mut __v_0;
                                                           let fresh13;
                                                           let fresh14 =
                                                               __x_0;
                                                           asm!("bswap ${0:q}"
                                                                : "=r"
                                                                (fresh13) :
                                                                "0"
                                                                (c2rust_asm_casts::AsmCast::cast_in(fresh12, fresh14))
                                                                :);
                                                           c2rust_asm_casts::AsmCast::cast_out(fresh12,
                                                                                               fresh14,
                                                                                               fresh13);
                                                       }
                                                       __v_0
                                                   });
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
      let fresh15 = &mut __v;
      let fresh16;
      let fresh17 = __x;
      asm!("bswap ${0:q}" : "=r" (fresh16) : "0"
                      (c2rust_asm_casts::AsmCast::cast_in(fresh15, fresh17))
                      :);
      c2rust_asm_casts::AsmCast::cast_out(fresh15, fresh17, fresh16);
    }
    __v
  };
  cnt = 1i32 as libc::c_uint;
  remaining = size;
  while remaining != 0i32 as libc::c_uint {
    let mut n: libc::c_uint = 0;
    cnt = cnt.wrapping_add(1);
    *(nonce.as_mut_ptr().offset(12) as *mut uint32_t) = {
      let mut __v: libc::c_uint = 0;
      let mut __x: libc::c_uint = cnt;
      if 0 != 0 {
        __v = (__x & 0xff000000u32) >> 24i32
          | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
          | (__x & 0xff00i32 as libc::c_uint) << 8i32
          | (__x & 0xffi32 as libc::c_uint) << 24i32
      } else {
        let fresh18 = &mut __v;
        let fresh19;
        let fresh20 = __x;
        asm!("bswap $0" : "=r" (fresh19) : "0"
                          (c2rust_asm_casts::AsmCast::cast_in(fresh18, fresh20))
                          :);
        c2rust_asm_casts::AsmCast::cast_out(fresh18, fresh20, fresh19);
      }
      __v
    };
    aes_encrypt_one_block(
      &mut (*tls).aes_encrypt,
      nonce.as_mut_ptr() as *const libc::c_void,
      scratch.as_mut_ptr() as *mut libc::c_void,
    );
    n = if remaining > 16i32 as libc::c_uint {
      16i32 as libc::c_uint
    } else {
      remaining
    };
    xorbuf(
      buf as *mut libc::c_void,
      scratch.as_mut_ptr() as *const libc::c_void,
      n,
    );
    buf = buf.offset(n as isize);
    remaining = remaining.wrapping_sub(n)
  }
  aesgcm_GHASH(
    (*tls).H.as_mut_ptr(),
    aad.as_mut_ptr(),
    (*tls).outbuf.offset(OUTBUF_PFX as libc::c_int as isize),
    size,
    authtag.as_mut_ptr(),
  );
  *(nonce.as_mut_ptr().offset(12) as *mut uint32_t) = {
    let mut __v: libc::c_uint = 0;
    let mut __x: libc::c_uint = 1i32 as libc::c_uint;
    if 0 != 0 {
      __v = (__x & 0xff000000u32) >> 24i32
        | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
        | (__x & 0xff00i32 as libc::c_uint) << 8i32
        | (__x & 0xffi32 as libc::c_uint) << 24i32
    } else {
      let fresh21 = &mut __v;
      let fresh22;
      let fresh23 = __x;
      asm!("bswap $0" : "=r" (fresh22) : "0"
                      (c2rust_asm_casts::AsmCast::cast_in(fresh21, fresh23))
                      :);
      c2rust_asm_casts::AsmCast::cast_out(fresh21, fresh23, fresh22);
    }
    __v
  };
  aes_encrypt_one_block(
    &mut (*tls).aes_encrypt,
    nonce.as_mut_ptr() as *const libc::c_void,
    scratch.as_mut_ptr() as *mut libc::c_void,
  );
  xorbuf_aligned_AES_BLOCK_SIZE(
    authtag.as_mut_ptr() as *mut libc::c_void,
    scratch.as_mut_ptr() as *const libc::c_void,
  );
  memcpy(
    buf as *mut libc::c_void,
    authtag.as_mut_ptr() as *const libc::c_void,
    ::std::mem::size_of::<[uint8_t; 16]>() as libc::c_ulong,
  );
  /* Write out */
  xhdr = (*tls)
    .outbuf
    .offset(OUTBUF_PFX as libc::c_int as isize)
    .offset(-8)
    .offset(-(RECHDR_LEN as libc::c_int as isize)) as *mut libc::c_void as *mut record_hdr;
  size = (size as libc::c_ulong).wrapping_add(
    (8i32 as libc::c_ulong).wrapping_add(::std::mem::size_of::<[uint8_t; 16]>() as libc::c_ulong),
  ) as libc::c_uint as libc::c_uint;
  /*xhdr->type = type; - already is */
  (*xhdr).proto_maj = 3i32 as uint8_t;
  (*xhdr).proto_min = 3i32 as uint8_t;
  (*xhdr).len16_hi = (size >> 8i32) as uint8_t;
  (*xhdr).len16_lo = (size & 0xffi32 as libc::c_uint) as uint8_t;
  size = size.wrapping_add(RECHDR_LEN as libc::c_int as libc::c_uint);
  xwrite((*tls).ofd, xhdr as *const libc::c_void, size as size_t);
}
unsafe extern "C" fn xwrite_encrypted(
  mut tls: *mut tls_state_t,
  mut size: libc::c_uint,
  mut type_0: libc::c_uint,
) {
  if (*tls).flags & ENCRYPTION_AESGCM as libc::c_int as libc::c_uint == 0 {
    xwrite_encrypted_and_hmac_signed(tls, size, type_0);
    return;
  }
  xwrite_encrypted_aesgcm(tls, size, type_0);
}
unsafe extern "C" fn xwrite_handshake_record(mut tls: *mut tls_state_t, mut size: libc::c_uint) {
  let mut buf: *mut uint8_t = (*tls).outbuf.offset(OUTBUF_PFX as libc::c_int as isize);
  let mut xhdr: *mut record_hdr =
    buf.offset(-(RECHDR_LEN as libc::c_int as isize)) as *mut libc::c_void as *mut record_hdr;
  (*xhdr).type_0 = 22i32 as uint8_t;
  (*xhdr).proto_maj = 3i32 as uint8_t;
  (*xhdr).proto_min = 3i32 as uint8_t;
  (*xhdr).len16_hi = (size >> 8i32) as uint8_t;
  (*xhdr).len16_lo = (size & 0xffi32 as libc::c_uint) as uint8_t;
  xwrite(
    (*tls).ofd,
    xhdr as *const libc::c_void,
    (RECHDR_LEN as libc::c_int as libc::c_uint).wrapping_add(size) as size_t,
  );
}
unsafe extern "C" fn xwrite_and_update_handshake_hash(
  mut tls: *mut tls_state_t,
  mut size: libc::c_uint,
) {
  if (*tls).flags & ENCRYPT_ON_WRITE as libc::c_int as libc::c_uint == 0 {
    let mut buf: *mut uint8_t = 0 as *mut uint8_t;
    xwrite_handshake_record(tls, size);
    /* Handshake hash does not include record headers */
    buf = (*tls).outbuf.offset(OUTBUF_PFX as libc::c_int as isize);
    hash_handshake(tls, buf as *const libc::c_void, size);
    return;
  }
  xwrite_encrypted(tls, size, 22i32 as libc::c_uint);
}
unsafe extern "C" fn tls_has_buffered_record(mut tls: *mut tls_state_t) -> libc::c_int {
  let mut buffered: libc::c_int = (*tls).buffered_size;
  let mut xhdr: *mut record_hdr = 0 as *mut record_hdr;
  let mut rec_size: libc::c_int = 0;
  if buffered < RECHDR_LEN as libc::c_int {
    return 0i32;
  }
  xhdr =
    (*tls).inbuf.offset((*tls).ofs_to_buffered as isize) as *mut libc::c_void as *mut record_hdr;
  rec_size = RECHDR_LEN as libc::c_int
    + (0x100i32 * (*xhdr).len16_hi as libc::c_int + (*xhdr).len16_lo as libc::c_int);
  if buffered < rec_size {
    return 0i32;
  }
  return rec_size;
}
unsafe extern "C" fn alert_text(mut code: libc::c_int) -> *const libc::c_char {
  match code {
    20 => return b"bad MAC\x00" as *const u8 as *const libc::c_char,
    50 => return b"decode error\x00" as *const u8 as *const libc::c_char,
    51 => return b"decrypt error\x00" as *const u8 as *const libc::c_char,
    40 => return b"handshake failure\x00" as *const u8 as *const libc::c_char,
    112 => return b"unrecognized name\x00" as *const u8 as *const libc::c_char,
    _ => {}
  }
  return itoa(code);
}
unsafe extern "C" fn tls_aesgcm_decrypt(
  mut tls: *mut tls_state_t,
  mut buf: *mut uint8_t,
  mut size: libc::c_int,
) {
  //uint8_t aad[13 + 3] ALIGNED_long; /* +3 creates [16] buffer, simplifying GHASH() */
  let mut nonce: [uint8_t; 16] = [0; 16]; /* +4 creates space for AES block counter */
  let mut scratch: [uint8_t; 16] = [0; 16]; //[16]
                                            //uint8_t authtag[AES_BLOCK_SIZE] ALIGNED_long; //[16]
  let mut remaining: libc::c_uint = 0;
  let mut cnt: libc::c_uint = 0;
  //memcpy(aad, buf, 8);
  //aad[8] = type;
  //aad[9] = TLS_MAJ;
  //aad[10] = TLS_MIN;
  //aad[11] = size >> 8;
  // /* set aad[12], and clear aad[13..15] */
  //COUNTER(aad) = SWAP_LE32(size & 0xff);
  memcpy(
    nonce.as_mut_ptr() as *mut libc::c_void,
    (*tls).server_write_IV as *const libc::c_void,
    4i32 as libc::c_ulong,
  ); /* yes, first cnt here is 2 (!) */
  memcpy(
    nonce.as_mut_ptr().offset(4) as *mut libc::c_void,
    buf as *const libc::c_void,
    8i32 as libc::c_ulong,
  );
  cnt = 1i32 as libc::c_uint;
  remaining = size as libc::c_uint;
  while remaining != 0i32 as libc::c_uint {
    let mut n: libc::c_uint = 0;
    cnt = cnt.wrapping_add(1);
    *(nonce.as_mut_ptr().offset(12) as *mut uint32_t) = {
      let mut __v: libc::c_uint = 0;
      let mut __x: libc::c_uint = cnt;
      if 0 != 0 {
        __v = (__x & 0xff000000u32) >> 24i32
          | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
          | (__x & 0xff00i32 as libc::c_uint) << 8i32
          | (__x & 0xffi32 as libc::c_uint) << 24i32
      } else {
        let fresh24 = &mut __v;
        let fresh25;
        let fresh26 = __x;
        asm!("bswap $0" : "=r" (fresh25) : "0"
                          (c2rust_asm_casts::AsmCast::cast_in(fresh24, fresh26))
                          :);
        c2rust_asm_casts::AsmCast::cast_out(fresh24, fresh26, fresh25);
      }
      __v
    };
    aes_encrypt_one_block(
      &mut (*tls).aes_decrypt,
      nonce.as_mut_ptr() as *const libc::c_void,
      scratch.as_mut_ptr() as *mut libc::c_void,
    );
    n = if remaining > 16i32 as libc::c_uint {
      16i32 as libc::c_uint
    } else {
      remaining
    };
    xorbuf3(
      buf as *mut libc::c_void,
      scratch.as_mut_ptr() as *const libc::c_void,
      buf.offset(8) as *const libc::c_void,
      n,
    );
    buf = buf.offset(n as isize);
    remaining = remaining.wrapping_sub(n)
  }
  //aesgcm_GHASH(tls->H, aad, tls->inbuf + RECHDR_LEN, size, authtag);
  //COUNTER(nonce) = htonl(1);
  //aes_encrypt_one_block(&tls->aes_encrypt, nonce, scratch);
  //xorbuf_aligned_AES_BLOCK_SIZE(authtag, scratch);
  //memcmp(buf, authtag, sizeof(authtag)) || DIE("HASH DOES NOT MATCH!");
}
unsafe extern "C" fn tls_xread_record(
  mut tls: *mut tls_state_t,
  mut expected: *const libc::c_char,
) -> libc::c_int {
  let mut rem: libc::c_int = 0;
  let mut current_block: u64;
  let mut xhdr: *mut record_hdr = 0 as *mut record_hdr;
  let mut sz: libc::c_int = 0;
  let mut total: libc::c_int = 0;
  let mut target: libc::c_int = 0;
  's_18: loop
  //This possibly needs to be cached and shown only if
  //a fatal alert follows
  //			bb_error_msg("TLS %s from peer (alert code %d): %s",
  //				"warning",
  //				p[1], alert_text(p[1])
  //			);
  /* discard it, get next record */
  {
    total = (*tls).buffered_size;
    if total != 0i32 {
      memmove(
        (*tls).inbuf as *mut libc::c_void,
        (*tls).inbuf.offset((*tls).ofs_to_buffered as isize) as *const libc::c_void,
        total as libc::c_ulong,
      );
      //dbg("<< remaining at %d [%d] ", tls->ofs_to_buffered, total);
      //dump_raw_in("<< %s\n", tls->inbuf, total);
    }
    *bb_errno = 0i32;
    target = MAX_INBUF as libc::c_int;
    loop {
      rem = 0;
      if total >= RECHDR_LEN as libc::c_int && target == MAX_INBUF as libc::c_int {
        xhdr = (*tls).inbuf as *mut libc::c_void as *mut record_hdr;
        target = RECHDR_LEN as libc::c_int
          + (0x100i32 * (*xhdr).len16_hi as libc::c_int + (*xhdr).len16_lo as libc::c_int);
        if target > MAX_INBUF as libc::c_int
          || (*xhdr).proto_maj as libc::c_int != 3i32
          || (*xhdr).proto_min as libc::c_int != 3i32
        {
          sz = if total < target { total } else { target };
          bad_record_die(tls, expected, sz);
        }
      }
      /* if total >= target, we have a full packet (and possibly more)... */
      if total - target >= 0i32 {
        (*tls).buffered_size = total - target;
        (*tls).ofs_to_buffered = target;
        //dbg("<< stashing at %d [%d] ", tls->ofs_to_buffered, tls->buffered_size);
        //dump_hex("<< %s\n", tls->inbuf + tls->ofs_to_buffered, tls->buffered_size);
        sz = target - RECHDR_LEN as libc::c_int;
        /* Needs to be decrypted? */
        if (*tls).min_encrypted_len_on_read != 0i32 as libc::c_uint {
          if sz < (*tls).min_encrypted_len_on_read as libc::c_int {
            bb_error_msg_and_die(
              b"bad encrypted len:%u\x00" as *const u8 as *const libc::c_char,
              sz,
            );
          }
          if (*tls).flags & ENCRYPTION_AESGCM as libc::c_int as libc::c_uint != 0 {
            /* AESGCM */
            let mut p: *mut uint8_t = (*tls).inbuf.offset(RECHDR_LEN as libc::c_int as isize); /* we will overwrite nonce, drop hash */
            sz -= 8i32 + 16i32;
            tls_aesgcm_decrypt(tls, p, sz);
          } else if (*tls).min_encrypted_len_on_read > SHA256_OUTSIZE as libc::c_int as libc::c_uint
          {
            /* AES+SHA */
            let mut p_0: *mut uint8_t = (*tls).inbuf.offset(RECHDR_LEN as libc::c_int as isize);
            let mut padding_len: libc::c_int = 0;
            if sz & 16i32 - 1i32 != 0 {
              bb_error_msg_and_die(
                b"bad encrypted len:%u\x00" as *const u8 as *const libc::c_char,
                sz,
              );
            }
            /* Decrypt content+MAC+padding, moving it over IV in the process */
            sz -= 16i32; /* we will overwrite IV now */
            aes_cbc_decrypt(
              &mut (*tls).aes_decrypt,
              p_0 as *mut libc::c_void,
              p_0.offset(16) as *const libc::c_void,
              sz as size_t,
              p_0 as *mut libc::c_void,
            );
            padding_len = *p_0.offset((sz - 1i32) as isize) as libc::c_int;
            padding_len += 1;
            sz -= SHA256_OUTSIZE as libc::c_int + padding_len
          } else {
            /* if nonzero, then it's TLS_RSA_WITH_NULL_SHA256: drop MAC */
            /* else: no encryption yet on input, subtract zero = NOP */
            sz = (sz as libc::c_uint).wrapping_sub((*tls).min_encrypted_len_on_read) as libc::c_int
              as libc::c_int
          }
        }
        if sz < 0i32 {
          bb_simple_error_msg_and_die(
            b"encrypted data too short\x00" as *const u8 as *const libc::c_char,
          );
        }
        //dump_hex("<< %s\n", tls->inbuf, RECHDR_LEN + sz);
        xhdr = (*tls).inbuf as *mut libc::c_void as *mut record_hdr;
        if (*xhdr).type_0 as libc::c_int == 21i32 && sz >= 2i32 {
          current_block = 9925100494328262799;
          break;
        } else {
          current_block = 13910774313357589740;
          break;
        }
      } else {
        /* input buffer is grown only as needed */
        rem = (*tls).inbuf_size - total;
        if rem == 0i32 {
          (*tls).inbuf_size += MAX_INBUF as libc::c_int / 8i32;
          if (*tls).inbuf_size > MAX_INBUF as libc::c_int {
            (*tls).inbuf_size = MAX_INBUF as libc::c_int
          }
          rem = (*tls).inbuf_size - total;
          (*tls).inbuf = xrealloc(
            (*tls).inbuf as *mut libc::c_void,
            (*tls).inbuf_size as size_t,
          ) as *mut uint8_t
        }
        sz = safe_read(
          (*tls).ifd,
          (*tls).inbuf.offset(total as isize) as *mut libc::c_void,
          rem as size_t,
        ) as libc::c_int;
        if sz <= 0i32 {
          if sz == 0i32 && total == 0i32 {
            current_block = 11459959175219260272;
            break 's_18;
          } else {
            current_block = 18377268871191777778;
            break 's_18;
          }
        } else {
          total += sz
        }
      }
    }
    match current_block {
      13910774313357589740 => {
        /* RFC 5246 is not saying it explicitly, but sha256 hash
         * in our FINISHED record must include data of incoming packets too!
         */
        if *(*tls).inbuf.offset(0) as libc::c_int == 22i32 {
          /* HANDSHAKE HASH: */
          // && do_we_know_which_hash_to_use /* server_hello() might not know it in the future! */
          hash_handshake(
            tls,
            (*tls).inbuf.offset(RECHDR_LEN as libc::c_int as isize) as *const libc::c_void,
            sz as libc::c_uint,
          );
        }
        current_block = 10435735846551762309;
        break;
      }
      _ => {
        let mut p_1: *mut uint8_t = (*tls).inbuf.offset(RECHDR_LEN as libc::c_int as isize);
        if *p_1.offset(0) as libc::c_int == 2i32 {
          /* fatal */
          bb_error_msg_and_die(
            b"TLS %s from peer (alert code %d): %s\x00" as *const u8 as *const libc::c_char,
            b"error\x00" as *const u8 as *const libc::c_char,
            *p_1.offset(1) as libc::c_int,
            alert_text(*p_1.offset(1) as libc::c_int),
          );
        }
        if *p_1.offset(0) as libc::c_int == 1i32 {
          /* warning */
          if !(*p_1.offset(1) as libc::c_int == 0i32) {
            continue;
          }
          /* "close_notify" warning: it's EOF */
          sz = 0i32;
          current_block = 10435735846551762309;
          break;
        } else {
          /* p[0] not 1 or 2: not defined in protocol */
          sz = 0i32;
          current_block = 10435735846551762309;
          break;
        }
      }
    }
  }
  match current_block {
    11459959175219260272 =>
    /* "Abrupt" EOF, no TLS shutdown (seen from kernel.org) */
    {
      (*tls).buffered_size = 0i32
    }
    18377268871191777778 => {
      bb_perror_msg_and_die(
        b"short read, have only %d\x00" as *const u8 as *const libc::c_char,
        total,
      );
    }
    _ => {}
  }
  return sz;
}
unsafe extern "C" fn binary_to_pstm(
  mut pstm_n: *mut pstm_int,
  mut bin_ptr: *mut uint8_t,
  mut len: libc::c_uint,
) {
  pstm_init_for_read_unsigned_bin(pstm_n, len);
  pstm_read_unsigned_bin(pstm_n, bin_ptr, len as int32);
  //return bin_ptr + len;
}
/*
 * DER parsing routines
 */
unsafe extern "C" fn get_der_len(
  mut bodyp: *mut *mut uint8_t,
  mut der: *mut uint8_t,
  mut end: *mut uint8_t,
) -> libc::c_uint {
  let mut len: libc::c_uint = 0;
  let mut len1: libc::c_uint = 0;
  if (end.wrapping_offset_from(der) as libc::c_long) < 2i32 as libc::c_long {
    xfunc_die();
  }
  //	if ((der[0] & 0x1f) == 0x1f) /* not single-byte item code? */
  //		xfunc_die();
  len = *der.offset(1) as libc::c_uint; /* maybe it's short len */
  if len >= 0x80i32 as libc::c_uint {
    /* no, it's long */
    if len == 0x80i32 as libc::c_uint
      || (end.wrapping_offset_from(der) as libc::c_long)
        < len.wrapping_sub(0x7ei32 as libc::c_uint) as libc::c_int as libc::c_long
    {
      /* 0x80 is "0 bytes of len", invalid DER: must use short len if can */
      /* need 3 or 4 bytes for 81, 82 */
      xfunc_die();
    }
    //		if (len < 0x80)
    //			xfunc_die(); /* invalid DER: must use short len if can */
    len1 = *der.offset(2) as libc::c_uint; /* if (len == 0x81) it's "ii 81 xx", fetch xx */
    if len > 0x82i32 as libc::c_uint {
      /* >0x82 is "3+ bytes of len", should not happen realistically */
      xfunc_die();
    }
    if len == 0x82i32 as libc::c_uint {
      /* it's "ii 82 xx yy" */
      len1 = (0x100i32 as libc::c_uint)
        .wrapping_mul(len1)
        .wrapping_add(*der.offset(3) as libc::c_uint);
      der = der.offset(1)
      /* skip [yy] */
    } /* skip [xx] */
    der = der.offset(1); /* skip [code]+[1byte] */
    len = len1
  }
  der = der.offset(2);
  if (end.wrapping_offset_from(der) as libc::c_long) < len as libc::c_int as libc::c_long {
    xfunc_die();
  }
  *bodyp = der;
  return len;
}
unsafe extern "C" fn enter_der_item(
  mut der: *mut uint8_t,
  mut endp: *mut *mut uint8_t,
) -> *mut uint8_t {
  let mut new_der: *mut uint8_t = 0 as *mut uint8_t;
  let mut len: libc::c_uint = get_der_len(&mut new_der, der, *endp);
  /* Move "end" position to cover only this item */
  *endp = new_der.offset(len as isize);
  return new_der;
}
unsafe extern "C" fn skip_der_item(mut der: *mut uint8_t, mut end: *mut uint8_t) -> *mut uint8_t {
  let mut new_der: *mut uint8_t = 0 as *mut uint8_t;
  let mut len: libc::c_uint = get_der_len(&mut new_der, der, end);
  /* Skip body */
  new_der = new_der.offset(len as isize);
  return new_der;
}
unsafe extern "C" fn der_binary_to_pstm(
  mut pstm_n: *mut pstm_int,
  mut der: *mut uint8_t,
  mut end: *mut uint8_t,
) {
  let mut bin_ptr: *mut uint8_t = 0 as *mut uint8_t;
  let mut len: libc::c_uint = get_der_len(&mut bin_ptr, der, end);
  binary_to_pstm(pstm_n, bin_ptr, len);
}
unsafe extern "C" fn find_key_in_der_cert(
  mut tls: *mut tls_state_t,
  mut der: *mut uint8_t,
  mut len: libc::c_int,
) {
  /* Certificate is a DER-encoded data structure. Each DER element has a length,
   * which makes it easy to skip over large compound elements of any complexity
   * without parsing them. Example: partial decode of kernel.org certificate:
   *  SEQ 0x05ac/1452 bytes (Certificate): 308205ac
   *    SEQ 0x0494/1172 bytes (tbsCertificate): 30820494
   *      [ASN_CONTEXT_SPECIFIC | ASN_CONSTRUCTED | 0] 3 bytes: a003
   *        INTEGER (version): 0201 02
   *      INTEGER 0x11 bytes (serialNumber): 0211 00 9f85bf664b0cddafca508679501b2be4
   *      //^^^^^^note: matrixSSL also allows [ASN_CONTEXT_SPECIFIC | ASN_PRIMITIVE | 2] = 0x82 type
   *      SEQ 0x0d bytes (signatureAlgo): 300d
   *        OID 9 bytes: 0609 2a864886f70d01010b (OID_SHA256_RSA_SIG 42.134.72.134.247.13.1.1.11)
   *        NULL: 0500
   *      SEQ 0x5f bytes (issuer): 305f
   *        SET 11 bytes: 310b
   *          SEQ 9 bytes: 3009
   *            OID 3 bytes: 0603 550406
   *            Printable string "FR": 1302 4652
   *        SET 14 bytes: 310e
   *          SEQ 12 bytes: 300c
   *            OID 3 bytes: 0603 550408
   *            Printable string "Paris": 1305 5061726973
   *        SET 14 bytes: 310e
   *          SEQ 12 bytes: 300c
   *            OID 3 bytes: 0603 550407
   *            Printable string "Paris": 1305 5061726973
   *        SET 14 bytes: 310e
   *          SEQ 12 bytes: 300c
   *            OID 3 bytes: 0603 55040a
   *            Printable string "Gandi": 1305 47616e6469
   *        SET 32 bytes: 3120
   *          SEQ 30 bytes: 301e
   *            OID 3 bytes: 0603 550403
   *            Printable string "Gandi Standard SSL CA 2": 1317 47616e6469205374616e646172642053534c2043412032
   *      SEQ 30 bytes (validity): 301e
   *        TIME "161011000000Z": 170d 3136313031313030303030305a
   *        TIME "191011235959Z": 170d 3139313031313233353935395a
   *      SEQ 0x5b/91 bytes (subject): 305b //I did not decode this
   *          3121301f060355040b1318446f6d61696e20436f
   *          6e74726f6c2056616c6964617465643121301f06
   *          0355040b1318506f73697469766553534c204d75
   *          6c74692d446f6d61696e31133011060355040313
   *          0a6b65726e656c2e6f7267
   *      SEQ 0x01a2/418 bytes (subjectPublicKeyInfo): 308201a2
   *        SEQ 13 bytes (algorithm): 300d
   *          OID 9 bytes: 0609 2a864886f70d010101 (OID_RSA_KEY_ALG 42.134.72.134.247.13.1.1.1)
   *          NULL: 0500
   *        BITSTRING 0x018f/399 bytes (publicKey): 0382018f
   *          ????: 00
   *          //after the zero byte, it appears key itself uses DER encoding:
   *          SEQ 0x018a/394 bytes: 3082018a
   *            INTEGER 0x0181/385 bytes (modulus): 02820181
   *                  00b1ab2fc727a3bef76780c9349bf3
   *                  ...24 more blocks of 15 bytes each...
   *                  90e895291c6bc8693b65
   *            INTEGER 3 bytes (exponent): 0203 010001
   *      [ASN_CONTEXT_SPECIFIC | ASN_CONSTRUCTED | 0x3] 0x01e5 bytes (X509v3 extensions): a38201e5
   *        SEQ 0x01e1 bytes: 308201e1
   *        ...
   * Certificate is a sequence of three elements:
   *	tbsCertificate (SEQ)
   *	signatureAlgorithm (AlgorithmIdentifier)
   *	signatureValue (BIT STRING)
   *
   * In turn, tbsCertificate is a sequence of:
   *	version
   *	serialNumber
   *	signatureAlgo (AlgorithmIdentifier)
   *	issuer (Name, has complex structure)
   *	validity (Validity, SEQ of two Times)
   *	subject (Name)
   *	subjectPublicKeyInfo (SEQ)
   *	...
   *
   * subjectPublicKeyInfo is a sequence of:
   *	algorithm (AlgorithmIdentifier)
   *	publicKey (BIT STRING)
   *
   * We need Certificate.tbsCertificate.subjectPublicKeyInfo.publicKey
   *
   * Example of an ECDSA key:
   *      SEQ 0x59 bytes (subjectPublicKeyInfo): 3059
   *        SEQ 0x13 bytes (algorithm): 3013
   *          OID 7 bytes: 0607 2a8648ce3d0201   (OID_ECDSA_KEY_ALG 42.134.72.206.61.2.1)
   *          OID 8 bytes: 0608 2a8648ce3d030107 (OID_EC_prime256v1 42.134.72.206.61.3.1.7)
   *        BITSTRING 0x42 bytes (publicKey): 0342
   *          0004 53af f65e 50cc 7959 7e29 0171 c75c
   *          7335 e07d f45b 9750 b797 3a38 aebb 2ac6
   *          8329 2748 e77e 41cb d482 2ce6 05ec a058
   *          f3ab d561 2f4c d845 9ad3 7252 e3de bd3b
   *          9012
   */
  let mut end: *mut uint8_t = der.offset(len as isize);
  /* enter "Certificate" item: [der, end) will be only Cert */
  der = enter_der_item(der, &mut end);
  /* enter "tbsCertificate" item: [der, end) will be only tbsCert */
  der = enter_der_item(der, &mut end);
  /*
   * Skip version field only if it is present. For a v1 certificate, the
   * version field won't be present since v1 is the default value for the
   * version field and fields with default values should be omitted (see
   * RFC 5280 sections 4.1 and 4.1.2.1). If the version field is present
   * it will have a tag class of 2 (context-specific), bit 6 as 1
   * (constructed), and a tag number of 0 (see ITU-T X.690 sections 8.1.2
   * and 8.14).
   */
  /* bits 7-6: 10 */
  /* bit 5: 1 */
  /* bits 4-0: 00000 */
  if *der.offset(0) as libc::c_int == 0xa0i32 {
    der = skip_der_item(der, end)
  } /* version */
  /* skip up to subjectPublicKeyInfo */
  der = skip_der_item(der, end); /* serialNumber */
  der = skip_der_item(der, end); /* signatureAlgo */
  der = skip_der_item(der, end); /* issuer */
  der = skip_der_item(der, end); /* validity */
  der = skip_der_item(der, end); /* subject */
  /* enter subjectPublicKeyInfo */
  der = enter_der_item(der, &mut end);
  /* check subjectPublicKeyInfo.algorithm */
  static mut OID_RSA_KEY_ALG: [uint8_t; 13] = [
    0x30i32 as uint8_t,
    0xdi32 as uint8_t,
    0x6i32 as uint8_t,
    0x9i32 as uint8_t,
    0x2ai32 as uint8_t,
    0x86i32 as uint8_t,
    0x48i32 as uint8_t,
    0x86i32 as uint8_t,
    0xf7i32 as uint8_t,
    0xdi32 as uint8_t,
    0x1i32 as uint8_t,
    0x1i32 as uint8_t,
    0x1i32 as uint8_t,
  ];
  static mut OID_ECDSA_KEY_ALG: [uint8_t; 11] = [
    0x30i32 as uint8_t,
    0x13i32 as uint8_t,
    0x6i32 as uint8_t,
    0x7i32 as uint8_t,
    0x2ai32 as uint8_t,
    0x86i32 as uint8_t,
    0x48i32 as uint8_t,
    0xcei32 as uint8_t,
    0x3di32 as uint8_t,
    0x2i32 as uint8_t,
    0x1i32 as uint8_t,
  ];
  if memcmp(
    der as *const libc::c_void,
    OID_RSA_KEY_ALG.as_ptr() as *const libc::c_void,
    ::std::mem::size_of::<[uint8_t; 13]>() as libc::c_ulong,
  ) == 0i32
  {
    (*tls).flags |= GOT_CERT_RSA_KEY_ALG as libc::c_int as libc::c_uint
  } else if memcmp(
    der as *const libc::c_void,
    OID_ECDSA_KEY_ALG.as_ptr() as *const libc::c_void,
    ::std::mem::size_of::<[uint8_t; 11]>() as libc::c_ulong,
  ) == 0i32
  {
  } else {
    bb_simple_error_msg_and_die(b"not RSA or ECDSA cert\x00" as *const u8 as *const libc::c_char);
  }
  if (*tls).flags & GOT_CERT_RSA_KEY_ALG as libc::c_int as libc::c_uint != 0 {
    /* parse RSA key: */
    //based on getAsnRsaPubKey(), pkcs1ParsePrivBin() is also of note
    /* skip subjectPublicKeyInfo.algorithm */
    der = skip_der_item(der, end);
    /* enter subjectPublicKeyInfo.publicKey */
    //die_if_not_this_der_type(der, end, 0x03); /* must be BITSTRING */
    der = enter_der_item(der, &mut end);
    if (end.wrapping_offset_from(der) as libc::c_long) < 14i32 as libc::c_long {
      xfunc_die();
    }
    /* example format:
     * ignore bits: 00
     * SEQ 0x018a/394 bytes: 3082018a
     *   INTEGER 0x0181/385 bytes (modulus): 02820181 XX...XXX
     *   INTEGER 3 bytes (exponent): 0203 010001
     */
    if *der as libc::c_int != 0i32 {
      /* "ignore bits", should be 0 */
      xfunc_die(); /* enter SEQ */
    }
    der = der.offset(1);
    der = enter_der_item(der, &mut end);
    /* memset(tls->hsd->server_rsa_pub_key, 0, sizeof(tls->hsd->server_rsa_pub_key)); - already is */
    der_binary_to_pstm(&mut (*(*tls).hsd).server_rsa_pub_key.N, der, end); /* modulus */
    der = skip_der_item(der, end); /* exponent */
    der_binary_to_pstm(&mut (*(*tls).hsd).server_rsa_pub_key.e, der, end);
    (*(*tls).hsd).server_rsa_pub_key.size =
      pstm_unsigned_bin_size(&mut (*(*tls).hsd).server_rsa_pub_key.N) as uint32
  };
  /* else: ECDSA key. It is not used for generating encryption keys,
   * it is used only to sign the EC public key (which comes in ServerKey message).
   * Since we do not verify cert validity, verifying signature on EC public key
   * wouldn't add any security. Thus, we do nothing here.
   */
}
/*
 * TLS Handshake routines
 */
unsafe extern "C" fn tls_xread_handshake_block(
  mut tls: *mut tls_state_t,
  mut min_len: libc::c_int,
) -> libc::c_int {
  let mut xhdr: *mut record_hdr = 0 as *mut record_hdr;
  let mut len: libc::c_int = tls_xread_record(
    tls,
    b"handshake record\x00" as *const u8 as *const libc::c_char,
  );
  xhdr = (*tls).inbuf as *mut libc::c_void as *mut record_hdr;
  if len < min_len || (*xhdr).type_0 as libc::c_int != 22i32 {
    bad_record_die(
      tls,
      b"handshake record\x00" as *const u8 as *const libc::c_char,
      len,
    );
  }
  return len;
}
#[inline(always)]
unsafe extern "C" fn fill_handshake_record_hdr(
  mut buf: *mut libc::c_void,
  mut type_0: libc::c_uint,
  mut len: libc::c_uint,
) {
  let mut h: *mut handshake_hdr = buf as *mut handshake_hdr;
  len = len.wrapping_sub(4i32 as libc::c_uint);
  (*h).type_0 = type_0 as uint8_t;
  (*h).len24_hi = (len >> 16i32) as uint8_t;
  (*h).len24_mid = (len >> 8i32) as uint8_t;
  (*h).len24_lo = (len & 0xffi32 as libc::c_uint) as uint8_t;
}
unsafe extern "C" fn send_client_hello_and_alloc_hsd(
  mut tls: *mut tls_state_t,
  mut sni: *const libc::c_char,
) {
  static mut ciphers: [uint8_t; 20] = [
    0i32 as uint8_t,
    (2i32 + (7i32 + 6i32 * 0i32 + 0i32) * 2i32) as uint8_t,
    0i32 as uint8_t,
    0xffi32 as uint8_t,
    0xc0i32 as uint8_t,
    0x23i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x27i32 as uint8_t,
    0xc0i32 as uint8_t,
    0x2bi32 as uint8_t,
    0xc0i32 as uint8_t,
    0x2fi32 as uint8_t,
    0i32 as uint8_t,
    0x3ci32 as uint8_t,
    0i32 as uint8_t,
    0x3di32 as uint8_t,
    0i32 as uint8_t,
    0x9ci32 as uint8_t,
    0x1i32 as uint8_t,
    0i32 as uint8_t,
  ];
  static mut supported_groups: [uint8_t; 8] = [
    0i32 as uint8_t,
    0xai32 as uint8_t,
    0i32 as uint8_t,
    0x4i32 as uint8_t,
    0i32 as uint8_t,
    0x2i32 as uint8_t,
    0i32 as uint8_t,
    0x1di32 as uint8_t,
  ];
  //static const uint8_t signature_algorithms[] = {
  //	000d
  //	0020
  //	001e
  //	0601 0602 0603 0501 0502 0503 0401 0402 0403 0301 0302 0303 0201 0202 0203
  //};
  let mut record: *mut client_hello = 0 as *mut client_hello;
  let mut ptr: *mut uint8_t = 0 as *mut uint8_t;
  let mut len: libc::c_int = 0;
  let mut ext_len: libc::c_int = 0;
  let mut sni_len: libc::c_int = if !sni.is_null() {
    strnlen(sni, (127i32 - 5i32) as size_t)
  } else {
    0i32 as libc::c_ulong
  } as libc::c_int;
  ext_len = 0i32;
  /* is.gd responds with "handshake failure" to our hello if there's no supported_groups element */
  ext_len = (ext_len as libc::c_ulong)
    .wrapping_add(::std::mem::size_of::<[uint8_t; 8]>() as libc::c_ulong) as libc::c_int
    as libc::c_int;
  if sni_len != 0 {
    ext_len += 9i32 + sni_len
  }
  /* +2 is for "len of all extensions" 2-byte field */
  len = (::std::mem::size_of::<client_hello>() as libc::c_ulong)
    .wrapping_add(2i32 as libc::c_ulong)
    .wrapping_add(ext_len as libc::c_ulong) as libc::c_int; /* the "requested" version of the protocol, */
  record = tls_get_zeroed_outbuf(tls, len) as *mut client_hello; /* can be higher than one in record headers */
  fill_handshake_record_hdr(
    record as *mut libc::c_void,
    1i32 as libc::c_uint,
    len as libc::c_uint,
  );
  (*record).proto_maj = 3i32 as uint8_t;
  (*record).proto_min = 3i32 as uint8_t;
  tls_get_random(
    (*record).rand32.as_mut_ptr() as *mut libc::c_void,
    ::std::mem::size_of::<[uint8_t; 32]>() as libc::c_ulong as libc::c_uint,
  );
  /* record->session_id_len = 0; - already is */
  memcpy(
    &mut (*record).cipherid_len16_hi as *mut uint8_t as *mut libc::c_void,
    ciphers.as_ptr() as *const libc::c_void,
    ::std::mem::size_of::<[uint8_t; 20]>() as libc::c_ulong,
  );
  ptr = record.offset(1) as *mut libc::c_void as *mut uint8_t;
  let fresh27 = ptr;
  ptr = ptr.offset(1);
  *fresh27 = (ext_len >> 8i32) as uint8_t;
  let fresh28 = ptr;
  ptr = ptr.offset(1);
  *fresh28 = ext_len as uint8_t;
  if sni_len != 0 {
    //ptr[0] = 0;             //
    //ptr[1] = 0;             //extension_type
    //ptr[2] = 0;         //
    *ptr.offset(3) = (sni_len + 5i32) as uint8_t; //list len
                                                  //ptr[4] = 0;             //
    *ptr.offset(5) = (sni_len + 3i32) as uint8_t; //len of 1st SNI
                                                  //ptr[6] = 0;         //name type
                                                  //ptr[7] = 0;             //
    *ptr.offset(8) = sni_len as uint8_t; //name len
    ptr = mempcpy(
      &mut *ptr.offset(9) as *mut uint8_t as *mut libc::c_void,
      sni as *const libc::c_void,
      sni_len as size_t,
    ) as *mut uint8_t
  }
  memcpy(
    ptr as *mut libc::c_void,
    supported_groups.as_ptr() as *const libc::c_void,
    ::std::mem::size_of::<[uint8_t; 8]>() as libc::c_ulong,
  );
  (*tls).hsd = xzalloc(::std::mem::size_of::<tls_handshake_data>() as libc::c_ulong)
    as *mut tls_handshake_data;
  /* HANDSHAKE HASH: ^^^ + len if need to save saved_client_hello */
  memcpy(
    (*(*tls).hsd).client_and_server_rand32.as_mut_ptr() as *mut libc::c_void,
    (*record).rand32.as_mut_ptr() as *const libc::c_void,
    ::std::mem::size_of::<[uint8_t; 32]>() as libc::c_ulong,
  );
  /* HANDSHAKE HASH:
   tls->hsd->saved_client_hello_size = len;
   memcpy(tls->hsd->saved_client_hello, record, len);
  */
  /* Can hash immediately only if we know which MAC hash to use.
   * So far we do know: it's sha256:
   */
  sha256_begin(&mut (*(*tls).hsd).handshake_hash_ctx);
  xwrite_and_update_handshake_hash(tls, len as libc::c_uint);
  /* if this would become infeasible: save tls->hsd->saved_client_hello,
   * use "xwrite_handshake_record(tls, len)" here,
   * and hash saved_client_hello later.
   */
}
unsafe extern "C" fn get_server_hello(mut tls: *mut tls_state_t) {
  let mut hp: *mut server_hello = 0 as *mut server_hello;
  let mut cipherid: *mut uint8_t = 0 as *mut uint8_t;
  let mut cipherid1: uint8_t = 0;
  let mut len: libc::c_int = 0;
  let mut len24: libc::c_int = 0;
  len = tls_xread_handshake_block(tls, 74i32 - 32i32);
  hp = (*tls).inbuf as *mut libc::c_void as *mut server_hello;
  // 74 bytes:
  // 02  000046 03|03   58|78|cf|c1 50|a5|49|ee|7e|29|48|71|fe|97|fa|e8|2d|19|87|72|90|84|9d|37|a3|f0|cb|6f|5f|e3|3c|2f |20  |d8|1a|78|96|52|d6|91|01|24|b3|d6|5b|b7|d0|6c|b3|e1|78|4e|3c|95|de|74|a0|ba|eb|a7|3a|ff|bd|a2|bf |00|9c |00|
  //SvHl len=70 maj.min unixtime^^^ 28randbytes^^^^^^^^^^^^^^^^^^^^^^^^^^^^_^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^_^^^ slen sid32bytes^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cipSel comprSel
  if (*hp).type_0 as libc::c_int != 2i32
    || (*hp).len24_hi as libc::c_int != 0i32
    || (*hp).len24_mid as libc::c_int != 0i32
    || (*hp).proto_maj as libc::c_int != 3i32
    || (*hp).proto_min as libc::c_int != 3i32
  {
    bad_record_die(
      tls,
      b"\'server hello\'\x00" as *const u8 as *const libc::c_char,
      len,
    );
  }
  cipherid = &mut (*hp).cipherid_hi;
  len24 = (*hp).len24_lo as libc::c_int;
  if (*hp).session_id_len as libc::c_int != 32i32 {
    if (*hp).session_id_len as libc::c_int != 0i32 {
      bad_record_die(
        tls,
        b"\'server hello\'\x00" as *const u8 as *const libc::c_char,
        len,
      );
    }
    /* what len would be if session id would be present */
    cipherid = cipherid.offset(-32);
    len24 += 32i32
  }
  if len24 < 70i32 {
    bad_record_die(
      tls,
      b"\'server hello\'\x00" as *const u8 as *const libc::c_char,
      len,
    );
  }
  memcpy(
    (*(*tls).hsd)
      .client_and_server_rand32
      .as_mut_ptr()
      .offset(32) as *mut libc::c_void,
    (*hp).rand32.as_mut_ptr() as *const libc::c_void,
    ::std::mem::size_of::<[uint8_t; 32]>() as libc::c_ulong,
  );
  // session_id_len == 0: no session id
  // "The server
  // may return an empty session_id to indicate that the session will
  // not be cached and therefore cannot be resumed."
  /* Set up encryption params based on selected cipher */
  cipherid1 = *cipherid.offset(1);
  (*tls).cipher_id =
    (0x100i32 * *cipherid.offset(0) as libc::c_int + cipherid1 as libc::c_int) as uint16_t;
  (*tls).key_size = AES256_KEYSIZE as libc::c_int as libc::c_uint;
  (*tls).MAC_size = SHA256_OUTSIZE as libc::c_int as libc::c_uint;
  /*tls->IV_size = 0; - already is */
  if *cipherid.offset(0) as libc::c_int == 0xc0i32 {
    /* All C0xx are ECDHE */
    (*tls).flags |= NEED_EC_KEY as libc::c_int as libc::c_uint;
    if cipherid1 as libc::c_int & 1i32 != 0 {
      /* Odd numbered C0xx use AES128 (even ones use AES256) */
      (*tls).key_size = AES128_KEYSIZE as libc::c_int as libc::c_uint
    }
    if 0i32 != 0 && cipherid1 as libc::c_int <= 0x19i32 {
      (*tls).MAC_size = SHA1_OUTSIZE as libc::c_int as libc::c_uint
    } else if cipherid1 as libc::c_int >= 0x2bi32 && cipherid1 as libc::c_int <= 0x30i32 {
      /* C02B,2C,2F,30 are AES-GCM */
      (*tls).flags |= ENCRYPTION_AESGCM as libc::c_int as libc::c_uint;
      (*tls).MAC_size = 0i32 as libc::c_uint;
      (*tls).IV_size = 4i32 as libc::c_uint
    }
  } else {
    /* All 00xx are RSA */
    if 0i32 != 0 && cipherid1 as libc::c_int == 0x2fi32
      || cipherid1 as libc::c_int == 0x3ci32
      || cipherid1 as libc::c_int == 0x9ci32
    {
      (*tls).key_size = AES128_KEYSIZE as libc::c_int as libc::c_uint
    }
    if 0i32 != 0 && cipherid1 as libc::c_int <= 0x35i32 {
      (*tls).MAC_size = SHA1_OUTSIZE as libc::c_int as libc::c_uint
    } else if cipherid1 as libc::c_int == 0x9ci32 {
      /*|| cipherid1 == 0x9D*/
      /* 009C,9D are AES-GCM */
      (*tls).flags |= ENCRYPTION_AESGCM as libc::c_int as libc::c_uint;
      (*tls).MAC_size = 0i32 as libc::c_uint;
      (*tls).IV_size = 4i32 as libc::c_uint
    }
  };
  /* Handshake hash eventually destined to FINISHED record
   * is sha256 regardless of cipher
   * (at least for all ciphers defined by RFC5246).
   * It's not sha1 for AES_128_CBC_SHA - only MAC is sha1, not this hash.
   */
  /* HANDSHAKE HASH:
   sha256_begin(&tls->hsd->handshake_hash_ctx);
   hash_handshake(tls, ">> client hello hash:%s",
     tls->hsd->saved_client_hello, tls->hsd->saved_client_hello_size
   );
   hash_handshake(tls, "<< server hello hash:%s",
     tls->inbuf + RECHDR_LEN, len
   );
  */
}
unsafe extern "C" fn get_server_cert(mut tls: *mut tls_state_t) {
  let mut xhdr: *mut record_hdr = 0 as *mut record_hdr;
  let mut certbuf: *mut uint8_t = 0 as *mut uint8_t;
  let mut len: libc::c_int = 0;
  let mut len1: libc::c_int = 0;
  len = tls_xread_handshake_block(tls, 10i32);
  xhdr = (*tls).inbuf as *mut libc::c_void as *mut record_hdr;
  certbuf = xhdr.offset(1) as *mut libc::c_void as *mut uint8_t;
  if *certbuf.offset(0) as libc::c_int != 11i32 {
    bad_record_die(
      tls,
      b"certificate\x00" as *const u8 as *const libc::c_char,
      len,
    );
  }
  // 4392 bytes:
  // 0b  00|11|24 00|11|21 00|05|b0 30|82|05|ac|30|82|04|94|a0|03|02|01|02|02|11|00|9f|85|bf|66|4b|0c|dd|af|ca|50|86|79|50|1b|2b|e4|30|0d...
  //Cert len=4388 ChainLen CertLen^ DER encoded X509 starts here. openssl x509 -in FILE -inform DER -noout -text
  len1 = get24be(certbuf.offset(1)) as libc::c_int;
  if len1 > len - 4i32 {
    tls_error_die(tls, 1817i32);
  }
  len = len1;
  len1 = get24be(certbuf.offset(4)) as libc::c_int;
  if len1 > len - 3i32 {
    tls_error_die(tls, 1820i32);
  }
  len = len1;
  len1 = get24be(certbuf.offset(7)) as libc::c_int;
  if len1 > len - 3i32 {
    tls_error_die(tls, 1823i32);
  }
  len = len1;
  if len != 0 {
    find_key_in_der_cert(tls, certbuf.offset(10), len);
  };
}
/* On input, len is known to be >= 4.
 * The record is known to be SERVER_KEY_EXCHANGE.
 */
unsafe extern "C" fn process_server_key(mut tls: *mut tls_state_t, mut len: libc::c_int) {
  let mut xhdr: *mut record_hdr = 0 as *mut record_hdr;
  let mut keybuf: *mut uint8_t = 0 as *mut uint8_t;
  let mut len1: libc::c_int = 0;
  let mut t32: uint32_t = 0;
  xhdr = (*tls).inbuf as *mut libc::c_void as *mut record_hdr;
  keybuf = xhdr.offset(1) as *mut libc::c_void as *mut uint8_t;
  //seen from is.gd: it selects curve_x25519:
  //  0c 00006e //SERVER_KEY_EXCHANGE, len
  //    03 //curve_type: named curve
  //    001d //curve_x25519
  //server-chosen EC point, and then signed_params
  //      (RFC 8422: "A hash of the params, with the signature
  //      appropriate to that hash applied.  The private key corresponding
  //      to the certified public key in the server's Certificate message is
  //      used for signing.")
  //follow. Format unclear/guessed:
  //    20 //eccPubKeyLen
  //      25511923d73b70dd2f60e66ba2f3fda31a9c25170963c7a3a972e481dbb2835d //eccPubKey (32bytes)
  //    0203 //hashSigAlg: 2:SHA1 (4:SHA256 5:SHA384 6:SHA512), 3:ECDSA (1:RSA)
  //    0046 //len (16bit)
  //      30 44 //SEQ, len
  //        02 20 //INTEGER, len
  //          2e18e7c2a9badd0a70cd3059a6ab114539b9f5163568911147386cd77ed7c412 //32bytes
  //this item ^^^^^ is sometimes 33 bytes (with all container sizes also +1)
  //        02 20 //INTEGER, len
  //          64523d6216cb94c43c9b20e377d8c52c55be6703fd6730a155930c705eaf3af6 //32bytes
  //same about this item ^^^^^
  //seen from ftp.openbsd.org
  //(which only accepts ECDHE-RSA-AESnnn-GCM-SHAnnn and ECDHE-RSA-CHACHA20-POLY1305 ciphers):
  //  0c 000228 //SERVER_KEY_EXCHANGE, len
  //    03 //curve_type: named curve
  //    001d //curve_x25519
  //    20 //eccPubKeyLen
  //      eef7a15c43b71a4c7eaa48a39369399cc4332e569ec90a83274cc92596705c1a //eccPubKey
  //    0401 //hashSigAlg: 4:SHA256, 1:RSA
  //    0200 //len
  //      //0x200 bytes follow
  /* Get and verify length */
  len1 = get24be(keybuf.offset(1)) as libc::c_int;
  if len1 > len - 4i32 {
    tls_error_die(tls, 1877i32);
  }
  len = len1;
  if len < 1i32 + 2i32 + 1i32 + 32i32 {
    tls_error_die(tls, 1879i32);
  }
  keybuf = keybuf.offset(4);
  /* So far we only support curve_x25519 */
  t32 = *(keybuf as *mut bb__aliased_uint32_t);
  if t32
    != ({
      let mut __v: libc::c_uint = 0;
      let mut __x: libc::c_uint = 0x3001d20i32 as libc::c_uint;
      if 0 != 0 {
        __v = (__x & 0xff000000u32) >> 24i32
          | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
          | (__x & 0xff00i32 as libc::c_uint) << 8i32
          | (__x & 0xffi32 as libc::c_uint) << 24i32
      } else {
        let fresh29 = &mut __v;
        let fresh30;
        let fresh31 = __x;
        asm!("bswap $0" : "=r" (fresh30) : "0"
                         (c2rust_asm_casts::AsmCast::cast_in(fresh29, fresh31))
                         :);
        c2rust_asm_casts::AsmCast::cast_out(fresh29, fresh31, fresh30);
      }
      __v
    })
  {
    bb_simple_error_msg_and_die(
      b"elliptic curve is not x25519\x00" as *const u8 as *const libc::c_char,
    );
  }
  memcpy(
    (*(*tls).hsd).ecc_pub_key32.as_mut_ptr() as *mut libc::c_void,
    keybuf.offset(4) as *const libc::c_void,
    32i32 as libc::c_ulong,
  );
  (*tls).flags |= GOT_EC_KEY as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn send_empty_client_cert(mut tls: *mut tls_state_t) {
  let mut record: *mut client_empty_cert = 0 as *mut client_empty_cert;
  record = tls_get_zeroed_outbuf(
    tls,
    ::std::mem::size_of::<client_empty_cert>() as libc::c_ulong as libc::c_int,
  ) as *mut client_empty_cert;
  //fill_handshake_record_hdr(record, HANDSHAKE_CERTIFICATE, sizeof(*record));
  //record->cert_chain_len24_hi = 0;
  //record->cert_chain_len24_mid = 0;
  //record->cert_chain_len24_lo = 0;
  // same as above:
  (*record).type_0 = 11i32 as uint8_t;
  (*record).len24_lo = 3i32 as uint8_t;
  xwrite_and_update_handshake_hash(
    tls,
    ::std::mem::size_of::<client_empty_cert>() as libc::c_ulong as libc::c_uint,
  );
}
unsafe extern "C" fn send_client_key_exchange(mut tls: *mut tls_state_t) {
  //FIXME: better size estimate
  let mut record: *mut client_key_exchange = tls_get_zeroed_outbuf(
    tls,
    ::std::mem::size_of::<client_key_exchange>() as libc::c_ulong as libc::c_int,
  ) as *mut client_key_exchange;
  let mut rsa_premaster: [uint8_t; 48] = [0; 48];
  let mut x25519_premaster: [uint8_t; 32] = [0; 32];
  let mut premaster: *mut uint8_t = 0 as *mut uint8_t;
  let mut premaster_size: libc::c_int = 0;
  let mut len: libc::c_int = 0;
  if (*tls).flags & NEED_EC_KEY as libc::c_int as libc::c_uint == 0 {
    /* RSA */
    if (*tls).flags & GOT_CERT_RSA_KEY_ALG as libc::c_int as libc::c_uint == 0 {
      bb_simple_error_msg(b"server cert is not RSA\x00" as *const u8 as *const libc::c_char);
    }
    tls_get_random(
      rsa_premaster.as_mut_ptr() as *mut libc::c_void,
      ::std::mem::size_of::<[uint8_t; 48]>() as libc::c_ulong as libc::c_uint,
    );
    // RFC 5246
    // "Note: The version number in the PreMasterSecret is the version
    // offered by the client in the ClientHello.client_version, not the
    // version negotiated for the connection."
    rsa_premaster[0] = 3i32 as uint8_t;
    rsa_premaster[1] = 3i32 as uint8_t;
    len = psRsaEncryptPub(
      &mut (*(*tls).hsd).server_rsa_pub_key,
      rsa_premaster.as_mut_ptr(),
      ::std::mem::size_of::<[uint8_t; 48]>() as libc::c_ulong as uint32,
      (*record).key.as_mut_ptr().offset(2),
      (::std::mem::size_of::<[uint8_t; 4098]>() as libc::c_ulong)
        .wrapping_sub(2i32 as libc::c_ulong) as uint32,
    );
    /* keylen16 exists for RSA (in TLS, not in SSL), but not for some other key types */
    (*record).key[0] = (len >> 8i32) as uint8_t;
    (*record).key[1] = (len & 0xffi32) as uint8_t;
    len += 2i32;
    premaster = rsa_premaster.as_mut_ptr();
    premaster_size = ::std::mem::size_of::<[uint8_t; 48]>() as libc::c_ulong as libc::c_int
  } else {
    /* ECDHE */
    static mut basepoint9: [uint8_t; 32] = [
      9i32 as uint8_t,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
    ]; //[32]
    let mut privkey: [uint8_t; 32] = [0; 32];
    if (*tls).flags & GOT_EC_KEY as libc::c_int as libc::c_uint == 0 {
      bb_simple_error_msg(b"server did not provide EC key\x00" as *const u8 as *const libc::c_char);
    }
    /* Generate random private key, see RFC 7748 */
    tls_get_random(
      privkey.as_mut_ptr() as *mut libc::c_void,
      ::std::mem::size_of::<[uint8_t; 32]>() as libc::c_ulong as libc::c_uint,
    );
    privkey[0] = (privkey[0] as libc::c_int & 0xf8i32) as uint8_t;
    privkey[(32i32 - 1i32) as usize] =
      (privkey[(32i32 - 1i32) as usize] as libc::c_int & 0x7fi32 | 0x40i32) as uint8_t;
    /* Compute public key */
    curve25519(
      (*record).key.as_mut_ptr().offset(1),
      privkey.as_mut_ptr(),
      basepoint9.as_ptr(),
    );
    /* Compute premaster using peer's public key */
    curve25519(
      x25519_premaster.as_mut_ptr(),
      privkey.as_mut_ptr(),
      (*(*tls).hsd).ecc_pub_key32.as_mut_ptr(),
    );
    len = 32i32;
    (*record).key[0] = len as uint8_t;
    len += 1;
    premaster = x25519_premaster.as_mut_ptr();
    premaster_size = ::std::mem::size_of::<[uint8_t; 32]>() as libc::c_ulong as libc::c_int
  }
  (*record).type_0 = 16i32 as uint8_t;
  /* record->len24_hi = 0; - already is */
  (*record).len24_mid = (len >> 8i32) as uint8_t;
  (*record).len24_lo = (len & 0xffi32) as uint8_t;
  len += 4i32;
  xwrite_and_update_handshake_hash(tls, len as libc::c_uint);
  // RFC 5246
  // For all key exchange methods, the same algorithm is used to convert
  // the pre_master_secret into the master_secret.  The pre_master_secret
  // should be deleted from memory once the master_secret has been
  // computed.
  //      master_secret = PRF(pre_master_secret, "master secret",
  //                          ClientHello.random + ServerHello.random)
  //                          [0..47];
  // The master secret is always exactly 48 bytes in length.  The length
  // of the premaster secret will vary depending on key exchange method.
  prf_hmac_sha256(
    (*(*tls).hsd).master_secret.as_mut_ptr(),
    ::std::mem::size_of::<[uint8_t; 48]>() as libc::c_ulong as libc::c_uint,
    premaster,
    premaster_size as libc::c_uint,
    b"master secret\x00" as *const u8 as *const libc::c_char,
    (*(*tls).hsd).client_and_server_rand32.as_mut_ptr(),
    ::std::mem::size_of::<[uint8_t; 64]>() as libc::c_ulong as libc::c_uint,
  );
  // RFC 5246
  // 6.3.  Key Calculation
  //
  // The Record Protocol requires an algorithm to generate keys required
  // by the current connection state (see Appendix A.6) from the security
  // parameters provided by the handshake protocol.
  //
  // The master secret is expanded into a sequence of secure bytes, which
  // is then split to a client write MAC key, a server write MAC key, a
  // client write encryption key, and a server write encryption key.  Each
  // of these is generated from the byte sequence in that order.  Unused
  // values are empty.  Some AEAD ciphers may additionally require a
  // client write IV and a server write IV (see Section 6.2.3.3).
  //
  // When keys and MAC keys are generated, the master secret is used as an
  // entropy source.
  //
  // To generate the key material, compute
  //
  //    key_block = PRF(SecurityParameters.master_secret,
  //                    "key expansion",
  //                    SecurityParameters.server_random +
  //                    SecurityParameters.client_random);
  //
  // until enough output has been generated.  Then, the key_block is
  // partitioned as follows:
  //
  //    client_write_MAC_key[SecurityParameters.mac_key_length]
  //    server_write_MAC_key[SecurityParameters.mac_key_length]
  //    client_write_key[SecurityParameters.enc_key_length]
  //    server_write_key[SecurityParameters.enc_key_length]
  //    client_write_IV[SecurityParameters.fixed_iv_length]
  //    server_write_IV[SecurityParameters.fixed_iv_length]
  let mut tmp64: [uint8_t; 64] = [0; 64];
  /* make "server_rand32 + client_rand32" */
  memcpy(
    &mut *tmp64.as_mut_ptr().offset(0) as *mut uint8_t as *mut libc::c_void,
    &mut *(*(*tls).hsd)
      .client_and_server_rand32
      .as_mut_ptr()
      .offset(32) as *mut uint8_t as *const libc::c_void,
    32i32 as libc::c_ulong,
  );
  memcpy(
    &mut *tmp64.as_mut_ptr().offset(32) as *mut uint8_t as *mut libc::c_void,
    &mut *(*(*tls).hsd)
      .client_and_server_rand32
      .as_mut_ptr()
      .offset(0) as *mut uint8_t as *const libc::c_void,
    32i32 as libc::c_ulong,
  );
  prf_hmac_sha256(
    (*tls).client_write_MAC_key.as_mut_ptr(),
    (2i32 as libc::c_uint).wrapping_mul(
      (*tls)
        .MAC_size
        .wrapping_add((*tls).key_size)
        .wrapping_add((*tls).IV_size),
    ),
    (*(*tls).hsd).master_secret.as_mut_ptr(),
    ::std::mem::size_of::<[uint8_t; 48]>() as libc::c_ulong as libc::c_uint,
    b"key expansion\x00" as *const u8 as *const libc::c_char,
    tmp64.as_mut_ptr(),
    64i32 as libc::c_uint,
  );
  (*tls).client_write_key = (*tls)
    .client_write_MAC_key
    .as_mut_ptr()
    .offset((2i32 as libc::c_uint).wrapping_mul((*tls).MAC_size) as isize);
  (*tls).server_write_key = (*tls).client_write_key.offset((*tls).key_size as isize);
  (*tls).client_write_IV = (*tls).server_write_key.offset((*tls).key_size as isize);
  (*tls).server_write_IV = (*tls).client_write_IV.offset((*tls).IV_size as isize);
  aes_setkey(
    &mut (*tls).aes_decrypt,
    (*tls).server_write_key as *const libc::c_void,
    (*tls).key_size,
  );
  aes_setkey(
    &mut (*tls).aes_encrypt,
    (*tls).client_write_key as *const libc::c_void,
    (*tls).key_size,
  );
  let mut iv: [uint8_t; 16] = [0; 16];
  memset(
    iv.as_mut_ptr() as *mut libc::c_void,
    0i32,
    16i32 as libc::c_ulong,
  );
  aes_encrypt_one_block(
    &mut (*tls).aes_encrypt,
    iv.as_mut_ptr() as *const libc::c_void,
    (*tls).H.as_mut_ptr() as *mut libc::c_void,
  );
}
static mut rec_CHANGE_CIPHER_SPEC: [uint8_t; 6] = [
  20i32 as uint8_t,
  3i32 as uint8_t,
  3i32 as uint8_t,
  0i32 as uint8_t,
  0o1i32 as uint8_t,
  0o1i32 as uint8_t,
];
unsafe extern "C" fn send_change_cipher_spec(mut tls: *mut tls_state_t) {
  xwrite(
    (*tls).ofd,
    rec_CHANGE_CIPHER_SPEC.as_ptr() as *const libc::c_void,
    ::std::mem::size_of::<[uint8_t; 6]>() as libc::c_ulong,
  );
}
// 7.4.9.  Finished
// A Finished message is always sent immediately after a change
// cipher spec message to verify that the key exchange and
// authentication processes were successful.  It is essential that a
// change cipher spec message be received between the other handshake
// messages and the Finished message.
//...
// The Finished message is the first one protected with the just
// negotiated algorithms, keys, and secrets.  Recipients of Finished
// messages MUST verify that the contents are correct.  Once a side
// has sent its Finished message and received and validated the
// Finished message from its peer, it may begin to send and receive
// application data over the connection.
//...
// struct {
//     opaque verify_data[verify_data_length];
// } Finished;
//
// verify_data
//    PRF(master_secret, finished_label, Hash(handshake_messages))
//       [0..verify_data_length-1];
//
// finished_label
//    For Finished messages sent by the client, the string
//    "client finished".  For Finished messages sent by the server,
//    the string "server finished".
//
// Hash denotes a Hash of the handshake messages.  For the PRF
// defined in Section 5, the Hash MUST be the Hash used as the basis
// for the PRF.  Any cipher suite which defines a different PRF MUST
// also define the Hash to use in the Finished computation.
//
// In previous versions of TLS, the verify_data was always 12 octets
// long.  In the current version of TLS, it depends on the cipher
// suite.  Any cipher suite which does not explicitly specify
// verify_data_length has a verify_data_length equal to 12.  This
// includes all existing cipher suites.
unsafe extern "C" fn send_client_finished(mut tls: *mut tls_state_t) {
  let mut record: *mut finished = tls_get_outbuf(
    tls,
    ::std::mem::size_of::<finished>() as libc::c_ulong as libc::c_int,
  ) as *mut finished;
  let mut handshake_hash: [uint8_t; 32] = [0; 32];
  let mut len: libc::c_uint = 0;
  fill_handshake_record_hdr(
    record as *mut libc::c_void,
    20i32 as libc::c_uint,
    ::std::mem::size_of::<finished>() as libc::c_ulong as libc::c_uint,
  );
  len = sha1_end(
    &mut (*(*tls).hsd).handshake_hash_ctx,
    handshake_hash.as_mut_ptr() as *mut libc::c_void,
  );
  prf_hmac_sha256(
    (*record).prf_result.as_mut_ptr(),
    ::std::mem::size_of::<[uint8_t; 12]>() as libc::c_ulong as libc::c_uint,
    (*(*tls).hsd).master_secret.as_mut_ptr(),
    ::std::mem::size_of::<[uint8_t; 48]>() as libc::c_ulong as libc::c_uint,
    b"client finished\x00" as *const u8 as *const libc::c_char,
    handshake_hash.as_mut_ptr(),
    len,
  );
  xwrite_encrypted(
    tls,
    ::std::mem::size_of::<finished>() as libc::c_ulong as libc::c_uint,
    22i32 as libc::c_uint,
  );
}
#[no_mangle]
pub unsafe extern "C" fn tls_handshake(mut tls: *mut tls_state_t, mut sni: *const libc::c_char) {
  // Client              RFC 5246                Server
  // (*) - optional messages, not always sent
  //
  // ClientHello          ------->
  //                                        ServerHello
  //                                       Certificate*
  //                                 ServerKeyExchange*
  //                                CertificateRequest*
  //                      <-------      ServerHelloDone
  // Certificate*
  // ClientKeyExchange
  // CertificateVerify*
  // [ChangeCipherSpec]
  // Finished             ------->
  //                                 [ChangeCipherSpec]
  //                      <-------             Finished
  // Application Data     <------>     Application Data
  let mut len: libc::c_int = 0;
  let mut got_cert_req: libc::c_int = 0;
  send_client_hello_and_alloc_hsd(tls, sni);
  get_server_hello(tls);
  // RFC 5246
  // The server MUST send a Certificate message whenever the agreed-
  // upon key exchange method uses certificates for authentication
  // (this includes all key exchange methods defined in this document
  // except DH_anon).  This message will always immediately follow the
  // ServerHello message.
  //
  // IOW: in practice, Certificate *always* follows.
  // (for example, kernel.org does not even accept DH_anon cipher id)
  get_server_cert(tls);
  len = tls_xread_handshake_block(tls, 4i32);
  if *(*tls).inbuf.offset(RECHDR_LEN as libc::c_int as isize) as libc::c_int == 12i32 {
    // 459 bytes:
    // 0c   00|01|c7 03|00|17|41|04|87|94|2e|2f|68|d0|c9|f4|97|a8|2d|ef|ed|67|ea|c6|f3|b3|56|47|5d|27|b6|bd|ee|70|25|30|5e|b0|8e|f6|21|5a...
    //SvKey len=455^
    // with TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA: 461 bytes:
    // 0c   00|01|c9 03|00|17|41|04|cd|9b|b4|29|1f|f6|b0|c2|84|82|7f|29|6a|47|4e|ec|87|0b|c1|9c|69|e1|f8|c6|d0|53|e9|27|90|a5|c8|02|15|75...
    //
    // RFC 8422 5.4. Server Key Exchange
    // This message is sent when using the ECDHE_ECDSA, ECDHE_RSA, and
    // ECDH_anon key exchange algorithms.
    // This message is used to convey the server's ephemeral ECDH public key
    // (and the corresponding elliptic curve domain parameters) to the
    // client.
    if (*tls).flags & NEED_EC_KEY as libc::c_int as libc::c_uint != 0 {
      process_server_key(tls, len);
    }
    // read next handshake block
    len = tls_xread_handshake_block(tls, 4i32)
  }
  got_cert_req = (*(*tls).inbuf.offset(RECHDR_LEN as libc::c_int as isize) as libc::c_int == 13i32)
    as libc::c_int;
  if got_cert_req != 0 {
    // RFC 5246: "If no suitable certificate is available,
    // the client MUST send a certificate message containing no
    // certificates.  That is, the certificate_list structure has a
    // length of zero. ...
    // Client certificates are sent using the Certificate structure
    // defined in Section 7.4.2."
    // (i.e. the same format as server certs)
    /*send_empty_client_cert(tls); - WRONG (breaks handshake hash calc) */
    /* need to hash _all_ server replies first, up to ServerHelloDone */
    len = tls_xread_handshake_block(tls, 4i32)
  }
  if *(*tls).inbuf.offset(RECHDR_LEN as libc::c_int as isize) as libc::c_int != 14i32 {
    bad_record_die(
      tls,
      b"\'server hello done\'\x00" as *const u8 as *const libc::c_char,
      len,
    );
  }
  // 0e 000000 (len:0)
  if got_cert_req != 0 {
    send_empty_client_cert(tls);
  }
  send_client_key_exchange(tls);
  send_change_cipher_spec(tls);
  /* from now on we should send encrypted */
  /* tls->write_seq64_be = 0; - already is */
  (*tls).flags |= ENCRYPT_ON_WRITE as libc::c_int as libc::c_uint;
  send_client_finished(tls);
  /* Get CHANGE_CIPHER_SPEC */
  len = tls_xread_record(
    tls,
    b"switch to encrypted traffic\x00" as *const u8 as *const libc::c_char,
  );
  if len != 1i32
    || memcmp(
      (*tls).inbuf as *const libc::c_void,
      rec_CHANGE_CIPHER_SPEC.as_ptr() as *const libc::c_void,
      6i32 as libc::c_ulong,
    ) != 0i32
  {
    bad_record_die(
      tls,
      b"switch to encrypted traffic\x00" as *const u8 as *const libc::c_char,
      len,
    );
  }
  if 0i32 != 0 && (*tls).cipher_id as libc::c_int == 0x3bi32 {
    (*tls).min_encrypted_len_on_read = (*tls).MAC_size
  } else if (*tls).flags & ENCRYPTION_AESGCM as libc::c_int as libc::c_uint == 0 {
    let mut mac_blocks: libc::c_uint = ((SHA256_OUTSIZE as libc::c_int + 16i32 - 1i32)
      as libc::c_uint)
      .wrapping_div(16i32 as libc::c_uint);
    /* all incoming packets now should be encrypted and have
     * at least IV + (MAC padded to blocksize):
     */
    (*tls).min_encrypted_len_on_read =
      (16i32 as libc::c_uint).wrapping_add(mac_blocks.wrapping_mul(16i32 as libc::c_uint))
  } else {
    (*tls).min_encrypted_len_on_read = (8i32 + 16i32) as libc::c_uint
  }
  /* Get (encrypted) FINISHED from the server */
  len = tls_xread_record(
    tls,
    b"\'server finished\'\x00" as *const u8 as *const libc::c_char,
  );
  if len < 4i32 || *(*tls).inbuf.offset(RECHDR_LEN as libc::c_int as isize) as libc::c_int != 20i32
  {
    bad_record_die(
      tls,
      b"\'server finished\'\x00" as *const u8 as *const libc::c_char,
      len,
    );
  }
  /* application data can be sent/received */
  /* free handshake data */
  psRsaKey_clear(&mut (*(*tls).hsd).server_rsa_pub_key);
  //	if (PARANOIA)
  //		memset(tls->hsd, 0, tls->hsd->hsd_size);
  free((*tls).hsd as *mut libc::c_void);
  (*tls).hsd = 0 as *mut tls_handshake_data;
}
unsafe extern "C" fn tls_xwrite(mut tls: *mut tls_state_t, mut len: libc::c_int) {
  xwrite_encrypted(tls, len as libc::c_uint, 23i32 as libc::c_uint);
}

/*
 * Busybox main internal header file
 *
 * Based in part on code from sash, Copyright (c) 1999 by David I. Bell
 * Permission has been granted to redistribute this code under GPL.
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
/* TODO: and glibc? */
/* There are two incompatible basename's, let's not use them! */
/* See the dirname/basename man page for details */
/* dirname,basename */
/* Don't do this here:
 * #include <sys/sysinfo.h>
 * Some linux/ includes pull in conflicting definition
 * of struct sysinfo (only in some toolchanins), which breaks build.
 * Include sys/sysinfo.h only in those files which need it.
 */
/* Just in case libc doesn't define some of these... */
//This breaks on bionic:
//# if !defined(__socklen_t_defined) && !defined(_SOCKLEN_T_DECLARED)
// /* We #define socklen_t *after* includes, otherwise we get
// * typedef redefinition errors from system headers
// * (in case "is it defined already" detection above failed)
// */
//#  define socklen_t bb_socklen_t
//   typedef unsigned socklen_t;
//# endif
//if this is still needed, add a fix along the lines of
//  ifdef SPECIFIC_BROKEN_LIBC_CHECK / typedef socklen_t / endif
//in platform.h instead!
/*
 * Use '%m' to append error string on platforms that support it,
 * '%s' and strerror() on those that don't.
 */
/*nothing*/
/* Some libc's forget to declare these, do it ourself */
/* klogctl is in libc's klog.h, but we cheat and not #include that */
/* Busybox does not use threads, we can speed up stdio. */
/* Above functions are required by POSIX.1-2008, below ones are extensions */
/* musl <= 1.1.15 does not support fflush_unlocked(NULL) */
//# undef  fflush
//# define fflush(stream) fflush_unlocked(stream)
/* Make all declarations hidden (-fvisibility flag only affects definitions) */
/* (don't include system headers after this until corresponding pop!) */
/* Tested to work correctly with all int types (IIRC :]) */
/* Large file support */
/* Note that CONFIG_LFS=y forces bbox to be built with all common ops
 * (stat, lseek etc) mapped to "largefile" variants by libc.
 * Practically it means that open() automatically has O_LARGEFILE added
 * and all filesize/file_offset parameters and struct members are "large"
 * (in today's world - signed 64bit). For full support of large files,
 * we need a few helper #defines (below) and careful use of off_t
 * instead of int/ssize_t. No lseek64(), O_LARGEFILE etc necessary */
/* CONFIG_LFS is on */
/* "long" is long enough on this system */
/* usage: sz = BB_STRTOOFF(s, NULL, 10); if (errno || sz < 0) die(); */
/* usage: printf("size: %"OFF_FMT"d (%"OFF_FMT"x)\n", sz, sz); */
/* scary. better ideas? (but do *test* them first!) */
/* Users report bionic to use 32-bit off_t even if LARGEFILE support is requested.
 * We misdetected that. Don't let it build:
 */
/* Some useful definitions */
/* Macros for min/max.  */
/* buffer allocation schemes */
/* glibc uses __errno_location() to get a ptr to errno */
/* We can just memorize it once - no multithreading in busybox :) */
/* dmalloc will redefine these to it's own implementation. It is safe
 * to have the prototypes here unconditionally.  */
/* After v = xrealloc_vector(v, SHIFT, idx) it's ok to use
 * at least v[idx] and v[idx+1], for all idx values.
 * SHIFT specifies how many new elements are added (1:2, 2:4, ..., 8:256...)
 * when all elements are used up. New elements are zeroed out.
 * xrealloc_vector(v, SHIFT, idx) *MUST* be called with consecutive IDXs -
 * skipping an index is a bad bug - it may miss a realloc!
 */
//TODO: supply a pointer to char[11] buffer (avoid statics)?
/* cp.c, mv.c, install.c depend on these values. CAREFUL when changing them! */
/* -p */
/* !-d */
/* -R */
/* -f */
/* -i */
/* -l */
/* -s */
/* -L */
/* -H */
/* -a = -pdR (mapped in cp.c) */
/* -r = -dR  (mapped in cp.c) */
/* -P = -d   (mapped in cp.c) */
/* -v */
/* -u */
/* -T */
/* --remove-destination */
/* bit 17 skipped for "cp --parents" */
/* cp --reflink=auto */
/* cp --reflink[=always] */
/*
 * Hole. cp may have some bits set here,
 * they should not affect remove_file()/copy_file()
 */
/* NB: without FILEUTILS_RECUR in flags, it will basically "cat"
 * the source, not copy (unless "source" is a directory).
 * This makes "cp /dev/null file" and "install /dev/null file" (!!!)
 * work coreutils-compatibly. */
/*ACTION_REVERSE      = (1 << 4), - unused */
/* more than enough for "/dev/ttyXXX" */
/* bb_copyfd_XX print read/write errors and return -1 if they occur */
/* "short" copy can be detected by return value < size */
/* this helper yells "short read!" if param is not -1 */
/* xxxx_strip version can modify its parameter:
 * "/"        -> "/"
 * "abc"      -> "abc"
 * "abc/def"  -> "def"
 * "abc/def/" -> "def" !!
 */
/* "abc/def/" -> "" and it never modifies 'path' */
/* Simpler version: does not special case "/" string */
/* NB: can violate const-ness (similarly to strchr) */
/* !RETURNS_MALLOC: it's a realloc-like function */
/* bb_signals(BB_FATAL_SIGS, handler) catches all signals which
 * otherwise would kill us, except for those resulting from bugs:
 * SIGSEGV, SIGILL, SIGFPE.
 * Other fatal signals not included (TODO?):
 * SIGBUS   Bus error (bad memory access)
 * SIGPOLL  Pollable event. Synonym of SIGIO
 * SIGPROF  Profiling timer expired
 * SIGSYS   Bad argument to routine
 * SIGTRAP  Trace/breakpoint trap
 *
 * The only known arch with some of these sigs not fitting
 * into 32 bits is parisc (SIGXCPU=33, SIGXFSZ=34, SIGSTKFLT=36).
 * Dance around with long long to guard against that...
 */
// Write to pipe with no readers
// Quit from keyboard
// Abort signal from abort(3)
// Timer signal from alarm(2)
// Virtual alarm clock
// CPU time limit exceeded
// File size limit exceeded
// Yes kids, these are also fatal!
/* Unlike signal() and bb_signals, sets handler with sigaction()
 * and in a way that while signal handler is run, no other signals
 * will be blocked; syscalls will not be restarted: */
/* syscalls like read() will be interrupted with EINTR: */
/* syscalls like read() won't be interrupted (though select/poll will be): */
/* Will do sigaction(signum, act, NULL): */
/* SIG_BLOCK/SIG_UNBLOCK all signals: */
/* Return old set in the same set: */
/* Standard handler which just records signo */
/* not FAST_FUNC! */
/* In this form code with pipes is much more readable */
/* Useful for having small structure members/global variables */
/* | AF_DECnet */
/* | AF_IPX */
/* SO_REUSEADDR allows a server to rebind to an address that is already
 * "in use" by old connections to e.g. previous server instance which is
 * killed or crashed. Without it bind will fail until all such connections
 * time out. Linux does not allow multiple live binds on same ip:port
 * regardless of SO_REUSEADDR (unlike some other flavors of Unix).
 * Turn it on before you call bind(). */
/* On Linux this never fails. */
/* NB: returns port in host byte order */
/* Create stream socket, and allocate suitable lsa.
 * (lsa of correct size and lsa->sa.sa_family (AF_INET/AF_INET6))
 * af == AF_UNSPEC will result in trying to create IPv6 socket,
 * and if kernel doesn't support it, fall back to IPv4.
 * This is useful if you plan to bind to resulting local lsa.
 */
/* Create server socket bound to bindaddr:port. bindaddr can be NULL,
 * numeric IP ("N.N.N.N") or numeric IPv6 address,
 * and can have ":PORT" suffix (for IPv6 use "[X:X:...:X]:PORT").
 * Only if there is no suffix, port argument is used */
/* NB: these set SO_REUSEADDR before bind */
/* Create client TCP socket connected to peer:port. Peer cannot be NULL.
 * Peer can be numeric IP ("N.N.N.N"), numeric IPv6 address or hostname,
 * and can have ":PORT" suffix (for IPv6 use "[X:X:...:X]:PORT").
 * If there is no suffix, port argument is used */
/* Connect to peer identified by lsa */
/* Get local address of bound or accepted socket */
/* Get remote address of connected or accepted socket */
/* Return malloc'ed len_and_sockaddr with socket address of host:port
 * Currently will return IPv4 or IPv6 sockaddrs only
 * (depending on host), but in theory nothing prevents e.g.
 * UNIX socket address being returned, IPX sockaddr etc...
 * On error does bb_error_msg and returns NULL */
/* Version which dies on error */
/* Same, useful if you want to force family (e.g. IPv6) */
/* Assign sin[6]_port member if the socket is an AF_INET[6] one,
 * otherwise no-op. Useful for ftp.
 * NB: does NOT do htons() internally, just direct assignment. */
/* Retrieve sin[6]_port or return -1 for non-INET[6] lsa's */
/* Reverse DNS. Returns NULL on failure. */
/* This one doesn't append :PORTNUM */
/* This one also doesn't fall back to dotted IP (returns NULL) */
/* inet_[ap]ton on steroids */
// "old" (ipv4 only) API
// users: traceroute.c hostname.c - use _list_ of all IPs
// Also mount.c and inetd.c are using gethostbyname(),
// + inet_common.c has additional IPv4-only stuff
/* opaque */
// RFC 5246
// sequence number
//   Each connection state contains a sequence number, which is
//   maintained separately for read and write states.  The sequence
//   number MUST be set to zero whenever a connection state is made the
//   active state.  Sequence numbers are of type uint64 and may not
//   exceed 2^64-1.
/*uint64_t read_seq64_be;*/
/*uint8_t *server_write_MAC_key;*/
//used by AES_GCM
// To run a test server using openssl:
// openssl req -x509 -newkey rsa:$((4096/4*3)) -keyout key.pem -out server.pem -nodes -days 99999 -subj '/CN=localhost'
// openssl s_server -key key.pem -cert server.pem -debug -tls1_2
//
// Unencryped SHA256 example:
// openssl req -x509 -newkey rsa:$((4096/4*3)) -keyout key.pem -out server.pem -nodes -days 99999 -subj '/CN=localhost'
// openssl s_server -key key.pem -cert server.pem -debug -tls1_2 -cipher NULL
// openssl s_client -connect 127.0.0.1:4433 -debug -tls1_2 -cipher NULL-SHA256
#[no_mangle]
pub unsafe extern "C" fn tls_run_copy_loop(mut tls: *mut tls_state_t, mut flags: libc::c_uint) {
  let mut inbuf_size: libc::c_int = 0;
  let INBUF_STEP: libc::c_int = 4i32 * 1024i32;
  let mut pfds: [pollfd; 2] = [pollfd {
    fd: 0,
    events: 0,
    revents: 0,
  }; 2];
  pfds[0].fd = 0i32;
  pfds[0].events = 0x1i32 as libc::c_short;
  pfds[1].fd = (*tls).ifd;
  pfds[1].events = 0x1i32 as libc::c_short;
  inbuf_size = INBUF_STEP;
  's_36: loop {
    let mut nread: libc::c_int = 0;
    if safe_poll(pfds.as_mut_ptr(), 2i32 as nfds_t, -1i32) < 0i32 {
      bb_simple_perror_msg_and_die(b"poll\x00" as *const u8 as *const libc::c_char);
    }
    if pfds[0].revents != 0 {
      let mut buf: *mut libc::c_void = 0 as *mut libc::c_void;
      buf = tls_get_outbuf(tls, inbuf_size);
      nread = safe_read(0i32, buf, inbuf_size as size_t) as libc::c_int;
      if nread < 1i32 {
        /* We'd want to do this: */
        /* Close outgoing half-connection so they get EOF,
         * but leave incoming alone so we can see response
         */
        //shutdown(tls->ofd, SHUT_WR);
        /* But TLS has no way to encode this,
         * doubt it's ok to do it "raw"
         */
        pfds[0].fd = -1i32; /* mem usage optimization */
        tls_free_outbuf(tls);
        if flags & (1i32 << 0i32) as libc::c_uint != 0 {
          break;
        }
      } else {
        if nread == inbuf_size {
          /* TLS has per record overhead, if input comes fast,
           * read, encrypt and send bigger chunks
           */
          inbuf_size += INBUF_STEP;
          if inbuf_size > 1i32 << 14i32 {
            inbuf_size = 1i32 << 14i32
          }
        }
        tls_xwrite(tls, nread);
      }
    }
    if !(pfds[1].revents != 0) {
      continue;
    }
    loop {
      nread = tls_xread_record(
        tls,
        b"encrypted data\x00" as *const u8 as *const libc::c_char,
      );
      if nread < 1i32 {
        break 's_36;
      }
      if *(*tls).inbuf.offset(0) as libc::c_int != 23i32 {
        bad_record_die(
          tls,
          b"encrypted data\x00" as *const u8 as *const libc::c_char,
          nread,
        );
      }
      xwrite(
        1i32,
        (*tls).inbuf.offset(RECHDR_LEN as libc::c_int as isize) as *const libc::c_void,
        nread as size_t,
      );
      /* We may already have a complete next record buffered,
       * can process it without network reads (and possible blocking)
       */
      if !(tls_has_buffered_record(tls) != 0) {
        break;
      }
    }
  }
}
