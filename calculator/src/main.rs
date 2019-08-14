use reverse;
use std::io;
use std::io::Write;
use std::process::exit;

fn main() {
    let exit_token = "0";
    loop {
        let mut exp = String::new();
        print!("0 to exit >>> ");

        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut exp).unwrap();
        let trimmed_exp = exp.trim().to_string();
        if trimmed_exp == exit_token {
            exit(0)
        }
        let result = reverse::eval(&trimmed_exp.to_string());

        match result {
            Ok(result) => {
                println!("{}", result);
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }
}
