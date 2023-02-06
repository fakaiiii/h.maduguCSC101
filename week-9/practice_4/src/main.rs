use std::fs::OpenOPtions;
use std::io::write;

fn main() {

    let mut file = OpenOPtions::new().append(true).open("data.txt").expect(
        "cannot open file");
    file.write_all("\nHello Class".as_bytes()).expect("write failed");
    file.write_all("\nThis is the appendage to the document."
        .as_bytes()).expect("write failed");
    println!("file append success");    
}