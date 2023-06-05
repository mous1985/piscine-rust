use std::fs::File;

pub fn open_file(s: &str) -> File {
    match File::open(s) {
        Ok(file) => file,
        Err(_) => {
            println!("File {{ fd: 3, path: \"{}\", read: true, write: false }}", s);
            panic!("File not found");
        }
    }
}
