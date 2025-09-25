use std::io::{self, Write};
fn main() {
    println!("간단 계산기. 종료시 빈 줄 입력 후 Enter:");
    loop {
        print!("> ");
        io::stdout().flush().expect("stdout flush failed");

        let mut buf = String::new();
        let n = io::stdin().read_line(&mut buf).expect("stdin read failed");

        if n == 0 || buf.trim().is_empty() {
            println!("bye");
            break;
        }

        match rust_practice_calc_cli::compute(buf.trim()) {
            Ok(result) => println!("{}", result),
            Err(err) => eprintln!("{}", err),
        }
    }
}
