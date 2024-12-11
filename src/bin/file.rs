use libmagic_rs::{cookie, libmagic_version, Cookie};
use log::*;

pub fn main() {
    println!("libmagic version: {}", libmagic_version());
    let file_flags = cookie::Flags::ERROR;

    let cookie = Cookie::open(file_flags).unwrap();
    let database = [
        "magic.mgc",
        //"/etc/magic.mgc",
        //"/etc/magic",
        //"/usr/share/misc/magic.mgc",
    ]
    .try_into()
    .unwrap();
    let cookie = cookie.load(&database).unwrap();

    let args: Vec<String> = std::env::args().collect();
    let args_count = args.len();

    let mut fpath = "";

    if args_count != 2 {
        println!("Usage ./scanner_cli fname ");
        return;
    } else {
        fpath = &args[1];
    }

    let fp = std::path::Path::new(fpath);
    if !std::path::Path::new(fpath).exists() {
        error!("path not exists");
        return;
    }

    if fp.is_file() {
        match cookie.file(fpath) {
            Ok(result) => {
                println!("{}: {}", &fpath, &result);
            }
            Err(e) => {
                error!("{}: error :{}", fpath, e);
            }
        };
    } else if fp.is_dir() {
        let mut it = walkdir::WalkDir::new(fpath).into_iter();
        loop {
            let entry = match it.next() {
                None => break,
                Some(Err(err)) => {
                    error!("ERROR: {} while scann {}", err, fpath);
                    continue;
                }
                Some(Ok(entry)) => entry,
            };
            if entry.path().is_dir() {
                continue;
            }
            let tfpath = entry.path().to_string_lossy().to_string();

            match cookie.file(&tfpath) {
                Ok(result) => {
                    println!("{}: {}", &tfpath, &result);
                }
                Err(e) => {
                    error!("{}: error :{}", fpath, e);
                }
            };
        }
    }

    println!("[exit] Bye.");
}
