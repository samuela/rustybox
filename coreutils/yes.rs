use std::io::Write;

pub fn yes_main(args: &[&str]) -> ! {
  let line = if args.len() > 1 {
    args[1..].join(" ")
  } else {
    "y".to_string()
  };

  // This nonsense is necessary in order to prevent a panic with things like
  // `yes | head`. See https://github.com/BurntSushi/advent-of-code/issues/17.
  loop {
    writeln!(std::io::stdout(), "{}", line).unwrap_or_else(|_| std::process::exit(1));
  }
}
