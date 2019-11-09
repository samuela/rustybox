use libc;
use libc::open;




/*
 * Utility routines.
 *
 * Copyright (C) 2016 Denys Vlasenko
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
//config:config FEATURE_USE_BSS_TAIL
//config:	bool "Use the end of BSS page"
//config:	default n
//config:	help
//config:	Attempt to reclaim a small unused part of BSS.
//config:
//config:	Executables have the following parts:
//config:	= read-only executable code and constants, also known as "text"
//config:	= read-write data
//config:	= non-initialized (zeroed on demand) data, also known as "bss"
//config:
//config:	At link time, "text" is padded to a full page. At runtime, all "text"
//config:	pages are mapped RO and executable.
//config:
//config:	"Data" starts on the next page boundary, but is not padded
//config:	to a full page at the end. "Bss" starts wherever "data" ends.
//config:	At runtime, "data" pages are mapped RW and they are file-backed
//config:	(this includes a small portion of "bss" which may live in the last
//config:	partial page of "data").
//config:	Pages which are fully in "bss" are mapped to anonymous memory.
//config:
//config:	"Bss" end is usually not page-aligned. There is an unused space
//config:	in the last page. Linker marks its start with the "_end" symbol.
//config:
//config:	This option will attempt to use that space for bb_common_bufsiz1[]
//config:	array. If it fits after _end, it will be used, and COMMON_BUFSIZE
//config:	will be enlarged from its guaranteed minimum size of 1 kbyte.
//config:	This may require recompilation a second time, since value of _end
//config:	is known only after final link.
//config:
//config:	If you are getting a build error like this:
//config:		appletlib.c:(.text.main+0xd): undefined reference to '_end'
//config:	disable this option.
//kbuild:lib-y += common_bufsiz.o
/* We use it for "global" data via *(struct global*)bb_common_bufsiz1.
 * Since gcc insists on aligning struct global's members, it would be a pity
 * (and an alignment fault on some CPUs) to mess it up. */
#[no_mangle]
pub static mut bb_common_bufsiz1: [libc::c_char; 1024] = [0; 1024];
