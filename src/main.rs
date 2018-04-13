use std::process::exit;
use std::env;
use std::fs::File;
use std::io;
use std::io::{BufReader, Read, BufWriter, Write};
use std::str::FromStr;

fn main() {
    let (file, filename, key, encrypted) = match get_args() {
        Ok(args) => args,
        Err(e) => return standard_io_error_handling(e),
    };
    if encrypted {
        match decrypt_and_save(file, filename, key) {
            Ok(_) => println!("Success! You can find the decrypted file in the same directory with the original name."),
            Err(e) => standard_io_error_handling(e),
        }
    } else {
        match encrypt_and_save(file, filename, key) {
            Ok(_) => {
                println!("Success! You can find the encrypted file in the same directory with a .encrypt suffix");
                println!("Advise: You can now test to decrypt the file again and then should delete the both unencrypted files.");
                println!("Otherwise you whould have just wasted your time and no one wants to waist their time.");
            }
            Err(e) => standard_io_error_handling(e),
        }
    }
}

fn get_args() -> Result<(File, String, u8, bool), io::Error> {
    let file = match env::args().nth(1) {
        Some(f) => f,
        None => return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "No argument given")),
    };
    let key = match env::args().nth(2) {
        Some(k) => match u8::from_str(&k) {
            Ok(k) => k,
            Err(e) => return Err(io::Error::new(io::ErrorKind::Other, format!("{}", e))),
        }
        None => return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "No key given")),
    };
    let mut encrypted = false;
    if file.ends_with(".encrypt") {
        encrypted = true;
    }
    Ok((File::open(&file)?, file, key, encrypted))
}

fn encrypt_and_save(f: File, mut name: String, key: u8) -> Result<(), io::Error> {
    let buf_reader = BufReader::new(f);
    name.push_str(".encrypt");
    match check_file_existance(&name) {
        Some(s) => name = s,
        None => (),
    }
    let mut buffer = BufWriter::new(File::create(name)?);
    for byte in buf_reader.bytes() {
        buffer.write(&[encrypt(byte?, key)])?;
    }
    buffer.flush()?;
    Ok(())
}

fn decrypt_and_save(f: File, mut name: String, key: u8) -> Result<(), io::Error> {
    let buf_reader = BufReader::new(f);
    name = name.trim_right_matches(".encrypt").to_string();
    println!("{}", name);
    match check_file_existance(&name) {
        Some(s) => name = s,
        None => (),
    }
    let mut buffer = BufWriter::new(File::create(name)?);
    for byte in buf_reader.bytes() {
        buffer.write(&[decrypt(byte?, key)])?;
    }
    buffer.flush()?;
    Ok(())
}

fn encrypt(byte: u8, key: u8) -> u8 {
    byte.wrapping_add(key)
}

fn decrypt(byte: u8, key: u8) -> u8 {
    byte.wrapping_sub(key)
}

fn check_file_existance(name: &String) -> Option<String> {
    match File::open(&name) {
        Ok(_) => {
            loop {
                let mut input = String::new();
                println!("The file {} already exists. Do you want to overwrite it? (y/n)", name);
                match io::stdin().read_line(&mut input) {
                    Ok(_) => (),
                    Err(e) => {
                        println!("ERROR: {}", e);
                        exit(1);
                    }
                }
                if input.eq("y\n") {
                    break;
                }
                if input.eq("n\n") {
                    println!("Please choose a new name for the file.");
                    println!("Note that this time there won't be a check if the file already exists.");
                    input.clear();
                    match io::stdin().read_line(&mut input) {
                        Ok(_) => return Some(input),
                        Err(e) => {
                            println!("ERROR: {}", e);
                            exit(1);
                        }
                    }
                }
            }
            
        }
        Err(_) => (),
    }
    None
}

fn standard_io_error_handling(e: io::Error) {
    println!("ERROR: {}", e);
    exit(1);
}
