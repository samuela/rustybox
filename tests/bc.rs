mod common;
use common::exe;
use duct::cmd;
use std::io::Write;

fn simple_test(stdin: &str, expected_stdout: &str) {
  let stdout = cmd!(exe(), "bc").stdin_bytes(stdin).read().unwrap();
  assert_eq!(stdout, expected_stdout);
}

fn file_test(file: &str, stdin: &str, expected_stdout: &str) {
  let mut input_file = tempfile::NamedTempFile::new().unwrap();
  writeln!(input_file, "{}", file).unwrap();
  let input_file_path = input_file
    .path()
    .canonicalize()
    .unwrap()
    .to_str()
    .unwrap()
    .to_string();
  let stdout = cmd!(exe(), "bc", input_file_path)
    .stdin_bytes(stdin)
    .read()
    .unwrap();
  assert_eq!(stdout, expected_stdout);
}

#[test]
fn comment() {
  simple_test("1 /* comment */ + 2", "3")
}

#[test]
fn slash_star_slash() {
  simple_test("1 /*/ + 2 */ + 3", "4")
}

#[test]
fn comment_with_double_quote() {
  simple_test("1 /* \" */ + 2", "3")
}

#[test]
fn weird_string_comment() {
  simple_test("\"string/*\";9", "string/*9")
}

#[test]
fn unterminated_hash_comment() {
  simple_test("#foo", "")
}

#[test]
fn backslash() {
  simple_test("1 \\\n + 2", "3")
}

#[test]
fn string() {
  simple_test("\"STR\n\"", "STR")
}

#[test]
fn read_four() {
  file_test("read();halt", "4", "4")
}

#[test]
fn read_squared() {
  file_test("read()^2;halt", "4", "16")
}

#[test]
fn read_multiply_read() {
  file_test("read()*read();halt", "4\n5", "20")
}

#[test]
fn if_0_else() {
  simple_test("if (0) 1 else 2; 9", "2\n9")
}

#[test]
fn if_1_else() {
  simple_test("if (1) 1 else 2; 9", "1\n9")
}

#[test]
fn if_1_if_1_else_else() {
  simple_test("if (1) if (1) 1 else 2 else 3; 9", "1\n9")
}

#[test]
fn if_0_else_if_1() {
  simple_test("if (0) 1 else if (1) 2; 9", "2\n9")
}

#[test]
fn for_double_semicolon() {
  simple_test("i=2; for (;;) { 2; if(--i==0) break; 3; }; 9", "2\n3\n2\n9")
}

#[test]
fn for_semicolon_cond_semicolon() {
  simple_test("i=0; for(;i<3;)++i; 9", "1\n2\n3\n9")
}

#[test]
fn for_no_init() {
  simple_test("i=1; for(;i<4;i++)i; 9", "1\n2\n3\n9")
}

#[test]
fn for_full() {
  simple_test("for(i=1;i<4;i++)i; 9", "1\n2\n3\n9")
}

#[test]
fn for_double_semicolon2() {
  simple_test("for (;;) {2;break}; 9", "2\n9")
}

#[test]
fn define_return() {
  simple_test("define w() {return}\nw();9", "0\n9")
}

#[test]
fn define_auto() {
  simple_test("define w() { auto z; return 8; }; w(); 9", "8\n9")
}

#[test]
fn define_auto_array_same_name() {
  simple_test("define w(x) { auto x[]; return x; }; w(8); 9", "8\n9")
}

#[test]
fn define_with_body_on_next_line() {
  simple_test("define w()\n{ auto z; return 8; }\nw()\n9", "8\n9")
}

#[test]
fn void_function() {
  simple_test("define void w() {print \"void\"}\nw()\n9", "void9")
}

// Extra POSIX compat - GNU bc does not allow this
#[test]
fn function_named_void() {
  simple_test("define void() {print \"void\"}\nvoid()\n9", "void0\n9")
}

// Extra POSIX compat - GNU bc does not allow this
#[test]
fn variable_named_void() {
  simple_test("void=6\nvoid\n9", "6\n9")
}

#[test]
fn if_cond_newline() {
  simple_test("if(0)\n3\n9", "9")
}

#[test]
fn if_cond_stmt_else_newline() {
  simple_test("if(0)3 else\n4\n9", "4\n9")
}

#[test]
fn while_cond_newline() {
  simple_test("i=9;while(--i)\ni\n9", "8\n7\n6\n5\n4\n3\n2\n1\n9")
}

#[test]
fn ifz_does_not_match_if_keyword() {
  simple_test("ifz=1;ifz\n++ifz;ifz++\nifz", "1\n2\n2\n3")
}

// had parse error on "f()-N"
#[test]
fn long_print() {
  let stdout = cmd!(exe(), "bc", "-l")
    .stdin_bytes("e(0)-2")
    .read()
    .unwrap();
  assert_eq!(stdout, "-1.00000000000000000000");
}

#[test]
fn not_a_and_b() {
  simple_test("(!a&&b)", "0")
}

// check that dc code is not messing this up (no NUL printing!)
#[test]
fn print_empty_string() {
  simple_test("print \"\"", "")
}

#[test]
fn print_123() {
  simple_test("print 1,2,3", "123")
}

#[test]
fn print_1_bracketed() {
  simple_test("{ print 1 }", "1")
}

#[test]
fn nested_loops_and_breaks() {
  simple_test(
    "\
if(1) {
    11
    while(1) {
        21
        while(1) {
            31
            break
            32
        }
        22
        break
        23
    }
    12
} else {
    88
}
99
  ",
    "\
11
21
31
22
12
99",
  )
}

#[test]
fn continue_in_if() {
  simple_test(
    "\
i=2
while(i--) {
	11
	if(i) {
		21
		continue
		22
	} else {
		31
		continue
	32
	}
	12
}
99
",
    "\
11
21
11
31
99",
  )
}

#[test]
fn continue_in_for() {
  simple_test(
    "\
for(i=1; i<3; i++) {
    i
    if(i==2) continue
    77
}
99
",
    "\
1
77
2
99",
  )
}

#[test]
fn ibase() {
  simple_test("a=ZZ;a;ibase=36;a=ZZ;a;ibase=Z;a=ZZ;a", "99\n1295\n1224")
}

#[test]
fn parsing_of_numbers() {
  let code = "\
for (b = 2; b <= 16; ++b) {
    if (b == 10) continue
    obase = 10
    print \"ibase = A; ibase = \", b, \"\n\"
    obase = b
    for (i = 0; i <= 65536; ++i) {
        i
        print \"0.\", i, \"\n\"
        print \"1.\", i, \"\n\"
        print i, \".\", i, \"\n\"
    }
}
";
  let stdout = cmd!(exe(), "bc")
    .stdin_bytes(code)
    .stderr_to_stdout()
    .pipe(cmd!(exe(), "bc").stderr_to_stdout())
    .pipe(cmd!(exe(), "md5sum").stderr_to_stdout())
    .read()
    .unwrap();
  assert_eq!(stdout, "465d8c01308d0863b6f5669e8a1c69fb  -");
}

#[test]
fn printing_of_numbers() {
  let code = "\
for (b = 2; b <= 101; ++b) {
	if (b == 10) continue
	s = b * b
	print \"obase = \", b, \"\n\"
	for (i = 0; i <= s; ++i) {
		i
		print \"0.\", i, \"\n\"
		print \"1.\", i, \"\n\"
		print i, \".\", i, \"\n\"
	}
	2189432174861923048671023498128347619023487610234689172304.192748960128745108927461089237469018723460
}
";
  let stdout = cmd!(exe(), "bc")
    .stdin_bytes(code)
    .stderr_to_stdout()
    .pipe(cmd!(exe(), "bc").stderr_to_stdout())
    .pipe(cmd!(exe(), "md5sum").stderr_to_stdout())
    .read()
    .unwrap();
  assert_eq!(stdout, "d884b35d251ca096410712743aeafb9e  -");
}
