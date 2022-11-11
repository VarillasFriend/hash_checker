use hash_checker::*;
use std::path::PathBuf;

fn main() {
    let mut args = std::env::args();
    args.next();

    let path1 = match args.next() {
        Some(arg) => arg,
        None => panic!("Didn't get path")
    };
    let path2 = match args.next() {
        Some(arg) => arg,
        None => panic!("Didn't get path")
    };
    let use_names = match args.next() {
        Some(arg) => {
            arg == String::from("--use-names")
        },
        None => false
    };

    let path1 = PathBuf::from(&path1);
    let path2 = PathBuf::from(&path2);

    let result = match use_names {
        true => compare_folders_with_names(path1, path2),
        false => compare_folders(path1, path2),
    };

    println!("{}", result);
}
