use rustc_version::version;

fn main() {
    println!("rustc version: {}", version().unwrap());
}
