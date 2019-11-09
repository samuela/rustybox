use libc;
use libc::unlink;


extern "C" {
  #[no_mangle]
  fn putenv(__string: *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn clearenv() -> libc::c_int;
  #[no_mangle]
  static mut environ: *mut *mut libc::c_char;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn puts(__s: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn fflush_stdout_and_exit(retval: libc::c_int) -> !;
  #[no_mangle]
  fn BB_EXECVP_or_die(argv: *mut *mut libc::c_char) -> !;
  #[no_mangle]
  fn getopt32long(
    argv: *mut *mut libc::c_char,
    optstring: *const libc::c_char,
    longopts: *const libc::c_char,
    _: ...
  ) -> u32;
  #[no_mangle]
  fn llist_pop(elm: *mut *mut llist_t) -> *mut libc::c_void;
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
}
use crate::libbb::llist::llist_t;



/*
 * env implementation for busybox
 *
 * Copyright (c) 1988, 1993, 1994
 * The Regents of the University of California.  All rights reserved.
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 *
 * Original copyright notice is retained at the end of this file.
 *
 * Modified for BusyBox by Erik Andersen <andersen@codepoet.org>
 */
/* Mar 16, 2003      Manuel Novoa III   (mjn3@codepoet.org)
 *
 * Fixed bug involving exit return codes if execvp fails.  Also added
 * output error checking.
 */
/*
 * Modified by Vladimir Oleynik <dzo@simtreas.ru> (C) 2003
 * - correct "-" option usage
 * - multiple "-u unsetenv" support
 * - GNU long option support
 * - use xfunc_error_retval
 */
//config:config ENV
//config:	bool "env (4 kb)"
//config:	default y
//config:	help
//config:	env is used to set an environment variable and run
//config:	a command; without options it displays the current
//config:	environment.
//applet:IF_ENV(APPLET_NOEXEC(env, env, BB_DIR_USR_BIN, BB_SUID_DROP, env))
//kbuild:lib-$(CONFIG_ENV) += env.o
/* BB_AUDIT SUSv3 compliant */
/* http://www.opengroup.org/onlinepubs/007904975/utilities/env.html */
//usage:#define env_trivial_usage
//usage:       "[-iu] [-] [name=value]... [PROG ARGS]"
//usage:#define env_full_usage "\n\n"
//usage:       "Print the current environment or run PROG after setting up\n"
//usage:       "the specified environment\n"
//usage:     "\n	-, -i	Start with an empty environment"
//usage:     "\n	-u	Remove variable from the environment"
#[no_mangle]
pub unsafe extern "C" fn env_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut opts: libc::c_uint = 0;
  let mut unset_env: *mut llist_t = 0 as *mut llist_t;
  opts = getopt32long(
    argv,
    b"+iu:*\x00" as *const u8 as *const libc::c_char,
    b"ignore-environment\x00\x00iunset\x00\x01u\x00" as *const u8 as *const libc::c_char,
    &mut unset_env as *mut *mut llist_t,
  );
  argv = argv.offset(optind as isize);
  if !(*argv.offset(0)).is_null()
    && (*(*argv.offset(0)).offset(0) as libc::c_int == '-' as i32
      && *(*argv.offset(0)).offset(1) == 0)
  {
    opts |= 1i32 as libc::c_uint;
    argv = argv.offset(1)
  }
  if opts & 1i32 as libc::c_uint != 0 {
    clearenv();
  }
  while !unset_env.is_null() {
    let mut var: *mut libc::c_char = llist_pop(&mut unset_env) as *mut libc::c_char;
    /* This does not handle -uVAR=VAL
     * (coreutils _sets_ the variable in that case): */
    /*unsetenv(var);*/
    /* This does, but uses somewhan undocumented feature that
     * putenv("name_without_equal_sign") unsets the variable: */
    putenv(var);
  }
  while !(*argv).is_null() && !strchr(*argv, '=' as i32).is_null() {
    if putenv(*argv) < 0i32 {
      bb_simple_perror_msg_and_die(b"putenv\x00" as *const u8 as *const libc::c_char);
    }
    argv = argv.offset(1)
  }
  if !(*argv.offset(0)).is_null() {
    BB_EXECVP_or_die(argv);
  }
  if !environ.is_null() {
    /* clearenv() may set environ == NULL! */
    let mut ep: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    ep = environ;
    while !(*ep).is_null() {
      puts(*ep);
      ep = ep.offset(1)
    }
  }
  fflush_stdout_and_exit(0i32);
}
/*
 * Copyright (c) 1988, 1993, 1994
 *      The Regents of the University of California.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * 3. BSD Advertising Clause omitted per the July 22, 1999 licensing change
 *    ftp://ftp.cs.berkeley.edu/pub/4bsd/README.Impt.License.Change
 *
 * 4. Neither the name of the University nor the names of its contributors
 *    may be used to endorse or promote products derived from this software
 *    without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE REGENTS AND CONTRIBUTORS ''AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE REGENTS OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */
