extern crate getopts;
use getopts::Options;
use std::env;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

pub fn echo_main(_argc: i32, _argv: *mut *mut i8) -> i32 {
    let mut output = String::new();
    let mut trailing_newline: bool = false;
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("n", "", "suppress trailing newline", "");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string())}
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
    };

    if matches.opt_present("n") {
        trailing_newline = true;
    };
    
    if !matches.free.is_empty() {
        for arg in matches.free {
            output.push_str(&arg);
            output.push(' ');
        }
    };

    if trailing_newline {
        print!("{}", output)
    } else{
        println!("{}", output);
    };

    return 1;
}