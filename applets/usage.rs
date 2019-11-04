pub struct usage_data {
  pub aname: &'static str,
  pub usage: &'static str,
}

static mut usage_array: [usage_data; 396] = [
  {
    let mut init =
             usage_data{aname: "gunzip",
                        usage: "[-cfkt] [FILE]...\n\nDecompress FILEs (or stdin)\n\n\t-c\tWrite to stdout\n\t-f\tForce\n\t-k\tKeep input files\n\t-t\tTest file integrity"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "zcat",
      usage: "[FILE]...\n\nDecompress to stdout",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "bunzip2",
                        usage: "[-cfk] [FILE]...\n\nDecompress FILEs (or stdin)\n\n\t-c\tWrite to stdout\n\t-f\tForce\n\t-k\tKeep input files"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "bzcat",
      usage: "[FILE]...\n\nDecompress to stdout",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "unlzma",
                        usage: "[-cfk] [FILE]...\n\nDecompress FILE (or stdin)\n\n\t-c\tWrite to stdout\n\t-f\tForce\n\t-k\tKeep input files"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "lzcat",
      usage: "[FILE]...\n\nDecompress to stdout",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "lzma",
                        usage: "-d [-cfk] [FILE]...\n\nDecompress FILE (or stdin)\n\n\t-d\tDecompress\n\t-c\tWrite to stdout\n\t-f\tForce\n\t-k\tKeep input files"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "unxz",
                        usage: "[-cfk] [FILE]...\n\nDecompress FILE (or stdin)\n\n\t-c\tWrite to stdout\n\t-f\tForce\n\t-k\tKeep input files"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "xzcat",
      usage: "[FILE]...\n\nDecompress to stdout",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "xz",
                        usage: "-d [-cfk] [FILE]...\n\nDecompress FILE (or stdin)\n\n\t-d\tDecompress\n\t-c\tWrite to stdout\n\t-f\tForce\n\t-k\tKeep input files"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "bzip2",
                        usage: "[OPTIONS] [FILE]...\n\nCompress FILEs (or stdin) with bzip2 algorithm\n\n\t-1..9\tCompression level\n\t-d\tDecompress\n\t-t\tTest file integrity\n\t-c\tWrite to stdout\n\t-f\tForce\n\t-k\tKeep input files"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "cpio",
                        usage: "[-dmvu] [-F FILE] [-R USER[:GRP]] [-H newc] [-tio] [-p DIR] [EXTR_FILE]...\n\nExtract (-i) or list (-t) files from a cpio archive, or\ntake file list from stdin and create an archive (-o) or copy files (-p)\n\nMain operation mode:\n\t-t\tList\n\t-i\tExtract EXTR_FILEs (or all)\n\t-o\tCreate (requires -H newc)\n\t-p DIR\tCopy files to DIR\nOptions:\n\t-H newc\tArchive format\n\t-d\tMake leading directories\n\t-m\tPreserve mtime\n\t-v\tVerbose\n\t-u\tOverwrite\n\t-F FILE\tInput (-t,-i,-p) or output (-o) file\n\t-R USER[:GRP]\tSet owner of created files\n\t-L\tDereference symlinks\n\t-0\tInput is separated by NULs"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "dpkg",
                        usage: "[-ilCPru] [-F OPT] PACKAGE\n\nInstall, remove and manage Debian packages\n\n\t-i,--install\tInstall the package\n\t-l,--list\tList of installed packages\n\t--configure\tConfigure an unpackaged package\n\t-P,--purge\tPurge all files of a package\n\t-r,--remove\tRemove all but the configuration files for a package\n\t--unpack\tUnpack a package, but don\'t configure it\n\t--force-depends\tIgnore dependency problems\n\t--force-confnew\tOverwrite existing config files when installing\n\t--force-confold\tKeep old config files when installing"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "dpkg-deb",
                        usage: "[-cefxX] FILE [DIR]\n\nPerform actions on Debian packages (.deb)\n\n\t-c\tList files\n\t-f\tPrint control fields\n\t-e\tExtract control files to DIR (default: ./DEBIAN)\n\t-x\tExtract files to DIR (no default)\n\t-X\tVerbose -x"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "gzip",
                        usage: "[-cfkdt] [FILE]...\n\nCompress FILEs (or stdin)\n\n\t-d\tDecompress\n\t-t\tTest file integrity\n\t-c\tWrite to stdout\n\t-f\tForce\n\t-k\tKeep input files"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "lzop",
                        usage: "[-cfUvd123456789CF] [FILE]...\n\n\t-1..9\tCompression level\n\t-d\tDecompress\n\t-c\tWrite to stdout\n\t-f\tForce\n\t-U\tDelete input files\n\t-v\tVerbose\n\t-F\tDon\'t store or verify checksum\n\t-C\tAlso write checksum of compressed block"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "rpm",
                        usage: "-i PACKAGE.rpm; rpm -qp[ildc] PACKAGE.rpm\n\nManipulate RPM packages\n\nCommands:\n\t-i\tInstall package\n\t-qp\tQuery package\n\t-qpi\tShow information\n\t-qpl\tList contents\n\t-qpd\tList documents\n\t-qpc\tList config files"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "rpm2cpio",
      usage: "PACKAGE.rpm\n\nOutput a cpio archive of the rpm file",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "tar",
                        usage: "c|x|t [-zJjahmvokO] [-f TARFILE] [-C DIR] [-T FILE] [-X FILE] [--exclude PATTERN]... [FILE]...\n\nCreate, extract, or list files from a tar file\n\n\tc\tCreate\n\tx\tExtract\n\tt\tList\n\t-f FILE\tName of TARFILE (\'-\' for stdin/out)\n\t-C DIR\tChange to DIR before operation\n\t-v\tVerbose\n\t-O\tExtract to stdout\n\t-m\tDon\'t restore mtime\n\t-o\tDon\'t restore user:group\n\t-k\tDon\'t replace existing files\n\t-z\t(De)compress using gzip\n\t-J\t(De)compress using xz\n\t-j\t(De)compress using bzip2\n\t-a\t(De)compress based on extension\n\t-h\tFollow symlinks\n\t-T FILE\tFile with names to include\n\t-X FILE\tFile with glob patterns to exclude\n\t--exclude PATTERN\tGlob pattern to exclude"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "unzip",
                        usage: "[-lnojpq] FILE[.zip] [FILE]... [-x FILE...] [-d DIR]\n\nExtract FILEs from ZIP archive\n\n\t-l\tList contents (with -q for short form)\n\t-n\tNever overwrite files (default: ask)\n\t-o\tOverwrite\n\t-j\tDo not restore paths\n\t-p\tPrint to stdout\n\t-q\tQuiet\n\t-x FILE\tExclude FILEs\n\t-d DIR\tExtract into DIR"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "chvt",
      usage: "N\n\nChange the foreground virtual terminal to /dev/ttyN",
    };
    init
  },
  {
    let mut init = usage_data {
      aname: "clear",
      usage: "\n\nClear screen",
    };
    init
  },
  {
    let mut init = usage_data {
      aname: "deallocvt",
      usage: "[N]\n\nDeallocate unused virtual terminal /dev/ttyN",
    };
    init
  },
  {
    let mut init = usage_data {
      aname: "dumpkmap",
      usage: "> keymap\n\nPrint a binary keyboard translation table to stdout",
    };
    init
  },
  {
    let mut init = usage_data {
      aname: "fgconsole",
      usage: "\n\nGet active console",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "kbd_mode",
                        usage: "[-a|k|s|u] [-C TTY]\n\nReport or set VT console keyboard mode\n\n\t-a\tDefault (ASCII)\n\t-k\tMedium-raw (keycode)\n\t-s\tRaw (scancode)\n\t-u\tUnicode (utf-8)\n\t-C TTY\tAffect TTY"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "loadfont",
      usage: "< font\n\nLoad a console font from stdin",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "setfont",
                        usage: "FONT [-m MAPFILE] [-C TTY]\n\nLoad a console font\n\n\t-m MAPFILE\tLoad console screen map\n\t-C TTY\t\tAffect TTY instead of /dev/tty"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "loadkmap",
      usage: "< keymap\n\nLoad a binary keyboard translation table from stdin",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "openvt",
                        usage: "[-c N] [-sw] [PROG ARGS]\n\nStart PROG on a new virtual terminal\n\n\t-c N\tUse specified VT\n\t-s\tSwitch to the VT\n\t-w\tWait for PROG to exit"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "reset",
      usage: "\n\nReset the screen",
    };
    init
  },
  {
    let mut init = usage_data {
      aname: "resize",
      usage: "\n\nResize the screen",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "setconsole",
                        usage: "[-r] [DEVICE]\n\nMake writes to /dev/console appear on DEVICE (default: /dev/tty).\nDoes not redirect kernel log output or reads from /dev/console.\n\n\t-r\tReset: writes to /dev/console go to kernel log tty(s)"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "setkeycodes",
                        usage: "{ SCANCODE KEYCODE }...\n\nModify kernel\'s scancode-to-keycode map,\nallowing unusual keyboards to generate usable keycodes.\n\nSCANCODE is either xx or e0xx (hexadecimal), KEYCODE is decimal."
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "setlogcons",
      usage: "[N]\n\nPin kernel output to VT console N. Default:0 (do not pin)",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "showkey",
                        usage: "[-a | -k | -s]\n\nShow keys pressed\n\n\t-a\tDisplay decimal/octal/hex values of the keys\n\t-k\tDisplay interpreted keycodes (default)\n\t-s\tDisplay raw scan-codes"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "basename",
      usage: "FILE [SUFFIX]\n\nStrip directory path and .SUFFIX from FILE",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "cat",
                        usage: "[-nbvteA] [FILE]...\n\nPrint FILEs to stdout\n\n\t-n\tNumber output lines\n\t-b\tNumber nonempty lines\n\t-v\tShow nonprinting characters as ^x or M-x\n\t-t\t...and tabs as ^I\n\t-e\t...and end lines with $\n\t-A\tSame as -vte"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "chgrp",
                        usage: "[-RhLHPcvf]... GROUP FILE...\n\nChange the group membership of each FILE to GROUP\n\n\t-R\tRecurse\n\t-h\tAffect symlinks instead of symlink targets\n\t-L\tTraverse all symlinks to directories\n\t-H\tTraverse symlinks on command line only\n\t-P\tDon\'t traverse symlinks (default)\n\t-c\tList changed files\n\t-v\tVerbose\n\t-f\tHide errors"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "chmod",
                        usage: "[-Rcvf] MODE[,MODE]... FILE...\n\nEach MODE is one or more of the letters ugoa, one of the\nsymbols +-= and one or more of the letters rwxst\n\n\t-R\tRecurse\n\t-c\tList changed files\n\t-v\tList all files\n\t-f\tHide errors"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "chown",
                        usage: "[-RhLHPcvf]... USER[:[GRP]] FILE...\n\nChange the owner and/or group of each FILE to USER and/or GRP\n\n\t-R\tRecurse\n\t-h\tAffect symlinks instead of symlink targets\n\t-L\tTraverse all symlinks to directories\n\t-H\tTraverse symlinks on command line only\n\t-P\tDon\'t traverse symlinks (default)\n\t-c\tList changed files\n\t-v\tList all files\n\t-f\tHide errors"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "chroot",
      usage: "NEWROOT [PROG ARGS]\n\nRun PROG with root directory set to NEWROOT",
    };
    init
  },
  {
    let mut init = usage_data {
      aname: "cksum",
      usage: "FILE...\n\nCalculate the CRC32 checksums of FILEs",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "comm",
                        usage: "[-123] FILE1 FILE2\n\nCompare FILE1 with FILE2\n\n\t-1\tSuppress lines unique to FILE1\n\t-2\tSuppress lines unique to FILE2\n\t-3\tSuppress lines common to both files"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "cp",
                        usage: "[OPTIONS] SOURCE... DEST\n\nCopy SOURCE(s) to DEST\n\n\t-a\tSame as -dpR\n\t-R,-r\tRecurse\n\t-d,-P\tPreserve symlinks (default if -R)\n\t-L\tFollow all symlinks\n\t-H\tFollow symlinks on command line\n\t-p\tPreserve file attributes if possible\n\t-f\tOverwrite\n\t-i\tPrompt before overwrite\n\t-l,-s\tCreate (sym)links\n\t-T\tTreat DEST as a normal file\n\t-u\tCopy only newer files"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "cut",
                        usage: "[OPTIONS] [FILE]...\n\nPrint selected fields from each input FILE to stdout\n\n\t-b LIST\tOutput only bytes from LIST\n\t-c LIST\tOutput only characters from LIST\n\t-d CHAR\tUse CHAR instead of tab as the field delimiter\n\t-s\tOutput only the lines containing delimiter\n\t-f N\tPrint only these fields\n\t-n\tIgnored"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "date",
                        usage: "[OPTIONS] [+FMT] [TIME]\n\nDisplay time (using +FMT), or set time\n\n\t[-s,--set] TIME\tSet time to TIME\n\t-u,--utc\tWork in UTC (don\'t convert to local time)\n\t-R,--rfc-2822\tOutput RFC-2822 compliant date string\n\t-I[SPEC]\tOutput ISO-8601 compliant date string\n\t\t\tSPEC=\'date\' (default) for date only,\n\t\t\t\'hours\', \'minutes\', or \'seconds\' for date and\n\t\t\ttime to the indicated precision\n\t-r,--reference FILE\tDisplay last modification time of FILE\n\t-d,--date TIME\tDisplay TIME, not \'now\'\n\t-D FMT\t\tUse FMT (strptime format) for -d TIME conversion\n\nRecognized TIME formats:\n\thh:mm[:ss]\n\t[YYYY.]MM.DD-hh:mm[:ss]\n\tYYYY-MM-DD hh:mm[:ss]\n\t[[[[[YY]YY]MM]DD]hh]mm[.ss]\n\t\'date TIME\' form accepts MMDDhhmm[[YY]YY][.ss] instead"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "dd",
                        usage: "[if=FILE] [of=FILE] [ibs=N obs=N/bs=N] [count=N] [skip=N] [seek=N]\n\t[conv=notrunc|noerror|sync|fsync]\n\t[iflag=skip_bytes|fullblock] [oflag=seek_bytes|append]\n\nCopy a file with converting and formatting\n\n\tif=FILE\t\tRead from FILE instead of stdin\n\tof=FILE\t\tWrite to FILE instead of stdout\n\tbs=N\t\tRead and write N bytes at a time\n\tibs=N\t\tRead N bytes at a time\n\tobs=N\t\tWrite N bytes at a time\n\tcount=N\t\tCopy only N input blocks\n\tskip=N\t\tSkip N input blocks\n\tseek=N\t\tSkip N output blocks\n\tconv=notrunc\tDon\'t truncate output file\n\tconv=noerror\tContinue after read errors\n\tconv=sync\tPad blocks with zeros\n\tconv=fsync\tPhysically write data out before finishing\n\tconv=swab\tSwap every pair of bytes\n\tiflag=skip_bytes\tskip=N is in bytes\n\tiflag=fullblock\tRead full blocks\n\toflag=seek_bytes\tseek=N is in bytes\n\toflag=append\tOpen output file in append mode\n\tstatus=noxfer\tSuppress rate output\n\tstatus=none\tSuppress all output\n\nN may be suffixed by c (1), w (2), b (512), kB (1000), k (1024), MB, M, GB, G"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "df",
                        usage: "[-PkmhTai] [-B SIZE] [FILESYSTEM]...\n\nPrint filesystem usage statistics\n\n\t-P\tPOSIX output format\n\t-k\t1024-byte blocks (default)\n\t-m\t1M-byte blocks\n\t-h\tHuman readable (e.g. 1K 243M 2G)\n\t-T\tPrint filesystem type\n\t-a\tShow all filesystems\n\t-i\tInodes\n\t-B SIZE\tBlocksize"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "dirname",
      usage: "FILENAME\n\nStrip non-directory suffix from FILENAME",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "dos2unix",
                        usage: "[-ud] [FILE]\n\nConvert FILE in-place from DOS to Unix format.\nWhen no file is given, use stdin/stdout.\n\n\t-u\tdos2unix\n\t-d\tunix2dos"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "unix2dos",
                        usage: "[-ud] [FILE]\n\nConvert FILE in-place from Unix to DOS format.\nWhen no file is given, use stdin/stdout.\n\n\t-u\tdos2unix\n\t-d\tunix2dos"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "du",
                        usage: "[-aHLdclsxhmk] [FILE]...\n\nSummarize disk space used for each FILE and/or directory\n\n\t-a\tShow file sizes too\n\t-L\tFollow all symlinks\n\t-H\tFollow symlinks on command line\n\t-d N\tLimit output to directories (and files with -a) of depth < N\n\t-c\tShow grand total\n\t-l\tCount sizes many times if hard linked\n\t-s\tDisplay only a total for each argument\n\t-x\tSkip directories on different filesystems\n\t-h\tSizes in human readable format (e.g., 1K 243M 2G)\n\t-m\tSizes in megabytes\n\t-k\tSizes in kilobytes (default)"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "echo",
                        usage: "[-neE] [ARG]...\n\nPrint the specified ARGs to stdout\n\n\t-n\tSuppress trailing newline\n\t-e\tInterpret backslash escapes (i.e., \\t=tab)\n\t-E\tDon\'t interpret backslash escapes (default)"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "env",
                        usage: "[-iu] [-] [name=value]... [PROG ARGS]\n\nPrint the current environment or run PROG after setting up\nthe specified environment\n\n\t-, -i\tStart with an empty environment\n\t-u\tRemove variable from the environment"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "expand",
                        usage: "[-i] [-t N] [FILE]...\n\nConvert tabs to spaces, writing to stdout\n\n\t-i\tDon\'t convert tabs after non blanks\n\t-t\tTabstops every N chars"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "unexpand",
                        usage: "[-fa][-t N] [FILE]...\n\nConvert spaces to tabs, writing to stdout\n\n\t-a\tConvert all blanks\n\t-f\tConvert only leading blanks\n\t-t N\tTabstops every N chars"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "expr",
                        usage: "EXPRESSION\n\nPrint the value of EXPRESSION to stdout\n\nEXPRESSION may be:\n\tARG1 | ARG2\tARG1 if it is neither null nor 0, otherwise ARG2\n\tARG1 & ARG2\tARG1 if neither argument is null or 0, otherwise 0\n\tARG1 < ARG2\t1 if ARG1 is less than ARG2, else 0. Similarly:\n\tARG1 <= ARG2\n\tARG1 = ARG2\n\tARG1 != ARG2\n\tARG1 >= ARG2\n\tARG1 > ARG2\n\tARG1 + ARG2\tSum of ARG1 and ARG2. Similarly:\n\tARG1 - ARG2\n\tARG1 * ARG2\n\tARG1 / ARG2\n\tARG1 % ARG2\n\tSTRING : REGEXP\t\tAnchored pattern match of REGEXP in STRING\n\tmatch STRING REGEXP\tSame as STRING : REGEXP\n\tsubstr STRING POS LENGTH Substring of STRING, POS counted from 1\n\tindex STRING CHARS\tIndex in STRING where any CHARS is found, or 0\n\tlength STRING\t\tLength of STRING\n\tquote TOKEN\t\tInterpret TOKEN as a string, even if\n\t\t\t\tit is a keyword like \'match\' or an\n\t\t\t\toperator like \'/\'\n\t(EXPRESSION)\t\tValue of EXPRESSION\n\nBeware that many operators need to be escaped or quoted for shells.\nComparisons are arithmetic if both ARGs are numbers, else\nlexicographical. Pattern matches return the string matched between\n\\( and \\) or null; if \\( and \\) are not used, they return the number\nof characters matched or 0."
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "factor",
      usage: "[NUMBER]...\n\nPrint prime factors",
    };
    init
  },
  {
    let mut init = usage_data {
      aname: "false",
      usage: "\x08",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "fold",
                        usage: "[-bs] [-w WIDTH] [FILE]...\n\nWrap input lines in each FILE (or stdin), writing to stdout\n\n\t-b\tCount bytes rather than columns\n\t-s\tBreak at spaces\n\t-w\tUse WIDTH columns instead of 80"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "head",
                        usage: "[OPTIONS] [FILE]...\n\nPrint first 10 lines of each FILE (or stdin) to stdout.\nWith more than one FILE, precede each with a filename header.\n\n\t-n N[kbm]\tPrint first N lines\n\t-n -N[kbm]\tPrint all except N last lines\n\t-c [-]N[kbm]\tPrint first N bytes\n\t-q\t\tNever print headers\n\t-v\t\tAlways print headers\n\nN may be suffixed by k (x1024), b (x512), or m (x1024^2)."
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "hostid",
      usage: "\n\nPrint out a unique 32-bit identifier for the machine",
    };
    init
  },
  {
    let mut init = usage_data {
      aname: "groups",
      usage: "[USER]\n\nPrint the group memberships of USER or for the current process",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "id",
                        usage: "[OPTIONS] [USER]\n\nPrint information about USER or the current user\n\n\t-u\tUser ID\n\t-g\tGroup ID\n\t-G\tSupplementary group IDs\n\t-n\tPrint names instead of numbers\n\t-r\tPrint real ID instead of effective ID"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "install",
                        usage: "[-cdDsp] [-o USER] [-g GRP] [-m MODE] [-t DIR] [SOURCE]... DEST\n\nCopy files and set attributes\n\n\t-c\tJust copy (default)\n\t-d\tCreate directories\n\t-D\tCreate leading target directories\n\t-s\tStrip symbol table\n\t-p\tPreserve date\n\t-o USER\tSet ownership\n\t-g GRP\tSet group ownership\n\t-m MODE\tSet permissions\n\t-t DIR\tInstall to DIR"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "link",
      usage: "FILE LINK\n\nCreate hard LINK to FILE",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "ln",
                        usage: "[OPTIONS] TARGET... LINK|DIR\n\nCreate a link LINK or DIR/TARGET to the specified TARGET(s)\n\n\t-s\tMake symlinks instead of hardlinks\n\t-f\tRemove existing destinations\n\t-n\tDon\'t dereference symlinks - treat like normal file\n\t-b\tMake a backup of the target (if exists) before link operation\n\t-S suf\tUse suffix instead of ~ when making backup files\n\t-T\tTreat LINK as a file, not DIR\n\t-v\tVerbose"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "logname",
      usage: "\n\nPrint the name of the current user",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "ls",
                        usage: "[-1AaCxdLHRFplinshrSXvctu] [-w WIDTH] [FILE]...\n\nList directory contents\n\n\t-1\tOne column output\n\t-a\tInclude entries which start with .\n\t-A\tLike -a, but exclude . and ..\n\t-x\tList by lines\n\t-d\tList directory entries instead of contents\n\t-L\tFollow symlinks\n\t-H\tFollow symlinks on command line\n\t-R\tRecurse\n\t-p\tAppend / to dir entries\n\t-F\tAppend indicator (one of */=@|) to entries\n\t-l\tLong listing format\n\t-i\tList inode numbers\n\t-n\tList numeric UIDs and GIDs instead of names\n\t-s\tList allocated blocks\n\t-lc\tList ctime\n\t-lu\tList atime\n\t--full-time\tList full date and time\n\t-h\tHuman readable sizes (1K 243M 2G)\n\t--group-directories-first\n\t-S\tSort by size\n\t-X\tSort by extension\n\t-v\tSort by version\n\t-t\tSort by mtime\n\t-tc\tSort by ctime\n\t-tu\tSort by atime\n\t-r\tReverse sort order\n\t-w N\tFormat N columns wide\n\t--color[={always,never,auto}]\tControl coloring"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "md5sum",
                        usage: "[-c[sw]] [FILE]...\n\nPrint or check MD5 checksums\n\n\t-c\tCheck sums against list in FILEs\n\t-s\tDon\'t output anything, status code shows success\n\t-w\tWarn about improperly formatted checksum lines"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "sha1sum",
                        usage: "[-c[sw]] [FILE]...\n\nPrint or check SHA1 checksums\n\n\t-c\tCheck sums against list in FILEs\n\t-s\tDon\'t output anything, status code shows success\n\t-w\tWarn about improperly formatted checksum lines"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "sha3sum",
                        usage: "[-c[sw]] [-a BITS] [FILE]...\n\nPrint or check SHA3 checksums\n\n\t-c\tCheck sums against list in FILEs\n\t-s\tDon\'t output anything, status code shows success\n\t-w\tWarn about improperly formatted checksum lines\n\t-a BITS\t224 (default), 256, 384, 512"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "sha256sum",
                        usage: "[-c[sw]] [FILE]...\n\nPrint or check SHA256 checksums\n\n\t-c\tCheck sums against list in FILEs\n\t-s\tDon\'t output anything, status code shows success\n\t-w\tWarn about improperly formatted checksum lines"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "sha512sum",
                        usage: "[-c[sw]] [FILE]...\n\nPrint or check SHA512 checksums\n\n\t-c\tCheck sums against list in FILEs\n\t-s\tDon\'t output anything, status code shows success\n\t-w\tWarn about improperly formatted checksum lines"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "mkdir",
                        usage: "[OPTIONS] DIRECTORY...\n\nCreate DIRECTORY\n\n\t-m MODE\tMode\n\t-p\tNo error if exists; make parent directories as needed"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "mkfifo",
      usage: "[-m MODE] NAME\n\nCreate named pipe\n\n\t-m MODE\tMode (default a=rw)",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "mknod",
                        usage: "[-m MODE] NAME TYPE [MAJOR MINOR]\n\nCreate a special file (block, character, or pipe)\n\n\t-m MODE\tCreation mode (default a=rw)\nTYPE:\n\tb\tBlock device\n\tc or u\tCharacter device\n\tp\tNamed pipe (MAJOR MINOR must be omitted)"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "mktemp",
                        usage: "[-dt] [-p DIR] [TEMPLATE]\n\nCreate a temporary file with name based on TEMPLATE and print its name.\nTEMPLATE must end with XXXXXX (e.g. [/dir/]nameXXXXXX).\nWithout TEMPLATE, -t tmp.XXXXXX is assumed.\n\n\t-d\tMake directory, not file\n\t-q\tFail silently on errors\n\t-t\tPrepend base directory name to TEMPLATE\n\t-p DIR\tUse DIR as a base directory (implies -t)\n\t-u\tDo not create anything; print a name\n\nBase directory is: -p DIR, else $TMPDIR, else /tmp"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "mv",
                        usage: "[-fin] SOURCE DEST\nor: mv [-fin] SOURCE... DIRECTORY\n\nRename SOURCE to DEST, or move SOURCE(s) to DIRECTORY\n\n\t-f\tDon\'t prompt before overwriting\n\t-i\tInteractive, prompt before overwrite\n\t-n\tDon\'t overwrite an existing file"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "nice",
                        usage: "[-n ADJUST] [PROG ARGS]\n\nChange scheduling priority, run PROG\n\n\t-n ADJUST\tAdjust priority by ADJUST"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "nl",
                        usage: "[OPTIONS] [FILE]...\n\nWrite FILEs to standard output with line numbers added\n\n\t-b STYLE\tWhich lines to number - a: all, t: nonempty, n: none\n\t-i N\t\tLine number increment\n\t-s STRING\tUse STRING as line number separator\n\t-v N\t\tStart from N\n\t-w N\t\tWidth of line numbers"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "nohup",
      usage: "PROG ARGS\n\nRun PROG immune to hangups, with output to a non-tty",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "nproc",
                        usage: "--all --ignore=N\n\nPrint number of available CPUs\n\n\t--all\t\tNumber of installed CPUs\n\t--ignore=N\tExclude N CPUs"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "od",
                        usage: "[-abcdfhilovxs] [-t TYPE] [-A RADIX] [-N SIZE] [-j SKIP] [-S MINSTR] [-w WIDTH] [FILE]...\n\nPrint FILEs (or stdin) unambiguously, as octal bytes by default"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "paste",
                        usage: "[OPTIONS] [FILE]...\n\nPaste lines from each input file, separated with tab\n\n\t-d LIST\tUse delimiters from LIST, not tab\n\t-s      Serial: one file at a time"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "printenv",
      usage: "[VARIABLE]...\n\nPrint environment VARIABLEs.\nIf no VARIABLE specified, print all.",
    };
    init
  },
  {
    let mut init = usage_data {
      aname: "printf",
      usage: "FORMAT [ARG]...\n\nFormat and print ARG(s) according to FORMAT (a-la C printf)",
    };
    init
  },
  {
    let mut init = usage_data {
      aname: "pwd",
      usage: "\n\nPrint the full filename of the current working directory",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "readlink",
                        usage: "[-fnv] FILE\n\nDisplay the value of a symlink\n\n\t-f\tCanonicalize by following all symlinks\n\t-n\tDon\'t add newline\n\t-v\tVerbose"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "realpath",
      usage: "FILE...\n\nReturn the absolute pathnames of given FILE",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "rm",
                        usage: "[-irf] FILE...\n\nRemove (unlink) FILEs\n\n\t-i\tAlways prompt before removing\n\t-f\tNever prompt\n\t-R,-r\tRecurse"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "rmdir",
                        usage: "[OPTIONS] DIRECTORY...\n\nRemove DIRECTORY if it is empty\n\n\t-p\tInclude parents\n\t--ignore-fail-on-non-empty"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "seq",
                        usage: "[-w] [-s SEP] [FIRST [INC]] LAST\n\nPrint numbers from FIRST to LAST, in steps of INC.\nFIRST, INC default to 1.\n\n\t-w\tPad to last with leading zeros\n\t-s SEP\tString separator"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "shred",
                        usage: "FILE...\n\nOverwrite/delete FILEs\n\n\t-f\tChmod to ensure writability\n\t-n N\tOverwrite N times (default 3)\n\t-z\tFinal overwrite with zeros\n\t-u\tRemove file"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "shuf",
                        usage: "[-e|-i L-H] [-n NUM] [-o FILE] [-z] [FILE|ARG...]\n\nRandomly permute lines\n\n\t-e\tTreat ARGs as lines\n\t-i L-H\tTreat numbers L-H as lines\n\t-n NUM\tOutput at most NUM lines\n\t-o FILE\tWrite to FILE, not standard output\n\t-z\tEnd lines with zero byte, not newline"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "sleep",
                        usage: "[N]...\n\nPause for a time equal to the total of the args given, where each arg can\nhave an optional suffix of (s)econds, (m)inutes, (h)ours, or (d)ays"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "sort",
                        usage: "[-nrugMcszbdfiokt] [-o FILE] [-k start[.offset][opts][,end[.offset][opts]] [-t CHAR] [FILE]...\n\nSort lines of text\n\n\t-o FILE\tOutput to FILE\n\t-c\tCheck whether input is sorted\n\t-b\tIgnore leading blanks\n\t-f\tIgnore case\n\t-i\tIgnore unprintable characters\n\t-d\tDictionary order (blank or alphanumeric only)\n\t-n\tSort numbers\n\t-g\tGeneral numerical sort\n\t-M\tSort month\n\t-V\tSort version\n\t-t CHAR\tField separator\n\t-k N[,M] Sort by Nth field\n\t-r\tReverse sort order\n\t-s\tStable (don\'t sort ties alphabetically)\n\t-u\tSuppress duplicate lines\n\t-z\tLines are terminated by NUL, not newline"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "split",
                        usage: "[OPTIONS] [INPUT [PREFIX]]\n\n\t-b N[k|m]\tSplit by N (kilo|mega)bytes\n\t-l N\t\tSplit by N lines\n\t-a N\t\tUse N letters as suffix"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "stat",
                        usage: "[OPTIONS] FILE...\n\nDisplay file (default) or filesystem status\n\n\t-c FMT\tUse the specified format\n\t-f\tDisplay filesystem status\n\t-L\tFollow links\n\t-t\tTerse display\n\nFMT sequences for files:\n %a\tAccess rights in octal\n %A\tAccess rights in human readable form\n %b\tNumber of blocks allocated (see %B)\n %B\tSize in bytes of each block reported by %b\n %d\tDevice number in decimal\n %D\tDevice number in hex\n %f\tRaw mode in hex\n %F\tFile type\n %g\tGroup ID\n %G\tGroup name\n %h\tNumber of hard links\n %i\tInode number\n %n\tFile name\n %N\tFile name, with -> TARGET if symlink\n %o\tI/O block size\n %s\tTotal size in bytes\n %t\tMajor device type in hex\n %T\tMinor device type in hex\n %u\tUser ID\n %U\tUser name\n %x\tTime of last access\n %X\tTime of last access as seconds since Epoch\n %y\tTime of last modification\n %Y\tTime of last modification as seconds since Epoch\n %z\tTime of last change\n %Z\tTime of last change as seconds since Epoch\n\nFMT sequences for file systems:\n %a\tFree blocks available to non-superuser\n %b\tTotal data blocks\n %c\tTotal file nodes\n %d\tFree file nodes\n %f\tFree blocks\n %i\tFile System ID in hex\n %l\tMaximum length of filenames\n %n\tFile name\n %s\tBlock size (for faster transfer)\n %S\tFundamental block size (for block counts)\n %t\tType in hex\n %T\tType in human readable form"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "stty",
                        usage: "[-a|g] [-F DEVICE] [SETTING]...\n\nWithout arguments, prints baud rate, line discipline,\nand deviations from stty sane\n\n\t-F DEVICE\tOpen device instead of stdin\n\t-a\t\tPrint all current settings in human-readable form\n\t-g\t\tPrint in stty-readable form\n\t[SETTING]\tSee manpage"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "sum",
                        usage: "[-rs] [FILE]...\n\nChecksum and count the blocks in a file\n\n\t-r\tUse BSD sum algorithm (1K blocks)\n\t-s\tUse System V sum algorithm (512byte blocks)"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "sync",
                        usage: "[-df] [FILE]...\n\nWrite all buffered blocks (in FILEs) to disk\n\t-d\tAvoid syncing metadata\n\t-f\tSync filesystems underlying FILEs"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "fsync",
      usage:
        "[-d] FILE...\n\nWrite all buffered blocks in FILEs to disk\n\n\t-d\tAvoid syncing metadata",
    };
    init
  },
  {
    let mut init = usage_data {
      aname: "tac",
      usage: "[FILE]...\n\nConcatenate FILEs and print them in reverse",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "tail",
                        usage: "[OPTIONS] [FILE]...\n\nPrint last 10 lines of each FILE (or stdin) to stdout.\nWith more than one FILE, precede each with a filename header.\n\n\t-f\t\tPrint data as file grows\n\t-c [+]N[kbm]\tPrint last N bytes\n\t-n N[kbm]\tPrint last N lines\n\t-n +N[kbm]\tStart on Nth line and print the rest\n\t-q\t\tNever print headers\n\t-s SECONDS\tWait SECONDS between reads with -f\n\t-v\t\tAlways print headers\n\t-F\t\tSame as -f, but keep retrying\n\nN may be suffixed by k (x1024), b (x512), or m (x1024^2)."
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "tee",
                        usage: "[-ai] [FILE]...\n\nCopy stdin to each FILE, and also to stdout\n\n\t-a\tAppend to the given FILEs, don\'t overwrite\n\t-i\tIgnore interrupt signals (SIGINT)"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "test",
      usage: "\x08",
    };
    init
  },
  {
    let mut init = usage_data {
      aname: "[",
      usage: "\x08",
    };
    init
  },
  {
    let mut init = usage_data {
      aname: "[[",
      usage: "\x08",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "timeout",
                        usage: "[-s SIG] SECS PROG ARGS\n\nRuns PROG. Sends SIG to it if it is not gone in SECS seconds.\nDefault SIG: TERM."
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "touch",
                        usage: "[-c] [-d DATE] [-t DATE] [-r FILE] FILE...\n\nUpdate the last-modified date on the given FILE[s]\n\n\t-c\tDon\'t create files\n\t-h\tDon\'t follow links\n\t-d DT\tDate/time to use\n\t-t DT\tDate/time to use\n\t-r FILE\tUse FILE\'s date/time"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "tr",
                        usage: "[-cds] STRING1 [STRING2]\n\nTranslate, squeeze, or delete characters from stdin, writing to stdout\n\n\t-c\tTake complement of STRING1\n\t-d\tDelete input characters coded STRING1\n\t-s\tSqueeze multiple output characters of STRING2 into one character"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "true",
      usage: "\x08",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "truncate",
                        usage: "[-c] -s SIZE FILE...\n\nTruncate FILEs to the given size\n\n\t-c\tDo not create files\n\t-s SIZE\tTruncate to SIZE"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "tty",
      usage:
        "\n\nPrint file name of stdin\'s terminal\n\n\t-s\tPrint nothing, only return exit status",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "uname",
                        usage: "[-amnrspvio]\n\nPrint system information\n\n\t-a\tPrint all\n\t-m\tThe machine (hardware) type\n\t-n\tHostname\n\t-r\tKernel release\n\t-s\tKernel name (default)\n\t-p\tProcessor type\n\t-v\tKernel version\n\t-i\tThe hardware platform\n\t-o\tOS name"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "arch",
      usage: "\n\nPrint system architecture",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "uniq",
                        usage: "[-cdu][-f,s,w N] [INPUT [OUTPUT]]\n\nDiscard duplicate lines\n\n\t-c\tPrefix lines by the number of occurrences\n\t-d\tOnly print duplicate lines\n\t-u\tOnly print unique lines\n\t-i\tIgnore case\n\t-f N\tSkip first N fields\n\t-s N\tSkip first N chars (after any skipped fields)\n\t-w N\tCompare N characters in line"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "unlink",
      usage: "FILE\n\nDelete FILE by calling unlink()",
    };
    init
  },
  {
    let mut init = usage_data {
      aname: "usleep",
      usage: "N\n\nPause for N microseconds",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "uudecode",
                        usage: "[-o OUTFILE] [INFILE]\n\nUudecode a file\nFinds OUTFILE in uuencoded source unless -o is given"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "base64",
      usage: "[-d] [FILE]\n\nBase64 encode or decode FILE to standard output\n\t-d\tDecode data",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "uuencode",
                        usage: "[-m] [FILE] STORED_FILENAME\n\nUuencode FILE (or stdin) to stdout\n\n\t-m\tUse base64 encoding per RFC1521"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "wc",
                        usage: "[-cmlwL] [FILE]...\n\nCount lines, words, and bytes for each FILE (or stdin)\n\n\t-c\tCount bytes\n\t-m\tCount characters\n\t-l\tCount newlines\n\t-w\tCount words\n\t-L\tPrint longest line length"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "users",
      usage: "\n\nPrint the users currently logged on",
    };
    init
  },
  {
    let mut init = usage_data {
      aname: "w",
      usage: "\n\nShow who is logged on",
    };
    init
  },
  {
    let mut init = usage_data {
      aname: "who",
      usage: "[-a]\n\nShow who is logged on\n\n\t-a\tShow all\n\t-H\tPrint column headers",
    };
    init
  },
  {
    let mut init = usage_data {
      aname: "whoami",
      usage: "\n\nPrint the user name associated with the current effective user id",
    };
    init
  },
  {
    let mut init = usage_data {
      aname: "yes",
      usage: "[STRING]\n\nRepeatedly output a line with STRING, or \'y\'",
    };
    init
  },
  {
    let mut init = usage_data {
      aname: "pipe_progress",
      usage: "\x08",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "run-parts",
                        usage: "[-a ARG]... [-u UMASK] [--reverse] [--test] [--exit-on-error] [--list] DIRECTORY\n\nRun a bunch of scripts in DIRECTORY\n\n\t-a ARG\t\tPass ARG as argument to scripts\n\t-u UMASK\tSet UMASK before running scripts\n\t--reverse\tReverse execution order\n\t--test\t\tDry run\n\t--exit-on-error\tExit if a script exits with non-zero\n\t--list\t\tPrint names of matching files even if they are not executable"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "start-stop-daemon",
                        usage: "[OPTIONS] [-S|-K] ... [-- ARGS...]\n\nSearch for matching processes, and then\n-K: stop all matching processes\n-S: start a process unless a matching process is found\n\nProcess matching:\n\t-u USERNAME|UID\tMatch only this user\'s processes\n\t-n NAME\t\tMatch processes with NAME\n\t\t\tin comm field in /proc/PID/stat\n\t-x EXECUTABLE\tMatch processes with this command\n\t\t\tin /proc/PID/cmdline\n\t-p FILE\t\tMatch a process with PID from FILE\n\tAll specified conditions must match\n-S only:\n\t-x EXECUTABLE\tProgram to run\n\t-a NAME\t\tZeroth argument\n\t-b\t\tBackground\n\t-N N\t\tChange nice level\n\t-c USER[:[GRP]]\tChange user/group\n\t-m\t\tWrite PID to pidfile specified by -p\n-K only:\n\t-s SIG\t\tSignal to send\n\t-t\t\tMatch only, exit with 0 if found\nOther:\n\t-o\t\tExit with status 0 if nothing is done\n\t-v\t\tVerbose\n\t-q\t\tQuiet"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "which",
      usage: "[COMMAND]...\n\nLocate a COMMAND",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "chattr",
                        usage: "[-R] [-v VERSION] [-+=AacDdijsStTu] FILE...\n\nChange ext2 file attributes\n\n\t-R\tRecurse\n\t-v VER\tSet version/generation number\nModifiers:\n\t-,+,=\tRemove/add/set attributes\nAttributes:\n\tA\tDon\'t track atime\n\ta\tAppend mode only\n\tc\tEnable compress\n\tD\tWrite dir contents synchronously\n\td\tDon\'t backup with dump\n\ti\tCannot be modified (immutable)\n\tj\tWrite all data to journal first\n\ts\tZero disk storage when deleted\n\tS\tWrite synchronously\n\tt\tDisable tail-merging of partial blocks with other files\n\tu\tAllow file to be undeleted"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "fsck",
                        usage: "[-ANPRTV] [-t FSTYPE] [FS_OPTS] [BLOCKDEV]...\n\nCheck and repair filesystems\n\n\t-A\tWalk /etc/fstab and check all filesystems\n\t-N\tDon\'t execute, just show what would be done\n\t-P\tWith -A, check filesystems in parallel\n\t-R\tWith -A, skip the root filesystem\n\t-T\tDon\'t show title on startup\n\t-V\tVerbose\n\t-t TYPE\tList of filesystem types to check"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "lsattr",
                        usage: "[-Radlv] [FILE]...\n\nList ext2 file attributes\n\n\t-R\tRecurse\n\t-a\tDon\'t hide entries starting with .\n\t-d\tList directory entries instead of contents\n\t-l\tList long flag names\n\t-v\tList version/generation number"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "awk",
                        usage: "[OPTIONS] [AWK_PROGRAM] [FILE]...\n\n\t-v VAR=VAL\tSet variable\n\t-F SEP\t\tUse SEP as field separator\n\t-f FILE\t\tRead program from FILE\n\t-e AWK_PROGRAM"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "cmp",
                        usage: "[-l] [-s] FILE1 [FILE2 [SKIP1 [SKIP2]]]\n\nCompare FILE1 with FILE2 (or stdin)\n\n\t-l\tWrite the byte numbers (decimal) and values (octal)\n\t\tfor all differing bytes\n\t-s\tQuiet"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "diff",
                        usage: "[-abBdiNqrTstw] [-L LABEL] [-S FILE] [-U LINES] FILE1 FILE2\n\nCompare files line by line and output the differences between them.\nThis implementation supports unified diffs only.\n\n\t-a\tTreat all files as text\n\t-b\tIgnore changes in the amount of whitespace\n\t-B\tIgnore changes whose lines are all blank\n\t-d\tTry hard to find a smaller set of changes\n\t-i\tIgnore case differences\n\t-L\tUse LABEL instead of the filename in the unified header\n\t-N\tTreat absent files as empty\n\t-q\tOutput only whether files differ\n\t-r\tRecurse\n\t-S\tStart with FILE when comparing directories\n\t-T\tMake tabs line up by prefixing a tab when necessary\n\t-s\tReport when two files are the same\n\t-t\tExpand tabs to spaces in output\n\t-U\tOutput LINES lines of context\n\t-w\tIgnore all whitespace"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "ed",
      usage: "[FILE]",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "patch",
                        usage: "[OPTIONS] [ORIGFILE [PATCHFILE]]\n\n\t-p N\tStrip N leading components from file names\n\t-i DIFF\tRead DIFF instead of stdin\n\t-R\tReverse patch\n\t-N\tIgnore already applied patches\n\t-E\tRemove output files if they become empty\n\t--dry-run\tDon\'t actually change files"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "sed",
                        usage: "[-i[SFX]] [-nrE] [-f FILE]... [-e CMD]... [FILE]...\nor: sed [-i[SFX]] [-nrE] CMD [FILE]...\n\n\t-e CMD\tAdd CMD to sed commands to be executed\n\t-f FILE\tAdd FILE contents to sed commands to be executed\n\t-i[SFX]\tEdit files in-place (otherwise sends to stdout)\n\t\tOptionally back files up, appending SFX\n\t-n\tSuppress automatic printing of pattern space\n\t-r,-E\tUse extended regex syntax\n\nIf no -e or -f, the first non-option argument is the sed command string.\nRemaining arguments are input files (stdin if none)."
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "vi",
                        usage: "[OPTIONS] [FILE]...\n\nEdit FILE\n\n\t-c CMD\tInitial command to run ($EXINIT also available)\n\t-R\tRead-only\n\t-H\tList available features"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "find",
                        usage: "[-HL] [PATH]... [OPTIONS] [ACTIONS]\n\nSearch for files and perform actions on them.\nFirst failed action stops processing of current file.\nDefaults: PATH is current directory, action is \'-print\'\n\n\t-L,-follow\tFollow symlinks\n\t-H\t\t...on command line only\n\t-xdev\t\tDon\'t descend directories on other filesystems\n\t-maxdepth N\tDescend at most N levels. -maxdepth 0 applies\n\t\t\tactions to command line arguments only\n\t-mindepth N\tDon\'t act on first N levels\n\t-depth\t\tAct on directory *after* traversing it\n\nActions:\n\t( ACTIONS )\tGroup actions for -o / -a\n\t! ACT\t\tInvert ACT\'s success/failure\n\tACT1 [-a] ACT2\tIf ACT1 fails, stop, else do ACT2\n\tACT1 -o ACT2\tIf ACT1 succeeds, stop, else do ACT2\n\t\t\tNote: -a has higher priority than -o\n\t-name PATTERN\tMatch file name (w/o directory name) to PATTERN\n\t-iname PATTERN\tCase insensitive -name\n\t-path PATTERN\tMatch path to PATTERN\n\t-ipath PATTERN\tCase insensitive -path\n\t-regex PATTERN\tMatch path to regex PATTERN\n\t-type X\t\tFile type is X (one of: f,d,l,b,c,s,p)\n\t-executable\tFile is executable\n\t-perm MASK\tAt least one mask bit (+MASK), all bits (-MASK),\n\t\t\tor exactly MASK bits are set in file\'s mode\n\t-mtime DAYS\tmtime is greater than (+N), less than (-N),\n\t\t\tor exactly N days in the past\n\t-mmin MINS\tmtime is greater than (+N), less than (-N),\n\t\t\tor exactly N minutes in the past\n\t-newer FILE\tmtime is more recent than FILE\'s\n\t-inum N\t\tFile has inode number N\n\t-user NAME/ID\tFile is owned by given user\n\t-group NAME/ID\tFile is owned by given group\n\t-size N[bck]\tFile size is N (c:bytes,k:kbytes,b:512 bytes(def.))\n\t\t\t+/-N: file size is bigger/smaller than N\n\t-links N\tNumber of links is greater than (+N), less than (-N),\n\t\t\tor exactly N\n\t-empty\t\tMatch empty file/directory\n\t-prune\t\tIf current file is directory, don\'t descend into it\nIf none of the following actions is specified, -print is assumed\n\t-print\t\tPrint file name\n\t-print0\t\tPrint file name, NUL terminated\n\t-exec CMD ARG ;\tRun CMD with all instances of {} replaced by\n\t\t\tfile name. Fails if CMD exits with nonzero\n\t-exec CMD ARG + Run CMD with {} replaced by list of file names\n\t-delete\t\tDelete current file/directory. Turns on -depth option\n\t-quit\t\tExit"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "grep",
                        usage: "[-HhnlLoqvsriwFE] [-m N] [-A/B/C N] PATTERN/-e PATTERN.../-f FILE [FILE]...\n\nSearch for PATTERN in FILEs (or stdin)\n\n\t-H\tAdd \'filename:\' prefix\n\t-h\tDo not add \'filename:\' prefix\n\t-n\tAdd \'line_no:\' prefix\n\t-l\tShow only names of files that match\n\t-L\tShow only names of files that don\'t match\n\t-c\tShow only count of matching lines\n\t-o\tShow only the matching part of line\n\t-q\tQuiet. Return 0 if PATTERN is found, 1 otherwise\n\t-v\tSelect non-matching lines\n\t-s\tSuppress open and read errors\n\t-r\tRecurse\n\t-i\tIgnore case\n\t-w\tMatch whole words only\n\t-x\tMatch whole lines only\n\t-F\tPATTERN is a literal (not regexp)\n\t-E\tPATTERN is an extended regexp\n\t-m N\tMatch up to N times per file\n\t-A N\tPrint N lines of trailing context\n\t-B N\tPrint N lines of leading context\n\t-C N\tSame as \'-A N -B N\'\n\t-e PTRN\tPattern to match\n\t-f FILE\tRead pattern from file"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "egrep",
      usage: "\x08",
    };
    init
  },
  {
    let mut init = usage_data {
      aname: "fgrep",
      usage: "\x08",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "xargs",
                        usage: "[OPTIONS] [PROG ARGS]\n\nRun PROG on every item given by stdin\n\n\t-0\tInput is separated by NULs\n\t-a FILE\tRead from FILE instead of stdin\n\t-r\tDon\'t run command if input is empty\n\t-t\tPrint the command on stderr before execution\n\t-p\tAsk user whether to run each command\n\t-E STR,-e[STR]\tSTR stops input processing\n\t-I STR\tReplace STR within PROG ARGS with input line\n\t-n N\tPass no more than N args to PROG\n\t-s N\tPass command line of no more than N bytes\n\t-P N\tRun up to N PROGs in parallel\n\t-x\tExit if size is exceeded"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "bootchartd",
                        usage: "start [PROG ARGS]|stop|init\n\nCreate /var/log/bootchart.tgz with boot chart data\n\nstart: start background logging; with PROG, run PROG, then kill logging with USR1\nstop: send USR1 to all bootchartd processes\ninit: start background logging; stop when getty/xdm is seen (for init scripts)\nUnder PID 1: as init, then exec $bootchart_init, /init, /sbin/init"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "halt",
                        usage: "[-d DELAY] [-n] [-f] [-w]\n\nHalt the system\n\n\t-d SEC\tDelay interval\n\t-n\tDo not sync\n\t-f\tForce (don\'t go through init)\n\t-w\tOnly write a wtmp record"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "poweroff",
                        usage: "[-d DELAY] [-n] [-f]\n\nHalt and shut off power\n\n\t-d SEC\tDelay interval\n\t-n\tDo not sync\n\t-f\tForce (don\'t go through init)"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "reboot",
                        usage: "[-d DELAY] [-n] [-f]\n\nReboot the system\n\n\t-d SEC\tDelay interval\n\t-n\tDo not sync\n\t-f\tForce (don\'t go through init)"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "init",
                        usage: "\n\nInit is the first process started during boot. It never exits.\nIt (re)spawns children according to /etc/inittab."
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "linuxrc",
      usage: "\x08",
    };
    init
  },
  {
    let mut init = usage_data {
      aname: "nuke",
      usage: "DIR...\n\nRemove DIRs",
    };
    init
  },
  {
    let mut init = usage_data {
      aname: "resume",
      usage: "BLOCKDEV [OFFSET]\n\nRestore system state from \'suspend-to-disk\' data in BLOCKDEV",
    };
    init
  },
  {
    let mut init = usage_data {
      aname: "add-shell",
      usage: "SHELL...\n\nAdd SHELLs to /etc/shells",
    };
    init
  },
  {
    let mut init = usage_data {
      aname: "remove-shell",
      usage: "SHELL...\n\nRemove SHELLs from /etc/shells",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "addgroup",
                        usage: "[-g GID] [-S] [USER] GROUP\n\nAdd a group or add a user to a group\n\n\t-g GID\tGroup id\n\t-S\tCreate a system group"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "adduser",
                        usage: "[OPTIONS] USER [GROUP]\n\nCreate new user, or add USER to GROUP\n\n\t-h DIR\t\tHome directory\n\t-g GECOS\tGECOS field\n\t-s SHELL\tLogin shell\n\t-G GRP\t\tGroup\n\t-S\t\tCreate a system user\n\t-D\t\tDon\'t assign a password\n\t-H\t\tDon\'t create home directory\n\t-u UID\t\tUser id\n\t-k SKEL\t\tSkeleton directory (/etc/skel)"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "chpasswd",
                        usage: "[--md5|--encrypted|--crypt-method|--root]\n\nRead user:password from stdin and update /etc/passwd\n\n\t-e,--encrypted\t\tSupplied passwords are in encrypted form\n\t-m,--md5\t\tEncrypt using md5, not des\n\t-c,--crypt-method ALG\tdes,md5,sha256/512 (default des)\n\t-R,--root DIR\t\tDirectory to chroot into"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "cryptpw",
                        usage: "[OPTIONS] [PASSWORD] [SALT]\n\nPrint crypt(3) hashed PASSWORD\n\n\t-P,--password-fd N\tRead password from fd N\n\t-m,--method TYPE\tdes,md5,sha256/512 (default des)\n\t-S,--salt SALT"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "mkpasswd",
                        usage: "[OPTIONS] [PASSWORD] [SALT]\n\nPrint crypt(3) hashed PASSWORD\n\n\t-P,--password-fd N\tRead password from fd N\n\t-m,--method TYPE\tdes,md5,sha256/512 (default des)\n\t-S,--salt SALT"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "deluser",
      usage: "[--remove-home] USER\n\nDelete USER from the system",
    };
    init
  },
  {
    let mut init = usage_data {
      aname: "delgroup",
      usage: "[USER] GROUP\n\nDelete group GROUP from the system or user USER from group GROUP",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "getty",
                        usage: "[OPTIONS] BAUD_RATE[,BAUD_RATE]... TTY [TERMTYPE]\n\nOpen TTY, prompt for login name, then invoke /bin/login\n\n\t-h\t\tEnable hardware RTS/CTS flow control\n\t-L\t\tSet CLOCAL (ignore Carrier Detect state)\n\t-m\t\tGet baud rate from modem\'s CONNECT status message\n\t-n\t\tDon\'t prompt for login name\n\t-w\t\tWait for CR or LF before sending /etc/issue\n\t-i\t\tDon\'t display /etc/issue\n\t-f ISSUE_FILE\tDisplay ISSUE_FILE instead of /etc/issue\n\t-l LOGIN\tInvoke LOGIN instead of /bin/login\n\t-t SEC\t\tTerminate after SEC if no login name is read\n\t-I INITSTR\tSend INITSTR before anything else\n\t-H HOST\t\tLog HOST into the utmp file as the hostname\n\nBAUD_RATE of 0 leaves it unchanged"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "login",
                        usage: "[-p] [-h HOST] [[-f] USER]\n\nBegin a new session on the system\n\n\t-f\tDon\'t authenticate (user already authenticated)\n\t-h HOST\tHost user came from (for network logins)\n\t-p\tPreserve environment"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "passwd",
                        usage: "[OPTIONS] [USER]\n\nChange USER\'s password (default: current user)\n\n\t-a ALG\tdes,md5,sha256/512 (default des)\n\t-d\tSet password to \'\'\n\t-l\tLock (disable) account\n\t-u\tUnlock (enable) account"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "su",
                        usage: "[-lmp] [-] [-s SH] [USER [SCRIPT ARGS / -c \'CMD\' ARG0 ARGS]]\n\nRun shell under USER (by default, root)\n\n\t-,-l\tClear environment, go to home dir, run shell as login shell\n\t-p,-m\tDo not set new $HOME, $SHELL, $USER, $LOGNAME\n\t-c CMD\tCommand to pass to \'sh -c\'\n\t-s SH\tShell to use instead of user\'s default"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "sulogin",
      usage: "[-t N] [TTY]\n\nSingle user login\n\n\t-t N\tTimeout",
    };
    init
  },
  {
    let mut init = usage_data {
      aname: "vlock",
      usage:
        "[-a]\n\nLock a virtual terminal. A password is required to unlock.\n\n\t-a\tLock all VTs",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "makemime",
                        usage: "[OPTIONS] [FILE]...\n\nCreate multipart MIME-encoded message from FILEs\n\n\t-o FILE\tOutput. Default: stdout\n\t-a HDR\tAdd header(s). Examples:\n\t\t\"From: user@host.org\", \"Date: `date -R`\"\n\t-c CT\tContent type. Default: application/octet-stream\n\t-C CS\tCharset. Default: us-ascii\n\nOther options are silently ignored"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "popmaildir",
                        usage: "[OPTIONS] MAILDIR [CONN_HELPER ARGS]\n\nFetch content of remote mailbox to local maildir\n\n\t-s\t\tSkip authorization\n\t-T\t\tGet messages with TOP instead of RETR\n\t-k\t\tKeep retrieved messages on the server\n\t-t SEC\t\tNetwork timeout\n\t-F \'PROG ARGS\'\tFilter program (may be repeated)\n\t-M \'PROG ARGS\'\tDelivery program\n\nFetch from plain POP3 server:\npopmaildir -k DIR nc pop3.server.com 110 <user_and_pass.txt\nFetch from SSLed POP3 server and delete fetched emails:\npopmaildir DIR -- openssl s_client -quiet -connect pop3.server.com:995 <user_and_pass.txt"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "reformime",
                        usage: "[OPTIONS]\n\nParse MIME-encoded message on stdin\n\n\t-x PREFIX\tExtract content of MIME sections to files\n\t-X PROG ARGS\tFilter content of MIME sections through PROG\n\t\t\tMust be the last option\n\nOther options are silently ignored"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "sendmail",
                        usage: "[-tv] [-f SENDER] [-amLOGIN 4<user_pass.txt | -auUSER -apPASS]\n\t\t[-w SECS] [-H \'PROG ARGS\' | -S HOST] [RECIPIENT_EMAIL]...\n\nRead email from stdin and send it\n\nStandard options:\n\t-t\t\tRead additional recipients from message body\n\t-f SENDER\tFor use in MAIL FROM:<sender>. Can be empty string\n\t\t\tDefault: -auUSER, or username of current UID\n\t-o OPTIONS\tVarious options. -oi implied, others are ignored\n\t-i\t\t-oi synonym, implied and ignored\n\nBusybox specific options:\n\t-v\t\tVerbose\n\t-w SECS\t\tNetwork timeout\n\t-H \'PROG ARGS\'\tRun connection helper. Examples:\n\t\topenssl s_client -quiet -tls1 -starttls smtp -connect smtp.gmail.com:25\n\t\topenssl s_client -quiet -tls1 -connect smtp.gmail.com:465\n\t\t\t$SMTP_ANTISPAM_DELAY: seconds to wait after helper connect\n\t-S HOST[:PORT]\tServer (default $SMTPHOST or 127.0.0.1)\n\t-amLOGIN\tLog in using AUTH LOGIN\n\t-amPLAIN\tor AUTH PLAIN\n\t\t\t(-amCRAM-MD5 not supported)\n\t-auUSER\t\tUsername for AUTH\n\t-apPASS \tPassword for AUTH\n\nIf no -a options are given, authentication is not done.\nIf -amLOGIN is given but no -au/-ap, user/password is read from fd #4.\nOther options are silently ignored; -oi is implied.\nUse makemime to create emails with attachments."
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "adjtimex",
                        usage: "[-q] [-o OFF] [-f FREQ] [-p TCONST] [-t TICK]\n\nRead or set kernel time variables. See adjtimex(2)\n\n\t-q\tQuiet\n\t-o OFF\tTime offset, microseconds\n\t-f FREQ\tFrequency adjust, integer kernel units (65536 is 1ppm)\n\t-t TICK\tMicroseconds per tick, usually 10000\n\t\t(positive -t or -f values make clock run faster)\n\t-p TCONST"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "bc",
                        usage: "[-sqlw] FILE...\n\nArbitrary precision calculator\n\n\t-q\tQuiet\n\t-l\tLoad standard math library\n\t-s\tBe POSIX compatible\n\t-w\tWarn if extensions are used\n\n$BC_LINE_LENGTH changes output width"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "dc",
                        usage: "[-x] [-eSCRIPT]... [-fFILE]... [FILE]...\n\nTiny RPN calculator. Operations:\n+, -, *, /, %, ~, ^, |,\np - print top of the stack without popping\nf - print entire stack\nk - pop the value and set the precision\ni - pop the value and set input radix\no - pop the value and set output radix\nExamples: dc -e\'2 2 + p\' -> 4, dc -e\'8 8 * 2 2 + / p\' -> 16"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "beep",
                        usage: "-f FREQ -l LEN -d DELAY -r COUNT -n\n\n\t-f\tFrequency in Hz\n\t-l\tLength in ms\n\t-d\tDelay in ms\n\t-r\tRepetitions\n\t-n\tStart new tone"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "chat",
                        usage: "EXPECT [SEND [EXPECT [SEND...]]]\n\nUseful for interacting with a modem connected to stdin/stdout.\nA script consists of \"expect-send\" argument pairs.\nExample:\nchat \'\' ATZ OK ATD123456 CONNECT \'\' ogin: pppuser word: ppppass \'~\'"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "conspy",
                        usage: "[-vcsndfFQ] [-x COL] [-y LINE] [CONSOLE_NO]\n\nA text-mode VNC like program for Linux virtual consoles.\nTo exit, quickly press ESC 3 times.\n\n\t-v\tDon\'t send keystrokes to the console\n\t-c\tCreate missing /dev/{tty,vcsa}N\n\t-s\tOpen a SHELL session\n\t-n\tBlack & white\n\t-d\tDump console to stdout\n\t-f\tFollow cursor\n\t-F\tAssume console is on a framebuffer device\n\t-Q\tDisable exit on ESC-ESC-ESC\n\t-x COL\tStarting column\n\t-y LINE\tStarting line"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "crond",
                        usage: "-fbS -l N -d N -L LOGFILE -c DIR\n\n\t-f\tForeground\n\t-b\tBackground (default)\n\t-S\tLog to syslog (default)\n\t-l N\tSet log level. Most verbose 0, default 8\n\t-d N\tSet log level, log to stderr\n\t-L FILE\tLog to FILE\n\t-c DIR\tCron dir. Default:/var/spool/cron/crontabs"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "crontab",
                        usage: "[-c DIR] [-u USER] [-ler]|[FILE]\n\n\t-c\tCrontab directory\n\t-u\tUser\n\t-l\tList crontab\n\t-e\tEdit crontab\n\t-r\tDelete crontab\n\tFILE\tReplace crontab by FILE (\'-\': stdin)"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "devmem",
                        usage: "ADDRESS [WIDTH [VALUE]]\n\nRead/write from physical address\n\n\tADDRESS\tAddress to act upon\n\tWIDTH\tWidth (8/16/...)\n\tVALUE\tData to be written"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "fbsplash",
                        usage: "-s IMGFILE [-c] [-d DEV] [-i INIFILE] [-f CMD]\n\n\t-s\tImage\n\t-c\tHide cursor\n\t-d\tFramebuffer device (default /dev/fb0)\n\t-i\tConfig file (var=value):\n\t\t\tBAR_LEFT,BAR_TOP,BAR_WIDTH,BAR_HEIGHT\n\t\t\tBAR_R,BAR_G,BAR_B,IMG_LEFT,IMG_TOP\n\t-f\tControl pipe (else exit after drawing image)\n\t\t\tcommands: \'NN\' (% for progress bar) or \'exit\'"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "hdparm",
                        usage: "[OPTIONS] [DEVICE]\n\n\t-a\tGet/set fs readahead\n\t-A\tSet drive read-lookahead flag (0/1)\n\t-b\tGet/set bus state (0 == off, 1 == on, 2 == tristate)\n\t-B\tSet Advanced Power Management setting (1-255)\n\t-c\tGet/set IDE 32-bit IO setting\n\t-C\tCheck IDE power mode status\n\t-d\tGet/set using_dma flag\n\t-D\tEnable/disable drive defect-mgmt\n\t-f\tFlush buffer cache for device on exit\n\t-g\tDisplay drive geometry\n\t-h\tDisplay terse usage information\n\t-i\tDisplay drive identification\n\t-I\tDetailed/current information directly from drive\n\t-k\tGet/set keep_settings_over_reset flag (0/1)\n\t-K\tSet drive keep_features_over_reset flag (0/1)\n\t-L\tSet drive doorlock (0/1) (removable harddisks only)\n\t-m\tGet/set multiple sector count\n\t-n\tGet/set ignore-write-errors flag (0/1)\n\t-p\tSet PIO mode on IDE interface chipset (0,1,2,3,4,...)\n\t-P\tSet drive prefetch count\n\t-Q\tGet/set DMA tagged-queuing depth (if supported)\n\t-r\tGet/set readonly flag (DANGEROUS to set)\n\t-R\tRegister an IDE interface (DANGEROUS)\n\t-S\tSet standby (spindown) timeout\n\t-t\tPerform device read timings\n\t-T\tPerform cache read timings\n\t-u\tGet/set unmaskirq flag (0/1)\n\t-U\tUnregister an IDE interface (DANGEROUS)\n\t-v\tDefaults; same as -mcudkrag for IDE drives\n\t-V\tDisplay program version and exit immediately\n\t-w\tPerform device reset (DANGEROUS)\n\t-W\tSet drive write-caching flag (0/1) (DANGEROUS)\n\t-x\tTristate device for hotswap (0/1) (DANGEROUS)\n\t-X\tSet IDE xfer mode (DANGEROUS)\n\t-y\tPut IDE drive in standby mode\n\t-Y\tPut IDE drive to sleep\n\t-Z\tDisable Seagate auto-powersaving mode\n\t-z\tReread partition table"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "hexedit",
      usage: "FILE\n\nEdit FILE in hexadecimal",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "i2cget",
                        usage: "[-fy] BUS CHIP-ADDRESS [DATA-ADDRESS [MODE]]\n\nRead from I2C/SMBus chip registers\n\n\tI2CBUS\tI2C bus number\n\tADDRESS\t0x03-0x77\nMODE is:\n\tb\tRead byte data (default)\n\tw\tRead word data\n\tc\tWrite byte/read byte\n\tAppend p for SMBus PEC\n\n\t-f\tForce access\n\t-y\tDisable interactive mode"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "i2cset",
                        usage: "[-fy] [-m MASK] BUS CHIP-ADDRESS DATA-ADDRESS [VALUE] ... [MODE]\n\nSet I2C registers\n\n\tI2CBUS\tI2C bus number\n\tADDRESS\t0x03-0x77\nMODE is:\n\tc\tByte, no value\n\tb\tByte data (default)\n\tw\tWord data\n\ti\tI2C block data\n\ts\tSMBus block data\n\tAppend p for SMBus PEC\n\n\t-f\tForce access\n\t-y\tDisable interactive mode\n\t-r\tRead back and compare the result\n\t-m MASK\tMask specifying which bits to write"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "i2cdump",
                        usage: "[-fy] [-r FIRST-LAST] BUS ADDR [MODE]\n\nExamine I2C registers\n\n\tI2CBUS\tI2C bus number\n\tADDRESS\t0x03-0x77\nMODE is:\n\tb\tByte (default)\n\tw\tWord\n\tW\tWord on even register addresses\n\ti\tI2C block\n\ts\tSMBus block\n\tc\tConsecutive byte\n\tAppend p for SMBus PEC\n\n\t-f\tForce access\n\t-y\tDisable interactive mode\n\t-r\tLimit the number of registers being accessed"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "i2cdetect",
                        usage: "-l | -F I2CBUS | [-ya] [-q|-r] I2CBUS [FIRST LAST]\n\nDetect I2C chips\n\n\t-l\tList installed buses\n\t-F BUS#\tList functionalities on this bus\n\t-y\tDisable interactive mode\n\t-a\tForce scanning of non-regular addresses\n\t-q\tUse smbus quick write commands for probing (default)\n\t-r\tUse smbus read byte commands for probing\n\tFIRST and LAST limit probing range"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "i2ctransfer",
                        usage: "[-fay] I2CBUS {rLENGTH[@ADDR] | wLENGTH[@ADDR] DATA...}...\n\nRead/write I2C data in one transfer\n\n\t-f\tForce access to busy addresses\n\t-a\tForce access to non-regular addresses\n\t-y\tDisable interactive mode"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "less",
                        usage: "[-EFIMmNSRh~] [FILE]...\n\nView FILE (or stdin) one screenful at a time\n\n\t-E\tQuit once the end of a file is reached\n\t-F\tQuit if entire file fits on first screen\n\t-I\tIgnore case in all searches\n\t-M,-m\tDisplay status line with line numbers\n\t\tand percentage through the file\n\t-N\tPrefix line number to each line\n\t-S\tTruncate long lines\n\t-R\tRemove color escape codes in input\n\t-~\tSuppress ~s displayed past EOF"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "lsscsi",
      usage: "\x08",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "makedevs",
                        usage: "[-d device_table] rootdir\n\nCreate a range of special files as specified in a device table.\nDevice table entries take the form of:\n<name> <type> <mode> <uid> <gid> <major> <minor> <start> <inc> <count>\nWhere name is the file name, type can be one of:\n\tf\tRegular file\n\td\tDirectory\n\tc\tCharacter device\n\tb\tBlock device\n\tp\tFifo (named pipe)\nuid is the user id for the target file, gid is the group id for the\ntarget file. The rest of the entries (major, minor, etc) apply to\nto device special files. A \'-\' may be used for blank entries."
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "man",
                        usage: "[-aw] MANPAGE...\n\nDisplay manual page\n\n\t-a\tDisplay all pages\n\t-w\tShow page locations\n\n$COLUMNS overrides output width"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "microcom",
                        usage: "[-d DELAY] [-t TIMEOUT] [-s SPEED] [-X] TTY\n\nCopy bytes for stdin to TTY and from TTY to stdout\n\n\t-d\tWait up to DELAY ms for TTY output before sending every\n\t\tnext byte to it\n\t-t\tExit if both stdin and TTY are silent for TIMEOUT ms\n\t-s\tSet serial line to SPEED\n\t-X\tDisable special meaning of NUL and Ctrl-X from stdin"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "mt",
                        usage: "[-f device] opcode value\n\nControl magnetic tape drive operation\n\nAvailable Opcodes:\n\nbsf bsfm bsr bss datacompression drvbuffer eof eom erase\nfsf fsfm fsr fss load lock mkpart nop offline ras1 ras2\nras3 reset retension rewind rewoffline seek setblk setdensity\nsetpart tell unload unlock weof wset"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "nandwrite",
                        usage: "[-np] [-s ADDR] MTD_DEVICE [FILE]\n\nWrite to MTD_DEVICE\n\n\t-n\tWrite without ecc\n\t-p\tPad to page size\n\t-s ADDR\tStart address"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "nanddump",
                        usage: "[-no] [--bb padbad|skipbad] [-s ADDR] [-l LEN] [-f FILE] MTD_DEVICE\n\nDump MTD_DEVICE\n\n\t-n\tRead without ecc\n\t-o\tDump oob data\n\t-s ADDR\tStart address\n\t-l LEN\tLength\n\t-f FILE\tDump to file (\'-\' for stdout)\n\t--bb METHOD\n\t\tskipbad: skip bad blocks\n\t\tpadbad: substitute bad blocks by 0xff (default)"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "partprobe",
      usage: "DEVICE...\n\nAsk kernel to rescan partition table",
    };
    init
  },
  {
    let mut init = usage_data {
      aname: "raidautorun",
      usage: "DEVICE\n\nTell the kernel to automatically search and start RAID arrays",
    };
    init
  },
  {
    let mut init = usage_data {
      aname: "readahead",
      usage: "[FILE]...\n\nPreload FILEs to RAM",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "runlevel",
                        usage: "[FILE]\n\nFind the current and previous system runlevel\n\nIf no utmp FILE exists or if no runlevel record can be found,\nprint \"unknown\""
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "rx",
      usage: "FILE\n\nReceive a file using the xmodem protocol",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "setfattr",
                        usage: "[-h] -n|-x ATTR [-v VALUE] FILE...\n\nSet extended attributes\n\n\t-h\t\tDo not follow symlinks\n\t-x ATTR\t\tRemove attribute ATTR\n\t-n ATTR\t\tSet attribute ATTR to VALUE\n\t-v VALUE\t(default: empty)"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "setserial",
                        usage: "[-abGvz] { DEVICE [PARAMETER [ARG]]... | -g DEVICE... }\n\nPrint or set serial port parameters\n\n\t-a\tPrint all\n\t-b\tPrint summary\n\t-G\tPrint as setserial PARAMETERs\n\t-v\tVerbose\n\t-z\tZero out serial flags before setting\n\t-g\tAll args are device names\n\nPARAMETERs: (* = takes ARG, ^ = can be turned off by preceding ^)\n\t*port, *irq, *divisor, *uart, *baud_base, *close_delay, *closing_wait,\n\t^fourport, ^auto_irq, ^skip_test, ^sak, ^session_lockout, ^pgrp_lockout,\n\t^callout_nohup, ^split_termios, ^hup_notify, ^low_latency, autoconfig,\n\tspd_normal, spd_hi, spd_vhi, spd_shi, spd_warp, spd_cust\nARG for uart:\n\tunknown, 8250, 16450, 16550, 16550A, Cirrus, 16650, 16650V2, 16750,\n\t16950, 16954, 16654, 16850, RSA, NS16550A, XSCALE, RM9000, OCTEON, AR7,\n\tU6_16550A"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "strings",
                        usage: "[-fo] [-t o/d/x] [-n LEN] [FILE]...\n\nDisplay printable strings in a binary file\n\n\t-f\t\tPrecede strings with filenames\n\t-o\t\tPrecede strings with octal offsets\n\t-t o/d/x\tPrecede strings with offsets in base 8/10/16\n\t-n LEN\t\tAt least LEN characters form a string (default 4)"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "time",
                        usage: "[-vpa] [-o FILE] PROG ARGS\n\nRun PROG, display resource usage when it exits\n\n\t-v\tVerbose\n\t-p\tPOSIX output format\n\t-f FMT\tCustom format\n\t-o FILE\tWrite result to FILE\n\t-a\tAppend (else overwrite)"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "ts",
      usage: "[-is] [STRFTIME]",
    };
    init
  },
  {
    let mut init = usage_data {
      aname: "ttysize",
      usage: "[w] [h]\n\nPrint dimensions of stdin tty, or 80x24",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "ubiattach",
                        usage: "-m MTD_NUM [-d UBI_NUM] [-O VID_HDR_OFF] UBI_CTRL_DEV\n\nAttach MTD device to UBI\n\n\t-m MTD_NUM\tMTD device number to attach\n\t-d UBI_NUM\tUBI device number to assign\n\t-O VID_HDR_OFF\tVID header offset"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "ubidetach",
      usage:
        "-d UBI_NUM UBI_CTRL_DEV\n\nDetach MTD device from UBI\n\n\t-d UBI_NUM\tUBI device number",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "ubimkvol",
                        usage: "-N NAME [-s SIZE | -m] UBI_DEVICE\n\nCreate UBI volume\n\n\t-a ALIGNMENT\tVolume alignment (default 1)\n\t-m\t\tSet volume size to maximum available\n\t-n VOLID\tVolume ID. If not specified,\n\t\t\tassigned automatically\n\t-N NAME\t\tVolume name\n\t-s SIZE\t\tSize in bytes\n\t-t TYPE\t\tVolume type (static|dynamic)"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "ubirmvol",
                        usage: "-n VOLID / -N VOLNAME UBI_DEVICE\n\nRemove UBI volume\n\n\t-n VOLID\tVolume ID\n\t-N VOLNAME\tVolume name"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "ubirsvol",
                        usage: "-n VOLID -s SIZE UBI_DEVICE\n\nResize UBI volume\n\n\t-n VOLID\tVolume ID\n\t-s SIZE\t\tSize in bytes"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "ubiupdatevol",
                        usage: "-t UBI_DEVICE | [-s SIZE] UBI_DEVICE IMG_FILE\n\nUpdate UBI volume\n\n\t-t\tTruncate to zero size\n\t-s SIZE\tSize in bytes to resize to"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "ubirename",
      usage:
        "UBI_DEVICE OLD_VOLNAME NEW_VOLNAME [OLD2 NEW2]...\n\nRename UBI volumes on UBI_DEVICE",
    };
    init
  },
  {
    let mut init = usage_data {
      aname: "volname",
      usage: "[DEVICE]\n\nShow CD volume name of the DEVICE (default /dev/cdrom)",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "watchdog",
                        usage: "[-t N[ms]] [-T N[ms]] [-F] DEV\n\nPeriodically write to watchdog device DEV\n\n\t-T N\tReboot after N seconds if not reset (default 60)\n\t-t N\tReset every N seconds (default 30)\n\t-F\tRun in foreground\n\nUse 500ms to specify period in milliseconds"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "modinfo",
                        usage: "[-adlpn0] [-F keyword] MODULE\n\n\t-a\t\tShortcut for \'-F author\'\n\t-d\t\tShortcut for \'-F description\'\n\t-l\t\tShortcut for \'-F license\'\n\t-p\t\tShortcut for \'-F parm\'\n\t-F keyword\tKeyword to look for\n\t-0\t\tSeparate output with NULs"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "lsmod",
      usage: "\n\nList loaded kernel modules",
    };
    init
  },
  {
    let mut init = usage_data {
      aname: "modprobe",
      usage: "[-rq] MODULE [SYMBOL=VALUE]...\n\n\t-r\tRemove MODULE\n\t-q\tQuiet",
    };
    init
  },
  {
    let mut init = usage_data {
      aname: "depmod",
      usage: "[-n]\n\nGenerate modules.dep.bb\n\n\t-n\tDry run: print file to stdout",
    };
    init
  },
  {
    let mut init = usage_data {
      aname: "insmod",
      usage: "FILE [SYMBOL=VALUE]...\n\nLoad kernel module",
    };
    init
  },
  {
    let mut init = usage_data {
      aname: "rmmod",
      usage: "MODULE...\n\nUnload kernel modules",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "arp",
                        usage: "\n[-vn]\t[-H HWTYPE] [-i IF] -a [HOSTNAME]\n[-v]\t\t    [-i IF] -d HOSTNAME [pub]\n[-v]\t[-H HWTYPE] [-i IF] -s HOSTNAME HWADDR [temp]\n[-v]\t[-H HWTYPE] [-i IF] -s HOSTNAME HWADDR [netmask MASK] pub\n[-v]\t[-H HWTYPE] [-i IF] -Ds HOSTNAME IFACE [netmask MASK] pub\n\nManipulate ARP cache\n\n\t-a\t\tDisplay (all) hosts\n\t-d\t\tDelete ARP entry\n\t-s\t\tSet new entry\n\t-v\t\tVerbose\n\t-n\t\tDon\'t resolve names\n\t-i IF\t\tNetwork interface\n\t-D\t\tRead HWADDR from IFACE\n\t-A,-p AF\tProtocol family\n\t-H HWTYPE\tHardware address type"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "arping",
                        usage: "[-fqbDUA] [-c CNT] [-w TIMEOUT] [-I IFACE] [-s SRC_IP] DST_IP\n\nSend ARP requests/replies\n\n\t-f\t\tQuit on first ARP reply\n\t-q\t\tQuiet\n\t-b\t\tKeep broadcasting, don\'t go unicast\n\t-D\t\tExit with 1 if DST_IP replies\n\t-U\t\tUnsolicited ARP mode, update your neighbors\n\t-A\t\tARP answer mode, update your neighbors\n\t-c N\t\tStop after sending N ARP requests\n\t-w TIMEOUT\tSeconds to wait for ARP reply\n\t-I IFACE\tInterface to use (default eth0)\n\t-s SRC_IP\tSender IP address\n\tDST_IP\t\tTarget IP address"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "brctl",
                        usage: "COMMAND [BRIDGE [ARGS]]\n\nManage ethernet bridges\nCommands:\n\tshow [BRIDGE]...\tShow bridges\n\taddbr BRIDGE\t\tCreate BRIDGE\n\tdelbr BRIDGE\t\tDelete BRIDGE\n\taddif BRIDGE IFACE\tAdd IFACE to BRIDGE\n\tdelif BRIDGE IFACE\tDelete IFACE from BRIDGE\n\tshowmacs BRIDGE\t\t\tList MAC addresses\n\tshowstp\tBRIDGE\t\t\tShow STP info\n\tstp BRIDGE 1/yes/on|0/no/off\tSet STP on/off\n\tsetageing BRIDGE SECONDS\tSet ageing time\n\tsetfd BRIDGE SECONDS\t\tSet bridge forward delay\n\tsethello BRIDGE SECONDS\t\tSet hello time\n\tsetmaxage BRIDGE SECONDS\tSet max message age\n\tsetbridgeprio BRIDGE PRIO\tSet bridge priority\n\tsetportprio BRIDGE IFACE PRIO\tSet port priority\n\tsetpathcost BRIDGE IFACE COST\tSet path cost"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "dnsd",
                        usage: "[-dvs] [-c CONFFILE] [-t TTL_SEC] [-p PORT] [-i ADDR]\n\nSmall static DNS server daemon\n\n\t-c FILE\tConfig file\n\t-t SEC\tTTL\n\t-p PORT\tListen on PORT\n\t-i ADDR\tListen on ADDR\n\t-d\tDaemonize\n\t-v\tVerbose\n\t-s\tSend successful replies only. Use this if you want\n\t\tto use /etc/resolv.conf with two nameserver lines:\n\t\t\tnameserver DNSD_SERVER\n\t\t\tnameserver NORMAL_DNS_SERVER"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "ether-wake",
                        usage: "[-b] [-i IFACE] [-p aa:bb:cc:dd[:ee:ff]/a.b.c.d] MAC\n\nSend a magic packet to wake up sleeping machines.\nMAC must be a station address (00:11:22:33:44:55) or\na hostname with a known \'ethers\' entry.\n\n\t-b\t\tBroadcast the packet\n\t-i IFACE\tInterface to use (default eth0)\n\t-p PASSWORD\tAppend four or six byte PASSWORD to the packet"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "ftpd",
                        usage: "[-wvS] [-a USER] [-t N] [-T N] [DIR]\n\nFTP server. Chroots to DIR, if this fails (run by non-root), cds to it.\nShould be used as inetd service, inetd.conf line:\n\t21 stream tcp nowait root ftpd ftpd /files/to/serve\nCan be run from tcpsvd:\n\ttcpsvd -vE 0.0.0.0 21 ftpd /files/to/serve\n\n\t-w\tAllow upload\n\t-A\tNo login required, client access occurs under ftpd\'s UID\n\t-a USER\tEnable \'anonymous\' login and map it to USER\n\t-v\tLog errors to stderr. -vv: verbose log\n\t-S\tLog errors to syslog. -SS: verbose log\n\t-t,-T N\tIdle and absolute timeout"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "ftpget",
                        usage: "[OPTIONS] HOST [LOCAL_FILE] REMOTE_FILE\n\nDownload a file via FTP\n\n\t-c\tContinue previous transfer\n\t-v\tVerbose\n\t-u USER\tUsername\n\t-p PASS\tPassword\n\t-P NUM\tPort"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "ftpput",
                        usage: "[OPTIONS] HOST [REMOTE_FILE] LOCAL_FILE\n\nUpload a file to a FTP server\n\n\t-v\tVerbose\n\t-u USER\tUsername\n\t-p PASS\tPassword\n\t-P NUM\tPort number"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "dnsdomainname",
      usage: "\x08",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "hostname",
                        usage: "[OPTIONS] [HOSTNAME | -F FILE]\n\nGet or set hostname or DNS domain name\n\n\t-s\tShort\n\t-i\tAddresses for the hostname\n\t-d\tDNS domain name\n\t-f\tFully qualified domain name\n\t-F FILE\tUse FILE\'s content as hostname"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "httpd",
                        usage: "[-ifv[v]] [-c CONFFILE] [-p [IP:]PORT] [-u USER[:GRP]] [-r REALM] [-h HOME]\nor httpd -d/-e/-m STRING\n\nListen for incoming HTTP requests\n\n\t-i\t\tInetd mode\n\t-f\t\tDon\'t daemonize\n\t-v[v]\t\tVerbose\n\t-p [IP:]PORT\tBind to IP:PORT (default *:80)\n\t-u USER[:GRP]\tSet uid/gid after binding to port\n\t-r REALM\tAuthentication Realm for Basic Authentication\n\t-h HOME\t\tHome directory (default .)\n\t-c FILE\t\tConfiguration file (default {/etc,HOME}/httpd.conf)\n\t-m STRING\tMD5 crypt STRING\n\t-e STRING\tHTML encode STRING\n\t-d STRING\tURL decode STRING"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "ifconfig",
                        usage: "[-a] interface [address]\n\nConfigure a network interface\n\n\t[add ADDRESS[/PREFIXLEN]]\n\t[del ADDRESS[/PREFIXLEN]]\n\t[[-]broadcast [ADDRESS]] [[-]pointopoint [ADDRESS]]\n\t[netmask ADDRESS] [dstaddr ADDRESS]\n\t[outfill NN] [keepalive NN]\n\t[hw ether|infiniband ADDRESS] [metric NN] [mtu NN]\n\t[[-]trailers] [[-]arp] [[-]allmulti]\n\t[multicast] [[-]promisc] [txqueuelen NN] [[-]dynamic]\n\t[mem_start NN] [io_addr NN] [irq NN]\n\t[up|down] ..."
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "ifenslave",
                        usage: "[-cdf] MASTER_IFACE SLAVE_IFACE...\n\nConfigure network interfaces for parallel routing\n\n\t-c\tChange active slave\n\t-d\tRemove slave interface from bonding device\n\t-f\tForce, even if interface is not Ethernet"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "ifplugd",
                        usage: "[OPTIONS]\n\nNetwork interface plug detection daemon\n\n\t-n\t\tDon\'t daemonize\n\t-s\t\tDon\'t log to syslog\n\t-i IFACE\tInterface\n\t-f/-F\t\tTreat link detection error as link down/link up\n\t\t\t(otherwise exit on error)\n\t-a\t\tDon\'t up interface at each link probe\n\t-M\t\tMonitor creation/destruction of interface\n\t\t\t(otherwise it must exist)\n\t-r PROG\t\tScript to run\n\t-x ARG\t\tExtra argument for script\n\t-I\t\tDon\'t exit on nonzero exit code from script\n\t-p\t\tDon\'t run \"up\" script on startup\n\t-q\t\tDon\'t run \"down\" script on exit\n\t-l\t\tAlways run script on startup\n\t-t SECS\t\tPoll time in seconds\n\t-u SECS\t\tDelay before running script after link up\n\t-d SECS\t\tDelay after link down\n\t-m MODE\t\tAPI mode (mii, priv, ethtool, wlan, iff, auto)\n\t-k\t\tKill running daemon"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "ifup",
                        usage: "[-anmvf] [-i FILE] IFACE...\n\n\t-a\tConfigure all interfaces\n\t-i FILE\tUse FILE instead of /etc/network/interfaces\n\t-n\tPrint out what would happen, but don\'t do it\n\t\t(note: doesn\'t disable mappings)\n\t-m\tDon\'t run any mappings\n\t-v\tPrint out what would happen before doing it\n\t-f\tForce configuration"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "ifdown",
                        usage: "[-anmvf] [-i FILE] IFACE...\n\n\t-a\tDeconfigure all interfaces\n\t-i FILE\tUse FILE for interface definitions\n\t-n\tPrint out what would happen, but don\'t do it\n\t\t(note: doesn\'t disable mappings)\n\t-m\tDon\'t run any mappings\n\t-v\tPrint out what would happen before doing it\n\t-f\tForce deconfiguration"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "inetd",
                        usage: "[-fe] [-q N] [-R N] [CONFFILE]\n\nListen for network connections and launch programs\n\n\t-f\tRun in foreground\n\t-e\tLog to stderr\n\t-q N\tSocket listen queue (default 128)\n\t-R N\tPause services after N connects/min\n\t\t(default 0 - disabled)\n\tDefault CONFFILE is /etc/inetd.conf"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "ip",
                        usage: "[OPTIONS] address|route|link|tunnel|neigh|rule [ARGS]\n\nOPTIONS := -f[amily] inet|inet6|link | -o[neline]\n\nip addr add|del IFADDR dev IFACE | show|flush [dev IFACE] [to PREFIX]\nip route list|flush|add|del|change|append|replace|test ROUTE\nip link set IFACE [up|down] [arp on|off] [multicast on|off]\n\t[promisc on|off] [mtu NUM] [name NAME] [qlen NUM] [address MAC]\n\t[master IFACE | nomaster]\nip tunnel add|change|del|show [NAME]\n\t[mode ipip|gre|sit] [remote ADDR] [local ADDR] [ttl TTL]\nip neigh show|flush [to PREFIX] [dev DEV] [nud STATE]\nip rule [list] | add|del SELECTOR ACTION"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "ipaddr",
                        usage: "add|del IFADDR dev IFACE | show|flush [dev IFACE] [to PREFIX]\n\nipaddr add|change|replace|delete dev IFACE IFADDR\n\tIFADDR := PREFIX | ADDR peer PREFIX [broadcast ADDR|+|-]\n\t\t[anycast ADDR] [label STRING] [scope SCOPE]\n\tPREFIX := ADDR[/MASK]\n\tSCOPE := [host|link|global|NUMBER]\nipaddr show|flush [dev IFACE] [scope SCOPE] [to PREFIX] [label PATTERN]"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "iplink",
                        usage: "set IFACE [up|down] [arp on|off] [multicast on|off]\n\t[promisc on|off] [mtu NUM] [name NAME] [qlen NUM] [address MAC]\n\t[master IFACE | nomaster]\niplink add [link IFACE] IFACE [address MAC] type TYPE [ARGS]\niplink delete IFACE type TYPE [ARGS]\n\tTYPE ARGS := vlan VLANARGS | vrf table NUM\n\tVLANARGS := id VLANID [protocol 802.1q|802.1ad] [reorder_hdr on|off]\n\t\t[gvrp on|off] [mvrp on|off] [loose_binding on|off]\niplink show [IFACE]"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "iproute",
                        usage: "list|flush|add|del|change|append|replace|test ROUTE\n\niproute list|flush SELECTOR\n\tSELECTOR := [root PREFIX] [match PREFIX] [proto RTPROTO]\n\tPREFIX := default|ADDR[/MASK]\niproute get ADDR [from ADDR iif IFACE]\n\t[oif IFACE] [tos TOS]\niproute add|del|change|append|replace|test ROUTE\n\tROUTE := NODE_SPEC [INFO_SPEC]\n\tNODE_SPEC := PREFIX [table TABLE_ID] [proto RTPROTO] [scope SCOPE] [metric METRIC]\n\tINFO_SPEC := NH OPTIONS\n\tNH := [via [inet|inet6] ADDR] [dev IFACE] [src ADDR] [onlink]\n\tOPTIONS := [mtu [lock] NUM] [advmss [lock] NUM]"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "iprule",
                        usage: "[list] | add|del SELECTOR ACTION\n\n\tSELECTOR := [from PREFIX] [to PREFIX] [tos TOS] [fwmark FWMARK]\n\t\t\t[dev IFACE] [pref NUMBER]\n\tACTION := [table TABLE_ID] [nat ADDR]\n\t\t\t[prohibit|reject|unreachable]\n\t\t\t[realms [SRCREALM/]DSTREALM]\n\tTABLE_ID := [local|main|default|NUMBER]"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "iptunnel",
                        usage: "add|change|del|show [NAME]\n\t[mode ipip|gre|sit] [remote ADDR] [local ADDR] [ttl TTL]\n\niptunnel add|change|del|show [NAME]\n\t[mode ipip|gre|sit] [remote ADDR] [local ADDR]\n\t[[i|o]seq] [[i|o]key KEY] [[i|o]csum]\n\t[ttl TTL] [tos TOS] [[no]pmtudisc] [dev PHYS_DEV]"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "ipneigh",
      usage: "show|flush [to PREFIX] [dev DEV] [nud STATE]",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "ipcalc",
                        usage: "[OPTIONS] ADDRESS[/PREFIX] [NETMASK]\n\nCalculate and display network settings from IP address\n\n\t-b\tBroadcast address\n\t-n\tNetwork address\n\t-m\tDefault netmask for IP\n\t-p\tPrefix for IP/NETMASK\n\t-h\tResolved host name\n\t-s\tNo error messages"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "fakeidentd",
                        usage: "[-fiw] [-b ADDR] [STRING]\n\nProvide fake ident (auth) service\n\n\t-f\tRun in foreground\n\t-i\tInetd mode\n\t-w\tInetd \'wait\' mode\n\t-b ADDR\tBind to specified address\n\tSTRING\tIdent answer string (default: nobody)"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "nameif",
                        usage: "[-s] [-c FILE] [IFNAME SELECTOR]...\n\nRename network interface while it in the down state.\nThe device matched by SELECTOR is renamed to IFACE.\nSELECTOR can be a combination of:\n\tdriver=STRING\n\tbus=STRING\n\tphy_address=NUM\n\t[mac=]XX:XX:XX:XX:XX:XX\n\n\t-c FILE\tConfiguration file (default: /etc/mactab)\n\t-s\tLog to syslog"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "nbd-client",
                        usage: "{ [-b BLKSIZE] [-N NAME] [-t SEC] [-p] HOST [PORT] | -d } BLOCKDEV\n\nConnect to HOST and provide network block device on BLOCKDEV"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "nc",
                        usage: "[OPTIONS] HOST PORT  - connect\nnc [OPTIONS] -l -p PORT [HOST] [PORT]  - listen\n\n\t-e PROG\tRun PROG after connect (must be last)\n\t-l\tListen mode, for inbound connects\n\t-lk\tWith -e, provides persistent server\n\t-p PORT\tLocal port\n\t-s ADDR\tLocal address\n\t-w SEC\tTimeout for connects and final net reads\n\t-i SEC\tDelay interval for lines sent\n\t-n\tDon\'t do DNS resolution\n\t-u\tUDP mode\n\t-v\tVerbose\n\t-o FILE\tHex dump traffic\n\t-z\tZero-I/O mode (scanning)"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "netstat",
                        usage: "[-ral] [-tuwx] [-enWp]\n\nDisplay networking information\n\n\t-r\tRouting table\n\t-a\tAll sockets\n\t-l\tListening sockets\n\t\tElse: connected sockets\n\t-t\tTCP sockets\n\t-u\tUDP sockets\n\t-w\tRaw sockets\n\t-x\tUnix sockets\n\t\tElse: all socket types\n\t-e\tOther/more information\n\t-n\tDon\'t resolve names\n\t-W\tWide display\n\t-p\tShow PID/program name for sockets"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "nslookup",
                        usage: "[-type=QUERY_TYPE] [-debug] HOST [DNS_SERVER]\n\nQuery DNS about HOST\n\nQUERY_TYPE: soa,ns,a,aaaa,cname,mx,txt,ptr,any"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "ntpd",
                        usage: "[-dnqNwl] [-I IFACE] [-S PROG] [-k KEYFILE] [-p [keyno:N:]PEER]...\n\nNTP client/server\n\n\t-d\tVerbose (may be repeated)\n\t-n\tDo not daemonize\n\t-q\tQuit after clock is set\n\t-N\tRun at high priority\n\t-w\tDo not set time (only query peers), implies -n\n\t-S PROG\tRun PROG after stepping time, stratum change, and every 11 min\n\t-k FILE\tKey file (ntp.keys compatible)\n\t-p [keyno:NUM:]PEER\n\t\tObtain time from PEER (may be repeated)\n\t\tUse key NUM for authentication\n\t\tIf -p is not given, \'server HOST\' lines\n\t\tfrom /etc/ntp.conf are used\n\t-l\tAlso run as server on port 123\n\t-I IFACE Bind server to IFACE, implies -l"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "ping",
                        usage: "[OPTIONS] HOST\n\nSend ICMP ECHO_REQUEST packets to network hosts\n\n\t-4,-6\t\tForce IP or IPv6 name resolution\n\t-c CNT\t\tSend only CNT pings\n\t-s SIZE\t\tSend SIZE data bytes in packets (default 56)\n\t-i SECS\t\tInterval\n\t-A\t\tPing as soon as reply is recevied\n\t-t TTL\t\tSet TTL\n\t-I IFACE/IP\tSource interface or IP address\n\t-W SEC\t\tSeconds to wait for the first response (default 10)\n\t\t\t(after all -c CNT packets are sent)\n\t-w SEC\t\tSeconds until ping exits (default:infinite)\n\t\t\t(can exit earlier with -c CNT)\n\t-q\t\tQuiet, only display output at start\n\t\t\tand when finished\n\t-p HEXBYTE\tPattern to use for payload"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "ping6",
                        usage: "[OPTIONS] HOST\n\nSend ICMP ECHO_REQUEST packets to network hosts\n\n\t-c CNT\t\tSend only CNT pings\n\t-s SIZE\t\tSend SIZE data bytes in packets (default 56)\n\t-i SECS\t\tInterval\n\t-A\t\tPing as soon as reply is recevied\n\t-I IFACE/IP\tSource interface or IP address\n\t-q\t\tQuiet, only display output at start\n\t\t\tand when finished\n\t-p HEXBYTE\tPattern to use for payload"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "pscan",
                        usage: "[-cb] [-p MIN_PORT] [-P MAX_PORT] [-t TIMEOUT] [-T MIN_RTT] HOST\n\nScan a host, print all open ports\n\n\t-c\tShow closed ports too\n\t-b\tShow blocked ports too\n\t-p\tScan from this port (default 1)\n\t-P\tScan up to this port (default 1024)\n\t-t\tTimeout (default 5000 ms)\n\t-T\tMinimum rtt (default 5 ms, increase for congested hosts)"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "route",
                        usage: "[{add|del|delete}]\n\nEdit kernel routing tables\n\n\t-n\tDon\'t resolve names\n\t-e\tDisplay other/more information\n\t-A inet{6}\tSelect address family"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "slattach",
                        usage: "[-ehmLF] [-c SCRIPT] [-s BAUD] [-p PROTOCOL] SERIAL_DEVICE\n\nConfigure serial line as SLIP network interface\n\n\t-p PROT\tProtocol: slip, cslip (default), slip6, clisp6, adaptive\n\t-s BAUD\tLine speed\n\t-e\tExit after initialization\n\t-h\tExit if carrier is lost (else never exits)\n\t-c PROG\tRun PROG on carrier loss\n\t-m\tDo NOT set raw 8bit mode\n\t-L\tEnable 3-wire operation\n\t-F\tDisable RTS/CTS flow control"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "ssl_client",
      usage: "[-e] -s FD [-r FD] [-n SNI]",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "tc",
                        usage: "OBJECT CMD [dev STRING]\n\nOBJECT: qdisc|class|filter\nCMD: add|del|change|replace|show\n\nqdisc [handle QHANDLE] [root|ingress|parent CLASSID]\n\t[[QDISC_KIND] [help|OPTIONS]]\n\tQDISC_KIND := [p|b]fifo|tbf|prio|cbq|red|etc.\nqdisc show [dev STRING] [ingress]\nclass [classid CLASSID] [root|parent CLASSID]\n\t[[QDISC_KIND] [help|OPTIONS] ]\nclass show [ dev STRING ] [root|parent CLASSID]\nfilter [pref PRIO] [protocol PROTO]\n\t[root|classid CLASSID] [handle FILTERID]\n\t[[FILTER_TYPE] [help|OPTIONS]]\nfilter show [dev STRING] [root|parent CLASSID]"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "tcpsvd",
                        usage: "[-hEv] [-c N] [-C N[:MSG]] [-b N] [-u USER] [-l NAME] IP PORT PROG\n\nCreate TCP socket, bind to IP:PORT and listen for incoming connections.\nRun PROG for each connection.\n\n\tIP PORT\t\tIP:PORT to listen on\n\tPROG ARGS\tProgram to run\n\t-u USER[:GRP]\tChange to user/group after bind\n\t-c N\t\tUp to N connections simultaneously (default 30)\n\t-b N\t\tAllow backlog of approximately N TCP SYNs (default 20)\n\t-C N[:MSG]\tAllow only up to N connections from the same IP:\n\t\t\tnew connections from this IP address are closed\n\t\t\timmediately, MSG is written to the peer before close\n\t-E\t\tDon\'t set up environment\n\t-h\t\tLook up peer\'s hostname\n\t-l NAME\t\tLocal hostname (else look up local hostname in DNS)\n\t-v\t\tVerbose\n\nEnvironment if no -E:\nPROTO=\'TCP\'\nTCPREMOTEADDR=\'ip:port\' (\'[ip]:port\' for IPv6)\nTCPLOCALADDR=\'ip:port\'\nTCPORIGDSTADDR=\'ip:port\' of destination before firewall\n\tUseful for REDIRECTed-to-local connections:\n\tiptables -t nat -A PREROUTING -p tcp --dport 80 -j REDIRECT --to 8080\nTCPCONCURRENCY=num_of_connects_from_this_ip\nIf -h:\nTCPLOCALHOST=\'hostname\' (-l NAME is used if specified)\nTCPREMOTEHOST=\'hostname\'"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "udpsvd",
                        usage: "[-hEv] [-c N] [-u USER] [-l NAME] IP PORT PROG\n\nCreate UDP socket, bind to IP:PORT and wait for incoming packets.\nRun PROG for each packet, redirecting all further packets with same\npeer ip:port to it.\n\n\tIP PORT\t\tIP:PORT to listen on\n\tPROG ARGS\tProgram to run\n\t-u USER[:GRP]\tChange to user/group after bind\n\t-c N\t\tUp to N connections simultaneously (default 30)\n\t-E\t\tDon\'t set up environment\n\t-h\t\tLook up peer\'s hostname\n\t-l NAME\t\tLocal hostname (else look up local hostname in DNS)\n\t-v\t\tVerbose\n\nEnvironment if no -E:\nPROTO=\'UDP\'\nUDPREMOTEADDR=\'ip:port\' (\'[ip]:port\' for IPv6)\nUDPLOCALADDR=\'ip:port\'\nIf -h:\nUDPLOCALHOST=\'hostname\' (-l NAME is used if specified)\nUDPREMOTEHOST=\'hostname\'"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "telnet",
                        usage: "[-a] [-l USER] HOST [PORT]\n\nConnect to telnet server\n\n\t-a\tAutomatic login with $USER variable\n\t-l USER\tAutomatic login as USER"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "telnetd",
                        usage: "[OPTIONS]\n\nHandle incoming telnet connections\n\n\t-l LOGIN\tExec LOGIN on connect\n\t-f ISSUE_FILE\tDisplay ISSUE_FILE instead of /etc/issue\n\t-K\t\tClose connection as soon as login exits\n\t\t\t(normally wait until all programs close slave pty)\n\t-p PORT\t\tPort to listen on\n\t-b ADDR[:PORT]\tAddress to bind to\n\t-F\t\tRun in foreground\n\t-i\t\tInetd mode\n\t-w SEC\t\tInetd \'wait\' mode, linger time SEC\n\t-S\t\tLog to syslog (implied by -i or without -F and -w)"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "tftp",
                        usage: "[OPTIONS] HOST [PORT]\n\nTransfer a file from/to tftp server\n\n\t-l FILE\tLocal FILE\n\t-r FILE\tRemote FILE\n\t-g\tGet file\n\t-p\tPut file\n\t-b SIZE\tTransfer blocks of SIZE octets"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "tftpd",
                        usage: "[-crl] [-u USER] [DIR]\n\nTransfer a file on tftp client\'s request\n\ntftpd should be used as an inetd service.\ntftpd\'s line for inetd.conf:\n\t69 dgram udp nowait root tftpd tftpd -l /files/to/serve\nIt also can be ran from udpsvd:\n\tudpsvd -vE 0.0.0.0 69 tftpd /files/to/serve\n\n\t-r\tProhibit upload\n\t-c\tAllow file creation via upload\n\t-u\tAccess files as USER\n\t-l\tLog to syslog (inetd mode requires this)"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "traceroute",
                        usage: "[-46FIlnrv] [-f 1ST_TTL] [-m MAXTTL] [-q PROBES] [-p PORT]\n\t[-t TOS] [-w WAIT_SEC] [-s SRC_IP] [-i IFACE]\n\t[-z PAUSE_MSEC] HOST [BYTES]\n\nTrace the route to HOST\n\n\t-4,-6\tForce IP or IPv6 name resolution\n\t-F\tSet don\'t fragment bit\n\t-I\tUse ICMP ECHO instead of UDP datagrams\n\t-l\tDisplay TTL value of the returned packet\n\t-n\tPrint numeric addresses\n\t-r\tBypass routing tables, send directly to HOST\n\t-v\tVerbose\n\t-f N\tFirst number of hops (default 1)\n\t-m N\tMax number of hops\n\t-q N\tNumber of probes per hop (default 3)\n\t-p N\tBase UDP port number used in probes\n\t\t(default 33434)\n\t-s IP\tSource address\n\t-i IFACE Source interface\n\t-t N\tType-of-service in probe packets (default 0)\n\t-w SEC\tTime to wait for a response (default 3)\n\t-g IP\tLoose source route gateway (8 max)"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "traceroute6",
                        usage: "[-nrv] [-m MAXTTL] [-q PROBES] [-p PORT]\n\t[-t TOS] [-w WAIT_SEC] [-s SRC_IP] [-i IFACE]\n\tHOST [BYTES]\n\nTrace the route to HOST\n\n\t-n\tPrint numeric addresses\n\t-r\tBypass routing tables, send directly to HOST\n\t-v\tVerbose\n\t-m N\tMax number of hops\n\t-q N\tNumber of probes per hop (default 3)\n\t-p N\tBase UDP port number used in probes\n\t\t(default 33434)\n\t-s IP\tSource address\n\t-i IFACE Source interface\n\t-t N\tType-of-service in probe packets (default 0)\n\t-w SEC\tTime wait for a response (default 3)"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "tunctl",
                        usage: "[-f device] ([-t name] | -d name) [-u owner] [-g group] [-b]\n\nCreate or delete tun interfaces\n\n\t-f name\t\ttun device (/dev/net/tun)\n\t-t name\t\tCreate iface \'name\'\n\t-d name\t\tDelete iface \'name\'\n\t-u owner\tSet iface owner\n\t-g group\tSet iface group\n\t-b\t\tBrief output"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "vconfig",
                        usage: "COMMAND [OPTIONS]\n\nCreate and remove virtual ethernet devices\n\n\tadd\t\tIFACE VLAN_ID\n\trem\t\tVLAN_NAME\n\tset_flag\tIFACE 0|1 VLAN_QOS\n\tset_egress_map\tVLAN_NAME SKB_PRIO VLAN_QOS\n\tset_ingress_map\tVLAN_NAME SKB_PRIO VLAN_QOS\n\tset_name_type\tNAME_TYPE"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "wget",
                        usage: "[-c|--continue] [--spider] [-q|--quiet] [-O|--output-document FILE]\n\t[-o|--output-file FILE] [--header \'header: value\'] [-Y|--proxy on/off]\n\t[-P DIR] [-S|--server-response] [-U|--user-agent AGENT] [-T SEC] URL...\n\nRetrieve files via HTTP or FTP\n\n\t--spider\tOnly check URL existence: $? is 0 if exists\n\t-c\t\tContinue retrieval of aborted transfer\n\t-q\t\tQuiet\n\t-P DIR\t\tSave to DIR (default .)\n\t-S    \t\tShow server response\n\t-T SEC\t\tNetwork read timeout is SEC seconds\n\t-O FILE\t\tSave to FILE (\'-\' for stdout)\n\t-o FILE\t\tLog messages to FILE\n\t-U STR\t\tUse STR for User-Agent header\n\t-Y on/off\tUse proxy"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "whois",
                        usage: "[-i] [-h SERVER] [-p PORT] NAME...\n\nQuery WHOIS info about NAME\n\n\t-i\tShow redirect results too\n\t-h,-p\tServer to query"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "zcip",
                        usage: "[OPTIONS] IFACE SCRIPT\n\nManage a ZeroConf IPv4 link-local address\n\n\t-f\t\tRun in foreground\n\t-q\t\tQuit after obtaining address\n\t-r 169.254.x.x\tRequest this address first\n\t-l x.x.0.0\tUse this range instead of 169.254\n\t-v\t\tVerbose\n\n$LOGGING=none\t\tSuppress logging\n$LOGGING=syslog \tLog to syslog\n\nWith no -q, runs continuously monitoring for ARP conflicts,\nexits only on I/O errors (link down etc)"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "lpd",
                        usage: "SPOOLDIR [HELPER [ARGS]]\n\nSPOOLDIR must contain (symlinks to) device nodes or directories\nwith names matching print queue names. In the first case, jobs are\nsent directly to the device. Otherwise each job is stored in queue\ndirectory and HELPER program is called. Name of file to print\nis passed in $DATAFILE variable.\nExample:\n\ttcpsvd -E 0 515 softlimit -m 999999 lpd /var/spool ./print"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "lpq",
                        usage: "[-P queue[@host[:port]]] [-U USERNAME] [-d JOBID]... [-fs]\n\n\t-P\tlp service to connect to (else uses $PRINTER)\n\t-d\tDelete jobs\n\t-f\tForce any waiting job to be printed\n\t-s\tShort display"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "lpr",
                        usage: "-P queue[@host[:port]] -U USERNAME -J TITLE -Vmh [FILE]...\n\n\t-P\tlp service to connect to (else uses $PRINTER)\n\t-m\tSend mail on completion\n\t-h\tPrint banner page too\n\t-V\tVerbose"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "free",
      usage: "[-b/k/m/g]\n\nDisplay the amount of free and used system memory",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "fuser",
                        usage: "[OPTIONS] FILE or PORT/PROTO\n\nFind processes which use FILEs or PORTs\n\n\t-m\tFind processes which use same fs as FILEs\n\t-4,-6\tSearch only IPv4/IPv6 space\n\t-s\tDon\'t display PIDs\n\t-k\tKill found processes\n\t-SIGNAL\tSignal to send (default: KILL)"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "iostat",
                        usage: "[-c] [-d] [-t] [-z] [-k|-m] [ALL|BLOCKDEV...] [INTERVAL [COUNT]]\n\nReport CPU and I/O statistics\n\n\t-c\tShow CPU utilization\n\t-d\tShow device utilization\n\t-t\tPrint current time\n\t-z\tOmit devices with no activity\n\t-k\tUse kb/s\n\t-m\tUse Mb/s"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "kill",
                        usage: "[-l] [-SIG] PID...\n\nSend a signal (default: TERM) to given PIDs\n\n\t-l\tList all signal names and numbers"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "killall",
                        usage: "[-l] [-q] [-SIG] PROCESS_NAME...\n\nSend a signal (default: TERM) to given processes\n\n\t-l\tList all signal names and numbers\n\t-q\tDon\'t complain if no processes were killed"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "killall5",
                        usage: "[-l] [-SIG] [-o PID]...\n\nSend a signal (default: TERM) to all processes outside current session\n\n\t-l\tList all signal names and numbers\n\t-o PID\tDon\'t signal this PID"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "lsof",
      usage: "\n\nShow all open files",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "mpstat",
                        usage: "[-A] [-I SUM|CPU|ALL|SCPU] [-u] [-P num|ALL] [INTERVAL [COUNT]]\n\nPer-processor statistics\n\n\t-A\t\t\tSame as -I ALL -u -P ALL\n\t-I SUM|CPU|ALL|SCPU\tReport interrupt statistics\n\t-P num|ALL\t\tProcessor to monitor\n\t-u\t\t\tReport CPU utilization"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "nmeter",
                        usage: "[-d MSEC] FORMAT_STRING\n\nMonitor system in real time\n\n -d MSEC\tMilliseconds between updates, default:1000, none:-1\n\nFormat specifiers:\n %Nc or %[cN]\tCPU. N - bar size (default 10)\n\t\t(displays: S:system U:user N:niced D:iowait I:irq i:softirq)\n %[nINTERFACE]\tNetwork INTERFACE\n %m\t\tAllocated memory\n %[mf]\t\tFree memory\n %[mt]\t\tTotal memory\n %s\t\tAllocated swap\n %f\t\tNumber of used file descriptors\n %Ni\t\tTotal/specific IRQ rate\n %x\t\tContext switch rate\n %p\t\tForks\n %[pn]\t\t# of processes\n %b\t\tBlock io\n %Nt\t\tTime (with N decimal points)\n %r\t\tPrint <cr> instead of <lf> at EOL"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "pgrep",
                        usage: "[-flanovx] [-s SID|-P PPID|PATTERN]\n\nDisplay process(es) selected by regex PATTERN\n\n\t-l\tShow command name too\n\t-a\tShow command line too\n\t-f\tMatch against entire command line\n\t-n\tShow the newest process only\n\t-o\tShow the oldest process only\n\t-v\tNegate the match\n\t-x\tMatch whole name (not substring)\n\t-s\tMatch session ID (0 for current)\n\t-P\tMatch parent process ID"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "pkill",
                        usage: "[-l|-SIGNAL] [-fnovx] [-s SID|-P PPID|PATTERN]\n\nSend a signal to process(es) selected by regex PATTERN\n\n\t-l\tList all signals\n\t-f\tMatch against entire command line\n\t-n\tSignal the newest process only\n\t-o\tSignal the oldest process only\n\t-v\tNegate the match\n\t-x\tMatch whole name (not substring)\n\t-s\tMatch session ID (0 for current)\n\t-P\tMatch parent process ID"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "pidof",
                        usage: "[OPTIONS] [NAME]...\n\nList PIDs of all processes with names that match NAMEs\n\n\t-s\tShow only one PID\n\t-o PID\tOmit given pid\n\t\tUse %PPID to omit pid of pidof\'s parent"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "pmap",
      usage: "[-xq] PID...\n\nDisplay process memory usage\n\n\t-x\tShow details\n\t-q\tQuiet",
    };
    init
  },
  {
    let mut init = usage_data {
      aname: "powertop",
      usage: "\n\nAnalyze power consumption on Intel-based laptops",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "ps",
                        usage: "[-o COL1,COL2=HEADER] [-T]\n\nShow list of processes\n\n\t-o COL1,COL2=HEADER\tSelect columns for display\n\t-T\t\t\tShow threads"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "pstree",
                        usage: "[-p] [PID|USER]\n\nDisplay process tree, optionally start from USER or PID\n\n\t-p\tShow pids"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "pwdx",
      usage: "PID...\n\nShow current directory for PIDs",
    };
    init
  },
  {
    let mut init = usage_data {
      aname: "smemcap",
      usage: ">SMEMDATA.TAR\n\nCollect memory usage data in /proc and write it to stdout",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "sysctl",
                        usage: "-p [-enq] [FILE...] / [-enqaw] [KEY[=VALUE]]...\n\nShow/set kernel parameters\n\n\t-p\tSet values from FILEs (default /etc/sysctl.conf)\n\t-e\tDon\'t warn about unknown keys\n\t-n\tDon\'t show key names\n\t-q      Quiet\n\t-a\tShow all values\n\t-w\tSet values"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "top",
                        usage: "[-bmH] [-n COUNT] [-d SECONDS]\n\nProvide a view of process activity in real time.\nRead the status of all processes from /proc each SECONDS\nand display a screenful of them.\nKeys:\n\tN/M/P/T: show CPU usage, sort by pid/mem/cpu/time\n\tS: show memory\n\tR: reverse sort\n\tH: toggle threads, 1: toggle SMP\n\tQ,^C: exit\nOptions:\n\t-b\tBatch mode\n\t-n N\tExit after N iterations\n\t-d SEC\tDelay between updates\n\t-m\tSame as \'s\' key\n\t-H\tShow threads"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "uptime",
      usage: "\n\nDisplay the time since the last boot",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "watch",
                        usage: "[-n SEC] [-t] PROG ARGS\n\nRun PROG periodically\n\n\t-n SEC\tLoop period (default 2)\n\t-t\tDon\'t print header"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "chpst",
                        usage: "[-vP012] [-u USER[:GRP]] [-U USER[:GRP]] [-e DIR]\n\t[-/ DIR] [-n NICE] [-m BYTES] [-d BYTES] [-o N]\n\t[-p N] [-f BYTES] [-c BYTES] PROG ARGS\n\nChange the process state, run PROG\n\n\t-u USER[:GRP]\tSet uid and gid\n\t-U USER[:GRP]\tSet $UID and $GID in environment\n\t-e DIR\t\tSet environment variables as specified by files\n\t\t\tin DIR: file=1st_line_of_file\n\t-/ DIR\t\tChroot to DIR\n\t-n NICE\t\tAdd NICE to nice value\n\t-m BYTES\tSame as -d BYTES -s BYTES -l BYTES\n\t-d BYTES\tLimit data segment\n\t-o N\t\tLimit number of open files per process\n\t-p N\t\tLimit number of processes per uid\n\t-f BYTES\tLimit output file sizes\n\t-c BYTES\tLimit core file size\n\t-v\t\tVerbose\n\t-P\t\tCreate new process group\n\t-0\t\tClose stdin\n\t-1\t\tClose stdout\n\t-2\t\tClose stderr"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "envdir",
                        usage: "DIR PROG ARGS\n\nSet various environment variables as specified by files\nin the directory DIR, run PROG"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "envuidgid",
      usage: "USER PROG ARGS\n\nSet $UID to USER\'s uid and $GID to USER\'s gid, run PROG",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "setuidgid",
                        usage: "USER PROG ARGS\n\nSet uid and gid to USER\'s uid and gid, drop supplementary group ids,\nrun PROG"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "softlimit",
                        usage: "[-a BYTES] [-m BYTES] [-d BYTES] [-s BYTES] [-l BYTES]\n\t[-f BYTES] [-c BYTES] [-r BYTES] [-o N] [-p N] [-t N]\n\tPROG ARGS\n\nSet soft resource limits, then run PROG\n\n\t-a BYTES\tLimit total size of all segments\n\t-m BYTES\tSame as -d BYTES -s BYTES -l BYTES -a BYTES\n\t-d BYTES\tLimit data segment\n\t-s BYTES\tLimit stack segment\n\t-l BYTES\tLimit locked memory size\n\t-o N\t\tLimit number of open files per process\n\t-p N\t\tLimit number of processes per uid\nOptions controlling file sizes:\n\t-f BYTES\tLimit output file sizes\n\t-c BYTES\tLimit core file size\nEfficiency opts:\n\t-r BYTES\tLimit resident set size\n\t-t N\t\tLimit CPU time, process receives\n\t\t\ta SIGXCPU after N seconds"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "runsv",
      usage: "DIR\n\nStart and monitor a service and optionally an appendant log service",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "runsvdir",
                        usage: "[-P] [-s SCRIPT] DIR\n\nStart a runsv process for each subdirectory. If it exits, restart it.\n\n\t-P\t\tPut each runsv in a new session\n\t-s SCRIPT\tRun SCRIPT <signo> after signal is processed"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "sv",
                        usage: "[-v] [-w SEC] CMD SERVICE_DIR...\n\nControl services monitored by runsv supervisor.\nCommands (only first character is enough):\n\nstatus: query service status\nup: if service isn\'t running, start it. If service stops, restart it\nonce: like \'up\', but if service stops, don\'t restart it\ndown: send TERM and CONT signals. If ./run exits, start ./finish\n\tif it exists. After it stops, don\'t restart service\nexit: send TERM and CONT signals to service and log service. If they exit,\n\trunsv exits too\npause, cont, hup, alarm, interrupt, quit, 1, 2, term, kill: send\nSTOP, CONT, HUP, ALRM, INT, QUIT, USR1, USR2, TERM, KILL signal to service"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "svc",
                        usage: "[-udopchaitkx] SERVICE_DIR...\n\nControl services monitored by runsv supervisor\n\n\t-u\tIf service is not running, start it; restart if it stops\n\t-d\tIf service is running, send TERM+CONT signals; do not restart it\n\t-o\tOnce: if service is not running, start it; do not restart it\n\t-pchaitk Send STOP, CONT, HUP, ALRM, INT, TERM, KILL signal to service\n\t-x\tExit: runsv will exit as soon as the service is down"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "svok",
                        usage: "SERVICE_DIR\n\nCheck whether runsv supervisor is running.\nExit code is 0 if it does, 100 if it does not,\n111 (with error message) if SERVICE_DIR does not exist."
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "svlogd",
                        usage: "[-tttv] [-r C] [-R CHARS] [-l MATCHLEN] [-b BUFLEN] DIR...\n\nRead log data from stdin and write to rotated log files in DIRs\n\n-r C\t\tReplace non-printable characters with C\n-R CHARS\tAlso replace CHARS with C (default _)\n-t\t\tTimestamp with @tai64n\n-tt\t\tTimestamp with yyyy-mm-dd_hh:mm:ss.sssss\n-ttt\t\tTimestamp with yyyy-mm-ddThh:mm:ss.sssss\n-v\t\tVerbose\n\nDIR/config file modifies behavior:\nsSIZE - when to rotate logs (default 1000000, 0 disables)\nnNUM - number of files to retain\n!PROG - process rotated log with PROG\n+,-PATTERN - (de)select line for logging\nE,ePATTERN - (de)select line for stderr"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "ash",
                        usage: "[-/+OPTIONS] [-/+o OPT]... [-c \'SCRIPT\' [ARG0 [ARGS]] / FILE [ARGS] / -s [ARGS]]\n\nUnix shell interpreter"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "sh",
                        usage: "[-/+OPTIONS] [-/+o OPT]... [-c \'SCRIPT\' [ARG0 [ARGS]] / FILE [ARGS] / -s [ARGS]]\n\nUnix shell interpreter"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "cttyhack",
                        usage: "[PROG ARGS]\n\nGive PROG a controlling tty if possible.\nExample for /etc/inittab (for busybox init):\n\t::respawn:/bin/cttyhack /bin/sh\nGiving controlling tty to shell running with PID 1:\n\t$ exec cttyhack sh\nStarting interactive shell from boot shell script:\n\tsetsid cttyhack sh"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "hush",
      usage:
        "[-enxl] [-c \'SCRIPT\' [ARG0 [ARGS]] / FILE [ARGS] / -s [ARGS]]\n\nUnix shell interpreter",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "klogd",
                        usage: "[-c N] [-n]\n\nLog kernel messages to syslog\n\n\t-c N\tPrint to console messages more urgent than prio N (1-8)\n\t-n\tRun in foreground"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "logger",
                        usage: "[OPTIONS] [MESSAGE]\n\nWrite MESSAGE (or stdin) to syslog\n\n\t-s\tLog to stderr as well as the system log\n\t-t TAG\tLog using the specified tag (defaults to user name)\n\t-p PRIO\tPriority (numeric or facility.level pair)"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "logread",
                        usage: "[-fF]\n\nShow messages in syslogd\'s circular buffer\n\n\t-f\tOutput data as log grows\n\t-F\tSame as -f, but dump buffer first"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "syslogd",
                        usage: "[OPTIONS]\n\nSystem logging utility\n\n\t-n\t\tRun in foreground\n\t-R HOST[:PORT]\tLog to HOST:PORT (default PORT:514)\n\t-L\t\tLog locally and via network (default is network only if -R)\n\t-C[size_kb]\tLog to shared mem buffer (use logread to read it)\n\t-K\t\tLog to kernel printk buffer (use dmesg to read it)\n\t-O FILE\t\tLog to FILE (default: /var/log/messages, stdout if -)\n\t-s SIZE\t\tMax size (KB) before rotation (default 200KB, 0=off)\n\t-b N\t\tN rotated logs to keep (default 1, max 99, 0=purge)\n\t-l N\t\tLog only messages more urgent than prio N (1-8)\n\t-S\t\tSmaller output\n\t-t\t\tStrip client-generated timestamps\n\t-D\t\tDrop duplicates\n\t-f FILE\t\tUse FILE as config (default:/etc/syslog.conf)"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "acpid",
                        usage: "[-df] [-c CONFDIR] [-l LOGFILE] [-a ACTIONFILE] [-M MAPFILE] [-e PROC_EVENT_FILE] [-p PIDFILE]\n\nListen to ACPI events and spawn specific helpers on event arrival\n\n\t-d\tLog to stderr, not log file (implies -f)\n\t-f\tRun in foreground\n\t-c DIR\tConfig directory [/etc/acpi]\n\t-e FILE\t/proc event file [/proc/acpi/event]\n\t-l FILE\tLog file [/var/log/acpid.log]\n\t-p FILE\tPid file [/var/run/acpid.pid]\n\t-a FILE\tAction file [/etc/acpid.conf]\n\t-M FILE Map file [/etc/acpi.map]\n\nAccept and ignore compatibility options -g -m -s -S -v"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "blkdiscard",
                        usage: "[-o OFS] [-l LEN] [-s] DEVICE\n\nDiscard sectors on DEVICE\n\n\t-o OFS\tByte offset into device\n\t-l LEN\tNumber of bytes to discard\n\t-s\tPerform a secure discard"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "blkid",
      usage: "[BLOCKDEV]...\n\nPrint UUIDs of all filesystems",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "blockdev",
                        usage: "OPTION BLOCKDEV\n\n\t--setro\t\tSet ro\n\t--setrw\t\tSet rw\n\t--getro\t\tGet ro\n\t--getss\t\tGet sector size\n\t--getbsz\tGet block size\n\t--setbsz BYTES\tSet block size\n\t--getsz\t\tGet device size in 512-byte sectors\n\t--getsize64\tGet device size in bytes\n\t--flushbufs\tFlush buffers\n\t--rereadpt\tReread partition table"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "cal",
                        usage: "[-jy] [[MONTH] YEAR]\n\nDisplay a calendar\n\n\t-j\tUse julian dates\n\t-y\tDisplay the entire year"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "chrt",
                        usage: "-m | -p [PRIO] PID | [-rfobi] PRIO PROG [ARGS]\n\nChange scheduling priority and class for a process\n\n\t-m\tShow min/max priorities\n\t-p\tOperate on PID\n\t-r\tSet SCHED_RR class\n\t-f\tSet SCHED_FIFO class\n\t-o\tSet SCHED_OTHER class\n\t-b\tSet SCHED_BATCH class\n\t-i\tSet SCHED_IDLE class"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "dmesg",
                        usage: "[-c] [-n LEVEL] [-s SIZE]\n\nPrint or control the kernel ring buffer\n\n\t-c\t\tClear ring buffer after printing\n\t-n LEVEL\tSet console logging level\n\t-s SIZE\t\tBuffer size\n\t-r\t\tPrint raw message buffer"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "eject",
                        usage: "[-t] [-T] [DEVICE]\n\nEject DEVICE or default /dev/cdrom\n\n\t-s\tSCSI device\n\t-t\tClose tray\n\t-T\tOpen/close tray (toggle)"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "fallocate",
                        usage: "[-o OFS] -l LEN FILE\n\nPreallocate space for FILE\n\n\t-o OFS\tOffset of range\n\t-l LEN\tLength of range"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "fatattr",
                        usage: "[-+rhsvda] FILE...\n\nChange file attributes on FAT filesystem\n\n\t-\tClear attributes\n\t+\tSet attributes\n\tr\tRead only\n\th\tHidden\n\ts\tSystem\n\tv\tVolume label\n\td\tDirectory\n\ta\tArchive"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "fbset",
      usage: "[OPTIONS] [MODE]\n\nShow and modify frame buffer settings",
    };
    init
  },
  {
    let mut init = usage_data {
      aname: "fdformat",
      usage: "[-n] DEVICE\n\nFormat floppy disk\n\n\t-n\tDon\'t verify after format",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "fdisk",
                        usage: "[-ul] [-C CYLINDERS] [-H HEADS] [-S SECTORS] [-b SSZ] DISK\n\nChange partition table\n\n\t-u\t\tStart and End are in sectors (instead of cylinders)\n\t-l\t\tShow partition table for each DISK, then exit\n\t-b 2048\t\t(for certain MO disks) use 2048-byte sectors\n\t-C CYLINDERS\tSet number of cylinders/heads/sectors\n\t-H HEADS\tTypically 255\n\t-S SECTORS\tTypically 63"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "findfs",
      usage: "LABEL=label or UUID=uuid\n\nFind a filesystem device based on a label or UUID",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "flock",
                        usage: "[-sxun] FD|{FILE [-c] PROG ARGS}\n\n[Un]lock file descriptor, or lock FILE, run PROG\n\n\t-s\tShared lock\n\t-x\tExclusive lock (default)\n\t-u\tUnlock FD\n\t-n\tFail rather than wait"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "fdflush",
      usage: "DEVICE\n\nForce floppy disk drive to detect disk change",
    };
    init
  },
  {
    let mut init = usage_data {
      aname: "freeramdisk",
      usage: "DEVICE\n\nFree all memory used by the specified ramdisk",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "fsck.minix",
                        usage: "[-larvsmf] BLOCKDEV\n\nCheck MINIX filesystem\n\n\t-l\tList all filenames\n\t-r\tPerform interactive repairs\n\t-a\tPerform automatic repairs\n\t-v\tVerbose\n\t-s\tOutput superblock information\n\t-m\tShow \"mode not cleared\" warnings\n\t-f\tForce file system check"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "fsfreeze",
      usage: "--[un]freeze MOUNTPOINT\n\nFlush and halt writes to MOUNTPOINT",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "fstrim",
                        usage: "[OPTIONS] MOUNTPOINT\n\n\t-o,--offset OFFSET\tOffset in bytes to discard from\n\t-l,--length LEN\t\tBytes to discard\n\t-m,--minimum MIN\tMinimum extent length\n\t-v,--verbose\t\tPrint number of discarded bytes"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "getopt",
                        usage: "[OPTIONS] [--] OPTSTRING PARAMS\n\n\t-a\t\tAllow long options starting with single -\n\t-l LOPT[,...]\tLong options to recognize\n\t-n PROGNAME\tThe name under which errors are reported\n\t-o OPTSTRING\tShort options to recognize\n\t-q\t\tNo error messages on unrecognized options\n\t-Q\t\tNo normal output\n\t-s SHELL\tSet shell quoting conventions\n\t-T\t\tVersion test (exits with 4)\n\t-u\t\tDon\'t quote output\n\nExample:\n\nO=`getopt -l bb: -- ab:c:: \"$@\"` || exit 1\neval set -- \"$O\"\nwhile true; do\n\tcase \"$1\" in\n\t-a)\techo A; shift;;\n\t-b|--bb) echo \"B:\'$2\'\"; shift 2;;\n\t-c)\tcase \"$2\" in\n\t\t\"\")\techo C; shift 2;;\n\t\t*)\techo \"C:\'$2\'\"; shift 2;;\n\t\tesac;;\n\t--)\tshift; break;;\n\t*)\techo Error; exit 1;;\n\tesac\ndone"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "hexdump",
                        usage: "[-bcCdefnosvxR] [FILE]...\n\nDisplay FILEs (or stdin) in a user specified format\n\n\t-b\t\t1-byte octal display\n\t-c\t\t1-byte character display\n\t-d\t\t2-byte decimal display\n\t-o\t\t2-byte octal display\n\t-x\t\t2-byte hex display\n\t-C\t\thex+ASCII 16 bytes per line\n\t-v\t\tShow all (no dup folding)\n\t-e FORMAT_STR\tExample: \'16/1 \"%02x|\"\"\\n\"\'\n\t-f FORMAT_FILE\n\t-n LENGTH\tShow only first LENGTH bytes\n\t-s OFFSET\tSkip OFFSET bytes\n\t-R\t\tReverse of \'hexdump -Cv\'"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "hd",
      usage: "FILE...\n\nhd is an alias for hexdump -C",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "xxd",
                        usage: "[OPTIONS] [FILE]\n\nHex dump FILE (or stdin)\n\n\t-g N\t\tBytes per group\n\t-c N\t\tBytes per line\n\t-p\t\tShow only hex bytes, assumes -c30\n\t-l LENGTH\tShow only first LENGTH bytes\n\t-s OFFSET\tSkip OFFSET bytes"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "hwclock",
                        usage: "[-r|--show] [-s|--hctosys] [-w|--systohc] [--systz] [--localtime] [-u|--utc] [-f|--rtc FILE]\n\nQuery and set hardware clock (RTC)\n\n\t-r\tShow hardware clock time\n\t-s\tSet system time from hardware clock\n\t-w\tSet hardware clock from system time\n\t--systz\tSet in-kernel timezone, correct system time\n\t\tif hardware clock is in local time\n\t-u\tAssume hardware clock is kept in UTC\n\t--localtime\tAssume hardware clock is kept in local time\n\t-f FILE\tUse specified device (e.g. /dev/rtc2)"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "ionice",
                        usage: "[-c 1-3] [-n 0-7] [-p PID] [PROG]\n\nChange I/O priority and class\n\n\t-c\tClass. 1:realtime 2:best-effort 3:idle\n\t-n\tPriority"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "ipcrm",
                        usage: "[-MQS key] [-mqs id]\n\nUpper-case options MQS remove an object by shmkey value.\nLower-case options remove an object by shmid value.\n\n\t-mM\tRemove memory segment after last detach\n\t-qQ\tRemove message queue\n\t-sS\tRemove semaphore"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "ipcs",
                        usage: "[[-smq] -i SHMID] | [[-asmq] [-tcplu]]\n\n\t-i ID\tShow specific resource\nResource specification:\n\t-m\tShared memory segments\n\t-q\tMessage queues\n\t-s\tSemaphore arrays\n\t-a\tAll (default)\nOutput format:\n\t-t\tTime\n\t-c\tCreator\n\t-p\tPid\n\t-l\tLimits\n\t-u\tSummary"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "last",
                        usage: "[-HW] [-f FILE]\n\nShow listing of the last users that logged into the system\n\n\t-W\tDisplay with no host column truncation\n\t-f FILE Read from FILE instead of /var/log/wtmp"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "losetup",
                        usage: "[-rP] [-o OFS] {-f|LOOPDEV} FILE: associate loop devices\n\tlosetup -c LOOPDEV: reread file size\n\tlosetup -d LOOPDEV: disassociate\n\tlosetup -a: show status\n\tlosetup -f: show next free loop device\n\n\t-o OFS\tStart OFS bytes into FILE\n\t-P\tScan for partitions\n\t-r\tRead-only\n\t-f\tShow/use next free loop device"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "lspci",
      usage: "[-mk]\n\nList all PCI devices\n\n\t-m\tParsable output\n\t-k\tShow driver",
    };
    init
  },
  {
    let mut init = usage_data {
      aname: "lsusb",
      usage: "\x08",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "mdev",
                        usage: "[-s] | [-df]\n\nmdev -s is to be run during boot to scan /sys and populate /dev.\nmdev -d[f]: daemon, listen on netlink.\n\t-f: stay in foreground.\n\nBare mdev is a kernel hotplug helper. To activate it:\n\techo /sbin/mdev >/proc/sys/kernel/hotplug\n\nIt uses /etc/mdev.conf with lines\n\t[-][ENV=regex;]...DEVNAME UID:GID PERM [>|=PATH]|[!] [@|$|*PROG]\nwhere DEVNAME is device name regex, @major,minor[-minor2], or\nenvironment variable regex. A common use of the latter is\nto load modules for hotplugged devices:\n\t$MODALIAS=.* 0:0 660 @modprobe \"$MODALIAS\"\n\nIf /dev/mdev.seq file exists, mdev will wait for its value\nto match $SEQNUM variable. This prevents plug/unplug races.\nTo activate this feature, create empty /dev/mdev.seq at boot.\n\nIf /dev/mdev.log file exists, debug log will be appended to it."
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "mesg",
                        usage: "[y|n]\n\nControl write access to your terminal\n\ty\tAllow write access to your terminal\n\tn\tDisallow write access to your terminal"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "mke2fs",
                        usage: "[-Fn] [-b BLK_SIZE] [-i INODE_RATIO] [-I INODE_SIZE] [-m RESERVED_PERCENT] [-L LABEL] BLOCKDEV [KBYTES]\n\n\t-b BLK_SIZE\tBlock size, bytes\n\t-F\t\tForce\n\t-i RATIO\tMax number of files is filesystem_size / RATIO\n\t-I BYTES\tInode size (min 128)\n\t-L LBL\t\tVolume label\n\t-m PERCENT\tPercent of blocks to reserve for admin\n\t-n\t\tDry run"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "mkfs.ext2",
                        usage: "[-Fn] [-b BLK_SIZE] [-i INODE_RATIO] [-I INODE_SIZE] [-m RESERVED_PERCENT] [-L LABEL] BLOCKDEV [KBYTES]\n\n\t-b BLK_SIZE\tBlock size, bytes\n\t-F\t\tForce\n\t-i RATIO\tMax number of files is filesystem_size / RATIO\n\t-I BYTES\tInode size (min 128)\n\t-L LBL\t\tVolume label\n\t-m PERCENT\tPercent of blocks to reserve for admin\n\t-n\t\tDry run"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "mkfs.minix",
                        usage: "[-c | -l FILE] [-nXX] [-iXX] BLOCKDEV [KBYTES]\n\nMake a MINIX filesystem\n\n\t-c\t\tCheck device for bad blocks\n\t-n [14|30]\tMaximum length of filenames\n\t-i INODES\tNumber of inodes for the filesystem\n\t-l FILE\t\tRead bad blocks list from FILE\n\t-v\t\tMake version 2 filesystem"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "mkdosfs",
                        usage: "[-v] [-n LABEL] BLOCKDEV [KBYTES]\n\nMake a FAT32 filesystem\n\n\t-v\tVerbose\n\t-n LBL\tVolume label"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "mkfs.vfat",
                        usage: "[-v] [-n LABEL] BLOCKDEV [KBYTES]\n\nMake a FAT32 filesystem\n\n\t-v\tVerbose\n\t-n LBL\tVolume label"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "mkswap",
                        usage: "[-L LBL] BLOCKDEV [KBYTES]\n\nPrepare BLOCKDEV to be used as swap partition\n\n\t-L LBL\tLabel"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "more",
      usage: "[FILE]...\n\nView FILE (or stdin) one screenful at a time",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "mount",
                        usage: "[OPTIONS] [-o OPT] DEVICE NODE\n\nMount a filesystem. Filesystem autodetection requires /proc.\n\n\t-a\t\tMount all filesystems in fstab\n\t-f\t\tDry run\n\t-v\t\tVerbose\n\t-r\t\tRead-only mount\n\t-t FSTYPE[,...]\tFilesystem type(s)\n\t-T FILE\t\tRead FILE instead of /etc/fstab\n\t-O OPT\t\tMount only filesystems with option OPT (-a only)\n-o OPT:\n\tloop\t\tIgnored (loop devices are autodetected)\n\t[a]sync\t\tWrites are [a]synchronous\n\t[no]atime\tDisable/enable updates to inode access times\n\t[no]diratime\tDisable/enable atime updates to directories\n\t[no]relatime\tDisable/enable atime updates relative to modification time\n\t[no]dev\t\t(Dis)allow use of special device files\n\t[no]exec\t(Dis)allow use of executable files\n\t[no]suid\t(Dis)allow set-user-id-root programs\n\t[r]shared\tConvert [recursively] to a shared subtree\n\t[r]slave\tConvert [recursively] to a slave subtree\n\t[r]private\tConvert [recursively] to a private subtree\n\t[un]bindable\tMake mount point [un]able to be bind mounted\n\t[r]bind\t\tBind a file or directory [recursively] to another location\n\tmove\t\tRelocate an existing mount point\n\tremount\t\tRemount a mounted filesystem, changing flags\n\tro\t\tSame as -r\n\nThere are filesystem-specific -o flags."
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "mountpoint",
                        usage: "[-q] <[-dn] DIR | -x DEVICE>\n\nCheck if the directory is a mountpoint\n\n\t-q\tQuiet\n\t-d\tPrint major/minor device number of the filesystem\n\t-n\tPrint device name of the filesystem\n\t-x\tPrint major/minor device number of the blockdevice"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "nologin",
      usage: "\n\nPolitely refuse a login",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "nsenter",
                        usage: "[OPTIONS] [PROG [ARGS]]\n\n\t-t PID\t\tTarget process to get namespaces from\n\t-m[FILE]\tEnter mount namespace\n\t-u[FILE]\tEnter UTS namespace (hostname etc)\n\t-i[FILE]\tEnter System V IPC namespace\n\t-n[FILE]\tEnter network namespace\n\t-p[FILE]\tEnter pid namespace\n\t-U[FILE]\tEnter user namespace\n\t-S UID\t\tSet uid in entered namespace\n\t-G GID\t\tSet gid in entered namespace\n\t--preserve-credentials\tDon\'t touch uids or gids\n\t-r[DIR]\t\tSet root directory\n\t-w[DIR]\t\tSet working directory\n\t-F\t\tDon\'t fork before exec\'ing PROG"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "pivot_root",
                        usage: "NEW_ROOT PUT_OLD\n\nMove the current root file system to PUT_OLD and make NEW_ROOT\nthe new root file system"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "rdate",
                        usage: "[-s/-p] HOST\n\nSet and print time from HOST using RFC 868\n\n\t-s\tOnly set system time\n\t-p\tOnly print time"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "rdev",
      usage: "\n\nPrint the device node associated with the filesystem mounted at \'/\'",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "readprofile",
                        usage: "[OPTIONS]\n\n\t-m mapfile\t(Default: /boot/System.map)\n\t-p profile\t(Default: /proc/profile)\n\t-M NUM\t\tSet the profiling multiplier to NUM\n\t-i\t\tPrint only info about the sampling step\n\t-v\t\tVerbose\n\t-a\t\tPrint all symbols, even if count is 0\n\t-b\t\tPrint individual histogram-bin counts\n\t-s\t\tPrint individual counters within functions\n\t-r\t\tReset all the counters (root only)\n\t-n\t\tDisable byte order auto-detection"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "renice",
                        usage: "[-n] PRIORITY [[-p | -g | -u] ID...]...\n\nChange scheduling priority of a running process\n\n\t-n\tAdd PRIORITY to current nice value\n\t\tWithout -n, nice value is set to PRIORITY\n\t-p\tProcess ids (default)\n\t-g\tProcess group ids\n\t-u\tProcess user names"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "rev",
      usage: "[FILE]...\n\nReverse lines of FILE",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "rtcwake",
                        usage: "[-a | -l | -u] [-d DEV] [-m MODE] [-s SEC | -t TIME]\n\nEnter a system sleep state until specified wakeup time\n\n\t-a,--auto\tRead clock mode from adjtime\n\t-l,--local\tClock is set to local time\n\t-u,--utc\tClock is set to UTC time\n\t-d,--device DEV\tSpecify the RTC device\n\t-m,--mode MODE\tSet sleep state (default: standby)\n\t-s,--seconds SEC Set timeout in SEC seconds from now\n\t-t,--time TIME\tSet timeout to TIME seconds from epoch"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "script",
                        usage: "[-afq] [-t[FILE]] [-c PROG] [OUTFILE]\n\nDefault OUTFILE is \'typescript\'\n\n\t-a\tAppend output\n\t-c PROG\tRun PROG, not shell\n\t-q\tQuiet\n\t-t[FILE] Send timing to stderr or FILE"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "scriptreplay",
      usage: "TIMINGFILE [TYPESCRIPT [DIVISOR]]\n\nPlay back typescripts, using timing information",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "setarch",
                        usage: "PERSONALITY [-R] PROG ARGS\n\nPERSONALITY may be:\n\tlinux32\tSet 32bit uname emulation\n\tlinux64\tSet 64bit uname emulation\n\n\t-R\tDisable address space randomization"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "linux32",
      usage: "\x08",
    };
    init
  },
  {
    let mut init = usage_data {
      aname: "linux64",
      usage: "\x08",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "setpriv",
                        usage: "[OPTIONS] PROG [ARGS]\n\nRun PROG with different privilege settings\n\n-d,--dump\t\tShow current capabilities\n--nnp,--no-new-privs\tIgnore setuid/setgid bits and file capabilities\n--inh-caps CAP,CAP\tSet inheritable capabilities\n--ambient-caps CAP,CAP\tSet ambient capabilities"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "setsid",
                        usage: "[-c] PROG ARGS\n\nRun PROG in a new session. PROG will have no controlling terminal\nand will not be affected by keyboard signals (^C etc).\n\n\t-c\tSet controlling terminal to stdin"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "swapon",
                        usage: "[-a] [-e] [-d[POL]] [-p PRI] [DEVICE]\n\nStart swapping on DEVICE\n\n\t-a\tStart swapping on all swap devices\n\t-d[POL]\tDiscard blocks at swapon (POL=once),\n\t\tas freed (POL=pages), or both (POL omitted)\n\t-e\tSilently skip devices that do not exist\n\t-p PRI\tSet swap device priority"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "swapoff",
      usage: "[-a] [DEVICE]\n\nStop swapping on DEVICE\n\n\t-a\tStop swapping on all swap devices",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "switch_root",
                        usage: "[-c CONSOLE_DEV] NEW_ROOT NEW_INIT [ARGS]\n\nFree initramfs and switch to another root fs:\nchroot to NEW_ROOT, delete all in /, move NEW_ROOT to /,\nexecute NEW_INIT. PID must be 1. NEW_ROOT must be a mountpoint.\n\n\t-c DEV\tReopen stdio to DEV after switch"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "run-init",
                        usage: "[-d CAP,CAP...] [-n] [-c CONSOLE_DEV] NEW_ROOT NEW_INIT [ARGS]\n\nFree initramfs and switch to another root fs:\nchroot to NEW_ROOT, delete all in /, move NEW_ROOT to /,\nexecute NEW_INIT. PID must be 1. NEW_ROOT must be a mountpoint.\n\n\t-c DEV\tReopen stdio to DEV after switch\n\t-d CAPS\tDrop capabilities\n\t-n\tDry run"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "taskset",
                        usage: "[-p] [HEXMASK] PID | PROG ARGS\n\nSet or get CPU affinity\n\n\t-p\tOperate on an existing PID"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "uevent",
                        usage: "[PROG [ARGS]]\n\nuevent runs PROG for every netlink notification.\nPROG\'s environment contains data passed from the kernel.\nTypical usage (daemon for dynamic device node creation):\n\t# uevent mdev & mdev -s"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "umount",
                        usage: "[OPTIONS] FILESYSTEM|DIRECTORY\n\nUnmount file systems\n\n\t-a\tUnmount all file systems\n\t-r\tTry to remount devices as read-only if mount is busy\n\t-l\tLazy umount (detach filesystem)\n\t-f\tForce umount (i.e., unreachable NFS server)\n\t-d\tFree loop device if it has been used\n\t-t FSTYPE[,...]\tUnmount only these filesystem type(s)"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "unshare",
                        usage: "[OPTIONS] [PROG [ARGS]]\n\n\t-m,--mount[=FILE]\tUnshare mount namespace\n\t-u,--uts[=FILE]\t\tUnshare UTS namespace (hostname etc.)\n\t-i,--ipc[=FILE]\t\tUnshare System V IPC namespace\n\t-n,--net[=FILE]\t\tUnshare network namespace\n\t-p,--pid[=FILE]\t\tUnshare PID namespace\n\t-U,--user[=FILE]\tUnshare user namespace\n\t-f,--fork\t\tFork before execing PROG\n\t-r,--map-root-user\tMap current user to root (implies -U)\n\t--mount-proc[=DIR]\tMount /proc filesystem first (implies -m)\n\t--propagation slave|shared|private|unchanged\n\t\t\t\tModify mount propagation in mount namespace\n\t--setgroups allow|deny\tControl the setgroups syscall in user namespaces"
                                ,};
    init
  },
  {
    let mut init = usage_data {
      aname: "wall",
      usage: "[FILE]\n\nWrite content of FILE or stdin to all logged-in users",
    };
    init
  },
  {
    let mut init =
             usage_data{aname: "udhcpc6",
                        usage: "[-fbnqvodR] [-i IFACE] [-r IPv6] [-s PROG] [-p PIDFILE]\n\t[-x OPT:VAL]... [-O OPT]...\n\n\t-i IFACE\tInterface to use (default eth0)\n\t-p FILE\t\tCreate pidfile\n\t-s PROG\t\tRun PROG at DHCP events (default /usr/share/udhcpc/default.script)\n\t-B\t\tRequest broadcast replies\n\t-t N\t\tSend up to N discover packets\n\t-T N\t\tPause between packets (default 3 seconds)\n\t-A N\t\tWait N seconds (default 20) after failure\n\t-f\t\tRun in foreground\n\t-b\t\tBackground if lease is not obtained\n\t-n\t\tExit if lease is not obtained\n\t-q\t\tExit after obtaining lease\n\t-R\t\tRelease IP on exit\n\t-S\t\tLog to syslog too\n\t-O OPT\t\tRequest option OPT from server (cumulative)\n\t-o\t\tDon\'t request any options (unless -O is given)\n\t-r IPv6\t\tRequest this address (\'no\' to not request any IP)\n\t-d\t\tRequest prefix\n\t-l\t\tSend \'information request\' instead of \'solicit\'\n\t\t\t(used for servers which do not assign IPv6 addresses)\n\t-x OPT:VAL\tInclude option OPT in sent packets (cumulative)\n\t\t\tExamples of string, numeric, and hex byte opts:\n\t\t\t-x hostname:bbox - option 12\n\t\t\t-x lease:3600 - option 51 (lease time)\n\t\t\t-x 0x3d:0100BEEFC0FFEE - option 61 (client id)\n\t\t\t-x 14:\'\"dumpfile\"\' - option 14 (shell-quoted)\n\t-v\t\tVerbose\nSignals:\n\tUSR1\tRenew lease\n\tUSR2\tRelease lease"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "udhcpc",
                        usage: "[-fbqvRB] [-a[MSEC]] [-t N] [-T SEC] [-A SEC/-n]\n\t[-i IFACE] [-s PROG] [-p PIDFILE]\n\t[-oC] [-r IP] [-V VENDOR] [-F NAME] [-x OPT:VAL]... [-O OPT]...\n\n\t-i IFACE\tInterface to use (default eth0)\n\t-s PROG\t\tRun PROG at DHCP events (default /usr/share/udhcpc/default.script)\n\t-p FILE\t\tCreate pidfile\n\t-B\t\tRequest broadcast replies\n\t-t N\t\tSend up to N discover packets (default 3)\n\t-T SEC\t\tPause between packets (default 3)\n\t-A SEC\t\tWait if lease is not obtained (default 20)\n\t-b\t\tBackground if lease is not obtained\n\t-n\t\tExit if lease is not obtained\n\t-q\t\tExit after obtaining lease\n\t-R\t\tRelease IP on exit\n\t-f\t\tRun in foreground\n\t-S\t\tLog to syslog too\n\t-a[MSEC]\tValidate offered address with ARP ping\n\t-r IP\t\tRequest this IP address\n\t-o\t\tDon\'t request any options (unless -O is given)\n\t-O OPT\t\tRequest option OPT from server (cumulative)\n\t-x OPT:VAL\tInclude option OPT in sent packets (cumulative)\n\t\t\tExamples of string, numeric, and hex byte opts:\n\t\t\t-x hostname:bbox - option 12\n\t\t\t-x lease:3600 - option 51 (lease time)\n\t\t\t-x 0x3d:0100BEEFC0FFEE - option 61 (client id)\n\t\t\t-x 14:\'\"dumpfile\"\' - option 14 (shell-quoted)\n\t-F NAME\t\tAsk server to update DNS mapping for NAME\n\t-V VENDOR\tVendor identifier (default \'udhcp VERSION\')\n\t-C\t\tDon\'t send MAC as client identifier\n\t-v\t\tVerbose\nSignals:\n\tUSR1\tRenew lease\n\tUSR2\tRelease lease"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "udhcpd",
                        usage: "[-fS] [-I ADDR] [CONFFILE]\n\nDHCP server\n\n\t-f\tRun in foreground\n\t-S\tLog to syslog too\n\t-I ADDR\tLocal address\n\t-a MSEC\tTimeout for ARP ping (default 2000)\nSignals:\n\tUSR1\tUpdate lease file"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "dhcprelay",
                        usage: "CLIENT_IFACE[,CLIENT_IFACE2]... SERVER_IFACE [SERVER_IP]\n\nRelay DHCP requests between clients and server"
                                ,};
    init
  },
  {
    let mut init =
             usage_data{aname: "dumpleases",
                        usage: "[-r|-a] [-d] [-f LEASEFILE]\n\nDisplay DHCP leases granted by udhcpd\n\n\t-f,--file FILE\tLease file\n\t-r,--remaining\tShow remaining time\n\t-a,--absolute\tShow expiration time\n\t-d,--decimal\tShow time in seconds"
                                ,};
    init
  },
];
