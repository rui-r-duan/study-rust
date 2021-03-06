#![allow(unused)]
fn main() {
    use std::env;

    let key = "PATH";
    match env::var(key) {
	Ok(val) => println!("{}: {:?}", key, val),
	Err(e) => println!("couldn't interpret {}: {}", key, e),
    }
}
