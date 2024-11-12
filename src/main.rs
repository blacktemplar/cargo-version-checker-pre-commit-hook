use rustc_version::{version, Version};
use std::env;

fn main() {
    let expected_version = env::args().nth(1).unwrap();
    assert_eq!(version().unwrap(), Version::parse(&expected_version).unwrap());
}
