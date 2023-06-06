use std::fs::OpenOptions;
use std::io::Write;
use std::io::ErrorKind;
pub fn open_or_create(file: &str, content: &str) {
    
    let mut file = match OpenOptions::new().write(true).create(true).open(file) {
        Ok(file) => file,  
        Err(ref e) if e.kind() == ErrorKind::AlreadyExists => {
            println!("File already exists");  
            return; 
        },
        Err(e) => panic!("Error while opening file: {:?}", e),  
    };

    match file.write_all(content.as_bytes()) {
        Ok(_) => println!("File created"),  
        Err(e) => panic!("Error while writing to file: {:?}", e),  
    }
}