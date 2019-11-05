use libc;
extern "C" {
  #[no_mangle]
  static mut optarg: *mut libc::c_char;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn getopt_long(
    ___argc: libc::c_int,
    ___argv: *const *mut libc::c_char,
    __shortopts: *const libc::c_char,
    __longopts: *const option,
    __longind: *mut libc::c_int,
  ) -> libc::c_int;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn string_array_len(argv: *mut *mut libc::c_char) -> libc::c_uint;
  #[no_mangle]
  fn overlapping_strcpy(dst: *mut libc::c_char, src: *const libc::c_char);
  #[no_mangle]
  fn xatoi_positive(numstr: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn llist_add_to_end(list_head: *mut *mut llist_t, data: *mut libc::c_void);
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
  pub name: *const libc::c_char,
  pub has_arg: libc::c_int,
  pub flag: *mut libc::c_int,
  pub val: libc::c_int,
}

use crate::librb::size_t;
use crate::librb::smallint;
use crate::librb::uint32_t;
pub type va_list = __builtin_va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct t_complementary {
  pub opt_char: libc::c_uchar,
  pub param_type: smallint,
  pub switch_on: libc::c_uint,
  pub switch_off: libc::c_uint,
  pub incongruously: libc::c_uint,
  pub requires: libc::c_uint,
  pub optarg: *mut *mut libc::c_void,
  pub counter: *mut libc::c_int,
}
pub const PARAM_INT: C2RustUnnamed = 2;
use crate::libbb::llist::llist_t;
pub const PARAM_LIST: C2RustUnnamed = 1;
pub type C2RustUnnamed = libc::c_uint;
pub const PARAM_STRING: C2RustUnnamed = 0;

/*
 * universal getopt32 implementation for busybox
 *
 * Copyright (C) 2003-2005  Vladimir Oleynik  <dzo@simtreas.ru>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//kbuild:lib-y += getopt32.o
/*      Documentation

uint32_t
getopt32(char **argv, const char *applet_opts, ...)

        The command line options are passed as the applet_opts string.

        If one of the given options is found, a flag value is added to
        the return value.

        The flag value is determined by the position of the char in
        applet_opts string.  For example:

        flags = getopt32(argv, "rnug");

        "r" will set 1    (bit 0)
        "n" will set 2    (bit 1)
        "u" will set 4    (bit 2)
        "g" will set 8    (bit 3)

        and so on.  You can also look at the return value as a bit
        field and each option sets one bit.

        On exit, global variable optind is set so that if you
        will do argc -= optind; argv += optind; then
        argc will be equal to number of remaining non-option
        arguments, first one would be in argv[0], next in argv[1] and so on
        (options and their parameters will be moved into argv[]
        positions prior to argv[optind]).

 "o:"   If one of the options requires an argument, then add a ":"
        after the char in applet_opts and provide a pointer to store
        the argument.  For example:

        char *pointer_to_arg_for_a;
        char *pointer_to_arg_for_b;
        char *pointer_to_arg_for_c;
        char *pointer_to_arg_for_d;

        flags = getopt32(argv, "a:b:c:d:",
                        &pointer_to_arg_for_a, &pointer_to_arg_for_b,
                        &pointer_to_arg_for_c, &pointer_to_arg_for_d);

        The type of the pointer may be controlled by "o::" or "o+" in
        the external string opt_complementary (see below for more info).

 "o::"  If option can have an *optional* argument, then add a "::"
        after its char in applet_opts and provide a pointer to store
        the argument.  Note that optional arguments _must_
        immediately follow the option: -oparam, not -o param.

 "o:+"  This means that the parameter for this option is a nonnegative integer.
        It will be processed with xatoi_positive() - allowed range
        is 0..INT_MAX.

        int param;  // "unsigned param;" will also work
        getopt32(argv, "p:+", &param);

 "o:*"  This means that the option can occur multiple times. Each occurrence
        will be saved as a llist_t element instead of char*.

        For example:
        The grep applet can have one or more "-e pattern" arguments.
        In this case you should use getopt32() as follows:

        llist_t *patterns = NULL;

        (this pointer must be initializated to NULL if the list is empty
        as required by llist_add_to_end(llist_t **old_head, char *new_item).)

        getopt32(argv, "e:*", &patterns);

        $ grep -e user -e root /etc/passwd
        root:x:0:0:root:/root:/bin/bash
        user:x:500:500::/home/user:/bin/bash

 "+"    If the first character in the applet_opts string is a plus,
        then option processing will stop as soon as a non-option is
        encountered in the argv array.  Useful for applets like env
        which should not process arguments to subprograms:
        env -i ls -d /
        Here we want env to process just the '-i', not the '-d'.

 "!"    Report bad options, missing required options,
        inconsistent options with all-ones return value (instead of abort).

 "^"    options string is "^optchars""\0""opt_complementary".

uint32_t
getopt32long(char **argv, const char *applet_opts, const char *logopts...)

        This allows you to define long options:

        static const char applet_longopts[] ALIGN1 =
                //"name\0"  has_arg     val
                "verbose\0" No_argument "v"
                ;
        opt = getopt32long(argv, applet_opts, applet_longopts, ...);

        The last element (val) typically is set to
        matching short option from applet_opts. If there is no matching
        char in applet_opts, then:
        - return bit has next position after short options
        - if has_arg is not "No_argument", use ptr for arg also
        - opt_complementary affects it too

        Note: a good applet will make long options configurable via the
        config process and not a required feature.  The current standard
        is to name the config option CONFIG_FEATURE_<applet>_LONG_OPTIONS.

opt_complementary - option modifiers.

 ":"    The colon (":") is used to separate groups of two or more chars
        and/or groups of chars and special characters (stating some
        conditions to be checked).

 "abc"  If groups of two or more chars are specified, the first char
        is the main option and the other chars are secondary options.
        Their flags will be turned on if the main option is found even
        if they are not specified on the command line.  For example:

        flags = getopt32(argv, "^abcd""\0""abc")

        If getopt() finds "-a" on the command line, then
        getopt32's return value will be as if "-a -b -c" were
        found.

 "ww"   Adjacent double options have a counter associated which indicates
        the number of occurrences of the option.
        For example the ps applet needs:
        if w is given once, GNU ps sets the width to 132,
        if w is given more than once, it is "unlimited"

        int w_counter = 0; // must be initialized!
        getopt32(argv, "^w""\0""ww", &w_counter);
        if (w_counter)
                width = (w_counter == 1) ? 132 : INT_MAX;
        else
                get_terminal_width(...&width...);

        w_counter is a pointer to an integer. It has to be passed to
        getopt32() after all other option argument sinks.

        For example: accept multiple -v to indicate the level of verbosity
        and for each -b optarg, add optarg to my_b. Finally, if b is given,
        turn off c and vice versa:

        llist_t *my_b = NULL;
        int verbose_level = 0;
        f = getopt32(argv, "^vb:*c"
      "\0""vv:b-c:c-b"
      , &my_b, &verbose_level);
        if (f & 2)       // -c after -b unsets -b flag
                while (my_b) dosomething_with(llist_pop(&my_b));
        if (my_b)        // but llist is stored if -b is specified
                free_llist(my_b);
        if (verbose_level) printf("verbose level is %d\n", verbose_level);

Special characters:

 "-N"   A dash as the first char in a opt_complementary group followed
        by a single digit (0-9) means that at least N non-option
        arguments must be present on the command line

 "=N"   An equal sign as the first char in a opt_complementary group followed
        by a single digit (0-9) means that exactly N non-option
        arguments must be present on the command line

 "?N"   A "?" as the first char in a opt_complementary group followed
        by a single digit (0-9) means that at most N arguments must be present
        on the command line.

 "V-"   An option with dash before colon or end-of-line results in
        bb_show_usage() being called if this option is encountered.
        This is typically used to implement "print verbose usage message
        and exit" option.

 "a-b"  A dash between two options causes the second of the two
        to be unset (and ignored) if it is given on the command line.

        [FIXME: what if they are the same? like "x-x"? Is it ever useful?]

        For example:
        The du applet has the options "-s" and "-d depth".  If
        getopt32 finds -s, then -d is unset or if it finds -d
        then -s is unset.  (Note:  busybox implements the GNU
        "--max-depth" option as "-d".)  To obtain this behavior, you
        set opt_complementary to "s-d:d-s".  Only one flag value is
        added to getopt32's return value depending on the
        position of the options on the command line.  If one of the
        two options requires an argument pointer (":" in applet_opts
        as in "d:") optarg is set accordingly.

        char *smax_print_depth;

        opt = getopt32(argv, "^sd:x""\0""s-d:d-s:x-x", &smax_print_depth);

        if (opt & 2)
                max_print_depth = atoi(smax_print_depth);
        if (opt & 4)
                printf("Detected odd -x usage\n");

 "a--b" A double dash between two options, or between an option and a group
        of options, means that they are mutually exclusive.  Unlike
        the "-" case above, an error will be forced if the options
        are used together.

        For example:
        The cut applet must have only one type of list specified, so
        -b, -c and -f are mutually exclusive and should raise an error
        if specified together.  In this case you must set
        opt_complementary to "b--cf:c--bf:f--bc".  If two of the
        mutually exclusive options are found, getopt32 will call
        bb_show_usage() and die.

 "x--x" Variation of the above, it means that -x option should occur
        at most once.

 "o+"   A plus after a char in opt_complementary means that the parameter
        for this option is a nonnegative integer. It will be processed
        with xatoi_positive() - allowed range is 0..INT_MAX.

        int param;  // "unsigned param;" will also work
        getopt32(argv, "^p:""\0""p+", &param);

 "o::"  A double colon after a char in opt_complementary means that the
        option can occur multiple times. Each occurrence will be saved as
        a llist_t element instead of char*.

        For example:
        The grep applet can have one or more "-e pattern" arguments.
        In this case you should use getopt32() as follows:

        llist_t *patterns = NULL;

        (this pointer must be initializated to NULL if the list is empty
        as required by llist_add_to_end(llist_t **old_head, char *new_item).)

        getopt32(argv, "^e:""\0""e::", &patterns);

        $ grep -e user -e root /etc/passwd
        root:x:0:0:root:/root:/bin/bash
        user:x:500:500::/home/user:/bin/bash

        "o+" and "o::" can be handled by "o:+" and "o:*" specifiers
        in option string (and it is preferred), but this does not work
        for "long options only" cases, such as tar --exclude=PATTERN,
        wget --header=HDR cases.

 "a?b"  A "?" between an option and a group of options means that
        at least one of them is required to occur if the first option
        occurs in preceding command line arguments.

        For example from "id" applet:

        // Don't allow -n -r -rn -ug -rug -nug -rnug
        flags = getopt32(argv, "^rnug""\0""r?ug:n?ug:u--g:g--u");

        This example allowed only:
        $ id; id -u; id -g; id -ru; id -nu; id -rg; id -ng; id -rnu; id -rng

 "X"    A opt_complementary group with just a single letter means
        that this option is required. If more than one such group exists,
        at least one option is required to occur (not all of them).
        For example from "start-stop-daemon" applet:

        // Don't allow -KS -SK, but -S or -K is required
        flags = getopt32(argv, "^KS...""\0""K:S:K--S:S--K");


        Don't forget to use ':'. For example, "?322-22-23X-x-a"
        is interpreted as "?3:22:-2:2-2:2-3Xa:2--x" -
        max 3 args; count uses of '-2'; min 2 args; if there is
        a '-2' option then unset '-3', '-X' and '-a'; if there is
        a '-2' and after it a '-x' then error out.
        But it's far too obfuscated. Use ':' to separate groups.
*/
/* Code here assumes that 'unsigned' is at least 32 bits wide */
#[no_mangle]
pub static mut bb_argv_dash: [*const libc::c_char; 2] = [
  b"-\x00" as *const u8 as *const libc::c_char,
  0 as *const libc::c_char,
];
#[no_mangle]
pub static mut option_mask32: uint32_t = 0;
static mut bb_null_long_options: [option; 1] = [{
  let mut init = option {
    name: 0 as *const libc::c_char,
    has_arg: 0i32,
    flag: 0 as *const libc::c_int as *mut libc::c_int,
    val: 0i32,
  };
  init
}];
/* Please keep getopt32 free from xmalloc */
unsafe extern "C" fn vgetopt32(
  mut argv: *mut *mut libc::c_char,
  mut applet_opts: *const libc::c_char,
  mut applet_long_options: *const libc::c_char,
  mut p: ::std::ffi::VaList,
) -> uint32_t {
  let mut current_block: u64; /* last stays zero-filled */
  let mut argc: libc::c_int = 0;
  let mut flags: libc::c_uint = 0i32 as libc::c_uint;
  let mut requires: libc::c_uint = 0i32 as libc::c_uint;
  let mut len: libc::c_uint = 0;
  let mut complementary: [t_complementary; 33] = [t_complementary {
    opt_char: 0,
    param_type: 0,
    switch_on: 0,
    switch_off: 0,
    incongruously: 0,
    requires: 0,
    optarg: 0 as *mut *mut libc::c_void,
    counter: 0 as *mut libc::c_int,
  }; 33];
  let mut dont_die_flag: libc::c_char = 0;
  let mut c: libc::c_int = 0;
  let mut s: *const libc::c_uchar = 0 as *const libc::c_uchar;
  let mut opt_complementary: *const libc::c_char = 0 as *const libc::c_char;
  let mut on_off: *mut t_complementary = 0 as *mut t_complementary;
  let mut l_o: *const option = 0 as *const option;
  let mut long_options: *mut option = &bb_null_long_options as *const [option; 1] as *mut option;
  let mut trigger: libc::c_uint = 0;
  let mut min_arg: libc::c_int = 0i32;
  let mut max_arg: libc::c_int = -1i32;
  let mut _spec_flgs: libc::c_int = 0i32; // assigned to but never used
  on_off = complementary.as_mut_ptr();
  memset(
    on_off as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<[t_complementary; 33]>() as libc::c_ulong,
  );
  len = strlen(applet_opts) as libc::c_uint;
  /* skip bbox extension */
  opt_complementary = 0 as *const libc::c_char;
  if *applet_opts.offset(0) as libc::c_int == '^' as i32 {
    applet_opts = applet_opts.offset(1);
    /* point it past terminating NUL */
    opt_complementary = applet_opts.offset(len as isize)
  }
  /* skip another bbox extension */
  dont_die_flag = *applet_opts.offset(0);
  if dont_die_flag as libc::c_int == '!' as i32 {
    applet_opts = applet_opts.offset(1)
  }
  let mut fresh0 = ::std::vec::from_elem(
    0,
    len.wrapping_add(1i32 as libc::c_uint) as libc::c_ulong as usize,
  );
  applet_opts = strcpy(fresh0.as_mut_ptr() as *mut libc::c_char, applet_opts);
  /* skip GNU extension */
  s = applet_opts as *const libc::c_uchar;
  if *s as libc::c_int == '+' as i32 || *s as libc::c_int == '-' as i32 {
    s = s.offset(1)
  }
  c = 0i32;
  while *s != 0 {
    if c >= 32i32 {
      break;
    }
    (*on_off).opt_char = *s;
    (*on_off).switch_on = 1u32 << c;
    s = s.offset(1);
    if *s as libc::c_int == ':' as i32 {
      (*on_off).optarg = p.arg::<*mut *mut libc::c_void>();
      if *s.offset(1) as libc::c_int == '+' as i32 || *s.offset(1) as libc::c_int == '*' as i32 {
        /* 'o:+' or 'o:*' */
        (*on_off).param_type = if *s.offset(1) as libc::c_int == '+' as i32 {
          PARAM_INT as libc::c_int
        } else {
          PARAM_LIST as libc::c_int
        } as smallint;
        overlapping_strcpy(
          (s as *mut libc::c_char).offset(1),
          (s as *mut libc::c_char).offset(2),
        );
      }
      loop
      /* skip possible 'o::' (or 'o:+:' !) */
      {
        s = s.offset(1); /* skip NUL, has_arg, val */
        if !(*s as libc::c_int == ':' as i32) {
          break;
        }
      }
    }
    on_off = on_off.offset(1);
    c += 1
  }
  if !applet_long_options.is_null() {
    let mut optstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_uint = 0;
    let mut count: libc::c_uint = 0;
    count = 1i32 as libc::c_uint;
    optstr = applet_long_options;
    while *optstr.offset(0) != 0 {
      optstr = optstr.offset(strlen(optstr).wrapping_add(3i32 as libc::c_ulong) as isize);
      count = count.wrapping_add(1)
    }
    /* count == no. of longopts + 1 */
    let mut fresh1 = ::std::vec::from_elem(
      0,
      (count as libc::c_ulong).wrapping_mul(::std::mem::size_of::<option>() as libc::c_ulong)
        as usize,
    );
    long_options = fresh1.as_mut_ptr() as *mut option;
    memset(
      long_options as *mut libc::c_void,
      0i32,
      (count as libc::c_ulong).wrapping_mul(::std::mem::size_of::<option>() as libc::c_ulong),
    );
    i = 0i32 as libc::c_uint;
    optstr = applet_long_options;
    loop {
      count = count.wrapping_sub(1);
      if !(count != 0) {
        break;
      }
      let ref mut fresh2 = (*long_options.offset(i as isize)).name;
      *fresh2 = optstr;
      optstr = optstr.offset(strlen(optstr).wrapping_add(1i32 as libc::c_ulong) as isize);
      let fresh3 = optstr;
      optstr = optstr.offset(1);
      (*long_options.offset(i as isize)).has_arg = *fresh3 as libc::c_uchar as libc::c_int;
      /* long_options[i].flag = NULL; */
      let fresh4 = optstr;
      optstr = optstr.offset(1);
      (*long_options.offset(i as isize)).val = *fresh4 as libc::c_uchar as libc::c_int;
      i = i.wrapping_add(1)
    }
    let mut current_block_53: u64;
    l_o = long_options;
    while !(*l_o).name.is_null() {
      if (*l_o).flag.is_null() {
        on_off = complementary.as_mut_ptr();
        loop {
          if !((*on_off).opt_char != 0) {
            current_block_53 = 17836213544692497527;
            break;
          }
          if (*on_off).opt_char as libc::c_int == (*l_o).val {
            current_block_53 = 17233182392562552756;
            break;
          }
          on_off = on_off.offset(1)
        }
        match current_block_53 {
          17233182392562552756 => {}
          _ => {
            if c >= 32i32 {
              break;
            }
            (*on_off).opt_char = (*l_o).val as libc::c_uchar;
            (*on_off).switch_on = 1u32 << c;
            if (*l_o).has_arg != 0i32 {
              (*on_off).optarg = p.arg::<*mut *mut libc::c_void>()
            }
            c += 1
          }
        }
      }
      l_o = l_o.offset(1)
    }
  }
  /* ENABLE_LONG_OPTS */
  s = opt_complementary as *const libc::c_uchar;
  if !s.is_null() {
    let mut current_block_101: u64;
    while *s != 0 {
      let mut pair: *mut t_complementary = 0 as *mut t_complementary;
      let mut pair_switch: *mut libc::c_uint = 0 as *mut libc::c_uint;
      if !(*s as libc::c_int == ':' as i32) {
        c = *s.offset(1) as libc::c_int;
        if *s as libc::c_int == '?' as i32 {
          if c < '0' as i32 || c > '9' as i32 {
            _spec_flgs |= 1i32
          } else {
            max_arg = c - '0' as i32;
            s = s.offset(1)
          }
        } else if *s as libc::c_int == '-' as i32 {
          if c >= '0' as i32 && c <= '9' as i32 {
            min_arg = c - '0' as i32;
            s = s.offset(1)
          }
        } else if *s as libc::c_int == '=' as i32 {
          max_arg = c - '0' as i32;
          min_arg = max_arg;
          s = s.offset(1)
        } else {
          on_off = complementary.as_mut_ptr();
          loop {
            if !((*on_off).opt_char != 0) {
              current_block_101 = 10411727741569490626;
              break;
            }
            if (*on_off).opt_char as libc::c_int == *s as libc::c_int {
              current_block_101 = 1332384019521138159;
              break;
            }
            on_off = on_off.offset(1)
          }
          match current_block_101 {
            10411727741569490626 => {
              /* Without this, diagnostic of such bugs is not easy */
              bb_error_msg_and_die(
                b"NO OPT %c!\x00" as *const u8 as *const libc::c_char,
                *s as libc::c_int,
              );
            }
            _ => {
              if c == ':' as i32 && *s.offset(2) as libc::c_int == ':' as i32 {
                (*on_off).param_type = PARAM_LIST as libc::c_int as smallint
              } else if c == '+' as i32
                && (*s.offset(2) as libc::c_int == ':' as i32
                  || *s.offset(2) as libc::c_int == '\u{0}' as i32)
              {
                (*on_off).param_type = PARAM_INT as libc::c_int as smallint;
                s = s.offset(1)
              } else if c == ':' as i32 || c == '\u{0}' as i32 {
                requires |= (*on_off).switch_on
              } else if c == '-' as i32
                && (*s.offset(2) as libc::c_int == ':' as i32
                  || *s.offset(2) as libc::c_int == '\u{0}' as i32)
              {
                flags |= (*on_off).switch_on;
                (*on_off).incongruously |= (*on_off).switch_on;
                s = s.offset(1)
              } else {
                if c == *s as libc::c_int {
                  (*on_off).counter = p.arg::<*mut libc::c_int>();
                  s = s.offset(1)
                }
                pair = on_off;
                pair_switch = &mut (*pair).switch_on;
                s = s.offset(1);
                while *s as libc::c_int != 0 && *s as libc::c_int != ':' as i32 {
                  if *s as libc::c_int == '?' as i32 {
                    pair_switch = &mut (*pair).requires
                  } else if *s as libc::c_int == '-' as i32 {
                    if pair_switch == &mut (*pair).switch_off as *mut libc::c_uint {
                      pair_switch = &mut (*pair).incongruously
                    } else {
                      pair_switch = &mut (*pair).switch_off
                    }
                  } else {
                    on_off = complementary.as_mut_ptr();
                    while (*on_off).opt_char != 0 {
                      if (*on_off).opt_char as libc::c_int == *s as libc::c_int {
                        *pair_switch |= (*on_off).switch_on;
                        break;
                      } else {
                        on_off = on_off.offset(1)
                      }
                    }
                  }
                  s = s.offset(1)
                }
                s = s.offset(-1)
              }
            }
          }
        }
      }
      s = s.offset(1)
    }
  }
  /* In case getopt32 was already called:
   * reset libc getopt() internal state.
   * run_nofork_applet() does this, but we might end up here
   * also via gunzip_main() -> gzip_main(). Play safe.
   */
  optind = 0i32;
  /* skip 0: some applets cheat: they do not actually HAVE argv[0] */
  argc = (1i32 as libc::c_uint).wrapping_add(string_array_len(argv.offset(1))) as libc::c_int;
  's_672: loop
  /* Note: just "getopt() <= 0" will not work well for
   * "fake" short options, like this one:
   * wget $'-\203' "Test: test" http://kernel.org/
   * (supposed to act as --header, but doesn't) */
  {
    c = getopt_long(argc, argv, applet_opts, long_options, 0 as *mut libc::c_int);
    if !(c != -1i32) {
      current_block = 6186816898867308296;
      break;
    }
    /* getopt prints "option requires an argument -- X"
     * and returns '?' if an option has no arg, but one is reqd */
    c &= 0xffi32; /* fight libc's sign extension */
    on_off = complementary.as_mut_ptr();
    while (*on_off).opt_char as libc::c_int != c {
      /* c can be NUL if long opt has non-NULL ->flag,
       * but we construct long opts so that flag
       * is always NULL (see above) */
      if (*on_off).opt_char as libc::c_int == '\u{0}' as i32 {
        /* && c != '\0' */
        current_block = 6652860238994955486;
        break 's_672;
      } else {
        on_off = on_off.offset(1)
      }
    }
    if flags & (*on_off).incongruously != 0 {
      current_block = 6652860238994955486;
      break;
    }
    trigger = (*on_off).switch_on & (*on_off).switch_off;
    flags &= !((*on_off).switch_off ^ trigger);
    flags |= (*on_off).switch_on ^ trigger;
    flags ^= trigger;
    if !(*on_off).counter.is_null() {
      *(*on_off).counter += 1
    }
    if !optarg.is_null() {
      if (*on_off).param_type as libc::c_int == PARAM_LIST as libc::c_int {
        llist_add_to_end(
          (*on_off).optarg as *mut *mut llist_t,
          optarg as *mut libc::c_void,
        );
      } else if (*on_off).param_type as libc::c_int == PARAM_INT as libc::c_int {
        //TODO: xatoi_positive indirectly pulls in printf machinery
        *((*on_off).optarg as *mut libc::c_uint) = xatoi_positive(optarg) as libc::c_uint
      } else if !(*on_off).optarg.is_null() {
        let ref mut fresh5 = *((*on_off).optarg as *mut *mut libc::c_char);
        *fresh5 = optarg
      }
    }
  }
  match current_block {
    6186816898867308296 =>
    /* check depending requires for given options */
    {
      on_off = complementary.as_mut_ptr();
      loop {
        if !((*on_off).opt_char != 0) {
          current_block = 12655303178690906525;
          break;
        }
        if (*on_off).requires != 0
          && flags & (*on_off).switch_on != 0
          && flags & (*on_off).requires == 0i32 as libc::c_uint
        {
          current_block = 6652860238994955486;
          break;
        }
        on_off = on_off.offset(1)
      }
      match current_block {
        6652860238994955486 => {}
        _ => {
          if !(requires != 0 && flags & requires == 0i32 as libc::c_uint) {
            argc -= optind;
            if !(argc < min_arg || max_arg >= 0i32 && argc > max_arg) {
              option_mask32 = flags;
              return flags;
            }
          }
        }
      }
    }
    _ => {}
  }
  /* c is probably '?' - "bad option" */
  if dont_die_flag as libc::c_int != '!' as i32 {
    bb_show_usage();
  }
  return -1i32 as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn getopt32(
  mut argv: *mut *mut libc::c_char,
  mut applet_opts: *const libc::c_char,
  mut args: ...
) -> uint32_t {
  let mut opt: uint32_t = 0;
  let mut p: ::std::ffi::VaListImpl;
  p = args.clone();
  opt = vgetopt32(argv, applet_opts, 0 as *const libc::c_char, p.as_va_list());
  return opt;
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
#[no_mangle]
pub unsafe extern "C" fn getopt32long(
  mut argv: *mut *mut libc::c_char,
  mut applet_opts: *const libc::c_char,
  mut longopts: *const libc::c_char,
  mut args: ...
) -> uint32_t {
  let mut opt: uint32_t = 0;
  let mut p: ::std::ffi::VaListImpl;
  p = args.clone();
  opt = vgetopt32(argv, applet_opts, longopts, p.as_va_list());
  return opt;
}
