use std::{fs::File, io::{Read, ErrorKind}};

fn main() {
    let mut f = File::open("hello.txt");
    
    let mut f = match f {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
				ErrorKind::NotFound => 
					match File::create("hello.txt") {
						Ok(fileC) => fileC,
						Err(error) => panic!("Problems with file {:?}", error),
					},
				other_error => panic!("Panic! {:?}", other_error),				
			}
        },
    };
    let mut buffer = String::new();
    f.read_to_string(&mut buffer);
    println!("{:?}", buffer);
}
