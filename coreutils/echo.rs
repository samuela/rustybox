extern crate getopts;
extern crate regex;
use getopts::Options;
use regex::Regex;
use std::env;

fn print_usage(program: &str, opts: Options) {
  let brief = format!("Usage: {} FILE [options]", program);
  print!("{}", opts.usage(&brief));
}

fn interpret_escapes(backslash_escapes: bool, matches: &getopts::Matches) -> String {
  if !backslash_escapes {
    let mut output = String::new();
    if !matches.free.is_empty() {
      for arg in &matches.free {
        output.push_str(&arg);
        output.push(' ');
      }
    };
    output
  } else {
    let mut output = String::new();
    if !matches.free.is_empty() {
      for arg in &matches.free {
        output = interpret_newline(&arg);
        output = interpret_horizontal_tab(&output);
        output = interpret_vertical_tab(&output);
        output = interpret_no_further_output(&output); // can be first for slight performance improvement
        output = interpret_backspace(&output);
        output = interpret_backslash(&output);
        output = interpret_carriage_return(&output);
        output = interpret_alert_bel(&output);
        output = interpret_form_feed(&output);
      }
    };
    output
  }
}

fn interpret_newline(input: &str) -> String {
  let mut output = String::new();
  let split_items = input.split(r"\n").collect::<Vec<&str>>();
  if split_items.len() == 1 {
    output = input.to_string();
  } else {
    for split_item in split_items {
      output.push_str(&(String::from(split_item) + &String::from("\n")));
    }
  };
  output
}

fn interpret_horizontal_tab(input: &str) -> String {
  let mut output = String::new();
  let split_items = input.split(r"\t").collect::<Vec<&str>>();
  if split_items.len() == 1 {
    output = input.to_string();
  } else {
    for split_item in split_items {
      output.push_str(&(String::from(split_item) + &String::from("\t")));
    }
  };
  output
}

fn interpret_no_further_output(input: &str) -> String {
  match input.find(r"\c") {
    Some(index) => input[..index].to_string(),
    None => input.to_string(),
  }
}

fn interpret_vertical_tab(input: &str) -> String {
  let mut output = String::new();
  let mut spaces_after_newline = 0;
  let split_items = input.split(r"\v").collect::<Vec<&str>>();
  if split_items.len() == 1 {
    output = input.to_string();
  } else {
    for split_item in split_items {
      output.push_str(
        &(String::from(split_item) + &String::from("\n") + &(" ".repeat(spaces_after_newline))),
      );
      spaces_after_newline += split_item.chars().count();
    }
  };
  output
}

fn interpret_form_feed(input: &str) -> String {
  // As of this moment, it seems like form feed for busybox echo does the same thing as vertical tab
  interpret_vertical_tab(input)
}

fn interpret_backspace(input: &str) -> String {
  let mut output = String::from(input);
  let substr_matches: Vec<_> = input.match_indices(r"\b").collect();
  if substr_matches.len() > 1 {
    let mut match_indexes: Vec<usize> = Vec::new();
    for substr_match in substr_matches {
      match_indexes.push(substr_match.0);
    }
    // (Edge) Case 1: '\b' is the first character, do nothing
    if match_indexes[0] == 0 {
      output = output.split_off(1);
      match_indexes.remove(0);
    }

    // (Edge) Case 2: '\b' is the last character, do nothing
    if (match_indexes.pop().unwrap() + 1) == input.chars().count() {
      output.split_at(match_indexes.pop().unwrap());
    }

    // Default Case
    for match_index in match_indexes {
      output.remove(match_index - 1); // remove the character before '\b'
      output.remove(match_index - 1); // remove '\'
      output.remove(match_index - 1); // remove 'b'
    }
  }
  output
}

fn interpret_backslash(input: &str) -> String {
  input.replace(r"\\", r"\")
}

fn interpret_carriage_return(input: &str) -> String {
  if input.contains(r"\r") {
    let mut output_len: usize = 0;
    for split_item in input.split(r"\r").collect::<Vec<&str>>() {
      if split_item.chars().count() > output_len {
        output_len = split_item.chars().count();
      }
    }
    let mut output: Vec<char> = Vec::with_capacity(output_len);
    for split_item in input.split(r"\r").collect::<Vec<&str>>() {
      for i in 0..(split_item.chars().count() - 1) {
        output[i] = split_item.chars().nth(i).unwrap();
      }
    }
    output.into_iter().collect()
  } else {
    input.to_string()
  }
}

fn interpret_alert_bel(input: &str) -> String {
  if input.contains(r"\a") {
    print!("{}", "\x07".repeat(input.match_indices(r"\a").count())); // sound the bell
    input.replace(r"\a", "")
  } else {
    input.to_string()
  }
}

fn interpret_octal_value(input: &str) -> String {
  let re = Regex::new(r"\0\d{0,3}").unwrap();
  if re.is_match(input) {
    let matches = re.find_iter(input);
    unimplemented!();
  } else {
    input.to_string()
  }
}

fn interpret_hex_value(input: &str) -> String {
  let re = Regex::new(r"\x[0-9&&[A-F]]{0,2}").unwrap();
  if re.is_match(input) {
    let matches = re.find_iter(input);
    unimplemented!();
  } else {
    input.to_string()
  }
}

pub fn echo_main(_argc: i32, _argv: *mut *mut i8) -> i32 {
  let mut trailing_newline: bool = false;
  let mut backslash_escapes: bool = false;
  let args: Vec<String> = env::args().collect();
  let program = args[0].clone();

  let mut opts = Options::new();
  opts.optopt("n", "", "suppress trailing newline", "");
  opts.optopt("e", "", "Interpret backslash escapes (i.e. \t=tab", "");
  opts.optopt("E", "", "Don't interpret backslash escapes (default)", "");
  opts.optflag("h", "help", "print this help menu");
  let matches = match opts.parse(&args[1..]) {
    Ok(m) => m,
    Err(f) => panic!(f.to_string()),
  };

  if matches.opt_present("h") {
    print_usage(&program, opts);
  };

  if matches.opt_present("E") {
    backslash_escapes = false;
  }

  if matches.opt_present("e") {
    backslash_escapes = true;
  }

  if matches.opt_present("n") {
    trailing_newline = true;
  };

  let output = interpret_escapes(backslash_escapes, &matches);

  if trailing_newline {
    print!("{}", output)
  } else {
    println!("{}", output);
  };

  return 1;
}
