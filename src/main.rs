use std::fs::File;
use std::io::{Read, Write};
use std::env::args;

fn main() {
    // Get the path to the file from arguments
    let args: Vec<String> = args().collect();
    let mut path: String = args[1].to_owned();
    
    println!("Path to file: {}", &path);
    
    // Read the file
    let mut f = File::open(&path).unwrap();
    let mut buf: Vec<u8> = vec![0u8; f.metadata().unwrap().len() as usize];
    f.read(&mut buf).expect("Error reading the file!");

    // Change first 16 bytes of the RPG Maker V Picture header to 8 bytes of the PNG header
    let png_head: Vec<u8> = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52];
    let result: Vec<u8> = [&png_head[..], &buf[32..]].concat();

    // Write the new file
    let end_filename = &mut path;
    end_filename.push_str(".png");
    let mut output_file = File::create(&end_filename).unwrap();
    output_file.write_all(&result).expect("Error writing the file!");

    // println!("{:?}", &buf);
    println!("Exported to {}", end_filename);
}
