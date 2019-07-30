use reverse::reverse::reverse;
use std::io;
use std::io::Write;
use std::process::exit;

fn main() {
    let mut exp = String::new();

    print!("Please write a expression: ");

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut exp).unwrap();
    let result = reverse(&exp);
    match result {
        Ok(n) => println!("Result: {}", n),
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
    };
}
