use std::env;

pub fn main() {
    let mut output = String::new();
    let args: Vec<String> = env::args().collect();
    let options: [String; 1] = ["--n".to_owned()];

    let trailing_newline = args[0] != options[0];
    

    for arg in &args[1..] {
        output.push_str(arg);
        output.push(' ');
    }

    if trailing_newline {
        print!("{}", output)
    } else{
        println!("{}", output);
    };
}