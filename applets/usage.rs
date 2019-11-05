pub struct usage_data {
  pub aname: &'static str,
  pub usage: &'static str,
}

// This is in exactly the same order as applets in applet_tables.rs.
static usage_array: [usage_data; 396] = [
  usage_data {
    aname: "gunzip",
    usage: "\
[-cfkt] [FILE]...

Decompress FILEs (or stdin)

\t-c\tWrite to stdout
\t-f\tForce
\t-k\tKeep input files
\t-t\tTest file integrity"
  },
  usage_data {
    aname: "zcat",
    usage: "\
[FILE]...

Decompress to stdout",
  },
  usage_data {
    aname: "bunzip2",
    usage: "\
[-cfk] [FILE]...

Decompress FILEs (or stdin)

\t-c\tWrite to stdout
\t-f\tForce
\t-k\tKeep input files"
  },
  usage_data {
    aname: "bzcat",
    usage: "\
[FILE]...

Decompress to stdout",
  },
  usage_data {
    aname: "unlzma",
    usage: "\
[-cfk] [FILE]...

Decompress FILE (or stdin)

\t-c\tWrite to stdout
\t-f\tForce
\t-k\tKeep input files"
  },
  usage_data {
    aname: "lzcat",
    usage: "\
[FILE]...

Decompress to stdout",
  },
  usage_data {
    aname: "lzma",
    usage: "\
-d [-cfk] [FILE]...

Decompress FILE (or stdin)

\t-d\tDecompress
\t-c\tWrite to stdout
\t-f\tForce
\t-k\tKeep input files"
  },
  usage_data {
    aname: "unxz",
    usage: "\
[-cfk] [FILE]...

Decompress FILE (or stdin)

\t-c\tWrite to stdout
\t-f\tForce
\t-k\tKeep input files"
  },
  usage_data {
    aname: "xzcat",
    usage: "\
[FILE]...

Decompress to stdout",
  },
  usage_data {
    aname: "xz",
    usage: "\
-d [-cfk] [FILE]...

Decompress FILE (or stdin)

\t-d\tDecompress
\t-c\tWrite to stdout
\t-f\tForce
\t-k\tKeep input files"
  },
  usage_data {
    aname: "bzip2",
    usage: "\
[OPTIONS] [FILE]...

Compress FILEs (or stdin) with bzip2 algorithm

\t-1..9\tCompression level
\t-d\tDecompress
\t-t\tTest file integrity
\t-c\tWrite to stdout
\t-f\tForce
\t-k\tKeep input files"
  },
  usage_data {
    aname: "cpio",
    usage: "\
[-dmvu] [-F FILE] [-R USER[:GRP]] [-H newc] [-tio] [-p DIR] [EXTR_FILE]...

Extract (-i) or list (-t) files from a cpio archive, or
take file list from stdin and create an archive (-o) or copy files (-p)

Main operation mode:
\t-t\tList
\t-i\tExtract EXTR_FILEs (or all)
\t-o\tCreate (requires -H newc)
\t-p DIR\tCopy files to DIR
Options:
\t-H newc\tArchive format
\t-d\tMake leading directories
\t-m\tPreserve mtime
\t-v\tVerbose
\t-u\tOverwrite
\t-F FILE\tInput (-t,-i,-p) or output (-o) file
\t-R USER[:GRP]\tSet owner of created files
\t-L\tDereference symlinks
\t-0\tInput is separated by NULs"
  },
  usage_data {
    aname: "dpkg",
    usage: "\
[-ilCPru] [-F OPT] PACKAGE

Install, remove and manage Debian packages

\t-i,--install\tInstall the package
\t-l,--list\tList of installed packages
\t--configure\tConfigure an unpackaged package
\t-P,--purge\tPurge all files of a package
\t-r,--remove\tRemove all but the configuration files for a package
\t--unpack\tUnpack a package, but don\'t configure it
\t--force-depends\tIgnore dependency problems
\t--force-confnew\tOverwrite existing config files when installing
\t--force-confold\tKeep old config files when installing"
  },
  usage_data {
    aname: "dpkg-deb",
    usage: "\
[-cefxX] FILE [DIR]

Perform actions on Debian packages (.deb)

\t-c\tList files
\t-f\tPrint control fields
\t-e\tExtract control files to DIR (default: ./DEBIAN)
\t-x\tExtract files to DIR (no default)
\t-X\tVerbose -x"
  },
  usage_data {
    aname: "gzip",
    usage: "\
[-cfkdt] [FILE]...

Compress FILEs (or stdin)

\t-d\tDecompress
\t-t\tTest file integrity
\t-c\tWrite to stdout
\t-f\tForce
\t-k\tKeep input files"
  },
  usage_data {
    aname: "lzop",
    usage: "\
[-cfUvd123456789CF] [FILE]...

\t-1..9\tCompression level
\t-d\tDecompress
\t-c\tWrite to stdout
\t-f\tForce
\t-U\tDelete input files
\t-v\tVerbose
\t-F\tDon\'t store or verify checksum
\t-C\tAlso write checksum of compressed block"
  },
  usage_data {
    aname: "rpm",
    usage: "\
-i PACKAGE.rpm; rpm -qp[ildc] PACKAGE.rpm

Manipulate RPM packages

Commands:
\t-i\tInstall package
\t-qp\tQuery package
\t-qpi\tShow information
\t-qpl\tList contents
\t-qpd\tList documents
\t-qpc\tList config files"
  },
  usage_data {
    aname: "rpm2cpio",
    usage: "\
PACKAGE.rpm

Output a cpio archive of the rpm file",
  },
  usage_data {
    aname: "tar",
    usage: "\
c|x|t [-zJjahmvokO] [-f TARFILE] [-C DIR] [-T FILE] [-X FILE] [--exclude PATTERN]... [FILE]...

Create, extract, or list files from a tar file

\tc\tCreate
\tx\tExtract
\tt\tList
\t-f FILE\tName of TARFILE (\'-\' for stdin/out)
\t-C DIR\tChange to DIR before operation
\t-v\tVerbose
\t-O\tExtract to stdout
\t-m\tDon\'t restore mtime
\t-o\tDon\'t restore user:group
\t-k\tDon\'t replace existing files
\t-z\t(De)compress using gzip
\t-J\t(De)compress using xz
\t-j\t(De)compress using bzip2
\t-a\t(De)compress based on extension
\t-h\tFollow symlinks
\t-T FILE\tFile with names to include
\t-X FILE\tFile with glob patterns to exclude
\t--exclude PATTERN\tGlob pattern to exclude"
  },
  usage_data {
    aname: "unzip",
    usage: "\
[-lnojpq] FILE[.zip] [FILE]... [-x FILE...] [-d DIR]

Extract FILEs from ZIP archive

\t-l\tList contents (with -q for short form)
\t-n\tNever overwrite files (default: ask)
\t-o\tOverwrite
\t-j\tDo not restore paths
\t-p\tPrint to stdout
\t-q\tQuiet
\t-x FILE\tExclude FILEs
\t-d DIR\tExtract into DIR"
  },
  usage_data {
    aname: "chvt",
    usage: "\
N

Change the foreground virtual terminal to /dev/ttyN",
  },
  usage_data {
    aname: "clear",
    usage: "\n\nClear screen",
  },
  usage_data {
    aname: "deallocvt",
    usage: "\
[N]

Deallocate unused virtual terminal /dev/ttyN",
  },
  usage_data {
    aname: "dumpkmap",
    usage: "\
> keymap

Print a binary keyboard translation table to stdout",
  },
  usage_data {
    aname: "fgconsole",
    usage: "\n\nGet active console",
  },
  usage_data {
    aname: "kbd_mode",
    usage: "\
[-a|k|s|u] [-C TTY]

Report or set VT console keyboard mode

\t-a\tDefault (ASCII)
\t-k\tMedium-raw (keycode)
\t-s\tRaw (scancode)
\t-u\tUnicode (utf-8)
\t-C TTY\tAffect TTY"
  },
  usage_data {
    aname: "loadfont",
    usage: "\
< font

Load a console font from stdin",
  },
  usage_data {
    aname: "setfont",
    usage: "\
FONT [-m MAPFILE] [-C TTY]

Load a console font

\t-m MAPFILE\tLoad console screen map
\t-C TTY\t\tAffect TTY instead of /dev/tty"
  },
  usage_data {
    aname: "loadkmap",
    usage: "\
< keymap

Load a binary keyboard translation table from stdin",
  },
  usage_data {
    aname: "openvt",
    usage: "\
[-c N] [-sw] [PROG ARGS]

Start PROG on a new virtual terminal

\t-c N\tUse specified VT
\t-s\tSwitch to the VT
\t-w\tWait for PROG to exit"
  },
  usage_data {
    aname: "reset",
    usage: "\n\nReset the screen",
  },
  usage_data {
    aname: "resize",
    usage: "\n\nResize the screen",
  },
  usage_data {
    aname: "setconsole",
    usage: "\
[-r] [DEVICE]

Make writes to /dev/console appear on DEVICE (default: /dev/tty).
Does not redirect kernel log output or reads from /dev/console.

\t-r\tReset: writes to /dev/console go to kernel log tty(s)"
  },
  usage_data {
    aname: "setkeycodes",
    usage: "\
{ SCANCODE KEYCODE }...

Modify kernel\'s scancode-to-keycode map,
allowing unusual keyboards to generate usable keycodes.

SCANCODE is either xx or e0xx (hexadecimal), KEYCODE is decimal."
  },
  usage_data {
    aname: "setlogcons",
    usage: "\
[N]

Pin kernel output to VT console N. Default:0 (do not pin)",
  },
  usage_data {
    aname: "showkey",
    usage: "\
[-a | -k | -s]

Show keys pressed

\t-a\tDisplay decimal/octal/hex values of the keys
\t-k\tDisplay interpreted keycodes (default)
\t-s\tDisplay raw scan-codes"
  },
  usage_data {
    aname: "basename",
    usage: "\
FILE [SUFFIX]

Strip directory path and .SUFFIX from FILE",
  },
  usage_data {
    aname: "cat",
    usage: "\
[-nbvteA] [FILE]...

Print FILEs to stdout

\t-n\tNumber output lines
\t-b\tNumber nonempty lines
\t-v\tShow nonprinting characters as ^x or M-x
\t-t\t...and tabs as ^I
\t-e\t...and end lines with $
\t-A\tSame as -vte"
  },
  usage_data {
    aname: "chgrp",
    usage: "\
[-RhLHPcvf]... GROUP FILE...

Change the group membership of each FILE to GROUP

\t-R\tRecurse
\t-h\tAffect symlinks instead of symlink targets
\t-L\tTraverse all symlinks to directories
\t-H\tTraverse symlinks on command line only
\t-P\tDon\'t traverse symlinks (default)
\t-c\tList changed files
\t-v\tVerbose
\t-f\tHide errors"
  },
  usage_data {
    aname: "chmod",
    usage: "\
[-Rcvf] MODE[,MODE]... FILE...

Each MODE is one or more of the letters ugoa, one of the
symbols +-= and one or more of the letters rwxst

\t-R\tRecurse
\t-c\tList changed files
\t-v\tList all files
\t-f\tHide errors"
  },
  usage_data {
    aname: "chown",
    usage: "\
[-RhLHPcvf]... USER[:[GRP]] FILE...

Change the owner and/or group of each FILE to USER and/or GRP

\t-R\tRecurse
\t-h\tAffect symlinks instead of symlink targets
\t-L\tTraverse all symlinks to directories
\t-H\tTraverse symlinks on command line only
\t-P\tDon\'t traverse symlinks (default)
\t-c\tList changed files
\t-v\tList all files
\t-f\tHide errors"
  },
  usage_data {
    aname: "chroot",
    usage: "\
NEWROOT [PROG ARGS]

Run PROG with root directory set to NEWROOT",
  },
  usage_data {
    aname: "cksum",
    usage: "\
FILE...

Calculate the CRC32 checksums of FILEs",
  },
  usage_data {
    aname: "comm",
    usage: "\
[-123] FILE1 FILE2

Compare FILE1 with FILE2

\t-1\tSuppress lines unique to FILE1
\t-2\tSuppress lines unique to FILE2
\t-3\tSuppress lines common to both files"
  },
  usage_data {
    aname: "cp",
    usage: "\
[OPTIONS] SOURCE... DEST

Copy SOURCE(s) to DEST

\t-a\tSame as -dpR
\t-R,-r\tRecurse
\t-d,-P\tPreserve symlinks (default if -R)
\t-L\tFollow all symlinks
\t-H\tFollow symlinks on command line
\t-p\tPreserve file attributes if possible
\t-f\tOverwrite
\t-i\tPrompt before overwrite
\t-l,-s\tCreate (sym)links
\t-T\tTreat DEST as a normal file
\t-u\tCopy only newer files"
  },
  usage_data {
    aname: "cut",
    usage: "\
[OPTIONS] [FILE]...

Print selected fields from each input FILE to stdout

\t-b LIST\tOutput only bytes from LIST
\t-c LIST\tOutput only characters from LIST
\t-d CHAR\tUse CHAR instead of tab as the field delimiter
\t-s\tOutput only the lines containing delimiter
\t-f N\tPrint only these fields
\t-n\tIgnored"
  },
  usage_data {
    aname: "date",
    usage: "\
[OPTIONS] [+FMT] [TIME]

Display time (using +FMT), or set time

\t[-s,--set] TIME\tSet time to TIME
\t-u,--utc\tWork in UTC (don\'t convert to local time)
\t-R,--rfc-2822\tOutput RFC-2822 compliant date string
\t-I[SPEC]\tOutput ISO-8601 compliant date string
\t\t\tSPEC=\'date\' (default) for date only,
\t\t\t\'hours\', \'minutes\', or \'seconds\' for date and
\t\t\ttime to the indicated precision
\t-r,--reference FILE\tDisplay last modification time of FILE
\t-d,--date TIME\tDisplay TIME, not \'now\'
\t-D FMT\t\tUse FMT (strptime format) for -d TIME conversion

Recognized TIME formats:
\thh:mm[:ss]
\t[YYYY.]MM.DD-hh:mm[:ss]
\tYYYY-MM-DD hh:mm[:ss]
\t[[[[[YY]YY]MM]DD]hh]mm[.ss]
\t\'date TIME\' form accepts MMDDhhmm[[YY]YY][.ss] instead"
  },
  usage_data {
    aname: "dd",
    usage: "\
[if=FILE] [of=FILE] [ibs=N obs=N/bs=N] [count=N] [skip=N] [seek=N]
\t[conv=notrunc|noerror|sync|fsync]
\t[iflag=skip_bytes|fullblock] [oflag=seek_bytes|append]

Copy a file with converting and formatting

\tif=FILE\t\tRead from FILE instead of stdin
\tof=FILE\t\tWrite to FILE instead of stdout
\tbs=N\t\tRead and write N bytes at a time
\tibs=N\t\tRead N bytes at a time
\tobs=N\t\tWrite N bytes at a time
\tcount=N\t\tCopy only N input blocks
\tskip=N\t\tSkip N input blocks
\tseek=N\t\tSkip N output blocks
\tconv=notrunc\tDon\'t truncate output file
\tconv=noerror\tContinue after read errors
\tconv=sync\tPad blocks with zeros
\tconv=fsync\tPhysically write data out before finishing
\tconv=swab\tSwap every pair of bytes
\tiflag=skip_bytes\tskip=N is in bytes
\tiflag=fullblock\tRead full blocks
\toflag=seek_bytes\tseek=N is in bytes
\toflag=append\tOpen output file in append mode
\tstatus=noxfer\tSuppress rate output
\tstatus=none\tSuppress all output

N may be suffixed by c (1), w (2), b (512), kB (1000), k (1024), MB, M, GB, G"
  },
  usage_data {
    aname: "df",
    usage: "\
[-PkmhTai] [-B SIZE] [FILESYSTEM]...

Print filesystem usage statistics

\t-P\tPOSIX output format
\t-k\t1024-byte blocks (default)
\t-m\t1M-byte blocks
\t-h\tHuman readable (e.g. 1K 243M 2G)
\t-T\tPrint filesystem type
\t-a\tShow all filesystems
\t-i\tInodes
\t-B SIZE\tBlocksize"
  },
  usage_data {
    aname: "dirname",
    usage: "\
FILENAME

Strip non-directory suffix from FILENAME",
  },
  usage_data {
    aname: "dos2unix",
    usage: "\
[-ud] [FILE]

Convert FILE in-place from DOS to Unix format.
When no file is given, use stdin/stdout.

\t-u\tdos2unix
\t-d\tunix2dos"
  },
  usage_data {
    aname: "unix2dos",
    usage: "\
[-ud] [FILE]

Convert FILE in-place from Unix to DOS format.
When no file is given, use stdin/stdout.

\t-u\tdos2unix
\t-d\tunix2dos"
  },
  usage_data {
    aname: "du",
    usage: "\
[-aHLdclsxhmk] [FILE]...

Summarize disk space used for each FILE and/or directory

\t-a\tShow file sizes too
\t-L\tFollow all symlinks
\t-H\tFollow symlinks on command line
\t-d N\tLimit output to directories (and files with -a) of depth < N
\t-c\tShow grand total
\t-l\tCount sizes many times if hard linked
\t-s\tDisplay only a total for each argument
\t-x\tSkip directories on different filesystems
\t-h\tSizes in human readable format (e.g., 1K 243M 2G)
\t-m\tSizes in megabytes
\t-k\tSizes in kilobytes (default)"
  },
  usage_data {
    aname: "echo",
    usage: "\
[-neE] [ARG]...

Print the specified ARGs to stdout

\t-n\tSuppress trailing newline
\t-e\tInterpret backslash escapes (i.e., \\t=tab)
\t-E\tDon\'t interpret backslash escapes (default)"
  },
  usage_data {
    aname: "env",
    usage: "\
[-iu] [-] [name=value]... [PROG ARGS]

Print the current environment or run PROG after setting up
the specified environment

\t-, -i\tStart with an empty environment
\t-u\tRemove variable from the environment"
  },
  usage_data {
    aname: "expand",
    usage: "\
[-i] [-t N] [FILE]...

Convert tabs to spaces, writing to stdout

\t-i\tDon\'t convert tabs after non blanks
\t-t\tTabstops every N chars"
  },
  usage_data {
    aname: "unexpand",
    usage: "\
[-fa][-t N] [FILE]...

Convert spaces to tabs, writing to stdout

\t-a\tConvert all blanks
\t-f\tConvert only leading blanks
\t-t N\tTabstops every N chars"
  },
  usage_data {
    aname: "expr",
    usage: "\
EXPRESSION

Print the value of EXPRESSION to stdout

EXPRESSION may be:
\tARG1 | ARG2\tARG1 if it is neither null nor 0, otherwise ARG2
\tARG1 & ARG2\tARG1 if neither argument is null or 0, otherwise 0
\tARG1 < ARG2\t1 if ARG1 is less than ARG2, else 0. Similarly:
\tARG1 <= ARG2
\tARG1 = ARG2
\tARG1 != ARG2
\tARG1 >= ARG2
\tARG1 > ARG2
\tARG1 + ARG2\tSum of ARG1 and ARG2. Similarly:
\tARG1 - ARG2
\tARG1 * ARG2
\tARG1 / ARG2
\tARG1 % ARG2
\tSTRING : REGEXP\t\tAnchored pattern match of REGEXP in STRING
\tmatch STRING REGEXP\tSame as STRING : REGEXP
\tsubstr STRING POS LENGTH Substring of STRING, POS counted from 1
\tindex STRING CHARS\tIndex in STRING where any CHARS is found, or 0
\tlength STRING\t\tLength of STRING
\tquote TOKEN\t\tInterpret TOKEN as a string, even if
\t\t\t\tit is a keyword like \'match\' or an
\t\t\t\toperator like \'/\'
\t(EXPRESSION)\t\tValue of EXPRESSION

Beware that many operators need to be escaped or quoted for shells.
Comparisons are arithmetic if both ARGs are numbers, else
lexicographical. Pattern matches return the string matched between
\\( and \\) or null; if \\( and \\) are not used, they return the number
of characters matched or 0."
  },
  usage_data {
    aname: "factor",
    usage: "\
[NUMBER]...

Print prime factors",
  },
  usage_data {
    aname: "false",
    usage: "\x08",
  },
  usage_data {
    aname: "fold",
    usage: "\
[-bs] [-w WIDTH] [FILE]...

Wrap input lines in each FILE (or stdin), writing to stdout

\t-b\tCount bytes rather than columns
\t-s\tBreak at spaces
\t-w\tUse WIDTH columns instead of 80"
  },
  usage_data {
    aname: "head",
    usage: "\
[OPTIONS] [FILE]...

Print first 10 lines of each FILE (or stdin) to stdout.
With more than one FILE, precede each with a filename header.

\t-n N[kbm]\tPrint first N lines
\t-n -N[kbm]\tPrint all except N last lines
\t-c [-]N[kbm]\tPrint first N bytes
\t-q\t\tNever print headers
\t-v\t\tAlways print headers

N may be suffixed by k (x1024), b (x512), or m (x1024^2)."
  },
  usage_data {
    aname: "hostid",
    usage: "\n\nPrint out a unique 32-bit identifier for the machine",
  },
  usage_data {
    aname: "groups",
    usage: "\
[USER]

Print the group memberships of USER or for the current process",
  },
  usage_data {
    aname: "id",
    usage: "\
[OPTIONS] [USER]

Print information about USER or the current user

\t-u\tUser ID
\t-g\tGroup ID
\t-G\tSupplementary group IDs
\t-n\tPrint names instead of numbers
\t-r\tPrint real ID instead of effective ID"
  },
  usage_data {
    aname: "install",
    usage: "\
[-cdDsp] [-o USER] [-g GRP] [-m MODE] [-t DIR] [SOURCE]... DEST

Copy files and set attributes

\t-c\tJust copy (default)
\t-d\tCreate directories
\t-D\tCreate leading target directories
\t-s\tStrip symbol table
\t-p\tPreserve date
\t-o USER\tSet ownership
\t-g GRP\tSet group ownership
\t-m MODE\tSet permissions
\t-t DIR\tInstall to DIR"
  },
  usage_data {
    aname: "link",
    usage: "\
FILE LINK

Create hard LINK to FILE",
  },
  usage_data {
    aname: "ln",
    usage: "\
[OPTIONS] TARGET... LINK|DIR

Create a link LINK or DIR/TARGET to the specified TARGET(s)

\t-s\tMake symlinks instead of hardlinks
\t-f\tRemove existing destinations
\t-n\tDon\'t dereference symlinks - treat like normal file
\t-b\tMake a backup of the target (if exists) before link operation
\t-S suf\tUse suffix instead of ~ when making backup files
\t-T\tTreat LINK as a file, not DIR
\t-v\tVerbose"
  },
  usage_data {
    aname: "logname",
    usage: "\n\nPrint the name of the current user",
  },
  usage_data {
    aname: "ls",
    usage: "\
[-1AaCxdLHRFplinshrSXvctu] [-w WIDTH] [FILE]...

List directory contents

\t-1\tOne column output
\t-a\tInclude entries which start with .
\t-A\tLike -a, but exclude . and ..
\t-x\tList by lines
\t-d\tList directory entries instead of contents
\t-L\tFollow symlinks
\t-H\tFollow symlinks on command line
\t-R\tRecurse
\t-p\tAppend / to dir entries
\t-F\tAppend indicator (one of */=@|) to entries
\t-l\tLong listing format
\t-i\tList inode numbers
\t-n\tList numeric UIDs and GIDs instead of names
\t-s\tList allocated blocks
\t-lc\tList ctime
\t-lu\tList atime
\t--full-time\tList full date and time
\t-h\tHuman readable sizes (1K 243M 2G)
\t--group-directories-first
\t-S\tSort by size
\t-X\tSort by extension
\t-v\tSort by version
\t-t\tSort by mtime
\t-tc\tSort by ctime
\t-tu\tSort by atime
\t-r\tReverse sort order
\t-w N\tFormat N columns wide
\t--color[={always,never,auto}]\tControl coloring"
  },
  usage_data {
    aname: "md5sum",
    usage: "\
[-c[sw]] [FILE]...

Print or check MD5 checksums

\t-c\tCheck sums against list in FILEs
\t-s\tDon\'t output anything, status code shows success
\t-w\tWarn about improperly formatted checksum lines"
  },
  usage_data {
    aname: "sha1sum",
    usage: "\
[-c[sw]] [FILE]...

Print or check SHA1 checksums

\t-c\tCheck sums against list in FILEs
\t-s\tDon\'t output anything, status code shows success
\t-w\tWarn about improperly formatted checksum lines"
  },
  usage_data {
    aname: "sha3sum",
    usage: "\
[-c[sw]] [-a BITS] [FILE]...

Print or check SHA3 checksums

\t-c\tCheck sums against list in FILEs
\t-s\tDon\'t output anything, status code shows success
\t-w\tWarn about improperly formatted checksum lines
\t-a BITS\t224 (default), 256, 384, 512"
  },
  usage_data {
    aname: "sha256sum",
    usage: "\
[-c[sw]] [FILE]...

Print or check SHA256 checksums

\t-c\tCheck sums against list in FILEs
\t-s\tDon\'t output anything, status code shows success
\t-w\tWarn about improperly formatted checksum lines"
  },
  usage_data {
    aname: "sha512sum",
    usage: "\
[-c[sw]] [FILE]...

Print or check SHA512 checksums

\t-c\tCheck sums against list in FILEs
\t-s\tDon\'t output anything, status code shows success
\t-w\tWarn about improperly formatted checksum lines"
  },
  usage_data {
    aname: "mkdir",
    usage: "\
[OPTIONS] DIRECTORY...

Create DIRECTORY

\t-m MODE\tMode
\t-p\tNo error if exists; make parent directories as needed"
  },
  usage_data {
    aname: "mkfifo",
    usage: "\
[-m MODE] NAME

Create named pipe

\t-m MODE\tMode (default a=rw)",
  },
  usage_data {
    aname: "mknod",
    usage: "\
[-m MODE] NAME TYPE [MAJOR MINOR]

Create a special file (block, character, or pipe)

\t-m MODE\tCreation mode (default a=rw)
TYPE:
\tb\tBlock device
\tc or u\tCharacter device
\tp\tNamed pipe (MAJOR MINOR must be omitted)"
  },
  usage_data {
    aname: "mktemp",
    usage: "\
[-dt] [-p DIR] [TEMPLATE]

Create a temporary file with name based on TEMPLATE and print its name.
TEMPLATE must end with XXXXXX (e.g. [/dir/]nameXXXXXX).
Without TEMPLATE, -t tmp.XXXXXX is assumed.

\t-d\tMake directory, not file
\t-q\tFail silently on errors
\t-t\tPrepend base directory name to TEMPLATE
\t-p DIR\tUse DIR as a base directory (implies -t)
\t-u\tDo not create anything; print a name

Base directory is: -p DIR, else $TMPDIR, else /tmp"
  },
  usage_data {
    aname: "mv",
    usage: "\
[-fin] SOURCE DEST
or: mv [-fin] SOURCE... DIRECTORY

Rename SOURCE to DEST, or move SOURCE(s) to DIRECTORY

\t-f\tDon\'t prompt before overwriting
\t-i\tInteractive, prompt before overwrite
\t-n\tDon\'t overwrite an existing file"
  },
  usage_data {
    aname: "nice",
    usage: "\
[-n ADJUST] [PROG ARGS]

Change scheduling priority, run PROG

\t-n ADJUST\tAdjust priority by ADJUST"
  },
  usage_data {
    aname: "nl",
    usage: "\
[OPTIONS] [FILE]...

Write FILEs to standard output with line numbers added

\t-b STYLE\tWhich lines to number - a: all, t: nonempty, n: none
\t-i N\t\tLine number increment
\t-s STRING\tUse STRING as line number separator
\t-v N\t\tStart from N
\t-w N\t\tWidth of line numbers"},
  usage_data {
    aname: "nohup",
    usage: "\
PROG ARGS

Run PROG immune to hangups, with output to a non-tty",
  },
  usage_data {
    aname: "nproc",
    usage: "\
--all --ignore=N

Print number of available CPUs

\t--all\t\tNumber of installed CPUs
\t--ignore=N\tExclude N CPUs"
  },
  usage_data {
    aname: "od",
    usage: "\
[-abcdfhilovxs] [-t TYPE] [-A RADIX] [-N SIZE] [-j SKIP] [-S MINSTR] [-w WIDTH] [FILE]...

Print FILEs (or stdin) unambiguously, as octal bytes by default"
  },
  usage_data {
    aname: "paste",
    usage: "\
[OPTIONS] [FILE]...

Paste lines from each input file, separated with tab

\t-d LIST\tUse delimiters from LIST, not tab
\t-s      Serial: one file at a time"
  },
  usage_data {
    aname: "printenv",
    usage: "\
[VARIABLE]...

Print environment VARIABLEs.
If no VARIABLE specified, print all.",
  },
  usage_data {
    aname: "printf",
    usage: "\
FORMAT [ARG]...

Format and print ARG(s) according to FORMAT (a-la C printf)",
  },
  usage_data {
    aname: "pwd",
    usage: "\n\nPrint the full filename of the current working directory",
  },
  usage_data {
    aname: "readlink",
    usage: "\
[-fnv] FILE

Display the value of a symlink

\t-f\tCanonicalize by following all symlinks
\t-n\tDon\'t add newline
\t-v\tVerbose"
  },
  usage_data {
    aname: "realpath",
    usage: "\
FILE...

Return the absolute pathnames of given FILE",
  },
  usage_data {
    aname: "rm",
    usage: "\
[-irf] FILE...

Remove (unlink) FILEs

\t-i\tAlways prompt before removing
\t-f\tNever prompt
\t-R,-r\tRecurse"
  },
  usage_data {
    aname: "rmdir",
    usage: "\
[OPTIONS] DIRECTORY...

Remove DIRECTORY if it is empty

\t-p\tInclude parents
\t--ignore-fail-on-non-empty"
  },
  usage_data {
    aname: "seq",
    usage: "\
[-w] [-s SEP] [FIRST [INC]] LAST

Print numbers from FIRST to LAST, in steps of INC.
FIRST, INC default to 1.

\t-w\tPad to last with leading zeros
\t-s SEP\tString separator"
  },
  usage_data {
    aname: "shred",
    usage: "\
FILE...

Overwrite/delete FILEs

\t-f\tChmod to ensure writability
\t-n N\tOverwrite N times (default 3)
\t-z\tFinal overwrite with zeros
\t-u\tRemove file"
  },
  usage_data {
    aname: "shuf",
    usage: "\
[-e|-i L-H] [-n NUM] [-o FILE] [-z] [FILE|ARG...]

Randomly permute lines

\t-e\tTreat ARGs as lines
\t-i L-H\tTreat numbers L-H as lines
\t-n NUM\tOutput at most NUM lines
\t-o FILE\tWrite to FILE, not standard output
\t-z\tEnd lines with zero byte, not newline"
  },
  usage_data {
    aname: "sleep",
    usage: "\
[N]...

Pause for a time equal to the total of the args given, where each arg can
have an optional suffix of (s)econds, (m)inutes, (h)ours, or (d)ays"
  },
  usage_data {
    aname: "sort",
    usage: "\
[-nrugMcszbdfiokt] [-o FILE] [-k start[.offset][opts][,end[.offset][opts]] [-t CHAR] [FILE]...

Sort lines of text

\t-o FILE\tOutput to FILE
\t-c\tCheck whether input is sorted
\t-b\tIgnore leading blanks
\t-f\tIgnore case
\t-i\tIgnore unprintable characters
\t-d\tDictionary order (blank or alphanumeric only)
\t-n\tSort numbers
\t-g\tGeneral numerical sort
\t-M\tSort month
\t-V\tSort version
\t-t CHAR\tField separator
\t-k N[,M] Sort by Nth field
\t-r\tReverse sort order
\t-s\tStable (don\'t sort ties alphabetically)
\t-u\tSuppress duplicate lines
\t-z\tLines are terminated by NUL, not newline"
  },
  usage_data {
    aname: "split",
    usage: "\
[OPTIONS] [INPUT [PREFIX]]

\t-b N[k|m]\tSplit by N (kilo|mega)bytes
\t-l N\t\tSplit by N lines
\t-a N\t\tUse N letters as suffix"
  },
  usage_data {
    aname: "stat",
    usage: "\
[OPTIONS] FILE...

Display file (default) or filesystem status

\t-c FMT\tUse the specified format
\t-f\tDisplay filesystem status
\t-L\tFollow links
\t-t\tTerse display

FMT sequences for files:
 %a\tAccess rights in octal
 %A\tAccess rights in human readable form
 %b\tNumber of blocks allocated (see %B)
 %B\tSize in bytes of each block reported by %b
 %d\tDevice number in decimal
 %D\tDevice number in hex
 %f\tRaw mode in hex
 %F\tFile type
 %g\tGroup ID
 %G\tGroup name
 %h\tNumber of hard links
 %i\tInode number
 %n\tFile name
 %N\tFile name, with -> TARGET if symlink
 %o\tI/O block size
 %s\tTotal size in bytes
 %t\tMajor device type in hex
 %T\tMinor device type in hex
 %u\tUser ID
 %U\tUser name
 %x\tTime of last access
 %X\tTime of last access as seconds since Epoch
 %y\tTime of last modification
 %Y\tTime of last modification as seconds since Epoch
 %z\tTime of last change
 %Z\tTime of last change as seconds since Epoch

FMT sequences for file systems:
 %a\tFree blocks available to non-superuser
 %b\tTotal data blocks
 %c\tTotal file nodes
 %d\tFree file nodes
 %f\tFree blocks
 %i\tFile System ID in hex
 %l\tMaximum length of filenames
 %n\tFile name
 %s\tBlock size (for faster transfer)
 %S\tFundamental block size (for block counts)
 %t\tType in hex
 %T\tType in human readable form"
  },
  usage_data {
    aname: "stty",
    usage: "\
[-a|g] [-F DEVICE] [SETTING]...

Without arguments, prints baud rate, line discipline,
and deviations from stty sane

\t-F DEVICE\tOpen device instead of stdin
\t-a\t\tPrint all current settings in human-readable form
\t-g\t\tPrint in stty-readable form
\t[SETTING]\tSee manpage"
  },
  usage_data {
    aname: "sum",
    usage: "\
[-rs] [FILE]...

Checksum and count the blocks in a file

\t-r\tUse BSD sum algorithm (1K blocks)
\t-s\tUse System V sum algorithm (512byte blocks)"
  },
  usage_data {
    aname: "sync",
    usage: "\
[-df] [FILE]...

Write all buffered blocks (in FILEs) to disk
\t-d\tAvoid syncing metadata
\t-f\tSync filesystems underlying FILEs"
  },
  usage_data {
    aname: "fsync",
    usage: "[-d] FILE...

        Write all buffered blocks in FILEs to disk

        \t-d\tAvoid syncing metadata",
  },
  usage_data {
    aname: "tac",
    usage: "\
[FILE]...

Concatenate FILEs and print them in reverse",
  },
  usage_data {
    aname: "tail",
    usage: "\
[OPTIONS] [FILE]...

Print last 10 lines of each FILE (or stdin) to stdout.
With more than one FILE, precede each with a filename header.

\t-f\t\tPrint data as file grows
\t-c [+]N[kbm]\tPrint last N bytes
\t-n N[kbm]\tPrint last N lines
\t-n +N[kbm]\tStart on Nth line and print the rest
\t-q\t\tNever print headers
\t-s SECONDS\tWait SECONDS between reads with -f
\t-v\t\tAlways print headers
\t-F\t\tSame as -f, but keep retrying

N may be suffixed by k (x1024), b (x512), or m (x1024^2)."
  },
  usage_data {
    aname: "tee",
    usage: "\
[-ai] [FILE]...

Copy stdin to each FILE, and also to stdout

\t-a\tAppend to the given FILEs, don\'t overwrite
\t-i\tIgnore interrupt signals (SIGINT)"
  },
  usage_data {
    aname: "test",
    usage: "\x08",
  },
  usage_data {
    aname: "[",
    usage: "\x08",
  },
  usage_data {
    aname: "[[",
    usage: "\x08",
  },
  usage_data {
    aname: "timeout",
    usage: "\
[-s SIG] SECS PROG ARGS

Runs PROG. Sends SIG to it if it is not gone in SECS seconds.
Default SIG: TERM."
  },
  usage_data {
    aname: "touch",
    usage: "\
[-c] [-d DATE] [-t DATE] [-r FILE] FILE...

Update the last-modified date on the given FILE[s]

\t-c\tDon\'t create files
\t-h\tDon\'t follow links
\t-d DT\tDate/time to use
\t-t DT\tDate/time to use
\t-r FILE\tUse FILE\'s date/time"
  },
  usage_data {
    aname: "tr",
    usage: "\
[-cds] STRING1 [STRING2]

Translate, squeeze, or delete characters from stdin, writing to stdout

\t-c\tTake complement of STRING1
\t-d\tDelete input characters coded STRING1
\t-s\tSqueeze multiple output characters of STRING2 into one character"
  },
  usage_data {
    aname: "true",
    usage: "\x08",
  },
  usage_data {
    aname: "truncate",
    usage: "\
[-c] -s SIZE FILE...

Truncate FILEs to the given size

\t-c\tDo not create files
\t-s SIZE\tTruncate to SIZE"
  },
  usage_data {
    aname: "tty",
    usage: "\


Print file name of stdin\'s terminal

\t-s\tPrint nothing, only return exit status",
  },
  usage_data {
    aname: "uname",
    usage: "\
[-amnrspvio]

Print system information

\t-a\tPrint all
\t-m\tThe machine (hardware) type
\t-n\tHostname
\t-r\tKernel release
\t-s\tKernel name (default)
\t-p\tProcessor type
\t-v\tKernel version
\t-i\tThe hardware platform
\t-o\tOS name"
  },
  usage_data {
    aname: "arch",
    usage: "\n\nPrint system architecture",
  },
  usage_data {
    aname: "uniq",
    usage: "\
[-cdu][-f,s,w N] [INPUT [OUTPUT]]

Discard duplicate lines

\t-c\tPrefix lines by the number of occurrences
\t-d\tOnly print duplicate lines
\t-u\tOnly print unique lines
\t-i\tIgnore case
\t-f N\tSkip first N fields
\t-s N\tSkip first N chars (after any skipped fields)
\t-w N\tCompare N characters in line"
  },
  usage_data {
    aname: "unlink",
    usage: "\
FILE

Delete FILE by calling unlink()",
  },
  usage_data {
    aname: "usleep",
    usage: "\
N

Pause for N microseconds",
  },
  usage_data {
    aname: "uudecode",
    usage: "\
[-o OUTFILE] [INFILE]

Uudecode a file
Finds OUTFILE in uuencoded source unless -o is given"
  },
  usage_data {
    aname: "base64",
    usage: "\
[-d] [FILE]

Base64 encode or decode FILE to standard output
\t-d\tDecode data",
  },
  usage_data {
    aname: "uuencode",
    usage: "\
[-m] [FILE] STORED_FILENAME

Uuencode FILE (or stdin) to stdout

\t-m\tUse base64 encoding per RFC1521"
  },
  usage_data {
    aname: "wc",
    usage: "\
[-cmlwL] [FILE]...

Count lines, words, and bytes for each FILE (or stdin)

\t-c\tCount bytes
\t-m\tCount characters
\t-l\tCount newlines
\t-w\tCount words
\t-L\tPrint longest line length"
  },
  usage_data {
    aname: "users",
    usage: "\n\nPrint the users currently logged on",
  },
  usage_data {
    aname: "w",
    usage: "\n\nShow who is logged on",
  },
  usage_data {
    aname: "who",
    usage: "\
[-a]

Show who is logged on

\t-a\tShow all
\t-H\tPrint column headers",
  },
  usage_data {
    aname: "whoami",
    usage: "\n\nPrint the user name associated with the current effective user id",
  },
  usage_data {
    aname: "yes",
    usage: "\
[STRING]

Repeatedly output a line with STRING, or \'y\'",
  },
  usage_data {
    aname: "pipe_progress",
    usage: "\x08",
  },
  usage_data {
    aname: "run-parts",
    usage: "\
[-a ARG]... [-u UMASK] [--reverse] [--test] [--exit-on-error] [--list] DIRECTORY

Run a bunch of scripts in DIRECTORY

\t-a ARG\t\tPass ARG as argument to scripts
\t-u UMASK\tSet UMASK before running scripts
\t--reverse\tReverse execution order
\t--test\t\tDry run
\t--exit-on-error\tExit if a script exits with non-zero
\t--list\t\tPrint names of matching files even if they are not executable"
  },
  usage_data {
    aname: "start-stop-daemon",
    usage: "\
[OPTIONS] [-S|-K] ... [-- ARGS...]

Search for matching processes, and then
-K: stop all matching processes
-S: start a process unless a matching process is found

Process matching:
\t-u USERNAME|UID\tMatch only this user\'s processes
\t-n NAME\t\tMatch processes with NAME
\t\t\tin comm field in /proc/PID/stat
\t-x EXECUTABLE\tMatch processes with this command
\t\t\tin /proc/PID/cmdline
\t-p FILE\t\tMatch a process with PID from FILE
\tAll specified conditions must match
-S only:
\t-x EXECUTABLE\tProgram to run
\t-a NAME\t\tZeroth argument
\t-b\t\tBackground
\t-N N\t\tChange nice level
\t-c USER[:[GRP]]\tChange user/group
\t-m\t\tWrite PID to pidfile specified by -p
-K only:
\t-s SIG\t\tSignal to send
\t-t\t\tMatch only, exit with 0 if found
Other:
\t-o\t\tExit with status 0 if nothing is done
\t-v\t\tVerbose
\t-q\t\tQuiet"
  },
  usage_data {
    aname: "which",
    usage: "\
[COMMAND]...

Locate a COMMAND",
  },
  usage_data {
    aname: "chattr",
    usage: "\
[-R] [-v VERSION] [-+=AacDdijsStTu] FILE...

Change ext2 file attributes

\t-R\tRecurse
\t-v VER\tSet version/generation number
Modifiers:
\t-,+,=\tRemove/add/set attributes
Attributes:
\tA\tDon\'t track atime
\ta\tAppend mode only
\tc\tEnable compress
\tD\tWrite dir contents synchronously
\td\tDon\'t backup with dump
\ti\tCannot be modified (immutable)
\tj\tWrite all data to journal first
\ts\tZero disk storage when deleted
\tS\tWrite synchronously
\tt\tDisable tail-merging of partial blocks with other files
\tu\tAllow file to be undeleted"
  },
  usage_data {
    aname: "fsck",
    usage: "\
[-ANPRTV] [-t FSTYPE] [FS_OPTS] [BLOCKDEV]...

Check and repair filesystems

\t-A\tWalk /etc/fstab and check all filesystems
\t-N\tDon\'t execute, just show what would be done
\t-P\tWith -A, check filesystems in parallel
\t-R\tWith -A, skip the root filesystem
\t-T\tDon\'t show title on startup
\t-V\tVerbose
\t-t TYPE\tList of filesystem types to check"
  },
  usage_data {
    aname: "lsattr",
    usage: "\
[-Radlv] [FILE]...

List ext2 file attributes

\t-R\tRecurse
\t-a\tDon\'t hide entries starting with .
\t-d\tList directory entries instead of contents
\t-l\tList long flag names
\t-v\tList version/generation number"
  },
  usage_data {
    aname: "awk",
    usage: "\
[OPTIONS] [AWK_PROGRAM] [FILE]...

\t-v VAR=VAL\tSet variable
\t-F SEP\t\tUse SEP as field separator
\t-f FILE\t\tRead program from FILE
\t-e AWK_PROGRAM"
  },
  usage_data {
    aname: "cmp",
    usage: "\
[-l] [-s] FILE1 [FILE2 [SKIP1 [SKIP2]]]

Compare FILE1 with FILE2 (or stdin)

\t-l\tWrite the byte numbers (decimal) and values (octal)
\t\tfor all differing bytes
\t-s\tQuiet"
  },
  usage_data {
    aname: "diff",
    usage: "\
[-abBdiNqrTstw] [-L LABEL] [-S FILE] [-U LINES] FILE1 FILE2

Compare files line by line and output the differences between them.
This implementation supports unified diffs only.

\t-a\tTreat all files as text
\t-b\tIgnore changes in the amount of whitespace
\t-B\tIgnore changes whose lines are all blank
\t-d\tTry hard to find a smaller set of changes
\t-i\tIgnore case differences
\t-L\tUse LABEL instead of the filename in the unified header
\t-N\tTreat absent files as empty
\t-q\tOutput only whether files differ
\t-r\tRecurse
\t-S\tStart with FILE when comparing directories
\t-T\tMake tabs line up by prefixing a tab when necessary
\t-s\tReport when two files are the same
\t-t\tExpand tabs to spaces in output
\t-U\tOutput LINES lines of context
\t-w\tIgnore all whitespace"
  },
  usage_data {
    aname: "ed",
    usage: "[FILE]",
  },
  usage_data {
    aname: "patch",
    usage: "\
[OPTIONS] [ORIGFILE [PATCHFILE]]

\t-p N\tStrip N leading components from file names
\t-i DIFF\tRead DIFF instead of stdin
\t-R\tReverse patch
\t-N\tIgnore already applied patches
\t-E\tRemove output files if they become empty
\t--dry-run\tDon\'t actually change files"
  },
  usage_data {
    aname: "sed",
    usage: "\
[-i[SFX]] [-nrE] [-f FILE]... [-e CMD]... [FILE]...
or: sed [-i[SFX]] [-nrE] CMD [FILE]...

\t-e CMD\tAdd CMD to sed commands to be executed
\t-f FILE\tAdd FILE contents to sed commands to be executed
\t-i[SFX]\tEdit files in-place (otherwise sends to stdout)
\t\tOptionally back files up, appending SFX
\t-n\tSuppress automatic printing of pattern space
\t-r,-E\tUse extended regex syntax

If no -e or -f, the first non-option argument is the sed command string.
Remaining arguments are input files (stdin if none)."
  },
  usage_data {
    aname: "vi",
    usage: "\
[OPTIONS] [FILE]...

Edit FILE

\t-c CMD\tInitial command to run ($EXINIT also available)
\t-R\tRead-only
\t-H\tList available features"
  },
  usage_data {
    aname: "find",
    usage: "\
[-HL] [PATH]... [OPTIONS] [ACTIONS]

Search for files and perform actions on them.
First failed action stops processing of current file.
Defaults: PATH is current directory, action is \'-print\'

\t-L,-follow\tFollow symlinks
\t-H\t\t...on command line only
\t-xdev\t\tDon\'t descend directories on other filesystems
\t-maxdepth N\tDescend at most N levels. -maxdepth 0 applies
\t\t\tactions to command line arguments only
\t-mindepth N\tDon\'t act on first N levels
\t-depth\t\tAct on directory *after* traversing it

Actions:
\t( ACTIONS )\tGroup actions for -o / -a
\t! ACT\t\tInvert ACT\'s success/failure
\tACT1 [-a] ACT2\tIf ACT1 fails, stop, else do ACT2
\tACT1 -o ACT2\tIf ACT1 succeeds, stop, else do ACT2
\t\t\tNote: -a has higher priority than -o
\t-name PATTERN\tMatch file name (w/o directory name) to PATTERN
\t-iname PATTERN\tCase insensitive -name
\t-path PATTERN\tMatch path to PATTERN
\t-ipath PATTERN\tCase insensitive -path
\t-regex PATTERN\tMatch path to regex PATTERN
\t-type X\t\tFile type is X (one of: f,d,l,b,c,s,p)
\t-executable\tFile is executable
\t-perm MASK\tAt least one mask bit (+MASK), all bits (-MASK),
\t\t\tor exactly MASK bits are set in file\'s mode
\t-mtime DAYS\tmtime is greater than (+N), less than (-N),
\t\t\tor exactly N days in the past
\t-mmin MINS\tmtime is greater than (+N), less than (-N),
\t\t\tor exactly N minutes in the past
\t-newer FILE\tmtime is more recent than FILE\'s
\t-inum N\t\tFile has inode number N
\t-user NAME/ID\tFile is owned by given user
\t-group NAME/ID\tFile is owned by given group
\t-size N[bck]\tFile size is N (c:bytes,k:kbytes,b:512 bytes(def.))
\t\t\t+/-N: file size is bigger/smaller than N
\t-links N\tNumber of links is greater than (+N), less than (-N),
\t\t\tor exactly N
\t-empty\t\tMatch empty file/directory
\t-prune\t\tIf current file is directory, don\'t descend into it
If none of the following actions is specified, -print is assumed
\t-print\t\tPrint file name
\t-print0\t\tPrint file name, NUL terminated
\t-exec CMD ARG ;\tRun CMD with all instances of {} replaced by
\t\t\tfile name. Fails if CMD exits with nonzero
\t-exec CMD ARG + Run CMD with {} replaced by list of file names
\t-delete\t\tDelete current file/directory. Turns on -depth option
\t-quit\t\tExit"
  },
  usage_data {
    aname: "grep",
    usage: "\
[-HhnlLoqvsriwFE] [-m N] [-A/B/C N] PATTERN/-e PATTERN.../-f FILE [FILE]...

Search for PATTERN in FILEs (or stdin)

\t-H\tAdd \'filename:\' prefix
\t-h\tDo not add \'filename:\' prefix
\t-n\tAdd \'line_no:\' prefix
\t-l\tShow only names of files that match
\t-L\tShow only names of files that don\'t match
\t-c\tShow only count of matching lines
\t-o\tShow only the matching part of line
\t-q\tQuiet. Return 0 if PATTERN is found, 1 otherwise
\t-v\tSelect non-matching lines
\t-s\tSuppress open and read errors
\t-r\tRecurse
\t-i\tIgnore case
\t-w\tMatch whole words only
\t-x\tMatch whole lines only
\t-F\tPATTERN is a literal (not regexp)
\t-E\tPATTERN is an extended regexp
\t-m N\tMatch up to N times per file
\t-A N\tPrint N lines of trailing context
\t-B N\tPrint N lines of leading context
\t-C N\tSame as \'-A N -B N\'
\t-e PTRN\tPattern to match
\t-f FILE\tRead pattern from file"
  },
  usage_data {
    aname: "egrep",
    usage: "\x08",
  },
  usage_data {
    aname: "fgrep",
    usage: "\x08",
  },
  usage_data {
    aname: "xargs",
    usage: "\
[OPTIONS] [PROG ARGS]

Run PROG on every item given by stdin

\t-0\tInput is separated by NULs
\t-a FILE\tRead from FILE instead of stdin
\t-r\tDon\'t run command if input is empty
\t-t\tPrint the command on stderr before execution
\t-p\tAsk user whether to run each command
\t-E STR,-e[STR]\tSTR stops input processing
\t-I STR\tReplace STR within PROG ARGS with input line
\t-n N\tPass no more than N args to PROG
\t-s N\tPass command line of no more than N bytes
\t-P N\tRun up to N PROGs in parallel
\t-x\tExit if size is exceeded"
  },
  usage_data {
    aname: "bootchartd",
    usage: "\
start [PROG ARGS]|stop|init

Create /var/log/bootchart.tgz with boot chart data

start: start background logging; with PROG, run PROG, then kill logging with USR1
stop: send USR1 to all bootchartd processes
init: start background logging; stop when getty/xdm is seen (for init scripts)
Under PID 1: as init, then exec $bootchart_init, /init, /sbin/init"
  },
  usage_data {
    aname: "halt",
    usage: "\
[-d DELAY] [-n] [-f] [-w]

Halt the system

\t-d SEC\tDelay interval
\t-n\tDo not sync
\t-f\tForce (don\'t go through init)
\t-w\tOnly write a wtmp record"
  },
  usage_data {
    aname: "poweroff",
    usage: "\
[-d DELAY] [-n] [-f]

Halt and shut off power

\t-d SEC\tDelay interval
\t-n\tDo not sync
\t-f\tForce (don\'t go through init)"
  },
  usage_data {
    aname: "reboot",
    usage: "\
[-d DELAY] [-n] [-f]

Reboot the system

\t-d SEC\tDelay interval
\t-n\tDo not sync
\t-f\tForce (don\'t go through init)"
  },
  usage_data {
    aname: "init",
    usage: "\n\nInit is the first process started during boot. It never exits.
It (re)spawns children according to /etc/inittab."
  },
  usage_data {
    aname: "linuxrc",
    usage: "\x08",
  },
  usage_data {
    aname: "nuke",
    usage: "\
DIR...

Remove DIRs",
  },
  usage_data {
    aname: "resume",
    usage: "\
BLOCKDEV [OFFSET]

Restore system state from \'suspend-to-disk\' data in BLOCKDEV",
  },
  usage_data {
    aname: "add-shell",
    usage: "\
SHELL...

Add SHELLs to /etc/shells",
  },
  usage_data {
    aname: "remove-shell",
    usage: "\
SHELL...

Remove SHELLs from /etc/shells",
  },
  usage_data {
    aname: "addgroup",
    usage: "\
[-g GID] [-S] [USER] GROUP

Add a group or add a user to a group

\t-g GID\tGroup id
\t-S\tCreate a system group"
  },
  usage_data {
    aname: "adduser",
    usage: "\
[OPTIONS] USER [GROUP]

Create new user, or add USER to GROUP

\t-h DIR\t\tHome directory
\t-g GECOS\tGECOS field
\t-s SHELL\tLogin shell
\t-G GRP\t\tGroup
\t-S\t\tCreate a system user
\t-D\t\tDon\'t assign a password
\t-H\t\tDon\'t create home directory
\t-u UID\t\tUser id
\t-k SKEL\t\tSkeleton directory (/etc/skel)"
  },
  usage_data {
    aname: "chpasswd",
    usage: "\
[--md5|--encrypted|--crypt-method|--root]

Read user:password from stdin and update /etc/passwd

\t-e,--encrypted\t\tSupplied passwords are in encrypted form
\t-m,--md5\t\tEncrypt using md5, not des
\t-c,--crypt-method ALG\tdes,md5,sha256/512 (default des)
\t-R,--root DIR\t\tDirectory to chroot into"
  },
  usage_data {
    aname: "cryptpw",
    usage: "\
[OPTIONS] [PASSWORD] [SALT]

Print crypt(3) hashed PASSWORD

\t-P,--password-fd N\tRead password from fd N
\t-m,--method TYPE\tdes,md5,sha256/512 (default des)
\t-S,--salt SALT"
  },
  usage_data {
    aname: "mkpasswd",
    usage: "\
[OPTIONS] [PASSWORD] [SALT]

Print crypt(3) hashed PASSWORD

\t-P,--password-fd N\tRead password from fd N
\t-m,--method TYPE\tdes,md5,sha256/512 (default des)
\t-S,--salt SALT"
  },
  usage_data {
    aname: "deluser",
    usage: "\
[--remove-home] USER

Delete USER from the system",
  },
  usage_data {
    aname: "delgroup",
    usage: "\
[USER] GROUP

Delete group GROUP from the system or user USER from group GROUP",
  },
  usage_data {
    aname: "getty",
    usage: "\
[OPTIONS] BAUD_RATE[,BAUD_RATE]... TTY [TERMTYPE]

Open TTY, prompt for login name, then invoke /bin/login

\t-h\t\tEnable hardware RTS/CTS flow control
\t-L\t\tSet CLOCAL (ignore Carrier Detect state)
\t-m\t\tGet baud rate from modem\'s CONNECT status message
\t-n\t\tDon\'t prompt for login name
\t-w\t\tWait for CR or LF before sending /etc/issue
\t-i\t\tDon\'t display /etc/issue
\t-f ISSUE_FILE\tDisplay ISSUE_FILE instead of /etc/issue
\t-l LOGIN\tInvoke LOGIN instead of /bin/login
\t-t SEC\t\tTerminate after SEC if no login name is read
\t-I INITSTR\tSend INITSTR before anything else
\t-H HOST\t\tLog HOST into the utmp file as the hostname

BAUD_RATE of 0 leaves it unchanged"
  },
  usage_data {
    aname: "login",
    usage: "\
[-p] [-h HOST] [[-f] USER]

Begin a new session on the system

\t-f\tDon\'t authenticate (user already authenticated)
\t-h HOST\tHost user came from (for network logins)
\t-p\tPreserve environment"
  },
  usage_data {
    aname: "passwd",
    usage: "\
[OPTIONS] [USER]

Change USER\'s password (default: current user)

\t-a ALG\tdes,md5,sha256/512 (default des)
\t-d\tSet password to \'\'
\t-l\tLock (disable) account
\t-u\tUnlock (enable) account"
  },
  usage_data {
    aname: "su",
    usage: "\
[-lmp] [-] [-s SH] [USER [SCRIPT ARGS / -c \'CMD\' ARG0 ARGS]]

Run shell under USER (by default, root)

\t-,-l\tClear environment, go to home dir, run shell as login shell
\t-p,-m\tDo not set new $HOME, $SHELL, $USER, $LOGNAME
\t-c CMD\tCommand to pass to \'sh -c\'
\t-s SH\tShell to use instead of user\'s default"
  },
  usage_data {
    aname: "sulogin",
    usage: "\
[-t N] [TTY]

Single user login

\t-t N\tTimeout",
  },
  usage_data {
    aname: "vlock",
    usage: "[-a]

        Lock a virtual terminal. A password is required to unlock.

        \t-a\tLock all VTs",
  },
  usage_data {
    aname: "makemime",
    usage: "\
[OPTIONS] [FILE]...

Create multipart MIME-encoded message from FILEs

\t-o FILE\tOutput. Default: stdout
\t-a HDR\tAdd header(s). Examples:
\t\t\"From: user@host.org\", \"Date: `date -R`\"
\t-c CT\tContent type. Default: application/octet-stream
\t-C CS\tCharset. Default: us-ascii

Other options are silently ignored"
  },
  usage_data {
    aname: "popmaildir",
    usage: "\
[OPTIONS] MAILDIR [CONN_HELPER ARGS]

Fetch content of remote mailbox to local maildir

\t-s\t\tSkip authorization
\t-T\t\tGet messages with TOP instead of RETR
\t-k\t\tKeep retrieved messages on the server
\t-t SEC\t\tNetwork timeout
\t-F \'PROG ARGS\'\tFilter program (may be repeated)
\t-M \'PROG ARGS\'\tDelivery program

Fetch from plain POP3 server:
popmaildir -k DIR nc pop3.server.com 110 <user_and_pass.txt
Fetch from SSLed POP3 server and delete fetched emails:
popmaildir DIR -- openssl s_client -quiet -connect pop3.server.com:995 <user_and_pass.txt"
  },
  usage_data {
    aname: "reformime",
    usage: "\
[OPTIONS]

Parse MIME-encoded message on stdin

\t-x PREFIX\tExtract content of MIME sections to files
\t-X PROG ARGS\tFilter content of MIME sections through PROG
\t\t\tMust be the last option

Other options are silently ignored"
  },
  usage_data {
    aname: "sendmail",
    usage: "\
[-tv] [-f SENDER] [-amLOGIN 4<user_pass.txt | -auUSER -apPASS]
\t\t[-w SECS] [-H \'PROG ARGS\' | -S HOST] [RECIPIENT_EMAIL]...

Read email from stdin and send it

Standard options:
\t-t\t\tRead additional recipients from message body
\t-f SENDER\tFor use in MAIL FROM:<sender>. Can be empty string
\t\t\tDefault: -auUSER, or username of current UID
\t-o OPTIONS\tVarious options. -oi implied, others are ignored
\t-i\t\t-oi synonym, implied and ignored

Busybox specific options:
\t-v\t\tVerbose
\t-w SECS\t\tNetwork timeout
\t-H \'PROG ARGS\'\tRun connection helper. Examples:
\t\topenssl s_client -quiet -tls1 -starttls smtp -connect smtp.gmail.com:25
\t\topenssl s_client -quiet -tls1 -connect smtp.gmail.com:465
\t\t\t$SMTP_ANTISPAM_DELAY: seconds to wait after helper connect
\t-S HOST[:PORT]\tServer (default $SMTPHOST or 127.0.0.1)
\t-amLOGIN\tLog in using AUTH LOGIN
\t-amPLAIN\tor AUTH PLAIN
\t\t\t(-amCRAM-MD5 not supported)
\t-auUSER\t\tUsername for AUTH
\t-apPASS \tPassword for AUTH

If no -a options are given, authentication is not done.
If -amLOGIN is given but no -au/-ap, user/password is read from fd #4.
Other options are silently ignored; -oi is implied.
Use makemime to create emails with attachments."
  },
  usage_data {
    aname: "adjtimex",
    usage: "\
[-q] [-o OFF] [-f FREQ] [-p TCONST] [-t TICK]

Read or set kernel time variables. See adjtimex(2)

\t-q\tQuiet
\t-o OFF\tTime offset, microseconds
\t-f FREQ\tFrequency adjust, integer kernel units (65536 is 1ppm)
\t-t TICK\tMicroseconds per tick, usually 10000
\t\t(positive -t or -f values make clock run faster)
\t-p TCONST"
  },
  usage_data {
    aname: "bc",
    usage: "\
[-sqlw] FILE...

Arbitrary precision calculator

\t-q\tQuiet
\t-l\tLoad standard math library
\t-s\tBe POSIX compatible
\t-w\tWarn if extensions are used

$BC_LINE_LENGTH changes output width"
  },
  usage_data {
    aname: "dc",
    usage: "\
[-x] [-eSCRIPT]... [-fFILE]... [FILE]...

Tiny RPN calculator. Operations:
+, -, *, /, %, ~, ^, |,
p - print top of the stack without popping
f - print entire stack
k - pop the value and set the precision
i - pop the value and set input radix
o - pop the value and set output radix
Examples: dc -e\'2 2 + p\' -> 4, dc -e\'8 8 * 2 2 + / p\' -> 16"
  },
  usage_data {
    aname: "beep",
    usage: "\
-f FREQ -l LEN -d DELAY -r COUNT -n

\t-f\tFrequency in Hz
\t-l\tLength in ms
\t-d\tDelay in ms
\t-r\tRepetitions
\t-n\tStart new tone"
  },
  usage_data {
    aname: "chat",
    usage: "\
EXPECT [SEND [EXPECT [SEND...]]]

Useful for interacting with a modem connected to stdin/stdout.
A script consists of \"expect-send\" argument pairs.
Example:
chat \'\' ATZ OK ATD123456 CONNECT \'\' ogin: pppuser word: ppppass \'~\'"
  },
  usage_data {
    aname: "conspy",
    usage: "\
[-vcsndfFQ] [-x COL] [-y LINE] [CONSOLE_NO]

A text-mode VNC like program for Linux virtual consoles.
To exit, quickly press ESC 3 times.

\t-v\tDon\'t send keystrokes to the console
\t-c\tCreate missing /dev/{tty,vcsa}N
\t-s\tOpen a SHELL session
\t-n\tBlack & white
\t-d\tDump console to stdout
\t-f\tFollow cursor
\t-F\tAssume console is on a framebuffer device
\t-Q\tDisable exit on ESC-ESC-ESC
\t-x COL\tStarting column
\t-y LINE\tStarting line"
  },
  usage_data {
    aname: "crond",
    usage: "\
-fbS -l N -d N -L LOGFILE -c DIR

\t-f\tForeground
\t-b\tBackground (default)
\t-S\tLog to syslog (default)
\t-l N\tSet log level. Most verbose 0, default 8
\t-d N\tSet log level, log to stderr
\t-L FILE\tLog to FILE
\t-c DIR\tCron dir. Default:/var/spool/cron/crontabs"
  },
  usage_data {
    aname: "crontab",
    usage: "\
[-c DIR] [-u USER] [-ler]|[FILE]

\t-c\tCrontab directory
\t-u\tUser
\t-l\tList crontab
\t-e\tEdit crontab
\t-r\tDelete crontab
\tFILE\tReplace crontab by FILE (\'-\': stdin)"
  },
  usage_data {
    aname: "devmem",
    usage: "\
ADDRESS [WIDTH [VALUE]]

Read/write from physical address

\tADDRESS\tAddress to act upon
\tWIDTH\tWidth (8/16/...)
\tVALUE\tData to be written"
  },
  usage_data {
    aname: "fbsplash",
    usage: "\
-s IMGFILE [-c] [-d DEV] [-i INIFILE] [-f CMD]

\t-s\tImage
\t-c\tHide cursor
\t-d\tFramebuffer device (default /dev/fb0)
\t-i\tConfig file (var=value):
\t\t\tBAR_LEFT,BAR_TOP,BAR_WIDTH,BAR_HEIGHT
\t\t\tBAR_R,BAR_G,BAR_B,IMG_LEFT,IMG_TOP
\t-f\tControl pipe (else exit after drawing image)
\t\t\tcommands: \'NN\' (% for progress bar) or \'exit\'"
  },
  usage_data {
    aname: "hdparm",
    usage: "\
[OPTIONS] [DEVICE]

\t-a\tGet/set fs readahead
\t-A\tSet drive read-lookahead flag (0/1)
\t-b\tGet/set bus state (0 == off, 1 == on, 2 == tristate)
\t-B\tSet Advanced Power Management setting (1-255)
\t-c\tGet/set IDE 32-bit IO setting
\t-C\tCheck IDE power mode status
\t-d\tGet/set using_dma flag
\t-D\tEnable/disable drive defect-mgmt
\t-f\tFlush buffer cache for device on exit
\t-g\tDisplay drive geometry
\t-h\tDisplay terse usage information
\t-i\tDisplay drive identification
\t-I\tDetailed/current information directly from drive
\t-k\tGet/set keep_settings_over_reset flag (0/1)
\t-K\tSet drive keep_features_over_reset flag (0/1)
\t-L\tSet drive doorlock (0/1) (removable harddisks only)
\t-m\tGet/set multiple sector count
\t-n\tGet/set ignore-write-errors flag (0/1)
\t-p\tSet PIO mode on IDE interface chipset (0,1,2,3,4,...)
\t-P\tSet drive prefetch count
\t-Q\tGet/set DMA tagged-queuing depth (if supported)
\t-r\tGet/set readonly flag (DANGEROUS to set)
\t-R\tRegister an IDE interface (DANGEROUS)
\t-S\tSet standby (spindown) timeout
\t-t\tPerform device read timings
\t-T\tPerform cache read timings
\t-u\tGet/set unmaskirq flag (0/1)
\t-U\tUnregister an IDE interface (DANGEROUS)
\t-v\tDefaults; same as -mcudkrag for IDE drives
\t-V\tDisplay program version and exit immediately
\t-w\tPerform device reset (DANGEROUS)
\t-W\tSet drive write-caching flag (0/1) (DANGEROUS)
\t-x\tTristate device for hotswap (0/1) (DANGEROUS)
\t-X\tSet IDE xfer mode (DANGEROUS)
\t-y\tPut IDE drive in standby mode
\t-Y\tPut IDE drive to sleep
\t-Z\tDisable Seagate auto-powersaving mode
\t-z\tReread partition table"
  },
  usage_data {
    aname: "hexedit",
    usage: "\
FILE

Edit FILE in hexadecimal",
  },
  usage_data {
    aname: "i2cget",
    usage: "\
[-fy] BUS CHIP-ADDRESS [DATA-ADDRESS [MODE]]

Read from I2C/SMBus chip registers

\tI2CBUS\tI2C bus number
\tADDRESS\t0x03-0x77
MODE is:
\tb\tRead byte data (default)
\tw\tRead word data
\tc\tWrite byte/read byte
\tAppend p for SMBus PEC

\t-f\tForce access
\t-y\tDisable interactive mode"
  },
  usage_data {
    aname: "i2cset",
    usage: "\
[-fy] [-m MASK] BUS CHIP-ADDRESS DATA-ADDRESS [VALUE] ... [MODE]

Set I2C registers

\tI2CBUS\tI2C bus number
\tADDRESS\t0x03-0x77
MODE is:
\tc\tByte, no value
\tb\tByte data (default)
\tw\tWord data
\ti\tI2C block data
\ts\tSMBus block data
\tAppend p for SMBus PEC

\t-f\tForce access
\t-y\tDisable interactive mode
\t-r\tRead back and compare the result
\t-m MASK\tMask specifying which bits to write"
  },
  usage_data {
    aname: "i2cdump",
    usage: "\
[-fy] [-r FIRST-LAST] BUS ADDR [MODE]

Examine I2C registers

\tI2CBUS\tI2C bus number
\tADDRESS\t0x03-0x77
MODE is:
\tb\tByte (default)
\tw\tWord
\tW\tWord on even register addresses
\ti\tI2C block
\ts\tSMBus block
\tc\tConsecutive byte
\tAppend p for SMBus PEC

\t-f\tForce access
\t-y\tDisable interactive mode
\t-r\tLimit the number of registers being accessed"
  },
  usage_data {
    aname: "i2cdetect",
    usage: "\
-l | -F I2CBUS | [-ya] [-q|-r] I2CBUS [FIRST LAST]

Detect I2C chips

\t-l\tList installed buses
\t-F BUS#\tList functionalities on this bus
\t-y\tDisable interactive mode
\t-a\tForce scanning of non-regular addresses
\t-q\tUse smbus quick write commands for probing (default)
\t-r\tUse smbus read byte commands for probing
\tFIRST and LAST limit probing range"
  },
  usage_data {
    aname: "i2ctransfer",
    usage: "\
[-fay] I2CBUS {rLENGTH[@ADDR] | wLENGTH[@ADDR] DATA...}...

Read/write I2C data in one transfer

\t-f\tForce access to busy addresses
\t-a\tForce access to non-regular addresses
\t-y\tDisable interactive mode"
  },
  usage_data {
    aname: "less",
    usage: "\
[-EFIMmNSRh~] [FILE]...

View FILE (or stdin) one screenful at a time

\t-E\tQuit once the end of a file is reached
\t-F\tQuit if entire file fits on first screen
\t-I\tIgnore case in all searches
\t-M,-m\tDisplay status line with line numbers
\t\tand percentage through the file
\t-N\tPrefix line number to each line
\t-S\tTruncate long lines
\t-R\tRemove color escape codes in input
\t-~\tSuppress ~s displayed past EOF"
  },
  usage_data {
    aname: "lsscsi",
    usage: "\x08",
  },
  usage_data {
    aname: "makedevs",
    usage: "\
[-d device_table] rootdir

Create a range of special files as specified in a device table.
Device table entries take the form of:
<name> <type> <mode> <uid> <gid> <major> <minor> <start> <inc> <count>
Where name is the file name, type can be one of:
\tf\tRegular file
\td\tDirectory
\tc\tCharacter device
\tb\tBlock device
\tp\tFifo (named pipe)
uid is the user id for the target file, gid is the group id for the
target file. The rest of the entries (major, minor, etc) apply to
to device special files. A \'-\' may be used for blank entries."
  },
  usage_data {
    aname: "man",
    usage: "\
[-aw] MANPAGE...

Display manual page

\t-a\tDisplay all pages
\t-w\tShow page locations

$COLUMNS overrides output width"
  },
  usage_data {
    aname: "microcom",
    usage: "\
[-d DELAY] [-t TIMEOUT] [-s SPEED] [-X] TTY

Copy bytes for stdin to TTY and from TTY to stdout

\t-d\tWait up to DELAY ms for TTY output before sending every
\t\tnext byte to it
\t-t\tExit if both stdin and TTY are silent for TIMEOUT ms
\t-s\tSet serial line to SPEED
\t-X\tDisable special meaning of NUL and Ctrl-X from stdin"
  },
  usage_data {
    aname: "mt",
    usage: "\
[-f device] opcode value

Control magnetic tape drive operation

Available Opcodes:

bsf bsfm bsr bss datacompression drvbuffer eof eom erase
fsf fsfm fsr fss load lock mkpart nop offline ras1 ras2
ras3 reset retension rewind rewoffline seek setblk setdensity
setpart tell unload unlock weof wset"
  },
  usage_data {
    aname: "nandwrite",
    usage: "\
[-np] [-s ADDR] MTD_DEVICE [FILE]

Write to MTD_DEVICE

\t-n\tWrite without ecc
\t-p\tPad to page size
\t-s ADDR\tStart address"
  },
  usage_data {
    aname: "nanddump",
    usage: "\
[-no] [--bb padbad|skipbad] [-s ADDR] [-l LEN] [-f FILE] MTD_DEVICE

Dump MTD_DEVICE

\t-n\tRead without ecc
\t-o\tDump oob data
\t-s ADDR\tStart address
\t-l LEN\tLength
\t-f FILE\tDump to file (\'-\' for stdout)
\t--bb METHOD
\t\tskipbad: skip bad blocks
\t\tpadbad: substitute bad blocks by 0xff (default)"
  },
  usage_data {
    aname: "partprobe",
    usage: "\
DEVICE...

Ask kernel to rescan partition table",
  },
  usage_data {
    aname: "raidautorun",
    usage: "\
DEVICE

Tell the kernel to automatically search and start RAID arrays",
  },
  usage_data {
    aname: "readahead",
    usage: "\
[FILE]...

Preload FILEs to RAM",
  },
  usage_data {
    aname: "runlevel",
    usage: "\
[FILE]

Find the current and previous system runlevel

If no utmp FILE exists or if no runlevel record can be found,
print \"unknown\""
  },
  usage_data {
    aname: "rx",
    usage: "\
FILE

Receive a file using the xmodem protocol",
  },
  usage_data {
    aname: "setfattr",
    usage: "\
[-h] -n|-x ATTR [-v VALUE] FILE...

Set extended attributes

\t-h\t\tDo not follow symlinks
\t-x ATTR\t\tRemove attribute ATTR
\t-n ATTR\t\tSet attribute ATTR to VALUE
\t-v VALUE\t(default: empty)"
  },
  usage_data {
    aname: "setserial",
    usage: "\
[-abGvz] { DEVICE [PARAMETER [ARG]]... | -g DEVICE... }

Print or set serial port parameters

\t-a\tPrint all
\t-b\tPrint summary
\t-G\tPrint as setserial PARAMETERs
\t-v\tVerbose
\t-z\tZero out serial flags before setting
\t-g\tAll args are device names

PARAMETERs: (* = takes ARG, ^ = can be turned off by preceding ^)
\t*port, *irq, *divisor, *uart, *baud_base, *close_delay, *closing_wait,
\t^fourport, ^auto_irq, ^skip_test, ^sak, ^session_lockout, ^pgrp_lockout,
\t^callout_nohup, ^split_termios, ^hup_notify, ^low_latency, autoconfig,
\tspd_normal, spd_hi, spd_vhi, spd_shi, spd_warp, spd_cust
ARG for uart:
\tunknown, 8250, 16450, 16550, 16550A, Cirrus, 16650, 16650V2, 16750,
\t16950, 16954, 16654, 16850, RSA, NS16550A, XSCALE, RM9000, OCTEON, AR7,
\tU6_16550A"
  },
  usage_data {
    aname: "strings",
    usage: "\
[-fo] [-t o/d/x] [-n LEN] [FILE]...

Display printable strings in a binary file

\t-f\t\tPrecede strings with filenames
\t-o\t\tPrecede strings with octal offsets
\t-t o/d/x\tPrecede strings with offsets in base 8/10/16
\t-n LEN\t\tAt least LEN characters form a string (default 4)"
  },
  usage_data {
    aname: "time",
    usage: "\
[-vpa] [-o FILE] PROG ARGS

Run PROG, display resource usage when it exits

\t-v\tVerbose
\t-p\tPOSIX output format
\t-f FMT\tCustom format
\t-o FILE\tWrite result to FILE
\t-a\tAppend (else overwrite)"
  },
  usage_data {
    aname: "ts",
    usage: "\
[-is] [STRFTIME]",
  },
  usage_data {
    aname: "ttysize",
    usage: "\
[w] [h]

Print dimensions of stdin tty, or 80x24",
  },
  usage_data {
    aname: "ubiattach",
    usage: "\
-m MTD_NUM [-d UBI_NUM] [-O VID_HDR_OFF] UBI_CTRL_DEV

Attach MTD device to UBI

\t-m MTD_NUM\tMTD device number to attach
\t-d UBI_NUM\tUBI device number to assign
\t-O VID_HDR_OFF\tVID header offset"
  },
  usage_data {
    aname: "ubidetach",
    usage: "\
-d UBI_NUM UBI_CTRL_DEV

Detach MTD device from UBI

\t-d UBI_NUM\tUBI device number",
  },
  usage_data {
    aname: "ubimkvol",
    usage: "\
-N NAME [-s SIZE | -m] UBI_DEVICE

Create UBI volume

\t-a ALIGNMENT\tVolume alignment (default 1)
\t-m\t\tSet volume size to maximum available
\t-n VOLID\tVolume ID. If not specified,
\t\t\tassigned automatically
\t-N NAME\t\tVolume name
\t-s SIZE\t\tSize in bytes
\t-t TYPE\t\tVolume type (static|dynamic)"
  },
  usage_data {
    aname: "ubirmvol",
    usage: "\
-n VOLID / -N VOLNAME UBI_DEVICE

Remove UBI volume

\t-n VOLID\tVolume ID
\t-N VOLNAME\tVolume name"
  },
  usage_data {
    aname: "ubirsvol",
    usage: "\
-n VOLID -s SIZE UBI_DEVICE

Resize UBI volume

\t-n VOLID\tVolume ID
\t-s SIZE\t\tSize in bytes"
  },
  usage_data {
    aname: "ubiupdatevol",
    usage: "\
-t UBI_DEVICE | [-s SIZE] UBI_DEVICE IMG_FILE

Update UBI volume

\t-t\tTruncate to zero size
\t-s SIZE\tSize in bytes to resize to"
  },
  usage_data {
    aname: "ubirename",
    usage: "UBI_DEVICE OLD_VOLNAME NEW_VOLNAME [OLD2 NEW2]...

        Rename UBI volumes on UBI_DEVICE",
  },
  usage_data {
    aname: "volname",
    usage: "\
[DEVICE]

Show CD volume name of the DEVICE (default /dev/cdrom)",
  },
  usage_data {
    aname: "watchdog",
    usage: "\
[-t N[ms]] [-T N[ms]] [-F] DEV

Periodically write to watchdog device DEV

\t-T N\tReboot after N seconds if not reset (default 60)
\t-t N\tReset every N seconds (default 30)
\t-F\tRun in foreground

Use 500ms to specify period in milliseconds"
  },
  usage_data {
    aname: "modinfo",
    usage: "\
[-adlpn0] [-F keyword] MODULE

\t-a\t\tShortcut for \'-F author\'
\t-d\t\tShortcut for \'-F description\'
\t-l\t\tShortcut for \'-F license\'
\t-p\t\tShortcut for \'-F parm\'
\t-F keyword\tKeyword to look for
\t-0\t\tSeparate output with NULs"
  },
  usage_data {
    aname: "lsmod",
    usage: "\n\nList loaded kernel modules",
  },
  usage_data {
    aname: "modprobe",
    usage: "\
[-rq] MODULE [SYMBOL=VALUE]...

\t-r\tRemove MODULE
\t-q\tQuiet",
  },
  usage_data {
    aname: "depmod",
    usage: "\
[-n]

Generate modules.dep.bb

\t-n\tDry run: print file to stdout",
  },
  usage_data {
    aname: "insmod",
    usage: "\
FILE [SYMBOL=VALUE]...

Load kernel module",
  },
  usage_data {
    aname: "rmmod",
    usage: "\
MODULE...

Unload kernel modules",
  },
  usage_data {
    aname: "arp",
    usage: "\
[-vn]\t[-H HWTYPE] [-i IF] -a [HOSTNAME]
[-v]\t\t    [-i IF] -d HOSTNAME [pub]
[-v]\t[-H HWTYPE] [-i IF] -s HOSTNAME HWADDR [temp]
[-v]\t[-H HWTYPE] [-i IF] -s HOSTNAME HWADDR [netmask MASK] pub
[-v]\t[-H HWTYPE] [-i IF] -Ds HOSTNAME IFACE [netmask MASK] pub

Manipulate ARP cache

\t-a\t\tDisplay (all) hosts
\t-d\t\tDelete ARP entry
\t-s\t\tSet new entry
\t-v\t\tVerbose
\t-n\t\tDon\'t resolve names
\t-i IF\t\tNetwork interface
\t-D\t\tRead HWADDR from IFACE
\t-A,-p AF\tProtocol family
\t-H HWTYPE\tHardware address type"
  },
  usage_data {
    aname: "arping",
    usage: "\
[-fqbDUA] [-c CNT] [-w TIMEOUT] [-I IFACE] [-s SRC_IP] DST_IP

Send ARP requests/replies

\t-f\t\tQuit on first ARP reply
\t-q\t\tQuiet
\t-b\t\tKeep broadcasting, don\'t go unicast
\t-D\t\tExit with 1 if DST_IP replies
\t-U\t\tUnsolicited ARP mode, update your neighbors
\t-A\t\tARP answer mode, update your neighbors
\t-c N\t\tStop after sending N ARP requests
\t-w TIMEOUT\tSeconds to wait for ARP reply
\t-I IFACE\tInterface to use (default eth0)
\t-s SRC_IP\tSender IP address
\tDST_IP\t\tTarget IP address"
  },
  usage_data {
    aname: "brctl",
    usage: "\
COMMAND [BRIDGE [ARGS]]

Manage ethernet bridges
Commands:
\tshow [BRIDGE]...\tShow bridges
\taddbr BRIDGE\t\tCreate BRIDGE
\tdelbr BRIDGE\t\tDelete BRIDGE
\taddif BRIDGE IFACE\tAdd IFACE to BRIDGE
\tdelif BRIDGE IFACE\tDelete IFACE from BRIDGE
\tshowmacs BRIDGE\t\t\tList MAC addresses
\tshowstp\tBRIDGE\t\t\tShow STP info
\tstp BRIDGE 1/yes/on|0/no/off\tSet STP on/off
\tsetageing BRIDGE SECONDS\tSet ageing time
\tsetfd BRIDGE SECONDS\t\tSet bridge forward delay
\tsethello BRIDGE SECONDS\t\tSet hello time
\tsetmaxage BRIDGE SECONDS\tSet max message age
\tsetbridgeprio BRIDGE PRIO\tSet bridge priority
\tsetportprio BRIDGE IFACE PRIO\tSet port priority
\tsetpathcost BRIDGE IFACE COST\tSet path cost"
  },
  usage_data {
    aname: "dnsd",
    usage: "\
[-dvs] [-c CONFFILE] [-t TTL_SEC] [-p PORT] [-i ADDR]

Small static DNS server daemon

\t-c FILE\tConfig file
\t-t SEC\tTTL
\t-p PORT\tListen on PORT
\t-i ADDR\tListen on ADDR
\t-d\tDaemonize
\t-v\tVerbose
\t-s\tSend successful replies only. Use this if you want
\t\tto use /etc/resolv.conf with two nameserver lines:
\t\t\tnameserver DNSD_SERVER
\t\t\tnameserver NORMAL_DNS_SERVER"
  },
  usage_data {
    aname: "ether-wake",
    usage: "\
[-b] [-i IFACE] [-p aa:bb:cc:dd[:ee:ff]/a.b.c.d] MAC

Send a magic packet to wake up sleeping machines.
MAC must be a station address (00:11:22:33:44:55) or
a hostname with a known \'ethers\' entry.

\t-b\t\tBroadcast the packet
\t-i IFACE\tInterface to use (default eth0)
\t-p PASSWORD\tAppend four or six byte PASSWORD to the packet"
  },
  usage_data {
    aname: "ftpd",
    usage: "\
[-wvS] [-a USER] [-t N] [-T N] [DIR]

FTP server. Chroots to DIR, if this fails (run by non-root), cds to it.
Should be used as inetd service, inetd.conf line:
\t21 stream tcp nowait root ftpd ftpd /files/to/serve
Can be run from tcpsvd:
\ttcpsvd -vE 0.0.0.0 21 ftpd /files/to/serve

\t-w\tAllow upload
\t-A\tNo login required, client access occurs under ftpd\'s UID
\t-a USER\tEnable \'anonymous\' login and map it to USER
\t-v\tLog errors to stderr. -vv: verbose log
\t-S\tLog errors to syslog. -SS: verbose log
\t-t,-T N\tIdle and absolute timeout"
  },
  usage_data {
    aname: "ftpget",
    usage: "\
[OPTIONS] HOST [LOCAL_FILE] REMOTE_FILE

Download a file via FTP

\t-c\tContinue previous transfer
\t-v\tVerbose
\t-u USER\tUsername
\t-p PASS\tPassword
\t-P NUM\tPort"
  },
  usage_data {
    aname: "ftpput",
    usage: "\
[OPTIONS] HOST [REMOTE_FILE] LOCAL_FILE

Upload a file to a FTP server

\t-v\tVerbose
\t-u USER\tUsername
\t-p PASS\tPassword
\t-P NUM\tPort number"
  },
  usage_data {
    aname: "dnsdomainname",
    usage: "\x08",
  },
  usage_data {
    aname: "hostname",
    usage: "\
[OPTIONS] [HOSTNAME | -F FILE]

Get or set hostname or DNS domain name

\t-s\tShort
\t-i\tAddresses for the hostname
\t-d\tDNS domain name
\t-f\tFully qualified domain name
\t-F FILE\tUse FILE\'s content as hostname"
  },
  usage_data {
    aname: "httpd",
    usage: "\
[-ifv[v]] [-c CONFFILE] [-p [IP:]PORT] [-u USER[:GRP]] [-r REALM] [-h HOME]
or httpd -d/-e/-m STRING

Listen for incoming HTTP requests

\t-i\t\tInetd mode
\t-f\t\tDon\'t daemonize
\t-v[v]\t\tVerbose
\t-p [IP:]PORT\tBind to IP:PORT (default *:80)
\t-u USER[:GRP]\tSet uid/gid after binding to port
\t-r REALM\tAuthentication Realm for Basic Authentication
\t-h HOME\t\tHome directory (default .)
\t-c FILE\t\tConfiguration file (default {/etc,HOME}/httpd.conf)
\t-m STRING\tMD5 crypt STRING
\t-e STRING\tHTML encode STRING
\t-d STRING\tURL decode STRING"
  },
  usage_data {
    aname: "ifconfig",
    usage: "\
[-a] interface [address]

Configure a network interface

\t[add ADDRESS[/PREFIXLEN]]
\t[del ADDRESS[/PREFIXLEN]]
\t[[-]broadcast [ADDRESS]] [[-]pointopoint [ADDRESS]]
\t[netmask ADDRESS] [dstaddr ADDRESS]
\t[outfill NN] [keepalive NN]
\t[hw ether|infiniband ADDRESS] [metric NN] [mtu NN]
\t[[-]trailers] [[-]arp] [[-]allmulti]
\t[multicast] [[-]promisc] [txqueuelen NN] [[-]dynamic]
\t[mem_start NN] [io_addr NN] [irq NN]
\t[up|down] ..."
  },
  usage_data {
    aname: "ifenslave",
    usage: "\
[-cdf] MASTER_IFACE SLAVE_IFACE...

Configure network interfaces for parallel routing

\t-c\tChange active slave
\t-d\tRemove slave interface from bonding device
\t-f\tForce, even if interface is not Ethernet"
  },
  usage_data {
    aname: "ifplugd",
    usage: "\
[OPTIONS]

Network interface plug detection daemon

\t-n\t\tDon\'t daemonize
\t-s\t\tDon\'t log to syslog
\t-i IFACE\tInterface
\t-f/-F\t\tTreat link detection error as link down/link up
\t\t\t(otherwise exit on error)
\t-a\t\tDon\'t up interface at each link probe
\t-M\t\tMonitor creation/destruction of interface
\t\t\t(otherwise it must exist)
\t-r PROG\t\tScript to run
\t-x ARG\t\tExtra argument for script
\t-I\t\tDon\'t exit on nonzero exit code from script
\t-p\t\tDon\'t run \"up\" script on startup
\t-q\t\tDon\'t run \"down\" script on exit
\t-l\t\tAlways run script on startup
\t-t SECS\t\tPoll time in seconds
\t-u SECS\t\tDelay before running script after link up
\t-d SECS\t\tDelay after link down
\t-m MODE\t\tAPI mode (mii, priv, ethtool, wlan, iff, auto)
\t-k\t\tKill running daemon"
  },
  usage_data {
    aname: "ifup",
    usage: "\
[-anmvf] [-i FILE] IFACE...

\t-a\tConfigure all interfaces
\t-i FILE\tUse FILE instead of /etc/network/interfaces
\t-n\tPrint out what would happen, but don\'t do it
\t\t(note: doesn\'t disable mappings)
\t-m\tDon\'t run any mappings
\t-v\tPrint out what would happen before doing it
\t-f\tForce configuration"
  },
  usage_data {
    aname: "ifdown",
    usage: "\
[-anmvf] [-i FILE] IFACE...

\t-a\tDeconfigure all interfaces
\t-i FILE\tUse FILE for interface definitions
\t-n\tPrint out what would happen, but don\'t do it
\t\t(note: doesn\'t disable mappings)
\t-m\tDon\'t run any mappings
\t-v\tPrint out what would happen before doing it
\t-f\tForce deconfiguration"
  },
  usage_data {
    aname: "inetd",
    usage: "\
[-fe] [-q N] [-R N] [CONFFILE]

Listen for network connections and launch programs

\t-f\tRun in foreground
\t-e\tLog to stderr
\t-q N\tSocket listen queue (default 128)
\t-R N\tPause services after N connects/min
\t\t(default 0 - disabled)
\tDefault CONFFILE is /etc/inetd.conf"
  },
  usage_data {
    aname: "ip",
    usage: "\
[OPTIONS] address|route|link|tunnel|neigh|rule [ARGS]

OPTIONS := -f[amily] inet|inet6|link | -o[neline]

ip addr add|del IFADDR dev IFACE | show|flush [dev IFACE] [to PREFIX]
ip route list|flush|add|del|change|append|replace|test ROUTE
ip link set IFACE [up|down] [arp on|off] [multicast on|off]
\t[promisc on|off] [mtu NUM] [name NAME] [qlen NUM] [address MAC]
\t[master IFACE | nomaster]
ip tunnel add|change|del|show [NAME]
\t[mode ipip|gre|sit] [remote ADDR] [local ADDR] [ttl TTL]
ip neigh show|flush [to PREFIX] [dev DEV] [nud STATE]
ip rule [list] | add|del SELECTOR ACTION"
  },
  usage_data {
    aname: "ipaddr",
    usage: "\
add|del IFADDR dev IFACE | show|flush [dev IFACE] [to PREFIX]

ipaddr add|change|replace|delete dev IFACE IFADDR
\tIFADDR := PREFIX | ADDR peer PREFIX [broadcast ADDR|+|-]
\t\t[anycast ADDR] [label STRING] [scope SCOPE]
\tPREFIX := ADDR[/MASK]
\tSCOPE := [host|link|global|NUMBER]
ipaddr show|flush [dev IFACE] [scope SCOPE] [to PREFIX] [label PATTERN]"
  },
  usage_data {
    aname: "iplink",
    usage: "\
set IFACE [up|down] [arp on|off] [multicast on|off]
\t[promisc on|off] [mtu NUM] [name NAME] [qlen NUM] [address MAC]
\t[master IFACE | nomaster]
iplink add [link IFACE] IFACE [address MAC] type TYPE [ARGS]
iplink delete IFACE type TYPE [ARGS]
\tTYPE ARGS := vlan VLANARGS | vrf table NUM
\tVLANARGS := id VLANID [protocol 802.1q|802.1ad] [reorder_hdr on|off]
\t\t[gvrp on|off] [mvrp on|off] [loose_binding on|off]
iplink show [IFACE]"
  },
  usage_data {
    aname: "iproute",
    usage: "\
list|flush|add|del|change|append|replace|test ROUTE

iproute list|flush SELECTOR
\tSELECTOR := [root PREFIX] [match PREFIX] [proto RTPROTO]
\tPREFIX := default|ADDR[/MASK]
iproute get ADDR [from ADDR iif IFACE]
\t[oif IFACE] [tos TOS]
iproute add|del|change|append|replace|test ROUTE
\tROUTE := NODE_SPEC [INFO_SPEC]
\tNODE_SPEC := PREFIX [table TABLE_ID] [proto RTPROTO] [scope SCOPE] [metric METRIC]
\tINFO_SPEC := NH OPTIONS
\tNH := [via [inet|inet6] ADDR] [dev IFACE] [src ADDR] [onlink]
\tOPTIONS := [mtu [lock] NUM] [advmss [lock] NUM]"
  },
  usage_data {
    aname: "iprule",
    usage: "\
[list] | add|del SELECTOR ACTION

\tSELECTOR := [from PREFIX] [to PREFIX] [tos TOS] [fwmark FWMARK]
\t\t\t[dev IFACE] [pref NUMBER]
\tACTION := [table TABLE_ID] [nat ADDR]
\t\t\t[prohibit|reject|unreachable]
\t\t\t[realms [SRCREALM/]DSTREALM]
\tTABLE_ID := [local|main|default|NUMBER]"
  },
  usage_data {
    aname: "iptunnel",
    usage: "\
add|change|del|show [NAME]
\t[mode ipip|gre|sit] [remote ADDR] [local ADDR] [ttl TTL]

iptunnel add|change|del|show [NAME]
\t[mode ipip|gre|sit] [remote ADDR] [local ADDR]
\t[[i|o]seq] [[i|o]key KEY] [[i|o]csum]
\t[ttl TTL] [tos TOS] [[no]pmtudisc] [dev PHYS_DEV]"
  },
  usage_data {
    aname: "ipneigh",
    usage: "\
show|flush [to PREFIX] [dev DEV] [nud STATE]",
  },
  usage_data {
    aname: "ipcalc",
    usage: "\
[OPTIONS] ADDRESS[/PREFIX] [NETMASK]

Calculate and display network settings from IP address

\t-b\tBroadcast address
\t-n\tNetwork address
\t-m\tDefault netmask for IP
\t-p\tPrefix for IP/NETMASK
\t-h\tResolved host name
\t-s\tNo error messages"
  },
  usage_data {
    aname: "fakeidentd",
    usage: "\
[-fiw] [-b ADDR] [STRING]

Provide fake ident (auth) service

\t-f\tRun in foreground
\t-i\tInetd mode
\t-w\tInetd \'wait\' mode
\t-b ADDR\tBind to specified address
\tSTRING\tIdent answer string (default: nobody)"
  },
  usage_data {
    aname: "nameif",
    usage: "\
[-s] [-c FILE] [IFNAME SELECTOR]...

Rename network interface while it in the down state.
The device matched by SELECTOR is renamed to IFACE.
SELECTOR can be a combination of:
\tdriver=STRING
\tbus=STRING
\tphy_address=NUM
\t[mac=]XX:XX:XX:XX:XX:XX

\t-c FILE\tConfiguration file (default: /etc/mactab)
\t-s\tLog to syslog"
  },
  usage_data {
    aname: "nbd-client",
    usage: "\
{ [-b BLKSIZE] [-N NAME] [-t SEC] [-p] HOST [PORT] | -d } BLOCKDEV

Connect to HOST and provide network block device on BLOCKDEV"
  },
  usage_data {
    aname: "nc",
    usage: "\
[OPTIONS] HOST PORT  - connect
nc [OPTIONS] -l -p PORT [HOST] [PORT]  - listen

\t-e PROG\tRun PROG after connect (must be last)
\t-l\tListen mode, for inbound connects
\t-lk\tWith -e, provides persistent server
\t-p PORT\tLocal port
\t-s ADDR\tLocal address
\t-w SEC\tTimeout for connects and final net reads
\t-i SEC\tDelay interval for lines sent
\t-n\tDon\'t do DNS resolution
\t-u\tUDP mode
\t-v\tVerbose
\t-o FILE\tHex dump traffic
\t-z\tZero-I/O mode (scanning)"
  },
  usage_data {
    aname: "netstat",
    usage: "\
[-ral] [-tuwx] [-enWp]

Display networking information

\t-r\tRouting table
\t-a\tAll sockets
\t-l\tListening sockets
\t\tElse: connected sockets
\t-t\tTCP sockets
\t-u\tUDP sockets
\t-w\tRaw sockets
\t-x\tUnix sockets
\t\tElse: all socket types
\t-e\tOther/more information
\t-n\tDon\'t resolve names
\t-W\tWide display
\t-p\tShow PID/program name for sockets"
  },
  usage_data {
    aname: "nslookup",
    usage: "\
[-type=QUERY_TYPE] [-debug] HOST [DNS_SERVER]

Query DNS about HOST

QUERY_TYPE: soa,ns,a,aaaa,cname,mx,txt,ptr,any"
  },
  usage_data {
    aname: "ntpd",
    usage: "\
[-dnqNwl] [-I IFACE] [-S PROG] [-k KEYFILE] [-p [keyno:N:]PEER]...

NTP client/server

\t-d\tVerbose (may be repeated)
\t-n\tDo not daemonize
\t-q\tQuit after clock is set
\t-N\tRun at high priority
\t-w\tDo not set time (only query peers), implies -n
\t-S PROG\tRun PROG after stepping time, stratum change, and every 11 min
\t-k FILE\tKey file (ntp.keys compatible)
\t-p [keyno:NUM:]PEER
\t\tObtain time from PEER (may be repeated)
\t\tUse key NUM for authentication
\t\tIf -p is not given, \'server HOST\' lines
\t\tfrom /etc/ntp.conf are used
\t-l\tAlso run as server on port 123
\t-I IFACE Bind server to IFACE, implies -l"
  },
  usage_data {
    aname: "ping",
    usage: "\
[OPTIONS] HOST

Send ICMP ECHO_REQUEST packets to network hosts

\t-4,-6\t\tForce IP or IPv6 name resolution
\t-c CNT\t\tSend only CNT pings
\t-s SIZE\t\tSend SIZE data bytes in packets (default 56)
\t-i SECS\t\tInterval
\t-A\t\tPing as soon as reply is recevied
\t-t TTL\t\tSet TTL
\t-I IFACE/IP\tSource interface or IP address
\t-W SEC\t\tSeconds to wait for the first response (default 10)
\t\t\t(after all -c CNT packets are sent)
\t-w SEC\t\tSeconds until ping exits (default:infinite)
\t\t\t(can exit earlier with -c CNT)
\t-q\t\tQuiet, only display output at start
\t\t\tand when finished
\t-p HEXBYTE\tPattern to use for payload"
  },
  usage_data {
    aname: "ping6",
    usage: "\
[OPTIONS] HOST

Send ICMP ECHO_REQUEST packets to network hosts

\t-c CNT\t\tSend only CNT pings
\t-s SIZE\t\tSend SIZE data bytes in packets (default 56)
\t-i SECS\t\tInterval
\t-A\t\tPing as soon as reply is recevied
\t-I IFACE/IP\tSource interface or IP address
\t-q\t\tQuiet, only display output at start
\t\t\tand when finished
\t-p HEXBYTE\tPattern to use for payload"
  },
  usage_data {
    aname: "pscan",
    usage: "\
[-cb] [-p MIN_PORT] [-P MAX_PORT] [-t TIMEOUT] [-T MIN_RTT] HOST

Scan a host, print all open ports

\t-c\tShow closed ports too
\t-b\tShow blocked ports too
\t-p\tScan from this port (default 1)
\t-P\tScan up to this port (default 1024)
\t-t\tTimeout (default 5000 ms)
\t-T\tMinimum rtt (default 5 ms, increase for congested hosts)"
  },
  usage_data {
    aname: "route",
    usage: "\
[{add|del|delete}]

Edit kernel routing tables

\t-n\tDon\'t resolve names
\t-e\tDisplay other/more information
\t-A inet{6}\tSelect address family"
  },
  usage_data {
    aname: "slattach",
    usage: "\
[-ehmLF] [-c SCRIPT] [-s BAUD] [-p PROTOCOL] SERIAL_DEVICE

Configure serial line as SLIP network interface

\t-p PROT\tProtocol: slip, cslip (default), slip6, clisp6, adaptive
\t-s BAUD\tLine speed
\t-e\tExit after initialization
\t-h\tExit if carrier is lost (else never exits)
\t-c PROG\tRun PROG on carrier loss
\t-m\tDo NOT set raw 8bit mode
\t-L\tEnable 3-wire operation
\t-F\tDisable RTS/CTS flow control"
  },
  usage_data {
    aname: "ssl_client",
    usage: "\
[-e] -s FD [-r FD] [-n SNI]",
  },
  usage_data {
    aname: "tc",
    usage: "\
OBJECT CMD [dev STRING]

OBJECT: qdisc|class|filter
CMD: add|del|change|replace|show

qdisc [handle QHANDLE] [root|ingress|parent CLASSID]
\t[[QDISC_KIND] [help|OPTIONS]]
\tQDISC_KIND := [p|b]fifo|tbf|prio|cbq|red|etc.
qdisc show [dev STRING] [ingress]
class [classid CLASSID] [root|parent CLASSID]
\t[[QDISC_KIND] [help|OPTIONS] ]
class show [ dev STRING ] [root|parent CLASSID]
filter [pref PRIO] [protocol PROTO]
\t[root|classid CLASSID] [handle FILTERID]
\t[[FILTER_TYPE] [help|OPTIONS]]
filter show [dev STRING] [root|parent CLASSID]"
  },
  usage_data {
    aname: "tcpsvd",
    usage: "\
[-hEv] [-c N] [-C N[:MSG]] [-b N] [-u USER] [-l NAME] IP PORT PROG

Create TCP socket, bind to IP:PORT and listen for incoming connections.
Run PROG for each connection.

\tIP PORT\t\tIP:PORT to listen on
\tPROG ARGS\tProgram to run
\t-u USER[:GRP]\tChange to user/group after bind
\t-c N\t\tUp to N connections simultaneously (default 30)
\t-b N\t\tAllow backlog of approximately N TCP SYNs (default 20)
\t-C N[:MSG]\tAllow only up to N connections from the same IP:
\t\t\tnew connections from this IP address are closed
\t\t\timmediately, MSG is written to the peer before close
\t-E\t\tDon\'t set up environment
\t-h\t\tLook up peer\'s hostname
\t-l NAME\t\tLocal hostname (else look up local hostname in DNS)
\t-v\t\tVerbose

Environment if no -E:
PROTO=\'TCP\'
TCPREMOTEADDR=\'ip:port\' (\'[ip]:port\' for IPv6)
TCPLOCALADDR=\'ip:port\'
TCPORIGDSTADDR=\'ip:port\' of destination before firewall
\tUseful for REDIRECTed-to-local connections:
\tiptables -t nat -A PREROUTING -p tcp --dport 80 -j REDIRECT --to 8080
TCPCONCURRENCY=num_of_connects_from_this_ip
If -h:
TCPLOCALHOST=\'hostname\' (-l NAME is used if specified)
TCPREMOTEHOST=\'hostname\'"
  },
  usage_data {
    aname: "udpsvd",
    usage: "\
[-hEv] [-c N] [-u USER] [-l NAME] IP PORT PROG

Create UDP socket, bind to IP:PORT and wait for incoming packets.
Run PROG for each packet, redirecting all further packets with same
peer ip:port to it.

\tIP PORT\t\tIP:PORT to listen on
\tPROG ARGS\tProgram to run
\t-u USER[:GRP]\tChange to user/group after bind
\t-c N\t\tUp to N connections simultaneously (default 30)
\t-E\t\tDon\'t set up environment
\t-h\t\tLook up peer\'s hostname
\t-l NAME\t\tLocal hostname (else look up local hostname in DNS)
\t-v\t\tVerbose

Environment if no -E:
PROTO=\'UDP\'
UDPREMOTEADDR=\'ip:port\' (\'[ip]:port\' for IPv6)
UDPLOCALADDR=\'ip:port\'
If -h:
UDPLOCALHOST=\'hostname\' (-l NAME is used if specified)
UDPREMOTEHOST=\'hostname\'"
  },
  usage_data {
    aname: "telnet",
    usage: "\
[-a] [-l USER] HOST [PORT]

Connect to telnet server

\t-a\tAutomatic login with $USER variable
\t-l USER\tAutomatic login as USER"
  },
  usage_data {
    aname: "telnetd",
    usage: "\
[OPTIONS]

Handle incoming telnet connections

\t-l LOGIN\tExec LOGIN on connect
\t-f ISSUE_FILE\tDisplay ISSUE_FILE instead of /etc/issue
\t-K\t\tClose connection as soon as login exits
\t\t\t(normally wait until all programs close slave pty)
\t-p PORT\t\tPort to listen on
\t-b ADDR[:PORT]\tAddress to bind to
\t-F\t\tRun in foreground
\t-i\t\tInetd mode
\t-w SEC\t\tInetd \'wait\' mode, linger time SEC
\t-S\t\tLog to syslog (implied by -i or without -F and -w)"
  },
  usage_data {
    aname: "tftp",
    usage: "\
[OPTIONS] HOST [PORT]

Transfer a file from/to tftp server

\t-l FILE\tLocal FILE
\t-r FILE\tRemote FILE
\t-g\tGet file
\t-p\tPut file
\t-b SIZE\tTransfer blocks of SIZE octets"
  },
  usage_data {
    aname: "tftpd",
    usage: "\
[-crl] [-u USER] [DIR]

Transfer a file on tftp client\'s request

tftpd should be used as an inetd service.
tftpd\'s line for inetd.conf:
\t69 dgram udp nowait root tftpd tftpd -l /files/to/serve
It also can be ran from udpsvd:
\tudpsvd -vE 0.0.0.0 69 tftpd /files/to/serve

\t-r\tProhibit upload
\t-c\tAllow file creation via upload
\t-u\tAccess files as USER
\t-l\tLog to syslog (inetd mode requires this)"
  },
  usage_data {
    aname: "traceroute",
    usage: "\
[-46FIlnrv] [-f 1ST_TTL] [-m MAXTTL] [-q PROBES] [-p PORT]
\t[-t TOS] [-w WAIT_SEC] [-s SRC_IP] [-i IFACE]
\t[-z PAUSE_MSEC] HOST [BYTES]

Trace the route to HOST

\t-4,-6\tForce IP or IPv6 name resolution
\t-F\tSet don\'t fragment bit
\t-I\tUse ICMP ECHO instead of UDP datagrams
\t-l\tDisplay TTL value of the returned packet
\t-n\tPrint numeric addresses
\t-r\tBypass routing tables, send directly to HOST
\t-v\tVerbose
\t-f N\tFirst number of hops (default 1)
\t-m N\tMax number of hops
\t-q N\tNumber of probes per hop (default 3)
\t-p N\tBase UDP port number used in probes
\t\t(default 33434)
\t-s IP\tSource address
\t-i IFACE Source interface
\t-t N\tType-of-service in probe packets (default 0)
\t-w SEC\tTime to wait for a response (default 3)
\t-g IP\tLoose source route gateway (8 max)"
  },
  usage_data {
    aname: "traceroute6",
    usage: "\
[-nrv] [-m MAXTTL] [-q PROBES] [-p PORT]
\t[-t TOS] [-w WAIT_SEC] [-s SRC_IP] [-i IFACE]
\tHOST [BYTES]

Trace the route to HOST

\t-n\tPrint numeric addresses
\t-r\tBypass routing tables, send directly to HOST
\t-v\tVerbose
\t-m N\tMax number of hops
\t-q N\tNumber of probes per hop (default 3)
\t-p N\tBase UDP port number used in probes
\t\t(default 33434)
\t-s IP\tSource address
\t-i IFACE Source interface
\t-t N\tType-of-service in probe packets (default 0)
\t-w SEC\tTime wait for a response (default 3)"
  },
  usage_data {
    aname: "tunctl",
    usage: "\
[-f device] ([-t name] | -d name) [-u owner] [-g group] [-b]

Create or delete tun interfaces

\t-f name\t\ttun device (/dev/net/tun)
\t-t name\t\tCreate iface \'name\'
\t-d name\t\tDelete iface \'name\'
\t-u owner\tSet iface owner
\t-g group\tSet iface group
\t-b\t\tBrief output"
  },
  usage_data {
    aname: "vconfig",
    usage: "\
COMMAND [OPTIONS]

Create and remove virtual ethernet devices

\tadd\t\tIFACE VLAN_ID
\trem\t\tVLAN_NAME
\tset_flag\tIFACE 0|1 VLAN_QOS
\tset_egress_map\tVLAN_NAME SKB_PRIO VLAN_QOS
\tset_ingress_map\tVLAN_NAME SKB_PRIO VLAN_QOS
\tset_name_type\tNAME_TYPE"
  },
  usage_data {
    aname: "wget",
    usage: "\
[-c|--continue] [--spider] [-q|--quiet] [-O|--output-document FILE]
\t[-o|--output-file FILE] [--header \'header: value\'] [-Y|--proxy on/off]
\t[-P DIR] [-S|--server-response] [-U|--user-agent AGENT] [-T SEC] URL...

Retrieve files via HTTP or FTP

\t--spider\tOnly check URL existence: $? is 0 if exists
\t-c\t\tContinue retrieval of aborted transfer
\t-q\t\tQuiet
\t-P DIR\t\tSave to DIR (default .)
\t-S    \t\tShow server response
\t-T SEC\t\tNetwork read timeout is SEC seconds
\t-O FILE\t\tSave to FILE (\'-\' for stdout)
\t-o FILE\t\tLog messages to FILE
\t-U STR\t\tUse STR for User-Agent header
\t-Y on/off\tUse proxy"
  },
  usage_data {
    aname: "whois",
    usage: "\
[-i] [-h SERVER] [-p PORT] NAME...

Query WHOIS info about NAME

\t-i\tShow redirect results too
\t-h,-p\tServer to query"
  },
  usage_data {
    aname: "zcip",
    usage: "\
[OPTIONS] IFACE SCRIPT

Manage a ZeroConf IPv4 link-local address

\t-f\t\tRun in foreground
\t-q\t\tQuit after obtaining address
\t-r 169.254.x.x\tRequest this address first
\t-l x.x.0.0\tUse this range instead of 169.254
\t-v\t\tVerbose

$LOGGING=none\t\tSuppress logging
$LOGGING=syslog \tLog to syslog

With no -q, runs continuously monitoring for ARP conflicts,
exits only on I/O errors (link down etc)"
  },
  usage_data {
    aname: "lpd",
    usage: "\
SPOOLDIR [HELPER [ARGS]]

SPOOLDIR must contain (symlinks to) device nodes or directories
with names matching print queue names. In the first case, jobs are
sent directly to the device. Otherwise each job is stored in queue
directory and HELPER program is called. Name of file to print
is passed in $DATAFILE variable.
Example:
\ttcpsvd -E 0 515 softlimit -m 999999 lpd /var/spool ./print"
  },
  usage_data {
    aname: "lpq",
    usage: "\
[-P queue[@host[:port]]] [-U USERNAME] [-d JOBID]... [-fs]

\t-P\tlp service to connect to (else uses $PRINTER)
\t-d\tDelete jobs
\t-f\tForce any waiting job to be printed
\t-s\tShort display"
  },
  usage_data {
    aname: "lpr",
    usage: "\
-P queue[@host[:port]] -U USERNAME -J TITLE -Vmh [FILE]...

\t-P\tlp service to connect to (else uses $PRINTER)
\t-m\tSend mail on completion
\t-h\tPrint banner page too
\t-V\tVerbose"
  },
  usage_data {
    aname: "free",
    usage: "\
[-b/k/m/g]

Display the amount of free and used system memory",
  },
  usage_data {
    aname: "fuser",
    usage: "\
[OPTIONS] FILE or PORT/PROTO

Find processes which use FILEs or PORTs

\t-m\tFind processes which use same fs as FILEs
\t-4,-6\tSearch only IPv4/IPv6 space
\t-s\tDon\'t display PIDs
\t-k\tKill found processes
\t-SIGNAL\tSignal to send (default: KILL)"
  },
  usage_data {
    aname: "iostat",
    usage: "\
[-c] [-d] [-t] [-z] [-k|-m] [ALL|BLOCKDEV...] [INTERVAL [COUNT]]

Report CPU and I/O statistics

\t-c\tShow CPU utilization
\t-d\tShow device utilization
\t-t\tPrint current time
\t-z\tOmit devices with no activity
\t-k\tUse kb/s
\t-m\tUse Mb/s"
  },
  usage_data {
    aname: "kill",
    usage: "\
[-l] [-SIG] PID...

Send a signal (default: TERM) to given PIDs

\t-l\tList all signal names and numbers"
  },
  usage_data {
    aname: "killall",
    usage: "\
[-l] [-q] [-SIG] PROCESS_NAME...

Send a signal (default: TERM) to given processes

\t-l\tList all signal names and numbers
\t-q\tDon\'t complain if no processes were killed"
  },
  usage_data {
    aname: "killall5",
    usage: "\
[-l] [-SIG] [-o PID]...

Send a signal (default: TERM) to all processes outside current session

\t-l\tList all signal names and numbers
\t-o PID\tDon\'t signal this PID"
  },
  usage_data {
    aname: "lsof",
    usage: "\n\nShow all open files",
  },
  usage_data {
    aname: "mpstat",
    usage: "\
[-A] [-I SUM|CPU|ALL|SCPU] [-u] [-P num|ALL] [INTERVAL [COUNT]]

Per-processor statistics

\t-A\t\t\tSame as -I ALL -u -P ALL
\t-I SUM|CPU|ALL|SCPU\tReport interrupt statistics
\t-P num|ALL\t\tProcessor to monitor
\t-u\t\t\tReport CPU utilization"
  },
  usage_data {
    aname: "nmeter",
    usage: "\
[-d MSEC] FORMAT_STRING

Monitor system in real time

 -d MSEC\tMilliseconds between updates, default:1000, none:-1

Format specifiers:
 %Nc or %[cN]\tCPU. N - bar size (default 10)
\t\t(displays: S:system U:user N:niced D:iowait I:irq i:softirq)
 %[nINTERFACE]\tNetwork INTERFACE
 %m\t\tAllocated memory
 %[mf]\t\tFree memory
 %[mt]\t\tTotal memory
 %s\t\tAllocated swap
 %f\t\tNumber of used file descriptors
 %Ni\t\tTotal/specific IRQ rate
 %x\t\tContext switch rate
 %p\t\tForks
 %[pn]\t\t# of processes
 %b\t\tBlock io
 %Nt\t\tTime (with N decimal points)
 %r\t\tPrint <cr> instead of <lf> at EOL"
  },
  usage_data {
    aname: "pgrep",
    usage: "\
[-flanovx] [-s SID|-P PPID|PATTERN]

Display process(es) selected by regex PATTERN

\t-l\tShow command name too
\t-a\tShow command line too
\t-f\tMatch against entire command line
\t-n\tShow the newest process only
\t-o\tShow the oldest process only
\t-v\tNegate the match
\t-x\tMatch whole name (not substring)
\t-s\tMatch session ID (0 for current)
\t-P\tMatch parent process ID"
  },
  usage_data {
    aname: "pkill",
    usage: "\
[-l|-SIGNAL] [-fnovx] [-s SID|-P PPID|PATTERN]

Send a signal to process(es) selected by regex PATTERN

\t-l\tList all signals
\t-f\tMatch against entire command line
\t-n\tSignal the newest process only
\t-o\tSignal the oldest process only
\t-v\tNegate the match
\t-x\tMatch whole name (not substring)
\t-s\tMatch session ID (0 for current)
\t-P\tMatch parent process ID"
  },
  usage_data {
    aname: "pidof",
    usage: "\
[OPTIONS] [NAME]...

List PIDs of all processes with names that match NAMEs

\t-s\tShow only one PID
\t-o PID\tOmit given pid
\t\tUse %PPID to omit pid of pidof\'s parent"
  },
  usage_data {
    aname: "pmap",
    usage: "\
[-xq] PID...

Display process memory usage

\t-x\tShow details
\t-q\tQuiet",
  },
  usage_data {
    aname: "powertop",
    usage: "\n\nAnalyze power consumption on Intel-based laptops",
  },
  usage_data {
    aname: "ps",
    usage: "\
[-o COL1,COL2=HEADER] [-T]

Show list of processes

\t-o COL1,COL2=HEADER\tSelect columns for display
\t-T\t\t\tShow threads"
  },
  usage_data {
    aname: "pstree",
    usage: "\
[-p] [PID|USER]

Display process tree, optionally start from USER or PID

\t-p\tShow pids"
  },
  usage_data {
    aname: "pwdx",
    usage: "\
PID...

Show current directory for PIDs",
  },
  usage_data {
    aname: "smemcap",
    usage: "\
>SMEMDATA.TAR

Collect memory usage data in /proc and write it to stdout",
  },
  usage_data {
    aname: "sysctl",
    usage: "\
-p [-enq] [FILE...] / [-enqaw] [KEY[=VALUE]]...

Show/set kernel parameters

\t-p\tSet values from FILEs (default /etc/sysctl.conf)
\t-e\tDon\'t warn about unknown keys
\t-n\tDon\'t show key names
\t-q      Quiet
\t-a\tShow all values
\t-w\tSet values"
  },
  usage_data {
    aname: "top",
    usage: "\
[-bmH] [-n COUNT] [-d SECONDS]

Provide a view of process activity in real time.
Read the status of all processes from /proc each SECONDS
and display a screenful of them.
Keys:
\tN/M/P/T: show CPU usage, sort by pid/mem/cpu/time
\tS: show memory
\tR: reverse sort
\tH: toggle threads, 1: toggle SMP
\tQ,^C: exit
Options:
\t-b\tBatch mode
\t-n N\tExit after N iterations
\t-d SEC\tDelay between updates
\t-m\tSame as \'s\' key
\t-H\tShow threads"
  },
  usage_data {
    aname: "uptime",
    usage: "\n\nDisplay the time since the last boot",
  },
  usage_data {
    aname: "watch",
    usage: "\
[-n SEC] [-t] PROG ARGS

Run PROG periodically

\t-n SEC\tLoop period (default 2)
\t-t\tDon\'t print header"
  },
  usage_data {
    aname: "chpst",
    usage: "\
[-vP012] [-u USER[:GRP]] [-U USER[:GRP]] [-e DIR]
\t[-/ DIR] [-n NICE] [-m BYTES] [-d BYTES] [-o N]
\t[-p N] [-f BYTES] [-c BYTES] PROG ARGS

Change the process state, run PROG

\t-u USER[:GRP]\tSet uid and gid
\t-U USER[:GRP]\tSet $UID and $GID in environment
\t-e DIR\t\tSet environment variables as specified by files
\t\t\tin DIR: file=1st_line_of_file
\t-/ DIR\t\tChroot to DIR
\t-n NICE\t\tAdd NICE to nice value
\t-m BYTES\tSame as -d BYTES -s BYTES -l BYTES
\t-d BYTES\tLimit data segment
\t-o N\t\tLimit number of open files per process
\t-p N\t\tLimit number of processes per uid
\t-f BYTES\tLimit output file sizes
\t-c BYTES\tLimit core file size
\t-v\t\tVerbose
\t-P\t\tCreate new process group
\t-0\t\tClose stdin
\t-1\t\tClose stdout
\t-2\t\tClose stderr"
  },
  usage_data {
    aname: "envdir",
    usage: "\
DIR PROG ARGS

Set various environment variables as specified by files
in the directory DIR, run PROG"
  },
  usage_data {
    aname: "envuidgid",
    usage: "\
USER PROG ARGS

Set $UID to USER\'s uid and $GID to USER\'s gid, run PROG",
  },
  usage_data {
    aname: "setuidgid",
    usage: "\
USER PROG ARGS

Set uid and gid to USER\'s uid and gid, drop supplementary group ids,
run PROG"
  },
  usage_data {
    aname: "softlimit",
    usage: "\
[-a BYTES] [-m BYTES] [-d BYTES] [-s BYTES] [-l BYTES]
\t[-f BYTES] [-c BYTES] [-r BYTES] [-o N] [-p N] [-t N]
\tPROG ARGS

Set soft resource limits, then run PROG

\t-a BYTES\tLimit total size of all segments
\t-m BYTES\tSame as -d BYTES -s BYTES -l BYTES -a BYTES
\t-d BYTES\tLimit data segment
\t-s BYTES\tLimit stack segment
\t-l BYTES\tLimit locked memory size
\t-o N\t\tLimit number of open files per process
\t-p N\t\tLimit number of processes per uid
Options controlling file sizes:
\t-f BYTES\tLimit output file sizes
\t-c BYTES\tLimit core file size
Efficiency opts:
\t-r BYTES\tLimit resident set size
\t-t N\t\tLimit CPU time, process receives
\t\t\ta SIGXCPU after N seconds"
  },
  usage_data {
    aname: "runsv",
    usage: "\
DIR

Start and monitor a service and optionally an appendant log service",
  },
  usage_data {
    aname: "runsvdir",
    usage: "\
[-P] [-s SCRIPT] DIR

Start a runsv process for each subdirectory. If it exits, restart it.

\t-P\t\tPut each runsv in a new session
\t-s SCRIPT\tRun SCRIPT <signo> after signal is processed"
  },
  usage_data {
    aname: "sv",
    usage: "\
[-v] [-w SEC] CMD SERVICE_DIR...

Control services monitored by runsv supervisor.
Commands (only first character is enough):

status: query service status
up: if service isn\'t running, start it. If service stops, restart it
once: like \'up\', but if service stops, don\'t restart it
down: send TERM and CONT signals. If ./run exits, start ./finish
\tif it exists. After it stops, don\'t restart service
exit: send TERM and CONT signals to service and log service. If they exit,
\trunsv exits too
pause, cont, hup, alarm, interrupt, quit, 1, 2, term, kill: send
STOP, CONT, HUP, ALRM, INT, QUIT, USR1, USR2, TERM, KILL signal to service"
  },
  usage_data {
    aname: "svc",
    usage: "\
[-udopchaitkx] SERVICE_DIR...

Control services monitored by runsv supervisor

\t-u\tIf service is not running, start it; restart if it stops
\t-d\tIf service is running, send TERM+CONT signals; do not restart it
\t-o\tOnce: if service is not running, start it; do not restart it
\t-pchaitk Send STOP, CONT, HUP, ALRM, INT, TERM, KILL signal to service
\t-x\tExit: runsv will exit as soon as the service is down"
  },
  usage_data {
    aname: "svok",
    usage: "\
SERVICE_DIR

Check whether runsv supervisor is running.
Exit code is 0 if it does, 100 if it does not,
111 (with error message) if SERVICE_DIR does not exist."
  },
  usage_data {
    aname: "svlogd",
    usage: "\
[-tttv] [-r C] [-R CHARS] [-l MATCHLEN] [-b BUFLEN] DIR...

Read log data from stdin and write to rotated log files in DIRs

-r C\t\tReplace non-printable characters with C
-R CHARS\tAlso replace CHARS with C (default _)
-t\t\tTimestamp with @tai64n
-tt\t\tTimestamp with yyyy-mm-dd_hh:mm:ss.sssss
-ttt\t\tTimestamp with yyyy-mm-ddThh:mm:ss.sssss
-v\t\tVerbose

DIR/config file modifies behavior:
sSIZE - when to rotate logs (default 1000000, 0 disables)
nNUM - number of files to retain
!PROG - process rotated log with PROG
+,-PATTERN - (de)select line for logging
E,ePATTERN - (de)select line for stderr"
  },
  usage_data {
    aname: "ash",
    usage: "\
[-/+OPTIONS] [-/+o OPT]... [-c \'SCRIPT\' [ARG0 [ARGS]] / FILE [ARGS] / -s [ARGS]]

Unix shell interpreter"
  },
  usage_data {
    aname: "sh",
    usage: "\
[-/+OPTIONS] [-/+o OPT]... [-c \'SCRIPT\' [ARG0 [ARGS]] / FILE [ARGS] / -s [ARGS]]

Unix shell interpreter"
  },
  usage_data {
    aname: "cttyhack",
    usage: "\
[PROG ARGS]

Give PROG a controlling tty if possible.
Example for /etc/inittab (for busybox init):
\t::respawn:/bin/cttyhack /bin/sh
Giving controlling tty to shell running with PID 1:
\t$ exec cttyhack sh
Starting interactive shell from boot shell script:
\tsetsid cttyhack sh"
  },
  usage_data {
    aname: "hush",
    usage: "\
[-enxl] [-c \'SCRIPT\' [ARG0 [ARGS]] / FILE [ARGS] / -s [ARGS]]

Unix shell interpreter",
  },
  usage_data {
    aname: "klogd",
    usage: "\
[-c N] [-n]

Log kernel messages to syslog

\t-c N\tPrint to console messages more urgent than prio N (1-8)
\t-n\tRun in foreground"
  },
  usage_data {
    aname: "logger",
    usage: "\
[OPTIONS] [MESSAGE]

Write MESSAGE (or stdin) to syslog

\t-s\tLog to stderr as well as the system log
\t-t TAG\tLog using the specified tag (defaults to user name)
\t-p PRIO\tPriority (numeric or facility.level pair)"
  },
  usage_data {
    aname: "logread",
    usage: "\
[-fF]

Show messages in syslogd\'s circular buffer

\t-f\tOutput data as log grows
\t-F\tSame as -f, but dump buffer first"
  },
  usage_data {
    aname: "syslogd",
    usage: "\
[OPTIONS]

System logging utility

\t-n\t\tRun in foreground
\t-R HOST[:PORT]\tLog to HOST:PORT (default PORT:514)
\t-L\t\tLog locally and via network (default is network only if -R)
\t-C[size_kb]\tLog to shared mem buffer (use logread to read it)
\t-K\t\tLog to kernel printk buffer (use dmesg to read it)
\t-O FILE\t\tLog to FILE (default: /var/log/messages, stdout if -)
\t-s SIZE\t\tMax size (KB) before rotation (default 200KB, 0=off)
\t-b N\t\tN rotated logs to keep (default 1, max 99, 0=purge)
\t-l N\t\tLog only messages more urgent than prio N (1-8)
\t-S\t\tSmaller output
\t-t\t\tStrip client-generated timestamps
\t-D\t\tDrop duplicates
\t-f FILE\t\tUse FILE as config (default:/etc/syslog.conf)"
  },
  usage_data {
    aname: "acpid",
    usage: "\
[-df] [-c CONFDIR] [-l LOGFILE] [-a ACTIONFILE] [-M MAPFILE] [-e PROC_EVENT_FILE] [-p PIDFILE]

Listen to ACPI events and spawn specific helpers on event arrival

\t-d\tLog to stderr, not log file (implies -f)
\t-f\tRun in foreground
\t-c DIR\tConfig directory [/etc/acpi]
\t-e FILE\t/proc event file [/proc/acpi/event]
\t-l FILE\tLog file [/var/log/acpid.log]
\t-p FILE\tPid file [/var/run/acpid.pid]
\t-a FILE\tAction file [/etc/acpid.conf]
\t-M FILE Map file [/etc/acpi.map]

Accept and ignore compatibility options -g -m -s -S -v"
  },
  usage_data {
    aname: "blkdiscard",
    usage: "\
[-o OFS] [-l LEN] [-s] DEVICE

Discard sectors on DEVICE

\t-o OFS\tByte offset into device
\t-l LEN\tNumber of bytes to discard
\t-s\tPerform a secure discard"
  },
  usage_data {
    aname: "blkid",
    usage: "\
[BLOCKDEV]...

Print UUIDs of all filesystems",
  },
  usage_data {
    aname: "blockdev",
    usage: "\
OPTION BLOCKDEV

\t--setro\t\tSet ro
\t--setrw\t\tSet rw
\t--getro\t\tGet ro
\t--getss\t\tGet sector size
\t--getbsz\tGet block size
\t--setbsz BYTES\tSet block size
\t--getsz\t\tGet device size in 512-byte sectors
\t--getsize64\tGet device size in bytes
\t--flushbufs\tFlush buffers
\t--rereadpt\tReread partition table"
  },
  usage_data {
    aname: "cal",
    usage: "\
[-jy] [[MONTH] YEAR]

Display a calendar

\t-j\tUse julian dates
\t-y\tDisplay the entire year"
  },
  usage_data {
    aname: "chrt",
    usage: "\
-m | -p [PRIO] PID | [-rfobi] PRIO PROG [ARGS]

Change scheduling priority and class for a process

\t-m\tShow min/max priorities
\t-p\tOperate on PID
\t-r\tSet SCHED_RR class
\t-f\tSet SCHED_FIFO class
\t-o\tSet SCHED_OTHER class
\t-b\tSet SCHED_BATCH class
\t-i\tSet SCHED_IDLE class"
  },
  usage_data {
    aname: "dmesg",
    usage: "\
[-c] [-n LEVEL] [-s SIZE]

Print or control the kernel ring buffer

\t-c\t\tClear ring buffer after printing
\t-n LEVEL\tSet console logging level
\t-s SIZE\t\tBuffer size
\t-r\t\tPrint raw message buffer"
  },
  usage_data {
    aname: "eject",
    usage: "\
[-t] [-T] [DEVICE]

Eject DEVICE or default /dev/cdrom

\t-s\tSCSI device
\t-t\tClose tray
\t-T\tOpen/close tray (toggle)"
  },
  usage_data {
    aname: "fallocate",
    usage: "\
[-o OFS] -l LEN FILE

Preallocate space for FILE

\t-o OFS\tOffset of range
\t-l LEN\tLength of range"
  },
  usage_data {
    aname: "fatattr",
    usage: "\
[-+rhsvda] FILE...

Change file attributes on FAT filesystem

\t-\tClear attributes
\t+\tSet attributes
\tr\tRead only
\th\tHidden
\ts\tSystem
\tv\tVolume label
\td\tDirectory
\ta\tArchive"
  },
  usage_data {
    aname: "fbset",
    usage: "\
[OPTIONS] [MODE]

Show and modify frame buffer settings",
  },
  usage_data {
    aname: "fdformat",
    usage: "\
[-n] DEVICE

Format floppy disk

\t-n\tDon\'t verify after format",
  },
  usage_data {
    aname: "fdisk",
    usage: "\
[-ul] [-C CYLINDERS] [-H HEADS] [-S SECTORS] [-b SSZ] DISK

Change partition table

\t-u\t\tStart and End are in sectors (instead of cylinders)
\t-l\t\tShow partition table for each DISK, then exit
\t-b 2048\t\t(for certain MO disks) use 2048-byte sectors
\t-C CYLINDERS\tSet number of cylinders/heads/sectors
\t-H HEADS\tTypically 255
\t-S SECTORS\tTypically 63"
  },
  usage_data {
    aname: "findfs",
    usage: "\
LABEL=label or UUID=uuid

Find a filesystem device based on a label or UUID",
  },
  usage_data {
    aname: "flock",
    usage: "\
[-sxun] FD|{FILE [-c] PROG ARGS}

[Un]lock file descriptor, or lock FILE, run PROG

\t-s\tShared lock
\t-x\tExclusive lock (default)
\t-u\tUnlock FD
\t-n\tFail rather than wait"
  },
  usage_data {
    aname: "fdflush",
    usage: "\
DEVICE

Force floppy disk drive to detect disk change",
  },
  usage_data {
    aname: "freeramdisk",
    usage: "\
DEVICE

Free all memory used by the specified ramdisk",
  },
  usage_data {
    aname: "fsck.minix",
    usage: "\
[-larvsmf] BLOCKDEV

Check MINIX filesystem

\t-l\tList all filenames
\t-r\tPerform interactive repairs
\t-a\tPerform automatic repairs
\t-v\tVerbose
\t-s\tOutput superblock information
\t-m\tShow \"mode not cleared\" warnings
\t-f\tForce file system check"
  },
  usage_data {
    aname: "fsfreeze",
    usage: "\
--[un]freeze MOUNTPOINT

Flush and halt writes to MOUNTPOINT",
  },
  usage_data {
    aname: "fstrim",
    usage: "\
[OPTIONS] MOUNTPOINT

\t-o,--offset OFFSET\tOffset in bytes to discard from
\t-l,--length LEN\t\tBytes to discard
\t-m,--minimum MIN\tMinimum extent length
\t-v,--verbose\t\tPrint number of discarded bytes"
  },
  usage_data {
    aname: "getopt",
    usage: "\
[OPTIONS] [--] OPTSTRING PARAMS

\t-a\t\tAllow long options starting with single -
\t-l LOPT[,...]\tLong options to recognize
\t-n PROGNAME\tThe name under which errors are reported
\t-o OPTSTRING\tShort options to recognize
\t-q\t\tNo error messages on unrecognized options
\t-Q\t\tNo normal output
\t-s SHELL\tSet shell quoting conventions
\t-T\t\tVersion test (exits with 4)
\t-u\t\tDon\'t quote output

Example:

O=`getopt -l bb: -- ab:c:: \"$@\"` || exit 1
eval set -- \"$O\"
while true; do
\tcase \"$1\" in
\t-a)\techo A; shift;;
\t-b|--bb) echo \"B:\'$2\'\"; shift 2;;
\t-c)\tcase \"$2\" in
\t\t\"\")\techo C; shift 2;;
\t\t*)\techo \"C:\'$2\'\"; shift 2;;
\t\tesac;;
\t--)\tshift; break;;
\t*)\techo Error; exit 1;;
\tesac
done"
  },
  usage_data {
    aname: "hexdump",
    usage: "\
[-bcCdefnosvxR] [FILE]...

Display FILEs (or stdin) in a user specified format

\t-b\t\t1-byte octal display
\t-c\t\t1-byte character display
\t-d\t\t2-byte decimal display
\t-o\t\t2-byte octal display
\t-x\t\t2-byte hex display
\t-C\t\thex+ASCII 16 bytes per line
\t-v\t\tShow all (no dup folding)
\t-e FORMAT_STR\tExample: \'16/1 \"%02x|\"\"\"\'
\t-f FORMAT_FILE
\t-n LENGTH\tShow only first LENGTH bytes
\t-s OFFSET\tSkip OFFSET bytes
\t-R\t\tReverse of \'hexdump -Cv\'"
  },
  usage_data {
    aname: "hd",
    usage: "\
FILE...

hd is an alias for hexdump -C",
  },
  usage_data {
    aname: "xxd",
    usage: "\
[OPTIONS] [FILE]

Hex dump FILE (or stdin)

\t-g N\t\tBytes per group
\t-c N\t\tBytes per line
\t-p\t\tShow only hex bytes, assumes -c30
\t-l LENGTH\tShow only first LENGTH bytes
\t-s OFFSET\tSkip OFFSET bytes"
  },
  usage_data {
    aname: "hwclock",
    usage: "\
[-r|--show] [-s|--hctosys] [-w|--systohc] [--systz] [--localtime] [-u|--utc] [-f|--rtc FILE]

Query and set hardware clock (RTC)

\t-r\tShow hardware clock time
\t-s\tSet system time from hardware clock
\t-w\tSet hardware clock from system time
\t--systz\tSet in-kernel timezone, correct system time
\t\tif hardware clock is in local time
\t-u\tAssume hardware clock is kept in UTC
\t--localtime\tAssume hardware clock is kept in local time
\t-f FILE\tUse specified device (e.g. /dev/rtc2)"
  },
  usage_data {
    aname: "ionice",
    usage: "\
[-c 1-3] [-n 0-7] [-p PID] [PROG]

Change I/O priority and class

\t-c\tClass. 1:realtime 2:best-effort 3:idle
\t-n\tPriority"
  },
  usage_data {
    aname: "ipcrm",
    usage: "\
[-MQS key] [-mqs id]

Upper-case options MQS remove an object by shmkey value.
Lower-case options remove an object by shmid value.

\t-mM\tRemove memory segment after last detach
\t-qQ\tRemove message queue
\t-sS\tRemove semaphore"
  },
  usage_data {
    aname: "ipcs",
    usage: "\
[[-smq] -i SHMID] | [[-asmq] [-tcplu]]

\t-i ID\tShow specific resource
Resource specification:
\t-m\tShared memory segments
\t-q\tMessage queues
\t-s\tSemaphore arrays
\t-a\tAll (default)
Output format:
\t-t\tTime
\t-c\tCreator
\t-p\tPid
\t-l\tLimits
\t-u\tSummary"
  },
  usage_data {
    aname: "last",
    usage: "\
[-HW] [-f FILE]

Show listing of the last users that logged into the system

\t-W\tDisplay with no host column truncation
\t-f FILE Read from FILE instead of /var/log/wtmp"
  },
  usage_data {
    aname: "losetup",
    usage: "\
[-rP] [-o OFS] {-f|LOOPDEV} FILE: associate loop devices
\tlosetup -c LOOPDEV: reread file size
\tlosetup -d LOOPDEV: disassociate
\tlosetup -a: show status
\tlosetup -f: show next free loop device

\t-o OFS\tStart OFS bytes into FILE
\t-P\tScan for partitions
\t-r\tRead-only
\t-f\tShow/use next free loop device"
  },
  usage_data {
    aname: "lspci",
    usage: "\
[-mk]

List all PCI devices

\t-m\tParsable output
\t-k\tShow driver",
  },
  usage_data {
    aname: "lsusb",
    usage: "\x08",
  },
  usage_data {
    aname: "mdev",
    usage: "\
[-s] | [-df]

mdev -s is to be run during boot to scan /sys and populate /dev.
mdev -d[f]: daemon, listen on netlink.
\t-f: stay in foreground.

Bare mdev is a kernel hotplug helper. To activate it:
\techo /sbin/mdev >/proc/sys/kernel/hotplug

It uses /etc/mdev.conf with lines
\t[-][ENV=regex;]...DEVNAME UID:GID PERM [>|=PATH]|[!] [@|$|*PROG]
where DEVNAME is device name regex, @major,minor[-minor2], or
environment variable regex. A common use of the latter is
to load modules for hotplugged devices:
\t$MODALIAS=.* 0:0 660 @modprobe \"$MODALIAS\"

If /dev/mdev.seq file exists, mdev will wait for its value
to match $SEQNUM variable. This prevents plug/unplug races.
To activate this feature, create empty /dev/mdev.seq at boot.

If /dev/mdev.log file exists, debug log will be appended to it."
  },
  usage_data {
    aname: "mesg",
    usage: "\
[y|n]

Control write access to your terminal
\ty\tAllow write access to your terminal
\tn\tDisallow write access to your terminal"
  },
  usage_data {
    aname: "mke2fs",
    usage: "\
[-Fn] [-b BLK_SIZE] [-i INODE_RATIO] [-I INODE_SIZE] [-m RESERVED_PERCENT] [-L LABEL] BLOCKDEV [KBYTES]

\t-b BLK_SIZE\tBlock size, bytes
\t-F\t\tForce
\t-i RATIO\tMax number of files is filesystem_size / RATIO
\t-I BYTES\tInode size (min 128)
\t-L LBL\t\tVolume label
\t-m PERCENT\tPercent of blocks to reserve for admin
\t-n\t\tDry run"
  },
  usage_data {
    aname: "mkfs.ext2",
    usage: "\
[-Fn] [-b BLK_SIZE] [-i INODE_RATIO] [-I INODE_SIZE] [-m RESERVED_PERCENT] [-L LABEL] BLOCKDEV [KBYTES]

\t-b BLK_SIZE\tBlock size, bytes
\t-F\t\tForce
\t-i RATIO\tMax number of files is filesystem_size / RATIO
\t-I BYTES\tInode size (min 128)
\t-L LBL\t\tVolume label
\t-m PERCENT\tPercent of blocks to reserve for admin
\t-n\t\tDry run"
  },
  usage_data {
    aname: "mkfs.minix",
    usage: "\
[-c | -l FILE] [-nXX] [-iXX] BLOCKDEV [KBYTES]

Make a MINIX filesystem

\t-c\t\tCheck device for bad blocks
\t-n [14|30]\tMaximum length of filenames
\t-i INODES\tNumber of inodes for the filesystem
\t-l FILE\t\tRead bad blocks list from FILE
\t-v\t\tMake version 2 filesystem"
  },
  usage_data {
    aname: "mkdosfs",
    usage: "\
[-v] [-n LABEL] BLOCKDEV [KBYTES]

Make a FAT32 filesystem

\t-v\tVerbose
\t-n LBL\tVolume label"
  },
  usage_data {
    aname: "mkfs.vfat",
    usage: "\
[-v] [-n LABEL] BLOCKDEV [KBYTES]

Make a FAT32 filesystem

\t-v\tVerbose
\t-n LBL\tVolume label"
  },
  usage_data {
    aname: "mkswap",
    usage: "\
[-L LBL] BLOCKDEV [KBYTES]

Prepare BLOCKDEV to be used as swap partition

\t-L LBL\tLabel"
  },
  usage_data {
    aname: "more",
    usage: "\
[FILE]...

View FILE (or stdin) one screenful at a time",
  },
  usage_data {
    aname: "mount",
    usage: "\
[OPTIONS] [-o OPT] DEVICE NODE

Mount a filesystem. Filesystem autodetection requires /proc.

\t-a\t\tMount all filesystems in fstab
\t-f\t\tDry run
\t-v\t\tVerbose
\t-r\t\tRead-only mount
\t-t FSTYPE[,...]\tFilesystem type(s)
\t-T FILE\t\tRead FILE instead of /etc/fstab
\t-O OPT\t\tMount only filesystems with option OPT (-a only)
-o OPT:
\tloop\t\tIgnored (loop devices are autodetected)
\t[a]sync\t\tWrites are [a]synchronous
\t[no]atime\tDisable/enable updates to inode access times
\t[no]diratime\tDisable/enable atime updates to directories
\t[no]relatime\tDisable/enable atime updates relative to modification time
\t[no]dev\t\t(Dis)allow use of special device files
\t[no]exec\t(Dis)allow use of executable files
\t[no]suid\t(Dis)allow set-user-id-root programs
\t[r]shared\tConvert [recursively] to a shared subtree
\t[r]slave\tConvert [recursively] to a slave subtree
\t[r]private\tConvert [recursively] to a private subtree
\t[un]bindable\tMake mount point [un]able to be bind mounted
\t[r]bind\t\tBind a file or directory [recursively] to another location
\tmove\t\tRelocate an existing mount point
\tremount\t\tRemount a mounted filesystem, changing flags
\tro\t\tSame as -r

There are filesystem-specific -o flags."
  },
  usage_data {
    aname: "mountpoint",
    usage: "\
[-q] <[-dn] DIR | -x DEVICE>

Check if the directory is a mountpoint

\t-q\tQuiet
\t-d\tPrint major/minor device number of the filesystem
\t-n\tPrint device name of the filesystem
\t-x\tPrint major/minor device number of the blockdevice"
  },
  usage_data {
    aname: "nologin",
    usage: "\n\nPolitely refuse a login",
  },
  usage_data {
    aname: "nsenter",
    usage: "\
[OPTIONS] [PROG [ARGS]]

\t-t PID\t\tTarget process to get namespaces from
\t-m[FILE]\tEnter mount namespace
\t-u[FILE]\tEnter UTS namespace (hostname etc)
\t-i[FILE]\tEnter System V IPC namespace
\t-n[FILE]\tEnter network namespace
\t-p[FILE]\tEnter pid namespace
\t-U[FILE]\tEnter user namespace
\t-S UID\t\tSet uid in entered namespace
\t-G GID\t\tSet gid in entered namespace
\t--preserve-credentials\tDon\'t touch uids or gids
\t-r[DIR]\t\tSet root directory
\t-w[DIR]\t\tSet working directory
\t-F\t\tDon\'t fork before exec\'ing PROG"
  },
  usage_data {
    aname: "pivot_root",
    usage: "\
NEW_ROOT PUT_OLD

Move the current root file system to PUT_OLD and make NEW_ROOT
the new root file system"
  },
  usage_data {
    aname: "rdate",
    usage: "\
[-s/-p] HOST

Set and print time from HOST using RFC 868

\t-s\tOnly set system time
\t-p\tOnly print time"
  },
  usage_data {
    aname: "rdev",
    usage: "\n\nPrint the device node associated with the filesystem mounted at \'/\'",
  },
  usage_data {
    aname: "readprofile",
    usage: "\
[OPTIONS]

\t-m mapfile\t(Default: /boot/System.map)
\t-p profile\t(Default: /proc/profile)
\t-M NUM\t\tSet the profiling multiplier to NUM
\t-i\t\tPrint only info about the sampling step
\t-v\t\tVerbose
\t-a\t\tPrint all symbols, even if count is 0
\t-b\t\tPrint individual histogram-bin counts
\t-s\t\tPrint individual counters within functions
\t-r\t\tReset all the counters (root only)
\t-n\t\tDisable byte order auto-detection"
  },
  usage_data {
    aname: "renice",
    usage: "\
[-n] PRIORITY [[-p | -g | -u] ID...]...

Change scheduling priority of a running process

\t-n\tAdd PRIORITY to current nice value
\t\tWithout -n, nice value is set to PRIORITY
\t-p\tProcess ids (default)
\t-g\tProcess group ids
\t-u\tProcess user names"
  },
  usage_data {
    aname: "rev",
    usage: "\
[FILE]...

Reverse lines of FILE",
  },
  usage_data {
    aname: "rtcwake",
    usage: "\
[-a | -l | -u] [-d DEV] [-m MODE] [-s SEC | -t TIME]

Enter a system sleep state until specified wakeup time

\t-a,--auto\tRead clock mode from adjtime
\t-l,--local\tClock is set to local time
\t-u,--utc\tClock is set to UTC time
\t-d,--device DEV\tSpecify the RTC device
\t-m,--mode MODE\tSet sleep state (default: standby)
\t-s,--seconds SEC Set timeout in SEC seconds from now
\t-t,--time TIME\tSet timeout to TIME seconds from epoch"
  },
  usage_data {
    aname: "script",
    usage: "\
[-afq] [-t[FILE]] [-c PROG] [OUTFILE]

Default OUTFILE is \'typescript\'

\t-a\tAppend output
\t-c PROG\tRun PROG, not shell
\t-q\tQuiet
\t-t[FILE] Send timing to stderr or FILE"
  },
  usage_data {
    aname: "scriptreplay",
    usage: "\
TIMINGFILE [TYPESCRIPT [DIVISOR]]

Play back typescripts, using timing information",
  },
  usage_data {
    aname: "setarch",
    usage: "\
PERSONALITY [-R] PROG ARGS

PERSONALITY may be:
\tlinux32\tSet 32bit uname emulation
\tlinux64\tSet 64bit uname emulation

\t-R\tDisable address space randomization"
  },
  usage_data {
    aname: "linux32",
    usage: "\x08",
  },
  usage_data {
    aname: "linux64",
    usage: "\x08",
  },
  usage_data {
    aname: "setpriv",
    usage: "\
[OPTIONS] PROG [ARGS]

Run PROG with different privilege settings

-d,--dump\t\tShow current capabilities
--nnp,--no-new-privs\tIgnore setuid/setgid bits and file capabilities
--inh-caps CAP,CAP\tSet inheritable capabilities
--ambient-caps CAP,CAP\tSet ambient capabilities"
  },
  usage_data {
    aname: "setsid",
    usage: "\
[-c] PROG ARGS

Run PROG in a new session. PROG will have no controlling terminal
and will not be affected by keyboard signals (^C etc).

\t-c\tSet controlling terminal to stdin"
  },
  usage_data {
    aname: "swapon",
    usage: "\
[-a] [-e] [-d[POL]] [-p PRI] [DEVICE]

Start swapping on DEVICE

\t-a\tStart swapping on all swap devices
\t-d[POL]\tDiscard blocks at swapon (POL=once),
\t\tas freed (POL=pages), or both (POL omitted)
\t-e\tSilently skip devices that do not exist
\t-p PRI\tSet swap device priority"
  },
  usage_data {
    aname: "swapoff",
    usage: "\
[-a] [DEVICE]

Stop swapping on DEVICE

\t-a\tStop swapping on all swap devices",
  },
  usage_data {
    aname: "switch_root",
    usage: "\
[-c CONSOLE_DEV] NEW_ROOT NEW_INIT [ARGS]

Free initramfs and switch to another root fs:
chroot to NEW_ROOT, delete all in /, move NEW_ROOT to /,
execute NEW_INIT. PID must be 1. NEW_ROOT must be a mountpoint.

\t-c DEV\tReopen stdio to DEV after switch"
  },
  usage_data {
    aname: "run-init",
    usage: "\
[-d CAP,CAP...] [-n] [-c CONSOLE_DEV] NEW_ROOT NEW_INIT [ARGS]

Free initramfs and switch to another root fs:
chroot to NEW_ROOT, delete all in /, move NEW_ROOT to /,
execute NEW_INIT. PID must be 1. NEW_ROOT must be a mountpoint.

\t-c DEV\tReopen stdio to DEV after switch
\t-d CAPS\tDrop capabilities
\t-n\tDry run"
  },
  usage_data {
    aname: "taskset",
    usage: "\
[-p] [HEXMASK] PID | PROG ARGS

Set or get CPU affinity

\t-p\tOperate on an existing PID"
  },
  usage_data {
    aname: "uevent",
    usage: "\
[PROG [ARGS]]

uevent runs PROG for every netlink notification.
PROG\'s environment contains data passed from the kernel.
Typical usage (daemon for dynamic device node creation):
\t# uevent mdev & mdev -s"
  },
  usage_data {
    aname: "umount",
    usage: "\
[OPTIONS] FILESYSTEM|DIRECTORY

Unmount file systems

\t-a\tUnmount all file systems
\t-r\tTry to remount devices as read-only if mount is busy
\t-l\tLazy umount (detach filesystem)
\t-f\tForce umount (i.e., unreachable NFS server)
\t-d\tFree loop device if it has been used
\t-t FSTYPE[,...]\tUnmount only these filesystem type(s)"
  },
  usage_data {
    aname: "unshare",
    usage: "\
[OPTIONS] [PROG [ARGS]]

\t-m,--mount[=FILE]\tUnshare mount namespace
\t-u,--uts[=FILE]\t\tUnshare UTS namespace (hostname etc.)
\t-i,--ipc[=FILE]\t\tUnshare System V IPC namespace
\t-n,--net[=FILE]\t\tUnshare network namespace
\t-p,--pid[=FILE]\t\tUnshare PID namespace
\t-U,--user[=FILE]\tUnshare user namespace
\t-f,--fork\t\tFork before execing PROG
\t-r,--map-root-user\tMap current user to root (implies -U)
\t--mount-proc[=DIR]\tMount /proc filesystem first (implies -m)
\t--propagation slave|shared|private|unchanged
\t\t\t\tModify mount propagation in mount namespace
\t--setgroups allow|deny\tControl the setgroups syscall in user namespaces"
  },
  usage_data {
    aname: "wall",
    usage: "\
[FILE]

Write content of FILE or stdin to all logged-in users",
  },
  usage_data {
    aname: "udhcpc6",
    usage: "\
[-fbnqvodR] [-i IFACE] [-r IPv6] [-s PROG] [-p PIDFILE]
\t[-x OPT:VAL]... [-O OPT]...

\t-i IFACE\tInterface to use (default eth0)
\t-p FILE\t\tCreate pidfile
\t-s PROG\t\tRun PROG at DHCP events (default /usr/share/udhcpc/default.script)
\t-B\t\tRequest broadcast replies
\t-t N\t\tSend up to N discover packets
\t-T N\t\tPause between packets (default 3 seconds)
\t-A N\t\tWait N seconds (default 20) after failure
\t-f\t\tRun in foreground
\t-b\t\tBackground if lease is not obtained
\t-n\t\tExit if lease is not obtained
\t-q\t\tExit after obtaining lease
\t-R\t\tRelease IP on exit
\t-S\t\tLog to syslog too
\t-O OPT\t\tRequest option OPT from server (cumulative)
\t-o\t\tDon\'t request any options (unless -O is given)
\t-r IPv6\t\tRequest this address (\'no\' to not request any IP)
\t-d\t\tRequest prefix
\t-l\t\tSend \'information request\' instead of \'solicit\'
\t\t\t(used for servers which do not assign IPv6 addresses)
\t-x OPT:VAL\tInclude option OPT in sent packets (cumulative)
\t\t\tExamples of string, numeric, and hex byte opts:
\t\t\t-x hostname:bbox - option 12
\t\t\t-x lease:3600 - option 51 (lease time)
\t\t\t-x 0x3d:0100BEEFC0FFEE - option 61 (client id)
\t\t\t-x 14:\'\"dumpfile\"\' - option 14 (shell-quoted)
\t-v\t\tVerbose
Signals:
\tUSR1\tRenew lease
\tUSR2\tRelease lease"
  },
  usage_data {
    aname: "udhcpc",
    usage: "\
[-fbqvRB] [-a[MSEC]] [-t N] [-T SEC] [-A SEC/-n]
\t[-i IFACE] [-s PROG] [-p PIDFILE]
\t[-oC] [-r IP] [-V VENDOR] [-F NAME] [-x OPT:VAL]... [-O OPT]...

\t-i IFACE\tInterface to use (default eth0)
\t-s PROG\t\tRun PROG at DHCP events (default /usr/share/udhcpc/default.script)
\t-p FILE\t\tCreate pidfile
\t-B\t\tRequest broadcast replies
\t-t N\t\tSend up to N discover packets (default 3)
\t-T SEC\t\tPause between packets (default 3)
\t-A SEC\t\tWait if lease is not obtained (default 20)
\t-b\t\tBackground if lease is not obtained
\t-n\t\tExit if lease is not obtained
\t-q\t\tExit after obtaining lease
\t-R\t\tRelease IP on exit
\t-f\t\tRun in foreground
\t-S\t\tLog to syslog too
\t-a[MSEC]\tValidate offered address with ARP ping
\t-r IP\t\tRequest this IP address
\t-o\t\tDon\'t request any options (unless -O is given)
\t-O OPT\t\tRequest option OPT from server (cumulative)
\t-x OPT:VAL\tInclude option OPT in sent packets (cumulative)
\t\t\tExamples of string, numeric, and hex byte opts:
\t\t\t-x hostname:bbox - option 12
\t\t\t-x lease:3600 - option 51 (lease time)
\t\t\t-x 0x3d:0100BEEFC0FFEE - option 61 (client id)
\t\t\t-x 14:\'\"dumpfile\"\' - option 14 (shell-quoted)
\t-F NAME\t\tAsk server to update DNS mapping for NAME
\t-V VENDOR\tVendor identifier (default \'udhcp VERSION\')
\t-C\t\tDon\'t send MAC as client identifier
\t-v\t\tVerbose
Signals:
\tUSR1\tRenew lease
\tUSR2\tRelease lease"
  },
  usage_data {
    aname: "udhcpd",
    usage: "\
[-fS] [-I ADDR] [CONFFILE]

DHCP server

\t-f\tRun in foreground
\t-S\tLog to syslog too
\t-I ADDR\tLocal address
\t-a MSEC\tTimeout for ARP ping (default 2000)
Signals:
\tUSR1\tUpdate lease file"
  },
  usage_data {
    aname: "dhcprelay",
    usage: "\
CLIENT_IFACE[,CLIENT_IFACE2]... SERVER_IFACE [SERVER_IP]

Relay DHCP requests between clients and server"
  },
  usage_data {
    aname: "dumpleases",
    usage: "\
[-r|-a] [-d] [-f LEASEFILE]

Display DHCP leases granted by udhcpd

\t-f,--file FILE\tLease file
\t-r,--remaining\tShow remaining time
\t-a,--absolute\tShow expiration time
\t-d,--decimal\tShow time in seconds"
  },];
