use libmagic_rs::{cookie, libmagic_version, Cookie};
use std::env;

pub fn main() {
    println!("libmagic version: {}", libmagic_version());
    let cookie = Cookie::open(
        cookie::Flags::ERROR | cookie::Flags::DEBUG,
        // | cookie::Flags::APPLE cookie::Flags::CONTINUE | cookie::Flags::MIME,
    )
    .unwrap();
    let database = [
        "magic.mgc",
        //"/etc/magic.mgc",
        //"/etc/magic",
        //"/usr/share/misc/magic.mgc",
    ]
    .try_into()
    .unwrap();
    let cookie = cookie.load(&database).unwrap();

    let target = "test.php";
    println!("{}", cookie.file(target).unwrap());

    println!("safe exit");
}
