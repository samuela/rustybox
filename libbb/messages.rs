use libc;

// TODO: fix all of these magic numbers

// const char bb_banner[] ALIGN1 = "BusyBox v" BB_VER BB_EXTRA_VERSION;
#[no_mangle]
pub static mut bb_banner: [libc::c_char; 46] = [
  66, 117, 115, 121, 66, 111, 120, 32, 118, 49, 46, 51, 50, 46, 48, 46, 103, 105, 116, 32, 40, 50,
  48, 49, 57, 45, 49, 48, 45, 51, 48, 32, 50, 48, 58, 49, 50, 58, 50, 55, 32, 85, 84, 67, 41, 0,
];

// "out of memory"
#[no_mangle]
pub static mut bb_msg_memory_exhausted: [libc::c_char; 14] = [
  111, 117, 116, 32, 111, 102, 32, 109, 101, 109, 111, 114, 121, 0,
];

// "invalid date '%s'"
#[no_mangle]
pub static mut bb_msg_invalid_date: [libc::c_char; 18] = [
  105, 110, 118, 97, 108, 105, 100, 32, 100, 97, 116, 101, 32, 39, 37, 115, 39, 0,
];

// "(unknown)"
#[no_mangle]
pub static mut bb_msg_unknown: [libc::c_char; 10] = [40, 117, 110, 107, 110, 111, 119, 110, 41, 0];

// "can't create raw socket"
#[no_mangle]
pub static mut bb_msg_can_not_create_raw_socket: [libc::c_char; 24] = [
  99, 97, 110, 39, 116, 32, 99, 114, 101, 97, 116, 101, 32, 114, 97, 119, 32, 115, 111, 99, 107,
  101, 116, 0,
];

// "permission denied (are you root?)"
#[no_mangle]
pub static mut bb_msg_perm_denied_are_you_root: [libc::c_char; 34] = [
  112, 101, 114, 109, 105, 115, 115, 105, 111, 110, 32, 100, 101, 110, 105, 101, 100, 32, 40, 97,
  114, 101, 32, 121, 111, 117, 32, 114, 111, 111, 116, 63, 41, 0,
];

// "you must be root"
#[no_mangle]
pub static mut bb_msg_you_must_be_root: [libc::c_char; 17] = [
  121, 111, 117, 32, 109, 117, 115, 116, 32, 98, 101, 32, 114, 111, 111, 116, 0,
];

// "%s requires an argument"
#[no_mangle]
pub static mut bb_msg_requires_arg: [libc::c_char; 24] = [
  37, 115, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 97, 114, 103, 117, 109,
  101, 110, 116, 0,
];

// "invalid argument '%s' to '%s'"
#[no_mangle]
pub static mut bb_msg_invalid_arg_to: [libc::c_char; 30] = [
  105, 110, 118, 97, 108, 105, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32, 39, 37, 115, 39,
  32, 116, 111, 32, 39, 37, 115, 39, 0,
];

// "standard input"
#[no_mangle]
pub static mut bb_msg_standard_input: [libc::c_char; 15] = [
  115, 116, 97, 110, 100, 97, 114, 100, 32, 105, 110, 112, 117, 116, 0,
];

// "standard output"
#[no_mangle]
pub static mut bb_msg_standard_output: [libc::c_char; 16] = [
  115, 116, 97, 110, 100, 97, 114, 100, 32, 111, 117, 116, 112, 117, 116, 0,
];

// "0123456789ABCDEF"
#[no_mangle]
pub static mut bb_hexdigits_upcase: [libc::c_char; 17] = [
  48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 65, 66, 67, 68, 69, 70, 0,
];

#[no_mangle]
pub static mut bb_busybox_exec_path: [libc::c_char; 15] = [
  47, 112, 114, 111, 99, 47, 115, 101, 108, 102, 47, 101, 120, 101, 0,
];

#[no_mangle]
pub static mut bb_default_login_shell: [libc::c_char; 9] = [45, 47, 98, 105, 110, 47, 115, 104, 0];

/* util-linux manpage says /sbin:/bin:/usr/sbin:/usr/bin,
 * but I want to save a few bytes here. Check libbb.h before changing! */
#[no_mangle]
pub static mut bb_PATH_root_path: [libc::c_char; 35] = [
  80, 65, 84, 72, 61, 47, 115, 98, 105, 110, 58, 47, 117, 115, 114, 47, 115, 98, 105, 110, 58, 47,
  98, 105, 110, 58, 47, 117, 115, 114, 47, 98, 105, 110, 0,
];

// Why the hell does this even exist?
/* explicitly = 0, otherwise gcc may make it a common variable
 * and it will end up in bss */
#[no_mangle]
pub static mut const_int_0: libc::c_int = 0i32;

/* This is usually something like "/var/adm/wtmp" or "/var/log/wtmp" */
#[no_mangle]
pub static mut bb_path_wtmp_file: [libc::c_char; 14] = [
  47, 118, 97, 114, 47, 108, 111, 103, 47, 119, 116, 109, 112, 0,
];
