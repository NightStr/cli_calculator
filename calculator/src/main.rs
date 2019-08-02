use reverse;
use std::io;
use std::io::Write;
use std::process::exit;

fn main() {
    let mut exp = String::new();

    print!("Please write a expression: ");

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut exp).unwrap();
    let result = reverse::eval(&exp.trim().to_string());

    match result {
        Ok(result) => {
            println!("Result {}", result);
        }
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    }
}
