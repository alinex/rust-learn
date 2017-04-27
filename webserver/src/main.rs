extern crate iron;
extern crate staticfile;
extern crate mount;

use std::path::Path;

use iron::Iron;
use staticfile::Static;
use mount::Mount;

fn main() {
    let mut mount = Mount::new();

    mount.mount("/", Static::new(Path::new(".")));

    println!("Server running on http://localhost:8080/");

    Iron::new(mount).http("127.0.0.1:8080").unwrap();
}
