use std::env;
use std::io::{self, Write};
use std::process;

fn main() {
    let output = process::Command::new("touch")
        .args(env::args())
        .output()
        .expect("unexpected grass");

    if output.status.success() {
        println!("Touching grass...");
    } else {
        println!("Touching grass not possible!");
    }

    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    process::exit(output.status.code().unwrap());
}
